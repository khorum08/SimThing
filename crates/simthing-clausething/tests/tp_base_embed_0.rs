use std::path::{Path, PathBuf};

use simthing_clausething::{hydrate_scenario, parse_raw_document};
use simthing_spec::{
    SimThingScenarioSpec, deserialize_scenario_authority, save_scenario_spec_to_canonical_json,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-mapeditor/tests/fixtures/tp_base_disc_1500.simthing-scenario.json")
}

fn fixture_path_text() -> String {
    fixture_path().to_string_lossy().replace('\\', "/")
}

fn fixture_scenario() -> SimThingScenarioSpec {
    let source = std::fs::read_to_string(fixture_path()).expect("read canonical base fixture");
    deserialize_scenario_authority(&source).expect("deserialize canonical base fixture")
}

fn fixture_source() -> String {
    std::fs::read_to_string(fixture_path()).expect("read canonical base fixture")
}

fn combined_clause(namespace: &str) -> String {
    format!(
        r#"
scenario = tp_base_embed_0 {{
    metadata = {{
        display_name = "TP Base Embed 0"
        runtime_owner = "scenario-container"
    }}
    static_galaxy_scenario = base_disc {{
        namespace = "{namespace}"
        source_json = "{}"
        map_quality_status = PASS
    }}
}}
"#,
        fixture_path_text()
    )
}

#[test]
fn combined_clause_parses_with_embedded_static_galaxy_scenario() {
    let source = combined_clause("tp_base");
    let document = parse_raw_document(source.as_bytes()).expect("parse combined clause");
    let pack = hydrate_scenario(&document).expect("hydrate combined clause");

    assert_eq!(pack.scenario_id, "tp_base_embed_0");
    assert_eq!(pack.embedded_static_galaxy_scenarios.len(), 1);
    assert_eq!(pack.grid_metadata.placements.len(), 1500);
    assert_eq!(pack.grid_metadata.grid_size, 300);
}

#[test]
fn embedded_base_lattice_round_trips_identical_to_canonical_artifact() {
    let source = combined_clause("tp_base");
    let document = parse_raw_document(source.as_bytes()).expect("parse combined clause");
    let pack = hydrate_scenario(&document).expect("hydrate combined clause");
    let embedded = &pack.embedded_static_galaxy_scenarios[0];
    let fixture = fixture_scenario();

    assert_eq!(embedded.source_structural_grid, fixture.structural_grid);
    assert_eq!(embedded.provenance, fixture.provenance);
    assert_eq!(embedded.scenario_id, fixture.scenario_id);

    let canonical = save_scenario_spec_to_canonical_json(&fixture).expect("canonical save");
    assert_eq!(canonical.byte_len, 889808);
    assert_eq!(canonical.canonical_json, fixture_source());
}

#[test]
fn base_ids_are_namespaced_into_overlay_location_targets() {
    let source = combined_clause("tp_base");
    let document = parse_raw_document(source.as_bytes()).expect("parse combined clause");
    let pack = hydrate_scenario(&document).expect("hydrate combined clause");
    let embedded = &pack.embedded_static_galaxy_scenarios[0];

    assert_eq!(embedded.namespaced_placements.len(), 1500);
    assert!(
        embedded
            .namespaced_placements
            .iter()
            .all(|placement| placement.location_id.starts_with("tp_base::"))
    );
    assert!(
        embedded
            .namespaced_placements
            .iter()
            .all(|placement| placement.target_id.starts_with("tp_base::"))
    );
    assert_eq!(
        pack.grid_metadata.placements,
        embedded.namespaced_placements
    );
}

#[test]
fn duplicate_namespaced_base_ids_hard_error_with_span() {
    let path = fixture_path_text();
    let source = format!(
        r#"
scenario = duplicate_base_namespace {{
    static_galaxy_scenario = first_base {{
        namespace = "tp_base"
        source_json = "{path}"
        map_quality_status = PASS
    }}
    static_galaxy_scenario = second_base {{
        namespace = "tp_base"
        source_json = "{path}"
        map_quality_status = PASS
    }}
}}
"#
    );
    let document = parse_raw_document(source.as_bytes()).expect("parse duplicate namespace source");
    let err = hydrate_scenario(&document).expect_err("duplicate namespace must hard-error");

    assert!(
        err.to_string()
            .contains("duplicate scenario location-target id"),
        "{err}"
    );
    assert!(err.span.is_some(), "duplicate-id error must carry a span");
}

#[test]
fn producer_provenance_remains_separate_from_overlay_runtime_owner() {
    let source = combined_clause("tp_base");
    let document = parse_raw_document(source.as_bytes()).expect("parse combined clause");
    let pack = hydrate_scenario(&document).expect("hydrate combined clause");
    let embedded = &pack.embedded_static_galaxy_scenarios[0];

    assert_eq!(
        pack.metadata.get("runtime_owner").map(String::as_str),
        Some("scenario-container")
    );
    assert_eq!(pack.game_mode.id, "tp_base_embed_0");
    assert_eq!(embedded.provenance.source, "MapGeneratorLibrary");
    assert_eq!(embedded.provenance.generator_seed, 770421);
    assert!(!pack.metadata.contains_key("generator_params_json"));
    assert!(!pack.metadata.contains_key("generator_seed"));
}
