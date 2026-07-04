//! Studio UI actions for SimThing-Spec scenario save/load (presentation layer over `scenario_io`).

use std::path::{Path, PathBuf};

use crate::scenario_io::{
    load_studio_session_from_scenario_path, save_current_session_scenario_to_path,
    SCENARIO_FILE_SUFFIX,
};
use crate::session::StudioSession;
use crate::studio_config::STUDIO_CONFIG_FILE_NAME;
use crate::studio_scenario_load::{
    canonicalize_scenario_display_path, default_picker_start_directory, NativeScenarioFilePicker,
    ScenarioFilePicker, ScenarioPickerOutcome,
};

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
/// Populate the scenario path text field programmatically (presentation/session state only).
pub fn set_programmatic_scenario_path(
    state: &mut StudioAppState,
    path: impl AsRef<Path>,
) -> Result<(), String> {
    let display = canonicalize_scenario_display_path(path.as_ref());
    validate_scenario_path_text(display.to_string_lossy().as_ref())?;
    state.scenario_path_text = display.display().to_string();
    Ok(())
}

#[derive(Debug, Clone)]
pub enum ScenarioPickerActionResult {
    Cancelled,
    InvalidPath {
        message: String,
    },
    Loaded {
        session: StudioSession,
        message: String,
    },
    Failed {
        message: String,
    },
}

pub fn open_native_scenario_load_picker(state: &mut StudioAppState) -> ScenarioPickerActionResult {
    load_scenario_with_picker(state, &NativeScenarioFilePicker)
}

pub fn load_scenario_with_picker<P: ScenarioFilePicker>(
    state: &mut StudioAppState,
    picker: &P,
) -> ScenarioPickerActionResult {
    let path_before = state.scenario_path_text.clone();
    let start_dir = default_picker_start_directory(&path_before);
    match picker.pick_open_scenario(&start_dir) {
        ScenarioPickerOutcome::Cancelled => ScenarioPickerActionResult::Cancelled,
        ScenarioPickerOutcome::Selected(selected) => {
            apply_picker_selection_and_load(state, &selected, &path_before)
        }
    }
}

pub fn load_scenario_manual_path_action(state: &mut StudioAppState) -> ScenarioActionResult {
    let path = match scenario_path_from_state(state) {
        Ok(path) => path,
        Err(reason) => {
            let message = format!("Scenario load failed: {reason}");
            record_scenario_io_status(state, message.clone());
            return ScenarioActionResult::InvalidPath { message };
        }
    };
    load_scenario_action(state, &path)
}

fn apply_picker_selection_and_load(
    state: &mut StudioAppState,
    selected: &Path,
    path_before: &str,
) -> ScenarioPickerActionResult {
    let display_path = canonicalize_scenario_display_path(selected);
    match validate_scenario_path_text(display_path.to_string_lossy().as_ref()) {
        Err(reason) => {
            state.scenario_path_text = path_before.to_string();
            let message = format!("Scenario load failed: {reason}");
            record_scenario_io_status(state, message.clone());
            ScenarioPickerActionResult::InvalidPath { message }
        }
        Ok(validated) => {
            state.scenario_path_text = validated.display().to_string();
            match load_scenario_action(state, &validated) {
                ScenarioActionResult::Loaded { session, message } => {
                    ScenarioPickerActionResult::Loaded { session, message }
                }
                ScenarioActionResult::Failed { message } => {
                    ScenarioPickerActionResult::Failed { message }
                }
                ScenarioActionResult::InvalidPath { message } => {
                    state.scenario_path_text = path_before.to_string();
                    ScenarioPickerActionResult::InvalidPath { message }
                }
                ScenarioActionResult::NoActiveSession { message }
                | ScenarioActionResult::Saved { message } => {
                    state.scenario_path_text = path_before.to_string();
                    ScenarioPickerActionResult::Failed { message }
                }
            }
        }
    }
}

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

    use crate::studio_scenario_load::{FakeScenarioFilePicker, ScenarioPickerOutcome};

    fn seed_scenario_file(name: &str) -> (tempfile::TempDir, PathBuf) {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join(name);
        let original = state_with_session();
        save_scenario_authority_to_path(
            &path,
            &original.session.as_ref().unwrap().scenario_authority,
        )
        .expect("seed");
        (dir, path)
    }

}
