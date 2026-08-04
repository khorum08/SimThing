//! EML-EXP-PRIMITIVE-0 consumer: sign-stable Logistic CostBand steering —
//! builder/interpreter/oracle parity and the smooth-curve contract that the
//! SELECT staircase cannot express.

use simthing_core::{
    admit_overlay_eml_program, eval_overlay_eml, logistic_steering_eml_nodes,
    logistic_steering_oracle, magnitude_band_eml_nodes, EmlPerProgramCap,
};

#[test]
fn eml_exp_primitive_0_logistic_steering_admits_and_matches_its_oracle_bit_for_bit() {
    let (lo, hi, k, x0) = (0.25f32, 4.0f32, 0.9f32, 6.0f32);
    let nodes = logistic_steering_eml_nodes(lo, hi, k, x0);
    assert_eq!(nodes.len(), 31, "pinned sign-stable construction is 31 nodes");
    let nodes = admit_overlay_eml_program(nodes, EmlPerProgramCap::DEFAULT)
        .expect("logistic steering admits under the overlay per-program cap");

    let mut previous = f32::NEG_INFINITY;
    let mut n = -40.0f32;
    while n <= 55.0 {
        let evaluated = eval_overlay_eml(&nodes, 0.0, n);
        let oracle = logistic_steering_oracle(lo, hi, k, x0, n);
        assert_eq!(
            evaluated.to_bits(),
            oracle.to_bits(),
            "interpreter/oracle parity at N={n}"
        );
        assert!(
            (lo..=hi).contains(&evaluated),
            "steering output stays inside [lo, hi] at N={n}"
        );
        assert!(
            evaluated >= previous,
            "steering curve is monotone rising at N={n}"
        );
        previous = evaluated;
        n += 0.125;
    }
    // Saturated tails reach the authored endpoints to within curve epsilon.
    assert!((eval_overlay_eml(&nodes, 0.0, -1.0e4) - lo).abs() < 1.0e-5);
    assert!((eval_overlay_eml(&nodes, 0.0, 1.0e4) - hi).abs() < 1.0e-5);
    // Midpoint: the two branches agree at N = x0 (C1 seam, value (lo+hi)/2).
    let seam = eval_overlay_eml(&nodes, 0.0, x0);
    assert!((seam - 0.5 * (lo + hi)).abs() < 1.0e-6);
}

/// The measured consumer necessity: the landed SELECT staircase's best
/// three-level approximation of the smooth steering response carries a
/// worst-case deviation that the curve eliminates — this excess (in basis
/// points of the response span) is the `measured_threshold_excess_bps`
/// evidence the admission ritual consumes.
#[test]
fn eml_exp_primitive_0_staircase_deviation_from_the_smooth_curve_is_measured_nonzero() {
    let (lo, hi, k, x0) = (0.25f32, 4.0f32, 0.9f32, 6.0f32);
    let curve = logistic_steering_eml_nodes(lo, hi, k, x0);
    // The staircase the 6.1b consumer authors today: thresholds bracketing x0,
    // plateau values sampled from the curve's own tiers.
    let mid = 0.5 * (lo + hi);
    let staircase = magnitude_band_eml_nodes(lo, mid, hi, x0 - 1.0, x0 + 1.0);
    let mut worst_abs = 0.0f32;
    let mut n = -40.0f32;
    while n <= 55.0 {
        let smooth = eval_overlay_eml(&curve, 0.0, n);
        let banded = eval_overlay_eml(&staircase, 0.0, n);
        worst_abs = worst_abs.max((smooth - banded).abs());
        n += 0.0625;
    }
    let excess_bps = (worst_abs / (hi - lo) * 10_000.0) as u32;
    assert!(
        excess_bps > 0,
        "staircase deviation must be measured, not asserted"
    );
    eprintln!(
        "EML_EXP_CONSUMER steering staircase worst_abs={worst_abs:.6} excess_bps={excess_bps}"
    );
}
