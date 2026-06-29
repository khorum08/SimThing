pub mod accumulator_op;
pub mod accumulator_op_builder;
pub mod accumulator_spec;
pub mod arena_layout;
pub mod column_index;
pub mod compiled_accumulator_plan;
pub mod eml_nodes;
pub mod eml_registry;
pub mod evaluate;
pub mod fission_child_spawn;
pub mod fission_clone_source;
pub mod ids;
pub mod intensity_eml;
pub mod overlay;
pub mod property;
pub mod reduction;
pub mod registry;
pub mod simthing;
pub mod slot_index;
pub mod structural_coord;

pub use accumulator_op::{
    AccumulatorOp, AccumulatorOpError, CombineFn, ConsumeMode, GateSpec, InputSpec, ScaleSpec,
    SoftAggregateGuard, SourceSpec, ThresholdDirection,
};
pub use accumulator_op_builder::{
    column_aware_reduction_op, conjunctive_recipe_registration_to_op, debt_band_next_threshold,
    discrete_transfer_registration_to_op, emit_on_threshold, emit_on_threshold_registration_to_op,
    manual_slot_range_sum_op, rebuild_conjunctive_recipe_ops, rebuild_discrete_transfer_ops,
    rebuild_emit_on_threshold_event_kinds, rebuild_emit_on_threshold_ops,
    refresh_emit_on_threshold_debt_band, resource_transfer_discrete, try_conjunctive_recipe,
    try_resource_transfer_discrete, AccumulatorOpBuilder, AccumulatorOpBuilderError,
    ColumnAwareReductionCombine, ColumnAwareReductionSpec, ConjunctiveRecipeInput,
    ConjunctiveRecipeRegistration, DiscreteTransferRegistration, EmitOnThresholdBuffer,
    EmitOnThresholdRegistration,
};
pub use accumulator_spec::{
    AccumulatorRole, AccumulatorSpec, ArenaName, BalanceSpec, LogTier, NumCountSource,
};
pub use arena_layout::{
    arena_internal_columns_present, expand_arena_internal_columns,
    property_needs_arena_internal_columns, ARENA_INTERNAL_COLUMN_ROLES,
};
pub use column_index::ColumnIndex;
pub use compiled_accumulator_plan::{
    is_exact_integer_f32, CompiledAccumulatorOpPlan, StructuralScalarChannel,
    EXACT_INTEGER_F32_BOUND,
};
pub use eml_nodes::{opcode as eml_opcode, EML_STACK_MAX};
pub use eml_registry::{
    classify_legacy_tree_meta, EmlConsumerKind, EmlConsumerMask, EmlExecutionClass,
    EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlRegistryError, EmlTreeId, EmlTreeMeta,
    MAX_EML_TREE_NODES, WHITELISTED_FORMULA_CLASSES,
};
pub use fission_child_spawn::ResolvedFissionChildBlueprint;
pub use fission_clone_source::{
    fission_clone_source_container_kinds_for_registry, fission_clone_source_label,
    is_fission_clone_source, prep_fission_clone_source_labels,
    prepare_fission_clone_sources_for_registry, prepare_fission_clone_sources_subtree,
    stamp_fission_clone_source_label, FissionCloneSourceLabel, FISSION_CLONE_SOURCE_PROPERTY_ID,
};
pub use ids::{
    advance_simthing_id_allocator_past, OverlayId, SimPropertyId, SimThingId,
    SimThingIdReservationError,
};
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
    PropertyValue, RoleOffset, SecondaryCondition, SimProperty, SimThingKindTag, SubFieldRole,
    SubFieldSpec, TransformOp,
};
pub use reduction::ReductionRule;
pub use registry::{DimensionRegistry, PropertyColumnRange};
pub use simthing::{
    is_arena_participant_node, kind_matches, reserve_simthing_ids_from_tree, SimThing, SimThingKind,
};
pub use slot_index::SlotIndex;
pub use structural_coord::{RenderCoord, StructuralCoord};
