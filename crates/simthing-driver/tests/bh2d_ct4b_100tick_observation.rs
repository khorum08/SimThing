//! BH-2D-OBS-100R — 100-tick CT-4b dynamic scenario observation pass (test-only).

mod support;

use std::path::PathBuf;
use std::sync::Mutex;

use simthing_gpu::GpuContext;
use support::ct4b_100tick_runner::{
    render_observation_markdown, run_observation_ticks, OBS_TICK_COUNT,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required for BH-2D-OBS-100R");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn obs_report_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/tests/bh2d_ct4b_100tick_scenario_observations.md")
}

fn assert_observation_run_dynamic(run: &support::ct4b_100tick_runner::ObservationRun) {
    assert_eq!(run.ticks.len(), OBS_TICK_COUNT as usize);
    let first = &run.ticks[0];
    let last = &run.ticks[99];

    assert!(last.max_choke_a > 0.0, "choke_a must be non-zero");
    assert!(
        last.profile0_probe_min_d.is_finite(),
        "compact probe must be finite at anchor"
    );

    let velocity_active = run.ticks.iter().any(|t| t.velocity_peak > 0.01);
    let d_shifted = (last.profile0_probe_min_d - first.profile0_probe_min_d).abs() > 0.05
        || (last.profile1_probe_min_d - first.profile1_probe_min_d).abs() > 0.05;
    let displacement = last.mean_displacement > 0.5;
    let candidates_moved = run.ticks.iter().any(|t| t.moved_candidates > 0);
    let choke_shifted = (last.max_choke_a - first.max_choke_a).abs() > 0.001
        || (last.overlap_peak - first.overlap_peak).abs() > 0.001;

    assert!(
        velocity_active || d_shifted || displacement || candidates_moved || choke_shifted,
        "100-tick run must show dynamic pressure/probe/displacement signal (not flat steady state)"
    );
}

#[test]
fn bh2d_ct4b_100tick_observation_smoke() {
    with_gpu(|ctx| {
        let run = run_observation_ticks(ctx, 10);
        assert_eq!(run.ticks.len(), 10);
        assert!(run.ticks[9].max_choke_a > 0.0);
        assert!(
            run.ticks
                .iter()
                .any(|t| t.moved_candidates > 0 || t.velocity_peak > 0.01),
            "smoke run should show early dynamic signal"
        );
    });
}

#[test]
#[ignore = "100-tick GPU observation pass; run explicitly to regenerate docs/tests report"]
fn bh2d_ct4b_100tick_observation() {
    with_gpu(|ctx| {
        let run = run_observation_ticks(ctx, OBS_TICK_COUNT);
        assert_observation_run_dynamic(&run);
        let markdown = render_observation_markdown(&run);
        assert!(markdown.contains("BH-2D-OBS-100R"));
        assert!(markdown.contains("moved_candidates"));
        assert!(markdown.contains("Candidate-F"));
        let path = obs_report_path();
        std::fs::write(&path, markdown).expect("write observation report");
        eprintln!("Wrote observation report to {}", path.display());
    });
}
