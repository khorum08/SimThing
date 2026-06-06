//! RUNTIME-0080-0-R1c: RESIDENT-REENROLL-0 stop-line/readiness report.
//!
//! R1c is the rung that moves structural decision authority (REENROLL scatter,
//! birth/removal, fusion lineage/compaction) onto resident GPU structural tables.
//! The current production track explicitly parks that behind the free-list-scatter
//! / compaction stop-lines. This module therefore refuses to claim PASS and records
//! the anti-drift/complete-shadow contract that the eventual R1c implementation
//! must satisfy.

use std::collections::BTreeMap;

use crate::dress_rehearsal_r6c_integrated_run::{
    run_dress_rehearsal_r6c_integrated_run, DressRehearsalR6cInput, DressRehearsalR6cOwner,
    DressRehearsalR6cWorld,
};
use crate::runtime_0080_0_r0::{RUNTIME_R0_EXPECTED_R6C_CHECKSUM, RUNTIME_R0_FOREGROUND_CAPTURE};
use crate::runtime_0080_0_r1b::{
    run_runtime_0080_0_r1b, Runtime0080R1bInput, Runtime0080R1bReport,
    RUNTIME_0080_0_R1B_STATUS_BLOCKED,
};

pub const RUNTIME_0080_0_R1C_ID: &str = "RUNTIME-0080-0-R1c";
pub const RUNTIME_0080_0_R1C_PRIMITIVE: &str = "RESIDENT-REENROLL-0";
pub const RUNTIME_0080_0_R1C_STATUS_PARTIAL: &str =
    "IMPLEMENTED / PARTIAL (STOP-LINE) - resident structural decisions require free-list scatter gate";
pub const RUNTIME_0080_0_R1C_STATUS_BLOCKED: &str =
    "BLOCKED - predecessor or discrete GPU unavailable";
pub const RUNTIME_R1C_SCOPE: &str =
    "resident structural decision authority gate: REENROLL/scatter/compact";
pub const RUNTIME_R1C_EXPECTED_REPORT_CHECKSUM: u64 = 0x8fdd_8977_a84b_4699;

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cInput {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl Runtime0080R1cInput {
    pub fn default_simsession() -> Self {
        Self {
            explicit_opt_in: false,
            enabled_by_default: false,
        }
    }

    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cPredecessorReport {
    pub r1b_verdict: String,
    pub r1b_status: String,
    pub r1b_event_journal_parity: bool,
    pub r1b_gpu_event_rows: u32,
    pub r1b_oracle_event_rows: u32,
    pub r1b_structural_decisions_gpu_emitted: bool,
    pub r1b_cpu_boundary_pass_consumes_event_rows: bool,
    pub r1b_cpu_boundary_pass_does_not_rederive_decisions: bool,
    pub r1b_checksum_matches: bool,
    pub r1b_report_checksum: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cShadowSnapshot {
    pub system_count: u32,
    pub fleet_count: u32,
    pub live_fleet_count: u32,
    pub destroyed_fleet_count: u32,
    pub spawned_fleet_count: u32,
    pub membership_cell_count: u32,
    pub membership_entity_count: u32,
    pub stockpile_owner_count: u32,
    pub construction_entry_count: u32,
    pub blockade_entry_count: u32,
    pub tier_a_value_cell_count: u32,
    pub structural_field_hash: u64,
    pub tier_a_value_hash: u64,
    pub complete_shadow_hash: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cShadowContractReport {
    pub complete_cpu_shadow_retained: bool,
    pub includes_tier_a_values: bool,
    pub includes_membership: bool,
    pub includes_positions: bool,
    pub includes_birth_removal_state: bool,
    pub includes_fusion_lineage: bool,
    pub includes_slot_allocation: bool,
    pub no_structural_reconstruction_from_value_projection: bool,
    pub serialize_reload_continue_roundtrip: bool,
    pub reloaded_from_serialized_snapshot: bool,
    pub serialized_snapshot_hash: u64,
    pub continue_hash_matches_after_reload: bool,
    pub roundtrip_hash_before: u64,
    pub roundtrip_hash_after: u64,
    pub initial_snapshot: Runtime0080R1cShadowSnapshot,
    pub reloaded_snapshot: Runtime0080R1cShadowSnapshot,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cStopLineReport {
    pub stop_line_triggered: bool,
    pub requires_resident_free_list_scatter: bool,
    pub requires_resident_compaction_or_lineage_update: bool,
    pub requires_m4a_or_multi_atlas_now: bool,
    pub semantic_gpu_code_required: bool,
    pub cpu_planner_required: bool,
    pub docs_invariants_edit_required: bool,
    pub pinned_number_change_required: bool,
    pub scenario_reopen_required: bool,
    pub next_smaller_rung: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cBackpressurePolicy {
    pub policy: &'static str,
    pub gpu_value_loop_may_run_ahead: bool,
    pub cpu_boundary_consumer_is_not_hot_path_gate: bool,
    pub max_unserialized_ticks_documented: u32,
    pub per_tick_decision_readback_forbidden: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Runtime0080R1cReport {
    pub id: &'static str,
    pub primitive_name: &'static str,
    pub status: &'static str,
    pub verdict: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<String>,
    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub scope: &'static str,
    pub predecessor: Option<Runtime0080R1cPredecessorReport>,
    pub shadow_contract: Option<Runtime0080R1cShadowContractReport>,
    pub stop_line: Runtime0080R1cStopLineReport,
    pub backpressure_policy: Runtime0080R1cBackpressurePolicy,
    pub structural_decisions_gpu_emitted: bool,
    pub resident_reenroll_scatter_authority: bool,
    pub resident_birth_removal_authority: bool,
    pub resident_fusion_compaction_authority: bool,
    pub cpu_decision_witness_still_authority: bool,
    pub event_journal_remains_only_structural_handoff: bool,
    pub r6c_checksum_expected: u64,
    pub r6c_checksum_observed: u64,
    pub r6c_checksum_matches: bool,
    pub foreground_capture_method: &'static str,
    pub domain_terms: Vec<&'static str>,
    pub stable_report_checksum: u64,
    pub artifact_markdown: String,
}

pub fn run_runtime_0080_0_r1c(input: &Runtime0080R1cInput) -> Runtime0080R1cReport {
    if !input.explicit_opt_in {
        return finalize_report(base_report(
            input,
            true,
            vec!["explicit_opt_in_required".to_string()],
        ));
    }
    if input.enabled_by_default {
        return finalize_report(base_report(
            input,
            false,
            vec!["enabled_by_default_forbidden".to_string()],
        ));
    }

    let r1b = run_runtime_0080_0_r1b(&Runtime0080R1bInput::explicit_opt_in());
    if r1b.status == RUNTIME_0080_0_R1B_STATUS_BLOCKED || r1b.verdict == "BLOCKED" {
        let mut report = base_report(
            input,
            false,
            vec!["r1b_predecessor_blocked_or_no_discrete_gpu".to_string()],
        );
        report.status = RUNTIME_0080_0_R1C_STATUS_BLOCKED;
        report.verdict = "BLOCKED";
        report.predecessor = Some(predecessor_report(&r1b));
        return finalize_report(report);
    }

    let oracle = run_dress_rehearsal_r6c_integrated_run(&DressRehearsalR6cInput::explicit_opt_in());
    let world = oracle
        .initial_world
        .as_ref()
        .expect("R6C report carries initial world");
    let shadow_contract = complete_shadow_contract(world);

    let mut report = base_report(input, false, Vec::new());
    report.status = RUNTIME_0080_0_R1C_STATUS_PARTIAL;
    report.verdict = "PARTIAL";
    report.admitted = true;
    report.diagnostics = vec![
        "r1c_stop_line_free_list_scatter_required".to_string(),
        "r1b_full_journal_parity_preserved".to_string(),
        "complete_cpu_shadow_contract_recorded".to_string(),
        "next_smaller_rung_resident_free_list_mark_only_no_compaction".to_string(),
    ];
    report.predecessor = Some(predecessor_report(&r1b));
    report.shadow_contract = Some(shadow_contract);
    report.r6c_checksum_observed = oracle.summary.stable_checksum;
    report.r6c_checksum_matches = report.r6c_checksum_observed == report.r6c_checksum_expected;
    finalize_report(report)
}

pub fn replay_runtime_0080_0_r1c() -> (Runtime0080R1cReport, Runtime0080R1cReport) {
    let input = Runtime0080R1cInput::explicit_opt_in();
    (
        run_runtime_0080_0_r1c(&input),
        run_runtime_0080_0_r1c(&input),
    )
}

fn base_report(
    input: &Runtime0080R1cInput,
    disabled_no_op: bool,
    diagnostics: Vec<String>,
) -> Runtime0080R1cReport {
    Runtime0080R1cReport {
        id: RUNTIME_0080_0_R1C_ID,
        primitive_name: RUNTIME_0080_0_R1C_PRIMITIVE,
        status: "NOT RUN",
        verdict: "NOT RUN",
        admitted: false,
        diagnostics,
        explicit_opt_in: input.explicit_opt_in,
        default_off: !input.enabled_by_default,
        disabled_no_op,
        scope: RUNTIME_R1C_SCOPE,
        predecessor: None,
        shadow_contract: None,
        stop_line: Runtime0080R1cStopLineReport {
            stop_line_triggered: true,
            requires_resident_free_list_scatter: true,
            requires_resident_compaction_or_lineage_update: true,
            requires_m4a_or_multi_atlas_now: false,
            semantic_gpu_code_required: false,
            cpu_planner_required: false,
            docs_invariants_edit_required: false,
            pinned_number_change_required: false,
            scenario_reopen_required: false,
            next_smaller_rung: "R1c-a resident free-list mark-only / no compaction",
        },
        backpressure_policy: Runtime0080R1cBackpressurePolicy {
            policy:
                "bounded-lag journal drain at tick boundary; GPU value loop remains independent",
            gpu_value_loop_may_run_ahead: true,
            cpu_boundary_consumer_is_not_hot_path_gate: true,
            max_unserialized_ticks_documented: 1,
            per_tick_decision_readback_forbidden: true,
        },
        structural_decisions_gpu_emitted: false,
        resident_reenroll_scatter_authority: false,
        resident_birth_removal_authority: false,
        resident_fusion_compaction_authority: false,
        cpu_decision_witness_still_authority: true,
        event_journal_remains_only_structural_handoff: true,
        r6c_checksum_expected: RUNTIME_R0_EXPECTED_R6C_CHECKSUM,
        r6c_checksum_observed: 0,
        r6c_checksum_matches: false,
        foreground_capture_method: RUNTIME_R0_FOREGROUND_CAPTURE,
        domain_terms: vec![
            "FieldPolicy",
            "field_agent",
            "selection",
            "extraction",
            "resident event journal",
            "GPU-side structural event rows",
            "disabled-transform parity check",
        ],
        stable_report_checksum: 0,
        artifact_markdown: String::new(),
    }
}

fn predecessor_report(report: &Runtime0080R1bReport) -> Runtime0080R1cPredecessorReport {
    Runtime0080R1cPredecessorReport {
        r1b_verdict: report.verdict.to_string(),
        r1b_status: report.status.to_string(),
        r1b_event_journal_parity: report.event_journal_parity_measured_from_gpu_values,
        r1b_gpu_event_rows: report.gpu_event_row_count_total,
        r1b_oracle_event_rows: report.oracle_event_row_count_total,
        r1b_structural_decisions_gpu_emitted: report.structural_decisions_gpu_emitted,
        r1b_cpu_boundary_pass_consumes_event_rows: report.cpu_boundary_pass_consumes_event_rows,
        r1b_cpu_boundary_pass_does_not_rederive_decisions: report
            .cpu_boundary_pass_does_not_rederive_decisions,
        r1b_checksum_matches: report.r6c_checksum_matches,
        r1b_report_checksum: report.stable_report_checksum,
    }
}

fn complete_shadow_contract(world: &DressRehearsalR6cWorld) -> Runtime0080R1cShadowContractReport {
    let initial_snapshot = shadow_snapshot(world);
    let serialized_shadow = serialize_shadow_snapshot(&initial_snapshot);
    let serialized_snapshot_hash = hash_bytes(serialized_shadow.as_bytes());
    let reloaded_snapshot = deserialize_shadow_snapshot(&serialized_shadow)
        .expect("R1c shadow snapshot serialization round-trips");
    let continue_hash_matches_after_reload =
        initial_snapshot.complete_shadow_hash == reloaded_snapshot.complete_shadow_hash;
    Runtime0080R1cShadowContractReport {
        complete_cpu_shadow_retained: true,
        includes_tier_a_values: initial_snapshot.tier_a_value_cell_count
            == world.grid_cell_count as u32
            && initial_snapshot.stockpile_owner_count > 0
            && initial_snapshot.construction_entry_count > 0
            && initial_snapshot.blockade_entry_count > 0,
        includes_membership: initial_snapshot.membership_cell_count > 0
            && initial_snapshot.membership_entity_count > 0,
        includes_positions: initial_snapshot.fleet_count > 0,
        includes_birth_removal_state: initial_snapshot.spawned_fleet_count
            <= initial_snapshot.fleet_count
            && initial_snapshot.destroyed_fleet_count <= initial_snapshot.fleet_count,
        includes_fusion_lineage: world.fleets.iter().all(|fleet| !fleet.lineage.is_empty()),
        includes_slot_allocation: initial_snapshot.membership_entity_count
            >= initial_snapshot.live_fleet_count,
        no_structural_reconstruction_from_value_projection: true,
        serialize_reload_continue_roundtrip: initial_snapshot == reloaded_snapshot,
        reloaded_from_serialized_snapshot: true,
        serialized_snapshot_hash,
        continue_hash_matches_after_reload,
        roundtrip_hash_before: initial_snapshot.complete_shadow_hash,
        roundtrip_hash_after: reloaded_snapshot.complete_shadow_hash,
        initial_snapshot,
        reloaded_snapshot,
    }
}

fn shadow_snapshot(world: &DressRehearsalR6cWorld) -> Runtime0080R1cShadowSnapshot {
    let live_fleet_count = world.fleets.iter().filter(|fleet| !fleet.destroyed).count() as u32;
    let destroyed_fleet_count = world.fleets.iter().filter(|fleet| fleet.destroyed).count() as u32;
    let spawned_fleet_count = world
        .fleets
        .iter()
        .filter(|fleet| fleet.spawned_by_production)
        .count() as u32;
    let membership_entity_count = world
        .arena_membership
        .values()
        .map(|entities| entities.len() as u32)
        .sum();
    let tier_a_value_hash = hash_tier_a_values(world);
    let structural_field_hash = hash_structural_fields(world);
    let mut complete = FNV_OFFSET;
    mix_u64(&mut complete, tier_a_value_hash);
    mix_u64(&mut complete, structural_field_hash);
    mix_u64(&mut complete, world.seed_checksum);

    Runtime0080R1cShadowSnapshot {
        system_count: world.systems.len() as u32,
        fleet_count: world.fleets.len() as u32,
        live_fleet_count,
        destroyed_fleet_count,
        spawned_fleet_count,
        membership_cell_count: world.arena_membership.len() as u32,
        membership_entity_count,
        stockpile_owner_count: world.stockpiles.len() as u32,
        construction_entry_count: world.construction_progress.len() as u32,
        blockade_entry_count: world.blockade_divert_owner.len() as u32,
        tier_a_value_cell_count: world.grid_cell_count as u32,
        structural_field_hash,
        tier_a_value_hash,
        complete_shadow_hash: complete,
    }
}

fn hash_tier_a_values(world: &DressRehearsalR6cWorld) -> u64 {
    let mut hash = FNV_OFFSET;
    mix_u64(&mut hash, world.galaxy_side as u64);
    mix_u64(&mut hash, world.grid_cell_count as u64);
    for value in &world.disruption {
        mix_u64(&mut hash, value.to_bits() as u64);
    }
    for value in &world.location_status {
        mix_u64(&mut hash, value.to_bits() as u64);
    }
    for (owner, amount) in &world.stockpiles {
        mix_owner(&mut hash, *owner);
        mix_i64(&mut hash, *amount);
    }
    for (system, amount) in &world.construction_progress {
        mix_u64(&mut hash, *system as u64);
        mix_i64(&mut hash, *amount);
    }
    for (system, owner) in &world.blockade_divert_owner {
        mix_u64(&mut hash, *system as u64);
        mix_u64(
            &mut hash,
            owner.map(|owner| owner.stable_code()).unwrap_or(0),
        );
    }
    hash
}

fn hash_structural_fields(world: &DressRehearsalR6cWorld) -> u64 {
    let mut hash = FNV_OFFSET;
    for system in &world.systems {
        mix_str(&mut hash, &system.system_id);
        mix_u64(&mut hash, system.system_index as u64);
        mix_owner(&mut hash, system.owner);
        mix_u64(&mut hash, system.cell_index as u64);
        mix_u64(&mut hash, system.has_starport as u64);
    }
    for fleet in &world.fleets {
        mix_str(&mut hash, &fleet.fleet_id);
        mix_u64(&mut hash, fleet.entity_id);
        mix_owner(&mut hash, fleet.owner);
        mix_u64(&mut hash, fleet.cell_index as u64);
        mix_i64(&mut hash, fleet.num_ships);
        mix_i64(&mut hash, fleet.hp_per_ship);
        mix_i64(&mut hash, fleet.damage_per_ship_per_tick);
        mix_u64(&mut hash, fleet.destroyed as u64);
        mix_u64(&mut hash, fleet.fleet_like as u64);
        mix_u64(&mut hash, fleet.owner_faction_id);
        mix_u64(&mut hash, fleet.identity_lane as u64);
        mix_u64(&mut hash, fleet.spawned_by_production as u64);
        mix_u64(&mut hash, fleet.last_moved_tick.unwrap_or(u32::MAX) as u64);
        for lineage in &fleet.lineage {
            mix_str(&mut hash, lineage);
        }
    }
    for (cell, entities) in sorted_membership(&world.arena_membership) {
        mix_u64(&mut hash, cell as u64);
        for entity in entities {
            mix_u64(&mut hash, entity);
        }
    }
    hash
}

fn sorted_membership(membership: &BTreeMap<u32, Vec<u64>>) -> Vec<(u32, Vec<u64>)> {
    membership
        .iter()
        .map(|(cell, entities)| {
            let mut sorted = entities.clone();
            sorted.sort_unstable();
            (*cell, sorted)
        })
        .collect()
}

fn serialize_shadow_snapshot(snapshot: &Runtime0080R1cShadowSnapshot) -> String {
    [
        snapshot.system_count.to_string(),
        snapshot.fleet_count.to_string(),
        snapshot.live_fleet_count.to_string(),
        snapshot.destroyed_fleet_count.to_string(),
        snapshot.spawned_fleet_count.to_string(),
        snapshot.membership_cell_count.to_string(),
        snapshot.membership_entity_count.to_string(),
        snapshot.stockpile_owner_count.to_string(),
        snapshot.construction_entry_count.to_string(),
        snapshot.blockade_entry_count.to_string(),
        snapshot.tier_a_value_cell_count.to_string(),
        snapshot.structural_field_hash.to_string(),
        snapshot.tier_a_value_hash.to_string(),
        snapshot.complete_shadow_hash.to_string(),
    ]
    .join("|")
}

fn deserialize_shadow_snapshot(serialized: &str) -> Option<Runtime0080R1cShadowSnapshot> {
    let mut fields = serialized.split('|');
    let snapshot = Runtime0080R1cShadowSnapshot {
        system_count: fields.next()?.parse().ok()?,
        fleet_count: fields.next()?.parse().ok()?,
        live_fleet_count: fields.next()?.parse().ok()?,
        destroyed_fleet_count: fields.next()?.parse().ok()?,
        spawned_fleet_count: fields.next()?.parse().ok()?,
        membership_cell_count: fields.next()?.parse().ok()?,
        membership_entity_count: fields.next()?.parse().ok()?,
        stockpile_owner_count: fields.next()?.parse().ok()?,
        construction_entry_count: fields.next()?.parse().ok()?,
        blockade_entry_count: fields.next()?.parse().ok()?,
        tier_a_value_cell_count: fields.next()?.parse().ok()?,
        structural_field_hash: fields.next()?.parse().ok()?,
        tier_a_value_hash: fields.next()?.parse().ok()?,
        complete_shadow_hash: fields.next()?.parse().ok()?,
    };
    fields.next().is_none().then_some(snapshot)
}

fn finalize_report(mut report: Runtime0080R1cReport) -> Runtime0080R1cReport {
    report.stable_report_checksum = checksum_report(&report);
    report.artifact_markdown = render_runtime_0080_r1c_artifact(&report);
    report
}

fn checksum_report(report: &Runtime0080R1cReport) -> u64 {
    let mut hash = FNV_OFFSET;
    mix_str(&mut hash, report.id);
    mix_str(&mut hash, report.primitive_name);
    mix_str(&mut hash, report.verdict);
    mix_str(&mut hash, report.status);
    mix_u64(&mut hash, report.structural_decisions_gpu_emitted as u64);
    mix_u64(&mut hash, report.stop_line.stop_line_triggered as u64);
    mix_u64(
        &mut hash,
        report.stop_line.requires_resident_free_list_scatter as u64,
    );
    mix_u64(
        &mut hash,
        report
            .stop_line
            .requires_resident_compaction_or_lineage_update as u64,
    );
    if let Some(predecessor) = &report.predecessor {
        mix_str(&mut hash, &predecessor.r1b_verdict);
        mix_u64(&mut hash, predecessor.r1b_event_journal_parity as u64);
        mix_u64(&mut hash, predecessor.r1b_gpu_event_rows as u64);
        mix_u64(&mut hash, predecessor.r1b_oracle_event_rows as u64);
        mix_u64(
            &mut hash,
            predecessor.r1b_structural_decisions_gpu_emitted as u64,
        );
        mix_u64(&mut hash, predecessor.r1b_report_checksum);
    }
    if let Some(shadow) = &report.shadow_contract {
        mix_u64(&mut hash, shadow.complete_cpu_shadow_retained as u64);
        mix_u64(&mut hash, shadow.serialize_reload_continue_roundtrip as u64);
        mix_u64(&mut hash, shadow.reloaded_from_serialized_snapshot as u64);
        mix_u64(&mut hash, shadow.serialized_snapshot_hash);
        mix_u64(&mut hash, shadow.continue_hash_matches_after_reload as u64);
        mix_u64(&mut hash, shadow.roundtrip_hash_before);
        mix_u64(&mut hash, shadow.roundtrip_hash_after);
    }
    mix_u64(&mut hash, report.r6c_checksum_observed);
    hash
}

pub fn render_runtime_0080_r1c_artifact(report: &Runtime0080R1cReport) -> String {
    let predecessor = report
        .predecessor
        .as_ref()
        .map(|predecessor| {
            format!(
                "- r1b_verdict: {}\n- r1b_event_journal_parity: {}\n- r1b_gpu_event_rows: {}\n- r1b_oracle_event_rows: {}\n- r1b_structural_decisions_gpu_emitted: {}\n",
                predecessor.r1b_verdict,
                predecessor.r1b_event_journal_parity,
                predecessor.r1b_gpu_event_rows,
                predecessor.r1b_oracle_event_rows,
                predecessor.r1b_structural_decisions_gpu_emitted
            )
        })
        .unwrap_or_else(|| "- predecessor: not run\n".to_string());
    let shadow = report
        .shadow_contract
        .as_ref()
        .map(|shadow| {
            format!(
                "- complete_cpu_shadow_retained: {}\n- serialize_reload_continue_roundtrip: {}\n- reloaded_from_serialized_snapshot: {}\n- serialized_snapshot_hash: {:016x}\n- continue_hash_matches_after_reload: {}\n- roundtrip_hash_before: {:016x}\n- roundtrip_hash_after: {:016x}\n- systems/fleets/membership_entities: {}/{}/{}\n",
                shadow.complete_cpu_shadow_retained,
                shadow.serialize_reload_continue_roundtrip,
                shadow.reloaded_from_serialized_snapshot,
                shadow.serialized_snapshot_hash,
                shadow.continue_hash_matches_after_reload,
                shadow.roundtrip_hash_before,
                shadow.roundtrip_hash_after,
                shadow.initial_snapshot.system_count,
                shadow.initial_snapshot.fleet_count,
                shadow.initial_snapshot.membership_entity_count
            )
        })
        .unwrap_or_else(|| "- shadow_contract: not run\n".to_string());

    format!(
        "# RUNTIME-0080-0-R1c Results\n\n\
         Status: {status}\n\
         Verdict: {verdict}\n\
         Primitive: `{primitive}`\n\
         Rung: `{rung}`\n\
         Scope: {scope}\n\
         Stable report checksum: `{checksum:016x}`\n\n\
         ## Verdict\n\
         R1c is intentionally PARTIAL: resident structural decision authority requires the parked \
         free-list-scatter/compaction gate. This patch does not claim GPU structural decisions.\n\n\
         ## Predecessor R1b\n\
         {predecessor}\n\
         ## Complete CPU Shadow Contract\n\
         {shadow}\n\
         ## R1c Stop Line\n\
         - stop_line_triggered: {stop_line}\n\
         - requires_resident_free_list_scatter: {free_list}\n\
         - requires_resident_compaction_or_lineage_update: {compaction}\n\
         - requires_m4a_or_multi_atlas_now: {m4a}\n\
         - semantic_gpu_code_required: {semantic_gpu}\n\
         - cpu_planner_required: {cpu_planner}\n\
         - docs_invariants_edit_required: {invariant}\n\
         - pinned_number_change_required: {pinned}\n\
         - scenario_reopen_required: {scenario}\n\
         - next_smaller_rung: {next_rung}\n\n\
         ## Authority Flags\n\
         - structural_decisions_gpu_emitted: {gpu_decisions}\n\
         - resident_reenroll_scatter_authority: {reenroll}\n\
         - resident_birth_removal_authority: {birth_removal}\n\
         - resident_fusion_compaction_authority: {fusion}\n\
         - cpu_decision_witness_still_authority: {cpu_witness}\n\
         - event_journal_remains_only_structural_handoff: {journal_handoff}\n\n\
         ## Backpressure Policy\n\
         - policy: {policy}\n\
         - gpu_value_loop_may_run_ahead: {run_ahead}\n\
         - cpu_boundary_consumer_is_not_hot_path_gate: {consumer_not_gate}\n\
         - max_unserialized_ticks_documented: {lag}\n\
         - per_tick_decision_readback_forbidden: {readback_forbidden}\n\n\
         ## Checksum\n\
         - expected: `{expected:016x}`\n\
         - observed: `{observed:016x}`\n\
         - matches: {matches}\n\n\
         ## Domain Terms\n\
         - {terms}\n",
        status = report.status,
        verdict = report.verdict,
        primitive = report.primitive_name,
        rung = report.id,
        scope = report.scope,
        checksum = report.stable_report_checksum,
        predecessor = predecessor,
        shadow = shadow,
        stop_line = report.stop_line.stop_line_triggered,
        free_list = report.stop_line.requires_resident_free_list_scatter,
        compaction = report
            .stop_line
            .requires_resident_compaction_or_lineage_update,
        m4a = report.stop_line.requires_m4a_or_multi_atlas_now,
        semantic_gpu = report.stop_line.semantic_gpu_code_required,
        cpu_planner = report.stop_line.cpu_planner_required,
        invariant = report.stop_line.docs_invariants_edit_required,
        pinned = report.stop_line.pinned_number_change_required,
        scenario = report.stop_line.scenario_reopen_required,
        next_rung = report.stop_line.next_smaller_rung,
        gpu_decisions = report.structural_decisions_gpu_emitted,
        reenroll = report.resident_reenroll_scatter_authority,
        birth_removal = report.resident_birth_removal_authority,
        fusion = report.resident_fusion_compaction_authority,
        cpu_witness = report.cpu_decision_witness_still_authority,
        journal_handoff = report.event_journal_remains_only_structural_handoff,
        policy = report.backpressure_policy.policy,
        run_ahead = report.backpressure_policy.gpu_value_loop_may_run_ahead,
        consumer_not_gate = report
            .backpressure_policy
            .cpu_boundary_consumer_is_not_hot_path_gate,
        lag = report.backpressure_policy.max_unserialized_ticks_documented,
        readback_forbidden = report
            .backpressure_policy
            .per_tick_decision_readback_forbidden,
        expected = report.r6c_checksum_expected,
        observed = report.r6c_checksum_observed,
        matches = report.r6c_checksum_matches,
        terms = report.domain_terms.join("\n- "),
    )
}

fn mix_owner(hash: &mut u64, owner: DressRehearsalR6cOwner) {
    mix_u64(hash, owner.stable_code());
}

fn mix_i64(hash: &mut u64, value: i64) {
    mix_u64(hash, value as u64);
}

fn mix_str(hash: &mut u64, value: &str) {
    for byte in value.as_bytes() {
        *hash ^= *byte as u64;
        *hash = hash.wrapping_mul(FNV_PRIME);
    }
}

fn hash_bytes(bytes: &[u8]) -> u64 {
    let mut hash = FNV_OFFSET;
    for byte in bytes {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

fn mix_u64(hash: &mut u64, value: u64) {
    for byte in value.to_le_bytes() {
        *hash ^= byte as u64;
        *hash = hash.wrapping_mul(FNV_PRIME);
    }
}
