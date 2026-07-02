//! CLAUSE-SPEC-0 — designer-authored FrontierV2 scenario admission tests.

use simthing_spec::{
    admit_clause_spec_frontier_v2_scenario, deserialize_clause_spec_frontier_v2_scenario_ron,
    preview_designer_admission_preflight, serialize_clause_spec_frontier_v2_scenario_ron,
    AcceptedFrontierArtifactTarget, ClauseSpecFrontierV2Scenario, ClauseSpecMovementFeedbackMode,
    ClauseSpecResourceFlowRoute, ClauseSpecStructuralFeedbackMode, DesignerAdmissionDiagnosticCode,
    MappingExecutionProfile, ResourceFlowExecutionProfile, ResourceFlowOptInMode,
};

const HAPPY_RON: &str = r#"(
    scenario_id: "clause_spec0_frontier_v2_happy",
    profile_name: "FrontierV2",
    enabled_by_default: false,
    grid: (rows: 8, cols: 8),
    closed_loop_ticks: 4,
    factions: [
        (faction_id: "faction_a", initial_seed: 32),
        (faction_id: "faction_b", initial_seed: 24),
    ],
    resource_flow: (
        opt_in: FlatStarOptIn,
        execution_profile: FlatStarResourceFlow,
        route: ResourceFlowAllocator,
        depth_cap: 2,
    ),
    mapping: (
        execution_profile: SparseRegionFieldV1,
        atlas: false,
        active_mask: false,
        perception: false,
        source_identity: false,
    ),
    movement_feedback: (
        mode: OwnColumnShadowOnly,
        allow_cross_entity_write: false,
        allow_production_write: false,
    ),
    structural_feedback: (
        mode: BoundaryRequestShadowOnly,
        allow_production_commitment: false,
    ),
    artifact_targets: (
        accepted_frontier_v2_fixture_artifacts: true,
        combined_feedback_fixture: true,
        own_column_shadow: true,
        boundary_request_shadow: true,
        resource_flow_allocator_route: true,
    ),
)"#;

fn happy() -> ClauseSpecFrontierV2Scenario {
    deserialize_clause_spec_frontier_v2_scenario_ron(HAPPY_RON).expect("happy RON parses")
}

fn assert_rejects_code(
    scenario: ClauseSpecFrontierV2Scenario,
    code: DesignerAdmissionDiagnosticCode,
) {
    let admission = admit_clause_spec_frontier_v2_scenario(&scenario);
    assert!(!admission.accepted, "expected rejection for {code:?}");
    assert!(
        admission.diagnostics.iter().any(|d| d.code == code),
        "missing {code:?}: {:?}",
        admission.diagnostics
    );
}

#[test]
fn clause_spec0_rejects_cpu_planner_urgency_commitment() {
    let mut planner = happy();
    planner.cpu_planner = true;
    assert_rejects_code(planner, DesignerAdmissionDiagnosticCode::CpuPlannerRejected);

    let mut urgency = happy();
    urgency.cpu_urgency = true;
    assert_rejects_code(urgency, DesignerAdmissionDiagnosticCode::CpuUrgencyRejected);

    let mut commitment = happy();
    commitment.cpu_commitment_emission = true;
    assert_rejects_code(
        commitment,
        DesignerAdmissionDiagnosticCode::CpuCommitmentEmissionRejected,
    );
}
