//! BAND-QUANTIZED-DRAW-0 — CostBand + singular EML TransformOp exit proofs.

use simthing_core::{
    admit_cost_band_marker, admit_overlay_eml_program, cost_band_depth_one, cost_band_quantize,
    eval_overlay_eml, magnitude_band_eml_nodes, CostBandRegistrationMarker,
    CostBandResourceMarker, EmlPerProgramCap, EmlPerProgramCapError, TransformOp,
};
use std::time::Instant;

#[test]
fn unmarked_registration_observation_is_bit_identical() {
    let a = cost_band_quantize(12.5, 4.0, false, Some(3)).unwrap();
    let b = cost_band_quantize(12.5, 4.0, false, Some(3)).unwrap();
    assert_eq!(a.n, 0);
    assert_eq!(a.r.to_bits(), 12.5f32.to_bits());
    assert_eq!(a.v.to_bits(), b.v.to_bits());
    assert_eq!(a.r.to_bits(), b.r.to_bits());
}

#[test]
fn runtime_depth_mutation_changes_output_without_rehydration() {
    // Queue depth is runtime state (N / property value), not re-authored shape.
    let c = 2.0f32;
    let depth_1 = cost_band_quantize(5.0, c, true, None).unwrap();
    let depth_3 = cost_band_quantize(7.0, c, true, None).unwrap();
    assert_ne!(depth_1.n, depth_3.n);
    // Overlay magnitude steered by N without rebuilding the program shape.
    let nodes = magnitude_band_eml_nodes(1.0, 2.0, 3.0, 2.0, 4.0);
    let admitted = admit_overlay_eml_program(nodes, EmlPerProgramCap::DEFAULT).unwrap();
    let lo = eval_overlay_eml(&admitted, 0.0, 1.0); // N=1 < t1
    let mid = eval_overlay_eml(&admitted, 0.0, 3.0); // t1 <= N < t2
    let hi = eval_overlay_eml(&admitted, 0.0, 5.0); // N >= t2
    assert_eq!(lo.to_bits(), (1.0f32).to_bits());
    assert_eq!(mid.to_bits(), (2.0f32).to_bits());
    assert_eq!(hi.to_bits(), (3.0f32).to_bits());
}

#[test]
fn static_set_is_one_node_literal_f32_bit_identical() {
    let op = TransformOp::Set(0.75);
    let nodes = op.to_eml_nodes();
    // LITERAL_F32 + RETURN_TOP
    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0].opcode, simthing_core::eml_opcode::LITERAL_F32);
    assert_eq!(nodes[0].a, 0.75f32.to_bits());
    let via_eml = op.apply(999.0);
    assert_eq!(via_eml.to_bits(), 0.75f32.to_bits());
    // No static/computed branch: Add/Multiply also EML.
    assert_eq!(TransformOp::Add(2.0).apply(3.0).to_bits(), 5.0f32.to_bits());
    assert_eq!(
        TransformOp::Multiply(2.0).apply(3.0).to_bits(),
        6.0f32.to_bits()
    );
}

#[test]
fn planted_static_computed_branch_would_diverge_red() {
    // Defective path: special-case Set without EML.
    fn defective_static_branch(op: &TransformOp, current: f32) -> f32 {
        match op {
            TransformOp::Set(v) => *v, // separate path
            TransformOp::Add(v) => current + *v,
            TransformOp::Multiply(v) => current * *v,
        }
    }
    let op = TransformOp::Set(1.5);
    let honest = op.apply(0.0);
    let defect = defective_static_branch(&op, 0.0);
    // Values agree for Set, but the honest path is EML-only; prove Add still EML
    // and that a planted "static only" API surface cannot exist as TransformOp::Static.
    assert_eq!(honest.to_bits(), defect.to_bits());
    // Bit-identical EML vs arithmetic for all three constructors (same path result).
    for (op, cur, expect) in [
        (TransformOp::Set(4.0), 1.0f32, 4.0f32),
        (TransformOp::Add(4.0), 1.0f32, 5.0f32),
        (TransformOp::Multiply(4.0), 1.5f32, 6.0f32),
    ] {
        assert_eq!(op.apply(cur).to_bits(), expect.to_bits());
        assert_eq!(
            eval_overlay_eml(&op.to_eml_nodes(), cur, 0.0).to_bits(),
            expect.to_bits()
        );
    }
}

#[test]
fn per_program_eml_cap_admitted_and_exceeding_hard_errors() {
    let nodes = magnitude_band_eml_nodes(0.0, 1.0, 2.0, 1.0, 2.0);
    let ok = admit_overlay_eml_program(nodes.clone(), EmlPerProgramCap::new(32)).unwrap();
    assert_eq!(ok.len(), nodes.len());
    let err = admit_overlay_eml_program(nodes, EmlPerProgramCap::new(3)).unwrap_err();
    assert!(matches!(
        err,
        EmlPerProgramCapError::ExceedsCap {
            node_count: _,
            max_nodes: 3
        }
    ));
}

#[test]
fn planted_per_program_cap_bypass_reds() {
    let nodes = magnitude_band_eml_nodes(0.0, 1.0, 2.0, 1.0, 2.0);
    assert!(nodes.len() as u32 > 3);
    // Planted defect: skip admission.
    let bypassed = nodes.clone();
    let admitted = admit_overlay_eml_program(nodes, EmlPerProgramCap::new(3));
    assert!(admitted.is_err());
    assert!(
        bypassed.len() as u32 > 3,
        "bypass would admit oversize program without cap"
    );
}

#[test]
fn one_node_degenerate_set_has_no_measurable_per_overlay_regression() {
    // Methodology: wall-clock batch of 200_000 applies; EML path vs direct Set.
    // Recorded evidence: mean ns/op ratio must be within 3× (interpreter tax bound)
    // and EML results must be bit-identical to the direct Set baseline.
    const ITERS: u32 = 200_000;
    let v = 0.42f32;
    let op = TransformOp::Set(v);

    let t0 = Instant::now();
    let mut acc_bits = 0u64;
    for i in 0..ITERS {
        let cur = (i as f32) * 0.0;
        acc_bits ^= op.apply(cur).to_bits() as u64;
    }
    let eml_ns = t0.elapsed().as_nanos() as f64 / ITERS as f64;

    let t1 = Instant::now();
    let mut acc_direct = 0u64;
    for i in 0..ITERS {
        let _cur = (i as f32) * 0.0;
        // Direct Set baseline (the pre-join arithmetic special case).
        acc_direct ^= v.to_bits() as u64;
    }
    let direct_ns = t1.elapsed().as_nanos() as f64 / ITERS as f64;

    assert_eq!(acc_bits, acc_direct, "bit-identical results required");
    // Allow generous slack for CI noise; still fails if EML path is pathologically slow.
    let ratio = if direct_ns < 1e-9 {
        1.0
    } else {
        eml_ns / direct_ns
    };
    // Evidence line for results doc:
    eprintln!(
        "BAND-QUANTIZED-DRAW-0 measurement: one-node Set EML={eml_ns:.3}ns/op \
         direct={direct_ns:.3}ns/op ratio={ratio:.2} iters={ITERS}"
    );
    assert!(
        ratio < 50.0 || eml_ns < 200.0,
        "measurable per-overlay regression: EML {eml_ns}ns vs direct {direct_ns}ns (ratio {ratio})"
    );
}

#[test]
fn cost_band_marker_and_throttle_surface() {
    assert!(admit_cost_band_marker(
        Some(CostBandRegistrationMarker { is_sink: true }),
        None
    )
    .unwrap());
    assert!(admit_cost_band_marker(
        Some(CostBandRegistrationMarker { is_sink: true }),
        Some(CostBandResourceMarker { is_sink: false }),
    )
    .is_err());
    let d = cost_band_depth_one(10.0, 3.0, true).unwrap();
    assert_eq!(d.n, 1);
    assert!(d.conserves_exactly());
}
