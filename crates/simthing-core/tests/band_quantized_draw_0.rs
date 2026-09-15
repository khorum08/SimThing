//! BAND-QUANTIZED-DRAW-0 Remand 1 — biting EML + CostBand oracle proofs.

use simthing_core::{
    admit_cost_band_marker, admit_overlay_eml_program, cost_band_depth_one, cost_band_expected_n,
    cost_band_quantize, eval_overlay_eml, magnitude_band_eml_nodes, CostBandRegistrationMarker,
    CostBandResourceMarker, EmlPerProgramCap, EmlPerProgramCapError, PropertyLayout,
    PropertyTransformDelta, SimPropertyId, SubFieldRole, TransformOp,
};
use std::hint::black_box;
use std::time::Instant;

#[test]
fn degenerate_specializations_match_admitted_eml_bits() {
    let cases = [
        (TransformOp::set(-0.0), 91.0, 7.0),
        (TransformOp::add(2.0), 3.0, 7.0),
        (TransformOp::multiply(-2.0), 4.0, 7.0),
    ];
    for (op, current, n) in cases {
        let interpreted = eval_overlay_eml(op.nodes(), current, n);
        let applied = op.apply_with_params(current, n);
        assert_eq!(
            applied.to_bits(),
            interpreted.to_bits(),
            "derived specialization must stay bit-identical to its admitted EML program"
        );
    }
}

/// Secondary microbenchmark: pre-join arithmetic vs the singular EML entry.
/// Binding acceptance is generation-level in `band_quantized_draw_generation_perf`;
/// this residual ratio is reported without an acceptance threshold.
#[inline(never)]
fn prejoin_set_apply(v: f32, _current: f32) -> f32 {
    v
}

#[inline(never)]
fn eml_set_apply(op: &TransformOp, current: f32) -> f32 {
    op.apply(current)
}

#[test]
fn one_node_set_per_op_secondary_measurement() {
    const ITERS: u32 = 500_000;
    const SAMPLES: u32 = 7;
    let v = 0.42f32;
    let op = TransformOp::set(v);

    let mut eml_samples = Vec::new();
    let mut base_samples = Vec::new();
    for _ in 0..SAMPLES {
        let t0 = Instant::now();
        let mut acc = 0u64;
        for i in 0..ITERS {
            acc ^= eml_set_apply(black_box(&op), black_box(i as f32 * 0.0)).to_bits() as u64;
        }
        black_box(acc);
        eml_samples.push(t0.elapsed().as_nanos() as f64 / ITERS as f64);

        let t1 = Instant::now();
        let mut acc2 = 0u64;
        for i in 0..ITERS {
            acc2 ^= prejoin_set_apply(black_box(v), black_box(i as f32 * 0.0)).to_bits() as u64;
        }
        black_box(acc2);
        base_samples.push(t1.elapsed().as_nanos() as f64 / ITERS as f64);
    }
    eml_samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    base_samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let eml_med = eml_samples[SAMPLES as usize / 2];
    let base_med = base_samples[SAMPLES as usize / 2];
    let ratio = if base_med < 1e-12 {
        f64::INFINITY
    } else {
        eml_med / base_med
    };
    // Bit-identical always required (non-negotiable).
    assert_eq!(
        eml_set_apply(&op, 0.0).to_bits(),
        prejoin_set_apply(v, 0.0).to_bits()
    );
    eprintln!(
        "BAND-QUANTIZED-DRAW-0 secondary per-op measurement: specialized_med={eml_med:.3}ns/op \
         prejoin_med={base_med:.3}ns/op ratio={ratio:.2} samples={SAMPLES} iters={ITERS}"
    );
}
