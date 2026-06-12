//! L1-0 — Designer admission substrate preflight tests.

use simthing_spec::{
    accepted_frontier_v2_artifact_target_ids, accepted_frontier_v2_artifact_targets,
    all_designer_admission_diagnostic_codes, evaluate_designer_admission_request,
    resolve_frontier_artifact_target_id, AcceptedFrontierArtifactTarget,
    DesignerAdmissionDiagnosticCode, DesignerAdmissionRequest, FieldPolicyLadderStage,
};

#[test]
fn l1_0_guardrail_diagnostic_codes_are_stable() {
    let codes = all_designer_admission_diagnostic_codes();
    assert_eq!(codes.len(), 38, "expected 38 stable diagnostic codes");

    let mut seen = std::collections::BTreeSet::new();
    for code in codes {
        let s = code.as_str();
        assert!(
            s.starts_with("L1-0-") || s.starts_with("MOBILITY-SCENARIO-0-"),
            "code must be L1-0 or MOBILITY-SCENARIO-0 prefixed: {s}"
        );
        assert!(seen.insert(s), "duplicate diagnostic code: {s}");
    }

    assert_eq!(
        DesignerAdmissionDiagnosticCode::MalformedManifestRejected.as_str(),
        "L1-0-MALFORMED-MANIFEST-REJECTED"
    );
    assert_eq!(
        DesignerAdmissionDiagnosticCode::UnknownArtifactTargetRejected.as_str(),
        "L1-0-UNKNOWN-ARTIFACT-TARGET-REJECTED"
    );
    assert_eq!(
        DesignerAdmissionDiagnosticCode::ResourceFlowBypassRejected.as_str(),
        "L1-0-RESOURCE-FLOW-BYPASS-REJECTED"
    );
    assert_eq!(
        DesignerAdmissionDiagnosticCode::FrontierV2FiveRequestRejected.as_str(),
        "L1-0-FRONTIERV2-5-REQUEST-REJECTED"
    );
}

#[test]
fn l1_0_rejects_frontier_v2_5_request() {
    let report = evaluate_designer_admission_request(DesignerAdmissionRequest::FrontierV2Five);
    assert!(!report.accepted);
    assert_eq!(report.diagnostics.len(), 1);
    assert_eq!(
        report.diagnostics[0].code,
        DesignerAdmissionDiagnosticCode::FrontierV2FiveRequestRejected
    );
}

#[test]
fn l1_0_rejects_act_event_obs_pipe_ladder_reopen() {
    for stage in [
        FieldPolicyLadderStage::Act5,
        FieldPolicyLadderStage::Event3,
        FieldPolicyLadderStage::Obs5,
        FieldPolicyLadderStage::Pipe1,
    ] {
        let report = evaluate_designer_admission_request(
            DesignerAdmissionRequest::FieldPolicyLadderReopen { stage },
        );
        assert!(!report.accepted, "expected rejection for {}", stage.label());
        assert_eq!(
            report.diagnostics[0].code,
            DesignerAdmissionDiagnosticCode::ActEventObsPipeLadderReopenRejected
        );
        assert!(report.diagnostics[0].message.contains(stage.label()));
    }
}

#[test]
fn l1_0_rejects_resource_flow_bypass() {
    let report = evaluate_designer_admission_request(DesignerAdmissionRequest::ResourceFlowBypass);
    assert!(!report.accepted);
    assert_eq!(
        report.diagnostics[0].code,
        DesignerAdmissionDiagnosticCode::ResourceFlowBypassRejected
    );
}

#[test]
fn l1_0_rejects_cross_entity_movement_write() {
    let rejected =
        evaluate_designer_admission_request(DesignerAdmissionRequest::CrossEntityMovementWrite {
            source_unit_id: 1,
            target_unit_id: 2,
        });
    assert!(!rejected.accepted);
    assert_eq!(
        rejected.diagnostics[0].code,
        DesignerAdmissionDiagnosticCode::CrossEntityMovementWriteRejected
    );

    let admitted =
        evaluate_designer_admission_request(DesignerAdmissionRequest::CrossEntityMovementWrite {
            source_unit_id: 7,
            target_unit_id: 7,
        });
    assert!(admitted.accepted);
    assert!(admitted.diagnostics.is_empty());
}

#[test]
fn l1_0_rejects_production_commitment_emission() {
    let report =
        evaluate_designer_admission_request(DesignerAdmissionRequest::ProductionCommitmentEmission);
    assert!(!report.accepted);
    assert_eq!(
        report.diagnostics[0].code,
        DesignerAdmissionDiagnosticCode::ProductionCommitmentEmissionRejected
    );
}

#[test]
fn l1_0_rejects_shared_pool_tick_write() {
    let report = evaluate_designer_admission_request(DesignerAdmissionRequest::SharedPoolTickWrite);
    assert!(!report.accepted);
    assert_eq!(
        report.diagnostics[0].code,
        DesignerAdmissionDiagnosticCode::SharedPoolTickWriteRejected
    );
}

#[test]
fn l1_0_rejects_clausething_runtime_request() {
    let report = evaluate_designer_admission_request(DesignerAdmissionRequest::ClauseThingRuntime);
    assert!(!report.accepted);
    assert_eq!(
        report.diagnostics[0].code,
        DesignerAdmissionDiagnosticCode::ClauseThingRuntimeRequestParked
    );
}

#[test]
fn l1_0_rejects_clause_script_parser_request() {
    let report = evaluate_designer_admission_request(DesignerAdmissionRequest::ClauseScriptParser);
    assert!(!report.accepted);
    assert_eq!(
        report.diagnostics[0].code,
        DesignerAdmissionDiagnosticCode::ClauseScriptParserRequestParked
    );
}

#[test]
fn l1_0_names_frontier_v2_artifact_targets_without_runtime_wiring() {
    let targets = accepted_frontier_v2_artifact_targets();
    assert_eq!(targets.len(), 5);

    let ids = accepted_frontier_v2_artifact_target_ids();
    assert_eq!(ids.len(), 5);

    for (target, id) in targets.iter().zip(ids.iter()) {
        assert_eq!(target.id(), *id);
        assert!(!target.description().is_empty());
        assert_eq!(resolve_frontier_artifact_target_id(id), Some(*target));
    }

    assert_eq!(
        AcceptedFrontierArtifactTarget::FrontierV2CombinedFeedbackFixture.id(),
        "FrontierV2CombinedFeedbackFixture"
    );
    assert_eq!(
        AcceptedFrontierArtifactTarget::ResourceFlowAllocatorRoute.id(),
        "ResourceFlowAllocatorRoute"
    );
    assert!(resolve_frontier_artifact_target_id("NotARealTarget").is_none());
}

#[test]
fn l1_0_no_simthing_sim_semantic_awareness() {
    let manifest = std::fs::read_to_string(env!("CARGO_MANIFEST_DIR").to_owned() + "/Cargo.toml")
        .expect("read simthing-spec Cargo.toml");
    let deps_section = manifest
        .split("[dev-dependencies]")
        .next()
        .expect("dependencies section");
    assert!(
        !deps_section.contains("simthing-sim"),
        "simthing-spec production dependencies must not include simthing-sim"
    );

    for path in [
        "src/designer_admission/mod.rs",
        "src/designer_admission/diagnostic.rs",
        "src/designer_admission/artifact_target.rs",
        "src/designer_admission/preflight.rs",
    ] {
        let contents = std::fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), path))
            .expect("read designer_admission source");
        for forbidden in [
            "use simthing_sim",
            "use simthing-sim",
            "simthing_sim::",
            "simthing-sim::",
        ] {
            assert!(
                !contents.contains(forbidden),
                "{path} must not import {forbidden}"
            );
        }
    }
}

#[test]
fn l1_0_no_implementer_self_acceptance() {
    for path in [
        "src/designer_admission/diagnostic.rs",
        "src/designer_admission/artifact_target.rs",
        "src/designer_admission/preflight.rs",
    ] {
        let contents = std::fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), path))
            .expect("read designer_admission source");
        for forbidden in [
            "Phase M closed",
            "Phase E closed",
            "ClauseThing unblocked",
            "design authority accepts",
            "self-acceptance",
        ] {
            assert!(
                !contents
                    .to_ascii_lowercase()
                    .contains(&forbidden.to_ascii_lowercase()),
                "{path} must not declare closure: {forbidden}"
            );
        }
    }
}
