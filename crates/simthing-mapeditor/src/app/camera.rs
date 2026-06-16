#![cfg(windows)]

use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

pub struct StudioCameraPlugin;

impl Plugin for StudioCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StudioCamera::default())
            .add_systems(Startup, spawn_camera);
    }
}

#[derive(Resource)]
pub struct StudioCamera {
    pub orbit_yaw: f32,
    pub orbit_pitch: f32,
    pub orbit_distance: f32,
    pub orbit_target: Vec3,
    pub overhead: bool,
    pub move_speed: f32,
    pub rmb_held: bool,
}

impl Default for StudioCamera {
    fn default() -> Self {
        Self {
            orbit_yaw: 0.6,
            orbit_pitch: 0.55,
            orbit_distance: 95.0,
            orbit_target: Vec3::ZERO,
            overhead: false,
            move_speed: 40.0,
            rmb_held: false,
        }
    }
}

#[derive(Component)]
pub(crate) struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(40.0, 35.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
        MainCamera,
    ));
}

pub fn reset_camera_after_generation(camera: &mut StudioCamera) {
    camera.orbit_yaw = 0.55;
    camera.orbit_pitch = 0.62;
    camera.orbit_distance = 95.0;
    camera.orbit_target = Vec3::ZERO;
    camera.overhead = false;
}

pub fn snap_overhead(camera: &mut StudioCamera) {
    camera.overhead = true;
    camera.orbit_pitch = std::f32::consts::FRAC_PI_2 - 0.001;
    camera.orbit_yaw = 0.0;
    camera.orbit_distance = 110.0;
}

pub fn camera_hotkeys_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera: ResMut<StudioCamera>,
) {
    if keyboard.just_pressed(KeyCode::KeyO) {
        snap_overhead(&mut camera);
    }
    if keyboard.just_pressed(KeyCode::KeyR) {
        reset_camera_after_generation(&mut camera);
    }
}

pub fn camera_control_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut scroll: EventReader<MouseWheel>,
    mut camera: ResMut<StudioCamera>,
    mut transforms: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
) {
    camera.rmb_held = mouse.pressed(MouseButton::Right);
    let dt = time.delta_secs();
    let forward = Vec3::new(-camera.orbit_yaw.sin(), 0.0, -camera.orbit_yaw.cos());
    let right = Vec3::new(forward.z, 0.0, -forward.x);
    let mut delta = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        delta += forward;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        delta -= forward;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        delta -= right;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        delta += right;
    }
    if delta.length_squared() > 0.0 {
        let speed = camera.move_speed;
        camera.orbit_target += delta.normalize() * speed * dt;
    }

    if camera.rmb_held {
        // simple orbit drag
        camera.orbit_yaw += 0.005;
    }

    for ev in scroll.read() {
        camera.orbit_distance = (camera.orbit_distance - ev.y * 4.0).clamp(25.0, 220.0);
    }

    if !camera.overhead && keyboard.pressed(KeyCode::ShiftLeft) {
        camera.orbit_pitch = (camera.orbit_pitch + 0.01).clamp(0.15, 1.2);
    }

    let pitch = if camera.overhead {
        std::f32::consts::FRAC_PI_2 - 0.001
    } else {
        camera.orbit_pitch
    };
    let yaw = camera.orbit_yaw;
    let dist = camera.orbit_distance;
    let target = camera.orbit_target;
    let offset = Vec3::new(
        dist * pitch.cos() * yaw.sin(),
        dist * pitch.sin(),
        dist * pitch.cos() * yaw.cos(),
    );
    for mut transform in &mut transforms {
        transform.translation = target + offset;
        transform.look_at(target, Vec3::Y);
    }
}
