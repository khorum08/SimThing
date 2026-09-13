//! B0b/OVL measurement scaffolding; remove with the 1.3 experiment.
//! Default off. Present-span QPCs are correlation BOUNDS, never DISPLAYED endpoints.
#![cfg(windows)]

use crate::app::StudioAppState;
use bevy::{
    diagnostic::FrameCount,
    input::{keyboard::KeyboardInput, ButtonState, InputSystem},
    log::{
        tracing,
        tracing_subscriber::{layer::Context, registry::Registry, Layer},
        BoxedLayer,
    },
    prelude::*,
    render::{Extract, ExtractSchedule, Render, RenderApp, RenderSet},
    window::PrimaryWindow,
};
use bevy_egui::{egui, EguiRenderOutput};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    fs::OpenOptions,
    io::Write,
    path::PathBuf,
    sync::{Arc, Mutex},
};

const INSTRUMENT: &str = "m16-displayed-probe-v1";
const CASES: [u32; 6] = [0, 4, 0, 4, 0, 4];
const MAX_FRAMES: usize = 50_000;
const MARKER_SIZE: f32 = 32.0;
const MARKER_TOP: f32 = 80.0;

#[link(name = "kernel32")]
extern "system" {
    fn QueryPerformanceCounter(value: *mut i64) -> i32;
    fn QueryPerformanceFrequency(value: *mut i64) -> i32;
}
fn qpc() -> Option<u64> {
    let mut value = 0i64;
    // SAFETY: Windows writes one i64 into this live, aligned output slot.
    let ok = unsafe { QueryPerformanceCounter(&mut value) };
    (ok != 0).then(|| u64::try_from(value).ok()).flatten()
}
fn qpc_frequency() -> Option<u64> {
    let mut value = 0i64;
    // SAFETY: the Windows ABI has the same output-pointer contract as QPC.
    let ok = unsafe { QueryPerformanceFrequency(&mut value) };
    (ok != 0 && value > 0).then_some(value as u64)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProbeConfig {
    output_directory: PathBuf,
    trace_ready: PathBuf,
    trace_finished: PathBuf,
    run_id: String,
    expected_source_identity: String,
    metadata: BTreeMap<String, String>,
}
impl ProbeConfig {
    fn validate(&self) -> Result<(), String> {
        crate::rehearsal_studio_m16_capture::CaptureConfig {
            output: self.output_directory.clone(),
            max_samples: MAX_FRAMES,
            metadata: self.metadata.clone(),
        }
        .validate()?;
        if self.run_id.is_empty()
            || !self
                .run_id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-')
            || self.expected_source_identity.is_empty()
            || self.trace_ready == self.trace_finished
        {
            return Err("invalid run id, source pin, or external trace paths".into());
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum Client {
    Egui,
    Native,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Epoch {
    capture: u64,
    scene: u64,
    resident: u64,
    client: Client,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ProbeInput {
    response_id: u64,
    trial: usize,
    delay_k: u32,
    input_qpc: u64,
    input_frame: u64,
    target_frame: u64,
    epoch: Epoch,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct FrameSpan {
    ordinal: usize,
    main_frame: u64,
    epoch: Epoch,
    response_id: Option<u64>,
    marker_in_render_data: bool,
    marker_pipeline_ready: bool,
    begin_qpc: u64,
    end_qpc: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Rejected {
    at_qpc: u64,
    reason: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ProbeCapture {
    instrument: String,
    process_id: u32,
    qpc_frequency: u64,
    config: ProbeConfig,
    epoch: Epoch,
    started_qpc: u64,
    stopped_qpc: Option<u64>,
    stop_reason: Option<String>,
    initial_facts: Value,
    final_facts: Value,
    inputs: Vec<ProbeInput>,
    frames: Vec<FrameSpan>,
    rejected: Vec<Rejected>,
}
impl ProbeCapture {
    fn ready_for_input(&self, frame: u64) -> bool {
        self.stop_reason.is_none()
            && self.inputs.len() < CASES.len()
            && self.frames.len() >= 60
            && self.inputs.last().is_none_or(|input| {
                frame >= input.target_frame.saturating_add(10)
                    && self.frames.iter().rev().any(|f| {
                        f.response_id == Some(input.response_id)
                            && f.marker_in_render_data
                            && f.marker_pipeline_ready
                    })
            })
    }
    fn ingest(&mut self, at: u64, frame: u64, response_id: u64) -> Result<(), String> {
        if !self.ready_for_input(frame) {
            return Err("probe not ready; wait for the OVL Ready row".into());
        }
        if at < self.started_qpc || self.inputs.last().is_some_and(|i| at <= i.input_qpc) {
            return Err("input QPC reversed or predates capture".into());
        }
        if !(1..2_097_152).contains(&response_id) {
            return Err("response identity exhausted".into());
        }
        let trial = self.inputs.len();
        let delay_k = CASES[trial];
        let target_frame = frame
            .checked_add(u64::from(delay_k))
            .ok_or("frame identity overflow")?;
        self.inputs.push(ProbeInput {
            response_id,
            trial,
            delay_k,
            input_qpc: at,
            input_frame: frame,
            target_frame,
            epoch: self.epoch.clone(),
        });
        Ok(())
    }
    fn visible_response(&self, frame: u64) -> Option<u64> {
        self.inputs
            .iter()
            .rev()
            .find(|i| frame >= i.target_frame)
            .map(|i| i.response_id)
    }
    fn stop(&mut self, at: u64, reason: &str) {
        if self.stop_reason.is_none() {
            self.stopped_qpc = Some(at);
            self.stop_reason = Some(reason.into());
        }
    }
    fn output_path(&self, attempt: u64) -> PathBuf {
        self.config.output_directory.join(format!(
            "{}-{:?}-{}-attempt{}.json",
            self.config.run_id, self.epoch.client, self.epoch.capture, attempt
        ))
    }
}
/// Injective opaque colour identity on the bounded response-id domain.
fn marker_rgb(id: u64) -> [u8; 3] {
    // Odd multiplication permutes the 21-bit domain while making adjacent
    // identities visibly different, rather than changing one channel by 1/255.
    let id = id.wrapping_mul(104_729) % 2_097_152;
    [
        64 + (id % 128) as u8,
        64 + ((id / 128) % 128) as u8,
        64 + ((id / 16_384) % 128) as u8,
    ]
}

#[derive(Clone, Default, Resource)]
struct ExtractedProbeFrame {
    frame: u64,
    epoch: Option<Epoch>,
    response: Option<u64>,
    native_entity: Option<Entity>,
    unobscured: bool,
    geometry: bool,
    ready: bool,
    marker_rect: Option<egui::Rect>,
}
#[derive(Default)]
struct ProbeState {
    config: Option<ProbeConfig>,
    frequency: u64,
    capture_epoch: u64,
    response_id: u64,
    capture: Option<ProbeCapture>,
    main: ExtractedProbeFrame,
    render: ExtractedProbeFrame,
    pending: Option<(ExtractedProbeFrame, u64)>,
    exported: bool,
    export_failed: bool,
    export_attempt: u64,
    status: String,
    observed_adapter: Value,
}
#[derive(Clone, Resource)]
pub struct PresentProbe(Arc<Mutex<ProbeState>>);
struct PresentLayer(PresentProbe);
fn clock_failed(probe: &PresentProbe) {
    if let Ok(mut s) = probe.0.lock() {
        s.status = "INVALID: QPC unavailable; no latency claim".into();
        s.pending = None;
        if let Some(c) = s.capture.as_mut().filter(|c| c.stop_reason.is_none()) {
            c.stop_reason = Some("QPC unavailable; stop timestamp unknown".into());
            c.rejected.push(Rejected {
                at_qpc: 0,
                reason: "QPC unavailable (0 is an invalidity sentinel, not a timestamp)".into(),
            });
        }
    }
}
fn is_present(id: &tracing::span::Id, context: &Context<'_, Registry>) -> bool {
    context.span(id).is_some_and(|s| {
        s.metadata().name() == "present_frames" && s.metadata().target() == "bevy_render::renderer"
    })
}
impl Layer<Registry> for PresentLayer {
    fn on_enter(&self, id: &tracing::span::Id, context: Context<'_, Registry>) {
        if !is_present(id, &context) {
            return;
        }
        let Ok(mut s) = self.0 .0.lock() else {
            return;
        };
        // Exclude time spent waiting for the telemetry/state lock from the
        // correlation interval. Present() cannot execute until this hook returns.
        let Some(at) = qpc() else {
            drop(s);
            clock_failed(&self.0);
            return;
        };
        if s.capture.as_ref().is_none_or(|c| c.stop_reason.is_some()) {
            return;
        }
        if s.render.epoch.as_ref() != s.capture.as_ref().map(|c| &c.epoch) {
            // Pipelined rendering may still be finishing the frame extracted
            // before F6. Retain its rejection, never credit it to the new epoch.
            if let Some(c) = &mut s.capture {
                c.rejected.push(Rejected {
                    at_qpc: at,
                    reason: "in-flight frame from before this capture epoch; excluded".into(),
                });
            }
            return;
        }
        if s.pending.is_some() {
            if let Some(c) = &mut s.capture {
                c.rejected.push(Rejected {
                    at_qpc: at,
                    reason: "overlapping present spans".into(),
                });
                c.stop(at, "invalid present span nesting");
            }
            return;
        }
        s.pending = Some((s.render.clone(), at));
    }
    fn on_exit(&self, id: &tracing::span::Id, context: Context<'_, Registry>) {
        if !is_present(id, &context) {
            return;
        }
        let Some(at) = qpc() else {
            clock_failed(&self.0);
            return;
        };
        let Ok(mut s) = self.0 .0.lock() else {
            return;
        };
        let Some((frame, begin)) = s.pending.take() else {
            return;
        };
        let Some(c) = &mut s.capture else {
            return;
        };
        if frame.epoch.as_ref() != Some(&c.epoch) || begin < c.started_qpc || at < begin {
            c.rejected.push(Rejected {
                at_qpc: at,
                reason: "present has stale capture/scene/resident identity or reversed QPC".into(),
            });
            c.stop(at, "invalid present correlation");
            return;
        }
        let ordinal = c.frames.len();
        c.frames.push(FrameSpan {
            ordinal,
            main_frame: frame.frame,
            epoch: c.epoch.clone(),
            response_id: frame.response,
            marker_in_render_data: frame.geometry,
            marker_pipeline_ready: frame.ready,
            begin_qpc: begin,
            end_qpc: at,
        });
        if c.frames.len() >= MAX_FRAMES {
            c.stop(at, "raw frame limit reached");
        }
    }
}
pub fn log_filter() -> String {
    if std::env::var_os("SIMTHING_M16_LATENCY_CONFIG").is_some() {
        "warn,simthing_mapeditor=info,bevy_render::renderer=info".into()
    } else {
        "warn,simthing_mapeditor=info".into()
    }
}
pub fn log_layer(app: &mut App) -> Option<BoxedLayer> {
    let path = std::env::var_os("SIMTHING_M16_LATENCY_CONFIG")?;
    let config = std::fs::read(path)
        .map_err(|e| e.to_string())
        .and_then(|bytes| serde_json::from_slice::<ProbeConfig>(&bytes).map_err(|e| e.to_string()))
        .and_then(|c| {
            c.validate()?;
            Ok(c)
        });
    let frequency = qpc_frequency().unwrap_or(0);
    let (config, status) = match config {
        Ok(c) if frequency > 0 => (Some(c), "Idle: load the pinned scene, then F6".into()),
        Ok(_) => (None, "INVALID: Windows QPC frequency unavailable".into()),
        Err(e) => (None, format!("Configuration refused: {e}")),
    };
    let probe = PresentProbe(Arc::new(Mutex::new(ProbeState {
        config,
        frequency,
        status,
        ..default()
    })));
    app.insert_resource(probe.clone());
    Some(Box::new(PresentLayer(probe)))
}
#[derive(Component)]
struct NativeMarker;
pub struct PresentProbePlugin;
impl Plugin for PresentProbePlugin {
    fn build(&self, app: &mut App) {
        let Some(probe) = app.world().get_resource::<PresentProbe>().cloned() else {
            return;
        };
        app.init_resource::<ExtractedProbeFrame>()
            .add_systems(Startup, spawn_marker)
            .add_systems(PreUpdate, ingest_input.before(InputSystem))
            .add_systems(
                PostUpdate,
                sync_native_marker.before(bevy::ui::UiSystem::Layout),
            )
            .add_systems(
                Last,
                finish_main_frame.before(bevy::diagnostic::update_frame_count),
            );
        if let Some(render) = app.get_sub_app_mut(RenderApp) {
            render
                .insert_resource(probe)
                .init_resource::<ExtractedProbeFrame>()
                .add_systems(ExtractSchedule, extract_frame)
                .add_systems(
                    Render,
                    inspect_geometry
                        .in_set(RenderSet::Queue)
                        .after(bevy_egui::render::systems::queue_pipelines_system),
                )
                .add_systems(
                    Render,
                    bind_render_frame
                        .in_set(RenderSet::Render)
                        .before(bevy::render::renderer::render_system),
                );
        }
    }
}
fn spawn_marker(mut commands: Commands) {
    // Plain rectangle on the existing native UI camera and pipeline.
    commands.spawn((
        NativeMarker,
        Node {
            display: Display::None,
            position_type: PositionType::Absolute,
            left: Val::Percent(75.0),
            top: Val::Px(MARKER_TOP),
            width: Val::Px(MARKER_SIZE),
            height: Val::Px(MARKER_SIZE),
            ..default()
        },
        GlobalZIndex(200),
        BackgroundColor(Color::BLACK),
    ));
}
fn start_capture(
    s: &mut ProbeState,
    state: &StudioAppState,
    window: &Window,
    at: u64,
) -> Result<(), String> {
    if s.capture.is_some() && !s.exported {
        return Err("prior raw capture retained; F7 stops/exports or retries export".into());
    }
    let config = s.config.clone().ok_or("valid configuration required")?;
    if config.trace_finished.exists()
        || std::fs::read_to_string(&config.trace_ready)
            .ok()
            .as_deref()
            .map(str::trim)
            != Some(&config.run_id)
    {
        return Err("external trace not running for this run id".into());
    }
    let mut facts = crate::rehearsal_studio_m16_capture::facts(state, window);
    facts["adapter"] = s.observed_adapter.clone();
    if facts["source_identity"].as_str() != Some(&config.expected_source_identity)
        || !state.live_bridge_readout.attached
        || state.live_bridge_reset_requested
        || !window.focused
        || modal_open(state)
        || !facts["paused"].as_bool().unwrap_or(false)
    {
        return Err(
            "load the pinned resident, pause, close modal dialogs and focus Studio before F6"
                .into(),
        );
    }
    s.capture_epoch = s
        .capture_epoch
        .checked_add(1)
        .ok_or("capture epoch exhausted")?;
    let epoch = Epoch {
        capture: s.capture_epoch,
        scene: state.scene_render_revision,
        resident: facts["resident_epoch"]
            .as_u64()
            .ok_or("resident epoch unavailable")?,
        client: if state.native_ui.enabled {
            Client::Native
        } else {
            Client::Egui
        },
    };
    let capture = ProbeCapture {
        instrument: INSTRUMENT.into(),
        process_id: std::process::id(),
        qpc_frequency: s.frequency,
        config,
        epoch,
        started_qpc: at,
        stopped_qpc: None,
        stop_reason: None,
        initial_facts: facts.clone(),
        final_facts: facts,
        inputs: Vec::with_capacity(6),
        frames: Vec::with_capacity(MAX_FRAMES),
        rejected: Vec::new(),
    };
    if capture.output_path(0).exists() {
        return Err("raw output already exists; use a new OVL run".into());
    }
    s.capture = Some(capture);
    s.exported = false;
    s.export_failed = false;
    s.export_attempt = 0;
    s.status = "Warming: wait for Ready, then F11".into();
    Ok(())
}
fn modal_open(state: &StudioAppState) -> bool {
    crate::rehearsal_studio_native_ui::native_suspended(state)
}
fn ingest_input(
    probe: Res<PresentProbe>,
    mut inputs: EventReader<KeyboardInput>,
    count: Res<FrameCount>,
    state: Res<StudioAppState>,
    windows: Query<(Entity, &Window), With<PrimaryWindow>>,
) {
    for event in inputs.read() {
        // First Studio-owned ingestion of the raw Bevy OS-input event, before
        // InputSystem/client processing. Not an OS hardware-event timestamp.
        let at = qpc();
        if event.state != ButtonState::Pressed
            || !matches!(event.key_code, KeyCode::F6 | KeyCode::F7 | KeyCode::F11)
        {
            continue;
        }
        let Some(at) = at else {
            clock_failed(&probe);
            continue;
        };
        let Ok(mut s) = probe.0.lock() else {
            return;
        };
        if windows
            .single()
            .map_or(true, |(entity, _)| entity != event.window)
        {
            s.status = "Probe/control key from a non-primary window refused".into();
            continue;
        }
        if event.repeat {
            if let Some(c) = s.capture.as_mut().filter(|c| c.stop_reason.is_none()) {
                c.rejected.push(Rejected {
                    at_qpc: at,
                    reason: "repeated probe/control key refused".into(),
                });
            }
            continue;
        }
        match event.key_code {
            KeyCode::F6 => {
                let result = windows
                    .single()
                    .map_err(|e| e.to_string())
                    .and_then(|(_, w)| start_capture(&mut s, &state, w, at));
                if let Err(e) = result {
                    s.status = e;
                }
            }
            KeyCode::F7 => {
                if let Some(c) = &mut s.capture {
                    c.stop(at, "operator stop");
                }
                s.export_failed = false;
            }
            KeyCode::F11 => {
                let id = s.response_id.saturating_add(1);
                let result = s
                    .capture
                    .as_mut()
                    .ok_or_else(|| "press F6 first".to_string())
                    .and_then(|c| c.ingest(at, u64::from(count.0), id));
                match result {
                    Ok(()) => {
                        s.response_id = id;
                        s.status = "Response pending its designated frame".into();
                    }
                    Err(e) => {
                        if let Some(c) = s.capture.as_mut().filter(|c| c.stop_reason.is_none()) {
                            c.rejected.push(Rejected {
                                at_qpc: at,
                                reason: e.clone(),
                            });
                        }
                        s.status = e;
                    }
                }
            }
            _ => {}
        }
    }
}
fn sync_native_marker(
    probe: Res<PresentProbe>,
    count: Res<FrameCount>,
    mut markers: Query<(Entity, &mut Node, &mut BackgroundColor), With<NativeMarker>>,
) {
    let Ok(mut s) = probe.0.lock() else {
        return;
    };
    let visible = s
        .capture
        .as_ref()
        .filter(|c| c.epoch.client == Client::Native)
        .and_then(|c| c.visible_response(u64::from(count.0)));
    for (entity, mut node, mut colour) in &mut markers {
        s.main.native_entity = Some(entity);
        node.display = if visible.is_some() {
            Display::Flex
        } else {
            Display::None
        };
        if let Some(id) = visible {
            let rgb = marker_rgb(id);
            colour.0 = Color::srgb_u8(rgb[0], rgb[1], rgb[2]);
        }
    }
}
fn finish_main_frame(
    probe: Res<PresentProbe>,
    count: Res<FrameCount>,
    state: Res<StudioAppState>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut extracted: ResMut<ExtractedProbeFrame>,
    mut exits: EventReader<AppExit>,
) {
    let Some(at) = qpc() else {
        clock_failed(&probe);
        return;
    };
    let Ok(mut s) = probe.0.lock() else {
        return;
    };
    let frame = u64::from(count.0);
    let closing = exits.read().next().is_some();
    let exported = s.exported;
    let observed_adapter = s.observed_adapter.clone();
    if let Some(c) = s.capture.as_mut().filter(|_| !exported) {
        if let Ok(window) = windows.single() {
            let mut facts = crate::rehearsal_studio_m16_capture::facts(&state, window);
            facts["adapter"] = observed_adapter.clone();
            if c.stop_reason.is_none()
                && ([
                    "scene",
                    "resident_epoch",
                    "native_enabled",
                    "paused",
                    "window_pixels",
                    "scale_factor",
                    "source_identity",
                    "profile_identity",
                    "dependencies",
                    "panels_hidden",
                    "adapter",
                ]
                .iter()
                .any(|k| c.initial_facts[*k] != facts[*k])
                    || !window.focused
                    || modal_open(&state)
                    || state.live_bridge_reset_requested
                    || !state.live_bridge_readout.attached)
            {
                c.stop(at, "scene/resident/condition/window/focus/modal boundary");
                c.rejected.push(Rejected {
                    at_qpc: at,
                    reason: "runtime qualification changed during capture".into(),
                });
            }
            c.final_facts = facts;
        } else {
            c.stop(at, "primary window unavailable");
        }
        if c.config.trace_finished.exists() {
            c.stop(at, "external trace stopped");
        }
        if closing {
            c.stop(at, "application exit");
        }
    }
    s.main.frame = frame;
    s.main.epoch = s.capture.as_ref().map(|c| c.epoch.clone());
    s.main.response = s.capture.as_ref().and_then(|c| c.visible_response(frame));
    *extracted = s.main.clone();
    if s.capture.as_ref().is_some_and(|c| c.stop_reason.is_some())
        && s.pending.is_none()
        && !s.exported
        && !s.export_failed
    {
        let c = s.capture.as_ref().unwrap();
        let path = c.output_path(s.export_attempt);
        let result = write_raw_capture(c, &path);
        s.export_attempt = s.export_attempt.saturating_add(1);
        match result {
            Ok(()) => {
                s.exported = true;
                s.status = format!(
                    "Raw JSON saved: {}",
                    path.file_name().unwrap().to_string_lossy()
                );
            }
            Err(e) => {
                s.export_failed = true;
                s.status = format!("EXPORT FAILED; raw retained; F7 retries: {e}");
            }
        }
    }
}
fn write_raw_capture(capture: &ProbeCapture, path: &std::path::Path) -> Result<(), String> {
    let bytes = serde_json::to_vec_pretty(capture).map_err(|e| e.to_string())?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    file.write_all(&bytes)
        .and_then(|_| file.sync_all())
        .map_err(|e| e.to_string())
}
fn extract_frame(mut commands: Commands, frame: Extract<Res<ExtractedProbeFrame>>) {
    commands.insert_resource((**frame).clone());
}
fn inspect_geometry(
    mut frame: ResMut<ExtractedProbeFrame>,
    native: Option<Res<bevy::ui::ExtractedUiNodes>>,
    egui_output: Query<&EguiRenderOutput>,
) {
    frame.geometry = false;
    frame.ready = false;
    let (Some(id), Some(epoch)) = (frame.response, frame.epoch.as_ref()) else {
        return;
    };
    if !frame.unobscured {
        return;
    }
    let rgb = marker_rgb(id);
    frame.geometry = match epoch.client {
        Client::Native => native.is_some_and(|nodes| {
            nodes.uinodes.iter().any(|node| {
                frame.native_entity == Some(node.main_entity.id())
                    && node.color == Color::srgb_u8(rgb[0], rgb[1], rgb[2]).to_linear()
                    && node.rect.width() > 0.0
                    && node.rect.height() > 0.0
                    && native_marker_unclipped(node)
                    && matches!(
                        node.item,
                        bevy::ui::ExtractedUiItem::Node {
                            node_type: bevy::ui::NodeType::Rect,
                            ..
                        }
                    )
            })
        }),
        Client::Egui => frame.marker_rect.is_some_and(|rect| {
            egui_output
                .iter()
                .any(|output| egui_marker_present(output, rect, rgb))
        }),
    };
}
fn native_marker_unclipped(node: &bevy::ui::ExtractedUiNode) -> bool {
    let bevy::ui::ExtractedUiItem::Node { transform, .. } = &node.item else {
        return false;
    };
    let half = node.rect.size() * 0.5;
    node.clip.is_none_or(|clip| {
        [
            Vec2::new(-half.x, -half.y),
            Vec2::new(half.x, -half.y),
            Vec2::new(half.x, half.y),
            Vec2::new(-half.x, half.y),
        ]
        .iter()
        .all(|p| clip.contains(transform.transform_point3(p.extend(0.0)).truncate()))
    })
}
fn bind_render_frame(
    mut frame: ResMut<ExtractedProbeFrame>,
    probe: Res<PresentProbe>,
    cache: Res<bevy::render::render_resource::PipelineCache>,
    egui_pipelines: Option<Res<bevy_egui::render::systems::EguiPipelines>>,
    egui_textures: Option<Res<bevy_egui::render::systems::EguiTextureBindGroups>>,
    egui_transforms: Option<Res<bevy_egui::render::systems::EguiTransforms>>,
    ui_phases: Option<
        Res<bevy::render::render_phase::ViewSortedRenderPhases<bevy::ui::TransparentUi>>,
    >,
) {
    frame.ready = match frame.epoch.as_ref().map(|e| e.client) {
        Some(Client::Egui) => egui_pipelines.is_some_and(|p| {
            p.0.len() == 1
                && p.0.iter().all(|(entity, id)| {
                    cache.get_render_pipeline(*id).is_some()
                        && egui_textures.as_ref().is_some_and(|t| {
                            t.0.contains_key(&bevy_egui::render::systems::EguiTextureId::Managed(
                                *entity, 0,
                            ))
                        })
                        && egui_transforms.as_ref().is_some_and(|t| {
                            t.bind_group.is_some() && t.offsets.contains_key(entity)
                        })
                })
        }),
        Some(Client::Native) => ui_phases.is_some_and(|phases| {
            phases.values().any(|phase| {
                phase.items.iter().any(|item| {
                    frame.native_entity == Some(item.entity.1.id())
                        && cache.get_render_pipeline(item.pipeline).is_some()
                })
            })
        }),
        None => false,
    };
    if let Ok(mut s) = probe.0.lock() {
        s.render = frame.clone();
    }
}

fn egui_marker_present(output: &EguiRenderOutput, rect: egui::Rect, rgb: [u8; 3]) -> bool {
    let colour = egui::Color32::from_rgb(rgb[0], rgb[1], rgb[2]);
    output.paint_jobs.iter().any(|job| {
        let egui::epaint::Primitive::Mesh(mesh) = &job.primitive else {
            return false;
        };
        if mesh.texture_id != egui::TextureId::Managed(0) || !job.clip_rect.contains_rect(rect) {
            return false;
        }
        let corners: Option<Vec<u32>> = [
            rect.left_top(),
            rect.right_top(),
            rect.right_bottom(),
            rect.left_bottom(),
        ]
        .iter()
        .map(|p| {
            mesh.vertices
                .iter()
                .position(|v| v.color == colour && v.pos == *p && v.uv == egui::epaint::WHITE_UV)
                .map(|i| i as u32)
        })
        .collect();
        let Some(corners) = corners else {
            return false;
        };
        // Referenced corners alone could be degenerate geometry. Require the
        // two actual indexed triangles that cover this opaque rectangle.
        let triangle = |a: usize, b: usize, c: usize| {
            let mut expected = [corners[a], corners[b], corners[c]];
            expected.sort_unstable();
            mesh.indices.chunks_exact(3).any(|t| {
                let mut actual = [t[0], t[1], t[2]];
                actual.sort_unstable();
                actual == expected
            })
        };
        (triangle(0, 1, 2) && triangle(0, 2, 3)) || (triangle(0, 1, 3) && triangle(1, 2, 3))
    })
}

/// Read-only pane in the existing egui pass. The full Studio_ops modal would
/// suspend the native condition; this measurement pane does not set that flag.
pub fn draw_ovl(
    ctx: &egui::Context,
    probe: &PresentProbe,
    frame: u64,
    telemetry: &crate::studio_performance_telemetry::StudioPerformanceTelemetry,
) {
    let Ok(mut s) = probe.0.lock() else {
        return;
    };
    s.observed_adapter = serde_json::json!({ "gpu_name": telemetry.gpu_name, "backend": telemetry.gpu_backend,
        "vendor_id": telemetry.gpu_vendor_id, "device_id": telemetry.gpu_device_id,
        "device_type": telemetry.gpu_device_type, "policy": telemetry.gpu_adapter_policy_status,
        "wgpu_present_mode": telemetry.present_mode });
    let screen = ctx.screen_rect();
    let rect = egui::Rect::from_min_size(
        egui::pos2(screen.width() * 0.75, MARKER_TOP),
        egui::vec2(MARKER_SIZE, MARKER_SIZE),
    );
    let visible = s
        .capture
        .as_ref()
        .and_then(|c| c.visible_response(frame).map(|id| (id, c.epoch.client)));
    if let Some((id, Client::Egui)) = visible {
        let rgb = marker_rgb(id);
        let mut mesh = egui::Mesh::default();
        mesh.add_colored_rect(rect, egui::Color32::from_rgb(rgb[0], rgb[1], rgb[2]));
        ctx.layer_painter(egui::LayerId::new(
            egui::Order::Foreground,
            egui::Id::new("m16_response_marker"),
        ))
        .add(egui::Shape::mesh(mesh));
    }
    egui::Window::new("Studio_ops Telemetry · M16 OVL")
        .default_pos(egui::pos2(screen.width() * 0.72, 150.0))
        .default_width(screen.width() * 0.26)
        .resizable(true)
        .vscroll(true)
        .show(ctx, |ui| {
            ui.label(INSTRUMENT);
            ui.label(format!(
                "GPU: {} · {}",
                telemetry.gpu_name.as_deref().unwrap_or("pending"),
                telemetry.gpu_backend.as_deref().unwrap_or("pending")
            ));
            ui.label("F6 start · F11 next response · F7 stop/export");
            ui.label("F8 selects native between captures.");
            ui.separator();
            ui.label(&s.status);
            if let Some(c) = &s.capture {
                ui.label(format!(
                    "Client: {:?} · capture {} · scene {} · resident {}",
                    c.epoch.client, c.epoch.capture, c.epoch.scene, c.epoch.resident
                ));
                ui.label(format!(
                    "PID {} · QPC frequency {} Hz",
                    c.process_id, c.qpc_frequency
                ));
                ui.label(format!(
                    "Responses: {}/6 · present spans: {} · refusals: {}",
                    c.inputs.len(),
                    c.frames.len(),
                    c.rejected.len()
                ));
                ui.label(if s.exported {
                    "Capture exported: take the screenshot now"
                } else if c.stop_reason.is_some() {
                    "Capture stopped: check the export status"
                } else if c.ready_for_input(frame) {
                    "Ready: press F11 once"
                } else if c.inputs.len() == 6 {
                    "Six inputs: wait one second, then F7"
                } else {
                    "Wait for Ready before the next F11"
                });
                if let Some(i) = c.inputs.last() {
                    ui.label(format!(
                        "Requested id {} · trial {} · K={}",
                        i.response_id,
                        i.trial + 1,
                        i.delay_k
                    ));
                    ui.label(format!(
                        "Input QPC: {} · frame {} · target {}",
                        i.input_qpc, i.input_frame, i.target_frame
                    ));
                }
                ui.label(format!(
                    "Marker response identity: {:?}",
                    visible.map(|v| v.0)
                ));
                if let Some(f) = c.frames.iter().find(|f| {
                    f.response_id == visible.map(|v| v.0)
                        && f.marker_in_render_data
                        && f.marker_pipeline_ready
                }) {
                    ui.label(format!(
                        "First prepared app frame: {} · span {}",
                        f.main_frame, f.ordinal
                    ));
                    ui.label(format!(
                        "Submission bounds: {} .. {}",
                        f.begin_qpc, f.end_qpc
                    ));
                } else {
                    ui.label("Matching prepared marker/present span: pending");
                }
                ui.label("Exact submission and DISPLAYED: pending offline PresentMon join");
                ui.label(format!(
                    "Stopped: {:?} · exported: {}",
                    c.stop_reason, s.exported
                ));
                ui.label(format!(
                    "Code: {}",
                    c.config
                        .metadata
                        .get("code_revision")
                        .map(String::as_str)
                        .unwrap_or("unavailable")
                ));
            }
            ui.small(
                "Only the coloured square is delayed. Telemetry updates are not latency endpoints.",
            );
        });
    s.main.marker_rect = Some(rect);
    s.main.unobscured = screen.contains_rect(rect)
        && match visible {
            Some((_, Client::Native)) => [
                rect.left_top(),
                rect.right_top(),
                rect.right_bottom(),
                rect.left_bottom(),
                rect.center(),
            ]
            .iter()
            .all(|p| {
                ctx.layer_id_at(*p)
                    .is_none_or(|layer| layer.order == egui::Order::Background)
            }),
            _ => true,
        };
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::log::tracing_subscriber::prelude::*;

    fn capture() -> ProbeCapture {
        ProbeCapture {
            instrument: INSTRUMENT.into(),
            process_id: std::process::id(),
            qpc_frequency: 10_000_000,
            config: ProbeConfig {
                output_directory: PathBuf::from("unused"),
                trace_ready: PathBuf::from("ready"),
                trace_finished: PathBuf::from("finished"),
                run_id: "unit".into(),
                expected_source_identity: "unit".into(),
                metadata: BTreeMap::new(),
            },
            epoch: Epoch {
                capture: 1,
                scene: 7,
                resident: 9,
                client: Client::Egui,
            },
            started_qpc: 1,
            stopped_qpc: None,
            stop_reason: None,
            initial_facts: Value::Null,
            final_facts: Value::Null,
            inputs: vec![],
            frames: vec![],
            rejected: vec![],
        }
    }
    fn warm(c: &mut ProbeCapture) {
        for n in 0..60 {
            c.frames.push(FrameSpan {
                ordinal: n,
                main_frame: n as u64,
                epoch: c.epoch.clone(),
                response_id: None,
                marker_in_render_data: false,
                marker_pipeline_ready: false,
                begin_qpc: 2 + n as u64 * 10,
                end_qpc: 3 + n as u64 * 10,
            });
        }
    }

    #[test]
    fn rehearsal_studio_m16_known_delay_preserves_prior_marker_and_refuses_closed_input() {
        let mut c = capture();
        assert!(
            c.ingest(10, 0, 1).is_err(),
            "unwarmed capture cannot accept a probe"
        );
        warm(&mut c);
        for trial in 0..6 {
            let frame = 100 + trial as u64 * 30;
            let id = trial as u64 + 1;
            c.ingest(1_000 + trial as u64 * 300, frame, id).unwrap();
            let k = u64::from(CASES[trial]);
            if k > 0 {
                assert_eq!(c.visible_response(frame + k - 1), Some(id - 1));
            }
            assert_eq!(c.visible_response(frame + k), Some(id));
            assert!(!c.ready_for_input(frame + k));
            c.frames.push(FrameSpan {
                ordinal: c.frames.len(),
                main_frame: frame + k,
                epoch: c.epoch.clone(),
                response_id: Some(id),
                marker_in_render_data: true,
                marker_pipeline_ready: true,
                begin_qpc: 1_100 + trial as u64 * 300,
                end_qpc: 1_101 + trial as u64 * 300,
            });
        }
        assert_eq!(
            c.inputs.iter().map(|i| i.delay_k).collect::<Vec<_>>(),
            CASES
        );
        assert!(
            c.ingest(10_000, 400, 7).is_err(),
            "six-trial limit is bounded"
        );
        c.stop(20_000, "operator stop");
        let frozen = serde_json::to_value(&c).unwrap();
        c.stop(30_000, "later reset cannot replace the stop endpoint");
        assert_eq!(serde_json::to_value(&c).unwrap(), frozen);
        assert!(c.ingest(40_000, 500, 8).is_err());
        assert_ne!(marker_rgb(1), marker_rgb(2));
        assert!(marker_rgb(1).iter().all(|v| (64..=191).contains(v)));
    }

    #[test]
    fn rehearsal_studio_m16_raw_event_ingestion_is_window_bound_and_precedes_button_processing() {
        let mut c = capture();
        warm(&mut c);
        let probe = PresentProbe(Arc::new(Mutex::new(ProbeState {
            capture: Some(c),
            ..default()
        })));
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, bevy::input::InputPlugin))
            .insert_resource(FrameCount(100))
            .insert_resource(probe.clone())
            .insert_resource(StudioAppState::default())
            .add_systems(PreUpdate, ingest_input.before(InputSystem));
        let window = app
            .world_mut()
            .spawn((Window::default(), PrimaryWindow))
            .id();
        let before = qpc().unwrap();
        let input = KeyboardInput {
            key_code: KeyCode::F11,
            logical_key: bevy::input::keyboard::Key::F11,
            state: ButtonState::Pressed,
            text: None,
            repeat: false,
            window,
        };
        app.world_mut().send_event(input.clone());
        app.update();
        let after = qpc().unwrap();
        let accepted = probe.0.lock().unwrap().capture.as_ref().unwrap().inputs[0].clone();
        assert!((before..=after).contains(&accepted.input_qpc));
        assert_eq!(accepted.input_frame, 100);
        assert!(app
            .world()
            .resource::<ButtonInput<KeyCode>>()
            .pressed(KeyCode::F11));
        let foreign_window = app.world_mut().spawn_empty().id();
        app.world_mut().send_event(KeyboardInput {
            window: foreign_window,
            ..input
        });
        app.update();
        assert_eq!(
            probe
                .0
                .lock()
                .unwrap()
                .capture
                .as_ref()
                .unwrap()
                .inputs
                .len(),
            1
        );
    }

    #[test]
    fn rehearsal_studio_m16_present_span_excludes_prior_epoch_and_clock_failure() {
        let c = capture();
        let epoch = c.epoch.clone();
        let probe = PresentProbe(Arc::new(Mutex::new(ProbeState {
            capture: Some(c),
            ..default()
        })));
        let subscriber = Registry::default().with(PresentLayer(probe.clone()));
        tracing::subscriber::with_default(subscriber, || {
            {
                let _span = tracing::info_span!(target: "bevy_render::renderer", "present_frames")
                    .entered();
            }
            assert!(probe
                .0
                .lock()
                .unwrap()
                .capture
                .as_ref()
                .unwrap()
                .frames
                .is_empty());
            probe.0.lock().unwrap().render = ExtractedProbeFrame {
                frame: 100,
                epoch: Some(epoch),
                response: Some(1),
                geometry: true,
                ready: true,
                ..default()
            };
            {
                let _span = tracing::info_span!(target: "bevy_render::renderer", "present_frames")
                    .entered();
            }
        });
        {
            let s = probe.0.lock().unwrap();
            let c = s.capture.as_ref().unwrap();
            assert_eq!(c.frames.len(), 1);
            assert_eq!(c.rejected.len(), 1);
            assert!(c.frames[0].end_qpc >= c.frames[0].begin_qpc);
            assert_eq!(c.frames[0].main_frame, 100);
            assert_eq!(c.frames[0].epoch, c.epoch);
        }
        clock_failed(&probe);
        let s = probe.0.lock().unwrap();
        let c = s.capture.as_ref().unwrap();
        assert!(c.stop_reason.as_ref().unwrap().contains("QPC unavailable"));
        assert!(
            c.stopped_qpc.is_none(),
            "do not fabricate a missing timestamp"
        );
        assert!(!c.ready_for_input(200));
    }

    #[test]
    fn rehearsal_studio_m16_tessellated_marker_refuses_clipping_texture_and_wrong_identity() {
        let ctx = egui::Context::default();
        let rect = egui::Rect::from_min_size(egui::pos2(40.0, 40.0), egui::vec2(32.0, 32.0));
        let rgb = marker_rgb(17);
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(500.0, 300.0),
                )),
                ..default()
            },
            |ctx| {
                let mut mesh = egui::Mesh::default();
                mesh.add_colored_rect(rect, egui::Color32::from_rgb(rgb[0], rgb[1], rgb[2]));
                ctx.layer_painter(egui::LayerId::background())
                    .add(egui::Shape::mesh(mesh));
            },
        );
        let mut rendered = EguiRenderOutput {
            paint_jobs: ctx.tessellate(output.shapes, output.pixels_per_point),
            ..default()
        };
        assert!(egui_marker_present(&rendered, rect, rgb));
        assert!(!egui_marker_present(&rendered, rect, marker_rgb(18)));
        let original = rendered.clone();
        rendered.paint_jobs[0].clip_rect.max.x = 50.0;
        assert!(!egui_marker_present(&rendered, rect, rgb));
        rendered = original.clone();
        if let egui::epaint::Primitive::Mesh(mesh) = &mut rendered.paint_jobs[0].primitive {
            mesh.indices = vec![0, 0, 1, 2, 2, 3];
        }
        assert!(!egui_marker_present(&rendered, rect, rgb));
        rendered = original;
        if let egui::epaint::Primitive::Mesh(mesh) = &mut rendered.paint_jobs[0].primitive {
            mesh.texture_id = egui::TextureId::User(3);
        }
        assert!(!egui_marker_present(&rendered, rect, rgb));
    }

    #[test]
    fn rehearsal_studio_m16_export_preserves_collisions_and_retains_raw_for_new_attempt() {
        let mut c = capture();
        warm(&mut c);
        c.stop(5_000, "operator stop");
        let dir = std::env::temp_dir().join(format!(
            "simthing-m16-{}-{}",
            std::process::id(),
            qpc().unwrap()
        ));
        std::fs::create_dir(&dir).unwrap();
        c.config.output_directory = dir.clone();
        let first = c.output_path(0);
        let retry = c.output_path(1);
        std::fs::write(&first, b"retained partial or prior evidence").unwrap();
        assert!(write_raw_capture(&c, &first).is_err());
        assert_eq!(
            std::fs::read(&first).unwrap(),
            b"retained partial or prior evidence"
        );
        write_raw_capture(&c, &retry).unwrap();
        let saved: ProbeCapture = serde_json::from_slice(&std::fs::read(&retry).unwrap()).unwrap();
        assert_eq!(
            serde_json::to_value(saved).unwrap(),
            serde_json::to_value(&c).unwrap()
        );
        std::fs::remove_file(first).unwrap();
        std::fs::remove_file(retry).unwrap();
        std::fs::remove_dir(dir).unwrap();
    }
}
