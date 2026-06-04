//! RF-T1 — Resource Flow execution opt-in RON roundtrip.

use simthing_spec::{
    deserialize_game_mode_ron, ArenaSpec, FissionPolicySpec, GameModeSpec, PropertyKey,
    ResourceFlowExecutionProfile, ResourceFlowOptInMode, ResourceFlowSpec,
};

fn sample_arena() -> ArenaSpec {
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
        explicit_participants: vec![],
        enrollment: None,
        wildcard_admission: None,
    }
}

#[test]
fn resource_flow_opt_in_mode_roundtrips_ron() {
    let spec = GameModeSpec {
        id: "rf_t1_roundtrip".into(),
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
            opt_in_mode: ResourceFlowOptInMode::FlatStarOptIn,
            arenas: vec![sample_arena()],
            couplings: vec![],
        }),
        resource_economy: None,
        ..Default::default()
    };

    let ron = ron::ser::to_string(&spec).expect("serialize game mode");
    let parsed = deserialize_game_mode_ron(&ron).expect("parse game mode");
    assert_eq!(
        parsed.resource_flow.as_ref().unwrap().opt_in_mode,
        ResourceFlowOptInMode::FlatStarOptIn
    );
}

#[test]
fn resource_flow_opt_in_default_disabled() {
    let text = r#"(
        opt_in_mode: Disabled,
        arenas: [],
        couplings: [],
    )"#;
    let spec: ResourceFlowSpec = ron::from_str(text).expect("parse partial flow spec");
    assert_eq!(spec.opt_in_mode, ResourceFlowOptInMode::Disabled);

    let text_missing = r#"(
        arenas: [],
        couplings: [],
    )"#;
    let spec_missing: ResourceFlowSpec = ron::from_str(text_missing).expect("parse without opt_in");
    assert_eq!(spec_missing.opt_in_mode, ResourceFlowOptInMode::Disabled);
}

#[test]
fn resource_flow_execution_profile_roundtrips_ron() {
    let spec = GameModeSpec {
        id: "rf_t4_roundtrip".into(),
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
            opt_in_mode: ResourceFlowOptInMode::Disabled,
            arenas: vec![sample_arena()],
            couplings: vec![],
        }),
        resource_economy: None,
        resource_flow_execution_profile: ResourceFlowExecutionProfile::FlatStarResourceFlow,
        region_fields: vec![],
        mapping_execution_profile: Default::default(),
    };

    let ron = ron::ser::to_string(&spec).expect("serialize game mode");
    let parsed = deserialize_game_mode_ron(&ron).expect("parse game mode");
    assert_eq!(
        parsed.resource_flow_execution_profile,
        ResourceFlowExecutionProfile::FlatStarResourceFlow
    );
}
