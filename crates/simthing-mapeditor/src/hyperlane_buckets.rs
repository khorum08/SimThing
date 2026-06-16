//! Hyperlane depth-bucket classification and alpha ordering (render-only).

use crate::view_model::StudioGalaxyRenderMeta;

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
}
