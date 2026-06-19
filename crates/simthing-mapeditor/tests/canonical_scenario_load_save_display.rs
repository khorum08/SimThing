//! STUDIO-SCENARIO-LOAD-SAVE-DISPLAY-0 — canonical Scenario tree load/save/display proofs.

use std::path::PathBuf;

use simthing_mapeditor::{
    build_studio_scenario_document, load_canonical_studio_document_from_path,
    load_studio_session_from_scenario_path, save_studio_scenario_with_document_roundtrip,
    studio_galaxy_map_gridcells_from_spec, StudioGridcellRole, StudioScenarioAuthorityKind,
    TERRAN_PIRATE_SKELETON_SCENARIO_ID,
};
use simthing_spec::{
    deserialize_scenario_authority, serialize_scenario_authority, set_galaxy_map_display_name,
    set_owner_display_name, validate_scenario_root_authority, ScenarioRootValidationMode,
    GALAXY_GRIDCELL_ROLE_INERT, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
};
use tempfile::TempDir;

const MINIMAL_SCENARIO_ID: &str = "minimal_scenario_root";
const GALAXYMAP_FIXTURE_ID: &str = "minimal_scenario_galaxymap";
const MINIMAL_OWNER_ID: &str = "minimal_owner";

fn minimal_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus/minimal_scenario_root.simthing-scenario.json")
}

fn galaxymap_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus/minimal_scenario_galaxymap.simthing-scenario.json")
}

fn terran_pirate_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json")
}

#[test]
fn studio_loads_minimal_canonical_scenario() {
    let (spec, document) =
        load_canonical_studio_document_from_path(&minimal_fixture_path()).expect("load");
    validate_scenario_root_authority(&spec, ScenarioRootValidationMode::Canonical)
        .expect("canonical validation");
    assert_eq!(document.scenario_id, MINIMAL_SCENARIO_ID);
    assert_eq!(
        document.authority_kind,
        StudioScenarioAuthorityKind::CanonicalScenario
    );
}

#[test]
fn studio_loads_canonical_galaxymap_fixture() {
    let (spec, document) =
        load_canonical_studio_document_from_path(&galaxymap_fixture_path()).expect("load");
    validate_scenario_root_authority(&spec, ScenarioRootValidationMode::Canonical)
        .expect("canonical validation");
    assert_eq!(document.scenario_id, GALAXYMAP_FIXTURE_ID);
    assert!(document.galaxy_map.is_some());
    assert_eq!(document.gridcells.len(), 2);
}

#[test]
fn studio_displays_scenario_gamesession_owner_galaxymap_tree() {
    let (_, document) =
        load_canonical_studio_document_from_path(&minimal_fixture_path()).expect("load");
    assert_eq!(document.scenario_id, MINIMAL_SCENARIO_ID);
    assert_eq!(document.schema_version, Some(1));
    assert!(document.source_label.is_some());
    let game_session = document.game_session.as_ref().expect("game session");
    assert!(game_session.simthing_id_raw > 0);
    assert_eq!(document.owners.len(), 1);
    let owner = &document.owners[0];
    assert_eq!(owner.owner_id, MINIMAL_OWNER_ID);
    assert_eq!(owner.display_name.as_deref(), Some("Minimal Owner"));
    assert_eq!(owner.archetype.as_deref(), Some("player"));
    assert_eq!(owner.silo_marker, Some(0));
    let galaxy_map = document.galaxy_map.as_ref().expect("galaxy map");
    assert!(galaxy_map.is_canonical_galaxy_map);
    assert_eq!(galaxy_map.galaxy_map_id.as_deref(), Some("minimal_galaxy"));
    assert_eq!(galaxy_map.display_name.as_deref(), Some("Minimal Galaxy"));
}

#[test]
fn studio_rebuilds_galaxymap_from_galaxymap_child() {
    let (spec, document) =
        load_canonical_studio_document_from_path(&galaxymap_fixture_path()).expect("load");
    let gridcells = studio_galaxy_map_gridcells_from_spec(&spec).expect("gridcells");
    assert_eq!(gridcells.len(), 2);
    assert_eq!(document.gridcells.len(), 2);
    assert_eq!(
        document.galaxy_map.as_ref().unwrap().simthing_id_raw,
        spec.structural_grid
            .map_container_id
            .parse::<u32>()
            .expect("map container id")
    );
    let session = load_studio_session_from_scenario_path(&galaxymap_fixture_path(), None)
        .expect("studio session");
    assert_eq!(session.hydration.grid.occupied_cells, 2);
    assert_eq!(session.view_model.stars.len(), 2);
}

#[test]
fn studio_distinguishes_inert_and_star_system_gridcells() {
    let (_, document) =
        load_canonical_studio_document_from_path(&galaxymap_fixture_path()).expect("load");
    let inert = document
        .gridcells
        .iter()
        .find(|cell| cell.role == StudioGridcellRole::Inert)
        .expect("inert cell");
    let star_system = document
        .gridcells
        .iter()
        .find(|cell| cell.role == StudioGridcellRole::StarSystem)
        .expect("star_system cell");
    assert_eq!(inert.generated_system_id, Some(1));
    assert_eq!(inert.structural_col, Some(0));
    assert_eq!(inert.structural_row, Some(0));
    assert_eq!(star_system.generated_system_id, Some(2));
    assert_eq!(star_system.structural_col, Some(1));
    assert_eq!(star_system.structural_row, Some(0));
    assert_eq!(GALAXY_GRIDCELL_ROLE_INERT, "inert");
    assert_eq!(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM, "star_system");
}

#[test]
fn studio_save_reload_preserves_canonical_tree() {
    let (spec, _) =
        load_canonical_studio_document_from_path(&galaxymap_fixture_path()).expect("load");
    validate_scenario_root_authority(&spec, ScenarioRootValidationMode::Canonical)
        .expect("pre-save validation");
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("roundtrip.simthing-scenario.json");
    let document = save_studio_scenario_with_document_roundtrip(&spec, &path).expect("save");
    let json = std::fs::read_to_string(&path).expect("read");
    let reloaded = deserialize_scenario_authority(&json).expect("deserialize");
    validate_scenario_root_authority(&reloaded, ScenarioRootValidationMode::Canonical)
        .expect("post-reload validation");
    let rebuilt = build_studio_scenario_document(&reloaded).expect("rebuild document");
    assert_eq!(
        rebuilt.authority_kind,
        StudioScenarioAuthorityKind::CanonicalScenario
    );
    assert_eq!(rebuilt.scenario_id, document.scenario_id);
    assert_eq!(rebuilt.owners.len(), document.owners.len());
    assert_eq!(rebuilt.gridcells.len(), document.gridcells.len());
    assert!(rebuilt.game_session.is_some());
    assert!(rebuilt.galaxy_map.is_some());
}

#[test]
fn studio_edit_owner_display_name_roundtrips() {
    let (mut spec, _) =
        load_canonical_studio_document_from_path(&minimal_fixture_path()).expect("load");
    set_owner_display_name(&mut spec, MINIMAL_OWNER_ID, "Edited Owner").expect("edit owner");
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("owner-edit.simthing-scenario.json");
    let document = save_studio_scenario_with_document_roundtrip(&spec, &path).expect("save");
    assert_eq!(
        document.owners[0].display_name.as_deref(),
        Some("Edited Owner")
    );
    let json = std::fs::read_to_string(&path).expect("read");
    let reloaded = deserialize_scenario_authority(&json).expect("deserialize");
    let rebuilt = build_studio_scenario_document(&reloaded).expect("rebuild");
    assert_eq!(
        rebuilt.owners[0].display_name.as_deref(),
        Some("Edited Owner")
    );
}

#[test]
fn studio_edit_galaxymap_display_name_roundtrips() {
    let (mut spec, _) =
        load_canonical_studio_document_from_path(&minimal_fixture_path()).expect("load");
    set_galaxy_map_display_name(&mut spec, "Edited Galaxy Map").expect("edit galaxy map");
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("galaxymap-edit.simthing-scenario.json");
    let document = save_studio_scenario_with_document_roundtrip(&spec, &path).expect("save");
    assert_eq!(
        document
            .galaxy_map
            .as_ref()
            .unwrap()
            .display_name
            .as_deref(),
        Some("Edited Galaxy Map")
    );
    let json = std::fs::read_to_string(&path).expect("read");
    let reloaded = deserialize_scenario_authority(&json).expect("deserialize");
    let rebuilt = build_studio_scenario_document(&reloaded).expect("rebuild");
    assert_eq!(
        rebuilt.galaxy_map.as_ref().unwrap().display_name.as_deref(),
        Some("Edited Galaxy Map")
    );
}

#[test]
fn studio_legacy_terran_pirate_still_loads_as_legacy_fixture() {
    let path = terran_pirate_fixture_path();
    assert!(path.is_file(), "terran pirate fixture missing");
    let (spec, document) = load_canonical_studio_document_from_path(&path).expect("load");
    assert_eq!(spec.scenario_id, TERRAN_PIRATE_SKELETON_SCENARIO_ID);
    assert_eq!(
        document.authority_kind,
        StudioScenarioAuthorityKind::LegacyWorldRoot
    );
    assert!(document.game_session.is_none());
    assert!(document.owners.is_empty());
    assert!(document.galaxy_map.is_some());
    assert!(
        !document
            .galaxy_map
            .as_ref()
            .expect("map")
            .is_canonical_galaxy_map
    );
    let session = load_studio_session_from_scenario_path(&path, None).expect("session");
    assert_eq!(
        session.scenario_document.authority_kind,
        StudioScenarioAuthorityKind::LegacyWorldRoot
    );
}

#[test]
fn studio_does_not_require_runtime_sim_or_gpu_dispatch() {
    let mapeditor_lib = include_str!("../src/lib.rs");
    let document_src = include_str!("../src/studio_scenario_document.rs");
    for forbidden in [
        "prove_gpu_buffer_residency_blocking",
        "prove_gpu_structural_validation_blocking",
        "GalaxyMapEngine",
        "OwnerEngine",
        "FactionEngine",
        "WorldEngine",
        "ScenarioEngine",
    ] {
        assert!(
            !document_src.contains(forbidden),
            "studio_scenario_document must not reference runtime/gpu engine surface `{forbidden}`"
        );
    }
    assert!(
        !mapeditor_lib.contains("mod simthing_sim"),
        "mapeditor lib must not own simthing-sim runtime"
    );
    let session = load_studio_session_from_scenario_path(&galaxymap_fixture_path(), None)
        .expect("canonical session without gpu");
    assert_eq!(
        session.scenario_document.authority_kind,
        StudioScenarioAuthorityKind::CanonicalScenario
    );
    let json = serialize_scenario_authority(&session.scenario_authority).expect("serialize");
    assert!(!json.is_empty());
}
