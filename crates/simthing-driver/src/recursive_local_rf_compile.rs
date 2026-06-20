//! RECURSIVE-LOCAL-RF-EVALUATOR-0 — compile recursive local RF evaluator proof plan.

use simthing_core::{CompiledAccumulatorOpPlan, StructuralScalarChannel};
use simthing_spec::{
    evaluate_recursive_local_rf, prove_recursive_local_rf_preserves_authority,
    recursive_local_rf_arena_aggregate_totals,
    recursive_local_rf_report_matches_planet_child_compatibility_slice,
    RecursiveLocalRfAuthorityProof, RecursiveLocalRfCompatibilityReport,
    RecursiveLocalRfEvaluationReport, SimThingScenarioSpec, SpecError,
};

use crate::owner_silo_accumulator_compile::compile_participant_channel_sum_plan;

/// GPU aggregate proof plan for per-arena surplus/demand totals.
#[derive(Debug, Clone, PartialEq)]
pub struct RecursiveLocalRfAggregateProofPlan {
    pub arena_location_id_raw: u32,
    pub owner_ref: String,
    pub resource_key: String,
    pub surplus_plan: CompiledAccumulatorOpPlan,
    pub demand_plan: CompiledAccumulatorOpPlan,
    pub source_indices: Vec<usize>,
}

/// Driver plan for recursive local RF evaluation, authority, and compatibility proof.
#[derive(Debug, Clone, PartialEq)]
pub struct RecursiveLocalRfPlan {
    pub evaluation_report: RecursiveLocalRfEvaluationReport,
    pub authority_proof: RecursiveLocalRfAuthorityProof,
    pub compatibility_report: RecursiveLocalRfCompatibilityReport,
    pub gpu_arena_aggregate_proof_plans: Vec<RecursiveLocalRfAggregateProofPlan>,
    pub scenario_authority_mutation_deferred: bool,
    pub participant_property_mutation_deferred: bool,
    pub semantic_execution_deferred: bool,
    pub previous_rf_ladder_compatibility_preserved: bool,
}

/// Compile recursive local RF evaluation report, authority proof, and GPU aggregate plans.
pub fn compile_recursive_local_rf_plan(
    scenario: &SimThingScenarioSpec,
) -> Result<RecursiveLocalRfPlan, SpecError> {
    let evaluation_report =
        evaluate_recursive_local_rf(scenario).map_err(|_| SpecError::ValidationFailed)?;

    let authority_proof = prove_recursive_local_rf_preserves_authority(scenario)
        .map_err(|_| SpecError::ValidationFailed)?;

    if !authority_proof.scenario_authority_unchanged {
        return Err(SpecError::ValidationFailed);
    }

    let compatibility_report =
        recursive_local_rf_report_matches_planet_child_compatibility_slice(scenario)
            .map_err(|_| SpecError::ValidationFailed)?;

    let gpu_arena_aggregate_proof_plans =
        compile_recursive_rf_aggregate_proof_plans(&evaluation_report)?;

    Ok(RecursiveLocalRfPlan {
        previous_rf_ladder_compatibility_preserved: evaluation_report
            .previous_rf_ladder_compatibility_preserved,
        evaluation_report,
        authority_proof,
        compatibility_report,
        gpu_arena_aggregate_proof_plans,
        scenario_authority_mutation_deferred: true,
        participant_property_mutation_deferred: true,
        semantic_execution_deferred: true,
    })
}

fn compile_recursive_rf_aggregate_proof_plans(
    report: &RecursiveLocalRfEvaluationReport,
) -> Result<Vec<RecursiveLocalRfAggregateProofPlan>, SpecError> {
    if report.arena_reports.is_empty() {
        return Ok(Vec::new());
    }

    let totals = recursive_local_rf_arena_aggregate_totals(report);
    let mut plans = Vec::with_capacity(totals.len());
    for ((arena_id, owner_ref, resource_key), (surplus_total, demand_total)) in totals {
        if surplus_total == 0 && demand_total == 0 {
            continue;
        }
        let matching: Vec<_> = report
            .arena_reports
            .iter()
            .find(|arena| arena.location_id_raw == arena_id)
            .map(|arena| arena.participant_rows.as_slice())
            .unwrap_or(&[])
            .iter()
            .enumerate()
            .filter(|(_, row)| row.owner_ref == owner_ref && row.resource_key == resource_key)
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        let row_count = matching.len().max(1) as u32;
        let surplus_plan = compile_participant_channel_sum_plan(
            row_count,
            StructuralScalarChannel(0),
            StructuralScalarChannel(1),
        );
        let demand_plan = compile_participant_channel_sum_plan(
            row_count,
            StructuralScalarChannel(0),
            StructuralScalarChannel(1),
        );
        let _ = (surplus_total, demand_total);
        plans.push(RecursiveLocalRfAggregateProofPlan {
            arena_location_id_raw: arena_id,
            owner_ref,
            resource_key,
            surplus_plan,
            demand_plan,
            source_indices: matching,
        });
    }
    Ok(plans)
}

pub fn recursive_local_rf_surplus_tick_inputs(
    plan: &RecursiveLocalRfPlan,
    proof_plan: &RecursiveLocalRfAggregateProofPlan,
) -> Vec<f32> {
    let arena = plan
        .evaluation_report
        .arena_reports
        .iter()
        .find(|arena| arena.location_id_raw == proof_plan.arena_location_id_raw);
    let Some(arena) = arena else {
        return vec![0.0; proof_plan.surplus_plan.slot_count as usize];
    };
    let slot_count = proof_plan.surplus_plan.slot_count as usize;
    let mut values = vec![0.0f32; slot_count];
    for (slot, &index) in proof_plan.source_indices.iter().enumerate() {
        if let Some(row) = arena.participant_rows.get(index) {
            values[slot] = row.surplus as f32;
        }
    }
    values
}

pub fn recursive_local_rf_demand_tick_inputs(
    plan: &RecursiveLocalRfPlan,
    proof_plan: &RecursiveLocalRfAggregateProofPlan,
) -> Vec<f32> {
    let arena = plan
        .evaluation_report
        .arena_reports
        .iter()
        .find(|arena| arena.location_id_raw == proof_plan.arena_location_id_raw);
    let Some(arena) = arena else {
        return vec![0.0; proof_plan.demand_plan.slot_count as usize];
    };
    let slot_count = proof_plan.demand_plan.slot_count as usize;
    let mut values = vec![0.0f32; slot_count];
    for (slot, &index) in proof_plan.source_indices.iter().enumerate() {
        if let Some(row) = arena.participant_rows.get(index) {
            values[slot] = row.demand as f32;
        }
    }
    values
}

pub fn recursive_local_rf_surplus_aggregate_slot(
    proof_plan: &RecursiveLocalRfAggregateProofPlan,
) -> usize {
    proof_plan.source_indices.len().max(1)
}

pub fn recursive_local_rf_demand_aggregate_slot(
    proof_plan: &RecursiveLocalRfAggregateProofPlan,
) -> usize {
    proof_plan.source_indices.len().max(1)
}

pub fn recursive_local_rf_cpu_surplus_total(
    plan: &RecursiveLocalRfPlan,
    proof_plan: &RecursiveLocalRfAggregateProofPlan,
) -> u32 {
    let arena = plan
        .evaluation_report
        .arena_reports
        .iter()
        .find(|arena| arena.location_id_raw == proof_plan.arena_location_id_raw);
    arena
        .and_then(|arena| {
            arena.settlements.iter().find(|settlement| {
                settlement.owner_ref == proof_plan.owner_ref
                    && settlement.resource_key == proof_plan.resource_key
            })
        })
        .map(|settlement| settlement.total_surplus)
        .unwrap_or(0)
}

pub fn recursive_local_rf_cpu_demand_total(
    plan: &RecursiveLocalRfPlan,
    proof_plan: &RecursiveLocalRfAggregateProofPlan,
) -> u32 {
    let arena = plan
        .evaluation_report
        .arena_reports
        .iter()
        .find(|arena| arena.location_id_raw == proof_plan.arena_location_id_raw);
    arena
        .and_then(|arena| {
            arena.settlements.iter().find(|settlement| {
                settlement.owner_ref == proof_plan.owner_ref
                    && settlement.resource_key == proof_plan.resource_key
            })
        })
        .map(|settlement| settlement.total_demand)
        .unwrap_or(0)
}
