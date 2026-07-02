//! E-10 — Resource Flow admission framework tests.

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, DimensionRegistry, LogTier,
    NumCountSource, PropertyLayout, SimProperty, SimPropertyId, SubFieldRole, SubFieldSpec,
};
use simthing_driver::compile_and_materialize_resource_flow;
use simthing_spec::{
    compile_property, compile_resource_flow_admission, ArenaSpec, BaseFlowDirectionSpec,
    BaseFlowObligationSpec, CouplingDelaySpec, CouplingSpec, ExplicitParticipantSpec,
    FissionPolicySpec, InstallTargetSpec, PropertyKey, PropertySpec,
    ResourceFlowCapacityBudgetSpec, ResourceFlowSpec, SpecError, WildcardAdmissionSpec,
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
