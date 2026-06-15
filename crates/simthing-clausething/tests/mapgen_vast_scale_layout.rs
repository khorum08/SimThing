//! STEAD-PRIVILEGE-0 — vast-scale layout proof.
//!
//! SimThing models **vast** spatial domains; 200×200 is a *small* reference map. This proves the two
//! halves of the §7 scaling model are decoupled:
//!
//! 1. **LAYOUT scales freely.** The gridcell-Location lattice honors emitted integer positions at far
//!    larger than the small 200×200 reference (here ~1000 per edge), one-system-per-cell, sparse.
//! 2. **Dense Movement-Front EXECUTION is a bounded local theater (§7 P1).** A vast lattice does not run
//!    its front as one dense-global field (the permanently-rejected pattern) — it honestly defers to the
//!    multi-theater **atlas** rung. The layout is valid; only the bounded-stencil execution is gated.

use simthing_clausething::{
    MAPGEN_CANONICAL_LATTICE_EDGE, MapGenLatticeOptions, MapGenMovementFrontErrorKind,
    generate_default_mapgen_movement_front_authoring, generate_mapgen_lattice_hierarchy,
    parse_mapgen_neutral_document,
};

/// Five systems spread across a ~1000-per-edge galactic layout — far larger than the 200×200 reference.
const VAST_DOC: &str = r#"
vast_scale_layout = {
    static_galaxy_scenario = {
        name = "Vast Scale Layout"
        random_hyperlanes = no
        system = { id = "0" name = "" position = { x = 0 y = 0 z = 0 } }
        system = { id = "1" name = "" position = { x = 1000 y = 0 z = 0 } }
        system = { id = "2" name = "" position = { x = 0 y = 1000 z = 0 } }
        system = { id = "3" name = "" position = { x = 1000 y = 1000 z = 0 } }
        system = { id = "7" name = "" position = { x = 500 y = 500 z = 0 } initializer = example_rim_initializer }
    }
    example_rim_initializer = { name = "Rim" planet = { count = 1 } deposit = { resources = { minerals = 4 } } }
}
"#;

#[test]
fn vast_gridcell_lattice_lays_out_far_beyond_the_small_canonical_reference() {
    let neutral = parse_mapgen_neutral_document(VAST_DOC.as_bytes()).expect("parse vast doc");
    let hierarchy = generate_mapgen_lattice_hierarchy(&neutral, MapGenLatticeOptions::default())
        .expect("vast layout lowers");

    // The lattice edge is the authored bounding box (1001) — vastly larger than the SMALL 200×200 reference.
    assert_eq!(hierarchy.pack.grid_metadata.grid_size, 1001);
    assert!(
        hierarchy.pack.grid_metadata.grid_size > MAPGEN_CANONICAL_LATTICE_EDGE,
        "200×200 is a small reference; SimThing anticipates far larger gridcell lattices"
    );
    assert!(
        hierarchy.pack.grid_metadata.grid_size > 256,
        "the old 256 layout ceiling is gone"
    );

    // Positions are honored at scale — the galactic pattern IS the lattice (not an emission-order block).
    assert_eq!(hierarchy.pack.grid_metadata.placements.len(), 5);
    let placement_of = |id: &str| -> (u32, u32) {
        hierarchy
            .pack
            .grid_metadata
            .placements
            .iter()
            .find(|placement| placement.location_id == id)
            .map(|placement| (placement.col, placement.row))
            .expect("placement")
    };
    assert_eq!(placement_of("0"), (0, 0));
    assert_eq!(placement_of("3"), (1000, 1000)); // far corner — honored, not compacted
    assert_eq!(placement_of("7"), (500, 500)); // centre

    // One-system-per-cell across the vast sparse lattice.
    let mut occupied = std::collections::BTreeSet::new();
    for placement in &hierarchy.pack.grid_metadata.placements {
        assert!(occupied.insert((placement.row, placement.col)));
    }
}

#[test]
fn movement_front_execution_over_a_vast_lattice_defers_to_the_atlas_not_dense_global() {
    // §7 P1: the dense stencil is a bounded local theater. A vast lattice's front is NOT one dense-global
    // field — it defers to the multi-theater atlas rung. The error must make clear the LAYOUT is valid and
    // only the bounded EXECUTION is gated.
    let neutral = parse_mapgen_neutral_document(VAST_DOC.as_bytes()).expect("parse vast doc");
    // Layout admits at vast scale (proven above); the dense front is what defers.
    generate_mapgen_lattice_hierarchy(&neutral, MapGenLatticeOptions::default())
        .expect("vast layout admits structurally");
    let err = generate_default_mapgen_movement_front_authoring(&neutral).expect_err(
        "dense Movement-Front execution over a vast lattice must defer to the atlas rung",
    );
    // TYPED deferral (STEAD-SCALE-1) — callers match a variant, not a string.
    assert!(
        err.is_atlas_deferral(),
        "deferral must be the typed AtlasDeferralRequired, got kind {:?}: {}",
        err.kind,
        err.message
    );
    assert_eq!(
        err.kind,
        MapGenMovementFrontErrorKind::AtlasDeferralRequired
    );
}
