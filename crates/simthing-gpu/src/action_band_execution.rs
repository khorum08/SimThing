//! Public GPU-foundation surface for the kernel-owned ActionBand operator.

pub use simthing_kernel::accumulator_op::action_band_execution::{
    target_kind as action_band_target_kind, ActionBandActiveInstanceGpu, ActionBandBandGpu,
    ActionBandCrossingBatch, ActionBandDependencyGpu, ActionBandEmissionBindingGpu,
    ActionBandEmissionDestination, ActionBandExecutionBucket, ActionBandExecutionError,
    ActionBandExecutionPlan, ActionBandExecutionReadback, ActionBandGpuExecution,
    ActionBandGpuSession, ActionBandProductionDispatch, ActionBandPropertyWrite,
    ActionBandStateGpu, ActionBandTemplateGpu, ACTIONBAND_INSTANCE_INITIALLY_ACTIVE,
    ACTIONBAND_INSTANCE_SUBORDINATE, ACTIONBAND_NO_PROGRAM, ACTIONBAND_STATE_ACTIVE,
    ACTIONBAND_STATE_TERMINAL,
};
