//! L1-1 — Designer admission RON preflight manifest + diagnostic preview tests.

use simthing_spec::{
    deserialize_designer_admission_preflight_manifest_ron, preview_designer_admission_preflight,
    serialize_designer_admission_preflight_manifest_ron, DesignerAdmissionDiagnosticCode,
    DesignerAdmissionPreflightManifest,
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
fn l1_1_preflight_preview_reports_structural_manifest_diagnostics() {
    let mut malformed = DesignerAdmissionPreflightManifest::frontier_v2_happy_path();
    malformed.manifest_id.clear();
    malformed.profile_name.clear();
    let preview = preview_designer_admission_preflight(&malformed);
    preview_rejects_code(
        &preview,
        DesignerAdmissionDiagnosticCode::MalformedManifestRejected,
    );
    assert!(!preview.diagnostics.iter().any(
        |d| d.code == DesignerAdmissionDiagnosticCode::SimthingSimSemanticStateRequestRejected
    ));

    let mut unknown = DesignerAdmissionPreflightManifest::frontier_v2_happy_path();
    unknown
        .requested_artifact_targets
        .push("UnknownFrontierArtifact".into());
    let preview = preview_designer_admission_preflight(&unknown);
    preview_rejects_code(
        &preview,
        DesignerAdmissionDiagnosticCode::UnknownArtifactTargetRejected,
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
fn l1_1_no_runtime_wiring_or_simthing_sim_awareness() {
    let manifest_path = format!("{}/Cargo.toml", env!("CARGO_MANIFEST_DIR"));
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
        let contents = std::fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), path))
            .expect("read source");
        for forbidden in [
            "use simthing_sim",
            "use simthing-sim",
            "simthing_sim::",
            "simthing-sim::",
            "simthing-gpu",
            "simthing_gpu::",
            "GpuSession",
        ] {
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
        let contents = std::fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), path))
            .expect("read source");
        for forbidden in [
            "Phase M closed",
            "Phase E closed",
            "ClauseThing unblocked",
            "self-acceptance",
        ] {
            assert!(
                !contents
                    .to_ascii_lowercase()
                    .contains(&forbidden.to_ascii_lowercase()),
                "{path} must not declare closure"
            );
        }
    }
}
