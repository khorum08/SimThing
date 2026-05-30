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
fn clause_spec0_happy_path_ron_scenario_admits() {
    let scenario = happy();
    let admission = admit_clause_spec_frontier_v2_scenario(&scenario);
    assert!(admission.accepted, "{:?}", admission.diagnostics);
    assert_eq!(admission.scenario_id, "clause_spec0_frontier_v2_happy");
    assert_eq!(admission.grid.rows, 8);
    assert_eq!(admission.closed_loop_ticks, 4);

    let serialized = serialize_clause_spec_frontier_v2_scenario_ron(&scenario).expect("serialize");
    let roundtrip =
        deserialize_clause_spec_frontier_v2_scenario_ron(&serialized).expect("roundtrip");
    assert_eq!(roundtrip, scenario);
}

#[test]
fn clause_spec0_happy_path_lowers_to_frontier_v2_artifact_targets() {
    let admission = admit_clause_spec_frontier_v2_scenario(&happy());
    assert!(admission.accepted);
    assert_eq!(admission.accepted_artifact_targets.len(), 5);
    assert!(admission
        .accepted_artifact_targets
        .contains(&AcceptedFrontierArtifactTarget::AcceptedFrontierV2FixtureArtifacts));
    assert!(admission
        .accepted_artifact_targets
        .contains(&AcceptedFrontierArtifactTarget::FrontierV2CombinedFeedbackFixture));
    assert!(admission
        .accepted_artifact_targets
        .contains(&AcceptedFrontierArtifactTarget::FrontierV2OwnColumnShadow));
    assert!(admission
        .accepted_artifact_targets
        .contains(&AcceptedFrontierArtifactTarget::FrontierV2BoundaryRequestShadow));
    assert!(admission
        .accepted_artifact_targets
        .contains(&AcceptedFrontierArtifactTarget::ResourceFlowAllocatorRoute));
    assert!(
        admission
            .lowering_summary
            .maps_to_frontier_v2_combined_feedback_fixture
    );
    assert_eq!(
        admission.lowering_summary.resource_route,
        AcceptedFrontierArtifactTarget::ResourceFlowAllocatorRoute
    );
    assert_eq!(
        admission.lowering_summary.movement,
        AcceptedFrontierArtifactTarget::FrontierV2OwnColumnShadow
    );
    assert_eq!(
        admission.lowering_summary.structural,
        AcceptedFrontierArtifactTarget::FrontierV2BoundaryRequestShadow
    );
    assert!(admission.lowering_summary.metadata_only);
}

#[test]
fn clause_spec0_uses_l1_preflight_preview() {
    let scenario = happy();
    let manifest = scenario.to_preflight_manifest();
    let preview = preview_designer_admission_preflight(&manifest);
    assert!(!preview.rejected);
    assert_eq!(preview.accepted_artifact_targets.len(), 5);

    let mut bypass = scenario;
    bypass.resource_flow.route = ClauseSpecResourceFlowRoute::ResourceFlowBypass;
    let preview = preview_designer_admission_preflight(&bypass.to_preflight_manifest());
    assert!(preview.rejected);
    assert!(preview
        .diagnostics
        .iter()
        .any(|d| d.code == DesignerAdmissionDiagnosticCode::ResourceFlowBypassRejected));
}

#[test]
fn clause_spec0_rejects_default_on() {
    let mut s = happy();
    s.enabled_by_default = true;
    assert_rejects_code(s, DesignerAdmissionDiagnosticCode::DefaultOnRejected);
}

#[test]
fn clause_spec0_rejects_non_frontier_v2_profile() {
    let mut s = happy();
    s.profile_name = "FrontierV1".into();
    assert_rejects_code(
        s,
        DesignerAdmissionDiagnosticCode::MalformedManifestRejected,
    );
}

#[test]
fn clause_spec0_rejects_grid_over_cap() {
    let mut s = happy();
    s.grid.rows = 33;
    assert_rejects_code(
        s,
        DesignerAdmissionDiagnosticCode::MalformedManifestRejected,
    );
}

#[test]
fn clause_spec0_rejects_closed_loop_ticks_under_two() {
    let mut s = happy();
    s.closed_loop_ticks = 1;
    assert_rejects_code(
        s,
        DesignerAdmissionDiagnosticCode::MalformedManifestRejected,
    );
}

#[test]
fn clause_spec0_rejects_resource_flow_bypass() {
    let mut s = happy();
    s.resource_flow.route = ClauseSpecResourceFlowRoute::ResourceFlowBypass;
    assert_rejects_code(
        s,
        DesignerAdmissionDiagnosticCode::ResourceFlowBypassRejected,
    );
}

#[test]
fn clause_spec0_rejects_nested_resource_flow_depth() {
    let mut s = happy();
    s.resource_flow.depth_cap = 3;
    assert_rejects_code(
        s,
        DesignerAdmissionDiagnosticCode::NestedE11BRequestedWithoutNamedScenario,
    );
}

#[test]
fn clause_spec0_rejects_cross_entity_movement_write() {
    let mut s = happy();
    s.movement_feedback.mode = ClauseSpecMovementFeedbackMode::CrossEntityWrite;
    s.movement_feedback.allow_cross_entity_write = true;
    assert_rejects_code(
        s,
        DesignerAdmissionDiagnosticCode::CrossEntityMovementWriteRejected,
    );
}

#[test]
fn clause_spec0_rejects_production_movement_write() {
    let mut s = happy();
    s.movement_feedback.allow_production_write = true;
    assert_rejects_code(
        s,
        DesignerAdmissionDiagnosticCode::ProductionMovementWriteRejected,
    );
}

#[test]
fn clause_spec0_rejects_production_commitment_emission() {
    let mut s = happy();
    s.structural_feedback.mode = ClauseSpecStructuralFeedbackMode::ProductionCommitment;
    assert_rejects_code(
        s,
        DesignerAdmissionDiagnosticCode::ProductionCommitmentEmissionRejected,
    );
}

#[test]
fn clause_spec0_rejects_shared_pool_tick_write() {
    let mut s = happy();
    s.resource_flow.shared_pool_tick_write = true;
    assert_rejects_code(
        s,
        DesignerAdmissionDiagnosticCode::SharedPoolTickWriteRejected,
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

#[test]
fn clause_spec0_rejects_semantic_wgsl_scheduler_cache() {
    let mut wgsl = happy();
    wgsl.mapping.semantic_wgsl = true;
    assert_rejects_code(
        wgsl,
        DesignerAdmissionDiagnosticCode::SemanticWgslRequestRejected,
    );

    let mut scheduler = happy();
    scheduler.scheduler_cache = true;
    assert_rejects_code(
        scheduler,
        DesignerAdmissionDiagnosticCode::SchedulerCacheRequestRejected,
    );
}

#[test]
fn clause_spec0_rejects_simthing_sim_semantic_state() {
    let mut s = happy();
    s.simthing_sim_semantic_state = true;
    assert_rejects_code(
        s,
        DesignerAdmissionDiagnosticCode::SimthingSimSemanticStateRequestRejected,
    );
}

#[test]
fn clause_spec0_rejects_atlas_active_mask_perception_source_identity() {
    let mut atlas = happy();
    atlas.mapping.atlas = true;
    assert_rejects_code(
        atlas,
        DesignerAdmissionDiagnosticCode::AtlasRequestedWithoutGate,
    );

    let mut active = happy();
    active.mapping.active_mask = true;
    assert_rejects_code(
        active,
        DesignerAdmissionDiagnosticCode::ActiveMaskRequestedWithoutGate,
    );

    let mut perception = happy();
    perception.mapping.perception = true;
    assert_rejects_code(
        perception,
        DesignerAdmissionDiagnosticCode::PerceptionFogRequestedWithoutGate,
    );

    let mut source = happy();
    source.mapping.source_identity = true;
    assert_rejects_code(
        source,
        DesignerAdmissionDiagnosticCode::SourceIdentityRequestedWithoutGate,
    );
}

#[test]
fn clause_spec0_rejects_lines_a_b_c_without_gate() {
    let mut line_a = happy();
    line_a.resource_flow.nested_e11b = true;
    assert_rejects_code(
        line_a,
        DesignerAdmissionDiagnosticCode::NestedE11BRequestedWithoutNamedScenario,
    );

    let mut line_b = happy();
    line_b.resource_flow.d2_hard_currency_allocator = true;
    assert_rejects_code(
        line_b,
        DesignerAdmissionDiagnosticCode::D2aRequestedWithoutNamedScenario,
    );

    let mut line_c = happy();
    line_c.mapping.atlas = true;
    assert_rejects_code(
        line_c,
        DesignerAdmissionDiagnosticCode::AtlasRequestedWithoutGate,
    );
}

#[test]
fn clause_spec0_rejects_frontier_v2_5() {
    let mut s = happy();
    s.frontier_v2_5 = true;
    assert_rejects_code(
        s,
        DesignerAdmissionDiagnosticCode::FrontierV2FiveRequestRejected,
    );
}

#[test]
fn clause_spec0_rejects_act_event_obs_pipe_ladder_reopen() {
    for mutate in [
        |s: &mut ClauseSpecFrontierV2Scenario| s.act_5 = true,
        |s: &mut ClauseSpecFrontierV2Scenario| s.event_3 = true,
        |s: &mut ClauseSpecFrontierV2Scenario| s.obs_5 = true,
        |s: &mut ClauseSpecFrontierV2Scenario| s.pipe_1 = true,
    ] {
        let mut s = happy();
        mutate(&mut s);
        assert_rejects_code(
            s,
            DesignerAdmissionDiagnosticCode::ActEventObsPipeLadderReopenRejected,
        );
    }
}

#[test]
fn clause_spec0_rejects_clause_script_and_clausething() {
    let mut script = happy();
    script.clause_script_parser = true;
    assert_rejects_code(
        script,
        DesignerAdmissionDiagnosticCode::ClauseScriptParserRequestParked,
    );

    let mut thing = happy();
    thing.clausething_runtime = true;
    assert_rejects_code(
        thing,
        DesignerAdmissionDiagnosticCode::ClauseThingRuntimeRequestParked,
    );
}

#[test]
fn clause_spec0_reports_unknown_artifact_target_with_specific_diagnostic() {
    let mut s = happy();
    s.artifact_targets
        .additional_targets
        .push("UnknownFrontierV2Target".into());
    assert_rejects_code(
        s,
        DesignerAdmissionDiagnosticCode::UnknownArtifactTargetRejected,
    );
}

#[test]
fn clause_spec0_reports_malformed_manifest_with_specific_diagnostic() {
    let mut s = happy();
    s.scenario_id.clear();
    assert_rejects_code(
        s,
        DesignerAdmissionDiagnosticCode::MalformedManifestRejected,
    );
}

#[test]
fn clause_spec0_no_runtime_wiring_or_simthing_sim_awareness() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    assert_eq!(
        ResourceFlowOptInMode::default(),
        ResourceFlowOptInMode::Disabled
    );
    assert_eq!(
        ResourceFlowExecutionProfile::default(),
        ResourceFlowExecutionProfile::DefaultDisabled
    );

    let cargo = std::fs::read_to_string(format!("{}/Cargo.toml", env!("CARGO_MANIFEST_DIR")))
        .expect("read Cargo.toml");
    let deps = cargo.split("[dev-dependencies]").next().unwrap_or(&cargo);
    assert!(!deps.contains("simthing-sim"));

    let source = std::fs::read_to_string(format!(
        "{}/src/designer_admission/clause_spec.rs",
        env!("CARGO_MANIFEST_DIR")
    ))
    .expect("read clause_spec.rs");
    for forbidden in [
        "use simthing_sim",
        "simthing_sim::",
        "simthing_gpu::",
        "GpuContext",
        "SimSession::open",
        "dispatch_workgroups",
    ] {
        assert!(
            !source.contains(forbidden),
            "forbidden runtime wiring: {forbidden}"
        );
    }
}

#[test]
fn clause_spec0_no_implementer_self_acceptance() {
    let source = std::fs::read_to_string(format!(
        "{}/src/designer_admission/clause_spec.rs",
        env!("CARGO_MANIFEST_DIR")
    ))
    .expect("read clause_spec.rs");
    for forbidden in [
        "Phase M closed",
        "Phase E closed",
        "M/E closed",
        "FrontierV2 accepted",
        "ClauseThing unblocked",
        "L2 accepted",
        "design authority accepts",
    ] {
        assert!(
            !source
                .to_ascii_lowercase()
                .contains(&forbidden.to_ascii_lowercase()),
            "source must not self-accept: {forbidden}"
        );
    }
}
