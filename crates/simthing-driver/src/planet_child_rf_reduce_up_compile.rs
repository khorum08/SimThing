//! PLANET-CHILD-RF-REDUCE-UP-0 — scoped reduce-up proof over existing AccumulatorOp surfaces.

use simthing_core::{CompiledAccumulatorOpPlan, StructuralScalarChannel};
use simthing_spec::{
    admit_intrinsic_owner_channels, evaluate_planet_child_rf_reduce_up_from_owner_view,
    planet_child_rf_participant_inputs_from_owner_view, scope_key_from_participant,
    IntrinsicOwnerChannelView, OwnerChannelScopeKey, PlanetChildRfAdmissionClassification,
    PlanetChildRfParticipantInput, PlanetChildRfReduceUpReport, SimThingScenarioSpec, SpecError,
};

use crate::owner_silo_accumulator_compile::compile_participant_channel_sum_plan;

/// Per-scope AccumulatorOp plans over participant indices in that bucket.
#[derive(Debug, Clone, PartialEq)]
pub struct PlanetChildRfBucketAccumulatorPlan {
    pub scope: OwnerChannelScopeKey,
    pub participant_indices: Vec<usize>,
    pub surplus_plan: CompiledAccumulatorOpPlan,
    pub deficit_plan: CompiledAccumulatorOpPlan,
}

/// Driver proof plan for scoped planet child RF reduce-up.
#[derive(Debug, Clone, PartialEq)]
pub struct PlanetChildRfReduceUpGpuProofPlan {
    pub reduce_up_report: PlanetChildRfReduceUpReport,
    pub participants: Vec<PlanetChildRfParticipantInput>,
    pub bucket_plans: Vec<PlanetChildRfBucketAccumulatorPlan>,
    /// Full owner-silo state mutation and disburse-down remain deferred.
    pub full_state_mutation_deferred: bool,
}

/// Compile scoped reduce-up buckets into per-bucket participant-sum AccumulatorOp plans.
pub fn compile_planet_child_rf_reduce_up_gpu_proof_plan(
    scenario: &SimThingScenarioSpec,
) -> Result<PlanetChildRfReduceUpGpuProofPlan, SpecError> {
    let owner_view =
        admit_intrinsic_owner_channels(scenario).map_err(|_| SpecError::ValidationFailedAt {
            site: "simthing-driver/planet_child_rf_reduce_up_compile",
        })?;
    compile_planet_child_rf_reduce_up_gpu_proof_plan_from_owner_view(&owner_view)
}

pub fn compile_planet_child_rf_reduce_up_gpu_proof_plan_from_owner_view(
    owner_view: &IntrinsicOwnerChannelView,
) -> Result<PlanetChildRfReduceUpGpuProofPlan, SpecError> {
    let reduce_up_report = evaluate_planet_child_rf_reduce_up_from_owner_view(owner_view);
    if reduce_up_report.classification == PlanetChildRfAdmissionClassification::Rejected {
        return Err(SpecError::ValidationFailedAt {
            site: "simthing-driver/planet_child_rf_reduce_up_compile",
        });
    }
    if !reduce_up_report.errors.is_empty() {
        return Err(SpecError::ValidationFailedAt {
            site: "simthing-driver/planet_child_rf_reduce_up_compile",
        });
    }

    let participants =
        planet_child_rf_participant_inputs_from_owner_view(owner_view).map_err(|_| {
            SpecError::ValidationFailedAt {
                site: "simthing-driver/planet_child_rf_reduce_up_compile",
            }
        })?;
    if participants.is_empty() || reduce_up_report.buckets.is_empty() {
        return Err(SpecError::ValidationFailedAt {
            site: "simthing-driver/planet_child_rf_reduce_up_compile",
        });
    }

    let mut bucket_plans = Vec::with_capacity(reduce_up_report.buckets.len());
    for bucket in &reduce_up_report.buckets {
        let participant_indices = participants
            .iter()
            .enumerate()
            .filter(|(_, participant)| scope_key_from_participant(participant) == bucket.scope)
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        if participant_indices.is_empty() {
            return Err(SpecError::ValidationFailedAt {
                site: "simthing-driver/planet_child_rf_reduce_up_compile",
            });
        }

        let participant_count = participant_indices.len() as u32;
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
        bucket_plans.push(PlanetChildRfBucketAccumulatorPlan {
            scope: bucket.scope.clone(),
            participant_indices,
            surplus_plan,
            deficit_plan,
        });
    }

    Ok(PlanetChildRfReduceUpGpuProofPlan {
        reduce_up_report,
        participants,
        bucket_plans,
        full_state_mutation_deferred: true,
    })
}

pub fn planet_child_rf_reduce_up_bucket_surplus_tick_inputs(
    plan: &PlanetChildRfReduceUpGpuProofPlan,
    bucket_plan: &PlanetChildRfBucketAccumulatorPlan,
) -> Vec<f32> {
    let slot_count = bucket_plan.surplus_plan.slot_count as usize;
    let mut values = vec![0.0f32; slot_count];
    for (slot, &participant_index) in bucket_plan.participant_indices.iter().enumerate() {
        values[slot] = plan.participants[participant_index].surplus as f32;
    }
    values
}

pub fn planet_child_rf_reduce_up_bucket_deficit_tick_inputs(
    plan: &PlanetChildRfReduceUpGpuProofPlan,
    bucket_plan: &PlanetChildRfBucketAccumulatorPlan,
) -> Vec<f32> {
    let slot_count = bucket_plan.deficit_plan.slot_count as usize;
    let mut values = vec![0.0f32; slot_count];
    for (slot, &participant_index) in bucket_plan.participant_indices.iter().enumerate() {
        values[slot] = plan.participants[participant_index].deficit as f32;
    }
    values
}

pub fn planet_child_rf_reduce_up_bucket_aggregate_slot(
    bucket_plan: &PlanetChildRfBucketAccumulatorPlan,
) -> usize {
    bucket_plan.participant_indices.len()
}

pub fn planet_child_rf_reduce_up_bucket_cpu_surplus_total(
    plan: &PlanetChildRfReduceUpGpuProofPlan,
    bucket_plan: &PlanetChildRfBucketAccumulatorPlan,
) -> u32 {
    bucket_plan
        .participant_indices
        .iter()
        .map(|&index| plan.participants[index].surplus)
        .sum()
}

pub fn planet_child_rf_reduce_up_bucket_cpu_deficit_total(
    plan: &PlanetChildRfReduceUpGpuProofPlan,
    bucket_plan: &PlanetChildRfBucketAccumulatorPlan,
) -> u32 {
    bucket_plan
        .participant_indices
        .iter()
        .map(|&index| plan.participants[index].deficit)
        .sum()
}
