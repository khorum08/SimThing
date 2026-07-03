//! Phase T-5 — resource economy 100-tick conservation burn-in tests.

#[path = "support/resource_economy_session.rs"]
mod support;

use simthing_driver::{run_emission_burn_in, run_transfer_recipe_burn_in};
use simthing_sim::PipelineFlags;
use support::{
    amount_col, open_live_emission_session, open_live_transfer_session, open_recipe_session,
    try_gpu,
};

#[test]
fn resource_economy_transfer_100_ticks_conserves_source_target_total() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let session = open_live_transfer_session();
    let reg = session.proto.registry.clone();
    let food_col = amount_col(&reg, "core", "food");
    let store_col = amount_col(&reg, "core", "store");
    let cohort_slot = support::cohort_food_slot(&support::live_slot_scenario());
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
    flat[((cohort_slot * n_dims + food_col) as usize)] = 200.0;
    flat[((0 * n_dims + store_col) as usize)] = 5.0;

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
fn resource_economy_recipe_100_ticks_conserves_inputs_and_outputs_as_expected() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let session = open_recipe_session();
    let reg = session.proto.registry.clone();
    let food_col = amount_col(&reg, "core", "food");
    let ore_col = amount_col(&reg, "core", "ore");
    let product_col = amount_col(&reg, "core", "product");
    let n_dims = reg.total_columns as u32;

    let recipes = session
        .spec_state
        .resource_economy_registry
        .as_ref()
        .unwrap()
        .registrations
        .recipes
        .clone();

    let mut flat = vec![0.0_f32; (session.state.n_slots * n_dims) as usize];
    flat[(0 * n_dims + food_col) as usize] = 300.0;
    flat[(0 * n_dims + ore_col) as usize] = 400.0;
    flat[(0 * n_dims + product_col) as usize] = 0.0;

    let watched = [(0, food_col), (0, ore_col), (0, product_col)];

    let mut state = session.state;
    let report =
        run_transfer_recipe_burn_in(&mut state, n_dims, &flat, &[], &recipes, &watched, 100, 1.0)
            .expect("recipe burn-in");

    assert_eq!(report.ticks_checked, 100);
    assert_eq!(
        report.max_abs_conservation_error.to_bits(),
        0.0_f32.to_bits()
    );
    assert!(report.replay_bit_exact);
}

#[test]
fn resource_economy_emission_100_ticks_matches_oracle() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let session = open_live_emission_session();
    let reg = session.proto.registry.clone();
    let food_col = amount_col(&reg, "core", "food");
    let cohort_slot = support::cohort_food_slot(&support::live_slot_scenario());
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
    flat[((cohort_slot * n_dims + food_col) as usize)] = 7.75;

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
fn resource_economy_transfer_and_emission_flags_default_false() {
    let flags = PipelineFlags::default();
    assert!(!flags.use_accumulator_transfer);
    assert!(!flags.use_accumulator_emission);
}

