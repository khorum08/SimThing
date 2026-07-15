//! STUDIO-LIVE-SESSION-BRIDGE-0 — production bridge from Studio clock to SimSession ticks.
//!
//! Consumes scheduled counts from [`crate::StudioSimClock`] and executes them through
//! [`simthing_driver::SimSession::step_once`]. Does not import workshop post-hydration modules,
//! invent gameplay systems, or mutate ScenarioSpec from UI/transport.

use std::collections::HashMap;
use std::path::PathBuf;

use simthing_core::{DimensionRegistry, SimProperty};
use simthing_driver::{Scenario, SessionError, SimSession, StepOnceOutcome};
use simthing_spec::{
    validate_scenario_links, validate_stead_mapping_consistency, SimThingScenarioSpec,
};

use crate::session::{StudioScenarioSummary, StudioSession};
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
    pub fleet_presence: StudioFleetPresenceMap,
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
            production_path: StudioLiveSessionBridge::production_path_label(),
            fleet_presence: StudioFleetPresenceMap::default(),
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
}

/// Production live handle over a loaded StudioSession authority.
///
/// `StudioSession.scenario_authority` is the construction source. The bridge holds an
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
    /// True if we attempted open and failed for non-GPU reasons.
    open_failed: bool,
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
            open_failed: false,
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

    pub fn production_path_label() -> &'static str {
        "simthing_driver::SimSession::open + step_once"
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
        self.open_failed = false;
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

        let scenario = driver_scenario_from_authority(&studio.scenario_authority).map_err(
            |e| StudioLiveSessionBridgeError::ScenarioConversion(e),
        )?;
        let fleet_presence = studio_fleet_presence_map_from_session(studio)
            .map_err(|e| StudioLiveSessionBridgeError::FleetPresenceReadback(e.to_string()))?;

        match SimSession::open(scenario) {
            Ok(sim) => {
                self.sim = Some(sim);
                self.status = StudioLiveSessionBridgeStatus::Ready;
                self.last_error = None;
                self.open_scenario_id = Some(studio.scenario_authority.scenario_id.clone());
                self.open_stead_valid = Some(studio.scenario_summary.stead_valid);
                self.open_links_valid = Some(studio.scenario_summary.links_valid);
                self.open_source_path = studio.scenario_path.clone();
                self.fleet_presence = fleet_presence;
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
                Err(StudioLiveSessionBridgeError::OpenFailed(msg))
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
        let Some(sim) = self.sim.as_mut() else {
            return Err(StudioLiveSessionBridgeError::NoStudioSession);
        };

        let mut ran = 0u64;
        for _ in 0..scheduled {
            match sim.step_once() {
                Ok(StepOnceOutcome { ticks_run, .. }) => {
                    ran = ran.saturating_add(ticks_run.max(1));
                    self.executed_ticks = self.executed_ticks.saturating_add(ticks_run.max(1));
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
            production_path: Self::production_path_label(),
            fleet_presence: self.fleet_presence.clone(),
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
pub fn driver_scenario_from_authority(
    spec: &SimThingScenarioSpec,
) -> Result<Scenario, String> {
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

#[cfg(test)]
mod unit_smoke {
    use super::*;

    #[test]
    fn default_unattached() {
        let b = StudioLiveSessionBridge::new();
        assert_eq!(b.status(), StudioLiveSessionBridgeStatus::Unattached);
        assert_eq!(b.executed_ticks(), 0);
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
}
