//! 15.13: one bound market uses resident precedence and live allocation in both postures.
use simthing_core::{
    bind_owner, ClearingExecutionPosture, DimensionRegistry, OwnerRef, SimThing, SimThingId,
    SimThingKind, SpecializationProfile, SubFieldRole, TransformOp,
};
use simthing_driver::resident_clearing_runtime::install_default_resident_rf_property;
use simthing_driver::{GrowthEntitlementMarketBinding, Scenario, SimSession};
use simthing_spec::{
    admit_specialization_flow_market, scenario_metadata_u32_value, AuthoredClearingProgram,
    ConservedOfferingSpec, DrawEnvelopeTemplateSpec, OfferingPriceVectorSpec, OwnerChannelScopeKey,
    ResourceKey, ScopeId, SpecializationFlowMarketSpec, OWNER_FLOW_DEMAND_PROPERTY_ID,
    OWNER_FLOW_PRIORITY_PROPERTY_ID, OWNER_SILO_CURRENT_PROPERTY_ID,
};
use std::collections::BTreeSet;

const RESOURCE: &str = "simthing::residency-row-capacity";

fn scope(root: SimThingId) -> OwnerChannelScopeKey {
    OwnerChannelScopeKey {
        owner_ref: OwnerRef::new("owner/15.13"),
        resource_key: ResourceKey::new(RESOURCE),
        scope_id: ScopeId::from_boundary(root),
    }
}

fn scenario(requests: [u32; 2], priorities: [u32; 2], supply: u32) -> (Scenario, [SimThingId; 2]) {
    let mut registry = DimensionRegistry::new();
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    bind_owner(&mut root, &OwnerRef::new("owner/15.13"));
    root.add_property(
        OWNER_SILO_CURRENT_PROPERTY_ID,
        scenario_metadata_u32_value(supply),
    );
    let ids = std::array::from_fn(|index| {
        let mut child = SimThing::new(SimThingKind::Cohort, 0);
        child.add_property(
            OWNER_FLOW_DEMAND_PROPERTY_ID,
            scenario_metadata_u32_value(requests[index]),
        );
        child.add_property(
            OWNER_FLOW_PRIORITY_PROPERTY_ID,
            scenario_metadata_u32_value(priorities[index]),
        );
        let id = child.id;
        root.add_child(child);
        id
    });
    let property = install_default_resident_rf_property(&mut registry, &mut root);
    let mut root_flow = registry.property(property).default_value();
    root_flow.set_role(
        &SubFieldRole::Named("intrinsic-flow".into()),
        &registry.property(property).layout,
        2.0,
    );
    root.add_property(property, root_flow);
    (
        Scenario {
            name: "15.13 bound market".into(),
            ticks_per_day: 1,
            max_days: 6,
            dt: 1.0,
            n_slots: 16,
            registry,
            root,
            shadow_seeds: Vec::new(),
            tick_patches: Vec::new(),
            install_targets: Default::default(),
        },
        ids,
    )
}

fn open(scenario: Scenario, posture: ClearingExecutionPosture, program: TransformOp) -> SimSession {
    let mut session = SimSession::open(scenario).unwrap();
    let triggers = BTreeSet::from(["current-boundary".to_owned()]);
    let market = admit_specialization_flow_market(
        &[SpecializationProfile {
            id: "ordinary-flow".into(),
            description: "15.13 bound policy".into(),
            requirements: Vec::new(),
        }],
        &triggers,
        SpecializationFlowMarketSpec {
            specialization_profile_id: "ordinary-flow".into(),
            offerings: vec![ConservedOfferingSpec {
                id: "offering".into(),
                resource_key: ResourceKey::new(RESOURCE),
                price: OfferingPriceVectorSpec {
                    unit_cost: 1.0,
                    default_clearing_weight: 1.0,
                },
            }],
            draw_envelopes: vec![DrawEnvelopeTemplateSpec {
                id: "draw".into(),
                offering_refs: vec!["offering".into()],
                lifecycle_trigger_refs: vec!["current-boundary".into()],
                min_quantity: 0,
                max_quantity: 1000,
            }],
        },
    )
    .unwrap();
    session
        .install_growth_entitlement_market(GrowthEntitlementMarketBinding::from_admitted_market(
            market,
            session.scenario.root.id,
            "offering",
            "draw",
            scope(session.scenario.root.id),
            triggers,
            AuthoredClearingProgram::new(program),
            1.0,
            100,
        ))
        .unwrap();
    assert_eq!(
        session
            .growth_entitlement_market()
            .resident_qualification()
            .unwrap()
            .exact_basis_identity(),
        simthing_gpu::ResidentExactBasisIdentity::LiveAllocatedFlow
    );
    session.set_clearing_execution_posture(posture).unwrap();
    session
}

#[derive(Clone, Copy, Debug)]
enum Loop {
    Step,
    Run,
    Record,
}

fn advance(session: &mut SimSession, path: Loop) -> Result<(), simthing_driver::SessionError> {
    match path {
        Loop::Step => session
            .step_once()
            .map(|outcome| assert!(outcome.boundary_reached)),
        Loop::Run => session
            .run(1)
            .map(|outcome| assert_eq!(outcome.boundaries_run, 1)),
        Loop::Record => {
            let directory = tempfile::tempdir().unwrap();
            session
                .record_to_path(&directory.path().join("policy.ldjson"), 1)
                .map(|outcome| assert_eq!(outcome.boundaries_run, 1))
        }
    }
}

fn products(session: &SimSession, ids: [SimThingId; 2], generation: u32) -> Vec<(u32, u32)> {
    ids.into_iter()
        .map(|id| {
            let facts: Vec<_> = session
                .integration_schedule()
                .entries()
                .iter()
                .filter_map(|entry| entry.resident_clearing_fact)
                .filter(|fact| {
                    fact.source_simthing_id_raw == id.raw() && fact.generation.get() == generation
                })
                .collect();
            assert_eq!(
                facts.len(),
                1,
                "one canonical product per claimant/generation"
            );
            (facts[0].granted, facts[0].unresolved)
        })
        .collect()
}

fn granted(session: &SimSession, ids: [SimThingId; 2], generation: u32) -> Vec<u32> {
    if session.clearing_execution_posture().is_resident_required() {
        return products(session, ids, generation)
            .into_iter()
            .map(|(g, _)| g)
            .collect();
    }
    ids.into_iter()
        .map(|id| {
            let quantities: Vec<_> = session
                .integration_schedule()
                .entries()
                .iter()
                .filter_map(|entry| entry.grant_lifecycle_fact.as_ref())
                .filter(|fact| {
                    fact.kind == simthing_core::GrantLifecycleFactKind::Accepted
                        && fact.generation.get() == generation
                })
                .flat_map(|fact| &fact.after)
                .filter(|state| state.grantee == id && state.granter == session.scenario.root.id)
                .map(|state| state.quantity)
                .collect();
            assert!(
                quantities.len() <= 1,
                "one canonical positive grant per claimant/generation"
            );
            quantities.first().copied().unwrap_or(0)
        })
        .collect()
}

// The CPU posture retains U privately until this existing observation door.
// Ending an independently replayed prefix exposes its actual final products;
// no observation is injected into any continuation or subsequent session.
fn finish_stream(
    session: &mut SimSession,
    ids: [SimThingId; 2],
    generation: u32,
    path: Loop,
) -> Vec<(u32, u32)> {
    let mut tree: SimThing =
        serde_json::from_value(serde_json::to_value(&session.proto.root).unwrap()).unwrap();
    for id in ids {
        let child = tree
            .children
            .iter_mut()
            .find(|child| child.id == id)
            .unwrap();
        assert!(child
            .remove_property(&OWNER_FLOW_DEMAND_PROPERTY_ID)
            .is_some());
    }
    session
        .proto
        .root
        .swap(simthing_sim::SimRuntimeTree::admit(tree));
    advance(session, path).unwrap();
    let facts: Vec<_> = session
        .integration_schedule()
        .entries()
        .iter()
        .filter_map(|entry| entry.neutral_stream_termination_fact.as_ref())
        .collect();
    assert_eq!(facts.len(), 1);
    assert_eq!(facts[0].termination_generation.get(), generation + 1);
    assert_eq!(facts[0].granter, session.scenario.root.id);
    assert_eq!(facts[0].final_products.len(), ids.len());
    ids.into_iter()
        .map(|id| {
            let matching: Vec<_> = facts[0]
                .final_products
                .iter()
                .filter(|product| product.source_simthing_id == id)
                .collect();
            assert_eq!(matching.len(), 1);
            assert_eq!(matching[0].generation.get(), generation);
            (matching[0].granted, matching[0].unresolved)
        })
        .collect()
}

fn assert_live_basis(session: &SimSession, ids: [SimThingId; 2]) {
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
    let values = session.state.read_values();
    let bases = ids.map(|id| {
        let slot = session.proto.allocator.slot_of(id).unwrap().raw() as usize;
        values[slot * session.state.n_dims as usize + columns.allocated_flow_col.raw()]
    });
    assert_eq!(bases, [1.0, 1.0], "actual Current allocated-flow cells");
}

fn policy_matrix(label: &str, program: TransformOp) {
    let (fixture, ids) = scenario([1, 1], [0, 1], 1);
    let mut refusals = Vec::new();
    for path in [Loop::Step, Loop::Run, Loop::Record] {
        for posture in [
            ClearingExecutionPosture::ResidentRequired,
            ClearingExecutionPosture::CpuVendorizedOracle,
        ] {
            let mut session = open(fixture.clone(), posture, program.clone());
            let result = advance(&mut session, path);
            assert_live_basis(&session, ids);
            match result {
                Ok(()) => {
                    assert_eq!(granted(&session, ids, 1), [1, 0]);
                    let actual = finish_stream(&mut session, ids, 1, path);
                    println!("15.13 {label}/{posture:?}/{path:?}: ACCEPT G/U={actual:?}; priorities=[0,1]; actual bases=[1,1]");
                    assert_eq!(actual, [(1, 0), (0, 1)]);
                }
                Err(error) => {
                    println!("15.13 {label}/{posture:?}/{path:?}: REFUSE {error:?}");
                    refusals.push(format!("{posture:?}/{path:?}: {error:?}"));
                }
            }
        }
    }
    assert!(
        refusals.is_empty(),
        "one admitted market must execute in both postures: {refusals:?}"
    );
}

#[test]
fn constant_score_preserves_resident_precedence_in_both_postures() {
    let program = TransformOp::set(1.0);
    let scores = [0, 1].map(|priority| program.apply_with_params(1.0, priority as f32));
    assert_eq!(scores, [1.0, 1.0]);
    println!("15.13 F1: authored scores={scores:?}; canonical priorities=[0,1]");
    policy_matrix("F1 constant", program);
}

#[test]
fn priority_score_preserves_resident_precedence_in_both_postures() {
    let program = TransformOp::admit_eml(
        vec![simthing_core::eml_nodes::EmlNode {
            opcode: simthing_core::eml_nodes::opcode::PARAM,
            flags: 0,
            a: 1,
            b: 0,
            c: 0,
            d: 0,
        }],
        simthing_core::EmlPerProgramCap::DEFAULT,
    )
    .unwrap();
    let scores = [0, 1].map(|priority| program.apply_with_params(1.0, priority as f32));
    assert_eq!(scores, [0.0, 1.0]);
    println!("15.13 F2: authored scores={scores:?}; canonical priorities=[0,1]");
    policy_matrix("F2 priority-sensitive", program);
}

#[test]
fn saturated_cap_products_recur_once_through_the_ordinary_session() {
    let (fixture, ids) = scenario([1, 100], [0, 0], 51);
    let expected = [
        [(1, 0), (50, 50)],
        [(2, 0), (49, 4)],
        [(0, 0), (4, 0)],
        [(0, 0), (0, 0)],
    ];
    let authored = [[1, 100], [2, 3], [0, 0], [0, 0]];
    for reverse in [false, true] {
        for path in [Loop::Step, Loop::Run, Loop::Record] {
            for posture in [
                ClearingExecutionPosture::ResidentRequired,
                ClearingExecutionPosture::CpuVendorizedOracle,
            ] {
                let mut observed = Vec::<Vec<(u32, u32)>>::new();
                // Replay complete prefixes to expose each final U through the
                // ordinary termination fact. Each prefix keeps one session,
                // lease and continuation across every temporal generation.
                for endpoint in 1..=expected.len() {
                    let mut permuted = fixture.clone();
                    if reverse {
                        permuted.root.children.reverse();
                    }
                    let mut session = open(permuted, posture, TransformOp::set(1.0));
                    let slots = ids.map(|id| session.proto.allocator.slot_of(id).unwrap().raw());
                    assert_eq!(slots[0] < slots[1], !reverse);
                    let identity = session.persisted_execution_identity();
                    for index in 0..endpoint {
                        let generation = index as u32 + 1;
                        if index != 0 {
                            for (id, demand) in ids.into_iter().zip(authored[index]) {
                                assert!(session.proto.root.add_property_to_node(
                                    id,
                                    OWNER_FLOW_DEMAND_PROPERTY_ID,
                                    scenario_metadata_u32_value(demand),
                                ));
                            }
                        }
                        advance(&mut session, path).unwrap();
                        assert_live_basis(&session, ids);
                        assert_eq!(session.coord.day_index(), u64::from(generation));
                        assert_eq!(session.persisted_execution_identity(), identity);
                        assert_eq!(
                            granted(&session, ids, generation),
                            expected[index].map(|(g, _)| g)
                        );
                        if posture.is_resident_required() {
                            assert_eq!(products(&session, ids, generation), expected[index]);
                        }
                    }
                    let before = session.integration_schedule().entries().to_vec();
                    let actual = finish_stream(&mut session, ids, endpoint as u32, path);
                    assert_eq!(session.persisted_execution_identity(), identity);
                    assert_eq!(
                        &session.integration_schedule().entries()[..before.len()],
                        before
                    );
                    assert_eq!(actual, expected[endpoint - 1]);
                    let prior_u = if endpoint == 1 {
                        [0, 0]
                    } else {
                        [observed[endpoint - 2][0].1, observed[endpoint - 2][1].1]
                    };
                    let effective =
                        std::array::from_fn::<_, 2, _>(|i| authored[endpoint - 1][i] + prior_u[i]);
                    assert_eq!(
                        actual.iter().map(|(g, u)| g + u).collect::<Vec<_>>(),
                        effective,
                        "canonical G+U equals authored demand plus one identity carry"
                    );
                    assert!(actual.iter().map(|(g, _)| *g).sum::<u32>() <= 51);
                    for index in 0..endpoint {
                        assert_eq!(
                            granted(&session, ids, index as u32 + 1),
                            expected[index].map(|(g, _)| g)
                        );
                    }
                    assert!(!session
                        .integration_schedule()
                        .entries()
                        .iter()
                        .any(|entry| {
                            matches!(entry.row_kind(),
                            simthing_core::IntegrationScheduleRowKind::GrowthEntitlementRefusal
                            | simthing_core::IntegrationScheduleRowKind::ResidencyPlacementCommit)
                        }));
                    println!("15.13 F3 {posture:?}/{path:?}/reverse={reverse} slots={slots:?} N={endpoint}: authored={:?} observed_prior_U={prior_u:?} effective={effective:?} canonical G/U={actual:?}; actual bases=[1,1]; uninterrupted prefix retired at N+1", authored[endpoint - 1]);
                    observed.push(actual);
                }
            }
        }
    }
}
