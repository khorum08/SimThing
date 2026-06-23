//! STUDIO-ANTIALIASING-POST-AA-0 — mutually exclusive Bevy FXAA/SMAA post-process modes.
//!
//! Temporal antialiasing is deferred due to ghosting/blur risk for moving camera and GPU TypeFace
//! labels. Multi-sample antialiasing is deferred to a future track because it has a different
//! geometry-edge cost profile and higher render-target cost.

use bevy::core_pipeline::fxaa::Fxaa;
use bevy::core_pipeline::smaa::{Smaa, SmaaPreset};
use bevy::ecs::system::EntityCommands;
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
    fn antialiasing_config_roundtrip_json() {
        for mode in StudioAntialiasingMode::ALL {
            let json = serde_json::to_string(&mode).expect("serialize");
            let loaded: StudioAntialiasingMode = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(loaded, mode);
        }
    }
}
