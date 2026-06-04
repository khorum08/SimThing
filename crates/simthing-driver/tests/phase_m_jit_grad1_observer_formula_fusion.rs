//! Phase M-JIT-GRAD-1 — GPU-resident observer + exact formula fusion prototype (Tier-2, test-only).
//!
//! Extends M-JIT-GRAD-0 by fusing spatial field sampling (`dx`/`dy`/descent) with an
//! exact-subset generic score in one GPU dispatch. Score uses exact-authoritative observer
//! outputs only — not approximate `mag2`. No `sqrt`, no production wiring.

use std::sync::Mutex;
use std::time::Instant;

use bytemuck::{Pod, Zeroable};
use simthing_gpu::GpuContext;
use simthing_spec::MappingExecutionProfile;
use wgpu::util::DeviceExt;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const WORKGROUP_SIZE: u32 = 64;
const BOUNDARY_CLAMP: u32 = 1;

const W0: f32 = 0.65;
const W1: f32 = 0.35;
const BIAS: f32 = 0.125;

const FORBIDDEN_SEMANTIC_TERMS: &[&str] = &[
    "faction",
    "ownership",
    "owner",
    "AI",
    "threat",
    "scarcity",
    "opportunity",
    "labor",
    "price",
    "logistics",
    "routing",
    "need",
    "demand",
    "supply",
    "personality",
    "drone",
    "SEAD",
    "simthing-sim",
    "ResourceEconomySpec",
    "SimSession",
];

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct FusionParams {
    width: u32,
    height: u32,
    n_dims: u32,
    n_observers: u32,
    boundary_mode: u32,
    w0_bits: u32,
    w1_bits: u32,
    bias_bits: u32,
    _pad0: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct ObserverInput {
    x: u32,
    y: u32,
    source_col: u32,
    _pad: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, PartialEq)]
struct ObserverScoreOutput {
    dx: f32,
    dy: f32,
    descent_x: f32,
    descent_y: f32,
    score: f32,
    _pad0: f32,
    _pad1: f32,
    _pad2: f32,
}

struct FusionRunResult {
    outputs: Vec<ObserverScoreOutput>,
    dispatch_count: u32,
    elapsed_ms: f64,
}

/// Option B lite: exact-subset score expression emitted with bitcast literals (M-JIT-0 style).
fn emit_exact_subset_score_wgsl(w0_bits: u32, w1_bits: u32, bias_bits: u32) -> String {
    format!(
        "    let w0 = bitcast<f32>({w0_bits}u);\n\
         let w1 = bitcast<f32>({w1_bits}u);\n\
         let bias = bitcast<f32>({bias_bits}u);\n\
         let score = fma(w0, descent_x, fma(w1, descent_y, bias));\n"
    )
}

fn build_fused_observer_score_wgsl(w0_bits: u32, w1_bits: u32, bias_bits: u32) -> String {
    let score_body = emit_exact_subset_score_wgsl(w0_bits, w1_bits, bias_bits);
    format!(
        r#"struct FusionParams {{
    width: u32,
    height: u32,
    n_dims: u32,
    n_observers: u32,
    boundary_mode: u32,
    w0_bits: u32,
    w1_bits: u32,
    bias_bits: u32,
    _pad0: u32,
}}

struct ObserverInput {{
    x: u32,
    y: u32,
    source_col: u32,
    _pad: u32,
}}

struct ObserverScoreOutput {{
    dx: f32,
    dy: f32,
    descent_x: f32,
    descent_y: f32,
    score: f32,
    _pad0: f32,
    _pad1: f32,
    _pad2: f32,
}}

@group(0) @binding(0) var<uniform> params: FusionParams;
@group(0) @binding(1) var<storage, read> fields: array<f32>;
@group(0) @binding(2) var<storage, read> observers: array<ObserverInput>;
@group(0) @binding(3) var<storage, read_write> outputs: array<ObserverScoreOutput>;

fn sample_field(x: i32, y: i32, source_col: u32) -> f32 {{
    if params.boundary_mode == 1u {{
        let cx = clamp(x, 0, i32(params.width) - 1);
        let cy = clamp(y, 0, i32(params.height) - 1);
        let idx = u32(cy) * params.width + u32(cx);
        let base = idx * params.n_dims;
        return fields[base + source_col];
    }}
    if x < 0 || y < 0 || x >= i32(params.width) || y >= i32(params.height) {{
        return 0.0;
    }}
    let idx = u32(y) * params.width + u32(x);
    let base = idx * params.n_dims;
    return fields[base + source_col];
}}

@compute @workgroup_size(64, 1, 1)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {{
    let observer_id = gid.x;
    if observer_id >= params.n_observers {{
        return;
    }}
    let obs = observers[observer_id];
    let ix = i32(obs.x);
    let iy = i32(obs.y);
    let sc = obs.source_col;
    let west = sample_field(ix - 1, iy, sc);
    let east = sample_field(ix + 1, iy, sc);
    let north = sample_field(ix, iy - 1, sc);
    let south = sample_field(ix, iy + 1, sc);
    let dx = 0.5 * (east - west);
    let dy = 0.5 * (south - north);
    let descent_x = -dx;
    let descent_y = -dy;
{score_body}    var out: ObserverScoreOutput;
    out.dx = dx;
    out.dy = dy;
    out.descent_x = descent_x;
    out.descent_y = descent_y;
    out.score = score;
    out._pad0 = 0.0;
    out._pad1 = 0.0;
    out._pad2 = 0.0;
    outputs[observer_id] = out;
}}
"#
    )
}

fn fused_wgsl() -> String {
    build_fused_observer_score_wgsl(W0.to_bits(), W1.to_bits(), BIAS.to_bits())
}

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let ctx = GpuContext::new_blocking().expect("GPU required");
    f(&ctx);
}

fn assert_shader_semantic_free(wgsl: &str) {
    for term in FORBIDDEN_SEMANTIC_TERMS {
        assert!(
            !wgsl.contains(term),
            "fused WGSL must be semantic-free; found `{term}`"
        );
    }
    assert!(
        !wgsl.contains("sqrt("),
        "fused exact path must not use sqrt"
    );
}

fn field_index(x: u32, y: u32, width: u32, n_dims: u32, col: u32) -> usize {
    ((y * width + x) * n_dims + col) as usize
}

fn sample_field_cpu(
    fields: &[f32],
    width: u32,
    height: u32,
    n_dims: u32,
    x: i32,
    y: i32,
    source_col: u32,
) -> f32 {
    let cx = x.clamp(0, width as i32 - 1) as u32;
    let cy = y.clamp(0, height as i32 - 1) as u32;
    fields[field_index(cx, cy, width, n_dims, source_col)]
}

fn cpu_score(descent_x: f32, descent_y: f32) -> f32 {
    W0.mul_add(descent_x, W1.mul_add(descent_y, BIAS))
}

fn cpu_fusion_oracle(
    fields: &[f32],
    width: u32,
    height: u32,
    n_dims: u32,
    obs: ObserverInput,
) -> ObserverScoreOutput {
    let ix = obs.x as i32;
    let iy = obs.y as i32;
    let sc = obs.source_col;
    let west = sample_field_cpu(fields, width, height, n_dims, ix - 1, iy, sc);
    let east = sample_field_cpu(fields, width, height, n_dims, ix + 1, iy, sc);
    let north = sample_field_cpu(fields, width, height, n_dims, ix, iy - 1, sc);
    let south = sample_field_cpu(fields, width, height, n_dims, ix, iy + 1, sc);
    let dx = 0.5 * (east - west);
    let dy = 0.5 * (south - north);
    let descent_x = -dx;
    let descent_y = -dy;
    ObserverScoreOutput {
        dx,
        dy,
        descent_x,
        descent_y,
        score: cpu_score(descent_x, descent_y),
        _pad0: 0.0,
        _pad1: 0.0,
        _pad2: 0.0,
    }
}

fn build_test_field(width: u32, height: u32, n_dims: u32, source_col: u32) -> Vec<f32> {
    let cells = (width * height * n_dims) as usize;
    let mut fields = vec![0.0f32; cells];
    for y in 0..height {
        for x in 0..width {
            let v = x as f32 * 1.5 + y as f32 * 2.25 + (x as f32 * y as f32) * 0.01;
            fields[field_index(x, y, width, n_dims, source_col)] = v;
        }
    }
    fields
}

fn run_fusion_gpu(
    ctx: &GpuContext,
    wgsl: &str,
    fields: &[f32],
    observers: &[ObserverInput],
    width: u32,
    height: u32,
    n_dims: u32,
) -> FusionRunResult {
    let device = &ctx.device;
    let queue = &ctx.queue;
    let n_observers = observers.len() as u32;

    let params = FusionParams {
        width,
        height,
        n_dims,
        n_observers,
        boundary_mode: BOUNDARY_CLAMP,
        w0_bits: W0.to_bits(),
        w1_bits: W1.to_bits(),
        bias_bits: BIAS.to_bits(),
        _pad0: 0,
    };

    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("jit_grad1_fusion"),
        source: wgpu::ShaderSource::Wgsl(wgsl.into()),
    });

    let uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("jit_grad1_params"),
        contents: bytemuck::bytes_of(&params),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let fields_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("jit_grad1_fields"),
        contents: bytemuck::cast_slice(fields),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let observers_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("jit_grad1_observers"),
        contents: bytemuck::cast_slice(observers),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let outputs_len = (observers.len() * std::mem::size_of::<ObserverScoreOutput>()) as u64;
    let outputs_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("jit_grad1_outputs"),
        size: outputs_len,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("jit_grad1_bgl"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("jit_grad1_pipeline"),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("jit_grad1_pl"),
                bind_group_layouts: &[&bgl],
                push_constant_ranges: &[],
            }),
        ),
        module: &module,
        entry_point: "main",
        compilation_options: Default::default(),
        cache: None,
    });

    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("jit_grad1_bg"),
        layout: &bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: fields_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: observers_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: outputs_buf.as_entire_binding(),
            },
        ],
    });

    let workgroups = n_observers.div_ceil(WORKGROUP_SIZE);
    let started = Instant::now();

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("jit_grad1_enc"),
    });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("jit_grad1_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bg, &[]);
        pass.dispatch_workgroups(workgroups, 1, 1);
    }
    queue.submit(Some(encoder.finish()));

    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("jit_grad1_readback"),
        size: outputs_len,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("jit_grad1_readback_enc"),
    });
    enc2.copy_buffer_to_buffer(&outputs_buf, 0, &staging, 0, outputs_len);
    queue.submit(Some(enc2.finish()));

    let slice = staging.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);
    let data = slice.get_mapped_range();
    let outputs: Vec<ObserverScoreOutput> = bytemuck::cast_slice(&data).to_vec();
    drop(data);
    staging.unmap();

    FusionRunResult {
        outputs,
        dispatch_count: 1,
        elapsed_ms: started.elapsed().as_secs_f64() * 1000.0,
    }
}

fn assert_fusion_parity(
    fields: &[f32],
    width: u32,
    height: u32,
    n_dims: u32,
    observers: &[ObserverInput],
    gpu_outputs: &[ObserverScoreOutput],
    context: &str,
) {
    assert_eq!(
        gpu_outputs.len(),
        observers.len(),
        "{context}: length mismatch"
    );
    for (i, obs) in observers.iter().enumerate() {
        let cpu = cpu_fusion_oracle(fields, width, height, n_dims, *obs);
        let gpu = gpu_outputs[i];
        assert_eq!(
            gpu.dx.to_bits(),
            cpu.dx.to_bits(),
            "{context} observer {i} dx"
        );
        assert_eq!(
            gpu.dy.to_bits(),
            cpu.dy.to_bits(),
            "{context} observer {i} dy"
        );
        assert_eq!(
            gpu.descent_x.to_bits(),
            cpu.descent_x.to_bits(),
            "{context} observer {i} descent_x"
        );
        assert_eq!(
            gpu.descent_y.to_bits(),
            cpu.descent_y.to_bits(),
            "{context} observer {i} descent_y"
        );
        assert_eq!(
            gpu.score.to_bits(),
            cpu.score.to_bits(),
            "{context} observer {i} score"
        );
    }
}

fn small_grid_observers(width: u32, height: u32, source_col: u32) -> Vec<ObserverInput> {
    let mk = |x: u32, y: u32| ObserverInput {
        x,
        y,
        source_col,
        _pad: 0,
    };
    vec![
        mk(width / 2, height / 2),
        mk(0, 0),
        mk(width - 1, 0),
        mk(0, height - 1),
        mk(width - 1, height - 1),
        mk(width / 2, 0),
        mk(0, height / 2),
        mk(width - 1, height / 2),
        mk(width / 2, height - 1),
        mk(1, 1),
        mk(width - 2, height - 2),
    ]
}

fn structured_observers_10000(width: u32, height: u32, source_col: u32) -> Vec<ObserverInput> {
    (0..10_000u32)
        .map(|i| ObserverInput {
            x: (i * 997) % width,
            y: (i * 313) % height,
            source_col,
            _pad: 0,
        })
        .collect()
}

fn oracle_sample_indices(n_observers: usize) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..16.min(n_observers)).collect();
    if n_observers > 16 {
        indices.extend((n_observers - 16)..n_observers);
    }
    for k in 0..32 {
        indices.push((k * 997) % n_observers);
    }
    indices.sort_unstable();
    indices.dedup();
    indices
}

#[test]
fn jit_grad1_fused_observer_score_shader_is_semantic_free() {
    let wgsl = fused_wgsl();
    assert_shader_semantic_free(&wgsl);
    assert!(wgsl.contains("fields"));
    assert!(wgsl.contains("observers"));
    assert!(wgsl.contains("outputs"));
    assert!(wgsl.contains("score"));
    assert!(wgsl.contains("descent_x"));
    assert!(wgsl.contains("bitcast<f32>"));
    assert!(
        !wgsl.contains("mag2"),
        "exact fused path must not emit mag2"
    );
}

#[test]
fn jit_grad1_small_grid_observer_score_parity() {
    with_gpu(|ctx| {
        let wgsl = fused_wgsl();
        let width = 8u32;
        let height = 8u32;
        let n_dims = 8u32;
        let source_col = 2u32;
        let fields = build_test_field(width, height, n_dims, source_col);
        let observers = small_grid_observers(width, height, source_col);
        let result = run_fusion_gpu(ctx, &wgsl, &fields, &observers, width, height, n_dims);
        assert_eq!(result.dispatch_count, 1);
        assert_fusion_parity(
            &fields,
            width,
            height,
            n_dims,
            &observers,
            &result.outputs,
            "small_grid",
        );
        println!(
            "small_grid_fusion: observers={}, dispatch_count={}, elapsed_ms={:.3}",
            observers.len(),
            result.dispatch_count,
            result.elapsed_ms
        );
    });
}

#[test]
fn jit_grad1_batches_10000_observer_scores_one_dispatch() {
    with_gpu(|ctx| {
        let wgsl = fused_wgsl();
        let width = 128u32;
        let height = 128u32;
        let n_dims = 4u32;
        let source_col = 0u32;
        let fields = build_test_field(width, height, n_dims, source_col);
        let observers = structured_observers_10000(width, height, source_col);
        assert!(observers.len() >= 10_000);

        let result = run_fusion_gpu(ctx, &wgsl, &fields, &observers, width, height, n_dims);
        assert_eq!(result.outputs.len(), 10_000);
        assert_eq!(result.dispatch_count, 1);

        let sample_indices = oracle_sample_indices(observers.len());
        let sampled_obs: Vec<ObserverInput> =
            sample_indices.iter().map(|&i| observers[i]).collect();
        let sampled_out: Vec<ObserverScoreOutput> =
            sample_indices.iter().map(|&i| result.outputs[i]).collect();
        assert_fusion_parity(
            &fields,
            width,
            height,
            n_dims,
            &sampled_obs,
            &sampled_out,
            "batch_10000_sample",
        );

        let workgroups = 10_000u32.div_ceil(WORKGROUP_SIZE);
        println!(
            "batch_10000_fusion: observers={}, dispatch_count={}, workgroups={}, workgroup_size={}, elapsed_ms={:.3}",
            observers.len(),
            result.dispatch_count,
            workgroups,
            WORKGROUP_SIZE,
            result.elapsed_ms
        );
    });
}

#[test]
fn jit_grad1_score_excludes_approximate_mag2() {
    let wgsl = fused_wgsl();
    assert!(
        !wgsl.contains("mag2"),
        "score formula must not reference approximate mag2"
    );
    let score_emit = emit_exact_subset_score_wgsl(W0.to_bits(), W1.to_bits(), BIAS.to_bits());
    assert!(!score_emit.contains("mag2"));
    assert!(score_emit.contains("descent_x"));
    assert!(score_emit.contains("descent_y"));
    assert!(!score_emit.contains("sqrt("));
    // M-JIT-GRAD-0 R1 posture: mag2 is diagnostic-only in GRAD-0; GRAD-1 fused output has no mag2 field.
    assert!(std::mem::size_of::<ObserverScoreOutput>() >= 32);
}

#[test]
fn jit_grad1_default_off_posture() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    assert_shader_semantic_free(&fused_wgsl());
    assert!(
        !fused_wgsl().contains("ResourceEconomySpec"),
        "fused WGSL must not reference ResourceEconomySpec"
    );
}
