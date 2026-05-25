//! GPU foundation for SimThing.
//!
//! Owns the wgpu device/queue and every persistent GPU buffer the simulation
//! reads or writes. CPU-side preparation, compute pipelines, and the GPU pass
//! sequencer build on top of `WorldGpuState`.

pub mod accumulator_op;
pub mod context;
pub mod overlay_prep;
pub mod passes;
pub mod projection;
pub mod reduction;
pub mod slot;
pub mod world_state;

pub use accumulator_op::{
    execute_ops_cpu, set_debug_readback_allowed, threshold_registrations_to_ops,
    AccumulatorOpGpu, AccumulatorOpSession, AccumulatorOpSessionError, EmissionRecord,
    SlotSummary, ThresholdEmission, ThresholdEmissionGpu, DEFAULT_THRESHOLD_EMISSION_CAPACITY,
};
pub use context::{GpuContext, GpuInitError};
pub use overlay_prep::build_overlay_deltas;
pub use passes::{AccumulatorPipelineSessions, Pipelines};
pub use projection::project_tree_to_values;
pub use reduction::{
    build_column_rule_descriptors, build_column_rules, build_topology, cpu_reduce_oracle,
    encode_column_rules, ColumnRuleDescriptor, Topology, TopologyState,
};
pub use slot::SlotAllocator;
pub use world_state::{
    build_governed_pairs, build_intensity_params, encode_rule, GovernedPair, IntensityParams,
    IntentDelta, OverlayDelta, ReduceParams, SlotDeltaRange, ThresholdEvent, ThresholdRegistration,
    WorldGpuState, CLAMP_BOUNDED, CLAMP_FLOORED, CLAMP_UNBOUNDED, DIR_DOWNWARD, DIR_EITHER,
    DIR_UPWARD, OP_ADD, OP_MULTIPLY, OP_SET, RULE_FIRST, RULE_MAX, RULE_MEAN, RULE_MIN, RULE_SUM,
    RULE_WEIGHTED_MEAN, THRESH_BUF_OUTPUT, THRESH_BUF_VALUES, WEIGHT_COL_NONE,
};
