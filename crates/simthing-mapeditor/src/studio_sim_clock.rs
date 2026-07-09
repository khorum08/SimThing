//! STUDIO-SIM-CLOCK-0 — Studio sim clock substrate.
//!
//! Owns operator transport schedule state (pause/play, rate, max TPS, tick index).
//! Schedules admitted sim ticks for later bridge/UI rungs; does **not** execute gameplay,
//! mutate ScenarioSpec, or use Bevy Time / egui widget state as authority.

/// Allowed acceleration multipliers for the Studio sim clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StudioSimClockRate {
    /// 1× — baseline schedule rate.
    Rate1x,
    /// 2× — twice the 1× schedule demand.
    Rate2x,
    /// 4× — four times the 1× schedule demand.
    Rate4x,
}

impl StudioSimClockRate {
    /// Integer multiplier applied to the 1× tick demand.
    pub const fn multiplier(self) -> u32 {
        match self {
            Self::Rate1x => 1,
            Self::Rate2x => 2,
            Self::Rate4x => 4,
        }
    }

    /// Multiplier as `f64` for scheduling math.
    pub const fn as_f64(self) -> f64 {
        self.multiplier() as f64
    }
}

/// Default 1× ticks-per-second baseline when a clock is constructed.
pub const STUDIO_SIM_CLOCK_DEFAULT_MAX_TPS: f64 = 10.0;

/// Documented rate-ratio tolerance for 2×/4× vs 1× under the same wall elapsed + TPS.
///
/// Ratios are exact under deterministic accumulator math when `elapsed * rate * max_tps`
/// is integral; this tolerance covers fractional remainder / flooring across rates.
pub const STUDIO_SIM_CLOCK_RATE_RATIO_TOLERANCE: f64 = 0.05;

/// Headless-testable Studio sim clock (presentation/transport substrate only).
///
/// Scheduling model (deterministic):
/// - While **playing**, each wall-second of elapsed time demands
///   `rate.multiplier() * max_tps` ticks (1× baseline = `max_tps`).
/// - Fractional demand accumulates; whole ticks are emitted and counted on `tick_index`.
/// - While **paused**, elapsed time does not schedule ticks and `tick_index` is frozen.
/// - `max_tps` must be finite and `> 0`; it bounds 1× demand and therefore caps storms
///   for a given rate (demand = rate × max_tps × dt, no unbounded open loop).
#[derive(Debug, Clone, PartialEq)]
pub struct StudioSimClock {
    paused: bool,
    rate: StudioSimClockRate,
    max_tps: f64,
    tick_index: u64,
    /// Fractional tick demand not yet emitted (always in `[0, 1)` after a schedule step).
    accumulator: f64,
}

impl Default for StudioSimClock {
    fn default() -> Self {
        Self::new()
    }
}

impl StudioSimClock {
    /// New clock: **paused**, 1×, default max TPS, tick index 0.
    pub fn new() -> Self {
        Self {
            paused: true,
            rate: StudioSimClockRate::Rate1x,
            max_tps: STUDIO_SIM_CLOCK_DEFAULT_MAX_TPS,
            tick_index: 0,
            accumulator: 0.0,
        }
    }

    /// Resume scheduling under the current rate and max TPS.
    pub fn play(&mut self) {
        self.paused = false;
    }

    /// Freeze scheduling; `tick_index` holds until Play.
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Set acceleration rate (1× / 2× / 4×).
    pub fn set_rate(&mut self, rate: StudioSimClockRate) {
        self.rate = rate;
    }

    /// Set the 1× max ticks-per-second baseline. Rejects non-finite or non-positive values.
    pub fn set_max_tps(&mut self, max_tps: f64) -> Result<(), StudioSimClockError> {
        if !max_tps.is_finite() || max_tps <= 0.0 {
            return Err(StudioSimClockError::InvalidMaxTps { max_tps });
        }
        self.max_tps = max_tps;
        Ok(())
    }

    /// Whether the clock is paused.
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Current acceleration rate.
    pub fn rate(&self) -> StudioSimClockRate {
        self.rate
    }

    /// Current 1× max ticks-per-second baseline.
    pub fn max_tps(&self) -> f64 {
        self.max_tps
    }

    /// Monotonic scheduled tick index (frozen while paused).
    pub fn tick_index(&self) -> u64 {
        self.tick_index
    }

    /// Alias for [`Self::tick_index`] — total ticks scheduled so far.
    pub fn scheduled_tick_count(&self) -> u64 {
        self.tick_index
    }

    /// Effective tick demand per wall-second while playing: `rate * max_tps`.
    pub fn effective_tps(&self) -> f64 {
        self.rate.as_f64() * self.max_tps
    }

    /// Advance the clock by deterministic wall elapsed seconds.
    ///
    /// Returns how many ticks were scheduled in this call. Does not invoke a driver
    /// session or mutate ScenarioSpec — callers (9.3) consume the count.
    pub fn advance(&mut self, elapsed_secs: f64) -> u64 {
        if self.paused {
            return 0;
        }
        if !elapsed_secs.is_finite() || elapsed_secs <= 0.0 {
            return 0;
        }
        self.accumulator += elapsed_secs * self.effective_tps();
        let whole = self.accumulator.floor();
        // Guard pathological float growth: never emit more than demand+1 for this step.
        let scheduled = whole.max(0.0) as u64;
        self.accumulator -= whole;
        if self.accumulator < 0.0 {
            self.accumulator = 0.0;
        }
        self.tick_index = self.tick_index.saturating_add(scheduled);
        scheduled
    }
}

/// Errors from clock configuration.
#[derive(Debug, Clone, PartialEq)]
pub enum StudioSimClockError {
    InvalidMaxTps { max_tps: f64 },
}

impl std::fmt::Display for StudioSimClockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidMaxTps { max_tps } => {
                write!(f, "max_tps must be finite and > 0 (got {max_tps})")
            }
        }
    }
}

impl std::error::Error for StudioSimClockError {}

#[cfg(test)]
mod unit_smoke {
    use super::*;

    #[test]
    fn default_is_paused_at_1x() {
        let clock = StudioSimClock::new();
        assert!(clock.is_paused());
        assert_eq!(clock.rate(), StudioSimClockRate::Rate1x);
        assert_eq!(clock.tick_index(), 0);
        assert_eq!(clock.max_tps(), STUDIO_SIM_CLOCK_DEFAULT_MAX_TPS);
    }
}
