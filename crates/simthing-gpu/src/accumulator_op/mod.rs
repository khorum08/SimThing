//! AccumulatorOp v2 Pass B — persistent GPU session and production-shaped kernel subset.
//!
//! B-2 is a **non-integrated kernel subset**. It does not integrate with
//! `BoundaryProtocol`. Operation-family semantics are authoritative only once
//! their C/E migration PRs pass parity.

mod bootstrap_validate;
mod cpu_oracle;
mod encode;
mod session;
mod types;

pub use cpu_oracle::{
    execute_ops_cpu, execute_ops_cpu_with_emissions, CpuOracleError,
};
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
