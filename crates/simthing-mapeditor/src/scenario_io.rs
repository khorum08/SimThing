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
            },
        }
    }

    fn write_small_scenario(path: &Path) {
        save_scenario_authority_to_path(path, &small_scenario()).expect("save small scenario");
    }

    #[test]
    fn scenario_authority_saves_whole_simthing_scenario_spec() {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("whole.simthing-scenario.json");
        let session = generated_session();
        save_current_session_scenario_to_path(&session, &path).expect("save");
        let text = std::fs::read_to_string(&path).expect("read");
        assert!(text.contains("scenario_id"));
        assert!(text.contains("structural_grid"));
        assert!(text.contains("map_container_id"));
        assert!(text.contains("placements"));
        assert!(text.contains("links"));
        assert!(text.contains("provenance"));
        assert!(text.contains("\"root\""));
    }

    #[test]
    fn scenario_authority_save_does_not_write_studio_config() {
        let dir = TempDir::new().expect("tempdir");
        let scenario_path = dir.path().join("model.simthing-scenario.json");
        let config_path = dir.path().join(STUDIO_CONFIG_FILE_NAME);
        let session = generated_session();
        save_current_session_scenario_to_path(&session, &scenario_path).expect("save");
        save_studio_config_to_path(&config_path, &SimThingStudioConfig::default())
            .expect("save config");
        let scenario_text = std::fs::read_to_string(&scenario_path).expect("read scenario");
        assert!(!scenario_text.contains(STUDIO_CONFIG_FILE_NAME));
        assert!(!scenario_text.contains("settings_dialog"));
        assert!(!scenario_text.contains("star_rendering"));
        assert!(!scenario_text.contains("hyperlane_rendering"));
        assert!(!scenario_text.contains(&format!(
            "\"schema_version\":{STUDIO_CONFIG_SCHEMA_VERSION}"
        )));
    }

    #[test]
    fn scenario_authority_save_does_not_write_view_model() {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("no-vm.simthing-scenario.json");
        let session = generated_session();
        save_current_session_scenario_to_path(&session, &path).expect("save");
        let text = std::fs::read_to_string(&path).expect("read");
        assert!(!text.contains("render_meta"));
        assert!(!text.contains("render_anchors"));
        assert!(!text.contains("StudioGalaxyViewModel"));
        assert!(!text.contains("vertical_thickness_scale"));
        assert!(!text.contains("world_x"));
    }

    #[test]
    fn scenario_authority_save_does_not_write_bevy_render_metadata() {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("no-bevy.simthing-scenario.json");
        let session = generated_session();
        save_current_session_scenario_to_path(&session, &path).expect("save");
        let text = std::fs::read_to_string(&path).expect("read");
        assert!(!text.contains("Bevy"));
        assert!(!text.contains("world_position"));
        assert!(!text.contains("sprite_scale"));
        assert!(!text.contains("emissive_strength"));
        assert!(!text.contains("depth_bucket"));
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

    #[test]
    fn scenario_authority_load_validates_stead_mapping() {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("invalid-stead.simthing-scenario.json");
        write_small_scenario(&path);
        let mut scenario = load_scenario_authority_from_path(&path).expect("load valid");
        scenario.structural_grid.placements.clear();
        scenario.structural_grid.frame.occupied_cells = 0;
        save_scenario_authority_to_path(&path, &scenario).expect("overwrite invalid");
        let err = load_scenario_authority_from_path(&path).expect_err("reject invalid STEAD");
        assert!(matches!(
            err,
            ScenarioIoError::Serde(ScenarioSerdeError::Validation(
                SteadMappingError::MissingGridcellLocation(_)
            ))
        ));
    }

    #[test]
    fn scenario_authority_load_rejects_invalid_map_container_id() {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("bad-map.simthing-scenario.json");
        write_small_scenario(&path);
        let mut scenario = load_scenario_authority_from_path(&path).expect("load valid");
        scenario.structural_grid.map_container_id = "99999999".to_string();
        save_scenario_authority_to_path(&path, &scenario).expect("overwrite invalid");
        let err = load_scenario_authority_from_path(&path).expect_err("reject dangling map id");
        assert!(matches!(
            err,
            ScenarioIoError::Serde(ScenarioSerdeError::Validation(
                SteadMappingError::DanglingMapContainerId(_)
            ))
        ));
    }

    #[test]
    fn scenario_authority_load_rejects_invalid_links() {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("bad-links.simthing-scenario.json");
        let mut scenario = small_scenario();
        scenario.links.push(SimThingScenarioLink {
            from_system_id: "1".to_string(),
            to_system_id: "42".to_string(),
        });
        save_scenario_authority_to_path(&path, &scenario).expect("save invalid links");
        let err = load_scenario_authority_from_path(&path).expect_err("reject invalid links");
        assert!(matches!(
            err,
            ScenarioIoError::Serde(ScenarioSerdeError::LinkValidation(_))
        ));
    }

    #[test]
    fn scenario_authority_load_reserves_simthing_ids() {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("reserve.simthing-scenario.json");
        write_small_scenario(&path);
        load_scenario_authority_from_path(&path).expect("load reserves ids");
        let spawned = SimThing::new(SimThingKind::Cohort, 0);
        let scenario = load_scenario_authority_from_path(&path).expect("reload");
        let max_existing = scenario.root.max_id_in_subtree().raw();
        assert!(spawned.id.raw() > max_existing);
    }

    #[test]
    fn new_simthing_after_scenario_load_does_not_collide() {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("collision.simthing-scenario.json");
        write_small_scenario(&path);
        let scenario = load_scenario_authority_from_path(&path).expect("load");
        let existing: BTreeSet<u32> = scenario
            .gridcell_locations()
            .map(|gridcell| gridcell.id.raw())
            .collect();
        reserve_simthing_ids_from_scenario(&scenario).expect("reserve");
        let spawned = SimThing::new(SimThingKind::Location, 0);
        assert!(!existing.contains(&spawned.id.raw()));
    }

    #[test]
    fn loaded_scenario_rebuilds_studio_hydration_boundary() {
        let session = generated_session();
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("hydration.simthing-scenario.json");
        save_current_session_scenario_to_path(&session, &path).expect("save");
        let loaded =
            load_studio_session_from_scenario_path(&path, Some(session.profile())).expect("load");
        assert_eq!(
            loaded.hydration.grid.occupied_cells,
            session.hydration.grid.occupied_cells
        );
        assert_eq!(
            loaded.hydration.simthing_spec_scenario_id,
            session.scenario_authority.scenario_id
        );
    }

    #[test]
    fn loaded_scenario_rebuilds_studio_view_model() {
        let session = generated_session();
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("vm.simthing-scenario.json");
        save_current_session_scenario_to_path(&session, &path).expect("save");
        let loaded =
            load_studio_session_from_scenario_path(&path, Some(session.profile())).expect("load");
        assert_eq!(
            loaded.view_model.stars.len(),
            session.view_model.stars.len()
        );
        assert_eq!(
            loaded.view_model.hyperlanes.len(),
            session.view_model.hyperlanes.len()
        );
    }

    #[test]
    fn loaded_scenario_projection_uses_structural_coords_not_render_coords() {
        let session = generated_session();
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("structural.simthing-scenario.json");
        save_current_session_scenario_to_path(&session, &path).expect("save");
        let loaded =
            load_studio_session_from_scenario_path(&path, Some(session.profile())).expect("load");
        for (star, placement) in loaded
            .view_model
            .stars
            .iter()
            .zip(loaded.scenario_authority.structural_grid.placements.iter())
        {
            assert_eq!(star.structural_col, placement.col);
            assert_eq!(star.structural_row, placement.row);
            assert_eq!(star.system_id, placement.system_id);
        }
        assert_eq!(
            loaded.view_model.structural_only_note,
            StudioGalaxyViewModel::RENDER_ONLY_NOTE
        );
    }

    #[test]
    fn studio_config_remains_presentation_only_after_scenario_io() {
        let dir = TempDir::new().expect("tempdir");
        let scenario_path = dir.path().join("after-io.simthing-scenario.json");
        let config_path = dir.path().join(STUDIO_CONFIG_FILE_NAME);
        let session = generated_session();
        save_current_session_scenario_to_path(&session, &scenario_path).expect("save");
        save_studio_config_to_path(&config_path, &SimThingStudioConfig::default())
            .expect("save config");
        load_studio_session_from_scenario_path(&scenario_path, Some(session.profile()))
            .expect("load scenario");
        let config_text = std::fs::read_to_string(&config_path).expect("read config");
        assert!(!config_text.contains("structural_grid"));
        assert!(!config_text.contains("map_container_id"));
        assert!(!config_text.contains("SimThingScenarioSpec"));
    }

    #[test]
    fn model_edit_then_save_preserves_authority_roundtrip() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let mut scenario = generate_simthing_spec_scenario(&output).expect("authority");
        let gridcell_raw = scenario.structural_grid.placements[0].simthing_id_raw;
        scenario.structural_grid.placements[0].col = 7;
        apply_gridcell_property_edit(
            &mut scenario,
            gridcell_raw,
            SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
            structural_property_value_u32(7),
        )
        .expect("edit");
        validate_stead_mapping_consistency(&scenario).expect("valid after edit");
        validate_scenario_links(&scenario).expect("valid links");

        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("edited.simthing-scenario.json");
        save_scenario_authority_to_path(&path, &scenario).expect("save edited");
        let loaded = load_scenario_authority_from_path(&path).expect("load edited");
        assert_eq!(loaded.structural_grid.placements[0].col, 7);
    }
}
