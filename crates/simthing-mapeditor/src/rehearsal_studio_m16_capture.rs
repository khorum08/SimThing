//! B0a passive capture. Durations are host wall intervals, never display latency.
//! Delete this module and its app/client hooks when the 1.3 experiment is retired.
#![cfg(windows)]

use std::{collections::BTreeMap, fs::OpenOptions, io::Write, path::PathBuf, time::Instant};

use bevy::{prelude::*, time::TimeUpdateStrategy, window::PrimaryWindow};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::app::StudioAppState;

pub const INSTRUMENT: &str = "m16-b0a-v1";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CaptureConfig {
    pub output: PathBuf,
    pub max_samples: usize,
    /// Operator qualification, retained verbatim alongside observed runtime facts.
    pub metadata: BTreeMap<String, String>,
}

impl CaptureConfig {
    pub fn validate(&self) -> Result<(), String> {
        if !(1..=1_000_000).contains(&self.max_samples) || self.output.as_os_str().is_empty() {
            return Err("output and max_samples (1..=1000000) required".into());
        }
        for key in [
            "condition",
            "repetition",
            "code_revision",
            "instrument_revision",
            "seed",
            "hardware",
            "backend",
            "driver",
            "compiler",
            "build_flags",
            "cache_state",
            "warmup",
            "exact_command",
            "operation_sequence",
            "workload_cardinalities",
            "presentation_settings",
            "subscriptions",
        ] {
            if self.metadata.get(key).is_none_or(|s| s.trim().is_empty()) {
                return Err(format!("missing qualification: {key}"));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Client {
    Egui,
    Native,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ObservationStamp {
    pub scene: u64,
    pub resident_epoch: u64,
    pub generation: u64,
    pub published_ns: u128,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind")]
pub enum Sample {
    Frame {
        start_ns: u128,
        end_ns: u128,
        delta_ns: u128,
        time_strategy: String,
    },
    Cpu {
        client: Client,
        scope: String,
        start_ns: u128,
        end_ns: u128,
    },
    Publication {
        stamp: ObservationStamp,
    },
    Consumption {
        client: Client,
        stamp: ObservationStamp,
        consumed_ns: u128,
    },
    Rejected {
        client: Client,
        at_ns: u128,
        reason: String,
    },
    Runtime {
        at_ns: u128,
        facts: Value,
    },
}

#[derive(Debug, Serialize)]
pub struct Capture {
    pub instrument: &'static str,
    pub config: CaptureConfig,
    pub started_ns: u128,
    pub stopped_ns: Option<u128>,
    pub stop_reason: Option<String>,
    pub initial_facts: Value,
    pub samples: Vec<Sample>,
}

/// Only measurement state: does not subscribe, command, tick, or hold a resident.
pub struct M16Capture {
    origin: Instant,
    resident_epoch: u64,
    publication: Option<ObservationStamp>,
    consumed: [Option<ObservationStamp>; 2],
    last_frame_end: Option<Instant>,
    last_facts: Option<Value>,
    pub capture: Option<Capture>,
}

impl Default for M16Capture {
    fn default() -> Self {
        Self {
            origin: Instant::now(),
            resident_epoch: 0,
            publication: None,
            consumed: [None; 2],
            last_frame_end: None,
            last_facts: None,
            capture: None,
        }
    }
}

impl M16Capture {
    fn ns(&self, at: Instant) -> u128 {
        at.saturating_duration_since(self.origin).as_nanos()
    }

    pub fn start(&mut self, config: CaptureConfig, facts: Value) -> Result<(), String> {
        config.validate()?;
        if self.capture.is_some() {
            return Err("capture retained; stop/export with F10 before starting another".into());
        }
        let started_ns = self.ns(Instant::now());
        self.last_frame_end = None;
        self.consumed = [None; 2];
        self.last_facts = Some(facts.clone());
        self.capture = Some(Capture {
            instrument: INSTRUMENT,
            config,
            started_ns,
            stopped_ns: None,
            stop_reason: None,
            initial_facts: facts,
            samples: Vec::new(),
        });
        Ok(())
    }

    pub fn stop(&mut self, reason: &str) {
        let at = self.ns(Instant::now());
        if let Some(c) = self.capture.as_mut().filter(|c| c.stopped_ns.is_none()) {
            c.stopped_ns = Some(at);
            c.stop_reason = Some(reason.into());
        }
    }

    fn push(&mut self, sample: Sample) {
        let Some(c) = self.capture.as_mut().filter(|c| c.stopped_ns.is_none()) else {
            return;
        };
        c.samples.push(sample);
        if c.samples.len() == c.config.max_samples {
            self.stop("capacity reached; entire raw prefix retained");
        }
    }

    pub fn active(&self) -> bool {
        self.capture
            .as_ref()
            .is_some_and(|c| c.stopped_ns.is_none())
    }

    /// Stop before I/O. Refuse overwrite and retain in-memory data after any error.
    pub fn export(&mut self) -> Result<PathBuf, String> {
        self.stop("operator stop");
        let c = self.capture.as_ref().ok_or("no retained capture")?;
        let bytes = serde_json::to_vec_pretty(c).map_err(|e| e.to_string())?;
        let path = c.config.output.clone();
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|e| e.to_string())?;
        file.write_all(&bytes)
            .and_then(|_| file.sync_all())
            .map_err(|e| e.to_string())?;
        self.capture = None;
        Ok(path)
    }

    pub fn frame(&mut self, time: &Time<Real>, strategy: &TimeUpdateStrategy) {
        if !self.active() {
            return;
        }
        let Some(end) = time.last_update() else {
            return;
        };
        if self.last_frame_end == Some(end) {
            return;
        }
        self.last_frame_end = Some(end);
        let Some(start) = end.checked_sub(time.delta()) else {
            return;
        };
        // The first interval straddling F9/warmup is outside this capture.
        if start < self.origin
            || self.ns(start) < self.capture.as_ref().unwrap().started_ns
            || time.delta().is_zero()
        {
            return;
        }
        self.push(Sample::Frame {
            start_ns: self.ns(start),
            end_ns: self.ns(end),
            delta_ns: time.delta().as_nanos(),
            time_strategy: match strategy {
                TimeUpdateStrategy::Automatic => "Automatic".into(),
                TimeUpdateStrategy::ManualInstant(_) => "ManualInstant (synthetic)".into(),
                TimeUpdateStrategy::ManualDuration(d) => {
                    format!("ManualDuration (synthetic): {} ns", d.as_nanos())
                }
            },
        });
    }

    pub fn cpu_start(&self) -> Option<Instant> {
        self.active().then(Instant::now)
    }

    pub fn cpu_end(&mut self, client: Client, scope: &str, start: Option<Instant>) {
        if let Some(start) = start {
            let end = Instant::now();
            self.push(Sample::Cpu {
                client,
                scope: scope.into(),
                start_ns: self.ns(start),
                end_ns: self.ns(end),
            });
        }
    }

    /// Called only at the existing successful resident replacement/reset doors.
    pub fn resident_reset(&mut self) {
        self.resident_epoch += 1;
        self.publication = None;
        self.consumed = [None; 2];
    }

    /// Timestamp first app publication of each (scene, resident, generation).
    /// Reassigning the same paused snapshot must never make its age look younger.
    pub fn publish(&mut self, scene: u64, generation: u64, attached: bool) {
        if !attached {
            self.publication = None;
            return;
        }
        if self.publication.is_some_and(|p| {
            (p.scene, p.resident_epoch, p.generation) == (scene, self.resident_epoch, generation)
        }) {
            return;
        }
        let stamp = ObservationStamp {
            scene,
            resident_epoch: self.resident_epoch,
            generation,
            published_ns: self.ns(Instant::now()),
        };
        self.publication = Some(stamp);
        self.push(Sample::Publication { stamp });
    }

    /// Adapter consumption only: this endpoint says nothing about visible pixels.
    pub fn consume(
        &mut self,
        client: Client,
        scene: u64,
        generation: u64,
        reset_pending: bool,
        attached: bool,
    ) {
        if !self.active() {
            return;
        }
        let at = self.ns(Instant::now());
        let valid = self.publication.filter(|p| {
            !reset_pending
                && attached
                && p.scene == scene
                && p.resident_epoch == self.resident_epoch
                && p.generation == generation
        });
        let Some(stamp) = valid else {
            self.push(Sample::Rejected {
                client,
                at_ns: at,
                reason:
                    "no matching current scene/resident/generation publication (or reset pending)"
                        .into(),
            });
            return;
        };
        let index = if client == Client::Egui { 0 } else { 1 };
        if self.consumed[index] == Some(stamp) {
            return;
        }
        self.consumed[index] = Some(stamp);
        self.push(Sample::Consumption {
            client,
            stamp,
            consumed_ns: at,
        });
    }

    fn runtime(&mut self, facts: Value) {
        if self.last_facts.as_ref() != Some(&facts) {
            self.last_facts = Some(facts.clone());
            self.push(Sample::Runtime {
                at_ns: self.ns(Instant::now()),
                facts,
            });
        }
    }
}

/// Identical production computation and boundaries for either client. CPU scope is
/// elapsed host wall time of shared projection preparation, excluding widgets/render.
pub fn measured_projection(
    state: &mut StudioAppState,
    client: Client,
) -> crate::StudioLiveObservationReadout {
    let started = state.m16.cpu_start();
    let clock = state.sim_clock_transport.readout();
    let obs = crate::build_studio_live_observation_readout(
        &clock,
        &state.live_bridge_readout,
        state.session.as_ref(),
    );
    state
        .m16
        .cpu_end(client, "shared_clock_and_observation_projection", started);
    obs
}

pub fn consume_projection(state: &mut StudioAppState, client: Client) {
    state.m16.consume(
        client,
        state.scene_render_revision,
        state.live_bridge_readout.executed_ticks,
        state.live_bridge_reset_requested,
        state.live_bridge_readout.attached,
    );
}

fn facts(state: &StudioAppState, window: &Window) -> Value {
    let clock = state.sim_clock_transport.readout();
    let provenance = state
        .session
        .as_ref()
        .and_then(|s| s.authored_live_profile.as_ref())
        .and_then(|p| p.source_cache_provenance.as_ref());
    json!({ "scene": state.scene_render_revision, "resident_epoch": state.m16.resident_epoch,
        "source_identity": provenance.map(|p| &p.cache.source_identity),
        "profile_identity": provenance.map(|p| &p.profile_identity),
        "dependencies": provenance.map(|p| &p.cache.dependencies),
        "native_enabled": state.native_ui.enabled, "paused": clock.paused,
        "rate": clock.rate_label, "max_tps": clock.max_tps,
        "selected_system": state.selection.selected_system_id,
        "systems": state.session.as_ref().map(|s| s.view_model.stars.len()),
        "bridge_path": state.live_bridge_readout.session_path_label,
        "reset_pending": state.live_bridge_reset_requested,
        "panels_hidden": state.performance_diagnostic_hide_panels,
        "window_pixels": [window.physical_width(), window.physical_height()],
        "scale_factor": window.scale_factor(), "focused": window.focused })
}

pub struct M16CapturePlugin;
impl Plugin for M16CapturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, capture_frame);
    }
}

fn capture_frame(
    mut state: ResMut<StudioAppState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time<Real>>,
    strategy: Res<TimeUpdateStrategy>,
) {
    let Ok(window) = windows.single() else {
        state.m16.stop("primary window unavailable");
        return;
    };
    if keyboard.just_pressed(KeyCode::F10) {
        match state.m16.export() {
            Ok(path) => info!("M16 raw capture saved: {}", path.display()),
            Err(e) => warn!("M16 export: {e}"),
        }
        return;
    }
    if keyboard.just_pressed(KeyCode::F9) {
        let result = std::env::var("SIMTHING_M16_CONFIG")
            .map_err(|e| e.to_string())
            .and_then(|p| std::fs::read(p).map_err(|e| e.to_string()))
            .and_then(|b| serde_json::from_slice::<CaptureConfig>(&b).map_err(|e| e.to_string()))
            .and_then(|c| {
                let observed = facts(&state, window);
                state.m16.start(c, observed)
            });
        if let Err(e) = result {
            warn!("M16 start refused: {e}");
        }
        return;
    }
    if state.m16.active() {
        let observed = facts(&state, window);
        state.m16.runtime(observed);
        state.m16.frame(&time, &strategy);
    }
}
