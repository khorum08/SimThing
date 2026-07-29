//! Sealed write-impact band-crossing deltas (WRITE-DOOR-BAND-DELTA-0).
//!
//! Minted only from fused-pass threshold emissions joined with registration
//! sidecars. External crates cannot forge write-impact evidence.

use crate::registration::{ThresholdRegistration, DIR_DOWNWARD, DIR_EITHER, DIR_UPWARD};
use crate::sealed::ThresholdEmission;

/// Deterministic direction of a crossed band edge.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BandCrossingDirection {
    Rising,
    Falling,
}

/// One sealed write-impact band-crossing delta derived in the fused GPU pass.
///
/// External crates cannot forge band deltas directly:
///
/// ```compile_fail
/// fn external_band_crossing_delta_forge() {
///     let _ = simthing_kernel::BandCrossingDelta {
///         reg_idx: 0,
///         slot: 0,
///         col: 0,
///         threshold: 0.0,
///         direction: simthing_kernel::BandCrossingDirection::Rising,
///         post_value: 0.0,
///         event_kind: 0,
///     };
/// }
/// ```
///
/// External crates cannot forge band deltas via a public named constructor:
///
/// ```compile_fail
/// fn external_band_crossing_delta_named_forge() {
///     let _ = simthing_kernel::BandCrossingDelta::from_fused_threshold_emission(
///         /* emission */ unimplemented!(),
///         /* reg */ unimplemented!(),
///     );
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BandCrossingDelta {
    reg_idx: u32,
    slot: u32,
    col: u32,
    threshold: f32,
    direction: BandCrossingDirection,
    post_value: f32,
    event_kind: u32,
}

impl BandCrossingDelta {
    pub fn reg_idx(&self) -> u32 {
        self.reg_idx
    }

    pub fn slot(&self) -> u32 {
        self.slot
    }

    pub fn col(&self) -> u32 {
        self.col
    }

    pub fn threshold(&self) -> f32 {
        self.threshold
    }

    pub fn direction(&self) -> BandCrossingDirection {
        self.direction
    }

    pub fn post_value(&self) -> f32 {
        self.post_value
    }

    pub fn event_kind(&self) -> u32 {
        self.event_kind
    }

    pub(crate) fn from_fused_threshold_emission(
        emission: &ThresholdEmission,
        reg: &ThresholdRegistration,
    ) -> Option<Self> {
        if emission.slot() != reg.slot || emission.col() != reg.col {
            return None;
        }
        let direction = match reg.direction {
            DIR_UPWARD => BandCrossingDirection::Rising,
            DIR_DOWNWARD => BandCrossingDirection::Falling,
            DIR_EITHER => {
                // Either-direction registrations still emit one evidence row; rising
                // when post is above the edge, otherwise falling.
                if emission.value() > reg.threshold {
                    BandCrossingDirection::Rising
                } else {
                    BandCrossingDirection::Falling
                }
            }
            _ => return None,
        };
        Some(Self {
            reg_idx: emission.reg_idx(),
            slot: emission.slot(),
            col: emission.col(),
            threshold: reg.threshold,
            direction,
            post_value: emission.value(),
            event_kind: reg.event_kind,
        })
    }
}

/// Join fused-pass threshold emissions with registration sidecars into sealed
/// write-impact deltas. Order follows emission append order (`reg_idx` ladder).
///
/// When `anchored_columns` is `Some`, only emissions whose column is listed are
/// retained (Unobserved / non-anchored stores skip write-impact evidence).
pub fn band_crossing_deltas_from_fused_emissions(
    emissions: &[ThresholdEmission],
    regs: &[ThresholdRegistration],
    anchored_columns: Option<&[u32]>,
) -> Vec<BandCrossingDelta> {
    let mut out = Vec::with_capacity(emissions.len());
    for emission in emissions {
        let Some(reg) = regs.get(emission.reg_idx() as usize) else {
            continue;
        };
        if let Some(cols) = anchored_columns {
            if !cols.contains(&emission.col()) {
                continue;
            }
        }
        if let Some(delta) = BandCrossingDelta::from_fused_threshold_emission(emission, reg) {
            out.push(delta);
        }
    }
    out
}

/// CPU-oracle twin: derive sealed deltas from previous/current buffers using the
/// same crossing predicate as Pass 7 / `cpu_oracle_threshold_events`.
pub fn cpu_oracle_band_crossing_deltas(
    previous_values: &[f32],
    values: &[f32],
    previous_output: &[f32],
    output: &[f32],
    n_dims: u32,
    regs: &[ThresholdRegistration],
    anchored_columns: Option<&[u32]>,
) -> Vec<BandCrossingDelta> {
    use crate::registration::THRESH_BUF_OUTPUT;
    use crate::sealed::ThresholdEmission;

    let mut emissions = Vec::new();
    for (reg_idx, r) in regs.iter().enumerate() {
        if let Some(cols) = anchored_columns {
            if !cols.contains(&r.col) {
                continue;
            }
        }
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
            emissions.push(ThresholdEmission::from_cpu_oracle(
                reg_idx as u32,
                r.slot,
                r.col,
                curr,
            ));
        }
    }
    band_crossing_deltas_from_fused_emissions(&emissions, regs, anchored_columns)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registration::THRESH_BUF_VALUES;
    use crate::sealed::ThresholdEmission;

    #[test]
    fn rising_edge_mints_rising_delta() {
        let reg = ThresholdRegistration {
            slot: 1,
            col: 2,
            threshold: 10.0,
            direction: DIR_UPWARD,
            event_kind: 7,
            buffer: THRESH_BUF_VALUES,
        };
        let emission = ThresholdEmission::from_cpu_oracle(0, 1, 2, 10.5);
        let delta = BandCrossingDelta::from_fused_threshold_emission(&emission, &reg).unwrap();
        assert_eq!(delta.direction(), BandCrossingDirection::Rising);
        assert_eq!(delta.post_value(), 10.5);
        assert_eq!(delta.threshold(), 10.0);
        assert_eq!(delta.event_kind(), 7);
    }

    #[test]
    fn exact_edge_landing_is_not_a_rising_crossing() {
        let regs = [ThresholdRegistration {
            slot: 0,
            col: 0,
            threshold: 5.0,
            direction: DIR_UPWARD,
            event_kind: 1,
            buffer: THRESH_BUF_VALUES,
        }];
        let prev = [5.0f32];
        let curr = [5.0f32];
        let deltas =
            cpu_oracle_band_crossing_deltas(&prev, &curr, &[], &[], 1, &regs, Some(&[0]));
        assert!(deltas.is_empty());
    }

    #[test]
    fn multi_edge_jump_emits_ordered_deltas() {
        let regs = [
            ThresholdRegistration {
                slot: 0,
                col: 0,
                threshold: 1.0,
                direction: DIR_UPWARD,
                event_kind: 10,
                buffer: THRESH_BUF_VALUES,
            },
            ThresholdRegistration {
                slot: 0,
                col: 0,
                threshold: 2.0,
                direction: DIR_UPWARD,
                event_kind: 11,
                buffer: THRESH_BUF_VALUES,
            },
        ];
        let prev = [0.5f32];
        let curr = [2.5f32];
        let deltas =
            cpu_oracle_band_crossing_deltas(&prev, &curr, &[], &[], 1, &regs, Some(&[0]));
        assert_eq!(deltas.len(), 2);
        assert_eq!(deltas[0].reg_idx(), 0);
        assert_eq!(deltas[1].reg_idx(), 1);
        assert_eq!(deltas[0].event_kind(), 10);
        assert_eq!(deltas[1].event_kind(), 11);
    }

    #[test]
    fn unobserved_columns_are_filtered_from_write_impact() {
        let regs = [ThresholdRegistration {
            slot: 0,
            col: 3,
            threshold: 1.0,
            direction: DIR_UPWARD,
            event_kind: 1,
            buffer: THRESH_BUF_VALUES,
        }];
        let prev = [0.0f32; 4];
        let mut curr = [0.0f32; 4];
        curr[3] = 2.0;
        let deltas =
            cpu_oracle_band_crossing_deltas(&prev, &curr, &[], &[], 4, &regs, Some(&[0, 1]));
        assert!(deltas.is_empty());
    }
}
