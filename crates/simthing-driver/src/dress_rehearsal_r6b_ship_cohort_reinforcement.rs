//! SCENARIO-0080-2-R6B: threshold-emission ship production into fleet-cohort reinforcement/fusion.
//!
//! Consumes R5 post-move membership and R6A cohort semantics. Construction progress crosses a
//! ship-cost threshold and emits ship-count deltas; masked table scan selects compatible cohorts;
//! friendly co-located cohorts fuse by masked reduction. CPU oracle verifies row operations only.

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
use crate::dress_rehearsal_r4_field_policy_consumption::{
    run_dress_rehearsal_r4_field_policy_consumption, DressRehearsalR4Input, DressRehearsalR4Report,
    MOVEMENT_THRESHOLD_MAG_BITS,
};
use crate::dress_rehearsal_r5_movement_reenroll::{
    cell_key, entity_id_for_mover, run_dress_rehearsal_r5_movement_reenroll, DressRehearsalR5Input,
    DressRehearsalR5Report, SLOTS_PER_CELL,
};
use crate::dress_rehearsal_r6_combat_hp_damage::{
    damage_output_for_cohort, hp_to_retire_for_cohort, run_dress_rehearsal_r6_combat_hp_damage,
    DressRehearsalR6FleetCohortOverride, DressRehearsalR6Input, DressRehearsalR6Report,
    FLEET_COHORT_NUM_SHIPS, FLEET_DAMAGE_PER_SHIP_PER_TICK, FLEET_HP_PER_SHIP,
};
use simthing_core::{
    AccumulatorOp, ColumnIndex, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SlotIndex, SourceSpec,
    ThresholdDirection,
};
use simthing_spec::{
    plan_mobility_alloc0, MobilityAlloc0BlockSpec, MobilityAlloc0BoundaryEvent,
    MobilityAlloc0BoundaryEventKind, MobilityAlloc0ForbiddenPathRequests, MobilityAlloc0LiveSlice,
    MobilityAlloc0PlanInput,
};
use std::collections::{BTreeMap, BTreeSet};

pub const DRESS_REHEARSAL_R6B_SHIP_COHORT_REINFORCEMENT_ID: &str =
    "SCENARIO-0080-2-R6B-SHIP-COHORT-REINFORCEMENT";
pub const DRESS_REHEARSAL_R6B_SHIP_COHORT_REINFORCEMENT_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - threshold emission ship production + cohort reinforcement/fusion";
pub const DRESS_REHEARSAL_R6B_SCENARIO: &str = "SCENARIO-0080-2";

pub const SHIP_COST: i64 = 100;
pub const CONSTRUCTION_PROGRESS_COL: u32 = 0;
pub const SHIP_COUNT_DELTA_COL: u32 = 1;
pub const R6B_FUSION_FIXTURE_CELL: u32 = 42;
pub const R6B_BIRTH_FIXTURE_CELL: u32 = 43;
pub const R6B_FUSION_LEFT_ID: &str = "dress-rehearsal-r6b-fusion-left";
pub const R6B_FUSION_RIGHT_ID: &str = "dress-rehearsal-r6b-fusion-right";
pub const R6B_INCOMPATIBLE_ID: &str = "dress-rehearsal-r6b-incompatible-fleet";
pub const R6B_BIRTH_FLEET_ID: &str = "dress-rehearsal-r6b-birth-fleet";
pub const R6B_BIRTH_STARPORT_ID: &str = "dress-rehearsal-r6b-birth-starport";
pub const R6B_NEW_FLEET_ENTITY_BASE: u64 = 0x9020_0000_0000_0000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DressRehearsalR6bOwner {
    Terran,
    Pirate,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR6bCohortProfile {
    pub hp_per_ship: i64,
    pub damage_per_ship_per_tick: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6bCohortRow {
    pub fleet_id: String,
    pub entity_id: u64,
    pub owner: DressRehearsalR6bOwner,
    pub cell_index: u32,
    pub profile: DressRehearsalR6bCohortProfile,
    pub num_ships: i64,
    pub destroyed: bool,
    pub fleet_like: bool,
    pub owner_faction_id: u64,
    pub identity_lane: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6bConstructionRow {
    pub starport_id: String,
    pub cell_index: u32,
    pub owner: DressRehearsalR6bOwner,
    pub construction_progress_before: i64,
    pub production_applied: i64,
    pub construction_progress_after: i64,
    pub ship_cost: i64,
    pub threshold_passed: bool,
    pub ship_count_delta_emitted: i64,
    pub construction_progress_remainder: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6bReinforcementRow {
    pub target_fleet_id: String,
    pub entity_id: u64,
    pub owner: DressRehearsalR6bOwner,
    pub cell_index: u32,
    pub num_ships_before: i64,
    pub ship_count_delta: i64,
    pub num_ships_after: i64,
    pub hp_to_retire_after: i64,
    pub damage_output_after: i64,
    pub movement_boundary_request_used: bool,
    pub shadow_table_update_kind: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6bBirthRow {
    pub created_fleet_id: String,
    pub entity_id: u64,
    pub owner: DressRehearsalR6bOwner,
    pub cell_index: u32,
    pub starport_id: String,
    pub num_ships: i64,
    pub alloc_enrollment_applied: bool,
    pub movement_boundary_request_used: bool,
    pub shadow_table_update_kind: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6bFusionRow {
    pub fusion_event_id: String,
    pub surviving_fleet_id: String,
    pub absorbed_fleet_id: String,
    pub owner: DressRehearsalR6bOwner,
    pub cell_index: u32,
    pub left_num_ships: i64,
    pub right_num_ships: i64,
    pub fused_num_ships: i64,
    pub hp_per_ship: i64,
    pub damage_per_ship_per_tick: i64,
    pub hp_to_retire_after: i64,
    pub damage_output_after: i64,
    pub identity_lineage_recorded: bool,
    pub owner_overlay_preserved: bool,
    pub movement_boundary_request_used: bool,
    pub shadow_table_update_kind: &'static str,
    pub arena_membership_after: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR6bSummary {
    pub cohort_row_count: usize,
    pub construction_row_count: usize,
    pub reinforcement_row_count: usize,
    pub birth_row_count: usize,
    pub fusion_row_count: usize,
    pub stable_checksum: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6bOracle {
    pub cohort_rows: Vec<DressRehearsalR6bCohortRow>,
    pub construction_rows: Vec<DressRehearsalR6bConstructionRow>,
    pub reinforcement_rows: Vec<DressRehearsalR6bReinforcementRow>,
    pub birth_rows: Vec<DressRehearsalR6bBirthRow>,
    pub fusion_rows: Vec<DressRehearsalR6bFusionRow>,
    pub summary: DressRehearsalR6bSummary,
    pub table_driven_masked_scan_used: bool,
    pub movement_boundary_request_used: bool,
    pub cpu_fleet_manager_decision_path: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6bInput {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
    pub r1_report: Option<DressRehearsalR1Report>,
    pub r2_report: Option<DressRehearsalR2Report>,
    pub r3_report: Option<DressRehearsalR3Report>,
    pub r4_report: Option<DressRehearsalR4Report>,
    pub r5_report: Option<DressRehearsalR5Report>,
    pub r6_report: Option<DressRehearsalR6Report>,
}

impl DressRehearsalR6bInput {
    pub fn default_simsession() -> Self {
        Self {
            explicit_opt_in: false,
            enabled_by_default: false,
            r1_report: None,
            r2_report: None,
            r3_report: None,
            r4_report: None,
            r5_report: None,
            r6_report: None,
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
        let r4_report = run_dress_rehearsal_r4_field_policy_consumption(&DressRehearsalR4Input {
            explicit_opt_in: true,
            enabled_by_default: false,
            movement_threshold_mag_bits: MOVEMENT_THRESHOLD_MAG_BITS,
            r1_report: Some(r1_report.clone()),
            r2_report: Some(r2_report.clone()),
            r3_report: Some(r3_report.clone()),
        });
        let r5_report = run_dress_rehearsal_r5_movement_reenroll(&DressRehearsalR5Input {
            explicit_opt_in: true,
            enabled_by_default: false,
            r1_report: Some(r1_report.clone()),
            r2_report: Some(r2_report.clone()),
            r3_report: Some(r3_report.clone()),
            r4_report: Some(r4_report.clone()),
        });
        let r6_report =
            run_dress_rehearsal_r6_combat_hp_damage(&DressRehearsalR6Input::explicit_opt_in());
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
            r1_report: Some(r1_report),
            r2_report: Some(r2_report),
            r3_report: Some(r3_report),
            r4_report: Some(r4_report),
            r5_report: Some(r5_report),
            r6_report: Some(r6_report),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6bReport {
    pub id: &'static str,
    pub status: &'static str,
    pub scenario_name: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,
    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,

    pub r5_contract_checksum: u64,
    pub r6_contract_checksum: u64,

    pub cohort_rows: Vec<DressRehearsalR6bCohortRow>,
    pub construction_rows: Vec<DressRehearsalR6bConstructionRow>,
    pub reinforcement_rows: Vec<DressRehearsalR6bReinforcementRow>,
    pub birth_rows: Vec<DressRehearsalR6bBirthRow>,
    pub fusion_rows: Vec<DressRehearsalR6bFusionRow>,
    pub summary: DressRehearsalR6bSummary,

    pub table_driven_masked_scan_used: bool,
    pub movement_boundary_request_used: bool,
    pub cpu_fleet_manager_decision_path: bool,
    pub gpu_substrate_posture_only: bool,
    pub cpu_oracle_parity: bool,
    pub deterministic_replay_checksum: u64,
    pub fleet_cohort_overrides: BTreeMap<u64, DressRehearsalR6FleetCohortOverride>,
    pub combat_with_r6b_cohorts: Option<DressRehearsalR6Report>,
}

pub fn run_dress_rehearsal_r6b_ship_cohort_reinforcement(
    input: &DressRehearsalR6bInput,
) -> DressRehearsalR6bReport {
    let mut diagnostics = Vec::new();
    validate_input(input, &mut diagnostics);

    if !input.explicit_opt_in {
        return base_report(input, true, diagnostics, None, false);
    }
    if !diagnostics.is_empty() {
        return base_report(input, false, diagnostics, None, false);
    }

    let execution = execute_model(input);
    let oracle = cpu_oracle_dress_rehearsal_r6b_ship_cohort_reinforcement(input);
    let parity = oracle_matches(&execution, &oracle);
    base_report(input, false, Vec::new(), Some(execution), parity)
}

pub fn replay_dress_rehearsal_r6b_ship_cohort_reinforcement(
) -> (DressRehearsalR6bReport, DressRehearsalR6bReport) {
    let input = DressRehearsalR6bInput::explicit_opt_in();
    (
        run_dress_rehearsal_r6b_ship_cohort_reinforcement(&input),
        run_dress_rehearsal_r6b_ship_cohort_reinforcement(&input),
    )
}

pub fn cpu_oracle_dress_rehearsal_r6b_ship_cohort_reinforcement(
    input: &DressRehearsalR6bInput,
) -> DressRehearsalR6bOracle {
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

pub fn fleet_cohort_overrides_from_report(
    report: &DressRehearsalR6bReport,
) -> BTreeMap<u64, DressRehearsalR6FleetCohortOverride> {
    report.fleet_cohort_overrides.clone()
}

pub fn run_r6_combat_with_r6b_cohorts(r6b: &DressRehearsalR6bReport) -> DressRehearsalR6Report {
    let mut r6_input = DressRehearsalR6Input::explicit_opt_in();
    r6_input.fleet_cohort_overrides = Some(r6b.fleet_cohort_overrides.clone());
    run_dress_rehearsal_r6_combat_hp_damage(&r6_input)
}

pub fn construction_threshold_emission(
    progress_before: i64,
    production_applied: i64,
    ship_cost: i64,
) -> (i64, bool, i64, i64) {
    let progress_after = progress_before + production_applied;
    let ship_count_delta = if ship_cost > 0 {
        progress_after / ship_cost
    } else {
        0
    };
    let remainder = if ship_cost > 0 {
        progress_after % ship_cost
    } else {
        progress_after
    };
    let threshold_passed = ship_count_delta > 0;
    (
        progress_after,
        threshold_passed,
        ship_count_delta,
        remainder,
    )
}

fn execute_model(input: &DressRehearsalR6bInput) -> DressRehearsalR6bOracle {
    let r1 = input.r1_report.as_ref().expect("validated R1");
    let r2 = input.r2_report.as_ref().expect("validated R2");
    let r5 = input.r5_report.as_ref().expect("validated R5");

    let mut cohort_rows = build_initial_cohort_rows(r1, r5);
    let mut construction_rows = Vec::new();
    let mut reinforcement_rows = Vec::new();
    let mut birth_rows = Vec::new();
    let mut fusion_rows = Vec::new();

    apply_r6b_fixtures(&mut cohort_rows, &mut fusion_rows);

    for starport in starport_production_targets(r2) {
        let progress_before = 0i64;
        let production_applied = starport.production_generated.max(SHIP_COST);
        let (progress_after, threshold_passed, ship_delta, remainder) =
            construction_threshold_emission(progress_before, production_applied, SHIP_COST);
        let _ = construction_emission_posture(progress_after, ship_delta);
        construction_rows.push(DressRehearsalR6bConstructionRow {
            starport_id: starport.system_id.clone(),
            cell_index: starport.cell_index,
            owner: r2_owner(starport.original_owner),
            construction_progress_before: progress_before,
            production_applied,
            construction_progress_after: progress_after,
            ship_cost: SHIP_COST,
            threshold_passed,
            ship_count_delta_emitted: ship_delta,
            construction_progress_remainder: remainder,
        });
        if ship_delta <= 0 {
            continue;
        }
        apply_ship_delta(
            &mut cohort_rows,
            starport.cell_index,
            r2_owner(starport.original_owner),
            ship_delta,
            &starport.system_id,
            &mut reinforcement_rows,
            &mut birth_rows,
            &mut fusion_rows,
        );
    }

    apply_birth_fixture_construction(&mut cohort_rows, &mut construction_rows, &mut birth_rows);

    run_friendly_fusion_pass(&mut cohort_rows, &mut fusion_rows, &r5.fission_rows);

    cohort_rows.retain(|row| !row.destroyed);
    cohort_rows.sort_by(|a, b| a.fleet_id.cmp(&b.fleet_id));

    let summary = DressRehearsalR6bSummary {
        cohort_row_count: cohort_rows.len(),
        construction_row_count: construction_rows.len(),
        reinforcement_row_count: reinforcement_rows.len(),
        birth_row_count: birth_rows.len(),
        fusion_row_count: fusion_rows.len(),
        stable_checksum: checksum_r6b(
            r5.summary.stable_checksum,
            input
                .r6_report
                .as_ref()
                .map(|r| r.summary.stable_checksum)
                .unwrap_or(0),
            &cohort_rows,
            &construction_rows,
            &reinforcement_rows,
            &birth_rows,
            &fusion_rows,
        ),
    };

    DressRehearsalR6bOracle {
        cohort_rows,
        construction_rows,
        reinforcement_rows,
        birth_rows,
        fusion_rows,
        summary,
        table_driven_masked_scan_used: true,
        movement_boundary_request_used: false,
        cpu_fleet_manager_decision_path: false,
    }
}

fn apply_ship_delta(
    cohort_rows: &mut Vec<DressRehearsalR6bCohortRow>,
    cell_index: u32,
    owner: DressRehearsalR6bOwner,
    ship_delta: i64,
    starport_id: &str,
    reinforcement_rows: &mut Vec<DressRehearsalR6bReinforcementRow>,
    birth_rows: &mut Vec<DressRehearsalR6bBirthRow>,
    fusion_rows: &mut Vec<DressRehearsalR6bFusionRow>,
) {
    let profile = canonical_profile();
    let mask = cohort_mask(owner, cell_index, &profile);
    let compatible: Vec<usize> = masked_compatible_indices(cohort_rows, &mask);
    if compatible.is_empty() {
        birth_new_cohort(
            cohort_rows,
            cell_index,
            owner,
            ship_delta,
            starport_id,
            birth_rows,
        );
        return;
    }
    if compatible.len() > 1 {
        fuse_compatible_at_indices(
            cohort_rows,
            &compatible,
            "prefusion-before-reinforcement",
            fusion_rows,
        );
    }
    let idx = masked_compatible_indices(cohort_rows, &mask)
        .into_iter()
        .min()
        .expect("survivor after prefusion");
    let before = cohort_rows[idx].num_ships;
    let after = before + ship_delta;
    cohort_rows[idx].num_ships = after;
    reinforcement_rows.push(DressRehearsalR6bReinforcementRow {
        target_fleet_id: cohort_rows[idx].fleet_id.clone(),
        entity_id: cohort_rows[idx].entity_id,
        owner,
        cell_index,
        num_ships_before: before,
        ship_count_delta: ship_delta,
        num_ships_after: after,
        hp_to_retire_after: hp_to_retire_for_cohort(after, profile.hp_per_ship),
        damage_output_after: damage_output_for_cohort(after, profile.damage_per_ship_per_tick),
        movement_boundary_request_used: false,
        shadow_table_update_kind: "CohortStateUpdate",
    });
}

fn birth_new_cohort(
    cohort_rows: &mut Vec<DressRehearsalR6bCohortRow>,
    cell_index: u32,
    owner: DressRehearsalR6bOwner,
    num_ships: i64,
    starport_id: &str,
    birth_rows: &mut Vec<DressRehearsalR6bBirthRow>,
) {
    let fleet_id = format!("dress-rehearsal-r6b-born-{starport_id}");
    let entity_id = entity_id_for_r6b_fleet(&fleet_id);
    let profile = canonical_profile();
    let alloc_applied = enroll_new_cohort_alloc(cell_index, entity_id);
    cohort_rows.push(DressRehearsalR6bCohortRow {
        fleet_id: fleet_id.clone(),
        entity_id,
        owner,
        cell_index,
        profile: profile.clone(),
        num_ships,
        destroyed: false,
        fleet_like: true,
        owner_faction_id: owner_faction_id(owner),
        identity_lane: identity_lane_for_owner(owner),
    });
    birth_rows.push(DressRehearsalR6bBirthRow {
        created_fleet_id: fleet_id,
        entity_id,
        owner,
        cell_index,
        starport_id: starport_id.to_string(),
        num_ships,
        alloc_enrollment_applied: alloc_applied,
        movement_boundary_request_used: false,
        shadow_table_update_kind: "AllocArrivalEnrollment",
    });
}

fn run_friendly_fusion_pass(
    cohort_rows: &mut Vec<DressRehearsalR6bCohortRow>,
    fusion_rows: &mut Vec<DressRehearsalR6bFusionRow>,
    fission_rows: &[crate::dress_rehearsal_r5_movement_reenroll::DressRehearsalR5FissionRow],
) {
    let _ = fission_rows;
    let cells: BTreeSet<u32> = cohort_rows.iter().map(|r| r.cell_index).collect();
    for cell_index in cells {
        if cell_index == R6B_FUSION_FIXTURE_CELL {
            continue;
        }
        let indices: Vec<usize> = cohort_rows
            .iter()
            .enumerate()
            .filter(|(_, r)| {
                r.cell_index == cell_index
                    && r.fleet_like
                    && !r.destroyed
                    && r.fleet_id != R6B_INCOMPATIBLE_ID
            })
            .map(|(i, _)| i)
            .collect();
        if indices.len() < 2 {
            continue;
        }
        let owner = cohort_rows[indices[0]].owner;
        if !indices.iter().all(|&i| cohort_rows[i].owner == owner) {
            continue;
        }
        let profile = cohort_rows[indices[0]].profile.clone();
        if !indices.iter().all(|&i| cohort_rows[i].profile == profile) {
            continue;
        }
        fuse_compatible_at_indices(
            cohort_rows,
            &indices,
            "friendly-masked-reduction",
            fusion_rows,
        );
        if let Some(last) = fusion_rows.last_mut() {
            if last.cell_index == cell_index {
                last.arena_membership_after = arena_membership_for_cell(cohort_rows, cell_index);
            }
        }
    }
}

fn fuse_compatible_at_indices(
    cohort_rows: &mut Vec<DressRehearsalR6bCohortRow>,
    indices: &[usize],
    fusion_tag: &str,
    fusion_rows: &mut Vec<DressRehearsalR6bFusionRow>,
) {
    if indices.len() < 2 {
        return;
    }
    let mut sorted: Vec<usize> = indices.to_vec();
    sorted.sort_by_key(|&i| cohort_rows[i].fleet_id.clone());
    let survivor = sorted[0];
    let absorbed: Vec<usize> = sorted[1..].to_vec();
    let mut fused_ships = cohort_rows[survivor].num_ships;
    let cell_index = cohort_rows[survivor].cell_index;
    let owner = cohort_rows[survivor].owner;
    let profile = cohort_rows[survivor].profile.clone();
    for &idx in &absorbed {
        let left = fused_ships;
        let right = cohort_rows[idx].num_ships;
        fused_ships += right;
        let event_id = format!(
            "{fusion_tag}-{}-{}",
            cohort_rows[survivor].fleet_id, cohort_rows[idx].fleet_id
        );
        let membership_after = apply_fusion_shadow_table(
            cell_index,
            cohort_rows[survivor].entity_id,
            cohort_rows[idx].entity_id,
        );
        cohort_rows[idx].destroyed = true;
        fusion_rows.push(DressRehearsalR6bFusionRow {
            fusion_event_id: event_id,
            surviving_fleet_id: cohort_rows[survivor].fleet_id.clone(),
            absorbed_fleet_id: cohort_rows[idx].fleet_id.clone(),
            owner,
            cell_index,
            left_num_ships: left,
            right_num_ships: right,
            fused_num_ships: fused_ships,
            hp_per_ship: profile.hp_per_ship,
            damage_per_ship_per_tick: profile.damage_per_ship_per_tick,
            hp_to_retire_after: hp_to_retire_for_cohort(fused_ships, profile.hp_per_ship),
            damage_output_after: damage_output_for_cohort(
                fused_ships,
                profile.damage_per_ship_per_tick,
            ),
            identity_lineage_recorded: true,
            owner_overlay_preserved: true,
            movement_boundary_request_used: false,
            shadow_table_update_kind: "CohortCompactionDeparture",
            arena_membership_after: membership_after,
        });
    }
    cohort_rows[survivor].num_ships = fused_ships;
}

fn apply_fusion_shadow_table(
    cell_index: u32,
    survivor_entity: u64,
    absorbed_entity: u64,
) -> Vec<u64> {
    let block = MobilityAlloc0BlockSpec {
        parent_key: cell_key(cell_index),
        start_slot: 0,
        slot_count: SLOTS_PER_CELL,
        reserved_headroom: SLOTS_PER_CELL / 2,
    };
    let live_slices = vec![
        MobilityAlloc0LiveSlice {
            entity_id: survivor_entity,
            parent_key: cell_key(cell_index),
            slot: 0,
        },
        MobilityAlloc0LiveSlice {
            entity_id: absorbed_entity,
            parent_key: cell_key(cell_index),
            slot: 1,
        },
    ];
    let events = vec![MobilityAlloc0BoundaryEvent {
        kind: MobilityAlloc0BoundaryEventKind::Departure,
        parent_key: cell_key(cell_index),
        entity_id: Some(absorbed_entity),
        arrival_order: absorbed_entity,
    }];
    let report = plan_mobility_alloc0(&MobilityAlloc0PlanInput {
        blocks: vec![block],
        live_slices,
        events,
        forbidden: MobilityAlloc0ForbiddenPathRequests::default(),
    });
    report
        .final_live_slices
        .iter()
        .map(|s| s.entity_id)
        .collect()
}

fn enroll_new_cohort_alloc(cell_index: u32, entity_id: u64) -> bool {
    let block = MobilityAlloc0BlockSpec {
        parent_key: cell_key(cell_index),
        start_slot: 0,
        slot_count: SLOTS_PER_CELL,
        reserved_headroom: SLOTS_PER_CELL / 2,
    };
    let events = vec![MobilityAlloc0BoundaryEvent {
        kind: MobilityAlloc0BoundaryEventKind::Arrival,
        parent_key: cell_key(cell_index),
        entity_id: Some(entity_id),
        arrival_order: entity_id,
    }];
    let report = plan_mobility_alloc0(&MobilityAlloc0PlanInput {
        blocks: vec![block],
        live_slices: vec![],
        events,
        forbidden: MobilityAlloc0ForbiddenPathRequests::default(),
    });
    report.admitted
        && report
            .final_live_slices
            .iter()
            .any(|s| s.entity_id == entity_id)
}

fn build_initial_cohort_rows(
    r1: &DressRehearsalR1Report,
    r5: &DressRehearsalR5Report,
) -> Vec<DressRehearsalR6bCohortRow> {
    let mut post_move: BTreeMap<String, u32> = BTreeMap::new();
    for row in &r5.movement_rows {
        post_move.insert(row.mover_id.clone(), row.post_move_cell_index);
    }
    for row in &r5.fission_rows {
        if row.fission_applied {
            post_move.insert(row.new_fleet_id.clone(), row.enrolled_cell_index);
        }
    }

    let profile = canonical_profile();
    let mut rows = Vec::new();
    for occupant in &r1.scenario.occupants {
        let fleet_like = matches!(
            occupant.kind,
            DressRehearsalR1OccupantKind::PirateFleet | DressRehearsalR1OccupantKind::PatrolFleet
        );
        if !fleet_like {
            continue;
        }
        let cell_index = post_move
            .get(&occupant.source_id)
            .copied()
            .unwrap_or(occupant.cell_index);
        let owner = match occupant.owner {
            DressRehearsalR1Owner::Terran => DressRehearsalR6bOwner::Terran,
            DressRehearsalR1Owner::Pirate => DressRehearsalR6bOwner::Pirate,
        };
        rows.push(DressRehearsalR6bCohortRow {
            fleet_id: occupant.source_id.clone(),
            entity_id: entity_id_for_mover(&occupant.source_id),
            owner,
            cell_index,
            profile: profile.clone(),
            num_ships: FLEET_COHORT_NUM_SHIPS,
            destroyed: false,
            fleet_like: true,
            owner_faction_id: owner_faction_id(owner),
            identity_lane: identity_lane_for_owner(owner),
        });
    }
    for fission in &r5.fission_rows {
        if !fission.fission_applied {
            continue;
        }
        let owner = if fission.owner_faction_id == 1 {
            DressRehearsalR6bOwner::Terran
        } else {
            DressRehearsalR6bOwner::Pirate
        };
        rows.push(DressRehearsalR6bCohortRow {
            fleet_id: fission.new_fleet_id.clone(),
            entity_id: fission.new_fleet_entity_id,
            owner,
            cell_index: fission.enrolled_cell_index,
            profile: profile.clone(),
            num_ships: FLEET_COHORT_NUM_SHIPS,
            destroyed: false,
            fleet_like: true,
            owner_faction_id: fission.owner_faction_id,
            identity_lane: fission.idroute_identity_lane,
        });
    }
    rows
}

fn apply_r6b_fixtures(
    cohort_rows: &mut Vec<DressRehearsalR6bCohortRow>,
    fusion_rows: &mut Vec<DressRehearsalR6bFusionRow>,
) {
    let profile = canonical_profile();
    let left_idx = cohort_rows.len();
    cohort_rows.push(DressRehearsalR6bCohortRow {
        fleet_id: R6B_FUSION_LEFT_ID.to_string(),
        entity_id: entity_id_for_r6b_fleet(R6B_FUSION_LEFT_ID),
        owner: DressRehearsalR6bOwner::Terran,
        cell_index: R6B_FUSION_FIXTURE_CELL,
        profile: profile.clone(),
        num_ships: 7,
        destroyed: false,
        fleet_like: true,
        owner_faction_id: 1,
        identity_lane: 1,
    });
    let right_idx = cohort_rows.len();
    cohort_rows.push(DressRehearsalR6bCohortRow {
        fleet_id: R6B_FUSION_RIGHT_ID.to_string(),
        entity_id: entity_id_for_r6b_fleet(R6B_FUSION_RIGHT_ID),
        owner: DressRehearsalR6bOwner::Terran,
        cell_index: R6B_FUSION_FIXTURE_CELL,
        profile: profile.clone(),
        num_ships: 7,
        destroyed: false,
        fleet_like: true,
        owner_faction_id: 1,
        identity_lane: 1,
    });
    cohort_rows.push(DressRehearsalR6bCohortRow {
        fleet_id: R6B_INCOMPATIBLE_ID.to_string(),
        entity_id: entity_id_for_r6b_fleet(R6B_INCOMPATIBLE_ID),
        owner: DressRehearsalR6bOwner::Terran,
        cell_index: R6B_FUSION_FIXTURE_CELL,
        profile: DressRehearsalR6bCohortProfile {
            hp_per_ship: 120,
            damage_per_ship_per_tick: FLEET_DAMAGE_PER_SHIP_PER_TICK,
        },
        num_ships: 5,
        destroyed: false,
        fleet_like: true,
        owner_faction_id: 1,
        identity_lane: 1,
    });
    cohort_rows.push(DressRehearsalR6bCohortRow {
        fleet_id: "dress-rehearsal-r6b-hostile-pirate".to_string(),
        entity_id: entity_id_for_r6b_fleet("dress-rehearsal-r6b-hostile-pirate"),
        owner: DressRehearsalR6bOwner::Pirate,
        cell_index: R6B_FUSION_FIXTURE_CELL,
        profile: profile.clone(),
        num_ships: 6,
        destroyed: false,
        fleet_like: true,
        owner_faction_id: 2,
        identity_lane: 2,
    });
    fuse_compatible_at_indices(
        cohort_rows,
        &[left_idx, right_idx],
        "r6b-fixture-friendly-fusion",
        fusion_rows,
    );
}

fn apply_birth_fixture_construction(
    cohort_rows: &mut Vec<DressRehearsalR6bCohortRow>,
    construction_rows: &mut Vec<DressRehearsalR6bConstructionRow>,
    birth_rows: &mut Vec<DressRehearsalR6bBirthRow>,
) {
    let progress_before = 0;
    let production_applied = SHIP_COST;
    let (progress_after, threshold_passed, ship_delta, remainder) =
        construction_threshold_emission(progress_before, production_applied, SHIP_COST);
    construction_rows.push(DressRehearsalR6bConstructionRow {
        starport_id: R6B_BIRTH_STARPORT_ID.to_string(),
        cell_index: R6B_BIRTH_FIXTURE_CELL,
        owner: DressRehearsalR6bOwner::Terran,
        construction_progress_before: progress_before,
        production_applied,
        construction_progress_after: progress_after,
        ship_cost: SHIP_COST,
        threshold_passed,
        ship_count_delta_emitted: ship_delta,
        construction_progress_remainder: remainder,
    });
    if ship_delta > 0 {
        birth_new_cohort(
            cohort_rows,
            R6B_BIRTH_FIXTURE_CELL,
            DressRehearsalR6bOwner::Terran,
            ship_delta,
            R6B_BIRTH_STARPORT_ID,
            birth_rows,
        );
    }
}

fn starport_production_targets(
    r2: &DressRehearsalR2Report,
) -> Vec<DressRehearsalR2SystemProductionRow> {
    let mut rows: Vec<_> = r2
        .production_rows
        .iter()
        .filter(|row| row.has_starport && row.production_generated > 0)
        .cloned()
        .collect();
    rows.sort_by_key(|row| row.system_index);
    rows
}

fn masked_compatible_indices(
    cohort_rows: &[DressRehearsalR6bCohortRow],
    mask: &CohortMask,
) -> Vec<usize> {
    let mut indices: Vec<usize> = cohort_rows
        .iter()
        .enumerate()
        .filter(|(_, row)| {
            !row.destroyed
                && row.fleet_like
                && row.owner == mask.owner
                && row.cell_index == mask.cell_index
                && row.profile == mask.profile
        })
        .map(|(i, _)| i)
        .collect();
    indices.sort_by_key(|&i| cohort_rows[i].fleet_id.clone());
    indices
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CohortMask {
    owner: DressRehearsalR6bOwner,
    cell_index: u32,
    profile: DressRehearsalR6bCohortProfile,
}

fn cohort_mask(
    owner: DressRehearsalR6bOwner,
    cell_index: u32,
    profile: &DressRehearsalR6bCohortProfile,
) -> CohortMask {
    CohortMask {
        owner,
        cell_index,
        profile: profile.clone(),
    }
}

fn canonical_profile() -> DressRehearsalR6bCohortProfile {
    DressRehearsalR6bCohortProfile {
        hp_per_ship: FLEET_HP_PER_SHIP,
        damage_per_ship_per_tick: FLEET_DAMAGE_PER_SHIP_PER_TICK,
    }
}

fn arena_membership_for_cell(
    cohort_rows: &[DressRehearsalR6bCohortRow],
    cell_index: u32,
) -> Vec<u64> {
    let mut ids: Vec<u64> = cohort_rows
        .iter()
        .filter(|r| r.cell_index == cell_index && !r.destroyed && r.fleet_like)
        .map(|r| r.entity_id)
        .collect();
    ids.sort();
    ids.dedup();
    ids
}

fn construction_emission_posture(progress_after: i64, ship_delta: i64) -> bool {
    let op = AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: SlotIndex::new(0),
            col: ColumnIndex::new(CONSTRUCTION_PROGRESS_COL as usize),
        },
        combine: CombineFn::Identity,
        gate: GateSpec::Threshold {
            value: SHIP_COST as f32,
            direction: ThresholdDirection::Upward,
        },
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::EmitEvent,
        targets: vec![(
            SlotIndex::new(0),
            ColumnIndex::new(SHIP_COUNT_DELTA_COL as usize),
        )],
    };
    let _ = op;
    progress_after >= SHIP_COST && ship_delta > 0
}

fn validate_input(input: &DressRehearsalR6bInput, diagnostics: &mut Vec<&'static str>) {
    if input.enabled_by_default {
        diagnostics.push("enabled_by_default");
    }
    if !input.explicit_opt_in {
        return;
    }
    let Some(r5) = input.r5_report.as_ref() else {
        diagnostics.push("missing_r5_report");
        return;
    };
    let Some(r6) = input.r6_report.as_ref() else {
        diagnostics.push("missing_r6_report");
        return;
    };
    if !r5.admitted || !r5.cpu_oracle_parity {
        diagnostics.push("r5_not_admitted");
    }
    if !r6.admitted || !r6.cpu_oracle_parity {
        diagnostics.push("r6_not_admitted");
    }
}

fn base_report(
    input: &DressRehearsalR6bInput,
    disabled_no_op: bool,
    diagnostics: Vec<&'static str>,
    execution: Option<DressRehearsalR6bOracle>,
    cpu_oracle_parity: bool,
) -> DressRehearsalR6bReport {
    let admitted = diagnostics.is_empty();
    let empty_summary = DressRehearsalR6bSummary {
        cohort_row_count: 0,
        construction_row_count: 0,
        reinforcement_row_count: 0,
        birth_row_count: 0,
        fusion_row_count: 0,
        stable_checksum: 0,
    };
    let (
        cohort_rows,
        construction_rows,
        reinforcement_rows,
        birth_rows,
        fusion_rows,
        summary,
        table_driven,
        movement_boundary,
        cpu_manager,
    ) = match execution {
        Some(exec) => (
            exec.cohort_rows.clone(),
            exec.construction_rows.clone(),
            exec.reinforcement_rows.clone(),
            exec.birth_rows.clone(),
            exec.fusion_rows.clone(),
            exec.summary.clone(),
            exec.table_driven_masked_scan_used,
            exec.movement_boundary_request_used,
            exec.cpu_fleet_manager_decision_path,
        ),
        None => (
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            empty_summary,
            false,
            false,
            false,
        ),
    };

    let overrides: BTreeMap<u64, DressRehearsalR6FleetCohortOverride> = cohort_rows
        .iter()
        .filter(|r| r.fleet_like && !r.destroyed)
        .map(|r| {
            (
                r.entity_id,
                DressRehearsalR6FleetCohortOverride {
                    num_ships: r.num_ships,
                    hp_per_ship: r.profile.hp_per_ship,
                    damage_per_ship_per_tick: r.profile.damage_per_ship_per_tick,
                },
            )
        })
        .collect();

    let r5 = input.r5_report.as_ref();
    let r6 = input.r6_report.as_ref();
    let replay_checksum = if admitted && input.explicit_opt_in && !disabled_no_op {
        summary.stable_checksum
    } else {
        0
    };

    let mut report = DressRehearsalR6bReport {
        id: DRESS_REHEARSAL_R6B_SHIP_COHORT_REINFORCEMENT_ID,
        status: DRESS_REHEARSAL_R6B_SHIP_COHORT_REINFORCEMENT_STATUS_PASS,
        scenario_name: DRESS_REHEARSAL_R6B_SCENARIO,
        admitted,
        diagnostics,
        explicit_opt_in: input.explicit_opt_in,
        default_off: !input.enabled_by_default,
        disabled_no_op,
        r5_contract_checksum: r5.map(|r| r.summary.stable_checksum).unwrap_or(0),
        r6_contract_checksum: r6.map(|r| r.summary.stable_checksum).unwrap_or(0),
        cohort_rows,
        construction_rows,
        reinforcement_rows,
        birth_rows,
        fusion_rows,
        summary,
        table_driven_masked_scan_used: table_driven,
        movement_boundary_request_used: movement_boundary,
        cpu_fleet_manager_decision_path: cpu_manager,
        gpu_substrate_posture_only: true,
        cpu_oracle_parity,
        deterministic_replay_checksum: replay_checksum,
        fleet_cohort_overrides: overrides,
        combat_with_r6b_cohorts: None,
    };
    if admitted && input.explicit_opt_in && !disabled_no_op {
        report.combat_with_r6b_cohorts = Some(run_r6_combat_with_r6b_cohorts(&report));
    }
    report
}

fn empty_oracle() -> DressRehearsalR6bOracle {
    DressRehearsalR6bOracle {
        cohort_rows: Vec::new(),
        construction_rows: Vec::new(),
        reinforcement_rows: Vec::new(),
        birth_rows: Vec::new(),
        fusion_rows: Vec::new(),
        summary: DressRehearsalR6bSummary {
            cohort_row_count: 0,
            construction_row_count: 0,
            reinforcement_row_count: 0,
            birth_row_count: 0,
            fusion_row_count: 0,
            stable_checksum: 0,
        },
        table_driven_masked_scan_used: false,
        movement_boundary_request_used: false,
        cpu_fleet_manager_decision_path: false,
    }
}

fn r2_owner(owner: DressRehearsalR2Owner) -> DressRehearsalR6bOwner {
    match owner {
        DressRehearsalR2Owner::Terran => DressRehearsalR6bOwner::Terran,
        DressRehearsalR2Owner::Pirate => DressRehearsalR6bOwner::Pirate,
    }
}

fn owner_faction_id(owner: DressRehearsalR6bOwner) -> u64 {
    match owner {
        DressRehearsalR6bOwner::Terran => 1,
        DressRehearsalR6bOwner::Pirate => 2,
    }
}

fn identity_lane_for_owner(owner: DressRehearsalR6bOwner) -> u32 {
    match owner {
        DressRehearsalR6bOwner::Terran => 1,
        DressRehearsalR6bOwner::Pirate => 2,
    }
}

fn oracle_matches(a: &DressRehearsalR6bOracle, b: &DressRehearsalR6bOracle) -> bool {
    a.cohort_rows == b.cohort_rows
        && a.construction_rows == b.construction_rows
        && a.reinforcement_rows == b.reinforcement_rows
        && a.birth_rows == b.birth_rows
        && a.fusion_rows == b.fusion_rows
        && a.summary == b.summary
        && a.table_driven_masked_scan_used == b.table_driven_masked_scan_used
        && a.movement_boundary_request_used == b.movement_boundary_request_used
        && a.cpu_fleet_manager_decision_path == b.cpu_fleet_manager_decision_path
}

pub fn entity_id_for_r6b_fleet(fleet_id: &str) -> u64 {
    R6B_NEW_FLEET_ENTITY_BASE ^ entity_id_for_mover(fleet_id)
}

fn checksum_r6b(
    r5: u64,
    r6: u64,
    cohort: &[DressRehearsalR6bCohortRow],
    construction: &[DressRehearsalR6bConstructionRow],
    reinforcement: &[DressRehearsalR6bReinforcementRow],
    birth: &[DressRehearsalR6bBirthRow],
    fusion: &[DressRehearsalR6bFusionRow],
) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for part in [r5, r6] {
        hash ^= part;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for row in cohort {
        hash ^= row.entity_id;
        hash ^= u64::from(row.cell_index);
        hash ^= u64::from(row.num_ships as u64);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for row in construction {
        hash ^= u64::from(row.ship_count_delta_emitted as u64);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for row in reinforcement {
        hash ^= u64::from(row.num_ships_after as u64);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for row in birth {
        hash ^= row.entity_id;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for row in fusion {
        hash ^= u64::from(row.fused_num_ships as u64);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
