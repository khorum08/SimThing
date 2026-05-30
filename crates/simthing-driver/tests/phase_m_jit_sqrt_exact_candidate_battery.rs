//! Phase M-JIT-SQRT-EXACT-0/1D — Shader/software deterministic sqrt candidate battery (Tier-2, test-only).
//!
//! SQRT-EXACT-0: Candidates A (legacy/dead on DX12) and B (fallback).
//! SQRT-EXACT-1D: Candidate D (`CorrectlyRoundedHwBitmask`) — lead candidate.
//! Candidate C / f64 / `F64RoundDown` are explicitly out of scope.
//! No production sqrt admission, no exact-authority promotion in this slice.

use std::sync::Mutex;

use simthing_gpu::GpuContext;
use simthing_spec::{
    landed_jit_kernel_descriptors, validate_exact_kernel_inputs, NativeMathClass, OutputAuthority,
    SpecError,
};
use simthing_spec::MappingExecutionProfile;

static GPU_MUTEX: Mutex<()> = Mutex::new(());
const SQRT_CR_D_WGSL: &str = include_str!("wgsl/sqrt_cr_d_candidate.wgsl");
const SQRT_CR_E_WGSL: &str = include_str!("wgsl/sqrt_cr_e_candidate.wgsl");
const SQRT_CR_F_WGSL: &str = include_str!("wgsl/sqrt_cr_f_candidate.wgsl");

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
];

const FORBIDDEN_EXACT0_TERMS: &[&str] = &["f64", "F64RoundDown", "SHADER_F64", "sqrt_cr_c"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExactSqrtCandidate {
    CorrectlyRoundedHwFma,
    CorrectlyRoundedNewtonTwoProduct,
    CorrectlyRoundedHwBitmask,
    CorrectlyRoundedIntegerOnly,
    CorrectlyRoundedHwBitmaskNormalized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExactCandidateClassification {
    ExactCandidatePendingExhaustiveSweep,
    ApproximateJitOnly,
    RejectedDeferred,
}

#[derive(Debug, Clone, Copy)]
struct SweepSummary {
    tested: usize,
    exact_bits: usize,
    max_ulp: u32,
    classification: ExactCandidateClassification,
}

#[derive(Debug, Clone, Copy)]
struct FmaProbeSummary {
    tested_positive_finite: usize,
    correction_count: usize,
    max_ulp: u32,
}

#[derive(Debug, Clone, Copy)]
struct DProbeSummary {
    tested: usize,
    native_mismatch: usize,
    d_mismatch: usize,
    correction_count: usize,
    up_count: usize,
    down_count: usize,
    d_changes_vs_native: usize,
}

fn candidate_label(candidate: ExactSqrtCandidate) -> &'static str {
    match candidate {
        ExactSqrtCandidate::CorrectlyRoundedHwFma => "CorrectlyRoundedHwFma",
        ExactSqrtCandidate::CorrectlyRoundedNewtonTwoProduct => {
            "CorrectlyRoundedNewtonTwoProduct"
        }
        ExactSqrtCandidate::CorrectlyRoundedHwBitmask => "CorrectlyRoundedHwBitmask",
        ExactSqrtCandidate::CorrectlyRoundedIntegerOnly => "CorrectlyRoundedIntegerOnly",
        ExactSqrtCandidate::CorrectlyRoundedHwBitmaskNormalized => {
            "CorrectlyRoundedHwBitmaskNormalized"
        }
    }
}

fn classify(max_ulp: u32) -> ExactCandidateClassification {
    if max_ulp == 0 {
        ExactCandidateClassification::ExactCandidatePendingExhaustiveSweep
    } else if max_ulp <= 2 {
        ExactCandidateClassification::ApproximateJitOnly
    } else {
        ExactCandidateClassification::RejectedDeferred
    }
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

fn assert_semantic_free(wgsl: &str) {
    for term in FORBIDDEN_SEMANTIC_TERMS {
        assert!(
            !wgsl.contains(term),
            "generated WGSL must be semantic-free; found `{term}`"
        );
    }
}

fn assert_exact0_forbidden(wgsl: &str) {
    for term in FORBIDDEN_EXACT0_TERMS {
        assert!(
            !wgsl.contains(term),
            "SQRT-EXACT-0 WGSL must not contain `{term}`"
        );
    }
}

fn emit_sqrt_cr_a_fn() -> &'static str {
    r#"fn is_non_finite_positive_or_nonpositive(x: f32) -> bool {
    if (!(x > 0.0)) { return true; }
    return (bitcast<u32>(x) & 0x7f800000u) >= 0x7f800000u;
}

fn sqrt_cr_a(x: f32) -> f32 {
    if (is_non_finite_positive_or_nonpositive(x)) { return sqrt(x); }
    var s = x;
    var scale = 1.0;
    if (x < 1.1754944e-38) {
        s = x * 1.6777216e7 * 1.6777216e7;
        scale = 1.0 / 4096.0;
    }
    let y = sqrt(s);
    let r = fma(-y, y, s);
    let u = abs(bitcast<f32>(bitcast<u32>(y) + 1u) - y);
    let b = fma(y, u, 0.25 * u * u);
    var out = y;
    if (r >  b) { out = bitcast<f32>(bitcast<u32>(y) + 1u); }
    else if (r < -b) { out = bitcast<f32>(bitcast<u32>(y) - 1u); }
    return out * scale;
}
"#
}

fn emit_sqrt_cr_b_fn() -> &'static str {
    r#"fn is_non_finite_positive_or_nonpositive(x: f32) -> bool {
    if (!(x > 0.0)) { return true; }
    return (bitcast<u32>(x) & 0x7f800000u) >= 0x7f800000u;
}

fn two_prod_resid(y: f32, x: f32) -> f32 {
    let c = 4097.0 * y;
    let yh = c - (c - y);
    let yl = y - yh;
    let p = y * y;
    let e = ((yh * yh - p) + 2.0 * yh * yl) + yl * yl;
    return (x - p) - e;
}

fn sqrt_cr_b(x: f32) -> f32 {
    if (is_non_finite_positive_or_nonpositive(x)) { return sqrt(x); }
    var s = x;
    var scale = 1.0;
    if (x < 1.1754944e-38) {
        s = x * 1.6777216e7 * 1.6777216e7;
        scale = 1.0 / 4096.0;
    }
    var y = bitcast<f32>(0x1fbd1df5u + (bitcast<u32>(s) >> 1u));
    y = 0.5 * (y + s / y);
    y = 0.5 * (y + s / y);
    y = 0.5 * (y + s / y);
    let r = two_prod_resid(y, s);
    let u = abs(bitcast<f32>(bitcast<u32>(y) + 1u) - y);
    let b = (y * u) + (0.25 * u * u);
    if (r >  b) { y = bitcast<f32>(bitcast<u32>(y) + 1u); }
    else if (r < -b) { y = bitcast<f32>(bitcast<u32>(y) - 1u); }
    return y * scale;
}
"#
}

fn emit_batch_wgsl(candidate: ExactSqrtCandidate, batch_count: u32) -> String {
    let sqrt_fn = match candidate {
        ExactSqrtCandidate::CorrectlyRoundedHwFma => {
            format!("{}\n", emit_sqrt_cr_a_fn())
        }
        ExactSqrtCandidate::CorrectlyRoundedNewtonTwoProduct => {
            format!("{}\n", emit_sqrt_cr_b_fn())
        }
        ExactSqrtCandidate::CorrectlyRoundedHwBitmask => format!("{SQRT_CR_D_WGSL}\n"),
        ExactSqrtCandidate::CorrectlyRoundedIntegerOnly => {
            panic!("Candidate E uses dedicated u32 bit-IO wrapper")
        }
        ExactSqrtCandidate::CorrectlyRoundedHwBitmaskNormalized => {
            panic!("Candidate F uses dedicated u32 bit-IO wrapper")
        }
    };
    let call = match candidate {
        ExactSqrtCandidate::CorrectlyRoundedHwFma => "sqrt_cr_a",
        ExactSqrtCandidate::CorrectlyRoundedNewtonTwoProduct => "sqrt_cr_b",
        ExactSqrtCandidate::CorrectlyRoundedHwBitmask => "sqrt_cr_d",
        ExactSqrtCandidate::CorrectlyRoundedIntegerOnly => "sqrt_cr_e_bits",
        ExactSqrtCandidate::CorrectlyRoundedHwBitmaskNormalized => "sqrt_cr_f_bits",
    };

    format!(
        r#"{sqrt_fn}
@group(0) @binding(0) var<storage, read_write> data: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {{
    let i = gid.x;
    if (i >= {batch_count}u) {{ return; }}
    let base = i * 2u;
    let x = data[base];
    let y = {call}(x);
    data[base + 1u] = y;
}}
"#
    )
}

fn emit_fma_probe_wgsl(batch_count: u32) -> String {
    format!(
        r#"{a}
@group(0) @binding(0) var<storage, read_write> data: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {{
    let i = gid.x;
    if (i >= {batch_count}u) {{ return; }}
    let base = i * 3u;
    let x = data[base];
    if (is_non_finite_positive_or_nonpositive(x)) {{
        data[base + 1u] = sqrt_cr_a(x);
        data[base + 2u] = 0.0;
        return;
    }}
    var s = x;
    var scale = 1.0;
    if (x < 1.1754944e-38) {{
        s = x * 1.6777216e7 * 1.6777216e7;
        scale = 1.0 / 4096.0;
    }}
    let y = sqrt(s);
    let r = fma(-y, y, s);
    let u = abs(bitcast<f32>(bitcast<u32>(y) + 1u) - y);
    let b = fma(y, u, 0.25 * u * u);
    var out = y;
    var corrected = 0.0;
    if (r >  b) {{ out = bitcast<f32>(bitcast<u32>(y) + 1u); corrected = 1.0; }}
    else if (r < -b) {{ out = bitcast<f32>(bitcast<u32>(y) - 1u); corrected = 1.0; }}
    data[base + 1u] = out * scale;
    data[base + 2u] = corrected;
}}
"#,
        a = emit_sqrt_cr_a_fn(),
        batch_count = batch_count
    )
}

fn emit_d_probe_wgsl(batch_count: u32) -> String {
    format!(
        r#"{d}
@group(0) @binding(0) var<storage, read_write> data: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {{
    let i = gid.x;
    if (i >= {batch_count}u) {{ return; }}
    let base = i * 6u;
    let x = data[base];
    let native_y = sqrt(x);
    data[base + 1u] = native_y;
    let d_y = sqrt_cr_d(x);
    data[base + 2u] = d_y;
    var corrected = 0.0;
    var up_snap = 0.0;
    var down_snap = 0.0;
    if (!(is_non_finite_positive_or_nonpositive(x))) {{
        let x_bits = bitcast<u32>(x);
        let exp = x_bits >> 23u;
        let mant = x_bits & 0x007fffffu;
        if (exp != 0u || mant == 0u) {{
            let y = sqrt(x);
            let r = dekker_residual_hardened(y, x);
            let u_up = abs(bitcast<f32>(bitcast<u32>(y) + 1u) - y);
            let u_dn = abs(y - bitcast<f32>(bitcast<u32>(y) - 1u));
            if (r > (y * u_up + 0.25 * u_up * u_up)) {{
                corrected = 1.0;
                up_snap = 1.0;
            }} else if (r < -(y * u_dn - 0.25 * u_dn * u_dn)) {{
                corrected = 1.0;
                down_snap = 1.0;
            }}
        }}
    }}
    data[base + 3u] = corrected;
    data[base + 4u] = up_snap;
    data[base + 5u] = down_snap;
}}
"#,
        d = SQRT_CR_D_WGSL,
        batch_count = batch_count
    )
}

fn emit_e_batch_wgsl(batch_count: u32) -> String {
    format!(
        r#"{e}
@group(0) @binding(0) var<storage, read_write> data: array<u32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {{
    let i = gid.x;
    if (i >= {batch_count}u) {{ return; }}
    let base = i * 4u;
    let input_bits = data[base];
    let output_bits = sqrt_cr_e_bits(input_bits);
    data[base + 1u] = output_bits;
    data[base + 2u] = 0u;
    data[base + 3u] = 0u;
}}
"#,
        e = SQRT_CR_E_WGSL,
        batch_count = batch_count
    )
}

fn emit_f_batch_wgsl(batch_count: u32) -> String {
    format!(
        r#"{f}
@group(0) @binding(0) var<storage, read_write> data: array<u32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {{
    let i = gid.x;
    if (i >= {batch_count}u) {{ return; }}
    let base = i * 4u;
    let input_bits = data[base];
    let output_bits = sqrt_cr_f_bits(input_bits);
    data[base + 1u] = output_bits;
    data[base + 2u] = 0u;
    data[base + 3u] = 0u;
}}
"#,
        f = SQRT_CR_F_WGSL,
        batch_count = batch_count
    )
}

fn emit_f_probe_wgsl(batch_count: u32) -> String {
    format!(
        r#"{f}
@group(0) @binding(0) var<storage, read_write> data: array<u32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {{
    let i = gid.x;
    if (i >= {batch_count}u) {{ return; }}
    let base = i * 6u;
    let input_bits = data[base];
    let x = bitcast<f32>(input_bits);
    let native_bits = bitcast<u32>(sqrt(x));
    let output_bits = sqrt_cr_f_bits(input_bits);
    data[base + 1u] = output_bits;
    data[base + 2u] = native_bits;
    var corrected = 0u;
    var up = 0u;
    var down = 0u;
    let exp = (input_bits >> 23u) & 0xffu;
    let sign = input_bits >> 31u;
    let finite_positive = (exp != 0xffu) && (sign == 0u) && (input_bits != 0u);
    if (finite_positive && output_bits != native_bits) {{
        corrected = 1u;
        if (output_bits > native_bits) {{
            up = 1u;
        }} else {{
            down = 1u;
        }}
    }}
    data[base + 3u] = corrected;
    data[base + 4u] = up;
    data[base + 5u] = down;
}}
"#,
        f = SQRT_CR_F_WGSL,
        batch_count = batch_count
    )
}

fn fnv1a64_hex(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in input.as_bytes() {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let ctx = GpuContext::new_blocking().expect("GPU required");
    f(&ctx);
}

fn run_batch_gpu(ctx: &GpuContext, wgsl: &str, inputs: &[f32], stride: u32) -> Vec<f32> {
    use wgpu::util::DeviceExt;
    let device = &ctx.device;
    let queue = &ctx.queue;
    let n = inputs.len() as u32;
    assert!(stride >= 2);

    let mut data = vec![0.0f32; (n * stride) as usize];
    for (i, x) in inputs.iter().enumerate() {
        data[(i as u32 * stride) as usize] = *x;
    }

    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("jit_sqrt_exact_candidate"),
        source: wgpu::ShaderSource::Wgsl(wgsl.into()),
    });

    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("jit_sqrt_exact_bgl"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("jit_sqrt_exact_pipeline"),
        layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("jit_sqrt_exact_pl"),
            bind_group_layouts: &[&bgl],
            push_constant_ranges: &[],
        })),
        module: &module,
        entry_point: "main",
        compilation_options: Default::default(),
        cache: None,
    });

    let bytes = std::mem::size_of_val(data.as_slice()) as u64;
    let storage = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("jit_sqrt_exact_values"),
        contents: bytemuck::cast_slice(&data),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("jit_sqrt_exact_bg"),
        layout: &bgl,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage.as_entire_binding(),
        }],
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("jit_sqrt_exact_enc"),
    });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("jit_sqrt_exact_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bg, &[]);
        pass.dispatch_workgroups(n.div_ceil(64), 1, 1);
    }
    queue.submit(Some(encoder.finish()));

    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("jit_sqrt_exact_readback"),
        size: bytes,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("jit_sqrt_exact_readback_enc"),
    });
    enc2.copy_buffer_to_buffer(&storage, 0, &staging, 0, bytes);
    queue.submit(Some(enc2.finish()));

    let slice = staging.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);
    let mapped = slice.get_mapped_range();
    let out: Vec<f32> = bytemuck::cast_slice(&mapped).to_vec();
    drop(mapped);
    staging.unmap();
    out
}

fn run_batch_gpu_u32(ctx: &GpuContext, wgsl: &str, inputs: &[u32], stride: u32) -> Vec<u32> {
    use wgpu::util::DeviceExt;
    let device = &ctx.device;
    let queue = &ctx.queue;
    let n = inputs.len() as u32;
    assert!(stride >= 2);

    let mut data = vec![0u32; (n * stride) as usize];
    for (i, x) in inputs.iter().enumerate() {
        data[(i as u32 * stride) as usize] = *x;
    }

    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("jit_sqrt_exact_candidate_u32"),
        source: wgpu::ShaderSource::Wgsl(wgsl.into()),
    });

    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("jit_sqrt_exact_bgl_u32"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("jit_sqrt_exact_pipeline_u32"),
        layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("jit_sqrt_exact_pl_u32"),
            bind_group_layouts: &[&bgl],
            push_constant_ranges: &[],
        })),
        module: &module,
        entry_point: "main",
        compilation_options: Default::default(),
        cache: None,
    });

    let bytes = std::mem::size_of_val(data.as_slice()) as u64;
    let storage = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("jit_sqrt_exact_values_u32"),
        contents: bytemuck::cast_slice(&data),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("jit_sqrt_exact_bg_u32"),
        layout: &bgl,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage.as_entire_binding(),
        }],
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("jit_sqrt_exact_enc_u32"),
    });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("jit_sqrt_exact_pass_u32"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bg, &[]);
        pass.dispatch_workgroups(n.div_ceil(64), 1, 1);
    }
    queue.submit(Some(encoder.finish()));

    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("jit_sqrt_exact_readback_u32"),
        size: bytes,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("jit_sqrt_exact_readback_enc_u32"),
    });
    enc2.copy_buffer_to_buffer(&storage, 0, &staging, 0, bytes);
    queue.submit(Some(enc2.finish()));

    let slice = staging.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);
    let mapped = slice.get_mapped_range();
    let out: Vec<u32> = bytemuck::cast_slice(&mapped).to_vec();
    drop(mapped);
    staging.unmap();
    out
}

fn run_candidate_batch(
    ctx: &GpuContext,
    candidate: ExactSqrtCandidate,
    inputs: &[f32],
) -> Vec<f32> {
    let n = inputs.len() as u32;
    let wgsl = emit_batch_wgsl(candidate, n);
    let raw = run_batch_gpu(ctx, &wgsl, inputs, 2);
    (0..inputs.len())
        .map(|i| raw[i * 2 + 1])
        .collect()
}

fn run_candidate_e_bits(ctx: &GpuContext, input_bits: &[u32]) -> Vec<(u32, u32, u32, u32)> {
    let n = input_bits.len() as u32;
    let wgsl = emit_e_batch_wgsl(n);
    let raw = run_batch_gpu_u32(ctx, &wgsl, input_bits, 4);
    (0..input_bits.len())
        .map(|i| {
            let base = i * 4;
            (raw[base], raw[base + 1], raw[base + 2], raw[base + 3])
        })
        .collect()
}

fn run_candidate_f_bits(ctx: &GpuContext, input_bits: &[u32]) -> Vec<(u32, u32, u32, u32)> {
    let n = input_bits.len() as u32;
    let wgsl = emit_f_batch_wgsl(n);
    let raw = run_batch_gpu_u32(ctx, &wgsl, input_bits, 4);
    (0..input_bits.len())
        .map(|i| {
            let base = i * 4;
            (raw[base], raw[base + 1], raw[base + 2], raw[base + 3])
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct FProbeSummary {
    tested: usize,
    native_mismatch: usize,
    f_mismatch: usize,
    correction_count: usize,
    up_count: usize,
    down_count: usize,
    f_changes_vs_native: usize,
}

fn run_candidate_f_probe(
    ctx: &GpuContext,
    input_bits: &[u32],
) -> (FProbeSummary, Vec<(u32, u32, u32, u32, u32, u32)>) {
    let n = input_bits.len() as u32;
    let wgsl = emit_f_probe_wgsl(n);
    let raw = run_batch_gpu_u32(ctx, &wgsl, input_bits, 6);
    let mut native_mismatch = 0usize;
    let mut f_mismatch = 0usize;
    let mut correction_count = 0usize;
    let mut up_count = 0usize;
    let mut down_count = 0usize;
    let mut f_changes_vs_native = 0usize;
    let mut rows = Vec::with_capacity(input_bits.len());
    for i in 0..input_bits.len() {
        let base = i * 6;
        let x_bits = raw[base];
        let out_bits = raw[base + 1];
        let native_bits = raw[base + 2];
        let corrected = raw[base + 3];
        let up = raw[base + 4];
        let down = raw[base + 5];
        let x = f32::from_bits(x_bits);
        let cpu = x.sqrt();
        if !cpu.is_nan() && native_bits != cpu.to_bits() {
            native_mismatch += 1;
        }
        if !cpu.is_nan() && out_bits != cpu.to_bits() {
            f_mismatch += 1;
        }
        if corrected != 0 {
            correction_count += 1;
        }
        if up != 0 {
            up_count += 1;
        }
        if down != 0 {
            down_count += 1;
        }
        if out_bits != native_bits {
            f_changes_vs_native += 1;
        }
        rows.push((x_bits, out_bits, native_bits, corrected, up, down));
    }

    (
        FProbeSummary {
            tested: input_bits.len(),
            native_mismatch,
            f_mismatch,
            correction_count,
            up_count,
            down_count,
            f_changes_vs_native,
        },
        rows,
    )
}

fn sweep_candidate(ctx: &GpuContext, candidate: ExactSqrtCandidate, inputs: &[f32]) -> SweepSummary {
    let outputs = run_candidate_batch(ctx, candidate, inputs);
    let mut max_ulp = 0u32;
    let mut exact_bits = 0usize;
    for (x, gpu) in inputs.iter().zip(outputs.iter()) {
        let cpu = x.sqrt();
        let ulp = ulp_distance(*gpu, cpu);
        max_ulp = max_ulp.max(ulp);
        if gpu.to_bits() == cpu.to_bits() {
            exact_bits += 1;
        }
    }
    SweepSummary {
        tested: inputs.len(),
        exact_bits,
        max_ulp,
        classification: classify(max_ulp),
    }
}

fn sweep_candidate_normal_only(
    ctx: &GpuContext,
    candidate: ExactSqrtCandidate,
    inputs: &[f32],
) -> SweepSummary {
    let filtered: Vec<f32> = inputs
        .iter()
        .copied()
        .filter(|x| *x == 0.0 || x.is_normal() || x.is_infinite())
        .collect();
    sweep_candidate(ctx, candidate, &filtered)
}

fn edge_rows() -> Vec<(&'static str, f32)> {
    vec![
        ("pos_zero", 0.0),
        ("neg_zero", -0.0),
        ("smallest_subnormal", f32::from_bits(1)),
        ("largest_subnormal", f32::from_bits(0x007F_FFFF)),
        ("smallest_normal", f32::from_bits(0x0080_0000)),
        ("one", 1.0),
        ("perfect_square_4", 4.0),
        ("perfect_square_9", 9.0),
        ("rounding_boundary_a", f32::from_bits(0x3f80_0001)),
        ("rounding_boundary_b", f32::from_bits(0x3f7f_ffff)),
        ("f32_max", f32::MAX),
        ("pos_inf", f32::INFINITY),
        ("quiet_nan", f32::NAN),
        ("neg_finite", -1.0),
        ("neg_inf", f32::NEG_INFINITY),
    ]
}

fn dense_stratified_corpus() -> Vec<f32> {
    let mut out = Vec::new();
    // Subnormal-heavy
    for bits in (1u32..=0x100).step_by(7) {
        out.push(f32::from_bits(bits));
    }
    // Exponent sweep with sparse mantissa bits
    for exp in 0u32..=254 {
        for mantissa in [0u32, 1, 0x400000, 0x7fffff] {
            let bits = (exp << 23) | mantissa;
            if bits <= 0x7F7F_FFFF {
                out.push(f32::from_bits(bits));
            }
        }
    }
    // Known SQRT-0 scalar corpus / off-by-one neighborhood
    for x in [
        0.2f32,
        0.3,
        3.1415927,
        10.75,
        12345.678,
        1.0e-20,
        1.0e-10,
        1.0e8,
        1.0e20,
        f32::MIN_POSITIVE,
    ] {
        out.push(x);
    }
    out.sort_by(|a, b| a.to_bits().cmp(&b.to_bits()));
    out.dedup_by(|a, b| a.to_bits() == b.to_bits());
    out
}

fn positive_finite_inputs(inputs: &[f32]) -> Vec<f32> {
    inputs
        .iter()
        .copied()
        .filter(|x| x.is_finite() && *x > 0.0)
        .collect()
}

fn sqrt0_descriptor() -> simthing_spec::KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == "m_jit_sqrt_0_candidate")
        .expect("sqrt0 descriptor")
}

fn grad0_descriptor() -> simthing_spec::KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == "m_jit_grad_0_observer")
        .expect("grad0 descriptor")
}

#[test]
fn sqrt_exact0_candidates_compile_semantic_free_wgsl() {
    for candidate in [
        ExactSqrtCandidate::CorrectlyRoundedHwFma,
        ExactSqrtCandidate::CorrectlyRoundedNewtonTwoProduct,
    ] {
        let wgsl = emit_batch_wgsl(candidate, 1);
        assert_semantic_free(&wgsl);
        assert_exact0_forbidden(&wgsl);
        assert!(wgsl.contains(match candidate {
            ExactSqrtCandidate::CorrectlyRoundedHwFma => "sqrt_cr_a",
            ExactSqrtCandidate::CorrectlyRoundedNewtonTwoProduct => "sqrt_cr_b",
            ExactSqrtCandidate::CorrectlyRoundedHwBitmask => unreachable!("SQRT-EXACT-0 loop excludes D"),
            ExactSqrtCandidate::CorrectlyRoundedIntegerOnly => {
                unreachable!("SQRT-EXACT-0 loop excludes E")
            }
            ExactSqrtCandidate::CorrectlyRoundedHwBitmaskNormalized => {
                unreachable!("SQRT-EXACT-0 loop excludes F")
            }
        }));
        with_gpu(|ctx| {
            let out = run_candidate_batch(ctx, candidate, &[4.0]);
            assert_eq!(out[0].to_bits(), 2.0f32.to_bits());
        });
    }

    let baseline = include_str!("../../simthing-gpu/src/shaders/accumulator_op.wgsl");
    assert!(!baseline.contains("sqrt("));
}

fn is_subnormal(x: f32) -> bool {
    x != 0.0 && x.is_finite() && !x.is_normal()
}

#[derive(Debug, Clone)]
struct EdgeRowResult {
    candidate: ExactSqrtCandidate,
    name: &'static str,
    x: f32,
    gpu: f32,
    cpu: f32,
    ulp: u32,
    exact_bits: bool,
}

#[test]
fn sqrt_exact0_edge_rows_match_cpu_oracle() {
    with_gpu(|ctx| {
        let mut results = Vec::new();
        for (name, x) in edge_rows() {
            for candidate in [
                ExactSqrtCandidate::CorrectlyRoundedHwFma,
                ExactSqrtCandidate::CorrectlyRoundedNewtonTwoProduct,
            ] {
                let gpu = run_candidate_batch(ctx, candidate, &[x])[0];
                let cpu = x.sqrt();
                let exact_bits = if cpu.is_nan() && gpu.is_nan() {
                    println!(
                        "{} {}: both NaN (gpu_bits={:#x} cpu_bits={:#x})",
                        candidate_label(candidate),
                        name,
                        gpu.to_bits(),
                        cpu.to_bits()
                    );
                    false
                } else {
                    gpu.to_bits() == cpu.to_bits()
                };
                let ulp = if cpu.is_nan() && gpu.is_nan() {
                    0
                } else {
                    ulp_distance(gpu, cpu)
                };
                if !exact_bits {
                    println!(
                        "{} edge `{}` x={:?} gpu={:?} cpu={:?} ulp={}",
                        candidate_label(candidate),
                        name,
                        x,
                        gpu,
                        cpu,
                        ulp
                    );
                }
                results.push(EdgeRowResult {
                    candidate,
                    name,
                    x,
                    gpu,
                    cpu,
                    ulp,
                    exact_bits,
                });
            }
        }

        for candidate in [
            ExactSqrtCandidate::CorrectlyRoundedHwFma,
            ExactSqrtCandidate::CorrectlyRoundedNewtonTwoProduct,
        ] {
            let rows: Vec<_> = results
                .iter()
                .filter(|r| r.candidate == candidate)
                .collect();
            let normal: Vec<_> = rows.iter().filter(|r| !is_subnormal(r.x)).collect();
            let subnormal: Vec<_> = rows.iter().filter(|r| is_subnormal(r.x)).collect();
            let normal_exact = normal.iter().filter(|r| r.exact_bits).count();
            let normal_max_ulp = normal.iter().map(|r| r.ulp).max().unwrap_or(0);
            let subnormal_exact = subnormal.iter().filter(|r| r.exact_bits).count();
            println!(
                "{} edge_rows: all={} exact={} normal={} normal_exact={} normal_max_ulp={} subnormal={} subnormal_exact={}",
                candidate_label(candidate),
                rows.len(),
                rows.iter().filter(|r| r.exact_bits).count(),
                normal.len(),
                normal_exact,
                normal_max_ulp,
                subnormal.len(),
                subnormal_exact
            );
        }

        assert_eq!(results.len(), edge_rows().len() * 2);
    });
}

#[test]
fn sqrt_exact0_dense_stratified_sweep() {
    with_gpu(|ctx| {
        let corpus = dense_stratified_corpus();
        for candidate in [
            ExactSqrtCandidate::CorrectlyRoundedHwFma,
            ExactSqrtCandidate::CorrectlyRoundedNewtonTwoProduct,
        ] {
            let summary = sweep_candidate(ctx, candidate, &corpus);
            let normal_summary = sweep_candidate_normal_only(ctx, candidate, &corpus);
            println!(
                "{} dense_stratified(all): tested={} exact_bits={} max_ulp={} class={:?}",
                candidate_label(candidate),
                summary.tested,
                summary.exact_bits,
                summary.max_ulp,
                summary.classification
            );
            println!(
                "{} dense_stratified(normal+zero+inf): tested={} exact_bits={} max_ulp={} class={:?}",
                candidate_label(candidate),
                normal_summary.tested,
                normal_summary.exact_bits,
                normal_summary.max_ulp,
                normal_summary.classification
            );
            assert!(summary.tested > 100);
        }
    });
}

#[test]
fn sqrt_exact0_fma_fusion_probe_for_candidate_a() {
    with_gpu(|ctx| {
        let corpus = positive_finite_inputs(&dense_stratified_corpus());
        let n = corpus.len() as u32;
        let wgsl = emit_fma_probe_wgsl(n);
        let raw = run_batch_gpu(ctx, &wgsl, &corpus, 3);
        let mut correction_count = 0usize;
        let mut max_ulp = 0u32;
        for (i, x) in corpus.iter().enumerate() {
            let base = i * 3;
            let gpu = raw[base + 1];
            let corrected = raw[base + 2];
            if corrected > 0.0 {
                correction_count += 1;
            }
            max_ulp = max_ulp.max(ulp_distance(gpu, x.sqrt()));
        }
        let summary = FmaProbeSummary {
            tested_positive_finite: corpus.len(),
            correction_count,
            max_ulp,
        };
        println!(
            "candidate_a_fma_probe: tested={} corrections={} max_ulp={}",
            summary.tested_positive_finite, summary.correction_count, summary.max_ulp
        );
        if summary.max_ulp > 0 {
            println!(
                "candidate_a_fma_probe_note: nonzero ULP likely indicates non-fused fma or residual miscorrection on this backend"
            );
        }
    });
}

#[test]
fn sqrt_exact0_b_candidate_no_fma_dependency() {
    let wgsl = emit_batch_wgsl(ExactSqrtCandidate::CorrectlyRoundedNewtonTwoProduct, 1);
    assert!(
        !wgsl.contains("fma("),
        "Candidate B must not depend on fma fusion"
    );
    let body = emit_sqrt_cr_b_fn();
    assert!(
        !body.contains("fma("),
        "Candidate B core must not use fma"
    );
    // Native sqrt appears only in special-value passthrough guard.
    assert!(body.contains("is_non_finite_positive_or_nonpositive"));
    assert!(
        body.matches("sqrt(").count() <= 1,
        "Candidate B should call native sqrt only for special-value passthrough"
    );
    assert!(body.contains("if (is_non_finite_positive_or_nonpositive(x))"));
    assert!(body.contains("two_prod_resid"));
}

#[test]
#[ignore = "full 2^31 finite non-negative f32 sweep; run with --ignored explicitly"]
fn sqrt_exact0_full_exhaustive_sweep_is_ignored_by_default() {
    with_gpu(|ctx| {
        const BATCH: u32 = 65536;
        let mut max_ulp_a = 0u32;
        let mut max_ulp_b = 0u32;
        let mut bits = 0u32;
        while bits <= 0x7F7F_FFFF {
            let end = bits.saturating_add(BATCH - 1).min(0x7F7F_FFFF);
            let batch: Vec<f32> = (bits..=end).map(f32::from_bits).collect();
            let out_a = run_candidate_batch(ctx, ExactSqrtCandidate::CorrectlyRoundedHwFma, &batch);
            let out_b = run_candidate_batch(
                ctx,
                ExactSqrtCandidate::CorrectlyRoundedNewtonTwoProduct,
                &batch,
            );
            for ((x, ga), gb) in batch.iter().zip(out_a.iter()).zip(out_b.iter()) {
                let cpu = x.sqrt();
                max_ulp_a = max_ulp_a.max(ulp_distance(*ga, cpu));
                max_ulp_b = max_ulp_b.max(ulp_distance(*gb, cpu));
            }
            bits = end.saturating_add(1);
            if bits == 0 {
                break;
            }
        }
        println!(
            "exhaustive_sweep: max_ulp_a={} max_ulp_b={}",
            max_ulp_a, max_ulp_b
        );
        assert_eq!(max_ulp_a, 0, "Candidate A requires max_ulp == 0 for promotion");
        assert_eq!(max_ulp_b, 0, "Candidate B requires max_ulp == 0 for promotion");
    });
}

#[test]
fn sqrt_exact0_no_promotion_yet() {
    let sqrt0 = sqrt0_descriptor();
    assert_eq!(sqrt0.native_math, NativeMathClass::ApproximateJitOnly);
    assert!(
        sqrt0
            .writes
            .iter()
            .all(|out| out.authority == OutputAuthority::ApproximateDiagnostic)
    );

    let grad0 = grad0_descriptor();
    let mag2 = grad0
        .writes
        .iter()
        .find(|out| out.name == "mag2")
        .expect("mag2 output");
    assert_eq!(mag2.authority, OutputAuthority::ApproximateDiagnostic);

    assert!(matches!(
        validate_exact_kernel_inputs(&sqrt0, &["sqrt_out"]),
        Err(SpecError::JitKernelDescriptorAdmission { .. })
    ));
    assert!(matches!(
        validate_exact_kernel_inputs(&grad0, &["mag2"]),
        Err(SpecError::JitKernelDescriptorAdmission { .. })
    ));

    assert!(
        !landed_jit_kernel_descriptors()
            .iter()
            .any(|desc| desc.id.contains("sqrt_exact")),
        "no exact sqrt kernel descriptor admitted yet"
    );
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );

    let baseline = include_str!("../../simthing-gpu/src/shaders/accumulator_op.wgsl");
    assert!(!baseline.contains("sqrt("));
}

// --- SQRT-EXACT-1D: Candidate D (`CorrectlyRoundedHwBitmask`) ---

fn edge_rows_1d() -> Vec<(&'static str, f32)> {
    let mut rows = edge_rows();
    rows.extend([
        ("pow2_half", 0.5f32),
        ("pow2_two", 2.0),
        ("pow2_256", 256.0),
        ("pow2_largest_normal_pow2", f32::from_bits(0x7f00_0000)),
        ("ab_fail_neighbor_lo", f32::from_bits(0x3f7f_fffe)),
        ("ab_fail_neighbor_hi", f32::from_bits(0x3f80_0002)),
    ]);
    rows.sort_by(|a, b| a.1.to_bits().cmp(&b.1.to_bits()));
    rows.dedup_by(|a, b| a.1.to_bits() == b.1.to_bits());
    rows
}

fn subnormal_corpus() -> Vec<f32> {
    let mut out = Vec::new();
    for bits in 1u32..=1024 {
        out.push(f32::from_bits(bits));
    }
    for i in 0..1024 {
        out.push(f32::from_bits(0x007F_FFFF - i));
    }
    for shift in 0..23 {
        out.push(f32::from_bits(1u32 << shift));
    }
    let mut state = 0x1234_5678u32;
    for _ in 0..512 {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let mant = (state & 0x007F_FFFF) | 1;
        out.push(f32::from_bits(mant));
    }
    out.sort_by(|a, b| a.to_bits().cmp(&b.to_bits()));
    out.dedup_by(|a, b| a.to_bits() == b.to_bits());
    out
}

fn ab_failure_neighborhood() -> Vec<f32> {
    let seeds = [
        f32::from_bits(0x3f80_0001),
        f32::from_bits(0x3f7f_ffff),
        f32::from_bits(0x3f7f_fffe),
        f32::from_bits(0x3f80_0002),
    ];
    let mut out = Vec::new();
    for s in seeds {
        let b = s.to_bits();
        for delta in -8i32..=8 {
            let nb = (b as i64 + i64::from(delta)).clamp(0, 0x7F7F_FFFF as i64) as u32;
            out.push(f32::from_bits(nb));
        }
    }
    out.sort_by(|a, b| a.to_bits().cmp(&b.to_bits()));
    out.dedup_by(|a, b| a.to_bits() == b.to_bits());
    out
}

fn dense_normal_corpus_1d() -> Vec<f32> {
    let mut out = dense_stratified_corpus();
    out.extend(ab_failure_neighborhood());
    out.retain(|x| *x == 0.0 || x.is_normal() || x.is_infinite());
    out.sort_by(|a, b| a.to_bits().cmp(&b.to_bits()));
    out.dedup_by(|a, b| a.to_bits() == b.to_bits());
    out
}

fn positive_finite_normal_inputs(inputs: &[f32]) -> Vec<f32> {
    inputs
        .iter()
        .copied()
        .filter(|x| x.is_finite() && *x > 0.0 && x.is_normal())
        .collect()
}

#[derive(Debug, Clone)]
struct DenseSweepDetail {
    summary: SweepSummary,
    correction_count: usize,
    up_count: usize,
    down_count: usize,
    worst: Vec<(f32, f32, f32, u32)>,
}

fn run_d_probe(ctx: &GpuContext, inputs: &[f32]) -> DProbeSummary {
    let n = inputs.len() as u32;
    let wgsl = emit_d_probe_wgsl(n);
    let raw = run_batch_gpu(ctx, &wgsl, inputs, 6);
    let mut native_mismatch = 0usize;
    let mut d_mismatch = 0usize;
    let mut correction_count = 0usize;
    let mut up_count = 0usize;
    let mut down_count = 0usize;
    let mut d_changes_vs_native = 0usize;
    for (i, x) in inputs.iter().enumerate() {
        let base = i * 6;
        let native_y = raw[base + 1];
        let d_y = raw[base + 2];
        let corrected = raw[base + 3];
        let up = raw[base + 4];
        let down = raw[base + 5];
        let cpu = x.sqrt();
        if !cpu.is_nan() && native_y.to_bits() != cpu.to_bits() {
            native_mismatch += 1;
        }
        if !cpu.is_nan() && d_y.to_bits() != cpu.to_bits() {
            d_mismatch += 1;
        }
        if corrected > 0.0 {
            correction_count += 1;
        }
        if up > 0.0 {
            up_count += 1;
        }
        if down > 0.0 {
            down_count += 1;
        }
        if d_y.to_bits() != native_y.to_bits() {
            d_changes_vs_native += 1;
        }
    }
    DProbeSummary {
        tested: inputs.len(),
        native_mismatch,
        d_mismatch,
        correction_count,
        up_count,
        down_count,
        d_changes_vs_native,
    }
}

fn sweep_d_dense_with_probe(ctx: &GpuContext, inputs: &[f32]) -> DenseSweepDetail {
    let summary = sweep_candidate(ctx, ExactSqrtCandidate::CorrectlyRoundedHwBitmask, inputs);
    let probe = run_d_probe(ctx, &positive_finite_normal_inputs(inputs));
    let outputs = run_candidate_batch(ctx, ExactSqrtCandidate::CorrectlyRoundedHwBitmask, inputs);
    let mut worst = Vec::new();
    for (x, gpu) in inputs.iter().zip(outputs.iter()) {
        let cpu = x.sqrt();
        let ulp = if cpu.is_nan() && gpu.is_nan() {
            0
        } else {
            ulp_distance(*gpu, cpu)
        };
        if ulp > 0 {
            worst.push((*x, *gpu, cpu, ulp));
        }
    }
    worst.sort_by(|a, b| b.3.cmp(&a.3));
    worst.truncate(8);
    DenseSweepDetail {
        summary,
        correction_count: probe.correction_count,
        up_count: probe.up_count,
        down_count: probe.down_count,
        worst,
    }
}

#[derive(Debug, Clone)]
struct SubnormalSweepDetail {
    tested: usize,
    exact_bits: usize,
    max_ulp: u32,
    integer_path_used: usize,
    flush_count: usize,
    worst: Vec<(f32, f32, f32, u32)>,
}

fn sweep_d_subnormal(ctx: &GpuContext, inputs: &[f32]) -> SubnormalSweepDetail {
    let outputs = run_candidate_batch(ctx, ExactSqrtCandidate::CorrectlyRoundedHwBitmask, inputs);
    let mut max_ulp = 0u32;
    let mut exact_bits = 0usize;
    let mut integer_path_used = 0usize;
    let mut flush_count = 0usize;
    let mut worst = Vec::new();
    for (x, gpu) in inputs.iter().zip(outputs.iter()) {
        if is_subnormal(*x) && *x > 0.0 {
            integer_path_used += 1;
        }
        let cpu = x.sqrt();
        let ulp = ulp_distance(*gpu, cpu);
        max_ulp = max_ulp.max(ulp);
        if gpu.to_bits() == cpu.to_bits() {
            exact_bits += 1;
        }
        if gpu.to_bits() == 0 && cpu.to_bits() != 0 {
            flush_count += 1;
        }
        if ulp > 0 {
            worst.push((*x, *gpu, cpu, ulp));
        }
    }
    worst.sort_by(|a, b| b.3.cmp(&a.3));
    worst.truncate(8);
    SubnormalSweepDetail {
        tested: inputs.len(),
        exact_bits,
        max_ulp,
        integer_path_used,
        flush_count,
        worst,
    }
}

#[test]
fn sqrt_exact1d_r1_candidate_d_uses_verbatim_wgsl_artifact() {
    assert!(!SQRT_CR_D_WGSL.is_empty(), "verbatim D WGSL artifact must be non-empty");
    assert!(SQRT_CR_D_WGSL.contains("fn sqrt_cr_d("));
    assert!(SQRT_CR_D_WGSL.contains("fn snap_directional("));
    assert!(SQRT_CR_D_WGSL.contains("fn dekker_residual_hardened("));
    assert!(SQRT_CR_D_WGSL.contains("fn sqrt_cr_d_subnormal_integer("));
    let batch = emit_batch_wgsl(ExactSqrtCandidate::CorrectlyRoundedHwBitmask, 1);
    let probe = emit_d_probe_wgsl(1);
    assert!(
        batch.contains(SQRT_CR_D_WGSL),
        "D batch wrapper must include verbatim artifact as contiguous substring"
    );
    assert!(
        probe.contains(SQRT_CR_D_WGSL),
        "D probe wrapper must include verbatim artifact as contiguous substring"
    );
    assert_eq!(batch.matches(SQRT_CR_D_WGSL).count(), 1);
    assert_eq!(probe.matches(SQRT_CR_D_WGSL).count(), 1);
}

#[test]
fn sqrt_exact1d_r1_verbatim_d_wgsl_compiles_semantic_free() {
    assert_semantic_free(SQRT_CR_D_WGSL);
    assert_exact0_forbidden(SQRT_CR_D_WGSL);
    let wgsl = emit_batch_wgsl(ExactSqrtCandidate::CorrectlyRoundedHwBitmask, 1);
    assert_semantic_free(&wgsl);
    assert_exact0_forbidden(&wgsl);
    assert!(wgsl.contains("sqrt_cr_d"));
    assert!(wgsl.contains("dekker_residual_hardened"));
    assert!(wgsl.contains("sqrt_cr_d_subnormal_integer"));
    assert!(!wgsl.contains("sqrt_cr_c"));
    assert!(!wgsl.contains("fma("));
    with_gpu(|ctx| {
        let out = run_candidate_batch(ctx, ExactSqrtCandidate::CorrectlyRoundedHwBitmask, &[4.0]);
        assert_eq!(out[0].to_bits(), 2.0f32.to_bits());
    });
}

#[test]
fn sqrt_exact1d_r1_verbatim_d_artifact_hash_recorded() {
    let hash = fnv1a64_hex(SQRT_CR_D_WGSL);
    println!(
        "sqrt_exact1d_r1_verbatim_d_artifact_hash_fnv1a64={hash} path=crates/simthing-driver/tests/wgsl/sqrt_cr_d_candidate.wgsl bytes={}",
        SQRT_CR_D_WGSL.len()
    );
    assert_eq!(hash.len(), 16);
}

#[test]
fn sqrt_exact1d_candidate_d_edge_rows() {
    with_gpu(|ctx| {
        let mut exact = 0usize;
        let mut normal_exact = 0usize;
        let mut normal_max_ulp = 0u32;
        let mut subnormal_exact = 0usize;
        let mut nan_ok = 0usize;
        for (name, x) in edge_rows_1d() {
            let gpu = run_candidate_batch(ctx, ExactSqrtCandidate::CorrectlyRoundedHwBitmask, &[x])[0];
            let cpu = x.sqrt();
            if cpu.is_nan() && gpu.is_nan() {
                nan_ok += 1;
                println!(
                    "D edge `{name}`: both NaN (gpu={:#x} cpu={:#x})",
                    gpu.to_bits(),
                    cpu.to_bits()
                );
                continue;
            }
            let ulp = ulp_distance(gpu, cpu);
            let bits_match = gpu.to_bits() == cpu.to_bits();
            if bits_match {
                exact += 1;
            }
            if is_subnormal(x) {
                if bits_match {
                    subnormal_exact += 1;
                }
            } else {
                normal_max_ulp = normal_max_ulp.max(ulp);
                if bits_match {
                    normal_exact += 1;
                }
            }
            if !bits_match {
                println!(
                    "D edge `{name}` x={x:?} gpu={gpu:?} cpu={cpu:?} ulp={ulp}"
                );
            }
        }
        println!(
            "D edge_rows: total={} exact={} normal_exact={} normal_max_ulp={} subnormal_exact={} nan_ok={}",
            edge_rows_1d().len(),
            exact,
            normal_exact,
            normal_max_ulp,
            subnormal_exact,
            nan_ok
        );
        assert!(edge_rows_1d().len() >= 15);
    });
}

#[test]
fn sqrt_exact1d_candidate_d_dense_normal_sweep() {
    with_gpu(|ctx| {
        let corpus = dense_normal_corpus_1d();
        let detail = sweep_d_dense_with_probe(ctx, &corpus);
        println!(
            "D dense_normal: tested={} exact_bits={} max_ulp={} class={:?} corrections={} up={} down={}",
            detail.summary.tested,
            detail.summary.exact_bits,
            detail.summary.max_ulp,
            detail.summary.classification,
            detail.correction_count,
            detail.up_count,
            detail.down_count
        );
        for (x, gpu, cpu, ulp) in &detail.worst {
            println!("D dense worst x={x:?} gpu={gpu:?} cpu={cpu:?} ulp={ulp}");
        }
        assert!(detail.summary.tested > 100);
    });
}

#[test]
fn sqrt_exact1d_candidate_d_subnormal_sweep() {
    with_gpu(|ctx| {
        let corpus = subnormal_corpus();
        let detail = sweep_d_subnormal(ctx, &corpus);
        println!(
            "D subnormal: tested={} exact_bits={} max_ulp={} integer_path={} flush={}",
            detail.tested,
            detail.exact_bits,
            detail.max_ulp,
            detail.integer_path_used,
            detail.flush_count
        );
        for (x, gpu, cpu, ulp) in &detail.worst {
            println!(
                "D subnormal worst x={x:?} ({:#x}) gpu={gpu:?} cpu={cpu:?} ulp={ulp}",
                x.to_bits()
            );
        }
        assert!(detail.tested > 2000);
        assert_eq!(detail.integer_path_used, detail.tested);
    });
}

#[test]
fn sqrt_exact1d_candidate_d_contraction_barrier_probe() {
    with_gpu(|ctx| {
        let mut corpus = ab_failure_neighborhood();
        corpus.extend(positive_finite_normal_inputs(&dense_stratified_corpus()));
        corpus.sort_by(|a, b| a.to_bits().cmp(&b.to_bits()));
        corpus.dedup_by(|a, b| a.to_bits() == b.to_bits());
        let probe = run_d_probe(ctx, &corpus);
        println!(
            "D contraction_barrier: tested={} native_mismatch={} d_mismatch={} corrections={} up={} down={} d_changes_vs_native={}",
            probe.tested,
            probe.native_mismatch,
            probe.d_mismatch,
            probe.correction_count,
            probe.up_count,
            probe.down_count,
            probe.d_changes_vs_native
        );
        assert!(probe.tested > 50);
    });
}

#[test]
fn sqrt_exact1d_candidate_b_fallback_still_classified() {
    let wgsl = emit_batch_wgsl(ExactSqrtCandidate::CorrectlyRoundedNewtonTwoProduct, 1);
    assert!(wgsl.contains("sqrt_cr_b"));
    assert!(!wgsl.contains("sqrt_cr_d"));
    with_gpu(|ctx| {
        let corpus = dense_normal_corpus_1d();
        let summary = sweep_candidate(ctx, ExactSqrtCandidate::CorrectlyRoundedNewtonTwoProduct, &corpus);
        println!(
            "B fallback still present: tested={} exact_bits={} max_ulp={} class={:?}",
            summary.tested,
            summary.exact_bits,
            summary.max_ulp,
            summary.classification
        );
        assert!(summary.tested > 100);
    });
}

#[test]
fn sqrt_exact1d_no_exact_authority_promotion() {
    let sqrt0 = sqrt0_descriptor();
    assert_eq!(sqrt0.native_math, NativeMathClass::ApproximateJitOnly);
    assert!(
        sqrt0
            .writes
            .iter()
            .all(|out| out.authority == OutputAuthority::ApproximateDiagnostic)
    );

    let grad0 = grad0_descriptor();
    let mag2 = grad0
        .writes
        .iter()
        .find(|out| out.name == "mag2")
        .expect("mag2 output");
    assert_eq!(mag2.authority, OutputAuthority::ApproximateDiagnostic);

    assert!(matches!(
        validate_exact_kernel_inputs(&sqrt0, &["sqrt_out"]),
        Err(SpecError::JitKernelDescriptorAdmission { .. })
    ));
    assert!(matches!(
        validate_exact_kernel_inputs(&grad0, &["mag2"]),
        Err(SpecError::JitKernelDescriptorAdmission { .. })
    ));

    assert!(
        !landed_jit_kernel_descriptors()
            .iter()
            .any(|desc| desc.id.contains("sqrt_exact")),
        "no exact sqrt kernel descriptor admitted yet"
    );
}

// --- SQRT-EXACT-2E: Candidate E (`CorrectlyRoundedIntegerOnly`) ---

#[derive(Debug, Clone)]
struct EBitSweepDetail {
    tested: usize,
    exact_bits: usize,
    max_ulp: u32,
    flush_count: usize,
    nan_class_only: usize,
    worst: Vec<(u32, u32, u32, u32)>,
}

fn sweep_e_bits(ctx: &GpuContext, input_bits: &[u32]) -> EBitSweepDetail {
    let rows = run_candidate_e_bits(ctx, input_bits);
    let mut exact_bits = 0usize;
    let mut max_ulp = 0u32;
    let mut flush_count = 0usize;
    let mut nan_class_only = 0usize;
    let mut worst = Vec::new();
    for (x_bits, out_bits, _, _) in rows {
        let x = f32::from_bits(x_bits);
        let cpu = x.sqrt();
        let gpu = f32::from_bits(out_bits);
        if cpu.is_nan() {
            if gpu.is_nan() {
                nan_class_only += 1;
            } else {
                worst.push((x_bits, out_bits, cpu.to_bits(), u32::MAX));
            }
            continue;
        }
        if out_bits == cpu.to_bits() {
            exact_bits += 1;
        } else {
            let ulp = ulp_distance(gpu, cpu);
            max_ulp = max_ulp.max(ulp);
            worst.push((x_bits, out_bits, cpu.to_bits(), ulp));
        }
        if out_bits == 0 && cpu.to_bits() != 0 {
            flush_count += 1;
        }
    }
    worst.sort_by(|a, b| b.3.cmp(&a.3));
    worst.truncate(10);
    EBitSweepDetail {
        tested: input_bits.len(),
        exact_bits,
        max_ulp,
        flush_count,
        nan_class_only,
        worst,
    }
}

fn sweep_f_bits(ctx: &GpuContext, input_bits: &[u32]) -> EBitSweepDetail {
    let rows = run_candidate_f_bits(ctx, input_bits);
    let mut exact_bits = 0usize;
    let mut max_ulp = 0u32;
    let mut flush_count = 0usize;
    let mut nan_class_only = 0usize;
    let mut worst = Vec::new();
    for (x_bits, out_bits, _, _) in rows {
        let x = f32::from_bits(x_bits);
        let cpu = x.sqrt();
        let gpu = f32::from_bits(out_bits);
        if cpu.is_nan() {
            if gpu.is_nan() {
                nan_class_only += 1;
            } else {
                worst.push((x_bits, out_bits, cpu.to_bits(), u32::MAX));
            }
            continue;
        }
        if out_bits == cpu.to_bits() {
            exact_bits += 1;
        } else {
            let ulp = ulp_distance(gpu, cpu);
            max_ulp = max_ulp.max(ulp);
            worst.push((x_bits, out_bits, cpu.to_bits(), ulp));
        }
        if out_bits == 0 && cpu.to_bits() != 0 {
            flush_count += 1;
        }
    }
    worst.sort_by(|a, b| b.3.cmp(&a.3));
    worst.truncate(10);
    EBitSweepDetail {
        tested: input_bits.len(),
        exact_bits,
        max_ulp,
        flush_count,
        nan_class_only,
        worst,
    }
}

fn positive_finite_normal_bits(inputs: &[f32]) -> Vec<u32> {
    inputs
        .iter()
        .copied()
        .filter(|x| x.is_finite() && *x > 0.0 && x.is_normal())
        .map(f32::to_bits)
        .collect()
}

fn edge_rows_2e_bits() -> Vec<(&'static str, u32)> {
    edge_rows_1d()
        .into_iter()
        .map(|(name, x)| (name, x.to_bits()))
        .collect()
}

fn candidate_e2_failure_rows_bits() -> Vec<u32> {
    vec![
        0x3f7f_fffe,
        0x3f7f_ffff,
        0x3f80_0001,
        0x3f80_0002,
        0x0080_0000,
        0x0100_0000,
        0x3fff_ffff,
        0x4000_0001,
    ]
}

#[derive(Debug, Clone, Copy)]
struct ExhaustiveRange {
    start: u32,
    end: u32,
    split_index: Option<u32>,
    total_splits: Option<u32>,
}

fn parse_u32_env(name: &str) -> Option<u32> {
    let raw = std::env::var(name).ok()?;
    let trimmed = raw.trim();
    if let Some(hex) = trimmed.strip_prefix("0x").or_else(|| trimmed.strip_prefix("0X")) {
        u32::from_str_radix(hex, 16).ok()
    } else {
        trimmed.parse::<u32>().ok()
    }
}

fn exhaustive_range_from_env() -> ExhaustiveRange {
    const DOMAIN_START: u32 = 0x0000_0000;
    const DOMAIN_END: u32 = 0x7F7F_FFFF;
    const DOMAIN_LEN: u64 = (DOMAIN_END as u64) - (DOMAIN_START as u64) + 1;

    let total_splits = parse_u32_env("SIMTHING_SQRT_E4_TOTAL_SPLITS");
    let split_index = parse_u32_env("SIMTHING_SQRT_E4_SPLIT_INDEX");
    if let (Some(total), Some(index)) = (total_splits, split_index) {
        assert!(total > 0, "SIMTHING_SQRT_E4_TOTAL_SPLITS must be > 0");
        assert!(
            index < total,
            "SIMTHING_SQRT_E4_SPLIT_INDEX must be < SIMTHING_SQRT_E4_TOTAL_SPLITS"
        );
        let total_u64 = total as u64;
        let index_u64 = index as u64;
        let base_len = DOMAIN_LEN / total_u64;
        let rem = DOMAIN_LEN % total_u64;
        let prefix_extra = index_u64.min(rem);
        let start_offset = index_u64 * base_len + prefix_extra;
        let len = base_len + u64::from(index_u64 < rem);
        assert!(len > 0, "split length must be positive");
        let start = DOMAIN_START.wrapping_add(start_offset as u32);
        let end = start.wrapping_add((len - 1) as u32);
        return ExhaustiveRange {
            start,
            end,
            split_index: Some(index),
            total_splits: Some(total),
        };
    }

    let start = parse_u32_env("SIMTHING_SQRT_E4_RANGE_START").unwrap_or(DOMAIN_START);
    let end = parse_u32_env("SIMTHING_SQRT_E4_RANGE_END").unwrap_or(DOMAIN_END);
    assert!(
        start <= end,
        "SIMTHING_SQRT_E4_RANGE_START must be <= SIMTHING_SQRT_E4_RANGE_END"
    );
    assert!(
        end <= DOMAIN_END,
        "SIMTHING_SQRT_E4_RANGE_END must be <= 0x7F7F_FFFF"
    );
    ExhaustiveRange {
        start,
        end,
        split_index: None,
        total_splits: None,
    }
}

#[test]
fn sqrt_exact3e_candidate_e_wgsl_compiles_semantic_free() {
    assert!(!SQRT_CR_E_WGSL.is_empty(), "E WGSL artifact must be non-empty");
    assert_semantic_free(SQRT_CR_E_WGSL);
    assert_exact0_forbidden(SQRT_CR_E_WGSL);
    assert!(SQRT_CR_E_WGSL.contains("fn sqrt_cr_e_bits("));
    let wgsl = emit_e_batch_wgsl(1);
    assert_semantic_free(&wgsl);
    assert_exact0_forbidden(&wgsl);
    with_gpu(|ctx| {
        let rows = run_candidate_e_bits(ctx, &[4.0f32.to_bits()]);
        assert_eq!(rows[0].1, 2.0f32.to_bits());
    });
}

#[test]
fn sqrt_exact3e_candidate_e_no_authoritative_fp_path() {
    assert!(SQRT_CR_E_WGSL.contains("sqrt_cr_e_bits"));
    assert!(!SQRT_CR_E_WGSL.contains("sqrt("));
    assert!(!SQRT_CR_E_WGSL.contains("fma("));
    assert!(!SQRT_CR_E_WGSL.contains("array<f32>"));
    let wgsl = emit_e_batch_wgsl(1);
    assert!(wgsl.contains("array<u32>"));
    assert!(!wgsl.contains("array<f32>"));
    with_gpu(|ctx| {
        let rows = run_candidate_e_bits(ctx, &[1.0f32.to_bits()]);
        let (_, out_bits, _, _) = rows[0];
        assert_eq!(out_bits, 1.0f32.to_bits());
    });
}

#[test]
fn sqrt_exact3e_candidate_e_uses_u32_bit_io() {
    assert!(SQRT_CR_E_WGSL.contains("sqrt_cr_e_bits"));
    let wgsl = emit_e_batch_wgsl(1);
    assert!(wgsl.contains("array<u32>"));
    with_gpu(|ctx| {
        let rows = run_candidate_e_bits(ctx, &[1.0f32.to_bits()]);
        let (_, out_bits, _, _) = rows[0];
        assert_eq!(out_bits, 1.0f32.to_bits());
    });
}

#[test]
fn sqrt_exact3e_candidate_e_artifact_hash_recorded() {
    let hash = fnv1a64_hex(SQRT_CR_E_WGSL);
    println!(
        "sqrt_exact3e_candidate_e_artifact_hash_fnv1a64={hash} path=crates/simthing-driver/tests/wgsl/sqrt_cr_e_candidate.wgsl bytes={}",
        SQRT_CR_E_WGSL.len()
    );
    assert_eq!(hash.len(), 16);
}

#[test]
fn sqrt_exact3e_candidate_e_edge_rows() {
    with_gpu(|ctx| {
        let rows = edge_rows_2e_bits();
        let outputs = run_candidate_e_bits(ctx, &rows.iter().map(|(_, b)| *b).collect::<Vec<_>>());
        let mut exact = 0usize;
        let mut normal_exact = 0usize;
        let mut normal_max_ulp = 0u32;
        let mut subnormal_exact = 0usize;
        let mut nan_class_only = 0usize;
        for ((name, x_bits), (_, out_bits, _, _)) in rows.iter().zip(outputs.iter()) {
            let x = f32::from_bits(*x_bits);
            let cpu = x.sqrt();
            let gpu = f32::from_bits(*out_bits);
            if cpu.is_nan() {
                if gpu.is_nan() {
                    nan_class_only += 1;
                    println!(
                        "E edge `{name}`: NaN class parity (gpu={:#x} cpu={:#x})",
                        out_bits,
                        cpu.to_bits()
                    );
                } else {
                    println!("E edge `{name}` expected NaN class, got {:#x}", out_bits);
                }
                continue;
            }
            let ulp = ulp_distance(gpu, cpu);
            let bits_match = *out_bits == cpu.to_bits();
            if bits_match {
                exact += 1;
            }
            if is_subnormal(x) {
                if bits_match {
                    subnormal_exact += 1;
                }
            } else {
                normal_max_ulp = normal_max_ulp.max(ulp);
                if bits_match {
                    normal_exact += 1;
                }
            }
            if !bits_match {
                println!(
                    "E edge `{name}` x_bits={:#x} out={:#x} cpu={:#x} ulp={}",
                    x_bits,
                    out_bits,
                    cpu.to_bits(),
                    ulp
                );
            }
        }
        println!(
            "E edge_rows: total={} exact={} normal_exact={} normal_max_ulp={} subnormal_exact={} nan_class_only={}",
            rows.len(),
            exact,
            normal_exact,
            normal_max_ulp,
            subnormal_exact,
            nan_class_only
        );
        assert!(rows.len() >= 21);
    });
}

#[test]
fn sqrt_exact3e_candidate_e_subnormal_sweep() {
    with_gpu(|ctx| {
        let bits: Vec<u32> = subnormal_corpus().into_iter().map(f32::to_bits).collect();
        let detail = sweep_e_bits(ctx, &bits);
        println!(
            "E subnormal: tested={} exact_bits={} max_ulp={} flush={} nan_class_only={}",
            detail.tested,
            detail.exact_bits,
            detail.max_ulp,
            detail.flush_count,
            detail.nan_class_only
        );
        for (x_bits, out_bits, cpu_bits, ulp) in &detail.worst {
            println!(
                "E subnormal worst x={:#x} out={:#x} cpu={:#x} ulp={}",
                x_bits, out_bits, cpu_bits, ulp
            );
        }
        assert!(detail.tested > 2000);
    });
}

#[test]
fn sqrt_exact3e_candidate_e_dense_normal_sweep() {
    with_gpu(|ctx| {
        let bits = positive_finite_normal_bits(&dense_normal_corpus_1d());
        let detail = sweep_e_bits(ctx, &bits);
        println!(
            "E dense_normal: tested={} exact_bits={} max_ulp={} flush={} nan_class_only={} class={:?}",
            detail.tested,
            detail.exact_bits,
            detail.max_ulp,
            detail.flush_count,
            detail.nan_class_only,
            classify(detail.max_ulp)
        );
        for (x_bits, out_bits, cpu_bits, ulp) in &detail.worst {
            println!(
                "E dense worst x={:#x} out={:#x} cpu={:#x} ulp={}",
                x_bits, out_bits, cpu_bits, ulp
            );
        }
        assert!(detail.tested > 100);
    });
}

#[test]
fn sqrt_exact3e_candidate_e_compared_to_d_and_e2() {
    with_gpu(|ctx| {
        let dense = positive_finite_normal_inputs(&dense_normal_corpus_1d());
        let dense_bits: Vec<u32> = dense.iter().map(|x| x.to_bits()).collect();
        let d_out = run_candidate_batch(ctx, ExactSqrtCandidate::CorrectlyRoundedHwBitmask, &dense);
        let e_rows = run_candidate_e_bits(ctx, &dense_bits);
        let mut d_mismatch = 0usize;
        let mut e_mismatch = 0usize;
        for ((x, d), (_, e_bits, _, _)) in dense.iter().zip(d_out.iter()).zip(e_rows.iter()) {
            let cpu_bits = x.sqrt().to_bits();
            if d.to_bits() != cpu_bits {
                d_mismatch += 1;
            }
            if *e_bits != cpu_bits {
                e_mismatch += 1;
            }
        }

        let subs = subnormal_corpus();
        let sub_bits: Vec<u32> = subs.iter().map(|x| x.to_bits()).collect();
        let d_sub = run_candidate_batch(ctx, ExactSqrtCandidate::CorrectlyRoundedHwBitmask, &subs);
        let e_sub = run_candidate_e_bits(ctx, &sub_bits);
        let d_sub_flush = subs
            .iter()
            .zip(d_sub.iter())
            .filter(|(x, y)| y.to_bits() == 0 && x.sqrt().to_bits() != 0)
            .count();
        let e_sub_flush = subs
            .iter()
            .zip(e_sub.iter())
            .filter(|(x, (_, e_bits, _, _))| *e_bits == 0 && x.sqrt().to_bits() != 0)
            .count();

        let e2_dense_mismatch = 788usize;
        let e2_sub_flush = 0usize;
        println!(
            "E3_vs_E2_vs_D: dense_d_mismatch={} dense_e2_mismatch={} dense_e3_mismatch={} sub_d_flush={} sub_e2_flush={} sub_e3_flush={}",
            d_mismatch, e2_dense_mismatch, e_mismatch, d_sub_flush, e2_sub_flush, e_sub_flush
        );
        assert!(dense.len() > 100);
    });
}

#[test]
fn sqrt_exact3e_no_exact_authority_promotion() {
    let sqrt0 = sqrt0_descriptor();
    assert_eq!(sqrt0.native_math, NativeMathClass::ApproximateJitOnly);
    assert!(
        sqrt0
            .writes
            .iter()
            .all(|out| out.authority == OutputAuthority::ApproximateDiagnostic)
    );

    let grad0 = grad0_descriptor();
    let mag2 = grad0
        .writes
        .iter()
        .find(|out| out.name == "mag2")
        .expect("mag2 output");
    assert_eq!(mag2.authority, OutputAuthority::ApproximateDiagnostic);

    assert!(matches!(
        validate_exact_kernel_inputs(&sqrt0, &["sqrt_out"]),
        Err(SpecError::JitKernelDescriptorAdmission { .. })
    ));
    assert!(matches!(
        validate_exact_kernel_inputs(&grad0, &["mag2"]),
        Err(SpecError::JitKernelDescriptorAdmission { .. })
    ));

    assert!(
        !landed_jit_kernel_descriptors()
            .iter()
            .any(|desc| desc.id.contains("sqrt_exact")),
        "no exact sqrt kernel descriptor admitted yet"
    );

    let baseline = include_str!("../../simthing-gpu/src/shaders/accumulator_op.wgsl");
    assert!(!baseline.contains("sqrt("));
}

#[test]
#[ignore = "full 2^31 finite non-negative f32 sweep for Candidate E; run with --ignored explicitly"]
fn sqrt_exact3e_candidate_e_full_exhaustive_sweep() {
    with_gpu(|ctx| {
        const DOMAIN_END: u32 = 0x7F7F_FFFF;
        let range = exhaustive_range_from_env();
        let batch = parse_u32_env("SIMTHING_SQRT_E4_BATCH").unwrap_or(1_048_576).max(1);
        let progress_every = parse_u32_env("SIMTHING_SQRT_E4_PROGRESS_EVERY")
            .unwrap_or(64)
            .max(1);
        let mut bits = range.start;
        let mut tested: u64 = 0;
        let mut exact_bits: u64 = 0;
        let mut flush_count: u64 = 0;
        let mut max_ulp = 0u32;
        let mut worst: Option<(u32, u32, u32, u32)> = None;
        let mut batch_idx = 0u64;
        while bits <= range.end {
            let end = bits.saturating_add(batch - 1).min(range.end);
            let batch_bits: Vec<u32> = (bits..=end).collect();
            let rows = run_candidate_e_bits(ctx, &batch_bits);
            for (x_bits, out_bits, _, _) in rows {
                let x = f32::from_bits(x_bits);
                let cpu = x.sqrt();
                let cpu_bits = cpu.to_bits();
                let ulp = ulp_distance(f32::from_bits(out_bits), cpu);
                if out_bits == cpu_bits {
                    exact_bits += 1;
                } else if worst.as_ref().is_none_or(|w| ulp > w.3) {
                    worst = Some((x_bits, out_bits, cpu_bits, ulp));
                }
                if out_bits == 0 && cpu_bits != 0 {
                    flush_count += 1;
                }
                max_ulp = max_ulp.max(ulp);
                tested += 1;
            }
            batch_idx += 1;
            if batch_idx.is_multiple_of(progress_every as u64) || end == range.end {
                println!(
                    "E exhaustive progress: start={:#010x} end={:#010x} tested={} max_ulp={} flush_count={} exact_bits={}",
                    range.start, range.end, tested, max_ulp, flush_count, exact_bits
                );
            }
            bits = end.saturating_add(1);
            if bits == 0 {
                break;
            }
        }

        let expected = (range.end as u64) - (range.start as u64) + 1;
        assert_eq!(
            tested, expected,
            "range coverage mismatch for exhaustive sweep"
        );
        assert_eq!(flush_count, 0, "exhaustive sweep must have flush_count == 0");
        assert_eq!(max_ulp, 0, "Candidate E exhaustive promotion requires max_ulp == 0");

        let split_tag = match (range.split_index, range.total_splits) {
            (Some(idx), Some(total)) => format!("split={idx}/{total}"),
            _ => "split=full_or_explicit_range".to_string(),
        };
        let worst_tag = if let Some((x_bits, out_bits, cpu_bits, ulp)) = worst {
            format!(
                "worst=x:{:#010x},out:{:#010x},cpu:{:#010x},ulp:{}",
                x_bits, out_bits, cpu_bits, ulp
            )
        } else {
            "worst=none".to_string()
        };
        println!(
            "E exhaustive summary: {split_tag} start={:#010x} end={:#010x} tested={} exact_bits={} max_ulp={} flush_count={} {}",
            range.start, range.end, tested, exact_bits, max_ulp, flush_count, worst_tag
        );

        let log_line = format!(
            "split_tag={} start={:#010x} end={:#010x} tested={} exact_bits={} max_ulp={} flush_count={} {}\n",
            split_tag, range.start, range.end, tested, exact_bits, max_ulp, flush_count, worst_tag
        );
        let repo_docs_tests = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/tests");
        std::fs::create_dir_all(&repo_docs_tests).expect("repo docs/tests must exist");
        let batch_log_path = repo_docs_tests.join("phase_m_jit_sqrt_exact4e_exhaustive_batches.log");
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&batch_log_path)
            .expect("open exhaustive batch log");
        use std::io::Write;
        file.write_all(log_line.as_bytes())
            .expect("write exhaustive batch log");

        assert!(
            range.end <= DOMAIN_END,
            "exhaustive range end must stay in finite non-negative domain"
        );
    });
}

// --- SQRT-EXACT-4F: Candidate F (`CorrectlyRoundedHwBitmaskNormalized`) ---

#[test]
fn sqrt_exact4f_candidate_f_wgsl_compiles_semantic_free() {
    assert!(!SQRT_CR_F_WGSL.is_empty(), "F WGSL artifact must be non-empty");
    assert_semantic_free(SQRT_CR_F_WGSL);
    assert_exact0_forbidden(SQRT_CR_F_WGSL);
    assert!(SQRT_CR_F_WGSL.contains("fn sqrt_cr_f_bits("));
    assert!(!SQRT_CR_F_WGSL.contains("sqrt_cr_c"));
    let source = include_str!("phase_m_jit_sqrt_exact_candidate_battery.rs");
    let no_dynamic_name = ["emit_sqrt_cr_f", "_fn"].concat();
    assert!(!source.contains(&no_dynamic_name));
    let no_dynamic_def = ["fn emit_", "sqrt_cr_f("].concat();
    assert!(!source.contains(&no_dynamic_def));
    let wgsl = emit_f_batch_wgsl(1);
    assert_semantic_free(&wgsl);
    assert_exact0_forbidden(&wgsl);
    with_gpu(|ctx| {
        let rows = run_candidate_f_bits(ctx, &[4.0f32.to_bits()]);
        assert_eq!(rows[0].1, 2.0f32.to_bits());
    });
}

#[test]
fn sqrt_exact4f_candidate_f_uses_verbatim_wgsl_artifact() {
    assert!(SQRT_CR_F_WGSL.contains("fn sqrt_cr_f_bits("));
    let batch = emit_f_batch_wgsl(1);
    let probe = emit_f_probe_wgsl(1);
    assert!(
        batch.contains(SQRT_CR_F_WGSL),
        "F batch wrapper must include verbatim artifact as contiguous substring"
    );
    assert!(
        probe.contains(SQRT_CR_F_WGSL),
        "F probe wrapper must include verbatim artifact as contiguous substring"
    );
    assert_eq!(batch.matches(SQRT_CR_F_WGSL).count(), 1);
    assert_eq!(probe.matches(SQRT_CR_F_WGSL).count(), 1);
    let hash = fnv1a64_hex(SQRT_CR_F_WGSL);
    println!(
        "sqrt_exact4f_candidate_f_artifact_hash_fnv1a64={hash} path=crates/simthing-driver/tests/wgsl/sqrt_cr_f_candidate.wgsl bytes={}",
        SQRT_CR_F_WGSL.len()
    );
    assert_eq!(hash.len(), 16);
}

#[test]
fn sqrt_exact4f_candidate_f_uses_u32_bit_io() {
    assert!(SQRT_CR_F_WGSL.contains("fn sqrt_cr_f_bits("));
    let wgsl = emit_f_batch_wgsl(1);
    assert!(wgsl.contains("array<u32>"));
    assert!(!wgsl.contains("array<f32>"));
    with_gpu(|ctx| {
        let rows = run_candidate_f_bits(ctx, &[1.0f32.to_bits()]);
        assert_eq!(rows[0].0, 1.0f32.to_bits());
        assert_eq!(rows[0].1, 1.0f32.to_bits());
    });
}

#[test]
fn sqrt_exact4f_candidate_f_edge_rows() {
    with_gpu(|ctx| {
        let mut rows: Vec<(String, u32)> = edge_rows_2e_bits()
            .into_iter()
            .map(|(name, bits)| (name.to_string(), bits))
            .collect();
        rows.extend(
            candidate_e2_failure_rows_bits()
                .into_iter()
                .map(|bits| (format!("e2_failure_{bits:08x}"), bits)),
        );
        rows.sort_by(|a, b| a.1.cmp(&b.1));
        rows.dedup_by(|a, b| a.1 == b.1);
        let input_bits: Vec<u32> = rows.iter().map(|(_, bits)| *bits).collect();
        let outputs = run_candidate_f_bits(ctx, &input_bits);
        let mut exact = 0usize;
        let mut normal_exact = 0usize;
        let mut normal_max_ulp = 0u32;
        let mut subnormal_exact = 0usize;
        let mut nan_class_only = 0usize;
        for ((name, x_bits), (_, out_bits, _, _)) in rows.iter().zip(outputs.iter()) {
            let x = f32::from_bits(*x_bits);
            let cpu = x.sqrt();
            let gpu = f32::from_bits(*out_bits);
            if cpu.is_nan() {
                assert!(gpu.is_nan(), "F edge `{name}` expected NaN class parity");
                nan_class_only += 1;
                println!(
                    "F edge `{name}` NaN class parity: out={:#x} cpu={:#x}",
                    out_bits,
                    cpu.to_bits()
                );
                continue;
            }
            let bits_match = *out_bits == cpu.to_bits();
            if bits_match {
                exact += 1;
            }
            let ulp = ulp_distance(gpu, cpu);
            if is_subnormal(x) {
                if bits_match {
                    subnormal_exact += 1;
                }
            } else {
                normal_max_ulp = normal_max_ulp.max(ulp);
                if bits_match {
                    normal_exact += 1;
                }
            }
            if !bits_match {
                println!(
                    "F edge mismatch `{name}` x={:#x} out={:#x} cpu={:#x} ulp={}",
                    x_bits,
                    out_bits,
                    cpu.to_bits(),
                    ulp
                );
            }
        }
        println!(
            "F edge_rows: total={} exact={} normal_exact={} normal_max_ulp={} subnormal_exact={} nan_class_only={}",
            rows.len(),
            exact,
            normal_exact,
            normal_max_ulp,
            subnormal_exact,
            nan_class_only
        );
        assert!(rows.len() >= 21);
    });
}

#[test]
fn sqrt_exact4f_candidate_f_subnormal_sweep() {
    with_gpu(|ctx| {
        let bits: Vec<u32> = subnormal_corpus().into_iter().map(f32::to_bits).collect();
        let detail = sweep_f_bits(ctx, &bits);
        println!(
            "F subnormal: tested={} exact_bits={} max_ulp={} flush_count={} worst_rows={}",
            detail.tested,
            detail.exact_bits,
            detail.max_ulp,
            detail.flush_count,
            detail.worst.len()
        );
        for (x_bits, out_bits, cpu_bits, ulp) in &detail.worst {
            println!(
                "F subnormal worst x={:#x} out={:#x} cpu={:#x} ulp={}",
                x_bits, out_bits, cpu_bits, ulp
            );
        }
        assert!(detail.tested > 2000);
    });
}

#[test]
fn sqrt_exact4f_candidate_f_dense_normal_sweep() {
    with_gpu(|ctx| {
        let mut bits = positive_finite_normal_bits(&dense_normal_corpus_1d());
        bits.extend(candidate_e2_failure_rows_bits());
        bits.sort_unstable();
        bits.dedup();
        let detail = sweep_f_bits(ctx, &bits);
        let (probe, _) = run_candidate_f_probe(ctx, &bits);
        println!(
            "F dense_normal: tested={} exact_bits={} max_ulp={} flush_count={} correction_count={} up={} down={} class={:?}",
            detail.tested,
            detail.exact_bits,
            detail.max_ulp,
            detail.flush_count,
            probe.correction_count,
            probe.up_count,
            probe.down_count,
            classify(detail.max_ulp)
        );
        for (x_bits, out_bits, cpu_bits, ulp) in &detail.worst {
            println!(
                "F dense worst x={:#x} out={:#x} cpu={:#x} ulp={}",
                x_bits, out_bits, cpu_bits, ulp
            );
        }
        assert!(detail.tested > 100);
    });
}

#[test]
fn sqrt_exact4f_candidate_f_compared_to_e3() {
    with_gpu(|ctx| {
        let mut dense_bits = positive_finite_normal_bits(&dense_normal_corpus_1d());
        dense_bits.extend(candidate_e2_failure_rows_bits());
        dense_bits.sort_unstable();
        dense_bits.dedup();
        let sub_bits: Vec<u32> = subnormal_corpus().into_iter().map(f32::to_bits).collect();

        let dense_e = sweep_e_bits(ctx, &dense_bits);
        let dense_f = sweep_f_bits(ctx, &dense_bits);
        let sub_e = sweep_e_bits(ctx, &sub_bits);
        let sub_f = sweep_f_bits(ctx, &sub_bits);

        let e_dense_rows = run_candidate_e_bits(ctx, &dense_bits);
        let f_dense_rows = run_candidate_f_bits(ctx, &dense_bits);
        let mut e_dense_mismatch = 0usize;
        let mut f_dense_mismatch = 0usize;
        let mut f_fails_e_pass_rows = Vec::new();
        for (((x_bits, e_bits, _, _), (_, f_bits, _, _)), x_bits_ref) in e_dense_rows
            .iter()
            .zip(f_dense_rows.iter())
            .zip(dense_bits.iter())
        {
            let cpu_bits = f32::from_bits(*x_bits_ref).sqrt().to_bits();
            let e_ok = *e_bits == cpu_bits;
            let f_ok = *f_bits == cpu_bits;
            if !e_ok {
                e_dense_mismatch += 1;
            }
            if !f_ok {
                f_dense_mismatch += 1;
            }
            if !f_ok && e_ok {
                f_fails_e_pass_rows.push((*x_bits, *f_bits, cpu_bits));
            }
        }
        f_fails_e_pass_rows.truncate(8);
        println!(
            "F_vs_E3: dense_e3_max_ulp={} dense_f_max_ulp={} sub_e3_max_ulp={} sub_f_max_ulp={} dense_e3_mismatch={} dense_f_mismatch={} f_fails_e3_pass_rows={}",
            dense_e.max_ulp,
            dense_f.max_ulp,
            sub_e.max_ulp,
            sub_f.max_ulp,
            e_dense_mismatch,
            f_dense_mismatch,
            f_fails_e_pass_rows.len()
        );
        for (x_bits, f_bits, cpu_bits) in &f_fails_e_pass_rows {
            println!(
                "F fails / E3 passes row x={:#x} f={:#x} cpu={:#x}",
                x_bits, f_bits, cpu_bits
            );
        }
        assert!(dense_bits.len() > 100);
        assert!(sub_bits.len() > 2000);
    });
}

#[test]
fn sqrt_exact4f_candidate_f_contraction_probe() {
    with_gpu(|ctx| {
        let mut bits = positive_finite_normal_bits(&dense_normal_corpus_1d());
        bits.extend(candidate_e2_failure_rows_bits());
        bits.sort_unstable();
        bits.dedup();
        let (probe, rows) = run_candidate_f_probe(ctx, &bits);
        let mut reassociation_rows = Vec::new();
        for (x_bits, out_bits, native_bits, _, _, _) in rows {
            let cpu_bits = f32::from_bits(x_bits).sqrt().to_bits();
            if out_bits != cpu_bits && native_bits != cpu_bits {
                reassociation_rows.push((x_bits, out_bits, native_bits, cpu_bits));
            }
        }
        reassociation_rows.truncate(8);
        println!(
            "F contraction_probe: tested={} native_mismatch={} f_mismatch={} corrections={} up={} down={} f_changes_vs_native={} residual_reassociation_rows={}",
            probe.tested,
            probe.native_mismatch,
            probe.f_mismatch,
            probe.correction_count,
            probe.up_count,
            probe.down_count,
            probe.f_changes_vs_native,
            reassociation_rows.len()
        );
        for (x_bits, out_bits, native_bits, cpu_bits) in reassociation_rows {
            println!(
                "F contraction row x={:#x} f={:#x} native={:#x} cpu={:#x}",
                x_bits, out_bits, native_bits, cpu_bits
            );
        }
        assert!(probe.tested > 100);
    });
}

#[test]
fn sqrt_exact4f_no_exact_authority_promotion() {
    let sqrt0 = sqrt0_descriptor();
    assert_eq!(sqrt0.native_math, NativeMathClass::ApproximateJitOnly);
    assert!(
        sqrt0
            .writes
            .iter()
            .all(|out| out.authority == OutputAuthority::ApproximateDiagnostic)
    );

    let grad0 = grad0_descriptor();
    let mag2 = grad0
        .writes
        .iter()
        .find(|out| out.name == "mag2")
        .expect("mag2 output");
    assert_eq!(mag2.authority, OutputAuthority::ApproximateDiagnostic);

    assert!(matches!(
        validate_exact_kernel_inputs(&sqrt0, &["sqrt_out"]),
        Err(SpecError::JitKernelDescriptorAdmission { .. })
    ));
    assert!(matches!(
        validate_exact_kernel_inputs(&grad0, &["mag2"]),
        Err(SpecError::JitKernelDescriptorAdmission { .. })
    ));

    assert!(
        !landed_jit_kernel_descriptors()
            .iter()
            .any(|desc| desc.id.contains("sqrt_exact")),
        "no exact sqrt kernel descriptor admitted yet"
    );

    let baseline = include_str!("../../simthing-gpu/src/shaders/accumulator_op.wgsl");
    assert!(!baseline.contains("sqrt("));
}

#[test]
fn sqrt_exact4f_perf_e3_vs_f_smoke() {
    use std::time::Instant;
    with_gpu(|ctx| {
        let mut state = 0xA1B2_C3D4u32;
        let mut build_bits = |n: usize| -> Vec<u32> {
            let mut out = Vec::with_capacity(n);
            for _ in 0..n {
                state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                let exp = 1u32 + ((state >> 24) % 253u32);
                let mant = state & 0x007f_ffff;
                out.push((exp << 23) | mant);
            }
            out
        };
        for size in [1_000usize, 10_000, 34_000, 100_000] {
            let bits = build_bits(size);
            let t0 = Instant::now();
            let e_rows = run_candidate_e_bits(ctx, &bits);
            let e_elapsed = t0.elapsed();
            let t1 = Instant::now();
            let f_rows = run_candidate_f_bits(ctx, &bits);
            let f_elapsed = t1.elapsed();
            assert_eq!(e_rows.len(), size);
            assert_eq!(f_rows.len(), size);
            let ratio = if e_elapsed.as_nanos() == 0 {
                0.0
            } else {
                f_elapsed.as_secs_f64() / e_elapsed.as_secs_f64()
            };
            println!(
                "F perf_smoke: inputs={} dispatch_count=1 includes_readback=true e3_time_ms={:.3} f_time_ms={:.3} f_over_e3_ratio={:.4}",
                size,
                e_elapsed.as_secs_f64() * 1000.0,
                f_elapsed.as_secs_f64() * 1000.0,
                ratio
            );
        }
        println!(
            "F perf_smoke_note: optional ignored large run not executed by default (suggested size=1000000)"
        );
    });
}

#[test]
fn sqrt_exact4f_perf_is_not_authority() {
    let sqrt0 = sqrt0_descriptor();
    assert_eq!(sqrt0.native_math, NativeMathClass::ApproximateJitOnly);
    assert!(
        !landed_jit_kernel_descriptors()
            .iter()
            .any(|desc| desc.id.contains("sqrt_exact")),
        "performance measurements must not promote exact authority"
    );
    println!(
        "F perf_not_authority: throughput evidence is not admission authority; exhaustive max_ulp==0 proof is still required and approximate performance mode is a separate product-policy gate"
    );
}

#[test]
#[ignore = "full 2^31 finite non-negative f32 sweep for Candidate F; run with --ignored explicitly"]
fn sqrt_exact4f_candidate_f_full_exhaustive_sweep() {
    with_gpu(|ctx| {
        let batch = parse_u32_env("SIMTHING_SQRT_F4_BATCH")
            .unwrap_or(1_048_576)
            .max(1);
        let mut bits = 0u32;
        let mut tested: u64 = 0;
        let mut exact_bits: u64 = 0;
        let mut max_ulp = 0u32;
        while bits <= 0x7F7F_FFFF {
            let end = bits.saturating_add(batch - 1).min(0x7F7F_FFFF);
            let batch_bits: Vec<u32> = (bits..=end).collect();
            let rows = run_candidate_f_bits(ctx, &batch_bits);
            for (x_bits, out_bits, _, _) in rows {
                let cpu = f32::from_bits(x_bits).sqrt();
                let cpu_bits = cpu.to_bits();
                if out_bits == cpu_bits {
                    exact_bits += 1;
                }
                max_ulp = max_ulp.max(ulp_distance(f32::from_bits(out_bits), cpu));
                tested += 1;
            }
            bits = end.saturating_add(1);
            if bits == 0 {
                break;
            }
        }
        println!(
            "F exhaustive: tested={} exact_bits={} max_ulp={}",
            tested, exact_bits, max_ulp
        );
        assert_eq!(tested, 0x7F80_0000u64);
        assert_eq!(max_ulp, 0, "Candidate F exhaustive promotion requires max_ulp == 0");
    });
}
