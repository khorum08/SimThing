//! Studio presentation wrapper for spec-owned planet child-location edit commands.

use simthing_spec::{
    apply_planet_local_grid_command, PlanetChildLocationEditError, PlanetChildLocationEditReport,
    PlanetLocalGridCommand,
};

use crate::session::StudioSession;
use crate::studio_admission_report::StudioScenarioAdmissionSummary;
use crate::studio_structural_edit::{
    rebuild_studio_session_after_authority_edit, StudioStructuralEditError,
};

#[derive(Debug, thiserror::Error)]
pub enum StudioPlanetChildLocationError {
    #[error(transparent)]
    PlanetEdit(#[from] PlanetChildLocationEditError),
    #[error(transparent)]
    Rebuild(#[from] StudioStructuralEditError),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StudioPlanetChildLocationOutcome {
    pub edit_report: PlanetChildLocationEditReport,
    pub admission_summary: StudioScenarioAdmissionSummary,
    pub planet_count: usize,
    pub structural_n4_ready: bool,
}

/// Apply a star-system local-grid command and rebuild Studio projections from authority.
pub fn studio_apply_planet_child_location_command(
    session: &mut StudioSession,
    command: PlanetLocalGridCommand,
) -> Result<StudioPlanetChildLocationOutcome, StudioPlanetChildLocationError> {
    let edit_report = apply_planet_local_grid_command(&mut session.scenario_authority, command)?;
    let outcome = rebuild_studio_session_after_authority_edit(session)?;
    Ok(StudioPlanetChildLocationOutcome {
        edit_report,
        admission_summary: outcome.admission_summary,
        planet_count: session.scenario_document.planets.len(),
        structural_n4_ready: outcome.structural_n4_ready,
    })
}
