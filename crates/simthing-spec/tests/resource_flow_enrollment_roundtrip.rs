//! E-2B — Resource Flow enrollment selector RON roundtrip.

use simthing_spec::{
    ArenaSpec, EnrollmentSelectorSpec, ExplicitParticipantSpec, FissionPolicySpec,
    InstallTargetSpec, PropertyKey, ResourceFlowSpec, deserialize_game_mode_ron,
};
use simthing_spec::GameModeSpec;

fn sample_arena_explicit_only() -> ArenaSpec {
    ArenaSpec {
        name: "food".into(),
        flow_property: PropertyKey::new("core", "food_flow"),
        balance_property: None,
        max_participants: 8,
        max_coupling_fanout: 4,
        max_orderband_depth: 16,
        fission_policy: FissionPolicySpec::Reject,
        reserved_orderband_depth: 0,
        reserved_gap_per_intermediate: 0,
        expected_max_children_per_intermediate: 0,
        explicit_participants: vec![ExplicitParticipantSpec::flat(1, 42)],
        enrollment: Some(EnrollmentSelectorSpec::ExplicitOnly),
        wildcard_admission: None,
    }
}

#[test]
fn resource_flow_enrollment_selector_roundtrips_ron() {
    let spec = GameModeSpec {
        id: "e2b_roundtrip".into(),
        display_name: String::new(),
        description: String::new(),
        spec_version: Default::default(),
        metadata: Default::default(),
        domain_packs: vec![],
        properties: vec![],
        overlays: vec![],
        capability_trees: vec![],
        events: vec![],
        resource_flow: Some(ResourceFlowSpec {
            arenas: vec![ArenaSpec {
                enrollment: Some(EnrollmentSelectorSpec::InstallTarget(
                    InstallTargetSpec::AllOfKind {
                        kind: "Cohort".into(),
                    },
                )),
                explicit_participants: vec![],
                ..sample_arena_explicit_only()
            }],
            couplings: vec![],
        ..Default::default()
        }),
        resource_economy: None,
        resource_flow_execution_profile: Default::default(),
    };

    let ron = ron::ser::to_string(&spec).expect("serialize game mode");
    let parsed = deserialize_game_mode_ron(&ron).expect("parse game mode");
    let arena = &parsed.resource_flow.as_ref().unwrap().arenas[0];
    assert_eq!(
        arena.enrollment,
        Some(EnrollmentSelectorSpec::InstallTarget(
            InstallTargetSpec::AllOfKind {
                kind: "Cohort".into(),
            },
        ))
    );
    assert!(arena.explicit_participants.is_empty());
}

#[test]
fn resource_flow_enrollment_explicit_only_preserves_existing_explicit_participants() {
    let arena = sample_arena_explicit_only();
    assert_eq!(
        arena.enrollment,
        Some(EnrollmentSelectorSpec::ExplicitOnly)
    );
    assert_eq!(arena.explicit_participants.len(), 1);
    assert_eq!(arena.explicit_participants[0].subtree_root_id, 42);
}
