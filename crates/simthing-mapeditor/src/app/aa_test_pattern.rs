#![cfg(windows)]

use bevy::prelude::*;

use crate::studio_aa_test_pattern::{
    pattern_root_transform, spawn_aa_test_pattern_root, AaTestPatternRuntime,
    AA_TEST_PATTERN_GEOMETRY_INSTANCES,
};

use super::camera::MainCamera;
use super::StudioAppState;

pub fn sync_aa_test_pattern_system(
    app_state: Res<StudioAppState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut runtime: ResMut<AaTestPatternRuntime>,
    camera: Query<&GlobalTransform, With<MainCamera>>,
) {
    let want_visible = app_state.show_aa_test_pattern;
    runtime.visible = want_visible;

    if !want_visible {
        if let Some(root) = runtime.root.take() {
            commands.entity(root).despawn();
        }
        runtime.geometry_instances = 0;
        return;
    }

    if runtime.root.is_none() {
        runtime.root = Some(spawn_aa_test_pattern_root(
            &mut commands,
            &mut meshes,
            &mut materials,
        ));
        runtime.geometry_instances = AA_TEST_PATTERN_GEOMETRY_INSTANCES;
    }

    if let (Some(root), Ok(camera_transform)) = (runtime.root, camera.single()) {
        commands
            .entity(root)
            .insert(pattern_root_transform(camera_transform));
    }
}
