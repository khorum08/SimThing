use std::path::{Path, PathBuf};

use simthing_clausething::{hydrate_scenario, parse_raw_document};
use simthing_core::SimThingKind;
use simthing_spec::{
    SimThingScenarioGrid, SimThingScenarioProvenance, SimThingScenarioSpec,
    SimThingStructuralGridFrame, deserialize_scenario_authority, game_session_child,
    game_session_galaxy_map, game_session_owners, owner_archetype, owner_color_index,
    owner_display_name, owner_entity_id, owner_silo_capacity, owner_silo_current,
    save_scenario_spec_to_canonical_json, validate_scenario_game_session_child,
    validate_session_galaxy_map, validate_session_owner_entities,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-mapeditor/tests/fixtures/tp_base_disc_1500.simthing-scenario.json")
}

fn fixture_path_text() -> String {
    fixture_path().to_string_lossy().replace('\\', "/")
}

fn combined_clause() -> String {
    format!(
        r#"
scenario = tp_owner_siblings_0 {{
    metadata = {{
        display_name = "TP Owner Siblings 0"
        runtime_owner = "scenario-container"
    }}
    static_galaxy_scenario = base_disc {{
        namespace = "tp_base"
        source_json = "{}"
        map_quality_status = PASS
    }}
    owner = terran {{
        owner_key = "terran"
        display_name = "Terran Compact"
        archetype = "settler_policy"
        color_index = 1
        stockpile_seed = 1200
        stockpile_capacity = 4800
        policy_profile = "balanced_expansion"
        personality_profile = "defensive_builder"
        capability_profile = "industrial_base"
    }}
    owner = pirate {{
        owner_key = "pirate"
        display_name = "Pirate Cartel"
        archetype = "raider_policy"
        color_index = 2
        stockpile_seed = 300
        stockpile_capacity = 1200
        policy_profile = "pressure_raids"
        personality_profile = "opportunistic"
        capability_profile = "fleet_pressure"
    }}
}}
"#,
        fixture_path_text()
    )
}

fn hydrate_pack() -> simthing_clausething::HydratedScenarioPack {
    let document = parse_raw_document(combined_clause().as_bytes()).expect("parse combined clause");
    hydrate_scenario(&document).expect("hydrate owner siblings clause")
}

fn authority_spec(pack: &simthing_clausething::HydratedScenarioPack) -> SimThingScenarioSpec {
    SimThingScenarioSpec {
        scenario_id: pack.scenario_id.clone(),
        root: pack
            .authority_root
            .clone()
            .expect("owner siblings pack carries canonical authority root"),
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: 0,
                height: 0,
                occupied_cells: 0,
            },
            map_container_id: String::new(),
            placements: Vec::new(),
        },
        links: Vec::new(),
        provenance: SimThingScenarioProvenance {
            source: "MapGeneratorLibrary".to_string(),
            generator_seed: 770421,
            generator_shape: "elliptical".to_string(),
            ..SimThingScenarioProvenance::default()
        },
    }
}

#[test]
fn scenario_container_parses_embedded_base_and_owner_blocks() {
    let pack = hydrate_pack();

    assert_eq!(pack.scenario_id, "tp_owner_siblings_0");
    assert_eq!(pack.embedded_static_galaxy_scenarios.len(), 1);
    assert_eq!(pack.owners.len(), 2);
    assert_eq!(pack.owners[0].owner_key, "terran");
    assert_eq!(pack.owners[1].owner_key, "pirate");
}

#[test]
fn owners_are_direct_gamesession_children() {
    let pack = hydrate_pack();
    let spec = authority_spec(&pack);

    validate_scenario_game_session_child(&spec).expect("canonical GameSession child");
    validate_session_owner_entities(&spec).expect("canonical direct owners");
    let owners = game_session_owners(&spec).expect("owner children");

    assert_eq!(owners.len(), 2);
    assert_eq!(owner_entity_id(owners[0]).as_deref(), Some("terran"));
    assert_eq!(owner_entity_id(owners[1]).as_deref(), Some("pirate"));
    assert!(owners.iter().all(|owner| owner.children.is_empty()));
}

#[test]
fn galaxy_map_remains_gamesession_sibling_not_owner_child() {
    let pack = hydrate_pack();
    let spec = authority_spec(&pack);

    validate_session_galaxy_map(&spec).expect("canonical direct galaxy map");
    let game_session = game_session_child(&spec).expect("GameSession child");
    let galaxy_map = game_session_galaxy_map(&spec).expect("GalaxyMap child");

    assert_eq!(game_session.children.len(), 3);
    assert_eq!(galaxy_map.kind, SimThingKind::Location);
    assert!(
        game_session
            .children
            .iter()
            .any(|child| child.id == galaxy_map.id)
    );
    assert!(
        game_session
            .children
            .iter()
            .filter(|child| child.kind == SimThingKind::Owner)
            .all(|owner| owner.children.iter().all(|child| child.id != galaxy_map.id))
    );
}

#[test]
fn embedded_base_placements_remain_unchanged_from_base_embed() {
    let pack = hydrate_pack();
    let embedded = &pack.embedded_static_galaxy_scenarios[0];
    let source = std::fs::read_to_string(fixture_path()).expect("read canonical base fixture");
    let fixture = deserialize_scenario_authority(&source).expect("deserialize base fixture");

    assert_eq!(embedded.source_structural_grid, fixture.structural_grid);
    assert_eq!(embedded.namespaced_placements.len(), 1500);
    assert_eq!(
        pack.grid_metadata.placements,
        embedded.namespaced_placements
    );
}

#[test]
fn duplicate_owner_ids_hard_error_with_span() {
    let source = format!(
        r#"
scenario = duplicate_owner {{
    static_galaxy_scenario = base_disc {{
        namespace = "tp_base"
        source_json = "{}"
        map_quality_status = PASS
    }}
    owner = terran {{
        owner_key = "terran"
        display_name = "Terran Compact"
    }}
    owner = pirate_alias {{
        owner_key = "terran"
        display_name = "Pirate Cartel"
    }}
}}
"#,
        fixture_path_text()
    );
    let document = parse_raw_document(source.as_bytes()).expect("parse duplicate owner source");
    let err = hydrate_scenario(&document).expect_err("duplicate owner must hard-error");

    assert!(
        err.to_string().contains("duplicate scenario owner id"),
        "{err}"
    );
    assert!(
        err.span.is_some(),
        "duplicate-owner error must carry a span"
    );
}

#[test]
fn unsupported_owner_fields_hard_error_with_span() {
    let source = format!(
        r#"
scenario = unsupported_owner_field {{
    static_galaxy_scenario = base_disc {{
        namespace = "tp_base"
        source_json = "{}"
        map_quality_status = PASS
    }}
    owner = terran {{
        owner_key = "terran"
        display_name = "Terran Compact"
        owner_column = "forbidden_here"
    }}
}}
"#,
        fixture_path_text()
    );
    let document = parse_raw_document(source.as_bytes()).expect("parse unsupported owner source");
    let err = hydrate_scenario(&document).expect_err("unsupported owner field must hard-error");

    assert!(err.to_string().contains("unsupported owner field"), "{err}");
    assert!(
        err.span.is_some(),
        "unsupported-owner-field error must carry a span"
    );
}

#[test]
fn scenario_roundtrip_preserves_owner_metadata_distinct_from_spatial_parentage() {
    let pack = hydrate_pack();
    let spec = authority_spec(&pack);
    let canonical = save_scenario_spec_to_canonical_json(&spec).expect("canonical save");
    let roundtrip =
        deserialize_scenario_authority(&canonical.canonical_json).expect("canonical roundtrip");
    let owners = game_session_owners(&roundtrip).expect("roundtrip owners");

    assert_eq!(owners.len(), 2);
    let terran = owners
        .iter()
        .copied()
        .find(|owner| owner_entity_id(owner).as_deref() == Some("terran"))
        .expect("terran owner");
    assert_eq!(
        owner_display_name(terran).as_deref(),
        Some("Terran Compact")
    );
    assert_eq!(owner_archetype(terran).as_deref(), Some("settler_policy"));
    assert_eq!(owner_color_index(terran), Some(1));
    assert_eq!(owner_silo_current(terran), Some(1200));
    assert_eq!(owner_silo_capacity(terran), Some(4800));
    assert!(terran.children.is_empty());

    let galaxy_map = game_session_galaxy_map(&roundtrip).expect("roundtrip galaxy map");
    assert_eq!(galaxy_map.kind, SimThingKind::Location);
}
