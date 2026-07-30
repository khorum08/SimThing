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
use simthing_driver::{
    allocator_eps_bound, allocator_from_disbursements, check_allocator_step, resolve_node_columns_for_property,
    AllocatorConservationViolation,
};
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

fn canonical_source() -> String {
    std::fs::read_to_string(repo_root().join("scenarios/terran_pirate_galaxy.clause"))
        .expect("read canonical clause")
}

/// RF-5 graduated expansion/minerals regression source: same Clause grammar, restore
/// the minerals need_binding (event_kind 91) while leaving 12.10 manufacturing as profile-only.
fn rf5_expansion_minerals_source(base: &str) -> String {
    const START: &str = "need_binding = terran_manufacturing_need";
    const END: &str = "weight_profile = pirate_disruption_need";
    let start = base
        .find(START)
        .expect("canonical must author manufacturing/hulls construction need_binding");
    let end = base[start..]
        .find(END)
        .map(|rel| start + rel)
        .expect("pirate disruption weight_profile follows construction need_binding");
    let mut out = String::with_capacity(base.len() + 64);
    out.push_str(&base[..start]);
    out.push_str(
        r#"need_binding = terran_expansion_need {
            profile = "expansion-need"
            participant = "terran"
            arena = "studio_recursive_owner_flow"
            input = {
                entity = "terran_shipyard"
                property = "tp_economy::terran_shipyard_minerals_quantity"
                role = Amount
            }
            weight = {
                entity = "terran"
                property = "tp_economy::terran_expansion_weight_stockpile"
                role = Amount
            }
            threshold = 1.0
            event_kind = 91
        }
        "#,
    );
    out.push_str(&base[end..]);
    out
}

fn coefficient_neutralized_source(base: &str) -> String {
    base.replacen("coefficient = 2.0", "coefficient = 0.0", 1)
}

/// Coefficient-neutralized authoring with disruption presence seeded below its Rising
/// threshold so unrelated event_kind 71 must fire while construction 92 cannot.
fn construction_gauge_falsifier_source(base: &str) -> String {
    let neutralized = coefficient_neutralized_source(base);
    let with_low_disruption = neutralized.replacen(
        "amount = 8\n            threshold = 3\n            direction = Rising\n            event_kind = 71",
        "amount = 1\n            threshold = 3\n            direction = Rising\n            event_kind = 71",
        1,
    );
    assert!(
        with_low_disruption.contains("coefficient = 0.0"),
        "falsifier source must neutralize production coefficient"
    );
    assert!(
        with_low_disruption.contains("amount = 1"),
        "falsifier source must seed disruption below Rising threshold"
    );
    with_low_disruption
}

fn sealed_threshold_event_kinds(sim: &mut simthing_driver::SimSession) -> Vec<u32> {
    sim.state
        .accumulator_runtime
        .as_mut()
        .and_then(|runtime| runtime.readback_threshold_events(&sim.state.ctx).ok())
        .map(|events| events.into_iter().map(|event| event.event_kind()).collect())
        .unwrap_or_default()
}

fn coupling_removed_source(base: &str) -> String {
    const COUPLING: &str = r#"        flow_coupling = pirate_raid_suppresses_shipyard {
            source = { location = "terran_shipyard" resource = "hulls" unit_cost = 1.0 }
            pressure = { location = "pirate_outpost" resource = "disruption" unit_cost = 1.0 }
            weight = { owner = "pirate" resource = "disruption_weight" unit_cost = 1.0 }
            sink = { location = "terran_shipyard" resource = "disrupted_hulls" }
            output_coefficient = 1.0
            order_band = 3
        }
"#;
    assert!(base.contains(COUPLING), "canonical must author the coupling block");
    base.replacen(COUPLING, "", 1)
}

fn hydrate_canonical_source(source: &str) -> Result<HydratedScenarioPack, String> {
    let document = parse_raw_document(source.as_bytes()).map_err(|e| e.to_string())?;
    let source_base = repo_root().join("scenarios");
    hydrate_scenario_with_source_base(&document, Some(&source_base)).map_err(|e| e.to_string())
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

fn run_canonical_need_variant(source: &str) -> simthing_mapeditor::StudioLiveSessionBridgeReadout {
    let pack = hydrate_canonical_source(source).expect("hydrate canonical TP need variant");
    let studio = studio_from_pack(&pack);
    let mut bridge = open_field_bridge(&studio);
    assert_eq!(
        bridge.readout().production_path,
        "simthing_driver::SimSession::open_from_spec + step_once"
    );
    let (open_input, open_weight, open_need) = {
        let sim = bridge.sim_session().expect("attached canonical session");
        let binding = sim
            .spec_state
            .resolved_need_bindings
            .first()
            .expect("canonical need binding");
        let values = sim.state.read_values();
        let n_dims = sim.state.n_dims as usize;
        let cell =
            |slot: u32, col: simthing_core::ColumnIndex| values[slot as usize * n_dims + col.raw()];
        (
            cell(binding.inputs[0].slot, binding.inputs[0].col),
            cell(binding.weights[0].slot, binding.weights[0].col),
            cell(binding.participant_slot, binding.need_col),
        )
    };
    assert!(
        open_input > 0.0,
        "authored input may be installed through the generic property path"
    );
    assert_eq!(
        (open_weight, open_need),
        (0.0, 0.0),
        "emission-backed weight and derived need must be zero after open; only ordinary GPU execution may make them live"
    );
    bridge
        .consume_scheduled_ticks(1)
        .expect("one canonical TP production tick");
    let readout = bridge.readout();
    println!(
        "RF-5 LIVE scenario={} open_input={} open_weight={} open_need={} tick={} profile={:?} weights={:?} need={:?} threshold={:?} result={:?} field_policy_events={}",
        readout.scenario_id.as_deref().unwrap_or("--"),
        open_input,
        open_weight,
        open_need,
        readout.executed_ticks,
        readout.recursive_rf.need_profile_id,
        readout.recursive_rf.need_weight_values,
        readout.recursive_rf.need_live_value,
        readout.recursive_rf.need_threshold,
        readout.recursive_rf.need_threshold_result,
        readout.recursive_rf.need_threshold_event_count,
    );
    readout
}

#[derive(Debug)]
struct RecursiveRfObservation {
    loaded_ancestor_aggregate: f32,
    live_ancestor_aggregate: f32,
    ancestor_aggregate: f32,
    ancestor_allocation: f32,
    leaf_allocations: Vec<f32>,
    measured_balance_delta: f32,
    named_child_intrinsic: f32,
}

fn execute_canonical_recursive_rf(
    disable_named_child: bool,
    disconnect_governed_balance: bool,
) -> RecursiveRfObservation {
    let pack = hydrate_canonical();
    let mut profile = authored_live_profile_from_pack(&pack);
    let rf = profile
        .recursive_rf
        .clone()
        .expect("canonical pack must compose an admitted recursive RF profile");
    let named_obligation = profile
        .game_mode
        .resource_flow
        .as_mut()
        .expect("RF spec")
        .base_obligations
        .iter_mut()
        .find(|obligation| obligation.id == "studio_rf_child_0_intrinsic")
        .expect("named child base obligation");
    let named_child_intrinsic = named_obligation.rate;
    if disable_named_child {
        named_obligation.rate = 0.0;
    }
    if disconnect_governed_balance {
        let property = profile
            .game_mode
            .properties
            .iter_mut()
            .find(|property| {
                property.namespace == rf.property_namespace && property.name == rf.property_name
            })
            .expect("RF property");
        property
            .sub_fields
            .iter_mut()
            .find(|subfield| subfield.role == SubFieldRole::Named("balance".into()))
            .expect("Balance subfield")
            .governed_by = None;
    }

    let mut studio = studio_from_pack(&pack);
    studio.authored_live_profile = Some(profile);
    let mut bridge = open_field_bridge(&studio);
    let before = bridge.readout().recursive_rf;
    assert!(before.active, "recursive RF runtime flag must be active");
    assert_eq!(before.sibling_count, 3);
    let balance_before = {
        let sim = bridge.sim_session().expect("attached");
        let property_id = sim
            .proto
            .registry
            .id_of(&rf.property_namespace, &rf.property_name)
            .expect("RF property id");
        let cols =
            resolve_node_columns_for_property(&sim.proto.registry, property_id, &rf.arena)
                .expect("RF columns");
        let balance_col = cols.balance_col.expect("Balance column");
        let owner_slot = sim
            .spec_state
            .arena_registry
            .participant_slot(rf.ancestor_id, 0)
            .expect("ancestor admitted")
            .raw();
        let values = sim.state.read_values();
        values[(owner_slot * sim.state.n_dims + balance_col.raw_u32()) as usize]
    };

    bridge
        .consume_scheduled_ticks(1)
        .expect("ordinary live step_once");
    let live_readout = bridge.readout().recursive_rf;
    let sim = bridge.sim_session().expect("attached");
    let property_id = sim
        .proto
        .registry
        .id_of(&rf.property_namespace, &rf.property_name)
        .expect("RF property id");
    let cols = resolve_node_columns_for_property(&sim.proto.registry, property_id, &rf.arena)
        .expect("RF columns");
    let balance_col = cols.balance_col.expect("Balance column");
    let participant_slot = |hosted_id| {
        sim.spec_state
            .arena_registry
            .participant_slot(hosted_id, 0)
            .expect("participant admitted")
            .raw()
    };
    let owner_slot = participant_slot(rf.ancestor_id);
    let resource_flow = profile_resource_flow(&studio);
    let arena = resource_flow.arenas.first().expect("one RF arena");
    let leaf_ids: Vec<_> = arena
        .explicit_participants
        .iter()
        .filter(|participant| {
            participant.parent_subtree_root_id == Some(rf.ancestor_id.raw() as u64)
        })
        .map(|participant| simthing_core::SimThingId::from_session_raw(participant.subtree_root_id))
        .collect();
    assert_eq!(leaf_ids.len(), 3, "real Owner must have three RF siblings");
    let values = sim.state.read_values();
    let cell = |slot: u32, col: simthing_core::ColumnIndex| {
        values[(slot * sim.state.n_dims + col.raw_u32()) as usize]
    };
    let leaf_allocations = leaf_ids
        .iter()
        .map(|id| {
            let slot = participant_slot(*id);
            cell(slot, cols.allocated_flow_col)
        })
        .collect();
    RecursiveRfObservation {
        loaded_ancestor_aggregate: before
            .ancestor_aggregate_before
            .expect("loaded Owner aggregate readout"),
        live_ancestor_aggregate: live_readout
            .ancestor_aggregate_after
            .expect("live Owner aggregate readout"),
        ancestor_aggregate: cell(owner_slot, cols.intrinsic_flow_sum_col),
        ancestor_allocation: cell(owner_slot, cols.allocated_flow_col),
        leaf_allocations,
        measured_balance_delta: cell(owner_slot, balance_col) - balance_before,
        named_child_intrinsic,
    }
}

fn profile_resource_flow(studio: &StudioSession) -> &simthing_spec::ResourceFlowSpec {
    studio
        .authored_live_profile
        .as_ref()
        .and_then(|profile| profile.game_mode.resource_flow.as_ref())
        .expect("Studio authored RF profile")
}
/// RF-5 load-bearing proof: the canonical authored scalar is the only difference
/// between the pair, and the generic need_binding drives both the live need cell
/// and sealed FIELD_POLICY outcome through the production Studio path.
#[test]
fn canonical_tp_generic_need_binding_live_weight_controls_need_and_field_policy() {
    let high_source = rf5_expansion_minerals_source(&canonical_source());
    assert_eq!(high_source.matches("current = 0.02").count(), 1);
    let low_source = high_source.replacen("current = 0.02", "current = 0.005", 1);

    let high = run_canonical_need_variant(&high_source);
    let low = run_canonical_need_variant(&low_source);
    for readout in [&high, &low] {
        assert_eq!(readout.scenario_id.as_deref(), Some("terran_pirate_galaxy"));
        assert_eq!(readout.executed_ticks, 1);
        assert_eq!(
            readout.recursive_rf.need_profile_id.as_deref(),
            Some("terran_expansion_need")
        );
        assert_eq!(
            readout.recursive_rf.need_profile_kind.as_deref(),
            Some("expansion-need")
        );
        assert_eq!(readout.recursive_rf.need_threshold, Some(1.0));
    }

    let high_need = high
        .recursive_rf
        .need_live_value
        .expect("high live need GPU readout");
    let low_need = low
        .recursive_rf
        .need_live_value
        .expect("low live need GPU readout");
    assert!(
        high_need > low_need && high_need > 1.0 && low_need < 1.0,
        "authored weight only must change actual live need across threshold: high={high_need} low={low_need}"
    );
    assert!(
        high.recursive_rf
            .need_weight_values
            .as_deref()
            .is_some_and(|v| v.contains("terran=0.020000")),
        "Studio must show actual high GPU weight value: {:?}",
        high.recursive_rf.need_weight_values
    );
    assert!(
        low.recursive_rf
            .need_weight_values
            .as_deref()
            .is_some_and(|v| v.contains("terran=0.005000")),
        "Studio must show actual low GPU weight value: {:?}",
        low.recursive_rf.need_weight_values
    );
    assert_eq!(high.recursive_rf.need_threshold_event_count, 1);
    assert_eq!(high.recursive_rf.need_threshold_result, Some("event"));
    assert_eq!(low.recursive_rf.need_threshold_event_count, 0);
    assert_eq!(low.recursive_rf.need_threshold_result, Some("no-event"));
}

/// RF-5 fail-closed proof: neither a missing profile join nor a property typo
/// degrades to an empty/neutral binding.
#[test]
fn canonical_tp_need_binding_removed_or_misbound_fails_closed() {
    let source = rf5_expansion_minerals_source(&canonical_source());
    let missing_profile = source.replacen(
        "weight_profile = terran_expansion_need",
        "weight_profile = terran_expansion_need_removed",
        1,
    );
    let error = hydrate_canonical_source(&missing_profile)
        .expect_err("missing profile join must fail hydrate");
    assert!(
        error.contains("no weight_profile with the same id"),
        "unexpected missing-profile diagnostic: {error}"
    );

    let misbound = source.replacen(
        "tp_economy::terran_expansion_weight_stockpile",
        "tp_economy::terran_expansion_weight_missing",
        1,
    );
    let pack = hydrate_canonical_source(&misbound)
        .expect("semantic typo survives parse/hydrate to admission");
    let studio = studio_from_pack(&pack);
    let mut bridge = StudioLiveSessionBridge::new();
    bridge.set_path_preference(StudioLiveSessionPathPreference::FieldBearing);
    let error = bridge
        .open_from_loaded_studio_session(&studio)
        .expect_err("misbound property must fail production session admission");
    let message = error.to_string();
    assert!(
        message.contains("terran_expansion_weight_missing")
            || message.contains("need binding")
            || message.contains("need_binding"),
        "unexpected misbound-property diagnostic: {message}"
    );
}

#[derive(Debug)]
struct EmergentTensionObservation {
    production_open: f32,
    production_after: f32,
    disruption_open: f32,
    disruption_after: f32,
    suppression_open: f32,
    suppression_after: f32,
    construction_crossings: u32,
    production_path: String,
}

fn run_emergent_tension_variant(source: &str, ticks: u64) -> EmergentTensionObservation {
    // Clause-source only — no post-hydration recipe/registry surgery.
    let pack = hydrate_canonical_source(source).expect("hydrate policy authoring");
    let studio = studio_from_pack(&pack);
    let mut bridge = open_field_bridge(&studio);
    let sim = bridge.sim_session().expect("attached");
    let shipyard = sim
        .scenario
        .install_targets
        .get("terran_shipyard")
        .and_then(|ids| ids.first().copied())
        .expect("terran_shipyard host");
    let outpost = sim
        .scenario
        .install_targets
        .get("pirate_outpost")
        .and_then(|ids| ids.first().copied())
        .expect("pirate_outpost host");
    let session_root = studio
        .authored_live_profile
        .as_ref()
        .and_then(|p| p.recursive_rf.as_ref())
        .map(|rf| rf.session_root_id)
        .expect("recursive RF session root");
    assert_ne!(
        shipyard, outpost,
        "local coupling requires distinct location hosts"
    );
    assert_ne!(
        shipyard, session_root,
        "shipyard must not collapse onto GameSession root"
    );
    assert_ne!(
        outpost, session_root,
        "pirate_outpost must not collapse onto GameSession root"
    );
    let binding = sim
        .spec_state
        .resolved_need_bindings
        .first()
        .expect("construction need binding");
    assert_eq!(binding.id, "terran_manufacturing_need");
    assert_eq!(binding.profile, "manufacturing-need");
    assert_eq!(binding.event_kind, 92);

    let production_open = amount_at_install_target(
        sim,
        "terran_shipyard",
        "tp_economy",
        "terran_shipyard_hulls_quantity",
    );
    let disruption_open = amount_at_install_target(
        sim,
        "pirate_outpost",
        "tp_economy",
        "pirate_outpost_disruption_presence",
    );
    let suppression_open = amount_at_install_target_or_zero(
        sim,
        "terran_shipyard",
        "tp_economy",
        "terran_shipyard_disrupted_hulls_quantity",
    );
    for _ in 0..ticks {
        bridge
            .consume_scheduled_ticks(1)
            .expect("ordinary canonical tick");
    }
    let readout = bridge.readout();
    let construction_crossings = readout.cumulative_construction_crossings as u32;
    let sim = bridge.sim_session().expect("attached");
    println!(
        "TP12_10_DIAG after_hulls={} crossings={} need={:?} weights={:?} thr={:?} last_result={:?} suppression={} hosts={{shipyard:{:?},outpost:{:?},root:{:?}}}",
        amount_at_install_target(
            sim,
            "terran_shipyard",
            "tp_economy",
            "terran_shipyard_hulls_quantity",
        ),
        construction_crossings,
        readout.recursive_rf.need_live_value,
        readout.recursive_rf.need_weight_values,
        readout.recursive_rf.need_threshold,
        readout.recursive_rf.need_threshold_result,
        amount_at_install_target(
            sim,
            "terran_shipyard",
            "tp_economy",
            "terran_shipyard_disrupted_hulls_quantity",
        ),
        shipyard,
        outpost,
        session_root,
    );
    EmergentTensionObservation {
        production_open,
        production_after: amount_at_install_target(
            sim,
            "terran_shipyard",
            "tp_economy",
            "terran_shipyard_hulls_quantity",
        ),
        disruption_open,
        disruption_after: amount_at_install_target(
            sim,
            "pirate_outpost",
            "tp_economy",
            "pirate_outpost_disruption_presence",
        ),
        suppression_open,
        suppression_after: amount_at_install_target_or_zero(
            sim,
            "terran_shipyard",
            "tp_economy",
            "terran_shipyard_disrupted_hulls_quantity",
        ),
        construction_crossings,
        production_path: readout.production_path.to_string(),
    }
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
    amount_at_install_target_or_zero(sim, target_id, namespace, name)
}

/// Coupling-removed Clause variants omit the sink property entirely.
fn amount_at_install_target_or_zero(
    sim: &simthing_driver::SimSession,
    target_id: &str,
    namespace: &str,
    name: &str,
) -> f32 {
    if sim.proto.registry.id_of(namespace, name).is_none() {
        return 0.0;
    }
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
            if emission.source_col.raw_u32() as usize == col {
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
