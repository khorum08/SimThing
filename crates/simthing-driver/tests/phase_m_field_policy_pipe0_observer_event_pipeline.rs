//! FIELD_POLICY-PIPE-0 — integrated GPU observer threshold event + compaction pipeline (Tier-2, test-only).
//!
//! Pass A: OBS-4-style threshold event rows from Q16.16 observer input.
//! Pass B: EVENT-0-style atomic compaction into compact event records.
//! No CPU filtering between passes; CPU oracle for verification only.

use std::sync::Mutex;
use std::time::Instant;

use simthing_gpu::GpuContext;
use simthing_spec::{
    fnv1a64_hex, is_field_policy_pipe0_observer_event_pipeline_descriptor,
    landed_jit_kernel_descriptors, validate_kernel_descriptor_admission,
    EventCompactionMembershipAuthority, EventCompactionOrderAuthority, MappingExecutionProfile,
    FIELD_POLICY_PIPE0_DESCRIPTOR_ID, FIELD_POLICY_PIPE0_LAYER_COUNT, MAG2_Q16_SCALE,
    SQRT_F_ARTIFACT_HASH,
};

use simthing_spec::OutputAuthority;

static GPU_MUTEX: Mutex<()> = Mutex::new(());
const SQRT_CR_F_WGSL: &str = include_str!("wgsl/sqrt_cr_f_candidate.wgsl");

const LAYER_COUNT: usize = FIELD_POLICY_PIPE0_LAYER_COUNT as usize;
const FIELDS_PER_LAYER: u32 = 3;
const INPUT_STRIDE: u32 = LAYER_COUNT as u32 * FIELDS_PER_LAYER + 4;
const EVENT_ROW_STRIDE: u32 = 5;
const RECORD_STRIDE: u32 = 5;
const BIAS_OFF: u32 = LAYER_COUNT as u32 * FIELDS_PER_LAYER;
const THRESHOLD_OFF: u32 = BIAS_OFF + 1;
const HYSTERESIS_OFF: u32 = BIAS_OFF + 2;
const PRIOR_STATE_OFF: u32 = BIAS_OFF + 3;
const Q16_SCALE_F: f32 = MAG2_Q16_SCALE as f32;
const ORDERING_CLASS: &str = "UnspecifiedAtomicOrder";

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
    "order",
    "route",
];

const FORBIDDEN_EXACT_TERMS: &[&str] = &["f64", "F64RoundDown", "SHADER_F64", "sqrt_cr_c"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LayerInput {
    gx: i32,
    gy: i32,
    w: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ObserverRow {
    layers: [LayerInput; LAYER_COUNT],
    bias: i32,
    threshold: i32,
    hysteresis: i32,
    prior_state: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EventRow {
    source_index: u32,
    event_code: u32,
    state: u32,
    score_fixed: i32,
    flags: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
struct EventRecord {
    source_index: u32,
    event_code: u32,
    state: u32,
    score_fixed: i32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct PipelineParams {
    row_count: u32,
    capacity: u32,
    _pad: [u32; 2],
}

struct PipelineOutcome {
    event_rows: Vec<EventRow>,
    event_count: u32,
    overflow: u32,
    records: Vec<EventRecord>,
    elapsed: std::time::Duration,
    dispatch_count: u32,
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

fn cpu_score_fixed(row: &ObserverRow) -> i32 {
    let mut score = row.bias;
    for layer in &row.layers {
        let mag_fixed = mag_bits_to_q16_fixed(cpu_mag_bits(layer.gx, layer.gy));
        score = score.wrapping_add(q16_mul(layer.w, mag_fixed));
    }
    score
}

fn cpu_threshold_state_event(row: &ObserverRow) -> (u32, u32, i32) {
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

fn emit_pipeline_wgsl() -> String {
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

fn pack_observer_inputs(rows: &[ObserverRow]) -> Vec<u32> {
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

fn cpu_event_rows(rows: &[ObserverRow]) -> Vec<EventRow> {
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

fn expected_records(event_rows: &[EventRow]) -> Vec<EventRecord> {
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

fn membership_exact(expected: &[EventRecord], got: &[EventRecord]) -> bool {
    if expected.len() != got.len() {
        return false;
    }
    let mut a = expected.to_vec();
    let mut b = got.to_vec();
    sort_records(&mut a);
    sort_records(&mut b);
    a == b
}

fn run_pipeline(
    ctx: &GpuContext,
    rows: &[ObserverRow],
    capacity: u32,
    repeat_cycles: u32,
    do_readback: bool,
) -> PipelineOutcome {
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
        label: Some("field_policy_pipe0_shader"),
        source: wgpu::ShaderSource::Wgsl(wgsl.into()),
    });

    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("field_policy_pipe0_bgl"),
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
        label: Some("field_policy_pipe0_pl"),
        bind_group_layouts: &[&bgl],
        push_constant_ranges: &[],
    });

    let threshold_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("field_policy_pipe0_threshold"),
        layout: Some(&pl),
        module: &module,
        entry_point: "threshold_pass",
        compilation_options: Default::default(),
        cache: None,
    });
    let compact_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("field_policy_pipe0_compact"),
        layout: Some(&pl),
        module: &module,
        entry_point: "compact_pass",
        compilation_options: Default::default(),
        cache: None,
    });

    let observer_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("field_policy_pipe0_observer"),
        contents: bytemuck::cast_slice(&observer_data),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let event_row_words = (n * EVENT_ROW_STRIDE) as usize;
    let event_rows_init = vec![0u32; event_row_words];
    let event_rows_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("field_policy_pipe0_event_rows"),
        contents: bytemuck::cast_slice(&event_rows_init),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let counters_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("field_policy_pipe0_counters"),
        contents: &[0u8; 8],
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let record_words = (capacity.max(1) * RECORD_STRIDE) as usize;
    let records_init = vec![0u32; record_words];
    let records_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("field_policy_pipe0_records"),
        contents: bytemuck::cast_slice(&records_init),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let params_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("field_policy_pipe0_params"),
        contents: bytemuck::bytes_of(&params),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("field_policy_pipe0_bg"),
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
            label: Some("field_policy_pipe0_enc"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("field_policy_pipe0_threshold_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&threshold_pipeline);
            pass.set_bind_group(0, &bg, &[]);
            pass.dispatch_workgroups(wg, 1, 1);
        }
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("field_policy_pipe0_compact_pass"),
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
        return PipelineOutcome {
            event_rows: Vec::new(),
            event_count: 0,
            overflow: 0,
            records: Vec::new(),
            elapsed,
            dispatch_count: repeat_cycles * 2,
        };
    }

    let counters_staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("field_policy_pipe0_counters_readback"),
        size: 8,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let event_rows_bytes = (event_row_words * 4) as u64;
    let event_rows_staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("field_policy_pipe0_event_rows_readback"),
        size: event_rows_bytes,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let records_bytes = (record_words * 4) as u64;
    let records_staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("field_policy_pipe0_records_readback"),
        size: records_bytes.max(4),
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("field_policy_pipe0_readback_enc"),
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

    PipelineOutcome {
        event_rows,
        event_count: meta[0],
        overflow: meta[1],
        records,
        elapsed,
        dispatch_count: repeat_cycles * 2,
    }
}

fn layer_input(gx: f32, gy: f32, w: f32) -> LayerInput {
    LayerInput {
        gx: f32_to_q16(gx),
        gy: f32_to_q16(gy),
        w: f32_to_q16(w),
    }
}

fn zero_layers() -> [LayerInput; LAYER_COUNT] {
    [layer_input(0.0, 0.0, 1.0); LAYER_COUNT]
}

fn edge_pipeline_cases() -> Vec<(Vec<ObserverRow>, u32, &'static str)> {
    vec![
        (
            vec![ObserverRow {
                layers: zero_layers(),
                bias: f32_to_q16(0.5),
                threshold: f32_to_q16(1.0),
                hysteresis: 0,
                prior_state: 0,
            }],
            8,
            "no_events",
        ),
        (
            vec![ObserverRow {
                layers: [layer_input(2.0, 0.0, 2.0); LAYER_COUNT],
                bias: f32_to_q16(1.0),
                threshold: f32_to_q16(1.0),
                hysteresis: 0,
                prior_state: 0,
            }],
            8,
            "single_event",
        ),
        (
            (0..6)
                .map(|i| ObserverRow {
                    layers: [layer_input(2.0, 0.0, 2.0); LAYER_COUNT],
                    bias: f32_to_q16(1.0),
                    threshold: f32_to_q16(1.0),
                    hysteresis: 0,
                    prior_state: i % 2,
                })
                .collect(),
            8,
            "all_events",
        ),
        (
            vec![
                ObserverRow {
                    layers: [layer_input(2.0, 0.0, 2.0); LAYER_COUNT],
                    bias: f32_to_q16(1.0),
                    threshold: f32_to_q16(1.0),
                    hysteresis: 0,
                    prior_state: 0,
                },
                ObserverRow {
                    layers: zero_layers(),
                    bias: f32_to_q16(0.5),
                    threshold: f32_to_q16(1.0),
                    hysteresis: 0,
                    prior_state: 1,
                },
                ObserverRow {
                    layers: [layer_input(0.5, 0.0, 1.0); LAYER_COUNT],
                    bias: 0,
                    threshold: f32_to_q16(2.0),
                    hysteresis: f32_to_q16(0.5),
                    prior_state: 1,
                },
            ],
            8,
            "up_down_hysteresis_hold",
        ),
        (
            (0..4)
                .map(|_i| ObserverRow {
                    layers: [layer_input(2.0, 0.0, 2.0); LAYER_COUNT],
                    bias: f32_to_q16(1.0),
                    threshold: f32_to_q16(1.0),
                    hysteresis: 0,
                    prior_state: 0,
                })
                .collect(),
            4,
            "capacity_exact_full",
        ),
        (
            (0..6)
                .map(|_i| ObserverRow {
                    layers: [layer_input(2.0, 0.0, 2.0); LAYER_COUNT],
                    bias: f32_to_q16(1.0),
                    threshold: f32_to_q16(1.0),
                    hysteresis: 0,
                    prior_state: 0,
                })
                .collect(),
            4,
            "capacity_overflow",
        ),
        (
            vec![ObserverRow {
                layers: [layer_input(2.0, 0.0, 2.0); LAYER_COUNT],
                bias: f32_to_q16(1.0),
                threshold: f32_to_q16(1.0),
                hysteresis: 0,
                prior_state: 0,
            }],
            0,
            "zero_capacity",
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

fn dense_observer_rows() -> Vec<ObserverRow> {
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
                        out.push(ObserverRow {
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

fn mobile_observer_rows(count: usize) -> Vec<ObserverRow> {
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
        out.push(ObserverRow {
            layers,
            bias: f32_to_q16(bias),
            threshold: f32_to_q16(thr),
            hysteresis: f32_to_q16(hys),
            prior_state: (idx % 2) as u32,
        });
    }
    out
}

fn verify_threshold_rows(got: &[EventRow], rows: &[ObserverRow]) -> (usize, usize, usize) {
    let expected = cpu_event_rows(rows);
    let mut score_exact = 0usize;
    let mut state_exact = 0usize;
    let mut event_exact = 0usize;
    for (g, e) in got.iter().zip(expected.iter()) {
        if g.score_fixed == e.score_fixed {
            score_exact += 1;
        }
        if g.state == e.state {
            state_exact += 1;
        }
        if g.event_code == e.event_code {
            event_exact += 1;
        }
    }
    (score_exact, state_exact, event_exact)
}

#[test]
fn field_policy_pipe0_wgsl_semantic_free() {
    let wgsl = emit_pipeline_wgsl();
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
    assert!(wgsl.contains("threshold_pass"));
    assert!(wgsl.contains("compact_pass"));
    assert!(wgsl.contains("atomicAdd"));
    assert!(!wgsl.contains("planner"));
    assert!(!wgsl.contains("urgency"));
    assert!(!wgsl.contains("commitment"));
    assert!(!wgsl.contains("scheduler"));
    assert!(!wgsl.contains("cache"));
    assert_eq!(fnv1a64_hex(SQRT_CR_F_WGSL), SQRT_F_ARTIFACT_HASH);
    println!(
        "field_policy_pipe0_wgsl: semantic_free=true F_hash={SQRT_F_ARTIFACT_HASH} ordering={ORDERING_CLASS}"
    );
}

#[test]
fn field_policy_pipe0_edge_rows() {
    with_gpu(|ctx| {
        for (rows, capacity, label) in edge_pipeline_cases() {
            let outcome = run_pipeline(ctx, &rows, capacity, 1, true);
            let expected_rows = cpu_event_rows(&rows);
            let expected = expected_records(&expected_rows);
            let expected_count = expected.len() as u32;
            let (score_exact, state_exact, event_exact) =
                verify_threshold_rows(&outcome.event_rows, &rows);
            let membership = if capacity >= expected_count {
                membership_exact(&expected, &outcome.records)
            } else {
                outcome.overflow == 1
            };
            println!(
                "field_policy_pipe0_edge[{label}]: rows={} capacity={capacity} event_count={} overflow={} score_exact={score_exact}/{} state_exact={state_exact}/{} event_exact={event_exact}/{} membership_ok={membership} ordering={ORDERING_CLASS}",
                rows.len(),
                outcome.event_count,
                outcome.overflow,
                rows.len(),
                rows.len(),
                rows.len(),
            );
            assert_eq!(score_exact, rows.len());
            assert_eq!(state_exact, rows.len());
            assert_eq!(event_exact, rows.len());
            assert_eq!(outcome.event_count, expected_count);
            if capacity >= expected_count {
                assert_eq!(outcome.overflow, 0);
                assert!(membership_exact(&expected, &outcome.records));
            } else if capacity == 0 && expected_count > 0 {
                assert_eq!(outcome.overflow, 1);
                assert!(outcome.records.is_empty());
            } else if expected_count > capacity {
                assert_eq!(outcome.overflow, 1);
            }
        }
    });
}

#[test]
fn field_policy_pipe0_dense_corpus() {
    with_gpu(|ctx| {
        let rows = dense_observer_rows();
        let capacity = rows.len() as u32;
        let outcome = run_pipeline(ctx, &rows, capacity, 1, true);
        let expected_rows = cpu_event_rows(&rows);
        let expected = expected_records(&expected_rows);
        let (score_exact, state_exact, event_exact) =
            verify_threshold_rows(&outcome.event_rows, &rows);
        println!(
            "field_policy_pipe0_dense: rows={} event_count={} overflow={} score_exact={score_exact} state_exact={state_exact} event_exact={event_exact} membership={}/{} ordering={ORDERING_CLASS}",
            rows.len(),
            outcome.event_count,
            outcome.overflow,
            expected.len(),
            outcome.records.len(),
        );
        assert_eq!(score_exact, rows.len());
        assert_eq!(state_exact, rows.len());
        assert_eq!(event_exact, rows.len());
        assert_eq!(outcome.event_count, expected.len() as u32);
        assert_eq!(outcome.overflow, 0);
        assert!(membership_exact(&expected, &outcome.records));
    });
}

#[test]
fn field_policy_pipe0_perf_34k_integrated_pipeline() {
    with_gpu(|ctx| {
        const N: usize = 34_000;
        let rows = mobile_observer_rows(N);
        let capacity = N as u32;
        let outcome = run_pipeline(ctx, &rows, capacity, 1, true);
        let expected_rows = cpu_event_rows(&rows);
        let expected = expected_records(&expected_rows);
        let sample = &outcome.event_rows[..512.min(outcome.event_rows.len())];
        let sample_rows = &rows[..512.min(rows.len())];
        let (score_exact, state_exact, event_exact) = verify_threshold_rows(sample, sample_rows);
        let elapsed_ms = outcome.elapsed.as_secs_f64() * 1000.0;
        let per_row_us = elapsed_ms * 1000.0 / N as f64;
        println!(
            "field_policy_pipe0_34k_integrated: rows={N} dispatches={} readback=true elapsed_ms={elapsed_ms:.3} per_row_us={per_row_us:.4} event_count={} overflow={} sample_score={score_exact}/512 sample_state={state_exact}/512 sample_event={event_exact}/512 membership={}/{} ordering={ORDERING_CLASS}",
            outcome.dispatch_count,
            outcome.event_count,
            outcome.overflow,
            expected.len(),
            outcome.records.len(),
        );
        assert_eq!(outcome.dispatch_count, 2);
        assert_eq!(score_exact, sample.len());
        assert_eq!(state_exact, sample.len());
        assert_eq!(event_exact, sample.len());
        assert_eq!(outcome.event_count, expected.len() as u32);
        assert_eq!(outcome.overflow, 0);
        assert!(membership_exact(&expected, &outcome.records));
    });
}

#[test]
fn field_policy_pipe0_perf_34k_capacity_variants() {
    with_gpu(|ctx| {
        const N: usize = 34_000;
        let rows = mobile_observer_rows(N);
        let expected_rows = cpu_event_rows(&rows);
        let expected = expected_records(&expected_rows);
        let event_count = expected.len() as u32;
        let capacities = [
            (0u32, "capacity_0"),
            (event_count / 2, "capacity_half"),
            (event_count, "capacity_exact"),
            (N as u32, "capacity_34k"),
        ];
        for (capacity, label) in capacities {
            let outcome = run_pipeline(ctx, &rows, capacity, 1, true);
            let membership = if capacity >= event_count {
                membership_exact(&expected, &outcome.records)
            } else {
                outcome.overflow == 1
            };
            println!(
                "field_policy_pipe0_34k_cap[{label}]: capacity={capacity} event_count={} written={} overflow={} membership_ok={membership}",
                outcome.event_count,
                outcome.records.len(),
                outcome.overflow,
            );
            assert_eq!(outcome.event_count, event_count);
            if capacity >= event_count {
                assert_eq!(outcome.overflow, 0);
                assert!(membership_exact(&expected, &outcome.records));
            } else if event_count > 0 {
                assert_eq!(outcome.overflow, 1);
            }
        }
    });
}

#[test]
fn field_policy_pipe0_perf_34k_warm_repeated_dispatch() {
    with_gpu(|ctx| {
        const N: usize = 34_000;
        const REPEATS: u32 = 32;
        let rows = mobile_observer_rows(N);
        let capacity = N as u32;
        let outcome = run_pipeline(ctx, &rows, capacity, REPEATS, true);
        let expected_rows = cpu_event_rows(&rows);
        let expected = expected_records(&expected_rows);
        let total_ms = outcome.elapsed.as_secs_f64() * 1000.0;
        let per_pipeline_ms = total_ms / REPEATS as f64;
        let per_row_us = per_pipeline_ms * 1000.0 / N as f64;
        println!(
            "field_policy_pipe0_34k_warm: repeats={REPEATS} total_ms={total_ms:.3} per_pipeline_ms={per_pipeline_ms:.4} per_row_us={per_row_us:.4} dispatches={} event_count={} overflow={} membership={}/{} ordering={ORDERING_CLASS}",
            outcome.dispatch_count,
            outcome.event_count,
            outcome.overflow,
            expected.len(),
            outcome.records.len(),
        );
        assert_eq!(outcome.dispatch_count, REPEATS * 2);
        assert_eq!(outcome.event_count, expected.len() as u32);
        assert_eq!(outcome.overflow, 0);
        assert!(membership_exact(&expected, &outcome.records));
    });
}

#[test]
fn field_policy_pipe0_no_default_runtime_wiring() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    let desc = landed_jit_kernel_descriptors()
        .into_iter()
        .find(|d| d.id == FIELD_POLICY_PIPE0_DESCRIPTOR_ID)
        .expect("pipe0 descriptor registered");
    assert!(desc.default_off);
    assert!(!desc.production_wiring);
    assert!(is_field_policy_pipe0_observer_event_pipeline_descriptor(
        &desc
    ));
    validate_kernel_descriptor_admission(&desc).expect("pipe0 admits");
    for out in &desc.writes {
        assert_eq!(out.authority, OutputAuthority::ExactAuthoritative);
    }
    let _ = EventCompactionMembershipAuthority::ExactAuthoritativeUnordered;
    let _ = EventCompactionOrderAuthority::UnspecifiedAtomicOrder;
    println!(
        "field_policy_pipe0_wiring: default_off=true production_wiring=false descriptor={FIELD_POLICY_PIPE0_DESCRIPTOR_ID}"
    );
}
