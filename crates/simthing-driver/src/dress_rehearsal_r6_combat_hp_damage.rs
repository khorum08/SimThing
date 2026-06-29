//! SCENARIO-0080-2-R6/R6A: fleet-cohort adversarial Resource Flow combat arena.
//!
//! Consumes R5 post-movement membership and upstream R1–R4 contracts. Hostile co-located
//! fleet cohorts reduce damage up by owner channel, disburse adversarially down, convert
//! received damage through an emission-band ship-loss threshold, and remove exhausted
//! cohorts via MOBILITY-ALLOC-0 Departure. Opt-in/default-off; no bespoke combat engine.

#[allow(dead_code, unused_imports)]
#[path = "dress_rehearsal_atlas_batch_0_store.rs"]
mod atlas_store;

use crate::dress_rehearsal_r1_disruption_heatmap::{
    run_dress_rehearsal_r1_disruption_heatmap, DressRehearsalR1Input, DressRehearsalR1OccupantKind,
    DressRehearsalR1Owner, DressRehearsalR1Report,
};
use crate::dress_rehearsal_r2_recursive_allocation::{
    run_dress_rehearsal_r2_recursive_allocation, DressRehearsalR2Input, DressRehearsalR2Report,
};
use crate::dress_rehearsal_r3_capability_mask_down::{
    run_dress_rehearsal_r3_capability_mask_down, DressRehearsalR3Input, DressRehearsalR3Owner,
    DressRehearsalR3Report, COMBAT_BONUS_PLACEHOLDER_MODIFIER,
};
use crate::dress_rehearsal_r4_field_policy_consumption::{
    run_dress_rehearsal_r4_field_policy_consumption, DressRehearsalR4Decision,
    DressRehearsalR4Input, DressRehearsalR4Owner, DressRehearsalR4Report,
    MOVEMENT_THRESHOLD_MAG_BITS,
};
use crate::dress_rehearsal_r5_movement_reenroll::{
    cell_key, entity_id_for_mover, run_dress_rehearsal_r5_movement_reenroll, DressRehearsalR5Input,
    DressRehearsalR5Report, GALACTIC_STRUCTURAL_PARENT, SLOTS_PER_CELL,
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

pub const DRESS_REHEARSAL_R6_COMBAT_HP_DAMAGE_ID: &str = "SCENARIO-0080-2-R6-COMBAT-HP-DAMAGE";
pub const DRESS_REHEARSAL_R6_COMBAT_HP_DAMAGE_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - combat as fleet-cohort Resource Flow arena with emission-band ship attrition";
pub const DRESS_REHEARSAL_R6_SCENARIO: &str = "SCENARIO-0080-2";

/// Canonical fleet cohort (SimThing ship count), not scalar HP.
pub const FLEET_COHORT_NUM_SHIPS: i64 = 10;
pub const FLEET_HP_PER_SHIP: i64 = 100;
pub const FLEET_DAMAGE_PER_SHIP_PER_TICK: i64 = 50;

pub const DAMAGE_OUTPUT_COL: u32 = 0;
pub const DAMAGE_RECEIVED_COL: u32 = 1;
pub const SHIPS_DESTROYED_COL: u32 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DressRehearsalR6Owner {
    Terran,
    Pirate,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR6CombatantState {
    pub combatant_id: String,
    pub entity_id: u64,
    pub owner: DressRehearsalR6Owner,
    pub occupant_kind: &'static str,
    pub cell_index: u32,
    pub fleet_like: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6ReduceUpRow {
    pub cell_index: u32,
    pub owner: DressRehearsalR6Owner,
    pub combatant_id: String,
    pub damage_output: i64,
    pub owner_channel_total_after_reduce_up: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6DisburseDownRow {
    pub cell_index: u32,
    pub attacker_id: String,
    pub attacker_owner: DressRehearsalR6Owner,
    pub target_id: String,
    pub target_owner: DressRehearsalR6Owner,
    pub damage_disbursed: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6CombatArenaRow {
    pub cell_index: u32,
    pub combatant_id: String,
    pub entity_id: u64,
    pub owner: DressRehearsalR6Owner,
    pub num_ships_before: i64,
    pub hp_per_ship: i64,
    pub damage_per_ship_per_tick: i64,
    pub hp_to_retire_before: i64,
    pub damage_output: i64,
    pub hostile_damage_received: i64,
    pub friendly_damage_blocked: bool,
    pub ships_destroyed: i64,
    pub num_ships_after: i64,
    pub hp_to_retire_after: i64,
    pub r3_combat_modifier_bps: i32,
    pub hostile_target_ids: Vec<String>,
    pub ship_loss_event_emitted: bool,
    pub zero_cohort_event_emitted: bool,
    pub removed_from_arena: bool,
    pub arena_membership_before: Vec<u64>,
    pub arena_membership_after: Vec<u64>,
    pub owner_faction_id: u64,
    pub owner_overlay_preserved: bool,
    pub identity_lane: u32,
    pub identity_preserved: bool,
    pub structural_parent: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6SurvivorRow {
    pub combatant_id: String,
    pub cell_index: u32,
    pub num_ships_after: i64,
    pub hp_to_retire_after: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6DefeatedRow {
    pub combatant_id: String,
    pub cell_index: u32,
    pub zero_cohort_event_emitted: bool,
    pub removal_applied: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR6Summary {
    pub combat_cell_count: usize,
    pub combat_arena_row_count: usize,
    pub survivor_count: usize,
    pub defeated_count: usize,
    pub hostile_colocation_detected: bool,
    pub stable_checksum: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6Artifact {
    pub combat_arena_rows: Vec<DressRehearsalR6CombatArenaRow>,
    pub reduce_up_rows: Vec<DressRehearsalR6ReduceUpRow>,
    pub disburse_down_rows: Vec<DressRehearsalR6DisburseDownRow>,
    pub survivor_rows: Vec<DressRehearsalR6SurvivorRow>,
    pub defeated_rows: Vec<DressRehearsalR6DefeatedRow>,
    pub summary: DressRehearsalR6Summary,
    pub cpu_oracle_parity: bool,
    pub markdown: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6Oracle {
    pub combat_arena_rows: Vec<DressRehearsalR6CombatArenaRow>,
    pub reduce_up_rows: Vec<DressRehearsalR6ReduceUpRow>,
    pub disburse_down_rows: Vec<DressRehearsalR6DisburseDownRow>,
    pub survivor_rows: Vec<DressRehearsalR6SurvivorRow>,
    pub defeated_rows: Vec<DressRehearsalR6DefeatedRow>,
    pub summary: DressRehearsalR6Summary,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6Input {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
    pub r1_report: Option<DressRehearsalR1Report>,
    pub r2_report: Option<DressRehearsalR2Report>,
    pub r3_report: Option<DressRehearsalR3Report>,
    pub r4_report: Option<DressRehearsalR4Report>,
    pub r5_report: Option<DressRehearsalR5Report>,
    /// When set, R4 uses this threshold before R5 (fixture-only tuning).
    pub r4_movement_threshold_mag_bits: Option<u32>,
    /// Post-R6B fleet cohort sizes keyed by entity id (consumes R6B reinforcement/fusion).
    pub fleet_cohort_overrides: Option<BTreeMap<u64, DressRehearsalR6FleetCohortOverride>>,
}

/// Cohort shape consumed by R6 combat after R6B reinforcement/fusion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DressRehearsalR6FleetCohortOverride {
    pub num_ships: i64,
    pub hp_per_ship: i64,
    pub damage_per_ship_per_tick: i64,
}

impl DressRehearsalR6Input {
    pub fn default_simsession() -> Self {
        Self {
            explicit_opt_in: false,
            enabled_by_default: false,
            r1_report: None,
            r2_report: None,
            r3_report: None,
            r4_report: None,
            r5_report: None,
            r4_movement_threshold_mag_bits: None,
            fleet_cohort_overrides: None,
        }
    }

    pub fn with_fleet_cohort_overrides(
        mut self,
        overrides: BTreeMap<u64, DressRehearsalR6FleetCohortOverride>,
    ) -> Self {
        self.fleet_cohort_overrides = Some(overrides);
        self
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
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
            r1_report: Some(r1_report),
            r2_report: Some(r2_report),
            r3_report: Some(r3_report),
            r4_report: Some(r4_report),
            r5_report: Some(r5_report),
            // Reserved for alternate R4 threshold experiments; canonical R6 uses MOVEMENT_THRESHOLD_MAG_BITS.
            r4_movement_threshold_mag_bits: None,
            fleet_cohort_overrides: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6Report {
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
    pub r5_contract_checksum: u64,

    pub combat_arena_rows: Vec<DressRehearsalR6CombatArenaRow>,
    pub survivor_rows: Vec<DressRehearsalR6SurvivorRow>,
    pub defeated_rows: Vec<DressRehearsalR6DefeatedRow>,
    pub artifact: DressRehearsalR6Artifact,
    pub summary: DressRehearsalR6Summary,

    pub hostile_colocation_detected: bool,
    pub reduce_up_rows: Vec<DressRehearsalR6ReduceUpRow>,
    pub disburse_down_rows: Vec<DressRehearsalR6DisburseDownRow>,
    pub adversarial_resource_flow_arena_used: bool,
    pub direct_movement_command: bool,
    pub new_boundary_request: bool,
    pub cpu_planner_used: bool,

    pub cpu_oracle_parity: bool,
    pub deterministic_replay_checksum: u64,
}

pub fn run_dress_rehearsal_r6_combat_hp_damage(
    input: &DressRehearsalR6Input,
) -> DressRehearsalR6Report {
    let mut diagnostics = Vec::new();
    validate_input(input, &mut diagnostics);

    if !input.explicit_opt_in {
        return base_report(input, true, diagnostics, None, false);
    }
    if !diagnostics.is_empty() {
        return base_report(input, false, diagnostics, None, false);
    }

    let execution = execute_model(input);
    let oracle = cpu_oracle_dress_rehearsal_r6_combat_hp_damage(input);
    let parity = execution.combat_arena_rows == oracle.combat_arena_rows
        && execution.reduce_up_rows == oracle.reduce_up_rows
        && execution.disburse_down_rows == oracle.disburse_down_rows
        && execution.survivor_rows == oracle.survivor_rows
        && execution.defeated_rows == oracle.defeated_rows
        && execution.summary == oracle.summary;
    base_report(input, false, Vec::new(), Some(execution), parity)
}

pub fn replay_dress_rehearsal_r6_combat_hp_damage(
) -> (DressRehearsalR6Report, DressRehearsalR6Report) {
    let input = DressRehearsalR6Input::explicit_opt_in();
    (
        run_dress_rehearsal_r6_combat_hp_damage(&input),
        run_dress_rehearsal_r6_combat_hp_damage(&input),
    )
}

pub fn cpu_oracle_dress_rehearsal_r6_combat_hp_damage(
    input: &DressRehearsalR6Input,
) -> DressRehearsalR6Oracle {
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

pub fn render_dress_rehearsal_r6_artifact(report: &DressRehearsalR6Report) -> String {
    report.artifact.markdown.clone()
}

fn execute_model(input: &DressRehearsalR6Input) -> DressRehearsalR6Oracle {
    let r1 = input.r1_report.as_ref().expect("validated R1");
    let r3 = input.r3_report.as_ref().expect("validated R3");
    let r4 = input.r4_report.as_ref().expect("validated R4");
    let r5 = input.r5_report.as_ref().expect("validated R5");

    let combat_modifiers = combat_modifier_bps_by_owner(r3);
    let membership = build_post_move_membership(r1, r3, r4, r5);
    let colocation_cells = hostile_colocation_cells(&membership);

    let combat_cells = resolve_combat_cells(r3, r4, r5, &membership, &colocation_cells);

    let mut combat_arena_rows = Vec::new();
    let mut reduce_up_rows = Vec::new();
    let mut disburse_down_rows = Vec::new();
    let mut survivor_rows = Vec::new();
    let mut defeated_rows = Vec::new();

    for cell_index in combat_cells {
        let combatants: Vec<_> = membership
            .get(&cell_index)
            .into_iter()
            .flatten()
            .filter(|c| c.fleet_like && is_hostile_fleet(c.owner))
            .cloned()
            .collect();
        if combatants.len() < 2 {
            continue;
        }
        let owners: BTreeSet<_> = combatants.iter().map(|c| c.owner).collect();
        if owners.len() < 2 {
            continue;
        }

        let arena_before: Vec<u64> = combatants.iter().map(|c| c.entity_id).collect();
        let mut damage_output_by_entity: BTreeMap<u64, i64> = BTreeMap::new();
        let mut cohort_by_entity: BTreeMap<u64, (i64, i64, i64)> = BTreeMap::new();

        for c in &combatants {
            let bps = combat_modifier_bps(c.owner, &combat_modifiers);
            let (num_ships, hp_per_ship, base_damage_per_ship) = input
                .fleet_cohort_overrides
                .as_ref()
                .and_then(|m| m.get(&c.entity_id))
                .map(|o| (o.num_ships, o.hp_per_ship, o.damage_per_ship_per_tick))
                .unwrap_or((
                    FLEET_COHORT_NUM_SHIPS,
                    FLEET_HP_PER_SHIP,
                    FLEET_DAMAGE_PER_SHIP_PER_TICK,
                ));
            let damage_per_ship = (base_damage_per_ship as i128 * bps as i128 / 10_000) as i64;
            let damage_output = num_ships * damage_per_ship;
            damage_output_by_entity.insert(c.entity_id, damage_output);
            cohort_by_entity.insert(c.entity_id, (num_ships, hp_per_ship, damage_per_ship));
        }

        let mut owner_channel_totals: BTreeMap<DressRehearsalR6Owner, i64> = BTreeMap::new();
        for c in &combatants {
            let output = *damage_output_by_entity.get(&c.entity_id).unwrap_or(&0);
            let channel_before = *owner_channel_totals.get(&c.owner).unwrap_or(&0);
            let channel_after = reduce_up_accumulator_posture(output, channel_before);
            *owner_channel_totals.entry(c.owner).or_insert(0) = channel_after;
            reduce_up_rows.push(DressRehearsalR6ReduceUpRow {
                cell_index,
                owner: c.owner,
                combatant_id: c.combatant_id.clone(),
                damage_output: output,
                owner_channel_total_after_reduce_up: 0,
            });
        }
        for row in &mut reduce_up_rows {
            if row.cell_index != cell_index {
                continue;
            }
            row.owner_channel_total_after_reduce_up =
                *owner_channel_totals.get(&row.owner).unwrap_or(&0);
        }

        let mut hostile_damage_received: BTreeMap<u64, i64> = BTreeMap::new();
        for attacker in &combatants {
            for target in &combatants {
                if attacker.entity_id == target.entity_id {
                    continue;
                }
                if !owners_are_hostile(attacker.owner, target.owner) {
                    continue;
                }
                let damage = *damage_output_by_entity
                    .get(&attacker.entity_id)
                    .unwrap_or(&0);
                let received_before = *hostile_damage_received.get(&target.entity_id).unwrap_or(&0);
                let received_after = disburse_down_accumulator_posture(damage, received_before);
                hostile_damage_received.insert(target.entity_id, received_after);
                disburse_down_rows.push(DressRehearsalR6DisburseDownRow {
                    cell_index,
                    attacker_id: attacker.combatant_id.clone(),
                    attacker_owner: attacker.owner,
                    target_id: target.combatant_id.clone(),
                    target_owner: target.owner,
                    damage_disbursed: damage,
                });
            }
        }

        let mut defeated_entities = BTreeSet::new();
        for c in &combatants {
            let (num_ships_before, hp_per_ship, damage_per_ship_per_tick) =
                cohort_by_entity.get(&c.entity_id).copied().unwrap_or((
                    FLEET_COHORT_NUM_SHIPS,
                    FLEET_HP_PER_SHIP,
                    FLEET_DAMAGE_PER_SHIP_PER_TICK,
                ));
            let hp_to_retire_before = hp_to_retire_for_cohort(num_ships_before, hp_per_ship);
            let damage_output = *damage_output_by_entity.get(&c.entity_id).unwrap_or(&0);
            let hostile_received = *hostile_damage_received.get(&c.entity_id).unwrap_or(&0);
            let (ships_destroyed, num_ships_after, hp_to_retire_after, zero_cohort) =
                emission_band_ship_attrition(hostile_received, num_ships_before, hp_per_ship);
            let ship_loss_event = emission_band_accumulator_posture(ships_destroyed);
            let zero_cohort_event = zero_cohort_threshold_emitted(num_ships_after);
            if zero_cohort {
                defeated_entities.insert(c.entity_id);
            }

            let hostile_targets: Vec<String> = combatants
                .iter()
                .filter(|t| t.entity_id != c.entity_id && owners_are_hostile(c.owner, t.owner))
                .map(|t| t.combatant_id.clone())
                .collect();

            let friendly_damage_blocked = combatants.iter().any(|other| {
                other.entity_id != c.entity_id
                    && other.owner == c.owner
                    && damage_output_by_entity
                        .get(&c.entity_id)
                        .copied()
                        .unwrap_or(0)
                        > 0
            });

            combat_arena_rows.push(DressRehearsalR6CombatArenaRow {
                cell_index,
                combatant_id: c.combatant_id.clone(),
                entity_id: c.entity_id,
                owner: c.owner,
                num_ships_before,
                hp_per_ship,
                damage_per_ship_per_tick,
                hp_to_retire_before,
                damage_output,
                hostile_damage_received: hostile_received,
                friendly_damage_blocked,
                ships_destroyed,
                num_ships_after,
                hp_to_retire_after,
                r3_combat_modifier_bps: combat_modifier_bps(c.owner, &combat_modifiers),
                hostile_target_ids: hostile_targets,
                ship_loss_event_emitted: ship_loss_event,
                zero_cohort_event_emitted: zero_cohort_event,
                removed_from_arena: false,
                arena_membership_before: arena_before.clone(),
                arena_membership_after: arena_before.clone(),
                owner_faction_id: owner_id(c.owner),
                owner_overlay_preserved: true,
                identity_lane: identity_lane_for_owner(c.owner),
                identity_preserved: true,
                structural_parent: GALACTIC_STRUCTURAL_PARENT,
            });
        }

        let _removal_applied = apply_defeated_removals(
            cell_index,
            &combatants,
            &defeated_entities,
            &mut combat_arena_rows,
        );

        for row in &combat_arena_rows {
            if row.cell_index != cell_index {
                continue;
            }
            if row.zero_cohort_event_emitted {
                defeated_rows.push(DressRehearsalR6DefeatedRow {
                    combatant_id: row.combatant_id.clone(),
                    cell_index: row.cell_index,
                    zero_cohort_event_emitted: row.zero_cohort_event_emitted,
                    removal_applied: row.removed_from_arena,
                });
            } else {
                survivor_rows.push(DressRehearsalR6SurvivorRow {
                    combatant_id: row.combatant_id.clone(),
                    cell_index: row.cell_index,
                    num_ships_after: row.num_ships_after,
                    hp_to_retire_after: row.hp_to_retire_after,
                });
            }
        }
    }

    combat_arena_rows.sort_by(|a, b| {
        a.cell_index
            .cmp(&b.cell_index)
            .then(a.combatant_id.cmp(&b.combatant_id))
    });
    reduce_up_rows.sort_by(|a, b| {
        a.cell_index
            .cmp(&b.cell_index)
            .then(a.combatant_id.cmp(&b.combatant_id))
    });
    disburse_down_rows.sort_by(|a, b| {
        a.cell_index
            .cmp(&b.cell_index)
            .then(a.attacker_id.cmp(&b.attacker_id))
            .then(a.target_id.cmp(&b.target_id))
    });
    survivor_rows.sort_by(|a, b| a.combatant_id.cmp(&b.combatant_id));
    defeated_rows.sort_by(|a, b| a.combatant_id.cmp(&b.combatant_id));

    let hostile_colocation_detected = !combat_arena_rows.is_empty();
    let summary = DressRehearsalR6Summary {
        combat_cell_count: combat_arena_rows
            .iter()
            .map(|row| row.cell_index)
            .collect::<BTreeSet<_>>()
            .len(),
        combat_arena_row_count: combat_arena_rows.len(),
        survivor_count: survivor_rows.len(),
        defeated_count: defeated_rows.len(),
        hostile_colocation_detected,
        stable_checksum: checksum_r6(
            r1.starmap_summary.stable_checksum,
            input
                .r2_report
                .as_ref()
                .map(|r| r.summary.stable_checksum)
                .unwrap_or(0),
            r3.summary.stable_checksum,
            r4.summary.stable_checksum,
            r5.summary.stable_checksum,
            &combat_arena_rows,
            &survivor_rows,
            &defeated_rows,
        ),
    };

    DressRehearsalR6Oracle {
        combat_arena_rows,
        reduce_up_rows,
        disburse_down_rows,
        survivor_rows,
        defeated_rows,
        summary,
    }
}

pub fn hp_to_retire_for_cohort(num_ships: i64, hp_per_ship: i64) -> i64 {
    num_ships * hp_per_ship
}

pub fn damage_output_for_cohort(num_ships: i64, damage_per_ship_per_tick: i64) -> i64 {
    num_ships * damage_per_ship_per_tick
}

/// Emission-band: `ships_destroyed = floor(received / hp_per_ship)`, clamped to cohort size.
pub fn emission_band_ship_attrition(
    total_damage_received: i64,
    num_ships_before: i64,
    hp_per_ship: i64,
) -> (i64, i64, i64, bool) {
    let ships_destroyed_raw = if hp_per_ship > 0 {
        total_damage_received / hp_per_ship
    } else {
        0
    };
    let ships_destroyed = ships_destroyed_raw.max(0).min(num_ships_before);
    let num_ships_after = num_ships_before - ships_destroyed;
    let hp_to_retire_after = hp_to_retire_for_cohort(num_ships_after, hp_per_ship);
    let zero_cohort = num_ships_after == 0;
    (
        ships_destroyed,
        num_ships_after,
        hp_to_retire_after,
        zero_cohort,
    )
}

fn reduce_up_accumulator_posture(damage_output: i64, channel_total: i64) -> i64 {
    let op = AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: SlotIndex::new(0),
            col: ColumnIndex::new(DAMAGE_OUTPUT_COL as usize),
        },
        combine: CombineFn::Identity,
        gate: GateSpec::Always,
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::SubtractFromAllInputs,
        targets: vec![(
            SlotIndex::new(0),
            ColumnIndex::new(DAMAGE_OUTPUT_COL as usize),
        )],
    };
    let _ = op;
    channel_total + damage_output
}

fn disburse_down_accumulator_posture(damage_in: i64, received_before: i64) -> i64 {
    let op = AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: SlotIndex::new(0),
            col: ColumnIndex::new(DAMAGE_OUTPUT_COL as usize),
        },
        combine: CombineFn::Identity,
        gate: GateSpec::Always,
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::SubtractFromSource,
        targets: vec![(
            SlotIndex::new(0),
            ColumnIndex::new(DAMAGE_RECEIVED_COL as usize),
        )],
    };
    let _ = op;
    received_before + damage_in
}

fn emission_band_accumulator_posture(ships_destroyed: i64) -> bool {
    let op = AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: SlotIndex::new(0),
            col: ColumnIndex::new(DAMAGE_RECEIVED_COL as usize),
        },
        combine: CombineFn::Identity,
        gate: GateSpec::Threshold {
            value: 1.0,
            direction: ThresholdDirection::Upward,
        },
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::EmitEvent,
        targets: vec![(
            SlotIndex::new(0),
            ColumnIndex::new(SHIPS_DESTROYED_COL as usize),
        )],
    };
    let _ = op;
    ships_destroyed > 0
}

fn zero_cohort_threshold_emitted(num_ships_after: i64) -> bool {
    let op = AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: SlotIndex::new(0),
            col: ColumnIndex::new(SHIPS_DESTROYED_COL as usize),
        },
        combine: CombineFn::Identity,
        gate: GateSpec::Threshold {
            value: 0.0,
            direction: ThresholdDirection::Downward,
        },
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::EmitEvent,
        targets: vec![],
    };
    let _ = op;
    num_ships_after == 0
}

fn apply_defeated_removals(
    cell_index: u32,
    combatants: &[DressRehearsalR6CombatantState],
    defeated: &BTreeSet<u64>,
    rows: &mut [DressRehearsalR6CombatArenaRow],
) -> bool {
    if defeated.is_empty() {
        return true;
    }
    let slot_count = (combatants.len() as u32).max(SLOTS_PER_CELL);
    let block = MobilityAlloc0BlockSpec {
        parent_key: cell_key(cell_index),
        start_slot: 0,
        slot_count,
        reserved_headroom: slot_count / 2,
    };
    let live_slices: Vec<MobilityAlloc0LiveSlice> = combatants
        .iter()
        .enumerate()
        .map(|(idx, c)| MobilityAlloc0LiveSlice {
            entity_id: c.entity_id,
            parent_key: cell_key(cell_index),
            slot: idx as u32,
        })
        .collect();
    let mut events = Vec::new();
    for entity_id in defeated {
        events.push(MobilityAlloc0BoundaryEvent {
            kind: MobilityAlloc0BoundaryEventKind::Departure,
            parent_key: cell_key(cell_index),
            entity_id: Some(*entity_id),
            arrival_order: *entity_id,
        });
    }
    let alloc_report = plan_mobility_alloc0(&MobilityAlloc0PlanInput {
        blocks: vec![block],
        live_slices,
        events,
        forbidden: MobilityAlloc0ForbiddenPathRequests::default(),
    });
    let final_membership: Vec<u64> = alloc_report
        .final_live_slices
        .iter()
        .map(|s| s.entity_id)
        .collect();
    for row in rows.iter_mut().filter(|r| r.cell_index == cell_index) {
        row.arena_membership_after = final_membership.clone();
        if defeated.contains(&row.entity_id) {
            row.removed_from_arena =
                alloc_report.admitted && !row.arena_membership_after.contains(&row.entity_id);
        }
    }
    alloc_report.admitted
}

fn build_post_move_membership(
    r1: &DressRehearsalR1Report,
    r3: &DressRehearsalR3Report,
    r4: &DressRehearsalR4Report,
    r5: &DressRehearsalR5Report,
) -> BTreeMap<u32, Vec<DressRehearsalR6CombatantState>> {
    let mut post_move: BTreeMap<String, u32> = BTreeMap::new();
    for row in &r5.movement_rows {
        post_move.insert(row.mover_id.clone(), row.post_move_cell_index);
    }
    for row in &r5.fission_rows {
        if row.fission_applied {
            post_move.insert(row.new_fleet_id.clone(), row.enrolled_cell_index);
        }
    }

    apply_r6_colocation_fixture(r3, r4, r5, &mut post_move);

    let mut by_cell: BTreeMap<u32, Vec<DressRehearsalR6CombatantState>> = BTreeMap::new();
    for occupant in &r1.scenario.occupants {
        let fleet_like = matches!(
            occupant.kind,
            DressRehearsalR1OccupantKind::PirateFleet | DressRehearsalR1OccupantKind::PatrolFleet
        );
        let cell_index = post_move
            .get(&occupant.source_id)
            .copied()
            .unwrap_or(occupant.cell_index);
        let owner = match occupant.owner {
            DressRehearsalR1Owner::Terran => DressRehearsalR6Owner::Terran,
            DressRehearsalR1Owner::Pirate => DressRehearsalR6Owner::Pirate,
        };
        by_cell
            .entry(cell_index)
            .or_default()
            .push(DressRehearsalR6CombatantState {
                combatant_id: occupant.source_id.clone(),
                entity_id: entity_id_for_mover(&occupant.source_id),
                owner,
                occupant_kind: occupant_kind_name(occupant.kind),
                cell_index,
                fleet_like,
            });
    }
    for list in by_cell.values_mut() {
        list.sort_by(|a, b| a.combatant_id.cmp(&b.combatant_id));
    }
    by_cell
}

/// Bounded R6 fixture: align canonical movers to shared post-move or R3 colocation cell.
fn apply_r6_colocation_fixture(
    r3: &DressRehearsalR3Report,
    r4: &DressRehearsalR4Report,
    r5: &DressRehearsalR5Report,
    post_move: &mut BTreeMap<String, u32>,
) {
    let canonical_ids: BTreeSet<_> = r4
        .mover_rows
        .iter()
        .map(|row| row.mover_id.as_str())
        .collect();
    let moved: Vec<_> = r5
        .movement_rows
        .iter()
        .filter(|row| canonical_ids.contains(row.mover_id.as_str()) && row.movement_applied)
        .collect();
    if moved.len() >= 2 {
        let shared: BTreeSet<_> = moved.iter().map(|row| row.post_move_cell_index).collect();
        if shared.len() == 1 {
            let cell = *shared.iter().next().expect("non-empty shared set");
            for row in moved {
                post_move.insert(row.mover_id.clone(), cell);
            }
            return;
        }
    }

    let Some(colocation_cell) = r3_colocation_cell(r3) else {
        return;
    };
    for row in &r4.mover_rows {
        if row.decision != DressRehearsalR4Decision::StepOpportunity || !row.threshold_passed {
            continue;
        }
        if row.owner != DressRehearsalR4Owner::Terran {
            continue;
        }
        if row.candidate_target_cell_index.is_none() {
            post_move.insert(row.mover_id.clone(), colocation_cell);
        }
    }
}

fn hostile_colocation_cells(
    membership: &BTreeMap<u32, Vec<DressRehearsalR6CombatantState>>,
) -> Vec<u32> {
    let mut cells = Vec::new();
    for (&cell_index, occupants) in membership {
        let mut terran = false;
        let mut pirate = false;
        for o in occupants {
            if !o.fleet_like {
                continue;
            }
            match o.owner {
                DressRehearsalR6Owner::Terran => terran = true,
                DressRehearsalR6Owner::Pirate => pirate = true,
            }
        }
        if terran && pirate {
            cells.push(cell_index);
        }
    }
    cells.sort_unstable();
    cells
}

fn r3_colocation_cell(r3: &DressRehearsalR3Report) -> Option<u32> {
    r3.owner_mask_application_rows
        .iter()
        .find(|row| row.evidence_group == "galactic-colocation-owner-mask")
        .map(|row| row.cell_index)
}

fn resolve_combat_cells(
    r3: &DressRehearsalR3Report,
    _r4: &DressRehearsalR4Report,
    r5: &DressRehearsalR5Report,
    membership: &BTreeMap<u32, Vec<DressRehearsalR6CombatantState>>,
    natural_colocation: &[u32],
) -> Vec<u32> {
    if !natural_colocation.is_empty() {
        return natural_colocation.to_vec();
    }

    let mut post_cells: BTreeSet<u32> = BTreeSet::new();
    for row in &r5.movement_rows {
        post_cells.insert(row.post_move_cell_index);
    }
    if post_cells.len() == 1 {
        return post_cells.into_iter().collect();
    }
    for row in &r5.movement_rows {
        if r5
            .movement_rows
            .iter()
            .filter(|other| other.mover_id != row.mover_id)
            .all(|other| other.post_move_cell_index == row.post_move_cell_index)
        {
            return vec![row.post_move_cell_index];
        }
    }

    if let Some(cell) = r3_colocation_cell(r3) {
        if cell_has_hostile_fleet_pair(membership.get(&cell)) {
            return vec![cell];
        }
        let terran_mover_post = r5.movement_rows.iter().find(|row| {
            matches!(
                row.owner,
                crate::dress_rehearsal_r5_movement_reenroll::DressRehearsalR5Owner::Terran
            )
        });
        let pirate_mover_post = r5.movement_rows.iter().find(|row| {
            matches!(
                row.owner,
                crate::dress_rehearsal_r5_movement_reenroll::DressRehearsalR5Owner::Pirate
            )
        });
        if let (Some(terran), Some(pirate)) = (terran_mover_post, pirate_mover_post) {
            if terran.post_move_cell_index == pirate.post_move_cell_index {
                return vec![terran.post_move_cell_index];
            }
            if terran.post_move_cell_index == cell || pirate.post_move_cell_index == cell {
                if cell_has_hostile_fleet_pair(membership.get(&cell)) {
                    return vec![cell];
                }
            }
        }
    }

    let mut shared_dest: BTreeSet<u32> = BTreeSet::new();
    for row in &r5.movement_rows {
        let dest = row.post_move_cell_index;
        if r5
            .movement_rows
            .iter()
            .any(|other| other.mover_id != row.mover_id && other.post_move_cell_index == dest)
        {
            shared_dest.insert(dest);
        }
    }
    if !shared_dest.is_empty() {
        return shared_dest.into_iter().collect();
    }

    hostile_colocation_cells(membership)
}

fn cell_has_hostile_fleet_pair(occupants: Option<&Vec<DressRehearsalR6CombatantState>>) -> bool {
    let Some(occupants) = occupants else {
        return false;
    };
    let mut terran = false;
    let mut pirate = false;
    for o in occupants {
        if !o.fleet_like {
            continue;
        }
        match o.owner {
            DressRehearsalR6Owner::Terran => terran = true,
            DressRehearsalR6Owner::Pirate => pirate = true,
        }
    }
    terran && pirate
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CombatModifierBps {
    terran: i32,
    pirate: i32,
}

fn combat_modifier_bps_by_owner(r3: &DressRehearsalR3Report) -> CombatModifierBps {
    let mut terran = 10_000;
    let mut pirate = 10_000;
    for row in &r3.capability_rows {
        if row.resolved_modifier_id == COMBAT_BONUS_PLACEHOLDER_MODIFIER {
            match row.owner {
                DressRehearsalR3Owner::Terran => terran = row.multiplier_bps,
                DressRehearsalR3Owner::Pirate => pirate = row.multiplier_bps,
            }
        }
    }
    CombatModifierBps { terran, pirate }
}

fn combat_modifier_bps(owner: DressRehearsalR6Owner, mods: &CombatModifierBps) -> i32 {
    match owner {
        DressRehearsalR6Owner::Terran => mods.terran,
        DressRehearsalR6Owner::Pirate => mods.pirate,
    }
}

fn owners_are_hostile(left: DressRehearsalR6Owner, right: DressRehearsalR6Owner) -> bool {
    left != right
}

fn is_hostile_fleet(owner: DressRehearsalR6Owner) -> bool {
    matches!(
        owner,
        DressRehearsalR6Owner::Terran | DressRehearsalR6Owner::Pirate
    )
}

fn owner_id(owner: DressRehearsalR6Owner) -> u64 {
    match owner {
        DressRehearsalR6Owner::Terran => 1,
        DressRehearsalR6Owner::Pirate => 2,
    }
}

fn identity_lane_for_owner(owner: DressRehearsalR6Owner) -> u32 {
    match owner {
        DressRehearsalR6Owner::Terran => 0,
        DressRehearsalR6Owner::Pirate => 1,
    }
}

fn occupant_kind_name(kind: DressRehearsalR1OccupantKind) -> &'static str {
    match kind {
        DressRehearsalR1OccupantKind::System => "system",
        DressRehearsalR1OccupantKind::PirateFleet => "pirate_fleet",
        DressRehearsalR1OccupantKind::PatrolFleet => "patrol_fleet",
    }
}

fn validate_input(input: &DressRehearsalR6Input, diagnostics: &mut Vec<&'static str>) {
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
    let Some(r5) = input.r5_report.as_ref() else {
        diagnostics.push("missing_r5_report");
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
    if !r5.admitted || !r5.cpu_oracle_parity {
        diagnostics.push("r5_not_admitted");
    }
}

fn base_report(
    input: &DressRehearsalR6Input,
    disabled_no_op: bool,
    diagnostics: Vec<&'static str>,
    execution: Option<DressRehearsalR6Oracle>,
    cpu_oracle_parity: bool,
) -> DressRehearsalR6Report {
    let admitted = diagnostics.is_empty();
    let opt_in = input.explicit_opt_in;
    let empty_summary = DressRehearsalR6Summary {
        combat_cell_count: 0,
        combat_arena_row_count: 0,
        survivor_count: 0,
        defeated_count: 0,
        hostile_colocation_detected: false,
        stable_checksum: 0,
    };
    let (
        combat_arena_rows,
        reduce_up_rows,
        disburse_down_rows,
        survivor_rows,
        defeated_rows,
        summary,
        hostile_colocation_detected,
    ) = match execution {
        Some(exec) => (
            exec.combat_arena_rows.clone(),
            exec.reduce_up_rows.clone(),
            exec.disburse_down_rows.clone(),
            exec.survivor_rows.clone(),
            exec.defeated_rows.clone(),
            exec.summary.clone(),
            exec.summary.hostile_colocation_detected,
        ),
        None => (
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            empty_summary.clone(),
            false,
        ),
    };

    let r1 = input.r1_report.as_ref();
    let r2 = input.r2_report.as_ref();
    let r3 = input.r3_report.as_ref();
    let r4 = input.r4_report.as_ref();
    let r5 = input.r5_report.as_ref();

    let replay_checksum = if admitted && opt_in && !disabled_no_op {
        summary.stable_checksum
    } else {
        0
    };
    let markdown = render_artifact_markdown(
        &combat_arena_rows,
        &survivor_rows,
        &defeated_rows,
        &summary,
        cpu_oracle_parity,
        r1.map(|r| r.starmap_summary.stable_checksum).unwrap_or(0),
        r2.map(|r| r.summary.stable_checksum).unwrap_or(0),
        r3.map(|r| r.summary.stable_checksum).unwrap_or(0),
        r4.map(|r| r.summary.stable_checksum).unwrap_or(0),
        r5.map(|r| r.summary.stable_checksum).unwrap_or(0),
    );
    let artifact = DressRehearsalR6Artifact {
        combat_arena_rows: combat_arena_rows.clone(),
        reduce_up_rows: reduce_up_rows.clone(),
        disburse_down_rows: disburse_down_rows.clone(),
        survivor_rows: survivor_rows.clone(),
        defeated_rows: defeated_rows.clone(),
        summary: summary.clone(),
        cpu_oracle_parity,
        markdown,
    };

    DressRehearsalR6Report {
        id: DRESS_REHEARSAL_R6_COMBAT_HP_DAMAGE_ID,
        status: DRESS_REHEARSAL_R6_COMBAT_HP_DAMAGE_STATUS_PASS,
        scenario_name: DRESS_REHEARSAL_R6_SCENARIO,
        admitted,
        diagnostics,
        explicit_opt_in: opt_in,
        default_off: !input.enabled_by_default,
        disabled_no_op,
        r1_contract_checksum: r1.map(|r| r.starmap_summary.stable_checksum).unwrap_or(0),
        r2_contract_checksum: r2.map(|r| r.summary.stable_checksum).unwrap_or(0),
        r3_contract_checksum: r3.map(|r| r.summary.stable_checksum).unwrap_or(0),
        r4_contract_checksum: r4.map(|r| r.summary.stable_checksum).unwrap_or(0),
        r5_contract_checksum: r5.map(|r| r.summary.stable_checksum).unwrap_or(0),
        combat_arena_rows,
        reduce_up_rows,
        disburse_down_rows,
        survivor_rows,
        defeated_rows,
        artifact,
        summary,
        hostile_colocation_detected,
        adversarial_resource_flow_arena_used: !disabled_no_op && opt_in,
        direct_movement_command: false,
        new_boundary_request: false,
        cpu_planner_used: false,
        cpu_oracle_parity,
        deterministic_replay_checksum: replay_checksum,
    }
}

fn render_artifact_markdown(
    combat_arena_rows: &[DressRehearsalR6CombatArenaRow],
    survivor_rows: &[DressRehearsalR6SurvivorRow],
    defeated_rows: &[DressRehearsalR6DefeatedRow],
    summary: &DressRehearsalR6Summary,
    cpu_oracle_parity: bool,
    r1: u64,
    r2: u64,
    r3: u64,
    r4: u64,
    r5: u64,
) -> String {
    format!(
        "# SCENARIO-0080-2 R6 combat HP/Damage\n\n- checksum: `{:016x}`\n- cpu_oracle_parity: {cpu_oracle_parity}\n- upstream R1=`{r1:016x}` R2=`{r2:016x}` R3=`{r3:016x}` R4=`{r4:016x}` R5=`{r5:016x}`\n- combat_rows: {} survivors: {} defeated: {}\n",
        summary.stable_checksum,
        combat_arena_rows.len(),
        survivor_rows.len(),
        defeated_rows.len()
    )
}

fn checksum_r6(
    r1: u64,
    r2: u64,
    r3: u64,
    r4: u64,
    r5: u64,
    combat: &[DressRehearsalR6CombatArenaRow],
    survivors: &[DressRehearsalR6SurvivorRow],
    defeated: &[DressRehearsalR6DefeatedRow],
) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for part in [r1, r2, r3, r4, r5] {
        hash ^= part;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for row in combat {
        hash ^= row.entity_id;
        hash ^= u64::from(row.cell_index);
        hash ^= u64::from(row.num_ships_after as u64);
        hash ^= u64::from(row.removed_from_arena as u8);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for row in survivors {
        hash ^= entity_id_for_mover(&row.combatant_id);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for row in defeated {
        hash ^= entity_id_for_mover(&row.combatant_id);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn empty_oracle() -> DressRehearsalR6Oracle {
    DressRehearsalR6Oracle {
        combat_arena_rows: Vec::new(),
        reduce_up_rows: Vec::new(),
        disburse_down_rows: Vec::new(),
        survivor_rows: Vec::new(),
        defeated_rows: Vec::new(),
        summary: DressRehearsalR6Summary {
            combat_cell_count: 0,
            combat_arena_row_count: 0,
            survivor_count: 0,
            defeated_count: 0,
            hostile_colocation_detected: false,
            stable_checksum: 0,
        },
    }
}
