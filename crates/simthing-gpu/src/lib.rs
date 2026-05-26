//! GPU foundation for SimThing.
//!
//! Owns the wgpu device/queue and every persistent GPU buffer the simulation
//! reads or writes. CPU-side preparation, compute pipelines, and the GPU pass
//! sequencer build on top of `WorldGpuState`.

pub mod accumulator_op;
pub mod context;
pub mod emission_accumulator;
pub mod intensity_accumulator;
pub mod overlay_orderband;
pub mod overlay_prep;
pub mod passes;
pub mod projection;
pub mod reduction;
pub mod reduction_orderband;
pub mod slot;
pub mod transfer_accumulator;
pub mod velocity_accumulator;
pub mod world_state;

pub use accumulator_op::{
    eval_eml_cpu, execute_ops_cpu, execute_threshold_ops_cpu, set_debug_readback_allowed,
    summaries_from_values, emit_on_threshold_registrations_to_gpu,
    emit_on_threshold_registrations_to_ops, threshold_registrations_to_ops,
    AccumulatorInputGpu, AccumulatorInputListTable,
    AccumulatorOpGpu, AccumulatorOpSession, AccumulatorOpSessionError, EmissionOpPlanSignature,
    EmissionRecord, EmlGpuProgramTable, EmlTreeRangeGpu, EmlUploadError, EncodeError,
    ExactnessClass, InputListRange, IntensityEmlOpPlanSignature, LegacyOracleFamily, OpSetHandle,
    OperationFamily, OverlayCompileCache, SlotSummary, ThresholdEmission, ThresholdEmissionGpu,
    TransferOpPlanSignature, WorldAccumulatorRuntime, WorldSummaryRuntime,
    DEFAULT_EML_NODE_CAPACITY, DEFAULT_EML_TREE_CAPACITY, DEFAULT_INPUT_LIST_CAPACITY,
    DEFAULT_THRESHOLD_EMISSION_CAPACITY,
};
pub use context::{GpuContext, GpuInitError};
pub use emission_accumulator::{
    emission_plan_signature_fields, encode_emission_plan, plan_emission_ops, EmissionFormula,
    EmissionPlan, EmissionPlanError, EmissionRegistration, EmissionSyncError,
    FORMULA_KIND_CONSTANT, FORMULA_KIND_EVAL_EML, FORMULA_KIND_IDENTITY_FLOOR, NO_CONSTANT,
    NO_MAX_EMIT, NO_TREE_ID,
};
pub use intensity_accumulator::{
    build_intensity_eml_entries, plan_intensity_eml_ops, register_intensity_eml_formulas,
    IntensityEmlEntry, IntensityEmlPlan,
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
pub use slot::SlotAllocator;
pub use transfer_accumulator::{
    conjunctive_recipe_registration_to_transfer, conjunctive_recipe_registrations_to_transfer,
    discrete_transfer_registration_to_transfer, discrete_transfer_registrations_to_transfer,
    encode_transfer_plan, plan_transfer_ops, TransferInputRef, TransferPlan, TransferPlanError,
    TransferRegistration, TransferSyncError,
};
pub use velocity_accumulator::{plan_velocity_integration, VelocityAccumulatorPlan};
pub use world_state::{
    build_governed_pairs, encode_rule, GovernedPair, IntentDelta, OverlayDelta, SlotDeltaRange,
    ThresholdEvent, ThresholdRegistration, WorldGpuState, CLAMP_BOUNDED, CLAMP_FLOORED,
    CLAMP_UNBOUNDED, DIR_DOWNWARD, DIR_EITHER, DIR_UPWARD, OP_ADD, OP_MULTIPLY, OP_SET, RULE_FIRST,
    RULE_MAX, RULE_MEAN, RULE_MIN, RULE_SUM, RULE_WEIGHTED_MEAN, THRESH_BUF_OUTPUT,
    THRESH_BUF_VALUES, WEIGHT_COL_NONE,
};
