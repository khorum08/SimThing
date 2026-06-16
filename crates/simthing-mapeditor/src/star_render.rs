//! Render-only star and hyperlane visual tuning helpers.

use crate::hyperlane_buckets::{bucket_alpha_for_meta, HyperlaneDepthBucket};
use crate::view_model::{
    anchor_for_system, StudioGalaxyRenderMeta, StudioStarView, StudioSystemRenderAnchor,
};

pub const DEFAULT_STAR_VISIBILITY_SCALE: f32 = 4.5;
pub const DEFAULT_LANE_VISIBILITY_SCALE: f32 = 0.75;
pub const MIN_STAR_WORLD_SCALE: f32 = 1.35;
pub const STAR_BASE_RADIUS: f32 = 0.72;
pub const PR2R4_STAR_FAR_AURA_SCALE_BASELINE: f32 = 0.16;
pub const PR2R4_STAR_NEAR_AURA_SCALE_BASELINE: f32 = 1.10;
pub const PR2R4_STAR_FAR_CORE_ALPHA_BASELINE: f32 = 0.72;
pub const STAR_AURA_EXTENT_REDUCTION_FACTOR: f32 = 0.50;
pub const DISTANT_STAR_LUMINOSITY_FALLOFF_FACTOR: f32 = 0.75;
pub const PR2R5_STAR_FAR_AURA_SCALE: f32 =
    PR2R4_STAR_FAR_AURA_SCALE_BASELINE * STAR_AURA_EXTENT_REDUCTION_FACTOR;
pub const PR2R5_STAR_NEAR_AURA_SCALE: f32 =
    PR2R4_STAR_NEAR_AURA_SCALE_BASELINE * STAR_AURA_EXTENT_REDUCTION_FACTOR;
pub const PR2R5_STAR_FAR_CORE_ALPHA: f32 =
    PR2R4_STAR_FAR_CORE_ALPHA_BASELINE * DISTANT_STAR_LUMINOSITY_FALLOFF_FACTOR;
pub const PR2R6_AURA_CAP_REDUCTION_FACTOR: f32 = 0.50;
pub const MID_TO_HORIZON_FALLOFF_START_DEPTH: f32 = 0.50;
pub const MID_TO_HORIZON_FALLOFF_FACTOR: f32 = 0.75;
pub const PR2R6_STAR_NEAR_AURA_SCALE: f32 =
    PR2R5_STAR_NEAR_AURA_SCALE * PR2R6_AURA_CAP_REDUCTION_FACTOR;
pub const STAR_DISTANCE_VISUAL_RENDER_ONLY_NOTE: &str =
    "star distance attenuation, core/aura scale, alpha, and bloom are editor render metadata only";

#[derive(Debug, Clone, PartialEq)]
pub struct StarRenderInstance {
    pub system_id: u32,
    pub position: [f32; 3],
    pub scale: f32,
    pub emissive_strength: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StarDistanceVisual {
    pub core_scale: f32,
    pub aura_scale: f32,
    pub core_alpha: f32,
    pub aura_alpha: f32,
}

pub fn star_visual_defaults() -> StudioGalaxyRenderMeta {
    StudioGalaxyRenderMeta::default()
}

pub fn star_world_scale(meta: &StudioGalaxyRenderMeta, radius_unit: f32) -> f32 {
    let scaled = meta.star_sprite_scale
        * meta.star_visibility_scale
        * STAR_BASE_RADIUS
        * (0.65 + radius_unit * 0.35);
    scaled.max(meta.min_star_world_scale)
}

pub fn star_distance_visual(
    camera_distance: f32,
    selected: bool,
    hovered: bool,
    meta: &StudioGalaxyRenderMeta,
) -> StarDistanceVisual {
    let t = normalized_star_camera_depth(camera_distance, meta);
    let eased_far = t * t * (3.0 - 2.0 * t);
    let close = 1.0 - eased_far;
    let far_half_taper = mid_to_horizon_extra_falloff(t);
    let scale_mul = if selected {
        meta.selected_star_scale_multiplier
    } else if hovered {
        meta.hovered_star_scale_multiplier
    } else {
        1.0
    };
    let alpha_boost = if selected {
        1.35
    } else if hovered {
        1.12
    } else {
        1.0
    };
    StarDistanceVisual {
        core_scale: lerp(meta.star_far_core_scale, meta.star_near_core_scale, close) * scale_mul,
        aura_scale: lerp(meta.star_far_aura_scale, meta.star_near_aura_scale, close)
            * scale_mul
            * far_half_taper,
        core_alpha: (lerp(meta.star_far_core_alpha, meta.star_near_core_alpha, close)
            * alpha_boost)
            .min(meta.star_near_core_alpha.max(meta.star_far_core_alpha))
            .clamp(0.0, 1.0)
            * far_half_taper,
        aura_alpha: (lerp(meta.star_far_aura_alpha, meta.star_near_aura_alpha, close)
            * alpha_boost)
            .min(meta.star_near_aura_alpha)
            .clamp(0.0, 1.0)
            * far_half_taper,
    }
}

pub fn normalized_star_camera_depth(camera_distance: f32, meta: &StudioGalaxyRenderMeta) -> f32 {
    let near = meta.star_near_distance.max(0.0);
    let far = meta.star_far_distance.max(near + f32::EPSILON);
    ((camera_distance - near) / (far - near)).clamp(0.0, 1.0)
}

pub fn mid_to_horizon_extra_falloff(normalized_depth: f32) -> f32 {
    let depth = normalized_depth.clamp(0.0, 1.0);
    if depth <= MID_TO_HORIZON_FALLOFF_START_DEPTH {
        return 1.0;
    }
    let t = ((depth - MID_TO_HORIZON_FALLOFF_START_DEPTH)
        / (1.0 - MID_TO_HORIZON_FALLOFF_START_DEPTH))
        .clamp(0.0, 1.0);
    lerp(1.0, MID_TO_HORIZON_FALLOFF_FACTOR, t)
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

pub fn prepare_star_render_instances(
    stars: &[StudioStarView],
    anchors: &[StudioSystemRenderAnchor],
) -> Vec<StarRenderInstance> {
    stars
        .iter()
        .filter_map(|star| {
            let anchor = anchor_for_system(anchors, star.system_id)?;
            Some(StarRenderInstance {
                system_id: star.system_id,
                position: anchor.world_position,
                scale: star.sprite_scale,
                emissive_strength: star.emissive_strength,
            })
        })
        .collect()
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
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
        let instances = prepare_star_render_instances(&vm.stars, &vm.render_anchors);
        assert_eq!(instances.len(), vm.stars.len());
        assert_eq!(instances.len(), output.result.placement.systems.len());
    }

    #[test]
    fn star_visual_uses_render_anchor_position() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let instances = prepare_star_render_instances(&vm.stars, &vm.render_anchors);
        let instance = instances.first().expect("star instance");
        let anchor = crate::view_model::anchor_for_system(&vm.render_anchors, instance.system_id)
            .expect("anchor");
        assert_eq!(instance.position, anchor.world_position);
    }

    #[test]
    fn star_distance_visual_far_is_small_point() {
        let meta = star_visual_defaults();
        let visual = star_distance_visual(meta.star_far_distance + 100.0, false, false, &meta);
        assert!(visual.core_scale <= meta.star_far_core_scale + f32::EPSILON);
        assert!(visual.core_scale < meta.star_near_core_scale);
    }

    #[test]
    fn star_distance_visual_far_aura_is_minimal() {
        let meta = star_visual_defaults();
        let visual = star_distance_visual(meta.star_far_distance + 100.0, false, false, &meta);
        assert!(visual.aura_alpha <= 0.01);
        assert!(visual.aura_scale <= meta.star_far_aura_scale + f32::EPSILON);
    }

    #[test]
    fn maximum_aura_radius_is_half_of_current_baseline_rule() {
        let meta = star_visual_defaults();
        assert!(
            (meta.star_near_aura_scale
                - PR2R5_STAR_NEAR_AURA_SCALE * PR2R6_AURA_CAP_REDUCTION_FACTOR)
                .abs()
                < f32::EPSILON
        );
        let near = star_distance_visual(meta.star_near_distance, false, false, &meta);
        assert!((near.aura_scale - meta.star_near_aura_scale).abs() < f32::EPSILON);
    }

    #[test]
    fn nearest_star_peak_luminosity_preserved() {
        let meta = star_visual_defaults();
        let near = star_distance_visual(meta.star_near_distance, false, false, &meta);
        assert!((near.core_alpha - meta.star_near_core_alpha).abs() < f32::EPSILON);
        assert_eq!(mid_to_horizon_extra_falloff(0.0), 1.0);
    }

    #[test]
    fn near_star_visibility_not_zeroed() {
        let meta = star_visual_defaults();
        let near = star_distance_visual(meta.star_near_distance, false, false, &meta);
        assert!(near.core_scale > 0.0);
        assert!(near.core_alpha >= 0.95);
        assert!(near.aura_scale > 0.0);
        assert!(near.aura_alpha > 0.0);
    }

    #[test]
    fn mid_to_horizon_extra_falloff_applies_to_aura_radius() {
        let meta = star_visual_defaults();
        let mid_distance = distance_for_depth(&meta, MID_TO_HORIZON_FALLOFF_START_DEPTH);
        let horizon_distance = distance_for_depth(&meta, 1.0);
        let mid = star_distance_visual(mid_distance, false, false, &meta);
        let horizon = star_distance_visual(horizon_distance, false, false, &meta);
        let horizon_base_aura = lerp(meta.star_far_aura_scale, meta.star_near_aura_scale, 0.0);
        assert_eq!(
            mid_to_horizon_extra_falloff(MID_TO_HORIZON_FALLOFF_START_DEPTH),
            1.0
        );
        assert_eq!(
            mid_to_horizon_extra_falloff(1.0),
            MID_TO_HORIZON_FALLOFF_FACTOR
        );
        assert!(mid.aura_scale > horizon.aura_scale);
        assert!(
            (horizon.aura_scale - horizon_base_aura * MID_TO_HORIZON_FALLOFF_FACTOR).abs()
                < f32::EPSILON
        );
    }

    #[test]
    fn mid_to_horizon_extra_falloff_applies_to_luminosity() {
        let meta = star_visual_defaults();
        let horizon = star_distance_visual(meta.star_far_distance, false, false, &meta);
        assert!(
            (horizon.core_alpha - meta.star_far_core_alpha * MID_TO_HORIZON_FALLOFF_FACTOR).abs()
                < f32::EPSILON
        );
        assert!(
            (horizon.aura_alpha - meta.star_far_aura_alpha * MID_TO_HORIZON_FALLOFF_FACTOR).abs()
                < f32::EPSILON
        );
    }

    #[test]
    fn mid_to_horizon_extra_falloff_interpolates_between_half_and_horizon() {
        assert_eq!(mid_to_horizon_extra_falloff(0.25), 1.0);
        assert_eq!(mid_to_horizon_extra_falloff(0.5), 1.0);
        assert!((mid_to_horizon_extra_falloff(0.75) - 0.875).abs() < f32::EPSILON);
        assert_eq!(mid_to_horizon_extra_falloff(1.0), 0.75);
    }

    #[test]
    fn star_distance_visual_near_is_larger_than_far() {
        let meta = star_visual_defaults();
        let near = star_distance_visual(meta.star_near_distance, false, false, &meta);
        let far = star_distance_visual(meta.star_far_distance, false, false, &meta);
        assert!(near.core_scale > far.core_scale);
        assert!(near.aura_scale > far.aura_scale);
        assert!(near.aura_alpha > far.aura_alpha);
    }

    #[test]
    fn star_distance_visual_selected_is_larger_or_brighter_than_unselected() {
        let meta = star_visual_defaults();
        let distance = (meta.star_near_distance + meta.star_far_distance) * 0.5;
        let selected = star_distance_visual(distance, true, false, &meta);
        let unselected = star_distance_visual(distance, false, false, &meta);
        assert!(selected.core_scale > unselected.core_scale);
        assert!(selected.core_alpha >= unselected.core_alpha);
        assert!(selected.aura_alpha >= unselected.aura_alpha);
    }

    #[test]
    fn star_distance_visual_aura_never_exceeds_configured_max() {
        let meta = star_visual_defaults();
        for distance in [0.0, meta.star_near_distance, 120.0, meta.star_far_distance] {
            let visual = star_distance_visual(distance, true, false, &meta);
            assert!(visual.aura_alpha <= meta.star_near_aura_alpha);
        }
    }

    #[test]
    fn aura_overview_scale_is_below_max_threshold() {
        let meta = star_visual_defaults();
        let visual = star_distance_visual(meta.star_far_distance, false, false, &meta);
        assert!(
            visual.aura_scale
                <= PR2R5_STAR_FAR_AURA_SCALE * MID_TO_HORIZON_FALLOFF_FACTOR + f32::EPSILON
        );
    }

    #[test]
    fn aura_overview_alpha_is_below_max_threshold() {
        let meta = star_visual_defaults();
        let visual = star_distance_visual(meta.star_far_distance, false, false, &meta);
        assert!(visual.aura_alpha <= 0.01);
    }

    #[test]
    fn star_visual_metadata_is_render_only() {
        assert!(STAR_DISTANCE_VISUAL_RENDER_ONLY_NOTE.contains("editor render metadata only"));
        assert!(STAR_DISTANCE_VISUAL_RENDER_ONLY_NOTE.contains("bloom"));
    }

    fn distance_for_depth(meta: &StudioGalaxyRenderMeta, depth: f32) -> f32 {
        meta.star_near_distance + (meta.star_far_distance - meta.star_near_distance) * depth
    }
}
