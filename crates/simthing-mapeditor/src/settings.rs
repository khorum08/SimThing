//! Persistent editor settings (RON) and Studio presentation config (`simthing-studio-config.json`).
//!
//! `SimThingStudioConfig` in `studio_config.rs` is presentation-only. Scenario/model authority
//! remains in `SimThingScenarioSpec` and is not stored in the Studio config file.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::camera_control::OrbitCameraState;
use crate::generation::GenerationProfile;
use crate::hyperlane_buckets::HyperlaneRenderSettings;
use crate::star_render::{StarFalloffSettings, StarNameplateSettings, StarRenderMode};

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
    #[serde(default = "default_base_star_blur_radius")]
    pub base_star_blur_radius: f32,
    #[serde(default = "default_falloff_distance_percent")]
    pub falloff_distance_percent: f32,
    #[serde(default = "default_falloff_star_blur_radius_percent")]
    pub falloff_star_blur_radius_percent: f32,
    #[serde(default = "default_falloff_star_opacity_percent")]
    pub falloff_star_opacity_percent: f32,
    #[serde(default)]
    pub star_render_mode: StarRenderMode,
    #[serde(default = "default_nameplate_relative_width_percent")]
    pub nameplate_relative_width_percent: f32,
    #[serde(default = "default_nameplate_base_transparency_percent")]
    pub nameplate_base_transparency_percent: f32,
    #[serde(default = "default_nameplate_relative_falloff_distance_percent")]
    pub nameplate_relative_falloff_distance_percent: f32,
    #[serde(default = "default_nameplate_relative_falloff_transparency_percent")]
    pub nameplate_relative_falloff_transparency_percent: f32,
    #[serde(default = "default_base_hyperlane_thickness_percent")]
    pub base_hyperlane_thickness_percent: f32,
    #[serde(default = "default_base_hyperlane_opacity_percent")]
    pub base_hyperlane_opacity_percent: f32,
    #[serde(default = "default_hyperlane_falloff_distance_percent")]
    pub hyperlane_falloff_distance_percent: f32,
    #[serde(default = "default_hyperlane_falloff_thickness_percent")]
    pub hyperlane_falloff_thickness_percent: f32,
    #[serde(default = "default_hyperlane_falloff_opacity_percent")]
    pub hyperlane_falloff_opacity_percent: f32,
    #[serde(default = "default_settings_dialog_position")]
    pub settings_dialog_position: [f32; 2],
    #[serde(default)]
    pub settings_dialog_visible: bool,
    #[serde(default)]
    pub antialiasing_mode: crate::studio_antialiasing::StudioAntialiasingMode,
}

impl Default for EditorSettings {
    fn default() -> Self {
        let star = StarFalloffSettings::default();
        let nameplate = StarNameplateSettings::default();
        let hyperlane = HyperlaneRenderSettings::default();
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
            base_star_blur_radius: star.base_blur_radius,
            falloff_distance_percent: star.falloff_distance_percent,
            falloff_star_blur_radius_percent: star.falloff_blur_radius_percent,
            falloff_star_opacity_percent: star.falloff_opacity_percent,
            star_render_mode: StarRenderMode::default(),
            nameplate_relative_width_percent: nameplate.relative_width_percent,
            nameplate_base_transparency_percent: nameplate.base_transparency_percent,
            nameplate_relative_falloff_distance_percent: nameplate
                .relative_falloff_distance_percent,
            nameplate_relative_falloff_transparency_percent: nameplate
                .relative_falloff_transparency_percent,
            base_hyperlane_thickness_percent: hyperlane.base_thickness_percent_of_star,
            base_hyperlane_opacity_percent: hyperlane.base_opacity_percent,
            hyperlane_falloff_distance_percent: hyperlane.falloff_distance_percent,
            hyperlane_falloff_thickness_percent: hyperlane.falloff_thickness_percent,
            hyperlane_falloff_opacity_percent: hyperlane.falloff_opacity_percent,
            settings_dialog_position: default_settings_dialog_position(),
            settings_dialog_visible: false,
            antialiasing_mode: crate::studio_antialiasing::StudioAntialiasingMode::Off,
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

    pub fn star_falloff_settings(&self) -> StarFalloffSettings {
        StarFalloffSettings {
            base_blur_radius: self.base_star_blur_radius,
            falloff_distance_percent: self.falloff_distance_percent,
            falloff_blur_radius_percent: self.falloff_star_blur_radius_percent,
            falloff_opacity_percent: self.falloff_star_opacity_percent,
        }
        .clamped()
    }

    pub fn set_star_falloff_settings(&mut self, settings: StarFalloffSettings) {
        let settings = settings.clamped();
        self.base_star_blur_radius = settings.base_blur_radius;
        self.falloff_distance_percent = settings.falloff_distance_percent;
        self.falloff_star_blur_radius_percent = settings.falloff_blur_radius_percent;
        self.falloff_star_opacity_percent = settings.falloff_opacity_percent;
    }

    pub fn star_render_mode(&self) -> StarRenderMode {
        self.star_render_mode
    }

    pub fn set_star_render_mode(&mut self, mode: StarRenderMode) {
        self.star_render_mode = mode;
    }

    pub fn star_nameplate_settings(&self) -> StarNameplateSettings {
        StarNameplateSettings {
            relative_width_percent: self.nameplate_relative_width_percent,
            base_transparency_percent: self.nameplate_base_transparency_percent,
            relative_falloff_distance_percent: self.nameplate_relative_falloff_distance_percent,
            relative_falloff_transparency_percent: self
                .nameplate_relative_falloff_transparency_percent,
        }
        .clamped()
    }

    pub fn set_star_nameplate_settings(&mut self, settings: StarNameplateSettings) {
        let settings = settings.clamped();
        self.nameplate_relative_width_percent = settings.relative_width_percent;
        self.nameplate_base_transparency_percent = settings.base_transparency_percent;
        self.nameplate_relative_falloff_distance_percent =
            settings.relative_falloff_distance_percent;
        self.nameplate_relative_falloff_transparency_percent =
            settings.relative_falloff_transparency_percent;
    }

    pub fn hyperlane_render_settings(&self) -> HyperlaneRenderSettings {
        HyperlaneRenderSettings {
            base_thickness_percent_of_star: self.base_hyperlane_thickness_percent,
            base_opacity_percent: self.base_hyperlane_opacity_percent,
            falloff_distance_percent: self.hyperlane_falloff_distance_percent,
            falloff_thickness_percent: self.hyperlane_falloff_thickness_percent,
            falloff_opacity_percent: self.hyperlane_falloff_opacity_percent,
        }
        .clamped()
    }

    pub fn set_hyperlane_render_settings(&mut self, settings: HyperlaneRenderSettings) {
        let settings = settings.clamped();
        self.base_hyperlane_thickness_percent = settings.base_thickness_percent_of_star;
        self.base_hyperlane_opacity_percent = settings.base_opacity_percent;
        self.hyperlane_falloff_distance_percent = settings.falloff_distance_percent;
        self.hyperlane_falloff_thickness_percent = settings.falloff_thickness_percent;
        self.hyperlane_falloff_opacity_percent = settings.falloff_opacity_percent;
    }

    pub fn antialiasing_mode(&self) -> crate::studio_antialiasing::StudioAntialiasingMode {
        self.antialiasing_mode
    }

    pub fn set_antialiasing_mode(
        &mut self,
        mode: crate::studio_antialiasing::StudioAntialiasingMode,
    ) {
        self.antialiasing_mode = mode.normalize();
    }
}

fn default_base_star_blur_radius() -> f32 {
    StarFalloffSettings::default().base_blur_radius
}

fn default_falloff_distance_percent() -> f32 {
    StarFalloffSettings::default().falloff_distance_percent
}

fn default_falloff_star_blur_radius_percent() -> f32 {
    StarFalloffSettings::default().falloff_blur_radius_percent
}

fn default_falloff_star_opacity_percent() -> f32 {
    StarFalloffSettings::default().falloff_opacity_percent
}

fn default_nameplate_relative_width_percent() -> f32 {
    StarNameplateSettings::default().relative_width_percent
}

fn default_nameplate_base_transparency_percent() -> f32 {
    StarNameplateSettings::default().base_transparency_percent
}

fn default_nameplate_relative_falloff_distance_percent() -> f32 {
    StarNameplateSettings::default().relative_falloff_distance_percent
}

fn default_nameplate_relative_falloff_transparency_percent() -> f32 {
    StarNameplateSettings::default().relative_falloff_transparency_percent
}

fn default_base_hyperlane_thickness_percent() -> f32 {
    HyperlaneRenderSettings::default().base_thickness_percent_of_star
}

fn default_base_hyperlane_opacity_percent() -> f32 {
    HyperlaneRenderSettings::default().base_opacity_percent
}

fn default_hyperlane_falloff_distance_percent() -> f32 {
    HyperlaneRenderSettings::default().falloff_distance_percent
}

fn default_hyperlane_falloff_thickness_percent() -> f32 {
    HyperlaneRenderSettings::default().falloff_thickness_percent
}

fn default_hyperlane_falloff_opacity_percent() -> f32 {
    HyperlaneRenderSettings::default().falloff_opacity_percent
}

fn default_settings_dialog_position() -> [f32; 2] {
    [520.0, 96.0]
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

}
