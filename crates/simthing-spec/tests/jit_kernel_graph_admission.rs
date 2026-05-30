//! Phase M-JIT-DESC-2 — Spec-layer JIT kernel graph composition admission preview.

use simthing_spec::{
    landed_jit_kernel_descriptors, validate_kernel_graph_admission, KernelDescriptorSpec,
    KernelGraphEdgeSpec, KernelGraphSpec, KernelLane, KernelOutputSpec, NativeMathClass,
    OutputAuthority, SpecError,
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

fn assert_graph_err(graph: &KernelGraphSpec, reason_substr: &str) {
    let err = validate_kernel_graph_admission(graph).expect_err("expected graph admission failure");
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
fn jit_desc2_valid_exact_graph_admits() {
    let graph = KernelGraphSpec {
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
    };
    validate_kernel_graph_admission(&graph).expect("valid exact graph should admit");
    println!("desc2_valid_graph: grad0->scorer exact descent edges admit");
}

#[test]
fn jit_desc2_rejects_mag2_as_exact_input() {
    let graph = KernelGraphSpec {
        nodes: vec![grad0(), grad1_style_scorer()],
        edges: vec![exact_edge(
            "m_jit_grad_0_observer",
            "mag2",
            "m_jit_grad_1_scorer",
            "descent_x",
        )],
    };
    assert_graph_err(&graph, "approximate/diagnostic");
}

#[test]
fn jit_desc2_rejects_sqrt_output_as_exact_input() {
    let graph = KernelGraphSpec {
        nodes: vec![sqrt0(), grad1_style_scorer()],
        edges: vec![exact_edge(
            "m_jit_sqrt_0_candidate",
            "sqrt_out",
            "m_jit_grad_1_scorer",
            "descent_x",
        )],
    };
    assert_graph_err(&graph, "approximate/diagnostic");
}

#[test]
fn jit_desc2_rejects_missing_output() {
    let graph = KernelGraphSpec {
        nodes: vec![grad1(), grad1_style_scorer()],
        edges: vec![exact_edge(
            "m_jit_grad_1_observer_score",
            "mag2",
            "m_jit_grad_1_scorer",
            "descent_x",
        )],
    };
    assert_graph_err(&graph, "not produced");
}

#[test]
fn jit_desc2_rejects_duplicate_node_ids() {
    let graph = KernelGraphSpec {
        nodes: vec![grad0(), grad0()],
        edges: vec![],
    };
    assert_graph_err(&graph, "duplicate node id");
}

#[test]
fn jit_desc2_rejects_missing_producer_or_consumer() {
    let missing_producer = KernelGraphSpec {
        nodes: vec![grad1_style_scorer()],
        edges: vec![exact_edge(
            "missing_producer",
            "descent_x",
            "m_jit_grad_1_scorer",
            "descent_x",
        )],
    };
    assert_graph_err(&missing_producer, "missing producer node");

    let missing_consumer = KernelGraphSpec {
        nodes: vec![grad0()],
        edges: vec![exact_edge(
            "m_jit_grad_0_observer",
            "descent_x",
            "missing_consumer",
            "descent_x",
        )],
    };
    assert_graph_err(&missing_consumer, "missing consumer node");
}

#[test]
fn jit_desc2_rejects_cycles() {
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
        exact_sqrt_artifact: None,
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
        exact_sqrt_artifact: None,
    };
    let graph = KernelGraphSpec {
        nodes: vec![node_a, node_b],
        edges: vec![
            exact_edge("m_jit_cycle_a", "out_a", "m_jit_cycle_b", "in_b"),
            exact_edge("m_jit_cycle_b", "out_b", "m_jit_cycle_a", "in_a"),
        ],
    };
    assert_graph_err(&graph, "cycle");
}

#[test]
fn jit_desc2_rejects_self_edges() {
    let graph = KernelGraphSpec {
        nodes: vec![grad0()],
        edges: vec![exact_edge(
            "m_jit_grad_0_observer",
            "descent_x",
            "m_jit_grad_0_observer",
            "descent_x",
        )],
    };
    assert_graph_err(&graph, "self-edge");
}

#[test]
fn jit_desc2_rejects_production_candidate_node() {
    let mut bad = grad0();
    bad.lane = KernelLane::ProductionCandidate;
    let graph = KernelGraphSpec {
        nodes: vec![bad, grad1_style_scorer()],
        edges: vec![exact_edge(
            "m_jit_grad_0_observer",
            "descent_x",
            "m_jit_grad_1_scorer",
            "descent_x",
        )],
    };
    assert_graph_err(&graph, "ProductionCandidate");
}

#[test]
fn jit_desc2_rejects_production_wired_node() {
    let mut bad = grad0();
    bad.production_wiring = true;
    let graph = KernelGraphSpec {
        nodes: vec![bad, grad1_style_scorer()],
        edges: vec![exact_edge(
            "m_jit_grad_0_observer",
            "descent_x",
            "m_jit_grad_1_scorer",
            "descent_x",
        )],
    };
    assert_graph_err(&graph, "production_wiring");
}

#[test]
fn jit_desc2_grad1_score_remains_exact_authoritative() {
    let grad1_desc = grad1();
    let score = grad1_desc
        .writes
        .iter()
        .find(|out| out.name == "score")
        .expect("grad1 score output");
    assert_eq!(score.authority, OutputAuthority::ExactAuthoritative);
}
