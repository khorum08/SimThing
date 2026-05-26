//! Loose perf ceilings for `simthing bench --check`.
//!
//! Baselines:
//! - v5 §18 legacy path: intent_stress ~20 ms/day, fission_stress ~53 ms/day.
//! - v7.5 S-1/S-5/S-6 sunset path: intent_stress routes 10k folded intents
//!   through mandatory AccumulatorOp registrations; local runs are ~170-260 ms/day.
//! Ceilings keep headroom for CI variance and slower GPUs.

use crate::RunSummary;

pub struct BenchCeiling {
    pub name: &'static str,
    pub max_ms_per_sim_day: f64,
}

pub const CEILINGS: &[BenchCeiling] = &[
    BenchCeiling {
        name: "intent_stress",
        max_ms_per_sim_day: 350.0,
    },
    BenchCeiling {
        name: "fission_stress",
        max_ms_per_sim_day: 200.0,
    },
];

pub fn ms_per_sim_day(elapsed_ms: f64, boundaries_run: u64) -> f64 {
    if boundaries_run == 0 {
        0.0
    } else {
        elapsed_ms / boundaries_run as f64
    }
}

pub fn check(scenario_name: &str, elapsed_ms: f64, summary: &RunSummary) -> Result<f64, String> {
    let ms_per_day = ms_per_sim_day(elapsed_ms, summary.boundaries_run);
    let Some(ceiling) = CEILINGS.iter().find(|c| c.name == scenario_name) else {
        return Ok(ms_per_day);
    };
    if summary.boundaries_run == 0 {
        return Ok(ms_per_day);
    }
    if ms_per_day > ceiling.max_ms_per_sim_day {
        return Err(format!(
            "{scenario_name}: {ms_per_day:.3} ms/sim-day exceeds ceiling {:.3}",
            ceiling.max_ms_per_sim_day
        ));
    }
    if scenario_name == "intent_stress" && summary.rmw_readback_bytes != 0 {
        return Err(format!(
            "intent_stress: expected zero RMW readback bytes, got {}",
            summary.rmw_readback_bytes
        ));
    }
    Ok(ms_per_day)
}
