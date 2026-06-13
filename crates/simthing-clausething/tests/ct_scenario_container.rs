//! BH3-CLOSEOUT PR2/PR3/PR4/PR5/PR6 scenario-container grammar/lowering guardrails.

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
fn self_link_is_rejected_with_distinct_endpoint_error() {
    let source = br#"
scenario = self_link_rejected {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    link = { from = alpha to = alpha }
}
"#;
    let document = parse_raw_document(source).expect("parse self-link scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(
        err.to_string()
            .contains("link endpoints must be distinct scenario locations"),
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
fn scenario_field_operator_missing_u_sat_is_rejected() {
    let source = br#"
scenario = missing_u_sat {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    field_operator = alpha_choke_flux {
        grid_size = 10
        source_col = 0
        target_col = 0
        n_dims = 6
        saturating_flux = {
            chi = 0.25
            choke_output_col = 2
        }
    }
}
"#;
    let document = parse_raw_document(source).expect("parse missing u_sat scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(err.to_string().contains("u_sat"), "{err}");
}

#[test]
fn scenario_field_operator_chi_above_cfl_is_rejected() {
    let source = br#"
scenario = bad_chi {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    field_operator = alpha_choke_flux {
        grid_size = 10
        source_col = 0
        target_col = 0
        n_dims = 6
        saturating_flux = {
            u_sat = 1.0
            chi = 0.5
            choke_output_col = 2
        }
    }
}
"#;
    let document = parse_raw_document(source).expect("parse bad chi scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(
        err.to_string().contains("CFL") || err.to_string().contains("chi"),
        "{err}"
    );
}

#[test]
fn scenario_field_operator_non_finite_values_are_rejected() {
    let source = br#"
scenario = non_finite {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    field_operator = alpha_choke_flux {
        grid_size = 10
        source_col = 0
        target_col = 0
        n_dims = 6
        saturating_flux = {
            u_sat = NaN
            chi = 0.25
            choke_output_col = 2
        }
    }
}
"#;
    let document = parse_raw_document(source).expect("parse non-finite scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(err.to_string().contains("finite"), "{err}");
}

#[test]
fn scenario_field_operator_unknown_output_binding_is_rejected() {
    let source = br#"
scenario = bad_choke_col {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    field_operator = alpha_choke_flux {
        grid_size = 10
        source_col = 0
        target_col = 0
        n_dims = 6
        saturating_flux = {
            u_sat = 1.0
            chi = 0.25
            choke_output_col = 9
        }
    }
}
"#;
    let document = parse_raw_document(source).expect("parse bad choke col scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(
        err.to_string().contains("choke_output_col") || err.to_string().contains("out of range"),
        "{err}"
    );
}

#[test]
fn scenario_second_field_operator_is_rejected() {
    let source = br#"
scenario = two_field_ops {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    field_operator = first_flux {
        grid_size = 10
        source_col = 0
        target_col = 0
        n_dims = 6
        saturating_flux = {
            u_sat = 1.0
            chi = 0.25
            choke_output_col = 2
        }
    }
    field_operator = second_flux {
        grid_size = 10
        source_col = 0
        target_col = 0
        n_dims = 6
        saturating_flux = {
            u_sat = 1.0
            chi = 0.25
            choke_output_col = 2
        }
    }
}
"#;
    let document = parse_raw_document(source).expect("parse two field ops scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(
        err.to_string().contains("at most 1 field_operator"),
        "{err}"
    );
}

#[test]
fn scenario_field_operator_forbidden_service_vocabulary_is_rejected() {
    let source = br#"
scenario = forbidden_field_op {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    field_operator = alpha_choke_flux {
        grid_size = 10
        source_col = 0
        target_col = 0
        n_dims = 6
        border = north
        saturating_flux = {
            u_sat = 1.0
            chi = 0.25
            choke_output_col = 2
        }
    }
}
"#;
    let document = parse_raw_document(source).expect("parse forbidden field op scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(
        err.to_string()
            .contains("outside BH-3 field_operator grammar"),
        "{err}"
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
fn scenario_palma_feedstock_missing_w_source_is_rejected() {
    let source = br#"
scenario = missing_w_source {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    field_operator = alpha_choke_flux {
        grid_size = 10
        source_col = 0
        target_col = 0
        n_dims = 6
        saturating_flux = {
            u_sat = 1.0
            chi = 0.25
            choke_output_col = 2
        }
    }
    palma_feedstock = alpha_wd {
        w_output_col = 3
        d_output_col = 4
    }
}
"#;
    let document = parse_raw_document(source).expect("parse missing w_source scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(err.to_string().contains("w_source"), "{err}");
}

#[test]
fn scenario_palma_feedstock_missing_w_output_col_is_rejected() {
    let source = br#"
scenario = missing_w_output {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    field_operator = alpha_choke_flux {
        grid_size = 10
        source_col = 0
        target_col = 0
        n_dims = 6
        saturating_flux = {
            u_sat = 1.0
            chi = 0.25
            choke_output_col = 2
        }
    }
    palma_feedstock = alpha_wd {
        w_source = alpha_choke_flux
        d_output_col = 4
    }
}
"#;
    let document = parse_raw_document(source).expect("parse missing w_output scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(err.to_string().contains("w_output_col"), "{err}");
}

#[test]
fn scenario_palma_feedstock_missing_d_output_col_is_rejected() {
    let source = br#"
scenario = missing_d_output {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    field_operator = alpha_choke_flux {
        grid_size = 10
        source_col = 0
        target_col = 0
        n_dims = 6
        saturating_flux = {
            u_sat = 1.0
            chi = 0.25
            choke_output_col = 2
        }
    }
    palma_feedstock = alpha_wd {
        w_source = alpha_choke_flux
        w_output_col = 3
    }
}
"#;
    let document = parse_raw_document(source).expect("parse missing d_output scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(err.to_string().contains("d_output_col"), "{err}");
}

#[test]
fn scenario_palma_feedstock_unknown_w_source_is_rejected() {
    let source = br#"
scenario = unknown_w_source {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    field_operator = alpha_choke_flux {
        grid_size = 10
        source_col = 0
        target_col = 0
        n_dims = 6
        saturating_flux = {
            u_sat = 1.0
            chi = 0.25
            choke_output_col = 2
        }
    }
    palma_feedstock = alpha_wd {
        w_source = missing_flux
        w_output_col = 3
        d_output_col = 4
    }
}
"#;
    let document = parse_raw_document(source).expect("parse unknown w_source scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(
        err.to_string().contains("not a scenario field_operator id"),
        "{err}"
    );
}

#[test]
fn scenario_palma_feedstock_invalid_w_output_col_is_rejected() {
    let source = br#"
scenario = bad_w_output {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    field_operator = alpha_choke_flux {
        grid_size = 10
        source_col = 0
        target_col = 0
        n_dims = 6
        saturating_flux = {
            u_sat = 1.0
            chi = 0.25
            choke_output_col = 2
        }
    }
    palma_feedstock = alpha_wd {
        w_source = alpha_choke_flux
        w_output_col = 9
        d_output_col = 4
    }
}
"#;
    let document = parse_raw_document(source).expect("parse bad w_output scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(
        err.to_string().contains("w_output_col") && err.to_string().contains("out of range"),
        "{err}"
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
fn scenario_palma_feedstock_route_and_movement_vocabulary_is_rejected() {
    for forbidden in [
        r#"route = { from = alpha to = beta }"#,
        r#"path = alpha"#,
        r#"predecessor = alpha"#,
        r#"movement = yes"#,
        r#"movement_order = 1"#,
        r#"waypoint = alpha"#,
        r#"destination = beta"#,
        r#"pathfinding = yes"#,
        r#"border = north"#,
        r#"frontline = north"#,
        r#"arbitrary_graph = yes"#,
        r#"non_grid_topology = yes"#,
    ] {
        let source = format!(
            r#"
scenario = forbidden_palma {{
    location = alpha {{ name = "Alpha" }}
    location = beta {{ name = "Beta" }}
    field_operator = alpha_choke_flux {{
        grid_size = 10
        source_col = 0
        target_col = 0
        n_dims = 6
        saturating_flux = {{
            u_sat = 1.0
            chi = 0.25
            choke_output_col = 2
        }}
    }}
    palma_feedstock = alpha_wd {{
        w_source = alpha_choke_flux
        w_output_col = 3
        d_output_col = 4
        {forbidden}
    }}
}}
"#
        );
        let document = parse_raw_document(source.as_bytes()).expect("parse forbidden palma");
        let err = hydrate_scenario(&document).unwrap_err();
        assert!(
            err.to_string()
                .contains("outside PR5 palma_feedstock grammar"),
            "{err}"
        );
    }
}

#[test]
fn scenario_second_palma_feedstock_is_rejected() {
    let source = br#"
scenario = two_palma {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    field_operator = alpha_choke_flux {
        grid_size = 10
        source_col = 0
        target_col = 0
        n_dims = 6
        saturating_flux = {
            u_sat = 1.0
            chi = 0.25
            choke_output_col = 2
        }
    }
    palma_feedstock = first_wd {
        w_source = alpha_choke_flux
        w_output_col = 3
        d_output_col = 4
    }
    palma_feedstock = second_wd {
        w_source = alpha_choke_flux
        w_output_col = 5
        d_output_col = 4
    }
}
"#;
    let document = parse_raw_document(source).expect("parse two palma scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(
        err.to_string().contains("at most 1 palma_feedstock"),
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
fn scenario_commitment_missing_threshold_is_rejected() {
    let source = br#"
scenario = missing_threshold {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    field_operator = alpha_choke_flux {
        grid_size = 10
        source_col = 0
        target_col = 0
        n_dims = 6
        saturating_flux = {
            u_sat = 1.0
            chi = 0.25
            choke_output_col = 2
        }
    }
    commitment = stabilize_alpha {
        event_kind = 7
        field_urgency = {
            source = alpha_choke_flux
            column = 2
            weight = 1.0
        }
    }
}
"#;
    let document = parse_raw_document(source).expect("parse missing threshold scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(err.to_string().contains("threshold"), "{err}");
}

#[test]
fn scenario_commitment_non_finite_threshold_is_rejected() {
    let source = br#"
scenario = bad_threshold {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    field_operator = alpha_choke_flux {
        grid_size = 10
        source_col = 0
        target_col = 0
        n_dims = 6
        saturating_flux = {
            u_sat = 1.0
            chi = 0.25
            choke_output_col = 2
        }
    }
    commitment = stabilize_alpha {
        threshold = NaN
        event_kind = 7
        field_urgency = {
            source = alpha_choke_flux
            column = 2
            weight = 1.0
        }
    }
}
"#;
    let document = parse_raw_document(source).expect("parse bad threshold scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(err.to_string().contains("finite"), "{err}");
}

#[test]
fn scenario_commitment_unknown_source_is_rejected() {
    let source = br#"
scenario = unknown_source {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    field_operator = alpha_choke_flux {
        grid_size = 10
        source_col = 0
        target_col = 0
        n_dims = 6
        saturating_flux = {
            u_sat = 1.0
            chi = 0.25
            choke_output_col = 2
        }
    }
    commitment = stabilize_alpha {
        threshold = 0.75
        event_kind = 7
        field_urgency = {
            source = missing_flux
            column = 2
            weight = 1.0
        }
    }
}
"#;
    let document = parse_raw_document(source).expect("parse unknown source scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(
        err.to_string().contains("not a scenario field_operator id"),
        "{err}"
    );
}

#[test]
fn scenario_commitment_bad_column_is_rejected() {
    let source = br#"
scenario = bad_column {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    field_operator = alpha_choke_flux {
        grid_size = 10
        source_col = 0
        target_col = 0
        n_dims = 6
        saturating_flux = {
            u_sat = 1.0
            chi = 0.25
            choke_output_col = 2
        }
    }
    commitment = stabilize_alpha {
        threshold = 0.75
        event_kind = 7
        field_urgency = {
            source = alpha_choke_flux
            column = 9
            weight = 1.0
        }
    }
}
"#;
    let document = parse_raw_document(source).expect("parse bad column scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(
        err.to_string().contains("column") && err.to_string().contains("out of range"),
        "{err}"
    );
}

#[test]
fn scenario_commitment_non_finite_weight_is_rejected() {
    let source = br#"
scenario = bad_weight {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    field_operator = alpha_choke_flux {
        grid_size = 10
        source_col = 0
        target_col = 0
        n_dims = 6
        saturating_flux = {
            u_sat = 1.0
            chi = 0.25
            choke_output_col = 2
        }
    }
    commitment = stabilize_alpha {
        threshold = 0.75
        event_kind = 7
        field_urgency = {
            source = alpha_choke_flux
            column = 2
            weight = NaN
        }
    }
}
"#;
    let document = parse_raw_document(source).expect("parse bad weight scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(err.to_string().contains("finite"), "{err}");
}

#[test]
fn scenario_commitment_unknown_attach_overlay_is_rejected() {
    let source = br#"
scenario = unknown_overlay {
    location = alpha {
        name = "Alpha"
        overlays = {
            modifier = {
                id = "alpha_pressure_bonus"
                targets_property = "simthing::alpha_pressure"
                amount_add = 2
            }
        }
    }
    location = beta { name = "Beta" }
    field_operator = alpha_choke_flux {
        grid_size = 10
        source_col = 0
        target_col = 0
        n_dims = 6
        saturating_flux = {
            u_sat = 1.0
            chi = 0.25
            choke_output_col = 2
        }
    }
    commitment = stabilize_alpha {
        threshold = 0.75
        event_kind = 7
        field_urgency = {
            source = alpha_choke_flux
            column = 2
            weight = 1.0
        }
        effect = {
            attach_overlay = missing_overlay
            target = alpha
        }
    }
}
"#;
    let document = parse_raw_document(source).expect("parse unknown overlay scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(
        err.to_string().contains("attach_overlay") && err.to_string().contains("missing_overlay"),
        "{err}"
    );
}

#[test]
fn scenario_second_commitment_is_rejected() {
    let source = br#"
scenario = two_commitments {
    location = alpha { name = "Alpha" }
    location = beta { name = "Beta" }
    field_operator = alpha_choke_flux {
        grid_size = 10
        source_col = 0
        target_col = 0
        n_dims = 6
        saturating_flux = {
            u_sat = 1.0
            chi = 0.25
            choke_output_col = 2
        }
    }
    commitment = first_commit {
        threshold = 0.75
        event_kind = 7
        field_urgency = {
            source = alpha_choke_flux
            column = 2
            weight = 1.0
        }
    }
    commitment = second_commit {
        threshold = 0.5
        event_kind = 8
        field_urgency = {
            source = alpha_choke_flux
            column = 2
            weight = 1.0
        }
    }
}
"#;
    let document = parse_raw_document(source).expect("parse two commitment scenario");
    let err = hydrate_scenario(&document).unwrap_err();
    assert!(err.to_string().contains("at most 1 commitment"), "{err}");
}

#[test]
fn scenario_commitment_route_and_movement_vocabulary_is_rejected() {
    for forbidden in [
        r#"route = { from = alpha to = beta }"#,
        r#"path = alpha"#,
        r#"predecessor = alpha"#,
        r#"movement = yes"#,
        r#"movement_order = 1"#,
        r#"waypoint = alpha"#,
        r#"destination = beta"#,
        r#"pathfinding = yes"#,
        r#"border = north"#,
        r#"frontline = north"#,
        r#"arbitrary_graph = yes"#,
        r#"non_grid_topology = yes"#,
    ] {
        let source = format!(
            r#"
scenario = forbidden_commitment {{
    location = alpha {{ name = "Alpha" }}
    location = beta {{ name = "Beta" }}
    field_operator = alpha_choke_flux {{
        grid_size = 10
        source_col = 0
        target_col = 0
        n_dims = 6
        saturating_flux = {{
            u_sat = 1.0
            chi = 0.25
            choke_output_col = 2
        }}
    }}
    commitment = stabilize_alpha {{
        threshold = 0.75
        event_kind = 7
        field_urgency = {{
            source = alpha_choke_flux
            column = 2
            weight = 1.0
        }}
        {forbidden}
    }}
}}
"#
        );
        let document = parse_raw_document(source.as_bytes()).expect("parse forbidden commitment");
        let err = hydrate_scenario(&document).unwrap_err();
        assert!(
            err.to_string().contains("outside PR6 commitment grammar"),
            "{err}"
        );
    }
}
