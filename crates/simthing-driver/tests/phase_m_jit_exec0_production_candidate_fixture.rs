//! Phase M-JIT-EXEC-0 — Default-off ProductionCandidatePreview execution fixture (Tier-2, test-only).
//!
//! Executes exactly one REG-1-admitted exact GRAD-0→scorer candidate via the GRAD-1-style fused
//! observer+score GPU path. Spec-layer REG-1 gate only; no runtime wiring.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use bytemuck::{Pod, Zeroable};
use simthing_gpu::GpuContext;
use simthing_spec::{
    preview_kernel_graph_identity, preview_kernel_registry_manifest,
    preview_production_candidate_registry_entry, KernelDescriptorSpec, KernelGraphEdgeSpec,
    KernelGraphRequestSpec, KernelGraphSpec, KernelLane, KernelOutputSpec,
    KernelRegistryEntryPreview, KernelRegistryLane, MappingExecutionProfile, NativeMathClass,
    OutputAuthority, SpecError,
};
use wgpu::util::DeviceExt;

static GPU_MUTEX: Mutex<()> = Mutex::new(());
static EXECUTION_HELPER_INVOKED: AtomicBool = AtomicBool::new(false);

const WORKGROUP_SIZE: u32 = 64;
const BOUNDARY_CLAMP: u32 = 1;

const W0: f32 = 0.65;
const W1: f32 = 0.35;
const BIAS: f32 = 0.125;

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

fn grad0_exact_only() -> KernelDescriptorSpec {
    KernelDescriptorSpec {
        id: "m_jit_grad_0_observer".into(),
        lane: KernelLane::TestOnly,
        reads: vec!["fields".into(), "observers".into()],
        writes: vec![
            KernelOutputSpec {
                name: "dx".into(),
                authority: OutputAuthority::ExactAuthoritative,
            },
            KernelOutputSpec {
                name: "dy".into(),
                authority: OutputAuthority::ExactAuthoritative,
            },
            KernelOutputSpec {
                name: "descent_x".into(),
                authority: OutputAuthority::ExactAuthoritative,
            },
            KernelOutputSpec {
                name: "descent_y".into(),
                authority: OutputAuthority::ExactAuthoritative,
            },
        ],
        native_math: NativeMathClass::None,
        semantic_free: true,
        default_off: true,
        production_wiring: false,
        exact_sqrt_artifact: None,
        pre_sqrt_contract: None,
        mag2_source_contract: None,
        score_authority_contract: None,
    }
}

fn grad1_style_scorer() -> KernelDescriptorSpec {
    KernelDescriptorSpec {
        id: "m_jit_grad_1_scorer".into(),
        lane: KernelLane::TestOnly,
        reads: vec!["descent_x".into(), "descent_y".into()],
        writes: vec![KernelOutputSpec {
            name: "score".into(),
            authority: OutputAuthority::ExactAuthoritative,
        }],
        native_math: NativeMathClass::None,
        semantic_free: true,
        default_off: true,
        production_wiring: false,
        exact_sqrt_artifact: None,
        pre_sqrt_contract: None,
        mag2_source_contract: None,
        score_authority_contract: None,
    }
}

fn exact_edge(from: &str, out: &str, to: &str, input: &str) -> KernelGraphEdgeSpec {
    KernelGraphEdgeSpec {
        from_kernel: from.into(),
        from_output: out.into(),
        to_kernel: to.into(),
        to_input: input.into(),
        required_authority: OutputAuthority::ExactAuthoritative,
    }
}

fn exact_grad0_to_scorer_graph() -> KernelGraphSpec {
    KernelGraphSpec {
        nodes: vec![grad0_exact_only(), grad1_style_scorer()],
        edges: vec![
            exact_edge(
                "m_jit_grad_0_observer",
                "descent_x",
                "m_jit_grad_1_scorer",
                "descent_x",
            ),
            exact_edge(
                "m_jit_grad_0_observer",
                "descent_y",
                "m_jit_grad_1_scorer",
                "descent_y",
            ),
        ],
    }
}

fn build_registry_entry(request_id: &str) -> Result<KernelRegistryEntryPreview, SpecError> {
    let requests = vec![KernelGraphRequestSpec {
        request_id: request_id.into(),
        graph: exact_grad0_to_scorer_graph(),
    }];
    let manifest = preview_kernel_registry_manifest(&requests)?;
    if manifest.entries.len() != 1 {
        return Err(SpecError::JitKernelDescriptorAdmission {
            kernel: "exec0_registry".into(),
            reason: format!(
                "expected one registry entry, got {}",
                manifest.entries.len()
            ),
        });
    }
    Ok(manifest.entries[0].clone())
}

/// REG-1 admission gate: manifest → production candidate preview.
fn admit_production_candidate(
    entry: &KernelRegistryEntryPreview,
) -> Result<KernelRegistryEntryPreview, SpecError> {
    preview_production_candidate_registry_entry(entry)
}

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

fn run_fusion_gpu(
    ctx: &GpuContext,
    wgsl: &str,
    fields: &[f32],
    observers: &[ObserverInput],
    width: u32,
    height: u32,
    n_dims: u32,
) -> FusionRunResult {
    EXECUTION_HELPER_INVOKED.store(true, Ordering::SeqCst);

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
        label: Some("jit_exec0_fusion"),
        source: wgpu::ShaderSource::Wgsl(wgsl.into()),
    });

    let uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("jit_exec0_params"),
        contents: bytemuck::bytes_of(&params),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let fields_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("jit_exec0_fields"),
        contents: bytemuck::cast_slice(fields),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let observers_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("jit_exec0_observers"),
        contents: bytemuck::cast_slice(observers),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let outputs_len = (observers.len() * std::mem::size_of::<ObserverScoreOutput>()) as u64;
    let outputs_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("jit_exec0_outputs"),
        size: outputs_len,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("jit_exec0_bgl"),
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
        label: Some("jit_exec0_pipeline"),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("jit_exec0_pl"),
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
        label: Some("jit_exec0_bg"),
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
        label: Some("jit_exec0_enc"),
    });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("jit_exec0_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bg, &[]);
        pass.dispatch_workgroups(workgroups, 1, 1);
    }
    queue.submit(Some(encoder.finish()));

    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("jit_exec0_readback"),
        size: outputs_len,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("jit_exec0_readback_enc"),
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

/// Full EXEC-0 flow: registry manifest → REG-1 admission → GPU execution.
fn execute_admitted_production_candidate(
    ctx: &GpuContext,
    request_id: &str,
    fields: &[f32],
    observers: &[ObserverInput],
    width: u32,
    height: u32,
    n_dims: u32,
) -> (KernelRegistryEntryPreview, FusionRunResult) {
    let entry = build_registry_entry(request_id).expect("registry manifest");
    assert_eq!(entry.lane, KernelRegistryLane::TestOnlyPreview);

    let candidate = admit_production_candidate(&entry).expect("REG-1 candidate admission");
    assert_eq!(
        candidate.lane,
        KernelRegistryLane::ProductionCandidatePreview
    );
    assert!(candidate.default_off);
    assert!(!candidate.production_wiring);
    assert!(!candidate.canonical_text.contains("mag2"));
    assert!(!candidate.canonical_text.contains("sqrt_out"));
    assert!(!candidate.canonical_text.contains("ApproximateJitOnly"));

    let wgsl = fused_wgsl();
    assert!(!wgsl.contains("sqrt("));
    assert!(!wgsl.contains("mag2"));

    let result = run_fusion_gpu(ctx, &wgsl, fields, observers, width, height, n_dims);
    (candidate, result)
}

fn try_execute_rejected_candidate(entry: &KernelRegistryEntryPreview) -> SpecError {
    admit_production_candidate(entry).expect_err("candidate must reject before execution")
}

#[test]
fn jit_exec0_candidate_admission_gates_execution() {
    EXECUTION_HELPER_INVOKED.store(false, Ordering::SeqCst);

    let entry = build_registry_entry("exec0_gate").expect("registry entry");
    let candidate = admit_production_candidate(&entry).expect("admission");
    assert_eq!(
        candidate.lane,
        KernelRegistryLane::ProductionCandidatePreview
    );

    let mut mag2_entry = entry.clone();
    mag2_entry
        .canonical_text
        .push_str("\n  write=mag2 authority=ApproximateDiagnostic");
    let err = try_execute_rejected_candidate(&mag2_entry);
    match err {
        SpecError::JitKernelDescriptorAdmission { reason, .. } => {
            assert!(reason.contains("mag2"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
    assert!(
        !EXECUTION_HELPER_INVOKED.load(Ordering::SeqCst),
        "rejected candidate must not invoke GPU execution helper"
    );

    with_gpu(|ctx| {
        EXECUTION_HELPER_INVOKED.store(false, Ordering::SeqCst);
        let (_, result) = execute_admitted_production_candidate(
            ctx,
            "exec0_gate_run",
            &build_test_field(8, 8, 4, 0),
            &[(ObserverInput {
                x: 2,
                y: 2,
                source_col: 0,
                _pad: 0,
            })],
            8,
            8,
            4,
        );
        assert!(EXECUTION_HELPER_INVOKED.load(Ordering::SeqCst));
        assert_eq!(result.dispatch_count, 1);
    });
}

#[test]
fn jit_exec0_production_candidate_grad1_executes_with_oracle_parity() {
    with_gpu(|ctx| {
        let width = 128u32;
        let height = 128u32;
        let n_dims = 4u32;
        let source_col = 0u32;
        let fields = build_test_field(width, height, n_dims, source_col);
        let observers = structured_observers_10000(width, height, source_col);
        assert!(observers.len() >= 10_000);

        let (candidate, result) = execute_admitted_production_candidate(
            ctx,
            "exec0_10000",
            &fields,
            &observers,
            width,
            height,
            n_dims,
        );

        assert_eq!(
            candidate.lane,
            KernelRegistryLane::ProductionCandidatePreview
        );
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
            "exec0_10000_sample",
        );

        let workgroups = 10_000u32.div_ceil(WORKGROUP_SIZE);
        println!(
            "exec0_10000: observers={}, dispatch_count={}, workgroups={}, elapsed_ms={:.3}",
            observers.len(),
            result.dispatch_count,
            workgroups,
            result.elapsed_ms
        );
    });
}

#[test]
fn jit_exec0_remains_default_off_no_production_wiring() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );

    let entry = build_registry_entry("exec0_posture").expect("registry entry");
    let candidate = admit_production_candidate(&entry).expect("admission");
    assert!(candidate.default_off);
    assert!(!candidate.production_wiring);

    let driver_lib = include_str!("../src/lib.rs");
    for forbidden in [
        "FirstSliceMappingSession::",
        "KernelCache::",
        "AccumulatorOpSession::",
        "tick_with_eml",
        "EmlGpuProgramTable",
    ] {
        assert!(
            !driver_lib.contains(forbidden),
            "simthing-driver lib must not reference `{forbidden}` for EXEC-0 posture"
        );
    }
    assert!(
        !driver_lib.contains("jit_exec0"),
        "EXEC-0 fixture must not wire into production driver lib"
    );
}

#[test]
fn jit_exec0_rejects_approximate_candidate_before_execution() {
    EXECUTION_HELPER_INVOKED.store(false, Ordering::SeqCst);

    let entry = build_registry_entry("exec0_reject").expect("registry entry");

    let mut mag2_entry = entry.clone();
    mag2_entry
        .canonical_text
        .push_str("\n  write=mag2 authority=ApproximateDiagnostic");
    let mag2_err = try_execute_rejected_candidate(&mag2_entry);
    match mag2_err {
        SpecError::JitKernelDescriptorAdmission { reason, .. } => {
            assert!(reason.contains("mag2"));
        }
        other => panic!("unexpected mag2 error: {other:?}"),
    }

    let sqrt = simthing_spec::landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == "m_jit_sqrt_0_candidate")
        .expect("sqrt0");
    let identity = preview_kernel_graph_identity(&KernelGraphSpec {
        nodes: vec![sqrt],
        edges: vec![],
    })
    .expect("sqrt identity");
    let sqrt_entry = KernelRegistryEntryPreview {
        stable_key: identity.stable_key,
        canonical_text: identity.canonical_text,
        request_ids: vec!["sqrt_req".into()],
        lane: KernelRegistryLane::TestOnlyPreview,
        default_off: true,
        production_wiring: false,
    };
    let sqrt_err = try_execute_rejected_candidate(&sqrt_entry);
    match sqrt_err {
        SpecError::JitKernelDescriptorAdmission { reason, .. } => {
            assert!(reason.contains("m_jit_sqrt_0_candidate"));
        }
        other => panic!("unexpected sqrt error: {other:?}"),
    }

    assert!(
        !EXECUTION_HELPER_INVOKED.load(Ordering::SeqCst),
        "approximate candidates must reject before GPU execution"
    );
}
