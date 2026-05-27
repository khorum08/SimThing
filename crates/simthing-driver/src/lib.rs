pub mod arena_allocation_oracle;
pub mod arena_allocation_plan;
pub mod arena_allocation_sync;
pub mod arena_hierarchy;
pub mod arena_participant;
pub mod arena_registry;
pub mod bench_limits;
pub mod child_share_eml;
pub mod install;
pub mod resource_flow_compile;
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
pub use arena_allocation_sync::{build_plan_for_tests, sync_resource_flow_accumulator, ResourceFlowSyncReport};
pub use arena_hierarchy::{
    build_custom_layout, build_execution_plan, build_flat_star_layout, resolve_node_columns,
    total_bands_for_depth, ArenaBandLayout, ArenaExecutionPlan, ArenaTreeLayout, HierarchyError,
    HierarchyNode, NodeColumnRefs,
};
pub use child_share_eml::{child_share_cpu, register_child_share_formula};
pub use arena_participant::{
    all_reserved_gap_slots, arena_participant_sibling_slots, materialize_arena_participants,
    slot_in_participant_sibling_range, slots_are_contiguous, try_alloc_participant_child_in_gap,
    ArenaParticipantAllocationReport, ArenaParticipantIndex, ArenaParticipantScaffold,
    GapAllocError, ReservedGapPool,
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
pub use resource_flow_compile::{
    compile_and_materialize_resource_flow, materialize_arena_registry,
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
