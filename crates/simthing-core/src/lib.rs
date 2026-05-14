pub mod evaluate;
pub mod ids;
pub mod overlay;
pub mod property;
pub mod registry;
pub mod simthing;

pub use ids::{OverlayId, SimPropertyId, SimThingId};
pub use overlay::{Overlay, OverlayKind, OverlayLifecycle, OverlaySource, PropertyTransformDelta};
pub use property::{
    DecayBehavior, Direction, ExpireHandler, FissionTemplate, FissionThreshold, FusionThreshold, IntensityBehavior, IntensityRange, PropertyLayout, PropertyValue, SimProperty,
    SubFieldRole, TransformOp, TransformSemantics,
};
pub use registry::{DimensionRegistry, PropertyColumnRange, SubFieldDef};
pub use simthing::{SimThing, SimThingKind};
