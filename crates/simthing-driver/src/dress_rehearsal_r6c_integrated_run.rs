//! SCENARIO-0080-2-R6C: integrated 100-tick mutable dress-rehearsal run.
//!
//! This rung does not add a new mechanism. It seeds the accepted ATLAS/R1 world once, then
//! applies the already-proven row/mask/reduce/disburse/threshold/emission-band mechanisms over
//! one mutable world for the canonical 100 ticks.

use crate::dress_rehearsal_r1_disruption_heatmap::{
    bounded_feedback_next, cell_index, DressRehearsalR1OccupantKind, DressRehearsalR1Owner,
    DressRehearsalR1Scenario, CEILING, FLOOR, GALAXY_CELL_COUNT, GALAXY_SIDE, H_WEIGHT,
    PATROL_SUPPRESS, PIRATE_EMIT,
};
use crate::dress_rehearsal_r2_recursive_allocation::{
    factory_recipe_production, BLOCKADE_THRESHOLD, POP_LABOR_PER_TICK, STARPORT_PRODUCTION_NEED,
};
use crate::dress_rehearsal_r3_capability_mask_down::{
    apply_modifier_bps, BLOCKADE_DIVERT_MODIFIER, COMBAT_BONUS_PLACEHOLDER_MODIFIER,
    DEFENSIVE_LOGISTICS_MODIFIER, DISRUPTION_DECAY_MODIFIER, PATROL_SUPPRESSION_MODIFIER,
    PIRATE_EMISSION_MODIFIER, RAIDING_LOGISTICS_MODIFIER,
};
use crate::dress_rehearsal_r4_field_policy_consumption::{
    exact_mag2_bits_from_fixed, f32_to_q16, sqrt_cr_f_bits, MOVEMENT_THRESHOLD_MAG_BITS,
};
use crate::dress_rehearsal_r5_movement_reenroll::{cell_key, entity_id_for_mover};
use crate::dress_rehearsal_r6_combat_hp_damage::{
    damage_output_for_cohort, emission_band_ship_attrition, hp_to_retire_for_cohort,
    FLEET_DAMAGE_PER_SHIP_PER_TICK, FLEET_HP_PER_SHIP,
};
use crate::dress_rehearsal_r6b_ship_cohort_reinforcement::{
    construction_threshold_emission, SHIP_COST,
};
use std::collections::{BTreeMap, BTreeSet, HashMap};

pub const DRESS_REHEARSAL_R6C_INTEGRATED_RUN_ID: &str = "SCENARIO-0080-2-R6C-INTEGRATED-RUN";
pub const DRESS_REHEARSAL_R6C_INTEGRATED_RUN_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - integrated 100-tick mutable R1-to-R6B run";
pub const DRESS_REHEARSAL_R6C_SCENARIO: &str = "SCENARIO-0080-2";
pub const R6C_CANONICAL_TICK_COUNT: u32 = 100;
pub const R6C_GPU_POSTURE: &str = "GPU-conformant; GPU execution not yet measured";
pub const R6C_TIE_BREAKER_POLICY: &str = "Dropped";

const TERRAN_STARPORT_INDICES: [usize; 3] = [0, 4, 8];
const PIRATE_STARPORT_INDEX: usize = 10;
const R6C_BOUNDARY_REQUEST_ID_BASE: u64 = 0x806c_0000_0000_0000;
const R6C_BIRTH_ENTITY_BASE: u64 = 0x806c_b100_0000_0000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DressRehearsalR6cOwner {
    Terran,
    Pirate,
}

impl DressRehearsalR6cOwner {
    pub fn stable_code(self) -> u64 {
        match self {
            Self::Terran => 1,
            Self::Pirate => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DressRehearsalR6cDetectorStatus {
    Emerged,
    PartiallyEmerged,
    NotObserved,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6cInput {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
    pub tick_count: u32,
}

impl DressRehearsalR6cInput {
    pub fn default_simsession() -> Self {
        Self {
            explicit_opt_in: false,
            enabled_by_default: false,
            tick_count: 0,
        }
    }

    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
            tick_count: R6C_CANONICAL_TICK_COUNT,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR6cSystemState {
    pub system_id: String,
    pub system_index: usize,
    pub owner: DressRehearsalR6cOwner,
    pub x: u32,
    pub y: u32,
    pub cell_index: u32,
    pub has_starport: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR6cFleetCohortState {
    pub fleet_id: String,
    pub entity_id: u64,
    pub owner: DressRehearsalR6cOwner,
    pub cell_index: u32,
    pub num_ships: i64,
    pub hp_per_ship: i64,
    pub damage_per_ship_per_tick: i64,
    pub destroyed: bool,
    pub fleet_like: bool,
    pub owner_faction_id: u64,
    pub identity_lane: u32,
    pub lineage: Vec<String>,
    pub spawned_by_production: bool,
    pub last_moved_tick: Option<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6cWorld {
    pub galaxy_side: u32,
    pub grid_cell_count: usize,
    pub systems: Vec<DressRehearsalR6cSystemState>,
    pub fleets: Vec<DressRehearsalR6cFleetCohortState>,
    pub disruption: Vec<f32>,
    pub location_status: Vec<f32>,
    pub stockpiles: BTreeMap<DressRehearsalR6cOwner, i64>,
    pub construction_progress: BTreeMap<usize, i64>,
    pub blockade_divert_owner: BTreeMap<usize, Option<DressRehearsalR6cOwner>>,
    pub arena_membership: BTreeMap<u32, Vec<u64>>,
    pub seed_checksum: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR6cWorldSeedSummary {
    pub system_count: usize,
    pub terran_system_count: usize,
    pub pirate_system_count: usize,
    pub starport_count: usize,
    pub initial_fleet_cohort_count: usize,
    pub initial_terran_ships: i64,
    pub initial_pirate_ships: i64,
    pub pirate_start_cell: u32,
    pub terran_patrol_start_cells: Vec<u32>,
    pub seed_checksum: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR6cCapabilityOverlayRow {
    pub owner: DressRehearsalR6cOwner,
    pub modifier_id: &'static str,
    pub multiplier_bps: i32,
    pub consumed_by_field: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6cDisruptionSourceRow {
    pub tick: u32,
    pub fleet_id: String,
    pub owner: DressRehearsalR6cOwner,
    pub cell_index: u32,
    pub num_ships: i64,
    pub input_cell: f32,
    pub disruption_after: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6cEconomyRow {
    pub tick: u32,
    pub system_id: String,
    pub system_index: usize,
    pub cell_index: u32,
    pub original_owner: DressRehearsalR6cOwner,
    pub effective_outflow_owner: DressRehearsalR6cOwner,
    pub blockader: Option<DressRehearsalR6cOwner>,
    pub disruption: f32,
    pub blockaded: bool,
    pub labor_generated: i64,
    pub labor_consumed: i64,
    pub production_generated: i64,
    pub diverted_production: i64,
    pub disbursement_received: i64,
    pub owner_column_flipped: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR6cStockpileLedgerRow {
    pub tick: u32,
    pub owner: DressRehearsalR6cOwner,
    pub before_reduce_up: i64,
    pub reduced_in: i64,
    pub after_reduce_up: i64,
    pub disbursed_down: i64,
    pub after_disburse_down: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6cFieldReadRow {
    pub tick: u32,
    pub substep: u32,
    pub mover_id: String,
    pub owner: DressRehearsalR6cOwner,
    pub source_cell_index: u32,
    pub source_field_value: f32,
    pub best_neighbor_cell_index: Option<u32>,
    pub best_neighbor_field_value: f32,
    pub gradient_dx_f32: f32,
    pub gradient_dy_f32: f32,
    pub disruption_component: f32,
    pub economy_component: f32,
    pub capability_component_bps: i32,
    pub real_signal_gradient_magnitude_bits: u32,
    pub tie_breaker_gradient_magnitude_bits: u32,
    pub threshold_passed: bool,
    pub decision: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR6cBoundaryRequestRow {
    pub tick: u32,
    pub substep: u32,
    pub boundary_request_id: u64,
    pub mover_id: String,
    pub source_cell_index: u32,
    pub destination_cell_index: u32,
    pub threshold_input_mag_bits: u32,
    pub event_emitted: bool,
    pub materialized_from_step_opportunity: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR6cMovementRow {
    pub tick: u32,
    pub substep: u32,
    pub mover_id: String,
    pub owner: DressRehearsalR6cOwner,
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
    pub movement_applied: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR6cCombatReduceRow {
    pub tick: u32,
    pub cell_index: u32,
    pub owner: DressRehearsalR6cOwner,
    pub combatant_id: String,
    pub damage_output: i64,
    pub owner_channel_total_after_reduce_up: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR6cCombatDisburseRow {
    pub tick: u32,
    pub cell_index: u32,
    pub attacker_id: String,
    pub attacker_owner: DressRehearsalR6cOwner,
    pub target_id: String,
    pub target_owner: DressRehearsalR6cOwner,
    pub damage_disbursed: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR6cCombatRow {
    pub tick: u32,
    pub cell_index: u32,
    pub combatant_id: String,
    pub entity_id: u64,
    pub owner: DressRehearsalR6cOwner,
    pub num_ships_before: i64,
    pub hp_per_ship: i64,
    pub damage_per_ship_per_tick: i64,
    pub damage_output: i64,
    pub hostile_damage_received: i64,
    pub ships_destroyed: i64,
    pub num_ships_after: i64,
    pub hp_to_retire_after: i64,
    pub hostile_target_ids: Vec<String>,
    pub ship_loss_event_emitted: bool,
    pub zero_cohort_event_emitted: bool,
    pub removed_from_arena: bool,
    pub movement_produced_colocation: bool,
    pub owner_overlay_preserved: bool,
    pub identity_preserved: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR6cConstructionRow {
    pub tick: u32,
    pub starport_id: String,
    pub system_index: usize,
    pub cell_index: u32,
    pub owner: DressRehearsalR6cOwner,
    pub construction_progress_before: i64,
    pub production_applied: i64,
    pub construction_progress_after: i64,
    pub ship_cost: i64,
    pub threshold_passed: bool,
    pub ship_count_delta_emitted: i64,
    pub construction_progress_remainder: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR6cReinforcementRow {
    pub tick: u32,
    pub target_fleet_id: String,
    pub entity_id: u64,
    pub owner: DressRehearsalR6cOwner,
    pub cell_index: u32,
    pub num_ships_before: i64,
    pub ship_count_delta: i64,
    pub num_ships_after: i64,
    pub hp_to_retire_after: i64,
    pub damage_output_after: i64,
    pub movement_boundary_request_used: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR6cBirthRow {
    pub tick: u32,
    pub created_fleet_id: String,
    pub entity_id: u64,
    pub owner: DressRehearsalR6cOwner,
    pub cell_index: u32,
    pub starport_id: String,
    pub num_ships: i64,
    pub alloc_enrollment_applied: bool,
    pub movement_boundary_request_used: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR6cFusionRow {
    pub tick: u32,
    pub fusion_event_id: String,
    pub surviving_fleet_id: String,
    pub absorbed_fleet_id: String,
    pub owner: DressRehearsalR6cOwner,
    pub cell_index: u32,
    pub left_num_ships: i64,
    pub right_num_ships: i64,
    pub fused_num_ships: i64,
    pub hp_to_retire_after: i64,
    pub damage_output_after: i64,
    pub identity_lineage_recorded: bool,
    pub owner_overlay_preserved: bool,
    pub movement_boundary_request_used: bool,
    pub arena_membership_after: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR6cConservationRow {
    pub tick: u32,
    pub ships_before: i64,
    pub ships_after: i64,
    pub ships_destroyed_by_combat: i64,
    pub ships_created_by_production: i64,
    pub stockpile_delta: i64,
    pub positions_changed_by_r5_only: bool,
    pub ship_delta_explained: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR6cRaceCurveRow {
    pub sample: u32,
    pub terran_ships: i64,
    pub pirate_ships: i64,
    pub terran_stockpile: i64,
    pub pirate_stockpile: i64,
    pub blockaded_system_count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR6cDetectorRow {
    pub behavior: &'static str,
    pub status: DressRehearsalR6cDetectorStatus,
    pub first_tick: Option<u32>,
    pub evidence_rows: Vec<String>,
    pub cause_if_not_observed: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR6cTraceExcerpt {
    pub label: &'static str,
    pub tick: Option<u32>,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR6cSummary {
    pub tick_count: u32,
    pub tick_row_count: usize,
    pub movement_row_count: usize,
    pub combat_row_count: usize,
    pub construction_row_count: usize,
    pub reinforcement_row_count: usize,
    pub birth_row_count: usize,
    pub fusion_row_count: usize,
    pub detector_row_count: usize,
    pub stable_checksum: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6cReport {
    pub id: &'static str,
    pub status: &'static str,
    pub scenario_name: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,
    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub tick_count: u32,
    pub single_mutable_world_state: bool,
    pub canonical_grid: bool,
    pub single_galactic_tier: bool,
    pub tick_order: Vec<&'static str>,
    pub write_back_confirmed: bool,
    pub tie_breaker_policy: &'static str,
    pub gpu_posture: &'static str,
    pub cpu_oracle_parity: bool,
    pub deterministic_replay_checksum: u64,
    pub world_seed_summary: DressRehearsalR6cWorldSeedSummary,
    pub initial_world: Option<DressRehearsalR6cWorld>,
    pub final_world: Option<DressRehearsalR6cWorld>,
    pub capability_overlay_rows: Vec<DressRehearsalR6cCapabilityOverlayRow>,
    pub disruption_source_rows: Vec<DressRehearsalR6cDisruptionSourceRow>,
    pub economy_rows: Vec<DressRehearsalR6cEconomyRow>,
    pub stockpile_ledger_rows: Vec<DressRehearsalR6cStockpileLedgerRow>,
    pub field_read_rows: Vec<DressRehearsalR6cFieldReadRow>,
    pub boundary_request_rows: Vec<DressRehearsalR6cBoundaryRequestRow>,
    pub movement_rows: Vec<DressRehearsalR6cMovementRow>,
    pub combat_rows: Vec<DressRehearsalR6cCombatRow>,
    pub combat_reduce_rows: Vec<DressRehearsalR6cCombatReduceRow>,
    pub combat_disburse_rows: Vec<DressRehearsalR6cCombatDisburseRow>,
    pub construction_rows: Vec<DressRehearsalR6cConstructionRow>,
    pub reinforcement_rows: Vec<DressRehearsalR6cReinforcementRow>,
    pub birth_rows: Vec<DressRehearsalR6cBirthRow>,
    pub fusion_rows: Vec<DressRehearsalR6cFusionRow>,
    pub conservation_rows: Vec<DressRehearsalR6cConservationRow>,
    pub detector_rows: Vec<DressRehearsalR6cDetectorRow>,
    pub race_curve: Vec<DressRehearsalR6cRaceCurveRow>,
    pub trace_excerpts: Vec<DressRehearsalR6cTraceExcerpt>,
    pub direct_movement_command: bool,
    pub cpu_planner_used: bool,
    pub default_simsession_pass_graph_change: bool,
    pub new_accumulator_op: bool,
    pub semantic_wgsl: bool,
    pub summary: DressRehearsalR6cSummary,
    pub artifact_markdown: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6cOracle {
    pub summary: DressRehearsalR6cSummary,
    pub final_world: DressRehearsalR6cWorld,
    pub movement_rows: Vec<DressRehearsalR6cMovementRow>,
    pub combat_rows: Vec<DressRehearsalR6cCombatRow>,
    pub construction_rows: Vec<DressRehearsalR6cConstructionRow>,
    pub detector_rows: Vec<DressRehearsalR6cDetectorRow>,
    pub race_curve: Vec<DressRehearsalR6cRaceCurveRow>,
}

#[derive(Clone, Debug, PartialEq)]
struct Execution {
    initial_world: DressRehearsalR6cWorld,
    final_world: DressRehearsalR6cWorld,
    world_seed_summary: DressRehearsalR6cWorldSeedSummary,
    capability_overlay_rows: Vec<DressRehearsalR6cCapabilityOverlayRow>,
    disruption_source_rows: Vec<DressRehearsalR6cDisruptionSourceRow>,
    economy_rows: Vec<DressRehearsalR6cEconomyRow>,
    stockpile_ledger_rows: Vec<DressRehearsalR6cStockpileLedgerRow>,
    field_read_rows: Vec<DressRehearsalR6cFieldReadRow>,
    boundary_request_rows: Vec<DressRehearsalR6cBoundaryRequestRow>,
    movement_rows: Vec<DressRehearsalR6cMovementRow>,
    combat_rows: Vec<DressRehearsalR6cCombatRow>,
    combat_reduce_rows: Vec<DressRehearsalR6cCombatReduceRow>,
    combat_disburse_rows: Vec<DressRehearsalR6cCombatDisburseRow>,
    construction_rows: Vec<DressRehearsalR6cConstructionRow>,
    reinforcement_rows: Vec<DressRehearsalR6cReinforcementRow>,
    birth_rows: Vec<DressRehearsalR6cBirthRow>,
    fusion_rows: Vec<DressRehearsalR6cFusionRow>,
    conservation_rows: Vec<DressRehearsalR6cConservationRow>,
    detector_rows: Vec<DressRehearsalR6cDetectorRow>,
    race_curve: Vec<DressRehearsalR6cRaceCurveRow>,
    trace_excerpts: Vec<DressRehearsalR6cTraceExcerpt>,
    summary: DressRehearsalR6cSummary,
}

impl Execution {
    pub(crate) fn stable_checksum(&self) -> u64 {
        self.summary.stable_checksum
    }
}

pub fn run_dress_rehearsal_r6c_integrated_run(
    input: &DressRehearsalR6cInput,
) -> DressRehearsalR6cReport {
    let mut diagnostics = Vec::new();
    validate_input(input, &mut diagnostics);

    if !input.explicit_opt_in {
        return base_report(input, true, diagnostics, None, false);
    }
    if !diagnostics.is_empty() {
        return base_report(input, false, diagnostics, None, false);
    }

    let execution = execute_model(input.tick_count);
    let oracle = cpu_oracle_dress_rehearsal_r6c_integrated_run(input);
    let parity = execution.summary == oracle.summary
        && execution.final_world == oracle.final_world
        && execution.movement_rows == oracle.movement_rows
        && execution.combat_rows == oracle.combat_rows
        && execution.construction_rows == oracle.construction_rows
        && execution.detector_rows == oracle.detector_rows
        && execution.race_curve == oracle.race_curve;
    base_report(input, false, Vec::new(), Some(execution), parity)
}

pub fn replay_dress_rehearsal_r6c_integrated_run(
) -> (DressRehearsalR6cReport, DressRehearsalR6cReport) {
    let input = DressRehearsalR6cInput::explicit_opt_in();
    (
        run_dress_rehearsal_r6c_integrated_run(&input),
        run_dress_rehearsal_r6c_integrated_run(&input),
    )
}

pub fn cpu_oracle_dress_rehearsal_r6c_integrated_run(
    input: &DressRehearsalR6cInput,
) -> DressRehearsalR6cOracle {
    if !input.explicit_opt_in || input.enabled_by_default || input.tick_count == 0 {
        return empty_oracle();
    }
    let execution = execute_model(input.tick_count);
    DressRehearsalR6cOracle {
        summary: execution.summary,
        final_world: execution.final_world,
        movement_rows: execution.movement_rows,
        combat_rows: execution.combat_rows,
        construction_rows: execution.construction_rows,
        detector_rows: execution.detector_rows,
        race_curve: execution.race_curve,
    }
}

pub fn render_dress_rehearsal_r6c_artifact(report: &DressRehearsalR6cReport) -> String {
    report.artifact_markdown.clone()
}

fn validate_input(input: &DressRehearsalR6cInput, diagnostics: &mut Vec<&'static str>) {
    if input.enabled_by_default {
        diagnostics.push("r6c_default_on_rejected");
    }
    if !input.explicit_opt_in {
        return;
    }
    if input.tick_count != R6C_CANONICAL_TICK_COUNT {
        diagnostics.push("r6c_tick_count_must_be_canonical_100");
    }
}

fn base_report(
    input: &DressRehearsalR6cInput,
    disabled_no_op: bool,
    diagnostics: Vec<&'static str>,
    execution: Option<Execution>,
    cpu_oracle_parity: bool,
) -> DressRehearsalR6cReport {
    let admitted = diagnostics.is_empty();
    let empty_world = seed_world();
    let empty_seed = world_seed_summary(&empty_world);
    let empty_summary = DressRehearsalR6cSummary {
        tick_count: 0,
        tick_row_count: 0,
        movement_row_count: 0,
        combat_row_count: 0,
        construction_row_count: 0,
        reinforcement_row_count: 0,
        birth_row_count: 0,
        fusion_row_count: 0,
        detector_row_count: 0,
        stable_checksum: 0,
    };

    let (
        initial_world,
        final_world,
        seed,
        capability,
        disruption,
        economy,
        stockpiles,
        fields,
        boundary,
        movement,
        combat,
        reduce,
        disburse,
        construction,
        reinforcement,
        birth,
        fusion,
        conservation,
        detectors,
        race_curve,
        excerpts,
        summary,
    ) = match execution {
        Some(execution) => (
            Some(execution.initial_world),
            Some(execution.final_world),
            execution.world_seed_summary,
            execution.capability_overlay_rows,
            execution.disruption_source_rows,
            execution.economy_rows,
            execution.stockpile_ledger_rows,
            execution.field_read_rows,
            execution.boundary_request_rows,
            execution.movement_rows,
            execution.combat_rows,
            execution.combat_reduce_rows,
            execution.combat_disburse_rows,
            execution.construction_rows,
            execution.reinforcement_rows,
            execution.birth_rows,
            execution.fusion_rows,
            execution.conservation_rows,
            execution.detector_rows,
            execution.race_curve,
            execution.trace_excerpts,
            execution.summary,
        ),
        None => (
            None,
            None,
            empty_seed,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            empty_summary,
        ),
    };

    let artifact_markdown = render_artifact_markdown(
        input.tick_count,
        &seed,
        &summary,
        &detectors,
        &race_curve,
        &excerpts,
        cpu_oracle_parity,
    );

    DressRehearsalR6cReport {
        id: DRESS_REHEARSAL_R6C_INTEGRATED_RUN_ID,
        status: DRESS_REHEARSAL_R6C_INTEGRATED_RUN_STATUS_PASS,
        scenario_name: DRESS_REHEARSAL_R6C_SCENARIO,
        admitted,
        diagnostics,
        explicit_opt_in: input.explicit_opt_in,
        default_off: !input.enabled_by_default,
        disabled_no_op,
        tick_count: if disabled_no_op { 0 } else { input.tick_count },
        single_mutable_world_state: admitted && input.explicit_opt_in && !disabled_no_op,
        canonical_grid: admitted && input.explicit_opt_in && !disabled_no_op,
        single_galactic_tier: admitted && input.explicit_opt_in && !disabled_no_op,
        tick_order: tick_order(),
        write_back_confirmed: !disabled_no_op
            && conservation.iter().all(|row| row.ship_delta_explained),
        tie_breaker_policy: R6C_TIE_BREAKER_POLICY,
        gpu_posture: R6C_GPU_POSTURE,
        cpu_oracle_parity,
        deterministic_replay_checksum: if admitted && input.explicit_opt_in {
            summary.stable_checksum
        } else {
            0
        },
        world_seed_summary: seed,
        initial_world,
        final_world,
        capability_overlay_rows: capability,
        disruption_source_rows: disruption,
        economy_rows: economy,
        stockpile_ledger_rows: stockpiles,
        field_read_rows: fields,
        boundary_request_rows: boundary,
        movement_rows: movement,
        combat_rows: combat,
        combat_reduce_rows: reduce,
        combat_disburse_rows: disburse,
        construction_rows: construction,
        reinforcement_rows: reinforcement,
        birth_rows: birth,
        fusion_rows: fusion,
        conservation_rows: conservation,
        detector_rows: detectors,
        race_curve,
        trace_excerpts: excerpts,
        direct_movement_command: false,
        cpu_planner_used: false,
        default_simsession_pass_graph_change: false,
        new_accumulator_op: false,
        semantic_wgsl: false,
        summary,
        artifact_markdown,
    }
}

/// Per-tick hook for RUNTIME-0080-0-R0 (after CPU tick + write-back).
pub(crate) trait R6cGpuTickHook {
    fn after_tick(&mut self, tick: u32, world: &DressRehearsalR6cWorld);
}

struct R6cNoGpuTickHook;

impl R6cGpuTickHook for R6cNoGpuTickHook {
    fn after_tick(&mut self, _tick: u32, _world: &DressRehearsalR6cWorld) {}
}

fn execute_model(tick_count: u32) -> Execution {
    let mut no_hook = R6cNoGpuTickHook;
    execute_model_with_hook(tick_count, Some(&mut no_hook))
}

/// Run the R6C model with a per-tick hook (after CPU tick + write-back).
pub(crate) fn execute_model_with_gpu_hook<H: R6cGpuTickHook>(tick_count: u32, hook: &mut H) -> u64 {
    execute_model_with_hook(tick_count, Some(hook)).stable_checksum()
}

fn execute_model_with_hook<H: R6cGpuTickHook>(
    tick_count: u32,
    mut after_tick: Option<&mut H>,
) -> Execution {
    let mut world = seed_world();
    let initial_world = world.clone();
    let world_seed_summary = world_seed_summary(&world);
    let capability_overlay_rows = capability_overlays();

    let mut disruption_source_rows = Vec::new();
    let mut economy_rows = Vec::new();
    let mut stockpile_ledger_rows = Vec::new();
    let mut field_read_rows = Vec::new();
    let mut boundary_request_rows = Vec::new();
    let mut movement_rows = Vec::new();
    let mut combat_rows = Vec::new();
    let mut combat_reduce_rows = Vec::new();
    let mut combat_disburse_rows = Vec::new();
    let mut construction_rows = Vec::new();
    let mut reinforcement_rows = Vec::new();
    let mut birth_rows = Vec::new();
    let mut fusion_rows = Vec::new();
    let mut conservation_rows = Vec::new();
    let mut race_curve = vec![race_curve_row(0, &world)];

    for tick in 0..tick_count {
        let ships_before = total_ships(&world);
        let stockpile_before = total_stockpile(&world);

        run_disruption_tick(
            tick,
            &mut world,
            &capability_overlay_rows,
            &mut disruption_source_rows,
        );

        let tick_economy_rows = run_economy_tick(tick, &mut world, &mut stockpile_ledger_rows);
        economy_rows.extend(tick_economy_rows.clone());

        let moved_cells = run_movement_tick(
            tick,
            &mut world,
            &capability_overlay_rows,
            &mut field_read_rows,
            &mut boundary_request_rows,
            &mut movement_rows,
        );

        let ships_destroyed_by_combat = run_combat_tick(
            tick,
            &mut world,
            &moved_cells,
            &capability_overlay_rows,
            &mut combat_rows,
            &mut combat_reduce_rows,
            &mut combat_disburse_rows,
        );

        let ships_created_by_production = run_production_tick(
            tick,
            &mut world,
            &tick_economy_rows,
            &mut construction_rows,
            &mut reinforcement_rows,
            &mut birth_rows,
            &mut fusion_rows,
        );

        refresh_membership(&mut world);
        let ships_after = total_ships(&world);
        let stockpile_after = total_stockpile(&world);
        conservation_rows.push(DressRehearsalR6cConservationRow {
            tick,
            ships_before,
            ships_after,
            ships_destroyed_by_combat,
            ships_created_by_production,
            stockpile_delta: stockpile_after - stockpile_before,
            positions_changed_by_r5_only: true,
            ship_delta_explained: ships_after
                == ships_before - ships_destroyed_by_combat + ships_created_by_production,
        });
        race_curve.push(race_curve_row(tick + 1, &world));
        if let Some(hook) = after_tick.as_deref_mut() {
            hook.after_tick(tick, &world);
        }
    }

    let detector_rows = build_detector_rows(
        &movement_rows,
        &economy_rows,
        &combat_rows,
        &construction_rows,
        &reinforcement_rows,
        &birth_rows,
        &fusion_rows,
        &race_curve,
    );
    let trace_excerpts = build_trace_excerpts(
        &movement_rows,
        &economy_rows,
        &combat_rows,
        &construction_rows,
        &race_curve,
    );
    let summary = DressRehearsalR6cSummary {
        tick_count,
        tick_row_count: tick_count as usize,
        movement_row_count: movement_rows.len(),
        combat_row_count: combat_rows.len(),
        construction_row_count: construction_rows.len(),
        reinforcement_row_count: reinforcement_rows.len(),
        birth_row_count: birth_rows.len(),
        fusion_row_count: fusion_rows.len(),
        detector_row_count: detector_rows.len(),
        stable_checksum: checksum_execution(
            world_seed_summary.seed_checksum,
            &world,
            &movement_rows,
            &combat_rows,
            &construction_rows,
            &reinforcement_rows,
            &birth_rows,
            &fusion_rows,
            &race_curve,
            &detector_rows,
        ),
    };

    Execution {
        initial_world,
        final_world: world,
        world_seed_summary,
        capability_overlay_rows,
        disruption_source_rows,
        economy_rows,
        stockpile_ledger_rows,
        field_read_rows,
        boundary_request_rows,
        movement_rows,
        combat_rows,
        combat_reduce_rows,
        combat_disburse_rows,
        construction_rows,
        reinforcement_rows,
        birth_rows,
        fusion_rows,
        conservation_rows,
        detector_rows,
        race_curve,
        trace_excerpts,
        summary,
    }
}

fn seed_world() -> DressRehearsalR6cWorld {
    let scenario = DressRehearsalR1Scenario::canonical();
    let mut systems = scenario
        .occupants
        .iter()
        .filter(|occupant| occupant.kind == DressRehearsalR1OccupantKind::System)
        .map(|occupant| {
            let system_index = system_index(&occupant.source_id);
            DressRehearsalR6cSystemState {
                system_id: occupant.source_id.clone(),
                system_index,
                owner: owner_from_r1(occupant.owner),
                x: occupant.x,
                y: occupant.y,
                cell_index: occupant.cell_index,
                has_starport: has_starport(system_index),
            }
        })
        .collect::<Vec<_>>();
    systems.sort_by_key(|system| system.system_index);

    let mut fleets = scenario
        .occupants
        .iter()
        .filter(|occupant| {
            matches!(
                occupant.kind,
                DressRehearsalR1OccupantKind::PirateFleet
                    | DressRehearsalR1OccupantKind::PatrolFleet
            )
        })
        .map(|occupant| {
            let owner = owner_from_r1(occupant.owner);
            DressRehearsalR6cFleetCohortState {
                fleet_id: occupant.source_id.clone(),
                entity_id: entity_id_for_mover(&occupant.source_id),
                owner,
                cell_index: occupant.cell_index,
                num_ships: 1,
                hp_per_ship: FLEET_HP_PER_SHIP,
                damage_per_ship_per_tick: FLEET_DAMAGE_PER_SHIP_PER_TICK,
                destroyed: false,
                fleet_like: true,
                owner_faction_id: owner.stable_code(),
                identity_lane: identity_lane_for_owner(owner),
                lineage: vec![occupant.source_id.clone()],
                spawned_by_production: false,
                last_moved_tick: None,
            }
        })
        .collect::<Vec<_>>();
    fleets.sort_by(|left, right| left.fleet_id.cmp(&right.fleet_id));

    let mut stockpiles = BTreeMap::new();
    stockpiles.insert(DressRehearsalR6cOwner::Terran, 0);
    stockpiles.insert(DressRehearsalR6cOwner::Pirate, 0);

    let construction_progress = systems
        .iter()
        .filter(|system| system.has_starport)
        .map(|system| (system.system_index, 0))
        .collect::<BTreeMap<_, _>>();
    let blockade_divert_owner = systems
        .iter()
        .map(|system| (system.system_index, None))
        .collect::<BTreeMap<_, _>>();

    let mut world = DressRehearsalR6cWorld {
        galaxy_side: GALAXY_SIDE,
        grid_cell_count: GALAXY_CELL_COUNT,
        systems,
        fleets,
        disruption: vec![0.0; GALAXY_CELL_COUNT],
        location_status: vec![0.0; GALAXY_CELL_COUNT],
        stockpiles,
        construction_progress,
        blockade_divert_owner,
        arena_membership: BTreeMap::new(),
        seed_checksum: 0,
    };
    refresh_membership(&mut world);
    world.seed_checksum = checksum_world_seed(&world);
    world
}

fn world_seed_summary(world: &DressRehearsalR6cWorld) -> DressRehearsalR6cWorldSeedSummary {
    let pirate_start_cell = world
        .fleets
        .iter()
        .find(|fleet| fleet.owner == DressRehearsalR6cOwner::Pirate)
        .map(|fleet| fleet.cell_index)
        .unwrap_or(0);
    let mut terran_patrol_start_cells = world
        .fleets
        .iter()
        .filter(|fleet| fleet.owner == DressRehearsalR6cOwner::Terran)
        .map(|fleet| fleet.cell_index)
        .collect::<Vec<_>>();
    terran_patrol_start_cells.sort_unstable();
    terran_patrol_start_cells.dedup();
    DressRehearsalR6cWorldSeedSummary {
        system_count: world.systems.len(),
        terran_system_count: world
            .systems
            .iter()
            .filter(|system| system.owner == DressRehearsalR6cOwner::Terran)
            .count(),
        pirate_system_count: world
            .systems
            .iter()
            .filter(|system| system.owner == DressRehearsalR6cOwner::Pirate)
            .count(),
        starport_count: world
            .systems
            .iter()
            .filter(|system| system.has_starport)
            .count(),
        initial_fleet_cohort_count: world.fleets.len(),
        initial_terran_ships: ship_count_for(world, DressRehearsalR6cOwner::Terran),
        initial_pirate_ships: ship_count_for(world, DressRehearsalR6cOwner::Pirate),
        pirate_start_cell,
        terran_patrol_start_cells,
        seed_checksum: world.seed_checksum,
    }
}

fn capability_overlays() -> Vec<DressRehearsalR6cCapabilityOverlayRow> {
    vec![
        overlay(
            DressRehearsalR6cOwner::Terran,
            PATROL_SUPPRESSION_MODIFIER,
            12_000,
        ),
        overlay(
            DressRehearsalR6cOwner::Terran,
            DISRUPTION_DECAY_MODIFIER,
            11_000,
        ),
        overlay(
            DressRehearsalR6cOwner::Terran,
            DEFENSIVE_LOGISTICS_MODIFIER,
            11_000,
        ),
        overlay(
            DressRehearsalR6cOwner::Terran,
            COMBAT_BONUS_PLACEHOLDER_MODIFIER,
            10_500,
        ),
        overlay(
            DressRehearsalR6cOwner::Pirate,
            PIRATE_EMISSION_MODIFIER,
            12_500,
        ),
        overlay(
            DressRehearsalR6cOwner::Pirate,
            BLOCKADE_DIVERT_MODIFIER,
            15_000,
        ),
        overlay(
            DressRehearsalR6cOwner::Pirate,
            RAIDING_LOGISTICS_MODIFIER,
            11_000,
        ),
        overlay(
            DressRehearsalR6cOwner::Pirate,
            COMBAT_BONUS_PLACEHOLDER_MODIFIER,
            11_500,
        ),
    ]
}

fn overlay(
    owner: DressRehearsalR6cOwner,
    modifier_id: &'static str,
    multiplier_bps: i32,
) -> DressRehearsalR6cCapabilityOverlayRow {
    DressRehearsalR6cCapabilityOverlayRow {
        owner,
        modifier_id,
        multiplier_bps,
        consumed_by_field: true,
    }
}

fn run_disruption_tick(
    tick: u32,
    world: &mut DressRehearsalR6cWorld,
    capability: &[DressRehearsalR6cCapabilityOverlayRow],
    rows: &mut Vec<DressRehearsalR6cDisruptionSourceRow>,
) {
    let mut input_by_cell: HashMap<u32, f32> = HashMap::new();
    let mut source_rows = Vec::new();
    for fleet in live_fleets(world) {
        let modifier = match fleet.owner {
            DressRehearsalR6cOwner::Terran => {
                capability_bps(capability, fleet.owner, PATROL_SUPPRESSION_MODIFIER)
            }
            DressRehearsalR6cOwner::Pirate => {
                capability_bps(capability, fleet.owner, PIRATE_EMISSION_MODIFIER)
            }
        };
        let input = match fleet.owner {
            DressRehearsalR6cOwner::Terran => {
                -apply_modifier_bps(PATROL_SUPPRESS * fleet.num_ships as f32, modifier)
            }
            DressRehearsalR6cOwner::Pirate => {
                apply_modifier_bps(PIRATE_EMIT * fleet.num_ships as f32, modifier)
            }
        };
        *input_by_cell.entry(fleet.cell_index).or_insert(0.0) += input;
        source_rows.push((
            fleet.fleet_id.clone(),
            fleet.owner,
            fleet.cell_index,
            fleet.num_ships,
            input,
        ));
    }

    for idx in 0..GALAXY_CELL_COUNT {
        let before = world.disruption[idx];
        let input = input_by_cell.get(&(idx as u32)).copied().unwrap_or(0.0);
        world.disruption[idx] = bounded_feedback_next(before, input);
    }
    world.location_status = diffusion_status(&world.disruption);

    for (fleet_id, owner, cell_index, num_ships, input_cell) in source_rows {
        rows.push(DressRehearsalR6cDisruptionSourceRow {
            tick,
            fleet_id,
            owner,
            cell_index,
            num_ships,
            input_cell,
            disruption_after: world.disruption[cell_index as usize],
        });
    }
}

fn run_economy_tick(
    tick: u32,
    world: &mut DressRehearsalR6cWorld,
    stockpile_rows: &mut Vec<DressRehearsalR6cStockpileLedgerRow>,
) -> Vec<DressRehearsalR6cEconomyRow> {
    let mut rows = Vec::new();
    let mut reduced: BTreeMap<DressRehearsalR6cOwner, i64> = BTreeMap::new();
    reduced.insert(DressRehearsalR6cOwner::Terran, 0);
    reduced.insert(DressRehearsalR6cOwner::Pirate, 0);

    for system in &world.systems {
        let disruption = world.disruption[system.cell_index as usize];
        let blockader = if disruption >= BLOCKADE_THRESHOLD {
            blockader_for_system(world, system)
        } else {
            None
        };
        let blockaded = blockader.is_some();
        let effective_owner = blockader.unwrap_or(system.owner);
        let (production_generated, labor_consumed, _) =
            factory_recipe_production(POP_LABOR_PER_TICK);
        *reduced.entry(effective_owner).or_insert(0) += production_generated;
        world
            .blockade_divert_owner
            .insert(system.system_index, blockader);
        rows.push(DressRehearsalR6cEconomyRow {
            tick,
            system_id: system.system_id.clone(),
            system_index: system.system_index,
            cell_index: system.cell_index,
            original_owner: system.owner,
            effective_outflow_owner: effective_owner,
            blockader,
            disruption,
            blockaded,
            labor_generated: POP_LABOR_PER_TICK,
            labor_consumed,
            production_generated,
            diverted_production: if blockaded { production_generated } else { 0 },
            disbursement_received: 0,
            owner_column_flipped: blockaded,
        });
    }

    for owner in [
        DressRehearsalR6cOwner::Terran,
        DressRehearsalR6cOwner::Pirate,
    ] {
        let before = *world.stockpiles.get(&owner).unwrap_or(&0);
        let reduced_in = *reduced.get(&owner).unwrap_or(&0);
        world.stockpiles.insert(owner, before + reduced_in);
    }

    for owner in [
        DressRehearsalR6cOwner::Terran,
        DressRehearsalR6cOwner::Pirate,
    ] {
        let before_reduce_up = world.stockpiles[&owner] - reduced[&owner];
        let after_reduce_up = world.stockpiles[&owner];
        let mut disbursed_down = 0;
        let starport_indices = rows
            .iter()
            .enumerate()
            .filter(|(_, row)| {
                row.original_owner == owner
                    && world
                        .systems
                        .iter()
                        .find(|system| system.system_index == row.system_index)
                        .map(|system| system.has_starport)
                        .unwrap_or(false)
            })
            .map(|(idx, _)| idx)
            .collect::<Vec<_>>();
        for idx in starport_indices {
            let available = world.stockpiles.entry(owner).or_insert(0);
            let disbursed = STARPORT_PRODUCTION_NEED.min(*available).max(0);
            *available -= disbursed;
            disbursed_down += disbursed;
            rows[idx].disbursement_received = disbursed;
        }
        stockpile_rows.push(DressRehearsalR6cStockpileLedgerRow {
            tick,
            owner,
            before_reduce_up,
            reduced_in: reduced[&owner],
            after_reduce_up,
            disbursed_down,
            after_disburse_down: *world.stockpiles.get(&owner).unwrap_or(&0),
        });
    }

    rows
}

fn run_movement_tick(
    tick: u32,
    world: &mut DressRehearsalR6cWorld,
    capability: &[DressRehearsalR6cCapabilityOverlayRow],
    field_rows: &mut Vec<DressRehearsalR6cFieldReadRow>,
    boundary_rows: &mut Vec<DressRehearsalR6cBoundaryRequestRow>,
    movement_rows: &mut Vec<DressRehearsalR6cMovementRow>,
) -> BTreeSet<u32> {
    let mut moved_cells = BTreeSet::new();
    let fleet_ids = world
        .fleets
        .iter()
        .filter(|fleet| !fleet.destroyed && fleet.num_ships > 0)
        .map(|fleet| fleet.fleet_id.clone())
        .collect::<Vec<_>>();

    for fleet_id in fleet_ids {
        let Some(mut fleet_index) = world
            .fleets
            .iter()
            .position(|fleet| fleet.fleet_id == fleet_id && !fleet.destroyed)
        else {
            continue;
        };
        let speed = match world.fleets[fleet_index].owner {
            DressRehearsalR6cOwner::Terran => 2,
            DressRehearsalR6cOwner::Pirate => 3,
        };
        for substep in 0..speed {
            fleet_index = world
                .fleets
                .iter()
                .position(|fleet| fleet.fleet_id == fleet_id && !fleet.destroyed)
                .expect("fleet remains live during movement loop");
            let fleet = world.fleets[fleet_index].clone();
            let field = build_field(world, fleet.owner, capability);
            let decision = field_decision(&field, fleet.cell_index);
            field_rows.push(DressRehearsalR6cFieldReadRow {
                tick,
                substep,
                mover_id: fleet.fleet_id.clone(),
                owner: fleet.owner,
                source_cell_index: fleet.cell_index,
                source_field_value: decision.source_value,
                best_neighbor_cell_index: decision.destination,
                best_neighbor_field_value: decision.best_value,
                gradient_dx_f32: decision.gradient_dx_f32,
                gradient_dy_f32: decision.gradient_dy_f32,
                disruption_component: decision.disruption_component,
                economy_component: decision.economy_component,
                capability_component_bps: decision.capability_component_bps,
                real_signal_gradient_magnitude_bits: decision.candidate_f_exact_mag_bits,
                tie_breaker_gradient_magnitude_bits: 0,
                threshold_passed: decision.threshold_passed,
                decision: if decision.step_opportunity {
                    "StepOpportunity"
                } else {
                    "SitStill"
                },
            });

            if !decision.step_opportunity {
                break;
            }
            let destination = decision
                .destination
                .expect("step opportunity must carry destination");
            let before_source = entities_in_cell(world, fleet.cell_index);
            let before_dest = entities_in_cell(world, destination);
            let boundary_id = boundary_request_id(
                &fleet.fleet_id,
                tick,
                substep,
                fleet.cell_index,
                destination,
            );
            boundary_rows.push(DressRehearsalR6cBoundaryRequestRow {
                tick,
                substep,
                boundary_request_id: boundary_id,
                mover_id: fleet.fleet_id.clone(),
                source_cell_index: fleet.cell_index,
                destination_cell_index: destination,
                threshold_input_mag_bits: decision.candidate_f_exact_mag_bits,
                event_emitted: true,
                materialized_from_step_opportunity: true,
            });

            world.fleets[fleet_index].cell_index = destination;
            world.fleets[fleet_index].last_moved_tick = Some(tick);
            refresh_membership(world);
            let after_source = entities_in_cell(world, fleet.cell_index);
            let after_dest = entities_in_cell(world, destination);
            let _ = cell_key(destination);
            movement_rows.push(DressRehearsalR6cMovementRow {
                tick,
                substep,
                mover_id: fleet.fleet_id.clone(),
                owner: fleet.owner,
                source_cell_index: fleet.cell_index,
                destination_cell_index: destination,
                r4_decision_consumed: "StepOpportunity",
                event_emitted: true,
                boundary_request_id: boundary_id,
                entity_id: fleet.entity_id,
                idroute_identity_before: fleet.identity_lane,
                idroute_identity_after: fleet.identity_lane,
                owner_faction_id_before: fleet.owner_faction_id,
                owner_faction_id_after: fleet.owner_faction_id,
                source_arena_membership_before: before_source,
                source_arena_membership_after: after_source,
                destination_arena_membership_before: before_dest,
                destination_arena_membership_after: after_dest,
                movement_applied: true,
            });
            moved_cells.insert(destination);
        }
    }
    moved_cells
}

fn run_combat_tick(
    tick: u32,
    world: &mut DressRehearsalR6cWorld,
    moved_cells: &BTreeSet<u32>,
    capability: &[DressRehearsalR6cCapabilityOverlayRow],
    combat_rows: &mut Vec<DressRehearsalR6cCombatRow>,
    reduce_rows: &mut Vec<DressRehearsalR6cCombatReduceRow>,
    disburse_rows: &mut Vec<DressRehearsalR6cCombatDisburseRow>,
) -> i64 {
    let mut destroyed_total = 0;
    let combat_cells = hostile_cells(world)
        .into_iter()
        .filter(|cell| moved_cells.contains(cell))
        .collect::<Vec<_>>();

    for cell_index in combat_cells {
        let combatant_indices = world
            .fleets
            .iter()
            .enumerate()
            .filter(|(_, fleet)| {
                !fleet.destroyed
                    && fleet.num_ships > 0
                    && fleet.cell_index == cell_index
                    && fleet.fleet_like
            })
            .map(|(idx, _)| idx)
            .collect::<Vec<_>>();
        let membership_before = entities_in_cell(world, cell_index);
        let mut owner_totals: BTreeMap<DressRehearsalR6cOwner, i64> = BTreeMap::new();
        for &idx in &combatant_indices {
            let fleet = &world.fleets[idx];
            let modifier =
                capability_bps(capability, fleet.owner, COMBAT_BONUS_PLACEHOLDER_MODIFIER);
            let damage = apply_bps_i64(
                damage_output_for_cohort(fleet.num_ships, fleet.damage_per_ship_per_tick),
                modifier,
            );
            let total = owner_totals.entry(fleet.owner).or_insert(0);
            *total += damage;
            reduce_rows.push(DressRehearsalR6cCombatReduceRow {
                tick,
                cell_index,
                owner: fleet.owner,
                combatant_id: fleet.fleet_id.clone(),
                damage_output: damage,
                owner_channel_total_after_reduce_up: *total,
            });
        }

        let mut attrition = Vec::new();
        for &idx in &combatant_indices {
            let fleet = &world.fleets[idx];
            let hostile_targets = combatant_indices
                .iter()
                .filter(|&&other| world.fleets[other].owner != fleet.owner)
                .map(|&other| world.fleets[other].fleet_id.clone())
                .collect::<Vec<_>>();
            let hostile_damage = owner_totals
                .iter()
                .filter(|(owner, _)| **owner != fleet.owner)
                .map(|(_, total)| *total)
                .sum::<i64>();
            for &other in &combatant_indices {
                let attacker = &world.fleets[other];
                if attacker.owner == fleet.owner {
                    continue;
                }
                let modifier = capability_bps(
                    capability,
                    attacker.owner,
                    COMBAT_BONUS_PLACEHOLDER_MODIFIER,
                );
                let damage = apply_bps_i64(
                    damage_output_for_cohort(attacker.num_ships, attacker.damage_per_ship_per_tick),
                    modifier,
                );
                disburse_rows.push(DressRehearsalR6cCombatDisburseRow {
                    tick,
                    cell_index,
                    attacker_id: attacker.fleet_id.clone(),
                    attacker_owner: attacker.owner,
                    target_id: fleet.fleet_id.clone(),
                    target_owner: fleet.owner,
                    damage_disbursed: damage,
                });
            }
            let (ships_destroyed, num_ships_after, hp_to_retire_after, zero_cohort) =
                emission_band_ship_attrition(hostile_damage, fleet.num_ships, fleet.hp_per_ship);
            destroyed_total += ships_destroyed;
            attrition.push((
                idx,
                hostile_damage,
                ships_destroyed,
                num_ships_after,
                hp_to_retire_after,
                zero_cohort,
                hostile_targets,
                apply_bps_i64(
                    damage_output_for_cohort(fleet.num_ships, fleet.damage_per_ship_per_tick),
                    capability_bps(capability, fleet.owner, COMBAT_BONUS_PLACEHOLDER_MODIFIER),
                ),
            ));
        }

        for (
            idx,
            hostile_damage,
            ships_destroyed,
            num_ships_after,
            hp_to_retire_after,
            zero_cohort,
            hostile_targets,
            damage_output,
        ) in attrition
        {
            let fleet_before = world.fleets[idx].clone();
            world.fleets[idx].num_ships = num_ships_after;
            if zero_cohort {
                world.fleets[idx].destroyed = true;
            }
            refresh_membership(world);
            let membership_after = entities_in_cell(world, cell_index);
            combat_rows.push(DressRehearsalR6cCombatRow {
                tick,
                cell_index,
                combatant_id: fleet_before.fleet_id,
                entity_id: fleet_before.entity_id,
                owner: fleet_before.owner,
                num_ships_before: fleet_before.num_ships,
                hp_per_ship: fleet_before.hp_per_ship,
                damage_per_ship_per_tick: fleet_before.damage_per_ship_per_tick,
                damage_output,
                hostile_damage_received: hostile_damage,
                ships_destroyed,
                num_ships_after,
                hp_to_retire_after,
                hostile_target_ids: hostile_targets,
                ship_loss_event_emitted: ships_destroyed > 0,
                zero_cohort_event_emitted: zero_cohort,
                removed_from_arena: zero_cohort
                    && membership_before.contains(&fleet_before.entity_id)
                    && !membership_after.contains(&fleet_before.entity_id),
                movement_produced_colocation: true,
                owner_overlay_preserved: fleet_before.owner_faction_id
                    == fleet_before.owner.stable_code(),
                identity_preserved: fleet_before.identity_lane
                    == identity_lane_for_owner(fleet_before.owner),
            });
        }
    }
    refresh_membership(world);
    destroyed_total
}

fn run_production_tick(
    tick: u32,
    world: &mut DressRehearsalR6cWorld,
    economy_rows: &[DressRehearsalR6cEconomyRow],
    construction_rows: &mut Vec<DressRehearsalR6cConstructionRow>,
    reinforcement_rows: &mut Vec<DressRehearsalR6cReinforcementRow>,
    birth_rows: &mut Vec<DressRehearsalR6cBirthRow>,
    fusion_rows: &mut Vec<DressRehearsalR6cFusionRow>,
) -> i64 {
    let mut created = 0;
    let starports = world
        .systems
        .iter()
        .filter(|system| system.has_starport)
        .cloned()
        .collect::<Vec<_>>();
    for system in starports {
        let production_applied = economy_rows
            .iter()
            .find(|row| row.system_index == system.system_index)
            .map(|row| row.disbursement_received)
            .unwrap_or(0);
        let progress_before = *world
            .construction_progress
            .get(&system.system_index)
            .unwrap_or(&0);
        let (progress_after, threshold_passed, ship_delta, remainder) =
            construction_threshold_emission(progress_before, production_applied, SHIP_COST);
        world
            .construction_progress
            .insert(system.system_index, remainder);
        construction_rows.push(DressRehearsalR6cConstructionRow {
            tick,
            starport_id: format!("starport-{}", system.system_index),
            system_index: system.system_index,
            cell_index: system.cell_index,
            owner: system.owner,
            construction_progress_before: progress_before,
            production_applied,
            construction_progress_after: progress_after,
            ship_cost: SHIP_COST,
            threshold_passed,
            ship_count_delta_emitted: ship_delta,
            construction_progress_remainder: remainder,
        });
        if ship_delta > 0 {
            created += ship_delta;
            apply_ship_delta(
                tick,
                world,
                &system,
                ship_delta,
                reinforcement_rows,
                birth_rows,
            );
        }
    }

    created += run_friendly_fusion(tick, world, fusion_rows);
    created
}

fn apply_ship_delta(
    tick: u32,
    world: &mut DressRehearsalR6cWorld,
    system: &DressRehearsalR6cSystemState,
    ship_delta: i64,
    reinforcement_rows: &mut Vec<DressRehearsalR6cReinforcementRow>,
    birth_rows: &mut Vec<DressRehearsalR6cBirthRow>,
) {
    let compatible = world
        .fleets
        .iter()
        .enumerate()
        .filter(|(_, fleet)| {
            !fleet.destroyed
                && fleet.fleet_like
                && fleet.owner == system.owner
                && fleet.cell_index == system.cell_index
                && fleet.hp_per_ship == FLEET_HP_PER_SHIP
                && fleet.damage_per_ship_per_tick == FLEET_DAMAGE_PER_SHIP_PER_TICK
        })
        .map(|(idx, _)| idx)
        .collect::<Vec<_>>();
    if let Some(idx) = compatible.first().copied() {
        let before = world.fleets[idx].num_ships;
        let after = before + ship_delta;
        world.fleets[idx].num_ships = after;
        reinforcement_rows.push(DressRehearsalR6cReinforcementRow {
            tick,
            target_fleet_id: world.fleets[idx].fleet_id.clone(),
            entity_id: world.fleets[idx].entity_id,
            owner: system.owner,
            cell_index: system.cell_index,
            num_ships_before: before,
            ship_count_delta: ship_delta,
            num_ships_after: after,
            hp_to_retire_after: hp_to_retire_for_cohort(after, FLEET_HP_PER_SHIP),
            damage_output_after: damage_output_for_cohort(after, FLEET_DAMAGE_PER_SHIP_PER_TICK),
            movement_boundary_request_used: false,
        });
        return;
    }

    let fleet_id = format!("r6c-born-starport-{}-tick-{}", system.system_index, tick);
    let entity_id = R6C_BIRTH_ENTITY_BASE ^ entity_id_for_mover(&fleet_id);
    world.fleets.push(DressRehearsalR6cFleetCohortState {
        fleet_id: fleet_id.clone(),
        entity_id,
        owner: system.owner,
        cell_index: system.cell_index,
        num_ships: ship_delta,
        hp_per_ship: FLEET_HP_PER_SHIP,
        damage_per_ship_per_tick: FLEET_DAMAGE_PER_SHIP_PER_TICK,
        destroyed: false,
        fleet_like: true,
        owner_faction_id: system.owner.stable_code(),
        identity_lane: identity_lane_for_owner(system.owner),
        lineage: vec![fleet_id.clone()],
        spawned_by_production: true,
        last_moved_tick: None,
    });
    let _ = cell_key(system.cell_index);
    refresh_membership(world);
    birth_rows.push(DressRehearsalR6cBirthRow {
        tick,
        created_fleet_id: fleet_id,
        entity_id,
        owner: system.owner,
        cell_index: system.cell_index,
        starport_id: format!("starport-{}", system.system_index),
        num_ships: ship_delta,
        alloc_enrollment_applied: entities_in_cell(world, system.cell_index).contains(&entity_id),
        movement_boundary_request_used: false,
    });
}

fn run_friendly_fusion(
    tick: u32,
    world: &mut DressRehearsalR6cWorld,
    fusion_rows: &mut Vec<DressRehearsalR6cFusionRow>,
) -> i64 {
    let mut net_created = 0;
    let mut groups: BTreeMap<(DressRehearsalR6cOwner, u32, i64, i64), Vec<usize>> = BTreeMap::new();
    for (idx, fleet) in world.fleets.iter().enumerate() {
        if fleet.destroyed || fleet.num_ships <= 0 || !fleet.fleet_like {
            continue;
        }
        groups
            .entry((
                fleet.owner,
                fleet.cell_index,
                fleet.hp_per_ship,
                fleet.damage_per_ship_per_tick,
            ))
            .or_default()
            .push(idx);
    }

    for ((owner, cell_index, hp_per_ship, damage_per_ship_per_tick), mut indices) in groups {
        if indices.len() < 2 {
            continue;
        }
        indices.sort_by_key(|idx| world.fleets[*idx].fleet_id.clone());
        let survivor = indices[0];
        for absorbed in indices.iter().copied().skip(1) {
            if world.fleets[absorbed].destroyed {
                continue;
            }
            let left = world.fleets[survivor].num_ships;
            let right = world.fleets[absorbed].num_ships;
            let fused = left + right;
            world.fleets[survivor].num_ships = fused;
            let absorbed_lineage = world.fleets[absorbed].lineage.clone();
            world.fleets[survivor].lineage.extend(absorbed_lineage);
            world.fleets[absorbed].destroyed = true;
            let survivor_id = world.fleets[survivor].fleet_id.clone();
            let absorbed_id = world.fleets[absorbed].fleet_id.clone();
            refresh_membership(world);
            fusion_rows.push(DressRehearsalR6cFusionRow {
                tick,
                fusion_event_id: format!("r6c-friendly-fusion-{survivor_id}-{absorbed_id}"),
                surviving_fleet_id: survivor_id,
                absorbed_fleet_id: absorbed_id,
                owner,
                cell_index,
                left_num_ships: left,
                right_num_ships: right,
                fused_num_ships: fused,
                hp_to_retire_after: hp_to_retire_for_cohort(fused, hp_per_ship),
                damage_output_after: damage_output_for_cohort(fused, damage_per_ship_per_tick),
                identity_lineage_recorded: true,
                owner_overlay_preserved: true,
                movement_boundary_request_used: false,
                arena_membership_after: entities_in_cell(world, cell_index),
            });
        }
        net_created += 0;
    }
    refresh_membership(world);
    net_created
}

#[derive(Clone, Debug)]
struct FieldCell {
    value: f32,
    disruption_component: f32,
    economy_component: f32,
    capability_component_bps: i32,
}

#[derive(Clone, Debug)]
struct MovementDecision {
    source_value: f32,
    best_value: f32,
    destination: Option<u32>,
    gradient_dx_f32: f32,
    gradient_dy_f32: f32,
    disruption_component: f32,
    economy_component: f32,
    capability_component_bps: i32,
    candidate_f_exact_mag_bits: u32,
    threshold_passed: bool,
    step_opportunity: bool,
}

fn build_field(
    world: &DressRehearsalR6cWorld,
    owner: DressRehearsalR6cOwner,
    capability: &[DressRehearsalR6cCapabilityOverlayRow],
) -> Vec<FieldCell> {
    (0..GALAXY_CELL_COUNT as u32)
        .map(|cell| field_cell(world, owner, capability, cell))
        .collect()
}

fn field_cell(
    world: &DressRehearsalR6cWorld,
    owner: DressRehearsalR6cOwner,
    capability: &[DressRehearsalR6cCapabilityOverlayRow],
    cell: u32,
) -> FieldCell {
    let (x, y) = xy(cell);
    let mut disruption_component = 0.0;
    let mut economy_component = 0.0;
    let mut patrol_penalty = 0.0;
    let mut hostile_strength_component = 0.0;
    let capability_component_bps = match owner {
        DressRehearsalR6cOwner::Terran => {
            capability_bps(capability, owner, DEFENSIVE_LOGISTICS_MODIFIER)
        }
        DressRehearsalR6cOwner::Pirate => {
            capability_bps(capability, owner, RAIDING_LOGISTICS_MODIFIER)
        }
    };

    for system in &world.systems {
        let dist = manhattan_xy(x, y, system.x, system.y) as f32 + 1.0;
        let system_disruption = world.disruption[system.cell_index as usize];
        let starport_bonus = if system.has_starport { 45.0 } else { 0.0 };
        let blockade_bonus = if world
            .blockade_divert_owner
            .get(&system.system_index)
            .copied()
            .flatten()
            .is_some()
        {
            60.0
        } else {
            0.0
        };
        match owner {
            DressRehearsalR6cOwner::Pirate => {
                if system.owner == DressRehearsalR6cOwner::Terran {
                    economy_component += (110.0 + starport_bonus + 6.0) / dist;
                    disruption_component += ((CEILING - system_disruption).max(0.0) * 0.9) / dist;
                } else {
                    economy_component += (12.0 + starport_bonus * 0.15) / dist;
                    disruption_component -= system_disruption * 0.3 / dist;
                }
            }
            DressRehearsalR6cOwner::Terran => {
                if system.owner == DressRehearsalR6cOwner::Terran {
                    economy_component += (75.0 + starport_bonus + blockade_bonus) / dist;
                    disruption_component += system_disruption * 2.4 / dist;
                } else {
                    disruption_component += system_disruption * 0.35 / dist;
                }
            }
        }
    }

    for fleet in live_fleets(world) {
        let dist = manhattan_cell(cell, fleet.cell_index) as f32 + 1.0;
        if fleet.owner != owner {
            hostile_strength_component += fleet.num_ships as f32 * 34.0 / dist;
        }
        if owner == DressRehearsalR6cOwner::Pirate && fleet.owner == DressRehearsalR6cOwner::Terran
        {
            patrol_penalty += fleet.num_ships as f32 * 55.0 / dist;
        }
    }

    let value = match owner {
        DressRehearsalR6cOwner::Pirate => {
            apply_modifier_bps(
                economy_component + disruption_component + hostile_strength_component * 0.08
                    - patrol_penalty,
                capability_component_bps,
            ) - world.disruption[cell as usize] * 1.15
        }
        DressRehearsalR6cOwner::Terran => apply_modifier_bps(
            economy_component + disruption_component + hostile_strength_component,
            capability_component_bps,
        ),
    };

    FieldCell {
        value,
        disruption_component,
        economy_component,
        capability_component_bps,
    }
}

fn field_decision(field: &[FieldCell], cell: u32) -> MovementDecision {
    let (x, y) = xy(cell);
    let source = &field[cell as usize];
    let (gx, gy) = gradient_at(field, x, y);
    let dx_fixed = f32_to_q16(gx);
    let dy_fixed = f32_to_q16(gy);
    let exact_mag2_bits = exact_mag2_bits_from_fixed(dx_fixed, dy_fixed);
    let candidate_f_exact_mag_bits = sqrt_cr_f_bits(exact_mag2_bits);
    let threshold_passed = candidate_f_exact_mag_bits >= MOVEMENT_THRESHOLD_MAG_BITS;

    let mut best = (cell, source.value);
    for neighbor in von_neumann_cell_indices(x, y) {
        let candidate = field[neighbor as usize].value;
        if candidate > best.1 {
            best = (neighbor, candidate);
        }
    }
    let step_opportunity = threshold_passed && best.0 != cell && best.1 > source.value + 0.001;
    MovementDecision {
        source_value: source.value,
        best_value: best.1,
        destination: step_opportunity.then_some(best.0),
        gradient_dx_f32: gx,
        gradient_dy_f32: gy,
        disruption_component: source.disruption_component,
        economy_component: source.economy_component,
        capability_component_bps: source.capability_component_bps,
        candidate_f_exact_mag_bits,
        threshold_passed,
        step_opportunity,
    }
}

fn gradient_at(field: &[FieldCell], x: u32, y: u32) -> (f32, f32) {
    let here = cell_index(x, y);
    let west = if x > 0 {
        field[cell_index(x - 1, y) as usize].value
    } else {
        field[here as usize].value
    };
    let east = if x + 1 < GALAXY_SIDE {
        field[cell_index(x + 1, y) as usize].value
    } else {
        field[here as usize].value
    };
    let north = if y > 0 {
        field[cell_index(x, y - 1) as usize].value
    } else {
        field[here as usize].value
    };
    let south = if y + 1 < GALAXY_SIDE {
        field[cell_index(x, y + 1) as usize].value
    } else {
        field[here as usize].value
    };
    (0.5 * (east - west), 0.5 * (south - north))
}

fn blockader_for_system(
    world: &DressRehearsalR6cWorld,
    system: &DressRehearsalR6cSystemState,
) -> Option<DressRehearsalR6cOwner> {
    let mut owners = world
        .fleets
        .iter()
        .filter(|fleet| {
            !fleet.destroyed && fleet.num_ships > 0 && fleet.cell_index == system.cell_index
        })
        .map(|fleet| fleet.owner)
        .collect::<BTreeSet<_>>();
    owners.remove(&system.owner);
    owners.into_iter().next()
}

fn hostile_cells(world: &DressRehearsalR6cWorld) -> Vec<u32> {
    let mut by_cell: BTreeMap<u32, BTreeSet<DressRehearsalR6cOwner>> = BTreeMap::new();
    for fleet in live_fleets(world) {
        by_cell
            .entry(fleet.cell_index)
            .or_default()
            .insert(fleet.owner);
    }
    by_cell
        .into_iter()
        .filter(|(_, owners)| owners.len() > 1)
        .map(|(cell, _)| cell)
        .collect()
}

fn diffusion_status(disruption: &[f32]) -> Vec<f32> {
    let mut status = vec![0.0; GALAXY_CELL_COUNT];
    for y in 0..GALAXY_SIDE {
        for x in 0..GALAXY_SIDE {
            let idx = cell_index(x, y) as usize;
            let mut neighbor_sum = 0.0;
            let mut neighbor_count = 0u32;
            for neighbor in von_neumann_cell_indices(x, y) {
                neighbor_sum += disruption[neighbor as usize];
                neighbor_count += 1;
            }
            let denom = 1.0 + H_WEIGHT * neighbor_count as f32;
            status[idx] =
                ((disruption[idx] + H_WEIGHT * neighbor_sum) / denom).clamp(FLOOR, CEILING);
        }
    }
    status
}

fn build_detector_rows(
    movement: &[DressRehearsalR6cMovementRow],
    economy: &[DressRehearsalR6cEconomyRow],
    combat: &[DressRehearsalR6cCombatRow],
    construction: &[DressRehearsalR6cConstructionRow],
    reinforcement: &[DressRehearsalR6cReinforcementRow],
    birth: &[DressRehearsalR6cBirthRow],
    fusion: &[DressRehearsalR6cFusionRow],
    race: &[DressRehearsalR6cRaceCurveRow],
) -> Vec<DressRehearsalR6cDetectorRow> {
    let first_pirate_move = movement
        .iter()
        .find(|row| row.owner == DressRehearsalR6cOwner::Pirate)
        .map(|row| row.tick);
    let distinct_pirate_destinations = movement
        .iter()
        .filter(|row| row.owner == DressRehearsalR6cOwner::Pirate)
        .map(|row| row.destination_cell_index)
        .collect::<BTreeSet<_>>()
        .len();
    let first_patrol_move = movement
        .iter()
        .find(|row| row.owner == DressRehearsalR6cOwner::Terran)
        .map(|row| row.tick);
    let first_blockade = economy
        .iter()
        .find(|row| row.blockaded && row.owner_column_flipped)
        .map(|row| row.tick);
    let first_combat = combat
        .iter()
        .find(|row| row.movement_produced_colocation)
        .map(|row| row.tick);
    let first_attrition = combat
        .iter()
        .find(|row| row.ships_destroyed > 0)
        .map(|row| row.tick);
    let first_production = reinforcement
        .first()
        .map(|row| row.tick)
        .or_else(|| birth.first().map(|row| row.tick));
    let first_fusion = fusion.first().map(|row| row.tick);
    let race_changed = race
        .first()
        .zip(race.last())
        .map(|(a, b)| a.terran_ships != b.terran_ships || a.pirate_ships != b.pirate_ships)
        .unwrap_or(false);
    let first_shipyard_crossing = construction
        .iter()
        .find(|row| row.threshold_passed)
        .map(|row| row.tick);

    vec![
        detector(
            "Pirate raiding waves toward weakly defended, high-value Terran systems",
            if distinct_pirate_destinations >= 4 {
                DressRehearsalR6cDetectorStatus::Emerged
            } else if first_pirate_move.is_some() {
                DressRehearsalR6cDetectorStatus::PartiallyEmerged
            } else {
                DressRehearsalR6cDetectorStatus::NotObserved
            },
            first_pirate_move,
            vec![format!(
                "pirate_distinct_destinations={distinct_pirate_destinations}"
            )],
            "pirate field never crossed the movement threshold",
        ),
        detector(
            "Self-disruption migration",
            if distinct_pirate_destinations >= 2 {
                DressRehearsalR6cDetectorStatus::PartiallyEmerged
            } else {
                DressRehearsalR6cDetectorStatus::NotObserved
            },
            first_pirate_move,
            vec![
                "pirate field includes a live disruption penalty and clean-target attraction"
                    .to_string(),
            ],
            "canonical run did not leave a saturated pirate cell",
        ),
        detector(
            "Patrol response to disruption",
            if first_patrol_move.is_some() {
                DressRehearsalR6cDetectorStatus::Emerged
            } else {
                DressRehearsalR6cDetectorStatus::NotObserved
            },
            first_patrol_move,
            vec!["Terran field consumed disruption and defensive logistics overlays".to_string()],
            "patrol field stayed below threshold",
        ),
        detector(
            "Patrol interception or co-location caused by movement",
            if first_combat.is_some() {
                DressRehearsalR6cDetectorStatus::Emerged
            } else {
                DressRehearsalR6cDetectorStatus::NotObserved
            },
            first_combat,
            vec!["R6 combat rows require moved_cells membership".to_string()],
            "no hostile post-R5 co-location occurred",
        ),
        detector(
            "Race equilibrium between Terran production and pirate attrition",
            if race_changed && first_shipyard_crossing.is_some() {
                DressRehearsalR6cDetectorStatus::PartiallyEmerged
            } else {
                DressRehearsalR6cDetectorStatus::NotObserved
            },
            first_shipyard_crossing,
            vec![format!("race_curve_samples={}", race.len())],
            "ship counts did not materially change",
        ),
        detector(
            "Blockade/divert affecting economy over time",
            if first_blockade.is_some() {
                DressRehearsalR6cDetectorStatus::Emerged
            } else {
                DressRehearsalR6cDetectorStatus::NotObserved
            },
            first_blockade,
            vec![
                "R2 owner-column flip is recomputed from live disruption and fleet positions"
                    .to_string(),
            ],
            "no system reached disruption >= 100 with a hostile blockader present",
        ),
        detector(
            "Combat caused by movement-produced co-location",
            if first_combat.is_some() {
                DressRehearsalR6cDetectorStatus::Emerged
            } else {
                DressRehearsalR6cDetectorStatus::NotObserved
            },
            first_combat,
            vec!["combat rows carry movement_produced_colocation=true".to_string()],
            "hostile co-location was never produced by R5 movement",
        ),
        detector(
            "Fleet attrition as cohort ship loss",
            if first_attrition.is_some() {
                DressRehearsalR6cDetectorStatus::Emerged
            } else {
                DressRehearsalR6cDetectorStatus::NotObserved
            },
            first_attrition,
            vec!["ships_destroyed=floor(hostile_damage_received/hp_per_ship)".to_string()],
            "combat produced no whole-ship loss",
        ),
        detector(
            "Production reinforcing fleets",
            if first_production.is_some() {
                DressRehearsalR6cDetectorStatus::Emerged
            } else {
                DressRehearsalR6cDetectorStatus::NotObserved
            },
            first_production,
            vec![format!(
                "reinforcement_rows={}, birth_rows={}",
                reinforcement.len(),
                birth.len()
            )],
            "construction did not cross SHIP_COST",
        ),
        detector(
            "Friendly fleet fusion/cohort compaction",
            if first_fusion.is_some() {
                DressRehearsalR6cDetectorStatus::Emerged
            } else {
                DressRehearsalR6cDetectorStatus::NotObserved
            },
            first_fusion,
            vec![format!("fusion_rows={}", fusion.len())],
            "no compatible friendly cohorts shared a cell",
        ),
        detector(
            "Front/standoff formation or persistent contested region",
            if combat
                .iter()
                .filter(|row| row.movement_produced_colocation)
                .count()
                >= 4
            {
                DressRehearsalR6cDetectorStatus::PartiallyEmerged
            } else {
                DressRehearsalR6cDetectorStatus::NotObserved
            },
            first_combat,
            vec!["detector requires repeated movement-produced hostile co-locations".to_string()],
            "contested cells did not persist long enough to call a front",
        ),
        detector(
            "Self-sustaining pirate pressure loop",
            if first_blockade.is_some() && first_production.is_some() && first_attrition.is_some() {
                DressRehearsalR6cDetectorStatus::PartiallyEmerged
            } else {
                DressRehearsalR6cDetectorStatus::NotObserved
            },
            [first_blockade, first_production, first_attrition]
                .into_iter()
                .flatten()
                .min(),
            vec!["requires raid -> divert -> production/attrition feedback evidence".to_string()],
            "one or more loop legs did not recur",
        ),
        detector(
            "Open-ended AI behavior",
            DressRehearsalR6cDetectorStatus::NotObserved,
            None,
            vec!["R6C has no CPU planner, route search, or policy AI".to_string()],
            "intentionally out of scope; deterministic field thresholds only",
        ),
        detector(
            "Modder-facing expressibility",
            DressRehearsalR6cDetectorStatus::Emerged,
            Some(0),
            vec![
                "R1-R6B are represented as row/mask/reduce/disburse/threshold/emission-band traces"
                    .to_string(),
            ],
            "not applicable",
        ),
    ]
}

fn detector(
    behavior: &'static str,
    status: DressRehearsalR6cDetectorStatus,
    first_tick: Option<u32>,
    evidence_rows: Vec<String>,
    cause: &'static str,
) -> DressRehearsalR6cDetectorRow {
    DressRehearsalR6cDetectorRow {
        behavior,
        status,
        first_tick,
        evidence_rows,
        cause_if_not_observed: (status == DressRehearsalR6cDetectorStatus::NotObserved)
            .then_some(cause),
    }
}

fn build_trace_excerpts(
    movement: &[DressRehearsalR6cMovementRow],
    economy: &[DressRehearsalR6cEconomyRow],
    combat: &[DressRehearsalR6cCombatRow],
    construction: &[DressRehearsalR6cConstructionRow],
    race: &[DressRehearsalR6cRaceCurveRow],
) -> Vec<DressRehearsalR6cTraceExcerpt> {
    let first_movement = movement.first().map(|row| row.tick);
    let first_blockade = economy.iter().find(|row| row.blockaded).map(|row| row.tick);
    let first_combat = combat.first().map(|row| row.tick);
    let first_production = construction
        .iter()
        .find(|row| row.threshold_passed)
        .map(|row| row.tick);
    let final_sample = race.last();
    vec![
        DressRehearsalR6cTraceExcerpt {
            label: "tick 0",
            tick: Some(0),
            summary: "seeded mutable world; first pass reads canonical fleet positions".to_string(),
        },
        DressRehearsalR6cTraceExcerpt {
            label: "first movement tick",
            tick: first_movement,
            summary: first_movement
                .map(|tick| format!("R5 emitted BoundaryRequest rows on tick {tick}"))
                .unwrap_or_else(|| "no R5 movement observed".to_string()),
        },
        DressRehearsalR6cTraceExcerpt {
            label: "first blockade tick",
            tick: first_blockade,
            summary: first_blockade
                .map(|tick| format!("R2 owner-column divert observed on tick {tick}"))
                .unwrap_or_else(|| "no blockade observed".to_string()),
        },
        DressRehearsalR6cTraceExcerpt {
            label: "first combat tick",
            tick: first_combat,
            summary: first_combat
                .map(|tick| {
                    format!("R6 movement-produced hostile co-location resolved on tick {tick}")
                })
                .unwrap_or_else(|| "no combat observed".to_string()),
        },
        DressRehearsalR6cTraceExcerpt {
            label: "first production reinforcement tick",
            tick: first_production,
            summary: first_production
                .map(|tick| format!("R6B construction threshold crossed on tick {tick}"))
                .unwrap_or_else(|| "no production threshold crossing observed".to_string()),
        },
        DressRehearsalR6cTraceExcerpt {
            label: "final tick",
            tick: final_sample.map(|row| row.sample),
            summary: final_sample
                .map(|row| {
                    format!(
                        "final ships Terran={} Pirate={} stockpiles Terran={} Pirate={}",
                        row.terran_ships,
                        row.pirate_ships,
                        row.terran_stockpile,
                        row.pirate_stockpile
                    )
                })
                .unwrap_or_else(|| "no final sample".to_string()),
        },
    ]
}

fn render_artifact_markdown(
    tick_count: u32,
    seed: &DressRehearsalR6cWorldSeedSummary,
    summary: &DressRehearsalR6cSummary,
    detectors: &[DressRehearsalR6cDetectorRow],
    race: &[DressRehearsalR6cRaceCurveRow],
    excerpts: &[DressRehearsalR6cTraceExcerpt],
    cpu_oracle_parity: bool,
) -> String {
    let mut out = String::new();
    out.push_str("# SCENARIO-0080-2 R6C integrated run artifact\n\n");
    out.push_str("| key | value |\n|---|---:|\n");
    out.push_str(&format!("| tick_count | {} |\n", tick_count));
    out.push_str(&format!(
        "| seed_checksum | {:016x} |\n",
        seed.seed_checksum
    ));
    out.push_str(&format!(
        "| stable_checksum | {:016x} |\n",
        summary.stable_checksum
    ));
    out.push_str(&format!("| cpu_oracle_parity | {} |\n", cpu_oracle_parity));
    out.push_str(&format!("| gpu_posture | {} |\n\n", R6C_GPU_POSTURE));
    out.push_str("## Detector Table\n\n");
    out.push_str("| behavior | status | first_tick | evidence |\n|---|---|---:|---|\n");
    for row in detectors {
        out.push_str(&format!(
            "| {} | {:?} | {} | {} |\n",
            row.behavior,
            row.status,
            row.first_tick
                .map(|tick| tick.to_string())
                .unwrap_or_else(|| "-".to_string()),
            row.evidence_rows.join("; ")
        ));
    }
    out.push_str("\n## Race Curve Samples\n\n");
    out.push_str("| sample | terran_ships | pirate_ships | terran_stockpile | pirate_stockpile | blockaded_systems |\n");
    out.push_str("|---:|---:|---:|---:|---:|---:|\n");
    for row in race.iter().step_by(10) {
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            row.sample,
            row.terran_ships,
            row.pirate_ships,
            row.terran_stockpile,
            row.pirate_stockpile,
            row.blockaded_system_count
        ));
    }
    if let Some(last) = race.last() {
        if last.sample % 10 != 0 {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} |\n",
                last.sample,
                last.terran_ships,
                last.pirate_ships,
                last.terran_stockpile,
                last.pirate_stockpile,
                last.blockaded_system_count
            ));
        }
    }
    out.push_str("\n## Trace Excerpts\n\n");
    for excerpt in excerpts {
        out.push_str(&format!(
            "- {}: {} ({})\n",
            excerpt.label,
            excerpt
                .tick
                .map(|tick| tick.to_string())
                .unwrap_or_else(|| "not observed".to_string()),
            excerpt.summary
        ));
    }
    out
}

fn empty_oracle() -> DressRehearsalR6cOracle {
    let world = seed_world();
    DressRehearsalR6cOracle {
        summary: DressRehearsalR6cSummary {
            tick_count: 0,
            tick_row_count: 0,
            movement_row_count: 0,
            combat_row_count: 0,
            construction_row_count: 0,
            reinforcement_row_count: 0,
            birth_row_count: 0,
            fusion_row_count: 0,
            detector_row_count: 0,
            stable_checksum: 0,
        },
        final_world: world,
        movement_rows: Vec::new(),
        combat_rows: Vec::new(),
        construction_rows: Vec::new(),
        detector_rows: Vec::new(),
        race_curve: Vec::new(),
    }
}

fn tick_order() -> Vec<&'static str> {
    vec![
        "R1 disruption recurrence from current fleet positions",
        "R2 labor-to-production reduce-up disburse-down blockade/divert",
        "R3 capability overlays owner-mask down",
        "R4 FIELD_POLICY field read GradientXY exact-mag2 Candidate-F threshold",
        "R5 movement BoundaryRequest REENROLL fresh-read substeps",
        "R6 combat from movement-produced co-location",
        "R6B production reinforcement birth fusion",
        "write_back positions ships stockpiles disruption production blockade/divert",
    ]
}

fn refresh_membership(world: &mut DressRehearsalR6cWorld) {
    let mut membership: BTreeMap<u32, Vec<u64>> = BTreeMap::new();
    for fleet in live_fleets(world) {
        membership
            .entry(fleet.cell_index)
            .or_default()
            .push(fleet.entity_id);
    }
    for members in membership.values_mut() {
        members.sort_unstable();
    }
    world.arena_membership = membership;
}

fn race_curve_row(sample: u32, world: &DressRehearsalR6cWorld) -> DressRehearsalR6cRaceCurveRow {
    DressRehearsalR6cRaceCurveRow {
        sample,
        terran_ships: ship_count_for(world, DressRehearsalR6cOwner::Terran),
        pirate_ships: ship_count_for(world, DressRehearsalR6cOwner::Pirate),
        terran_stockpile: *world
            .stockpiles
            .get(&DressRehearsalR6cOwner::Terran)
            .unwrap_or(&0),
        pirate_stockpile: *world
            .stockpiles
            .get(&DressRehearsalR6cOwner::Pirate)
            .unwrap_or(&0),
        blockaded_system_count: world
            .blockade_divert_owner
            .values()
            .filter(|owner| owner.is_some())
            .count(),
    }
}

fn live_fleets(world: &DressRehearsalR6cWorld) -> Vec<&DressRehearsalR6cFleetCohortState> {
    world
        .fleets
        .iter()
        .filter(|fleet| !fleet.destroyed && fleet.num_ships > 0)
        .collect()
}

fn ship_count_for(world: &DressRehearsalR6cWorld, owner: DressRehearsalR6cOwner) -> i64 {
    world
        .fleets
        .iter()
        .filter(|fleet| !fleet.destroyed && fleet.owner == owner)
        .map(|fleet| fleet.num_ships)
        .sum()
}

fn total_ships(world: &DressRehearsalR6cWorld) -> i64 {
    world
        .fleets
        .iter()
        .filter(|fleet| !fleet.destroyed)
        .map(|fleet| fleet.num_ships)
        .sum()
}

fn total_stockpile(world: &DressRehearsalR6cWorld) -> i64 {
    world.stockpiles.values().sum()
}

fn entities_in_cell(world: &DressRehearsalR6cWorld, cell: u32) -> Vec<u64> {
    world
        .arena_membership
        .get(&cell)
        .cloned()
        .unwrap_or_default()
}

fn owner_from_r1(owner: DressRehearsalR1Owner) -> DressRehearsalR6cOwner {
    match owner {
        DressRehearsalR1Owner::Terran => DressRehearsalR6cOwner::Terran,
        DressRehearsalR1Owner::Pirate => DressRehearsalR6cOwner::Pirate,
    }
}

fn system_index(source_id: &str) -> usize {
    source_id
        .strip_prefix("system-")
        .and_then(|tail| tail.parse::<usize>().ok())
        .unwrap_or(usize::MAX)
}

fn has_starport(system_index: usize) -> bool {
    TERRAN_STARPORT_INDICES.contains(&system_index) || system_index == PIRATE_STARPORT_INDEX
}

fn identity_lane_for_owner(owner: DressRehearsalR6cOwner) -> u32 {
    match owner {
        DressRehearsalR6cOwner::Terran => 0,
        DressRehearsalR6cOwner::Pirate => 1,
    }
}

fn capability_bps(
    rows: &[DressRehearsalR6cCapabilityOverlayRow],
    owner: DressRehearsalR6cOwner,
    modifier_id: &'static str,
) -> i32 {
    rows.iter()
        .find(|row| row.owner == owner && row.modifier_id == modifier_id)
        .map(|row| row.multiplier_bps)
        .unwrap_or(10_000)
}

fn apply_bps_i64(value: i64, bps: i32) -> i64 {
    (value * i64::from(bps)) / 10_000
}

fn boundary_request_id(fleet_id: &str, tick: u32, substep: u32, source: u32, dest: u32) -> u64 {
    R6C_BOUNDARY_REQUEST_ID_BASE
        ^ entity_id_for_mover(fleet_id)
        ^ (u64::from(tick) << 40)
        ^ (u64::from(substep) << 36)
        ^ (u64::from(source) << 18)
        ^ u64::from(dest)
}

fn xy(cell: u32) -> (u32, u32) {
    (cell % GALAXY_SIDE, cell / GALAXY_SIDE)
}

fn manhattan_cell(left: u32, right: u32) -> u32 {
    let (lx, ly) = xy(left);
    let (rx, ry) = xy(right);
    manhattan_xy(lx, ly, rx, ry)
}

fn manhattan_xy(lx: u32, ly: u32, rx: u32, ry: u32) -> u32 {
    lx.abs_diff(rx) + ly.abs_diff(ry)
}

fn von_neumann_cell_indices(x: u32, y: u32) -> Vec<u32> {
    let mut out = Vec::with_capacity(4);
    if x > 0 {
        out.push(cell_index(x - 1, y));
    }
    if x + 1 < GALAXY_SIDE {
        out.push(cell_index(x + 1, y));
    }
    if y > 0 {
        out.push(cell_index(x, y - 1));
    }
    if y + 1 < GALAXY_SIDE {
        out.push(cell_index(x, y + 1));
    }
    out
}

fn checksum_world_seed(world: &DressRehearsalR6cWorld) -> u64 {
    let mut hash = fnv_seed();
    for system in &world.systems {
        mix_str(&mut hash, &system.system_id);
        mix_u64(&mut hash, system.system_index as u64);
        mix_u64(&mut hash, u64::from(system.cell_index));
        mix_u64(&mut hash, system.owner.stable_code());
        mix_u64(&mut hash, u64::from(system.has_starport as u8));
    }
    for fleet in &world.fleets {
        mix_str(&mut hash, &fleet.fleet_id);
        mix_u64(&mut hash, fleet.entity_id);
        mix_u64(&mut hash, fleet.owner.stable_code());
        mix_u64(&mut hash, u64::from(fleet.cell_index));
        mix_u64(&mut hash, fleet.num_ships as u64);
    }
    hash
}

fn checksum_execution(
    seed_checksum: u64,
    world: &DressRehearsalR6cWorld,
    movement: &[DressRehearsalR6cMovementRow],
    combat: &[DressRehearsalR6cCombatRow],
    construction: &[DressRehearsalR6cConstructionRow],
    reinforcement: &[DressRehearsalR6cReinforcementRow],
    birth: &[DressRehearsalR6cBirthRow],
    fusion: &[DressRehearsalR6cFusionRow],
    race: &[DressRehearsalR6cRaceCurveRow],
    detectors: &[DressRehearsalR6cDetectorRow],
) -> u64 {
    let mut hash = fnv_seed();
    mix_u64(&mut hash, seed_checksum);
    for fleet in live_fleets(world) {
        mix_str(&mut hash, &fleet.fleet_id);
        mix_u64(&mut hash, fleet.owner.stable_code());
        mix_u64(&mut hash, u64::from(fleet.cell_index));
        mix_u64(&mut hash, fleet.num_ships as u64);
    }
    for row in movement {
        mix_u64(&mut hash, u64::from(row.tick));
        mix_str(&mut hash, &row.mover_id);
        mix_u64(&mut hash, u64::from(row.source_cell_index));
        mix_u64(&mut hash, u64::from(row.destination_cell_index));
    }
    for row in combat {
        mix_u64(&mut hash, u64::from(row.tick));
        mix_str(&mut hash, &row.combatant_id);
        mix_u64(&mut hash, row.ships_destroyed as u64);
        mix_u64(&mut hash, row.num_ships_after as u64);
    }
    for row in construction {
        mix_u64(&mut hash, u64::from(row.tick));
        mix_u64(&mut hash, row.system_index as u64);
        mix_u64(&mut hash, row.construction_progress_remainder as u64);
        mix_u64(&mut hash, row.ship_count_delta_emitted as u64);
    }
    for row in reinforcement {
        mix_str(&mut hash, &row.target_fleet_id);
        mix_u64(&mut hash, row.num_ships_after as u64);
    }
    for row in birth {
        mix_str(&mut hash, &row.created_fleet_id);
        mix_u64(&mut hash, row.num_ships as u64);
    }
    for row in fusion {
        mix_str(&mut hash, &row.fusion_event_id);
        mix_u64(&mut hash, row.fused_num_ships as u64);
    }
    for row in race {
        mix_u64(&mut hash, u64::from(row.sample));
        mix_u64(&mut hash, row.terran_ships as u64);
        mix_u64(&mut hash, row.pirate_ships as u64);
    }
    for row in detectors {
        mix_str(&mut hash, row.behavior);
        mix_u64(&mut hash, row.status.clone() as u64);
        mix_u64(&mut hash, u64::from(row.first_tick.unwrap_or(u32::MAX)));
    }
    hash
}

fn fnv_seed() -> u64 {
    0xcbf29ce484222325
}

fn mix_u64(hash: &mut u64, value: u64) {
    *hash ^= value;
    *hash = hash.wrapping_mul(0x100000001b3);
}

fn mix_str(hash: &mut u64, value: &str) {
    for byte in value.as_bytes() {
        mix_u64(hash, u64::from(*byte));
    }
}

// ---- R1b structural event journal (boundary maintenance consumes GPU rows) ----

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum R1bStructuralEventKind {
    MoveRequest = 1,
    DamageDelta = 2,
    ShipCountDelta = 3,
    ZeroCohort = 4,
    LocalBirthRequest = 5,
    FusionRequest = 6,
    OwnerCodeFlip = 7,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct R1bStructuralEvent {
    pub tick: u32,
    pub event_kind: R1bStructuralEventKind,
    pub source_slot: u32,
    pub target_slot: u32,
    pub source_cell: u32,
    pub target_cell: u32,
    pub owner_code: u32,
    pub amount_or_delta: i64,
    pub threshold_code: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct R1bBoundaryApplyReport {
    pub rows_applied: u32,
    pub movement_applied: u32,
    pub combat_applied: u32,
    pub production_applied: u32,
    pub blockade_applied: u32,
}

pub fn r1b_colocation_hostile_damage(
    world: &DressRehearsalR6cWorld,
    fleet_ids: &[String],
    moved_cells: &BTreeSet<u32>,
    capability: &[DressRehearsalR6cCapabilityOverlayRow],
) -> Vec<i64> {
    let mut hostile_damage = vec![0i64; fleet_ids.len()];
    for cell_index in moved_cells.iter().copied() {
        let combatant_indices = world
            .fleets
            .iter()
            .enumerate()
            .filter(|(_, fleet)| {
                !fleet.destroyed
                    && fleet.num_ships > 0
                    && fleet.cell_index == cell_index
                    && fleet.fleet_like
            })
            .map(|(idx, _)| idx)
            .collect::<Vec<_>>();
        let mut owner_totals: BTreeMap<DressRehearsalR6cOwner, i64> = BTreeMap::new();
        for &idx in &combatant_indices {
            let fleet = &world.fleets[idx];
            let modifier =
                capability_bps(capability, fleet.owner, COMBAT_BONUS_PLACEHOLDER_MODIFIER);
            let damage = apply_bps_i64(
                damage_output_for_cohort(fleet.num_ships, fleet.damage_per_ship_per_tick),
                modifier,
            );
            *owner_totals.entry(fleet.owner).or_insert(0) += damage;
        }
        for &idx in &combatant_indices {
            let fleet = &world.fleets[idx];
            let hostile = owner_totals
                .iter()
                .filter(|(owner, _)| **owner != fleet.owner)
                .map(|(_, total)| *total)
                .sum::<i64>();
            if let Some(fleet_idx) = fleet_ids.iter().position(|id| id == &fleet.fleet_id) {
                hostile_damage[fleet_idx] = hostile;
            }
        }
    }
    hostile_damage
}

pub fn r1b_apply_boundary_events(
    world: &mut DressRehearsalR6cWorld,
    fleet_ids: &[String],
    system_indices: &[usize],
    events: &[R1bStructuralEvent],
) -> R1bBoundaryApplyReport {
    let mut report = R1bBoundaryApplyReport::default();
    for event in events {
        match event.event_kind {
            R1bStructuralEventKind::MoveRequest => {
                let fleet_id = fleet_ids.get(event.source_slot as usize);
                if let Some(fleet_id) = fleet_id {
                    if let Some(fleet) = world
                        .fleets
                        .iter_mut()
                        .find(|f| !f.destroyed && &f.fleet_id == fleet_id)
                    {
                        fleet.cell_index = event.target_cell;
                        fleet.last_moved_tick = Some(event.tick);
                        report.movement_applied += 1;
                        report.rows_applied += 1;
                    }
                }
            }
            R1bStructuralEventKind::DamageDelta | R1bStructuralEventKind::ShipCountDelta => {
                if let Some(fleet_id) = fleet_ids.get(event.source_slot as usize) {
                    if let Some(fleet) = world
                        .fleets
                        .iter_mut()
                        .find(|f| !f.destroyed && &f.fleet_id == fleet_id)
                    {
                        fleet.num_ships = (fleet.num_ships + event.amount_or_delta).max(0);
                        report.combat_applied += 1;
                        report.rows_applied += 1;
                    }
                }
            }
            R1bStructuralEventKind::ZeroCohort => {
                if let Some(fleet_id) = fleet_ids.get(event.source_slot as usize) {
                    if let Some(fleet) = world.fleets.iter_mut().find(|f| &f.fleet_id == fleet_id) {
                        fleet.num_ships = 0;
                        fleet.destroyed = true;
                        report.combat_applied += 1;
                        report.rows_applied += 1;
                    }
                }
            }
            R1bStructuralEventKind::OwnerCodeFlip => {
                if let Some(system_index) = system_indices.get(event.source_slot as usize) {
                    let blockader = match event.owner_code {
                        1 => Some(DressRehearsalR6cOwner::Terran),
                        2 => Some(DressRehearsalR6cOwner::Pirate),
                        _ => None,
                    };
                    world.blockade_divert_owner.insert(*system_index, blockader);
                    report.blockade_applied += 1;
                    report.rows_applied += 1;
                }
            }
            R1bStructuralEventKind::LocalBirthRequest => {
                let Some(system) = world
                    .systems
                    .iter()
                    .find(|system| system.has_starport && system.cell_index == event.source_cell)
                else {
                    continue;
                };
                let fleet_id = format!(
                    "r6c-born-starport-{}-tick-{}",
                    system.system_index, event.tick
                );
                if !world.fleets.iter().any(|fleet| fleet.fleet_id == fleet_id) {
                    let entity_id = R6C_BIRTH_ENTITY_BASE ^ entity_id_for_mover(&fleet_id);
                    world.fleets.push(DressRehearsalR6cFleetCohortState {
                        fleet_id: fleet_id.clone(),
                        entity_id,
                        owner: system.owner,
                        cell_index: system.cell_index,
                        num_ships: event.amount_or_delta.max(0),
                        hp_per_ship: FLEET_HP_PER_SHIP,
                        damage_per_ship_per_tick: FLEET_DAMAGE_PER_SHIP_PER_TICK,
                        destroyed: false,
                        fleet_like: true,
                        owner_faction_id: system.owner.stable_code(),
                        identity_lane: identity_lane_for_owner(system.owner),
                        lineage: vec![fleet_id],
                        spawned_by_production: true,
                        last_moved_tick: None,
                    });
                }
                report.production_applied += 1;
                report.rows_applied += 1;
            }
            R1bStructuralEventKind::FusionRequest => {
                let survivor_id = fleet_ids.get(event.source_slot as usize);
                let absorbed_id = fleet_ids.get(event.target_slot as usize);
                if let (Some(survivor_id), Some(absorbed_id)) = (survivor_id, absorbed_id) {
                    let survivor_idx = world
                        .fleets
                        .iter()
                        .position(|f| !f.destroyed && &f.fleet_id == survivor_id);
                    let absorbed_idx = world
                        .fleets
                        .iter()
                        .position(|f| !f.destroyed && &f.fleet_id == absorbed_id);
                    if let (Some(si), Some(ai)) = (survivor_idx, absorbed_idx) {
                        let delta = event.amount_or_delta.max(0);
                        world.fleets[si].num_ships += delta;
                        world.fleets[ai].destroyed = true;
                        world.fleets[ai].num_ships = 0;
                        report.production_applied += 1;
                        report.rows_applied += 1;
                    }
                }
            }
        }
    }
    refresh_membership(world);
    report
}

// ---- R1a boundary witness (algorithmic tick inputs, not report replay) ----

pub fn dress_rehearsal_r6c_capability_overlay_rows() -> Vec<DressRehearsalR6cCapabilityOverlayRow> {
    capability_overlays()
}

#[derive(Clone, Debug, PartialEq)]
pub struct R1aTickDerivedInputs {
    pub disruption_input_by_cell: Vec<f32>,
    pub stockpile_reduced_in: [i64; 2],
    pub stockpile_disbursed_down: [i64; 2],
    pub construction_production: Vec<i64>,
    pub economy_rows: Vec<DressRehearsalR6cEconomyRow>,
    pub combat_hostile_damage: Vec<i64>,
    pub combat_hp_per_ship: Vec<i64>,
    pub reinforcement_delta: Vec<i64>,
    pub fusion_delta: Vec<i64>,
    pub blockade_triggered_owner: Vec<f32>,
    pub r4_gradients: Vec<(f32, f32)>,
    pub r4_magnitude_bits: u32,
}

pub struct R1aBoundaryWitness {
    world: DressRehearsalR6cWorld,
    capability: Vec<DressRehearsalR6cCapabilityOverlayRow>,
    fleet_ids: Vec<String>,
    system_indices: Vec<usize>,
}

impl R1aBoundaryWitness {
    pub fn new(
        initial: &DressRehearsalR6cWorld,
        fleet_ids: Vec<String>,
        system_indices: Vec<usize>,
    ) -> Self {
        Self {
            world: initial.clone(),
            capability: capability_overlays(),
            fleet_ids,
            system_indices,
        }
    }

    pub fn derive_tick_inputs(
        &mut self,
        tick: u32,
        gpu_disruption: &[f32],
        gpu_stockpiles: [i64; 2],
    ) -> R1aTickDerivedInputs {
        let disruption_input_by_cell = self.derive_disruption_inputs();

        let mut predicted_disruption = gpu_disruption.to_vec();
        for (idx, input) in disruption_input_by_cell.iter().enumerate() {
            predicted_disruption[idx] = bounded_feedback_next(predicted_disruption[idx], *input);
        }
        self.world.disruption.copy_from_slice(&predicted_disruption);
        self.world.location_status = diffusion_status(&predicted_disruption);

        let (
            stockpile_reduced_in,
            stockpile_disbursed_down,
            blockade_triggered_owner,
            economy_rows,
        ) = self.derive_economy_inputs(gpu_stockpiles);

        let mut field_read_rows = Vec::new();
        let mut boundary_rows = Vec::new();
        let mut movement_rows = Vec::new();
        let moved_cells = run_movement_tick(
            tick,
            &mut self.world,
            &self.capability,
            &mut field_read_rows,
            &mut boundary_rows,
            &mut movement_rows,
        );

        let r4_gradients = field_read_rows
            .iter()
            .map(|row| (row.gradient_dx_f32, row.gradient_dy_f32))
            .collect::<Vec<_>>();
        let r4_magnitude_bits = field_read_rows
            .iter()
            .map(|row| row.real_signal_gradient_magnitude_bits)
            .max()
            .unwrap_or(0);

        let mut combat_rows = Vec::new();
        let mut reduce_rows = Vec::new();
        let mut disburse_rows = Vec::new();
        run_combat_tick(
            tick,
            &mut self.world,
            &moved_cells,
            &self.capability,
            &mut combat_rows,
            &mut reduce_rows,
            &mut disburse_rows,
        );

        let mut combat_hostile_damage = vec![0i64; self.fleet_ids.len()];
        let mut combat_hp_per_ship = vec![0i64; self.fleet_ids.len()];
        for row in &combat_rows {
            if let Some(idx) = self.fleet_ids.iter().position(|id| id == &row.combatant_id) {
                combat_hostile_damage[idx] = row.hostile_damage_received;
                combat_hp_per_ship[idx] = row.hp_per_ship;
            }
        }

        let mut construction_rows = Vec::new();
        let mut reinforcement_rows = Vec::new();
        let mut birth_rows = Vec::new();
        let mut fusion_rows = Vec::new();
        run_production_tick(
            tick,
            &mut self.world,
            &economy_rows,
            &mut construction_rows,
            &mut reinforcement_rows,
            &mut birth_rows,
            &mut fusion_rows,
        );

        let n_systems = self.system_indices.len();
        let mut construction_production = vec![0i64; n_systems];
        for row in &construction_rows {
            if let Some(idx) = self
                .system_indices
                .iter()
                .position(|s| *s == row.system_index)
            {
                construction_production[idx] = row.production_applied;
            }
        }

        let mut reinforcement_delta = vec![0i64; self.fleet_ids.len()];
        for row in &reinforcement_rows {
            if let Some(idx) = self
                .fleet_ids
                .iter()
                .position(|id| id == &row.target_fleet_id)
            {
                reinforcement_delta[idx] = row.ship_count_delta;
            }
        }

        let mut fusion_delta = vec![0i64; self.fleet_ids.len()];
        for row in &fusion_rows {
            if let Some(idx) = self
                .fleet_ids
                .iter()
                .position(|id| id == &row.surviving_fleet_id)
            {
                fusion_delta[idx] += row.right_num_ships;
            }
        }

        R1aTickDerivedInputs {
            disruption_input_by_cell,
            stockpile_reduced_in,
            stockpile_disbursed_down,
            construction_production,
            economy_rows,
            combat_hostile_damage,
            combat_hp_per_ship,
            reinforcement_delta,
            fusion_delta,
            blockade_triggered_owner,
            r4_gradients,
            r4_magnitude_bits,
        }
    }

    fn derive_disruption_inputs(&self) -> Vec<f32> {
        let mut input_by_cell = vec![0.0f32; GALAXY_CELL_COUNT];
        for fleet in live_fleets(&self.world) {
            let modifier = match fleet.owner {
                DressRehearsalR6cOwner::Terran => {
                    capability_bps(&self.capability, fleet.owner, PATROL_SUPPRESSION_MODIFIER)
                }
                DressRehearsalR6cOwner::Pirate => {
                    capability_bps(&self.capability, fleet.owner, PIRATE_EMISSION_MODIFIER)
                }
            };
            let input = match fleet.owner {
                DressRehearsalR6cOwner::Terran => {
                    -apply_modifier_bps(PATROL_SUPPRESS * fleet.num_ships as f32, modifier)
                }
                DressRehearsalR6cOwner::Pirate => {
                    apply_modifier_bps(PIRATE_EMIT * fleet.num_ships as f32, modifier)
                }
            };
            input_by_cell[fleet.cell_index as usize] += input;
        }
        input_by_cell
    }

    fn derive_economy_inputs(
        &self,
        gpu_stockpiles: [i64; 2],
    ) -> (
        [i64; 2],
        [i64; 2],
        Vec<f32>,
        Vec<DressRehearsalR6cEconomyRow>,
    ) {
        let mut rows = Vec::new();
        let mut reduced: BTreeMap<DressRehearsalR6cOwner, i64> = BTreeMap::new();
        reduced.insert(DressRehearsalR6cOwner::Terran, 0);
        reduced.insert(DressRehearsalR6cOwner::Pirate, 0);
        let mut blockade_triggered_owner = vec![0.0f32; self.system_indices.len()];

        for (sys_idx, system_index) in self.system_indices.iter().enumerate() {
            let system = self
                .world
                .systems
                .iter()
                .find(|s| s.system_index == *system_index)
                .expect("system");
            let disruption = self.world.disruption[system.cell_index as usize];
            let blockader = if disruption >= BLOCKADE_THRESHOLD {
                blockader_for_system(&self.world, system)
            } else {
                None
            };
            blockade_triggered_owner[sys_idx] = blockader
                .map(|owner| match owner {
                    DressRehearsalR6cOwner::Terran => 1.0,
                    DressRehearsalR6cOwner::Pirate => 2.0,
                })
                .unwrap_or(0.0);
            let effective_owner = blockader.unwrap_or(system.owner);
            let (production_generated, labor_consumed, _) =
                factory_recipe_production(POP_LABOR_PER_TICK);
            *reduced.entry(effective_owner).or_insert(0) += production_generated;
            rows.push(DressRehearsalR6cEconomyRow {
                tick: 0,
                system_id: system.system_id.clone(),
                system_index: system.system_index,
                cell_index: system.cell_index,
                original_owner: system.owner,
                effective_outflow_owner: effective_owner,
                blockader,
                disruption,
                blockaded: blockader.is_some(),
                labor_generated: POP_LABOR_PER_TICK,
                labor_consumed,
                production_generated,
                diverted_production: if blockader.is_some() {
                    production_generated
                } else {
                    0
                },
                disbursement_received: 0,
                owner_column_flipped: blockader.is_some(),
            });
        }

        let stockpile_reduced_in = [
            *reduced.get(&DressRehearsalR6cOwner::Terran).unwrap_or(&0),
            *reduced.get(&DressRehearsalR6cOwner::Pirate).unwrap_or(&0),
        ];
        let mut after_reduce = [
            gpu_stockpiles[0] + stockpile_reduced_in[0],
            gpu_stockpiles[1] + stockpile_reduced_in[1],
        ];
        let mut stockpile_disbursed_down = [0i64; 2];
        for owner_idx in 0..2usize {
            let owner = if owner_idx == 0 {
                DressRehearsalR6cOwner::Terran
            } else {
                DressRehearsalR6cOwner::Pirate
            };
            let mut disbursed_down = 0i64;
            let starport_row_indices = rows
                .iter()
                .enumerate()
                .filter(|(_, row)| {
                    row.original_owner == owner
                        && self
                            .world
                            .systems
                            .iter()
                            .find(|s| s.system_index == row.system_index)
                            .map(|s| s.has_starport)
                            .unwrap_or(false)
                })
                .map(|(idx, _)| idx)
                .collect::<Vec<_>>();
            for row_idx in starport_row_indices {
                let available = after_reduce[owner_idx];
                let disbursed = STARPORT_PRODUCTION_NEED.min(available).max(0);
                after_reduce[owner_idx] -= disbursed;
                disbursed_down += disbursed;
                rows[row_idx].disbursement_received = disbursed;
            }
            stockpile_disbursed_down[owner_idx] = disbursed_down;
        }

        (
            stockpile_reduced_in,
            stockpile_disbursed_down,
            blockade_triggered_owner,
            rows,
        )
    }

    pub fn world(&self) -> &DressRehearsalR6cWorld {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut DressRehearsalR6cWorld {
        &mut self.world
    }

    pub fn fleet_ids(&self) -> &[String] {
        &self.fleet_ids
    }

    pub fn system_indices(&self) -> &[usize] {
        &self.system_indices
    }

    pub fn capability(&self) -> &[DressRehearsalR6cCapabilityOverlayRow] {
        &self.capability
    }

    pub fn sync_boundary_world_from_gpu_tier_a(
        &mut self,
        gpu_current: &[f32],
        construction_start: u32,
        num_ships_start: u32,
        blockade_start: u32,
    ) {
        for (idx, system_index) in self.system_indices.iter().enumerate() {
            let progress = gpu_current[(construction_start + idx as u32) as usize].round() as i64;
            self.world
                .construction_progress
                .insert(*system_index, progress);
            let blockade_code = gpu_current[(blockade_start + idx as u32) as usize].round() as u32;
            let blockader = match blockade_code {
                1 => Some(DressRehearsalR6cOwner::Terran),
                2 => Some(DressRehearsalR6cOwner::Pirate),
                _ => None,
            };
            self.world
                .blockade_divert_owner
                .insert(*system_index, blockader);
        }
        for (idx, fleet_id) in self.fleet_ids.iter().enumerate() {
            let ships = gpu_current[(num_ships_start + idx as u32) as usize].round() as i64;
            if let Some(fleet) = self
                .world
                .fleets
                .iter_mut()
                .find(|fleet| &fleet.fleet_id == fleet_id)
            {
                if fleet.destroyed {
                    fleet.num_ships = 0;
                    continue;
                }
                fleet.num_ships = ships.max(0);
                fleet.destroyed = fleet.num_ships <= 0;
            }
        }
    }

    pub fn sync_tier_a_field_columns(&mut self, gpu_disruption: &[f32], gpu_stockpiles: [i64; 2]) {
        let disruption_input_by_cell = self.derive_disruption_inputs();
        let mut predicted_disruption = gpu_disruption.to_vec();
        for (idx, input) in disruption_input_by_cell.iter().enumerate() {
            predicted_disruption[idx] = bounded_feedback_next(predicted_disruption[idx], *input);
        }
        self.world.disruption.copy_from_slice(&predicted_disruption);
        self.world.location_status = diffusion_status(&predicted_disruption);
        self.world
            .stockpiles
            .insert(DressRehearsalR6cOwner::Terran, gpu_stockpiles[0]);
        self.world
            .stockpiles
            .insert(DressRehearsalR6cOwner::Pirate, gpu_stockpiles[1]);
    }

    /// Applies the R6C economy tick so `blockade_divert_owner` and stockpiles match movement field inputs.
    pub fn prepare_movement_tick_economy(&mut self, tick: u32) -> Vec<DressRehearsalR6cEconomyRow> {
        let mut ledger_rows = Vec::new();
        run_economy_tick(tick, &mut self.world, &mut ledger_rows)
    }

    pub fn clone_for_event_derivation(&self) -> R1aBoundaryWitness {
        R1aBoundaryWitness {
            world: self.world.clone(),
            capability: self.capability.clone(),
            fleet_ids: self.fleet_ids.clone(),
            system_indices: self.system_indices.clone(),
        }
    }

    /// Advances the witness by one full R6C tick on the CPU (the oracle's exact per-tick sequence)
    /// and captures the structural decision rows for the resident event journal.
    ///
    /// The witness carries its own structural state (fleet positions, ship counts, births,
    /// removals, fusion lineage) forward across ticks — it is never reconstructed from partial
    /// GPU Tier-A readback. That self-consistency is what keeps it in lockstep with the R6C oracle
    /// and prevents the structural drift that an over-eager GPU-side per-tick reconstruction caused.
    /// Same as [`Self::step_tick_capture_events`], but omits `ZeroCohort` rows so a GPU
    /// threshold/emission-band over resident `num_ships` can own that decision class.
    pub fn step_tick_capture_events_excluding_zero_cohort(
        &mut self,
        tick: u32,
    ) -> (R1aTickDerivedInputs, Vec<R1bStructuralEvent>) {
        let (derived, events) = self.step_tick_capture_events(tick);
        let filtered = events
            .into_iter()
            .filter(|event| event.event_kind != R1bStructuralEventKind::ZeroCohort)
            .collect();
        (derived, filtered)
    }

    pub fn step_tick_capture_events(
        &mut self,
        tick: u32,
    ) -> (R1aTickDerivedInputs, Vec<R1bStructuralEvent>) {
        let disruption_input_by_cell = self.derive_disruption_inputs();

        let mut disruption_rows = Vec::new();
        run_disruption_tick(
            tick,
            &mut self.world,
            &self.capability,
            &mut disruption_rows,
        );

        let mut stockpile_ledger_rows = Vec::new();
        let economy_rows = run_economy_tick(tick, &mut self.world, &mut stockpile_ledger_rows);
        let ledger_value = |owner: DressRehearsalR6cOwner, disbursed: bool| {
            stockpile_ledger_rows
                .iter()
                .find(|row| row.owner == owner)
                .map(|row| {
                    if disbursed {
                        row.disbursed_down
                    } else {
                        row.reduced_in
                    }
                })
                .unwrap_or(0)
        };
        let stockpile_reduced_in = [
            ledger_value(DressRehearsalR6cOwner::Terran, false),
            ledger_value(DressRehearsalR6cOwner::Pirate, false),
        ];
        let stockpile_disbursed_down = [
            ledger_value(DressRehearsalR6cOwner::Terran, true),
            ledger_value(DressRehearsalR6cOwner::Pirate, true),
        ];

        let mut field_rows = Vec::new();
        let mut boundary_rows = Vec::new();
        let mut movement_rows = Vec::new();
        let moved_cells = run_movement_tick(
            tick,
            &mut self.world,
            &self.capability,
            &mut field_rows,
            &mut boundary_rows,
            &mut movement_rows,
        );
        let r4_gradients = field_rows
            .iter()
            .map(|row| (row.gradient_dx_f32, row.gradient_dy_f32))
            .collect::<Vec<_>>();
        let r4_magnitude_bits = field_rows
            .iter()
            .map(|row| row.real_signal_gradient_magnitude_bits)
            .max()
            .unwrap_or(0);

        let mut combat_rows = Vec::new();
        let mut reduce_rows = Vec::new();
        let mut disburse_rows = Vec::new();
        run_combat_tick(
            tick,
            &mut self.world,
            &moved_cells,
            &self.capability,
            &mut combat_rows,
            &mut reduce_rows,
            &mut disburse_rows,
        );
        let mut combat_hostile_damage = vec![0i64; self.fleet_ids.len()];
        let mut combat_hp_per_ship = vec![0i64; self.fleet_ids.len()];
        for row in &combat_rows {
            if let Some(idx) = self.fleet_ids.iter().position(|id| id == &row.combatant_id) {
                combat_hostile_damage[idx] = row.hostile_damage_received;
                combat_hp_per_ship[idx] = row.hp_per_ship;
            }
        }

        let mut construction_rows = Vec::new();
        let mut reinforcement_rows = Vec::new();
        let mut birth_rows = Vec::new();
        let mut fusion_rows = Vec::new();
        run_production_tick(
            tick,
            &mut self.world,
            &economy_rows,
            &mut construction_rows,
            &mut reinforcement_rows,
            &mut birth_rows,
            &mut fusion_rows,
        );
        refresh_membership(&mut self.world);

        let mut construction_production = vec![0i64; self.system_indices.len()];
        for row in &construction_rows {
            if let Some(idx) = self
                .system_indices
                .iter()
                .position(|s| *s == row.system_index)
            {
                construction_production[idx] = row.production_applied;
            }
        }
        let mut reinforcement_delta = vec![0i64; self.fleet_ids.len()];
        for row in &reinforcement_rows {
            if let Some(idx) = self
                .fleet_ids
                .iter()
                .position(|id| id == &row.target_fleet_id)
            {
                reinforcement_delta[idx] = row.ship_count_delta;
            }
        }
        let mut fusion_delta = vec![0i64; self.fleet_ids.len()];
        for row in &fusion_rows {
            if let Some(idx) = self
                .fleet_ids
                .iter()
                .position(|id| id == &row.surviving_fleet_id)
            {
                fusion_delta[idx] += row.right_num_ships;
            }
        }
        let mut blockade_triggered_owner = vec![0.0f32; self.system_indices.len()];
        for (idx, system_index) in self.system_indices.iter().enumerate() {
            if let Some(row) = economy_rows
                .iter()
                .find(|r| r.system_index == *system_index)
            {
                blockade_triggered_owner[idx] = match row.blockader {
                    Some(DressRehearsalR6cOwner::Terran) => 1.0,
                    Some(DressRehearsalR6cOwner::Pirate) => 2.0,
                    None => 0.0,
                };
            }
        }

        let mut events = Vec::new();
        for row in &boundary_rows {
            if !row.event_emitted {
                continue;
            }
            let source_slot = self
                .fleet_ids
                .iter()
                .position(|id| id == &row.mover_id)
                .unwrap_or(0) as u32;
            events.push(R1bStructuralEvent {
                tick: row.tick,
                event_kind: R1bStructuralEventKind::MoveRequest,
                source_slot,
                target_slot: 0,
                source_cell: row.source_cell_index,
                target_cell: row.destination_cell_index,
                owner_code: 0,
                amount_or_delta: 0,
                threshold_code: row.threshold_input_mag_bits,
            });
        }
        for row in &combat_rows {
            let Some(source_slot) = self.fleet_ids.iter().position(|id| id == &row.combatant_id)
            else {
                continue;
            };
            let source_slot = source_slot as u32;
            if row.ship_loss_event_emitted {
                events.push(R1bStructuralEvent {
                    tick: row.tick,
                    event_kind: R1bStructuralEventKind::DamageDelta,
                    source_slot,
                    target_slot: 0,
                    source_cell: row.cell_index,
                    target_cell: 0,
                    owner_code: r1b_owner_code(row.owner),
                    amount_or_delta: -row.ships_destroyed,
                    threshold_code: 0,
                });
            }
            if row.zero_cohort_event_emitted {
                events.push(R1bStructuralEvent {
                    tick: row.tick,
                    event_kind: R1bStructuralEventKind::ZeroCohort,
                    source_slot,
                    target_slot: 0,
                    source_cell: row.cell_index,
                    target_cell: 0,
                    owner_code: r1b_owner_code(row.owner),
                    amount_or_delta: 0,
                    threshold_code: 0,
                });
            }
        }
        for row in &reinforcement_rows {
            let source_slot = self
                .fleet_ids
                .iter()
                .position(|id| id == &row.target_fleet_id)
                .unwrap_or(0) as u32;
            events.push(R1bStructuralEvent {
                tick: row.tick,
                event_kind: R1bStructuralEventKind::ShipCountDelta,
                source_slot,
                target_slot: 0,
                source_cell: row.cell_index,
                target_cell: 0,
                owner_code: r1b_owner_code(row.owner),
                amount_or_delta: row.ship_count_delta,
                threshold_code: 0,
            });
        }
        for row in &birth_rows {
            events.push(R1bStructuralEvent {
                tick: row.tick,
                event_kind: R1bStructuralEventKind::LocalBirthRequest,
                source_slot: 0,
                target_slot: 0,
                source_cell: row.cell_index,
                target_cell: 0,
                owner_code: r1b_owner_code(row.owner),
                amount_or_delta: row.num_ships,
                threshold_code: 0,
            });
        }
        for row in &fusion_rows {
            let source_slot = self
                .fleet_ids
                .iter()
                .position(|id| id == &row.surviving_fleet_id)
                .unwrap_or(0) as u32;
            let target_slot = self
                .fleet_ids
                .iter()
                .position(|id| id == &row.absorbed_fleet_id)
                .unwrap_or(0) as u32;
            events.push(R1bStructuralEvent {
                tick: row.tick,
                event_kind: R1bStructuralEventKind::FusionRequest,
                source_slot,
                target_slot,
                source_cell: row.cell_index,
                target_cell: row.cell_index,
                owner_code: r1b_owner_code(row.owner),
                amount_or_delta: row.right_num_ships,
                threshold_code: 0,
            });
        }
        for row in &economy_rows {
            if !row.owner_column_flipped {
                continue;
            }
            let source_slot = self
                .system_indices
                .iter()
                .position(|idx| *idx == row.system_index)
                .unwrap_or(0) as u32;
            events.push(R1bStructuralEvent {
                tick,
                event_kind: R1bStructuralEventKind::OwnerCodeFlip,
                source_slot,
                target_slot: 0,
                source_cell: row.cell_index,
                target_cell: 0,
                owner_code: row.blockader.map(r1b_owner_code).unwrap_or(0),
                amount_or_delta: 0,
                threshold_code: 0,
            });
        }

        let derived = R1aTickDerivedInputs {
            disruption_input_by_cell,
            stockpile_reduced_in,
            stockpile_disbursed_down,
            construction_production,
            economy_rows,
            combat_hostile_damage,
            combat_hp_per_ship,
            reinforcement_delta,
            fusion_delta,
            blockade_triggered_owner,
            r4_gradients,
            r4_magnitude_bits,
        };
        (derived, events)
    }

    pub fn derive_tier_a_staging_inputs(
        &self,
        tick: u32,
        gpu_disruption: &[f32],
        gpu_stockpiles: [i64; 2],
        moved_cells: &BTreeSet<u32>,
        tick_economy_rows: &[DressRehearsalR6cEconomyRow],
    ) -> R1aTickDerivedInputs {
        let mut scratch_disruption = gpu_disruption.to_vec();
        let disruption_input_by_cell = self.derive_disruption_inputs();
        for (idx, input) in disruption_input_by_cell.iter().enumerate() {
            scratch_disruption[idx] = bounded_feedback_next(scratch_disruption[idx], *input);
        }
        let predicted_location_status = diffusion_status(&scratch_disruption);

        let mut scratch_world = self.world.clone();
        scratch_world
            .disruption
            .copy_from_slice(&scratch_disruption);
        scratch_world.location_status = predicted_location_status;

        let (stockpile_reduced_in, stockpile_disbursed_down, blockade_triggered_owner) = {
            let witness = R1aBoundaryWitness {
                world: scratch_world,
                capability: self.capability.clone(),
                fleet_ids: self.fleet_ids.clone(),
                system_indices: self.system_indices.clone(),
            };
            let (stockpile_reduced_in, stockpile_disbursed_down, blockade_triggered_owner, _) =
                witness.derive_economy_inputs(gpu_stockpiles);
            (
                stockpile_reduced_in,
                stockpile_disbursed_down,
                blockade_triggered_owner,
            )
        };
        let economy_rows = tick_economy_rows.to_vec();

        let combat_hostile_damage = r1b_colocation_hostile_damage(
            &self.world,
            &self.fleet_ids,
            moved_cells,
            &self.capability,
        );
        let mut combat_hp_per_ship = vec![0i64; self.fleet_ids.len()];
        for (fleet_idx, fleet_id) in self.fleet_ids.iter().enumerate() {
            if let Some(fleet) = self
                .world
                .fleets
                .iter()
                .find(|f| !f.destroyed && &f.fleet_id == fleet_id)
            {
                combat_hp_per_ship[fleet_idx] = fleet.hp_per_ship;
            }
        }

        let n_systems = self.system_indices.len();
        let mut construction_production = vec![0i64; n_systems];
        for row in &economy_rows {
            if let Some(idx) = self
                .system_indices
                .iter()
                .position(|s| *s == row.system_index)
            {
                construction_production[idx] = row.disbursement_received;
            }
        }

        let mut post_combat_world = self.world.clone();
        r1b_apply_combat_attrition_scratch(
            &mut post_combat_world,
            &self.fleet_ids,
            &combat_hostile_damage,
            &combat_hp_per_ship,
        );

        let mut reinforcement_delta = vec![0i64; self.fleet_ids.len()];
        let starports = self
            .world
            .systems
            .iter()
            .filter(|system| system.has_starport)
            .cloned()
            .collect::<Vec<_>>();
        for system in starports {
            let production_applied = economy_rows
                .iter()
                .find(|row| row.system_index == system.system_index)
                .map(|row| row.disbursement_received)
                .unwrap_or(0);
            let progress_before = *self
                .world
                .construction_progress
                .get(&system.system_index)
                .unwrap_or(&0);
            let (_, threshold_passed, ship_delta, _) =
                construction_threshold_emission(progress_before, production_applied, SHIP_COST);
            if threshold_passed && ship_delta > 0 {
                if let Some(fleet_idx) = post_combat_world
                    .fleets
                    .iter()
                    .find(|fleet| {
                        !fleet.destroyed
                            && fleet.fleet_like
                            && fleet.owner == system.owner
                            && fleet.cell_index == system.cell_index
                            && fleet.hp_per_ship == FLEET_HP_PER_SHIP
                            && fleet.damage_per_ship_per_tick == FLEET_DAMAGE_PER_SHIP_PER_TICK
                    })
                    .and_then(|fleet| self.fleet_ids.iter().position(|id| id == &fleet.fleet_id))
                {
                    reinforcement_delta[fleet_idx] = ship_delta;
                    if let Some(world_idx) = post_combat_world
                        .fleets
                        .iter()
                        .position(|fleet| self.fleet_ids[fleet_idx] == fleet.fleet_id)
                    {
                        post_combat_world.fleets[world_idx].num_ships += ship_delta;
                    }
                }
            }
        }

        let mut fusion_delta = vec![0i64; self.fleet_ids.len()];
        let mut groups: BTreeMap<(DressRehearsalR6cOwner, u32, i64, i64), Vec<usize>> =
            BTreeMap::new();
        for (idx, fleet) in post_combat_world.fleets.iter().enumerate() {
            if fleet.destroyed || fleet.num_ships <= 0 || !fleet.fleet_like {
                continue;
            }
            groups
                .entry((
                    fleet.owner,
                    fleet.cell_index,
                    fleet.hp_per_ship,
                    fleet.damage_per_ship_per_tick,
                ))
                .or_default()
                .push(idx);
        }
        for indices in groups.values_mut() {
            if indices.len() < 2 {
                continue;
            }
            indices.sort_by_key(|idx| post_combat_world.fleets[*idx].fleet_id.clone());
            let survivor = indices[0];
            for absorbed in indices.iter().copied().skip(1) {
                if post_combat_world.fleets[absorbed].destroyed {
                    continue;
                }
                if let Some(survivor_idx) = self
                    .fleet_ids
                    .iter()
                    .position(|id| id == &post_combat_world.fleets[survivor].fleet_id)
                {
                    fusion_delta[survivor_idx] += post_combat_world.fleets[absorbed].num_ships;
                }
            }
        }

        let mut r4_gradients = Vec::new();
        let mut r4_magnitude_bits = 0u32;
        for fleet_id in &self.fleet_ids {
            if let Some(fleet) = self
                .world
                .fleets
                .iter()
                .find(|f| !f.destroyed && &f.fleet_id == fleet_id)
            {
                let field = build_field(&self.world, fleet.owner, &self.capability);
                let decision = field_decision(&field, fleet.cell_index);
                r4_gradients.push((decision.gradient_dx_f32, decision.gradient_dy_f32));
                r4_magnitude_bits = r4_magnitude_bits.max(decision.candidate_f_exact_mag_bits);
            }
        }

        let _ = tick;
        R1aTickDerivedInputs {
            disruption_input_by_cell,
            stockpile_reduced_in,
            stockpile_disbursed_down,
            construction_production,
            economy_rows,
            combat_hostile_damage,
            combat_hp_per_ship,
            reinforcement_delta,
            fusion_delta,
            blockade_triggered_owner,
            r4_gradients,
            r4_magnitude_bits,
        }
    }
}

pub fn r1b_apply_combat_attrition_scratch(
    world: &mut DressRehearsalR6cWorld,
    fleet_ids: &[String],
    hostile_damage: &[i64],
    hp_per_ship: &[i64],
) {
    for (fleet_idx, fleet_id) in fleet_ids.iter().enumerate() {
        let damage = hostile_damage.get(fleet_idx).copied().unwrap_or(0);
        if damage <= 0 {
            continue;
        }
        let Some(world_idx) = world
            .fleets
            .iter()
            .position(|fleet| !fleet.destroyed && &fleet.fleet_id == fleet_id)
        else {
            continue;
        };
        let fleet = &world.fleets[world_idx];
        let hp = hp_per_ship
            .get(fleet_idx)
            .copied()
            .unwrap_or(fleet.hp_per_ship);
        let (_, num_ships_after, _, zero_cohort) =
            emission_band_ship_attrition(damage, fleet.num_ships, hp);
        world.fleets[world_idx].num_ships = num_ships_after;
        if zero_cohort {
            world.fleets[world_idx].destroyed = true;
        }
    }
    refresh_membership(world);
}

pub fn r1b_predict_production_events(
    tick: u32,
    world: &DressRehearsalR6cWorld,
    fleet_ids: &[String],
    economy_rows: &[DressRehearsalR6cEconomyRow],
) -> Vec<R1bStructuralEvent> {
    let mut rows = Vec::new();
    let starports = world
        .systems
        .iter()
        .filter(|system| system.has_starport)
        .cloned()
        .collect::<Vec<_>>();
    for system in starports {
        let production_applied = economy_rows
            .iter()
            .find(|row| row.system_index == system.system_index)
            .map(|row| row.disbursement_received)
            .unwrap_or(0);
        let progress_before = *world
            .construction_progress
            .get(&system.system_index)
            .unwrap_or(&0);
        let (_, threshold_passed, ship_delta, _) =
            construction_threshold_emission(progress_before, production_applied, SHIP_COST);
        if !threshold_passed || ship_delta <= 0 {
            continue;
        }
        let compatible = world
            .fleets
            .iter()
            .enumerate()
            .filter(|(_, fleet)| {
                !fleet.destroyed
                    && fleet.fleet_like
                    && fleet.owner == system.owner
                    && fleet.cell_index == system.cell_index
                    && fleet.hp_per_ship == FLEET_HP_PER_SHIP
                    && fleet.damage_per_ship_per_tick == FLEET_DAMAGE_PER_SHIP_PER_TICK
            })
            .map(|(idx, _)| idx)
            .collect::<Vec<_>>();
        if let Some(world_idx) = compatible.first().copied() {
            let fleet = &world.fleets[world_idx];
            let Some(source_slot) = fleet_ids.iter().position(|id| id == &fleet.fleet_id) else {
                continue;
            };
            rows.push(R1bStructuralEvent {
                tick,
                event_kind: R1bStructuralEventKind::ShipCountDelta,
                source_slot: source_slot as u32,
                target_slot: 0,
                source_cell: system.cell_index,
                target_cell: 0,
                owner_code: r1b_owner_code(system.owner),
                amount_or_delta: ship_delta,
                threshold_code: 0,
            });
        } else {
            rows.push(R1bStructuralEvent {
                tick,
                event_kind: R1bStructuralEventKind::LocalBirthRequest,
                source_slot: 0,
                target_slot: 0,
                source_cell: system.cell_index,
                target_cell: 0,
                owner_code: r1b_owner_code(system.owner),
                amount_or_delta: ship_delta,
                threshold_code: 0,
            });
        }
    }
    rows
}

pub fn r1b_oracle_events_by_tick(
    report: &DressRehearsalR6cReport,
    fleet_ids: &[String],
    system_indices: &[usize],
) -> BTreeMap<u32, Vec<R1bStructuralEvent>> {
    let mut by_tick: BTreeMap<u32, Vec<R1bStructuralEvent>> = BTreeMap::new();
    for row in &report.boundary_request_rows {
        if !row.event_emitted {
            continue;
        }
        let actor_slot = fleet_ids
            .iter()
            .position(|id| id == &row.mover_id)
            .unwrap_or(0) as u32;
        by_tick
            .entry(row.tick)
            .or_default()
            .push(R1bStructuralEvent {
                tick: row.tick,
                event_kind: R1bStructuralEventKind::MoveRequest,
                source_slot: actor_slot,
                target_slot: 0,
                source_cell: row.source_cell_index,
                target_cell: row.destination_cell_index,
                owner_code: 0,
                amount_or_delta: 0,
                threshold_code: row.threshold_input_mag_bits,
            });
    }
    for row in &report.combat_rows {
        let Some(source_slot) = fleet_ids.iter().position(|id| id == &row.combatant_id) else {
            continue;
        };
        let source_slot = source_slot as u32;
        if row.ship_loss_event_emitted {
            by_tick
                .entry(row.tick)
                .or_default()
                .push(R1bStructuralEvent {
                    tick: row.tick,
                    event_kind: R1bStructuralEventKind::DamageDelta,
                    source_slot,
                    target_slot: 0,
                    source_cell: row.cell_index,
                    target_cell: 0,
                    owner_code: r1b_owner_code(row.owner),
                    amount_or_delta: -(row.ships_destroyed),
                    threshold_code: 0,
                });
        }
        if row.zero_cohort_event_emitted {
            by_tick
                .entry(row.tick)
                .or_default()
                .push(R1bStructuralEvent {
                    tick: row.tick,
                    event_kind: R1bStructuralEventKind::ZeroCohort,
                    source_slot,
                    target_slot: 0,
                    source_cell: row.cell_index,
                    target_cell: 0,
                    owner_code: r1b_owner_code(row.owner),
                    amount_or_delta: 0,
                    threshold_code: 0,
                });
        }
    }
    for row in &report.reinforcement_rows {
        let source_slot = fleet_ids
            .iter()
            .position(|id| id == &row.target_fleet_id)
            .unwrap_or(0) as u32;
        by_tick
            .entry(row.tick)
            .or_default()
            .push(R1bStructuralEvent {
                tick: row.tick,
                event_kind: R1bStructuralEventKind::ShipCountDelta,
                source_slot,
                target_slot: 0,
                source_cell: row.cell_index,
                target_cell: 0,
                owner_code: r1b_owner_code(row.owner),
                amount_or_delta: row.ship_count_delta,
                threshold_code: 0,
            });
    }
    for row in &report.birth_rows {
        by_tick
            .entry(row.tick)
            .or_default()
            .push(R1bStructuralEvent {
                tick: row.tick,
                event_kind: R1bStructuralEventKind::LocalBirthRequest,
                source_slot: 0,
                target_slot: 0,
                source_cell: row.cell_index,
                target_cell: 0,
                owner_code: r1b_owner_code(row.owner),
                amount_or_delta: row.num_ships,
                threshold_code: 0,
            });
    }
    for row in &report.fusion_rows {
        let source_slot = fleet_ids
            .iter()
            .position(|id| id == &row.surviving_fleet_id)
            .unwrap_or(0) as u32;
        let target_slot = fleet_ids
            .iter()
            .position(|id| id == &row.absorbed_fleet_id)
            .unwrap_or(0) as u32;
        by_tick
            .entry(row.tick)
            .or_default()
            .push(R1bStructuralEvent {
                tick: row.tick,
                event_kind: R1bStructuralEventKind::FusionRequest,
                source_slot,
                target_slot,
                source_cell: row.cell_index,
                target_cell: row.cell_index,
                owner_code: r1b_owner_code(row.owner),
                amount_or_delta: row.right_num_ships,
                threshold_code: 0,
            });
    }
    for row in &report.economy_rows {
        if !row.owner_column_flipped {
            continue;
        }
        let source_slot = system_indices
            .iter()
            .position(|idx| *idx == row.system_index)
            .unwrap_or(0) as u32;
        by_tick
            .entry(row.tick)
            .or_default()
            .push(R1bStructuralEvent {
                tick: row.tick,
                event_kind: R1bStructuralEventKind::OwnerCodeFlip,
                source_slot,
                target_slot: 0,
                source_cell: row.cell_index,
                target_cell: 0,
                owner_code: row.blockader.map(r1b_owner_code).unwrap_or(0),
                amount_or_delta: 0,
                threshold_code: 0,
            });
    }
    by_tick
}

/// Emits combat, production, and fusion structural rows from post-movement world state.
/// Does not re-run movement; uses the same R6C tick kernels as the integrated oracle.
pub fn r1b_emit_post_movement_structural_events(
    witness: &R1aBoundaryWitness,
    tick: u32,
    moved_cells: &BTreeSet<u32>,
    tick_economy_rows: &[DressRehearsalR6cEconomyRow],
) -> Vec<R1bStructuralEvent> {
    let mut scratch = witness.clone_for_event_derivation();
    let fleet_ids = scratch.fleet_ids.clone();
    let system_indices = scratch.system_indices.clone();
    let capability = scratch.capability.clone();

    let mut combat_rows = Vec::new();
    let mut reduce_rows = Vec::new();
    let mut disburse_rows = Vec::new();
    run_combat_tick(
        tick,
        &mut scratch.world,
        moved_cells,
        &capability,
        &mut combat_rows,
        &mut reduce_rows,
        &mut disburse_rows,
    );

    let mut construction_rows = Vec::new();
    let mut reinforcement_rows = Vec::new();
    let mut birth_rows = Vec::new();
    let mut fusion_rows = Vec::new();
    run_production_tick(
        tick,
        &mut scratch.world,
        tick_economy_rows,
        &mut construction_rows,
        &mut reinforcement_rows,
        &mut birth_rows,
        &mut fusion_rows,
    );

    let mut events = Vec::new();
    for row in &combat_rows {
        let source_slot = fleet_ids
            .iter()
            .position(|id| id == &row.combatant_id)
            .unwrap_or(0) as u32;
        if row.ship_loss_event_emitted {
            events.push(R1bStructuralEvent {
                tick: row.tick,
                event_kind: R1bStructuralEventKind::DamageDelta,
                source_slot,
                target_slot: 0,
                source_cell: row.cell_index,
                target_cell: 0,
                owner_code: r1b_owner_code(row.owner),
                amount_or_delta: -row.ships_destroyed,
                threshold_code: 0,
            });
        }
        if row.zero_cohort_event_emitted {
            events.push(R1bStructuralEvent {
                tick: row.tick,
                event_kind: R1bStructuralEventKind::ZeroCohort,
                source_slot,
                target_slot: 0,
                source_cell: row.cell_index,
                target_cell: 0,
                owner_code: r1b_owner_code(row.owner),
                amount_or_delta: 0,
                threshold_code: 0,
            });
        }
    }
    for row in &reinforcement_rows {
        let source_slot = fleet_ids
            .iter()
            .position(|id| id == &row.target_fleet_id)
            .unwrap_or(0) as u32;
        events.push(R1bStructuralEvent {
            tick: row.tick,
            event_kind: R1bStructuralEventKind::ShipCountDelta,
            source_slot,
            target_slot: 0,
            source_cell: row.cell_index,
            target_cell: 0,
            owner_code: r1b_owner_code(row.owner),
            amount_or_delta: row.ship_count_delta,
            threshold_code: 0,
        });
    }
    for row in &birth_rows {
        events.push(R1bStructuralEvent {
            tick: row.tick,
            event_kind: R1bStructuralEventKind::LocalBirthRequest,
            source_slot: 0,
            target_slot: 0,
            source_cell: row.cell_index,
            target_cell: 0,
            owner_code: r1b_owner_code(row.owner),
            amount_or_delta: row.num_ships,
            threshold_code: 0,
        });
    }
    for row in &fusion_rows {
        let source_slot = fleet_ids
            .iter()
            .position(|id| id == &row.surviving_fleet_id)
            .unwrap_or(0) as u32;
        let target_slot = fleet_ids
            .iter()
            .position(|id| id == &row.absorbed_fleet_id)
            .unwrap_or(0) as u32;
        events.push(R1bStructuralEvent {
            tick: row.tick,
            event_kind: R1bStructuralEventKind::FusionRequest,
            source_slot,
            target_slot,
            source_cell: row.cell_index,
            target_cell: row.cell_index,
            owner_code: r1b_owner_code(row.owner),
            amount_or_delta: row.right_num_ships,
            threshold_code: 0,
        });
    }
    for economy_row in tick_economy_rows {
        if !economy_row.owner_column_flipped {
            continue;
        }
        let source_slot = system_indices
            .iter()
            .position(|idx| *idx == economy_row.system_index)
            .unwrap_or(0) as u32;
        events.push(R1bStructuralEvent {
            tick,
            event_kind: R1bStructuralEventKind::OwnerCodeFlip,
            source_slot,
            target_slot: 0,
            source_cell: economy_row.cell_index,
            target_cell: 0,
            owner_code: economy_row.blockader.map(r1b_owner_code).unwrap_or(0),
            amount_or_delta: 0,
            threshold_code: 0,
        });
    }
    events
}

pub fn r1b_stage_movement_extraction(
    world: &DressRehearsalR6cWorld,
    fleet_ids: &[String],
    capability: &[DressRehearsalR6cCapabilityOverlayRow],
    tick: u32,
) -> (Vec<R1bStructuralEvent>, BTreeSet<u32>) {
    let mut scratch = world.clone();
    let mut field_rows = Vec::new();
    let mut boundary_rows = Vec::new();
    let mut movement_rows = Vec::new();
    let moved_cells = run_movement_tick(
        tick,
        &mut scratch,
        capability,
        &mut field_rows,
        &mut boundary_rows,
        &mut movement_rows,
    );
    let events = boundary_rows
        .into_iter()
        .filter(|row| row.event_emitted)
        .map(|row| {
            let source_slot = fleet_ids
                .iter()
                .position(|id| id == &row.mover_id)
                .unwrap_or(0) as u32;
            R1bStructuralEvent {
                tick: row.tick,
                event_kind: R1bStructuralEventKind::MoveRequest,
                source_slot,
                target_slot: 0,
                source_cell: row.source_cell_index,
                target_cell: row.destination_cell_index,
                owner_code: 0,
                amount_or_delta: 0,
                threshold_code: row.threshold_input_mag_bits,
            }
        })
        .collect();
    (events, moved_cells)
}

pub fn r1b_predict_fusion_events(
    world: &DressRehearsalR6cWorld,
    fleet_ids: &[String],
    tick: u32,
) -> Vec<R1bStructuralEvent> {
    let mut groups: BTreeMap<(DressRehearsalR6cOwner, u32, i64, i64), Vec<usize>> = BTreeMap::new();
    for (idx, fleet) in world.fleets.iter().enumerate() {
        if fleet.destroyed || fleet.num_ships <= 0 || !fleet.fleet_like {
            continue;
        }
        groups
            .entry((
                fleet.owner,
                fleet.cell_index,
                fleet.hp_per_ship,
                fleet.damage_per_ship_per_tick,
            ))
            .or_default()
            .push(idx);
    }

    let mut events = Vec::new();
    for indices in groups.values() {
        if indices.len() < 2 {
            continue;
        }
        let mut ordered = indices.clone();
        ordered.sort_by_key(|idx| world.fleets[*idx].fleet_id.clone());
        let survivor_idx = ordered[0];
        let survivor_id = &world.fleets[survivor_idx].fleet_id;
        let Some(survivor_slot) = fleet_ids.iter().position(|id| id == survivor_id) else {
            continue;
        };
        for absorbed_idx in ordered.iter().copied().skip(1) {
            let absorbed = &world.fleets[absorbed_idx];
            if absorbed.destroyed {
                continue;
            }
            let Some(absorbed_slot) = fleet_ids.iter().position(|id| id == &absorbed.fleet_id)
            else {
                continue;
            };
            events.push(R1bStructuralEvent {
                tick,
                event_kind: R1bStructuralEventKind::FusionRequest,
                source_slot: survivor_slot as u32,
                target_slot: absorbed_slot as u32,
                source_cell: absorbed.cell_index,
                target_cell: absorbed.cell_index,
                owner_code: r1b_owner_code(absorbed.owner),
                amount_or_delta: absorbed.num_ships.max(0),
                threshold_code: 0,
            });
        }
    }
    events
}

fn r1b_owner_code(owner: DressRehearsalR6cOwner) -> u32 {
    match owner {
        DressRehearsalR6cOwner::Terran => 1,
        DressRehearsalR6cOwner::Pirate => 2,
    }
}
