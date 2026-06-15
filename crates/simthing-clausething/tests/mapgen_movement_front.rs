//! MapGen PR6 — Movement-Front L1/L2/L3 authoring tests.

use simthing_clausething::{
    MAPGEN_MF_COMMITMENT_ID, MAPGEN_MF_DEFAULT_HORIZON, MAPGEN_MF_FIELD_OPERATOR_ID,
    MAPGEN_MF_L2_REDUCTION_SCOPE, MAPGEN_MF_MAX_HORIZON, MAPGEN_MF_SOURCE_COL,
    MAPGEN_RF_SUPPRESSION_ARENA, MapGenLatticeOptions, MapGenMovementFrontOptions,
    MapGenResourceFlowOptions, assert_no_palma_feedstock, generate_default_mapgen_links_enrollment,
    generate_default_mapgen_movement_front_authoring, generate_mapgen_lattice_hierarchy,
    generate_mapgen_movement_front_authoring, generate_mapgen_resource_flow_enrollment,
    parse_mapgen_neutral_document, validate_l1_operator_locality, validate_options,
};
use simthing_core::SimThingKind;
use simthing_spec::{
    FIRST_SLICE_FIELD_URGENCY_COL, MappingExecutionProfile, RegionFieldOperatorSpec,
    compile_region_field_preview,
};

const RAW_FIXTURE: &str = include_str!("fixtures/mapgen/tiny_pentad_hub_slice_raw.clause");

fn default_links_enrollment() -> simthing_clausething::MapGenLinksEnrollment {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse raw fixture");
    generate_default_mapgen_links_enrollment(&neutral).expect("generate links")
}

fn default_authoring() -> simthing_clausething::MapGenMovementFrontAuthoring {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse raw fixture");
    generate_default_mapgen_movement_front_authoring(&neutral).expect("generate authoring")
}

#[test]
fn pr2_through_pr5_still_succeed_before_pr6() {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse");
    let hierarchy = generate_mapgen_lattice_hierarchy(&neutral, MapGenLatticeOptions::default())
        .expect("PR3 hierarchy");
    generate_mapgen_resource_flow_enrollment(&hierarchy, MapGenResourceFlowOptions::default())
        .expect("PR4 RF");
    default_links_enrollment();
}

#[test]
fn tiny_fixture_generates_movement_front_authoring() {
    let authoring = default_authoring();
    assert_eq!(authoring.expansion_report.l1_field_operator_count, 1);
    assert_eq!(authoring.pack.game_mode.region_fields.len(), 1);
    assert!(authoring.pack.commitment.is_some());
}

#[test]
fn l1_local_lattice_field_operator_feedstock_exists() {
    let authoring = default_authoring();
    let field = &authoring.pack.game_mode.region_fields[0];
    assert!(matches!(
        field.operator,
        RegionFieldOperatorSpec::SaturatingFlux {
            u_sat,
            chi,
            choke_output_col: Some(2),
        } if (u_sat - 1.0).abs() < f32::EPSILON && (chi - 0.25).abs() < f32::EPSILON
    ));
    assert_eq!(field.horizon, MAPGEN_MF_DEFAULT_HORIZON);
    assert!(!field.allow_extended_horizon);
    assert_eq!(field.grid_size, 7);
    assert!(field.pressure_binding.is_some());
    assert_eq!(
        field.pressure_binding.as_ref().unwrap().arena,
        MAPGEN_RF_SUPPRESSION_ARENA
    );
}

#[test]
fn l1_horizon_is_bounded_and_local() {
    let authoring = default_authoring();
    let field = &authoring.pack.game_mode.region_fields[0];
    assert!(field.horizon <= MAPGEN_MF_MAX_HORIZON);
    assert_eq!(authoring.expansion_report.l1_locality_bound, field.horizon);
    validate_l1_operator_locality(field).expect("locality guard");
}

#[test]
fn l2_reduction_feedstock_exists_without_widening_l1_horizon() {
    let authoring = default_authoring();
    let field = &authoring.pack.game_mode.region_fields[0];
    let reduction = field.reduction.as_ref().expect("L2 reduction");
    assert_eq!(reduction.child_slot_count, 49);
    assert_eq!(reduction.parent_slot, 49);
    assert_eq!(authoring.expansion_report.l2_reduction_count, 1);
    assert_eq!(
        authoring.expansion_report.l2_reduction_scope,
        MAPGEN_MF_L2_REDUCTION_SCOPE
    );
    assert_eq!(field.horizon, MAPGEN_MF_DEFAULT_HORIZON);
}

#[test]
fn l3_threshold_commitment_feedstock_exists() {
    let authoring = default_authoring();
    let commitment = authoring.pack.commitment.as_ref().expect("L3 commitment");
    assert_eq!(commitment.commitment_id, MAPGEN_MF_COMMITMENT_ID);
    assert_eq!(
        commitment.source_field_operator_id,
        MAPGEN_MF_FIELD_OPERATOR_ID
    );
    assert_eq!(commitment.commitment.threshold, 0.75);
    assert_eq!(commitment.commitment.event_kind, 7);
    assert_eq!(
        commitment.commitment.urgency_col,
        FIRST_SLICE_FIELD_URGENCY_COL
    );
    assert_eq!(authoring.expansion_report.l3_commitment_count, 1);
    assert_eq!(authoring.expansion_report.l3_thresholds, vec![0.75]);
}

#[test]
fn rf_pressure_binding_projects_suppression_arena_to_gridcells() {
    let authoring = default_authoring();
    let binding = authoring.pack.game_mode.region_fields[0]
        .pressure_binding
        .as_ref()
        .expect("pressure binding");
    assert_eq!(binding.placements.len(), 5);
    assert_eq!(
        authoring.expansion_report.rf_source_bindings,
        vec![format!("{MAPGEN_RF_SUPPRESSION_ARENA}::flow")]
    );
}

#[test]
fn generated_columns_include_source_choke_and_urgency() {
    let authoring = default_authoring();
    assert_eq!(
        authoring.expansion_report.generated_columns,
        vec![MAPGEN_MF_SOURCE_COL, 2, FIRST_SLICE_FIELD_URGENCY_COL]
    );
}

#[test]
fn generated_output_preserves_pr5_links_and_pr4_rf() {
    let authoring = default_authoring();
    assert_eq!(authoring.pack.grid_metadata.links.len(), 3);
    assert!(authoring.pack.game_mode.resource_flow.is_some());
    assert_eq!(authoring.pack.root.kind, SimThingKind::World);
}

#[test]
fn no_palma_or_runtime_compose_surfaces_are_generated() {
    let authoring = default_authoring();
    assert!(authoring.pack.palma_feedstock.is_none());
    assert!(authoring.pack.w_impedance_compose.is_none());
    assert!(authoring.pack.stress_compose.is_none());
    assert_no_palma_feedstock(&authoring.pack).expect("no PALMA guard");
}

#[test]
fn mapping_profile_remains_default_off() {
    let authoring = default_authoring();
    assert_eq!(
        authoring.pack.game_mode.mapping_execution_profile,
        MappingExecutionProfile::Disabled
    );
}

#[test]
fn region_field_admits_through_existing_preview_compiler() {
    let authoring = default_authoring();
    compile_region_field_preview(&authoring.pack.game_mode.region_fields[0])
        .expect("admit region field");
}

#[test]
fn expansion_report_declares_required_fields() {
    let authoring = default_authoring();
    let report = &authoring.expansion_report;
    assert_eq!(report.l1_horizon, MAPGEN_MF_DEFAULT_HORIZON);
    assert_eq!(report.forbidden_surface_count, 0);
    assert_eq!(
        report.unsafe_expansion_flags,
        vec!["l2_reduction_spans_full_lattice".to_string()]
    );
}

#[test]
fn convenience_default_pipeline_succeeds() {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse");
    generate_default_mapgen_movement_front_authoring(&neutral).expect("default pipeline");
}

#[test]
fn pr6_source_has_no_euclidean_adjacency_authority() {
    let source = include_str!("../src/mapgen_movement_front.rs");
    for forbidden in [
        "distance",
        "magnitude",
        "norm(",
        "sqrt",
        "length(",
        "normalize(",
        "hypot",
        "euclidean",
    ] {
        assert!(
            !source.contains(forbidden),
            "mapgen_movement_front.rs must not reference Euclidean authority `{forbidden}`"
        );
    }
}

#[test]
fn horizon_beyond_cap_is_rejected() {
    let links = default_links_enrollment();
    let err = generate_mapgen_movement_front_authoring(
        &links,
        MapGenMovementFrontOptions {
            horizon: MAPGEN_MF_MAX_HORIZON + 1,
            ..MapGenMovementFrontOptions::default()
        },
    )
    .unwrap_err();
    assert!(err.message.contains("horizon"));
}

#[test]
fn missing_commitment_threshold_is_rejected() {
    let err = validate_options(&MapGenMovementFrontOptions {
        threshold: f32::NAN,
        ..MapGenMovementFrontOptions::default()
    })
    .unwrap_err();
    assert!(err.message.contains("threshold"));
}

#[test]
fn missing_l1_horizon_cap_is_rejected() {
    let err = validate_options(&MapGenMovementFrontOptions {
        horizon: 0,
        ..MapGenMovementFrontOptions::default()
    })
    .unwrap_err();
    assert!(err.message.contains("horizon caps"));
}

#[test]
fn dense_global_diffusion_profile_is_rejected() {
    let authoring = default_authoring();
    let mut field = authoring.pack.game_mode.region_fields[0].clone();
    field.operator = RegionFieldOperatorSpec::SourceCappedNormalized;
    let err = validate_l1_operator_locality(&field).unwrap_err();
    assert!(err.message.contains("dense/global"));
}

#[test]
fn horizon_widening_flag_is_rejected() {
    let authoring = default_authoring();
    let mut field = authoring.pack.game_mode.region_fields[0].clone();
    field.allow_extended_horizon = true;
    let err = validate_l1_operator_locality(&field).unwrap_err();
    assert!(err.message.contains("horizon widening"));
}

#[test]
fn cpu_planner_vocabulary_is_rejected_in_generated_properties() {
    let authoring = default_authoring();
    for property in &authoring.pack.game_mode.properties {
        let haystack = format!(
            "{} {} {} {}",
            property.id, property.namespace, property.name, property.description
        );
        for forbidden in [
            "cpu_planner",
            "pathfinding",
            "predecessor",
            "movement_order",
        ] {
            assert!(
                !haystack.contains(forbidden),
                "generated property must not reference `{forbidden}`"
            );
        }
    }
}

#[test]
fn route_and_frontline_vocabulary_is_rejected_in_generated_game_mode_json() {
    let authoring = default_authoring();
    let json = serde_json::to_string(&authoring.pack.game_mode).expect("serialize game mode");
    for forbidden in ["route", "pathfinding", "predecessor", "border", "frontline"] {
        assert!(
            !json.contains(forbidden),
            "generated game mode must not reference `{forbidden}`"
        );
    }
}

#[test]
fn palma_feedstock_on_pack_is_rejected() {
    let mut authoring = default_authoring();
    authoring.pack.palma_feedstock = Some(simthing_clausething::HydratedScenarioPalmaFeedstock {
        feedstock_id: "early".into(),
        w_source_field_operator_id: MAPGEN_MF_FIELD_OPERATOR_ID.into(),
        w_output_col: 3,
        d_output_col: 4,
        grid_size: 3,
        n_dims: 6,
        source_col: 0,
        choke_output_col: Some(2),
    });
    let err = assert_no_palma_feedstock(&authoring.pack).unwrap_err();
    assert!(err.message.contains("PALMA"));
}
