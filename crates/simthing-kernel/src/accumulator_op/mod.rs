//! AccumulatorOp v2 Pass B — persistent GPU session and production-shaped kernel subset.
//!
//! B-2 is a **non-integrated kernel subset**. It does not integrate with
//! `BoundaryProtocol`. Operation-family semantics are authoritative only once
//! their C/E migration PRs pass parity.

mod bootstrap_validate;
mod cpu_oracle;
mod eml_program_table;
mod encode;
mod input_list_table;
mod packed_session_upload;
mod runtime;
mod session;
mod types;
mod wgsl_path;
mod world_summary;

pub use cpu_oracle::{
    eval_eml_cpu, execute_intent_deltas_cpu, execute_ops_cpu, execute_ops_cpu_with_emissions,
    execute_threshold_ops_cpu, CpuOracleError,
};
pub use eml_program_table::{
    EmlGpuProgramTable, EmlUploadError, DEFAULT_EML_NODE_CAPACITY, DEFAULT_EML_TREE_CAPACITY,
};
pub use encode::{
    emit_on_threshold_registrations_to_gpu, emit_on_threshold_registrations_to_ops,
    threshold_registrations_to_ops, validate_intent_deltas_no_duplicate_cells, EncodeError,
};
pub use input_list_table::{
    AccumulatorInputListTable, InputListRange, InputListUploadError, DEFAULT_INPUT_LIST_CAPACITY,
};
pub use packed_session_upload::{
    PackedAccumulatorUpload, PackedIntentUpload, PackedThresholdUpload,
};
pub use runtime::{
    EmissionOpPlanSignature, ExactnessClass, IntensityEmlOpPlanSignature, LegacyOracleFamily,
    OpSetHandle, OperationFamily, OverlayCompileCache, TransferOpPlanSignature,
    WorldAccumulatorRuntime,
};
pub use session::{
    debug_readback_allowed, scoped_debug_readback_allowed, set_debug_readback_allowed,
    AccumulatorOpSession, AccumulatorOpSessionError, DebugReadbackGuard, WORKGROUP_SIZE,
};
pub use types::AccumulatorOpGpu;
pub use types::{
    combine_kind, consume_kind, gate_kind, group_checksums, scale_kind, slot_checksum, source_kind,
    summaries_from_values, AccumulatorInputGpu, AccumulatorSummaryParams, AccumulatorTickParams,
    EmissionRecord, EmissionRecordGpu, EmlTreeRangeGpu, SlotSummary, SlotSummaryGpu,
    ThresholdEmission, ThresholdEmissionGpu, DEFAULT_EMISSION_CAPACITY,
    DEFAULT_THRESHOLD_EMISSION_CAPACITY,
};
pub use wgsl_path::{
    ao_wgsl0_fast_path_compatible, classify_ao_wgsl0_plan, AoWgsl0Compatibility,
    AoWgsl0FallbackReason, AoWgsl0PlanShape, AO_WGSL0_ENTRY_POINT, AO_WGSL0_N_BANDS_UNIFORM_FIELD,
};
pub use world_summary::WorldSummaryRuntime;
