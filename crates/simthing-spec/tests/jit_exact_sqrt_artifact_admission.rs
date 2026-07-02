//! SQRT-PROMOTE-0 — Artifact-backed Candidate F exact sqrt descriptor/admission tests.

use simthing_spec::{
    exact_sqrt_f_artifact_descriptor, fnv1a64_hex, is_exact_mag2_fixed_descriptor,
    is_exact_mag_f_from_mag2_descriptor, is_exact_sqrt_f_descriptor,
    is_mag_f_dxdy_probe_descriptor, landed_jit_kernel_descriptors,
    mag_f_from_dxdy_probe_kernel_descriptor, mag_f_from_exact_mag2_kernel_descriptor,
    preview_kernel_graph_identity, preview_kernel_registry_manifest,
    preview_production_candidate_registry_entry, sqrt_f_exact_kernel_descriptor,
    validate_exact_kernel_inputs, validate_exact_sqrt_artifact_binding,
    validate_kernel_descriptor_admission, validate_kernel_graph_admission,
    ExactPreSqrtInputContract, ExactSqrtArtifactDescriptor, ExactSqrtAuthorityClass,
    KernelDescriptorSpec, KernelGraphEdgeSpec, KernelGraphRequestSpec, KernelGraphSpec, KernelLane,
    KernelOutputSpec, KernelRegistryLane, MappingExecutionProfile, NativeMathClass,
    OutputAuthority, SpecError, MAG2_FIXED_DESCRIPTOR_ID, MAG_F_FROM_DXDY_PROBE_DESCRIPTOR_ID,
    MAG_F_FROM_MAG2_DESCRIPTOR_ID, SQRT_F_ARTIFACT_HASH, SQRT_F_ARTIFACT_PATH,
    SQRT_F_DESCRIPTOR_ID, SQRT_F_DOMAIN, SQRT_F_ENTRYPOINT, SQRT_F_IO_CONTRACT,
    SQRT_F_PROOF_REPORT,
};

fn sqrt0() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == "m_jit_sqrt_0_candidate")
        .expect("sqrt0 descriptor")
}

fn sqrt_f() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == SQRT_F_DESCRIPTOR_ID)
        .expect("sqrt F exact descriptor")
}

fn grad0() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == "m_jit_grad_0_observer")
        .expect("grad0 descriptor")
}

fn mag_from_mag2() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == MAG_F_FROM_MAG2_DESCRIPTOR_ID)
        .expect("mag from mag2 descriptor")
}

fn mag_dxdy_probe() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == MAG_F_FROM_DXDY_PROBE_DESCRIPTOR_ID)
        .expect("mag dx/dy probe descriptor")
}

fn grad1_style_scorer() -> KernelDescriptorSpec {
    KernelDescriptorSpec {
        id: "m_jit_grad_1_scorer".into(),
        lane: KernelLane::TestOnly,
        reads: vec!["descent_x".into(), "sqrt_out".into()],
        writes: vec![KernelOutputSpec {
            name: "score".into(),
            authority: OutputAuthority::ExactAuthoritative,
        }],
        native_math: NativeMathClass::None,
        semantic_free: true,
        default_off: true,
        production_wiring: false,
        exact_sqrt_artifact: None,
        pre_sqrt_contract: None,
        mag2_source_contract: None,
        score_authority_contract: None,
    }
}

fn exact_edge(from: &str, out: &str, to: &str, input: &str) -> KernelGraphEdgeSpec {
    KernelGraphEdgeSpec {
        from_kernel: from.into(),
        from_output: out.into(),
        to_kernel: to.into(),
        to_input: input.into(),
        required_authority: OutputAuthority::ExactAuthoritative,
    }
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
    let err =
        validate_exact_kernel_inputs(producer, inputs).expect_err("expected exact-input failure");
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
fn sqrt_promote0_f_descriptor_is_exact_deterministic() {
    let f = sqrt_f();
    assert!(is_exact_sqrt_f_descriptor(&f));
    validate_kernel_descriptor_admission(&f).expect("F descriptor should admit");

    let binding = f.exact_sqrt_artifact.as_ref().expect("artifact binding");
    assert_eq!(
        binding.authority_class,
        ExactSqrtAuthorityClass::ExactDeterministic
    );
    assert_eq!(binding.artifact_path, SQRT_F_ARTIFACT_PATH);
    assert_eq!(binding.artifact_hash_fnv1a64, SQRT_F_ARTIFACT_HASH);
    assert_eq!(binding.entrypoint, SQRT_F_ENTRYPOINT);
    assert_eq!(binding.io_contract, SQRT_F_IO_CONTRACT);
    assert_eq!(binding.proof_report, SQRT_F_PROOF_REPORT);
    assert_eq!(binding.domain, SQRT_F_DOMAIN);

    let sqrt_out = f
        .writes
        .iter()
        .find(|out| out.name == "sqrt_out")
        .expect("sqrt_out");
    assert_eq!(sqrt_out.authority, OutputAuthority::ExactAuthoritative);
    assert_eq!(f.native_math, NativeMathClass::None);

    println!(
        "sqrt_promote0_f: id={} hash={} entrypoint={}",
        f.id, SQRT_F_ARTIFACT_HASH, SQRT_F_ENTRYPOINT
    );
}
