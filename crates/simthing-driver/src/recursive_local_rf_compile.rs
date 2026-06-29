//! RECURSIVE-LOCAL-RF-GPU-RESIDENCY-REMEDIATION-0R — recursive local RF compile-plan
//! construction and GPU-compatible aggregate proof surfaces.
//!
//! GPU-residency doctrine: runtime RF aggregation lowers to flat rows/tables for AccumulatorOp
//! proof. CPU responsibilities are limited to deterministic oracle/reference validation,
//! semantic-side bookkeeping, compile-plan construction, and owner/user-facing reports.

use simthing_core::{CompiledAccumulatorOpPlan, StructuralScalarChannel};
use simthing_spec::{
    evaluate_recursive_local_rf, prove_recursive_local_rf_preserves_authority,
    recursive_local_rf_aggregate_source_rows, recursive_local_rf_arena_aggregate_totals,
    recursive_local_rf_report_matches_planet_child_compatibility_slice, OwnerRef,
    RecursiveLocalRfAggregateSourceRow, RecursiveLocalRfAuthorityProof,
    RecursiveLocalRfCompatibilityReport, RecursiveLocalRfEvaluationReport, ResourceKey,
    SimThingScenarioSpec, SpecError,
};

use crate::owner_silo_accumulator_compile::compile_participant_channel_sum_plan;

/// GPU aggregate proof plan for per-arena surplus/demand settlement totals.
#[derive(Debug, Clone, PartialEq)]
pub struct RecursiveLocalRfAggregateProofPlan {
    pub arena_location_id_raw: u32,
    pub owner_ref: OwnerRef,
    pub resource_key: ResourceKey,
    pub surplus_plan: CompiledAccumulatorOpPlan,
    pub demand_plan: CompiledAccumulatorOpPlan,
    /// Indices into `RecursiveLocalRfPlan::aggregate_source_rows`.
    pub source_indices: Vec<usize>,
}

/// Driver compile plan for recursive local RF oracle evaluation, authority proof, and GPU aggregate proof.
#[derive(Debug, Clone, PartialEq)]
pub struct RecursiveLocalRfPlan {
    pub evaluation_report: RecursiveLocalRfEvaluationReport,
    pub authority_proof: RecursiveLocalRfAuthorityProof,
    pub compatibility_report: RecursiveLocalRfCompatibilityReport,
    /// Flat GPU-compatible aggregate source rows for AccumulatorOp proof lowering.
    pub aggregate_source_rows: Vec<RecursiveLocalRfAggregateSourceRow>,
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

    let aggregate_source_rows = recursive_local_rf_aggregate_source_rows(&evaluation_report);
    let gpu_arena_aggregate_proof_plans =
        compile_recursive_rf_aggregate_proof_plans(&evaluation_report, &aggregate_source_rows)?;

    Ok(RecursiveLocalRfPlan {
        previous_rf_ladder_compatibility_preserved: evaluation_report
            .previous_rf_ladder_compatibility_preserved,
        evaluation_report,
        authority_proof,
        compatibility_report,
        aggregate_source_rows,
        gpu_arena_aggregate_proof_plans,
        scenario_authority_mutation_deferred: true,
        participant_property_mutation_deferred: true,
        semantic_execution_deferred: true,
    })
}

fn compile_recursive_rf_aggregate_proof_plans(
    report: &RecursiveLocalRfEvaluationReport,
    aggregate_source_rows: &[RecursiveLocalRfAggregateSourceRow],
) -> Result<Vec<RecursiveLocalRfAggregateProofPlan>, SpecError> {
    if report.arena_reports.is_empty() {
        return Ok(Vec::new());
    }

    let totals = recursive_local_rf_arena_aggregate_totals(report);
    let mut plans = Vec::with_capacity(totals.len());
    for ((parent_location_id, owner_ref, resource_key), (surplus_total, demand_total)) in totals {
        if surplus_total == 0 && demand_total == 0 {
            continue;
        }
        let arena_id = parent_location_id.raw();
        let matching: Vec<_> = aggregate_source_rows
            .iter()
            .enumerate()
            .filter(|(_, row)| {
                row.arena_location_id_raw == arena_id
                    && row.owner_ref == owner_ref
                    && row.resource_key == resource_key
            })
            .map(|(index, _)| index)
            .collect();
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
    let slot_count = proof_plan.surplus_plan.slot_count as usize;
    let mut values = vec![0.0f32; slot_count];
    for (slot, &index) in proof_plan.source_indices.iter().enumerate() {
        if let Some(row) = plan.aggregate_source_rows.get(index) {
            values[slot] = row.surplus as f32;
        }
    }
    values
}

pub fn recursive_local_rf_demand_tick_inputs(
    plan: &RecursiveLocalRfPlan,
    proof_plan: &RecursiveLocalRfAggregateProofPlan,
) -> Vec<f32> {
    let slot_count = proof_plan.demand_plan.slot_count as usize;
    let mut values = vec![0.0f32; slot_count];
    for (slot, &index) in proof_plan.source_indices.iter().enumerate() {
        if let Some(row) = plan.aggregate_source_rows.get(index) {
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

/// CPU validation oracle total for authoritative recursive settlement surplus.
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

/// CPU validation oracle total for authoritative recursive settlement demand.
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
