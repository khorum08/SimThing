//! BH3-CLOSEOUT PR2 scenario-container grammar/lowering guardrails.

use simthing_clausething::{hydrate_scenario, parse_raw_document};
use simthing_core::{SimThingKind, TransformOp};
use simthing_spec::InstallTargetSpec;

const FIXTURE: &str = include_str!("fixtures/ct_scenario_container_minimal.clause");

fn hydrate_fixture() -> simthing_clausething::HydratedScenarioPack {
    let document = parse_raw_document(FIXTURE.as_bytes()).expect("parse scenario fixture");
    hydrate_scenario(&document).expect("hydrate scenario fixture")
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
fn link_route_path_and_predecessor_are_not_pr2_grammar() {
    for forbidden in [
        r#"link = { from = "alpha" to = "beta" }"#,
        r#"route = { from = "alpha" to = "beta" }"#,
        r#"path = { from = "alpha" to = "beta" }"#,
        r#"predecessor = "alpha""#,
    ] {
        let source = format!(
            r#"
scenario = forbidden_pr2_field {{
    location = alpha {{ name = "Alpha" }}
    {forbidden}
}}
"#
        );
        let document = parse_raw_document(source.as_bytes()).expect("parse forbidden scenario");
        let err = hydrate_scenario(&document).unwrap_err();
        assert!(
            err.to_string()
                .contains("outside PR2 scenario-container grammar"),
            "{err}"
        );
    }
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
