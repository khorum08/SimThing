//! WeightedMean AccumulatorOp parity spike — gather/combine/scatter over parent child ranges.
//!
//! CORRECTNESS NOTE: WeightedMean uses canonical f32 multiply-add in fixed child order.
//! If extended with reordering, atomics, or f64 accumulation, parity classification may
//! change from BIT_EXACT to TOLERANCE_EXACT or FAIL.

use std::time::Instant;

use anyhow::{bail, Context, Result};
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;
use wgpu::{
    Backends, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages,
    CommandEncoderDescriptor, ComputePipeline, ComputePipelineDescriptor, Device, DeviceDescriptor,
    Features, Instance, InstanceDescriptor, Limits, Maintain, MapMode, MemoryHints,
    PipelineLayoutDescriptor, PowerPreference, Queue, RequestAdapterOptions, ShaderModuleDescriptor,
    ShaderStages,
};

pub const WORKGROUP_SIZE: u32 = 64;
pub const WARM_RUNS: usize = 10;
/// Conservative parity tolerance for long f32 reduction chains (FMA ordering may differ on GPU).
pub const DEFAULT_TOLERANCE: f32 = 1e-4;

pub const TIMING_NOTE: &str =
    "GPU warm timings include buffer upload, dispatch, wait, and readback; not pure shader time.";

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct WeightedChild {
    pub value: f32,
    pub weight: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ParentRange {
    pub offset: u32,
    pub len: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct WeightedMeanParams {
    pub n_parents: u32,
    pub _pad0: u32,
    pub _pad1: u32,
    pub _pad2: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct WeightedMeanOutput {
    pub value: f32,
}

#[derive(Debug, Clone)]
pub struct WeightedMeanReport {
    pub scenario_name: String,
    pub n_parents: usize,
    pub n_children: usize,
    pub warm_runs: usize,

    pub cpu_oracle_us: u64,
    /// Time for the first eval on an already-initialized harness (pipeline warmup only).
    pub gpu_cold_total_us: u64,
    pub gpu_warm_mean_us: u64,
    pub gpu_warm_min_us: u64,
    pub gpu_warm_max_us: u64,

    pub max_abs_error: f32,
    pub mean_abs_error: f32,
    pub max_ulp_diff: u32,

    pub bit_exact: bool,
    pub within_tolerance: bool,
    pub repeated_runs_identical: bool,

    pub zero_weight_cases: usize,
    pub single_child_cases: usize,
    pub mixed_magnitude_cases: usize,
    pub negative_value_cases: usize,

    pub correctness_gate: String,
    pub determinism_gate: String,
    pub parity_classification: String,
    pub accumulatorop_weightedmean_gate: String,
    pub timing_note: String,
}

#[derive(Clone)]
pub struct WeightedMeanScenario {
    pub name: String,
    pub children: Vec<WeightedChild>,
    pub ranges: Vec<ParentRange>,
}

pub struct WeightedMeanGpuHarness {
    device: Device,
    queue: Queue,
    pipeline: ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

#[derive(Debug, Clone, Copy, Default)]
struct ScenarioCoverage {
    zero_weight_cases: usize,
    single_child_cases: usize,
    mixed_magnitude_cases: usize,
    negative_value_cases: usize,
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

fn ordered_i32(x: f32) -> i32 {
    let bits = x.to_bits() as i32;
    if bits < 0 {
        i32::MIN - bits
    } else {
        bits
    }
}

pub fn max_ulp_diff(a: f32, b: f32) -> u32 {
    ordered_i32(a).abs_diff(ordered_i32(b))
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

fn outputs_identical(a: &[WeightedMeanOutput], b: &[WeightedMeanOutput]) -> bool {
    if bytemuck::cast_slice::<WeightedMeanOutput, u8>(a)
        == bytemuck::cast_slice::<WeightedMeanOutput, u8>(b)
    {
        return true;
    }
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x.value == y.value)
}

pub fn weighted_mean_cpu(
    children: &[WeightedChild],
    ranges: &[ParentRange],
) -> Vec<WeightedMeanOutput> {
    ranges
        .iter()
        .map(|range| {
            let offset = range.offset as usize;
            let len = range.len as usize;

            if len == 0 {
                return WeightedMeanOutput { value: 0.0 };
            }

            let first = &children[offset];
            let mut weighted_sum = first.value * first.weight;
            let mut weight_sum = first.weight;

            for child in &children[offset + 1..offset + len] {
                let scaled = child.value * child.weight;
                weighted_sum += scaled;
                weight_sum += child.weight;
            }

            let value = if weight_sum == 0.0 {
                0.0
            } else {
                weighted_sum / weight_sum
            };

            WeightedMeanOutput { value }
        })
        .collect()
}

pub fn validate_scenario(children: &[WeightedChild], ranges: &[ParentRange]) -> Result<()> {
    if children.len() > u32::MAX as usize {
        bail!(
            "children.len() {} exceeds u32::MAX",
            children.len()
        );
    }
    if ranges.len() > u32::MAX as usize {
        bail!("ranges.len() {} exceeds u32::MAX", ranges.len());
    }

    for (i, child) in children.iter().enumerate() {
        if !child.value.is_finite() {
            bail!("non-finite child value at index {i}");
        }
        if !child.weight.is_finite() {
            bail!("non-finite child weight at index {i}");
        }
        if child.weight < 0.0 {
            bail!("negative child weight at index {i}");
        }
    }

    for (i, range) in ranges.iter().enumerate() {
        let end = range.offset as u64 + range.len as u64;
        if end > children.len() as u64 {
            bail!(
                "range {i} offset+len ({end}) exceeds children.len() {}",
                children.len()
            );
        }
    }

    Ok(())
}

fn analyze_coverage(children: &[WeightedChild], ranges: &[ParentRange]) -> ScenarioCoverage {
    let mut coverage = ScenarioCoverage::default();

    for range in ranges {
        if range.len == 1 {
            coverage.single_child_cases += 1;
        }

        let offset = range.offset as usize;
        let len = range.len as usize;
        let slice = &children[offset..offset + len];

        let weight_sum: f32 = slice.iter().map(|c| c.weight).sum();
        if weight_sum == 0.0 {
            coverage.zero_weight_cases += 1;
        }

        for child in slice {
            if child.value < 0.0 {
                coverage.negative_value_cases += 1;
            }
            let av = child.value.abs();
            if av >= 1000.0 || (av > 0.0 && av <= 1e-2) {
                coverage.mixed_magnitude_cases += 1;
            }
        }
    }

    coverage
}

pub fn make_weighted_mean_scenario(
    name: &str,
    n_parents: usize,
    children_per_parent: usize,
) -> WeightedMeanScenario {
    let mut children = Vec::new();
    let mut ranges = Vec::with_capacity(n_parents);

    for parent in 0..n_parents {
        let len = match parent % 8 {
            0 => 0,
            1 => 1,
            _ => children_per_parent,
        };

        let offset = children.len();
        for j in 0..len {
            let value = match (parent + j) % 6 {
                0 => -1000.0 + j as f32 * 0.25,
                1 => 1000.0 - j as f32 * 0.125,
                2 => ((parent * 31 + j * 17) as f32 * 0.013).sin() * 10.0,
                3 => -(((parent * 7 + j * 11) as f32 * 0.021).cos() * 5.0),
                4 => 1e-3 * (j as f32 + 1.0),
                _ => parent as f32 * 0.001 + j as f32 * 0.01,
            };

            let weight = if parent % 16 == 0 {
                0.0
            } else {
                match (parent + j) % 5 {
                    0 => 0.001,
                    1 => 0.1,
                    2 => 1.0,
                    3 => 10.0,
                    _ => 100.0,
                }
            };

            children.push(WeightedChild { value, weight });
        }

        ranges.push(ParentRange {
            offset: offset as u32,
            len: len as u32,
        });
    }

    WeightedMeanScenario {
        name: name.to_string(),
        children,
        ranges,
    }
}

impl WeightedMeanGpuHarness {
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

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("simthing-workshop weighted_mean"),
                    required_features: Features::empty(),
                    required_limits: Limits::default(),
                    memory_hints: MemoryHints::default(),
                },
                None,
            )
            .await?;

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("weighted_mean"),
            source: wgpu::ShaderSource::Wgsl(include_str!("weighted_mean.wgsl").into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("weighted_mean_layout"),
            entries: &[
                storage_entry(0, true),
                storage_entry(1, true),
                storage_entry(2, false),
                uniform_entry(3),
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("weighted_mean_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("weighted_mean_pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "main",
            compilation_options: Default::default(),
            cache: None,
        });

        Ok(Self {
            device,
            queue,
            pipeline,
            bind_group_layout,
        })
    }

    pub fn eval(
        &self,
        children: &[WeightedChild],
        ranges: &[ParentRange],
    ) -> Result<Vec<WeightedMeanOutput>> {
        validate_scenario(children, ranges)?;

        if ranges.is_empty() {
            return Ok(Vec::new());
        }

        let n_parents = ranges.len();
        let params = WeightedMeanParams {
            n_parents: n_parents as u32,
            _pad0: 0,
            _pad1: 0,
            _pad2: 0,
        };

        let children_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("weighted_mean_children"),
            contents: bytemuck::cast_slice(children),
            usage: BufferUsages::STORAGE,
        });

        let ranges_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("weighted_mean_ranges"),
            contents: bytemuck::cast_slice(ranges),
            usage: BufferUsages::STORAGE,
        });

        let output_size = (n_parents * std::mem::size_of::<WeightedMeanOutput>()) as u64;
        let output_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("weighted_mean_outputs"),
            size: output_size,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let params_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("weighted_mean_params"),
            contents: bytemuck::bytes_of(&params),
            usage: BufferUsages::UNIFORM,
        });

        let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("weighted_mean_bind_group"),
            layout: &self.bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: children_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: ranges_buffer.as_entire_binding(),
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

        let workgroups = ((n_parents as u32) + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE;
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("weighted_mean_encoder"),
            });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("weighted_mean_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups(workgroups, 1, 1);
        }

        let readback = self.device.create_buffer(&BufferDescriptor {
            label: Some("weighted_mean_readback"),
            size: output_size,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        encoder.copy_buffer_to_buffer(&output_buffer, 0, &readback, 0, output_size);
        self.queue.submit(Some(encoder.finish()));

        Self::readback_outputs(&self.device, &readback, n_parents)
    }

    fn readback_outputs(
        device: &Device,
        readback: &Buffer,
        n_parents: usize,
    ) -> Result<Vec<WeightedMeanOutput>> {
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
        let outputs: Vec<WeightedMeanOutput> = bytemuck::cast_slice(
            &mapped[..n_parents * std::mem::size_of::<WeightedMeanOutput>()],
        )
        .to_vec();
        drop(mapped);
        readback.unmap();
        Ok(outputs)
    }
}

fn compare_outputs(
    cpu: &[WeightedMeanOutput],
    gpu: &[WeightedMeanOutput],
) -> (f32, f32, u32, bool) {
    let mut max_abs_error = 0.0f32;
    let mut sum_abs_error = 0.0f32;
    let mut max_ulp = 0u32;
    let mut bit_exact = true;

    for (c, g) in cpu.iter().zip(gpu.iter()) {
        let err = (c.value - g.value).abs();
        max_abs_error = max_abs_error.max(err);
        sum_abs_error += err;
        max_ulp = max_ulp.max(max_ulp_diff(c.value, g.value));
        if c.value.to_bits() != g.value.to_bits() {
            bit_exact = false;
        }
    }

    let mean_abs_error = if cpu.is_empty() {
        0.0
    } else {
        sum_abs_error / cpu.len() as f32
    };

    (max_abs_error, mean_abs_error, max_ulp, bit_exact)
}

fn build_report(
    scenario: &WeightedMeanScenario,
    cpu_oracle_us: u64,
    gpu_cold_total_us: u64,
    warm_samples: &[u64],
    gpu_reference: &[WeightedMeanOutput],
    cpu_reference: &[WeightedMeanOutput],
    repeated_runs_identical: bool,
    coverage: ScenarioCoverage,
) -> WeightedMeanReport {
    let (max_abs_error, mean_abs_error, max_ulp_diff, bit_exact) =
        compare_outputs(cpu_reference, gpu_reference);
    let within_tolerance = max_abs_error <= DEFAULT_TOLERANCE;

    let parity_classification = if bit_exact && repeated_runs_identical {
        "BIT_EXACT".to_string()
    } else if within_tolerance && repeated_runs_identical {
        "TOLERANCE_EXACT".to_string()
    } else {
        "FAIL".to_string()
    };

    let correctness_gate = if within_tolerance {
        "PASS".to_string()
    } else {
        "FAIL".to_string()
    };

    let determinism_gate = if repeated_runs_identical {
        "PASS".to_string()
    } else {
        "FAIL".to_string()
    };

    let accumulatorop_weightedmean_gate =
        if correctness_gate == "PASS" && determinism_gate == "PASS" && within_tolerance {
            "PASS".to_string()
        } else {
            "FAIL".to_string()
        };

    let (gpu_warm_mean_us, gpu_warm_min_us, gpu_warm_max_us) = warm_stats(warm_samples);

    WeightedMeanReport {
        scenario_name: scenario.name.clone(),
        n_parents: scenario.ranges.len(),
        n_children: scenario.children.len(),
        warm_runs: WARM_RUNS,
        cpu_oracle_us,
        gpu_cold_total_us,
        gpu_warm_mean_us,
        gpu_warm_min_us,
        gpu_warm_max_us,
        max_abs_error,
        mean_abs_error,
        max_ulp_diff,
        bit_exact,
        within_tolerance,
        repeated_runs_identical,
        zero_weight_cases: coverage.zero_weight_cases,
        single_child_cases: coverage.single_child_cases,
        mixed_magnitude_cases: coverage.mixed_magnitude_cases,
        negative_value_cases: coverage.negative_value_cases,
        correctness_gate,
        determinism_gate,
        parity_classification,
        accumulatorop_weightedmean_gate,
        timing_note: TIMING_NOTE.to_string(),
    }
}

pub fn compare_weighted_mean_rich(scenario: &WeightedMeanScenario) -> Result<WeightedMeanReport> {
    let mut harness = WeightedMeanGpuHarness::new()?;
    compare_weighted_mean_rich_with_harness(&mut harness, scenario)
}

pub fn compare_weighted_mean_rich_with_harness(
    harness: &WeightedMeanGpuHarness,
    scenario: &WeightedMeanScenario,
) -> Result<WeightedMeanReport> {
    validate_scenario(&scenario.children, &scenario.ranges)?;

    let coverage = analyze_coverage(&scenario.children, &scenario.ranges);

    let t0 = Instant::now();
    let cpu_reference = weighted_mean_cpu(&scenario.children, &scenario.ranges);
    let cpu_oracle_us = t0.elapsed().as_micros() as u64;

    let cold_start = Instant::now();
    let _cold_gpu = harness.eval(&scenario.children, &scenario.ranges)?;
    let gpu_cold_total_us = cold_start.elapsed().as_micros() as u64;

    let mut warm_samples = Vec::with_capacity(WARM_RUNS);
    let mut warm_baseline: Option<Vec<WeightedMeanOutput>> = None;
    let mut repeated_runs_identical = true;

    for _ in 0..WARM_RUNS {
        let t = Instant::now();
        let repeat = harness.eval(&scenario.children, &scenario.ranges)?;
        warm_samples.push(t.elapsed().as_micros() as u64);
        match &warm_baseline {
            None => warm_baseline = Some(repeat),
            Some(base) if !outputs_identical(&repeat, base) => {
                repeated_runs_identical = false;
            }
            _ => {}
        }
    }

    let gpu_reference = warm_baseline.unwrap_or_default();

    Ok(build_report(
        scenario,
        cpu_oracle_us,
        gpu_cold_total_us,
        &warm_samples,
        &gpu_reference,
        &cpu_reference,
        repeated_runs_identical,
        coverage,
    ))
}

pub fn format_report(report: &WeightedMeanReport) -> String {
    crate::weighted_mean_report::format_weighted_mean_report(report)
}
