//! Map-plane radius plateau falloff metric for Studio presentation (stars, nameplates, hyperlanes).

use bevy::prelude::{Camera, GlobalTransform, Vec2, Vec3};

/// GPU / telemetry falloff mode sentinels (mirrors `simthing_tools::style::TextStyleGlobalsGpu`).
pub const FALLOFF_MODE_CAMERA_DISTANCE: f32 = 0.0;
pub const FALLOFF_MODE_VISUAL_HORIZON: f32 = 1.0;
pub const FALLOFF_MODE_MAP_RADIUS: f32 = 2.0;

/// Map-plane AABB corners used for max view distance.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MapPlaneBounds {
    pub min_x: f32,
    pub max_x: f32,
    pub min_z: f32,
    pub max_z: f32,
}

impl MapPlaneBounds {
    pub fn corners(&self) -> [Vec2; 4] {
        [
            Vec2::new(self.min_x, self.min_z),
            Vec2::new(self.max_x, self.min_z),
            Vec2::new(self.min_x, self.max_z),
            Vec2::new(self.max_x, self.max_z),
        ]
    }

    pub fn from_world_positions(positions: &[[f32; 3]]) -> Self {
        if positions.is_empty() {
            return Self {
                min_x: -50.0,
                max_x: 50.0,
                min_z: -50.0,
                max_z: 50.0,
            };
        }
        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_z = f32::INFINITY;
        let mut max_z = f32::NEG_INFINITY;
        for pos in positions {
            min_x = min_x.min(pos[0]);
            max_x = max_x.max(pos[0]);
            min_z = min_z.min(pos[2]);
            max_z = max_z.max(pos[2]);
        }
        Self {
            min_x,
            max_x,
            min_z,
            max_z,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MapViewOriginSource {
    BottomCenterViewportRay,
    CameraFocusProjected,
    CameraPositionProjected,
    GalaxyCenter,
}

/// Per-frame map-radius falloff ruler (presentation only).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StudioMapRadiusFalloffContext {
    pub view_origin: Vec2,
    pub map_max_view_distance: f32,
    pub origin_source: MapViewOriginSource,
}

impl Default for StudioMapRadiusFalloffContext {
    fn default() -> Self {
        Self {
            view_origin: Vec2::ZERO,
            map_max_view_distance: 1.0,
            origin_source: MapViewOriginSource::GalaxyCenter,
        }
    }
}

impl StudioMapRadiusFalloffContext {
    pub fn map_max_view_distance(&self) -> f32 {
        self.map_max_view_distance.max(f32::EPSILON)
    }
}

/// Intersect the bottom-center viewport ray with the map plane (y = map_plane_y).
pub fn viewport_bottom_center_map_origin(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    viewport_width: f32,
    viewport_height: f32,
    map_plane_y: f32,
) -> Option<Vec2> {
    if viewport_width <= 0.0 || viewport_height <= 0.0 {
        return None;
    }
    let viewport_pos = Vec2::new(viewport_width * 0.5, 0.0);
    let ray = camera
        .viewport_to_world(camera_transform, viewport_pos)
        .ok()?;
    let dir = ray.direction.normalize();
    if dir.y.abs() <= f32::EPSILON {
        return None;
    }
    let t = (map_plane_y - ray.origin.y) / dir.y;
    if t < 0.0 {
        return None;
    }
    let hit = ray.origin + dir * t;
    Some(Vec2::new(hit.x, hit.z))
}

fn project_xz(point: Vec3) -> Vec2 {
    Vec2::new(point.x, point.z)
}

/// Build the map-radius falloff context for the current camera and map bounds.
pub fn compute_map_radius_falloff_context(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    viewport_width: f32,
    viewport_height: f32,
    map_bounds: MapPlaneBounds,
    camera_focus: Option<Vec3>,
    map_plane_y: f32,
) -> StudioMapRadiusFalloffContext {
    let (view_origin, origin_source) = if let Some(origin) = viewport_bottom_center_map_origin(
        camera,
        camera_transform,
        viewport_width,
        viewport_height,
        map_plane_y,
    ) {
        (origin, MapViewOriginSource::BottomCenterViewportRay)
    } else if let Some(focus) = camera_focus {
        (project_xz(focus), MapViewOriginSource::CameraFocusProjected)
    } else {
        let cam = camera_transform.translation();
        if cam.is_finite() {
            (
                project_xz(cam),
                MapViewOriginSource::CameraPositionProjected,
            )
        } else {
            (Vec2::ZERO, MapViewOriginSource::GalaxyCenter)
        }
    };

    let map_max_view_distance = map_bounds
        .corners()
        .iter()
        .map(|corner| view_origin.distance(*corner))
        .fold(0.0_f32, f32::max)
        .max(f32::EPSILON);

    StudioMapRadiusFalloffContext {
        view_origin,
        map_max_view_distance,
        origin_source,
    }
}

/// Normalized map-radius progress in 0.0–1.0.
pub fn map_radius_progress(context: &StudioMapRadiusFalloffContext, point: Vec2) -> f32 {
    let distance = context.view_origin.distance(point);
    (distance / context.map_max_view_distance()).clamp(0.0, 1.0)
}

/// Map-radius progress as 0–100 percent (matches slider / legacy depth_percent scale).
pub fn map_radius_progress_percent(context: &StudioMapRadiusFalloffContext, point: Vec2) -> f32 {
    map_radius_progress(context, point) * 100.0
}

pub fn world_position_map_progress_percent(
    context: &StudioMapRadiusFalloffContext,
    world_position: [f32; 3],
) -> f32 {
    map_radius_progress_percent(context, Vec2::new(world_position[0], world_position[2]))
}

/// Plateau falloff factor: 0 inside plateau, linear 0→1 across remaining range.
pub fn plateau_falloff_t(progress: f32, plateau_end: f32) -> f32 {
    if plateau_end >= 1.0 {
        0.0
    } else if progress <= plateau_end {
        0.0
    } else {
        ((progress - plateau_end) / (1.0 - plateau_end)).clamp(0.0, 1.0)
    }
}

pub fn plateau_falloff_t_percent(progress_percent: f32, plateau_end_percent: f32) -> f32 {
    plateau_falloff_t(
        (progress_percent / 100.0).clamp(0.0, 1.0),
        (plateau_end_percent / 100.0).clamp(0.0, 1.0),
    )
}

pub fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

/// Interpolate from base to target across the post-plateau range (linear).
pub fn plateau_interpolate(
    base: f32,
    target: f32,
    progress_percent: f32,
    plateau_end_percent: f32,
) -> f32 {
    let t = plateau_falloff_t_percent(progress_percent, plateau_end_percent);
    lerp_f32(base, target, t)
}

pub fn origin_source_label(source: MapViewOriginSource) -> &'static str {
    match source {
        MapViewOriginSource::BottomCenterViewportRay => "bottom-center viewport ray",
        MapViewOriginSource::CameraFocusProjected => "camera focus projected",
        MapViewOriginSource::CameraPositionProjected => "camera position projected",
        MapViewOriginSource::GalaxyCenter => "galaxy center",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_radius_progress_at_origin_is_zero() {
        let ctx = StudioMapRadiusFalloffContext {
            view_origin: Vec2::ZERO,
            map_max_view_distance: 100.0,
            origin_source: MapViewOriginSource::GalaxyCenter,
        };
        assert!((map_radius_progress(&ctx, Vec2::ZERO) - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn map_radius_progress_at_farthest_corner_is_one() {
        let bounds = MapPlaneBounds {
            min_x: -40.0,
            max_x: 40.0,
            min_z: -30.0,
            max_z: 30.0,
        };
        let ctx = StudioMapRadiusFalloffContext {
            view_origin: Vec2::new(-40.0, -30.0),
            map_max_view_distance: bounds
                .corners()
                .iter()
                .map(|c| Vec2::new(-40.0, -30.0).distance(*c))
                .fold(0.0_f32, f32::max),
            origin_source: MapViewOriginSource::GalaxyCenter,
        };
        let progress = map_radius_progress(&ctx, Vec2::new(40.0, 30.0));
        assert!((progress - 1.0).abs() < 0.001);
    }

    #[test]
    fn plateau_falloff_t_is_zero_inside_plateau() {
        assert!((plateau_falloff_t(0.25, 0.5) - 0.0).abs() < f32::EPSILON);
        assert!((plateau_falloff_t(0.5, 0.5) - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn plateau_falloff_t_reaches_one_at_edge() {
        assert!((plateau_falloff_t(1.0, 0.5) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn plateau_falloff_hundred_percent_plateau_never_fades() {
        assert!((plateau_falloff_t_percent(100.0, 100.0) - 0.0).abs() < f32::EPSILON);
        assert!((plateau_interpolate(1.0, 0.2, 100.0, 100.0) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn map_bounds_from_positions_matches_corners() {
        let bounds = MapPlaneBounds::from_world_positions(&[[-10.0, 0.0, -5.0], [20.0, 1.0, 15.0]]);
        assert!((bounds.min_x + 10.0).abs() < f32::EPSILON);
        assert!((bounds.max_x - 20.0).abs() < f32::EPSILON);
        assert!((bounds.min_z + 5.0).abs() < f32::EPSILON);
        assert!((bounds.max_z - 15.0).abs() < f32::EPSILON);
    }
}
