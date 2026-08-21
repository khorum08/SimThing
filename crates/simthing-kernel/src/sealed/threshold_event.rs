//! Sealed threshold decision events (KERNEL-EMISSION-SEAL-0 authority surface).
//!
//! EVENT-GENERATION-STAMP-0: generation is required at every mint. GPU POD is not widened.

use bytemuck::{Pod, Zeroable};

use crate::registration::{ThresholdRegistration, DIR_DOWNWARD, DIR_UPWARD, THRESH_BUF_OUTPUT};

/// One sparse threshold-crossing event emitted by Pass 7.
///
/// External crates cannot forge decision events directly:
///
/// ```compile_fail,E0616
/// fn external_threshold_event_field_access(value: &simthing_kernel::ThresholdEvent) {
///     let _ = value.generation;
/// }
/// ```
///
/// External crates cannot forge decision events via a public named constructor:
///
/// ```compile_fail,E0599
/// fn external_threshold_event_named_forge() {
///     let _ = simthing_kernel::ThresholdEvent::from_boundary_delivery(0, 0, 999.0, 7);
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThresholdEvent {
    slot: u32,
    col: u32,
    value: f32,
    event_kind: u32,
    /// Producing tree generation. Stamped at mint — not a GPU POD field.
    generation: u32,
    production_sealed: bool,
}

/// GPU byte-layout mirror for Pass 7 `event_candidates` (transport only).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ThresholdEventGpu {
    pub slot: u32,
    pub col: u32,
    pub value: f32,
    pub event_kind: u32,
}

impl ThresholdEvent {
    pub fn slot(&self) -> u32 {
        self.slot
    }

    pub fn col(&self) -> u32 {
        self.col
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn event_kind(&self) -> u32 {
        self.event_kind
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn is_production_sealed(&self) -> bool {
        self.production_sealed
    }

    pub(crate) fn from_kernel_pass7_readback(
        slot: u32,
        col: u32,
        value: f32,
        event_kind: u32,
        generation: u32,
    ) -> Self {
        Self {
            slot,
            col,
            value,
            event_kind,
            generation,
            production_sealed: true,
        }
    }

    pub(crate) fn from_gpu_readback(gpu: &ThresholdEventGpu, generation: u32) -> Self {
        Self::from_kernel_pass7_readback(gpu.slot, gpu.col, gpu.value, gpu.event_kind, generation)
    }
}

/// CPU-oracle twin of Pass 7 threshold scan for parity and test fixtures.
///
/// Events are produced only when buffer state crosses a registered threshold — not from
/// caller-picked `(slot, col, value, event_kind)` tuples. `generation` is the producing
/// tree's generation and is stamped on every event by construction.
pub fn cpu_oracle_threshold_events(
    previous_values: &[f32],
    values: &[f32],
    previous_output: &[f32],
    output: &[f32],
    n_dims: u32,
    regs: &[ThresholdRegistration],
    generation: u32,
) -> Vec<ThresholdEvent> {
    let mut events = Vec::new();
    for r in regs {
        let addr = (r.slot * n_dims + r.col) as usize;
        let (prev, curr) = if r.buffer == THRESH_BUF_OUTPUT {
            (previous_output[addr], output[addr])
        } else {
            (previous_values[addr], values[addr])
        };
        let up = prev <= r.threshold && curr > r.threshold;
        let down = prev >= r.threshold && curr < r.threshold;
        let crossed = match r.direction {
            DIR_UPWARD => up,
            DIR_DOWNWARD => down,
            _ => up || down,
        };
        if crossed {
            events.push(ThresholdEvent::from_kernel_pass7_readback(
                r.slot,
                r.col,
                curr,
                r.event_kind,
                generation,
            ));
        }
    }
    events
}
