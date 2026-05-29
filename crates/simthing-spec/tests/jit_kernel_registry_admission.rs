//! Phase M-JIT-REG-1 — Production-candidate registry admission preview.

use simthing_spec::{
    preview_kernel_registry_manifest, preview_production_candidate_registry_entry,
    preview_kernel_graph_identity, KernelDescriptorSpec, KernelGraphEdgeSpec,
    KernelGraphRequestSpec, KernelGraphSpec, KernelLane, KernelOutputSpec,
    KernelRegistryEntryPreview, KernelRegistryLane, NativeMathClass, OutputAuthority,
    SpecError,
};

fn grad0_exact_only() -> KernelDescriptorSpec {
    KernelDescriptorSpec {
        id: "m_jit_grad_0_observer".into(),
        lane: KernelLane::TestOnly,
        reads: vec!["fields".into(), "observers".into()],
        writes: vec![
            KernelOutputSpec {
                name: "dx".into(),
                authority: OutputAuthority::ExactAuthoritative,
            },
            KernelOutputSpec {
                name: "dy".into(),
                authority: OutputAuthority::ExactAuthoritative,
            },
            KernelOutputSpec {
                name: "descent_x".into(),
                authority: OutputAuthority::ExactAuthoritative,
            },
            KernelOutputSpec {
                name: "descent_y".into(),
                authority: OutputAuthority::ExactAuthoritative,
            },
        ],
        native_math: NativeMathClass::None,
        semantic_free: true,
        default_off: true,
        production_wiring: false,
    }
}

fn grad1_style_scorer() -> KernelDescriptorSpec {
    KernelDescriptorSpec {
        id: "m_jit_grad_1_scorer".into(),
        lane: KernelLane::TestOnly,
        reads: vec!["descent_x".into(), "descent_y".into()],
        writes: vec![KernelOutputSpec {
            name: "score".into(),
            authority: OutputAuthority::ExactAuthoritative,
        }],
        native_math: NativeMathClass::None,
        semantic_free: true,
        default_off: true,
        production_wiring: false,
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

fn exact_grad0_to_scorer_graph() -> KernelGraphSpec {
    KernelGraphSpec {
        nodes: vec![grad0_exact_only(), grad1_style_scorer()],
        edges: vec![
            exact_edge(
                "m_jit_grad_0_observer",
                "descent_x",
                "m_jit_grad_1_scorer",
                "descent_x",
            ),
            exact_edge(
                "m_jit_grad_0_observer",
                "descent_y",
                "m_jit_grad_1_scorer",
                "descent_y",
            ),
        ],
    }
}

fn exact_grad0_to_scorer_entry() -> KernelRegistryEntryPreview {
    let requests = vec![KernelGraphRequestSpec {
        request_id: "req_exact".into(),
        graph: exact_grad0_to_scorer_graph(),
    }];
    let manifest = preview_kernel_registry_manifest(&requests).expect("registry manifest");
    assert_eq!(manifest.entries.len(), 1);
    manifest.entries[0].clone()
}

fn assert_candidate_err(entry: &KernelRegistryEntryPreview, reason_substr: &str) {
    let err = preview_production_candidate_registry_entry(entry)
        .expect_err("expected production-candidate admission failure");
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
fn jit_reg1_exact_grad1_style_entry_admits_as_production_candidate_preview() {
    let entry = exact_grad0_to_scorer_entry();
    assert_eq!(entry.lane, KernelRegistryLane::TestOnlyPreview);
    assert!(entry.stable_key.starts_with("jit-graph-v1:"));
    assert!(!entry.canonical_text.contains("mag2"));
    assert!(!entry.canonical_text.contains("ApproximateDiagnostic"));

    let candidate =
        preview_production_candidate_registry_entry(&entry).expect("candidate admission");
    assert_eq!(candidate.lane, KernelRegistryLane::ProductionCandidatePreview);
    assert!(candidate.default_off);
    assert!(!candidate.production_wiring);
    assert_eq!(candidate.stable_key, entry.stable_key);
    assert_eq!(candidate.canonical_text, entry.canonical_text);
    println!(
        "reg1_admit: key={} lane=ProductionCandidatePreview",
        candidate.stable_key
    );
}

#[test]
fn jit_reg1_rejects_mag2_candidate() {
    let mag2_graph = KernelGraphSpec {
        nodes: vec![
            simthing_spec::landed_jit_kernel_descriptors()
                .into_iter()
                .find(|desc| desc.id == "m_jit_grad_0_observer")
                .expect("grad0"),
            grad1_style_scorer(),
        ],
        edges: vec![exact_edge(
            "m_jit_grad_0_observer",
            "mag2",
            "m_jit_grad_1_scorer",
            "descent_x",
        )],
    };
    let err = preview_kernel_registry_manifest(&[KernelGraphRequestSpec {
        request_id: "mag2_bad".into(),
        graph: mag2_graph,
    }])
    .expect_err("mag2 graph should reject upstream");
    match err {
        SpecError::JitKernelDescriptorAdmission { reason, .. } => {
            assert!(reason.contains("approximate/diagnostic"));
        }
        other => panic!("unexpected error: {other:?}"),
    }

    let mut entry = exact_grad0_to_scorer_entry();
    entry.canonical_text.push_str("\n  write=mag2 authority=ApproximateDiagnostic");
    assert_candidate_err(&entry, "mag2");
}

#[test]
fn jit_reg1_rejects_sqrt_candidate() {
    let sqrt = simthing_spec::landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == "m_jit_sqrt_0_candidate")
        .expect("sqrt0");
    let identity = preview_kernel_graph_identity(&KernelGraphSpec {
        nodes: vec![sqrt],
        edges: vec![],
    })
    .expect("sqrt identity");
    let entry = KernelRegistryEntryPreview {
        stable_key: identity.stable_key,
        canonical_text: identity.canonical_text,
        request_ids: vec!["sqrt_req".into()],
        lane: KernelRegistryLane::TestOnlyPreview,
        default_off: true,
        production_wiring: false,
    };
    assert_candidate_err(&entry, "m_jit_sqrt_0_candidate");
}

#[test]
fn jit_reg1_rejects_semantic_canonical_text() {
    let mut entry = exact_grad0_to_scorer_entry();
    entry.canonical_text.push_str("\nnode id=m_jit_faction_observer");
    assert_candidate_err(&entry, "forbidden semantic term");
}

#[test]
fn jit_reg1_rejects_default_on_or_production_wired() {
    let mut default_on = exact_grad0_to_scorer_entry();
    default_on.default_off = false;
    assert_candidate_err(&default_on, "default_off");

    let mut prod_wired = exact_grad0_to_scorer_entry();
    prod_wired.production_wiring = true;
    assert_candidate_err(&prod_wired, "production_wiring");
}

#[test]
fn jit_reg1_rejects_bad_stable_key() {
    let mut bad_prefix = exact_grad0_to_scorer_entry();
    bad_prefix.stable_key = "bad-key:001".into();
    assert_candidate_err(&bad_prefix, "jit-graph-v1");

    let mut empty_key = exact_grad0_to_scorer_entry();
    empty_key.stable_key.clear();
    assert_candidate_err(&empty_key, "jit-graph-v1");
}

#[test]
fn jit_reg1_rejects_unsorted_request_ids() {
    let mut unsorted = exact_grad0_to_scorer_entry();
    unsorted.request_ids = vec!["req_b".into(), "req_a".into()];
    assert_candidate_err(&unsorted, "sorted");
}

#[test]
fn jit_reg1_candidate_preview_has_no_scheduler_cache_or_dispatch() {
    let source = include_str!("../src/compile/jit_kernel_registry_preview.rs");
    for forbidden in [
        "dispatch_workgroups",
        "create_shader_module",
        "GpuContext",
        "EmlGpuProgramTable",
        "AccumulatorOpSession",
        "tick_with_eml",
        "simthing_gpu",
        "simthing_driver",
    ] {
        assert!(
            !source.contains(forbidden),
            "registry admission module must not reference `{forbidden}`"
        );
    }
    assert!(
        !source.contains("cache.insert") && !source.contains("KernelCache"),
        "registry admission module must not implement runtime cache"
    );
}
