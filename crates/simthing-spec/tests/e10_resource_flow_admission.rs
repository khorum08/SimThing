//! E-10 — Resource Flow admission framework tests.

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, DimensionRegistry, LogTier,
    NumCountSource, PropertyLayout, SimProperty, SimPropertyId, SubFieldRole, SubFieldSpec,
};
use simthing_driver::compile_and_materialize_resource_flow;
use simthing_spec::{
    compile_property, compile_resource_flow_admission, ArenaSpec, BaseFlowDirectionSpec,
    BaseFlowObligationSpec, CouplingDelaySpec, CouplingSpec, ExplicitParticipantSpec,
    FissionPolicySpec, InstallTargetSpec, PropertyKey, PropertySpec, ResourceFlowSpec, SpecError,
    WildcardAdmissionSpec,
};

fn flow_subfield(role_name: &str, accumulator: AccumulatorSpec) -> SubFieldSpec {
    SubFieldSpec {
        role: SubFieldRole::Named(role_name.into()),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: role_name.into(),
        display_range: None,
        governed_by: None,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: Some(accumulator),
    }
}

fn register_flow_property(reg: &mut DimensionRegistry, ns: &str, name: &str) -> SimPropertyId {
    let spec = PropertySpec {
        id: name.into(),
        namespace: ns.into(),
        name: name.into(),
        display_name: String::new(),
        description: String::new(),
        sub_fields: vec![flow_subfield(
            "flow",
            AccumulatorSpec {
                role: AccumulatorRole::IntrinsicFlow,
                log_tier: LogTier::Summary,
            },
        )],
    };
    compile_property(&spec, reg).unwrap().0
}

fn food_arena_spec(max_participants: u32) -> ArenaSpec {
    ArenaSpec {
        name: "food".into(),
        flow_property: PropertyKey::new("core", "food_flow"),
        balance_property: None,
        max_participants,
        max_coupling_fanout: 4,
        max_orderband_depth: 8,
        fission_policy: FissionPolicySpec::Reevaluate,
        reserved_orderband_depth: 0,
        reserved_gap_per_intermediate: 0,
        expected_max_children_per_intermediate: 0,
        explicit_participants: vec![ExplicitParticipantSpec::flat(1, 42)],
        enrollment: None,
        wildcard_admission: None,
    }
}

fn research_arena_spec() -> ArenaSpec {
    ArenaSpec {
        name: "research".into(),
        flow_property: PropertyKey::new("core", "research_flow"),
        balance_property: None,
        max_participants: 8,
        max_coupling_fanout: 4,
        max_orderband_depth: 8,
        fission_policy: FissionPolicySpec::Reevaluate,
        reserved_orderband_depth: 0,
        reserved_gap_per_intermediate: 0,
        expected_max_children_per_intermediate: 0,
        explicit_participants: vec![ExplicitParticipantSpec::flat(2, 43)],
        enrollment: None,
        wildcard_admission: None,
    }
}

fn setup_two_arena_registry() -> DimensionRegistry {
    let mut reg = DimensionRegistry::new();
    register_flow_property(&mut reg, "core", "food_flow");
    register_flow_property(&mut reg, "core", "research_flow");
    register_flow_property(&mut reg, "core", "suppression_flow");
    reg
}

fn base_obligation(id: &str, arena: &str, rate: f32) -> BaseFlowObligationSpec {
    BaseFlowObligationSpec {
        id: id.into(),
        arena: arena.into(),
        install: InstallTargetSpec::ScenarioListed {
            target_id: "producer".into(),
        },
        direction: BaseFlowDirectionSpec::Produce,
        rate,
    }
}

#[test]
fn e10_rejects_implicit_participation() {
    let reg = setup_two_arena_registry();
    let mut arena = food_arena_spec(4);
    arena.explicit_participants.clear();
    arena.wildcard_admission = None;
    let spec = ResourceFlowSpec {
        arenas: vec![arena],
        couplings: vec![],
        ..Default::default()
    };
    let err = compile_resource_flow_admission(&spec, &reg).unwrap_err();
    assert!(matches!(err, SpecError::ImplicitParticipation { .. }));

    let err = compile_and_materialize_resource_flow(&spec, &reg).unwrap_err();
    assert!(matches!(err, SpecError::ImplicitParticipation { .. }));
}

#[test]
fn e10_accepts_base_intrinsic_flow_obligation_authoring() {
    let reg = setup_two_arena_registry();
    let spec = ResourceFlowSpec {
        arenas: vec![food_arena_spec(4)],
        couplings: vec![],
        base_obligations: vec![
            base_obligation("food_produce", "food", 10.0),
            BaseFlowObligationSpec {
                id: "food_upkeep".into(),
                arena: "food".into(),
                install: InstallTargetSpec::ScenarioListed {
                    target_id: "producer".into(),
                },
                direction: BaseFlowDirectionSpec::Upkeep,
                rate: 2.0,
            },
        ],
        ..Default::default()
    };
    let compiled = compile_resource_flow_admission(&spec, &reg).expect("compile");
    assert_eq!(compiled.arenas.len(), 1);
}

#[test]
fn e10_rejects_duplicate_base_intrinsic_flow_obligation_ids() {
    let reg = setup_two_arena_registry();
    let spec = ResourceFlowSpec {
        arenas: vec![food_arena_spec(4)],
        couplings: vec![],
        base_obligations: vec![
            base_obligation("food_base", "food", 10.0),
            base_obligation("food_base", "food", 1.0),
        ],
        ..Default::default()
    };
    let err = compile_resource_flow_admission(&spec, &reg).unwrap_err();
    assert!(matches!(
        err,
        SpecError::DuplicateBaseFlowObligation { id } if id == "food_base"
    ));
}

#[test]
fn e10_rejects_base_intrinsic_flow_obligation_unknown_arena() {
    let reg = setup_two_arena_registry();
    let spec = ResourceFlowSpec {
        arenas: vec![food_arena_spec(4)],
        couplings: vec![],
        base_obligations: vec![base_obligation("missing", "energy", 1.0)],
        ..Default::default()
    };
    let err = compile_resource_flow_admission(&spec, &reg).unwrap_err();
    assert!(matches!(
        err,
        SpecError::UnknownArenaReference { arena, .. } if arena == "energy"
    ));
}

#[test]
fn e10_rejects_base_intrinsic_flow_obligation_invalid_rate() {
    let reg = setup_two_arena_registry();
    let spec = ResourceFlowSpec {
        arenas: vec![food_arena_spec(4)],
        couplings: vec![],
        base_obligations: vec![base_obligation("bad_rate", "food", f32::NAN)],
        ..Default::default()
    };
    let err = compile_resource_flow_admission(&spec, &reg).unwrap_err();
    assert!(matches!(
        err,
        SpecError::InvalidBaseFlowObligationRate { id } if id == "bad_rate"
    ));
}

#[test]
fn e10_rejects_unknown_arena_role_reference() {
    let mut reg = DimensionRegistry::new();
    let spec = PropertySpec {
        id: "participant".into(),
        namespace: "core".into(),
        name: "planet_food".into(),
        display_name: String::new(),
        description: String::new(),
        sub_fields: vec![flow_subfield(
            "allocated",
            AccumulatorSpec {
                role: AccumulatorRole::AllocatedFlow {
                    arena: "missing_arena".into(),
                },
                log_tier: LogTier::Summary,
            },
        )],
    };
    compile_property(&spec, &mut reg).unwrap();
    register_flow_property(&mut reg, "core", "food_flow");

    let flow_spec = ResourceFlowSpec {
        arenas: vec![food_arena_spec(4)],
        couplings: vec![],
        ..Default::default()
    };
    let err = compile_resource_flow_admission(&flow_spec, &reg).unwrap_err();
    assert!(matches!(
        err,
        SpecError::UnknownArenaRoleReference { arena, .. } if arena == "missing_arena"
    ));
}

#[test]
fn e10_rejects_unbounded_wildcard_without_cap() {
    let reg = setup_two_arena_registry();
    let mut arena = food_arena_spec(4);
    arena.explicit_participants.clear();
    arena.wildcard_admission = Some(WildcardAdmissionSpec {
        max_expansion: None,
        expanded_count: 0,
    });
    let spec = ResourceFlowSpec {
        arenas: vec![arena],
        couplings: vec![],
        ..Default::default()
    };
    let err = compile_resource_flow_admission(&spec, &reg).unwrap_err();
    assert!(matches!(err, SpecError::UnboundedWildcardAdmission { .. }));
}

#[test]
fn e10_enforces_max_participants() {
    let reg = setup_two_arena_registry();
    let mut arena = food_arena_spec(1);
    arena
        .explicit_participants
        .push(ExplicitParticipantSpec::flat(2, 99));
    let spec = ResourceFlowSpec {
        arenas: vec![arena],
        couplings: vec![],
        ..Default::default()
    };
    let err = compile_and_materialize_resource_flow(&spec, &reg).unwrap_err();
    assert!(matches!(err, SpecError::MaxParticipantsExceeded { .. }));
}

#[test]
fn e10_rejects_all_algebraic_coupling_cycle() {
    let reg = setup_two_arena_registry();
    let mut suppression = food_arena_spec(4);
    suppression.name = "suppression".into();
    suppression.flow_property = PropertyKey::new("core", "suppression_flow");
    suppression.explicit_participants = vec![ExplicitParticipantSpec::flat(3, 44)];

    let spec = ResourceFlowSpec {
        arenas: vec![food_arena_spec(4), research_arena_spec(), suppression],
        couplings: vec![
            CouplingSpec {
                from_arena: "food".into(),
                to_arena: "research".into(),
                delay: CouplingDelaySpec::Algebraic,
            },
            CouplingSpec {
                from_arena: "research".into(),
                to_arena: "suppression".into(),
                delay: CouplingDelaySpec::Algebraic,
            },
            CouplingSpec {
                from_arena: "suppression".into(),
                to_arena: "food".into(),
                delay: CouplingDelaySpec::Algebraic,
            },
        ],
        ..Default::default()
    };
    let err = compile_and_materialize_resource_flow(&spec, &reg).unwrap_err();
    assert!(matches!(err, SpecError::AllAlgebraicCouplingCycle));
}

#[test]
fn e10_allows_cycle_with_delay_edge() {
    let reg = setup_two_arena_registry();
    let spec = ResourceFlowSpec {
        arenas: vec![food_arena_spec(4), research_arena_spec()],
        couplings: vec![
            CouplingSpec {
                from_arena: "food".into(),
                to_arena: "research".into(),
                delay: CouplingDelaySpec::Algebraic,
            },
            CouplingSpec {
                from_arena: "research".into(),
                to_arena: "food".into(),
                delay: CouplingDelaySpec::OneTickDelay,
            },
        ],
        ..Default::default()
    };
    assert!(compile_and_materialize_resource_flow(&spec, &reg).is_ok());
}

#[test]
fn e10_rejects_hidden_fanout() {
    let reg = setup_two_arena_registry();
    let mut food = food_arena_spec(4);
    food.max_coupling_fanout = 1;
    let spec = ResourceFlowSpec {
        arenas: vec![food, research_arena_spec()],
        couplings: vec![
            CouplingSpec {
                from_arena: "food".into(),
                to_arena: "research".into(),
                delay: CouplingDelaySpec::Algebraic,
            },
            CouplingSpec {
                from_arena: "research".into(),
                to_arena: "food".into(),
                delay: CouplingDelaySpec::OneTickDelay,
            },
        ],
        ..Default::default()
    };
    let err = compile_and_materialize_resource_flow(&spec, &reg).unwrap_err();
    assert!(matches!(err, SpecError::HiddenFanoutExceeded { .. }));
}

#[test]
fn e10_rejects_orderband_budget_excess() {
    let reg = setup_two_arena_registry();
    let mut arena = food_arena_spec(4);
    arena.reserved_orderband_depth = 9;
    arena.max_orderband_depth = 8;
    let spec = ResourceFlowSpec {
        arenas: vec![arena],
        couplings: vec![],
        ..Default::default()
    };
    let err = compile_and_materialize_resource_flow(&spec, &reg).unwrap_err();
    assert!(matches!(err, SpecError::MaxOrderBandDepthExceeded { .. }));
}

#[test]
fn e10_rejects_unresolved_balance_num_count_source() {
    let mut reg = DimensionRegistry::new();
    register_flow_property(&mut reg, "core", "food_flow");
    let balance_spec = PropertySpec {
        id: "debt".into(),
        namespace: "core".into(),
        name: "food_balance".into(),
        display_name: String::new(),
        description: String::new(),
        sub_fields: vec![flow_subfield(
            "balance",
            AccumulatorSpec {
                role: AccumulatorRole::Balance(BalanceSpec {
                    unit_cost: Some(1.0),
                    num_count_source: Some(NumCountSource::Column {
                        property_id: SimPropertyId(999),
                        role: SubFieldRole::Amount,
                    }),
                }),
                log_tier: LogTier::Summary,
            },
        )],
    };
    compile_property(&balance_spec, &mut reg).unwrap();

    let spec = ResourceFlowSpec {
        arenas: vec![food_arena_spec(4)],
        couplings: vec![],
        ..Default::default()
    };
    let err = compile_resource_flow_admission(&spec, &reg).unwrap_err();
    assert!(matches!(
        err,
        SpecError::UnresolvedBalanceNumCountSource { .. }
    ));
}

#[test]
fn e10_expansion_report_is_stable() {
    let reg = setup_two_arena_registry();
    let spec = ResourceFlowSpec {
        arenas: vec![food_arena_spec(4), research_arena_spec()],
        couplings: vec![CouplingSpec {
            from_arena: "food".into(),
            to_arena: "research".into(),
            delay: CouplingDelaySpec::OneTickDelay,
        }],
        ..Default::default()
    };
    let (_, report_a) = compile_and_materialize_resource_flow(&spec, &reg).unwrap();
    let (_, report_b) = compile_and_materialize_resource_flow(&spec, &reg).unwrap();
    assert_eq!(report_a, report_b);
    assert_eq!(report_a.arena_count, 2);
    assert_eq!(report_a.participant_count, 2);
    assert_eq!(report_a.coupling_count, 1);
    assert_eq!(
        report_a.per_arena_participant_counts,
        vec![("food".into(), 1), ("research".into(), 1)]
    );
    assert_eq!(report_a.total_orderband_depth_reserved, 0);
    assert_eq!(report_a.total_registration_estimate, Some(3));
}

#[test]
fn e10_does_not_import_arena_registry_into_simthing_sim() {
    let sim_cargo = include_str!("../../simthing-sim/Cargo.toml");
    assert!(
        !sim_cargo.contains("simthing-driver"),
        "simthing-sim must not depend on simthing-driver"
    );
    assert!(
        !sim_cargo.contains("simthing-mapeditor"),
        "simthing-sim must not depend on simthing-mapeditor"
    );
    assert!(
        !sim_cargo.contains("simthing-spec"),
        "simthing-sim must not depend on simthing-spec"
    );
    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("ArenaRegistry"));
    assert!(!sim_lib.contains("arena_registry"));

    let mapping_plan_compile_src =
        include_str!("../../simthing-driver/src/mapping_plan_compile.rs");
    assert!(!mapping_plan_compile_src.contains("deserialize_scenario_authority"));
    assert!(!mapping_plan_compile_src.contains("include_str!("));
    assert!(!mapping_plan_compile_src.contains("StructuredFieldStencilOp"));
    assert!(!mapping_plan_compile_src.contains("simthing_mapeditor"));
    for token in [
        "pathfinding",
        "predecessor",
        "came_from",
        "route_object",
        "border_service",
        "frontline_service",
        "cpu_planner",
    ] {
        assert!(
            !mapping_plan_compile_src.contains(token),
            "mapping_plan_compile must not reference `{token}`"
        );
    }

    let mapping_tick_src = include_str!("../../simthing-sim/src/mapping_plan_tick.rs");
    for token in [
        "simthing_driver",
        "simthing_spec",
        "simthing_mapeditor",
        "SimThingScenarioSpec",
        "deserialize_scenario_authority",
        "pathfinding",
        "predecessor",
        "came_from",
        "route_object",
        "border_service",
        "frontline_service",
        "cpu_planner",
    ] {
        assert!(
            !mapping_tick_src.contains(token),
            "mapping_plan_tick must not reference `{token}`"
        );
    }
    assert!(
        mapping_tick_src.contains("SimGpuMappingReadbackPolicy::None => None"),
        "mapping None policy must not populate proof_values"
    );
    assert!(
        mapping_tick_src.contains("MinPlusTraversalExecutionMode::GpuResident"),
        "MinPlus None must use GpuResident"
    );
    assert!(
        mapping_tick_src.contains("MinPlusTraversalExecutionMode::DiagnosticReadback"),
        "MinPlus ProofReadback must use DiagnosticReadback"
    );
    assert!(
        mapping_tick_src.contains("scoped_debug_readback_allowed"),
        "structured-field proof readback must use scoped guard"
    );
    let tick_start = mapping_tick_src.find("pub fn tick(").expect("mapping tick");
    let tick_end = mapping_tick_src[tick_start..]
        .find("\nfn structured_field_values_buffer")
        .expect("mapping tick end");
    let tick_body = &mapping_tick_src[tick_start..tick_start + tick_end];
    assert!(
        !tick_body.contains("set_debug_readback_allowed"),
        "mapping tick must not silently enable debug readback"
    );
    assert!(
        tick_body.contains("if readback == SimGpuMappingReadbackPolicy::ProofReadback"),
        "mapping proof readback must be explicit"
    );

    let atlas_partition_src =
        include_str!("../../simthing-driver/src/structural_n4_atlas_partition.rs");
    for token in [
        "simthing_sim",
        "SimGpuMappingAtlasScheduler",
        "SimGpuMappingTickState",
        "StructuredFieldStencilOp",
        "simthing_mapeditor",
        "pathfinding",
        "predecessor",
        "came_from",
        "route_object",
        "border_service",
        "frontline_service",
        "cpu_planner",
    ] {
        assert!(
            !atlas_partition_src.contains(token),
            "structural_n4_atlas_partition must not reference `{token}`"
        );
    }

    let atlas_scheduler_src = include_str!("../../simthing-sim/src/mapping_atlas_scheduler.rs");
    for token in [
        "simthing_driver",
        "simthing_spec",
        "simthing_mapeditor",
        "SimThingScenarioSpec",
        "deserialize_scenario_authority",
        "structural_grid",
        "pathfinding",
        "predecessor",
        "came_from",
        "route_object",
        "border_service",
        "frontline_service",
        "cpu_planner",
    ] {
        assert!(
            !atlas_scheduler_src.contains(token),
            "mapping_atlas_scheduler must not reference `{token}`"
        );
    }
    assert!(
        atlas_scheduler_src.contains("state.tick(ctx, theater_input, readback)"),
        "atlas scheduler must delegate readback policy to resident tick state"
    );
    assert!(
        !atlas_scheduler_src.contains("readback_after_ping_pong"),
        "atlas scheduler must not call proof readback helpers directly"
    );
    assert!(
        !atlas_scheduler_src.contains("set_debug_readback_allowed"),
        "atlas scheduler must not silently enable debug readback"
    );

    let forbidden_test_imports = [
        "simthing_driver",
        "simthing_mapeditor",
        "simthing_spec",
        "deserialize_scenario_authority",
        "include_str!(\"../../../scenarios/",
    ];
    let sim_tests =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../simthing-sim/tests");
    walk_sim_tests_for_forbidden_imports(&sim_tests, &forbidden_test_imports);
}

fn walk_sim_tests_for_forbidden_imports(dir: &std::path::Path, forbidden: &[&str]) {
    for entry in std::fs::read_dir(dir)
        .unwrap_or_else(|err| panic!("read simthing-sim tests dir {}: {err}", dir.display()))
    {
        let entry = entry.expect("simthing-sim tests entry");
        let path = entry.path();
        if path.is_dir() {
            walk_sim_tests_for_forbidden_imports(&path, forbidden);
            continue;
        }
        if path.extension().is_some_and(|ext| ext == "rs") {
            let source = std::fs::read_to_string(&path)
                .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
            for token in forbidden {
                assert!(
                    !source.contains(token),
                    "{} must not reference upward seam token `{token}`",
                    path.display()
                );
            }
        }
    }
}

#[test]
fn e10_owner_doctrine_and_evidence_reclassification_guards() {
    let simthing_src = include_str!("../../simthing-core/src/simthing.rs");
    for forbidden in [
        "overlays, not nodes",
        "non-physical groupings are overlays",
        "Political structures, factions",
        "factions, and all non-physical groupings are overlays",
    ] {
        assert!(
            !simthing_src.contains(forbidden),
            "simthing.rs must not treat owner/faction entities as overlays-only: found `{forbidden}`"
        );
    }
    assert!(
        simthing_src.contains("sibling children of the Session root"),
        "simthing.rs must state owner entities are Session children"
    );
    assert!(
        simthing_src.contains("not overlays and not spatial parents"),
        "simthing.rs must state owners are not overlays and not spatial parents"
    );
    assert!(
        simthing_src.contains("SimThingKind::Owner"),
        "simthing.rs must admit canonical SimThingKind::Owner"
    );

    let constitution = include_str!("../../../docs/design_0_0_8_3.md");
    assert!(
        constitution.contains("Terminology correction — owner, not faction"),
        "active constitution must carry owner-not-faction terminology correction"
    );
    let section_0_end = constitution
        .find("### 0.1 Maximal SimThing conformance")
        .expect("§0.1 heading");
    let section_0 = &constitution[..section_0_end];
    let lower = section_0.to_ascii_lowercase();
    assert!(
        !lower.contains("factions are"),
        "§0 must not use bare factions as constitutional ontology"
    );
    assert!(
        !lower.contains("faction entities"),
        "§0 must not use faction entities as constitutional ontology"
    );
    assert!(
        section_0.contains("owner-faction"),
        "§0 may retain owner-faction as legacy example language only"
    );

    let evidence_index = include_str!("../../../docs/tests/current_evidence_index.md");
    assert!(
        evidence_index.contains("Lower-layer golden-fixture doctrine"),
        "evidence index must document lower-layer golden-fixture doctrine"
    );
    assert!(
        evidence_index.contains(
            "Terran Pirate is a golden fixture for lower-layer compile/scheduler/GPU proofs"
        ),
        "evidence index must state Terran Pirate golden-fixture doctrine"
    );
    for row in [
        "TERRAN-PIRATE-SCENARIO-SKELETON-0",
        "TERRAN-PIRATE-MAPPING-FIRST-SLICE-0",
        "STRUCTURAL-N4-THEATER-COMPILE-0",
        "DRIVER-STRUCTURAL-ATLAS-HALO-0",
    ] {
        let marker = format!("**{row}**");
        let row_start = evidence_index
            .find(&marker)
            .unwrap_or_else(|| panic!("evidence index missing row {row}"));
        let row_line_end = evidence_index[row_start..]
            .find('\n')
            .map(|offset| row_start + offset)
            .unwrap_or(evidence_index.len());
        let row_line = &evidence_index[row_start..row_line_end];
        assert!(
            row_line.contains("LOWER_LAYER_GOLDEN_FIXTURE"),
            "evidence row {row} must be tagged LOWER_LAYER_GOLDEN_FIXTURE"
        );
        assert!(
            row_line.contains("not scenario ontology"),
            "evidence row {row} must deny scenario-ontology completion"
        );
    }
    assert!(
        evidence_index.contains(
            "Future main-track scenario PRs must introduce or generalize scenario/session/owner/map ingestion capability"
        ),
        "evidence index must carry hygiene-kabuki relapse guardrail"
    );

    let scenario_src = include_str!("../src/spec/scenario.rs");
    assert!(
        scenario_src.contains("Canonical save/load authority")
            && scenario_src.contains("Scenario")
            && scenario_src.contains("file root"),
        "scenario.rs must document Scenario SimThing as canonical file root"
    );
    assert!(
        scenario_src.contains("Transitional serde mirror"),
        "scenario.rs must mark sidecar fields as transitional"
    );
    assert!(
        scenario_src.contains("validate_scenario_root_authority"),
        "scenario.rs must expose canonical Scenario-root validation"
    );
    assert!(
        scenario_src.contains("validate_legacy_world_root_compatibility"),
        "scenario.rs must expose explicit legacy World-root compatibility"
    );
    assert!(
        scenario_src.contains("LegacyWorldRootAdmitted"),
        "legacy World admission must be a named compatibility outcome"
    );
    assert!(
        !scenario_src.contains("root.kind != SimThingKind::World")
            || scenario_src.contains("spatial_authority_root"),
        "STEAD validation must not treat bare World root as the only canonical path"
    );
    assert!(
        simthing_src.contains("SimThingKind::Scenario"),
        "simthing-core must admit SimThingKind::Scenario for canonical file roots"
    );
    assert!(
        !scenario_src.contains("(seed & 0xFFFF_FFFF) as f32"),
        "scenario.rs must not use lossy 32-bit-half f32 seed encoding"
    );
    assert!(
        !scenario_src.contains("(seed >> 32) as f32"),
        "scenario.rs must not use lossy 32-bit-half f32 seed encoding"
    );

    let scenario_root_test_src = include_str!("scenario_serializable_simthing_root.rs");
    assert!(
        scenario_root_test_src.contains("scenario_seed_roundtrips_u64_max"),
        "scenario seed tests must cover u64::MAX"
    );
    assert!(
        scenario_root_test_src.contains("scenario_seed_roundtrips_low_high_mixed_pattern"),
        "scenario seed tests must cover mixed high/low bit pattern"
    );
    assert!(
        simthing_src.contains("SimThingKind::GameSession"),
        "simthing-core must admit SimThingKind::GameSession for canonical session roots"
    );
    assert!(
        scenario_src.contains("validate_scenario_game_session_child"),
        "scenario.rs must expose GameSession child validation"
    );
    assert!(
        scenario_src.contains("MissingGameSessionChild"),
        "canonical Scenario validation must require GameSession child"
    );

    let production_doc = include_str!("../../../docs/0.8.3 Simthing Studio Production.md");
    assert!(
        production_doc.contains("root: Scenario") && production_doc.contains("GameSession"),
        "production synthesis current authority must be Scenario -> GameSession"
    );
    let authority_section = production_doc
        .split("## Generated Galaxy Authority")
        .nth(1)
        .and_then(|s| s.split("## ").next())
        .unwrap_or("");
    assert!(
        !authority_section.contains("root: World"),
        "production synthesis current authority must not present root: World as canonical"
    );
    assert!(
        production_doc.contains("lower-layer golden fixture"),
        "production synthesis must classify Terran Pirate as lower-layer golden fixture"
    );
    assert!(
        scenario_src.contains("validate_session_owner_entities"),
        "scenario.rs must expose Owner entity validation"
    );
    assert!(
        production_doc.contains("-> Owner(s)")
            || (production_doc.contains("GameSession") && production_doc.contains("Owner(s)")),
        "production synthesis current authority must be Scenario -> GameSession -> Owner"
    );
    assert!(
        !authority_section
            .to_ascii_lowercase()
            .contains("owners are overlays"),
        "active doctrine must not say Owners are overlays"
    );
    assert!(
        !authority_section
            .to_ascii_lowercase()
            .contains("owners are spatial parents")
            && !authority_section
                .to_ascii_lowercase()
                .contains("owners as spatial parents"),
        "active authority summary must not present Owners as spatial parents"
    );
    assert!(
        scenario_src.contains("validate_session_galaxy_map"),
        "scenario.rs must expose GalaxyMap / WorldStateMap validation"
    );
    assert!(
        production_doc.contains("GalaxyMap")
            && production_doc.contains("Owner(s)")
            && production_doc.contains("GameSession"),
        "production synthesis current authority must be Scenario -> GameSession -> Owner(s) -> GalaxyMap"
    );
    assert!(
        !authority_section
            .to_ascii_lowercase()
            .contains("world is the canonical spatial root"),
        "production synthesis must not present World as the canonical spatial root"
    );
    let owner_test_src = include_str!("scenario_owner_entities.rs");
    assert!(
        owner_test_src.contains("owner_nested_under_galaxymap_is_rejected"),
        "Owner directness must reject nested Owner under GalaxyMap"
    );
    for forbidden_engine in [
        "OwnerEngine",
        "FactionEngine",
        "GalaxyMapEngine",
        "WorldEngine",
    ] {
        assert!(
            !simthing_src.contains(forbidden_engine) && !scenario_src.contains(forbidden_engine),
            "core/spec must not introduce {forbidden_engine}"
        );
    }
    let sim_src = include_str!("../../simthing-sim/src/lib.rs");
    assert!(
        !sim_src.contains("OwnerEngine")
            && !sim_src.contains("FactionEngine")
            && !sim_src.contains("GalaxyMapEngine")
            && !sim_src.contains("WorldEngine"),
        "simthing-sim must not introduce Owner/GalaxyMap/World engines"
    );

    let ingestion_src = include_str!("../src/spec/scenario_ingestion.rs");
    assert!(
        ingestion_src.contains("ingest_scenario_from_str")
            && ingestion_src.contains("ingest_scenario"),
        "scenario ingestion API must exist in simthing-spec"
    );
    assert!(
        ingestion_src.contains("ScenarioIngestionClassification")
            && ingestion_src.contains("Admitted")
            && ingestion_src.contains("PartiallyAdmitted")
            && ingestion_src.contains("Rejected")
            && ingestion_src.contains("Unsupported"),
        "ingestion must expose Admitted / PartiallyAdmitted / Rejected / Unsupported"
    );
    assert!(
        ingestion_src.contains("ScenarioDeferralKind")
            && ingestion_src.contains("LegacyWorldRootCompatibility")
            && ingestion_src.contains("PlanetsNotYetAdmitted"),
        "ingestion must expose typed deferrals"
    );
    assert!(
        production_doc.contains("GENERAL-SCENARIO-INGESTION-ADMISSION-0")
            && production_doc.contains("arbitrary Scenario ingestion"),
        "production synthesis must name GENERAL-SCENARIO-INGESTION-ADMISSION-0"
    );
    let gpu_lib = include_str!("../../simthing-gpu/src/lib.rs");
    assert!(
        !gpu_lib.contains("ingest_scenario") && !gpu_lib.contains("ScenarioIngestion"),
        "ingestion authority must not live in simthing-gpu"
    );
    let mapeditor_lib = include_str!("../../simthing-mapeditor/src/lib.rs");
    assert!(
        !mapeditor_lib.contains("ingest_scenario_from_str")
            && !mapeditor_lib.contains("ScenarioIngestionResult"),
        "mapeditor/Studio must not own ingestion authority"
    );
    for forbidden_engine in [
        "ScenarioEngine",
        "IngestionEngine",
        "OwnerEngine",
        "FactionEngine",
        "GalaxyMapEngine",
        "WorldEngine",
        "EconomyEngine",
        "SiloEngine",
        "StockpileEngine",
    ] {
        assert!(
            !simthing_src.contains(forbidden_engine)
                && !scenario_src.contains(forbidden_engine)
                && !ingestion_src.contains(forbidden_engine),
            "core/spec ingestion must not introduce {forbidden_engine}"
        );
    }

    let session_flow_src = include_str!("../src/spec/session_resource_flow.rs");
    assert!(
        session_flow_src.contains("evaluate_owner_silo_flow"),
        "owner-silo flow API must exist in simthing-spec"
    );
    assert!(
        scenario_src.contains("OWNER_FLOW_OWNER_REF_PROPERTY_ID")
            && scenario_src.contains("apply_participant_owner_flow_metadata"),
        "owner references must be properties/columns, not spatial parenting"
    );
    assert!(
        session_flow_src.contains("not spatial parenting"),
        "owner-silo flow must document property-based ownership"
    );
    assert!(
        production_doc.contains("SESSION-RESOURCE-FLOW-SILOS-0"),
        "production synthesis must name SESSION-RESOURCE-FLOW-SILOS-0"
    );
    let driver_silo_src = include_str!("../../simthing-driver/src/session_resource_flow_silos.rs");
    assert!(
        driver_silo_src.contains("compile_resource_flow_admission")
            && driver_silo_src.contains("explicit_participants"),
        "driver owner-silo materialization must reuse explicit-participant ResourceFlow admission"
    );
    assert!(
        !driver_silo_src.contains("OwnerEngine")
            && !session_flow_src.contains("OwnerEngine")
            && !session_flow_src.contains("FactionEngine"),
        "owner-silo flow must not use OwnerEngine/FactionEngine"
    );
    let silo_test_src = include_str!("session_resource_flow_silos.rs");
    assert!(
        silo_test_src.contains("owner_silo_does_not_require_owner_spatial_parenting"),
        "owner-silo tests must guard against spatial-parent ownership"
    );
    for forbidden_engine in [
        "OwnerEngine",
        "FactionEngine",
        "EconomyEngine",
        "SiloEngine",
        "StockpileEngine",
    ] {
        assert!(
            !sim_src.contains(forbidden_engine)
                && !driver_silo_src.contains(forbidden_engine)
                && !session_flow_src.contains(forbidden_engine),
            "core/spec/driver/sim must not introduce {forbidden_engine}"
        );
    }
}

#[test]
fn e10_rejects_property_possession_without_explicit_admission() {
    let mut reg = DimensionRegistry::new();
    register_flow_property(&mut reg, "core", "food_flow");
    let participant = PropertySpec {
        id: "planet".into(),
        namespace: "core".into(),
        name: "planet_food".into(),
        display_name: String::new(),
        description: String::new(),
        sub_fields: vec![flow_subfield(
            "allocated",
            AccumulatorSpec {
                role: AccumulatorRole::AllocatedFlow {
                    arena: "food".into(),
                },
                log_tier: LogTier::Summary,
            },
        )],
    };
    compile_property(&participant, &mut reg).unwrap();

    let mut arena = food_arena_spec(4);
    arena.explicit_participants.clear();
    arena.wildcard_admission = None;

    let spec = ResourceFlowSpec {
        arenas: vec![arena],
        couplings: vec![],
        ..Default::default()
    };
    let err = compile_resource_flow_admission(&spec, &reg).unwrap_err();
    assert!(matches!(
        err,
        SpecError::PropertyPossessionNotArenaAdmission { .. }
            | SpecError::ImplicitParticipation { .. }
    ));
}

#[test]
fn e10_rejects_duplicate_arena_role_binding_on_same_property() {
    let mut reg = DimensionRegistry::new();
    register_flow_property(&mut reg, "core", "food_flow");
    let prop = SimProperty {
        namespace: "core".into(),
        name: "dual_bind".into(),
        layout: PropertyLayout {
            sub_fields: vec![
                flow_subfield(
                    "alloc_a",
                    AccumulatorSpec {
                        role: AccumulatorRole::AllocatedFlow {
                            arena: "food".into(),
                        },
                        log_tier: LogTier::Summary,
                    },
                ),
                flow_subfield(
                    "alloc_b",
                    AccumulatorSpec {
                        role: AccumulatorRole::AllocatedFlow {
                            arena: "food".into(),
                        },
                        log_tier: LogTier::Summary,
                    },
                ),
            ],
        },
        decay: None,
        intensity_behavior: None,
        fission_templates: vec![],
        fusion_templates: vec![],
        on_expire: None,
        description: String::new(),
        intensity_labels: vec![],
    };
    reg.register(prop);

    let spec = ResourceFlowSpec {
        arenas: vec![food_arena_spec(4)],
        couplings: vec![],
        ..Default::default()
    };
    let err = compile_resource_flow_admission(&spec, &reg).unwrap_err();
    assert!(matches!(err, SpecError::DuplicateArenaRoleBinding { .. }));
}
