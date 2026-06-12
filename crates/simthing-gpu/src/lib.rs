//! GPU foundation for SimThing.
//!
//! Owns the wgpu device/queue and every persistent GPU buffer the simulation
//! reads or writes. CPU-side preparation, compute pipelines, and the GPU pass
//! sequencer build on top of `WorldGpuState`.

pub use wgpu;
pub mod accumulator_op;
pub mod atlas_mask;
pub mod candidate_f_magnitude;
pub mod context;
pub mod emission_accumulator;
pub mod indexed_scatter;
pub mod intensity_accumulator;
pub mod min_plus_stencil;
pub mod min_plus_traversal_d_probe;
pub mod overlay_orderband;
pub mod overlay_prep;
pub mod passes;
pub mod projection;
pub mod reduction;
pub mod reduction_orderband;
pub mod slot;
pub mod structured_field_stencil;
pub mod transfer_accumulator;
pub mod velocity_accumulator;
pub mod world_state;

pub use accumulator_op::{
    ao_wgsl0_fast_path_compatible, classify_ao_wgsl0_plan, emit_on_threshold_registrations_to_gpu,
    emit_on_threshold_registrations_to_ops, eval_eml_cpu, execute_ops_cpu,
    execute_threshold_ops_cpu, set_debug_readback_allowed, summaries_from_values,
    threshold_registrations_to_ops, AccumulatorInputGpu, AccumulatorInputListTable,
    AccumulatorOpGpu, AccumulatorOpSession, AccumulatorOpSessionError, AoWgsl0Compatibility,
    AoWgsl0FallbackReason, AoWgsl0PlanShape, EmissionOpPlanSignature, EmissionRecord,
    EmlGpuProgramTable, EmlTreeRangeGpu, EmlUploadError, EncodeError, ExactnessClass,
    InputListRange, IntensityEmlOpPlanSignature, LegacyOracleFamily, OpSetHandle, OperationFamily,
    OverlayCompileCache, SlotSummary, ThresholdEmission, ThresholdEmissionGpu,
    TransferOpPlanSignature, WorldAccumulatorRuntime, WorldSummaryRuntime, AO_WGSL0_ENTRY_POINT,
    DEFAULT_EML_NODE_CAPACITY, DEFAULT_EML_TREE_CAPACITY, DEFAULT_INPUT_LIST_CAPACITY,
    DEFAULT_THRESHOLD_EMISSION_CAPACITY,
};
pub use atlas_mask::{
    atlas_cell_index, atlas_config, atlas_dims, atlas_side, atlas_slot_xy, build_flush_atlas,
    combined_fingerprint_hex, corridor_t44_max_error, cpu_atlas_horizon,
    cpu_caller_managed_atlas_protocol, fnv64_hash_f32, full_tile_l_inf, make_atlas_mask_params,
    max_full_tile_error, tile_origin, vram_multiplier, AtlasIsolationMode, AtlasIsolationPolicy,
    AtlasMaskGpuOp, AtlasMaskParamsGpu, AtlasNormalizeVariant, C0AtlasFixtureShape,
    C0_DEFAULT_N_DIMS,
};
pub use candidate_f_magnitude::{
    max_candidate_f_magnitude_bits, write_max_candidate_f_magnitude_bits, CandidateFMagnitudeError,
    GradientPairGpu,
};
pub use context::{GpuContext, GpuInitError};
pub use emission_accumulator::{
    emission_plan_signature_fields, encode_emission_plan, plan_emission_ops, EmissionFormula,
    EmissionPlan, EmissionPlanError, EmissionRegistration, EmissionSyncError,
    FORMULA_KIND_CONSTANT, FORMULA_KIND_EVAL_EML, FORMULA_KIND_IDENTITY_FLOOR, NO_CONSTANT,
    NO_MAX_EMIT, NO_TREE_ID,
};
pub use indexed_scatter::{
    cpu_scatter_indexed, validate_scatter_entries, IndexedScatterError, IndexedScatterOp,
    ScatterEntry,
};
pub use intensity_accumulator::{
    build_intensity_eml_entries, plan_intensity_eml_ops, register_intensity_eml_formulas,
    IntensityEmlEntry, IntensityEmlPlan,
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
pub use overlay_orderband::{plan_overlay_orderband, OverlayOrderBandPlan};
pub use overlay_prep::build_overlay_deltas;
pub use passes::{AccumulatorPipelineSessions, Pipelines};
pub use projection::project_tree_to_values;
pub use reduction::{
    build_column_rule_descriptors, build_column_rules, build_topology, cpu_reduce_oracle,
    cpu_reduce_oracle_call_count, encode_column_rules, reset_cpu_reduce_oracle_call_count,
    ColumnRuleDescriptor, Topology, TopologyState,
};
pub use reduction_orderband::{
    plan_reduction_orderband, reduction_soft_band_for_depth_bucket, ReductionOrderBandPlan,
    ReductionPlanError,
};
pub use slot::{SlotAllocError, SlotAllocator};
pub use structured_field_stencil::{
    cpu_compute_c_at, cpu_horizon, cpu_stencil_step, params_from_config, FieldStencilParamsGpu,
    StructuredFieldExecutionOptions, StructuredFieldExecutionReport,
    StructuredFieldStencilBoundaryMode, StructuredFieldStencilConfig,
    StructuredFieldStencilDebugReport, StructuredFieldStencilError, StructuredFieldStencilMaskMode,
    StructuredFieldStencilOp, StructuredFieldStencilOperator, StructuredFieldStencilSourcePolicy,
    DEFAULT_HORIZON_CAP, EXTENDED_HORIZON_CAP, SATURATING_FLUX_CHI_CFL_MAX,
};
pub use transfer_accumulator::{
    conjunctive_recipe_registration_to_transfer, conjunctive_recipe_registrations_to_transfer,
    discrete_transfer_registration_to_transfer, discrete_transfer_registrations_to_transfer,
    encode_transfer_plan, plan_transfer_ops, TransferInputRef, TransferPlan, TransferPlanError,
    TransferRegistration, TransferSyncError,
};
pub use velocity_accumulator::{
    plan_governed_integration, plan_governed_integration_at_band, plan_velocity_integration,
    GovernedIntegrationPlan, PlannerError, VelocityAccumulatorPlan,
};
pub use world_state::{
    build_governed_pairs, encode_rule, governed_pairs_for_property, GovernedPair, IntentDelta,
    OverlayDelta, SlotDeltaRange, ThresholdEvent, ThresholdRegistration, WorldGpuState,
    CLAMP_BOUNDED, CLAMP_FLOORED, CLAMP_UNBOUNDED, DIR_DOWNWARD, DIR_EITHER, DIR_UPWARD, OP_ADD,
    OP_MULTIPLY, OP_SET, RULE_FIRST, RULE_MAX, RULE_MEAN, RULE_MIN, RULE_SUM, RULE_WEIGHTED_MEAN,
    THRESH_BUF_OUTPUT, THRESH_BUF_VALUES, WEIGHT_COL_NONE,
};
