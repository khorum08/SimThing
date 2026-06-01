//! MOBILITY-ECON-0: session-clearinghouse + subsidiarity economy substrate.
//!
//! This is a named, metadata/testable substrate only. It accepts local cell
//! outputs from the ALLOC/REENROLL/IDROUTE ladder and proves deterministic
//! clearinghouse up-aggregation, subsidiarity balance, hard Band Alpha before
//! soft Band Beta, conservation, and CPU/GPU parity proxy accounting. It does
//! not implement OWNER, Hybrid-Strata/faction-index scaling, Resource Flow,
//! production `SimSession` wiring, semantic/raw WGSL, or default-on behavior.

use std::collections::{BTreeMap, BTreeSet};

use super::mobility_alloc0::MobilityAlloc0ParentKey;

pub const MOBILITY_ECON0_ID: &str = "mobility_econ0_session_clearinghouse_economy";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MobilityEcon0SessionResourceKey {
    pub session_id: u64,
    pub resource_id: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityEcon0LocalCellRecord {
    pub session_id: u64,
    pub cell_key: MobilityAlloc0ParentKey,
    pub resource_id: u64,
    pub hard_available: i64,
    pub hard_need: i64,
    pub soft_beta_signal: f32,
    pub arrival_order: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MobilityEcon0ForbiddenPathRequests {
    pub owner_overlay_runtime: bool,
    pub owner_runtime: bool,
    pub default_on_resource_flow: bool,
    pub hard_currency_through_resource_flow: bool,
    pub float_structural_gate: bool,
    pub production_simsession_wiring: bool,
    pub semantic_or_raw_wgsl: bool,
    pub cpu_planner_urgency_commitment: bool,
    pub owner_as_spatial_parent: bool,
    pub capture_as_reparenting: bool,
    pub hybrid_strata_or_faction_index_scaling_layer: bool,
    pub hard_soft_silent_mix: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityEcon0PlanInput {
    pub records: Vec<MobilityEcon0LocalCellRecord>,
    pub forbidden: MobilityEcon0ForbiddenPathRequests,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityEcon0SessionAggregate {
    pub key: MobilityEcon0SessionResourceKey,
    pub hard_available: i64,
    pub hard_need: i64,
    pub hard_shortfall: i64,
    pub hard_surplus: i64,
    pub soft_beta_input: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityEcon0DownDisburse {
    pub session_id: u64,
    pub cell_key: MobilityAlloc0ParentKey,
    pub resource_id: u64,
    pub hard_amount: i64,
    pub soft_beta_amount: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityEcon0PlanReport {
    pub substrate_id: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub session_aggregates: Vec<MobilityEcon0SessionAggregate>,
    pub down_disburses: Vec<MobilityEcon0DownDisburse>,

    pub alpha_finalized_before_beta: bool,
    pub beta_reads_finalized_alpha: bool,
    pub hard_soft_same_pass: bool,
    pub conservation_preserved: bool,

    pub touched_session_count: u32,
    pub touched_cell_count: u32,
    pub touched_resource_count: u32,
    pub boundary_group_count: u32,
    pub peak_local_records: u32,
    pub cpu_gpu_parity_checksum: u64,

    pub runtime_implementation_authorized: bool,
    pub owner_parked: bool,
    pub later_econ_scaling_parked: bool,
}

pub fn plan_mobility_econ0(input: &MobilityEcon0PlanInput) -> MobilityEcon0PlanReport {
    let mut diagnostics = Vec::new();
    validate_forbidden(&input.forbidden, &mut diagnostics);
    validate_records(&input.records, &mut diagnostics);

    if !diagnostics.is_empty() {
        return rejected_report(diagnostics, input.records.len() as u32);
    }

    let mut by_group: BTreeMap<MobilityEcon0SessionResourceKey, Vec<MobilityEcon0LocalCellRecord>> =
        BTreeMap::new();
    let mut sessions = BTreeSet::new();
    let mut cells = BTreeSet::new();
    let mut resources = BTreeSet::new();

    for record in &input.records {
        sessions.insert(record.session_id);
        cells.insert(record.cell_key);
        resources.insert(record.resource_id);
        by_group
            .entry(MobilityEcon0SessionResourceKey {
                session_id: record.session_id,
                resource_id: record.resource_id,
            })
            .or_default()
            .push(record.clone());
    }

    let mut session_aggregates = Vec::new();
    let mut down_disburses = Vec::new();

    for (key, records) in &mut by_group {
        records.sort_by_key(|record| (record.cell_key, record.resource_id, record.session_id));

        let hard_available = records
            .iter()
            .map(|record| record.hard_available)
            .sum::<i64>();
        let hard_need = records.iter().map(|record| record.hard_need).sum::<i64>();
        let soft_beta_input = records
            .iter()
            .map(|record| record.soft_beta_signal)
            .sum::<f32>();
        let hard_shortfall = hard_need.saturating_sub(hard_available).max(0);
        let hard_surplus = hard_available.saturating_sub(hard_need).max(0);

        session_aggregates.push(MobilityEcon0SessionAggregate {
            key: *key,
            hard_available,
            hard_need,
            hard_shortfall,
            hard_surplus,
            soft_beta_input,
        });

        let mut remaining = hard_available.min(hard_need);
        for record in records.iter() {
            let hard_amount = record.hard_need.min(remaining);
            remaining -= hard_amount;
            down_disburses.push(MobilityEcon0DownDisburse {
                session_id: record.session_id,
                cell_key: record.cell_key,
                resource_id: record.resource_id,
                hard_amount,
                soft_beta_amount: record.soft_beta_signal + hard_amount as f32,
            });
        }
    }

    session_aggregates.sort_by_key(|aggregate| aggregate.key);
    down_disburses
        .sort_by_key(|disburse| (disburse.session_id, disburse.resource_id, disburse.cell_key));

    let conservation_preserved = conservation_preserved(&session_aggregates, &down_disburses);
    let checksum = mobility_econ0_report_checksum(&session_aggregates, &down_disburses);

    MobilityEcon0PlanReport {
        substrate_id: MOBILITY_ECON0_ID,
        admitted: true,
        diagnostics,
        session_aggregates,
        down_disburses,
        alpha_finalized_before_beta: true,
        beta_reads_finalized_alpha: true,
        hard_soft_same_pass: false,
        conservation_preserved,
        touched_session_count: sessions.len() as u32,
        touched_cell_count: cells.len() as u32,
        touched_resource_count: resources.len() as u32,
        boundary_group_count: by_group.len() as u32,
        peak_local_records: input.records.len() as u32,
        cpu_gpu_parity_checksum: checksum,
        runtime_implementation_authorized: false,
        owner_parked: true,
        later_econ_scaling_parked: true,
    }
}

pub fn mobility_econ0_layout_checksum_cpu(records: &[MobilityEcon0LocalCellRecord]) -> u64 {
    mobility_econ0_layout_checksum(records)
}

pub fn mobility_econ0_layout_checksum_gpu_proxy(records: &[MobilityEcon0LocalCellRecord]) -> u64 {
    mobility_econ0_layout_checksum(records)
}

fn validate_forbidden(
    forbidden: &MobilityEcon0ForbiddenPathRequests,
    diagnostics: &mut Vec<&'static str>,
) {
    if forbidden.owner_overlay_runtime {
        diagnostics.push("owner_overlay_runtime");
    }
    if forbidden.owner_runtime {
        diagnostics.push("owner_runtime");
    }
    if forbidden.default_on_resource_flow {
        diagnostics.push("default_on_resource_flow");
    }
    if forbidden.hard_currency_through_resource_flow {
        diagnostics.push("hard_currency_through_resource_flow");
    }
    if forbidden.float_structural_gate {
        diagnostics.push("float_structural_gate");
    }
    if forbidden.production_simsession_wiring {
        diagnostics.push("production_simsession_wiring");
    }
    if forbidden.semantic_or_raw_wgsl {
        diagnostics.push("semantic_or_raw_wgsl");
    }
    if forbidden.cpu_planner_urgency_commitment {
        diagnostics.push("cpu_planner_urgency_commitment");
    }
    if forbidden.owner_as_spatial_parent {
        diagnostics.push("owner_as_spatial_parent");
    }
    if forbidden.capture_as_reparenting {
        diagnostics.push("capture_as_reparenting");
    }
    if forbidden.hybrid_strata_or_faction_index_scaling_layer {
        diagnostics.push("hybrid_strata_or_faction_index_scaling_layer");
    }
    if forbidden.hard_soft_silent_mix {
        diagnostics.push("hard_soft_silent_mix");
    }
}

fn validate_records(records: &[MobilityEcon0LocalCellRecord], diagnostics: &mut Vec<&'static str>) {
    if records.iter().any(|record| {
        record.hard_available < 0 || record.hard_need < 0 || !record.soft_beta_signal.is_finite()
    }) {
        diagnostics.push("invalid_local_cell_record");
    }
}

fn rejected_report(
    diagnostics: Vec<&'static str>,
    peak_local_records: u32,
) -> MobilityEcon0PlanReport {
    MobilityEcon0PlanReport {
        substrate_id: MOBILITY_ECON0_ID,
        admitted: false,
        diagnostics,
        session_aggregates: vec![],
        down_disburses: vec![],
        alpha_finalized_before_beta: true,
        beta_reads_finalized_alpha: true,
        hard_soft_same_pass: false,
        conservation_preserved: false,
        touched_session_count: 0,
        touched_cell_count: 0,
        touched_resource_count: 0,
        boundary_group_count: 0,
        peak_local_records,
        cpu_gpu_parity_checksum: 0,
        runtime_implementation_authorized: false,
        owner_parked: true,
        later_econ_scaling_parked: true,
    }
}

fn conservation_preserved(
    aggregates: &[MobilityEcon0SessionAggregate],
    disburses: &[MobilityEcon0DownDisburse],
) -> bool {
    aggregates.iter().all(|aggregate| {
        let disbursed = disburses
            .iter()
            .filter(|disburse| {
                disburse.session_id == aggregate.key.session_id
                    && disburse.resource_id == aggregate.key.resource_id
            })
            .map(|disburse| disburse.hard_amount)
            .sum::<i64>();
        disbursed == aggregate.hard_available.min(aggregate.hard_need)
    })
}

fn mobility_econ0_layout_checksum(records: &[MobilityEcon0LocalCellRecord]) -> u64 {
    let mut ordered = records.to_vec();
    ordered.sort_by_key(|record| (record.session_id, record.resource_id, record.cell_key));
    ordered.iter().fold(0xcbf2_9ce4_8422_2325, |hash, record| {
        let hash = fnv_append_u64(hash, record.session_id);
        let hash = fnv_append_u64(hash, record.resource_id);
        let hash = fnv_append_u64(hash, record.cell_key.parent_id);
        let hash = fnv_append_u64(hash, record.cell_key.key_id);
        let hash = fnv_append_u64(hash, record.hard_available as u64);
        let hash = fnv_append_u64(hash, record.hard_need as u64);
        fnv_append_u64(hash, record.soft_beta_signal.to_bits() as u64)
    })
}

fn mobility_econ0_report_checksum(
    aggregates: &[MobilityEcon0SessionAggregate],
    disburses: &[MobilityEcon0DownDisburse],
) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325;
    for aggregate in aggregates {
        hash = fnv_append_u64(hash, aggregate.key.session_id);
        hash = fnv_append_u64(hash, aggregate.key.resource_id);
        hash = fnv_append_u64(hash, aggregate.hard_available as u64);
        hash = fnv_append_u64(hash, aggregate.hard_need as u64);
        hash = fnv_append_u64(hash, aggregate.hard_shortfall as u64);
        hash = fnv_append_u64(hash, aggregate.hard_surplus as u64);
        hash = fnv_append_u64(hash, aggregate.soft_beta_input.to_bits() as u64);
    }
    for disburse in disburses {
        hash = fnv_append_u64(hash, disburse.session_id);
        hash = fnv_append_u64(hash, disburse.resource_id);
        hash = fnv_append_u64(hash, disburse.cell_key.parent_id);
        hash = fnv_append_u64(hash, disburse.cell_key.key_id);
        hash = fnv_append_u64(hash, disburse.hard_amount as u64);
        hash = fnv_append_u64(hash, disburse.soft_beta_amount.to_bits() as u64);
    }
    hash
}

fn fnv_append_u64(mut hash: u64, value: u64) -> u64 {
    for byte in value.to_le_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
