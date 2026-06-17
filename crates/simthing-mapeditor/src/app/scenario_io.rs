//! Studio UI actions for SimThing-Spec scenario save/load (presentation layer over `scenario_io`).

use std::path::{Path, PathBuf};

use crate::scenario_io::{
    load_studio_session_from_scenario_path, save_current_session_scenario_to_path,
    SCENARIO_FILE_SUFFIX,
};
use crate::session::StudioSession;
use crate::studio_config::STUDIO_CONFIG_FILE_NAME;

use super::StudioAppState;

pub const DEFAULT_SCENARIO_PATH: &str = "simthing-current.simthing-scenario.json";
const SETTINGS_RON_FILE_NAME: &str = "settings.ron";

#[derive(Debug, Clone)]
pub enum ScenarioActionResult {
    Saved {
        message: String,
    },
    Loaded {
        session: StudioSession,
        message: String,
    },
    NoActiveSession {
        message: String,
    },
    InvalidPath {
        message: String,
    },
    Failed {
        message: String,
    },
}

impl ScenarioActionResult {
    pub fn message(&self) -> &str {
        match self {
            Self::Saved { message }
            | Self::Loaded { message, .. }
            | Self::NoActiveSession { message }
            | Self::InvalidPath { message }
            | Self::Failed { message } => message,
        }
    }

    pub fn is_success(&self) -> bool {
        matches!(self, Self::Saved { .. } | Self::Loaded { .. })
    }
}

/// Validates scenario path text for UI save/load. Rejects studio config and settings paths.
pub fn validate_scenario_path_text(path_text: &str) -> Result<PathBuf, String> {
    let trimmed = path_text.trim();
    if trimmed.is_empty() {
        return Err("scenario path is empty".into());
    }
    let file_name = Path::new(trimmed)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(trimmed);
    if file_name == STUDIO_CONFIG_FILE_NAME {
        return Err(format!(
            "scenario path cannot be {STUDIO_CONFIG_FILE_NAME} (studio config is presentation-only)"
        ));
    }
    if file_name == SETTINGS_RON_FILE_NAME {
        return Err(format!(
            "scenario path cannot be {SETTINGS_RON_FILE_NAME} (window/generation metadata only)"
        ));
    }
    if !trimmed.ends_with(SCENARIO_FILE_SUFFIX) {
        return Err(format!(
            "scenario path must use the {SCENARIO_FILE_SUFFIX} suffix"
        ));
    }
    Ok(PathBuf::from(trimmed))
}

pub fn scenario_path_from_state(state: &StudioAppState) -> Result<PathBuf, String> {
    validate_scenario_path_text(&state.scenario_path_text)
}

fn record_scenario_io_status(state: &mut StudioAppState, message: String) {
    state.last_scenario_io_status = message.clone();
    state.status_message = message;
}

pub fn save_scenario_action(state: &mut StudioAppState, path: &Path) -> ScenarioActionResult {
    let path = match validate_scenario_path_text(path.to_string_lossy().as_ref()) {
        Ok(path) => path,
        Err(reason) => {
            let message = format!("Scenario save failed: {reason}");
            record_scenario_io_status(state, message.clone());
            return ScenarioActionResult::InvalidPath { message };
        }
    };

    let Some(session) = state.session.as_ref() else {
        let message = "Scenario save failed: no active session".to_string();
        record_scenario_io_status(state, message.clone());
        return ScenarioActionResult::NoActiveSession { message };
    };

    match save_current_session_scenario_to_path(session, &path) {
        Ok(()) => {
            let message = format!("Scenario saved: {}", path.display());
            record_scenario_io_status(state, message.clone());
            ScenarioActionResult::Saved { message }
        }
        Err(err) => {
            let message = format!("Scenario save failed: {err}");
            record_scenario_io_status(state, message.clone());
            ScenarioActionResult::Failed { message }
        }
    }
}

/// Loads scenario authority from disk. On success returns a new session for the caller to adopt.
/// On failure the current `state.session` is left unchanged.
pub fn load_scenario_action(state: &mut StudioAppState, path: &Path) -> ScenarioActionResult {
    let path = match validate_scenario_path_text(path.to_string_lossy().as_ref()) {
        Ok(path) => path,
        Err(reason) => {
            let message = format!("Scenario load failed: {reason}");
            record_scenario_io_status(state, message.clone());
            return ScenarioActionResult::InvalidPath { message };
        }
    };

    match load_studio_session_from_scenario_path(&path, Some(state.profile.clone())) {
        Ok(session) => {
            let message = format!("Scenario loaded: {}", path.display());
            state.last_scenario_io_status = message.clone();
            ScenarioActionResult::Loaded { session, message }
        }
        Err(err) => {
            let message = format!("Scenario load failed: {err}");
            record_scenario_io_status(state, message.clone());
            ScenarioActionResult::Failed { message }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;
    use crate::generation::{run_generation, GenerationProfile};
    use crate::scenario_io::save_scenario_authority_to_path;
    use crate::session::{StudioSession, StudioSessionSource};
    use crate::studio_config::{
        save_studio_config_to_path, SimThingStudioConfig, STUDIO_CONFIG_FILE_NAME,
        STUDIO_CONFIG_SCHEMA_VERSION,
    };

    fn session_with_authority() -> StudioSession {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        StudioSession::from_generation(profile, output).expect("session")
    }

    fn state_with_session() -> StudioAppState {
        let mut state = StudioAppState::default();
        state.session = Some(session_with_authority());
        state
    }

    #[test]
    fn scenario_path_defaults_to_simthing_scenario_suffix() {
        let state = StudioAppState::default();
        assert_eq!(state.scenario_path_text, DEFAULT_SCENARIO_PATH);
        assert!(state.scenario_path_text.ends_with(SCENARIO_FILE_SUFFIX));
        validate_scenario_path_text(&state.scenario_path_text).expect("default path valid");
    }

    #[test]
    fn scenario_path_rejects_or_warns_non_scenario_config_path() {
        let err = validate_scenario_path_text(STUDIO_CONFIG_FILE_NAME).expect_err("reject config");
        assert!(err.contains(STUDIO_CONFIG_FILE_NAME));
        let err = validate_scenario_path_text("settings.ron").expect_err("reject ron");
        assert!(err.contains("settings.ron"));
        let err = validate_scenario_path_text("galaxy.json").expect_err("reject wrong suffix");
        assert!(err.contains(SCENARIO_FILE_SUFFIX));
    }

    #[test]
    fn scenario_save_ui_requires_active_session() {
        let mut state = StudioAppState::default();
        state.session = None;
        let path = PathBuf::from(DEFAULT_SCENARIO_PATH);
        let result = save_scenario_action(&mut state, &path);
        assert!(matches!(
            result,
            ScenarioActionResult::NoActiveSession { .. }
        ));
        assert!(state.last_scenario_io_status.contains("no active session"));
    }

    #[test]
    fn scenario_save_ui_writes_simthing_scenario_file() {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("ui-save.simthing-scenario.json");
        let mut state = state_with_session();
        let result = save_scenario_action(&mut state, &path);
        assert!(matches!(result, ScenarioActionResult::Saved { .. }));
        assert!(path.exists());
        let text = fs::read_to_string(&path).expect("read scenario");
        assert!(text.contains("structural_grid"));
        assert!(text.contains("scenario_id"));
    }

    #[test]
    fn scenario_save_ui_does_not_write_studio_config() {
        let dir = TempDir::new().expect("tempdir");
        let scenario_path = dir.path().join("sep.simthing-scenario.json");
        let config_path = dir.path().join(STUDIO_CONFIG_FILE_NAME);
        let mut state = state_with_session();
        assert!(save_scenario_action(&mut state, &scenario_path).is_success());
        save_studio_config_to_path(&config_path, &SimThingStudioConfig::default())
            .expect("config save");
        let scenario_text = fs::read_to_string(&scenario_path).expect("read scenario");
        assert!(!scenario_text.contains("settings_dialog"));
        assert!(!scenario_text.contains(&format!(
            "\"schema_version\":{STUDIO_CONFIG_SCHEMA_VERSION}"
        )));
    }

    #[test]
    fn scenario_save_ui_reports_success() {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("ok.simthing-scenario.json");
        let mut state = state_with_session();
        let result = save_scenario_action(&mut state, &path);
        assert!(result.is_success());
        assert!(state.status_message.contains("Scenario saved:"));
        assert_eq!(state.last_scenario_io_status, state.status_message);
    }

    #[test]
    fn scenario_save_ui_reports_failure_without_panicking() {
        let mut state = state_with_session();
        let result = save_scenario_action(&mut state, Path::new(STUDIO_CONFIG_FILE_NAME));
        assert!(!result.is_success());
        assert!(state.status_message.contains("Scenario save failed:"));
    }

    #[test]
    fn scenario_load_ui_loads_simthing_scenario_authority() {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("load.simthing-scenario.json");
        let original = state_with_session();
        let authority = original
            .session
            .as_ref()
            .unwrap()
            .scenario_authority
            .clone();
        save_scenario_authority_to_path(&path, &authority).expect("seed file");

        let mut state = StudioAppState::default();
        let result = load_scenario_action(&mut state, &path);
        let ScenarioActionResult::Loaded { session, .. } = result else {
            panic!("expected loaded session, got {result:?}");
        };
        assert_eq!(
            session.scenario_authority.scenario_id,
            authority.scenario_id
        );
        assert_eq!(
            session.scenario_authority.structural_grid,
            authority.structural_grid
        );
    }

    #[test]
    fn scenario_load_ui_rebuilds_hydration_projection() {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("hydrate.simthing-scenario.json");
        let original = state_with_session();
        let session = original.session.as_ref().unwrap();
        save_scenario_authority_to_path(&path, &session.scenario_authority).expect("seed");

        let mut state = StudioAppState::default();
        let ScenarioActionResult::Loaded {
            session: loaded, ..
        } = load_scenario_action(&mut state, &path)
        else {
            panic!("expected load success");
        };
        assert_eq!(
            loaded.hydration.grid.occupied_cells,
            session.hydration.grid.occupied_cells
        );
    }

    #[test]
    fn scenario_load_ui_rebuilds_view_model() {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("vm.simthing-scenario.json");
        let original = state_with_session();
        let session = original.session.as_ref().unwrap();
        save_scenario_authority_to_path(&path, &session.scenario_authority).expect("seed");

        let mut state = StudioAppState::default();
        let ScenarioActionResult::Loaded {
            session: loaded, ..
        } = load_scenario_action(&mut state, &path)
        else {
            panic!("expected load success");
        };
        assert_eq!(
            loaded.view_model.stars.len(),
            session.view_model.stars.len()
        );
    }

    #[test]
    fn scenario_load_ui_preserves_current_session_on_failure() {
        let mut state = state_with_session();
        let before_id = state
            .session
            .as_ref()
            .unwrap()
            .scenario_authority
            .scenario_id
            .clone();
        let missing = PathBuf::from("missing-file.simthing-scenario.json");
        let result = load_scenario_action(&mut state, &missing);
        assert!(matches!(result, ScenarioActionResult::Failed { .. }));
        assert_eq!(
            state
                .session
                .as_ref()
                .unwrap()
                .scenario_authority
                .scenario_id,
            before_id
        );
    }

    #[test]
    fn scenario_load_ui_reports_success() {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("success.simthing-scenario.json");
        let original = state_with_session();
        save_scenario_authority_to_path(
            &path,
            &original.session.as_ref().unwrap().scenario_authority,
        )
        .expect("seed");

        let mut state = StudioAppState::default();
        let result = load_scenario_action(&mut state, &path);
        assert!(matches!(result, ScenarioActionResult::Loaded { .. }));
        assert!(result.message().contains("Scenario loaded:"));
    }

    #[test]
    fn scenario_load_ui_reports_failure() {
        let mut state = state_with_session();
        let result = load_scenario_action(&mut state, Path::new("bad.json"));
        assert!(matches!(result, ScenarioActionResult::InvalidPath { .. }));
        assert!(state.status_message.contains("Scenario load failed:"));
    }

    #[test]
    fn scenario_io_status_is_presentation_only() {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("status.simthing-scenario.json");
        let mut state = state_with_session();
        assert!(save_scenario_action(&mut state, &path).is_success());
        assert!(state.last_scenario_io_status.contains("Scenario saved:"));
        let text = fs::read_to_string(&path).expect("read");
        assert!(!text.contains("last_scenario_io_status"));
        assert!(!text.contains("scenario_path_text"));
    }

    #[test]
    fn loaded_session_authority_is_loaded_scenario_not_synthetic_output() {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("authority.simthing-scenario.json");
        let original = state_with_session();
        let authority = original
            .session
            .as_ref()
            .unwrap()
            .scenario_authority
            .clone();
        save_scenario_authority_to_path(&path, &authority).expect("seed");

        let mut state = StudioAppState::default();
        let ScenarioActionResult::Loaded { session, .. } = load_scenario_action(&mut state, &path)
        else {
            panic!("expected load");
        };
        assert_eq!(
            session.scenario_authority.scenario_id,
            authority.scenario_id
        );
        assert_eq!(
            session.scenario_authority.structural_grid,
            authority.structural_grid
        );
        assert!(session.is_loaded_scenario());
        assert!(session.generated_output.is_none());
    }

    #[test]
    fn save_load_ui_preserves_loaded_session_source() {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("source-preserve.simthing-scenario.json");
        let original = state_with_session();
        save_scenario_authority_to_path(
            &path,
            &original.session.as_ref().unwrap().scenario_authority,
        )
        .expect("seed");

        let mut state = StudioAppState::default();
        let ScenarioActionResult::Loaded { session, .. } = load_scenario_action(&mut state, &path)
        else {
            panic!("expected load success");
        };
        assert!(session.is_loaded_scenario());
        assert!(!session.is_generated());
        assert!(session.generated_output.is_none());
        assert!(matches!(
            session.source,
            StudioSessionSource::LoadedScenario { .. }
        ));
    }

    #[test]
    fn load_failure_preserves_session_source() {
        let mut state = state_with_session();
        assert!(state.session.as_ref().unwrap().is_generated());
        let before_source = state.session.as_ref().unwrap().source.clone();
        let missing = PathBuf::from("missing-source.simthing-scenario.json");
        let result = load_scenario_action(&mut state, &missing);
        assert!(matches!(result, ScenarioActionResult::Failed { .. }));
        let after = state.session.as_ref().unwrap();
        assert!(after.is_generated());
        assert_eq!(after.source, before_source);
        assert!(after.generated_output.is_some());
    }
}
