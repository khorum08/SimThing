//! BH-3-AUTHORING-0 — ClauseThing parse/lowering tests.

use simthing_clausething::{hydrate_field_operator_pack, parse_raw_document};
use simthing_spec::{MappingExecutionProfile, RegionFieldOperatorSpec, StressOperatorSpec};

const FIXTURE: &str = include_str!("fixtures/bh3_field_operator.clause");
const MISSING_U_SAT: &str = include_str!("fixtures/bh3_missing_u_sat.clause");
const INVALID_CHI: &str = include_str!("fixtures/bh3_invalid_chi.clause");

#[test]
fn bh3_authoring_parses_field_operator_block() {
    let document = parse_raw_document(FIXTURE.as_bytes()).expect("parse fixture");
    let pack = hydrate_field_operator_pack(&document).expect("hydrate fixture");
    assert_eq!(pack.game_mode.id, "simthing_bh3_field_operator");
    assert_eq!(pack.game_mode.display_name, "BH-3 Field Operator Authoring");
    assert_eq!(
        pack.game_mode.mapping_execution_profile,
        MappingExecutionProfile::Disabled
    );
    assert!(pack.game_mode.resource_flow.is_none());
}

#[test]
fn bh3_authoring_lowers_semantic_free_bounded_spec() {
    let document = parse_raw_document(FIXTURE.as_bytes()).expect("parse fixture");
    let pack = hydrate_field_operator_pack(&document).expect("hydrate fixture");
    assert_eq!(pack.game_mode.region_fields.len(), 1);
    let field = &pack.game_mode.region_fields[0];
    assert_eq!(field.grid_size, 10);
    assert_eq!(field.source_col, 0);
    assert_eq!(field.target_col, 0);
    assert!(matches!(
        field.operator,
        RegionFieldOperatorSpec::SaturatingFlux {
            u_sat,
            chi,
            choke_output_col: Some(2),
        } if (u_sat - 1.0).abs() < f32::EPSILON && (chi - 0.25).abs() < f32::EPSILON
    ));
    assert!(field.commitment.is_some());
    assert!(field.parent_formula.is_some());

    let w = pack
        .w_impedance_compose
        .as_ref()
        .expect("field_impedance lowered");
    assert_eq!(w.profiles.len(), 1);
    assert_eq!(w.profiles[0].output_w_col, 4);

    let stress = pack.stress_compose.as_ref().expect("field_stress lowered");
    assert_eq!(stress.profiles.len(), 1);
    assert!(matches!(
        stress.profiles[0].operator,
        StressOperatorSpec::Overlap
    ));
}

#[test]
fn bh3_authoring_rejects_missing_u_sat() {
    let document = parse_raw_document(MISSING_U_SAT.as_bytes()).expect("parse invalid fixture");
    let err = hydrate_field_operator_pack(&document).unwrap_err();
    assert!(err.message.contains("u_sat"));
}

#[test]
fn bh3_authoring_rejects_invalid_chi_literal() {
    let document = parse_raw_document(INVALID_CHI.as_bytes()).expect("parse invalid chi fixture");
    let pack = hydrate_field_operator_pack(&document).expect("hydrate for admission check");
    let err =
        simthing_spec::compile_region_field_preview(&pack.game_mode.region_fields[0]).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("chi") || msg.contains("SaturatingFlux"));
}
