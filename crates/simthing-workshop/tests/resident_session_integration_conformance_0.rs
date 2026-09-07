//! 15.8 exit referee: ordinary SimSession, persistent authored flow claimant.
use simthing_core::{
    bind_owner, AccumulatorRole, DimensionRegistry, IntegrationScheduleRowKind, OwnerRef,
    PersistenceDeformationProgram, SimThing, SimThingId, SimThingKind, SpecializationProfile,
    TransformOp,
};
use simthing_driver::resident_clearing_runtime::{
    install_default_resident_rf_property, ResidentClearingRuntimeError,
};
use simthing_driver::{GrowthEntitlementMarketBinding, Scenario, SimSession, SpecSessionState};
use simthing_spec::{
    admit_specialization_flow_market, scenario_metadata_u32_value, AuthoredClearingProgram,
    ConservedOfferingSpec, DrawEnvelopeTemplateSpec, OfferingPriceVectorSpec, OwnerChannelScopeKey,
    PersistenceDeformationBinding, PersistenceDeformationBindings, ResourceKey, ScopeId,
    SpecializationFlowMarketSpec, OWNER_FLOW_DEMAND_PROPERTY_ID, OWNER_SILO_CURRENT_PROPERTY_ID,
};
use std::collections::BTreeSet;

const RESOURCE: &str = "simthing::residency-row-capacity";

fn scope(root: SimThingId, resource: &str) -> OwnerChannelScopeKey {
    OwnerChannelScopeKey {
        owner_ref: OwnerRef::new("owner/15.8"),
        resource_key: ResourceKey::new(resource),
        scope_id: ScopeId::from_boundary(root),
    }
}

fn market(root: SimThingId, resource: &str, max_quantity: u32) -> GrowthEntitlementMarketBinding {
    market_with_minimum(root, resource, 1, max_quantity)
}

fn market_with_minimum(
    root: SimThingId,
    resource: &str,
    min_quantity: u32,
    max_quantity: u32,
) -> GrowthEntitlementMarketBinding {
    let triggers = BTreeSet::from(["current-boundary".into()]);
    let admitted = admit_market_fixture(resource, min_quantity, max_quantity, &triggers).unwrap();
    GrowthEntitlementMarketBinding::from_admitted_market(
        admitted,
        root,
        "offering",
        "draw",
        scope(root, resource),
        triggers,
        AuthoredClearingProgram::new(TransformOp::set(1.0)),
        1.0,
        100,
    )
}

fn admit_market_fixture(
    resource: &str,
    min_quantity: u32,
    max_quantity: u32,
    triggers: &BTreeSet<String>,
) -> Result<simthing_spec::AdmittedSpecializationFlowMarket, simthing_spec::FlowMarketAdmissionError>
{
    admit_specialization_flow_market(
        &[SpecializationProfile {
            id: "ordinary-flow".into(),
            description: "persistent ordinary owner-flow claimant".into(),
            requirements: Vec::new(),
        }],
        triggers,
        SpecializationFlowMarketSpec {
            specialization_profile_id: "ordinary-flow".into(),
            offerings: vec![ConservedOfferingSpec {
                id: "offering".into(),
                resource_key: ResourceKey::new(resource),
                price: OfferingPriceVectorSpec {
                    unit_cost: 1.0,
                    default_clearing_weight: 1.0,
                },
            }],
            draw_envelopes: vec![DrawEnvelopeTemplateSpec {
                id: "draw".into(),
                offering_refs: vec!["offering".into()],
                lifecycle_trigger_refs: vec!["current-boundary".into()],
                min_quantity,
                max_quantity,
            }],
        },
    )
}

fn scenario() -> (Scenario, SimThingId) {
    let mut registry = DimensionRegistry::new();
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    bind_owner(&mut root, &OwnerRef::new("owner/15.8"));
    root.add_property(
        OWNER_SILO_CURRENT_PROPERTY_ID,
        scenario_metadata_u32_value(4),
    );
    let mut claimant = SimThing::new(SimThingKind::Cohort, 0);
    claimant.add_property(
        OWNER_FLOW_DEMAND_PROPERTY_ID,
        scenario_metadata_u32_value(10),
    );
    let id = claimant.id;
    root.add_child(claimant);
    let property = install_default_resident_rf_property(&mut registry, &mut root);
    // A second real, independently named arena makes list position observable.
    let mut other = registry.property(property).clone();
    other.namespace = "other".into();
    other.name = "flow".into();
    for field in &mut other.layout.sub_fields {
        if let Some(spec) = &mut field.accumulator_spec {
            match &mut spec.role {
                AccumulatorRole::AllocatedFlow { arena }
                | AccumulatorRole::AllocatorWeight { arena } => *arena = "other-flow".into(),
                _ => {}
            }
        }
    }
    let other_id = registry.register(other);
    let value = registry.property(other_id).default_value();
    root.add_property(other_id, value.clone());
    root.children[0].add_property(other_id, value);
    (
        Scenario {
            name: "15.8 ordinary flow".into(),
            ticks_per_day: 1,
            max_days: 4,
            dt: 1.0,
            n_slots: 16,
            registry,
            root,
            shadow_seeds: Vec::new(),
            tick_patches: Vec::new(),
            install_targets: Default::default(),
        },
        id,
    )
}

fn clause_policy(base: f32) -> PersistenceDeformationProgram {
    use simthing_clausething::{
        compile_persistence_deformation_script_value, parse_raw_document, raw::RawValue,
    };
    let source = format!("script_value = {{ id = flow_persistence base = {base} }}");
    let doc = parse_raw_document(source.as_bytes()).unwrap();
    let RawValue::Block(root) = &doc.root else {
        panic!("ClauseScript block");
    };
    compile_persistence_deformation_script_value(&root.properties[0], 100)
        .unwrap()
        .1
}

fn install(session: &mut SimSession, claimant: SimThingId, policy: Option<f32>, reverse: bool) {
    let mut arenas = session.spec_state.arena_registry.clone();
    let mut other = arenas.arenas[0].clone();
    other.name = "other-flow".into();
    other.flow_property_id = session.proto.registry.id_of("other", "flow").unwrap();
    other.participant_range.0 = arenas.participants.len() as u32;
    arenas.arenas.push(other);
    let mut participants = arenas.participants.clone();
    for member in &mut participants {
        member.arena_idx = 1;
    }
    arenas.participants.extend(participants);
    if reverse {
        arenas.arenas.reverse();
        for member in &mut arenas.participants {
            member.arena_idx = 1 - member.arena_idx;
        }
    }
    let mut spec = SpecSessionState::new();
    spec.arena_registry = arenas;
    spec.persistence_deformations = PersistenceDeformationBindings::admit(policy.map(|base| {
        PersistenceDeformationBinding::new(
            scope(session.scenario.root.id, RESOURCE),
            claimant,
            clause_policy(base),
        )
    }))
    .unwrap();
    session.install_spec_state(spec).unwrap();
    session
        .install_growth_entitlement_market(market(session.scenario.root.id, RESOURCE, 100))
        .unwrap();
    let qualification = session
        .growth_entitlement_market()
        .resident_qualification()
        .unwrap();
    assert_eq!(
        qualification.flow_property_id(),
        session
            .proto
            .registry
            .id_of("simthing", "residency-row-capacity")
            .unwrap()
    );
    assert_eq!(qualification.arena_idx(), u32::from(reverse));
    println!(
        "15.8 semantic arena=residency-row-capacity property={:?} physical_index={}",
        qualification.flow_property_id(),
        qualification.arena_idx()
    );
}

fn facts(session: &SimSession, claimant: SimThingId) -> Vec<(u32, u32, u32)> {
    session
        .integration_schedule()
        .entries()
        .iter()
        .filter_map(|entry| entry.resident_clearing_fact)
        .filter(|fact| fact.source_simthing_id_raw == claimant.raw())
        .map(|fact| (fact.generation.get(), fact.granted, fact.unresolved))
        .collect()
}

fn cross_product(
    input: (Scenario, SimThingId),
    policy: Option<f32>,
    reverse: bool,
    record: bool,
) -> Vec<(u32, u32, u32)> {
    let (scenario, claimant) = input;
    let mut session = SimSession::open(scenario).unwrap();
    install(&mut session, claimant, policy, reverse);
    let identity = session.persisted_execution_identity();
    assert!(session.step_once().unwrap().boundary_reached);
    assert_eq!(facts(&session, claimant), [(1, 4, 6)]);
    // Authorship happens after N has finished; no future datum exists at N.
    assert!(session.proto.root.add_property_to_node(
        claimant,
        OWNER_FLOW_DEMAND_PROPERTY_ID,
        scenario_metadata_u32_value(2)
    ));
    if record {
        let dir = tempfile::tempdir().unwrap();
        session
            .record_to_path(&dir.path().join("ordinary-flow.ldjson"), 1)
            .unwrap();
    } else {
        assert!(session.step_once().unwrap().boundary_reached);
    }
    assert_eq!(session.persisted_execution_identity(), identity);
    assert_eq!(session.coord.day_index(), 2);
    let expected = if policy == Some(0.5) { 5 } else { 8 };
    let result = facts(&session, claimant);
    assert_eq!(result, [(1, 4, 6), (2, 4, expected - 4)]);
    assert!(!session
        .integration_schedule()
        .entries()
        .iter()
        .any(|entry| matches!(
            entry.row_kind(),
            IntegrationScheduleRowKind::ResidencyPlacementCommit
                | IntegrationScheduleRowKind::GrowthEntitlementRefusal
        )));
    let property = session
        .proto
        .registry
        .id_of("simthing", "residency-row-capacity")
        .unwrap();
    let columns = simthing_driver::resolve_node_columns_for_property(
        &session.proto.registry,
        property,
        "residency-row-capacity",
    )
    .unwrap();
    let slot = session.proto.allocator.slot_of(claimant).unwrap().raw() as usize;
    let flow = session.state.read_values()
        [slot * session.state.n_dims as usize + columns.allocated_flow_col.raw()];
    assert!(
        flow.is_finite() && flow > 0.0,
        "actual N+1 ordinary RF cell: {flow}"
    );
    println!("15.8 claimant={} scope={:?} N: authored10 -> G4/U6; N+1: authored2 -> effective{expected} -> G4/U{}; RF={flow}; reverse={reverse}; record={record}", claimant.raw(), scope(session.scenario.root.id, RESOURCE), expected - 4);
    result
}

#[test]
fn ordinary_session_identity_half_and_registry_permutation_cross_real_generations() {
    for policy in [None, Some(1.0), Some(0.5)] {
        let fixture = scenario();
        assert_eq!(
            cross_product(fixture.clone(), policy, false, false),
            cross_product(fixture, policy, true, false)
        );
    }
}

#[test]
fn recording_session_uses_the_same_late_mint_and_authored_policy() {
    cross_product(scenario(), Some(0.5), true, true);
}

#[test]
fn unmatched_market_refuses_symmetrically_in_both_arena_orders() {
    let fixture = scenario();
    for reverse in [false, true] {
        let (scenario, claimant) = fixture.clone();
        let mut session = SimSession::open(scenario).unwrap();
        install(&mut session, claimant, None, reverse);
        let error = session
            .install_growth_entitlement_market(market(
                session.scenario.root.id,
                "unbound::resource",
                100,
            ))
            .unwrap_err();
        assert!(
            matches!(
                error,
                simthing_driver::SessionError::ResidentClearing(
                    ResidentClearingRuntimeError::MarketCannotLower { .. }
                )
            ),
            "{error:?}"
        );
        println!("15.8 reverse={reverse}: {error}");
    }
}

#[test]
fn structural_batches_cannot_overwrite_continuing_flow_products() {
    let (scenario, claimant) = scenario();
    let mut session = SimSession::open(scenario).unwrap();
    install(&mut session, claimant, None, false);
    session.step_once().unwrap();
    session.proto.root.add_property_to_node(
        claimant,
        OWNER_FLOW_DEMAND_PROPERTY_ID,
        scenario_metadata_u32_value(2),
    );
    // Same continuous flow claimant; the new structural claimant has no demand datum.
    session
        .tx
        .submit_boundary(simthing_feeder::BoundaryRequest::AddChild {
            parent: session.scenario.root.id,
            child: SimThing::new(SimThingKind::Cohort, 2),
        })
        .unwrap();
    session.step_once().unwrap();
    session.step_once().unwrap();
    assert_eq!(facts(&session, claimant), [(1, 4, 6), (2, 4, 4), (3, 4, 2)]);
    println!("15.8 interleaved structural placement: continuing flow 10/8/6, U6/U4/U2 across three permits and topology rebind");
}

#[test]
fn both_postures_exercise_the_same_draw_envelope_at_the_current_boundary() {
    for posture in [
        simthing_core::ClearingExecutionPosture::ResidentRequired,
        simthing_core::ClearingExecutionPosture::CpuVendorizedOracle,
    ] {
        let (scenario, claimant) = scenario();
        let mut session = SimSession::open(scenario).unwrap();
        install(&mut session, claimant, None, false);
        session.set_clearing_execution_posture(posture).unwrap();
        session
            .install_growth_entitlement_market(market(session.scenario.root.id, RESOURCE, 9))
            .unwrap();
        let error = session.step_once().unwrap_err();
        assert!(error.to_string().contains("Draw"), "{error:?}");
        assert!(facts(&session, claimant).is_empty());
        println!("15.8 posture={posture:?} authored10/max9: {error}");
    }
}

#[derive(Clone, Copy, Debug)]
enum Departure {
    Owner,
    Property,
    Node,
}

#[derive(Clone, Copy, Debug)]
enum SessionLoop {
    Step,
    Run,
    Record,
}

fn advance(session: &mut SimSession, path: SessionLoop) {
    let before = session.coord.day_index();
    match path {
        SessionLoop::Step => assert!(session.step_once().unwrap().boundary_reached),
        SessionLoop::Run => assert_eq!(session.run(1).unwrap().boundaries_run, 1),
        SessionLoop::Record => {
            let dir = tempfile::tempdir().unwrap();
            session
                .record_to_path(&dir.path().join("neutral.ldjson"), 1)
                .unwrap();
        }
    }
    assert_eq!(session.coord.day_index(), before + 1);
}

// Change only authored membership on the existing runtime tree. The public
// admitted-tree swap is needed for removal; the session, coordinator, lease,
// resident state, allocator, continuation and schedule are never replaced.
fn depart(session: &mut SimSession, claimant: SimThingId, cause: Departure) {
    let mut tree: SimThing =
        serde_json::from_value(serde_json::to_value(&session.proto.root).unwrap()).unwrap();
    let index = tree
        .children
        .iter()
        .position(|node| node.id == claimant)
        .unwrap();
    match cause {
        Departure::Owner => {
            bind_owner(&mut tree.children[index], &OwnerRef::new("owner/departed"));
            assert_eq!(
                tree.children[index].property(OWNER_FLOW_DEMAND_PROPERTY_ID),
                Some(&scenario_metadata_u32_value(10))
            );
        }
        Departure::Property => {
            assert!(tree.children[index]
                .remove_property(&OWNER_FLOW_DEMAND_PROPERTY_ID)
                .is_some());
        }
        Departure::Node => {
            tree.children.remove(index);
        }
    }
    session
        .proto
        .root
        .swap(simthing_sim::SimRuntimeTree::admit(tree));
}

fn reenter(session: &mut SimSession, original: &SimThing, cause: Departure) {
    let claimant = original.id;
    match cause {
        Departure::Owner => {
            let mut node = original.clone();
            bind_owner(&mut node, &OwnerRef::new("owner/15.8"));
            assert!(session.proto.root.add_property_to_node(
                claimant,
                simthing_core::OWNER_CHANNEL_PROPERTY_ID,
                node.property(simthing_core::OWNER_CHANNEL_PROPERTY_ID)
                    .unwrap()
                    .clone(),
            ));
        }
        Departure::Property => {}
        Departure::Node => {
            assert!(session.proto.root.append_child(
                session.scenario.root.id,
                simthing_sim::SimRuntimeTree::admit(original.clone()),
            ));
        }
    }
    assert!(session.proto.root.add_property_to_node(
        claimant,
        OWNER_FLOW_DEMAND_PROPERTY_ID,
        scenario_metadata_u32_value(2),
    ));
}

fn terminations(session: &SimSession) -> Vec<simthing_core::NeutralStreamTerminationFact> {
    session
        .integration_schedule()
        .entries()
        .iter()
        .filter_map(|entry| entry.neutral_stream_termination_fact.clone())
        .collect()
}

fn assert_final_product(
    fact: &simthing_core::NeutralStreamTerminationFact,
    root: SimThingId,
    claimant: SimThingId,
    retired_at: u32,
    born_at: u32,
    g: u32,
    u: u32,
) {
    assert_eq!(fact.granter, root);
    assert_eq!(fact.owner_ref, OwnerRef::new("owner/15.8"));
    assert_eq!(fact.resource_key, RESOURCE);
    assert_eq!(fact.scope_id, ScopeId::from_boundary(root).as_str());
    assert_eq!(fact.termination_generation.get(), retired_at);
    assert_eq!(
        fact.final_products,
        [simthing_core::NeutralStreamFinalProduct {
            source_simthing_id: claimant,
            granted: g,
            unresolved: u,
            generation: simthing_core::GenerationStamp::new(born_at),
        }]
    );
}

fn assert_history_only_replay(
    session: &SimSession,
    fact: &simthing_core::NeutralStreamTerminationFact,
) {
    use simthing_core::{
        AncestorStandingPolicyView, AuthoredSeamStaleness, GenerationStamp, GenerationStamped,
        IntegrationSchedule, StandingViewDoubleBuffer,
    };
    use simthing_spec::{
        integrate_stamped_reduce_up, reduce_owner_channel_rf, replay_async_owner_channel_rf_seam,
        OwnerChannelRfOwnAggregate, ParentRfIntegrationState,
    };
    // Real nonzero RF state is the independent comparator. The new row must
    // affect neither its conserved buckets/fold nor standing replay.
    let product = reduce_owner_channel_rf(
        &session.scenario.root,
        &[OwnerChannelRfOwnAggregate {
            simthing_id: fact.final_products[0].source_simthing_id,
            resource_key: ResourceKey::new(RESOURCE),
            surplus: 3,
            deficit: 7,
        }],
        GenerationStamp::new(1),
    )
    .unwrap();
    let mut baseline = IntegrationSchedule::new();
    let mut state = ParentRfIntegrationState::default();
    integrate_stamped_reduce_up(GenerationStamp::new(4), &product, &mut state, &mut baseline)
        .unwrap();
    assert_eq!(
        (
            state.product_count,
            state.surplus_total,
            state.deficit_total
        ),
        (1, 3, 7)
    );
    let standing = GenerationStamped::stamp(
        GenerationStamp::new(4),
        AncestorStandingPolicyView::new(OwnerRef::new("owner/15.8"), Vec::new()),
    );
    let mut buffer = StandingViewDoubleBuffer::new();
    buffer.stage(standing.clone());
    buffer
        .publish_at_generation_barrier(
            GenerationStamp::new(4),
            AuthoredSeamStaleness::new(0),
            &mut baseline,
        )
        .unwrap();
    let mut with_termination = baseline.clone();
    with_termination.record_neutral_stream_termination(fact.clone());
    let before =
        replay_async_owner_channel_rf_seam(&baseline, &[product.clone()], &[standing.clone()])
            .unwrap();
    let after =
        replay_async_owner_channel_rf_seam(&with_termination, &[product], &[standing.clone()])
            .unwrap();
    assert_eq!(before.parent_state, state);
    assert_eq!(before.standing_reads, [standing]);
    assert_eq!(after, before);

    let encoded = serde_json::to_vec(session.integration_schedule()).unwrap();
    let decoded: IntegrationSchedule = serde_json::from_slice(&encoded).unwrap();
    assert_eq!(serde_json::to_vec(&decoded).unwrap(), encoded);
    assert_eq!(decoded.entries(), session.integration_schedule().entries());
}

#[test]
fn departing_last_flow_claimant_terminates_neutrally() {
    use simthing_core::ClearingExecutionPosture::{CpuVendorizedOracle, ResidentRequired};
    // Retain the original RED's never-started control in both postures.
    for posture in [ResidentRequired, CpuVendorizedOracle] {
        let (scenario, claimant) = scenario();
        let mut session = SimSession::open(scenario).unwrap();
        install(&mut session, claimant, None, false);
        session.set_clearing_execution_posture(posture).unwrap();
        depart(&mut session, claimant, Departure::Owner);
        advance(&mut session, SessionLoop::Step);
        assert!(terminations(&session).is_empty());
        assert!(session.integration_schedule().entries().is_empty());
    }

    for cause in [Departure::Owner, Departure::Property, Departure::Node] {
        for path in [SessionLoop::Step, SessionLoop::Run, SessionLoop::Record] {
            let fixture = scenario();
            let mut equivalent = None;
            for posture in [ResidentRequired, CpuVendorizedOracle] {
                let (scenario, claimant) = fixture.clone();
                let original = scenario.root.children[0].clone();
                let root = scenario.root.id;
                let mut session = SimSession::open(scenario).unwrap();
                install(
                    &mut session,
                    claimant,
                    None,
                    matches!(path, SessionLoop::Record),
                );
                session.set_clearing_execution_posture(posture).unwrap();
                let identity = session.persisted_execution_identity();
                advance(&mut session, path);
                let prior = session.integration_schedule().entries().to_vec();
                depart(&mut session, claimant, cause);
                advance(&mut session, path);
                let retired = terminations(&session);
                assert_eq!(retired.len(), 1);
                assert_final_product(&retired[0], root, claimant, 2, 1, 4, 6);
                let rows = session.integration_schedule().entries();
                assert_eq!(&rows[..prior.len()], prior.as_slice());
                assert_eq!(
                    rows.len(),
                    prior.len() + 1,
                    "no demand/product/cost/consequence at departure"
                );
                assert_eq!(
                    rows.last().unwrap().row_kind(),
                    IntegrationScheduleRowKind::NeutralStreamTermination
                );
                assert_history_only_replay(&session, &retired[0]);
                let clean = rows.to_vec();
                advance(&mut session, path);
                advance(&mut session, path);
                assert_eq!(
                    session.integration_schedule().entries(),
                    clean.as_slice(),
                    "two clean generations cannot resurrect U or add departed products"
                );

                reenter(&mut session, &original, cause);
                advance(&mut session, path);
                if posture.is_resident_required() {
                    assert_eq!(facts(&session, claimant), [(1, 4, 6), (5, 2, 0)]);
                }
                // Retire again to observe the already-born CPU result as well;
                // this is not a reconstruction of carry in a test helper.
                depart(&mut session, claimant, Departure::Property);
                advance(&mut session, path);
                let final_history = terminations(&session);
                assert_eq!(final_history.len(), 2);
                assert_final_product(&final_history[1], root, claimant, 6, 5, 2, 0);
                assert_eq!(session.persisted_execution_identity(), identity);
                // Canonical rows (including deterministic keys) match across
                // postures for identical persistent identities and scope.
                let canonical: Vec<_> = session
                    .integration_schedule()
                    .entries()
                    .iter()
                    .filter(|entry| {
                        entry.row_kind() == IntegrationScheduleRowKind::NeutralStreamTermination
                    })
                    .cloned()
                    .collect();
                if let Some(ref expected) = equivalent {
                    assert_eq!(&canonical, expected);
                } else {
                    equivalent = Some(canonical);
                }
                println!("15.11 cause={cause:?} loop={path:?} posture={posture:?}: N1 G4/U6; termination N2; N3/N4 no rows; same-ID reentry N5 G2/U0; second termination N6; deterministic history-only replay PASS");
            }
        }
    }
}

#[test]
fn draw_bounds_admit_zero_inclusively_and_refuse_reversed_bounds() {
    use simthing_spec::{
        DrawAuthorizationError, FlowMarketAdmissionError, RuntimeOwnerSiloDemandBucket,
    };
    let triggers = BTreeSet::from(["current-boundary".into()]);
    let (scenario, claimant) = scenario();
    for (min, max) in [(0, 0), (0, 100), (1, 100)] {
        let market = admit_market_fixture(RESOURCE, min, max, &triggers).unwrap();
        let draw = market.draw_envelope("draw").unwrap();
        assert_eq!((draw.min_quantity, draw.max_quantity), (min, max));
        for requested in [0, min, max, max + 1] {
            let key = scope(scenario.root.id, RESOURCE);
            let outcome = market.authorize_draw(
                "draw",
                "offering",
                RuntimeOwnerSiloDemandBucket {
                    owner_ref: key.owner_ref,
                    resource_key: key.resource_key,
                    scope_id: key.scope_id,
                    requested,
                    priority: 100,
                    source_simthing_id_raw: Some(claimant.raw()),
                },
                1.0,
                &triggers,
            );
            if (min..=max).contains(&requested) {
                assert_eq!(outcome.unwrap().demand.requested, requested);
            } else {
                assert_eq!(
                    outcome.unwrap_err(),
                    DrawAuthorizationError::QuantityOutsideEnvelope {
                        draw: "draw".into(),
                        requested,
                        min,
                        max,
                    }
                );
            }
        }
        println!("15.11 Draw [{min},{max}] admitted; existing inclusive authorization exact");
    }
    for (min, max) in [(1, 0), (101, 100)] {
        let error = admit_market_fixture(RESOURCE, min, max, &triggers).unwrap_err();
        assert_eq!(
            error,
            FlowMarketAdmissionError::InvalidDrawBounds {
                draw: "draw".into()
            }
        );
        assert!(error
            .to_string()
            .contains("0 <= min_quantity <= max_quantity"));
        println!("15.11 reversed Draw [{min},{max}] rejected: {error}");
    }
}

#[test]
fn authored_zero_continues_the_stream_or_refuses_at_draw() {
    use simthing_core::ClearingExecutionPosture::{CpuVendorizedOracle, ResidentRequired};
    // Refusal remains an ordinary Draw result in both postures, independently
    // of the admitted-zero execution and final-product provenance matrix.
    for posture in [ResidentRequired, CpuVendorizedOracle] {
        let (scenario, claimant) = scenario();
        let mut session = SimSession::open(scenario).unwrap();
        install(&mut session, claimant, None, false);
        session.set_clearing_execution_posture(posture).unwrap();
        advance(&mut session, SessionLoop::Step);
        let before = session.integration_schedule().entries().to_vec();
        assert!(session.proto.root.add_property_to_node(
            claimant,
            OWNER_FLOW_DEMAND_PROPERTY_ID,
            scenario_metadata_u32_value(0),
        ));
        let error = session.step_once().unwrap_err();
        assert!(error.to_string().contains("Draw"), "{error:?}");
        assert_eq!(session.integration_schedule().entries(), before.as_slice());
        assert!(terminations(&session).is_empty());
        println!("15.11 refused zero posture={posture:?}: {error}");
    }
    let mut failures = Vec::new();
    for zero_generations in [1, 2, 3] {
        let fixture = scenario();
        'posture: for posture in [ResidentRequired, CpuVendorizedOracle] {
            let (scenario, claimant) = fixture.clone();
            let root = scenario.root.id;
            let mut session = SimSession::open(scenario).unwrap();
            install(&mut session, claimant, None, false);
            session.set_clearing_execution_posture(posture).unwrap();
            session
                .install_growth_entitlement_market(market_with_minimum(root, RESOURCE, 0, 100))
                .unwrap();
            advance(&mut session, SessionLoop::Step);
            assert!(session.proto.root.add_property_to_node(
                claimant,
                OWNER_FLOW_DEMAND_PROPERTY_ID,
                scenario_metadata_u32_value(0),
            ));
            for _ in 0..zero_generations {
                let generation = session.coord.day_index() + 1;
                match session.step_once() {
                    Ok(outcome) => {
                        assert!(outcome.boundary_reached);
                        assert_eq!(session.coord.day_index(), generation);
                    }
                    Err(error) => {
                        assert!(terminations(&session).is_empty());
                        let failure = format!("{posture:?} N{generation} authored0: {error:?}; already-born resident facts={:?}; no termination", facts(&session, claimant));
                        println!("15.11 zero runtime FAILURE: {failure}");
                        failures.push(failure);
                        continue 'posture;
                    }
                }
            }
            assert!(
                terminations(&session).is_empty(),
                "zero is membership, not departure"
            );
            if posture.is_resident_required() {
                let expected = [(1, 4, 6), (2, 4, 2), (3, 2, 0), (4, 0, 0)];
                assert_eq!(
                    facts(&session, claimant),
                    expected[..zero_generations as usize + 1]
                );
            }
            depart(&mut session, claimant, Departure::Property);
            advance(&mut session, SessionLoop::Step);
            let retired = terminations(&session);
            assert_eq!(retired.len(), 1);
            let (g, u) = match zero_generations {
                1 => (4, 2),
                2 => (2, 0),
                3 => (0, 0),
                _ => unreachable!(),
            };
            println!("15.11 zero provenance posture={posture:?}: zero generations={zero_generations}; expected N{} G{g}/U{u}; actual termination={:?}", zero_generations + 1, retired[0]);
            let expected = simthing_core::NeutralStreamFinalProduct {
                source_simthing_id: claimant,
                granted: g,
                unresolved: u,
                generation: simthing_core::GenerationStamp::new(zero_generations + 1),
            };
            if retired[0].final_products != [expected.clone()] {
                failures.push(format!("{posture:?} termination N{} lost final product: expected {expected:?}, actual {:?}", zero_generations + 2, retired[0].final_products));
                continue;
            }
            assert_final_product(
                &retired[0],
                root,
                claimant,
                zero_generations + 2,
                zero_generations + 1,
                g,
                u,
            );
            println!("15.11 admitted zero posture={posture:?}: zero generations={zero_generations}, final G{g}/U{u}, no termination until property removal");
        }
    }
    assert!(
        failures.is_empty(),
        "admitted-zero lifecycle failures: {failures:#?}"
    );
}

#[test]
fn initial_zero_members_produce_same_canonical_result_in_both_postures() {
    use simthing_core::ClearingExecutionPosture::{CpuVendorizedOracle, ResidentRequired};
    let mut failures = Vec::new();
    for max in [0, 100] {
        let fixture = scenario();
        for posture in [ResidentRequired, CpuVendorizedOracle] {
            let (mut scenario, claimant) = fixture.clone();
            let root = scenario.root.id;
            scenario.root.children[0].add_property(
                OWNER_FLOW_DEMAND_PROPERTY_ID,
                scenario_metadata_u32_value(0),
            );
            let mut session = SimSession::open(scenario).unwrap();
            install(&mut session, claimant, None, false);
            session.set_clearing_execution_posture(posture).unwrap();
            session
                .install_growth_entitlement_market(market_with_minimum(root, RESOURCE, 0, max))
                .unwrap();
            let outcome = session.step_once();
            println!("15.11 initial zero [{},{max}] posture={posture:?}: {outcome:?}; resident facts={:?}", 0, facts(&session, claimant));
            assert!(terminations(&session).is_empty(), "zero is not departure");
            if !outcome.as_ref().is_ok_and(|step| step.boundary_reached) {
                failures.push(format!("initial zero [0,{max}] {posture:?}: {outcome:?}"));
                continue;
            }
            if posture.is_resident_required() && facts(&session, claimant) != [(1, 0, 0)] {
                failures.push(format!(
                    "initial zero [0,{max}] resident lost canonical G0/U0@1: {:?}",
                    facts(&session, claimant)
                ));
            }
            // No positive lifecycle relation is allowed for the zero result.
            assert!(session
                .integration_schedule()
                .entries()
                .iter()
                .all(|entry| entry.grant_lifecycle_fact.is_none()));
            depart(&mut session, claimant, Departure::Property);
            advance(&mut session, SessionLoop::Step);
            let retired = terminations(&session);
            assert_eq!(retired.len(), 1);
            let expected = simthing_core::NeutralStreamFinalProduct {
                source_simthing_id: claimant,
                granted: 0,
                unresolved: 0,
                generation: simthing_core::GenerationStamp::new(1),
            };
            if retired[0].final_products != [expected] {
                failures.push(format!(
                    "initial zero [0,{max}] {posture:?} departure lost provenance: {:?}",
                    retired[0]
                ));
            } else {
                assert_final_product(&retired[0], root, claimant, 2, 1, 0, 0);
            }
        }
    }
    assert!(
        failures.is_empty(),
        "initial zero member failures: {failures:#?}"
    );
}

#[test]
fn partial_departure_stays_fail_closed_and_empty_set_cannot_launder_fault() {
    use simthing_core::ClearingExecutionPosture::{CpuVendorizedOracle, ResidentRequired};
    for posture in [ResidentRequired, CpuVendorizedOracle] {
        // 15.12 supersedes only the lawful partial-membership refusal. The
        // established identity and post-touch empty-set fault fence remain.
        for supply in [4, 40] {
            let (mut scenario, claimant) = scenario();
            scenario.root.add_property(
                OWNER_SILO_CURRENT_PROPERTY_ID,
                scenario_metadata_u32_value(supply),
            );
            let mut other = SimThing::new(SimThingKind::Cohort, 0);
            other.properties = scenario.root.children[0].properties.clone();
            let survivor = other.id;
            scenario.root.add_child(other);
            let mut session = SimSession::open(scenario).unwrap();
            install(&mut session, claimant, None, false);
            session.set_clearing_execution_posture(posture).unwrap();
            advance(&mut session, SessionLoop::Step);
            depart(&mut session, claimant, Departure::Property);
            advance(&mut session, SessionLoop::Step);
            assert_eq!(terminations(&session).len(), 1);
            assert_eq!(terminations(&session)[0].final_products.len(), 1);
            assert_eq!(
                terminations(&session)[0].final_products[0].source_simthing_id,
                claimant
            );
            // The existing positive-minimum Draw still refuses an authored zero
            // on the surviving stream; this touched refusal cannot be laundered.
            assert!(session.proto.root.add_property_to_node(
                survivor,
                OWNER_FLOW_DEMAND_PROPERTY_ID,
                scenario_metadata_u32_value(0)
            ));
            let before = session.integration_schedule().entries().to_vec();
            let error = session.step_once().unwrap_err();
            assert!(error.to_string().contains("Draw"), "{error:?}");
            assert_eq!(session.integration_schedule().entries(), before.as_slice());
            assert_eq!(terminations(&session).len(), 1);
            // The refused Draw touched this generation. Even total departure
            // afterwards cannot reset it, publish a termination, or run a hot tick.
            depart(&mut session, survivor, Departure::Property);
            let day = session.coord.day_index();
            let cells = session.state.read_values();
            let identity = session.persisted_execution_identity();
            let refused = session.step_once().unwrap_err();
            let expected = simthing_core::TreeExecutionContextError::GenerationFaulted {
                generation: simthing_core::GenerationStamp::new(3),
            }
            .to_string();
            assert!(
                matches!(&refused,
                    simthing_driver::SessionError::ExecutionIdentity(message) if message == &expected
                ),
                "{refused:?}"
            );
            assert_eq!(session.coord.day_index(), day);
            assert_eq!(session.persisted_execution_identity(), identity);
            assert_eq!(session.state.read_values(), cells);
            assert_eq!(session.integration_schedule().entries(), before.as_slice());
            println!("15.12 partial admitted, invalid Draw refused posture={posture:?} supply={supply}: {error}; then empty set: {refused}; no duplicate termination/no retry effects");
        }
    }
}
