//! Readback bridges for `simthing-gpu` session paths (authority minters stay crate-private).

use super::sealed::{
    EmissionRecord, EmissionRecordGpu, ThresholdEmission, ThresholdEmissionGpu, ThresholdEvent,
    ThresholdEventGpu,
};

pub fn emission_records_from_gpu(gpu: &[EmissionRecordGpu]) -> Vec<EmissionRecord> {
    gpu.iter().map(EmissionRecord::from_gpu_readback).collect()
}

pub fn threshold_emissions_from_gpu(gpu: &[ThresholdEmissionGpu]) -> Vec<ThresholdEmission> {
    gpu.iter()
        .map(ThresholdEmission::from_gpu_readback)
        .collect()
}

pub fn threshold_events_from_gpu(gpu: &[ThresholdEventGpu]) -> Vec<ThresholdEvent> {
    gpu.iter().map(ThresholdEvent::from_gpu_readback).collect()
}

pub fn emission_record_from_cpu_oracle(reg_idx: u32, emit_count: u32) -> EmissionRecord {
    EmissionRecord::from_cpu_oracle(reg_idx, emit_count)
}

pub fn threshold_emission_from_cpu_oracle(
    reg_idx: u32,
    slot: u32,
    col: u32,
    value: f32,
) -> ThresholdEmission {
    ThresholdEmission::from_cpu_oracle(reg_idx, slot, col, value)
}

pub fn threshold_event_from_pass7_readback(
    slot: u32,
    col: u32,
    value: f32,
    event_kind: u32,
) -> ThresholdEvent {
    ThresholdEvent::from_kernel_pass7_readback(slot, col, value, event_kind)
}

pub fn emission_record_from_kernel_emit_event(reg_idx: u32, emit_count: u32) -> EmissionRecord {
    EmissionRecord::from_kernel_emit_event(reg_idx, emit_count)
}
