//! CLAUSE-SPEC-0 driver compile smoke.
//!
//! Proves the admitted simthing-spec metadata points at the existing accepted
//! FrontierV2 fixture support without opening production SimSession wiring.

#[path = "support/frontier_v2.rs"]
#[allow(dead_code)]
mod frontier_v2;

use frontier_v2::{
    frontier_v2_smoke_skeleton, validate_frontier_v2_admission,
    FRONTIER_V2_4_COMBINED_FEEDBACK_TICKS, FRONTIER_V2_4_FIXTURE_ID,
};
use simthing_spec::{
    admit_clause_spec_frontier_v2_scenario, deserialize_clause_spec_frontier_v2_scenario_ron,
    AcceptedFrontierArtifactTarget,
};

const DESIGNER_RON: &str = r#"(
    scenario_id: "clause_spec0_frontier_v2_driver_smoke",
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

#[test]
fn clause_spec0_driver_smoke_lowers_to_existing_frontier_v2_fixture_support() {
    let scenario =
        deserialize_clause_spec_frontier_v2_scenario_ron(DESIGNER_RON).expect("parse RON");
    let admission = admit_clause_spec_frontier_v2_scenario(&scenario);
    assert!(admission.accepted, "{:?}", admission.diagnostics);
    assert_eq!(
        admission.closed_loop_ticks,
        FRONTIER_V2_4_COMBINED_FEEDBACK_TICKS
    );
    assert!(admission
        .accepted_artifact_targets
        .contains(&AcceptedFrontierArtifactTarget::FrontierV2CombinedFeedbackFixture));
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

    let skeleton = frontier_v2_smoke_skeleton();
    assert!(validate_frontier_v2_admission(&skeleton).accepted);
    assert!(!skeleton.enabled_by_default);

    println!("clause_spec0_driver_smoke: fixture_id={FRONTIER_V2_4_FIXTURE_ID} metadata_only=true");
}
