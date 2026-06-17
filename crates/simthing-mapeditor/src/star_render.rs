//! Render-only star and hyperlane visual tuning helpers.

use bevy::prelude::*;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StarRenderMode {
    BloomStarburst,
    CrispCircle,
}

impl Default for StarRenderMode {
    fn default() -> Self {
        Self::BloomStarburst
    }
}

impl StarRenderMode {
    pub const ALL: [Self; 2] = [Self::BloomStarburst, Self::CrispCircle];

    pub fn label(self) -> &'static str {
        match self {
            Self::BloomStarburst => "Bloom / Starburst",
            Self::CrispCircle => "Crisp Circle",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StarFalloffSettings {
    pub base_blur_radius: f32,
    pub falloff_distance_percent: f32,
    pub falloff_blur_radius_percent: f32,
    pub falloff_opacity_percent: f32,
}

impl Default for StarFalloffSettings {
    fn default() -> Self {
        Self {
            base_blur_radius: PR2R6_STAR_NEAR_AURA_SCALE,
            falloff_distance_percent: 100.0,
            falloff_blur_radius_percent: PR2R5_STAR_FAR_AURA_SCALE * MID_TO_HORIZON_FALLOFF_FACTOR
                / PR2R6_STAR_NEAR_AURA_SCALE
                * 100.0,
            falloff_opacity_percent: 2.7,
        }
    }
}

impl StarFalloffSettings {
    pub fn clamped(self) -> Self {
        Self {
            base_blur_radius: self.base_blur_radius.clamp(0.0, 1.0),
            falloff_distance_percent: self.falloff_distance_percent.clamp(1.0, 100.0),
            falloff_blur_radius_percent: self.falloff_blur_radius_percent.clamp(0.0, 100.0),
            falloff_opacity_percent: self.falloff_opacity_percent.clamp(0.0, 100.0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StarFalloffVisual {
    pub blur_radius: f32,
    pub opacity: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StarBillboardRenderSettings {
    pub base_star_blur_radius: f32,
    pub falloff_distance_percent: f32,
    pub falloff_star_blur_radius_percent: f32,
    pub falloff_star_opacity_percent: f32,
    pub near_distance: f32,
    pub far_horizon_distance: f32,
    pub selected_star_scale_multiplier: f32,
    pub hovered_star_scale_multiplier: f32,
    pub far_core_scale: f32,
    pub near_core_scale: f32,
    pub near_core_alpha: f32,
    pub near_aura_alpha: f32,
    pub render_mode: StarRenderMode,
}

impl StarBillboardRenderSettings {
    pub fn from_meta(meta: &StudioGalaxyRenderMeta) -> Self {
        let falloff = meta.star_falloff_settings.clamped();
        Self {
            base_star_blur_radius: falloff.base_blur_radius,
            falloff_distance_percent: falloff.falloff_distance_percent,
            falloff_star_blur_radius_percent: falloff.falloff_blur_radius_percent,
            falloff_star_opacity_percent: falloff.falloff_opacity_percent,
            near_distance: meta.star_near_distance.max(0.0),
            far_horizon_distance: meta
                .star_far_distance
                .max(meta.star_near_distance.max(0.0) + f32::EPSILON),
            selected_star_scale_multiplier: meta.selected_star_scale_multiplier,
            hovered_star_scale_multiplier: meta.hovered_star_scale_multiplier,
            far_core_scale: meta.star_far_core_scale,
            near_core_scale: meta.star_near_core_scale,
            near_core_alpha: meta.star_near_core_alpha,
            near_aura_alpha: meta.star_near_aura_alpha,
            render_mode: meta.star_render_mode,
        }
    }

    pub fn falloff_settings(&self) -> StarFalloffSettings {
        StarFalloffSettings {
            base_blur_radius: self.base_star_blur_radius,
            falloff_distance_percent: self.falloff_distance_percent,
            falloff_blur_radius_percent: self.falloff_star_blur_radius_percent,
            falloff_opacity_percent: self.falloff_star_opacity_percent,
        }
        .clamped()
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct StarBillboardInstance {
    pub system_id: u32,
    pub structural_col: u32,
    pub structural_row: u32,
    pub anchor_position: Vec3,
    pub base_scale_variation: f32,
    pub base_intensity_variation: f32,
    pub selected: bool,
    pub hovered: bool,
}

impl StarBillboardInstance {
    pub fn with_view_state(mut self, selected: bool, hovered: bool) -> Self {
        self.selected = selected;
        self.hovered = hovered;
        self
    }
}

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
    pub aura_radius: f32,
    pub core_alpha: f32,
    pub aura_alpha: f32,
    pub luminosity: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StarRadiusVisual {
    pub core_radius: f32,
    pub aura_radius: f32,
    pub opacity: f32,
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
    let settings = StarBillboardRenderSettings::from_meta(meta);
    let depth_percent = normalized_billboard_camera_depth_percent(camera_distance, &settings);
    compute_star_distance_visual(depth_percent, selected, hovered, &settings)
}

pub fn nearest_camera_star_disc_width_world(meta: &StudioGalaxyRenderMeta) -> f32 {
    let settings = StarBillboardRenderSettings::from_meta(meta);
    let visual = compute_star_distance_visual(0.0, false, false, &settings);
    (star_world_scale(meta, 1.0) * visual.core_scale).max(f32::EPSILON)
}

pub fn compute_star_distance_visual(
    camera_depth_percent: f32,
    selected: bool,
    hovered: bool,
    settings: &StarBillboardRenderSettings,
) -> StarDistanceVisual {
    let t = (camera_depth_percent / 100.0).clamp(0.0, 1.0);
    let radius = compute_star_radius_visual(
        camera_depth_percent,
        settings,
        settings.render_mode,
        selected,
        hovered,
    );
    let falloff = compute_star_falloff_visual(camera_depth_percent, settings.falloff_settings());
    let eased_far = t * t * (3.0 - 2.0 * t);
    let close = 1.0 - eased_far;
    let alpha_boost = if selected {
        1.35
    } else if hovered {
        1.12
    } else {
        1.0
    };
    let core_alpha = (settings.near_core_alpha * radius.opacity * alpha_boost)
        .min(settings.near_core_alpha)
        .clamp(0.0, 1.0);
    let aura_alpha = (settings.near_aura_alpha * falloff.opacity * alpha_boost)
        .min(settings.near_aura_alpha)
        .clamp(0.0, 1.0);
    let aura_alpha = if settings.render_mode == StarRenderMode::CrispCircle {
        0.0
    } else {
        aura_alpha
    };
    StarDistanceVisual {
        core_scale: radius.core_radius
            * lerp(settings.far_core_scale, settings.near_core_scale, close).max(0.1),
        aura_scale: radius.aura_radius,
        aura_radius: radius.aura_radius,
        core_alpha,
        aura_alpha,
        luminosity: core_alpha,
    }
}

pub fn compute_star_radius_visual(
    camera_depth_percent: f32,
    settings: &StarBillboardRenderSettings,
    mode: StarRenderMode,
    selected: bool,
    hovered: bool,
) -> StarRadiusVisual {
    let falloff = compute_star_falloff_visual(camera_depth_percent, settings.falloff_settings());
    let scale_mul = if selected {
        settings.selected_star_scale_multiplier
    } else if hovered {
        settings.hovered_star_scale_multiplier
    } else {
        1.0
    };
    match mode {
        StarRenderMode::BloomStarburst => StarRadiusVisual {
            core_radius: falloff.blur_radius * scale_mul,
            aura_radius: falloff.blur_radius * scale_mul,
            opacity: falloff.opacity,
        },
        StarRenderMode::CrispCircle => StarRadiusVisual {
            core_radius: falloff.blur_radius * scale_mul,
            aura_radius: 0.0,
            opacity: falloff.opacity,
        },
    }
}

pub fn normalized_star_camera_depth(camera_distance: f32, meta: &StudioGalaxyRenderMeta) -> f32 {
    normalized_billboard_camera_depth_percent(
        camera_distance,
        &StarBillboardRenderSettings::from_meta(meta),
    ) / 100.0
}

pub fn normalized_billboard_camera_depth_percent(
    camera_distance: f32,
    settings: &StarBillboardRenderSettings,
) -> f32 {
    let near = settings.near_distance.max(0.0);
    let far = settings.far_horizon_distance.max(near + f32::EPSILON);
    (((camera_distance - near) / (far - near)).clamp(0.0, 1.0)) * 100.0
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

pub fn compute_star_falloff_visual(
    camera_depth_percent: f32,
    settings: StarFalloffSettings,
) -> StarFalloffVisual {
    let settings = settings.clamped();
    let depth = camera_depth_percent.clamp(0.0, 100.0);
    let falloff_at = settings.falloff_distance_percent;
    let target_blur = settings.base_blur_radius * settings.falloff_blur_radius_percent / 100.0;
    let target_opacity = settings.falloff_opacity_percent / 100.0;
    if depth <= falloff_at {
        let t = if falloff_at <= f32::EPSILON {
            1.0
        } else {
            (depth / falloff_at).clamp(0.0, 1.0)
        };
        return StarFalloffVisual {
            blur_radius: lerp(settings.base_blur_radius, target_blur, t),
            opacity: lerp(1.0, target_opacity, t),
        };
    }
    let horizon_t = ((depth - falloff_at) / (100.0 - falloff_at).max(f32::EPSILON)).clamp(0.0, 1.0);
    let horizon_taper = lerp(1.0, MID_TO_HORIZON_FALLOFF_FACTOR, horizon_t);
    StarFalloffVisual {
        blur_radius: target_blur * horizon_taper,
        opacity: target_opacity * horizon_taper,
    }
}

pub fn apply_star_falloff_settings_to_meta(
    meta: &mut StudioGalaxyRenderMeta,
    settings: StarFalloffSettings,
) {
    let settings = settings.clamped();
    meta.star_falloff_settings = settings;
    meta.star_near_aura_scale = settings.base_blur_radius;
    let horizon = compute_star_falloff_visual(100.0, settings);
    meta.star_far_aura_scale = horizon.blur_radius;
    meta.star_far_core_alpha = horizon.opacity;
    meta.star_far_aura_alpha = meta.star_near_aura_alpha * horizon.opacity;
}

pub fn apply_star_render_mode_to_meta(meta: &mut StudioGalaxyRenderMeta, mode: StarRenderMode) {
    meta.star_render_mode = mode;
}

pub fn star_visuals_dirty_after_settings_change(
    previous_settings: StarFalloffSettings,
    next_settings: StarFalloffSettings,
    previous_mode: StarRenderMode,
    next_mode: StarRenderMode,
) -> bool {
    previous_settings.clamped() != next_settings.clamped() || previous_mode != next_mode
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
    prepare_star_billboard_instances(stars, anchors, None, None)
        .into_iter()
        .map(|star| StarRenderInstance {
            system_id: star.system_id,
            position: star.anchor_position.to_array(),
            scale: star.base_scale_variation,
            emissive_strength: star.base_intensity_variation,
        })
        .collect()
}

pub fn prepare_star_billboard_instances(
    stars: &[StudioStarView],
    anchors: &[StudioSystemRenderAnchor],
    selected_system_id: Option<u32>,
    hovered_system_id: Option<u32>,
) -> Vec<StarBillboardInstance> {
    stars
        .iter()
        .filter_map(|star| {
            let anchor = anchor_for_system(anchors, star.system_id)?;
            Some(StarBillboardInstance {
                system_id: star.system_id,
                structural_col: anchor.structural_col,
                structural_row: anchor.structural_row,
                anchor_position: Vec3::from_array(anchor.world_position),
                base_scale_variation: star.sprite_scale,
                base_intensity_variation: star.emissive_strength,
                selected: selected_system_id == Some(star.system_id),
                hovered: hovered_system_id == Some(star.system_id),
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
    fn star_billboard_instance_count_matches_system_count() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let instances = prepare_star_billboard_instances(&vm.stars, &vm.render_anchors, None, None);
        assert_eq!(instances.len(), vm.stars.len());
        assert_eq!(instances.len(), output.result.placement.systems.len());
    }

    #[test]
    fn star_billboard_instances_use_render_anchors() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let instances = prepare_star_billboard_instances(&vm.stars, &vm.render_anchors, None, None);
        let instance = instances.first().expect("star instance");
        let anchor = crate::view_model::anchor_for_system(&vm.render_anchors, instance.system_id)
            .expect("anchor");
        assert_eq!(instance.anchor_position.to_array(), anchor.world_position);
    }

    #[test]
    fn star_billboard_anchor_preserves_structural_coord_reference() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let instances = prepare_star_billboard_instances(&vm.stars, &vm.render_anchors, None, None);
        for instance in instances {
            let anchor =
                crate::view_model::anchor_for_system(&vm.render_anchors, instance.system_id)
                    .expect("anchor");
            assert_eq!(instance.structural_col, anchor.structural_col);
            assert_eq!(instance.structural_row, anchor.structural_row);
        }
    }

    #[test]
    fn legacy_star_render_instances_wrap_billboard_instances() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let instances = prepare_star_render_instances(&vm.stars, &vm.render_anchors);
        let instance = instances.first().expect("legacy star instance");
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
    fn star_distance_visual_near_peak_luminosity_preserved() {
        let meta = star_visual_defaults();
        let near = star_distance_visual(meta.star_near_distance, false, false, &meta);
        assert!((near.core_alpha - meta.star_near_core_alpha).abs() < f32::EPSILON);
        assert!((near.luminosity - meta.star_near_core_alpha).abs() < f32::EPSILON);
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
        let mid_distance = distance_for_depth(&meta, 0.5);
        let horizon_distance = distance_for_depth(&meta, 1.0);
        let mid = star_distance_visual(mid_distance, false, false, &meta);
        let horizon = star_distance_visual(horizon_distance, false, false, &meta);
        assert_eq!(
            mid_to_horizon_extra_falloff(MID_TO_HORIZON_FALLOFF_START_DEPTH),
            1.0
        );
        assert_eq!(
            mid_to_horizon_extra_falloff(1.0),
            MID_TO_HORIZON_FALLOFF_FACTOR
        );
        assert!(mid.aura_scale > horizon.aura_scale);
    }

    #[test]
    fn mid_to_horizon_extra_falloff_applies_to_luminosity() {
        let meta = star_visual_defaults();
        let horizon = star_distance_visual(meta.star_far_distance, false, false, &meta);
        let target = compute_star_falloff_visual(100.0, meta.star_falloff_settings);
        assert!((horizon.core_alpha - target.opacity).abs() < f32::EPSILON);
        assert!((horizon.aura_alpha - meta.star_near_aura_alpha * target.opacity).abs() < 0.0001);
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
    fn star_distance_visual_far_is_smaller_than_near() {
        let meta = star_visual_defaults();
        let near = star_distance_visual(meta.star_near_distance, false, false, &meta);
        let far = star_distance_visual(meta.star_far_distance, false, false, &meta);
        assert!(far.core_scale < near.core_scale);
        assert!(far.aura_radius < near.aura_radius);
        assert!(far.luminosity < near.luminosity);
    }

    #[test]
    fn selected_star_visual_is_larger_or_brighter_than_unselected() {
        let meta = star_visual_defaults();
        let distance = (meta.star_near_distance + meta.star_far_distance) * 0.5;
        let selected = star_distance_visual(distance, true, false, &meta);
        let unselected = star_distance_visual(distance, false, false, &meta);
        assert!(selected.core_scale > unselected.core_scale);
        assert!(selected.core_alpha >= unselected.core_alpha);
        assert!(selected.aura_alpha >= unselected.aura_alpha);
    }

    #[test]
    fn hovered_star_visual_is_larger_or_brighter_than_unhovered() {
        let meta = star_visual_defaults();
        let distance = (meta.star_near_distance + meta.star_far_distance) * 0.5;
        let hovered = star_distance_visual(distance, false, true, &meta);
        let unhovered = star_distance_visual(distance, false, false, &meta);
        assert!(hovered.core_scale > unhovered.core_scale);
        assert!(hovered.core_alpha >= unhovered.core_alpha);
        assert!(hovered.aura_alpha >= unhovered.aura_alpha);
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
        assert!(visual.aura_scale <= meta.star_near_aura_scale + f32::EPSILON);
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

    #[test]
    fn nearest_camera_star_disc_width_is_positive_render_value() {
        let meta = star_visual_defaults();
        assert!(nearest_camera_star_disc_width_world(&meta) > 0.0);
    }

    #[test]
    fn base_star_blur_radius_updates_render_meta() {
        let mut meta = star_visual_defaults();
        let settings = StarFalloffSettings {
            base_blur_radius: 0.42,
            ..Default::default()
        };
        apply_star_falloff_settings_to_meta(&mut meta, settings);
        assert_eq!(meta.star_falloff_settings.base_blur_radius, 0.42);
        assert_eq!(meta.star_near_aura_scale, 0.42);
    }

    #[test]
    fn falloff_distance_percent_updates_render_meta() {
        let mut meta = star_visual_defaults();
        let settings = StarFalloffSettings {
            falloff_distance_percent: 64.0,
            ..Default::default()
        };
        apply_star_falloff_settings_to_meta(&mut meta, settings);
        assert_eq!(meta.star_falloff_settings.falloff_distance_percent, 64.0);
    }

    #[test]
    fn falloff_star_blur_radius_percent_updates_render_meta() {
        let mut meta = star_visual_defaults();
        let settings = StarFalloffSettings {
            falloff_blur_radius_percent: 33.0,
            ..Default::default()
        };
        apply_star_falloff_settings_to_meta(&mut meta, settings);
        assert_eq!(meta.star_falloff_settings.falloff_blur_radius_percent, 33.0);
    }

    #[test]
    fn falloff_star_opacity_percent_updates_render_meta() {
        let mut meta = star_visual_defaults();
        let settings = StarFalloffSettings {
            falloff_opacity_percent: 44.0,
            ..Default::default()
        };
        apply_star_falloff_settings_to_meta(&mut meta, settings);
        assert_eq!(meta.star_falloff_settings.falloff_opacity_percent, 44.0);
    }

    #[test]
    fn compute_star_falloff_visual_reaches_target_radius_at_falloff_distance() {
        let settings = StarFalloffSettings {
            base_blur_radius: 0.8,
            falloff_distance_percent: 40.0,
            falloff_blur_radius_percent: 25.0,
            falloff_opacity_percent: 70.0,
        };
        let visual = compute_star_falloff_visual(40.0, settings);
        assert!((visual.blur_radius - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_star_falloff_visual_reaches_target_opacity_at_falloff_distance() {
        let settings = StarFalloffSettings {
            base_blur_radius: 0.8,
            falloff_distance_percent: 40.0,
            falloff_blur_radius_percent: 25.0,
            falloff_opacity_percent: 70.0,
        };
        let visual = compute_star_falloff_visual(40.0, settings);
        assert!((visual.opacity - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn star_distance_visual_reaches_settings_falloff_radius_at_falloff_distance() {
        let settings =
            test_billboard_settings(0.8, 40.0, 25.0, 70.0, StarRenderMode::BloomStarburst);
        let visual = compute_star_distance_visual(
            settings.falloff_distance_percent,
            false,
            false,
            &settings,
        );
        assert!((visual.aura_radius - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn star_distance_visual_reaches_settings_falloff_opacity_at_falloff_distance() {
        let settings =
            test_billboard_settings(0.8, 40.0, 25.0, 70.0, StarRenderMode::BloomStarburst);
        let visual = compute_star_distance_visual(
            settings.falloff_distance_percent,
            false,
            false,
            &settings,
        );
        assert!((visual.luminosity - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn base_star_blur_radius_changes_computed_billboard_radius() {
        let low = test_billboard_settings(0.11, 100.0, 20.0, 70.0, StarRenderMode::BloomStarburst);
        let high = test_billboard_settings(0.80, 100.0, 20.0, 70.0, StarRenderMode::BloomStarburst);
        let low_visual =
            compute_star_radius_visual(0.0, &low, StarRenderMode::BloomStarburst, false, false);
        let high_visual =
            compute_star_radius_visual(0.0, &high, StarRenderMode::BloomStarburst, false, false);
        assert!(high_visual.aura_radius > low_visual.aura_radius * 6.0);
        assert!(high_visual.core_radius > low_visual.core_radius * 6.0);
    }

    #[test]
    fn base_star_blur_radius_changes_computed_crisp_circle_radius() {
        let low = test_billboard_settings(0.11, 100.0, 20.0, 70.0, StarRenderMode::CrispCircle);
        let high = test_billboard_settings(0.80, 100.0, 20.0, 70.0, StarRenderMode::CrispCircle);
        let low_visual =
            compute_star_radius_visual(0.0, &low, StarRenderMode::CrispCircle, false, false);
        let high_visual =
            compute_star_radius_visual(0.0, &high, StarRenderMode::CrispCircle, false, false);
        assert!(high_visual.core_radius > low_visual.core_radius * 6.0);
        assert_eq!(low_visual.aura_radius, 0.0);
        assert_eq!(high_visual.aura_radius, 0.0);
    }

    #[test]
    fn falloff_star_blur_radius_reaches_expected_radius_at_falloff_distance() {
        let settings =
            test_billboard_settings(0.8, 40.0, 13.0, 70.0, StarRenderMode::BloomStarburst);
        let visual = compute_star_radius_visual(
            settings.falloff_distance_percent,
            &settings,
            StarRenderMode::BloomStarburst,
            false,
            false,
        );
        assert!((visual.aura_radius - 0.104).abs() < 0.0001);
    }

    #[test]
    fn falloff_star_opacity_reaches_expected_opacity_at_falloff_distance() {
        let settings =
            test_billboard_settings(0.8, 40.0, 13.0, 70.0, StarRenderMode::BloomStarburst);
        let visual = compute_star_radius_visual(
            settings.falloff_distance_percent,
            &settings,
            StarRenderMode::BloomStarburst,
            false,
            false,
        );
        assert!((visual.opacity - 0.70).abs() < f32::EPSILON);
    }

    #[test]
    fn changing_base_star_blur_radius_marks_star_visuals_dirty() {
        let previous = StarFalloffSettings {
            base_blur_radius: 0.11,
            ..Default::default()
        };
        let next = StarFalloffSettings {
            base_blur_radius: 0.80,
            ..Default::default()
        };
        assert!(star_visuals_dirty_after_settings_change(
            previous,
            next,
            StarRenderMode::BloomStarburst,
            StarRenderMode::BloomStarburst
        ));
    }

    #[test]
    fn settings_change_updates_star_visual_without_regenerating_galaxy() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let mut vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let seed = vm.seed;
        let star_count = vm.stars.len();
        let before = star_distance_visual(90.0, false, false, &vm.render_meta);
        vm.apply_star_falloff_settings(StarFalloffSettings {
            base_blur_radius: 0.80,
            falloff_distance_percent: 65.0,
            falloff_blur_radius_percent: 13.0,
            falloff_opacity_percent: 40.0,
        });
        vm.apply_star_render_mode(StarRenderMode::CrispCircle);
        let after = star_distance_visual(90.0, false, false, &vm.render_meta);
        assert_eq!(vm.seed, seed);
        assert_eq!(vm.stars.len(), star_count);
        assert_ne!(before.core_scale, after.core_scale);
        assert_eq!(after.aura_radius, 0.0);
    }

    #[test]
    fn crisp_circle_mode_uses_shared_render_anchor() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let mut vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        vm.apply_star_render_mode(StarRenderMode::CrispCircle);
        let instance = prepare_star_billboard_instances(&vm.stars, &vm.render_anchors, None, None)
            .into_iter()
            .next()
            .expect("instance");
        let anchor = crate::view_model::anchor_for_system(&vm.render_anchors, instance.system_id)
            .expect("anchor");
        assert_eq!(vm.render_meta.star_render_mode, StarRenderMode::CrispCircle);
        assert_eq!(instance.anchor_position.to_array(), anchor.world_position);
    }

    #[test]
    fn bloom_mode_uses_shared_render_anchor() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let instance = prepare_star_billboard_instances(&vm.stars, &vm.render_anchors, None, None)
            .into_iter()
            .next()
            .expect("instance");
        let anchor = crate::view_model::anchor_for_system(&vm.render_anchors, instance.system_id)
            .expect("anchor");
        assert_eq!(
            vm.render_meta.star_render_mode,
            StarRenderMode::BloomStarburst
        );
        assert_eq!(instance.anchor_position.to_array(), anchor.world_position);
    }

    #[test]
    fn selected_star_radius_or_emphasis_exceeds_unselected() {
        let settings =
            test_billboard_settings(0.5, 100.0, 20.0, 70.0, StarRenderMode::BloomStarburst);
        let selected =
            compute_star_radius_visual(0.0, &settings, StarRenderMode::BloomStarburst, true, false);
        let unselected = compute_star_radius_visual(
            0.0,
            &settings,
            StarRenderMode::BloomStarburst,
            false,
            false,
        );
        assert!(selected.core_radius > unselected.core_radius);
        assert!(selected.aura_radius > unselected.aura_radius);
    }

    #[test]
    fn hovered_star_radius_or_emphasis_exceeds_unhovered() {
        let settings = test_billboard_settings(0.5, 100.0, 20.0, 70.0, StarRenderMode::CrispCircle);
        let hovered =
            compute_star_radius_visual(0.0, &settings, StarRenderMode::CrispCircle, false, true);
        let unhovered =
            compute_star_radius_visual(0.0, &settings, StarRenderMode::CrispCircle, false, false);
        assert!(hovered.core_radius > unhovered.core_radius);
        assert_eq!(hovered.aura_radius, 0.0);
    }

    #[test]
    fn hyperlane_endpoints_remain_attached_to_render_anchors_after_radius_change() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let mut vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        vm.apply_star_falloff_settings(StarFalloffSettings {
            base_blur_radius: 0.80,
            falloff_blur_radius_percent: 13.0,
            ..Default::default()
        });
        for segment in vm.hyperlane_render_segments() {
            let from = crate::view_model::anchor_for_system_str(
                &vm.render_anchors,
                &segment.from_system_id,
            )
            .expect("from anchor");
            let to =
                crate::view_model::anchor_for_system_str(&vm.render_anchors, &segment.to_system_id)
                    .expect("to anchor");
            assert_eq!(segment.from, from.world_position);
            assert_eq!(segment.to, to.world_position);
        }
    }

    #[test]
    fn settings_update_changes_star_visual_without_regenerating_galaxy() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let mut vm = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        let seed = vm.seed;
        let star_count = vm.stars.len();
        let anchor_count = vm.render_anchors.len();
        let before = star_distance_visual(120.0, false, false, &vm.render_meta);
        vm.apply_star_falloff_settings(StarFalloffSettings {
            base_blur_radius: 0.25,
            falloff_distance_percent: 50.0,
            falloff_blur_radius_percent: 10.0,
            falloff_opacity_percent: 20.0,
        });
        let after = star_distance_visual(120.0, false, false, &vm.render_meta);
        assert_eq!(vm.seed, seed);
        assert_eq!(vm.stars.len(), star_count);
        assert_eq!(vm.render_anchors.len(), anchor_count);
        assert_ne!(before.aura_radius, after.aura_radius);
        assert_ne!(before.luminosity, after.luminosity);
    }

    fn test_billboard_settings(
        base_star_blur_radius: f32,
        falloff_distance_percent: f32,
        falloff_star_blur_radius_percent: f32,
        falloff_star_opacity_percent: f32,
        render_mode: StarRenderMode,
    ) -> StarBillboardRenderSettings {
        StarBillboardRenderSettings {
            base_star_blur_radius,
            falloff_distance_percent,
            falloff_star_blur_radius_percent,
            falloff_star_opacity_percent,
            near_distance: 10.0,
            far_horizon_distance: 110.0,
            selected_star_scale_multiplier: 1.85,
            hovered_star_scale_multiplier: 1.22,
            far_core_scale: 0.1,
            near_core_scale: 0.68,
            near_core_alpha: 1.0,
            near_aura_alpha: 0.22,
            render_mode,
        }
    }

    fn distance_for_depth(meta: &StudioGalaxyRenderMeta, depth: f32) -> f32 {
        meta.star_near_distance + (meta.star_far_distance - meta.star_near_distance) * depth
    }
}
