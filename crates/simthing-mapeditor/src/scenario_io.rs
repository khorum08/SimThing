//! SimThing-Spec scenario authority file IO (separate from `simthing-studio-config.json`).
//!
//! Scenario files serialize the whole `SimThingScenarioSpec`. Studio projections rebuild from
//! authority on load; view models, hydration indexes, Bevy state, and presentation config are not
//! persisted as model truth.

use std::path::Path;

use simthing_spec::{
    deserialize_scenario_authority, serialize_scenario_authority, ScenarioSerdeError,
    SimThingScenarioSpec,
};
use thiserror::Error;

use crate::generation::GenerationProfile;
use crate::hydration::StudioHydrationError;
use crate::session::StudioSession;
use crate::studio_scenario_document::StudioScenarioDocumentError;

pub const SCENARIO_FILE_SUFFIX: &str = ".simthing-scenario.json";
pub const SCENARIO_TMP_SUFFIX: &str = "simthing-scenario.json.tmp";

#[derive(Debug, Error)]
pub enum ScenarioIoError {
    #[error("scenario file IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("scenario authority serde error: {0}")]
    Serde(#[from] ScenarioSerdeError),
    #[error("scenario projection rebuild failed: {0}")]
    Hydration(#[from] StudioHydrationError),
    #[error("studio scenario document build failed: {0}")]
    ScenarioDocument(#[from] StudioScenarioDocumentError),
}

pub fn scenario_file_name(stem: &str) -> String {
    format!("{stem}{SCENARIO_FILE_SUFFIX}")
}

pub fn save_scenario_authority_to_path(
    path: &Path,
    scenario: &SimThingScenarioSpec,
) -> Result<(), ScenarioIoError> {
    let json = serialize_scenario_authority(scenario)?;
    atomic_write(path, &json)
}

pub fn load_scenario_authority_from_path(
    path: &Path,
) -> Result<SimThingScenarioSpec, ScenarioIoError> {
    let text = std::fs::read_to_string(path)?;
    Ok(deserialize_scenario_authority(&text)?)
}

pub fn save_current_session_scenario_to_path(
    session: &StudioSession,
    path: &Path,
) -> Result<(), ScenarioIoError> {
    save_scenario_authority_to_path(path, &session.scenario_authority)
}

pub fn load_studio_session_from_scenario_path(
    path: &Path,
    profile_hint: Option<GenerationProfile>,
) -> Result<StudioSession, ScenarioIoError> {
    let scenario_authority = load_scenario_authority_from_path(path)?;
    StudioSession::from_loaded_scenario(scenario_authority, path.to_path_buf(), profile_hint)
        .map_err(ScenarioIoError::from)
}

fn atomic_write(path: &Path, contents: &str) -> Result<(), ScenarioIoError> {
    let tmp = path.with_extension(SCENARIO_TMP_SUFFIX);
    std::fs::write(&tmp, contents)?;
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    std::fs::rename(&tmp, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use simthing_core::{SimThing, SimThingKind};
    use simthing_spec::{
        apply_gridcell_property_edit, reserve_simthing_ids_from_scenario,
        structural_property_value_u32, validate_scenario_links, validate_stead_mapping_consistency,
        SimThingScenarioGrid, SimThingScenarioLink, SimThingScenarioProvenance,
        SimThingStructuralGridFrame, SimThingStructuralGridPlacement, SteadMappingError,
        SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
    };
    use tempfile::TempDir;

    use super::*;
    use crate::generation::{run_generation, GenerationProfile};
    use crate::hydration::generate_simthing_spec_scenario;
    use crate::studio_config::{
        save_studio_config_to_path, SimThingStudioConfig, STUDIO_CONFIG_FILE_NAME,
        STUDIO_CONFIG_SCHEMA_VERSION,
    };
    use crate::view_model::StudioGalaxyViewModel;

    fn generated_session() -> StudioSession {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        StudioSession::from_generation(profile, output).expect("session")
    }

    fn small_scenario() -> SimThingScenarioSpec {
        let mut root = SimThing::new(SimThingKind::World, 0);
        let mut map = SimThing::new(SimThingKind::Location, 0);
        let map_raw = map.id.raw();
        let mut cell = SimThing::new(SimThingKind::Location, 0);
        cell.add_property(
            SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
            structural_property_value_u32(1),
        );
        cell.add_property(
            SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
            structural_property_value_u32(3),
        );
        cell.add_property(
            SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
            structural_property_value_u32(2),
        );
        let mut payload = SimThing::new(SimThingKind::Cohort, 0);
        payload.add_property(
            SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
            structural_property_value_u32(1),
        );
        cell.add_child(payload);
        let cell_raw = cell.id.raw();
        map.add_child(cell);
        root.add_child(map);
        SimThingScenarioSpec {
            scenario_id: "small_spec".to_string(),
            root,
            structural_grid: SimThingScenarioGrid {
                frame: SimThingStructuralGridFrame {
                    width: 8,
                    height: 8,
                    occupied_cells: 1,
                },
                map_container_id: map_raw.to_string(),
                placements: vec![SimThingStructuralGridPlacement {
                    location_id: "small_cell".to_string(),
                    target_id: "small_cell".to_string(),
                    system_id: 1,
                    row: 2,
                    col: 3,
                    simthing_id_raw: cell_raw,
                }],
            },
            links: Vec::new(),
            provenance: SimThingScenarioProvenance {
                source: "test".to_string(),
                generator_seed: 42,
                generator_shape: "spiral_2".to_string(),
                ..Default::default()
            },
        }
    }

    fn write_small_scenario(path: &Path) {
        save_scenario_authority_to_path(path, &small_scenario()).expect("save small scenario");
    }

    #[test]
    fn scenario_authority_load_roundtrip_preserves_root_tree() {
        let session = generated_session();
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("root.simthing-scenario.json");
        save_current_session_scenario_to_path(&session, &path).expect("save");
        let loaded = load_scenario_authority_from_path(&path).expect("load");
        assert_eq!(
            loaded.root.subtree_size(),
            session.scenario_authority.root.subtree_size()
        );
    }

    #[test]
    fn scenario_authority_load_roundtrip_preserves_structural_grid() {
        let session = generated_session();
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("grid.simthing-scenario.json");
        save_current_session_scenario_to_path(&session, &path).expect("save");
        let loaded = load_scenario_authority_from_path(&path).expect("load");
        assert_eq!(
            loaded.structural_grid,
            session.scenario_authority.structural_grid
        );
    }

    #[test]
    fn scenario_authority_load_roundtrip_preserves_map_container_binding() {
        let session = generated_session();
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("map.simthing-scenario.json");
        save_current_session_scenario_to_path(&session, &path).expect("save");
        let loaded = load_scenario_authority_from_path(&path).expect("load");
        assert_eq!(
            loaded.structural_grid.map_container_id,
            session.scenario_authority.structural_grid.map_container_id
        );
        simthing_spec::resolve_map_container(&loaded).expect("binding preserved");
    }

    #[test]
    fn scenario_authority_load_roundtrip_preserves_links() {
        let session = generated_session();
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("links.simthing-scenario.json");
        save_current_session_scenario_to_path(&session, &path).expect("save");
        let loaded = load_scenario_authority_from_path(&path).expect("load");
        assert_eq!(loaded.links, session.scenario_authority.links);
    }

    #[test]
    fn scenario_authority_load_roundtrip_preserves_provenance() {
        let session = generated_session();
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("prov.simthing-scenario.json");
        save_current_session_scenario_to_path(&session, &path).expect("save");
        let loaded = load_scenario_authority_from_path(&path).expect("load");
        assert_eq!(loaded.provenance, session.scenario_authority.provenance);
    }

    #[test]
    fn scenario_authority_load_roundtrip_preserves_gridcell_children() {
        let session = generated_session();
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("children.simthing-scenario.json");
        save_current_session_scenario_to_path(&session, &path).expect("save");
        let loaded = load_scenario_authority_from_path(&path).expect("load");
        assert!(loaded
            .gridcell_locations()
            .all(|gridcell| !gridcell.children.is_empty()));
    }

}
