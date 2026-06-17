#![cfg(windows)]

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;

use crate::hyperlane_buckets::{
    bucket_base_rgba, classify_hyperlane_camera_depth_bucket, compute_hyperlane_visual,
    hyperlane_camera_depth_percent, selected_incident_lane_alpha, HyperlaneCameraDepthThresholds,
    HyperlaneDepthBucket, HYPERLANE_CORE_FRACTION,
};
use crate::selection::incident_hyperlanes_for_system;
use crate::session::StudioSession;
use crate::star_render::{
    nearest_camera_star_disc_width_world, prepare_star_billboard_instances, StarBillboardInstance,
};
use crate::starburst::{
    generate_star_aura_image, generate_star_circle_image, generate_starburst_image,
};
use crate::view_model::{build_hyperlane_render_segments, HyperlaneRenderSegment};

use super::GalaxySceneRoot;

#[derive(Component)]
pub struct GalaxyStar {
    pub instance: StarBillboardInstance,
    pub layer: StarVisualLayer,
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
                ))
                .id();
            root.stars.push((star.system_id, entity));
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
        [0.0, 0.0, 0.0],
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

pub fn sync_hyperlane_colors_system(
    session: Res<super::StudioAppState>,
    camera: Query<&GlobalTransform, With<super::camera::MainCamera>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    hyperlanes: Query<(
        &Mesh3d,
        &MeshMaterial3d<StandardMaterial>,
        &GalaxyHyperlanes,
    )>,
) {
    let Some(session) = session.session.as_ref() else {
        return;
    };
    if session.view_model.hyperlanes.is_empty() {
        return;
    }
    let Ok(cam) = camera.single() else {
        return;
    };
    let cam_pos = cam.translation();
    let meta = &session.view_model.render_meta;
    let segments = build_hyperlane_render_segments(
        &session.view_model.hyperlanes,
        &session.view_model.render_anchors,
    );

    for (mesh_handle, mat_handle, marker) in &hyperlanes {
        if let Some(mesh) = meshes.get_mut(&mesh_handle.0) {
            *mesh = build_hyperlane_bucket_mesh(&segments, marker.0, cam_pos.to_array(), meta);
        }
        if let Some(material) = materials.get_mut(&mat_handle.0) {
            material.base_color = Color::WHITE;
        }
    }
}

fn build_hyperlane_bucket_mesh(
    segments: &[HyperlaneRenderSegment],
    bucket: HyperlaneDepthBucket,
    camera_position: [f32; 3],
    meta: &crate::view_model::StudioGalaxyRenderMeta,
) -> Mesh {
    let mut positions = Vec::new();
    let mut colors = Vec::new();
    let mut indices = Vec::new();
    let thresholds = HyperlaneCameraDepthThresholds::from_meta(meta);
    let nearest_star_width = nearest_camera_star_disc_width_world(meta);
    for lane in segments {
        let lane_bucket =
            classify_hyperlane_camera_depth_bucket(camera_position, lane.from, lane.to, thresholds);
        if lane_bucket != bucket {
            continue;
        }
        let depth_percent =
            hyperlane_camera_depth_percent(camera_position, lane.from, lane.to, meta);
        let visual = compute_hyperlane_visual(
            depth_percent,
            nearest_star_width,
            &meta.hyperlane_render_settings,
        );
        if !visual.visible {
            continue;
        }
        push_hyperlane_visual_strip(
            &mut positions,
            &mut colors,
            &mut indices,
            lane.from,
            lane.to,
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

fn push_hyperlane_visual_strip(
    positions: &mut Vec<[f32; 3]>,
    colors: &mut Vec<[f32; 4]>,
    indices: &mut Vec<u32>,
    from: [f32; 3],
    to: [f32; 3],
    bucket: HyperlaneDepthBucket,
    thickness_world: f32,
    core_opacity: f32,
) {
    let from = Vec3::from_array(from);
    let to = Vec3::from_array(to);
    let delta = to - from;
    let flat = Vec3::new(delta.x, 0.0, delta.z);
    let perp = if flat.length_squared() > f32::EPSILON {
        Vec3::new(-flat.z, 0.0, flat.x).normalize()
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
        Query<&mut Visibility, With<GalaxyStar>>,
        Query<&mut Visibility, With<GalaxyHyperlanes>>,
        Query<&mut Visibility, With<SelectedHyperlaneHighlight>>,
    )>,
) {
    for mut visibility in &mut visibility_queries.p0() {
        *visibility = if state.show_stars {
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
