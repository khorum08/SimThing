//! Hyperlane depth-bucket classification and alpha ordering (render-only).

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HyperlaneDepthBucket {
    Near,
    Mid,
    Far,
}

impl HyperlaneDepthBucket {
    pub const ALL: [Self; 3] = [Self::Near, Self::Mid, Self::Far];
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

pub fn bucket_base_rgba(bucket: HyperlaneDepthBucket) -> (f32, f32, f32, f32) {
    match bucket {
        HyperlaneDepthBucket::Near => (0.42, 0.62, 0.92, 0.32),
        HyperlaneDepthBucket::Mid => (0.30, 0.46, 0.68, 0.16),
        HyperlaneDepthBucket::Far => (0.20, 0.24, 0.36, 0.06),
    }
}

pub fn bucket_alpha_for_camera_distance(
    bucket: HyperlaneDepthBucket,
    camera_distance: f32,
    fade_start: f32,
    fade_end: f32,
) -> f32 {
    let (_, _, _, base_alpha) = bucket_base_rgba(bucket);
    let t = ((camera_distance - fade_start) / (fade_end - fade_start)).clamp(0.0, 1.0);
    base_alpha * (1.0 - t * 0.85)
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
        let near = bucket_base_rgba(HyperlaneDepthBucket::Near).3;
        let mid = bucket_base_rgba(HyperlaneDepthBucket::Mid).3;
        let far = bucket_base_rgba(HyperlaneDepthBucket::Far).3;
        assert!(near > mid);
        assert!(mid > far);
    }
}
