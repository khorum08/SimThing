#![cfg(windows)]

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use simthing_tools::{StudioTypefaceLabel, TextLabelRenderMode, WorldTextBillboard};

use crate::hyperlane_buckets::{
    bucket_base_rgba, classify_hyperlane_camera_depth_bucket, compute_hyperlane_visual,
    hyperlane_camera_depth_percent, selected_incident_lane_alpha, HyperlaneCameraDepthThresholds,
    HyperlaneDepthBucket, HYPERLANE_CORE_FRACTION,
};
use crate::selection::incident_hyperlanes_for_system;
use crate::session::StudioSession;
use crate::star_render::{
    nearest_camera_star_disc_width_world, prepare_star_billboard_instances,
    star_nameplate_world_billboard, StarBillboardInstance, StarBillboardRenderSettings,
    StarNameplateSettings,
};
use crate::starburst::{
    generate_star_aura_image, generate_star_circle_image, generate_starburst_image,
};
use crate::studio_render_loop_dirty_gate::{
    hyperlane_render_settings_key, hyperlane_render_should_rebuild, quantize_hyperlane_camera_key,
    HyperlaneRenderCacheState, StarVisualAppliedKey, StarVisualSyncCacheState,
    StudioRenderLoopCaches,
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
    let simthing_ids: std::collections::HashMap<u32, u32> = session
        .scenario_authority
        .structural_grid
        .placements
        .iter()
        .map(|placement| (placement.system_id, placement.simthing_id_raw))
        .collect();
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
        if let Some(simthing_id) = simthing_ids.get(&star.system_id).copied() {
            let entity = commands
                .spawn((
                    StudioTypefaceLabel::entity_name(
                        format_simthing_nameplate_id(simthing_id),
                        48.0,
                        [0.92, 0.96, 1.0, 1.0],
                    )
                    .with_render_mode(TextLabelRenderMode::Raster),
                    star_nameplate_world_billboard(
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
    let mesh = build_hyperlane_bucket_mesh(
        &vm.hyperlane_render_segments(),
        bucket,
        HyperlaneRibbonCamera::default(),
        &vm.render_meta,
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

fn mesh_vertex_count(mesh: &Mesh) -> usize {
    mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        .map(|attr| attr.len())
        .unwrap_or(0)
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
    format!("SIM-{raw_id:06}")
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
    let key = NameplateSyncKey {
        star: state.star_falloff_settings.clamped(),
        nameplate: settings.star_nameplate_settings(),
        scene_revision: state.scene_render_revision,
    };
    if *last_key == Some(key) {
        return;
    }
    let star_settings = StarBillboardRenderSettings::from_meta(&session.view_model.render_meta);
    for (nameplate, mut placement) in &mut nameplates {
        let next =
            star_nameplate_world_billboard(nameplate.instance, &star_settings, key.nameplate);
        if *placement != next {
            *placement = next;
        }
    }
    *last_key = Some(key);
}

fn view_mode_key(view_mode: StudioViewMode) -> u8 {
    match view_mode {
        StudioViewMode::ThreeD => 0,
        StudioViewMode::OverheadStrategic => 1,
    }
}

pub fn sync_hyperlane_colors_system(
    app_state: Res<super::StudioAppState>,
    studio_camera: Res<StudioCamera>,
    camera_transform: Query<&GlobalTransform, With<super::camera::MainCamera>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    hyperlanes: Query<(
        &Mesh3d,
        &MeshMaterial3d<StandardMaterial>,
        &GalaxyHyperlanes,
    )>,
    mut caches: ResMut<StudioRenderLoopCaches>,
    mut perf: ResMut<StudioPerformanceTelemetryState>,
) {
    perf.telemetry.hyperlane_sync_calls = perf.telemetry.hyperlane_sync_calls.saturating_add(1);
    perf.telemetry.render_frame_index = perf.telemetry.render_frame_index.saturating_add(1);

    let Some(session) = app_state.session.as_ref() else {
        return;
    };
    if session.view_model.hyperlanes.is_empty() {
        return;
    }
    let Ok(cam) = camera_transform.single() else {
        return;
    };
    let cam_pos = cam.translation();
    let cam_transform = cam.compute_transform();
    let view_mode = studio_camera.view_mode();
    let camera_key = quantize_hyperlane_camera_key(
        cam_pos.to_array(),
        (cam_transform.rotation * Vec3::X).to_array(),
        (cam_transform.rotation * Vec3::Y).to_array(),
        view_mode_key(view_mode),
    );
    let settings_key = hyperlane_render_settings_key(app_state.hyperlane_render_settings);
    let generation = app_state.scene_render_revision;
    let cache = &mut caches.hyperlane;
    let should_rebuild = hyperlane_render_should_rebuild(
        cache.last_camera_key,
        camera_key,
        cache.last_render_settings_key,
        settings_key,
        cache.last_view_model_generation,
        generation,
        cache.dirty,
    );
    if !should_rebuild {
        return;
    }

    let started = std::time::Instant::now();
    let camera = HyperlaneRibbonCamera {
        position: cam_pos.to_array(),
        right: (cam_transform.rotation * Vec3::X).to_array(),
        up: (cam_transform.rotation * Vec3::Y).to_array(),
        view_mode,
    };
    let meta = &session.view_model.render_meta;
    let segments = build_hyperlane_render_segments(
        &session.view_model.hyperlanes,
        &session.view_model.render_anchors,
    );

    let mut total_vertices = 0usize;
    let mut total_indices = 0usize;
    for (mesh_handle, mat_handle, marker) in &hyperlanes {
        let built = build_hyperlane_bucket_mesh(&segments, marker.0, camera, meta);
        total_vertices = total_vertices.saturating_add(mesh_vertex_count(&built));
        if let Some(indices) = built.indices() {
            total_indices = total_indices.saturating_add(match indices {
                Indices::U16(values) => values.len(),
                Indices::U32(values) => values.len(),
            });
        }
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
    }

    cache.dirty = false;
    cache.last_camera_key = Some(camera_key);
    cache.last_render_settings_key = Some(settings_key);
    cache.last_view_model_generation = generation;
}

/// Mark hyperlane meshes dirty after scene rebuild or render-settings change.
pub fn mark_hyperlane_render_dirty(cache: &mut HyperlaneRenderCacheState) {
    cache.dirty = true;
    cache.last_camera_key = None;
    cache.last_render_settings_key = None;
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
) -> Mesh {
    let mut positions = Vec::new();
    let mut colors = Vec::new();
    let mut indices = Vec::new();
    let thresholds = HyperlaneCameraDepthThresholds::from_meta(meta);
    let nearest_star_width = nearest_camera_star_disc_width_world(meta);
    for lane in segments {
        let lane_bucket =
            classify_hyperlane_camera_depth_bucket(camera.position, lane.from, lane.to, thresholds);
        if lane_bucket != bucket {
            continue;
        }
        let depth_percent =
            hyperlane_camera_depth_percent(camera.position, lane.from, lane.to, meta);
        let visual = compute_hyperlane_visual(
            depth_percent,
            nearest_star_width,
            &meta.hyperlane_render_settings,
        );
        if !visual.visible {
            continue;
        }
        let width_dir = hyperlane_ribbon_width_dir(lane.from, lane.to, camera);
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
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

#[derive(Debug, Clone, Copy)]
struct HyperlaneRibbonCamera {
    position: [f32; 3],
    right: [f32; 3],
    up: [f32; 3],
    view_mode: StudioViewMode,
}

impl Default for HyperlaneRibbonCamera {
    fn default() -> Self {
        Self {
            position: [40.0, 35.0, 40.0],
            right: [1.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            view_mode: StudioViewMode::ThreeD,
        }
    }
}

fn hyperlane_ribbon_width_dir(from: [f32; 3], to: [f32; 3], camera: HyperlaneRibbonCamera) -> Vec3 {
    let from = Vec3::from_array(from);
    let to = Vec3::from_array(to);
    let Some(lane_dir) = normalized(to - from) else {
        return Vec3::X;
    };
    match camera.view_mode.hyperlane_render_path() {
        HyperlaneRibbonRenderPath::CameraFacing3D
        | HyperlaneRibbonRenderPath::OverheadLegibility => {
            let midpoint = (from + to) * 0.5;
            let view_dir =
                normalized(Vec3::from_array(camera.position) - midpoint).unwrap_or(Vec3::Y);
            compute_camera_facing_width_dir(
                lane_dir,
                view_dir,
                Vec3::from_array(camera.right),
                Vec3::from_array(camera.up),
            )
        }
    }
}

fn compute_camera_facing_width_dir(
    lane_dir: Vec3,
    view_dir: Vec3,
    camera_right: Vec3,
    camera_up: Vec3,
) -> Vec3 {
    let lane_dir = normalized(lane_dir).unwrap_or(Vec3::Z);
    if let Some(width) = normalized(lane_dir.cross(view_dir)) {
        return width;
    }
    stable_perpendicular(lane_dir, camera_right, camera_up)
}

fn stable_perpendicular(axis: Vec3, primary: Vec3, secondary: Vec3) -> Vec3 {
    for candidate in [primary, secondary, Vec3::Y, Vec3::X, Vec3::Z] {
        let projected = candidate - axis * candidate.dot(axis);
        if let Some(width) = normalized(projected) {
            return width;
        }
    }
    Vec3::X
}

fn normalized(value: Vec3) -> Option<Vec3> {
    if value.length_squared() > f32::EPSILON {
        Some(value.normalize())
    } else {
        None
    }
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
    let perp = normalized(Vec3::from_array(width_dir)).unwrap_or(Vec3::X);
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

    #[test]
    fn nameplate_formats_the_authoritative_raw_simthing_id() {
        assert_eq!(format_simthing_nameplate_id(42), "SIM-000042");
    }

    #[test]
    fn hyperlane_visual_strip_uses_transparent_edges_and_opaque_core() {
        let mut positions = Vec::new();
        let mut colors = Vec::new();
        let mut indices = Vec::new();
        push_hyperlane_visual_strip(
            &mut positions,
            &mut colors,
            &mut indices,
            [0.0, 0.0, 0.0],
            [10.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            HyperlaneDepthBucket::Near,
            1.0,
            0.6,
        );
        assert_eq!(positions.len(), 8);
        assert_eq!(indices.len(), 18);
        assert_eq!(colors[0][3], 0.0);
        assert!((colors[2][3] - 0.6).abs() < f32::EPSILON);
        assert!((colors[5][3] - 0.6).abs() < f32::EPSILON);
        assert_eq!(colors[7][3], 0.0);
    }

    #[test]
    fn camera_facing_ribbon_width_is_nonzero_for_edge_on_lane() {
        let width = compute_camera_facing_width_dir(Vec3::X, Vec3::Z, Vec3::X, Vec3::Y);
        assert!(width.length() > 0.99);
        assert!(width.dot(Vec3::X).abs() < 1e-5);
    }

    #[test]
    fn camera_facing_ribbon_uses_render_anchor_endpoints() {
        let mut positions = Vec::new();
        let mut colors = Vec::new();
        let mut indices = Vec::new();
        let from = [2.0, 4.0, 6.0];
        let to = [12.0, 4.0, 6.0];
        push_hyperlane_visual_strip(
            &mut positions,
            &mut colors,
            &mut indices,
            from,
            to,
            [0.0, 0.0, 1.0],
            HyperlaneDepthBucket::Near,
            2.0,
            0.8,
        );
        let from_mid = (Vec3::from_array(positions[0]) + Vec3::from_array(positions[6])) * 0.5;
        let to_mid = (Vec3::from_array(positions[1]) + Vec3::from_array(positions[7])) * 0.5;
        assert_eq!(from_mid.to_array(), from);
        assert_eq!(to_mid.to_array(), to);
    }

    #[test]
    fn camera_facing_ribbon_degenerate_case_uses_stable_fallback() {
        let width = compute_camera_facing_width_dir(Vec3::X, Vec3::X, Vec3::X, Vec3::Y);
        assert!(width.length() > 0.99);
        assert!(width.dot(Vec3::X).abs() < 1e-5);
        assert!(width.dot(Vec3::Y).abs() > 0.99);
    }

    #[test]
    fn hyperlane_ribbon_preserves_anchor_height() {
        let mut positions = Vec::new();
        let mut colors = Vec::new();
        let mut indices = Vec::new();
        push_hyperlane_visual_strip(
            &mut positions,
            &mut colors,
            &mut indices,
            [0.0, 4.0, 0.0],
            [10.0, 8.0, 0.0],
            [0.0, 0.0, 1.0],
            HyperlaneDepthBucket::Near,
            1.0,
            0.6,
        );
        for from_vertex in [0, 2, 4, 6] {
            assert_eq!(positions[from_vertex][1], 4.0);
        }
        for to_vertex in [1, 3, 5, 7] {
            assert_eq!(positions[to_vertex][1], 8.0);
        }
    }

    #[test]
    fn hyperlane_ribbon_thickness_applies_settings() {
        let mut positions = Vec::new();
        let mut colors = Vec::new();
        let mut indices = Vec::new();
        push_hyperlane_visual_strip(
            &mut positions,
            &mut colors,
            &mut indices,
            [0.0, 0.0, 0.0],
            [10.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            HyperlaneDepthBucket::Near,
            2.0,
            0.6,
        );
        let lower_edge = Vec3::from_array(positions[0]);
        let upper_edge = Vec3::from_array(positions[6]);
        assert!((lower_edge.distance(upper_edge) - 2.0).abs() < f32::EPSILON);
    }
}
