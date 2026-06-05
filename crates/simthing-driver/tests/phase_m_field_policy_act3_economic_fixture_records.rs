//! FIELD_POLICY-ACT-3 — Economic V1-style fixture substrate records from ACT-2 admission records (Tier-2, test-only).
//!
//! Consumes admission_record; emits fixture substrate records under fixed integer mapping table.
//! No CPU filtering between GPU passes; CPU oracle for verification only.

use std::sync::Mutex;
use std::time::Instant;

use simthing_gpu::GpuContext;
use simthing_spec::{
    is_field_policy_act3_economic_fixture_records_descriptor, landed_jit_kernel_descriptors,
    validate_kernel_descriptor_admission, MappingExecutionProfile,
    PhaseEEconomicFixtureRecordAuthority, FIELD_POLICY_ACT1_ADMITTED_TABLE_SIZE,
    FIELD_POLICY_ACT3_DESCRIPTOR_ID, FIELD_POLICY_EVENT1_CODE_COUNT,
};

use simthing_spec::OutputAuthority;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const CODE_COUNT: usize = FIELD_POLICY_EVENT1_CODE_COUNT as usize;
const RECORD_STRIDE: u32 = 5;
const RED_OUT_STRIDE: u32 = 6;
const PROP_STRIDE: u32 = 5;
const SUMMARY_STRIDE: u32 = 7;
const ADMIT_STRIDE: u32 = 7;
const FIXTURE_STRIDE: u32 = 10;
const MAPPING_TABLE_SIZE: usize = 8;
const ADMITTED_TABLE_SIZE: usize = FIELD_POLICY_ACT1_ADMITTED_TABLE_SIZE as usize;
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
const FLAG_FIX_EMITTED: u32 = 1;
const FLAG_FIX_REJ_NOT_ADMITTED: u32 = 2;
const FLAG_FIX_REJ_UNKNOWN: u32 = 4;
const FLAG_FIX_INPUT_OVF: u32 = 8;
const FLAG_FIX_SUM_OVF: u32 = 16;
const ORDERING_CLASS: &str = "OrderInvariantExact";

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
    "decision",
    "allocate",
    "production",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ProposalSummary {
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
struct ConsumerParams {
    proposal_capacity: u32,
    admitted_count: u32,
    _pad: [u32; 2],
}

struct ConsumerOutcome {
    summary: ProposalSummary,
    proposal_count: u32,
    proposal_overflow: u32,
    elapsed: std::time::Duration,
}

struct FullChainOutcome {
    reductions: [ReductionResult; CODE_COUNT],
    proposal_count: u32,
    proposal_overflow: u32,
    summary: ProposalSummary,
    admission: AdmissionRecord,
    fixture: FixtureRecord,
    elapsed: std::time::Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AdmissionRecord {
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
struct AdmissionRulesGpu {
    admission_code: u32,
    min_accepted: u32,
    min_max_score: i32,
    max_invalid: u32,
    _pad: u32,
}

struct AdmitOutcome {
    admission: AdmissionRecord,
    elapsed: std::time::Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FixtureRecord {
    record_code: u32,
    source_admission_code: u32,
    accepted_count: u32,
    invalid_count: u32,
    summary_lo: u32,
    summary_hi: i32,
    max_score: i32,
    priority: i32,
    tier: u32,
    flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct MappingEntryGpu {
    admission_code: u32,
    record_code: u32,
    priority: i32,
    tier: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct MappingParamsGpu {
    mapping_count: u32,
    _pad: [u32; 3],
}

struct FixtureOutcome {
    fixture: FixtureRecord,
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

fn default_admitted_table() -> [u32; ADMITTED_TABLE_SIZE] {
    let mut table = [0u32; ADMITTED_TABLE_SIZE];
    table[0] = 1001;
    table[1] = 1002;
    table[2] = 1003;
    table[3] = 2001;
    table[4] = 2002;
    table
}

fn admitted_count() -> u32 {
    5
}

fn default_admission_rules() -> AdmissionRulesGpu {
    AdmissionRulesGpu {
        admission_code: 5001,
        min_accepted: 1,
        min_max_score: 0,
        max_invalid: 10,
        _pad: 0,
    }
}

fn smoke_admission_rules() -> AdmissionRulesGpu {
    AdmissionRulesGpu {
        admission_code: 5001,
        min_accepted: 1,
        min_max_score: 0,
        max_invalid: 100,
        _pad: 0,
    }
}

fn default_mapping_table() -> [MappingEntryGpu; MAPPING_TABLE_SIZE] {
    let mut table = [MappingEntryGpu {
        admission_code: 0,
        record_code: 0,
        priority: 0,
        tier: 0,
    }; MAPPING_TABLE_SIZE];
    table[0] = MappingEntryGpu {
        admission_code: 5001,
        record_code: 9001,
        priority: 100,
        tier: 1,
    };
    table[1] = MappingEntryGpu {
        admission_code: 5002,
        record_code: 9002,
        priority: 200,
        tier: 2,
    };
    table[2] = MappingEntryGpu {
        admission_code: 5003,
        record_code: 9003,
        priority: 300,
        tier: 3,
    };
    table
}

fn mapping_count() -> u32 {
    3
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

fn emit_fixture_wgsl() -> &'static str {
    r#"
const ADM_ADMITTED: u32 = 1u;
const ADM_INPUT_OVF: u32 = 16u;
const ADM_SUM_OVF: u32 = 32u;
const FIX_EMITTED: u32 = 1u;
const FIX_REJ_NOT_ADMITTED: u32 = 2u;
const FIX_REJ_UNKNOWN: u32 = 4u;
const FIX_INPUT_OVF: u32 = 8u;
const FIX_SUM_OVF: u32 = 16u;

struct LookupEntry {
    admission_code: u32,
    record_code: u32,
    priority: i32,
    tier: u32,
}

struct Params {
    lookup_count: u32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
}

@group(0) @binding(0) var<storage, read> admission_record: array<u32>;
@group(0) @binding(1) var<storage, read_write> fixture_record: array<u32>;
@group(0) @binding(2) var<storage, read> lookup_table: array<LookupEntry>;
@group(0) @binding(3) var<uniform> params: Params;

@compute @workgroup_size(1)
fn fixture_pass(@builtin(global_invocation_id) gid: vec3<u32>) {
    if (gid.x != 0u) { return; }
    let admission_code = admission_record[0];
    let accepted = admission_record[1];
    let invalid = admission_record[2];
    let sum_lo = admission_record[3];
    let sum_hi = bitcast<i32>(admission_record[4]);
    let max_score = bitcast<i32>(admission_record[5]);
    let adm_flags = admission_record[6];
    var flags = 0u;
    if ((adm_flags & ADM_INPUT_OVF) != 0u) { flags = flags | FIX_INPUT_OVF; }
    if ((adm_flags & ADM_SUM_OVF) != 0u) { flags = flags | FIX_SUM_OVF; }
    var record_code = 0u;
    var priority = 0i;
    var tier = 0u;
    if ((adm_flags & ADM_ADMITTED) == 0u) {
        flags = flags | FIX_REJ_NOT_ADMITTED;
    } else if ((flags & (FIX_INPUT_OVF | FIX_SUM_OVF)) != 0u) {
        // overflow blocks emission
    } else {
        var found = false;
        for (var i = 0u; i < params.lookup_count; i = i + 1u) {
            let entry = lookup_table[i];
            if (entry.admission_code == admission_code) {
                record_code = entry.record_code;
                priority = entry.priority;
                tier = entry.tier;
                found = true;
                break;
            }
        }
        if (found) {
            flags = flags | FIX_EMITTED;
        } else {
            flags = flags | FIX_REJ_UNKNOWN;
        }
    }
    fixture_record[0] = record_code;
    fixture_record[1] = admission_code;
    fixture_record[2] = accepted;
    fixture_record[3] = invalid;
    fixture_record[4] = sum_lo;
    fixture_record[5] = bitcast<u32>(sum_hi);
    fixture_record[6] = bitcast<u32>(max_score);
    fixture_record[7] = bitcast<u32>(priority);
    fixture_record[8] = tier;
    fixture_record[9] = flags;
}
"#
}

fn is_admitted(code: u32, table: &[u32; ADMITTED_TABLE_SIZE], count: u32) -> bool {
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

fn cpu_consume(
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
        if is_admitted(prop.proposal_code, admitted, admitted_n) {
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

fn summary_eq(got: ProposalSummary, exp: ProposalSummary) -> bool {
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

fn cpu_admit(summary: ProposalSummary, rules: &AdmissionRulesGpu) -> AdmissionRecord {
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

fn admission_eq(got: AdmissionRecord, exp: AdmissionRecord) -> bool {
    got.admission_code == exp.admission_code
        && got.accepted_count == exp.accepted_count
        && got.invalid_count == exp.invalid_count
        && got.summary_lo == exp.summary_lo
        && got.summary_hi == exp.summary_hi
        && got.max_score == exp.max_score
        && got.flags == exp.flags
}

fn pack_admission(admission: AdmissionRecord) -> [u32; 7] {
    [
        admission.admission_code,
        admission.accepted_count,
        admission.invalid_count,
        admission.summary_lo,
        bytemuck::cast(admission.summary_hi),
        bytemuck::cast(admission.max_score),
        admission.flags,
    ]
}

fn cpu_fixture(
    admission: AdmissionRecord,
    table: &[MappingEntryGpu; MAPPING_TABLE_SIZE],
    count: u32,
) -> FixtureRecord {
    let mut flags = 0u32;
    if admission.flags & FLAG_ADM_INPUT_OVF != 0 {
        flags |= FLAG_FIX_INPUT_OVF;
    }
    if admission.flags & FLAG_ADM_SUM_OVF != 0 {
        flags |= FLAG_FIX_SUM_OVF;
    }
    let mut record_code = 0u32;
    let mut priority = 0i32;
    let mut tier = 0u32;
    if admission.flags & FLAG_ADM_ADMITTED == 0 {
        flags |= FLAG_FIX_REJ_NOT_ADMITTED;
    } else if flags & (FLAG_FIX_INPUT_OVF | FLAG_FIX_SUM_OVF) != 0 {
        // overflow blocks emission
    } else if let Some(entry) = table
        .iter()
        .take(count as usize)
        .find(|e| e.admission_code == admission.admission_code)
    {
        record_code = entry.record_code;
        priority = entry.priority;
        tier = entry.tier;
        flags |= FLAG_FIX_EMITTED;
    } else {
        flags |= FLAG_FIX_REJ_UNKNOWN;
    }
    FixtureRecord {
        record_code,
        source_admission_code: admission.admission_code,
        accepted_count: admission.accepted_count,
        invalid_count: admission.invalid_count,
        summary_lo: admission.summary_lo,
        summary_hi: admission.summary_hi,
        max_score: admission.max_score,
        priority,
        tier,
        flags,
    }
}

fn decode_fixture(words: &[u32]) -> FixtureRecord {
    FixtureRecord {
        record_code: words[0],
        source_admission_code: words[1],
        accepted_count: words[2],
        invalid_count: words[3],
        summary_lo: words[4],
        summary_hi: bytemuck::cast(words[5]),
        max_score: bytemuck::cast(words[6]),
        priority: bytemuck::cast(words[7]),
        tier: words[8],
        flags: words[9],
    }
}

fn fixture_eq(got: FixtureRecord, exp: FixtureRecord) -> bool {
    got.record_code == exp.record_code
        && got.source_admission_code == exp.source_admission_code
        && got.accepted_count == exp.accepted_count
        && got.invalid_count == exp.invalid_count
        && got.summary_lo == exp.summary_lo
        && got.summary_hi == exp.summary_hi
        && got.max_score == exp.max_score
        && got.priority == exp.priority
        && got.tier == exp.tier
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
        label: Some("field_policy_act1_consume"),
        source: wgpu::ShaderSource::Wgsl(emit_consume_wgsl().into()),
    });
    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("field_policy_act1_consume_bgl"),
        entries: &[
            storage_ro(0),
            storage_ro(1),
            storage_ro(2),
            storage_rw(3),
            uniform_entry(4),
        ],
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("field_policy_act1_consume"),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("field_policy_act1_consume_pl"),
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
        label: Some("field_policy_act2_admit"),
        source: wgpu::ShaderSource::Wgsl(emit_admit_wgsl().into()),
    });
    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("field_policy_act2_admit_bgl"),
        entries: &[storage_ro(0), storage_rw(1), uniform_entry(2)],
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("field_policy_act2_admit"),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("field_policy_act2_admit_pl"),
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

fn run_fixture_gpu(
    ctx: &GpuContext,
    admission: AdmissionRecord,
    table: &[MappingEntryGpu; MAPPING_TABLE_SIZE],
    mapping_n: u32,
    repeat_dispatches: u32,
) -> FixtureOutcome {
    use wgpu::util::DeviceExt;
    let device = &ctx.device;
    let queue = &ctx.queue;
    let admission_packed = pack_admission(admission);
    let mparams = MappingParamsGpu {
        mapping_count: mapping_n,
        _pad: [0, 0, 0],
    };

    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("field_policy_act3_fixture"),
        source: wgpu::ShaderSource::Wgsl(emit_fixture_wgsl().into()),
    });
    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("field_policy_act3_fixture_bgl"),
        entries: &[
            storage_ro(0),
            storage_rw(1),
            storage_ro(2),
            uniform_entry(3),
        ],
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("field_policy_act3_fixture"),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("field_policy_act3_fixture_pl"),
                bind_group_layouts: &[&bgl],
                push_constant_ranges: &[],
            }),
        ),
        module: &module,
        entry_point: "fixture_pass",
        compilation_options: Default::default(),
        cache: None,
    });

    let admission_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("admission_in"),
        contents: bytemuck::cast_slice(&admission_packed),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let fixture_init = vec![0u32; FIXTURE_STRIDE as usize];
    let fixture_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("fixture"),
        contents: bytemuck::cast_slice(&fixture_init),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });
    let mapping_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("mapping"),
        contents: bytemuck::cast_slice(table),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let mparams_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("mparams"),
        contents: bytemuck::bytes_of(&mparams),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("fixture_bg"),
        layout: &bgl,
        entries: &[
            bind_entry(0, &admission_buf),
            bind_entry(1, &fixture_buf),
            bind_entry(2, &mapping_buf),
            bind_entry(3, &mparams_buf),
        ],
    });

    let t0 = Instant::now();
    for _ in 0..repeat_dispatches {
        let mut enc =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("fixture"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &bg, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        queue.submit(Some(enc.finish()));
    }
    let elapsed = t0.elapsed();

    let staging = staging_buf(device, (FIXTURE_STRIDE * 4) as u64);
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    enc2.copy_buffer_to_buffer(&fixture_buf, 0, &staging, 0, (FIXTURE_STRIDE * 4) as u64);
    queue.submit(Some(enc2.finish()));
    let words = read_u32s(device, &staging, FIXTURE_STRIDE as usize);
    FixtureOutcome {
        fixture: decode_fixture(&words),
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

fn run_bucket_reduce_propose_consume_gpu(
    ctx: &GpuContext,
    compact_records: &[EventRecord],
    capacity_per_code: u32,
    rules: &[ProposalRuleGpu; CODE_COUNT],
    proposal_capacity: u32,
    admitted: &[u32; ADMITTED_TABLE_SIZE],
    admitted_n: u32,
    admission_rules: &AdmissionRulesGpu,
    mapping_table: &[MappingEntryGpu; MAPPING_TABLE_SIZE],
    mapping_n: u32,
    repeat_dispatches: u32,
) -> FullChainOutcome {
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
    let consume_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("consume"),
        source: wgpu::ShaderSource::Wgsl(emit_consume_wgsl().into()),
    });
    let admit_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("admit"),
        source: wgpu::ShaderSource::Wgsl(emit_admit_wgsl().into()),
    });
    let fixture_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("fixture"),
        source: wgpu::ShaderSource::Wgsl(emit_fixture_wgsl().into()),
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
    let fixture_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("fixture_bgl"),
        entries: &[
            storage_ro(0),
            storage_rw(1),
            storage_ro(2),
            uniform_entry(3),
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
    let consume_pipe = mk_pipe(&consume_module, &consume_bgl, "consume_pass");
    let admit_pipe = mk_pipe(&admit_module, &admit_bgl, "admit_pass");
    let fixture_pipe = mk_pipe(&fixture_module, &fixture_bgl, "fixture_pass");
    let mparams = MappingParamsGpu {
        mapping_count: mapping_n,
        _pad: [0, 0, 0],
    };
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
    let fixture_init = vec![0u32; FIXTURE_STRIDE as usize];
    let fixture_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("fixture"),
        contents: bytemuck::cast_slice(&fixture_init),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });
    let mapping_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("mapping"),
        contents: bytemuck::cast_slice(mapping_table),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let mparams_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("mparams"),
        contents: bytemuck::bytes_of(&mparams),
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
    let bg_fixture = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("bg_fixture"),
        layout: &fixture_bgl,
        entries: &[
            bind_entry(0, &admission_buf),
            bind_entry(1, &fixture_buf),
            bind_entry(2, &mapping_buf),
            bind_entry(3, &mparams_buf),
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
        {
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("fixture"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&fixture_pipe);
            pass.set_bind_group(0, &bg_fixture, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        queue.submit(Some(enc.finish()));
    }
    let elapsed = t0.elapsed();

    let red_staging = staging_buf(device, (CODE_COUNT as u32 * RED_OUT_STRIDE * 4) as u64);
    let meta_staging = staging_buf(device, 8);
    let sum_staging = staging_buf(device, (SUMMARY_STRIDE * 4) as u64);
    let admit_staging = staging_buf(device, (ADMIT_STRIDE * 4) as u64);
    let fixture_staging = staging_buf(device, (FIXTURE_STRIDE * 4) as u64);
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
    enc2.copy_buffer_to_buffer(
        &fixture_buf,
        0,
        &fixture_staging,
        0,
        (FIXTURE_STRIDE * 4) as u64,
    );
    queue.submit(Some(enc2.finish()));

    let red_vec = read_u32s(device, &red_staging, CODE_COUNT * RED_OUT_STRIDE as usize);
    let meta = read_u32s(device, &meta_staging, 2);
    let sum_vec = read_u32s(device, &sum_staging, SUMMARY_STRIDE as usize);
    let admit_vec = read_u32s(device, &admit_staging, ADMIT_STRIDE as usize);
    let fixture_vec = read_u32s(device, &fixture_staging, FIXTURE_STRIDE as usize);
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
    let admission = decode_admission(&admit_vec);
    FullChainOutcome {
        reductions,
        proposal_count: meta[0],
        proposal_overflow: meta[1],
        summary,
        admission,
        fixture: decode_fixture(&fixture_vec),
        elapsed,
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

fn edge_fixture_cases() -> Vec<(AdmissionRecord, &'static str)> {
    let table = default_mapping_table();
    let admitted_known = cpu_admit(
        ProposalSummary {
            accepted_count: 3,
            ignored_count: 0,
            invalid_count: 0,
            summary_lo: 100,
            summary_hi: 0,
            max_score: 500,
            flags: 0,
        },
        &AdmissionRulesGpu {
            admission_code: 5001,
            min_accepted: 1,
            min_max_score: 0,
            max_invalid: 10,
            _pad: 0,
        },
    );
    let mut unknown_code = admitted_known;
    unknown_code.admission_code = 5099;
    unknown_code.flags = FLAG_ADM_ADMITTED;
    let mut not_admitted = admitted_known;
    not_admitted.flags = FLAG_ADM_REJ_COUNT;
    let mut input_ovf = admitted_known;
    input_ovf.flags = FLAG_ADM_INPUT_OVF;
    let mut sum_ovf = admitted_known;
    sum_ovf.flags = FLAG_ADM_SUM_OVF;
    let mut zero_accepted = admitted_known;
    zero_accepted.accepted_count = 0;
    let mut large_accepted = admitted_known;
    large_accepted.accepted_count = 1_000_000;
    let mut neg_score = admitted_known;
    neg_score.max_score = -100;
    let mut large_score = admitted_known;
    large_score.max_score = 2_000_000;
    let mut code5002 = admitted_known;
    code5002.admission_code = 5002;
    let _ = table;
    vec![
        (admitted_known, "admitted_known_code"),
        (unknown_code, "admitted_unknown_code"),
        (not_admitted, "not_admitted"),
        (input_ovf, "input_overflow"),
        (sum_ovf, "summary_overflow"),
        (zero_accepted, "zero_accepted"),
        (large_accepted, "large_accepted"),
        (neg_score, "negative_max_score"),
        (large_score, "large_positive_score"),
        (code5002, "mapped_code_5002"),
    ]
}

fn dense_admissions() -> Vec<AdmissionRecord> {
    (0..64)
        .map(|idx| {
            let code = 5001 + (idx % 4) as u32;
            let mut flags = FLAG_ADM_ADMITTED;
            if idx % 13 == 0 {
                flags = FLAG_ADM_REJ_COUNT;
            } else if idx % 17 == 0 {
                flags = FLAG_ADM_INPUT_OVF;
            } else if idx % 19 == 0 {
                flags = FLAG_ADM_SUM_OVF;
            } else if idx % 7 == 0 {
                flags = FLAG_ADM_ADMITTED;
            }
            AdmissionRecord {
                admission_code: code,
                accepted_count: 1 + idx as u32 % 5,
                invalid_count: idx as u32 % 3,
                summary_lo: (idx * 41) as u32,
                summary_hi: 0,
                max_score: (idx as i32).wrapping_mul(97),
                flags,
            }
        })
        .collect()
}

fn verify_chain_fixture(
    outcome: &FullChainOutcome,
    compact: &[EventRecord],
    cap: u32,
    rules: &[ProposalRuleGpu; CODE_COUNT],
    prop_cap: u32,
    admitted: &[u32; ADMITTED_TABLE_SIZE],
    admitted_n: u32,
    admission_rules: &AdmissionRulesGpu,
    mapping_table: &[MappingEntryGpu; MAPPING_TABLE_SIZE],
    mapping_n: u32,
) {
    let (buckets, counts) = cpu_bucket_from_compact(compact, cap);
    let exp_reds = reductions_from_buckets(&buckets, counts, cap);
    let (exp_count, exp_ovf, exp_props) = cpu_propose(&exp_reds, rules, prop_cap);
    let exp_summary = cpu_consume(
        exp_count, exp_ovf, &exp_props, prop_cap, admitted, admitted_n,
    );
    let exp_admission = cpu_admit(exp_summary, admission_rules);
    let exp_fixture = cpu_fixture(exp_admission, mapping_table, mapping_n);
    assert_eq!(outcome.proposal_count, exp_count);
    assert_eq!(outcome.proposal_overflow, exp_ovf);
    assert!(summary_eq(outcome.summary, exp_summary));
    assert!(admission_eq(outcome.admission, exp_admission));
    assert!(fixture_eq(outcome.fixture, exp_fixture));
}

#[test]
fn field_policy_act3_wgsl_semantic_free() {
    let wgsl = emit_fixture_wgsl();
    for term in FORBIDDEN_SEMANTIC_TERMS {
        assert!(!wgsl.contains(term), "forbidden `{term}`");
    }
    for term in FORBIDDEN_EXACT_TERMS {
        assert!(!wgsl.contains(term));
    }
    assert!(wgsl.contains("fixture_pass"));
    assert!(wgsl.contains("fixture_record"));
    assert!(!wgsl.contains("scheduler"));
    println!("field_policy_act3_wgsl: semantic_free=true ordering={ORDERING_CLASS}");
}

#[test]
fn field_policy_act3_fixture_record_edge_rows() {
    let table = default_mapping_table();
    let mapping_n = mapping_count();
    with_gpu(|ctx| {
        for (admission, label) in edge_fixture_cases() {
            let outcome = run_fixture_gpu(ctx, admission, &table, mapping_n, 1);
            let exp = cpu_fixture(admission, &table, mapping_n);
            assert!(fixture_eq(outcome.fixture, exp), "{label}");
            println!(
                "field_policy_act3_edge[{label}]: record_code={} flags={} emitted={} ordering={ORDERING_CLASS}",
                outcome.fixture.record_code,
                outcome.fixture.flags,
                outcome.fixture.flags & FLAG_FIX_EMITTED != 0,
            );
        }
    });
}

#[test]
fn field_policy_act3_dense_fixture_record_corpus() {
    let table = default_mapping_table();
    let mapping_n = mapping_count();
    with_gpu(|ctx| {
        for (idx, admission) in dense_admissions().into_iter().enumerate() {
            let outcome = run_fixture_gpu(ctx, admission, &table, mapping_n, 1);
            let exp = cpu_fixture(admission, &table, mapping_n);
            assert!(fixture_eq(outcome.fixture, exp), "dense[{idx}]");
        }
        println!(
            "field_policy_act3_dense: rows={} ordering={ORDERING_CLASS}",
            64
        );
    });
}

#[test]
fn field_policy_act3_act2_to_fixture_record_smoke() {
    let rules = rules_for_smoke();
    let admitted = default_admitted_table();
    let admitted_n = admitted_count();
    let adm_rules = smoke_admission_rules();
    let mapping = default_mapping_table();
    let mapping_n = mapping_count();
    with_gpu(|ctx| {
        let mut compact = Vec::new();
        for idx in 0..256u32 {
            let code = 1 + (idx % 3);
            compact.push(rec(idx, code, idx % 2, (idx as i32) * 100));
        }
        let cap = 256u32;
        let prop_cap = 64u32;
        let outcome = run_bucket_reduce_propose_consume_gpu(
            ctx, &compact, cap, &rules, prop_cap, &admitted, admitted_n, &adm_rules, &mapping,
            mapping_n, 1,
        );
        verify_chain_fixture(
            &outcome, &compact, cap, &rules, prop_cap, &admitted, admitted_n, &adm_rules, &mapping,
            mapping_n,
        );
        println!(
            "field_policy_act3_act2_smoke: admission_code={} admission_flags={} record_code={} record_flags={} priority={} tier={} dispatches=6 ordering={ORDERING_CLASS}",
            outcome.admission.admission_code,
            outcome.admission.flags,
            outcome.fixture.record_code,
            outcome.fixture.flags,
            outcome.fixture.priority,
            outcome.fixture.tier,
        );
    });
}

#[test]
fn field_policy_act3_full_chain_fixture_record_smoke() {
    let compact = pipe_compact_corpus();
    let rules = rules_for_smoke();
    let admitted = default_admitted_table();
    let admitted_n = admitted_count();
    let adm_rules = smoke_admission_rules();
    let mapping = default_mapping_table();
    let mapping_n = mapping_count();
    with_gpu(|ctx| {
        let outcome = run_bucket_reduce_propose_consume_gpu(
            ctx, &compact, 512, &rules, 64, &admitted, admitted_n, &adm_rules, &mapping, mapping_n,
            1,
        );
        verify_chain_fixture(
            &outcome, &compact, 512, &rules, 64, &admitted, admitted_n, &adm_rules, &mapping,
            mapping_n,
        );
        let (_, counts) = cpu_bucket_from_compact(&compact, 512);
        println!(
            "field_policy_act3_full_chain: compact={} event_count={} bucket_counts={counts:?} proposal_count={} accepted={} admission_code={} record_code={} record_flags={} overflow={} ordering={ORDERING_CLASS}",
            compact.len(),
            compact.len(),
            outcome.proposal_count,
            outcome.summary.accepted_count,
            outcome.admission.admission_code,
            outcome.fixture.record_code,
            outcome.fixture.flags,
            outcome.admission.flags & (FLAG_ADM_INPUT_OVF | FLAG_ADM_SUM_OVF),
        );
    });
}

const N: usize = 34_000;
const REPEATS: u32 = 32;

#[test]
fn field_policy_act3_perf_34k_fixture_records() {
    let records = balanced_12_records(N);
    let rules = rules_for_smoke();
    let admitted = default_admitted_table();
    let admitted_n = admitted_count();
    let adm_rules = smoke_admission_rules();
    let mapping = default_mapping_table();
    let mapping_n = mapping_count();
    with_gpu(|ctx| {
        let outcome = run_bucket_reduce_propose_consume_gpu(
            ctx, &records, 4096, &rules, 64, &admitted, admitted_n, &adm_rules, &mapping,
            mapping_n, 1,
        );
        let elapsed_ms = outcome.elapsed.as_secs_f64() * 1000.0;
        let per_record_us = elapsed_ms * 1000.0 / N as f64;
        println!(
            "field_policy_act3_34k: dispatches=6 elapsed_ms={elapsed_ms:.3} readback=true event_count={N} proposal_count={} accepted={} admission_code={} record_code={} overflow={} per_record_us={per_record_us:.4} ordering={ORDERING_CLASS}",
            outcome.proposal_count,
            outcome.summary.accepted_count,
            outcome.admission.admission_code,
            outcome.fixture.record_code,
            outcome.admission.flags & (FLAG_ADM_INPUT_OVF | FLAG_ADM_SUM_OVF),
        );
    });
}

#[test]
fn field_policy_act3_perf_34k_warm_repeated_dispatch() {
    let records = balanced_12_records(N);
    let rules = rules_for_smoke();
    let admitted = default_admitted_table();
    let admitted_n = admitted_count();
    let adm_rules = smoke_admission_rules();
    let mapping = default_mapping_table();
    let mapping_n = mapping_count();
    with_gpu(|ctx| {
        let outcome = run_bucket_reduce_propose_consume_gpu(
            ctx, &records, 4096, &rules, 64, &admitted, admitted_n, &adm_rules, &mapping,
            mapping_n, REPEATS,
        );
        let total_ms = outcome.elapsed.as_secs_f64() * 1000.0;
        let per_pipeline_ms = total_ms / REPEATS as f64;
        let per_record_us = total_ms * 1000.0 / (N as f64 * REPEATS as f64);
        println!(
            "field_policy_act3_34k_warm: repeats={REPEATS} total_ms={total_ms:.3} per_pipeline_ms={per_pipeline_ms:.4} per_record_us={per_record_us:.4} record_code={} overflow={} ordering={ORDERING_CLASS}",
            outcome.fixture.record_code,
            outcome.admission.flags & (FLAG_ADM_INPUT_OVF | FLAG_ADM_SUM_OVF),
        );
    });
}

#[test]
fn field_policy_act3_no_default_runtime_wiring() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    let desc = landed_jit_kernel_descriptors()
        .into_iter()
        .find(|d| d.id == FIELD_POLICY_ACT3_DESCRIPTOR_ID)
        .expect("act3 descriptor");
    assert!(desc.default_off);
    assert!(!desc.production_wiring);
    assert!(is_field_policy_act3_economic_fixture_records_descriptor(
        &desc
    ));
    validate_kernel_descriptor_admission(&desc).expect("act3 admits");
    for out in &desc.writes {
        assert_eq!(out.authority, OutputAuthority::ExactAuthoritative);
    }
    let _ = PhaseEEconomicFixtureRecordAuthority::ExactAuthoritative;
    println!(
        "field_policy_act3_wiring: default_off=true descriptor={FIELD_POLICY_ACT3_DESCRIPTOR_ID}"
    );
}

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
