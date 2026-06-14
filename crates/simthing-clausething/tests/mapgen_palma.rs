//! MapGen PR7 — PALMA W/D reach feedstock tests.

use simthing_clausething::{
    MAPGEN_MF_CHOKE_OUTPUT_COL, MAPGEN_MF_FIELD_OPERATOR_ID, MAPGEN_MF_SOURCE_COL,
    MAPGEN_PALMA_D_OUTPUT_COL, MAPGEN_PALMA_FEEDSTOCK_ID, MAPGEN_PALMA_W_OUTPUT_COL,
    MapGenLatticeOptions, MapGenPalmaOptions, MapGenResourceFlowOptions,
    build_palma_feedstock_from_region_field, build_w_impedance_compose_from_palma,
    generate_default_mapgen_links_enrollment, generate_default_mapgen_movement_front_authoring,
    generate_default_mapgen_palma_feedstock, generate_mapgen_lattice_hierarchy,
    generate_mapgen_palma_feedstock, generate_mapgen_resource_flow_enrollment,
    parse_mapgen_neutral_document,
};
use simthing_core::SimThingKind;
use simthing_spec::{
    MappingExecutionProfile, RegionFieldOperatorSpec, compile_w_impedance_compose_preview,
};

const RAW_FIXTURE: &str = include_str!("fixtures/mapgen/tiny_pentad_hub_slice_raw.clause");

fn default_movement_front() -> simthing_clausething::MapGenMovementFrontAuthoring {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse raw fixture");
    generate_default_mapgen_movement_front_authoring(&neutral).expect("generate PR6 authoring")
}

fn default_palma() -> simthing_clausething::MapGenPalmaFeedstockAuthoring {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse raw fixture");
    generate_default_mapgen_palma_feedstock(&neutral).expect("generate PR7 PALMA")
}

#[test]
fn pr2_through_pr6_still_succeed_before_pr7() {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse");
    let hierarchy = generate_mapgen_lattice_hierarchy(&neutral, MapGenLatticeOptions::default())
        .expect("PR3 hierarchy");
    generate_mapgen_resource_flow_enrollment(&hierarchy, MapGenResourceFlowOptions::default())
        .expect("PR4 RF");
    generate_default_mapgen_links_enrollment(&neutral).expect("PR5 links");
    default_movement_front();
}

#[test]
fn tiny_fixture_generates_palma_feedstock() {
    let authoring = default_palma();
    assert!(authoring.pack.palma_feedstock.is_some());
    assert!(authoring.pack.w_impedance_compose.is_some());
    assert_eq!(authoring.expansion_report.palma_feedstock_count, 1);
}

#[test]
fn palma_w_source_binds_to_pr6_field_and_choke_column() {
    let authoring = default_palma();
    let palma = authoring
        .pack
        .palma_feedstock
        .as_ref()
        .expect("palma feedstock");
    assert_eq!(palma.feedstock_id, MAPGEN_PALMA_FEEDSTOCK_ID);
    assert_eq!(
        palma.w_source_field_operator_id,
        MAPGEN_MF_FIELD_OPERATOR_ID
    );
    assert_eq!(palma.choke_output_col, Some(MAPGEN_MF_CHOKE_OUTPUT_COL));
    assert_eq!(
        authoring.expansion_report.w_source_column,
        MAPGEN_MF_CHOKE_OUTPUT_COL
    );
    assert_eq!(
        authoring.expansion_report.w_output_column,
        MAPGEN_PALMA_W_OUTPUT_COL
    );
    assert_eq!(
        authoring.expansion_report.d_output_column,
        MAPGEN_PALMA_D_OUTPUT_COL
    );
}

#[test]
fn palma_d_output_column_is_declared_and_bounded() {
    let authoring = default_palma();
    let palma = authoring.pack.palma_feedstock.as_ref().expect("palma");
    assert_eq!(palma.d_output_col, MAPGEN_PALMA_D_OUTPUT_COL);
    assert!(palma.d_output_col < palma.n_dims);
    assert_ne!(palma.d_output_col, palma.source_col);
    assert_ne!(palma.d_output_col, palma.w_output_col);
}

#[test]
fn w_impedance_compose_admits_from_palma_feedstock() {
    let authoring = default_palma();
    let palma = authoring.pack.palma_feedstock.as_ref().expect("palma");
    let w_spec = build_w_impedance_compose_from_palma(palma);
    compile_w_impedance_compose_preview(&w_spec).expect("w compose admission");
    assert_eq!(w_spec.profiles[0].output_w_col, MAPGEN_PALMA_W_OUTPUT_COL);
    assert_eq!(w_spec.choke_a_col, MAPGEN_MF_CHOKE_OUTPUT_COL);
    assert_eq!(w_spec.base_w_col, MAPGEN_MF_SOURCE_COL);
}

#[test]
fn palma_feedstock_remains_default_off_authoring_only() {
    let authoring = default_palma();
    assert_eq!(
        authoring.pack.game_mode.mapping_execution_profile,
        MappingExecutionProfile::Disabled
    );
    assert_eq!(authoring.expansion_report.default_off_status, true);
    assert_eq!(authoring.expansion_report.route_surface_count, 0);
    assert_eq!(authoring.expansion_report.predecessor_surface_count, 0);
}

#[test]
fn palma_output_is_field_feedstock_not_route_or_movement() {
    let authoring = default_palma();
    let json = serde_json::to_string(&authoring.pack.game_mode).expect("serialize game mode");
    for forbidden in [
        "pathfinding",
        "predecessor",
        "movement_order",
        "destination",
        "route",
    ] {
        assert!(
            !json.contains(forbidden),
            "generated game mode must not reference `{forbidden}`"
        );
    }
}

#[test]
fn generated_output_preserves_pr6_movement_front_and_prior_rungs() {
    let authoring = default_palma();
    assert_eq!(authoring.pack.game_mode.region_fields.len(), 1);
    assert!(authoring.pack.commitment.is_some());
    assert_eq!(authoring.pack.grid_metadata.links.len(), 3);
    assert!(authoring.pack.game_mode.resource_flow.is_some());
    assert_eq!(authoring.pack.root.kind, SimThingKind::World);
}

#[test]
fn no_stress_compose_or_runtime_surfaces_are_generated() {
    let authoring = default_palma();
    assert!(authoring.pack.stress_compose.is_none());
}

#[test]
fn expansion_report_declares_required_fields() {
    let authoring = default_palma();
    let report = &authoring.expansion_report;
    assert_eq!(report.grid_size, 3);
    assert_eq!(report.n_dims, 6);
    assert_eq!(report.source_col, MAPGEN_MF_SOURCE_COL);
    assert_eq!(report.choke_output_col, MAPGEN_MF_CHOKE_OUTPUT_COL);
    assert!(report.unsafe_expansion_flags.is_empty());
}

#[test]
fn convenience_default_pipeline_succeeds() {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse");
    generate_default_mapgen_palma_feedstock(&neutral).expect("default pipeline");
}

#[test]
fn pr7_source_has_no_euclidean_adjacency_authority() {
    let source = include_str!("../src/mapgen_palma.rs");
    for forbidden in [
        "distance",
        "magnitude",
        "norm(",
        "sqrt",
        "length(",
        "normalize(",
        "hypot",
        "euclidean",
        "nearest",
    ] {
        assert!(
            !source.contains(forbidden),
            "mapgen_palma.rs must not reference Euclidean authority `{forbidden}`"
        );
    }
}

#[test]
fn unknown_w_source_is_rejected() {
    let movement_front = default_movement_front();
    let err = generate_mapgen_palma_feedstock(
        &movement_front,
        MapGenPalmaOptions {
            w_source_field_operator_id: "unknown_operator",
            ..MapGenPalmaOptions::default()
        },
    )
    .unwrap_err();
    assert!(err.message.contains("w_source"));
}

#[test]
fn d_output_column_out_of_bounds_is_rejected() {
    let movement_front = default_movement_front();
    let err = generate_mapgen_palma_feedstock(
        &movement_front,
        MapGenPalmaOptions {
            d_output_col: MAPGEN_MF_SOURCE_COL,
            ..MapGenPalmaOptions::default()
        },
    )
    .unwrap_err();
    assert!(err.message.contains("d_output_col"));
}

#[test]
fn missing_d_output_column_via_admission_is_rejected() {
    let movement_front = default_movement_front();
    let field = &movement_front.pack.game_mode.region_fields[0];
    let err = build_palma_feedstock_from_region_field(
        MAPGEN_PALMA_FEEDSTOCK_ID,
        MAPGEN_MF_FIELD_OPERATOR_ID,
        MAPGEN_PALMA_W_OUTPUT_COL,
        MAPGEN_PALMA_W_OUTPUT_COL,
        field,
    )
    .unwrap_err();
    assert!(err.to_string().contains("must differ"));
}

#[test]
fn palma_without_pr6_movement_front_is_rejected() {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse");
    let links = generate_default_mapgen_links_enrollment(&neutral).expect("links");
    let err = generate_mapgen_palma_feedstock(
        &simthing_clausething::MapGenMovementFrontAuthoring {
            pack: links.pack,
            expansion_report: simthing_clausething::MapGenMovementFrontAuthoringReport {
                l1_field_operator_count: 0,
                l1_horizon: 0,
                l1_locality_bound: 0,
                l2_reduction_count: 0,
                l2_reduction_scope: String::new(),
                l3_commitment_count: 0,
                l3_thresholds: Vec::new(),
                generated_columns: Vec::new(),
                rf_source_bindings: Vec::new(),
                forbidden_surface_count: 0,
                unsafe_expansion_flags: Vec::new(),
            },
        },
        MapGenPalmaOptions::default(),
    )
    .unwrap_err();
    assert!(err.message.contains("PR6 Movement-Front"));
}

#[test]
fn route_and_predecessor_vocabulary_is_rejected_in_generated_properties() {
    let authoring = default_palma();
    for property in &authoring.pack.game_mode.properties {
        let haystack = format!(
            "{} {} {} {}",
            property.id, property.namespace, property.name, property.description
        );
        for forbidden in ["route", "predecessor", "pathfinding", "movement_order"] {
            assert!(
                !haystack.contains(forbidden),
                "generated property must not reference `{forbidden}`"
            );
        }
    }
}

#[test]
fn render_positions_are_not_palma_w_d_authority() {
    let authoring = default_palma();
    let palma = authoring.pack.palma_feedstock.as_ref().expect("palma");
    assert_eq!(palma.source_col, MAPGEN_MF_SOURCE_COL);
    assert_eq!(palma.choke_output_col, Some(MAPGEN_MF_CHOKE_OUTPUT_COL));
    for property in &authoring.pack.game_mode.properties {
        if property.namespace == "mapgen" && property.name.starts_with("render_") {
            assert_ne!(
                palma.w_output_col.to_string(),
                property.description,
                "PALMA W column must not bind to inert render metadata"
            );
            assert_ne!(
                palma.d_output_col.to_string(),
                property.description,
                "PALMA D column must not bind to inert render metadata"
            );
        }
    }
}

#[test]
fn pr6_region_field_must_be_saturating_flux_for_palma() {
    let mut movement_front = default_movement_front();
    movement_front.pack.game_mode.region_fields[0].operator =
        RegionFieldOperatorSpec::SourceCappedNormalized;
    let err = generate_mapgen_palma_feedstock(&movement_front, MapGenPalmaOptions::default())
        .unwrap_err();
    assert!(err.message.contains("SaturatingFlux"));
}
