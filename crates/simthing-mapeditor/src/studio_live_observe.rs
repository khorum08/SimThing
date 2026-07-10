//! STUDIO-LIVE-OBSERVE-0 — pure observation projection over clock + bridge + session.
//!
//! Presentation-only. Combines existing readouts; does not schedule ticks, call
//! `SimSession::step_once`, mutate ScenarioSpec, import workshop residue, or invent
//! gameplay / planner / CPU decision semantics.

use crate::session::{StudioScenarioSummary, StudioSession, StudioSessionSource};
use crate::studio_live_session_bridge::{
    StudioLiveSessionBridgeReadout, StudioLiveSessionBridgeStatus,
};
use crate::studio_sim_clock_ui::StudioSimClockReadout;

/// Operator-visible source kind for a Studio session (presentation label only).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudioLiveObservationSourceKind {
    /// No StudioSession loaded.
    None,
    /// Session from generation path.
    Generated,
    /// Session loaded from scenario authority path (JSON / clause-hydrated Spec).
    LoadedScenario,
}

impl StudioLiveObservationSourceKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::None => "no loaded session",
            Self::Generated => "generated",
            Self::LoadedScenario => "loaded scenario",
        }
    }
}

/// Compact pure snapshot of live clock + bridge + session identity for UI / CI.
///
/// Not schedule authority. Not model authority. Rebuild from current sources each frame.
#[derive(Debug, Clone, PartialEq)]
pub struct StudioLiveObservationReadout {
    // --- Clock (from StudioSimClockTransport / StudioSimClockReadout) ---
    pub clock_paused: bool,
    pub clock_playing: bool,
    pub clock_rate_label: &'static str,
    pub max_tps: f64,
    pub effective_tps: f64,
    pub scheduled_tick_index: u64,

    // --- Live bridge (from StudioLiveSessionBridgeReadout) ---
    pub bridge_status: StudioLiveSessionBridgeStatus,
    pub bridge_status_label: &'static str,
    pub bridge_executed_ticks: u64,
    pub bridge_last_scheduled_batch: u64,
    pub bridge_last_error: Option<String>,
    pub bridge_production_path: &'static str,
    /// Bridge-open scenario id snapshot when attached; independent of UI session borrow.
    pub bridge_scenario_id: Option<String>,
    pub bridge_stead_valid: Option<bool>,

    // --- Session (from StudioSession when loaded) ---
    pub session_loaded: bool,
    pub source_kind: StudioLiveObservationSourceKind,
    pub source_kind_label: &'static str,
    pub scenario_id: Option<String>,
    pub system_count: Option<u32>,
    pub link_count: Option<u32>,
    pub stead_valid: Option<bool>,
    pub links_valid: Option<bool>,
    pub rf_ready: Option<bool>,
    /// Already-available structural projection field (not a planner).
    pub occupied_cells: Option<u64>,
    pub session_status_message: Option<String>,
}

impl StudioLiveObservationReadout {
    /// Default unattached / no-session observation (nothing fabricated as "running").
    pub fn default_unattached() -> Self {
        let bridge = StudioLiveSessionBridgeReadout::default_unattached();
        let clock = StudioSimClockReadout {
            paused: true,
            playing: false,
            rate: crate::StudioSimClockRate::Rate1x,
            rate_label: "1×",
            max_tps: crate::STUDIO_SIM_CLOCK_DEFAULT_MAX_TPS,
            effective_tps: 0.0,
            tick_index: 0,
        };
        build_studio_live_observation_readout(&clock, &bridge, None)
    }
}

/// Build a pure observation snapshot from existing presentation-safe sources.
///
/// Does not advance the clock, open/execute the bridge, or touch ScenarioSpec.
pub fn build_studio_live_observation_readout(
    clock: &StudioSimClockReadout,
    bridge: &StudioLiveSessionBridgeReadout,
    session: Option<&StudioSession>,
) -> StudioLiveObservationReadout {
    let (source_kind, summary, status_message) = match session {
        None => (StudioLiveObservationSourceKind::None, None, None),
        Some(s) => {
            let kind = match &s.source {
                StudioSessionSource::Generated { .. } => StudioLiveObservationSourceKind::Generated,
                StudioSessionSource::LoadedScenario { .. } => {
                    StudioLiveObservationSourceKind::LoadedScenario
                }
            };
            (kind, Some(&s.scenario_summary), Some(s.status_message()))
        }
    };

    let from_summary = |sum: &StudioScenarioSummary| {
        (
            Some(sum.scenario_id.clone()),
            Some(sum.system_count),
            Some(sum.link_count),
            Some(sum.stead_valid),
            Some(sum.links_valid),
            Some(sum.rf_ready),
            Some(sum.occupied_cells),
        )
    };

    let (scenario_id, system_count, link_count, stead_valid, links_valid, rf_ready, occupied_cells) =
        match summary {
            Some(sum) => from_summary(sum),
            None => (None, None, None, None, None, None, None),
        };

    StudioLiveObservationReadout {
        clock_paused: clock.paused,
        clock_playing: clock.playing,
        clock_rate_label: clock.rate_label,
        max_tps: clock.max_tps,
        effective_tps: clock.effective_tps,
        scheduled_tick_index: clock.tick_index,

        bridge_status: bridge.status,
        bridge_status_label: bridge.status_label,
        bridge_executed_ticks: bridge.executed_ticks,
        bridge_last_scheduled_batch: bridge.last_scheduled_batch,
        bridge_last_error: bridge.last_error.clone(),
        bridge_production_path: bridge.production_path,
        bridge_scenario_id: bridge.scenario_id.clone(),
        bridge_stead_valid: bridge.stead_valid,

        session_loaded: session.is_some(),
        source_kind,
        source_kind_label: source_kind.label(),
        scenario_id,
        system_count,
        link_count,
        stead_valid,
        links_valid,
        rf_ready,
        occupied_cells,
        session_status_message: status_message,
    }
}

/// Source-text scan: observation module must not import workshop residue or invent gameplay APIs.
///
/// Contiguous patterns are assembled at runtime so this function body does not self-match.
///
/// catches: observer importing workshop residue or inventing gameplay summaries.
pub fn observe_module_source_forbids_workshop_residue(source: &str) -> Result<(), String> {
    // Workshop crate path tokens (split).
    let t_ws_rs = format!("{}{}", "simthing_", "workshop");
    let t_ws_hy = format!("{}{}", "simthing", "-workshop");
    // Driver / planner tokens (split so the forbid body is not a positive match).
    let step = format!("{}{}", "step_once", "(");
    let open = format!("{}{}", "SimSession::", "open");
    let g1 = format!("{}{}", "Cpu", "Planner");
    let g2 = format!("{}{}", "cpu_", "planner");
    let g3 = format!("{}{}", "GameMode", "Attach");
    let g4 = format!("{}{}", "Diplomacy", "System");
    let g5 = format!("{}{}", "Combat", "System");
    let g6 = format!("{}{}", "Economy", "Planner");
    let gameplay = [
        g1.as_str(),
        g2.as_str(),
        g3.as_str(),
        g4.as_str(),
        g5.as_str(),
        g6.as_str(),
    ];

    for line in source.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("//") || trimmed.starts_with("//!") || trimmed.starts_with("///") {
            continue;
        }
        if trimmed.contains("observe_module_source_forbids") {
            continue;
        }
        // Workshop import / dependency path.
        if (trimmed.contains("use ") || trimmed.contains("::") || trimmed.contains("extern crate"))
            && (trimmed.contains(&t_ws_rs) || trimmed.contains(&t_ws_hy))
        {
            return Err(format!(
                "studio_live_observe forbids workshop residue import: {trimmed}"
            ));
        }
        if trimmed.contains(&step) || trimmed.contains(&open) {
            return Err(format!(
                "studio_live_observe forbids driver execution token in: {trimmed}"
            ));
        }
        for token in gameplay {
            if trimmed.contains(token) {
                return Err(format!(
                    "studio_live_observe forbids gameplay/planner token {token} in: {trimmed}"
                ));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod unit_smoke {
    use super::*;
    use crate::studio_live_session_bridge::StudioLiveSessionBridgeReadout;
    use crate::studio_sim_clock_ui::StudioSimClockReadout;
    use crate::StudioSimClockRate;

    #[test]
    fn default_unattached_reports_no_session() {
        let o = StudioLiveObservationReadout::default_unattached();
        assert!(!o.session_loaded);
        assert_eq!(o.source_kind, StudioLiveObservationSourceKind::None);
        assert_eq!(o.bridge_status, StudioLiveSessionBridgeStatus::Unattached);
        assert_eq!(o.scheduled_tick_index, 0);
        assert_eq!(o.bridge_executed_ticks, 0);
        assert!(o.scenario_id.is_none());
    }

    #[test]
    fn build_copies_clock_and_bridge_fields() {
        let clock = StudioSimClockReadout {
            paused: false,
            playing: true,
            rate: StudioSimClockRate::Rate2x,
            rate_label: "2×",
            max_tps: 20.0,
            effective_tps: 40.0,
            tick_index: 7,
        };
        let mut bridge = StudioLiveSessionBridgeReadout::default_unattached();
        bridge.status = StudioLiveSessionBridgeStatus::Running;
        bridge.status_label = "running";
        bridge.executed_ticks = 3;
        bridge.last_scheduled_batch = 3;
        let o = build_studio_live_observation_readout(&clock, &bridge, None);
        assert!(o.clock_playing);
        assert_eq!(o.clock_rate_label, "2×");
        assert_eq!(o.scheduled_tick_index, 7);
        assert_eq!(o.bridge_executed_ticks, 3);
        assert_eq!(o.bridge_status_label, "running");
        assert!(!o.session_loaded);
    }
}
