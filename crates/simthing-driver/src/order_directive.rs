//! ORDER-WEIGHT-CLASS-0 — typed class-bound operator directive submission.
//!
//! Resolves an authored order-weight class id into an ordinary
//! `OverlaySource::Player` Transient overlay and submits it through the
//! existing player-intent feeder path. No second queue or execution mechanism.

use simthing_core::{
    DissolveCondition, Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource,
    PropertyTransformDelta, SimPropertyId, SimThingId, SubFieldRole, TransformOp,
};
use simthing_feeder::FeederSender;
use simthing_spec::{
    validate_runtime_player_overlay_magnitude, OrderWeightClassSpec, SpecError,
};
use thiserror::Error;

/// Request to submit a class-bound destination/order directive.
#[derive(Clone, Debug)]
pub struct OrderDirectiveRequest {
    /// Authored class id from the admitted order-weight class table.
    pub class_id: String,
    /// Host that receives the Player overlay (e.g. ordered destination leaf).
    pub target: SimThingId,
    /// Weight/need property locus (must already exist on the target host).
    pub property_id: SimPropertyId,
    /// Sub-field role on the weight/need property (typically Named("weight")).
    pub sub_field: SubFieldRole,
    /// Declarative arrival (or other) dissolution condition at a generation boundary.
    pub dissolve: DissolveCondition,
}

#[derive(Debug, Error)]
pub enum OrderDirectiveError {
    #[error("unknown order_weight_class `{class_id}`")]
    UnknownClass { class_id: String },
    #[error("order-weight admission: {0}")]
    Admission(#[from] SpecError),
    #[error("feeder disconnected")]
    FeederDisconnected,
}

/// Resolve `class_id` against the admitted class table and build the ordinary
/// Player Transient overlay (price injection). Does not submit.
pub fn build_order_directive_overlay(
    classes: &[OrderWeightClassSpec],
    req: &OrderDirectiveRequest,
) -> Result<(Overlay, f32), OrderDirectiveError> {
    let class = classes
        .iter()
        .find(|c| c.id == req.class_id)
        .ok_or_else(|| OrderDirectiveError::UnknownClass {
            class_id: req.class_id.clone(),
        })?;

    let overlay = Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Instruction,
        source: OverlaySource::Player,
        affects: vec![req.target],
        transform: PropertyTransformDelta {
            property_id: req.property_id,
            sub_field_deltas: vec![(req.sub_field.clone(), TransformOp::Add(class.magnitude))],
        },
        lifecycle: OverlayLifecycle::Transient {
            dissolution_conditions: vec![req.dissolve.clone()],
        },
    };
    Ok((overlay, class.magnitude))
}

/// Build and submit a class-bound order directive through the ordinary player-intent feeder.
pub fn submit_order_directive(
    tx: &FeederSender,
    classes: &[OrderWeightClassSpec],
    req: OrderDirectiveRequest,
) -> Result<OverlayId, OrderDirectiveError> {
    let (overlay, _magnitude) = build_order_directive_overlay(classes, &req)?;
    let id = overlay.id;
    // Class-resolved path is the sole sanctioned constructor for dominant
    // Player weights. Bypass attempts use raw submit + runtime magnitude gate.
    tx.submit_player_intent(req.target, overlay)
        .map_err(|_| OrderDirectiveError::FeederDisconnected)?;
    Ok(id)
}

/// Gate a raw Player overlay against the class table (no class id on core Overlay).
/// Dominant magnitudes must use [`submit_order_directive`].
pub fn gate_raw_player_overlay(
    overlay: &Overlay,
    classes: &[OrderWeightClassSpec],
) -> Result<(), OrderDirectiveError> {
    let mags: Vec<f32> = overlay
        .transform
        .sub_field_deltas
        .iter()
        .map(|(_, op)| match op {
            TransformOp::Add(v) | TransformOp::Multiply(v) | TransformOp::Set(v) => *v,
        })
        .collect();
    validate_runtime_player_overlay_magnitude(overlay.source.clone(), &mags, classes)?;
    Ok(())
}
