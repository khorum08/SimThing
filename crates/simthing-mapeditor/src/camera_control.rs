//! Pure orbit camera helpers (testable without Bevy).

pub const ORBIT_PITCH_MIN: f32 = 0.15;
pub const ORBIT_PITCH_MAX: f32 = 1.2;
pub const DEFAULT_ORBIT_SENSITIVITY: f32 = 0.003;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrbitCameraState {
    pub orbit_yaw: f32,
    pub orbit_pitch: f32,
    pub orbit_distance: f32,
    pub orbit_target: [f32; 3],
    pub overhead: bool,
}

impl Default for OrbitCameraState {
    fn default() -> Self {
        Self {
            orbit_yaw: 0.6,
            orbit_pitch: 0.55,
            orbit_distance: 95.0,
            orbit_target: [0.0, 0.0, 0.0],
            overhead: false,
        }
    }
}

pub fn reset_camera_after_generation(camera: &mut OrbitCameraState) {
    camera.orbit_yaw = 0.55;
    camera.orbit_pitch = 0.62;
    camera.orbit_distance = 95.0;
    camera.orbit_target = [0.0, 0.0, 0.0];
    camera.overhead = false;
}

pub fn snap_overhead(camera: &mut OrbitCameraState) {
    camera.overhead = true;
    camera.orbit_pitch = std::f32::consts::FRAC_PI_2 - 0.001;
    camera.orbit_yaw = 0.0;
    camera.orbit_distance = 110.0;
}

pub fn apply_orbit_delta(
    camera: &mut OrbitCameraState,
    mouse_delta_x: f32,
    mouse_delta_y: f32,
    sensitivity: f32,
) {
    if mouse_delta_x == 0.0 && mouse_delta_y == 0.0 {
        return;
    }
    camera.orbit_yaw += mouse_delta_x * sensitivity;
    camera.orbit_pitch =
        (camera.orbit_pitch - mouse_delta_y * sensitivity).clamp(ORBIT_PITCH_MIN, ORBIT_PITCH_MAX);
    camera.overhead = false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_action_overhead_updates_camera_state() {
        let mut camera = OrbitCameraState::default();
        snap_overhead(&mut camera);
        assert!(camera.overhead);
        assert_eq!(camera.orbit_yaw, 0.0);
        assert!((camera.orbit_pitch - (std::f32::consts::FRAC_PI_2 - 0.001)).abs() < 1e-5);
    }

    #[test]
    fn camera_action_reset_updates_camera_state() {
        let mut camera = OrbitCameraState {
            orbit_yaw: 2.0,
            orbit_pitch: 1.0,
            orbit_distance: 50.0,
            orbit_target: [5.0, 0.0, 3.0],
            overhead: true,
        };
        reset_camera_after_generation(&mut camera);
        assert!(!camera.overhead);
        assert_eq!(camera.orbit_yaw, 0.55);
        assert_eq!(camera.orbit_pitch, 0.62);
        assert_eq!(camera.orbit_distance, 95.0);
        assert_eq!(camera.orbit_target, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn rmb_without_mouse_delta_does_not_rotate() {
        let mut camera = OrbitCameraState::default();
        let before = camera;
        apply_orbit_delta(&mut camera, 0.0, 0.0, DEFAULT_ORBIT_SENSITIVITY);
        assert_eq!(camera.orbit_yaw, before.orbit_yaw);
        assert_eq!(camera.orbit_pitch, before.orbit_pitch);
    }

    #[test]
    fn mouse_delta_orbit_changes_yaw_and_pitch() {
        let mut camera = OrbitCameraState::default();
        apply_orbit_delta(&mut camera, 10.0, -5.0, DEFAULT_ORBIT_SENSITIVITY);
        assert_ne!(camera.orbit_yaw, 0.6);
        assert_ne!(camera.orbit_pitch, 0.55);
        assert!(!camera.overhead);
    }

    #[test]
    fn orbit_pitch_clamps() {
        let mut camera = OrbitCameraState {
            orbit_pitch: ORBIT_PITCH_MAX,
            ..Default::default()
        };
        apply_orbit_delta(&mut camera, 0.0, 10_000.0, DEFAULT_ORBIT_SENSITIVITY);
        assert!((camera.orbit_pitch - ORBIT_PITCH_MIN).abs() < f32::EPSILON);

        camera.orbit_pitch = ORBIT_PITCH_MIN;
        apply_orbit_delta(&mut camera, 0.0, -10_000.0, DEFAULT_ORBIT_SENSITIVITY);
        assert!((camera.orbit_pitch - ORBIT_PITCH_MAX).abs() < f32::EPSILON);
    }
}
