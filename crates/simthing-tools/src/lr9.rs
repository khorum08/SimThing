//! LR9 final typeface performance gate — structured scenario profiles over the Bevy runtime.

use std::time::Instant;

use bevy::prelude::*;

use crate::{
    bevy::{
        profile_bevy_fixed_width_numeric_damage_bench, profile_bevy_text_bench,
        spawn_static_and_numeric_damage_labels, spawn_static_text_labels, text_deform_diagnostics,
        text_label_entity_counts, text_path_warp_diagnostics, text_perf_diagnostics,
        text_style_diagnostics, BevyTextBenchProfile, SimthingToolsTextPlugin, TextLabel,
        TextPerfDiagnostics,
    },
    deform::{TextDeformParams, TextDeformTableResource},
    path::{TextPathParams, TextPathTableResource},
    studio_labels::{
        studio_typeface_label_diagnostics, StudioDamageTextEmitter, StudioTypefaceLabel,
        StudioTypefaceLabelPlugin,
    },
    style::{TextStyleRow, TextStyleTableResource, GRADIENT_MODE_LINEAR_U},
    text_render::{
        text_atlas_render_diagnostics, text_deform_render_diagnostics,
        text_path_warp_render_diagnostics, text_style_render_diagnostics,
    },
    warp::{TextWarpParams, TextWarpTableResource},
};

const FIXTURE_FONT: &[u8] = include_bytes!("../../simthing-workshop/assets/typeface/test_font.ttf");

pub const LR9_ATLAS_SIZE: u32 = 4096;
pub const LR9_LABEL_PX: f32 = 24.0;

/// CI-friendly smoke scale (structural + budget smoke under ~30s).
pub const LR9_CI_CONFIG: Lr9Config = Lr9Config {
    flat_labels: 1_000,
    numeric_damage_labels: 100,
    styled_labels: 256,
    warped_labels: 64,
    studio_labels: 32,
    noop_frames: 20,
    damage_frames: 10,
    atlas_size: LR9_ATLAS_SIZE,
};

/// Binding-scale profile for manual `#[ignore]` runs.
pub const LR9_BINDING_CONFIG: Lr9Config = Lr9Config {
    flat_labels: 5_000,
    numeric_damage_labels: 5_000,
    styled_labels: 512,
    warped_labels: 256,
    studio_labels: 64,
    noop_frames: 60,
    damage_frames: 60,
    atlas_size: LR9_ATLAS_SIZE,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lr9Config {
    pub flat_labels: usize,
    pub numeric_damage_labels: usize,
    pub styled_labels: usize,
    pub warped_labels: usize,
    pub studio_labels: usize,
    pub noop_frames: usize,
    pub damage_frames: usize,
    pub atlas_size: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lr9MetricsSnapshot {
    pub validation_host: String,
    pub adapter: String,
    pub total_labels: usize,
    pub total_glyph_instances: u64,
    pub tessellated_vertex_count: u64,
    pub style_row_count: u64,
    pub path_row_count: u64,
    pub warp_row_count: u64,
    pub atlas_bind_group_create_count: u64,
    pub atlas_bind_group_reuse_count: u64,
    pub style_buffer_create_count: u64,
    pub style_buffer_write_count: u64,
    pub style_bind_group_create_count: u64,
    pub style_bind_group_reuse_count: u64,
    pub deform_buffer_create_count: u64,
    pub deform_buffer_write_count: u64,
    pub path_buffer_create_count: u64,
    pub path_buffer_write_count: u64,
    pub warp_buffer_create_count: u64,
    pub warp_buffer_write_count: u64,
    pub shape_rebuild_count: u64,
    pub raster_generation_count: u64,
    pub msdf_generation_count: u64,
    pub instance_rebuild_count: u64,
    pub queued_draw_count: u64,
    pub queued_instance_count: u64,
    pub manifest_reload_count: u64,
    pub runtime_svg_parse_count: u64,
    pub bespoke_text_fallback_count: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lr9ScenarioProfile {
    pub scenario: &'static str,
    pub config: Lr9Config,
    pub metrics: Lr9MetricsSnapshot,
    pub avg_noop_update_ms: f64,
    pub max_noop_update_ms: f64,
    pub avg_changed_update_ms: f64,
    pub max_changed_update_ms: f64,
    pub perf: TextPerfDiagnostics,
}

pub fn fixture_font_bytes() -> Vec<u8> {
    FIXTURE_FONT.to_vec()
}

pub fn validation_host_label() -> String {
    format!(
        "{} {} {}",
        std::env::consts::OS,
        std::env::consts::ARCH,
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "unknown-cpu".into())
    )
}

pub fn adapter_label() -> String {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    });
    pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: None,
    }))
    .map(|adapter| {
        let info = adapter.get_info();
        format!("REAL_ADAPTER_OBSERVED: {}", info.name)
    })
    .unwrap_or_else(|| "ADAPTER_SKIPPED".into())
}

pub fn lr9_cpu_bevy_app(atlas_size: u32) -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .add_plugins(SimthingToolsTextPlugin::with_atlas_size(
            fixture_font_bytes(),
            atlas_size,
        ));
    app
}

pub fn lr9_studio_shell_app(atlas_size: u32) -> App {
    let mut app = lr9_cpu_bevy_app(atlas_size);
    app.add_plugins(StudioTypefaceLabelPlugin);
    app
}

fn clear_app_exit(app: &mut App) {
    if let Some(mut exits) = app.world_mut().get_resource_mut::<Events<AppExit>>() {
        exits.clear();
    }
}

pub fn lr9_timed_updates(app: &mut App, frames: usize) -> (f64, f64) {
    if frames == 0 {
        return (0.0, 0.0);
    }
    let mut total_ms = 0.0_f64;
    let mut max_ms = 0.0_f64;
    for _ in 0..frames {
        clear_app_exit(app);
        let start = Instant::now();
        app.update();
        let elapsed = start.elapsed().as_secs_f64() * 1000.0;
        total_ms += elapsed;
        max_ms = max_ms.max(elapsed);
    }
    (total_ms / frames as f64, max_ms)
}

pub fn install_dynamic_style_rows(app: &mut App) {
    let mut style = app.world_mut().resource_mut::<TextStyleTableResource>();
    style
        .set_row(
            1,
            TextStyleRow::solid_fill(1.0, 0.0, 0.0, 1.0)
                .with_outline([0.0, 0.0, 0.0, 1.0], 1.5)
                .with_glow([1.0, 0.4, 0.1, 0.8], 4.0),
        )
        .expect("style slot 1");
    style
        .set_row(
            2,
            TextStyleRow::linear_gradient_u([1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 1.0, 1.0])
                .with_pulse(0.25, 2.0, 0.0),
        )
        .expect("style slot 2");
    style
        .set_row(
            3,
            TextStyleRow {
                fill_rgba: [0.2, 0.8, 0.3, 1.0],
                accent_rgba: [0.9, 0.9, 0.2, 1.0],
                gradient_mode: GRADIENT_MODE_LINEAR_U,
                glow_radius: 6.0,
                glow_rgba: [0.2, 1.0, 0.4, 0.7],
                pulse_amplitude: 0.15,
                pulse_frequency: 1.5,
                ..Default::default()
            },
        )
        .expect("style slot 3");
}

pub fn install_warp_tables(app: &mut App) {
    {
        let mut deform = app.world_mut().resource_mut::<TextDeformTableResource>();
        deform
            .set_row(1, TextDeformParams::skew(0.25, 0.0))
            .expect("deform slot 1");
    }
    {
        let mut path = app.world_mut().resource_mut::<TextPathTableResource>();
        path.set_row(1, TextPathParams::arc([80.0, 90.0], [180.0, 90.0], 50.0))
            .expect("path slot 1");
    }
    {
        let mut warp = app.world_mut().resource_mut::<TextWarpTableResource>();
        warp.set_row(
            1,
            TextWarpParams::lattice2x2(0.35, [[0.0, 0.0], [24.0, 0.0], [0.0, 18.0], [24.0, 18.0]]),
        )
        .expect("warp slot 1");
    }
}

pub fn spawn_styled_labels(app: &mut App, count: usize) {
    install_dynamic_style_rows(app);
    let world = app.world_mut();
    for index in 0..count {
        let slot = (index % 3 + 1) as u16;
        world.spawn(
            TextLabel::raster(
                format!("Styled {index}"),
                LR9_LABEL_PX,
                [1.0, 1.0, 1.0, 1.0],
            )
            .with_style_slot(slot),
        );
    }
}

pub fn spawn_warped_nameplate_labels(app: &mut App, count: usize) {
    install_warp_tables(app);
    let world = app.world_mut();
    for index in 0..count {
        world.spawn(
            TextLabel::raster(
                format!("Empire {index}"),
                LR9_LABEL_PX,
                [0.92, 0.94, 1.0, 1.0],
            )
            .with_style_slot(2)
            .with_deform_slot(1)
            .with_path_slot(1)
            .with_warp_slot(1),
        );
    }
}

pub fn spawn_studio_seam_labels(app: &mut App, count: usize) {
    let world = app.world_mut();
    for index in 0..count {
        let mut label =
            StudioTypefaceLabel::entity_name(format!("World {index}"), LR9_LABEL_PX, [1.0; 4]);
        if index % 4 == 0 {
            label = label.with_icon_name("test.background-accent");
        }
        world.spawn(label);
    }
    world.spawn(StudioDamageTextEmitter::default());
}

pub fn collect_lr9_metrics(app: &App, total_labels: usize) -> Lr9MetricsSnapshot {
    let perf = text_perf_diagnostics(app);
    let style = text_style_diagnostics(app);
    let deform = text_deform_diagnostics(app);
    let path_warp = text_path_warp_diagnostics(app);
    let studio = studio_typeface_label_diagnostics(app);
    let atlas = text_atlas_render_diagnostics(app);
    let style_render = text_style_render_diagnostics(app);
    let deform_render = text_deform_render_diagnostics(app);
    let path_render = text_path_warp_render_diagnostics(app);
    let df = app
        .world()
        .get_resource::<crate::bevy::TypefaceAtlas>()
        .map(|atlas| atlas.distance_field_diagnostics())
        .unwrap_or_default();

    Lr9MetricsSnapshot {
        validation_host: validation_host_label(),
        adapter: adapter_label(),
        total_labels,
        total_glyph_instances: perf
            .extracted_instance_count
            .max(perf.queued_instance_count),
        tessellated_vertex_count: deform.tessellated_vertex_count,
        style_row_count: style.style_table_upload_count,
        path_row_count: path_warp.path_table_upload_count,
        warp_row_count: path_warp.warp_table_upload_count,
        atlas_bind_group_create_count: atlas.atlas_bind_group_create_count,
        atlas_bind_group_reuse_count: atlas.atlas_bind_group_reuse_count,
        style_buffer_create_count: style_render.globals_buffer_create_count
            + style_render.rows_buffer_create_count,
        style_buffer_write_count: style_render.globals_buffer_write_count
            + style_render.rows_buffer_write_count,
        style_bind_group_create_count: style_render.style_bind_group_create_count,
        style_bind_group_reuse_count: style_render.style_bind_group_reuse_count,
        deform_buffer_create_count: deform_render.rows_buffer_create_count,
        deform_buffer_write_count: deform_render.rows_buffer_write_count,
        path_buffer_create_count: path_render.path_buffer_create_count,
        path_buffer_write_count: path_render.path_buffer_write_count,
        warp_buffer_create_count: path_render.warp_buffer_create_count,
        warp_buffer_write_count: path_render.warp_buffer_write_count,
        shape_rebuild_count: perf.shape_rebuild_count,
        raster_generation_count: app
            .world()
            .get_resource::<crate::bevy::TypefaceAtlas>()
            .map(|a| a.cpu_stats().rasterize_count)
            .unwrap_or(0),
        msdf_generation_count: df.glyph_msdf_generate_count + df.icon_msdf_generate_count,
        instance_rebuild_count: perf.instance_rebuild_count,
        queued_draw_count: perf.queued_draw_count,
        queued_instance_count: perf.queued_instance_count,
        manifest_reload_count: studio.manifest_reload_count,
        runtime_svg_parse_count: studio.runtime_svg_parse_count,
        bespoke_text_fallback_count: studio.bespoke_text_fallback_count,
    }
}

fn profile_from_bevy(
    scenario: &'static str,
    config: Lr9Config,
    app: &mut App,
    bevy: BevyTextBenchProfile,
    total_labels: usize,
) -> Lr9ScenarioProfile {
    Lr9ScenarioProfile {
        scenario,
        config,
        metrics: collect_lr9_metrics(app, total_labels),
        avg_noop_update_ms: bevy.avg_noop_update_ms,
        max_noop_update_ms: bevy.max_noop_update_ms,
        avg_changed_update_ms: bevy.avg_damage_update_ms,
        max_changed_update_ms: bevy.max_damage_update_ms,
        perf: bevy.diagnostics_after_noop,
    }
}

pub fn profile_flat_animated_labels(config: Lr9Config) -> Lr9ScenarioProfile {
    let mut app = lr9_cpu_bevy_app(config.atlas_size);
    spawn_static_text_labels(&mut app, config.flat_labels, LR9_LABEL_PX);
    let total = config.flat_labels;
    let damage: Vec<Entity> = Vec::new();
    let bevy = profile_bevy_text_bench(&mut app, &damage, config.noop_frames, 0);
    profile_from_bevy("A_flat_5k_animated", config, &mut app, bevy, total)
}

pub fn profile_numeric_damage_lane(config: Lr9Config) -> Lr9ScenarioProfile {
    let mut app = lr9_cpu_bevy_app(config.atlas_size);
    let damage = spawn_static_and_numeric_damage_labels(
        &mut app,
        0,
        config.numeric_damage_labels,
        LR9_LABEL_PX,
    );
    let total = config.numeric_damage_labels;
    let bevy = profile_bevy_fixed_width_numeric_damage_bench(
        &mut app,
        &damage,
        config.noop_frames,
        config.damage_frames,
    );
    profile_from_bevy("B_numeric_damage_5k", config, &mut app, bevy, total)
}

pub fn profile_dynamic_style_labels(config: Lr9Config) -> Lr9ScenarioProfile {
    let mut app = lr9_cpu_bevy_app(config.atlas_size);
    spawn_styled_labels(&mut app, config.styled_labels);
    clear_app_exit(&mut app);
    app.update();
    let upload_before = text_style_diagnostics(&app).style_table_upload_count;
    let (avg_noop, max_noop) = lr9_timed_updates(&mut app, config.noop_frames);
    let upload_after_noop = text_style_diagnostics(&app).style_table_upload_count;
    debug_assert_eq!(
        upload_after_noop, upload_before,
        "style rows must not reupload on noop frames"
    );

    let style_gen_before = app
        .world()
        .get_resource::<TextStyleTableResource>()
        .map(|r| r.rows_generation)
        .unwrap_or(0);
    {
        let mut style = app.world_mut().resource_mut::<TextStyleTableResource>();
        style
            .set_row(4, TextStyleRow::solid_fill(0.0, 1.0, 0.0, 1.0))
            .expect("slot 4");
    }
    clear_app_exit(&mut app);
    app.update();
    let style_gen_after = app
        .world()
        .get_resource::<TextStyleTableResource>()
        .map(|r| r.rows_generation)
        .unwrap_or(0);
    let upload_after_change = text_style_diagnostics(&app).style_table_upload_count;

    let (labels, _) = text_label_entity_counts(&mut app);
    assert!(style_gen_after > style_gen_before);
    assert!(upload_after_change > upload_after_noop);

    Lr9ScenarioProfile {
        scenario: "C_dynamic_style",
        config,
        metrics: collect_lr9_metrics(&app, labels),
        avg_noop_update_ms: avg_noop,
        max_noop_update_ms: max_noop,
        avg_changed_update_ms: avg_noop,
        max_changed_update_ms: max_noop,
        perf: text_perf_diagnostics(&app),
    }
}

pub fn profile_warped_nameplates(config: Lr9Config) -> Lr9ScenarioProfile {
    let mut app = lr9_cpu_bevy_app(config.atlas_size);
    spawn_warped_nameplate_labels(&mut app, config.warped_labels);
    clear_app_exit(&mut app);
    app.update();
    let rebuild_before = text_path_warp_diagnostics(&app).path_warp_rebuild_count;
    let (avg_noop, max_noop) = lr9_timed_updates(&mut app, config.noop_frames);
    let rebuild_after_noop = text_path_warp_diagnostics(&app).path_warp_rebuild_count;

    {
        let mut path = app.world_mut().resource_mut::<TextPathTableResource>();
        path.set_row(
            2,
            TextPathParams::quadratic_bezier([20.0, 40.0], [120.0, 10.0], [220.0, 40.0]),
        )
        .expect("path slot 2");
    }
    app.world_mut().spawn(
        TextLabel::raster("Changed", LR9_LABEL_PX, [1.0, 1.0, 1.0, 1.0])
            .with_path_slot(2)
            .with_deform_slot(1),
    );
    clear_app_exit(&mut app);
    let start = Instant::now();
    app.update();
    let changed_ms = start.elapsed().as_secs_f64() * 1000.0;
    let rebuild_after_change = text_path_warp_diagnostics(&app).path_warp_rebuild_count;

    let (labels, _) = text_label_entity_counts(&mut app);
    assert_eq!(rebuild_after_noop, rebuild_before);
    assert!(rebuild_after_change >= rebuild_before);

    Lr9ScenarioProfile {
        scenario: "D_warped_nameplates",
        config,
        metrics: collect_lr9_metrics(&app, labels),
        avg_noop_update_ms: avg_noop,
        max_noop_update_ms: max_noop,
        avg_changed_update_ms: changed_ms,
        max_changed_update_ms: changed_ms,
        perf: text_perf_diagnostics(&app),
    }
}

pub fn profile_studio_seam_labels(config: Lr9Config) -> Lr9ScenarioProfile {
    let mut app = lr9_studio_shell_app(config.atlas_size);
    spawn_studio_seam_labels(&mut app, config.studio_labels);
    clear_app_exit(&mut app);
    app.update();
    let emitter_entity = {
        let world = app.world_mut();
        let mut q = world.query_filtered::<Entity, With<StudioDamageTextEmitter>>();
        q.iter(world).next()
    };
    if let Some(entity) = emitter_entity {
        app.world_mut()
            .entity_mut(entity)
            .get_mut::<StudioDamageTextEmitter>()
            .expect("emitter")
            .emit(42);
    }
    let (avg_noop, max_noop) = lr9_timed_updates(&mut app, config.noop_frames);
    let (labels, draw_entities) = text_label_entity_counts(&mut app);
    assert!(draw_entities <= 1, "bounded draw entity count");
    Lr9ScenarioProfile {
        scenario: "E_studio_seam",
        config,
        metrics: collect_lr9_metrics(&app, labels),
        avg_noop_update_ms: avg_noop,
        max_noop_update_ms: max_noop,
        avg_changed_update_ms: 0.0,
        max_changed_update_ms: 0.0,
        perf: text_perf_diagnostics(&app),
    }
}

pub fn format_lr9_scenario_report(profile: &Lr9ScenarioProfile) -> String {
    format!(
        "scenario={} labels={} avg_noop_ms={:.4} max_noop_ms={:.4} avg_changed_ms={:.4} max_changed_ms={:.4} \
shape_rebuild={} instance_rebuild={} tess_vertices={} style_uploads={} path_uploads={} warp_uploads={} \
draws={} instances={} adapter={}",
        profile.scenario,
        profile.metrics.total_labels,
        profile.avg_noop_update_ms,
        profile.max_noop_update_ms,
        profile.avg_changed_update_ms,
        profile.max_changed_update_ms,
        profile.metrics.shape_rebuild_count,
        profile.metrics.instance_rebuild_count,
        profile.metrics.tessellated_vertex_count,
        profile.metrics.style_row_count,
        profile.metrics.path_row_count,
        profile.metrics.warp_row_count,
        profile.metrics.queued_draw_count,
        profile.metrics.queued_instance_count,
        profile.metrics.adapter,
    )
}
