//! SQRT-PROMOTE-0 — Artifact-backed Candidate F exact sqrt descriptor/admission tests.

use simthing_spec::{
    exact_sqrt_f_artifact_descriptor, fnv1a64_hex, is_exact_mag2_fixed_descriptor,
    is_exact_mag_f_from_mag2_descriptor,
    is_exact_sqrt_f_descriptor, is_mag_f_dxdy_probe_descriptor,
    landed_jit_kernel_descriptors, mag_f_from_dxdy_probe_kernel_descriptor,
    mag_f_from_exact_mag2_kernel_descriptor, preview_kernel_graph_identity,
    preview_kernel_registry_manifest, preview_production_candidate_registry_entry,
    sqrt_f_exact_kernel_descriptor, validate_exact_kernel_inputs,
    validate_exact_sqrt_artifact_binding,
    validate_kernel_descriptor_admission, validate_kernel_graph_admission,
    ExactPreSqrtInputContract, ExactSqrtArtifactDescriptor, ExactSqrtAuthorityClass,
    KernelDescriptorSpec, KernelGraphEdgeSpec, KernelGraphRequestSpec, KernelGraphSpec,
    KernelLane, KernelOutputSpec, KernelRegistryLane, MappingExecutionProfile, NativeMathClass,
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
fn sqrt_promote0_f_descriptor_is_exact_deterministic() {
    let f = sqrt_f();
    assert!(is_exact_sqrt_f_descriptor(&f));
    validate_kernel_descriptor_admission(&f).expect("F descriptor should admit");

    let binding = f.exact_sqrt_artifact.as_ref().expect("artifact binding");
    assert_eq!(binding.authority_class, ExactSqrtAuthorityClass::ExactDeterministic);
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

#[test]
fn sqrt_promote0_native_sqrt_remains_approximate() {
    let sqrt0 = sqrt0();
    assert_eq!(sqrt0.native_math, NativeMathClass::ApproximateJitOnly);
    assert!(sqrt0.exact_sqrt_artifact.is_none());
    assert!(
        sqrt0
            .writes
            .iter()
            .all(|out| out.authority == OutputAuthority::ApproximateDiagnostic)
    );

    for desc in landed_jit_kernel_descriptors() {
        if desc.native_math == NativeMathClass::ApproximateJitOnly {
            assert!(
                desc.writes
                    .iter()
                    .all(|out| out.authority != OutputAuthority::ExactAuthoritative),
                "ApproximateJitOnly kernel {} must not claim exact outputs",
                desc.id
            );
        }
    }

    let baseline = include_str!("../../simthing-gpu/src/shaders/accumulator_op.wgsl");
    assert!(!baseline.contains("sqrt("));
}

#[test]
fn sqrt_promote0_validate_exact_inputs_accepts_f_backed_sqrt_only() {
    validate_exact_kernel_inputs(&sqrt_f(), &["sqrt_out"]).expect("F sqrt_out exact");

    let graph = KernelGraphSpec {
        nodes: vec![sqrt_f(), grad1_style_scorer()],
        edges: vec![exact_edge(
            SQRT_F_DESCRIPTOR_ID,
            "sqrt_out",
            "m_jit_grad_1_scorer",
            "sqrt_out",
        )],
    };
    validate_kernel_graph_admission(&graph).expect("F-backed sqrt graph should admit");

    assert_exact_input_err(&sqrt0(), &["sqrt_out"], "approximate/diagnostic");
    assert_exact_input_err(&grad0(), &["mag2"], "approximate/diagnostic");

    let mut d_like = sqrt_f();
    d_like.id = "m_jit_sqrt_d_like".into();
    d_like.exact_sqrt_artifact = None;
    assert_exact_input_err(&d_like, &["sqrt_out"], "artifact-backed Candidate F");

    let mut recomposed = sqrt_f();
    recomposed.exact_sqrt_artifact = Some(ExactSqrtArtifactDescriptor {
        artifact_path: "arbitrary/recomposed/sqrt_cr_f.wgsl".into(),
        artifact_hash_fnv1a64: SQRT_F_ARTIFACT_HASH.into(),
        entrypoint: SQRT_F_ENTRYPOINT.into(),
        io_contract: SQRT_F_IO_CONTRACT.into(),
        proof_report: SQRT_F_PROOF_REPORT.into(),
        domain: SQRT_F_DOMAIN.into(),
        authority_class: ExactSqrtAuthorityClass::ExactDeterministic,
    });
    assert_admission_err(&recomposed, "artifact path mismatch");
}

#[test]
fn sqrt_promote0_f_artifact_hash_guard() {
    let binding = exact_sqrt_f_artifact_descriptor();
    validate_exact_sqrt_artifact_binding(SQRT_F_DESCRIPTOR_ID, &binding).expect("pinned binding");

    let artifact = include_str!("../../simthing-driver/tests/wgsl/sqrt_cr_f_candidate.wgsl");
    assert_eq!(fnv1a64_hex(artifact), SQRT_F_ARTIFACT_HASH);

    let mut wrong_hash = sqrt_f();
    wrong_hash.exact_sqrt_artifact = Some(ExactSqrtArtifactDescriptor {
        artifact_hash_fnv1a64: "0000000000000000".into(),
        ..exact_sqrt_f_artifact_descriptor()
    });
    assert_admission_err(&wrong_hash, "hash mismatch");

    let mut missing_hash = sqrt_f();
    missing_hash.exact_sqrt_artifact = Some(ExactSqrtArtifactDescriptor {
        artifact_hash_fnv1a64: String::new(),
        ..exact_sqrt_f_artifact_descriptor()
    });
    assert_admission_err(&missing_hash, "hash must not be empty");

    let mut wrong_entry = sqrt_f();
    wrong_entry.exact_sqrt_artifact = Some(ExactSqrtArtifactDescriptor {
        entrypoint: "sqrt_cr_f".into(),
        ..exact_sqrt_f_artifact_descriptor()
    });
    assert_admission_err(&wrong_entry, "entrypoint mismatch");

    let mut wrong_path = sqrt_f();
    wrong_path.exact_sqrt_artifact = Some(ExactSqrtArtifactDescriptor {
        artifact_path: "wrong/path.wgsl".into(),
        ..exact_sqrt_f_artifact_descriptor()
    });
    assert_admission_err(&wrong_path, "path mismatch");

    let mut no_proof = sqrt_f();
    no_proof.exact_sqrt_artifact = Some(ExactSqrtArtifactDescriptor {
        proof_report: String::new(),
        ..exact_sqrt_f_artifact_descriptor()
    });
    assert_admission_err(&no_proof, "proof_report must not be empty");

    let mut f_like_no_identity = sqrt_f_exact_kernel_descriptor();
    f_like_no_identity.exact_sqrt_artifact = None;
    assert_admission_err(&f_like_no_identity, "requires artifact-backed Candidate F binding");
}

#[test]
fn sqrt_promote0_production_candidate_admission_accepts_f_exact_path() {
    let graph = KernelGraphSpec {
        nodes: vec![sqrt_f(), grad1_style_scorer()],
        edges: vec![exact_edge(
            SQRT_F_DESCRIPTOR_ID,
            "sqrt_out",
            "m_jit_grad_1_scorer",
            "sqrt_out",
        )],
    };
    let identity = preview_kernel_graph_identity(&graph).expect("F graph identity");
    assert!(identity.canonical_text.contains(SQRT_F_ARTIFACT_HASH));

    let manifest = preview_kernel_registry_manifest(&[KernelGraphRequestSpec {
        request_id: "req_f_exact".into(),
        graph,
    }])
    .expect("F exact registry manifest");
    let entry = manifest.entries.first().expect("one entry");
    assert_eq!(entry.lane, KernelRegistryLane::TestOnlyPreview);
    assert!(entry.default_off);
    assert!(!entry.production_wiring);

    let candidate =
        preview_production_candidate_registry_entry(entry).expect("F exact production candidate");
    assert_eq!(candidate.lane, KernelRegistryLane::ProductionCandidatePreview);
    assert!(candidate.default_off);
    assert!(!candidate.production_wiring);
    println!(
        "sqrt_promote0_prod_candidate: key={} lane=ProductionCandidatePreview",
        candidate.stable_key
    );
}

#[test]
fn sqrt_promote0_no_runtime_default_wiring() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );

    for desc in landed_jit_kernel_descriptors() {
        assert!(desc.default_off, "{} must stay default-off", desc.id);
        assert!(
            !desc.production_wiring,
            "{} must stay production_wiring=false",
            desc.id
        );
    }

    let source = include_str!("../src/compile/jit_exact_sqrt_artifact_admission.rs");
    for forbidden in [
        "dispatch_workgroups",
        "create_shader_module",
        "GpuContext",
        "simthing_gpu",
        "simthing_driver",
        "simthing-sim",
        "ResourceEconomySpec",
        "SimSession",
        "KernelCache",
        "cache.insert",
    ] {
        assert!(
            !source.contains(forbidden),
            "exact sqrt admission module must not reference `{forbidden}`"
        );
    }
}

#[test]
fn sqrt_mag0_r1_mag_from_exact_mag2_admits_exact_mag() {
    let from_mag2 = mag_from_mag2();
    assert!(is_exact_mag_f_from_mag2_descriptor(&from_mag2));
    validate_kernel_descriptor_admission(&from_mag2).expect("from_mag2 admits");
    validate_exact_kernel_inputs(&from_mag2, &["mag"]).expect("exact mag from mag2");
    assert_eq!(
        from_mag2.pre_sqrt_contract,
        Some(ExactPreSqrtInputContract::ExactMag2Bits)
    );
}

#[test]
fn sqrt_mag0_r1_raw_dxdy_probe_rejects_exact_mag_authority() {
    let probe = mag_dxdy_probe();
    assert!(is_mag_f_dxdy_probe_descriptor(&probe));
    validate_kernel_descriptor_admission(&probe).expect("probe admits");
    assert_eq!(
        probe.pre_sqrt_contract,
        Some(ExactPreSqrtInputContract::RawDxDyProbe)
    );
    assert_exact_input_err(&probe, &["mag"], "ExactMag2Bits pre-sqrt contract");

    let mut bad = mag_f_from_dxdy_probe_kernel_descriptor();
    bad.writes = vec![KernelOutputSpec {
        name: "mag".into(),
        authority: OutputAuthority::ExactAuthoritative,
    }];
    assert_admission_err(
        &bad,
        "raw dx/dy multiply-add probe cannot claim exact-authoritative mag output",
    );
}

#[test]
fn sqrt_mag0_r1_exact_mag_requires_pre_sqrt_contract() {
    let mut no_contract = mag_f_from_exact_mag2_kernel_descriptor();
    no_contract.pre_sqrt_contract = None;
    assert_admission_err(
        &no_contract,
        "exact-authoritative mag requires an ExactPreSqrtInputContract",
    );
}

#[test]
fn sqrt_mag2_0_fixed_mag2_descriptor_admits_exact_mag2_bits() {
    let fixed = landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == MAG2_FIXED_DESCRIPTOR_ID)
        .expect("mag2 fixed");
    assert!(is_exact_mag2_fixed_descriptor(&fixed));
    validate_kernel_descriptor_admission(&fixed).expect("fixed mag2 admits");
    validate_exact_kernel_inputs(&fixed, &["mag2_bits"]).expect("mag2_bits exact");

    let grad0 = grad0();
    assert_exact_input_err(&grad0, &["mag2"], "approximate/diagnostic");
}
