//! Shared helpers for resource economy session integration tests.

#![allow(dead_code)]

use simthing_core::{
    ClampBehavior, DimensionRegistry, PropertyLayout, PropertyValue, SimProperty, SimThing,
    SimThingKind, SubFieldRole, SubFieldSpec,
};
use simthing_driver::Scenario;
use simthing_gpu::{GpuContext, SlotAllocator};
use simthing_spec::{
    metadata::DisplayMeta, version::SpecVersion, EmissionFormulaSpec, GameModeSpec,
    PropertyKey, PropertySpec, ResourceEconomySpec, ResourceEmissionSpec, ResourceTransferSpec,
};

pub fn try_gpu() -> bool {
    GpuContext::new_blocking().is_ok()
}

pub fn amount_property_spec(ns: &str, name: &str) -> PropertySpec {
    PropertySpec {
        id: format!("{ns}_{name}"),
        namespace: ns.into(),
        name: name.into(),
        display_name: name.into(),
        description: String::new(),
        sub_fields: vec![SubFieldSpec {
            role: SubFieldRole::Named("amount".into()),
            width: 1,
            clamp: ClampBehavior::Unbounded,
            velocity_max: None,
            default: 0.0,
            display_name: "amount".into(),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: None,
        }],
    }
}

pub fn amount_transfer(id: &str, source: &str, target: &str, amount: f32) -> ResourceTransferSpec {
    ResourceTransferSpec {
        id: id.into(),
        source: PropertyKey::new("core", source),
        source_role: SubFieldRole::Named("amount".into()),
        target: PropertyKey::new("core", target),
        target_role: SubFieldRole::Named("amount".into()),
        amount,
        order_band: 0,
    }
}

pub fn identity_emission(id: &str, source: &str) -> ResourceEmissionSpec {
    ResourceEmissionSpec {
        id: id.into(),
        source: PropertyKey::new("core", source),
        source_role: SubFieldRole::Named("amount".into()),
        formula: EmissionFormulaSpec::IdentityFloor,
    }
}

pub fn base_game_mode() -> GameModeSpec {
    GameModeSpec {
        id: "resource_economy_test".into(),
        display_name: "resource economy test".into(),
        description: String::new(),
        spec_version: SpecVersion::default(),
        metadata: DisplayMeta::default(),
        ..Default::default()
    }
}

pub fn transfer_game_mode() -> GameModeSpec {
    let mut mode = base_game_mode();
    mode.properties = vec![
        amount_property_spec("core", "food"),
        amount_property_spec("core", "store"),
    ];
    mode.resource_economy = Some(ResourceEconomySpec {
        transfers: vec![amount_transfer("t1", "food", "store", 1.0)],
        ..Default::default()
    });
    mode
}

pub fn emission_game_mode() -> GameModeSpec {
    let mut mode = base_game_mode();
    mode.properties = vec![amount_property_spec("core", "food")];
    mode.resource_economy = Some(ResourceEconomySpec {
        emissions: vec![identity_emission("e1", "food")],
        ..Default::default()
    });
    mode
}

pub fn register_amount(reg: &mut DimensionRegistry, ns: &str, name: &str) -> simthing_core::SimPropertyId {
    reg.register(SimProperty {
        namespace: ns.into(),
        name: name.into(),
        layout: PropertyLayout {
            sub_fields: vec![SubFieldSpec {
                role: SubFieldRole::Named("amount".into()),
                width: 1,
                clamp: ClampBehavior::Unbounded,
                velocity_max: None,
                default: 0.0,
                display_name: "amount".into(),
                display_range: None,
                governed_by: None,
                reduction_override: None,
                soft_aggregate_guard: None,
                accumulator_spec: None,
            }],
        },
        decay: None,
        intensity_behavior: None,
        fission_templates: vec![],
        fusion_templates: vec![],
        on_expire: None,
        description: String::new(),
        intensity_labels: vec![],
    })
}

/// World root holds `store`; cohort child holds `food` for live slot resolution tests.
pub fn live_slot_scenario() -> Scenario {
    let mut reg = DimensionRegistry::new();
    let food = register_amount(&mut reg, "core", "food");
    let store = register_amount(&mut reg, "core", "store");

    let food_layout = reg.property(food).layout.clone();
    let store_layout = reg.property(store).layout.clone();

    let mut world = SimThing::new(SimThingKind::World, 0);
    world.add_property(store, PropertyValue::from_layout(&store_layout));

    let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
    cohort.add_property(food, PropertyValue::from_layout(&food_layout));
    world.add_child(cohort);

    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&world);

    Scenario {
        name: "live_slot_economy".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 8,
        registry: reg,
        root: world,
        shadow_seeds: vec![],
        tick_patches: vec![],
        install_targets: Default::default(),
    }
}

pub fn live_slot_game_mode() -> GameModeSpec {
    let mut mode = base_game_mode();
    mode.resource_economy = Some(ResourceEconomySpec {
        transfers: vec![amount_transfer("t1", "food", "store", 1.0)],
        ..Default::default()
    });
    mode
}

pub fn cohort_food_slot(scenario: &Scenario) -> u32 {
    let cohort_id = scenario.root.children[0].id;
    scenario
        .root
        .children
        .iter()
        .find(|c| c.id == cohort_id)
        .and_then(|_| {
            let mut alloc = SlotAllocator::new();
            alloc.populate_from_tree(&scenario.root);
            alloc.slot_of(cohort_id)
        })
        .expect("cohort slot")
}
