use simthing_spec::{
    deserialize_capability_tree_ron, deserialize_game_mode_ron, validate_capability_tree,
    ActivationMode, DisplayMeta, DomainPackSpec, GameModeSpec, InstallTargetSpec, SpecVersion,
};

#[test]
fn loads_minimal_capability_tree_ron() {
    let text = include_str!("fixtures/minimal_tech_tree.ron");
    let spec = deserialize_capability_tree_ron(text).expect("parse capability tree");
    assert_eq!(spec.tree_id, "minimal_tech");
    assert_eq!(spec.categories[0].entries[0].id, "chemical_drive");
}

#[test]
fn loads_minimal_game_mode_ron() {
    let text = include_str!("fixtures/minimal_game_mode.ron");
    let spec = deserialize_game_mode_ron(text).expect("parse game mode");
    assert_eq!(spec.id, "terran_campaign");
    assert_eq!(spec.domain_packs.len(), 1);
    assert_eq!(
        spec.domain_packs[0].capability_trees[0].tree_id,
        "terran_tech"
    );
}

#[test]
fn loads_install_target_examples() {
    let all_factions = include_str!("../../../docs/examples/game_mode_install_all_factions.ron");
    let listed = include_str!("../../../docs/examples/game_mode_install_scenario_listed.ron");
    let session_root = include_str!("../../../docs/examples/game_mode_install_session_root.ron");

    let all = deserialize_game_mode_ron(all_factions).expect("all factions example");
    assert_eq!(
        all.capability_trees[0].install,
        InstallTargetSpec::AllOfKind {
            kind: "Faction".into()
        }
    );

    let listed = deserialize_game_mode_ron(listed).expect("scenario listed example");
    assert_eq!(
        listed.capability_trees[0].install,
        InstallTargetSpec::ScenarioListed {
            target_id: "player_faction".into()
        }
    );

    let root = deserialize_game_mode_ron(session_root).expect("session root example");
    assert_eq!(
        root.capability_trees[0].install,
        InstallTargetSpec::SessionRoot
    );
}

#[test]
fn game_mode_round_trips_with_metadata() {
    let spec = GameModeSpec {
        id: "test_mode".into(),
        display_name: "Test Mode".into(),
        description: "desc".into(),
        spec_version: SpecVersion {
            major: 1,
            minor: 2,
            patch: 3,
        },
        metadata: DisplayMeta {
            description: "meta".into(),
            icon: Some("icon.png".into()),
            tags: vec!["a".into(), "b".into()],
        },
        domain_packs: vec![],
        properties: vec![],
        overlays: vec![],
        capability_trees: vec![],
        events: vec![],
        resource_flow: None,
        resource_economy: None,
        resource_flow_execution_profile: Default::default(),
        region_fields: vec![],
        mapping_execution_profile: Default::default(),
    };

    let json = serde_json::to_string(&spec).unwrap();
    let round: GameModeSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(round.metadata.tags, vec!["a", "b"]);
    assert_eq!(round.spec_version.minor, 2);
}

#[test]
fn domain_pack_round_trips() {
    let pack = DomainPackSpec {
        id: "pack".into(),
        display_name: "Pack".into(),
        metadata: DisplayMeta::default(),
        properties: vec![],
        overlays: vec![],
        capability_trees: vec![],
        events: vec![],
    };
    let json = serde_json::to_string(&pack).unwrap();
    let round: DomainPackSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(round.id, "pack");
}

#[test]
fn activation_mode_deserializes_threshold_and_player_selection() {
    let threshold: ActivationMode = serde_json::from_str("\"Threshold\"").unwrap();
    let selection: ActivationMode = serde_json::from_str("\"PlayerSelection\"").unwrap();
    assert_eq!(threshold, ActivationMode::Threshold);
    assert_eq!(selection, ActivationMode::PlayerSelection);
}

#[test]
fn validate_capability_tree_from_fixture() {
    let text = include_str!("fixtures/minimal_tech_tree.ron");
    let spec = deserialize_capability_tree_ron(text).unwrap();
    let diagnostics = validate_capability_tree(&spec).unwrap();
    assert!(diagnostics.diagnostics.is_empty());
}
