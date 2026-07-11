#![cfg(windows)]

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use simthing_tools::{StudioTypefaceLabel, TextLabelRenderMode, WorldTextBillboard};

use crate::hyperlane_buckets::{
    bucket_base_rgba, classify_hyperlane_camera_depth_bucket, compute_hyperlane_visual,
    hyperlane_camera_depth_percent, hyperlane_map_radius_progress_percent,
    hyperlane_midpoint_map_radius_progress_percent, selected_incident_lane_alpha,
    HyperlaneCameraDepthThresholds, HyperlaneDepthBucket, HYPERLANE_CORE_FRACTION,
};
use crate::hyperlane_ribbon::{
    count_non_finite_vertex_positions, hyperlane_rebuild_is_valid, hyperlane_ribbon_width_dir,
    is_valid_width_dir, HyperlaneMeshStats, HyperlaneRibbonBasis, HyperlaneRibbonCamera,
    HyperlaneWidthDirOutcome,
};
use crate::selection::incident_hyperlanes_for_system;
use crate::session::StudioSession;
use crate::star_render::{
    nearest_camera_star_disc_width_world, prepare_star_billboard_instances,
    star_nameplate_gpu_screen_label, StarBillboardInstance, StarBillboardRenderSettings,
    StarNameplateSettings,
};
use crate::starburst::{
    generate_star_aura_image, generate_star_circle_image, generate_starburst_image,
};
use crate::studio_render_loop_dirty_gate::{
    hyperlane_basis_mismatch_angle_deg, hyperlane_basis_mismatch_exceeds_epsilon,
    hyperlane_camera_basis_from_transform, hyperlane_render_settings_key,
    hyperlane_render_should_rebuild, quantize_hyperlane_camera_key, HyperlaneRenderCacheState,
    StarVisualAppliedKey, StarVisualSyncCacheState, StudioRenderLoopCaches,
    HYPERLANE_BASIS_MISMATCH_REBUILD_EPSILON_DEG,
};
use crate::view_model::{build_hyperlane_render_segments, HyperlaneRenderSegment};

use super::camera::{HyperlaneRibbonRenderPath, StudioCamera, StudioViewMode};
use super::performance_telemetry::StudioPerformanceTelemetryState;
use super::GalaxySceneRoot;

#[derive(Component)]
pub struct GalaxyStar {
    pub instance: StarBillboardInstance,
    pub layer: StarVisualLayer,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct GalaxyStarNameplate {
    pub instance: StarBillboardInstance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StarVisualLayer {
    Core,
    Aura,
}

#[derive(Component)]
pub struct GalaxyHyperlanes(pub HyperlaneDepthBucket);

#[derive(Component)]
pub struct SelectedHyperlaneHighlight;

#[derive(Resource)]
pub struct StarVisualAssets {
    pub core_texture: Handle<Image>,
    pub aura_texture: Handle<Image>,
    pub circle_texture: Handle<Image>,
    pub quad: Handle<Mesh>,
}

pub fn init_star_visual_assets(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let core_texture = images.add(generate_starburst_image(64));
    let aura_texture = images.add(generate_star_aura_image(64));
    let circle_texture = images.add(generate_star_circle_image(64));
    let quad = meshes.add(Rectangle::new(1.0, 1.0));
    commands.insert_resource(StarVisualAssets {
        core_texture,
        aura_texture,
        circle_texture,
        quad,
    });
}

pub fn rebuild_galaxy_scene(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    assets: &StarVisualAssets,
    root: &mut GalaxySceneRoot,
    session: &StudioSession,
) {
    despawn_galaxy(commands, root);
    let vm = &session.view_model;
    // 11.5: display name + owner faction color_rgb (unowned = neutral). Presentation only.
    let nameplates = crate::studio_faction_nameplates::star_nameplate_presentations(
        &session.scenario_authority,
    );
    let billboard_settings = StarBillboardRenderSettings::from_meta(&vm.render_meta);
    for star in prepare_star_billboard_instances(&vm.stars, &vm.render_anchors, None, None) {
        for layer in [StarVisualLayer::Aura, StarVisualLayer::Core] {
            let texture = match layer {
                StarVisualLayer::Core => assets.core_texture.clone(),
                StarVisualLayer::Aura => assets.aura_texture.clone(),
            };
            let (base_color, emissive_factor) = match layer {
                StarVisualLayer::Core => (Color::srgba(0.88, 0.95, 1.0, 0.9), 1.0),
                StarVisualLayer::Aura => (Color::srgba(0.34, 0.66, 1.0, 0.08), 0.22),
            };
            let material = materials.add(StandardMaterial {
                base_color,
                base_color_texture: Some(texture.clone()),
                emissive: LinearRgba::new(
                    star.base_intensity_variation * 1.25 * emissive_factor,
                    star.base_intensity_variation * 1.32 * emissive_factor,
                    star.base_intensity_variation * 1.45 * emissive_factor,
                    1.0,
                ),
                emissive_texture: Some(texture),
                unlit: true,
                alpha_mode: AlphaMode::Add,
                cull_mode: None,
                ..default()
            });
            let entity = commands
                .spawn((
                    Mesh3d(assets.quad.clone()),
                    MeshMaterial3d(material),
                    Transform::from_translation(star.anchor_position)
                        .with_scale(Vec3::splat(star.base_scale_variation)),
                    GalaxyStar {
                        instance: star,
                        layer,
                    },
                    StarVisualAppliedKey::default(),
                ))
                .id();
            root.stars.push((star.system_id, entity));
        }
        if let Some((display_name, rgba)) = nameplates.get(&star.system_id) {
            let entity = commands
                .spawn((
                    StudioTypefaceLabel::entity_name(display_name.clone(), 48.0, *rgba)
                        .with_render_mode(TextLabelRenderMode::Raster),
                    star_nameplate_gpu_screen_label(
                        star,
                        &billboard_settings,
                        StarNameplateSettings::default(),
                    ),
                    GalaxyStarNameplate { instance: star },
                    Visibility::Visible,
                ))
                .id();
            root.nameplates.push(entity);
        }
    }

    if vm.hyperlanes.is_empty() {
        return;
    }

    for bucket in HyperlaneDepthBucket::ALL {
        spawn_hyperlane_bucket(commands, meshes, materials, root, vm, bucket);
    }
}

fn spawn_hyperlane_bucket(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    root: &mut GalaxySceneRoot,
    vm: &crate::view_model::StudioGalaxyViewModel,
    bucket: HyperlaneDepthBucket,
) {
    let (mesh, _) = build_hyperlane_bucket_mesh(
        &vm.hyperlane_render_segments(),
        bucket,
        HyperlaneRibbonCamera::default(),
        &vm.render_meta,
        true,
        None,
    );
    let mesh_handle = meshes.add(mesh);
    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        cull_mode: None,
        ..default()
    });
    let idx = bucket_index(bucket);
    root.hyperlane_buckets[idx] = Some(
        commands
            .spawn((
                Mesh3d(mesh_handle),
                MeshMaterial3d(material),
                GalaxyHyperlanes(bucket),
            ))
            .id(),
    );
}

pub fn rebuild_highlight_hyperlanes(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    root: &mut GalaxySceneRoot,
    session: &StudioSession,
    selected_system_id: Option<u32>,
) {
    if let Some(entity) = root.highlight_hyperlanes.take() {
        commands.entity(entity).despawn();
    }
    let Some(selected_id) = selected_system_id else {
        return;
    };
    let incident = incident_hyperlanes_for_system(&session.view_model.hyperlanes, selected_id);
    if incident.is_empty() {
        return;
    }
    let mut positions = Vec::with_capacity(incident.len() * 2);
    let segments = session.view_model.hyperlane_render_segments();
    for (from_id, to_id) in &incident {
        let Some(lane) = segments
            .iter()
            .find(|lane| lane.from_system_id == *from_id && lane.to_system_id == *to_id)
        else {
            continue;
        };
        positions.push([lane.from[0], lane.from[1], lane.from[2]]);
        positions.push([lane.to[0], lane.to[1], lane.to[2]]);
    }
    if positions.is_empty() {
        return;
    }
    let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    let mesh_handle = meshes.add(mesh);
    let material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.72, 0.92, 1.0, selected_incident_lane_alpha()),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    root.highlight_hyperlanes = Some(
        commands
            .spawn((
                Mesh3d(mesh_handle),
                MeshMaterial3d(material),
                SelectedHyperlaneHighlight,
            ))
            .id(),
    );
}

fn bucket_index(bucket: HyperlaneDepthBucket) -> usize {
    match bucket {
        HyperlaneDepthBucket::Near => 0,
        HyperlaneDepthBucket::Mid => 1,
        HyperlaneDepthBucket::Far => 2,
    }
}

fn despawn_galaxy(commands: &mut Commands, root: &mut GalaxySceneRoot) {
    for (_, entity) in root.stars.drain(..) {
        commands.entity(entity).despawn();
    }
    for entity in root.nameplates.drain(..) {
        commands.entity(entity).despawn();
    }
    for slot in root.hyperlane_buckets.iter_mut() {
        if let Some(entity) = slot.take() {
            commands.entity(entity).despawn();
        }
    }
    if let Some(entity) = root.highlight_hyperlanes.take() {
        commands.entity(entity).despawn();
    }
    if let Some(entity) = root.core_glow.take() {
        commands.entity(entity).despawn();
    }
}

pub fn format_simthing_nameplate_id(raw_id: u32) -> String {
    crate::studio_faction_nameplates::fallback_simthing_nameplate_id(raw_id)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct NameplateSyncKey {
    star: crate::star_render::StarFalloffSettings,
    nameplate: StarNameplateSettings,
    scene_revision: u64,
}

pub(super) fn sync_star_nameplate_settings_system(
    state: Res<super::StudioAppState>,
    settings: Res<super::resources::StudioSettings>,
    mut nameplates: Query<(&GalaxyStarNameplate, &mut WorldTextBillboard)>,
    mut last_key: Local<Option<NameplateSyncKey>>,
) {
    let Some(session) = state.session.as_ref() else {
        return;
    };
    let star_settings = StarBillboardRenderSettings::from_meta(&session.view_model.render_meta);
    let key = NameplateSyncKey {
        star: star_settings.falloff_settings(),
        nameplate: settings.star_nameplate_settings(),
        scene_revision: state.scene_render_revision,
    };
    if *last_key == Some(key) {
        return;
    }
    for (nameplate, mut placement) in &mut nameplates {
        let next =
            star_nameplate_gpu_screen_label(nameplate.instance, &star_settings, key.nameplate);
        if *placement != next {
            *placement = next;
        }
    }
    *last_key = Some(key);
}

pub(super) fn sync_star_nameplate_focus_system(
    state: Res<super::StudioAppState>,
    mut nameplates: Query<(&mut GalaxyStarNameplate, &mut WorldTextBillboard)>,
    mut last_selection: Local<(Option<u32>, Option<u32>)>,
) {
    let current = (
        state.selection.selected_system_id,
        state.selection.hovered_system_id,
    );
    if *last_selection == current {
        return;
    }
    *last_selection = current;
    for (mut nameplate, mut placement) in &mut nameplates {
        let selected = state.selection.selected_system_id == Some(nameplate.instance.system_id);
        let hovered = state.selection.hovered_system_id == Some(nameplate.instance.system_id);
        let next_focused = selected || hovered;
        if nameplate.instance.selected != selected
            || nameplate.instance.hovered != hovered
            || placement.gpu_screen_label_focused != next_focused
        {
            nameplate.instance.selected = selected;
            nameplate.instance.hovered = hovered;
            placement.gpu_screen_label_focused = next_focused;
        }
    }
}

fn ribbon_basis_for_view_mode(view_mode: StudioViewMode) -> HyperlaneRibbonBasis {
    match view_mode.hyperlane_render_path() {
        HyperlaneRibbonRenderPath::CameraFacing3D => HyperlaneRibbonBasis::CameraFacing3D,
        HyperlaneRibbonRenderPath::OverheadLegibility => HyperlaneRibbonBasis::OverheadLegibility,
    }
}

fn view_mode_key(view_mode: StudioViewMode) -> u8 {
    match view_mode {
        StudioViewMode::ThreeD => 0,
        StudioViewMode::OverheadStrategic => 1,
    }
}

fn format_camera_key(key: crate::studio_render_loop_dirty_gate::HyperlaneCameraKey) -> String {
    format!(
        "pos [{}, {}, {}] right [{}, {}, {}] up [{}, {}, {}] forward [{}, {}, {}] mode {}",
        key.position[0],
        key.position[1],
        key.position[2],
        key.right[0],
        key.right[1],
        key.right[2],
        key.up[0],
        key.up[1],
        key.up[2],
        key.forward[0],
        key.forward[1],
        key.forward[2],
        key.view_mode,
    )
}
pub fn sync_hyperlane_colors_system(
    app_state: Res<super::StudioAppState>,
    studio_camera: Res<StudioCamera>,
    camera_transform: Query<&Transform, With<super::camera::MainCamera>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    hyperlanes: Query<(
        &Mesh3d,
        &MeshMaterial3d<StandardMaterial>,
        &GalaxyHyperlanes,
    )>,
    mut caches: ResMut<StudioRenderLoopCaches>,
    mut perf: ResMut<StudioPerformanceTelemetryState>,
    falloff_state: Res<super::StudioMapRadiusFalloffState>,
) {
    perf.telemetry.hyperlane_sync_calls = perf.telemetry.hyperlane_sync_calls.saturating_add(1);
    perf.telemetry.render_frame_index = perf.telemetry.render_frame_index.saturating_add(1);

    let Some(session) = app_state.session.as_ref() else {
        return;
    };
    if session.view_model.hyperlanes.is_empty() {
        return;
    };
    let Ok(cam_transform) = camera_transform.single() else {
        return;
    };
    let current_basis = hyperlane_camera_basis_from_transform(cam_transform);
    let cam_right = current_basis.right;
    let cam_up = current_basis.up;
    let cam_forward = current_basis.forward;
    let view_mode = studio_camera.view_mode();
    let camera_key = quantize_hyperlane_camera_key(
        current_basis.position,
        cam_right,
        cam_up,
        cam_forward,
        view_mode_key(view_mode),
    );
    let settings_key = hyperlane_render_settings_key(app_state.hyperlane_render_settings);
    let generation = app_state.scene_render_revision;
    let cache = &mut caches.hyperlane;
    let rotation_active = studio_camera.rmb_held;
    let rotation_just_ended = cache.last_rmb_held && !rotation_active;
    cache.last_rmb_held = rotation_active;
    let use_plateau = app_state.star_falloff_metric.uses_plateau_curve();
    let map_context = falloff_state.valid.then_some(&falloff_state.context);

    let mesh_build_basis = cache.last_mesh_build_basis;
    let basis_mismatch = hyperlane_basis_mismatch_exceeds_epsilon(
        current_basis,
        mesh_build_basis,
        HYPERLANE_BASIS_MISMATCH_REBUILD_EPSILON_DEG,
    );
    let basis_mismatch_right_deg = mesh_build_basis.map_or(0.0, |last| {
        hyperlane_basis_mismatch_angle_deg(current_basis.right, last.right)
    });
    let basis_mismatch_up_deg = mesh_build_basis.map_or(0.0, |last| {
        hyperlane_basis_mismatch_angle_deg(current_basis.up, last.up)
    });
    let basis_mismatch_forward_deg = mesh_build_basis.map_or(0.0, |last| {
        hyperlane_basis_mismatch_angle_deg(current_basis.forward, last.forward)
    });
    let rotation_delta_since_rebuild_deg = mesh_build_basis.map_or(0.0, |last| {
        hyperlane_basis_mismatch_angle_deg(current_basis.forward, last.forward)
    });

    {
        let telemetry = &mut perf.telemetry;
        telemetry.hyperlane_last_camera_key = cache
            .last_camera_key
            .map(format_camera_key)
            .unwrap_or_else(|| "—".into());
        telemetry.hyperlane_current_camera_key = format_camera_key(camera_key);
        telemetry.hyperlane_camera_right = cam_right;
        telemetry.hyperlane_camera_up = cam_up;
        telemetry.hyperlane_camera_forward = cam_forward;
        telemetry.hyperlane_view_mode = view_mode_key(view_mode);
        telemetry.hyperlane_mesh_build_camera_right =
            mesh_build_basis.map(|b| b.right).unwrap_or([f32::NAN; 3]);
        telemetry.hyperlane_mesh_build_camera_up =
            mesh_build_basis.map(|b| b.up).unwrap_or([f32::NAN; 3]);
        telemetry.hyperlane_mesh_build_camera_forward =
            mesh_build_basis.map(|b| b.forward).unwrap_or([f32::NAN; 3]);
        telemetry.hyperlane_mesh_build_camera_key = cache
            .last_camera_key
            .map(format_camera_key)
            .unwrap_or_else(|| "—".into());
        telemetry.hyperlane_basis_mismatch_right_deg = basis_mismatch_right_deg;
        telemetry.hyperlane_basis_mismatch_up_deg = basis_mismatch_up_deg;
        telemetry.hyperlane_basis_mismatch_forward_deg = basis_mismatch_forward_deg;
        telemetry.hyperlane_frames_since_rebuild = cache.frames_since_rebuild;
        telemetry.hyperlane_rmb_orbit_active = rotation_active;
        telemetry.hyperlane_rotation_delta_since_rebuild_deg = rotation_delta_since_rebuild_deg;
        telemetry.hyperlane_stale_basis_rebuild_count = cache.stale_basis_rebuild_count;
        telemetry.hyperlane_basis_mismatch_active = basis_mismatch;
    }

    let should_rebuild = hyperlane_render_should_rebuild(
        cache.last_camera_key,
        camera_key,
        cache.last_render_settings_key,
        settings_key,
        cache.last_view_model_generation,
        generation,
        cache.dirty,
        rotation_active,
        rotation_just_ended,
        basis_mismatch,
    );
    if !should_rebuild {
        cache.frames_since_rebuild = cache.frames_since_rebuild.saturating_add(1);
        return;
    }

    if basis_mismatch && mesh_build_basis.is_some() {
        cache.stale_basis_rebuild_count = cache.stale_basis_rebuild_count.saturating_add(1);
    }

    let started = std::time::Instant::now();
    let camera = HyperlaneRibbonCamera {
        position: current_basis.position,
        right: cam_right,
        up: cam_up,
        forward: cam_forward,
        basis: ribbon_basis_for_view_mode(view_mode),
    };
    let meta = &session.view_model.render_meta;
    let segments = build_hyperlane_render_segments(
        &session.view_model.hyperlanes,
        &session.view_model.render_anchors,
    );
    let base_opacity = app_state
        .hyperlane_render_settings
        .clamped()
        .base_opacity_percent;

    let mut built_buckets = Vec::new();
    for (mesh_handle, mat_handle, marker) in &hyperlanes {
        let (built, stats) = build_hyperlane_bucket_mesh(
            &segments,
            marker.0,
            camera,
            meta,
            use_plateau,
            map_context,
        );
        built_buckets.push((mesh_handle.clone(), mat_handle.clone(), built, stats));
    }

    let bucket_stats: Vec<HyperlaneMeshStats> = built_buckets
        .iter()
        .map(|(_, _, _, stats)| *stats)
        .collect();
    let rebuild_valid = hyperlane_rebuild_is_valid(&bucket_stats, segments.len(), base_opacity);

    if !rebuild_valid {
        cache.dirty = true;
        perf.telemetry.hyperlane_invalid_rebuild_rejected = perf
            .telemetry
            .hyperlane_invalid_rebuild_rejected
            .saturating_add(1);
        if !cache.invalid_rebuild_warning_logged {
            bevy::log::warn!(
                "hyperlane mesh rebuild rejected: invalid ribbon geometry at camera key {}",
                format_camera_key(camera_key)
            );
            cache.invalid_rebuild_warning_logged = true;
        }
        record_hyperlane_bucket_telemetry(&mut perf.telemetry, &bucket_stats, segments.len());
        if let (Some(ctx), Some(first)) = (map_context, segments.first()) {
            record_hyperlane_falloff_sample_telemetry(
                &mut perf.telemetry,
                ctx,
                first.from,
                first.to,
            );
        }
        return;
    }

    cache.invalid_rebuild_warning_logged = false;

    let mut total_vertices = 0usize;
    let mut total_indices = 0usize;
    for (mesh_handle, mat_handle, built, stats) in built_buckets {
        total_vertices = total_vertices.saturating_add(stats.vertex_count);
        total_indices = total_indices.saturating_add(stats.index_count);
        if let Some(mesh) = meshes.get_mut(&mesh_handle.0) {
            *mesh = built;
        }
        if let Some(material) = materials.get_mut(&mat_handle.0) {
            if material.base_color != Color::WHITE {
                material.base_color = Color::WHITE;
            }
        }
    }

    let elapsed_ms = started.elapsed().as_secs_f64() * 1000.0;
    perf.telemetry.hyperlane_mesh_rebuilds =
        perf.telemetry.hyperlane_mesh_rebuilds.saturating_add(1);
    perf.telemetry.hyperlane_rebuild_count = perf.telemetry.hyperlane_mesh_rebuilds;
    let rebuild_count = perf.telemetry.hyperlane_mesh_rebuilds;
    {
        let telemetry = &mut perf.telemetry;
        crate::studio_render_loop_dirty_gate::render_loop_telemetry_record_timing(
            &mut telemetry.hyperlane_mesh_rebuild_last_ms,
            &mut telemetry.hyperlane_mesh_rebuild_avg_ms,
            elapsed_ms,
            rebuild_count,
        );
        telemetry.hyperlane_segments_last_count = segments.len();
        telemetry.hyperlane_vertices_last_count = total_vertices;
        telemetry.hyperlane_indices_last_count = total_indices;
        record_hyperlane_bucket_telemetry(telemetry, &bucket_stats, segments.len());
        if let (Some(ctx), Some(first)) = (map_context, segments.first()) {
            record_hyperlane_falloff_sample_telemetry(telemetry, ctx, first.from, first.to);
        }
    }

    cache.dirty = false;
    cache.last_camera_key = Some(camera_key);
    cache.last_render_settings_key = Some(settings_key);
    cache.last_view_model_generation = generation;
    cache.last_mesh_build_basis = Some(current_basis);
    cache.frames_since_rebuild = 0;
}

fn record_hyperlane_falloff_sample_telemetry(
    telemetry: &mut crate::studio_performance_telemetry::StudioPerformanceTelemetry,
    ctx: &crate::falloff_metric::StudioMapRadiusFalloffContext,
    from: [f32; 3],
    to: [f32; 3],
) {
    telemetry.hyperlane_falloff_sample_mode = "closest segment point".into();
    telemetry.hyperlane_falloff_sample_midpoint_progress_pct = Some(
        hyperlane_midpoint_map_radius_progress_percent(ctx, from, to),
    );
    telemetry.hyperlane_falloff_sample_closest_progress_pct =
        Some(hyperlane_map_radius_progress_percent(ctx, from, to));
}

fn record_hyperlane_bucket_telemetry(
    telemetry: &mut crate::studio_performance_telemetry::StudioPerformanceTelemetry,
    bucket_stats: &[HyperlaneMeshStats],
    source_segments: usize,
) {
    telemetry.hyperlane_source_segment_count = source_segments;
    telemetry.hyperlane_degenerate_width_dir_count = bucket_stats
        .iter()
        .map(|s| s.degenerate_width_dir_count)
        .sum();
    telemetry.hyperlane_nan_inf_vertex_count =
        bucket_stats.iter().map(|s| s.nan_inf_vertex_count).sum();
    telemetry.hyperlane_zero_length_segment_count = bucket_stats
        .iter()
        .map(|s| s.zero_length_segment_count)
        .sum();
    telemetry.hyperlane_falloff_culled_segment_count = bucket_stats
        .iter()
        .map(|s| s.falloff_culled_segment_count)
        .sum();
    for (idx, stats) in bucket_stats.iter().enumerate().take(3) {
        telemetry.hyperlane_bucket_segment_count[idx] = stats.bucket_segment_count;
        telemetry.hyperlane_bucket_vertex_count[idx] = stats.vertex_count;
        telemetry.hyperlane_bucket_index_count[idx] = stats.index_count;
    }
}

/// Mark hyperlane meshes dirty after scene rebuild or render-settings change.
pub fn mark_hyperlane_render_dirty(cache: &mut HyperlaneRenderCacheState) {
    cache.dirty = true;
    cache.last_camera_key = None;
    cache.last_render_settings_key = None;
    cache.last_mesh_build_basis = None;
    cache.frames_since_rebuild = 0;
}

/// Mark star visual material/scale sync dirty after scene rebuild or render-settings change.
pub fn mark_star_visual_render_dirty(cache: &mut StarVisualSyncCacheState) {
    cache.dirty = true;
    cache.last_sync_key = None;
}

fn build_hyperlane_bucket_mesh(
    segments: &[HyperlaneRenderSegment],
    bucket: HyperlaneDepthBucket,
    camera: HyperlaneRibbonCamera,
    meta: &crate::view_model::StudioGalaxyRenderMeta,
    use_plateau: bool,
    map_context: Option<&crate::falloff_metric::StudioMapRadiusFalloffContext>,
) -> (Mesh, HyperlaneMeshStats) {
    let mut positions = Vec::new();
    let mut colors = Vec::new();
    let mut indices = Vec::new();
    let mut stats = HyperlaneMeshStats {
        source_segment_count: segments.len(),
        ..Default::default()
    };
    let thresholds = HyperlaneCameraDepthThresholds::from_meta(meta);
    let nearest_star_width = nearest_camera_star_disc_width_world(meta);
    for lane in segments {
        let lane_bucket =
            classify_hyperlane_camera_depth_bucket(camera.position, lane.from, lane.to, thresholds);
        if lane_bucket != bucket {
            continue;
        }
        let depth_percent = if use_plateau {
            map_context
                .map(|ctx| hyperlane_map_radius_progress_percent(ctx, lane.from, lane.to))
                .unwrap_or(100.0)
        } else {
            hyperlane_camera_depth_percent(camera.position, lane.from, lane.to, meta)
        };
        let visual = compute_hyperlane_visual(
            depth_percent,
            nearest_star_width,
            &meta.hyperlane_render_settings,
            use_plateau,
        );
        if !visual.visible {
            stats.falloff_culled_segment_count =
                stats.falloff_culled_segment_count.saturating_add(1);
            continue;
        }
        stats.bucket_segment_count = stats.bucket_segment_count.saturating_add(1);
        let delta = Vec3::from_array(lane.to) - Vec3::from_array(lane.from);
        if delta.length_squared() <= f32::EPSILON {
            stats.zero_length_segment_count = stats.zero_length_segment_count.saturating_add(1);
            continue;
        }
        let (width_dir, outcome) = hyperlane_ribbon_width_dir(lane.from, lane.to, camera);
        if !is_valid_width_dir(width_dir) {
            stats.degenerate_width_dir_count = stats.degenerate_width_dir_count.saturating_add(1);
            continue;
        }
        if outcome != HyperlaneWidthDirOutcome::CameraFacingCross {
            stats.degenerate_width_dir_count = stats.degenerate_width_dir_count.saturating_add(1);
        }
        push_hyperlane_visual_strip(
            &mut positions,
            &mut colors,
            &mut indices,
            lane.from,
            lane.to,
            width_dir.to_array(),
            bucket,
            visual.thickness_world,
            visual.core_opacity,
        );
    }
    stats.nan_inf_vertex_count = count_non_finite_vertex_positions(&positions);
    stats.vertex_count = positions.len();
    stats.index_count = indices.len();
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));
    (mesh, stats)
}

fn push_hyperlane_visual_strip(
    positions: &mut Vec<[f32; 3]>,
    colors: &mut Vec<[f32; 4]>,
    indices: &mut Vec<u32>,
    from: [f32; 3],
    to: [f32; 3],
    width_dir: [f32; 3],
    bucket: HyperlaneDepthBucket,
    thickness_world: f32,
    core_opacity: f32,
) {
    let from = Vec3::from_array(from);
    let to = Vec3::from_array(to);
    let perp = Vec3::from_array(width_dir);
    let perp = if is_valid_width_dir(perp) {
        perp.normalize()
    } else {
        Vec3::X
    };
    let half = thickness_world * 0.5;
    let core_half = half * HYPERLANE_CORE_FRACTION;
    let offsets = [-half, -core_half, core_half, half];
    let alphas = [0.0, core_opacity, core_opacity, 0.0];
    let (r, g, b, _) = bucket_base_rgba(bucket);
    let base_index = positions.len() as u32;
    for (offset, alpha) in offsets.into_iter().zip(alphas) {
        let offset_vec = perp * offset;
        positions.push((from + offset_vec).to_array());
        colors.push([r, g, b, alpha]);
        positions.push((to + offset_vec).to_array());
        colors.push([r, g, b, alpha]);
    }
    for strip in 0..3 {
        let i = base_index + strip * 2;
        indices.extend_from_slice(&[i, i + 1, i + 2, i + 1, i + 3, i + 2]);
    }
}

pub fn sync_render_debug_visibility_system(
    state: Res<super::StudioAppState>,
    mut visibility_queries: ParamSet<(
        Query<(&GalaxyStar, &mut Visibility)>,
        Query<&mut Visibility, With<GalaxyHyperlanes>>,
        Query<&mut Visibility, With<SelectedHyperlaneHighlight>>,
        Query<&mut Visibility, With<GalaxyStarNameplate>>,
    )>,
) {
    for (star, mut visibility) in &mut visibility_queries.p0() {
        let star_visible = state.show_stars
            && !(state.performance_diagnostic_hide_star_aura
                && star.layer == StarVisualLayer::Aura);
        *visibility = if star_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    for mut visibility in &mut visibility_queries.p1() {
        *visibility = if state.show_hyperlanes
            && state
                .hyperlane_render_settings
                .clamped()
                .base_opacity_percent
                > 0.0
        {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    for mut visibility in &mut visibility_queries.p2() {
        *visibility = if state.show_hyperlanes
            && state
                .hyperlane_render_settings
                .clamped()
                .base_opacity_percent
                > 0.0
        {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    for mut visibility in &mut visibility_queries.p3() {
        *visibility = if state.show_stars {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_camera_facing_width_dir;

}
