//! STUDIO-FIELD-SESSION-ELEVATE-0 — multi-tick proof on the 12.8-authored canonical scenario.
//!
//! §12: scenario-specific residue homes here. Production open/seed/threshold wiring is
//! exercised only through the Studio live bridge (no test-side replica).

use std::env;
use std::path::{Path, PathBuf};

use simthing_clausething::{
    hydrate_scenario_with_source_base, parse_raw_document, HydratedScenarioPack,
};
use simthing_core::SubFieldRole;
use simthing_mapeditor::{
    authored_live_profile_from_pack, runtime_vertical_seed_scenario_spec, StudioLiveSessionBridge,
    StudioLiveSessionBridgeError, StudioLiveSessionPath, StudioLiveSessionPathPreference,
    StudioSession,
};
use simthing_spec::EmissionFormulaSpec;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

fn hydrate_canonical() -> HydratedScenarioPack {
    let clause_path = repo_root().join("scenarios/terran_pirate_galaxy.clause");
    let text = std::fs::read_to_string(&clause_path).expect("read canonical clause");
    let document = parse_raw_document(text.as_bytes()).expect("parse");
    let base = clause_path.parent().expect("parent");
    hydrate_scenario_with_source_base(&document, Some(base)).expect("hydrate")
}

/// Production Studio session + authored live profile (same elevation path as Studio UI).
fn studio_from_pack(pack: &HydratedScenarioPack) -> StudioSession {
    let mut studio = StudioSession::from_loaded_scenario(
        runtime_vertical_seed_scenario_spec(),
        repo_root().join("scenarios/terran_pirate_galaxy.clause"),
        None,
    )
    .expect("studio session");
    studio.scenario_authority.scenario_id = pack.scenario_id.clone();
    studio.scenario_summary.scenario_id = pack.scenario_id.clone();
    studio.with_authored_live_profile(authored_live_profile_from_pack(pack))
}

fn open_field_bridge(studio: &StudioSession) -> StudioLiveSessionBridge {
    let mut bridge = StudioLiveSessionBridge::new();
    bridge.set_path_preference(StudioLiveSessionPathPreference::FieldBearing);
    match bridge.open_from_loaded_studio_session(studio) {
        Ok(()) => {}
        Err(StudioLiveSessionBridgeError::Unsupported(msg)) => {
            panic!(
                "TP-FIELD-SESSION-ELEVATE-0: GPU/adapter Unsupported is a FAIL (not a skip): {msg}"
            );
        }
        Err(e) => panic!("production field-bearing open failed: {e}"),
    }
    assert_eq!(bridge.session_path(), StudioLiveSessionPath::FieldBearing);
    bridge
}

fn amount_col(sim: &simthing_driver::SimSession, namespace: &str, name: &str) -> usize {
    let reg = &sim.proto.registry;
    let pid = reg
        .id_of(namespace, name)
        .unwrap_or_else(|| panic!("missing {namespace}::{name}"));
    let layout = &reg.property(pid).layout;
    reg.column_range(pid)
        .col_for_role(&SubFieldRole::Amount, layout)
        .expect("amount")
        .raw_u32() as usize
}

/// Exact install-target host slot Amount (owner/location shell).
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
            .unwrap_or_else(|| panic!("no GPU slot for `{target_id}`")),
    );
    let col = amount_col(sim, namespace, name);
    let n_dims = sim.state.n_dims as usize;
    let idx = slot * n_dims + col;
    sim.state
        .read_values()
        .get(idx)
        .copied()
        .unwrap_or_else(|| panic!("OOB read slot={slot} col={col}"))
}

fn amount(sim: &simthing_driver::SimSession, namespace: &str, name: &str) -> f32 {
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
    values.get(col).copied().unwrap_or(0.0)
}

/// Clone canonical pack with disruption Constant lowered below Rising thr so live
/// overlay/RF evolution can cross during ordinary step_once (no open-time scan).
fn pack_below_threshold_disruption() -> HydratedScenarioPack {
    let mut pack = hydrate_canonical();
    let thr = pack
        .game_mode
        .resource_economy
        .as_ref()
        .and_then(|e| e.emit_on_threshold.first())
        .map(|t| t.threshold)
        .unwrap_or(3.0);
    if let Some(economy) = pack.game_mode.resource_economy.as_mut() {
        for emission in &mut economy.emissions {
            if emission.id.contains("presence") || emission.source.name.contains("disruption") {
                emission.formula = EmissionFormulaSpec::Constant(thr - 1.0);
            }
        }
    }
    pack
}

/// catches: 12.8 disruption emitter not materializing / not live under production bridge.
#[test]
fn canonical_disruption_accretes_from_authored_emitter() {
    let pack = hydrate_canonical();
    assert!(pack.field_economy.is_some(), "12.8 field economy must hydrate");
    let studio = studio_from_pack(&pack);
    let mut bridge = open_field_bridge(&studio);
    assert_eq!(
        bridge.readout().cumulative_decision_events,
        0,
        "canonical open must not invent a time-zero decision"
    );
    let open_amount = amount(
        bridge.sim_session().expect("attached"),
        "tp_economy",
        "pirate_outpost_disruption_presence",
    );
    assert!(
        open_amount >= 8.0,
        "disruption Constant seed must materialize via production open: {open_amount}"
    );
    bridge.consume_scheduled_ticks(3).expect("ticks");
    let after = amount(
        bridge.sim_session().expect("attached"),
        "tp_economy",
        "pirate_outpost_disruption_presence",
    );
    let samples = bridge.readout().field_accretion_samples;
    let series: Vec<f32> = samples
        .iter()
        .filter(|s| s.property_key.contains("disruption_presence"))
        .map(|s| s.amount)
        .collect();
    let changed = (after - open_amount).abs() > 1e-4
        || series.windows(2).any(|w| (w[0] - w[1]).abs() > 1e-4);
    assert!(
        changed,
        "canonical disruption must show a live per-tick delta: open={open_amount} after={after} series={series:?}"
    );
}

/// catches: silo transfer severed OR owner-policy overlay application deleted.
#[test]
fn canonical_production_need_accrete_from_buildings_and_overlays() {
    const TICKS: u64 = 4;
    let pack = hydrate_canonical();
    let studio = studio_from_pack(&pack);
    let mut bridge = open_field_bridge(&studio);
    let sim = bridge.sim_session().expect("attached");
    let current_before = amount(sim, "tp_economy", "terran_minerals_current");
    let stockpile_before = amount(sim, "tp_economy", "terran_minerals_stockpile");
    assert!(
        current_before >= 40.0,
        "terran minerals silo current must seed: {current_before}"
    );
    bridge.consume_scheduled_ticks(TICKS).expect("ticks");
    let sim = bridge.sim_session().expect("attached");
    let current_after = amount(sim, "tp_economy", "terran_minerals_current");
    let stockpile_after = amount(sim, "tp_economy", "terran_minerals_stockpile");
    assert!(
        stockpile_after > stockpile_before || current_after < current_before,
        "silo transfer must move mass: current {current_before}->{current_after} stockpile {stockpile_before}->{stockpile_after}"
    );
    assert!(
        sim.proto
            .registry
            .id_of("tp_economy", "terran_shipyard_hulls_quantity")
            .is_some(),
        "hulls quantity from production building must install"
    );

    // Authored policy targets (not minerals proxy): terran hulls + pirate disruption.
    let hulls_with =
        amount_at_install_target(sim, "terran", "tp_economy", "terran_shipyard_hulls_quantity");
    let disruption_with = amount_at_install_target(
        sim,
        "pirate",
        "tp_economy",
        "pirate_outpost_disruption_presence",
    );

    let mut stripped = hydrate_canonical();
    stripped
        .game_mode
        .overlays
        .retain(|o| !o.id.contains("owner_policy"));
    assert!(
        pack.game_mode.overlays.len() > stripped.game_mode.overlays.len(),
        "canonical must author owner_policy overlays"
    );
    let studio_stripped = studio_from_pack(&stripped);
    let mut bridge2 = open_field_bridge(&studio_stripped);
    bridge2.consume_scheduled_ticks(TICKS).expect("ticks");
    let sim2 = bridge2.sim_session().expect("attached");
    let hulls_without = amount_at_install_target(
        sim2,
        "terran",
        "tp_economy",
        "terran_shipyard_hulls_quantity",
    );
    let disruption_without = amount_at_install_target(
        sim2,
        "pirate",
        "tp_economy",
        "pirate_outpost_disruption_presence",
    );

    // Require a real with≠without differential on at least one authored policy host.
    // No open→after escape.
    let hulls_delta = (hulls_with - hulls_without).abs();
    let disruption_delta = (disruption_with - disruption_without).abs();
    assert!(
        hulls_delta > 1e-3 || disruption_delta > 1e-3,
        "owner-policy must change exact owner-slot values after identical ticks: hulls with={hulls_with} without={hulls_without}; disruption with={disruption_with} without={disruption_without}"
    );
}

/// catches: threshold evaluation deleted while registration remains; open-time invention.
#[test]
fn canonical_decision_fires_only_on_threshold_crossing() {
    const TICKS: u64 = 4;
    // Below-threshold Constant seed + authored Rising thr + Crisis overlay evolution.
    let pack = pack_below_threshold_disruption();
    let thr = pack
        .game_mode
        .resource_economy
        .as_ref()
        .and_then(|e| e.emit_on_threshold.first())
        .map(|t| t.threshold)
        .expect("canonical disruption threshold");
    let studio = studio_from_pack(&pack);
    let mut bridge = open_field_bridge(&studio);
    let thr_regs = bridge
        .sim_session()
        .expect("attached")
        .spec_state
        .resource_economy_registry
        .as_ref()
        .map(|r| r.registrations.emit_on_threshold.len())
        .unwrap_or(0);
    assert!(
        thr_regs >= 1,
        "canonical disruption presence must install emit_on_threshold via production open"
    );
    let open_disruption = amount(
        bridge.sim_session().expect("attached"),
        "tp_economy",
        "pirate_outpost_disruption_presence",
    );
    assert!(
        open_disruption < thr,
        "prepared initial disruption must start below Rising thr: open={open_disruption} thr={thr}"
    );
    assert_eq!(
        bridge.readout().cumulative_decision_events,
        0,
        "zero decisions at open"
    );

    bridge.consume_scheduled_ticks(TICKS).expect("ticks");
    let after = amount(
        bridge.sim_session().expect("attached"),
        "tp_economy",
        "pirate_outpost_disruption_presence",
    );
    let decisions = bridge.readout().cumulative_decision_events;
    assert!(
        after > open_disruption || after >= thr,
        "live accretion/overlay must move disruption toward/above thr: open={open_disruption} after={after} thr={thr}"
    );
    assert!(
        decisions > 0,
        "ordinary live ticks must produce a positive sealed threshold-event count; got {decisions} (open={open_disruption} after={after} thr={thr})"
    );

    // No-threshold control: same below-threshold pack with thresholds stripped.
    let mut pack_none = pack_below_threshold_disruption();
    if let Some(economy) = pack_none.game_mode.resource_economy.as_mut() {
        economy.emit_on_threshold.clear();
    }
    let studio_none = studio_from_pack(&pack_none);
    let mut bridge_none = open_field_bridge(&studio_none);
    assert_eq!(
        bridge_none
            .sim_session()
            .expect("attached")
            .spec_state
            .resource_economy_registry
            .as_ref()
            .map(|r| r.registrations.emit_on_threshold.len())
            .unwrap_or(0),
        0
    );
    bridge_none.consume_scheduled_ticks(TICKS).expect("ticks");
    assert_eq!(
        bridge_none.readout().cumulative_decision_events,
        0,
        "no decision events when no threshold is authored"
    );
}
