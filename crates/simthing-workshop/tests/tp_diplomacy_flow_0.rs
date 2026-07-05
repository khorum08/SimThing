//! TP-DIPLOMACY-FLOW-0 — workshop-homed diplomacy RF lanes over generic ClauseThing hydration.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Mutex;

use simthing_clausething::{hydrate_scenario, parse_raw_document, HydratedScenarioPack};
use simthing_core::{DimensionRegistry, SimProperty, SimThing, SimThingKind, SubFieldRole};
use simthing_driver::{
    compile_planet_child_rf_reduce_up_gpu_proof_plan, planet_child_rf_reduce_up_bucket_aggregate_slot,
    planet_child_rf_reduce_up_bucket_cpu_deficit_total, planet_child_rf_reduce_up_bucket_cpu_surplus_total,
    planet_child_rf_reduce_up_bucket_deficit_tick_inputs, planet_child_rf_reduce_up_bucket_surplus_tick_inputs,
    Scenario, SimSession,
};
use simthing_gpu::{
    emit_on_threshold_registrations_to_gpu, set_debug_readback_allowed, GpuContext,
    ThresholdEvent, DEFAULT_THRESHOLD_EMISSION_CAPACITY,
};
use simthing_sim::{execute_accumulator_plan_tick_cpu, SimGpuAccumulatorTickState, SimGpuReadbackPolicy};
use simthing_spec::{
    evaluate_planet_child_rf_reduce_up, evaluate_runtime_rf_tick, is_owner_entity_kind,
    owner_silo_writeback_inputs_from_planet_child_reduce_up, runtime_owner_silo_states_from_scenario,
    SimThingScenarioGrid, SimThingScenarioSpec, SimThingScenarioProvenance, SimThingStructuralGridFrame,
};
use simthing_workshop::{
    apply_diplomacy_post_hydration, BASELINE_BORDER_DISTRUST_SURPLUS, HOSTILITY_COMMITMENT_EVENT_KIND,
    HOSTILITY_DISTRUST_THRESHOLD,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn fixture_json_path() -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-mapeditor/tests/fixtures/tp_base_disc_1500.simthing-scenario.json")
        .to_string_lossy()
        .replace('\\', "/")
}

fn base_clause() -> String {
    format!(
        r#"
scenario = tp_diplomacy_flow_0 {{
    metadata = {{
        display_name = "TP Diplomacy Flow 0"
        runtime_owner = "scenario-container"
    }}
    static_galaxy_scenario = base_disc {{
        namespace = "tp_base"
        source_json = "{}"
        map_quality_status = PASS
    }}
    owner = terran {{
        owner_key = "terran"
        display_name = "Terran Compact"
        archetype = "settler_policy"
    }}
    owner = pirate {{
        owner_key = "pirate"
        display_name = "Pirate Cartel"
        archetype = "raider_policy"
    }}
    ownership_volume = terran_core {{
        owner = "terran"
        count = 200
        selection = chebyshev_contiguous
        seed = 770421
        anchor_row = 199
        anchor_col = 80
    }}
    ownership_volume = pirate_border {{
        owner = "pirate"
        count = 50
        selection = chebyshev_contiguous
        adjacent_to = "terran_core"
        seed = 770421
    }}
    planet_surface_payload = owned_system_payload {{
        applies_to = owned_systems
        planets_per_system_min = 1
        surface_grid = "1x1"
        factory_min = 1
        cohort_min = 1
        category_map = {{
            pop_factory = {{ kind = Cohort depth = 3 }}
        }}
        resource = {{
            id = "tp_minerals"
            namespace = "tp"
            name = "minerals"
            display_name = "Minerals"
        }}
        modifier = {{
            pop_factory_minerals_produces_mult = 0.10
            pop_factory_minerals_upkeep_add = 1
        }}
    }}
    planet_surface_payload = neutral_system_payload {{
        applies_to = neutral_systems
        planets_per_system_min = 1
        surface_grid = "1x1"
        factory_min = 0
        cohort_min = 0
    }}
}}
"#,
        fixture_json_path()
    )
}

fn hydrate_base_pack() -> HydratedScenarioPack {
    let document = parse_raw_document(base_clause().as_bytes()).expect("parse diplomacy base clause");
    hydrate_scenario(&document).expect("hydrate base TP clause without diplomacy")
}

fn scenario_from_pack(pack: &HydratedScenarioPack) -> SimThingScenarioSpec {
    SimThingScenarioSpec {
        scenario_id: pack.scenario_id.clone(),
        root: pack
            .authority_root
            .clone()
            .expect("authority root required for diplomacy proofs"),
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: 0,
                height: 0,
                occupied_cells: 0,
            },
            map_container_id: String::new(),
            placements: Vec::new(),
        },
        links: Vec::new(),
        provenance: SimThingScenarioProvenance::default(),
    }
}

fn require_gpu() -> (GpuContext, std::sync::MutexGuard<'static, ()>) {
    let guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    set_debug_readback_allowed(true);
    let ctx = GpuContext::new_blocking().expect("TP-DIPLOMACY-FLOW-0 requires a real GPU adapter");
    (ctx, guard)
}

fn clone_owner_shell(source: &SimThing) -> SimThing {
    let mut owner = source.clone();
    owner.properties.clear();
    owner.children.clear();
    owner
}

fn threshold_session_root(pack: &HydratedScenarioPack) -> SimThing {
    let authority = pack.authority_root.as_ref().expect("authority root");
    let session = authority
        .children
        .iter()
        .find(|child| child.kind == SimThingKind::GameSession)
        .expect("GameSession child");
    let mut root = SimThing::new(SimThingKind::World, 0);
    for child in &session.children {
        if is_owner_entity_kind(&child.kind) {
            root.add_child(clone_owner_shell(child));
        }
    }
    root
}

fn open_session(pack: &HydratedScenarioPack) -> SimSession {
    let mut registry = DimensionRegistry::new();
    let _ = registry.register(SimProperty::simple("_session", "seed", 0));
    let scenario = Scenario {
        name: "tp_diplomacy_flow_0".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 16,
        registry,
        root: threshold_session_root(pack),
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: pack.install_targets.clone().into_iter().collect(),
    };
    SimSession::open_from_spec(scenario, &pack.game_mode).expect("open diplomacy session")
}

fn run_gpu_threshold_scan(session: &mut SimSession) -> Vec<ThresholdEvent> {
    let mut threshold_session = session
        .state
        .accumulator_runtime
        .as_mut()
        .expect("accumulator runtime")
        .take_threshold_session()
        .expect("threshold registrations uploaded");
    session
        .state
        .dispatch_accumulator_threshold_scan(&mut threshold_session)
        .expect("GPU threshold scan");
    let events = threshold_session
        .readback_threshold_events(&session.state.ctx)
        .expect("threshold readback");
    session
        .state
        .accumulator_runtime
        .as_mut()
        .expect("accumulator runtime")
        .restore_threshold_session(Some(threshold_session));
    events
}

fn sync_threshold_emissions(session: &mut SimSession) {
    let Some(registry) = session.spec_state.resource_economy_registry.as_ref() else {
        return;
    };
    if registry.registrations.emit_on_threshold.is_empty() {
        return;
    }
    let gpu_regs = emit_on_threshold_registrations_to_gpu(&registry.registrations.emit_on_threshold);
    session
        .state
        .ensure_threshold_accumulator(DEFAULT_THRESHOLD_EMISSION_CAPACITY);
    session
        .state
        .upload_accumulator_threshold_ops(&gpu_regs)
        .expect("upload emit_on_threshold registrations");
}

fn set_owner_distrust_crossing(
    session: &mut SimSession,
    distrust_col: u32,
    previous: f32,
    current: f32,
) {
    let n_dims = session.state.n_dims;
    let root_slot = session
        .proto
        .allocator
        .slot_of(session.scenario.root.id)
        .expect("session root slot");
    let idx = (root_slot.raw() * n_dims + distrust_col) as usize;
    let mut current_flat = session.state.read_values();
    current_flat[idx] = current;
    let mut previous_flat = current_flat.clone();
    previous_flat[idx] = previous;
    session
        .state
        .install_resolved_previous_values_at_boundary(&previous_flat);
    session
        .state
        .install_resolved_values_at_boundary(&current_flat);
}

fn owner_distrust_col(session: &SimSession) -> u32 {
    let pid = session
        .proto
        .registry
        .id_of("tp", "owner_distrust")
        .expect("owner_distrust registered");
    session
        .proto
        .registry
        .column_range(pid)
        .col_for_role(
            &SubFieldRole::Amount,
            &session.proto.registry.property(pid).layout,
        )
        .expect("amount column") as u32
}

#[test]
fn workshop_post_hydration_application_is_required() {
    let mut pack = hydrate_base_pack();
    let before = evaluate_planet_child_rf_reduce_up(&scenario_from_pack(&pack)).surplus_total;
    apply_diplomacy_post_hydration(&mut pack).expect("workshop diplomacy apply");
    let after = evaluate_planet_child_rf_reduce_up(&scenario_from_pack(&pack)).surplus_total;
    assert!(
        after > before,
        "distrust seeding must increase reduce-up surplus: before={before} after={after}"
    );
    assert!(
        after - before >= BASELINE_BORDER_DISTRUST_SURPLUS,
        "expected at least one baseline distrust stamp: delta={}",
        after - before
    );
}

#[test]
fn influence_round_trip_reduces_to_owner() {
    let mut pack = hydrate_base_pack();
    apply_diplomacy_post_hydration(&mut pack).expect("workshop diplomacy apply");
    let spec = scenario_from_pack(&pack);
    let tick = evaluate_runtime_rf_tick(&spec).expect("runtime RF tick");
    assert!(tick.owner_silo_writeback_ready, "writeback must be ready");
    assert!(!tick.writeback_results.is_empty(), "writeback results required");
    let owner_refs: BTreeMap<_, _> = tick
        .writeback_results
        .iter()
        .map(|row| (row.owner_ref.as_str(), row.applied_surplus))
        .collect();
    assert!(
        owner_refs.get("terran").copied().unwrap_or(0) > 0
            || owner_refs.get("pirate").copied().unwrap_or(0) > 0,
        "distrust must reduce up to at least one owner silo: {owner_refs:?}"
    );
    assert_eq!(tick.disburse_allocated_total, 0, "no hand-copy owner writes in this proof");
}

#[test]
fn distrust_threshold_emits_hostility_commitment() {
    let mut pack = hydrate_base_pack();
    apply_diplomacy_post_hydration(&mut pack).expect("workshop diplomacy apply");
    let spec = scenario_from_pack(&pack);
    let reduce_up = evaluate_planet_child_rf_reduce_up(&spec);
    let inputs =
        owner_silo_writeback_inputs_from_planet_child_reduce_up(&reduce_up).expect("writeback inputs");
    let initial = runtime_owner_silo_states_from_scenario(&spec).expect("initial silo states");
    let writeback = simthing_spec::apply_owner_silo_runtime_writeback_cpu(&initial, &inputs)
        .expect("writeback cpu");
    let max_next = writeback
        .iter()
        .map(|row| row.next_current)
        .max()
        .unwrap_or(0);
    assert!(
        max_next as f32 >= HOSTILITY_DISTRUST_THRESHOLD,
        "seeded distrust must cross hostility threshold after reduce-up/writeback: max_next={max_next}"
    );

    let mut session = open_session(&pack);
    sync_threshold_emissions(&mut session);

    let distrust_col = owner_distrust_col(&session);

    set_owner_distrust_crossing(
        &mut session,
        distrust_col,
        HOSTILITY_DISTRUST_THRESHOLD - 2.0,
        HOSTILITY_DISTRUST_THRESHOLD - 1.0,
    );
    let pre = run_gpu_threshold_scan(&mut session);
    assert!(
        !pre
            .iter()
            .any(|event| event.event_kind() == HOSTILITY_COMMITMENT_EVENT_KIND),
        "sub-threshold distrust must not emit hostility yet"
    );

    set_owner_distrust_crossing(
        &mut session,
        distrust_col,
        HOSTILITY_DISTRUST_THRESHOLD - 1.0,
        max_next as f32,
    );
    let events = run_gpu_threshold_scan(&mut session);
    assert!(
        events
            .iter()
            .any(|event| event.event_kind() == HOSTILITY_COMMITMENT_EVENT_KIND),
        "hostility commitment must emit via threshold crossing: events={events:?}"
    );
}

#[test]
fn trust_distrust_gpu_matches_cpu_oracle() {
    let (_ctx, _guard) = require_gpu();
    let mut pack = hydrate_base_pack();
    apply_diplomacy_post_hydration(&mut pack).expect("workshop diplomacy apply");
    let spec = scenario_from_pack(&pack);
    let plan = compile_planet_child_rf_reduce_up_gpu_proof_plan(&spec).expect("compile reduce-up plan");
    let bucket = plan
        .bucket_plans
        .first()
        .expect("at least one distrust reduce-up bucket");
    let aggregate = planet_child_rf_reduce_up_bucket_aggregate_slot(bucket);
    let surplus_inputs = planet_child_rf_reduce_up_bucket_surplus_tick_inputs(&plan, bucket);
    let deficit_inputs = planet_child_rf_reduce_up_bucket_deficit_tick_inputs(&plan, bucket);

    let surplus_cpu =
        execute_accumulator_plan_tick_cpu(&bucket.surplus_plan, &surplus_inputs).expect("surplus cpu");
    let deficit_cpu =
        execute_accumulator_plan_tick_cpu(&bucket.deficit_plan, &deficit_inputs).expect("deficit cpu");

    let ctx = GpuContext::new_blocking().expect("gpu for reduce-up parity");
    let mut surplus_state =
        SimGpuAccumulatorTickState::new(&ctx, bucket.surplus_plan.clone()).expect("surplus gpu init");
    let surplus_gpu = surplus_state
        .tick(&ctx, &surplus_inputs, SimGpuReadbackPolicy::ProofReadback)
        .expect("surplus gpu tick")
        .expect("surplus gpu readback");
    let mut deficit_state =
        SimGpuAccumulatorTickState::new(&ctx, bucket.deficit_plan.clone()).expect("deficit gpu init");
    let deficit_gpu = deficit_state
        .tick(&ctx, &deficit_inputs, SimGpuReadbackPolicy::ProofReadback)
        .expect("deficit gpu tick")
        .expect("deficit gpu readback");

    let cpu_surplus_total = planet_child_rf_reduce_up_bucket_cpu_surplus_total(&plan, bucket);
    let cpu_deficit_total = planet_child_rf_reduce_up_bucket_cpu_deficit_total(&plan, bucket);

    assert_eq!(surplus_cpu[aggregate], cpu_surplus_total as f32);
    assert_eq!(deficit_cpu[aggregate], cpu_deficit_total as f32);
    assert_eq!(surplus_cpu[aggregate], surplus_gpu[aggregate]);
    assert_eq!(deficit_cpu[aggregate], deficit_gpu[aggregate]);
}