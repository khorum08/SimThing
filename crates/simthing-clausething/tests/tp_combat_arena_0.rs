//! TP-COMBAT-ARENA-0: HP/Damage RF combat arena for two-fleet contact.

use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;

use simthing_clausething::{hydrate_scenario, parse_raw_document, HydratedScenarioPack};
use simthing_core::{
    ColumnIndex, DimensionRegistry, DiscreteTransferRegistration, PropertyValue, SimProperty,
    SimThing, SimThingId, SimThingKind, SlotIndex, SubFieldRole, TransformOp,
};
use simthing_driver::{
    materialize_resource_economy_registry_for_session, run_transfer_recipe_cpu_oracle,
    Scenario, SimSession,
};
use simthing_spec::compile_resource_economy;
use simthing_feeder::{BoundaryRequest, ScriptedEventTriggerEvent};
use simthing_gpu::{
    discrete_transfer_registrations_to_transfer, project_tree_to_values, set_debug_readback_allowed,
    AccumulatorPipelineSessions, GpuContext, Pipelines, SlotAllocator, WorldGpuState,
};
use simthing_sim::{apply_structural_mutations, SimRuntimeTree};
use simthing_spec::compile::compile_overlay;
use simthing_spec::{
    compile_property, InstallTargetSpec, ScriptedEventBoundaryContext, ScriptedEventBoundaryHandler,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn fixture_json_path() -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-mapeditor/tests/fixtures/tp_base_disc_1500.simthing-scenario.json")
        .to_string_lossy()
        .replace('\\', "/")
}

fn clause_text(owner_bonus_block: &str) -> String {
    include_str!("fixtures/tp_combat_arena_0.clause")
        .replace("{{FIXTURE_JSON}}", &fixture_json_path())
        .replace("{{OWNER_BONUS_BLOCK}}", owner_bonus_block)
}

fn hydrate_pack(owner_bonus_block: &str) -> HydratedScenarioPack {
    let document = parse_raw_document(clause_text(owner_bonus_block).as_bytes()).expect("parse clause");
    hydrate_scenario(&document).expect("hydrate combat arena clause")
}

fn scenario_from_pack(pack: &HydratedScenarioPack) -> Scenario {
    let authority = pack
        .authority_root
        .clone()
        .expect("combat pack carries authority root");
    let mut root = SimThing::new(SimThingKind::World, 0);
    for enrollment in &combat_payload(pack).enrollments {
        let source = find_simthing_by_id(&authority, enrollment.simthing_id).expect("combat ship");
        root.add_child(clone_ship_shell(source));
    }
    let mut registry = DimensionRegistry::new();
    let _ = registry.register(SimProperty::simple("_session", "seed", 0));
    Scenario {
        name: "tp_combat_arena_0".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 32,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: pack.install_targets.clone().into_iter().collect(),
    }
}

fn find_simthing_by_id(root: &SimThing, id: SimThingId) -> Option<&SimThing> {
    if root.id == id {
        return Some(root);
    }
    for child in &root.children {
        if let Some(found) = find_simthing_by_id(child, id) {
            return Some(found);
        }
    }
    None
}

fn clone_ship_shell(source: &SimThing) -> SimThing {
    let mut ship = source.clone();
    ship.properties.clear();
    ship.children.clear();
    ship
}

fn seed_combat_ship_properties(
    root: &mut SimThing,
    pack: &HydratedScenarioPack,
    registry: &DimensionRegistry,
) {
    for enrollment in &combat_payload(pack).enrollments {
        let ship = find_simthing_by_id_mut(root, enrollment.simthing_id).expect("combat ship");
        for (name, amount) in [
            (enrollment.hull_property.as_str(), 0.0_f32),
            (enrollment.weapon_property.as_str(), enrollment.weapon_damage),
        ] {
            let property_id = registry.id_of("tp", name).expect("combat property");
            let layout = registry.property(property_id).layout.clone();
            let mut value = registry.property(property_id).default_value();
            value.set_role(&SubFieldRole::Amount, &layout, amount);
            ship.add_property(property_id, value);
        }
    }
}

fn find_simthing_by_id_mut(root: &mut SimThing, id: SimThingId) -> Option<&mut SimThing> {
    if root.id == id {
        return Some(root);
    }
    for child in &mut root.children {
        if let Some(found) = find_simthing_by_id_mut(child, id) {
            return Some(found);
        }
    }
    None
}

fn open_session(pack: &HydratedScenarioPack) -> SimSession {
    let scenario = scenario_from_pack(pack);
    let mut session = SimSession::open_from_spec(scenario, &pack.game_mode).expect("open combat session");
    let mut admitted = session.scenario.root.clone();
    seed_combat_ship_properties(&mut admitted, pack, &session.proto.registry);
    session.scenario.root = admitted.clone();
    session.proto.allocator.populate_from_tree(&admitted);
    session.proto.root = SimRuntimeTree::admit(admitted);
    if let Some(resource_economy) = pack.game_mode.resource_economy.as_ref() {
        let eml_registry = simthing_core::EmlExpressionRegistry::new();
        let compiled = compile_resource_economy(
            resource_economy,
            &session.proto.registry,
            &eml_registry,
        )
        .expect("compile combat resource economy");
        let mut rematerialized = materialize_resource_economy_registry_for_session(
            &compiled,
            &session.proto.registry,
            &eml_registry,
            &session.scenario.root,
            &session.proto.allocator,
        )
        .expect("materialize combat transfer slots on ships");
        rematerialized.generation = session
            .spec_state
            .resource_economy_registry
            .as_ref()
            .map(|registry| registry.generation.saturating_add(1))
            .unwrap_or(1);
        session.spec_state.resource_economy_registry = Some(rematerialized);
    }
    session
        .sync_resource_economy_if_enabled()
        .expect("re-sync combat transfer registry after ship seed");
    let n_dims = session.state.n_dims as usize;
    let mut flat = session.state.read_values();
    let projected_len = session.proto.allocator.capacity() as usize * n_dims;
    project_tree_to_values(
        &session.scenario.root,
        &session.proto.registry,
        &session.proto.allocator,
        n_dims,
        &mut flat[..projected_len],
    );
    session.state.install_resolved_values_at_boundary(&flat);
    session
}

fn require_gpu() -> (GpuContext, std::sync::MutexGuard<'static, ()>) {
    let guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    set_debug_readback_allowed(true);
    let ctx = GpuContext::new_blocking().expect("TP-COMBAT-ARENA-0 requires a real GPU adapter");
    (ctx, guard)
}

fn cell_index(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn property_amount_col(registry: &DimensionRegistry, ns: &str, name: &str) -> u32 {
    let property_id = registry.id_of(ns, name).expect("property registered");
    registry
        .column_range(property_id)
        .col_for_role(&SubFieldRole::Amount, &registry.property(property_id).layout)
        .expect("amount column")
        .raw_u32()
}

fn slot_for_ship(session: &SimSession, ship_id: SimThingId) -> SlotIndex {
    session
        .proto
        .allocator
        .slot_of(ship_id)
        .expect("ship slot allocated")
}

fn combat_payload(pack: &HydratedScenarioPack) -> &simthing_clausething::HydratedCombatArenaPayload {
    pack.combat_arena_payload
        .as_ref()
        .expect("combat arena payload hydrated")
}

fn terran_enrollment(
    pack: &HydratedScenarioPack,
) -> &simthing_clausething::HydratedCombatShipEnrollment {
    combat_payload(pack)
        .enrollments
        .iter()
        .find(|ship| ship.owner == "terran")
        .expect("terran combat ship")
}

fn pirate_enrollment(
    pack: &HydratedScenarioPack,
) -> &simthing_clausething::HydratedCombatShipEnrollment {
    combat_payload(pack)
        .enrollments
        .iter()
        .find(|ship| ship.owner == "pirate")
        .expect("pirate combat ship")
}

fn effective_weapon_damage(
    registry: &DimensionRegistry,
    pack: &HydratedScenarioPack,
    enrollment: &simthing_clausething::HydratedCombatShipEnrollment,
) -> f32 {
    let property_id = registry
        .id_of("tp", &enrollment.weapon_property)
        .expect("weapon property");
    let layout = &registry.property(property_id).layout;
    let mut value = PropertyValue::from_layout(layout);
    value.set_role(&SubFieldRole::Amount, layout, enrollment.weapon_damage);
    let overlay = pack.game_mode.overlays.iter().find(|overlay| {
        matches!(
            &overlay.install,
            InstallTargetSpec::ScenarioListed { target_id }
                if target_id == &format!("combat_ship_{}", enrollment.id)
        )
    });
    if let Some(overlay_spec) = overlay {
        let (compiled, diag) = compile_overlay(overlay_spec, registry).expect("compile overlay");
        assert!(diag.diagnostics.is_empty(), "overlay admission must be clean");
        compiled
            .transform
            .apply_to_data(value.raw_lanes_mut(), layout);
    }
    value.get_role(&SubFieldRole::Amount, layout)
}

#[test]
fn gpu_two_fleet_contact_matches_transfer_oracle() {
    let (_ctx, _guard) = require_gpu();
    let pack = hydrate_pack("");
    let payload = combat_payload(&pack);
    assert_eq!(payload.enrollments.len(), 2, "one ship per hostile side");
    assert_eq!(payload.transfers.len(), 2, "bidirectional hostile transfers");

    let mut session = open_session(&pack);
    assert!(
        session.proto.flags.use_accumulator_transfer,
        "combat arena must opt into accumulator transfer"
    );
    session
        .sync_resource_economy_if_enabled()
        .expect("transfer economy sync");
    assert!(
        session.state.accumulator_transfer_active,
        "transfer accumulator must be armed after sync"
    );

    let transfers = session
        .spec_state
        .resource_economy_registry
        .as_ref()
        .expect("materialized transfer registry")
        .registrations
        .transfers
        .clone();
    let n_dims = session.state.n_dims;
    let registry = session.proto.registry.clone();

    let terran = terran_enrollment(&pack);
    let pirate = pirate_enrollment(&pack);
    let terran_slot = slot_for_ship(&session, terran.simthing_id).raw();
    let pirate_slot = slot_for_ship(&session, pirate.simthing_id).raw();
    let terran_weapon_col = property_amount_col(&registry, "tp", &terran.weapon_property);
    let pirate_weapon_col = property_amount_col(&registry, "tp", &pirate.weapon_property);
    let terran_hull_col = property_amount_col(&registry, "tp", &terran.hull_property);
    let pirate_hull_col = property_amount_col(&registry, "tp", &pirate.hull_property);

    let mut flat = session.state.read_values();
    assert_eq!(
        flat[cell_index(terran_slot, terran_weapon_col, n_dims)].to_bits(),
        terran.weapon_damage.to_bits(),
        "terran weapon must be seeded on live ship slot"
    );
    assert_eq!(
        flat[cell_index(pirate_slot, pirate_weapon_col, n_dims)].to_bits(),
        pirate.weapon_damage.to_bits(),
        "pirate weapon must be seeded on live ship slot"
    );

    let watched = [
        (terran_slot, terran_hull_col),
        (pirate_slot, pirate_hull_col),
    ];
    let gpu_regs = discrete_transfer_registrations_to_transfer(&transfers);
    session
        .state
        .sync_transfer_accumulator(&gpu_regs)
        .expect("upload combat transfer plan");
    assert!(
        session.state.accumulator_transfer_bands > 0,
        "expected transfer bands, got {}",
        session.state.accumulator_transfer_bands
    );

    let mut cpu_flat = flat.clone();
    run_transfer_recipe_cpu_oracle(
        &mut cpu_flat,
        n_dims,
        &transfers,
        &[],
    )
    .expect("cpu transfer oracle");

    session.state.install_resolved_values_at_boundary(&flat);
    let mut isolated = WorldGpuState::new(
        GpuContext::new_blocking().expect("gpu for isolated transfer parity"),
        &registry,
        session.state.n_slots,
    );
    isolated
        .sync_transfer_accumulator(&gpu_regs)
        .expect("isolated transfer plan");
    isolated.install_resolved_values_at_boundary(&flat);
    let pipelines = Pipelines::new(&isolated.ctx);
    let mut transfer_session = isolated
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .take_transfer_session();
    pipelines.run_tick_pipeline_with_accumulators(
        &mut isolated,
        1.0,
        AccumulatorPipelineSessions {
            intent: None,
            threshold: None,
            overlay_add: None,
            reduction_soft: None,
            velocity: None,
            intensity_eml: None,
            transfer: transfer_session.as_mut(),
            emission: None,
            encode_world_summary: false,
        },
    );
    isolated
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .restore_transfer_session(transfer_session);
    let gpu_flat = isolated.read_values();
    for &(slot, col) in &watched {
        let cpu = cpu_flat[cell_index(slot, col, n_dims)];
        let gpu = gpu_flat[cell_index(slot, col, n_dims)];
        assert_eq!(
            cpu.to_bits(),
            gpu.to_bits(),
            "damage column slot {slot} col {col} cpu={cpu} gpu={gpu} transfers={:?}",
            transfers
        );
    }
    assert_eq!(
        cpu_flat[cell_index(pirate_slot, pirate_hull_col, n_dims)].to_bits(),
        terran.weapon_damage.to_bits()
    );
    assert_eq!(
        cpu_flat[cell_index(terran_slot, terran_hull_col, n_dims)].to_bits(),
        pirate.weapon_damage.to_bits()
    );

}

#[test]
fn zero_hp_threshold_requests_boundary_removal() {
    let pack = hydrate_pack("");
    let pirate = pirate_enrollment(&pack);
    let capacity = combat_payload(&pack).hull_capacity;

    let session = open_session(&pack);
    let spec_state = &session.spec_state;
    let registry = &session.proto.registry;
    let mut root = session.scenario.root.clone();
    let mut allocator = session.proto.allocator.clone();

    let pirate_slot = allocator
        .slot_of(pirate.simthing_id)
        .expect("pirate slot")
        .raw();
    let hull_col = property_amount_col(&registry, "tp", &pirate.hull_property);
    let n_dims = registry.total_columns as usize;
    let mut shadow = vec![0.0_f32; allocator.capacity() as usize * n_dims];
    let hull_idx = pirate_slot as usize * n_dims + hull_col as usize;
    shadow[hull_idx] = capacity;

    let event_id = format!("{}_zero_hull_removal", pirate.id);
    let definition = spec_state
        .scripted_event_definitions
        .values()
        .find(|def| def.id.0 == event_id)
        .expect("zero-hull removal event installed")
        .clone();

    let mut slot_to_thing = HashMap::new();
    for slot in 0..allocator.capacity() as u32 {
        if let Some(id) = allocator.owner_of(SlotIndex::new(slot)) {
            slot_to_thing.insert(slot, id);
        }
    }

    let mut requests = Vec::new();
    let mut diagnostics = Vec::new();
    let mut cooldowns = HashMap::new();
    let handler = ScriptedEventBoundaryHandler {
        registry: &registry,
        definitions: std::slice::from_ref(&definition),
    };
    let mut ctx = ScriptedEventBoundaryContext {
        n_dims,
        shadow: &shadow,
        current_slot: pirate_slot,
        slot_to_thing: &slot_to_thing,
        cooldowns: &mut cooldowns,
        requests: &mut requests,
        diagnostics: &mut diagnostics,
    };
    handler.handle_tick(
        &[ScriptedEventTriggerEvent {
            event_id: event_id.clone(),
        }],
        &mut ctx,
    );
    assert!(diagnostics.is_empty(), "unexpected diagnostics: {diagnostics:?}");
    assert_eq!(requests.len(), 1, "one boundary removal request");
    assert!(
        matches!(requests[0], BoundaryRequest::Remove { target } if target == pirate.simthing_id),
        "zero-HP threshold must request removal of the current ship"
    );

    let mut runtime_root = SimRuntimeTree::admit(root);
    let size_before = runtime_root.subtree_size();
    let mut shadow_live = shadow.clone();
    let mut registry_mut = registry.clone();
    let outcome = apply_structural_mutations(
        requests,
        &mut runtime_root,
        &mut allocator,
        &mut registry_mut,
        &mut shadow_live,
        n_dims,
        None,
    );
    assert_eq!(outcome.removes, 1);
    assert!(outcome.tombstoned.contains(&pirate.simthing_id));
    assert!(runtime_root.subtree_size() < size_before);
    assert!(
        allocator.slot_of(pirate.simthing_id).is_none(),
        "removed ship slot must be tombstoned/recycled at boundary"
    );
}

#[test]
fn owner_weapon_damage_mult_changes_damage_via_overlay_only() {
    let baseline = hydrate_pack("");
    let bonus = hydrate_pack(
        r#"
        owner_bonus_owner = "terran"
        modifier = {
            ship_weapon_damage_mult = 0.5
        }
        "#,
    );

    let bonus_payload = combat_payload(&bonus);
    assert_eq!(bonus_payload.owner_bonus_mult, Some(0.5), "modifier lowers to owner_bonus_mult");
    fn has_combat_weapon_overlay(pack: &HydratedScenarioPack) -> bool {
        pack.game_mode.overlays.iter().any(|overlay| {
            overlay.targets_property.contains("combat_") && overlay.targets_property.contains("weapon")
        })
    }
    assert!(!has_combat_weapon_overlay(&baseline));
    assert!(has_combat_weapon_overlay(&bonus));
    let combat_overlay = bonus
        .game_mode
        .overlays
        .iter()
        .find(|overlay| {
            overlay.targets_property.contains("combat_") && overlay.targets_property.contains("weapon")
        })
        .expect("combat weapon overlay");
    let overlay_mult = combat_overlay
        .sub_field_deltas
        .iter()
        .find(|(role, _)| *role == SubFieldRole::Amount)
        .map(|(_, op)| op)
        .expect("weapon overlay delta");
    assert!(
        matches!(overlay_mult, TransformOp::Multiply(v) if v.to_bits() == 1.5_f32.to_bits()),
        "overlay must encode ship_weapon_damage_mult through TransformOp::Multiply"
    );

    let mut registry = DimensionRegistry::new();
    for prop in &bonus.game_mode.properties {
        compile_property(prop, &mut registry).expect("compile property");
    }

    let terran = terran_enrollment(&bonus);
    let pirate = pirate_enrollment(&bonus);
    let base_terran_weapon =
        effective_weapon_damage(&registry, &baseline, terran_enrollment(&baseline));
    let bonus_terran_weapon = effective_weapon_damage(&registry, &bonus, terran);
    let base_pirate_weapon =
        effective_weapon_damage(&registry, &baseline, pirate_enrollment(&baseline));
    let bonus_pirate_weapon = effective_weapon_damage(&registry, &bonus, pirate);

    assert_eq!(base_terran_weapon.to_bits(), 25.0_f32.to_bits());
    assert_eq!(bonus_terran_weapon.to_bits(), 37.5_f32.to_bits());
    assert_eq!(base_pirate_weapon.to_bits(), bonus_pirate_weapon.to_bits());
    assert_eq!(base_pirate_weapon.to_bits(), 30.0_f32.to_bits());

    let mut base_flat = vec![0.0_f32; registry.total_columns as usize * 4];
    let mut bonus_flat = base_flat.clone();
    let terran_weapon_col = property_amount_col(&registry, "tp", &terran.weapon_property);
    let pirate_hull_col = property_amount_col(&registry, "tp", &pirate.hull_property);
    let n_dims = registry.total_columns as u32;
    base_flat[cell_index(1, terran_weapon_col, n_dims)] = base_terran_weapon;
    bonus_flat[cell_index(1, terran_weapon_col, n_dims)] = bonus_terran_weapon;

    let transfer = bonus_payload
        .transfers
        .iter()
        .find(|t| t.id.contains("terran"))
        .expect("terran transfer");
    let registrations = [DiscreteTransferRegistration {
        source_slot: SlotIndex::new(1),
        source_col: ColumnIndex::new(terran_weapon_col as usize),
        target_slot: SlotIndex::new(2),
        target_col: ColumnIndex::new(pirate_hull_col as usize),
        amount: transfer.amount,
        order_band: transfer.order_band,
    }];

    run_transfer_recipe_cpu_oracle(&mut base_flat, n_dims, &registrations, &[]).expect("baseline oracle");
    run_transfer_recipe_cpu_oracle(&mut bonus_flat, n_dims, &registrations, &[]).expect("bonus oracle");

    let base_hull = base_flat[cell_index(2, pirate_hull_col, n_dims)];
    let bonus_hull = bonus_flat[cell_index(2, pirate_hull_col, n_dims)];
    assert_eq!(base_hull.to_bits(), 25.0_f32.to_bits());
    assert_eq!(bonus_hull.to_bits(), 37.5_f32.to_bits());
    assert!(
        (bonus_hull - base_hull).to_bits() == (bonus_terran_weapon - base_terran_weapon).to_bits(),
        "resolved damage delta must equal overlay-adjusted weapon delta only"
    );
}