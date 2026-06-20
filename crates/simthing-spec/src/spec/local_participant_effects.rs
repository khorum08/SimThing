//! LOCAL-PARTICIPANT-EFFECTS-0 — runtime participant effect previews under tick shell proof.

use std::collections::BTreeSet;

use super::runtime_local_allocation::RuntimeLocalAllocationState;
use super::runtime_rf_tick::evaluate_runtime_rf_tick;
use super::runtime_tick_shell::{evaluate_runtime_tick_shell, RuntimeTickId};
use super::scenario::SimThingScenarioSpec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalParticipantEffectsErrorKind {
    RuntimeTickShellRejected,
    MissingSourceSimThingId,
    DuplicateSourceEffect,
    ArithmeticOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalParticipantEffectsError {
    pub kind: LocalParticipantEffectsErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalParticipantEffectsDeferralKind {
    EconomyExecutionDeferred,
    ParticipantPropertyMutationDeferred,
    ScenarioAuthorityMutationDeferred,
    StudioPresentationDeferred,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalParticipantEffectsDeferral {
    pub kind: LocalParticipantEffectsDeferralKind,
    pub reason: String,
}

/// Per-source runtime participant effect preview derived from allocation state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeLocalParticipantEffect {
    pub source_simthing_id_raw: u32,
    pub owner_ref: String,
    pub resource_key: String,
    pub scope_id: String,
    pub requested: u32,
    pub allocated: u32,
    pub unmet: u32,
    pub satisfied: bool,
    pub effect_application_deferred: bool,
}

/// Proof/report-only local participant effects boundary.
#[derive(Debug, Clone, PartialEq)]
pub struct LocalParticipantEffectsReport {
    pub effect_count: u32,
    pub owner_channel_count: u32,
    pub requested_total: u32,
    pub allocated_total: u32,
    pub unmet_total: u32,
    pub satisfied_count: u32,
    pub unsatisfied_count: u32,
    pub effects: Vec<RuntimeLocalParticipantEffect>,
    pub economy_execution_deferred: bool,
    pub participant_property_mutation_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub errors: Vec<LocalParticipantEffectsError>,
    pub deferrals: Vec<LocalParticipantEffectsDeferral>,
}

/// Convert runtime-local allocation states into participant effect previews.
pub fn local_participant_effects_from_allocations(
    allocations: &[RuntimeLocalAllocationState],
) -> Result<LocalParticipantEffectsReport, LocalParticipantEffectsError> {
    if allocations.is_empty() {
        return Ok(empty_effects_report());
    }

    let mut effects = Vec::with_capacity(allocations.len());
    let mut seen_sources = BTreeSet::new();
    let mut owner_channels = BTreeSet::new();
    let mut requested_total: u32 = 0;
    let mut allocated_total: u32 = 0;
    let mut unmet_total: u32 = 0;
    let mut satisfied_count: u32 = 0;
    let mut unsatisfied_count: u32 = 0;

    for allocation in allocations {
        if allocation.source_simthing_id_raw == 0 {
            return Err(LocalParticipantEffectsError {
                kind: LocalParticipantEffectsErrorKind::MissingSourceSimThingId,
                message: "allocation requires non-zero source_simthing_id_raw".to_string(),
            });
        }

        let dedupe_key = (
            allocation.owner_ref.clone(),
            allocation.resource_key.clone(),
            allocation.scope_id.clone(),
            allocation.source_simthing_id_raw,
        );
        if !seen_sources.insert(dedupe_key) {
            return Err(LocalParticipantEffectsError {
                kind: LocalParticipantEffectsErrorKind::DuplicateSourceEffect,
                message: format!(
                    "duplicate effect for source SimThing id {}",
                    allocation.source_simthing_id_raw
                ),
            });
        }

        requested_total = requested_total
            .checked_add(allocation.requested)
            .ok_or_else(|| LocalParticipantEffectsError {
                kind: LocalParticipantEffectsErrorKind::ArithmeticOverflow,
                message: "requested_total overflow".to_string(),
            })?;
        allocated_total = allocated_total
            .checked_add(allocation.allocated)
            .ok_or_else(|| LocalParticipantEffectsError {
                kind: LocalParticipantEffectsErrorKind::ArithmeticOverflow,
                message: "allocated_total overflow".to_string(),
            })?;
        unmet_total = unmet_total.checked_add(allocation.unmet).ok_or_else(|| {
            LocalParticipantEffectsError {
                kind: LocalParticipantEffectsErrorKind::ArithmeticOverflow,
                message: "unmet_total overflow".to_string(),
            }
        })?;

        let satisfied = allocation.unmet == 0;
        if satisfied {
            satisfied_count =
                satisfied_count
                    .checked_add(1)
                    .ok_or_else(|| LocalParticipantEffectsError {
                        kind: LocalParticipantEffectsErrorKind::ArithmeticOverflow,
                        message: "satisfied_count overflow".to_string(),
                    })?;
        } else {
            unsatisfied_count =
                unsatisfied_count
                    .checked_add(1)
                    .ok_or_else(|| LocalParticipantEffectsError {
                        kind: LocalParticipantEffectsErrorKind::ArithmeticOverflow,
                        message: "unsatisfied_count overflow".to_string(),
                    })?;
        }

        owner_channels.insert((
            allocation.owner_ref.clone(),
            allocation.resource_key.clone(),
        ));

        effects.push(RuntimeLocalParticipantEffect {
            source_simthing_id_raw: allocation.source_simthing_id_raw,
            owner_ref: allocation.owner_ref.clone(),
            resource_key: allocation.resource_key.clone(),
            scope_id: allocation.scope_id.clone(),
            requested: allocation.requested,
            allocated: allocation.allocated,
            unmet: allocation.unmet,
            satisfied,
            effect_application_deferred: true,
        });
    }

    effects.sort_by(|a, b| {
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

    let effect_count = u32::try_from(effects.len()).map_err(|_| LocalParticipantEffectsError {
        kind: LocalParticipantEffectsErrorKind::ArithmeticOverflow,
        message: "effect_count exceeds u32".to_string(),
    })?;

    Ok(LocalParticipantEffectsReport {
        effect_count,
        owner_channel_count: owner_channels.len() as u32,
        requested_total,
        allocated_total,
        unmet_total,
        satisfied_count,
        unsatisfied_count,
        effects,
        economy_execution_deferred: true,
        participant_property_mutation_deferred: true,
        scenario_authority_mutation_deferred: true,
        errors: Vec::new(),
        deferrals: default_deferrals(),
    })
}

/// Evaluate local participant effects from Scenario authority under the tick shell (read-only).
pub fn evaluate_local_participant_effects(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
) -> Result<LocalParticipantEffectsReport, LocalParticipantEffectsError> {
    if tick_id.0 == 0 {
        return Err(LocalParticipantEffectsError {
            kind: LocalParticipantEffectsErrorKind::RuntimeTickShellRejected,
            message: "tick id must be non-zero".to_string(),
        });
    }

    evaluate_runtime_tick_shell(scenario, tick_id).map_err(|e| LocalParticipantEffectsError {
        kind: LocalParticipantEffectsErrorKind::RuntimeTickShellRejected,
        message: format!("{:?}: {}", e.kind, e.message),
    })?;

    let tick_report =
        evaluate_runtime_rf_tick(scenario).map_err(|e| LocalParticipantEffectsError {
            kind: LocalParticipantEffectsErrorKind::RuntimeTickShellRejected,
            message: format!("{:?}: {}", e.kind, e.message),
        })?;

    local_participant_effects_from_allocations(&tick_report.local_allocation_report.states)
}

fn empty_effects_report() -> LocalParticipantEffectsReport {
    LocalParticipantEffectsReport {
        effect_count: 0,
        owner_channel_count: 0,
        requested_total: 0,
        allocated_total: 0,
        unmet_total: 0,
        satisfied_count: 0,
        unsatisfied_count: 0,
        effects: Vec::new(),
        economy_execution_deferred: true,
        participant_property_mutation_deferred: true,
        scenario_authority_mutation_deferred: true,
        errors: Vec::new(),
        deferrals: default_deferrals(),
    }
}

fn default_deferrals() -> Vec<LocalParticipantEffectsDeferral> {
    vec![
        LocalParticipantEffectsDeferral {
            kind: LocalParticipantEffectsDeferralKind::EconomyExecutionDeferred,
            reason: "full economy execution remains deferred".to_string(),
        },
        LocalParticipantEffectsDeferral {
            kind: LocalParticipantEffectsDeferralKind::ParticipantPropertyMutationDeferred,
            reason: "participant SimThing properties are not mutated by effect previews"
                .to_string(),
        },
        LocalParticipantEffectsDeferral {
            kind: LocalParticipantEffectsDeferralKind::ScenarioAuthorityMutationDeferred,
            reason: "Scenario authority is not mutated by effect previews".to_string(),
        },
        LocalParticipantEffectsDeferral {
            kind: LocalParticipantEffectsDeferralKind::StudioPresentationDeferred,
            reason: "Studio local participant effects presentation remains deferred".to_string(),
        },
    ]
}

/// Aggregate allocated and unmet totals per owner/resource for GPU proof comparison.
pub fn local_participant_effects_aggregate_totals(
    report: &LocalParticipantEffectsReport,
) -> std::collections::BTreeMap<(String, String), (u32, u32)> {
    use std::collections::BTreeMap;
    let mut totals: BTreeMap<(String, String), (u32, u32)> = BTreeMap::new();
    for effect in &report.effects {
        let entry = totals
            .entry((effect.owner_ref.clone(), effect.resource_key.clone()))
            .or_insert((0, 0));
        entry.0 = entry.0.saturating_add(effect.allocated);
        entry.1 = entry.1.saturating_add(effect.unmet);
    }
    totals
}
