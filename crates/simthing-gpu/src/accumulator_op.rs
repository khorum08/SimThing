//! Thin re-export of kernel accumulator runtime (KERNEL-DISPATCH-INCRATE-0).

pub use simthing_kernel::{
    ao_wgsl0_fast_path_compatible, classify_ao_wgsl0_plan, debug_readback_allowed,
    emit_on_threshold_registrations_to_gpu, emit_on_threshold_registrations_to_ops, eval_eml_cpu,
    execute_intent_deltas_cpu, execute_ops_cpu, execute_ops_cpu_with_emissions,
    execute_threshold_ops_cpu, scoped_debug_readback_allowed, set_debug_readback_allowed,
    summaries_from_values, threshold_registrations_to_ops, AccumulatorInputGpu,
    AccumulatorInputListTable, AccumulatorOpGpu, AccumulatorOpSession, AccumulatorOpSessionError,
    AoWgsl0Compatibility, AoWgsl0FallbackReason, AoWgsl0PlanShape, DebugReadbackGuard,
    EmissionOpPlanSignature, EmlGpuProgramTable, EmlTreeRangeGpu, EmlUploadError, EncodeError,
    ExactnessClass, InputListRange, InputListUploadError, IntensityEmlOpPlanSignature,
    LegacyOracleFamily, OpSetHandle, OperationFamily, OverlayCompileCache, PackedAccumulatorUpload,
    PackedIntentUpload, PackedThresholdUpload, SlotSummary, TransferOpPlanSignature,
    WorldAccumulatorRuntime, WorldSummaryRuntime, AO_WGSL0_ENTRY_POINT, DEFAULT_EML_NODE_CAPACITY,
    DEFAULT_EML_TREE_CAPACITY, DEFAULT_INPUT_LIST_CAPACITY, DEFAULT_THRESHOLD_EMISSION_CAPACITY,
    WORKGROUP_SIZE,
};

pub mod encode {
    pub use simthing_kernel::{
        emit_on_threshold_registrations_to_gpu, emit_on_threshold_registrations_to_ops,
        threshold_registrations_to_ops, validate_intent_deltas_no_duplicate_cells, EncodeError,
    };
}

pub mod types {
    pub use simthing_kernel::AccumulatorOpGpu;
}
