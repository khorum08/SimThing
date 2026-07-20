//! RF-NEED-BINDING-AUTHORING-0 (RF-5A) proofs.
//!
//! §12 homing: workshop only. Two scenario-agnostic Clause fixtures.
//! Zero TP tokens in clausething. Ordinary open_from_spec + step_once.
//!
//! DA Modified Option A: staged GPU projection + LIVE/DISCONNECT/STATIC controls.
//! Mid-session re-authoring API is not required for RF-5A.

use std::collections::HashMap;

use simthing_clausething::{hydrate_scenario, parse_raw_document};
use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, DimensionRegistry, LogTier,
    SimThing, SimThingId, SimThingKind, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{InstallError, Scenario, SessionError, SimSession};
use simthing_gpu::SlotAllocator;
use simthing_spec::{
    compile_property, ArenaSpec, BaseFlowDirectionSpec, BaseFlowObligationSpec, DomainPackSpec,
    ExplicitParticipantSpec, FissionPolicySpec, InstallTargetSpec, PropertyKey, PropertySpec,
    ResourceEconomyOptInMode, ResourceFlowExecutionProfile, ResourceFlowOptInMode,
    ResourceFlowSpec,
};

const ARENA: &str = "foundry";
const NEED_KIND: u32 = 91;
const ORD_KIND: u32 = 77;
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
            output = { resource = "tools" coefficient = 1 }
            throttle_hint_max_per_tick = 1
        }
        disruption_presence = ridge_smoke {
            location = "ridge"
            resource = "smoke"
            amount = 0
            threshold = 100
            event_kind = 177
        }
        flow_coupling = smoke_suppresses_tools {
            source = { location = "ridge" resource = "tools" unit_cost = 1 }
            pressure = { location = "ridge" resource = "smoke" unit_cost = 1 }
            weight = { owner = "guild" resource = "weight_token" unit_cost = 1 }
            sink = { location = "ridge" resource = "spoiled_tools" }
            output_coefficient = 1
            order_band = 1
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
            output = { resource = "tools" coefficient = 1 }
            throttle_hint_max_per_tick = 1
        }
        disruption_presence = ridge_smoke {
            location = "ridge"
            resource = "smoke"
            amount = 0
            threshold = 100
            event_kind = 177
        }
        flow_coupling = smoke_suppresses_tools {
            source = { location = "ridge" resource = "tools" unit_cost = 1 }
            pressure = { location = "ridge" resource = "smoke" unit_cost = 1 }
            weight = { owner = "guild" resource = "weight_token" unit_cost = 1 }
            sink = { location = "ridge" resource = "spoiled_tools" }
            output_coefficient = 1
            order_band = 1
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
            output = { resource = "pressure" coefficient = 1 }
            throttle_hint_max_per_tick = 1
        }
        disruption_presence = spring_silt {
            location = "spring"
            resource = "silt"
            amount = 0
            threshold = 100
            event_kind = 178
        }
        flow_coupling = silt_suppresses_pressure {
            source = { location = "spring" resource = "pressure" unit_cost = 1 }
            pressure = { location = "spring" resource = "silt" unit_cost = 1 }
            weight = { owner = "council" resource = "weight_token" unit_cost = 1 }
            sink = { location = "spring" resource = "lost_pressure" }
            output_coefficient = 1
            order_band = 1
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

/// Unit substrate: hydrate Clause, host owners via install_targets, admit RF arena.
/// Property instances come from economy materialization on entity hosts (not need invent).
#[derive(Debug)]
enum OpenError {
    Frontend(String),
    Session(SessionError),
}

impl std::fmt::Display for OpenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Frontend(error) => f.write_str(error),
            Self::Session(error) => std::fmt::Display::fmt(error, f),
        }
    }
}

impl OpenError {
    fn to_lowercase(&self) -> String {
        self.to_string().to_lowercase()
    }

    fn span_token(&self) -> Option<usize> {
        match self {
            Self::Session(SessionError::Install(InstallError::NeedBindingInvalid {
                span_token,
                ..
            })) => *span_token,
            _ => None,
        }
    }
}

fn open_clause(text: &str) -> Result<SimSession, OpenError> {
    open_clause_opts(text, OpenOpts::default())
}

#[derive(Clone, Default)]
struct OpenOpts {
    /// Duplicate install_targets entry for an entity (ambiguity falsifier).
    duplicate_entity: Option<&'static str>,
    /// Drop participant from arena explicit list.
    drop_participant_from_arena: bool,
    /// Cross-row happy path: second owner hosts weight stockpile.
    cross_row_second_owner: bool,
    /// Attach an ordinary (non-need) emit_on_threshold after hydrate for bite proof.
    ordinary_threshold_on_ore: bool,
    /// LIVE-TRACKING: authored per-tick transfer depletes installed weight state.
    live_tracking_transfer: bool,
    /// STATIC: authored Constant supplies install seed, economy execution stays off.
    static_installed_source: bool,
    /// Override every host-qualified economy row while preserving authored spans.
    economy_host_override: Option<&'static str>,
    /// Place one already-guild-qualified property on union as well.
    conflicting_economy_host: bool,
    /// Resolve a second binding on the same participant with a distinct source.
    second_binding_same_participant: bool,
}

fn open_clause_opts(text: &str, opts: OpenOpts) -> Result<SimSession, OpenError> {
    let doc =
        parse_raw_document(text.as_bytes()).map_err(|e| OpenError::Frontend(e.to_string()))?;
    let pack = hydrate_scenario(&doc).map_err(|e| OpenError::Frontend(e.to_string()))?;
    let mut registry = DimensionRegistry::new();
    compile_property(&flow_prop(), &mut registry).expect("flow");

    let mut root = SimThing::new(SimThingKind::World, 0);
    let session_root = SimThing::new(SimThingKind::Cohort, 0);
    let owner = SimThing::new(SimThingKind::Cohort, 0);
    let child = SimThing::new(SimThingKind::Cohort, 0);
    let owner2 = SimThing::new(SimThingKind::Cohort, 0);
    let ids = [session_root.id, owner.id, child.id, owner2.id];
    root.add_child(session_root);
    root.add_child(owner);
    root.add_child(child);
    root.add_child(owner2);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let slot = |id: SimThingId| allocator.slot_of(id).expect("s").raw();

    let mut install_targets = HashMap::new();
    install_targets.insert("guild".into(), vec![ids[1]]);
    install_targets.insert("council".into(), vec![ids[1]]);
    install_targets.insert("root".into(), vec![ids[0]]);
    install_targets.insert("ridge".into(), vec![ids[2]]);
    install_targets.insert("spring".into(), vec![ids[2]]);
    if opts.cross_row_second_owner {
        // Second entity hosts weight stockpile (cross-row projection happy path).
        install_targets.insert("union".into(), vec![ids[3]]);
    }
    if opts.conflicting_economy_host {
        install_targets.insert("union".into(), vec![ids[3]]);
    }
    if let Some(dup) = opts.duplicate_entity {
        install_targets
            .entry(dup.into())
            .and_modify(|v| v.push(ids[3]))
            .or_insert_with(|| vec![ids[1], ids[3]]);
    }
    for o in &pack.owners {
        install_targets
            .entry(o.owner_key.clone())
            .or_insert_with(|| vec![ids[1]]);
    }

    let mut game_mode = pack.game_mode.clone();
    game_mode
        .properties
        .retain(|p| p.namespace == "forge" || p.namespace == "civic" || p.namespace == "workshop");
    game_mode.capability_trees.clear();
    game_mode.events.clear();
    game_mode.region_fields.clear();
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
        econ.recipes.clear();
        if let Some(host) = opts.economy_host_override {
            for transfer in &mut econ.transfers {
                transfer.source_host_entity = Some(host.into());
                transfer.target_host_entity = Some(host.into());
            }
            for emission in &mut econ.emissions {
                emission.host_entity = Some(host.into());
            }
            for threshold in &mut econ.emit_on_threshold {
                threshold.host_entity = Some(host.into());
            }
        }
        if opts.conflicting_economy_host {
            let mut conflicting = econ
                .emissions
                .iter()
                .find(|emission| emission.source.name.contains("ore_current"))
                .expect("ore emission")
                .clone();
            conflicting.id.push_str("_conflicting_union");
            conflicting.host_entity = Some("union".into());
            econ.emissions.push(conflicting);
        }
        if opts.static_installed_source {
            // Constant emissions remain solely as the admitted install seed.
            // Disabled execution means no per-tick authority touches the sources.
            econ.opt_in_mode = ResourceEconomyOptInMode::Disabled;
            econ.transfers.clear();
            for p in &mut game_mode.properties {
                if p.namespace == "forge" && p.name.contains("weight_token_current") {
                    if p.sub_fields.is_empty() {
                        p.sub_fields.push(simthing_core::SubFieldSpec {
                            role: SubFieldRole::Amount,
                            width: 1,
                            clamp: simthing_core::ClampBehavior::Unbounded,
                            velocity_max: None,
                            default: 3.0,
                            display_name: "amount".into(),
                            display_range: None,
                            governed_by: None,
                            reduction_override: None,
                            soft_aggregate_guard: None,
                            accumulator_spec: None,
                        });
                    } else {
                        for sf in &mut p.sub_fields {
                            if matches!(sf.role, SubFieldRole::Amount) {
                                sf.default = 3.0;
                            }
                        }
                    }
                }
            }
        }
        if opts.live_tracking_transfer {
            // Authored drip transfer only — no Constant emissions (would re-write sources).
            // Initial values enter via PropertySpec Amount defaults (place path).
            use simthing_spec::ResourceTransferSpec;
            // Ore may keep Constant emission (static at 1.0). Weight must not —
            // only the drip transfer mutates weight current.
            econ.emissions
                .retain(|e| e.source.name.contains("ore_current"));
            econ.transfers.clear();
            econ.transfers.push(ResourceTransferSpec {
                id: "live_weight_drip".into(),
                source: PropertyKey::new("forge", "guild_weight_token_current"),
                source_role: SubFieldRole::Amount,
                target: PropertyKey::new("forge", "guild_weight_token_stockpile"),
                target_role: SubFieldRole::Amount,
                amount: 0.25,
                order_band: 0,
                source_host_entity: Some("guild".into()),
                target_host_entity: Some("guild".into()),
                source_host_span_token: None,
                target_host_span_token: None,
            });
            // Weight initial value via PropertySpec Amount default (place path).
            for p in &mut game_mode.properties {
                if p.namespace != "forge" {
                    continue;
                }
                let v = if p.name.contains("weight_token_current") {
                    3.0
                } else if p.name.contains("weight_token_stockpile") {
                    0.0
                } else {
                    continue;
                };
                if p.sub_fields.is_empty() {
                    p.sub_fields.push(simthing_core::SubFieldSpec {
                        role: SubFieldRole::Amount,
                        width: 1,
                        clamp: simthing_core::ClampBehavior::Unbounded,
                        velocity_max: None,
                        default: v,
                        display_name: "amount".into(),
                        display_range: None,
                        governed_by: None,
                        reduction_override: None,
                        soft_aggregate_guard: None,
                        accumulator_spec: None,
                    });
                } else {
                    for sf in &mut p.sub_fields {
                        if matches!(sf.role, SubFieldRole::Amount) {
                            sf.default = v;
                        }
                    }
                }
            }
        } else if !opts.static_installed_source {
            econ.transfers.clear();
        }
        if opts.ordinary_threshold_on_ore {
            // Unrelated thr on weight stockpile; drip transfer moves stockpile 0→0.25
            // across thr 0.1 without dense previous-value reseed.
            use simthing_spec::{EmitOnThresholdSpec, ResourceTransferSpec, TriggerDirection};
            econ.emit_on_threshold.push(EmitOnThresholdSpec {
                id: "ordinary_stockpile_thr".into(),
                source: PropertyKey::new("forge", "guild_weight_token_stockpile"),
                source_role: SubFieldRole::Amount,
                threshold: 0.1,
                direction: TriggerDirection::Rising,
                event_kind: ORD_KIND,
                buffer: Default::default(),
                host_entity: Some("guild".into()),
                host_span_token: None,
            });
            econ.transfers.push(ResourceTransferSpec {
                id: "ord_weight_drip".into(),
                source: PropertyKey::new("forge", "guild_weight_token_current"),
                source_role: SubFieldRole::Amount,
                target: PropertyKey::new("forge", "guild_weight_token_stockpile"),
                target_role: SubFieldRole::Amount,
                amount: 0.25,
                order_band: 0,
                source_host_entity: Some("guild".into()),
                target_host_entity: Some("guild".into()),
                source_host_span_token: None,
                target_host_span_token: None,
            });
        }
    }

    let mut need_bindings = game_mode
        .resource_flow
        .as_ref()
        .map(|rf| rf.need_bindings.clone())
        .unwrap_or_default();
    if opts.second_binding_same_participant {
        let mut second = need_bindings[0].clone();
        second.id = "second_same_participant".into();
        second.event_kind = NEED_KIND + 1;
        second.weights[0] = second.inputs[0].clone();
        need_bindings.push(second);
    }

    let mut participants = vec![
        ExplicitParticipantSpec::flat(slot(ids[0]), ids[0].raw()),
        ExplicitParticipantSpec::nested(slot(ids[1]), ids[1].raw(), ids[0].raw() as u64),
        ExplicitParticipantSpec::nested(slot(ids[2]), ids[2].raw(), ids[1].raw() as u64),
    ];
    if opts.drop_participant_from_arena {
        // Keep root only — owner participant absent from arena admission.
        participants.truncate(1);
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
            explicit_participants: participants,
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
    SimSession::open_from_spec(scenario, &game_mode).map_err(OpenError::Session)
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

fn count_events(sim: &mut SimSession, kind: u32) -> usize {
    sim.state
        .accumulator_runtime
        .as_mut()
        .and_then(|r| r.readback_threshold_events(&sim.state.ctx).ok())
        .map(|ev| ev.iter().filter(|e| e.event_kind() == kind).count())
        .unwrap_or(0)
}

fn assert_err_contains(result: Result<SimSession, OpenError>, needles: &[&str]) {
    match result {
        Ok(_) => panic!("expected admission failure, got Ok"),
        Err(e) => {
            let lower = e.to_lowercase();
            let hit = needles.iter().any(|n| lower.contains(&n.to_lowercase()));
            assert!(hit, "error `{e}` missing any of {needles:?}");
        }
    }
}

fn hydrate_err(text: &str) -> String {
    let doc = parse_raw_document(text.as_bytes()).expect("parse");
    match hydrate_scenario(&doc) {
        Ok(_) => panic!("expected hydrate failure"),
        Err(e) => e.to_string(),
    }
}

// ── Grammar / hydrate ──────────────────────────────────────────────────────

#[test]
fn two_scenarios_hydrate_need_binding_generic_form() {
    for text in [FOUNDRY, AQUEDUCT] {
        let doc = parse_raw_document(text.as_bytes()).expect("parse");
        let pack = hydrate_scenario(&doc).expect("hydrate");
        let fe = pack.field_economy.as_ref().expect("fe");
        assert_eq!(fe.need_bindings.len(), 1);
        assert!(!fe.need_bindings[0].stack.gadgets.is_empty());
        let b = &fe.need_bindings[0];
        assert!(!b.arena.is_empty() && b.arena != "default");
        assert!(b.threshold > 0.0);
        assert!(b.event_kind > 0);
        assert!(b.source_span_token.is_some());
        assert!(b.participant_span_token.is_some());
        assert!(b.arena_span_token.is_some());
        assert!(b.inputs[0].source_span_token.is_some());
        let flow = pack.game_mode.resource_flow.as_ref().expect("rf");
        assert_eq!(flow.need_bindings.len(), 1);
    }
}

#[test]
fn profile_join_missing_fails_spanned() {
    let bad = FOUNDRY.replace(
        "need_binding = expansion_need {",
        "need_binding = no_such_profile {",
    );
    let e = hydrate_err(&bad);
    assert!(
        e.contains("no weight_profile") || e.contains("empty stack"),
        "{e}"
    );
}

#[test]
fn profile_join_mismatch_fails() {
    let bad = FOUNDRY.replace(
        "need_binding = expansion_need {\n            profile = \"expansion-need\"",
        "need_binding = expansion_need {\n            profile = \"other-need\"",
    );
    let e = hydrate_err(&bad);
    assert!(e.contains("mismatches") || e.contains("profile"), "{e}");
}

#[test]
fn missing_threshold_fails_spanned() {
    let bad = FOUNDRY.replace(
        "threshold = 0.5\n            event_kind = 91\n",
        "event_kind = 91\n",
    );
    let e = hydrate_err(&bad);
    assert!(e.to_lowercase().contains("threshold"), "{e}");
}

#[test]
fn missing_arena_fails_spanned() {
    let bad = FOUNDRY.replace("arena = \"foundry\"\n", "");
    let e = hydrate_err(&bad);
    assert!(e.to_lowercase().contains("arena"), "{e}");
}

// ── Admission falsifiers ───────────────────────────────────────────────────

#[test]
fn absent_entity_fails_closed() {
    let bad = FOUNDRY.replace("participant = \"guild\"", "participant = \"missing\"");
    assert_err_contains(
        open_clause(&bad),
        &["not admitted", "install", "entity", "need_binding"],
    );
}

#[test]
fn absent_source_entity_fails_closed() {
    let bad = FOUNDRY.replace(
        "entity = \"guild\"\n                property = \"forge::guild_ore_current\"",
        "entity = \"ghost\"\n                property = \"forge::guild_ore_current\"",
    );
    assert_err_contains(open_clause(&bad), &["entity", "install_targets", "ghost"]);
}

#[test]
fn ambiguous_entity_fails_closed() {
    assert_spanned_error(
        open_clause_opts(
            FOUNDRY,
            OpenOpts {
                duplicate_entity: Some("guild"),
                ..Default::default()
            },
        ),
        economy_owner_span(FOUNDRY),
    );
}

#[test]
fn missing_economy_host_fails_at_authored_span() {
    assert_spanned_error(
        open_clause_opts(
            FOUNDRY,
            OpenOpts {
                economy_host_override: Some("ghost"),
                ..Default::default()
            },
        ),
        economy_owner_span(FOUNDRY),
    );
}

#[test]
fn duplicate_conflicting_property_placement_fails_at_host_span() {
    assert_spanned_error(
        open_clause_opts(
            FOUNDRY,
            OpenOpts {
                conflicting_economy_host: true,
                ..Default::default()
            },
        ),
        economy_owner_span(FOUNDRY),
    );
}

#[test]
fn named_entity_without_referenced_property_fails_at_source_span() {
    let bad = FOUNDRY.replacen(
        "entity = \"guild\"\n                property = \"forge::guild_ore_current\"",
        "entity = \"union\"\n                property = \"forge::guild_ore_current\"",
        1,
    );
    assert_spanned_error(
        open_clause_opts(
            &bad,
            OpenOpts {
                cross_row_second_owner: true,
                ..Default::default()
            },
        ),
        first_input_span(&bad),
    );
}

#[test]
fn source_missing_property_fails_closed() {
    let bad = FOUNDRY.replace(
        "property = \"forge::guild_ore_current\"",
        "property = \"forge::guild_no_such_current\"",
    );
    assert_err_contains(
        open_clause(&bad),
        &["not registered", "does not own", "property"],
    );
}

#[test]
fn property_missing_role_fails_closed() {
    // Amount is the only admitted role in RF-5A grammar; unsupported role is hydrate-spanned.
    let bad = FOUNDRY.replace("role = Amount", "role = Velocity");
    let e = hydrate_err(&bad);
    assert!(
        e.contains("unsupported") || e.contains("Amount only") || e.contains("role"),
        "{e}"
    );
}

#[test]
fn participant_not_admitted_to_arena_fails() {
    assert_err_contains(
        open_clause_opts(
            FOUNDRY,
            OpenOpts {
                drop_participant_from_arena: true,
                ..Default::default()
            },
        ),
        &["not admitted", "arena", "participant"],
    );
}

#[test]
fn prepare_need_cells_does_not_invent_missing_flow() {
    // prepare_need_binding_cells zeros an existing need cell only — never invents
    // a flow PropertyValue on the participant wrapper.
    use simthing_driver::need_binding::{prepare_need_binding_cells, ResolvedNeedBinding};
    let mut registry = DimensionRegistry::new();
    compile_property(&flow_prop(), &mut registry).expect("flow");
    let flow_pid = registry.id_of("workshop", "foundry_flow").expect("id");
    let need_col = registry
        .column_range(flow_pid)
        .col_for_role(
            &SubFieldRole::Named("weight".into()),
            &registry.property(flow_pid).layout,
        )
        .expect("weight col");
    let mut root = SimThing::new(SimThingKind::World, 0);
    let wrapper = SimThing::new(SimThingKind::Cohort, 0);
    let wrapper_id = wrapper.id;
    root.add_child(wrapper);
    // Deliberately omit foundry_flow on wrapper — prepare must fail closed.
    let resolved = vec![ResolvedNeedBinding {
        id: "t".into(),
        profile: "expansion-need".into(),
        participant_slot: 1,
        participant_id: wrapper_id,
        eml_source_slot: 1,
        need_col,
        inputs: vec![],
        weights: vec![],
        staged_input_cols: vec![],
        staged_weight_cols: vec![],
        nodes: vec![],
        threshold: 0.5,
        event_kind: NEED_KIND,
    }];
    let err = prepare_need_binding_cells(&resolved, &registry, &mut root);
    match err {
        Ok(()) => panic!("must not invent missing participant flow property"),
        Err(e) => {
            let s = e.to_string().to_lowercase();
            assert!(
                s.contains("missing") || s.contains("invent") || s.contains("flow"),
                "{e}"
            );
        }
    }
}

const CROSS_ROW: &str = r#"
scenario = foundry_valley {
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
    field_economy = valley_economy {
        namespace = "forge"
        stockpile_silo = guild_ore {
            owner = "guild"
            resource = "ore"
            current = 1
        }
        stockpile_silo = union_weight {
            owner = "union"
            resource = "weight_token"
            current = 3.0
        }
        production_building = ridge_foundry {
            location = "ridge"
            input = { resource = "ore" amount = 1 }
            output = { resource = "tools" coefficient = 1 }
            throttle_hint_max_per_tick = 1
        }
        disruption_presence = ridge_smoke {
            location = "ridge"
            resource = "smoke"
            amount = 0
            threshold = 100
            event_kind = 177
        }
        flow_coupling = smoke_suppresses_tools {
            source = { location = "ridge" resource = "tools" unit_cost = 1 }
            pressure = { location = "ridge" resource = "smoke" unit_cost = 1 }
            weight = { owner = "guild" resource = "weight_token" unit_cost = 1 }
            sink = { location = "ridge" resource = "spoiled_tools" }
            output_coefficient = 1
            order_band = 1
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
                entity = "union"
                property = "forge::union_weight_token_current"
                role = Amount
            }
            threshold = 0.5
            event_kind = 91
        }
    }
}
"#;

#[test]
fn cross_row_sources_project_via_stage_happy_path() {
    let mut sim = match open_clause_opts(
        CROSS_ROW,
        OpenOpts {
            cross_row_second_owner: true,
            ..Default::default()
        },
    ) {
        Ok(s) => s,
        Err(e) if e.to_lowercase().contains("adapter") => panic!("GPU FAIL: {e}"),
        Err(e) => panic!("{e}"),
    };
    let b = &sim.spec_state.resolved_need_bindings[0];
    assert_ne!(
        b.inputs[0].slot, b.weights[0].slot,
        "cross-row requires distinct source slots"
    );
    assert_eq!(b.eml_source_slot, b.participant_slot);
    assert_ne!(b.inputs[0].slot, b.participant_slot);
    sim.step_once().expect("step");
    let need = read_need(&sim);
    assert!(need >= NEED_THR, "cross-row staged need={need}");
    assert_eq!(count_events(&mut sim, NEED_KIND), 1);
}

#[test]
fn two_bindings_same_participant_need_target_collision_fails_spanned() {
    let expected_span = first_participant_span(FOUNDRY_HIGH);
    let result = open_clause_opts(
        FOUNDRY_HIGH,
        OpenOpts {
            static_installed_source: true,
            second_binding_same_participant: true,
            ..Default::default()
        },
    );
    match result {
        Ok(_) => panic!("duplicate participant need target must fail admission"),
        Err(error) => {
            assert_eq!(error.span_token(), Some(expected_span), "{error}");
            let message = error.to_string();
            assert!(message.contains("need target collision"), "{message}");
            assert!(message.contains("participant slot"), "{message}");
            println!(
                "RF-5A duplicate need target rejected at span {expected_span}: {message}"
            );
        }
    }
}

// ── Live open / sealed events ──────────────────────────────────────────────

#[test]
fn open_step_paired_need_exact_event_counts() {
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
    // Production open already seeded Constants + uploaded thresholds + armed post-RF rescan.
    let b = &low.spec_state.resolved_need_bindings[0];
    assert_eq!(b.inputs[0].entity, "guild");
    assert_eq!(b.weights[0].entity, "guild");
    assert_ne!(b.inputs[0].col, b.need_col, "source cell ≠ need cell");

    low.step_once().expect("step");
    high.step_once().expect("step");
    let nl = read_need(&low);
    let nh = read_need(&high);
    assert!(nh > nl, "low={nl} high={nh}");
    assert!(nl < NEED_THR, "low={nl}");
    assert!(nh >= NEED_THR, "high={nh}");
    assert_eq!(
        count_events(&mut low, NEED_KIND),
        0,
        "below thr → zero need events"
    );
    assert_eq!(
        count_events(&mut high, NEED_KIND),
        1,
        "crossing → exactly one need event"
    );
}

#[test]
fn ordinary_unrelated_threshold_fires_exactly_once() {
    let mut sim = match open_clause_opts(
        FOUNDRY_HIGH,
        OpenOpts {
            ordinary_threshold_on_ore: true,
            ..Default::default()
        },
    ) {
        Ok(s) => s,
        Err(e) if e.to_lowercase().contains("adapter") => panic!("GPU FAIL: {e}"),
        Err(e) => panic!("{e}"),
    };
    let guild_id = sim.scenario.install_targets["guild"][0];
    let guild_slot = sim
        .proto
        .allocator
        .slot_of(guild_id)
        .expect("guild slot")
        .raw();
    let economy = sim
        .spec_state
        .resource_economy_registry
        .as_ref()
        .expect("economy registry");
    assert!(
        economy
            .registrations
            .emissions
            .iter()
            .filter(|emission| emission.source_slot == guild_slot)
            .count()
            == 2,
        "owner-silo emissions must materialize on authored guild row"
    );
    let threshold = economy
        .registrations
        .emit_on_threshold
        .iter()
        .find(|threshold| threshold.event_kind == ORD_KIND)
        .expect("ordinary threshold");
    assert_eq!(
        threshold.slot.raw(),
        guild_slot,
        "host-qualified threshold must materialize on authored guild row"
    );
    // The authored 0.25 drip crosses the ordinary 0.1 threshold on tick 1.
    // step_once includes both the full threshold scan and the post-RF need-only
    // append rescan, so the final buffer proves that the latter neither erases
    // nor duplicates the unrelated ordinary event.
    sim.step_once().expect("named crossing tick 1");
    {
        let runtime = sim
            .state
            .accumulator_runtime
            .as_mut()
            .expect("threshold runtime");
        let session = runtime.take_threshold_session().expect("threshold session");
        let emissions = session
            .readback_threshold_emissions(&sim.state.ctx)
            .expect("threshold emissions");
        println!(
            "RF-5A crossing tick 1 raw threshold reg_idx={:?}",
            emissions.iter().map(|event| event.reg_idx()).collect::<Vec<_>>()
        );
        let mut reg_indices = emissions
            .iter()
            .map(|event| event.reg_idx())
            .collect::<Vec<_>>();
        reg_indices.sort_unstable();
        reg_indices.dedup();
        assert_eq!(
            reg_indices.len(),
            2,
            "full-scan ordinary and need-only append must retain distinct registration identities"
        );
        runtime.restore_threshold_session(Some(session));
    }
    let ord = count_events(&mut sim, ORD_KIND);
    let need = count_events(&mut sim, NEED_KIND);
    assert_eq!(
        ord, 1,
        "kind {ORD_KIND} must remain exactly once after the post-RF need rescan"
    );
    assert_eq!(need, 1, "need kind {NEED_KIND} must emit exactly once");
    println!(
        "RF-5A crossing tick 1 exact counts: ordinary kind {ORD_KIND}={ord}; need kind {NEED_KIND}={need}"
    );
}

#[test]
fn aqueduct_second_scenario_same_generic_path() {
    let mut sim = match open_clause(AQUEDUCT) {
        Ok(s) => s,
        Err(e) if e.to_lowercase().contains("adapter") => panic!("GPU FAIL: {e}"),
        Err(e) => panic!("{e}"),
    };
    assert_eq!(sim.spec_state.resolved_need_bindings.len(), 1);
    sim.step_once().expect("step");
    assert!(read_need(&sim) > 0.0);
}

fn read_cell(sim: &SimSession, slot: u32, col: simthing_core::ColumnIndex) -> f32 {
    let v = sim.state.read_values();
    let n = sim.state.n_dims as usize;
    v[slot as usize * n + col.raw()]
}

fn read_staged_weight(sim: &SimSession) -> f32 {
    let b = &sim.spec_state.resolved_need_bindings[0];
    read_cell(sim, b.participant_slot, b.staged_weight_cols[0])
}

fn read_source_weight(sim: &SimSession) -> f32 {
    let b = &sim.spec_state.resolved_need_bindings[0];
    read_cell(sim, b.weights[0].slot, b.weights[0].col)
}

fn economy_owner_span(text: &str) -> usize {
    let doc = parse_raw_document(text.as_bytes()).expect("parse span fixture");
    let pack = hydrate_scenario(&doc).expect("hydrate span fixture");
    pack.field_economy
        .as_ref()
        .expect("field economy")
        .stockpile_silos[0]
        .owner_span_token
        .expect("owner span")
}

fn first_input_span(text: &str) -> usize {
    let doc = parse_raw_document(text.as_bytes()).expect("parse span fixture");
    let pack = hydrate_scenario(&doc).expect("hydrate span fixture");
    pack.field_economy
        .as_ref()
        .expect("field economy")
        .need_bindings[0]
        .inputs[0]
        .source_span_token
        .expect("input source span")
}

fn first_participant_span(text: &str) -> usize {
    let doc = parse_raw_document(text.as_bytes()).expect("parse span fixture");
    let pack = hydrate_scenario(&doc).expect("hydrate span fixture");
    pack.field_economy
        .as_ref()
        .expect("field economy")
        .need_bindings[0]
        .participant_span_token
        .expect("participant span")
}

fn assert_spanned_error(result: Result<SimSession, OpenError>, expected_span: usize) {
    match result {
        Ok(_) => panic!("expected spanned admission failure, got Ok"),
        Err(error) => assert_eq!(
            error.span_token(),
            Some(expected_span),
            "wrong/missing span for `{error}`"
        ),
    }
}

fn assert_measurement(label: &str, actual: f32, expected: f32) {
    const GPU_EPS: f32 = 1.0e-6;
    assert!(
        (actual - expected).abs() <= GPU_EPS,
        "{label}: expected {expected:.6}, observed {actual:.6}"
    );
}

/// LIVE-TRACKING: ordinary transfer drip changes source; stage + need track.
/// Exact closed form: ore=1.0 constant; weight starts 3.0; drip 0.25/tick.
/// need = ore * weight = weight.
#[test]
fn live_tracking_stage_and_need_follow_source() {
    let mut sim = match open_clause_opts(
        FOUNDRY_HIGH,
        OpenOpts {
            live_tracking_transfer: true,
            ..Default::default()
        },
    ) {
        Ok(s) => s,
        Err(e) if e.to_lowercase().contains("adapter") => panic!("GPU FAIL: {e}"),
        Err(e) => panic!("{e}"),
    };
    let guild_id = sim.scenario.install_targets["guild"][0];
    let guild_slot = sim
        .proto
        .allocator
        .slot_of(guild_id)
        .expect("guild slot")
        .raw();
    let transfer = &sim
        .spec_state
        .resource_economy_registry
        .as_ref()
        .expect("resource economy registry")
        .registrations
        .transfers[0];
    assert_eq!(
        transfer.source_slot.raw(),
        guild_slot,
        "host-qualified transfer source must materialize on authored guild row"
    );
    assert_eq!(
        transfer.target_slot.raw(),
        guild_slot,
        "host-qualified transfer target must materialize on authored guild row"
    );
    sim.step_once().expect("t1");
    let src1 = read_source_weight(&sim);
    let st1 = read_staged_weight(&sim);
    let n1 = read_need(&sim);
    // Independent closed form: source_t = 3.0 - 0.25*t; ore_t = 1.0;
    // stage_t = source_t; need_t = ore_t * stage_t.
    assert_measurement("LIVE tick1 source", src1, 2.75);
    assert_measurement("LIVE tick1 stage", st1, 2.75);
    assert_measurement("LIVE tick1 need", n1, 2.75);
    sim.step_once().expect("t2");
    let src2 = read_source_weight(&sim);
    let st2 = read_staged_weight(&sim);
    let n2 = read_need(&sim);
    assert_measurement("LIVE tick2 source", src2, 2.50);
    assert_measurement("LIVE tick2 stage", st2, 2.50);
    assert_measurement("LIVE tick2 need", n2, 2.50);
    println!(
        "LIVE exact: t1 source/stage/need={src1:.2}/{st1:.2}/{n1:.2}; t2={src2:.2}/{st2:.2}/{n2:.2}"
    );
}

/// DISCONNECT: remove only stage projections → sources keep moving, stage/need freeze.
#[test]
fn disconnect_control_freezes_stage_without_projection() {
    let mut sim = match open_clause_opts(
        FOUNDRY_HIGH,
        OpenOpts {
            live_tracking_transfer: true,
            ..Default::default()
        },
    ) {
        Ok(s) => s,
        Err(e) if e.to_lowercase().contains("adapter") => panic!("GPU FAIL: {e}"),
        Err(e) => panic!("{e}"),
    };
    sim.step_once().expect("prime");
    let st_before = read_staged_weight(&sim);
    let n_before = read_need(&sim);
    let src_before = read_source_weight(&sim);
    sim.harness_resync_resource_flow_without_need_stage_projections()
        .expect("harness disconnect resync");
    sim.step_once().expect("after disconnect");
    let src_after = read_source_weight(&sim);
    let st_after = read_staged_weight(&sim);
    let n_after = read_need(&sim);
    assert_measurement("DISCONNECT tick1 source", src_before, 2.75);
    assert_measurement("DISCONNECT tick1 stage", st_before, 2.75);
    assert_measurement("DISCONNECT tick1 need", n_before, 2.75);
    assert_measurement("DISCONNECT tick2 source", src_after, 2.50);
    assert_measurement("DISCONNECT tick2 frozen stage", st_after, 2.75);
    assert_measurement("DISCONNECT tick2 frozen need", n_after, 2.75);
    println!(
        "DISCONNECT exact: t1 source/stage/need={src_before:.2}/{st_before:.2}/{n_before:.2}; t2={src_after:.2}/{st_after:.2}/{n_after:.2}"
    );
}

/// STATIC: sources held static → staged cells and need remain static.
#[test]
fn static_control_holds_stage_and_need() {
    let mut sim = match open_clause_opts(
        FOUNDRY_HIGH,
        OpenOpts {
            static_installed_source: true,
            ..Default::default()
        },
    ) {
        Ok(s) => s,
        Err(e) if e.to_lowercase().contains("adapter") => panic!("GPU FAIL: {e}"),
        Err(e) => panic!("{e}"),
    };
    sim.step_once().expect("t1");
    let src1 = read_source_weight(&sim);
    let st1 = read_staged_weight(&sim);
    let n1 = read_need(&sim);
    println!("STATIC t1 observed source/stage/need={src1:.6}/{st1:.6}/{n1:.6}");
    // Independent closed form: installed weight remains 3.0 with economy
    // execution disabled; stage=3.0 and need=ore(1.0)*weight(3.0)=3.0.
    assert_measurement("STATIC tick1 source", src1, 3.0);
    assert_measurement("STATIC tick1 stage", st1, 3.0);
    assert_measurement("STATIC tick1 need", n1, 3.0);
    sim.step_once().expect("t2");
    let src2 = read_source_weight(&sim);
    let st2 = read_staged_weight(&sim);
    let n2 = read_need(&sim);
    assert_measurement("STATIC tick2 source", src2, 3.0);
    assert_measurement("STATIC tick2 stage", st2, 3.0);
    assert_measurement("STATIC tick2 need", n2, 3.0);
    println!(
        "STATIC exact: t1 source/stage/need={src1:.2}/{st1:.2}/{n1:.2}; t2={src2:.2}/{st2:.2}/{n2:.2}"
    );
}

#[test]
fn full_cell_source_of_authority_recorded() {
    let sim = open_clause(FOUNDRY_HIGH).expect("open");
    let b = &sim.spec_state.resolved_need_bindings[0];
    for cell in b.inputs.iter().chain(b.weights.iter()) {
        assert!(!cell.entity.is_empty());
        assert!(cell.slot > 0 || cell.simthing_id.raw() > 0);
        let _ = cell.col;
        let _ = &cell.role;
    }
    assert_eq!(b.eml_source_slot, b.participant_slot);
    assert!(!b.staged_input_cols.is_empty());
    assert!(!b.staged_weight_cols.is_empty());
}
