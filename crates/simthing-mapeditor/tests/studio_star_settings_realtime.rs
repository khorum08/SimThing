//! STUDIO-PERFORMANCE-TELEMETRY-WINDOW-0 — #868 follow-up: settings change mutates visual with camera fixed.

use simthing_mapeditor::star_render::{
    compute_star_distance_visual, StarBillboardRenderSettings, StarRenderMode,
};
use simthing_mapeditor::{
    star_visual_per_star_should_write, StarVisualAppliedKey, StarVisualSyncCacheState,
};

fn billboard_settings(
    base_star_blur_radius: f32,
    falloff_star_opacity_percent: f32,
) -> StarBillboardRenderSettings {
    StarBillboardRenderSettings {
        base_star_blur_radius,
        falloff_distance_percent: 50.0,
        falloff_star_blur_radius_percent: 10.0,
        falloff_star_opacity_percent,
        near_distance: 10.0,
        far_horizon_distance: 110.0,
        selected_star_scale_multiplier: 1.85,
        hovered_star_scale_multiplier: 1.22,
        far_core_scale: 0.1,
        near_core_scale: 0.68,
        near_core_alpha: 1.0,
        near_aura_alpha: 0.22,
        render_mode: StarRenderMode::BloomStarburst,
    }
}

#[test]
fn settings_star_render_change_mutates_visual_with_camera_fixed() {
    let applied_key = StarVisualAppliedKey {
        selected: false,
        hovered: false,
        render_mode: StarRenderMode::BloomStarburst,
        depth_bucket_or_quantized_percent: 500,
        layer: 0,
    };
    let visual_key = applied_key;

    // #868 failure mode: outer dirty + matching inner keys must still write once.
    assert!(
        star_visual_per_star_should_write(true, applied_key, visual_key),
        "force_resync must bypass per-star skip when keys match"
    );
    assert!(
        !star_visual_per_star_should_write(false, applied_key, visual_key),
        "steady-state frames keep the cheap per-star skip"
    );

    let mut cache = StarVisualSyncCacheState::default();
    cache.dirty = true;
    let force_resync = cache.dirty;
    assert!(force_resync);

    let base_settings = billboard_settings(0.25, 10.0);
    let changed_settings = billboard_settings(0.85, 80.0);

    let camera_depth_percent = 42.0;
    let visual_before =
        compute_star_distance_visual(camera_depth_percent, false, false, &base_settings, true);
    let visual_after =
        compute_star_distance_visual(camera_depth_percent, false, false, &changed_settings, true);

    assert_ne!(
        visual_before.core_scale, visual_after.core_scale,
        "settings change must alter computed star visual at fixed camera depth"
    );
    assert!(
        visual_after.core_alpha > visual_before.core_alpha,
        "settings change must alter star material alpha at fixed camera depth"
    );
}
