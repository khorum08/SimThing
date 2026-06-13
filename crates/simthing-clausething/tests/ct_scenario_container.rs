//! BH3-CLOSEOUT PR2/PR3 scenario-container grammar/lowering guardrails.

use simthing_clausething::{
    HydratedScenarioGridPlacement, HydratedScenarioLink, hydrate_scenario, parse_raw_document,
};
use simthing_core::{SimThingKind, TransformOp};
use simthing_spec::InstallTargetSpec;

const FIXTURE: &str = include_str!("fixtures/ct_scenario_container_minimal.clause");
const LINK_FIXTURE: &str = include_str!("fixtures/ct_scenario_container_with_links.clause");

fn hydrate_fixture() -> simthing_clausething::HydratedScenarioPack {
    let document = parse_raw_document(FIXTURE.as_bytes()).expect("parse scenario fixture");
    hydrate_scenario(&document).expect("hydrate scenario fixture")
}

fn hydrate_link_fixture() -> simthing_clausething::HydratedScenarioPack {
    let document = parse_raw_document(LINK_FIXTURE.as_bytes()).expect("parse linked fixture");
    hydrate_scenario(&document).expect("hydrate linked fixture")
}

#[test]
fn minimal_multi_location_scenario_parses_and_lowers() {
    let pack = hydrate_fixture();

    assert_eq!(pack.scenario_id, "bh3_closeout_pr2_minimal");
    assert_eq!(
        pack.metadata.get("display_name").map(String::as_str),
        Some("BH3 Closeout PR2 Minimal Scenario")
    );
    assert_eq!(pack.root.kind, SimThingKind::World);
    assert_eq!(pack.root.children.len(), 3);
    assert!(
        pack.root
            .children
            .iter()
            .all(|child| child.kind == SimThingKind::Location)
    );

    let authored_ids: Vec<_> = pack
        .root_node
        .children
        .iter()
        .map(|node| node.id.as_str())
        .collect();
    assert_eq!(authored_ids, vec!["alpha", "beta", "gamma"]);
    assert_eq!(pack.game_mode.id, "bh3_closeout_pr2_minimal");
    assert_eq!(pack.game_mode.properties.len(), 3);
    assert_eq!(pack.game_mode.overlays.len(), 2);
    assert!(pack.grid_metadata.links.is_empty());
}

#[test]
fn location_properties_overlays_and_children_survive_lowering() {
    let pack = hydrate_fixture();
    let alpha = pack
        .root_node
        .children
        .iter()
        .find(|node| node.id == "alpha")
        .expect("alpha location");

    assert_eq!(alpha.display_name, "Alpha Basin");
    assert_eq!(alpha.properties[0].id, "alpha_pressure");
    assert_eq!(alpha.overlays[0].id, "alpha_pressure_bonus");
    assert_eq!(
        alpha.overlays[0].install,
        InstallTargetSpec::ScenarioListed {
            target_id: "alpha".into()
        }
    );
    assert_eq!(
        alpha.overlays[0].sub_field_deltas[0].1,
        TransformOp::Add(2.0)
    );
    assert_eq!(alpha.children.len(), 1);
    assert_eq!(alpha.children[0].id, "alpha_worker_band");
    assert_eq!(alpha.children[0].kind, SimThingKind::Cohort);
    assert_eq!(alpha.children[0].properties[0].id, "alpha_worker_capacity");

    let install_target = pack
        .install_targets
        .get("alpha")
        .expect("alpha install target");
    assert_eq!(install_target, &vec![alpha.simthing_id]);
    assert!(pack.install_targets.contains_key("alpha_worker_band"));
}

#[test]
fn scenario_links_lower_to_bounded_grid_metadata() {
    let pack = hydrate_link_fixture();

    assert_eq!(pack.scenario_id, "bh3_closeout_pr3_links");
    assert_eq!(pack.root.children.len(), 3);
    assert_eq!(pack.grid_metadata.grid_size, 2);
    assert_eq!(pack.grid_metadata.max_fanout, 4);
    assert_eq!(
        pack.grid_metadata.links,
        vec![HydratedScenarioLink {
            from: "alpha".into(),
            to: "beta".into()
        }]
    );
    assert_eq!(
        pack.grid_metadata.placements,
        vec![
            HydratedScenarioGridPlacement {
                location_id: "alpha".into(),
                target_id: "alpha".into(),
                row: 0,
                col: 0
            },
            HydratedScenarioGridPlacement {
                location_id: "beta".into(),
                target_id: "beta".into(),
                row: 0,
                col: 1
            },
            HydratedScenarioGridPlacement {
                location_id: "gamma".into(),
                target_id: "gamma".into(),
                row: 1,
                col: 0
            }
        ]
    );
    assert!(pack.install_targets.contains_key("alpha"));
    assert!(pack.install_targets.contains_key("beta"));

    let json = serde_json::to_string(&pack.game_mode).expect("serialize game mode");
    assert!(!json.contains("grid_metadata"));
    assert!(!json.contains("\"links\""));
}

#[test]
fn duplicate_and_reversed_links_are_canonicalized_deterministically() {
    let source = br#"
scenario = duplicate_links {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    link = { from = beta to = alpha }
    link = { from = alpha to = beta }
    link = { from = beta to = alpha }
}
"#;
    let document = parse_raw_document(source).expect("parse duplicate links scenario");
    let pack = hydrate_scenario(&document).expect("hydrate duplicate links scenario");

    assert_eq!(
        pack.grid_metadata.links,
        vec![HydratedScenarioLink {
            from: "alpha".into(),
            to: "beta".into()
        }]
    );
}

#[test]
fn link_unknown_endpoint_is_rejected() {
    let source = br#"
scenario = unknown_link_endpoint {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    link = { from = alpha to = delta }
}
"#;
    let document = parse_raw_document(source).expect("parse unknown endpoint scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(
        err.to_string()
            .contains("link endpoint `delta` is not a scenario location"),
        "{err}"
    );
}

#[test]
fn link_fanout_cap_is_rejected_before_any_topology_runtime_exists() {
    let source = br#"
scenario = too_many_links {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    location = gamma { name = "Gamma" }
    location = delta { name = "Delta" }
    location = epsilon { name = "Epsilon" }
    location = zeta { name = "Zeta" }
    link = { from = alpha to = beta }
    link = { from = alpha to = gamma }
    link = { from = alpha to = delta }
    link = { from = alpha to = epsilon }
    link = { from = alpha to = zeta }
}
"#;
    let document = parse_raw_document(source).expect("parse fanout scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(err.to_string().contains("above PR3 N4 cap"), "{err}");
}

#[test]
fn non_n4_links_are_rejected_instead_of_becoming_arbitrary_topology() {
    let source = br#"
scenario = diagonal_link {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    location = gamma { name = "Gamma" }
    location = delta { name = "Delta" }
    link = { from = alpha to = delta }
}
"#;
    let document = parse_raw_document(source).expect("parse diagonal link scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(
        err.to_string()
            .contains("outside PR3 row-major N4 grid adjacency"),
        "{err}"
    );
}

#[test]
fn game_mode_debug_json_is_stable_for_generic_surfaces() {
    let pack = hydrate_fixture();
    let json = serde_json::to_string(&pack.game_mode).expect("serialize game mode");
    assert!(json.contains("\"id\":\"bh3_closeout_pr2_minimal\""));
    assert!(json.contains("\"id\":\"alpha_pressure\""));
    assert!(json.contains("\"target_id\":\"alpha\""));
    assert!(!json.contains("link"));
    assert!(!json.contains("route"));
    assert!(!json.contains("path"));
    assert!(!json.contains("predecessor"));
}

#[test]
fn duplicate_location_ids_are_rejected() {
    let source = br#"
scenario = duplicate_location_ids {
    location = alpha { name = "Alpha" }
    location = alpha { name = "Alpha Again" }
}
"#;
    let document = parse_raw_document(source).expect("parse duplicate scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(
        err.to_string().contains("duplicate scenario node id"),
        "{err}"
    );
}

#[test]
fn route_path_movement_and_arbitrary_topology_are_not_pr3_grammar() {
    for forbidden in [
        r#"route = { from = "alpha" to = "beta" }"#,
        r#"path = { from = "alpha" to = "beta" }"#,
        r#"predecessor = "alpha""#,
        r#"movement = { from = "alpha" to = "beta" }"#,
        r#"border = "north""#,
        r#"frontline = "north""#,
        r#"pathfinding = yes"#,
        r#"arbitrary_graph = { node = "alpha" }"#,
        r#"non_grid_topology = yes"#,
    ] {
        let source = format!(
            r#"
scenario = forbidden_pr3_field {{
    location = alpha {{ name = "Alpha" }}
    {forbidden}
}}
"#
        );
        let document = parse_raw_document(source.as_bytes()).expect("parse forbidden scenario");
        let err = hydrate_scenario(&document).unwrap_err();
        assert!(
            err.to_string()
                .contains("outside PR3 scenario-container grammar"),
            "{err}"
        );
    }
}

#[test]
fn nested_link_is_rejected_so_links_remain_scenario_level_metadata() {
    let source = br#"
scenario = nested_link {
    location = alpha {
        name = "Alpha"
        link = { from = alpha to = beta }
    }
    location = beta { name = "Beta" }
}
"#;
    let document = parse_raw_document(source).expect("parse nested link scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(
        err.to_string()
            .contains("outside PR3 scenario-container grammar"),
        "{err}"
    );
}

#[test]
fn custom_or_deprecated_child_kinds_are_rejected() {
    let source = br#"
scenario = bad_child_kind {
    location = alpha {
        children = {
            child = custom_thing {
                kind = Custom
            }
        }
    }
}
"#;
    let document = parse_raw_document(source).expect("parse child kind scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(err.to_string().contains("not admitted for PR2"), "{err}");
}
