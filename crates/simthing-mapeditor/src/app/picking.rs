#![cfg(windows)]

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContexts;

use crate::selection::{
    apply_star_click, pick_nearest_star_screen, ScreenStarProjection, DEFAULT_PICK_RADIUS_PX,
};
use crate::star_render::star_emissive_strength;

use super::camera::MainCamera;
use super::galaxy_render::{rebuild_highlight_hyperlanes, GalaxyStar};
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

    let mut projections = Vec::with_capacity(session.view_model.stars.len());
    for star in &session.view_model.stars {
        let world = Vec3::new(star.world_x, star.world_y, star.world_z);
        if let Ok(viewport) = camera.world_to_viewport(camera_transform, world) {
            let depth = camera_transform.translation().distance(world);
            projections.push(ScreenStarProjection {
                system_id: star.system_id,
                screen_x: viewport.x,
                screen_y: viewport.y,
                depth,
            });
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
        let scale_mul = if selected {
            1.35
        } else if hovered {
            1.15
        } else {
            1.0
        };
        transform.scale = Vec3::splat(view.sprite_scale * scale_mul);
        if let Some(material) = materials.get_mut(&material_handle.0) {
            let emissive = star_emissive_strength(view.emissive_strength, selected, hovered);
            material.emissive = LinearRgba::new(emissive * 0.9, emissive * 0.95, emissive, 1.0);
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
