pub mod arena_allocation_oracle;
pub mod arena_allocation_plan;
pub mod arena_allocation_sync;
pub mod arena_hierarchy;
pub mod arena_participant;
pub mod arena_registry;
pub mod bench_limits;
pub mod child_share_eml;
pub mod install;
pub mod resource_economy_burn_in;
pub mod resource_economy_compile;
pub mod resource_economy_oracle;
pub mod resource_economy_sync;
pub mod resource_flow_burn_in;
pub mod resource_flow_compile;
pub mod resource_flow_enrollment;
pub mod resource_flow_opt_in_burn_in;
pub mod resource_flow_opt_in_product_soak;
pub mod resource_flow_opt_in_telemetry;
pub mod resource_flow_scenario_class_burn_in;
pub mod resource_flow_dynamic_enrollment_soak;
pub mod resource_flow_fission_enrollment;
pub mod resource_flow_preflight;
pub mod scenario;
pub mod session;
pub mod spec_replay;
pub mod spec_session;

pub use arena_allocation_oracle::{
    run_arena_allocation_oracle, ArenaAllocationOracleTrace,
};
pub use arena_allocation_plan::{
    max_disbursement_band, plan_arena_allocation, AllocationPlanError, ArenaAllocationPlan,
};
pub use arena_allocation_sync::{
    build_plan_for_tests, sync_resource_flow_accumulator, ResourceFlowSyncError,
    ResourceFlowSyncReport,
};
pub use arena_hierarchy::{
    build_custom_layout, build_execution_plan, build_flat_star_layout, build_nested_layout,
    resolve_node_columns, total_bands_for_depth, ArenaBandLayout, ArenaExecutionPlan,
    ArenaTreeLayout, HierarchyError, HierarchyNode, NodeColumnRefs,
};
pub use child_share_eml::{child_share_cpu, register_child_share_formula};
pub use arena_participant::{
    all_reserved_gap_slots, arena_participant_sibling_slots, commit_dynamic_arena_root_append,
    gap_pool_snapshot, materialize_arena_participants, nested_fission_gap_report,
    prepare_dynamic_arena_root_append, refresh_fission_participant_child,
    reserve_gap_pools_for_parent_slots,
    slot_in_participant_sibling_range, slots_are_contiguous, try_alloc_participant_child_in_gap,
    try_append_arena_root_sibling_participant, ArenaParticipantAllocationReport,
    ArenaParticipantIndex, ArenaParticipantScaffold, DynamicEnrollmentError, GapAllocError,
    NestedFissionGapReport, PendingDynamicArenaRootParticipant, ReservedGapPool,
};
pub use arena_registry::{
    ArenaCoupling, ArenaDiagnostic, ArenaExpansionReport, ArenaIdx, ArenaParticipant,
    ArenaRefreshReport, ArenaRegistry, ArenaRegistryBuilder, ArenaRegistryError, CouplingDelay,
    FissionPolicy, GpuArenaDescriptor, SlotId,
};
pub use bench_limits::{check as check_bench_ceiling, ms_per_sim_day, CEILINGS};
pub use install::{
    compile_and_install, install_atomic, preview_install, InstallError, InstallPreview,
};
pub use resource_economy_compile::{
    find_property_owner, materialize_resource_economy_registrations,
    materialize_resource_economy_registrations_with_slots,
    materialize_resource_economy_registry, materialize_resource_economy_registry_for_session,
    resolve_live_property_slot, ResourceEconomyCompileError, ResourceEconomyMaterializationReport,
    ResourceEconomyRegistry, ResourceEconomyRegistrations,
};
pub use resource_economy_burn_in::{
    run_emission_burn_in, run_transfer_recipe_burn_in, ResourceEconomyBurnInReport,
};
pub use resource_economy_oracle::{
    assert_discrete_transfer_conserved, run_emission_cpu_oracle, run_transfer_recipe_cpu_oracle,
    sum_cells, ResourceEconomyOracleError,
};
pub use resource_economy_sync::{
    sync_resource_economy_accumulator, sync_resource_economy_if_present,
    ResourceEconomySyncError, ResourceEconomySyncReport,
};
pub use resource_flow_burn_in::{
    run_flat_star_burn_in, ResourceFlowBurnInReport, ResourceFlowScenarioBurnInReport,
    ResourceFlowSoakSummaryReport,
};
pub use resource_flow_compile::{
    compile_and_materialize_resource_flow, materialize_arena_registry,
};
pub use resource_flow_enrollment::{resolve_resource_flow_enrollment, EnrollmentError};
pub use resource_flow_opt_in_burn_in::{
    clone_for_replay, fixture_disabled_populated_spec, fixture_dynamic_multi_fission,
    fixture_dynamic_single_fission, fixture_repeated_resync, fixture_replay_static,
    fixture_static_flat_star_10_participants, fixture_static_flat_star_64_participants,
    fixture_static_flat_star_skewed_weights, fixture_two_arena_no_coupling,
    fixture_wildcard_rejected, open_fixture_session,
    open_fixture_session_with_execution_profile, run_opt_in_burn_in, assert_fixture_contract,
    RfT2BurnInFixture, RfT2BurnInReport, RfT2EnrollmentKind, RfT2OptInSession,
    RF_T2_DISABLED_POPULATED, RF_T2_DYNAMIC_MULTI_FISSION, RF_T2_DYNAMIC_SINGLE_FISSION,
    RF_T2_STATIC_FLAT_STAR_10, RF_T2_STATIC_FLAT_STAR_64, RF_T2_STATIC_FLAT_STAR_SKEWED,
    RF_T2_TWO_ARENA_NO_COUPLING, RF_T2_WILDCARD_REJECTED,
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
pub use resource_flow_opt_in_telemetry::{
    collect_resource_flow_opt_in_telemetry, flag_source_from_opt_in_mode,
    ResourceFlowFlagSource, ResourceFlowOptInTelemetryReport,
};
pub use resource_flow_dynamic_enrollment_soak::{
    initial_dynamic_enrollment_sync, run_dynamic_enrollment_gpu_burn_in,
    run_dynamic_enrollment_resync_cycles, DynamicEnrollmentBoundaryMetrics,
    DynamicEnrollmentSoakReport,
};
pub use resource_flow_fission_enrollment::{
    react_to_fission_resource_flow_enrollment, DynamicFissionEnrollmentAdmission,
    DynamicFissionEnrollmentRejection, DynamicFissionEnrollmentReport,
};
pub use resource_flow_preflight::validate_resource_flow_preflight;
pub use simthing_gpu::SlotAllocError;
pub use scenario::{Scenario, ScenarioError, ShadowSeed};
pub use session::{RunSummary, SessionError, SimSession};
pub use spec_replay::{
    apply_spec_delta, apply_spec_snapshot, collect_spec_snapshot, diff_and_emit,
    json_to_spec_deltas, open_replay_with_spec, read_spec_replay_file, spec_deltas_to_json,
    CapabilityStateSnapshot, LoadedReplay, QueuedSelectionSnapshot, ReplayOpenError,
    ScriptedCooldownSnapshot, SpecDelta, SpecSnapshot,
};
pub use spec_session::{
    CapabilityInstanceKey, PreBoundarySnapshot, SpecSessionError, SpecSessionState,
};
