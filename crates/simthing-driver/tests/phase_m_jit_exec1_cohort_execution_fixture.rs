//! Phase M-JIT-EXEC-1 — ProductionCandidatePreview cohort execution fixture (Tier-2, test-only).
//!
//! Groups identical exact GRAD-0→scorer graph requests into one REG-1-admitted cohort entry,
//! then executes a combined observer batch in one GPU dispatch. Spec-layer fixture only.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use bytemuck::{Pod, Zeroable};
use simthing_gpu::GpuContext;
use simthing_spec::{
    preview_kernel_registry_manifest, preview_production_candidate_registry_entry,
    preview_kernel_graph_identity, KernelDescriptorSpec, KernelGraphEdgeSpec,
    KernelGraphRequestSpec, KernelGraphSpec, KernelLane, KernelOutputSpec,
    KernelRegistryEntryPreview, KernelRegistryLane, KernelRegistryManifestPreview,
    MappingExecutionProfile, NativeMathClass, OutputAuthority, SpecError,
};
use wgpu::util::DeviceExt;

static GPU_MUTEX: Mutex<()> = Mutex::new(());
static EXECUTION_HELPER_INVOKED: AtomicBool = AtomicBool::new(false);

const WORKGROUP_SIZE: u32 = 64;
const BOUNDARY_CLAMP: u32 = 1;
const OBSERVERS_PER_REQUEST: usize = 10_000;

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

#[derive(Debug)]
struct FusionRunResult {
    outputs: Vec<ObserverScoreOutput>,
    dispatch_count: u32,
    elapsed_ms: f64,
}

struct RequestSegment {
    request_id: String,
    start: usize,
    len: usize,
}

struct CohortObserverBatch {
    combined: Vec<ObserverInput>,
    segments: Vec<RequestSegment>,
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
    }
}

fn grad1_style_scorer_with_bias_read() -> KernelDescriptorSpec {
    KernelDescriptorSpec {
        id: "m_jit_grad_1_scorer".into(),
        lane: KernelLane::TestOnly,
        reads: vec!["descent_x".into(), "descent_y".into(), "bias".into()],
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

fn reordered_exact_grad0_to_scorer_graph() -> KernelGraphSpec {
    KernelGraphSpec {
        nodes: vec![grad1_style_scorer(), grad0_exact_only()],
        edges: vec![
            exact_edge(
                "m_jit_grad_0_observer",
                "descent_y",
                "m_jit_grad_1_scorer",
                "descent_y",
            ),
            exact_edge(
                "m_jit_grad_0_observer",
                "descent_x",
                "m_jit_grad_1_scorer",
                "descent_x",
            ),
        ],
    }
}

fn distinct_exact_grad0_to_scorer_graph() -> KernelGraphSpec {
    KernelGraphSpec {
        nodes: vec![grad0_exact_only(), grad1_style_scorer_with_bias_read()],
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

fn identical_cohort_requests() -> Vec<KernelGraphRequestSpec> {
    vec![
        KernelGraphRequestSpec {
            request_id: "exec1_req_alpha".into(),
            graph: exact_grad0_to_scorer_graph(),
        },
        KernelGraphRequestSpec {
            request_id: "exec1_req_beta".into(),
            graph: reordered_exact_grad0_to_scorer_graph(),
        },
    ]
}

fn mixed_distinct_cohort_requests() -> Vec<KernelGraphRequestSpec> {
    vec![
        KernelGraphRequestSpec {
            request_id: "exec1_exact".into(),
            graph: exact_grad0_to_scorer_graph(),
        },
        KernelGraphRequestSpec {
            request_id: "exec1_distinct".into(),
            graph: distinct_exact_grad0_to_scorer_graph(),
        },
    ]
}

fn build_registry_manifest(
    requests: &[KernelGraphRequestSpec],
) -> Result<KernelRegistryManifestPreview, SpecError> {
    preview_kernel_registry_manifest(requests)
}

fn extract_single_cohort_entry(
    manifest: &KernelRegistryManifestPreview,
) -> Result<KernelRegistryEntryPreview, SpecError> {
    if manifest.entries.len() != 1 {
        return Err(SpecError::JitKernelDescriptorAdmission {
            kernel: "exec1_cohort".into(),
            reason: format!(
                "EXEC-1 single-cohort execution requires one manifest entry, got {}",
                manifest.entries.len()
            ),
        });
    }
    Ok(manifest.entries[0].clone())
}

fn admit_production_candidate(
    entry: &KernelRegistryEntryPreview,
) -> Result<KernelRegistryEntryPreview, SpecError> {
    preview_production_candidate_registry_entry(entry)
}

fn admit_single_cohort_candidate(
    manifest: &KernelRegistryManifestPreview,
) -> Result<KernelRegistryEntryPreview, SpecError> {
    let entry = extract_single_cohort_entry(manifest)?;
    admit_production_candidate(&entry)
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

fn structured_observers_for_request(
    width: u32,
    height: u32,
    source_col: u32,
    seed: u32,
) -> Vec<ObserverInput> {
    (0..OBSERVERS_PER_REQUEST as u32)
        .map(|i| ObserverInput {
            x: ((i * 997) + seed * 17) % width,
            y: ((i * 313) + seed * 41) % height,
            source_col,
            _pad: 0,
        })
        .collect()
}

fn build_combined_cohort_batch(
    request_ids: &[(&str, u32)],
    width: u32,
    height: u32,
    source_col: u32,
) -> CohortObserverBatch {
    let mut combined = Vec::new();
    let mut segments = Vec::new();
    for (request_id, seed) in request_ids {
        let start = combined.len();
        let obs = structured_observers_for_request(width, height, source_col, *seed);
        let len = obs.len();
        combined.extend(obs);
        segments.push(RequestSegment {
            request_id: (*request_id).into(),
            start,
            len,
        });
    }
    CohortObserverBatch {
        combined,
        segments,
    }
}

fn oracle_sample_indices(segment_len: usize) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..16.min(segment_len)).collect();
    if segment_len > 16 {
        indices.extend((segment_len - 16)..segment_len);
    }
    for k in 0..32 {
        indices.push((k * 997) % segment_len);
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
        label: Some("jit_exec1_fusion"),
        source: wgpu::ShaderSource::Wgsl(wgsl.into()),
    });

    let uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("jit_exec1_params"),
        contents: bytemuck::bytes_of(&params),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let fields_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("jit_exec1_fields"),
        contents: bytemuck::cast_slice(fields),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let observers_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("jit_exec1_observers"),
        contents: bytemuck::cast_slice(observers),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let outputs_len = (observers.len() * std::mem::size_of::<ObserverScoreOutput>()) as u64;
    let outputs_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("jit_exec1_outputs"),
        size: outputs_len,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("jit_exec1_bgl"),
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
        label: Some("jit_exec1_pipeline"),
        layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("jit_exec1_pl"),
            bind_group_layouts: &[&bgl],
            push_constant_ranges: &[],
        })),
        module: &module,
        entry_point: "main",
        compilation_options: Default::default(),
        cache: None,
    });

    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("jit_exec1_bg"),
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
        label: Some("jit_exec1_enc"),
    });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("jit_exec1_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bg, &[]);
        pass.dispatch_workgroups(workgroups, 1, 1);
    }
    queue.submit(Some(encoder.finish()));

    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("jit_exec1_readback"),
        size: outputs_len,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("jit_exec1_readback_enc"),
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
    assert_eq!(gpu_outputs.len(), observers.len(), "{context}: length mismatch");
    for (i, obs) in observers.iter().enumerate() {
        let cpu = cpu_fusion_oracle(fields, width, height, n_dims, *obs);
        let gpu = gpu_outputs[i];
        assert_eq!(gpu.dx.to_bits(), cpu.dx.to_bits(), "{context} observer {i} dx");
        assert_eq!(gpu.dy.to_bits(), cpu.dy.to_bits(), "{context} observer {i} dy");
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
        assert_eq!(gpu.score.to_bits(), cpu.score.to_bits(), "{context} observer {i} score");
    }
}

fn assert_segment_parity(
    fields: &[f32],
    width: u32,
    height: u32,
    n_dims: u32,
    batch: &CohortObserverBatch,
    gpu_outputs: &[ObserverScoreOutput],
) {
    for segment in &batch.segments {
        let local_indices = oracle_sample_indices(segment.len);
        let global_indices: Vec<usize> = local_indices.iter().map(|&i| segment.start + i).collect();
        let sampled_obs: Vec<ObserverInput> = global_indices
            .iter()
            .map(|&i| batch.combined[i])
            .collect();
        let sampled_out: Vec<ObserverScoreOutput> = global_indices
            .iter()
            .map(|&i| gpu_outputs[i])
            .collect();
        assert_fusion_parity(
            fields,
            width,
            height,
            n_dims,
            &sampled_obs,
            &sampled_out,
            &format!("exec1_segment_{}", segment.request_id),
        );
    }
}

/// EXEC-1 flow: cohort manifest → single entry → REG-1 admission → one combined GPU dispatch.
fn try_execute_admitted_cohort(
    ctx: &GpuContext,
    requests: &[KernelGraphRequestSpec],
    fields: &[f32],
    batch: &CohortObserverBatch,
    width: u32,
    height: u32,
    n_dims: u32,
) -> Result<(KernelRegistryEntryPreview, FusionRunResult), SpecError> {
    let manifest = build_registry_manifest(requests)?;
    let entry = extract_single_cohort_entry(&manifest)?;
    if entry.lane != KernelRegistryLane::TestOnlyPreview {
        return Err(SpecError::JitKernelDescriptorAdmission {
            kernel: entry.stable_key.clone(),
            reason: "expected TestOnlyPreview lane before candidate admission".into(),
        });
    }
    if entry.request_ids.len() < 2 {
        return Err(SpecError::JitKernelDescriptorAdmission {
            kernel: entry.stable_key.clone(),
            reason: "cohort entry must reference at least two request ids".into(),
        });
    }
    for expected in requests.iter().map(|r| r.request_id.as_str()) {
        if entry
            .request_ids
            .binary_search_by(|id| id.as_str().cmp(expected))
            .is_err()
        {
            return Err(SpecError::JitKernelDescriptorAdmission {
                kernel: entry.stable_key.clone(),
                reason: format!("missing request id `{expected}` in cohort entry"),
            });
        }
    }
    let mut sorted = entry.request_ids.clone();
    sorted.sort();
    if entry.request_ids != sorted {
        return Err(SpecError::JitKernelDescriptorAdmission {
            kernel: entry.stable_key.clone(),
            reason: "request_ids must be sorted".into(),
        });
    }

    let candidate = admit_production_candidate(&entry)?;
    if candidate.lane != KernelRegistryLane::ProductionCandidatePreview {
        return Err(SpecError::JitKernelDescriptorAdmission {
            kernel: candidate.stable_key.clone(),
            reason: "expected ProductionCandidatePreview lane".into(),
        });
    }
    if !candidate.default_off || candidate.production_wiring {
        return Err(SpecError::JitKernelDescriptorAdmission {
            kernel: candidate.stable_key.clone(),
            reason: "candidate must remain default_off with production_wiring false".into(),
        });
    }

    let wgsl = fused_wgsl();
    if wgsl.contains("sqrt(") || wgsl.contains("mag2") {
        return Err(SpecError::JitKernelDescriptorAdmission {
            kernel: candidate.stable_key.clone(),
            reason: "exact fused path must exclude sqrt and mag2".into(),
        });
    }

    let result = run_fusion_gpu(ctx, &wgsl, fields, &batch.combined, width, height, n_dims);
    Ok((candidate, result))
}

fn execute_admitted_cohort(
    ctx: &GpuContext,
    requests: &[KernelGraphRequestSpec],
    fields: &[f32],
    batch: &CohortObserverBatch,
    width: u32,
    height: u32,
    n_dims: u32,
) -> (KernelRegistryEntryPreview, FusionRunResult) {
    try_execute_admitted_cohort(ctx, requests, fields, batch, width, height, n_dims)
        .expect("admitted cohort execution")
}

fn try_execute_mixed_cohort(
    manifest: &KernelRegistryManifestPreview,
) -> SpecError {
    extract_single_cohort_entry(manifest).expect_err("mixed cohort must not execute as one batch")
}

#[test]
fn jit_exec1_cohort_admission_gates_execution() {
    EXECUTION_HELPER_INVOKED.store(false, Ordering::SeqCst);

    let requests = identical_cohort_requests();
    let manifest = build_registry_manifest(&requests).expect("cohort manifest");
    assert_eq!(manifest.entries.len(), 1);
    let entry = &manifest.entries[0];
    assert_eq!(entry.request_ids, vec!["exec1_req_alpha", "exec1_req_beta"]);

    let candidate = admit_production_candidate(entry).expect("admission");
    assert_eq!(candidate.lane, KernelRegistryLane::ProductionCandidatePreview);

    let mixed = build_registry_manifest(&mixed_distinct_cohort_requests()).expect("mixed manifest");
    assert_eq!(mixed.entries.len(), 2);
    let err = try_execute_mixed_cohort(&mixed);
    match err {
        SpecError::JitKernelDescriptorAdmission { reason, .. } => {
            assert!(reason.contains("one manifest entry"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
    assert!(
        !EXECUTION_HELPER_INVOKED.load(Ordering::SeqCst),
        "mixed cohort must not invoke GPU execution helper"
    );

    with_gpu(|ctx| {
        EXECUTION_HELPER_INVOKED.store(false, Ordering::SeqCst);
        let batch = build_combined_cohort_batch(
            &[("exec1_req_alpha", 0), ("exec1_req_beta", 1)],
            8,
            8,
            0,
        );
        let (_, result) = execute_admitted_cohort(ctx, &requests, &build_test_field(8, 8, 4, 0), &batch, 8, 8, 4);
        assert!(EXECUTION_HELPER_INVOKED.load(Ordering::SeqCst));
        assert_eq!(result.dispatch_count, 1);
    });
}

#[test]
fn jit_exec1_production_candidate_cohort_executes_with_oracle_parity() {
    with_gpu(|ctx| {
        let width = 128u32;
        let height = 128u32;
        let n_dims = 4u32;
        let source_col = 0u32;
        let fields = build_test_field(width, height, n_dims, source_col);
        let requests = identical_cohort_requests();
        let batch = build_combined_cohort_batch(
            &[("exec1_req_alpha", 0), ("exec1_req_beta", 1)],
            width,
            height,
            source_col,
        );

        assert_eq!(batch.segments.len(), 2);
        assert!(batch.combined.len() >= 20_000);

        let (candidate, result) =
            execute_admitted_cohort(ctx, &requests, &fields, &batch, width, height, n_dims);

        assert_eq!(candidate.lane, KernelRegistryLane::ProductionCandidatePreview);
        assert_eq!(candidate.request_ids.len(), 2);
        assert_eq!(result.outputs.len(), batch.combined.len());
        assert_eq!(result.dispatch_count, 1);

        assert_segment_parity(&fields, width, height, n_dims, &batch, &result.outputs);

        let workgroups = batch.combined.len() as u32 / WORKGROUP_SIZE + 1;
        println!(
            "exec1_cohort: requests={}, observers={}, dispatch_count={}, workgroups~={}, elapsed_ms={:.3}",
            candidate.request_ids.len(),
            batch.combined.len(),
            result.dispatch_count,
            workgroups,
            result.elapsed_ms
        );
    });
}

#[test]
fn jit_exec1_distinct_graphs_remain_separate_entries() {
    let requests = mixed_distinct_cohort_requests();
    let manifest = build_registry_manifest(&requests).expect("mixed manifest");
    assert_eq!(manifest.entries.len(), 2);

    let err = try_execute_mixed_cohort(&manifest);
    match err {
        SpecError::JitKernelDescriptorAdmission { reason, .. } => {
            assert!(reason.contains("one manifest entry"));
        }
        other => panic!("unexpected error: {other:?}"),
    }

    EXECUTION_HELPER_INVOKED.store(false, Ordering::SeqCst);
    with_gpu(|ctx| {
        let batch = build_combined_cohort_batch(
            &[("exec1_exact", 0), ("exec1_distinct", 1)],
            8,
            8,
            0,
        );
        let err = try_execute_admitted_cohort(
            ctx,
            &requests,
            &build_test_field(8, 8, 4, 0),
            &batch,
            8,
            8,
            4,
        )
        .expect_err("mixed cohort must not execute");
        match err {
            SpecError::JitKernelDescriptorAdmission { reason, .. } => {
                assert!(reason.contains("one manifest entry"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
        assert!(
            !EXECUTION_HELPER_INVOKED.load(Ordering::SeqCst),
            "mixed cohort must not reach GPU execution helper"
        );
    });
}

#[test]
fn jit_exec1_rejects_approximate_candidate_before_execution() {
    EXECUTION_HELPER_INVOKED.store(false, Ordering::SeqCst);

    let manifest = build_registry_manifest(&identical_cohort_requests()).expect("manifest");
    let mut entry = manifest.entries[0].clone();
    entry
        .canonical_text
        .push_str("\n  write=mag2 authority=ApproximateDiagnostic");
    let mag2_err = admit_production_candidate(&entry).expect_err("mag2 must reject");
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
        exact_sqrt_artifact: None,
        pre_sqrt_contract: None,
    };
    let sqrt_err = admit_production_candidate(&sqrt_entry).expect_err("sqrt must reject");
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

#[test]
fn jit_exec1_remains_default_off_no_production_wiring() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );

    let manifest = build_registry_manifest(&identical_cohort_requests()).expect("manifest");
    let candidate = admit_single_cohort_candidate(&manifest).expect("admission");
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
            "simthing-driver lib must not reference `{forbidden}` for EXEC-1 posture"
        );
    }
    assert!(
        !driver_lib.contains("jit_exec1"),
        "EXEC-1 fixture must not wire into production driver lib"
    );
}
