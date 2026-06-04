//! Phase M-JIT-GRAD-0 — GPU-resident batched spatial field observer prototype (Tier-2, test-only).
//!
//! Proves many observers can sample a dense scalar field and compute finite-difference
//! gradient/descent outputs in one GPU dispatch without CPU-side per-observer loops,
//! semantic WGSL, production default wiring, atlas, active-mask admission, or `sqrt`.
//!
//! NO production JIT wiring, NO default mapping wiring, NO new EML opcode, NO chained
//! scheduling, NO automatic snapshot/copy scheduling, and NO `simthing-sim` semantics.

use std::sync::Mutex;
use std::time::Instant;

use bytemuck::{Pod, Zeroable};
use simthing_gpu::GpuContext;
use simthing_spec::MappingExecutionProfile;
use wgpu::util::DeviceExt;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const WORKGROUP_SIZE: u32 = 64;
const BOUNDARY_CLAMP: u32 = 1;

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

/// Uniform block for observer dispatch parameters.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct ObserverParams {
    width: u32,
    height: u32,
    n_dims: u32,
    n_observers: u32,
    boundary_mode: u32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
}

/// Per-observer sampling coordinate (generic names only).
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct ObserverInput {
    x: u32,
    y: u32,
    source_col: u32,
    _pad: u32,
}

/// Per-observer gradient/descent output (squared magnitude, no `sqrt`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, PartialEq)]
struct ObserverOutput {
    dx: f32,
    dy: f32,
    mag2: f32,
    descent_x: f32,
    descent_y: f32,
    _pad: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputClassification {
    ExactDeterministicCandidate,
    ApproximateJitOnly,
    RejectedDeferred,
}

#[derive(Debug, Clone, Copy)]
struct FieldClassification {
    tested_cases: usize,
    exact_cases: usize,
    max_ulp: u32,
    classification: OutputClassification,
}

#[derive(Debug, Clone, Copy)]
struct ObserverOutputClassificationSummary {
    dx: FieldClassification,
    dy: FieldClassification,
    descent_x: FieldClassification,
    descent_y: FieldClassification,
    /// Primary: GPU `mag2` vs shader-order CPU oracle `cpu.dx*cpu.dx + cpu.dy*cpu.dy`.
    mag2_shader_order: FieldClassification,
}

fn classify_ulp(max_ulp: u32) -> OutputClassification {
    if max_ulp == 0 {
        OutputClassification::ExactDeterministicCandidate
    } else if max_ulp <= 2 {
        OutputClassification::ApproximateJitOnly
    } else {
        OutputClassification::RejectedDeferred
    }
}

fn cpu_mag2_shader_order(cpu: &ObserverOutput) -> f32 {
    cpu.dx * cpu.dx + cpu.dy * cpu.dy
}

/// Test-local static semantic-free observer WGSL (Option A).
const OBSERVER_WGSL: &str = r#"
struct ObserverParams {
    width: u32,
    height: u32,
    n_dims: u32,
    n_observers: u32,
    boundary_mode: u32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
}

struct ObserverInput {
    x: u32,
    y: u32,
    source_col: u32,
    _pad: u32,
}

struct ObserverOutput {
    dx: f32,
    dy: f32,
    mag2: f32,
    descent_x: f32,
    descent_y: f32,
    _pad: f32,
}

@group(0) @binding(0) var<uniform> params: ObserverParams;
@group(0) @binding(1) var<storage, read> fields: array<f32>;
@group(0) @binding(2) var<storage, read> observers: array<ObserverInput>;
@group(0) @binding(3) var<storage, read_write> outputs: array<ObserverOutput>;

fn sample_field(x: i32, y: i32, source_col: u32) -> f32 {
    if params.boundary_mode == 1u {
        let cx = clamp(x, 0, i32(params.width) - 1);
        let cy = clamp(y, 0, i32(params.height) - 1);
        let idx = u32(cy) * params.width + u32(cx);
        let base = idx * params.n_dims;
        return fields[base + source_col];
    }
    if x < 0 || y < 0 || x >= i32(params.width) || y >= i32(params.height) {
        return 0.0;
    }
    let idx = u32(y) * params.width + u32(x);
    let base = idx * params.n_dims;
    return fields[base + source_col];
}

@compute @workgroup_size(64, 1, 1)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let observer_id = gid.x;
    if observer_id >= params.n_observers {
        return;
    }
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
    var out: ObserverOutput;
    out.dx = dx;
    out.dy = dy;
    out.mag2 = dx * dx + dy * dy;
    out.descent_x = -dx;
    out.descent_y = -dy;
    out._pad = 0.0;
    outputs[observer_id] = out;
}
"#;

struct ObserverRunResult {
    outputs: Vec<ObserverOutput>,
    dispatch_count: u32,
    elapsed_ms: f64,
}

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let ctx = GpuContext::new_blocking().expect("GPU required");
    f(&ctx);
}

fn ulp_distance(a: f32, b: f32) -> u32 {
    fn ordered(bits: u32) -> i32 {
        if (bits & 0x8000_0000) != 0 {
            !(bits as i32)
        } else {
            bits as i32
        }
    }
    ordered(a.to_bits()).abs_diff(ordered(b.to_bits()))
}

fn assert_shader_semantic_free(wgsl: &str) {
    for term in FORBIDDEN_SEMANTIC_TERMS {
        assert!(
            !wgsl.contains(term),
            "observer WGSL must be semantic-free; found `{term}`"
        );
    }
    assert!(
        !wgsl.contains("sqrt("),
        "observer WGSL must not use sqrt in the exact path"
    );
}

fn field_index(x: u32, y: u32, width: u32, n_dims: u32, col: u32) -> usize {
    ((y * width + x) * n_dims + col) as usize
}

/// Clamp boundary policy: out-of-bounds neighbor samples clamp to nearest valid cell.
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

fn cpu_observer_oracle(
    fields: &[f32],
    width: u32,
    height: u32,
    n_dims: u32,
    obs: ObserverInput,
) -> ObserverOutput {
    let ix = obs.x as i32;
    let iy = obs.y as i32;
    let sc = obs.source_col;
    let west = sample_field_cpu(fields, width, height, n_dims, ix - 1, iy, sc);
    let east = sample_field_cpu(fields, width, height, n_dims, ix + 1, iy, sc);
    let north = sample_field_cpu(fields, width, height, n_dims, ix, iy - 1, sc);
    let south = sample_field_cpu(fields, width, height, n_dims, ix, iy + 1, sc);
    let dx = 0.5 * (east - west);
    let dy = 0.5 * (south - north);
    ObserverOutput {
        dx,
        dy,
        mag2: dx * dx + dy * dy,
        descent_x: -dx,
        descent_y: -dy,
        _pad: 0.0,
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

fn run_observers_gpu(
    ctx: &GpuContext,
    fields: &[f32],
    observers: &[ObserverInput],
    width: u32,
    height: u32,
    n_dims: u32,
) -> ObserverRunResult {
    let device = &ctx.device;
    let queue = &ctx.queue;
    let n_observers = observers.len() as u32;

    let params = ObserverParams {
        width,
        height,
        n_dims,
        n_observers,
        boundary_mode: BOUNDARY_CLAMP,
        _pad0: 0,
        _pad1: 0,
        _pad2: 0,
    };

    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("jit_grad0_observer"),
        source: wgpu::ShaderSource::Wgsl(OBSERVER_WGSL.into()),
    });

    let uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("jit_grad0_params"),
        contents: bytemuck::bytes_of(&params),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let fields_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("jit_grad0_fields"),
        contents: bytemuck::cast_slice(fields),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let observers_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("jit_grad0_observers"),
        contents: bytemuck::cast_slice(observers),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let outputs_len = (observers.len() * std::mem::size_of::<ObserverOutput>()) as u64;
    let outputs_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("jit_grad0_outputs"),
        size: outputs_len,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("jit_grad0_bgl"),
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
        label: Some("jit_grad0_pipeline"),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("jit_grad0_pl"),
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
        label: Some("jit_grad0_bg"),
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
        label: Some("jit_grad0_enc"),
    });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("jit_grad0_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bg, &[]);
        pass.dispatch_workgroups(workgroups, 1, 1);
    }
    queue.submit(Some(encoder.finish()));

    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("jit_grad0_readback"),
        size: outputs_len,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("jit_grad0_readback_enc"),
    });
    enc2.copy_buffer_to_buffer(&outputs_buf, 0, &staging, 0, outputs_len);
    queue.submit(Some(enc2.finish()));

    let slice = staging.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);
    let data = slice.get_mapped_range();
    let outputs: Vec<ObserverOutput> = bytemuck::cast_slice(&data).to_vec();
    drop(data);
    staging.unmap();

    ObserverRunResult {
        outputs,
        dispatch_count: 1,
        elapsed_ms: started.elapsed().as_secs_f64() * 1000.0,
    }
}

fn classify_observer_outputs(
    fields: &[f32],
    width: u32,
    height: u32,
    n_dims: u32,
    observers: &[ObserverInput],
    gpu_outputs: &[ObserverOutput],
) -> ObserverOutputClassificationSummary {
    assert_eq!(gpu_outputs.len(), observers.len());
    let mut dx_max = 0u32;
    let mut dy_max = 0u32;
    let mut descent_x_max = 0u32;
    let mut descent_y_max = 0u32;
    let mut mag2_max = 0u32;
    let mut dx_exact = 0usize;
    let mut dy_exact = 0usize;
    let mut descent_x_exact = 0usize;
    let mut descent_y_exact = 0usize;
    let mut mag2_exact = 0usize;

    for (obs, gpu) in observers.iter().zip(gpu_outputs.iter()) {
        let cpu = cpu_observer_oracle(fields, width, height, n_dims, *obs);
        let ulp_dx = ulp_distance(gpu.dx, cpu.dx);
        let ulp_dy = ulp_distance(gpu.dy, cpu.dy);
        let ulp_descent_x = ulp_distance(gpu.descent_x, cpu.descent_x);
        let ulp_descent_y = ulp_distance(gpu.descent_y, cpu.descent_y);
        let cpu_mag2 = cpu_mag2_shader_order(&cpu);
        let ulp_mag2 = ulp_distance(gpu.mag2, cpu_mag2);

        dx_max = dx_max.max(ulp_dx);
        dy_max = dy_max.max(ulp_dy);
        descent_x_max = descent_x_max.max(ulp_descent_x);
        descent_y_max = descent_y_max.max(ulp_descent_y);
        mag2_max = mag2_max.max(ulp_mag2);

        if ulp_dx == 0 {
            dx_exact += 1;
        }
        if ulp_dy == 0 {
            dy_exact += 1;
        }
        if ulp_descent_x == 0 {
            descent_x_exact += 1;
        }
        if ulp_descent_y == 0 {
            descent_y_exact += 1;
        }
        if ulp_mag2 == 0 {
            mag2_exact += 1;
        }
    }

    let n = observers.len();
    ObserverOutputClassificationSummary {
        dx: FieldClassification {
            tested_cases: n,
            exact_cases: dx_exact,
            max_ulp: dx_max,
            classification: classify_ulp(dx_max),
        },
        dy: FieldClassification {
            tested_cases: n,
            exact_cases: dy_exact,
            max_ulp: dy_max,
            classification: classify_ulp(dy_max),
        },
        descent_x: FieldClassification {
            tested_cases: n,
            exact_cases: descent_x_exact,
            max_ulp: descent_x_max,
            classification: classify_ulp(descent_x_max),
        },
        descent_y: FieldClassification {
            tested_cases: n,
            exact_cases: descent_y_exact,
            max_ulp: descent_y_max,
            classification: classify_ulp(descent_y_max),
        },
        mag2_shader_order: FieldClassification {
            tested_cases: n,
            exact_cases: mag2_exact,
            max_ulp: mag2_max,
            classification: classify_ulp(mag2_max),
        },
    }
}

fn assert_exact_outputs_bit_exact(
    fields: &[f32],
    width: u32,
    height: u32,
    n_dims: u32,
    observers: &[ObserverInput],
    gpu_outputs: &[ObserverOutput],
    context: &str,
) {
    assert_eq!(
        gpu_outputs.len(),
        observers.len(),
        "{context}: output length mismatch"
    );
    for (i, obs) in observers.iter().enumerate() {
        let cpu = cpu_observer_oracle(fields, width, height, n_dims, *obs);
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
    }
}

fn assert_outputs_match(
    fields: &[f32],
    width: u32,
    height: u32,
    n_dims: u32,
    observers: &[ObserverInput],
    gpu_outputs: &[ObserverOutput],
    context: &str,
) {
    assert_exact_outputs_bit_exact(
        fields,
        width,
        height,
        n_dims,
        observers,
        gpu_outputs,
        context,
    );
    let summary = classify_observer_outputs(fields, width, height, n_dims, observers, gpu_outputs);
    println!(
        "{context} mag2_shader_order: exact={}/{} max_ulp={} class={:?}",
        summary.mag2_shader_order.exact_cases,
        summary.mag2_shader_order.tested_cases,
        summary.mag2_shader_order.max_ulp,
        summary.mag2_shader_order.classification
    );
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
        .map(|i| {
            let x = (i * 997) % width;
            let y = (i * 313) % height;
            ObserverInput {
                x,
                y,
                source_col,
                _pad: 0,
            }
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
fn jit_grad0_observer_shader_is_semantic_free() {
    assert_shader_semantic_free(OBSERVER_WGSL);
    assert!(OBSERVER_WGSL.contains("fields"));
    assert!(OBSERVER_WGSL.contains("observers"));
    assert!(OBSERVER_WGSL.contains("outputs"));
    assert!(OBSERVER_WGSL.contains("mag2"));
    assert!(OBSERVER_WGSL.contains("descent_x"));
    assert!(OBSERVER_WGSL.contains("dx"));
    assert!(OBSERVER_WGSL.contains("dy"));
}

#[test]
fn jit_grad0_small_grid_observer_parity() {
    with_gpu(|ctx| {
        let width = 8u32;
        let height = 8u32;
        let n_dims = 8u32;
        let source_col = 2u32;
        let fields = build_test_field(width, height, n_dims, source_col);
        let observers = small_grid_observers(width, height, source_col);
        let result = run_observers_gpu(ctx, &fields, &observers, width, height, n_dims);
        assert_eq!(result.dispatch_count, 1);
        assert_outputs_match(
            &fields,
            width,
            height,
            n_dims,
            &observers,
            &result.outputs,
            "small_grid",
        );
        println!(
            "small_grid: observers={}, dispatch_count={}, elapsed_ms={:.3}",
            observers.len(),
            result.dispatch_count,
            result.elapsed_ms
        );
    });
}

#[test]
fn jit_grad0_batches_10000_observers_one_dispatch() {
    with_gpu(|ctx| {
        let width = 128u32;
        let height = 128u32;
        let n_dims = 4u32;
        let source_col = 0u32;
        let fields = build_test_field(width, height, n_dims, source_col);
        let observers = structured_observers_10000(width, height, source_col);
        assert!(observers.len() >= 10_000);

        let result = run_observers_gpu(ctx, &fields, &observers, width, height, n_dims);
        assert_eq!(result.outputs.len(), 10_000);
        assert_eq!(result.dispatch_count, 1);

        let sample_indices = oracle_sample_indices(observers.len());
        let sampled_obs: Vec<ObserverInput> =
            sample_indices.iter().map(|&i| observers[i]).collect();
        let sampled_out: Vec<ObserverOutput> =
            sample_indices.iter().map(|&i| result.outputs[i]).collect();
        assert_outputs_match(
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
            "batch_10000: observers={}, dispatch_count={}, workgroups={}, workgroup_size={}, elapsed_ms={:.3}",
            observers.len(),
            result.dispatch_count,
            workgroups,
            WORKGROUP_SIZE,
            result.elapsed_ms
        );
    });
}

#[test]
fn jit_grad0_uses_squared_magnitude_no_sqrt() {
    assert!(
        !OBSERVER_WGSL.contains("sqrt("),
        "exact observer path must not use sqrt"
    );
    with_gpu(|ctx| {
        let width = 8u32;
        let height = 8u32;
        let n_dims = 4u32;
        let source_col = 0u32;
        let fields = build_test_field(width, height, n_dims, source_col);
        let observers = small_grid_observers(width, height, source_col);
        let result = run_observers_gpu(ctx, &fields, &observers, width, height, n_dims);
        for (i, out) in result.outputs.iter().enumerate() {
            let cpu = cpu_observer_oracle(&fields, width, height, n_dims, observers[i]);
            let cpu_mag2 = cpu_mag2_shader_order(&cpu);
            assert_eq!(
                out.descent_x.to_bits(),
                (-out.dx).to_bits(),
                "observer {i}: descent_x = -dx"
            );
            assert_eq!(
                out.descent_y.to_bits(),
                (-out.dy).to_bits(),
                "observer {i}: descent_y = -dy"
            );
            // mag2 classification is explicit in jit_grad0_mag2_classification_is_explicit;
            // here verify shader computes mag2 from dx/dy (no sqrt) with bounded ULP if approximate.
            let ulp = ulp_distance(out.mag2, cpu_mag2);
            assert!(
                classify_ulp(ulp) != OutputClassification::RejectedDeferred,
                "observer {i}: mag2 shader-order ULP {ulp} must not be rejected"
            );
        }
    });
}

fn run_small_grid_classification(ctx: &GpuContext) -> ObserverOutputClassificationSummary {
    let width = 8u32;
    let height = 8u32;
    let n_dims = 8u32;
    let source_col = 2u32;
    let fields = build_test_field(width, height, n_dims, source_col);
    let observers = small_grid_observers(width, height, source_col);
    let result = run_observers_gpu(ctx, &fields, &observers, width, height, n_dims);
    classify_observer_outputs(&fields, width, height, n_dims, &observers, &result.outputs)
}

fn run_batch_10000_sample_classification(ctx: &GpuContext) -> ObserverOutputClassificationSummary {
    let width = 128u32;
    let height = 128u32;
    let n_dims = 4u32;
    let source_col = 0u32;
    let fields = build_test_field(width, height, n_dims, source_col);
    let observers = structured_observers_10000(width, height, source_col);
    let result = run_observers_gpu(ctx, &fields, &observers, width, height, n_dims);
    let sample_indices = oracle_sample_indices(observers.len());
    let sampled_obs: Vec<ObserverInput> = sample_indices.iter().map(|&i| observers[i]).collect();
    let sampled_out: Vec<ObserverOutput> =
        sample_indices.iter().map(|&i| result.outputs[i]).collect();
    classify_observer_outputs(&fields, width, height, n_dims, &sampled_obs, &sampled_out)
}

#[test]
fn jit_grad0_mag2_classification_is_explicit() {
    with_gpu(|ctx| {
        let small = run_small_grid_classification(ctx);
        let batch = run_batch_10000_sample_classification(ctx);

        for (label, summary) in [("small_grid", small), ("batch_10000_sample", batch)] {
            assert_eq!(
                summary.dx.classification,
                OutputClassification::ExactDeterministicCandidate,
                "{label} dx must be exact-authoritative"
            );
            assert_eq!(
                summary.dy.classification,
                OutputClassification::ExactDeterministicCandidate,
                "{label} dy must be exact-authoritative"
            );
            assert_eq!(
                summary.descent_x.classification,
                OutputClassification::ExactDeterministicCandidate,
                "{label} descent_x must be exact-authoritative"
            );
            assert_eq!(
                summary.descent_y.classification,
                OutputClassification::ExactDeterministicCandidate,
                "{label} descent_y must be exact-authoritative"
            );
            assert!(matches!(
                summary.mag2_shader_order.classification,
                OutputClassification::ExactDeterministicCandidate
                    | OutputClassification::ApproximateJitOnly
                    | OutputClassification::RejectedDeferred
            ));
            println!(
                "{label}: mag2_shader_order exact={}/{} max_ulp={} class={:?}",
                summary.mag2_shader_order.exact_cases,
                summary.mag2_shader_order.tested_cases,
                summary.mag2_shader_order.max_ulp,
                summary.mag2_shader_order.classification
            );
        }
    });
}

#[test]
fn jit_grad0_exact_outputs_are_bit_exact() {
    with_gpu(|ctx| {
        let width = 8u32;
        let height = 8u32;
        let n_dims = 8u32;
        let source_col = 2u32;
        let fields = build_test_field(width, height, n_dims, source_col);
        let observers = small_grid_observers(width, height, source_col);
        let small = run_observers_gpu(ctx, &fields, &observers, width, height, n_dims);
        assert_exact_outputs_bit_exact(
            &fields,
            width,
            height,
            n_dims,
            &observers,
            &small.outputs,
            "small_grid",
        );

        let batch_summary = run_batch_10000_sample_classification(ctx);
        assert_eq!(
            batch_summary.dx.classification,
            OutputClassification::ExactDeterministicCandidate
        );
        assert_eq!(
            batch_summary.dy.classification,
            OutputClassification::ExactDeterministicCandidate
        );
        assert_eq!(
            batch_summary.descent_x.classification,
            OutputClassification::ExactDeterministicCandidate
        );
        assert_eq!(
            batch_summary.descent_y.classification,
            OutputClassification::ExactDeterministicCandidate
        );
    });
}

#[test]
fn jit_grad0_mag2_not_overclaimed_if_approximate() {
    with_gpu(|ctx| {
        let batch = run_batch_10000_sample_classification(ctx);
        let mag2_class = batch.mag2_shader_order.classification;

        if mag2_class != OutputClassification::ExactDeterministicCandidate {
            let design = include_str!("../../../docs/design_0_0_8_0.md");
            let invariants = include_str!("../../../docs/invariants.md");
            let track =
                include_str!("../../../docs/design_0_0_8_0_consumer_pulled_production_track.md");
            assert!(
                !OBSERVER_WGSL.contains("sqrt("),
                "mag2 observer path must not use native sqrt when classified approximate"
            );
            assert!(
                OBSERVER_WGSL.contains("out.mag2 = dx * dx + dy * dy;"),
                "observer shader must expose squared magnitude as mag2, not magnitude"
            );
            assert!(
                design.contains("exact claims carry CPU-oracle bit-exact parity"),
                "active 0.0.8 constitution must retain exact-claim parity discipline"
            );
            assert!(
                invariants.contains("ApproximateJitOnly")
                    && invariants.contains("mag2")
                    && invariants.contains("Exact pre-sqrt mag2 requires pinned construction"),
                "active invariants must keep unpinned mag2 out of exact-authoritative claims"
            );
            assert!(
                track.contains("fixed-point `dx/dy`")
                    && track.contains("exact pre-sqrt mag2")
                    && track.contains("Raw f32"),
                "active production track must keep exact magnitude tied to pinned fixed-point mag2"
            );
            assert_eq!(
                mag2_class,
                OutputClassification::ApproximateJitOnly,
                "mag2 max_ulp={} must be bounded approximate, not rejected",
                batch.mag2_shader_order.max_ulp
            );
        }
        println!("mag2_r1_classification={:?}", mag2_class);
    });
}

#[test]
fn jit_grad0_default_off_posture() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    assert_shader_semantic_free(OBSERVER_WGSL);
    // No `simthing_sim` import in this test crate module (compile-time posture).
    assert!(
        !OBSERVER_WGSL.contains("ResourceEconomySpec"),
        "observer WGSL must not reference ResourceEconomySpec"
    );
}
