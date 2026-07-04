//! Render-only star and hyperlane visual tuning helpers.

use bevy::prelude::*;

use crate::hyperlane_buckets::{bucket_alpha_for_meta, HyperlaneDepthBucket};
use crate::view_model::{
    anchor_for_system, StudioGalaxyRenderMeta, StudioStarView, StudioSystemRenderAnchor,
};
use simthing_tools::{WorldTextBillboard, WorldTextPlacementMode};

use crate::falloff_metric::{
    plateau_falloff_t_percent, plateau_interpolate, world_position_map_progress_percent,
    StudioMapRadiusFalloffContext,
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
/// Minimum projected label height (px) before screen-companion nameplates hard-cull (legacy telemetry).
pub const MIN_LEGIBLE_NAMEPLATE_PX: f32 = 18.0;
/// Hard readability cutoff for unselected star nameplates.
pub const MIN_UNSELECTED_LABEL_HEIGHT_PX: f32 = 24.0;
/// Lower readability cutoff for selected or hovered star nameplates (shader may bump to this minimum).
pub const MIN_FOCUSED_LABEL_HEIGHT_PX: f32 = 16.0;
/// Maximum unselected labels before global density gate hides them.
pub const MAX_OVERVIEW_LABELS: usize = 250;
/// Maximum estimated label coverage fraction before global density gate.
pub const MAX_LABEL_COVERAGE: f32 = 0.15;
/// Nameplate label height tracks 100% of the rendered star visual envelope at the current depth.
pub const STAR_NAMEPLATE_HEIGHT_FACTOR: f32 = 1.0;
pub const PR2R6_STAR_NEAR_AURA_SCALE: f32 =
    PR2R5_STAR_NEAR_AURA_SCALE * PR2R6_AURA_CAP_REDUCTION_FACTOR;
pub const STAR_DISTANCE_VISUAL_RENDER_ONLY_NOTE: &str =
    "star distance attenuation, core/aura scale, alpha, and bloom are editor render metadata only";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StarNameplateSettings {
    /// Historical serialized name; interpreted as uniform relative size (not horizontal-only width).
    pub relative_width_percent: f32,
    pub base_transparency_percent: f32,
    pub relative_falloff_distance_percent: f32,
    pub relative_falloff_transparency_percent: f32,
}

impl Default for StarNameplateSettings {
    fn default() -> Self {
        Self {
            relative_width_percent: 100.0,
            base_transparency_percent: 100.0,
            relative_falloff_distance_percent: 50.0,
            relative_falloff_transparency_percent: 50.0,
        }
    }
}

impl StarNameplateSettings {
    pub fn clamped(self) -> Self {
        Self {
            relative_width_percent: self.relative_width_percent.clamp(20.0, 200.0),
            base_transparency_percent: self.base_transparency_percent.clamp(0.0, 100.0),
            relative_falloff_distance_percent: self
                .relative_falloff_distance_percent
                .clamp(5.0, 100.0),
            relative_falloff_transparency_percent: self
                .relative_falloff_transparency_percent
                .clamp(0.0, 100.0),
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StarNameplateDebugMode {
    /// Production default: every on-screen label eligible; Settings sliders govern alpha/falloff/scale.
    #[default]
    AllLabelsSettingsDriven,
    /// Debug: apply readability floor and overview density caps.
    AutoLodDebug,
    /// Debug: hide all unselected labels.
    FocusedOnlyDebug,
    /// Debug: bypass LOD/readability/offscreen culls; still respects Settings falloff/alpha.
    ForceAllDebug,
}

impl StarNameplateDebugMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::AllLabelsSettingsDriven => "All labels — settings driven",
            Self::AutoLodDebug => "Auto LOD debug",
            Self::FocusedOnlyDebug => "Focused only debug",
            Self::ForceAllDebug => "Force all debug",
        }
    }

    pub fn is_debug_override(self) -> bool {
        !matches!(self, Self::AllLabelsSettingsDriven)
    }

    pub fn is_force_all_debug(self) -> bool {
        matches!(self, Self::ForceAllDebug)
    }

    pub fn applies_lod_readability_gates(self) -> bool {
        matches!(self, Self::AutoLodDebug | Self::FocusedOnlyDebug)
    }
}

/// GPU globals patch for screen-companion nameplate LOD (written each frame; no glyph rebuild).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StarNameplateLodGlobals {
    pub min_focused_px: f32,
    pub unselected_global_alpha: f32,
    pub min_unselected_px: f32,
}

impl Default for StarNameplateLodGlobals {
    fn default() -> Self {
        Self {
            min_focused_px: 0.0,
            unselected_global_alpha: 1.0,
            min_unselected_px: 0.0,
        }
    }
}

impl StarNameplateDebugMode {
    pub fn lod_globals(self, auto_density_alpha: f32) -> StarNameplateLodGlobals {
        match self {
            Self::AllLabelsSettingsDriven => StarNameplateLodGlobals {
                min_unselected_px: 0.0,
                min_focused_px: 0.0,
                unselected_global_alpha: 1.0,
            },
            Self::AutoLodDebug => StarNameplateLodGlobals {
                min_unselected_px: MIN_UNSELECTED_LABEL_HEIGHT_PX,
                min_focused_px: MIN_FOCUSED_LABEL_HEIGHT_PX,
                unselected_global_alpha: auto_density_alpha.clamp(0.0, 1.0),
            },
            Self::FocusedOnlyDebug => StarNameplateLodGlobals {
                unselected_global_alpha: 0.0,
                min_unselected_px: MIN_UNSELECTED_LABEL_HEIGHT_PX,
                min_focused_px: MIN_FOCUSED_LABEL_HEIGHT_PX,
            },
            Self::ForceAllDebug => StarNameplateLodGlobals {
                // Sentinel: negative min px bypasses offscreen/alpha-epsilon culls in shader; falloff still applies.
                min_unselected_px: -1.0,
                min_focused_px: -1.0,
                unselected_global_alpha: 1.0,
            },
        }
    }
}

/// Estimate whether unselected nameplates should render at all given density.
pub fn nameplate_unselected_global_lod_alpha(
    label_count: usize,
    label_height_px: f32,
    label_width_px: f32,
    viewport_area_px: f32,
) -> f32 {
    if label_count == 0 {
        return 1.0;
    }
    let coverage = label_count as f32 * label_height_px.max(0.0) * label_width_px.max(0.0)
        / viewport_area_px.max(1.0);
    if label_count > MAX_OVERVIEW_LABELS || coverage > MAX_LABEL_COVERAGE {
        0.0
    } else {
        1.0
    }
}

/// Per-label CPU-side LOD gate for telemetry (mirrors shader hard cuts).
pub fn nameplate_label_passes_readability_gate(
    label_height_px: f32,
    focused: bool,
    debug_mode: StarNameplateDebugMode,
) -> bool {
    if matches!(
        debug_mode,
        StarNameplateDebugMode::AllLabelsSettingsDriven | StarNameplateDebugMode::ForceAllDebug
    ) {
        return true;
    }
    let effective = nameplate_effective_label_height_px(label_height_px, focused);
    let min_height = if focused {
        MIN_FOCUSED_LABEL_HEIGHT_PX
    } else {
        MIN_UNSELECTED_LABEL_HEIGHT_PX
    };
    effective >= min_height
}

pub fn nameplate_label_passes_density_gate(
    focused: bool,
    unselected_global_alpha: f32,
    debug_mode: StarNameplateDebugMode,
) -> bool {
    if matches!(
        debug_mode,
        StarNameplateDebugMode::AllLabelsSettingsDriven | StarNameplateDebugMode::ForceAllDebug
    ) {
        return true;
    }
    if debug_mode == StarNameplateDebugMode::AutoLodDebug {
        return focused || unselected_global_alpha > 0.5;
    }
    focused || unselected_global_alpha > 0.5
}

pub fn star_max_layer_scale(visual: StarDistanceVisual, mode: StarRenderMode) -> f32 {
    match mode {
        StarRenderMode::BloomStarburst => visual.core_scale.max(visual.aura_radius),
        StarRenderMode::CrispCircle => visual.core_scale,
    }
}

/// Rendered star visual diameter in world units using the same math as `sync_star_visuals_system`.
pub fn star_rendered_visual_envelope_world_diameter(
    instance: StarBillboardInstance,
    star_settings: &StarBillboardRenderSettings,
    camera_depth_percent: f32,
) -> f32 {
    let visual = compute_star_distance_visual(
        camera_depth_percent,
        instance.selected,
        instance.hovered,
        star_settings,
        true,
    );
    instance.base_scale_variation
        * star_max_layer_scale(visual, star_settings.render_mode)
        * STAR_NAMEPLATE_HEIGHT_FACTOR
}

pub fn star_nameplate_visual_envelope_near_world(
    instance: StarBillboardInstance,
    star_settings: &StarBillboardRenderSettings,
) -> f32 {
    star_rendered_visual_envelope_world_diameter(instance, star_settings, 0.0)
}

pub fn star_nameplate_envelope_height_ratio(
    instance: StarBillboardInstance,
    star_settings: &StarBillboardRenderSettings,
    camera_depth_percent: f32,
) -> f32 {
    let near = star_nameplate_visual_envelope_near_world(instance, star_settings);
    let at_depth =
        star_rendered_visual_envelope_world_diameter(instance, star_settings, camera_depth_percent);
    if near <= f32::EPSILON {
        1.0
    } else {
        (at_depth / near).clamp(0.0, 8.0)
    }
}

/// Approximate vertical screen span for a world-space height at the given anchor (telemetry only).
pub fn estimate_world_vertical_span_screen_px(
    anchor: Vec3,
    camera_pos: Vec3,
    world_span: f32,
    viewport_height: f32,
) -> f32 {
    if world_span <= 0.0 || viewport_height <= 0.0 {
        return 0.0;
    }
    // Camera3d default vertical FOV is PI/4 radians.
    const DEFAULT_VERTICAL_FOV_RAD: f32 = std::f32::consts::FRAC_PI_4;
    let distance = camera_pos.distance(anchor).max(0.001);
    let px_per_world = viewport_height / (2.0 * (DEFAULT_VERTICAL_FOV_RAD * 0.5).tan() * distance);
    world_span * px_per_world
}

#[deprecated(note = "use star_nameplate_visual_envelope_near_world")]
pub fn nameplate_near_label_height_world(
    instance: StarBillboardInstance,
    star_settings: &StarBillboardRenderSettings,
) -> f32 {
    star_nameplate_visual_envelope_near_world(instance, star_settings)
}

pub fn nameplate_effective_falloff_distance_percent(
    star_falloff_distance_percent: f32,
    nameplate_relative_falloff_distance_percent: f32,
) -> f32 {
    let star = star_falloff_distance_percent.clamp(0.0, 100.0);
    let relative = nameplate_relative_falloff_distance_percent.clamp(0.0, 100.0);
    (star * relative / 100.0).min(star)
}

/// Mirrors legacy ramp falloff in `text_instanced.wgsl` for debug metrics only.
pub fn world_text_distance_falloff(
    depth_percent: f32,
    falloff_percent: f32,
    target_value: f32,
    horizon_taper: f32,
) -> f32 {
    let depth = depth_percent.clamp(0.0, 100.0);
    let falloff_at = falloff_percent.clamp(0.0, 100.0).max(0.0001);
    if depth <= falloff_at {
        let t = (depth / falloff_at).clamp(0.0, 1.0);
        return lerp(1.0, target_value.clamp(0.0, 1.0), t);
    }
    let horizon_t = ((depth - falloff_at) / (100.0 - falloff_at).max(0.0001)).clamp(0.0, 1.0);
    target_value.clamp(0.0, 1.0) * lerp(1.0, horizon_taper.clamp(0.0, 1.0), horizon_t)
}

/// Map-radius plateau falloff alpha (production default).
pub fn world_text_plateau_falloff(
    progress_percent: f32,
    plateau_end_percent: f32,
    target_value: f32,
) -> f32 {
    plateau_interpolate(
        1.0,
        target_value.clamp(0.0, 1.0),
        progress_percent,
        plateau_end_percent,
    )
}

/// GPU screen-label falloff alpha (star ceiling × label ramp at effective distance).
pub fn nameplate_gpu_screen_label_falloff_alpha(
    depth_percent: f32,
    billboard: &WorldTextBillboard,
    use_plateau: bool,
) -> f32 {
    let star_at = billboard.ceiling_falloff_percent;
    let effective_at = billboard.relative_falloff_percent.min(star_at).max(0.0);
    if use_plateau {
        let star_alpha =
            world_text_plateau_falloff(depth_percent, star_at, billboard.ceiling_target_alpha);
        let label_ramp = world_text_plateau_falloff(
            depth_percent,
            effective_at,
            billboard.relative_target_alpha,
        );
        return star_alpha * label_ramp;
    }
    let star_alpha = world_text_distance_falloff(
        depth_percent,
        star_at,
        billboard.ceiling_target_alpha,
        billboard.horizon_taper,
    );
    let label_ramp = world_text_distance_falloff(
        depth_percent,
        effective_at,
        billboard.relative_target_alpha,
        billboard.horizon_taper,
    );
    star_alpha * label_ramp
}

pub fn star_nameplate_gpu_screen_label(
    instance: StarBillboardInstance,
    star_settings: &StarBillboardRenderSettings,
    nameplate_settings: StarNameplateSettings,
) -> WorldTextBillboard {
    let nameplate = nameplate_settings.clamped();
    let star_falloff = star_settings.falloff_settings();
    let visual_envelope_near = star_nameplate_visual_envelope_near_world(instance, star_settings);
    let star_falloff_distance = star_falloff.falloff_distance_percent;
    let effective_falloff_distance = nameplate_effective_falloff_distance_percent(
        star_falloff_distance,
        nameplate.relative_falloff_distance_percent,
    );
    // Settings semantics (production visibility):
    // - relative_width_percent (historical name) -> uniform label size vs star blur; width preserves natural aspect.
    // - base_transparency_percent -> alpha ceiling vs star opacity (base_alpha_ratio).
    // - relative_falloff_distance_percent × star falloff distance -> effective label falloff distance.
    // - relative_falloff_transparency_percent -> label alpha target at effective falloff vs star alpha.
    WorldTextBillboard {
        anchor: instance.anchor_position,
        near_height: visual_envelope_near,
        visual_envelope_world_height: visual_envelope_near,
        width_ratio: nameplate.relative_width_percent / 100.0,
        vertical_gap_ratio: 0.10,
        near_distance: star_settings.near_distance,
        far_distance: star_settings.far_horizon_distance,
        target_height_ratio: star_falloff.falloff_blur_radius_percent / 100.0,
        ceiling_falloff_percent: star_falloff.falloff_distance_percent,
        ceiling_target_alpha: star_falloff.falloff_opacity_percent / 100.0,
        base_alpha_ratio: nameplate.base_transparency_percent / 100.0,
        relative_falloff_percent: effective_falloff_distance,
        relative_target_alpha: nameplate.relative_falloff_transparency_percent / 100.0,
        horizon_taper: MID_TO_HORIZON_FALLOFF_FACTOR,
        placement_mode: WorldTextPlacementMode::GpuScreenLabel,
        gpu_screen_label_focused: instance.selected || instance.hovered,
    }
    .clamped()
}

/// Effective label height after focused minimum-readable bump (mirrors GPU screen-label shader).
pub fn nameplate_effective_label_height_px(projected_height_px: f32, focused: bool) -> f32 {
    if focused {
        projected_height_px.max(MIN_FOCUSED_LABEL_HEIGHT_PX)
    } else {
        projected_height_px
    }
}

/// Uniform label height after relative-size scale (mirrors GPU screen-label shader).
pub fn nameplate_scaled_label_height_px(
    projected_star_visual_diameter_px: f32,
    relative_size_ratio: f32,
    focused: bool,
) -> f32 {
    nameplate_effective_label_height_px(projected_star_visual_diameter_px, focused)
        * relative_size_ratio
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
    compute_star_distance_visual(depth_percent, selected, hovered, &settings, true)
}

pub fn nearest_camera_star_disc_width_world(meta: &StudioGalaxyRenderMeta) -> f32 {
    let settings = StarBillboardRenderSettings::from_meta(meta);
    let visual = compute_star_distance_visual(0.0, false, false, &settings, true);
    (star_world_scale(meta, 1.0) * visual.core_scale).max(f32::EPSILON)
}

pub fn compute_star_distance_visual(
    progress_percent: f32,
    selected: bool,
    hovered: bool,
    settings: &StarBillboardRenderSettings,
    use_plateau: bool,
) -> StarDistanceVisual {
    let t = (progress_percent / 100.0).clamp(0.0, 1.0);
    let radius = compute_star_radius_visual(
        progress_percent,
        settings,
        settings.render_mode,
        selected,
        hovered,
        use_plateau,
    );
    let falloff =
        compute_star_falloff_visual(progress_percent, settings.falloff_settings(), use_plateau);
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
    progress_percent: f32,
    settings: &StarBillboardRenderSettings,
    mode: StarRenderMode,
    selected: bool,
    hovered: bool,
    use_plateau: bool,
) -> StarRadiusVisual {
    let falloff =
        compute_star_falloff_visual(progress_percent, settings.falloff_settings(), use_plateau);
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

/// Presentation-space high horizon for Studio star/nameplate visual falloff (screen pixels).
/// The falloff metric uses a high horizon at 25% from the top, centered horizontally.
/// This is not the 3D camera projection center; it is the artist/Studio visual falloff ruler.
pub const STAR_FALLOFF_BASE_X_FRACTION: f32 = 0.5;
pub const STAR_FALLOFF_BASE_Y_FRACTION: f32 = 1.0;
pub const STAR_FALLOFF_VANISHING_X_FRACTION: f32 = 0.5;
pub const STAR_FALLOFF_VANISHING_Y_FRACTION: f32 = 0.25;

/// Foreground-to-high-horizon ruler for Studio star/nameplate falloff (screen pixels).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VisualHorizonFalloffRuler {
    pub base_px: [f32; 2],
    pub vanishing_px: [f32; 2],
}

impl VisualHorizonFalloffRuler {
    pub fn from_viewport(viewport_width: f32, viewport_height: f32) -> Self {
        Self {
            base_px: [
                viewport_width * STAR_FALLOFF_BASE_X_FRACTION,
                viewport_height * STAR_FALLOFF_BASE_Y_FRACTION,
            ],
            vanishing_px: [
                viewport_width * STAR_FALLOFF_VANISHING_X_FRACTION,
                viewport_height * STAR_FALLOFF_VANISHING_Y_FRACTION,
            ],
        }
    }
}

/// Falloff progress metric for stars and GPU nameplates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StarFalloffMetric {
    /// Default: 0% at view origin on map plane, 100% at farthest map corner.
    #[default]
    MapRadiusPlateau,
    /// Debug: 0% at viewport bottom center, 100% at high horizon (25% from top).
    VisualHorizon,
    /// Telemetry/debug: legacy camera-distance normalization.
    CameraDistanceDebug,
}

impl StarFalloffMetric {
    pub fn label(self) -> &'static str {
        match self {
            Self::MapRadiusPlateau => "Map radius plateau",
            Self::VisualHorizon => "Visual high horizon debug",
            Self::CameraDistanceDebug => "Camera distance debug",
        }
    }

    pub fn uses_plateau_curve(self) -> bool {
        matches!(self, Self::MapRadiusPlateau)
    }

    pub fn gpu_falloff_mode(self) -> f32 {
        match self {
            Self::MapRadiusPlateau => crate::falloff_metric::FALLOFF_MODE_MAP_RADIUS,
            Self::VisualHorizon => crate::falloff_metric::FALLOFF_MODE_VISUAL_HORIZON,
            Self::CameraDistanceDebug => crate::falloff_metric::FALLOFF_MODE_CAMERA_DISTANCE,
        }
    }
}

/// Project a world anchor to viewport pixel coordinates (y down).
pub fn world_anchor_screen_px(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    anchor: Vec3,
    viewport_width: f32,
    viewport_height: f32,
) -> Option<[f32; 2]> {
    let ndc = camera.world_to_ndc(camera_transform, anchor)?;
    Some([
        (ndc.x * 0.5 + 0.5) * viewport_width,
        (1.0 - ndc.y) * 0.5 * viewport_height,
    ])
}

/// Visual progress along the foreground (bottom center) → high horizon ruler.
pub fn visual_horizon_falloff_progress_percent(
    screen_px: [f32; 2],
    ruler: &VisualHorizonFalloffRuler,
) -> f32 {
    let dx = ruler.vanishing_px[0] - ruler.base_px[0];
    let dy = ruler.vanishing_px[1] - ruler.base_px[1];
    let len_sq = dx * dx + dy * dy;
    if len_sq <= f32::EPSILON {
        return 100.0;
    }
    let sx = screen_px[0] - ruler.base_px[0];
    let sy = screen_px[1] - ruler.base_px[1];
    let progress = (sx * dx + sy * dy) / len_sq;
    (progress.clamp(0.0, 1.0)) * 100.0
}

/// Screen pixel on the visual high-horizon ruler at the given progress percent.
pub fn visual_horizon_ruler_point_at_progress_percent(
    ruler: &VisualHorizonFalloffRuler,
    progress_percent: f32,
) -> [f32; 2] {
    let t = (progress_percent / 100.0).clamp(0.0, 1.0);
    [
        ruler.base_px[0] + (ruler.vanishing_px[0] - ruler.base_px[0]) * t,
        ruler.base_px[1] + (ruler.vanishing_px[1] - ruler.base_px[1]) * t,
    ]
}

/// Screen Y as a fraction from the top (0 = top edge, 1 = bottom edge).
pub fn visual_horizon_ruler_screen_y_fraction_from_top(progress_percent: f32) -> f32 {
    let t = (progress_percent / 100.0).clamp(0.0, 1.0);
    STAR_FALLOFF_BASE_Y_FRACTION
        + (STAR_FALLOFF_VANISHING_Y_FRACTION - STAR_FALLOFF_BASE_Y_FRACTION) * t
}

/// Shared star/nameplate falloff progress.
pub fn star_falloff_progress_percent(
    metric: StarFalloffMetric,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    anchor: Vec3,
    camera_distance: f32,
    star_settings: &StarBillboardRenderSettings,
    viewport_width: f32,
    viewport_height: f32,
    map_context: Option<&StudioMapRadiusFalloffContext>,
) -> f32 {
    match metric {
        StarFalloffMetric::MapRadiusPlateau => map_context
            .map(|ctx| world_position_map_progress_percent(ctx, anchor.to_array()))
            .unwrap_or(100.0),
        StarFalloffMetric::VisualHorizon => world_anchor_screen_px(
            camera,
            camera_transform,
            anchor,
            viewport_width,
            viewport_height,
        )
        .map(|px| {
            visual_horizon_falloff_progress_percent(
                px,
                &VisualHorizonFalloffRuler::from_viewport(viewport_width, viewport_height),
            )
        })
        .unwrap_or(100.0),
        StarFalloffMetric::CameraDistanceDebug => {
            normalized_billboard_camera_depth_percent(camera_distance, star_settings)
        }
    }
}

/// Star opacity falloff alpha at the given progress (mirrors GPU star ceiling).
pub fn star_falloff_alpha_at_progress(
    progress_percent: f32,
    billboard: &WorldTextBillboard,
    use_plateau: bool,
) -> f32 {
    if use_plateau {
        return world_text_plateau_falloff(
            progress_percent,
            billboard.ceiling_falloff_percent,
            billboard.ceiling_target_alpha,
        );
    }
    world_text_distance_falloff(
        progress_percent,
        billboard.ceiling_falloff_percent,
        billboard.ceiling_target_alpha,
        billboard.horizon_taper,
    )
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
    progress_percent: f32,
    settings: StarFalloffSettings,
    use_plateau: bool,
) -> StarFalloffVisual {
    let settings = settings.clamped();
    let target_blur = settings.base_blur_radius * settings.falloff_blur_radius_percent / 100.0;
    let target_opacity = settings.falloff_opacity_percent / 100.0;
    if use_plateau {
        let t = plateau_falloff_t_percent(progress_percent, settings.falloff_distance_percent);
        return StarFalloffVisual {
            blur_radius: lerp(settings.base_blur_radius, target_blur, t),
            opacity: lerp(1.0, target_opacity, t),
        };
    }
    let depth = progress_percent.clamp(0.0, 100.0);
    let falloff_at = settings.falloff_distance_percent;
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
    let horizon = compute_star_falloff_visual(100.0, settings, true);
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
