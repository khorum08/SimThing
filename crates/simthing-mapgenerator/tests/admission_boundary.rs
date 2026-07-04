//! Lower-boundary admission representatives for mapgenerator Tier 2 promotion targets.
//! Direct parser/validator checks without galaxy generation, emission, or runtime integration.

use simthing_mapgenerator::{
    parse_shape_param_assignment, validate_special_route_edges, LatticeCoord, MapGeneratorParams,
    PlacedSystemSeed, ShapeParamParseError, ShapePlacement, ShapeRegistry, SpecialRouteEdge,
    SpecialRouteError, SpecialRouteKind, ValidationError,
};

fn registry() -> ShapeRegistry {
    ShapeRegistry::default()
}

fn two_system_placement() -> ShapePlacement {
    ShapePlacement {
        systems: vec![
            PlacedSystemSeed {
                id: 0,
                coord: LatticeCoord { col: 0, row: 0 },
                bucket: None,
            },
            PlacedSystemSeed {
                id: 1,
                coord: LatticeCoord { col: 1, row: 0 },
                bucket: None,
            },
        ],
    }
}

#[test]
fn admission_hard_error_boundary_rejects_infinite_shape_param_token() {
    let err = parse_shape_param_assignment("jitter=inf").unwrap_err();
    assert!(matches!(err, ShapeParamParseError::NonNumeric { .. }));
}

#[test]
fn finite_number_admission_boundary_rejects_nan_shape_param_token() {
    let err = parse_shape_param_assignment("arm_width=NaN").unwrap_err();
    assert!(matches!(err, ShapeParamParseError::NonNumeric { .. }));
}

#[test]
fn missing_or_unknown_reference_admission_boundary_rejects_unknown_shape_param_key() {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "elliptical".into();
    params.shape.shape_params.insert("bogus_key".into(), 1.0);
    let err = params.validate(&registry()).unwrap_err();
    assert!(matches!(err, ValidationError::UnknownShapeParam { .. }));
}

#[test]
fn parser_span_admission_boundary_rejects_non_positive_num_stars() {
    let mut params = MapGeneratorParams::default();
    params.scale_core.num_stars = 0;
    let err = params.validate(&registry()).unwrap_err();
    assert!(matches!(err, ValidationError::MustBePositive { .. }));
}

#[test]
fn topology_admission_boundary_rejects_hyperlane_min_greater_than_max() {
    let mut params = MapGeneratorParams::default();
    params.hyperlane.num_hyperlanes_min = 5;
    params.hyperlane.num_hyperlanes_max = 2;
    let err = params.validate(&registry()).unwrap_err();
    assert!(matches!(err, ValidationError::MinGreaterThanMax { .. }));
}

#[test]
fn duplicate_id_admission_boundary_rejects_duplicate_special_route_edges() {
    let placement = two_system_placement();
    let err = validate_special_route_edges(
        &placement,
        &[
            SpecialRouteEdge {
                kind: SpecialRouteKind::WormholePair,
                from: "0".into(),
                to: "1".into(),
            },
            SpecialRouteEdge {
                kind: SpecialRouteKind::Gateway,
                from: "0".into(),
                to: "1".into(),
            },
        ],
    )
    .unwrap_err();
    assert!(matches!(err, SpecialRouteError::DuplicateEdge { .. }));
}