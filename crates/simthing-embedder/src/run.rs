//! **Run** — one initialize/start/tick/serialize lifecycle.
//!
//! CPU clearing remains reference-oracle vocabulary, not ordinary embedding
//! architecture. The one retained Vendor Door oracle view is explicitly named
//! [`cpu_filter_oracle`]; the former unqualified paths are sealed absent:
//!
//! ```compile_fail,E0432
//! use simthing_embedder::run::clear_constrained_claims_at_generation;
//! ```
//!
//! ```compile_fail,E0432
//! use simthing_embedder::run::clear_stamped_owner_channels;
//! ```

use std::path::Path;
use thiserror::Error;

pub use simthing_core::ExecutionPosture;
pub use simthing_driver::{
    apply_spec_delta, apply_spec_snapshot, open_replay_with_spec, read_spec_replay_file,
    GrowthEntitlementMarketBinding, LoadedReplay, ReplayOpenError, RunSummary, Scenario,
    SessionError, SimSession, SpecDelta, SpecSnapshot, StepOnceOutcome,
};
pub use simthing_gpu::{ResidencyPlacementDisposition, ResidencyPlacementOutcome};
pub use simthing_sim::{ReplayDriver, ReplayError, ReplayFrame, ReplaySnapshot};
pub use simthing_spec::{
    AuthoredClearingProgram, ClearingRemainderAuthority, ConstrainedClaim,
    ConstrainedClearingError, ConstrainedClearingResult, ConstrainedGrant, GameModeSpec,
};

/// Explicit reference-oracle vocabulary for embedding proofs.
///
/// This is a conversion-free item alias. It owns no clearing semantics, state,
/// adapter, wrapper, or runtime authority.
pub mod cpu_filter_oracle {
    pub use simthing_spec::clear_constrained_claims_at_generation;
}

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

/// Initialize the single production session with deferred admitted field
/// compilation and explicit, bounded-authored Triad columns.
///
/// Admission and execution are owned by the existing driver seam; this facade
/// validates the same owner boundary as [`initialize`] and delegates the
/// compiler so it receives the live post-admission registry width.
pub fn initialize_with_admitted_field_sweeps<F>(
    scenario: Scenario,
    game_mode: &GameModeSpec,
    compile_field_sweeps: F,
    triad_columns: (
        crate::bind::ColumnIndex,
        crate::bind::ColumnIndex,
        crate::bind::ColumnIndex,
    ),
    comparative_bands: crate::bind::ComparativeProjectionBands,
    authored_opt_out_reason: Option<&'static str>,
) -> Result<SimSession, InitializeError>
where
    F: FnOnce(
        u32,
    ) -> Result<
        Vec<crate::bind::FieldSweepRegistration>,
        crate::bind::FieldSweepAdmissionError,
    >,
{
    simthing_core::validate_owner_binding_boundaries(&scenario.root)?;
    Ok(SimSession::open_from_spec_with_admitted_field_sweeps(
        scenario,
        game_mode,
        compile_field_sweeps,
        triad_columns,
        comparative_bands,
        authored_opt_out_reason,
    )?)
}

/// Freeze one already-admitted 11.2a market binding into the graduated 11.2c
/// session path before the first tick.
pub fn install_growth_entitlement_market(
    session: &mut SimSession,
    binding: GrowthEntitlementMarketBinding,
) -> Result<(), SessionError> {
    session.install_growth_entitlement_market(binding)
}

/// Ask the existing session generation authority to realize an already-cleared
/// grant in caller-authored physical vocabulary.
pub fn realize_market_grant_residency(
    session: &mut SimSession,
    market: &crate::derive::AdmittedSpecializationFlowMarket,
    grant: &crate::derive::MarketGrantRecord,
    proposed: crate::populate::ResidencyExtent,
) -> Result<ResidencyPlacementOutcome, SessionError> {
    session.realize_market_grant_residency(market, grant, proposed)
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
