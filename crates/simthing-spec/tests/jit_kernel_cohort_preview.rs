//! Phase M-JIT-COHORT-0 — Spec-layer kernel graph cohort grouping preview.

use std::collections::BTreeMap;

use simthing_spec::{
    preview_kernel_graph_cohorts, KernelDescriptorSpec, KernelGraphCohortPreview,
    KernelGraphCohortPreviewSet, KernelGraphEdgeSpec, KernelGraphRequestSpec, KernelGraphSpec,
    KernelLane, KernelOutputSpec, NativeMathClass, OutputAuthority, SpecError,
};

fn grad0() -> KernelDescriptorSpec {
    simthing_spec::landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == "m_jit_grad_0_observer")
        .expect("grad0 descriptor")
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
    }
}

fn grad1_style_scorer_with_bias_read() -> KernelDescriptorSpec {
    KernelDescriptorSpec {
        id: "m_jit_grad_1_scorer".into(),
        lane: KernelLane::TestOnly,
        reads: vec!["descent_x".into(), "descent_y".into(), "bias".into()],
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

fn reordered_grad0_to_scorer_graph() -> KernelGraphSpec {
    KernelGraphSpec {
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
    }
}

fn distinct_grad0_to_scorer_graph() -> KernelGraphSpec {
    KernelGraphSpec {
        nodes: vec![grad0(), grad1_style_scorer_with_bias_read()],
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

fn assert_cohort_err(requests: &[KernelGraphRequestSpec], reason_substr: &str) {
    let err = preview_kernel_graph_cohorts(requests).expect_err("expected cohort failure");
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

/// Test-local collision guard exercise: inject resolved identities without public API bypass.
fn test_group_cohort_previews_from_resolved(
    requests: &[KernelGraphRequestSpec],
    resolved: &[(String, String)],
) -> Result<KernelGraphCohortPreviewSet, SpecError> {
    if requests.len() != resolved.len() {
        return Err(SpecError::JitKernelDescriptorAdmission {
            kernel: "cohort".into(),
            reason: "request count must match resolved identity count".into(),
        });
    }

    let mut seen_request_ids = std::collections::HashSet::new();
    for request in requests {
        if !seen_request_ids.insert(request.request_id.clone()) {
            return Err(SpecError::JitKernelDescriptorAdmission {
                kernel: request.request_id.clone(),
                reason: format!("duplicate request id `{}`", request.request_id),
            });
        }
    }

    let mut groups: BTreeMap<String, (String, Vec<String>)> = BTreeMap::new();
    for (request, (stable_key, canonical_text)) in requests.iter().zip(resolved.iter()) {
        match groups.get_mut(stable_key) {
            Some((existing_canonical, request_ids)) => {
                if existing_canonical != canonical_text {
                    return Err(SpecError::JitKernelDescriptorAdmission {
                        kernel: "cohort".into(),
                        reason: format!(
                            "stable key `{stable_key}` maps to conflicting canonical text for request `{}`",
                            request.request_id
                        ),
                    });
                }
                request_ids.push(request.request_id.clone());
            }
            None => {
                groups.insert(
                    stable_key.clone(),
                    (canonical_text.clone(), vec![request.request_id.clone()]),
                );
            }
        }
    }

    let mut cohorts = Vec::with_capacity(groups.len());
    for (stable_key, (canonical_text, mut request_ids)) in groups {
        request_ids.sort();
        cohorts.push(KernelGraphCohortPreview {
            stable_key,
            canonical_text,
            request_ids,
        });
    }

    Ok(KernelGraphCohortPreviewSet { cohorts })
}

#[test]
fn jit_cohort0_identical_graphs_group_together() {
    let requests = vec![
        KernelGraphRequestSpec {
            request_id: "req_b".into(),
            graph: reordered_grad0_to_scorer_graph(),
        },
        KernelGraphRequestSpec {
            request_id: "req_a".into(),
            graph: valid_grad0_to_scorer_graph(),
        },
    ];
    let preview = preview_kernel_graph_cohorts(&requests).expect("cohort preview");
    assert_eq!(preview.cohorts.len(), 1);
    assert_eq!(preview.cohorts[0].request_ids, vec!["req_a", "req_b"]);
    assert!(preview.cohorts[0].stable_key.starts_with("jit-graph-v1:"));
    assert!(!preview.cohorts[0].canonical_text.is_empty());
    println!(
        "cohort0_identical: key={} count=2",
        preview.cohorts[0].stable_key
    );
}

#[test]
fn jit_cohort0_distinct_graphs_split() {
    let requests = vec![
        KernelGraphRequestSpec {
            request_id: "base".into(),
            graph: valid_grad0_to_scorer_graph(),
        },
        KernelGraphRequestSpec {
            request_id: "variant".into(),
            graph: distinct_grad0_to_scorer_graph(),
        },
    ];
    let preview = preview_kernel_graph_cohorts(&requests).expect("cohort preview");
    assert_eq!(preview.cohorts.len(), 2);
    assert_ne!(preview.cohorts[0].stable_key, preview.cohorts[1].stable_key);
    assert_ne!(
        preview.cohorts[0].canonical_text,
        preview.cohorts[1].canonical_text
    );
    assert_eq!(preview.cohorts[0].request_ids, vec!["base"]);
    assert_eq!(preview.cohorts[1].request_ids, vec!["variant"]);
}

#[test]
fn jit_cohort0_invalid_graph_rejects() {
    let mag2 = KernelGraphRequestSpec {
        request_id: "mag2_bad".into(),
        graph: KernelGraphSpec {
            nodes: vec![grad0(), grad1_style_scorer()],
            edges: vec![exact_edge(
                "m_jit_grad_0_observer",
                "mag2",
                "m_jit_grad_1_scorer",
                "descent_x",
            )],
        },
    };
    assert_cohort_err(&[mag2], "approximate/diagnostic");

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
        pre_sqrt_contract: None,
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
        pre_sqrt_contract: None,
    };
    let cycle = KernelGraphRequestSpec {
        request_id: "cycle_bad".into(),
        graph: KernelGraphSpec {
            nodes: vec![node_a, node_b],
            edges: vec![
                exact_edge("m_jit_cycle_a", "out_a", "m_jit_cycle_b", "in_b"),
                exact_edge("m_jit_cycle_b", "out_b", "m_jit_cycle_a", "in_a"),
            ],
        },
    };
    assert_cohort_err(&[cycle], "cycle");
}

#[test]
fn jit_cohort0_duplicate_request_ids_reject() {
    let requests = vec![
        KernelGraphRequestSpec {
            request_id: "dup".into(),
            graph: valid_grad0_to_scorer_graph(),
        },
        KernelGraphRequestSpec {
            request_id: "dup".into(),
            graph: reordered_grad0_to_scorer_graph(),
        },
    ];
    assert_cohort_err(&requests, "duplicate request id");
}

#[test]
fn jit_cohort0_output_stable_under_request_order_variation() {
    let requests_a = vec![
        KernelGraphRequestSpec {
            request_id: "req_1".into(),
            graph: valid_grad0_to_scorer_graph(),
        },
        KernelGraphRequestSpec {
            request_id: "req_2".into(),
            graph: reordered_grad0_to_scorer_graph(),
        },
        KernelGraphRequestSpec {
            request_id: "req_3".into(),
            graph: distinct_grad0_to_scorer_graph(),
        },
    ];
    let requests_b = vec![
        KernelGraphRequestSpec {
            request_id: "req_3".into(),
            graph: distinct_grad0_to_scorer_graph(),
        },
        KernelGraphRequestSpec {
            request_id: "req_1".into(),
            graph: valid_grad0_to_scorer_graph(),
        },
        KernelGraphRequestSpec {
            request_id: "req_2".into(),
            graph: reordered_grad0_to_scorer_graph(),
        },
    ];

    let preview_a = preview_kernel_graph_cohorts(&requests_a).expect("preview_a");
    let preview_b = preview_kernel_graph_cohorts(&requests_b).expect("preview_b");
    assert_eq!(preview_a, preview_b);
}

#[test]
fn jit_cohort0_rejects_same_key_different_canonical_text() {
    let requests = vec![
        KernelGraphRequestSpec {
            request_id: "req_a".into(),
            graph: valid_grad0_to_scorer_graph(),
        },
        KernelGraphRequestSpec {
            request_id: "req_b".into(),
            graph: reordered_grad0_to_scorer_graph(),
        },
    ];
    let err = test_group_cohort_previews_from_resolved(
        &requests,
        &[
            ("jit-graph-v1:deadbeefdeadbeef".into(), "canonical_a".into()),
            ("jit-graph-v1:deadbeefdeadbeef".into(), "canonical_b".into()),
        ],
    )
    .expect_err("collision should reject");
    match err {
        SpecError::JitKernelDescriptorAdmission { reason, .. } => {
            assert!(
                reason.contains("conflicting canonical text"),
                "unexpected reason: {reason}"
            );
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn jit_cohort0_preview_has_no_cache_scheduler_or_dispatch() {
    let source = include_str!("../src/compile/jit_kernel_cohort_preview.rs");
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
            "cohort module must not reference `{forbidden}`"
        );
    }
    assert!(
        !source.contains("cache.insert")
            && !source.contains("KernelCache")
            && !source.contains("HashMap")
            && !source.contains("test_group_cohort_previews_from_resolved"),
        "cohort module must not implement runtime cache or export collision-test helper"
    );
}
