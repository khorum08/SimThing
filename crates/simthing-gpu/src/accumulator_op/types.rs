//! GPU-layout types for AccumulatorOp v2 Pass B.

use bytemuck::{Pod, Zeroable};
pub use simthing_core::EmlNodeGpu;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct EmlTreeRangeGpu {
    pub node_offset: u32,
    pub node_count: u32,
    pub execution_class: u32,
    pub flags: u32,
}

pub const DEFAULT_EMISSION_CAPACITY: u32 = 1024;
/// Default capacity for the C-1 threshold-crossing buffer. Sized for typical
/// boundary workloads; callers that register many more thresholds should
/// override via `AccumulatorOpSession::new_attached(_, _, _, capacity)`.
/// `BoundaryProtocol::sync_accumulator_threshold_ops` lifts this to
/// `max(n_thresholds, DEFAULT_THRESHOLD_EMISSION_CAPACITY)` so production
/// usage scales with the registered count.
pub const DEFAULT_THRESHOLD_EMISSION_CAPACITY: u32 = 4096;

/// Compact threshold crossing record (C-1 parallel emission stream).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThresholdEmission {
    pub reg_idx: u32,
    pub slot: u32,
    pub col: u32,
    pub value: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ThresholdEmissionGpu {
    pub reg_idx: u32,
    pub slot: u32,
    pub col: u32,
    pub value: f32,
}

/// Production B-4 summary tier (32 B/slot on GPU).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlotSummary {
    pub slot: u32,
    pub flags: u32,
    pub checksum_all: u32,
    pub group_checksums: [u32; 4],
}

/// Compact GPU-resolved emission record (Pass C readback tier).
///
/// Compact emission record written by B-2 `EmitEvent` ops. B-2 owns capacity
/// checks and atomic `emission_count` increments; threshold-gated emission
/// migration lands in C-1/C-8.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmissionRecord {
    pub reg_idx: u32,
    pub emit_count: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct SlotSummaryGpu {
    pub slot: u32,
    pub flags: u32,
    pub checksum_all: u32,
    pub _pad: u32,
    pub group_checksums: [u32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct EmissionRecordGpu {
    pub reg_idx: u32,
    pub emit_count: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct AccumulatorOpGpu {
    pub source_kind: u32,
    pub source_slot: u32,
    pub source_col: u32,
    pub source_count: u32,
    pub combine_kind: u32,
    pub combine_a: u32,
    pub combine_b: u32,
    pub combine_c: u32,
    pub combine_d: u32,
    pub gate_kind: u32,
    pub gate_a: u32,
    pub gate_b: u32,
    pub scale_kind: u32,
    pub scale_a: u32,
    pub consume: u32,
    pub target0_slot: u32,
    pub target0_col: u32,
    pub target1_slot: u32,
    pub target1_col: u32,
    pub target2_slot: u32,
    pub target2_col: u32,
    pub target3_slot: u32,
    pub target3_col: u32,
    pub n_targets: u32,
    pub _pad: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct AccumulatorTickParams {
    pub n_ops: u32,
    pub current_band: u32,
    pub n_slots: u32,
    pub n_dims: u32,
    pub emission_capacity: u32,
    pub threshold_emission_capacity: u32,
    /// C-7: per-tick delta time for IntegrateWithClamp (f32 bit pattern).
    pub dt_bits: u32,
    pub _pad1: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct AccumulatorSummaryParams {
    pub n_slots: u32,
    pub n_dims: u32,
    pub _pad0: u32,
    pub _pad1: u32,
}

pub mod source_kind {
    pub const CONSTANT: u32 = 0;
    pub const SLOT_VALUE: u32 = 1;
    pub const SLOT_RANGE: u32 = 2;
    /// Conjunctive multi-input (MinAcrossInputs). Full 4-input GPU support
    /// lands in E-3. For now encodes the first input only; source_count
    /// carries the declared input count for validation.
    pub const CONJUNCTIVE_CROSSING: u32 = 3;
}

pub mod combine_kind {
    pub const IDENTITY: u32 = 0;
    pub const SUM: u32 = 1;
    pub const MEAN: u32 = 2; // C-5/C-6
    pub const MAX: u32 = 3; // C-6
    pub const MIN: u32 = 4; // C-6
    pub const WEIGHTED_MEAN: u32 = 5; // C-5 (Opus-gated)
    /// C-2 intent delta: `values[slot,col] = values[slot,col] * mul + add`.
    pub const AFFINE_INTENT: u32 = 6; // C-2 (implemented)
    pub const PRODUCT: u32 = 7; // C-4
    pub const LAST_BY_PRIORITY: u32 = 8; // C-4
    pub const INTEGRATE_CLAMP: u32 = 9; // C-7
    pub const CROSSING_FORMULA: u32 = 10; // E-1
    pub const MIN_ACROSS_INPUTS: u32 = 11; // E-3
    pub const EVAL_EML: u32 = 12; // C-8 (Opus-gated)
    pub const FIRST: u32 = 13; // C-6 reduction
}

pub mod gate_kind {
    /// Matches `GateSpec::Threshold` ordinal in simthing-core.
    pub const THRESHOLD: u32 = 1;
    pub const ALWAYS: u32 = 0;
    pub const ORDER_BAND: u32 = 4;
}

pub mod consume_kind {
    pub const NONE: u32 = 0;
    pub const SUBTRACT_FROM_SOURCE: u32 = 1;
    /// Matches `ConsumeMode::SubtractFromAllInputs` ordinal in simthing-core.
    pub const SUBTRACT_FROM_ALL_INPUTS: u32 = 2;
    /// Matches `ConsumeMode::ResetTarget` ordinal in simthing-core.
    pub const RESET_TARGET: u32 = 3;
    /// Matches `ConsumeMode::ScaleTarget` ordinal in simthing-core.
    pub const SCALE_TARGET: u32 = 4;
    /// Matches `ConsumeMode::EmitEvent` ordinal in simthing-core.
    pub const EMIT_EVENT: u32 = 5;
    /// Matches `ConsumeMode::AddToTarget` ordinal in simthing-core.
    pub const ADD_TO_TARGET: u32 = 6;
}

pub mod scale_kind {
    pub const IDENTITY: u32 = 0;
    pub const CONSTANT: u32 = 1;
}

/// Compute per-group XOR checksums for one slot row (CPU oracle for B-4).
pub fn group_checksums(values: &[f32], slot: u32, n_dims: u32) -> [u32; 4] {
    let base = slot as usize * n_dims as usize;
    let group_size = n_dims.div_ceil(4);
    let mut out = [0u32; 4];
    for g in 0..4u32 {
        let start = g * group_size;
        if start >= n_dims {
            continue;
        }
        let end = ((g + 1) * group_size).min(n_dims);
        for col in start..end {
            out[g as usize] ^= values[base + col as usize].to_bits();
        }
    }
    out
}

/// Compute a slot-row checksum from a flat values buffer (CPU reference).
pub fn slot_checksum(values: &[f32], slot: u32, n_dims: u32) -> u32 {
    let base = slot as usize * n_dims as usize;
    let mut checksum = 0u32;
    for col in 0..n_dims as usize {
        checksum ^= values[base + col].to_bits();
    }
    checksum
}

/// Build per-slot summaries for every slot in the matrix.
pub fn summaries_from_values(values: &[f32], n_slots: u32, n_dims: u32) -> Vec<SlotSummary> {
    (0..n_slots)
        .map(|slot| SlotSummary {
            slot,
            flags: 0,
            checksum_all: slot_checksum(values, slot, n_dims),
            group_checksums: group_checksums(values, slot, n_dims),
        })
        .collect()
}

#[cfg(test)]
mod summary_tests {
    use super::*;

    #[test]
    fn b4_summary_format_roundtrip() {
        let gpu = SlotSummaryGpu {
            slot: 3,
            flags: 0,
            checksum_all: 42,
            _pad: 0,
            group_checksums: [1, 2, 3, 4],
        };
        let cpu = SlotSummary {
            slot: gpu.slot,
            flags: gpu.flags,
            checksum_all: gpu.checksum_all,
            group_checksums: gpu.group_checksums,
        };
        assert_eq!(cpu.slot, gpu.slot);
        assert_eq!(cpu.checksum_all, gpu.checksum_all);
        assert_eq!(cpu.group_checksums, gpu.group_checksums);
    }

    #[test]
    fn b4_flags_zero_by_default() {
        let values = vec![1.0, 2.0, 3.0, 4.0];
        let s = summaries_from_values(&values, 1, 4)[0];
        assert_eq!(s.flags, 0);
    }

    #[test]
    fn b4_single_column_change_isolates_one_group() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let mut b = a.clone();
        b[2] = 99.0;
        let sa = summaries_from_values(&a, 1, 6)[0];
        let sb = summaries_from_values(&b, 1, 6)[0];
        assert_eq!(sa.group_checksums[0], sb.group_checksums[0]);
        assert_ne!(sa.group_checksums[1], sb.group_checksums[1]);
    }

    #[test]
    fn b4_n_dims_less_than_four() {
        let values = vec![1.0, 2.0];
        let s = summaries_from_values(&values, 1, 2)[0];
        assert_ne!(s.group_checksums[0], 0);
        assert_eq!(s.group_checksums[2], 0);
        assert_eq!(s.group_checksums[3], 0);
    }

    #[test]
    fn b4_n_dims_sixty_four_groups_cover_all_columns() {
        let values: Vec<f32> = (0..64).map(|i| (i + 1) as f32).collect();
        let s = summaries_from_values(&values, 1, 64)[0];
        assert_ne!(s.group_checksums[0], 0);
        assert_ne!(s.group_checksums[3], 0);
        assert_eq!(s.checksum_all, slot_checksum(&values, 0, 64));
    }
}
