#![cfg(windows)]

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContexts;

use crate::selection::{
    apply_star_click, pick_nearest_star_screen, screen_star_projection_from_anchor,
    DEFAULT_PICK_RADIUS_PX,
};
use crate::star_render::{
    compute_star_distance_visual, star_emissive_strength, star_falloff_progress_percent,
    StarBillboardRenderSettings, StarRenderMode,
};
use crate::studio_render_loop_dirty_gate::{
    billboard_should_sync, picking_projection_should_rebuild, quantize_billboard_camera_key,
    quantize_picking_projection_key, quantize_star_depth_percent, star_falloff_settings_key,
    star_visual_per_star_should_write, star_visuals_should_sync, StarVisualAppliedKey,
    StarVisualSyncKey, StudioRenderLoopCaches,
};

use super::camera::MainCamera;
use super::galaxy_render::{
    rebuild_highlight_hyperlanes, GalaxyStar, StarVisualAssets, StarVisualLayer,
};
use super::performance_telemetry::StudioPerformanceTelemetryState;
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
    mut caches: ResMut<StudioRenderLoopCaches>,
    mut perf: ResMut<StudioPerformanceTelemetryState>,
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

    perf.telemetry.picking_projection_calls =
        perf.telemetry.picking_projection_calls.saturating_add(1);
    let started = std::time::Instant::now();
    let cam_tf = camera_transform.compute_transform();
    let projection_key = quantize_picking_projection_key(
        camera_transform.translation().to_array(),
        cam_tf.rotation.to_array(),
        window.resolution.width() as u32,
        window.resolution.height() as u32,
        session.view_model.render_anchors.len(),
        state.scene_render_revision,
    );
    let picking_cache = &mut caches.picking;
    let rebuild = picking_projection_should_rebuild(picking_cache.last_key, projection_key);
    if rebuild {
        picking_cache.cached_projections.clear();
        picking_cache
            .cached_projections
            .reserve(session.view_model.render_anchors.len());
        for anchor in &session.view_model.render_anchors {
            let world = Vec3::from_array(anchor.world_position);
            if let Ok(viewport) = camera.world_to_viewport(camera_transform, world) {
                let projection = screen_star_projection_from_anchor(
                    anchor,
                    viewport.x,
                    viewport.y,
                    camera_transform.translation().to_array(),
                );
                picking_cache.cached_projections.push(projection);
            }
        }
        picking_cache.last_key = Some(projection_key);
    }

    let elapsed_ms = started.elapsed().as_secs_f64() * 1000.0;
    let call_count = perf.telemetry.picking_projection_calls;
    let projected_count = caches.picking.cached_projections.len();
    {
        let telemetry = &mut perf.telemetry;
        crate::studio_render_loop_dirty_gate::render_loop_telemetry_record_timing(
            &mut telemetry.picking_projection_last_ms,
            &mut telemetry.picking_projection_avg_ms,
            elapsed_ms,
            call_count,
        );
        telemetry.picking_projected_anchor_count = projected_count;
    }

    let hover = pick_nearest_star_screen(
        cursor.x,
        cursor.y,
        DEFAULT_PICK_RADIUS_PX,
        &caches.picking.cached_projections,
    );
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
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut stars: Query<(
        &GalaxyStar,
        &mut Transform,
        &MeshMaterial3d<StandardMaterial>,
        &mut StarVisualAppliedKey,
    )>,
    assets: Res<StarVisualAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut caches: ResMut<StudioRenderLoopCaches>,
    mut perf: ResMut<StudioPerformanceTelemetryState>,
    falloff_state: Res<super::StudioMapRadiusFalloffState>,
) {
    perf.telemetry.star_visual_sync_calls = perf.telemetry.star_visual_sync_calls.saturating_add(1);

    let Some(session) = state.session.as_ref() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera.single() else {
        return;
    };
    let Ok(window) = windows.single() else {
        return;
    };
    let camera_pos = camera_transform.translation();
    let viewport_width = window.resolution.width();
    let viewport_height = window.resolution.height();
    let falloff_metric = state.star_falloff_metric;
    let settings = StarBillboardRenderSettings::from_meta(&session.view_model.render_meta);
    // 11.6: render-only owned-set highlight from selected star's owner_flow_owner_ref.
    // Does not alter selection.selected_system_id or nameplate focus semantics.
    let owned_highlight = crate::studio_faction_nameplates::owned_star_highlight_system_ids(
        &session.scenario_authority,
        state.selection.selected_system_id,
    );
    let sync_key = StarVisualSyncKey {
        camera_position: quantize_billboard_camera_key(camera_pos.to_array()).position,
        selected_system_id: state.selection.selected_system_id,
        hovered_system_id: state.selection.hovered_system_id,
        render_mode: settings.render_mode,
        falloff_settings: star_falloff_settings_key(settings.falloff_settings()),
        view_model_generation: state.scene_render_revision,
    };
    if !star_visuals_should_sync(
        caches.star_visual.last_sync_key,
        sync_key,
        caches.star_visual.dirty,
    ) {
        return;
    }

    // A render-settings change marks the cache dirty but does NOT alter the per-star applied key
    // (which tracks only selection/hover/render-mode/depth/layer). Force a one-frame full re-apply so
    // Settings sliders (star radius/opacity/falloff/render-mode) take effect immediately instead of
    // waiting for a camera move to break the per-star key. Steady-state frames keep the cheap per-star
    // gate, so the dirty-gate performance gain is preserved.
    let force_resync = caches.star_visual.dirty;
    let use_plateau = falloff_metric.uses_plateau_curve();
    let map_context = falloff_state.valid.then_some(&falloff_state.context);

    let started = std::time::Instant::now();
    let mut entity_count = 0usize;
    for (star, mut transform, material_handle, mut applied_key) in &mut stars {
        entity_count += 1;
        // visual_selected = actual selected OR co-owned set highlight (render-only; not selection model).
        let visual_selected = crate::studio_faction_nameplates::star_visual_selected_for_owned_set(
            star.instance.system_id,
            state.selection.selected_system_id,
            &owned_highlight,
        );
        let hovered = state.selection.hovered_system_id == Some(star.instance.system_id);
        let instance = star.instance.with_view_state(visual_selected, hovered);
        let distance = camera_pos.distance(instance.anchor_position);
        let depth_percent = star_falloff_progress_percent(
            falloff_metric,
            camera,
            camera_transform,
            instance.anchor_position,
            distance,
            &settings,
            viewport_width,
            viewport_height,
            map_context,
        );
        let layer_code = match star.layer {
            StarVisualLayer::Core => 0,
            StarVisualLayer::Aura => 1,
        };
        let visual_key = StarVisualAppliedKey {
            selected: visual_selected,
            hovered,
            render_mode: settings.render_mode,
            depth_bucket_or_quantized_percent: quantize_star_depth_percent(depth_percent),
            layer: layer_code,
        };
        if !star_visual_per_star_should_write(force_resync, *applied_key, visual_key) {
            continue;
        }

        let visual = compute_star_distance_visual(
            depth_percent,
            visual_selected,
            hovered,
            &settings,
            use_plateau,
        );
        let (layer_scale, alpha, emissive_factor, color, texture) =
            match (settings.render_mode, star.layer) {
                (StarRenderMode::BloomStarburst, StarVisualLayer::Core) => (
                    visual.core_scale,
                    visual.core_alpha,
                    1.0,
                    (0.88, 0.95, 1.0),
                    assets.core_texture.clone(),
                ),
                (StarRenderMode::BloomStarburst, StarVisualLayer::Aura) => (
                    visual.aura_radius,
                    visual.aura_alpha,
                    0.20,
                    (0.34, 0.66, 1.0),
                    assets.aura_texture.clone(),
                ),
                (StarRenderMode::CrispCircle, StarVisualLayer::Core) => (
                    visual.core_scale,
                    visual.core_alpha,
                    1.0,
                    (0.88, 0.95, 1.0),
                    assets.circle_texture.clone(),
                ),
                (StarRenderMode::CrispCircle, StarVisualLayer::Aura) => (
                    visual.aura_radius,
                    0.0,
                    0.0,
                    (0.34, 0.66, 1.0),
                    assets.aura_texture.clone(),
                ),
            };
        transform.translation = instance.anchor_position;
        transform.scale = Vec3::splat(instance.base_scale_variation * layer_scale);
        if let Some(material) = materials.get_mut(&material_handle.0) {
            let emissive = star_emissive_strength(
                instance.base_intensity_variation,
                visual_selected,
                hovered,
            );
            let base_color = Color::srgba(color.0, color.1, color.2, alpha);
            let emissive_color = LinearRgba::new(
                emissive * 1.25 * alpha * emissive_factor,
                emissive * 1.32 * alpha * emissive_factor,
                emissive * 1.45 * alpha * emissive_factor,
                1.0,
            );
            if material.base_color_texture.as_ref() != Some(&texture) {
                material.base_color_texture = Some(texture.clone());
            }
            if material.emissive_texture.as_ref() != Some(&texture) {
                material.emissive_texture = Some(texture);
            }
            if material.base_color != base_color {
                material.base_color = base_color;
            }
            if material.emissive != emissive_color {
                material.emissive = emissive_color;
            }
        }
        *applied_key = visual_key;
    }

    let elapsed_ms = started.elapsed().as_secs_f64() * 1000.0;
    let call_count = perf.telemetry.star_visual_sync_calls;
    {
        let telemetry = &mut perf.telemetry;
        crate::studio_render_loop_dirty_gate::render_loop_telemetry_record_timing(
            &mut telemetry.star_visual_sync_last_ms,
            &mut telemetry.star_visual_sync_avg_ms,
            elapsed_ms,
            call_count,
        );
        telemetry.star_visual_entities_last_count = entity_count;
    }
    caches.star_visual.dirty = false;
    caches.star_visual.last_sync_key = Some(sync_key);
}

pub fn billboard_stars_system(
    camera: Query<&GlobalTransform, With<MainCamera>>,
    mut stars: Query<&mut Transform, With<GalaxyStar>>,
    mut caches: ResMut<StudioRenderLoopCaches>,
    mut perf: ResMut<StudioPerformanceTelemetryState>,
) {
    perf.telemetry.billboard_sync_calls = perf.telemetry.billboard_sync_calls.saturating_add(1);
    let Ok(cam) = camera.single() else {
        return;
    };
    let cam_pos = cam.translation();
    let camera_key = quantize_billboard_camera_key(cam_pos.to_array());
    if !billboard_should_sync(caches.billboard.last_camera_key, camera_key) {
        return;
    }

    let started = std::time::Instant::now();
    let mut entity_count = 0usize;
    for mut transform in &mut stars {
        entity_count += 1;
        transform.look_at(cam_pos, Vec3::Y);
    }
    let elapsed_ms = started.elapsed().as_secs_f64() * 1000.0;
    let call_count = perf.telemetry.billboard_sync_calls;
    {
        let telemetry = &mut perf.telemetry;
        crate::studio_render_loop_dirty_gate::render_loop_telemetry_record_timing(
            &mut telemetry.billboard_sync_last_ms,
            &mut telemetry.billboard_sync_avg_ms,
            elapsed_ms,
            call_count,
        );
        telemetry.billboard_entities_last_count = entity_count;
    }
    caches.billboard.last_camera_key = Some(camera_key);
}
