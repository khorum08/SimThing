//! LOCAL-EFFECT-APPLICATION-AUTHORITY-0 — compile local effect application authority boundary.

use simthing_core::{CompiledAccumulatorOpPlan, StructuralScalarChannel};
use simthing_spec::{
    evaluate_runtime_local_effect_application, local_effect_application_aggregate_totals,
    prove_local_effect_application_preserves_authority, LocalEffectApplicationAuthorityProof,
    RuntimeLocalEffectApplicationReport, RuntimeTickId, SimThingScenarioSpec, SpecError,
};

use crate::local_participant_effects_compile::compile_local_participant_effects_plan;
use crate::owner_silo_accumulator_compile::compile_participant_channel_sum_plan;
use crate::runtime_tick_history_compile::compile_runtime_tick_history_plan;

/// GPU aggregate proof plan for runtime_applied/unmet totals per owner/resource channel.
#[derive(Debug, Clone, PartialEq)]
pub struct LocalEffectApplicationAggregateProofPlan {
    pub owner_ref: String,
    pub resource_key: String,
    pub runtime_applied_plan: CompiledAccumulatorOpPlan,
    pub unmet_plan: CompiledAccumulatorOpPlan,
    pub source_record_indices: Vec<usize>,
}

/// Driver plan composing tick history, local effects, and application authority boundary.
#[derive(Debug, Clone, PartialEq)]
pub struct LocalEffectApplicationPlan {
    pub tick_history_plan: crate::runtime_tick_history_compile::RuntimeTickHistoryPlan,
    pub local_participant_effects_plan:
        crate::local_participant_effects_compile::LocalParticipantEffectsPlan,
    pub application_report: RuntimeLocalEffectApplicationReport,
    pub authority_proof: LocalEffectApplicationAuthorityProof,
    pub gpu_application_aggregate_proof_plans: Vec<LocalEffectApplicationAggregateProofPlan>,
    pub semantic_effect_execution_deferred: bool,
    pub participant_property_mutation_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
}

/// Compile tick history + local effects into application report and authority proof.
pub fn compile_local_effect_application_plan(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    replay_count: u32,
) -> Result<LocalEffectApplicationPlan, SpecError> {
    if tick_id.0 == 0 {
        return Err(SpecError::ValidationFailed);
    }

    let tick_history_plan = compile_runtime_tick_history_plan(scenario, tick_id, replay_count)?;
    let local_participant_effects_plan = compile_local_participant_effects_plan(scenario, tick_id)?;

    let application_report = evaluate_runtime_local_effect_application(scenario, tick_id)
        .map_err(|_| SpecError::ValidationFailed)?;

    let authority_proof = prove_local_effect_application_preserves_authority(scenario, tick_id)
        .map_err(|_| SpecError::ValidationFailed)?;

    if !authority_proof.scenario_authority_unchanged {
        return Err(SpecError::ValidationFailed);
    }

    let gpu_application_aggregate_proof_plans =
        compile_application_aggregate_proof_plans(&application_report)?;

    Ok(LocalEffectApplicationPlan {
        tick_history_plan,
        local_participant_effects_plan,
        application_report,
        authority_proof,
        gpu_application_aggregate_proof_plans,
        semantic_effect_execution_deferred: true,
        participant_property_mutation_deferred: true,
        scenario_authority_mutation_deferred: true,
        savefile_mutation_deferred: true,
    })
}

fn compile_application_aggregate_proof_plans(
    report: &RuntimeLocalEffectApplicationReport,
) -> Result<Vec<LocalEffectApplicationAggregateProofPlan>, SpecError> {
    if report.records.is_empty() {
        return Ok(Vec::new());
    }

    let totals = local_effect_application_aggregate_totals(report);
    let mut plans = Vec::with_capacity(totals.len());
    for ((owner_ref, resource_key), _totals) in totals {
        let source_record_indices = report
            .records
            .iter()
            .enumerate()
            .filter(|(_, record)| {
                record.owner_ref == owner_ref && record.resource_key == resource_key
            })
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        if source_record_indices.is_empty() {
            return Err(SpecError::ValidationFailed);
        }

        let participant_count = source_record_indices.len() as u32;
        let runtime_applied_plan = compile_participant_channel_sum_plan(
            participant_count,
            StructuralScalarChannel(0),
            StructuralScalarChannel(1),
        );
        let unmet_plan = compile_participant_channel_sum_plan(
            participant_count,
            StructuralScalarChannel(0),
            StructuralScalarChannel(1),
        );
        plans.push(LocalEffectApplicationAggregateProofPlan {
            owner_ref,
            resource_key,
            runtime_applied_plan,
            unmet_plan,
            source_record_indices,
        });
    }
    Ok(plans)
}

pub fn local_effect_application_runtime_applied_tick_inputs(
    plan: &LocalEffectApplicationPlan,
    proof_plan: &LocalEffectApplicationAggregateProofPlan,
) -> Vec<f32> {
    let slot_count = proof_plan.runtime_applied_plan.slot_count as usize;
    let mut values = vec![0.0f32; slot_count];
    for (slot, &record_index) in proof_plan.source_record_indices.iter().enumerate() {
        values[slot] = plan.application_report.records[record_index].runtime_applied_amount as f32;
    }
    values
}

pub fn local_effect_application_unmet_tick_inputs(
    plan: &LocalEffectApplicationPlan,
    proof_plan: &LocalEffectApplicationAggregateProofPlan,
) -> Vec<f32> {
    let slot_count = proof_plan.unmet_plan.slot_count as usize;
    let mut values = vec![0.0f32; slot_count];
    for (slot, &record_index) in proof_plan.source_record_indices.iter().enumerate() {
        values[slot] = plan.application_report.records[record_index].unmet as f32;
    }
    values
}

pub fn local_effect_application_aggregate_slot(
    proof_plan: &LocalEffectApplicationAggregateProofPlan,
) -> usize {
    proof_plan.source_record_indices.len()
}

pub fn local_effect_application_cpu_runtime_applied_total(
    plan: &LocalEffectApplicationPlan,
    proof_plan: &LocalEffectApplicationAggregateProofPlan,
) -> u32 {
    proof_plan
        .source_record_indices
        .iter()
        .map(|&index| plan.application_report.records[index].runtime_applied_amount)
        .fold(0u32, |acc, v| acc.saturating_add(v))
}

pub fn local_effect_application_cpu_unmet_total(
    plan: &LocalEffectApplicationPlan,
    proof_plan: &LocalEffectApplicationAggregateProofPlan,
) -> u32 {
    proof_plan
        .source_record_indices
        .iter()
        .map(|&index| plan.application_report.records[index].unmet)
        .fold(0u32, |acc, v| acc.saturating_add(v))
}
