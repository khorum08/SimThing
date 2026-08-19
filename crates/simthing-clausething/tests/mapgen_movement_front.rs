//! MapGen PR6 — Movement-Front L1/L2/L3 authoring tests.

use simthing_clausething::{
    assert_no_palma_feedstock, generate_default_mapgen_links_enrollment,
    generate_default_mapgen_movement_front_authoring, generate_mapgen_lattice_hierarchy,
    generate_mapgen_movement_front_authoring, generate_mapgen_resource_flow_enrollment,
    parse_mapgen_neutral_document, validate_l1_operator_locality, validate_options,
    MapGenLatticeOptions, MapGenMovementFrontOptions, MapGenResourceFlowOptions,
    MAPGEN_MF_COMMITMENT_ID, MAPGEN_MF_DEFAULT_HORIZON, MAPGEN_MF_FIELD_OPERATOR_ID,
    MAPGEN_MF_L2_REDUCTION_SCOPE, MAPGEN_MF_MAX_HORIZON, MAPGEN_MF_SOURCE_COL,
    MAPGEN_RF_SUPPRESSION_ARENA,
};
use simthing_core::SimThingKind;
use simthing_spec::{
    compile_region_field_preview, MappingExecutionProfile, RegionFieldOperatorSpec,
    FIRST_SLICE_FIELD_URGENCY_COL,
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
fn l1_horizon_is_bounded_and_local() {
    let authoring = default_authoring();
    let field = &authoring.pack.game_mode.region_fields[0];
    assert!(field.horizon <= MAPGEN_MF_MAX_HORIZON);
    assert_eq!(authoring.expansion_report.l1_locality_bound, field.horizon);
    validate_l1_operator_locality(field).expect("locality guard");
}
