//! EML stack-machine node layout shared by CPU registry validation and GPU upload.

use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

/// Closed execution-resource vocabulary shared by ordinary and field EML.
///
/// Selection is derived from validated program facts; callers never author raw
/// stack or node-buffer sizes.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum EmlResourceClass {
    /// Smallest class covering the measured common formula census.
    #[default]
    CompactStack4,
    /// Compatibility class preserving the former fixed-32 admission envelope.
    LegacyFixed32,
}

impl EmlResourceClass {
    pub const ALL: [Self; 2] = [Self::CompactStack4, Self::LegacyFixed32];

    pub const fn stack_slots(self) -> u32 {
        match self {
            Self::CompactStack4 => 4,
            Self::LegacyFixed32 => 32,
        }
    }

    pub const fn max_tree_nodes(self) -> u32 {
        match self {
            Self::CompactStack4 => 16,
            Self::LegacyFixed32 => 32,
        }
    }

    pub const fn fits(self, node_count: u32, peak_stack: u32) -> bool {
        node_count > 0
            && node_count <= self.max_tree_nodes()
            && peak_stack > 0
            && peak_stack <= self.stack_slots()
    }

    /// Deterministically returns the smallest closed class covering both facts.
    pub fn smallest_fitting(node_count: u32, peak_stack: u32) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|class| class.fits(node_count, peak_stack))
    }

    /// Returns the larger class needed to execute programs from both inputs.
    pub const fn join(self, other: Self) -> Self {
        if self.stack_slots() >= other.stack_slots()
            && self.max_tree_nodes() >= other.max_tree_nodes()
        {
            self
        } else {
            other
        }
    }
}

/// Legacy compatibility ceiling. Admission must use [`EmlResourceClass`].
pub const EML_STACK_MAX: u32 = EmlResourceClass::LegacyFixed32.stack_slots();

pub mod opcode {
    pub const LITERAL_F32: u32 = 0;
    pub const SLOT_VALUE: u32 = 1;
    pub const PARAM: u32 = 2;
    /// FIELD-SWEEP-N4-PARITY-0: read a column from the current sweep target.
    ///
    /// Field-context-only: ordinary slot-local EvalEML admission rejects this opcode.
    pub const TARGET_VALUE: u32 = 3;
    /// FIELD-SWEEP-N4-PARITY-0: read a column from the current ordered neighbor.
    ///
    /// Field-context-only: ordinary slot-local EvalEML admission rejects this opcode.
    pub const NEIGHBOR_VALUE: u32 = 4;

    pub const ADD: u32 = 10;
    pub const SUB: u32 = 11;
    pub const MUL: u32 = 12;
    pub const NEG: u32 = 13;
    pub const DIV: u32 = 14;

    pub const MIN: u32 = 20;
    pub const MAX: u32 = 21;
    pub const CLAMP_BOUNDED: u32 = 22;
    pub const CLAMP_FLOORED: u32 = 23;
    pub const ABS: u32 = 24;
    pub const FLOOR: u32 = 25;

    pub const CMP_LT: u32 = 30;
    pub const CMP_LE: u32 = 31;
    pub const CMP_GT: u32 = 32;
    pub const CMP_GE: u32 = 33;
    pub const CMP_EQ: u32 = 34;

    pub const SELECT: u32 = 40;
    pub const RETURN_TOP: u32 = 50;
}

/// Postfix EML program node (24 B, WGSL-aligned).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct EmlNode {
    pub opcode: u32,
    pub flags: u32,
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: u32,
}

pub fn execution_class_to_u32(class: super::EmlExecutionClass) -> u32 {
    match class {
        super::EmlExecutionClass::ExactDeterministic => 0,
        super::EmlExecutionClass::SoftDeterministic => 1,
        super::EmlExecutionClass::FastApproximate => 2,
        super::EmlExecutionClass::CpuOracleOnly => 3,
    }
}
