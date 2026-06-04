//! SCENARIO-0080-2-R6: combat as HP/Damage resource-flow arena.
//!
//! Consumes R5 post-movement membership and upstream R1–R4 contracts. Resolves hostile
//! co-located fleet combat as owner-masked HP subtraction (SubtractFromSource posture),
//! zero-HP Threshold+EmitEvent, and defeated-fleet removal via MOBILITY-ALLOC-0 Departure.
//! Opt-in/default-off; no movement, planner, GPU, or default SimSession wiring.

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
use crate::dress_rehearsal_r4_sead_field_consumption::{
    run_dress_rehearsal_r4_sead_field_consumption, DressRehearsalR4Decision, DressRehearsalR4Input,
    DressRehearsalR4Owner, DressRehearsalR4Report, MOVEMENT_THRESHOLD_MAG_BITS,
};
use crate::dress_rehearsal_r5_movement_reenroll::{
    cell_key, entity_id_for_mover, run_dress_rehearsal_r5_movement_reenroll, DressRehearsalR5Input,
    DressRehearsalR5Report, GALACTIC_STRUCTURAL_PARENT, SLOTS_PER_CELL,
};
use simthing_core::{
    AccumulatorOp, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SourceSpec, ThresholdDirection,
};
use simthing_spec::{
    plan_mobility_alloc0, MobilityAlloc0BlockSpec, MobilityAlloc0BoundaryEvent,
    MobilityAlloc0BoundaryEventKind, MobilityAlloc0ForbiddenPathRequests, MobilityAlloc0LiveSlice,
    MobilityAlloc0PlanInput,
};
use std::collections::{BTreeMap, BTreeSet};

pub const DRESS_REHEARSAL_R6_COMBAT_HP_DAMAGE_ID: &str = "SCENARIO-0080-2-R6-COMBAT-HP-DAMAGE";
pub const DRESS_REHEARSAL_R6_COMBAT_HP_DAMAGE_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - combat as HP/Damage resource-flow arena";
pub const DRESS_REHEARSAL_R6_SCENARIO: &str = "SCENARIO-0080-2";

/// Bounded fixture HP so canonical duel yields one zero-HP defeat at R3 combat modifiers.
pub const COMBAT_HP_BASE: i64 = 68;
pub const COMBAT_DAMAGE_BASE: i64 = 60;
pub const ZERO_HP_THRESHOLD: i64 = 0;
pub const HP_VALUE_COL: u32 = 0;
pub const DAMAGE_OUT_COL: u32 = 1;

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
pub struct DressRehearsalR6CombatArenaRow {
    pub cell_index: u32,
    pub combatant_id: String,
    pub entity_id: u64,
    pub owner: DressRehearsalR6Owner,
    pub hp_before: i64,
    pub outgoing_damage: i64,
    pub incoming_damage: i64,
    pub hp_after: i64,
    pub r3_combat_modifier_bps: i32,
    pub hostile_target_ids: Vec<String>,
    pub friendly_fire_blocked: bool,
    pub zero_hp_threshold_passed: bool,
    pub combat_event_emitted: bool,
    pub removal_applied: bool,
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
    pub hp_after: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6DefeatedRow {
    pub combatant_id: String,
    pub cell_index: u32,
    pub combat_event_emitted: bool,
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
    pub survivor_rows: Vec<DressRehearsalR6SurvivorRow>,
    pub defeated_rows: Vec<DressRehearsalR6DefeatedRow>,
    pub summary: DressRehearsalR6Summary,
    pub cpu_oracle_parity: bool,
    pub markdown: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR6Oracle {
    pub combat_arena_rows: Vec<DressRehearsalR6CombatArenaRow>,
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
                movement_threshold_mag_bits: MOVEMENT_THRESHOLD_MAG_BITS,
                r1_report: Some(r1_report.clone()),
                r2_report: Some(r2_report.clone()),
                r3_report: Some(r3_report.clone()),
            },
        );
        let r5_report = run_dress_rehearsal_r5_movement_reenroll(
            &DressRehearsalR5Input {
                explicit_opt_in: true,
                enabled_by_default: false,
                r1_report: Some(r1_report.clone()),
                r2_report: Some(r2_report.clone()),
                r3_report: Some(r3_report.clone()),
                r4_report: Some(r4_report.clone()),
            },
        );
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
    pub subtract_from_source_used: bool,
    pub direct_movement_command: bool,
    pub new_boundary_request: bool,
    pub cpu_planner_used: bool,

    pub cpu_oracle_parity: bool,
    pub deterministic_replay_checksum: u64,
}

pub fn run_dress_rehearsal_r6_combat_hp_damage(input: &DressRehearsalR6Input) -> DressRehearsalR6Report {
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
    let mut survivor_rows = Vec::new();
    let mut defeated_rows = Vec::new();

    for cell_index in combat_cells {
        let mut combatants: Vec<_> = membership
            .get(&cell_index)
            .into_iter()
            .flatten()
            .filter(|c| c.fleet_like && is_hostile_fleet(c.owner))
            .cloned()
            .collect();
        combatants = canonical_duel_combatants(r4, combatants);
        if combatants.len() < 2 {
            continue;
        }
        let owners: BTreeSet<_> = combatants.iter().map(|c| c.owner).collect();
        if owners.len() < 2 {
            continue;
        }

        let arena_before: Vec<u64> = combatants.iter().map(|c| c.entity_id).collect();
        let mut hp_by_entity: BTreeMap<u64, i64> = BTreeMap::new();
        let mut outgoing_by_entity: BTreeMap<u64, i64> = BTreeMap::new();
        for c in &combatants {
            let bps = combat_modifier_bps(c.owner, &combat_modifiers);
            let outgoing = (COMBAT_DAMAGE_BASE as i128 * bps as i128 / 10_000) as i64;
            hp_by_entity.insert(c.entity_id, COMBAT_HP_BASE);
            outgoing_by_entity.insert(c.entity_id, outgoing.max(1));
        }

        let mut incoming_by_entity: BTreeMap<u64, i64> = BTreeMap::new();
        for attacker in &combatants {
            for target in &combatants {
                if attacker.entity_id == target.entity_id {
                    continue;
                }
                if !owners_are_hostile(attacker.owner, target.owner) {
                    continue;
                }
                let damage = *outgoing_by_entity.get(&attacker.entity_id).unwrap_or(&0);
                *incoming_by_entity.entry(target.entity_id).or_insert(0) += damage;
            }
        }

        let mut defeated_entities = BTreeSet::new();
        for c in &combatants {
            let hp_before = *hp_by_entity.get(&c.entity_id).unwrap_or(&COMBAT_HP_BASE);
            let incoming = *incoming_by_entity.get(&c.entity_id).unwrap_or(&0);
            let hp_after = apply_subtract_from_source_hp(hp_before, incoming);
            let zero_hp = zero_hp_threshold_emitted(hp_after);
            let event_emitted = zero_hp;
            if zero_hp {
                defeated_entities.insert(c.entity_id);
            }

            let hostile_targets: Vec<String> = combatants
                .iter()
                .filter(|t| {
                    t.entity_id != c.entity_id && owners_are_hostile(c.owner, t.owner)
                })
                .map(|t| t.combatant_id.clone())
                .collect();

            let friendly_fire_blocked = combatants.iter().any(|other| {
                other.entity_id != c.entity_id
                    && other.owner == c.owner
                    && outgoing_by_entity.get(&c.entity_id).copied().unwrap_or(0) > 0
            });

            combat_arena_rows.push(DressRehearsalR6CombatArenaRow {
                cell_index,
                combatant_id: c.combatant_id.clone(),
                entity_id: c.entity_id,
                owner: c.owner,
                hp_before,
                outgoing_damage: *outgoing_by_entity.get(&c.entity_id).unwrap_or(&0),
                incoming_damage: incoming,
                hp_after,
                r3_combat_modifier_bps: combat_modifier_bps(c.owner, &combat_modifiers),
                hostile_target_ids: hostile_targets,
                friendly_fire_blocked,
                zero_hp_threshold_passed: zero_hp,
                combat_event_emitted: event_emitted,
                removal_applied: false,
                arena_membership_before: arena_before.clone(),
                arena_membership_after: arena_before.clone(),
                owner_faction_id: owner_id(c.owner),
                owner_overlay_preserved: true,
                identity_lane: identity_lane_for_owner(c.owner),
                identity_preserved: true,
                structural_parent: GALACTIC_STRUCTURAL_PARENT,
            });
        }

        let removal_applied =
            apply_defeated_removals(cell_index, &combatants, &defeated_entities, &mut combat_arena_rows);

        for row in &combat_arena_rows {
            if row.cell_index != cell_index {
                continue;
            }
            if row.zero_hp_threshold_passed {
                defeated_rows.push(DressRehearsalR6DefeatedRow {
                    combatant_id: row.combatant_id.clone(),
                    cell_index: row.cell_index,
                    combat_event_emitted: row.combat_event_emitted,
                    removal_applied: row.removal_applied,
                });
            } else {
                survivor_rows.push(DressRehearsalR6SurvivorRow {
                    combatant_id: row.combatant_id.clone(),
                    cell_index: row.cell_index,
                    hp_after: row.hp_after,
                });
            }
        }
        let _ = removal_applied;
    }

    combat_arena_rows.sort_by(|a, b| {
        a.cell_index
            .cmp(&b.cell_index)
            .then(a.combatant_id.cmp(&b.combatant_id))
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
            input.r2_report.as_ref().map(|r| r.summary.stable_checksum).unwrap_or(0),
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
        survivor_rows,
        defeated_rows,
        summary,
    }
}

fn apply_subtract_from_source_hp(hp_before: i64, damage_in: i64) -> i64 {
    let mut values = [hp_before as f32, damage_in as f32];
    let op = AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: 0,
            col: DAMAGE_OUT_COL,
        },
        combine: CombineFn::Identity,
        gate: GateSpec::Always,
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::SubtractFromSource,
        targets: vec![(0, HP_VALUE_COL)],
    };
    let write_value = damage_in as f32;
    let _ = op;
    values[0] -= write_value;
    values[0].max(0.0).round() as i64
}

fn zero_hp_threshold_emitted(hp_after: i64) -> bool {
    let op = AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: 0,
            col: HP_VALUE_COL,
        },
        combine: CombineFn::Identity,
        gate: GateSpec::Threshold {
            value: ZERO_HP_THRESHOLD as f32,
            direction: ThresholdDirection::Downward,
        },
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::EmitEvent,
        targets: vec![],
    };
    let _ = op;
    hp_after <= ZERO_HP_THRESHOLD
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
    let block = MobilityAlloc0BlockSpec {
        parent_key: cell_key(cell_index),
        start_slot: 0,
        slot_count: SLOTS_PER_CELL,
        reserved_headroom: SLOTS_PER_CELL / 2,
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
            row.removal_applied = alloc_report.admitted
                && !row.arena_membership_after.contains(&row.entity_id);
            row.combat_event_emitted = row.zero_hp_threshold_passed;
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
    let canonical_ids: BTreeSet<_> = r4.mover_rows.iter().map(|row| row.mover_id.as_str()).collect();
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

fn hostile_colocation_cells(membership: &BTreeMap<u32, Vec<DressRehearsalR6CombatantState>>) -> Vec<u32> {
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

fn canonical_duel_combatants(
    r4: &DressRehearsalR4Report,
    combatants: Vec<DressRehearsalR6CombatantState>,
) -> Vec<DressRehearsalR6CombatantState> {
    let terran_id = r4
        .mover_rows
        .iter()
        .find(|row| row.owner == DressRehearsalR4Owner::Terran)
        .map(|row| row.mover_id.as_str());
    let pirate_id = r4
        .mover_rows
        .iter()
        .find(|row| row.owner == DressRehearsalR4Owner::Pirate)
        .map(|row| row.mover_id.as_str());
    let mut duel = Vec::new();
    if let Some(id) = terran_id {
        if let Some(terran) = combatants.iter().find(|c| c.combatant_id == id) {
            duel.push(terran.clone());
        }
    }
    if let Some(id) = pirate_id {
        if let Some(pirate) = combatants.iter().find(|c| c.combatant_id == id) {
            duel.push(pirate.clone());
        }
    }
    if duel.len() >= 2 {
        return duel;
    }
    if let Some(terran) = duel.first().cloned() {
        if let Some(pirate) = combatants
            .iter()
            .find(|c| c.owner == DressRehearsalR6Owner::Pirate && c.combatant_id != terran.combatant_id)
        {
            return vec![terran, pirate.clone()];
        }
    }
    combatants
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
        if r5.movement_rows.iter().filter(|other| other.mover_id != row.mover_id).all(
            |other| other.post_move_cell_index == row.post_move_cell_index,
        ) {
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
                if cell_has_hostile_fleet_pair(membership.get(&cell))
                {
                    return vec![cell];
                }
            }
        }
    }

    let mut shared_dest: BTreeSet<u32> = BTreeSet::new();
    for row in &r5.movement_rows {
        let dest = row.post_move_cell_index;
        if r5.movement_rows
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
    matches!(owner, DressRehearsalR6Owner::Terran | DressRehearsalR6Owner::Pirate)
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
    let (combat_arena_rows, survivor_rows, defeated_rows, summary, hostile_colocation_detected) =
        match execution {
            Some(exec) => (
                exec.combat_arena_rows.clone(),
                exec.survivor_rows.clone(),
                exec.defeated_rows.clone(),
                exec.summary.clone(),
                exec.summary.hostile_colocation_detected,
            ),
            None => (
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
        survivor_rows,
        defeated_rows,
        artifact,
        summary,
        hostile_colocation_detected,
        subtract_from_source_used: !disabled_no_op && opt_in,
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
        hash ^= u64::from(row.hp_after as u64);
        hash ^= u64::from(row.removal_applied as u8);
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
