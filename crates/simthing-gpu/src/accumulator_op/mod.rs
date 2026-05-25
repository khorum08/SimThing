//! AccumulatorOp v2 Pass B — persistent GPU session and bootstrap kernel.

mod cpu_oracle;
mod encode;
mod session;
mod types;

pub use cpu_oracle::{execute_ops_cpu, CpuOracleError};
pub use encode::EncodeError;
pub use types::AccumulatorOpGpu;
pub use session::{
    set_debug_readback_allowed, AccumulatorOpSession, AccumulatorOpSessionError, WORKGROUP_SIZE,
};
pub use types::{
    combine_kind, consume_kind, gate_kind, scale_kind, source_kind, summaries_from_values,
    slot_checksum, AccumulatorSummaryParams, AccumulatorTickParams, EmissionRecord,
    EmissionRecordGpu, SlotSummary, SlotSummaryGpu, DEFAULT_EMISSION_CAPACITY,
};
