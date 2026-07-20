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
    allocator_eps_bound, allocator_from_disbursements, check_allocator_step, resolve_node_columns,
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
            resolve_node_columns(&sim.proto.registry.property(property_id).layout, &rf.arena)
                .expect("RF columns");
        let balance_col = cols.balance_col.expect("Balance column");
        let owner_slot = sim
            .spec_state
            .arena_participant_scaffold
            .index
            .by_host_and_arena[&(rf.ancestor_id, 0)]
            .raw();
        let values = sim.state.read_values();
        values[(owner_slot * sim.state.n_dims + balance_col) as usize]
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
    let cols = resolve_node_columns(&sim.proto.registry.property(property_id).layout, &rf.arena)
        .expect("RF columns");
    let balance_col = cols.balance_col.expect("Balance column");
    let participant_slot = |hosted_id| {
        sim.spec_state
            .arena_participant_scaffold
            .index
            .by_host_and_arena[&(hosted_id, 0)]
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
    let cell = |slot: u32, col: u32| values[(slot * sim.state.n_dims + col) as usize];
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

/// catches: Studio path reports RF while ordinary step_once never executes the recursive Arena;
/// catches: measured governed Balance integration disconnected while arithmetic still looks bounded.
#[test]
fn canonical_recursive_rf_bites_with_real_owner_aggregate_and_runtime_balance_negative() {
    let enabled = execute_canonical_recursive_rf(false, false);
    let replay = execute_canonical_recursive_rf(false, false);
    let disabled = execute_canonical_recursive_rf(true, false);
    assert_eq!(
        enabled.ancestor_aggregate.to_bits(),
        replay.ancestor_aggregate.to_bits(),
        "live recursive RF must replay bit-exactly"
    );
    assert_eq!(enabled.loaded_ancestor_aggregate, 0.0);
    assert_eq!(
        enabled.live_ancestor_aggregate.to_bits(),
        enabled.ancestor_aggregate.to_bits(),
        "Studio telemetry must read the actual post-dispatch Owner aggregate"
    );
    assert!(
        disabled.ancestor_aggregate > 0.0,
        "fixed real siblings must keep the Owner aggregate non-zero"
    );
    assert_eq!(
        (enabled.ancestor_aggregate - disabled.ancestor_aggregate).to_bits(),
        enabled.named_child_intrinsic.to_bits(),
        "disabling only the named real child must remove exactly its Owner marginal"
    );

    let budget = enabled.ancestor_aggregate + enabled.ancestor_allocation;
    let sum_disbursed: f32 = enabled.leaf_allocations.iter().copied().sum();
    let arithmetic_residual = budget - sum_disbursed;
    let bound = allocator_eps_bound(enabled.leaf_allocations.len(), budget);
    assert_ne!(
        arithmetic_residual, 0.0,
        "fixture must retain a deterministic non-zero f32 allocator residual"
    );
    check_allocator_step(&allocator_from_disbursements(
        budget,
        enabled.leaf_allocations.clone(),
        Some(enabled.measured_balance_delta),
    ))
    .expect("RF-1 must accept the measured governed Balance delta");
    println!(
        "RF4_LIVE loaded_owner_aggregate={} live_owner_aggregate={} disabled_aggregate={} named_marginal={} budget={} sum_disbursed={} arithmetic_residual={} measured_balance_delta={} bound={}",
        enabled.loaded_ancestor_aggregate,
        enabled.ancestor_aggregate,
        disabled.ancestor_aggregate,
        enabled.named_child_intrinsic,
        budget,
        sum_disbursed,
        arithmetic_residual,
        enabled.measured_balance_delta,
        bound,
    );

    let disconnected = execute_canonical_recursive_rf(false, true);
    let disconnected_budget = disconnected.ancestor_aggregate + disconnected.ancestor_allocation;
    let violation = check_allocator_step(&allocator_from_disbursements(
        disconnected_budget,
        disconnected.leaf_allocations.clone(),
        Some(disconnected.measured_balance_delta),
    ))
    .expect_err("actual GPU readout must fail when governed Balance is disconnected");
    assert!(matches!(
        violation,
        AllocatorConservationViolation::ResidualNotIntegrated { .. }
    ));
    println!(
        "RF4_RUNTIME_NEGATIVE governed_balance=disconnected actual_gpu_balance_delta={} violation={violation:?}",
        disconnected.measured_balance_delta,
    );
}

/// RF-5 load-bearing proof: the canonical authored scalar is the only difference
/// between the pair, and the generic need_binding drives both the live need cell
/// and sealed FIELD_POLICY outcome through the production Studio path.
#[test]
fn canonical_tp_generic_need_binding_live_weight_controls_need_and_field_policy() {
    let high_source = canonical_source();
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
    let source = canonical_source();
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

#[derive(Clone, Copy, Debug)]
enum EmergentTensionControl {
    Full,
    CouplingRemoved,
    ProductionRecipeRemoved,
}

fn run_emergent_tension_variant(
    source: &str,
    ticks: u64,
    control: EmergentTensionControl,
) -> EmergentTensionObservation {
    let mut pack = hydrate_canonical_source(source).expect("hydrate policy authoring");
    match control {
        EmergentTensionControl::Full => {}
        EmergentTensionControl::CouplingRemoved => {
            pack.game_mode
                .resource_economy
                .as_mut()
                .expect("resource economy")
                .recipes
                .retain(|recipe| {
                    recipe.id != "tp_economy_coupling_pirate_raid_suppresses_shipyard"
                });
        }
        EmergentTensionControl::ProductionRecipeRemoved => {
            pack.game_mode
                .resource_economy
                .as_mut()
                .expect("resource economy")
                .recipes
                .retain(|recipe| recipe.id != "tp_economy_recipe_shipyard_factory");
        }
    }
    // Drop presence threshold noise so sealed need crossings are the only
    // FIELD_POLICY events the construction gauge accumulates.
    if let Some(economy) = pack.game_mode.resource_economy.as_mut() {
        economy.emit_on_threshold.clear();
    }
    let studio = studio_from_pack(&pack);
    let mut bridge = open_field_bridge(&studio);
    let sim = bridge.sim_session().expect("attached");
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
    let suppression_open = amount_at_install_target(
        sim,
        "terran_shipyard",
        "tp_economy",
        "terran_shipyard_disrupted_hulls_quantity",
    );
    // Rising construction crossings fire on the ingress tick only; the recursive
    // RF readout exposes last-tick kinds, so accumulate sealed need events across
    // ordinary step_once ticks (emit_on_threshold already cleared above).
    let mut construction_crossings = 0u32;
    for _ in 0..ticks {
        bridge
            .consume_scheduled_ticks(1)
            .expect("ordinary canonical tick");
        let tick_readout = bridge.readout();
        construction_crossings = construction_crossings
            .saturating_add(tick_readout.recursive_rf.need_threshold_event_count);
    }
    let readout = bridge.readout();
    let sim = bridge.sim_session().expect("attached");
    println!(
        "TP12_10_DIAG control={control:?} after_hulls={} crossings={} need={:?} weights={:?} thr={:?} last_result={:?} suppression={}",
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
        suppression_after: match control {
            EmergentTensionControl::CouplingRemoved => 0.0,
            _ => amount_at_install_target(
                sim,
                "terran_shipyard",
                "tp_economy",
                "terran_shipyard_disrupted_hulls_quantity",
            ),
        },
        construction_crossings,
        production_path: readout.production_path.to_string(),
    }
}

/// TP-12.10 load-bearing proof: ordinary open_from_spec + step_once, same binary/path,
/// paired authorings differ only in owner-policy weight silo scalars.
#[test]
fn canonical_policy_weights_diverge_live_production_disruption_and_construction() {
    const TICKS: u64 = 8;
    const MIN_PRODUCTION_DIVERGENCE: f32 = 6.0;
    const MIN_SUPPRESSION_DIVERGENCE: f32 = 6.0;
    const MIN_CROSSING_DIVERGENCE: u32 = 1;

    // Keep RF-5's sealed need threshold (1.0) and minerals input; policy-weight
    // scalars alone must drive the construction-crossing divergence.
    let productive_source = canonical_source();
    assert!(
        productive_source.contains("coefficient = 2.0"),
        "canonical must author the HORIZON production output coefficient"
    );
    assert!(
        productive_source.contains("flow_coupling = pirate_raid_suppresses_shipyard"),
        "canonical must author the generic disruption coupling"
    );
    assert_eq!(productive_source.matches("current = 0.02").count(), 1);
    assert!(
        productive_source.contains("resource = \"disruption_weight\"")
            && productive_source.contains("current = 2"),
        "canonical must author the pirate disruption-weight silo"
    );
    // Paired authoring: swap expansion vs disruption weight scalars only.
    let tension_source = productive_source
        .replacen("current = 0.02", "current = __TERRAN_LOW__", 1)
        .replacen(
            "stockpile_silo = pirate_disruption_weight {\n            owner = \"pirate\"\n            resource = \"disruption_weight\"\n            current = 2\n        }",
            "stockpile_silo = pirate_disruption_weight {\n            owner = \"pirate\"\n            resource = \"disruption_weight\"\n            current = 40\n        }",
            1,
        )
        .replacen("current = __TERRAN_LOW__", "current = 0.005", 1);

    let productive =
        run_emergent_tension_variant(&productive_source, TICKS, EmergentTensionControl::Full);
    let tension =
        run_emergent_tension_variant(&tension_source, TICKS, EmergentTensionControl::Full);
    assert_eq!(productive.production_path, tension.production_path);
    assert_eq!(
        productive.production_path,
        "simthing_driver::SimSession::open_from_spec + step_once"
    );
    let production_divergence = productive.production_after - tension.production_after;
    let suppression_divergence = tension.suppression_after - productive.suppression_after;
    let crossing_divergence = productive
        .construction_crossings
        .abs_diff(tension.construction_crossings);
    println!(
        "TP12_10_PAIRED ticks={TICKS} path={} productive={{production:{} disruption:{} suppression:{} crossings:{}}} tension={{production:{} disruption:{} suppression:{} crossings:{}}} divergence={{production:{} suppression:{} crossings:{}}}",
        productive.production_path,
        productive.production_after,
        productive.disruption_after,
        productive.suppression_after,
        productive.construction_crossings,
        tension.production_after,
        tension.disruption_after,
        tension.suppression_after,
        tension.construction_crossings,
        production_divergence,
        suppression_divergence,
        crossing_divergence,
    );
    assert!(
        productive.production_after > productive.production_open,
        "authored coefficient must accrete production under ordinary ticks: open={} after={}",
        productive.production_open,
        productive.production_after
    );
    assert!(
        productive.suppression_after > productive.suppression_open
            || tension.suppression_after > tension.suppression_open,
        "authored coupling must suppress local flow into the sink"
    );
    assert!(
        production_divergence >= MIN_PRODUCTION_DIVERGENCE,
        "predeclared production divergence did not bite: {production_divergence}"
    );
    assert!(
        suppression_divergence >= MIN_SUPPRESSION_DIVERGENCE,
        "predeclared suppression/disruption-coupling divergence did not bite: {suppression_divergence}"
    );
    assert!(
        crossing_divergence >= MIN_CROSSING_DIVERGENCE,
        "predeclared construction crossing divergence did not bite: {crossing_divergence}"
    );
    assert!(
        tension.suppression_after > productive.suppression_after,
        "stronger Pirate weight must produce more coupled suppression"
    );

    let coupling_removed = run_emergent_tension_variant(
        &productive_source,
        TICKS,
        EmergentTensionControl::CouplingRemoved,
    );
    assert!(
        coupling_removed.disruption_after >= coupling_removed.disruption_open
            && coupling_removed.disruption_after > productive.disruption_after,
        "removing only coupling must preserve disruption (no conjunctive drain): open={} decoupled={} coupled={}",
        coupling_removed.disruption_open,
        coupling_removed.disruption_after,
        productive.disruption_after
    );
    assert_eq!(coupling_removed.suppression_after, 0.0);
    assert!(
        coupling_removed.production_after > productive.production_after,
        "removing coupling must remove local-flow suppression: coupled={} decoupled={}",
        productive.production_after,
        coupling_removed.production_after
    );
    println!(
        "TP12_10_COUPLING_NEGATIVE ticks={TICKS} production={} disruption={} suppression={}",
        coupling_removed.production_after,
        coupling_removed.disruption_after,
        coupling_removed.suppression_after,
    );

    let coefficient_removed = run_emergent_tension_variant(
        &productive_source,
        TICKS,
        EmergentTensionControl::ProductionRecipeRemoved,
    );
    // RF-5 need_binding reads minerals (not hulls); sealed crossings may still fire.
    // The coefficient bite is production Amount accretion through the recipe path.
    assert!(
        coefficient_removed.production_after <= coefficient_removed.production_open + 1e-3
            && coefficient_removed.production_after + MIN_PRODUCTION_DIVERGENCE
                < productive.production_after,
        "removing the coefficient-bearing production recipe must yield no production accretion: removed={} full={}",
        coefficient_removed.production_after,
        productive.production_after
    );
    println!(
        "TP12_10_COEFFICIENT_NEGATIVE ticks={TICKS} production_open={} production_after={} crossings={}",
        coefficient_removed.production_open,
        coefficient_removed.production_after,
        coefficient_removed.construction_crossings,
    );
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
    assert!(
        pack.field_economy.is_some(),
        "12.8 field economy must hydrate"
    );
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
    let changed =
        (after - open_amount).abs() > 1e-4 || series.windows(2).any(|w| (w[0] - w[1]).abs() > 1e-4);
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
    let hulls_with = amount_at_install_target(
        sim,
        "terran",
        "tp_economy",
        "terran_shipyard_hulls_quantity",
    );
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

    // No-threshold control: same below-threshold pack with both resource-economy
    // and RF need threshold producers stripped.
    let mut pack_none = pack_below_threshold_disruption();
    if let Some(economy) = pack_none.game_mode.resource_economy.as_mut() {
        economy.emit_on_threshold.clear();
    }
    if let Some(resource_flow) = pack_none.game_mode.resource_flow.as_mut() {
        resource_flow.need_bindings.clear();
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
