//! L1-0 — Designer admission substrate preflight tests.

use simthing_spec::{
    accepted_frontier_v2_artifact_target_ids, accepted_frontier_v2_artifact_targets,
    all_designer_admission_diagnostic_codes, evaluate_designer_admission_request,
    resolve_frontier_artifact_target_id, AcceptedFrontierArtifactTarget,
    DesignerAdmissionDiagnosticCode, DesignerAdmissionRequest, FieldPolicyLadderStage,
};
