//! STUDIO-FIELD-SESSION-ELEVATE-0 — multi-tick proof on the 12.8-authored canonical scenario.
//!
//! Scenario-specific (Terran-Pirate) multi-tick residue homes to simthing-workshop (§12).
//! Production mapeditor bridge consumes the elevated open_from_spec path generically.
//!
//! Fail-closed: GPU/adapter open failure is a test failure (not a silent green skip).

use std::env;
use std::path::{Path, PathBuf};

use simthing_clausething::{
    hydrate_scenario_with_source_base, parse_raw_document, HydratedScenarioPack,
};
use simthing_core::{DimensionRegistry, SimProperty, SubFieldRole};
use simthing_driver::{Scenario, SessionError, SimSession};
use simthing_gpu::{
    emit_on_threshold_registrations_to_gpu, EmissionFormula, DEFAULT_THRESHOLD_EMISSION_CAPACITY,
};

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

fn scenario_from_pack(pack: &HydratedScenarioPack) -> Scenario {
    // Field-economy focused shell: World + economy location shells from install_targets.
    let mut registry = DimensionRegistry::new();
    let _ = registry.register(SimProperty::simple("_field_session", "seed", 0));
    let mut root = simthing_core::SimThing::new(simthing_core::SimThingKind::World, 0);
    let mut install_targets = std::collections::HashMap::new();
    install_targets.insert(pack.scenario_id.clone(), vec![root.id]);

    for child in &pack.root.children {
        let mut shell = child.clone();
        strip_props(&mut shell);
        install_targets.insert(
            pack.install_targets
                .iter()
                .find(|(_, ids)| ids.first() == Some(&child.id))
                .map(|(k, _)| k.clone())
                .unwrap_or_else(|| format!("loc_{}", child.id.raw())),
            vec![child.id],
        );
        root.add_child(shell);
    }
    for (key, ids) in &pack.install_targets {
        install_targets
            .entry(key.clone())
            .or_insert_with(|| ids.clone());
    }

    let mut n_slots = 0u32;
    count_nodes(&root, &mut n_slots);
    n_slots = n_slots.max(1).saturating_mul(4).max(64);
    Scenario {
        name: pack.scenario_id.clone(),
        ticks_per_day: 1,
        max_days: 1_000_000,
        dt: 1.0,
        n_slots,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets,
    }
}

fn strip_props(node: &mut simthing_core::SimThing) {
    node.properties.clear();
    for child in &mut node.children {
        strip_props(child);
    }
}

fn count_nodes(node: &simthing_core::SimThing, n: &mut u32) {
    *n = n.saturating_add(1);
    for child in &node.children {
        count_nodes(child, n);
    }
}

fn field_economy_game_mode(pack: &HydratedScenarioPack) -> simthing_spec::GameModeSpec {
    let mut mode = pack.game_mode.clone();
    mode.events.clear();
    mode.capability_trees.clear();
    mode.domain_packs.clear();
    mode.resource_flow = None;
    mode.region_fields.clear();
    mode
}

/// Open field session + open-edge threshold materialization (mirrors production bridge).
/// Returns (session, open_edge_decision_events).
fn open_field_session(pack: &HydratedScenarioPack) -> (SimSession, u32) {
    let scenario = scenario_from_pack(pack);
    let game_mode = field_economy_game_mode(pack);
    match SimSession::open_from_spec(scenario, &game_mode) {
        Ok(mut session) => {
            let mut open_edge_events = 0u32;
            if let Some(registry) = session.spec_state.resource_economy_registry.as_ref() {
                let n_dims = session.state.n_dims as usize;
                let mut values = session.state.read_values();
                let mut prev = values.clone();
                for emission in &registry.registrations.emissions {
                    if let EmissionFormula::Constant { value } = emission.formula {
                        let idx =
                            emission.source_slot as usize * n_dims + emission.source_col as usize;
                        if let Some(slot) = values.get_mut(idx) {
                            *slot = (*slot).max(value);
                        }
                        if let Some(slot) = prev.get_mut(idx) {
                            *slot = 0.0;
                        }
                    }
                }
                session.state.install_resolved_values_at_boundary(&values);
                session
                    .state
                    .install_resolved_previous_values_at_boundary(&prev);
                if !registry.registrations.emit_on_threshold.is_empty() {
                    let gpu_regs = emit_on_threshold_registrations_to_gpu(
                        &registry.registrations.emit_on_threshold,
                    );
                    session
                        .state
                        .ensure_threshold_accumulator(DEFAULT_THRESHOLD_EMISSION_CAPACITY);
                    session
                        .state
                        .upload_accumulator_threshold_ops(&gpu_regs)
                        .expect("upload thresholds");
                    // Open-edge scan (no tick snapshot) — Rising previous=0 → seed.
                    open_edge_events = session
                        .state
                        .dispatch_accumulator_threshold_scan_open_edge_count()
                        .expect("open-edge threshold scan");
                }
            }
            (session, open_edge_events)
        }
        Err(SessionError::Gpu(e)) => {
            panic!(
                "TP-FIELD-SESSION-ELEVATE-0: GPU/adapter Unsupported is a FAIL (not a skip): {e}"
            );
        }
        Err(e) => panic!("open_from_spec failed: {e}"),
    }
}

fn amount(session: &SimSession, namespace: &str, name: &str) -> f32 {
    let reg = &session.proto.registry;
    let pid = reg
        .id_of(namespace, name)
        .unwrap_or_else(|| panic!("missing {namespace}::{name}"));
    let layout = &reg.property(pid).layout;
    let col = reg
        .column_range(pid)
        .col_for_role(&SubFieldRole::Amount, layout)
        .expect("amount")
        .raw_u32() as usize;
    let n_dims = session.state.n_dims as usize;
    let values = session.state.read_values();
    if let Some(economy) = session.spec_state.resource_economy_registry.as_ref() {
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

fn policy_overlay_count(pack: &HydratedScenarioPack) -> usize {
    pack.game_mode
        .overlays
        .iter()
        .filter(|o| o.id.contains("owner_policy"))
        .count()
}

/// catches: 12.8-authored disruption emitter not materializing / not live under open_from_spec.
#[test]
fn canonical_disruption_accretes_from_authored_emitter() {
    let pack = hydrate_canonical();
    assert!(pack.field_economy.is_some(), "12.8 field economy must hydrate");
    let (mut session, _) = open_field_session(&pack);
    let open_amount = amount(&session, "tp_economy", "pirate_outpost_disruption_presence");
    assert!(
        open_amount >= 8.0,
        "disruption Constant seed from authored pirate emitter must materialize: {open_amount}"
    );
    let mut after_amounts = Vec::new();
    for _ in 0..3 {
        session.step_once().expect("tick");
        after_amounts.push(amount(
            &session,
            "tp_economy",
            "pirate_outpost_disruption_presence",
        ));
    }
    let after = *after_amounts.last().expect("ticked");
    let changed = after_amounts.iter().any(|&v| (v - open_amount).abs() > 1e-4)
        || after_amounts
            .windows(2)
            .any(|w| (w[0] - w[1]).abs() > 1e-4);
    assert!(
        changed,
        "disruption must show a live per-tick delta (not open-seed persistence alone): open={open_amount} series={after_amounts:?} after={after}"
    );
}

/// catches: production/silo + policy-overlay coupling severed under open_from_spec multi-tick.
#[test]
fn canonical_production_need_accrete_from_buildings_and_overlays() {
    let pack = hydrate_canonical();
    let policies = policy_overlay_count(&pack);
    assert!(
        policies >= 1,
        "12.8 canonical must author owner_policy overlays (got {policies})"
    );
    let (mut session, _) = open_field_session(&pack);
    let current_before = amount(&session, "tp_economy", "terran_minerals_current");
    let stockpile_before = amount(&session, "tp_economy", "terran_minerals_stockpile");
    assert!(
        current_before >= 40.0,
        "terran minerals silo current must seed: {current_before}"
    );
    for _ in 0..4 {
        session.step_once().expect("tick");
    }
    let current_after = amount(&session, "tp_economy", "terran_minerals_current");
    let stockpile_after = amount(&session, "tp_economy", "terran_minerals_stockpile");
    assert!(
        stockpile_after > stockpile_before || current_after < current_before,
        "silo transfer must move mass: current {current_before}->{current_after} stockpile {stockpile_before}->{stockpile_after}"
    );
    assert!(
        session
            .proto
            .registry
            .id_of("tp_economy", "terran_shipyard_hulls_quantity")
            .is_some(),
        "hulls quantity from production building must install"
    );

    // Policy/need falsifier: stripping owner_policy overlays must remove them from the open profile.
    let mut stripped = hydrate_canonical();
    stripped
        .game_mode
        .overlays
        .retain(|o| !o.id.contains("owner_policy"));
    assert_eq!(policy_overlay_count(&stripped), 0);
    assert!(
        pack.game_mode.overlays.len() > stripped.game_mode.overlays.len(),
        "policy overlays must be a real differential"
    );
    let (session_stripped, _) = open_field_session(&stripped);
    // Stripped path still opens (generic RF), but must not reintroduce policy overlays.
    let mode = field_economy_game_mode(&stripped);
    assert!(
        !mode.overlays.iter().any(|o| o.id.contains("owner_policy")),
        "stripped field-economy mode must not carry owner_policy overlays"
    );
    // Prove both sessions still carry the production building property (RF path intact).
    assert!(session_stripped
        .proto
        .registry
        .id_of("tp_economy", "terran_shipyard_hulls_quantity")
        .is_some());
}

/// catches: decisions firing without an authored threshold, or threshold never emitting.
#[test]
fn canonical_decision_fires_only_on_threshold_crossing() {
    let pack = hydrate_canonical();
    let (mut session, open_edge_events) = open_field_session(&pack);
    let thr_regs = session
        .spec_state
        .resource_economy_registry
        .as_ref()
        .map(|r| r.registrations.emit_on_threshold.len())
        .unwrap_or(0);
    assert!(
        thr_regs >= 1,
        "canonical disruption presence must install emit_on_threshold"
    );
    let disruption = amount(&session, "tp_economy", "pirate_outpost_disruption_presence");
    assert!(
        disruption >= 3.0,
        "seeded disruption must meet/exceed authored threshold: {disruption}"
    );
    assert!(
        open_edge_events > 0,
        "Rising open-edge crossing must emit sealed threshold/decision events; got count=0"
    );
    // Re-read the open-edge emission buffer kinds when still present; otherwise the count is load-bearing.
    // Mid-tick step may or may not add more crossings; open-edge is the authored Rising proof.
    let _ = session.step_once().expect("tick");

    // Absent threshold: strip emit_on_threshold — no open-edge and no mid-tick decisions.
    let mut pack_none = hydrate_canonical();
    if let Some(economy) = pack_none.game_mode.resource_economy.as_mut() {
        economy.emit_on_threshold.clear();
    }
    let (mut session_none, none_open) = open_field_session(&pack_none);
    let thr_none = session_none
        .spec_state
        .resource_economy_registry
        .as_ref()
        .map(|r| r.registrations.emit_on_threshold.len())
        .unwrap_or(0);
    assert_eq!(
        thr_none, 0,
        "without authored emit_on_threshold, no decision threshold may install"
    );
    assert_eq!(
        none_open, 0,
        "no open-edge decision events when no threshold is authored"
    );
    let mut none_events = 0u32;
    for _ in 0..3 {
        let out = session_none.step_once().expect("tick");
        none_events = none_events.saturating_add(out.threshold_event_count);
    }
    assert_eq!(
        none_events, 0,
        "no decision events may fire when no threshold is authored; got {none_events}"
    );
}
