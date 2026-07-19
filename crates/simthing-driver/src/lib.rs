pub mod arena_allocation_oracle;
pub mod arena_allocation_plan;
pub mod arena_allocation_sync;
pub mod arena_hierarchy;
pub mod arena_participant;
pub mod arena_pressure;
pub mod arena_registry;
pub mod atlas_0080_0;
pub mod bench_limits;
pub mod child_share_eml;
pub mod compound_field_0080_2;
pub mod control_0080_0;
pub mod control_0080_1;
pub mod default_schedule_0080_0;
pub mod default_schedule_0080_1;
pub mod demo_0080_0;
pub mod demo_0080_1;
pub mod disruption_decay_0080_2;
pub mod dress_rehearsal_r1_disruption_heatmap;
pub mod dress_rehearsal_r2_recursive_allocation;
pub mod dress_rehearsal_r3_capability_mask_down;
pub mod dress_rehearsal_r4_field_policy_consumption;
pub mod dress_rehearsal_r5_movement_reenroll;
pub mod dress_rehearsal_r6_combat_hp_damage;
pub mod dress_rehearsal_r6b_ship_cohort_reinforcement;
pub mod dress_rehearsal_r6c_integrated_run;
pub mod econ_scale_0080_0;
pub mod field_scheduler;
pub mod first_slice_mapping_runtime;
pub mod gameplay_0080_0;
pub mod gameplay_0080_1;
pub mod gated_rates;
pub mod need_weight_profile;
pub mod gpu_measure_0080_0;
pub mod gradient_follow_0080_2;
pub mod install;
pub mod loaded_scenario_recursive_rf_runtime_compile;
pub mod loaded_scenario_runtime_report_chain_compile;
pub mod loaded_scenario_studio_session_envelope_compile;
pub mod local_allocation_recursive_source_compile;
pub mod local_effect_application_compile;
pub mod local_effect_recursive_source_compile;
pub mod local_participant_effects_compile;
pub mod mapping_plan_compile;
pub mod min_plus_traversal_field;
pub mod owner_silo_accumulator_compile;
pub mod owner_silo_disburse_down_compile;
pub mod owner_silo_recursive_source_compile;
pub mod owner_silo_runtime_writeback_compile;
pub mod planet_child_rf_accumulator_compile;
pub mod planet_child_rf_reduce_up_compile;
pub mod production_path_0080_0;
pub mod production_path_0080_1;
pub mod recursive_local_rf_compile;
pub mod recursive_rf_reconciliation_compile;
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
pub mod rf_conservation_oracle;
pub mod runtime_0080_0_r0;
pub mod runtime_0080_0_r1a;
pub mod runtime_0080_0_r1b;
pub mod runtime_0080_0_r1c;
pub mod runtime_0080_0_r1c_a;
pub mod runtime_0080_0_r1c_b;
pub mod runtime_0080_0_r1c_c;
pub mod runtime_0080_0_r1c_d;
pub mod runtime_0080_0_r1c_e;
pub mod runtime_0080_0_r1c_f;
pub mod runtime_0080_0_r2;
pub(crate) mod runtime_0080_0_r2_substrate;
pub mod runtime_0080_rr_0;
pub mod runtime_0080_rr_1;
pub mod runtime_0080_rr_2;
pub mod runtime_0080_rr_3;
pub mod runtime_0080_rr_4;
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

pub use arena_allocation_oracle::{run_arena_allocation_oracle, ArenaAllocationOracleTrace};
pub use rf_conservation_oracle::{
    allocator_eps_bound, allocator_from_disbursements, check_allocator_step, check_arena_structural,
    check_conservation, check_recipe_exact, flat_star_observations,
    leaf_allocated_from_cells, orphan_ids, AllocatorConservationViolation,
    AllocatorStepObservation, ArenaConservationSnapshot, ArenaParticipantObservation,
    ArenaStructuralEvidence, ConservationReport, RecipeConservationViolation,
    RecipeInvocationObservation, StructuralConservationViolation,
};
pub use arena_allocation_plan::{
    max_disbursement_band, plan_arena_allocation, AllocationPlanError, ArenaAllocationPlan,
};
pub use arena_allocation_sync::{
    build_plan_for_tests, sync_resource_flow_accumulator, ResourceFlowSyncError,
    ResourceFlowSyncReport,
};
pub use arena_hierarchy::{
    build_custom_layout, build_execution_plan, build_execution_plan_from_authoring,
    build_flat_star_layout, build_nested_layout, nested_hierarchy_materialization_report,
    resolve_node_columns, total_bands_for_depth, ArenaBandLayout, ArenaExecutionPlan,
    ArenaTreeLayout, HierarchyError, HierarchyNode, NestedHierarchyMaterializationReport,
    NodeColumnRefs,
};
pub use arena_participant::{
    all_reserved_gap_slots, arena_participant_sibling_slots, commit_dynamic_arena_root_append,
    commit_dynamic_arena_root_append_to_authoring, gap_pool_snapshot,
    materialize_arena_participants, nested_fission_gap_report, prepare_dynamic_arena_root_append,
    prepare_dynamic_arena_root_append_from_authoring, refresh_fission_participant_child,
    refresh_fission_participant_child_on_authoring, reserve_gap_pools_for_parent_slots,
    slot_in_participant_sibling_range, slots_are_contiguous, try_alloc_participant_child_in_gap,
    try_append_arena_root_sibling_participant,
    try_append_arena_root_sibling_participant_on_authoring, ArenaParticipantAllocationReport,
    ArenaParticipantIndex, ArenaParticipantScaffold, DynamicEnrollmentError, GapAllocError,
    NestedFissionGapReport, PendingDynamicArenaRootParticipant, ReservedGapPool,
};
pub use arena_pressure::{
    compile_arena_pressure_scatter, project_arena_pressure_seeds, ArenaPressureError,
};
pub use arena_registry::{
    ArenaCoupling, ArenaDiagnostic, ArenaExpansionReport, ArenaIdx, ArenaParticipant,
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
pub use bench_limits::{check as check_bench_ceiling, ms_per_sim_day, CEILINGS};
pub use child_share_eml::{child_share_cpu, register_child_share_formula};
pub use compound_field_0080_2::{
    replay_compound_field_0080_2, run_compound_field_0080_2, CompoundField0082ForbiddenRequests,
    CompoundField0082Gate, CompoundField0082Input, CompoundField0082NodePos,
    CompoundField0082Report, CompoundField0082Surface, CompoundField0082TickSnapshot,
    CompoundField0082Weights, BASE_DESIRABILITY, COMPOUND_FIELD_0080_2_ID,
    COMPOUND_FIELD_0080_2_SCENARIO, COMPOUND_FIELD_0080_2_STATUS_PASS, DESIRABILITY_MAX,
};
pub use control_0080_0::{
    admit_control_0080_0, replay_admit_control_0080_0, Control0080AdmissionInput,
    Control0080AdmissionReport, Control0080Command, Control0080CommandBatch,
    Control0080ForbiddenRequests, Control0080Gate, Control0080RejectedCommand, Control0080Surface,
    CONTROL_0080_0_ID, CONTROL_0080_0_SCENARIO, CONTROL_0080_0_STATUS_PASS,
};
pub use control_0080_1::{
    admit_control_0080_1, replay_admit_control_0080_1, Control0081AdmissionInput,
    Control0081AdmissionReport, Control0081BoundedConfig, Control0081Command,
    Control0081CommandBatch, Control0081CommandTranscriptRow, Control0081ForbiddenRequests,
    Control0081Gate, Control0081RejectedCommand, Control0081Surface, CONTROL_0080_1_ID,
    CONTROL_0080_1_SCENARIO, CONTROL_0080_1_STATUS_PASS,
};
pub use default_schedule_0080_0::{
    replay_default_schedule_0080_0, run_default_schedule_0080_0,
    DefaultSchedule0080ForbiddenRequests, DefaultSchedule0080Gate, DefaultSchedule0080Input,
    DefaultSchedule0080Location, DefaultSchedule0080PirateState,
    DefaultSchedule0080PirateStepReport, DefaultSchedule0080RunReport, DefaultSchedule0080Step,
    DefaultSchedule0080StepReport, DefaultSchedule0080Surface, DEFAULT_SCHEDULE_0080_0_ID,
    DEFAULT_SCHEDULE_0080_0_SCENARIO, DEFAULT_SCHEDULE_0080_0_STATUS_1A_PASS,
    DEFAULT_SCHEDULE_0080_0_STATUS_1B_PASS,
};
pub use default_schedule_0080_1::{
    replay_default_schedule_0080_1, run_default_schedule_0080_1,
    DefaultSchedule0081BoundaryDecision, DefaultSchedule0081ForbiddenRequests,
    DefaultSchedule0081Gate, DefaultSchedule0081Input, DefaultSchedule0081MovementOutcome,
    DefaultSchedule0081RunReport, DefaultSchedule0081ShipFaction, DefaultSchedule0081Step,
    DefaultSchedule0081StepReport, DefaultSchedule0081Surface, DEFAULT_SCHEDULE_0080_1_ID,
    DEFAULT_SCHEDULE_0080_1_SCENARIO, DEFAULT_SCHEDULE_0080_1_STATUS_PASS,
};
pub use demo_0080_0::{
    canonical_control_input, replay_demo_0080_0, run_demo_0080_0, Demo0080ForbiddenRequests,
    Demo0080Gate, Demo0080Input, Demo0080MovementDay, Demo0080MovementRecord, Demo0080Report,
    Demo0080Surface, DEMO_0080_0_ID, DEMO_0080_0_SCENARIO, DEMO_0080_0_STATUS_PASS,
};
pub use demo_0080_1::{
    canonical_control_input_0080_1, replay_demo_0080_1, run_demo_0080_1, Demo0081CommandRow,
    Demo0081ForbiddenRequests, Demo0081Gate, Demo0081Input, Demo0081MovementRow, Demo0081Report,
    Demo0081Surface, DEMO_0080_1_ID, DEMO_0080_1_SCENARIO, DEMO_0080_1_STATUS_PASS,
};
pub use disruption_decay_0080_2::{
    replay_disruption_decay_0080_2, run_disruption_decay_0080_2, DisruptionDecay0082DecayWeights,
    DisruptionDecay0082ForbiddenRequests, DisruptionDecay0082Gate, DisruptionDecay0082Input,
    DisruptionDecay0082Presence, DisruptionDecay0082Report, DisruptionDecay0082RetentionFactor,
    DisruptionDecay0082Row, DisruptionDecay0082Surface, DISRUPTION_DECAY_0080_2_ID,
    DISRUPTION_DECAY_0080_2_SCENARIO, DISRUPTION_DECAY_0080_2_STATUS_PASS, DISRUPTION_MAX,
    DISRUPTION_SCALE,
};
pub use dress_rehearsal_r1_disruption_heatmap::{
    bounded_feedback_next, cell_index as dress_rehearsal_r1_cell_index,
    cpu_oracle_dress_rehearsal_r1_disruption_heatmap, render_dress_rehearsal_r1_artifact,
    replay_dress_rehearsal_r1_disruption_heatmap, run_dress_rehearsal_r1_disruption_heatmap,
    DressRehearsalR1Artifact, DressRehearsalR1ArtifactRow, DressRehearsalR1CellInput,
    DressRehearsalR1CellInputEntry, DressRehearsalR1Channel, DressRehearsalR1DiffusionRow,
    DressRehearsalR1ForbiddenRequests, DressRehearsalR1Gate, DressRehearsalR1GridCell,
    DressRehearsalR1Hotspot, DressRehearsalR1Input, DressRehearsalR1OccupantContribution,
    DressRehearsalR1OccupantKind, DressRehearsalR1Oracle, DressRehearsalR1Owner,
    DressRehearsalR1RecurrenceRow, DressRehearsalR1Report, DressRehearsalR1Scenario,
    DressRehearsalR1Summary, DressRehearsalR1Surface, CEILING, DECAY, DISRUPTION_COL,
    DRESS_REHEARSAL_R1_DISRUPTION_HEATMAP_ID, DRESS_REHEARSAL_R1_DISRUPTION_HEATMAP_STATUS_PASS,
    DRESS_REHEARSAL_R1_SCENARIO, FLOOR, GAIN, GALAXY_CELL_COUNT, GALAXY_SIDE, HOTSPOT_COUNT,
    H_WEIGHT, LOCATION_STATUS_COL, PATROL_SUPPRESS, PIRATE_EMIT, SYSTEM_COUNT,
};
pub use dress_rehearsal_r2_recursive_allocation::{
    cpu_oracle_dress_rehearsal_r2_recursive_allocation, factory_recipe_production,
    render_dress_rehearsal_r2_artifact, replay_dress_rehearsal_r2_recursive_allocation,
    run_dress_rehearsal_r2_recursive_allocation, DressRehearsalR2AffectedSystemRow,
    DressRehearsalR2Artifact, DressRehearsalR2DeficitDisbursementRow,
    DressRehearsalR2DivertedProductionRow, DressRehearsalR2FactoryRecipe, DressRehearsalR2Input,
    DressRehearsalR2OccupantPosition, DressRehearsalR2Oracle, DressRehearsalR2Owner,
    DressRehearsalR2Report, DressRehearsalR2StockpileLedgerRow, DressRehearsalR2StockpileSeed,
    DressRehearsalR2Summary, DressRehearsalR2SystemProductionRow, BLOCKADE_THRESHOLD,
    DRESS_REHEARSAL_R2_RECURSIVE_ALLOCATION_ID,
    DRESS_REHEARSAL_R2_RECURSIVE_ALLOCATION_STATUS_PASS, DRESS_REHEARSAL_R2_SCENARIO,
    FACTORY_UNIT_COST_LABOR, POP_LABOR_PER_TICK, PRODUCTION_PER_RECIPE, STARPORT_PRODUCTION_NEED,
    TOP_AFFECTED_COUNT,
};
pub use dress_rehearsal_r3_capability_mask_down::{
    apply_modifier_bps, cpu_oracle_dress_rehearsal_r3_capability_mask_down,
    render_dress_rehearsal_r3_artifact, replay_dress_rehearsal_r3_capability_mask_down,
    run_dress_rehearsal_r3_capability_mask_down, DressRehearsalR3Artifact,
    DressRehearsalR3CapabilityRow, DressRehearsalR3Input, DressRehearsalR3ModifiedEconomySignalRow,
    DressRehearsalR3ModifiedR1SignalRow, DressRehearsalR3ModifierOverlayRow,
    DressRehearsalR3Oracle, DressRehearsalR3Owner, DressRehearsalR3OwnerMaskApplicationRow,
    DressRehearsalR3Report, DressRehearsalR3Summary, BLOCKADE_DIVERT_MODIFIER, BPS_ONE,
    COMBAT_BONUS_PLACEHOLDER_MODIFIER, DEFENSIVE_LOGISTICS_MODIFIER, DISRUPTION_DECAY_MODIFIER,
    DRESS_REHEARSAL_R3_CAPABILITY_MASK_DOWN_ID,
    DRESS_REHEARSAL_R3_CAPABILITY_MASK_DOWN_STATUS_PASS, DRESS_REHEARSAL_R3_SCENARIO,
    MAX_MODIFIER_BPS, MIN_MODIFIER_BPS, PATROL_SUPPRESSION_MODIFIER, PIRATE_EMISSION_MODIFIER,
    RAIDING_LOGISTICS_MODIFIER,
};
pub use dress_rehearsal_r4_field_policy_consumption::{
    cpu_mag2_sum, cpu_oracle_dress_rehearsal_r4_field_policy_consumption,
    exact_mag2_bits_from_fixed, f32_to_q16, mag2_u64_q16_to_f32_bits,
    render_dress_rehearsal_r4_artifact, replay_dress_rehearsal_r4_field_policy_consumption,
    run_dress_rehearsal_r4_field_policy_consumption, sqrt_cr_f_bits, DressRehearsalR4Artifact,
    DressRehearsalR4CompositeComponentRow, DressRehearsalR4Decision,
    DressRehearsalR4ExactMagnitudeRow, DressRehearsalR4Input, DressRehearsalR4MoverDecisionRow,
    DressRehearsalR4Oracle, DressRehearsalR4Owner, DressRehearsalR4Report, DressRehearsalR4Summary,
    DRESS_REHEARSAL_R4_FIELD_POLICY_CONSUMPTION_ID,
    DRESS_REHEARSAL_R4_FIELD_POLICY_CONSUMPTION_STATUS_PASS, DRESS_REHEARSAL_R4_SCENARIO,
    MOVEMENT_THRESHOLD_MAG_BITS,
};
pub use dress_rehearsal_r5_movement_reenroll::{
    cpu_oracle_dress_rehearsal_r5_movement_reenroll, render_dress_rehearsal_r5_artifact,
    replay_dress_rehearsal_r5_movement_reenroll, run_dress_rehearsal_r5_movement_reenroll,
    DressRehearsalR5Artifact, DressRehearsalR5BoundaryRequestRow, DressRehearsalR5FissionRow,
    DressRehearsalR5Input, DressRehearsalR5MovementRow, DressRehearsalR5Oracle,
    DressRehearsalR5Owner, DressRehearsalR5Report, DressRehearsalR5SitStillRow,
    DressRehearsalR5Summary, DRESS_REHEARSAL_R5_MOVEMENT_REENROLL_ID,
    DRESS_REHEARSAL_R5_MOVEMENT_REENROLL_STATUS_PASS, DRESS_REHEARSAL_R5_SCENARIO,
    GALACTIC_STRUCTURAL_PARENT,
};
pub use dress_rehearsal_r6_combat_hp_damage::{
    cpu_oracle_dress_rehearsal_r6_combat_hp_damage, damage_output_for_cohort,
    emission_band_ship_attrition, hp_to_retire_for_cohort, render_dress_rehearsal_r6_artifact,
    replay_dress_rehearsal_r6_combat_hp_damage, run_dress_rehearsal_r6_combat_hp_damage,
    DressRehearsalR6Artifact, DressRehearsalR6CombatArenaRow, DressRehearsalR6DefeatedRow,
    DressRehearsalR6DisburseDownRow, DressRehearsalR6FleetCohortOverride, DressRehearsalR6Input,
    DressRehearsalR6Oracle, DressRehearsalR6Owner, DressRehearsalR6ReduceUpRow,
    DressRehearsalR6Report, DressRehearsalR6Summary, DressRehearsalR6SurvivorRow,
    DRESS_REHEARSAL_R6_COMBAT_HP_DAMAGE_ID, DRESS_REHEARSAL_R6_COMBAT_HP_DAMAGE_STATUS_PASS,
    DRESS_REHEARSAL_R6_SCENARIO, FLEET_COHORT_NUM_SHIPS, FLEET_DAMAGE_PER_SHIP_PER_TICK,
    FLEET_HP_PER_SHIP,
};
pub use dress_rehearsal_r6b_ship_cohort_reinforcement::{
    construction_threshold_emission, cpu_oracle_dress_rehearsal_r6b_ship_cohort_reinforcement,
    fleet_cohort_overrides_from_report, replay_dress_rehearsal_r6b_ship_cohort_reinforcement,
    run_dress_rehearsal_r6b_ship_cohort_reinforcement, run_r6_combat_with_r6b_cohorts,
    DressRehearsalR6bBirthRow, DressRehearsalR6bCohortRow, DressRehearsalR6bConstructionRow,
    DressRehearsalR6bFusionRow, DressRehearsalR6bInput, DressRehearsalR6bOracle,
    DressRehearsalR6bOwner, DressRehearsalR6bReinforcementRow, DressRehearsalR6bReport,
    DressRehearsalR6bSummary, DRESS_REHEARSAL_R6B_SCENARIO,
    DRESS_REHEARSAL_R6B_SHIP_COHORT_REINFORCEMENT_ID,
    DRESS_REHEARSAL_R6B_SHIP_COHORT_REINFORCEMENT_STATUS_PASS, R6B_FUSION_FIXTURE_CELL,
    R6B_FUSION_LEFT_ID, R6B_FUSION_RIGHT_ID, SHIP_COST,
};
pub use dress_rehearsal_r6c_integrated_run::{
    cpu_oracle_dress_rehearsal_r6c_integrated_run, render_dress_rehearsal_r6c_artifact,
    replay_dress_rehearsal_r6c_integrated_run, run_dress_rehearsal_r6c_integrated_run,
    DressRehearsalR6cBirthRow, DressRehearsalR6cBoundaryRequestRow,
    DressRehearsalR6cCapabilityOverlayRow, DressRehearsalR6cCombatDisburseRow,
    DressRehearsalR6cCombatReduceRow, DressRehearsalR6cCombatRow, DressRehearsalR6cConservationRow,
    DressRehearsalR6cConstructionRow, DressRehearsalR6cDetectorRow,
    DressRehearsalR6cDetectorStatus, DressRehearsalR6cEconomyRow, DressRehearsalR6cFieldReadRow,
    DressRehearsalR6cFleetCohortState, DressRehearsalR6cFusionRow, DressRehearsalR6cInput,
    DressRehearsalR6cMovementRow, DressRehearsalR6cOracle, DressRehearsalR6cOwner,
    DressRehearsalR6cRaceCurveRow, DressRehearsalR6cReinforcementRow, DressRehearsalR6cReport,
    DressRehearsalR6cStockpileLedgerRow, DressRehearsalR6cSummary, DressRehearsalR6cSystemState,
    DressRehearsalR6cTraceExcerpt, DressRehearsalR6cWorld, DressRehearsalR6cWorldSeedSummary,
    DRESS_REHEARSAL_R6C_INTEGRATED_RUN_ID, DRESS_REHEARSAL_R6C_INTEGRATED_RUN_STATUS_PASS,
    DRESS_REHEARSAL_R6C_SCENARIO, R6C_CANONICAL_TICK_COUNT, R6C_GPU_POSTURE,
    R6C_TIE_BREAKER_POLICY,
};
pub use econ_scale_0080_0::{
    replay_econ_scale_0080_0, run_econ_scale_0080_0, EconScale0080ClearingInput,
    EconScale0080ClearingReport, EconScale0080Faction, EconScale0080FactionIndex,
    EconScale0080ForbiddenRequests, EconScale0080Gate, EconScale0080Input,
    EconScale0080Participant, EconScale0080RunReport, EconScale0080Scenario,
    EconScale0080StarsystemEconomy, EconScale0080Surface, ECON_SCALE_0080_0_DEFAULT_SEED,
    ECON_SCALE_0080_0_FACTION_COUNT, ECON_SCALE_0080_0_ID,
    ECON_SCALE_0080_0_MAX_PARTICIPANTS_PER_STARSYSTEM, ECON_SCALE_0080_0_SCENARIO,
    ECON_SCALE_0080_0_STATUS_PASS,
};
pub use field_scheduler::{
    count_cadence_due_ticks, execute_scheduled_regions_with,
    execute_single_scheduled_stencil_region, visit_scheduled_regions, DirtyRegionState,
    FieldCadence, FieldDispatchDecision, FieldDispatchReason, FieldDispatchSchedule,
    FieldGridDescriptor, FieldId, FieldRegionId, FieldRegionRegistration, FieldScheduleState,
    FieldScheduler, FieldSchedulerError, FieldSchedulerReport, ScheduledRegionsExecutionSummary,
    ScheduledSingleStencilExecution, ScheduledStencilExecutionError,
};
pub use first_slice_mapping_runtime::{
    compiled_cadence_to_field_cadence, compiled_stencil_to_gpu_config, estimate_first_slice_budget,
    FirstSliceCommitmentReport, FirstSliceMappingError, FirstSliceMappingReport,
    FirstSliceMappingSession, FirstSliceReadinessReport, FirstSliceResidencyReport,
    FirstSliceResidencyStatus, FirstSliceSeed, FirstSliceSummaryReport, FirstSliceSummaryStatus,
    FirstSliceTickOptions,
};
pub use gameplay_0080_0::{
    export_gameplay_0080_text, observe_gameplay_0080_0, replay_observe_gameplay_0080_0,
    Gameplay0080ForbiddenRequests, Gameplay0080LocationSummary, Gameplay0080ObservationGate,
    Gameplay0080ObservationInput, Gameplay0080ObservationReport, Gameplay0080ObservationSurface,
    Gameplay0080StepTranscript, Gameplay0080Transcript, GAMEPLAY_0080_0_ID,
    GAMEPLAY_0080_0_SCENARIO, GAMEPLAY_0080_0_STATUS_PASS,
};
pub use gameplay_0080_1::{
    export_gameplay_0080_1_text, observe_gameplay_0080_1, replay_observe_gameplay_0080_1,
    Gameplay0081AtlasSummary, Gameplay0081FactionEconSummary, Gameplay0081ForbiddenRequests,
    Gameplay0081Gate, Gameplay0081Input, Gameplay0081MovementRow, Gameplay0081ObservationReport,
    Gameplay0081StarmapShape, Gameplay0081Summary, Gameplay0081Surface, Gameplay0081Transcript,
    GAMEPLAY_0080_1_ID, GAMEPLAY_0080_1_SCENARIO, GAMEPLAY_0080_1_STATUS_PASS,
};
pub use gated_rates::{
    build_gated_rate_ops, resolve_gated_rates, seed_gated_rate_base_columns, ResolvedGatedRate,
    RATE_BASE_SUB_FIELD,
};
pub use need_weight_profile::{
    binding_from_hydrated_stack, build_need_weight_profile_ops, extract_weighted_accumulator,
    inject_need_threshold_into_economy, prepare_need_weight_participant_cells,
    register_post_rf_need_threshold_rescan, resolve_need_weight_profiles, NeedWeightSourceCell,
    ResolvedNeedWeightProfile,
};
pub use gpu_measure_0080_0::{
    render_gpu_measure_0080_0_report, replay_gpu_measure_0080_0, run_gpu_measure_0080_0,
    GpuMeasure0080AdapterReport, GpuMeasure0080Input, GpuMeasure0080Report,
    GpuMeasure0080ShapeReport, GPU_MEASURE_0080_0_ID, GPU_MEASURE_0080_0_STATUS_PASS,
    GPU_MEASURE_R4_F32_BOUND, GPU_MEASURE_VERDICT_INTEGER_BIT_EXACT,
    GPU_MEASURE_VERDICT_UNMEASURED, GPU_MEASURE_VERDICT_VERIFIED_APPROXIMATE,
};
pub use gradient_follow_0080_2::{
    replay_gradient_follow_0080_2, run_gradient_follow_0080_2, GradientFollow0082ForbiddenRequests,
    GradientFollow0082Gate, GradientFollow0082Input, GradientFollow0082MoveRow,
    GradientFollow0082Report, GradientFollow0082Surface, GRADIENT_FOLLOW_0080_2_ID,
    GRADIENT_FOLLOW_0080_2_SCENARIO, GRADIENT_FOLLOW_0080_2_STATUS_PASS,
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
pub use min_plus_traversal_field::{
    TraversalFieldBandError, TraversalFieldBandSession, TraversalFieldDispatchReport,
    TraversalFieldExecutionMode, TraversalFieldExecutionOptions, TraversalFieldGpuInput,
    TraversalFieldGpuOutputHandle, TraversalFieldGridBinding,
    TraversalFieldShadowColumnCompatInput, TraversalFieldWInputKind,
    TRAVERSAL_FIELD_BAND_DEFAULT_ENABLED, TRAVERSAL_FIELD_ID, TRAVERSAL_FIELD_REGION_ID,
    TRAVERSAL_FIELD_UTILITY_ID,
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
pub use production_path_0080_1::{
    replay_production_path_0080_1, run_production_path_0080_1,
    ProductionPath0081FieldPolicyCompositeGapTerms, ProductionPath0081ForbiddenRequests,
    ProductionPath0081Gate, ProductionPath0081Input, ProductionPath0081OwnerOverlaySummary,
    ProductionPath0081OwnershipAggregationSummary, ProductionPath0081Report,
    ProductionPath0081Scenario, ProductionPath0081Surface, PRODUCTION_PATH_0080_1_ID,
    PRODUCTION_PATH_0080_1_SCENARIO, PRODUCTION_PATH_0080_1_STATUS_PASS, SCENARIO_0080_1_GATE_ID,
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
pub use runtime_0080_0_r0::{
    render_runtime_0080_r0_artifact, replay_runtime_0080_0_r0, run_runtime_0080_0_r0,
    Runtime0080R0AdapterReport, Runtime0080R0Input, Runtime0080R0Report,
    Runtime0080R0ResidencyTraceRow, RUNTIME_0080_0_R0_ID, RUNTIME_0080_0_R0_STATUS_PARTIAL,
    RUNTIME_0080_0_R0_STATUS_PASS, RUNTIME_R0_EXPECTED_R6C_CHECKSUM, RUNTIME_R0_FOREGROUND_CAPTURE,
    RUNTIME_R0_GPU_BLOCKED, RUNTIME_R0_R4_F32_BOUND, RUNTIME_R0_SUBSTRATE_GAP,
    RUNTIME_R0_WHOLE_RUN_GPU_MEASURED, RUNTIME_R0_WHOLE_RUN_PARTIAL,
    RUNTIME_R0_WHOLE_RUN_UNMEASURED,
};
pub use runtime_0080_0_r1a::{
    render_runtime_0080_r1a_artifact, replay_runtime_0080_0_r1a, run_runtime_0080_0_r1a,
    run_runtime_0080_0_r1a_negative_control, run_runtime_0080_0_r1a_with_transforms_enabled,
    Runtime0080R1aAdapterReport, Runtime0080R1aAntiFakeEvidence, Runtime0080R1aBoundarySummary,
    Runtime0080R1aCoveredColumnReport, Runtime0080R1aDisabledTransformRow,
    Runtime0080R1aExactBitProof, Runtime0080R1aInput, Runtime0080R1aInputSource,
    Runtime0080R1aMeasuredCounters, Runtime0080R1aReport, Runtime0080R1aSubstratePrimitiveReport,
    Runtime0080R1aTraceRow, RUNTIME_0080_0_R1A_ID, RUNTIME_0080_0_R1A_PRIMITIVE,
    RUNTIME_0080_0_R1A_STATUS_BLOCKED, RUNTIME_0080_0_R1A_STATUS_PARTIAL,
    RUNTIME_0080_0_R1A_STATUS_PASS, RUNTIME_R1A_EXPECTED_REPORT_CHECKSUM,
    RUNTIME_R1A_REGISTERS_WORLD_GPU_STATE_PIPELINES, RUNTIME_R1A_SCOPE,
};
pub use runtime_0080_0_r1b::{
    render_runtime_0080_r1b_artifact, replay_runtime_0080_0_r1b, run_runtime_0080_0_r1b,
    run_runtime_0080_0_r1b_with_event_writers_enabled, Runtime0080R1bEventWriterParityCheck,
    Runtime0080R1bFreeSlotMarkSource, Runtime0080R1bInput, Runtime0080R1bKindRowCount,
    Runtime0080R1bLocalBirthRequestSource, Runtime0080R1bReport, Runtime0080R1bTraceRow,
    RUNTIME_0080_0_R1B_ID, RUNTIME_0080_0_R1B_PRIMITIVE, RUNTIME_0080_0_R1B_STATUS_BLOCKED,
    RUNTIME_0080_0_R1B_STATUS_PARTIAL, RUNTIME_0080_0_R1B_STATUS_PASS, RUNTIME_R1B_SCOPE,
};
pub use runtime_0080_0_r1c::{
    render_runtime_0080_r1c_artifact, replay_runtime_0080_0_r1c, run_runtime_0080_0_r1c,
    Runtime0080R1cBackpressurePolicy, Runtime0080R1cInput, Runtime0080R1cPredecessorReport,
    Runtime0080R1cReport, Runtime0080R1cShadowContractReport, Runtime0080R1cShadowSnapshot,
    Runtime0080R1cStopLineReport, RUNTIME_0080_0_R1C_ID, RUNTIME_0080_0_R1C_PRIMITIVE,
    RUNTIME_0080_0_R1C_STATUS_BLOCKED, RUNTIME_0080_0_R1C_STATUS_PARTIAL,
    RUNTIME_R1C_EXPECTED_REPORT_CHECKSUM, RUNTIME_R1C_SCOPE,
};
pub use runtime_0080_0_r1c_a::{
    render_runtime_0080_r1c_a_artifact, replay_runtime_0080_0_r1c_a, run_runtime_0080_0_r1c_a,
    run_runtime_0080_0_r1c_a_with_mark_writers_enabled, Runtime0080R1cAInput,
    Runtime0080R1cAMarkTraceRow, Runtime0080R1cAMarkerReport, Runtime0080R1cAPredecessorReport,
    Runtime0080R1cAReport, RUNTIME_0080_0_R1C_A_ID, RUNTIME_0080_0_R1C_A_PRIMITIVE,
    RUNTIME_0080_0_R1C_A_STATUS_BLOCKED, RUNTIME_0080_0_R1C_A_STATUS_PASS,
    RUNTIME_R1C_A_EXPECTED_REPORT_CHECKSUM, RUNTIME_R1C_A_SCOPE,
};
pub use runtime_0080_0_r1c_b::{
    render_runtime_0080_r1c_b_artifact, replay_runtime_0080_0_r1c_b, run_runtime_0080_0_r1c_b,
    run_runtime_0080_0_r1c_b_with_allocation_writers_enabled, Runtime0080R1cBAllocationRow,
    Runtime0080R1cBBoundaryPassReport, Runtime0080R1cBDisabledAllocationWriterCheck,
    Runtime0080R1cBInput, Runtime0080R1cBPreservationSummary, Runtime0080R1cBReport,
    RUNTIME_0080_0_R1C_B_ID, RUNTIME_0080_0_R1C_B_PRIMITIVE, RUNTIME_0080_0_R1C_B_STATUS_BLOCKED,
    RUNTIME_0080_0_R1C_B_STATUS_PARTIAL, RUNTIME_0080_0_R1C_B_STATUS_PASS,
    RUNTIME_R1C_B_EXPECTED_REPORT_CHECKSUM, RUNTIME_R1C_B_SCOPE,
};
pub use runtime_0080_0_r1c_c::{
    render_runtime_0080_r1c_c_artifact, replay_runtime_0080_0_r1c_c, run_runtime_0080_0_r1c_c,
    run_runtime_0080_0_r1c_c_with_membership_writers_enabled, Runtime0080R1cCCpuShadowReport,
    Runtime0080R1cCDisabledMembershipWriterCheck, Runtime0080R1cCInput,
    Runtime0080R1cCMembershipDeltaRow, Runtime0080R1cCPreservationSummary, Runtime0080R1cCReport,
    RUNTIME_0080_0_R1C_C_ID, RUNTIME_0080_0_R1C_C_PRIMITIVE, RUNTIME_0080_0_R1C_C_STATUS_BLOCKED,
    RUNTIME_0080_0_R1C_C_STATUS_PARTIAL, RUNTIME_0080_0_R1C_C_STATUS_PASS,
    RUNTIME_R1C_C_EXPECTED_REPORT_CHECKSUM, RUNTIME_R1C_C_SCOPE,
};
pub use runtime_0080_0_r1c_d::{
    render_runtime_0080_r1c_d_artifact, replay_runtime_0080_0_r1c_d, run_runtime_0080_0_r1c_d,
    run_runtime_0080_0_r1c_d_with_writers_enabled, Runtime0080R1cDCompactedViewRow,
    Runtime0080R1cDCompactionRow, Runtime0080R1cDCpuShadowReport,
    Runtime0080R1cDDisabledWriterCheck, Runtime0080R1cDInput, Runtime0080R1cDLineageRow,
    Runtime0080R1cDReport, RUNTIME_0080_0_R1C_D_ID, RUNTIME_0080_0_R1C_D_PRIMITIVE,
    RUNTIME_0080_0_R1C_D_STATUS_BLOCKED, RUNTIME_0080_0_R1C_D_STATUS_PARTIAL,
    RUNTIME_0080_0_R1C_D_STATUS_PASS, RUNTIME_R1C_D_EXPECTED_REPORT_CHECKSUM, RUNTIME_R1C_D_SCOPE,
};
pub use runtime_0080_0_r1c_e::{
    render_runtime_0080_r1c_e_artifact, replay_runtime_0080_0_r1c_e, run_runtime_0080_0_r1c_e,
    run_runtime_0080_0_r1c_e_with_writers_enabled, Runtime0080R1cECompactedSlotRow,
    Runtime0080R1cECpuShadowReport, Runtime0080R1cEDisabledWriterCheck, Runtime0080R1cEInput,
    Runtime0080R1cEMembershipRemapRow, Runtime0080R1cEReport, Runtime0080R1cESlotRemapRow,
    RUNTIME_0080_0_R1C_E_ID, RUNTIME_0080_0_R1C_E_PRIMITIVE, RUNTIME_0080_0_R1C_E_STATUS_BLOCKED,
    RUNTIME_0080_0_R1C_E_STATUS_PARTIAL, RUNTIME_0080_0_R1C_E_STATUS_PASS,
    RUNTIME_R1C_E_EXPECTED_REPORT_CHECKSUM, RUNTIME_R1C_E_SCOPE,
};
pub use runtime_0080_0_r1c_f::{
    render_runtime_0080_r1c_f_artifact, replay_runtime_0080_0_r1c_f, run_runtime_0080_0_r1c_f,
    run_runtime_0080_0_r1c_f_with_zero_cohort_emitter_enabled, Runtime0080R1cFDisabledEmitterCheck,
    Runtime0080R1cFInput, Runtime0080R1cFReport, Runtime0080R1cFZeroCohortRow,
    RUNTIME_0080_0_R1C_F_ID, RUNTIME_0080_0_R1C_F_PRIMITIVE, RUNTIME_0080_0_R1C_F_STATUS_BLOCKED,
    RUNTIME_0080_0_R1C_F_STATUS_PARTIAL, RUNTIME_0080_0_R1C_F_STATUS_PASS,
    RUNTIME_R1C_F_EXPECTED_REPORT_CHECKSUM, RUNTIME_R1C_F_SCOPE,
};
pub use runtime_0080_0_r2::{
    render_runtime_0080_r2_artifact, run_runtime_0080_0_r2, Runtime0080R2Input,
    Runtime0080R2MemoryFootprint, Runtime0080R2Profiling, Runtime0080R2Report,
    Runtime0080R2TickTimingRow, Runtime0080R2TickTraceRow, RUNTIME_0080_0_R2_ID,
    RUNTIME_0080_0_R2_PRIMITIVE, RUNTIME_0080_0_R2_STATUS_BLOCKED,
    RUNTIME_0080_0_R2_STATUS_PARTIAL, RUNTIME_0080_0_R2_STATUS_PASS,
    RUNTIME_R2_EXPECTED_REPORT_CHECKSUM, RUNTIME_R2_SCOPE,
};
pub use runtime_0080_0_r2_substrate::R2SubstrateOutcome as Runtime0080R2SubstrateOutcome;
pub use runtime_0080_rr_0::{
    build_recursive_world, replay_runtime_0080_rr_0, run_runtime_0080_rr_0,
    Runtime0080Rr0DeviationRecord, Runtime0080Rr0EntityCounts, Runtime0080Rr0FactionStockpile,
    Runtime0080Rr0Galaxy, Runtime0080Rr0GalaxyCell, Runtime0080Rr0Input, Runtime0080Rr0OracleTick,
    Runtime0080Rr0Owner, Runtime0080Rr0Planet, Runtime0080Rr0RecursiveWorld, Runtime0080Rr0Report,
    Runtime0080Rr0ScopeLedgerRow, Runtime0080Rr0Starport, Runtime0080Rr0Surface,
    Runtime0080Rr0SurfaceCell, Runtime0080Rr0SurfaceChild, Runtime0080Rr0System,
    Runtime0080Rr0SystemGridCell, RUNTIME_0080_RR_0_ID, RUNTIME_0080_RR_0_STATUS_BLOCKED,
    RUNTIME_0080_RR_0_STATUS_PARTIAL, RUNTIME_0080_RR_0_STATUS_PASS,
    RUNTIME_RR_0_EXPECTED_REPORT_CHECKSUM,
};
pub use runtime_0080_rr_1::{
    canonical_access_pattern, replay_runtime_0080_rr_1, run_runtime_0080_rr_1,
    try_access_surface_for_system, try_access_system_at_galaxy_cell, Runtime0080Rr1ChildVisibility,
    Runtime0080Rr1DeviationRecord, Runtime0080Rr1Input, Runtime0080Rr1LeakageProof,
    Runtime0080Rr1MappingParityRow, Runtime0080Rr1Report, Runtime0080Rr1ResidencyRequest,
    Runtime0080Rr1ResidencySnapshot, Runtime0080Rr1ScopeLedgerRow, Runtime0080Rr1SystemHandle,
    Runtime0080Rr1TierCounts, Runtime0080Rr1TierId, RR_1_GALAXY_CELL_COUNT, RR_1_GALAXY_SIDE,
    RR_1_SURFACE_CELL_COUNT, RR_1_SURFACE_SIDE, RR_1_SYSTEM_CELL_COUNT, RR_1_SYSTEM_COUNT,
    RR_1_SYSTEM_SIDE, RUNTIME_0080_RR_1_ID, RUNTIME_0080_RR_1_STATUS_BLOCKED,
    RUNTIME_0080_RR_1_STATUS_PARTIAL, RUNTIME_0080_RR_1_STATUS_PASS,
    RUNTIME_RR_1_EXPECTED_REPORT_CHECKSUM,
};
pub use runtime_0080_rr_2::{
    replay_runtime_0080_rr_2, run_runtime_0080_rr_2, Runtime0080Rr2DeviationRecord,
    Runtime0080Rr2Input, Runtime0080Rr2ParityRow, Runtime0080Rr2Report,
    Runtime0080Rr2ScopeLedgerRow, Runtime0080Rr2SurfaceCellBinding, Runtime0080Rr2SurfaceProof,
    RR_2_ACTIVE_SURFACE_COUNT, RR_2_COL_LABOR, RR_2_COL_PRODUCTION, RR_2_SURFACE_CELL_COUNT,
    RR_2_SURFACE_N_DIMS, RUNTIME_0080_RR_2_ID, RUNTIME_0080_RR_2_STATUS_BLOCKED,
    RUNTIME_0080_RR_2_STATUS_PARTIAL, RUNTIME_0080_RR_2_STATUS_PASS,
    RUNTIME_RR_2_EXPECTED_REPORT_CHECKSUM,
};
pub use runtime_0080_rr_3::{
    replay_runtime_0080_rr_3, run_runtime_0080_rr_3, Runtime0080Rr3DeviationRecord,
    Runtime0080Rr3Input, Runtime0080Rr3Report, Runtime0080Rr3ScopeLedgerRow,
    Runtime0080Rr3SystemBinding, Runtime0080Rr3TierTransition, Runtime0080Rr3TransitionRow,
    RR_3_COL_LABOR, RR_3_COL_PRODUCTION, RR_3_N_DIMS, RR_3_SLOTS_PER_SYSTEM, RUNTIME_0080_RR_3_ID,
    RUNTIME_0080_RR_3_STATUS_BLOCKED, RUNTIME_0080_RR_3_STATUS_PARTIAL,
    RUNTIME_0080_RR_3_STATUS_PASS, RUNTIME_RR_3_EXPECTED_REPORT_CHECKSUM,
};
pub use runtime_0080_rr_4::{
    replay_runtime_0080_rr_4, run_runtime_0080_rr_4, Runtime0080Rr4DeviationRecord,
    Runtime0080Rr4FinalStateRow, Runtime0080Rr4Input, Runtime0080Rr4MemoryFootprint,
    Runtime0080Rr4Profiling, Runtime0080Rr4Report, Runtime0080Rr4ScopeLedgerRow,
    Runtime0080Rr4TickParityRow, Runtime0080Rr4TickTimingRow, RUNTIME_0080_RR_4_ID,
    RUNTIME_0080_RR_4_STATUS_BLOCKED, RUNTIME_0080_RR_4_STATUS_PARTIAL,
    RUNTIME_0080_RR_4_STATUS_PASS, RUNTIME_RR_4_EXPECTED_REPORT_CHECKSUM,
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
    compile_runtime_rf_tick_plan, RuntimeRfTickGpuProofSummary, RuntimeRfTickPlan,
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
pub use session::{RunSummary, SessionError, SimSession, StepOnceOutcome};
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
    compile_structural_link_neighbor_sum_plan, DriverCompileError,
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
