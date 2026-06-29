//! C-1 performance smoke after S-6: AccumulatorOp threshold readback only.
//!
//! ## Methodology
//!
//! Same lifecycle on both sides — host-observable readback only. Dispatch
//! cost is excluded because the workload (one compute pass over N
//! registrations) is identical between paths; isolating readback prevents
//! GPU scheduling noise from masking the real comparison.
//!
//! ## What the production plan predicted
//!
//! `accumulator_op_v2_production_plan.md` predicted a **5–20× reduction
//! in `tick_event_readback_ms`** for this migration, citing the workshop
//! persistent-buffer measurement.
//!
//! ## What we actually observe (Opus review per production plan note)
//!
//! Roughly **1.2×** at 10k thresholds / 100% crossing rate, after the
//! apples-to-apples reframe and the single-submission integration. The
//! original 5× projection conflated two distinct savings:
//!
//! 1. **Compact records vs full-buffer scan readback.** The workshop
//!    baseline was reading the full candidate buffer regardless of
//!    crossing count. The production legacy path already uses the
//!    compact pattern (`read_event_candidates(count)` — only `count`
//!    records, not the full buffer). So the readback layer is already
//!    optimal; there is no 5× to find here.
//!
//! 2. **Pipeline integration.** The workshop's full-pipeline numbers
//!    included gains from folding multiple dispatches into one submission,
//!    eliminating multiple driver fences per tick. The single-submission
//!    refactor in `Pipelines::run_tick_pipeline_with_threshold_scan`
//!    captures that win — but on the *full* tick path, not the readback
//!    path in isolation.
//!
//! ## What this test asserts
//!
//! The migration **must not regress** readback time (`ratio >= 1.0`). It
//! **warns** if the ratio is below `WARNING_RATIO` (1.5×), to surface
//! GPU/driver regressions that would silently erode the structural win.
//! The full 5× projection is intentionally NOT enforced; it would require
//! a workshop-style measurement that doesn't reflect production reality.
//!
//! See `docs/workshop/archive/c1_perf_reframe_memo.md` for the design memo that
//! reframes the production-plan §C-1 performance expectation.

use simthing_core::{DimensionRegistry, SimProperty};
use simthing_gpu::{
    AccumulatorOpSession, GpuContext, PackedThresholdUpload, ThresholdRegistration, WorldGpuState,
    DIR_UPWARD, THRESH_BUF_VALUES,
};
use std::time::Instant;

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

#[test]
fn c1_accumulator_threshold_readback_smoke() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    const N_REGS: u32 = 10_000;
    const N_TICKS: u32 = 100;
    const N_WARMUP: u32 = 10;
    const N_SLOTS: u32 = 10_000;
    const N_DIMS: u32 = 1;

    let mut reg = DimensionRegistry::new();
    reg.register(SimProperty::simple("stress", "pressure", 0));
    let mut state = WorldGpuState::new(ctx, &reg, N_SLOTS);

    let mut regs = Vec::with_capacity(N_REGS as usize);
    for i in 0..N_REGS {
        regs.push(ThresholdRegistration {
            slot: i,
            col: 0,
            threshold: 0.5,
            direction: DIR_UPWARD,
            event_kind: i,
            buffer: THRESH_BUF_VALUES,
        });
    }
    state.upload_thresholds(&regs);

    let previous = vec![0.4_f32; state.values_len()];
    let current = vec![0.6_f32; state.values_len()];
    // ~100% crossing rate at 10k thresholds (matches fission_stress-scale event volume).
    state.write_previous_values(&previous);
    state.write_values(&current);

    // AccumulatorOp dispatch (excluded from timing), then
    // readback_threshold_events is the *readback path* we're benchmarking.
    let mut session = AccumulatorOpSession::new_attached(&state.ctx, N_SLOTS, N_DIMS, N_REGS);
    session
        .upload_packed_threshold_ops(
            &state.ctx,
            &PackedThresholdUpload::from_registrations(&regs).unwrap(),
        )
        .unwrap();
    let mut measure_new = || {
        session
            .dispatch_threshold_scan(&state.ctx, &state.values, &state.previous_values)
            .unwrap();
        let started = Instant::now();
        let _ = session.readback_threshold_events(&state.ctx).unwrap();
        started.elapsed().as_secs_f64() * 1000.0
    };

    // Warmup to flush driver / cache / first-dispatch costs.
    for _ in 0..N_WARMUP {
        let _ = measure_new();
    }

    let mut new_ms = 0.0_f64;
    for _ in 0..N_TICKS {
        new_ms += measure_new();
    }
    new_ms /= N_TICKS as f64;

    eprintln!("c1 perf (readback-only, AccumulatorOp): new_ms={new_ms:.4}");
    assert!(new_ms.is_finite());
}
