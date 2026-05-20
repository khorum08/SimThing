//! End-to-end GPU tests for `SimSession`. Skips cleanly when no adapter.

use std::io::BufReader;

use simthing_driver::{Scenario, SimSession};
use simthing_gpu::GpuContext;
use simthing_sim::{ReplayDriver, ReplayReader};

fn try_gpu() -> bool {
    GpuContext::new_blocking().is_ok()
}

#[test]
fn rebellion_demo_ron_runs_fission_via_sim_session() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let ron = include_str!("../../../scenarios/rebellion_demo.ron");
    let scenario = Scenario::from_ron_str(ron).expect("scenario parse");
    let mut session = SimSession::open(scenario).expect("session open");

    let summary = session.run(4).expect("session run");

    assert_eq!(summary.boundaries_run, 4);
    assert_eq!(summary.ticks_run, 4);
    assert!(
        summary.fission_events >= 1,
        "rebellion demo should fission within 4 days, got {}",
        summary.fission_events
    );
    assert_eq!(
        session.proto.root.subtree_size(),
        4,
        "world + location + cohort + one fission child"
    );
    assert_eq!(session.proto.fission_lineage().len(), 1);
}

#[test]
fn record_rebellion_demo_replay_round_trips_structural_state() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("session.replay.ldjson");

    let ron = include_str!("../../../scenarios/rebellion_demo.ron");
    let scenario = Scenario::from_ron_str(ron).expect("scenario parse");
    let mut session = SimSession::open(scenario).expect("session open");

    let summary = session
        .record_to_path(&path, 4)
        .expect("record session");
    assert_eq!(summary.frames_written, 4);
    assert!(summary.fission_events >= 1);

    let file = std::fs::File::open(&path).expect("replay file");
    let mut reader = ReplayReader::new(BufReader::new(file));
    let mut driver = ReplayDriver::from_snapshot(reader.read_snapshot().expect("snapshot"));

    let mut frames = 0u32;
    while let Some(frame) = reader.next_frame().expect("read frame") {
        driver.apply_frame(frame);
        frames += 1;
    }

    assert_eq!(frames, 4);
    assert_eq!(driver.root.subtree_size(), 4);
    assert_eq!(driver.fission_lineage.len(), 1);
}
