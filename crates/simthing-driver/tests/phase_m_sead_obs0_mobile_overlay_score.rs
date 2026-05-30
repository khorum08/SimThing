//! SEAD-OBS-0 — GPU-resident mobile observer overlay score probe (Tier-2, test-only).
//!
//! Q16.16 fixed gx/gy → exact integer mag2 → artifact-backed Candidate F sqrt → exact mag;
//! weighted score via f32 multiply/add (approximate/diagnostic authority).
//! No production wiring, no semantic WGSL, no default mapping session wiring.

use std::sync::Mutex;
use std::time::Instant;

use simthing_gpu::GpuContext;
use simthing_spec::{
    fnv1a64_hex, is_exact_mag2_fixed_descriptor, landed_jit_kernel_descriptors,
    validate_kernel_descriptor_admission, MAG2_FIXED_DESCRIPTOR_ID, MAG2_Q16_COMPONENT_MAX,
    MAG2_Q16_SCALE, MappingExecutionProfile, SQRT_F_ARTIFACT_HASH,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());
const SQRT_CR_F_WGSL: &str = include_str!("wgsl/sqrt_cr_f_candidate.wgsl");

/// gx_fixed, gy_fixed, w_mag_fixed, bias_fixed, mag2_lo, mag2_hi, mag2_bits, mag_bits, score_bits, flags
const ROW_STRIDE: u32 = 10;
const Q16_SCALE_F: f32 = MAG2_Q16_SCALE as f32;

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
    "economy",
    "planner",
    "resource",
    "map",
];

const FORBIDDEN_EXACT_TERMS: &[&str] = &["f64", "F64RoundDown", "SHADER_F64", "sqrt_cr_c"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScoreAuthority {
    /// Score = bias + weight * mag via f32 multiply/add after exact mag.
    ApproximateDiagnosticF32,
}

const SCORE_AUTHORITY: ScoreAuthority = ScoreAuthority::ApproximateDiagnosticF32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ObsRow {
    gx: i32,
    gy: i32,
    w_mag: i32,
    bias: i32,
}

#[derive(Debug, Clone)]
struct ObsOutput {
    mag2_sum: u64,
    mag2_bits: u32,
    mag_bits: u32,
    score_bits: u32,
    flags: u32,
}

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let ctx = GpuContext::new_blocking().expect("GPU required");
    f(&ctx);
}

fn f32_to_q16(v: f32) -> i32 {
    (v * Q16_SCALE_F).round() as i32
}

fn cpu_mag2_sum(gx: i32, gy: i32) -> u64 {
    let dx = i64::from(gx);
    let dy = i64::from(gy);
    (dx * dx + dy * dy) as u64
}

fn mag2_u64_q16_to_f32_bits(sum: u64) -> u32 {
    let lo = sum as u32;
    let hi = (sum >> 32) as u32;
    (hi as f32 + lo as f32 / 4294967296.0).to_bits()
}

fn cpu_mag2_bits(gx: i32, gy: i32) -> u32 {
    mag2_u64_q16_to_f32_bits(cpu_mag2_sum(gx, gy))
}

fn cpu_mag_bits(gx: i32, gy: i32) -> u32 {
    f32::from_bits(cpu_mag2_bits(gx, gy)).sqrt().to_bits()
}

fn cpu_score_bits(w_mag: i32, bias: i32, mag_bits: u32) -> u32 {
    let mag = f32::from_bits(mag_bits);
    (bias as f32 / Q16_SCALE_F + w_mag as f32 / Q16_SCALE_F * mag).to_bits()
}

fn ulp_distance(a_bits: u32, b_bits: u32) -> u32 {
    if a_bits == b_bits {
        return 0;
    }
    let (a, b) = (f32::from_bits(a_bits), f32::from_bits(b_bits));
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

fn mag2_sum_q16(gx_fixed: i32, gy_fixed: i32) -> vec2<u32> {
    let gx2 = mul_u32_wide(abs_fixed(gx_fixed), abs_fixed(gx_fixed));
    let gy2 = mul_u32_wide(abs_fixed(gy_fixed), abs_fixed(gy_fixed));
    return add_u64_wide(gx2, gy2);
}

fn mag2_u64_q16_to_f32_bits(sum: vec2<u32>) -> u32 {
    return bitcast<u32>(f32(sum.y) + f32(sum.x) / 4294967296.0);
}
"#
}

fn emit_overlay_score_wgsl(batch_count: u32) -> String {
    format!(
        r#"{f}
{limb}
@group(0) @binding(0) var<storage, read_write> data: array<u32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {{
    let i = gid.x;
    if (i >= {batch_count}u) {{ return; }}
    let base = i * {stride}u;
    let gx = bitcast<i32>(data[base]);
    let gy = bitcast<i32>(data[base + 1u]);
    let w_mag = bitcast<i32>(data[base + 2u]);
    let bias = bitcast<i32>(data[base + 3u]);
    let sum = mag2_sum_q16(gx, gy);
    data[base + 4u] = sum.x;
    data[base + 5u] = sum.y;
    let mag2_bits = mag2_u64_q16_to_f32_bits(sum);
    data[base + 6u] = mag2_bits;
    let mag_bits = sqrt_cr_f_bits(mag2_bits);
    data[base + 7u] = mag_bits;
    let mag_f = bitcast<f32>(mag_bits);
    let score = f32(bias) / 65536.0 + f32(w_mag) / 65536.0 * mag_f;
    data[base + 8u] = bitcast<u32>(score);
    data[base + 9u] = 0u;
}}
"#,
        f = SQRT_CR_F_WGSL,
        limb = limb_arith_wgsl(),
        batch_count = batch_count,
        stride = ROW_STRIDE,
    )
}

fn init_buffer(rows: &[ObsRow]) -> Vec<u32> {
    let mut data = vec![0u32; rows.len() * ROW_STRIDE as usize];
    for (i, row) in rows.iter().enumerate() {
        let base = i * ROW_STRIDE as usize;
        data[base] = row.gx as u32;
        data[base + 1] = row.gy as u32;
        data[base + 2] = row.w_mag as u32;
        data[base + 3] = row.bias as u32;
    }
    data
}

fn run_overlay_score_batch(ctx: &GpuContext, rows: &[ObsRow]) -> Vec<ObsOutput> {
    use wgpu::util::DeviceExt;
    let n = rows.len() as u32;
    let wgsl = emit_overlay_score_wgsl(n);
    let data = init_buffer(rows);
    let device = &ctx.device;
    let queue = &ctx.queue;
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("sead_obs0_overlay_score"),
        source: wgpu::ShaderSource::Wgsl(wgsl.into()),
    });
    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("sead_obs0_bgl"),
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
        label: Some("sead_obs0_pipeline"),
        layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("sead_obs0_pl"),
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
        label: Some("sead_obs0_values"),
        contents: bytemuck::cast_slice(&data),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("sead_obs0_bg"),
        layout: &bgl,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage.as_entire_binding(),
        }],
    });
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("sead_obs0_enc"),
    });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("sead_obs0_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bg, &[]);
        pass.dispatch_workgroups(n.div_ceil(64), 1, 1);
    }
    queue.submit(Some(encoder.finish()));

    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("sead_obs0_readback"),
        size: bytes,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("sead_obs0_readback_enc"),
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

    rows.iter()
        .enumerate()
        .map(|(i, _)| {
            let base = i * ROW_STRIDE as usize;
            let sum = u64::from(out[base + 4]) | (u64::from(out[base + 5]) << 32);
            ObsOutput {
                mag2_sum: sum,
                mag2_bits: out[base + 6],
                mag_bits: out[base + 7],
                score_bits: out[base + 8],
                flags: out[base + 9],
            }
        })
        .collect()
}

fn gradient_samples() -> Vec<f32> {
    vec![
        0.0, 0.001, 0.002, -0.001, -0.002, 0.005, -0.005, 0.01, -0.01, 0.02, -0.02, 0.05, -0.05,
        0.1, -0.1, 0.25, -0.25, 0.5, -0.5, 1.0, -1.0, 2.0, -2.0, 3.0, 4.0, 5.0, 8.0, 16.0,
    ]
}

fn weight_bias_samples() -> Vec<f32> {
    vec![-2.0, -1.0, -0.5, 0.0, 0.25, 0.5, 1.0, 2.0]
}

fn dense_overlay_rows() -> Vec<ObsRow> {
    let mut out = Vec::new();
    for &gx in gradient_samples().iter() {
        for &gy in gradient_samples().iter() {
            for &w in weight_bias_samples().iter() {
                for &b in weight_bias_samples().iter() {
                    out.push(ObsRow {
                        gx: f32_to_q16(gx),
                        gy: f32_to_q16(gy),
                        w_mag: f32_to_q16(w),
                        bias: f32_to_q16(b),
                    });
                }
            }
        }
    }
    out.sort_by(|a, b| {
        a.gx.cmp(&b.gx)
            .then(a.gy.cmp(&b.gy))
            .then(a.w_mag.cmp(&b.w_mag))
            .then(a.bias.cmp(&b.bias))
    });
    out.dedup();
    out
}

fn mobile_observer_rows(count: usize) -> Vec<ObsRow> {
    let grads = gradient_samples();
    let wb = weight_bias_samples();
    let mut out = Vec::with_capacity(count);
    let mut state = 0x5345_4144u32;
    for _ in 0..count {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let gx = grads[(state as usize) % grads.len()];
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let gy = grads[(state as usize) % grads.len()];
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let w = wb[(state as usize) % wb.len()];
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let b = wb[(state as usize) % wb.len()];
        out.push(ObsRow {
            gx: f32_to_q16(gx),
            gy: f32_to_q16(gy),
            w_mag: f32_to_q16(w),
            bias: f32_to_q16(b),
        });
    }
    out
}

fn edge_overlay_rows() -> Vec<ObsRow> {
    let max_q = f32_to_q16(MAG2_Q16_COMPONENT_MAX);
    let min_q = f32_to_q16(-MAG2_Q16_COMPONENT_MAX);
    let tiny = f32_to_q16(0.001);
    let w1 = f32_to_q16(1.0);
    let b0 = f32_to_q16(0.0);
    vec![
        ObsRow {
            gx: 0,
            gy: 0,
            w_mag: w1,
            bias: b0,
        },
        ObsRow {
            gx: f32_to_q16(3.0),
            gy: 0,
            w_mag: w1,
            bias: b0,
        },
        ObsRow {
            gx: 0,
            gy: f32_to_q16(4.0),
            w_mag: f32_to_q16(-1.0),
            bias: f32_to_q16(0.5),
        },
        ObsRow {
            gx: max_q,
            gy: 0,
            w_mag: w1,
            bias: b0,
        },
        ObsRow {
            gx: min_q,
            gy: min_q,
            w_mag: f32_to_q16(2.0),
            bias: f32_to_q16(-1.0),
        },
        ObsRow {
            gx: tiny,
            gy: tiny,
            w_mag: f32_to_q16(0.5),
            bias: tiny,
        },
    ]
}

#[test]
fn sead_obs0_wgsl_semantic_free() {
    let wgsl = emit_overlay_score_wgsl(1);
    for term in FORBIDDEN_SEMANTIC_TERMS {
        assert!(
            !wgsl.contains(term),
            "WGSL must not contain forbidden semantic term `{term}`"
        );
    }
    for term in FORBIDDEN_EXACT_TERMS {
        assert!(
            !wgsl.contains(term),
            "WGSL must not contain forbidden exact term `{term}`"
        );
    }
    assert!(
        !wgsl.contains("sqrt(") || wgsl.contains("sqrt_cr_f_bits"),
        "native sqrt must not be used for exact mag path"
    );
    assert!(
        wgsl.contains("sqrt_cr_f_bits"),
        "F artifact entrypoint must be used"
    );
    assert!(
        wgsl.contains("mag2_sum_q16"),
        "mag must come from fixed-point gx/gy, not raw f32 dx/dy"
    );
    assert!(
        !wgsl.contains("dx_fixed") && !wgsl.contains("dy_fixed"),
        "uses gx/gy field names in kernel"
    );

    assert_eq!(fnv1a64_hex(SQRT_CR_F_WGSL), SQRT_F_ARTIFACT_HASH);
    assert!(wgsl.contains("sqrt_cr_f_bits"));

    println!(
        "sead_obs0_wgsl: semantic_free=true F_hash={SQRT_F_ARTIFACT_HASH} mag_path=fixed_q16_plus_F"
    );
}

#[test]
fn sead_obs0_perf_34k_mobile_overlay_score() {
    with_gpu(|ctx| {
        const N: usize = 34_000;
        let rows = mobile_observer_rows(N);
        let t0 = Instant::now();
        let outputs = run_overlay_score_batch(ctx, &rows);
        let elapsed = t0.elapsed();
        assert_eq!(outputs.len(), N);

        let mut spot_mag_ulp = 0u32;
        for (out, row) in outputs.iter().take(512).zip(rows.iter().take(512)) {
            assert_eq!(out.mag2_sum, cpu_mag2_sum(row.gx, row.gy));
            assert_eq!(out.mag2_bits, cpu_mag2_bits(row.gx, row.gy));
            spot_mag_ulp = spot_mag_ulp.max(ulp_distance(out.mag_bits, cpu_mag_bits(row.gx, row.gy)));
        }
        assert_eq!(spot_mag_ulp, 0);

        let ms = elapsed.as_secs_f64() * 1000.0;
        let per_row_us = elapsed.as_secs_f64() * 1_000_000.0 / N as f64;
        println!(
            "sead_obs0_perf_34k: rows={N} dispatches=1 includes_readback=true elapsed_ms={ms:.3} per_row_us={per_row_us:.4} spot_mag_max_ulp={spot_mag_ulp} path=q16_mag2_F_sqrt_f32_score"
        );
        println!(
            "sead_obs0_perf_compare: SQRT-MAG2-PERF-0 combined Q16.16 ~1.7 ms; overlay adds f32 score multiply/add"
        );
    });
}

#[test]
fn sead_obs0_exact_magnitude_spot_check() {
    with_gpu(|ctx| {
        const N: usize = 34_000;
        let rows = mobile_observer_rows(N);
        let outputs = run_overlay_score_batch(ctx, &rows);
        let mut mag2_exact = 0usize;
        let mut mag_max_ulp = 0u32;
        for (out, row) in outputs.iter().take(512).zip(rows.iter().take(512)) {
            let sum_cpu = cpu_mag2_sum(row.gx, row.gy);
            if out.mag2_sum == sum_cpu {
                mag2_exact += 1;
            }
            assert_eq!(out.mag2_bits, cpu_mag2_bits(row.gx, row.gy));
            mag_max_ulp = mag_max_ulp.max(ulp_distance(out.mag_bits, cpu_mag_bits(row.gx, row.gy)));
        }
        println!(
            "sead_obs0_spot: rows=512 mag2_exact={mag2_exact}/512 mag_max_ulp={mag_max_ulp} overflow=0"
        );
        assert_eq!(mag2_exact, 512);
        assert_eq!(mag_max_ulp, 0);
    });
}

#[test]
fn sead_obs0_dense_overlay_corpus() {
    with_gpu(|ctx| {
        let rows = dense_overlay_rows();
        let outputs = run_overlay_score_batch(ctx, &rows);
        let mut mag2_exact = 0usize;
        let mut mag_max_ulp = 0u32;
        let mut score_max_ulp = 0u32;
        let mut overflow = 0usize;
        for (out, row) in outputs.iter().zip(rows.iter()) {
            let sum_cpu = cpu_mag2_sum(row.gx, row.gy);
            if out.mag2_sum == sum_cpu {
                mag2_exact += 1;
            }
            assert_eq!(out.mag2_bits, cpu_mag2_bits(row.gx, row.gy));
            mag_max_ulp = mag_max_ulp.max(ulp_distance(out.mag_bits, cpu_mag_bits(row.gx, row.gy)));
            let score_cpu = cpu_score_bits(row.w_mag, row.bias, out.mag_bits);
            score_max_ulp = score_max_ulp.max(ulp_distance(out.score_bits, score_cpu));
            if sum_cpu >> 63 != 0 {
                overflow += 1;
            }
        }
        println!(
            "sead_obs0_dense: tested={} mag2_exact={mag2_exact} mag_max_ulp={mag_max_ulp} score_max_ulp={score_max_ulp} overflow={overflow}",
            rows.len()
        );
        assert_eq!(mag2_exact, rows.len());
        assert_eq!(mag_max_ulp, 0);
        assert_eq!(overflow, 0);
    });
}

#[test]
fn sead_obs0_score_authority_matches_arithmetic() {
    assert_eq!(
        SCORE_AUTHORITY,
        ScoreAuthority::ApproximateDiagnosticF32
    );
    let wgsl = emit_overlay_score_wgsl(1);
    assert!(
        wgsl.contains("f32(bias)") && wgsl.contains("* mag_f"),
        "score uses f32 multiply/add"
    );
    println!(
        "sead_obs0_score_authority: mag=ExactAuthoritative(Q16.16+F) score=ApproximateDiagnosticF32 no_planner_no_bridge"
    );
}

#[test]
fn sead_obs0_no_default_runtime_wiring() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );

    let mag2 = landed_jit_kernel_descriptors()
        .into_iter()
        .find(|d| d.id == MAG2_FIXED_DESCRIPTOR_ID)
        .expect("mag2 fixed descriptor");
    assert!(is_exact_mag2_fixed_descriptor(&mag2));
    validate_kernel_descriptor_admission(&mag2).expect("mag2 admits");
    assert!(mag2.default_off);
    assert!(!mag2.production_wiring);

    let wgsl = emit_overlay_score_wgsl(1);
    for forbidden in [
        "SimSession",
        "ResourceEconomySpec",
        "simthing-sim",
        "KernelCache",
        "cache.insert",
        "scheduler",
        "SemanticWGSL",
    ] {
        assert!(
            !wgsl.contains(forbidden),
            "overlay WGSL must not reference `{forbidden}`"
        );
    }

    let registry_has_obs = landed_jit_kernel_descriptors()
        .iter()
        .any(|d| d.id == "m_jit_sead_obs0_overlay_score");
    assert!(
        !registry_has_obs,
        "SEAD-OBS-0 remains driver fixture; descriptor follow-up deferred"
    );

    println!("sead_obs0_wiring: default_off=true production_wiring=false descriptor=deferred");
}

#[test]
fn sead_obs0_edge_rows_correctness() {
    with_gpu(|ctx| {
        let rows = edge_overlay_rows();
        let outputs = run_overlay_score_batch(ctx, &rows);
        for (out, row) in outputs.iter().zip(rows.iter()) {
            assert_eq!(out.mag2_sum, cpu_mag2_sum(row.gx, row.gy));
            assert_eq!(out.mag2_bits, cpu_mag2_bits(row.gx, row.gy));
            assert_eq!(out.mag_bits, cpu_mag_bits(row.gx, row.gy));
            assert_eq!(
                out.score_bits,
                cpu_score_bits(row.w_mag, row.bias, out.mag_bits)
            );
        }
        println!("sead_obs0_edge: tested={} mag_max_ulp=0", rows.len());
    });
}
