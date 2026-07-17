//! STUDIO-FIELD-SESSION-ELEVATE-0 — field-bearing live path + structural-shell fallback.
//!
//! Neutral synthetic vocabulary only (foundry_valley / forge) — no scenario-named
//! sealed-crate hits for WORKSHOP-HOMING-DETECTION.
//!
//! Fail-closed: Unsupported/GPU open failure is a test failure (not a silent green skip).

use std::path::PathBuf;

use simthing_clausething::{hydrate_scenario, parse_raw_document, HydratedScenarioPack};
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
        owner_policy_overlay = guild_expansion_policy {
            owner = "guild"
            targets_property = "forge::ridge_tools_quantity"
            amount_mult = 1.15
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

fn hydrate_foundry() -> HydratedScenarioPack {
    let document = parse_raw_document(FOUNDRY_SCENARIO.as_bytes()).expect("parse");
    hydrate_scenario(&document).expect("hydrate foundry")
}

fn field_bearing_studio_session_from(pack: &HydratedScenarioPack) -> StudioSession {
    let mut studio = StudioSession::from_loaded_scenario(
        runtime_vertical_seed_scenario_spec(),
        PathBuf::from("tests/fixtures/foundry_valley_field_bearing.clause"),
        None,
    )
    .expect("studio session");
    studio.scenario_authority.scenario_id = pack.scenario_id.clone();
    studio.scenario_summary.scenario_id = pack.scenario_id.clone();
    studio.with_authored_live_profile(authored_live_profile_from_pack(pack))
}

fn field_bearing_studio_session() -> StudioSession {
    field_bearing_studio_session_from(&hydrate_foundry())
}

/// Fail-closed open: Unsupported is a hard failure (GPU required for load-bearing proofs).
fn open_fail_closed(
    bridge: &mut StudioLiveSessionBridge,
    studio: &StudioSession,
) -> Result<(), StudioLiveSessionBridgeError> {
    match bridge.open_from_loaded_studio_session(studio) {
        Ok(()) => Ok(()),
        Err(StudioLiveSessionBridgeError::Unsupported(msg)) => {
            panic!(
                "STUDIO-FIELD-SESSION-ELEVATE-0: GPU/adapter Unsupported is a FAIL (not a skip): {msg}"
            );
        }
        Err(e) => Err(e),
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

fn policy_overlay_count(pack: &HydratedScenarioPack) -> usize {
    pack.game_mode
        .overlays
        .iter()
        .filter(|o| o.id.contains("owner_policy"))
        .count()
}

/// catches: bridge still only opening structural-shell when a field-economy profile is present.
#[test]
fn field_bearing_path_opens_via_open_from_spec_when_profile_present() {
    let studio = field_bearing_studio_session();
    assert!(studio.authored_live_profile.is_some());
    let mut bridge = StudioLiveSessionBridge::new();
    bridge.set_path_preference(StudioLiveSessionPathPreference::Auto);
    open_fail_closed(&mut bridge, &studio).expect("field-bearing open");
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
    open_fail_closed(&mut bridge, &studio).expect("shell open");
    assert_eq!(bridge.session_path(), StudioLiveSessionPath::StructuralShell);
    assert_eq!(
        bridge.readout().production_path,
        "simthing_driver::SimSession::open + step_once"
    );
    let executed = bridge.consume_scheduled_ticks(2).expect("shell ticks");
    assert!(executed >= 2);
}

/// catches: disruption emission coupling severed (no Constant seed / no live tick delta).
#[test]
fn disruption_accretes_from_authored_emitter_under_live_ticks() {
    let studio = field_bearing_studio_session();
    let mut bridge = StudioLiveSessionBridge::new();
    bridge.set_path_preference(StudioLiveSessionPathPreference::FieldBearing);
    open_fail_closed(&mut bridge, &studio).expect("open");
    let sim = bridge.sim_session().expect("attached");
    let open_amount = amount_for_property(sim, "forge", "basin_smoke_presence");
    assert!(
        open_amount >= 4.0,
        "authored disruption Constant must materialize at open: got {open_amount}"
    );
    bridge.consume_scheduled_ticks(3).expect("ticks");
    let samples = bridge.readout().field_accretion_samples;
    assert!(
        samples
            .iter()
            .any(|s| s.property_key.contains("basin_smoke_presence")),
        "per-tick field accretion samples must track the disruption presence property: {samples:?}"
    );
    let after = amount_for_property(
        bridge.sim_session().expect("attached"),
        "forge",
        "basin_smoke_presence",
    );
    // Live RF/tick path must move the field off its open-time seed (or sample a tick delta).
    let sample_delta = samples
        .iter()
        .filter(|s| s.property_key.contains("basin_smoke_presence"))
        .map(|s| s.amount)
        .collect::<Vec<_>>();
    let sample_changed = sample_delta
        .windows(2)
        .any(|w| (w[0] - w[1]).abs() > 1e-4);
    assert!(
        (after - open_amount).abs() > 1e-4 || sample_changed,
        "disruption must show a live per-tick delta (not open-seed persistence alone): open={open_amount} after={after} samples={sample_delta:?}"
    );
}

/// catches: production/silo transfer + policy-overlay coupling severed.
#[test]
fn production_and_need_accrete_from_buildings_and_overlays() {
    let pack = hydrate_foundry();
    let policies = policy_overlay_count(&pack);
    assert!(
        policies >= 1,
        "foundry must author at least one owner_policy_overlay for policy coupling proof"
    );
    let studio = field_bearing_studio_session_from(&pack);
    let mut bridge = StudioLiveSessionBridge::new();
    bridge.set_path_preference(StudioLiveSessionPathPreference::FieldBearing);
    open_fail_closed(&mut bridge, &studio).expect("open");
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

    // Policy/need falsifier: stripping owner_policy overlays must change the installed overlay set.
    let mut stripped = hydrate_foundry();
    stripped
        .game_mode
        .overlays
        .retain(|o| !o.id.contains("owner_policy"));
    assert_eq!(
        policy_overlay_count(&stripped),
        0,
        "stripped pack must have zero owner_policy overlays"
    );
    let with_n = pack.game_mode.overlays.len();
    let without_n = stripped.game_mode.overlays.len();
    assert!(
        with_n > without_n,
        "policy overlays must be a real differential in the authored profile: with={with_n} without={without_n}"
    );
    // Both paths must open and still run silo RF (generic pipeline), but the stripped profile
    // must not re-admit the policy overlay ids.
    let studio_stripped = field_bearing_studio_session_from(&stripped);
    let mut bridge2 = StudioLiveSessionBridge::new();
    bridge2.set_path_preference(StudioLiveSessionPathPreference::FieldBearing);
    open_fail_closed(&mut bridge2, &studio_stripped).expect("open stripped");
    let profile = studio_stripped
        .authored_live_profile
        .as_ref()
        .expect("profile");
    assert!(
        !profile
            .game_mode
            .overlays
            .iter()
            .any(|o| o.id.contains("owner_policy")),
        "stripped live profile must not carry owner_policy overlays into the field-bearing open"
    );
    bridge2.consume_scheduled_ticks(2).expect("ticks");
}

/// catches: threshold registration/decision wiring severed, or decisions without a threshold.
#[test]
fn decision_fires_only_as_threshold_crossing() {
    let studio = field_bearing_studio_session();
    let mut bridge = StudioLiveSessionBridge::new();
    bridge.set_path_preference(StudioLiveSessionPathPreference::FieldBearing);
    open_fail_closed(&mut bridge, &studio).expect("open");
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
    // Open-edge Rising: previous=0, values=seed observed at field-bearing open
    // (before step_once snapshot would erase the edge).
    let decisions_at_open = bridge.readout().cumulative_decision_events;
    assert!(
        decisions_at_open > 0,
        "Rising open-edge threshold crossing must produce a nonzero decision event count at open; got {decisions_at_open}"
    );
    bridge.consume_scheduled_ticks(1).expect("tick");
    assert!(
        bridge.readout().cumulative_decision_events >= decisions_at_open,
        "decision counter must be monotonic across ticks"
    );

    // Same duration, no threshold: zero decisions.
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
    open_fail_closed(&mut bridge2, &no_thr).expect("open no-threshold");
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
    open_fail_closed(&mut bridge, &studio).expect("open");
    let _ = bridge.consume_scheduled_ticks(2).expect("ticks");
    let after = serialize_scenario_authority(&studio.scenario_authority).expect("ser");
    assert_eq!(before, after, "Spec authority must be unchanged by bridge ticks");
    studio.scenario_authority.scenario_id.push_str("_mut");
    let mutated = serialize_scenario_authority(&studio.scenario_authority).expect("ser");
    assert_ne!(before, mutated);
}
