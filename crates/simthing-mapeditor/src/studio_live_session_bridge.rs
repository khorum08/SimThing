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

use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, DimensionRegistry, LogTier,
    OverlayKind, SimProperty, SimThing, SimThingId, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{
    resolve_node_columns, system_id_by_host_raw_from_structural_authority, GpuValuesSnapshot,
    LiveDisruptionAuthorityReadback, Scenario, SessionError, SimSession, StepOnceOutcome,
};
use simthing_gpu::SlotAllocator;
use simthing_spec::{
    compile_property, disruption_readout_snapshot_with_readback, game_session_child,
    game_session_owners, owner_entity_id, planet_child_rf_participant_inputs,
    validate_scenario_links, validate_stead_mapping_consistency, ArenaSpec,
    BaseFlowDirectionSpec, BaseFlowObligationSpec, ExplicitParticipantSpec, FissionPolicySpec,
    GameModeSpec, InstallTargetSpec, PropertyKey, PropertySpec, ResourceEconomyOptInMode,
    ResourceFlowExecutionProfile, ResourceFlowOptInMode, ResourceFlowSpec, SimThingScenarioSpec,
};

use crate::session::{StudioScenarioSummary, StudioSession};
use crate::studio_disruption_readout::{
    studio_disruption_readout_map_from_session, studio_disruption_readout_map_from_snapshot,
    StudioDisruptionReadoutMap,
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
    /// Canonical recursive Arena RF projection, when the authority tree exposes
    /// an admitted Owner plus at least three authored producer children.
    pub recursive_rf: Option<StudioRecursiveRfProfile>,
    /// Authored location id → generated system id from STEAD (row,col) join to
    /// the embedded lattice / rebound Spec placements.
    pub location_system_ids: BTreeMap<String, u32>,
}

/// Stable labels/identities for the live RF locus shown by Studio and asserted by CI.
#[derive(Debug, Clone, PartialEq)]
pub struct StudioRecursiveRfProfile {
    pub arena: String,
    pub property_namespace: String,
    pub property_name: String,
    pub named_child_label: String,
    pub named_child_id: SimThingId,
    pub ancestor_label: String,
    pub ancestor_id: SimThingId,
    pub session_root_id: SimThingId,
    pub sibling_count: u32,
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
            recursive_rf: None,
            location_system_ids: BTreeMap::new(),
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

/// Actual GPU-backed recursive RF values. Emission rows are deliberately separate diagnostics.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StudioRecursiveRfReadout {
    pub active: bool,
    pub execution_profile: &'static str,
    pub arena: Option<String>,
    pub named_child: Option<String>,
    pub ancestor: Option<String>,
    pub sibling_count: u32,
    pub ancestor_aggregate_before: Option<f32>,
    pub ancestor_aggregate_after: Option<f32>,
    pub root_balance_before: Option<f32>,
    pub root_balance_after: Option<f32>,
    /// RF-5 admitted need / weight_profile transport (read-only).
    pub need_profile_id: Option<String>,
    pub need_profile_kind: Option<String>,
    pub need_weight_values: Option<String>,
    pub need_live_value: Option<f32>,
    pub need_threshold: Option<f32>,
    pub need_threshold_result: Option<&'static str>,
    pub need_threshold_event_count: u32,
    /// Cumulative sealed crossings for the admitted construction/manufacturing need
    /// binding's exact `event_kind` (not global FIELD_POLICY noise).
    pub cumulative_construction_crossings: u64,
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
    pub cumulative_construction_crossings: u64,
    pub fleet_presence: StudioFleetPresenceMap,
    pub disruption_readout: StudioDisruptionReadoutMap,
    pub recursive_rf: StudioRecursiveRfReadout,
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
            cumulative_construction_crossings: 0,
            fleet_presence: StudioFleetPresenceMap::default(),
            disruption_readout: StudioDisruptionReadoutMap::default(),
            recursive_rf: StudioRecursiveRfReadout::default(),
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
    #[error("threshold event readback failed after {executed} executed ticks: {message}")]
    ThresholdReadbackFailed { executed: u64, message: String },
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
    cumulative_construction_crossings: u64,
    /// Resolved emission loci (slot/col) sampled after each tick (field-bearing only).
    sample_loci: Vec<FieldAccretionSampleLocus>,
    /// Exact RF aggregate/Balance cells resolved from the installed Arena registry.
    recursive_rf_locus: Option<RecursiveRfSampleLocus>,
    recursive_rf_readout: StudioRecursiveRfReadout,
    /// Spec authority retained for live disruption snapshot refresh.
    readout_authority: Option<SimThingScenarioSpec>,
    /// Authored location → system_id join captured at open from the live profile.
    location_system_ids: BTreeMap<String, u32>,
}

/// Exact GPU locus for one economy emission sample (authoritative materialization).
#[derive(Debug, Clone, PartialEq, Eq)]
struct FieldAccretionSampleLocus {
    property_key: String,
    source_slot: u32,
    source_col: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RecursiveRfSampleLocus {
    ancestor_slot: u32,
    aggregate_col: u32,
    root_slot: u32,
    balance_col: u32,
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
            cumulative_construction_crossings: 0,
            sample_loci: Vec::new(),
            recursive_rf_locus: None,
            recursive_rf_readout: StudioRecursiveRfReadout::default(),
            readout_authority: None,
            location_system_ids: BTreeMap::new(),
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
        self.cumulative_construction_crossings = 0;
        self.sample_loci.clear();
        self.recursive_rf_locus = None;
        self.recursive_rf_readout = StudioRecursiveRfReadout::default();
        self.readout_authority = None;
        self.location_system_ids.clear();
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
        self.cumulative_construction_crossings = 0;
        self.sample_loci.clear();
        self.recursive_rf_locus = None;
        self.recursive_rf_readout = StudioRecursiveRfReadout::default();
        self.readout_authority = None;
        self.location_system_ids.clear();
        self.disruption_readout = StudioDisruptionReadoutMap::default();

        let path = Self::resolve_session_path(
            self.path_preference,
            studio.authored_live_profile.as_ref(),
        )?;
        self.session_path = path;

        let fleet_presence = studio_fleet_presence_map_from_session(studio)
            .map_err(|e| StudioLiveSessionBridgeError::FleetPresenceReadback(e.to_string()))?;

        let open_result = match path {
            StudioLiveSessionPath::StructuralShell => {
                let scenario = driver_scenario_from_authority(&studio.scenario_authority)
                    .map_err(StudioLiveSessionBridgeError::ScenarioConversion)?;
                SimSession::open(scenario)
            }
            StudioLiveSessionPath::FieldBearing => {
                let profile = studio
                    .authored_live_profile
                    .as_ref()
                    .ok_or(StudioLiveSessionBridgeError::FieldBearingProfileMissing)?;
                let scenario = driver_scenario_field_bearing_from_profile(profile)
                    .map_err(StudioLiveSessionBridgeError::ScenarioConversion)?;
                // Field-bearing open installs RF/resource-economy + property surfaces only.
                // Combat/event/capability trees remain out of this path (they re-root install
                // and are not required for 12.8 field accretion under live ticks).
                let field_mode = field_bearing_game_mode(&profile.game_mode);
                match SimSession::open_from_spec(scenario, &field_mode) {
                    Ok(sim) => {
                        // Bind telemetry to admitted emission source_slot/source_col. Values
                        // become live only through ordinary production GPU execution.
                        self.sample_loci = emission_sample_loci_from_session(&sim);
                        let snapshot = GpuValuesSnapshot::from_session(&sim);
                        // Tick-0 sample: open-time field state before the first step_once so
                        // the OVL table can show open→live deltas (not only post-tick plateaus).
                        self.field_accretion_samples = collect_field_accretion_sample_from_snapshot(
                            &snapshot,
                            &self.sample_loci,
                            0,
                        );
                        if let Some(rf_profile) = profile.recursive_rf.as_ref() {
                            let (locus, readout) = recursive_rf_locus_from_session(
                                &sim, rf_profile,
                            )
                            .map_err(StudioLiveSessionBridgeError::FieldBearingOpenFailed)?;
                            self.recursive_rf_locus = Some(locus);
                            self.recursive_rf_readout = readout;
                        }
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
                self.readout_authority = Some(studio.scenario_authority.clone());
                self.location_system_ids = studio
                    .authored_live_profile
                    .as_ref()
                    .map(|profile| profile.location_system_ids.clone())
                    .unwrap_or_default();
                if path == StudioLiveSessionPath::FieldBearing {
                    self.refresh_live_disruption_readout()?;
                } else {
                    self.disruption_readout = studio_disruption_readout_map_from_session(studio)
                        .map_err(|e| {
                            StudioLiveSessionBridgeError::DisruptionReadback(e.to_string())
                        })?;
                }
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
            let step_result = self.sim.as_mut().expect("sim present").step_once();
            match step_result {
                Ok(StepOnceOutcome { ticks_run, .. }) => {
                    let step_ticks = ticks_run.max(1);
                    ran = ran.saturating_add(step_ticks);
                    self.executed_ticks = self.executed_ticks.saturating_add(step_ticks);
                    // Re-read AccumulatorOp threshold emissions left by the production tick
                    // (existing public runtime API — no driver/kernel seam widening).
                    let threshold_observation = {
                        let sim = self.sim.as_mut().expect("sim present");
                        last_tick_threshold_event_kinds(sim)
                    };
                    let threshold_event_kinds =
                        self.apply_threshold_event_observation(threshold_observation)?;
                    let threshold_event_count = threshold_event_kinds.len() as u32;
                    self.last_decision_event_count = threshold_event_count;
                    self.cumulative_decision_events = self
                        .cumulative_decision_events
                        .saturating_add(threshold_event_count as u64);
                    if field_bearing {
                        let snapshot = {
                            let sim = self.sim.as_ref().expect("sim present");
                            GpuValuesSnapshot::from_session(sim)
                        };
                        let mut sample = collect_field_accretion_sample_from_snapshot(
                            &snapshot,
                            &sample_loci,
                            self.executed_ticks,
                        );
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
                        if let (Some(sim), Some(locus)) =
                            (self.sim.as_ref(), self.recursive_rf_locus.as_ref())
                        {
                            update_recursive_rf_readout(
                                &mut self.recursive_rf_readout,
                                sim,
                                locus,
                                &threshold_event_kinds,
                            );
                            self.cumulative_construction_crossings = self
                                .cumulative_construction_crossings
                                .saturating_add(
                                    self.recursive_rf_readout.need_threshold_event_count as u64,
                                );
                            self.recursive_rf_readout.cumulative_construction_crossings =
                                self.cumulative_construction_crossings;
                        }
                        self.refresh_live_disruption_readout_from_snapshot(&snapshot)?;
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
            cumulative_construction_crossings: self.cumulative_construction_crossings,
            fleet_presence: self.fleet_presence.clone(),
            disruption_readout: self.disruption_readout.clone(),
            recursive_rf: self.recursive_rf_readout.clone(),
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

    /// Refresh `disruption_readout` from one coherent live GPU snapshot.
    fn refresh_live_disruption_readout(&mut self) -> Result<(), StudioLiveSessionBridgeError> {
        let Some(sim) = self.sim.as_ref() else {
            return Ok(());
        };
        let snapshot = GpuValuesSnapshot::from_session(sim);
        self.refresh_live_disruption_readout_from_snapshot(&snapshot)
    }

    fn refresh_live_disruption_readout_from_snapshot(
        &mut self,
        snapshot: &GpuValuesSnapshot,
    ) -> Result<(), StudioLiveSessionBridgeError> {
        let Some(authority) = self.readout_authority.as_ref() else {
            return Ok(());
        };
        let Some(sim) = self.sim.as_ref() else {
            return Ok(());
        };
        let loci = sim
            .spec_state
            .resource_economy_registry
            .as_ref()
            .map(|economy| economy.observation_loci.as_slice())
            .unwrap_or(&[]);
        if loci.is_empty() {
            self.disruption_readout = studio_disruption_readout_map_from_snapshot(
                &simthing_spec::disruption_readout_snapshot(authority).map_err(|e| {
                    StudioLiveSessionBridgeError::DisruptionReadback(e.to_string())
                })?,
            );
            return Ok(());
        }
        let system_id_by_host_raw = system_id_by_host_raw_from_structural_authority(
            &authority.structural_grid.placements,
            &sim.scenario.install_targets,
            loci,
            &self.location_system_ids,
        )
        .map_err(|e| StudioLiveSessionBridgeError::DisruptionReadback(e.to_string()))?;
        if system_id_by_host_raw.is_empty() {
            // Loci exist but no structural join is available on this authority
            // (e.g. seed Spec over a synthetic field profile) — fail-soft Absent.
            self.disruption_readout = studio_disruption_readout_map_from_snapshot(
                &simthing_spec::disruption_readout_snapshot(authority).map_err(|e| {
                    StudioLiveSessionBridgeError::DisruptionReadback(e.to_string())
                })?,
            );
            return Ok(());
        }
        let readback = LiveDisruptionAuthorityReadback {
            snapshot,
            registry: &sim.proto.registry,
            allocator: &sim.proto.allocator,
            loci,
            system_id_by_host_raw: &system_id_by_host_raw,
        };
        let typed = disruption_readout_snapshot_with_readback(authority, &readback)
            .map_err(|e| StudioLiveSessionBridgeError::DisruptionReadback(e.to_string()))?;
        self.disruption_readout = studio_disruption_readout_map_from_snapshot(&typed);
        Ok(())
    }

    pub fn sim_session_mut(&mut self) -> Option<&mut SimSession> {
        self.sim.as_mut()
    }

    fn apply_threshold_event_observation(
        &mut self,
        observation: Result<Vec<u32>, String>,
    ) -> Result<Vec<u32>, StudioLiveSessionBridgeError> {
        observation.map_err(|message| {
            self.status = StudioLiveSessionBridgeStatus::Errored;
            self.last_error = Some(message.clone());
            StudioLiveSessionBridgeError::ThresholdReadbackFailed {
                executed: self.executed_ticks,
                message,
            }
        })
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
/// Drops combat/event/capability trees that re-root install. ResourceFlow is retained:
/// the field-bearing path executes its admitted recursive Arena through ordinary `step_once`.
pub fn field_bearing_game_mode(mode: &GameModeSpec) -> GameModeSpec {
    let mut field = mode.clone();
    if field.resource_flow.is_none() {
        // Small field-economy fixtures have no admitted RF topology. Keep
        // their historical field-bearing economy path default-disabled.
        field.resource_flow_execution_profile = ResourceFlowExecutionProfile::DefaultDisabled;
    }
    let rf_property_keys: Vec<_> = field
        .resource_flow
        .as_ref()
        .into_iter()
        .flat_map(|rf| &rf.arenas)
        .flat_map(|arena| {
            std::iter::once(&arena.flow_property).chain(arena.balance_property.as_ref())
        })
        .map(|key| (key.namespace.clone(), key.name.clone()))
        .collect();
    let is_rf_property = |property: &PropertySpec| {
        rf_property_keys
            .iter()
            .any(|(namespace, name)| property.namespace == *namespace && property.name == *name)
    };
    // The recursive RF planner consumes property-local columns. The paired
    // scenario builder pre-registers these properties first, so omit their
    // duplicate envelope definitions during generic spec installation.
    field
        .properties
        .retain(|property| !is_rf_property(property));
    for pack in &mut field.domain_packs {
        pack.properties.retain(|property| !is_rf_property(property));
    }
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
    let mut registered_rf_property = false;
    if let Some(resource_flow) = profile.game_mode.resource_flow.as_ref() {
        let authored_properties = profile.game_mode.properties.iter().chain(
            profile
                .game_mode
                .domain_packs
                .iter()
                .flat_map(|pack| pack.properties.iter()),
        );
        for property in authored_properties {
            let referenced = resource_flow.arenas.iter().any(|arena| {
                (arena.flow_property.namespace == property.namespace
                    && arena.flow_property.name == property.name)
                    || arena.balance_property.as_ref().is_some_and(|balance| {
                        balance.namespace == property.namespace && balance.name == property.name
                    })
            });
            if referenced
                && registry
                    .id_of(&property.namespace, &property.name)
                    .is_none()
            {
                compile_property(property, &mut registry)
                    .map_err(|e| format!("pre-register field-bearing RF property: {e}"))?;
                registered_rf_property = true;
            }
        }
    }
    if !registered_rf_property {
        let _ = registry.register(SimProperty::simple("_studio_live_bridge", "seed", 0));
    }

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

/// Read sealed AccumulatorOp threshold event kinds from the most recent production tick.
/// Uses the existing public runtime readback (events remain until the next prepare).
fn last_tick_threshold_event_kinds(sim: &mut SimSession) -> Result<Vec<u32>, String> {
    let Some(runtime) = sim.state.accumulator_runtime.as_mut() else {
        return Err("threshold event runtime unavailable after production tick".into());
    };
    runtime
        .readback_threshold_events(&sim.state.ctx)
        .map(|events| events.into_iter().map(|event| event.event_kind()).collect())
        .map_err(|error| format!("sealed threshold event readback failed: {error}"))
}

/// Build telemetry loci from **materialized** emission + recipe-target cells
/// (slot/col authority only; read-only presentation sampling).
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
    for recipe in &registry.registrations.recipes {
        let key = (recipe.target_slot.raw(), recipe.target_col.raw_u32());
        if !seen.insert(key) {
            continue;
        }
        let property_key = property_key_for_col(reg, recipe.target_col.raw_u32())
            .unwrap_or_else(|| format!("col:{}", recipe.target_col.raw_u32()));
        out.push(FieldAccretionSampleLocus {
            property_key,
            source_slot: recipe.target_slot.raw(),
            source_col: recipe.target_col.raw_u32(),
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
    let snapshot = GpuValuesSnapshot::from_session(sim);
    collect_field_accretion_sample_from_snapshot(&snapshot, sample_loci, tick_index)
}

fn collect_field_accretion_sample_from_snapshot(
    snapshot: &GpuValuesSnapshot,
    sample_loci: &[FieldAccretionSampleLocus],
    tick_index: u64,
) -> Vec<StudioFieldAccretionSample> {
    let values = snapshot.values();
    let n_dims = snapshot.n_dims();
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

fn recursive_rf_locus_from_session(
    sim: &SimSession,
    profile: &StudioRecursiveRfProfile,
) -> Result<(RecursiveRfSampleLocus, StudioRecursiveRfReadout), String> {
    let property_id = sim
        .proto
        .registry
        .id_of(&profile.property_namespace, &profile.property_name)
        .ok_or_else(|| {
            format!(
                "missing recursive RF property {}::{}",
                profile.property_namespace, profile.property_name
            )
        })?;
    let cols = resolve_node_columns(
        &sim.proto.registry.property(property_id).layout,
        &profile.arena,
    )
    .map_err(|e| format!("resolve recursive RF columns: {e}"))?;
    let balance_col = cols
        .balance_col
        .ok_or_else(|| "recursive RF property has no governed Balance column".to_string())?;
    let participant_slot = |hosted_id| {
        sim.spec_state
            .arena_participant_scaffold
            .index
            .by_host_and_arena
            .get(&(hosted_id, 0))
            .copied()
            .map(|slot| slot.raw())
    };
    let ancestor_slot = participant_slot(profile.ancestor_id)
        .ok_or_else(|| "recursive RF ancestor participant is not hosted".to_string())?;
    let root_slot = participant_slot(profile.session_root_id)
        .ok_or_else(|| "recursive RF session-root participant is not hosted".to_string())?;
    let locus = RecursiveRfSampleLocus {
        ancestor_slot,
        aggregate_col: cols.intrinsic_flow_sum_col,
        root_slot,
        balance_col,
    };
    let values = sim.state.read_values();
    let n_dims = sim.state.n_dims;
    let aggregate_before = values
        .get((ancestor_slot * n_dims + locus.aggregate_col) as usize)
        .copied();
    let balance_before = values
        .get((root_slot * n_dims + balance_col) as usize)
        .copied();
    let mut readout = StudioRecursiveRfReadout {
        active: sim.state.accumulator_resource_flow_active,
        execution_profile: "RecursiveArenaResourceFlow",
        arena: Some(profile.arena.clone()),
        named_child: Some(profile.named_child_label.clone()),
        ancestor: Some(profile.ancestor_label.clone()),
        sibling_count: profile.sibling_count,
        ancestor_aggregate_before: aggregate_before,
        ancestor_aggregate_after: aggregate_before,
        root_balance_before: balance_before,
        root_balance_after: balance_before,
        ..Default::default()
    };
    fill_need_binding_readout(&mut readout, sim, None);
    Ok((locus, readout))
}

fn update_recursive_rf_readout(
    readout: &mut StudioRecursiveRfReadout,
    sim: &SimSession,
    locus: &RecursiveRfSampleLocus,
    threshold_event_kinds: &[u32],
) {
    let values = sim.state.read_values();
    let n_dims = sim.state.n_dims;
    readout.active = sim.state.accumulator_resource_flow_active;
    readout.ancestor_aggregate_after = values
        .get((locus.ancestor_slot * n_dims + locus.aggregate_col) as usize)
        .copied();
    readout.root_balance_after = values
        .get((locus.root_slot * n_dims + locus.balance_col) as usize)
        .copied();
    fill_need_binding_readout(readout, sim, Some(threshold_event_kinds));
}

fn fill_need_binding_readout(
    readout: &mut StudioRecursiveRfReadout,
    sim: &SimSession,
    threshold_event_kinds: Option<&[u32]>,
) {
    let Some(binding) = sim.spec_state.resolved_need_bindings.first() else {
        readout.need_profile_id = None;
        readout.need_profile_kind = None;
        readout.need_weight_values = None;
        readout.need_live_value = None;
        readout.need_threshold = None;
        readout.need_threshold_result = None;
        readout.need_threshold_event_count = 0;
        return;
    };
    let values = sim.state.read_values();
    let n_dims = sim.state.n_dims as usize;
    let idx = binding.participant_slot as usize * n_dims + binding.need_col.raw();
    let live = values.get(idx).copied();
    readout.need_profile_id = Some(binding.id.clone());
    readout.need_profile_kind = Some(binding.profile.clone());
    readout.need_weight_values = Some(
        binding
            .weights
            .iter()
            .map(|weight| {
                let idx = weight.slot as usize * n_dims + weight.col.raw();
                let value = values.get(idx).copied().unwrap_or(f32::NAN);
                format!("{}={value:.6}", weight.entity)
            })
            .collect::<Vec<_>>()
            .join(","),
    );
    readout.need_live_value = live;
    readout.need_threshold = Some(binding.threshold);
    readout.need_threshold_event_count = threshold_event_kinds
        .map(|kinds| {
            kinds
                .iter()
                .filter(|kind| **kind == binding.event_kind)
                .count() as u32
        })
        .unwrap_or(0);
    readout.need_threshold_result = Some(match threshold_event_kinds {
        None => "not-run",
        Some(_) if readout.need_threshold_event_count > 0 => "event",
        Some(_) => "no-event",
    });
}

fn tree_contains_simthing_id(node: &SimThing, id: SimThingId) -> bool {
    if node.id == id {
        return true;
    }
    node.children
        .iter()
        .any(|child| tree_contains_simthing_id(child, id))
}

fn find_simthing_in_tree(node: &SimThing, id: SimThingId) -> Option<&SimThing> {
    if node.id == id {
        return Some(node);
    }
    for child in &node.children {
        if let Some(found) = find_simthing_in_tree(child, id) {
            return Some(found);
        }
    }
    None
}

fn graft_authored_location_hosts(
    scenario: &mut simthing_spec::SimThingScenarioSpec,
    pack: &simthing_clausething::HydratedScenarioPack,
) -> Option<()> {
    use simthing_core::SimThingKind;
    let owner_keys: std::collections::HashSet<&str> = pack
        .owners
        .iter()
        .map(|owner| owner.owner_key.as_str())
        .collect();
    let mut to_graft = Vec::new();
    for (key, ids) in &pack.install_targets {
        if owner_keys.contains(key.as_str()) || key == &pack.scenario_id {
            continue;
        }
        let Some(host_id) = ids.first().copied() else {
            continue;
        };
        if tree_contains_simthing_id(&scenario.root, host_id) {
            continue;
        }
        let host = find_simthing_in_tree(&pack.root, host_id)?;
        if host.kind != SimThingKind::Location {
            continue;
        }
        to_graft.push(host.clone());
    }
    let session = scenario
        .root
        .children
        .iter_mut()
        .find(|child| child.kind == SimThingKind::GameSession)?;
    for host in to_graft {
        session.add_child(host);
    }
    Some(())
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

const STUDIO_RF_ARENA: &str = "studio_recursive_owner_flow";
const STUDIO_RF_NAMESPACE: &str = "studio_live_rf";
const STUDIO_RF_PROPERTY: &str = "owner_flow";
// This f32 budget deliberately leaves a bounded, deterministic allocator
// residual at the real Owner when its three equal-weight children disburse.
// The governed Balance path must integrate that non-zero residue.
const STUDIO_RF_ROOT_BUDGET: f32 = 12.1;

fn rf_subfield(name: &str, role: AccumulatorRole, default: f32) -> SubFieldSpec {
    SubFieldSpec {
        role: SubFieldRole::Named(name.into()),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default,
        display_name: name.into(),
        display_range: None,
        governed_by: None,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: Some(AccumulatorSpec {
            role,
            log_tier: LogTier::Summary,
        }),
    }
}

fn rf_balance_rate_subfield() -> SubFieldSpec {
    SubFieldSpec {
        role: SubFieldRole::Named("balance_rate".into()),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: "balance_rate".into(),
        display_range: None,
        governed_by: None,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: None,
    }
}

fn rf_balance_subfield() -> SubFieldSpec {
    SubFieldSpec {
        role: SubFieldRole::Named("balance".into()),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: "balance".into(),
        display_range: None,
        governed_by: Some(SubFieldRole::Named("balance_rate".into())),
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: Some(AccumulatorSpec {
            role: AccumulatorRole::Balance(BalanceSpec::default()),
            log_tier: LogTier::Summary,
        }),
    }
}

fn studio_recursive_rf_property() -> PropertySpec {
    PropertySpec {
        id: STUDIO_RF_PROPERTY.into(),
        namespace: STUDIO_RF_NAMESPACE.into(),
        name: STUDIO_RF_PROPERTY.into(),
        display_name: "Studio live owner flow".into(),
        description: "Canonical Owner sibling aggregate for Studio live RF telemetry".into(),
        sub_fields: vec![
            rf_subfield("flow", AccumulatorRole::IntrinsicFlow, 0.0),
            rf_subfield(
                "allocated",
                AccumulatorRole::AllocatedFlow {
                    arena: STUDIO_RF_ARENA.into(),
                },
                0.0,
            ),
            rf_subfield(
                "weight",
                AccumulatorRole::AllocatorWeight {
                    arena: STUDIO_RF_ARENA.into(),
                },
                1.0,
            ),
            rf_balance_rate_subfield(),
            rf_balance_subfield(),
        ],
    }
}

fn compose_recursive_rf_profile(
    pack: &simthing_clausething::HydratedScenarioPack,
) -> Option<(
    SimThing,
    HashMap<String, Vec<SimThingId>>,
    GameModeSpec,
    StudioRecursiveRfProfile,
)> {
    let mut scenario = simthing_clausething::project_pack_to_authority_tree_candidate(pack).ok()?;
    // Authored Location hosts (terran_shipyard / pirate_outpost) live on the hydrate
    // World tree (`pack.root`), not the GalaxyMap authority tree. Graft them under
    // GameSession so local-flow couplings keep distinct source/pressure/sink identity
    // without collapsing onto the session root.
    graft_authored_location_hosts(&mut scenario, pack)?;
    let session_root_id = game_session_child(&scenario).ok()?.id;
    let owners = game_session_owners(&scenario).ok()?;
    let participant_inputs = planet_child_rf_participant_inputs(&scenario).ok()?;

    let mut selected_owner = None;
    let mut selected_children = Vec::new();
    for owner in owners {
        let Some(owner_ref) = owner_entity_id(owner) else {
            continue;
        };
        let candidates: Vec<_> = participant_inputs
            .iter()
            .filter(|row| row.owner_ref.as_str() == owner_ref && row.surplus > 0)
            .take(3)
            .cloned()
            .collect();
        if candidates.len() == 3 {
            selected_owner = Some((owner_ref, owner.id));
            selected_children = candidates;
            break;
        }
    }
    let (owner_ref, owner_id) = selected_owner?;

    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&scenario.root);
    let hosted_slot = |id: SimThingId| allocator.slot_of(id).map(|slot| slot.raw());
    let mut participants = vec![
        ExplicitParticipantSpec::flat(hosted_slot(session_root_id)?, session_root_id.raw()),
        ExplicitParticipantSpec::nested(
            hosted_slot(owner_id)?,
            owner_id.raw(),
            session_root_id.raw() as u64,
        ),
    ];

    let mut install_targets: HashMap<String, Vec<SimThingId>> = pack
        .install_targets
        .iter()
        .map(|(key, ids)| (key.clone(), ids.clone()))
        .collect();
    // Owner keys retain their exact Owner host from the projected authority tree.
    for owner in game_session_owners(&scenario).ok()? {
        if let Some(key) = owner_entity_id(owner) {
            install_targets.insert(key, vec![owner.id]);
        }
    }

    let root_target = "studio_rf_root".to_string();
    install_targets.insert(root_target.clone(), vec![session_root_id]);
    let mut base_obligations = vec![BaseFlowObligationSpec {
        id: "studio_rf_root_budget".into(),
        arena: STUDIO_RF_ARENA.into(),
        install: InstallTargetSpec::ScenarioListed {
            target_id: root_target,
        },
        direction: BaseFlowDirectionSpec::Produce,
        rate: STUDIO_RF_ROOT_BUDGET,
    }];
    for (index, row) in selected_children.iter().enumerate() {
        let id = SimThingId::from_session_raw(row.simthing_id_raw);
        participants.push(ExplicitParticipantSpec::nested(
            hosted_slot(id)?,
            id.raw(),
            owner_id.raw() as u64,
        ));
        let target_id = format!("studio_rf_child_{index}");
        install_targets.insert(target_id.clone(), vec![id]);
        base_obligations.push(BaseFlowObligationSpec {
            id: format!("studio_rf_child_{index}_intrinsic"),
            arena: STUDIO_RF_ARENA.into(),
            install: InstallTargetSpec::ScenarioListed {
                target_id: target_id.clone(),
            },
            direction: BaseFlowDirectionSpec::Produce,
            rate: row.surplus as f32,
        });
    }

    let mut game_mode = pack.game_mode.clone();
    game_mode.properties.push(studio_recursive_rf_property());
    let authored_need = game_mode
        .resource_flow
        .as_ref()
        .map(|rf| rf.need_bindings.clone())
        .unwrap_or_default();
    game_mode.resource_flow = Some(ResourceFlowSpec {
        opt_in_mode: ResourceFlowOptInMode::Disabled,
        arenas: vec![ArenaSpec {
            name: STUDIO_RF_ARENA.into(),
            flow_property: PropertyKey::new(STUDIO_RF_NAMESPACE, STUDIO_RF_PROPERTY),
            balance_property: Some(PropertyKey::new(STUDIO_RF_NAMESPACE, STUDIO_RF_PROPERTY)),
            max_participants: 8,
            max_coupling_fanout: 4,
            max_orderband_depth: 16,
            fission_policy: FissionPolicySpec::Reject,
            reserved_orderband_depth: 0,
            reserved_gap_per_intermediate: 0,
            expected_max_children_per_intermediate: 0,
            explicit_participants: participants,
            enrollment: None,
            wildcard_admission: None,
        }],
        base_obligations,
        need_bindings: authored_need,
        ..Default::default()
    });
    game_mode.resource_flow_execution_profile =
        ResourceFlowExecutionProfile::RecursiveArenaResourceFlow;

    let named = &selected_children[0];
    Some((
        scenario.root,
        install_targets,
        game_mode,
        StudioRecursiveRfProfile {
            arena: STUDIO_RF_ARENA.into(),
            property_namespace: STUDIO_RF_NAMESPACE.into(),
            property_name: STUDIO_RF_PROPERTY.into(),
            named_child_label: format!(
                "{} SIM-{:06}",
                named.participant_kind_label, named.simthing_id_raw
            ),
            named_child_id: SimThingId::from_session_raw(named.simthing_id_raw),
            ancestor_label: format!("Owner {owner_ref}"),
            ancestor_id: owner_id,
            session_root_id,
            sibling_count: selected_children.len() as u32,
        },
    ))
}

/// Build an authored live profile from a hydrated scenario pack (production elevation).
///
/// When the canonical authority exposes admitted RF participants, the profile uses that
/// exact tree and an ordinary admitted recursive Arena. Smaller field-economy fixtures retain
/// their legacy hydrate root only because they have no authority-tree participant substrate.
pub fn authored_live_profile_from_pack(
    pack: &simthing_clausething::HydratedScenarioPack,
) -> StudioAuthoredLiveProfile {
    let location_system_ids = location_system_ids_from_pack(pack);
    if let Some((root, install_targets, game_mode, recursive_rf)) =
        compose_recursive_rf_profile(pack)
    {
        let mut profile = StudioAuthoredLiveProfile::from_hydrated_pack(
            game_mode,
            install_targets,
            root,
            pack.field_economy.is_some(),
        );
        profile.recursive_rf = Some(recursive_rf);
        profile.location_system_ids = location_system_ids;
        return profile;
    }

    let root = pack.root.clone();
    let mut install_targets: HashMap<String, Vec<SimThingId>> = pack
        .install_targets
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    for owner in &pack.owners {
        install_targets
            .entry(owner.owner_key.clone())
            .or_insert_with(|| vec![root.id]);
    }
    let mut profile = StudioAuthoredLiveProfile::from_hydrated_pack(
        pack.game_mode.clone(),
        install_targets,
        root,
        pack.field_economy.is_some(),
    );
    profile.location_system_ids = location_system_ids;
    profile
}

/// Join hydrate grid placements to embedded lattice system ids by exact (row,col).
fn location_system_ids_from_pack(
    pack: &simthing_clausething::HydratedScenarioPack,
) -> BTreeMap<String, u32> {
    let mut by_rc: BTreeMap<(u32, u32), u32> = BTreeMap::new();
    if let Some(embedded) = pack.embedded_static_galaxy_scenarios.first() {
        for placement in &embedded.source_structural_grid.placements {
            by_rc.insert((placement.row, placement.col), placement.system_id);
        }
    }
    let mut out = BTreeMap::new();
    for placement in &pack.grid_metadata.placements {
        let Some(&system_id) = by_rc.get(&(placement.row, placement.col)) else {
            continue;
        };
        out.insert(placement.location_id.clone(), system_id);
        out.insert(placement.target_id.clone(), system_id);
    }
    out
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
    fn threshold_observation_error_is_not_no_event() {
        let mut bridge = StudioLiveSessionBridge::new();
        let error = bridge
            .apply_threshold_event_observation(Err(
                "threshold event runtime unavailable after production tick".into(),
            ))
            .expect_err("missing sealed readback must fail closed");
        assert!(matches!(
            error,
            StudioLiveSessionBridgeError::ThresholdReadbackFailed { .. }
        ));
        assert_eq!(bridge.status(), StudioLiveSessionBridgeStatus::Errored);
        assert_eq!(bridge.recursive_rf_readout.need_threshold_result, None);
    }

    #[test]
    fn field_bridge_forbids_dense_boundary_seeding() {
        let source = include_str!("studio_live_session_bridge.rs");
        let values_install = [".install_resolved", "_values_at_boundary("].concat();
        let previous_install = [".install_resolved_previous", "_values_at_boundary("].concat();
        assert!(!source.contains(&values_install));
        assert!(!source.contains(&previous_install));
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
