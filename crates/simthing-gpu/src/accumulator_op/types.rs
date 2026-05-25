//! GPU-layout types for AccumulatorOp v2 Pass B.

use bytemuck::{Pod, Zeroable};

pub const DEFAULT_EMISSION_CAPACITY: u32 = 1024;

/// Provisional B-1/B-2 summary tier.
///
/// Final `SlotSummary` shape is not locked until B-4. Do not treat this
/// checksum-only format as the production readback contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlotSummary {
    pub slot:     u32,
    pub checksum: u32,
}

/// Compact GPU-resolved emission record (Pass C readback tier).
///
/// Compact emission record written by B-2 `EmitEvent` ops. B-2 owns capacity
/// checks and atomic `emission_count` increments; threshold-gated emission
/// migration lands in C-1/C-8.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmissionRecord {
    pub reg_idx:    u32,
    pub emit_count: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct SlotSummaryGpu {
    pub slot:     u32,
    pub checksum: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct EmissionRecordGpu {
    pub reg_idx:    u32,
    pub emit_count: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct AccumulatorOpGpu {
    pub source_kind:  u32,
    pub source_slot:  u32,
    pub source_col:   u32,
    pub source_count: u32,
    pub combine_kind: u32,
    pub combine_a:    u32,
    pub combine_b:    u32,
    pub combine_c:    u32,
    pub combine_d:    u32,
    pub gate_kind:    u32,
    pub gate_a:       u32,
    pub gate_b:       u32,
    pub scale_kind:   u32,
    pub scale_a:      u32,
    pub consume:      u32,
    pub target0_slot: u32,
    pub target0_col:  u32,
    pub target1_slot: u32,
    pub target1_col:  u32,
    pub target2_slot: u32,
    pub target2_col:  u32,
    pub target3_slot: u32,
    pub target3_col:  u32,
    pub n_targets:    u32,
    pub _pad:         u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct AccumulatorTickParams {
    pub n_ops:             u32,
    pub current_band:      u32,
    pub n_slots:           u32,
    pub n_dims:            u32,
    pub emission_capacity: u32,
    pub _pad0:             u32,
    pub _pad1:             u32,
    pub _pad2:             u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct AccumulatorSummaryParams {
    pub n_slots: u32,
    pub n_dims:  u32,
    pub _pad0:   u32,
    pub _pad1:   u32,
}

pub mod source_kind {
    pub const CONSTANT:   u32 = 0;
    pub const SLOT_VALUE: u32 = 1;
    pub const SLOT_RANGE: u32 = 2;
}

pub mod combine_kind {
    pub const IDENTITY: u32 = 0;
    pub const SUM:      u32 = 1;
}

pub mod gate_kind {
    pub const ALWAYS:     u32 = 0;
    pub const ORDER_BAND: u32 = 4;
}

pub mod consume_kind {
    pub const NONE:                 u32 = 0;
    pub const SUBTRACT_FROM_SOURCE: u32 = 1;
    /// Matches `ConsumeMode::EmitEvent` ordinal in simthing-core.
    pub const EMIT_EVENT: u32 = 5;
}

pub mod scale_kind {
    pub const IDENTITY: u32 = 0;
    pub const CONSTANT: u32 = 1;
}

/// Compute a slot-row checksum from a flat values buffer (CPU reference).
pub fn slot_checksum(values: &[f32], slot: u32, n_dims: u32) -> u32 {
    let base = slot as usize * n_dims as usize;
    let mut checksum = 0u32;
    for col in 0..n_dims as usize {
        checksum = checksum.wrapping_add(values[base + col].to_bits());
    }
    checksum
}

/// Build per-slot summaries for every slot in the matrix.
pub fn summaries_from_values(values: &[f32], n_slots: u32, n_dims: u32) -> Vec<SlotSummary> {
    (0..n_slots)
        .map(|slot| SlotSummary {
            slot,
            checksum: slot_checksum(values, slot, n_dims),
        })
        .collect()
}
