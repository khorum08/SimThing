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
