//! Designer admission preflight evaluation (L1-0 shared substrate).

use super::artifact_target::AcceptedFrontierArtifactTarget;
use super::diagnostic::{
    designer_admission_diagnostic_for_rejection, DesignerAdmissionDiagnostic,
    DesignerAdmissionRejectionKind,
};

/// Designer-facing request surface evaluated by the L1 admission substrate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DesignerAdmissionRequest {
    DefaultOn,
    ResourceFlowBypass,
    CrossEntityMovementWrite {
        source_unit_id: u32,
        target_unit_id: u32,
    },
    ProductionMovementWrite,
    ProductionCommitmentEmission,
    SharedPoolTickWrite,
    ParallelFixtureEconomy,
    CpuPlanner,
    CpuUrgency,
    CpuCommitmentEmission,
    SemanticWgsl,
    SchedulerCache,
    SimthingSimSemanticState,
    AtlasBatching,
    ActiveMask,
    PerceptionFog,
    SourceIdentity,
    NestedE11B,
    E11B5DynamicEnrollment,
    D2aBoundaryScheduling,
    ClauseScriptParser,
    ClauseThingRuntime,
    FrontierV2Five,
    FieldPolicyLadderReopen {
        stage: FieldPolicyLadderStage,
    },
}

/// FIELD_POLICY ladder stage identifiers guarded against unauthorized reopen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldPolicyLadderStage {
    Act5,
    Event3,
    Obs5,
    Pipe1,
}

impl FieldPolicyLadderStage {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Act5 => "ACT-5",
            Self::Event3 => "EVENT-3",
            Self::Obs5 => "OBS-5",
            Self::Pipe1 => "PIPE-1",
        }
    }
}

/// Result of evaluating a designer admission request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DesignerAdmissionPreflightReport {
    pub accepted: bool,
    pub diagnostics: Vec<DesignerAdmissionDiagnostic>,
}

impl DesignerAdmissionPreflightReport {
    pub fn rejected(diagnostics: Vec<DesignerAdmissionDiagnostic>) -> Self {
        Self {
            accepted: false,
            diagnostics,
        }
    }

    pub fn admitted() -> Self {
        Self {
            accepted: true,
            diagnostics: Vec::new(),
        }
    }
}

/// Evaluate a designer admission request against the L1 guardrail substrate.
pub fn evaluate_designer_admission_request(
    request: DesignerAdmissionRequest,
) -> DesignerAdmissionPreflightReport {
    match request {
        DesignerAdmissionRequest::DefaultOn => {
            reject(DesignerAdmissionRejectionKind::DefaultOnRequest)
        }
        DesignerAdmissionRequest::ResourceFlowBypass => {
            reject(DesignerAdmissionRejectionKind::ResourceFlowBypass)
        }
        DesignerAdmissionRequest::CrossEntityMovementWrite {
            source_unit_id,
            target_unit_id,
        } => {
            if source_unit_id == target_unit_id {
                DesignerAdmissionPreflightReport::admitted()
            } else {
                reject(DesignerAdmissionRejectionKind::CrossEntityMovementWrite)
            }
        }
        DesignerAdmissionRequest::ProductionMovementWrite => {
            reject(DesignerAdmissionRejectionKind::ProductionMovementWrite)
        }
        DesignerAdmissionRequest::ProductionCommitmentEmission => {
            reject(DesignerAdmissionRejectionKind::ProductionCommitmentEmission)
        }
        DesignerAdmissionRequest::SharedPoolTickWrite => {
            reject(DesignerAdmissionRejectionKind::SharedPoolTickWrite)
        }
        DesignerAdmissionRequest::ParallelFixtureEconomy => {
            reject(DesignerAdmissionRejectionKind::ParallelFixtureEconomy)
        }
        DesignerAdmissionRequest::CpuPlanner => reject(DesignerAdmissionRejectionKind::CpuPlanner),
        DesignerAdmissionRequest::CpuUrgency => reject(DesignerAdmissionRejectionKind::CpuUrgency),
        DesignerAdmissionRequest::CpuCommitmentEmission => {
            reject(DesignerAdmissionRejectionKind::CpuCommitmentEmission)
        }
        DesignerAdmissionRequest::SemanticWgsl => {
            reject(DesignerAdmissionRejectionKind::SemanticWgslRequest)
        }
        DesignerAdmissionRequest::SchedulerCache => {
            reject(DesignerAdmissionRejectionKind::SchedulerCacheRequest)
        }
        DesignerAdmissionRequest::SimthingSimSemanticState => {
            reject(DesignerAdmissionRejectionKind::SimthingSimSemanticStateRequest)
        }
        DesignerAdmissionRequest::AtlasBatching => {
            reject(DesignerAdmissionRejectionKind::AtlasWithoutGate)
        }
        DesignerAdmissionRequest::ActiveMask => {
            reject(DesignerAdmissionRejectionKind::ActiveMaskWithoutGate)
        }
        DesignerAdmissionRequest::PerceptionFog => {
            reject(DesignerAdmissionRejectionKind::PerceptionFogWithoutGate)
        }
        DesignerAdmissionRequest::SourceIdentity => {
            reject(DesignerAdmissionRejectionKind::SourceIdentityWithoutGate)
        }
        DesignerAdmissionRequest::NestedE11B => {
            reject(DesignerAdmissionRejectionKind::NestedE11BWithoutNamedScenario)
        }
        DesignerAdmissionRequest::E11B5DynamicEnrollment => {
            reject(DesignerAdmissionRejectionKind::E11B5WithoutNamedScenario)
        }
        DesignerAdmissionRequest::D2aBoundaryScheduling => {
            reject(DesignerAdmissionRejectionKind::D2aWithoutNamedScenario)
        }
        DesignerAdmissionRequest::ClauseScriptParser => {
            reject(DesignerAdmissionRejectionKind::ClauseScriptParserParked)
        }
        DesignerAdmissionRequest::ClauseThingRuntime => {
            reject(DesignerAdmissionRejectionKind::ClauseThingRuntimeParked)
        }
        DesignerAdmissionRequest::FrontierV2Five => {
            reject(DesignerAdmissionRejectionKind::FrontierV2FiveRejected)
        }
        DesignerAdmissionRequest::FieldPolicyLadderReopen { stage } => {
            let mut diagnostic = designer_admission_diagnostic_for_rejection(
                DesignerAdmissionRejectionKind::FieldPolicyLadderReopenRejected,
            );
            diagnostic.message = format!(
                "{} ladder reopen request is rejected at designer admission",
                stage.label()
            );
            DesignerAdmissionPreflightReport::rejected(vec![diagnostic])
        }
    }
}

fn reject(kind: DesignerAdmissionRejectionKind) -> DesignerAdmissionPreflightReport {
    DesignerAdmissionPreflightReport::rejected(vec![designer_admission_diagnostic_for_rejection(
        kind,
    )])
}

/// Resolve an artifact target identifier string to the typed lowering target.
pub fn resolve_frontier_artifact_target_id(id: &str) -> Option<AcceptedFrontierArtifactTarget> {
    match id {
        "AcceptedFrontierV2FixtureArtifacts" => {
            Some(AcceptedFrontierArtifactTarget::AcceptedFrontierV2FixtureArtifacts)
        }
        "FrontierV2CombinedFeedbackFixture" => {
            Some(AcceptedFrontierArtifactTarget::FrontierV2CombinedFeedbackFixture)
        }
        "FrontierV2OwnColumnShadow" => {
            Some(AcceptedFrontierArtifactTarget::FrontierV2OwnColumnShadow)
        }
        "FrontierV2BoundaryRequestShadow" => {
            Some(AcceptedFrontierArtifactTarget::FrontierV2BoundaryRequestShadow)
        }
        "ResourceFlowAllocatorRoute" => {
            Some(AcceptedFrontierArtifactTarget::ResourceFlowAllocatorRoute)
        }
        _ => None,
    }
}
