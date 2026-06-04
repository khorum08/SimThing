//! SEAD V1 live GPU pipeline helpers (PIPE-0 + ACT-2 chain) for FrontierV1-5 fixtures (test-only).
//!
//! Extracted from `phase_m_sead_pipe0_observer_event_pipeline` and
//! `phase_m_sead_act2_proposal_admission_records`. No new semantic WGSL or descriptors.

use std::sync::Mutex;
use std::time::Instant;

use simthing_gpu::GpuContext;
use simthing_spec::{
    MAG2_Q16_SCALE, SEAD_ACT1_ADMITTED_TABLE_SIZE, SEAD_EVENT1_CODE_COUNT, SEAD_PIPE0_LAYER_COUNT,
};

pub static GPU_MUTEX: Mutex<()> = Mutex::new(());

/// Returns a GPU context when hardware is available (no mutex).
pub fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

pub fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let ctx = GpuContext::new_blocking().expect("GPU required");
    f(&ctx);
}

// --- PIPE-0 (observer threshold + compaction) ---

const SQRT_CR_F_WGSL: &str = include_str!("../wgsl/sqrt_cr_f_candidate.wgsl");

const LAYER_COUNT: usize = SEAD_PIPE0_LAYER_COUNT as usize;
const FIELDS_PER_LAYER: u32 = 3;
const INPUT_STRIDE: u32 = LAYER_COUNT as u32 * FIELDS_PER_LAYER + 4;
const EVENT_ROW_STRIDE: u32 = 5;
const RECORD_STRIDE: u32 = 5;
const BIAS_OFF: u32 = LAYER_COUNT as u32 * FIELDS_PER_LAYER;
const THRESHOLD_OFF: u32 = BIAS_OFF + 1;
const HYSTERESIS_OFF: u32 = BIAS_OFF + 2;
const PRIOR_STATE_OFF: u32 = BIAS_OFF + 3;
const Q16_SCALE_F: f32 = MAG2_Q16_SCALE as f32;
pub const PIPE0_ORDERING_CLASS: &str = "UnspecifiedAtomicOrder";

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
    "urgency",
    "commitment",
    "order",
    "route",
];

const FORBIDDEN_EXACT_TERMS: &[&str] = &["f64", "F64RoundDown", "SHADER_F64", "sqrt_cr_c"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayerInput {
    gx: i32,
    gy: i32,
    w: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObserverRow {
    layers: [LayerInput; LAYER_COUNT],
    bias: i32,
    threshold: i32,
    hysteresis: i32,
    prior_state: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventRow {
    source_index: u32,
    event_code: u32,
    state: u32,
    score_fixed: i32,
    flags: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct EventRecord {
    source_index: u32,
    event_code: u32,
    state: u32,
    score_fixed: i32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PipelineParams {
    row_count: u32,
    capacity: u32,
    _pad: [u32; 2],
}

pub struct Pipe0Outcome {
    event_rows: Vec<EventRow>,
    event_count: u32,
    overflow: u32,
    records: Vec<EventRecord>,
    elapsed: std::time::Duration,
    dispatch_count: u32,
}

pub fn f32_to_q16(v: f32) -> i32 {
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

pub fn cpu_score_fixed(row: &ObserverRow) -> i32 {
    let mut score = row.bias;
    for layer in &row.layers {
        let mag_fixed = mag_bits_to_q16_fixed(cpu_mag_bits(layer.gx, layer.gy));
        score = score.wrapping_add(q16_mul(layer.w, mag_fixed));
    }
    score
}

pub fn cpu_threshold_state_event(row: &ObserverRow) -> (u32, u32, i32) {
    let score = cpu_score_fixed(row);
    let up = row.threshold.wrapping_add(row.hysteresis);
    let down = row.threshold.wrapping_sub(row.hysteresis);
    if row.prior_state == 0 && score >= up {
        (1, 1, score)
    } else if row.prior_state == 1 && score <= down {
        (0, 2, score)
    } else {
        (row.prior_state, 0, score)
    }
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

pub fn emit_pipeline_wgsl() -> String {
    format!(
        r#"{f}
{limb}

const OBS_INPUT_STRIDE: u32 = {input_stride}u;
const EVENT_ROW_STRIDE: u32 = {event_stride}u;
const RECORD_STRIDE: u32 = {record_stride}u;
const LAYER_COUNT: u32 = {layer_count}u;
const FIELDS_PER_LAYER: u32 = {fields_per_layer}u;

struct Params {{
    row_count: u32,
    capacity: u32,
    _pad0: u32,
    _pad1: u32,
}}

@group(0) @binding(0) var<storage, read> observer_inputs: array<u32>;
@group(0) @binding(1) var<storage, read_write> event_rows: array<u32>;
@group(0) @binding(2) var<storage, read_write> counters: array<atomic<u32>, 2>;
@group(0) @binding(3) var<storage, read_write> records: array<u32>;
@group(0) @binding(4) var<uniform> params: Params;

@compute @workgroup_size(64)
fn threshold_pass(@builtin(global_invocation_id) gid: vec3<u32>) {{
    let i = gid.x;
    if (i >= params.row_count) {{ return; }}
    let base = i * OBS_INPUT_STRIDE;
    let bias = bitcast<i32>(observer_inputs[base + {bias_off}u]);
    let threshold = bitcast<i32>(observer_inputs[base + {threshold_off}u]);
    let hysteresis = bitcast<i32>(observer_inputs[base + {hysteresis_off}u]);
    let prior_state = observer_inputs[base + {prior_off}u];
    var score_fixed = bias;
    for (var layer = 0u; layer < LAYER_COUNT; layer = layer + 1u) {{
        let in_off = base + layer * FIELDS_PER_LAYER;
        let gx = bitcast<i32>(observer_inputs[in_off]);
        let gy = bitcast<i32>(observer_inputs[in_off + 1u]);
        let w = bitcast<i32>(observer_inputs[in_off + 2u]);
        let sum = mag2_sum_q16(gx, gy);
        let mag2_bits = mag2_u64_q16_to_f32_bits(sum);
        let mag_bits = sqrt_cr_f_bits(mag2_bits);
        let mag_fixed = mag_bits_to_q16_fixed(mag_bits);
        score_fixed = score_fixed + q16_mul(w, mag_fixed);
    }}
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
    let out_base = i * EVENT_ROW_STRIDE;
    event_rows[out_base] = i;
    event_rows[out_base + 1u] = event_code;
    event_rows[out_base + 2u] = state_out;
    event_rows[out_base + 3u] = bitcast<u32>(score_fixed);
    event_rows[out_base + 4u] = 0u;
}}

@compute @workgroup_size(64)
fn compact_pass(@builtin(global_invocation_id) gid: vec3<u32>) {{
    let i = gid.x;
    if (i >= params.row_count) {{ return; }}
    let base = i * EVENT_ROW_STRIDE;
    let code = event_rows[base + 1u];
    if (code == 0u) {{ return; }}
    let slot = atomicAdd(&counters[0], 1u);
    if (slot >= params.capacity) {{
        atomicStore(&counters[1], 1u);
        return;
    }}
    let out_base = slot * RECORD_STRIDE;
    records[out_base] = event_rows[base];
    records[out_base + 1u] = code;
    records[out_base + 2u] = event_rows[base + 2u];
    records[out_base + 3u] = event_rows[base + 3u];
    records[out_base + 4u] = 0u;
}}
"#,
        f = SQRT_CR_F_WGSL,
        limb = limb_arith_wgsl(),
        input_stride = INPUT_STRIDE,
        event_stride = EVENT_ROW_STRIDE,
        record_stride = RECORD_STRIDE,
        layer_count = LAYER_COUNT,
        fields_per_layer = FIELDS_PER_LAYER,
        bias_off = BIAS_OFF,
        threshold_off = THRESHOLD_OFF,
        hysteresis_off = HYSTERESIS_OFF,
        prior_off = PRIOR_STATE_OFF,
    )
}

pub fn pack_observer_inputs(rows: &[ObserverRow]) -> Vec<u32> {
    let mut data = vec![0u32; rows.len() * INPUT_STRIDE as usize];
    for (i, row) in rows.iter().enumerate() {
        let base = i * INPUT_STRIDE as usize;
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

pub fn cpu_event_rows(rows: &[ObserverRow]) -> Vec<EventRow> {
    rows.iter()
        .enumerate()
        .map(|(i, row)| {
            let (state, code, score) = cpu_threshold_state_event(row);
            EventRow {
                source_index: i as u32,
                event_code: code,
                state,
                score_fixed: score,
                flags: 0,
            }
        })
        .collect()
}

pub fn cpu_pipe0_expected_records(event_rows: &[EventRow]) -> Vec<EventRecord> {
    event_rows
        .iter()
        .filter(|row| row.event_code != 0)
        .map(|row| EventRecord {
            source_index: row.source_index,
            event_code: row.event_code,
            state: row.state,
            score_fixed: row.score_fixed,
        })
        .collect()
}

fn sort_records(records: &mut [EventRecord]) {
    records.sort_by(|a, b| {
        (a.source_index, a.event_code, a.state, a.score_fixed).cmp(&(
            b.source_index,
            b.event_code,
            b.state,
            b.score_fixed,
        ))
    });
}

pub fn cpu_pipe0_membership_exact(expected: &[EventRecord], got: &[EventRecord]) -> bool {
    if expected.len() != got.len() {
        return false;
    }
    let mut a = expected.to_vec();
    let mut b = got.to_vec();
    sort_records(&mut a);
    sort_records(&mut b);
    a == b
}

pub fn run_pipe0_gpu(
    ctx: &GpuContext,
    rows: &[ObserverRow],
    capacity: u32,
    repeat_cycles: u32,
    do_readback: bool,
) -> Pipe0Outcome {
    use wgpu::util::DeviceExt;
    let device = &ctx.device;
    let queue = &ctx.queue;
    let n = rows.len() as u32;
    let wgsl = emit_pipeline_wgsl();
    let observer_data = pack_observer_inputs(rows);
    let params = PipelineParams {
        row_count: n,
        capacity,
        _pad: [0, 0],
    };

    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("sead_pipe0_shader"),
        source: wgpu::ShaderSource::Wgsl(wgsl.into()),
    });

    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("sead_pipe0_bgl"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
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
            wgpu::BindGroupLayoutEntry {
                binding: 4,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("sead_pipe0_pl"),
        bind_group_layouts: &[&bgl],
        push_constant_ranges: &[],
    });

    let threshold_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("sead_pipe0_threshold"),
        layout: Some(&pl),
        module: &module,
        entry_point: "threshold_pass",
        compilation_options: Default::default(),
        cache: None,
    });
    let compact_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("sead_pipe0_compact"),
        layout: Some(&pl),
        module: &module,
        entry_point: "compact_pass",
        compilation_options: Default::default(),
        cache: None,
    });

    let observer_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("sead_pipe0_observer"),
        contents: bytemuck::cast_slice(&observer_data),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let event_row_words = (n * EVENT_ROW_STRIDE) as usize;
    let event_rows_init = vec![0u32; event_row_words];
    let event_rows_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("sead_pipe0_event_rows"),
        contents: bytemuck::cast_slice(&event_rows_init),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let counters_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("sead_pipe0_counters"),
        contents: &[0u8; 8],
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let record_words = (capacity.max(1) * RECORD_STRIDE) as usize;
    let records_init = vec![0u32; record_words];
    let records_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("sead_pipe0_records"),
        contents: bytemuck::cast_slice(&records_init),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let params_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("sead_pipe0_params"),
        contents: bytemuck::bytes_of(&params),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("sead_pipe0_bg"),
        layout: &bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: observer_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: event_rows_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: counters_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: records_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 4,
                resource: params_buf.as_entire_binding(),
            },
        ],
    });

    let wg = n.div_ceil(64);
    let t0 = Instant::now();
    for _ in 0..repeat_cycles {
        queue.write_buffer(&counters_buf, 0, &[0u8; 8]);
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("sead_pipe0_enc"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("sead_pipe0_threshold_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&threshold_pipeline);
            pass.set_bind_group(0, &bg, &[]);
            pass.dispatch_workgroups(wg, 1, 1);
        }
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("sead_pipe0_compact_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&compact_pipeline);
            pass.set_bind_group(0, &bg, &[]);
            pass.dispatch_workgroups(wg, 1, 1);
        }
        queue.submit(Some(encoder.finish()));
    }
    let elapsed = t0.elapsed();

    if !do_readback {
        return Pipe0Outcome {
            event_rows: Vec::new(),
            event_count: 0,
            overflow: 0,
            records: Vec::new(),
            elapsed,
            dispatch_count: repeat_cycles * 2,
        };
    }

    let counters_staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("sead_pipe0_counters_readback"),
        size: 8,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let event_rows_bytes = (event_row_words * 4) as u64;
    let event_rows_staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("sead_pipe0_event_rows_readback"),
        size: event_rows_bytes,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let records_bytes = (record_words * 4) as u64;
    let records_staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("sead_pipe0_records_readback"),
        size: records_bytes.max(4),
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("sead_pipe0_readback_enc"),
    });
    enc2.copy_buffer_to_buffer(&counters_buf, 0, &counters_staging, 0, 8);
    enc2.copy_buffer_to_buffer(&event_rows_buf, 0, &event_rows_staging, 0, event_rows_bytes);
    if records_bytes > 0 {
        enc2.copy_buffer_to_buffer(&records_buf, 0, &records_staging, 0, records_bytes);
    }
    queue.submit(Some(enc2.finish()));

    let counters_slice = counters_staging.slice(..);
    counters_slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);
    let counters_mapped = counters_slice.get_mapped_range();
    let meta: [u32; 2] = bytemuck::cast_slice(&counters_mapped)[..2]
        .try_into()
        .unwrap();
    drop(counters_mapped);
    counters_staging.unmap();

    let event_rows_slice = event_rows_staging.slice(..);
    event_rows_slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);
    let er_mapped = event_rows_slice.get_mapped_range();
    let er_words: &[u32] = bytemuck::cast_slice(&er_mapped);
    let mut event_rows = Vec::with_capacity(n as usize);
    for i in 0..n as usize {
        let base = i * EVENT_ROW_STRIDE as usize;
        event_rows.push(EventRow {
            source_index: er_words[base],
            event_code: er_words[base + 1],
            state: er_words[base + 2],
            score_fixed: bytemuck::cast(er_words[base + 3]),
            flags: er_words[base + 4],
        });
    }
    drop(er_mapped);
    event_rows_staging.unmap();

    let mut records = Vec::new();
    if records_bytes > 0 {
        let rec_slice = records_staging.slice(..);
        rec_slice.map_async(wgpu::MapMode::Read, |_| {});
        device.poll(wgpu::Maintain::Wait);
        let rec_mapped = rec_slice.get_mapped_range();
        let rec_words: &[u32] = bytemuck::cast_slice(&rec_mapped);
        let written = meta[0].min(capacity) as usize;
        records.reserve(written);
        for slot in 0..written {
            let base = slot * RECORD_STRIDE as usize;
            records.push(EventRecord {
                source_index: rec_words[base],
                event_code: rec_words[base + 1],
                state: rec_words[base + 2],
                score_fixed: bytemuck::cast(rec_words[base + 3]),
            });
        }
        drop(rec_mapped);
        records_staging.unmap();
    }

    Pipe0Outcome {
        event_rows,
        event_count: meta[0],
        overflow: meta[1],
        records,
        elapsed,
        dispatch_count: repeat_cycles * 2,
    }
}

pub fn layer_input(gx: f32, gy: f32, w: f32) -> LayerInput {
    LayerInput {
        gx: f32_to_q16(gx),
        gy: f32_to_q16(gy),
        w: f32_to_q16(w),
    }
}

fn zero_layers() -> [LayerInput; LAYER_COUNT] {
    [layer_input(0.0, 0.0, 1.0); LAYER_COUNT]
}

/// Build one PIPE-0 observer row from layer gains and threshold parameters.
pub fn observer_row_from_layers(
    layers: [LayerInput; LAYER_COUNT],
    bias: f32,
    threshold: f32,
    hysteresis: i32,
    prior_state: u32,
) -> ObserverRow {
    ObserverRow {
        layers,
        bias: f32_to_q16(bias),
        threshold: f32_to_q16(threshold),
        hysteresis,
        prior_state,
    }
}

/// Field-derived observer rows for FrontierV1-5 (two rows for ACT-2 min_count on event_code 1).
pub fn frontier_field_observer_rows(urgency: f32, threat: f32) -> Vec<ObserverRow> {
    let g0 = (urgency / 100.0).clamp(1.5, 4.0);
    let g1 = (threat / 1000.0).clamp(1.5, 4.0);
    let mut layers = zero_layers();
    layers[0] = layer_input(g0, 0.0, 2.0);
    layers[1] = layer_input(g1, 0.0, 2.0);
    let row = observer_row_from_layers(layers, 1.0, 1.0, 0, 0);
    let (_, event_code, _) = cpu_threshold_state_event(&row);
    debug_assert_eq!(
        event_code, 1,
        "frontier observer row must emit event_code 1"
    );
    vec![row.clone(), row]
}

pub fn pipe0_records_to_act2(records: &[EventRecord]) -> Vec<Act2EventRecord> {
    records
        .iter()
        .map(|r| Act2EventRecord {
            source_index: r.source_index,
            event_code: r.event_code,
            state: r.state,
            score_fixed: r.score_fixed,
        })
        .collect()
}

impl Pipe0Outcome {
    pub fn event_count(&self) -> u32 {
        self.event_count
    }

    pub fn overflow(&self) -> u32 {
        self.overflow
    }

    pub fn records(&self) -> &[EventRecord] {
        &self.records
    }
}

impl Act2ChainOutcome {
    pub fn proposal_count(&self) -> u32 {
        self.proposal_count
    }

    pub fn proposal_overflow(&self) -> u32 {
        self.proposal_overflow
    }

    pub fn admission(&self) -> AdmissionRecord {
        self.admission
    }

    pub fn summary(&self) -> ProposalSummary {
        self.summary
    }
}

impl ProposalRecord {
    pub fn proposal_code(&self) -> u32 {
        self.proposal_code
    }
}

impl ProposalSummary {
    pub fn accepted_count(&self) -> u32 {
        self.accepted_count
    }
}

impl AdmissionRecord {
    pub fn admission_code(&self) -> u32 {
        self.admission_code
    }

    pub fn flags(&self) -> u32 {
        self.flags
    }

    pub fn admitted(&self) -> bool {
        self.flags & FLAG_ADM_ADMITTED != 0
    }
}

// --- ACT-2 (bucket -> reduce -> propose -> consume -> admit) ---

const CODE_COUNT: usize = SEAD_EVENT1_CODE_COUNT as usize;
const RED_OUT_STRIDE: u32 = 6;
const PROP_STRIDE: u32 = 5;
const SUMMARY_STRIDE: u32 = 7;
const ADMIT_STRIDE: u32 = 7;
const ADMITTED_TABLE_SIZE: usize = SEAD_ACT1_ADMITTED_TABLE_SIZE as usize;
const FLAG_RED_EMPTY: u32 = 1;
const FLAG_RED_SUM_OVERFLOW: u32 = 2;
const FLAG_RULE_MAX: u32 = 1;
const FLAG_RULE_SUM: u32 = 2;
const FLAG_SUM_OVERFLOW: u32 = 1;
const FLAG_INPUT_OVERFLOW: u32 = 2;
const FLAG_ADM_ADMITTED: u32 = 1;
const FLAG_ADM_REJ_COUNT: u32 = 2;
const FLAG_ADM_REJ_SCORE: u32 = 4;
const FLAG_ADM_REJ_INVALID: u32 = 8;
const FLAG_ADM_INPUT_OVF: u32 = 16;
const FLAG_ADM_SUM_OVF: u32 = 32;
pub const ACT2_ORDERING_CLASS: &str = "OrderInvariantExact";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProposalRecord {
    source_code: u32,
    proposal_code: u32,
    count: u32,
    score: i32,
    flags: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReductionResult {
    count: u32,
    sum_lo: u32,
    sum_hi: i32,
    min_score: i32,
    max_score: i32,
    flags: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Act2EventRecord {
    source_index: u32,
    event_code: u32,
    state: u32,
    score_fixed: i32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ProposalRuleGpu {
    min_count: u32,
    threshold_max: i32,
    threshold_sum_lo: u32,
    threshold_sum_hi: i32,
    proposal_code_max: u32,
    proposal_code_sum: u32,
    enable_sum_rule: u32,
    _pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ProposeParams {
    code_count: u32,
    proposal_capacity: u32,
    _pad: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ReduceParams {
    capacity_per_code: u32,
    code_count: u32,
    _pad: [u32; 2],
}

pub struct ProposalOutcome {
    proposal_count: u32,
    proposal_overflow: u32,
    proposals: Vec<ProposalRecord>,
    elapsed: std::time::Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProposalSummary {
    accepted_count: u32,
    ignored_count: u32,
    invalid_count: u32,
    summary_lo: u32,
    summary_hi: i32,
    max_score: i32,
    flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ConsumerParams {
    proposal_capacity: u32,
    admitted_count: u32,
    _pad: [u32; 2],
}

pub struct ConsumerOutcome {
    summary: ProposalSummary,
    proposal_count: u32,
    proposal_overflow: u32,
    elapsed: std::time::Duration,
}

pub struct Act2ChainOutcome {
    reductions: [ReductionResult; CODE_COUNT],
    proposal_count: u32,
    proposal_overflow: u32,
    summary: ProposalSummary,
    admission: AdmissionRecord,
    elapsed: std::time::Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmissionRecord {
    admission_code: u32,
    accepted_count: u32,
    invalid_count: u32,
    summary_lo: u32,
    summary_hi: i32,
    max_score: i32,
    flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct AdmissionRulesGpu {
    admission_code: u32,
    min_accepted: u32,
    min_max_score: i32,
    max_invalid: u32,
    _pad: u32,
}

pub struct AdmitOutcome {
    admission: AdmissionRecord,
    elapsed: std::time::Duration,
}

pub fn default_rules() -> [ProposalRuleGpu; CODE_COUNT] {
    [
        ProposalRuleGpu {
            min_count: u32::MAX,
            threshold_max: i32::MAX,
            threshold_sum_lo: u32::MAX,
            threshold_sum_hi: i32::MAX,
            proposal_code_max: 0,
            proposal_code_sum: 0,
            enable_sum_rule: 0,
            _pad: 0,
        },
        ProposalRuleGpu {
            min_count: 2,
            threshold_max: 500,
            threshold_sum_lo: 0,
            threshold_sum_hi: 0,
            proposal_code_max: 1001,
            proposal_code_sum: 2001,
            enable_sum_rule: 1,
            _pad: 0,
        },
        ProposalRuleGpu {
            min_count: 1,
            threshold_max: 200,
            threshold_sum_lo: 0,
            threshold_sum_hi: 0,
            proposal_code_max: 1002,
            proposal_code_sum: 2002,
            enable_sum_rule: 0,
            _pad: 0,
        },
        ProposalRuleGpu {
            min_count: 3,
            threshold_max: 1000,
            threshold_sum_lo: 0,
            threshold_sum_hi: 0,
            proposal_code_max: 1003,
            proposal_code_sum: 2003,
            enable_sum_rule: 0,
            _pad: 0,
        },
    ]
}

pub fn default_admitted_table() -> [u32; ADMITTED_TABLE_SIZE] {
    let mut table = [0u32; ADMITTED_TABLE_SIZE];
    table[0] = 1001;
    table[1] = 1002;
    table[2] = 1003;
    table[3] = 2001;
    table[4] = 2002;
    table
}

pub fn default_admitted_count() -> u32 {
    5
}

pub fn default_admission_rules() -> AdmissionRulesGpu {
    AdmissionRulesGpu {
        admission_code: 5001,
        min_accepted: 1,
        min_max_score: 0,
        max_invalid: 10,
        _pad: 0,
    }
}

pub fn smoke_admission_rules() -> AdmissionRulesGpu {
    AdmissionRulesGpu {
        admission_code: 5001,
        min_accepted: 1,
        min_max_score: 0,
        max_invalid: 100,
        _pad: 0,
    }
}

fn emit_admit_wgsl() -> &'static str {
    r#"
const SUM_SUM_OVF: u32 = 1u;
const SUM_IN_OVF: u32 = 2u;
const ADM_ADMITTED: u32 = 1u;
const ADM_REJ_COUNT: u32 = 2u;
const ADM_REJ_SCORE: u32 = 4u;
const ADM_REJ_INVALID: u32 = 8u;
const ADM_INPUT_OVF: u32 = 16u;
const ADM_SUM_OVF: u32 = 32u;

struct Rules {
    admission_code: u32,
    min_accepted: u32,
    min_max_score: i32,
    max_invalid: u32,
    _pad: u32,
}

@group(0) @binding(0) var<storage, read> proposal_summary: array<u32>;
@group(0) @binding(1) var<storage, read_write> admission_record: array<u32>;
@group(0) @binding(2) var<uniform> rules: Rules;

@compute @workgroup_size(1)
fn admit_pass(@builtin(global_invocation_id) gid: vec3<u32>) {
    if (gid.x != 0u) { return; }
    let accepted = proposal_summary[0];
    let invalid = proposal_summary[2];
    let sum_lo = proposal_summary[3];
    let sum_hi = bitcast<i32>(proposal_summary[4]);
    let max_score = bitcast<i32>(proposal_summary[5]);
    let sum_flags = proposal_summary[6];
    var flags = 0u;
    if ((sum_flags & SUM_IN_OVF) != 0u) { flags = flags | ADM_INPUT_OVF; }
    if ((sum_flags & SUM_SUM_OVF) != 0u) { flags = flags | ADM_SUM_OVF; }
    if (accepted < rules.min_accepted) { flags = flags | ADM_REJ_COUNT; }
    if (max_score < rules.min_max_score) { flags = flags | ADM_REJ_SCORE; }
    if (invalid > rules.max_invalid) { flags = flags | ADM_REJ_INVALID; }
    if ((flags & (ADM_REJ_COUNT | ADM_REJ_SCORE | ADM_REJ_INVALID | ADM_INPUT_OVF | ADM_SUM_OVF)) == 0u) {
        flags = flags | ADM_ADMITTED;
    }
    admission_record[0] = rules.admission_code;
    admission_record[1] = accepted;
    admission_record[2] = invalid;
    admission_record[3] = sum_lo;
    admission_record[4] = bitcast<u32>(sum_hi);
    admission_record[5] = bitcast<u32>(max_score);
    admission_record[6] = flags;
}
"#
}

pub fn is_admitted_proposal_code(
    code: u32,
    table: &[u32; ADMITTED_TABLE_SIZE],
    count: u32,
) -> bool {
    for slot in 0..count as usize {
        if table[slot] == code {
            return true;
        }
    }
    false
}

fn emit_consume_wgsl() -> &'static str {
    r#"
const PROP_STRIDE: u32 = 5u;
const ADMITTED_TABLE_SIZE: u32 = 16u;
const FLAG_SUM_OVF: u32 = 1u;
const FLAG_INPUT_OVF: u32 = 2u;

struct Params {
    proposal_capacity: u32,
    admitted_count: u32,
    _pad0: u32,
    _pad1: u32,
}

@group(0) @binding(0) var<storage, read> proposal_meta: array<u32, 2>;
@group(0) @binding(1) var<storage, read> proposal_records: array<u32>;
@group(0) @binding(2) var<storage, read> admitted_codes: array<u32, ADMITTED_TABLE_SIZE>;
@group(0) @binding(3) var<storage, read_write> proposal_summary: array<u32>;
@group(0) @binding(4) var<uniform> params: Params;

fn is_admitted(code: u32) -> bool {
    for (var i = 0u; i < params.admitted_count; i = i + 1u) {
        if (admitted_codes[i] == code) { return true; }
    }
    return false;
}

fn i64_add_i32_checked(hi: i32, lo: u32, add: i32) -> vec3<u32> {
    let add_lo = bitcast<u32>(add);
    let new_lo = lo + add_lo;
    var new_hi = hi;
    var ovf = 0u;
    if (add >= 0) {
        let carry = select(0u, 1u, new_lo < lo);
        new_hi = hi + i32(carry);
        if (hi == 2147483647 && carry == 1u) { ovf = 1u; }
    } else {
        let borrow = select(0u, 1u, new_lo > lo);
        new_hi = hi - 1;
        if (borrow == 0u) { new_hi = hi; }
        if (hi == -2147483648 && borrow == 1u) { ovf = 1u; }
    }
    return vec3(bitcast<u32>(new_hi), new_lo, ovf);
}

@compute @workgroup_size(1)
fn consume_pass(@builtin(global_invocation_id) gid: vec3<u32>) {
    if (gid.x != 0u) { return; }
    let proposal_count = proposal_meta[0];
    let proposal_overflow = proposal_meta[1];
    let scan = min(proposal_count, params.proposal_capacity);
    var accepted = 0u;
    var invalid = 0u;
    var sum_hi: i32 = 0;
    var sum_lo: u32 = 0u;
    var sum_ovf = 0u;
    var max_s: i32 = 0;
    var has_any = false;
    for (var slot = 0u; slot < scan; slot = slot + 1u) {
        let base = slot * PROP_STRIDE;
        let code = proposal_records[base + 1u];
        let score = bitcast<i32>(proposal_records[base + 3u]);
        if (is_admitted(code)) {
            accepted = accepted + 1u;
            if (!has_any) {
                max_s = score;
                has_any = true;
            } else {
                max_s = max(max_s, score);
            }
            if (sum_ovf == 0u) {
                let step = i64_add_i32_checked(sum_hi, sum_lo, score);
                sum_hi = bitcast<i32>(step.x);
                sum_lo = step.y;
                if (step.z != 0u) { sum_ovf = 1u; }
            }
        } else {
            invalid = invalid + 1u;
        }
    }
    let ignored = proposal_count - scan;
    var flags = 0u;
    if (sum_ovf != 0u) { flags = flags | FLAG_SUM_OVF; }
    if (proposal_overflow != 0u) { flags = flags | FLAG_INPUT_OVF; }
    proposal_summary[0] = accepted;
    proposal_summary[1] = ignored;
    proposal_summary[2] = invalid;
    proposal_summary[3] = sum_lo;
    proposal_summary[4] = bitcast<u32>(sum_hi);
    proposal_summary[5] = bitcast<u32>(max_s);
    proposal_summary[6] = flags;
}
"#
}

fn limbs_to_i64(hi: i32, lo: u32) -> i64 {
    ((i64::from(hi)) << 32) | ((lo as u64) & 0xFFFF_FFFF) as i64
}

pub fn act2_event_rec(index: u32, code: u32, state: u32, score: i32) -> Act2EventRecord {
    Act2EventRecord {
        source_index: index,
        event_code: code,
        state,
        score_fixed: score,
    }
}

fn emit_proposal_wgsl() -> &'static str {
    r#"
const RED_STRIDE: u32 = 6u;
const PROP_STRIDE: u32 = 5u;
const CODE_COUNT: u32 = 4u;
const FLAG_RULE_MAX: u32 = 1u;
const FLAG_RULE_SUM: u32 = 2u;
const FLAG_PROP_OVF: u32 = 4u;
const RED_EMPTY: u32 = 1u;
const RED_SUM_OVF: u32 = 2u;

struct Rule {
    min_count: u32,
    threshold_max: i32,
    threshold_sum_lo: u32,
    threshold_sum_hi: i32,
    proposal_code_max: u32,
    proposal_code_sum: u32,
    enable_sum_rule: u32,
    _pad: u32,
}

struct Params {
    code_count: u32,
    proposal_capacity: u32,
    _pad0: u32,
    _pad1: u32,
}

@group(0) @binding(0) var<storage, read> reductions: array<u32>;
@group(0) @binding(1) var<storage, read> rules: array<Rule, CODE_COUNT>;
@group(0) @binding(2) var<storage, read_write> proposal_meta: array<atomic<u32>, 2>;
@group(0) @binding(3) var<storage, read_write> proposal_records: array<u32>;
@group(0) @binding(4) var<uniform> params: Params;

fn i64_ge(hi_a: i32, lo_a: u32, hi_b: i32, lo_b: u32) -> bool {
    if (hi_a != hi_b) { return hi_a > hi_b; }
    return lo_a >= lo_b;
}

fn try_emit(source_code: u32, proposal_code: u32, count: u32, score: i32, flags: u32) {
    let slot = atomicAdd(&proposal_meta[0], 1u);
    if (slot >= params.proposal_capacity) {
        atomicStore(&proposal_meta[1], 1u);
        return;
    }
    let base = slot * PROP_STRIDE;
    proposal_records[base] = source_code;
    proposal_records[base + 1u] = proposal_code;
    proposal_records[base + 2u] = count;
    proposal_records[base + 3u] = bitcast<u32>(score);
    proposal_records[base + 4u] = flags;
}

@compute @workgroup_size(1)
fn propose_pass(@builtin(global_invocation_id) gid: vec3<u32>) {
    let code = gid.x;
    if (code >= params.code_count) { return; }
    let rule = rules[code];
    let red = code * RED_STRIDE;
    let count = reductions[red];
    let sum_lo = reductions[red + 1u];
    let sum_hi = bitcast<i32>(reductions[red + 2u]);
    let max_score = bitcast<i32>(reductions[red + 4u]);
    let red_flags = reductions[red + 5u];
    if ((red_flags & RED_EMPTY) != 0u) { return; }
    var pass_flags = red_flags;
    if (count >= rule.min_count && max_score >= rule.threshold_max) {
        try_emit(code, rule.proposal_code_max, count, max_score, FLAG_RULE_MAX | pass_flags);
    }
    if (rule.enable_sum_rule != 0u && (red_flags & RED_SUM_OVF) == 0u && count >= rule.min_count) {
        if (i64_ge(sum_hi, sum_lo, rule.threshold_sum_hi, rule.threshold_sum_lo)) {
            try_emit(code, rule.proposal_code_sum, count, max_score, FLAG_RULE_SUM | pass_flags);
        }
    }
}
"#
}

fn emit_reduction_wgsl() -> &'static str {
    r#"
const RECORD_STRIDE: u32 = 5u;
const CODE_COUNT: u32 = 4u;
const OUT_STRIDE: u32 = 6u;
const FLAG_EMPTY: u32 = 1u;
const FLAG_SUM_OVERFLOW: u32 = 2u;

struct Params {
    capacity_per_code: u32,
    code_count: u32,
    _pad0: u32,
    _pad1: u32,
}

@group(0) @binding(0) var<storage, read> bucket_counts: array<u32, CODE_COUNT>;
@group(0) @binding(1) var<storage, read> bucket_records: array<u32>;
@group(0) @binding(2) var<storage, read_write> reductions: array<u32>;
@group(0) @binding(3) var<uniform> params: Params;

fn i64_add_i32_checked(hi: i32, lo: u32, add: i32) -> vec3<u32> {
    let add_lo = bitcast<u32>(add);
    let new_lo = lo + add_lo;
    var new_hi = hi;
    var ovf = 0u;
    if (add >= 0) {
        let carry = select(0u, 1u, new_lo < lo);
        new_hi = hi + i32(carry);
        if (hi == 2147483647 && carry == 1u) { ovf = 1u; }
    } else {
        let borrow = select(0u, 1u, new_lo > lo);
        new_hi = hi - 1;
        if (borrow == 0u) { new_hi = hi; }
        if (hi == -2147483648 && borrow == 1u) { ovf = 1u; }
    }
    return vec3(bitcast<u32>(new_hi), new_lo, ovf);
}

@compute @workgroup_size(1)
fn reduce_pass(@builtin(global_invocation_id) gid: vec3<u32>) {
    let code = gid.x;
    if (code >= params.code_count) { return; }
    let attempt = bucket_counts[code];
    let scan = min(attempt, params.capacity_per_code);
    var sum_hi: i32 = 0;
    var sum_lo: u32 = 0u;
    var sum_ovf = 0u;
    var min_s: i32 = 0;
    var max_s: i32 = 0;
    var has_any = false;
    for (var slot = 0u; slot < scan; slot = slot + 1u) {
        let base = code * params.capacity_per_code * RECORD_STRIDE + slot * RECORD_STRIDE;
        let score = bitcast<i32>(bucket_records[base + 3u]);
        if (!has_any) {
            min_s = score;
            max_s = score;
            has_any = true;
        } else {
            min_s = min(min_s, score);
            max_s = max(max_s, score);
        }
        if (sum_ovf == 0u) {
            let step = i64_add_i32_checked(sum_hi, sum_lo, score);
            sum_hi = bitcast<i32>(step.x);
            sum_lo = step.y;
            if (step.z != 0u) { sum_ovf = 1u; }
        }
    }
    let out = code * OUT_STRIDE;
    var flags = 0u;
    if (!has_any) { flags = FLAG_EMPTY; }
    if (sum_ovf != 0u) { flags = flags | FLAG_SUM_OVERFLOW; }
    reductions[out] = scan;
    reductions[out + 1u] = sum_lo;
    reductions[out + 2u] = bitcast<u32>(sum_hi);
    reductions[out + 3u] = bitcast<u32>(min_s);
    reductions[out + 4u] = bitcast<u32>(max_s);
    reductions[out + 5u] = flags;
}
"#
}

fn emit_bucket_wgsl() -> &'static str {
    r#"
const RECORD_STRIDE: u32 = 5u;
const CODE_COUNT: u32 = 4u;
struct BucketParams { record_count: u32, capacity_per_code: u32, code_count: u32, _pad: u32, }
@group(0) @binding(0) var<storage, read> records: array<u32>;
@group(0) @binding(1) var<storage, read_write> bucket_counts: array<atomic<u32>, CODE_COUNT>;
@group(0) @binding(2) var<storage, read_write> bucket_overflow: array<atomic<u32>, CODE_COUNT>;
@group(0) @binding(3) var<storage, read_write> bucket_records: array<u32>;
@group(0) @binding(4) var<storage, read_write> bucket_meta: array<atomic<u32>, 1>;
@group(0) @binding(5) var<uniform> bucket_params: BucketParams;
@compute @workgroup_size(64)
fn bucket_pass(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= bucket_params.record_count) { return; }
    let base = i * RECORD_STRIDE;
    let code = records[base + 1u];
    if (code == 0u) { return; }
    if (code >= bucket_params.code_count) { atomicAdd(&bucket_meta[0], 1u); return; }
    let slot = atomicAdd(&bucket_counts[code], 1u);
    if (slot >= bucket_params.capacity_per_code) { atomicStore(&bucket_overflow[code], 1u); return; }
    let out = code * bucket_params.capacity_per_code * RECORD_STRIDE + slot * RECORD_STRIDE;
    bucket_records[out] = records[base];
    bucket_records[out + 1u] = code;
    bucket_records[out + 2u] = records[base + 2u];
    bucket_records[out + 3u] = records[base + 3u];
    bucket_records[out + 4u] = 0u;
}
"#
}

pub fn cpu_propose(
    reductions: &[ReductionResult; CODE_COUNT],
    rules: &[ProposalRuleGpu; CODE_COUNT],
    capacity: u32,
) -> (u32, u32, Vec<ProposalRecord>) {
    let mut all = Vec::new();
    for code in 0..CODE_COUNT {
        let r = reductions[code];
        let rule = rules[code];
        if r.flags & FLAG_RED_EMPTY != 0 {
            continue;
        }
        if r.count >= rule.min_count && r.max_score >= rule.threshold_max {
            all.push(ProposalRecord {
                source_code: code as u32,
                proposal_code: rule.proposal_code_max,
                count: r.count,
                score: r.max_score,
                flags: FLAG_RULE_MAX | (r.flags & FLAG_RED_SUM_OVERFLOW),
            });
        }
        if rule.enable_sum_rule != 0
            && r.flags & FLAG_RED_SUM_OVERFLOW == 0
            && r.count >= rule.min_count
        {
            let sum = limbs_to_i64(r.sum_hi, r.sum_lo);
            let thr = limbs_to_i64(rule.threshold_sum_hi as i32, rule.threshold_sum_lo);
            if sum >= thr {
                all.push(ProposalRecord {
                    source_code: code as u32,
                    proposal_code: rule.proposal_code_sum,
                    count: r.count,
                    score: r.max_score,
                    flags: FLAG_RULE_SUM | (r.flags & FLAG_RED_SUM_OVERFLOW),
                });
            }
        }
    }
    let attempted = all.len() as u32;
    let overflow = if attempted > capacity { 1u32 } else { 0u32 };
    let written = attempted.min(capacity) as usize;
    (attempted, overflow, all[..written].to_vec())
}

pub fn cpu_consume(
    proposal_count: u32,
    proposal_overflow: u32,
    proposals: &[ProposalRecord],
    proposal_capacity: u32,
    admitted: &[u32; ADMITTED_TABLE_SIZE],
    admitted_n: u32,
) -> ProposalSummary {
    let scan = proposal_count.min(proposal_capacity) as usize;
    let mut accepted = 0u32;
    let mut invalid = 0u32;
    let mut sum: i64 = 0;
    let mut sum_overflow = false;
    let mut max_s = 0i32;
    let mut has_any = false;
    for prop in &proposals[..scan.min(proposals.len())] {
        if is_admitted_proposal_code(prop.proposal_code, admitted, admitted_n) {
            accepted += 1;
            if !has_any {
                max_s = prop.score;
                has_any = true;
            } else {
                max_s = max_s.max(prop.score);
            }
            match sum.checked_add(i64::from(prop.score)) {
                Some(v) => sum = v,
                None => sum_overflow = true,
            }
        } else {
            invalid += 1;
        }
    }
    let ignored = proposal_count.saturating_sub(proposal_capacity);
    let mut flags = 0u32;
    if sum_overflow {
        flags |= FLAG_SUM_OVERFLOW;
    }
    if proposal_overflow != 0 {
        flags |= FLAG_INPUT_OVERFLOW;
    }
    ProposalSummary {
        accepted_count: accepted,
        ignored_count: ignored,
        invalid_count: invalid,
        summary_lo: sum as u32,
        summary_hi: (sum >> 32) as i32,
        max_score: if has_any { max_s } else { 0 },
        flags,
    }
}

fn pack_proposals(proposals: &[ProposalRecord]) -> Vec<u32> {
    let mut data = Vec::with_capacity(proposals.len() * PROP_STRIDE as usize);
    for p in proposals {
        data.push(p.source_code);
        data.push(p.proposal_code);
        data.push(p.count);
        data.push(bytemuck::cast(p.score));
        data.push(p.flags);
    }
    data
}

fn decode_summary(words: &[u32]) -> ProposalSummary {
    ProposalSummary {
        accepted_count: words[0],
        ignored_count: words[1],
        invalid_count: words[2],
        summary_lo: words[3],
        summary_hi: bytemuck::cast(words[4]),
        max_score: bytemuck::cast(words[5]),
        flags: words[6],
    }
}

pub fn summary_eq(got: ProposalSummary, exp: ProposalSummary) -> bool {
    if got.accepted_count != exp.accepted_count
        || got.ignored_count != exp.ignored_count
        || got.invalid_count != exp.invalid_count
        || got.max_score != exp.max_score
        || (got.flags & FLAG_SUM_OVERFLOW) != (exp.flags & FLAG_SUM_OVERFLOW)
        || (got.flags & FLAG_INPUT_OVERFLOW) != (exp.flags & FLAG_INPUT_OVERFLOW)
    {
        return false;
    }
    if exp.flags & FLAG_SUM_OVERFLOW == 0 {
        return limbs_to_i64(got.summary_hi, got.summary_lo)
            == limbs_to_i64(exp.summary_hi, exp.summary_lo);
    }
    true
}

fn pack_summary(summary: ProposalSummary) -> [u32; 7] {
    [
        summary.accepted_count,
        summary.ignored_count,
        summary.invalid_count,
        summary.summary_lo,
        bytemuck::cast(summary.summary_hi),
        bytemuck::cast(summary.max_score),
        summary.flags,
    ]
}

pub fn cpu_admit(summary: ProposalSummary, rules: &AdmissionRulesGpu) -> AdmissionRecord {
    let mut flags = 0u32;
    if summary.flags & FLAG_INPUT_OVERFLOW != 0 {
        flags |= FLAG_ADM_INPUT_OVF;
    }
    if summary.flags & FLAG_SUM_OVERFLOW != 0 {
        flags |= FLAG_ADM_SUM_OVF;
    }
    if summary.accepted_count < rules.min_accepted {
        flags |= FLAG_ADM_REJ_COUNT;
    }
    if summary.max_score < rules.min_max_score {
        flags |= FLAG_ADM_REJ_SCORE;
    }
    if summary.invalid_count > rules.max_invalid {
        flags |= FLAG_ADM_REJ_INVALID;
    }
    if flags
        & (FLAG_ADM_REJ_COUNT
            | FLAG_ADM_REJ_SCORE
            | FLAG_ADM_REJ_INVALID
            | FLAG_ADM_INPUT_OVF
            | FLAG_ADM_SUM_OVF)
        == 0
    {
        flags |= FLAG_ADM_ADMITTED;
    }
    AdmissionRecord {
        admission_code: rules.admission_code,
        accepted_count: summary.accepted_count,
        invalid_count: summary.invalid_count,
        summary_lo: summary.summary_lo,
        summary_hi: summary.summary_hi,
        max_score: summary.max_score,
        flags,
    }
}

fn decode_admission(words: &[u32]) -> AdmissionRecord {
    AdmissionRecord {
        admission_code: words[0],
        accepted_count: words[1],
        invalid_count: words[2],
        summary_lo: words[3],
        summary_hi: bytemuck::cast(words[4]),
        max_score: bytemuck::cast(words[5]),
        flags: words[6],
    }
}

pub fn admission_eq(got: AdmissionRecord, exp: AdmissionRecord) -> bool {
    got.admission_code == exp.admission_code
        && got.accepted_count == exp.accepted_count
        && got.invalid_count == exp.invalid_count
        && got.summary_lo == exp.summary_lo
        && got.summary_hi == exp.summary_hi
        && got.max_score == exp.max_score
        && got.flags == exp.flags
}

fn pack_reductions(reds: &[ReductionResult; CODE_COUNT]) -> Vec<u32> {
    let mut data = vec![0u32; (CODE_COUNT as u32 * RED_OUT_STRIDE) as usize];
    for code in 0..CODE_COUNT {
        let base = code * RED_OUT_STRIDE as usize;
        let r = reds[code];
        data[base] = r.count;
        data[base + 1] = r.sum_lo;
        data[base + 2] = bytemuck::cast(r.sum_hi);
        data[base + 3] = bytemuck::cast(r.min_score);
        data[base + 4] = bytemuck::cast(r.max_score);
        data[base + 5] = r.flags;
    }
    data
}

pub fn pack_act2_records(records: &[Act2EventRecord]) -> Vec<u32> {
    let mut data = Vec::with_capacity(records.len() * RECORD_STRIDE as usize);
    for r in records {
        data.push(r.source_index);
        data.push(r.event_code);
        data.push(r.state);
        data.push(bytemuck::cast(r.score_fixed));
        data.push(0);
    }
    data
}

fn storage_ro(binding: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only: true },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn storage_rw(binding: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only: false },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn uniform_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn staging_buf(device: &wgpu::Device, size: u64) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("staging"),
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn bind_entry(binding: u32, buf: &wgpu::Buffer) -> wgpu::BindGroupEntry<'_> {
    wgpu::BindGroupEntry {
        binding,
        resource: buf.as_entire_binding(),
    }
}

fn read_u32s(device: &wgpu::Device, buf: &wgpu::Buffer, count: usize) -> Vec<u32> {
    let slice = buf.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);
    let mapped = slice.get_mapped_range();
    let out: Vec<u32> = bytemuck::cast_slice(&mapped)[..count].to_vec();
    drop(mapped);
    buf.unmap();
    out
}

fn decode_proposals(words: &[u32], count: usize) -> Vec<ProposalRecord> {
    let mut out = Vec::with_capacity(count);
    for slot in 0..count {
        let base = slot * PROP_STRIDE as usize;
        out.push(ProposalRecord {
            source_code: words[base],
            proposal_code: words[base + 1],
            count: words[base + 2],
            score: bytemuck::cast(words[base + 3]),
            flags: words[base + 4],
        });
    }
    out
}

fn run_consume_gpu(
    ctx: &GpuContext,
    proposal_count: u32,
    proposal_overflow: u32,
    proposals: &[ProposalRecord],
    proposal_capacity: u32,
    admitted: &[u32; ADMITTED_TABLE_SIZE],
    admitted_n: u32,
    repeat_dispatches: u32,
) -> ConsumerOutcome {
    use wgpu::util::DeviceExt;
    let device = &ctx.device;
    let queue = &ctx.queue;
    let cparams = ConsumerParams {
        proposal_capacity,
        admitted_count: admitted_n,
        _pad: [0, 0],
    };
    let packed = if proposals.is_empty() {
        vec![0u32]
    } else {
        pack_proposals(proposals)
    };
    let prop_words = (proposal_capacity.max(1) * PROP_STRIDE) as usize;

    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("sead_act1_consume"),
        source: wgpu::ShaderSource::Wgsl(emit_consume_wgsl().into()),
    });
    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("sead_act1_consume_bgl"),
        entries: &[
            storage_ro(0),
            storage_ro(1),
            storage_ro(2),
            storage_rw(3),
            uniform_entry(4),
        ],
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("sead_act1_consume"),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("sead_act1_consume_pl"),
                bind_group_layouts: &[&bgl],
                push_constant_ranges: &[],
            }),
        ),
        module: &module,
        entry_point: "consume_pass",
        compilation_options: Default::default(),
        cache: None,
    });

    let mut meta_init = [0u32; 2];
    meta_init[0] = proposal_count;
    meta_init[1] = proposal_overflow;
    let meta_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("consume_meta"),
        contents: bytemuck::cast_slice(&meta_init),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let prop_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("consume_props"),
        contents: bytemuck::cast_slice(&packed),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let admitted_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("admitted"),
        contents: bytemuck::cast_slice(admitted),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let summary_init = vec![0u32; SUMMARY_STRIDE as usize];
    let summary_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("summary"),
        contents: bytemuck::cast_slice(&summary_init),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });
    let cparams_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("cparams"),
        contents: bytemuck::bytes_of(&cparams),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("consume_bg"),
        layout: &bgl,
        entries: &[
            bind_entry(0, &meta_buf),
            bind_entry(1, &prop_buf),
            bind_entry(2, &admitted_buf),
            bind_entry(3, &summary_buf),
            bind_entry(4, &cparams_buf),
        ],
    });

    let t0 = Instant::now();
    for _ in 0..repeat_dispatches {
        let mut enc =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("consume"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &bg, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        queue.submit(Some(enc.finish()));
    }
    let elapsed = t0.elapsed();

    let sum_staging = staging_buf(device, (SUMMARY_STRIDE * 4) as u64);
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    enc2.copy_buffer_to_buffer(
        &summary_buf,
        0,
        &sum_staging,
        0,
        (SUMMARY_STRIDE * 4) as u64,
    );
    queue.submit(Some(enc2.finish()));
    let words = read_u32s(device, &sum_staging, SUMMARY_STRIDE as usize);
    ConsumerOutcome {
        summary: decode_summary(&words),
        proposal_count,
        proposal_overflow,
        elapsed,
    }
}

fn run_admit_gpu(
    ctx: &GpuContext,
    summary: ProposalSummary,
    rules: &AdmissionRulesGpu,
    repeat_dispatches: u32,
) -> AdmitOutcome {
    use wgpu::util::DeviceExt;
    let device = &ctx.device;
    let queue = &ctx.queue;
    let summary_packed = pack_summary(summary);

    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("sead_act2_admit"),
        source: wgpu::ShaderSource::Wgsl(emit_admit_wgsl().into()),
    });
    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("sead_act2_admit_bgl"),
        entries: &[storage_ro(0), storage_rw(1), uniform_entry(2)],
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("sead_act2_admit"),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("sead_act2_admit_pl"),
                bind_group_layouts: &[&bgl],
                push_constant_ranges: &[],
            }),
        ),
        module: &module,
        entry_point: "admit_pass",
        compilation_options: Default::default(),
        cache: None,
    });

    let summary_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("summary_in"),
        contents: bytemuck::cast_slice(&summary_packed),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let admit_init = vec![0u32; ADMIT_STRIDE as usize];
    let admit_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("admission"),
        contents: bytemuck::cast_slice(&admit_init),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });
    let rules_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("adm_rules"),
        contents: bytemuck::bytes_of(rules),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("admit_bg"),
        layout: &bgl,
        entries: &[
            bind_entry(0, &summary_buf),
            bind_entry(1, &admit_buf),
            bind_entry(2, &rules_buf),
        ],
    });

    let t0 = Instant::now();
    for _ in 0..repeat_dispatches {
        let mut enc =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("admit"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &bg, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        queue.submit(Some(enc.finish()));
    }
    let elapsed = t0.elapsed();

    let staging = staging_buf(device, (ADMIT_STRIDE * 4) as u64);
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    enc2.copy_buffer_to_buffer(&admit_buf, 0, &staging, 0, (ADMIT_STRIDE * 4) as u64);
    queue.submit(Some(enc2.finish()));
    let words = read_u32s(device, &staging, ADMIT_STRIDE as usize);
    AdmitOutcome {
        admission: decode_admission(&words),
        elapsed,
    }
}

fn run_proposals_gpu(
    ctx: &GpuContext,
    reductions: &[ReductionResult; CODE_COUNT],
    rules: &[ProposalRuleGpu; CODE_COUNT],
    proposal_capacity: u32,
    repeat_dispatches: u32,
    do_readback: bool,
) -> ProposalOutcome {
    use wgpu::util::DeviceExt;
    let device = &ctx.device;
    let queue = &ctx.queue;
    let params = ProposeParams {
        code_count: CODE_COUNT as u32,
        proposal_capacity,
        _pad: [0, 0],
    };
    let red_packed = pack_reductions(reductions);

    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("sead_act0_propose"),
        source: wgpu::ShaderSource::Wgsl(emit_proposal_wgsl().into()),
    });
    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("sead_act0_bgl"),
        entries: &[
            storage_ro(0),
            storage_ro(1),
            storage_rw(2),
            storage_rw(3),
            uniform_entry(4),
        ],
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("sead_act0_propose"),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("sead_act0_pl"),
                bind_group_layouts: &[&bgl],
                push_constant_ranges: &[],
            }),
        ),
        module: &module,
        entry_point: "propose_pass",
        compilation_options: Default::default(),
        cache: None,
    });

    let red_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("reductions"),
        contents: bytemuck::cast_slice(&red_packed),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let rules_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("rules"),
        contents: bytemuck::cast_slice(rules),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let meta_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("proposal_meta"),
        contents: &[0u8; 8],
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let prop_words = (proposal_capacity.max(1) * PROP_STRIDE) as usize;
    let prop_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("proposals"),
        contents: bytemuck::cast_slice(&vec![0u32; prop_words]),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });
    let params_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("propose_params"),
        contents: bytemuck::bytes_of(&params),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("sead_act0_bg"),
        layout: &bgl,
        entries: &[
            bind_entry(0, &red_buf),
            bind_entry(1, &rules_buf),
            bind_entry(2, &meta_buf),
            bind_entry(3, &prop_buf),
            bind_entry(4, &params_buf),
        ],
    });

    let t0 = Instant::now();
    for _ in 0..repeat_dispatches {
        queue.write_buffer(&meta_buf, 0, &[0u8; 8]);
        let mut enc =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("propose"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &bg, &[]);
            pass.dispatch_workgroups(CODE_COUNT as u32, 1, 1);
        }
        queue.submit(Some(enc.finish()));
    }
    let elapsed = t0.elapsed();

    if !do_readback {
        return ProposalOutcome {
            proposal_count: 0,
            proposal_overflow: 0,
            proposals: Vec::new(),
            elapsed,
        };
    }

    let meta_staging = staging_buf(device, 8);
    let prop_staging = staging_buf(device, (prop_words * 4) as u64);
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    enc2.copy_buffer_to_buffer(&meta_buf, 0, &meta_staging, 0, 8);
    enc2.copy_buffer_to_buffer(&prop_buf, 0, &prop_staging, 0, (prop_words * 4) as u64);
    queue.submit(Some(enc2.finish()));

    let meta = read_u32s(device, &meta_staging, 2);
    let prop_words_read = read_u32s(device, &prop_staging, prop_words);
    let written = meta[0].min(proposal_capacity) as usize;
    ProposalOutcome {
        proposal_count: meta[0],
        proposal_overflow: meta[1],
        proposals: decode_proposals(&prop_words_read, written),
        elapsed,
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BucketParams {
    record_count: u32,
    capacity_per_code: u32,
    code_count: u32,
    _pad: u32,
}

pub fn run_act2_chain_gpu(
    ctx: &GpuContext,
    compact_records: &[Act2EventRecord],
    capacity_per_code: u32,
    rules: &[ProposalRuleGpu; CODE_COUNT],
    proposal_capacity: u32,
    admitted: &[u32; ADMITTED_TABLE_SIZE],
    admitted_n: u32,
    admission_rules: &AdmissionRulesGpu,
    repeat_dispatches: u32,
) -> Act2ChainOutcome {
    use wgpu::util::DeviceExt;
    let device = &ctx.device;
    let queue = &ctx.queue;
    let packed = if compact_records.is_empty() {
        vec![0u32]
    } else {
        pack_act2_records(compact_records)
    };
    let bparams = BucketParams {
        record_count: compact_records.len() as u32,
        capacity_per_code,
        code_count: CODE_COUNT as u32,
        _pad: 0,
    };
    let rparams = ReduceParams {
        capacity_per_code,
        code_count: CODE_COUNT as u32,
        _pad: [0, 0],
    };
    let pparams = ProposeParams {
        code_count: CODE_COUNT as u32,
        proposal_capacity,
        _pad: [0, 0],
    };

    let bucket_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("bucket"),
        source: wgpu::ShaderSource::Wgsl(emit_bucket_wgsl().into()),
    });
    let reduce_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("reduce"),
        source: wgpu::ShaderSource::Wgsl(emit_reduction_wgsl().into()),
    });
    let propose_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("propose"),
        source: wgpu::ShaderSource::Wgsl(emit_proposal_wgsl().into()),
    });
    let consume_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("consume"),
        source: wgpu::ShaderSource::Wgsl(emit_consume_wgsl().into()),
    });
    let admit_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("admit"),
        source: wgpu::ShaderSource::Wgsl(emit_admit_wgsl().into()),
    });

    let bucket_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("bucket_bgl"),
        entries: &[
            storage_ro(0),
            storage_rw(1),
            storage_rw(2),
            storage_rw(3),
            storage_rw(4),
            uniform_entry(5),
        ],
    });
    let reduce_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("reduce_bgl"),
        entries: &[
            storage_ro(0),
            storage_ro(1),
            storage_rw(2),
            uniform_entry(3),
        ],
    });
    let propose_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("propose_bgl"),
        entries: &[
            storage_ro(0),
            storage_ro(1),
            storage_rw(2),
            storage_rw(3),
            uniform_entry(4),
        ],
    });
    let consume_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("consume_bgl"),
        entries: &[
            storage_ro(0),
            storage_ro(1),
            storage_ro(2),
            storage_rw(3),
            uniform_entry(4),
        ],
    });
    let admit_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("admit_bgl"),
        entries: &[storage_ro(0), storage_rw(1), uniform_entry(2)],
    });

    let mk_pipe = |mod_: &wgpu::ShaderModule, bgl: &wgpu::BindGroupLayout, entry: &str| {
        device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some(entry),
            layout: Some(
                &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some(entry),
                    bind_group_layouts: &[bgl],
                    push_constant_ranges: &[],
                }),
            ),
            module: mod_,
            entry_point: entry,
            compilation_options: Default::default(),
            cache: None,
        })
    };

    let bucket_pipe = mk_pipe(&bucket_module, &bucket_bgl, "bucket_pass");
    let reduce_pipe = mk_pipe(&reduce_module, &reduce_bgl, "reduce_pass");
    let propose_pipe = mk_pipe(&propose_module, &propose_bgl, "propose_pass");
    let consume_pipe = mk_pipe(&consume_module, &consume_bgl, "consume_pass");
    let admit_pipe = mk_pipe(&admit_module, &admit_bgl, "admit_pass");
    let cparams = ConsumerParams {
        proposal_capacity,
        admitted_count: admitted_n,
        _pad: [0, 0],
    };

    let rec_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("records"),
        contents: bytemuck::cast_slice(&packed),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let counts_atomic = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("counts_atomic"),
        contents: &[0u8; CODE_COUNT * 4],
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let overflow_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("overflow"),
        contents: &[0u8; CODE_COUNT * 4],
        usage: wgpu::BufferUsages::STORAGE,
    });
    let bwords = (CODE_COUNT as u32 * capacity_per_code * RECORD_STRIDE) as usize;
    let bucket_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("buckets"),
        contents: bytemuck::cast_slice(&vec![0u32; bwords.max(1)]),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let meta_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("meta"),
        contents: &[0u8; 4],
        usage: wgpu::BufferUsages::STORAGE,
    });
    let bparams_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("bparams"),
        contents: bytemuck::bytes_of(&bparams),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let counts_read = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("counts_read"),
        contents: &[0u8; CODE_COUNT * 4],
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });
    let red_init = vec![0u32; (CODE_COUNT as u32 * RED_OUT_STRIDE) as usize];
    let red_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("red"),
        contents: bytemuck::cast_slice(&red_init),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });
    let rparams_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("rparams"),
        contents: bytemuck::bytes_of(&rparams),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let rules_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("rules"),
        contents: bytemuck::cast_slice(rules),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let prop_meta = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("prop_meta"),
        contents: &[0u8; 8],
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let prop_words = (proposal_capacity.max(1) * PROP_STRIDE) as usize;
    let prop_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("props"),
        contents: bytemuck::cast_slice(&vec![0u32; prop_words]),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });
    let pparams_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("pparams"),
        contents: bytemuck::bytes_of(&pparams),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let admitted_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("admitted"),
        contents: bytemuck::cast_slice(admitted),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let summary_init = vec![0u32; SUMMARY_STRIDE as usize];
    let summary_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("summary"),
        contents: bytemuck::cast_slice(&summary_init),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });
    let cparams_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("cparams"),
        contents: bytemuck::bytes_of(&cparams),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let adm_rules_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("adm_rules"),
        contents: bytemuck::bytes_of(admission_rules),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let admit_init = vec![0u32; ADMIT_STRIDE as usize];
    let admission_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("admission"),
        contents: bytemuck::cast_slice(&admit_init),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });

    let bg_bucket = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("bg_bucket"),
        layout: &bucket_bgl,
        entries: &[
            bind_entry(0, &rec_buf),
            bind_entry(1, &counts_atomic),
            bind_entry(2, &overflow_buf),
            bind_entry(3, &bucket_buf),
            bind_entry(4, &meta_buf),
            bind_entry(5, &bparams_buf),
        ],
    });
    let bg_reduce = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("bg_reduce"),
        layout: &reduce_bgl,
        entries: &[
            bind_entry(0, &counts_read),
            bind_entry(1, &bucket_buf),
            bind_entry(2, &red_buf),
            bind_entry(3, &rparams_buf),
        ],
    });
    let bg_propose = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("bg_propose"),
        layout: &propose_bgl,
        entries: &[
            bind_entry(0, &red_buf),
            bind_entry(1, &rules_buf),
            bind_entry(2, &prop_meta),
            bind_entry(3, &prop_buf),
            bind_entry(4, &pparams_buf),
        ],
    });
    let bg_consume = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("bg_consume"),
        layout: &consume_bgl,
        entries: &[
            bind_entry(0, &prop_meta),
            bind_entry(1, &prop_buf),
            bind_entry(2, &admitted_buf),
            bind_entry(3, &summary_buf),
            bind_entry(4, &cparams_buf),
        ],
    });
    let bg_admit = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("bg_admit"),
        layout: &admit_bgl,
        entries: &[
            bind_entry(0, &summary_buf),
            bind_entry(1, &admission_buf),
            bind_entry(2, &adm_rules_buf),
        ],
    });

    let t0 = Instant::now();
    for _ in 0..repeat_dispatches {
        queue.write_buffer(&counts_atomic, 0, &[0u8; CODE_COUNT * 4]);
        queue.write_buffer(&prop_meta, 0, &[0u8; 8]);
        let mut enc =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("bucket"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&bucket_pipe);
            pass.set_bind_group(0, &bg_bucket, &[]);
            pass.dispatch_workgroups(bparams.record_count.div_ceil(64), 1, 1);
        }
        enc.copy_buffer_to_buffer(&counts_atomic, 0, &counts_read, 0, (CODE_COUNT * 4) as u64);
        {
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("reduce"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&reduce_pipe);
            pass.set_bind_group(0, &bg_reduce, &[]);
            pass.dispatch_workgroups(CODE_COUNT as u32, 1, 1);
        }
        {
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("propose"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&propose_pipe);
            pass.set_bind_group(0, &bg_propose, &[]);
            pass.dispatch_workgroups(CODE_COUNT as u32, 1, 1);
        }
        {
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("consume"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&consume_pipe);
            pass.set_bind_group(0, &bg_consume, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        {
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("admit"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&admit_pipe);
            pass.set_bind_group(0, &bg_admit, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        queue.submit(Some(enc.finish()));
    }
    let elapsed = t0.elapsed();

    let red_staging = staging_buf(device, (CODE_COUNT as u32 * RED_OUT_STRIDE * 4) as u64);
    let meta_staging = staging_buf(device, 8);
    let sum_staging = staging_buf(device, (SUMMARY_STRIDE * 4) as u64);
    let admit_staging = staging_buf(device, (ADMIT_STRIDE * 4) as u64);
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    enc2.copy_buffer_to_buffer(
        &red_buf,
        0,
        &red_staging,
        0,
        (CODE_COUNT as u32 * RED_OUT_STRIDE * 4) as u64,
    );
    enc2.copy_buffer_to_buffer(&prop_meta, 0, &meta_staging, 0, 8);
    enc2.copy_buffer_to_buffer(
        &summary_buf,
        0,
        &sum_staging,
        0,
        (SUMMARY_STRIDE * 4) as u64,
    );
    enc2.copy_buffer_to_buffer(
        &admission_buf,
        0,
        &admit_staging,
        0,
        (ADMIT_STRIDE * 4) as u64,
    );
    queue.submit(Some(enc2.finish()));

    let red_vec = read_u32s(device, &red_staging, CODE_COUNT * RED_OUT_STRIDE as usize);
    let meta = read_u32s(device, &meta_staging, 2);
    let sum_vec = read_u32s(device, &sum_staging, SUMMARY_STRIDE as usize);
    let admit_vec = read_u32s(device, &admit_staging, ADMIT_STRIDE as usize);
    let mut reductions = [ReductionResult {
        count: 0,
        sum_lo: 0,
        sum_hi: 0,
        min_score: 0,
        max_score: 0,
        flags: 0,
    }; CODE_COUNT];
    for code in 0..CODE_COUNT {
        let base = code * RED_OUT_STRIDE as usize;
        reductions[code] = ReductionResult {
            count: red_vec[base],
            sum_lo: red_vec[base + 1],
            sum_hi: bytemuck::cast(red_vec[base + 2]),
            min_score: bytemuck::cast(red_vec[base + 3]),
            max_score: bytemuck::cast(red_vec[base + 4]),
            flags: red_vec[base + 5],
        };
    }
    let summary = decode_summary(&sum_vec);
    Act2ChainOutcome {
        reductions,
        proposal_count: meta[0],
        proposal_overflow: meta[1],
        summary,
        admission: decode_admission(&admit_vec),
        elapsed,
    }
}

pub fn cpu_reduce(records: &[Act2EventRecord]) -> ReductionResult {
    if records.is_empty() {
        return ReductionResult {
            count: 0,
            sum_lo: 0,
            sum_hi: 0,
            min_score: 0,
            max_score: 0,
            flags: FLAG_RED_EMPTY,
        };
    }
    let mut sum: i64 = 0;
    let mut sum_overflow = false;
    let mut min_s = records[0].score_fixed;
    let mut max_s = records[0].score_fixed;
    for rec in records {
        min_s = min_s.min(rec.score_fixed);
        max_s = max_s.max(rec.score_fixed);
        match sum.checked_add(i64::from(rec.score_fixed)) {
            Some(v) => sum = v,
            None => sum_overflow = true,
        }
    }
    let mut flags = 0u32;
    if sum_overflow {
        flags |= FLAG_RED_SUM_OVERFLOW;
    }
    ReductionResult {
        count: records.len() as u32,
        sum_lo: sum as u32,
        sum_hi: (sum >> 32) as i32,
        min_score: min_s,
        max_score: max_s,
        flags,
    }
}

pub fn cpu_bucket_from_compact(
    records: &[Act2EventRecord],
    capacity: u32,
) -> (Vec<Vec<Act2EventRecord>>, [u32; CODE_COUNT]) {
    let mut buckets: [Vec<Act2EventRecord>; CODE_COUNT] = std::array::from_fn(|_| Vec::new());
    let mut counts = [0u32; CODE_COUNT];
    for rec in records {
        if rec.event_code == 0 || rec.event_code >= CODE_COUNT as u32 {
            continue;
        }
        let code = rec.event_code as usize;
        counts[code] += 1;
        if buckets[code].len() as u32 >= capacity {
            continue;
        }
        buckets[code].push(*rec);
    }
    (buckets.to_vec(), counts)
}

pub fn reductions_from_buckets(
    buckets: &[Vec<Act2EventRecord>],
    counts: [u32; CODE_COUNT],
    cap: u32,
) -> [ReductionResult; CODE_COUNT] {
    let mut out = [ReductionResult {
        count: 0,
        sum_lo: 0,
        sum_hi: 0,
        min_score: 0,
        max_score: 0,
        flags: FLAG_RED_EMPTY,
    }; CODE_COUNT];
    for code in 0..CODE_COUNT {
        let scan = counts[code].min(cap) as usize;
        let slice = &buckets[code][..scan.min(buckets[code].len())];
        out[code] = cpu_reduce(slice);
    }
    out
}

pub fn rules_for_smoke() -> [ProposalRuleGpu; CODE_COUNT] {
    let mut rules = default_rules();
    rules[1].threshold_sum_lo = 100_000;
    rules[1].threshold_sum_hi = 0;
    rules[2].threshold_max = 100;
    rules
}

pub fn verify_act2_chain_admission(
    outcome: &Act2ChainOutcome,
    compact: &[Act2EventRecord],
    cap: u32,
    rules: &[ProposalRuleGpu; CODE_COUNT],
    prop_cap: u32,
    admitted: &[u32; ADMITTED_TABLE_SIZE],
    admitted_n: u32,
    admission_rules: &AdmissionRulesGpu,
) {
    let (buckets, counts) = cpu_bucket_from_compact(compact, cap);
    let exp_reds = reductions_from_buckets(&buckets, counts, cap);
    let (exp_count, exp_ovf, exp_props) = cpu_propose(&exp_reds, rules, prop_cap);
    let exp_summary = cpu_consume(
        exp_count, exp_ovf, &exp_props, prop_cap, admitted, admitted_n,
    );
    let exp_admission = cpu_admit(exp_summary, admission_rules);
    assert_eq!(outcome.proposal_count, exp_count);
    assert_eq!(outcome.proposal_overflow, exp_ovf);
    assert!(summary_eq(outcome.summary, exp_summary));
    assert!(admission_eq(outcome.admission, exp_admission));
}
