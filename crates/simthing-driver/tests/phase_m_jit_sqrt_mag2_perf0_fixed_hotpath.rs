//! SQRT-MAG2-PERF-0 — Performance decomposition for exact fixed-point mag2 + F sqrt hot path.
//!
//! Decomposes 34k FIELD_POLICY hot-path cost (readback, mag2-only, F-only, combined) and probes
//! exact-preserving optimizations. Does not weaken exactness or change Candidate F.

use std::sync::Mutex;
use std::time::Instant;

use simthing_gpu::GpuContext;

static GPU_MUTEX: Mutex<()> = Mutex::new(());
const SQRT_CR_F_WGSL: &str = include_str!("wgsl/sqrt_cr_f_candidate.wgsl");
const ROW_STRIDE: u32 = 6;
const Q16_SCALE: u32 = 1 << 16;
const Q12_SCALE: u32 = 1 << 12;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BenchPath {
    ReadbackBaseline,
    Mag2OnlyQ16,
    Mag2OnlyQ16LoOnlyConv,
    Mag2OnlyQ12,
    FSqrtOnly,
    CombinedQ16,
    CombinedQ12,
    SplitMag2ThenSqrt,
}

struct BenchOutcome {
    outputs: Vec<(u64, u32, u32)>,
    elapsed_ms: f64,
    dispatches: u32,
    includes_readback: bool,
}

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let ctx = GpuContext::new_blocking().expect("GPU required");
    f(&ctx);
}

fn f32_to_q(scale: u32, v: f32) -> i32 {
    (v * scale as f32).round() as i32
}

fn f32_to_q16(v: f32) -> i32 {
    f32_to_q(Q16_SCALE, v)
}

fn f32_to_q12(v: f32) -> i32 {
    f32_to_q(Q12_SCALE, v)
}

fn cpu_mag2_sum(dx_fixed: i32, dy_fixed: i32) -> u64 {
    let dx = i64::from(dx_fixed);
    let dy = i64::from(dy_fixed);
    (dx * dx + dy * dy) as u64
}

fn mag2_sum_to_f32_bits_hi_lo(sum_lo: u32, sum_hi: u32, scale_sq: f32) -> u32 {
    (sum_hi as f32 + sum_lo as f32 / scale_sq).to_bits()
}

fn mag2_sum_to_f32_bits_lo_only(sum_lo: u32, scale_sq: f32) -> u32 {
    (sum_lo as f32 / scale_sq).to_bits()
}

fn cpu_mag2_bits_q16(dx: i32, dy: i32) -> u32 {
    let sum = cpu_mag2_sum(dx, dy);
    mag2_sum_to_f32_bits_hi_lo(sum as u32, (sum >> 32) as u32, 4294967296.0)
}

fn cpu_mag2_bits_q12(dx: i32, dy: i32) -> u32 {
    let sum = cpu_mag2_sum(dx, dy);
    mag2_sum_to_f32_bits_hi_lo(
        sum as u32,
        (sum >> 32) as u32,
        (Q12_SCALE * Q12_SCALE) as f32,
    )
}

fn cpu_mag_bits(mag2_bits: u32) -> u32 {
    f32::from_bits(mag2_bits).sqrt().to_bits()
}

fn ulp_distance(a: u32, b: u32) -> u32 {
    if a == b {
        return 0;
    }
    let (a, b) = (f32::from_bits(a), f32::from_bits(b));
    if a.is_nan() || b.is_nan() {
        return u32::MAX;
    }
    let diff = (a - b).abs();
    if diff == 0.0 {
        return 0;
    }
    let scale = a.abs().max(b.abs());
    if scale == 0.0 {
        return 1;
    }
    (diff / f32::EPSILON / scale).ceil() as u32
}

fn limb_arith_wgsl() -> &'static str {
    r#"
fn abs_fixed(v: i32) -> u32 {
    if (v < 0) { return u32(0u - bitcast<u32>(v)); }
    return u32(v);
}

fn mul_u32_wide(a: u32, b: u32) -> vec2<u32> {
    let mask = 65535u;
    let a0 = a & mask;
    let a1 = a >> 16u;
    let b0 = b & mask;
    let b1 = b >> 16u;
    let z0 = a0 * b0;
    let z1 = a0 * b1;
    let z2 = a1 * b0;
    let z3 = a1 * b1;
    let t = (z0 >> 16u) + (z1 & mask) + (z2 & mask);
    let lo = (z0 & mask) | ((t & mask) << 16u);
    let hi = z3 + (z1 >> 16u) + (z2 >> 16u) + (t >> 16u);
    return vec2<u32>(lo, hi);
}

fn add_u64_wide(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    let lo = a.x + b.x;
    let carry = select(0u, 1u, lo < a.x);
    let hi = a.y + b.y + carry;
    return vec2<u32>(lo, hi);
}

fn mag2_sum_q16(dx_fixed: i32, dy_fixed: i32) -> vec2<u32> {
    let dx2 = mul_u32_wide(abs_fixed(dx_fixed), abs_fixed(dx_fixed));
    let dy2 = mul_u32_wide(abs_fixed(dy_fixed), abs_fixed(dy_fixed));
    return add_u64_wide(dx2, dy2);
}
"#
}

fn emit_wgsl(path: BenchPath, batch_count: u32) -> String {
    let limb = limb_arith_wgsl();
    let q12_scale_sq = (Q12_SCALE * Q12_SCALE) as f32;
    let body = match path {
        BenchPath::ReadbackBaseline => "    data[base + 2u] = data[base];\n".to_string(),
        BenchPath::Mag2OnlyQ16 => format!(
            r#"{limb}
fn mag2_u64_q16_to_f32_bits(sum: vec2<u32>) -> u32 {{
    return bitcast<u32>(f32(sum.y) + f32(sum.x) / 4294967296.0);
}}
@group(0) @binding(0) var<storage, read_write> data: array<u32>;
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {{
    let i = gid.x;
    if (i >= {batch_count}u) {{ return; }}
    let base = i * {stride}u;
    let sum = mag2_sum_q16(bitcast<i32>(data[base]), bitcast<i32>(data[base + 1u]));
    data[base + 2u] = sum.x;
    data[base + 3u] = sum.y;
    data[base + 4u] = mag2_u64_q16_to_f32_bits(sum);
    data[base + 5u] = 0u;
}}
"#,
            limb = limb,
            batch_count = batch_count,
            stride = ROW_STRIDE,
        ),
        BenchPath::Mag2OnlyQ16LoOnlyConv => format!(
            r#"{limb}
@group(0) @binding(0) var<storage, read_write> data: array<u32>;
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {{
    let i = gid.x;
    if (i >= {batch_count}u) {{ return; }}
    let base = i * {stride}u;
    let sum = mag2_sum_q16(bitcast<i32>(data[base]), bitcast<i32>(data[base + 1u]));
    data[base + 2u] = sum.x;
    data[base + 3u] = sum.y;
    data[base + 4u] = bitcast<u32>(f32(sum.x) / 4294967296.0);
    data[base + 5u] = 0u;
}}
"#,
            limb = limb,
            batch_count = batch_count,
            stride = ROW_STRIDE,
        ),
        BenchPath::Mag2OnlyQ12 => format!(
            r#"{limb}
fn mag2_u64_q12_to_f32_bits(sum: vec2<u32>) -> u32 {{
    return bitcast<u32>(f32(sum.y) + f32(sum.x) / {q12_scale_sq});
}}
@group(0) @binding(0) var<storage, read_write> data: array<u32>;
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {{
    let i = gid.x;
    if (i >= {batch_count}u) {{ return; }}
    let base = i * {stride}u;
    let sum = mag2_sum_q16(bitcast<i32>(data[base]), bitcast<i32>(data[base + 1u]));
    data[base + 2u] = sum.x;
    data[base + 3u] = sum.y;
    data[base + 4u] = mag2_u64_q12_to_f32_bits(sum);
    data[base + 5u] = 0u;
}}
"#,
            limb = limb,
            batch_count = batch_count,
            stride = ROW_STRIDE,
            q12_scale_sq = q12_scale_sq,
        ),
        BenchPath::FSqrtOnly => format!(
            r#"{f}
@group(0) @binding(0) var<storage, read_write> data: array<u32>;
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {{
    let i = gid.x;
    if (i >= {batch_count}u) {{ return; }}
    let base = i * {stride}u;
    data[base + 5u] = sqrt_cr_f_bits(data[base + 4u]);
}}
"#,
            f = SQRT_CR_F_WGSL,
            batch_count = batch_count,
            stride = ROW_STRIDE,
        ),
        BenchPath::CombinedQ16 => format!(
            r#"{f}
{limb}
fn mag2_u64_q16_to_f32_bits(sum: vec2<u32>) -> u32 {{
    return bitcast<u32>(f32(sum.y) + f32(sum.x) / 4294967296.0);
}}
@group(0) @binding(0) var<storage, read_write> data: array<u32>;
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {{
    let i = gid.x;
    if (i >= {batch_count}u) {{ return; }}
    let base = i * {stride}u;
    let sum = mag2_sum_q16(bitcast<i32>(data[base]), bitcast<i32>(data[base + 1u]));
    data[base + 2u] = sum.x;
    data[base + 3u] = sum.y;
    let mag2_bits = mag2_u64_q16_to_f32_bits(sum);
    data[base + 4u] = mag2_bits;
    data[base + 5u] = sqrt_cr_f_bits(mag2_bits);
}}
"#,
            f = SQRT_CR_F_WGSL,
            limb = limb,
            batch_count = batch_count,
            stride = ROW_STRIDE,
        ),
        BenchPath::CombinedQ12 => format!(
            r#"{f}
{limb}
fn mag2_u64_q12_to_f32_bits(sum: vec2<u32>) -> u32 {{
    return bitcast<u32>(f32(sum.y) + f32(sum.x) / {q12_scale_sq});
}}
@group(0) @binding(0) var<storage, read_write> data: array<u32>;
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {{
    let i = gid.x;
    if (i >= {batch_count}u) {{ return; }}
    let base = i * {stride}u;
    let sum = mag2_sum_q16(bitcast<i32>(data[base]), bitcast<i32>(data[base + 1u]));
    data[base + 2u] = sum.x;
    data[base + 3u] = sum.y;
    let mag2_bits = mag2_u64_q12_to_f32_bits(sum);
    data[base + 4u] = mag2_bits;
    data[base + 5u] = sqrt_cr_f_bits(mag2_bits);
}}
"#,
            f = SQRT_CR_F_WGSL,
            limb = limb,
            batch_count = batch_count,
            stride = ROW_STRIDE,
            q12_scale_sq = q12_scale_sq,
        ),
        BenchPath::SplitMag2ThenSqrt => format!(
            r#"{f}
{limb}
fn mag2_u64_q16_to_f32_bits(sum: vec2<u32>) -> u32 {{
    return bitcast<u32>(f32(sum.y) + f32(sum.x) / 4294967296.0);
}}
@group(0) @binding(0) var<storage, read_write> data: array<u32>;
@compute @workgroup_size(64)
fn mag2_pass(@builtin(global_invocation_id) gid: vec3<u32>) {{
    let i = gid.x;
    if (i >= {batch_count}u) {{ return; }}
    let base = i * {stride}u;
    let sum = mag2_sum_q16(bitcast<i32>(data[base]), bitcast<i32>(data[base + 1u]));
    data[base + 2u] = sum.x;
    data[base + 3u] = sum.y;
    data[base + 4u] = mag2_u64_q16_to_f32_bits(sum);
}}
@compute @workgroup_size(64)
fn sqrt_pass(@builtin(global_invocation_id) gid: vec3<u32>) {{
    let i = gid.x;
    if (i >= {batch_count}u) {{ return; }}
    let base = i * {stride}u;
    data[base + 5u] = sqrt_cr_f_bits(data[base + 4u]);
}}
"#,
            f = SQRT_CR_F_WGSL,
            limb = limb,
            batch_count = batch_count,
            stride = ROW_STRIDE,
        ),
    };

    if matches!(
        path,
        BenchPath::Mag2OnlyQ16
            | BenchPath::Mag2OnlyQ16LoOnlyConv
            | BenchPath::Mag2OnlyQ12
            | BenchPath::FSqrtOnly
            | BenchPath::CombinedQ16
            | BenchPath::CombinedQ12
            | BenchPath::SplitMag2ThenSqrt
    ) {
        return body;
    }

    format!(
        r#"{f}
@group(0) @binding(0) var<storage, read_write> data: array<u32>;
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {{
    let i = gid.x;
    if (i >= {batch_count}u) {{ return; }}
    let base = i * {stride}u;
{body}}}"#,
        f = SQRT_CR_F_WGSL,
        batch_count = batch_count,
        stride = ROW_STRIDE,
        body = body,
    )
}

fn init_buffer(pairs: &[(i32, i32)], prefill_mag2: bool) -> Vec<u32> {
    let n = pairs.len();
    let mut data = vec![0u32; n * ROW_STRIDE as usize];
    for (i, (dx, dy)) in pairs.iter().enumerate() {
        let base = i * ROW_STRIDE as usize;
        data[base] = *dx as u32;
        data[base + 1] = *dy as u32;
        if prefill_mag2 {
            data[base + 4] = cpu_mag2_bits_q16(*dx, *dy);
        }
    }
    data
}

fn run_bench(
    ctx: &GpuContext,
    pairs: &[(i32, i32)],
    path: BenchPath,
    repeat_dispatches: u32,
    do_readback: bool,
) -> BenchOutcome {
    use wgpu::util::DeviceExt;
    let n = pairs.len() as u32;
    let prefill = path == BenchPath::FSqrtOnly;
    let data = init_buffer(pairs, prefill);
    let wgsl = emit_wgsl(path, n);
    let device = &ctx.device;
    let queue = &ctx.queue;
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("jit_mag2_perf0"),
        source: wgpu::ShaderSource::Wgsl(wgsl.into()),
    });
    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("jit_mag2_perf0_bgl"),
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
    let pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("jit_mag2_perf0_pl"),
        bind_group_layouts: &[&bgl],
        push_constant_ranges: &[],
    });

    let (pipeline, pipeline2) = if path == BenchPath::SplitMag2ThenSqrt {
        let p1 = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("jit_mag2_perf0_mag2"),
            layout: Some(&pl),
            module: &module,
            entry_point: "mag2_pass",
            compilation_options: Default::default(),
            cache: None,
        });
        let p2 = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("jit_mag2_perf0_sqrt"),
            layout: Some(&pl),
            module: &module,
            entry_point: "sqrt_pass",
            compilation_options: Default::default(),
            cache: None,
        });
        (p1, Some(p2))
    } else {
        let p = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("jit_mag2_perf0_main"),
            layout: Some(&pl),
            module: &module,
            entry_point: "main",
            compilation_options: Default::default(),
            cache: None,
        });
        (p, None)
    };

    let bytes = std::mem::size_of_val(data.as_slice()) as u64;
    let storage = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("jit_mag2_perf0_storage"),
        contents: bytemuck::cast_slice(&data),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("jit_mag2_perf0_bg"),
        layout: &bgl,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage.as_entire_binding(),
        }],
    });

    let t0 = Instant::now();
    let groups = n.div_ceil(64);
    for _ in 0..repeat_dispatches {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("jit_mag2_perf0_enc"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("jit_mag2_perf0_pass"),
                timestamp_writes: None,
            });
            pass.set_bind_group(0, &bg, &[]);
            pass.set_pipeline(&pipeline);
            pass.dispatch_workgroups(groups, 1, 1);
            if let Some(p2) = &pipeline2 {
                pass.set_pipeline(p2);
                pass.dispatch_workgroups(groups, 1, 1);
            }
        }
        queue.submit(Some(encoder.finish()));
    }

    let mut out = Vec::new();
    if do_readback {
        let staging = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("jit_mag2_perf0_readback"),
            size: bytes,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("jit_mag2_perf0_readback_enc"),
        });
        enc2.copy_buffer_to_buffer(&storage, 0, &staging, 0, bytes);
        queue.submit(Some(enc2.finish()));
        let slice = staging.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        device.poll(wgpu::Maintain::Wait);
        let mapped = slice.get_mapped_range();
        let raw: Vec<u32> = bytemuck::cast_slice(&mapped).to_vec();
        drop(mapped);
        staging.unmap();
        out = pairs
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let base = i * ROW_STRIDE as usize;
                let sum = u64::from(raw[base + 2]) | (u64::from(raw[base + 3]) << 32);
                (sum, raw[base + 4], raw[base + 5])
            })
            .collect();
    } else {
        device.poll(wgpu::Maintain::Wait);
    }

    let elapsed = t0.elapsed();
    BenchOutcome {
        outputs: out,
        elapsed_ms: elapsed.as_secs_f64() * 1000.0,
        dispatches: repeat_dispatches * if pipeline2.is_some() { 2 } else { 1 },
        includes_readback: do_readback,
    }
}

fn gradient_samples() -> Vec<f32> {
    vec![
        0.0, 0.001, 0.002, -0.001, -0.002, 0.005, -0.005, 0.01, -0.01, 0.02, -0.02, 0.05, -0.05,
        0.1, -0.1, 0.25, -0.25, 0.5, -0.5, 1.0, -1.0, 2.0, -2.0, 3.0, 4.0, 5.0, 8.0, 16.0,
    ]
}

fn dense_fixed_pairs_q16() -> Vec<(i32, i32)> {
    let mut out = Vec::new();
    for &dx in gradient_samples().iter() {
        for &dy in gradient_samples().iter() {
            out.push((f32_to_q16(dx), f32_to_q16(dy)));
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    out.dedup();
    out
}

fn dense_fixed_pairs_q12() -> Vec<(i32, i32)> {
    let mut out = Vec::new();
    for &dx in gradient_samples().iter() {
        for &dy in gradient_samples().iter() {
            out.push((f32_to_q12(dx), f32_to_q12(dy)));
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    out.dedup();
    out
}

fn mobile_simthing_fixed_pairs_q12(count: usize) -> Vec<(i32, i32)> {
    let samples = gradient_samples();
    let mut out = Vec::with_capacity(count);
    let mut state = 0x5345_4144u32;
    for _ in 0..count {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let dx = samples[(state as usize) % samples.len()];
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let dy = samples[(state as usize) % samples.len()];
        out.push((f32_to_q12(dx), f32_to_q12(dy)));
    }
    out
}

fn mobile_simthing_fixed_pairs(count: usize) -> Vec<(i32, i32)> {
    let samples = gradient_samples();
    let mut out = Vec::with_capacity(count);
    let mut state = 0x5345_4144u32;
    for _ in 0..count {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let dx = samples[(state as usize) % samples.len()];
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let dy = samples[(state as usize) % samples.len()];
        out.push((f32_to_q16(dx), f32_to_q16(dy)));
    }
    out
}

fn print_timing(label: &str, n: usize, outcome: &BenchOutcome) {
    let per = outcome.elapsed_ms * 1000.0 / n as f64;
    println!(
        "{label}: rows={n} dispatches={} includes_readback={} elapsed_ms={:.3} per_entity_us={per:.4}",
        outcome.dispatches, outcome.includes_readback, outcome.elapsed_ms
    );
}

const N34K: usize = 34_000;

#[test]
fn sqrt_mag2_perf0_candidate_b_lo_only_conversion_rejected() {
    with_gpu(|ctx| {
        let rows = dense_fixed_pairs_q16();
        let mut hi_nonzero = 0usize;
        let mut lo_only_mismatch = 0usize;
        for (dx, dy) in &rows {
            let sum = cpu_mag2_sum(*dx, *dy);
            let hi = (sum >> 32) as u32;
            if hi != 0 {
                hi_nonzero += 1;
            }
            let lo = sum as u32;
            let hi_lo = mag2_sum_to_f32_bits_hi_lo(lo, hi, 4294967296.0);
            let lo_only = mag2_sum_to_f32_bits_lo_only(lo, 4294967296.0);
            if hi_lo != lo_only {
                lo_only_mismatch += 1;
            }
        }
        let outcome = run_bench(ctx, &rows, BenchPath::Mag2OnlyQ16LoOnlyConv, 1, true);
        let mut gpu_lo_only_wrong = 0usize;
        for ((_, mag2, _), (dx, dy)) in outcome.outputs.iter().zip(rows.iter()) {
            if *mag2 != cpu_mag2_bits_q16(*dx, *dy) {
                gpu_lo_only_wrong += 1;
            }
        }
        println!(
            "sqrt_mag2_perf0_candidate_b: hi_nonzero={hi_nonzero}/784 lo_only_mismatch={lo_only_mismatch} gpu_wrong={gpu_lo_only_wrong} verdict=REJECTED_for_full_FIELD_POLICY_range"
        );
        assert!(hi_nonzero > 0, "need hi!=0 rows in dense corpus");
        assert!(lo_only_mismatch > 0);
        assert!(gpu_lo_only_wrong > 0);
    });
}

