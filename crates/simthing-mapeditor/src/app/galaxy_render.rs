#![cfg(windows)]

use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;

use crate::hyperlane_buckets::{
    bucket_alpha_for_camera_distance, bucket_base_rgba, HyperlaneDepthBucket,
};
use crate::session::StudioSession;

use super::GalaxySceneRoot;

#[derive(Component)]
pub struct GalaxyStar;

#[derive(Component)]
pub struct GalaxyHyperlanes(pub HyperlaneDepthBucket);

pub fn rebuild_galaxy_scene(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    root: &mut GalaxySceneRoot,
    session: &StudioSession,
) {
    despawn_galaxy(commands, root);
    let vm = &session.view_model;
    let sphere = meshes.add(Sphere::new(1.0));
    for star in &vm.stars {
        let material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.75, 0.88, 1.0),
            emissive: LinearRgba::new(
                star.emissive_strength * 0.9,
                star.emissive_strength * 0.95,
                star.emissive_strength,
                1.0,
            ),
            unlit: true,
            ..default()
        });
        let entity = commands
            .spawn((
                Mesh3d(sphere.clone()),
                MeshMaterial3d(material),
                Transform::from_xyz(star.world_x, star.world_y, star.world_z)
                    .with_scale(Vec3::splat(star.sprite_scale)),
                GalaxyStar,
            ))
            .id();
        root.stars.push(entity);
    }

    if vm.hyperlanes.is_empty() {
        return;
    }

    for bucket in HyperlaneDepthBucket::ALL {
        let mut positions = Vec::new();
        for lane in vm
            .hyperlanes
            .iter()
            .filter(|lane| lane.depth_bucket == bucket)
        {
            positions.push([lane.from[0], lane.from[1], lane.from[2]]);
            positions.push([lane.to[0], lane.to[1], lane.to[2]]);
        }
        if positions.is_empty() {
            continue;
        }
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
}

fn bucket_index(bucket: HyperlaneDepthBucket) -> usize {
    match bucket {
        HyperlaneDepthBucket::Near => 0,
        HyperlaneDepthBucket::Mid => 1,
        HyperlaneDepthBucket::Far => 2,
    }
}

fn despawn_galaxy(commands: &mut Commands, root: &mut GalaxySceneRoot) {
    for entity in root.stars.drain(..) {
        commands.entity(entity).despawn();
    }
    for slot in root.hyperlane_buckets.iter_mut() {
        if let Some(entity) = slot.take() {
            commands.entity(entity).despawn();
        }
    }
    if let Some(entity) = root.core_glow.take() {
        commands.entity(entity).despawn();
    }
}

pub fn sync_hyperlane_colors_system(
    session: Res<super::StudioAppState>,
    camera: Query<&GlobalTransform, With<super::camera::MainCamera>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    hyperlanes: Query<(&MeshMaterial3d<StandardMaterial>, &GalaxyHyperlanes)>,
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

    for bucket in HyperlaneDepthBucket::ALL {
        let lanes: Vec<_> = session
            .view_model
            .hyperlanes
            .iter()
            .filter(|lane| lane.depth_bucket == bucket)
            .collect();
        if lanes.is_empty() {
            continue;
        }
        let avg_dist = lanes
            .iter()
            .map(|lane| {
                let mid = Vec3::new(
                    (lane.from[0] + lane.to[0]) * 0.5,
                    (lane.from[1] + lane.to[1]) * 0.5,
                    (lane.from[2] + lane.to[2]) * 0.5,
                );
                cam_pos.distance(mid)
            })
            .sum::<f32>()
            / lanes.len() as f32;
        let alpha = bucket_alpha_for_camera_distance(
            bucket,
            avg_dist,
            meta.hyperlane_depth_fade_start,
            meta.hyperlane_depth_fade_end,
        );
        let (r, g, b, _) = bucket_base_rgba(bucket);
        for (mat_handle, marker) in &hyperlanes {
            if marker.0 != bucket {
                continue;
            }
            if let Some(material) = materials.get_mut(&mat_handle.0) {
                material.base_color = Color::srgba(r, g, b, alpha);
            }
        }
    }
}
