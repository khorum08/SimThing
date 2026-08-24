//! **Bind** — band consequences/thresholds and read-only CPU observation.

pub use simthing_driver::{
    compile_crossing_consequence_session as action_band_commitments,
    compile_gu_yang_n4_field_sweeps, compile_palma_n4_field_sweep, ActionBandActiveInstance,
    ActionBandNativeLaneAdmission, ComparativeProjectionBands, CrossingConsequenceAdmissionError,
    CrossingConsequenceBinding, CrossingConsequenceSession, GuYangN4FieldSweepSpec,
    GuYangStallOutputs, PalmaN4FieldSweepSpec, SessionShadowView,
};
pub use simthing_gpu::{FieldSweepAdmissionError, FieldSweepRegistration};
pub use simthing_sim::{
    AggregateAlertRegistration, CostBandSemantic, ThresholdRegistry, ThresholdSemantic,
    VelocityAlertRegistration,
};

pub use simthing_core::{AuthoredColumnAdmitError, ColumnIndex, SlotIndex};

/// One read-only row copied from the existing Gu-Yang comparative outputs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GuYangStallObservation {
    pub row: u32,
    pub net_flux: f32,
    pub gross_flux: f32,
    pub stall: f32,
}

/// Admit a vendor-authored matrix column through the bounded authored door.
///
/// The Vendor Door cannot construct the tuple field directly:
///
/// ```compile_fail,E0423
/// use simthing_embedder::bind::ColumnIndex;
/// let _forged = ColumnIndex(0);
/// ```
pub fn authored_column(raw: u32, bound: u32) -> Result<ColumnIndex, AuthoredColumnAdmitError> {
    ColumnIndex::try_from_admitted_authored(raw, bound)
}

/// Register a values-plane threshold on the ordinary session builder path.
pub fn velocity_threshold(
    session: &mut simthing_driver::SimSession,
    registration: VelocityAlertRegistration,
) {
    session.proto.register_velocity_alert(registration);
}

/// Register an aggregate threshold on the ordinary session builder path.
pub fn aggregate_threshold(
    session: &mut simthing_driver::SimSession,
    registration: AggregateAlertRegistration,
) {
    session.proto.register_aggregate_alert(registration);
}

/// Resolve a runtime queued draw through the production CostBand registry.
pub fn queued_draw(
    registry: &mut ThresholdRegistry,
    event_kind: u32,
    available: f32,
    authored_unit_cost: f32,
) -> Result<simthing_core::CostBandDraw, simthing_core::CostBandAdmissionError> {
    registry.resolve_cost_band_draw(event_kind, available, authored_unit_cost)
}

/// Borrow the session's generation-coherent, read-only CPU shadow.
pub fn shadow(session: &simthing_driver::SimSession) -> SessionShadowView<'_> {
    session.shadow_view()
}

/// Copy the live comparative stall lanes for observation only.
///
/// This delegates column identity to the admitted [`GuYangStallOutputs`] on
/// [`simthing_driver::SpecSessionState`] and values to the existing ordinary
/// mapping readback. The result has no write, submission, or decision door.
pub fn observe_gu_yang_stall(
    session: &simthing_driver::SimSession,
) -> Option<Vec<GuYangStallObservation>> {
    let outputs = session
        .spec_state
        .comparative_projection
        .as_ref()?
        .stall_outputs;
    let mapping = session.mapping.as_ref()?;
    let n_dims = session.state.n_dims as usize;
    let values = mapping
        .hot
        .mapping
        .readback_canonical_field(&session.state.ctx);
    Some(
        values
            .chunks_exact(n_dims)
            .enumerate()
            .map(|(row, values)| GuYangStallObservation {
                row: row as u32,
                net_flux: values[outputs.net_flux_col.raw()],
                gross_flux: values[outputs.gross_flux_col.raw()],
                stall: values[outputs.stall_col.raw()],
            })
            .collect(),
    )
}
