//! WeightedMean current-vs-pivot performance benchmark — workshop-local A/B comparison.
//!
//! Compares a production-shaped broad reduction + overlay materialization path against
//! a targeted AccumulatorOp-style WeightedMean path. Not a production pipeline benchmark.

use std::time::Instant;

use anyhow::{Context, Result};
use bytemuck::{Pod, Zeroable};
use serde::Serialize;
use wgpu::util::DeviceExt;
use wgpu::{
    Backends, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages,
    CommandEncoderDescriptor, ComputePipeline, ComputePipelineDescriptor, Device, DeviceDescriptor,
    Features, Instance, InstanceDescriptor, Maintain, MapMode, MemoryHints,
    PipelineLayoutDescriptor, PowerPreference, Queue, RequestAdapterOptions, ShaderModuleDescriptor,
    ShaderStages,
};

use crate::weighted_mean::{LOOSE_TOLERANCE, STRICT_TOLERANCE, TIMING_NOTE};

pub const WORKGROUP_SIZE: u32 = 64;
pub const WARM_RUNS: usize = 10;
pub const PIVOT_MODE: &str = "P1_overlay_then_targeted";

pub const RULE_MEAN: u32 = 0;
pub const RULE_SUM: u32 = 1;
pub const RULE_MAX: u32 = 2;
pub const RULE_MIN: u32 = 3;
pub const RULE_FIRST: u32 = 4;
pub const RULE_WEIGHTED_MEAN: u32 = 5;

pub const OVERLAY_ADD: u32 = 0;
pub const OVERLAY_MUL: u32 = 1;
pub const OVERLAY_SET: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PerfParams {
    pub n_parents: u32,
    pub children_per_parent: u32,
    pub n_dims: u32,
    pub n_weighted_mean_cols: u32,
    pub n_overlays: u32,
    pub _pad0: u32,
    pub _pad1: u32,
    pub _pad2: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ColumnRule {
    pub rule_kind: u32,
    pub weight_col: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct OverlayDelta {
    pub child_index: u32,
    pub col: u32,
    pub op: u32,
    pub value: f32,
}

#[derive(Debug, Clone)]
pub struct WeightedMeanPerfScenario {
    pub name: String,
    pub n_parents: usize,
    pub children_per_parent: usize,
    pub n_dims: usize,
    pub weighted_mean_cols: Vec<u32>,
    pub weight_cols: Vec<u32>,
    pub overlay_density: f32,
    pub child_values: Vec<f32>,
    pub column_rules: Vec<ColumnRule>,
    pub overlays: Vec<OverlayDelta>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WeightedMeanPerfReport {
    pub scenario_name: String,
    pub pivot_mode: String,
    pub n_parents: usize,
    pub children_per_parent: usize,
    pub n_children: usize,
    pub n_dims: usize,
    pub weighted_mean_col_count: usize,
    pub overlay_density: f32,
    pub n_overlays: usize,
    pub warm_runs: usize,

    pub current_warm_mean_us: u64,
    pub current_warm_min_us: u64,
    pub current_warm_max_us: u64,

    pub pivot_warm_mean_us: u64,
    pub pivot_warm_min_us: u64,
    pub pivot_warm_max_us: u64,

    pub speedup_pivot_vs_current: f32,

    pub current_max_abs_error: f32,
    pub pivot_max_abs_error: f32,
    pub current_parity_classification: String,
    pub pivot_parity_classification: String,

    pub current_deterministic: bool,
    pub pivot_deterministic: bool,

    pub interpretation: String,
    pub timing_note: String,
}

pub struct WeightedMeanPerfHarness {
    device: Device,
    queue: Queue,
    max_buffer_size: u64,
    current_overlay_pipeline: ComputePipeline,
    current_reduce_pipeline: ComputePipeline,
    pivot_pipeline: ComputePipeline,
    current_overlay_layout: wgpu::BindGroupLayout,
    current_reduce_layout: wgpu::BindGroupLayout,
    pivot_layout: wgpu::BindGroupLayout,
}

fn storage_entry(binding: u32, read_only: bool) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility: ShaderStages::COMPUTE,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Storage { read_only },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn uniform_entry(binding: u32) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility: ShaderStages::COMPUTE,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn warm_stats(samples: &[u64]) -> (u64, u64, u64) {
    if samples.is_empty() {
        return (0, 0, 0);
    }
    let sum: u64 = samples.iter().sum();
    let mean = sum / samples.len() as u64;
    let min = *samples.iter().min().unwrap();
    let max = *samples.iter().max().unwrap();
    (mean, min, max)
}

fn f32_slices_identical(a: &[f32], b: &[f32]) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x.to_bits() == y.to_bits())
}

pub fn n_children(scenario: &WeightedMeanPerfScenario) -> usize {
    scenario.n_parents * scenario.children_per_parent
}

pub fn make_weighted_mean_perf_scenario(
    name: &str,
    n_parents: usize,
    children_per_parent: usize,
    n_dims: usize,
    weighted_mean_col_count: usize,
    overlay_density: f32,
) -> WeightedMeanPerfScenario {
    let wm_count = weighted_mean_col_count.min(n_dims / 2).max(0);
    let weighted_mean_cols: Vec<u32> = (0..wm_count).map(|i| (i * 2) as u32).collect();
    let weight_cols: Vec<u32> = weighted_mean_cols.iter().map(|c| c + 1).collect();

    let mut column_rules = vec![ColumnRule {
        rule_kind: RULE_FIRST,
        weight_col: 0,
    }; n_dims];

    for col in 0..n_dims {
        let col_u = col as u32;
        if weighted_mean_cols.contains(&col_u) {
            column_rules[col] = ColumnRule {
                rule_kind: RULE_WEIGHTED_MEAN,
                weight_col: col_u + 1,
            };
        } else if col % 4 == 0 {
            column_rules[col] = ColumnRule {
                rule_kind: RULE_SUM,
                weight_col: 0,
            };
        } else {
            column_rules[col] = ColumnRule {
                rule_kind: RULE_FIRST,
                weight_col: 0,
            };
        }
    }

    let n_child_rows = n_parents * children_per_parent;
    let mut child_values = vec![0.0f32; n_child_rows * n_dims];

    for child_index in 0..n_child_rows {
        for col in 0..n_dims {
            let idx = child_index * n_dims + col;
            if weight_cols.contains(&(col as u32)) {
                child_values[idx] = 0.1 + ((child_index + col) % 17) as f32 * 0.01;
            } else {
                child_values[idx] =
                    ((child_index as f32 * 0.013 + col as f32 * 0.017).sin() * 10.0) as f32;
            }
        }
    }

    let mut overlays = Vec::new();
    if overlay_density > 0.0 {
        let mut relevant_cols: Vec<u32> = weighted_mean_cols.clone();
        relevant_cols.extend(&weight_cols);
        relevant_cols.sort_unstable();
        relevant_cols.dedup();

        for child_index in 0..n_child_rows {
            for &col in &relevant_cols {
                let apply = if overlay_density >= 1.0 {
                    true
                } else {
                    (((child_index * 31 + col as usize * 17) % 1000) as f32)
                        < overlay_density * 1000.0
                };
                if !apply {
                    continue;
                }
                let op = match (child_index + col as usize) % 3 {
                    0 => OVERLAY_ADD,
                    1 => OVERLAY_MUL,
                    _ => OVERLAY_SET,
                };
                let value = match op {
                    OVERLAY_ADD => 0.05,
                    OVERLAY_MUL => 1.01,
                    _ => 0.5,
                };
                overlays.push(OverlayDelta {
                    child_index: child_index as u32,
                    col,
                    op,
                    value,
                });
            }
        }
    }

    WeightedMeanPerfScenario {
        name: name.to_string(),
        n_parents,
        children_per_parent,
        n_dims,
        weighted_mean_cols,
        weight_cols,
        overlay_density,
        child_values,
        column_rules,
        overlays,
    }
}

fn apply_overlays_cpu(values: &mut [f32], scenario: &WeightedMeanPerfScenario) {
    for overlay in &scenario.overlays {
        let idx = overlay.child_index as usize * scenario.n_dims + overlay.col as usize;
        match overlay.op {
            OVERLAY_ADD => values[idx] += overlay.value,
            OVERLAY_MUL => values[idx] *= overlay.value,
            OVERLAY_SET => values[idx] = overlay.value,
            _ => {}
        }
    }
}

fn weighted_mean_for_parent(
    values: &[f32],
    scenario: &WeightedMeanPerfScenario,
    parent: usize,
    col: u32,
    weight_col: u32,
) -> f32 {
    let n_dims = scenario.n_dims;
    let cpp = scenario.children_per_parent;
    let base = parent * cpp;

    let first_v = values[base * n_dims + col as usize];
    let first_w = values[base * n_dims + weight_col as usize];
    let mut weighted_sum = first_v * first_w;
    let mut weight_sum = first_w;

    for j in 1..cpp {
        let child = base + j;
        let v = values[child * n_dims + col as usize];
        let w = values[child * n_dims + weight_col as usize];
        weighted_sum += v * w;
        weight_sum += w;
    }

    if weight_sum == 0.0 {
        0.0
    } else {
        weighted_sum / weight_sum
    }
}

fn reduce_column_for_parent(
    values: &[f32],
    scenario: &WeightedMeanPerfScenario,
    parent: usize,
    col: usize,
) -> f32 {
    let rule = scenario.column_rules[col];
    let n_dims = scenario.n_dims;
    let cpp = scenario.children_per_parent;
    let base = parent * cpp;

    match rule.rule_kind {
        RULE_WEIGHTED_MEAN => {
            weighted_mean_for_parent(values, scenario, parent, col as u32, rule.weight_col)
        }
        RULE_SUM => {
            let mut acc = values[base * n_dims + col];
            for j in 1..cpp {
                acc += values[(base + j) * n_dims + col];
            }
            acc
        }
        RULE_MEAN => {
            let mut acc = values[base * n_dims + col];
            for j in 1..cpp {
                acc += values[(base + j) * n_dims + col];
            }
            acc / cpp as f32
        }
        _ => values[base * n_dims + col],
    }
}

pub fn cpu_current_style_reference(scenario: &WeightedMeanPerfScenario) -> Vec<f32> {
    let mut values = scenario.child_values.clone();
    apply_overlays_cpu(&mut values, scenario);

    let mut outputs = vec![0.0f32; scenario.n_parents * scenario.n_dims];
    for parent in 0..scenario.n_parents {
        for col in 0..scenario.n_dims {
            outputs[parent * scenario.n_dims + col] =
                reduce_column_for_parent(&values, scenario, parent, col);
        }
    }
    outputs
}

pub fn cpu_pivot_reference(scenario: &WeightedMeanPerfScenario) -> Vec<f32> {
    let mut values = scenario.child_values.clone();
    apply_overlays_cpu(&mut values, scenario);

    let wm_count = scenario.weighted_mean_cols.len();
    let mut outputs = vec![0.0f32; scenario.n_parents * wm_count];
    for parent in 0..scenario.n_parents {
        for (wm_idx, (&col, &wcol)) in scenario
            .weighted_mean_cols
            .iter()
            .zip(scenario.weight_cols.iter())
            .enumerate()
        {
            outputs[parent * wm_count + wm_idx] =
                weighted_mean_for_parent(&values, scenario, parent, col, wcol);
        }
    }
    outputs
}

pub fn extract_wm_from_current(current: &[f32], scenario: &WeightedMeanPerfScenario) -> Vec<f32> {
    let wm_count = scenario.weighted_mean_cols.len();
    let mut out = vec![0.0f32; scenario.n_parents * wm_count];
    for parent in 0..scenario.n_parents {
        for (wm_idx, &col) in scenario.weighted_mean_cols.iter().enumerate() {
            out[parent * wm_count + wm_idx] =
                current[parent * scenario.n_dims + col as usize];
        }
    }
    out
}

fn max_abs_error(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).abs())
        .fold(0.0f32, f32::max)
}

pub fn classify_parity(max_error: f32, deterministic: bool) -> String {
    if !deterministic {
        return "FAIL".to_string();
    }
    if max_error == 0.0 {
        "BIT_EXACT".to_string()
    } else if max_error <= STRICT_TOLERANCE {
        "STRICT_TOLERANCE".to_string()
    } else if max_error <= LOOSE_TOLERANCE {
        "LOOSE_TOLERANCE".to_string()
    } else {
        "FAIL".to_string()
    }
}

fn build_interpretation(
    report: &WeightedMeanPerfReport,
    speedup: f32,
) -> String {
    let current_ok = report.current_parity_classification != "FAIL";
    let pivot_ok = report.pivot_parity_classification != "FAIL";

    if pivot_ok
        && current_ok
        && speedup > 1.5
        && report.weighted_mean_col_count < report.n_dims
    {
        return "Pivot targeted WeightedMean is beneficial for this sparse aggregate shape.".to_string();
    }
    if report.weighted_mean_col_count >= report.n_dims / 2 {
        return "Expected: broad reduction amortizes work when many columns are needed.".to_string();
    }
    if report.overlay_density >= 0.9 {
        return "Overlay materialization dominates both paths at high overlay density.".to_string();
    }
    if speedup <= 1.0 {
        return "Current broad reduction remains competitive for this shape.".to_string();
    }
    format!(
        "Pivot faster ({speedup:.2}x) but below 1.5x sparse-target threshold or correctness gate nuance; interpret with workload context."
    )
}

impl WeightedMeanPerfHarness {
    pub fn new() -> Result<Self> {
        pollster::block_on(Self::new_async())
    }

    async fn new_async() -> Result<Self> {
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::PRIMARY,
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await
            .context("no suitable GPU adapter found")?;

        let limits = adapter.limits();
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("simthing-workshop weighted_mean_perf"),
                    required_features: Features::empty(),
                    required_limits: limits.clone(),
                    memory_hints: MemoryHints::default(),
                },
                None,
            )
            .await?;

        let current_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("weighted_mean_current_style"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("weighted_mean_current_style.wgsl").into(),
            ),
        });

        let pivot_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("weighted_mean_pivot_style"),
            source: wgpu::ShaderSource::Wgsl(include_str!("weighted_mean_pivot_style.wgsl").into()),
        });

        let current_overlay_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("wm_perf_overlay_layout"),
            entries: &[
                storage_entry(0, false),
                storage_entry(1, true),
                uniform_entry(2),
            ],
        });

        let current_reduce_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("wm_perf_current_reduce_layout"),
            entries: &[
                storage_entry(0, true),
                storage_entry(1, true),
                storage_entry(2, false),
                uniform_entry(3),
            ],
        });

        let pivot_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("wm_perf_pivot_layout"),
            entries: &[
                storage_entry(0, true),
                storage_entry(1, true),
                storage_entry(2, true),
                storage_entry(3, false),
                uniform_entry(4),
            ],
        });

        let current_overlay_pipeline =
            device.create_compute_pipeline(&ComputePipelineDescriptor {
                label: Some("wm_perf_overlay"),
                layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                    label: Some("wm_perf_overlay_pl"),
                    bind_group_layouts: &[&current_overlay_layout],
                    push_constant_ranges: &[],
                })),
                module: &current_shader,
                entry_point: "apply_overlays",
                compilation_options: Default::default(),
                cache: None,
            });

        let current_reduce_pipeline =
            device.create_compute_pipeline(&ComputePipelineDescriptor {
                label: Some("wm_perf_current_reduce"),
                layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                    label: Some("wm_perf_current_reduce_pl"),
                    bind_group_layouts: &[&current_reduce_layout],
                    push_constant_ranges: &[],
                })),
                module: &current_shader,
                entry_point: "current_reduce",
                compilation_options: Default::default(),
                cache: None,
            });

        let pivot_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("wm_perf_pivot"),
            layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("wm_perf_pivot_pl"),
                bind_group_layouts: &[&pivot_layout],
                push_constant_ranges: &[],
            })),
            module: &pivot_shader,
            entry_point: "pivot_weighted_mean",
            compilation_options: Default::default(),
            cache: None,
        });

        Ok(Self {
            device,
            queue,
            max_buffer_size: limits.max_buffer_size,
            current_overlay_pipeline,
            current_reduce_pipeline,
            pivot_pipeline,
            current_overlay_layout,
            current_reduce_layout,
            pivot_layout,
        })
    }

    fn values_buffer_bytes(scenario: &WeightedMeanPerfScenario) -> u64 {
        (n_children(scenario) * scenario.n_dims * std::mem::size_of::<f32>()) as u64
    }

    fn parents_per_chunk(scenario: &WeightedMeanPerfScenario, max_buffer: u64) -> usize {
        let per_parent =
            scenario.children_per_parent * scenario.n_dims * std::mem::size_of::<f32>();
        if per_parent == 0 {
            return scenario.n_parents.max(1);
        }
        let budget = max_buffer.saturating_mul(80) / 100;
        ((budget as usize / per_parent).max(1)).min(scenario.n_parents.max(1))
    }

    fn slice_scenario(
        scenario: &WeightedMeanPerfScenario,
        parent_start: usize,
        parent_count: usize,
    ) -> WeightedMeanPerfScenario {
        let cpp = scenario.children_per_parent;
        let nd = scenario.n_dims;
        let child_start = parent_start * cpp;
        let child_rows = parent_count * cpp;

        let child_values = scenario.child_values
            [child_start * nd..(child_start + child_rows) * nd]
            .to_vec();

        let overlays = scenario
            .overlays
            .iter()
            .filter_map(|o| {
                let ci = o.child_index as usize;
                if ci >= child_start && ci < child_start + child_rows {
                    Some(OverlayDelta {
                        child_index: (ci - child_start) as u32,
                        col: o.col,
                        op: o.op,
                        value: o.value,
                    })
                } else {
                    None
                }
            })
            .collect();

        WeightedMeanPerfScenario {
            name: format!("{}_chunk_{parent_start}", scenario.name),
            n_parents: parent_count,
            children_per_parent: scenario.children_per_parent,
            n_dims: scenario.n_dims,
            weighted_mean_cols: scenario.weighted_mean_cols.clone(),
            weight_cols: scenario.weight_cols.clone(),
            overlay_density: scenario.overlay_density,
            child_values,
            column_rules: scenario.column_rules.clone(),
            overlays,
        }
    }

    fn perf_params(scenario: &WeightedMeanPerfScenario) -> PerfParams {
        PerfParams {
            n_parents: scenario.n_parents as u32,
            children_per_parent: scenario.children_per_parent as u32,
            n_dims: scenario.n_dims as u32,
            n_weighted_mean_cols: scenario.weighted_mean_cols.len() as u32,
            n_overlays: scenario.overlays.len() as u32,
            _pad0: 0,
            _pad1: 0,
            _pad2: 0,
        }
    }

    fn run_overlay_pass(
        &self,
        values_buffer: &Buffer,
        scenario: &WeightedMeanPerfScenario,
    ) -> Result<()> {
        if scenario.overlays.is_empty() {
            return Ok(());
        }

        let params = Self::perf_params(scenario);
        let overlays_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("wm_perf_overlays"),
            contents: bytemuck::cast_slice(&scenario.overlays),
            usage: BufferUsages::STORAGE,
        });
        let params_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("wm_perf_overlay_params"),
            contents: bytemuck::bytes_of(&params),
            usage: BufferUsages::UNIFORM,
        });

        let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("wm_perf_overlay_bg"),
            layout: &self.current_overlay_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: values_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: overlays_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: params_buffer.as_entire_binding(),
                },
            ],
        });

        let workgroups =
            ((scenario.overlays.len() as u32) + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE;
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("wm_perf_overlay_enc"),
            });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("wm_perf_overlay_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.current_overlay_pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups(workgroups.max(1), 1, 1);
        }
        self.queue.submit(Some(encoder.finish()));
        Ok(())
    }

    pub fn run_current_style(&self, scenario: &WeightedMeanPerfScenario) -> Result<Vec<f32>> {
        if Self::values_buffer_bytes(scenario) <= self.max_buffer_size {
            return self.run_current_style_once(scenario);
        }

        let chunk_parents = Self::parents_per_chunk(scenario, self.max_buffer_size);
        let mut full = vec![0.0f32; scenario.n_parents * scenario.n_dims];
        let mut parent_start = 0;
        while parent_start < scenario.n_parents {
            let count = chunk_parents.min(scenario.n_parents - parent_start);
            let chunk = Self::slice_scenario(scenario, parent_start, count);
            let partial = self.run_current_style_once(&chunk)?;
            for p in 0..count {
                let dst = (parent_start + p) * scenario.n_dims;
                let src = p * scenario.n_dims;
                full[dst..dst + scenario.n_dims].copy_from_slice(&partial[src..src + scenario.n_dims]);
            }
            parent_start += count;
        }
        Ok(full)
    }

    fn run_current_style_once(&self, scenario: &WeightedMeanPerfScenario) -> Result<Vec<f32>> {
        let n_child_rows = n_children(scenario);
        let _values_size = (n_child_rows * scenario.n_dims * std::mem::size_of::<f32>()) as u64;
        let outputs_size =
            (scenario.n_parents * scenario.n_dims * std::mem::size_of::<f32>()) as u64;

        let values_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("wm_perf_current_values"),
            contents: bytemuck::cast_slice(&scenario.child_values),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });

        self.run_overlay_pass(&values_buffer, scenario)?;

        let rules_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("wm_perf_column_rules"),
            contents: bytemuck::cast_slice(&scenario.column_rules),
            usage: BufferUsages::STORAGE,
        });

        let output_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("wm_perf_current_outputs"),
            size: outputs_size,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let params = Self::perf_params(scenario);
        let params_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("wm_perf_current_params"),
            contents: bytemuck::bytes_of(&params),
            usage: BufferUsages::UNIFORM,
        });

        let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("wm_perf_current_reduce_bg"),
            layout: &self.current_reduce_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: values_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: rules_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: output_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: params_buffer.as_entire_binding(),
                },
            ],
        });

        let workgroups = ((scenario.n_parents as u32) + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE;
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("wm_perf_current_enc"),
            });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("wm_perf_current_reduce_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.current_reduce_pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups(workgroups, 1, 1);
        }

        let readback = self.device.create_buffer(&BufferDescriptor {
            label: Some("wm_perf_current_readback"),
            size: outputs_size,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        encoder.copy_buffer_to_buffer(&output_buffer, 0, &readback, 0, outputs_size);
        self.queue.submit(Some(encoder.finish()));

        Self::readback_f32(&self.device, &readback, scenario.n_parents * scenario.n_dims)
    }

    pub fn run_pivot_style(&self, scenario: &WeightedMeanPerfScenario) -> Result<Vec<f32>> {
        if Self::values_buffer_bytes(scenario) <= self.max_buffer_size {
            return self.run_pivot_style_once(scenario);
        }

        let wm_count = scenario.weighted_mean_cols.len();
        let chunk_parents = Self::parents_per_chunk(scenario, self.max_buffer_size);
        let mut full = vec![0.0f32; scenario.n_parents * wm_count];
        let mut parent_start = 0;
        while parent_start < scenario.n_parents {
            let count = chunk_parents.min(scenario.n_parents - parent_start);
            let chunk = Self::slice_scenario(scenario, parent_start, count);
            let partial = self.run_pivot_style_once(&chunk)?;
            for p in 0..count {
                let dst = (parent_start + p) * wm_count;
                let src = p * wm_count;
                full[dst..dst + wm_count].copy_from_slice(&partial[src..src + wm_count]);
            }
            parent_start += count;
        }
        Ok(full)
    }

    fn run_pivot_style_once(&self, scenario: &WeightedMeanPerfScenario) -> Result<Vec<f32>> {
        let wm_count = scenario.weighted_mean_cols.len();
        let outputs_len = scenario.n_parents * wm_count;
        let outputs_size = (outputs_len * std::mem::size_of::<f32>()) as u64;

        let values_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("wm_perf_pivot_values"),
            contents: bytemuck::cast_slice(&scenario.child_values),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });

        self.run_overlay_pass(&values_buffer, scenario)?;

        let wm_cols_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("wm_perf_wm_cols"),
            contents: bytemuck::cast_slice(&scenario.weighted_mean_cols),
            usage: BufferUsages::STORAGE,
        });
        let weight_cols_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("wm_perf_weight_cols"),
            contents: bytemuck::cast_slice(&scenario.weight_cols),
            usage: BufferUsages::STORAGE,
        });

        let output_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("wm_perf_pivot_outputs"),
            size: outputs_size,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let params = Self::perf_params(scenario);
        let params_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("wm_perf_pivot_params"),
            contents: bytemuck::bytes_of(&params),
            usage: BufferUsages::UNIFORM,
        });

        let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("wm_perf_pivot_bg"),
            layout: &self.pivot_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: values_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: wm_cols_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: weight_cols_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: output_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 4,
                    resource: params_buffer.as_entire_binding(),
                },
            ],
        });

        let total_ops = scenario.n_parents * wm_count;
        let workgroups = ((total_ops as u32) + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE;
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("wm_perf_pivot_enc"),
            });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("wm_perf_pivot_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.pivot_pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups(workgroups.max(1), 1, 1);
        }

        let readback = self.device.create_buffer(&BufferDescriptor {
            label: Some("wm_perf_pivot_readback"),
            size: outputs_size,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        encoder.copy_buffer_to_buffer(&output_buffer, 0, &readback, 0, outputs_size);
        self.queue.submit(Some(encoder.finish()));

        Self::readback_f32(&self.device, &readback, outputs_len)
    }

    fn readback_f32(device: &Device, readback: &Buffer, len: usize) -> Result<Vec<f32>> {
        let slice = readback.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(MapMode::Read, move |r| {
            let _ = tx.send(r);
        });
        device.poll(Maintain::Wait);
        rx.recv()
            .context("map_async sender dropped")?
            .context("buffer map failed")?;

        let mapped = slice.get_mapped_range();
        let outputs: Vec<f32> =
            bytemuck::cast_slice(&mapped[..len * std::mem::size_of::<f32>()]).to_vec();
        drop(mapped);
        readback.unmap();
        Ok(outputs)
    }
}

pub fn compare_weighted_mean_perf_with_harness(
    harness: &WeightedMeanPerfHarness,
    scenario: &WeightedMeanPerfScenario,
) -> Result<WeightedMeanPerfReport> {
    let cpu_wm = cpu_pivot_reference(scenario);
    let cpu_current = cpu_current_style_reference(scenario);
    let cpu_wm_from_current = extract_wm_from_current(&cpu_current, scenario);

    let _ = harness.run_current_style(scenario)?;
    let _ = harness.run_pivot_style(scenario)?;

    let mut current_warm = Vec::with_capacity(WARM_RUNS);
    let mut pivot_warm = Vec::with_capacity(WARM_RUNS);
    let mut current_baseline: Option<Vec<f32>> = None;
    let mut pivot_baseline: Option<Vec<f32>> = None;
    let mut current_deterministic = true;
    let mut pivot_deterministic = true;

    for _ in 0..WARM_RUNS {
        let t0 = Instant::now();
        let current = harness.run_current_style(scenario)?;
        current_warm.push(t0.elapsed().as_micros() as u64);
        match &current_baseline {
            None => current_baseline = Some(current),
            Some(base) if !f32_slices_identical(&current, base) => {
                current_deterministic = false;
            }
            _ => {}
        }

        let t1 = Instant::now();
        let pivot = harness.run_pivot_style(scenario)?;
        pivot_warm.push(t1.elapsed().as_micros() as u64);
        match &pivot_baseline {
            None => pivot_baseline = Some(pivot),
            Some(base) if !f32_slices_identical(&pivot, base) => {
                pivot_deterministic = false;
            }
            _ => {}
        }
    }

    let current_gpu = current_baseline.unwrap_or_default();
    let pivot_gpu = pivot_baseline.unwrap_or_default();
    let current_wm_gpu = extract_wm_from_current(&current_gpu, scenario);

    let current_max_abs_error = max_abs_error(&cpu_wm_from_current, &current_wm_gpu);
    let pivot_max_abs_error = max_abs_error(&cpu_wm, &pivot_gpu);

    let current_parity_classification =
        classify_parity(current_max_abs_error, current_deterministic);
    let pivot_parity_classification = classify_parity(pivot_max_abs_error, pivot_deterministic);

    let (current_warm_mean_us, current_warm_min_us, current_warm_max_us) =
        warm_stats(&current_warm);
    let (pivot_warm_mean_us, pivot_warm_min_us, pivot_warm_max_us) = warm_stats(&pivot_warm);

    let speedup_pivot_vs_current = if pivot_warm_mean_us > 0 {
        current_warm_mean_us as f32 / pivot_warm_mean_us as f32
    } else {
        0.0
    };

    let mut report = WeightedMeanPerfReport {
        scenario_name: scenario.name.clone(),
        pivot_mode: PIVOT_MODE.to_string(),
        n_parents: scenario.n_parents,
        children_per_parent: scenario.children_per_parent,
        n_children: n_children(scenario),
        n_dims: scenario.n_dims,
        weighted_mean_col_count: scenario.weighted_mean_cols.len(),
        overlay_density: scenario.overlay_density,
        n_overlays: scenario.overlays.len(),
        warm_runs: WARM_RUNS,
        current_warm_mean_us,
        current_warm_min_us,
        current_warm_max_us,
        pivot_warm_mean_us,
        pivot_warm_min_us,
        pivot_warm_max_us,
        speedup_pivot_vs_current,
        current_max_abs_error,
        pivot_max_abs_error,
        current_parity_classification,
        pivot_parity_classification,
        current_deterministic,
        pivot_deterministic,
        interpretation: String::new(),
        timing_note: TIMING_NOTE.to_string(),
    };
    report.interpretation = build_interpretation(&report, speedup_pivot_vs_current);
    Ok(report)
}

pub fn compare_weighted_mean_perf(scenario: &WeightedMeanPerfScenario) -> Result<WeightedMeanPerfReport> {
    let harness = WeightedMeanPerfHarness::new()?;
    compare_weighted_mean_perf_with_harness(&harness, scenario)
}

pub fn format_perf_report(report: &WeightedMeanPerfReport) -> String {
    crate::weighted_mean_perf_report::format_weighted_mean_perf_report(report)
}

pub fn write_perf_reports(report: &WeightedMeanPerfReport) -> Result<()> {
    crate::weighted_mean_perf_report::write_perf_reports(report).map_err(Into::into)
}
