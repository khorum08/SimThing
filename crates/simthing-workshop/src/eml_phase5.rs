//! EML Phase 5 intensity-update spike — hand-authored expression tree, CPU + GPU evaluators.
//!
//! CORRECTNESS NOTE: The intensity formula (abs, mul, add, sub, select, clamp01) uses only
//! IEEE 754 operations with guaranteed exact behavior on both CPU and GPU. Error is exactly
//! 0.0 on all tested hardware. If this formula is extended to include exp() or log(), error
//! will no longer be zero and tolerances (1e-4 / 1e-5) must be validated empirically. The
//! assertions in tests/eml_phase5_intensity.rs use 1e-4 / 1e-5 as conservative guards, not
//! because drift was observed.

use std::time::Instant;

use anyhow::{bail, Context, Result};
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;
use wgpu::{
    Backends, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages,
    CommandEncoderDescriptor, ComputePipeline, ComputePipelineDescriptor, Device, DeviceDescriptor,
    Features, Instance, InstanceDescriptor, Limits, Maintain, MapMode, MemoryHints,
    PipelineLayoutDescriptor, PowerPreference, Queue, RequestAdapterOptions,
    ShaderModuleDescriptor, ShaderStages,
};

pub const MAX_NODES: usize = 32;
pub const WARM_RUNS: usize = 10;

pub const OP_CONST: u32 = 0;
pub const OP_INPUT_VELOCITY: u32 = 1;
pub const OP_INPUT_INTENSITY: u32 = 2;
pub const OP_ABS: u32 = 3;
pub const OP_GREATER_THAN: u32 = 4;
pub const OP_MUL: u32 = 5;
pub const OP_ADD: u32 = 6;
pub const OP_SUB: u32 = 7;
pub const OP_SELECT: u32 = 8;
pub const OP_CLAMP01: u32 = 9;

pub const TIMING_NOTE: &str =
    "GPU warm timings include buffer upload, dispatch, wait, and readback; not pure shader time.";

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct IntensityInput {
    pub velocity: f32,
    pub intensity: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct IntensityOutput {
    pub value: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct EmlParams {
    pub n_slots: u32,
    pub root_node: u32,
    pub _pad0: u32,
    pub _pad1: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct IntensityFormulaParams {
    pub n_slots: u32,
    pub velocity_threshold: f32,
    pub build_coefficient: f32,
    pub decay_coefficient: f32,
    pub dt: f32,
    pub _pad0: u32,
    pub _pad1: u32,
    pub _pad2: u32,
}

impl IntensityFormulaParams {
    pub fn new(
        n_slots: u32,
        velocity_threshold: f32,
        build_coefficient: f32,
        decay_coefficient: f32,
        dt: f32,
    ) -> Self {
        Self {
            n_slots,
            velocity_threshold,
            build_coefficient,
            decay_coefficient,
            dt,
            _pad0: 0,
            _pad1: 0,
            _pad2: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct EmlNode {
    pub op: u32,
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub value: f32,
    pub _pad0: u32,
    pub _pad1: u32,
    pub _pad2: u32,
}

#[derive(Debug)]
pub struct EmlGpuReport {
    pub n_slots: usize,
    pub max_abs_error: f32,
    pub mean_abs_error: f32,
    pub gpu_eval_us: u64,
    pub cpu_eval_us: u64,
    pub repeated_runs_identical: bool,
}

#[derive(Debug, Clone)]
pub struct EmlGpuRichReport {
    pub n_slots: usize,

    pub cpu_node_eval_us: u64,
    pub cpu_direct_eval_us: u64,

    /// Time for the first eval_eml call on an already-initialized harness.
    /// Captures pipeline warmup cost. Does NOT include device/pipeline creation.
    pub gpu_eml_cold_total_us: u64,
    pub gpu_eml_warm_mean_us: u64,
    pub gpu_eml_warm_min_us: u64,
    pub gpu_eml_warm_max_us: u64,

    pub gpu_hardcoded_warm_mean_us: u64,
    pub gpu_hardcoded_warm_min_us: u64,
    pub gpu_hardcoded_warm_max_us: u64,

    pub eml_vs_cpu_max_abs_error: f32,
    pub eml_vs_cpu_mean_abs_error: f32,

    pub hardcoded_vs_cpu_max_abs_error: f32,
    pub hardcoded_vs_cpu_mean_abs_error: f32,

    pub eml_vs_hardcoded_max_abs_error: f32,
    pub eml_vs_hardcoded_mean_abs_error: f32,

    /// `gpu_eml_warm_mean_us / gpu_hardcoded_warm_mean_us`.
    /// 1.0 means EML costs the same as hardcoded.
    /// Decision-relevant: acceptable if < 3.0 at 100k slots.
    pub eml_vs_hardcoded_overhead_ratio: f32,

    pub eml_repeated_runs_identical: bool,
    pub hardcoded_repeated_runs_identical: bool,

    pub warm_runs: usize,
    pub timing_note: String,
}

pub struct EmlGpuHarness {
    device: Device,
    queue: Queue,
    eml_pipeline: ComputePipeline,
    hardcoded_pipeline: ComputePipeline,
    eml_bind_group_layout: wgpu::BindGroupLayout,
    hardcoded_bind_group_layout: wgpu::BindGroupLayout,
    cached_node_buffer: Option<(Vec<EmlNode>, Buffer)>,
}

fn node(op: u32, a: u32, b: u32, c: u32, value: f32) -> EmlNode {
    EmlNode {
        op,
        a,
        b,
        c,
        value,
        _pad0: 0,
        _pad1: 0,
        _pad2: 0,
    }
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

fn compare_errors(a: &[f32], b: &[IntensityOutput]) -> (f32, f32) {
    let mut max_abs = 0.0f32;
    let mut sum_abs = 0.0f32;
    for (x, y) in a.iter().zip(b.iter()) {
        let err = (x - y.value).abs();
        max_abs = max_abs.max(err);
        sum_abs += err;
    }
    let mean_abs = if a.is_empty() {
        0.0
    } else {
        sum_abs / a.len() as f32
    };
    (max_abs, mean_abs)
}

fn compare_output_pairs(a: &[IntensityOutput], b: &[IntensityOutput]) -> (f32, f32) {
    let left: Vec<f32> = a.iter().map(|o| o.value).collect();
    compare_errors(&left, b)
}

fn outputs_identical(a: &[IntensityOutput], b: &[IntensityOutput]) -> bool {
    if bytemuck::cast_slice::<IntensityOutput, u8>(a)
        == bytemuck::cast_slice::<IntensityOutput, u8>(b)
    {
        return true;
    }
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x.value == y.value)
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

/// Hand-authored intensity-update expression tree (topologically sorted, 16 nodes, root 15).
pub fn intensity_update_nodes(
    velocity_threshold: f32,
    build_coefficient: f32,
    decay_coefficient: f32,
    dt: f32,
) -> (Vec<EmlNode>, u32) {
    let nodes = vec![
        node(OP_INPUT_VELOCITY, 0, 0, 0, 0.0),
        node(OP_INPUT_INTENSITY, 0, 0, 0, 0.0),
        node(OP_ABS, 0, 0, 0, 0.0),
        node(OP_CONST, 0, 0, 0, velocity_threshold),
        node(OP_GREATER_THAN, 2, 3, 0, 0.0),
        node(OP_CONST, 0, 0, 0, build_coefficient),
        node(OP_MUL, 5, 2, 0, 0.0),
        node(OP_CONST, 0, 0, 0, dt),
        node(OP_MUL, 6, 7, 0, 0.0),
        node(OP_ADD, 1, 8, 0, 0.0),
        node(OP_CONST, 0, 0, 0, decay_coefficient),
        node(OP_MUL, 10, 1, 0, 0.0),
        node(OP_MUL, 11, 7, 0, 0.0),
        node(OP_SUB, 1, 12, 0, 0.0),
        node(OP_SELECT, 4, 9, 13, 0.0),
        node(OP_CLAMP01, 14, 0, 0, 0.0),
    ];
    assert!(nodes.len() <= MAX_NODES);
    (nodes, 15)
}

/// Direct CPU reference matching production `intensity_update.wgsl` FMA evaluation order.
pub fn intensity_update_direct_cpu(
    input: IntensityInput,
    velocity_threshold: f32,
    build_coefficient: f32,
    decay_coefficient: f32,
    dt: f32,
) -> f32 {
    let vel_abs = input.velocity.abs();
    let next = if vel_abs > velocity_threshold {
        let scaled = build_coefficient * vel_abs;
        let delta = scaled * dt;
        input.intensity + delta
    } else {
        let scaled = decay_coefficient * input.intensity;
        let delta = scaled * dt;
        input.intensity - delta
    };
    next.clamp(0.0, 1.0)
}

pub fn eval_cpu_node(nodes: &[EmlNode], root: u32, input: IntensityInput) -> f32 {
    assert!(nodes.len() <= MAX_NODES);
    assert!((root as usize) < nodes.len());

    fn eval(nodes: &[EmlNode], idx: u32, input: IntensityInput) -> f32 {
        let n = &nodes[idx as usize];
        match n.op {
            OP_CONST => n.value,
            OP_INPUT_VELOCITY => input.velocity,
            OP_INPUT_INTENSITY => input.intensity,
            OP_ABS => eval(nodes, n.a, input).abs(),
            OP_GREATER_THAN => {
                if eval(nodes, n.a, input) > eval(nodes, n.b, input) {
                    1.0
                } else {
                    0.0
                }
            }
            OP_MUL => eval(nodes, n.a, input) * eval(nodes, n.b, input),
            OP_ADD => eval(nodes, n.a, input) + eval(nodes, n.b, input),
            OP_SUB => eval(nodes, n.a, input) - eval(nodes, n.b, input),
            OP_SELECT => {
                if eval(nodes, n.a, input) != 0.0 {
                    eval(nodes, n.b, input)
                } else {
                    eval(nodes, n.c, input)
                }
            }
            OP_CLAMP01 => eval(nodes, n.a, input).clamp(0.0, 1.0),
            op => panic!("unknown EML opcode: {op}"),
        }
    }

    eval(nodes, root, input)
}

pub fn make_inputs(n: usize) -> Vec<IntensityInput> {
    make_inputs_with_params(n, 0.1)
}

pub fn make_inputs_with_params(n: usize, threshold: f32) -> Vec<IntensityInput> {
    let edge_cases = [
        IntensityInput {
            velocity: 0.0,
            intensity: 0.0,
        },
        IntensityInput {
            velocity: 0.0,
            intensity: 1.0,
        },
        IntensityInput {
            velocity: threshold - 0.00001,
            intensity: 0.5,
        },
        IntensityInput {
            velocity: threshold,
            intensity: 0.5,
        },
        IntensityInput {
            velocity: threshold + 0.00001,
            intensity: 0.5,
        },
        IntensityInput {
            velocity: -threshold - 0.00001,
            intensity: 0.5,
        },
        IntensityInput {
            velocity: 1000.0,
            intensity: 0.2,
        },
        IntensityInput {
            velocity: -1000.0,
            intensity: 0.8,
        },
    ];

    let mut inputs = Vec::with_capacity(n);
    for &case in edge_cases.iter().take(n) {
        assert!(case.velocity.is_finite());
        assert!(case.intensity.is_finite());
        inputs.push(case);
    }

    for i in edge_cases.len()..n {
        let velocity = (i as f32 * 0.013).sin() * 5.0;
        let intensity = ((i as f32 * 0.017).cos() * 0.5 + 0.5).clamp(0.0, 1.0);
        assert!(velocity.is_finite());
        assert!(intensity.is_finite());
        inputs.push(IntensityInput {
            velocity,
            intensity,
        });
    }

    inputs
}

pub fn validate_nodes(nodes: &[EmlNode], root_node: u32) -> Result<()> {
    if nodes.is_empty() {
        bail!("EML node list is empty");
    }
    if nodes.len() > MAX_NODES {
        bail!(
            "EML node list has {} nodes; max is {}",
            nodes.len(),
            MAX_NODES
        );
    }
    if root_node as usize >= nodes.len() {
        bail!(
            "root_node {} out of bounds for {} nodes",
            root_node,
            nodes.len()
        );
    }
    for (i, n) in nodes.iter().enumerate() {
        match n.op {
            OP_CONST | OP_INPUT_VELOCITY | OP_INPUT_INTENSITY | OP_ABS | OP_GREATER_THAN
            | OP_MUL | OP_ADD | OP_SUB | OP_SELECT | OP_CLAMP01 => {}
            op => bail!("unknown EML opcode {op} at node {i}"),
        }
    }
    Ok(())
}

fn validate_finite_inputs(inputs: &[IntensityInput]) -> Result<()> {
    for (i, input) in inputs.iter().enumerate() {
        if !input.velocity.is_finite() || !input.intensity.is_finite() {
            bail!("non-finite input at slot {i}: {:?}", input);
        }
    }
    Ok(())
}

fn validate_finite_outputs(outputs: &[IntensityOutput]) -> Result<()> {
    for (i, output) in outputs.iter().enumerate() {
        if !output.value.is_finite() {
            bail!("non-finite GPU output at slot {i}: {}", output.value);
        }
    }
    Ok(())
}

impl EmlGpuHarness {
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
                    label: Some("simthing-workshop eml_phase5"),
                    required_features: Features::empty(),
                    required_limits: Limits::default(),
                    memory_hints: MemoryHints::default(),
                },
                None,
            )
            .await?;

        let eml_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("eml_phase5"),
            source: wgpu::ShaderSource::Wgsl(include_str!("eml_phase5.wgsl").into()),
        });

        let hardcoded_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("eml_phase5_hardcoded"),
            source: wgpu::ShaderSource::Wgsl(include_str!("eml_phase5_hardcoded.wgsl").into()),
        });

        let eml_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("eml_phase5_layout"),
            entries: &[
                storage_entry(0, true),
                storage_entry(1, true),
                storage_entry(2, false),
                uniform_entry(3),
            ],
        });

        let hardcoded_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("eml_phase5_hardcoded_layout"),
                entries: &[
                    storage_entry(0, true),
                    storage_entry(1, false),
                    uniform_entry(2),
                ],
            });

        let eml_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("eml_phase5_pipeline_layout"),
            bind_group_layouts: &[&eml_bind_group_layout],
            push_constant_ranges: &[],
        });

        let hardcoded_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("eml_phase5_hardcoded_pipeline_layout"),
            bind_group_layouts: &[&hardcoded_bind_group_layout],
            push_constant_ranges: &[],
        });

        let eml_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("eml_phase5_pipeline"),
            layout: Some(&eml_pipeline_layout),
            module: &eml_shader,
            entry_point: "main",
            compilation_options: Default::default(),
            cache: None,
        });

        let hardcoded_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("eml_phase5_hardcoded_pipeline"),
            layout: Some(&hardcoded_pipeline_layout),
            module: &hardcoded_shader,
            entry_point: "main",
            compilation_options: Default::default(),
            cache: None,
        });

        Ok(Self {
            device,
            queue,
            eml_pipeline,
            hardcoded_pipeline,
            eml_bind_group_layout,
            hardcoded_bind_group_layout,
            cached_node_buffer: None,
        })
    }

    fn get_or_upload_node_buffer(&mut self, nodes: &[EmlNode]) -> Result<()> {
        let needs_upload = match &self.cached_node_buffer {
            Some((cached, _)) => cached.as_slice() != nodes,
            None => true,
        };

        if needs_upload {
            let buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("eml_phase5_nodes"),
                    contents: bytemuck::cast_slice(nodes),
                    usage: BufferUsages::STORAGE,
                });
            self.cached_node_buffer = Some((nodes.to_vec(), buffer));
        }

        Ok(())
    }

    pub fn eval_eml(
        &mut self,
        inputs: &[IntensityInput],
        nodes: &[EmlNode],
        root_node: u32,
    ) -> Result<Vec<IntensityOutput>> {
        validate_finite_inputs(inputs)?;
        validate_nodes(nodes, root_node)?;

        if inputs.is_empty() {
            return Ok(Vec::new());
        }

        self.get_or_upload_node_buffer(nodes)?;
        let node_buffer = &self.cached_node_buffer.as_ref().unwrap().1;

        let n_slots = inputs.len();
        let params = EmlParams {
            n_slots: n_slots as u32,
            root_node,
            _pad0: 0,
            _pad1: 0,
        };

        let input_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("eml_phase5_inputs"),
                contents: bytemuck::cast_slice(inputs),
                usage: BufferUsages::STORAGE,
            });

        let output_size = (n_slots * std::mem::size_of::<IntensityOutput>()) as u64;
        let output_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("eml_phase5_outputs"),
            size: output_size,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let params_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("eml_phase5_params"),
                contents: bytemuck::bytes_of(&params),
                usage: BufferUsages::UNIFORM,
            });

        let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("eml_phase5_bind_group"),
            layout: &self.eml_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: input_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: node_buffer.as_entire_binding(),
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

        let workgroups = ((n_slots as u32) + 63) / 64;
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("eml_phase5_encoder"),
            });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("eml_phase5_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.eml_pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups(workgroups, 1, 1);
        }

        let readback = self.device.create_buffer(&BufferDescriptor {
            label: Some("eml_phase5_readback"),
            size: output_size,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        encoder.copy_buffer_to_buffer(&output_buffer, 0, &readback, 0, output_size);
        self.queue.submit(Some(encoder.finish()));

        let outputs = Self::readback_outputs(&self.device, &readback, n_slots)?;
        validate_finite_outputs(&outputs)?;
        Ok(outputs)
    }

    pub fn eval_hardcoded(
        &self,
        inputs: &[IntensityInput],
        params: IntensityFormulaParams,
    ) -> Result<Vec<IntensityOutput>> {
        validate_finite_inputs(inputs)?;

        if inputs.is_empty() {
            return Ok(Vec::new());
        }

        let n_slots = inputs.len();
        let mut gpu_params = params;
        gpu_params.n_slots = n_slots as u32;

        let input_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("eml_phase5_hardcoded_inputs"),
                contents: bytemuck::cast_slice(inputs),
                usage: BufferUsages::STORAGE,
            });

        let output_size = (n_slots * std::mem::size_of::<IntensityOutput>()) as u64;
        let output_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("eml_phase5_hardcoded_outputs"),
            size: output_size,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let params_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("eml_phase5_hardcoded_params"),
                contents: bytemuck::bytes_of(&gpu_params),
                usage: BufferUsages::UNIFORM,
            });

        let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("eml_phase5_hardcoded_bind_group"),
            layout: &self.hardcoded_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: input_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: output_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: params_buffer.as_entire_binding(),
                },
            ],
        });

        let workgroups = ((n_slots as u32) + 63) / 64;
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("eml_phase5_hardcoded_encoder"),
            });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("eml_phase5_hardcoded_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.hardcoded_pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups(workgroups, 1, 1);
        }

        let readback = self.device.create_buffer(&BufferDescriptor {
            label: Some("eml_phase5_hardcoded_readback"),
            size: output_size,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        encoder.copy_buffer_to_buffer(&output_buffer, 0, &readback, 0, output_size);
        self.queue.submit(Some(encoder.finish()));

        let outputs = Self::readback_outputs(&self.device, &readback, n_slots)?;
        validate_finite_outputs(&outputs)?;
        Ok(outputs)
    }

    fn readback_outputs(
        device: &Device,
        readback: &Buffer,
        n_slots: usize,
    ) -> Result<Vec<IntensityOutput>> {
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
        let outputs: Vec<IntensityOutput> =
            bytemuck::cast_slice(&mapped[..n_slots * std::mem::size_of::<IntensityOutput>()])
                .to_vec();
        drop(mapped);
        readback.unmap();
        Ok(outputs)
    }
}

pub fn run_gpu_eval_intensity(
    inputs: &[IntensityInput],
    nodes: &[EmlNode],
    root_node: u32,
) -> Result<Vec<IntensityOutput>> {
    let mut harness = EmlGpuHarness::new()?;
    harness.eval_eml(inputs, nodes, root_node)
}

pub fn compare_cpu_gpu_rich(
    inputs: &[IntensityInput],
    nodes: &[EmlNode],
    root_node: u32,
    formula_params: IntensityFormulaParams,
) -> Result<EmlGpuRichReport> {
    let mut harness = EmlGpuHarness::new()?;
    compare_cpu_gpu_rich_with_harness(&mut harness, inputs, nodes, root_node, formula_params)
}

pub fn compare_cpu_gpu_rich_with_harness(
    harness: &mut EmlGpuHarness,
    inputs: &[IntensityInput],
    nodes: &[EmlNode],
    root_node: u32,
    formula_params: IntensityFormulaParams,
) -> Result<EmlGpuRichReport> {
    validate_finite_inputs(inputs)?;
    validate_nodes(nodes, root_node)?;

    let t0 = Instant::now();
    let cpu_node_outputs: Vec<f32> = inputs
        .iter()
        .map(|input| eval_cpu_node(nodes, root_node, *input))
        .collect();
    let cpu_node_eval_us = t0.elapsed().as_micros() as u64;

    let t1 = Instant::now();
    let cpu_direct_outputs: Vec<f32> = inputs
        .iter()
        .map(|input| {
            intensity_update_direct_cpu(
                *input,
                formula_params.velocity_threshold,
                formula_params.build_coefficient,
                formula_params.decay_coefficient,
                formula_params.dt,
            )
        })
        .collect();
    let cpu_direct_eval_us = t1.elapsed().as_micros() as u64;

    let cold_start = Instant::now();
    let eml_cold_outputs = harness.eval_eml(inputs, nodes, root_node)?;
    let gpu_eml_cold_total_us = cold_start.elapsed().as_micros() as u64;

    let mut eml_warm_samples = Vec::with_capacity(WARM_RUNS);
    let mut eml_warm_baseline: Option<Vec<IntensityOutput>> = None;
    let mut eml_repeated_runs_identical = true;
    for _ in 0..WARM_RUNS {
        let t = Instant::now();
        let repeat = harness.eval_eml(inputs, nodes, root_node)?;
        eml_warm_samples.push(t.elapsed().as_micros() as u64);
        match &eml_warm_baseline {
            None => eml_warm_baseline = Some(repeat),
            Some(base) if !outputs_identical(&repeat, base) => {
                eml_repeated_runs_identical = false;
            }
            _ => {}
        }
    }

    let mut hardcoded_warm_samples = Vec::with_capacity(WARM_RUNS);
    let mut hardcoded_baseline: Option<Vec<IntensityOutput>> = None;
    let mut hardcoded_repeated_runs_identical = true;
    for _ in 0..WARM_RUNS {
        let t = Instant::now();
        let repeat = harness.eval_hardcoded(inputs, formula_params)?;
        hardcoded_warm_samples.push(t.elapsed().as_micros() as u64);
        match &hardcoded_baseline {
            None => hardcoded_baseline = Some(repeat),
            Some(base) if !outputs_identical(&repeat, base) => {
                hardcoded_repeated_runs_identical = false;
            }
            _ => {}
        }
    }

    let hardcoded_outputs = hardcoded_baseline.unwrap_or_default();

    let (eml_vs_cpu_max_abs_error, eml_vs_cpu_mean_abs_error) =
        compare_errors(&cpu_node_outputs, &eml_cold_outputs);
    let (hardcoded_vs_cpu_max_abs_error, hardcoded_vs_cpu_mean_abs_error) =
        compare_errors(&cpu_direct_outputs, &hardcoded_outputs);
    let (eml_vs_hardcoded_max_abs_error, eml_vs_hardcoded_mean_abs_error) =
        compare_output_pairs(&eml_cold_outputs, &hardcoded_outputs);

    let (gpu_eml_warm_mean_us, gpu_eml_warm_min_us, gpu_eml_warm_max_us) =
        warm_stats(&eml_warm_samples);
    let (gpu_hardcoded_warm_mean_us, gpu_hardcoded_warm_min_us, gpu_hardcoded_warm_max_us) =
        warm_stats(&hardcoded_warm_samples);

    let eml_vs_hardcoded_overhead_ratio = if gpu_hardcoded_warm_mean_us == 0 {
        0.0
    } else {
        gpu_eml_warm_mean_us as f32 / gpu_hardcoded_warm_mean_us as f32
    };

    Ok(EmlGpuRichReport {
        n_slots: inputs.len(),
        cpu_node_eval_us,
        cpu_direct_eval_us,
        gpu_eml_cold_total_us,
        gpu_eml_warm_mean_us,
        gpu_eml_warm_min_us,
        gpu_eml_warm_max_us,
        gpu_hardcoded_warm_mean_us,
        gpu_hardcoded_warm_min_us,
        gpu_hardcoded_warm_max_us,
        eml_vs_cpu_max_abs_error,
        eml_vs_cpu_mean_abs_error,
        hardcoded_vs_cpu_max_abs_error,
        hardcoded_vs_cpu_mean_abs_error,
        eml_vs_hardcoded_max_abs_error,
        eml_vs_hardcoded_mean_abs_error,
        eml_vs_hardcoded_overhead_ratio,
        eml_repeated_runs_identical,
        hardcoded_repeated_runs_identical,
        warm_runs: WARM_RUNS,
        timing_note: TIMING_NOTE.to_string(),
    })
}

pub fn compare_cpu_gpu(
    inputs: &[IntensityInput],
    nodes: &[EmlNode],
    root_node: u32,
) -> Result<EmlGpuReport> {
    let formula_params = IntensityFormulaParams::new(inputs.len() as u32, 0.0, 0.0, 0.0, 0.0);
    let rich = compare_cpu_gpu_rich(inputs, nodes, root_node, formula_params)?;
    Ok(EmlGpuReport {
        n_slots: rich.n_slots,
        max_abs_error: rich.eml_vs_cpu_max_abs_error,
        mean_abs_error: rich.eml_vs_cpu_mean_abs_error,
        gpu_eval_us: rich.gpu_eml_cold_total_us,
        cpu_eval_us: rich.cpu_node_eval_us,
        repeated_runs_identical: rich.eml_repeated_runs_identical,
    })
}

pub fn format_rich_report(report: &EmlGpuRichReport) -> String {
    crate::report::format_rich_report(report)
}
