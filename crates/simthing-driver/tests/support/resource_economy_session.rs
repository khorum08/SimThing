//! Shared helpers for resource economy session integration tests.

#![allow(dead_code)]

use simthing_core::{
    ClampBehavior, DimensionRegistry, PropertyLayout, PropertyValue, SimProperty, SimThing,
    SimThingKind, SubFieldRole, SubFieldSpec,
};
use simthing_driver::Scenario;
use simthing_gpu::{GpuContext, SlotAllocator};
use simthing_spec::{
    metadata::DisplayMeta, version::SpecVersion, EmissionFormulaSpec, GameModeSpec, PropertyKey,
    PropertySpec, RecipeInputSpec, ResourceEconomySpec, ResourceEmissionSpec, ResourceRecipeSpec,
    ResourceTransferSpec,
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

pub fn recipe_game_mode() -> GameModeSpec {
    let mut mode = base_game_mode();
    mode.resource_economy = Some(ResourceEconomySpec {
        recipes: vec![ResourceRecipeSpec {
            id: "r1".into(),
            inputs: vec![
                RecipeInputSpec {
                    property: PropertyKey::new("core", "food"),
                    role: SubFieldRole::Named("amount".into()),
                    unit_cost: 1.0,
                },
                RecipeInputSpec {
                    property: PropertyKey::new("core", "ore"),
                    role: SubFieldRole::Named("amount".into()),
                    unit_cost: 2.0,
                },
            ],
            target: PropertyKey::new("core", "product"),
            target_role: SubFieldRole::Named("amount".into()),
            throttle_hint_max_per_tick: 99,
        }],
        ..Default::default()
    });
    mode
}

pub fn amount_col(reg: &DimensionRegistry, ns: &str, name: &str) -> u32 {
    let pid = reg.id_of(ns, name).expect("property registered");
    reg.column_range(pid)
        .col_for_role(
            &SubFieldRole::Named("amount".into()),
            &reg.property(pid).layout,
        )
        .expect("amount column") as u32
}

pub fn open_live_transfer_session() -> simthing_driver::SimSession {
    let scenario = live_slot_scenario();
    let mut session = simthing_driver::SimSession::open_from_spec(scenario, &live_slot_game_mode())
        .expect("open");
    session.proto.flags.use_accumulator_transfer = true;
    session.sync_resource_economy_if_enabled().expect("sync");
    session
}

pub fn open_live_emission_session() -> simthing_driver::SimSession {
    let mut scenario = live_slot_scenario();
    scenario.registry = scenario.registry.clone();
    let mut mode = base_game_mode();
    mode.resource_economy = Some(ResourceEconomySpec {
        emissions: vec![identity_emission("e1", "food")],
        ..Default::default()
    });
    let mut session = simthing_driver::SimSession::open_from_spec(scenario, &mode).expect("open");
    session.proto.flags.use_accumulator_eml = true;
    session.proto.flags.use_accumulator_emission = true;
    session.sync_resource_economy_if_enabled().expect("sync");
    session
}

pub fn open_rebellion_transfer_session() -> simthing_driver::SimSession {
    let ron = include_str!("../../../../scenarios/rebellion_demo.ron");
    let scenario = simthing_driver::Scenario::from_ron_str(ron).expect("scenario");
    let mut session =
        simthing_driver::SimSession::open_from_spec(scenario, &transfer_game_mode()).expect("open");
    session.proto.flags.use_accumulator_transfer = true;
    session.sync_resource_economy_if_enabled().expect("sync");
    session
}

/// Recipe fixture: food, ore, and product on world root (single slot for GPU conjunctive parity).
pub fn recipe_scenario() -> Scenario {
    let mut reg = DimensionRegistry::new();
    let food = register_amount(&mut reg, "core", "food");
    let ore = register_amount(&mut reg, "core", "ore");
    let product = register_amount(&mut reg, "core", "product");

    let food_layout = reg.property(food).layout.clone();
    let ore_layout = reg.property(ore).layout.clone();
    let product_layout = reg.property(product).layout.clone();

    let mut world = SimThing::new(SimThingKind::World, 0);
    world.add_property(food, PropertyValue::from_layout(&food_layout));
    world.add_property(ore, PropertyValue::from_layout(&ore_layout));
    world.add_property(product, PropertyValue::from_layout(&product_layout));

    Scenario {
        name: "recipe_economy".into(),
        ticks_per_day: 1,
        max_days: 4,
        dt: 1.0,
        n_slots: 4,
        registry: reg,
        root: world,
        shadow_seeds: vec![],
        tick_patches: vec![],
        install_targets: Default::default(),
    }
}

pub fn open_recipe_session() -> simthing_driver::SimSession {
    let scenario = recipe_scenario();
    let mut session =
        simthing_driver::SimSession::open_from_spec(scenario, &recipe_game_mode()).expect("open");
    session.proto.flags.use_accumulator_transfer = true;
    session.sync_resource_economy_if_enabled().expect("sync");
    session
}

pub fn register_amount(
    reg: &mut DimensionRegistry,
    ns: &str,
    name: &str,
) -> simthing_core::SimPropertyId {
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
