//! STEAD-CONTRACT-0 — Resource-Flow / Accumulator ⇄ STEAD spatial binding.
//!
//! RF stays generic, but an arena whose participants are gridcell `Location`s is **spatially indexed
//! through STEAD**: each participant must have a structural `(row,col)` placement in `grid_metadata`
//! (never render metadata), and the arena records the `StructuralGridFrame` it indexes through. See
//! `docs/stead_spatial_contract.md` §5.

use simthing_clausething::{
    HydratedScenarioGridMetadata, HydratedScenarioGridPlacement, PR3_MAX_LINK_FANOUT,
    SpatialBindingMode, generate_default_mapgen_resource_flow_enrollment,
    parse_mapgen_neutral_document, validate_spatial_binding,
};

const RAW_FIXTURE: &str = include_str!("fixtures/mapgen/tiny_pentad_hub_slice_raw.clause");

fn enrollment() -> simthing_clausething::MapGenResourceFlowEnrollment {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse");
    generate_default_mapgen_resource_flow_enrollment(&neutral).expect("rf enrollment")
}

fn metadata(placements: Vec<(&str, u32, u32)>) -> HydratedScenarioGridMetadata {
    HydratedScenarioGridMetadata {
        grid_size: 16,
        max_fanout: PR3_MAX_LINK_FANOUT,
        placements: placements
            .into_iter()
            .map(|(id, row, col)| HydratedScenarioGridPlacement {
                location_id: id.into(),
                target_id: id.into(),
                row,
                col,
            })
            .collect(),
        links: Vec::new(),
    }
}

#[test]
fn rf_location_participants_require_structural_grid_placements() {
    // Every pentad gridcell Location participant has a structural placement → enrollment succeeds and
    // every arena is recorded SpatiallyBoundToGridcellLocations.
    let e = enrollment();
    assert!(!e.expansion_report.arenas.is_empty());
    for arena in &e.expansion_report.arenas {
        assert_eq!(
            arena.spatial_binding.binding_mode,
            SpatialBindingMode::SpatiallyBoundToGridcellLocations
        );
    }
}

#[test]
fn rf_spatial_binding_reports_grid_frame() {
    let e = enrollment();
    let suppression = e
        .expansion_report
        .arenas
        .iter()
        .find(|a| a.arena_id == "mapgen_suppression")
        .expect("suppression arena");
    // The structural frame is recorded (width/height/occupied), indexed through grid_metadata.
    assert_eq!(suppression.spatial_binding.grid_width, Some(7)); // pentad edge
    assert_eq!(suppression.spatial_binding.grid_height, Some(7));
    assert_eq!(suppression.spatial_binding.occupied_cells, Some(5));
}

#[test]
fn rf_spatial_binding_uses_mapgen_grid_metadata_not_render_metadata() {
    // The recorded frame width/height equals the STRUCTURAL grid_size, not any render-position extent.
    let e = enrollment();
    let frame_w = e.expansion_report.arenas[0]
        .spatial_binding
        .grid_width
        .unwrap();
    assert_eq!(frame_w, e.pack.grid_metadata.grid_size);
    assert_eq!(
        e.expansion_report.arenas[0].spatial_binding.occupied_cells,
        Some(e.pack.grid_metadata.placements.len() as u64)
    );
}

#[test]
fn rf_spatial_binding_rejects_missing_location_placement() {
    let md = metadata(vec![("0", 1, 1)]);
    let err = validate_spatial_binding(
        SpatialBindingMode::SpatiallyBoundToGridcellLocations,
        &["0".into(), "missing".into()],
        &md,
    )
    .expect_err("a Location participant without a structural placement must be rejected");
    assert!(
        err.message.contains("no structural grid placement"),
        "{}",
        err.message
    );
}

#[test]
fn rf_spatial_binding_rejects_duplicate_location_placement_if_not_already_guarded() {
    let md = metadata(vec![("0", 1, 1), ("0", 2, 2)]); // two placements, same Location id
    let err = validate_spatial_binding(
        SpatialBindingMode::SpatiallyBoundToGridcellLocations,
        &["0".into()],
        &md,
    )
    .expect_err("duplicate structural placement for a Location must be rejected");
    assert!(
        err.message.contains("duplicate structural grid placement"),
        "{}",
        err.message
    );
}

#[test]
fn rf_spatially_neutral_arena_does_not_require_grid_metadata() {
    // Generic (non-Location) RF stays spatially neutral: no grid metadata required.
    let empty = metadata(Vec::new());
    validate_spatial_binding(
        SpatialBindingMode::SpatiallyNeutral,
        &["whatever".into(), "non_location".into()],
        &empty,
    )
    .expect("spatially-neutral RF needs no structural grid");
}

#[test]
fn accumulator_spatial_pressure_over_locations_uses_structural_gridcell_identity() {
    // The suppression accumulator/arena pressure over gridcell Locations is indexed by the STRUCTURAL
    // gridcell identity: its bound frame's occupied-cell count equals the structural placement count,
    // and equals the suppression participant count (one accumulator participant per placed gridcell).
    let e = enrollment();
    let suppression = e
        .expansion_report
        .arenas
        .iter()
        .find(|a| a.arena_id == "mapgen_suppression")
        .expect("suppression arena");
    assert_eq!(
        suppression.spatial_binding.occupied_cells,
        Some(suppression.participant_count as u64)
    );
    assert_eq!(
        suppression.participant_count as usize,
        e.pack.grid_metadata.placements.len()
    );
}
