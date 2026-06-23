//! Presentation-only Studio config persisted as `simthing-studio-config.json`.
//!
//! This is **not** scenario/model authority. It stores editor UI/render preferences only.
//! `SimThingScenarioSpec` and generated galaxy data must never be serialized here.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::hyperlane_buckets::HyperlaneRenderSettings;
use crate::settings::PersistedCameraState;
use crate::star_render::{StarFalloffSettings, StarNameplateSettings, StarRenderMode};

pub const STUDIO_CONFIG_FILE_NAME: &str = "simthing-studio-config.json";
pub const STUDIO_CONFIG_TMP_SUFFIX: &str = "json.tmp";
pub const STUDIO_CONFIG_SCHEMA_VERSION: u32 = 1;

/// Presentation-only Studio configuration. Never includes scenario/model authority.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SimThingStudioConfig {
    pub schema_version: u32,
    pub settings_dialog: SettingsDialogConfig,
    pub star_rendering: StarRenderConfig,
    #[serde(default = "default_nameplate_render_config")]
    pub nameplate_rendering: NameplateRenderConfig,
    pub hyperlane_rendering: HyperlaneRenderConfig,
    pub view: StudioViewConfig,
    pub camera: Option<StudioCameraConfig>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SettingsDialogConfig {
    pub visible: bool,
    pub position: [f32; 2],
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StarRenderConfig {
    pub base_blur_radius: f32,
    pub falloff_distance_percent: f32,
    pub falloff_blur_radius_percent: f32,
    pub falloff_opacity_percent: f32,
    pub render_mode: StarRenderMode,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NameplateRenderConfig {
    /// Historical serialized name; interpreted as uniform relative size (not horizontal-only width).
    pub relative_width_percent: f32,
    pub base_transparency_percent: f32,
    pub relative_falloff_distance_percent: f32,
    pub relative_falloff_transparency_percent: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HyperlaneRenderConfig {
    pub base_thickness_percent_of_star: f32,
    pub base_opacity_percent: f32,
    pub falloff_distance_percent: f32,
    pub falloff_thickness_percent: f32,
    pub falloff_opacity_percent: f32,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StudioViewModeSetting {
    ThreeD,
    OverheadStrategic,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StudioViewConfig {
    pub show_stars: bool,
    pub show_hyperlanes: bool,
    pub view_mode: StudioViewModeSetting,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StudioCameraConfig {
    pub orbit_yaw: f32,
    pub orbit_pitch: f32,
    pub orbit_distance: f32,
    pub orbit_target: [f32; 3],
    pub overhead: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StudioConfigLoadOutcome {
    MissingDefaults,
    Loaded {
        config: SimThingStudioConfig,
        warnings: Vec<String>,
    },
    RejectedDefaults {
        reason: String,
    },
}

#[derive(Debug, Error)]
pub enum StudioConfigError {
    #[error("failed to read studio config: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to serialize studio config: {0}")]
    Serialize(String),
    #[error("failed to deserialize studio config: {0}")]
    Deserialize(String),
}

impl Default for SimThingStudioConfig {
    fn default() -> Self {
        let star = StarFalloffSettings::default();
        let nameplate = StarNameplateSettings::default();
        let hyperlane = HyperlaneRenderSettings::default();
        Self {
            schema_version: STUDIO_CONFIG_SCHEMA_VERSION,
            settings_dialog: SettingsDialogConfig {
                visible: false,
                position: [520.0, 96.0],
            },
            star_rendering: StarRenderConfig::from_star_settings(star, StarRenderMode::default()),
            nameplate_rendering: NameplateRenderConfig::from_nameplate_settings(nameplate),
            hyperlane_rendering: HyperlaneRenderConfig::from_hyperlane_settings(hyperlane),
            view: StudioViewConfig::default(),
            camera: Some(StudioCameraConfig::from_persisted(
                PersistedCameraState::default(),
            )),
        }
    }
}

impl Default for StudioViewConfig {
    fn default() -> Self {
        Self {
            show_stars: true,
            show_hyperlanes: true,
            view_mode: StudioViewModeSetting::ThreeD,
        }
    }
}

impl StarRenderConfig {
    pub fn from_star_settings(settings: StarFalloffSettings, mode: StarRenderMode) -> Self {
        let settings = settings.clamped();
        Self {
            base_blur_radius: settings.base_blur_radius,
            falloff_distance_percent: settings.falloff_distance_percent,
            falloff_blur_radius_percent: settings.falloff_blur_radius_percent,
            falloff_opacity_percent: settings.falloff_opacity_percent,
            render_mode: mode,
        }
    }

    pub fn to_star_settings(self) -> (StarFalloffSettings, StarRenderMode) {
        (
            StarFalloffSettings {
                base_blur_radius: self.base_blur_radius,
                falloff_distance_percent: self.falloff_distance_percent,
                falloff_blur_radius_percent: self.falloff_blur_radius_percent,
                falloff_opacity_percent: self.falloff_opacity_percent,
            }
            .clamped(),
            self.render_mode,
        )
    }
}

impl NameplateRenderConfig {
    pub fn from_nameplate_settings(settings: StarNameplateSettings) -> Self {
        let settings = settings.clamped();
        Self {
            relative_width_percent: settings.relative_width_percent,
            base_transparency_percent: settings.base_transparency_percent,
            relative_falloff_distance_percent: settings.relative_falloff_distance_percent,
            relative_falloff_transparency_percent: settings.relative_falloff_transparency_percent,
        }
    }

    pub fn to_nameplate_settings(self) -> StarNameplateSettings {
        StarNameplateSettings {
            relative_width_percent: self.relative_width_percent,
            base_transparency_percent: self.base_transparency_percent,
            relative_falloff_distance_percent: self.relative_falloff_distance_percent,
            relative_falloff_transparency_percent: self.relative_falloff_transparency_percent,
        }
        .clamped()
    }
}

fn default_nameplate_render_config() -> NameplateRenderConfig {
    NameplateRenderConfig::from_nameplate_settings(StarNameplateSettings::default())
}

impl HyperlaneRenderConfig {
    pub fn from_hyperlane_settings(settings: HyperlaneRenderSettings) -> Self {
        let settings = settings.clamped();
        Self {
            base_thickness_percent_of_star: settings.base_thickness_percent_of_star,
            base_opacity_percent: settings.base_opacity_percent,
            falloff_distance_percent: settings.falloff_distance_percent,
            falloff_thickness_percent: settings.falloff_thickness_percent,
            falloff_opacity_percent: settings.falloff_opacity_percent,
        }
    }

    pub fn to_hyperlane_settings(self) -> HyperlaneRenderSettings {
        HyperlaneRenderSettings {
            base_thickness_percent_of_star: self.base_thickness_percent_of_star,
            base_opacity_percent: self.base_opacity_percent,
            falloff_distance_percent: self.falloff_distance_percent,
            falloff_thickness_percent: self.falloff_thickness_percent,
            falloff_opacity_percent: self.falloff_opacity_percent,
        }
        .clamped()
    }
}

impl StudioCameraConfig {
    pub fn from_persisted(camera: PersistedCameraState) -> Self {
        Self {
            orbit_yaw: camera.orbit_yaw,
            orbit_pitch: camera.orbit_pitch,
            orbit_distance: camera.orbit_distance,
            orbit_target: camera.orbit_target,
            overhead: camera.overhead,
        }
    }

    pub fn to_persisted(self) -> PersistedCameraState {
        PersistedCameraState {
            orbit_yaw: self.orbit_yaw,
            orbit_pitch: self.orbit_pitch,
            orbit_distance: self.orbit_distance,
            orbit_target: self.orbit_target,
            overhead: self.overhead,
        }
    }
}

impl SimThingStudioConfig {
    pub fn config_path() -> PathBuf {
        PathBuf::from(STUDIO_CONFIG_FILE_NAME)
    }

    pub fn settings_dialog_defaults() -> SettingsDialogConfig {
        SimThingStudioConfig::default().settings_dialog
    }

    pub fn star_rendering_defaults() -> StarRenderConfig {
        SimThingStudioConfig::default().star_rendering
    }

    pub fn hyperlane_rendering_defaults() -> HyperlaneRenderConfig {
        SimThingStudioConfig::default().hyperlane_rendering
    }

    pub fn nameplate_rendering_defaults() -> NameplateRenderConfig {
        default_nameplate_render_config()
    }

    pub fn from_presentation_state(
        settings_dialog_visible: bool,
        settings_dialog_position: [f32; 2],
        star_falloff: StarFalloffSettings,
        star_render_mode: StarRenderMode,
        nameplate: StarNameplateSettings,
        hyperlane: HyperlaneRenderSettings,
        show_stars: bool,
        show_hyperlanes: bool,
        view_mode: StudioViewModeSetting,
        camera: Option<PersistedCameraState>,
    ) -> Self {
        Self {
            schema_version: STUDIO_CONFIG_SCHEMA_VERSION,
            settings_dialog: SettingsDialogConfig {
                visible: settings_dialog_visible,
                position: settings_dialog_position,
            },
            star_rendering: StarRenderConfig::from_star_settings(star_falloff, star_render_mode),
            nameplate_rendering: NameplateRenderConfig::from_nameplate_settings(nameplate),
            hyperlane_rendering: HyperlaneRenderConfig::from_hyperlane_settings(hyperlane),
            view: StudioViewConfig {
                show_stars,
                show_hyperlanes,
                view_mode,
            },
            camera: camera.map(StudioCameraConfig::from_persisted),
        }
    }

    pub fn load_at_startup() -> StudioConfigLoadOutcome {
        let path = Self::config_path();
        if !path.exists() {
            return StudioConfigLoadOutcome::MissingDefaults;
        }
        match load_studio_config_from_path(&path) {
            Ok(outcome) => outcome,
            Err(err) => StudioConfigLoadOutcome::RejectedDefaults {
                reason: err.to_string(),
            },
        }
    }

    pub fn save_to_default_path(&self) -> Result<(), StudioConfigError> {
        save_studio_config_to_path(&Self::config_path(), self)
    }
}

pub fn load_studio_config_from_str(
    src: &str,
) -> Result<StudioConfigLoadOutcome, StudioConfigError> {
    let raw: SimThingStudioConfig =
        serde_json::from_str(src).map_err(|err| StudioConfigError::Deserialize(err.to_string()))?;
    Ok(validate_and_normalize_studio_config(raw))
}

pub fn save_studio_config_to_string(
    config: &SimThingStudioConfig,
) -> Result<String, StudioConfigError> {
    serde_json::to_string_pretty(config)
        .map_err(|err| StudioConfigError::Serialize(err.to_string()))
}

pub fn load_studio_config_from_path(
    path: &Path,
) -> Result<StudioConfigLoadOutcome, StudioConfigError> {
    let text = std::fs::read_to_string(path)?;
    load_studio_config_from_str(&text)
}

pub fn save_studio_config_to_path(
    path: &Path,
    config: &SimThingStudioConfig,
) -> Result<(), StudioConfigError> {
    let text = save_studio_config_to_string(config)?;
    atomic_write(path, &text)
}

/// Validation policy:
/// - Malformed JSON or unsupported `schema_version`: reject entire file.
/// - NaN/Inf anywhere in numeric fields: reject entire file.
/// - Ordinary out-of-range values: clamp to accepted bounds and record warnings.
pub fn validate_and_normalize_studio_config(
    mut config: SimThingStudioConfig,
) -> StudioConfigLoadOutcome {
    if config.schema_version != STUDIO_CONFIG_SCHEMA_VERSION {
        return StudioConfigLoadOutcome::RejectedDefaults {
            reason: format!(
                "unsupported schema_version {} (expected {})",
                config.schema_version, STUDIO_CONFIG_SCHEMA_VERSION
            ),
        };
    }

    let mut warnings = Vec::new();

    if reject_non_finite_config(&config) {
        return StudioConfigLoadOutcome::RejectedDefaults {
            reason: "studio config contains NaN or infinite values".to_string(),
        };
    }

    if !StarRenderMode::ALL.contains(&config.star_rendering.render_mode) {
        return StudioConfigLoadOutcome::RejectedDefaults {
            reason: "studio config star render_mode is not recognized".to_string(),
        };
    }

    let (star, mode) = config.star_rendering.clone().to_star_settings();
    let clamped_star = StarRenderConfig::from_star_settings(star, mode);
    if clamped_star != config.star_rendering {
        warnings.push("clamped star rendering values to accepted bounds".to_string());
        config.star_rendering = clamped_star;
    }

    let clamped_hyperlane = HyperlaneRenderConfig::from_hyperlane_settings(
        config.hyperlane_rendering.clone().to_hyperlane_settings(),
    );
    if clamped_hyperlane != config.hyperlane_rendering {
        warnings.push("clamped hyperlane rendering values to accepted bounds".to_string());
        config.hyperlane_rendering = clamped_hyperlane;
    }

    let clamped_nameplate = NameplateRenderConfig::from_nameplate_settings(
        config.nameplate_rendering.clone().to_nameplate_settings(),
    );
    if clamped_nameplate != config.nameplate_rendering {
        warnings.push("clamped nameplate rendering values to accepted bounds".to_string());
        config.nameplate_rendering = clamped_nameplate;
    }

    if !config.settings_dialog.position[0].is_finite()
        || !config.settings_dialog.position[1].is_finite()
    {
        return StudioConfigLoadOutcome::RejectedDefaults {
            reason: "settings dialog position is not finite".to_string(),
        };
    }

    if let Some(camera) = &config.camera {
        if !camera.orbit_yaw.is_finite()
            || !camera.orbit_pitch.is_finite()
            || !camera.orbit_distance.is_finite()
            || !camera.orbit_target[0].is_finite()
            || !camera.orbit_target[1].is_finite()
            || !camera.orbit_target[2].is_finite()
        {
            return StudioConfigLoadOutcome::RejectedDefaults {
                reason: "camera values are not finite".to_string(),
            };
        }
        if camera.orbit_distance < 0.0 {
            warnings.push("clamped camera orbit_distance to non-negative".to_string());
            config.camera = Some(StudioCameraConfig {
                orbit_distance: camera.orbit_distance.max(0.0),
                ..camera.clone()
            });
        }
    }

    StudioConfigLoadOutcome::Loaded { config, warnings }
}

fn reject_non_finite_config(config: &SimThingStudioConfig) -> bool {
    let star = &config.star_rendering;
    if !is_finite_f32(star.base_blur_radius)
        || !is_finite_f32(star.falloff_distance_percent)
        || !is_finite_f32(star.falloff_blur_radius_percent)
        || !is_finite_f32(star.falloff_opacity_percent)
    {
        return true;
    }
    let hyperlane = &config.hyperlane_rendering;
    if !is_finite_f32(hyperlane.base_thickness_percent_of_star)
        || !is_finite_f32(hyperlane.base_opacity_percent)
        || !is_finite_f32(hyperlane.falloff_distance_percent)
        || !is_finite_f32(hyperlane.falloff_thickness_percent)
        || !is_finite_f32(hyperlane.falloff_opacity_percent)
    {
        return true;
    }
    let nameplate = &config.nameplate_rendering;
    if !is_finite_f32(nameplate.relative_width_percent)
        || !is_finite_f32(nameplate.base_transparency_percent)
        || !is_finite_f32(nameplate.relative_falloff_distance_percent)
        || !is_finite_f32(nameplate.relative_falloff_transparency_percent)
    {
        return true;
    }
    if !is_finite_f32(config.settings_dialog.position[0])
        || !is_finite_f32(config.settings_dialog.position[1])
    {
        return true;
    }
    if let Some(camera) = &config.camera {
        if !is_finite_f32(camera.orbit_yaw)
            || !is_finite_f32(camera.orbit_pitch)
            || !is_finite_f32(camera.orbit_distance)
            || !is_finite_f32(camera.orbit_target[0])
            || !is_finite_f32(camera.orbit_target[1])
            || !is_finite_f32(camera.orbit_target[2])
        {
            return true;
        }
    }
    false
}

fn is_finite_f32(value: f32) -> bool {
    value.is_finite()
}

fn atomic_write(path: &Path, contents: &str) -> Result<(), StudioConfigError> {
    let tmp = path.with_extension(STUDIO_CONFIG_TMP_SUFFIX);
    std::fs::write(&tmp, contents)?;
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    std::fs::rename(&tmp, path)?;
    Ok(())
}

pub fn apply_studio_config_to_editor_settings(
    config: &SimThingStudioConfig,
    settings: &mut crate::settings::EditorSettings,
) {
    settings.settings_dialog_visible = config.settings_dialog.visible;
    settings.settings_dialog_position = config.settings_dialog.position;
    let (star, mode) = config.star_rendering.clone().to_star_settings();
    settings.set_star_falloff_settings(star);
    settings.set_star_render_mode(mode);
    settings
        .set_star_nameplate_settings(config.nameplate_rendering.clone().to_nameplate_settings());
    settings
        .set_hyperlane_render_settings(config.hyperlane_rendering.clone().to_hyperlane_settings());
    if let Some(camera) = &config.camera {
        settings.last_camera = camera.clone().to_persisted();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::StudioSession;
    use crate::settings::EditorSettings;
    use tempfile::TempDir;

    #[test]
    fn studio_config_defaults_are_valid() {
        let config = SimThingStudioConfig::default();
        match validate_and_normalize_studio_config(config) {
            StudioConfigLoadOutcome::Loaded { warnings, .. } => assert!(warnings.is_empty()),
            other => panic!("expected valid defaults, got {other:?}"),
        }
    }

    #[test]
    fn studio_config_serializes_to_json() {
        let json =
            save_studio_config_to_string(&SimThingStudioConfig::default()).expect("serialize");
        assert!(json.contains("\"schema_version\""));
        assert!(json.contains("\"star_rendering\""));
    }

    #[test]
    fn studio_config_deserializes_from_json() {
        let json =
            save_studio_config_to_string(&SimThingStudioConfig::default()).expect("serialize");
        match load_studio_config_from_str(&json).expect("load") {
            StudioConfigLoadOutcome::Loaded { config, .. } => {
                assert_eq!(config.schema_version, STUDIO_CONFIG_SCHEMA_VERSION);
            }
            other => panic!("expected loaded config, got {other:?}"),
        }
    }

    #[test]
    fn studio_config_roundtrip_preserves_star_settings() {
        let mut config = SimThingStudioConfig::default();
        config.star_rendering = StarRenderConfig::from_star_settings(
            StarFalloffSettings {
                base_blur_radius: 0.42,
                falloff_distance_percent: 55.0,
                falloff_blur_radius_percent: 33.0,
                falloff_opacity_percent: 12.0,
            },
            StarRenderMode::CrispCircle,
        );
        let json = save_studio_config_to_string(&config).expect("serialize");
        let loaded = match load_studio_config_from_str(&json).expect("load") {
            StudioConfigLoadOutcome::Loaded { config, .. } => config,
            other => panic!("expected loaded config, got {other:?}"),
        };
        assert_eq!(loaded.star_rendering, config.star_rendering);
    }

    #[test]
    fn studio_config_roundtrip_preserves_nameplate_settings() {
        let mut config = SimThingStudioConfig::default();
        config.nameplate_rendering =
            NameplateRenderConfig::from_nameplate_settings(StarNameplateSettings {
                relative_width_percent: 135.0,
                base_transparency_percent: 72.0,
                relative_falloff_distance_percent: 43.0,
                relative_falloff_transparency_percent: 18.0,
            });
        let json = save_studio_config_to_string(&config).expect("serialize");
        let loaded = match load_studio_config_from_str(&json).expect("load") {
            StudioConfigLoadOutcome::Loaded { config, .. } => config,
            other => panic!("expected loaded config, got {other:?}"),
        };
        assert_eq!(loaded.nameplate_rendering, config.nameplate_rendering);
    }

    #[test]
    fn legacy_config_without_nameplates_uses_nameplate_defaults() {
        let mut value = serde_json::to_value(SimThingStudioConfig::default()).expect("serialize");
        value
            .as_object_mut()
            .expect("config object")
            .remove("nameplate_rendering");
        let json = serde_json::to_string(&value).expect("json");
        let loaded = match load_studio_config_from_str(&json).expect("load") {
            StudioConfigLoadOutcome::Loaded { config, .. } => config,
            other => panic!("expected loaded config, got {other:?}"),
        };
        assert_eq!(
            loaded.nameplate_rendering,
            NameplateRenderConfig::from_nameplate_settings(StarNameplateSettings::default())
        );
    }

    #[test]
    fn studio_config_roundtrip_preserves_hyperlane_settings() {
        let mut config = SimThingStudioConfig::default();
        config.hyperlane_rendering =
            HyperlaneRenderConfig::from_hyperlane_settings(HyperlaneRenderSettings {
                base_thickness_percent_of_star: 14.0,
                base_opacity_percent: 41.0,
                falloff_distance_percent: 62.0,
                falloff_thickness_percent: 18.0,
                falloff_opacity_percent: 9.0,
            });
        let json = save_studio_config_to_string(&config).expect("serialize");
        let loaded = match load_studio_config_from_str(&json).expect("load") {
            StudioConfigLoadOutcome::Loaded { config, .. } => config,
            other => panic!("expected loaded config, got {other:?}"),
        };
        assert_eq!(loaded.hyperlane_rendering, config.hyperlane_rendering);
    }

    #[test]
    fn studio_config_roundtrip_preserves_settings_dialog_state() {
        let mut config = SimThingStudioConfig::default();
        config.settings_dialog.visible = true;
        config.settings_dialog.position = [812.0, 144.0];
        let json = save_studio_config_to_string(&config).expect("serialize");
        let loaded = match load_studio_config_from_str(&json).expect("load") {
            StudioConfigLoadOutcome::Loaded { config, .. } => config,
            other => panic!("expected loaded config, got {other:?}"),
        };
        assert_eq!(loaded.settings_dialog, config.settings_dialog);
    }

    #[test]
    fn studio_config_rejects_malformed_json() {
        let err = load_studio_config_from_str("{not json").expect_err("malformed");
        assert!(matches!(err, StudioConfigError::Deserialize(_)));
    }

    #[test]
    fn studio_config_rejects_unsupported_schema_version() {
        let mut config = SimThingStudioConfig::default();
        config.schema_version = 99;
        let json = save_studio_config_to_string(&config).expect("serialize");
        match load_studio_config_from_str(&json).expect("load") {
            StudioConfigLoadOutcome::RejectedDefaults { reason } => {
                assert!(reason.contains("unsupported schema_version"));
            }
            other => panic!("expected rejection, got {other:?}"),
        }
    }

    #[test]
    fn studio_config_rejects_nan_or_infinite_values() {
        let mut config = SimThingStudioConfig::default();
        config.star_rendering.base_blur_radius = f32::NAN;
        match validate_and_normalize_studio_config(config) {
            StudioConfigLoadOutcome::RejectedDefaults { reason } => {
                assert!(reason.contains("NaN"));
            }
            other => panic!("expected rejection, got {other:?}"),
        }
    }

    #[test]
    fn studio_config_clamps_or_rejects_out_of_range_values_according_to_policy() {
        let mut config = SimThingStudioConfig::default();
        config.star_rendering.base_blur_radius = 9.0;
        config.hyperlane_rendering.base_thickness_percent_of_star = 99.0;
        match validate_and_normalize_studio_config(config) {
            StudioConfigLoadOutcome::Loaded { config, warnings } => {
                assert_eq!(config.star_rendering.base_blur_radius, 1.0);
                assert_eq!(
                    config.hyperlane_rendering.base_thickness_percent_of_star,
                    25.0
                );
                assert_eq!(warnings.len(), 2);
            }
            other => panic!("expected clamped load, got {other:?}"),
        }
    }

    #[test]
    fn startup_missing_config_uses_defaults() {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join(STUDIO_CONFIG_FILE_NAME);
        assert!(!path.exists());
        let outcome = load_studio_config_from_path(&path);
        assert!(outcome.is_err());
    }

    #[test]
    fn startup_valid_config_applies_settings() {
        let mut settings = EditorSettings::default();
        let config = SimThingStudioConfig::from_presentation_state(
            true,
            [640.0, 120.0],
            StarFalloffSettings {
                base_blur_radius: 0.5,
                falloff_distance_percent: 40.0,
                falloff_blur_radius_percent: 20.0,
                falloff_opacity_percent: 30.0,
            },
            StarRenderMode::CrispCircle,
            StarNameplateSettings::default(),
            HyperlaneRenderSettings::default(),
            true,
            false,
            StudioViewModeSetting::OverheadStrategic,
            Some(PersistedCameraState::default()),
        );
        apply_studio_config_to_editor_settings(&config, &mut settings);
        assert!(settings.settings_dialog_visible);
        assert_eq!(settings.star_render_mode(), StarRenderMode::CrispCircle);
    }

    #[test]
    fn startup_invalid_config_uses_defaults_and_records_warning() {
        let json = r#"{"schema_version":2,"settings_dialog":{"visible":false,"position":[0,0]},"star_rendering":{"base_blur_radius":0.1,"falloff_distance_percent":50.0,"falloff_blur_radius_percent":10.0,"falloff_opacity_percent":10.0,"render_mode":"BloomStarburst"},"hyperlane_rendering":{"base_thickness_percent_of_star":8.0,"base_opacity_percent":75.0,"falloff_distance_percent":100.0,"falloff_thickness_percent":24.0,"falloff_opacity_percent":16.0},"view":{"show_stars":true,"show_hyperlanes":true,"view_mode":"three_d"},"camera":null}"#;
        match load_studio_config_from_str(json).expect("parse") {
            StudioConfigLoadOutcome::RejectedDefaults { reason } => {
                assert!(reason.contains("unsupported schema_version"));
            }
            other => panic!("expected rejection, got {other:?}"),
        }
    }

    #[test]
    fn settings_x_close_saves_config() {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join(STUDIO_CONFIG_FILE_NAME);
        let config = SimThingStudioConfig::from_presentation_state(
            false,
            [500.0, 80.0],
            StarFalloffSettings::default(),
            StarRenderMode::default(),
            StarNameplateSettings::default(),
            HyperlaneRenderSettings::default(),
            true,
            true,
            StudioViewModeSetting::ThreeD,
            None,
        );
        save_studio_config_to_path(&path, &config).expect("save");
        let loaded = load_studio_config_from_path(&path).expect("load");
        match loaded {
            StudioConfigLoadOutcome::Loaded { config: round, .. } => {
                assert_eq!(round.settings_dialog.position, [500.0, 80.0])
            }
            other => panic!("expected loaded config, got {other:?}"),
        }
    }

    #[test]
    fn settings_bottom_close_saves_config() {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join(STUDIO_CONFIG_FILE_NAME);
        let config = SimThingStudioConfig::default();
        save_studio_config_to_path(&path, &config).expect("save");
        assert!(path.exists());
        let text = std::fs::read_to_string(&path).expect("read");
        assert!(!text.is_empty());
    }

    #[test]
    fn app_exit_saves_config() {
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join(STUDIO_CONFIG_FILE_NAME);
        let config = SimThingStudioConfig::from_presentation_state(
            true,
            [300.0, 200.0],
            StarFalloffSettings::default(),
            StarRenderMode::CrispCircle,
            StarNameplateSettings::default(),
            HyperlaneRenderSettings::default(),
            false,
            true,
            StudioViewModeSetting::ThreeD,
            Some(PersistedCameraState::default()),
        );
        save_studio_config_to_path(&path, &config).expect("save");
        let tmp = path.with_extension(STUDIO_CONFIG_TMP_SUFFIX);
        assert!(!tmp.exists());
    }

    #[test]
    fn settings_reset_restores_defaults() {
        let defaults = SimThingStudioConfig::default();
        let mut mutated = defaults.clone();
        mutated.star_rendering.base_blur_radius = 0.99;
        mutated.hyperlane_rendering.base_opacity_percent = 12.0;
        mutated.settings_dialog.position = [1.0, 2.0];
        mutated.settings_dialog.visible = true;

        let reset_star = SimThingStudioConfig::star_rendering_defaults();
        let reset_hyperlane = SimThingStudioConfig::hyperlane_rendering_defaults();
        let reset_dialog = SimThingStudioConfig::settings_dialog_defaults();

        assert_ne!(mutated.star_rendering, reset_star);
        mutated.star_rendering = reset_star;
        mutated.hyperlane_rendering = reset_hyperlane;
        mutated.settings_dialog = reset_dialog;

        assert_eq!(mutated.star_rendering, defaults.star_rendering);
        assert_eq!(mutated.hyperlane_rendering, defaults.hyperlane_rendering);
        assert_eq!(mutated.settings_dialog, defaults.settings_dialog);
    }

    #[test]
    fn settings_reset_does_not_modify_scenario_authority() {
        let profile = crate::generation::GenerationProfile::default_spiral_2_dense_3000();
        let output = crate::generation::run_generation(&profile).expect("generate");
        let session = StudioSession::from_generation(profile, output).expect("session");
        let scenario_id = session.scenario_authority.scenario_id.clone();
        let placement_count = session.scenario_authority.structural_grid.placements.len();

        let _reset_defaults = SimThingStudioConfig::default();

        assert_eq!(session.scenario_authority.scenario_id, scenario_id);
        assert_eq!(
            session.scenario_authority.structural_grid.placements.len(),
            placement_count
        );
    }

    #[test]
    fn studio_config_does_not_serialize_simthing_scenario_authority() {
        let json =
            save_studio_config_to_string(&SimThingStudioConfig::default()).expect("serialize");
        for forbidden in [
            "SimThingScenarioSpec",
            "\"root\"",
            "structural_grid",
            "scenario_id",
            "placements",
            "simthing_id_raw",
        ] {
            assert!(
                !json.contains(forbidden),
                "config leaked forbidden key {forbidden}"
            );
        }
    }

    #[test]
    fn studio_config_does_not_serialize_structural_grid() {
        let json =
            save_studio_config_to_string(&SimThingStudioConfig::default()).expect("serialize");
        assert!(!json.contains("structural_grid"));
        assert!(!json.contains("map_container_id"));
    }
}
