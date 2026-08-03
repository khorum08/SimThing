//! BAND-QUANTIZED-DRAW-0 — CostBand is THE resource-sink definition.
//!
//! Observation is the base case (a crossing costs nothing). Action is observation
//! with a CostBand attached. Every sink IS a CostBand; there is no rival sink path
//! and no opt-in mode flag.
//!
//! Algebra (exact by construction):
//!   N = min(floor(V/C), throttle?)
//!   R = V − N·C
//!   V = N·C + R

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// One quantized CostBand draw. `N` is units completed; `R` is the residue.
/// Booleans are depth 1: the same path with `N ∈ {0,1}` — no separate fire branch.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CostBandDraw {
    /// Available value V (operand; often `BandCrossingDelta::post_value()`).
    pub v: f32,
    /// Unit cost C (operand; often `BandCrossingDelta::threshold()`).
    pub c: f32,
    /// Units completed this step: `min(floor(V/C), throttle)`.
    pub n: u32,
    /// Residue: `V − N·C` (exact for the chosen N under f32 arithmetic order).
    pub r: f32,
}

impl CostBandDraw {
    /// Exact conservation for this draw: `R` must be the mint `V − N·C` under
    /// the same f32 ops (definitional exactness — IEEE re-association of
    /// `N·C + (V − N·C)` need not recover `V` bit-exactly).
    pub fn conserves_exactly(self) -> bool {
        let r_expected = self.v - (self.n as f32) * self.c;
        self.r.to_bits() == r_expected.to_bits()
    }

    /// `N` must match the floor/throttle oracle (independent of recomputed R).
    pub fn n_matches_oracle(
        self,
        is_sink: bool,
        throttle_hint_max_per_tick: Option<u32>,
    ) -> bool {
        match cost_band_expected_n(self.v, self.c, is_sink, throttle_hint_max_per_tick) {
            Ok(expected) => self.n == expected,
            Err(_) => false,
        }
    }
}

/// Exact floor/throttle oracle for `N` (does not consult a stored draw's R).
pub fn cost_band_expected_n(
    v: f32,
    c: f32,
    is_sink: bool,
    throttle_hint_max_per_tick: Option<u32>,
) -> Result<u32, CostBandAdmissionError> {
    if !v.is_finite() || v < 0.0 {
        return Err(CostBandAdmissionError::InvalidAvailableValue);
    }
    if !is_sink {
        return Ok(0);
    }
    if !c.is_finite() || c <= 0.0 {
        return Err(CostBandAdmissionError::InvalidUnitCost);
    }
    if let Some(0) = throttle_hint_max_per_tick {
        return Err(CostBandAdmissionError::InvalidThrottle);
    }
    let raw = (v / c).floor();
    let raw_n = if raw <= 0.0 {
        0u32
    } else if raw >= u32::MAX as f32 {
        u32::MAX
    } else {
        raw as u32
    };
    Ok(match throttle_hint_max_per_tick {
        Some(t) => raw_n.min(t),
        None => raw_n,
    })
}

/// Authored CostBand / sink marker for one registration (`event_kind` key).
/// Marker rides the CPU-side semantic table — never the GPU POD.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CostBandRegistrationMarker {
    /// When true, this registration is a sink (CostBand). Observation-only
    /// registrations leave this false — observation remains the base case.
    pub is_sink: bool,
}

/// Optional per-resource default sink marker. Per-registration wins.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CostBandResourceMarker {
    pub is_sink: bool,
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum CostBandAdmissionError {
    #[error("CostBand marker ambiguity: per-registration and per-resource disagree")]
    AmbiguousMarker,
    #[error("CostBand unit cost C must be finite and > 0")]
    InvalidUnitCost,
    #[error("CostBand available value V must be finite and >= 0")]
    InvalidAvailableValue,
    #[error("CostBand throttle_hint_max_per_tick must be > 0 when present")]
    InvalidThrottle,
}

/// Resolve the effective sink marker. Per-registration wins; disagreement is
/// an admission hard-error (never a silent default).
pub fn admit_cost_band_marker(
    registration: Option<CostBandRegistrationMarker>,
    resource: Option<CostBandResourceMarker>,
) -> Result<bool, CostBandAdmissionError> {
    match (registration, resource) {
        (Some(r), Some(res)) if r.is_sink != res.is_sink => {
            Err(CostBandAdmissionError::AmbiguousMarker)
        }
        (Some(r), _) => Ok(r.is_sink),
        (None, Some(res)) => Ok(res.is_sink),
        (None, None) => Ok(false), // observation base case
    }
}

/// Quantize a CostBand draw. When `is_sink` is false, returns `N=0, R=V`
/// (observation: nothing consumed). When true, `N = min(floor(V/C), throttle)`.
///
/// Boolean depth-1 is the same function: callers pass the same path with
/// throttle `Some(1)` or with C large enough that N ∈ {0,1}.
pub fn cost_band_quantize(
    v: f32,
    c: f32,
    is_sink: bool,
    throttle_hint_max_per_tick: Option<u32>,
) -> Result<CostBandDraw, CostBandAdmissionError> {
    if !v.is_finite() || v < 0.0 {
        return Err(CostBandAdmissionError::InvalidAvailableValue);
    }
    if !is_sink {
        // Observation base case: no consumption.
        return Ok(CostBandDraw {
            v,
            c: 0.0,
            n: 0,
            r: v,
        });
    }
    if !c.is_finite() || c <= 0.0 {
        return Err(CostBandAdmissionError::InvalidUnitCost);
    }
    if let Some(0) = throttle_hint_max_per_tick {
        return Err(CostBandAdmissionError::InvalidThrottle);
    }

    // floor(V/C) in u32 domain without inventing a second channel.
    let raw = (v / c).floor();
    let raw_n = if raw <= 0.0 {
        0u32
    } else if raw >= u32::MAX as f32 {
        u32::MAX
    } else {
        raw as u32
    };
    let n = match throttle_hint_max_per_tick {
        Some(t) => raw_n.min(t),
        None => raw_n,
    };
    // R = V − N·C — same order as the conservation identity.
    let r = v - (n as f32) * c;
    Ok(CostBandDraw { v, c, n, r })
}

/// Depth-1 CostBand (boolean / command-deficit degenerate). Same algebra path.
pub fn cost_band_depth_one(
    v: f32,
    c: f32,
    is_sink: bool,
) -> Result<CostBandDraw, CostBandAdmissionError> {
    cost_band_quantize(v, c, is_sink, Some(1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unmarked_observation_is_bit_identical_no_consumption() {
        let d = cost_band_quantize(10.0, 3.0, false, Some(5)).unwrap();
        assert_eq!(d.n, 0);
        assert_eq!(d.r.to_bits(), 10.0f32.to_bits());
        assert!(d.conserves_exactly());
    }

    #[test]
    fn exact_conservation_v_eq_n_c_plus_r_randomized() {
        let mut seed = 0xC05Bu32;
        for _ in 0..200 {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let v = ((seed >> 8) % 10_000) as f32 / 10.0 + 0.1;
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let c = ((seed >> 8) % 500) as f32 / 10.0 + 0.05;
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let throttle = if seed & 1 == 0 {
                Some(((seed >> 16) % 20) + 1)
            } else {
                None
            };
            let d = cost_band_quantize(v, c, true, throttle).unwrap();
            assert!(
                d.conserves_exactly(),
                "V={} C={} N={} R={} recon={}",
                d.v,
                d.c,
                d.n,
                d.r,
                (d.n as f32) * d.c + d.r
            );
        }
    }

    #[test]
    fn planted_lost_remainder_reds() {
        let d = cost_band_quantize(10.0, 3.0, true, None).unwrap();
        // Honest draw: N=3, R=1.
        assert_eq!(d.n, 3);
        assert!((d.r - 1.0).abs() < 1e-6);
        // Planted defect: drop remainder.
        let lost = CostBandDraw {
            v: d.v,
            c: d.c,
            n: d.n,
            r: 0.0,
        };
        assert!(
            !lost.conserves_exactly(),
            "lost remainder must fail conservation"
        );
    }

    #[test]
    fn planted_off_by_one_n_reds() {
        let d = cost_band_quantize(10.0, 3.0, true, None).unwrap();
        // Mutant that only bumps N but recomputes R consistently still "conserves"
        // definitionally — the referee must catch wrong N via the oracle.
        let wrong_n = d.n.saturating_add(1);
        let recomputed_r = d.v - (wrong_n as f32) * d.c;
        let off = CostBandDraw {
            v: d.v,
            c: d.c,
            n: wrong_n,
            r: recomputed_r,
        };
        assert!(
            off.conserves_exactly(),
            "precondition: recomputed-R mutant still definitionally conserves"
        );
        assert!(
            !off.n_matches_oracle(true, None),
            "off-by-one N must fail the floor/throttle oracle even when R is recomputed"
        );
        assert!(d.n_matches_oracle(true, None));
    }

    #[test]
    fn throttle_cap_enforced() {
        let d = cost_band_quantize(100.0, 1.0, true, Some(3)).unwrap();
        assert_eq!(d.n, 3);
        assert!((d.r - 97.0).abs() < 1e-5);
        assert!(d.conserves_exactly());
    }

    #[test]
    fn boolean_depth_one_same_path() {
        // Afford one unit: N=1.
        let fire = cost_band_depth_one(5.0, 4.0, true).unwrap();
        assert_eq!(fire.n, 1);
        assert!(fire.conserves_exactly());
        // Cannot afford: N=0.
        let miss = cost_band_depth_one(3.0, 4.0, true).unwrap();
        assert_eq!(miss.n, 0);
        assert!(miss.conserves_exactly());
        // Same function as full quantize with throttle 1.
        let via = cost_band_quantize(5.0, 4.0, true, Some(1)).unwrap();
        assert_eq!(fire, via);
    }

    #[test]
    fn ambiguous_marker_hard_errors() {
        let err = admit_cost_band_marker(
            Some(CostBandRegistrationMarker { is_sink: true }),
            Some(CostBandResourceMarker { is_sink: false }),
        )
        .unwrap_err();
        assert_eq!(err, CostBandAdmissionError::AmbiguousMarker);
    }

    #[test]
    fn per_registration_wins_when_agreeing_shape() {
        // Agreement: both say sink.
        assert_eq!(
            admit_cost_band_marker(
                Some(CostBandRegistrationMarker { is_sink: true }),
                Some(CostBandResourceMarker { is_sink: true }),
            )
            .unwrap(),
            true
        );
        // Registration alone.
        assert_eq!(
            admit_cost_band_marker(Some(CostBandRegistrationMarker { is_sink: true }), None)
                .unwrap(),
            true
        );
        // Resource alone.
        assert_eq!(
            admit_cost_band_marker(None, Some(CostBandResourceMarker { is_sink: true })).unwrap(),
            true
        );
    }

    /// Planted defect: a separate boolean branch would skip quantize for depth-1.
    /// The production path forbids that — depth-1 must call the same quantize.
    #[test]
    fn planted_separate_boolean_branch_reds() {
        fn defective_boolean_fire(v: f32, c: f32) -> u32 {
            // Forbidden separate path: did-it-fire without residue.
            if v >= c {
                1
            } else {
                0
            }
        }
        let v = 5.0f32;
        let c = 2.0f32;
        let honest = cost_band_depth_one(v, c, true).unwrap();
        let defective_n = defective_boolean_fire(v, c);
        // Defective path agrees on N for this input but loses residue discipline:
        let defective = CostBandDraw {
            v,
            c,
            n: defective_n,
            r: 0.0, // dropped
        };
        assert!(honest.conserves_exactly());
        assert!(
            !defective.conserves_exactly(),
            "separate boolean path that drops residue must RED conservation"
        );
    }
}
