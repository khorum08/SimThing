use simthing_mapeditor::{
    FrostedGlassFrameTelemetry, FrostedGlassPanelRegistry, FrostedGlassRenderPlan,
    StudioScenarioLibraryModel, StudioSimClockTransport, StudioSimClockTransportCommand,
    FROSTED_GLASS_MAX_PANELS,
};

const MODULE: &str = include_str!("../src/studio_frosted_glass.rs");
const SHADER: &str = include_str!("../src/shaders/studio_frosted_glass.wgsl");
const UI: &str = include_str!("../src/app/ui.rs");

#[test]
fn frosted_glass_panel_frame_keeps_darkening_tint() {
    assert!(UI.contains("studio_panel_frame(opacity"));
    assert!(UI.contains("(opacity * 210.0) as u8"));
    assert!(UI.contains("from_rgba_unmultiplied"));
}

#[test]
fn frosted_glass_requires_real_blur_plan() {
    for entry in [
        "downsample",
        "blur_horizontal",
        "blur_vertical",
        "composite",
    ] {
        assert!(
            SHADER.contains(&format!("fn {entry}")),
            "missing {entry} pass"
        );
    }
    assert!(SHADER.contains("textureSample(auxiliary_texture"));
}

#[test]
fn frosted_glass_settings_dialog_uses_frosted_surface() {
    let settings = &UI
        [UI.find("fn draw_settings_dialog").unwrap()..UI.find("fn draw_telemetry_dialog").unwrap()];
    assert!(settings.contains("register_frosted_rect"));
    assert!(settings.contains("studio_panel_frame(0.82"));
}

#[test]
fn frosted_glass_telemetry_dialog_uses_frosted_surface() {
    let telemetry = &UI
        [UI.find("fn draw_telemetry_dialog").unwrap()..UI.find("fn draw_collapsed_tab").unwrap()];
    assert!(telemetry.contains("register_frosted_rect"));
    assert!(telemetry.contains("studio_panel_frame(0.82"));
}

#[test]
fn frosted_glass_left_panel_uses_frosted_surface() {
    let left = &UI
        [UI.find("fn draw_left_panel").unwrap()..UI.find("fn draw_sim_clock_transport").unwrap()];
    assert!(left.contains("register_frosted_rect"));
    assert!(left.contains("studio_panel_frame(opacity"));
}

#[test]
fn frosted_glass_modal_lifecycle_preserved() {
    let mut transport = StudioSimClockTransport::default();
    transport
        .apply(StudioSimClockTransportCommand::Play)
        .unwrap();
    let mut library = StudioScenarioLibraryModel::default();
    library.open(&mut transport);
    assert!(library.visible);
    assert!(transport.readout().paused);
    library.cancel(&mut transport);
    assert!(!library.visible);
    assert!(
        transport.readout().paused,
        "closing must not restore autoplay"
    );
    assert!(UI.contains("response.inner || response.should_close()"));
    assert!(UI.contains("clear_scenario_library_pending_actions(ctx)"));
}

#[test]
fn frosted_glass_single_shared_blur_target() {
    let plan = FrostedGlassRenderPlan::default();
    assert_eq!(plan.shared_target_count, 1);
    assert!(MODULE.contains("studio_frosted_glass_shared_blur_ping"));
    assert!(!MODULE.contains("Vec<FrostedGlassTextures>"));
}

#[test]
fn frosted_glass_downsampled_or_bounded_pass() {
    let plan = FrostedGlassRenderPlan::default();
    assert_eq!(plan.downsample_factor, 8);
    assert_eq!(plan.blur_pass_count, 2);
    assert!(plan.blur_radius_target_px <= 4.0);
    assert_eq!(FROSTED_GLASS_MAX_PANELS, 8);
    assert!(!SHADER.contains("textureLoad"));
}

#[test]
fn frosted_glass_records_before_after_frame_time() {
    let mut telemetry = FrostedGlassFrameTelemetry::default();
    for _ in 0..30 {
        assert!(!telemetry.record_frame_ms(8.0));
    }
    for _ in 0..60 {
        telemetry.record_frame_ms(10.0);
    }
    assert!(telemetry.effect_enabled());
    assert_eq!(telemetry.baseline_frame_ms, Some(10.0));
    for _ in 0..60 {
        telemetry.record_frame_ms(11.0);
    }
    assert_eq!(telemetry.frosted_frame_ms, Some(11.0));
}

#[test]
fn frosted_glass_no_spec_mutation() {
    let mut registry = FrostedGlassPanelRegistry::default();
    registry.register_logical_rect([10.0, 20.0], [110.0, 220.0], [1000.0, 800.0]);
    assert_eq!(registry.panel_count(), 1);
    assert!(!MODULE.contains("SimThingScenarioSpec"));
    assert!(!SHADER.contains("storage"));
}

#[test]
fn frosted_glass_no_gameplay_or_clock_semantics() {
    let mut transport = StudioSimClockTransport::default();
    transport
        .apply(StudioSimClockTransportCommand::Play)
        .unwrap();
    let before = transport.readout();
    let mut registry = FrostedGlassPanelRegistry::default();
    registry.begin_frame();
    let after = transport.readout();
    assert_eq!(before, after);
    assert!(!MODULE.contains("StudioSimClock"));
    assert!(!MODULE.contains("StudioSession"));
}
