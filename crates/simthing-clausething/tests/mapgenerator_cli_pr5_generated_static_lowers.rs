//! MapGeneratorCLI PR5 — prove PR4 `static_galaxy_scenario` output lowers through the closed MapGen path.
//!
//! PR5 must not modify `crates/simthing-clausething/src/`; shared-initializer child-id uniqueness relies on
//! the separate 0.0.8.2.5 lowerer amendment (Part A).

use std::collections::BTreeSet;

use simthing_clausething::{
    HydratedScenarioNode, MapGenLatticeOptions, assert_allowed_simthing_kinds,
    collect_gridcell_location_ids, generate_mapgen_lattice_hierarchy,
    parse_mapgen_neutral_document,
};
use simthing_core::SimThingKind;
use simthing_mapgenerator::{
    LatticeCoord, MapGeneratorParams, ScenarioEmitter, ScenarioEmitterConfig, ShapeRegistry,
    place_and_emit_scenario, validate_default,
};

const FORBIDDEN_OUTPUT_TERMS: &[&str] = &[
    "metadata = {",
    "lattice = {",
    "location = ",
    "field_operator",
    "add_hyperlane",
    "nebula",
    " route",
    " path",
    "predecessor",
    "movement",
    "border",
    "frontline",
    "sqrt",
    "hypot",
    "distance",
    "normalize",
    "PALMA",
    "ResourceFlow",
    "Movement-Front",
    "commitment",
    "BoundaryRequest",
    "link =",
];

struct GeneratedSample {
    text: String,
    params: MapGeneratorParams,
    cells: Vec<LatticeCoord>,
    scenario_id: String,
    fixture_lattice_edge: u32,
}

fn static_sample() -> GeneratedSample {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "static".into();
    params.mode = simthing_mapgenerator::GenerationMode::ArbitraryStatic;
    params.arbitrary.explicit_point_cloud_path = Some("test/fixture.json".into());
    params.scale_core.num_stars = 4;
    params.scale_core.lattice_size = Some(8);
    params.scale_core.core_radius = 0.0;
    params.seed = 4242;
    params.nebula.num_nebulas = 0;
    let cells = vec![
        LatticeCoord { col: 1, row: 2 },
        LatticeCoord { col: 5, row: 3 },
        LatticeCoord { col: 2, row: 6 },
        LatticeCoord { col: 7, row: 1 },
    ];
    let config = ScenarioEmitterConfig::from_params(&params);
    let scenario_id = config.scenario_id.clone();
    let registry = ShapeRegistry::default();
    let emitter = ScenarioEmitter::new(config);
    validate_default(&params).expect("params valid");
    let text = place_and_emit_scenario(&params, &registry, Some(&cells), &emitter)
        .expect("place and emit")
        .into_string();
    GeneratedSample {
        text,
        params,
        cells,
        scenario_id,
        fixture_lattice_edge: 3,
    }
}

fn elliptical_sample() -> GeneratedSample {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "elliptical".into();
    params.scale_core.num_stars = 5;
    params.scale_core.lattice_size = Some(12);
    params.scale_core.core_radius = 0.0;
    params.seed = 77;
    params.nebula.num_nebulas = 0;
    let config = ScenarioEmitterConfig::from_params(&params);
    let scenario_id = config.scenario_id.clone();
    let registry = ShapeRegistry::default();
    let emitter = ScenarioEmitter::new(config);
    validate_default(&params).expect("params valid");
    let text = place_and_emit_scenario(&params, &registry, None, &emitter)
        .expect("place and emit")
        .into_string();
    GeneratedSample {
        text,
        params,
        cells: Vec::new(),
        scenario_id,
        fixture_lattice_edge: 3,
    }
}

fn assert_no_forbidden_terms(text: &str) {
    let lower = text.to_ascii_lowercase();
    for term in FORBIDDEN_OUTPUT_TERMS {
        assert!(
            !lower.contains(&term.to_ascii_lowercase()),
            "forbidden term {term:?} found in emitted text"
        );
    }
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

fn inert_axis(node: &simthing_clausething::HydratedScenarioNode, prefix: &str) -> Option<String> {
    node.properties.iter().find_map(|property| {
        if property.name.starts_with(prefix) {
            property
                .description
                .strip_prefix("inert=")
                .map(str::to_string)
        } else {
            None
        }
    })
}

fn collect_node_ids(node: &HydratedScenarioNode, ids: &mut BTreeSet<String>) {
    ids.insert(node.id.clone());
    for child in &node.children {
        collect_node_ids(child, ids);
    }
}

fn count_nodes(node: &HydratedScenarioNode) -> usize {
    1 + node.children.iter().map(count_nodes).sum::<usize>()
}

fn lower_generated(sample: &GeneratedSample) -> simthing_clausething::MapGenLatticeHierarchy {
    let neutral =
        parse_mapgen_neutral_document(sample.text.as_bytes()).expect("parse generated neutral AST");
    generate_mapgen_lattice_hierarchy(
        &neutral,
        MapGenLatticeOptions {
            fixture_lattice_edge: sample.fixture_lattice_edge,
            ..Default::default()
        },
    )
    .expect("generate lattice hierarchy from generated output")
}

#[test]
fn generated_static_scenario_parses_as_mapgen_neutral_ast() {
    let sample = static_sample();
    assert!(sample.text.contains("static_galaxy_scenario = {"));
    assert_no_forbidden_terms(&sample.text);
    parse_mapgen_neutral_document(sample.text.as_bytes()).expect("neutral AST parse");
}

#[test]
fn generated_static_scenario_lowers_to_lattice_locations() {
    let sample = static_sample();
    let hierarchy = lower_generated(&sample);
    let gridcell_ids = collect_gridcell_location_ids(&hierarchy.pack.root_node);
    assert_eq!(gridcell_ids, vec!["0", "1", "2", "3"]);
    assert_eq!(hierarchy.pack.scenario_id, sample.scenario_id);
    assert_eq!(hierarchy.pack.root.kind, SimThingKind::World);
    assert_allowed_simthing_kinds(&hierarchy.pack.root_node).expect("allowed kinds");
}

#[test]
fn generated_system_count_matches_placement_count() {
    let sample = static_sample();
    let hierarchy = lower_generated(&sample);
    assert_eq!(
        hierarchy.pack.grid_metadata.placements.len(),
        sample.cells.len()
    );
    assert_eq!(
        collect_gridcell_location_ids(&hierarchy.pack.root_node).len(),
        sample.cells.len()
    );
}

#[test]
fn generated_system_ids_preserved() {
    let sample = static_sample();
    let hierarchy = lower_generated(&sample);
    for (index, placement) in hierarchy.pack.grid_metadata.placements.iter().enumerate() {
        let expected = index.to_string();
        assert_eq!(placement.location_id, expected);
        assert_eq!(placement.target_id, expected);
        assert!(find_node(&hierarchy.pack.root_node, &expected).is_some());
    }
}

#[test]
fn generated_integer_positions_preserved() {
    let sample = static_sample();
    let hierarchy = lower_generated(&sample);
    for (index, cell) in sample.cells.iter().enumerate() {
        let node = find_node(&hierarchy.pack.root_node, &index.to_string()).expect("gridcell");
        assert_eq!(
            inert_axis(node, &format!("render_position_x_{index}")).as_deref(),
            Some(cell.col.to_string().as_str())
        );
        assert_eq!(
            inert_axis(node, &format!("render_position_y_{index}")).as_deref(),
            Some(cell.row.to_string().as_str())
        );
        assert_eq!(
            inert_axis(node, &format!("render_position_z_{index}")).as_deref(),
            Some("0")
        );
    }
}

#[test]
fn generated_initializer_bareword_resolves() {
    let sample = static_sample();
    let hierarchy = lower_generated(&sample);
    for system_id in collect_gridcell_location_ids(&hierarchy.pack.root_node) {
        let node = find_node(&hierarchy.pack.root_node, &system_id).expect("gridcell");
        assert!(
            node.children.iter().any(|child| {
                child.kind == SimThingKind::Cohort && child.display_name.contains("Planet Payload")
            }),
            "system {system_id} missing initializer cohort child"
        );
    }
}

#[test]
fn generated_shared_initializer_lowers_with_unique_child_node_ids() {
    let hierarchy = lower_generated(&static_sample());
    let mut ids = BTreeSet::new();
    collect_node_ids(&hierarchy.pack.root_node, &mut ids);
    assert_eq!(
        ids.len(),
        count_nodes(&hierarchy.pack.root_node),
        "duplicate global node ids after lowering shared initializer across systems"
    );
    for system_id in collect_gridcell_location_ids(&hierarchy.pack.root_node) {
        let node = find_node(&hierarchy.pack.root_node, &system_id).expect("gridcell");
        let planet_id = format!("{system_id}_example_rim_initializer_planet");
        assert!(
            node.children.iter().any(|child| child.id == planet_id),
            "expected system-scoped planet child {planet_id}"
        );
    }
}

#[test]
fn generated_every_system_block_emits_initializer_line() {
    let sample = static_sample();
    let text = &sample.text;
    let system_blocks = text.matches("        system = {").count();
    let initializer_lines = text
        .matches("            initializer = example_rim_initializer")
        .count();
    assert_eq!(system_blocks, sample.cells.len());
    assert_eq!(initializer_lines, sample.cells.len());
    assert_eq!(
        text.matches("    example_rim_initializer = {").count(),
        1,
        "sibling initializer definition must be deduped once"
    );
}

#[test]
fn generated_output_requires_no_frontend_widening() {
    let sample = static_sample();
    lower_generated(&sample);
    assert!(sample.text.starts_with("generated_static = {"));
    assert!(sample.text.contains("random_hyperlanes = no"));
}

#[test]
fn generated_output_has_no_links_for_pr5() {
    let hierarchy = lower_generated(&static_sample());
    assert!(hierarchy.pack.grid_metadata.links.is_empty());
    assert!(!static_sample().text.contains("add_hyperlane"));
}

#[test]
fn generated_output_has_no_field_operators_for_pr5() {
    let hierarchy = lower_generated(&static_sample());
    assert!(hierarchy.pack.w_impedance_compose.is_none());
    assert!(hierarchy.pack.stress_compose.is_none());
    assert!(hierarchy.pack.palma_feedstock.is_none());
    assert!(hierarchy.pack.commitment.is_none());
}

#[test]
fn generated_output_has_no_forbidden_semantic_terms() {
    assert_no_forbidden_terms(&static_sample().text);
    assert_no_forbidden_terms(&elliptical_sample().text);
}

#[test]
fn generated_elliptical_scenario_also_lowers_without_links() {
    let sample = elliptical_sample();
    let hierarchy = lower_generated(&sample);
    assert_eq!(
        collect_gridcell_location_ids(&hierarchy.pack.root_node).len(),
        sample.params.scale_core.num_stars as usize
    );
    assert!(hierarchy.pack.grid_metadata.links.is_empty());
}

#[test]
fn generated_static_output_uses_bareword_initializer_not_quoted_string() {
    let text = static_sample().text;
    assert!(text.contains("            initializer = example_rim_initializer"));
    assert!(!text.contains("initializer = \"example_rim_initializer\""));
}

#[test]
fn generated_static_output_includes_sibling_initializer_definition() {
    let text = static_sample().text;
    assert!(text.contains("    example_rim_initializer = {"));
    assert!(text.contains("        planet = { count = 1 }"));
}

#[test]
fn generated_static_system_blocks_have_required_shape() {
    let text = static_sample().text;
    assert!(text.contains("            id = \"0\""));
    assert!(text.contains("            name = \"\""));
    assert!(text.contains("            position = { x = 1 y = 2 z = 0 }"));
}

#[test]
fn crate_still_has_no_mapgenerator_dependency_on_clausething() {
    let manifest = include_str!("../../simthing-mapgenerator/Cargo.toml");
    assert!(!manifest.contains("simthing-clausething"));
}

#[test]
fn mapgenerator_is_dev_dependency_only_in_clausething() {
    let manifest = include_str!("../Cargo.toml");
    let dependencies = manifest
        .split("[dev-dependencies]")
        .next()
        .expect("[dependencies] section")
        .split("[dependencies]")
        .nth(1)
        .expect("[dependencies] section");
    assert!(
        !dependencies.contains("simthing-mapgenerator"),
        "simthing-mapgenerator must not appear under [dependencies]"
    );
    let dev_dependencies = manifest
        .split("[dev-dependencies]")
        .nth(1)
        .expect("[dev-dependencies] section")
        .split("[features]")
        .next()
        .expect("[dev-dependencies] body");
    assert!(
        dev_dependencies.contains("simthing-mapgenerator"),
        "simthing-mapgenerator must appear under [dev-dependencies]"
    );
}
