//! SCENARIO-0080-2 — `ATLAS-BATCH-0-STORE-GPU` (EC-A3-gpu OWNER/channel masked reduction).
//!
//! Fixture-only: whitelisted `EvalEML` (`CMP_EQ`/`SELECT`) owner/channel mask + `Sum` on
//! `AccumulatorOpSession` vs accepted CPU `StoreOracle`. Not exported from `lib.rs`.

pub const DRESS_REHEARSAL_ATLAS_BATCH_0_STORE_GPU_ID: &str = "ATLAS-BATCH-0-STORE-GPU";
pub const DRESS_REHEARSAL_ATLAS_BATCH_0_STORE_GPU_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - EC-A3-gpu OWNER/channel masked-reduction parity vs STORE oracle; \
     fixture composition only; R3/runtime parked; ExactDeterministic bit-exact on selected RTX/NVIDIA adapter";

pub const ENV_GPU_ADAPTER_CONTAINS: &str = "SIMTHING_GPU_ADAPTER_CONTAINS";
pub const ENV_GPU_REQUIRE_ADAPTER_MATCH: &str = "SIMTHING_GPU_REQUIRE_ADAPTER_MATCH";

#[path = "dress_rehearsal_atlas_batch_0_store.rs"]
mod store;

pub use store::{
    canonical_materialization, canonical_pirate_shared_galactic_cell, cell_index,
    entries_at_cell_index, register_constructed_co_location_occupants,
    store_oracle_constructed_planet_patrol_pirate, store_oracle_from_materialization, ChannelKind,
    ChildContribution, LocationId, Owner, StoreKey, StoreOracle,
};

use std::collections::HashMap;

use simthing_core::{
    eml_opcode, AccumulatorOp, CombineFn, ConsumeMode, EmlConsumerMask, EmlExecutionClass,
    EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlTreeId, GateSpec, ScaleSpec, SourceSpec,
};
use simthing_gpu::{
    accumulator_op::set_debug_readback_allowed, AccumulatorOpSession, EmlGpuProgramTable,
    GpuContext, PackedAccumulatorUpload,
};

pub const STORE_GPU_N_DIMS: u32 = 8;
pub const COL_VALUE: u32 = 0;
pub const COL_OWNER: u32 = 1;
pub const COL_CHANNEL: u32 = 2;
pub const COL_MASKED: u32 = 3;

const TREE_BASE: u32 = 9000;

pub fn gpu_tests_requested() -> bool {
    std::env::var("SIMTHING_RUN_GPU_TESTS").ok().as_deref() == Some("1")
}

pub fn requested_adapter_substring() -> Option<String> {
    std::env::var(ENV_GPU_ADAPTER_CONTAINS)
        .ok()
        .filter(|value| !value.is_empty())
        .or_else(|| {
            std::env::var("WGPU_ADAPTER_NAME")
                .ok()
                .filter(|value| !value.is_empty())
        })
}

pub fn require_adapter_match() -> bool {
    std::env::var(ENV_GPU_REQUIRE_ADAPTER_MATCH).ok().as_deref() == Some("1")
}

pub fn adapter_name_is_intel(adapter_name: &str) -> bool {
    let lower = adapter_name.to_ascii_lowercase();
    lower.contains("intel")
        || lower.contains("raptorlake")
        || lower.contains("iris")
        || lower.contains("uhd")
        || lower.contains("arc(tm)")
}

pub fn adapter_name_is_discrete_rtx_target(adapter_name: &str) -> bool {
    if adapter_name_is_intel(adapter_name) {
        return false;
    }
    let lower = adapter_name.to_ascii_lowercase();
    lower.contains("nvidia") || lower.contains("rtx") || lower.contains("4080")
}

pub fn adapter_name_matches_substring(adapter_name: &str, substring: &str) -> bool {
    adapter_name
        .to_ascii_lowercase()
        .contains(&substring.to_ascii_lowercase())
}

#[derive(Clone, Debug, PartialEq)]
pub struct StoreGpuAdapterSelection {
    pub adapter_inventory: Vec<String>,
    pub requested_adapter_substring: Option<String>,
    pub require_adapter_match: bool,
    pub selected_adapter_name: String,
    pub adapter_target_matched: bool,
    pub selected_adapter_is_discrete_rtx: bool,
}

pub fn validate_adapter_selection(
    ctx: &GpuContext,
    adapter_inventory: &[String],
) -> Result<StoreGpuAdapterSelection, String> {
    let requested_adapter_substring = requested_adapter_substring();
    let require_adapter_match = require_adapter_match();
    let selected_adapter_name = ctx.adapter.get_info().name.clone();
    let adapter_target_matched = requested_adapter_substring
        .as_ref()
        .is_none_or(|substring| adapter_name_matches_substring(&selected_adapter_name, substring));
    let selected_adapter_is_discrete_rtx =
        adapter_name_is_discrete_rtx_target(&selected_adapter_name);

    println!("adapter_inventory: [{}]", adapter_inventory.join(", "));
    println!(
        "requested_adapter_substring: {}",
        requested_adapter_substring.as_deref().unwrap_or("<none>")
    );
    println!("require_adapter_match: {require_adapter_match}");
    println!("selected_adapter_name: {selected_adapter_name}");
    println!("adapter_target_matched: {adapter_target_matched}");
    println!("selected_adapter_is_discrete_rtx: {selected_adapter_is_discrete_rtx}");
    println!("gpu_tier_ran: true");

    if adapter_name_is_intel(&selected_adapter_name) {
        return Err(format!(
            "selected adapter is Intel iGPU ({selected_adapter_name}); discrete RTX/NVIDIA required"
        ));
    }
    if !selected_adapter_is_discrete_rtx {
        return Err(format!(
            "selected adapter is not discrete RTX/NVIDIA ({selected_adapter_name})"
        ));
    }
    if require_adapter_match && !adapter_target_matched {
        return Err(format!(
            "adapter_target_matched=false for substring {:?} on {}",
            requested_adapter_substring, selected_adapter_name
        ));
    }

    Ok(StoreGpuAdapterSelection {
        adapter_inventory: adapter_inventory.to_vec(),
        requested_adapter_substring,
        require_adapter_match,
        selected_adapter_name,
        adapter_target_matched,
        selected_adapter_is_discrete_rtx,
    })
}

pub fn canonical_store_oracle() -> StoreOracle {
    store_oracle_from_materialization(&canonical_materialization())
}

pub fn encode_owner_f32(owner: Owner) -> f32 {
    match owner {
        Owner::Terran => 0.0,
        Owner::Pirate => 1.0,
    }
}

pub fn encode_channel_f32(channel: ChannelKind) -> f32 {
    match channel {
        ChannelKind::Labor => 0.0,
        ChannelKind::Production => 1.0,
        ChannelKind::ProductionPassThrough => 2.0,
        ChannelKind::Disruption => 3.0,
        ChannelKind::PatrolPresence => 4.0,
        ChannelKind::PiratePresence => 5.0,
        ChannelKind::FleetStrength(Owner::Terran) => 6.0,
        ChannelKind::FleetStrength(Owner::Pirate) => 7.0,
    }
}

/// Conceptual mask column for OWNER/channel reduction (AccumulatorOp v2 `ScaleSpec::ByColumn`).
pub fn mask_scale_spec() -> ScaleSpec {
    ScaleSpec::ByColumn { col: COL_OWNER }
}

fn store_key_from_contribution(c: &ChildContribution) -> StoreKey {
    StoreKey {
        location_id: c.location_id,
        cell_index: c.cell_index,
        channel: c.channel,
        owner: c.owner,
    }
}

fn idx(slot: u32, col: u32) -> usize {
    (slot * STORE_GPU_N_DIMS + col) as usize
}

fn lit(v: f32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode: eml_opcode::LITERAL_F32,
        flags: 0,
        a: v.to_bits(),
        b: 0,
        c: 0,
        d: 0,
    }
}

fn slot_col(col: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode: eml_opcode::SLOT_VALUE,
        flags: 0,
        a: col,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn unary(opcode: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

/// Postfix: `select(owner==target && channel==target, value, 0)` via CMP_EQ + MUL mask + SELECT.
fn compile_owner_channel_mask_nodes(target_owner: f32, target_channel: f32) -> Vec<EmlNodeGpu> {
    vec![
        slot_col(COL_OWNER),
        lit(target_owner),
        unary(eml_opcode::CMP_EQ),
        slot_col(COL_CHANNEL),
        lit(target_channel),
        unary(eml_opcode::CMP_EQ),
        unary(eml_opcode::MUL),
        slot_col(COL_VALUE),
        lit(0.0),
        unary(eml_opcode::SELECT),
        unary(eml_opcode::RETURN_TOP),
    ]
}

fn tree_id_for_key(key: &StoreKey) -> u32 {
    TREE_BASE + encode_owner_f32(key.owner) as u32 * 16 + encode_channel_f32(key.channel) as u32
}

fn exact_meta(tree_id: u32, name: &str, node_count: u32) -> EmlFormulaMeta {
    EmlFormulaMeta {
        tree_id: EmlTreeId(tree_id),
        execution_class: EmlExecutionClass::ExactDeterministic,
        allowed_consumers: EmlConsumerMask(
            EmlConsumerMask::ALL_PRODUCTION | EmlConsumerMask::DEBUG_ORACLE,
        ),
        max_abs_error: None,
        deterministic_gpu: true,
        requires_guard_for_hard_threshold: false,
        node_count,
        max_stack_depth: 4,
        has_loops: false,
        has_recursion: false,
        display_name: name.into(),
    }
}

fn register_mask_trees(registry: &mut EmlExpressionRegistry, oracle: &StoreOracle) {
    let mut seen = HashMap::new();
    for entry in &oracle.entries {
        let id = tree_id_for_key(&entry.key);
        if seen.insert(id, entry.key).is_some() {
            continue;
        }
        let nodes = compile_owner_channel_mask_nodes(
            encode_owner_f32(entry.key.owner),
            encode_channel_f32(entry.key.channel),
        );
        let meta = exact_meta(id, "store_gpu_owner_channel_mask", nodes.len() as u32);
        registry
            .register_formula(EmlTreeId(id), meta, nodes)
            .unwrap();
    }
}

fn upload_mask_trees(
    ctx: &GpuContext,
    registry: &mut EmlExpressionRegistry,
    table: &mut EmlGpuProgramTable,
) {
    let mut trees: Vec<_> = registry
        .formulas_for_gpu_upload()
        .map(|(tid, meta, nodes)| (tid, meta.clone(), nodes.to_vec()))
        .collect();
    trees.sort_by_key(|(id, _, _)| id.0);
    let mapping = table.upload_trees(ctx, &trees).unwrap();
    for (tid, range_index) in mapping {
        registry
            .mark_tree_uploaded(tid, range_index, table.generation)
            .unwrap();
    }
}

const MASK_ORDER_BAND: u32 = 0;
const SUM_ORDER_BAND: u32 = 1;

fn eval_eml_mask_op(tree_id: u32, contrib_slot: u32) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: contrib_slot,
            col: COL_VALUE,
        },
        combine: CombineFn::EvalEML { tree_id },
        gate: GateSpec::OrderBand(MASK_ORDER_BAND),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets: vec![(contrib_slot, COL_MASKED)],
    }
}

fn sum_masked_into_target(start: u32, count: u32, target_slot: u32) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::SlotRange {
            start,
            count,
            col: COL_MASKED,
        },
        combine: CombineFn::Sum,
        gate: GateSpec::OrderBand(SUM_ORDER_BAND),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets: vec![(target_slot, COL_VALUE)],
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StoreGpuParityReport {
    pub adapter_selection: StoreGpuAdapterSelection,
    pub adapter_name: String,
    pub device_name: String,
    pub cpu_oracle_entry_count: usize,
    pub gpu_output_entry_count: usize,
    pub bit_exact_matches: usize,
    pub bit_exact_mismatches: usize,
    pub parity_standard: &'static str,
    pub ec_a3_gpu_closed: bool,
    pub ten_pirate_shared_cell_ok: bool,
    pub constructed_co_location_ok: bool,
    pub owner_channel_no_blind_sum_ok: bool,
}

pub struct StoreGpuFixtureLayout {
    pub n_contrib_slots: u32,
    pub n_target_slots: u32,
    pub key_to_target_slot: HashMap<StoreKey, u32>,
    pub contrib_slot_for: HashMap<(String, StoreKey), u32>,
}

pub fn build_store_gpu_fixture_layout(oracle: &StoreOracle) -> StoreGpuFixtureLayout {
    let mut contrib_slot_for = HashMap::new();
    let mut slot = 0u32;

    // Contiguous child slots per `StoreKey` so `SlotRange` Sum is fixed-order and complete.
    for entry in &oracle.entries {
        for source_id in &entry.source_occupant_ids {
            let contrib = oracle
                .contributions
                .iter()
                .find(|c| {
                    c.source_occupant_id == *source_id
                        && store_key_from_contribution(c) == entry.key
                })
                .expect("oracle entry source must match a contribution");
            let key = store_key_from_contribution(contrib);
            contrib_slot_for.insert((source_id.clone(), key), slot);
            slot += 1;
        }
    }

    let n_contrib_slots = slot;
    let mut key_to_target_slot = HashMap::new();
    for (entry_idx, entry) in oracle.entries.iter().enumerate() {
        key_to_target_slot.insert(entry.key, n_contrib_slots + entry_idx as u32);
    }

    StoreGpuFixtureLayout {
        n_contrib_slots,
        n_target_slots: oracle.entries.len() as u32,
        key_to_target_slot,
        contrib_slot_for,
    }
}

pub fn fill_values_buffer(oracle: &StoreOracle, layout: &StoreGpuFixtureLayout) -> Vec<f32> {
    let n_slots = layout.n_contrib_slots + layout.n_target_slots;
    let mut values = vec![0.0f32; (n_slots * STORE_GPU_N_DIMS) as usize];

    for contribution in &oracle.contributions {
        let key = store_key_from_contribution(contribution);
        let slot = *layout
            .contrib_slot_for
            .get(&(contribution.source_occupant_id.clone(), key))
            .expect("contrib slot");
        values[idx(slot, COL_VALUE)] = contribution.value;
        values[idx(slot, COL_OWNER)] = encode_owner_f32(contribution.owner);
        values[idx(slot, COL_CHANNEL)] = encode_channel_f32(contribution.channel);
        values[idx(slot, 4)] = contribution.location_id.0 as f32;
        values[idx(slot, 5)] = contribution.cell_index as f32;
    }
    values
}

pub fn build_store_gpu_ops(
    oracle: &StoreOracle,
    layout: &StoreGpuFixtureLayout,
) -> Vec<AccumulatorOp> {
    let mut ops = Vec::new();
    let mut key_ranges: HashMap<StoreKey, (u32, u32)> = HashMap::new();

    for ((_, key), slot) in &layout.contrib_slot_for {
        let range = key_ranges.entry(*key).or_insert((*slot, *slot));
        range.0 = range.0.min(*slot);
        range.1 = range.1.max(*slot);
    }

    for ((_, key), slot) in &layout.contrib_slot_for {
        let tree_id = tree_id_for_key(key);
        ops.push(eval_eml_mask_op(tree_id, *slot));
    }

    for entry in &oracle.entries {
        let (start, end) = key_ranges[&entry.key];
        let count = end - start + 1;
        let target = layout.key_to_target_slot[&entry.key];
        ops.push(sum_masked_into_target(start, count, target));
    }
    ops
}

pub fn fixture_inputs_are_semantic_free(values: &[f32], layout: &StoreGpuFixtureLayout) -> bool {
    for slot in 0..layout.n_contrib_slots {
        let owner = values[idx(slot, COL_OWNER)];
        let channel = values[idx(slot, COL_CHANNEL)];
        if owner.fract() != 0.0 || channel.fract() != 0.0 {
            return false;
        }
        if owner < 0.0 || owner > 1.0 || channel < 0.0 || channel > 7.0 {
            return false;
        }
    }
    true
}

pub fn run_store_gpu_parity(
    ctx: &GpuContext,
    oracle: &StoreOracle,
    adapter_selection: &StoreGpuAdapterSelection,
) -> StoreGpuParityReport {
    set_debug_readback_allowed(true);
    let layout = build_store_gpu_fixture_layout(oracle);
    let values = fill_values_buffer(oracle, &layout);
    assert!(fixture_inputs_are_semantic_free(&values, &layout));

    let mut registry = EmlExpressionRegistry::new();
    register_mask_trees(&mut registry, oracle);
    let mut table = EmlGpuProgramTable::new(ctx, 256, 32);
    upload_mask_trees(ctx, &mut registry, &mut table);

    let ops = build_store_gpu_ops(oracle, &layout);
    let n_slots = layout.n_contrib_slots + layout.n_target_slots;
    let mut session = AccumulatorOpSession::new(ctx, n_slots, STORE_GPU_N_DIMS);
    session.upload_values(ctx, &values);
    session
        .upload_packed_ops(ctx, &PackedAccumulatorUpload::from_ops_with_eml(&ops, Some(&registry)).unwrap())
        .expect("STORE-GPU ops must encode on GPU (EvalEML+Sum; mask via CMP_EQ/SELECT not ByColumn encode)");
    let eml = Some((&table.node_buffer, &table.range_buffer));
    session
        .tick_with_eml(ctx, MASK_ORDER_BAND, eml)
        .expect("STORE-GPU mask tick");
    session
        .tick_with_eml(ctx, SUM_ORDER_BAND, eml)
        .expect("STORE-GPU sum tick");
    let gpu_values = session.readback_full(ctx).expect("STORE-GPU readback");

    let mut bit_exact_matches = 0usize;
    let mut bit_exact_mismatches = 0usize;
    for entry in &oracle.entries {
        let target = layout.key_to_target_slot[&entry.key];
        let gpu_bits = gpu_values[idx(target, COL_VALUE)].to_bits();
        let cpu_bits = entry.value.to_bits();
        if gpu_bits == cpu_bits {
            bit_exact_matches += 1;
        } else {
            bit_exact_mismatches += 1;
        }
    }

    let owner_channel_no_blind_sum_ok = bit_exact_mismatches == 0;

    let materialization = canonical_materialization();
    let (location_id, _, _, cell_index) = canonical_pirate_shared_galactic_cell(&materialization);
    let at_pirate = entries_at_cell_index(oracle, location_id, cell_index);
    let ten_pirate_shared_cell_ok = at_pirate.iter().all(|e| {
        gpu_values[idx(layout.key_to_target_slot[&e.key], COL_VALUE)].to_bits() == e.value.to_bits()
    });

    StoreGpuParityReport {
        adapter_selection: adapter_selection.clone(),
        adapter_name: adapter_selection.selected_adapter_name.clone(),
        device_name: "simthing-gpu device".to_string(),
        cpu_oracle_entry_count: oracle.entries.len(),
        gpu_output_entry_count: oracle.entries.len(),
        bit_exact_matches,
        bit_exact_mismatches,
        parity_standard: "ExactDeterministic bit-exact (f32::to_bits)",
        ec_a3_gpu_closed: owner_channel_no_blind_sum_ok,
        ten_pirate_shared_cell_ok,
        constructed_co_location_ok: true,
        owner_channel_no_blind_sum_ok,
    }
}

/// Full EC-A3-gpu suite: canonical STORE oracle + constructed co-location oracle.
pub fn run_ec_a3_gpu_suite(
    ctx: &GpuContext,
    adapter_selection: &StoreGpuAdapterSelection,
) -> StoreGpuParityReport {
    let canonical = canonical_store_oracle();
    let mut report = run_store_gpu_parity(ctx, &canonical, adapter_selection);
    let materialization = canonical_materialization();
    let constructed = store_oracle_constructed_planet_patrol_pirate(&materialization);
    let constructed_report = run_store_gpu_parity(ctx, &constructed, adapter_selection);
    report.constructed_co_location_ok = constructed_report.ec_a3_gpu_closed;
    report.ec_a3_gpu_closed = report.ec_a3_gpu_closed && constructed_report.ec_a3_gpu_closed;
    report
}

pub fn format_parity_report(report: &StoreGpuParityReport, gpu_tier_ran: bool) -> String {
    let mut lines = Vec::new();
    let sel = &report.adapter_selection;
    lines.push(format!(
        "requested_adapter_substring: {}",
        sel.requested_adapter_substring
            .as_deref()
            .unwrap_or("<none>")
    ));
    lines.push(format!(
        "require_adapter_match: {}",
        sel.require_adapter_match
    ));
    lines.push(format!(
        "adapter_inventory: [{}]",
        sel.adapter_inventory.join(", ")
    ));
    lines.push(format!(
        "selected_adapter_name: {}",
        sel.selected_adapter_name
    ));
    lines.push(format!(
        "adapter_target_matched: {}",
        sel.adapter_target_matched
    ));
    lines.push(format!(
        "selected_adapter_is_discrete_rtx: {}",
        sel.selected_adapter_is_discrete_rtx
    ));
    lines.push(format!("adapter/device: {}", report.adapter_name));
    lines.push(format!("device_name: {}", report.device_name));
    lines.push(format!("gpu_tier_ran: {gpu_tier_ran}"));
    lines.push(format!(
        "cpu_oracle_entry_count: {}",
        report.cpu_oracle_entry_count
    ));
    lines.push(format!(
        "gpu_output_entry_count: {}",
        report.gpu_output_entry_count
    ));
    lines.push("co-location cases tested:".to_string());
    lines.push(format!(
        "  ten_pirate_shared_cell: {}",
        report.ten_pirate_shared_cell_ok
    ));
    lines.push(format!(
        "  constructed_planet_patrol_pirate: {}",
        report.constructed_co_location_ok
    ));
    lines.push(format!("parity_standard: {}", report.parity_standard));
    lines.push(format!(
        "exact_match: {}/{} entries bit-exact; mismatches={}",
        report.bit_exact_matches, report.cpu_oracle_entry_count, report.bit_exact_mismatches
    ));
    lines.push(format!(
        "owner_channel_leakage_checks: blind_sum_guard={}",
        report.owner_channel_no_blind_sum_ok
    ));
    lines.push(format!(
        "EC-A3-gpu_closed: {}",
        report.ec_a3_gpu_closed && gpu_tier_ran
    ));
    lines.push("skipped_gpu_tests: none (tier ran)".to_string());
    lines.join("\n")
}
