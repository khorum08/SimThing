//! UNIFIED-FACILITY-CONVERGENCE-WITNESS-0 composed production witness.
//!
//! The predecessor proof
//! `six_real_doors_publish_conserved_sparse_lanes_and_cross_actionband_without_rebind`
//! establishes the real grant-to-resident-lane arrow used here. This witness
//! extends that production arrow through the ordinary ActionBand routed
//! consequence door and the canonical boundary/delta-log replay authority.

use std::collections::BTreeSet;

use simthing_core::owner_channel::OwnerRef;
use simthing_core::{
    eml_opcode, grant_disbursement_capacity_overlay, grant_disbursement_capacity_property,
    grant_disbursement_capacity_value, DimensionRegistry, Direction, EmitOnThresholdBuffer,
    EmitOnThresholdRegistration, EmlConsumerMask, EmlExecutionClass, EmlExpressionRegistry,
    EmlFormulaMeta, EmlTreeId, GenerationStamp, Overlay, OverlayId, OverlayKind, OverlayLifecycle,
    OverlaySource, PropertyTransformDelta, SimProperty, SimThing, SimThingId, SimThingKind,
    SlotIndex, SubFieldRole, ThresholdDirection, TransformOp, GRANT_LANE_OCCUPIED,
};
use simthing_driver::{
    compile_crossing_consequence_session, ActionBandActiveInstance, ActionBandNativeLaneAdmission,
    RoutedOverlayDelivery, Scenario, SimSession,
};
use simthing_sim::{
    BoundaryDeltaEntry, CostBandSemantic, ReplayDriver, ReplayFrame, ThresholdRegistry,
    VelocityAlertRegistration,
};
use simthing_spec::{
    admit_specialization_flow_market, clear_constrained_claims_at_generation,
    ActionBandAdmissionBudgetSpec, ActionBandBandSpec, ActionBandChannelBindingSpec,
    ActionBandChannelKind, ActionBandSessionBuildDoor, ActionBandSessionSpec, ActionBandTargetSpec,
    ActionBandTemplateSpec, AdmittedSpecializationFlowMarket, AuthoredClearingProgram,
    ClearingRemainderAuthority, ConservedOfferingSpec, ConstrainedClaim, ConstrainedGrant,
    ConstrainedSupply, DrawEnvelopeTemplateSpec, OfferingPriceVectorSpec, OwnerChannelScopeKey,
    ResourceKey, RuntimeOwnerSiloDemandBucket, ScalarBoundDirection, ScopeId,
    SpecializationFlowMarketSpec,
};

const EVENT_PROGRAM: u32 = 88_430;
const HORIZON: u32 = 4;
const CROSSING_QUANTITY: u32 = 3;
const CROSSING_THRESHOLD: f32 = 2.0;
const PASSTHROUGH_QUANTITY: u32 = 1;

struct Fixture {
    scenario: Scenario,
    lane_property: simthing_core::SimPropertyId,
    effect_property: simthing_core::SimPropertyId,
    source: SimThingId,
    bypass_source: SimThingId,
    granter: SimThingId,
    terminal: SimThingId,
    bypass_terminal: SimThingId,
}

fn fixture() -> Fixture {
    let mut registry = DimensionRegistry::new();
    let lane_property = grant_disbursement_capacity_property();
    let lane_layout = lane_property.layout.clone();
    let lane_property_id = registry.register(lane_property);
    let effect_property = registry.register(SimProperty::simple(
        "unified-facility-convergence",
        "terminal-effect",
        0,
    ));

    let resident_lane = |label: &str| {
        let mut node = SimThing::new(SimThingKind::Custom(label.to_string()), 0);
        node.add_property(
            lane_property_id,
            grant_disbursement_capacity_value(&lane_layout, 20),
        );
        node.overlays.push(grant_disbursement_capacity_overlay(
            node.id,
            lane_property_id,
            20,
        ));
        node
    };
    let source = resident_lane("load-bearing-resident");
    let bypass_source = resident_lane("residency-passthrough");
    let granter = SimThing::new(SimThingKind::Custom("real-granter".into()), 0);

    let terminal_node = |label: &str| {
        let mut node = SimThing::new(SimThingKind::Custom(label.to_string()), 0);
        node.add_property(
            effect_property,
            registry.property(effect_property).default_value(),
        );
        node
    };
    let terminal = terminal_node("authoritative-terminal");
    let bypass_terminal = terminal_node("overlay-passthrough-terminal");
    let ids = (
        source.id,
        bypass_source.id,
        granter.id,
        terminal.id,
        bypass_terminal.id,
    );

    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    root.add_child(source);
    root.add_child(bypass_source);
    root.add_child(granter);
    root.add_child(terminal);
    root.add_child(bypass_terminal);

    Fixture {
        scenario: Scenario {
            name: "unified-facility-convergence-witness-0".into(),
            ticks_per_day: 1,
            max_days: 16,
            dt: 1.0,
            n_slots: 6,
            registry,
            root,
            shadow_seeds: Vec::new(),
            tick_patches: Vec::new(),
            install_targets: Default::default(),
        },
        lane_property: lane_property_id,
        effect_property,
        source: ids.0,
        bypass_source: ids.1,
        granter: ids.2,
        terminal: ids.3,
        bypass_terminal: ids.4,
    }
}

fn admitted_market() -> AdmittedSpecializationFlowMarket {
    admit_specialization_flow_market(
        &simthing_core::seed_profiles(),
        &BTreeSet::from(["while-active".to_string()]),
        SpecializationFlowMarketSpec {
            specialization_profile_id: "session-root".into(),
            offerings: vec![ConservedOfferingSpec {
                id: "lane-capacity".into(),
                resource_key: ResourceKey::new("grant-capacity"),
                price: OfferingPriceVectorSpec {
                    unit_cost: 1.0,
                    default_clearing_weight: 1.0,
                },
            }],
            draw_envelopes: vec![DrawEnvelopeTemplateSpec {
                id: "lane-draw".into(),
                offering_refs: vec!["lane-capacity".into()],
                lifecycle_trigger_refs: vec!["while-active".into()],
                min_quantity: 1,
                max_quantity: 20,
            }],
        },
    )
    .expect("the graduated market admits")
}

fn clear_grant(
    granter: SimThingId,
    grantee: SimThingId,
    quantity: u32,
    generation: GenerationStamp,
) -> ConstrainedGrant {
    let scope = OwnerChannelScopeKey {
        owner_ref: OwnerRef::new("unified-facility-owner"),
        resource_key: ResourceKey::new("grant-capacity"),
        scope_id: ScopeId::from_boundary(granter),
    };
    let demand = RuntimeOwnerSiloDemandBucket {
        owner_ref: scope.owner_ref.clone(),
        resource_key: scope.resource_key.clone(),
        scope_id: scope.scope_id.clone(),
        requested: quantity,
        priority: 0,
        source_simthing_id_raw: Some(grantee.raw()),
    };
    let claim = ConstrainedClaim::from_runtime_demand(&demand, 1.0).unwrap();
    clear_constrained_claims_at_generation(
        &[ConstrainedSupply {
            scope,
            available: quantity,
        }],
        &[claim],
        &AuthoredClearingProgram::new(TransformOp::set(1.0)),
        ClearingRemainderAuthority {
            granter,
            generation,
        },
    )
    .unwrap()[0]
        .grants[0]
        .clone()
}

fn lane_column(
    registry: &DimensionRegistry,
    property_id: simthing_core::SimPropertyId,
) -> simthing_core::ColumnIndex {
    registry
        .column_range(property_id)
        .col_for_role(
            &SubFieldRole::Named(GRANT_LANE_OCCUPIED.to_string()),
            &registry.property(property_id).layout,
        )
        .unwrap()
}

fn routed_overlay(origin: SimThingId, property_id: simthing_core::SimPropertyId) -> Overlay {
    Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Instruction,
        source: OverlaySource::Ai,
        origin,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id,
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::set(9.0))],
        },
        lifecycle: OverlayLifecycle::UntilDissolved,
    }
}

fn install_action_band(
    session: &mut SimSession,
    fixture: &Fixture,
    source_threshold: f32,
    source_route: SimThingId,
) -> [OverlayId; 2] {
    let occupied = lane_column(&session.proto.registry, fixture.lane_property);
    let source_slot = session.proto.allocator.slot_of(fixture.source).unwrap();
    let bypass_slot = session
        .proto
        .allocator
        .slot_of(fixture.bypass_source)
        .unwrap();
    for sim_thing_id in [fixture.source, fixture.bypass_source] {
        session
            .proto
            .register_velocity_alert(VelocityAlertRegistration {
                sim_thing_id,
                property_id: fixture.lane_property,
                sub_field: SubFieldRole::Named(GRANT_LANE_OCCUPIED.to_string()),
                threshold: if sim_thing_id == fixture.source {
                    source_threshold
                } else {
                    CROSSING_THRESHOLD
                },
                direction: Direction::Rising,
                cost_band: CostBandSemantic::observation(),
            });
    }
    session
        .proto
        .initial_gpu_sync(&session.coord, &mut session.state)
        .expect("pre-open thresholds use the ordinary boundary compiler");
    let thresholds = vec![
        EmitOnThresholdRegistration {
            slot: source_slot,
            col: occupied,
            threshold: source_threshold,
            direction: ThresholdDirection::Upward,
            event_kind: 0,
            buffer: EmitOnThresholdBuffer::Values,
        },
        EmitOnThresholdRegistration {
            slot: bypass_slot,
            col: occupied,
            threshold: CROSSING_THRESHOLD,
            direction: ThresholdDirection::Upward,
            event_kind: 1,
            buffer: EmitOnThresholdBuffer::Values,
        },
    ];

    let program = EmlTreeId(EVENT_PROGRAM);
    let mut nodes = TransformOp::set(1.0).to_eml_nodes();
    nodes.push(simthing_core::eml_nodes::EmlNode {
        opcode: eml_opcode::RETURN_TOP,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    });
    let mut eml = EmlExpressionRegistry::new();
    eml.register_formula(
        program,
        EmlFormulaMeta {
            tree_id: program,
            execution_class: EmlExecutionClass::ExactDeterministic,
            allowed_consumers: EmlConsumerMask(EmlConsumerMask::ALL_PRODUCTION),
            max_abs_error: None,
            deterministic_gpu: true,
            requires_guard_for_hard_threshold: false,
            node_count: nodes.len() as u32,
            max_stack_depth: 1,
            has_loops: false,
            has_recursion: false,
            display_name: "unified-facility-crossing".into(),
        },
        nodes,
    )
    .unwrap();

    let template =
        |id: &str, threshold_registration_index: u32, emission_index: u32| ActionBandTemplateSpec {
            id: id.into(),
            label: None,
            axis_channels: vec![ActionBandChannelBindingSpec {
                column: occupied.raw_u32(),
                kind: ActionBandChannelKind::Primitive,
            }],
            target: ActionBandTargetSpec::ScalarBound {
                channel: occupied.raw_u32(),
                bound: thresholds[threshold_registration_index as usize].threshold,
                direction: ScalarBoundDirection::AtLeast,
            },
            velocity: None,
            bands: vec![ActionBandBandSpec {
                threshold_registration_index,
                eml_program: Some(program.0),
                emission_binding_indices: vec![emission_index],
            }],
            subordinate_template_ids: Vec::new(),
            max_active_subordinates: 0,
            reserved_instance_rows: 1,
            requirement_semantics: Default::default(),
        };
    let spec = ActionBandSessionSpec {
        budget: ActionBandAdmissionBudgetSpec {
            axis_channel_count: 2,
            dependency_binding_count: 0,
            storage_rows: 2,
            eml_program_count: 1,
            emission_binding_count: 2,
        },
        templates: vec![
            template("load-bearing", 0, 0),
            template("passthrough", 1, 1),
        ],
    };
    let mut door = ActionBandSessionBuildDoor::new();
    let frozen = door
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

    let source_overlay = routed_overlay(fixture.source, fixture.effect_property);
    let bypass_overlay = routed_overlay(fixture.bypass_source, fixture.effect_property);
    let overlay_ids = [source_overlay.id, bypass_overlay.id];
    let consequences = [
        RoutedOverlayDelivery::admit(source_route, source_overlay).unwrap(),
        RoutedOverlayDelivery::admit(fixture.bypass_terminal, bypass_overlay).unwrap(),
    ];
    let commitments = compile_crossing_consequence_session(
        &frozen,
        &eml,
        &consequences,
        &[
            ActionBandActiveInstance::new(
                frozen.templates()[0].index(),
                SlotIndex::new(source_slot.raw()),
                [0.0; 4],
            ),
            ActionBandActiveInstance::new(
                frozen.templates()[1].index(),
                SlotIndex::new(bypass_slot.raw()),
                [0.0; 4],
            ),
        ],
        &lanes,
    )
    .unwrap();
    session
        .install_action_band_commitments(commitments)
        .unwrap();
    overlay_ids
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Case {
    EmergenceControl,
    Emergence,
    ResidencyPassthrough,
    GrantingPassthrough,
    CrossingPassthrough,
    OverlayPassthrough,
}

#[derive(Debug)]
struct Evidence {
    terminal_profile: Vec<usize>,
    replay_profile: Vec<usize>,
    first_crossing_generation: Option<u32>,
    first_delivery_generation: Option<u32>,
    first_consequence_generation: Option<u32>,
    crossing_count: u64,
    routed_delivery_count: u64,
    grant_fact_count: usize,
    resident_capacity: usize,
    admitted_dimensions: usize,
    action_band_generation_at_admission: u32,
    replay_used_shadow_checkpoint: bool,
}

fn terminal_observable(root: &simthing_sim::SimRuntimeTree, terminal: SimThingId) -> usize {
    root.overlay_count(terminal)
        .expect("the authoritative terminal remains resident")
}

fn run_case(case: Case) -> Evidence {
    let fixture = fixture();
    let market = admitted_market();
    let mut session = SimSession::open(fixture.scenario.clone()).expect("GPU session opens");
    let replay_snapshot = session.proto.snapshot(0);

    let source_threshold = if case == Case::CrossingPassthrough {
        (CROSSING_QUANTITY + 1) as f32
    } else {
        CROSSING_THRESHOLD
    };
    let source_route = if case == Case::OverlayPassthrough {
        fixture.bypass_terminal
    } else {
        fixture.terminal
    };
    let routed_ids = install_action_band(&mut session, &fixture, source_threshold, source_route);

    // Carry is measured at the admitted, pre-run cardinality. No compaction or
    // summary surface exists between this measurement and session admission.
    for node in [
        fixture.source,
        fixture.bypass_source,
        fixture.granter,
        fixture.terminal,
        fixture.bypass_terminal,
    ] {
        assert!(session.proto.allocator.slot_of(node).is_some());
    }
    let resident_capacity = session.proto.allocator.capacity();
    let admitted_dimensions = session.proto.registry.total_columns;
    let action_band_generation_at_admission = session.action_band_execution_generation().unwrap();

    let grants = match case {
        Case::EmergenceControl => vec![(fixture.source, PASSTHROUGH_QUANTITY)],
        Case::Emergence | Case::OverlayPassthrough => {
            vec![(fixture.source, CROSSING_QUANTITY)]
        }
        Case::ResidencyPassthrough => vec![(fixture.bypass_source, CROSSING_QUANTITY)],
        Case::GrantingPassthrough => vec![
            (fixture.source, PASSTHROUGH_QUANTITY),
            (fixture.bypass_source, CROSSING_QUANTITY),
        ],
        Case::CrossingPassthrough => vec![
            (fixture.source, CROSSING_QUANTITY),
            (fixture.bypass_source, CROSSING_QUANTITY),
        ],
    };
    for (grantee, quantity) in grants {
        let cleared = clear_grant(fixture.granter, grantee, quantity, GenerationStamp::new(0));
        session
            .record_cleared_market_grant(&market, fixture.granter, "lane-capacity", &cleared)
            .unwrap();
    }

    let mut terminal_profile = vec![terminal_observable(&session.proto.root, fixture.terminal)];
    let mut frames = Vec::new();
    let mut first_crossing_generation = None;
    let mut first_delivery_generation = None;
    let mut first_consequence_generation = None;
    let mut crossing_count = 0;
    let mut routed_delivery_count = 0;
    for generation in 1..=HORIZON {
        let summary = session.run(1).expect("every variant remains a healthy run");
        assert_eq!(summary.boundaries_run, 1);
        if summary.action_band_crossings > 0 && first_crossing_generation.is_none() {
            first_crossing_generation = Some(generation);
        }
        if summary.action_band_routed_deliveries > 0 && first_delivery_generation.is_none() {
            first_delivery_generation = Some(generation);
        }
        crossing_count += summary.action_band_crossings;
        routed_delivery_count += summary.action_band_routed_deliveries;

        let entries = session.proto.take_delta_log();
        if first_consequence_generation.is_none()
            && entries.iter().any(|entry| {
                matches!(entry, BoundaryDeltaEntry::OverlayAttached { overlay, .. }
                    if routed_ids.contains(&overlay.id))
            })
        {
            first_consequence_generation = Some(generation);
        }
        frames.push(ReplayFrame {
            day: generation,
            entries,
            shadow_values: None,
            spec_entries: Vec::new(),
            injection_entries: Vec::new(),
        });
        terminal_profile.push(terminal_observable(&session.proto.root, fixture.terminal));
    }

    let mut replay = ReplayDriver::from_snapshot(replay_snapshot).unwrap();
    let mut replay_profile = vec![terminal_observable(&replay.root, fixture.terminal)];
    for frame in frames {
        replay.apply_frame(frame);
        replay_profile.push(terminal_observable(&replay.root, fixture.terminal));
    }
    assert_eq!(replay_profile, terminal_profile);

    Evidence {
        terminal_profile,
        replay_profile,
        first_crossing_generation,
        first_delivery_generation,
        first_consequence_generation,
        crossing_count,
        routed_delivery_count,
        grant_fact_count: replay.grant_lifecycle_facts.len(),
        resident_capacity,
        admitted_dimensions,
        action_band_generation_at_admission,
        replay_used_shadow_checkpoint: replay.shadow_values.is_some(),
    }
}

#[test]
fn unified_facilities_compose_at_generation_speed_and_four_passthroughs_stay_green() {
    let control = run_case(Case::EmergenceControl);
    let emergence = run_case(Case::Emergence);
    let residency = run_case(Case::ResidencyPassthrough);
    let granting = run_case(Case::GrantingPassthrough);
    let crossing = run_case(Case::CrossingPassthrough);
    let overlay = run_case(Case::OverlayPassthrough);

    // One authored data-only perturbation: accepted quantity 1 -> 3. The
    // canonical terminal tree does not move until the real grant publication,
    // ordinary crossing, and following-boundary routed consequence all occur.
    assert_eq!(control.terminal_profile, vec![0, 0, 0, 0, 0]);
    assert_eq!(
        emergence.terminal_profile,
        vec![0, 0, 0, 1, 1],
        "{emergence:?}"
    );
    assert_eq!(emergence.first_crossing_generation, Some(2));
    assert_eq!(emergence.first_delivery_generation, Some(2));
    assert_eq!(emergence.first_consequence_generation, Some(3));
    assert_eq!(emergence.crossing_count, 1);
    assert_eq!(emergence.routed_delivery_count, 1);
    assert_eq!(emergence.grant_fact_count, 1);

    // Each A1 passthrough completes the same horizon. A real secondary route
    // keeps granting, crossing, and overlay execution live where necessary,
    // while the one authoritative terminal remains static for the named leg.
    for red in [&residency, &granting, &crossing, &overlay] {
        assert_eq!(red.terminal_profile, vec![0, 0, 0, 0, 0]);
        assert_eq!(red.replay_profile, red.terminal_profile);
        assert_eq!(red.crossing_count, 1);
        assert_eq!(red.routed_delivery_count, 1);
        assert_eq!(red.first_consequence_generation, Some(3));
    }
    assert_eq!(residency.grant_fact_count, 1);
    assert_eq!(granting.grant_fact_count, 2);
    assert_eq!(crossing.grant_fact_count, 2);
    assert_eq!(overlay.grant_fact_count, 1);

    // Admitted carry is exact and is observed before every run: root + five
    // real residents, the four-lane grant property + three effect columns,
    // and the one frozen ActionBand facility generation.
    for evidence in [
        &control, &emergence, &residency, &granting, &crossing, &overlay,
    ] {
        assert_eq!(evidence.resident_capacity, 6);
        assert_eq!(evidence.admitted_dimensions, 7);
        assert_eq!(evidence.action_band_generation_at_admission, 0);
        assert!(!evidence.replay_used_shadow_checkpoint);
    }
}
