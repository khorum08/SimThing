//! PLANET-CHILD-RF-GPU-PARTICIPANT-0 — lower planet/non-grid child RF participants to AccumulatorOp.

use simthing_core::{CompiledAccumulatorOpPlan, StructuralScalarChannel};
use simthing_spec::{
    admit_intrinsic_owner_channels, evaluate_planet_child_rf_admission_from_owner_view,
    planet_child_rf_participant_inputs_from_owner_view, IntrinsicOwnerChannelView,
    PlanetChildRfAdmissionClassification, PlanetChildRfAdmissionReport,
    PlanetChildRfParticipantInput, SimThingScenarioSpec, SpecError,
};

use crate::owner_silo_accumulator_compile::compile_participant_channel_sum_plan;

/// Driver-compiled planet child RF GPU tick plan over existing AccumulatorOp surfaces.
#[derive(Debug, Clone, PartialEq)]
pub struct PlanetChildRfGpuTickPlan {
    pub surplus_plan: CompiledAccumulatorOpPlan,
    pub deficit_plan: CompiledAccumulatorOpPlan,
    pub admission: PlanetChildRfAdmissionReport,
    pub participants: Vec<PlanetChildRfParticipantInput>,
    /// Full owner-silo state mutation (reduce-up/disburse-down writes) remains deferred.
    pub full_state_mutation_deferred: bool,
}

/// Compile admitted planet/non-grid child RF participants into generic participant-sum plans.
pub fn compile_planet_child_rf_gpu_tick_plan(
    scenario: &SimThingScenarioSpec,
) -> Result<PlanetChildRfGpuTickPlan, SpecError> {
    let owner_view =
        admit_intrinsic_owner_channels(scenario).map_err(|_| SpecError::ValidationFailedAt {
            site: "simthing-driver/planet_child_rf_accumulator_compile",
        })?;
    compile_planet_child_rf_gpu_tick_plan_from_owner_view(&owner_view)
}

pub fn compile_planet_child_rf_gpu_tick_plan_from_owner_view(
    owner_view: &IntrinsicOwnerChannelView,
) -> Result<PlanetChildRfGpuTickPlan, SpecError> {
    let admission = evaluate_planet_child_rf_admission_from_owner_view(owner_view);
    if admission.classification == PlanetChildRfAdmissionClassification::Rejected {
        return Err(SpecError::ValidationFailedAt {
            site: "simthing-driver/planet_child_rf_accumulator_compile",
        });
    }
    let participants =
        planet_child_rf_participant_inputs_from_owner_view(owner_view).map_err(|_| {
            SpecError::ValidationFailedAt {
                site: "simthing-driver/planet_child_rf_accumulator_compile",
            }
        })?;
    if participants.is_empty() {
        return Err(SpecError::ValidationFailedAt {
            site: "simthing-driver/planet_child_rf_accumulator_compile",
        });
    }

    let participant_count = participants.len() as u32;
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

    Ok(PlanetChildRfGpuTickPlan {
        surplus_plan,
        deficit_plan,
        admission,
        participants,
        full_state_mutation_deferred: true,
    })
}

pub fn planet_child_rf_surplus_tick_inputs(plan: &PlanetChildRfGpuTickPlan) -> Vec<f32> {
    let slot_count = plan.surplus_plan.slot_count as usize;
    let mut values = vec![0.0f32; slot_count];
    for (slot, participant) in plan.participants.iter().enumerate() {
        values[slot] = participant.surplus as f32;
    }
    values
}

pub fn planet_child_rf_deficit_tick_inputs(plan: &PlanetChildRfGpuTickPlan) -> Vec<f32> {
    let slot_count = plan.deficit_plan.slot_count as usize;
    let mut values = vec![0.0f32; slot_count];
    for (slot, participant) in plan.participants.iter().enumerate() {
        values[slot] = participant.deficit as f32;
    }
    values
}

pub fn planet_child_rf_participant_surplus_total(plan: &PlanetChildRfGpuTickPlan) -> u32 {
    plan.participants.iter().map(|p| p.surplus).sum()
}

pub fn planet_child_rf_participant_deficit_total(plan: &PlanetChildRfGpuTickPlan) -> u32 {
    plan.participants.iter().map(|p| p.deficit).sum()
}

pub fn planet_child_rf_aggregate_slot(plan: &PlanetChildRfGpuTickPlan) -> usize {
    plan.participants.len()
}
