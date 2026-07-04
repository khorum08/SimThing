//! Native/injectable scenario file picker and path canonicalization (presentation only).

use std::path::{Path, PathBuf};

use crate::scenario_io::SCENARIO_FILE_SUFFIX;

pub const LOAD_SCENARIO_DIALOG_TITLE: &str = "Load SimThing Scenario";
pub const SCENARIO_DIALOG_FILTER_NAME: &str = "SimThing Scenario";
pub const SCENARIO_DIALOG_FILTER_EXT: &str = "simthing-scenario.json";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScenarioPickerOutcome {
    Selected(PathBuf),
    Cancelled,
}

pub trait ScenarioFilePicker {
    fn pick_open_scenario(&self, start_dir: &Path) -> ScenarioPickerOutcome;
}

pub struct NativeScenarioFilePicker;

impl ScenarioFilePicker for NativeScenarioFilePicker {
    fn pick_open_scenario(&self, start_dir: &Path) -> ScenarioPickerOutcome {
        let dialog = rfd::FileDialog::new()
            .set_title(LOAD_SCENARIO_DIALOG_TITLE)
            .add_filter(SCENARIO_DIALOG_FILTER_NAME, &[SCENARIO_DIALOG_FILTER_EXT])
            .set_directory(start_dir);
        match dialog.pick_file() {
            Some(path) => ScenarioPickerOutcome::Selected(path),
            None => ScenarioPickerOutcome::Cancelled,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FakeScenarioFilePicker {
    pub outcome: ScenarioPickerOutcome,
}

impl ScenarioFilePicker for FakeScenarioFilePicker {
    fn pick_open_scenario(&self, _start_dir: &Path) -> ScenarioPickerOutcome {
        self.outcome.clone()
    }
}

pub fn default_picker_start_directory(current_path_text: &str) -> PathBuf {
    let trimmed = current_path_text.trim();
    if !trimmed.is_empty() {
        let candidate = PathBuf::from(trimmed);
        if let Some(parent) = candidate.parent() {
            if parent.is_dir() {
                return parent.to_path_buf();
            }
        }
        if candidate.is_dir() {
            return candidate;
        }
    }
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

/// Canonicalize for display/load when possible; fall back to absolute path.
pub fn canonicalize_scenario_display_path(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| {
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()
                .map(|cwd| cwd.join(path))
                .unwrap_or_else(|_| path.to_path_buf())
        }
    })
}

pub fn scenario_path_has_valid_suffix(path: &Path) -> bool {
    path.to_string_lossy().ends_with(SCENARIO_FILE_SUFFIX)
}

#[cfg(test)]
mod tests {
    use super::*;

}
