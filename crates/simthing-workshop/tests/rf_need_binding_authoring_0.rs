//! RF-NEED-BINDING-AUTHORING-0 (RF-5A) proofs.
//!
//! §12 homing: workshop only. Two scenario-agnostic Clause fixtures.
//! Zero TP tokens in clausething. Ordinary open_from_spec + step_once.

use std::collections::HashMap;

use simthing_clausething::{hydrate_scenario, parse_raw_document};
use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, DimensionRegistry, LogTier,
    SimThing, SimThingId, SimThingKind, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{Scenario, SimSession};
use simthing_gpu::{EmissionFormula, SlotAllocator};
use simthing_spec::{
    compile_property, ArenaSpec, BaseFlowDirectionSpec, BaseFlowObligationSpec, DomainPackSpec,
    ExplicitParticipantSpec, FissionPolicySpec, InstallTargetSpec, PropertyKey, PropertySpec,
    ResourceEconomyOptInMode, ResourceFlowExecutionProfile,
    ResourceFlowOptInMode, ResourceFlowSpec,
};

const ARENA: &str = "foundry";
const NEED_KIND: u32 = 91;
const NEED_THR: f32 = 0.5;

const FOUNDRY: &str = r#"
scenario = foundry_valley {
    owner = guild {
        owner_key = "guild"
        display_name = "Guild"
        archetype = "industrial"
    }
    location = ridge { display_name = "Ridge" }
    field_economy = valley_economy {
        namespace = "forge"
        stockpile_silo = guild_ore {
            owner = "guild"
            resource = "ore"
            current = 1
        }
        stockpile_silo = guild_weight {
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
        weight_profile = expansion_need {
            profile = "expansion-need"
            input = { input_col = 0 weight_col = 10 }
            output_col = 12
        }
        need_binding = expansion_need {
            profile = "expansion-need"
            participant = "guild"
            arena = "foundry"
            input = {
                entity = "guild"
                property = "forge::guild_ore_current"
                role = Amount
            }
            weight = {
                entity = "guild"
                property = "forge::guild_weight_token_current"
                role = Amount
            }
            threshold = 0.5
            event_kind = 91
        }
    }
}
"#;

const FOUNDRY_HIGH: &str = r#"
scenario = foundry_valley {
    owner = guild {
        owner_key = "guild"
        display_name = "Guild"
        archetype = "industrial"
    }
    location = ridge { display_name = "Ridge" }
    field_economy = valley_economy {
        namespace = "forge"
        stockpile_silo = guild_ore {
            owner = "guild"
            resource = "ore"
            current = 1
        }
        stockpile_silo = guild_weight {
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
        weight_profile = expansion_need {
            profile = "expansion-need"
            input = { input_col = 0 weight_col = 10 }
            output_col = 12
        }
        need_binding = expansion_need {
            profile = "expansion-need"
            participant = "guild"
            arena = "foundry"
            input = {
                entity = "guild"
                property = "forge::guild_ore_current"
                role = Amount
            }
            weight = {
                entity = "guild"
                property = "forge::guild_weight_token_current"
                role = Amount
            }
            threshold = 0.5
            event_kind = 91
        }
    }
}
"#;

const AQUEDUCT: &str = r#"
scenario = aqueduct_delta {
    owner = council {
        owner_key = "council"
        display_name = "Council"
        archetype = "civic"
    }
    location = spring { display_name = "Spring" }
    field_economy = waterworks {
        namespace = "civic"
        stockpile_silo = council_water {
            owner = "council"
            resource = "water"
            current = 2
        }
        stockpile_silo = council_weight {
            owner = "council"
            resource = "weight_token"
            current = 1.0
        }
        production_building = pump_house {
            location = "spring"
            input = { resource = "water" amount = 1 }
            output = { resource = "pressure" }
            throttle_hint_max_per_tick = 1
        }
        weight_profile = manufacturing_need {
            profile = "manufacturing-need"
            input = { input_col = 2 weight_col = 5 }
            output_col = 6
        }
        need_binding = manufacturing_need {
            profile = "manufacturing-need"
            participant = "council"
            arena = "foundry"
            input = {
                entity = "council"
                property = "civic::council_water_current"
                role = Amount
            }
            weight = {
                entity = "council"
                property = "civic::council_weight_token_current"
                role = Amount
            }
            threshold = 0.5
            event_kind = 91
        }
    }
}
"#;

fn flow_prop() -> PropertySpec {
    PropertySpec {
        id: "foundry_flow".into(),
        namespace: "workshop".into(),
        name: "foundry_flow".into(),
        display_name: String::new(),
        description: String::new(),
        sub_fields: vec![
            sf("flow", AccumulatorRole::IntrinsicFlow),
            sf(
                "allocated",
                AccumulatorRole::AllocatedFlow {
                    arena: ARENA.into(),
                },
            ),
            sf(
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

fn sf(name: &str, role: AccumulatorRole) -> SubFieldSpec {
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

fn open_clause(text: &str) -> Result<SimSession, String> {
    let doc = parse_raw_document(text.as_bytes()).map_err(|e| e.to_string())?;
    let pack = hydrate_scenario(&doc).map_err(|e| e.to_string())?;
    let mut registry = DimensionRegistry::new();
    compile_property(&flow_prop(), &mut registry).expect("flow");

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
    let slot = |id: SimThingId| allocator.slot_of(id).expect("s").raw();

    let mut install_targets = HashMap::new();
    install_targets.insert("guild".into(), vec![ids[1]]);
    install_targets.insert("council".into(), vec![ids[1]]);
    install_targets.insert("root".into(), vec![ids[0]]);
    for o in &pack.owners {
        install_targets
            .entry(o.owner_key.clone())
            .or_insert_with(|| vec![ids[1]]);
    }

    let mut game_mode = pack.game_mode.clone();
    game_mode.properties.retain(|p| {
        p.namespace == "forge" || p.namespace == "civic" || p.namespace == "workshop"
    });
    game_mode.capability_trees.clear();
    game_mode.events.clear();
    game_mode.region_fields.clear();
    // Admitted domain-pack path for overlays (ADR); move field overlays into pack.
    if !game_mode.overlays.is_empty() {
        let overlays = std::mem::take(&mut game_mode.overlays);
        game_mode.domain_packs.push(DomainPackSpec {
            id: "field_overlays".into(),
            display_name: "field overlays".into(),
            metadata: Default::default(),
            properties: vec![],
            overlays,
            capability_trees: vec![],
            events: vec![],
        });
    }
    if let Some(econ) = game_mode.resource_economy.as_mut() {
        econ.opt_in_mode = ResourceEconomyOptInMode::TransferAndEmission;
        econ.transfers.clear();
        econ.recipes.clear();
    }

    let mut need_bindings = game_mode
        .resource_flow
        .as_ref()
        .map(|rf| rf.need_bindings.clone())
        .unwrap_or_default();
    for b in &mut need_bindings {
        b.arena = ARENA.into();
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
        need_bindings,
        ..Default::default()
    });
    game_mode.resource_flow_execution_profile =
        ResourceFlowExecutionProfile::RecursiveArenaResourceFlow;

    let scenario = Scenario {
        name: "rf5a".into(),
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

fn seed_and_upload(sim: &mut SimSession) {
    use simthing_gpu::{
        emit_on_threshold_registrations_to_gpu, DEFAULT_THRESHOLD_EMISSION_CAPACITY,
    };
    if let Some(reg) = sim.spec_state.resource_economy_registry.as_ref() {
        let n = sim.state.n_dims as usize;
        let mut values = sim.state.read_values();
        let mut prev = values.clone();
        for e in &reg.registrations.emissions {
            if let EmissionFormula::Constant { value } = e.formula {
                let idx = e.source_slot as usize * n + e.source_col as usize;
                if let Some(v) = values.get_mut(idx) {
                    *v = (*v).max(value);
                }
                if let Some(v) = prev.get_mut(idx) {
                    *v = 0.0;
                }
            }
        }
        sim.state.install_resolved_values_at_boundary(&values);
        sim.state
            .install_resolved_previous_values_at_boundary(&prev);
        if !reg.registrations.emit_on_threshold.is_empty() {
            let gpu = emit_on_threshold_registrations_to_gpu(&reg.registrations.emit_on_threshold);
            sim.state
                .ensure_threshold_accumulator(DEFAULT_THRESHOLD_EMISSION_CAPACITY);
            let _ = sim.state.upload_accumulator_threshold_ops(&gpu);
            simthing_driver::need_binding::register_post_rf_need_threshold_rescan(
                &mut sim.state,
                &sim.spec_state.resolved_need_bindings,
            );
        }
    }
}

fn read_need(sim: &SimSession) -> f32 {
    let b = sim
        .spec_state
        .resolved_need_bindings
        .first()
        .expect("need binding");
    let v = sim.state.read_values();
    let n = sim.state.n_dims as usize;
    v[b.participant_slot as usize * n + b.need_col.raw()]
}

fn live_need_events(sim: &mut SimSession) -> usize {
    sim.state
        .accumulator_runtime
        .as_mut()
        .and_then(|r| r.readback_threshold_events(&sim.state.ctx).ok())
        .map(|ev| ev.iter().filter(|e| e.event_kind() == NEED_KIND).count())
        .unwrap_or(0)
}

#[test]
fn two_scenarios_hydrate_need_binding_generic_form() {
    for text in [FOUNDRY, AQUEDUCT] {
        let doc = parse_raw_document(text.as_bytes()).expect("parse");
        let pack = hydrate_scenario(&doc).expect("hydrate");
        let fe = pack.field_economy.as_ref().expect("fe");
        assert_eq!(fe.need_bindings.len(), 1);
        assert!(!fe.need_bindings[0].stack.gadgets.is_empty());
        let flow = pack.game_mode.resource_flow.as_ref().expect("rf");
        assert_eq!(flow.need_bindings.len(), 1);
    }
}

#[test]
fn absent_entity_fails_closed() {
    let bad = FOUNDRY.replace("participant = \"guild\"", "participant = \"missing\"");
    match open_clause(&bad) {
        Ok(_) => panic!("must fail"),
        Err(e) => assert!(
            e.contains("not admitted") || e.contains("install") || e.contains("need_binding") || e.contains("entity"),
            "{e}"
        ),
    }
}

#[test]
fn open_step_paired_need_and_sealed_events() {
    let mut low = match open_clause(FOUNDRY) {
        Ok(s) => s,
        Err(e) if e.to_lowercase().contains("adapter") => panic!("GPU FAIL: {e}"),
        Err(e) => panic!("{e}"),
    };
    let mut high = match open_clause(FOUNDRY_HIGH) {
        Ok(s) => s,
        Err(e) if e.to_lowercase().contains("adapter") => panic!("GPU FAIL: {e}"),
        Err(e) => panic!("{e}"),
    };
    assert_eq!(low.spec_state.resolved_need_bindings.len(), 1);
    seed_and_upload(&mut low);
    seed_and_upload(&mut high);
    low.step_once().expect("step");
    high.step_once().expect("step");
    let nl = read_need(&low);
    let nh = read_need(&high);
    assert!(nh > nl, "low={nl} high={nh}");
    assert!(nl < NEED_THR, "low={nl}");
    assert!(nh >= NEED_THR, "high={nh}");
    assert_eq!(live_need_events(&mut low), 0);
    assert!(live_need_events(&mut high) > 0);
}

#[test]
fn live_authored_weight_mutation_via_emission_refresh() {
    let mut sim = match open_clause(FOUNDRY) {
        Ok(s) => s,
        Err(e) if e.to_lowercase().contains("adapter") => panic!("GPU FAIL: {e}"),
        Err(e) => panic!("{e}"),
    };
    seed_and_upload(&mut sim);
    sim.step_once().expect("s1");
    let before = read_need(&sim);
    let b = sim.spec_state.resolved_need_bindings[0].clone();
    let w = &b.weights[0];
    // Production-shaped refresh: mutate Constant emission (economy path) and re-seed.
    {
        let econ = sim.spec_state.resource_economy_registry.as_mut().unwrap();
        for e in &mut econ.registrations.emissions {
            if e.source_slot == w.slot && e.source_col == w.col.raw_u32() {
                e.formula = EmissionFormula::Constant { value: 5.0 };
            }
        }
        econ.generation = econ.generation.saturating_add(1);
    }
    seed_and_upload(&mut sim);
    sim.step_once().expect("s2");
    let after = read_need(&sim);
    assert!(after > before + 0.5, "before={before} after={after}");
}

#[test]
fn aqueduct_second_scenario_same_generic_path() {
    let mut sim = match open_clause(AQUEDUCT) {
        Ok(s) => s,
        Err(e) if e.to_lowercase().contains("adapter") => panic!("GPU FAIL: {e}"),
        Err(e) => panic!("{e}"),
    };
    assert_eq!(sim.spec_state.resolved_need_bindings.len(), 1);
    seed_and_upload(&mut sim);
    sim.step_once().expect("step");
    assert!(read_need(&sim) > 0.0);
}
