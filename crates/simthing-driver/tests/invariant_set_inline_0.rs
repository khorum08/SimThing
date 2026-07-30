//! TP-PURGE-0 Stage B — mechanism-scoped Invariant Set proofs over inline-constructed input.
//!
//! Corpus/fixture/generator coupling forbidden. These referees bite only their named
//! substrate mechanisms (threshold-upload packing; allocator residual bound). They must
//! not be cited as survivors for mobility, EML, Studio observation, replay, map-gen,
//! or typeface rows — see Remand 2 `5135691949` and
//! `docs/tests/tp_purge_0_stage_b_replacement_map.tsv`.

use bytemuck::cast_slice;
use simthing_driver::{
    check_allocator_step, AllocatorConservationViolation, AllocatorStepObservation,
};
use simthing_gpu::{
    PackedThresholdUpload, ThresholdRegistration, DIR_UPWARD, THRESH_BUF_VALUES,
};

fn packed_upload_bytes(upload: &PackedThresholdUpload) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(cast_slice(upload.ops()));
    out.extend_from_slice(cast_slice(upload.threshold_event_kinds()));
    out
}

#[test]
fn determinism_packed_threshold_upload_byte_identical_twice() {
    let regs = [ThresholdRegistration {
        slot: 0,
        col: 0,
        threshold: 0.5,
        direction: DIR_UPWARD,
        event_kind: 7,
        buffer: THRESH_BUF_VALUES,
    }];
    let a = PackedThresholdUpload::from_registrations(&regs).expect("pack a");
    let b = PackedThresholdUpload::from_registrations(&regs).expect("pack b");
    assert_eq!(
        packed_upload_bytes(&a),
        packed_upload_bytes(&b),
        "identical inline threshold registrations must pack to identical POD bytes"
    );
}

#[test]
fn boundedness_allocator_residual_beyond_eps_bound_fails() {
    // Planted defect: disburse residual beyond O(eps*n) must FAIL even if Balance claims it.
    let obs = AllocatorStepObservation {
        budget: 10.0,
        disbursed: vec![2.0, 7.0], // residual 1.0 >> O(eps*n)
        balance_residual: Some(1.0),
    };
    let err = check_allocator_step(&obs).expect_err("must bite on O(eps*n) breach");
    match err {
        AllocatorConservationViolation::ResidualExceedsBound {
            abs_residual,
            bound,
            ..
        } => {
            assert!(abs_residual > bound);
            assert!(abs_residual > 0.5);
        }
        other => panic!("expected ResidualExceedsBound, got {other:?}"),
    }
}

#[test]
fn pack_cardinality_distinguishes_registration_count() {
    // Mechanism: packed threshold upload cardinality tracks registration count
    // (CPU-side pack precondition the GPU path consumes). Not a CPU/GPU parity claim —
    // live parity is s6_threshold_events_match_cpu_golden.
    let one = [ThresholdRegistration {
        slot: 0,
        col: 0,
        threshold: 0.25,
        direction: DIR_UPWARD,
        event_kind: 1,
        buffer: THRESH_BUF_VALUES,
    }];
    let two = [
        one[0],
        ThresholdRegistration {
            slot: 1,
            col: 0,
            threshold: 0.75,
            direction: DIR_UPWARD,
            event_kind: 2,
            buffer: THRESH_BUF_VALUES,
        },
    ];
    let p1 = PackedThresholdUpload::from_registrations(&one).expect("pack1");
    let p2 = PackedThresholdUpload::from_registrations(&two).expect("pack2");
    assert_ne!(
        packed_upload_bytes(&p1),
        packed_upload_bytes(&p2),
        "pack POD bytes must distinguish registration cardinality"
    );
    assert_eq!(p1.ops().len(), 1);
    assert_eq!(p2.ops().len(), 2);
}
