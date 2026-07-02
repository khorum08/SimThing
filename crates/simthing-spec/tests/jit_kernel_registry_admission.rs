//! Phase M-JIT-REG-1 — Production-candidate registry admission preview.

use simthing_spec::{
    preview_kernel_graph_identity, preview_kernel_registry_manifest,
    preview_production_candidate_registry_entry, KernelDescriptorSpec, KernelGraphEdgeSpec,
    KernelGraphRequestSpec, KernelGraphSpec, KernelLane, KernelOutputSpec,
    KernelRegistryEntryPreview, KernelRegistryLane, NativeMathClass, OutputAuthority, SpecError,
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
        exact_sqrt_artifact: None,
        pre_sqrt_contract: None,
        mag2_source_contract: None,
        score_authority_contract: None,
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
