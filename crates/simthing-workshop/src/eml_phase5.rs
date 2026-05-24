//! EML Phase 5 intensity-update spike — hand-authored expression tree, CPU + GPU evaluators.

use std::time::Instant;

use anyhow::{bail, Context, Result};
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;
use wgpu::{
    Backends, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, BufferBindingType, BufferDescriptor, BufferUsages,
    CommandEncoderDescriptor, ComputePipelineDescriptor, DeviceDescriptor, Features, Instance,
    InstanceDescriptor, Limits, Maintain, MapMode, MemoryHints, PipelineLayoutDescriptor,
    PowerPreference, RequestAdapterOptions, ShaderModuleDescriptor, ShaderStages,
};

pub const MAX_NODES: usize = 32;

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

pub fn run_gpu_eval_intensity(
    inputs: &[IntensityInput],
    nodes: &[EmlNode],
    root_node: u32,
) -> Result<Vec<IntensityOutput>> {
    validate_finite_inputs(inputs)?;
    assert!(nodes.len() <= MAX_NODES);
    assert!((root_node as usize) < nodes.len());

    let n_slots = inputs.len();
    let params = EmlParams {
        n_slots: n_slots as u32,
        root_node,
        _pad0: 0,
        _pad1: 0,
    };

    pollster::block_on(async {
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

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("eml_phase5"),
            source: wgpu::ShaderSource::Wgsl(include_str!("eml_phase5.wgsl").into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("eml_phase5_layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 3,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("eml_phase5_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("eml_phase5_pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "main",
            compilation_options: Default::default(),
            cache: None,
        });

        let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("eml_phase5_inputs"),
            contents: bytemuck::cast_slice(inputs),
            usage: BufferUsages::STORAGE,
        });

        let node_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("eml_phase5_nodes"),
            contents: bytemuck::cast_slice(nodes),
            usage: BufferUsages::STORAGE,
        });

        let output_size = (n_slots * std::mem::size_of::<IntensityOutput>()) as u64;
        let output_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("eml_phase5_outputs"),
            size: output_size.max(4),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let params_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("eml_phase5_params"),
            contents: bytemuck::bytes_of(&params),
            usage: BufferUsages::UNIFORM,
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("eml_phase5_bind_group"),
            layout: &bind_group_layout,
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
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("eml_phase5_encoder"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("eml_phase5_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups(workgroups, 1, 1);
        }

        let readback = device.create_buffer(&BufferDescriptor {
            label: Some("eml_phase5_readback"),
            size: output_size.max(4),
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        encoder.copy_buffer_to_buffer(&output_buffer, 0, &readback, 0, output_size.max(4));
        queue.submit(Some(encoder.finish()));

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
        let outputs: Vec<IntensityOutput> = bytemuck::cast_slice(&mapped).to_vec();
        drop(mapped);
        readback.unmap();

        validate_finite_outputs(&outputs)?;
        Ok(outputs)
    })
}

pub fn compare_cpu_gpu(
    inputs: &[IntensityInput],
    nodes: &[EmlNode],
    root_node: u32,
) -> Result<EmlGpuReport> {
    validate_finite_inputs(inputs)?;

    let t0 = Instant::now();
    let cpu_outputs: Vec<f32> = inputs
        .iter()
        .map(|input| eval_cpu_node(nodes, root_node, *input))
        .collect();
    let cpu_eval_us = t0.elapsed().as_micros() as u64;

    let t1 = Instant::now();
    let gpu_outputs = run_gpu_eval_intensity(inputs, nodes, root_node)?;
    let gpu_eval_us = t1.elapsed().as_micros() as u64;

    let mut max_abs_error = 0.0f32;
    let mut sum_abs_error = 0.0f32;
    for (cpu, gpu) in cpu_outputs.iter().zip(gpu_outputs.iter()) {
        let err = (cpu - gpu.value).abs();
        max_abs_error = max_abs_error.max(err);
        sum_abs_error += err;
    }
    let mean_abs_error = if inputs.is_empty() {
        0.0
    } else {
        sum_abs_error / inputs.len() as f32
    };

    let mut repeated_runs_identical = true;
    for _ in 1..10 {
        let repeat = run_gpu_eval_intensity(inputs, nodes, root_node)?;
        let bytes_match = bytemuck::cast_slice::<IntensityOutput, u8>(&repeat)
            == bytemuck::cast_slice::<IntensityOutput, u8>(&gpu_outputs);
        if bytes_match {
            continue;
        }
        let f32_match = repeat
            .iter()
            .zip(gpu_outputs.iter())
            .all(|(a, b)| a.value == b.value);
        if !f32_match {
            repeated_runs_identical = false;
            break;
        }
    }

    Ok(EmlGpuReport {
        n_slots: inputs.len(),
        max_abs_error,
        mean_abs_error,
        gpu_eval_us,
        cpu_eval_us,
        repeated_runs_identical,
    })
}
