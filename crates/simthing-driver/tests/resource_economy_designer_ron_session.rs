//! Phase T designer-authored resource economy RON session smoke coverage.

use simthing_core::{
    DimensionRegistry, PropertyValue, SimProperty, SimThing, SimThingKind, SubFieldRole,
};
use simthing_driver::{Scenario, SimSession};
use simthing_gpu::GpuContext;
use simthing_spec::deserialize_game_mode_ron;

const FIXTURE: &str =
    include_str!("../../simthing-spec/tests/fixtures/game_modes/resource_economy_smoke.ron");

fn try_gpu() -> bool {
    GpuContext::new_blocking().is_ok()
}

fn fixture_scenario() -> Scenario {
    let mut registry = DimensionRegistry::new();
    let boot = registry.register(SimProperty::simple("session", "boot", 0));
    let boot_layout = registry.property(boot).layout.clone();
    let mut root = SimThing::new(SimThingKind::World, 0);
    root.add_property(boot, PropertyValue::from_layout(&boot_layout));

    Scenario {
        name: "phase_t_designer_resource_economy".into(),
        ticks_per_day: 1,
        max_days: 5,
        dt: 1.0,
        n_slots: 8,
        registry,
        root,
        shadow_seeds: vec![],
        tick_patches: vec![],
        install_targets: Default::default(),
    }
}

#[test]
fn resource_economy_designer_ron_open_from_spec_succeeds() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let game_mode = deserialize_game_mode_ron(FIXTURE).expect("designer RON fixture parses");
    let session =
        SimSession::open_from_spec(fixture_scenario(), &game_mode).expect("open_from_spec");
    let registry = session
        .spec_state
        .resource_economy_registry
        .as_ref()
        .expect("resource economy registry stored");

    assert!(session.proto.flags.use_accumulator_transfer);
    assert!(session.proto.flags.use_accumulator_eml);
    assert!(session.proto.flags.use_accumulator_emission);
    assert!(
        !session.proto.flags.use_accumulator_resource_flow,
        "resource economy RON must not change Resource Flow posture"
    );
    assert_eq!(registry.generation, 1);
    assert_eq!(registry.registrations.transfers.len(), 1);
    assert_eq!(registry.registrations.recipes.len(), 1);
    assert_eq!(registry.registrations.emissions.len(), 1);
    assert_eq!(session.spec_state.resource_economy_uploaded_generation(), 1);
}

#[test]
fn resource_economy_designer_ron_short_run_conservation_or_no_error() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let game_mode = deserialize_game_mode_ron(FIXTURE).expect("designer RON fixture parses");
    let mut session =
        SimSession::open_from_spec(fixture_scenario(), &game_mode).expect("open_from_spec");
    let summary = session.run(5).expect("short resource economy fixture run");

    assert_eq!(summary.boundaries_run, 5);
    assert!(summary.ticks_run >= 5);
    assert!(session.state.accumulator_transfer_active);
    assert!(session.state.accumulator_emission_active);
}

#[test]
fn resource_economy_designer_ron_materializes_transfer_recipe_and_emission_slots() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let game_mode = deserialize_game_mode_ron(FIXTURE).expect("designer RON fixture parses");
    let session =
        SimSession::open_from_spec(fixture_scenario(), &game_mode).expect("open_from_spec");
    let registrations = &session
        .spec_state
        .resource_economy_registry
        .as_ref()
        .expect("resource economy registry stored")
        .registrations;

    let transfer = &registrations.transfers[0];
    assert_eq!(transfer.source_slot, 0);
    assert_eq!(transfer.target_slot, 0);
    assert_ne!(transfer.source_col, transfer.target_col);
    assert_eq!(transfer.amount, 1.0);

    let recipe = &registrations.recipes[0];
    assert_eq!(recipe.inputs.len(), 2);
    assert_eq!(recipe.target_slot, 0);
    assert_eq!(recipe.throttle_hint_max_per_tick, 4);
    assert!(
        recipe.inputs.iter().all(|input| input.slot == 0),
        "auto-populated fixture properties should resolve to the world root slot"
    );
    assert!(
        recipe
            .inputs
            .iter()
            .all(|input| input.col != recipe.target_col),
        "recipe inputs and output should resolve to distinct property columns"
    );

    let emission = &registrations.emissions[0];
    assert_eq!(emission.source_slot, 0);
    assert!(matches!(
        emission.formula,
        simthing_gpu::EmissionFormula::Constant { value } if value == 1.0
    ));

    let amount = SubFieldRole::Named("amount".into());
    let signal = session
        .proto
        .registry
        .id_of("core", "signal")
        .expect("signal property registered");
    let signal_value = session
        .proto
        .root
        .properties
        .get(&signal)
        .unwrap_or_else(|| panic!("resource economy property auto-populated on root"));
    let signal_layout = &session.proto.registry.property(signal).layout;
    assert_eq!(signal_value.get_role(&amount, signal_layout), 1.0);
}
