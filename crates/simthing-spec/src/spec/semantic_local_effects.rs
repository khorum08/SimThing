//! SEMANTIC-LOCAL-EFFECT-TYPES-0 — typed runtime semantic effect outputs.

use std::collections::BTreeSet;

use super::channel_key::{OwnerRef, ResourceKey, ScopeId};
use super::local_effect_application::{
    evaluate_runtime_local_effect_application, RuntimeLocalEffectApplicationReport,
};
use super::runtime_tick_history::scenario_authority_digest;
use super::runtime_tick_shell::RuntimeTickId;
use super::scenario::SimThingScenarioSpec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SemanticLocalEffectKind {
    ResourceSatisfied,
    ResourceShortfall,
    RuntimeAppliedAmount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticLocalEffectErrorKind {
    LocalEffectApplicationRejected,
    MissingSourceSimThingId,
    DuplicateSemanticOutput,
    ArithmeticOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticLocalEffectError {
    pub kind: SemanticLocalEffectErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticLocalEffectDeferralKind {
    SemanticExecutionDeferred,
    ParticipantPropertyMutationDeferred,
    ScenarioAuthorityMutationDeferred,
    SavefileMutationDeferred,
    StudioPresentationDeferred,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticLocalEffectDeferral {
    pub kind: SemanticLocalEffectDeferralKind,
    pub reason: String,
}

/// Typed semantic effect output derived from an application record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticLocalEffectOutput {
    pub source_simthing_id_raw: u32,
    pub owner_ref: OwnerRef,
    pub resource_key: ResourceKey,
    pub scope_id: ScopeId,
    pub effect_kind: SemanticLocalEffectKind,
    pub amount: u32,
    pub semantic_execution_deferred: bool,
    pub participant_property_mutation_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
}

/// Proof/report-only typed semantic local effects boundary.
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticLocalEffectReport {
    pub output_count: u32,
    pub owner_channel_count: u32,
    pub runtime_applied_total: u32,
    pub shortfall_total: u32,
    pub satisfied_output_count: u32,
    pub shortfall_output_count: u32,
    pub outputs: Vec<SemanticLocalEffectOutput>,
    pub semantic_execution_deferred: bool,
    pub participant_property_mutation_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
    pub errors: Vec<SemanticLocalEffectError>,
    pub deferrals: Vec<SemanticLocalEffectDeferral>,
}

/// Authority preservation proof for semantic local effects evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticLocalEffectAuthorityProof {
    pub scenario_authority_digest_before: String,
    pub scenario_authority_digest_after: String,
    pub scenario_authority_unchanged: bool,
    pub participant_property_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
    pub semantic_execution_deferred: bool,
}

/// Convert application records into typed semantic effect outputs.
pub fn semantic_local_effects_from_application(
    application_report: &RuntimeLocalEffectApplicationReport,
) -> Result<SemanticLocalEffectReport, SemanticLocalEffectError> {
    if application_report.records.is_empty() {
        return Ok(empty_semantic_report());
    }

    let mut outputs = Vec::new();
    let mut seen_outputs = BTreeSet::new();
    let mut owner_channels = BTreeSet::new();
    let mut runtime_applied_total: u32 = 0;
    let mut shortfall_total: u32 = 0;
    let mut satisfied_output_count: u32 = 0;
    let mut shortfall_output_count: u32 = 0;

    for record in &application_report.records {
        if record.source_simthing_id_raw == 0 {
            return Err(SemanticLocalEffectError {
                kind: SemanticLocalEffectErrorKind::MissingSourceSimThingId,
                message: "application record requires non-zero source_simthing_id_raw".to_string(),
            });
        }

        owner_channels.insert((record.owner_ref.clone(), record.resource_key.clone()));

        if record.runtime_applied_amount > 0 {
            push_output(
                &mut outputs,
                &mut seen_outputs,
                record,
                SemanticLocalEffectKind::RuntimeAppliedAmount,
                record.runtime_applied_amount,
            )?;
            runtime_applied_total = runtime_applied_total
                .checked_add(record.runtime_applied_amount)
                .ok_or_else(|| SemanticLocalEffectError {
                    kind: SemanticLocalEffectErrorKind::ArithmeticOverflow,
                    message: "runtime_applied_total overflow".to_string(),
                })?;
        }

        if record.satisfied {
            push_output(
                &mut outputs,
                &mut seen_outputs,
                record,
                SemanticLocalEffectKind::ResourceSatisfied,
                record.runtime_applied_amount,
            )?;
            satisfied_output_count =
                satisfied_output_count
                    .checked_add(1)
                    .ok_or_else(|| SemanticLocalEffectError {
                        kind: SemanticLocalEffectErrorKind::ArithmeticOverflow,
                        message: "satisfied_output_count overflow".to_string(),
                    })?;
        }

        if record.unmet > 0 {
            push_output(
                &mut outputs,
                &mut seen_outputs,
                record,
                SemanticLocalEffectKind::ResourceShortfall,
                record.unmet,
            )?;
            shortfall_total = shortfall_total.checked_add(record.unmet).ok_or_else(|| {
                SemanticLocalEffectError {
                    kind: SemanticLocalEffectErrorKind::ArithmeticOverflow,
                    message: "shortfall_total overflow".to_string(),
                }
            })?;
            shortfall_output_count =
                shortfall_output_count
                    .checked_add(1)
                    .ok_or_else(|| SemanticLocalEffectError {
                        kind: SemanticLocalEffectErrorKind::ArithmeticOverflow,
                        message: "shortfall_output_count overflow".to_string(),
                    })?;
        }
    }

    outputs.sort_by(|a, b| {
        (
            &a.owner_ref,
            &a.resource_key,
            &a.scope_id,
            a.source_simthing_id_raw,
            a.effect_kind,
        )
            .cmp(&(
                &b.owner_ref,
                &b.resource_key,
                &b.scope_id,
                b.source_simthing_id_raw,
                b.effect_kind,
            ))
    });

    let output_count = u32::try_from(outputs.len()).map_err(|_| SemanticLocalEffectError {
        kind: SemanticLocalEffectErrorKind::ArithmeticOverflow,
        message: "output_count exceeds u32".to_string(),
    })?;

    Ok(SemanticLocalEffectReport {
        output_count,
        owner_channel_count: owner_channels.len() as u32,
        runtime_applied_total,
        shortfall_total,
        satisfied_output_count,
        shortfall_output_count,
        outputs,
        semantic_execution_deferred: true,
        participant_property_mutation_deferred: true,
        scenario_authority_mutation_deferred: true,
        savefile_mutation_deferred: true,
        errors: Vec::new(),
        deferrals: default_deferrals(),
    })
}

/// Evaluate typed semantic local effects from Scenario authority (read-only).
pub fn evaluate_semantic_local_effects(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    _replay_count: u32,
) -> Result<SemanticLocalEffectReport, SemanticLocalEffectError> {
    let application_report =
        evaluate_runtime_local_effect_application(scenario, tick_id).map_err(|e| {
            SemanticLocalEffectError {
                kind: SemanticLocalEffectErrorKind::LocalEffectApplicationRejected,
                message: format!("{:?}: {}", e.kind, e.message),
            }
        })?;

    semantic_local_effects_from_application(&application_report)
}

/// Prove Scenario authority is unchanged after semantic local effects evaluation.
pub fn prove_semantic_local_effects_preserve_authority(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    replay_count: u32,
) -> Result<SemanticLocalEffectAuthorityProof, SemanticLocalEffectError> {
    let scenario_authority_digest_before =
        scenario_authority_digest(scenario).map_err(|e| SemanticLocalEffectError {
            kind: SemanticLocalEffectErrorKind::LocalEffectApplicationRejected,
            message: e.message,
        })?;

    let _report = evaluate_semantic_local_effects(scenario, tick_id, replay_count)?;

    let scenario_authority_digest_after =
        scenario_authority_digest(scenario).map_err(|e| SemanticLocalEffectError {
            kind: SemanticLocalEffectErrorKind::LocalEffectApplicationRejected,
            message: e.message,
        })?;

    let scenario_authority_unchanged =
        scenario_authority_digest_before == scenario_authority_digest_after;

    Ok(SemanticLocalEffectAuthorityProof {
        scenario_authority_digest_before,
        scenario_authority_digest_after,
        scenario_authority_unchanged,
        participant_property_mutation_deferred: true,
        savefile_mutation_deferred: true,
        semantic_execution_deferred: true,
    })
}

fn push_output(
    outputs: &mut Vec<SemanticLocalEffectOutput>,
    seen_outputs: &mut BTreeSet<(OwnerRef, ResourceKey, ScopeId, u32, u8)>,
    record: &super::local_effect_application::RuntimeLocalEffectApplicationRecord,
    effect_kind: SemanticLocalEffectKind,
    amount: u32,
) -> Result<(), SemanticLocalEffectError> {
    let kind_code = semantic_effect_kind_code(effect_kind);
    let dedupe_key = (
        record.owner_ref.clone(),
        record.resource_key.clone(),
        record.scope_id.clone(),
        record.source_simthing_id_raw,
        kind_code,
    );
    if !seen_outputs.insert(dedupe_key) {
        return Err(SemanticLocalEffectError {
            kind: SemanticLocalEffectErrorKind::DuplicateSemanticOutput,
            message: format!(
                "duplicate semantic output for source {} kind {:?}",
                record.source_simthing_id_raw, effect_kind
            ),
        });
    }

    outputs.push(SemanticLocalEffectOutput {
        source_simthing_id_raw: record.source_simthing_id_raw,
        owner_ref: record.owner_ref.clone(),
        resource_key: record.resource_key.clone(),
        scope_id: record.scope_id.clone(),
        effect_kind,
        amount,
        semantic_execution_deferred: true,
        participant_property_mutation_deferred: true,
        scenario_authority_mutation_deferred: true,
        savefile_mutation_deferred: true,
    });
    Ok(())
}

fn semantic_effect_kind_code(kind: SemanticLocalEffectKind) -> u8 {
    match kind {
        SemanticLocalEffectKind::ResourceSatisfied => 0,
        SemanticLocalEffectKind::ResourceShortfall => 1,
        SemanticLocalEffectKind::RuntimeAppliedAmount => 2,
    }
}

fn empty_semantic_report() -> SemanticLocalEffectReport {
    SemanticLocalEffectReport {
        output_count: 0,
        owner_channel_count: 0,
        runtime_applied_total: 0,
        shortfall_total: 0,
        satisfied_output_count: 0,
        shortfall_output_count: 0,
        outputs: Vec::new(),
        semantic_execution_deferred: true,
        participant_property_mutation_deferred: true,
        scenario_authority_mutation_deferred: true,
        savefile_mutation_deferred: true,
        errors: Vec::new(),
        deferrals: default_deferrals(),
    }
}

fn default_deferrals() -> Vec<SemanticLocalEffectDeferral> {
    vec![
        SemanticLocalEffectDeferral {
            kind: SemanticLocalEffectDeferralKind::SemanticExecutionDeferred,
            reason: "typed semantic outputs are defined; semantic execution remains deferred"
                .to_string(),
        },
        SemanticLocalEffectDeferral {
            kind: SemanticLocalEffectDeferralKind::ParticipantPropertyMutationDeferred,
            reason: "participant SimThing properties are not mutated by semantic outputs"
                .to_string(),
        },
        SemanticLocalEffectDeferral {
            kind: SemanticLocalEffectDeferralKind::ScenarioAuthorityMutationDeferred,
            reason: "Scenario authority is not mutated by semantic outputs".to_string(),
        },
        SemanticLocalEffectDeferral {
            kind: SemanticLocalEffectDeferralKind::SavefileMutationDeferred,
            reason: "savefile and persistent timeline mutation remain deferred".to_string(),
        },
        SemanticLocalEffectDeferral {
            kind: SemanticLocalEffectDeferralKind::StudioPresentationDeferred,
            reason: "Studio semantic local effects presentation remains deferred".to_string(),
        },
    ]
}

/// Aggregate runtime_applied and shortfall totals per owner/resource for GPU proof comparison.
pub fn semantic_local_effects_aggregate_totals(
    report: &SemanticLocalEffectReport,
) -> std::collections::BTreeMap<(OwnerRef, ResourceKey), (u32, u32)> {
    use std::collections::BTreeMap;
    let mut totals: BTreeMap<(OwnerRef, ResourceKey), (u32, u32)> = BTreeMap::new();
    for output in &report.outputs {
        let entry = totals
            .entry((output.owner_ref.clone(), output.resource_key.clone()))
            .or_insert((0, 0));
        match output.effect_kind {
            SemanticLocalEffectKind::RuntimeAppliedAmount => {
                entry.0 = entry.0.saturating_add(output.amount);
            }
            SemanticLocalEffectKind::ResourceShortfall => {
                entry.1 = entry.1.saturating_add(output.amount);
            }
            SemanticLocalEffectKind::ResourceSatisfied => {}
        }
    }
    totals
}
