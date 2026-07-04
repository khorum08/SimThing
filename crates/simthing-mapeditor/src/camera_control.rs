//! Pure orbit camera helpers (testable without Bevy).

pub const ORBIT_PITCH_MIN: f32 = 0.15;
pub const ORBIT_PITCH_MAX: f32 = 1.2;
pub const DEFAULT_ORBIT_SENSITIVITY: f32 = 0.003;

/// Minimum orbit distance after STUDIO-PERFORMANCE-TELEMETRY-WINDOW-0 (200% closer than prior 25.0).
pub const CAMERA_MIN_ORBIT_DISTANCE: f32 = 12.5;
pub const CAMERA_MAX_ORBIT_DISTANCE: f32 = 220.0;
pub const CAMERA_SCROLL_ZOOM_STEP: f32 = 4.0;
/// Prior minimum bound before zoom-in change (regression reference only).
pub const CAMERA_PREVIOUS_MIN_ORBIT_DISTANCE: f32 = 25.0;

pub fn apply_scroll_zoom(current_distance: f32, scroll_delta_y: f32) -> f32 {
    (current_distance - scroll_delta_y * CAMERA_SCROLL_ZOOM_STEP)
        .clamp(CAMERA_MIN_ORBIT_DISTANCE, CAMERA_MAX_ORBIT_DISTANCE)
}

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

}
