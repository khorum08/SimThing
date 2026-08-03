pub mod accumulator_op;
pub mod accumulator_op_builder;
pub mod accumulator_spec;
pub mod automaton;
pub mod anchor_remap;
pub mod anchor_table;
pub mod arena_layout;
pub mod column_index;
pub mod compiled_accumulator_plan;
pub mod cost_band;
pub mod eml_nodes;
pub mod eml_registry;
pub mod evaluate;
pub mod fission_child_spawn;
pub mod fission_clone_source;
pub mod generation_stamp;
pub mod owner_channel;
pub mod ids;
pub mod intensity_eml;
pub mod overlay;
pub mod placed_participant;
pub mod property;
pub mod reduction;
pub mod registry;
pub mod residency;
pub mod simthing;
pub mod slot_index;
pub mod specialization;
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
pub use automaton::{
    deliver_deficit_directive, deliver_predicate_broadcast, deliver_routed_overlay,
    deliver_standing_directive, inherit_active_overlays, overlay_origin_structural_coord,
    DirectiveDeliveryReceipt, LiveOverlayRoutes, OverlayDeliveryError,
};
pub use anchor_remap::{
    derive_exact_anchor_remaps, expected_anchored_remap_keys, validate_anchor_remap_for_encode,
    validate_exact_anchor_remap_endpoints, AnchorLocusRemap, AnchorRemapEncodeError,
    AnchorRemapOperation, AnchorRemapSection, AnchoredLocusMap,
};
pub use anchor_table::{
    apply_anchor_remaps_to_table, apply_band_crossings_to_anchor_table,
    mint_anchor_table_from_admission, refresh_anchor_table_magnitudes, AnchorIdentity, AnchorTable,
    AnchorTableRow, BandIndex,
};
pub use arena_layout::{
    arena_internal_columns_present, expand_arena_internal_columns, need_stage_role_names,
    property_needs_arena_internal_columns, ARENA_INTERNAL_COLUMN_ROLES, NEED_STAGE_MAX_PAIRS,
};
pub use cost_band::{
    admit_cost_band_marker, cost_band_depth_one, cost_band_expected_n, cost_band_quantize,
    CostBandAdmissionError, CostBandDraw, CostBandRegistrationMarker, CostBandResourceMarker,
};
pub use column_index::{AuthoredColumnAdmitError, ColumnIndex};
pub use compiled_accumulator_plan::{
    is_exact_integer_f32, CompiledAccumulatorOpPlan, StructuralScalarChannel,
    EXACT_INTEGER_F32_BOUND,
};
pub use eml_nodes::{opcode as eml_opcode, EmlResourceClass, EML_STACK_MAX};
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
pub use generation_stamp::{
    admit_dispatch_minted_overlay, dispatch_until_dissolved, integrate_stamped_product,
    integrate_unstamped_product_forbidden, replay_integration_schedule, BackpressurePolicy,
    DispatchOverlayError, GenerationStamp, GenerationStamped, IntegrateError, IntegrationReceipt,
    IntegrationSchedule, IntegrationScheduleEntry, RingPushOutcome, StampedEgressEntry,
    StampedEventRing,
};
pub use overlay::{
    DissolveCondition, Overlay, OverlayKind, OverlayLifecycle, OverlaySource,
    PropertyTransformDelta,
};
pub use evaluate::{RoutedPredicate, RoutedPredicateComparison, TransformStack};
pub use placed_participant::{
    validate_and_mint_placed_participants_by_location_id,
    validate_location_ids_have_structural_placements, PlacedParticipant,
    PlacedParticipantValidationError, StructuralGridPlacement,
};
pub use property::{
    admit_overlay_eml_program, eval_overlay_eml, magnitude_band_eml_nodes, ClampBehavior,
    DecayBehavior, Direction, EmlPerProgramCap, EmlPerProgramCapError, ExpireEffect, ExpireHandler,
    FissionTemplate, FissionThreshold, FusionThreshold, IntensityBehavior, IntensityRange,
    PropertyLayout, PropertyAdmissionDisposition, PropertyValue, RoleOffset, SecondaryCondition,
    SimProperty, SimThingKindTag, SubFieldRole, SubFieldSpec, TransformOp,
};
pub use reduction::ReductionRule;
pub use registry::{
    DimensionRegistry, PropertyAdmissionReport, PropertyColumnRange,
    ResourcePropertyDispositionRow,
};
pub use residency::{ObjectResidencyRelation, ObjectResidencyRelease, ObjectResidencyRequest};
pub use simthing::{
    kind_matches, reserve_simthing_ids_from_tree, ResourceParentEdge, SimThing, SimThingKind,
};
pub use slot_index::SlotIndex;
pub use specialization::{
    derive_specializations, kind_identity, seed_profiles, DeclaredSpecialization, KindIdentity,
    SpecializationError, SpecializationObservations, SpecializationProfile, SpecializationReport,
    SpecializationRequirement, SpecializationRow, PROFILE_OWNER_SEAT, PROFILE_SESSION_ROOT,
    PROFILE_SPATIAL,
};
pub use structural_coord::{RenderCoord, StructuralCoord};
