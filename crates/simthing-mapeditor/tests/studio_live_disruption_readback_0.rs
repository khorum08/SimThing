//! Live STEAD disruption readback — Remand 3 biting proofs (orch `5026225403`).

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

use simthing_clausething::{
    hydrate_scenario_with_source_base, parse_raw_document, rebind_pack_to_structural_rebind_ready,
    HydratedScenarioPack,
};
use simthing_core::{
    ClampBehavior, DimensionRegistry, SimThingId, SubFieldRole, SubFieldSpec, TransformOp,
};
use simthing_driver::{
    observe_hosted_property_cell, system_id_by_host_raw_from_structural_authority,
    GpuValuesSnapshot, HostedPropertyLocus, HostedPropertyObservationError,
    LiveDisruptionAuthorityReadback,
};
use simthing_gpu::SlotAllocator;
use simthing_mapeditor::{
    attach_disruption_host_structural_placements, authored_live_profile_from_pack,
    disruption_host_entities_from_pack, disruption_select_screen_from_raw,
    selected_disruption_select_screen, StudioLiveSessionBridge, StudioLiveSessionBridgeError,
    StudioLiveSessionPath, StudioLiveSessionPathPreference, StudioSession,
};
use simthing_spec::{
    compile_property, DisruptionAuthorityReadback, EmissionFormulaSpec, PropertyKey, PropertySpec,
    RecipeInputSpec, ResourceRecipeSpec, SimThingStructuralGridPlacement,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

fn hydrate_canonical() -> HydratedScenarioPack {
    let clause_path = repo_root().join("scenarios/terran_pirate_galaxy.clause");
    let text = std::fs::read_to_string(&clause_path).expect("read clause");
    let document = parse_raw_document(text.as_bytes()).expect("parse");
    hydrate_scenario_with_source_base(&document, Some(clause_path.parent().unwrap()))
        .expect("hydrate")
}

fn studio_from_pack(pack: &HydratedScenarioPack) -> StudioSession {
    let (scenario, _) =
        rebind_pack_to_structural_rebind_ready(pack).expect("StructuralRebindReady");
    let mut studio = StudioSession::from_loaded_scenario(
        scenario,
        repo_root().join("scenarios/terran_pirate_galaxy.clause"),
        None,
    )
    .expect("studio session");
    attach_disruption_host_structural_placements(
        &mut studio.scenario_authority,
        disruption_host_entities_from_pack(pack),
    );
    studio.with_authored_live_profile(authored_live_profile_from_pack(pack))
}

fn open_field_bridge(studio: &StudioSession) -> StudioLiveSessionBridge {
    let mut bridge = StudioLiveSessionBridge::new();
    bridge.set_path_preference(StudioLiveSessionPathPreference::FieldBearing);
    match bridge.open_from_loaded_studio_session(studio) {
        Ok(()) => {}
        Err(StudioLiveSessionBridgeError::Unsupported(msg)) => {
            panic!("GPU/adapter Unsupported is FAIL: {msg}");
        }
        Err(e) => panic!("field-bearing open failed: {e}"),
    }
    assert_eq!(bridge.session_path(), StudioLiveSessionPath::FieldBearing);
    bridge
}

fn host_system_id(studio: &StudioSession, host: &str) -> u32 {
    studio
        .scenario_authority
        .structural_grid
        .placements
        .iter()
        .rev()
        .find(|placement| placement.location_id == host || placement.target_id == host)
        .map(|placement| placement.system_id)
        .expect("host structural placement")
}

fn unrelated_system_id(studio: &StudioSession, host_system: u32) -> u32 {
    studio
        .scenario_authority
        .structural_grid
        .placements
        .iter()
        .map(|placement| placement.system_id)
        .find(|&system_id| system_id != host_system)
        .expect("unrelated star system")
}

/// Zero Crisis seed + recipe that credits presence under ordinary step_once.
fn pack_zero_seed_presence_with_recipe() -> HydratedScenarioPack {
    let mut pack = hydrate_canonical();
    if let Some(economy) = pack.game_mode.resource_economy.as_mut() {
        for emission in &mut economy.emissions {
            // Exact presence identity only — do not match `*_disruption_weight_*`.
            if emission.source.name.ends_with("_presence")
                || emission.source.name == "pirate_outpost_disruption_presence"
            {
                emission.formula = EmissionFormulaSpec::Constant(0.0);
            }
        }
        economy.recipes.push(ResourceRecipeSpec {
            id: "disruption_from_pirate_weight".into(),
            inputs: vec![RecipeInputSpec {
                // Seeded at open via silo current emission (stockpile fills later).
                property: PropertyKey::new("tp_economy", "pirate_disruption_weight_current"),
                role: SubFieldRole::Amount,
                unit_cost: 1.0,
                host_entity: Some("pirate".into()),
                host_span_token: None,
            }],
            target: PropertyKey::new("tp_economy", "pirate_outpost_disruption_presence"),
            target_role: SubFieldRole::Amount,
            target_host_entity: Some("pirate_outpost".into()),
            target_host_span_token: None,
            output_coefficient: 5.0,
            order_band: 0,
            throttle_hint_max_per_tick: 1,
        });
    } else {
        panic!("canonical pack must expose resource_economy for recipe injection");
    }
    for overlay in &mut pack.game_mode.overlays {
        if overlay.targets_property.contains("disruption_presence") {
            for (_, op) in &mut overlay.sub_field_deltas {
                match op {
                    TransformOp::Add(_) => *op = TransformOp::Add(0.0),
                    TransformOp::Multiply(_) => *op = TransformOp::Multiply(1.0),
                    TransformOp::Set(_) => *op = TransformOp::Set(0.0),
                }
            }
        }
    }
    pack
}

/// catches: live map stays Absent/0 while field-bearing disruption accretes under step_once.
#[test]
fn canonical_host_system_moves_zero_to_nonzero_unrelated_stays_zero() {
    let pack = pack_zero_seed_presence_with_recipe();
    assert!(
        pack.game_mode
            .resource_economy
            .as_ref()
            .is_some_and(|e| e.recipes.iter().any(|r| r.id == "disruption_from_pirate_weight")),
        "zero-seed fixture must retain injected recipe"
    );
    let studio = studio_from_pack(&pack);
    let pirate_sys = host_system_id(&studio, "pirate_outpost");
    let other_sys = unrelated_system_id(&studio, pirate_sys);
    assert_ne!(pirate_sys, other_sys);

    let mut bridge = open_field_bridge(&studio);
    let open_map = bridge.readout().disruption_readout;
    let open_pirate = open_map
        .by_system_id
        .get(&pirate_sys)
        .map(|r| r.max_disruption_accreted())
        .unwrap_or(0.0);
    assert_eq!(
        open_pirate, 0.0,
        "pre-tick typed disruption must be exact 0.0, got {open_pirate}"
    );
    let open_other = open_map
        .by_system_id
        .get(&other_sys)
        .map(|r| r.max_disruption_accreted())
        .unwrap_or(0.0);
    assert_eq!(open_other, 0.0);

    bridge.consume_scheduled_ticks(1).expect("step_once");
    let after = bridge.readout().disruption_readout;
    let after_pirate = after
        .by_system_id
        .get(&pirate_sys)
        .map(|r| r.max_disruption_accreted())
        .unwrap_or(0.0);
    assert!(
        after_pirate > 0.0,
        "ordinary step_once must move host 0 -> nonzero: after={after_pirate}"
    );
    let after_other = after
        .by_system_id
        .get(&other_sys)
        .map(|r| r.max_disruption_accreted())
        .unwrap_or(0.0);
    assert_eq!(
        after_other, 0.0,
        "unrelated system must stay 0.0, got {after_other}"
    );
}

/// catches: host-placement swap ignored / hard-coded system id.
#[test]
fn authored_host_placement_swap_moves_system_id_with_zero_code_change() {
    let pack = hydrate_canonical();
    let mut studio = studio_from_pack(&pack);
    let original = host_system_id(&studio, "pirate_outpost");
    let swapped_sys = unrelated_system_id(&studio, original);
    assert_ne!(original, swapped_sys);

    for placement in &mut studio.scenario_authority.structural_grid.placements {
        if placement.location_id == "pirate_outpost" || placement.target_id == "pirate_outpost"
        {
            placement.system_id = swapped_sys;
        }
    }

    let bridge = open_field_bridge(&studio);
    let map = bridge.readout().disruption_readout;
    let on_swapped = map
        .by_system_id
        .get(&swapped_sys)
        .map(|r| r.max_disruption_accreted())
        .unwrap_or(0.0);
    let on_original = map
        .by_system_id
        .get(&original)
        .map(|r| r.max_disruption_accreted())
        .unwrap_or(0.0);
    assert!(
        on_swapped > 0.0,
        "swapped host placement must read on the new system_id"
    );
    assert_eq!(
        on_original, 0.0,
        "original system must clear after placement swap"
    );
}

/// catches: two loci reduce by sum/first/global instead of exact max via production readback.
#[test]
fn two_loci_in_one_system_report_exact_max() {
    let mut registry = DimensionRegistry::new();
    let property = PropertySpec {
        id: "ns_p".into(),
        namespace: "ns".into(),
        name: "p".into(),
        display_name: "p".into(),
        description: "max proof".into(),
        sub_fields: vec![SubFieldSpec {
            role: SubFieldRole::Amount,
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
    };
    compile_property(&property, &mut registry).expect("compile");
    let n_dims = registry.total_columns;
    let host_a = SimThingId::from_session_raw(10);
    let host_b = SimThingId::from_session_raw(11);
    let mut allocator = SlotAllocator::new();
    let slot_a = allocator.alloc(host_a);
    let slot_b = allocator.alloc(host_b);
    let mut values = vec![0.0f32; allocator.capacity() * n_dims];
    let pid = registry.id_of("ns", "p").expect("pid");
    let col = registry
        .column_range(pid)
        .col_for_role(&SubFieldRole::Amount, &registry.property(pid).layout)
        .expect("amount col")
        .raw();
    values[usize::from(slot_a) * n_dims + col] = 3.0;
    values[usize::from(slot_b) * n_dims + col] = 8.0;
    let snapshot = GpuValuesSnapshot::from_values_for_test(values, n_dims);

    let loci = vec![
        HostedPropertyLocus {
            host_id: host_a,
            host_entity: Some("a".into()),
            property: PropertyKey::new("ns", "p"),
            role: SubFieldRole::Amount,
        },
        HostedPropertyLocus {
            host_id: host_b,
            host_entity: Some("b".into()),
            property: PropertyKey::new("ns", "p"),
            role: SubFieldRole::Amount,
        },
    ];
    let system_id_by_host_raw = BTreeMap::from([(10u32, 7u32), (11u32, 7u32)]);
    let readback = LiveDisruptionAuthorityReadback {
        snapshot: &snapshot,
        registry: &registry,
        allocator: &allocator,
        loci: &loci,
        system_id_by_host_raw: &system_id_by_host_raw,
    };
    let by_system = readback
        .max_disruption_accreted_by_system_id()
        .expect("readback")
        .expect("Some");
    assert_eq!(
        by_system.get(&7),
        Some(&8.0),
        "production LiveDisruptionAuthorityReadback must report max(3,8)=8"
    );
}

/// catches: map frozen at open while runtime disruption changes.
#[test]
fn live_map_refreshes_when_runtime_disruption_changes() {
    let pack = hydrate_canonical();
    let studio = studio_from_pack(&pack);
    let pirate_sys = host_system_id(&studio, "pirate_outpost");
    let mut bridge = open_field_bridge(&studio);
    let before = bridge
        .readout()
        .disruption_readout
        .by_system_id
        .get(&pirate_sys)
        .map(|r| r.max_disruption_accreted())
        .unwrap_or(0.0);
    assert!(before > 0.0, "open must observe authored Crisis seed");
    bridge.consume_scheduled_ticks(4).expect("ticks");
    let after = bridge
        .readout()
        .disruption_readout
        .by_system_id
        .get(&pirate_sys)
        .map(|r| r.max_disruption_accreted())
        .unwrap_or(0.0);
    assert!(
        (after - before).abs() > 1e-4,
        "live refresh must change exact map value: before={before} after={after}"
    );
}

/// catches: structural-shell path invents live nonzero disruption rows.
#[test]
fn structural_shell_absent_field_stays_typed_zero() {
    let pack = hydrate_canonical();
    let studio = studio_from_pack(&pack);
    let mut bridge = StudioLiveSessionBridge::new();
    bridge.set_path_preference(StudioLiveSessionPathPreference::StructuralShell);
    match bridge.open_from_loaded_studio_session(&studio) {
        Ok(()) => {}
        Err(StudioLiveSessionBridgeError::Unsupported(msg)) => {
            panic!("GPU/adapter Unsupported is FAIL: {msg}");
        }
        Err(e) => panic!("shell open failed: {e}"),
    }
    assert!(bridge
        .readout()
        .disruption_readout
        .by_system_id
        .values()
        .all(|r| r.max_disruption_accreted() == 0.0));
}

/// catches: 12.3 telemetry/piecewise diverges from live map row.
#[test]
fn selected_star_telemetry_matches_live_map_and_piecewise() {
    let pack = hydrate_canonical();
    let studio = studio_from_pack(&pack);
    let pirate_sys = host_system_id(&studio, "pirate_outpost");
    let bridge = open_field_bridge(&studio);
    let raw = bridge
        .readout()
        .disruption_readout
        .by_system_id
        .get(&pirate_sys)
        .map(|r| r.max_disruption_accreted())
        .unwrap_or(0.0);
    assert!(raw > 0.0);
    let screen = selected_disruption_select_screen(
        Some(pirate_sys),
        &bridge.readout().disruption_readout,
    );
    let expected = disruption_select_screen_from_raw(raw);
    assert_eq!(screen.raw_disruption, raw);
    assert_eq!(screen.blur_scale, expected.blur_scale);
    assert_eq!(screen.red_fraction, expected.red_fraction);
}

/// catches: nonempty typed loci with total/partial structural miss must fail loud.
#[test]
fn structural_mapping_total_and_partial_miss_fail_loud() {
    let loci = vec![HostedPropertyLocus {
        host_id: SimThingId::from_session_raw(99),
        host_entity: Some("missing".into()),
        property: PropertyKey::new("ns", "p"),
        role: SubFieldRole::Amount,
    }];
    let total_err = system_id_by_host_raw_from_structural_authority(
        &[],
        &HashMap::new(),
        &loci,
        &BTreeMap::new(),
    )
    .expect_err("all-miss must fail loud");
    assert!(
        total_err.to_string().contains("total structural mapping"),
        "unexpected total-miss diagnostic: {total_err}"
    );

    let placements = vec![SimThingStructuralGridPlacement {
        location_id: "a".into(),
        target_id: "a".into(),
        system_id: 1,
        row: 0,
        col: 0,
        simthing_id_raw: 10,
    }];
    let mixed = vec![
        HostedPropertyLocus {
            host_id: SimThingId::from_session_raw(10),
            host_entity: Some("a".into()),
            property: PropertyKey::new("ns", "p"),
            role: SubFieldRole::Amount,
        },
        HostedPropertyLocus {
            host_id: SimThingId::from_session_raw(11),
            host_entity: Some("missing".into()),
            property: PropertyKey::new("ns", "p"),
            role: SubFieldRole::Amount,
        },
    ];
    let partial_err = system_id_by_host_raw_from_structural_authority(
        &placements,
        &HashMap::new(),
        &mixed,
        &BTreeMap::from([("a".into(), 1u32)]),
    )
    .expect_err("partial must fail loud");
    assert!(
        partial_err.to_string().contains("partial structural mapping"),
        "unexpected partial-miss diagnostic: {partial_err}"
    );
}

/// catches: production field-bearing open with typed loci and no Spec join fails loud.
#[test]
fn field_bearing_unmapped_typed_loci_fail_loud_on_open() {
    let pack = hydrate_canonical();
    let (scenario, _) =
        rebind_pack_to_structural_rebind_ready(&pack).expect("StructuralRebindReady");
    let mut studio = StudioSession::from_loaded_scenario(
        scenario,
        repo_root().join("scenarios/terran_pirate_galaxy.clause"),
        None,
    )
    .expect("studio session");
    // Deliberately omit attach_disruption_host_structural_placements.
    studio = studio.with_authored_live_profile(authored_live_profile_from_pack(&pack));
    assert!(
        !studio
            .authored_live_profile
            .as_ref()
            .unwrap()
            .disruption_observation_loci
            .is_empty(),
        "typed disruption_presence loci must survive hydrate"
    );

    let mut bridge = StudioLiveSessionBridge::new();
    bridge.set_path_preference(StudioLiveSessionPathPreference::FieldBearing);
    let err = bridge
        .open_from_loaded_studio_session(&studio)
        .expect_err("unmapped typed loci must fail loud");
    let message = err.to_string();
    assert!(
        message.contains("structural mapping") || message.contains("DisruptionReadback"),
        "unexpected unmapped-open diagnostic: {message}"
    );
}

/// catches: forced unknown property / role / unallocated host fail loud on the observation door.
#[test]
fn observation_door_unknown_property_role_and_host_fail_loud() {
    let mut registry = DimensionRegistry::new();
    let property = PropertySpec {
        id: "ns_p".into(),
        namespace: "ns".into(),
        name: "p".into(),
        display_name: "p".into(),
        description: "door".into(),
        sub_fields: vec![SubFieldSpec {
            role: SubFieldRole::Amount,
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
    };
    compile_property(&property, &mut registry).expect("compile");
    let n_dims = registry.total_columns.max(1);
    let host = SimThingId::from_session_raw(3);
    let mut allocator = SlotAllocator::new();
    allocator.alloc(host);
    let snapshot = GpuValuesSnapshot::from_values_for_test(vec![0.0; n_dims * 4], n_dims);

    let unknown_property = observe_hosted_property_cell(
        &registry,
        &allocator,
        &snapshot,
        host,
        &PropertyKey::new("ns", "missing"),
        &SubFieldRole::Amount,
    )
    .expect_err("unknown property");
    assert!(matches!(
        unknown_property,
        HostedPropertyObservationError::UnknownProperty { .. }
    ));

    let unknown_role = observe_hosted_property_cell(
        &registry,
        &allocator,
        &snapshot,
        host,
        &PropertyKey::new("ns", "p"),
        &SubFieldRole::Named("nope".into()),
    )
    .expect_err("unknown role");
    assert!(matches!(
        unknown_role,
        HostedPropertyObservationError::UnknownRole { .. }
    ));

    let unallocated = observe_hosted_property_cell(
        &registry,
        &allocator,
        &snapshot,
        SimThingId::from_session_raw(99),
        &PropertyKey::new("ns", "p"),
        &SubFieldRole::Amount,
    )
    .expect_err("unallocated host");
    assert!(matches!(
        unallocated,
        HostedPropertyObservationError::HostHasNoSlot { .. }
    ));
}
