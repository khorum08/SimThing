//! Phase T-6 — limited opt-in scenario flagging for resource economy execution.

#[path = "support/resource_economy_session.rs"]
mod support;

use simthing_driver::{
    run_emission_burn_in, run_transfer_recipe_burn_in, ResourceEconomySyncError, SimSession,
};
use simthing_spec::{
    EmissionFormulaSpec, ResourceEconomyOptInMode, ResourceEconomySpec, ResourceEmissionSpec,
};
use support::{
    amount_col, amount_transfer, base_game_mode, cohort_food_slot, identity_emission,
    live_slot_game_mode, live_slot_scenario, try_gpu,
};

fn opt_in_transfer_mode() -> simthing_spec::GameModeSpec {
    let mut mode = live_slot_game_mode();
    mode.resource_economy.as_mut().unwrap().opt_in_mode = ResourceEconomyOptInMode::TransferOnly;
    mode
}

fn opt_in_emission_mode() -> simthing_spec::GameModeSpec {
    let mut mode = base_game_mode();
    mode.resource_economy = Some(ResourceEconomySpec {
        opt_in_mode: ResourceEconomyOptInMode::EmissionOnly,
        emissions: vec![identity_emission("e1", "food")],
        ..Default::default()
    });
    mode
}

fn opt_in_transfer_and_emission_mode() -> simthing_spec::GameModeSpec {
    let mut mode = base_game_mode();
    mode.resource_economy = Some(ResourceEconomySpec {
        opt_in_mode: ResourceEconomyOptInMode::TransferAndEmission,
        transfers: vec![amount_transfer("t1", "food", "store", 1.0)],
        emissions: vec![identity_emission("e1", "food")],
        ..Default::default()
    });
    mode
}

fn open_live(mode: &simthing_spec::GameModeSpec) -> SimSession {
    SimSession::open_from_spec(live_slot_scenario(), mode).expect("open_from_spec")
}

#[test]
fn resource_economy_opt_in_transfer_enables_transfer_flag_only() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let session = open_live(&opt_in_transfer_mode());
    assert!(session.proto.flags.use_accumulator_transfer);
    assert!(!session.proto.flags.use_accumulator_emission);
    assert!(!session.proto.flags.use_accumulator_resource_flow);
    assert!(session.state.accumulator_transfer_active);
    assert!(!session.state.accumulator_emission_active);
}

#[test]
fn resource_economy_opt_in_emission_enables_emission_and_eml_flags_only() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let session = open_live(&opt_in_emission_mode());
    assert!(session.proto.flags.use_accumulator_eml);
    assert!(session.proto.flags.use_accumulator_emission);
    assert!(!session.proto.flags.use_accumulator_transfer);
    assert!(!session.proto.flags.use_accumulator_resource_flow);
    assert!(session.state.accumulator_emission_active);
    assert!(!session.state.accumulator_transfer_active);
}

#[test]
fn resource_economy_opt_in_transfer_and_emission_enables_both_paths() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let session = open_live(&opt_in_transfer_and_emission_mode());
    assert!(session.proto.flags.use_accumulator_transfer);
    assert!(session.proto.flags.use_accumulator_eml);
    assert!(session.proto.flags.use_accumulator_emission);
    assert!(!session.proto.flags.use_accumulator_resource_flow);
    assert!(session.state.accumulator_transfer_active);
    assert!(session.state.accumulator_emission_active);
}

#[test]
fn resource_economy_default_session_flags_remain_false() {
    let flags = simthing_sim::PipelineFlags::default();
    assert!(!flags.use_accumulator_transfer);
    assert!(!flags.use_accumulator_emission);
    assert!(!flags.use_accumulator_resource_flow);

    let spec = ResourceEconomySpec::default();
    assert_eq!(spec.opt_in_mode, ResourceEconomyOptInMode::Disabled);
}
#[test]
fn resource_economy_opt_in_transfer_runs_100_tick_burn_in() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let session = open_live(&opt_in_transfer_mode());
    let reg = session.proto.registry.clone();
    let food_col = amount_col(&reg, "core", "food");
    let store_col = amount_col(&reg, "core", "store");
    let cohort_slot = cohort_food_slot(&support::live_slot_scenario());
    let n_dims = reg.total_columns as u32;
    let transfers = session
        .spec_state
        .resource_economy_registry
        .as_ref()
        .unwrap()
        .registrations
        .transfers
        .clone();

    let mut flat = vec![0.0_f32; (session.state.n_slots * n_dims) as usize];
    flat[(cohort_slot * n_dims + food_col) as usize] = 200.0;
    flat[(store_col) as usize] = 5.0;

    let mut state = session.state;
    let report = run_transfer_recipe_burn_in(
        &mut state,
        n_dims,
        &flat,
        &transfers,
        &[],
        &[(cohort_slot, food_col), (0, store_col)],
        100,
        1.0,
    )
    .expect("transfer burn-in");

    assert_eq!(report.ticks_checked, 100);
    assert_eq!(
        report.max_abs_conservation_error.to_bits(),
        0.0_f32.to_bits()
    );
    assert!(report.replay_bit_exact);
}

#[test]
fn resource_economy_opt_in_emission_runs_100_tick_burn_in() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let session = open_live(&opt_in_emission_mode());
    let reg = session.proto.registry.clone();
    let food_col = amount_col(&reg, "core", "food");
    let cohort_slot = cohort_food_slot(&support::live_slot_scenario());
    let n_dims = reg.total_columns as u32;
    let emissions = session
        .spec_state
        .resource_economy_registry
        .as_ref()
        .unwrap()
        .registrations
        .emissions
        .clone();

    let mut flat = vec![0.0_f32; (session.state.n_slots * n_dims) as usize];
    flat[(cohort_slot * n_dims + food_col) as usize] = 7.75;

    let mut state = session.state;
    let report = run_emission_burn_in(&mut state, n_dims, &flat, &emissions, 100, 1.0)
        .expect("emission burn-in");

    assert_eq!(report.ticks_checked, 100);
    assert_eq!(
        report.max_abs_conservation_error.to_bits(),
        0.0_f32.to_bits()
    );
    assert!(report.replay_bit_exact);
}

#[test]
fn resource_economy_opt_in_does_not_enable_resource_flow() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    for mode in [
        opt_in_transfer_mode(),
        opt_in_emission_mode(),
        opt_in_transfer_and_emission_mode(),
    ] {
        let session = open_live(&mode);
        assert!(
            !session.proto.flags.use_accumulator_resource_flow,
            "resource economy opt-in must not enable Resource Flow"
        );
    }
}

#[test]
fn resource_economy_opt_in_no_simthing_sim_spec_import() {
    let sim_cargo = include_str!("../../simthing-sim/Cargo.toml");
    assert!(
        !sim_cargo.contains("simthing-spec"),
        "simthing-sim must remain spec-free"
    );
}

#[test]
fn resource_economy_opt_in_mode_roundtrips_ron() {
    let spec = ResourceEconomySpec {
        opt_in_mode: ResourceEconomyOptInMode::EmissionOnly,
        emissions: vec![ResourceEmissionSpec {
            id: "emit_food".into(),
            source: simthing_spec::PropertyKey::new("core", "food"),
            source_role: simthing_core::SubFieldRole::Named("amount".into()),
            formula: EmissionFormulaSpec::IdentityFloor,
        }],
        ..Default::default()
    };
    let text = ron::ser::to_string(&spec).expect("serialize resource economy");
    assert!(text.contains("EmissionOnly"));
    let round: ResourceEconomySpec = ron::from_str(&text).expect("roundtrip resource economy");
    assert_eq!(round.opt_in_mode, ResourceEconomyOptInMode::EmissionOnly);
    assert_eq!(round, spec);
}
