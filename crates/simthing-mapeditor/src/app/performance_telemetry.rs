//! Bevy systems for Studio Settings performance telemetry.

use bevy::app::{App, Plugin};
use bevy::diagnostic::{DiagnosticsStore, FrameCount};
use bevy::prelude::*;
use bevy::render::{renderer::RenderAdapterInfo, RenderApp};

use crate::studio_performance_telemetry::{
    bytes_to_vram_mb, estimate_studio_allocated_vram_bytes, read_fps_from_diagnostics,
    StudioPerformanceTelemetry,
};

const VRAM_ESTIMATE_INTERVAL_SECS: f32 = 0.5;

/// Cached VRAM scan scheduling state (presentation only).
#[derive(Resource, Default)]
pub struct StudioPerformanceTelemetryState {
    pub telemetry: StudioPerformanceTelemetry,
    pub vram_dirty: bool,
    last_vram_scan_elapsed_secs: f32,
}

pub fn init_studio_performance_telemetry(mut commands: Commands) {
    commands.init_resource::<StudioPerformanceTelemetryState>();
}

pub fn update_studio_fps_telemetry(
    diagnostics: Res<DiagnosticsStore>,
    frame_count: Res<FrameCount>,
    mut state: ResMut<StudioPerformanceTelemetryState>,
) {
    state.telemetry.fps = read_fps_from_diagnostics(&diagnostics);
    state.telemetry.last_update_frame = frame_count.0 as u64;
}

pub fn update_studio_vram_telemetry(
    time: Res<Time>,
    images: Res<Assets<Image>>,
    meshes: Res<Assets<Mesh>>,
    mut state: ResMut<StudioPerformanceTelemetryState>,
) {
    let elapsed = time.elapsed_secs();
    let due = state.vram_dirty
        || state.last_vram_scan_elapsed_secs == 0.0
        || elapsed - state.last_vram_scan_elapsed_secs >= VRAM_ESTIMATE_INTERVAL_SECS;
    if !due {
        return;
    }

    let started = std::time::Instant::now();
    let (total, texture_bytes, mesh_bytes, buffer_bytes) =
        estimate_studio_allocated_vram_bytes(&images, &meshes);
    state.telemetry.allocated_vram_bytes_estimate = total;
    state.telemetry.allocated_vram_mb_estimate = bytes_to_vram_mb(total);
    state.telemetry.texture_bytes_estimate = texture_bytes;
    state.telemetry.mesh_bytes_estimate = mesh_bytes;
    state.telemetry.buffer_bytes_estimate = buffer_bytes;
    state.last_vram_scan_elapsed_secs = elapsed;
    state.vram_dirty = false;
    state.telemetry.vram_scan_last_ms = Some(started.elapsed().as_secs_f64() * 1000.0);
}

/// Copies render-subapp adapter identity into main-world telemetry after renderer init.
pub struct StudioGpuIdentityInitPlugin;

impl Plugin for StudioGpuIdentityInitPlugin {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, app: &mut App) {
        let gpu_identity = app.get_sub_app(RenderApp).and_then(|render_sub_app| {
            render_sub_app
                .world()
                .get_resource::<RenderAdapterInfo>()
                .map(|adapter_info| {
                    (
                        adapter_info.name.clone(),
                        format!("{:?}", adapter_info.backend),
                    )
                })
        });
        let Some((name, backend)) = gpu_identity else {
            return;
        };
        let Some(mut state) = app
            .world_mut()
            .get_resource_mut::<StudioPerformanceTelemetryState>()
        else {
            return;
        };
        state.telemetry.gpu_name = Some(name);
        state.telemetry.gpu_backend = Some(backend);
    }
}
