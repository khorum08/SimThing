//! FIELD_POLICY-OBS-2 — Multi-layer GPU-resident observer overlay score probe (Tier-2, test-only).
//!
//! Four Q16.16 gx/gy layers → exact mag per layer → f32 weighted sum score (diagnostic).
//! No production wiring, no semantic WGSL.

use std::sync::Mutex;
use std::time::Instant;

use simthing_gpu::GpuContext;
use simthing_spec::{
    fnv1a64_hex, is_field_policy_obs2_multilayer_overlay_score_descriptor,
    landed_jit_kernel_descriptors, validate_kernel_descriptor_admission, MappingExecutionProfile,
    FIELD_POLICY_OBS2_DESCRIPTOR_ID, FIELD_POLICY_OBS2_LAYER_COUNT, MAG2_Q16_SCALE,
    SQRT_F_ARTIFACT_HASH,
};

use simthing_spec::OutputAuthority;

static GPU_MUTEX: Mutex<()> = Mutex::new(());
const SQRT_CR_F_WGSL: &str = include_str!("wgsl/sqrt_cr_f_candidate.wgsl");

const LAYER_COUNT: usize = FIELD_POLICY_OBS2_LAYER_COUNT as usize;
const FIELDS_PER_LAYER: u32 = 3;
/// 4×(gx,gy,w) + bias + 4×mag_bits + score + flags
const ROW_STRIDE: u32 = LAYER_COUNT as u32 * FIELDS_PER_LAYER + 1 + LAYER_COUNT as u32 + 2;
const MAG_BITS_BASE: u32 = LAYER_COUNT as u32 * FIELDS_PER_LAYER + 1;
const SCORE_BITS_OFF: u32 = MAG_BITS_BASE + LAYER_COUNT as u32;
const FLAGS_OFF: u32 = SCORE_BITS_OFF + 1;
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
    "FIELD_POLICY",
    "economy",
    "planner",
    "resource",
    "map",
    "faction",
    "threat",
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

fn cpu_score_bits(row: &MultiLayerRow) -> u32 {
    let mut score = row.bias as f32 / Q16_SCALE_F;
    for layer in &row.layers {
        let mag = f32::from_bits(cpu_mag_bits(layer.gx, layer.gy));
        score += layer.w as f32 / Q16_SCALE_F * mag;
    }
    score.to_bits()
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

fn emit_multilayer_wgsl(batch_count: u32) -> String {
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
    var score = f32(bias) / 65536.0;
    for (var layer = 0u; layer < {layer_count}u; layer = layer + 1u) {{
        let in_off = base + layer * {fields_per_layer}u;
        let gx = bitcast<i32>(data[in_off]);
        let gy = bitcast<i32>(data[in_off + 1u]);
        let w = bitcast<i32>(data[in_off + 2u]);
        let sum = mag2_sum_q16(gx, gy);
        let mag2_bits = mag2_u64_q16_to_f32_bits(sum);
        let mag_bits = sqrt_cr_f_bits(mag2_bits);
        data[base + {mag_base}u + layer] = mag_bits;
        score = score + f32(w) / 65536.0 * bitcast<f32>(mag_bits);
    }}
    data[base + {score_off}u] = bitcast<u32>(score);
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
        score_off = SCORE_BITS_OFF,
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
    dispatches: u32,
    includes_readback: bool,
}

fn run_multilayer_batch_repeated(
    ctx: &GpuContext,
    rows: &[MultiLayerRow],
    repeat_dispatches: u32,
    do_readback: bool,
) -> WarmRunOutcome {
    use wgpu::util::DeviceExt;
    let n = rows.len() as u32;
    let wgsl = emit_multilayer_wgsl(n);
    let data = init_buffer(rows);
    let device = &ctx.device;
    let queue = &ctx.queue;
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("field_policy_obs2_multilayer"),
        source: wgpu::ShaderSource::Wgsl(wgsl.into()),
    });
    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("field_policy_obs2_bgl"),
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
        label: Some("field_policy_obs2_pipeline"),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("field_policy_obs2_pl"),
                bind_group_layouts: &[&bgl],
                push_constant_ranges: &[],
            }),
        ),
        module: &module,
        entry_point: "main",
        compilation_options: Default::default(),
        cache: None,
    });
    let bytes = std::mem::size_of_val(data.as_slice()) as u64;
    let storage = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("field_policy_obs2_values"),
        contents: bytemuck::cast_slice(&data),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("field_policy_obs2_bg"),
        layout: &bgl,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage.as_entire_binding(),
        }],
    });
    let t0 = Instant::now();
    for _ in 0..repeat_dispatches {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("field_policy_obs2_enc"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("field_policy_obs2_pass"),
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
            dispatches: repeat_dispatches,
            includes_readback: false,
        };
    }

    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("field_policy_obs2_readback"),
        size: bytes,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("field_policy_obs2_readback_enc"),
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
                score_bits: out[base + SCORE_BITS_OFF as usize],
                flags: out[base + FLAGS_OFF as usize],
            }
        })
        .collect();

    WarmRunOutcome {
        outputs,
        elapsed,
        dispatches: repeat_dispatches,
        includes_readback: true,
    }
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

#[test]
fn field_policy_obs2_wgsl_semantic_free() {
    let wgsl = emit_multilayer_wgsl(1);
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
    assert!(wgsl.contains("sqrt_cr_f_bits"));
    assert!(wgsl.contains("mag2_sum_q16"));
    assert_eq!(fnv1a64_hex(SQRT_CR_F_WGSL), SQRT_F_ARTIFACT_HASH);
    println!(
        "field_policy_obs2_wgsl: semantic_free=true layers={LAYER_COUNT} F_hash={SQRT_F_ARTIFACT_HASH}"
    );
}

#[test]
fn field_policy_obs2_dense_multilayer_correctness() {
    with_gpu(|ctx| {
        let rows = dense_multilayer_rows();
        let outputs = run_multilayer_batch_repeated(ctx, &rows, 1, true).outputs;
        let mut mag2_exact = 0usize;
        let mut mag_max_ulp = 0u32;
        let mut score_max_ulp = 0u32;
        let mut overflow = 0usize;
        let layer_mag_slots = rows.len() * LAYER_COUNT;
        for (out, row) in outputs.iter().zip(rows.iter()) {
            for (layer, inp) in row.layers.iter().enumerate() {
                let sum = cpu_mag2_bits(inp.gx, inp.gy);
                if out.mag_bits[layer] == cpu_mag_bits(inp.gx, inp.gy) {
                    mag2_exact += 1;
                }
                let cpu_mag2 = cpu_mag2_bits(inp.gx, inp.gy);
                let _ = cpu_mag2;
                mag_max_ulp = mag_max_ulp.max(ulp_distance(
                    out.mag_bits[layer],
                    cpu_mag_bits(inp.gx, inp.gy),
                ));
                let dx = i64::from(inp.gx);
                let dy = i64::from(inp.gy);
                if (dx * dx + dy * dy) as u64 >> 63 != 0 {
                    overflow += 1;
                }
            }
            score_max_ulp = score_max_ulp.max(ulp_distance(out.score_bits, cpu_score_bits(row)));
        }
        println!(
            "field_policy_obs2_dense: rows={} layer_mag_slots={layer_mag_slots} mag2_exact={mag2_exact} mag_max_ulp={mag_max_ulp} score_max_ulp={score_max_ulp} overflow={overflow} score_authority=ApproximateDiagnosticF32",
            rows.len()
        );
        assert_eq!(mag2_exact, layer_mag_slots);
        assert_eq!(mag_max_ulp, 0);
        assert_eq!(overflow, 0);
    });
}

#[test]
fn field_policy_obs2_score_authority_is_approximate() {
    let wgsl = emit_multilayer_wgsl(1);
    assert!(wgsl.contains("var score") && wgsl.contains("+ f32(w)"));
    let obs2 = landed_jit_kernel_descriptors()
        .into_iter()
        .find(|d| d.id == FIELD_POLICY_OBS2_DESCRIPTOR_ID)
        .expect("obs2 descriptor");
    assert!(is_field_policy_obs2_multilayer_overlay_score_descriptor(
        &obs2
    ));
    validate_kernel_descriptor_admission(&obs2).expect("obs2 admits");
    let score = obs2
        .writes
        .iter()
        .find(|w| w.name == "score_bits")
        .expect("score_bits");
    assert_eq!(score.authority, OutputAuthority::ApproximateDiagnostic);
    println!("field_policy_obs2_score_authority: ApproximateDiagnosticF32 per_layer_mag=Exact");
}

#[test]
fn field_policy_obs2_no_default_runtime_wiring() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    let obs2 = landed_jit_kernel_descriptors()
        .into_iter()
        .find(|d| d.id == FIELD_POLICY_OBS2_DESCRIPTOR_ID)
        .expect("obs2 descriptor");
    assert!(obs2.default_off);
    assert!(!obs2.production_wiring);
    validate_kernel_descriptor_admission(&obs2).expect("obs2 admits");

    let wgsl = emit_multilayer_wgsl(1);
    for forbidden in [
        "SimSession",
        "ResourceEconomySpec",
        "simthing-sim",
        "KernelCache",
        "scheduler",
    ] {
        assert!(!wgsl.contains(forbidden));
    }
    println!("field_policy_obs2_wiring: descriptor=landed no_scheduler_no_bridge");
}
