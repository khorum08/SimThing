//! KERNEL-CRATE-EXTRACT-0 — authoritative runtime admission surface.
//!
//! Owns sealed resolved-state write authority, decision/emission records, spatial participation
//! proofs, and readback bridges consumed by `simthing-gpu` / `simthing-sim`. Consumers observe
//! via read-only accessors; producers route through AccumulatorOp / BoundaryProtocol channels.

#![forbid(unsafe_code)]

pub mod participation;
pub mod readback;
pub mod registration;
pub mod resolved;
pub mod sealed;

pub use participation::{
    validate_and_mint_placed_participants_by_location_id,
    validate_location_ids_have_structural_placements, PlacedParticipant,
    PlacedParticipantValidationError, StructuralGridPlacement,
};
pub use readback::{
    emission_record_from_cpu_oracle, emission_record_from_kernel_emit_event,
    emission_records_from_gpu, threshold_emission_from_cpu_oracle, threshold_emissions_from_gpu,
    threshold_event_from_pass7_readback, threshold_events_from_gpu,
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
