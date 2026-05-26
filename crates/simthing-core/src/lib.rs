pub mod accumulator_op;
pub mod eml_nodes;
pub mod eml_registry;
pub mod evaluate;
pub mod ids;
pub mod intensity_eml;
pub mod overlay;
pub mod property;
pub mod reduction;
pub mod registry;
pub mod simthing;

pub use accumulator_op::{
    AccumulatorOp, AccumulatorOpError, CombineFn, ConsumeMode, GateSpec, InputSpec, ScaleSpec,
    SoftAggregateGuard, SourceSpec, ThresholdDirection,
};
pub use eml_nodes::{opcode as eml_opcode, EML_STACK_MAX};
pub use eml_registry::{
    classify_legacy_tree_meta, EmlConsumerKind, EmlConsumerMask, EmlExecutionClass,
    EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlRegistryError, EmlTreeId, EmlTreeMeta,
    MAX_EML_TREE_NODES, WHITELISTED_FORMULA_CLASSES,
};
pub use ids::{OverlayId, SimPropertyId, SimThingId};
pub use intensity_eml::{
    compile_intensity_behavior_to_eml, intensity_eml_direct_cpu, intensity_tree_id,
    INTENSITY_EML_TREE_ID_BASE,
};
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
