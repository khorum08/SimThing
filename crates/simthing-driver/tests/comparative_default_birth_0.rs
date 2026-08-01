//! COMPARATIVE-DEFAULT-BIRTH-0 (5.8b) — STOP after orchestrator remand `5153911298`.
//!
//! Withdraws the Scenario.field_plan_admission side-door and role-named
//! FieldPlanAdmissionReport taxonomy. Proves ordinary install still does not
//! invent comparative birth without an already-admitted field-plan producer.

use simthing_core::{
    DimensionRegistry, PropertyAdmissionDisposition, SimProperty, SimThing, SimThingKind,
};
use simthing_driver::{compile_and_install, Scenario};
use simthing_gpu::SlotAllocator;
use simthing_spec::{GameModeSpec, SpecVersion};
use std::collections::HashMap;

fn empty_game_mode() -> GameModeSpec {
    GameModeSpec {
        id: "cdb0-stop".into(),
        display_name: "cdb0-stop".into(),
        description: String::new(),
        spec_version: SpecVersion::default(),
        metadata: Default::default(),
        domain_packs: Vec::new(),
        properties: Vec::new(),
        overlays: Vec::new(),
        order_weight_classes: Vec::new(),
        capability_trees: Vec::new(),
        events: Vec::new(),
        resource_flow: None,
        resource_economy: None,
        resource_flow_execution_profile: Default::default(),
        region_fields: Vec::new(),
        mapping_execution_profile: Default::default(),
    }
}

fn ordinary_scenario(n_slots: u32, registry: DimensionRegistry) -> Scenario {
    Scenario {
        name: "cdb0-stop".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 0.0,
        n_slots,
        registry,
        root: SimThing::new(SimThingKind::World, 0),
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::new(),
    }
}

/// S3 fence: ordinary install does not invent comparative birth.
/// Remand `5153911298` defect 1 — Scenario.field_plan_admission is gone;
/// no re-homed input door; no parallel install enrollment.
#[test]
fn ordinary_install_does_not_invent_comparative_birth() {
    let mut registry = DimensionRegistry::new();
    let _ = registry.register(SimProperty::simple("_seed", "pad", 0));
    for (ns, name) in [
        ("feed", "e0"),
        ("feed", "e1"),
        ("feed", "d"),
        ("feed", "u"),
        ("feed", "c"),
    ] {
        let mut p = SimProperty::simple(ns, name, 1);
        p.admission_disposition = PropertyAdmissionDisposition::Anchored;
        registry.register(p);
    }

    let scenario = ordinary_scenario(4, registry.clone());
    let game = empty_game_mode();
    let mut root = scenario.root.clone();
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&root);
    let mut reg = registry;
    let state = compile_and_install(&game, &scenario, &mut reg, &mut root, &mut alloc)
        .expect("ordinary install");

    assert!(
        state.comparative_projection.is_none(),
        "STOP: ordinary install must not invent comparative birth without an \
         already-admitted field-plan producer inside compile_and_install"
    );
}

/// Structural witness: Scenario has no field_plan_admission field (defect 1).
/// Compile-time: the Scenario struct initializer above omits any such field.
/// Runtime: inventory/results document the withdrawn side-door.
#[test]
fn scenario_has_no_field_plan_admission_side_door() {
    // If Scenario.field_plan_admission is reintroduced, the STOP results doc
    // and this harness must be updated only under a DA producer ruling.
    let scenario = ordinary_scenario(1, DimensionRegistry::new());
    let _ = scenario.name;
    // Field absence is compile-checked by the initializer in ordinary_scenario.
    assert_eq!(scenario.n_slots, 1);
}
