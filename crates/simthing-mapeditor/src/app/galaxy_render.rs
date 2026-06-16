#![cfg(windows)]

use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;

use crate::session::StudioSession;

use super::GalaxySceneRoot;

#[derive(Component)]
pub struct GalaxyStar;

#[derive(Component)]
pub struct GalaxyHyperlanes;

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
    let mut positions = Vec::with_capacity(vm.hyperlanes.len() * 2);
    for lane in &vm.hyperlanes {
        positions.push([lane.from[0], lane.from[1], lane.from[2]]);
        positions.push([lane.to[0], lane.to[1], lane.to[2]]);
    }
    let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    let mesh_handle = meshes.add(mesh);
    let material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.45, 0.72, 1.0, 0.55),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    root.hyperlanes = Some(
        commands
            .spawn((
                Mesh3d(mesh_handle),
                MeshMaterial3d(material),
                GalaxyHyperlanes,
            ))
            .id(),
    );
}

fn despawn_galaxy(commands: &mut Commands, root: &mut GalaxySceneRoot) {
    for entity in root.stars.drain(..) {
        commands.entity(entity).despawn();
    }
    if let Some(entity) = root.hyperlanes.take() {
        commands.entity(entity).despawn();
    }
    if let Some(entity) = root.core_glow.take() {
        commands.entity(entity).despawn();
    }
}

pub fn sync_hyperlane_colors_system(
    session: Res<super::StudioAppState>,
    camera: Query<&GlobalTransform, With<super::camera::MainCamera>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    hyperlanes: Query<&MeshMaterial3d<StandardMaterial>, With<GalaxyHyperlanes>>,
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
    let Ok(mat_handle) = hyperlanes.single() else {
        return;
    };
    let Some(material) = materials.get_mut(&mat_handle.0) else {
        return;
    };
    let mid_dist = session
        .view_model
        .hyperlanes
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
        / session.view_model.hyperlanes.len() as f32;
    let t = ((mid_dist - meta.hyperlane_depth_fade_start)
        / (meta.hyperlane_depth_fade_end - meta.hyperlane_depth_fade_start))
        .clamp(0.0, 1.0);
    let alpha = meta.hyperlane_alpha_near * (1.0 - t) + meta.hyperlane_alpha_far * t;
    material.base_color =
        Color::srgba(0.35 + (1.0 - t) * 0.25, 0.55 + (1.0 - t) * 0.25, 1.0, alpha);
}
