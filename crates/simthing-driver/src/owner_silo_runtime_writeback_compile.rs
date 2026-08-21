//! OWNER-SILO-RUNTIME-WRITEBACK-0 — compile scoped reduce-up into runtime owner-silo writeback plans.

use simthing_core::{CompiledAccumulatorOpPlan, StructuralScalarChannel};
use simthing_spec::{
    admit_intrinsic_owner_channels, apply_owner_silo_runtime_writeback_cpu,
    evaluate_planet_child_rf_reduce_up_from_owner_view,
    owner_silo_writeback_inputs_from_planet_child_reduce_up,
    runtime_owner_silo_states_from_scenario, IntrinsicOwnerChannelView, OwnerRef,
    PlanetChildRfAdmissionClassification, PlanetChildRfReduceUpReport, ResourceKey,
    RuntimeOwnerSiloState, RuntimeOwnerSiloWritebackInput, RuntimeOwnerSiloWritebackResult,
    SimThingScenarioSpec, SpecError,
};

use crate::owner_silo_accumulator_compile::compile_participant_channel_sum_plan;

/// GPU aggregate proof plan for owner/resource channel net totals before CPU writeback.
#[derive(Debug, Clone, PartialEq)]
pub struct OwnerSiloWritebackAggregateProofPlan {
    pub owner_ref: OwnerRef,
    pub resource_key: ResourceKey,
    pub surplus_plan: CompiledAccumulatorOpPlan,
    pub deficit_plan: CompiledAccumulatorOpPlan,
    pub source_bucket_indices: Vec<usize>,
}

/// Driver plan for runtime owner-silo writeback from planet child RF reduce-up.
#[derive(Debug, Clone, PartialEq)]
pub struct OwnerSiloRuntimeWritebackPlan {
    pub reduce_up_report: PlanetChildRfReduceUpReport,
    pub writeback_inputs: Vec<RuntimeOwnerSiloWritebackInput>,
    pub initial_owner_silos: Vec<RuntimeOwnerSiloState>,
    pub cpu_results: Vec<RuntimeOwnerSiloWritebackResult>,
    pub gpu_aggregate_proof_plans: Vec<OwnerSiloWritebackAggregateProofPlan>,
    /// Scenario authority is never mutated by runtime writeback.
    pub scenario_authority_mutation_deferred: bool,
    /// Economic disburse-down remains deferred.
    pub disburse_down_deferred: bool,
}

/// Compile reduce-up report into runtime writeback plan with CPU oracle and GPU aggregate proof.
pub fn compile_owner_silo_runtime_writeback_plan(
    scenario: &SimThingScenarioSpec,
) -> Result<OwnerSiloRuntimeWritebackPlan, SpecError> {
    let owner_view =
        admit_intrinsic_owner_channels(scenario).map_err(|_| SpecError::ValidationFailedAt {
            site: "simthing-driver/owner_silo_runtime_writeback_compile",
        })?;
    compile_owner_silo_runtime_writeback_plan_from_owner_view(&owner_view)
}

pub fn compile_owner_silo_runtime_writeback_plan_from_owner_view(
    owner_view: &IntrinsicOwnerChannelView,
) -> Result<OwnerSiloRuntimeWritebackPlan, SpecError> {
    let reduce_up_report = evaluate_planet_child_rf_reduce_up_from_owner_view(owner_view);
    if reduce_up_report.classification == PlanetChildRfAdmissionClassification::Rejected {
        return Err(SpecError::ValidationFailedAt {
            site: "simthing-driver/owner_silo_runtime_writeback_compile",
        });
    }
    if !reduce_up_report.errors.is_empty() {
        return Err(SpecError::ValidationFailedAt {
            site: "simthing-driver/owner_silo_runtime_writeback_compile",
        });
    }

    let initial_owner_silos = runtime_owner_silo_states_from_scenario(owner_view.scenario())
        .map_err(|_| SpecError::ValidationFailedAt {
            site: "simthing-driver/owner_silo_runtime_writeback_compile",
        })?;
    if initial_owner_silos.is_empty() {
        return Err(SpecError::ValidationFailedAt {
            site: "simthing-driver/owner_silo_runtime_writeback_compile",
        });
    }

    let writeback_inputs = owner_silo_writeback_inputs_from_planet_child_reduce_up(
        &reduce_up_report,
    )
    .map_err(|_| SpecError::ValidationFailedAt {
        site: "simthing-driver/owner_silo_runtime_writeback_compile",
    })?;

    let cpu_results =
        apply_owner_silo_runtime_writeback_cpu(&initial_owner_silos, &writeback_inputs).map_err(
            |_| SpecError::ValidationFailedAt {
                site: "simthing-driver/owner_silo_runtime_writeback_compile",
            },
        )?;

    let gpu_aggregate_proof_plans =
        compile_writeback_aggregate_proof_plans(&reduce_up_report, &writeback_inputs)?;

    Ok(OwnerSiloRuntimeWritebackPlan {
        reduce_up_report,
        writeback_inputs,
        initial_owner_silos,
        cpu_results,
        gpu_aggregate_proof_plans,
        scenario_authority_mutation_deferred: true,
        disburse_down_deferred: true,
    })
}

fn compile_writeback_aggregate_proof_plans(
    reduce_up: &PlanetChildRfReduceUpReport,
    writeback_inputs: &[RuntimeOwnerSiloWritebackInput],
) -> Result<Vec<OwnerSiloWritebackAggregateProofPlan>, SpecError> {
    let mut plans = Vec::with_capacity(writeback_inputs.len());
    for input in writeback_inputs {
        let source_bucket_indices = reduce_up
            .buckets
            .iter()
            .enumerate()
            .filter(|(_, bucket)| {
                bucket.scope.owner_ref == input.owner_ref
                    && bucket.scope.resource_key == input.resource_key
            })
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        if source_bucket_indices.is_empty() {
            return Err(SpecError::ValidationFailedAt {
                site: "simthing-driver/owner_silo_runtime_writeback_compile",
            });
        }

        let participant_count = source_bucket_indices.len() as u32;
        let surplus_plan = compile_participant_channel_sum_plan(
            participant_count,
            StructuralScalarChannel::INPUT,
            StructuralScalarChannel::OUTPUT,
        );
        let deficit_plan = compile_participant_channel_sum_plan(
            participant_count,
            StructuralScalarChannel::INPUT,
            StructuralScalarChannel::OUTPUT,
        );
        plans.push(OwnerSiloWritebackAggregateProofPlan {
            owner_ref: input.owner_ref.clone(),
            resource_key: input.resource_key.clone(),
            surplus_plan,
            deficit_plan,
            source_bucket_indices,
        });
    }
    Ok(plans)
}

pub fn owner_silo_writeback_aggregate_surplus_tick_inputs(
    plan: &OwnerSiloRuntimeWritebackPlan,
    proof_plan: &OwnerSiloWritebackAggregateProofPlan,
) -> Vec<f32> {
    let slot_count = proof_plan.surplus_plan.slot_count as usize;
    let mut values = vec![0.0f32; slot_count];
    for (slot, &bucket_index) in proof_plan.source_bucket_indices.iter().enumerate() {
        values[slot] = plan.reduce_up_report.buckets[bucket_index].net_surplus as f32;
    }
    values
}

pub fn owner_silo_writeback_aggregate_deficit_tick_inputs(
    plan: &OwnerSiloRuntimeWritebackPlan,
    proof_plan: &OwnerSiloWritebackAggregateProofPlan,
) -> Vec<f32> {
    let slot_count = proof_plan.deficit_plan.slot_count as usize;
    let mut values = vec![0.0f32; slot_count];
    for (slot, &bucket_index) in proof_plan.source_bucket_indices.iter().enumerate() {
        values[slot] = plan.reduce_up_report.buckets[bucket_index].net_deficit as f32;
    }
    values
}

pub fn owner_silo_writeback_aggregate_slot(
    proof_plan: &OwnerSiloWritebackAggregateProofPlan,
) -> usize {
    proof_plan.source_bucket_indices.len()
}
