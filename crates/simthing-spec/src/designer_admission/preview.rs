//! Designer admission preflight preview report (L1-1).

use super::artifact_target::AcceptedFrontierArtifactTarget;
use super::diagnostic::{
    designer_admission_diagnostic, DesignerAdmissionDiagnostic, DesignerAdmissionDiagnosticCode,
};
use super::manifest::DesignerAdmissionPreflightManifest;
use super::preflight::{
    evaluate_designer_admission_request, resolve_frontier_artifact_target_id,
    DesignerAdmissionRequest, FieldPolicyLadderStage,
};

/// Preview report for a designer admission preflight manifest.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DesignerAdmissionPreviewReport {
    pub manifest_id: String,
    pub accepted_artifact_targets: Vec<String>,
    pub diagnostics: Vec<DesignerAdmissionDiagnostic>,
    pub rejected: bool,
    pub summary_lines: Vec<String>,
}

/// Preview a shallow preflight manifest using the L1-0 guardrail vocabulary.
///
/// This is **preflight-only** — it does not admit or compile a full scenario.
pub fn preview_designer_admission_preflight(
    manifest: &DesignerAdmissionPreflightManifest,
) -> DesignerAdmissionPreviewReport {
    let mut diagnostics = Vec::new();
    let mut accepted_artifact_targets = Vec::new();
    let mut summary_lines = vec![
        "preflight-only: no full scenario admission".into(),
        format!("profile_name: {}", manifest.profile_name),
    ];

    if manifest.manifest_id.trim().is_empty() {
        diagnostics.push(designer_admission_diagnostic(
            DesignerAdmissionDiagnosticCode::MalformedManifestRejected,
            "manifest_id must be non-empty",
            Some("assign a stable manifest_id for preflight tracing"),
        ));
    }

    if manifest.profile_name.trim().is_empty() {
        diagnostics.push(designer_admission_diagnostic(
            DesignerAdmissionDiagnosticCode::MalformedManifestRejected,
            "profile_name must be non-empty",
            Some("name the requested scenario posture profile"),
        ));
    }

    if manifest.enabled_by_default {
        push_request_diagnostics(
            &mut diagnostics,
            evaluate_designer_admission_request(DesignerAdmissionRequest::DefaultOn),
        );
    }

    for target_id in &manifest.requested_artifact_targets {
        match resolve_frontier_artifact_target_id(target_id) {
            Some(target) => {
                accepted_artifact_targets.push(target.id().to_string());
            }
            None => {
                diagnostics.push(designer_admission_diagnostic(
                    DesignerAdmissionDiagnosticCode::UnknownArtifactTargetRejected,
                    format!("unknown requested_artifact_target: {target_id}"),
                    Some(
                        "use accepted FrontierV2 artifact target identifiers from L1-0 vocabulary",
                    ),
                ));
            }
        }
    }

    collect_feature_tokens(manifest, |token| {
        for request in feature_token_to_requests(token, manifest) {
            push_request_diagnostics(
                &mut diagnostics,
                evaluate_designer_admission_request(request),
            );
        }
    });

    let rejected = !diagnostics.is_empty();
    if rejected {
        summary_lines.push(format!(
            "rejected: {} guardrail diagnostic(s)",
            diagnostics.len()
        ));
    } else {
        summary_lines.push("accepted: preflight posture passes guardrail checks".into());
    }

    if !accepted_artifact_targets.is_empty() {
        summary_lines.push(format!(
            "accepted artifact targets (metadata only): {}",
            accepted_artifact_targets.join(", ")
        ));
    }

    DesignerAdmissionPreviewReport {
        manifest_id: manifest.manifest_id.clone(),
        accepted_artifact_targets,
        diagnostics,
        rejected,
        summary_lines,
    }
}

fn push_request_diagnostics(
    out: &mut Vec<DesignerAdmissionDiagnostic>,
    report: super::preflight::DesignerAdmissionPreflightReport,
) {
    if !report.accepted {
        out.extend(report.diagnostics);
    }
}

fn collect_feature_tokens(manifest: &DesignerAdmissionPreflightManifest, mut f: impl FnMut(&str)) {
    for token in &manifest.requested_guardrail_overrides {
        f(token);
    }
    for token in &manifest.requested_runtime_features {
        f(token);
    }
    for token in &manifest.requested_mapping_features {
        f(token);
    }
    for token in &manifest.requested_resource_flow_features {
        f(token);
    }
    for token in &manifest.requested_authoring_frontend {
        f(token);
    }
}

fn feature_token_to_requests(
    token: &str,
    manifest: &DesignerAdmissionPreflightManifest,
) -> Vec<DesignerAdmissionRequest> {
    match token {
        "default_on" => vec![DesignerAdmissionRequest::DefaultOn],
        "resource_flow_bypass" => vec![DesignerAdmissionRequest::ResourceFlowBypass],
        "cross_entity_movement_write" => {
            let source = manifest.cross_entity_movement_source_unit.unwrap_or(1);
            let target = manifest.cross_entity_movement_target_unit.unwrap_or(2);
            vec![DesignerAdmissionRequest::CrossEntityMovementWrite {
                source_unit_id: source,
                target_unit_id: target,
            }]
        }
        "production_movement_write" => vec![DesignerAdmissionRequest::ProductionMovementWrite],
        "production_commitment_emission" => {
            vec![DesignerAdmissionRequest::ProductionCommitmentEmission]
        }
        "shared_pool_tick_write" => vec![DesignerAdmissionRequest::SharedPoolTickWrite],
        "parallel_fixture_economy" => vec![DesignerAdmissionRequest::ParallelFixtureEconomy],
        "cpu_planner" => vec![DesignerAdmissionRequest::CpuPlanner],
        "cpu_urgency" => vec![DesignerAdmissionRequest::CpuUrgency],
        "cpu_commitment_emission" => vec![DesignerAdmissionRequest::CpuCommitmentEmission],
        "semantic_wgsl" => vec![DesignerAdmissionRequest::SemanticWgsl],
        "scheduler_cache" => vec![DesignerAdmissionRequest::SchedulerCache],
        "simthing_sim_semantic_state" => {
            vec![DesignerAdmissionRequest::SimthingSimSemanticState]
        }
        "frontier_v2_5" => vec![DesignerAdmissionRequest::FrontierV2Five],
        "atlas_batching" => vec![DesignerAdmissionRequest::AtlasBatching],
        "active_mask" => vec![DesignerAdmissionRequest::ActiveMask],
        "perception_fog" => vec![DesignerAdmissionRequest::PerceptionFog],
        "source_identity" => vec![DesignerAdmissionRequest::SourceIdentity],
        "nested_e11b" => vec![DesignerAdmissionRequest::NestedE11B],
        "e11b5_dynamic_enrollment" => vec![DesignerAdmissionRequest::E11B5DynamicEnrollment],
        "d2a_boundary_scheduling" => vec![DesignerAdmissionRequest::D2aBoundaryScheduling],
        "clause_script_parser" => vec![DesignerAdmissionRequest::ClauseScriptParser],
        "clausething_runtime" => vec![DesignerAdmissionRequest::ClauseThingRuntime],
        "act_5" => vec![DesignerAdmissionRequest::FieldPolicyLadderReopen {
            stage: FieldPolicyLadderStage::Act5,
        }],
        "event_3" => vec![DesignerAdmissionRequest::FieldPolicyLadderReopen {
            stage: FieldPolicyLadderStage::Event3,
        }],
        "obs_5" => vec![DesignerAdmissionRequest::FieldPolicyLadderReopen {
            stage: FieldPolicyLadderStage::Obs5,
        }],
        "pipe_1" => vec![DesignerAdmissionRequest::FieldPolicyLadderReopen {
            stage: FieldPolicyLadderStage::Pipe1,
        }],
        _ => Vec::new(),
    }
}

/// Resolve preview artifact target ids to typed targets (metadata only).
pub fn preview_accepted_artifact_targets(
    target_ids: &[String],
) -> Vec<AcceptedFrontierArtifactTarget> {
    target_ids
        .iter()
        .filter_map(|id| resolve_frontier_artifact_target_id(id))
        .collect()
}
