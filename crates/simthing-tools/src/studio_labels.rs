//! Studio/game label seam — presentation-only adapter over the typeface runtime (LR8).

use std::path::PathBuf;

use bevy::prelude::*;

use crate::{
    icons::IconSet,
    manifest::{bake_icon_manifest, fixture_manifest_path, IconManifestBake},
    path::TextPathSlot,
    style::TextStyleSlot,
    warp::TextWarpSlot,
    TypefaceAtlas,
};

use super::bevy::TextLabelRenderMode;

/// Baked fixture manifest + icon tiles shared with the typeface atlas (import/staging only).
#[derive(Resource)]
pub struct TypefaceIconSet {
    pub icons: IconSet,
    pub bake: IconManifestBake,
}

/// Studio label category (presentation metadata only — not simulation authority).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StudioLabelKind {
    #[default]
    EntityName,
    RegionName,
    DamageText,
    DebugProbe,
}

/// Presentation label authored by Studio/game consumers; synced to typeface components on change.
#[derive(Component, Debug, Clone, PartialEq)]
pub struct StudioTypefaceLabel {
    pub text: String,
    pub kind: StudioLabelKind,
    pub px: f32,
    pub color: [f32; 4],
    pub style_slot: TextStyleSlot,
    pub render_mode: TextLabelRenderMode,
    pub icon_name: Option<String>,
    pub deform_slot: crate::deform::TextDeformSlot,
    pub path_slot: TextPathSlot,
    pub warp_slot: TextWarpSlot,
}

impl StudioTypefaceLabel {
    pub fn entity_name(text: impl Into<String>, px: f32, color: [f32; 4]) -> Self {
        Self {
            text: text.into(),
            kind: StudioLabelKind::EntityName,
            px,
            color,
            style_slot: 0,
            render_mode: TextLabelRenderMode::Raster,
            icon_name: None,
            deform_slot: 0,
            path_slot: 0,
            warp_slot: 0,
        }
    }

    pub fn damage_value(value: i32, px: f32, color: [f32; 4]) -> Self {
        Self {
            text: format!("-{value}"),
            kind: StudioLabelKind::DamageText,
            px,
            color,
            style_slot: 0,
            render_mode: TextLabelRenderMode::Raster,
            icon_name: None,
            deform_slot: 0,
            path_slot: 0,
            warp_slot: 0,
        }
    }

    pub fn with_style_slot(mut self, slot: TextStyleSlot) -> Self {
        self.style_slot = slot;
        self
    }

    pub fn with_render_mode(mut self, mode: TextLabelRenderMode) -> Self {
        self.render_mode = mode;
        self
    }

    pub fn with_icon_name(mut self, name: impl Into<String>) -> Self {
        self.icon_name = Some(name.into());
        self
    }

    pub fn with_deform_slot(mut self, slot: crate::deform::TextDeformSlot) -> Self {
        self.deform_slot = slot;
        self
    }

    pub fn with_path_slot(mut self, slot: TextPathSlot) -> Self {
        self.path_slot = slot;
        self
    }

    pub fn with_warp_slot(mut self, slot: TextWarpSlot) -> Self {
        self.warp_slot = slot;
        self
    }
}

/// Transient damage-text emitter — queues numeric values for staging into typeface labels.
#[derive(Component, Debug, Default, Clone)]
pub struct StudioDamageTextEmitter {
    pub pending_values: Vec<i32>,
}

impl StudioDamageTextEmitter {
    pub fn emit(&mut self, value: i32) {
        self.pending_values.push(value);
    }
}

/// Diagnostics proving Studio seam uses typeface runtime without fallback churn.
#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct StudioTypefaceLabelDiagnostics {
    pub labels_spawned: u64,
    pub labels_updated: u64,
    pub no_op_reuse_count: u64,
    pub manifest_icon_resolve_count: u64,
    pub manifest_reload_count: u64,
    pub runtime_svg_parse_count: u64,
    pub bespoke_text_fallback_count: u64,
}

/// Plugin configuration for fixture manifest bake at import/staging time.
#[derive(Resource, Clone, Debug)]
pub struct StudioTypefaceLabelConfig {
    pub manifest_path: Option<PathBuf>,
    pub icon_px: f32,
}

impl Default for StudioTypefaceLabelConfig {
    fn default() -> Self {
        Self {
            manifest_path: Some(fixture_manifest_path()),
            icon_px: 32.0,
        }
    }
}

/// Adds Studio label resources and bakes the fixture manifest once after atlas init.
pub struct StudioTypefaceLabelPlugin;

impl Plugin for StudioTypefaceLabelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StudioTypefaceLabelDiagnostics>()
            .init_resource::<StudioTypefaceLabelConfig>()
            .add_systems(PostStartup, bake_studio_fixture_manifest);
    }
}

pub fn studio_typeface_label_diagnostics(app: &App) -> StudioTypefaceLabelDiagnostics {
    app.world()
        .get_resource::<StudioTypefaceLabelDiagnostics>()
        .copied()
        .unwrap_or_default()
}

pub fn icon_name_to_codepoint(bake: &IconManifestBake, name: &str) -> Option<u32> {
    bake.name_to_codepoint.get(name).copied()
}

pub fn resolve_studio_display_text(
    text: &str,
    icon_name: &Option<String>,
    bake: Option<&IconManifestBake>,
    diagnostics: &mut StudioTypefaceLabelDiagnostics,
) -> String {
    let Some(name) = icon_name else {
        return text.to_string();
    };
    let Some(bake) = bake else {
        return text.to_string();
    };
    let Some(codepoint) = icon_name_to_codepoint(bake, name) else {
        return text.to_string();
    };
    diagnostics.manifest_icon_resolve_count += 1;
    let ch = char::from_u32(codepoint).unwrap_or('\u{fffd}');
    format!("{ch}{text}")
}

pub fn try_parse_damage_value(text: &str) -> Option<i32> {
    let rest = text.strip_prefix('-')?;
    if rest.is_empty() || !rest.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    text.parse().ok()
}

pub fn spawn_studio_typeface_label(commands: &mut Commands, label: StudioTypefaceLabel) -> Entity {
    commands.spawn(label).id()
}

fn bake_studio_fixture_manifest(
    config: Res<StudioTypefaceLabelConfig>,
    mut atlas: ResMut<TypefaceAtlas>,
    mut diagnostics: ResMut<StudioTypefaceLabelDiagnostics>,
    mut commands: Commands,
) {
    let Some(path) = config.manifest_path.as_ref() else {
        return;
    };
    let mut icons = IconSet::new();
    match bake_icon_manifest(path, &mut icons, &mut atlas.cpu, config.icon_px) {
        Ok(bake) => {
            diagnostics.manifest_reload_count += 1;
            commands.insert_resource(TypefaceIconSet { icons, bake });
        }
        Err(_) => {}
    }
}
