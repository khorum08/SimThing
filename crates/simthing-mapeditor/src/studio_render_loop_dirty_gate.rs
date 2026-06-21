//! STUDIO-RENDER-LOOP-DIRTY-GATE-0 — render-loop dirty gates and telemetry helpers.

use bevy::prelude::Component;

use crate::hyperlane_buckets::HyperlaneRenderSettings;
use crate::star_render::{StarFalloffSettings, StarRenderMode};

/// Quantized camera key for hyperlane mesh rebuild decisions (presentation only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HyperlaneCameraKey {
    pub position: [i32; 3],
    pub right: [i32; 3],
    pub up: [i32; 3],
    pub view_mode: u8,
}

/// Exact hyperlane render settings key (clamped slider precision).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HyperlaneRenderSettingsKey {
    pub base_thickness_percent_of_star: i32,
    pub base_opacity_percent: i32,
    pub falloff_distance_percent: i32,
    pub falloff_thickness_percent: i32,
    pub falloff_opacity_percent: i32,
}

/// Combined render-loop cache resources for Bevy system param limits.
#[derive(Debug, Clone, Default, bevy::prelude::Resource)]
pub struct StudioRenderLoopCaches {
    pub hyperlane: HyperlaneRenderCacheState,
    pub star_visual: StarVisualSyncCacheState,
    pub billboard: BillboardSyncCacheState,
    pub picking: PickingProjectionCacheState,
}

/// Cache state for hyperlane mesh rebuild dirty gating.
#[derive(Debug, Clone, Default)]
pub struct HyperlaneRenderCacheState {
    pub dirty: bool,
    pub last_camera_key: Option<HyperlaneCameraKey>,
    pub last_render_settings_key: Option<HyperlaneRenderSettingsKey>,
    pub last_view_model_generation: u64,
}

/// Global star-visual sync key (camera + selection + settings + session generation).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StarVisualSyncKey {
    pub camera_position: [i32; 3],
    pub selected_system_id: Option<u32>,
    pub hovered_system_id: Option<u32>,
    pub render_mode: StarRenderMode,
    pub falloff_settings: StarFalloffSettingsKey,
    pub view_model_generation: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StarFalloffSettingsKey {
    pub base_blur_radius: i32,
    pub falloff_distance_percent: i32,
    pub falloff_opacity_percent: i32,
}

/// Per-star applied visual key to skip redundant material writes.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct StarVisualAppliedKey {
    pub selected: bool,
    pub hovered: bool,
    pub render_mode: StarRenderMode,
    pub depth_bucket_or_quantized_percent: u16,
    pub layer: u8,
}

/// Cache state for star visual material/scale sync dirty gating.
#[derive(Debug, Clone, Default)]
pub struct StarVisualSyncCacheState {
    pub dirty: bool,
    pub last_sync_key: Option<StarVisualSyncKey>,
}

/// Quantized camera position key for billboard orientation updates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BillboardCameraKey {
    pub position: [i32; 3],
}

#[derive(Debug, Clone, Default)]
pub struct BillboardSyncCacheState {
    pub last_camera_key: Option<BillboardCameraKey>,
}

/// Picking projection cache key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PickingProjectionKey {
    pub camera_position: [i32; 3],
    pub camera_rotation: [i32; 4],
    pub window_width: u32,
    pub window_height: u32,
    pub anchor_count: usize,
    pub view_model_generation: u64,
}

#[derive(Debug, Clone, Default)]
pub struct PickingProjectionCacheState {
    pub last_key: Option<PickingProjectionKey>,
    pub cached_projections: Vec<crate::selection::ScreenStarProjection>,
}

const POSITION_QUANTUM: f32 = 0.5;
const DIRECTION_QUANTUM: f32 = 0.01;
const ROTATION_QUANTUM: f32 = 0.01;

fn quantize_axis(value: f32, quantum: f32) -> i32 {
    (value / quantum).round() as i32
}

fn quantize_position(position: [f32; 3]) -> [i32; 3] {
    [
        quantize_axis(position[0], POSITION_QUANTUM),
        quantize_axis(position[1], POSITION_QUANTUM),
        quantize_axis(position[2], POSITION_QUANTUM),
    ]
}

fn quantize_direction(direction: [f32; 3]) -> [i32; 3] {
    [
        quantize_axis(direction[0], DIRECTION_QUANTUM),
        quantize_axis(direction[1], DIRECTION_QUANTUM),
        quantize_axis(direction[2], DIRECTION_QUANTUM),
    ]
}

pub fn hyperlane_render_settings_key(
    settings: HyperlaneRenderSettings,
) -> HyperlaneRenderSettingsKey {
    let s = settings.clamped();
    HyperlaneRenderSettingsKey {
        base_thickness_percent_of_star: quantize_axis(s.base_thickness_percent_of_star, 0.1),
        base_opacity_percent: quantize_axis(s.base_opacity_percent, 0.1),
        falloff_distance_percent: quantize_axis(s.falloff_distance_percent, 0.1),
        falloff_thickness_percent: quantize_axis(s.falloff_thickness_percent, 0.1),
        falloff_opacity_percent: quantize_axis(s.falloff_opacity_percent, 0.1),
    }
}

pub fn star_falloff_settings_key(settings: StarFalloffSettings) -> StarFalloffSettingsKey {
    let s = settings.clamped();
    StarFalloffSettingsKey {
        base_blur_radius: quantize_axis(s.base_blur_radius, 0.01),
        falloff_distance_percent: quantize_axis(s.falloff_distance_percent, 0.1),
        falloff_opacity_percent: quantize_axis(s.falloff_opacity_percent, 0.1),
    }
}

/// Build a quantized hyperlane camera key from world-space camera state.
pub fn quantize_hyperlane_camera_key(
    position: [f32; 3],
    right: [f32; 3],
    up: [f32; 3],
    view_mode: u8,
) -> HyperlaneCameraKey {
    HyperlaneCameraKey {
        position: quantize_position(position),
        right: quantize_direction(right),
        up: quantize_direction(up),
        view_mode,
    }
}

pub fn quantize_billboard_camera_key(position: [f32; 3]) -> BillboardCameraKey {
    BillboardCameraKey {
        position: quantize_position(position),
    }
}

pub fn quantize_star_depth_percent(depth_percent: f32) -> u16 {
    (depth_percent.clamp(0.0, 100.0) * 10.0).round() as u16
}

pub fn hyperlane_render_should_rebuild(
    previous_camera: Option<HyperlaneCameraKey>,
    current_camera: HyperlaneCameraKey,
    previous_settings: Option<HyperlaneRenderSettingsKey>,
    current_settings: HyperlaneRenderSettingsKey,
    previous_generation: u64,
    current_generation: u64,
    dirty: bool,
) -> bool {
    if dirty {
        return true;
    }
    if current_generation != previous_generation {
        return true;
    }
    if previous_camera != Some(current_camera) {
        return true;
    }
    if previous_settings != Some(current_settings) {
        return true;
    }
    false
}

pub fn star_visuals_should_sync(
    previous_key: Option<StarVisualSyncKey>,
    current_key: StarVisualSyncKey,
    dirty: bool,
) -> bool {
    if dirty {
        return true;
    }
    previous_key != Some(current_key)
}

pub fn billboard_should_sync(
    previous_key: Option<BillboardCameraKey>,
    current_key: BillboardCameraKey,
) -> bool {
    previous_key != Some(current_key)
}

pub fn picking_projection_should_rebuild(
    previous_key: Option<PickingProjectionKey>,
    current_key: PickingProjectionKey,
) -> bool {
    previous_key != Some(current_key)
}

pub fn quantize_picking_projection_key(
    camera_position: [f32; 3],
    camera_rotation: [f32; 4],
    window_width: u32,
    window_height: u32,
    anchor_count: usize,
    view_model_generation: u64,
) -> PickingProjectionKey {
    PickingProjectionKey {
        camera_position: quantize_position(camera_position),
        camera_rotation: [
            quantize_axis(camera_rotation[0], ROTATION_QUANTUM),
            quantize_axis(camera_rotation[1], ROTATION_QUANTUM),
            quantize_axis(camera_rotation[2], ROTATION_QUANTUM),
            quantize_axis(camera_rotation[3], ROTATION_QUANTUM),
        ],
        window_width,
        window_height,
        anchor_count,
        view_model_generation,
    }
}

/// Record a timing sample into last/avg millisecond fields (presentation only).
pub fn render_loop_telemetry_record_timing(
    last_ms: &mut Option<f64>,
    avg_ms: &mut Option<f64>,
    sample_ms: f64,
    sample_count: u64,
) {
    *last_ms = Some(sample_ms);
    match avg_ms {
        Some(avg) => {
            let n = sample_count.max(1) as f64;
            *avg = (*avg * (n - 1.0) + sample_ms) / n;
        }
        None => *avg_ms = Some(sample_ms),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_camera_key() -> HyperlaneCameraKey {
        HyperlaneCameraKey {
            position: [80, 70, 80],
            right: [100, 0, 0],
            up: [0, 100, 0],
            view_mode: 0,
        }
    }

    fn sample_settings_key() -> HyperlaneRenderSettingsKey {
        HyperlaneRenderSettingsKey {
            base_thickness_percent_of_star: 80,
            base_opacity_percent: 750,
            falloff_distance_percent: 1000,
            falloff_thickness_percent: 240,
            falloff_opacity_percent: 500,
        }
    }

    fn sample_star_sync_key() -> StarVisualSyncKey {
        StarVisualSyncKey {
            camera_position: [80, 70, 80],
            selected_system_id: None,
            hovered_system_id: None,
            render_mode: StarRenderMode::BloomStarburst,
            falloff_settings: StarFalloffSettingsKey {
                base_blur_radius: 11,
                falloff_distance_percent: 1000,
                falloff_opacity_percent: 700,
            },
            view_model_generation: 1,
        }
    }

    #[test]
    fn hyperlane_dirty_gate_skips_when_camera_and_settings_unchanged() {
        let camera = sample_camera_key();
        let settings = sample_settings_key();
        assert!(!hyperlane_render_should_rebuild(
            Some(camera),
            camera,
            Some(settings),
            settings,
            1,
            1,
            false,
        ));
    }

    #[test]
    fn hyperlane_dirty_gate_rebuilds_when_settings_change() {
        let camera = sample_camera_key();
        let prev = sample_settings_key();
        let mut next = prev;
        next.base_opacity_percent += 10;
        assert!(hyperlane_render_should_rebuild(
            Some(camera),
            camera,
            Some(prev),
            next,
            1,
            1,
            false,
        ));
    }

    #[test]
    fn hyperlane_dirty_gate_rebuilds_when_camera_key_changes() {
        let prev = sample_camera_key();
        let mut next = prev;
        next.position[0] += 1;
        let settings = sample_settings_key();
        assert!(hyperlane_render_should_rebuild(
            Some(prev),
            next,
            Some(settings),
            settings,
            1,
            1,
            false,
        ));
    }

    #[test]
    fn hyperlane_dirty_gate_rebuilds_when_session_changes() {
        let camera = sample_camera_key();
        let settings = sample_settings_key();
        assert!(hyperlane_render_should_rebuild(
            Some(camera),
            camera,
            Some(settings),
            settings,
            1,
            2,
            false,
        ));
    }

    #[test]
    fn star_visual_dirty_gate_skips_when_camera_selection_settings_unchanged() {
        let key = sample_star_sync_key();
        assert!(!star_visuals_should_sync(Some(key), key, false));
    }

    #[test]
    fn star_visual_dirty_gate_rebuilds_when_selection_changes() {
        let prev = sample_star_sync_key();
        let mut next = prev;
        next.selected_system_id = Some(42);
        assert!(star_visuals_should_sync(Some(prev), next, false));
    }

    #[test]
    fn star_visual_dirty_gate_rebuilds_when_hover_changes() {
        let prev = sample_star_sync_key();
        let mut next = prev;
        next.hovered_system_id = Some(7);
        assert!(star_visuals_should_sync(Some(prev), next, false));
    }

    #[test]
    fn star_visual_dirty_gate_rebuilds_when_render_settings_change() {
        let prev = sample_star_sync_key();
        let mut next = prev;
        next.render_mode = StarRenderMode::CrispCircle;
        assert!(star_visuals_should_sync(Some(prev), next, false));
    }

    #[test]
    fn quantize_hyperlane_camera_key_rounds_small_jitter() {
        let a =
            quantize_hyperlane_camera_key([40.1, 35.2, 40.3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], 0);
        let b =
            quantize_hyperlane_camera_key([40.2, 35.1, 40.4], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], 0);
        assert_eq!(a, b);
    }

    #[test]
    fn render_loop_telemetry_records_timing() {
        let mut last = None;
        let mut avg = None;
        render_loop_telemetry_record_timing(&mut last, &mut avg, 2.0, 1);
        render_loop_telemetry_record_timing(&mut last, &mut avg, 4.0, 2);
        assert_eq!(last, Some(4.0));
        assert!((avg.unwrap() - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn picking_projection_cache_reuses_when_camera_window_session_unchanged() {
        let key = quantize_picking_projection_key(
            [10.0, 20.0, 30.0],
            [0.0, 0.0, 0.0, 1.0],
            1920,
            1080,
            1000,
            5,
        );
        assert!(!picking_projection_should_rebuild(Some(key), key));
    }

    #[test]
    fn picking_projection_cache_rebuilds_when_camera_changes() {
        let prev = quantize_picking_projection_key(
            [10.0, 20.0, 30.0],
            [0.0, 0.0, 0.0, 1.0],
            1920,
            1080,
            1000,
            5,
        );
        let next = quantize_picking_projection_key(
            [50.0, 20.0, 30.0],
            [0.0, 0.0, 0.0, 1.0],
            1920,
            1080,
            1000,
            5,
        );
        assert!(picking_projection_should_rebuild(Some(prev), next));
    }

    #[test]
    fn picking_projection_cache_rebuilds_when_window_size_changes() {
        let prev = quantize_picking_projection_key(
            [10.0, 20.0, 30.0],
            [0.0, 0.0, 0.0, 1.0],
            1920,
            1080,
            1000,
            5,
        );
        let next = quantize_picking_projection_key(
            [10.0, 20.0, 30.0],
            [0.0, 0.0, 0.0, 1.0],
            1280,
            720,
            1000,
            5,
        );
        assert!(picking_projection_should_rebuild(Some(prev), next));
    }
}
