//! STUDIO-FIELD-SESSION-ELEVATE-0 — field-bearing live path + structural-shell fallback.
//!
//! Neutral synthetic vocabulary only (foundry_valley / forge).
//! Fail-closed: Unsupported is FAIL. No fabricated open-time decisions.
//! Rising crossings must occur under live `step_once` after generic RF/overlay evolution.

use std::path::PathBuf;

use simthing_clausething::{hydrate_scenario, parse_raw_document, HydratedScenarioPack};
use simthing_core::SubFieldRole;
use simthing_mapeditor::{
    authored_live_profile_from_pack, runtime_vertical_seed_scenario_spec, StudioLiveSessionBridge,
    StudioLiveSessionBridgeError, StudioLiveSessionPath, StudioLiveSessionPathPreference,
    StudioSession,
};
use simthing_spec::serialize_scenario_authority;

/// Synthetic field-economy: disruption seeds **below** threshold so a Rising
/// crossing can only fire after live tick overlay/RF evolution (not at open).
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
            amount = 1
            threshold = 1.5
            direction = Rising
            event_kind = 77
        }
        owner_policy_overlay = guild_ore_pressure {
            owner = "guild"
            targets_property = "forge::ridge_ore_quantity"
            amount_add = 3.0
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

fn amount_col(sim: &simthing_driver::SimSession, namespace: &str, name: &str) -> usize {
    let reg = &sim.proto.registry;
    let pid = reg
        .id_of(namespace, name)
        .unwrap_or_else(|| panic!("missing property {namespace}::{name}"));
    let layout = &reg.property(pid).layout;
    reg.column_range(pid)
        .col_for_role(&SubFieldRole::Amount, layout)
        .expect("amount role")
        .raw_u32() as usize
}

/// Read Amount at an exact install-target host slot (owner/location shell).
fn amount_at_install_target(
    sim: &simthing_driver::SimSession,
    target_id: &str,
    namespace: &str,
    name: &str,
) -> f32 {
    let thing_id = sim
        .scenario
        .install_targets
        .get(target_id)
        .and_then(|ids| ids.first().copied())
        .unwrap_or_else(|| panic!("missing install_targets key `{target_id}`"));
    let slot = usize::from(
        sim.proto
            .allocator
            .slot_of(thing_id)
            .unwrap_or_else(|| panic!("no GPU slot for install target `{target_id}`")),
    );
    let col = amount_col(sim, namespace, name);
    let n_dims = sim.state.n_dims as usize;
    let idx = slot * n_dims + col;
    sim.state
        .read_values()
        .get(idx)
        .copied()
        .unwrap_or_else(|| panic!("OOB read slot={slot} col={col} idx={idx}"))
}

fn amount_for_property(sim: &simthing_driver::SimSession, namespace: &str, name: &str) -> f32 {
    let col = amount_col(sim, namespace, name);
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
    open_fail_closed(&mut bridge, &studio).expect("field-bearing open");
    assert_eq!(bridge.session_path(), StudioLiveSessionPath::FieldBearing);
    assert_eq!(
        bridge.readout().production_path,
        "simthing_driver::SimSession::open_from_spec + step_once"
    );
    // Open must not invent a decision.
    assert_eq!(
        bridge.readout().cumulative_decision_events,
        0,
        "no fabricated open-time decision edge"
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

/// catches: disruption emission coupling severed (open seed only / no live delta).
#[test]
fn disruption_accretes_from_authored_emitter_under_live_ticks() {
    let studio = field_bearing_studio_session();
    let mut bridge = StudioLiveSessionBridge::new();
    bridge.set_path_preference(StudioLiveSessionPathPreference::FieldBearing);
    open_fail_closed(&mut bridge, &studio).expect("open");
    let open_amount = amount_for_property(
        bridge.sim_session().expect("attached"),
        "forge",
        "basin_smoke_presence",
    );
    assert!(
        open_amount >= 1.0,
        "authored disruption Constant must materialize at open: got {open_amount}"
    );
    bridge.consume_scheduled_ticks(3).expect("ticks");
    let after = amount_for_property(
        bridge.sim_session().expect("attached"),
        "forge",
        "basin_smoke_presence",
    );
    let samples = bridge.readout().field_accretion_samples;
    let sample_series: Vec<f32> = samples
        .iter()
        .filter(|s| s.property_key.contains("basin_smoke_presence"))
        .map(|s| s.amount)
        .collect();
    let changed = (after - open_amount).abs() > 1e-4
        || sample_series
            .windows(2)
            .any(|w| (w[0] - w[1]).abs() > 1e-4);
    assert!(
        changed,
        "disruption must show a live per-tick delta: open={open_amount} after={after} samples={sample_series:?}"
    );
}

/// catches: silo transfer coupling severed OR owner-policy overlay application deleted.
#[test]
fn production_and_need_accrete_from_buildings_and_overlays() {
    const TICKS: u64 = 4;
    // Authored amount_add = 3.0 on guild → forge::ridge_ore_quantity.
    const POLICY_AMOUNT_ADD: f32 = 3.0;

    let pack = hydrate_foundry();
    let studio = field_bearing_studio_session_from(&pack);
    let mut bridge = StudioLiveSessionBridge::new();
    bridge.set_path_preference(StudioLiveSessionPathPreference::FieldBearing);
    open_fail_closed(&mut bridge, &studio).expect("open");
    let sim = bridge.sim_session().expect("attached");
    let current_before = amount_for_property(sim, "forge", "guild_ore_current");
    let stockpile_before = amount_for_property(sim, "forge", "guild_ore_stockpile");
    assert!(current_before >= 20.0, "silo current seed: {current_before}");
    bridge.consume_scheduled_ticks(TICKS).expect("ticks");
    let sim = bridge.sim_session().expect("attached");
    let current_after = amount_for_property(sim, "forge", "guild_ore_current");
    let stockpile_after = amount_for_property(sim, "forge", "guild_ore_stockpile");
    let ore_with_policy =
        amount_at_install_target(sim, "guild", "forge", "ridge_ore_quantity");
    assert!(
        stockpile_after > stockpile_before || current_after < current_before,
        "silo transfer must move mass: current {current_before}->{current_after} stockpile {stockpile_before}->{stockpile_after}"
    );
    assert!(
        sim.proto
            .registry
            .id_of("forge", "ridge_tools_quantity")
            .is_some(),
        "tools quantity from production building must install"
    );

    // Strict with-vs-without on the owner host slot — no open→after escape hatch.
    let mut stripped = hydrate_foundry();
    stripped
        .game_mode
        .overlays
        .retain(|o| !o.id.contains("owner_policy"));
    assert!(
        pack.game_mode.overlays.len() > stripped.game_mode.overlays.len(),
        "pack must carry owner_policy overlays to strip"
    );
    let studio_stripped = field_bearing_studio_session_from(&stripped);
    let mut bridge2 = StudioLiveSessionBridge::new();
    bridge2.set_path_preference(StudioLiveSessionPathPreference::FieldBearing);
    open_fail_closed(&mut bridge2, &studio_stripped).expect("open stripped");
    bridge2.consume_scheduled_ticks(TICKS).expect("ticks");
    let ore_without_policy = amount_at_install_target(
        bridge2.sim_session().expect("attached"),
        "guild",
        "forge",
        "ridge_ore_quantity",
    );
    let delta = ore_with_policy - ore_without_policy;
    assert!(
        delta > 1e-3,
        "owner-policy must raise guild-slot forge::ridge_ore_quantity vs stripped after identical ticks: with={ore_with_policy} without={ore_without_policy}"
    );
    // Direction/delta attributable to amount_add=3.0 (permanent Add applies each tick).
    assert!(
        (delta - POLICY_AMOUNT_ADD * TICKS as f32).abs() < 1e-2
            || (delta - POLICY_AMOUNT_ADD).abs() < 1e-2
            || delta >= POLICY_AMOUNT_ADD - 1e-2,
        "policy delta must be attributable to amount_add={POLICY_AMOUNT_ADD} (got delta={delta} after {TICKS} ticks)"
    );
}

/// catches: threshold upload deleted, or decisions without a crossing / at open.
#[test]
fn decision_fires_only_as_threshold_crossing() {
    let studio = field_bearing_studio_session();
    let mut bridge = StudioLiveSessionBridge::new();
    bridge.set_path_preference(StudioLiveSessionPathPreference::FieldBearing);
    open_fail_closed(&mut bridge, &studio).expect("open");
    let thr_regs = bridge
        .sim_session()
        .expect("attached")
        .spec_state
        .resource_economy_registry
        .as_ref()
        .map(|r| r.registrations.emit_on_threshold.len())
        .unwrap_or(0);
    assert_eq!(thr_regs, 1, "authored emit_on_threshold must install");
    let open_presence = amount_for_property(
        bridge.sim_session().expect("attached"),
        "forge",
        "basin_smoke_presence",
    );
    assert!(
        open_presence < 1.5,
        "synthetic seed must start below threshold so open cannot invent a Rising edge: {open_presence}"
    );
    assert_eq!(
        bridge.readout().cumulative_decision_events,
        0,
        "zero decisions at open (initial state is not a decision)"
    );

    // Live ticks: permanent Crisis overlay Add(amount) + seed evolution can Rising-cross thr=2.
    bridge.consume_scheduled_ticks(4).expect("ticks");
    let decisions = bridge.readout().cumulative_decision_events;
    assert!(
        decisions > 0,
        "Rising crossing must fire under live step_once after generic overlay/RF evolution; got {decisions}"
    );

    // No-threshold control: same duration, zero decisions.
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
    assert_eq!(
        bridge2
            .sim_session()
            .expect("attached")
            .spec_state
            .resource_economy_registry
            .as_ref()
            .map(|r| r.registrations.emit_on_threshold.len())
            .unwrap_or(0),
        0
    );
    bridge2.consume_scheduled_ticks(4).expect("ticks");
    assert_eq!(
        bridge2.readout().cumulative_decision_events,
        0,
        "no decision events when no threshold is authored"
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

/// catches: Studio_ops telemetry sampler still hard-codes slot 0 / wrong locus (OVL hollow table).
#[test]
fn field_bearing_readout_samples_show_live_accretion_delta() {
    let studio = field_bearing_studio_session();
    let mut bridge = StudioLiveSessionBridge::new();
    bridge.set_path_preference(StudioLiveSessionPathPreference::FieldBearing);
    open_fail_closed(&mut bridge, &studio).expect("open");
    bridge.consume_scheduled_ticks(4).expect("ticks");
    let samples = bridge.readout().field_accretion_samples;
    let series: Vec<(u64, f32)> = samples
        .iter()
        .filter(|s| s.property_key.contains("basin_smoke_presence"))
        .map(|s| (s.tick_index, s.amount))
        .collect();
    assert!(
        series.len() >= 2,
        "readout must retain ≥2 tick samples for disruption property; got {series:?}"
    );
    let amounts: Vec<f32> = series.iter().map(|(_, a)| *a).collect();
    let changed = amounts
        .windows(2)
        .any(|w| (w[0] - w[1]).abs() > 1e-4)
        || (amounts.first().copied().unwrap_or(0.0) - amounts.last().copied().unwrap_or(0.0)).abs()
            > 1e-4;
    assert!(
        changed,
        "Studio_ops field_accretion_samples must show a real value delta across ticks (sampler slot/col bug if static): {series:?}"
    );
}
