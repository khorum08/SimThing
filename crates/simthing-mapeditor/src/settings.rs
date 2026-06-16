//! Persistent editor settings (RON).

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::camera_control::OrbitCameraState;
use crate::generation::GenerationProfile;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PersistedCameraState {
    pub orbit_yaw: f32,
    pub orbit_pitch: f32,
    pub orbit_distance: f32,
    pub orbit_target: [f32; 3],
    pub overhead: bool,
}

impl Default for PersistedCameraState {
    fn default() -> Self {
        let orbit = OrbitCameraState::default();
        Self {
            orbit_yaw: orbit.orbit_yaw,
            orbit_pitch: orbit.orbit_pitch,
            orbit_distance: orbit.orbit_distance,
            orbit_target: orbit.orbit_target,
            overhead: orbit.overhead,
        }
    }
}

impl From<&OrbitCameraState> for PersistedCameraState {
    fn from(value: &OrbitCameraState) -> Self {
        Self {
            orbit_yaw: value.orbit_yaw,
            orbit_pitch: value.orbit_pitch,
            orbit_distance: value.orbit_distance,
            orbit_target: value.orbit_target,
            overhead: value.overhead,
        }
    }
}

impl From<PersistedCameraState> for OrbitCameraState {
    fn from(value: PersistedCameraState) -> Self {
        Self {
            orbit_yaw: value.orbit_yaw,
            orbit_pitch: value.orbit_pitch,
            orbit_distance: value.orbit_distance,
            orbit_target: value.orbit_target,
            overhead: value.overhead,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowModeSetting {
    Windowed,
    BorderlessFullscreen,
    ExclusiveFullscreen,
}

impl Default for WindowModeSetting {
    fn default() -> Self {
        Self::BorderlessFullscreen
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditorSettings {
    pub window_mode: WindowModeSetting,
    pub exclusive_fullscreen_enabled: bool,
    pub last_window_size: [u32; 2],
    pub last_panel_width: f32,
    pub left_panel_collapsed: bool,
    pub last_generation_params: GenerationProfile,
    pub last_session_path: Option<PathBuf>,
    pub last_report_path: Option<PathBuf>,
    pub last_scenario_path: Option<PathBuf>,
    pub camera_preset: String,
    pub last_selected_system_id: Option<u32>,
    pub last_camera: PersistedCameraState,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            window_mode: WindowModeSetting::default(),
            exclusive_fullscreen_enabled: false,
            last_window_size: [1600, 900],
            last_panel_width: 0.20,
            left_panel_collapsed: false,
            last_generation_params: GenerationProfile::default_spiral_2_dense_3000(),
            last_session_path: None,
            last_report_path: None,
            last_scenario_path: None,
            camera_preset: "three_quarter".into(),
            last_selected_system_id: None,
            last_camera: PersistedCameraState::default(),
        }
    }
}

impl EditorSettings {
    pub fn settings_path() -> PathBuf {
        if let Some(base) = std::env::var_os("APPDATA") {
            PathBuf::from(base)
                .join("SimThing")
                .join("Studio")
                .join("settings.ron")
        } else {
            PathBuf::from("settings.ron")
        }
    }

    pub fn load() -> Self {
        let path = Self::settings_path();
        if !path.exists() {
            return Self::default();
        }
        match std::fs::read_to_string(&path) {
            Ok(text) => ron::from_str(&text).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) -> Result<(), SettingsError> {
        let path = Self::settings_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let text =
            ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::new().compact_arrays(true))
                .map_err(|err| SettingsError::Serialize(err.to_string()))?;
        std::fs::write(path, text)?;
        Ok(())
    }

    pub fn remember_paths(
        &mut self,
        session: Option<&Path>,
        report: Option<&Path>,
        scenario: Option<&Path>,
    ) {
        self.last_session_path = session.map(Path::to_path_buf);
        self.last_report_path = report.map(Path::to_path_buf);
        self.last_scenario_path = scenario.map(Path::to_path_buf);
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SettingsError {
    #[error("failed to read/write settings: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to serialize settings: {0}")]
    Serialize(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn editor_settings_default_roundtrip() {
        let settings = EditorSettings::default();
        let text = ron::ser::to_string_pretty(&settings, Default::default()).expect("serialize");
        let parsed: EditorSettings = ron::from_str(&text).expect("deserialize");
        assert_eq!(settings.window_mode, parsed.window_mode);
        assert_eq!(
            settings.last_generation_params.shape,
            parsed.last_generation_params.shape
        );
    }

    #[test]
    fn editor_settings_remembers_last_paths() {
        let mut settings = EditorSettings::default();
        settings.remember_paths(
            Some(Path::new("sessions/last.ron")),
            Some(Path::new("reports/last.json")),
            Some(Path::new("scenarios/last.txt")),
        );
        assert_eq!(
            settings.last_session_path.as_deref(),
            Some(Path::new("sessions/last.ron"))
        );
        assert_eq!(
            settings.last_report_path.as_deref(),
            Some(Path::new("reports/last.json"))
        );
        assert_eq!(
            settings.last_scenario_path.as_deref(),
            Some(Path::new("scenarios/last.txt"))
        );
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("settings.ron");
        let text = ron::ser::to_string_pretty(&settings, Default::default()).expect("serialize");
        std::fs::write(&path, text).expect("write");
        let loaded: EditorSettings =
            ron::from_str(&std::fs::read_to_string(path).expect("read")).expect("parse");
        assert_eq!(loaded.last_report_path, settings.last_report_path);
    }
}
