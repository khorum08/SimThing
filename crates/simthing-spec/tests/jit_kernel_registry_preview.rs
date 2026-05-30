//! Phase M-JIT-REG-0 — Spec-layer test-only kernel registry manifest preview.

use simthing_spec::{
    preview_kernel_registry_manifest, validate_kernel_registry_manifest_preview,
    KernelDescriptorSpec, KernelGraphEdgeSpec, KernelGraphRequestSpec, KernelGraphSpec,
    KernelLane, KernelOutputSpec, KernelRegistryEntryPreview, KernelRegistryLane,
    KernelRegistryManifestPreview, NativeMathClass, OutputAuthority, SpecError,
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
        mag2_source_contract: None,
        score_authority_contract: None,
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

fn three_request_batch() -> Vec<KernelGraphRequestSpec> {
    vec![
        KernelGraphRequestSpec {
            request_id: "req_a".into(),
            graph: valid_grad0_to_scorer_graph(),
        },
        KernelGraphRequestSpec {
            request_id: "req_b".into(),
            graph: reordered_grad0_to_scorer_graph(),
        },
        KernelGraphRequestSpec {
            request_id: "req_variant".into(),
            graph: distinct_grad0_to_scorer_graph(),
        },
    ]
}

fn assert_registry_err(requests: &[KernelGraphRequestSpec], reason_substr: &str) {
    let err =
        preview_kernel_registry_manifest(requests).expect_err("expected registry preview failure");
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

fn assert_manifest_validation_err(manifest: &KernelRegistryManifestPreview, reason_substr: &str) {
    let err = validate_kernel_registry_manifest_preview(manifest)
        .expect_err("expected manifest validation failure");
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

fn sample_entry(stable_key: &str) -> KernelRegistryEntryPreview {
    KernelRegistryEntryPreview {
        stable_key: stable_key.into(),
        canonical_text: "graph\nnode id=sample".into(),
        request_ids: vec!["req_1".into()],
        lane: KernelRegistryLane::TestOnlyPreview,
        default_off: true,
        production_wiring: false,
    }
}

#[test]
fn jit_reg0_manifest_builds_from_cohort_preview() {
    let manifest = preview_kernel_registry_manifest(&three_request_batch())
        .expect("registry manifest");
    assert_eq!(manifest.entries.len(), 2);

    let paired = manifest
        .entries
        .iter()
        .find(|entry| entry.request_ids.len() == 2)
        .expect("paired cohort entry");
    let solo = manifest
        .entries
        .iter()
        .find(|entry| entry.request_ids.len() == 1)
        .expect("solo cohort entry");

    assert_eq!(paired.request_ids, vec!["req_a", "req_b"]);
    assert_eq!(solo.request_ids, vec!["req_variant"]);
    assert!(manifest.entries[0].stable_key <= manifest.entries[1].stable_key);
    println!(
        "reg0_manifest: entries={} paired={} solo={}",
        manifest.entries.len(),
        paired.stable_key,
        solo.stable_key
    );
}

#[test]
fn jit_reg0_manifest_stable_under_request_order() {
    let batch_a = three_request_batch();
    let batch_b = vec![
        batch_a[2].clone(),
        batch_a[0].clone(),
        batch_a[1].clone(),
    ];
    let manifest_a = preview_kernel_registry_manifest(&batch_a).expect("manifest_a");
    let manifest_b = preview_kernel_registry_manifest(&batch_b).expect("manifest_b");
    assert_eq!(manifest_a, manifest_b);
}

#[test]
fn jit_reg0_invalid_graph_rejects_before_manifest() {
    let mag2 = vec![KernelGraphRequestSpec {
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
    }];
    assert_registry_err(&mag2, "approximate/diagnostic");

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
        mag2_source_contract: None,
        score_authority_contract: None,
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
        mag2_source_contract: None,
        score_authority_contract: None,
    };
    let cycle = vec![KernelGraphRequestSpec {
        request_id: "cycle_bad".into(),
        graph: KernelGraphSpec {
            nodes: vec![node_a, node_b],
            edges: vec![
                exact_edge("m_jit_cycle_a", "out_a", "m_jit_cycle_b", "in_b"),
                exact_edge("m_jit_cycle_b", "out_b", "m_jit_cycle_a", "in_a"),
            ],
        },
    }];
    assert_registry_err(&cycle, "cycle");
}

#[test]
fn jit_reg0_duplicate_request_ids_reject() {
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
    assert_registry_err(&requests, "duplicate request id");
}

#[test]
fn jit_reg0_entries_are_test_only_default_off_no_production_wiring() {
    let manifest = preview_kernel_registry_manifest(&three_request_batch())
        .expect("registry manifest");
    for entry in &manifest.entries {
        assert_eq!(entry.lane, KernelRegistryLane::TestOnlyPreview);
        assert!(entry.default_off);
        assert!(!entry.production_wiring);
        assert!(!entry.canonical_text.is_empty());
        assert!(!entry.request_ids.is_empty());
    }
}

#[test]
fn jit_reg0_manifest_validation_rejects_production_shape() {
    assert_manifest_validation_err(
        &KernelRegistryManifestPreview { entries: vec![] },
        "at least one entry",
    );

    assert_manifest_validation_err(
        &KernelRegistryManifestPreview {
            entries: vec![sample_entry("key_a"), sample_entry("key_a")],
        },
        "duplicate stable key",
    );

    let mut empty_text = sample_entry("key_empty_text");
    empty_text.canonical_text.clear();
    assert_manifest_validation_err(
        &KernelRegistryManifestPreview {
            entries: vec![empty_text],
        },
        "canonical text must not be empty",
    );

    let mut empty_requests = sample_entry("key_empty_requests");
    empty_requests.request_ids.clear();
    assert_manifest_validation_err(
        &KernelRegistryManifestPreview {
            entries: vec![empty_requests],
        },
        "at least one request id",
    );

    let mut default_on = sample_entry("key_default_on");
    default_on.default_off = false;
    assert_manifest_validation_err(
        &KernelRegistryManifestPreview {
            entries: vec![default_on],
        },
        "default_off",
    );

    let mut prod_wired = sample_entry("key_prod_wired");
    prod_wired.production_wiring = true;
    assert_manifest_validation_err(
        &KernelRegistryManifestPreview {
            entries: vec![prod_wired],
        },
        "production_wiring",
    );
}

#[test]
fn jit_reg0_preview_has_no_cache_scheduler_or_dispatch() {
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
            "registry module must not reference `{forbidden}`"
        );
    }
    assert!(
        !source.contains("cache.insert")
            && !source.contains("KernelCache")
            && !source.contains("HashMap"),
        "registry module must not implement runtime cache"
    );
}
