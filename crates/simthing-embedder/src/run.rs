//! **Run** — one initialize/start/tick/serialize lifecycle.

use std::path::Path;
use thiserror::Error;

pub use simthing_core::ExecutionPosture;
pub use simthing_driver::{RunSummary, Scenario, SessionError, SimSession, StepOnceOutcome};
pub use simthing_spec::GameModeSpec;

#[derive(Debug, Error)]
pub enum InitializeError {
    #[error(transparent)]
    Ownership(#[from] simthing_core::OwnerBoundaryValidationError),
    #[error(transparent)]
    Session(#[from] SessionError),
}

/// Initialize the single production session after intrinsic-owner admission.
pub fn initialize(
    scenario: Scenario,
    game_mode: &GameModeSpec,
) -> Result<SimSession, InitializeError> {
    simthing_core::validate_owner_binding_boundaries(&scenario.root)?;
    Ok(SimSession::open_from_spec(scenario, game_mode)?)
}

/// Initialize the single production session with already-admitted field
/// registrations and explicit, bounded-authored Triad columns.
///
/// Admission and execution are owned by the existing driver seam; this facade
/// validates the same owner boundary as [`initialize`] and delegates directly.
pub fn initialize_with_admitted_field_sweeps(
    scenario: Scenario,
    game_mode: &GameModeSpec,
    field_sweep_registrations: Vec<crate::bind::FieldSweepRegistration>,
    triad_columns: (
        crate::bind::ColumnIndex,
        crate::bind::ColumnIndex,
        crate::bind::ColumnIndex,
    ),
    comparative_bands: crate::bind::ComparativeProjectionBands,
    authored_opt_out_reason: Option<&'static str>,
) -> Result<SimSession, InitializeError> {
    simthing_core::validate_owner_binding_boundaries(&scenario.root)?;
    Ok(SimSession::open_from_spec_with_admitted_field_sweeps(
        scenario,
        game_mode,
        field_sweep_registrations,
        triad_columns,
        comparative_bands,
        authored_opt_out_reason,
    )?)
}

/// Start/select paced or continuous scheduling over the same kernel.
pub fn start(session: &mut SimSession, posture: ExecutionPosture) -> Result<(), SessionError> {
    session.set_execution_posture(posture)
}

/// Advance exactly one production hot cycle.
pub fn tick(session: &mut SimSession) -> Result<StepOnceOutcome, SessionError> {
    session.step_once()
}

/// Serialize through the existing replay/history writer while running to the cap.
pub fn serialize(
    session: &mut SimSession,
    path: &Path,
    max_days: u32,
) -> Result<RunSummary, SessionError> {
    session.record_to_path(path, max_days)
}
