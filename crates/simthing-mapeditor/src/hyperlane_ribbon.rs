//! Camera-facing hyperlane ribbon basis and mesh validity helpers (presentation only).

use bevy::prelude::Vec3;

const WIDTH_DIR_EPSILON_SQ: f32 = 1e-8;

/// Ribbon basis mode (mirrors `StudioViewMode::hyperlane_render_path` without app coupling).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HyperlaneRibbonBasis {
    #[default]
    CameraFacing3D,
    OverheadLegibility,
}

/// Camera state used when building hyperlane ribbon strips.
#[derive(Debug, Clone, Copy)]
pub struct HyperlaneRibbonCamera {
    pub position: [f32; 3],
    pub right: [f32; 3],
    pub up: [f32; 3],
    pub forward: [f32; 3],
    pub basis: HyperlaneRibbonBasis,
}

impl Default for HyperlaneRibbonCamera {
    fn default() -> Self {
        Self {
            position: [40.0, 35.0, 40.0],
            right: [1.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            forward: [0.0, 0.0, -1.0],
            basis: HyperlaneRibbonBasis::CameraFacing3D,
        }
    }
}

impl HyperlaneRibbonCamera {
    pub fn from_transform(
        position: Vec3,
        rotation: bevy::prelude::Quat,
        basis: HyperlaneRibbonBasis,
    ) -> Self {
        Self {
            position: position.to_array(),
            right: (rotation * Vec3::X).to_array(),
            up: (rotation * Vec3::Y).to_array(),
            forward: (rotation * Vec3::NEG_Z).to_array(),
            basis,
        }
    }
}

/// Build statistics for one hyperlane bucket mesh rebuild.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct HyperlaneMeshStats {
    pub source_segment_count: usize,
    pub bucket_segment_count: usize,
    pub falloff_culled_segment_count: usize,
    pub vertex_count: usize,
    pub index_count: usize,
    pub degenerate_width_dir_count: u32,
    pub nan_inf_vertex_count: u32,
    pub zero_length_segment_count: u32,
}

/// Outcome of width-direction computation for telemetry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HyperlaneWidthDirOutcome {
    CameraFacingCross,
    ProjectedCameraRight,
    ProjectedCameraUp,
    WorldAxisFallback,
}

/// Compute a finite, non-zero ribbon width direction perpendicular to the lane segment.
pub fn hyperlane_ribbon_width_dir(
    from: [f32; 3],
    to: [f32; 3],
    camera: HyperlaneRibbonCamera,
) -> (Vec3, HyperlaneWidthDirOutcome) {
    let from = Vec3::from_array(from);
    let to = Vec3::from_array(to);
    let delta = to - from;
    if delta.length_squared() <= WIDTH_DIR_EPSILON_SQ || !delta.is_finite() {
        return (Vec3::X, HyperlaneWidthDirOutcome::WorldAxisFallback);
    }
    let lane_dir = normalized_finite(delta).unwrap_or(Vec3::X);
    match camera.basis {
        HyperlaneRibbonBasis::CameraFacing3D | HyperlaneRibbonBasis::OverheadLegibility => {
            let midpoint = (from + to) * 0.5;
            let view_dir =
                normalized_finite(Vec3::from_array(camera.position) - midpoint).unwrap_or(Vec3::Y);
            compute_camera_facing_width_dir(
                lane_dir,
                view_dir,
                Vec3::from_array(camera.right),
                Vec3::from_array(camera.up),
            )
        }
    }
}

pub fn compute_camera_facing_width_dir(
    lane_dir: Vec3,
    view_dir: Vec3,
    camera_right: Vec3,
    camera_up: Vec3,
) -> (Vec3, HyperlaneWidthDirOutcome) {
    let lane_dir = normalized_finite(lane_dir).unwrap_or(Vec3::Z);
    let view_dir = normalized_finite(view_dir).unwrap_or(Vec3::Y);

    if let Some(width) = normalized_finite(lane_dir.cross(view_dir)) {
        return (width, HyperlaneWidthDirOutcome::CameraFacingCross);
    }
    if let Some(width) = project_perpendicular_to_lane(lane_dir, camera_right) {
        return (width, HyperlaneWidthDirOutcome::ProjectedCameraRight);
    }
    if let Some(width) = project_perpendicular_to_lane(lane_dir, camera_up) {
        return (width, HyperlaneWidthDirOutcome::ProjectedCameraUp);
    }
    (
        deterministic_world_axis_fallback(lane_dir),
        HyperlaneWidthDirOutcome::WorldAxisFallback,
    )
}

fn project_perpendicular_to_lane(axis: Vec3, candidate: Vec3) -> Option<Vec3> {
    if !candidate.is_finite() {
        return None;
    }
    let projected = candidate - axis * candidate.dot(axis);
    normalized_finite(projected)
}

fn deterministic_world_axis_fallback(lane_dir: Vec3) -> Vec3 {
    let mut best_axis = Vec3::Y;
    let mut best_align = lane_dir.dot(Vec3::Y).abs();
    for axis in [Vec3::X, Vec3::Z] {
        let align = lane_dir.dot(axis).abs();
        if align < best_align {
            best_axis = axis;
            best_align = align;
        }
    }
    if let Some(width) = project_perpendicular_to_lane(lane_dir, best_axis) {
        return width;
    }
    for axis in [Vec3::X, Vec3::Y, Vec3::Z] {
        if let Some(width) = project_perpendicular_to_lane(lane_dir, axis) {
            return width;
        }
    }
    Vec3::X
}

fn normalized_finite(value: Vec3) -> Option<Vec3> {
    if !value.is_finite() {
        return None;
    }
    if value.length_squared() > WIDTH_DIR_EPSILON_SQ {
        Some(value.normalize())
    } else {
        None
    }
}

pub fn is_valid_width_dir(value: Vec3) -> bool {
    value.is_finite() && value.length_squared() > WIDTH_DIR_EPSILON_SQ
}

pub fn count_non_finite_vertex_positions(positions: &[[f32; 3]]) -> u32 {
    positions
        .iter()
        .filter(|p| !p[0].is_finite() || !p[1].is_finite() || !p[2].is_finite())
        .count() as u32
}

/// Reject meshes that would wipe visible hyperlanes due to a transient camera singularity.
pub fn hyperlane_mesh_is_valid(stats: HyperlaneMeshStats, base_opacity_percent: f32) -> bool {
    if stats.nan_inf_vertex_count > 0 {
        return false;
    }
    if stats.bucket_segment_count == 0 {
        return true;
    }
    if base_opacity_percent <= 0.0 {
        return true;
    }
    stats.vertex_count > 0 && stats.index_count > 0
}

/// Aggregate validity when hyperlanes exist and should be visible.
pub fn hyperlane_rebuild_is_valid(
    bucket_stats: &[HyperlaneMeshStats],
    total_source_segments: usize,
    base_opacity_percent: f32,
) -> bool {
    if total_source_segments == 0 || base_opacity_percent <= 0.0 {
        return true;
    }
    let total_bucket_segments: usize = bucket_stats.iter().map(|s| s.bucket_segment_count).sum();
    let total_vertices: usize = bucket_stats.iter().map(|s| s.vertex_count).sum();
    if bucket_stats.iter().any(|s| s.nan_inf_vertex_count > 0) {
        return false;
    }
    if total_bucket_segments == 0 && total_source_segments > 0 {
        return false;
    }
    if total_vertices == 0 && total_bucket_segments > 0 {
        return false;
    }
    bucket_stats
        .iter()
        .all(|stats| hyperlane_mesh_is_valid(*stats, base_opacity_percent))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_camera() -> HyperlaneRibbonCamera {
        HyperlaneRibbonCamera {
            position: [0.0, 0.0, 10.0],
            right: [1.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            forward: [0.0, 0.0, -1.0],
            basis: HyperlaneRibbonBasis::CameraFacing3D,
        }
    }

    #[test]
    fn hyperlane_width_dir_finite_when_lane_parallel_camera_forward() {
        let camera = HyperlaneRibbonCamera {
            forward: [0.0, 0.0, -1.0],
            ..sample_camera()
        };
        let (width, _) = hyperlane_ribbon_width_dir([0.0, 0.0, 0.0], [0.0, 0.0, 20.0], camera);
        assert!(is_valid_width_dir(width));
        assert!(width.dot(Vec3::Z).abs() < 1e-4);
    }

    #[test]
    fn hyperlane_width_dir_finite_when_lane_parallel_camera_right() {
        let camera = HyperlaneRibbonCamera {
            position: [10.0, 0.0, 0.0],
            right: [0.0, 0.0, 1.0],
            up: [0.0, 1.0, 0.0],
            forward: [-1.0, 0.0, 0.0],
            basis: HyperlaneRibbonBasis::CameraFacing3D,
        };
        let (width, _) = hyperlane_ribbon_width_dir([0.0, 0.0, 0.0], [20.0, 0.0, 0.0], camera);
        assert!(is_valid_width_dir(width));
    }

    #[test]
    fn hyperlane_width_dir_finite_when_lane_parallel_camera_up() {
        let camera = HyperlaneRibbonCamera {
            position: [0.0, 10.0, 0.0],
            right: [1.0, 0.0, 0.0],
            up: [0.0, 0.0, 1.0],
            forward: [0.0, -1.0, 0.0],
            basis: HyperlaneRibbonBasis::CameraFacing3D,
        };
        let (width, _) = hyperlane_ribbon_width_dir([0.0, 0.0, 0.0], [0.0, 20.0, 0.0], camera);
        assert!(is_valid_width_dir(width));
    }

    #[test]
    fn hyperlane_width_dir_fallback_is_nonzero_and_finite() {
        let (width, outcome) = compute_camera_facing_width_dir(Vec3::X, Vec3::X, Vec3::X, Vec3::Y);
        assert!(is_valid_width_dir(width));
        assert_eq!(outcome, HyperlaneWidthDirOutcome::ProjectedCameraUp);
        assert!(width.dot(Vec3::X).abs() < 1e-4);
    }

    #[test]
    fn valid_hyperlane_mesh_with_geometry_passes() {
        let stats = HyperlaneMeshStats {
            bucket_segment_count: 4,
            vertex_count: 32,
            index_count: 48,
            ..Default::default()
        };
        assert!(hyperlane_mesh_is_valid(stats, 75.0));
    }
}
