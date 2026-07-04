//! BH3-CLOSEOUT PR2–PR7 scenario-container grammar/lowering guardrails (LIVE_GUARDRAIL battery).
//!
//! Focused closeout command: `cargo test -p simthing-clausething --test ct_scenario_container`.
//! Covers parse/lower for canonical sample, SaturatingFlux, PALMA W/D feedstock, FIELD_POLICY
//! commitment, bounded links/grid metadata, default-off posture, and semantic-free lowering.

use simthing_clausething::{
    HydratedScenarioGridPlacement, HydratedScenarioLink, hydrate_scenario, parse_raw_document,
};
use simthing_core::{SimThingKind, TransformOp};
use simthing_spec::compile_region_field_preview;
use simthing_spec::{
    FIRST_SLICE_FIELD_URGENCY_COL, InstallTargetSpec, MappingExecutionProfile,
    RegionFieldOperatorSpec,
};

const FIXTURE: &str = include_str!("fixtures/ct_scenario_container_minimal.clause");
const LINK_FIXTURE: &str = include_str!("fixtures/ct_scenario_container_with_links.clause");
const FIELD_OPERATOR_FIXTURE: &str =
    include_str!("fixtures/ct_scenario_container_with_field_operator.clause");
const PALMA_FEEDSTOCK_FIXTURE: &str =
    include_str!("fixtures/ct_scenario_container_with_palma_feedstock.clause");
const COMMITMENT_FIXTURE: &str =
    include_str!("fixtures/ct_scenario_container_with_commitment.clause");
const CANONICAL_SAMPLE_FIXTURE: &str = include_str!("fixtures/ct_bh3_closeout_sample.clause");

fn hydrate_fixture() -> simthing_clausething::HydratedScenarioPack {
    let document = parse_raw_document(FIXTURE.as_bytes()).expect("parse scenario fixture");
    hydrate_scenario(&document).expect("hydrate scenario fixture")
}

fn hydrate_link_fixture() -> simthing_clausething::HydratedScenarioPack {
    let document = parse_raw_document(LINK_FIXTURE.as_bytes()).expect("parse linked fixture");
    hydrate_scenario(&document).expect("hydrate linked fixture")
}

fn hydrate_field_operator_fixture() -> simthing_clausething::HydratedScenarioPack {
    let document =
        parse_raw_document(FIELD_OPERATOR_FIXTURE.as_bytes()).expect("parse field op fixture");
    hydrate_scenario(&document).expect("hydrate field op fixture")
}

fn hydrate_palma_feedstock_fixture() -> simthing_clausething::HydratedScenarioPack {
    let document = parse_raw_document(PALMA_FEEDSTOCK_FIXTURE.as_bytes())
        .expect("parse palma feedstock fixture");
    hydrate_scenario(&document).expect("hydrate palma feedstock fixture")
}

fn hydrate_commitment_fixture() -> simthing_clausething::HydratedScenarioPack {
    let document =
        parse_raw_document(COMMITMENT_FIXTURE.as_bytes()).expect("parse commitment fixture");
    hydrate_scenario(&document).expect("hydrate commitment fixture")
}

fn hydrate_canonical_sample_fixture() -> simthing_clausething::HydratedScenarioPack {
    let document =
        parse_raw_document(CANONICAL_SAMPLE_FIXTURE.as_bytes()).expect("parse canonical sample");
    hydrate_scenario(&document).expect("hydrate canonical sample")
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
fn scenario_with_field_operator_parses_and_lowers() {
    let pack = hydrate_field_operator_fixture();

    assert_eq!(pack.scenario_id, "bh3_closeout_pr4_field_op");
    assert_eq!(pack.root.children.len(), 2);
    assert_eq!(
        pack.grid_metadata.links,
        vec![HydratedScenarioLink {
            from: "alpha".into(),
            to: "beta".into()
        }]
    );
    assert_eq!(pack.game_mode.region_fields.len(), 1);
    assert_eq!(
        pack.game_mode.region_fields[0].name,
        "alpha_choke_flux_field"
    );
}

#[test]
fn scenario_field_operator_lowers_through_bh3_generic_surfaces() {
    let pack = hydrate_field_operator_fixture();
    let field = &pack.game_mode.region_fields[0];

    assert!(matches!(
        field.operator,
        RegionFieldOperatorSpec::SaturatingFlux {
            u_sat,
            chi,
            choke_output_col: Some(2),
        } if (u_sat - 1.0).abs() < f32::EPSILON && (chi - 0.25).abs() < f32::EPSILON
    ));
    assert!(pack.w_impedance_compose.is_none());
    assert!(pack.stress_compose.is_none());

    compile_region_field_preview(field).expect("admitted region field");
}

#[test]
fn scenario_field_operator_preserves_default_off_posture() {
    let pack = hydrate_field_operator_fixture();
    assert_eq!(
        pack.game_mode.mapping_execution_profile,
        MappingExecutionProfile::Disabled
    );
}

#[test]
fn scenario_with_palma_feedstock_parses_and_lowers() {
    let pack = hydrate_palma_feedstock_fixture();
    assert_eq!(pack.scenario_id, "bh3_closeout_pr5_palma");
    assert_eq!(pack.game_mode.region_fields.len(), 1);
    let palma = pack
        .palma_feedstock
        .as_ref()
        .expect("palma feedstock metadata");
    assert_eq!(palma.feedstock_id, "alpha_wd");
    assert_eq!(palma.w_source_field_operator_id, "alpha_choke_flux");
    assert_eq!(palma.w_output_col, 3);
    assert_eq!(palma.d_output_col, 4);
    assert_eq!(palma.n_dims, 6);
    assert_eq!(palma.choke_output_col, Some(2));
}

#[test]
fn scenario_palma_feedstock_lowers_without_runtime_semantics() {
    let pack = hydrate_palma_feedstock_fixture();
    let json = serde_json::to_string(&pack.game_mode).expect("serialize game mode");
    assert!(!json.contains("pathfinding"));
    assert!(!json.contains("predecessor"));
    assert!(!json.contains("movement"));
    assert_eq!(
        pack.game_mode.mapping_execution_profile,
        MappingExecutionProfile::Disabled
    );
}

#[test]
fn scenario_palma_feedstock_requires_field_operator() {
    let source = br#"
scenario = palma_without_field_op {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    palma_feedstock = alpha_wd {
        w_source = alpha_choke_flux
        w_output_col = 3
        d_output_col = 4
    }
}
"#;
    let document = parse_raw_document(source).expect("parse palma without field op scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(
        err.to_string()
            .contains("requires a scenario field_operator"),
        "{err}"
    );
}

#[test]
fn scenario_with_commitment_parses_and_lowers() {
    let pack = hydrate_commitment_fixture();
    assert_eq!(pack.scenario_id, "bh3_closeout_pr6_commitment");
    assert!(pack.palma_feedstock.is_some());
    let commitment = pack.commitment.as_ref().expect("commitment metadata");
    assert_eq!(commitment.commitment_id, "stabilize_alpha");
    assert_eq!(commitment.source_field_operator_id, "alpha_choke_flux");
    assert_eq!(commitment.field_urgency_column, Some(2));
    assert_eq!(commitment.commitment.threshold, 0.75);
    assert_eq!(commitment.commitment.event_kind, 7);
    assert_eq!(
        commitment.commitment.urgency_col,
        FIRST_SLICE_FIELD_URGENCY_COL
    );

    let field = &pack.game_mode.region_fields[0];
    assert!(field.parent_formula.is_some());
    assert!(field.reduction.is_some());
    let region_commitment = field.commitment.as_ref().expect("region field commitment");
    assert_eq!(region_commitment.threshold, 0.75);
    let effect = region_commitment
        .effect
        .as_ref()
        .expect("commitment effect");
    assert_eq!(effect.target_id, "alpha");
    assert_eq!(effect.targets_property, "simthing::alpha_pressure");
}

#[test]
fn scenario_commitment_lowers_without_runtime_semantics() {
    let pack = hydrate_commitment_fixture();
    let json = serde_json::to_string(&pack.game_mode).expect("serialize game mode");
    assert!(!json.contains("pathfinding"));
    assert!(!json.contains("cpu_planner"));
    assert!(!json.contains("movement"));
    assert_eq!(
        pack.game_mode.mapping_execution_profile,
        MappingExecutionProfile::Disabled
    );
}

#[test]
fn canonical_sample_parses_and_lowers() {
    let pack = hydrate_canonical_sample_fixture();
    assert_eq!(pack.scenario_id, "ct_bh3_closeout_sample");
    assert_eq!(
        pack.metadata.get("display_name").map(String::as_str),
        Some("BH3 Closeout Canonical Sample")
    );

    assert_eq!(pack.root_node.children.len(), 3);
    let location_ids: Vec<_> = pack
        .root_node
        .children
        .iter()
        .map(|node| node.id.as_str())
        .collect();
    assert_eq!(location_ids, vec!["alpha", "beta", "gamma"]);

    let alpha = pack
        .root_node
        .children
        .iter()
        .find(|node| node.id == "alpha")
        .expect("alpha");
    assert_eq!(alpha.properties.len(), 1);
    assert_eq!(alpha.overlays.len(), 1);
    assert_eq!(alpha.children.len(), 1);
    assert_eq!(alpha.children[0].kind, SimThingKind::Cohort);

    assert_eq!(pack.game_mode.properties.len(), 4);
    assert_eq!(pack.game_mode.overlays.len(), 2);
    assert_eq!(pack.grid_metadata.links.len(), 2);
    assert!(pack.grid_metadata.links.contains(&HydratedScenarioLink {
        from: "alpha".into(),
        to: "beta".into(),
    }));
    assert!(pack.grid_metadata.links.contains(&HydratedScenarioLink {
        from: "alpha".into(),
        to: "gamma".into(),
    }));

    assert_eq!(pack.game_mode.region_fields.len(), 1);
    let field = &pack.game_mode.region_fields[0];
    assert!(matches!(
        field.operator,
        RegionFieldOperatorSpec::SaturatingFlux { .. }
    ));
    assert!(field.parent_formula.is_some());
    assert!(field.reduction.is_some());
    assert!(field.commitment.is_some());

    let palma = pack.palma_feedstock.as_ref().expect("palma feedstock");
    assert_eq!(palma.feedstock_id, "alpha_wd");
    assert_eq!(palma.w_source_field_operator_id, "alpha_choke_flux");

    let commitment = pack.commitment.as_ref().expect("commitment metadata");
    assert_eq!(commitment.commitment_id, "stabilize_alpha");
    assert_eq!(commitment.commitment.threshold, 0.75);
    assert_eq!(
        commitment.commitment.urgency_col,
        FIRST_SLICE_FIELD_URGENCY_COL
    );
}

#[test]
fn canonical_sample_preserves_default_off_posture() {
    let pack = hydrate_canonical_sample_fixture();
    assert_eq!(
        pack.game_mode.mapping_execution_profile,
        MappingExecutionProfile::Disabled
    );
    let json = serde_json::to_string(&pack).expect("serialize scenario pack");
    assert!(!json.contains("\"enabled\":true"));
}

#[test]
fn canonical_sample_contains_no_movement_or_pathfinding_semantics() {
    let pack = hydrate_canonical_sample_fixture();
    let json = serde_json::to_string(&pack.game_mode).expect("serialize game mode");
    for forbidden in [
        "pathfinding",
        "predecessor",
        "movement_order",
        "waypoint",
        "destination",
        "frontline",
        "border",
        "arbitrary_graph",
        "non_grid_topology",
        "cpu_planner",
    ] {
        assert!(
            !json.contains(forbidden),
            "canonical sample must not contain `{forbidden}` semantics"
        );
    }
}
