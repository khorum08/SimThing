//! Map-plane radius plateau falloff metric for Studio presentation (stars, nameplates, hyperlanes).

use bevy::prelude::{Camera, GlobalTransform, Vec2, Vec3};

/// GPU / telemetry falloff mode sentinels (mirrors `simthing_tools::style::TextStyleGlobalsGpu`).
pub const FALLOFF_MODE_CAMERA_DISTANCE: f32 = 0.0;
pub const FALLOFF_MODE_VISUAL_HORIZON: f32 = 1.0;
pub const FALLOFF_MODE_MAP_RADIUS: f32 = 2.0;

/// Bevy `Camera::viewport_to_world` uses physical pixel coordinates with origin at the top-left;
/// y increases downward (y-down).
pub const VIEWPORT_COORDINATE_CONVENTION: &str =
    "y-down physical pixels (Bevy viewport_to_world, origin top-left)";

/// Bottom-center of the viewport in physical pixel coordinates (y-down).
pub const VIEWPORT_BOTTOM_CENTER_X_FRACTION: f32 = 0.5;
pub const VIEWPORT_BOTTOM_CENTER_Y_FRACTION: f32 = 1.0;

const MIN_MAP_MAX_VIEW_DISTANCE: f32 = 1.0;
const MAX_MAP_MAX_VIEW_DISTANCE_DIAGONAL_MULTIPLIER: f32 = 4.0;

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

    pub fn diagonal(&self) -> f32 {
        let w = (self.max_x - self.min_x).max(0.0);
        let h = (self.max_z - self.min_z).max(0.0);
        (w * w + h * h).sqrt().max(1.0)
    }

    pub fn extent_margin(&self) -> f32 {
        (self.max_x - self.min_x)
            .max(self.max_z - self.min_z)
            .max(1.0)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapViewOriginSource {
    BottomCenterViewportRay,
    BottomCenterViewportRayClampedToBounds,
    CameraFocusProjected,
    CameraPositionProjected,
    GalaxyCenter,
    RetainedPreviousContext,
}

/// Per-frame diagnostic snapshot for map-radius falloff origin resolution.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MapRadiusFalloffDiagnostics {
    pub viewport_convention: &'static str,
    pub bottom_center_viewport_px: [f32; 2],
    pub raw_ray_origin: [f32; 3],
    pub raw_ray_direction: [f32; 3],
    pub raw_map_plane_hit: Option<[f32; 2]>,
    pub origin_clamped: bool,
    pub bounds_min: [f32; 2],
    pub bounds_max: [f32; 2],
    pub context_frame: u64,
    pub updated_after_camera: bool,
    pub retained_previous_context: bool,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MapRadiusFalloffComputeOutput {
    pub context: StudioMapRadiusFalloffContext,
    pub diagnostics: MapRadiusFalloffDiagnostics,
    pub valid: bool,
}

/// Bottom-center viewport position in physical pixels (y-down convention).
pub fn viewport_bottom_center_physical_px(viewport_width: f32, viewport_height: f32) -> Vec2 {
    Vec2::new(
        viewport_width * VIEWPORT_BOTTOM_CENTER_X_FRACTION,
        viewport_height * VIEWPORT_BOTTOM_CENTER_Y_FRACTION,
    )
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewportMapPlaneRayCast {
    pub viewport_px: Vec2,
    pub ray_origin: Vec3,
    pub ray_direction: Vec3,
    pub hit_xz: Option<Vec2>,
}

/// Cast a viewport pixel ray onto the map plane (y = map_plane_y).
pub fn cast_viewport_ray_to_map_plane(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    viewport_px: Vec2,
    map_plane_y: f32,
) -> ViewportMapPlaneRayCast {
    let mut cast = ViewportMapPlaneRayCast {
        viewport_px,
        ray_origin: Vec3::ZERO,
        ray_direction: Vec3::NEG_Z,
        hit_xz: None,
    };
    let Ok(ray) = camera.viewport_to_world(camera_transform, viewport_px) else {
        return cast;
    };
    cast.ray_origin = ray.origin;
    let dir = ray.direction.normalize();
    cast.ray_direction = dir;
    if dir.y.abs() <= f32::EPSILON {
        return cast;
    }
    let t = (map_plane_y - ray.origin.y) / dir.y;
    if t < 0.0 {
        return cast;
    }
    let hit = ray.origin + dir * t;
    cast.hit_xz = Some(Vec2::new(hit.x, hit.z));
    cast
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
    let viewport_px = viewport_bottom_center_physical_px(viewport_width, viewport_height);
    cast_viewport_ray_to_map_plane(camera, camera_transform, viewport_px, map_plane_y).hit_xz
}

fn project_xz(point: Vec3) -> Vec2 {
    Vec2::new(point.x, point.z)
}

fn clamp_xz_to_bounds(point: Vec2, bounds: MapPlaneBounds) -> Vec2 {
    Vec2::new(
        point.x.clamp(bounds.min_x, bounds.max_x),
        point.y.clamp(bounds.min_z, bounds.max_z),
    )
}

fn xz_outside_bounds_by_margin(point: Vec2, bounds: MapPlaneBounds, margin: f32) -> bool {
    point.x < bounds.min_x - margin
        || point.x > bounds.max_x + margin
        || point.y < bounds.min_z - margin
        || point.y > bounds.max_z + margin
}

pub fn map_max_view_distance_from_origin(view_origin: Vec2, bounds: MapPlaneBounds) -> f32 {
    bounds
        .corners()
        .iter()
        .map(|corner| view_origin.distance(*corner))
        .fold(0.0_f32, f32::max)
        .max(MIN_MAP_MAX_VIEW_DISTANCE)
}

pub fn context_is_valid(context: &StudioMapRadiusFalloffContext, bounds: MapPlaneBounds) -> bool {
    if !context.view_origin.x.is_finite() || !context.view_origin.y.is_finite() {
        return false;
    }
    let distance = context.map_max_view_distance;
    if !distance.is_finite() || distance < MIN_MAP_MAX_VIEW_DISTANCE {
        return false;
    }
    let max_reasonable =
        bounds.diagonal() * MAX_MAP_MAX_VIEW_DISTANCE_DIAGONAL_MULTIPLIER + bounds.extent_margin();
    distance <= max_reasonable
}

fn focus_or_camera_origin(
    camera_focus: Option<Vec3>,
    camera_transform: &GlobalTransform,
) -> (Vec2, MapViewOriginSource) {
    if let Some(focus) = camera_focus {
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
    }
}

fn resolve_view_origin_from_ray(
    cast: ViewportMapPlaneRayCast,
    bounds: MapPlaneBounds,
    camera_focus: Option<Vec3>,
    camera_transform: &GlobalTransform,
) -> (Vec2, MapViewOriginSource, bool) {
    let Some(hit) = cast.hit_xz else {
        let (origin, source) = focus_or_camera_origin(camera_focus, camera_transform);
        return (origin, source, false);
    };
    let margin = bounds.extent_margin();
    if xz_outside_bounds_by_margin(hit, bounds, margin) {
        let (origin, source) = focus_or_camera_origin(camera_focus, camera_transform);
        return (origin, source, false);
    }
    let clamped = clamp_xz_to_bounds(hit, bounds);
    if (clamped - hit).length_squared() > f32::EPSILON {
        (
            clamped,
            MapViewOriginSource::BottomCenterViewportRayClampedToBounds,
            true,
        )
    } else {
        (hit, MapViewOriginSource::BottomCenterViewportRay, false)
    }
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
    context_frame: u64,
    updated_after_camera: bool,
) -> MapRadiusFalloffComputeOutput {
    let viewport_px = viewport_bottom_center_physical_px(viewport_width, viewport_height);
    let cast = cast_viewport_ray_to_map_plane(camera, camera_transform, viewport_px, map_plane_y);
    let (view_origin, origin_source, origin_clamped) =
        resolve_view_origin_from_ray(cast, map_bounds, camera_focus, camera_transform);
    let map_max_view_distance = map_max_view_distance_from_origin(view_origin, map_bounds);
    let context = StudioMapRadiusFalloffContext {
        view_origin,
        map_max_view_distance,
        origin_source,
    };
    let diagnostics = MapRadiusFalloffDiagnostics {
        viewport_convention: VIEWPORT_COORDINATE_CONVENTION,
        bottom_center_viewport_px: [viewport_px.x, viewport_px.y],
        raw_ray_origin: cast.ray_origin.to_array(),
        raw_ray_direction: cast.ray_direction.to_array(),
        raw_map_plane_hit: cast.hit_xz.map(|p| [p.x, p.y]),
        origin_clamped,
        bounds_min: [map_bounds.min_x, map_bounds.min_z],
        bounds_max: [map_bounds.max_x, map_bounds.max_z],
        context_frame,
        updated_after_camera,
        retained_previous_context: false,
    };
    let valid = context_is_valid(&context, map_bounds);
    MapRadiusFalloffComputeOutput {
        context,
        diagnostics,
        valid,
    }
}

/// When the freshly computed context is invalid, retain the previous valid context if available.
pub fn stabilize_map_radius_falloff_output(
    computed: MapRadiusFalloffComputeOutput,
    previous: Option<(StudioMapRadiusFalloffContext, MapRadiusFalloffDiagnostics)>,
) -> MapRadiusFalloffComputeOutput {
    if computed.valid {
        return computed;
    }
    if let Some((context, mut diagnostics)) = previous {
        diagnostics.retained_previous_context = true;
        return MapRadiusFalloffComputeOutput {
            context,
            diagnostics,
            valid: true,
        };
    }
    MapRadiusFalloffComputeOutput {
        context: StudioMapRadiusFalloffContext {
            origin_source: MapViewOriginSource::GalaxyCenter,
            ..Default::default()
        },
        diagnostics: MapRadiusFalloffDiagnostics {
            retained_previous_context: false,
            ..computed.diagnostics
        },
        valid: false,
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
        MapViewOriginSource::BottomCenterViewportRayClampedToBounds => {
            "bottom-center viewport ray (clamped to bounds)"
        }
        MapViewOriginSource::CameraFocusProjected => "camera focus projected",
        MapViewOriginSource::CameraPositionProjected => "camera position projected",
        MapViewOriginSource::GalaxyCenter => "galaxy center",
        MapViewOriginSource::RetainedPreviousContext => "retained previous valid context",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::Transform;

    #[test]
    fn viewport_bottom_center_uses_y_down_physical_coordinates() {
        let px = viewport_bottom_center_physical_px(1920.0, 1080.0);
        assert!((px.x - 960.0).abs() < f32::EPSILON);
        assert!((px.y - 1080.0).abs() < f32::EPSILON);
        assert!(
            (px.y - 0.0).abs() > 1.0,
            "y=0 is top-center in y-down convention"
        );
    }

    #[test]
    fn clamp_xz_keeps_point_inside_bounds() {
        let bounds = MapPlaneBounds {
            min_x: -10.0,
            max_x: 10.0,
            min_z: -5.0,
            max_z: 5.0,
        };
        let clamped = clamp_xz_to_bounds(Vec2::new(20.0, 0.0), bounds);
        assert!((clamped.x - 10.0).abs() < f32::EPSILON);
        assert!((clamped.y - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn far_outside_hit_uses_focus_fallback_in_resolve() {
        let bounds = MapPlaneBounds {
            min_x: -10.0,
            max_x: 10.0,
            min_z: -10.0,
            max_z: 10.0,
        };
        let cast = ViewportMapPlaneRayCast {
            viewport_px: Vec2::new(100.0, 200.0),
            ray_origin: Vec3::ZERO,
            ray_direction: Vec3::NEG_Y,
            hit_xz: Some(Vec2::new(500.0, 500.0)),
        };
        let focus = Vec3::new(1.0, 0.0, 2.0);
        let transform =
            GlobalTransform::from(Transform::from_translation(Vec3::new(0.0, 50.0, 0.0)));
        let (origin, source, clamped) =
            resolve_view_origin_from_ray(cast, bounds, Some(focus), &transform);
        assert_eq!(source, MapViewOriginSource::CameraFocusProjected);
        assert!((origin.x - 1.0).abs() < f32::EPSILON);
        assert!((origin.y - 2.0).abs() < f32::EPSILON);
        assert!(!clamped);
    }

    #[test]
    fn slightly_outside_hit_clamps_to_bounds() {
        let bounds = MapPlaneBounds {
            min_x: -10.0,
            max_x: 10.0,
            min_z: -10.0,
            max_z: 10.0,
        };
        let cast = ViewportMapPlaneRayCast {
            viewport_px: Vec2::new(100.0, 200.0),
            ray_origin: Vec3::ZERO,
            ray_direction: Vec3::NEG_Y,
            hit_xz: Some(Vec2::new(12.0, -3.0)),
        };
        let transform = GlobalTransform::IDENTITY;
        let (origin, source, clamped) =
            resolve_view_origin_from_ray(cast, bounds, None, &transform);
        assert_eq!(
            source,
            MapViewOriginSource::BottomCenterViewportRayClampedToBounds
        );
        assert!((origin.x - 10.0).abs() < f32::EPSILON);
        assert!((origin.y - -3.0).abs() < f32::EPSILON);
        assert!(clamped);
    }

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
            map_max_view_distance: map_max_view_distance_from_origin(
                Vec2::new(-40.0, -30.0),
                bounds,
            ),
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
