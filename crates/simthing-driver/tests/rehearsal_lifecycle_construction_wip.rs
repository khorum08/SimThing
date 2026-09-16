//! Leaf A stock-settlement prerequisite, Boards 5689832213 / 5689833128.
//! Canonical Balance is governed by a distinct balance_rate, as in ClauseThing
//! hydration. Required-success assertions preserve the RF leaf-residual law.
//! A production settlement failure leaves Model 1 undecided and Model 2 closed.
use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, DimensionRegistry, LogTier,
    PropertyValue, SimPropertyId, SimThing, SimThingId, SimThingKind, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{
    compile_and_materialize_resource_flow, derive_resource_flow_admission,
    sync_resource_flow_accumulator, Scenario, SimSession,
};
use simthing_gpu::{GpuContext, SlotAllocator, WorldGpuState};
use simthing_spec::{compile_property, GameModeSpec, PropertySpec, ResourceFlowSpec};

fn role(name: &str) -> SubFieldRole {
    SubFieldRole::Named(name.into())
}

fn property(registry: &mut DimensionRegistry, name: &str) -> SimPropertyId {
    let field = |name: &str, accumulator: Option<AccumulatorRole>| SubFieldSpec {
        role: role(name),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: name.into(),
        display_range: None,
        governed_by: None,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: accumulator.map(|role| AccumulatorSpec {
            role,
            log_tier: LogTier::Summary,
        }),
    };
    let mut balance = field(
        "balance",
        Some(AccumulatorRole::Balance(BalanceSpec::default())),
    );
    balance.governed_by = Some(role("balance_rate"));
    compile_property(
        &PropertySpec {
            admission_disposition: Default::default(),
            id: name.into(),
            namespace: "leaf_a".into(),
            name: name.into(),
            display_name: name.into(),
            description: String::new(),
            sub_fields: vec![
                field("flow", Some(AccumulatorRole::IntrinsicFlow)),
                field(
                    "allocated",
                    Some(AccumulatorRole::AllocatedFlow { arena: name.into() }),
                ),
                field(
                    "weight",
                    Some(AccumulatorRole::AllocatorWeight { arena: name.into() }),
                ),
                field("balance_rate", None),
                balance,
            ],
        },
        registry,
    )
    .expect("ordinary property admission")
    .0
}

#[derive(Clone)]
struct Fixture {
    scenario: Scenario,
    resources: Vec<SimPropertyId>,
    projects: [SimThingId; 2],
}

fn fixture(names: &[&str], zero_weights: bool, seeded_rate: bool) -> Fixture {
    let mut registry = DimensionRegistry::new();
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut p = SimThing::new(SimThingKind::Cohort, 0);
    let mut q = SimThing::new(SimThingKind::Cohort, 0);
    let mut resources = Vec::new();
    for &name in names {
        let pid = property(&mut registry, name);
        resources.push(pid);
        let layout = &registry.property(pid).layout;
        let weights = if zero_weights {
            [0.0, 0.0]
        } else if name == "a" {
            [3.0, 1.0]
        } else {
            [1.0, 3.0]
        };
        for (node, flow, weight, rate) in [
            (&mut root, if seeded_rate { 0.0 } else { 1.0 }, 0.0, 0.0),
            (&mut p, 0.0, weights[0], if seeded_rate { 0.5 } else { 0.0 }),
            (&mut q, 0.0, weights[1], 0.0),
        ] {
            let mut value = PropertyValue::from_layout(layout);
            for (name, number) in [("flow", flow), ("weight", weight), ("balance_rate", rate)] {
                value.set_role(&role(name), layout, number);
            }
            node.add_property(pid, value);
        }
    }
    let projects = [p.id, q.id];
    root.add_child(p);
    root.add_child(q);
    Fixture {
        scenario: Scenario {
            name: "construction-leaf-a-wip".into(),
            registry,
            root,
            n_slots: 16,
            ticks_per_day: 1,
            max_days: 1,
            dt: 1.0,
            shadow_seeds: vec![],
            tick_patches: vec![],
            install_targets: Default::default(),
        },
        resources,
        projects,
    }
}

fn admitted(f: &Fixture, allocator: &SlotAllocator, reverse: bool) -> ResourceFlowSpec {
    let mut rf =
        derive_resource_flow_admission(None, &f.scenario.registry, &f.scenario.root, allocator)
            .expect("ordinary derived RF participation")
            .spec
            .unwrap();
    for arena in &mut rf.arenas {
        arena.max_orderband_depth = 16;
    }
    if reverse {
        rf.arenas.reverse();
    }
    rf
}

// Per resource: root/P/Q, each [allocated flow, balance rate, owned balance].
type Snapshot = Vec<[[f32; 3]; 3]>;
fn observed(
    f: &Fixture,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    values: &[f32],
) -> Snapshot {
    f.resources
        .iter()
        .map(|&pid| {
            [f.scenario.root.id, f.projects[0], f.projects[1]].map(|id| {
                ["allocated", "balance_rate", "balance"].map(|name| {
                    let col = registry
                        .column_range(pid)
                        .col_for_role(&role(name), &registry.property(pid).layout)
                        .unwrap();
                    values[allocator.slot_of(id).unwrap().as_usize() * registry.total_columns
                        + col.raw()]
                })
            })
        })
        .collect()
}

fn component(f: &Fixture, reverse_arenas: bool) -> Snapshot {
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&f.scenario.root).unwrap();
    let (arenas, _) = compile_and_materialize_resource_flow(
        &admitted(f, &allocator, reverse_arenas),
        &f.scenario.registry,
    )
    .unwrap();
    let mut state = WorldGpuState::new(
        GpuContext::new_blocking().expect("GPU required; no skip"),
        &f.scenario.registry,
        allocator.capacity() as u32,
    );
    let mut values = vec![0.0; state.values_len()];
    simthing_gpu::project_tree_to_values(
        &f.scenario.root,
        &f.scenario.registry,
        &allocator,
        f.scenario.registry.total_columns,
        &mut values,
    );
    state.install_resolved_values_at_boundary(&values); // Initial fixture, never live correction.
    let plan = sync_resource_flow_accumulator(
        &mut state,
        &f.scenario.registry,
        &arenas,
        &[],
        &[],
        &Default::default(),
    )
    .unwrap();
    state.run_resource_flow_bands(plan.n_bands, 1.0);
    observed(f, &f.scenario.registry, &allocator, &state.read_values())
}

fn ordinary(mut f: Fixture, reverse_arenas: bool) -> Snapshot {
    simthing_driver::resident_clearing_runtime::install_default_resident_rf_property(
        &mut f.scenario.registry,
        &mut f.scenario.root,
    );
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&f.scenario.root).unwrap();
    let spec = GameModeSpec {
        id: "construction-leaf-a-wip".into(),
        resource_flow: Some(admitted(&f, &allocator, reverse_arenas)),
        ..Default::default()
    };
    let mut session = SimSession::open_from_spec(f.scenario.clone(), &spec)
        .expect("qualified ordinary resident ingress");
    session.step_once().expect("one ordinary generation");
    observed(
        &f,
        &session.proto.registry,
        &session.proto.allocator,
        &session.state.read_values(),
    )
}

#[test]
fn parent_surplus_integrates_through_existing_balance_door() {
    for ordinary_path in [false, true] {
        let f = fixture(&["a"], true, false);
        let actual = if ordinary_path {
            ordinary(f, false)
        } else {
            component(&f, false)
        };
        let expected = vec![[[0.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]];
        println!("parent surplus control ordinary={ordinary_path}: {actual:?}");
        assert_eq!(
            actual, expected,
            "existing residual closure and governed integration positive controls"
        );
    }
}

#[test]
fn seeded_leaf_rate_integrates_once_in_one_ordinary_generation() {
    // Diagnostic initial-rate control ONLY: never a delivered-material claim.
    // It also guards against disguising missing settlement by copying flow to rate.
    let f = fixture(&["a"], true, true);
    let expected = vec![[[0.0, 0.0, 0.0], [0.0, 0.5, 0.5], [0.0, 0.0, 0.0]]];
    let component_result = component(&f, false);
    println!("seeded rate component: {component_result:?}");
    assert_eq!(
        component_result, expected,
        "standalone governed integration control"
    );
    let actual = ordinary(f, false);
    println!("seeded rate ordinary: {actual:?}; expected={expected:?}");
    assert_eq!(
        actual, expected,
        "one dt=1 generation must integrate rate 0.5 exactly once"
    );
}

fn expected(names: &[&str]) -> Snapshot {
    names
        .iter()
        .map(|&name| {
            let [p, q] = if name == "a" {
                [0.75, 0.25]
            } else {
                [0.25, 0.75]
            };
            [[0.0, 0.0, 0.0], [p, p, p], [q, q, q]]
        })
        .collect()
}

#[test]
fn leaf_residual_must_settle_as_owned_balance_for_each_resource() {
    let mut failures = Vec::new();
    for names in [vec!["a"], vec!["b"], vec!["a", "b"]] {
        let original = fixture(&names, false, false);
        for reverse_children in [false, true] {
            let mut f = original.clone();
            if reverse_children {
                f.scenario.root.children.reverse();
            }
            for reverse_arenas in [false, true] {
                let actual = component(&f, reverse_arenas);
                println!("component {names:?} children_reversed={reverse_children} arenas_reversed={reverse_arenas}: {actual:?}; expected={:?}", expected(&names));
                if actual != expected(&names) {
                    failures.push(format!(
                        "component {names:?}/{reverse_children}/{reverse_arenas}: {actual:?}"
                    ));
                }
            }
        }
    }
    // Same required stock law through ordinary ingress, before the one final assertion.
    let original = fixture(&["a", "b"], false, false);
    for reverse_children in [false, true] {
        let mut f = original.clone();
        if reverse_children {
            f.scenario.root.children.reverse();
        }
        for reverse_arenas in [false, true] {
            let actual = ordinary(f.clone(), reverse_arenas);
            println!("ordinary children_reversed={reverse_children} arenas_reversed={reverse_arenas}: {actual:?}; expected={:?}", expected(&["a", "b"]));
            if actual != expected(&["a", "b"]) {
                failures.push(format!(
                    "ordinary {reverse_children}/{reverse_arenas}: {actual:?}"
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "ordinary delivered flow never settled into project-owned WIP: {failures:?}"
    );
}
