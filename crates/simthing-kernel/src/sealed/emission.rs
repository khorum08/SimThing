//! Sealed emission records (KERNEL-EMISSION-SEAL-0 authority surface).

use bytemuck::{Pod, Zeroable};

pub const DEFAULT_EMISSION_CAPACITY: u32 = 1024;
pub const DEFAULT_THRESHOLD_EMISSION_CAPACITY: u32 = 4096;

/// Compact threshold crossing record (C-1 parallel emission stream).
///
/// External crates cannot forge threshold emissions directly:
///
/// ```compile_fail
/// fn external_threshold_emission_forge() {
///     let _ = simthing_kernel::ThresholdEmission {
///         reg_idx: 0,
///         slot: 0,
///         col: 0,
///         value: 0.0,
///     };
/// }
/// ```
///
/// External crates cannot forge threshold emissions via a public named constructor:
///
/// ```compile_fail
/// fn external_threshold_emission_named_forge() {
///     let _ = simthing_kernel::ThresholdEmission::from_kernel_threshold_crossing(0, 0, 0, 0.0);
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThresholdEmission {
    reg_idx: u32,
    slot: u32,
    col: u32,
    value: f32,
}

impl ThresholdEmission {
    pub fn reg_idx(&self) -> u32 {
        self.reg_idx
    }

    pub fn slot(&self) -> u32 {
        self.slot
    }

    pub fn col(&self) -> u32 {
        self.col
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub(crate) fn from_kernel_threshold_crossing(
        reg_idx: u32,
        slot: u32,
        col: u32,
        value: f32,
    ) -> Self {
        Self {
            reg_idx,
            slot,
            col,
            value,
        }
    }

    pub(crate) fn from_cpu_oracle(reg_idx: u32, slot: u32, col: u32, value: f32) -> Self {
        Self::from_kernel_threshold_crossing(reg_idx, slot, col, value)
    }

    pub(crate) fn from_gpu_readback(gpu: &ThresholdEmissionGpu) -> Self {
        Self::from_kernel_threshold_crossing(gpu.reg_idx, gpu.slot, gpu.col, gpu.value)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ThresholdEmissionGpu {
    pub reg_idx: u32,
    pub slot: u32,
    pub col: u32,
    pub value: f32,
}

/// Compact emission record written by B-2 `EmitEvent` ops.
///
/// External crates cannot forge emission records directly:
///
/// ```compile_fail
/// fn external_emission_record_forge() {
///     let _ = simthing_kernel::EmissionRecord {
///         reg_idx: 0,
///         emit_count: 1,
///     };
/// }
/// ```
///
/// External crates cannot forge emission records via a public named constructor:
///
/// ```compile_fail
/// fn external_emission_record_named_forge() {
///     let _ = simthing_kernel::EmissionRecord::from_kernel_emit_event(0, 1);
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmissionRecord {
    reg_idx: u32,
    emit_count: u32,
}

impl EmissionRecord {
    pub fn reg_idx(&self) -> u32 {
        self.reg_idx
    }

    pub fn emit_count(&self) -> u32 {
        self.emit_count
    }

    pub(crate) fn from_kernel_emit_event(reg_idx: u32, emit_count: u32) -> Self {
        Self {
            reg_idx,
            emit_count,
        }
    }

    pub(crate) fn from_cpu_oracle(reg_idx: u32, emit_count: u32) -> Self {
        Self::from_kernel_emit_event(reg_idx, emit_count)
    }

    pub(crate) fn from_gpu_readback(gpu: &EmissionRecordGpu) -> Self {
        Self::from_kernel_emit_event(gpu.reg_idx, gpu.emit_count)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct EmissionRecordGpu {
    pub reg_idx: u32,
    pub emit_count: u32,
}
