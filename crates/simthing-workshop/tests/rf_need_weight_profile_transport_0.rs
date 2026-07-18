//! RF-NEED-WEIGHT-PROFILE-TRANSPORT-0 (RF-5) remand-2 + ontology addendum proofs.
//!
//! §12 homing: workshop only. Production path is ordinary `open_from_spec` +
//! `step_once`. GPU/adapter Unsupported is FAIL.
//!
//! Ontology: live locus is always (slot, column). Host owns authored Amounts;
//! participant wrappers receive on-device Identity projection each RF band —
//! never install-time CPU PropertyValue copy or CPU overlay recompute.
//!
//! Production compose attaches stacks by binding.id only; bare weight_profiles
//! without complete companion bindings → AdmissionGap.

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
use simthing_gpu::SlotAllocator;
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

const NEUTRAL_LOW: &str = r#"
scenario = foundry_valley {
    metadata = { display_name = "Foundry Valley Low" }
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
    location = ridge { display_name = "Ridge" }
    location = basin { display_name = "Basin" }
    field_economy = valley_economy {
        namespace = "forge"
        stockpile_silo = guild_ore {
            owner = "guild"
            resource = "ore"
            current = 5
        }
        stockpile_silo = guild_weight_store {
            owner = "guild"
            resource = "weight_token"
            current = 0
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
            threshold = 99
            direction = Rising
            event_kind = 77
        }
        owner_policy_overlay = guild_weight_policy {
            owner = "guild"
            targets_property = "forge::guild_weight_token_current"
            amount_add = 0.2
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
    owner = union {
        owner_key = "union"
        display_name = "Union"
        archetype = "industrial"
    }
    location = ridge { display_name = "Ridge" }
    location = basin { display_name = "Basin" }
    field_economy = valley_economy {
        namespace = "forge"
        stockpile_silo = guild_ore {
            owner = "guild"
            resource = "ore"
            current = 5
        }
        stockpile_silo = guild_weight_store {
            owner = "guild"
            resource = "weight_token"
            current = 0
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
            threshold = 99
            direction = Rising
            event_kind = 77
        }
        owner_policy_overlay = guild_weight_policy {
            owner = "guild"
            targets_property = "forge::guild_weight_token_current"
            amount_add = 3.0
        }
        weight_profile = expansion_need {
            profile = "expansion-need"
            input = { input_col = 0 weight_col = 10 }
            output_col = 12
        }
    }
}
"#;

fn flow_subfield(name: &str, role: AccumulatorRole) -> SubFieldSpec {
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

fn flow_property_spec() -> PropertySpec {
    PropertySpec {
        id: "foundry_flow".into(),
        namespace: "workshop".into(),
        name: "foundry_flow".into(),
        display_name: String::new(),
        description: String::new(),
        sub_fields: vec![
            flow_subfield("flow", AccumulatorRole::IntrinsicFlow),
            flow_subfield(
                "allocated",
                AccumulatorRole::AllocatedFlow {
                    arena: ARENA.into(),
                },
            ),
            flow_subfield(
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

/// Explicit complete binding (fixture authoring — not zip/first-stockpile inventing).
fn explicit_neutral_authored_binding() -> NeedWeightProfileBindingSpec {
    binding_from_hydrated_stack(
        "expansion_need",
        "expansion-need",
        simthing_spec::EmlGadgetStackSpec { gadgets: vec![] },
        ARENA,
        InstallTargetSpec::ScenarioListed {
            target_id: "guild".into(),
        },
        // Stockpile current (transfers cleared in open path so debit cannot
        // zero the host Amount before RF projection in the same tick).
        vec![PropertyKey::new("forge", "guild_ore_current")],
        vec![PropertyKey::new("forge", "guild_weight_token_current")],
        Some(NeedWeightProfileThresholdSpec {
            threshold: NEED_THRESHOLD,
            event_kind: NEED_EVENT_KIND,
        }),
    )
}

fn explicit_tp_authored_bindings() -> Vec<NeedWeightProfileBindingSpec> {
    let empty = simthing_spec::EmlGadgetStackSpec { gadgets: vec![] };
    let minerals_current = PropertyKey::new("tp_economy", "terran_minerals_current");
    let minerals_field = PropertyKey::new("tp_economy", "terran_shipyard_minerals_quantity");
    let hulls_qty = PropertyKey::new("tp_economy", "terran_shipyard_hulls_quantity");
    let disruption = PropertyKey::new("tp_economy", "pirate_outpost_disruption_presence");
    vec![
        binding_from_hydrated_stack(
            "terran_expansion_need",
            "expansion-need",
            empty.clone(),
            ARENA,
            InstallTargetSpec::ScenarioListed {
                target_id: "terran".into(),
            },
            vec![minerals_current.clone(), minerals_field.clone()],
            vec![hulls_qty.clone(), hulls_qty.clone()],
            Some(NeedWeightProfileThresholdSpec {
                threshold: NEED_THRESHOLD,
                event_kind: NEED_EVENT_KIND,
            }),
        ),
        binding_from_hydrated_stack(
            "terran_manufacturing_need",
            "manufacturing-need",
            empty.clone(),
            ARENA,
            InstallTargetSpec::ScenarioListed {
                target_id: "terran".into(),
            },
            vec![minerals_current],
            vec![hulls_qty],
            None,
        ),
        binding_from_hydrated_stack(
            "pirate_disruption_need",
            "disruption-need",
            empty,
            ARENA,
            InstallTargetSpec::ScenarioListed {
                target_id: "pirate".into(),
            },
            vec![disruption.clone()],
            vec![disruption],
            None,
        ),
    ]
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

fn open_from_clause_text(
    clause: &str,
    authored: Vec<NeedWeightProfileBindingSpec>,
    misbind: bool,
) -> Result<SimSession, String> {
    let document = parse_raw_document(clause.as_bytes()).map_err(|e| e.to_string())?;
    let pack = hydrate_scenario(&document).map_err(|e| e.to_string())?;
    open_from_pack(&pack, authored, misbind)
}

fn open_from_pack(
    pack: &HydratedScenarioPack,
    mut authored: Vec<NeedWeightProfileBindingSpec>,
    misbind: bool,
) -> Result<SimSession, String> {
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
    install_targets.insert("union".into(), vec![ids[1]]);
    install_targets.insert("root".into(), vec![ids[0]]);
    install_targets.insert("terran".into(), vec![ids[1]]);
    install_targets.insert("pirate".into(), vec![ids[2]]);
    for o in &pack.owners {
        install_targets
            .entry(o.owner_key.clone())
            .or_insert_with(|| vec![ids[1]]);
    }
    // Location-keyed field-economy overlays (quantity/presence) install via
    // ScenarioListed location ids — map them onto the session root host.
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
        for b in &economy.production_buildings {
            install_targets
                .entry(b.location.clone())
                .or_insert_with(|| vec![ids[0]]);
        }
    }

    let mut game_mode = pack.game_mode.clone();
    game_mode.properties.retain(|p| {
        p.namespace == "forge"
            || p.namespace == "field_economy"
            || p.namespace == "workshop"
            || p.namespace == "tp_economy"
            || p.id.starts_with("forge")
            || p.id.contains("valley")
    });
    game_mode.domain_packs.clear();
    game_mode.capability_trees.clear();
    game_mode.events.clear();
    game_mode.region_fields.clear();
    let retained: std::collections::HashSet<(String, String)> = game_mode
        .properties
        .iter()
        .map(|p| (p.namespace.clone(), p.name.clone()))
        .collect();
    game_mode.overlays.retain(|o| {
        if let Some((ns, name)) = o.targets_property.split_once("::") {
            retained.contains(&(ns.to_string(), name.to_string()))
        } else {
            false
        }
    });
    if let Some(econ) = game_mode.resource_economy.as_mut() {
        econ.opt_in_mode = ResourceEconomyOptInMode::TransferAndEmission;
        econ.emissions
            .retain(|e| retained.contains(&(e.source.namespace.clone(), e.source.name.clone())));
        // Drop silo transfers for RF-5 need transport proofs: transfer would
        // debit stockpile-current mid-tick and race the need projection.
        econ.transfers.clear();
        econ.emit_on_threshold.retain(|e| {
            retained.contains(&(e.source.namespace.clone(), e.source.name.clone()))
        });
        econ.recipes.clear();
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

fn read_need(sim: &SimSession) -> f32 {
    let binding = sim
        .spec_state
        .resolved_need_weight_profiles
        .first()
        .expect("resolved need binding");
    let values = sim.state.read_values();
    let n_dims = sim.state.n_dims as usize;
    values[binding.participant_slot as usize * n_dims + binding.need_col.raw()]
}

fn read_host_weight(sim: &SimSession) -> f32 {
    let binding = sim
        .spec_state
        .resolved_need_weight_profiles
        .first()
        .expect("resolved");
    let cell = binding.weight_cells.first().expect("weight cell");
    let values = sim.state.read_values();
    let n_dims = sim.state.n_dims as usize;
    values[cell.source_slot as usize * n_dims + cell.col.raw()]
}

fn live_sealed_events(sim: &mut SimSession) -> Vec<simthing_gpu::ThresholdEvent> {
    let Some(runtime) = sim.state.accumulator_runtime.as_mut() else {
        return Vec::new();
    };
    runtime
        .readback_threshold_events(&sim.state.ctx)
        .unwrap_or_default()
}

fn count_event_kind(events: &[simthing_gpu::ThresholdEvent], kind: u32) -> usize {
    events.iter().filter(|e| e.event_kind() == kind).count()
}

fn materialize_constant_emission_seeds(session: &mut SimSession) -> Result<(), String> {
    use simthing_gpu::EmissionFormula;
    let Some(registry) = session.spec_state.resource_economy_registry.as_ref() else {
        return Ok(());
    };
    let n_dims = session.state.n_dims as usize;
    let mut values = session.state.read_values();
    let mut prev = values.clone();
    let mut any = false;
    for emission in &registry.registrations.emissions {
        let EmissionFormula::Constant { value } = emission.formula else {
            continue;
        };
        if !value.is_finite() {
            continue;
        }
        let idx = emission.source_slot as usize * n_dims + emission.source_col as usize;
        if let Some(slot) = values.get_mut(idx) {
            *slot = (*slot).max(value);
            any = true;
        }
        if let Some(slot) = prev.get_mut(idx) {
            *slot = 0.0;
        }
    }
    if !any {
        return Ok(());
    }
    session.state.install_resolved_values_at_boundary(&values);
    session
        .state
        .install_resolved_previous_values_at_boundary(&prev);
    Ok(())
}

fn upload_economy_thresholds(session: &mut SimSession) -> Result<(), String> {
    use simthing_gpu::{
        emit_on_threshold_registrations_to_gpu, DEFAULT_THRESHOLD_EMISSION_CAPACITY,
    };
    // Same constant-emission seed path Studio field-bearing uses (values buffer).
    materialize_constant_emission_seeds(session)?;
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
        .map_err(|e| format!("upload emit_on_threshold: {e}"))?;
    register_post_rf_need_threshold_rescan(
        &mut session.state,
        &session.spec_state.resolved_need_weight_profiles,
    );
    Ok(())
}

fn open_neutral(clause: &str) -> SimSession {
    match open_from_clause_text(clause, vec![explicit_neutral_authored_binding()], false) {
        Ok(s) => s,
        Err(e) if e.to_lowercase().contains("adapter") || e.to_lowercase().contains("gpu") => {
            panic!("RF-5: GPU/adapter Unsupported is FAIL: {e}");
        }
        Err(e) => panic!("open: {e}"),
    }
}

#[test]
fn bare_weight_profiles_without_bindings_are_admission_gap() {
    let document = parse_raw_document(NEUTRAL_LOW.as_bytes()).expect("parse");
    let pack = hydrate_scenario(&document).expect("hydrate");
    match compose_need_weight_bindings(&pack, ARENA, &[]) {
        NeedWeightComposeOutcome::AdmissionGap {
            missing_fields,
            weight_profile_ids,
            ..
        } => {
            assert!(
                missing_fields.iter().any(|f| f.contains("install")),
                "gap must name install: {missing_fields:?}"
            );
            assert_eq!(weight_profile_ids, vec!["expansion_need".to_string()]);
        }
        NeedWeightComposeOutcome::Bindings(b) => {
            panic!("must not invent bindings from bare weight_profiles, got {b:?}")
        }
    }
}

#[test]
fn source_host_and_participant_rows_are_distinct_full_cells() {
    let sim = open_neutral(NEUTRAL_LOW);
    let b = sim
        .spec_state
        .resolved_need_weight_profiles
        .first()
        .expect("binding");
    assert!(
        !b.weight_cells.is_empty() && !b.input_cells.is_empty(),
        "full-cell sources required"
    );
    for cell in b.input_cells.iter().chain(b.weight_cells.iter()) {
        assert_ne!(
            cell.source_slot, b.participant_slot,
            "host source_slot must differ from arena participant wrapper (ontology); \
             got source={} participant={} for {}",
            cell.source_slot, b.participant_slot, cell.property.name
        );
        assert_eq!(cell.source_id, b.hosted_id, "source id is install host");
    }
}

#[test]
fn paired_clause_authorings_change_live_need_and_sealed_threshold_events() {
    let mut low = open_neutral(NEUTRAL_LOW);
    let mut high = open_neutral(NEUTRAL_HIGH);

    upload_economy_thresholds(&mut low).expect("upload low thr");
    upload_economy_thresholds(&mut high).expect("upload high thr");

    low.step_once().expect("step low");
    high.step_once().expect("step high");

    let need_low = read_need(&low);
    let need_high = read_need(&high);
    assert!(
        need_high > need_low,
        "paired overlay authorings must diverge need: low={need_low} high={need_high}"
    );
    assert!(
        need_low < NEED_THRESHOLD,
        "low need {need_low} must stay below threshold {NEED_THRESHOLD}"
    );
    assert!(
        need_high >= NEED_THRESHOLD,
        "high need {need_high} must cross threshold {NEED_THRESHOLD}"
    );

    let events_low = live_sealed_events(&mut low);
    let events_high = live_sealed_events(&mut high);
    assert_eq!(
        count_event_kind(&events_low, NEED_EVENT_KIND),
        0,
        "below-threshold live sealed need events must be 0: {events_low:?}"
    );
    assert!(
        count_event_kind(&events_high, NEED_EVENT_KIND) > 0,
        "crossing must emit live sealed need event post-step_once: {events_high:?}"
    );
}

/// Live mutation: open once, change only an authored host **input** source cell
/// (no overlay recompute of this cell), step again, observe need change without
/// reopen/reseed/CPU participant copy.
#[test]
fn live_host_source_mutation_updates_need_without_reopen() {
    let mut sim = open_neutral(NEUTRAL_LOW);
    upload_economy_thresholds(&mut sim).expect("upload thr");
    sim.step_once().expect("step1");
    let need_before = read_need(&sim);

    // Mutate only the authored host input Amount cell (source row). Weight is
    // overlay-driven each tick; input is the live host Amount we can raise.
    let binding = sim
        .spec_state
        .resolved_need_weight_profiles
        .first()
        .expect("binding")
        .clone();
    let cell = binding.input_cells.first().expect("input").clone();
    assert_ne!(cell.source_slot, binding.participant_slot);
    let n_dims = sim.state.n_dims as usize;
    let mut values = sim.state.read_values();
    let host_idx = cell.source_slot as usize * n_dims + cell.col.raw();
    let host_before = values[host_idx];
    let raised = host_before + 10.0;
    values[host_idx] = raised;
    sim.state.install_resolved_values_at_boundary(&values);

    sim.step_once().expect("step2");
    let need_after = read_need(&sim);
    assert!(
        need_after > need_before + 1e-3,
        "live host input mutation must raise need without reopen: before={need_before} after={need_after} host_before={host_before}"
    );
    // Projection carried a higher host input into need. Source host and participant
    // remain distinct rows (ontology).
    assert_ne!(cell.source_slot, binding.participant_slot);
    let _ = raised;
}

#[test]
fn empty_weight_properties_fail_closed() {
    let document = parse_raw_document(NEUTRAL_LOW.as_bytes()).expect("parse");
    let pack = hydrate_scenario(&document).expect("hydrate");
    let mut authored = explicit_neutral_authored_binding();
    authored.weight_properties.clear();
    match open_from_pack(&pack, vec![authored], false) {
        Ok(_) => panic!("empty weights must fail"),
        Err(err) => assert!(
            err.contains("weight_properties")
                || err.contains("admission_gap")
                || err.to_lowercase().contains("need weight"),
            "unexpected: {err}"
        ),
    }
}

#[test]
fn misbound_install_target_fails_closed() {
    match open_from_clause_text(
        NEUTRAL_LOW,
        vec![explicit_neutral_authored_binding()],
        true,
    ) {
        Ok(_) => panic!("misbound install must fail closed"),
        Err(err) => {
            assert!(
                err.contains("not admitted")
                    || err.contains("NoMatchingOwners")
                    || err.contains("not defined")
                    || err.to_lowercase().contains("need weight")
                    || err.to_lowercase().contains("install"),
                "unexpected: {err}"
            );
        }
    }
}

#[test]
fn post_rf_rescan_captures_need_without_duplicating_ordinary_threshold() {
    let mut high = open_neutral(NEUTRAL_HIGH);
    upload_economy_thresholds(&mut high).expect("upload thr");
    high.step_once().expect("step");
    let events = live_sealed_events(&mut high);
    let need_n = count_event_kind(&events, NEED_EVENT_KIND);
    let ordinary_n = count_event_kind(&events, ORDINARY_EVENT_KIND);
    assert_eq!(
        need_n, 1,
        "need crossing exactly once post-RF, got {need_n} events={events:?}"
    );
    assert!(
        ordinary_n <= 1,
        "ordinary threshold must not be duplicated, got {ordinary_n}"
    );
}

#[test]
fn canonical_tp_and_neutral_share_production_composer_path() {
    let scenarios_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../scenarios");
    let tp_path = scenarios_dir.join("terran_pirate_galaxy.clause");
    let tp_bytes = std::fs::read(&tp_path).expect("read canonical clause");
    let tp_doc = parse_raw_document(&tp_bytes).expect("parse TP");
    let tp_pack = simthing_clausething::hydrate_scenario_with_source_base(
        &tp_doc,
        Some(scenarios_dir.as_path()),
    )
    .expect("hydrate TP");

    match compose_need_weight_bindings(&tp_pack, ARENA, &[]) {
        NeedWeightComposeOutcome::AdmissionGap {
            weight_profile_ids, ..
        } => {
            assert!(weight_profile_ids.len() >= 2);
        }
        NeedWeightComposeOutcome::Bindings(b) => {
            panic!("must not invent TP bindings, got {b:?}")
        }
    }

    let tp_composed =
        compose_or_err(&tp_pack, &explicit_tp_authored_bindings()).expect("compose TP");
    let owners: Vec<_> = tp_composed
        .iter()
        .map(|b| match &b.install {
            InstallTargetSpec::ScenarioListed { target_id } => target_id.clone(),
            _ => String::new(),
        })
        .collect();
    assert!(
        owners.iter().any(|o| o == "terran") && owners.iter().any(|o| o == "pirate"),
        "multi-profile owners: {owners:?}"
    );

    let mut tp_session = match open_from_pack(&tp_pack, explicit_tp_authored_bindings(), false) {
        Ok(s) => s,
        Err(e) if e.to_lowercase().contains("adapter") || e.to_lowercase().contains("gpu") => {
            panic!("RF-5: GPU/adapter Unsupported is FAIL: {e}");
        }
        Err(e) => panic!("open TP: {e}"),
    };
    let b0 = tp_session
        .spec_state
        .resolved_need_weight_profiles
        .first()
        .expect("tp binding");
    assert_ne!(
        b0.weight_cells[0].source_slot, b0.participant_slot,
        "TP full-cell host ≠ participant"
    );
    upload_economy_thresholds(&mut tp_session).expect("TP thr");
    tp_session.step_once().expect("step TP");

    let mut neutral = open_neutral(NEUTRAL_LOW);
    upload_economy_thresholds(&mut neutral).expect("neutral thr");
    neutral.step_once().expect("step neutral");
}
