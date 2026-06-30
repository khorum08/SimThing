//! Threshold registration POD and direction/buffer constants.

use bytemuck::{Pod, Zeroable};

pub const DIR_UPWARD: u32 = 0;
pub const DIR_DOWNWARD: u32 = 1;
pub const DIR_EITHER: u32 = 2;

pub const THRESH_BUF_VALUES: u32 = 0;
pub const THRESH_BUF_OUTPUT: u32 = 1;

/// One GPU threshold registration. Resolved (slot, col) pair plus trigger threshold,
/// direction, and opaque `event_kind` for downstream CPU interpretation.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ThresholdRegistration {
    pub slot: u32,
    pub col: u32,
    pub threshold: f32,
    pub direction: u32,
    pub event_kind: u32,
    pub buffer: u32,
}
