//! Phase M-JIT-DESC-1 — Spec-layer JIT kernel descriptor admission preview.

use simthing_spec::{
    landed_jit_kernel_descriptors, validate_exact_kernel_inputs,
    validate_kernel_descriptor_admission, KernelDescriptorSpec, KernelLane, KernelOutputSpec,
    NativeMathClass, OutputAuthority, SpecError,
};

fn grad0() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == "m_jit_grad_0_observer")
        .expect("grad0 descriptor")
}

fn grad1() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == "m_jit_grad_1_observer_score")
        .expect("grad1 descriptor")
}

fn sqrt0() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == "m_jit_sqrt_0_candidate")
        .expect("sqrt0 descriptor")
}

fn assert_admission_err(spec: &KernelDescriptorSpec, reason_substr: &str) {
    let err = validate_kernel_descriptor_admission(spec).expect_err("expected admission failure");
    match err {
        SpecError::JitKernelDescriptorAdmission { reason, .. } => {
            assert!(
                reason.contains(reason_substr),
                "expected `{reason_substr}` in `{reason}`"
            );
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

fn assert_exact_input_err(producer: &KernelDescriptorSpec, inputs: &[&str], reason_substr: &str) {
    let err = validate_exact_kernel_inputs(producer, inputs).expect_err("expected exact-input failure");
    match err {
        SpecError::JitKernelDescriptorAdmission { reason, .. } => {
            assert!(
                reason.contains(reason_substr),
                "expected `{reason_substr}` in `{reason}`"
            );
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn jit_desc1_landed_test_only_descriptors_admit() {
    for desc in landed_jit_kernel_descriptors() {
        validate_kernel_descriptor_admission(&desc)
            .unwrap_or_else(|err| panic!("{} should admit: {err}", desc.id));
        assert_eq!(desc.lane, KernelLane::TestOnly);
    }
    println!(
        "desc1_admit: count={} all=TestOnly",
        landed_jit_kernel_descriptors().len()
    );
}

#[test]
fn jit_desc1_rejects_approximate_output_as_exact_input() {
    assert_exact_input_err(&grad0(), &["mag2"], "approximate/diagnostic");
    assert_exact_input_err(&sqrt0(), &["sqrt_out"], "approximate/diagnostic");

    validate_exact_kernel_inputs(&grad0(), &["dx", "dy"]).expect("dx/dy exact");
    validate_exact_kernel_inputs(&grad0(), &["descent_x", "descent_y"]).expect("descent exact");
    validate_exact_kernel_inputs(&grad1(), &["score"]).expect("score exact");

    assert!(
        grad1()
            .writes
            .iter()
            .all(|out| out.name != "mag2"),
        "GRAD-1 must not write mag2"
    );
}

#[test]
fn jit_desc1_rejects_production_wiring() {
    let mut desc = grad0();
    desc.production_wiring = true;
    assert_admission_err(&desc, "production_wiring");
}

#[test]
fn jit_desc1_rejects_default_on() {
    let mut desc = grad0();
    desc.default_off = false;
    assert_admission_err(&desc, "default_off");
}

#[test]
fn jit_desc1_rejects_semantic_descriptor_names() {
    let mut desc = grad0();
    desc.id = "m_jit_faction_observer".into();
    assert_admission_err(&desc, "forbidden semantic term");

    let mut desc = grad0();
    desc.reads.push("threat_level".into());
    assert_admission_err(&desc, "forbidden semantic term");

    let mut desc = grad0();
    desc.writes.push(KernelOutputSpec {
        name: "personality_score".into(),
        authority: OutputAuthority::ExactAuthoritative,
    });
    assert_admission_err(&desc, "forbidden semantic term");
}

#[test]
fn jit_desc1_rejects_approximate_native_math_exact_output() {
    let desc = KernelDescriptorSpec {
        id: "m_jit_sqrt_bad_exact".into(),
        lane: KernelLane::TestOnly,
        reads: vec!["values".into()],
        writes: vec![KernelOutputSpec {
            name: "sqrt_out".into(),
            authority: OutputAuthority::ExactAuthoritative,
        }],
        native_math: NativeMathClass::ApproximateJitOnly,
        semantic_free: true,
        default_off: true,
        production_wiring: false,
    };
    assert_admission_err(&desc, "approximate native math");
}

#[test]
fn jit_desc1_rejects_production_candidate_lane() {
    let mut desc = grad0();
    desc.lane = KernelLane::ProductionCandidate;
    assert_admission_err(&desc, "ProductionCandidate");
}

#[test]
fn jit_desc1_rejects_duplicate_outputs() {
    let mut desc = grad0();
    desc.writes.push(KernelOutputSpec {
        name: "dx".into(),
        authority: OutputAuthority::ExactAuthoritative,
    });
    assert_admission_err(&desc, "duplicate output");
}
