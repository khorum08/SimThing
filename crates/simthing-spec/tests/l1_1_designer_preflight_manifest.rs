//! L1-1 — Designer admission RON preflight manifest + diagnostic preview tests.

use simthing_spec::{
    deserialize_designer_admission_preflight_manifest_ron,
    preview_designer_admission_preflight, serialize_designer_admission_preflight_manifest_ron,
    DesignerAdmissionDiagnosticCode, DesignerAdmissionPreflightManifest,
};

const HAPPY_PATH_RON: &str = r#"(
    manifest_id: "frontier_v2_preflight_happy",
    profile_name: "FrontierV2",
    enabled_by_default: false,
    requested_artifact_targets: [
        "AcceptedFrontierV2FixtureArtifacts",
        "FrontierV2CombinedFeedbackFixture",
        "ResourceFlowAllocatorRoute",
    ],
)"#;

fn preview_with_token(field: &str, token: &str) -> simthing_spec::DesignerAdmissionPreviewReport {
    let mut manifest = DesignerAdmissionPreflightManifest::frontier_v2_happy_path();
    match field {
        "guardrail" => manifest.requested_guardrail_overrides.push(token.into()),
        "runtime" => manifest.requested_runtime_features.push(token.into()),
        "mapping" => manifest.requested_mapping_features.push(token.into()),
        "resource_flow" => manifest.requested_resource_flow_features.push(token.into()),
        "authoring" => manifest.requested_authoring_frontend.push(token.into()),
        _ => panic!("unknown field {field}"),
    }
    preview_designer_admission_preflight(&manifest)
}

fn preview_rejects_code(
    report: &simthing_spec::DesignerAdmissionPreviewReport,
    code: DesignerAdmissionDiagnosticCode,
) {
    assert!(report.rejected, "expected rejection for {code:?}");
    assert!(
        report.diagnostics.iter().any(|d| d.code == code),
        "missing diagnostic {code:?}: {:?}",
        report.diagnostics
    );
}

#[test]
fn l1_1_happy_path_preflight_manifest_roundtrips() {
    let parsed = deserialize_designer_admission_preflight_manifest_ron(HAPPY_PATH_RON)
        .expect("parse happy path RON");
    assert_eq!(parsed.profile_name, "FrontierV2");
    assert!(!parsed.enabled_by_default);
    assert_eq!(parsed.requested_artifact_targets.len(), 3);

    let serialized =
        serialize_designer_admission_preflight_manifest_ron(&parsed).expect("serialize");
    let roundtrip = deserialize_designer_admission_preflight_manifest_ron(&serialized)
        .expect("roundtrip parse");
    assert_eq!(roundtrip, parsed);

    let preview = preview_designer_admission_preflight(&parsed);
    assert!(!preview.rejected);
    assert_eq!(preview.accepted_artifact_targets.len(), 3);
    assert!(preview
        .summary_lines
        .iter()
        .any(|l| l.contains("preflight-only")));
}

#[test]
fn l1_1_preflight_preview_accepts_frontier_v2_targets() {
    let manifest = DesignerAdmissionPreflightManifest::frontier_v2_happy_path();
    let preview = preview_designer_admission_preflight(&manifest);
    assert!(!preview.rejected);
    assert_eq!(
        preview.accepted_artifact_targets,
        vec![
            "AcceptedFrontierV2FixtureArtifacts",
            "FrontierV2CombinedFeedbackFixture",
            "ResourceFlowAllocatorRoute",
        ]
    );
}

#[test]
fn l1_1_preflight_preview_rejects_default_on() {
    let mut manifest = DesignerAdmissionPreflightManifest::frontier_v2_happy_path();
    manifest.enabled_by_default = true;
    let preview = preview_designer_admission_preflight(&manifest);
    preview_rejects_code(&preview, DesignerAdmissionDiagnosticCode::DefaultOnRejected);
}

#[test]
fn l1_1_preflight_preview_rejects_resource_flow_bypass() {
    let preview = preview_with_token("guardrail", "resource_flow_bypass");
    preview_rejects_code(
        &preview,
        DesignerAdmissionDiagnosticCode::ResourceFlowBypassRejected,
    );
}

#[test]
fn l1_1_preflight_preview_rejects_cross_entity_movement_write() {
    let preview = preview_with_token("guardrail", "cross_entity_movement_write");
    preview_rejects_code(
        &preview,
        DesignerAdmissionDiagnosticCode::CrossEntityMovementWriteRejected,
    );
}

#[test]
fn l1_1_preflight_preview_rejects_production_commitment_emission() {
    let preview = preview_with_token("guardrail", "production_commitment_emission");
    preview_rejects_code(
        &preview,
        DesignerAdmissionDiagnosticCode::ProductionCommitmentEmissionRejected,
    );
}

#[test]
fn l1_1_preflight_preview_rejects_shared_pool_tick_write() {
    let preview = preview_with_token("guardrail", "shared_pool_tick_write");
    preview_rejects_code(
        &preview,
        DesignerAdmissionDiagnosticCode::SharedPoolTickWriteRejected,
    );
}

#[test]
fn l1_1_preflight_preview_parks_clause_script_and_clausething() {
    let script = preview_with_token("authoring", "clause_script_parser");
    preview_rejects_code(
        &script,
        DesignerAdmissionDiagnosticCode::ClauseScriptParserRequestParked,
    );

    let clausething = preview_with_token("authoring", "clausething_runtime");
    preview_rejects_code(
        &clausething,
        DesignerAdmissionDiagnosticCode::ClauseThingRuntimeRequestParked,
    );
}

#[test]
fn l1_1_preflight_preview_rejects_frontier_v2_5() {
    let preview = preview_with_token("runtime", "frontier_v2_5");
    preview_rejects_code(
        &preview,
        DesignerAdmissionDiagnosticCode::FrontierV2FiveRequestRejected,
    );
}

#[test]
fn l1_1_preflight_preview_rejects_sead_ladder_reopen() {
    for token in ["act_5", "event_3", "obs_5", "pipe_1"] {
        let preview = preview_with_token("runtime", token);
        preview_rejects_code(
            &preview,
            DesignerAdmissionDiagnosticCode::ActEventObsPipeLadderReopenRejected,
        );
    }
}

#[test]
fn l1_1_preflight_preview_rejects_parked_v7_8_lines() {
    let atlas = preview_with_token("mapping", "atlas_batching");
    preview_rejects_code(
        &atlas,
        DesignerAdmissionDiagnosticCode::AtlasRequestedWithoutGate,
    );

    let active = preview_with_token("mapping", "active_mask");
    preview_rejects_code(
        &active,
        DesignerAdmissionDiagnosticCode::ActiveMaskRequestedWithoutGate,
    );

    let perception = preview_with_token("mapping", "perception_fog");
    preview_rejects_code(
        &perception,
        DesignerAdmissionDiagnosticCode::PerceptionFogRequestedWithoutGate,
    );

    let source = preview_with_token("mapping", "source_identity");
    preview_rejects_code(
        &source,
        DesignerAdmissionDiagnosticCode::SourceIdentityRequestedWithoutGate,
    );

    let nested = preview_with_token("resource_flow", "nested_e11b");
    preview_rejects_code(
        &nested,
        DesignerAdmissionDiagnosticCode::NestedE11BRequestedWithoutNamedScenario,
    );

    let e11b5 = preview_with_token("resource_flow", "e11b5_dynamic_enrollment");
    preview_rejects_code(
        &e11b5,
        DesignerAdmissionDiagnosticCode::E11B5RequestedWithoutNamedScenario,
    );

    let d2a = preview_with_token("resource_flow", "d2a_boundary_scheduling");
    preview_rejects_code(
        &d2a,
        DesignerAdmissionDiagnosticCode::D2aRequestedWithoutNamedScenario,
    );
}

#[test]
fn l1_1_no_runtime_wiring_or_simthing_sim_awareness() {
    let manifest_path = format!(
        "{}/Cargo.toml",
        env!("CARGO_MANIFEST_DIR")
    );
    let manifest = std::fs::read_to_string(manifest_path).expect("read Cargo.toml");
    let deps_section = manifest
        .split("[dev-dependencies]")
        .next()
        .expect("dependencies section");
    assert!(!deps_section.contains("simthing-sim"));

    for path in [
        "src/designer_admission/manifest.rs",
        "src/designer_admission/preview.rs",
    ] {
        let contents = std::fs::read_to_string(format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            path
        ))
        .expect("read source");
        for forbidden in ["use simthing_sim", "use simthing-sim", "simthing_sim::", "simthing-sim::", "simthing-gpu", "simthing_gpu::", "GpuSession"]
        {
            assert!(
                !contents.contains(forbidden),
                "{path} must not reference runtime wiring {forbidden}"
            );
        }
    }
}

#[test]
fn l1_1_no_implementer_self_acceptance() {
    for path in [
        "src/designer_admission/manifest.rs",
        "src/designer_admission/preview.rs",
    ] {
        let contents = std::fs::read_to_string(format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            path
        ))
        .expect("read source");
        for forbidden in [
            "Phase M closed",
            "Phase E closed",
            "ClauseThing unblocked",
            "self-acceptance",
        ] {
            assert!(
                !contents.to_ascii_lowercase().contains(&forbidden.to_ascii_lowercase()),
                "{path} must not declare closure"
            );
        }
    }
}
