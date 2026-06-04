//! L1-0 — Designer-facing admission substrate (shared diagnostics + guardrail vocabulary).
//!
//! Provides the shared admission vocabulary that L2 / `CLAUSE-SPEC-0` will consume.
//! This module does **not** admit a full designer-authored FrontierV2 scenario, invoke
//! runtime, or wire production `SimSession` behavior.

mod artifact_target;
mod atlas;
mod clause_spec;
mod diagnostic;
mod manifest;
mod mobility_alloc0;
mod mobility_audit0;
mod mobility_econ0;
mod mobility_idroute0;
mod mobility_owner0;
mod mobility_reenroll0;
mod mobility_runtime0;
mod mobility_runtime1a;
mod mobility_scenario0;
mod preflight;
mod preview;
mod v7_8_line_scenarios;

pub use artifact_target::{
    accepted_frontier_v2_artifact_target_ids, accepted_frontier_v2_artifact_targets,
    AcceptedFrontierArtifactTarget,
};
pub use atlas::{
    AtlasAdmissionDecision, AtlasAdmissionProfile, AtlasAdmissionSpec, AtlasIsolationAdmissionMode,
};
pub use clause_spec::{
    admit_clause_spec_frontier_v2_scenario, ClauseSpecArtifactTargets, ClauseSpecFaction,
    ClauseSpecFrontierV2Admission, ClauseSpecFrontierV2LoweringSummary,
    ClauseSpecFrontierV2Scenario, ClauseSpecGrid, ClauseSpecMapping, ClauseSpecMovementFeedback,
    ClauseSpecMovementFeedbackMode, ClauseSpecResourceFlow, ClauseSpecResourceFlowRoute,
    ClauseSpecStructuralFeedback, ClauseSpecStructuralFeedbackMode,
    CLAUSE_SPEC_FRONTIER_V2_GRID_CAP, CLAUSE_SPEC_FRONTIER_V2_MIN_TICKS,
    CLAUSE_SPEC_FRONTIER_V2_PROFILE,
};
pub use diagnostic::{
    all_designer_admission_diagnostic_codes, designer_admission_diagnostic,
    designer_admission_diagnostic_for_rejection, DesignerAdmissionDiagnostic,
    DesignerAdmissionDiagnosticCode, DesignerAdmissionRejectionKind, DesignerFacingGuardrailClass,
};
pub use manifest::DesignerAdmissionPreflightManifest;
pub use mobility_alloc0::{
    mobility_alloc0_layout_checksum_cpu, mobility_alloc0_layout_checksum_gpu_proxy,
    plan_mobility_alloc0, MobilityAlloc0Assignment, MobilityAlloc0BlockSpec,
    MobilityAlloc0BoundaryEvent, MobilityAlloc0BoundaryEventKind,
    MobilityAlloc0ForbiddenPathRequests, MobilityAlloc0LiveSlice, MobilityAlloc0ParentKey,
    MobilityAlloc0PlanInput, MobilityAlloc0PlanReport, MOBILITY_ALLOC0_ID,
};
pub use mobility_audit0::{
    audit_mobility_owner_band_budget, audit_mobility_owner_band_budget_with_ceiling,
    mobility_audit0_family_budgets, mobility_audit0_packet_matches_accepted_constants,
    mobility_audit0_required_orderband_depth, MobilityAudit0CirculationFamily,
    MobilityAudit0FamilyBudget, MobilityAudit0Report, MobilityAudit0ScenarioConstants,
    MobilityAudit0Verdict, MOBILITY_AUDIT0_CURRENT_MAX_ORDERBAND_DEPTH, MOBILITY_AUDIT0_ID,
    MOBILITY_AUDIT0_NARROWING_CEILING,
};
pub use mobility_econ0::{
    mobility_econ0_layout_checksum_cpu, mobility_econ0_layout_checksum_gpu_proxy,
    plan_mobility_econ0, MobilityEcon0DownDisburse, MobilityEcon0ForbiddenPathRequests,
    MobilityEcon0LocalCellRecord, MobilityEcon0PlanInput, MobilityEcon0PlanReport,
    MobilityEcon0SessionAggregate, MobilityEcon0SessionResourceKey, MOBILITY_ECON0_ID,
};
pub use mobility_idroute0::{
    mobility_idroute0_layout_checksum_cpu, mobility_idroute0_layout_checksum_gpu_proxy,
    plan_mobility_idroute0, DirectedDisburse, IdentityLane, MobilityIdroute0ForbiddenPathRequests,
    MobilityIdroute0LocalRecord, MobilityIdroute0PlanInput, MobilityIdroute0PlanReport,
    PerIdentitySum, MOBILITY_IDROUTE0_ID,
};
pub use mobility_owner0::{
    mobility_owner0_layout_checksum_cpu, mobility_owner0_layout_checksum_gpu_proxy,
    plan_mobility_owner0, MobilityOwner0AppliedOverlay, MobilityOwner0ColumnKind,
    MobilityOwner0ColumnValue, MobilityOwner0FissionResult, MobilityOwner0ForbiddenPathRequests,
    MobilityOwner0GenerationResync, MobilityOwner0LocalRecord, MobilityOwner0Overlay,
    MobilityOwner0OwnerChange, MobilityOwner0PlanInput, MobilityOwner0PlanReport,
    MOBILITY_OWNER0_CURRENT_MAX_ORDERBAND_DEPTH, MOBILITY_OWNER0_ID,
    MOBILITY_OWNER0_REQUIRED_ORDERBAND_DEPTH,
};
pub use mobility_reenroll0::{
    mobility_reenroll0_layout_checksum_cpu, mobility_reenroll0_layout_checksum_gpu_proxy,
    plan_mobility_reenroll0, MobilityReenroll0CommittedMove,
    MobilityReenroll0ForbiddenPathRequests, MobilityReenroll0Move, MobilityReenroll0PlanInput,
    MobilityReenroll0PlanReport, MobilityReenroll0RegistryState, MOBILITY_REENROLL0_ID,
};
pub use mobility_runtime0::{
    compose_mobility_runtime0, MobilityRuntime0CompositionInput, MobilityRuntime0CompositionReport,
    MobilityRuntime0ForbiddenPathRequests, MobilityRuntime0HarnessConfig, MOBILITY_RUNTIME0_ID,
    MOBILITY_RUNTIME0_ORDER,
};
pub use mobility_runtime1a::{
    run_mobility_runtime1a_production_fixture, MobilityRuntime1aFixtureGate,
    MobilityRuntime1aForbiddenPathRequests, MobilityRuntime1aProductionFixtureInput,
    MobilityRuntime1aProductionFixtureReport, MobilityRuntime1aSimSessionSurface,
    MOBILITY_RUNTIME1A_ID, MOBILITY_RUNTIME1A_NAMED_GATE, MOBILITY_RUNTIME1A_RUNTIME_FIXTURE_GATE,
};
pub use mobility_scenario0::{
    admit_mobility_scenario0_packet, mobility_scenario0_packet, MobilityAllocationBounds,
    MobilityBlockadeSemantics, MobilityIdentityBoundary, MobilityIdentityChannelBudget,
    MobilityOwnerColumn, MobilityOwnerRelationDiscipline, MobilityOwnerRelationKind,
    MobilityQuantityClasses, MobilityRoutingMode, MobilityRoutingPolicy,
    MobilityScenario0Admission, MobilityScenario0GuardrailRequests, MobilityScenario0Packet,
    MobilityScenario0ParameterSummary, MobilityScenario0Status, MobilitySoakProfile,
    MobilitySupplyScope, MobilityTheaterScale, MobilityTheaterShape,
    MOBILITY_SCENARIO0_ENTITY_TARGET, MOBILITY_SCENARIO0_ID,
};
pub use preflight::{
    evaluate_designer_admission_request, resolve_frontier_artifact_target_id,
    DesignerAdmissionPreflightReport, DesignerAdmissionRequest, SeadLadderStage,
};
pub use preview::{
    preview_accepted_artifact_targets, preview_designer_admission_preflight,
    DesignerAdmissionPreviewReport,
};
pub use v7_8_line_scenarios::{
    admit_v7_8_line_scenario_pack, v7_8_met_consumer_scenario_pack, V78AtlasVramBudget,
    V78HardCurrencyContentionOrderingClaim, V78LineGateStatus, V78LineScenario,
    V78LineScenarioClaim, V78LineScenarioPack, V78LineScenarioPackAdmission,
    V78LineScenarioStatusRecord, V78MultiTheaterAtlasMappingClaim, V78NamedConsumerScenario,
    V78NestedResourceFlowDepthFanoutClaim, V78PromotedLine, V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES,
    V78_MET_SCENARIO_PACK_ID,
};
