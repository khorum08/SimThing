//! LOCAL-PARTICIPANT-EFFECTS-0 — compile tick shell into local participant effect previews.

use simthing_core::{CompiledAccumulatorOpPlan, StructuralScalarChannel};
use simthing_spec::{
    evaluate_local_participant_effects, local_participant_effects_aggregate_totals,
    LocalParticipantEffectsReport, RuntimeTickId, SimThingScenarioSpec, SpecError,
};

use crate::owner_silo_accumulator_compile::compile_participant_channel_sum_plan;
use crate::runtime_tick_shell_compile::compile_runtime_tick_shell_plan;

/// GPU aggregate proof plan for allocated/unmet effect totals per owner/resource channel.
#[derive(Debug, Clone, PartialEq)]
pub struct LocalParticipantEffectAggregateProofPlan {
    pub owner_ref: String,
    pub resource_key: String,
    pub allocated_plan: CompiledAccumulatorOpPlan,
    pub unmet_plan: CompiledAccumulatorOpPlan,
    pub source_effect_indices: Vec<usize>,
}

/// Driver plan composing tick shell into local participant effect previews.
#[derive(Debug, Clone, PartialEq)]
pub struct LocalParticipantEffectsPlan {
    pub tick_shell_plan: crate::runtime_tick_shell_compile::RuntimeTickShellPlan,
    pub effects_report: LocalParticipantEffectsReport,
    pub gpu_effect_aggregate_proof_plans: Vec<LocalParticipantEffectAggregateProofPlan>,
    pub economy_execution_deferred: bool,
    pub participant_property_mutation_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
}

/// Compile tick shell plan and evaluate local participant effect previews.
pub fn compile_local_participant_effects_plan(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
) -> Result<LocalParticipantEffectsPlan, SpecError> {
    if tick_id.0 == 0 {
        return Err(SpecError::ValidationFailed);
    }

    let tick_shell_plan = compile_runtime_tick_shell_plan(scenario, tick_id)?;

    let effects_report = evaluate_local_participant_effects(scenario, tick_id)
        .map_err(|_| SpecError::ValidationFailed)?;

    let gpu_effect_aggregate_proof_plans = compile_effect_aggregate_proof_plans(&effects_report)?;

    Ok(LocalParticipantEffectsPlan {
        tick_shell_plan,
        effects_report,
        gpu_effect_aggregate_proof_plans,
        economy_execution_deferred: true,
        participant_property_mutation_deferred: true,
        scenario_authority_mutation_deferred: true,
    })
}

fn compile_effect_aggregate_proof_plans(
    report: &LocalParticipantEffectsReport,
) -> Result<Vec<LocalParticipantEffectAggregateProofPlan>, SpecError> {
    if report.effects.is_empty() {
        return Ok(Vec::new());
    }

    let totals = local_participant_effects_aggregate_totals(report);
    let mut plans = Vec::with_capacity(totals.len());
    for ((owner_ref, resource_key), _totals) in totals {
        let source_effect_indices = report
            .effects
            .iter()
            .enumerate()
            .filter(|(_, effect)| {
                effect.owner_ref == owner_ref && effect.resource_key == resource_key
            })
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        if source_effect_indices.is_empty() {
            return Err(SpecError::ValidationFailed);
        }

        let participant_count = source_effect_indices.len() as u32;
        let allocated_plan = compile_participant_channel_sum_plan(
            participant_count,
            StructuralScalarChannel(0),
            StructuralScalarChannel(1),
        );
        let unmet_plan = compile_participant_channel_sum_plan(
            participant_count,
            StructuralScalarChannel(0),
            StructuralScalarChannel(1),
        );
        plans.push(LocalParticipantEffectAggregateProofPlan {
            owner_ref,
            resource_key,
            allocated_plan,
            unmet_plan,
            source_effect_indices,
        });
    }
    Ok(plans)
}

pub fn local_participant_effects_allocated_tick_inputs(
    plan: &LocalParticipantEffectsPlan,
    proof_plan: &LocalParticipantEffectAggregateProofPlan,
) -> Vec<f32> {
    let slot_count = proof_plan.allocated_plan.slot_count as usize;
    let mut values = vec![0.0f32; slot_count];
    for (slot, &effect_index) in proof_plan.source_effect_indices.iter().enumerate() {
        values[slot] = plan.effects_report.effects[effect_index].allocated as f32;
    }
    values
}

pub fn local_participant_effects_unmet_tick_inputs(
    plan: &LocalParticipantEffectsPlan,
    proof_plan: &LocalParticipantEffectAggregateProofPlan,
) -> Vec<f32> {
    let slot_count = proof_plan.unmet_plan.slot_count as usize;
    let mut values = vec![0.0f32; slot_count];
    for (slot, &effect_index) in proof_plan.source_effect_indices.iter().enumerate() {
        values[slot] = plan.effects_report.effects[effect_index].unmet as f32;
    }
    values
}

pub fn local_participant_effects_aggregate_slot(
    proof_plan: &LocalParticipantEffectAggregateProofPlan,
) -> usize {
    proof_plan.source_effect_indices.len()
}

pub fn local_participant_effects_cpu_allocated_total(
    plan: &LocalParticipantEffectsPlan,
    proof_plan: &LocalParticipantEffectAggregateProofPlan,
) -> u32 {
    proof_plan
        .source_effect_indices
        .iter()
        .map(|&index| plan.effects_report.effects[index].allocated)
        .fold(0u32, |acc, v| acc.saturating_add(v))
}

pub fn local_participant_effects_cpu_unmet_total(
    plan: &LocalParticipantEffectsPlan,
    proof_plan: &LocalParticipantEffectAggregateProofPlan,
) -> u32 {
    proof_plan
        .source_effect_indices
        .iter()
        .map(|&index| plan.effects_report.effects[index].unmet)
        .fold(0u32, |acc, v| acc.saturating_add(v))
}
