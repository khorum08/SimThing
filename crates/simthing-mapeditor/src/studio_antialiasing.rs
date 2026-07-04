//! STUDIO-ANTIALIASING-POST-AA-0 / STUDIO-ANTIALIASING-MSAA-0 — mutually exclusive Bevy
//! FXAA/SMAA post-process and MSAA geometry-edge modes.
//!
//! Temporal antialiasing is deferred due to ghosting/blur risk for moving camera and GPU TypeFace
//! labels.

use bevy::core_pipeline::fxaa::Fxaa;
use bevy::core_pipeline::smaa::{Smaa, SmaaPreset};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::Resource;
use bevy::render::view::Msaa;
use serde::{Deserialize, Serialize};

/// Mutually exclusive Studio antialiasing mode.
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
    Msaa2x,
    Msaa4x,
    Msaa8x,
}

impl StudioAntialiasingMode {
    pub const ALL: [Self; 9] = [
        Self::Off,
        Self::Fxaa,
        Self::SmaaLow,
        Self::SmaaMedium,
        Self::SmaaHigh,
        Self::SmaaUltra,
        Self::Msaa2x,
        Self::Msaa4x,
        Self::Msaa8x,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Off => "Off",
            Self::Fxaa => "FXAA",
            Self::SmaaLow => "SMAA Low",
            Self::SmaaMedium => "SMAA Medium",
            Self::SmaaHigh => "SMAA High",
            Self::SmaaUltra => "SMAA Ultra",
            Self::Msaa2x => "MSAA 2x",
            Self::Msaa4x => "MSAA 4x",
            Self::Msaa8x => "MSAA 8x",
        }
    }

    pub fn smaa_preset(self) -> Option<SmaaPreset> {
        match self {
            Self::SmaaLow => Some(SmaaPreset::Low),
            Self::SmaaMedium => Some(SmaaPreset::Medium),
            Self::SmaaHigh => Some(SmaaPreset::High),
            Self::SmaaUltra => Some(SmaaPreset::Ultra),
            _ => None,
        }
    }

    pub fn is_smaa(self) -> bool {
        self.smaa_preset().is_some()
    }

    pub fn is_msaa(self) -> bool {
        matches!(self, Self::Msaa2x | Self::Msaa4x | Self::Msaa8x)
    }

    pub fn expected_msaa_samples(self) -> u32 {
        match self {
            Self::Msaa2x => 2,
            Self::Msaa4x => 4,
            Self::Msaa8x => 8,
            _ => 1,
        }
    }

    pub fn msaa_component(self) -> Msaa {
        match self {
            Self::Msaa2x => Msaa::Sample2,
            Self::Msaa4x => Msaa::Sample4,
            Self::Msaa8x => Msaa::Sample8,
            _ => Msaa::Off,
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

/// Tracks AA apply events for Video Options Debug telemetry.
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

/// Snapshot of AA components on the primary Camera3d.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct AntialiasingComponentSnapshot {
    pub primary_camera_found: bool,
    pub camera_entity_index: Option<u32>,
    pub fxaa_present: bool,
    pub smaa_present: bool,
    pub smaa_preset: Option<SmaaPreset>,
    pub msaa_samples: u32,
    pub msaa_component_present: bool,
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
        StudioAntialiasingMode::SmaaLow
        | StudioAntialiasingMode::SmaaMedium
        | StudioAntialiasingMode::SmaaHigh
        | StudioAntialiasingMode::SmaaUltra => (false, true, selected.smaa_preset()),
        StudioAntialiasingMode::Msaa2x
        | StudioAntialiasingMode::Msaa4x
        | StudioAntialiasingMode::Msaa8x => (false, false, None),
    }
}

pub fn msaa_active(samples: u32) -> bool {
    samples > 1
}

pub fn msaa_sample_count_label(samples: u32) -> &'static str {
    match samples {
        1 => "1/off",
        2 => "2",
        4 => "4",
        8 => "8",
        _ => "unknown",
    }
}

pub fn post_aa_component_active(snapshot: AntialiasingComponentSnapshot) -> bool {
    snapshot.fxaa_present || snapshot.smaa_present
}

pub fn dual_post_aa_components_active(snapshot: AntialiasingComponentSnapshot) -> bool {
    snapshot.fxaa_present && snapshot.smaa_present
}

pub fn smaa_expected_active(selected: StudioAntialiasingMode) -> bool {
    selected.is_smaa()
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
        || snapshot.msaa_samples != selected.expected_msaa_samples()
}

pub fn snapshot_from_camera_components(
    entity: bevy::prelude::Entity,
    fxaa: Option<&Fxaa>,
    smaa: Option<&Smaa>,
    msaa: Option<&Msaa>,
) -> AntialiasingComponentSnapshot {
    AntialiasingComponentSnapshot {
        primary_camera_found: true,
        camera_entity_index: Some(entity.index()),
        fxaa_present: fxaa.is_some(),
        smaa_present: smaa.is_some(),
        smaa_preset: smaa.map(|component| component.preset),
        msaa_samples: msaa.map(|component| component.samples()).unwrap_or(1),
        msaa_component_present: msaa.is_some(),
    }
}

/// Apply a mutually exclusive antialiasing mode to the Studio camera entity.
pub fn apply_studio_antialiasing_mode(entity: &mut EntityCommands, mode: StudioAntialiasingMode) {
    let msaa = mode.msaa_component();
    match mode {
        StudioAntialiasingMode::Off => {
            entity.remove::<Fxaa>();
            entity.remove::<Smaa>();
            entity.insert(msaa);
        }
        StudioAntialiasingMode::Fxaa => {
            entity.insert(Fxaa::default());
            entity.remove::<Smaa>();
            entity.insert(msaa);
        }
        StudioAntialiasingMode::SmaaLow
        | StudioAntialiasingMode::SmaaMedium
        | StudioAntialiasingMode::SmaaHigh
        | StudioAntialiasingMode::SmaaUltra => {
            entity.remove::<Fxaa>();
            entity.insert(Smaa {
                preset: mode.smaa_preset().expect("SMAA mode has preset"),
            });
            entity.insert(msaa);
        }
        StudioAntialiasingMode::Msaa2x
        | StudioAntialiasingMode::Msaa4x
        | StudioAntialiasingMode::Msaa8x => {
            entity.remove::<Fxaa>();
            entity.remove::<Smaa>();
            entity.insert(msaa);
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

}
