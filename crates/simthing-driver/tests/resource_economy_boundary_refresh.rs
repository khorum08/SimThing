//! Phase T-5 — resource economy boundary refresh tests.

#[path = "support/resource_economy_session.rs"]
mod support;

use simthing_driver::{ResourceEconomySyncError, SessionError};
use support::{
    emission_game_mode, open_rebellion_transfer_session, transfer_game_mode, try_gpu,
};

#[test]
fn resource_economy_boundary_refresh_runs_after_structural_boundary() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let mut session = open_rebellion_transfer_session();
    let uploads_before = session
        .state
        .accumulator_runtime
        .as_ref()
        .map(|r| r.transfer_op_upload_count())
        .unwrap_or(0);
    assert!(uploads_before >= 1, "install sync must upload transfer ops");

    let summary = session.run(1).expect("boundary run with transfer flag on");
    assert!(summary.boundaries_run >= 1, "session must execute at least one boundary");
    assert!(
        session.state.accumulator_transfer_active,
        "transfer dispatch must remain active after boundary refresh"
    );
}

#[test]
fn resource_economy_boundary_refresh_generation_skip_stable() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let mut session = open_rebellion_transfer_session();
    session.run(1).expect("first boundary");
    let after_first = session
        .state
        .accumulator_runtime
        .as_ref()
        .map(|r| r.transfer_op_upload_count())
        .unwrap_or(0);

    session.run(1).expect("second boundary");
    let after_second = session
        .state
        .accumulator_runtime
        .as_ref()
        .map(|r| r.transfer_op_upload_count())
        .unwrap_or(0);

    assert_eq!(
        after_first, after_second,
        "unchanged generation must skip re-upload across boundaries"
    );
    assert_eq!(session.spec_state.resource_economy_uploaded_generation(), 1);
}

#[test]
fn resource_economy_boundary_refresh_reuploads_after_generation_change() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let mut session = open_rebellion_transfer_session();
    session.sync_resource_economy_if_enabled().expect("initial sync");
    {
        let registry = session
            .spec_state
            .resource_economy_registry
            .as_mut()
            .expect("registry");
        registry.generation = 2;
    }
    session.sync_resource_economy_if_enabled().expect("generation bump sync");
    assert_eq!(
        session.spec_state.resource_economy_uploaded_generation(),
        2,
        "generation bump must update uploaded generation marker"
    );
}

#[test]
fn resource_economy_boundary_refresh_flag_off_rejects_populated_transfer() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let ron = include_str!("../../../scenarios/rebellion_demo.ron");
    let scenario = simthing_driver::Scenario::from_ron_str(ron).expect("scenario");
    let mut session =
        simthing_driver::SimSession::open_from_spec(scenario, &transfer_game_mode()).expect("open");
    assert!(!session.proto.flags.use_accumulator_transfer);

    let err = session
        .sync_resource_economy_if_enabled()
        .expect_err("boundary sync must reject populated transfer spec when flag is off");
    assert!(matches!(
        err,
        SessionError::ResourceEconomy(ResourceEconomySyncError::TransferFlagOffPopulatedSpec)
    ));
}

#[test]
fn resource_economy_boundary_refresh_flag_off_rejects_populated_emission() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let ron = include_str!("../../../scenarios/rebellion_demo.ron");
    let scenario = simthing_driver::Scenario::from_ron_str(ron).expect("scenario");
    let mut session =
        simthing_driver::SimSession::open_from_spec(scenario, &emission_game_mode()).expect("open");
    assert!(!session.proto.flags.use_accumulator_emission);

    let err = session
        .sync_resource_economy_if_enabled()
        .expect_err("boundary sync must reject populated emission spec when flag is off");
    assert!(matches!(
        err,
        SessionError::ResourceEconomy(ResourceEconomySyncError::EmissionFlagOffPopulatedSpec)
    ));
}
