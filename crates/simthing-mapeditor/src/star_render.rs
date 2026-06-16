//! Render-only star and hyperlane visual tuning helpers.

use crate::hyperlane_buckets::{bucket_base_rgba, HyperlaneDepthBucket};
use crate::view_model::StudioGalaxyRenderMeta;

pub const DEFAULT_STAR_VISIBILITY_SCALE: f32 = 3.5;
pub const DEFAULT_LANE_VISIBILITY_SCALE: f32 = 0.45;
pub const MIN_STAR_WORLD_SCALE: f32 = 0.75;
pub const STAR_BASE_RADIUS: f32 = 0.55;

pub fn star_visual_defaults() -> StudioGalaxyRenderMeta {
    StudioGalaxyRenderMeta {
        vertical_thickness_scale: 1.0,
        core_bulge_strength: 0.85,
        core_bulge_radius: 0.22,
        star_sprite_scale: 1.0,
        star_visibility_scale: DEFAULT_STAR_VISIBILITY_SCALE,
        lane_visibility_scale: DEFAULT_LANE_VISIBILITY_SCALE,
        min_star_world_scale: MIN_STAR_WORLD_SCALE,
        hyperlane_alpha_near: 0.72,
        hyperlane_alpha_far: 0.08,
        hyperlane_depth_fade_start: 20.0,
        hyperlane_depth_fade_end: 120.0,
    }
}

pub fn star_world_scale(meta: &StudioGalaxyRenderMeta, radius_unit: f32) -> f32 {
    let scaled = meta.star_sprite_scale
        * meta.star_visibility_scale
        * STAR_BASE_RADIUS
        * (0.65 + radius_unit * 0.35);
    scaled.max(meta.min_star_world_scale)
}

pub fn star_emissive_strength(base: f32, selected: bool, hovered: bool) -> f32 {
    let multiplier = if selected {
        2.4
    } else if hovered {
        1.7
    } else {
        1.35
    };
    base * multiplier
}

pub fn hyperlane_bucket_alpha(bucket: HyperlaneDepthBucket, meta: &StudioGalaxyRenderMeta) -> f32 {
    bucket_base_rgba(bucket).3 * meta.lane_visibility_scale
}

pub fn hyperlane_default_opacity_is_less_than_star_emphasis() {
    // compile-time helper anchor for tests
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star_render_meta_default_size_is_visible_at_overview() {
        let meta = star_visual_defaults();
        let scale = star_world_scale(&meta, 0.5);
        assert!(scale >= MIN_STAR_WORLD_SCALE);
        assert!(scale >= 1.0);
    }

    #[test]
    fn star_render_meta_has_minimum_visual_size() {
        let meta = star_visual_defaults();
        let small = star_world_scale(&meta, 0.0);
        assert!(small >= MIN_STAR_WORLD_SCALE);
        let mut tiny = star_visual_defaults();
        tiny.star_visibility_scale = 0.01;
        assert!((star_world_scale(&tiny, 0.0) - MIN_STAR_WORLD_SCALE).abs() < f32::EPSILON);
    }

    #[test]
    fn hyperlane_default_opacity_is_less_than_star_emphasis() {
        let meta = star_visual_defaults();
        let near_lane = hyperlane_bucket_alpha(HyperlaneDepthBucket::Near, &meta);
        let star_emissive = star_emissive_strength(0.6, false, false);
        assert!(near_lane < 0.35);
        assert!(star_emissive > near_lane);
    }

    #[test]
    fn selected_star_highlight_is_brighter_than_unselected_star() {
        let base = 0.7;
        assert!(
            star_emissive_strength(base, true, false) > star_emissive_strength(base, false, false)
        );
    }

    #[test]
    fn starburst_render_meta_is_render_only() {
        assert!(crate::starburst::STARBURST_RENDER_ONLY_NOTE.contains("presentation-only"));
    }
}
