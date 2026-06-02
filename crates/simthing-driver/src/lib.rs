pub mod arena_allocation_oracle;
pub mod arena_allocation_plan;
pub mod arena_allocation_sync;
pub mod arena_hierarchy;
pub mod arena_participant;
pub mod arena_registry;
pub mod bench_limits;
pub mod child_share_eml;
pub mod control_0080_0;
pub mod default_schedule_0080_0;
pub mod demo_0080_0;
pub mod field_scheduler;
pub mod first_slice_mapping_runtime;
pub mod gameplay_0080_0;
pub mod install;
pub mod production_path_0080_0;
pub mod resource_economy_boundary_schedule;
pub mod resource_economy_burn_in;
pub mod resource_economy_compile;
pub mod resource_economy_oracle;
pub mod resource_economy_sync;
pub mod resource_flow_burn_in;
pub mod resource_flow_compile;
pub mod resource_flow_dynamic_enrollment_soak;
pub mod resource_flow_enrollment;
pub mod resource_flow_fission_enrollment;
pub mod resource_flow_flat_star_continued_soak;
pub mod resource_flow_opt_in_burn_in;
pub mod resource_flow_opt_in_product_soak;
pub mod resource_flow_opt_in_telemetry;
pub mod resource_flow_preflight;
pub mod resource_flow_scenario_class_burn_in;
pub mod scenario;
pub mod session;
pub mod spec_replay;
pub mod spec_session;

pub use arena_allocation_oracle::{run_arena_allocation_oracle, ArenaAllocationOracleTrace};
pub use arena_allocation_plan::{
    max_disbursement_band, plan_arena_allocation, AllocationPlanError, ArenaAllocationPlan,
};
pub use arena_allocation_sync::{
    build_plan_for_tests, sync_resource_flow_accumulator, ResourceFlowSyncError,
    ResourceFlowSyncReport,
};
pub use arena_hierarchy::{
    build_custom_layout, build_execution_plan, build_flat_star_layout, build_nested_layout,
    nested_hierarchy_materialization_report, resolve_node_columns, total_bands_for_depth,
    ArenaBandLayout, ArenaExecutionPlan, ArenaTreeLayout, HierarchyError, HierarchyNode,
    NestedHierarchyMaterializationReport, NodeColumnRefs,
};
pub use arena_participant::{
    all_reserved_gap_slots, arena_participant_sibling_slots, commit_dynamic_arena_root_append,
    gap_pool_snapshot, materialize_arena_participants, nested_fission_gap_report,
    prepare_dynamic_arena_root_append, refresh_fission_participant_child,
    reserve_gap_pools_for_parent_slots, slot_in_participant_sibling_range, slots_are_contiguous,
    try_alloc_participant_child_in_gap, try_append_arena_root_sibling_participant,
    ArenaParticipantAllocationReport, ArenaParticipantIndex, ArenaParticipantScaffold,
    DynamicEnrollmentError, GapAllocError, NestedFissionGapReport,
    PendingDynamicArenaRootParticipant, ReservedGapPool,
};
pub use arena_registry::{
    ArenaCoupling, ArenaDiagnostic, ArenaExpansionReport, ArenaIdx, ArenaParticipant,
    ArenaRefreshReport, ArenaRegistry, ArenaRegistryBuilder, ArenaRegistryError, CouplingDelay,
    FissionPolicy, GpuArenaDescriptor, SlotId,
};
pub use bench_limits::{check as check_bench_ceiling, ms_per_sim_day, CEILINGS};
pub use child_share_eml::{child_share_cpu, register_child_share_formula};
pub use default_schedule_0080_0::{
    replay_default_schedule_0080_0, run_default_schedule_0080_0, DefaultSchedule0080ForbiddenRequests,
    DefaultSchedule0080Gate, DefaultSchedule0080Input, DefaultSchedule0080Location,
    DefaultSchedule0080PirateState, DefaultSchedule0080PirateStepReport,
    DefaultSchedule0080RunReport, DefaultSchedule0080Step, DefaultSchedule0080StepReport,
    DefaultSchedule0080Surface,
    DEFAULT_SCHEDULE_0080_0_ID, DEFAULT_SCHEDULE_0080_0_SCENARIO,
    DEFAULT_SCHEDULE_0080_0_STATUS_1A_PASS, DEFAULT_SCHEDULE_0080_0_STATUS_1B_PASS,
};
pub use gameplay_0080_0::{
    export_gameplay_0080_text, observe_gameplay_0080_0, replay_observe_gameplay_0080_0,
    Gameplay0080ForbiddenRequests, Gameplay0080LocationSummary, Gameplay0080ObservationGate,
    Gameplay0080ObservationInput, Gameplay0080ObservationReport, Gameplay0080ObservationSurface,
    Gameplay0080StepTranscript, Gameplay0080Transcript, GAMEPLAY_0080_0_ID,
    GAMEPLAY_0080_0_SCENARIO, GAMEPLAY_0080_0_STATUS_PASS,
};
pub use control_0080_0::{
    admit_control_0080_0, replay_admit_control_0080_0, Control0080AdmissionInput,
    Control0080AdmissionReport, Control0080Command, Control0080CommandBatch, Control0080ForbiddenRequests,
    Control0080Gate, Control0080RejectedCommand, Control0080Surface, CONTROL_0080_0_ID,
    CONTROL_0080_0_SCENARIO, CONTROL_0080_0_STATUS_PASS,
};
pub use demo_0080_0::{
    canonical_control_input, replay_demo_0080_0, run_demo_0080_0, Demo0080ForbiddenRequests,
    Demo0080Gate, Demo0080Input, Demo0080MovementDay, Demo0080MovementRecord, Demo0080Report,
    Demo0080Surface, DEMO_0080_0_ID, DEMO_0080_0_SCENARIO, DEMO_0080_0_STATUS_PASS,
};
pub use field_scheduler::{
    count_cadence_due_ticks, execute_scheduled_regions_with, execute_single_scheduled_stencil_region,
    visit_scheduled_regions, DirtyRegionState, FieldCadence, FieldDispatchDecision,
    FieldDispatchReason, FieldDispatchSchedule, FieldGridDescriptor, FieldId, FieldRegionId,
    FieldRegionRegistration, FieldScheduleState, FieldScheduler, FieldSchedulerError,
    FieldSchedulerReport, ScheduledRegionsExecutionSummary, ScheduledSingleStencilExecution,
    ScheduledStencilExecutionError,
};
pub use first_slice_mapping_runtime::{
    compiled_cadence_to_field_cadence, compiled_stencil_to_gpu_config, estimate_first_slice_budget,
    FirstSliceCommitmentReport, FirstSliceMappingError, FirstSliceMappingReport,
    FirstSliceMappingSession, FirstSliceReadinessReport, FirstSliceResidencyReport,
    FirstSliceResidencyStatus, FirstSliceSeed, FirstSliceSummaryReport, FirstSliceSummaryStatus,
    FirstSliceTickOptions,
};
pub use install::{
    compile_and_install, install_atomic, preview_install, InstallError, InstallPreview,
};
pub use production_path_0080_0::{
    replay_production_path_0080_0, run_production_path_0080_0, LocalPatrolEconomyCell,
    LocalPatrolEconomyScenario, ProductionPath0080ForbiddenRequests, ProductionPath0080Gate,
    ProductionPath0080Input, ProductionPath0080Report, ProductionPath0080Surface,
    PRODUCTION_PATH_0080_0_ALLOWED_ECONOMY_VALUES, PRODUCTION_PATH_0080_0_ID,
    PRODUCTION_PATH_0080_0_SCENARIO, PRODUCTION_PATH_0080_0_STATUS_PASS,
    SCENARIO_0080_0_GATE_ID,
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
pub use resource_flow_dynamic_enrollment_soak::{
    initial_dynamic_enrollment_sync, run_dynamic_enrollment_gpu_burn_in,
    run_dynamic_enrollment_resync_cycles, DynamicEnrollmentBoundaryMetrics,
    DynamicEnrollmentSoakReport,
};
pub use resource_flow_enrollment::{resolve_resource_flow_enrollment, EnrollmentError};
pub use resource_flow_fission_enrollment::{
    react_to_fission_resource_flow_enrollment, DynamicFissionEnrollmentAdmission,
    DynamicFissionEnrollmentRejection, DynamicFissionEnrollmentReport,
};
pub use resource_flow_flat_star_continued_soak::{
    continued_static_512_participant_count, fixture_continued_dynamic_policy_a,
    fixture_continued_multi_arena_no_coupling, fixture_continued_replay,
    fixture_continued_static_512_participants, fixture_continued_static_skewed_weights,
    open_continued_profile_session, run_continued_replay_pair, run_continued_soak_with_summary,
    FlatStarContinuedSoakSummary,
};
pub use resource_flow_opt_in_burn_in::{
    assert_fixture_contract, clone_for_replay, fixture_disabled_populated_spec,
    fixture_dynamic_multi_fission, fixture_dynamic_single_fission,
    fixture_product_static_512_participants, fixture_profile_static_512_participants,
    fixture_repeated_resync, fixture_replay_static, fixture_static_flat_star_10_participants,
    fixture_static_flat_star_64_participants, fixture_static_flat_star_skewed_weights,
    fixture_two_arena_no_coupling, fixture_wildcard_rejected, open_fixture_session,
    open_fixture_session_with_execution_profile, run_opt_in_burn_in, RfT2BurnInFixture,
    RfT2BurnInReport, RfT2EnrollmentKind, RfT2OptInSession, RF_CONTINUED_DYNAMIC_POLICY_A,
    RF_CONTINUED_MULTI_ARENA, RF_CONTINUED_REPLAY, RF_CONTINUED_STATIC_512,
    RF_CONTINUED_STATIC_SKEWED, RF_T2_DISABLED_POPULATED, RF_T2_DYNAMIC_MULTI_FISSION,
    RF_T2_DYNAMIC_SINGLE_FISSION, RF_T2_STATIC_FLAT_STAR_10, RF_T2_STATIC_FLAT_STAR_64,
    RF_T2_STATIC_FLAT_STAR_SKEWED, RF_T2_TWO_ARENA_NO_COUPLING, RF_T2_WILDCARD_REJECTED,
};
pub use resource_flow_opt_in_product_soak::{
    assert_telemetry_contract, fixture_product_disabled_spec_diagnostics,
    fixture_product_dynamic_fission_cadence, fixture_product_multi_arena_no_coupling,
    fixture_product_multi_session_replay, fixture_product_rejection_telemetry,
    fixture_product_repeated_resync, fixture_product_static_128_participants,
    fixture_product_static_256_participants, open_product_session, run_multi_session_replay,
    run_product_soak_with_telemetry, telemetry_for_open_session, RF_T3_PRODUCT_DISABLED,
    RF_T3_PRODUCT_DYNAMIC_FISSION, RF_T3_PRODUCT_MULTI_ARENA, RF_T3_PRODUCT_MULTI_SESSION,
    RF_T3_PRODUCT_REJECTION, RF_T3_PRODUCT_RESYNC, RF_T3_PRODUCT_STATIC_128,
    RF_T3_PRODUCT_STATIC_256,
};
pub use resource_flow_opt_in_telemetry::{
    collect_resource_flow_opt_in_telemetry, flag_source_from_opt_in_mode, ResourceFlowFlagSource,
    ResourceFlowOptInTelemetryReport,
};
pub use resource_flow_preflight::validate_resource_flow_preflight;
pub use resource_flow_scenario_class_burn_in::{
    assert_profile_telemetry_contract, fixture_profile_disabled_or_default,
    fixture_profile_dynamic_fission_cadence, fixture_profile_multi_arena_no_coupling,
    fixture_profile_multi_session_replay, fixture_profile_rejection_telemetry,
    fixture_profile_repeated_resync, fixture_profile_static_128_participants,
    fixture_profile_static_256_participants, open_default_profile_session, open_profile_session,
    profile_telemetry_for_open_session, run_profile_multi_session_replay,
    run_profile_soak_with_telemetry, RF_T5_PROFILE_DISABLED, RF_T5_PROFILE_DYNAMIC_FISSION,
    RF_T5_PROFILE_MULTI_ARENA, RF_T5_PROFILE_MULTI_SESSION, RF_T5_PROFILE_REJECTION,
    RF_T5_PROFILE_RESYNC, RF_T5_PROFILE_STATIC_128, RF_T5_PROFILE_STATIC_256,
};
pub use scenario::{Scenario, ScenarioError, ShadowSeed};
pub use session::{RunSummary, SessionError, SimSession};
pub use simthing_gpu::SlotAllocError;
pub use spec_replay::{
    apply_spec_delta, apply_spec_snapshot, collect_spec_snapshot, diff_and_emit,
    json_to_spec_deltas, open_replay_with_spec, read_spec_replay_file, spec_deltas_to_json,
    CapabilityStateSnapshot, LoadedReplay, QueuedSelectionSnapshot, ReplayOpenError,
    ScriptedCooldownSnapshot, SpecDelta, SpecSnapshot,
};
pub use spec_session::{
    CapabilityInstanceKey, PreBoundarySnapshot, SpecSessionError, SpecSessionState,
};
