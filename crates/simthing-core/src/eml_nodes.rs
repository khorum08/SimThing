//! EML stack-machine node layout shared by CPU registry validation and GPU upload.

use bytemuck::{Pod, Zeroable};

pub const EML_STACK_MAX: u32 = 16;

pub mod opcode {
    pub const LITERAL_F32: u32 = 0;
    pub const SLOT_VALUE: u32 = 1;
    pub const PARAM: u32 = 2;

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
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
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
