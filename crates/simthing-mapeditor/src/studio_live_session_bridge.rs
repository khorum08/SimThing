//! STUDIO-LIVE-SESSION-BRIDGE-0 / STUDIO-FIELD-SESSION-ELEVATE-0 —
//! production bridge from Studio clock to SimSession ticks.
//!
//! Consumes scheduled counts from [`crate::StudioSimClock`] and executes them through
//! [`simthing_driver::SimSession::step_once`]. Does not import workshop post-hydration modules,
//! invent gameplay systems, or mutate ScenarioSpec from UI/transport.
//!
//! Session paths:
//! - **structural-shell** (fallback): `SimSession::open` with property maps stripped
//! - **field-bearing** (elevation): `SimSession::open_from_spec` + authored profile
//!   (GameModeSpec / install_targets / session root from clause hydrate)

use std::collections::HashMap;
use std::path::PathBuf;

use simthing_core::{
    DimensionRegistry, OverlayKind, SimProperty, SimThing, SimThingId, SimThingKind, SubFieldRole,
};
use simthing_driver::{Scenario, SessionError, SimSession, StepOnceOutcome};
use simthing_gpu::{
    emit_on_threshold_registrations_to_gpu, EmissionFormula, DEFAULT_THRESHOLD_EMISSION_CAPACITY,
};
use simthing_spec::{
    validate_scenario_links, validate_stead_mapping_consistency, GameModeSpec,
    ResourceEconomyOptInMode, SimThingScenarioSpec,
};

use crate::session::{StudioScenarioSummary, StudioSession};
use crate::studio_disruption_readout::{
    studio_disruption_readout_map_from_session, StudioDisruptionReadoutMap,
};
use crate::studio_fleet_presence::{
    studio_fleet_presence_map_from_session, StudioFleetPresenceMap,
};
use crate::studio_sim_clock::StudioSimClock;

/// Request that a bridge attached to replaced Studio authority be detached before ticking.
pub fn request_live_bridge_reset_after_session_replacement(reset_requested: &mut bool) {
    *reset_requested = true;
}

/// Consume a replacement reset before the caller can advance the live clock.
pub fn apply_live_bridge_reset_before_tick(
    reset_requested: &mut bool,
    bridge: &mut StudioLiveSessionBridge,
) -> bool {
    if !*reset_requested {
        return false;
    }
    bridge.detach();
    *reset_requested = false;
    true
}

/// Operator-visible bridge lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudioLiveSessionBridgeStatus {
    /// No Studio session attached / no live handle.
    Unattached,
    /// Live SimSession open and ready for scheduled ticks.
    Ready,
    /// Clock is playing and bridge is consuming ticks.
    Running,
    /// Clock is paused; live handle may still be open.
    PausedByClock,
    /// Last open/tick failed (message in bridge last_error).
    Errored,
    /// Production path unavailable (e.g. no GPU adapter).
    Unsupported,
}

/// Which production session path the bridge opened (or will open).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StudioLiveSessionPath {
    /// Property-strip structural shell (`SimSession::open`).
    #[default]
    StructuralShell,
    /// Field-bearing authored profile (`SimSession::open_from_spec`).
    FieldBearing,
}

impl StudioLiveSessionPath {
    pub fn label(self) -> &'static str {
        match self {
            Self::StructuralShell => "structural-shell",
            Self::FieldBearing => "field-bearing",
        }
    }

    pub fn production_path_label(self) -> &'static str {
        match self {
            Self::StructuralShell => "simthing_driver::SimSession::open + step_once",
            Self::FieldBearing => "simthing_driver::SimSession::open_from_spec + step_once",
        }
    }
}

/// Operator preference for which path to open.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StudioLiveSessionPathPreference {
    /// Field-bearing when an authored field profile is present; else structural-shell.
    #[default]
    Auto,
    /// Force structural-shell fallback (always selectable).
    StructuralShell,
    /// Force field-bearing (errors if no authored profile).
    FieldBearing,
}

impl StudioLiveSessionPathPreference {
    pub fn label(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::StructuralShell => "structural-shell",
            Self::FieldBearing => "field-bearing",
        }
    }
}

/// Authored live profile retained from clause hydrate (not from ScenarioSpec mutation).
///
/// Elevates the TP-LIVE-RUN-0 workshop residue: `open_from_spec` + authored GameMode /
/// install targets / session root so field economy accretes under production ticks.
#[derive(Debug, Clone)]
pub struct StudioAuthoredLiveProfile {
    pub game_mode: GameModeSpec,
    pub install_targets: HashMap<String, Vec<SimThingId>>,
    /// Runtime tree shell matching `install_targets` (hydrate pack root).
    pub session_root: SimThing,
    pub field_economy_present: bool,
}

impl StudioAuthoredLiveProfile {
    pub fn from_hydrated_pack(
        game_mode: GameModeSpec,
        install_targets: impl IntoIterator<Item = (String, Vec<SimThingId>)>,
        session_root: SimThing,
        field_economy_present: bool,
    ) -> Self {
        Self {
            game_mode,
            install_targets: install_targets.into_iter().collect(),
            session_root,
            field_economy_present,
        }
    }

    /// True when the authored profile carries field-economy / resource-economy opt-in.
    pub fn supports_field_bearing(&self) -> bool {
        if self.field_economy_present {
            return true;
        }
        match self
            .game_mode
            .resource_economy
            .as_ref()
            .map(|economy| economy.opt_in_mode)
        {
            Some(ResourceEconomyOptInMode::Disabled) | None => false,
            Some(
                ResourceEconomyOptInMode::TransferOnly
                | ResourceEconomyOptInMode::EmissionOnly
                | ResourceEconomyOptInMode::TransferAndEmission,
            ) => true,
        }
    }
}

/// One per-tick sample of field accretion (presentation / ops telemetry only).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StudioFieldAccretionSample {
    pub tick_index: u64,
    pub property_key: String,
    pub amount: f32,
    pub decision_events: u32,
}

/// Compact readout for UI / CI (presentation projection only; Send+Sync).
#[derive(Debug, Clone, PartialEq)]
pub struct StudioLiveSessionBridgeReadout {
    pub status: StudioLiveSessionBridgeStatus,
    pub status_label: &'static str,
    pub executed_ticks: u64,
    pub last_scheduled_batch: u64,
    pub last_error: Option<String>,
    pub scenario_id: Option<String>,
    pub stead_valid: Option<bool>,
    pub production_path: &'static str,
    pub session_path: StudioLiveSessionPath,
    pub session_path_label: &'static str,
    pub path_preference_label: &'static str,
    pub field_accretion_samples: Vec<StudioFieldAccretionSample>,
    pub last_decision_event_count: u32,
    pub cumulative_decision_events: u64,
    pub fleet_presence: StudioFleetPresenceMap,
    pub disruption_readout: StudioDisruptionReadoutMap,
}

impl StudioLiveSessionBridgeReadout {
    pub fn default_unattached() -> Self {
        Self {
            status: StudioLiveSessionBridgeStatus::Unattached,
            status_label: status_label(StudioLiveSessionBridgeStatus::Unattached),
            executed_ticks: 0,
            last_scheduled_batch: 0,
            last_error: None,
            scenario_id: None,
            stead_valid: None,
            production_path: StudioLiveSessionPath::StructuralShell.production_path_label(),
            session_path: StudioLiveSessionPath::StructuralShell,
            session_path_label: StudioLiveSessionPath::StructuralShell.label(),
            path_preference_label: StudioLiveSessionPathPreference::Auto.label(),
            field_accretion_samples: Vec::new(),
            last_decision_event_count: 0,
            cumulative_decision_events: 0,
            fleet_presence: StudioFleetPresenceMap::default(),
            disruption_readout: StudioDisruptionReadoutMap::default(),
        }
    }
}

/// Errors from open / tick consumption (never silently swallowed).
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum StudioLiveSessionBridgeError {
    #[error("no StudioSession loaded to open a live bridge")]
    NoStudioSession,
    #[error("live session open failed: {0}")]
    OpenFailed(String),
    #[error("live session tick failed after {executed} executed ticks: {message}")]
    TickFailed { executed: u64, message: String },
    #[error("GPU / adapter unavailable for production SimSession: {0}")]
    Unsupported(String),
    #[error("scenario conversion failed: {0}")]
    ScenarioConversion(String),
    #[error("fleet presence readback failed: {0}")]
    FleetPresenceReadback(String),
    #[error("disruption readout failed: {0}")]
    DisruptionReadback(String),
    #[error("field-bearing path requires an authored live profile")]
    FieldBearingProfileMissing,
    #[error("field-bearing open_from_spec failed: {0}")]
    FieldBearingOpenFailed(String),
}

/// Production live handle over a loaded StudioSession authority.
///
/// `StudioSession.scenario_authority` is Spec presentation authority. The bridge holds an
/// optional [`SimSession`] for execution only; UI/transport never write Spec.
pub struct StudioLiveSessionBridge {
    status: StudioLiveSessionBridgeStatus,
    sim: Option<SimSession>,
    executed_ticks: u64,
    last_scheduled_batch: u64,
    last_error: Option<String>,
    /// Snapshot of Studio authority identity at open (for identity proofs).
    open_scenario_id: Option<String>,
    open_stead_valid: Option<bool>,
    open_links_valid: Option<bool>,
    open_source_path: Option<PathBuf>,
    fleet_presence: StudioFleetPresenceMap,
    disruption_readout: StudioDisruptionReadoutMap,
    /// True if we attempted open and failed for non-GPU reasons.
    open_failed: bool,
    path_preference: StudioLiveSessionPathPreference,
    session_path: StudioLiveSessionPath,
    field_accretion_samples: Vec<StudioFieldAccretionSample>,
    last_decision_event_count: u32,
    cumulative_decision_events: u64,
    /// Resolved emission loci (slot/col) sampled after each tick (field-bearing only).
    sample_loci: Vec<FieldAccretionSampleLocus>,
}

/// Exact GPU locus for one economy emission sample (authoritative materialization).
#[derive(Debug, Clone, PartialEq, Eq)]
struct FieldAccretionSampleLocus {
    property_key: String,
    source_slot: u32,
    source_col: u32,
}

impl Default for StudioLiveSessionBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl StudioLiveSessionBridge {
    pub fn new() -> Self {
        Self {
            status: StudioLiveSessionBridgeStatus::Unattached,
            sim: None,
            executed_ticks: 0,
            last_scheduled_batch: 0,
            last_error: None,
            open_scenario_id: None,
            open_stead_valid: None,
            open_links_valid: None,
            open_source_path: None,
            fleet_presence: StudioFleetPresenceMap::default(),
            disruption_readout: StudioDisruptionReadoutMap::default(),
            open_failed: false,
            path_preference: StudioLiveSessionPathPreference::Auto,
            session_path: StudioLiveSessionPath::StructuralShell,
            field_accretion_samples: Vec::new(),
            last_decision_event_count: 0,
            cumulative_decision_events: 0,
            sample_loci: Vec::new(),
        }
    }

    pub fn status(&self) -> StudioLiveSessionBridgeStatus {
        self.status
    }

    pub fn executed_ticks(&self) -> u64 {
        self.executed_ticks
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    pub fn is_attached(&self) -> bool {
        self.sim.is_some()
    }

    pub fn session_path(&self) -> StudioLiveSessionPath {
        self.session_path
    }

    pub fn path_preference(&self) -> StudioLiveSessionPathPreference {
        self.path_preference
    }

    /// Select structural-shell vs field-bearing (selectable fallback retained).
    pub fn set_path_preference(&mut self, preference: StudioLiveSessionPathPreference) {
        self.path_preference = preference;
    }

    pub fn production_path_label() -> &'static str {
        // Default label for structural-shell (legacy callers / unit proofs).
        StudioLiveSessionPath::StructuralShell.production_path_label()
    }

    /// Detach live handle (keeps StudioSession authority untouched).
    pub fn detach(&mut self) {
        self.sim = None;
        self.status = StudioLiveSessionBridgeStatus::Unattached;
        self.executed_ticks = 0;
        self.last_scheduled_batch = 0;
        self.last_error = None;
        self.open_scenario_id = None;
        self.open_stead_valid = None;
        self.open_links_valid = None;
        self.open_source_path = None;
        self.fleet_presence = StudioFleetPresenceMap::default();
        self.disruption_readout = StudioDisruptionReadoutMap::default();
        self.open_failed = false;
        self.session_path = StudioLiveSessionPath::StructuralShell;
        self.field_accretion_samples.clear();
        self.last_decision_event_count = 0;
        self.cumulative_decision_events = 0;
        self.sample_loci.clear();
    }

    /// Resolve which path to open given preference + optional authored profile.
    pub fn resolve_session_path(
        preference: StudioLiveSessionPathPreference,
        profile: Option<&StudioAuthoredLiveProfile>,
    ) -> Result<StudioLiveSessionPath, StudioLiveSessionBridgeError> {
        match preference {
            StudioLiveSessionPathPreference::StructuralShell => {
                Ok(StudioLiveSessionPath::StructuralShell)
            }
            StudioLiveSessionPathPreference::FieldBearing => {
                if profile.map(|p| p.supports_field_bearing()).unwrap_or(false) {
                    Ok(StudioLiveSessionPath::FieldBearing)
                } else {
                    Err(StudioLiveSessionBridgeError::FieldBearingProfileMissing)
                }
            }
            StudioLiveSessionPathPreference::Auto => {
                if profile.map(|p| p.supports_field_bearing()).unwrap_or(false) {
                    Ok(StudioLiveSessionPath::FieldBearing)
                } else {
                    Ok(StudioLiveSessionPath::StructuralShell)
                }
            }
        }
    }

    /// Open / re-open live SimSession from loaded Studio authority.
    pub fn open_from_loaded_studio_session(
        &mut self,
        studio: &StudioSession,
    ) -> Result<(), StudioLiveSessionBridgeError> {
        self.sim = None;
        self.executed_ticks = 0;
        self.last_scheduled_batch = 0;
        self.open_failed = false;
        self.field_accretion_samples.clear();
        self.last_decision_event_count = 0;
        self.cumulative_decision_events = 0;
        self.sample_loci.clear();

        let path = Self::resolve_session_path(
            self.path_preference,
            studio.authored_live_profile.as_ref(),
        )?;
        self.session_path = path;

        let fleet_presence = studio_fleet_presence_map_from_session(studio)
            .map_err(|e| StudioLiveSessionBridgeError::FleetPresenceReadback(e.to_string()))?;
        let disruption_readout = studio_disruption_readout_map_from_session(studio)
            .map_err(|e| StudioLiveSessionBridgeError::DisruptionReadback(e.to_string()))?;

        let open_result = match path {
            StudioLiveSessionPath::StructuralShell => {
                let scenario = driver_scenario_from_authority(&studio.scenario_authority)
                    .map_err(StudioLiveSessionBridgeError::ScenarioConversion)?;
                SimSession::open(scenario)
            }
            StudioLiveSessionPath::FieldBearing => {
                let profile = studio.authored_live_profile.as_ref().ok_or(
                    StudioLiveSessionBridgeError::FieldBearingProfileMissing,
                )?;
                let scenario = driver_scenario_field_bearing_from_profile(profile)
                    .map_err(StudioLiveSessionBridgeError::ScenarioConversion)?;
                // Field-bearing open installs RF/resource-economy + property surfaces only.
                // Combat/event/capability trees remain out of this path (they re-root install
                // and are not required for 12.8 field accretion under live ticks).
                let field_mode = field_bearing_game_mode(&profile.game_mode);
                match SimSession::open_from_spec(scenario, &field_mode) {
                    Ok(mut sim) => {
                        if let Err(e) = ensure_resource_economy_threshold_ops(&mut sim) {
                            return Err(StudioLiveSessionBridgeError::FieldBearingOpenFailed(e));
                        }
                        // Authored Constant seeds are initial state only — not a time-zero
                        // decision. Decision counts accumulate from live step_once only.
                        // Bind telemetry to materialized emission source_slot/source_col.
                        self.sample_loci = emission_sample_loci_from_session(&sim);
                        // Tick-0 sample: open-time field state before the first step_once so
                        // the OVL table can show open→live deltas (not only post-tick plateaus).
                        self.field_accretion_samples =
                            collect_field_accretion_sample(&sim, &self.sample_loci, 0);
                        Ok(sim)
                    }
                    Err(e) => Err(e),
                }
            }
        };

        match open_result {
            Ok(sim) => {
                self.sim = Some(sim);
                self.status = StudioLiveSessionBridgeStatus::Ready;
                self.last_error = None;
                self.open_scenario_id = Some(studio.scenario_authority.scenario_id.clone());
                self.open_stead_valid = Some(studio.scenario_summary.stead_valid);
                self.open_links_valid = Some(studio.scenario_summary.links_valid);
                self.open_source_path = studio.scenario_path.clone();
                self.fleet_presence = fleet_presence;
                self.disruption_readout = disruption_readout;
                Ok(())
            }
            Err(SessionError::Gpu(e)) => {
                self.status = StudioLiveSessionBridgeStatus::Unsupported;
                let msg = e.to_string();
                self.last_error = Some(msg.clone());
                self.open_failed = true;
                Err(StudioLiveSessionBridgeError::Unsupported(msg))
            }
            Err(e) => {
                self.status = StudioLiveSessionBridgeStatus::Errored;
                let msg = e.to_string();
                self.last_error = Some(msg.clone());
                self.open_failed = true;
                match path {
                    StudioLiveSessionPath::FieldBearing => {
                        Err(StudioLiveSessionBridgeError::FieldBearingOpenFailed(msg))
                    }
                    StudioLiveSessionPath::StructuralShell => {
                        Err(StudioLiveSessionBridgeError::OpenFailed(msg))
                    }
                }
            }
        }
    }

    /// Lazy open: if unattached and studio present, open; surface errors (no silent TP fallback).
    pub fn ensure_open(
        &mut self,
        studio: Option<&StudioSession>,
    ) -> Result<(), StudioLiveSessionBridgeError> {
        if self.sim.is_some() {
            return Ok(());
        }
        let Some(studio) = studio else {
            return Err(StudioLiveSessionBridgeError::NoStudioSession);
        };
        self.open_from_loaded_studio_session(studio)
    }

    /// Consume `scheduled` ticks from the clock via production `step_once`.
    ///
    /// Failed ticks do **not** increment `executed_ticks`. Stops on first error.
    pub fn consume_scheduled_ticks(
        &mut self,
        scheduled: u64,
    ) -> Result<u64, StudioLiveSessionBridgeError> {
        self.last_scheduled_batch = scheduled;
        if scheduled == 0 {
            return Ok(0);
        }
        if self.sim.is_none() {
            return Err(StudioLiveSessionBridgeError::NoStudioSession);
        }

        let field_bearing = self.session_path == StudioLiveSessionPath::FieldBearing;
        let sample_loci = self.sample_loci.clone();
        let mut ran = 0u64;
        for _ in 0..scheduled {
            let step_result = self
                .sim
                .as_mut()
                .expect("sim present")
                .step_once();
            match step_result {
                Ok(StepOnceOutcome { ticks_run, .. }) => {
                    let step_ticks = ticks_run.max(1);
                    ran = ran.saturating_add(step_ticks);
                    self.executed_ticks = self.executed_ticks.saturating_add(step_ticks);
                    // Re-read AccumulatorOp threshold emissions left by the production tick
                    // (existing public runtime API — no driver/kernel seam widening).
                    let threshold_event_count = {
                        let sim = self.sim.as_mut().expect("sim present");
                        last_tick_threshold_event_count(sim)
                    };
                    self.last_decision_event_count = threshold_event_count;
                    self.cumulative_decision_events = self
                        .cumulative_decision_events
                        .saturating_add(threshold_event_count as u64);
                    if field_bearing {
                        let mut sample = {
                            let sim = self.sim.as_ref().expect("sim present");
                            collect_field_accretion_sample(sim, &sample_loci, self.executed_ticks)
                        };
                        if let Some(first) = sample.first_mut() {
                            first.decision_events = threshold_event_count;
                        } else if threshold_event_count > 0 {
                            sample.push(StudioFieldAccretionSample {
                                tick_index: self.executed_ticks,
                                property_key: "decision".into(),
                                amount: 0.0,
                                decision_events: threshold_event_count,
                            });
                        }
                        self.field_accretion_samples.extend(sample);
                        const MAX_SAMPLES: usize = 16;
                        if self.field_accretion_samples.len() > MAX_SAMPLES {
                            let drop_n = self.field_accretion_samples.len() - MAX_SAMPLES;
                            self.field_accretion_samples.drain(0..drop_n);
                        }
                    }
                }
                Err(e) => {
                    self.status = StudioLiveSessionBridgeStatus::Errored;
                    let message = e.to_string();
                    self.last_error = Some(message.clone());
                    return Err(StudioLiveSessionBridgeError::TickFailed {
                        executed: self.executed_ticks,
                        message,
                    });
                }
            }
        }
        self.last_error = None;
        Ok(ran)
    }

    /// Drive bridge from wall elapsed + clock + optional studio session (Play path).
    ///
    /// Returns executed ticks this call. Does not mutate ScenarioSpec.
    pub fn tick_from_clock(
        &mut self,
        clock: &mut StudioSimClock,
        studio: Option<&StudioSession>,
        elapsed_secs: f64,
    ) -> Result<u64, StudioLiveSessionBridgeError> {
        if clock.is_paused() {
            if self.sim.is_some() {
                self.status = StudioLiveSessionBridgeStatus::PausedByClock;
            }
            self.last_scheduled_batch = 0;
            return Ok(0);
        }

        match self.ensure_open(studio) {
            Ok(()) => {}
            Err(StudioLiveSessionBridgeError::Unsupported(_)) => {
                // Adapter missing: still advance clock accounting so UI shows schedule,
                // but do not pretend live ticks executed.
                let _ = clock.advance(elapsed_secs);
                return Ok(0);
            }
            Err(e) => return Err(e),
        }

        let scheduled = clock.advance(elapsed_secs);
        if scheduled == 0 {
            self.status = StudioLiveSessionBridgeStatus::Running;
            return Ok(0);
        }
        self.status = StudioLiveSessionBridgeStatus::Running;
        self.consume_scheduled_ticks(scheduled)
    }

    pub fn readout(&self) -> StudioLiveSessionBridgeReadout {
        StudioLiveSessionBridgeReadout {
            status: self.status,
            status_label: status_label(self.status),
            executed_ticks: self.executed_ticks,
            last_scheduled_batch: self.last_scheduled_batch,
            last_error: self.last_error.clone(),
            scenario_id: self.open_scenario_id.clone(),
            stead_valid: self.open_stead_valid,
            production_path: self.session_path.production_path_label(),
            session_path: self.session_path,
            session_path_label: self.session_path.label(),
            path_preference_label: self.path_preference.label(),
            field_accretion_samples: self.field_accretion_samples.clone(),
            last_decision_event_count: self.last_decision_event_count,
            cumulative_decision_events: self.cumulative_decision_events,
            fleet_presence: self.fleet_presence.clone(),
            disruption_readout: self.disruption_readout.clone(),
        }
    }

    /// Identity snapshot for STEAD/session hold proofs (from open, not re-derived from SimSession).
    pub fn open_identity(&self) -> Option<BridgeOpenIdentity> {
        Some(BridgeOpenIdentity {
            scenario_id: self.open_scenario_id.clone()?,
            stead_valid: self.open_stead_valid?,
            links_valid: self.open_links_valid?,
            source_path: self.open_source_path.clone(),
        })
    }

    /// Access live session for headless multi-tick proofs (field samples).
    pub fn sim_session(&self) -> Option<&SimSession> {
        self.sim.as_ref()
    }

    pub fn sim_session_mut(&mut self) -> Option<&mut SimSession> {
        self.sim.as_mut()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BridgeOpenIdentity {
    pub scenario_id: String,
    pub stead_valid: bool,
    pub links_valid: bool,
    pub source_path: Option<PathBuf>,
}

fn status_label(s: StudioLiveSessionBridgeStatus) -> &'static str {
    match s {
        StudioLiveSessionBridgeStatus::Unattached => "unattached",
        StudioLiveSessionBridgeStatus::Ready => "ready",
        StudioLiveSessionBridgeStatus::Running => "running",
        StudioLiveSessionBridgeStatus::PausedByClock => "paused",
        StudioLiveSessionBridgeStatus::Errored => "errored",
        StudioLiveSessionBridgeStatus::Unsupported => "unsupported",
    }
}

/// Lower Studio authority Spec into a structural driver [`Scenario`] (no GameMode/RF attach).
///
/// ScenarioSpec property ids are stable product identifiers (e.g. 8_300_000), not dense
/// registry indices. For structural live ticks we clone the tree and **strip property maps**
/// so GPU projection does not index sparse product IDs into a dense registry. Structure
/// (kind / id / children) is preserved; Spec authority remains on StudioSession unchanged.
pub fn driver_scenario_from_authority(spec: &SimThingScenarioSpec) -> Result<Scenario, String> {
    let mut registry = DimensionRegistry::new();
    // Seed column so total_columns >= 1 for GPU buffer shape (structural shell).
    let _ = registry.register(SimProperty::simple("_studio_live_bridge", "seed", 0));

    let mut root = spec.root.clone();
    strip_property_maps(&mut root);

    let mut n_slots = 0u32;
    count_tree_nodes(&root, &mut n_slots);
    n_slots = n_slots.max(1).saturating_mul(2).max(16);

    Ok(Scenario {
        name: if spec.scenario_id.is_empty() {
            "studio_live_bridge".into()
        } else {
            spec.scenario_id.clone()
        },
        ticks_per_day: 1,
        max_days: 1_000_000,
        dt: 1.0,
        n_slots,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::new(),
    })
}

/// Project a GameModeSpec onto the field-bearing install surface (RF/economy only).
///
/// Drops combat/event/capability/resource-flow trees that re-root install and are not
/// required for authored field accretion under `open_from_spec`.
pub fn field_bearing_game_mode(mode: &GameModeSpec) -> GameModeSpec {
    let mut field = mode.clone();
    field.events.clear();
    field.capability_trees.clear();
    // Drop non-field domain packs (combat/etc.). Field-economy overlays live on
    // game_mode.overlays, but install only applies DomainPackSpec::overlays
    // (envelope overlays are ADR-deferred). Elevate only field-economy overlay
    // kinds (Policy/Crisis/Infrastructure) so payload/combat overlays with
    // non-namespace targets stay out of the field-bearing install path.
    let field_overlays: Vec<_> = std::mem::take(&mut field.overlays)
        .into_iter()
        .filter(|o| {
            // Field-economy overlays always use `namespace::name` targets and the
            // Policy/Crisis/Infrastructure kinds from the 12.6 lowerer.
            o.targets_property.contains("::")
                && matches!(
                    o.kind,
                    OverlayKind::Policy | OverlayKind::Crisis | OverlayKind::Infrastructure
                )
        })
        .collect();
    field.domain_packs.clear();
    if !field_overlays.is_empty() {
        field.domain_packs.push(simthing_spec::DomainPackSpec {
            id: "field_bearing_overlays".into(),
            display_name: "Field-bearing overlays".into(),
            metadata: Default::default(),
            properties: Vec::new(),
            overlays: field_overlays,
            capability_trees: Vec::new(),
            events: Vec::new(),
        });
    }
    field.resource_flow = None;
    field.region_fields.clear();
    field
}

/// Build a field-bearing driver [`Scenario`] from the authored live profile.
///
/// Uses the hydrate pack session root + install targets so RF/overlay ScenarioListed
/// targets resolve. Property maps on the root are stripped; `open_from_spec` reinstalls
/// authored properties/overlays from GameModeSpec through the generic install path.
pub fn driver_scenario_field_bearing_from_profile(
    profile: &StudioAuthoredLiveProfile,
) -> Result<Scenario, String> {
    let mut registry = DimensionRegistry::new();
    let _ = registry.register(SimProperty::simple("_studio_live_bridge", "seed", 0));

    let mut root = profile.session_root.clone();
    strip_property_maps(&mut root);

    let mut n_slots = 0u32;
    count_tree_nodes(&root, &mut n_slots);
    n_slots = n_slots.max(1).saturating_mul(2).max(16);

    Ok(Scenario {
        name: if profile.game_mode.id.is_empty() {
            "studio_field_bearing".into()
        } else {
            profile.game_mode.id.clone()
        },
        ticks_per_day: 1,
        max_days: 1_000_000,
        dt: 1.0,
        n_slots,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: profile.install_targets.clone(),
    })
}

/// Open-time wiring for field-bearing sessions:
/// 1. Seed property columns from authored Constant emission formulas (initial state only)
/// 2. Upload authored `emit_on_threshold` registrations through the generic GPU threshold bridge
///
/// Not per-tick economy logic — install/profile residue elevation only. Does **not**
/// fabricate an open-time decision edge; decisions are observed only after live `step_once`.
fn ensure_resource_economy_threshold_ops(session: &mut SimSession) -> Result<(), String> {
    materialize_authored_constant_emission_seeds(session)?;
    let Some(registry) = session.spec_state.resource_economy_registry.as_ref() else {
        return Ok(());
    };
    if registry.registrations.emit_on_threshold.is_empty() {
        return Ok(());
    }
    let gpu_regs =
        emit_on_threshold_registrations_to_gpu(&registry.registrations.emit_on_threshold);
    session
        .state
        .ensure_threshold_accumulator(DEFAULT_THRESHOLD_EMISSION_CAPACITY);
    session
        .state
        .upload_accumulator_threshold_ops(&gpu_regs)
        .map_err(|e| format!("upload emit_on_threshold: {e}"))?;
    Ok(())
}

/// Count sealed AccumulatorOp threshold events from the most recent production tick.
/// Uses the existing public runtime readback (events remain until the next prepare).
fn last_tick_threshold_event_count(sim: &mut SimSession) -> u32 {
    let Some(runtime) = sim.state.accumulator_runtime.as_mut() else {
        return 0;
    };
    runtime
        .readback_threshold_events(&sim.state.ctx)
        .map(|events| events.len() as u32)
        .unwrap_or(0)
}

fn materialize_authored_constant_emission_seeds(session: &mut SimSession) -> Result<(), String> {
    let Some(registry) = session.spec_state.resource_economy_registry.as_ref() else {
        return Ok(());
    };
    let n_dims = session.state.n_dims as usize;
    let mut values = session.state.read_values();
    let mut prev = values.clone();
    let mut any = false;
    for emission in &registry.registrations.emissions {
        let EmissionFormula::Constant { value } = emission.formula else {
            continue;
        };
        if !value.is_finite() {
            continue;
        }
        let idx = emission.source_slot as usize * n_dims + emission.source_col as usize;
        if let Some(slot) = values.get_mut(idx) {
            // Authored constant is the emission's field seed (overlay-coupled initial pressure).
            *slot = (*slot).max(value);
            any = true;
        }
        // Previous stays at 0 (or lower) so Rising thresholds can observe a first-tick crossing.
        if let Some(slot) = prev.get_mut(idx) {
            *slot = 0.0;
        }
    }
    if !any {
        return Ok(());
    }
    session
        .state
        .install_resolved_values_at_boundary(&values);
    session
        .state
        .install_resolved_previous_values_at_boundary(&prev);
    Ok(())
}

/// Build telemetry loci from **materialized** emission registrations (slot/col authority).
fn emission_sample_loci_from_session(sim: &SimSession) -> Vec<FieldAccretionSampleLocus> {
    let Some(registry) = sim.spec_state.resource_economy_registry.as_ref() else {
        return Vec::new();
    };
    let reg = &sim.proto.registry;
    let mut out = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for emission in &registry.registrations.emissions {
        let key = (emission.source_slot, emission.source_col);
        if !seen.insert(key) {
            continue;
        }
        let property_key = property_key_for_col(reg, emission.source_col)
            .unwrap_or_else(|| format!("col:{}", emission.source_col));
        out.push(FieldAccretionSampleLocus {
            property_key,
            source_slot: emission.source_slot,
            source_col: emission.source_col,
        });
    }
    out
}

fn property_key_for_col(reg: &DimensionRegistry, col: u32) -> Option<String> {
    let (pid, _) = reg.column_owners.get(col as usize).copied()?;
    let prop = reg.try_property(pid)?;
    Some(format!("{}::{}", prop.namespace, prop.name))
}

fn collect_field_accretion_sample(
    sim: &SimSession,
    sample_loci: &[FieldAccretionSampleLocus],
    tick_index: u64,
) -> Vec<StudioFieldAccretionSample> {
    let values = sim.state.read_values();
    let n_dims = sim.state.n_dims as usize;
    let mut samples = Vec::new();
    for locus in sample_loci {
        let idx = locus.source_slot as usize * n_dims + locus.source_col as usize;
        let Some(&amount) = values.get(idx) else {
            continue;
        };
        samples.push(StudioFieldAccretionSample {
            tick_index,
            property_key: locus.property_key.clone(),
            amount,
            // Filled by the caller from last-tick sealed threshold count.
            decision_events: 0,
        });
    }
    samples
}

fn strip_property_maps(node: &mut simthing_core::SimThing) {
    node.properties.clear();
    for child in &mut node.children {
        strip_property_maps(child);
    }
}

fn count_tree_nodes(node: &simthing_core::SimThing, n: &mut u32) {
    *n = n.saturating_add(1);
    for child in &node.children {
        count_tree_nodes(child, n);
    }
}

/// Compare Studio summary identity fields for multi-tick hold proofs.
pub fn studio_summary_identity_eq(a: &StudioScenarioSummary, b: &StudioScenarioSummary) -> bool {
    a.scenario_id == b.scenario_id
        && a.system_count == b.system_count
        && a.link_count == b.link_count
        && a.grid_width == b.grid_width
        && a.grid_height == b.grid_height
        && a.occupied_cells == b.occupied_cells
        && a.stead_valid == b.stead_valid
        && a.links_valid == b.links_valid
}

/// Re-validate STEAD/links on Spec (authority unchanged by bridge ticks).
pub fn revalidate_authority_stead(spec: &SimThingScenarioSpec) -> (bool, bool) {
    (
        validate_stead_mapping_consistency(spec).is_ok(),
        validate_scenario_links(spec).is_ok(),
    )
}

/// Static proof: this module has no production import of workshop live-run residue.
pub fn bridge_module_source_forbids_workshop_residue() -> bool {
    let src = include_str!("studio_live_session_bridge.rs");
    // Contiguous import patterns (split so this function body does not self-match).
    let a = ["simthing_", "workshop"];
    let b = ["post_", "hydration"];
    let t1 = format!("{}{}", a[0], a[1]);
    let t2 = format!("{}{}", b[0], b[1]);
    for line in src.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("//") || trimmed.starts_with("//!") || trimmed.starts_with("///") {
            continue;
        }
        if trimmed.contains("bridge_module_source_forbids") {
            continue;
        }
        if (trimmed.contains("use ") || trimmed.contains("::"))
            && (trimmed.contains(&t1) || trimmed.contains(&t2))
        {
            return false;
        }
    }
    true
}

/// Build an authored live profile from a hydrated scenario pack (production elevates this).
///
/// Field-economy owner_policy overlays install via `ScenarioListed { owner_key }`.
/// Hydrate location trees alone do not always register owner keys on
/// `install_targets`; inject owner shells so the existing domain-pack overlay
/// install path can resolve them without bespoke tick logic.
pub fn authored_live_profile_from_pack(
    pack: &simthing_clausething::HydratedScenarioPack,
) -> StudioAuthoredLiveProfile {
    let mut root = pack.root.clone();
    let mut install_targets: HashMap<String, Vec<SimThingId>> = pack
        .install_targets
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    for owner in &pack.owners {
        if install_targets.contains_key(&owner.owner_key) {
            continue;
        }
        let mut shell = SimThing::new(SimThingKind::Custom("Owner".into()), 0);
        // Prefer hydrate's stable owner id when present so later joins stay coherent.
        if owner.simthing_id.raw() != 0 {
            shell.id = owner.simthing_id;
        }
        let id = shell.id;
        root.add_child(shell);
        install_targets.insert(owner.owner_key.clone(), vec![id]);
    }
    StudioAuthoredLiveProfile::from_hydrated_pack(
        pack.game_mode.clone(),
        install_targets,
        root,
        pack.field_economy.is_some(),
    )
}

#[cfg(test)]
mod unit_smoke {
    use super::*;

    #[test]
    fn default_unattached() {
        let b = StudioLiveSessionBridge::new();
        assert_eq!(b.status(), StudioLiveSessionBridgeStatus::Unattached);
        assert_eq!(b.executed_ticks(), 0);
        assert_eq!(b.session_path(), StudioLiveSessionPath::StructuralShell);
        assert!(bridge_module_source_forbids_workshop_residue());
    }

    #[test]
    fn driver_scenario_from_vertical_seed() {
        let spec = crate::runtime_vertical_seed_scenario_spec();
        let sc = driver_scenario_from_authority(&spec).expect("convert");
        assert_eq!(sc.name, spec.scenario_id);
        assert!(sc.n_slots >= 16);
        assert_eq!(sc.root.kind, spec.root.kind);
    }

    #[test]
    fn resolve_path_auto_prefers_field_bearing_when_profile_supports() {
        let mut mode = GameModeSpec::default();
        mode.id = "probe".into();
        mode.resource_economy = Some(simthing_spec::ResourceEconomySpec {
            opt_in_mode: ResourceEconomyOptInMode::TransferAndEmission,
            ..Default::default()
        });
        let profile = StudioAuthoredLiveProfile::from_hydrated_pack(
            mode,
            std::iter::empty(),
            SimThing::new(simthing_core::SimThingKind::World, 0),
            true,
        );
        let path = StudioLiveSessionBridge::resolve_session_path(
            StudioLiveSessionPathPreference::Auto,
            Some(&profile),
        )
        .expect("resolve");
        assert_eq!(path, StudioLiveSessionPath::FieldBearing);
    }

    #[test]
    fn resolve_path_structural_shell_selectable_even_with_profile() {
        let mut mode = GameModeSpec::default();
        mode.resource_economy = Some(simthing_spec::ResourceEconomySpec {
            opt_in_mode: ResourceEconomyOptInMode::TransferAndEmission,
            ..Default::default()
        });
        let profile = StudioAuthoredLiveProfile::from_hydrated_pack(
            mode,
            std::iter::empty(),
            SimThing::new(simthing_core::SimThingKind::World, 0),
            true,
        );
        let path = StudioLiveSessionBridge::resolve_session_path(
            StudioLiveSessionPathPreference::StructuralShell,
            Some(&profile),
        )
        .expect("resolve");
        assert_eq!(path, StudioLiveSessionPath::StructuralShell);
    }
}
