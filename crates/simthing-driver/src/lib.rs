pub mod action_band_consequence;
pub mod action_band_execution_compile;
pub mod action_band_semantic_shadow;
pub mod arena_allocation_oracle;
pub mod arena_allocation_plan;
pub mod arena_allocation_sync;
pub mod arena_hierarchy;
pub mod arena_pressure;
pub mod arena_registry;
pub mod atlas_0080_0;
pub mod automaton_reception;
pub mod bench_limits;
pub mod child_share_eml;
pub mod comparative_default_birth;
pub mod comparative_projection;
pub mod field_scheduler;
pub mod field_sweep_compile;
pub mod gated_rates;
pub mod growth_entitlement;
pub mod hosted_property_observation;
pub mod install;
pub mod loaded_scenario_recursive_rf_runtime_compile;
pub mod loaded_scenario_runtime_report_chain_compile;
pub mod loaded_scenario_studio_session_envelope_compile;
pub mod local_allocation_recursive_source_compile;
pub mod local_effect_application_compile;
pub mod local_effect_recursive_source_compile;
pub mod local_participant_effects_compile;
pub mod mapping_plan_compile;
pub mod mapping_runtime;
pub mod min_plus_traversal_field;
pub mod need_binding;
pub mod order_directive;
pub mod owner_channel_rf_compile;
pub mod owner_silo_accumulator_compile;
pub mod owner_silo_disburse_down_compile;
pub mod owner_silo_recursive_source_compile;
pub mod owner_silo_runtime_writeback_compile;
pub mod planet_child_rf_accumulator_compile;
pub mod planet_child_rf_reduce_up_compile;
pub mod production_path_0080_0;
pub mod recursive_local_rf_compile;
pub mod recursive_rf_reconciliation_compile;
pub mod residency_market;
pub mod resource_economy_boundary_schedule;
pub mod resource_economy_burn_in;
pub mod resource_economy_compile;
pub mod resource_economy_oracle;
pub mod resource_economy_sync;
pub mod resource_flow_burn_in;
pub mod resource_flow_compile;
pub mod resource_flow_convergence_burn_in;
pub mod resource_flow_derivation;
pub mod resource_flow_dynamic_enrollment_soak;
pub mod resource_flow_enrollment;
pub mod resource_flow_fission_enrollment;
pub mod resource_flow_preflight;
pub mod rf_conservation_oracle;
pub mod runtime_local_allocation_compile;
pub mod runtime_participant_property_mutation_boundary_compile;
pub mod runtime_participant_state_mutation_compile;
pub mod runtime_rf_tick_compile;
pub mod runtime_rf_tick_source_compile;
pub mod runtime_rf_tick_source_select_compile;
pub mod runtime_tick_history_compile;
pub mod runtime_tick_shell_compile;
pub mod scenario;
pub mod scenario_candidate_from_runtime_compile;
pub mod scenario_candidate_save_reopen_compile;
pub mod scenario_canonical_io_compile;
pub mod scenario_ingestion_compile;
pub mod scenario_property_mutation_authority_boundary_compile;
pub mod scenario_stead_map_roundtrip_compile;
pub mod semantic_effect_execution_boundary_compile;
pub mod semantic_local_effects_compile;
pub mod semantic_local_effects_recursive_source_compile;
pub mod semantic_participant_delta_preview_compile;
pub mod session;
pub mod session_resource_flow_silos;
pub mod simulation_fabric;
pub mod spec_replay;
pub mod spec_session;
pub mod stress_compose_bridge;
pub mod structural_link_accumulator_compile;
pub mod structural_n4_atlas_partition;
pub mod structural_n4_theater_compile;
pub mod w_impedance_compose_bridge;

pub use action_band_consequence::{
    compile_crossing_consequence_session, submit_routed_overlay_product,
    CrossingConsequenceAdmissionError, CrossingConsequenceBinding, CrossingConsequenceDispatch,
    CrossingConsequenceDispatchError, CrossingConsequenceDispatchOutcome,
    CrossingConsequenceSession, ResidentNextWrite, RoutedOverlayDelivery, RoutedOverlayProduct,
    StructuralAuthorization,
};
pub use action_band_execution_compile::{
    compile_action_band_gpu_execution, compile_action_band_gpu_execution_with_native_lanes,
    frozen_admission_binding_id, ActionBandActiveInstance, ActionBandExecutionCompileError,
    ActionBandNativeLaneAdmission, ActionBandSessionOrigin, ActionBandStructuralApplyError,
    CompiledActionBandConservedProgressBinding, CompiledActionBandGpuExecution,
    FrozenActionBandStructuralRequests,
};
pub use action_band_semantic_shadow::{
    carry_bound_observables, designation_for_template, ActionBandBoundDispatch,
    ActionBandSemanticReadback, ActionBandSemanticSession, ActionBandTransitProjection,
    BoundObservableIdentity, FieldNeutralityGate, SealedActionBandAuthority, SemanticShadowError,
    SemanticallySealedProduction, FIELD_NEUTRALITY_OUTCOME,
};
pub use arena_allocation_oracle::{run_arena_allocation_oracle, ArenaAllocationOracleTrace};
pub use arena_allocation_plan::{
    max_disbursement_band, plan_arena_allocation, plan_arena_allocation_with_pressure,
    plan_resident_exact_apportionment, AllocationPlanError, ArenaAllocationPlan,
};
pub use arena_allocation_sync::{
    build_plan_for_tests, sync_resource_flow_accumulator,
    sync_resource_flow_accumulator_with_pressure, ResourceFlowSyncError, ResourceFlowSyncReport,
};
pub use arena_hierarchy::{
    build_custom_layout, build_execution_plan, build_execution_plan_from_authoring,
    build_flat_star_layout, build_nested_layout, nested_hierarchy_materialization_report,
    resolve_node_columns, resolve_node_columns_for_property, total_bands_for_depth,
    ArenaBandLayout, ArenaExecutionPlan, ArenaTreeLayout, HierarchyError, HierarchyNode,
    NestedHierarchyMaterializationReport, NodeColumnRefs,
};
pub use arena_pressure::{
    compile_arena_pressure_scatter, project_arena_pressure_seeds, ArenaPressureError,
};
pub use arena_registry::{
    ArenaCoupling, ArenaDiagnostic, ArenaExpansionReport, ArenaIdx, ArenaMember,
    ArenaRefreshReport, ArenaRegistry, ArenaRegistryBuilder, ArenaRegistryError, CouplingDelay,
    FissionPolicy, GpuArenaDescriptor, SlotId,
};
pub use atlas_0080_0::{
    replay_atlas_0080_0, run_atlas_0080_0, Atlas0080Cell, Atlas0080DescentAscentReport,
    Atlas0080ForbiddenRequests, Atlas0080Gate, Atlas0080Input, Atlas0080Report,
    Atlas0080ResidencyReport, Atlas0080ResidencyRequest, Atlas0080ResidencyState,
    Atlas0080Scenario, Atlas0080Surface, Atlas0080TheaterId, ATLAS_0080_0_DEFAULT_SEED,
    ATLAS_0080_0_ID, ATLAS_0080_0_LOGICAL_LOCATION_COUNT, ATLAS_0080_0_PLANET_SIDE,
    ATLAS_0080_0_SCENARIO, ATLAS_0080_0_STARMAP_SIDE, ATLAS_0080_0_STARSYSTEM_COUNT,
    ATLAS_0080_0_STARSYSTEM_SIDE, ATLAS_0080_0_STATUS_PASS,
};
pub use automaton_reception::{
    receive_command_deficits_from_disbursement, CommandDeficit, CommandDeficitReceptionError,
    CommandDeficitReceptionReport,
};
pub use bench_limits::{check as check_bench_ceiling, ms_per_sim_day, CEILINGS};
pub use child_share_eml::{child_share_cpu, register_child_share_formula};
pub use comparative_default_birth::{
    admit_comparative_from_emitters_and_topology, admit_comparative_from_field_plan,
    admit_field_plan_from_region_fields, FieldPlanAdmissionError, FieldPlanAdmissionReport,
    SealedFieldTopology,
};
pub use comparative_projection::{
    admit_comparative_projections, comparative_event_kind, comparative_projection_cpu_oracle,
    compile_comparative_bundle, neighbor_slots_from_grid, neighbor_slots_from_link_rows,
    ComparativeBandReadouts, ComparativeDerivedPropertyIds, ComparativeEmitterClass,
    ComparativeProjectionAdmission, ComparativeProjectionBands, ComparativeProjectionBundle,
    ComparativeProjectionDisposition, ComparativeProjectionError, ComparativeProjectionOutputs,
    ComparativeProjectionRequest, ComparativeThresholdPlan, GuYangStallOutputs,
    BAND_READOUT_COLUMN_COUNT, COMPARATIVE_DERIVED_COLUMN_COUNT, GUYANG_STALL_DERIVED_COLUMN_COUNT,
};
pub use field_scheduler::{
    count_cadence_due_ticks, execute_scheduled_regions_with, visit_scheduled_regions,
    DirtyRegionState, FieldCadence, FieldDispatchDecision, FieldDispatchReason,
    FieldDispatchSchedule, FieldGridDescriptor, FieldId, FieldRegionId, FieldRegionRegistration,
    FieldScheduleState, FieldScheduler, FieldSchedulerError, FieldSchedulerReport,
    ScheduledRegionsExecutionSummary, ScheduledSingleStencilExecution,
};
pub use field_sweep_compile::{
    compile_gu_yang_n4_field_sweeps, compile_gu_yang_overlay_parameterized_n4_field_sweeps,
    compile_palma_n4_field_sweep, compile_palma_overlay_parameterized_n4_field_sweep,
    compile_stead_overlay_parameterized_n4_field_sweep, GuYangN4FieldSweepSpec,
    GuYangOverlayParameterizedN4Spec, PalmaN4FieldSweepSpec, PalmaOverlayParameterizedN4Spec,
    SteadOverlayParameterizedN4Spec,
};
pub use gated_rates::{
    build_gated_rate_ops, resolve_gated_rates, seed_gated_rate_base_columns, ResolvedGatedRate,
    ResolvedMagnitude, RATE_BASE_SUB_FIELD,
};
pub use growth_entitlement::{GrowthEntitlementError, GrowthEntitlementMarketBinding};
pub use hosted_property_observation::{
    observe_hosted_property_cell, system_id_by_host_raw_from_structural_authority,
    AnchorTableSnapshot, GpuValuesSnapshot, HostedPropertyLocus, HostedPropertyObservationError,
    LiveDisruptionAuthorityReadback,
};
pub use install::{
    compile_and_install, install_atomic, preview_install, InstallError, InstallPreview,
};
pub use loaded_scenario_recursive_rf_runtime_compile::{
    compile_loaded_scenario_recursive_rf_runtime_plan_from_json_str,
    LoadedScenarioRecursiveRfRuntimePlan,
};
pub use loaded_scenario_runtime_report_chain_compile::{
    compile_loaded_scenario_runtime_report_chain_plan_from_json_str,
    LoadedScenarioRuntimeReportChainPlan,
};
pub use loaded_scenario_studio_session_envelope_compile::{
    compile_loaded_scenario_studio_session_envelope_plan_from_json_str,
    LoadedScenarioStudioSessionEnvelopePlan,
};
pub use local_allocation_recursive_source_compile::{
    compile_local_allocation_recursive_source_plan, LocalAllocationRecursiveSourcePlan,
};
pub use local_effect_application_compile::{
    compile_local_effect_application_plan, local_effect_application_aggregate_slot,
    local_effect_application_cpu_runtime_applied_total, local_effect_application_cpu_unmet_total,
    local_effect_application_runtime_applied_tick_inputs,
    local_effect_application_unmet_tick_inputs, LocalEffectApplicationAggregateProofPlan,
    LocalEffectApplicationPlan,
};
pub use local_effect_recursive_source_compile::{
    compile_local_effect_recursive_source_plan, LocalEffectRecursiveSourcePlan,
};
pub use local_participant_effects_compile::{
    compile_local_participant_effects_plan, local_participant_effects_aggregate_slot,
    local_participant_effects_allocated_tick_inputs, local_participant_effects_cpu_allocated_total,
    local_participant_effects_cpu_unmet_total, local_participant_effects_unmet_tick_inputs,
    LocalParticipantEffectAggregateProofPlan, LocalParticipantEffectsPlan,
};
pub use mapping_plan_compile::{
    compile_mapping_plan_from_admitted_theater, compile_structured_field_mapping_plan,
    MappingPlanCompileError, MappingPlanCompileSpec,
};
pub use mapping_runtime::{
    compiled_cadence_to_field_cadence, compiled_stencil_to_gpu_config, estimate_first_slice_budget,
    field_urgency_eml_nodes, field_urgency_plan_channels, FirstSliceCommitmentReport,
    FirstSliceMappingError, FirstSliceMappingReport, FirstSliceMappingSession,
    FirstSliceReadinessReport, FirstSliceResidencyReport, FirstSliceResidencyStatus,
    FirstSliceSeed, FirstSliceSummaryReport, FirstSliceSummaryStatus, FirstSliceTickOptions,
    EML_RESOURCE, EML_WEIGHT_PRESSURE, EML_WEIGHT_RESOURCE,
};
pub use min_plus_traversal_field::{
    TraversalFieldBandError, TraversalFieldBandSession, TraversalFieldDispatchReport,
    TraversalFieldExecutionMode, TraversalFieldExecutionOptions, TraversalFieldGpuInput,
    TraversalFieldGpuOutputHandle, TraversalFieldGridBinding,
    TraversalFieldShadowColumnCompatInput, TraversalFieldWInputKind, TRAVERSAL_FIELD_ID,
    TRAVERSAL_FIELD_REGION_ID, TRAVERSAL_FIELD_UTILITY_ID,
};
pub use order_directive::{
    build_order_directive_overlay, order_directive_injections_from_frame, AdmittedOrderWeightClass,
    OrderDirectiveError, OrderDirectiveInjection, OrderDirectiveRequest,
};
pub use owner_channel_rf_compile::{
    compile_owner_channel_rf_gpu_proof_plan, owner_channel_rf_bucket_aggregate_slot,
    owner_channel_rf_bucket_deficit_tick_inputs, owner_channel_rf_bucket_surplus_tick_inputs,
    prove_owner_channel_rf_cpu_gpu_parity, OwnerChannelRfBucketAccumulatorPlan,
    OwnerChannelRfGpuParityReport, OwnerChannelRfGpuProofError, OwnerChannelRfGpuProofPlan,
};
pub use owner_silo_accumulator_compile::{
    compile_owner_silo_gpu_tick_plan, owner_silo_aggregate_slot, owner_silo_deficit_tick_inputs,
    owner_silo_participant_deficit_total, owner_silo_participant_surplus_total,
    owner_silo_surplus_tick_inputs, OwnerSiloGpuTickPlan,
};
pub use owner_silo_disburse_down_compile::{
    compile_owner_silo_disburse_down_plan, owner_silo_disburse_down_cpu_demand_aggregate_total,
    owner_silo_disburse_down_demand_aggregate_slot,
    owner_silo_disburse_down_demand_aggregate_tick_inputs, OwnerSiloDemandAggregateProofPlan,
    OwnerSiloDisburseDownPlan,
};
pub use owner_silo_recursive_source_compile::{
    compile_owner_silo_recursive_source_plan, OwnerSiloRecursiveSourcePlan,
};
pub use owner_silo_runtime_writeback_compile::{
    compile_owner_silo_runtime_writeback_plan, owner_silo_writeback_aggregate_deficit_tick_inputs,
    owner_silo_writeback_aggregate_slot, owner_silo_writeback_aggregate_surplus_tick_inputs,
    OwnerSiloRuntimeWritebackPlan, OwnerSiloWritebackAggregateProofPlan,
};
pub use planet_child_rf_accumulator_compile::{
    compile_planet_child_rf_gpu_tick_plan, planet_child_rf_aggregate_slot,
    planet_child_rf_deficit_tick_inputs, planet_child_rf_participant_deficit_total,
    planet_child_rf_participant_surplus_total, planet_child_rf_surplus_tick_inputs,
    PlanetChildRfGpuTickPlan,
};
pub use planet_child_rf_reduce_up_compile::{
    compile_planet_child_rf_reduce_up_gpu_proof_plan,
    planet_child_rf_reduce_up_bucket_aggregate_slot,
    planet_child_rf_reduce_up_bucket_cpu_deficit_total,
    planet_child_rf_reduce_up_bucket_cpu_surplus_total,
    planet_child_rf_reduce_up_bucket_deficit_tick_inputs,
    planet_child_rf_reduce_up_bucket_surplus_tick_inputs, PlanetChildRfBucketAccumulatorPlan,
    PlanetChildRfReduceUpGpuProofPlan,
};
pub use production_path_0080_0::{
    replay_production_path_0080_0, run_production_path_0080_0, LocalPatrolEconomyCell,
    LocalPatrolEconomyScenario, ProductionPath0080ForbiddenRequests, ProductionPath0080Gate,
    ProductionPath0080Input, ProductionPath0080Report, ProductionPath0080Surface,
    PRODUCTION_PATH_0080_0_ALLOWED_ECONOMY_VALUES, PRODUCTION_PATH_0080_0_ID,
    PRODUCTION_PATH_0080_0_SCENARIO, PRODUCTION_PATH_0080_0_STATUS_PASS, SCENARIO_0080_0_GATE_ID,
};
pub use recursive_local_rf_compile::{
    compile_recursive_local_rf_plan, recursive_local_rf_cpu_demand_total,
    recursive_local_rf_cpu_surplus_total, recursive_local_rf_demand_aggregate_slot,
    recursive_local_rf_demand_tick_inputs, recursive_local_rf_surplus_aggregate_slot,
    recursive_local_rf_surplus_tick_inputs, RecursiveLocalRfAggregateProofPlan,
    RecursiveLocalRfPlan,
};
pub use recursive_rf_reconciliation_compile::{
    compile_recursive_rf_reconciliation_plan, RecursiveRfReconciliationPlan,
};
pub use resource_economy_boundary_schedule::{
    BoundaryScheduleEntry, BoundaryScheduleKey, ResourceEconomyBoundaryScheduleReport,
    KIND_RANK_RECIPE, KIND_RANK_TRANSFER,
};
pub use resource_economy_burn_in::{
    run_emission_burn_in, run_transfer_recipe_burn_in, ResourceEconomyBurnInReport,
};
pub use resource_economy_compile::{
    find_property_owner, materialize_resource_economy_registrations,
    materialize_resource_economy_registrations_with_slots, materialize_resource_economy_registry,
    materialize_resource_economy_registry_for_session, resolve_live_property_slot,
    ResourceEconomyCompileError, ResourceEconomyMaterializationReport,
    ResourceEconomyRegistrations, ResourceEconomyRegistry,
};
pub use resource_economy_oracle::{
    assert_discrete_transfer_conserved, run_emission_cpu_oracle, run_transfer_recipe_cpu_oracle,
    sum_cells, ResourceEconomyOracleError,
};
pub use resource_economy_sync::{
    sync_resource_economy_accumulator, sync_resource_economy_if_present, ResourceEconomySyncError,
    ResourceEconomySyncReport,
};
pub use resource_flow_burn_in::{
    run_flat_star_burn_in, ResourceFlowBurnInReport, ResourceFlowScenarioBurnInReport,
    ResourceFlowSoakSummaryReport,
};
pub use resource_flow_compile::{
    compile_and_materialize_resource_flow, materialize_arena_registry,
};
pub use resource_flow_convergence_burn_in::{
    assert_fixture_contract, clone_for_replay, fixture_convergence_static_512_participants,
    fixture_dynamic_multi_fission, fixture_dynamic_single_fission, fixture_repeated_resync,
    fixture_replay_static, fixture_static_flat_star_10_participants,
    fixture_static_flat_star_64_participants, fixture_static_flat_star_skewed_weights,
    fixture_two_arena_no_coupling, fixture_wildcard_rejected, open_fixture_session,
    run_resource_flow_burn_in, RfT2BurnInFixture, RfT2BurnInReport, RfT2EnrollmentKind,
    RfT2Session, RF_CONVERGENCE_STATIC_512, RF_T2_DYNAMIC_MULTI_FISSION,
    RF_T2_DYNAMIC_SINGLE_FISSION, RF_T2_STATIC_FLAT_STAR_10, RF_T2_STATIC_FLAT_STAR_64,
    RF_T2_STATIC_FLAT_STAR_SKEWED, RF_T2_TWO_ARENA_NO_COUPLING, RF_T2_WILDCARD_REJECTED,
};
pub use resource_flow_derivation::{
    derive_resource_flow_admission, ArenaAdmissionOrigin, DerivedArenaParticipation,
    DerivedParticipant, ResolvedResourceFlowAdmission, ResourceFlowDerivationError,
    ResourceFlowDerivationReport,
};
pub use resource_flow_dynamic_enrollment_soak::{
    initial_dynamic_enrollment_sync, run_dynamic_enrollment_gpu_burn_in,
    run_dynamic_enrollment_resync_cycles, DynamicEnrollmentBoundaryMetrics,
    DynamicEnrollmentSoakReport,
};
pub use resource_flow_enrollment::{resolve_resource_flow_enrollment, EnrollmentError};
pub use resource_flow_fission_enrollment::{
    react_to_fission_resource_flow_enrollment,
    react_to_fission_resource_flow_enrollment_on_authoring, DynamicFissionEnrollmentAdmission,
    DynamicFissionEnrollmentRejection, DynamicFissionEnrollmentReport,
};
pub use resource_flow_preflight::validate_resource_flow_preflight;
pub use rf_conservation_oracle::{
    allocator_eps_bound, allocator_from_disbursements, check_allocator_step,
    check_arena_structural, check_conservation, check_recipe_exact, flat_star_observations,
    leaf_allocated_from_cells, orphan_ids, AllocatorConservationViolation,
    AllocatorStepObservation, ArenaConservationSnapshot, ArenaMemberObservation,
    ArenaStructuralEvidence, ConservationReport, RecipeConservationViolation,
    RecipeInvocationObservation, StructuralConservationViolation,
};
pub use runtime_local_allocation_compile::{
    compile_runtime_local_allocation_application_plan, runtime_local_allocation_aggregate_slot,
    runtime_local_allocation_aggregate_tick_inputs, runtime_local_allocation_cpu_aggregate_total,
    RuntimeLocalAllocationAggregateProofPlan, RuntimeLocalAllocationApplicationPlan,
};
pub use runtime_participant_property_mutation_boundary_compile::{
    compile_runtime_participant_property_mutation_boundary_plan,
    RuntimeParticipantPropertyMutationBoundaryPlan,
};
pub use runtime_participant_state_mutation_compile::{
    compile_runtime_participant_state_mutation_plan, RuntimeParticipantStateMutationPlan,
};
pub use runtime_rf_tick_compile::{
    compile_runtime_rf_tick_plan, produce_runtime_rf_next_generation_demands_for_tick,
    RuntimeRfTickGpuProofSummary, RuntimeRfTickPlan,
};
pub use runtime_rf_tick_source_compile::{
    compile_runtime_rf_tick_source_comparison_plan,
    compile_runtime_tick_shell_with_rf_source_comparison_plan, RuntimeRfTickSourceComparisonPlan,
    RuntimeTickShellRfSourceComparisonPlan,
};
pub use runtime_rf_tick_source_select_compile::{
    compile_runtime_rf_tick_source_selection_plan,
    compile_runtime_tick_shell_with_selectable_rf_source_plan, RuntimeRfTickSourceSelectionPlan,
    RuntimeTickShellSelectableRfSourcePlan,
};
pub use runtime_tick_history_compile::{compile_runtime_tick_history_plan, RuntimeTickHistoryPlan};
pub use runtime_tick_shell_compile::{
    compile_runtime_tick_shell_plan, RuntimeTickShellGpuProofSummary, RuntimeTickShellPlan,
};
pub use scenario::{Scenario, ScenarioError, ShadowSeed};
pub use scenario_candidate_from_runtime_compile::{
    compile_scenario_candidate_from_runtime_plan_from_json_str, ScenarioCandidateFromRuntimePlan,
};
pub use scenario_candidate_save_reopen_compile::{
    compile_scenario_candidate_save_reopen_plan_from_json_str, ScenarioCandidateSaveReopenPlan,
};
pub use scenario_canonical_io_compile::{
    compile_scenario_canonical_io_plan_from_json_str, ScenarioCanonicalIoPlan,
};
pub use scenario_ingestion_compile::evaluate_scenario_compile_readiness;
pub use scenario_property_mutation_authority_boundary_compile::{
    compile_scenario_property_mutation_authority_boundary_plan,
    ScenarioPropertyMutationAuthorityBoundaryPlan,
};
pub use scenario_stead_map_roundtrip_compile::{
    compile_scenario_stead_map_roundtrip_plan_from_json_str, ScenarioSteadMapRoundtripPlan,
};
pub use semantic_effect_execution_boundary_compile::{
    compile_semantic_effect_execution_boundary_plan, SemanticEffectExecutionBoundaryPlan,
};
pub use semantic_local_effects_compile::{
    compile_semantic_local_effects_plan, semantic_local_effects_applied_output_indices,
    semantic_local_effects_cpu_runtime_applied_total, semantic_local_effects_cpu_shortfall_total,
    semantic_local_effects_runtime_applied_aggregate_slot,
    semantic_local_effects_runtime_applied_tick_inputs,
    semantic_local_effects_shortfall_aggregate_slot,
    semantic_local_effects_shortfall_output_indices, semantic_local_effects_shortfall_tick_inputs,
    SemanticLocalEffectAggregateProofPlan, SemanticLocalEffectsPlan,
};
pub use semantic_local_effects_recursive_source_compile::{
    compile_semantic_local_effects_recursive_source_plan, SemanticLocalEffectsRecursiveSourcePlan,
};
pub use semantic_participant_delta_preview_compile::{
    compile_semantic_participant_delta_preview_plan, SemanticParticipantDeltaPreviewPlan,
};
pub use session::{
    ActionBandExecutionIngressError, RunSummary, SessionError, SessionShadowView, SimSession,
    StepOnceOutcome,
};
pub use session_resource_flow_silos::{
    build_owner_silo_resource_flow_spec, compile_and_materialize_owner_silo_flow,
    compile_and_materialize_owner_silo_flow_via_resource_flow, compile_owner_silo_flow_admission,
    OwnerSiloFlowMaterializationReport,
};
pub use simthing_core::StructuralCoord;
pub use simthing_gpu::SlotAllocError;
pub use simulation_fabric::{
    run_mapping_hot_dispatch, run_simulation_fabric_hot_cycle, run_simulation_fabric_hot_step,
    run_simulation_fabric_pre_tick_enqueue, run_simulation_fabric_tick, FabricHotCycleOutcome,
    FabricHotCycleParams, FabricHotStepOutcome, FabricHotStepParams, FabricMappingHotReport,
    FabricTickOutcome, HotFabricParts, MappingHotPathState, SimulationFabric,
};
pub use spec_replay::{
    apply_spec_delta, apply_spec_snapshot, collect_spec_snapshot, diff_and_emit,
    json_to_spec_deltas, open_replay_with_spec, read_spec_replay_file, spec_deltas_to_json,
    CapabilityStateSnapshot, LoadedReplay, QueuedSelectionSnapshot, ReplayOpenError,
    ScriptedCooldownSnapshot, SpecDelta, SpecSnapshot,
};
pub use spec_session::{
    CapabilityInstanceKey, PreBoundarySnapshot, SpecSessionError, SpecSessionState,
};
pub use stress_compose_bridge::compiled_stress_compose_to_gpu_config;
pub use structural_link_accumulator_compile::{
    compile_structural_link_field_adjacency, compile_structural_link_neighbor_sum_plan,
    DriverCompileError,
};
pub use structural_n4_atlas_partition::{
    compile_structural_n4_atlas, CompiledStructuralN4Atlas, CrossPartitionHaloCoverage,
    DeferredCrossPartitionN4Edge, PartitionedStructuralN4Theater, StructuralAtlasAdmission,
    StructuralAtlasPartitionProfile, StructuralTheaterCellRole, StructuralTheaterCoordPadding,
    StructuralTheaterHaloCell, StructuralTheaterOrigin,
};
pub use structural_n4_theater_compile::{
    compile_structural_n4_theater, AtlasDeferralReason, CompiledStructuralN4Theater,
    CompiledStructuralPlacement, StructuralTheaterAdmission, StructuralTheaterCompileError,
};
pub use w_impedance_compose_bridge::{
    compiled_w_impedance_compose_to_gpu_config, composed_w_min_plus_stencil_config,
};
