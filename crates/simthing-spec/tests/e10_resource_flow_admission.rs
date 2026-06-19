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
