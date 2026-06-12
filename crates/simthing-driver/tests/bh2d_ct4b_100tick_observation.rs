//! BH-2D-OBS-100 — 100-tick CT-4b scenario observation pass (test-only).

mod support;

use std::path::PathBuf;
use std::sync::Mutex;

use simthing_gpu::GpuContext;
use support::ct4b_100tick_runner::{
    render_observation_markdown, run_observation_ticks, OBS_TICK_COUNT,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required for BH-2D-OBS-100");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn obs_report_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/tests/bh2d_ct4b_100tick_scenario_observations.md")
}

fn assert_observation_run_sane(run: &support::ct4b_100tick_runner::ObservationRun) {
    assert_eq!(run.ticks.len(), OBS_TICK_COUNT as usize);
    let last = &run.ticks[99];
    assert!(last.max_choke_a > 0.0, "choke_a must evolve over 100 ticks");
    assert!(last.overlap_peak >= 0.0);
    assert!(
        (last.profile1_probe_min_d - last.profile0_probe_min_d).abs() > 0.01
            || (last.anchor_w1 - last.anchor_w0).abs() > 0.01,
        "profiles must diverge by tick 99"
    );
    assert!(
        last.profile0_probe_min_d.is_finite(),
        "compact probe must be finite at anchor"
    );
}

#[test]
fn bh2d_ct4b_100tick_observation_smoke() {
    with_gpu(|ctx| {
        let run = run_observation_ticks(ctx, 10);
        assert_eq!(run.ticks.len(), 10);
        assert!(run.ticks[9].max_choke_a > 0.0);
    });
}

#[test]
#[ignore = "100-tick GPU observation pass; run explicitly to regenerate docs/tests report"]
fn bh2d_ct4b_100tick_observation() {
    with_gpu(|ctx| {
        let run = run_observation_ticks(ctx, OBS_TICK_COUNT);
        assert_observation_run_sane(&run);
        let markdown = render_observation_markdown(&run);
        assert!(markdown.contains("tick | max_choke_a"));
        assert!(markdown.contains("Candidate-F"));
        let path = obs_report_path();
        std::fs::write(&path, markdown).expect("write observation report");
        eprintln!("Wrote observation report to {}", path.display());
    });
}
