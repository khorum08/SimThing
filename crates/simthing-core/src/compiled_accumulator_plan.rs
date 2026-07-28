//! Driver-compiled AccumulatorOp execution plans (semantic-free).

use crate::{AccumulatorOp, ColumnIndex};

/// Column index for a structural scalar channel in the AccumulatorOp value grid.
///
/// Plan-local channel ids (for example 0 = input grid channel, 1 = output grid channel)
/// are authored structural grid channels, not layout-resolved property role columns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StructuralScalarChannel(pub ColumnIndex);

impl StructuralScalarChannel {
    /// Mint a plan-local authored structural grid channel id.
    pub fn from_authored_channel(raw: u32) -> Self {
        Self(ColumnIndex::from_raw_for_oracle_or_rehearsal(raw as usize))
    }
}

/// AccumulatorOp plan assembled by `simthing-driver` and executed under `simthing-sim` tick ownership.
#[derive(Debug, Clone, PartialEq)]
pub struct CompiledAccumulatorOpPlan {
    pub slot_count: u32,
    pub n_dims: u32,
    pub input_channel: StructuralScalarChannel,
    pub output_channel: StructuralScalarChannel,
    pub ops: Vec<AccumulatorOp>,
}

/// f32 values with magnitude ≤ 2^24 and zero fractional part are exact integers in IEEE-754 single
/// precision — sufficient for the vertical-seed and bounded structural neighbor-sum proofs.
pub const EXACT_INTEGER_F32_BOUND: f32 = (1u32 << 24) as f32;

pub fn is_exact_integer_f32(value: f32) -> bool {
    value.is_finite() && value.fract() == 0.0 && value.abs() <= EXACT_INTEGER_F32_BOUND
}
