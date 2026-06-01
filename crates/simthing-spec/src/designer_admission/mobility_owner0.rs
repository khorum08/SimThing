//! MOBILITY-OWNER-0: owner-relations + latched modifier overlay substrate.
//!
//! This is a named, metadata/testable substrate only. It models owner
//! relations as explicit columns, applies latched modifier overlays through
//! deterministic owner-column matching, and reports owner-column generation
//! resync/fission outcomes. It does not implement production runtime
//! integration, Resource Flow runtime, OWNER gameplay, Hybrid-Strata/faction
//! index scaling, semantic/raw WGSL, CPU planner logic, or default-on behavior.

use std::collections::{BTreeMap, BTreeSet};

use super::mobility_alloc0::MobilityAlloc0ParentKey;

pub const MOBILITY_OWNER0_ID: &str = "mobility_owner0_owner_relations_latched_overlay";
pub const MOBILITY_OWNER0_REQUIRED_ORDERBAND_DEPTH: u32 = 13;
pub const MOBILITY_OWNER0_CURRENT_MAX_ORDERBAND_DEPTH: u32 = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MobilityOwner0ColumnKind {
    Faction,
    Species,
    Blueprint,
    Tech,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MobilityOwner0ColumnValue {
    pub kind: MobilityOwner0ColumnKind,
    pub owner_id: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityOwner0LocalRecord {
    pub entity_id: u64,
    pub cell_key: MobilityAlloc0ParentKey,
    pub cohort_count: u32,
    pub owner_columns: Vec<MobilityOwner0ColumnValue>,
    pub generation: u64,
    pub blocked_by_blockade: bool,
    pub arrival_order: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityOwner0Overlay {
    pub owner: MobilityOwner0ColumnValue,
    pub modifier_id: u64,
    pub modifier_amount: i64,
    pub arrival_order: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityOwner0OwnerChange {
    pub entity_id: u64,
    pub kind: MobilityOwner0ColumnKind,
    pub from_owner_id: u64,
    pub to_owner_id: u64,
    pub changed_count: u32,
    pub new_entity_id: Option<u64>,
    pub capture: bool,
    pub arrival_order: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MobilityOwner0ForbiddenPathRequests {
    pub owner_as_spatial_parent: bool,
    pub capture_as_reparenting: bool,
    pub nested_arena_reparenting: bool,
    pub default_on_resource_flow: bool,
    pub hard_currency_through_resource_flow: bool,
    pub production_simsession_wiring: bool,
    pub semantic_or_raw_wgsl: bool,
    pub cpu_planner_urgency_commitment: bool,
    pub hybrid_strata_or_faction_index_scaling_layer: bool,
    pub production_runtime_integration: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityOwner0PlanInput {
    pub records: Vec<MobilityOwner0LocalRecord>,
    pub overlays: Vec<MobilityOwner0Overlay>,
    pub owner_changes: Vec<MobilityOwner0OwnerChange>,
    pub forbidden: MobilityOwner0ForbiddenPathRequests,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityOwner0AppliedOverlay {
    pub entity_id: u64,
    pub cell_key: MobilityAlloc0ParentKey,
    pub owner: MobilityOwner0ColumnValue,
    pub modifier_id: u64,
    pub modifier_amount: i64,
    pub blocked_by_blockade: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityOwner0GenerationResync {
    pub entity_id: u64,
    pub old_generation: u64,
    pub new_generation: u64,
    pub changed_owner: MobilityOwner0ColumnValue,
    pub no_silent_rebind: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityOwner0FissionResult {
    pub source_entity_id: u64,
    pub new_entity_id: u64,
    pub retained_count: u32,
    pub fission_count: u32,
    pub retained_owner_columns: Vec<MobilityOwner0ColumnValue>,
    pub fission_owner_columns: Vec<MobilityOwner0ColumnValue>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityOwner0PlanReport {
    pub substrate_id: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub applied_overlays: Vec<MobilityOwner0AppliedOverlay>,
    pub generation_resyncs: Vec<MobilityOwner0GenerationResync>,
    pub fission_results: Vec<MobilityOwner0FissionResult>,

    pub touched_cell_count: u32,
    pub touched_entity_count: u32,
    pub touched_owner_count: u32,
    pub overlay_count: u32,
    pub owner_change_count: u32,
    pub peak_local_records: u32,
    pub modifier_dispersal_count: u32,
    pub dirtyonly_noop_count: u32,
    pub spawned_arena_column_count: u32,
    pub spawned_aggregation_column_count: u32,
    pub owner_columns_are_spatial_parents: bool,
    pub capture_reparented: bool,
    pub blockade_dropped_latched_modifier: bool,
    pub owner_band_budget_preserved: bool,
    pub required_orderband_depth: u32,
    pub max_orderband_depth: u32,
    pub cpu_gpu_parity_checksum: u64,

    pub runtime_implementation_authorized: bool,
    pub production_runtime_integration_parked: bool,
    pub later_econ_scaling_parked: bool,
}

pub fn plan_mobility_owner0(input: &MobilityOwner0PlanInput) -> MobilityOwner0PlanReport {
    let mut diagnostics = Vec::new();
    validate_forbidden(&input.forbidden, &mut diagnostics);
    validate_records(input, &mut diagnostics);

    if !diagnostics.is_empty() {
        return rejected_report(diagnostics, input.records.len() as u32);
    }

    let mut records = input.records.clone();
    records.sort_by_key(|record| (record.cell_key, record.entity_id));

    let overlays_by_owner = canonical_overlays(&input.overlays);
    let mut applied_overlays = apply_overlays(&records, &overlays_by_owner);
    let (mut generation_resyncs, mut fission_results) = apply_owner_changes(&records, input);

    applied_overlays.sort_by_key(|applied| {
        (
            applied.entity_id,
            applied.owner,
            applied.modifier_id,
            applied.cell_key,
        )
    });
    generation_resyncs.sort_by_key(|resync| {
        (
            resync.entity_id,
            resync.changed_owner,
            resync.new_generation,
        )
    });
    fission_results.sort_by_key(|fission| (fission.source_entity_id, fission.new_entity_id));

    let touched_cells = records
        .iter()
        .map(|record| record.cell_key)
        .collect::<BTreeSet<_>>();
    let touched_owners = records
        .iter()
        .flat_map(|record| record.owner_columns.iter().copied())
        .chain(input.overlays.iter().map(|overlay| overlay.owner))
        .collect::<BTreeSet<_>>();

    let modifier_dispersal_count = if input.owner_changes.is_empty() {
        0
    } else {
        applied_overlays.len() as u32
    };
    let dirtyonly_noop_count = if input.owner_changes.is_empty() {
        applied_overlays.len() as u32
    } else {
        0
    };
    let checksum = mobility_owner0_report_checksum(
        &records,
        &input.overlays,
        &applied_overlays,
        &generation_resyncs,
        &fission_results,
    );

    MobilityOwner0PlanReport {
        substrate_id: MOBILITY_OWNER0_ID,
        admitted: true,
        diagnostics,
        applied_overlays,
        generation_resyncs,
        fission_results,
        touched_cell_count: touched_cells.len() as u32,
        touched_entity_count: records.len() as u32,
        touched_owner_count: touched_owners.len() as u32,
        overlay_count: input.overlays.len() as u32,
        owner_change_count: input.owner_changes.len() as u32,
        peak_local_records: input.records.len() as u32 + input.owner_changes.len() as u32,
        modifier_dispersal_count,
        dirtyonly_noop_count,
        spawned_arena_column_count: 0,
        spawned_aggregation_column_count: 0,
        owner_columns_are_spatial_parents: false,
        capture_reparented: false,
        blockade_dropped_latched_modifier: false,
        owner_band_budget_preserved: true,
        required_orderband_depth: MOBILITY_OWNER0_REQUIRED_ORDERBAND_DEPTH,
        max_orderband_depth: MOBILITY_OWNER0_CURRENT_MAX_ORDERBAND_DEPTH,
        cpu_gpu_parity_checksum: checksum,
        runtime_implementation_authorized: false,
        production_runtime_integration_parked: true,
        later_econ_scaling_parked: true,
    }
}

pub fn mobility_owner0_layout_checksum_cpu(records: &[MobilityOwner0LocalRecord]) -> u64 {
    mobility_owner0_layout_checksum(records)
}

pub fn mobility_owner0_layout_checksum_gpu_proxy(records: &[MobilityOwner0LocalRecord]) -> u64 {
    mobility_owner0_layout_checksum(records)
}

fn validate_forbidden(
    forbidden: &MobilityOwner0ForbiddenPathRequests,
    diagnostics: &mut Vec<&'static str>,
) {
    if forbidden.owner_as_spatial_parent {
        diagnostics.push("owner_as_spatial_parent");
    }
    if forbidden.capture_as_reparenting {
        diagnostics.push("capture_as_reparenting");
    }
    if forbidden.nested_arena_reparenting {
        diagnostics.push("nested_arena_reparenting");
    }
    if forbidden.default_on_resource_flow {
        diagnostics.push("default_on_resource_flow");
    }
    if forbidden.hard_currency_through_resource_flow {
        diagnostics.push("hard_currency_through_resource_flow");
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
    if forbidden.hybrid_strata_or_faction_index_scaling_layer {
        diagnostics.push("hybrid_strata_or_faction_index_scaling_layer");
    }
    if forbidden.production_runtime_integration {
        diagnostics.push("production_runtime_integration");
    }
}

fn validate_records(input: &MobilityOwner0PlanInput, diagnostics: &mut Vec<&'static str>) {
    let records_by_entity = input
        .records
        .iter()
        .map(|record| (record.entity_id, record))
        .collect::<BTreeMap<_, _>>();
    let mut entity_ids = BTreeSet::new();

    for record in &input.records {
        if !entity_ids.insert(record.entity_id) || record.cohort_count == 0 {
            diagnostics.push("invalid_owner_local_record");
        }
        let mut kinds = BTreeSet::new();
        for column in &record.owner_columns {
            if !kinds.insert(column.kind) {
                diagnostics.push("duplicate_owner_column");
            }
        }
    }

    for change in &input.owner_changes {
        let Some(record) = records_by_entity.get(&change.entity_id) else {
            diagnostics.push("unknown_owner_change_entity");
            continue;
        };
        if change.changed_count == 0 || change.changed_count > record.cohort_count {
            diagnostics.push("invalid_owner_change_count");
        }
        if change.changed_count < record.cohort_count && change.new_entity_id.is_none() {
            diagnostics.push("partial_owner_change_requires_fission_entity");
        }
        if change
            .new_entity_id
            .is_some_and(|new_entity_id| entity_ids.contains(&new_entity_id))
        {
            diagnostics.push("fission_entity_id_already_live");
        }
        if !record.owner_columns.contains(&MobilityOwner0ColumnValue {
            kind: change.kind,
            owner_id: change.from_owner_id,
        }) {
            diagnostics.push("owner_change_from_column_mismatch");
        }
    }
}

fn canonical_overlays(
    overlays: &[MobilityOwner0Overlay],
) -> BTreeMap<MobilityOwner0ColumnValue, Vec<MobilityOwner0Overlay>> {
    let mut by_owner: BTreeMap<MobilityOwner0ColumnValue, Vec<MobilityOwner0Overlay>> =
        BTreeMap::new();
    for overlay in overlays {
        by_owner
            .entry(overlay.owner)
            .or_default()
            .push(overlay.clone());
    }
    for owner_overlays in by_owner.values_mut() {
        owner_overlays.sort_by_key(|overlay| (overlay.owner, overlay.modifier_id));
    }
    by_owner
}

fn apply_overlays(
    records: &[MobilityOwner0LocalRecord],
    overlays_by_owner: &BTreeMap<MobilityOwner0ColumnValue, Vec<MobilityOwner0Overlay>>,
) -> Vec<MobilityOwner0AppliedOverlay> {
    let mut applied = Vec::new();
    for record in records {
        let mut owner_columns = record.owner_columns.clone();
        owner_columns.sort();
        for owner in owner_columns {
            if let Some(overlays) = overlays_by_owner.get(&owner) {
                for overlay in overlays {
                    applied.push(MobilityOwner0AppliedOverlay {
                        entity_id: record.entity_id,
                        cell_key: record.cell_key,
                        owner,
                        modifier_id: overlay.modifier_id,
                        modifier_amount: overlay.modifier_amount,
                        blocked_by_blockade: record.blocked_by_blockade,
                    });
                }
            }
        }
    }
    applied
}

fn apply_owner_changes(
    records: &[MobilityOwner0LocalRecord],
    input: &MobilityOwner0PlanInput,
) -> (
    Vec<MobilityOwner0GenerationResync>,
    Vec<MobilityOwner0FissionResult>,
) {
    let records_by_entity = records
        .iter()
        .map(|record| (record.entity_id, record))
        .collect::<BTreeMap<_, _>>();
    let mut changes = input.owner_changes.clone();
    changes.sort_by_key(|change| (change.entity_id, change.kind, change.to_owner_id));

    let mut resyncs = Vec::new();
    let mut fissions = Vec::new();
    for change in changes {
        let Some(record) = records_by_entity.get(&change.entity_id) else {
            continue;
        };
        let changed_owner = MobilityOwner0ColumnValue {
            kind: change.kind,
            owner_id: change.to_owner_id,
        };
        resyncs.push(MobilityOwner0GenerationResync {
            entity_id: record.entity_id,
            old_generation: record.generation,
            new_generation: record.generation + 1,
            changed_owner,
            no_silent_rebind: true,
        });

        if change.changed_count < record.cohort_count {
            let retained_count = record.cohort_count - change.changed_count;
            let retained_owner_columns = record.owner_columns.clone();
            let mut fission_owner_columns = record.owner_columns.clone();
            replace_owner_column(&mut fission_owner_columns, changed_owner);
            fission_owner_columns.sort();
            fissions.push(MobilityOwner0FissionResult {
                source_entity_id: record.entity_id,
                new_entity_id: change.new_entity_id.unwrap_or(0),
                retained_count,
                fission_count: change.changed_count,
                retained_owner_columns,
                fission_owner_columns,
            });
        }
    }

    (resyncs, fissions)
}

fn replace_owner_column(
    owner_columns: &mut Vec<MobilityOwner0ColumnValue>,
    new_value: MobilityOwner0ColumnValue,
) {
    if let Some(column) = owner_columns
        .iter_mut()
        .find(|column| column.kind == new_value.kind)
    {
        *column = new_value;
    }
}

fn rejected_report(
    diagnostics: Vec<&'static str>,
    peak_local_records: u32,
) -> MobilityOwner0PlanReport {
    MobilityOwner0PlanReport {
        substrate_id: MOBILITY_OWNER0_ID,
        admitted: false,
        diagnostics,
        applied_overlays: vec![],
        generation_resyncs: vec![],
        fission_results: vec![],
        touched_cell_count: 0,
        touched_entity_count: 0,
        touched_owner_count: 0,
        overlay_count: 0,
        owner_change_count: 0,
        peak_local_records,
        modifier_dispersal_count: 0,
        dirtyonly_noop_count: 0,
        spawned_arena_column_count: 0,
        spawned_aggregation_column_count: 0,
        owner_columns_are_spatial_parents: false,
        capture_reparented: false,
        blockade_dropped_latched_modifier: false,
        owner_band_budget_preserved: false,
        required_orderband_depth: MOBILITY_OWNER0_REQUIRED_ORDERBAND_DEPTH,
        max_orderband_depth: MOBILITY_OWNER0_CURRENT_MAX_ORDERBAND_DEPTH,
        cpu_gpu_parity_checksum: 0,
        runtime_implementation_authorized: false,
        production_runtime_integration_parked: true,
        later_econ_scaling_parked: true,
    }
}

fn mobility_owner0_layout_checksum(records: &[MobilityOwner0LocalRecord]) -> u64 {
    let mut ordered = records.to_vec();
    ordered.sort_by_key(|record| (record.cell_key, record.entity_id));
    ordered.iter().fold(0xcbf2_9ce4_8422_2325, |hash, record| {
        let hash = fnv_append_u64(hash, record.entity_id);
        let hash = fnv_append_u64(hash, record.cell_key.parent_id);
        let hash = fnv_append_u64(hash, record.cell_key.key_id);
        let hash = fnv_append_u64(hash, record.cohort_count as u64);
        let hash = fnv_append_u64(hash, record.generation);
        let mut columns = record.owner_columns.clone();
        columns.sort();
        columns.iter().fold(hash, |hash, column| {
            let hash = fnv_append_u64(hash, owner_kind_code(column.kind));
            fnv_append_u64(hash, column.owner_id)
        })
    })
}

fn mobility_owner0_report_checksum(
    records: &[MobilityOwner0LocalRecord],
    overlays: &[MobilityOwner0Overlay],
    applied: &[MobilityOwner0AppliedOverlay],
    resyncs: &[MobilityOwner0GenerationResync],
    fissions: &[MobilityOwner0FissionResult],
) -> u64 {
    let mut hash = mobility_owner0_layout_checksum(records);
    let mut overlays = overlays.to_vec();
    overlays.sort_by_key(|overlay| (overlay.owner, overlay.modifier_id));
    for overlay in overlays {
        hash = fnv_append_u64(hash, owner_kind_code(overlay.owner.kind));
        hash = fnv_append_u64(hash, overlay.owner.owner_id);
        hash = fnv_append_u64(hash, overlay.modifier_id);
        hash = fnv_append_u64(hash, overlay.modifier_amount as u64);
    }
    for overlay in applied {
        hash = fnv_append_u64(hash, overlay.entity_id);
        hash = fnv_append_u64(hash, owner_kind_code(overlay.owner.kind));
        hash = fnv_append_u64(hash, overlay.owner.owner_id);
        hash = fnv_append_u64(hash, overlay.modifier_id);
        hash = fnv_append_u64(hash, overlay.modifier_amount as u64);
    }
    for resync in resyncs {
        hash = fnv_append_u64(hash, resync.entity_id);
        hash = fnv_append_u64(hash, resync.old_generation);
        hash = fnv_append_u64(hash, resync.new_generation);
        hash = fnv_append_u64(hash, owner_kind_code(resync.changed_owner.kind));
        hash = fnv_append_u64(hash, resync.changed_owner.owner_id);
    }
    for fission in fissions {
        hash = fnv_append_u64(hash, fission.source_entity_id);
        hash = fnv_append_u64(hash, fission.new_entity_id);
        hash = fnv_append_u64(hash, fission.retained_count as u64);
        hash = fnv_append_u64(hash, fission.fission_count as u64);
    }
    hash
}

fn owner_kind_code(kind: MobilityOwner0ColumnKind) -> u64 {
    match kind {
        MobilityOwner0ColumnKind::Faction => 1,
        MobilityOwner0ColumnKind::Species => 2,
        MobilityOwner0ColumnKind::Blueprint => 3,
        MobilityOwner0ColumnKind::Tech => 4,
    }
}

fn fnv_append_u64(mut hash: u64, value: u64) -> u64 {
    for byte in value.to_le_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
