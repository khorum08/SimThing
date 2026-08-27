//! GRANT-DISBURSEMENT-LANE-0 integrated production and replay witness.

use std::collections::BTreeSet;

use simthing_core::owner_channel::OwnerRef;
use simthing_core::{
    eml_opcode, grant_disbursement_capacity_overlay, grant_disbursement_capacity_property,
    grant_disbursement_capacity_value, DimensionRegistry, Direction, EmitOnThresholdBuffer,
    EmitOnThresholdRegistration, EmlConsumerMask, EmlExecutionClass, EmlExpressionRegistry,
    EmlFormulaMeta, EmlTreeId, GenerationStamp, GrantLifecycleFactKind, IntegrationScheduleRowKind,
    PropertyTransformDelta, SimProperty, SimThing, SimThingId, SimThingKind, SlotIndex,
    SubFieldRole, ThresholdDirection, TransformOp, GRANT_DISBURSEMENT_NAMESPACE,
    GRANT_DISBURSEMENT_PROPERTY, GRANT_LANE_CAPACITY, GRANT_LANE_FREE, GRANT_LANE_IN_FLIGHT,
    GRANT_LANE_OCCUPIED,
};
use simthing_driver::{
    compile_crossing_consequence_session, ActionBandActiveInstance, ActionBandNativeLaneAdmission,
    CrossingConsequenceBinding, Scenario, SimSession,
};
use simthing_feeder::patcher::ShadowFreshness;
use simthing_feeder::{BoundaryRequest, PatchTransform, PatcherStats, TransformPatcher};
use simthing_gpu::{ActionBandEmissionBindingGpu, ActionBandPropertyWrite};
use simthing_sim::{
    BoundaryDeltaEntry, CostBandSemantic, ReplayDriver, ReplayError, ReplayFrame,
    ReplayGrowthError, ThresholdRegistry, VelocityAlertRegistration,
};
use simthing_spec::{
    admit_specialization_flow_market, clear_constrained_claims_at_generation,
    ActionBandAdmissionBudgetSpec, ActionBandBandSpec, ActionBandChannelBindingSpec,
    ActionBandChannelKind, ActionBandSessionBuildDoor, ActionBandSessionSpec, ActionBandTargetSpec,
    ActionBandTemplateSpec, AdmittedSpecializationFlowMarket, AuthoredClearingProgram,
    ClearingRemainderAuthority, ConservedOfferingSpec, ConstrainedClaim, ConstrainedGrant,
    ConstrainedSupply, DrawEnvelopeTemplateSpec, GrantReleaseCause, OfferingPriceVectorSpec,
    OwnerChannelScopeKey, ResourceKey, RuntimeOwnerSiloDemandBucket, ScalarBoundDirection, ScopeId,
    SpecializationFlowMarketSpec,
};

const EVENT_KIND: u32 = 88_410;

struct LaneFixture {
    scenario: Scenario,
    property_id: simthing_core::SimPropertyId,
    effect_id: simthing_core::SimPropertyId,
    source: SimThingId,
    left: SimThingId,
    right: SimThingId,
    fused: SimThingId,
}

fn lane_fixture() -> LaneFixture {
    let mut registry = DimensionRegistry::new();
    let lane_property = grant_disbursement_capacity_property();
    let lane_layout = lane_property.layout.clone();
    let property_id = registry.register(lane_property);
    let effect_id = registry.register(SimProperty::simple("grant-lane-proof", "effect", 0));

    let granting_node = |label: &str| {
        let mut node = SimThing::new(SimThingKind::Custom(label.to_string()), 0);
        node.add_property(
            property_id,
            grant_disbursement_capacity_value(&lane_layout, 20),
        );
        node.overlays.push(grant_disbursement_capacity_overlay(
            node.id,
            property_id,
            20,
        ));
        node
    };
    let mut source = granting_node("source");
    source.add_property(effect_id, registry.property(effect_id).default_value());
    let left = granting_node("left");
    let right = granting_node("right");
    let fused = granting_node("fused");
    let ids = (source.id, left.id, right.id, fused.id);

    // The inactive sibling proves semantic sparsity: the registry has the
    // column, but this logical node does not carry the property.
    let inactive = SimThing::new(SimThingKind::Custom("inactive".into()), 0);
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    root.add_child(source);
    root.add_child(left);
    root.add_child(right);
    root.add_child(fused);
    root.add_child(inactive);

    LaneFixture {
        scenario: Scenario {
            name: "grant-disbursement-lane-0".into(),
            ticks_per_day: 1,
            max_days: 32,
            dt: 1.0,
            n_slots: 6,
            registry,
            root,
            shadow_seeds: Vec::new(),
            tick_patches: Vec::new(),
            install_targets: Default::default(),
        },
        property_id,
        effect_id,
        source: ids.0,
        left: ids.1,
        right: ids.2,
        fused: ids.3,
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
    .expect("grant lane market admits")
}

fn clear_grant(
    granter: SimThingId,
    grantee: SimThingId,
    quantity: u32,
    generation: GenerationStamp,
) -> ConstrainedGrant {
    let scope = OwnerChannelScopeKey {
        owner_ref: OwnerRef::new("grant-lane-owner"),
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
    lane: &str,
) -> simthing_core::ColumnIndex {
    registry
        .column_range(property_id)
        .col_for_role(
            &SubFieldRole::Named(lane.to_string()),
            &registry.property(property_id).layout,
        )
        .unwrap()
}

fn lane_values(
    session: &SimSession,
    node: SimThingId,
    property_id: simthing_core::SimPropertyId,
) -> [u32; 4] {
    let values = session.state.read_values();
    let slot = session.proto.allocator.slot_of(node).unwrap().raw() as usize;
    let base = slot * session.state.n_dims as usize;
    [
        GRANT_LANE_FREE,
        GRANT_LANE_IN_FLIGHT,
        GRANT_LANE_OCCUPIED,
        GRANT_LANE_CAPACITY,
    ]
    .map(|name| values[base + lane_column(&session.proto.registry, property_id, name).raw()] as u32)
}

fn replay_lane_values(
    replay: &ReplayDriver,
    node: SimThingId,
    property_id: simthing_core::SimPropertyId,
) -> [u32; 4] {
    let n_dims = replay.registry.total_columns;
    let mut values = vec![0.0; replay.allocator.capacity() * n_dims];
    replay
        .root
        .project_to_values(&replay.registry, &replay.allocator, n_dims, &mut values);
    let slot = replay.allocator.slot_of(node).unwrap().as_usize();
    let base = slot * n_dims;
    [
        GRANT_LANE_FREE,
        GRANT_LANE_IN_FLIGHT,
        GRANT_LANE_OCCUPIED,
        GRANT_LANE_CAPACITY,
    ]
    .map(|name| values[base + lane_column(&replay.registry, property_id, name).raw()] as u32)
}

fn install_action_band(
    session: &mut SimSession,
    source: SimThingId,
    property_id: simthing_core::SimPropertyId,
    effect_id: simthing_core::SimPropertyId,
) {
    let occupied = lane_column(&session.proto.registry, property_id, GRANT_LANE_OCCUPIED);
    let effect = session
        .proto
        .registry
        .column_range(effect_id)
        .col_for_role(
            &SubFieldRole::Amount,
            &session.proto.registry.property(effect_id).layout,
        )
        .unwrap();
    let source_slot = session.proto.allocator.slot_of(source).unwrap();
    session
        .proto
        .register_velocity_alert(VelocityAlertRegistration {
            sim_thing_id: source,
            property_id,
            sub_field: SubFieldRole::Named(GRANT_LANE_OCCUPIED.to_string()),
            threshold: 1.0,
            direction: Direction::Rising,
            cost_band: CostBandSemantic::observation(),
        });
    session
        .proto
        .initial_gpu_sync(&session.coord, &mut session.state)
        .expect("pre-open threshold uses the ordinary boundary compiler");
    let thresholds = vec![EmitOnThresholdRegistration {
        slot: source_slot,
        col: occupied,
        threshold: 1.0,
        direction: ThresholdDirection::Upward,
        event_kind: 0,
        buffer: EmitOnThresholdBuffer::Values,
    }];

    let program = EmlTreeId(EVENT_KIND);
    let mut nodes = TransformOp::set(3.0).to_eml_nodes();
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
            display_name: "grant-lane-crossing".into(),
        },
        nodes,
    )
    .unwrap();

    let spec = ActionBandSessionSpec {
        budget: ActionBandAdmissionBudgetSpec {
            axis_channel_count: 1,
            dependency_binding_count: 0,
            storage_rows: 1,
            eml_program_count: 1,
            emission_binding_count: 1,
        },
        templates: vec![ActionBandTemplateSpec {
            id: "grant-lane-band".into(),
            label: None,
            axis_channels: vec![ActionBandChannelBindingSpec {
                column: occupied.raw_u32(),
                kind: ActionBandChannelKind::Primitive,
            }],
            target: ActionBandTargetSpec::ScalarBound {
                channel: occupied.raw_u32(),
                bound: 1.0,
                direction: ScalarBoundDirection::AtLeast,
            },
            velocity: None,
            bands: vec![ActionBandBandSpec {
                threshold_registration_index: 0,
                eml_program: Some(program.0),
                emission_binding_indices: vec![0],
            }],
            subordinate_template_ids: Vec::new(),
            max_active_subordinates: 0,
            reserved_instance_rows: 1,
            requirement_semantics: Default::default(),
        }],
    };
    let mut door = ActionBandSessionBuildDoor::new();
    let frozen = door
        .admit_once_at_session_build(&spec, &session.proto.registry, &eml, &thresholds)
        .unwrap()
        .clone();
    let lanes = ActionBandNativeLaneAdmission::from_existing_surfaces(
        &session.proto.registry,
        &[effect],
        &[],
        &thresholds,
        &ThresholdRegistry::new(),
    );
    let resident = lanes
        .bind_resident_next(ActionBandEmissionBindingGpu::property_next(
            effect.raw_u32(),
            ActionBandPropertyWrite::Set,
        ))
        .unwrap();
    assert!(matches!(
        resident,
        CrossingConsequenceBinding::ResidentNextWrite(_)
    ));
    let commitments = compile_crossing_consequence_session(
        &frozen,
        &eml,
        &[resident],
        &[ActionBandActiveInstance::new(
            frozen.templates()[0].index(),
            SlotIndex::new(source_slot.raw()),
            [0.0; 4],
        )],
        &lanes,
    )
    .unwrap();

    session
        .install_action_band_commitments(commitments)
        .unwrap();
}

#[test]
fn six_real_doors_publish_conserved_sparse_lanes_and_cross_actionband_without_rebind() {
    let fixture = lane_fixture();
    let market = admitted_market();
    let mut session = SimSession::open(fixture.scenario).expect("GPU session opens");
    let replay_snapshot = session.proto.snapshot(0);
    install_action_band(
        &mut session,
        fixture.source,
        fixture.property_id,
        fixture.effect_id,
    );
    let admitted_shape = session.action_band_execution_generation();

    let accepted_clearance = clear_grant(fixture.fused, fixture.source, 6, GenerationStamp::new(0));
    let mut grant = session
        .record_cleared_market_grant(&market, fixture.fused, "lane-capacity", &accepted_clearance)
        .unwrap();

    // Generation N cannot publish its own fact. N+1 is the sole writer, and
    // the following hot cycle observes the ordinary upward crossing.
    assert_eq!(
        lane_values(&session, fixture.source, fixture.property_id),
        [20, 0, 0, 20]
    );
    let first = session.run(1).unwrap();
    assert_eq!(first.action_band_crossings, 0);
    assert_eq!(
        lane_values(&session, fixture.source, fixture.property_id),
        [20, 0, 0, 20]
    );
    let crossing = session.run(1).unwrap();
    assert_eq!(
        lane_values(&session, fixture.source, fixture.property_id),
        [14, 0, 6, 20]
    );
    assert_eq!(crossing.action_band_crossings, 1);
    assert_eq!(session.action_band_execution_generation(), Some(1));
    assert_eq!(admitted_shape, Some(0));
    let accepted_delta_entries = session.proto.delta_log().to_vec();
    let accepted_band_deltas: Vec<_> = accepted_delta_entries
        .iter()
        .filter_map(|entry| match entry {
            BoundaryDeltaEntry::BandCrossingDeltasApplied { deltas } if !deltas.is_empty() => {
                Some(deltas)
            }
            _ => None,
        })
        .collect();
    assert_eq!(accepted_band_deltas.len(), 1);

    let renew_generation = GenerationStamp::new(session.coord.day_index() as u32);
    let renewal = clear_grant(fixture.fused, fixture.source, 2, renew_generation);
    session
        .renew_market_grant(&market, &mut grant, &renewal)
        .unwrap();
    session.run(2).unwrap();
    assert_eq!(
        lane_values(&session, fixture.source, fixture.property_id),
        [12, 0, 8, 20]
    );

    let released = session.revoke_market_grant(&market, &mut grant, 1).unwrap();
    assert_eq!(released.quantity, 1);
    session.run(2).unwrap();
    assert_eq!(
        lane_values(&session, fixture.source, fixture.property_id),
        [13, 0, 7, 20]
    );

    let partition = session
        .partition_market_grant(&market, grant, &[(fixture.left, 3), (fixture.right, 4)])
        .unwrap();
    session.run(2).unwrap();
    assert_eq!(
        lane_values(&session, fixture.source, fixture.property_id),
        [20, 0, 0, 20]
    );
    assert_eq!(
        lane_values(&session, fixture.left, fixture.property_id),
        [17, 0, 3, 20]
    );
    assert_eq!(
        lane_values(&session, fixture.right, fixture.property_id),
        [16, 0, 4, 20]
    );

    let fused = session
        .transfer_market_grants(&market, partition, fixture.fused)
        .unwrap();
    session.run(2).unwrap();
    assert_eq!(
        lane_values(&session, fixture.left, fixture.property_id),
        [20, 0, 0, 20]
    );
    assert_eq!(
        lane_values(&session, fixture.right, fixture.property_id),
        [20, 0, 0, 20]
    );
    assert_eq!(
        lane_values(&session, fixture.fused, fixture.property_id),
        [13, 0, 7, 20]
    );

    let release = session
        .release_market_grant(&market, fused, GrantReleaseCause::ExplicitTermination)
        .unwrap();
    assert_eq!(release.quantity, 7);
    session.run(2).unwrap();
    assert_eq!(
        lane_values(&session, fixture.fused, fixture.property_id),
        [20, 0, 0, 20]
    );

    let kinds: Vec<_> = session
        .integration_schedule()
        .entries()
        .iter()
        .filter(|entry| entry.row_kind().is_grant_lifecycle())
        .map(|entry| entry.row_kind())
        .collect();
    assert_eq!(
        kinds,
        vec![
            IntegrationScheduleRowKind::GrantAccepted,
            IntegrationScheduleRowKind::GrantRenewed,
            IntegrationScheduleRowKind::GrantRevoked,
            IntegrationScheduleRowKind::GrantPartitioned,
            IntegrationScheduleRowKind::GrantTransferred,
            IntegrationScheduleRowKind::GrantReleased,
        ]
    );
    let facts: Vec<_> = session
        .integration_schedule()
        .entries()
        .iter()
        .filter_map(|entry| entry.grant_lifecycle_fact.as_ref())
        .collect();
    assert_eq!(facts.len(), 6);
    let fact_shapes: Vec<_> = facts
        .iter()
        .map(|fact| {
            (
                fact.kind,
                fact.before
                    .iter()
                    .map(|state| (state.grantee, state.quantity))
                    .collect::<Vec<_>>(),
                fact.after
                    .iter()
                    .map(|state| (state.grantee, state.quantity))
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    assert_eq!(
        fact_shapes,
        vec![
            (
                GrantLifecycleFactKind::Accepted,
                vec![(fixture.source, 0)],
                vec![(fixture.source, 6)]
            ),
            (
                GrantLifecycleFactKind::Renewed,
                vec![(fixture.source, 6)],
                vec![(fixture.source, 8)]
            ),
            (
                GrantLifecycleFactKind::Revoked,
                vec![(fixture.source, 8)],
                vec![(fixture.source, 7)]
            ),
            (
                GrantLifecycleFactKind::Partitioned,
                vec![(fixture.source, 7)],
                vec![(fixture.left, 3), (fixture.right, 4)]
            ),
            (
                GrantLifecycleFactKind::Transferred,
                vec![(fixture.left, 3), (fixture.right, 4)],
                vec![(fixture.fused, 7)]
            ),
            (
                GrantLifecycleFactKind::Released,
                vec![(fixture.fused, 7)],
                vec![(fixture.fused, 0)]
            ),
        ]
    );
    assert_eq!(facts[3].kind, GrantLifecycleFactKind::Partitioned);
    assert_eq!(
        facts[3].affected_nodes(),
        vec![fixture.source, fixture.left, fixture.right]
    );
    assert_eq!(facts[4].kind, GrantLifecycleFactKind::Transferred);
    assert_eq!(
        facts[4].affected_nodes(),
        vec![fixture.left, fixture.right, fixture.fused]
    );

    let delta_entries = session.proto.delta_log().to_vec();
    assert_eq!(
        delta_entries
            .iter()
            .filter(|entry| matches!(entry, BoundaryDeltaEntry::GrantLifecycleFact { .. }))
            .count(),
        6
    );

    // A shadow checkpoint alone is not a causal shortcut: it may restore
    // presentation bytes, but cannot mint or realize a lifecycle fact.
    let mut checkpoint_only = ReplayDriver::from_snapshot(replay_snapshot.clone()).unwrap();
    checkpoint_only.apply_frame(ReplayFrame {
        day: session.coord.day_index() as u32,
        entries: Vec::new(),
        shadow_values: Some(session.coord.shadow.clone()),
        spec_entries: Vec::new(),
        injection_entries: Vec::new(),
    });
    assert!(checkpoint_only.grant_lifecycle_facts.is_empty());

    let mut accepted_replay = ReplayDriver::from_snapshot(replay_snapshot.clone()).unwrap();
    accepted_replay.apply_frame(ReplayFrame {
        day: 2,
        entries: accepted_delta_entries,
        shadow_values: None,
        spec_entries: Vec::new(),
        injection_entries: Vec::new(),
    });
    assert_eq!(accepted_replay.grant_lifecycle_facts.len(), 1);
    assert_eq!(
        replay_lane_values(&accepted_replay, fixture.source, fixture.property_id),
        [14, 0, 6, 20]
    );
    assert_eq!(accepted_replay.last_band_crossing_deltas.len(), 1);

    let mut replay = ReplayDriver::from_snapshot(replay_snapshot).unwrap();
    replay.apply_frame(ReplayFrame {
        day: session.coord.day_index() as u32,
        entries: delta_entries,
        shadow_values: None,
        spec_entries: Vec::new(),
        injection_entries: Vec::new(),
    });
    assert_eq!(replay.grant_lifecycle_facts.len(), 6);
    assert!(matches!(
        replay.attempt_grant_lifecycle_reclear_forbidden(),
        Err(ReplayGrowthError::GrantLifecycleReclearForbidden)
    ));
}

#[test]
fn singular_schedule_and_lane_authority_reds() {
    // The actual public boundary door cannot forge a protected lane overlay.
    let live_fixture = lane_fixture();
    let mut live_session = SimSession::open(live_fixture.scenario).unwrap();
    let live_before = lane_values(&live_session, live_fixture.source, live_fixture.property_id);
    let live_overlay_count = live_session
        .proto
        .root
        .overlay_count(live_fixture.source)
        .unwrap();
    live_session
        .tx
        .submit_boundary(BoundaryRequest::AttachOverlay {
            target: live_fixture.source,
            overlay: grant_disbursement_capacity_overlay(
                live_fixture.source,
                live_fixture.property_id,
                7,
            ),
            source_generation: GenerationStamp::new(0),
        })
        .unwrap();
    let rejected = live_session.run(1).unwrap();
    assert_eq!(rejected.boundary_grant_lane_authority_rejections, 1);
    assert_eq!(
        live_session.proto.root.overlay_count(live_fixture.source),
        Some(live_overlay_count)
    );
    assert_eq!(
        lane_values(&live_session, live_fixture.source, live_fixture.property_id),
        live_before
    );

    // The replay door rejects the same causal bypass before changing either
    // semantic base state or overlay structure.
    let replay_fixture = lane_fixture();
    let replay_snapshot = SimSession::open(replay_fixture.scenario)
        .unwrap()
        .proto
        .snapshot(0);
    let mut forged_replay = ReplayDriver::from_snapshot(replay_snapshot).unwrap();
    let replay_before = replay_lane_values(
        &forged_replay,
        replay_fixture.source,
        replay_fixture.property_id,
    );
    let replay_overlay_count = forged_replay
        .root
        .overlay_count(replay_fixture.source)
        .unwrap();
    assert!(matches!(
        forged_replay.try_apply_frame(ReplayFrame {
            day: 1,
            entries: vec![BoundaryDeltaEntry::OverlayAttached {
                target: replay_fixture.source,
                overlay: grant_disbursement_capacity_overlay(
                    replay_fixture.source,
                    replay_fixture.property_id,
                    7,
                ),
            }],
            shadow_values: None,
            spec_entries: Vec::new(),
            injection_entries: Vec::new(),
        }),
        Err(ReplayError::GrantLaneCausalBypass)
    ));
    assert_eq!(forged_replay.day, 0);
    assert!(forged_replay.grant_lifecycle_facts.is_empty());
    assert_eq!(
        forged_replay.root.overlay_count(replay_fixture.source),
        Some(replay_overlay_count)
    );
    assert_eq!(
        replay_lane_values(
            &forged_replay,
            replay_fixture.source,
            replay_fixture.property_id
        ),
        replay_before
    );

    // Two real renewals with the same kind, generation, and provenance are
    // two ordered facts, not evidence of a second writer.
    let repeated_fixture = lane_fixture();
    let market = admitted_market();
    let mut repeated_session = SimSession::open(repeated_fixture.scenario).unwrap();
    let repeated_snapshot = repeated_session.proto.snapshot(0);
    let accepted = clear_grant(
        repeated_fixture.fused,
        repeated_fixture.source,
        2,
        GenerationStamp::new(0),
    );
    let mut grant = repeated_session
        .record_cleared_market_grant(&market, repeated_fixture.fused, "lane-capacity", &accepted)
        .unwrap();
    for _ in 0..2 {
        let renewal = clear_grant(
            repeated_fixture.fused,
            repeated_fixture.source,
            1,
            GenerationStamp::new(0),
        );
        repeated_session
            .renew_market_grant(&market, &mut grant, &renewal)
            .unwrap();
    }
    let repeated_facts: Vec<_> = repeated_session
        .integration_schedule()
        .entries()
        .iter()
        .filter_map(|entry| entry.grant_lifecycle_fact.as_ref())
        .collect();
    assert_eq!(repeated_facts.len(), 3);
    assert_eq!(
        repeated_facts
            .iter()
            .map(|fact| fact.kind)
            .collect::<Vec<_>>(),
        vec![
            GrantLifecycleFactKind::Accepted,
            GrantLifecycleFactKind::Renewed,
            GrantLifecycleFactKind::Renewed,
        ]
    );
    assert_eq!(
        repeated_facts
            .iter()
            .map(|fact| {
                (
                    fact.before[0].quantity,
                    fact.after[0].quantity,
                    fact.generation,
                    fact.provenance,
                )
            })
            .collect::<Vec<_>>(),
        vec![
            (0, 2, GenerationStamp::new(0), repeated_facts[0].provenance,),
            (2, 3, GenerationStamp::new(0), repeated_facts[0].provenance,),
            (3, 4, GenerationStamp::new(0), repeated_facts[0].provenance,),
        ]
    );
    repeated_session.run(2).unwrap();
    assert_eq!(
        lane_values(
            &repeated_session,
            repeated_fixture.source,
            repeated_fixture.property_id
        ),
        [16, 0, 4, 20]
    );
    let repeated_delta = repeated_session.proto.delta_log().to_vec();
    let delta_kinds: Vec<_> = repeated_delta
        .iter()
        .filter_map(|entry| match entry {
            BoundaryDeltaEntry::GrantLifecycleFact { fact } => Some(fact.kind),
            _ => None,
        })
        .collect();
    assert_eq!(
        delta_kinds,
        vec![
            GrantLifecycleFactKind::Accepted,
            GrantLifecycleFactKind::Renewed,
            GrantLifecycleFactKind::Renewed,
        ]
    );
    let mut repeated_replay = ReplayDriver::from_snapshot(repeated_snapshot).unwrap();
    repeated_replay.apply_frame(ReplayFrame {
        day: 2,
        entries: repeated_delta,
        shadow_values: None,
        spec_entries: Vec::new(),
        injection_entries: Vec::new(),
    });
    assert_eq!(repeated_replay.grant_lifecycle_facts.len(), 3);
    assert_eq!(
        replay_lane_values(
            &repeated_replay,
            repeated_fixture.source,
            repeated_fixture.property_id
        ),
        [16, 0, 4, 20]
    );

    // The generic numeric patcher remains a separate planted negative door.
    let fixture = lane_fixture();
    let session = SimSession::open(fixture.scenario).unwrap();
    let n_dims = session.state.n_dims as usize;
    let mut values = session.state.read_values();
    let before = values.clone();
    let patch = PatchTransform {
        target: fixture.source,
        delta: PropertyTransformDelta {
            property_id: fixture.property_id,
            sub_field_deltas: vec![(
                SubFieldRole::Named(GRANT_LANE_FREE.to_string()),
                TransformOp::set(0.0),
            )],
        },
    };
    let mut patcher = TransformPatcher::new(session.state.n_slots as usize);
    let mut stats = PatcherStats::default();
    patcher.apply_one(
        &patch,
        &session.proto.registry,
        &session.proto.allocator,
        n_dims,
        &mut values,
        &mut stats,
        ShadowFreshness::GpuSynced,
    );
    assert_eq!(stats.protected_grant_lane_write_forbidden, 1);
    assert_eq!(stats.applied_writes, 0);
    assert_eq!(values, before);
}

#[test]
fn lane_schema_is_optional_sparse_and_inert_without_a_fact() {
    let fixture = lane_fixture();
    assert_eq!(
        fixture
            .scenario
            .registry
            .id_of(GRANT_DISBURSEMENT_NAMESPACE, GRANT_DISBURSEMENT_PROPERTY),
        Some(fixture.property_id)
    );
    let inactive = fixture.scenario.root.children[4].id;
    assert!(fixture.scenario.root.children[4]
        .property(fixture.property_id)
        .is_none());
    let session = SimSession::open(fixture.scenario).unwrap();
    assert_eq!(session.proto.root.overlay_count(fixture.source), Some(1));
    assert_eq!(session.proto.root.overlay_count(inactive), Some(0));
    assert_eq!(
        lane_values(&session, fixture.source, fixture.property_id),
        [20, 0, 0, 20]
    );
    assert!(session
        .proto
        .root
        .snapshot_node(inactive)
        .unwrap()
        .property_ids
        .is_empty());
}
