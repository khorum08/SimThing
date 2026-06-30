//! KERNEL-CRATE-EXTRACT-0 — authoritative runtime admission surface.
//!
//! Owns sealed resolved-state write authority, decision/emission records, spatial participation
//! proofs, and kernel-owned GPU readback buffers consumed by `simthing-gpu` / `simthing-sim`.
//! Consumers observe via read-only accessors; producers route through AccumulatorOp / BoundaryProtocol channels.

#![forbid(unsafe_code)]

pub mod cpu_oracle;
pub mod emission_oracle;
pub mod gpu_readback;
pub mod participation;
pub mod readback;
pub mod registration;
pub mod resolved;
pub mod sealed;

pub use cpu_oracle::{
    execute_ops_cpu, execute_ops_cpu_with_emissions, execute_threshold_ops_cpu, CpuOracleError,
};
pub use emission_oracle::{
    cpu_oracle_emission_records, EmissionOracleError, EmissionOracleFormula,
    EmissionOracleRegistration,
};
pub use gpu_readback::{
    EmissionRecordReadback, KernelReadbackError, ThresholdEmissionReadback,
    ThresholdEventCandidatesReadback,
};
pub use participation::{
    validate_and_mint_placed_participants_by_location_id,
    validate_location_ids_have_structural_placements, PlacedParticipant,
    PlacedParticipantValidationError, StructuralGridPlacement,
};
pub use registration::{
    ThresholdRegistration, DIR_DOWNWARD, DIR_EITHER, DIR_UPWARD, THRESH_BUF_OUTPUT,
    THRESH_BUF_VALUES,
};
pub use resolved::ResolvedGpuBuffers;
pub use sealed::{
    cpu_oracle_threshold_events, EmissionRecord, EmissionRecordGpu, ResolvedWriteAuthority,
    ThresholdEmission, ThresholdEmissionGpu, ThresholdEvent, ThresholdEventGpu,
    DEFAULT_EMISSION_CAPACITY, DEFAULT_THRESHOLD_EMISSION_CAPACITY,
};

#[cfg(test)]
mod dependency_budget;
