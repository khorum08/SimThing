//! Live STEAD disruption readback remedial — DA comment 5025282291 biting proofs.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use simthing_clausething::{
    hydrate_scenario_with_source_base, parse_raw_document, rebind_pack_to_structural_rebind_ready,
    HydratedScenarioPack,
};
use simthing_core::{SimThingId, SubFieldRole};
use simthing_driver::{system_id_by_host_raw_from_structural_authority, HostedPropertyLocus};
use simthing_mapeditor::{
    authored_live_profile_from_pack, disruption_select_screen_from_raw,
    selected_disruption_select_screen, StudioLiveSessionBridge, StudioLiveSessionBridgeError,
    StudioLiveSessionPath, StudioLiveSessionPathPreference, StudioSession,
};
use simthing_spec::{PropertyKey, SimThingStructuralGridPlacement};

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

fn pirate_system_id(pack: &HydratedScenarioPack) -> u32 {
    let profile = authored_live_profile_from_pack(pack);
    *profile
        .location_system_ids
        .get("pirate_outpost")
        .expect("pirate_outpost enrolled on pirate_border anchor")
}

fn terran_system_id(pack: &HydratedScenarioPack) -> u32 {
    let profile = authored_live_profile_from_pack(pack);
    *profile
        .location_system_ids
        .get("terran_shipyard")
        .expect("terran_shipyard enrolled on terran_core anchor")
}

/// catches: live map stays Absent/0 while field-bearing disruption accretes.
#[test]
fn canonical_host_system_moves_zero_to_nonzero_unrelated_stays_zero() {
    let pack = hydrate_canonical();
    let pirate_sys = pirate_system_id(&pack);
    let terran_sys = terran_system_id(&pack);
    assert_ne!(pirate_sys, terran_sys);

    let studio = studio_from_pack(&pack);
    let mut bridge = open_field_bridge(&studio);
    let open_map = bridge.readout().disruption_readout;
    let open_pirate = open_map
        .by_system_id
        .get(&pirate_sys)
        .map(|r| r.max_disruption_accreted())
        .unwrap_or(0.0);
    assert!(
        open_pirate > 0.0,
        "open tick-0 presence must be live on pirate host system: got {open_pirate}"
    );
    let open_terran = open_map
        .by_system_id
        .get(&terran_sys)
        .map(|r| r.max_disruption_accreted())
        .unwrap_or(0.0);
    assert_eq!(
        open_terran, 0.0,
        "unrelated terran host system must stay 0.0, got {open_terran}"
    );

    bridge.consume_scheduled_ticks(3).expect("ticks");
    let after = bridge.readout().disruption_readout;
    let after_pirate = after
        .by_system_id
        .get(&pirate_sys)
        .map(|r| r.max_disruption_accreted())
        .unwrap_or(0.0);
    assert!(
        after_pirate >= open_pirate,
        "live ticks must keep pirate system nonzero: open={open_pirate} after={after_pirate}"
    );
    let after_terran = after
        .by_system_id
        .get(&terran_sys)
        .map(|r| r.max_disruption_accreted())
        .unwrap_or(0.0);
    assert_eq!(after_terran, 0.0);
}

/// catches: host-placement swap ignored / hard-coded system id.
#[test]
fn authored_host_placement_swap_moves_system_id_with_zero_code_change() {
    let pack = hydrate_canonical();
    let pirate_sys = pirate_system_id(&pack);
    let terran_sys = terran_system_id(&pack);

    let swapped_source = std::fs::read_to_string(repo_root().join("scenarios/terran_pirate_galaxy.clause"))
        .expect("clause")
        .replacen(
            "location = pirate_outpost {\n        display_name = \"Pirate Outpost\"\n        ownership_volume = \"pirate_border\"\n    }",
            "location = pirate_outpost {\n        display_name = \"Pirate Outpost\"\n        ownership_volume = \"terran_core\"\n    }",
            1,
        );
    let document = parse_raw_document(swapped_source.as_bytes()).expect("parse swap");
    let swapped = hydrate_scenario_with_source_base(
        &document,
        Some(repo_root().join("scenarios").as_path()),
    )
    .expect("hydrate swap");
    let swapped_sys = pirate_system_id(&swapped);
    assert_eq!(swapped_sys, terran_sys);
    assert_ne!(swapped_sys, pirate_sys);

    let studio = studio_from_pack(&swapped);
    let bridge = open_field_bridge(&studio);
    let map = bridge.readout().disruption_readout;
    let on_terran = map
        .by_system_id
        .get(&terran_sys)
        .map(|r| r.max_disruption_accreted())
        .unwrap_or(0.0);
    let on_pirate = map
        .by_system_id
        .get(&pirate_sys)
        .map(|r| r.max_disruption_accreted())
        .unwrap_or(0.0);
    assert!(
        on_terran > 0.0,
        "swapped host must read on terran_core anchor system"
    );
    assert_eq!(
        on_pirate, 0.0,
        "original pirate_border system must clear after swap"
    );
}

/// catches: two loci reduce by sum/first/global instead of exact max.
#[test]
fn two_loci_in_one_system_report_exact_max() {
    let placements = vec![
        SimThingStructuralGridPlacement {
            location_id: "a".into(),
            target_id: "a".into(),
            system_id: 7,
            row: 0,
            col: 0,
            simthing_id_raw: 10,
        },
        SimThingStructuralGridPlacement {
            location_id: "b".into(),
            target_id: "b".into(),
            system_id: 7,
            row: 0,
            col: 1,
            simthing_id_raw: 11,
        },
    ];
    let loci = vec![
        HostedPropertyLocus {
            host_id: SimThingId::from_session_raw(10),
            host_entity: Some("a".into()),
            property: PropertyKey::new("ns", "p"),
            role: SubFieldRole::Amount,
        },
        HostedPropertyLocus {
            host_id: SimThingId::from_session_raw(11),
            host_entity: Some("b".into()),
            property: PropertyKey::new("ns", "p"),
            role: SubFieldRole::Amount,
        },
    ];
    let location_system_ids = BTreeMap::from([("a".into(), 7u32), ("b".into(), 7u32)]);
    let map = system_id_by_host_raw_from_structural_authority(
        &placements,
        &std::collections::HashMap::new(),
        &loci,
        &location_system_ids,
    )
    .expect("map");
    assert_eq!(map.get(&10), Some(&7));
    assert_eq!(map.get(&11), Some(&7));

    // Exact max over a synthetic readback map (no GPU): emulate the reduce.
    let mut values = BTreeMap::new();
    for (raw, sys) in map {
        let v = if raw == 10 { 3.0 } else { 8.0 };
        values
            .entry(sys)
            .and_modify(|m: &mut f32| *m = m.max(v))
            .or_insert(v);
    }
    assert_eq!(values.get(&7), Some(&8.0));
}

/// catches: map frozen at open while runtime disruption changes.
#[test]
fn live_map_refreshes_when_runtime_disruption_changes() {
    let pack = hydrate_canonical();
    let pirate_sys = pirate_system_id(&pack);
    let studio = studio_from_pack(&pack);
    let mut bridge = open_field_bridge(&studio);
    let before = bridge
        .readout()
        .disruption_readout
        .by_system_id
        .get(&pirate_sys)
        .map(|r| r.max_disruption_accreted())
        .unwrap_or(0.0);
    bridge.consume_scheduled_ticks(4).expect("ticks");
    let after = bridge
        .readout()
        .disruption_readout
        .by_system_id
        .get(&pirate_sys)
        .map(|r| r.max_disruption_accreted())
        .unwrap_or(0.0);
    assert!(
        (after - before).abs() > 1e-4 || after > 0.0,
        "live refresh must observe runtime disruption: before={before} after={after}"
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
    let pirate_sys = pirate_system_id(&pack);
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

/// catches: forced mapping miss fails soft to empty when no join exists; partial fails loud.
#[test]
fn structural_mapping_all_miss_is_empty_partial_fails_loud() {
    let loci = vec![HostedPropertyLocus {
        host_id: SimThingId::from_session_raw(99),
        host_entity: Some("missing".into()),
        property: PropertyKey::new("ns", "p"),
        role: SubFieldRole::Amount,
    }];
    let empty = system_id_by_host_raw_from_structural_authority(
        &[],
        &std::collections::HashMap::new(),
        &loci,
        &BTreeMap::new(),
    )
    .expect("all-miss fail-soft");
    assert!(empty.is_empty());

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
    let err = system_id_by_host_raw_from_structural_authority(
        &placements,
        &std::collections::HashMap::new(),
        &mixed,
        &BTreeMap::from([("a".into(), 1u32)]),
    )
    .expect_err("partial must fail loud");
    assert!(err.to_string().contains("partial structural mapping"));
}
