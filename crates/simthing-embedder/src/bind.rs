//! **Bind** — band consequences/thresholds and read-only CPU observation.

pub use simthing_driver::{
    compile_crossing_consequence_session as action_band_commitments, ActionBandActiveInstance,
    ActionBandNativeLaneAdmission, CrossingConsequenceAdmissionError, CrossingConsequenceBinding,
    CrossingConsequenceSession, SessionShadowView,
};
pub use simthing_sim::{
    AggregateAlertRegistration, CostBandSemantic, ThresholdRegistry, ThresholdSemantic,
    VelocityAlertRegistration,
};

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
