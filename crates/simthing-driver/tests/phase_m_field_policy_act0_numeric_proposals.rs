//! FIELD_POLICY-ACT-0 — GPU-resident numeric action proposals from EVENT-2 bucket reductions (Tier-2, test-only).
//!
//! Consumes per-code reductions; emits bounded numeric proposal records under fixed integer rules.
//! No CPU filtering between GPU passes; CPU oracle for verification only.

use std::collections::BTreeMap;
use std::sync::Mutex;
use std::time::Instant;

use simthing_gpu::GpuContext;
use simthing_spec::{
    is_field_policy_act0_numeric_proposals_descriptor, landed_jit_kernel_descriptors,
    validate_kernel_descriptor_admission, MappingExecutionProfile,
    NumericProposalMembershipAuthority, NumericProposalOrderAuthority,
    FIELD_POLICY_ACT0_DESCRIPTOR_ID, FIELD_POLICY_EVENT1_CODE_COUNT,
};

use simthing_spec::OutputAuthority;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const CODE_COUNT: usize = FIELD_POLICY_EVENT1_CODE_COUNT as usize;
const RECORD_STRIDE: u32 = 5;
const RED_OUT_STRIDE: u32 = 6;
const PROP_STRIDE: u32 = 5;
const FLAG_RED_EMPTY: u32 = 1;
const FLAG_RED_SUM_OVERFLOW: u32 = 2;
const FLAG_RULE_MAX: u32 = 1;
const FLAG_RULE_SUM: u32 = 2;
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
    "buy",
    "sell",
    "ship",
    "factory",
];

const FORBIDDEN_EXACT_TERMS: &[&str] = &["f64", "F64RoundDown", "SHADER_F64", "sqrt_cr_c"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ProposalRecord {
    source_code: u32,
    proposal_code: u32,
    count: u32,
    score: i32,
    flags: u32,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EventRecord {
    source_index: u32,
    event_code: u32,
    state: u32,
    score_fixed: i32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct ProposalRuleGpu {
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
struct ProposeParams {
    code_count: u32,
    proposal_capacity: u32,
    _pad: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct ReduceParams {
    capacity_per_code: u32,
    code_count: u32,
    _pad: [u32; 2],
}

struct ProposalOutcome {
    proposal_count: u32,
    proposal_overflow: u32,
    proposals: Vec<ProposalRecord>,
    elapsed: std::time::Duration,
}

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let ctx = GpuContext::new_blocking().expect("GPU required");
    f(&ctx);
}

fn default_rules() -> [ProposalRuleGpu; CODE_COUNT] {
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

fn limbs_to_i64(hi: i32, lo: u32) -> i64 {
    ((i64::from(hi)) << 32) | ((lo as u64) & 0xFFFF_FFFF) as i64
}

fn rec(index: u32, code: u32, state: u32, score: i32) -> EventRecord {
    EventRecord {
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

fn cpu_propose(
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

fn proposal_key(p: &ProposalRecord) -> (u32, u32, u32, i32, u32) {
    (p.source_code, p.proposal_code, p.count, p.score, p.flags)
}

fn membership_exact(got: &[ProposalRecord], exp: &[ProposalRecord]) -> bool {
    let mut g: BTreeMap<(u32, u32, u32, i32, u32), u32> = BTreeMap::new();
    let mut e: BTreeMap<(u32, u32, u32, i32, u32), u32> = BTreeMap::new();
    for p in got {
        *g.entry(proposal_key(p)).or_insert(0) += 1;
    }
    for p in exp {
        *e.entry(proposal_key(p)).or_insert(0) += 1;
    }
    g == e
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
        label: Some("field_policy_act0_propose"),
        source: wgpu::ShaderSource::Wgsl(emit_proposal_wgsl().into()),
    });
    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("field_policy_act0_bgl"),
        entries: &[
            storage_ro(0),
            storage_ro(1),
            storage_rw(2),
            storage_rw(3),
            uniform_entry(4),
        ],
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("field_policy_act0_propose"),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("field_policy_act0_pl"),
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
        label: Some("field_policy_act0_bg"),
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
struct BucketParams {
    record_count: u32,
    capacity_per_code: u32,
    code_count: u32,
    _pad: u32,
}

struct ChainOutcome {
    reductions: [ReductionResult; CODE_COUNT],
    proposal_count: u32,
    proposal_overflow: u32,
    proposals: Vec<ProposalRecord>,
    elapsed: std::time::Duration,
    dispatch_count: u32,
}

fn run_bucket_reduce_propose_gpu(
    ctx: &GpuContext,
    compact_records: &[EventRecord],
    capacity_per_code: u32,
    rules: &[ProposalRuleGpu; CODE_COUNT],
    proposal_capacity: u32,
    repeat_dispatches: u32,
) -> ChainOutcome {
    use wgpu::util::DeviceExt;
    let device = &ctx.device;
    let queue = &ctx.queue;
    let packed = if compact_records.is_empty() {
        vec![0u32]
    } else {
        pack_records(compact_records)
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
        queue.submit(Some(enc.finish()));
    }
    let elapsed = t0.elapsed();

    let red_staging = staging_buf(device, (CODE_COUNT as u32 * RED_OUT_STRIDE * 4) as u64);
    let meta_staging = staging_buf(device, 8);
    let prop_staging = staging_buf(device, (prop_words * 4) as u64);
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    enc2.copy_buffer_to_buffer(
        &red_buf,
        0,
        &red_staging,
        0,
        (CODE_COUNT as u32 * RED_OUT_STRIDE * 4) as u64,
    );
    enc2.copy_buffer_to_buffer(&prop_meta, 0, &meta_staging, 0, 8);
    enc2.copy_buffer_to_buffer(&prop_buf, 0, &prop_staging, 0, (prop_words * 4) as u64);
    queue.submit(Some(enc2.finish()));

    let red_vec = read_u32s(device, &red_staging, CODE_COUNT * RED_OUT_STRIDE as usize);
    let meta = read_u32s(device, &meta_staging, 2);
    let prop_vec = read_u32s(device, &prop_staging, prop_words);
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
    let written = meta[0].min(proposal_capacity) as usize;
    ChainOutcome {
        reductions,
        proposal_count: meta[0],
        proposal_overflow: meta[1],
        proposals: decode_proposals(&prop_vec, written),
        elapsed,
        dispatch_count: 3 * repeat_dispatches,
    }
}

fn cpu_reduce(records: &[EventRecord]) -> ReductionResult {
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

fn reductions_from_buckets(
    buckets: &[Vec<EventRecord>],
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

fn balanced_12_records(count: usize) -> Vec<EventRecord> {
    let mut out = Vec::with_capacity(count);
    for idx in 0..count {
        let code = 1 + (idx % 2) as u32;
        out.push(rec(
            idx as u32,
            code,
            idx as u32 % 2,
            (idx as i32).wrapping_mul(17),
        ));
    }
    out
}

fn edge_rules() -> [ProposalRuleGpu; CODE_COUNT] {
    let mut rules = default_rules();
    for rule in &mut rules {
        rule.enable_sum_rule = 0;
    }
    rules
}

fn edge_proposal_cases() -> Vec<([ReductionResult; CODE_COUNT], u32, &'static str)> {
    let empty = [ReductionResult {
        count: 0,
        sum_lo: 0,
        sum_hi: 0,
        min_score: 0,
        max_score: 0,
        flags: FLAG_RED_EMPTY,
    }; CODE_COUNT];
    let mut below_count = empty;
    below_count[1] = ReductionResult {
        count: 1,
        sum_lo: 600,
        sum_hi: 0,
        min_score: 600,
        max_score: 600,
        flags: 0,
    };
    let mut at_count = below_count;
    at_count[1].count = 2;
    let mut above_count = at_count;
    above_count[1].count = 5;
    let mut below_score = at_count;
    below_score[1].max_score = 499;
    let mut at_score = at_count;
    at_score[1].max_score = 500;
    let mut above_score = at_count;
    above_score[1].max_score = 2000;
    let mut multi = empty;
    multi[1] = above_score[1];
    multi[2] = ReductionResult {
        count: 1,
        sum_lo: 500,
        sum_hi: 0,
        min_score: 500,
        max_score: 500,
        flags: 0,
    };
    let mut sum_ovf = at_count;
    sum_ovf[1].flags = FLAG_RED_SUM_OVERFLOW;
    vec![
        (empty, 64, "empty_reductions"),
        (below_count, 64, "count_below"),
        (at_count, 64, "count_equal"),
        (above_count, 64, "count_above"),
        (below_score, 64, "score_below"),
        (at_score, 64, "score_equal"),
        (above_score, 64, "score_above"),
        (multi, 64, "multiple_codes"),
        (above_score, 0, "zero_capacity"),
        (multi, 2, "capacity_exact"),
        (multi, 1, "capacity_overflow"),
        (sum_ovf, 64, "input_sum_overflow"),
    ]
}

fn dense_reductions() -> [ReductionResult; CODE_COUNT] {
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
    reductions_from_buckets(&buckets, counts, 4096)
}

// Pipe smoke: compact records mirror PIPE-0 mobile output; bucket→reduce→propose on GPU without CPU filtering.
fn pipe_compact_corpus() -> Vec<EventRecord> {
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
    compact
}

fn rules_for_smoke() -> [ProposalRuleGpu; CODE_COUNT] {
    let mut rules = default_rules();
    rules[1].threshold_sum_lo = 100_000;
    rules[1].threshold_sum_hi = 0;
    rules[2].threshold_max = 100;
    rules
}

#[test]
fn field_policy_act0_wgsl_semantic_free() {
    let wgsl = format!(
        "{}\n{}\n{}",
        emit_proposal_wgsl(),
        emit_reduction_wgsl(),
        emit_bucket_wgsl()
    );
    for term in FORBIDDEN_SEMANTIC_TERMS {
        assert!(!wgsl.contains(term), "forbidden `{term}`");
    }
    for term in FORBIDDEN_EXACT_TERMS {
        assert!(!wgsl.contains(term));
    }
    assert!(wgsl.contains("propose_pass"));
    assert!(!wgsl.contains("scheduler"));
    assert!(!wgsl.contains("cache"));
    println!("field_policy_act0_wgsl: semantic_free=true ordering={ORDERING_CLASS}");
}

#[test]
fn field_policy_act0_proposal_edge_rows() {
    let rules = edge_rules();
    with_gpu(|ctx| {
        for (reds_arr, cap, label) in edge_proposal_cases() {
            let outcome = run_proposals_gpu(ctx, &reds_arr, &rules, cap, 1, true);
            let (exp_count, exp_ovf, exp_props) = cpu_propose(&reds_arr, &rules, cap);
            assert_eq!(outcome.proposal_count, exp_count, "{label} count");
            assert_eq!(outcome.proposal_overflow, exp_ovf, "{label} overflow");
            if exp_ovf == 0 {
                assert!(
                    membership_exact(&outcome.proposals, &exp_props),
                    "{label} membership"
                );
            }
            println!(
                "field_policy_act0_edge[{label}]: count={} overflow={} written={} ordering={ORDERING_CLASS}",
                outcome.proposal_count,
                outcome.proposal_overflow,
                outcome.proposals.len(),
            );
        }
    });
}

#[test]
fn field_policy_act0_dense_proposal_corpus() {
    let rules = default_rules();
    with_gpu(|ctx| {
        let reds = dense_reductions();
        let outcome = run_proposals_gpu(ctx, &reds, &rules, 256, 1, true);
        let (exp_count, exp_ovf, exp_props) = cpu_propose(&reds, &rules, 256);
        assert_eq!(outcome.proposal_count, exp_count);
        assert_eq!(outcome.proposal_overflow, exp_ovf);
        assert!(membership_exact(&outcome.proposals, &exp_props));
        println!(
            "field_policy_act0_dense: count={} overflow={} membership=exact ordering={ORDERING_CLASS}",
            outcome.proposal_count, outcome.proposal_overflow
        );
    });
}

#[test]
fn field_policy_act0_event2_to_proposal_smoke() {
    let rules = rules_for_smoke();
    with_gpu(|ctx| {
        let mut compact = Vec::new();
        for idx in 0..256u32 {
            let code = 1 + (idx % 3);
            compact.push(rec(idx, code, idx % 2, (idx as i32) * 100));
        }
        let cap = 256u32;
        let prop_cap = 64u32;
        let outcome = run_bucket_reduce_propose_gpu(ctx, &compact, cap, &rules, prop_cap, 1);
        let (buckets, counts) = cpu_bucket_from_compact(&compact, cap);
        let exp_reds = reductions_from_buckets(&buckets, counts, cap);
        for code in 0..CODE_COUNT {
            assert_eq!(outcome.reductions[code].count, exp_reds[code].count);
            assert_eq!(outcome.reductions[code].max_score, exp_reds[code].max_score);
            assert_eq!(
                outcome.reductions[code].flags & FLAG_RED_EMPTY,
                exp_reds[code].flags & FLAG_RED_EMPTY
            );
        }
        let (exp_count, exp_ovf, exp_props) = cpu_propose(&exp_reds, &rules, prop_cap);
        assert_eq!(outcome.proposal_count, exp_count);
        assert_eq!(outcome.proposal_overflow, exp_ovf);
        assert!(membership_exact(&outcome.proposals, &exp_props));
        println!(
            "field_policy_act0_event2_smoke: records={} proposal_count={} overflow={} dispatches=3 ordering={ORDERING_CLASS}",
            compact.len(),
            outcome.proposal_count,
            outcome.proposal_overflow,
        );
    });
}

#[test]
fn field_policy_act0_pipe_to_proposal_smoke() {
    let rules = rules_for_smoke();
    with_gpu(|ctx| {
        let compact = pipe_compact_corpus();
        let cap = 512u32;
        let prop_cap = 128u32;
        let outcome = run_bucket_reduce_propose_gpu(ctx, &compact, cap, &rules, prop_cap, 1);
        let (buckets, counts) = cpu_bucket_from_compact(&compact, cap);
        let exp_reds = reductions_from_buckets(&buckets, counts, cap);
        let (exp_count, exp_ovf, exp_props) = cpu_propose(&exp_reds, &rules, prop_cap);
        assert_eq!(outcome.proposal_count, exp_count);
        assert_eq!(outcome.proposal_overflow, exp_ovf);
        if exp_ovf == 0 {
            assert!(membership_exact(&outcome.proposals, &exp_props));
        }
        println!(
            "field_policy_act0_pipe_smoke: compact={} event_count={} bucket_counts={counts:?} proposal_count={} overflow={} ordering={ORDERING_CLASS}",
            compact.len(),
            compact.len(),
            outcome.proposal_count,
            outcome.proposal_overflow,
        );
    });
}

#[test]
fn field_policy_act0_no_default_runtime_wiring() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    let desc = landed_jit_kernel_descriptors()
        .into_iter()
        .find(|d| d.id == FIELD_POLICY_ACT0_DESCRIPTOR_ID)
        .expect("act0 descriptor");
    assert!(desc.default_off);
    assert!(!desc.production_wiring);
    assert!(is_field_policy_act0_numeric_proposals_descriptor(&desc));
    validate_kernel_descriptor_admission(&desc).expect("act0 admits");
    for out in &desc.writes {
        assert_eq!(out.authority, OutputAuthority::ExactAuthoritative);
    }
    let _ = NumericProposalMembershipAuthority::ExactAuthoritativeUnordered;
    let _ = NumericProposalOrderAuthority::UnspecifiedAtomicOrder;
    println!(
        "field_policy_act0_wiring: default_off=true descriptor={FIELD_POLICY_ACT0_DESCRIPTOR_ID}"
    );
}
