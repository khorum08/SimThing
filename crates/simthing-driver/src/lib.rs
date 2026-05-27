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
    build_custom_layout, build_execution_plan, build_flat_star_layout, resolve_node_columns,
    total_bands_for_depth, ArenaBandLayout, ArenaExecutionPlan, ArenaTreeLayout, HierarchyError,
    HierarchyNode, NodeColumnRefs,
};
pub use child_share_eml::{child_share_cpu, register_child_share_formula};
pub use arena_participant::{
    all_reserved_gap_slots, arena_participant_sibling_slots, commit_dynamic_arena_root_append,
    materialize_arena_participants, prepare_dynamic_arena_root_append,
    slot_in_participant_sibling_range, slots_are_contiguous, try_alloc_participant_child_in_gap,
    try_append_arena_root_sibling_participant, ArenaParticipantAllocationReport,
    ArenaParticipantIndex, ArenaParticipantScaffold, DynamicEnrollmentError, GapAllocError,
    PendingDynamicArenaRootParticipant, ReservedGapPool,
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
