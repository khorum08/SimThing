//! Render-only star and hyperlane visual tuning helpers.

use crate::hyperlane_buckets::{bucket_alpha_for_meta, HyperlaneDepthBucket};
use crate::view_model::{StudioGalaxyRenderMeta, StudioStarView};

pub const DEFAULT_STAR_VISIBILITY_SCALE: f32 = 4.5;
pub const DEFAULT_LANE_VISIBILITY_SCALE: f32 = 0.75;
pub const MIN_STAR_WORLD_SCALE: f32 = 1.35;
pub const STAR_BASE_RADIUS: f32 = 0.72;
pub const STAR_BILLBOARD_Y_LIFT: f32 = 0.85;

#[derive(Debug, Clone, PartialEq)]
pub struct StarRenderInstance {
    pub system_id: u32,
    pub position: [f32; 3],
    pub scale: f32,
    pub emissive_strength: f32,
}

pub fn star_visual_defaults() -> StudioGalaxyRenderMeta {
    StudioGalaxyRenderMeta {
        vertical_thickness_scale: 1.0,
        core_bulge_strength: 0.85,
        core_bulge_radius: 0.22,
        star_sprite_scale: 1.0,
        star_visibility_scale: DEFAULT_STAR_VISIBILITY_SCALE,
        lane_visibility_scale: DEFAULT_LANE_VISIBILITY_SCALE,
        min_star_world_scale: MIN_STAR_WORLD_SCALE,
        lane_near_alpha: 0.75,
        lane_mid_alpha: 0.42,
        lane_far_alpha: 0.16,
        lane_far_min_alpha: 0.045,
        hyperlane_depth_near_max: 100.0,
        hyperlane_depth_mid_max: 155.0,
    }
}

pub fn star_world_scale(meta: &StudioGalaxyRenderMeta, radius_unit: f32) -> f32 {
    let scaled = meta.star_sprite_scale
        * meta.star_visibility_scale
        * STAR_BASE_RADIUS
        * (0.65 + radius_unit * 0.35);
    scaled.max(meta.min_star_world_scale)
}

pub fn star_scale_multiplier(selected: bool, hovered: bool) -> f32 {
    if selected {
        2.0
    } else if hovered {
        1.5
    } else {
        1.0
    }
}

pub fn star_emissive_strength(base: f32, selected: bool, hovered: bool) -> f32 {
    let multiplier = if selected {
        3.0
    } else if hovered {
        2.1
    } else {
        1.55
    };
    base * multiplier
}

pub fn hyperlane_bucket_alpha(bucket: HyperlaneDepthBucket, meta: &StudioGalaxyRenderMeta) -> f32 {
    bucket_alpha_for_meta(bucket, meta)
}

pub fn prepare_star_render_instances(stars: &[StudioStarView]) -> Vec<StarRenderInstance> {
    stars
        .iter()
        .map(|star| StarRenderInstance {
            system_id: star.system_id,
            position: [
                star.world_x,
                star.world_y + STAR_BILLBOARD_Y_LIFT,
                star.world_z,
            ],
            scale: star.sprite_scale,
            emissive_strength: star.emissive_strength,
        })
        .collect()
}

pub fn hyperlane_default_opacity_is_less_than_star_emphasis() {
    // compile-time helper anchor for tests
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::{run_generation, GenerationProfile};
    use crate::view_model::StudioGalaxyViewModel;

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
        let selected_star_emissive = star_emissive_strength(0.6, true, false);
        assert!(near_lane < 0.65);
        assert!(star_emissive > near_lane);
        assert!(selected_star_emissive > near_lane * 3.0);
    }

    #[test]
    fn selected_star_highlight_is_brighter_than_unselected_star() {
        let base = 0.7;
        assert!(
            star_emissive_strength(base, true, false) > star_emissive_strength(base, false, false)
        );
        assert!(star_scale_multiplier(true, false) > star_scale_multiplier(false, false));
    }

    #[test]
    fn starburst_render_meta_is_render_only() {
        assert!(crate::starburst::STARBURST_RENDER_ONLY_NOTE.contains("presentation-only"));
    }

    #[test]
    fn star_render_preparation_count_matches_system_count() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let instances = prepare_star_render_instances(&vm.stars);
        assert_eq!(instances.len(), vm.stars.len());
        assert_eq!(instances.len(), output.result.placement.systems.len());
    }
}
