//! Hyperlane depth-bucket classification and alpha ordering (render-only).

use bevy::prelude::Vec2;

use crate::view_model::StudioGalaxyRenderMeta;

pub const HYPERLANE_EDGE_FALLOFF_FRACTION_EACH_SIDE: f32 = 0.10;
pub const HYPERLANE_CORE_FRACTION: f32 = 0.80;
pub const MIN_HYPERLANE_THICKNESS_WORLD: f32 = 0.025;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HyperlaneDepthBucket {
    Near,
    Mid,
    Far,
}

impl HyperlaneDepthBucket {
    pub const ALL: [Self; 3] = [Self::Near, Self::Mid, Self::Far];
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HyperlaneCameraDepthThresholds {
    pub near_max_distance: f32,
    pub mid_max_distance: f32,
}

impl HyperlaneCameraDepthThresholds {
    pub fn from_meta(meta: &StudioGalaxyRenderMeta) -> Self {
        Self {
            near_max_distance: meta.hyperlane_depth_near_max,
            mid_max_distance: meta.hyperlane_depth_mid_max,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HyperlaneRenderSettings {
    pub base_thickness_percent_of_star: f32,
    pub base_opacity_percent: f32,
    pub falloff_distance_percent: f32,
    pub falloff_thickness_percent: f32,
    pub falloff_opacity_percent: f32,
}

impl Default for HyperlaneRenderSettings {
    fn default() -> Self {
        Self {
            base_thickness_percent_of_star: 8.0,
            base_opacity_percent: 75.0,
            falloff_distance_percent: 100.0,
            falloff_thickness_percent: 24.0,
            falloff_opacity_percent: 16.0,
        }
    }
}

impl HyperlaneRenderSettings {
    pub fn clamped(self) -> Self {
        Self {
            base_thickness_percent_of_star: self.base_thickness_percent_of_star.clamp(1.0, 25.0),
            base_opacity_percent: self.base_opacity_percent.clamp(0.0, 100.0),
            falloff_distance_percent: self.falloff_distance_percent.clamp(1.0, 100.0),
            falloff_thickness_percent: self.falloff_thickness_percent.clamp(0.0, 100.0),
            falloff_opacity_percent: self.falloff_opacity_percent.clamp(0.0, 100.0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HyperlaneVisual {
    pub thickness_world: f32,
    pub core_opacity: f32,
    pub edge_falloff_fraction_each_side: f32,
    pub visible: bool,
}

pub fn compute_hyperlane_visual(
    range_progress_percent: f32,
    nearest_star_disc_width_world: f32,
    settings: &HyperlaneRenderSettings,
    use_plateau: bool,
) -> HyperlaneVisual {
    let settings = settings.clamped();
    if settings.base_opacity_percent <= 0.0 {
        return HyperlaneVisual {
            thickness_world: 0.0,
            core_opacity: 0.0,
            edge_falloff_fraction_each_side: HYPERLANE_EDGE_FALLOFF_FRACTION_EACH_SIDE,
            visible: false,
        };
    }

    let star_width = nearest_star_disc_width_world.max(f32::EPSILON);
    let max_base = star_width * 0.25;
    let minimum = MIN_HYPERLANE_THICKNESS_WORLD
        .min(max_base)
        .max(f32::EPSILON);
    let base_thickness =
        (star_width * settings.base_thickness_percent_of_star / 100.0).clamp(minimum, max_base);
    let target_thickness = base_thickness * settings.falloff_thickness_percent / 100.0;
    let base_opacity = settings.base_opacity_percent / 100.0;
    let target_opacity = base_opacity * settings.falloff_opacity_percent / 100.0;
    let falloff_at = settings.falloff_distance_percent;
    let (thickness, opacity) = if use_plateau {
        let t =
            crate::falloff_metric::plateau_falloff_t_percent(range_progress_percent, falloff_at);
        (
            crate::falloff_metric::lerp_f32(base_thickness, target_thickness, t),
            crate::falloff_metric::lerp_f32(base_opacity, target_opacity, t),
        )
    } else {
        let depth = range_progress_percent.clamp(0.0, 100.0);
        let t = (depth / falloff_at.max(f32::EPSILON)).clamp(0.0, 1.0);
        (
            crate::falloff_metric::lerp_f32(base_thickness, target_thickness, t),
            crate::falloff_metric::lerp_f32(base_opacity, target_opacity, t),
        )
    };

    HyperlaneVisual {
        thickness_world: thickness.max(minimum).min(max_base),
        core_opacity: opacity.clamp(0.0, 1.0),
        edge_falloff_fraction_each_side: HYPERLANE_EDGE_FALLOFF_FRACTION_EACH_SIDE,
        visible: opacity > 0.0,
    }
}

pub fn closest_point_on_segment_2d(point: Vec2, from: Vec2, to: Vec2) -> Vec2 {
    let segment = to - from;
    let len_sq = segment.length_squared();
    if len_sq <= f32::EPSILON || !len_sq.is_finite() {
        return from;
    }
    let t = ((point - from).dot(segment) / len_sq).clamp(0.0, 1.0);
    from + segment * t
}

pub fn hyperlane_midpoint_map_radius_progress_percent(
    context: &crate::falloff_metric::StudioMapRadiusFalloffContext,
    from: [f32; 3],
    to: [f32; 3],
) -> f32 {
    let mid = Vec2::new((from[0] + to[0]) * 0.5, (from[2] + to[2]) * 0.5);
    crate::falloff_metric::map_radius_progress_percent(context, mid)
}

pub fn hyperlane_map_radius_progress_percent(
    context: &crate::falloff_metric::StudioMapRadiusFalloffContext,
    from: [f32; 3],
    to: [f32; 3],
) -> f32 {
    let from2 = Vec2::new(from[0], from[2]);
    let to2 = Vec2::new(to[0], to[2]);
    let sample = closest_point_on_segment_2d(context.view_origin, from2, to2);
    crate::falloff_metric::map_radius_progress_percent(context, sample)
}

pub fn hyperlane_camera_depth_percent(
    camera_position: [f32; 3],
    from: [f32; 3],
    to: [f32; 3],
    meta: &StudioGalaxyRenderMeta,
) -> f32 {
    let distance = camera_distance_to_hyperlane_midpoint(camera_position, from, to);
    let near = meta.star_near_distance.max(0.0);
    let far = meta.star_far_distance.max(near + f32::EPSILON);
    ((distance - near) / (far - near)).clamp(0.0, 1.0) * 100.0
}

pub fn apply_hyperlane_render_settings_to_meta(
    meta: &mut StudioGalaxyRenderMeta,
    settings: HyperlaneRenderSettings,
) {
    let settings = settings.clamped();
    meta.hyperlane_render_settings = settings;
    meta.lane_visibility_scale = settings.base_opacity_percent / 100.0;
}

pub fn hyperlane_visuals_dirty_after_settings_change(
    previous: HyperlaneRenderSettings,
    next: HyperlaneRenderSettings,
) -> bool {
    previous.clamped() != next.clamped()
}

pub fn classify_hyperlane_depth_bucket(normalized_midpoint_dist: f32) -> HyperlaneDepthBucket {
    let t = normalized_midpoint_dist.clamp(0.0, 1.0);
    if t < 1.0 / 3.0 {
        HyperlaneDepthBucket::Near
    } else if t < 2.0 / 3.0 {
        HyperlaneDepthBucket::Mid
    } else {
        HyperlaneDepthBucket::Far
    }
}

pub fn hyperlane_segment_midpoint(from: [f32; 3], to: [f32; 3]) -> [f32; 3] {
    [
        (from[0] + to[0]) * 0.5,
        (from[1] + to[1]) * 0.5,
        (from[2] + to[2]) * 0.5,
    ]
}

pub fn camera_distance_to_hyperlane_midpoint(
    camera_position: [f32; 3],
    from: [f32; 3],
    to: [f32; 3],
) -> f32 {
    let mid = hyperlane_segment_midpoint(from, to);
    let dx = camera_position[0] - mid[0];
    let dy = camera_position[1] - mid[1];
    let dz = camera_position[2] - mid[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}

pub fn classify_hyperlane_camera_depth_bucket(
    camera_position: [f32; 3],
    from: [f32; 3],
    to: [f32; 3],
    thresholds: HyperlaneCameraDepthThresholds,
) -> HyperlaneDepthBucket {
    let distance = camera_distance_to_hyperlane_midpoint(camera_position, from, to);
    if distance <= thresholds.near_max_distance {
        HyperlaneDepthBucket::Near
    } else if distance <= thresholds.mid_max_distance {
        HyperlaneDepthBucket::Mid
    } else {
        HyperlaneDepthBucket::Far
    }
}

pub fn bucket_base_rgba(bucket: HyperlaneDepthBucket) -> (f32, f32, f32, f32) {
    match bucket {
        HyperlaneDepthBucket::Near => (0.56, 0.78, 1.0, 0.60),
        HyperlaneDepthBucket::Mid => (0.32, 0.50, 0.74, 0.30),
        HyperlaneDepthBucket::Far => (0.18, 0.22, 0.30, 0.13),
    }
}

pub fn bucket_alpha_for_meta(bucket: HyperlaneDepthBucket, meta: &StudioGalaxyRenderMeta) -> f32 {
    let base = match bucket {
        HyperlaneDepthBucket::Near => meta.lane_near_alpha,
        HyperlaneDepthBucket::Mid => meta.lane_mid_alpha,
        HyperlaneDepthBucket::Far => meta.lane_far_alpha,
    };
    (base * meta.lane_visibility_scale).max(meta.lane_far_min_alpha)
}

pub fn selected_incident_lane_alpha() -> f32 {
    0.95
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hyperlane_depth_bucket_near_mid_far() {
        assert_eq!(
            classify_hyperlane_depth_bucket(0.0),
            HyperlaneDepthBucket::Near
        );
        assert_eq!(
            classify_hyperlane_depth_bucket(0.2),
            HyperlaneDepthBucket::Near
        );
        assert_eq!(
            classify_hyperlane_depth_bucket(0.4),
            HyperlaneDepthBucket::Mid
        );
        assert_eq!(
            classify_hyperlane_depth_bucket(0.8),
            HyperlaneDepthBucket::Far
        );
    }

    #[test]
    fn hyperlane_bucket_alpha_ordering_near_greater_than_far() {
        let meta = StudioGalaxyRenderMeta::default();
        let near = bucket_alpha_for_meta(HyperlaneDepthBucket::Near, &meta);
        let mid = bucket_alpha_for_meta(HyperlaneDepthBucket::Mid, &meta);
        let far = bucket_alpha_for_meta(HyperlaneDepthBucket::Far, &meta);
        assert!(near > mid);
        assert!(mid > far);
    }

    #[test]
    fn hyperlane_camera_depth_classifies_near_mid_far() {
        let thresholds = HyperlaneCameraDepthThresholds {
            near_max_distance: 20.0,
            mid_max_distance: 60.0,
        };
        let camera = [0.0, 0.0, 0.0];
        assert_eq!(
            classify_hyperlane_camera_depth_bucket(
                camera,
                [8.0, 0.0, 0.0],
                [12.0, 0.0, 0.0],
                thresholds
            ),
            HyperlaneDepthBucket::Near
        );
        assert_eq!(
            classify_hyperlane_camera_depth_bucket(
                camera,
                [38.0, 0.0, 0.0],
                [42.0, 0.0, 0.0],
                thresholds
            ),
            HyperlaneDepthBucket::Mid
        );
        assert_eq!(
            classify_hyperlane_camera_depth_bucket(
                camera,
                [78.0, 0.0, 0.0],
                [82.0, 0.0, 0.0],
                thresholds
            ),
            HyperlaneDepthBucket::Far
        );
    }

    #[test]
    fn hyperlane_camera_depth_alpha_ordering_near_greater_than_mid_greater_than_far() {
        let meta = StudioGalaxyRenderMeta::default();
        assert!(
            bucket_alpha_for_meta(HyperlaneDepthBucket::Near, &meta)
                > bucket_alpha_for_meta(HyperlaneDepthBucket::Mid, &meta)
        );
        assert!(
            bucket_alpha_for_meta(HyperlaneDepthBucket::Mid, &meta)
                > bucket_alpha_for_meta(HyperlaneDepthBucket::Far, &meta)
        );
    }

    #[test]
    fn camera_relative_lane_fade_still_present() {
        let meta = StudioGalaxyRenderMeta::default();
        let camera = [0.0, 0.0, 0.0];
        let thresholds = HyperlaneCameraDepthThresholds::from_meta(&meta);
        let near = classify_hyperlane_camera_depth_bucket(
            camera,
            [10.0, 0.0, 0.0],
            [20.0, 0.0, 0.0],
            thresholds,
        );
        let far = classify_hyperlane_camera_depth_bucket(
            camera,
            [220.0, 0.0, 0.0],
            [240.0, 0.0, 0.0],
            thresholds,
        );
        assert_eq!(near, HyperlaneDepthBucket::Near);
        assert_eq!(far, HyperlaneDepthBucket::Far);
        assert!(bucket_alpha_for_meta(near, &meta) > bucket_alpha_for_meta(far, &meta));
    }

    #[test]
    fn far_hyperlane_alpha_has_legible_minimum() {
        let mut meta = StudioGalaxyRenderMeta::default();
        meta.lane_visibility_scale = 0.01;
        assert_eq!(
            bucket_alpha_for_meta(HyperlaneDepthBucket::Far, &meta),
            meta.lane_far_min_alpha
        );
    }

    #[test]
    fn selected_incident_lane_overrides_depth_fade() {
        let meta = StudioGalaxyRenderMeta::default();
        assert!(
            selected_incident_lane_alpha()
                > bucket_alpha_for_meta(HyperlaneDepthBucket::Near, &meta)
        );
    }

    #[test]
    fn hyperlane_settings_defaults_exist() {
        let settings = HyperlaneRenderSettings::default().clamped();
        assert!(settings.base_thickness_percent_of_star > 0.0);
        assert!(settings.base_thickness_percent_of_star <= 25.0);
        assert!(settings.base_opacity_percent > 0.0);
    }

    #[test]
    fn base_hyperlane_opacity_zero_hides_lanes() {
        let settings = HyperlaneRenderSettings {
            base_opacity_percent: 0.0,
            ..Default::default()
        };
        let visual = compute_hyperlane_visual(0.0, 10.0, &settings, true);
        assert!(!visual.visible);
        assert_eq!(visual.core_opacity, 0.0);
    }

    #[test]
    fn hyperlane_opacity_zero_hides_all_ribbons() {
        let settings = HyperlaneRenderSettings {
            base_opacity_percent: 0.0,
            ..Default::default()
        };
        for depth in [0.0, 50.0, 100.0] {
            let visual = compute_hyperlane_visual(depth, 10.0, &settings, false);
            assert!(!visual.visible);
            assert_eq!(visual.thickness_world, 0.0);
            assert_eq!(visual.core_opacity, 0.0);
        }
    }

    #[test]
    fn base_hyperlane_opacity_nonzero_keeps_lanes_visible() {
        let settings = HyperlaneRenderSettings {
            base_opacity_percent: 1.0,
            ..Default::default()
        };
        let visual = compute_hyperlane_visual(0.0, 10.0, &settings, true);
        assert!(visual.visible);
        assert!(visual.core_opacity > 0.0);
    }

    #[test]
    fn base_hyperlane_thickness_minimum_is_legible_nonzero() {
        let settings = HyperlaneRenderSettings {
            base_thickness_percent_of_star: 1.0,
            ..Default::default()
        };
        let visual = compute_hyperlane_visual(0.0, 10.0, &settings, true);
        assert!(visual.thickness_world >= MIN_HYPERLANE_THICKNESS_WORLD);
    }

    #[test]
    fn base_hyperlane_thickness_max_is_at_most_25_percent_of_star_disc() {
        let settings = HyperlaneRenderSettings {
            base_thickness_percent_of_star: 100.0,
            ..Default::default()
        };
        let visual = compute_hyperlane_visual(0.0, 10.0, &settings, true);
        assert!(visual.thickness_world <= 2.5 + f32::EPSILON);
    }

    #[test]
    fn hyperlane_visual_reaches_falloff_thickness_at_falloff_distance() {
        let settings = HyperlaneRenderSettings {
            base_thickness_percent_of_star: 20.0,
            falloff_distance_percent: 40.0,
            falloff_thickness_percent: 50.0,
            ..Default::default()
        };
        let visual = compute_hyperlane_visual(100.0, 10.0, &settings, true);
        assert!((visual.thickness_world - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn hyperlane_visual_reaches_falloff_opacity_at_falloff_distance() {
        let settings = HyperlaneRenderSettings {
            base_opacity_percent: 80.0,
            falloff_distance_percent: 40.0,
            falloff_opacity_percent: 25.0,
            ..Default::default()
        };
        let visual = compute_hyperlane_visual(100.0, 10.0, &settings, true);
        assert!((visual.core_opacity - 0.20).abs() < f32::EPSILON);
    }

    #[test]
    fn hyperlane_visual_core_fraction_is_80_percent() {
        assert!((HYPERLANE_CORE_FRACTION - 0.80).abs() < f32::EPSILON);
    }

    #[test]
    fn soft_edge_core_fraction_is_80_percent() {
        assert!((HYPERLANE_CORE_FRACTION - 0.80).abs() < f32::EPSILON);
    }

    #[test]
    fn hyperlane_visual_edge_falloff_is_10_percent_each_side() {
        let visual = compute_hyperlane_visual(0.0, 10.0, &HyperlaneRenderSettings::default(), true);
        assert!((visual.edge_falloff_fraction_each_side - 0.10).abs() < f32::EPSILON);
    }

    #[test]
    fn soft_edge_side_falloff_is_10_percent_each_side() {
        let visual = compute_hyperlane_visual(0.0, 10.0, &HyperlaneRenderSettings::default(), true);
        assert!((visual.edge_falloff_fraction_each_side - 0.10).abs() < f32::EPSILON);
    }

    #[test]
    fn closest_point_on_segment_2d_clamps_to_endpoints() {
        let from = Vec2::new(10.0, 0.0);
        let to = Vec2::new(20.0, 0.0);
        let closest = closest_point_on_segment_2d(Vec2::ZERO, from, to);
        assert!((closest - from).length_squared() < f32::EPSILON);
        let closest = closest_point_on_segment_2d(Vec2::new(100.0, 0.0), from, to);
        assert!((closest - to).length_squared() < f32::EPSILON);
    }

    #[test]
    fn hyperlane_closest_point_falloff_keeps_foreground_crossing_lane_visible() {
        use crate::falloff_metric::{MapViewOriginSource, StudioMapRadiusFalloffContext};

        let ctx = StudioMapRadiusFalloffContext {
            view_origin: Vec2::ZERO,
            map_max_view_distance: 100.0,
            origin_source: MapViewOriginSource::GalaxyCenter,
        };
        let from = [0.0, 0.0, 0.0];
        let to = [200.0, 0.0, 0.0];
        let midpoint_progress = hyperlane_midpoint_map_radius_progress_percent(&ctx, from, to);
        let closest_progress = hyperlane_map_radius_progress_percent(&ctx, from, to);
        assert!((midpoint_progress - 100.0).abs() < 0.1);
        assert!((closest_progress - 0.0).abs() < 0.1);

        let settings = HyperlaneRenderSettings {
            falloff_distance_percent: 10.0,
            falloff_opacity_percent: 0.0,
            ..Default::default()
        };
        let midpoint_visual = compute_hyperlane_visual(midpoint_progress, 10.0, &settings, true);
        let closest_visual = compute_hyperlane_visual(closest_progress, 10.0, &settings, true);
        assert!(!midpoint_visual.visible);
        assert!(closest_visual.visible);
    }
}
