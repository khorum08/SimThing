#![cfg(windows)]

use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;

use crate::hyperlane_buckets::{
    bucket_alpha_for_meta, bucket_base_rgba, classify_hyperlane_camera_depth_bucket,
    selected_incident_lane_alpha, HyperlaneCameraDepthThresholds, HyperlaneDepthBucket,
};
use crate::selection::incident_hyperlanes_for_system;
use crate::session::StudioSession;
use crate::star_render::prepare_star_render_instances;
use crate::starburst::{generate_star_aura_image, generate_starburst_image};

use super::GalaxySceneRoot;

#[derive(Component)]
pub struct GalaxyStar {
    pub system_id: u32,
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
    pub quad: Handle<Mesh>,
}

pub fn init_star_visual_assets(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let core_texture = images.add(generate_starburst_image(64));
    let aura_texture = images.add(generate_star_aura_image(64));
    let quad = meshes.add(Rectangle::new(1.0, 1.0));
    commands.insert_resource(StarVisualAssets {
        core_texture,
        aura_texture,
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
    for star in prepare_star_render_instances(&vm.stars) {
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
                    star.emissive_strength * 1.25 * emissive_factor,
                    star.emissive_strength * 1.32 * emissive_factor,
                    star.emissive_strength * 1.45 * emissive_factor,
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
                    Transform::from_xyz(star.position[0], star.position[1], star.position[2])
                        .with_scale(Vec3::splat(star.scale)),
                    GalaxyStar {
                        system_id: star.system_id,
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
    let positions: Vec<[f32; 3]> = vm
        .hyperlanes
        .iter()
        .filter(|lane| lane.depth_bucket == bucket)
        .flat_map(|lane| {
            [
                [lane.from[0], lane.from[1], lane.from[2]],
                [lane.to[0], lane.to[1], lane.to[2]],
            ]
        })
        .collect();
    let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    let mesh_handle = meshes.add(mesh);
    let (r, g, b, a) = bucket_base_rgba(bucket);
    let material = materials.add(StandardMaterial {
        base_color: Color::srgba(r, g, b, a),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
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
    for (from_id, to_id) in &incident {
        let Some(lane) = session
            .view_model
            .hyperlanes
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
    let thresholds = HyperlaneCameraDepthThresholds::from_meta(meta);
    let mut near_positions = Vec::new();
    let mut mid_positions = Vec::new();
    let mut far_positions = Vec::new();

    for lane in &session.view_model.hyperlanes {
        let bucket = classify_hyperlane_camera_depth_bucket(
            cam_pos.to_array(),
            lane.from,
            lane.to,
            thresholds,
        );
        let positions = match bucket {
            HyperlaneDepthBucket::Near => &mut near_positions,
            HyperlaneDepthBucket::Mid => &mut mid_positions,
            HyperlaneDepthBucket::Far => &mut far_positions,
        };
        positions.push(lane.from);
        positions.push(lane.to);
    }

    for (mesh_handle, mat_handle, marker) in &hyperlanes {
        let positions = match marker.0 {
            HyperlaneDepthBucket::Near => &near_positions,
            HyperlaneDepthBucket::Mid => &mid_positions,
            HyperlaneDepthBucket::Far => &far_positions,
        };
        if let Some(mesh) = meshes.get_mut(&mesh_handle.0) {
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());
        }
        let (r, g, b, _) = bucket_base_rgba(marker.0);
        if let Some(material) = materials.get_mut(&mat_handle.0) {
            material.base_color = Color::srgba(r, g, b, bucket_alpha_for_meta(marker.0, meta));
        }
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
        *visibility = if state.show_hyperlanes {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    for mut visibility in &mut visibility_queries.p2() {
        *visibility = if state.show_hyperlanes {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}
