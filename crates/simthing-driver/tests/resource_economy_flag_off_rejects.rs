//! Phase T-4 — flag-off populated spec rejection tests.

#[path = "support/resource_economy_session.rs"]
mod support;

use simthing_driver::{SimSession, ResourceEconomySyncError};
use support::{emission_game_mode, transfer_game_mode};

fn open_transfer_session() -> SimSession {
    let ron = include_str!("../../../scenarios/rebellion_demo.ron");
    let scenario = simthing_driver::Scenario::from_ron_str(ron).expect("scenario");
    SimSession::open_from_spec(scenario, &transfer_game_mode()).expect("open_from_spec")
}

fn open_emission_session() -> SimSession {
    let ron = include_str!("../../../scenarios/rebellion_demo.ron");
    let scenario = simthing_driver::Scenario::from_ron_str(ron).expect("scenario");
    SimSession::open_from_spec(scenario, &emission_game_mode()).expect("open_from_spec")
}

#[test]
fn resource_economy_flag_off_transfer_spec_rejects_boundary_sync() {
    let mut session = open_transfer_session();
    assert!(!session.proto.flags.use_accumulator_transfer);
    let err = session
        .sync_resource_economy_if_enabled()
        .expect_err("populated transfer spec must reject when flag is off");
    assert!(matches!(
        err,
        simthing_driver::SessionError::ResourceEconomy(
            ResourceEconomySyncError::TransferFlagOffPopulatedSpec
        )
    ));
}

#[test]
fn resource_economy_flag_off_emission_spec_rejects_boundary_sync() {
    let mut session = open_emission_session();
    assert!(!session.proto.flags.use_accumulator_emission);
    let err = session
        .sync_resource_economy_if_enabled()
        .expect_err("populated emission spec must reject when flag is off");
    assert!(matches!(
        err,
        simthing_driver::SessionError::ResourceEconomy(
            ResourceEconomySyncError::EmissionFlagOffPopulatedSpec
        )
    ));
}
