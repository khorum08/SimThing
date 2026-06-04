//! Phase T-4 — resource economy session open + sync tests.

#[path = "support/resource_economy_session.rs"]
mod support;

use simthing_driver::SimSession;
use simthing_spec::GameModeSpec;
use support::{
    emission_game_mode, live_slot_game_mode, live_slot_scenario, transfer_game_mode, try_gpu,
};

fn open_with(game_mode: &GameModeSpec) -> SimSession {
    let ron = include_str!("../../../scenarios/rebellion_demo.ron");
    let scenario = simthing_driver::Scenario::from_ron_str(ron).expect("scenario");
    SimSession::open_from_spec(scenario, game_mode).expect("open_from_spec")
}

#[test]
fn resource_economy_session_open_stores_registry() {
    let session = open_with(&transfer_game_mode());
    let registry = session
        .spec_state
        .resource_economy_registry
        .as_ref()
        .expect("resource economy registry stored");
    assert_eq!(registry.registrations.transfers.len(), 1);
    assert_eq!(registry.generation, 1);
}

#[test]
fn resource_economy_flag_on_transfer_uploads_existing_accumulator_path() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }
    let mut session = open_with(&transfer_game_mode());
    session.proto.flags.use_accumulator_transfer = true;
    session
        .sync_resource_economy_if_enabled()
        .expect("transfer sync");
    assert!(session.state.accumulator_transfer_active);
    let uploads = session
        .state
        .accumulator_runtime
        .as_ref()
        .map(|r| r.transfer_op_upload_count())
        .unwrap_or(0);
    assert!(
        uploads >= 1,
        "expected transfer op upload via existing path"
    );
}

#[test]
fn resource_economy_flag_on_emission_uploads_existing_accumulator_path() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }
    let mut session = open_with(&emission_game_mode());
    session.proto.flags.use_accumulator_eml = true;
    session.proto.flags.use_accumulator_emission = true;
    session
        .sync_resource_economy_if_enabled()
        .expect("emission sync");
    assert!(session.state.accumulator_emission_active);
    let uploads = session
        .state
        .accumulator_runtime
        .as_ref()
        .map(|r| r.emission_op_upload_count())
        .unwrap_or(0);
    assert!(
        uploads >= 1,
        "expected emission op upload via existing path"
    );
}

#[test]
fn resource_economy_generation_keyed_skip_avoids_reupload_when_unchanged() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }
    let mut session = open_with(&transfer_game_mode());
    session.proto.flags.use_accumulator_transfer = true;
    session
        .sync_resource_economy_if_enabled()
        .expect("first sync");
    let first = session
        .state
        .accumulator_runtime
        .as_ref()
        .map(|r| r.transfer_op_upload_count())
        .unwrap_or(0);
    session
        .sync_resource_economy_if_enabled()
        .expect("second sync");
    let second = session
        .state
        .accumulator_runtime
        .as_ref()
        .map(|r| r.transfer_op_upload_count())
        .unwrap_or(0);
    assert_eq!(
        first, second,
        "generation-keyed skip should avoid re-upload"
    );
    assert_eq!(session.spec_state.resource_economy_uploaded_generation(), 1);
}

#[test]
fn resource_economy_session_uses_live_slot_resolution_not_property_id_placeholder() {
    let scenario = live_slot_scenario();
    let cohort_slot = support::cohort_food_slot(&scenario);
    assert_ne!(
        cohort_slot, 0,
        "cohort slot must differ from property-id placeholder 0"
    );

    let session =
        SimSession::open_from_spec(scenario, &live_slot_game_mode()).expect("open_from_spec");
    let transfer = &session
        .spec_state
        .resource_economy_registry
        .as_ref()
        .expect("registry")
        .registrations
        .transfers[0];
    assert_eq!(
        transfer.source_slot, cohort_slot,
        "food source must resolve to cohort live slot, not property id 0"
    );
    assert_eq!(transfer.target_slot, 0, "store on world root uses slot 0");
}

#[test]
fn resource_economy_simthing_sim_remains_spec_free() {
    let sim_cargo = include_str!("../../simthing-sim/Cargo.toml");
    assert!(
        !sim_cargo.contains("simthing-spec"),
        "simthing-sim must not depend on simthing-spec"
    );
}
