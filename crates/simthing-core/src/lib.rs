pub mod evaluate;
pub mod ids;
pub mod overlay;
pub mod property;
pub mod reduction;
pub mod accumulator_op;
pub mod eml_registry;
pub mod registry;
pub mod simthing;

pub use accumulator_op::{
    AccumulatorOp, AccumulatorOpError, CombineFn, ConsumeMode, GateSpec, InputSpec, ScaleSpec,
    SoftAggregateGuard, SourceSpec, ThresholdDirection,
};
pub use eml_registry::{
    EmlExpressionRegistry, EmlRegistryError, EmlTreeId, EmlTreeMeta, MAX_EML_TREE_NODES,
    WHITELISTED_FORMULA_CLASSES,
};
pub use ids::{OverlayId, SimPropertyId, SimThingId};
pub use overlay::{
    DissolveCondition, Overlay, OverlayKind, OverlayLifecycle, OverlaySource,
    PropertyTransformDelta,
};
pub use property::{
    ClampBehavior, DecayBehavior, Direction, ExpireEffect, ExpireHandler, FissionTemplate,
    FissionThreshold, FusionThreshold, IntensityBehavior, IntensityRange, PropertyLayout,
    PropertyValue, SecondaryCondition, SimProperty, SimThingKindTag, SubFieldRole, SubFieldSpec,
    TransformOp,
};
pub use reduction::ReductionRule;
pub use registry::{DimensionRegistry, PropertyColumnRange};
pub use simthing::{kind_matches, SimThing, SimThingKind};
