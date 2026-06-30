//! Packed session-upload packets for AccumulatorOp GPU upload boundaries.
//!
//! Public session upload consumes only packed packets; semantic ops and free GPU row slices
//! must be converted at pack time (see module doc compile_fail proofs).
//!
//! ```compile_fail
//! use simthing_gpu::PackedAccumulatorUpload;
//!
//! fn packed_accumulator_upload_fields_private_compile_fail() {
//!     let _ = PackedAccumulatorUpload {
//!         ops: vec![],
//!         input_list: vec![],
//!     };
//! }
//! ```
//!
//! ```compile_fail
//! use simthing_core::AccumulatorOp;
//! use simthing_gpu::{AccumulatorOpSession, GpuContext, PackedThresholdUpload, PackedIntentUpload};
//!
//! fn session_upload_rejects_accumulator_ops_compile_fail(
//!     session: &mut AccumulatorOpSession,
//!     ctx: &GpuContext,
//!     ops: &[AccumulatorOp],
//! ) {
//!     let _ = session.upload_ops(ctx, ops);
//! }
//! ```
//!
//! ```compile_fail
//! use simthing_gpu::{AccumulatorOpSession, GpuContext, ThresholdRegistration};
//!
//! fn session_upload_rejects_threshold_registrations_compile_fail(
//!     session: &mut AccumulatorOpSession,
//!     ctx: &GpuContext,
//!     regs: &[ThresholdRegistration],
//! ) {
//!     let _ = session.upload_threshold_ops(ctx, regs);
//! }
//! ```
//!
//! ```compile_fail
//! use simthing_gpu::{AccumulatorOpSession, GpuContext, IntentDelta};
//!
//! fn session_upload_rejects_intent_deltas_compile_fail(
//!     session: &mut AccumulatorOpSession,
//!     ctx: &GpuContext,
//!     deltas: &[IntentDelta],
//! ) {
//!     let _ = session.upload_intent_ops(ctx, deltas);
//! }
//! ```
//!
//! ```compile_fail
//! use simthing_gpu::{AccumulatorOpGpu, AccumulatorOpSession, GpuContext};
//!
//! fn session_upload_rejects_free_gpu_ops_compile_fail(
//!     session: &mut AccumulatorOpSession,
//!     ctx: &GpuContext,
//!     ops: &[AccumulatorOpGpu],
//! ) {
//!     let _ = session.upload_gpu_ops(ctx, ops);
//! }
//! ```
//!
//! ```compile_fail
//! use simthing_core::{AccumulatorOp, EmlExpressionRegistry};
//! use simthing_gpu::{AccumulatorOpSession, GpuContext};
//!
//! fn session_upload_rejects_eml_registry_argument_compile_fail(
//!     session: &mut AccumulatorOpSession,
//!     ctx: &GpuContext,
//!     ops: &[AccumulatorOp],
//!     eml: Option<&EmlExpressionRegistry>,
//! ) {
//!     let _ = session.upload_ops_with_eml(ctx, ops, eml);
//! }
//! ```

use simthing_core::{AccumulatorOp, EmlExpressionRegistry, InputSpec, SourceSpec};

use crate::registration::ThresholdRegistration;

use crate::world_state::IntentDelta;

use super::bootstrap_validate::validate_no_contention;
use super::encode::{threshold_registrations_to_ops, EncodeError};
use super::input_list_table::InputListRange;
use super::types::{AccumulatorInputGpu, AccumulatorOpGpu};

/// Byte-ready bootstrap / pre-encoded accumulator op upload packet.
#[derive(Debug, Clone, PartialEq)]
pub struct PackedAccumulatorUpload {
    ops: Vec<AccumulatorOpGpu>,
    input_list: Vec<AccumulatorInputGpu>,
}

/// Byte-ready threshold-gated op upload packet with event-kind sidecar.
#[derive(Debug, Clone, PartialEq)]
pub struct PackedThresholdUpload {
    ops: Vec<AccumulatorOpGpu>,
    threshold_event_kinds: Vec<u32>,
}

/// Byte-ready folded intent upload packet.
#[derive(Debug, Clone, PartialEq)]
pub struct PackedIntentUpload {
    ops: Vec<AccumulatorOpGpu>,
}

impl PackedAccumulatorUpload {
    pub fn from_ops(ops: &[AccumulatorOp]) -> Result<Self, EncodeError> {
        Self::from_ops_with_eml(ops, None)
    }

    pub fn from_ops_with_eml(
        ops: &[AccumulatorOp],
        eml: Option<&EmlExpressionRegistry>,
    ) -> Result<Self, EncodeError> {
        let gpu_ops = AccumulatorOpGpu::encode_bootstrap_set_with_eml(ops, eml)?;
        Ok(Self {
            ops: gpu_ops,
            input_list: Vec::new(),
        })
    }

    pub fn from_ops_resolving_input_lists(ops: &[AccumulatorOp]) -> Result<Self, EncodeError> {
        let mut flat_inputs = Vec::new();
        let mut gpu_ops = Vec::with_capacity(ops.len());
        for op in ops {
            if let SourceSpec::ConjunctiveCrossing { inputs } = &op.source {
                let offset = flat_inputs.len() as u32;
                for InputSpec {
                    slot,
                    col,
                    unit_cost,
                } in inputs
                {
                    flat_inputs.push(AccumulatorInputGpu {
                        slot: slot.raw(),
                        col: col.raw_u32(),
                        unit_cost_bits: unit_cost.to_bits(),
                        flags: 0,
                    });
                }
                let range = InputListRange {
                    offset,
                    count: inputs.len() as u32,
                };
                gpu_ops.push(AccumulatorOpGpu::from_op_with_input_list(op, range)?);
            } else {
                gpu_ops.push(AccumulatorOpGpu::from_op(op)?);
            }
        }
        validate_no_contention(&gpu_ops)?;
        Ok(Self {
            ops: gpu_ops,
            input_list: flat_inputs,
        })
    }

    pub fn from_gpu_ops(ops: Vec<AccumulatorOpGpu>) -> Result<Self, EncodeError> {
        Ok(Self {
            ops,
            input_list: Vec::new(),
        })
    }

    pub fn ops(&self) -> &[AccumulatorOpGpu] {
        &self.ops
    }

    pub fn input_list(&self) -> &[AccumulatorInputGpu] {
        &self.input_list
    }
}

impl PackedThresholdUpload {
    pub fn from_registrations(regs: &[ThresholdRegistration]) -> Result<Self, EncodeError> {
        let (ops, event_kinds) = threshold_registrations_to_ops(regs)?;
        let mut gpu_ops = AccumulatorOpGpu::encode_threshold_set(&ops)?;
        for (op, reg) in gpu_ops.iter_mut().zip(regs) {
            op.source_count = reg.buffer;
        }
        Ok(Self {
            ops: gpu_ops,
            threshold_event_kinds: event_kinds,
        })
    }

    pub fn ops(&self) -> &[AccumulatorOpGpu] {
        &self.ops
    }

    pub fn threshold_event_kinds(&self) -> &[u32] {
        &self.threshold_event_kinds
    }
}

impl PackedIntentUpload {
    pub fn from_deltas(deltas: &[IntentDelta]) -> Result<Self, EncodeError> {
        Ok(Self {
            ops: AccumulatorOpGpu::encode_intent_deltas(deltas)?,
        })
    }

    pub fn ops(&self) -> &[AccumulatorOpGpu] {
        &self.ops
    }
}

#[cfg(test)]
mod tests {
    use simthing_core::{CombineFn, ConsumeMode, GateSpec, ScaleSpec, SlotIndex, SourceSpec};

    use super::*;
    use crate::world_state::{DIR_UPWARD, THRESH_BUF_VALUES};

    fn crossing_op() -> AccumulatorOp {
        AccumulatorOp {
            source: SourceSpec::ConjunctiveCrossing {
                inputs: vec![
                    InputSpec {
                        slot: SlotIndex::new(0),
                        col: simthing_core::ColumnIndex::new(0),
                        unit_cost: 1.0,
                    },
                    InputSpec {
                        slot: SlotIndex::new(1),
                        col: simthing_core::ColumnIndex::new(0),
                        unit_cost: 2.0,
                    },
                ],
            },
            combine: CombineFn::Sum,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::AddToTarget,
            targets: vec![(SlotIndex::new(0), simthing_core::ColumnIndex::new(0))],
        }
    }

    #[test]
    fn packed_accumulator_upload_encodes_same_ops() {
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(0),
                col: simthing_core::ColumnIndex::new(0),
            },
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::AddToTarget,
            targets: vec![(SlotIndex::new(0), simthing_core::ColumnIndex::new(0))],
        };
        let legacy = AccumulatorOpGpu::encode_bootstrap_set(std::slice::from_ref(&op)).unwrap();
        let packed = PackedAccumulatorUpload::from_ops(std::slice::from_ref(&op)).unwrap();
        assert_eq!(packed.ops(), &legacy);
        assert!(packed.input_list().is_empty());
    }

    #[test]
    fn packed_accumulator_upload_resolves_input_lists_same_as_legacy() {
        let op = crossing_op();
        let packed =
            PackedAccumulatorUpload::from_ops_resolving_input_lists(std::slice::from_ref(&op))
                .unwrap();
        assert_eq!(packed.ops().len(), 1);
        assert_eq!(packed.input_list().len(), 2);
        assert_eq!(
            packed.ops()[0].source_kind,
            super::super::types::source_kind::INPUT_LIST
        );
        assert_eq!(packed.ops()[0].source_count, 2);
    }

    #[test]
    fn packed_threshold_upload_preserves_event_kinds_and_source_buffer() {
        let regs = vec![ThresholdRegistration {
            slot: 0,
            col: 0,
            threshold: 1.0,
            direction: DIR_UPWARD,
            event_kind: 42,
            buffer: THRESH_BUF_VALUES,
        }];
        let packed = PackedThresholdUpload::from_registrations(&regs).unwrap();
        assert_eq!(packed.threshold_event_kinds(), &[42]);
        assert_eq!(packed.ops().len(), 1);
        assert_eq!(packed.ops()[0].source_count, THRESH_BUF_VALUES);
    }

    #[test]
    fn packed_intent_upload_encoding_preserved() {
        let deltas = vec![IntentDelta {
            slot: 0,
            col: 0,
            mul: 2.0,
            add: 3.0,
        }];
        let legacy = AccumulatorOpGpu::encode_intent_deltas(&deltas).unwrap();
        let packed = PackedIntentUpload::from_deltas(&deltas).unwrap();
        assert_eq!(packed.ops(), &legacy);
    }

    #[test]
    fn packed_gpu_op_upload_preserves_raw_op_bytes() {
        let op = AccumulatorOpGpu::from_op(&AccumulatorOp {
            source: SourceSpec::Constant(7.0),
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::AddToTarget,
            targets: vec![(SlotIndex::new(0), simthing_core::ColumnIndex::new(0))],
        })
        .unwrap();
        let packed = PackedAccumulatorUpload::from_gpu_ops(vec![op]).unwrap();
        assert_eq!(packed.ops(), std::slice::from_ref(&op));
    }
}
