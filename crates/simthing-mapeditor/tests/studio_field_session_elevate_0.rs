//! STUDIO-FIELD-SESSION-ELEVATE-0 — field-bearing live path + structural-shell fallback.
//!
//! Neutral synthetic vocabulary only (foundry_valley / forge) — no scenario-named
//! sealed-crate hits for WORKSHOP-HOMING-DETECTION.

use std::path::PathBuf;

use simthing_clausething::{hydrate_scenario, parse_raw_document};
use simthing_core::SubFieldRole;
use simthing_mapeditor::{
    authored_live_profile_from_pack, runtime_vertical_seed_scenario_spec, StudioLiveSessionBridge,
    StudioLiveSessionBridgeError, StudioLiveSessionPath, StudioLiveSessionPathPreference,
    StudioSession,
};
use simthing_spec::serialize_scenario_authority;

/// Neutral synthetic field-economy scenario (same grammar as 12.6 fixture vocabulary).
const FOUNDRY_SCENARIO: &str = r#"
scenario = foundry_valley {
    metadata = {
        display_name = "Foundry Valley"
    }
    owner = guild {
        owner_key = "guild"
        display_name = "Guild"
        archetype = "industrial"
    }
    owner = union {
        owner_key = "union"
        display_name = "Union"
        archetype = "industrial"
    }
    location = ridge {
        display_name = "Ridge"
    }
    location = basin {
        display_name = "Basin"
    }
    field_economy = valley_economy {
        namespace = "forge"
        field_resource_quantity = ridge_ore {
            location = "ridge"
            resource = "ore"
            amount = 12
        }
        production_building = ridge_foundry {
            location = "ridge"
            input = { resource = "ore" amount = 2 }
            output = { resource = "tools" }
            throttle_hint_max_per_tick = 3
        }
        stockpile_silo = guild_ore {
            owner = "guild"
            resource = "ore"
            current = 20
        }
        disruption_presence = basin_smoke {
            location = "basin"
            resource = "smoke"
            amount = 4
            threshold = 2
            direction = Rising
            event_kind = 77
        }
        weight_profile = expansion_need {
            profile = "expansion-need"
            input = { input_col = 0 weight_col = 10 }
            input = { input_col = 1 weight_col = 11 }
            output_col = 12
        }
    }
}
"#;

fn hydrate_foundry() -> simthing_clausething::HydratedScenarioPack {
    let document = parse_raw_document(FOUNDRY_SCENARIO.as_bytes()).expect("parse");
    hydrate_scenario(&document).expect("hydrate foundry")
}

fn field_bearing_studio_session() -> StudioSession {
    let pack = hydrate_foundry();
    let mut studio = StudioSession::from_loaded_scenario(
        runtime_vertical_seed_scenario_spec(),
        PathBuf::from("tests/fixtures/foundry_valley_field_bearing.clause"),
        None,
    )
    .expect("studio session");
    studio.scenario_authority.scenario_id = pack.scenario_id.clone();
    studio.scenario_summary.scenario_id = pack.scenario_id.clone();
    studio.with_authored_live_profile(authored_live_profile_from_pack(&pack))
}

fn try_open_or_skip(bridge: &mut StudioLiveSessionBridge, studio: &StudioSession) -> bool {
    match bridge.open_from_loaded_studio_session(studio) {
        Ok(()) => true,
        Err(StudioLiveSessionBridgeError::Unsupported(msg)) => {
            eprintln!("STUDIO-FIELD-SESSION-ELEVATE-0: GPU_SKIPPED ({msg})");
            false
        }
        Err(e) => panic!("unexpected open error: {e}"),
    }
}

fn amount_for_property(sim: &simthing_driver::SimSession, namespace: &str, name: &str) -> f32 {
    let reg = &sim.proto.registry;
    let pid = reg
        .id_of(namespace, name)
        .unwrap_or_else(|| panic!("missing property {namespace}::{name}"));
    let layout = &reg.property(pid).layout;
    let col = reg
        .column_range(pid)
        .col_for_role(&SubFieldRole::Amount, layout)
        .expect("amount role")
        .raw_u32() as usize;
    let n_dims = sim.state.n_dims as usize;
    let values = sim.state.read_values();
    if let Some(economy) = sim.spec_state.resource_economy_registry.as_ref() {
        for emission in &economy.registrations.emissions {
            if emission.source_col as usize == col {
                let idx = emission.source_slot as usize * n_dims + col;
                if let Some(v) = values.get(idx) {
                    return *v;
                }
            }
        }
        for transfer in &economy.registrations.transfers {
            if transfer.target_col.raw_u32() as usize == col {
                let idx = transfer.target_slot.raw() as usize * n_dims + col;
                if let Some(v) = values.get(idx) {
                    return *v;
                }
            }
            if transfer.source_col.raw_u32() as usize == col {
                let idx = transfer.source_slot.raw() as usize * n_dims + col;
                if let Some(v) = values.get(idx) {
                    return *v;
                }
            }
        }
    }
    let n_slots = sim.state.n_slots as usize;
    let mut best = 0.0f32;
    for slot in 0..n_slots {
        let idx = slot * n_dims + col;
        if let Some(&v) = values.get(idx) {
            if v > best {
                best = v;
            }
        }
    }
    best
}

/// catches: bridge still only opening structural-shell when a field-economy profile is present.
#[test]
fn field_bearing_path_opens_via_open_from_spec_when_profile_present() {
    let studio = field_bearing_studio_session();
    assert!(studio.authored_live_profile.is_some());
    let mut bridge = StudioLiveSessionBridge::new();
    bridge.set_path_preference(StudioLiveSessionPathPreference::Auto);
    if !try_open_or_skip(&mut bridge, &studio) {
        return;
    }
    assert_eq!(bridge.session_path(), StudioLiveSessionPath::FieldBearing);
    assert_eq!(
        bridge.readout().production_path,
        "simthing_driver::SimSession::open_from_spec + step_once"
    );
    let executed = bridge.consume_scheduled_ticks(3).expect("multi-tick");
    assert!(executed >= 3);
    assert!(!bridge.readout().field_accretion_samples.is_empty());
}

/// catches: structural-shell fallback deleted or no longer selectable under field profile.
#[test]
fn structural_shell_fallback_still_selectable() {
    let studio = field_bearing_studio_session();
    let mut bridge = StudioLiveSessionBridge::new();
    bridge.set_path_preference(StudioLiveSessionPathPreference::StructuralShell);
    if !try_open_or_skip(&mut bridge, &studio) {
        return;
    }
    assert_eq!(bridge.session_path(), StudioLiveSessionPath::StructuralShell);
    assert_eq!(
        bridge.readout().production_path,
        "simthing_driver::SimSession::open + step_once"
    );
    let executed = bridge.consume_scheduled_ticks(2).expect("shell ticks");
    assert!(executed >= 2);
}

/// catches: disruption emission coupling severed (no Constant seed / no presence property).
#[test]
fn disruption_accretes_from_authored_emitter_under_live_ticks() {
    let studio = field_bearing_studio_session();
    let mut bridge = StudioLiveSessionBridge::new();
    bridge.set_path_preference(StudioLiveSessionPathPreference::FieldBearing);
    if !try_open_or_skip(&mut bridge, &studio) {
        return;
    }
    let sim = bridge.sim_session().expect("attached");
    // Open-time materialization seeds authored Constant(4) onto the presence column.
    let seeded = amount_for_property(sim, "forge", "basin_smoke_presence");
    assert!(
        seeded >= 4.0,
        "authored disruption emitter Constant must materialize at open: got {seeded}"
    );
    bridge.consume_scheduled_ticks(3).expect("ticks");
    let samples = bridge.readout().field_accretion_samples;
    assert!(
        samples
            .iter()
            .any(|s| s.property_key.contains("basin_smoke_presence")),
        "per-tick field accretion samples must track the disruption presence property"
    );
    // Live values remain field-coupled after multi-tick (RF may move mass; must stay non-vacant
    // or samples must show the authored key each tick).
    let after = amount_for_property(
        bridge.sim_session().expect("attached"),
        "forge",
        "basin_smoke_presence",
    );
    assert!(
        after > 0.0 || samples.iter().any(|s| s.amount > 0.0),
        "disruption field must remain live after multi-tick; after={after} samples={samples:?}"
    );
}

/// catches: production/silo transfer coupling severed (buildings/overlays not installed).
#[test]
fn production_and_need_accrete_from_buildings_and_overlays() {
    let studio = field_bearing_studio_session();
    let mut bridge = StudioLiveSessionBridge::new();
    bridge.set_path_preference(StudioLiveSessionPathPreference::FieldBearing);
    if !try_open_or_skip(&mut bridge, &studio) {
        return;
    }
    let sim = bridge.sim_session().expect("attached");
    let current_before = amount_for_property(sim, "forge", "guild_ore_current");
    let stockpile_before = amount_for_property(sim, "forge", "guild_ore_stockpile");
    assert!(
        current_before >= 20.0,
        "silo current Constant seed must materialize: {current_before}"
    );
    bridge.consume_scheduled_ticks(4).expect("ticks");
    let sim = bridge.sim_session().expect("attached");
    let current_after = amount_for_property(sim, "forge", "guild_ore_current");
    let stockpile_after = amount_for_property(sim, "forge", "guild_ore_stockpile");
    // Silo transfer (current -> stockpile) is the generic RF proof of building/silo coupling.
    assert!(
        stockpile_after > stockpile_before || current_after < current_before,
        "silo transfer must move mass under live ticks: current {current_before}->{current_after} stockpile {stockpile_before}->{stockpile_after}"
    );
    assert!(
        sim.proto
            .registry
            .id_of("forge", "ridge_tools_quantity")
            .is_some(),
        "tools quantity from production building must install"
    );
}

/// catches: threshold registration/decision wiring severed, or decisions without a threshold.
#[test]
fn decision_fires_only_as_threshold_crossing() {
    let studio = field_bearing_studio_session();
    let mut bridge = StudioLiveSessionBridge::new();
    bridge.set_path_preference(StudioLiveSessionPathPreference::FieldBearing);
    if !try_open_or_skip(&mut bridge, &studio) {
        return;
    }
    let sim = bridge.sim_session().expect("attached");
    let thr_regs = sim
        .spec_state
        .resource_economy_registry
        .as_ref()
        .map(|r| r.registrations.emit_on_threshold.len())
        .unwrap_or(0);
    assert_eq!(
        thr_regs, 1,
        "authored disruption presence must lower one emit_on_threshold"
    );
    let disruption = amount_for_property(sim, "forge", "basin_smoke_presence");
    assert!(
        disruption >= 2.0,
        "seeded disruption must meet/exceed authored threshold at open: {disruption}"
    );
    // Rising threshold: previous seeded 0, current >= 4 — first tick may emit event_kind 77.
    bridge.consume_scheduled_ticks(1).expect("tick");
    let _ = bridge.readout().cumulative_decision_events;

    // Absent a reachable threshold: raise thr above any seed — registry still present but
    // value cannot cross. Falsify by stripping emit_on_threshold from profile entirely.
    let mut pack = hydrate_foundry();
    if let Some(economy) = pack.game_mode.resource_economy.as_mut() {
        economy.emit_on_threshold.clear();
    }
    let mut no_thr = StudioSession::from_loaded_scenario(
        runtime_vertical_seed_scenario_spec(),
        PathBuf::from("tests/fixtures/foundry_valley_no_threshold.clause"),
        None,
    )
    .expect("session");
    no_thr = no_thr.with_authored_live_profile(authored_live_profile_from_pack(&pack));
    let mut bridge2 = StudioLiveSessionBridge::new();
    bridge2.set_path_preference(StudioLiveSessionPathPreference::FieldBearing);
    if !try_open_or_skip(&mut bridge2, &no_thr) {
        return;
    }
    let thr_regs2 = bridge2
        .sim_session()
        .expect("attached")
        .spec_state
        .resource_economy_registry
        .as_ref()
        .map(|r| r.registrations.emit_on_threshold.len())
        .unwrap_or(0);
    assert_eq!(
        thr_regs2, 0,
        "without authored emit_on_threshold, no decision threshold may be installed"
    );
    bridge2.consume_scheduled_ticks(3).expect("ticks");
    assert_eq!(
        bridge2.readout().cumulative_decision_events,
        0,
        "no decision events may fire when no threshold is authored"
    );
}

/// catches: live bridge/UI mutating ScenarioSpec authority.
#[test]
fn field_bearing_ticks_do_not_mutate_scenario_spec() {
    let mut studio = field_bearing_studio_session();
    let before = serialize_scenario_authority(&studio.scenario_authority).expect("ser");
    let mut bridge = StudioLiveSessionBridge::new();
    bridge.set_path_preference(StudioLiveSessionPathPreference::Auto);
    if try_open_or_skip(&mut bridge, &studio) {
        let _ = bridge.consume_scheduled_ticks(2);
    }
    let after = serialize_scenario_authority(&studio.scenario_authority).expect("ser");
    assert_eq!(before, after, "Spec authority must be unchanged by bridge ticks");
    studio.scenario_authority.scenario_id.push_str("_mut");
    let mutated = serialize_scenario_authority(&studio.scenario_authority).expect("ser");
    assert_ne!(before, mutated);
}
