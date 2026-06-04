//! SCENARIO-0080-2-R5: movement via BoundaryRequest + REENROLL + mobility substrate.
//!
//! Consumes R4 sit-still vs step-opportunity rows; materializes bounded BoundaryRequests only
//! from threshold+event posture; routes through MOBILITY-RUNTIME-0 (ALLOC/REENROLL/IDROUTE/OWNER).
//! Optional gated starport→Fleet fission via ALLOC arrival + owner overlay. Opt-in/default-off.

#[allow(dead_code, unused_imports)]
#[path = "dress_rehearsal_atlas_batch_0_store.rs"]
mod atlas_store;

use crate::dress_rehearsal_r1_disruption_heatmap::{
    run_dress_rehearsal_r1_disruption_heatmap, DressRehearsalR1Input, DressRehearsalR1OccupantKind,
    DressRehearsalR1Owner, DressRehearsalR1Report,
};
use crate::dress_rehearsal_r2_recursive_allocation::{
    run_dress_rehearsal_r2_recursive_allocation, DressRehearsalR2Input, DressRehearsalR2Owner,
    DressRehearsalR2Report, DressRehearsalR2SystemProductionRow,
};
use crate::dress_rehearsal_r3_capability_mask_down::{
    run_dress_rehearsal_r3_capability_mask_down, DressRehearsalR3Input, DressRehearsalR3Report,
};
use crate::dress_rehearsal_r4_sead_field_consumption::{
    run_dress_rehearsal_r4_sead_field_consumption, DressRehearsalR4Decision,
    DressRehearsalR4Input, DressRehearsalR4Owner, DressRehearsalR4Report,
};
use simthing_spec::{
    compose_mobility_runtime0, plan_mobility_alloc0, IdentityLane, MobilityAlloc0BlockSpec,
    MobilityAlloc0BoundaryEvent, MobilityAlloc0BoundaryEventKind, MobilityAlloc0ForbiddenPathRequests,
    MobilityAlloc0LiveSlice, MobilityAlloc0ParentKey, MobilityAlloc0PlanInput,
    MobilityEcon0ForbiddenPathRequests, MobilityEcon0PlanInput, MobilityIdroute0ForbiddenPathRequests,
    MobilityIdroute0LocalRecord, MobilityIdroute0PlanInput, MobilityOwner0ColumnKind,
    MobilityOwner0ColumnValue, MobilityOwner0ForbiddenPathRequests, MobilityOwner0LocalRecord,
    MobilityOwner0Overlay, MobilityOwner0PlanInput, MobilityReenroll0ForbiddenPathRequests,
    MobilityReenroll0Move, MobilityReenroll0PlanInput, MobilityReenroll0RegistryState,
    MobilityRuntime0CompositionInput, MobilityRuntime0ForbiddenPathRequests,
    MobilityRuntime0HarnessConfig,
};
use std::collections::{BTreeMap, BTreeSet};

pub const DRESS_REHEARSAL_R5_MOVEMENT_REENROLL_ID: &str =
    "SCENARIO-0080-2-R5-MOVEMENT-REENROLL";
pub const DRESS_REHEARSAL_R5_MOVEMENT_REENROLL_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - movement via BoundaryRequest + REENROLL + mobility substrate";
pub const DRESS_REHEARSAL_R5_SCENARIO: &str = "SCENARIO-0080-2";
pub const GALACTIC_STRUCTURAL_PARENT: &str = "galactic-location-0";
pub const GALACTIC_PARENT_ID: u64 = 0;
pub const SLOTS_PER_CELL: u32 = 8;
pub const BOUNDARY_REQUEST_ID_BASE: u64 = 0x8050_0000_0000_0000;
pub const FISSION_FLEET_ENTITY_BASE: u64 = 0x9010_0000_0000_0000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DressRehearsalR5Owner {
    Terran,
    Pirate,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR5BoundaryRequestRow {
    pub boundary_request_id: u64,
    pub mover_id: String,
    pub threshold_input_mag_bits: u32,
    pub event_emitted: bool,
    pub source_cell_index: u32,
    pub destination_cell_index: u32,
    pub materialized_from_r4_step_opportunity: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR5MovementRow {
    pub mover_id: String,
    pub owner: DressRehearsalR5Owner,
    pub source_cell_index: u32,
    pub destination_cell_index: u32,
    pub r4_decision_consumed: &'static str,
    pub event_emitted: bool,
    pub boundary_request_id: u64,
    pub entity_id: u64,
    pub idroute_identity_before: u32,
    pub idroute_identity_after: u32,
    pub owner_faction_id_before: u64,
    pub owner_faction_id_after: u64,
    pub source_arena_membership_before: Vec<u64>,
    pub source_arena_membership_after: Vec<u64>,
    pub destination_arena_membership_before: Vec<u64>,
    pub destination_arena_membership_after: Vec<u64>,
    pub structural_parent_before: &'static str,
    pub structural_parent_after: &'static str,
    pub movement_applied: bool,
    pub post_move_cell_index: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR5SitStillRow {
    pub mover_id: String,
    pub reason: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR5FissionRow {
    pub starport_id: String,
    pub production_trigger: &'static str,
    pub new_fleet_id: String,
    pub new_fleet_entity_id: u64,
    pub owner_faction_id: u64,
    pub enrolled_cell_index: u32,
    pub idroute_identity_lane: u32,
    pub fission_applied: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR5Summary {
    pub movement_row_count: usize,
    pub sit_still_row_count: usize,
    pub boundary_request_count: usize,
    pub fission_row_count: usize,
    pub mobility_substrate_admitted: bool,
    pub stable_checksum: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR5Artifact {
    pub movement_rows: Vec<DressRehearsalR5MovementRow>,
    pub sit_still_rows: Vec<DressRehearsalR5SitStillRow>,
    pub boundary_request_rows: Vec<DressRehearsalR5BoundaryRequestRow>,
    pub fission_rows: Vec<DressRehearsalR5FissionRow>,
    pub summary: DressRehearsalR5Summary,
    pub cpu_oracle_parity: bool,
    pub markdown: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR5Oracle {
    pub movement_rows: Vec<DressRehearsalR5MovementRow>,
    pub sit_still_rows: Vec<DressRehearsalR5SitStillRow>,
    pub boundary_request_rows: Vec<DressRehearsalR5BoundaryRequestRow>,
    pub fission_rows: Vec<DressRehearsalR5FissionRow>,
    pub summary: DressRehearsalR5Summary,
    pub mobility_substrate_diagnostics: Vec<&'static str>,
    pub mobility_composed_cpu_checksum: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR5Input {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
    pub r1_report: Option<DressRehearsalR1Report>,
    pub r2_report: Option<DressRehearsalR2Report>,
    pub r3_report: Option<DressRehearsalR3Report>,
    pub r4_report: Option<DressRehearsalR4Report>,
}

impl DressRehearsalR5Input {
    pub fn default_simsession() -> Self {
        Self {
            explicit_opt_in: false,
            enabled_by_default: false,
            r1_report: None,
            r2_report: None,
            r3_report: None,
            r4_report: None,
        }
    }

    pub fn explicit_opt_in() -> Self {
        let r1_report =
            run_dress_rehearsal_r1_disruption_heatmap(&DressRehearsalR1Input::explicit_opt_in());
        let r2_report = run_dress_rehearsal_r2_recursive_allocation(
            &DressRehearsalR2Input::with_r1_report(r1_report.clone()),
        );
        let r3_report = run_dress_rehearsal_r3_capability_mask_down(
            &DressRehearsalR3Input::with_reports(r1_report.clone(), r2_report.clone()),
        );
        let r4_report = run_dress_rehearsal_r4_sead_field_consumption(
            &DressRehearsalR4Input {
                explicit_opt_in: true,
                enabled_by_default: false,
                movement_threshold_mag_bits:
                    crate::dress_rehearsal_r4_sead_field_consumption::MOVEMENT_THRESHOLD_MAG_BITS,
                r1_report: Some(r1_report.clone()),
                r2_report: Some(r2_report.clone()),
                r3_report: Some(r3_report.clone()),
            },
        );
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
            r1_report: Some(r1_report),
            r2_report: Some(r2_report),
            r3_report: Some(r3_report),
            r4_report: Some(r4_report),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR5Report {
    pub id: &'static str,
    pub status: &'static str,
    pub scenario_name: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,
    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,

    pub r1_contract_checksum: u64,
    pub r2_contract_checksum: u64,
    pub r3_contract_checksum: u64,
    pub r4_contract_checksum: u64,
    pub r1_cpu_oracle_parity: bool,
    pub r2_cpu_oracle_parity: bool,
    pub r3_cpu_oracle_parity: bool,
    pub r4_cpu_oracle_parity: bool,

    pub movement_rows: Vec<DressRehearsalR5MovementRow>,
    pub sit_still_rows: Vec<DressRehearsalR5SitStillRow>,
    pub boundary_request_rows: Vec<DressRehearsalR5BoundaryRequestRow>,
    pub fission_rows: Vec<DressRehearsalR5FissionRow>,
    pub artifact: DressRehearsalR5Artifact,
    pub summary: DressRehearsalR5Summary,

    pub mobility_substrate_admitted: bool,
    pub mobility_substrate_diagnostics: Vec<&'static str>,
    pub mobility_composed_cpu_checksum: u64,
    pub fission_substrate_available: bool,
    pub fission_blocked_reason: Option<&'static str>,

    pub direct_movement_command: bool,
    pub external_boundary_request: bool,
    pub cpu_planner_used: bool,
    pub default_simsession_pass_graph_change: bool,
    pub new_shader_or_wgsl: bool,

    pub cpu_oracle_parity: bool,
    pub deterministic_replay_checksum: u64,
}

pub fn run_dress_rehearsal_r5_movement_reenroll(
    input: &DressRehearsalR5Input,
) -> DressRehearsalR5Report {
    let mut diagnostics = Vec::new();
    validate_input(input, &mut diagnostics);

    if !input.explicit_opt_in {
        return base_report(input, true, diagnostics, None, false);
    }
    if !diagnostics.is_empty() {
        return base_report(input, false, diagnostics, None, false);
    }

    let execution = execute_model(input);
    let oracle = cpu_oracle_dress_rehearsal_r5_movement_reenroll(input);
    let parity = execution.movement_rows == oracle.movement_rows
        && execution.sit_still_rows == oracle.sit_still_rows
        && execution.boundary_request_rows == oracle.boundary_request_rows
        && execution.fission_rows == oracle.fission_rows
        && execution.summary == oracle.summary;
    base_report(input, false, Vec::new(), Some(execution), parity)
}

pub fn replay_dress_rehearsal_r5_movement_reenroll(
) -> (DressRehearsalR5Report, DressRehearsalR5Report) {
    let input = DressRehearsalR5Input::explicit_opt_in();
    (
        run_dress_rehearsal_r5_movement_reenroll(&input),
        run_dress_rehearsal_r5_movement_reenroll(&input),
    )
}

pub fn cpu_oracle_dress_rehearsal_r5_movement_reenroll(
    input: &DressRehearsalR5Input,
) -> DressRehearsalR5Oracle {
    if !input.explicit_opt_in || input.enabled_by_default {
        return empty_oracle();
    }
    let mut diagnostics = Vec::new();
    validate_input(input, &mut diagnostics);
    if !diagnostics.is_empty() {
        return empty_oracle();
    }
    execute_model(input)
}

pub fn render_dress_rehearsal_r5_artifact(report: &DressRehearsalR5Report) -> String {
    report.artifact.markdown.clone()
}

fn execute_model(input: &DressRehearsalR5Input) -> DressRehearsalR5Oracle {
    let r1 = input.r1_report.as_ref().expect("validated R1");
    let r2 = input.r2_report.as_ref().expect("validated R2");
    let r4 = input.r4_report.as_ref().expect("validated R4");

    let mut boundary_request_rows = Vec::new();
    let mut sit_still_rows = Vec::new();
    let mut reenroll_moves = Vec::new();
    let mut move_meta: Vec<(String, u64, u32, u32, u32, u64, u64)> = Vec::new();

    for row in &r4.mover_rows {
        match row.decision {
            DressRehearsalR4Decision::SitStill => {
                sit_still_rows.push(DressRehearsalR5SitStillRow {
                    mover_id: row.mover_id.clone(),
                    reason: if row.threshold_passed {
                        "r4_sit_still_despite_threshold"
                    } else {
                        "r4_threshold_not_passed"
                    },
                });
            }
            DressRehearsalR4Decision::StepOpportunity => {
                let Some(dest) = row.candidate_target_cell_index else {
                    sit_still_rows.push(DressRehearsalR5SitStillRow {
                        mover_id: row.mover_id.clone(),
                        reason: "step_opportunity_missing_target",
                    });
                    continue;
                };
                if dest == row.cell_index {
                    sit_still_rows.push(DressRehearsalR5SitStillRow {
                        mover_id: row.mover_id.clone(),
                        reason: "step_opportunity_target_equals_source",
                    });
                    continue;
                }
                if !row.threshold_passed {
                    sit_still_rows.push(DressRehearsalR5SitStillRow {
                        mover_id: row.mover_id.clone(),
                        reason: "step_opportunity_without_threshold_event",
                    });
                    continue;
                }
                let entity_id = entity_id_for_mover(&row.mover_id);
                let boundary_request_id =
                    boundary_request_id_for(&row.mover_id, row.cell_index, dest);
                boundary_request_rows.push(DressRehearsalR5BoundaryRequestRow {
                    boundary_request_id,
                    mover_id: row.mover_id.clone(),
                    threshold_input_mag_bits: row.candidate_f_exact_mag_bits,
                    event_emitted: true,
                    source_cell_index: row.cell_index,
                    destination_cell_index: dest,
                    materialized_from_r4_step_opportunity: true,
                });
                reenroll_moves.push(MobilityReenroll0Move {
                    entity_id,
                    origin: cell_key(row.cell_index),
                    destination: cell_key(dest),
                    arrival_order: boundary_request_id,
                });
                move_meta.push((
                    row.mover_id.clone(),
                    entity_id,
                    row.cell_index,
                    dest,
                    identity_lane_for_owner(row.owner),
                    owner_from_r4(row.owner),
                    boundary_request_id,
                ));
            }
        }
    }

    let (fission_rows, fission_live_updates) =
        build_fission_rows_and_alloc(r1, r2);

    let mut active_entities: BTreeSet<u64> = reenroll_moves.iter().map(|mv| mv.entity_id).collect();
    for (entity_id, _) in &fission_live_updates {
        active_entities.insert(*entity_id);
    }
    let cell_indices = collect_cell_indices(&reenroll_moves, &fission_rows);
    let blocks = build_blocks(&cell_indices);
    let block_by_cell = block_start_by_cell(&blocks);
    let mut live_slices = build_initial_live_slices(r1, &block_by_cell, &active_entities);
    for (entity_id, cell_index) in fission_live_updates {
        if !live_slices.iter().any(|s| s.entity_id == entity_id) {
            let slot = next_slot_in_cell(&live_slices, &block_by_cell, cell_index);
            live_slices.push(MobilityAlloc0LiveSlice {
                entity_id,
                parent_key: cell_key(cell_index),
                slot,
            });
        }
    }
    let composition_input = build_composition_input(
        r1,
        blocks.clone(),
        live_slices.clone(),
        reenroll_moves.clone(),
    );
    let composition = compose_mobility_runtime0(&composition_input);
    reenroll_moves.sort_by_key(|mv| (mv.entity_id, mv.origin, mv.destination));
    let mut movement_rows = Vec::new();
    for (
        mover_id,
        entity_id,
        source_cell,
        dest_cell,
        identity_lane,
        owner_id,
        boundary_id,
    ) in move_meta
    {
        let committed = composition
            .reenroll_report
            .as_ref()
            .and_then(|report| {
                report
                    .committed_moves
                    .iter()
                    .find(|mv| mv.entity_id == entity_id)
                    .cloned()
            });
        let final_slices = composition
            .reenroll_report
            .as_ref()
            .map(|report| report.final_live_slices.clone())
            .unwrap_or_default();
        let movement_applied = committed.is_some();
        let post_move_cell = if movement_applied {
            dest_cell
        } else {
            source_cell
        };
        movement_rows.push(DressRehearsalR5MovementRow {
            mover_id,
            owner: owner_from_faction(owner_id),
            source_cell_index: source_cell,
            destination_cell_index: dest_cell,
            r4_decision_consumed: "StepOpportunity",
            event_emitted: true,
            boundary_request_id: boundary_id,
            entity_id,
            idroute_identity_before: identity_lane,
            idroute_identity_after: identity_lane,
            owner_faction_id_before: owner_id,
            owner_faction_id_after: owner_id,
            source_arena_membership_before: entities_in_cell(&live_slices, source_cell),
            source_arena_membership_after: entities_in_cell(&final_slices, source_cell),
            destination_arena_membership_before: entities_in_cell(&live_slices, dest_cell),
            destination_arena_membership_after: entities_in_cell(&final_slices, dest_cell),
            structural_parent_before: GALACTIC_STRUCTURAL_PARENT,
            structural_parent_after: GALACTIC_STRUCTURAL_PARENT,
            movement_applied,
            post_move_cell_index: post_move_cell,
        });
    }

    let mut fission_rows_out = fission_rows;
    for row in &mut fission_rows_out {
        let key = cell_key(row.enrolled_cell_index);
        row.fission_applied = composition
            .alloc_report
            .as_ref()
            .map(|alloc| {
                alloc.final_live_slices.iter().any(|slice| {
                    slice.entity_id == row.new_fleet_entity_id && slice.parent_key == key
                })
            })
            .unwrap_or(false);
        if !row.fission_applied {
            row.fission_applied = live_slices.iter().any(|slice| {
                slice.entity_id == row.new_fleet_entity_id
                    && slice.parent_key == cell_key(row.enrolled_cell_index)
            });
        }
    }

    let summary = DressRehearsalR5Summary {
        movement_row_count: movement_rows.len(),
        sit_still_row_count: sit_still_rows.len(),
        boundary_request_count: boundary_request_rows.len(),
        fission_row_count: fission_rows_out.len(),
        mobility_substrate_admitted: composition.admitted,
        stable_checksum: checksum_r5(
            r1.starmap_summary.stable_checksum,
            r2.summary.stable_checksum,
            input
                .r3_report
                .as_ref()
                .map(|r| r.summary.stable_checksum)
                .unwrap_or(0),
            r4.summary.stable_checksum,
            &movement_rows,
            &sit_still_rows,
            &boundary_request_rows,
            &fission_rows_out,
        ),
    };

    movement_rows.sort_by(|a, b| a.mover_id.cmp(&b.mover_id));
    sit_still_rows.sort_by(|a, b| a.mover_id.cmp(&b.mover_id));
    boundary_request_rows.sort_by_key(|row| row.boundary_request_id);

    DressRehearsalR5Oracle {
        movement_rows,
        sit_still_rows,
        boundary_request_rows,
        fission_rows: fission_rows_out,
        summary,
        mobility_substrate_diagnostics: composition.diagnostics,
        mobility_composed_cpu_checksum: composition.composed_cpu_checksum,
    }
}

fn build_fission_rows_and_alloc(
    r1: &DressRehearsalR1Report,
    r2: &DressRehearsalR2Report,
) -> (Vec<DressRehearsalR5FissionRow>, Vec<(u64, u32)>) {
    let _ = r1;
    let mut rows = Vec::new();
    let mut alloc_targets = Vec::new();
    let mut candidates: Vec<&DressRehearsalR2SystemProductionRow> = r2
        .production_rows
        .iter()
        .filter(|row| row.has_starport && row.production_generated > 0)
        .collect();
    candidates.sort_by_key(|row| row.system_index);
    if let Some(starport_row) = candidates.first() {
        let new_fleet_id = format!(
            "dress-rehearsal-r5-fission-fleet-{}",
            starport_row.system_id
        );
        let entity_id = entity_id_for_mover(&new_fleet_id);
        let owner_id = match starport_row.original_owner {
            DressRehearsalR2Owner::Terran => 1,
            DressRehearsalR2Owner::Pirate => 2,
        };
        rows.push(DressRehearsalR5FissionRow {
            starport_id: starport_row.system_id.clone(),
            production_trigger: "starport_production_generated",
            new_fleet_id: new_fleet_id.clone(),
            new_fleet_entity_id: entity_id,
            owner_faction_id: owner_id,
            enrolled_cell_index: starport_row.cell_index,
            idroute_identity_lane: identity_lane_for_owner(match starport_row.original_owner {
                DressRehearsalR2Owner::Terran => DressRehearsalR4Owner::Terran,
                DressRehearsalR2Owner::Pirate => DressRehearsalR4Owner::Pirate,
            }),
            fission_applied: false,
        });
        alloc_targets.push((entity_id, starport_row.cell_index));

        let blocks = build_blocks(&BTreeSet::from([starport_row.cell_index]));
        let events = vec![MobilityAlloc0BoundaryEvent {
            kind: MobilityAlloc0BoundaryEventKind::Arrival,
            parent_key: cell_key(starport_row.cell_index),
            entity_id: Some(entity_id),
            arrival_order: entity_id,
        }];
        let alloc_report = plan_mobility_alloc0(&MobilityAlloc0PlanInput {
            blocks,
            live_slices: vec![],
            events,
            forbidden: MobilityAlloc0ForbiddenPathRequests::default(),
        });
        if alloc_report.admitted {
            for row in &mut rows {
                row.fission_applied = alloc_report.final_live_slices.iter().any(|slice| {
                    slice.entity_id == entity_id
                        && slice.parent_key == cell_key(starport_row.cell_index)
                });
            }
        }
    }
    (rows, alloc_targets)
}

fn build_composition_input(
    r1: &DressRehearsalR1Report,
    blocks: Vec<MobilityAlloc0BlockSpec>,
    live_slices: Vec<MobilityAlloc0LiveSlice>,
    moves: Vec<MobilityReenroll0Move>,
) -> MobilityRuntime0CompositionInput {
    let entity_owner = entity_owner_map(r1);
    let mut records: Vec<MobilityIdroute0LocalRecord> = Vec::new();
    let mut owner_records: Vec<MobilityOwner0LocalRecord> = Vec::new();
    for slice in &live_slices {
        let identity = entity_owner
            .get(&slice.entity_id)
            .copied()
            .map(identity_lane_for_owner)
            .unwrap_or(0);
        let faction = entity_owner.get(&slice.entity_id).copied().map(owner_from_r4).unwrap_or(1);
        records.push(MobilityIdroute0LocalRecord {
            entity_id: slice.entity_id,
            parent_key: slice.parent_key,
            identity: IdentityLane(identity),
            hard_value: 1,
            soft_value: 1.0,
        });
        owner_records.push(MobilityOwner0LocalRecord {
            entity_id: slice.entity_id,
            cell_key: slice.parent_key,
            cohort_count: 1,
            owner_columns: vec![MobilityOwner0ColumnValue {
                kind: MobilityOwner0ColumnKind::Faction,
                owner_id: faction,
            }],
            generation: 0,
            blocked_by_blockade: false,
            arrival_order: slice.entity_id,
        });
    }
    for mv in &moves {
        if !records.iter().any(|rec| rec.entity_id == mv.entity_id) {
            let identity = entity_owner
                .get(&mv.entity_id)
                .copied()
                .map(identity_lane_for_owner)
                .unwrap_or(0);
            records.push(MobilityIdroute0LocalRecord {
                entity_id: mv.entity_id,
                parent_key: mv.origin,
                identity: IdentityLane(identity),
                hard_value: 1,
                soft_value: 1.0,
            });
        }
        if !owner_records.iter().any(|rec| rec.entity_id == mv.entity_id) {
            let faction = entity_owner
                .get(&mv.entity_id)
                .copied()
                .map(owner_from_r4)
                .unwrap_or(1);
            owner_records.push(MobilityOwner0LocalRecord {
                entity_id: mv.entity_id,
                cell_key: mv.origin,
                cohort_count: 1,
                owner_columns: vec![MobilityOwner0ColumnValue {
                    kind: MobilityOwner0ColumnKind::Faction,
                    owner_id: faction,
                }],
                generation: 0,
                blocked_by_blockade: false,
                arrival_order: mv.entity_id,
            });
        }
    }

    MobilityRuntime0CompositionInput {
        config: MobilityRuntime0HarnessConfig::opt_in_test_harness(),
        alloc: MobilityAlloc0PlanInput {
            blocks: blocks.clone(),
            live_slices: live_slices.clone(),
            events: vec![],
            forbidden: MobilityAlloc0ForbiddenPathRequests::default(),
        },
        reenroll: MobilityReenroll0PlanInput {
            registry: MobilityReenroll0RegistryState {
                blocks,
                live_slices,
                origin_generations: BTreeMap::new(),
                destination_generations: BTreeMap::new(),
            },
            moves,
            forbidden: MobilityReenroll0ForbiddenPathRequests::default(),
        },
        idroute: MobilityIdroute0PlanInput {
            records,
            max_factions_per_cell: 4,
            forbidden: MobilityIdroute0ForbiddenPathRequests::default(),
        },
        econ: MobilityEcon0PlanInput {
            records: vec![],
            forbidden: MobilityEcon0ForbiddenPathRequests::default(),
        },
        owner: MobilityOwner0PlanInput {
            records: owner_records,
            overlays: vec![MobilityOwner0Overlay {
                owner: MobilityOwner0ColumnValue {
                    kind: MobilityOwner0ColumnKind::Faction,
                    owner_id: 1,
                },
                modifier_id: 42,
                modifier_amount: 0,
                arrival_order: 0,
            }],
            owner_changes: vec![],
            forbidden: MobilityOwner0ForbiddenPathRequests::default(),
        },
        forbidden: MobilityRuntime0ForbiddenPathRequests::default(),
    }
}

fn block_start_by_cell(blocks: &[MobilityAlloc0BlockSpec]) -> BTreeMap<u32, u32> {
    blocks
        .iter()
        .map(|block| (block.parent_key.key_id as u32, block.start_slot))
        .collect()
}

fn next_slot_in_cell(
    live_slices: &[MobilityAlloc0LiveSlice],
    block_by_cell: &BTreeMap<u32, u32>,
    cell_index: u32,
) -> u32 {
    let base = block_by_cell.get(&cell_index).copied().unwrap_or(0);
    let used = live_slices
        .iter()
        .filter(|slice| slice.parent_key == cell_key(cell_index))
        .count() as u32;
    base + used
}

fn build_initial_live_slices(
    r1: &DressRehearsalR1Report,
    block_by_cell: &BTreeMap<u32, u32>,
    active_entities: &BTreeSet<u64>,
) -> Vec<MobilityAlloc0LiveSlice> {
    let mut slices = Vec::new();
    let mut fleet_occupants: Vec<_> = r1
        .scenario
        .occupants
        .iter()
        .filter(|o| {
            matches!(
                o.kind,
                DressRehearsalR1OccupantKind::PirateFleet | DressRehearsalR1OccupantKind::PatrolFleet
            )
        })
        .collect();
    fleet_occupants.sort_by(|a, b| a.source_id.cmp(&b.source_id));
    for occupant in fleet_occupants {
        let entity_id = entity_id_for_mover(&occupant.source_id);
        if !active_entities.contains(&entity_id) {
            continue;
        }
        let slot = next_slot_in_cell(&slices, block_by_cell, occupant.cell_index);
        slices.push(MobilityAlloc0LiveSlice {
            entity_id,
            parent_key: cell_key(occupant.cell_index),
            slot,
        });
    }
    slices
}

fn build_blocks(cell_indices: &BTreeSet<u32>) -> Vec<MobilityAlloc0BlockSpec> {
    let mut blocks = Vec::new();
    let mut start_slot = 0u32;
    for &cell in cell_indices {
        blocks.push(MobilityAlloc0BlockSpec {
            parent_key: cell_key(cell),
            start_slot,
            slot_count: SLOTS_PER_CELL,
            reserved_headroom: SLOTS_PER_CELL / 2,
        });
        start_slot = start_slot.saturating_add(SLOTS_PER_CELL);
    }
    blocks
}

fn collect_cell_indices(
    moves: &[MobilityReenroll0Move],
    fission_rows: &[DressRehearsalR5FissionRow],
) -> BTreeSet<u32> {
    let mut cells = BTreeSet::new();
    for mv in moves {
        cells.insert(mv.origin.key_id as u32);
        cells.insert(mv.destination.key_id as u32);
    }
    for row in fission_rows {
        cells.insert(row.enrolled_cell_index);
    }
    cells
}

fn entities_in_cell(slices: &[MobilityAlloc0LiveSlice], cell_index: u32) -> Vec<u64> {
    let key = cell_key(cell_index);
    let mut ids: Vec<u64> = slices
        .iter()
        .filter(|slice| slice.parent_key == key)
        .map(|slice| slice.entity_id)
        .collect();
    ids.sort_unstable();
    ids
}

fn entity_owner_map(r1: &DressRehearsalR1Report) -> BTreeMap<u64, DressRehearsalR4Owner> {
    let mut map = BTreeMap::new();
    for occupant in &r1.scenario.occupants {
        if matches!(
            occupant.kind,
            DressRehearsalR1OccupantKind::PirateFleet | DressRehearsalR1OccupantKind::PatrolFleet
        ) {
            let owner = match occupant.owner {
                DressRehearsalR1Owner::Terran => DressRehearsalR4Owner::Terran,
                DressRehearsalR1Owner::Pirate => DressRehearsalR4Owner::Pirate,
            };
            map.insert(entity_id_for_mover(&occupant.source_id), owner);
        }
    }
    map
}

fn cell_key(cell_index: u32) -> MobilityAlloc0ParentKey {
    MobilityAlloc0ParentKey {
        parent_id: GALACTIC_PARENT_ID,
        key_id: u64::from(cell_index),
    }
}

fn entity_id_for_mover(mover_id: &str) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in mover_id.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn boundary_request_id_for(mover_id: &str, source: u32, dest: u32) -> u64 {
    BOUNDARY_REQUEST_ID_BASE
        ^ entity_id_for_mover(mover_id)
        ^ (u64::from(source) << 32)
        ^ u64::from(dest)
}

fn identity_lane_for_owner(owner: DressRehearsalR4Owner) -> u32 {
    match owner {
        DressRehearsalR4Owner::Terran => 0,
        DressRehearsalR4Owner::Pirate => 1,
    }
}

fn owner_from_r4(owner: DressRehearsalR4Owner) -> u64 {
    match owner {
        DressRehearsalR4Owner::Terran => 1,
        DressRehearsalR4Owner::Pirate => 2,
    }
}

fn owner_from_faction(owner_id: u64) -> DressRehearsalR5Owner {
    if owner_id == 2 {
        DressRehearsalR5Owner::Pirate
    } else {
        DressRehearsalR5Owner::Terran
    }
}

fn validate_input(input: &DressRehearsalR5Input, diagnostics: &mut Vec<&'static str>) {
    if input.enabled_by_default {
        diagnostics.push("enabled_by_default");
    }
    if !input.explicit_opt_in {
        return;
    }
    let Some(r1) = input.r1_report.as_ref() else {
        diagnostics.push("missing_r1_report");
        return;
    };
    let Some(r2) = input.r2_report.as_ref() else {
        diagnostics.push("missing_r2_report");
        return;
    };
    let Some(r3) = input.r3_report.as_ref() else {
        diagnostics.push("missing_r3_report");
        return;
    };
    let Some(r4) = input.r4_report.as_ref() else {
        diagnostics.push("missing_r4_report");
        return;
    };
    if !r1.admitted || !r1.cpu_oracle_parity {
        diagnostics.push("r1_not_admitted");
    }
    if !r2.admitted || !r2.cpu_oracle_parity {
        diagnostics.push("r2_not_admitted");
    }
    if !r3.admitted || !r3.cpu_oracle_parity {
        diagnostics.push("r3_not_admitted");
    }
    if !r4.admitted || !r4.cpu_oracle_parity {
        diagnostics.push("r4_not_admitted");
    }
}

fn base_report(
    input: &DressRehearsalR5Input,
    disabled_no_op: bool,
    diagnostics: Vec<&'static str>,
    execution: Option<DressRehearsalR5Oracle>,
    cpu_oracle_parity: bool,
) -> DressRehearsalR5Report {
    let admitted = diagnostics.is_empty();
    let opt_in = input.explicit_opt_in;
    let empty_summary = DressRehearsalR5Summary {
        movement_row_count: 0,
        sit_still_row_count: 0,
        boundary_request_count: 0,
        fission_row_count: 0,
        mobility_substrate_admitted: false,
        stable_checksum: 0,
    };
    let (
        movement_rows,
        sit_still_rows,
        boundary_request_rows,
        fission_rows,
        summary,
        mobility_substrate_admitted,
        mobility_substrate_diagnostics,
        mobility_composed_cpu_checksum,
        fission_substrate_available,
        fission_blocked_reason,
    ) = match execution {
        Some(exec) => (
            exec.movement_rows.clone(),
            exec.sit_still_rows.clone(),
            exec.boundary_request_rows.clone(),
            exec.fission_rows.clone(),
            exec.summary.clone(),
            exec.summary.mobility_substrate_admitted,
            exec.mobility_substrate_diagnostics.clone(),
            exec.mobility_composed_cpu_checksum,
            !exec.fission_rows.is_empty(),
            None,
        ),
        None => (
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            empty_summary.clone(),
            false,
            Vec::new(),
            0,
            false,
            if opt_in && !disabled_no_op {
                Some("execution_not_run")
            } else {
                None
            },
        ),
    };

    let r1 = input.r1_report.as_ref();
    let r2 = input.r2_report.as_ref();
    let r3 = input.r3_report.as_ref();
    let r4 = input.r4_report.as_ref();

    let markdown = render_artifact_markdown(
        &movement_rows,
        &sit_still_rows,
        &boundary_request_rows,
        &fission_rows,
        &summary,
        cpu_oracle_parity,
        r1.map(|r| r.starmap_summary.stable_checksum).unwrap_or(0),
        r2.map(|r| r.summary.stable_checksum).unwrap_or(0),
        r3.map(|r| r.summary.stable_checksum).unwrap_or(0),
        r4.map(|r| r.summary.stable_checksum).unwrap_or(0),
    );
    let replay_checksum = if admitted && opt_in && !disabled_no_op {
        summary.stable_checksum
    } else {
        0
    };
    let artifact = DressRehearsalR5Artifact {
        movement_rows: movement_rows.clone(),
        sit_still_rows: sit_still_rows.clone(),
        boundary_request_rows: boundary_request_rows.clone(),
        fission_rows: fission_rows.clone(),
        summary: summary.clone(),
        cpu_oracle_parity,
        markdown,
    };

    DressRehearsalR5Report {
        id: DRESS_REHEARSAL_R5_MOVEMENT_REENROLL_ID,
        status: DRESS_REHEARSAL_R5_MOVEMENT_REENROLL_STATUS_PASS,
        scenario_name: DRESS_REHEARSAL_R5_SCENARIO,
        admitted,
        diagnostics,
        explicit_opt_in: opt_in,
        default_off: !input.enabled_by_default,
        disabled_no_op,
        r1_contract_checksum: r1.map(|r| r.starmap_summary.stable_checksum).unwrap_or(0),
        r2_contract_checksum: r2.map(|r| r.summary.stable_checksum).unwrap_or(0),
        r3_contract_checksum: r3.map(|r| r.summary.stable_checksum).unwrap_or(0),
        r4_contract_checksum: r4.map(|r| r.summary.stable_checksum).unwrap_or(0),
        r1_cpu_oracle_parity: r1.map(|r| r.cpu_oracle_parity).unwrap_or(false),
        r2_cpu_oracle_parity: r2.map(|r| r.cpu_oracle_parity).unwrap_or(false),
        r3_cpu_oracle_parity: r3.map(|r| r.cpu_oracle_parity).unwrap_or(false),
        r4_cpu_oracle_parity: r4.map(|r| r.cpu_oracle_parity).unwrap_or(false),
        movement_rows,
        sit_still_rows,
        boundary_request_rows,
        fission_rows,
        artifact,
        summary,
        mobility_substrate_admitted,
        mobility_substrate_diagnostics,
        mobility_composed_cpu_checksum,
        fission_substrate_available,
        fission_blocked_reason,
        direct_movement_command: false,
        external_boundary_request: false,
        cpu_planner_used: false,
        default_simsession_pass_graph_change: false,
        new_shader_or_wgsl: false,
        cpu_oracle_parity,
        deterministic_replay_checksum: replay_checksum,
    }
}

fn render_artifact_markdown(
    movement_rows: &[DressRehearsalR5MovementRow],
    sit_still_rows: &[DressRehearsalR5SitStillRow],
    boundary_rows: &[DressRehearsalR5BoundaryRequestRow],
    fission_rows: &[DressRehearsalR5FissionRow],
    summary: &DressRehearsalR5Summary,
    cpu_oracle_parity: bool,
    r1_checksum: u64,
    r2_checksum: u64,
    r3_checksum: u64,
    r4_checksum: u64,
) -> String {
    let mut out = String::new();
    out.push_str("# SCENARIO-0080-2 R5 movement / REENROLL\n\n");
    out.push_str(&format!(
        "- checksum: `{:016x}`\n- cpu_oracle_parity: {cpu_oracle_parity}\n",
        summary.stable_checksum
    ));
    out.push_str(&format!(
        "- upstream: R1=`{r1_checksum:016x}` R2=`{r2_checksum:016x}` R3=`{r3_checksum:016x}` R4=`{r4_checksum:016x}`\n"
    ));
    out.push_str(&format!(
        "- movement_rows: {} sit_still: {} boundary_requests: {} fission: {}\n",
        movement_rows.len(),
        sit_still_rows.len(),
        boundary_rows.len(),
        fission_rows.len()
    ));
    out
}

fn checksum_r5(
    r1: u64,
    r2: u64,
    r3: u64,
    r4: u64,
    movement: &[DressRehearsalR5MovementRow],
    sit_still: &[DressRehearsalR5SitStillRow],
    boundary: &[DressRehearsalR5BoundaryRequestRow],
    fission: &[DressRehearsalR5FissionRow],
) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for part in [r1, r2, r3, r4] {
        hash ^= part;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for row in movement {
        hash ^= row.entity_id;
        hash ^= u64::from(row.source_cell_index);
        hash ^= u64::from(row.destination_cell_index);
        hash ^= u64::from(row.movement_applied as u8);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for row in sit_still {
        hash ^= entity_id_for_mover(&row.mover_id);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for row in boundary {
        hash ^= row.boundary_request_id;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for row in fission {
        hash ^= row.new_fleet_entity_id;
        hash ^= u64::from(row.fission_applied as u8);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn empty_oracle() -> DressRehearsalR5Oracle {
    DressRehearsalR5Oracle {
        movement_rows: Vec::new(),
        sit_still_rows: Vec::new(),
        boundary_request_rows: Vec::new(),
        fission_rows: Vec::new(),
        summary: DressRehearsalR5Summary {
            movement_row_count: 0,
            sit_still_row_count: 0,
            boundary_request_count: 0,
            fission_row_count: 0,
            mobility_substrate_admitted: false,
            stable_checksum: 0,
        },
        mobility_substrate_diagnostics: Vec::new(),
        mobility_composed_cpu_checksum: 0,
    }
}
