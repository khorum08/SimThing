//! Live STEAD disruption readback — Remand-4 / DA enrollment `5027107657` proofs.
//!
//! Production path only: Clause hydrate `system_target` → pack location_system_ids →
//! Studio field-bearing open. No synthetic Spec placement attach.

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

use simthing_clausething::{
    hydrate_scenario_with_source_base, parse_raw_document, rebind_pack_to_structural_rebind_ready,
    HydratedScenarioPack,
};
use simthing_core::{SimThingId, SubFieldRole, TransformOp};
use simthing_driver::{
    observe_hosted_property_cell, system_id_by_host_raw_from_structural_authority, GpuValuesSnapshot,
    HostedPropertyLocus, HostedPropertyObservationError,
};
use simthing_mapeditor::{
    authored_live_profile_from_pack, disruption_select_screen_from_raw,
    selected_disruption_select_screen, StudioLiveSessionBridge, StudioLiveSessionBridgeError,
    StudioLiveSessionPath, StudioLiveSessionPathPreference, StudioSession,
};
use simthing_spec::{
    EmissionFormulaSpec, PropertyKey, RecipeInputSpec, ResourceRecipeSpec,
    SimThingStructuralGridPlacement,
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

fn hydrate_source(source: &str) -> Result<HydratedScenarioPack, String> {
    let document = parse_raw_document(source.as_bytes()).map_err(|e| e.to_string())?;
    hydrate_scenario_with_source_base(&document, Some(repo_root().join("scenarios").as_path()))
        .map_err(|e| e.to_string())
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

fn host_system_id(pack: &HydratedScenarioPack, host: &str) -> u32 {
    *authored_live_profile_from_pack(pack)
        .location_system_ids
        .get(host)
        .unwrap_or_else(|| panic!("production system_target enrollment missing for `{host}`"))
}

fn unrelated_system_id(pack: &HydratedScenarioPack, host_system: u32) -> u32 {
    authored_live_profile_from_pack(pack)
        .location_system_ids
        .values()
        .copied()
        .find(|&system_id| system_id != host_system)
        .expect("unrelated enrolled system")
}

fn pack_zero_seed_presence_with_recipe() -> HydratedScenarioPack {
    let mut pack = hydrate_canonical();
    if let Some(economy) = pack.game_mode.resource_economy.as_mut() {
        for emission in &mut economy.emissions {
            if emission.source.name.ends_with("_presence")
                || emission.source.name == "pirate_outpost_disruption_presence"
            {
                emission.formula = EmissionFormulaSpec::Constant(0.0);
            }
        }
        economy.recipes.push(ResourceRecipeSpec {
            id: "disruption_from_pirate_weight".into(),
            inputs: vec![RecipeInputSpec {
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
        panic!("canonical pack must expose resource_economy");
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
    let pirate_sys = host_system_id(&pack, "pirate_outpost");
    let other_sys = unrelated_system_id(&pack, pirate_sys);
    assert_ne!(pirate_sys, other_sys);

    let studio = studio_from_pack(&pack);
    let mut bridge = open_field_bridge(&studio);
    let open_pirate = bridge
        .readout()
        .disruption_readout
        .by_system_id
        .get(&pirate_sys)
        .map(|r| r.max_disruption_accreted())
        .unwrap_or(0.0);
    assert_eq!(open_pirate, 0.0, "pre-tick must be exact 0.0");
    assert_eq!(
        bridge
            .readout()
            .disruption_readout
            .by_system_id
            .get(&other_sys)
            .map(|r| r.max_disruption_accreted())
            .unwrap_or(0.0),
        0.0
    );

    bridge.consume_scheduled_ticks(1).expect("step_once");
    let after_pirate = bridge
        .readout()
        .disruption_readout
        .by_system_id
        .get(&pirate_sys)
        .map(|r| r.max_disruption_accreted())
        .unwrap_or(0.0);
    assert!(
        after_pirate > 0.0,
        "ordinary step_once must move host 0 -> nonzero: after={after_pirate}"
    );
    assert_eq!(
        bridge
            .readout()
            .disruption_readout
            .by_system_id
            .get(&other_sys)
            .map(|r| r.max_disruption_accreted())
            .unwrap_or(0.0),
        0.0
    );
}

/// catches: hard-coded system id / enrollment ignored when only system_target changes.
#[test]
fn authored_system_target_swap_moves_system_id_with_zero_code_change() {
    let pack = hydrate_canonical();
    let original = host_system_id(&pack, "pirate_outpost");
    // Free lattice cell (not terran_shipyard / pirate_outpost): pirate_border sample.
    const SWAP_TARGET: &str = "row186_col121";

    let swapped_source = std::fs::read_to_string(repo_root().join("scenarios/terran_pirate_galaxy.clause"))
        .expect("clause")
        .replacen(
            "location = pirate_outpost {\n        display_name = \"Pirate Outpost\"\n        system_target = \"row158_col110\"\n    }",
            &format!(
                "location = pirate_outpost {{\n        display_name = \"Pirate Outpost\"\n        system_target = \"{SWAP_TARGET}\"\n    }}"
            ),
            1,
        );
    let swapped = hydrate_source(&swapped_source).expect("hydrate swap");
    let swapped_sys = host_system_id(&swapped, "pirate_outpost");
    assert_ne!(swapped_sys, original);

    let profile = authored_live_profile_from_pack(&swapped);
    assert_eq!(
        profile.location_system_ids.get("pirate_outpost"),
        Some(&swapped_sys),
        "outpost enrollment must follow the authored system_target"
    );
    assert_ne!(
        profile.location_system_ids.get("pirate_outpost"),
        Some(&original)
    );

    let studio = studio_from_pack(&swapped);
    let bridge = open_field_bridge(&studio);
    let map = bridge.readout().disruption_readout;
    assert!(
        map.by_system_id
            .get(&swapped_sys)
            .map(|r| r.max_disruption_accreted())
            .unwrap_or(0.0)
            > 0.0,
        "swapped system_target must read on the new enrolled system"
    );
    // Original may still be nonzero via fleet-home fan-out; outpost enrollment alone moved.
}

/// catches: fleet-payload fan-out accretes per pirate fleet home; fleet-free stays zero.
#[test]
fn pirate_fleet_home_systems_accrue_local_disruption_independently() {
    let pack = hydrate_canonical();
    let pirate_payload = pack
        .fleet_ship_payloads
        .iter()
        .find(|payload| payload.id == "pirate_fleets")
        .expect("pirate_fleets payload");
    let embedded = pack
        .embedded_static_galaxy_scenarios
        .first()
        .expect("embedded");
    let mut fleet_home_systems = std::collections::BTreeSet::new();
    for placement in &pirate_payload.placements {
        let local_target = placement
            .target_id
            .rsplit_once("::")
            .map(|(_, local)| local)
            .unwrap_or(placement.target_id.as_str());
        let system_id = embedded
            .source_structural_grid
            .placements
            .iter()
            .find(|cell| {
                cell.target_id == placement.target_id
                    || cell.location_id == placement.target_id
                    || cell.target_id == local_target
                    || cell.location_id == local_target
                    || (cell.row == placement.row && cell.col == placement.col)
            })
            .map(|cell| cell.system_id)
            .unwrap_or_else(|| panic!("fleet home {} missing lattice join", placement.target_id));
        fleet_home_systems.insert(system_id);
    }
    assert!(
        fleet_home_systems.len() >= 2,
        "need multiple pirate fleet-home systems for OVL falsifier"
    );

    let studio = studio_from_pack(&pack);
    let bridge = open_field_bridge(&studio);
    let map = bridge.readout().disruption_readout;
    let mut nonzero_homes = 0usize;
    for system_id in &fleet_home_systems {
        let raw = map
            .by_system_id
            .get(system_id)
            .map(|r| r.max_disruption_accreted())
            .unwrap_or(0.0);
        if raw > 0.0 {
            nonzero_homes += 1;
        }
    }
    assert!(
        nonzero_homes >= 2,
        "expected >=2 pirate fleet-home systems with local disruption, got {nonzero_homes}"
    );

    let fleet_free = (0..1500u32)
        .find(|system_id| {
            !fleet_home_systems.contains(system_id)
                && map
                    .by_system_id
                    .get(system_id)
                    .map(|r| r.max_disruption_accreted())
                    .unwrap_or(0.0)
                    == 0.0
        })
        .expect("fleet-free comparison system at zero");
    assert_eq!(
        map.by_system_id
            .get(&fleet_free)
            .map(|r| r.max_disruption_accreted())
            .unwrap_or(0.0),
        0.0
    );
}

/// catches: two typed loci on one enrolled system reduce by production exact max.
#[test]
fn two_typed_loci_on_one_enrolled_system_report_exact_max() {
    let mut source =
        std::fs::read_to_string(repo_root().join("scenarios/terran_pirate_galaxy.clause"))
            .expect("clause");
    let needle = r#"        disruption_presence = pirate_raid_presence {
            location = "pirate_outpost"
            resource = "disruption"
            amount = 8
            threshold = 3
            direction = Rising
            event_kind = 71
        }"#;
    let dual = r#"        disruption_presence = pirate_raid_presence {
            location = "pirate_outpost"
            resource = "disruption"
            amount = 3
            threshold = 99
            direction = Rising
            event_kind = 71
        }
        disruption_presence = pirate_smoke_presence {
            location = "pirate_outpost"
            resource = "raid_smoke"
            amount = 8
            threshold = 99
            direction = Rising
            event_kind = 72
        }"#;
    assert!(source.contains(needle), "canonical disruption block missing");
    source = source.replacen(needle, dual, 1);
    // Neutralize owner mult that would inflate the max away from exact authored amounts.
    source = source.replacen(
        r#"        owner_policy_overlay = pirate_disruption_policy {
            owner = "pirate"
            targets_property = "tp_economy::pirate_outpost_disruption_presence"
            amount_mult = 1.35
        }"#,
        "",
        1,
    );
    let pack = hydrate_source(&source).expect("hydrate dual presence");
    let pirate_sys = host_system_id(&pack, "pirate_outpost");
    let studio = studio_from_pack(&pack);
    let bridge = open_field_bridge(&studio);
    let raw = bridge
        .readout()
        .disruption_readout
        .by_system_id
        .get(&pirate_sys)
        .map(|r| r.max_disruption_accreted())
        .unwrap_or(0.0);
    assert_eq!(
        raw, 8.0,
        "production max over two typed loci on one system must be exact max(3,8)=8"
    );
}

/// catches: map frozen at open while runtime disruption changes.
#[test]
fn live_map_refreshes_when_runtime_disruption_changes() {
    let pack = hydrate_canonical();
    let pirate_sys = host_system_id(&pack, "pirate_outpost");
    let studio = studio_from_pack(&pack);
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
    let pirate_sys = host_system_id(&pack, "pirate_outpost");
    let studio = studio_from_pack(&pack);
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

/// catches: missing / unknown / duplicate system_target fail loud at hydrate admission.
#[test]
fn system_target_missing_unknown_and_duplicate_fail_loud() {
    let base = std::fs::read_to_string(repo_root().join("scenarios/terran_pirate_galaxy.clause"))
        .expect("clause");

    let missing = base.replacen(
        "        system_target = \"row158_col110\"\n",
        "",
        1,
    );
    let err = hydrate_source(&missing).expect_err("missing system_target on live host");
    assert!(
        err.contains("no system_target") || err.contains("live spatial"),
        "unexpected missing-target diagnostic: {err}"
    );

    let unknown = base.replacen(
        "system_target = \"row158_col110\"",
        "system_target = \"row0_col0\"",
        1,
    );
    let err = hydrate_source(&unknown).expect_err("unknown lattice cell");
    assert!(
        err.contains("not an embedded placement") || err.contains("system_target"),
        "unexpected unknown-target diagnostic: {err}"
    );

    let duplicate = base.replacen(
        "system_target = \"row158_col110\"",
        "system_target = \"row199_col80\"",
        1,
    );
    let err = hydrate_source(&duplicate).expect_err("duplicate cell claim");
    assert!(
        err.contains("already claimed") || err.contains("system_target"),
        "unexpected duplicate-target diagnostic: {err}"
    );
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
    assert!(total_err.to_string().contains("total structural mapping"));

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
    assert!(partial_err.to_string().contains("partial structural mapping"));
}

/// catches: zero / multiple install_targets hosts fail loud on live refresh (no .first()).
#[test]
fn exact_one_install_target_host_cardinality_fail_loud() {
    let pack = hydrate_canonical();
    let studio = studio_from_pack(&pack);
    let mut bridge = open_field_bridge(&studio);
    {
        let sim = bridge.sim_session_mut().expect("sim");
        sim.scenario
            .install_targets
            .insert("pirate_outpost".into(), vec![]);
    }
    let err = bridge
        .consume_scheduled_ticks(1)
        .expect_err("zero hosts must fail loud on refresh");
    assert!(
        err.to_string().contains("zero install_targets")
            || err.to_string().contains("DisruptionReadback"),
        "unexpected zero-host diagnostic: {err}"
    );

    let studio = studio_from_pack(&pack);
    let mut bridge = open_field_bridge(&studio);
    {
        let sim = bridge.sim_session_mut().expect("sim");
        let a = SimThingId::from_session_raw(1);
        let b = SimThingId::from_session_raw(2);
        sim.scenario
            .install_targets
            .insert("pirate_outpost".into(), vec![a, b]);
    }
    let err = bridge
        .consume_scheduled_ticks(1)
        .expect_err("ambiguous hosts must fail loud on refresh");
    assert!(
        err.to_string().contains("ambiguous") || err.to_string().contains("DisruptionReadback"),
        "unexpected ambiguous-host diagnostic: {err}"
    );
}

/// catches: forced unknown property / role / unallocated host fail loud on the observation door.
#[test]
fn observation_door_unknown_property_role_and_host_fail_loud() {
    let pack = hydrate_canonical();
    let studio = studio_from_pack(&pack);
    let bridge = open_field_bridge(&studio);
    let sim = bridge.sim_session().expect("sim");
    let host = sim
        .scenario
        .install_targets
        .get("pirate_outpost")
        .and_then(|ids| ids.first().copied())
        .expect("pirate_outpost host");
    let snapshot = GpuValuesSnapshot::from_session(sim);

    let unknown_property = observe_hosted_property_cell(
        &sim.proto.registry,
        &sim.proto.allocator,
        &snapshot,
        host,
        &PropertyKey::new("tp_economy", "missing_property"),
        &SubFieldRole::Amount,
    )
    .expect_err("unknown property");
    assert!(matches!(
        unknown_property,
        HostedPropertyObservationError::UnknownProperty { .. }
    ));

    let unknown_role = observe_hosted_property_cell(
        &sim.proto.registry,
        &sim.proto.allocator,
        &snapshot,
        host,
        &PropertyKey::new("tp_economy", "pirate_outpost_disruption_presence"),
        &SubFieldRole::Named("nope".into()),
    )
    .expect_err("unknown role");
    assert!(matches!(
        unknown_role,
        HostedPropertyObservationError::UnknownRole { .. }
    ));

    let unallocated = observe_hosted_property_cell(
        &sim.proto.registry,
        &sim.proto.allocator,
        &snapshot,
        SimThingId::from_session_raw(u32::MAX - 7),
        &PropertyKey::new("tp_economy", "pirate_outpost_disruption_presence"),
        &SubFieldRole::Amount,
    )
    .expect_err("unallocated host");
    assert!(matches!(
        unallocated,
        HostedPropertyObservationError::HostHasNoSlot { .. }
    ));
}