//! RF-NEED-WEIGHT-PROFILE-TRANSPORT-0 (RF-5) remand-3 proofs.
//!
//! Ontology: resolve each source via existing property owner; EvalEML on that
//! source row; write need only to admitted participant AllocatorWeight.
//! No property invent, no same-col participant mirror, no global overlay ADR break.
//!
//! Canonical bare TP: production compose → typed AdmissionGap (BLOCKED transport).
//! Neutral: complete explicit companion bindings + open_from_spec + step_once.

use std::collections::HashMap;
use std::path::PathBuf;

use simthing_clausething::{
    compose_need_weight_bindings, hydrate_scenario, parse_raw_document, HydratedScenarioPack,
    NeedWeightComposeOutcome,
};
use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, DimensionRegistry, LogTier,
    SimThing, SimThingId, SimThingKind, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{
    binding_from_hydrated_stack, register_post_rf_need_threshold_rescan, Scenario, SimSession,
};
use simthing_gpu::{EmissionFormula, SlotAllocator};
use simthing_spec::{
    compile_property, ArenaSpec, BaseFlowDirectionSpec, BaseFlowObligationSpec,
    ExplicitParticipantSpec, FissionPolicySpec, InstallTargetSpec, NeedWeightProfileBindingSpec,
    NeedWeightProfileThresholdSpec, PropertyKey, PropertySpec, ResourceEconomyOptInMode,
    ResourceEconomySpec, ResourceFlowExecutionProfile, ResourceFlowOptInMode, ResourceFlowSpec,
};

const ARENA: &str = "foundry";
const NEED_EVENT_KIND: u32 = 91;
const ORDINARY_EVENT_KIND: u32 = 77;
const NEED_THRESHOLD: f32 = 0.5;

/// Neutral low: weight via stockpile **current** (authored emission seed), not overlay.
const NEUTRAL_LOW: &str = r#"
scenario = foundry_valley {
    metadata = { display_name = "Foundry Valley Low" }
    owner = guild {
        owner_key = "guild"
        display_name = "Guild"
        archetype = "industrial"
    }
    location = ridge { display_name = "Ridge" }
    location = basin { display_name = "Basin" }
    field_economy = valley_economy {
        namespace = "forge"
        stockpile_silo = guild_ore {
            owner = "guild"
            resource = "ore"
            current = 1
        }
        stockpile_silo = guild_weight_store {
            owner = "guild"
            resource = "weight_token"
            current = 0.2
        }
        production_building = ridge_foundry {
            location = "ridge"
            input = { resource = "ore" amount = 1 }
            output = { resource = "tools" }
            throttle_hint_max_per_tick = 1
        }
        field_resource_quantity = ridge_ore {
            location = "ridge"
            resource = "ore"
            amount = 1
        }
        # Ordinary threshold that WILL fire (amount 0 → seed 1 crosses 0.5 Rising).
        disruption_presence = basin_smoke {
            location = "basin"
            resource = "smoke"
            amount = 0
            threshold = 0.5
            direction = Rising
            event_kind = 77
        }
        weight_profile = expansion_need {
            profile = "expansion-need"
            input = { input_col = 0 weight_col = 10 }
            output_col = 12
        }
    }
}
"#;

const NEUTRAL_HIGH: &str = r#"
scenario = foundry_valley {
    metadata = { display_name = "Foundry Valley High" }
    owner = guild {
        owner_key = "guild"
        display_name = "Guild"
        archetype = "industrial"
    }
    location = ridge { display_name = "Ridge" }
    location = basin { display_name = "Basin" }
    field_economy = valley_economy {
        namespace = "forge"
        stockpile_silo = guild_ore {
            owner = "guild"
            resource = "ore"
            current = 1
        }
        stockpile_silo = guild_weight_store {
            owner = "guild"
            resource = "weight_token"
            current = 3.0
        }
        production_building = ridge_foundry {
            location = "ridge"
            input = { resource = "ore" amount = 1 }
            output = { resource = "tools" }
            throttle_hint_max_per_tick = 1
        }
        field_resource_quantity = ridge_ore {
            location = "ridge"
            resource = "ore"
            amount = 1
        }
        disruption_presence = basin_smoke {
            location = "basin"
            resource = "smoke"
            amount = 0
            threshold = 0.5
            direction = Rising
            event_kind = 77
        }
        weight_profile = expansion_need {
            profile = "expansion-need"
            input = { input_col = 0 weight_col = 10 }
            output_col = 12
        }
    }
}
"#;

fn flow_property_spec() -> PropertySpec {
    PropertySpec {
        id: "foundry_flow".into(),
        namespace: "workshop".into(),
        name: "foundry_flow".into(),
        display_name: String::new(),
        description: String::new(),
        sub_fields: vec![
            sub("flow", AccumulatorRole::IntrinsicFlow),
            sub(
                "allocated",
                AccumulatorRole::AllocatedFlow {
                    arena: ARENA.into(),
                },
            ),
            sub(
                "weight",
                AccumulatorRole::AllocatorWeight {
                    arena: ARENA.into(),
                },
            ),
            SubFieldSpec {
                role: SubFieldRole::Named("balance_rate".into()),
                width: 1,
                clamp: ClampBehavior::Unbounded,
                velocity_max: None,
                default: 0.0,
                display_name: "balance_rate".into(),
                display_range: None,
                governed_by: None,
                reduction_override: None,
                soft_aggregate_guard: None,
                accumulator_spec: None,
            },
            SubFieldSpec {
                role: SubFieldRole::Named("balance".into()),
                width: 1,
                clamp: ClampBehavior::Unbounded,
                velocity_max: None,
                default: 0.0,
                display_name: "balance".into(),
                display_range: None,
                governed_by: Some(SubFieldRole::Named("balance_rate".into())),
                reduction_override: None,
                soft_aggregate_guard: None,
                accumulator_spec: Some(AccumulatorSpec {
                    role: AccumulatorRole::Balance(BalanceSpec::default()),
                    log_tier: LogTier::Summary,
                }),
            },
        ],
    }
}

fn sub(name: &str, role: AccumulatorRole) -> SubFieldSpec {
    SubFieldSpec {
        role: SubFieldRole::Named(name.into()),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: name.into(),
        display_range: None,
        governed_by: None,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: Some(AccumulatorSpec {
            role,
            log_tier: LogTier::Summary,
        }),
    }
}

fn explicit_neutral_binding() -> NeedWeightProfileBindingSpec {
    binding_from_hydrated_stack(
        "expansion_need",
        "expansion-need",
        simthing_spec::EmlGadgetStackSpec { gadgets: vec![] },
        ARENA,
        InstallTargetSpec::ScenarioListed {
            target_id: "guild".into(),
        },
        vec![PropertyKey::new("forge", "guild_ore_current")],
        vec![PropertyKey::new("forge", "guild_weight_token_current")],
        Some(NeedWeightProfileThresholdSpec {
            threshold: NEED_THRESHOLD,
            event_kind: NEED_EVENT_KIND,
        }),
    )
}

fn compose_or_err(
    pack: &HydratedScenarioPack,
    authored: &[NeedWeightProfileBindingSpec],
) -> Result<Vec<NeedWeightProfileBindingSpec>, String> {
    match compose_need_weight_bindings(pack, ARENA, authored) {
        NeedWeightComposeOutcome::Bindings(b) => Ok(b),
        NeedWeightComposeOutcome::AdmissionGap {
            reason,
            missing_fields,
            weight_profile_ids,
        } => Err(format!(
            "admission_gap: {reason} missing={missing_fields:?} profiles={weight_profile_ids:?}"
        )),
    }
}

fn open_from_pack(
    pack: &HydratedScenarioPack,
    authored: Vec<NeedWeightProfileBindingSpec>,
    misbind: bool,
) -> Result<SimSession, String> {
    let mut authored = authored;
    if misbind {
        for b in &mut authored {
            b.install = InstallTargetSpec::ScenarioListed {
                target_id: "missing_owner".into(),
            };
        }
    }
    let bindings = compose_or_err(pack, &authored)?;

    let mut registry = DimensionRegistry::new();
    compile_property(&flow_property_spec(), &mut registry).expect("seed flow");

    let mut root = SimThing::new(SimThingKind::World, 0);
    let session_root = SimThing::new(SimThingKind::Cohort, 0);
    let owner = SimThing::new(SimThingKind::Cohort, 0);
    let child = SimThing::new(SimThingKind::Cohort, 0);
    let ids = [session_root.id, owner.id, child.id];
    root.add_child(session_root);
    root.add_child(owner);
    root.add_child(child);

    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let slot = |id: SimThingId| allocator.slot_of(id).expect("hosted").raw();

    let mut install_targets = HashMap::new();
    install_targets.insert("guild".into(), vec![ids[1]]);
    install_targets.insert("root".into(), vec![ids[0]]);
    for o in &pack.owners {
        install_targets
            .entry(o.owner_key.clone())
            .or_insert_with(|| vec![ids[1]]);
    }
    if let Some(economy) = pack.field_economy.as_ref() {
        for q in &economy.field_resource_quantities {
            install_targets
                .entry(q.location.clone())
                .or_insert_with(|| vec![ids[0]]);
        }
        for p in &economy.disruption_presences {
            install_targets
                .entry(p.location.clone())
                .or_insert_with(|| vec![ids[0]]);
        }
    }

    let mut game_mode = pack.game_mode.clone();
    game_mode.properties.retain(|p| {
        p.namespace == "forge" || p.namespace == "workshop" || p.namespace == "field_economy"
    });
    game_mode.domain_packs.clear();
    game_mode.capability_trees.clear();
    game_mode.events.clear();
    game_mode.region_fields.clear();
    // Do not install game_mode.overlays (ADR deferred). Weights come from silo current.
    game_mode.overlays.clear();

    let retained: std::collections::HashSet<(String, String)> = game_mode
        .properties
        .iter()
        .map(|p| (p.namespace.clone(), p.name.clone()))
        .collect();
    if let Some(econ) = game_mode.resource_economy.as_mut() {
        econ.opt_in_mode = ResourceEconomyOptInMode::TransferAndEmission;
        econ.emissions
            .retain(|e| retained.contains(&(e.source.namespace.clone(), e.source.name.clone())));
        econ.transfers.clear();
        econ.recipes.clear();
        econ.emit_on_threshold.retain(|e| {
            retained.contains(&(e.source.namespace.clone(), e.source.name.clone()))
        });
    } else {
        game_mode.resource_economy = Some(ResourceEconomySpec {
            opt_in_mode: ResourceEconomyOptInMode::TransferAndEmission,
            ..Default::default()
        });
    }

    game_mode.resource_flow = Some(ResourceFlowSpec {
        opt_in_mode: ResourceFlowOptInMode::Disabled,
        arenas: vec![ArenaSpec {
            name: ARENA.into(),
            flow_property: PropertyKey::new("workshop", "foundry_flow"),
            balance_property: Some(PropertyKey::new("workshop", "foundry_flow")),
            max_participants: 8,
            max_coupling_fanout: 4,
            max_orderband_depth: 16,
            fission_policy: FissionPolicySpec::Reject,
            reserved_orderband_depth: 0,
            reserved_gap_per_intermediate: 0,
            expected_max_children_per_intermediate: 0,
            explicit_participants: vec![
                ExplicitParticipantSpec::flat(slot(ids[0]), ids[0].raw()),
                ExplicitParticipantSpec::nested(slot(ids[1]), ids[1].raw(), ids[0].raw() as u64),
                ExplicitParticipantSpec::nested(slot(ids[2]), ids[2].raw(), ids[1].raw() as u64),
            ],
            enrollment: None,
            wildcard_admission: None,
        }],
        base_obligations: vec![BaseFlowObligationSpec {
            id: "root_budget".into(),
            arena: ARENA.into(),
            install: InstallTargetSpec::ScenarioListed {
                target_id: "root".into(),
            },
            direction: BaseFlowDirectionSpec::Produce,
            rate: 10.0,
        }],
        need_weight_profiles: bindings,
        ..Default::default()
    });
    game_mode.resource_flow_execution_profile =
        ResourceFlowExecutionProfile::RecursiveArenaResourceFlow;

    let scenario = Scenario {
        name: "rf5_need_transport".into(),
        ticks_per_day: 8,
        max_days: 1,
        dt: 1.0,
        n_slots: 64,
        registry,
        root,
        shadow_seeds: vec![],
        tick_patches: vec![],
        install_targets,
    };
    SimSession::open_from_spec(scenario, &game_mode).map_err(|e| format!("{e}"))
}

fn open_neutral(clause: &str) -> SimSession {
    let document = parse_raw_document(clause.as_bytes()).expect("parse");
    let pack = hydrate_scenario(&document).expect("hydrate");
    match open_from_pack(&pack, vec![explicit_neutral_binding()], false) {
        Ok(s) => s,
        Err(e) if e.to_lowercase().contains("adapter") || e.to_lowercase().contains("gpu") => {
            panic!("RF-5: GPU/adapter Unsupported is FAIL: {e}");
        }
        Err(e) => panic!("open: {e}"),
    }
}

fn materialize_emission_seeds(session: &mut SimSession) {
    let Some(registry) = session.spec_state.resource_economy_registry.as_ref() else {
        return;
    };
    let n_dims = session.state.n_dims as usize;
    let mut values = session.state.read_values();
    let mut prev = values.clone();
    let mut any = false;
    for emission in &registry.registrations.emissions {
        let EmissionFormula::Constant { value } = emission.formula else {
            continue;
        };
        let idx = emission.source_slot as usize * n_dims + emission.source_col as usize;
        if let Some(v) = values.get_mut(idx) {
            *v = (*v).max(value);
            any = true;
        }
        if let Some(v) = prev.get_mut(idx) {
            *v = 0.0;
        }
    }
    // Ordinary disruption: seed current amount above thr so Rising fires once.
    if let Some(registry) = session.spec_state.resource_economy_registry.as_ref() {
        for reg in &registry.registrations.emit_on_threshold {
            if reg.event_kind == ORDINARY_EVENT_KIND {
                let idx = reg.slot.raw() as usize * n_dims + reg.col.raw();
                if let Some(v) = values.get_mut(idx) {
                    *v = reg.threshold + 0.5;
                    any = true;
                }
                if let Some(v) = prev.get_mut(idx) {
                    *v = 0.0;
                }
            }
        }
    }
    if any {
        session.state.install_resolved_values_at_boundary(&values);
        session
            .state
            .install_resolved_previous_values_at_boundary(&prev);
    }
}

fn upload_thr(session: &mut SimSession) -> Result<(), String> {
    use simthing_gpu::{
        emit_on_threshold_registrations_to_gpu, DEFAULT_THRESHOLD_EMISSION_CAPACITY,
    };
    materialize_emission_seeds(session);
    let Some(registry) = session.spec_state.resource_economy_registry.as_ref() else {
        return Ok(());
    };
    if registry.registrations.emit_on_threshold.is_empty() {
        return Ok(());
    }
    let gpu_regs =
        emit_on_threshold_registrations_to_gpu(&registry.registrations.emit_on_threshold);
    session
        .state
        .ensure_threshold_accumulator(DEFAULT_THRESHOLD_EMISSION_CAPACITY);
    session
        .state
        .upload_accumulator_threshold_ops(&gpu_regs)
        .map_err(|e| format!("{e}"))?;
    register_post_rf_need_threshold_rescan(
        &mut session.state,
        &session.spec_state.resolved_need_weight_profiles,
    );
    Ok(())
}

fn read_need(sim: &SimSession) -> f32 {
    let b = sim
        .spec_state
        .resolved_need_weight_profiles
        .first()
        .expect("binding");
    let values = sim.state.read_values();
    let n_dims = sim.state.n_dims as usize;
    values[b.participant_slot as usize * n_dims + b.need_col.raw()]
}

fn live_events(sim: &mut SimSession) -> Vec<simthing_gpu::ThresholdEvent> {
    sim.state
        .accumulator_runtime
        .as_mut()
        .and_then(|r| r.readback_threshold_events(&sim.state.ctx).ok())
        .unwrap_or_default()
}

fn count_kind(events: &[simthing_gpu::ThresholdEvent], kind: u32) -> usize {
    events.iter().filter(|e| e.event_kind() == kind).count()
}

#[test]
fn bare_weight_profiles_without_bindings_are_admission_gap() {
    let document = parse_raw_document(NEUTRAL_LOW.as_bytes()).expect("parse");
    let pack = hydrate_scenario(&document).expect("hydrate");
    match compose_need_weight_bindings(&pack, ARENA, &[]) {
        NeedWeightComposeOutcome::AdmissionGap {
            weight_profile_ids, ..
        } => {
            assert_eq!(weight_profile_ids, vec!["expansion_need".to_string()]);
        }
        NeedWeightComposeOutcome::Bindings(b) => panic!("must not invent: {b:?}"),
    }
}

#[test]
fn sources_resolve_from_authored_property_owners_not_install_host_rehome() {
    let sim = open_neutral(NEUTRAL_LOW);
    let b = sim
        .spec_state
        .resolved_need_weight_profiles
        .first()
        .expect("binding");
    // All sources share one eml_source_slot from find_property_owner.
    assert_eq!(b.eml_source_slot, b.input_cells[0].source_slot);
    assert_eq!(b.eml_source_slot, b.weight_cells[0].source_slot);
    // Need writes to participant; EvalEML reads source row.
    // Participant wrapper is distinct unless property owner is the hosted install node
    // and happens to share the wrapper slot (it must not for flow cell).
    assert_ne!(
        b.participant_slot, b.eml_source_slot,
        "need target participant must not be forced to equal source row re-home"
    );
    for cell in b.input_cells.iter().chain(b.weight_cells.iter()) {
        assert_eq!(
            cell.source_slot, b.eml_source_slot,
            "sources co-located for slot-local EvalEML"
        );
    }
}

#[test]
fn paired_authorings_live_need_and_sealed_gpu_events() {
    let mut low = open_neutral(NEUTRAL_LOW);
    let mut high = open_neutral(NEUTRAL_HIGH);
    upload_thr(&mut low).expect("thr");
    upload_thr(&mut high).expect("thr");
    low.step_once().expect("step");
    high.step_once().expect("step");
    let need_low = read_need(&low);
    let need_high = read_need(&high);
    assert!(
        need_high > need_low,
        "low={need_low} high={need_high}"
    );
    assert!(need_low < NEED_THRESHOLD, "low={need_low}");
    assert!(need_high >= NEED_THRESHOLD, "high={need_high}");
    assert_eq!(count_kind(&live_events(&mut low), NEED_EVENT_KIND), 0);
    assert!(count_kind(&live_events(&mut high), NEED_EVENT_KIND) > 0);
}

/// Live mutation via economy Constant formula (admitted path), not dense-vector poke.
#[test]
fn live_emission_formula_mutation_updates_need_without_reopen() {
    let mut sim = open_neutral(NEUTRAL_LOW);
    upload_thr(&mut sim).expect("thr");
    sim.step_once().expect("step1");
    let need_before = read_need(&sim);
    let b = sim
        .spec_state
        .resolved_need_weight_profiles
        .first()
        .expect("b")
        .clone();
    let w_col = b.weight_cells[0].col.raw_u32();
    let w_slot = b.weight_cells[0].source_slot;

    // Admitted path: mutate Constant emission for weight property, re-seed, step.
    {
        let econ = sim
            .spec_state
            .resource_economy_registry
            .as_mut()
            .expect("econ");
        for e in &mut econ.registrations.emissions {
            if e.source_slot == w_slot && e.source_col == w_col {
                e.formula = EmissionFormula::Constant { value: 5.0 };
            }
        }
        econ.generation = econ.generation.saturating_add(1);
    }
    materialize_emission_seeds(&mut sim);
    sim.step_once().expect("step2");
    let need_after = read_need(&sim);
    assert!(
        need_after > need_before + 0.5,
        "before={need_before} after={need_after}"
    );
}

#[test]
fn empty_weight_properties_fail_closed() {
    let document = parse_raw_document(NEUTRAL_LOW.as_bytes()).expect("parse");
    let pack = hydrate_scenario(&document).expect("hydrate");
    let mut b = explicit_neutral_binding();
    b.weight_properties.clear();
    match open_from_pack(&pack, vec![b], false) {
        Ok(_) => panic!("must fail"),
        Err(e) => assert!(e.contains("weight") || e.contains("admission"), "{e}"),
    }
}

#[test]
fn misbound_install_fails_closed() {
    let document = parse_raw_document(NEUTRAL_LOW.as_bytes()).expect("parse");
    let pack = hydrate_scenario(&document).expect("hydrate");
    match open_from_pack(&pack, vec![explicit_neutral_binding()], true) {
        Ok(_) => panic!("must fail"),
        Err(e) => assert!(
            e.contains("not admitted") || e.to_lowercase().contains("install") || e.contains("need"),
            "{e}"
        ),
    }
}

#[test]
fn post_rf_rescan_exactly_once_ordinary_and_need() {
    let mut high = open_neutral(NEUTRAL_HIGH);
    upload_thr(&mut high).expect("thr");
    high.step_once().expect("step");
    let events = live_events(&mut high);
    let need_n = count_kind(&events, NEED_EVENT_KIND);
    let ordinary_n = count_kind(&events, ORDINARY_EVENT_KIND);
    assert_eq!(
        need_n, 1,
        "need exactly once: {events:?}"
    );
    assert_eq!(
        ordinary_n, 1,
        "ordinary thr that actually crossed exactly once: {events:?}"
    );
}

/// Canonical TP: production Studio lowerer surfaces typed AdmissionGap (BLOCKED).
/// Rust companion bindings do not close the canonical transport gap.
#[test]
fn canonical_tp_studio_path_is_admission_gap_blocked() {
    let scenarios_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../scenarios");
    let tp_path = scenarios_dir.join("terran_pirate_galaxy.clause");
    let tp_bytes = std::fs::read(&tp_path).expect("read");
    let tp_doc = parse_raw_document(&tp_bytes).expect("parse");
    let tp_pack = simthing_clausething::hydrate_scenario_with_source_base(
        &tp_doc,
        Some(scenarios_dir.as_path()),
    )
    .expect("hydrate");

    // Bare clause weight_profiles → compose AdmissionGap.
    match compose_need_weight_bindings(&tp_pack, "studio_rf_arena", &[]) {
        NeedWeightComposeOutcome::AdmissionGap {
            weight_profile_ids, ..
        } => {
            assert!(weight_profile_ids.len() >= 2, "{weight_profile_ids:?}");
        }
        NeedWeightComposeOutcome::Bindings(b) => panic!("must not invent TP bindings: {b:?}"),
    }

    // Production Studio profile carries typed gap (not silent empty success).
    let profile = simthing_mapeditor::authored_live_profile_from_pack(&tp_pack);
    assert!(
        profile.rf5_admission_gap.is_some(),
        "Studio profile must expose typed RF-5 AdmissionGap for bare TP"
    );
    let gap = profile.rf5_admission_gap.as_ref().unwrap();
    assert!(
        gap.contains("admission gap") || gap.contains("RF-5"),
        "gap telemetry: {gap}"
    );
    // No invented need_weight_profiles on production GameMode when gap.
    let n_bindings = profile
        .game_mode
        .resource_flow
        .as_ref()
        .map(|rf| rf.need_weight_profiles.len())
        .unwrap_or(0);
    assert_eq!(
        n_bindings, 0,
        "must not invent complete bindings on GameMode when gap"
    );

    // Neutral second scenario still opens through production compose+open+step.
    let mut neutral = open_neutral(NEUTRAL_LOW);
    upload_thr(&mut neutral).expect("thr");
    neutral.step_once().expect("step neutral");
    assert_eq!(neutral.spec_state.resolved_need_weight_profiles.len(), 1);
}
