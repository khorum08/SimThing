pub mod accumulator_op;
pub mod accumulator_op_builder;
pub mod accumulator_spec;
pub mod anchor_remap;
pub mod anchor_table;
pub mod arena_layout;
pub mod automaton;
pub mod column_index;
pub mod compiled_accumulator_plan;
pub mod cost_band;
pub mod eml_exp;
pub mod eml_ln;
pub mod eml_nodes;
pub mod eml_registry;
pub mod evaluate;
pub mod execution_posture;
pub mod fission_child_spawn;
pub mod fission_clone_source;
pub mod generation_stamp;
pub mod grant_lifecycle;
pub mod ids;
pub mod intensity_eml;
pub mod overlay;
pub mod overlay_lifecycle_deadline;
pub mod owner_channel;
pub mod persistence_deformation;
pub mod placed_participant;
pub mod property;
pub mod reduction;
pub mod registry;
pub mod residency;
pub mod residency_tier;
pub mod simthing;
pub mod slot_index;
pub mod specialization;
pub mod structural_coord;
pub mod tree_execution_context;

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
pub use anchor_remap::{
    derive_epoch_rebind_section, derive_exact_anchor_remaps, expected_anchored_remap_keys,
    expected_epoch_rebind_row_moves, resolve_slot_through_chain, validate_anchor_remap_for_encode,
    validate_epoch_rebind_section, validate_exact_anchor_remap_endpoints, AnchorLocusRemap,
    AnchorRemapEncodeError, AnchorRemapOperation, AnchorRemapSection, AnchoredLocusMap,
    BindingTableSnapshot, RemapKey, RemapSubject,
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
pub use automaton::{
    capture_ancestor_standing_policy, deliver_deficit_directive, deliver_predicate_broadcast,
    deliver_routed_overlay, deliver_standing_directive, inherit_active_overlays,
    overlay_origin_structural_coord, DirectiveDeliveryReceipt, LiveOverlayRoutes,
    OverlayDeliveryError,
};
pub use column_index::{AuthoredColumnAdmitError, ColumnIndex};
pub use compiled_accumulator_plan::{
    is_exact_integer_f32, CompiledAccumulatorOpPlan, StructuralScalarChannel,
    EXACT_INTEGER_F32_BOUND,
};
pub use cost_band::{
    admit_cost_band_marker, cost_band_depth_one, cost_band_expected_n, cost_band_quantize,
    CostBandAdmissionError, CostBandDraw, CostBandRegistrationMarker, CostBandResourceMarker,
};
pub use eml_exp::{
    eml_exp_pinned_f32, EML_EXP_ALGORITHM_IDENTITY, EML_EXP_DOMAIN_MAX, EML_EXP_DOMAIN_MAX_BITS,
    EML_EXP_DOMAIN_MIN, EML_EXP_DOMAIN_MIN_BITS, EML_EXP_SATURATION_CEILING_BITS,
    EML_EXP_SEQUENCE_VERSION,
};
pub use eml_nodes::{opcode as eml_opcode, EmlResourceClass, EML_STACK_MAX};
pub use eml_registry::{
    classify_legacy_tree_meta, EmlConsumerKind, EmlConsumerMask, EmlExecutionClass,
    EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlRegistryError, EmlTreeId, EmlTreeMeta,
    MAX_EML_TREE_NODES, WHITELISTED_FORMULA_CLASSES,
};
pub use evaluate::{RoutedPredicate, RoutedPredicateComparison, TransformStack};
pub use execution_posture::{ClearingExecutionPosture, ExecutionPosture, ExecutionPostureError};
pub use fission_child_spawn::ResolvedFissionChildBlueprint;
pub use fission_clone_source::{
    fission_clone_source_container_kinds_for_registry, fission_clone_source_label,
    is_fission_clone_source, prep_fission_clone_source_labels,
    prepare_fission_clone_sources_for_registry, prepare_fission_clone_sources_subtree,
    stamp_fission_clone_source_label, FissionCloneSourceLabel, FISSION_CLONE_SOURCE_PROPERTY_ID,
};
pub use generation_stamp::{
    admit_dispatch_minted_overlay, dispatch_until_dissolved, integrate_stamped_product,
    integrate_unstamped_product_forbidden, replay_integration_schedule, replay_standing_views,
    AncestorStandingPolicyView, AuthoredSeamStaleness, BackpressurePolicy, DispatchOverlayError,
    GenerationStamp, GenerationStamped, GrantLifecycleScheduleError, IntegrateError,
    IntegrationReceipt, IntegrationSchedule, IntegrationScheduleEntry, IntegrationScheduleRowKind,
    ResidentClearingScheduleFact, ResidentScheduleError, ResidentScheduleReservation,
    RingPushOutcome, RoutedGenerationDuration, StampedEgressEntry, StampedEventRing,
    StandingViewDoubleBuffer,
};
pub use grant_lifecycle::{
    grant_disbursement_capacity_overlay, grant_disbursement_capacity_property,
    grant_disbursement_capacity_value, GrantLifecycleFact, GrantLifecycleFactKind,
    GrantLifecycleRelationshipState, GrantLifecycleReleaseCause, GRANT_DISBURSEMENT_NAMESPACE,
    GRANT_DISBURSEMENT_PROPERTY, GRANT_LANE_CAPACITY, GRANT_LANE_FREE, GRANT_LANE_IN_FLIGHT,
    GRANT_LANE_OCCUPIED,
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
pub use overlay_lifecycle_deadline::{
    admit_dissolve_conditions, admit_overlay_lifecycle, establish_overlay_deadline,
    rebase_routed_overlay_duration, OverlayLifecycleAdmitError,
};
pub use owner_channel::{
    bind_owner, declared_owner, is_ownership_crossing, resolve_owner, resolve_owners_in_order,
    unbind_owner, unowned, validate_owner_binding_boundaries, AuthoredOwnerRefError,
    OwnerBoundaryValidationError, OwnerInternError, OwnerInterner, OwnerLayoutId, OwnerRef,
    OwnerResolutionError, OWNER_CHANNEL_PROPERTY_ID, UNOWNED_OWNER_REF,
};
pub use persistence_deformation::{
    PersistenceDeformationAdmissionError, PersistenceDeformationError,
    PersistenceDeformationProgram, MAX_EXACT_PERSISTENCE_DEFORMATION_CAP,
};
pub use placed_participant::{
    validate_and_mint_placed_participants_by_location_id,
    validate_location_ids_have_structural_placements, PlacedParticipant,
    PlacedParticipantValidationError, StructuralGridPlacement,
};
pub use property::{
    admit_overlay_eml_program, eval_overlay_eml, logistic_steering_eml_nodes,
    logistic_steering_oracle, magnitude_band_eml_nodes, ClampBehavior, DecayBehavior, Direction,
    EmlPerProgramCap, EmlPerProgramCapError, ExpireEffect, ExpireHandler, FissionTemplate,
    FissionThreshold, FusionThreshold, IntensityBehavior, IntensityRange,
    PropertyAdmissionDisposition, PropertyLayout, PropertyValue, RoleOffset, SecondaryCondition,
    SimProperty, SimThingKindTag, SubFieldRole, SubFieldSpec, TransformOp,
};
pub use reduction::ReductionRule;
pub use registry::{
    DimensionRegistry, PropertyAdmissionReport, PropertyColumnRange, ResourcePropertyDispositionRow,
};
pub use residency::{ObjectResidencyRelation, ObjectResidencyRelease, ObjectResidencyRequest};
pub use residency_tier::{
    materialize_granting_census, resolve_residency_draw, AdjacencyParticipation,
    CapacityPartitionError, GrantingNodeCensusLanes, LaneSet, ResidencyCapacityPartition,
    ResidencyChurnClass, ResidencyDrawShape, ResidencyShapeClass, ResidencyTierRow, SessionTierSet,
    SparseGrantingCensus, TierAdmissionError, TierId, SESSION_TIER_WIDTH_LIMIT,
};
pub use simthing::{
    kind_matches, reserve_simthing_ids_from_tree, ResourceParentEdge, SimThing, SimThingKind,
};
pub use slot_index::{CellSpaceIndex, SlotIndex};
pub use specialization::{
    derive_specializations, kind_identity, query_owner_specializations, seed_profiles,
    DeclaredSpecialization, KindIdentity, OwnerSpecializationRow, SpecializationError,
    SpecializationObservations, SpecializationProfile, SpecializationReport,
    SpecializationRequirement, SpecializationRow, PROFILE_OWNER_SEAT, PROFILE_SESSION_ROOT,
    PROFILE_SPATIAL,
};
pub use structural_coord::{RenderCoord, StructuralCoord};
pub use tree_execution_context::{
    ExecutionIncarnation, PersistedTreeExecutionIdentity, RealmQualified, RecordedTreeForkIdentity,
    SeamEmissionOrdinal, SeamFact, SeamFactId, TreeExecutionAuthority, TreeExecutionBinding,
    TreeExecutionContext, TreeExecutionContextError, TreeExecutionLease,
    TreeExecutionLeaseVerifier, TreeGenerationAuthority, TreeGenerationPermit, TreeIdentityError,
    TreeRealmId,
};
