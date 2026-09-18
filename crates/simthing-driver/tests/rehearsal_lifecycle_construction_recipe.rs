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
        max_units_per_generation: None,
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

// Continuation after #2066. Existing prerequisite tests above remain unchanged.
fn stop_joint_sources(
    session: &SimSession,
    parent: SimThingId,
    pids: [SimPropertyId; 3],
) -> Vec<OverlayId> {
    pids[..2]
        .iter()
        .map(|pid| {
            let id = OverlayId::new();
            session
                .tx
                .submit_boundary(BoundaryRequest::AttachOverlay {
                    target: parent,
                    source_generation: GenerationStamp::new(0),
                    overlay: Overlay {
                        id,
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
            id
        })
        .collect()
}

fn joint_projects_preserve_partial_wip_and_consume_only_complete_owned_inputs() {
    for pulse in [[1.0, 1.0], [1.0, 0.0], [4.0, 4.0]] {
        for children in [false, true] {
            for arenas in [false, true] {
                for hosts in [["p", "q"], ["q", "p"]] {
                    let (f, spec, pids) = build(pulse, children, arenas, &hosts);
                    let ids = f.projects;
                    let parent = f.scenario.root.id;
                    let mut session = SimSession::open_from_spec(f.scenario, &spec).unwrap();
                    stop_joint_sources(&session, parent, pids);
                    for generation in 1..=5 {
                        session.step_once().expect("joint ordinary generation");
                        let actual = stock(&session, ids, pids, &session.state.read_values());
                        let mut expected = [
                            [pulse[0] * 0.75, pulse[1] * 0.25, 0.0],
                            [pulse[0] * 0.25, pulse[1] * 0.75, 0.0],
                        ];
                        if pulse == [4.0, 4.0] && generation >= 2 {
                            for row in &mut expected {
                                row[0] -= 1.0;
                                row[1] -= 1.0;
                                row[2] = 1.0;
                            }
                        }
                        println!("joint pulse={pulse:?} children={children} arenas={arenas} hosts={hosts:?} g={generation}: {actual:?}");
                        assert_eq!(actual, expected);
                        let made = actual[0][2] + actual[1][2];
                        assert_eq!(
                            [
                                actual[0][0] + actual[1][0] + made,
                                actual[0][1] + actual[1][1] + made
                            ],
                            pulse
                        );
                    }
                }
            }
        }
    }
}

fn cancelled_partial_wip_survives_recovery_reparent_and_restart() {
    for cancelled in [0, 1] {
        for reversed in [false, true] {
            let hosts = if reversed { ["q", "p"] } else { ["p", "q"] };
            let (mut f, mut spec, pids) = build([1.0, 1.0], reversed, reversed, &hosts);
            // Authored cancellation disposition: keep the whole owned WIP host
            // alive in recovery storage. Its identity/stock/residency are retained;
            // no product-placement commitment has been acquired at this stage.
            let depot = SimThing::new(SimThingKind::Location, 0);
            let depot_id = depot.id;
            let parent = f.scenario.root.id;
            f.scenario.root.add_child(depot);
            simthing_driver::resident_clearing_runtime::install_default_resident_rf_property(
                &mut f.scenario.registry,
                &mut f.scenario.root,
            );
            let mut allocator = SlotAllocator::new();
            allocator.install_initial_tree(&f.scenario.root).unwrap();
            spec.resource_flow = Some(admitted(&f, &allocator, reversed));
            let ids = f.projects;
            let mut session = SimSession::open_from_spec(f.scenario, &spec).unwrap();
            let stops = stop_joint_sources(&session, parent, pids);
            session.step_once().unwrap();
            let expected = [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]];
            assert_eq!(
                stock(&session, ids, pids, &session.state.read_values()),
                expected
            );
            let slots = ids.map(|id| session.proto.allocator.slot_of(id).unwrap());
            session
                .tx
                .submit_boundary(BoundaryRequest::Reparent {
                    child: ids[cancelled],
                    new_parent: depot_id,
                })
                .unwrap();
            for generation in 2..=3 {
                let result = session.step_once();
                println!("cancel project={cancelled} reversed={reversed} g={generation} result={result:?}");
                result.expect("ordinary cancellation/recovery boundary");
                assert_eq!(
                    stock(&session, ids, pids, &session.state.read_values()),
                    expected
                );
                assert_eq!(
                    ids.map(|id| session.proto.allocator.slot_of(id).unwrap()),
                    slots
                );
                assert_eq!(
                    session.proto.root.child_id(depot_id, 0),
                    Some(ids[cancelled])
                );
            }
            session
                .tx
                .submit_boundary(BoundaryRequest::Reparent {
                    child: ids[cancelled],
                    new_parent: parent,
                })
                .unwrap();
            for id in stops {
                session
                    .tx
                    .submit_boundary(BoundaryRequest::SuspendOverlay {
                        target: parent,
                        overlay_id: id,
                    })
                    .unwrap();
            }
            // Suspending a transform does not invert its past value write.
            // Restart explicitly re-authors source FLOW (never WIP stock).
            for pid in &pids[..2] {
                session
                    .tx
                    .submit_boundary(BoundaryRequest::AttachOverlay {
                        target: parent,
                        source_generation: GenerationStamp::new(3),
                        overlay: Overlay {
                            id: OverlayId::new(),
                            kind: OverlayKind::Policy,
                            source: OverlaySource::System,
                            origin: parent,
                            affects: vec![parent],
                            transform: PropertyTransformDelta {
                                property_id: *pid,
                                sub_field_deltas: vec![(role("flow"), TransformOp::set(1.0))],
                            },
                            lifecycle: OverlayLifecycle::UntilDissolved,
                        },
                    })
                    .unwrap();
            }
            session.step_once().expect("restart boundary");
            assert_eq!(
                stock(&session, ids, pids, &session.state.read_values()),
                expected
            );
            // The root policy propagates: restart authors flow=1 on root AND
            // both project producers. Each generation supplies 3A+3B; RF adds
            // (1.75,1.25) to P and (1.25,1.75) to Q after recipe consumption.
            // This is explicit new supply, never a refund or stock patch.
            for generation in 5..=8 {
                session.step_once().expect("ordinary restarted generation");
                let actual = stock(&session, ids, pids, &session.state.read_values());
                let expected = match generation {
                    5 => [[2.5, 1.5, 0.0], [1.5, 2.5, 0.0]],
                    6 => [[3.25, 1.75, 1.0], [1.75, 3.25, 1.0]],
                    7 => [[4.0, 2.0, 2.0], [2.0, 4.0, 2.0]],
                    8 => [[3.75, 1.25, 4.0], [1.25, 3.75, 4.0]],
                    _ => unreachable!(),
                };
                let delivered = 1.0 + 3.0 * (generation - 4) as f32;
                let made = actual[0][2] + actual[1][2];
                assert_eq!(
                    [
                        actual[0][0] + actual[1][0] + made,
                        actual[0][1] + actual[1][1] + made
                    ],
                    [delivered; 2]
                );
                println!(
                    "restart project={cancelled} reversed={reversed} g={generation}: {actual:?}"
                );
                assert_eq!(actual, expected);
            }
        }
    }
}

fn install_birth_on_funded_output(
    session: &mut SimSession,
    project: SimThingId,
    product: SimPropertyId,
    child: SimThing,
) {
    use simthing_core::{
        Direction, EmitOnThresholdBuffer, EmitOnThresholdRegistration, EmlExpressionRegistry,
        ThresholdDirection,
    };
    use simthing_driver::{
        compile_crossing_consequence_session, ActionBandActiveInstance,
        ActionBandNativeLaneAdmission, StructuralAuthorization,
    };
    use simthing_sim::{CostBandSemantic, ThresholdRegistry, VelocityAlertRegistration};
    use simthing_spec::{
        ActionBandAdmissionBudgetSpec, ActionBandBandSpec, ActionBandChannelBindingSpec,
        ActionBandChannelKind, ActionBandSessionBuildDoor, ActionBandSessionSpec,
        ActionBandTargetSpec, ActionBandTemplateSpec, ScalarBoundDirection,
    };
    let col = session
        .proto
        .registry
        .column_range(product)
        .col_for_role(
            &role("balance"),
            &session.proto.registry.property(product).layout,
        )
        .unwrap();
    let slot = session.proto.allocator.slot_of(project).unwrap();
    session
        .proto
        .register_velocity_alert(VelocityAlertRegistration {
            sim_thing_id: project,
            property_id: product,
            sub_field: role("balance"),
            threshold: 0.5,
            direction: Direction::Rising,
            cost_band: CostBandSemantic::observation(),
        });
    session
        .proto
        .initial_gpu_sync(&session.coord, &mut session.state)
        .unwrap();
    // Phase-5 crossing is strict >; integral recipe output crosses 0.5 on its first complete unit.
    let thresholds = vec![EmitOnThresholdRegistration {
        slot,
        col,
        threshold: 0.5,
        direction: ThresholdDirection::Upward,
        event_kind: 0,
        buffer: EmitOnThresholdBuffer::Values,
    }];
    let eml = EmlExpressionRegistry::new();
    let spec = ActionBandSessionSpec {
        budget: ActionBandAdmissionBudgetSpec {
            axis_channel_count: 1,
            dependency_binding_count: 0,
            storage_rows: 1,
            eml_program_count: 0,
            emission_binding_count: 1,
        },
        templates: vec![ActionBandTemplateSpec {
            id: "funded-first-birth".into(),
            label: None,
            axis_channels: vec![ActionBandChannelBindingSpec {
                column: col.raw_u32(),
                kind: ActionBandChannelKind::Primitive,
            }],
            target: ActionBandTargetSpec::ScalarBound {
                channel: col.raw_u32(),
                bound: 0.5,
                direction: ScalarBoundDirection::AtLeast,
            },
            velocity: None,
            bands: vec![ActionBandBandSpec {
                threshold_registration_index: 0,
                eml_program: None,
                emission_binding_indices: vec![0],
            }],
            subordinate_template_ids: vec![],
            max_active_subordinates: 0,
            reserved_instance_rows: 1,
            requirement_semantics: Default::default(),
        }],
    };
    let frozen = ActionBandSessionBuildDoor::new()
        .admit_once_at_session_build(&spec, &session.proto.registry, &eml, &thresholds)
        .unwrap()
        .clone();
    let lanes = ActionBandNativeLaneAdmission::from_existing_surfaces(
        &session.proto.registry,
        &[],
        &[],
        &thresholds,
        &ThresholdRegistry::new(),
    );
    let consequence = StructuralAuthorization::admit(BoundaryRequest::AddChild {
        parent: project,
        child,
    })
    .unwrap();
    let commitments = compile_crossing_consequence_session(
        &frozen,
        &eml,
        &[consequence],
        &[ActionBandActiveInstance::new(
            frozen.templates()[0].index(),
            slot,
            [0.0; 4],
        )],
        &lanes,
    )
    .unwrap();
    session
        .install_action_band_commitments(commitments)
        .unwrap();
}

fn first_funded_structural_birth_must_complete_and_leave_ordinary_session_usable() {
    let mut failures = Vec::new();
    for pulse in [[1.0, 1.0], [4.0, 4.0]] {
        for reversed in [false, true] {
            let hosts = if reversed { ["q", "p"] } else { ["p", "q"] };
            let (f, spec, pids) = build(pulse, reversed, reversed, &hosts);
            let parent = f.scenario.root.id;
            let ids = f.projects;
            let mut child = SimThing::new(SimThingKind::Cohort, 0);
            let child_id = child.id;
            let component = SimThing::new(SimThingKind::Cohort, 0);
            let component_id = component.id;
            child.add_child(component);
            child.add_property(
                pids[2],
                PropertyValue::from_layout(&f.scenario.registry.property(pids[2]).layout),
            );
            let mut session = SimSession::open_from_spec(f.scenario, &spec).unwrap();
            install_birth_on_funded_output(&mut session, ids[0], pids[2], child);
            stop_joint_sources(&session, parent, pids);
            let columns = session.proto.registry.total_columns;
            let n_slots = session.state.n_slots;
            for generation in 1..=5 {
                let result = session.step_once();
                let values = session.state.read_values();
                let actual = stock(&session, ids, pids, &values);
                let born = session.proto.root.contains_id(child_id);
                let placed = session
                    .proto
                    .allocator
                    .committed_residency_placement(parent, child_id)
                    .is_some();
                println!("birth pulse={pulse:?} reversed={reversed} g={generation} result={result:?} stock={actual:?} born={born} component={} placed={placed} action_generation={:?}", session.proto.root.contains_id(component_id), session.action_band_execution_generation());
                assert_eq!(session.proto.registry.total_columns, columns);
                assert_eq!(
                    session.state.n_slots, n_slots,
                    "preallocated shape never resizes"
                );
                if let Err(error) = result {
                    // Diagnose the touched-generation failure without repairing or
                    // rolling back any state. Required success assertion remains RED.
                    let before = (
                        session.coord.tick_index(),
                        session.coord.day_index(),
                        session.integration_schedule().entries().len(),
                        session.proto.allocator.binding_table_snapshot(),
                    );
                    for attempt in 0..3 {
                        let retry = session
                            .step_once()
                            .expect_err("faulted generation must not retry economics");
                        println!("birth retry {attempt}: {retry:?}");
                        assert!(
                            matches!(retry, simthing_driver::SessionError::ExecutionIdentity(ref message) if message.contains("fault"))
                        );
                        assert_eq!(session.state.read_values(), values);
                        assert_eq!(
                            (
                                session.coord.tick_index(),
                                session.coord.day_index(),
                                session.integration_schedule().entries().len(),
                                session.proto.allocator.binding_table_snapshot()
                            ),
                            before
                        );
                    }
                    failures.push(format!("pulse={pulse:?}/reversed={reversed}/g={generation}: {error:?}; born={born}; placed={placed}"));
                    break;
                }
                if pulse == [1.0, 1.0] {
                    assert!(
                        !born && !placed,
                        "incomplete inputs cannot mint a structural product"
                    );
                    assert_eq!(actual, [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]);
                } else if generation >= 3 {
                    assert!(born && placed);
                    assert!(session.proto.root.contains_id(component_id));
                    assert_eq!(actual, [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]);
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "funded structural boundary cannot finish: {failures:?}"
    );
}

fn ordinary_add_child_control_without_actionband_can_finish_and_continue() {
    let (f, spec, pids) = build([4.0, 4.0], false, false, &["p", "q"]);
    let parent = f.scenario.root.id;
    let ids = f.projects;
    let mut child = SimThing::new(SimThingKind::Cohort, 0);
    let child_id = child.id;
    let component = SimThing::new(SimThingKind::Cohort, 0);
    let component_id = component.id;
    child.add_child(component);
    child.add_property(
        pids[2],
        PropertyValue::from_layout(&f.scenario.registry.property(pids[2]).layout),
    );
    let mut session = SimSession::open_from_spec(f.scenario, &spec).unwrap();
    stop_joint_sources(&session, parent, pids);
    session.step_once().unwrap();
    session.step_once().unwrap();
    // Diagnostic only: fixed external boundary request, no ActionBand installed.
    // This control is NOT a funded construction path or a substitute solution.
    session
        .tx
        .submit_boundary(BoundaryRequest::AddChild {
            parent: ids[0],
            child,
        })
        .unwrap();
    for generation in 3..=4 {
        let result = session.step_once();
        println!("external AddChild control g={generation}: {result:?}");
        result.expect("ordinary AddChild without the frozen ActionBand should remain usable");
        assert!(session.proto.root.contains_id(child_id));
        assert!(session.proto.root.contains_id(component_id));
        assert!(session
            .proto
            .allocator
            .committed_residency_placement(parent, child_id)
            .is_some());
        assert_eq!(
            stock(&session, ids, pids, &session.state.read_values()),
            [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
        );
    }
}

#[test]
fn ordinary_joint_lifecycle_matrix() {
    let cases: &[(&str, fn())] = &[
        (
            "joint material accounting",
            joint_projects_preserve_partial_wip_and_consume_only_complete_owned_inputs,
        ),
        (
            "partial cancellation and restart",
            cancelled_partial_wip_survives_recovery_reparent_and_restart,
        ),
        (
            "external AddChild isolation control",
            ordinary_add_child_control_without_actionband_can_finish_and_continue,
        ),
        (
            "funded structural boundary required success",
            first_funded_structural_birth_must_complete_and_leave_ordinary_session_usable,
        ),
    ];
    for (name, run) in cases {
        println!("LIFECYCLE CASE START: {name}");
        run();
        println!("LIFECYCLE CASE PASS: {name}");
    }
}

#[test]
fn funded_product_cancellation_must_release_placement_and_continue() {
    let mut failures = Vec::new();
    // The external control isolates canonical Remove; it is not a funded birth.
    for funded in [false, true] {
        for project in 0..2 {
            for reversed in [false, true] {
                let hosts = if reversed { ["q", "p"] } else { ["p", "q"] };
                let (f, spec, pids) = build([4.0, 4.0], reversed, reversed, &hosts);
                let root = f.scenario.root.id;
                let ids = f.projects;
                let mut child = SimThing::new(SimThingKind::Cohort, 0);
                let child_id = child.id;
                let component = SimThing::new(SimThingKind::Cohort, 0);
                let component_id = component.id;
                child.add_child(component);
                child.add_property(
                    pids[2],
                    PropertyValue::from_layout(&f.scenario.registry.property(pids[2]).layout),
                );
                let mut session = SimSession::open_from_spec(f.scenario, &spec).unwrap();
                let initial_bindings = session.proto.allocator.binding_table_snapshot();
                let initial_live = session.proto.allocator.live_count();
                let shape = (session.state.n_slots, session.proto.registry.total_columns);
                let external_child = if funded {
                    install_birth_on_funded_output(&mut session, ids[project], pids[2], child);
                    None
                } else {
                    Some(child)
                };
                stop_joint_sources(&session, root, pids);
                for generation in 1..=2 {
                    session.step_once().unwrap();
                    assert!(!session.proto.root.contains_id(child_id));
                    assert!(session
                        .proto
                        .allocator
                        .committed_residency_placement(root, child_id)
                        .is_none());
                    let expected = if generation == 1 {
                        [[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]]
                    } else {
                        [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
                    };
                    assert_eq!(
                        stock(&session, ids, pids, &session.state.read_values()),
                        expected
                    );
                }
                if let Some(child) = external_child {
                    session
                        .tx
                        .submit_boundary(BoundaryRequest::AddChild {
                            parent: ids[project],
                            child,
                        })
                        .unwrap();
                }
                session
                    .step_once()
                    .expect("the repaired funded birth must finish G3");
                assert!(session.proto.root.contains_id(child_id));
                assert!(session.proto.root.contains_id(component_id));
                let placement = session
                    .proto
                    .allocator
                    .committed_residency_placement(root, child_id)
                    .unwrap();
                assert_eq!(placement.quantity(), 2);
                assert_eq!(session.proto.allocator.live_count(), initial_live + 2);
                let child_slot = session.proto.allocator.slot_of(child_id).unwrap();
                let component_slot = session.proto.allocator.slot_of(component_id).unwrap();
                println!("cancel setup funded={funded} project={project} reversed={reversed} g=3 placement={placement:?} slots={child_slot:?}/{component_slot:?}");

                // Fixed authored scrap policy after acquisition: consumed A/B stay
                // consumed, residual owned WIP stays owned, and there is no refund.
                // Scalar product output is an accounting receipt, not a reservation.
                // Readback above is assertion-only, never an economic decision.
                session
                    .tx
                    .submit_boundary(BoundaryRequest::Remove { target: child_id })
                    .unwrap();
                let result = session.step_once();
                let values = session.state.read_values();
                let remaining = session
                    .proto
                    .allocator
                    .committed_residency_placement(root, child_id);
                println!("cancel funded={funded} project={project} reversed={reversed} g=4 result={result:?} placement={remaining:?} stock={:?} live={}", stock(&session, ids, pids, &values), session.proto.allocator.live_count());
                assert!(!session.proto.root.contains_id(child_id));
                assert!(!session.proto.root.contains_id(component_id));
                assert_eq!(
                    session.proto.allocator.binding_table_snapshot(),
                    initial_bindings
                );
                assert_eq!(session.proto.allocator.live_count(), initial_live);
                assert!(session.proto.allocator.relation_of(child_id).is_none());
                assert!(session.proto.allocator.relation_of(component_id).is_none());
                assert_eq!(
                    (session.state.n_slots, session.proto.registry.total_columns),
                    shape
                );
                assert_eq!(
                    stock(&session, ids, pids, &values),
                    [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
                );
                if remaining.is_some() {
                    failures.push(format!("funded={funded}/project={project}/reversed={reversed}: removed subtree retains committed placement {remaining:?}"));
                }
                if let Err(error) = result {
                    let before = (
                        session.coord.tick_index(),
                        session.coord.day_index(),
                        session.integration_schedule().entries().len(),
                        session.proto.allocator.binding_table_snapshot(),
                        session.action_band_execution_generation(),
                    );
                    for attempt in 0..3 {
                        let retry = session
                            .step_once()
                            .expect_err("touched cancellation must remain fail-stop");
                        println!("cancel retry funded={funded} project={project} reversed={reversed} attempt={attempt}: {retry:?}");
                        assert!(
                            matches!(retry, simthing_driver::SessionError::ExecutionIdentity(ref message) if message.contains("fault"))
                        );
                        assert_eq!(session.state.read_values(), values);
                        assert_eq!(
                            (
                                session.coord.tick_index(),
                                session.coord.day_index(),
                                session.integration_schedule().entries().len(),
                                session.proto.allocator.binding_table_snapshot(),
                                session.action_band_execution_generation(),
                            ),
                            before
                        );
                        assert_eq!(
                            session
                                .proto
                                .allocator
                                .committed_residency_placement(root, child_id),
                            remaining
                        );
                    }
                    failures.push(format!("funded={funded}/project={project}/reversed={reversed}: canonical cancellation cannot finish: {error:?}"));
                } else {
                    for generation in 5..=6 {
                        session
                            .step_once()
                            .expect("successful cancellation must leave a usable session");
                        assert_eq!(
                            stock(&session, ids, pids, &session.state.read_values()),
                            [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
                        );
                        assert_eq!(
                            session.proto.allocator.binding_table_snapshot(),
                            initial_bindings
                        );
                        println!("cancel continuation funded={funded} project={project} reversed={reversed} g={generation}: healthy");
                    }
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "funded cancellation/release required success: {failures:?}"
    );
}

#[test]
fn canonical_multi_remove_must_preserve_requested_identities() {
    let mut failures = Vec::new();
    // These requests commute by identity. Reversed request orders are controls,
    // not a repair or an instruction to sort around a broken mutation boundary.
    for indices in [[0, 2], [2, 0], [3, 4], [4, 3]] {
        let (mut f, mut spec, pids) = build([4.0, 4.0], false, false, &["p", "q"]);
        let root = f.scenario.root.id;
        let projects = f.projects;
        let siblings = (0..5)
            .map(|_| SimThing::new(SimThingKind::Cohort, 0))
            .collect::<Vec<_>>();
        let sibling_ids = siblings.iter().map(|node| node.id).collect::<Vec<_>>();
        for sibling in siblings {
            f.scenario.root.add_child(sibling);
        }
        simthing_driver::resident_clearing_runtime::install_default_resident_rf_property(
            &mut f.scenario.registry,
            &mut f.scenario.root,
        );
        let mut allocator = SlotAllocator::new();
        allocator.install_initial_tree(&f.scenario.root).unwrap();
        spec.resource_flow = Some(admitted(&f, &allocator, false));
        let mut session = SimSession::open_from_spec(f.scenario, &spec).unwrap();
        let initial = session.proto.allocator.binding_table_snapshot();
        let capacity = session.proto.allocator.growth_capacity_available(root);
        let requested = indices.map(|index| sibling_ids[index]);
        stop_joint_sources(&session, root, pids);
        for target in requested {
            session
                .tx
                .submit_boundary(BoundaryRequest::Remove { target })
                .unwrap();
        }
        // Catch only to collect every diagnostic/control and inspect fail-stop.
        // A panic is always retained as a failed required-success obligation.
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| session.step_once()));
        let actual_removed = sibling_ids
            .iter()
            .copied()
            .filter(|id| !session.proto.root.contains_id(*id))
            .collect::<Vec<_>>();
        println!("multi-remove indices={indices:?} requested={requested:?} actual_removed={actual_removed:?}");
        for (id, slot) in &initial {
            let should_survive = !requested.contains(id);
            if session.proto.root.contains_id(*id) != should_survive
                || session.proto.allocator.slot_of(*id) != should_survive.then_some(*slot)
            {
                failures.push(format!("indices={indices:?}: wrong identity disposition id={id:?}, requested={requested:?}, actual_removed={actual_removed:?}"));
            }
        }
        match result {
            Ok(Ok(step)) => {
                println!("multi-remove indices={indices:?}: completed {step:?}");
                assert_eq!(
                    stock(&session, projects, pids, &session.state.read_values()),
                    [[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]]
                );
                assert_eq!(
                    session.proto.allocator.growth_capacity_available(root),
                    capacity + 2
                );
            }
            error => {
                let detail = match error {
                    Ok(Err(error)) => format!("returned {error:?}"),
                    Err(payload) => format!(
                        "panicked: {}",
                        payload
                            .downcast_ref::<String>()
                            .map(String::as_str)
                            .or_else(|| payload.downcast_ref::<&str>().copied())
                            .unwrap_or("non-string panic")
                    ),
                    Ok(Ok(_)) => unreachable!(),
                };
                println!("multi-remove indices={indices:?}: {detail}");
                failures.push(format!("indices={indices:?}: {detail}"));
                let values = session.state.read_values();
                let before = (
                    session.coord.tick_index(),
                    session.coord.day_index(),
                    session.integration_schedule().entries().len(),
                    session.proto.allocator.binding_table_snapshot(),
                    session.proto.root.direct_child_ids(),
                );
                for attempt in 0..3 {
                    let retry = session
                        .step_once()
                        .expect_err("unfinished economic boundary remains fail-stop");
                    println!("multi-remove retry indices={indices:?} attempt={attempt}: {retry:?}");
                    assert!(
                        matches!(retry, simthing_driver::SessionError::ExecutionIdentity(ref message) if message.contains("fault"))
                    );
                    assert_eq!(session.state.read_values(), values);
                    assert_eq!(
                        (
                            session.coord.tick_index(),
                            session.coord.day_index(),
                            session.integration_schedule().entries().len(),
                            session.proto.allocator.binding_table_snapshot(),
                            session.proto.root.direct_child_ids(),
                        ),
                        before
                    );
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "canonical identity-based cancellation required success: {failures:?}"
    );
}

fn install_funded_birth_sequence(
    session: &mut SimSession,
    project: SimThingId,
    product: SimPropertyId,
    children: Vec<SimThing>,
) {
    use simthing_core::{
        Direction, EmitOnThresholdBuffer, EmitOnThresholdRegistration, EmlExpressionRegistry,
        ThresholdDirection,
    };
    use simthing_driver::{
        compile_crossing_consequence_session, ActionBandActiveInstance,
        ActionBandNativeLaneAdmission, StructuralAuthorization,
    };
    use simthing_sim::{CostBandSemantic, ThresholdRegistry, VelocityAlertRegistration};
    use simthing_spec::{
        ActionBandAdmissionBudgetSpec, ActionBandBandSpec, ActionBandChannelBindingSpec,
        ActionBandChannelKind, ActionBandSessionBuildDoor, ActionBandSessionSpec,
        ActionBandTargetSpec, ActionBandTemplateSpec, ScalarBoundDirection,
    };
    let col = session
        .proto
        .registry
        .column_range(product)
        .col_for_role(
            &role("balance"),
            &session.proto.registry.property(product).layout,
        )
        .unwrap();
    let slot = session.proto.allocator.slot_of(project).unwrap();
    let mut thresholds = Vec::new();
    for index in 0..children.len() {
        let threshold = index as f32 + 0.5;
        session
            .proto
            .register_velocity_alert(VelocityAlertRegistration {
                sim_thing_id: project,
                property_id: product,
                sub_field: role("balance"),
                threshold,
                direction: Direction::Rising,
                cost_band: CostBandSemantic::observation(),
            });
        thresholds.push(EmitOnThresholdRegistration {
            slot,
            col,
            threshold,
            direction: ThresholdDirection::Upward,
            event_kind: index as u32,
            buffer: EmitOnThresholdBuffer::Values,
        });
    }
    session
        .proto
        .initial_gpu_sync(&session.coord, &mut session.state)
        .unwrap();
    let eml = EmlExpressionRegistry::new();
    let spec = ActionBandSessionSpec {
        budget: ActionBandAdmissionBudgetSpec {
            axis_channel_count: 1,
            dependency_binding_count: 0,
            storage_rows: children.len() as u32,
            eml_program_count: 0,
            emission_binding_count: children.len() as u32,
        },
        templates: (0..children.len())
            .map(|index| ActionBandTemplateSpec {
                id: format!("funded-distinct-birth-{index}"),
                label: None,
                axis_channels: vec![ActionBandChannelBindingSpec {
                    column: col.raw_u32(),
                    kind: ActionBandChannelKind::Primitive,
                }],
                target: ActionBandTargetSpec::ScalarBound {
                    channel: col.raw_u32(),
                    bound: index as f32 + 0.5,
                    direction: ScalarBoundDirection::AtLeast,
                },
                velocity: None,
                bands: vec![ActionBandBandSpec {
                    threshold_registration_index: index as u32,
                    eml_program: None,
                    emission_binding_indices: vec![index as u32],
                }],
                subordinate_template_ids: vec![],
                max_active_subordinates: 0,
                reserved_instance_rows: 1,
                requirement_semantics: Default::default(),
            })
            .collect(),
    };
    let frozen = ActionBandSessionBuildDoor::new()
        .admit_once_at_session_build(&spec, &session.proto.registry, &eml, &thresholds)
        .unwrap()
        .clone();
    let lanes = ActionBandNativeLaneAdmission::from_existing_surfaces(
        &session.proto.registry,
        &[],
        &[],
        &thresholds,
        &ThresholdRegistry::new(),
    );
    let consequences = children
        .into_iter()
        .map(|child| {
            StructuralAuthorization::admit(BoundaryRequest::AddChild {
                parent: project,
                child,
            })
            .unwrap()
        })
        .collect::<Vec<_>>();
    let active = frozen
        .templates()
        .iter()
        .map(|template| ActionBandActiveInstance::new(template.index(), slot, [0.0; 4]))
        .collect::<Vec<_>>();
    let commitments =
        compile_crossing_consequence_session(&frozen, &eml, &consequences, &active, &lanes)
            .unwrap();
    session
        .install_action_band_commitments(commitments)
        .unwrap();
}

fn replace_authored_source_flow(
    session: &SimSession,
    root: SimThingId,
    pids: [SimPropertyId; 3],
    previous: Vec<OverlayId>,
    generation: u32,
    flow: f32,
) -> Vec<OverlayId> {
    for overlay_id in previous {
        session
            .tx
            .submit_boundary(BoundaryRequest::SuspendOverlay {
                target: root,
                overlay_id,
            })
            .unwrap();
    }
    pids[..2]
        .iter()
        .map(|pid| {
            let id = OverlayId::new();
            session
                .tx
                .submit_boundary(BoundaryRequest::AttachOverlay {
                    target: root,
                    source_generation: GenerationStamp::new(generation - 1),
                    overlay: Overlay {
                        id,
                        kind: OverlayKind::Policy,
                        source: OverlaySource::System,
                        origin: root,
                        affects: vec![root],
                        transform: PropertyTransformDelta {
                            property_id: *pid,
                            sub_field_deltas: vec![(role("flow"), TransformOp::set(flow))],
                        },
                        lifecycle: OverlayLifecycle::UntilDissolved,
                    },
                })
                .unwrap();
            id
        })
        .collect()
}

#[test]
fn ordinary_funded_placement_and_restart_matrix() {
    let mut failures = Vec::new();
    use simthing_core::ObjectResidencyRelation;
    use simthing_sim::{
        BoundaryDeltaEntry, OrdinaryGrowthRefusalReason, RecordedGrowthResidencyFact,
    };
    for project in 0..2 {
        for reversed in [false, true] {
            let hosts = if reversed { ["q", "p"] } else { ["p", "q"] };
            let (mut f, mut spec, pids) = build([4.0, 4.0], reversed, reversed, &hosts);
            let root = f.scenario.root.id;
            let ids = f.projects;
            // Fully occupied eight-row world. Fixed authored removals at G1
            // leave two separated holes: enough total grant, no two-row extent.
            let blockers = (0..5)
                .map(|_| SimThing::new(SimThingKind::Cohort, 0))
                .collect::<Vec<_>>();
            let blocker_ids = blockers.iter().map(|node| node.id).collect::<Vec<_>>();
            for blocker in blockers {
                f.scenario.root.add_child(blocker);
            }
            f.scenario.n_slots = 8;
            simthing_driver::resident_clearing_runtime::install_default_resident_rf_property(
                &mut f.scenario.registry,
                &mut f.scenario.root,
            );
            let mut initial = SlotAllocator::new();
            initial.install_initial_tree(&f.scenario.root).unwrap();
            spec.resource_flow = Some(admitted(&f, &initial, reversed));
            let layout = f.scenario.registry.property(pids[2]).layout.clone();
            let mut products = Vec::new();
            let mut profiles = Vec::new();
            for ordinal in 0..3 {
                let mut product = SimThing::new(SimThingKind::Cohort, 0);
                let mut component = SimThing::new(SimThingKind::Cohort, 0);
                for (node, base) in [(&mut product, 7.0), (&mut component, 9.0)] {
                    let mut value = PropertyValue::from_layout(&layout);
                    value.set_role(&role("balance"), &layout, base + ordinal as f32);
                    node.add_property(pids[2], value);
                }
                let overlay_id = OverlayId::new();
                product.overlays.push(Overlay {
                    id: overlay_id,
                    kind: OverlayKind::Policy,
                    source: OverlaySource::System,
                    origin: product.id,
                    affects: vec![product.id],
                    transform: PropertyTransformDelta {
                        property_id: pids[2],
                        sub_field_deltas: vec![(
                            role("balance"),
                            TransformOp::set(17.0 + ordinal as f32),
                        )],
                    },
                    lifecycle: OverlayLifecycle::UntilDissolved,
                });
                profiles.push((product.id, component.id, overlay_id));
                product.add_child(component);
                products.push(product);
            }
            let mut session = SimSession::open_from_spec(f.scenario, &spec).unwrap();
            install_funded_birth_sequence(&mut session, ids[project], pids[2], products);
            let mut source_overlays = stop_joint_sources(&session, root, pids);
            for target in [blocker_ids[0], blocker_ids[2]] {
                session
                    .tx
                    .submit_boundary(BoundaryRequest::Remove { target })
                    .unwrap();
            }
            let shape = (session.state.n_slots, session.proto.registry.total_columns);
            let bound_slots = ids.map(|id| session.proto.allocator.slot_of(id).unwrap());
            let mut first_success_placement = None;
            for generation in 1..=14 {
                // A fixed authored script, independent of all readback. Failed
                // placement scraps its already-consumed recipe inputs (no refund).
                // Restart pays new A/B; the old refused candidate never retries.
                if generation == 4 {
                    for target in [blocker_ids[1], blocker_ids[3], blocker_ids[4]] {
                        session
                            .tx
                            .submit_boundary(BoundaryRequest::Remove { target })
                            .unwrap();
                    }
                }
                if generation == 9 {
                    session
                        .tx
                        .submit_boundary(BoundaryRequest::Remove {
                            target: profiles[1].0,
                        })
                        .unwrap();
                }
                if [4, 5, 9, 10].contains(&generation) {
                    let flow = if generation == 4 || generation == 9 {
                        1.0
                    } else {
                        0.0
                    };
                    source_overlays = replace_authored_source_flow(
                        &session,
                        root,
                        pids,
                        source_overlays,
                        generation,
                        flow,
                    );
                }
                let result = session.step_once();
                let values = session.state.read_values();
                let actual = stock(&session, ids, pids, &values);
                println!("placement/restart project={project} reversed={reversed} g={generation} result={result:?} stock={actual:?} capacity={} live={} present={:?} action_generation={:?}", session.proto.allocator.growth_capacity_available(root), session.proto.allocator.live_count(), profiles.iter().map(|(id,_,_)| session.proto.root.contains_id(*id)).collect::<Vec<_>>(), session.action_band_execution_generation());
                if let Err(ref error) = result {
                    let before = (
                        session.coord.tick_index(),
                        session.coord.day_index(),
                        session.integration_schedule().entries().len(),
                        session.proto.allocator.binding_table_snapshot(),
                        session.action_band_execution_generation(),
                    );
                    for attempt in 0..3 {
                        let retry = session
                            .step_once()
                            .expect_err("touched generation stays fail-stop");
                        println!("placement/restart fault={error:?} retry={attempt}: {retry:?}");
                        assert!(
                            matches!(retry, simthing_driver::SessionError::ExecutionIdentity(ref message) if message.contains("fault"))
                        );
                        assert_eq!(session.state.read_values(), values);
                        assert_eq!(
                            (
                                session.coord.tick_index(),
                                session.coord.day_index(),
                                session.integration_schedule().entries().len(),
                                session.proto.allocator.binding_table_snapshot(),
                                session.action_band_execution_generation()
                            ),
                            before
                        );
                    }
                }
                if let Err(error) = result {
                    failures.push(format!("placement/restart project={project} reversed={reversed} g={generation}: {error:?}"));
                    break;
                }
                for (index, id) in blocker_ids.iter().enumerate() {
                    let present = generation < 4 && index != 0 && index != 2;
                    assert_eq!(
                        session.proto.root.contains_id(*id),
                        present,
                        "exact blocker identity"
                    );
                    assert_eq!(session.proto.allocator.slot_of(*id).is_some(), present);
                }
                if generation == 4 || generation == 9 {
                    assert_eq!(session.proto.allocator.growth_capacity_available(root), 5);
                }
                assert_eq!(
                    (session.state.n_slots, session.proto.registry.total_columns),
                    shape
                );
                assert_eq!(
                    ids.map(|id| session.proto.allocator.slot_of(id).unwrap()),
                    bound_slots
                );
                let expected = match generation {
                    1 => [[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]],
                    2..=4 => [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]],
                    5 => [[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]],
                    6..=9 => [[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]],
                    10 => [[4.5, 1.5, 2.0], [1.5, 4.5, 2.0]],
                    _ => [[3.5, 0.5, 3.0], [0.5, 3.5, 3.0]],
                };
                assert_eq!(actual, expected);
                let supplied = 4.0
                    + if generation >= 5 { 3.0 } else { 0.0 }
                    + if generation >= 10 { 3.0 } else { 0.0 };
                for resource in 0..2 {
                    assert_eq!(
                        actual[0][resource] + actual[1][resource] + actual[0][2] + actual[1][2],
                        supplied
                    );
                }
                let entries = session.proto.take_delta_log();
                let refusals = entries
                    .iter()
                    .filter_map(|entry| match entry {
                        BoundaryDeltaEntry::GrowthResidencyRefused {
                            fact: RecordedGrowthResidencyFact::Refused(refusal),
                        } => Some(refusal),
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                if generation == 3 {
                    assert_eq!(refusals.len(), 1);
                    let refusal = refusals[0];
                    println!("typed placement refusal: {refusal:?}");
                    assert_eq!(refusal.candidate().grantee(), profiles[0].0);
                    assert_eq!(refusal.attempted_generation().get(), 3);
                    assert_eq!(refusal.revalue_generation().get(), 4);
                    let OrdinaryGrowthRefusalReason::Placement(placement_refusal) =
                        refusal.reason()
                    else {
                        panic!("fragmentation must be a typed placement refusal: {refusal:?}");
                    };
                    assert!(matches!(
                        placement_refusal.reason(),
                        simthing_gpu::ResidencyPlacementRefusalReason::NoContiguousExtent {
                            quantity: 2,
                            ..
                        }
                    ));
                    assert_eq!(placement_refusal.retained_unmet_quantity(), 2);
                    assert!(refusal.market_grant_key().is_some());
                    assert_eq!(session.proto.allocator.growth_capacity_available(root), 2);
                } else {
                    assert!(refusals.is_empty());
                }
                assert!(!session.proto.root.contains_id(profiles[0].0));
                assert!(!session.proto.root.contains_id(profiles[0].1));
                assert!(session.proto.allocator.slot_of(profiles[0].1).is_none());
                assert!(session.proto.allocator.relation_of(profiles[0].0).is_none());
                assert!(session.proto.allocator.relation_of(profiles[0].1).is_none());
                assert!(session
                    .proto
                    .allocator
                    .committed_residency_placement(root, profiles[0].0)
                    .is_none());
                assert!(session.proto.allocator.slot_of(profiles[0].0).is_none());
                for ordinal in 1..=2 {
                    let (id, component, overlay) = profiles[ordinal];
                    let born_now = generation == if ordinal == 1 { 7 } else { 12 };
                    let present = if ordinal == 1 {
                        (7..9).contains(&generation)
                    } else {
                        generation >= 12
                    };
                    assert_eq!(session.proto.root.contains_id(id), present);
                    assert_eq!(session.proto.root.contains_id(component), present);
                    if present {
                        let snapshot = session.proto.root.snapshot_node(id).unwrap();
                        assert_eq!(snapshot.children, vec![component]);
                        assert!(snapshot.property_ids.contains(&pids[2]));
                        assert!(snapshot.overlay_ids.contains(&overlay));
                        assert_eq!(
                            session.proto.allocator.relation_of(id),
                            Some(ObjectResidencyRelation::ChildOf(ids[project]))
                        );
                        assert_eq!(
                            session.proto.allocator.relation_of(component),
                            Some(ObjectResidencyRelation::ChildOf(id))
                        );
                        let placement = session
                            .proto
                            .allocator
                            .committed_residency_placement(root, id)
                            .unwrap();
                        assert_eq!(placement.quantity(), 2);
                        if born_now {
                            assert!(entries.iter().any(|entry| matches!(entry, BoundaryDeltaEntry::SimThingAdded { parent, node, residency } if *parent == ids[project] && node.id() == id && residency.placement() == placement)));
                            println!("fresh completion ordinal={ordinal} id={id:?} component={component:?} overlay={overlay:?} placement={placement:?}");
                            if ordinal == 1 {
                                first_success_placement = Some(placement);
                            } else {
                                let prior = first_success_placement.unwrap();
                                assert_eq!(
                                    placement.extent(),
                                    prior.extent(),
                                    "freed capacity reused"
                                );
                                assert_ne!(
                                    placement.identity(),
                                    prior.identity(),
                                    "fresh grant and grantee"
                                );
                            }
                        } else {
                            let col = session
                                .proto
                                .registry
                                .column_range(pids[2])
                                .col_for_role(&role("balance"), &layout)
                                .unwrap();
                            for observed in [id, component] {
                                let slot = session.proto.allocator.slot_of(observed).unwrap();
                                let value =
                                    values[slot.as_usize() * shape.1 + col.raw_u32() as usize];
                                println!(
                                    "fresh observation id={observed:?} slot={slot:?} value={value}"
                                );
                                assert_eq!(value, 17.0 + ordinal as f32);
                            }
                        }
                    } else {
                        assert!(session.proto.allocator.slot_of(id).is_none());
                        assert!(session.proto.allocator.slot_of(component).is_none());
                        assert!(session
                            .proto
                            .allocator
                            .committed_residency_placement(root, id)
                            .is_none());
                    }
                }
            }
            assert_ne!(profiles[1].0, profiles[2].0);
            assert_ne!(profiles[1].1, profiles[2].1);
            assert_ne!(profiles[1].2, profiles[2].2);
        }
    }
    separated_funded_crossings_without_placement_refusal(&mut failures);
    assert!(
        failures.is_empty(),
        "ordinary funded continuation cannot finish: {failures:?}"
    );
}

// Isolation control: the same separated recipe crossings with ample contiguous
// capacity and no removal, fragmentation, refusal, or cancellation. Both births
// must complete through the installed structural door; errors remain failures.
fn separated_funded_crossings_without_placement_refusal(failures: &mut Vec<String>) {
    for project in 0..2 {
        for reversed in [false, true] {
            let hosts = if reversed { ["q", "p"] } else { ["p", "q"] };
            let (f, spec, pids) = build([4.0, 4.0], reversed, reversed, &hosts);
            let root = f.scenario.root.id;
            let ids = f.projects;
            let mut children = Vec::new();
            let mut products = Vec::new();
            for _ in 0..2 {
                let mut child = SimThing::new(SimThingKind::Cohort, 0);
                child.add_property(
                    pids[2],
                    PropertyValue::from_layout(&f.scenario.registry.property(pids[2]).layout),
                );
                child.add_child(SimThing::new(SimThingKind::Cohort, 0));
                products.push(child.id);
                children.push(child);
            }
            let mut session = SimSession::open_from_spec(f.scenario, &spec).unwrap();
            install_funded_birth_sequence(&mut session, ids[project], pids[2], children);
            let mut source_overlays = stop_joint_sources(&session, root, pids);
            for generation in 1..=8 {
                if generation == 4 || generation == 5 {
                    source_overlays = replace_authored_source_flow(
                        &session,
                        root,
                        pids,
                        source_overlays,
                        generation,
                        if generation == 4 { 1.0 } else { 0.0 },
                    );
                }
                let result = session.step_once();
                let values = session.state.read_values();
                let actual = stock(&session, ids, pids, &values);
                println!("no-refusal control project={project} reversed={reversed} g={generation} result={result:?} stock={actual:?} action_generation={:?}", session.action_band_execution_generation());
                let expected = match generation {
                    1 => [[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]],
                    2..=4 => [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]],
                    5 => [[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]],
                    _ => [[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]],
                };
                assert_eq!(actual, expected);
                assert!(!session.proto.delta_log().iter().any(|entry| matches!(
                    entry,
                    simthing_sim::BoundaryDeltaEntry::GrowthResidencyRefused { .. }
                )));
                assert_eq!(session.proto.root.contains_id(products[0]), generation >= 3);
                assert_eq!(
                    session
                        .proto
                        .allocator
                        .committed_residency_placement(root, products[0])
                        .is_some(),
                    generation >= 3
                );
                assert_eq!(session.proto.root.contains_id(products[1]), generation >= 7);
                if let Err(error) = result {
                    let before = (
                        session.coord.tick_index(),
                        session.coord.day_index(),
                        session.integration_schedule().entries().len(),
                        session.proto.allocator.binding_table_snapshot(),
                        session.action_band_execution_generation(),
                        session
                            .proto
                            .allocator
                            .committed_residency_placement(root, products[0]),
                    );
                    for attempt in 0..3 {
                        let retry = session
                            .step_once()
                            .expect_err("touched control generation stays fail-stop");
                        println!("no-refusal control retry={attempt}: {retry:?}");
                        assert!(
                            matches!(retry, simthing_driver::SessionError::ExecutionIdentity(ref message) if message.contains("fault"))
                        );
                        assert_eq!(session.state.read_values(), values);
                        assert_eq!(
                            (
                                session.coord.tick_index(),
                                session.coord.day_index(),
                                session.integration_schedule().entries().len(),
                                session.proto.allocator.binding_table_snapshot(),
                                session.action_band_execution_generation(),
                                session
                                    .proto
                                    .allocator
                                    .committed_residency_placement(root, products[0])
                            ),
                            before
                        );
                    }
                    failures.push(format!("no-refusal control project={project} reversed={reversed} g={generation}: {error:?}"));
                    break;
                }
            }
        }
    }
}

// Terminal additions deliberately leave every historical witness above intact.
// Four independent switches (rather than one correlated reversal) distinguish
// authored order from the allocator's physical placement of otherwise inert rows.
fn terminal_order_case(project: usize, order: [bool; 4]) -> Vec<([[f32; 3]; 2], usize, u32)> {
    use simthing_core::ObjectResidencyRelation;
    use simthing_sim::BoundaryDeltaEntry;
    let [children_reversed, arenas_reversed, recipes_reversed, prefix_spacers] = order;
    let hosts = if recipes_reversed {
        ["q", "p"]
    } else {
        ["p", "q"]
    };
    let (mut f, mut spec, pids) = build([4.0, 4.0], children_reversed, arenas_reversed, &hosts);
    let root = f.scenario.root.id;
    let ids = f.projects;
    let spacers = (0..4)
        .map(|_| SimThing::new(SimThingKind::Cohort, 0))
        .collect::<Vec<_>>();
    let spacer_ids = spacers.iter().map(|node| node.id).collect::<Vec<_>>();
    if prefix_spacers {
        f.scenario.root.children.splice(0..0, spacers);
    } else {
        f.scenario.root.children.extend(spacers);
    }
    f.scenario.n_slots = 8;
    simthing_driver::resident_clearing_runtime::install_default_resident_rf_property(
        &mut f.scenario.registry,
        &mut f.scenario.root,
    );
    let mut initial = SlotAllocator::new();
    initial.install_initial_tree(&f.scenario.root).unwrap();
    spec.resource_flow = Some(admitted(&f, &initial, arenas_reversed));
    let layout = f.scenario.registry.property(pids[2]).layout.clone();
    let mut products = Vec::new();
    let mut profiles = Vec::new();
    for ordinal in 0..2 {
        let mut product = SimThing::new(SimThingKind::Cohort, 0);
        let mut component = SimThing::new(SimThingKind::Cohort, 0);
        for (node, base) in [(&mut product, 7.0), (&mut component, 9.0)] {
            let mut value = PropertyValue::from_layout(&layout);
            value.set_role(&role("balance"), &layout, base + ordinal as f32);
            node.add_property(pids[2], value);
        }
        let overlay = OverlayId::new();
        product.overlays.push(Overlay {
            id: overlay,
            kind: OverlayKind::Policy,
            source: OverlaySource::System,
            origin: product.id,
            affects: vec![product.id],
            transform: PropertyTransformDelta {
                property_id: pids[2],
                sub_field_deltas: vec![(role("balance"), TransformOp::set(17.0 + ordinal as f32))],
            },
            lifecycle: OverlayLifecycle::UntilDissolved,
        });
        profiles.push((product.id, component.id, overlay));
        product.add_child(component);
        products.push(product);
    }
    let mut session = SimSession::open_from_spec(f.scenario, &spec).unwrap();
    install_funded_birth_sequence(&mut session, ids[project], pids[2], products);
    let bound_slots = ids.map(|id| session.proto.allocator.slot_of(id).unwrap());
    let mut physical_rows = bound_slots.map(|slot| slot.raw());
    physical_rows.sort();
    // This assertion prevents a canonicalizing install from silently making
    // the physical-order dimension vacuous.
    assert_eq!(physical_rows, if prefix_spacers { [5, 6] } else { [1, 2] });
    let mut source_overlays = stop_joint_sources(&session, root, pids);
    for target in spacer_ids {
        session
            .tx
            .submit_boundary(BoundaryRequest::Remove { target })
            .unwrap();
    }
    let col = session
        .proto
        .registry
        .column_range(pids[2])
        .col_for_role(&role("balance"), &layout)
        .unwrap();
    let shape = (session.state.n_slots, session.proto.registry.total_columns);
    let mut first_placement = None;
    let mut trace = Vec::new();
    let mut births = Vec::new();
    let mut crossing_sources = Vec::new();
    for generation in 1..=8 {
        if generation == 4 {
            session
                .tx
                .submit_boundary(BoundaryRequest::Remove {
                    target: profiles[0].0,
                })
                .unwrap();
        }
        if generation == 4 || generation == 5 {
            source_overlays = replace_authored_source_flow(
                &session,
                root,
                pids,
                source_overlays,
                generation,
                if generation == 4 { 1.0 } else { 0.0 },
            );
        }
        session
            .step_once()
            .expect("independent ordering preserves ordinary funded continuation");
        let values = session.state.read_values();
        let actual = stock(&session, ids, pids, &values);
        let expected = match generation {
            1 => [[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]],
            2..=4 => [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]],
            5 => [[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]],
            _ => [[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]],
        };
        assert_eq!(
            actual, expected,
            "project={project} order={order:?} g={generation}"
        );
        for resource in 0..2 {
            assert_eq!(
                actual[0][resource] + actual[1][resource] + actual[0][2] + actual[1][2],
                if generation < 5 { 4.0 } else { 7.0 }
            );
        }
        assert_eq!(
            (session.state.n_slots, session.proto.registry.total_columns),
            shape
        );
        assert_eq!(
            ids.map(|id| session.proto.allocator.slot_of(id).unwrap()),
            bound_slots
        );
        let entries = session.proto.take_delta_log();
        assert!(!entries
            .iter()
            .any(|entry| matches!(entry, BoundaryDeltaEntry::GrowthResidencyRefused { .. })));
        for entry in &entries {
            if let BoundaryDeltaEntry::BandCrossingDeltasApplied { deltas } = entry {
                for delta in deltas {
                    assert_eq!(delta.sim_thing_id(), ids[project]);
                    assert_eq!(delta.property_id(), pids[2]);
                    assert_eq!(delta.slot(), bound_slots[project]);
                    assert_eq!(delta.col(), col);
                    crossing_sources.push(delta.generation());
                }
            }
        }
        for (ordinal, &(id, component, overlay)) in profiles.iter().enumerate() {
            let born_now = generation == if ordinal == 0 { 3 } else { 7 };
            let present = if ordinal == 0 {
                generation == 3
            } else {
                generation >= 7
            };
            for identity in [id, component] {
                assert_eq!(session.proto.root.contains_id(identity), present);
                assert_eq!(session.proto.allocator.slot_of(identity).is_some(), present);
                assert_eq!(
                    session.proto.allocator.relation_of(identity).is_some(),
                    present
                );
            }
            if !present {
                assert!(session
                    .proto
                    .allocator
                    .committed_residency_placement(root, id)
                    .is_none());
                continue;
            }
            let snapshot = session.proto.root.snapshot_node(id).unwrap();
            assert_eq!(snapshot.children, vec![component]);
            assert!(snapshot.property_ids.contains(&pids[2]));
            assert!(snapshot.overlay_ids.contains(&overlay));
            assert!(session
                .proto
                .root
                .snapshot_node(component)
                .unwrap()
                .property_ids
                .contains(&pids[2]));
            assert_eq!(
                session.proto.allocator.relation_of(id),
                Some(ObjectResidencyRelation::ChildOf(ids[project]))
            );
            assert_eq!(
                session.proto.allocator.relation_of(component),
                Some(ObjectResidencyRelation::ChildOf(id))
            );
            assert!(session
                .proto
                .root
                .snapshot_node(ids[project])
                .unwrap()
                .children
                .contains(&id));
            let placement = session
                .proto
                .allocator
                .committed_residency_placement(root, id)
                .unwrap();
            assert_eq!(placement.quantity(), 2);
            if born_now {
                assert!(entries.iter().any(|entry| matches!(entry, BoundaryDeltaEntry::SimThingAdded { parent, node, residency }
                    if *parent == ids[project] && node.id() == id && residency.placement() == placement)));
                births.push(id);
                if ordinal == 0 {
                    first_placement = Some(placement);
                } else {
                    let prior = first_placement.unwrap();
                    assert_eq!(
                        placement.extent(),
                        prior.extent(),
                        "ordinary product uses released extent"
                    );
                    assert_ne!(placement.identity(), prior.identity());
                }
            } else {
                for identity in [id, component] {
                    let slot = session.proto.allocator.slot_of(identity).unwrap();
                    assert_eq!(session.proto.allocator.owner_of(slot), Some(identity));
                    assert_eq!(
                        values[slot.as_usize() * shape.1 + col.raw_u32() as usize],
                        17.0 + ordinal as f32
                    );
                }
            }
        }
        let ordinal = session.action_band_execution_generation().unwrap();
        assert_eq!(
            ordinal,
            if generation < 2 {
                0
            } else if generation < 6 {
                1
            } else {
                2
            }
        );
        trace.push((actual, session.proto.allocator.live_count(), ordinal));
    }
    assert_eq!(
        births,
        profiles.iter().map(|profile| profile.0).collect::<Vec<_>>()
    );
    assert_ne!(profiles[0].0, profiles[1].0);
    assert_ne!(profiles[0].1, profiles[1].1);
    assert_ne!(profiles[0].2, profiles[1].2);
    assert_eq!(
        crossing_sources,
        vec![1, 5],
        "sealed source generations survive quiet boundaries"
    );
    println!("terminal independent order project={project} order={order:?} host_rows={bound_slots:?} births={births:?} source_generations={crossing_sources:?} trace={trace:?}");
    trace
}

#[test]
fn ordinary_construction_terminal_matrix() {
    let mut failures = terminal_identity_negatives();
    terminal_early_crossing_negative(&mut failures);
    let mut canonical = None;
    for project in 0..2 {
        for bits in 0..16 {
            let order = std::array::from_fn(|index| bits & (1 << index) != 0);
            let trace = terminal_order_case(project, order);
            if let Some(expected) = &canonical {
                assert_eq!(&trace, expected);
            } else {
                canonical = Some(trace);
            }
        }
    }
    assert!(
        failures.is_empty(),
        "ordinary integrated negatives must fail closed: {failures:?}"
    );
}

fn terminal_fail_stop(session: &mut SimSession, label: &str) {
    let before = (
        session.coord.tick_index(),
        session.coord.day_index(),
        session.integration_schedule().entries().len(),
        session.proto.allocator.binding_table_snapshot(),
        session.action_band_execution_generation(),
    );
    let values = session.state.read_values();
    for attempt in 0..3 {
        let error = session
            .step_once()
            .expect_err("touched invalid generation must stay stopped");
        println!("terminal fail-stop {label} retry={attempt}: {error:?}");
        assert!(
            matches!(error, simthing_driver::SessionError::ExecutionIdentity(ref message) if message.contains("fault"))
        );
        assert_eq!(session.state.read_values(), values);
        assert_eq!(
            (
                session.coord.tick_index(),
                session.coord.day_index(),
                session.integration_schedule().entries().len(),
                session.proto.allocator.binding_table_snapshot(),
                session.action_band_execution_generation()
            ),
            before
        );
    }
}

fn terminal_identity_negatives() -> Vec<String> {
    use simthing_driver::{ActionBandExecutionIngressError as Ingress, SessionError};
    use simthing_sim::BoundaryDeltaEntry;
    let mut failures = Vec::new();
    for project in 0..2 {
        for case in [
            "stale-binding",
            "foreign-binding",
            "remove-bound",
            "duplicate-product",
            "changed-crossing-threshold",
        ] {
            let (f, spec, pids) = build([4.0, 4.0], project == 1, false, &["p", "q"]);
            let root = f.scenario.root.id;
            let ids = f.projects;
            let mut first = SimThing::new(SimThingKind::Cohort, 0);
            let component = SimThing::new(SimThingKind::Cohort, 0);
            let component_id = component.id;
            first.add_child(component);
            let second = if case == "duplicate-product" {
                first.clone()
            } else {
                SimThing::new(SimThingKind::Cohort, 0)
            };
            let products = [first.id, second.id];
            let mut session = SimSession::open_from_spec(f.scenario, &spec).unwrap();
            install_funded_birth_sequence(&mut session, ids[project], pids[2], vec![first, second]);
            let mut source_overlays = stop_joint_sources(&session, root, pids);
            for _ in 0..3 {
                session.step_once().unwrap();
            }
            assert!(session.proto.root.contains_id(products[0]));
            let first_placement = session
                .proto
                .allocator
                .committed_residency_placement(root, products[0])
                .unwrap();
            session.proto.take_delta_log();
            if case == "stale-binding" || case == "foreign-binding" {
                // Adversarial negative plants only: no dispatch, alternate
                // policy, or repair occurs through these allocator mutations.
                if case == "stale-binding" {
                    let mut changed = session.proto.allocator.binding_table_snapshot();
                    let a = changed[&ids[project]];
                    let b = changed[&component_id];
                    changed.insert(ids[project], b);
                    changed.insert(component_id, a);
                    session
                        .proto
                        .allocator
                        .epoch_rebind(&changed, &Default::default(), &Default::default())
                        .unwrap();
                } else {
                    let mut foreign = SimThing::new(SimThingKind::World, 0);
                    for _ in 0..4 {
                        foreign.add_child(SimThing::new(SimThingKind::Cohort, 0));
                    }
                    let mut allocator = SlotAllocator::new();
                    allocator.install_initial_tree(&foreign).unwrap();
                    session.proto.allocator = allocator;
                }
                let before = (
                    session.coord.tick_index(),
                    session.coord.day_index(),
                    session.integration_schedule().entries().len(),
                    session.action_band_execution_generation(),
                );
                let values = session.state.read_values();
                for attempt in 0..3 {
                    let error = session
                        .step_once()
                        .expect_err("foreign/stale binding must refuse before GPU work");
                    println!("terminal negative project={project} case={case} attempt={attempt}: {error:?}");
                    assert!(matches!(
                        error,
                        SessionError::ActionBandIngress(Ingress::BindingTableStale)
                    ));
                    assert_eq!(
                        (
                            session.coord.tick_index(),
                            session.coord.day_index(),
                            session.integration_schedule().entries().len(),
                            session.action_band_execution_generation()
                        ),
                        before
                    );
                    assert_eq!(session.state.read_values(), values);
                }
                continue;
            }
            if case == "remove-bound" {
                session
                    .tx
                    .submit_boundary(BoundaryRequest::Remove {
                        target: ids[project],
                    })
                    .unwrap();
                let error = session
                    .step_once()
                    .expect_err("installed crossing participant cannot be retired silently");
                println!("terminal negative project={project} case={case}: {error:?}");
                assert!(
                    matches!(error, SessionError::ActionBandIngress(Ingress::BoundIdentityRemapped { id }) if id == u64::from(ids[project].raw()))
                );
                assert!(!session.proto.root.contains_id(ids[project]));
                assert!(!session.proto.root.contains_id(products[0]));
                assert!(session
                    .proto
                    .allocator
                    .committed_residency_placement(root, products[0])
                    .is_none());
                terminal_fail_stop(&mut session, case);
                continue;
            }
            if case == "changed-crossing-threshold" {
                // Keep the installed structural table and its registration
                // indices/participant unchanged, but replace the second source
                // with a DIFFERENT threshold via the public alert door.
                // The production builder/GPU mint creates the evidence. No
                // BandCrossingDelta, emission token, or generation is forged.
                let mut alerts = session.proto.velocity_alerts().to_vec();
                assert_eq!(alerts.len(), 2);
                alerts[1].threshold = 1.25;
                session.proto.clear_velocity_alerts();
                for alert in alerts {
                    session.proto.register_velocity_alert(alert);
                }
            }
            let mut rejected = false;
            let mut changed_crossing_seen = false;
            for generation in 4..=8 {
                if generation == 4 || generation == 5 {
                    source_overlays = replace_authored_source_flow(
                        &session,
                        root,
                        pids,
                        source_overlays,
                        generation,
                        if generation == 4 { 1.0 } else { 0.0 },
                    );
                }
                let result = session.step_once();
                let values = session.state.read_values();
                let actual = stock(&session, ids, pids, &values);
                let entries = session.proto.take_delta_log();
                let crossings = entries
                    .iter()
                    .filter_map(|entry| match entry {
                        BoundaryDeltaEntry::BandCrossingDeltasApplied { deltas } => Some(deltas),
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                changed_crossing_seen |= crossings
                    .iter()
                    .flat_map(|deltas| deltas.iter())
                    .any(|delta| delta.reg_idx() == 1 && delta.threshold() == 1.25);
                println!("terminal negative project={project} case={case} g={generation} result={result:?} stock={actual:?} products={:?} action_generation={:?} crossings={crossings:?}", products.map(|id| session.proto.root.contains_id(id)), session.action_band_execution_generation());
                if let Err(error) = result {
                    if case == "duplicate-product" {
                        assert!(
                            matches!(&error, SessionError::GpuSync(simthing_sim::GpuSyncError::GrowthEntitlement(message)) if message.contains(&format!("growth grantee {} is already resident", products[0].raw()))),
                            "duplicate identity typed provenance: {error:?}"
                        );
                        assert_eq!(generation, 7);
                        assert_eq!(
                            session
                                .proto
                                .allocator
                                .committed_residency_placement(root, products[0]),
                            Some(first_placement)
                        );
                        assert_eq!(session.proto.allocator.live_count(), 5);
                    } else {
                        assert!(
                            matches!(&error, SessionError::ActionBandIngress(_)),
                            "crossing provenance typed ingress refusal: {error:?}"
                        );
                        assert!(!session.proto.root.contains_id(products[1]));
                    }
                    terminal_fail_stop(&mut session, case);
                    rejected = true;
                    break;
                }
            }
            if !rejected {
                if case == "changed-crossing-threshold" {
                    assert!(
                        changed_crossing_seen,
                        "negative must reach the canonical sealed crossing door"
                    );
                }
                failures.push(format!("project={project} case={case}: accepted invalid provenance/identity through G8"));
            }
        }
    }
    failures
}

fn terminal_early_crossing_negative(failures: &mut Vec<String>) {
    use simthing_driver::{ActionBandExecutionIngressError as Ingress, SessionError};
    use simthing_sim::BoundaryDeltaEntry;
    for project in 0..2 {
        // Same public clear/re-register operation in both arms. The control
        // preserves the frozen 1.5 threshold; the negative changes it to 0.75
        // after commitments are installed, before ordinary G1. A real sealed
        // 0.75 crossing cannot authorize the second frozen 1.5-bound product.
        for threshold in [1.5, 0.75] {
            let (f, spec, pids) = build([4.0, 4.0], project == 1, false, &["p", "q"]);
            let root = f.scenario.root.id;
            let ids = f.projects;
            let products = [
                SimThing::new(SimThingKind::Cohort, 0),
                SimThing::new(SimThingKind::Cohort, 0),
            ];
            let product_ids = products.each_ref().map(|product| product.id);
            let mut session = SimSession::open_from_spec(f.scenario, &spec).unwrap();
            install_funded_birth_sequence(&mut session, ids[project], pids[2], products.to_vec());
            stop_joint_sources(&session, root, pids);
            let mut alerts = session.proto.velocity_alerts().to_vec();
            assert_eq!(alerts[1].threshold, 1.5);
            alerts[1].threshold = threshold;
            session.proto.clear_velocity_alerts();
            for alert in alerts {
                session.proto.register_velocity_alert(alert);
            }
            let mut rejected = false;
            let mut source_seen = false;
            for generation in 1..=4 {
                let result = session.step_once();
                let actual = stock(&session, ids, pids, &session.state.read_values());
                let entries = session.proto.take_delta_log();
                let crossings = entries
                    .iter()
                    .filter_map(|entry| match entry {
                        BoundaryDeltaEntry::BandCrossingDeltasApplied { deltas } => Some(deltas),
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                source_seen |= crossings
                    .iter()
                    .flat_map(|deltas| deltas.iter())
                    .any(|delta| {
                        delta.reg_idx() == 1
                            && delta.threshold() == threshold
                            && delta.post_value() == 1.0
                    });
                println!("terminal early crossing project={project} frozen=1.5 runtime={threshold} g={generation} result={result:?} stock={actual:?} products={:?} action_generation={:?} crossings={crossings:?}", product_ids.map(|id| session.proto.root.contains_id(id)), session.action_band_execution_generation());
                if generation == 3 {
                    for id in product_ids {
                        println!("terminal early product id={id:?} parent={:?} slot={:?} placement={:?} added={}",
                            session.proto.allocator.relation_of(id), session.proto.allocator.slot_of(id),
                            session.proto.allocator.committed_residency_placement(root, id),
                            entries.iter().any(|entry| matches!(entry, BoundaryDeltaEntry::SimThingAdded { node, .. } if node.id() == id)));
                    }
                }
                if let Err(error) = result {
                    assert_eq!(
                        threshold, 0.75,
                        "unchanged-registration control must remain healthy"
                    );
                    assert!(
                        matches!(error, SessionError::ActionBandIngress(_)),
                        "typed crossing ingress refusal: {error:?}"
                    );
                    assert!(!session.proto.root.contains_id(product_ids[1]));
                    // A stale-admission preflight is allowed to refuse before
                    // touching G1. A dispatch refusal after work is fail-stop.
                    if matches!(error, SessionError::ActionBandIngress(Ingress::Dispatch(_))) {
                        terminal_fail_stop(&mut session, "early-crossing");
                    }
                    rejected = true;
                    break;
                }
                if generation >= 2 {
                    assert_eq!(actual, [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]);
                }
                if threshold == 1.5 {
                    assert_eq!(
                        session.proto.root.contains_id(product_ids[0]),
                        generation >= 3
                    );
                    assert!(!session.proto.root.contains_id(product_ids[1]));
                }
            }
            if threshold == 0.75 && !rejected {
                assert!(
                    source_seen,
                    "real production-minted mismatched crossing must be exercised"
                );
                failures.push(format!("project={project} frozen=1.5 runtime=0.75: no typed refusal; second_product_present={}", session.proto.root.contains_id(product_ids[1])));
            }
        }
    }
}
