//! Sealed emission records (KERNEL-EMISSION-SEAL-0 authority surface).
//!
//! EVENT-GENERATION-STAMP-0: sealed CPU-side records carry a generation stamp **by
//! construction** at every mint. GPU POD layouts are **not** widened (hot-loop fence);
//! generation is applied at the seal/readback boundary from the producing tree's
//! generation authority. There is no optional post-hoc stamp step on the production path.

use bytemuck::{Pod, Zeroable};

pub const DEFAULT_EMISSION_CAPACITY: u32 = 1024;
pub const DEFAULT_THRESHOLD_EMISSION_CAPACITY: u32 = 4096;

/// Compact threshold crossing record (C-1 parallel emission stream).
///
/// External crates cannot forge threshold emissions directly:
///
/// ```compile_fail,E0616
/// fn external_threshold_emission_field_access(value: &simthing_kernel::ThresholdEmission) {
///     let _ = value.production_sealed;
/// }
/// ```
///
/// External crates cannot forge threshold emissions via a public named constructor:
///
/// ```compile_fail,E0624
/// fn external_threshold_emission_named_forge() {
///     let _ = simthing_kernel::ThresholdEmission::from_kernel_threshold_crossing(0, 0, 0, 0.0, 0);
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThresholdEmission {
    reg_idx: u32,
    slot: u32,
    col: u32,
    value: f32,
    /// Producing tree generation. Stamped at mint — not a GPU POD field.
    generation: u32,
    /// True only when minted through a production seal door (oracle/readback with generation).
    production_sealed: bool,
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

    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn is_production_sealed(&self) -> bool {
        self.production_sealed
    }

    /// Production seal: generation is mandatory and recorded by construction.
    pub(crate) fn from_kernel_threshold_crossing(
        reg_idx: u32,
        slot: u32,
        col: u32,
        value: f32,
        generation: u32,
    ) -> Self {
        Self {
            reg_idx,
            slot,
            col,
            value,
            generation,
            production_sealed: true,
        }
    }

    pub(crate) fn from_cpu_oracle(
        reg_idx: u32,
        slot: u32,
        col: u32,
        value: f32,
        generation: u32,
    ) -> Self {
        Self::from_kernel_threshold_crossing(reg_idx, slot, col, value, generation)
    }

    pub(crate) fn from_gpu_readback(gpu: &ThresholdEmissionGpu, generation: u32) -> Self {
        Self::from_kernel_threshold_crossing(gpu.reg_idx, gpu.slot, gpu.col, gpu.value, generation)
    }

    /// Planted-defect helper: strip the production seal so egress must reject it.
    #[cfg(test)]
    pub(crate) fn strip_production_seal_for_planted_defect(mut self) -> Self {
        self.production_sealed = false;
        self
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
/// ```compile_fail,E0616
/// fn external_emission_record_field_access(value: &simthing_kernel::EmissionRecord) {
///     let _ = value.production_sealed;
/// }
/// ```
///
/// External crates cannot forge emission records via a public named constructor:
///
/// ```compile_fail,E0624
/// fn external_emission_record_named_forge() {
///     let _ = simthing_kernel::EmissionRecord::from_kernel_emit_event(0, 1, 0);
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmissionRecord {
    reg_idx: u32,
    emit_count: u32,
    /// Producing tree generation. Stamped at mint — not a GPU POD field.
    generation: u32,
    /// True only when minted through a production seal door.
    production_sealed: bool,
}

impl EmissionRecord {
    pub fn reg_idx(&self) -> u32 {
        self.reg_idx
    }

    pub fn emit_count(&self) -> u32 {
        self.emit_count
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn is_production_sealed(&self) -> bool {
        self.production_sealed
    }

    /// Production seal: generation is mandatory and recorded by construction.
    pub(crate) fn from_kernel_emit_event(reg_idx: u32, emit_count: u32, generation: u32) -> Self {
        Self {
            reg_idx,
            emit_count,
            generation,
            production_sealed: true,
        }
    }

    pub(crate) fn from_cpu_oracle(reg_idx: u32, emit_count: u32, generation: u32) -> Self {
        Self::from_kernel_emit_event(reg_idx, emit_count, generation)
    }

    pub(crate) fn from_gpu_readback(gpu: &EmissionRecordGpu, generation: u32) -> Self {
        Self::from_kernel_emit_event(gpu.reg_idx, gpu.emit_count, generation)
    }

    /// Planted-defect helper: strip the production seal so egress must reject it.
    #[cfg(test)]
    pub(crate) fn strip_production_seal_for_planted_defect(mut self) -> Self {
        self.production_sealed = false;
        self
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct EmissionRecordGpu {
    pub reg_idx: u32,
    pub emit_count: u32,
}

#[cfg(test)]
mod generation_stamp_tests {
    use super::*;
    use simthing_core::{
        BackpressurePolicy, GenerationStamp, StampedEgressEntry, StampedEventRing,
    };

    #[test]
    fn production_mint_stamps_generation_without_widening_gpu_pod() {
        assert_eq!(std::mem::size_of::<EmissionRecordGpu>(), 8);
        assert_eq!(std::mem::size_of::<ThresholdEmissionGpu>(), 16);

        let emission = EmissionRecord::from_cpu_oracle(3, 2, 11);
        assert_eq!(emission.reg_idx(), 3);
        assert_eq!(emission.emit_count(), 2);
        assert_eq!(emission.generation(), 11);
        assert!(emission.is_production_sealed());

        let threshold = ThresholdEmission::from_cpu_oracle(1, 4, 0, 1.5, 11);
        assert_eq!(threshold.generation(), 11);
        assert!(threshold.is_production_sealed());
    }

    #[test]
    fn omitting_production_seal_is_rejected_by_stamped_event_egress() {
        let sealed = EmissionRecord::from_cpu_oracle(0, 1, 7);
        let stripped = sealed.strip_production_seal_for_planted_defect();
        assert!(!stripped.is_production_sealed());

        let mut ring = StampedEventRing::admit(4, BackpressurePolicy::OverwriteOldest);
        assert!(
            push_emission_to_production_egress(&mut ring, &sealed).is_ok(),
            "sealed production record must enter egress"
        );
        assert!(
            push_emission_to_production_egress(&mut ring, &stripped).is_err(),
            "planted unsealed bypass must RED at production egress"
        );
    }

    #[test]
    fn successive_generations_stamp_distinctly_and_ring_honors_forced_lag() {
        // Production-sequence referee: generation authority advances 1 → 2,
        // sealed records carry each generation, ring applies backpressure under lag.
        let mut ring = StampedEventRing::admit(1, BackpressurePolicy::OverwriteOldest);

        for gen in [1u32, 2u32] {
            let sealed = EmissionRecord::from_cpu_oracle(0, gen, gen);
            assert_eq!(sealed.generation(), gen);
            assert!(sealed.is_production_sealed());
            push_emission_to_production_egress(&mut ring, &sealed).unwrap();
        }
        // Capacity 1 + overwrite: only gen 2 remains after forced lag.
        assert_eq!(ring.len(), 1);
        assert_eq!(ring.entries()[0].generation, GenerationStamp::new(2));
        assert_eq!(ring.backpressure_actions, 1);

        // CPU oracle parity path stamps the same generation (not literal 0).
        let parity = EmissionRecord::from_cpu_oracle(9, 3, 7);
        assert_eq!(parity.generation(), 7);
        assert_ne!(parity.generation(), 0);
    }

    fn push_emission_to_production_egress(
        ring: &mut StampedEventRing,
        record: &EmissionRecord,
    ) -> Result<(), &'static str> {
        if !record.is_production_sealed() {
            return Err("unstamped/unsealed emission cannot enter production egress");
        }
        ring.push(StampedEgressEntry {
            generation: GenerationStamp::new(record.generation()),
            key: record.reg_idx() as u64,
            payload_bits: record.emit_count() as u64,
        });
        Ok(())
    }
}
