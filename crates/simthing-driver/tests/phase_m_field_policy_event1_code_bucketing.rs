//! FIELD_POLICY-EVENT-1 — GPU-resident event-code bucketing from compact event records (Tier-2, test-only).
//!
//! Consumes compact unordered event records; emits per-code counts, bounded buckets, overflow flags.
//! No CPU filtering between GPU passes; CPU oracle for verification only.

use std::sync::Mutex;
use std::time::Instant;

use simthing_gpu::GpuContext;
use simthing_spec::{
    fnv1a64_hex, is_field_policy_event1_code_bucketing_descriptor, landed_jit_kernel_descriptors,
    validate_kernel_descriptor_admission, EventCodeBucketMembershipAuthority,
    EventCodeBucketOrderAuthority, MappingExecutionProfile, FIELD_POLICY_EVENT1_CODE_COUNT,
    FIELD_POLICY_EVENT1_DESCRIPTOR_ID, FIELD_POLICY_PIPE0_LAYER_COUNT, MAG2_Q16_SCALE,
    SQRT_F_ARTIFACT_HASH,
};

use simthing_spec::OutputAuthority;

static GPU_MUTEX: Mutex<()> = Mutex::new(());
const SQRT_CR_F_WGSL: &str = include_str!("wgsl/sqrt_cr_f_candidate.wgsl");

const CODE_COUNT: usize = FIELD_POLICY_EVENT1_CODE_COUNT as usize;
const RECORD_STRIDE: u32 = 5;
const LAYER_COUNT: usize = FIELD_POLICY_PIPE0_LAYER_COUNT as usize;
const FIELDS_PER_LAYER: u32 = 3;
const OBS_INPUT_STRIDE: u32 = LAYER_COUNT as u32 * FIELDS_PER_LAYER + 4;
const EVENT_ROW_STRIDE: u32 = 5;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
struct EventRecord {
    source_index: u32,
    event_code: u32,
    state: u32,
    score_fixed: i32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct BucketParams {
    record_count: u32,
    capacity_per_code: u32,
    code_count: u32,
    _pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct IntegratedParams {
    row_count: u32,
    compact_capacity: u32,
    capacity_per_code: u32,
    code_count: u32,
}

struct BucketOutcome {
    bucket_counts: [u32; CODE_COUNT],
    bucket_overflow: [u32; CODE_COUNT],
    buckets: [Vec<EventRecord>; CODE_COUNT],
    invalid_code_count: u32,
    elapsed: std::time::Duration,
}

struct IntegratedOutcome {
    event_count: u32,
    compact_overflow: u32,
    compact_records: Vec<EventRecord>,
    bucket: BucketOutcome,
    dispatch_count: u32,
}

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let ctx = GpuContext::new_blocking().expect("GPU required");
    f(&ctx);
}

fn emit_bucketing_wgsl() -> &'static str {
    r#"
const RECORD_STRIDE: u32 = 5u;
const CODE_COUNT: u32 = 4u;

struct Params {
    record_count: u32,
    capacity_per_code: u32,
    code_count: u32,
    _pad: u32,
}

@group(0) @binding(0) var<storage, read> records: array<u32>;
@group(0) @binding(1) var<storage, read_write> bucket_counts: array<atomic<u32>, CODE_COUNT>;
@group(0) @binding(2) var<storage, read_write> bucket_overflow: array<atomic<u32>, CODE_COUNT>;
@group(0) @binding(3) var<storage, read_write> bucket_records: array<u32>;
@group(0) @binding(4) var<storage, read_write> bucket_meta: array<atomic<u32>, 1>;
@group(0) @binding(5) var<uniform> params: Params;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= params.record_count) { return; }
    let base = i * RECORD_STRIDE;
    let code = records[base + 1u];
    if (code == 0u) { return; }
    if (code >= params.code_count) {
        atomicAdd(&bucket_meta[0], 1u);
        return;
    }
    let slot = atomicAdd(&bucket_counts[code], 1u);
    if (slot >= params.capacity_per_code) {
        atomicStore(&bucket_overflow[code], 1u);
        return;
    }
    let out_base = code * params.capacity_per_code * RECORD_STRIDE + slot * RECORD_STRIDE;
    bucket_records[out_base] = records[base];
    bucket_records[out_base + 1u] = code;
    bucket_records[out_base + 2u] = records[base + 2u];
    bucket_records[out_base + 3u] = records[base + 3u];
    bucket_records[out_base + 4u] = 0u;
}
"#
}

fn pack_records(records: &[EventRecord]) -> Vec<u32> {
    let mut data = Vec::with_capacity(records.len() * RECORD_STRIDE as usize);
    for rec in records {
        data.push(rec.source_index);
        data.push(rec.event_code);
        data.push(rec.state);
        data.push(bytemuck::cast(rec.score_fixed));
        data.push(0);
    }
    data
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

fn cpu_bucket(
    records: &[EventRecord],
    capacity_per_code: u32,
) -> (
    [u32; CODE_COUNT],
    [u32; CODE_COUNT],
    [Vec<EventRecord>; CODE_COUNT],
    u32,
) {
    let mut counts = [0u32; CODE_COUNT];
    let mut overflow = [0u32; CODE_COUNT];
    let mut buckets: [Vec<EventRecord>; CODE_COUNT] = std::array::from_fn(|_| Vec::new());
    let mut invalid = 0u32;
    for rec in records {
        if rec.event_code == 0 {
            continue;
        }
        if rec.event_code >= CODE_COUNT as u32 {
            invalid += 1;
            continue;
        }
        let code = rec.event_code as usize;
        counts[code] += 1;
        if buckets[code].len() as u32 >= capacity_per_code {
            overflow[code] = 1;
        } else {
            buckets[code].push(*rec);
        }
    }
    (counts, overflow, buckets, invalid)
}

fn run_bucketing(
    ctx: &GpuContext,
    records: &[EventRecord],
    capacity_per_code: u32,
    repeat_dispatches: u32,
    do_readback: bool,
) -> BucketOutcome {
    use wgpu::util::DeviceExt;
    let device = &ctx.device;
    let queue = &ctx.queue;
    let record_count = records.len() as u32;
    let params = BucketParams {
        record_count,
        capacity_per_code,
        code_count: CODE_COUNT as u32,
        _pad: 0,
    };
    let packed = if records.is_empty() {
        vec![0u32]
    } else {
        pack_records(records)
    };
    let bucket_words = (CODE_COUNT as u32 * capacity_per_code.max(1) * RECORD_STRIDE) as usize;

    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("field_policy_event1_bucketing"),
        source: wgpu::ShaderSource::Wgsl(emit_bucketing_wgsl().into()),
    });

    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("field_policy_event1_bgl"),
        entries: &[
            storage_entry(0, true),
            storage_entry(1, false),
            storage_entry(2, false),
            storage_entry(3, false),
            storage_entry(4, false),
            uniform_entry(5),
        ],
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("field_policy_event1_pipeline"),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("field_policy_event1_pl"),
                bind_group_layouts: &[&bgl],
                push_constant_ranges: &[],
            }),
        ),
        module: &module,
        entry_point: "main",
        compilation_options: Default::default(),
        cache: None,
    });

    let records_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("field_policy_event1_records"),
        contents: bytemuck::cast_slice(&packed),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let counts_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("field_policy_event1_counts"),
        contents: &[0u8; CODE_COUNT * 4],
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let overflow_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("field_policy_event1_overflow"),
        contents: &[0u8; CODE_COUNT * 4],
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let bucket_init = vec![0u32; bucket_words];
    let bucket_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("field_policy_event1_buckets"),
        contents: bytemuck::cast_slice(&bucket_init),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let meta_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("field_policy_event1_meta"),
        contents: &[0u8; 4],
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let params_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("field_policy_event1_params"),
        contents: bytemuck::bytes_of(&params),
        usage: wgpu::BufferUsages::UNIFORM,
    });

    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("field_policy_event1_bg"),
        layout: &bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: records_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: counts_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: overflow_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: bucket_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 4,
                resource: meta_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 5,
                resource: params_buf.as_entire_binding(),
            },
        ],
    });

    let wg = record_count.div_ceil(64);
    let t0 = Instant::now();
    for _ in 0..repeat_dispatches {
        queue.write_buffer(&counts_buf, 0, &[0u8; CODE_COUNT * 4]);
        queue.write_buffer(&overflow_buf, 0, &[0u8; CODE_COUNT * 4]);
        queue.write_buffer(&meta_buf, 0, &[0u8; 4]);
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("field_policy_event1_enc"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("field_policy_event1_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &bg, &[]);
            pass.dispatch_workgroups(wg, 1, 1);
        }
        queue.submit(Some(encoder.finish()));
    }
    let elapsed = t0.elapsed();

    if !do_readback {
        return BucketOutcome {
            bucket_counts: [0; CODE_COUNT],
            bucket_overflow: [0; CODE_COUNT],
            buckets: std::array::from_fn(|_| Vec::new()),
            invalid_code_count: 0,
            elapsed,
        };
    }

    read_bucket_outcome(
        device,
        queue,
        &counts_buf,
        &overflow_buf,
        &bucket_buf,
        &meta_buf,
        capacity_per_code,
        elapsed,
    )
}

fn read_bucket_outcome(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    counts_buf: &wgpu::Buffer,
    overflow_buf: &wgpu::Buffer,
    bucket_buf: &wgpu::Buffer,
    meta_buf: &wgpu::Buffer,
    capacity_per_code: u32,
    elapsed: std::time::Duration,
) -> BucketOutcome {
    let counts_staging = staging_buf(device, (CODE_COUNT * 4) as u64);
    let overflow_staging = staging_buf(device, (CODE_COUNT * 4) as u64);
    let meta_staging = staging_buf(device, 4);
    let bucket_words = (CODE_COUNT as u32 * capacity_per_code.max(1) * RECORD_STRIDE) as u64 * 4;
    let bucket_staging = staging_buf(device, bucket_words.max(4));

    let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("field_policy_event1_readback"),
    });
    enc.copy_buffer_to_buffer(counts_buf, 0, &counts_staging, 0, (CODE_COUNT * 4) as u64);
    enc.copy_buffer_to_buffer(
        overflow_buf,
        0,
        &overflow_staging,
        0,
        (CODE_COUNT * 4) as u64,
    );
    enc.copy_buffer_to_buffer(meta_buf, 0, &meta_staging, 0, 4);
    if bucket_words > 0 {
        enc.copy_buffer_to_buffer(bucket_buf, 0, &bucket_staging, 0, bucket_words);
    }
    queue.submit(Some(enc.finish()));

    let counts_vec = map_u32_array(device, &counts_staging, CODE_COUNT);
    let overflow_vec = map_u32_array(device, &overflow_staging, CODE_COUNT);
    let counts: [u32; CODE_COUNT] = counts_vec.try_into().expect("counts");
    let overflow: [u32; CODE_COUNT] = overflow_vec.try_into().expect("overflow");
    let invalid = map_u32_array(device, &meta_staging, 1)[0];

    let mut buckets: [Vec<EventRecord>; CODE_COUNT] = std::array::from_fn(|_| Vec::new());
    if bucket_words > 0 {
        let slice = bucket_staging.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        device.poll(wgpu::Maintain::Wait);
        let mapped = slice.get_mapped_range();
        let words: &[u32] = bytemuck::cast_slice(&mapped);
        for code in 0..CODE_COUNT {
            let written = counts[code].min(capacity_per_code) as usize;
            buckets[code].reserve(written);
            for slot in 0..written {
                let base = (code as u32 * capacity_per_code * RECORD_STRIDE
                    + slot as u32 * RECORD_STRIDE) as usize;
                buckets[code].push(EventRecord {
                    source_index: words[base],
                    event_code: words[base + 1],
                    state: words[base + 2],
                    score_fixed: bytemuck::cast(words[base + 3]),
                });
            }
        }
        drop(mapped);
        bucket_staging.unmap();
    }

    BucketOutcome {
        bucket_counts: counts,
        bucket_overflow: overflow,
        buckets,
        invalid_code_count: invalid,
        elapsed,
    }
}

fn storage_entry(binding: u32, read_only: bool) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only },
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
        label: Some("field_policy_event1_staging"),
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn map_u32_array(device: &wgpu::Device, buf: &wgpu::Buffer, count: usize) -> Vec<u32> {
    let slice = buf.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);
    let mapped = slice.get_mapped_range();
    let out: Vec<u32> = bytemuck::cast_slice(&mapped)[..count].to_vec();
    drop(mapped);
    buf.unmap();
    let mut arr = vec![0u32; count];
    arr.copy_from_slice(&out);
    arr.try_into().ok().unwrap_or(out)
}

fn rec(index: u32, code: u32, state: u32, score: i32) -> EventRecord {
    EventRecord {
        source_index: index,
        event_code: code,
        state,
        score_fixed: score,
    }
}

fn edge_bucket_cases() -> Vec<(Vec<EventRecord>, u32, &'static str)> {
    vec![
        (vec![], 8, "no_events"),
        (vec![rec(0, 1, 1, 100)], 8, "only_code_1"),
        (vec![rec(1, 2, 0, 200)], 8, "only_code_2"),
        (
            vec![
                rec(0, 1, 1, 10),
                rec(1, 2, 0, 20),
                rec(2, 3, 1, 30),
                rec(3, 1, 1, 40),
            ],
            8,
            "mixed_codes",
        ),
        (vec![rec(0, 0, 0, 5), rec(1, 1, 1, 10)], 8, "code_0_ignored"),
        (vec![rec(0, 4, 0, 1), rec(1, 1, 1, 2)], 8, "invalid_code"),
        (
            (0..4).map(|i| rec(i, 1, 1, i as i32)).collect(),
            4,
            "capacity_exact_full",
        ),
        (
            (0..6).map(|i| rec(i, 2, 0, i as i32)).collect(),
            4,
            "capacity_overflow",
        ),
        (vec![rec(0, 1, 1, 1)], 0, "zero_capacity"),
    ]
}

fn dense_bucket_records() -> Vec<EventRecord> {
    let mut out = Vec::new();
    for idx in 0..8192u32 {
        let code = match idx % 9 {
            0 => 0,
            1 | 2 | 3 => 1,
            4 | 5 => 2,
            6 | 7 => 3,
            _ => 5,
        };
        out.push(rec(idx, code, idx % 2, (idx as i32).wrapping_mul(655)));
    }
    out
}

fn distribution_records(count: usize, label: &str) -> Vec<EventRecord> {
    let mut out = Vec::with_capacity(count);
    let mut state = 0x5345_5645u32;
    for idx in 0..count {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let code = match label {
            "all_code_1" => 1,
            "balanced_12" => 1 + (idx % 2) as u32,
            "balanced_123" => 1 + (idx % 3) as u32,
            "skewed_90_10" => {
                if (state % 100) < 90 {
                    1
                } else {
                    2
                }
            }
            "invalid_mix" => {
                if idx % 17 == 0 {
                    (4 + (idx % 3)) as u32
                } else {
                    1 + (idx % 3) as u32
                }
            }
            _ => 1,
        };
        out.push(rec(
            idx as u32,
            code,
            idx as u32 % 2,
            (idx as i32).wrapping_mul(17),
        ));
    }
    out
}

// --- Integrated PIPE-0 → bucket (3 GPU passes) ---

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

fn f32_to_q16(v: f32) -> i32 {
    (v * Q16_SCALE_F).round() as i32
}

fn layer_input(gx: f32, gy: f32, w: f32) -> LayerInput {
    LayerInput {
        gx: f32_to_q16(gx),
        gy: f32_to_q16(gy),
        w: f32_to_q16(w),
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
    let a0 = a & mask; let a1 = a >> 16u;
    let b0 = b & mask; let b1 = b >> 16u;
    let z0 = a0 * b0; let z1 = a0 * b1; let z2 = a1 * b0; let z3 = a1 * b1;
    let t = (z0 >> 16u) + (z1 & mask) + (z2 & mask);
    let lo = (z0 & mask) | ((t & mask) << 16u);
    let hi = z3 + (z1 >> 16u) + (z2 >> 16u) + (t >> 16u);
    return vec2<u32>(lo, hi);
}
fn add_u64_wide(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    let lo = a.x + b.x;
    let carry = select(0u, 1u, lo < a.x);
    return vec2<u32>(lo, a.y + b.y + carry);
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
fn u64_shr16(v: vec2<u32>) -> u32 { return (v.y << 16u) | (v.x >> 16u); }
fn q16_mul(a: i32, b: i32) -> i32 {
    let neg = (a < 0) != (b < 0);
    let prod = mul_u32_wide(abs_fixed(a), abs_fixed(b));
    let mag = u64_shr16(prod);
    if (neg) { return bitcast<i32>(0u - mag); }
    return i32(mag);
}
"#
}

fn emit_integrated_wgsl() -> String {
    format!(
        r#"{f}
{limb}
const OBS_INPUT_STRIDE: u32 = {obs_stride}u;
const EVENT_ROW_STRIDE: u32 = 5u;
const RECORD_STRIDE: u32 = 5u;
const LAYER_COUNT: u32 = {layer_count}u;
const FIELDS_PER_LAYER: u32 = 3u;
const CODE_COUNT: u32 = 4u;

struct Params {{
    row_count: u32,
    compact_capacity: u32,
    capacity_per_code: u32,
    code_count: u32,
}}

@group(0) @binding(0) var<storage, read> observer_inputs: array<u32>;
@group(0) @binding(1) var<storage, read_write> event_rows: array<u32>;
@group(0) @binding(2) var<storage, read_write> compact_counters: array<atomic<u32>, 2>;
@group(0) @binding(3) var<storage, read_write> compact_records: array<u32>;
@group(0) @binding(4) var<storage, read_write> bucket_counts: array<atomic<u32>, CODE_COUNT>;
@group(0) @binding(5) var<storage, read_write> bucket_overflow: array<atomic<u32>, CODE_COUNT>;
@group(0) @binding(6) var<storage, read_write> bucket_records: array<u32>;
@group(0) @binding(7) var<storage, read_write> bucket_meta: array<atomic<u32>, 1>;
@group(0) @binding(8) var<uniform> params: Params;

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
        let mag_bits = sqrt_cr_f_bits(mag2_u64_q16_to_f32_bits(sum));
        score_fixed = score_fixed + q16_mul(w, mag_bits_to_q16_fixed(mag_bits));
    }}
    var state_out = prior_state;
    var event_code = 0u;
    let up = threshold + hysteresis;
    let down = threshold - hysteresis;
    if (prior_state == 0u && score_fixed >= up) {{ state_out = 1u; event_code = 1u; }}
    else if (prior_state == 1u && score_fixed <= down) {{ state_out = 0u; event_code = 2u; }}
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
    let slot = atomicAdd(&compact_counters[0], 1u);
    if (slot >= params.compact_capacity) {{
        atomicStore(&compact_counters[1], 1u);
        return;
    }}
    let out_base = slot * RECORD_STRIDE;
    compact_records[out_base] = event_rows[base];
    compact_records[out_base + 1u] = code;
    compact_records[out_base + 2u] = event_rows[base + 2u];
    compact_records[out_base + 3u] = event_rows[base + 3u];
    compact_records[out_base + 4u] = 0u;
}}

@compute @workgroup_size(64)
fn bucket_pass(@builtin(global_invocation_id) gid: vec3<u32>) {{
    let i = gid.x;
    let event_total = atomicLoad(&compact_counters[0]);
    let written = min(event_total, params.compact_capacity);
    if (i >= written) {{ return; }}
    let base = i * RECORD_STRIDE;
    let code = compact_records[base + 1u];
    if (code == 0u) {{ return; }}
    if (code >= params.code_count) {{
        atomicAdd(&bucket_meta[0], 1u);
        return;
    }}
    let slot = atomicAdd(&bucket_counts[code], 1u);
    if (slot >= params.capacity_per_code) {{
        atomicStore(&bucket_overflow[code], 1u);
        return;
    }}
    let out_base = code * params.capacity_per_code * RECORD_STRIDE + slot * RECORD_STRIDE;
    bucket_records[out_base] = compact_records[base];
    bucket_records[out_base + 1u] = code;
    bucket_records[out_base + 2u] = compact_records[base + 2u];
    bucket_records[out_base + 3u] = compact_records[base + 3u];
    bucket_records[out_base + 4u] = 0u;
}}
"#,
        f = SQRT_CR_F_WGSL,
        limb = limb_arith_wgsl(),
        obs_stride = OBS_INPUT_STRIDE,
        layer_count = LAYER_COUNT,
        bias_off = BIAS_OFF,
        threshold_off = THRESHOLD_OFF,
        hysteresis_off = HYSTERESIS_OFF,
        prior_off = PRIOR_STATE_OFF,
    )
}

fn pack_observer_inputs(rows: &[ObserverRow]) -> Vec<u32> {
    let mut data = vec![0u32; rows.len() * OBS_INPUT_STRIDE as usize];
    for (i, row) in rows.iter().enumerate() {
        let base = i * OBS_INPUT_STRIDE as usize;
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

fn mobile_observer_rows(count: usize) -> Vec<ObserverRow> {
    let grads = [
        0.0, 0.01, -0.01, 0.1, -0.1, 0.5, -0.5, 1.0, -1.0, 2.0, -2.0, 4.0, 8.0, 16.0,
    ];
    let weights = [-2.0, -0.5, 0.0, 0.5, 1.0, 2.0];
    let thresholds = [-2.0, -0.5, 0.0, 0.5, 1.0, 2.0, 4.0, 8.0];
    let hysteresis = [0.0, 0.125, 0.25, 0.5, 1.0];
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

fn run_integrated_three_pass(
    ctx: &GpuContext,
    rows: &[ObserverRow],
    compact_capacity: u32,
    capacity_per_code: u32,
) -> IntegratedOutcome {
    use wgpu::util::DeviceExt;
    let device = &ctx.device;
    let queue = &ctx.queue;
    let n = rows.len() as u32;
    let params = IntegratedParams {
        row_count: n,
        compact_capacity,
        capacity_per_code,
        code_count: CODE_COUNT as u32,
    };
    let observer_data = pack_observer_inputs(rows);
    let wgsl = emit_integrated_wgsl();
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("field_policy_event1_integrated"),
        source: wgpu::ShaderSource::Wgsl(wgsl.into()),
    });

    let entries: [wgpu::BindGroupLayoutEntry; 9] = [
        storage_entry(0, true),
        storage_entry(1, false),
        storage_entry(2, false),
        storage_entry(3, false),
        storage_entry(4, false),
        storage_entry(5, false),
        storage_entry(6, false),
        storage_entry(7, false),
        uniform_entry(8),
    ];
    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("field_policy_event1_integrated_bgl"),
        entries: &entries,
    });
    let pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("field_policy_event1_integrated_pl"),
        bind_group_layouts: &[&bgl],
        push_constant_ranges: &[],
    });
    let mk_pipe = |entry: &str| {
        device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some(entry),
            layout: Some(&pl),
            module: &module,
            entry_point: entry,
            compilation_options: Default::default(),
            cache: None,
        })
    };
    let threshold_pipeline = mk_pipe("threshold_pass");
    let compact_pipeline = mk_pipe("compact_pass");
    let bucket_pipeline = mk_pipe("bucket_pass");

    let observer_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("obs"),
        contents: bytemuck::cast_slice(&observer_data),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let event_rows = vec![0u32; (n * EVENT_ROW_STRIDE) as usize];
    let event_rows_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("event_rows"),
        contents: bytemuck::cast_slice(&event_rows),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let compact_counters_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("compact_counters"),
        contents: &[0u8; 8],
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let compact_words = (compact_capacity.max(1) * RECORD_STRIDE) as usize;
    let compact_records_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("compact_records"),
        contents: bytemuck::cast_slice(&vec![0u32; compact_words]),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let bucket_words = (CODE_COUNT as u32 * capacity_per_code.max(1) * RECORD_STRIDE) as usize;
    let bucket_records_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("bucket_records"),
        contents: bytemuck::cast_slice(&vec![0u32; bucket_words]),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let bucket_counts_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("bucket_counts"),
        contents: &[0u8; CODE_COUNT * 4],
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let bucket_overflow_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("bucket_overflow"),
        contents: &[0u8; CODE_COUNT * 4],
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let bucket_meta_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("bucket_meta"),
        contents: &[0u8; 4],
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let params_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("params"),
        contents: bytemuck::bytes_of(&params),
        usage: wgpu::BufferUsages::UNIFORM,
    });

    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("integrated_bg"),
        layout: &bgl,
        entries: &[
            bind(0, &observer_buf),
            bind(1, &event_rows_buf),
            bind(2, &compact_counters_buf),
            bind(3, &compact_records_buf),
            bind(4, &bucket_counts_buf),
            bind(5, &bucket_overflow_buf),
            bind(6, &bucket_records_buf),
            bind(7, &bucket_meta_buf),
            bind(8, &params_buf),
        ],
    });

    let wg = n.div_ceil(64);
    queue.write_buffer(&compact_counters_buf, 0, &[0u8; 8]);
    queue.write_buffer(&bucket_counts_buf, 0, &[0u8; CODE_COUNT * 4]);
    queue.write_buffer(&bucket_overflow_buf, 0, &[0u8; CODE_COUNT * 4]);
    queue.write_buffer(&bucket_meta_buf, 0, &[0u8; 4]);

    let t0 = Instant::now();
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("integrated_enc"),
    });
    for (pipe, name) in [
        (&threshold_pipeline, "threshold"),
        (&compact_pipeline, "compact"),
        (&bucket_pipeline, "bucket"),
    ] {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some(name),
            timestamp_writes: None,
        });
        pass.set_pipeline(pipe);
        pass.set_bind_group(0, &bg, &[]);
        pass.dispatch_workgroups(wg, 1, 1);
    }
    queue.submit(Some(encoder.finish()));
    let elapsed = t0.elapsed();

    let compact_meta_staging = staging_buf(device, 8);
    let compact_staging = staging_buf(device, (compact_words * 4) as u64);
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    enc2.copy_buffer_to_buffer(&compact_counters_buf, 0, &compact_meta_staging, 0, 8);
    enc2.copy_buffer_to_buffer(
        &compact_records_buf,
        0,
        &compact_staging,
        0,
        (compact_words * 4) as u64,
    );
    queue.submit(Some(enc2.finish()));

    let compact_meta = map_u32_array(device, &compact_meta_staging, 2);
    let event_count = compact_meta[0];
    let compact_overflow = compact_meta[1];
    let written = event_count.min(compact_capacity) as usize;

    let slice = compact_staging.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);
    let mapped = slice.get_mapped_range();
    let words: &[u32] = bytemuck::cast_slice(&mapped);
    let mut compact_records = Vec::with_capacity(written);
    for slot in 0..written {
        let base = slot * RECORD_STRIDE as usize;
        compact_records.push(EventRecord {
            source_index: words[base],
            event_code: words[base + 1],
            state: words[base + 2],
            score_fixed: bytemuck::cast(words[base + 3]),
        });
    }
    drop(mapped);
    compact_staging.unmap();

    let bucket = read_bucket_outcome(
        device,
        queue,
        &bucket_counts_buf,
        &bucket_overflow_buf,
        &bucket_records_buf,
        &bucket_meta_buf,
        capacity_per_code,
        elapsed,
    );

    IntegratedOutcome {
        event_count,
        compact_overflow,
        compact_records,
        bucket,
        dispatch_count: 3,
    }
}

fn bind(binding: u32, buf: &wgpu::Buffer) -> wgpu::BindGroupEntry<'_> {
    wgpu::BindGroupEntry {
        binding,
        resource: buf.as_entire_binding(),
    }
}

fn verify_bucket_outcome(
    got: &BucketOutcome,
    records: &[EventRecord],
    capacity_per_code: u32,
) -> bool {
    let (exp_counts, exp_overflow, exp_buckets, exp_invalid) =
        cpu_bucket(records, capacity_per_code);
    if got.bucket_counts != exp_counts {
        return false;
    }
    if got.bucket_overflow != exp_overflow {
        return false;
    }
    if got.invalid_code_count != exp_invalid {
        return false;
    }
    for code in 0..CODE_COUNT {
        let written = got.buckets[code].len();
        let expected_written = exp_counts[code].min(capacity_per_code) as usize;
        if written != expected_written {
            return false;
        }
        if exp_overflow[code] == 0 {
            if !membership_exact(&exp_buckets[code], &got.buckets[code]) {
                return false;
            }
        }
    }
    true
}

#[test]
fn field_policy_event1_wgsl_semantic_free() {
    let wgsl = emit_bucketing_wgsl();
    for term in FORBIDDEN_SEMANTIC_TERMS {
        assert!(!wgsl.contains(term), "forbidden term `{term}`");
    }
    for term in FORBIDDEN_EXACT_TERMS {
        assert!(!wgsl.contains(term));
    }
    assert!(wgsl.contains("bucket_counts"));
    assert!(wgsl.contains("atomicAdd"));
    assert!(!wgsl.contains("planner"));
    assert!(!wgsl.contains("scheduler"));
    let integrated = emit_integrated_wgsl();
    assert!(integrated.contains("bucket_pass"));
    assert_eq!(fnv1a64_hex(SQRT_CR_F_WGSL), SQRT_F_ARTIFACT_HASH);
    println!("field_policy_event1_wgsl: semantic_free=true ordering={ORDERING_CLASS}");
}

#[test]
fn field_policy_event1_bucket_edge_rows() {
    with_gpu(|ctx| {
        for (records, capacity, label) in edge_bucket_cases() {
            let outcome = run_bucketing(ctx, &records, capacity, 1, true);
            let ok = verify_bucket_outcome(&outcome, &records, capacity);
            println!(
                "field_policy_event1_edge[{label}]: records={} capacity={capacity} counts={:?} overflow={:?} invalid={} membership_ok={ok} ordering={ORDERING_CLASS}",
                records.len(),
                &outcome.bucket_counts,
                &outcome.bucket_overflow,
                outcome.invalid_code_count,
            );
            assert!(ok);
        }
    });
}

#[test]
fn field_policy_event1_bucket_dense_corpus() {
    with_gpu(|ctx| {
        let records = dense_bucket_records();
        let capacity = 4096;
        let outcome = run_bucketing(ctx, &records, capacity, 1, true);
        assert!(verify_bucket_outcome(&outcome, &records, capacity));
        println!(
            "field_policy_event1_dense: records={} invalid={} counts={:?} ordering={ORDERING_CLASS}",
            records.len(),
            outcome.invalid_code_count,
            &outcome.bucket_counts,
        );
    });
}

#[test]
fn field_policy_event1_pipe0_to_bucket_smoke() {
    with_gpu(|ctx| {
        const N: usize = 512;
        let rows = mobile_observer_rows(N);
        let compact_capacity = N as u32;
        let capacity_per_code = N as u32;
        let outcome = run_integrated_three_pass(ctx, &rows, compact_capacity, capacity_per_code);
        let ok =
            verify_bucket_outcome(&outcome.bucket, &outcome.compact_records, capacity_per_code);
        println!(
            "field_policy_event1_pipe0_smoke: rows={N} dispatches={} event_count={} compact_overflow={} counts={:?} invalid={} membership_ok={ok}",
            outcome.dispatch_count,
            outcome.event_count,
            outcome.compact_overflow,
            &outcome.bucket.bucket_counts,
            outcome.bucket.invalid_code_count,
        );
        assert_eq!(outcome.dispatch_count, 3);
        assert_eq!(outcome.compact_overflow, 0);
        assert!(ok);
    });
}

#[test]
fn field_policy_event1_no_default_runtime_wiring() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    let desc = landed_jit_kernel_descriptors()
        .into_iter()
        .find(|d| d.id == FIELD_POLICY_EVENT1_DESCRIPTOR_ID)
        .expect("event1 descriptor");
    assert!(desc.default_off);
    assert!(!desc.production_wiring);
    assert!(is_field_policy_event1_code_bucketing_descriptor(&desc));
    validate_kernel_descriptor_admission(&desc).expect("event1 admits");
    for out in &desc.writes {
        assert_eq!(out.authority, OutputAuthority::ExactAuthoritative);
    }
    let _ = EventCodeBucketMembershipAuthority::ExactAuthoritativeUnordered;
    let _ = EventCodeBucketOrderAuthority::UnspecifiedAtomicOrder;
    println!("field_policy_event1_wiring: default_off=true descriptor={FIELD_POLICY_EVENT1_DESCRIPTOR_ID}");
}
