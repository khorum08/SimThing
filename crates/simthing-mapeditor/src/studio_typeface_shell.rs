//! Studio app-shell mount for the LR8 typeface label seam (shared by `run_studio` and tests).

use bevy::prelude::*;
use simthing_tools::{
    SimthingToolsTextPlugin, StudioDamageTextEmitter, StudioTypefaceLabel,
    StudioTypefaceLabelPlugin, TypefaceIconSet,
};

const TYPEFACE_FIXTURE_FONT: &[u8] =
    include_bytes!("../../simthing-workshop/assets/typeface/test_font.ttf");

/// True after the Studio shell has mounted typeface plugins and the fixture manifest bake is visible.
#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct StudioTypefaceShellMounted(pub bool);

/// Bundles `SimthingToolsTextPlugin`, `StudioTypefaceLabelPlugin`, and presentation probe staging.
pub struct StudioTypefaceShellPlugin;

impl Plugin for StudioTypefaceShellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StudioTypefaceShellMounted>()
            // Live egui window: disable the offscreen-only LUT D3 view fix, which otherwise mutates
            // the tonemapping LUT image at app scope and black-screens the Studio (egui suppressed,
            // STUDIO-TYPEFACE-STARTUP-FIX-0R). Galaxy-map text renders via the main/overlay camera,
            // not the offscreen tonemapping path that needs that fix.
            .add_plugins(
                SimthingToolsTextPlugin::new(typeface_fixture_font_bytes())
                    .without_lut_d3_view_fix()
                    .world_text_only(),
            )
            .add_plugins(StudioTypefaceLabelPlugin)
            .add_systems(Startup, stage_studio_typeface_presentation_probe)
            .add_systems(Update, mark_studio_typeface_shell_mounted);
    }
}

pub fn typeface_fixture_font_bytes() -> Vec<u8> {
    TYPEFACE_FIXTURE_FONT.to_vec()
}

pub fn mount_studio_typeface_plugins(app: &mut App) {
    app.add_plugins(StudioTypefaceShellPlugin);
}

fn stage_studio_typeface_presentation_probe(mut commands: Commands) {
    commands.spawn(StudioTypefaceLabel::entity_name(
        "Studio",
        24.0,
        [0.92, 0.94, 1.0, 0.85],
    ));
    commands.spawn(StudioDamageTextEmitter::default());
}

fn mark_studio_typeface_shell_mounted(
    icons: Option<Res<TypefaceIconSet>>,
    mut mounted: ResMut<StudioTypefaceShellMounted>,
) {
    if icons.is_some() {
        mounted.0 = true;
    }
}
