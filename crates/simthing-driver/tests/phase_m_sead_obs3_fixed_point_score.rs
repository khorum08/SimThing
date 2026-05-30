//! SEAD-OBS-3 — Fixed-point aggregate score for multi-layer observer overlay (Tier-2, test-only).
//!
//! Four Q16.16 gx/gy layers → exact mag per layer → Q16.16 weighted sum score (exact).
//! OBS-2 f32 score descriptor remains ApproximateDiagnostic. No production wiring.

use std::sync::Mutex;
use std::time::Instant;

use simthing_gpu::GpuContext;
use simthing_spec::{
    fnv1a64_hex, is_sead_obs2_multilayer_overlay_score_descriptor,
    is_sead_obs3_multilayer_fixed_score_descriptor, landed_jit_kernel_descriptors,
    validate_kernel_descriptor_admission, MAG2_Q16_SCALE, MappingExecutionProfile,
    SEAD_OBS2_DESCRIPTOR_ID, SEAD_OBS3_DESCRIPTOR_ID, SEAD_OBS3_LAYER_COUNT, SQRT_F_ARTIFACT_HASH,
};

use simthing_spec::OutputAuthority;

static GPU_MUTEX: Mutex<()> = Mutex::new(());
const SQRT_CR_F_WGSL: &str = include_str!("wgsl/sqrt_cr_f_candidate.wgsl");

const LAYER_COUNT: usize = SEAD_OBS3_LAYER_COUNT as usize;
const FIELDS_PER_LAYER: u32 = 3;
const ROW_STRIDE: u32 = LAYER_COUNT as u32 * FIELDS_PER_LAYER + 1 + LAYER_COUNT as u32 + 2;
const MAG_BITS_BASE: u32 = LAYER_COUNT as u32 * FIELDS_PER_LAYER + 1;
const SCORE_FIXED_OFF: u32 = MAG_BITS_BASE + LAYER_COUNT as u32;
const FLAGS_OFF: u32 = SCORE_FIXED_OFF + 1;
const Q16_SCALE_F: f32 = MAG2_Q16_SCALE as f32;

const FORBIDDEN_SEMANTIC_TERMS: &[&str] = &[
    "faction", "ownership", "owner", "AI", "threat", "scarcity", "opportunity", "labor", "price",
    "logistics", "routing", "need", "demand", "supply", "personality", "drone", "SEAD", "economy",
    "planner", "resource", "map", "threat",
];

const FORBIDDEN_EXACT_TERMS: &[&str] = &["f64", "F64RoundDown", "SHADER_F64", "sqrt_cr_c"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LayerInput {
    gx: i32,
    gy: i32,
    w: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MultiLayerRow {
    layers: [LayerInput; LAYER_COUNT],
    bias: i32,
}

#[derive(Debug, Clone)]
struct MultiLayerOutput {
    mag_bits: [u32; LAYER_COUNT],
    score_fixed: i32,
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

fn cpu_mag2_bits(gx: i32, gy: i32) -> u32 {
    let dx = i64::from(gx);
    let dy = i64::from(gy);
    let sum = (dx * dx + dy * dy) as u64;
    let lo = sum as u32;
    let hi = (sum >> 32) as u32;
    (hi as f32 + lo as f32 / 4294967296.0).to_bits()
}

fn cpu_mag_bits(gx: i32, gy: i32) -> u32 {
    f32::from_bits(cpu_mag2_bits(gx, gy)).sqrt().to_bits()
}

fn mag_bits_to_q16_fixed(mag_bits: u32) -> i32 {
    (f32::from_bits(mag_bits) * Q16_SCALE_F).round_ties_even() as i32
}

fn q16_mul(a: i32, b: i32) -> i32 {
    (i64::from(a) * i64::from(b) / i64::from(MAG2_Q16_SCALE)) as i32
}

fn cpu_score_fixed(row: &MultiLayerRow) -> i32 {
    let mut score = row.bias;
    for layer in &row.layers {
        let mag_fixed = mag_bits_to_q16_fixed(cpu_mag_bits(layer.gx, layer.gy));
        score = score.wrapping_add(q16_mul(layer.w, mag_fixed));
    }
    score
}

fn score_accumulation_overflow(row: &MultiLayerRow) -> bool {
    let mut acc = i64::from(row.bias);
    for layer in &row.layers {
        let mag_fixed = i64::from(mag_bits_to_q16_fixed(cpu_mag_bits(layer.gx, layer.gy)));
        let term = (i64::from(layer.w) * mag_fixed) / i64::from(MAG2_Q16_SCALE);
        acc = acc.saturating_add(term);
        if acc > i64::from(i32::MAX) || acc < i64::from(i32::MIN) {
            return true;
        }
    }
    false
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

fn mag_bits_to_q16_fixed(mag_bits: u32) -> i32 {
    return i32(round(bitcast<f32>(mag_bits) * 65536.0));
}

fn u64_shr16(v: vec2<u32>) -> u32 {
    return (v.y << 16u) | (v.x >> 16u);
}

fn q16_mul(a: i32, b: i32) -> i32 {
    let neg = (a < 0) != (b < 0);
    let prod = mul_u32_wide(abs_fixed(a), abs_fixed(b));
    let mag = u64_shr16(prod);
    if (neg) {
        return bitcast<i32>(0u - mag);
    }
    return i32(mag);
}
"#
}

fn emit_multilayer_fixed_score_wgsl(batch_count: u32) -> String {
    format!(
        r#"{f}
{limb}
@group(0) @binding(0) var<storage, read_write> data: array<u32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {{
    let i = gid.x;
    if (i >= {batch_count}u) {{ return; }}
    let base = i * {stride}u;
    let bias = bitcast<i32>(data[base + {bias_off}u]);
    var score_fixed = bias;
    for (var layer = 0u; layer < {layer_count}u; layer = layer + 1u) {{
        let in_off = base + layer * {fields_per_layer}u;
        let gx = bitcast<i32>(data[in_off]);
        let gy = bitcast<i32>(data[in_off + 1u]);
        let w = bitcast<i32>(data[in_off + 2u]);
        let sum = mag2_sum_q16(gx, gy);
        let mag2_bits = mag2_u64_q16_to_f32_bits(sum);
        let mag_bits = sqrt_cr_f_bits(mag2_bits);
        data[base + {mag_base}u + layer] = mag_bits;
        let mag_fixed = mag_bits_to_q16_fixed(mag_bits);
        score_fixed = score_fixed + q16_mul(w, mag_fixed);
    }}
    data[base + {score_off}u] = bitcast<u32>(score_fixed);
    data[base + {flags_off}u] = 0u;
}}
"#,
        f = SQRT_CR_F_WGSL,
        limb = limb_arith_wgsl(),
        batch_count = batch_count,
        stride = ROW_STRIDE,
        layer_count = LAYER_COUNT,
        fields_per_layer = FIELDS_PER_LAYER,
        bias_off = LAYER_COUNT as u32 * FIELDS_PER_LAYER,
        mag_base = MAG_BITS_BASE,
        score_off = SCORE_FIXED_OFF,
        flags_off = FLAGS_OFF,
    )
}

fn init_buffer(rows: &[MultiLayerRow]) -> Vec<u32> {
    let mut data = vec![0u32; rows.len() * ROW_STRIDE as usize];
    for (i, row) in rows.iter().enumerate() {
        let base = i * ROW_STRIDE as usize;
        for (layer, inp) in row.layers.iter().enumerate() {
            let off = base + layer * FIELDS_PER_LAYER as usize;
            data[off] = inp.gx as u32;
            data[off + 1] = inp.gy as u32;
            data[off + 2] = inp.w as u32;
        }
        data[base + LAYER_COUNT * FIELDS_PER_LAYER as usize] = row.bias as u32;
    }
    data
}

struct WarmRunOutcome {
    outputs: Vec<MultiLayerOutput>,
    elapsed: std::time::Duration,
}

fn run_multilayer_batch_repeated(
    ctx: &GpuContext,
    rows: &[MultiLayerRow],
    repeat_dispatches: u32,
    do_readback: bool,
) -> WarmRunOutcome {
    use wgpu::util::DeviceExt;
    let n = rows.len() as u32;
    let wgsl = emit_multilayer_fixed_score_wgsl(n);
    let data = init_buffer(rows);
    let device = &ctx.device;
    let queue = &ctx.queue;
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("sead_obs3_multilayer_fixed"),
        source: wgpu::ShaderSource::Wgsl(wgsl.into()),
    });
    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("sead_obs3_bgl"),
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
        label: Some("sead_obs3_pipeline"),
        layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("sead_obs3_pl"),
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
        label: Some("sead_obs3_values"),
        contents: bytemuck::cast_slice(&data),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("sead_obs3_bg"),
        layout: &bgl,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage.as_entire_binding(),
        }],
    });
    let t0 = Instant::now();
    for _ in 0..repeat_dispatches {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("sead_obs3_enc"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("sead_obs3_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &bg, &[]);
            pass.dispatch_workgroups(n.div_ceil(64), 1, 1);
        }
        queue.submit(Some(encoder.finish()));
    }
    let elapsed = t0.elapsed();

    if !do_readback {
        return WarmRunOutcome {
            outputs: Vec::new(),
            elapsed,
        };
    }

    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("sead_obs3_readback"),
        size: bytes,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("sead_obs3_readback_enc"),
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

    let outputs = rows
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let base = i * ROW_STRIDE as usize;
            let mut mag_bits = [0u32; LAYER_COUNT];
            for layer in 0..LAYER_COUNT {
                mag_bits[layer] = out[base + MAG_BITS_BASE as usize + layer];
            }
            MultiLayerOutput {
                mag_bits,
                score_fixed: bytemuck::cast(out[base + SCORE_FIXED_OFF as usize]),
                flags: out[base + FLAGS_OFF as usize],
            }
        })
        .collect();

    WarmRunOutcome { outputs, elapsed }
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

fn dense_multilayer_rows() -> Vec<MultiLayerRow> {
    let grads = gradient_samples();
    let weights = weight_bias_samples();
    let mut out = Vec::new();
    for (pair_idx, _) in grads.iter().enumerate() {
        for _ in grads.iter() {
            for &bias in weights.iter() {
                let layers = std::array::from_fn(|layer| {
                    let gi = (pair_idx + layer * 5) % grads.len();
                    let gj = (pair_idx + layer * 3 + 1) % grads.len();
                    let wi = (pair_idx + layer) % weights.len();
                    LayerInput {
                        gx: f32_to_q16(grads[gi]),
                        gy: f32_to_q16(grads[gj]),
                        w: f32_to_q16(weights[wi]),
                    }
                });
                out.push(MultiLayerRow {
                    layers,
                    bias: f32_to_q16(bias),
                });
            }
        }
    }
    out
}

fn mobile_multilayer_rows(count: usize) -> Vec<MultiLayerRow> {
    let grads = gradient_samples();
    let weights = weight_bias_samples();
    let mut out = Vec::with_capacity(count);
    let mut state = 0x5345_4144u32;
    for idx in 0..count {
        let layers = std::array::from_fn(|layer| {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let gx = grads[(state as usize) % grads.len()];
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let gy = grads[(state as usize) % grads.len()];
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let w = weights[(state as usize) % weights.len()];
            LayerInput {
                gx: f32_to_q16(gx),
                gy: f32_to_q16(gy),
                w: f32_to_q16(w),
            }
        });
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let bias = weights[(state as usize + idx) % weights.len()];
        out.push(MultiLayerRow {
            layers,
            bias: f32_to_q16(bias),
        });
    }
    out
}

fn edge_multilayer_rows() -> Vec<(MultiLayerRow, &'static str)> {
    let zero_layer = |w: f32| LayerInput {
        gx: 0,
        gy: 0,
        w: f32_to_q16(w),
    };
    let axis = |gx: f32, gy: f32, w: f32| LayerInput {
        gx: f32_to_q16(gx),
        gy: f32_to_q16(gy),
        w: f32_to_q16(w),
    };
    vec![
        (
            MultiLayerRow {
                layers: [zero_layer(1.0); LAYER_COUNT],
                bias: 0,
            },
            "zero_gradients",
        ),
        (
            MultiLayerRow {
                layers: [axis(1.0, 0.0, 0.0); LAYER_COUNT],
                bias: f32_to_q16(1.0),
            },
            "zero_weights",
        ),
        (
            MultiLayerRow {
                layers: [
                    axis(1.0, 0.0, 2.0),
                    axis(0.0, 1.0, -2.0),
                    axis(-1.0, 0.0, 0.5),
                    axis(0.0, -1.0, -0.5),
                ],
                bias: f32_to_q16(-1.0),
            },
            "pos_neg_weights",
        ),
        (
            MultiLayerRow {
                layers: [axis(16.0, 0.0, 1.0), zero_layer(0.0), zero_layer(0.0), zero_layer(0.0)],
                bias: f32_to_q16(2.0),
            },
            "single_active_layer",
        ),
        (
            MultiLayerRow {
                layers: [
                    axis(16.0, 0.0, 2.0),
                    axis(-16.0, 0.0, 2.0),
                    axis(0.0, 16.0, -1.0),
                    axis(0.0, -16.0, -1.0),
                ],
                bias: f32_to_q16(2.0),
            },
            "all_four_active_max_grad",
        ),
        (
            MultiLayerRow {
                layers: [
                    axis(8.0, 8.0, 2.0),
                    axis(-8.0, 8.0, 2.0),
                    axis(8.0, -8.0, 2.0),
                    axis(-8.0, -8.0, 2.0),
                ],
                bias: f32_to_q16(4.0),
            },
            "near_overflow_boundary",
        ),
    ]
}

#[test]
fn sead_obs3_fixed_score_edge_rows() {
    with_gpu(|ctx| {
        let cases = edge_multilayer_rows();
        let rows: Vec<MultiLayerRow> = cases.iter().map(|(row, _)| row.clone()).collect();
        let outputs = run_multilayer_batch_repeated(ctx, &rows, 1, true).outputs;
        let mut mag2_exact = 0usize;
        let mut mag_max_ulp = 0u32;
        let mut score_exact = 0usize;
        let mut overflow = 0usize;
        let mut worst: Option<(String, i32, i32)> = None;

        for ((out, row), (_, label)) in outputs.iter().zip(rows.iter()).zip(cases.iter()) {
            for (layer, inp) in row.layers.iter().enumerate() {
                if out.mag_bits[layer] == cpu_mag_bits(inp.gx, inp.gy) {
                    mag2_exact += 1;
                }
                mag_max_ulp = mag_max_ulp.max(ulp_distance(
                    out.mag_bits[layer],
                    cpu_mag_bits(inp.gx, inp.gy),
                ));
            }
            let expected = cpu_score_fixed(row);
            if out.score_fixed == expected {
                score_exact += 1;
            } else if worst.is_none() {
                worst = Some((label.to_string(), out.score_fixed, expected));
            }
            if score_accumulation_overflow(row) {
                overflow += 1;
            }
        }

        println!(
            "sead_obs3_edge: cases={} mag2_exact={mag2_exact} mag_max_ulp={mag_max_ulp} score_exact={score_exact}/{} overflow={overflow} worst={worst:?}",
            cases.len(),
            cases.len()
        );
        assert_eq!(mag2_exact, cases.len() * LAYER_COUNT);
        assert_eq!(mag_max_ulp, 0);
        assert_eq!(score_exact, cases.len());
        assert_eq!(overflow, 0);
    });
}

#[test]
fn sead_obs3_fixed_score_dense_corpus() {
    with_gpu(|ctx| {
        let rows = dense_multilayer_rows();
        let outputs = run_multilayer_batch_repeated(ctx, &rows, 1, true).outputs;
        let mut mag_exact = 0usize;
        let mut mag_max_ulp = 0u32;
        let mut score_exact = 0usize;
        let mut overflow = 0usize;
        let layer_mag_slots = rows.len() * LAYER_COUNT;

        for (out, row) in outputs.iter().zip(rows.iter()) {
            for (layer, inp) in row.layers.iter().enumerate() {
                if out.mag_bits[layer] == cpu_mag_bits(inp.gx, inp.gy) {
                    mag_exact += 1;
                }
                mag_max_ulp = mag_max_ulp.max(ulp_distance(
                    out.mag_bits[layer],
                    cpu_mag_bits(inp.gx, inp.gy),
                ));
            }
            if out.score_fixed == cpu_score_fixed(row) {
                score_exact += 1;
            }
            if score_accumulation_overflow(row) {
                overflow += 1;
            }
        }

        println!(
            "sead_obs3_dense: rows={} layer_mag_slots={layer_mag_slots} mag_exact={mag_exact} mag_max_ulp={mag_max_ulp} score_exact={score_exact} overflow={overflow} score_authority=ExactQ16WeightedSum",
            rows.len()
        );
        assert_eq!(mag_exact, layer_mag_slots);
        assert_eq!(mag_max_ulp, 0);
        assert_eq!(score_exact, rows.len());
        assert_eq!(overflow, 0);
    });
}

#[test]
fn sead_obs3_perf_34k_fixed_score() {
    with_gpu(|ctx| {
        const N: usize = 34_000;
        let rows = mobile_multilayer_rows(N);
        let t0 = Instant::now();
        let outcome = run_multilayer_batch_repeated(ctx, &rows, 1, true);
        let elapsed = t0.elapsed();
        assert_eq!(outcome.outputs.len(), N);

        let mut spot_mag_ulp = 0u32;
        let mut spot_score_exact = 0usize;
        let mut overflow = 0usize;
        for (out, row) in outcome.outputs.iter().take(512).zip(rows.iter().take(512)) {
            for (layer, inp) in row.layers.iter().enumerate() {
                spot_mag_ulp = spot_mag_ulp.max(ulp_distance(
                    out.mag_bits[layer],
                    cpu_mag_bits(inp.gx, inp.gy),
                ));
            }
            if out.score_fixed == cpu_score_fixed(row) {
                spot_score_exact += 1;
            }
            if score_accumulation_overflow(row) {
                overflow += 1;
            }
        }
        assert_eq!(spot_mag_ulp, 0);
        assert_eq!(spot_score_exact, 512);

        let ms = elapsed.as_secs_f64() * 1000.0;
        let per_row_us = elapsed.as_secs_f64() * 1_000_000.0 / N as f64;
        println!(
            "sead_obs3_perf_34k: rows={N} layers={LAYER_COUNT} dispatches=1 includes_readback=true elapsed_ms={ms:.3} per_row_us={per_row_us:.4} spot_mag_max_ulp={spot_mag_ulp} spot_score_exact={spot_score_exact}/512 overflow={overflow} score_authority=ExactQ16WeightedSum"
        );
    });
}

#[test]
fn sead_obs3_perf_34k_warm_repeated_dispatch() {
    with_gpu(|ctx| {
        const N: usize = 34_000;
        const REPEAT: u32 = 32;
        let rows = mobile_multilayer_rows(N);
        let _ = run_multilayer_batch_repeated(ctx, &rows, 1, false);
        let outcome = run_multilayer_batch_repeated(ctx, &rows, REPEAT, true);
        assert_eq!(outcome.outputs.len(), N);

        let mut spot_mag_ulp = 0u32;
        for (out, row) in outcome.outputs.iter().take(512).zip(rows.iter().take(512)) {
            for (layer, inp) in row.layers.iter().enumerate() {
                spot_mag_ulp = spot_mag_ulp.max(ulp_distance(
                    out.mag_bits[layer],
                    cpu_mag_bits(inp.gx, inp.gy),
                ));
            }
        }
        assert_eq!(spot_mag_ulp, 0);

        let total_ms = outcome.elapsed.as_secs_f64() * 1000.0;
        let per_dispatch_ms = total_ms / REPEAT as f64;
        let per_row_us =
            outcome.elapsed.as_secs_f64() * 1_000_000.0 / (N as f64 * REPEAT as f64);
        let per_layer_mag_us = per_row_us / LAYER_COUNT as f64;
        println!(
            "sead_obs3_warm_34k: rows={N} layers={LAYER_COUNT} dispatches={REPEAT} includes_readback=true total_ms={total_ms:.3} per_dispatch_ms={per_dispatch_ms:.3} per_row_us={per_row_us:.4} per_layer_mag_us={per_layer_mag_us:.4} spot_mag_max_ulp={spot_mag_ulp} score_authority=ExactQ16WeightedSum"
        );
        println!("sead_obs3_warm_compare: SEAD-OBS-2 warm ~0.238 ms/dispatch f32 score");
    });
}

#[test]
fn sead_obs3_score_authority_fixed_point() {
    let wgsl = emit_multilayer_fixed_score_wgsl(1);
    assert!(wgsl.contains("score_fixed") || wgsl.contains("mag_bits_to_q16_fixed"));
    assert!(wgsl.contains("q16_mul"));
    assert!(!wgsl.contains("var score = f32(bias)"));

    let obs3 = landed_jit_kernel_descriptors()
        .into_iter()
        .find(|d| d.id == SEAD_OBS3_DESCRIPTOR_ID)
        .expect("obs3 descriptor");
    assert!(is_sead_obs3_multilayer_fixed_score_descriptor(&obs3));
    validate_kernel_descriptor_admission(&obs3).expect("obs3 admits");

    let score_fixed = obs3
        .writes
        .iter()
        .find(|w| w.name == "score_fixed")
        .expect("score_fixed");
    assert_eq!(score_fixed.authority, OutputAuthority::ExactAuthoritative);

    let obs2 = landed_jit_kernel_descriptors()
        .into_iter()
        .find(|d| d.id == SEAD_OBS2_DESCRIPTOR_ID)
        .expect("obs2 descriptor");
    assert!(is_sead_obs2_multilayer_overlay_score_descriptor(&obs2));
    let score_bits = obs2
        .writes
        .iter()
        .find(|w| w.name == "score_bits")
        .expect("score_bits");
    assert_eq!(score_bits.authority, OutputAuthority::ApproximateDiagnostic);
    println!("sead_obs3_score_authority: score_fixed=ExactQ16WeightedSum obs2_score_bits=ApproximateDiagnosticF32");
}

#[test]
fn sead_obs3_no_default_runtime_wiring() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    let obs3 = landed_jit_kernel_descriptors()
        .into_iter()
        .find(|d| d.id == SEAD_OBS3_DESCRIPTOR_ID)
        .expect("obs3 descriptor");
    assert!(obs3.default_off);
    assert!(!obs3.production_wiring);
    validate_kernel_descriptor_admission(&obs3).expect("obs3 admits");

    let wgsl = emit_multilayer_fixed_score_wgsl(1);
    for term in FORBIDDEN_SEMANTIC_TERMS {
        assert!(
            !wgsl.contains(term),
            "WGSL must not contain forbidden semantic term `{term}`"
        );
    }
    for term in FORBIDDEN_EXACT_TERMS {
        assert!(!wgsl.contains(term));
    }
    assert!(wgsl.contains("sqrt_cr_f_bits"));
    assert_eq!(fnv1a64_hex(SQRT_CR_F_WGSL), SQRT_F_ARTIFACT_HASH);

    for forbidden in [
        "SimSession",
        "ResourceEconomySpec",
        "simthing-sim",
        "KernelCache",
        "scheduler",
    ] {
        assert!(!wgsl.contains(forbidden));
    }
    println!("sead_obs3_wiring: descriptor=landed no_scheduler_no_bridge semantic_free=true");
}
