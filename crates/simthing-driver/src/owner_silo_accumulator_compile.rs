//! SIM-GPU-OWNER-SILO-RESOURCE-FLOW-TICK-0 — lower admitted owner-silo participants to AccumulatorOp plans.

use simthing_core::{
    AccumulatorOp, CombineFn, CompiledAccumulatorOpPlan, ConsumeMode, GateSpec, InputSpec,
    ScaleSpec, SourceSpec, StructuralScalarChannel,
};
use simthing_spec::{
    evaluate_owner_silo_flow, owner_silo_flow_participant_inputs, OwnerSiloAdmissionClassification,
    OwnerSiloAdmissionReport, OwnerSiloFlowParticipantInput, SimThingScenarioSpec, SpecError,
};

/// Driver-compiled owner-silo GPU tick plan over existing AccumulatorOp surfaces.
#[derive(Debug, Clone, PartialEq)]
pub struct OwnerSiloGpuTickPlan {
    pub surplus_plan: CompiledAccumulatorOpPlan,
    pub deficit_plan: CompiledAccumulatorOpPlan,
    pub admission: OwnerSiloAdmissionReport,
    pub participants: Vec<OwnerSiloFlowParticipantInput>,
    /// Full owner-silo state mutation (reduce-up/disburse-down writes) remains deferred.
    pub full_state_mutation_deferred: bool,
}

/// Compile admitted owner-silo flow into generic participant-sum AccumulatorOp plans.
///
/// Rejected admission refuses compilation. Participants are explicit only via
/// [`owner_silo_flow_participant_inputs`]. No owner-specific GPU primitive is introduced.
pub fn compile_owner_silo_gpu_tick_plan(
    scenario: &SimThingScenarioSpec,
) -> Result<OwnerSiloGpuTickPlan, SpecError> {
    let admission = evaluate_owner_silo_flow(scenario);
    if admission.classification == OwnerSiloAdmissionClassification::Rejected {
        return Err(SpecError::ValidationFailed);
    }
    let participants =
        owner_silo_flow_participant_inputs(scenario).map_err(|_| SpecError::ValidationFailed)?;
    if participants.is_empty() {
        return Err(SpecError::ValidationFailed);
    }

    let participant_count = participants.len() as u32;
    let surplus_plan = compile_participant_channel_sum_plan(
        participant_count,
        StructuralScalarChannel(0),
        StructuralScalarChannel(1),
    );
    let deficit_plan = compile_participant_channel_sum_plan(
        participant_count,
        StructuralScalarChannel(0),
        StructuralScalarChannel(1),
    );

    Ok(OwnerSiloGpuTickPlan {
        surplus_plan,
        deficit_plan,
        admission,
        participants,
        full_state_mutation_deferred: true,
    })
}

pub(crate) fn compile_participant_channel_sum_plan(
    participant_count: u32,
    input_channel: StructuralScalarChannel,
    output_channel: StructuralScalarChannel,
) -> CompiledAccumulatorOpPlan {
    let aggregate_slot = participant_count;
    let slot_count = participant_count + 1;
    let n_dims = input_channel.0.max(output_channel.0) + 1;
    let inputs: Vec<InputSpec> = (0..participant_count)
        .map(|slot| InputSpec {
            slot,
            col: input_channel.0,
            unit_cost: 1.0,
        })
        .collect();

    let ops = vec![AccumulatorOp {
        source: SourceSpec::ConjunctiveCrossing { inputs },
        combine: CombineFn::Sum,
        gate: GateSpec::Always,
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::AddToTarget,
        targets: vec![(aggregate_slot, output_channel.0)],
    }];

    CompiledAccumulatorOpPlan {
        slot_count,
        n_dims,
        input_channel,
        output_channel,
        ops,
    }
}

/// Per-slot surplus inputs for resident accumulator tick (aggregate slot seeded to zero).
pub fn owner_silo_surplus_tick_inputs(plan: &OwnerSiloGpuTickPlan) -> Vec<f32> {
    let slot_count = plan.surplus_plan.slot_count as usize;
    let mut values = vec![0.0f32; slot_count];
    for (slot, participant) in plan.participants.iter().enumerate() {
        values[slot] = participant.surplus as f32;
    }
    values
}

/// Per-slot deficit inputs for resident accumulator tick (aggregate slot seeded to zero).
pub fn owner_silo_deficit_tick_inputs(plan: &OwnerSiloGpuTickPlan) -> Vec<f32> {
    let slot_count = plan.deficit_plan.slot_count as usize;
    let mut values = vec![0.0f32; slot_count];
    for (slot, participant) in plan.participants.iter().enumerate() {
        values[slot] = participant.deficit as f32;
    }
    values
}

/// CPU oracle total of admitted participant surplus amounts.
pub fn owner_silo_participant_surplus_total(plan: &OwnerSiloGpuTickPlan) -> u32 {
    plan.participants.iter().map(|p| p.surplus).sum()
}

/// CPU oracle total of admitted participant deficit amounts.
pub fn owner_silo_participant_deficit_total(plan: &OwnerSiloGpuTickPlan) -> u32 {
    plan.participants.iter().map(|p| p.deficit).sum()
}

/// Aggregate-slot output index for participant-sum accumulator plans.
pub fn owner_silo_aggregate_slot(plan: &OwnerSiloGpuTickPlan) -> usize {
    plan.participants.len()
}
