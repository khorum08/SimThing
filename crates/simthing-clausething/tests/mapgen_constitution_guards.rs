//! MapGen PR9 — constitutional guard hardening (Candidate F, P1/horizon, one-system-per-cell).
//!
//! Consolidates cross-cutting admission guards before PR10 end-to-end sample. No new generator
//! capabilities — tests and small validation helpers only.

use simthing_clausething::{
    MAPGEN_MF_DEFAULT_HORIZON, MAPGEN_MF_MAX_HORIZON, MapGenLatticeOptions,
    MapGenMovementFrontOptions, assert_allowed_simthing_kinds, collect_gridcell_location_ids,
    generate_default_mapgen_links_enrollment, generate_default_mapgen_movement_front_authoring,
    generate_default_mapgen_palma_feedstock, generate_mapgen_lattice_hierarchy,
    generate_mapgen_movement_front_authoring, parse_mapgen_neutral_document,
    validate_l1_operator_locality, validate_one_system_per_gridcell, validate_options,
};
use simthing_core::SimThingKind;
use simthing_spec::{
    MappingExecutionProfile, RegionFieldOperatorSpec, compile_region_field_preview,
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
fn pr1_through_pr8_authoring_pipeline_still_succeeds() {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse");
    generate_mapgen_lattice_hierarchy(&neutral, MapGenLatticeOptions::default())
        .expect("PR3 hierarchy");
    generate_default_mapgen_links_enrollment(&neutral).expect("PR5 links");
    generate_default_mapgen_movement_front_authoring(&neutral).expect("PR6 movement front");
    full_palma_authoring();
}

#[test]
fn mapgen_generator_modules_have_no_euclidean_authority() {
    assert_no_violations(
        MAPGEN_GENERATOR_SOURCES,
        EUCLIDEAN_AUTHORITY_PATTERNS,
        "MapGen generator source",
    );
}

#[test]
fn gpu_scheduling_modules_have_no_euclidean_authority() {
    assert_no_violations(
        GPU_SCHEDULING_SOURCES,
        EUCLIDEAN_AUTHORITY_PATTERNS,
        "GPU scheduling source",
    );
}

#[test]
fn mapgen_active_api_has_no_forbidden_kind_tokens() {
    let lib_src = include_str!("../src/lib.rs");
    for token in FORBIDDEN_KIND_TOKENS {
        assert!(
            !lib_src.contains(token),
            "public mapgen API must not reference forbidden kind token `{token}`"
        );
    }
    for (name, source) in MAPGEN_GENERATOR_SOURCES {
        for token in FORBIDDEN_KIND_TOKENS {
            assert!(
                !source.contains(token),
                "{name} must not reference forbidden kind token `{token}`"
            );
        }
    }
}

#[test]
fn full_pipeline_preserves_one_system_per_gridcell() {
    let authoring = full_palma_authoring();
    validate_one_system_per_gridcell(&authoring.pack.grid_metadata).expect("unique placements");
    assert_eq!(authoring.pack.grid_metadata.placements.len(), 5);
}

#[test]
fn duplicate_gridcell_placement_is_rejected() {
    use simthing_clausething::HydratedScenarioGridMetadata;
    let metadata = HydratedScenarioGridMetadata {
        grid_size: 3,
        max_fanout: 1,
        placements: vec![
            simthing_clausething::HydratedScenarioGridPlacement {
                location_id: "0".into(),
                target_id: "0".into(),
                row: 0,
                col: 0,
            },
            simthing_clausething::HydratedScenarioGridPlacement {
                location_id: "9".into(),
                target_id: "9".into(),
                row: 0,
                col: 0,
            },
        ],
        links: Vec::new(),
    };
    let err = validate_one_system_per_gridcell(&metadata).expect_err("duplicate cell");
    assert!(err.message.contains("duplicate gridcell placement"));
}

#[test]
fn gridcells_remain_ordinary_location_simthings() {
    let authoring = full_palma_authoring();
    for id in collect_gridcell_location_ids(&authoring.pack.root_node) {
        let node = find_node(&authoring.pack.root_node, &id).expect("gridcell node");
        assert_eq!(node.kind, SimThingKind::Location);
    }
    assert_allowed_simthing_kinds(&authoring.pack.root_node).expect("allowed kinds");
}

#[test]
fn render_positions_are_inert_metadata_only() {
    let authoring = full_palma_authoring();
    for id in collect_gridcell_location_ids(&authoring.pack.root_node) {
        let node = find_node(&authoring.pack.root_node, &id).expect("gridcell");
        for axis_prefix in [
            "render_position_x_",
            "render_position_y_",
            "render_position_z_",
        ] {
            let property = node
                .properties
                .iter()
                .find(|p| p.name.starts_with(axis_prefix))
                .unwrap_or_else(|| panic!("missing {axis_prefix} on {id}"));
            assert!(
                property.description.contains("inert="),
                "{axis_prefix} on {id} must be inert metadata"
            );
        }
    }
}

#[test]
fn hyperlane_links_use_lattice_placements_not_render_coordinates() {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse");
    let links = generate_default_mapgen_links_enrollment(&neutral).expect("links");
    let placement_map: std::collections::BTreeMap<String, (u32, u32)> = links
        .pack
        .grid_metadata
        .placements
        .iter()
        .map(|p| (p.location_id.clone(), (p.row, p.col)))
        .collect();
    for link in &links.pack.grid_metadata.links {
        let from = placement_map[&link.from];
        let to = placement_map[&link.to];
        assert!(
            is_n4_neighbor(from, to),
            "link {from:?}->{to:?} must be N4 lattice adjacency"
        );
    }
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

#[test]
fn horizon_above_cap_is_rejected() {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse");
    let links = generate_default_mapgen_links_enrollment(&neutral).expect("links");
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
fn allow_extended_horizon_is_rejected() {
    let authoring = full_palma_authoring();
    let mut field = authoring.pack.game_mode.region_fields[0].clone();
    field.allow_extended_horizon = true;
    let err = validate_l1_operator_locality(&field).unwrap_err();
    assert!(err.message.contains("horizon widening"));
}

#[test]
fn dense_global_diffusion_profile_is_rejected() {
    let authoring = full_palma_authoring();
    let mut field = authoring.pack.game_mode.region_fields[0].clone();
    field.operator = RegionFieldOperatorSpec::SourceCappedNormalized;
    let err = validate_l1_operator_locality(&field).unwrap_err();
    assert!(err.message.contains("dense/global"));
}

#[test]
fn l2_reduction_does_not_widen_l1_horizon() {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse");
    let movement = generate_default_mapgen_movement_front_authoring(&neutral).expect("PR6");
    let palma = full_palma_authoring();
    let mf_field = &movement.pack.game_mode.region_fields[0];
    let palma_field = &palma.pack.game_mode.region_fields[0];
    assert!(mf_field.reduction.is_some());
    assert_eq!(mf_field.horizon, palma_field.horizon);
    assert_eq!(mf_field.horizon, MAPGEN_MF_DEFAULT_HORIZON);
}

#[test]
fn scheduled_concurrency_feedstock_does_not_widen_horizon() {
    let authoring = full_palma_authoring();
    let field = &authoring.pack.game_mode.region_fields[0];
    assert_eq!(field.horizon, MAPGEN_MF_DEFAULT_HORIZON);
    assert!(!field.allow_extended_horizon);
    compile_region_field_preview(field).expect("PR8 feedstock admits under bounded horizon");
}

#[test]
fn palma_remains_field_feedstock_not_route_output() {
    let authoring = full_palma_authoring();
    assert!(authoring.pack.palma_feedstock.is_some());
    assert_eq!(authoring.expansion_report.route_surface_count, 0);
    assert_eq!(authoring.expansion_report.predecessor_surface_count, 0);
}

#[test]
fn full_pipeline_generated_surfaces_have_no_forbidden_semantics() {
    assert_pack_has_no_forbidden_vocabulary(&full_palma_authoring().pack);
}

#[test]
fn mapping_profile_remains_default_off_across_full_pipeline() {
    let authoring = full_palma_authoring();
    assert_eq!(
        authoring.pack.game_mode.mapping_execution_profile,
        MappingExecutionProfile::Disabled
    );
}

#[test]
fn pr8_gpu_helpers_have_no_forbidden_semantics() {
    assert_no_violations(
        PR8_GUARD_SOURCES,
        &[
            "pathfinding",
            "predecessor",
            "movement_order",
            "euclidean",
            "sqrt(",
        ],
        "PR8 GPU helper source",
    );
}

#[test]
fn pr8_harness_documents_compact_probe_only_readback() {
    let harness = include_str!("../../simthing-driver/tests/mapgen_pr8_scheduled_concurrency.rs");
    assert!(
        harness.contains("Compact D probe readback only"),
        "PR8 harness must document compact probe posture"
    );
    assert!(
        harness.contains("report.mapping.field_values.is_none()"),
        "PR8 harness must assert no full-field mapping readback"
    );
}

#[test]
fn movement_front_options_reject_missing_horizon_cap() {
    let err = validate_options(&MapGenMovementFrontOptions {
        horizon: 0,
        ..MapGenMovementFrontOptions::default()
    })
    .unwrap_err();
    assert!(err.message.contains("horizon caps"));
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
