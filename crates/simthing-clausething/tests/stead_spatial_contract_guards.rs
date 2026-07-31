//! STEAD-CONTRACT-0 — executable spatial contract guard.
//!
//! Fails when active source/docs reintroduce withdrawn STEAD drift doctrine, when the budget-admission
//! surface regresses, or when the evidence ledger goes stale. Fast, deterministic, no GPU. See
//! `docs/stead_spatial_contract.md`.

use simthing_clausething::{
    admit_structural_grid, generate_default_mapgen_movement_front_authoring,
    generate_default_mapgen_palma_feedstock, generate_mapgen_lattice_hierarchy,
    parse_mapgen_neutral_document, MapGenLatticeOptions, MapGenMovementFrontErrorKind,
    MapgenStructuralGridBudget, StructuralGridFrame,
};
use simthing_core::{eml_opcode, ColumnIndex, SlotIndex};
use simthing_driver::{
    compile_gu_yang_n4_field_sweeps, compile_palma_n4_field_sweep, GuYangN4FieldSweepSpec,
    PalmaN4FieldSweepSpec,
};
use simthing_gpu::{
    FIELD_SWEEP_LEGACY_PROGRAM_NODES, FIELD_SWEEP_LEGACY_STACK_SLOTS, GRID_N4_NSEW, GRID_N4_WENS,
};

/// A small dense layout (edge ≤ 10) that admits a single bounded execution theater for PALMA/Gu-Yang.
const SMALL_DENSE_DOC: &str = r#"
small = {
    static_galaxy_scenario = {
        name = "Small"
        random_hyperlanes = no
        system = { id = "0" name = "" position = { x = 0 y = 0 z = 0 } }
        system = { id = "1" name = "" position = { x = 5 y = 0 z = 0 } }
        system = { id = "2" name = "" position = { x = 0 y = 5 z = 0 } }
        system = { id = "3" name = "" position = { x = 5 y = 5 z = 0 } initializer = rim_initializer }
    }
    rim_initializer = { name = "Rim" planet = { count = 1 } deposit = { resources = { minerals = 4 } } }
}
"#;

// ---- curated ACTIVE files (the ADR is excluded: it legitimately documents the withdrawal) ----
const ACTIVE_DOCS: &[(&str, &str)] = &[
    (
        "core_design",
        include_str!("../../../docs/simthing_core_design.md"),
    ),
    (
        "design_0_0_8_3",
        include_str!("../../../docs/design_0_0_8_3.md"),
    ),
    (
        "ClauseThingDoc",
        include_str!("../../../docs/clausething/ClauseThingDoc.md"),
    ),
    (
        "MapGeneratorCLI",
        include_str!("../../../docs/clausething/MapGeneratorCLI.md"),
    ),
    (
        "MapGenThing",
        include_str!("../../../docs/clausething/MapGenThing.md"),
    ),
    ("agents", include_str!("../../../docs/agents.md")),
    (
        "stead_spatial_contract",
        include_str!("../../../docs/stead_spatial_contract.md"),
    ),
];
const ACTIVE_SOURCE: &[(&str, &str)] = &[
    ("mapgen_lattice", include_str!("../src/mapgen_lattice.rs")),
    (
        "mapgen_movement_front",
        include_str!("../src/mapgen_movement_front.rs"),
    ),
    (
        "mapgen_resource_flow",
        include_str!("../src/mapgen_resource_flow.rs"),
    ),
    ("mapgen_palma", include_str!("../src/mapgen_palma.rs")),
    ("mapgen_links", include_str!("../src/mapgen_links.rs")),
    // Producer-side (MapGeneratorCLI) modules most prone to reintroducing positions-inert drift. The
    // closed lowerer is upstream of these, but a producer comment can still poison the doctrine.
    (
        "mapgenerator_emitter",
        include_str!("../../simthing-mapgenerator/src/emitter.rs"),
    ),
    (
        "mapgenerator_topology",
        include_str!("../../simthing-mapgenerator/src/topology.rs"),
    ),
];
const EVIDENCE_INDEX: &str = include_str!("../../../docs/tests/current_evidence_index.md");
const ACTIVE_CONSTITUTION: &str = include_str!("../../../docs/design_0_0_8_3.md");
const STEAD_CONTRACT: &str = include_str!("../../../docs/stead_spatial_contract.md");

/// A markdown section is "exempt" (may quote forbidden phrases) iff its heading names a withdrawal.
fn heading_is_exempt(heading: &str) -> bool {
    let h = heading.to_ascii_lowercase();
    h.contains("forbidden") || h.contains("withdrawn") || h.contains("correction")
}

/// Find a forbidden phrase in active content. Docs are scanned section-aware (forbidden/withdrawn
/// sections are exempt); source has no headings so it is strict.
fn scan_for_phrase(files: &[(&str, &str)], phrase: &str, section_aware: bool) -> Vec<String> {
    let needle = phrase.to_ascii_lowercase();
    let mut hits = Vec::new();
    for (name, content) in files {
        let mut exempt = false;
        for (lineno, line) in content.lines().enumerate() {
            if section_aware && line.trim_start().starts_with('#') {
                exempt = heading_is_exempt(line);
            }
            if exempt {
                continue;
            }
            if line.to_ascii_lowercase().contains(&needle) {
                hits.push(format!("{name}:{}: {}", lineno + 1, line.trim()));
            }
        }
    }
    hits
}

fn assert_phrase_absent(phrase: &str) {
    let mut hits = scan_for_phrase(ACTIVE_DOCS, phrase, true);
    hits.extend(scan_for_phrase(ACTIVE_SOURCE, phrase, false));
    assert!(
        hits.is_empty(),
        "withdrawn STEAD drift phrase `{phrase}` reappeared in active source/docs:\n{}",
        hits.join("\n")
    );
}
#[test]
fn mapgen_lattice_must_export_structural_budget_admission() {
    // Compile-time: these symbols must exist. Runtime: admission accepts a huge sparse grid by default.
    let budget = MapgenStructuralGridBudget::default();
    let stats =
        admit_structural_grid(100_000, 100_000, 3, 0, &budget).expect("budget admission exists");
    assert_eq!(stats.cell_count, 10_000_000_000u128);
    // The structural frame helper is the bound substrate for spatial surfaces.
    let _frame_ty: fn(&simthing_clausething::HydratedScenarioGridMetadata) -> StructuralGridFrame =
        StructuralGridFrame::from_grid_metadata;
}
#[test]
fn movement_front_large_layout_must_typed_defer_to_atlas() {
    // Mirrors the proven vast-scale doc shape (initializer declared beside the scenario), span 60 ≫ 10.
    let doc = r#"
big = {
    static_galaxy_scenario = {
        name = "Big"
        random_hyperlanes = no
        system = { id = "0" name = "" position = { x = 0 y = 0 z = 0 } }
        system = { id = "1" name = "" position = { x = 60 y = 0 z = 0 } }
        system = { id = "2" name = "" position = { x = 0 y = 60 z = 0 } }
        system = { id = "3" name = "" position = { x = 60 y = 60 z = 0 } }
        system = { id = "7" name = "" position = { x = 30 y = 30 z = 0 } initializer = rim_initializer }
    }
    rim_initializer = { name = "Rim" planet = { count = 1 } deposit = { resources = { minerals = 4 } } }
}
"#;
    let neutral = parse_mapgen_neutral_document(doc.as_bytes()).expect("parse");
    // Layout admits at this scale (proven elsewhere); the dense front typed-defers.
    generate_mapgen_lattice_hierarchy(&neutral, MapGenLatticeOptions::default())
        .expect("layout admits");
    let err = generate_default_mapgen_movement_front_authoring(&neutral)
        .expect_err("dense MF over a large layout must defer");
    assert!(err.is_atlas_deferral());
    assert_eq!(
        err.kind,
        MapGenMovementFrontErrorKind::AtlasDeferralRequired
    );
}

#[test]
fn palma_and_gu_yang_n4_must_compile_through_generic_field_sweep() {
    let palma = compile_palma_n4_field_sweep(PalmaN4FieldSweepSpec {
        width: 4,
        height: 4,
        n_dims: 2,
        d_col: ColumnIndex::try_from_admitted_authored(0, 2).expect("PALMA d column"),
        w_col: ColumnIndex::try_from_admitted_authored(1, 2).expect("PALMA W column"),
        destination_slot: SlotIndex::new(15),
        inf_sentinel: 1.0e20,
    })
    .expect("PALMA N4 must admit through the generic field-sweep registration");
    assert_eq!(palma.adjacency().offsets(), &GRID_N4_WENS);
    assert_eq!(
        palma.resource_class().stack_slots(),
        FIELD_SWEEP_LEGACY_STACK_SLOTS
    );
    assert_eq!(
        palma.resource_class().max_program_nodes(),
        FIELD_SWEEP_LEGACY_PROGRAM_NODES
    );
    assert!(palma
        .map_program()
        .iter()
        .any(|node| node.opcode == eml_opcode::NEIGHBOR_VALUE));

    let [conductance, flux] = compile_gu_yang_n4_field_sweeps(GuYangN4FieldSweepSpec {
        width: 4,
        height: 4,
        n_dims: 2,
        value_col: ColumnIndex::try_from_admitted_authored(0, 2).expect("Gu-Yang value column"),
        conductance_col: ColumnIndex::try_from_admitted_authored(1, 2)
            .expect("Gu-Yang conductance column"),
        saturation: 4.0,
        chi: 0.75,
        dt: 0.125,
    })
    .expect("Gu-Yang N4 must admit through generic proof-present registrations");
    assert_eq!(conductance.adjacency().offsets(), &GRID_N4_NSEW);
    assert_eq!(flux.adjacency().offsets(), &GRID_N4_NSEW);
    assert_eq!(
        flux.resource_class().stack_slots(),
        FIELD_SWEEP_LEGACY_STACK_SLOTS
    );
}
