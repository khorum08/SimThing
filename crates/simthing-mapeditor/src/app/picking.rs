#![cfg(windows)]

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContexts;

use crate::selection::{
    apply_star_click, pick_nearest_star_screen, screen_star_projection_from_anchor,
    DEFAULT_PICK_RADIUS_PX,
};
use crate::star_render::{star_distance_visual, star_emissive_strength};
use crate::view_model::anchor_for_system;

use super::camera::MainCamera;
use super::galaxy_render::{rebuild_highlight_hyperlanes, GalaxyStar, StarVisualLayer};
use super::StudioAppState;

pub fn selection_keyboard_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<StudioAppState>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        state.selection.clear();
    }
}

pub fn star_pick_system(
    mut contexts: EguiContexts,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut state: ResMut<StudioAppState>,
) {
    let Some(session) = state.session.as_ref() else {
        state.selection.set_hover(None);
        return;
    };

    if contexts
        .ctx_mut()
        .map(|ctx| ctx.wants_pointer_input())
        .unwrap_or(false)
    {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor) = window.cursor_position() else {
        state.selection.set_hover(None);
        return;
    };
    let Ok((camera, camera_transform)) = camera.single() else {
        return;
    };

    let mut projections = Vec::with_capacity(session.view_model.render_anchors.len());
    for anchor in &session.view_model.render_anchors {
        let world = Vec3::from_array(anchor.world_position);
        if let Ok(viewport) = camera.world_to_viewport(camera_transform, world) {
            let projection = screen_star_projection_from_anchor(
                anchor,
                viewport.x,
                viewport.y,
                camera_transform.translation().to_array(),
            );
            projections.push(projection);
        }
    }

    let hover = pick_nearest_star_screen(cursor.x, cursor.y, DEFAULT_PICK_RADIUS_PX, &projections);
    state.selection.set_hover(hover);

    if mouse.just_pressed(MouseButton::Left) {
        if let Some(system_id) = hover {
            apply_star_click(&mut state.selection, system_id);
        }
    }
}

pub fn sync_selection_highlight_system(
    state: Res<StudioAppState>,
    mut last_selected: Local<Option<u32>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut scene_root: ResMut<super::GalaxySceneRoot>,
) {
    if *last_selected == state.selection.selected_system_id {
        return;
    }
    *last_selected = state.selection.selected_system_id;
    let Some(session) = state.session.as_ref() else {
        return;
    };
    rebuild_highlight_hyperlanes(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut scene_root,
        session,
        state.selection.selected_system_id,
    );
}

pub fn sync_star_visuals_system(
    state: Res<StudioAppState>,
    camera: Query<&GlobalTransform, With<MainCamera>>,
    mut stars: Query<(
        &GalaxyStar,
        &mut Transform,
        &MeshMaterial3d<StandardMaterial>,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Some(session) = state.session.as_ref() else {
        return;
    };
    let Ok(camera_transform) = camera.single() else {
        return;
    };
    let camera_pos = camera_transform.translation();
    let meta = &session.view_model.render_meta;
    for (star, mut transform, material_handle) in &mut stars {
        let Some(view) = session
            .view_model
            .stars
            .iter()
            .find(|s| s.system_id == star.system_id)
        else {
            continue;
        };
        let selected = state.selection.selected_system_id == Some(star.system_id);
        let hovered = state.selection.hovered_system_id == Some(star.system_id);
        let Some(anchor) = anchor_for_system(&session.view_model.render_anchors, star.system_id)
        else {
            continue;
        };
        let star_pos = Vec3::from_array(anchor.world_position);
        let visual = star_distance_visual(camera_pos.distance(star_pos), selected, hovered, meta);
        let (layer_scale, alpha, emissive_factor, color) = match star.layer {
            StarVisualLayer::Core => (visual.core_scale, visual.core_alpha, 1.0, (0.88, 0.95, 1.0)),
            StarVisualLayer::Aura => (
                visual.aura_scale,
                visual.aura_alpha,
                0.20,
                (0.34, 0.66, 1.0),
            ),
        };
        transform.scale = Vec3::splat(view.sprite_scale * layer_scale);
        if let Some(material) = materials.get_mut(&material_handle.0) {
            let emissive = star_emissive_strength(view.emissive_strength, selected, hovered);
            material.base_color = Color::srgba(color.0, color.1, color.2, alpha);
            material.emissive = LinearRgba::new(
                emissive * 1.25 * alpha * emissive_factor,
                emissive * 1.32 * alpha * emissive_factor,
                emissive * 1.45 * alpha * emissive_factor,
                1.0,
            );
        }
    }
}

pub fn billboard_stars_system(
    camera: Query<&GlobalTransform, With<MainCamera>>,
    mut stars: Query<&mut Transform, With<GalaxyStar>>,
) {
    let Ok(cam) = camera.single() else {
        return;
    };
    let cam_pos = cam.translation();
    for mut transform in &mut stars {
        transform.look_at(cam_pos, Vec3::Y);
    }
}
