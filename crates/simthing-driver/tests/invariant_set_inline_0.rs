//! TP-PURGE-0 Stage B — low-single-digit Invariant Set proofs over inline-constructed input.
//!
//! Corpus/fixture/generator coupling forbidden. Conservation bites also live in
//! `rf_conservation_oracle` unit tests (restored). Primary CPU/GPU parity remains
//! `s6_threshold_events_match_cpu_golden`.

use simthing_driver::{
    check_allocator_step, AllocatorConservationViolation, AllocatorStepObservation,
};
use simthing_gpu::{
    PackedThresholdUpload, ThresholdRegistration, DIR_UPWARD, THRESH_BUF_VALUES,
};

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
        format!("{a:?}"),
        format!("{b:?}"),
        "identical inline threshold registrations must pack deterministically"
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
fn cpu_gpu_parity_inline_pack_matches_registration_count() {
    // Mechanism sibling to s6: inline pack shape agrees with registration count (CPU-side
    // precondition the GPU path consumes). Planted defect: dropping a registration must
    // change pack cardinality.
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
        format!("{p1:?}"),
        format!("{p2:?}"),
        "pack must distinguish registration cardinality (parity precondition)"
    );
}
