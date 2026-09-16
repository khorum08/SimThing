//! Construction Leaf A: ordinary owned stock -> recipe consumption.
//! Scalar recipe output is NOT a structural birth or full Model-1 pass.
use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, DimensionRegistry,
    GenerationStamp, LogTier, Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource,
    PropertyTransformDelta, PropertyValue, SimPropertyId, SimThing, SimThingId, SimThingKind,
    SubFieldRole, SubFieldSpec, TransformOp,
};
use simthing_driver::{derive_resource_flow_admission, Scenario, SimSession};
use simthing_feeder::BoundaryRequest;
use simthing_gpu::SlotAllocator;
use simthing_spec::{
    compile_property, GameModeSpec, PropertyKey, PropertySpec, RecipeInputSpec,
    ResourceEconomySpec, ResourceFlowSpec, ResourceRecipeSpec,
};

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

fn fixture(names: &[&str]) -> Fixture {
    let mut registry = DimensionRegistry::new();
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut p = SimThing::new(SimThingKind::Cohort, 0);
    let mut q = SimThing::new(SimThingKind::Cohort, 0);
    let mut resources = Vec::new();
    for &name in names {
        let pid = property(&mut registry, name);
        resources.push(pid);
        let layout = &registry.property(pid).layout;
        let weights = if name == "a" { [3.0, 1.0] } else { [1.0, 3.0] };
        for (node, flow, weight, rate) in [
            (&mut root, 1.0, 0.0, 0.0),
            (&mut p, 0.0, weights[0], 0.0),
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

fn recipe(host: &str, band: u32) -> ResourceRecipeSpec {
    ResourceRecipeSpec {
        id: format!("complete-{host}"),
        inputs: ["a", "b"]
            .map(|name| RecipeInputSpec {
                property: PropertyKey::new("leaf_a", name),
                role: role("balance"),
                unit_cost: 1.0,
                host_entity: Some(host.into()),
                host_span_token: None,
            })
            .to_vec(),
        target: PropertyKey::new("leaf_a", "product"),
        target_role: role("balance"),
        target_host_entity: Some(host.into()),
        target_host_span_token: None,
        output_coefficient: 1.0,
        order_band: band,
        throttle_hint_max_per_tick: 1,
    }
}

fn build(
    pulse: [f32; 2],
    reverse_children: bool,
    reverse_arenas: bool,
    hosts: &[&str],
) -> (Fixture, GameModeSpec, [SimPropertyId; 3]) {
    let mut f = fixture(&["a", "b"]);
    for (pid, flow) in f.resources.iter().zip(pulse) {
        f.scenario.root.properties.get_mut(pid).unwrap().set_role(
            &role("flow"),
            &f.scenario.registry.property(*pid).layout,
            flow,
        );
    }
    let mut balance = f
        .scenario
        .registry
        .property(f.resources[0])
        .layout
        .sub_fields[0]
        .clone();
    balance.role = role("balance");
    balance.accumulator_spec = None;
    let product = compile_property(
        &PropertySpec {
            admission_disposition: Default::default(),
            id: "product".into(),
            namespace: "leaf_a".into(),
            name: "product".into(),
            display_name: "recipe output".into(),
            description: String::new(),
            sub_fields: vec![balance],
        },
        &mut f.scenario.registry,
    )
    .unwrap()
    .0;
    for node in &mut f.scenario.root.children {
        node.add_property(
            product,
            PropertyValue::from_layout(&f.scenario.registry.property(product).layout),
        );
    }
    for (name, id) in ["p", "q"].into_iter().zip(f.projects) {
        f.scenario.install_targets.insert(name.into(), vec![id]);
    }
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
        id: "construction-leaf-a-recipe".into(),
        resource_flow: Some(admitted(&f, &allocator, reverse_arenas)),
        resource_economy: Some(ResourceEconomySpec {
            // Authored ordered phases; each recipe still reads its own named host.
            recipes: hosts
                .iter()
                .map(|&host| recipe(host, if host == "p" { 0 } else { 1 }))
                .collect(),
            ..Default::default()
        }),
        ..Default::default()
    };
    let properties = [f.resources[0], f.resources[1], product];
    (f, spec, properties)
}

fn open(
    pulse: [f32; 2],
    reverse_children: bool,
    reverse_arenas: bool,
    host: &str,
) -> (SimSession, [SimThingId; 2], [SimPropertyId; 3]) {
    let (f, spec, properties) = build(pulse, reverse_children, reverse_arenas, &[host]);
    let parent = f.scenario.root.id;
    let ids = f.projects;
    let session = SimSession::open_from_spec(f.scenario, &spec)
        .expect("ordinary single-project recipe admission");
    // Authored one-generation source pulse. Boundary requests stop production
    // after G1; no readback-derived patch, stock write, or fixture reinstall.
    for pid in &properties[..2] {
        session
            .tx
            .submit_boundary(BoundaryRequest::AttachOverlay {
                target: parent,
                source_generation: GenerationStamp::new(0),
                overlay: Overlay {
                    id: OverlayId::new(),
                    kind: OverlayKind::Policy,
                    source: OverlaySource::System,
                    origin: parent,
                    affects: vec![parent],
                    transform: PropertyTransformDelta {
                        property_id: *pid,
                        sub_field_deltas: vec![(role("flow"), TransformOp::multiply(0.0))],
                    },
                    lifecycle: OverlayLifecycle::UntilDissolved,
                },
            })
            .unwrap();
    }
    (session, ids, properties)
}

fn stock(
    session: &SimSession,
    ids: [SimThingId; 2],
    pids: [SimPropertyId; 3],
    values: &[f32],
) -> [[f32; 3]; 2] {
    ids.map(|id| {
        pids.map(|pid| {
            let col = session
                .proto
                .registry
                .column_range(pid)
                .col_for_role(
                    &role("balance"),
                    &session.proto.registry.property(pid).layout,
                )
                .unwrap();
            values[session.proto.allocator.slot_of(id).unwrap().as_usize()
                * session.proto.registry.total_columns
                + col.raw()]
        })
    })
}

#[test]
fn ordinary_recipe_preserves_incomplete_wip_and_consumes_funded_stock_once() {
    let mut failures = Vec::new();
    for pulse in [[1.0, 1.0], [1.0, 0.0], [4.0, 4.0]] {
        for host in ["p", "q"] {
            for reversed in [false, true] {
                let reverse_children = reversed;
                let reverse_arenas = reversed;
                let (mut session, ids, pids) = open(pulse, reverse_children, reverse_arenas, host);
                for generation in 1..=5 {
                    session.step_once().expect("ordinary recipe generation");
                    let values = session.state.read_values();
                    let actual = stock(&session, ids, pids, &values);
                    println!("recipe host={host} pulse={pulse:?} children_reversed={reverse_children} arenas_reversed={reverse_arenas} g={generation}: {actual:?}");
                    let made = actual[0][2] + actual[1][2];
                    let totals = [
                        actual[0][0] + actual[1][0] + made,
                        actual[0][1] + actual[1][1] + made,
                    ];
                    // Independent assertion oracle only; never fed back into the session.
                    let mut expected = [
                        [pulse[0] * 0.75, pulse[1] * 0.25, 0.0],
                        [pulse[0] * 0.25, pulse[1] * 0.75, 0.0],
                    ];
                    if pulse == [4.0, 4.0] && generation >= 2 {
                        let project = if host == "p" { 0 } else { 1 };
                        expected[project][0] -= 1.0;
                        expected[project][1] -= 1.0;
                        expected[project][2] = 1.0;
                    }
                    if totals != pulse || actual != expected {
                        failures.push(format!("pulse={pulse:?}/children={reverse_children}/arenas={reverse_arenas}/g={generation}: {actual:?}; material accounting={totals:?}"));
                    }
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "ordinary recipe/stock conservation failed: {failures:?}"
    );
}

#[test]
fn two_explicit_project_recipe_hosts_must_admit_in_one_ordinary_session() {
    let mut failures = Vec::new();
    for reversed_children in [false, true] {
        for reversed_arenas in [false, true] {
            for hosts in [["p", "q"], ["q", "p"]] {
                let (f, spec, _) = build([1.0, 1.0], reversed_children, reversed_arenas, &hosts);
                let mut allocator = SlotAllocator::new();
                allocator.install_initial_tree(&f.scenario.root).unwrap();
                let eml = simthing_core::EmlExpressionRegistry::new();
                let compiled = simthing_spec::compile_resource_economy(
                    spec.resource_economy.as_ref().unwrap(),
                    &f.scenario.registry,
                    &eml,
                )
                .unwrap();
                let materialized =
                    simthing_driver::materialize_resource_economy_registry_for_session(
                        &compiled,
                        &f.scenario.registry,
                        &eml,
                        &f.scenario.root,
                        &allocator,
                        &f.scenario,
                    )
                    .unwrap();
                let mut actual_slots: Vec<_> = materialized
                    .registrations
                    .recipes
                    .iter()
                    .map(|recipe| {
                        assert!(
                            recipe
                                .inputs
                                .iter()
                                .all(|input| input.slot == recipe.target_slot),
                            "each recipe consumes only its explicit host stock"
                        );
                        recipe.target_slot.raw()
                    })
                    .collect();
                actual_slots.sort();
                let mut expected_slots = f.projects.map(|id| allocator.slot_of(id).unwrap().raw());
                expected_slots.sort();
                assert_eq!(actual_slots, expected_slots);
                let result = SimSession::open_from_spec(f.scenario, &spec);
                match result {
                    Ok(mut session) => { session.step_once().expect("admitted two-project generation"); }
                    Err(error) => failures.push(format!("hosts={hosts:?}/children={reversed_children}/arenas={reversed_arenas}: {error:?}")),
                }
                println!("joint hosts={hosts:?} children={reversed_children} arenas={reversed_arenas}: materialized host slots={actual_slots:?}; ordinary={}", failures.last().cloned().unwrap_or_else(|| "admitted".into()));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "distinct explicit recipe hosts were conflated: {failures:?}"
    );
}

#[test]
fn unresolved_or_ambiguous_recipe_host_still_refuses_with_provenance() {
    for ambiguous in [false, true] {
        let (mut f, mut spec, _) = build([1.0, 1.0], false, false, &["p"]);
        if ambiguous {
            f.scenario
                .install_targets
                .insert("p".into(), f.projects.to_vec());
        } else {
            f.scenario.install_targets.remove("p");
        }
        spec.resource_economy.as_mut().unwrap().recipes[0].inputs[0].host_span_token = Some(37);
        let error = SimSession::open_from_spec(f.scenario, &spec)
            .err()
            .expect("invalid host must refuse");
        match error {
            simthing_driver::SessionError::Install(
                simthing_driver::InstallError::NeedBindingInvalid {
                    binding,
                    reason,
                    span_token,
                },
            ) => {
                assert_eq!(binding, "resource_economy");
                assert_eq!(span_token, Some(37));
                assert!(reason.contains(if ambiguous {
                    "ambiguous (2 hosts)"
                } else {
                    "not in install_targets"
                }));
                println!("negative ambiguous={ambiguous}: {reason}; span={span_token:?}");
            }
            other => panic!("wrong host refusal: {other:?}"),
        }
    }
}
