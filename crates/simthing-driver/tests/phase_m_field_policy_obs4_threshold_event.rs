//! FIELD_POLICY-OBS-4 — GPU-resident threshold event emission from exact observer scores (Tier-2, test-only).
//!
//! OBS-3 exact score + Q16.16 threshold/hysteresis → deterministic state/event codes.
//! No CPU planner, no production wiring.

use std::sync::Mutex;
use std::time::Instant;

use simthing_gpu::GpuContext;
use simthing_spec::{
    fnv1a64_hex, is_field_policy_obs4_threshold_event_descriptor, landed_jit_kernel_descriptors,
    validate_kernel_descriptor_admission, EventAuthorityContract, MappingExecutionProfile,
    ThresholdAuthorityContract, FIELD_POLICY_OBS4_DESCRIPTOR_ID, FIELD_POLICY_OBS4_LAYER_COUNT,
    MAG2_Q16_SCALE, SQRT_F_ARTIFACT_HASH,
};

use simthing_spec::OutputAuthority;

static GPU_MUTEX: Mutex<()> = Mutex::new(());
const SQRT_CR_F_WGSL: &str = include_str!("wgsl/sqrt_cr_f_candidate.wgsl");

const LAYER_COUNT: usize = FIELD_POLICY_OBS4_LAYER_COUNT as usize;
const FIELDS_PER_LAYER: u32 = 3;
const INPUT_FIELDS: u32 = LAYER_COUNT as u32 * FIELDS_PER_LAYER + 4;
const ROW_STRIDE: u32 = INPUT_FIELDS + LAYER_COUNT as u32 + 4;
const BIAS_OFF: u32 = LAYER_COUNT as u32 * FIELDS_PER_LAYER;
const THRESHOLD_OFF: u32 = BIAS_OFF + 1;
const HYSTERESIS_OFF: u32 = BIAS_OFF + 2;
const PRIOR_STATE_OFF: u32 = BIAS_OFF + 3;
const MAG_BITS_BASE: u32 = INPUT_FIELDS;
const SCORE_FIXED_OFF: u32 = MAG_BITS_BASE + LAYER_COUNT as u32;
const STATE_OFF: u32 = SCORE_FIXED_OFF + 1;
const EVENT_CODE_OFF: u32 = STATE_OFF + 1;
const FLAGS_OFF: u32 = EVENT_CODE_OFF + 1;
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
    "urgency",
    "commitment",
];

const FORBIDDEN_EXACT_TERMS: &[&str] = &["f64", "F64RoundDown", "SHADER_F64", "sqrt_cr_c"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LayerInput {
    gx: i32,
    gy: i32,
    w: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ThresholdRow {
    layers: [LayerInput; LAYER_COUNT],
    bias: i32,
    threshold: i32,
    hysteresis: i32,
    prior_state: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ThresholdOutput {
    mag_bits: [u32; LAYER_COUNT],
    score_fixed: i32,
    state: u32,
    event_code: u32,
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

fn cpu_score_fixed(row: &ThresholdRow) -> i32 {
    let mut score = row.bias;
    for layer in &row.layers {
        let mag_fixed = mag_bits_to_q16_fixed(cpu_mag_bits(layer.gx, layer.gy));
        score = score.wrapping_add(q16_mul(layer.w, mag_fixed));
    }
    score
}

fn cpu_threshold_state_event(row: &ThresholdRow) -> (u32, u32) {
    let score = cpu_score_fixed(row);
    let up = row.threshold.wrapping_add(row.hysteresis);
    let down = row.threshold.wrapping_sub(row.hysteresis);
    if row.prior_state == 0 && score >= up {
        (1, 1)
    } else if row.prior_state == 1 && score <= down {
        (0, 2)
    } else {
        (row.prior_state, 0)
    }
}

fn score_accumulation_overflow(row: &ThresholdRow) -> bool {
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

fn emit_threshold_event_wgsl(batch_count: u32) -> String {
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
    let threshold = bitcast<i32>(data[base + {threshold_off}u]);
    let hysteresis = bitcast<i32>(data[base + {hysteresis_off}u]);
    let prior_state = data[base + {prior_off}u];
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
    var state_out = prior_state;
    var event_code = 0u;
    let up = threshold + hysteresis;
    let down = threshold - hysteresis;
    if (prior_state == 0u && score_fixed >= up) {{
        state_out = 1u;
        event_code = 1u;
    }} else if (prior_state == 1u && score_fixed <= down) {{
        state_out = 0u;
        event_code = 2u;
    }}
    data[base + {state_off}u] = state_out;
    data[base + {event_off}u] = event_code;
    data[base + {flags_off}u] = 0u;
}}
"#,
        f = SQRT_CR_F_WGSL,
        limb = limb_arith_wgsl(),
        batch_count = batch_count,
        stride = ROW_STRIDE,
        layer_count = LAYER_COUNT,
        fields_per_layer = FIELDS_PER_LAYER,
        bias_off = BIAS_OFF,
        threshold_off = THRESHOLD_OFF,
        hysteresis_off = HYSTERESIS_OFF,
        prior_off = PRIOR_STATE_OFF,
        mag_base = MAG_BITS_BASE,
        score_off = SCORE_FIXED_OFF,
        state_off = STATE_OFF,
        event_off = EVENT_CODE_OFF,
        flags_off = FLAGS_OFF,
    )
}

fn init_buffer(rows: &[ThresholdRow]) -> Vec<u32> {
    let mut data = vec![0u32; rows.len() * ROW_STRIDE as usize];
    for (i, row) in rows.iter().enumerate() {
        let base = i * ROW_STRIDE as usize;
        for (layer, inp) in row.layers.iter().enumerate() {
            let off = base + layer * FIELDS_PER_LAYER as usize;
            data[off] = inp.gx as u32;
            data[off + 1] = inp.gy as u32;
            data[off + 2] = inp.w as u32;
        }
        data[base + BIAS_OFF as usize] = row.bias as u32;
        data[base + THRESHOLD_OFF as usize] = row.threshold as u32;
        data[base + HYSTERESIS_OFF as usize] = row.hysteresis as u32;
        data[base + PRIOR_STATE_OFF as usize] = row.prior_state;
    }
    data
}

struct WarmRunOutcome {
    outputs: Vec<ThresholdOutput>,
    elapsed: std::time::Duration,
}

fn run_threshold_batch(
    ctx: &GpuContext,
    rows: &[ThresholdRow],
    repeat_dispatches: u32,
    do_readback: bool,
) -> WarmRunOutcome {
    use wgpu::util::DeviceExt;
    let n = rows.len() as u32;
    let wgsl = emit_threshold_event_wgsl(n);
    let data = init_buffer(rows);
    let device = &ctx.device;
    let queue = &ctx.queue;
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("field_policy_obs4_threshold_event"),
        source: wgpu::ShaderSource::Wgsl(wgsl.into()),
    });
    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("field_policy_obs4_bgl"),
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
        label: Some("field_policy_obs4_pipeline"),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("field_policy_obs4_pl"),
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
        label: Some("field_policy_obs4_values"),
        contents: bytemuck::cast_slice(&data),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("field_policy_obs4_bg"),
        layout: &bgl,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage.as_entire_binding(),
        }],
    });
    let t0 = Instant::now();
    for _ in 0..repeat_dispatches {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("field_policy_obs4_enc"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("field_policy_obs4_pass"),
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
        label: Some("field_policy_obs4_readback"),
        size: bytes,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("field_policy_obs4_readback_enc"),
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
            ThresholdOutput {
                mag_bits,
                score_fixed: bytemuck::cast(out[base + SCORE_FIXED_OFF as usize]),
                state: out[base + STATE_OFF as usize],
                event_code: out[base + EVENT_CODE_OFF as usize],
                flags: out[base + FLAGS_OFF as usize],
            }
        })
        .collect();

    WarmRunOutcome { outputs, elapsed }
}

fn layer_input(gx: f32, gy: f32, w: f32) -> LayerInput {
    LayerInput {
        gx: f32_to_q16(gx),
        gy: f32_to_q16(gy),
        w: f32_to_q16(w),
    }
}

fn edge_threshold_rows() -> Vec<(ThresholdRow, &'static str)> {
    let zero_layers = [layer_input(0.0, 0.0, 1.0); LAYER_COUNT];
    vec![
        (
            ThresholdRow {
                layers: zero_layers,
                bias: f32_to_q16(0.5),
                threshold: f32_to_q16(1.0),
                hysteresis: 0,
                prior_state: 0,
            },
            "below_threshold",
        ),
        (
            ThresholdRow {
                layers: [layer_input(1.0, 0.0, 1.0); LAYER_COUNT],
                bias: 0,
                threshold: f32_to_q16(1.0),
                hysteresis: 0,
                prior_state: 0,
            },
            "equal_threshold",
        ),
        (
            ThresholdRow {
                layers: [layer_input(2.0, 0.0, 2.0); LAYER_COUNT],
                bias: f32_to_q16(1.0),
                threshold: f32_to_q16(1.0),
                hysteresis: f32_to_q16(0.25),
                prior_state: 0,
            },
            "cross_up_prior0",
        ),
        (
            ThresholdRow {
                layers: [layer_input(0.5, 0.0, 1.0); LAYER_COUNT],
                bias: 0,
                threshold: f32_to_q16(2.0),
                hysteresis: f32_to_q16(0.5),
                prior_state: 1,
            },
            "cross_down_prior1",
        ),
        (
            ThresholdRow {
                layers: [layer_input(1.0, 0.0, 1.0); LAYER_COUNT],
                bias: 0,
                threshold: f32_to_q16(1.0),
                hysteresis: f32_to_q16(0.5),
                prior_state: 0,
            },
            "within_hysteresis_band",
        ),
        (
            ThresholdRow {
                layers: [layer_input(1.0, 0.0, 1.0); LAYER_COUNT],
                bias: 0,
                threshold: f32_to_q16(1.0),
                hysteresis: 0,
                prior_state: 1,
            },
            "zero_hysteresis",
        ),
        (
            ThresholdRow {
                layers: [layer_input(1.0, 0.0, -1.0); LAYER_COUNT],
                bias: f32_to_q16(-2.0),
                threshold: f32_to_q16(-1.0),
                hysteresis: f32_to_q16(0.25),
                prior_state: 0,
            },
            "negative_score",
        ),
        (
            ThresholdRow {
                layers: [layer_input(16.0, 0.0, 2.0); LAYER_COUNT],
                bias: f32_to_q16(4.0),
                threshold: f32_to_q16(8.0),
                hysteresis: f32_to_q16(1.0),
                prior_state: 0,
            },
            "large_positive_score",
        ),
    ]
}

fn gradient_samples() -> Vec<f32> {
    vec![
        0.0, 0.01, -0.01, 0.1, -0.1, 0.5, -0.5, 1.0, -1.0, 2.0, -2.0, 4.0, 8.0, 16.0,
    ]
}

fn weight_bias_samples() -> Vec<f32> {
    vec![-2.0, -0.5, 0.0, 0.5, 1.0, 2.0]
}

fn threshold_samples() -> Vec<f32> {
    vec![-2.0, -0.5, 0.0, 0.5, 1.0, 2.0, 4.0, 8.0]
}

fn hysteresis_samples() -> Vec<f32> {
    vec![0.0, 0.125, 0.25, 0.5, 1.0]
}

fn dense_threshold_rows() -> Vec<ThresholdRow> {
    let grads = gradient_samples();
    let weights = weight_bias_samples();
    let thresholds = threshold_samples();
    let hysteresis = hysteresis_samples();
    let mut out = Vec::new();
    for (pair_idx, _) in grads.iter().enumerate() {
        for _ in grads.iter() {
            for &bias in weights.iter() {
                for (ti, &thr) in thresholds.iter().enumerate() {
                    for &hys in hysteresis.iter() {
                        let prior = ((pair_idx + ti) % 2) as u32;
                        let layers = std::array::from_fn(|layer| {
                            let gi = (pair_idx + layer * 3) % grads.len();
                            let gj = (pair_idx + layer * 2 + 1) % grads.len();
                            let wi = (pair_idx + layer + ti) % weights.len();
                            layer_input(grads[gi], grads[gj], weights[wi])
                        });
                        out.push(ThresholdRow {
                            layers,
                            bias: f32_to_q16(bias),
                            threshold: f32_to_q16(thr),
                            hysteresis: f32_to_q16(hys),
                            prior_state: prior,
                        });
                    }
                }
            }
        }
    }
    out
}

fn mobile_threshold_rows(count: usize) -> Vec<ThresholdRow> {
    let grads = gradient_samples();
    let weights = weight_bias_samples();
    let thresholds = threshold_samples();
    let hysteresis = hysteresis_samples();
    let mut out = Vec::with_capacity(count);
    let mut state = 0x5345_4144u32;
    for idx in 0..count {
        let layers = std::array::from_fn(|_| {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let gx = grads[(state as usize) % grads.len()];
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let gy = grads[(state as usize) % grads.len()];
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let w = weights[(state as usize) % weights.len()];
            layer_input(gx, gy, w)
        });
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let bias = weights[(state as usize + idx) % weights.len()];
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let thr = thresholds[(state as usize) % thresholds.len()];
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let hys = hysteresis[(state as usize) % hysteresis.len()];
        out.push(ThresholdRow {
            layers,
            bias: f32_to_q16(bias),
            threshold: f32_to_q16(thr),
            hysteresis: f32_to_q16(hys),
            prior_state: (idx % 2) as u32,
        });
    }
    out
}

fn verify_outputs(
    outputs: &[ThresholdOutput],
    rows: &[ThresholdRow],
) -> (usize, usize, usize, usize, usize) {
    let mut score_exact = 0usize;
    let mut state_exact = 0usize;
    let mut event_exact = 0usize;
    let mut overflow = 0usize;
    let mut events_up = 0usize;
    for (out, row) in outputs.iter().zip(rows.iter()) {
        if out.score_fixed == cpu_score_fixed(row) {
            score_exact += 1;
        }
        let (exp_state, exp_event) = cpu_threshold_state_event(row);
        if out.state == exp_state {
            state_exact += 1;
        }
        if out.event_code == exp_event {
            event_exact += 1;
        }
        if score_accumulation_overflow(row) {
            overflow += 1;
        }
        if out.event_code == 1 {
            events_up += 1;
        }
    }
    (score_exact, state_exact, event_exact, overflow, events_up)
}

#[test]
fn field_policy_obs4_event_authority_is_exact_deterministic() {
    let desc = landed_jit_kernel_descriptors()
        .into_iter()
        .find(|d| d.id == FIELD_POLICY_OBS4_DESCRIPTOR_ID)
        .expect("obs4 descriptor");
    assert!(is_field_policy_obs4_threshold_event_descriptor(&desc));
    validate_kernel_descriptor_admission(&desc).expect("obs4 admits");

    let state = desc
        .writes
        .iter()
        .find(|w| w.name == "state_u32")
        .expect("state_u32");
    let event = desc
        .writes
        .iter()
        .find(|w| w.name == "event_code_u32")
        .expect("event_code_u32");
    assert_eq!(state.authority, OutputAuthority::ExactAuthoritative);
    assert_eq!(event.authority, OutputAuthority::ExactAuthoritative);

    let wgsl = emit_threshold_event_wgsl(1);
    assert!(!wgsl.contains("SimSession"));
    assert!(!wgsl.contains("planner"));
    assert!(!wgsl.contains("urgency"));
    assert!(!wgsl.contains("commitment"));

    let _ = ThresholdAuthorityContract::ExactQ16Threshold;
    let _ = EventAuthorityContract::ExactDeterministicEventFlag;
    println!(
        "field_policy_obs4_event_authority: state=Exact event_code=Exact gpu_resident=true no_cpu_planner=true"
    );
}
