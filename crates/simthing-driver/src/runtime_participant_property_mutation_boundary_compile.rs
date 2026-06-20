//! RUNTIME-PARTICIPANT-PROPERTY-MUTATION-BOUNDARY-0 — compile plan for property mutation boundary.

use simthing_spec::{
    evaluate_runtime_participant_property_mutation_boundary,
    RuntimeParticipantPropertyMutationBoundaryReport, RuntimeParticipantPropertyMutationSourceMode,
    RuntimeTickId, SimThingScenarioSpec, SpecError,
};

use crate::runtime_participant_state_mutation_compile::{
    compile_runtime_participant_state_mutation_plan, RuntimeParticipantStateMutationPlan,
};

/// Driver compile plan for runtime participant property mutation boundary integration.
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeParticipantPropertyMutationBoundaryPlan {
    pub runtime_state_mutation_plan: RuntimeParticipantStateMutationPlan,
    pub property_mutation_boundary_report: RuntimeParticipantPropertyMutationBoundaryReport,
    pub selected_source_mode: RuntimeParticipantPropertyMutationSourceMode,
    pub runtime_property_view_mutation_applied: bool,
    pub scenario_simthing_property_mutation_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
    pub persistent_history_deferred: bool,
    pub gpu_residency_doctrine_preserved: bool,
    pub no_new_gpu_primitive_required: bool,
    pub fused_recursive_rf_kernel_present: bool,
}

/// Compile runtime participant property mutation boundary plan without altering default paths.
pub fn compile_runtime_participant_property_mutation_boundary_plan(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: RuntimeParticipantPropertyMutationSourceMode,
    replay_count: u32,
) -> Result<RuntimeParticipantPropertyMutationBoundaryPlan, SpecError> {
    if tick_id.0 == 0 {
        return Err(SpecError::ValidationFailed);
    }

    let state_mutation_mode = match source_mode {
        RuntimeParticipantPropertyMutationSourceMode::LegacyPlanetChildOwnerSilo => {
            simthing_spec::RuntimeParticipantStateMutationSourceMode::LegacyPlanetChildOwnerSilo
        }
        RuntimeParticipantPropertyMutationSourceMode::RecursiveRuntimeStateSelectable => {
            simthing_spec::RuntimeParticipantStateMutationSourceMode::RecursiveDeltaPreviewSelectable
        }
    };

    let runtime_state_mutation_plan = compile_runtime_participant_state_mutation_plan(
        scenario,
        tick_id,
        state_mutation_mode,
        replay_count,
    )?;
    let property_mutation_boundary_report =
        evaluate_runtime_participant_property_mutation_boundary(
            scenario,
            tick_id,
            source_mode,
            replay_count,
        )
        .map_err(|_| SpecError::ValidationFailed)?;

    Ok(RuntimeParticipantPropertyMutationBoundaryPlan {
        selected_source_mode: property_mutation_boundary_report.selected_source_mode,
        runtime_property_view_mutation_applied: property_mutation_boundary_report
            .runtime_property_view_mutation_applied,
        scenario_simthing_property_mutation_deferred: property_mutation_boundary_report
            .scenario_simthing_property_mutation_deferred,
        scenario_authority_mutation_deferred: property_mutation_boundary_report
            .scenario_authority_mutation_deferred,
        savefile_mutation_deferred: property_mutation_boundary_report.savefile_mutation_deferred,
        persistent_history_deferred: property_mutation_boundary_report.persistent_history_deferred,
        gpu_residency_doctrine_preserved: true,
        no_new_gpu_primitive_required: true,
        fused_recursive_rf_kernel_present: false,
        runtime_state_mutation_plan,
        property_mutation_boundary_report,
    })
}
