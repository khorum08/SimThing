//! GPU foundation for SimThing.
//!
//! Broad GPU utilities and thin re-export surfaces for the authoritative kernel runtime.
//! Authoritative dispatch, encode, readback, and sealed buffer ownership live in `simthing-kernel`.

pub use wgpu;
pub mod accumulator_op;
pub mod atlas_mask;
pub mod candidate_f_magnitude;
pub mod min_plus_stencil;
pub mod min_plus_traversal_d_probe;
pub mod saturating_flux_choke_threshold;
pub mod scheduled_w_palma_batch;
pub mod stress_compose;
pub mod structural_upload;
pub mod structural_validation;
pub mod structured_field_stencil;
pub mod w_impedance_compose;

pub use atlas_mask::{
    atlas_cell_index, atlas_config, atlas_dims, atlas_side, atlas_slot_xy, build_flush_atlas,
    combined_fingerprint_hex, corridor_t44_max_error, cpu_atlas_horizon,
    cpu_caller_managed_atlas_protocol, fnv64_hash_f32, full_tile_l_inf, make_atlas_mask_params,
    max_full_tile_error, tile_origin, vram_multiplier, AtlasIsolationMode, AtlasIsolationPolicy,
    AtlasMaskGpuOp, AtlasMaskParamsGpu, AtlasNormalizeVariant, C0AtlasFixtureShape,
    C0_DEFAULT_N_DIMS,
};
pub use candidate_f_magnitude::{
    max_candidate_f_magnitude_bits, CandidateFMagnitudeError, CandidateFMagnitudeReport,
    CandidateFMagnitudeRequest, GradientPairGpu,
};
pub use min_plus_stencil::{
    cell_index, cpu_min_plus_d_from_w, cpu_min_plus_relaxation, cpu_min_plus_step, extract_d_flat,
    max_d_field_error, pack_w_and_initial_d, MinPlusPingPongSide, MinPlusStencilConfig,
    MinPlusStencilError, MinPlusStencilOp, MinPlusTraversalDispatchReport,
    MinPlusTraversalExecutionMode, MinPlusTraversalExecutionOptions, MinPlusTraversalFieldOp,
    MinPlusTraversalGpuOutputHandle, MinPlusTraversalInput, MinPlusTraversalWInputKind,
    MIN_PLUS_INF, MIN_PLUS_MAX_ITERATIONS,
};
pub use min_plus_traversal_d_probe::{
    cpu_probe_d_at_candidates, MinPlusTraversalDProbeConfig, MinPlusTraversalDProbeError,
    MinPlusTraversalDProbeOp, MinPlusTraversalDProbeResult, TRAVERSAL_D_PROBE_MAX_CANDIDATES,
};
pub use saturating_flux_choke_threshold::{
    cpu_choke_threshold_oracle, pass1_workgroup_count, SaturatingFluxChokeThresholdConfig,
    SaturatingFluxChokeThresholdError, SaturatingFluxChokeThresholdOp,
    SaturatingFluxChokeThresholdResult, CHOKE_THRESHOLD_COMPACT_FLOATS,
    CHOKE_THRESHOLD_PARTIAL_FLOATS, CHOKE_THRESHOLD_REDUCE_WORKGROUP_SIZE,
};
pub use scheduled_w_palma_batch::{
    dispatch_scheduled_w_palma_chain, dispatch_serial_w_palma_chain, ScheduledWPalmaChainError,
    ScheduledWPalmaChainEvidence,
};
pub use simthing_core::SlotIndex;
pub use simthing_kernel::{
    ao_wgsl0_fast_path_compatible, build_column_rule_descriptors, build_column_rules,
    build_governed_pairs, build_intensity_eml_entries, build_overlay_deltas, build_topology,
    classify_ao_wgsl0_plan, conjunctive_recipe_registration_to_transfer,
    conjunctive_recipe_registrations_to_transfer, cpu_oracle_emission_records,
    cpu_oracle_threshold_events, cpu_reduce_oracle, cpu_reduce_oracle_call_count,
    cpu_scatter_indexed, debug_readback_allowed, discrete_transfer_registration_to_transfer,
    discrete_transfer_registrations_to_transfer, emit_on_threshold_registrations_to_gpu,
    emit_on_threshold_registrations_to_ops, encode_column_rules, encode_emission_plan, encode_rule,
    encode_transfer_plan, eval_eml_cpu, execute_ops_cpu, execute_threshold_ops_cpu,
    governed_pairs_for_property, plan_emission_ops, plan_governed_integration,
    plan_governed_integration_at_band, plan_intensity_eml_ops, plan_overlay_orderband,
    plan_reduction_orderband, plan_transfer_ops, plan_velocity_integration, project_tree_to_values,
    reduction_soft_band_for_depth_bucket, register_intensity_eml_formulas,
    reset_cpu_reduce_oracle_call_count, scoped_debug_readback_allowed, set_debug_readback_allowed,
    summaries_from_values, threshold_registrations_to_ops,
    validate_and_mint_placed_participants_by_location_id,
    validate_location_ids_have_structural_placements, validate_scatter_entries,
    AccumulatorInputGpu, AccumulatorInputListTable, AccumulatorOpGpu, AccumulatorOpSession,
    AccumulatorOpSessionError, AccumulatorPipelineSessions, AoWgsl0Compatibility,
    AoWgsl0FallbackReason, AoWgsl0PlanShape, ColumnRuleDescriptor, DebugReadbackGuard,
    EmissionFormula, EmissionOpPlanSignature, EmissionPlan, EmissionPlanError, EmissionRecord,
    EmissionRecordGpu, EmissionRegistration, EmissionSyncError, EmlGpuProgramTable,
    EmlTreeRangeGpu, EmlUploadError, EncodeError, ExactnessClass, GovernedIntegrationPlan,
    GovernedPair, GpuContext, GpuInitError, IndexedScatterError, IndexedScatterOp, InputListRange,
    IntensityEmlEntry, IntensityEmlOpPlanSignature, IntensityEmlPlan, IntentDelta,
    LegacyOracleFamily, OpSetHandle, OperationFamily, OverlayCompileCache, OverlayDelta,
    OverlayOrderBandPlan, PackedAccumulatorUpload, PackedIntentUpload, PackedThresholdUpload,
    Pipelines, PlacedParticipant, PlacedParticipantValidationError, PlannerError,
    ReductionOrderBandPlan, ReductionPlanError, ResolvedWriteAuthority, ScatterEntry,
    SlotAllocError, SlotAllocator, SlotDeltaRange, SlotSummary, StructuralGridPlacement,
    ThresholdEmission, ThresholdEmissionGpu, ThresholdEvent, ThresholdEventGpu,
    ThresholdRegistration, Topology, TopologyState, TransferInputRef, TransferOpPlanSignature,
    TransferPlan, TransferPlanError, TransferRegistration, TransferSyncError,
    VelocityAccumulatorPlan, WorldAccumulatorRuntime, WorldGpuState, WorldSummaryRuntime,
    AO_WGSL0_ENTRY_POINT, CLAMP_BOUNDED, CLAMP_FLOORED, CLAMP_UNBOUNDED, DEFAULT_EML_NODE_CAPACITY,
    DEFAULT_EML_TREE_CAPACITY, DEFAULT_INPUT_LIST_CAPACITY, DEFAULT_THRESHOLD_EMISSION_CAPACITY,
    DIR_DOWNWARD, DIR_EITHER, DIR_UPWARD, FORMULA_KIND_CONSTANT, FORMULA_KIND_EVAL_EML,
    FORMULA_KIND_IDENTITY_FLOOR, NO_CONSTANT, NO_MAX_EMIT, NO_TREE_ID, OP_ADD, OP_MULTIPLY, OP_SET,
    RULE_FIRST, RULE_MAX, RULE_MEAN, RULE_MIN, RULE_SUM, RULE_WEIGHTED_MEAN, THRESH_BUF_OUTPUT,
    THRESH_BUF_VALUES, WEIGHT_COL_NONE,
};
pub use stress_compose::{
    cpu_stress_compose_oracle, StressComposeConfig, StressComposeError, StressComposeOp,
    StressComposeProfile, STRESS_COMPOSE_MAX_INPUT_FIELDS, STRESS_COMPOSE_MAX_PROFILES,
    STRESS_COMPOSE_WORKGROUP_SIZE, STRESS_OP_MISMATCH, STRESS_OP_OVERLAP, STRESS_OP_VELOCITY,
    STRESS_OP_WEIGHTED,
};
pub use structural_upload::{
    readback_buffer_bytes_blocking, readback_matches_source, readback_pod_blocking,
    readback_structural_upload_blocking, source_row_bytes, upload_structural_rows_to_gpu,
    PackedUpload, StructuralFrameGpuRow, StructuralLinkGpuRow, StructuralLocationGpuRow,
    StructuralUploadError, StructuralUploadGpuBuffers, StructuralUploadGpuReport,
    StructuralUploadReadback, StructuralUploadRows, FRAME_ROW_BYTES, LINK_ROW_BYTES,
    LOCATION_ROW_BYTES,
};
pub use structural_validation::{
    initial_validation_report, scan_for_forbidden_validation_tokens,
    validate_structural_rows_on_gpu, validate_structural_upload_on_gpu, StructuralValidationError,
    StructuralValidationReportGpu, VALIDATION_REPORT_BYTES,
};
pub use structured_field_stencil::{
    cpu_compute_c_at, cpu_compute_choke_at, cpu_compute_choke_readout_at, cpu_horizon,
    cpu_stencil_step, params_from_config, FieldStencilParamsGpu, StructuredFieldExecutionOptions,
    StructuredFieldExecutionReport, StructuredFieldStencilBoundaryMode,
    StructuredFieldStencilConfig, StructuredFieldStencilDebugReport, StructuredFieldStencilError,
    StructuredFieldStencilMaskMode, StructuredFieldStencilOp, StructuredFieldStencilOperator,
    StructuredFieldStencilSourcePolicy, DEFAULT_HORIZON_CAP, EXTENDED_HORIZON_CAP,
    SATURATING_FLUX_CHI_CFL_MAX,
};
pub use w_impedance_compose::{
    cpu_w_impedance_compose_oracle, WImpedanceComposeConfig, WImpedanceComposeError,
    WImpedanceComposeOp, WImpedanceComposeProfile, W_IMPEDANCE_COMPOSE_MAX_PROFILES,
    W_IMPEDANCE_COMPOSE_WORKGROUP_SIZE,
};
