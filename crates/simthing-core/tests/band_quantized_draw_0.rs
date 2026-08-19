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
fn unmarked_registration_observation_is_bit_identical() {
    let a = cost_band_quantize(12.5, 4.0, false, Some(3)).unwrap();
    let b = cost_band_quantize(12.5, 4.0, false, Some(3)).unwrap();
    assert_eq!(a.n, 0);
    assert_eq!(a.r.to_bits(), 12.5f32.to_bits());
    assert_eq!(a.v.to_bits(), b.v.to_bits());
    assert!(a.n_matches_oracle(false, Some(3)));
}

#[test]
fn static_set_is_genuinely_one_node_literal_f32() {
    let op = TransformOp::set(0.75);
    let nodes = op.to_eml_nodes();
    assert_eq!(
        nodes.len(),
        1,
        "Set must be the one-node LITERAL_F32(v) program"
    );
    assert_eq!(nodes[0].opcode, simthing_core::eml_opcode::LITERAL_F32);
    assert_eq!(nodes[0].a, 0.75f32.to_bits());
    assert_eq!(op.apply(999.0).to_bits(), 0.75f32.to_bits());
    // Cap accounting matches representation.
    assert!(admit_overlay_eml_program(nodes, EmlPerProgramCap::new(1)).is_ok());
}

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

#[test]
fn ordinary_overlay_n_dependent_eml_same_path() {
    let nodes = magnitude_band_eml_nodes(1.0, 2.0, 3.0, 2.0, 4.0);
    let op = TransformOp::admit_eml(nodes, EmlPerProgramCap::DEFAULT).expect("admit");
    assert!(op.as_set_literal().is_none());
    assert!(op.nodes().len() > 1);
    let layout = PropertyLayout::standard(0);
    let mut data = vec![0.0f32];
    let delta = PropertyTransformDelta {
        property_id: SimPropertyId(0),
        sub_field_deltas: vec![(SubFieldRole::Amount, op)],
    };
    delta.apply_to_data_with_n(&mut data, &layout, 1.0);
    assert_eq!(data[0].to_bits(), 1.0f32.to_bits());
    delta.apply_to_data_with_n(&mut data, &layout, 3.0);
    assert_eq!(data[0].to_bits(), 2.0f32.to_bits());
    delta.apply_to_data_with_n(&mut data, &layout, 5.0);
    assert_eq!(data[0].to_bits(), 3.0f32.to_bits());
}

#[test]
fn per_program_cap_at_admission_not_optional_helper() {
    let nodes = magnitude_band_eml_nodes(0.0, 1.0, 2.0, 1.0, 2.0);
    assert!(nodes.len() as u32 > 3);
    let err = TransformOp::admit_eml(nodes, EmlPerProgramCap::new(3)).unwrap_err();
    assert!(matches!(
        err,
        EmlPerProgramCapError::ExceedsCap { max_nodes: 3, .. }
    ));
    // Cap-bypass forge is a compile_fail on TransformOp { nodes: ... }
    // (private field). Public API only admits via admit_eml.
}

#[test]
fn off_by_one_n_with_recomputed_r_fails_oracle() {
    let d = cost_band_quantize(10.0, 3.0, true, None).unwrap();
    let wrong_n = d.n + 1;
    let mutant = simthing_core::CostBandDraw {
        v: d.v,
        c: d.c,
        n: wrong_n,
        r: d.v - (wrong_n as f32) * d.c,
    };
    assert!(mutant.conserves_exactly());
    assert!(!mutant.n_matches_oracle(true, None));
    assert_eq!(cost_band_expected_n(10.0, 3.0, true, None).unwrap(), d.n);
}

#[test]
fn runtime_depth_mutation_changes_output_without_rehydration() {
    let op = TransformOp::admit_eml(
        magnitude_band_eml_nodes(1.0, 2.0, 3.0, 2.0, 4.0),
        EmlPerProgramCap::DEFAULT,
    )
    .unwrap();
    let lo = op.apply_with_params(0.0, 1.0);
    let hi = op.apply_with_params(0.0, 5.0);
    assert_ne!(lo.to_bits(), hi.to_bits());
}

#[test]
fn cost_band_marker_and_throttle_surface() {
    assert!(
        admit_cost_band_marker(Some(CostBandRegistrationMarker { is_sink: true }), None).unwrap()
    );
    assert!(admit_cost_band_marker(
        Some(CostBandRegistrationMarker { is_sink: true }),
        Some(CostBandResourceMarker { is_sink: false }),
    )
    .is_err());
    let d = cost_band_depth_one(10.0, 3.0, true).unwrap();
    assert_eq!(d.n, 1);
    assert!(d.n_matches_oracle(true, Some(1)));
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
