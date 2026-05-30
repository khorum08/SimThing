//! L1-0 — Designer-facing admission substrate (shared diagnostics + guardrail vocabulary).
//!
//! Provides the shared admission vocabulary that L2 / `CLAUSE-SPEC-0` will consume.
//! This module does **not** admit a full designer-authored FrontierV2 scenario, invoke
//! runtime, or wire production `SimSession` behavior.

mod artifact_target;
mod diagnostic;
mod preflight;

pub use artifact_target::{
    accepted_frontier_v2_artifact_target_ids, accepted_frontier_v2_artifact_targets,
    AcceptedFrontierArtifactTarget,
};
pub use diagnostic::{
    all_designer_admission_diagnostic_codes, designer_admission_diagnostic,
    designer_admission_diagnostic_for_rejection, DesignerAdmissionDiagnostic,
    DesignerAdmissionDiagnosticCode, DesignerAdmissionRejectionKind,
    DesignerFacingGuardrailClass,
};
pub use preflight::{
    evaluate_designer_admission_request, resolve_frontier_artifact_target_id,
    DesignerAdmissionPreflightReport, DesignerAdmissionRequest, SeadLadderStage,
};
