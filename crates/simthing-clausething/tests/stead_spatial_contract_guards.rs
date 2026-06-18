//! STEAD-CONTRACT-0 — executable spatial contract guard.
//!
//! Fails when active source/docs reintroduce withdrawn STEAD drift doctrine, when the budget-admission
//! surface regresses, or when the evidence ledger goes stale. Fast, deterministic, no GPU. See
//! `docs/stead_spatial_contract.md`.

use simthing_clausething::{
    MapGenLatticeOptions, MapGenMovementFrontErrorKind, MapgenStructuralGridBudget,
    StructuralGridFrame, admit_structural_grid, generate_default_mapgen_movement_front_authoring,
    generate_default_mapgen_palma_feedstock, generate_mapgen_lattice_hierarchy,
    parse_mapgen_neutral_document,
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
fn active_docs_must_not_call_positions_inert() {
    assert_phrase_absent("positions are inert");
}

#[test]
fn active_docs_must_not_call_shape_cosmetic() {
    assert_phrase_absent("shape is cosmetic");
}

#[test]
fn active_docs_must_not_say_topology_is_the_lattice() {
    assert_phrase_absent("topology is the lattice");
}

#[test]
fn active_source_must_not_call_structural_grid_coordinates_inert() {
    // Source has no withdrawal sections — strict.
    for phrase in [
        "positions are inert",
        "render positions are inert metadata only",
        "authored positions are inert",
    ] {
        let hits = scan_for_phrase(ACTIVE_SOURCE, phrase, false);
        assert!(
            hits.is_empty(),
            "active source reasserts `{phrase}`:\n{}",
            hits.join("\n")
        );
    }
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
fn mapgen_lattice_must_not_export_mapgen_max_lattice_edge() {
    // The fixed structural edge cap is gone (STEAD-SCALE-1). Its definition must not be reintroduced.
    let (_, src) = ACTIVE_SOURCE
        .iter()
        .find(|(n, _)| *n == "mapgen_lattice")
        .unwrap();
    assert!(
        !src.contains("pub const MAPGEN_MAX_LATTICE_EDGE"),
        "MAPGEN_MAX_LATTICE_EDGE (a fixed structural edge cap) must not be reintroduced"
    );
}

#[test]
fn mapgen_lattice_must_honor_emitted_integer_positions() {
    // The lowerer must place a Location at its EMITTED position, never emission-order row-major fill.
    let doc = r#"
honor = {
    static_galaxy_scenario = {
        name = "Honor"
        system = { id = "a" name = "" position = { x = 0 y = 0 z = 0 } }
        system = { id = "b" name = "" position = { x = 9 y = 4 z = 0 } }
    }
}
"#;
    let neutral = parse_mapgen_neutral_document(doc.as_bytes()).expect("parse");
    let h = generate_mapgen_lattice_hierarchy(&neutral, MapGenLatticeOptions::default())
        .expect("lower");
    let b = h
        .pack
        .grid_metadata
        .placements
        .iter()
        .find(|p| p.location_id == "b")
        .expect("b placed");
    assert_eq!(
        (b.col, b.row),
        (9, 4),
        "emitted position honored, not index-order"
    );
    assert_ne!(
        (b.col, b.row),
        (1, 0),
        "must NOT be emission-order row-major fill"
    );
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
fn palma_feedstock_indexes_structural_grid_and_emits_no_routes() {
    // PALMA W/D is a FIELD over the structural lattice — its grid_size is grid_metadata's (structural),
    // and it emits no routes/predecessors/paths (PALMA is not pathfinding; Gu-Yang is not a frontline
    // service). Run on a small dense layout so the bounded theater admits in one slice.
    let neutral = parse_mapgen_neutral_document(SMALL_DENSE_DOC.as_bytes()).expect("parse");
    let hierarchy = generate_mapgen_lattice_hierarchy(&neutral, MapGenLatticeOptions::default())
        .expect("lower");
    let structural_grid_size = hierarchy.pack.grid_metadata.grid_size;
    assert!(
        structural_grid_size <= 10,
        "small dense fixture stays within one theater"
    );

    let palma = generate_default_mapgen_palma_feedstock(&neutral).expect("PALMA feedstock authors");
    assert_eq!(
        palma.expansion_report.grid_size, structural_grid_size,
        "PALMA grid_size is the STRUCTURAL grid_metadata size, not render-derived"
    );
    assert_eq!(
        palma.expansion_report.route_surface_count, 0,
        "PALMA emits no routes"
    );
    assert_eq!(
        palma.expansion_report.predecessor_surface_count, 0,
        "PALMA emits no predecessors/came_from — it is a field, not pathfinding"
    );
}

#[test]
fn transient_constitution_section_0_must_carry_stead_clause_and_contract_pointer() {
    // §0 is the carry-forward spine that every future constitution must copy forward verbatim. The STEAD
    // substrate must be anchored THERE (not only in agents.md / core design), or a mechanical "copy §0"
    // promotion could drop it. Prove the §0.8 clause + the mandatory contract pointer live inside §0
    // (between the "## 0." heading and the next top-level "## " heading).
    let lines: Vec<&str> = ACTIVE_CONSTITUTION.lines().collect();
    let start = lines
        .iter()
        .position(|l| l.starts_with("## 0.") || l.trim_start().starts_with("## 0 "))
        .expect("constitution has a `## 0.` transient-constitution section");
    let end = lines[start + 1..]
        .iter()
        .position(|l| l.starts_with("## ") && !l.starts_with("## 0"))
        .map(|rel| start + 1 + rel)
        .unwrap_or(lines.len());
    let section_0 = lines[start..end].join("\n");

    assert!(
        section_0.contains("### 0.8 STEAD/Mapping spatial substrate carry-forward"),
        "transient constitution §0 must contain the §0.8 STEAD/Mapping carry-forward clause (it drifted \
         three times; §0 is what makes it survive a mechanical `copy §0` constitution promotion)"
    );
    assert!(
        section_0.contains("stead_spatial_contract.md"),
        "§0.8 must carry the mandatory `stead_spatial_contract.md` pointer forward inside §0"
    );
    assert!(
        section_0
            .to_ascii_lowercase()
            .contains("propagate to every future"),
        "§0.8 must state the clause + pointer MUST propagate to every future constitution version"
    );
}

#[test]
fn stead_contract_carries_structural_execution_convergence_section() {
    // §10 binds the Studio→GPU horizon: each structural execution surface routes to an EXISTING sanctioned
    // operator (no bespoke kernel), compiled by simthing-driver and dispatched by simthing-sim. This fences
    // the convergence contract durably so the link-RF + Gu-Yang/PALMA border surfaces are never built bespoke.
    let lower = STEAD_CONTRACT.to_ascii_lowercase();
    assert!(
        lower.contains("structural execution convergence"),
        "stead_spatial_contract.md must carry the §10 structural execution convergence contract"
    );
    for needle in [
        "simthing-driver",
        "simthing-sim",
        "accumulatorop",
        "input_list",
        "saturating_flux_choke_threshold",
        "structured_field_stencil",
        "min_plus_stencil",
        "w_impedance_compose",
    ] {
        assert!(
            lower.contains(needle),
            "§10 convergence contract must name the existing operator/owner `{needle}` (no bespoke kernel)"
        );
    }
    // The three horizon surfaces must all be named so none is silently omitted / forked.
    for surface in ["link", "gu-yang", "palma"] {
        assert!(
            lower.contains(surface),
            "§10 must name the `{surface}` execution surface"
        );
    }
}

#[test]
fn current_evidence_index_must_not_have_pending_rows_for_merged_amendments() {
    // A row marked CURRENT_EVIDENCE (merged) must carry a real SHA, never a placeholder.
    for (lineno, line) in EVIDENCE_INDEX.lines().enumerate() {
        if line.contains("CURRENT_EVIDENCE")
            && (line.contains("(this change)") || line.contains("(pending)"))
        {
            panic!(
                "current_evidence_index:{}: merged amendment row still has a placeholder provenance:\n{}",
                lineno + 1,
                line.trim()
            );
        }
    }
}
