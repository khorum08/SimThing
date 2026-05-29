//! Phase M-JIT-KEY-0 — Spec-layer deterministic kernel graph identity preview.

use simthing_spec::{
    landed_jit_kernel_descriptors, preview_kernel_graph_identity,
    validate_kernel_graph_admission, KernelDescriptorSpec, KernelGraphEdgeSpec, KernelGraphSpec,
    KernelLane, KernelOutputSpec, NativeMathClass, OutputAuthority, SpecError,
};

const FORBIDDEN_SEMANTIC_TERMS: &[&str] = &[
    "faction",
    "ownership",
    "owner",
    "AI",
    "threat",
    "scarcity",
    "opportunity",
    "labor",
    "price",
    "logistics",
    "routing",
    "need",
    "demand",
    "supply",
    "personality",
    "drone",
    "SEAD",
    "simthing-sim",
    "ResourceEconomySpec",
    "SimSession",
];

fn grad0() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == "m_jit_grad_0_observer")
        .expect("grad0 descriptor")
}

fn sqrt0() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == "m_jit_sqrt_0_candidate")
        .expect("sqrt0 descriptor")
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

fn valid_grad0_to_scorer_graph() -> KernelGraphSpec {
    KernelGraphSpec {
        nodes: vec![grad0(), grad1_style_scorer()],
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

fn assert_identity_err(graph: &KernelGraphSpec, reason_substr: &str) {
    let err = preview_kernel_graph_identity(graph).expect_err("expected identity failure");
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
fn jit_key0_identity_stable_across_node_and_edge_order() {
    let graph_a = valid_grad0_to_scorer_graph();
    let graph_b = KernelGraphSpec {
        nodes: vec![grad1_style_scorer(), grad0()],
        edges: vec![
            exact_edge(
                "m_jit_grad_0_observer",
                "descent_y",
                "m_jit_grad_1_scorer",
                "descent_y",
            ),
            exact_edge(
                "m_jit_grad_0_observer",
                "descent_x",
                "m_jit_grad_1_scorer",
                "descent_x",
            ),
        ],
    };

    let id_a = preview_kernel_graph_identity(&graph_a).expect("graph_a identity");
    let id_b = preview_kernel_graph_identity(&graph_b).expect("graph_b identity");

    assert_eq!(id_a.canonical_text, id_b.canonical_text);
    assert_eq!(id_a.stable_key, id_b.stable_key);
    println!(
        "key0_stable: canonical_len={} stable_key={}",
        id_a.canonical_text.len(),
        id_a.stable_key
    );
}

#[test]
fn jit_key0_identity_changes_when_output_authority_changes() {
    let base = preview_kernel_graph_identity(&valid_grad0_to_scorer_graph()).expect("base identity");

    let mut changed = valid_grad0_to_scorer_graph();
    for node in &mut changed.nodes {
        if node.id == "m_jit_grad_0_observer" {
            for out in &mut node.writes {
                if out.name == "descent_x" {
                    out.authority = OutputAuthority::ApproximateDiagnostic;
                }
            }
        }
    }
    assert!(
        validate_kernel_graph_admission(&changed).is_err(),
        "approximate descent_x should fail exact edge admission"
    );
    assert_identity_err(&changed, "approximate/diagnostic");

    let mut scorer_only = valid_grad0_to_scorer_graph();
    for node in &mut scorer_only.nodes {
        if node.id == "m_jit_grad_1_scorer" {
            node.writes[0].authority = OutputAuthority::ApproximateDiagnostic;
        }
    }
    let changed_id =
        preview_kernel_graph_identity(&scorer_only).expect("scorer authority change admits");
    assert_ne!(base.canonical_text, changed_id.canonical_text);
    assert_ne!(base.stable_key, changed_id.stable_key);
}

#[test]
fn jit_key0_identity_changes_when_native_math_changes() {
    let base = preview_kernel_graph_identity(&valid_grad0_to_scorer_graph()).expect("base identity");

    let mut sqrt_graph = KernelGraphSpec {
        nodes: vec![sqrt0(), grad1_style_scorer()],
        edges: vec![exact_edge(
            "m_jit_sqrt_0_candidate",
            "sqrt_out",
            "m_jit_grad_1_scorer",
            "descent_x",
        )],
    };
    assert_identity_err(&sqrt_graph, "approximate/diagnostic");

    for node in &mut sqrt_graph.nodes {
        if node.id == "m_jit_sqrt_0_candidate" {
            node.native_math = NativeMathClass::None;
        }
    }
    assert_identity_err(&sqrt_graph, "approximate/diagnostic");

    let mut native_math_graph = valid_grad0_to_scorer_graph();
    for node in &mut native_math_graph.nodes {
        if node.id == "m_jit_grad_0_observer" {
            node.native_math = NativeMathClass::ApproximateJitOnly;
        }
    }
    assert!(
        validate_kernel_graph_admission(&native_math_graph).is_err(),
        "ApproximateJitOnly with exact outputs should fail admission"
    );
    assert_identity_err(&native_math_graph, "approximate native math");

    let mut admitted_change = valid_grad0_to_scorer_graph();
    for node in &mut admitted_change.nodes {
        if node.id == "m_jit_grad_1_scorer" {
            node.reads.push("bias".into());
        }
    }
    let changed_id =
        preview_kernel_graph_identity(&admitted_change).expect("read list change admits");
    assert_ne!(base.canonical_text, changed_id.canonical_text);
    assert_ne!(base.stable_key, changed_id.stable_key);
}

#[test]
fn jit_key0_invalid_graph_identity_rejects() {
    let mag2_graph = KernelGraphSpec {
        nodes: vec![grad0(), grad1_style_scorer()],
        edges: vec![exact_edge(
            "m_jit_grad_0_observer",
            "mag2",
            "m_jit_grad_1_scorer",
            "descent_x",
        )],
    };
    assert_identity_err(&mag2_graph, "approximate/diagnostic");

    let node_a = KernelDescriptorSpec {
        id: "m_jit_cycle_a".into(),
        lane: KernelLane::TestOnly,
        reads: vec!["in_a".into()],
        writes: vec![KernelOutputSpec {
            name: "out_a".into(),
            authority: OutputAuthority::ExactAuthoritative,
        }],
        native_math: NativeMathClass::None,
        semantic_free: true,
        default_off: true,
        production_wiring: false,
    };
    let node_b = KernelDescriptorSpec {
        id: "m_jit_cycle_b".into(),
        lane: KernelLane::TestOnly,
        reads: vec!["in_b".into()],
        writes: vec![KernelOutputSpec {
            name: "out_b".into(),
            authority: OutputAuthority::ExactAuthoritative,
        }],
        native_math: NativeMathClass::None,
        semantic_free: true,
        default_off: true,
        production_wiring: false,
    };
    let cycle_graph = KernelGraphSpec {
        nodes: vec![node_a, node_b],
        edges: vec![
            exact_edge("m_jit_cycle_a", "out_a", "m_jit_cycle_b", "in_b"),
            exact_edge("m_jit_cycle_b", "out_b", "m_jit_cycle_a", "in_a"),
        ],
    };
    assert_identity_err(&cycle_graph, "cycle");

    let missing_output_graph = KernelGraphSpec {
        nodes: vec![grad0(), grad1_style_scorer()],
        edges: vec![exact_edge(
            "m_jit_grad_0_observer",
            "missing_out",
            "m_jit_grad_1_scorer",
            "descent_x",
        )],
    };
    assert_identity_err(&missing_output_graph, "not produced");
}

#[test]
fn jit_key0_canonical_text_is_semantic_free() {
    let identity = preview_kernel_graph_identity(&valid_grad0_to_scorer_graph())
        .expect("valid graph identity");
    assert!(!identity.canonical_text.is_empty());
    for term in FORBIDDEN_SEMANTIC_TERMS {
        assert!(
            !identity.canonical_text.contains(term),
            "canonical text must not contain forbidden term `{term}`"
        );
    }
}

#[test]
fn jit_key0_identity_preview_has_no_cache_or_scheduler() {
    let source = include_str!("../src/compile/jit_kernel_graph_identity.rs");
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
            "identity module must not reference `{forbidden}`"
        );
    }
    assert!(
        !source.contains("cache.insert")
            && !source.contains("KernelCache")
            && !source.contains("HashMap")
            && !source.contains("BTreeMap"),
        "identity module must not implement runtime cache or registry lookup tables"
    );
}

#[test]
fn jit_key0_landed_grad0_to_scorer_identity_preview() {
    let identity = preview_kernel_graph_identity(&valid_grad0_to_scorer_graph())
        .expect("grad0->scorer identity");
    assert!(!identity.canonical_text.is_empty());
    assert!(identity.stable_key.starts_with("jit-graph-v1:"));
    assert!(identity.canonical_text.contains("m_jit_grad_0_observer"));
    assert!(identity.canonical_text.contains("m_jit_grad_1_scorer"));
    assert!(identity.canonical_text.contains("descent_x"));
    assert!(identity.canonical_text.contains("ExactAuthoritative"));
    println!(
        "key0_grad0_scorer: stable_key={} canonical_lines={}",
        identity.stable_key,
        identity.canonical_text.lines().count()
    );
}
