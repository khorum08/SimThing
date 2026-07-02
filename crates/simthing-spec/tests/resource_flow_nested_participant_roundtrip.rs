//! E-11B — nested explicit participant `parent_subtree_root_id` RON/serde smoke.

use simthing_spec::{
    deserialize_game_mode_ron, ArenaSpec, ExplicitParticipantSpec, FissionPolicySpec, GameModeSpec,
    PropertyKey, ResourceFlowSpec,
};

fn sample_arena_with_nested() -> ArenaSpec {
    ArenaSpec {
        name: "food".into(),
        flow_property: PropertyKey::new("core", "food_flow"),
        balance_property: None,
        max_participants: 16,
        max_coupling_fanout: 4,
        max_orderband_depth: 16,
        fission_policy: FissionPolicySpec::Reject,
        reserved_orderband_depth: 0,
        reserved_gap_per_intermediate: 0,
        expected_max_children_per_intermediate: 0,
        explicit_participants: vec![
            ExplicitParticipantSpec::flat(1, 100),
            ExplicitParticipantSpec::nested(2, 101, 100),
            ExplicitParticipantSpec::nested(3, 102, 101),
            ExplicitParticipantSpec::nested(4, 103, 101),
        ],
        enrollment: None,
        wildcard_admission: None,
    }
}

#[test]
fn resource_flow_nested_participant_parent_field_roundtrips_ron() {
    let spec = GameModeSpec {
        id: "e11b_nested_parent_roundtrip".into(),
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
            arenas: vec![sample_arena_with_nested()],
            couplings: vec![],
            ..Default::default()
        }),
        resource_economy: None,
        ..Default::default()
    };

    let ron = ron::ser::to_string(&spec).expect("serialize game mode");
    assert!(ron.contains("parent_subtree_root_id"));

    let parsed = deserialize_game_mode_ron(&ron).expect("parse game mode");
    let participants =
        &parsed.resource_flow.as_ref().expect("resource flow").arenas[0].explicit_participants;

    assert_eq!(participants.len(), 4);
    assert_eq!(participants[0].parent_subtree_root_id, None);
    assert_eq!(participants[1].parent_subtree_root_id, Some(100));
    assert_eq!(participants[2].parent_subtree_root_id, Some(101));
    assert_eq!(participants[3].parent_subtree_root_id, Some(101));

    let reserialized = ron::ser::to_string(&parsed).expect("reserialize");
    let reparsed = deserialize_game_mode_ron(&reserialized).expect("reparse");
    let reparsed_participants =
        &reparsed.resource_flow.as_ref().unwrap().arenas[0].explicit_participants;
    assert_eq!(reparsed_participants, participants);
}
