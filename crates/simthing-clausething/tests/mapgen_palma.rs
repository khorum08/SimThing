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
fn palma_d_output_column_is_declared_and_bounded() {
    let authoring = default_palma();
    let palma = authoring.pack.palma_feedstock.as_ref().expect("palma");
    assert_eq!(palma.d_output_col, MAPGEN_PALMA_D_OUTPUT_COL);
    assert!(palma.d_output_col < palma.n_dims);
    assert_ne!(palma.d_output_col, palma.source_col);
    assert_ne!(palma.d_output_col, palma.w_output_col);
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
