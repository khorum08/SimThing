//! Studio presentation wrapper for spec-owned structural placement edit commands.

use simthing_spec::{
    apply_structural_placement_command, StructuralPlacementCommand, StructuralPlacementEditError,
    StructuralPlacementEditReport,
};

use crate::hydration::{studio_projection_from_scenario_authority, StudioHydrationError};
use crate::scenario_projection::{build_gpu_residency_readiness, build_structural_projection};
use crate::session::{StudioScenarioSummary, StudioSession};
use crate::studio_admission_report::{
    build_studio_admission_summary_from_spec, StudioScenarioAdmissionSummary,
};
use crate::studio_scenario_document::{
    build_studio_scenario_document_with_admission, StudioScenarioDocumentError,
};
use crate::view_model::StudioGalaxyViewModel;

#[derive(Debug, thiserror::Error)]
pub enum StudioStructuralEditError {
    #[error(transparent)]
    StructuralEdit(#[from] StructuralPlacementEditError),
    #[error("studio projection rebuild failed: {0}")]
    Hydration(#[from] StudioHydrationError),
    #[error("studio scenario document rebuild failed: {0}")]
    ScenarioDocument(#[from] StudioScenarioDocumentError),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StudioStructuralEditOutcome {
    pub edit_report: StructuralPlacementEditReport,
    pub admission_summary: StudioScenarioAdmissionSummary,
    pub gridcell_count: usize,
    pub structural_n4_ready: bool,
}

/// Apply a structural placement command and rebuild Studio projections from authority.
pub fn studio_apply_structural_placement_command(
    session: &mut StudioSession,
    command: StructuralPlacementCommand,
) -> Result<StudioStructuralEditOutcome, StudioStructuralEditError> {
    let edit_report = apply_structural_placement_command(&mut session.scenario_authority, command)?;
    let outcome = rebuild_studio_session_after_authority_edit(session)?;
    Ok(StudioStructuralEditOutcome {
        edit_report,
        admission_summary: outcome.admission_summary,
        gridcell_count: outcome.gridcell_count,
        structural_n4_ready: outcome.structural_n4_ready,
    })
}

struct RebuildSnapshot {
    admission_summary: StudioScenarioAdmissionSummary,
    gridcell_count: usize,
    structural_n4_ready: bool,
}

fn rebuild_studio_session_after_authority_edit(
    session: &mut StudioSession,
) -> Result<RebuildSnapshot, StudioStructuralEditError> {
    let scenario = &session.scenario_authority;
    let admission_summary =
        build_studio_admission_summary_from_spec(&scenario.scenario_id, scenario);
    let structural_n4_ready = admission_summary.compile_readiness.structural_n4_ready;
    let gridcell_count = scenario.structural_grid.placements.len();

    session.hydration = studio_projection_from_scenario_authority(scenario)?;
    session.view_model = StudioGalaxyViewModel::from_hydration(&session.hydration);
    session.structural_projection =
        build_structural_projection(scenario).map_err(StudioHydrationError::from)?;
    session.gpu_residency_readiness =
        build_gpu_residency_readiness(scenario, &session.structural_projection);
    session.scenario_summary = StudioScenarioSummary::from_scenario(scenario, session.report());
    session.scenario_document =
        build_studio_scenario_document_with_admission(scenario, Some(admission_summary.clone()))?;
    session.admission_summary = admission_summary.clone();

    Ok(RebuildSnapshot {
        admission_summary,
        gridcell_count,
        structural_n4_ready,
    })
}
