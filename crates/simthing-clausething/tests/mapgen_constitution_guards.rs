//! MapGen PR9 — constitutional guard hardening (Candidate F, P1/horizon, one-system-per-cell).
//!
//! Consolidates cross-cutting admission guards before PR10 end-to-end sample. No new generator
//! capabilities — tests and small validation helpers only.

use simthing_clausething::{
    assert_allowed_simthing_kinds, collect_gridcell_location_ids,
    generate_default_mapgen_links_enrollment, generate_default_mapgen_movement_front_authoring,
    generate_default_mapgen_palma_feedstock, generate_mapgen_lattice_hierarchy,
    generate_mapgen_movement_front_authoring, parse_mapgen_neutral_document,
    validate_l1_operator_locality, validate_one_system_per_gridcell, validate_options,
    MapGenLatticeOptions, MapGenMovementFrontOptions, MAPGEN_MF_DEFAULT_HORIZON,
    MAPGEN_MF_MAX_HORIZON,
};
use simthing_core::SimThingKind;
use simthing_spec::{
    compile_region_field_preview, MappingExecutionProfile, RegionFieldOperatorSpec,
};

const RAW_FIXTURE: &str = include_str!("fixtures/mapgen/tiny_pentad_hub_slice_raw.clause");

const EUCLIDEAN_AUTHORITY_PATTERNS: &[&str] = &[
    "sqrt(",
    "length(",
    "normalize(",
    "hypot(",
    "magnitude(",
    "norm(",
    "euclidean",
    "distance(type=euclidean)",
];

const MAPGEN_GENERATOR_SOURCES: &[(&str, &str)] = &[
    (
        "mapgen_neutral_ast",
        include_str!("../src/mapgen_neutral_ast.rs"),
    ),
    ("mapgen_lattice", include_str!("../src/mapgen_lattice.rs")),
    (
        "mapgen_resource_flow",
        include_str!("../src/mapgen_resource_flow.rs"),
    ),
    ("mapgen_links", include_str!("../src/mapgen_links.rs")),
    (
        "mapgen_movement_front",
        include_str!("../src/mapgen_movement_front.rs"),
    ),
    ("mapgen_palma", include_str!("../src/mapgen_palma.rs")),
];

const GPU_SCHEDULING_SOURCES: &[(&str, &str)] = &[(
    "scheduled_w_palma_batch",
    include_str!("../../simthing-gpu/src/scheduled_w_palma_batch.rs"),
)];

const PR8_GUARD_SOURCES: &[(&str, &str)] = &[
    (
        "scheduled_w_palma_batch",
        include_str!("../../simthing-gpu/src/scheduled_w_palma_batch.rs"),
    ),
    (
        "w_impedance_compose_bridge",
        include_str!("../../simthing-driver/src/w_impedance_compose_bridge.rs"),
    ),
];

const FORBIDDEN_GENERATED_VOCABULARY: &[&str] = &[
    "route",
    "pathfinding",
    "predecessor",
    "movement_order",
    "destination_plan",
    "fleet_path",
    "border_service",
    "frontline",
    "cpu_planner",
    "graph_engine",
];

const FORBIDDEN_KIND_TOKENS: &[&str] = &[
    "GridCellKind",
    "RegionCellKind",
    "SystemKind",
    "SimThingKind::GridCell",
    "SimThingKind::RegionCell",
    "SimThingKind::System",
];

fn full_palma_authoring() -> simthing_clausething::MapGenPalmaFeedstockAuthoring {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse fixture");
    generate_default_mapgen_palma_feedstock(&neutral).expect("PR7 palma feedstock")
}

fn scan_sources_for_patterns(modules: &[(&str, &str)], patterns: &[&str]) -> Vec<(String, String)> {
    let mut violations = Vec::new();
    for (name, source) in modules {
        for pattern in patterns {
            if source.contains(pattern) {
                violations.push(((*name).to_string(), (*pattern).to_string()));
            }
        }
    }
    violations
}

fn assert_no_violations(modules: &[(&str, &str)], patterns: &[&str], label: &str) {
    let violations = scan_sources_for_patterns(modules, patterns);
    assert!(
        violations.is_empty(),
        "{label} must not reference forbidden patterns: {violations:?}"
    );
}

fn walk_property_haystacks(
    node: &simthing_clausething::HydratedScenarioNode,
    out: &mut Vec<String>,
) {
    for property in &node.properties {
        out.push(format!(
            "{} {} {} {}",
            property.id, property.namespace, property.name, property.description
        ));
    }
    for child in &node.children {
        walk_property_haystacks(child, out);
    }
}

fn assert_pack_has_no_forbidden_vocabulary(pack: &simthing_clausething::HydratedScenarioPack) {
    let mut haystacks = Vec::new();
    for property in &pack.game_mode.properties {
        haystacks.push(format!(
            "{} {} {} {}",
            property.id, property.namespace, property.name, property.description
        ));
    }
    walk_property_haystacks(&pack.root_node, &mut haystacks);
    let game_mode_json =
        serde_json::to_string(&pack.game_mode).expect("serialize game mode for guard scan");
    haystacks.push(game_mode_json);
    for haystack in haystacks {
        for forbidden in FORBIDDEN_GENERATED_VOCABULARY {
            assert!(
                !haystack.contains(forbidden),
                "generated surface must not reference `{forbidden}`"
            );
        }
    }
}

fn is_n4_neighbor(left: (u32, u32), right: (u32, u32)) -> bool {
    (left.0 == right.0 && left.1.abs_diff(right.1) == 1)
        || (left.1 == right.1 && left.0.abs_diff(right.0) == 1)
}
#[test]
fn l1_horizon_remains_bounded_at_default() {
    let authoring = full_palma_authoring();
    let field = &authoring.pack.game_mode.region_fields[0];
    assert_eq!(field.horizon, MAPGEN_MF_DEFAULT_HORIZON);
    assert!(field.horizon <= MAPGEN_MF_MAX_HORIZON);
    assert!(!field.allow_extended_horizon);
    validate_l1_operator_locality(field).expect("bounded locality");
}
fn find_node<'a>(
    node: &'a simthing_clausething::HydratedScenarioNode,
    id: &str,
) -> Option<&'a simthing_clausething::HydratedScenarioNode> {
    if node.id == id {
        return Some(node);
    }
    for child in &node.children {
        if let Some(found) = find_node(child, id) {
            return Some(found);
        }
    }
    None
}
