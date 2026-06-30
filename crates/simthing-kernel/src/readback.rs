//! Readback bridges for `simthing-gpu` session paths (authority minters stay crate-private).
//!
//! Every bridge returning a sealed authoritative type requires [`ReadbackAuthority`].
//! External crates cannot launder forged GPU POD into sealed events:
//!
//! ```compile_fail
//! fn external_pod_bridge_launder() {
//!     let forged = simthing_kernel::ThresholdEventGpu {
//!         slot: 0,
//!         col: 0,
//!         value: 999.0,
//!         event_kind: 7,
//!     };
//!     let _events = simthing_kernel::threshold_events_from_gpu(&[forged]);
//! }
//! ```
//!
//! External crates cannot launder forged emission POD without readback authority:
//!
//! ```compile_fail
//! fn external_emission_pod_bridge_launder() {
//!     let forged = simthing_kernel::EmissionRecordGpu {
//!         reg_idx: 0,
//!         emit_count: 99,
//!     };
//!     let _records = simthing_kernel::emission_records_from_gpu(&[forged]);
//! }
//! ```

use super::sealed::{
    EmissionRecord, EmissionRecordGpu, ReadbackAuthority, ThresholdEmission, ThresholdEmissionGpu,
    ThresholdEvent, ThresholdEventGpu,
};

pub fn emission_records_from_gpu(
    gpu: &[EmissionRecordGpu],
    _authority: ReadbackAuthority,
) -> Vec<EmissionRecord> {
    gpu.iter().map(EmissionRecord::from_gpu_readback).collect()
}

pub fn threshold_emissions_from_gpu(
    gpu: &[ThresholdEmissionGpu],
    _authority: ReadbackAuthority,
) -> Vec<ThresholdEmission> {
    gpu.iter()
        .map(ThresholdEmission::from_gpu_readback)
        .collect()
}

pub fn threshold_events_from_gpu(
    gpu: &[ThresholdEventGpu],
    _authority: ReadbackAuthority,
) -> Vec<ThresholdEvent> {
    gpu.iter().map(ThresholdEvent::from_gpu_readback).collect()
}

pub fn emission_record_from_cpu_oracle(
    reg_idx: u32,
    emit_count: u32,
    _authority: ReadbackAuthority,
) -> EmissionRecord {
    EmissionRecord::from_cpu_oracle(reg_idx, emit_count)
}

pub fn threshold_emission_from_cpu_oracle(
    reg_idx: u32,
    slot: u32,
    col: u32,
    value: f32,
    _authority: ReadbackAuthority,
) -> ThresholdEmission {
    ThresholdEmission::from_cpu_oracle(reg_idx, slot, col, value)
}

pub fn threshold_event_from_pass7_readback(
    slot: u32,
    col: u32,
    value: f32,
    event_kind: u32,
    _authority: ReadbackAuthority,
) -> ThresholdEvent {
    ThresholdEvent::from_kernel_pass7_readback(slot, col, value, event_kind)
}

pub fn emission_record_from_kernel_emit_event(
    reg_idx: u32,
    emit_count: u32,
    _authority: ReadbackAuthority,
) -> EmissionRecord {
    EmissionRecord::from_kernel_emit_event(reg_idx, emit_count)
}
