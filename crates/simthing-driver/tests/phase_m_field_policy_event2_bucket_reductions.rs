//! FIELD_POLICY-EVENT-2 — GPU-resident per-bucket reductions from event-code buckets (Tier-2, test-only).
//!
//! Consumes EVENT-1 bucket records; emits per-code count/sum/min/max with explicit overflow flags.
//! No CPU filtering between GPU passes; CPU oracle for verification only.

use std::sync::Mutex;
use std::time::Instant;

use simthing_gpu::GpuContext;
use simthing_spec::{
    is_field_policy_event2_bucket_reductions_descriptor, landed_jit_kernel_descriptors,
    validate_kernel_descriptor_admission, EventBucketReductionInputAuthority,
    EventBucketReductionOrderAuthority, MappingExecutionProfile, FIELD_POLICY_EVENT1_CODE_COUNT,
    FIELD_POLICY_EVENT2_DESCRIPTOR_ID,
};

use simthing_spec::OutputAuthority;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const CODE_COUNT: usize = FIELD_POLICY_EVENT1_CODE_COUNT as usize;
const RECORD_STRIDE: u32 = 5;
const RED_OUT_STRIDE: u32 = 6;
const FLAG_EMPTY: u32 = 1;
const FLAG_SUM_OVERFLOW: u32 = 2;
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
struct EventRecord {
    source_index: u32,
    event_code: u32,
    state: u32,
    score_fixed: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ReductionResult {
    count: u32,
    sum_lo: u32,
    sum_hi: i32,
    min_score: i32,
    max_score: i32,
    flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct ReduceParams {
    capacity_per_code: u32,
    code_count: u32,
    _pad: [u32; 2],
}

struct ReduceOutcome {
    per_code: [ReductionResult; CODE_COUNT],
    elapsed: std::time::Duration,
}

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let ctx = GpuContext::new_blocking().expect("GPU required");
    f(&ctx);
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
        if (hi == 2147483647 && carry == 1u) {
            ovf = 1u;
        }
    } else {
        let borrow = select(0u, 1u, new_lo > lo);
        new_hi = hi - 1;
        if (borrow == 0u) {
            new_hi = hi;
        }
        if (hi == -2147483648 && borrow == 1u) {
            ovf = 1u;
        }
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
            if (step.z != 0u) {
                sum_ovf = 1u;
            }
        }
    }
    let out = code * OUT_STRIDE;
    var flags = 0u;
    if (!has_any) {
        flags = FLAG_EMPTY;
    }
    if (sum_ovf != 0u) {
        flags = flags | FLAG_SUM_OVERFLOW;
    }
    reductions[out] = scan;
    reductions[out + 1u] = sum_lo;
    reductions[out + 2u] = bitcast<u32>(sum_hi);
    reductions[out + 3u] = bitcast<u32>(min_s);
    reductions[out + 4u] = bitcast<u32>(max_s);
    reductions[out + 5u] = flags;
}
"#
}

fn limbs_to_i64(hi: i32, lo: u32) -> i64 {
    ((i64::from(hi)) << 32) | ((lo as u64) & 0xFFFF_FFFF) as i64
}

fn cpu_reduce(records: &[EventRecord]) -> ReductionResult {
    if records.is_empty() {
        return ReductionResult {
            count: 0,
            sum_lo: 0,
            sum_hi: 0,
            min_score: 0,
            max_score: 0,
            flags: FLAG_EMPTY,
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
        flags |= FLAG_SUM_OVERFLOW;
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

fn pack_bucket_records(buckets: &[Vec<EventRecord>], capacity_per_code: u32) -> Vec<u32> {
    let words = (CODE_COUNT as u32 * capacity_per_code * RECORD_STRIDE) as usize;
    let mut data = vec![0u32; words.max(1)];
    for code in 0..CODE_COUNT {
        for (slot, rec) in buckets[code].iter().enumerate() {
            let base = (code as u32 * capacity_per_code * RECORD_STRIDE
                + slot as u32 * RECORD_STRIDE) as usize;
            data[base] = rec.source_index;
            data[base + 1] = rec.event_code;
            data[base + 2] = rec.state;
            data[base + 3] = bytemuck::cast(rec.score_fixed);
            data[base + 4] = 0;
        }
    }
    data
}

fn run_reductions(
    ctx: &GpuContext,
    bucket_counts: [u32; CODE_COUNT],
    bucket_records: &[u32],
    capacity_per_code: u32,
    repeat_dispatches: u32,
    do_readback: bool,
) -> ReduceOutcome {
    use wgpu::util::DeviceExt;
    let device = &ctx.device;
    let queue = &ctx.queue;
    let params = ReduceParams {
        capacity_per_code,
        code_count: CODE_COUNT as u32,
        _pad: [0, 0],
    };

    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("field_policy_event2_reduction"),
        source: wgpu::ShaderSource::Wgsl(emit_reduction_wgsl().into()),
    });

    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("field_policy_event2_bgl"),
        entries: &[
            storage_ro(0),
            storage_ro(1),
            storage_rw(2),
            uniform_entry(3),
        ],
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("field_policy_event2_reduce"),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("field_policy_event2_pl"),
                bind_group_layouts: &[&bgl],
                push_constant_ranges: &[],
            }),
        ),
        module: &module,
        entry_point: "reduce_pass",
        compilation_options: Default::default(),
        cache: None,
    });

    let counts_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("bucket_counts"),
        contents: bytemuck::cast_slice(&bucket_counts),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let records_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("bucket_records"),
        contents: bytemuck::cast_slice(bucket_records),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let red_init = vec![0u32; (CODE_COUNT as u32 * RED_OUT_STRIDE) as usize];
    let red_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("reductions"),
        contents: bytemuck::cast_slice(&red_init),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let params_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("reduce_params"),
        contents: bytemuck::bytes_of(&params),
        usage: wgpu::BufferUsages::UNIFORM,
    });

    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("field_policy_event2_bg"),
        layout: &bgl,
        entries: &[
            bind_entry(0, &counts_buf),
            bind_entry(1, &records_buf),
            bind_entry(2, &red_buf),
            bind_entry(3, &params_buf),
        ],
    });

    let t0 = Instant::now();
    for _ in 0..repeat_dispatches {
        let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("field_policy_event2_enc"),
        });
        {
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("reduce"),
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
        return ReduceOutcome {
            per_code: [ReductionResult {
                count: 0,
                sum_lo: 0,
                sum_hi: 0,
                min_score: 0,
                max_score: 0,
                flags: 0,
            }; CODE_COUNT],
            elapsed,
        };
    }

    let red_bytes = (CODE_COUNT as u32 * RED_OUT_STRIDE * 4) as u64;
    let staging = staging_buf(device, red_bytes);
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    enc2.copy_buffer_to_buffer(&red_buf, 0, &staging, 0, red_bytes);
    queue.submit(Some(enc2.finish()));

    let slice = staging.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);
    let mapped = slice.get_mapped_range();
    let words: &[u32] = bytemuck::cast_slice(&mapped);
    let mut per_code = [ReductionResult {
        count: 0,
        sum_lo: 0,
        sum_hi: 0,
        min_score: 0,
        max_score: 0,
        flags: 0,
    }; CODE_COUNT];
    for code in 0..CODE_COUNT {
        let base = code * RED_OUT_STRIDE as usize;
        per_code[code] = ReductionResult {
            count: words[base],
            sum_lo: words[base + 1],
            sum_hi: bytemuck::cast(words[base + 2]),
            min_score: bytemuck::cast(words[base + 3]),
            max_score: bytemuck::cast(words[base + 4]),
            flags: words[base + 5],
        };
    }
    drop(mapped);
    staging.unmap();

    ReduceOutcome { per_code, elapsed }
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

fn rec(index: u32, code: u32, state: u32, score: i32) -> EventRecord {
    EventRecord {
        source_index: index,
        event_code: code,
        state,
        score_fixed: score,
    }
}

fn verify_reductions(
    got: &[ReductionResult; CODE_COUNT],
    buckets: &[Vec<EventRecord>],
    bucket_counts: [u32; CODE_COUNT],
    capacity_per_code: u32,
) -> bool {
    for code in 0..CODE_COUNT {
        let scan = bucket_counts[code].min(capacity_per_code) as usize;
        let slice = &buckets[code][..scan.min(buckets[code].len())];
        let exp = cpu_reduce(slice);
        let g = got[code];
        if g.count != exp.count {
            return false;
        }
        if g.min_score != exp.min_score || g.max_score != exp.max_score {
            return false;
        }
        if (g.flags & FLAG_EMPTY) != (exp.flags & FLAG_EMPTY) {
            return false;
        }
        if (g.flags & FLAG_SUM_OVERFLOW) != (exp.flags & FLAG_SUM_OVERFLOW) {
            return false;
        }
        if exp.flags & FLAG_SUM_OVERFLOW == 0 {
            if limbs_to_i64(g.sum_hi, g.sum_lo) != limbs_to_i64(exp.sum_hi, exp.sum_lo) {
                return false;
            }
        }
    }
    true
}

fn edge_reduction_cases() -> Vec<(Vec<Vec<EventRecord>>, [u32; CODE_COUNT], u32, &'static str)> {
    vec![
        (vec![vec![], vec![], vec![], vec![]], [0; 4], 8, "empty"),
        (
            vec![vec![], vec![rec(0, 1, 1, 100)], vec![], vec![]],
            [0, 1, 0, 0],
            8,
            "single",
        ),
        (
            vec![
                vec![],
                vec![rec(0, 1, 0, 1000), rec(1, 1, 0, 2000)],
                vec![],
                vec![],
            ],
            [0, 2, 0, 0],
            8,
            "all_positive",
        ),
        (
            vec![
                vec![],
                vec![],
                vec![rec(0, 2, 0, -500), rec(1, 2, 0, -1000)],
                vec![],
            ],
            [0, 0, 2, 0],
            8,
            "all_negative",
        ),
        (
            vec![
                vec![],
                vec![rec(0, 1, 0, -100), rec(1, 1, 0, 200)],
                vec![rec(2, 2, 0, 50), rec(3, 2, 0, -50)],
                vec![],
            ],
            [0, 2, 2, 0],
            8,
            "mixed_signs",
        ),
        (
            vec![
                vec![],
                vec![rec(0, 1, 0, 5), rec(1, 1, 0, 5)],
                vec![],
                vec![],
            ],
            [0, 2, 0, 0],
            8,
            "min_max_ties",
        ),
        (
            vec![
                vec![],
                (0..4).map(|i| rec(i, 1, 0, i as i32 * 100)).collect(),
                vec![],
                vec![],
            ],
            [0, 4, 0, 0],
            4,
            "capacity_full",
        ),
        (
            vec![
                vec![],
                (0..6).map(|i| rec(i, 1, 0, i as i32)).collect(),
                vec![],
                vec![],
            ],
            [0, 6, 0, 0],
            4,
            "overflowed_input",
        ),
        (
            vec![
                vec![],
                vec![rec(0, 1, 0, i32::MAX), rec(1, 1, 0, 1)],
                vec![],
                vec![],
            ],
            [0, 2, 0, 0],
            8,
            "sum_overflow",
        ),
    ]
}

fn dense_buckets() -> (Vec<Vec<EventRecord>>, [u32; CODE_COUNT]) {
    let mut buckets: [Vec<EventRecord>; CODE_COUNT] = std::array::from_fn(|_| Vec::new());
    for idx in 0..4096u32 {
        let code = 1 + (idx % 3);
        buckets[code as usize].push(rec(idx, code, idx % 2, (idx as i32).wrapping_mul(655)));
    }
    let counts = [
        0,
        buckets[1].len() as u32,
        buckets[2].len() as u32,
        buckets[3].len() as u32,
    ];
    (buckets.to_vec(), counts)
}

fn balanced_12_records(count: usize) -> (Vec<Vec<EventRecord>>, [u32; CODE_COUNT]) {
    let mut buckets: [Vec<EventRecord>; CODE_COUNT] = std::array::from_fn(|_| Vec::new());
    for idx in 0..count {
        let code = 1 + (idx % 2) as u32;
        buckets[code as usize].push(rec(
            idx as u32,
            code,
            idx as u32 % 2,
            (idx as i32).wrapping_mul(17),
        ));
    }
    let counts = [0, buckets[1].len() as u32, buckets[2].len() as u32, 0];
    (buckets.to_vec(), counts)
}

// --- Integrated GPU: bucket + reduce (and optional compact/threshold) ---

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

fn pack_records(records: &[EventRecord]) -> Vec<u32> {
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

fn run_bucket_then_reduce_gpu(
    ctx: &GpuContext,
    compact_records: &[EventRecord],
    capacity_per_code: u32,
) -> ([u32; CODE_COUNT], [ReductionResult; CODE_COUNT]) {
    use wgpu::util::DeviceExt;
    let device = &ctx.device;
    let queue = &ctx.queue;
    let packed = if compact_records.is_empty() {
        vec![0u32]
    } else {
        pack_records(compact_records)
    };
    #[repr(C)]
    #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
    struct BucketParams {
        record_count: u32,
        capacity_per_code: u32,
        code_count: u32,
        _pad: u32,
    }
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

    let bucket_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("bucket"),
        source: wgpu::ShaderSource::Wgsl(emit_bucket_wgsl().into()),
    });
    let reduce_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("reduce"),
        source: wgpu::ShaderSource::Wgsl(emit_reduction_wgsl().into()),
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

    let bucket_pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("bucket_pl"),
        bind_group_layouts: &[&bucket_bgl],
        push_constant_ranges: &[],
    });
    let reduce_pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("reduce_pl"),
        bind_group_layouts: &[&reduce_bgl],
        push_constant_ranges: &[],
    });

    let bucket_pipe = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("bucket_pass"),
        layout: Some(&bucket_pl),
        module: &bucket_module,
        entry_point: "bucket_pass",
        compilation_options: Default::default(),
        cache: None,
    });
    let reduce_pipe = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("reduce_pass"),
        layout: Some(&reduce_pl),
        module: &reduce_module,
        entry_point: "reduce_pass",
        compilation_options: Default::default(),
        cache: None,
    });

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

    queue.write_buffer(&counts_atomic, 0, &[0u8; CODE_COUNT * 4]);
    let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
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
    queue.submit(Some(enc.finish()));

    let counts_staging = staging_buf(device, (CODE_COUNT * 4) as u64);
    let red_staging = staging_buf(device, (CODE_COUNT as u32 * RED_OUT_STRIDE * 4) as u64);
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    enc2.copy_buffer_to_buffer(&counts_read, 0, &counts_staging, 0, (CODE_COUNT * 4) as u64);
    enc2.copy_buffer_to_buffer(
        &red_buf,
        0,
        &red_staging,
        0,
        (CODE_COUNT as u32 * RED_OUT_STRIDE * 4) as u64,
    );
    queue.submit(Some(enc2.finish()));

    let counts_vec = read_u32s(device, &counts_staging, CODE_COUNT);
    let red_vec = read_u32s(device, &red_staging, CODE_COUNT * RED_OUT_STRIDE as usize);
    let mut counts = [0u32; CODE_COUNT];
    counts.copy_from_slice(&counts_vec[..CODE_COUNT]);
    let mut per_code = [ReductionResult {
        count: 0,
        sum_lo: 0,
        sum_hi: 0,
        min_score: 0,
        max_score: 0,
        flags: 0,
    }; CODE_COUNT];
    for code in 0..CODE_COUNT {
        let base = code * RED_OUT_STRIDE as usize;
        per_code[code] = ReductionResult {
            count: red_vec[base],
            sum_lo: red_vec[base + 1],
            sum_hi: bytemuck::cast(red_vec[base + 2]),
            min_score: bytemuck::cast(red_vec[base + 3]),
            max_score: bytemuck::cast(red_vec[base + 4]),
            flags: red_vec[base + 5],
        };
    }
    (counts, per_code)
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

fn cpu_bucket_from_compact(
    records: &[EventRecord],
    capacity: u32,
) -> (Vec<Vec<EventRecord>>, [u32; CODE_COUNT]) {
    let mut buckets: [Vec<EventRecord>; CODE_COUNT] = std::array::from_fn(|_| Vec::new());
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

#[test]
fn field_policy_event2_wgsl_semantic_free() {
    let wgsl = emit_reduction_wgsl();
    for term in FORBIDDEN_SEMANTIC_TERMS {
        assert!(!wgsl.contains(term), "forbidden `{term}`");
    }
    for term in FORBIDDEN_EXACT_TERMS {
        assert!(!wgsl.contains(term));
    }
    assert!(wgsl.contains("reduce_pass"));
    assert!(wgsl.contains("sum_hi"));
    assert!(!wgsl.contains("planner"));
    assert!(!wgsl.contains("wrapping_add"));
    println!("field_policy_event2_wgsl: semantic_free=true ordering={ORDERING_CLASS}");
}

#[test]
fn field_policy_event2_reduction_edge_rows() {
    with_gpu(|ctx| {
        for (buckets, counts, cap, label) in edge_reduction_cases() {
            let packed = pack_bucket_records(&buckets, cap);
            let outcome = run_reductions(ctx, counts, &packed, cap, 1, true);
            let ok = verify_reductions(&outcome.per_code, &buckets, counts, cap);
            println!(
                "field_policy_event2_edge[{label}]: counts={counts:?} ok={ok} flags={:?} ordering={ORDERING_CLASS}",
                outcome.per_code.iter().map(|r| r.flags).collect::<Vec<_>>()
            );
            assert!(ok, "edge case {label}");
        }
    });
}

#[test]
fn field_policy_event2_reduction_dense_corpus() {
    with_gpu(|ctx| {
        let (buckets, counts) = dense_buckets();
        let cap = 4096;
        let packed = pack_bucket_records(&buckets, cap);
        let outcome = run_reductions(ctx, counts, &packed, cap, 1, true);
        assert!(verify_reductions(&outcome.per_code, &buckets, counts, cap));
        println!("field_policy_event2_dense: counts={counts:?} sums_ok ordering={ORDERING_CLASS}");
    });
}

#[test]
fn field_policy_event2_event1_to_reductions_smoke() {
    with_gpu(|ctx| {
        let mut compact = Vec::new();
        for idx in 0..256u32 {
            let code = 1 + (idx % 3);
            compact.push(rec(idx, code, idx % 2, (idx as i32) * 100));
        }
        let (counts, reductions) = run_bucket_then_reduce_gpu(ctx, &compact, 256);
        let (buckets, exp_counts) = cpu_bucket_from_compact(&compact, 256);
        assert_eq!(counts, exp_counts);
        assert!(verify_reductions(&reductions, &buckets, counts, 256));
        println!(
            "field_policy_event2_event1_smoke: records={} counts={counts:?} ordering={ORDERING_CLASS}",
            compact.len()
        );
    });
}

#[test]
fn field_policy_event2_pipe0_to_bucket_reductions_smoke() {
    with_gpu(|ctx| {
        // PIPE-0 compact output simulated as GPU would produce: use same mobile pattern scores
        let mut compact = Vec::new();
        for idx in 0..512u32 {
            if idx % 3 != 0 {
                compact.push(rec(
                    idx,
                    if idx % 2 == 0 { 1 } else { 2 },
                    idx % 2,
                    (idx as i32).wrapping_mul(655),
                ));
            }
        }
        let cap = 512u32;
        let (counts, reductions) = run_bucket_then_reduce_gpu(ctx, &compact, cap);
        let (buckets, exp_counts) = cpu_bucket_from_compact(&compact, cap);
        assert_eq!(counts, exp_counts);
        assert!(verify_reductions(&reductions, &buckets, counts, cap));
        println!(
            "field_policy_event2_pipe0_smoke: compact={} event_count={} counts={counts:?} ordering={ORDERING_CLASS}",
            compact.len(),
            compact.len(),
        );
    });
}

#[test]
fn field_policy_event2_perf_34k_bucket_reductions() {
    with_gpu(|ctx| {
        const N: usize = 34_000;
        const CAP: u32 = 20_000;
        let (buckets, counts) = balanced_12_records(N);
        let packed = pack_bucket_records(&buckets, CAP);
        let t0 = Instant::now();
        let outcome = run_reductions(ctx, counts, &packed, CAP, 1, true);
        let elapsed_ms = t0.elapsed().as_secs_f64() * 1000.0;
        let per_record_us = elapsed_ms * 1000.0 / N as f64;
        assert!(verify_reductions(&outcome.per_code, &buckets, counts, CAP));
        println!(
            "field_policy_event2_34k: elapsed_ms={elapsed_ms:.3} per_record_us={per_record_us:.4} counts={counts:?} per_code={:?} ordering={ORDERING_CLASS}",
            outcome
                .per_code
                .iter()
                .map(|r| (r.count, r.flags))
                .collect::<Vec<_>>()
        );
    });
}

#[test]
fn field_policy_event2_perf_34k_warm_repeated_dispatch() {
    with_gpu(|ctx| {
        const N: usize = 34_000;
        const CAP: u32 = 20_000;
        const REPEATS: u32 = 32;
        let (buckets, counts) = balanced_12_records(N);
        let packed = pack_bucket_records(&buckets, CAP);
        let outcome = run_reductions(ctx, counts, &packed, CAP, REPEATS, true);
        let total_ms = outcome.elapsed.as_secs_f64() * 1000.0;
        let per_dispatch_ms = total_ms / REPEATS as f64;
        let per_record_us = per_dispatch_ms * 1000.0 / N as f64;
        assert!(verify_reductions(&outcome.per_code, &buckets, counts, CAP));
        println!(
            "field_policy_event2_34k_warm: repeats={REPEATS} total_ms={total_ms:.3} per_dispatch_ms={per_dispatch_ms:.4} per_record_us={per_record_us:.4} counts={counts:?} ordering={ORDERING_CLASS}"
        );
    });
}

#[test]
fn field_policy_event2_no_default_runtime_wiring() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    let desc = landed_jit_kernel_descriptors()
        .into_iter()
        .find(|d| d.id == FIELD_POLICY_EVENT2_DESCRIPTOR_ID)
        .expect("event2 descriptor");
    assert!(desc.default_off);
    assert!(!desc.production_wiring);
    assert!(is_field_policy_event2_bucket_reductions_descriptor(&desc));
    validate_kernel_descriptor_admission(&desc).expect("event2 admits");
    for out in &desc.writes {
        assert_eq!(out.authority, OutputAuthority::ExactAuthoritative);
    }
    let _ = EventBucketReductionInputAuthority::ExactAuthoritativeUnordered;
    let _ = EventBucketReductionOrderAuthority::UnspecifiedAtomicOrder;
    println!("field_policy_event2_wiring: default_off=true descriptor={FIELD_POLICY_EVENT2_DESCRIPTOR_ID}");
}
