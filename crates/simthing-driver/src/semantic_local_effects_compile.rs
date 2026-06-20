//! SEMANTIC-LOCAL-EFFECT-TYPES-0 — compile typed semantic local effect outputs.

use simthing_core::{CompiledAccumulatorOpPlan, StructuralScalarChannel};
use simthing_spec::{
    evaluate_semantic_local_effects, prove_semantic_local_effects_preserve_authority,
    semantic_local_effects_aggregate_totals, RuntimeTickId, SemanticLocalEffectAuthorityProof,
    SemanticLocalEffectKind, SemanticLocalEffectReport, SimThingScenarioSpec, SpecError,
};

use crate::local_effect_application_compile::compile_local_effect_application_plan;
use crate::owner_silo_accumulator_compile::compile_participant_channel_sum_plan;

/// GPU aggregate proof plan for runtime_applied/shortfall semantic totals per owner/resource.
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticLocalEffectAggregateProofPlan {
    pub owner_ref: String,
    pub resource_key: String,
    pub runtime_applied_plan: CompiledAccumulatorOpPlan,
    pub shortfall_plan: CompiledAccumulatorOpPlan,
    pub source_output_indices: Vec<usize>,
}

/// Driver plan composing application boundary into typed semantic effect outputs.
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticLocalEffectsPlan {
    pub local_effect_application_plan:
        crate::local_effect_application_compile::LocalEffectApplicationPlan,
    pub semantic_report: SemanticLocalEffectReport,
    pub authority_proof: SemanticLocalEffectAuthorityProof,
    pub gpu_semantic_aggregate_proof_plans: Vec<SemanticLocalEffectAggregateProofPlan>,
    pub semantic_execution_deferred: bool,
    pub participant_property_mutation_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
}

/// Compile local effect application into typed semantic effect report and authority proof.
pub fn compile_semantic_local_effects_plan(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    replay_count: u32,
) -> Result<SemanticLocalEffectsPlan, SpecError> {
    if tick_id.0 == 0 {
        return Err(SpecError::ValidationFailed);
    }

    let local_effect_application_plan =
        compile_local_effect_application_plan(scenario, tick_id, replay_count)?;

    let semantic_report = evaluate_semantic_local_effects(scenario, tick_id, replay_count)
        .map_err(|_| SpecError::ValidationFailed)?;

    let authority_proof =
        prove_semantic_local_effects_preserve_authority(scenario, tick_id, replay_count)
            .map_err(|_| SpecError::ValidationFailed)?;

    if !authority_proof.scenario_authority_unchanged {
        return Err(SpecError::ValidationFailed);
    }

    let gpu_semantic_aggregate_proof_plans =
        compile_semantic_aggregate_proof_plans(&semantic_report)?;

    Ok(SemanticLocalEffectsPlan {
        local_effect_application_plan,
        semantic_report,
        authority_proof,
        gpu_semantic_aggregate_proof_plans,
        semantic_execution_deferred: true,
        participant_property_mutation_deferred: true,
        scenario_authority_mutation_deferred: true,
        savefile_mutation_deferred: true,
    })
}

fn compile_semantic_aggregate_proof_plans(
    report: &SemanticLocalEffectReport,
) -> Result<Vec<SemanticLocalEffectAggregateProofPlan>, SpecError> {
    if report.outputs.is_empty() {
        return Ok(Vec::new());
    }

    let totals = semantic_local_effects_aggregate_totals(report);
    let mut plans = Vec::with_capacity(totals.len());
    for ((owner_ref, resource_key), _totals) in totals {
        let applied_indices = report
            .outputs
            .iter()
            .enumerate()
            .filter(|(_, output)| {
                output.owner_ref == owner_ref
                    && output.resource_key == resource_key
                    && output.effect_kind == SemanticLocalEffectKind::RuntimeAppliedAmount
            })
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        let shortfall_indices = report
            .outputs
            .iter()
            .enumerate()
            .filter(|(_, output)| {
                output.owner_ref == owner_ref
                    && output.resource_key == resource_key
                    && output.effect_kind == SemanticLocalEffectKind::ResourceShortfall
            })
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        if applied_indices.is_empty() {
            return Err(SpecError::ValidationFailed);
        }

        let applied_count = applied_indices.len() as u32;
        // AccumulatorOp requires at least one input slot; zero-shortfall channels use one zero slot.
        let shortfall_count = shortfall_indices.len().max(1) as u32;
        let runtime_applied_plan = compile_participant_channel_sum_plan(
            applied_count,
            StructuralScalarChannel(0),
            StructuralScalarChannel(1),
        );
        let shortfall_plan = compile_participant_channel_sum_plan(
            shortfall_count,
            StructuralScalarChannel(0),
            StructuralScalarChannel(1),
        );
        let mut source_output_indices = applied_indices;
        source_output_indices.extend(shortfall_indices);
        plans.push(SemanticLocalEffectAggregateProofPlan {
            owner_ref,
            resource_key,
            runtime_applied_plan,
            shortfall_plan,
            source_output_indices,
        });
    }
    Ok(plans)
}

pub fn semantic_local_effects_applied_output_indices(
    plan: &SemanticLocalEffectsPlan,
    proof_plan: &SemanticLocalEffectAggregateProofPlan,
) -> Vec<usize> {
    proof_plan
        .source_output_indices
        .iter()
        .copied()
        .filter(|&index| {
            plan.semantic_report.outputs[index].effect_kind
                == SemanticLocalEffectKind::RuntimeAppliedAmount
        })
        .collect()
}

pub fn semantic_local_effects_shortfall_output_indices(
    plan: &SemanticLocalEffectsPlan,
    proof_plan: &SemanticLocalEffectAggregateProofPlan,
) -> Vec<usize> {
    proof_plan
        .source_output_indices
        .iter()
        .copied()
        .filter(|&index| {
            plan.semantic_report.outputs[index].effect_kind
                == SemanticLocalEffectKind::ResourceShortfall
        })
        .collect()
}

pub fn semantic_local_effects_runtime_applied_tick_inputs(
    plan: &SemanticLocalEffectsPlan,
    proof_plan: &SemanticLocalEffectAggregateProofPlan,
) -> Vec<f32> {
    let indices = semantic_local_effects_applied_output_indices(plan, proof_plan);
    let slot_count = proof_plan.runtime_applied_plan.slot_count as usize;
    let mut values = vec![0.0f32; slot_count];
    for (slot, &output_index) in indices.iter().enumerate() {
        values[slot] = plan.semantic_report.outputs[output_index].amount as f32;
    }
    values
}

pub fn semantic_local_effects_shortfall_tick_inputs(
    plan: &SemanticLocalEffectsPlan,
    proof_plan: &SemanticLocalEffectAggregateProofPlan,
) -> Vec<f32> {
    let indices = semantic_local_effects_shortfall_output_indices(plan, proof_plan);
    let slot_count = proof_plan.shortfall_plan.slot_count as usize;
    let mut values = vec![0.0f32; slot_count];
    for (slot, &output_index) in indices.iter().enumerate() {
        values[slot] = plan.semantic_report.outputs[output_index].amount as f32;
    }
    values
}

pub fn semantic_local_effects_runtime_applied_aggregate_slot(
    plan: &SemanticLocalEffectsPlan,
    proof_plan: &SemanticLocalEffectAggregateProofPlan,
) -> usize {
    semantic_local_effects_applied_output_indices(plan, proof_plan).len()
}

pub fn semantic_local_effects_shortfall_aggregate_slot(
    plan: &SemanticLocalEffectsPlan,
    proof_plan: &SemanticLocalEffectAggregateProofPlan,
) -> usize {
    semantic_local_effects_shortfall_output_indices(plan, proof_plan)
        .len()
        .max(1)
}

pub fn semantic_local_effects_cpu_runtime_applied_total(
    plan: &SemanticLocalEffectsPlan,
    proof_plan: &SemanticLocalEffectAggregateProofPlan,
) -> u32 {
    semantic_local_effects_applied_output_indices(plan, proof_plan)
        .iter()
        .map(|&index| plan.semantic_report.outputs[index].amount)
        .fold(0u32, |acc, v| acc.saturating_add(v))
}

pub fn semantic_local_effects_cpu_shortfall_total(
    plan: &SemanticLocalEffectsPlan,
    proof_plan: &SemanticLocalEffectAggregateProofPlan,
) -> u32 {
    semantic_local_effects_shortfall_output_indices(plan, proof_plan)
        .iter()
        .map(|&index| plan.semantic_report.outputs[index].amount)
        .fold(0u32, |acc, v| acc.saturating_add(v))
}
