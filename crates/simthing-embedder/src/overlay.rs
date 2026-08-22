//! **Overlay** — attributable authored change with a definable horizon.

pub use simthing_core::{
    DissolveCondition, Overlay, OverlayKind, OverlaySource, PropertyTransformDelta,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OverlayDoorError {
    #[error("overlay origin is not admitted by the supplied authority tree: {0}")]
    Origin(#[from] simthing_core::OwnerResolutionError),
    #[error(transparent)]
    Lifecycle(#[from] simthing_core::DispatchOverlayError),
}

/// Author the only dispatch-minted overlay shape exposed by the Vendor Door.
///
/// The caller supplies the originating `SimThing`; a foreign/synthesized node
/// fails against `authority_root`. Empty horizons fail through the graduated
/// dispatch lifecycle admission.
///
/// A bare synthesized id cannot cross the type boundary as an origin:
///
/// ```compile_fail,E0308
/// use simthing_core::{OverlayKind, OverlaySource, PropertyTransformDelta,
///     SimPropertyId, SimThing, SimThingId, SimThingKind};
/// use simthing_embedder::overlay::authored;
/// let root = SimThing::new(SimThingKind::Custom("root".into()), 0);
/// let transform = PropertyTransformDelta {
///     property_id: SimPropertyId(1),
///     sub_field_deltas: Vec::new(),
/// };
/// let _ = authored(
///     &root,
///     SimThingId::new(),
///     OverlayKind::Instruction,
///     OverlaySource::System,
///     vec![root.id],
///     transform,
///     vec![simthing_core::DissolveCondition::AtSessionEnd],
/// );
/// ```
pub fn authored(
    authority_root: &simthing_core::SimThing,
    origin: &simthing_core::SimThing,
    kind: OverlayKind,
    source: OverlaySource,
    affects: Vec<simthing_core::SimThingId>,
    transform: PropertyTransformDelta,
    dissolution_conditions: Vec<DissolveCondition>,
) -> Result<Overlay, OverlayDoorError> {
    simthing_core::resolve_owner(authority_root, origin.id)?;
    let lifecycle = simthing_core::dispatch_until_dissolved(dissolution_conditions)?;
    let overlay = Overlay {
        id: simthing_core::OverlayId::new(),
        kind,
        source,
        origin: origin.id,
        affects,
        transform,
        lifecycle,
    };
    simthing_core::admit_dispatch_minted_overlay(&overlay)?;
    Ok(overlay)
}
