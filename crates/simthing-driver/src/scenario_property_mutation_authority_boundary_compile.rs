//! SCENARIO-PROPERTY-MUTATION-AUTHORITY-BOUNDARY-0 — compile plan for cloned candidate property mutation.

use simthing_spec::{
    evaluate_scenario_property_mutation_authority_boundary, RuntimeTickId,
    ScenarioPropertyMutationAuthorityBoundaryReport, ScenarioPropertyMutationSourceMode,
    SimThingScenarioSpec, SpecError,
};

use crate::runtime_participant_property_mutation_boundary_compile::{
    compile_runtime_participant_property_mutation_boundary_plan,
    RuntimeParticipantPropertyMutationBoundaryPlan,
};

/// Driver compile plan for scenario property mutation authority boundary integration.
#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioPropertyMutationAuthorityBoundaryPlan {
    pub runtime_property_mutation_boundary_plan: RuntimeParticipantPropertyMutationBoundaryPlan,
    pub scenario_property_mutation_report: ScenarioPropertyMutationAuthorityBoundaryReport,
    pub selected_source_mode: ScenarioPropertyMutationSourceMode,
    pub original_scenario_unchanged: bool,
    pub candidate_scenario_mutated: bool,
    pub candidate_property_mutation_applied: bool,
    pub input_scenario_property_mutation_deferred: bool,
    pub savefile_mutation_deferred: bool,
    pub persistent_history_deferred: bool,
    pub gpu_residency_doctrine_preserved: bool,
    pub no_new_gpu_primitive_required: bool,
    pub fused_recursive_rf_kernel_present: bool,
}

/// Compile scenario property mutation authority boundary plan without altering default paths.
pub fn compile_scenario_property_mutation_authority_boundary_plan(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
    source_mode: ScenarioPropertyMutationSourceMode,
    replay_count: u32,
) -> Result<ScenarioPropertyMutationAuthorityBoundaryPlan, SpecError> {
    if tick_id.0 == 0 {
        return Err(SpecError::ValidationFailed);
    }

    let property_view_mode = match source_mode {
        ScenarioPropertyMutationSourceMode::LegacyPlanetChildOwnerSilo => {
            simthing_spec::RuntimeParticipantPropertyMutationSourceMode::LegacyPlanetChildOwnerSilo
        }
        ScenarioPropertyMutationSourceMode::RecursiveRuntimePropertyViewSelectable => {
            simthing_spec::RuntimeParticipantPropertyMutationSourceMode::RecursiveRuntimeStateSelectable
        }
    };

    let runtime_property_mutation_boundary_plan =
        compile_runtime_participant_property_mutation_boundary_plan(
            scenario,
            tick_id,
            property_view_mode,
            replay_count,
        )?;
    let scenario_property_mutation_report = evaluate_scenario_property_mutation_authority_boundary(
        scenario,
        tick_id,
        source_mode,
        replay_count,
    )
    .map_err(|_| SpecError::ValidationFailed)?;

    Ok(ScenarioPropertyMutationAuthorityBoundaryPlan {
        selected_source_mode: scenario_property_mutation_report.selected_source_mode,
        original_scenario_unchanged: scenario_property_mutation_report.original_scenario_unchanged,
        candidate_scenario_mutated: scenario_property_mutation_report.candidate_scenario_mutated,
        candidate_property_mutation_applied: scenario_property_mutation_report
            .candidate_property_mutation_applied,
        input_scenario_property_mutation_deferred: scenario_property_mutation_report
            .input_scenario_property_mutation_deferred,
        savefile_mutation_deferred: scenario_property_mutation_report.savefile_mutation_deferred,
        persistent_history_deferred: scenario_property_mutation_report.persistent_history_deferred,
        gpu_residency_doctrine_preserved: true,
        no_new_gpu_primitive_required: true,
        fused_recursive_rf_kernel_present: false,
        runtime_property_mutation_boundary_plan,
        scenario_property_mutation_report,
    })
}
