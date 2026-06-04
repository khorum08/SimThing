//! Phase T-5 — resource economy replay determinism tests.

#[path = "support/resource_economy_session.rs"]
mod support;

use simthing_driver::{
    open_replay_with_spec, read_spec_replay_file, run_transfer_recipe_burn_in, SimSession,
};
use support::{
    amount_col, live_slot_game_mode, live_slot_scenario, open_live_transfer_session,
    transfer_game_mode, try_gpu,
};

#[test]
fn resource_economy_replay_same_seed_same_frames() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let run_once = || {
        let mut session = open_live_transfer_session();
        let reg = session.proto.registry.clone();
        let food_col = amount_col(&reg, "core", "food");
        let store_col = amount_col(&reg, "core", "store");
        let cohort_slot = support::cohort_food_slot(&live_slot_scenario());
        let n_dims = reg.total_columns as u32;
        let mut flat = vec![0.0_f32; (session.state.n_slots * n_dims) as usize];
        flat[((cohort_slot * n_dims + food_col) as usize)] = 12.0;
        flat[((0 * n_dims + store_col) as usize)] = 3.0;

        let transfers = session
            .spec_state
            .resource_economy_registry
            .as_ref()
            .unwrap()
            .registrations
            .transfers
            .clone();
        run_transfer_recipe_burn_in(
            &mut session.state,
            n_dims,
            &flat,
            &transfers,
            &[],
            &[(cohort_slot, food_col), (0, store_col)],
            4,
            1.0,
        )
        .expect("burn-in")
    };

    let report_a = run_once();
    let report_b = run_once();
    assert_eq!(report_a.ticks_checked, 4);
    assert_eq!(report_a.max_abs_error(), report_b.max_abs_error());
    assert!(report_a.replay_bit_exact);
}

#[test]
fn resource_economy_replay_records_spec_snapshot_with_resource_economy_registry() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let scenario = live_slot_scenario();
    let game_mode = live_slot_game_mode();
    let mut session = SimSession::open_from_spec(scenario.clone(), &game_mode).expect("open");
    assert!(
        session
            .spec_state
            .resource_economy_registry
            .as_ref()
            .is_some(),
        "session must store materialized registry before record"
    );
    let live_transfer_count = session
        .spec_state
        .resource_economy_registry
        .as_ref()
        .unwrap()
        .registrations
        .transfers
        .len();

    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("resource_economy.replay.ldjson");
    session.record_to_path(&path, 1).expect("record");

    let loaded = read_spec_replay_file(&path).expect("read replay");
    assert!(
        loaded.spec_snapshot.is_some(),
        "resource economy session must emit spec_snapshot line"
    );

    let (replay_session, _driver, _frames) =
        open_replay_with_spec(&path, &game_mode, scenario).expect("replay open");
    let replay_registry = replay_session
        .spec_state
        .resource_economy_registry
        .as_ref()
        .expect("replay reinstall must restore resource economy registry");
    assert_eq!(
        replay_registry.registrations.transfers.len(),
        live_transfer_count
    );
    assert_eq!(replay_registry.generation, 1);
}

#[test]
fn resource_economy_replay_boundary_sync_deterministic() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let record_path = |path: &std::path::Path| {
        let ron = include_str!("../../../scenarios/rebellion_demo.ron");
        let mut scenario = simthing_driver::Scenario::from_ron_str(ron).expect("scenario");
        scenario.max_days = 2;
        let game_mode = transfer_game_mode();
        let mut session = SimSession::open_from_spec(scenario.clone(), &game_mode).expect("open");
        session.proto.flags.use_accumulator_transfer = true;
        session.sync_resource_economy_if_enabled().expect("sync");
        session.record_to_path(path, 2).expect("record");
        (
            session
                .state
                .accumulator_runtime
                .as_ref()
                .map(|r| r.transfer_op_upload_count())
                .unwrap_or(0),
            game_mode,
            scenario,
        )
    };

    let dir = tempfile::tempdir().expect("tempdir");
    let path_a = dir.path().join("a.replay.ldjson");
    let path_b = dir.path().join("b.replay.ldjson");
    let (uploads_a, game_mode, scenario) = record_path(&path_a);
    let (uploads_b, _, _) = record_path(&path_b);
    assert_eq!(uploads_a, uploads_b);

    let loaded_a = read_spec_replay_file(&path_a).expect("read a");
    let loaded_b = read_spec_replay_file(&path_b).expect("read b");
    assert_eq!(loaded_a.frames.len(), loaded_b.frames.len());

    let (replay_a, _, _) =
        open_replay_with_spec(&path_a, &game_mode, scenario.clone()).expect("open a");
    let (replay_b, _, _) = open_replay_with_spec(&path_b, &game_mode, scenario).expect("open b");
    assert_eq!(
        replay_a.spec_state.resource_economy_uploaded_generation(),
        replay_b.spec_state.resource_economy_uploaded_generation()
    );
}

trait BurnInReportExt {
    fn max_abs_error(&self) -> f32;
}

impl BurnInReportExt for simthing_driver::ResourceEconomyBurnInReport {
    fn max_abs_error(&self) -> f32 {
        self.max_abs_conservation_error
    }
}
