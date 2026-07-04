//! STUDIO-ANTIALIASING-TEST-PATTERN-0 — 3D geometry-edge AA diagnostic overlay.

use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;

pub const AA_TEST_PATTERN_MATERIAL_LABEL: &str = "StandardMaterial/unlit/opaque/aa-test-pattern";
pub const AA_TEST_PATTERN_STRIP_COUNT: usize = 12;
pub const AA_TEST_PATTERN_FAN_COUNT: usize = 1;
pub const AA_TEST_PATTERN_GEOMETRY_INSTANCES: usize =
    AA_TEST_PATTERN_STRIP_COUNT + AA_TEST_PATTERN_FAN_COUNT;

const STRIP_ANGLES_DEG: [f32; 6] = [5.0, 10.0, 15.0, 22.5, 30.0, 45.0];
const STRIP_LENGTH: f32 = 14.0;
const STRIP_WIDTHS: [f32; 2] = [0.18, 0.08];

#[derive(Component)]
pub struct AaTestPatternRoot;

#[derive(Component)]
pub struct AaTestPatternStrip;

#[derive(Component)]
pub struct AaTestPatternFan;

#[derive(Resource, Default, Clone, Debug, PartialEq, Eq)]
pub struct AaTestPatternRuntime {
    pub root: Option<Entity>,
    pub visible: bool,
    pub geometry_instances: usize,
}

impl AaTestPatternRuntime {
    pub fn material_label(&self) -> &'static str {
        AA_TEST_PATTERN_MATERIAL_LABEL
    }
}

pub fn thin_strip_mesh(length: f32, width: f32) -> Mesh {
    let half_len = length * 0.5;
    let half_w = width * 0.5;
    let positions = vec![
        [-half_len, -half_w, 0.0],
        [half_len, -half_w, 0.0],
        [half_len, half_w, 0.0],
        [-half_len, half_w, 0.0],
    ];
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32(vec![0, 1, 2, 0, 2, 3]));
    mesh
}

pub fn triangle_fan_mesh(radius: f32, segments: u8) -> Mesh {
    let mut positions = vec![[0.0, 0.0, 0.0]];
    for i in 0..=segments {
        let t = i as f32 / segments as f32;
        let angle = t * std::f32::consts::FRAC_PI_2;
        positions.push([angle.cos() * radius, angle.sin() * radius, 0.0]);
    }
    let mut indices: Vec<u32> = Vec::new();
    for i in 1..segments {
        indices.extend_from_slice(&[0, i as u32, (i + 1) as u32]);
    }
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

pub fn pattern_root_transform(camera: &GlobalTransform) -> Transform {
    let forward = camera.forward().as_vec3();
    let right = camera.right().as_vec3();
    let up = camera.up().as_vec3();
    let distance = 10.0;
    let position = camera.translation() + forward * distance + right * -4.0 + up * -2.8;
    let rotation = Quat::from_mat3(&Mat3::from_cols(right, up, -forward));
    Transform::from_translation(position).with_rotation(rotation)
}

pub fn spawn_aa_test_pattern_root(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Entity {
    let strip_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.95, 0.15, 0.15),
        unlit: true,
        alpha_mode: AlphaMode::Opaque,
        cull_mode: None,
        ..default()
    });
    let thin_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.95, 0.95),
        unlit: true,
        alpha_mode: AlphaMode::Opaque,
        cull_mode: None,
        ..default()
    });
    let fan_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.95, 0.95, 0.15),
        unlit: true,
        alpha_mode: AlphaMode::Opaque,
        cull_mode: None,
        ..default()
    });

    commands
        .spawn((AaTestPatternRoot, Transform::default(), Visibility::Visible))
        .with_children(|parent| {
            let mut row = 0usize;
            for &width in &STRIP_WIDTHS {
                for &angle_deg in &STRIP_ANGLES_DEG {
                    let mesh = meshes.add(thin_strip_mesh(STRIP_LENGTH, width));
                    let material = if row % 2 == 0 {
                        strip_material.clone()
                    } else {
                        thin_material.clone()
                    };
                    let y = row as f32 * 0.32 - 1.8;
                    parent.spawn((
                        Mesh3d(mesh),
                        MeshMaterial3d(material),
                        AaTestPatternStrip,
                        Transform::from_translation(Vec3::new(0.0, y, 0.0))
                            .with_rotation(Quat::from_rotation_z(angle_deg.to_radians())),
                    ));
                    row += 1;
                }
            }

            let fan_mesh = meshes.add(triangle_fan_mesh(2.2, 8));
            parent.spawn((
                Mesh3d(fan_mesh),
                MeshMaterial3d(fan_material),
                AaTestPatternFan,
                Transform::from_translation(Vec3::new(-5.5, -2.2, 0.0)),
            ));
        })
        .id()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::render::mesh::VertexAttributeValues;

}
