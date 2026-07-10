//! STUDIO-SIM-CLOCK-UI-0 — transport façade over [`StudioSimClock`].
//!
//! Presentation/operator controls and headless CI hooks only. The clock remains the sole
//! schedule authority; this module does not invent a second timer, scheduler, or live-tick bridge.
//! Invalid max-TPS values route through [`StudioSimClock::set_max_tps`] (no UI-only sanitization).

use crate::studio_sim_clock::{
    StudioSimClock, StudioSimClockError, StudioSimClockRate, STUDIO_SIM_CLOCK_DEFAULT_MAX_TPS,
};

/// Operator transport actions that project onto [`StudioSimClock`].
#[derive(Debug, Clone, PartialEq)]
pub enum StudioSimClockTransportCommand {
    Pause,
    Play,
    Rate1x,
    Rate2x,
    Rate4x,
    /// Draft text for max TPS (parsed + validated via the clock).
    SetMaxTpsText(String),
    /// Already-parsed max TPS (still validated by the clock).
    SetMaxTps(f64),
}

/// Read-only projection of clock state for UI readout / CI assertions.
#[derive(Debug, Clone, PartialEq)]
pub struct StudioSimClockReadout {
    pub paused: bool,
    pub playing: bool,
    pub rate: StudioSimClockRate,
    pub rate_label: &'static str,
    pub max_tps: f64,
    pub effective_tps: f64,
    pub tick_index: u64,
}

impl StudioSimClockReadout {
    pub fn from_clock(clock: &StudioSimClock) -> Self {
        let paused = clock.is_paused();
        Self {
            paused,
            playing: !paused,
            rate: clock.rate(),
            rate_label: rate_label(clock.rate()),
            max_tps: clock.max_tps(),
            effective_tps: clock.effective_tps(),
            tick_index: clock.tick_index(),
        }
    }
}

/// Compact transport surface state: one clock + draft TPS field (presentation only).
///
/// The draft string is UI chrome; applying it always goes through
/// [`StudioSimClock::set_max_tps`].
#[derive(Debug, Clone, PartialEq)]
pub struct StudioSimClockTransport {
    clock: StudioSimClock,
    /// Editable max-TPS field text (not schedule authority until applied).
    max_tps_draft: String,
    /// Last apply error for max TPS (presentation feedback only).
    last_error: Option<String>,
}

impl Default for StudioSimClockTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl StudioSimClockTransport {
    pub fn new() -> Self {
        let clock = StudioSimClock::new();
        Self {
            max_tps_draft: format_max_tps(clock.max_tps()),
            clock,
            last_error: None,
        }
    }

    /// Borrow the underlying schedule authority (bridge rungs consume this later).
    pub fn clock(&self) -> &StudioSimClock {
        &self.clock
    }

    pub fn clock_mut(&mut self) -> &mut StudioSimClock {
        &mut self.clock
    }

    pub fn max_tps_draft(&self) -> &str {
        &self.max_tps_draft
    }

    pub fn max_tps_draft_mut(&mut self) -> &mut String {
        &mut self.max_tps_draft
    }

    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    pub fn readout(&self) -> StudioSimClockReadout {
        StudioSimClockReadout::from_clock(&self.clock)
    }

    /// Apply a transport command. Max-TPS paths use the clock's validation.
    pub fn apply(
        &mut self,
        command: StudioSimClockTransportCommand,
    ) -> Result<(), StudioSimClockError> {
        match command {
            StudioSimClockTransportCommand::Pause => {
                self.clock.pause();
                self.last_error = None;
                Ok(())
            }
            StudioSimClockTransportCommand::Play => {
                self.clock.play();
                self.last_error = None;
                Ok(())
            }
            StudioSimClockTransportCommand::Rate1x => {
                self.clock.set_rate(StudioSimClockRate::Rate1x);
                self.last_error = None;
                Ok(())
            }
            StudioSimClockTransportCommand::Rate2x => {
                self.clock.set_rate(StudioSimClockRate::Rate2x);
                self.last_error = None;
                Ok(())
            }
            StudioSimClockTransportCommand::Rate4x => {
                self.clock.set_rate(StudioSimClockRate::Rate4x);
                self.last_error = None;
                Ok(())
            }
            StudioSimClockTransportCommand::SetMaxTps(max_tps) => self.apply_max_tps(max_tps),
            StudioSimClockTransportCommand::SetMaxTpsText(text) => {
                self.max_tps_draft = text;
                self.apply_max_tps_draft()
            }
        }
    }

    /// Parse the draft field and apply through the clock validator.
    pub fn apply_max_tps_draft(&mut self) -> Result<(), StudioSimClockError> {
        let parsed = parse_max_tps_draft(&self.max_tps_draft).map_err(|max_tps| {
            let err = StudioSimClockError::InvalidMaxTps { max_tps };
            self.last_error = Some(err.to_string());
            err
        })?;
        self.apply_max_tps(parsed)
    }

    fn apply_max_tps(&mut self, max_tps: f64) -> Result<(), StudioSimClockError> {
        match self.clock.set_max_tps(max_tps) {
            Ok(()) => {
                self.max_tps_draft = format_max_tps(self.clock.max_tps());
                self.last_error = None;
                Ok(())
            }
            Err(e) => {
                self.last_error = Some(e.to_string());
                Err(e)
            }
        }
    }
}

/// Human-readable rate label for readout / buttons.
pub fn rate_label(rate: StudioSimClockRate) -> &'static str {
    match rate {
        StudioSimClockRate::Rate1x => "1×",
        StudioSimClockRate::Rate2x => "2×",
        StudioSimClockRate::Rate4x => "4×",
    }
}

fn format_max_tps(max_tps: f64) -> String {
    if (max_tps - STUDIO_SIM_CLOCK_DEFAULT_MAX_TPS).abs() < f64::EPSILON {
        "10".to_string()
    } else if max_tps.fract() == 0.0 && max_tps.abs() < 1e12 {
        format!("{}", max_tps as i64)
    } else {
        format!("{max_tps}")
    }
}

/// Parse draft max-TPS text. Empty / non-numeric map to a sentinel rejected by the clock.
fn parse_max_tps_draft(text: &str) -> Result<f64, f64> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err(f64::NAN);
    }
    trimmed.parse::<f64>().map_err(|_| f64::NAN)
}

#[cfg(test)]
mod unit_smoke {
    use super::*;

    #[test]
    fn transport_default_is_paused_at_1x() {
        let t = StudioSimClockTransport::new();
        let r = t.readout();
        assert!(r.paused);
        assert!(!r.playing);
        assert_eq!(r.rate, StudioSimClockRate::Rate1x);
        assert_eq!(r.rate_label, "1×");
        assert_eq!(r.tick_index, 0);
    }
}
