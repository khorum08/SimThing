//! STUDIO-ANTIALIASING-POST-AA-0 — mutually exclusive Bevy FXAA/SMAA post-process modes.
//!
//! Temporal antialiasing is deferred due to ghosting/blur risk for moving camera and GPU TypeFace
//! labels. Multi-sample antialiasing is deferred to a future track because it has a different
//! geometry-edge cost profile and higher render-target cost.

use bevy::core_pipeline::fxaa::Fxaa;
use bevy::core_pipeline::smaa::{Smaa, SmaaPreset};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

/// Mutually exclusive Studio post-process antialiasing mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum StudioAntialiasingMode {
    #[default]
    Off,
    Fxaa,
    SmaaLow,
    SmaaMedium,
    SmaaHigh,
    SmaaUltra,
}

impl StudioAntialiasingMode {
    pub const ALL: [Self; 6] = [
        Self::Off,
        Self::Fxaa,
        Self::SmaaLow,
        Self::SmaaMedium,
        Self::SmaaHigh,
        Self::SmaaUltra,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Off => "Off",
            Self::Fxaa => "FXAA",
            Self::SmaaLow => "SMAA Low",
            Self::SmaaMedium => "SMAA Medium",
            Self::SmaaHigh => "SMAA High",
            Self::SmaaUltra => "SMAA Ultra",
        }
    }

    pub fn smaa_preset(self) -> Option<SmaaPreset> {
        match self {
            Self::SmaaLow => Some(SmaaPreset::Low),
            Self::SmaaMedium => Some(SmaaPreset::Medium),
            Self::SmaaHigh => Some(SmaaPreset::High),
            Self::SmaaUltra => Some(SmaaPreset::Ultra),
            Self::Off | Self::Fxaa => None,
        }
    }

    /// Normalize unknown serialized values to the conservative default.
    pub fn normalize(self) -> Self {
        if Self::ALL.contains(&self) {
            self
        } else {
            Self::Off
        }
    }
}

/// Where the active Studio antialiasing mode came from at startup or after UI edits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum StudioAntialiasingModeSource {
    #[default]
    DefaultFallback,
    LoadedStudioConfig,
    CurrentUiState,
}

impl StudioAntialiasingModeSource {
    pub fn label(self) -> &'static str {
        match self {
            Self::DefaultFallback => "default fallback",
            Self::LoadedStudioConfig => "loaded studio config",
            Self::CurrentUiState => "current UI state",
        }
    }
}

/// Tracks post-process AA apply events for Video Options Debug telemetry.
#[derive(Resource, Clone, Debug, PartialEq, Eq, Default)]
pub struct StudioAntialiasingApplyState {
    pub apply_generation: u64,
    pub last_applied_mode: Option<StudioAntialiasingMode>,
    pub last_applied_frame: u64,
}

impl StudioAntialiasingApplyState {
    pub fn record_apply(&mut self, mode: StudioAntialiasingMode, frame: u64) {
        self.apply_generation = self.apply_generation.saturating_add(1);
        self.last_applied_mode = Some(mode);
        self.last_applied_frame = frame;
    }
}

/// Snapshot of post-process AA components on the primary Camera3d.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct AntialiasingComponentSnapshot {
    pub primary_camera_found: bool,
    pub camera_entity_index: Option<u32>,
    pub fxaa_present: bool,
    pub smaa_present: bool,
    pub smaa_preset: Option<SmaaPreset>,
}

pub fn smaa_preset_label(preset: SmaaPreset) -> &'static str {
    match preset {
        SmaaPreset::Low => "Low",
        SmaaPreset::Medium => "Medium",
        SmaaPreset::High => "High",
        SmaaPreset::Ultra => "Ultra",
    }
}

pub fn expected_antialiasing_components(
    selected: StudioAntialiasingMode,
) -> (bool, bool, Option<SmaaPreset>) {
    match selected {
        StudioAntialiasingMode::Off => (false, false, None),
        StudioAntialiasingMode::Fxaa => (true, false, None),
        mode => (false, true, mode.smaa_preset()),
    }
}

pub fn dual_post_aa_components_active(snapshot: AntialiasingComponentSnapshot) -> bool {
    snapshot.fxaa_present && snapshot.smaa_present
}

pub fn antialiasing_component_state_mismatch(
    selected: StudioAntialiasingMode,
    snapshot: AntialiasingComponentSnapshot,
) -> bool {
    if !snapshot.primary_camera_found {
        return true;
    }
    let (expect_fxaa, expect_smaa, expect_preset) = expected_antialiasing_components(selected);
    snapshot.fxaa_present != expect_fxaa
        || snapshot.smaa_present != expect_smaa
        || snapshot.smaa_preset != expect_preset
}

/// Apply a mutually exclusive antialiasing mode to the Studio camera entity.
pub fn apply_studio_antialiasing_mode(entity: &mut EntityCommands, mode: StudioAntialiasingMode) {
    match mode {
        StudioAntialiasingMode::Off => {
            entity.remove::<Fxaa>();
            entity.remove::<Smaa>();
        }
        StudioAntialiasingMode::Fxaa => {
            entity.insert(Fxaa::default());
            entity.remove::<Smaa>();
        }
        StudioAntialiasingMode::SmaaLow
        | StudioAntialiasingMode::SmaaMedium
        | StudioAntialiasingMode::SmaaHigh
        | StudioAntialiasingMode::SmaaUltra => {
            entity.remove::<Fxaa>();
            entity.insert(Smaa {
                preset: mode.smaa_preset().expect("SMAA mode has preset"),
            });
        }
    }
}

pub fn apply_and_record_studio_antialiasing_mode(
    entity: &mut EntityCommands,
    mode: StudioAntialiasingMode,
    apply_state: &mut StudioAntialiasingApplyState,
    frame: u64,
) {
    apply_studio_antialiasing_mode(entity, mode);
    apply_state.record_apply(mode, frame);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn antialiasing_default_is_off() {
        assert_eq!(
            StudioAntialiasingMode::default(),
            StudioAntialiasingMode::Off
        );
    }

    #[test]
    fn antialiasing_labels_are_unique() {
        let labels: Vec<_> = StudioAntialiasingMode::ALL
            .iter()
            .map(|mode| mode.label())
            .collect();
        let mut unique = labels.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(labels.len(), unique.len());
    }

    #[test]
    fn antialiasing_smaa_presets_match_modes() {
        assert!(matches!(
            StudioAntialiasingMode::SmaaLow.smaa_preset(),
            Some(SmaaPreset::Low)
        ));
        assert!(matches!(
            StudioAntialiasingMode::SmaaUltra.smaa_preset(),
            Some(SmaaPreset::Ultra)
        ));
        assert!(StudioAntialiasingMode::Fxaa.smaa_preset().is_none());
        assert!(StudioAntialiasingMode::Off.smaa_preset().is_none());
    }

    #[test]
    fn antialiasing_mismatch_when_fxaa_selected_but_smaa_present() {
        let snapshot = AntialiasingComponentSnapshot {
            primary_camera_found: true,
            camera_entity_index: Some(1),
            fxaa_present: false,
            smaa_present: true,
            smaa_preset: Some(SmaaPreset::Low),
        };
        assert!(antialiasing_component_state_mismatch(
            StudioAntialiasingMode::Fxaa,
            snapshot
        ));
    }

    #[test]
    fn antialiasing_no_mismatch_when_smaa_low_matches_components() {
        let snapshot = AntialiasingComponentSnapshot {
            primary_camera_found: true,
            camera_entity_index: Some(1),
            fxaa_present: false,
            smaa_present: true,
            smaa_preset: Some(SmaaPreset::Low),
        };
        assert!(!antialiasing_component_state_mismatch(
            StudioAntialiasingMode::SmaaLow,
            snapshot
        ));
    }

    #[test]
    fn antialiasing_apply_state_records_generation() {
        let mut state = StudioAntialiasingApplyState::default();
        state.record_apply(StudioAntialiasingMode::Fxaa, 42);
        assert_eq!(state.apply_generation, 1);
        assert_eq!(state.last_applied_mode, Some(StudioAntialiasingMode::Fxaa));
        assert_eq!(state.last_applied_frame, 42);
    }

    #[test]
    fn antialiasing_config_roundtrip_json() {
        for mode in StudioAntialiasingMode::ALL {
            let json = serde_json::to_string(&mode).expect("serialize");
            let loaded: StudioAntialiasingMode = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(loaded, mode);
        }
    }
}
