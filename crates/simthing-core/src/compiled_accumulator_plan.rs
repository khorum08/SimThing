//! Driver-compiled AccumulatorOp execution plans (semantic-free).

use crate::{AccumulatorOp, ColumnIndex};

/// Plan-local structural grid channel id (input/output lanes in a plan-owned
/// `n_dims` buffer). Distinct from registry [`ColumnIndex`] — never a role-pathway
/// global column by itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StructuralScalarChannel(u32);

impl StructuralScalarChannel {
    pub const INPUT: Self = Self(0);
    pub const OUTPUT: Self = Self(1);

    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Seal this plan-local channel into a [`ColumnIndex`] for AccumulatorOp
    /// plans that own their own `n_dims` grid (not property-role pathway columns).
    pub fn into_plan_column(self) -> ColumnIndex {
        ColumnIndex::from_structural_plan_channel(self.0)
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
