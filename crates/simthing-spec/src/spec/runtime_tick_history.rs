//! RUNTIME-TICK-HISTORY-REPLAY-0 — deterministic tick history and replay evidence.

use super::local_participant_effects::evaluate_local_participant_effects;
use super::runtime_tick_shell::{evaluate_runtime_tick_shell, RuntimeTickId, RuntimeTickStage};
use super::scenario::{serialize_scenario_authority, SimThingScenarioSpec};

/// Maximum allowed replay evaluations per history proof.
pub const MAX_RUNTIME_TICK_REPLAY_COUNT: u32 = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeTickHistoryErrorKind {
    RuntimeTickShellRejected,
    LocalParticipantEffectsRejected,
    InvalidReplayCount,
    ScenarioAuthorityDigestFailed,
    EntryDigestFailed,
    ReplayMismatch,
    ArithmeticOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeTickHistoryError {
    pub kind: RuntimeTickHistoryErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeTickReplayMismatch {
    pub replay_index: u32,
    pub expected_digest: String,
    pub actual_digest: String,
    pub reason: String,
}

/// Deterministic proof entry for one tick evaluation over shell + local effects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeTickHistoryEntry {
    pub tick_id: RuntimeTickId,
    pub scenario_authority_digest: String,
    pub stage_order: Vec<RuntimeTickStage>,
    pub participant_count: u32,
    pub reduce_up_bucket_count: u32,
    pub owner_silo_writeback_count: u32,
    pub disburse_down_result_count: u32,
    pub local_allocation_count: u32,
    pub local_effect_count: u32,
    pub allocated_total: u32,
    pub unmet_total: u32,
    pub satisfied_count: u32,
    pub unsatisfied_count: u32,
    pub economy_execution_deferred: bool,
    pub participant_property_mutation_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub local_effect_application_deferred: bool,
    pub entry_digest: String,
}

/// Proof report from repeated tick evaluation over unchanged Scenario authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeTickReplayReport {
    pub replay_count: u32,
    pub all_replays_match: bool,
    pub entries: Vec<RuntimeTickHistoryEntry>,
    pub mismatches: Vec<RuntimeTickReplayMismatch>,
    pub scenario_authority_unchanged: bool,
}

/// Evaluate one deterministic runtime tick history entry.
pub fn evaluate_runtime_tick_history_entry(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
) -> Result<RuntimeTickHistoryEntry, RuntimeTickHistoryError> {
    if tick_id.0 == 0 {
        return Err(RuntimeTickHistoryError {
            kind: RuntimeTickHistoryErrorKind::RuntimeTickShellRejected,
            message: "tick id must be non-zero".to_string(),
        });
    }

    let scenario_authority_digest = scenario_authority_digest(scenario)?;

    let shell_report =
        evaluate_runtime_tick_shell(scenario, tick_id).map_err(|e| RuntimeTickHistoryError {
            kind: RuntimeTickHistoryErrorKind::RuntimeTickShellRejected,
            message: format!("{:?}: {}", e.kind, e.message),
        })?;

    let effects_report = evaluate_local_participant_effects(scenario, tick_id).map_err(|e| {
        RuntimeTickHistoryError {
            kind: RuntimeTickHistoryErrorKind::LocalParticipantEffectsRejected,
            message: format!("{:?}: {}", e.kind, e.message),
        }
    })?;

    let mut entry = RuntimeTickHistoryEntry {
        tick_id,
        scenario_authority_digest,
        stage_order: shell_report.stage_order.clone(),
        participant_count: shell_report.participant_count,
        reduce_up_bucket_count: shell_report.reduce_up_bucket_count,
        owner_silo_writeback_count: shell_report.owner_silo_writeback_count,
        disburse_down_result_count: shell_report.disburse_down_result_count,
        local_allocation_count: shell_report.local_allocation_count,
        local_effect_count: effects_report.effect_count,
        allocated_total: effects_report.allocated_total,
        unmet_total: effects_report.unmet_total,
        satisfied_count: effects_report.satisfied_count,
        unsatisfied_count: effects_report.unsatisfied_count,
        economy_execution_deferred: true,
        participant_property_mutation_deferred: true,
        scenario_authority_mutation_deferred: true,
        local_effect_application_deferred: true,
        entry_digest: String::new(),
    };

    entry.entry_digest = entry_digest(&entry).map_err(|msg| RuntimeTickHistoryError {
        kind: RuntimeTickHistoryErrorKind::EntryDigestFailed,
        message: msg,
    })?;

    Ok(entry)
}

/// Replay the same tick evaluation and verify deterministic entry digests.
pub fn replay_runtime_tick_history(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    replay_count: u32,
) -> Result<RuntimeTickReplayReport, RuntimeTickHistoryError> {
    if replay_count == 0 || replay_count > MAX_RUNTIME_TICK_REPLAY_COUNT {
        return Err(RuntimeTickHistoryError {
            kind: RuntimeTickHistoryErrorKind::InvalidReplayCount,
            message: format!(
                "replay_count must be in 1..={MAX_RUNTIME_TICK_REPLAY_COUNT}, got {replay_count}"
            ),
        });
    }

    let authority_before =
        serialize_scenario_authority(scenario).map_err(|e| RuntimeTickHistoryError {
            kind: RuntimeTickHistoryErrorKind::ScenarioAuthorityDigestFailed,
            message: e.to_string(),
        })?;

    let mut entries = Vec::with_capacity(replay_count as usize);
    let mut mismatches = Vec::new();
    let baseline = evaluate_runtime_tick_history_entry(scenario, tick_id)?;
    let expected_digest = baseline.entry_digest.clone();
    entries.push(baseline);

    for replay_index in 1..replay_count {
        let entry = evaluate_runtime_tick_history_entry(scenario, tick_id)?;
        if entry.entry_digest != expected_digest {
            mismatches.push(RuntimeTickReplayMismatch {
                replay_index,
                expected_digest: expected_digest.clone(),
                actual_digest: entry.entry_digest.clone(),
                reason: "entry_digest mismatch on replay".to_string(),
            });
        }
        entries.push(entry);
    }

    let authority_after =
        serialize_scenario_authority(scenario).map_err(|e| RuntimeTickHistoryError {
            kind: RuntimeTickHistoryErrorKind::ScenarioAuthorityDigestFailed,
            message: e.to_string(),
        })?;

    let all_replays_match = mismatches.is_empty();
    if !all_replays_match {
        return Err(RuntimeTickHistoryError {
            kind: RuntimeTickHistoryErrorKind::ReplayMismatch,
            message: format!("{} replay digest mismatches", mismatches.len()),
        });
    }

    Ok(RuntimeTickReplayReport {
        replay_count,
        all_replays_match,
        entries,
        mismatches,
        scenario_authority_unchanged: authority_before == authority_after,
    })
}

/// Deterministic digest of canonical Scenario authority serialization.
pub fn scenario_authority_digest(
    scenario: &SimThingScenarioSpec,
) -> Result<String, RuntimeTickHistoryError> {
    let json = serialize_scenario_authority(scenario).map_err(|e| RuntimeTickHistoryError {
        kind: RuntimeTickHistoryErrorKind::ScenarioAuthorityDigestFailed,
        message: e.to_string(),
    })?;
    Ok(fnv1a64_hex(&json))
}

fn entry_digest(entry: &RuntimeTickHistoryEntry) -> Result<String, String> {
    let stage_codes: Vec<u8> = entry
        .stage_order
        .iter()
        .map(runtime_tick_stage_code)
        .collect();
    let payload = format!(
        "tick={};auth={};stages={:?};participants={};reduce_up={};writeback={};disburse={};alloc={};effects={};allocated={};unmet={};satisfied={};unsatisfied={};econ_def={};prop_def={};auth_def={};effect_def={}",
        entry.tick_id.0,
        entry.scenario_authority_digest,
        stage_codes,
        entry.participant_count,
        entry.reduce_up_bucket_count,
        entry.owner_silo_writeback_count,
        entry.disburse_down_result_count,
        entry.local_allocation_count,
        entry.local_effect_count,
        entry.allocated_total,
        entry.unmet_total,
        entry.satisfied_count,
        entry.unsatisfied_count,
        entry.economy_execution_deferred as u8,
        entry.participant_property_mutation_deferred as u8,
        entry.scenario_authority_mutation_deferred as u8,
        entry.local_effect_application_deferred as u8,
    );
    Ok(fnv1a64_hex(&payload))
}

fn runtime_tick_stage_code(stage: &RuntimeTickStage) -> u8 {
    match stage {
        RuntimeTickStage::RuntimeRfTickComposition => 0,
        RuntimeTickStage::ParticipantAdmission => 1,
        RuntimeTickStage::ReduceUp => 2,
        RuntimeTickStage::OwnerSiloWriteback => 3,
        RuntimeTickStage::OwnerSiloDisburseDown => 4,
        RuntimeTickStage::RuntimeLocalAllocation => 5,
    }
}

fn fnv1a64_hex(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in input.as_bytes() {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}
