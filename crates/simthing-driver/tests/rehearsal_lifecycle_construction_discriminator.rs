//! Leaf A allocation prerequisite, Board 5687371290 / 5687372904.
//! This deliberately requires independent resource policies. A RED result is a
//! production binding blocker, not evidence for atomic Model-2 commitment.
use simthing_core::{
    AccumulatorRole, AccumulatorSpec, ClampBehavior, DimensionRegistry, LogTier, PropertyValue,
    SimPropertyId, SimThing, SimThingId, SimThingKind, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{
    compile_and_materialize_resource_flow, derive_resource_flow_admission,
    sync_resource_flow_accumulator, Scenario, SimSession,
};
use simthing_gpu::{GpuContext, SlotAllocator, WorldGpuState};
use simthing_spec::{compile_property, GameModeSpec, PropertySpec};

fn role(name: &str) -> SubFieldRole {
    SubFieldRole::Named(name.into())
}

fn flow_property(registry: &mut DimensionRegistry, name: &str) -> SimPropertyId {
    let field = |name: &str, accumulator| SubFieldSpec {
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
        accumulator_spec: Some(AccumulatorSpec {
            role: accumulator,
            log_tier: LogTier::Summary,
        }),
    };
    compile_property(
        &PropertySpec {
            admission_disposition: Default::default(),
            id: name.into(),
            namespace: "leaf_a".into(),
            name: name.into(),
            display_name: name.into(),
            description: String::new(),
            sub_fields: vec![
                field("flow", AccumulatorRole::IntrinsicFlow),
                field(
                    "allocated",
                    AccumulatorRole::AllocatedFlow { arena: name.into() },
                ),
                field(
                    "weight",
                    AccumulatorRole::AllocatorWeight { arena: name.into() },
                ),
            ],
        },
        registry,
    )
    .expect("admit ordinary RF property")
    .0
}

#[derive(Clone)]
struct Fixture {
    scenario: Scenario,
    resources: Vec<SimPropertyId>,
    projects: [SimThingId; 2],
}

fn fixture(names: &[&str], reverse_children: bool) -> Fixture {
    let mut registry = DimensionRegistry::new();
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut p = SimThing::new(SimThingKind::Cohort, 0);
    let mut q = SimThing::new(SimThingKind::Cohort, 0);
    let mut resources = Vec::new();
    for &name in names {
        let pid = flow_property(&mut registry, name);
        resources.push(pid);
        let layout = &registry.property(pid).layout;
        // One unit per second of each distinct resource, exactly one dt=1 pulse.
        // Opposed policies: A P:Q=3:1, B P:Q=1:3. These are authored inputs,
        // never host-computed allocation results or a second inventory.
        let weights = if name == "a" { [3.0, 1.0] } else { [1.0, 3.0] };
        for (node, flow, weight) in [
            (&mut root, 1.0, 0.0),
            (&mut p, 0.0, weights[0]),
            (&mut q, 0.0, weights[1]),
        ] {
            let mut value = PropertyValue::from_layout(layout);
            value.set_role(&role("flow"), layout, flow);
            value.set_role(&role("weight"), layout, weight);
            node.add_property(pid, value);
        }
    }
    let projects = [p.id, q.id];
    root.add_child(p);
    root.add_child(q);
    if reverse_children {
        root.children.reverse();
    }
    Fixture {
        scenario: Scenario {
            name: "construction-leaf-a-allocation".into(),
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

fn admitted(f: &Fixture, allocator: &SlotAllocator) -> simthing_spec::ResourceFlowSpec {
    let mut rf =
        derive_resource_flow_admission(None, &f.scenario.registry, &f.scenario.root, allocator)
            .expect("derive ordinary resource-parent participation")
            .spec
            .unwrap();
    for arena in &mut rf.arenas {
        // Explicit authored plan budget; no gate or execution bypass.
        arena.max_orderband_depth = 16;
    }
    rf
}

fn observed(
    f: &Fixture,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    values: &[f32],
) -> Vec<[f32; 2]> {
    f.resources
        .iter()
        .map(|&pid| {
            let c = registry
                .column_range(pid)
                .col_for_role(&role("allocated"), &registry.property(pid).layout)
                .unwrap();
            f.projects.map(|id| {
                values[allocator.slot_of(id).unwrap().as_usize() * registry.total_columns + c.raw()]
            })
        })
        .collect()
}

fn component(f: &Fixture, reverse_arenas: bool) -> Vec<[f32; 2]> {
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&f.scenario.root).unwrap();
    let mut rf = admitted(f, &allocator);
    if reverse_arenas {
        rf.arenas.reverse();
    }
    let (arenas, _) = compile_and_materialize_resource_flow(&rf, &f.scenario.registry).unwrap();
    let mut state = WorldGpuState::new(
        GpuContext::new_blocking().expect("GPU required"),
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
    state.install_resolved_values_at_boundary(&values);
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

#[test]
fn each_resource_policy_works_alone_but_joint_plan_must_keep_both() {
    for (name, expected) in [("a", [0.75, 0.25]), ("b", [0.25, 0.75])] {
        let actual = component(&fixture(&[name], false), false);
        println!("single {name}: {actual:?}");
        assert_eq!(actual, vec![expected], "single-resource positive control");
    }
    let original = fixture(&["a", "b"], false);
    let mut reversed = original.clone();
    reversed.scenario.root.children.reverse(); // Same logical identities, different slots.
    let expected = vec![[0.75, 0.25], [0.25, 0.75]];
    let mut failures = Vec::new();
    for (children, f) in [("P,Q", &original), ("Q,P", &reversed)] {
        for reverse_arenas in [false, true] {
            let actual = component(f, reverse_arenas);
            println!("component children={children} reverse_arenas={reverse_arenas}: {actual:?}; expected={expected:?}");
            if actual != expected {
                failures.push((children, reverse_arenas, actual));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "independent-resource policy binding failed: {failures:?}"
    );
}

#[test]
fn ordinary_resident_session_must_preserve_opposite_resource_policies() {
    let mut failures = Vec::new();
    let original = fixture(&["a", "b"], false);
    for reverse_children in [false, true] {
        let mut f = original.clone();
        if reverse_children {
            f.scenario.root.children.reverse();
        }
        simthing_driver::resident_clearing_runtime::install_default_resident_rf_property(
            &mut f.scenario.registry,
            &mut f.scenario.root,
        );
        let mut allocator = SlotAllocator::new();
        allocator.install_initial_tree(&f.scenario.root).unwrap();
        let spec = GameModeSpec {
            id: "construction-leaf-a-allocation".into(),
            resource_flow: Some(admitted(&f, &allocator)),
            ..Default::default()
        };
        let mut session = SimSession::open_from_spec(f.scenario.clone(), &spec)
            .expect("qualified ordinary resident session");
        session
            .step_once()
            .expect("one ordinary generation executes");
        let actual = observed(
            &f,
            &session.proto.registry,
            &session.proto.allocator,
            &session.state.read_values(),
        );
        let expected = vec![[0.75, 0.25], [0.25, 0.75]];
        println!("ordinary reverse_children={reverse_children}: {actual:?}; expected={expected:?}");
        if actual != expected {
            failures.push((reverse_children, actual));
        }
    }
    assert!(
        failures.is_empty(),
        "ordinary two-input discriminator cannot lawfully allocate opposite policies: {failures:?}"
    );
}
