//! LOCAL-EFFECT-APPLICATION-AUTHORITY-0 — runtime local effect application authority boundary.

use std::collections::BTreeSet;

use super::local_participant_effects::{
    evaluate_local_participant_effects, RuntimeLocalParticipantEffect,
};
use super::runtime_tick_history::scenario_authority_digest;
use super::runtime_tick_shell::RuntimeTickId;
use super::scenario::SimThingScenarioSpec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalEffectApplicationErrorKind {
    LocalParticipantEffectsRejected,
    MissingSourceSimThingId,
    DuplicateSourceApplication,
    ArithmeticOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalEffectApplicationError {
    pub kind: LocalEffectApplicationErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalEffectApplicationDeferralKind {
    SemanticEffectExecutionDeferred,
    ParticipantPropertyMutationDeferred,
    ScenarioAuthorityMutationDeferred,
    SavefileMutationDeferred,
    StudioPresentationDeferred,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalEffectApplicationDeferral {
    pub kind: LocalEffectApplicationDeferralKind,
    pub reason: String,
}

/// Per-source runtime effect application record derived from effect preview.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeLocalEffectApplicationRecord {
    pub source_simthing_id_raw: u32,
    pub owner_ref: String,
    pub resource_key: String,
    pub scope_id: String,
    pub requested: u32,
    pub allocated: u32,
    pub unmet: u32,
    pub satisfied: bool,
    pub runtime_applied_amount: u32,
    pub semantic_effect_deferred: bool,
    pub participant_property_mutation_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
}

/// Proof/report-only local effect application boundary.
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeLocalEffectApplicationReport {
    pub application_count: u32,
    pub owner_channel_count: u32,
    pub requested_total: u32,
    pub allocated_total: u32,
    pub unmet_total: u32,
    pub runtime_applied_total: u32,
    pub satisfied_count: u32,
    pub unsatisfied_count: u32,
    pub records: Vec<RuntimeLocalEffectApplicationRecord>,
    pub semantic_effect_execution_deferred: bool,
    pub participant_property_mutation_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
    pub errors: Vec<LocalEffectApplicationError>,
    pub deferrals: Vec<LocalEffectApplicationDeferral>,
}

/// Authority preservation proof for local effect application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalEffectApplicationAuthorityProof {
    pub scenario_authority_digest_before: String,
    pub scenario_authority_digest_after: String,
    pub scenario_authority_unchanged: bool,
    pub participant_property_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
    pub semantic_effect_execution_deferred: bool,
}

/// Convert local participant effect previews into runtime application records.
pub fn apply_runtime_local_effect_records(
    effects: &[RuntimeLocalParticipantEffect],
) -> Result<RuntimeLocalEffectApplicationReport, LocalEffectApplicationError> {
    if effects.is_empty() {
        return Ok(empty_application_report());
    }

    let mut records = Vec::with_capacity(effects.len());
    let mut seen_sources = BTreeSet::new();
    let mut owner_channels = BTreeSet::new();
    let mut requested_total: u32 = 0;
    let mut allocated_total: u32 = 0;
    let mut unmet_total: u32 = 0;
    let mut runtime_applied_total: u32 = 0;
    let mut satisfied_count: u32 = 0;
    let mut unsatisfied_count: u32 = 0;

    for effect in effects {
        if effect.source_simthing_id_raw == 0 {
            return Err(LocalEffectApplicationError {
                kind: LocalEffectApplicationErrorKind::MissingSourceSimThingId,
                message: "effect requires non-zero source_simthing_id_raw".to_string(),
            });
        }

        let dedupe_key = (
            effect.owner_ref.clone(),
            effect.resource_key.clone(),
            effect.scope_id.clone(),
            effect.source_simthing_id_raw,
        );
        if !seen_sources.insert(dedupe_key) {
            return Err(LocalEffectApplicationError {
                kind: LocalEffectApplicationErrorKind::DuplicateSourceApplication,
                message: format!(
                    "duplicate application for source SimThing id {}",
                    effect.source_simthing_id_raw
                ),
            });
        }

        requested_total = requested_total
            .checked_add(effect.requested)
            .ok_or_else(|| LocalEffectApplicationError {
                kind: LocalEffectApplicationErrorKind::ArithmeticOverflow,
                message: "requested_total overflow".to_string(),
            })?;
        allocated_total = allocated_total
            .checked_add(effect.allocated)
            .ok_or_else(|| LocalEffectApplicationError {
                kind: LocalEffectApplicationErrorKind::ArithmeticOverflow,
                message: "allocated_total overflow".to_string(),
            })?;
        unmet_total =
            unmet_total
                .checked_add(effect.unmet)
                .ok_or_else(|| LocalEffectApplicationError {
                    kind: LocalEffectApplicationErrorKind::ArithmeticOverflow,
                    message: "unmet_total overflow".to_string(),
                })?;
        runtime_applied_total = runtime_applied_total
            .checked_add(effect.allocated)
            .ok_or_else(|| LocalEffectApplicationError {
                kind: LocalEffectApplicationErrorKind::ArithmeticOverflow,
                message: "runtime_applied_total overflow".to_string(),
            })?;

        if effect.satisfied {
            satisfied_count =
                satisfied_count
                    .checked_add(1)
                    .ok_or_else(|| LocalEffectApplicationError {
                        kind: LocalEffectApplicationErrorKind::ArithmeticOverflow,
                        message: "satisfied_count overflow".to_string(),
                    })?;
        } else {
            unsatisfied_count =
                unsatisfied_count
                    .checked_add(1)
                    .ok_or_else(|| LocalEffectApplicationError {
                        kind: LocalEffectApplicationErrorKind::ArithmeticOverflow,
                        message: "unsatisfied_count overflow".to_string(),
                    })?;
        }

        owner_channels.insert((effect.owner_ref.clone(), effect.resource_key.clone()));

        records.push(RuntimeLocalEffectApplicationRecord {
            source_simthing_id_raw: effect.source_simthing_id_raw,
            owner_ref: effect.owner_ref.clone(),
            resource_key: effect.resource_key.clone(),
            scope_id: effect.scope_id.clone(),
            requested: effect.requested,
            allocated: effect.allocated,
            unmet: effect.unmet,
            satisfied: effect.satisfied,
            runtime_applied_amount: effect.allocated,
            semantic_effect_deferred: true,
            participant_property_mutation_deferred: true,
            scenario_authority_mutation_deferred: true,
        });
    }

    records.sort_by(|a, b| {
        (
            &a.owner_ref,
            &a.resource_key,
            &a.scope_id,
            a.source_simthing_id_raw,
        )
            .cmp(&(
                &b.owner_ref,
                &b.resource_key,
                &b.scope_id,
                b.source_simthing_id_raw,
            ))
    });

    let application_count =
        u32::try_from(records.len()).map_err(|_| LocalEffectApplicationError {
            kind: LocalEffectApplicationErrorKind::ArithmeticOverflow,
            message: "application_count exceeds u32".to_string(),
        })?;

    Ok(RuntimeLocalEffectApplicationReport {
        application_count,
        owner_channel_count: owner_channels.len() as u32,
        requested_total,
        allocated_total,
        unmet_total,
        runtime_applied_total,
        satisfied_count,
        unsatisfied_count,
        records,
        semantic_effect_execution_deferred: true,
        participant_property_mutation_deferred: true,
        scenario_authority_mutation_deferred: true,
        savefile_mutation_deferred: true,
        errors: Vec::new(),
        deferrals: default_deferrals(),
    })
}

/// Evaluate local effect application from Scenario authority (read-only).
pub fn evaluate_runtime_local_effect_application(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
) -> Result<RuntimeLocalEffectApplicationReport, LocalEffectApplicationError> {
    let effects_report = evaluate_local_participant_effects(scenario, tick_id).map_err(|e| {
        LocalEffectApplicationError {
            kind: LocalEffectApplicationErrorKind::LocalParticipantEffectsRejected,
            message: format!("{:?}: {}", e.kind, e.message),
        }
    })?;

    apply_runtime_local_effect_records(&effects_report.effects)
}

/// Prove Scenario authority is unchanged after local effect application evaluation.
pub fn prove_local_effect_application_preserves_authority(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
) -> Result<LocalEffectApplicationAuthorityProof, LocalEffectApplicationError> {
    let scenario_authority_digest_before =
        scenario_authority_digest(scenario).map_err(|e| LocalEffectApplicationError {
            kind: LocalEffectApplicationErrorKind::LocalParticipantEffectsRejected,
            message: e.message,
        })?;

    let _application = evaluate_runtime_local_effect_application(scenario, tick_id)?;

    let scenario_authority_digest_after =
        scenario_authority_digest(scenario).map_err(|e| LocalEffectApplicationError {
            kind: LocalEffectApplicationErrorKind::LocalParticipantEffectsRejected,
            message: e.message,
        })?;

    let scenario_authority_unchanged =
        scenario_authority_digest_before == scenario_authority_digest_after;

    Ok(LocalEffectApplicationAuthorityProof {
        scenario_authority_digest_before,
        scenario_authority_digest_after,
        scenario_authority_unchanged,
        participant_property_mutation_deferred: true,
        savefile_mutation_deferred: true,
        semantic_effect_execution_deferred: true,
    })
}

fn empty_application_report() -> RuntimeLocalEffectApplicationReport {
    RuntimeLocalEffectApplicationReport {
        application_count: 0,
        owner_channel_count: 0,
        requested_total: 0,
        allocated_total: 0,
        unmet_total: 0,
        runtime_applied_total: 0,
        satisfied_count: 0,
        unsatisfied_count: 0,
        records: Vec::new(),
        semantic_effect_execution_deferred: true,
        participant_property_mutation_deferred: true,
        scenario_authority_mutation_deferred: true,
        savefile_mutation_deferred: true,
        errors: Vec::new(),
        deferrals: default_deferrals(),
    }
}

fn default_deferrals() -> Vec<LocalEffectApplicationDeferral> {
    vec![
        LocalEffectApplicationDeferral {
            kind: LocalEffectApplicationDeferralKind::SemanticEffectExecutionDeferred,
            reason: "semantic economy/consumption/supply effects remain deferred".to_string(),
        },
        LocalEffectApplicationDeferral {
            kind: LocalEffectApplicationDeferralKind::ParticipantPropertyMutationDeferred,
            reason: "participant SimThing properties are not mutated by application records"
                .to_string(),
        },
        LocalEffectApplicationDeferral {
            kind: LocalEffectApplicationDeferralKind::ScenarioAuthorityMutationDeferred,
            reason: "Scenario authority is not mutated by application records".to_string(),
        },
        LocalEffectApplicationDeferral {
            kind: LocalEffectApplicationDeferralKind::SavefileMutationDeferred,
            reason: "savefile and persistent timeline mutation remain deferred".to_string(),
        },
        LocalEffectApplicationDeferral {
            kind: LocalEffectApplicationDeferralKind::StudioPresentationDeferred,
            reason: "Studio local effect application presentation remains deferred".to_string(),
        },
    ]
}

/// Aggregate runtime_applied and unmet totals per owner/resource for GPU proof comparison.
pub fn local_effect_application_aggregate_totals(
    report: &RuntimeLocalEffectApplicationReport,
) -> std::collections::BTreeMap<(String, String), (u32, u32)> {
    use std::collections::BTreeMap;
    let mut totals: BTreeMap<(String, String), (u32, u32)> = BTreeMap::new();
    for record in &report.records {
        let entry = totals
            .entry((record.owner_ref.clone(), record.resource_key.clone()))
            .or_insert((0, 0));
        entry.0 = entry.0.saturating_add(record.runtime_applied_amount);
        entry.1 = entry.1.saturating_add(record.unmet);
    }
    totals
}
