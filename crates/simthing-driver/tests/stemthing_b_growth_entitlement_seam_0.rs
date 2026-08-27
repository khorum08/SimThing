use std::collections::BTreeMap;

use simthing_core::{
    DimensionRegistry, GenerationStamp, IntegrationSchedule, IntegrationScheduleRowKind,
    SimProperty, SimThing, SimThingKind,
};
use simthing_driver::{GrowthEntitlementMarketBinding, Scenario, SimSession};
use simthing_feeder::BoundaryRequest;
use simthing_gpu::{
    GrowthResidencyCommit, ProvisionalResidencyEntitlement, ResidencyExtent, SlotAllocator,
};
use simthing_sim::{
    BoundaryDeltaEntry, GrowthEntitlementDecision, OrdinaryGrowthCandidate, OrdinaryGrowthOrigin,
    OrdinaryGrowthRefusalReason, RecordedGrowthResidencyFact, ReplayDriver, ReplayFrame,
    ReplayGrowthError,
};

fn add_child_scenario(n_slots: u32) -> (Scenario, simthing_core::SimThingId) {
    let mut registry = DimensionRegistry::new();
    registry.register(SimProperty::simple("growth", "witness", 0));
    let parent = SimThing::new(SimThingKind::Location, 0);
    let parent_id = parent.id;
    let mut root = SimThing::new(SimThingKind::World, 0);
    root.add_child(parent);
    (
        Scenario {
            name: "growth-entitlement-add-child".into(),
            ticks_per_day: 1,
            max_days: 4,
            dt: 1.0,
            n_slots,
            registry,
            root,
            shadow_seeds: Vec::new(),
            tick_patches: Vec::new(),
            install_targets: Default::default(),
        },
        parent_id,
    )
}

#[test]
fn implicit_root_market_add_child_refusal_and_replay_use_one_authority_chain() {
    let (scenario, parent_id) = add_child_scenario(5);
    let mut session = SimSession::open(scenario).expect("GPU session opens");
    assert!(session
        .growth_entitlement_market()
        .is_implicit_root_standing());
    let replay_snapshot = session.proto.snapshot(0);
    let registry_columns = session.proto.registry.total_columns;

    let child = SimThing::new(SimThingKind::Cohort, 1);
    let child_id = child.id;
    session
        .tx
        .submit_boundary(BoundaryRequest::AddChild {
            parent: parent_id,
            child,
        })
        .unwrap();
    let step = session.step_once().expect("accepted AddChild boundary");
    assert!(step.boundary_reached);
    assert!(session.proto.root.contains_id(child_id));
    assert!(session.proto.allocator.slot_of(child_id).is_some());
    let accepted_placement = session
        .proto
        .allocator
        .committed_residency_placement(session.scenario.root.id, child_id)
        .expect("11.2b placement committed before attach");
    assert_eq!(accepted_placement.quantity(), 1);
    assert_eq!(session.proto.registry.total_columns, registry_columns);
    assert!(session.coord.n_slots() >= accepted_placement.extent().end_exclusive());
    assert!(session.state.n_slots >= accepted_placement.extent().end_exclusive());
    assert_eq!(
        session
            .integration_schedule()
            .entries()
            .iter()
            .filter(|row| row.row_kind() == IntegrationScheduleRowKind::ResidencyPlacementCommit)
            .count(),
        1
    );

    let accepted_entries = session.proto.take_delta_log();
    let accepted_fact = accepted_entries.iter().find_map(|entry| match entry {
        BoundaryDeltaEntry::SimThingAdded { residency, .. } => Some(*residency),
        _ => None,
    });
    let accepted_fact = accepted_fact.expect("accepted grant/placement fact recorded");
    assert_eq!(accepted_fact.entitlement().grantee(), child_id);
    assert_ne!(accepted_fact.entitlement().market_grant_key(), 0);

    let mut replay = ReplayDriver::from_snapshot(replay_snapshot).expect("replay snapshot install");
    replay.apply_frame(ReplayFrame {
        day: 0,
        entries: accepted_entries,
        ..Default::default()
    });
    assert!(replay.root.contains_id(child_id));
    assert_eq!(
        replay.allocator.slot_of(child_id),
        session.proto.allocator.slot_of(child_id)
    );
    assert_eq!(
        replay.attempt_growth_reclear_forbidden(),
        Err(ReplayGrowthError::ReplayReclearForbidden)
    );

    // Only one row remains. This complete four-row candidate receives a real
    // partial grant, stays U, and must neither attach nor mint any row.
    let mut refused_child = SimThing::new(SimThingKind::Cohort, 2);
    for _ in 0..3 {
        refused_child.add_child(SimThing::new(SimThingKind::Cohort, 2));
    }
    let refused_id = refused_child.id;
    let live_before = session.proto.allocator.live_count();
    let facts_before = session
        .integration_schedule()
        .entries()
        .iter()
        .filter(|row| row.row_kind() == IntegrationScheduleRowKind::GrowthEntitlementRefusal)
        .count();
    session
        .tx
        .submit_boundary(BoundaryRequest::AddChild {
            parent: parent_id,
            child: refused_child,
        })
        .unwrap();
    session.step_once().expect("refused AddChild boundary");
    assert!(!session.proto.root.contains_id(refused_id));
    assert!(session.proto.allocator.slot_of(refused_id).is_none());
    assert_eq!(session.proto.allocator.live_count(), live_before);
    assert_eq!(
        session
            .integration_schedule()
            .entries()
            .iter()
            .filter(|row| row.row_kind() == IntegrationScheduleRowKind::GrowthEntitlementRefusal)
            .count(),
        facts_before + 1
    );

    let refused_entries = session.proto.take_delta_log();
    let refused_facts: Vec<_> = refused_entries
        .iter()
        .filter_map(|entry| match entry {
            BoundaryDeltaEntry::GrowthResidencyRefused { fact } => Some(fact),
            _ => None,
        })
        .collect();
    assert_eq!(refused_facts.len(), 1, "one canonical U fact; no retry");
    let RecordedGrowthResidencyFact::Refused(refusal) = refused_facts[0] else {
        panic!("refusal entry carried an accepted fact")
    };
    assert_eq!(refusal.candidate().grantee(), refused_id);
    assert_eq!(
        refusal.revalue_generation().get(),
        refusal.attempted_generation().get() + 1
    );
    replay.apply_frame(ReplayFrame {
        day: 1,
        entries: refused_entries,
        ..Default::default()
    });
    assert!(!replay.root.contains_id(refused_id));
    assert!(replay.allocator.slot_of(refused_id).is_none());
    assert!(matches!(
        replay.growth_residency_facts.last(),
        Some(RecordedGrowthResidencyFact::Refused(_))
    ));
}

#[test]
fn real_fission_clears_places_then_attaches_through_the_implicit_market() {
    let scenario = Scenario::rebellion_demo("growth-entitlement-fission".into(), 1, 1, 1.0, 8);
    let location_id = scenario.root.children[0].id;
    let parent_id = scenario.root.children[0].children[0].id;
    let mut session = SimSession::open(scenario).expect("GPU session opens");
    let before = session.proto.root.child_count(parent_id).unwrap();

    session.step_once().expect("fission boundary");

    assert_eq!(session.proto.root.child_count(parent_id), Some(before + 1));
    let spawned = session.proto.root.child_id(parent_id, before).unwrap();
    assert!(session.proto.allocator.slot_of(spawned).is_some());
    assert!(session
        .proto
        .allocator
        .committed_residency_placement(session.scenario.root.id, spawned)
        .is_some());
    assert!(session.proto.root.contains_id(location_id));
    assert!(session.proto.delta_log().iter().any(
        |entry| matches!(entry, BoundaryDeltaEntry::FissionOccurred { residency, .. }
            if residency.entitlement().grantee() == spawned)
    ));
}

#[test]
fn fabricated_market_grant_key_is_typed_refusal_without_attach_row_or_retry_and_revalues_next_generation(
) {
    let (scenario, parent_id) = add_child_scenario(5);
    let root_id = scenario.root.id;
    let mut session = SimSession::open(scenario).expect("GPU session opens");
    let child = SimThing::new(SimThingKind::Cohort, 1);
    let child_id = child.id;
    let candidate =
        OrdinaryGrowthCandidate::new(parent_id, child_id, 1, OrdinaryGrowthOrigin::AddChild);
    let binding = session.growth_entitlement_market().clone();
    let mut grant_schedule = IntegrationSchedule::new();
    let real_decision = binding
        .resolve_batch(
            &session.proto.allocator,
            GenerationStamp::new(0),
            &[candidate],
            &mut grant_schedule,
        )
        .expect("11.2a clears the candidate")
        .pop()
        .expect("one candidate has one decision");
    let GrowthEntitlementDecision::Granted {
        entitlement: real_entitlement,
        provenance,
        ..
    } = real_decision
    else {
        panic!("fixture must receive a full real grant")
    };
    let fabricated_key = real_entitlement.market_grant_key() ^ (1_u64 << 63);
    let fabricated = ProvisionalResidencyEntitlement::try_new(
        real_entitlement.granter(),
        real_entitlement.grantee(),
        fabricated_key,
        real_entitlement.quantity(),
        real_entitlement.granted_generation(),
    )
    .expect("the remanded bare-key credential is constructible");

    // Reproduce the DA falsifier through the raw kernel placement bridge. A
    // fabricated entitlement can still obtain a raw commit, but that commit is
    // not the ordinary-mutation capability introduced by the provenance seal.
    let mut forged_allocator = session.proto.allocator.clone();
    let mut forged_schedule = IntegrationSchedule::new();
    let forged_commit = forged_allocator
        .realize_unattached_growth_residency(
            fabricated,
            parent_id,
            GenerationStamp::new(0),
            &mut forged_schedule,
        )
        .expect("raw placement reproduces the pre-remand open credential path");
    assert_eq!(
        forged_commit.entitlement().market_grant_key(),
        fabricated_key
    );

    session
        .tx
        .submit_boundary(BoundaryRequest::AddChild {
            parent: parent_id,
            child: child.clone(),
        })
        .unwrap();
    let n_dims = session.proto.registry.total_columns;
    session.patcher.drain(
        &session.rx,
        &session.proto.registry,
        &session.proto.allocator,
        n_dims,
        &mut session.coord.shadow,
        None,
    );
    let mut schedule = IntegrationSchedule::new();
    let refused = session
        .proto
        .execute_with_boundary_hook_and_growth(
            Vec::new(),
            &mut session.patcher,
            &mut session.coord,
            &mut session.state,
            0,
            &mut schedule,
            |_| {},
            |_, _, candidates, _| {
                assert_eq!(candidates, &[candidate]);
                Ok(vec![GrowthEntitlementDecision::granted(
                    candidate, fabricated, provenance,
                )])
            },
        )
        .expect("fabricated credential is an ordinary typed refusal");

    assert!(!session.proto.root.contains_id(child_id));
    assert!(session.proto.allocator.slot_of(child_id).is_none());
    assert!(session
        .proto
        .allocator
        .committed_residency_placement(root_id, child_id)
        .is_none());
    assert_eq!(refused.growth_residency_facts.len(), 1);
    let RecordedGrowthResidencyFact::Refused(refusal) = &refused.growth_residency_facts[0] else {
        panic!("fabricated key produced an accepted fact")
    };
    assert!(matches!(
        refusal.reason(),
        OrdinaryGrowthRefusalReason::GrantProvenanceMismatch {
            recorded_market_grant_key,
            presented_market_grant_key,
        } if *recorded_market_grant_key == provenance.stable_key()
            && *presented_market_grant_key == fabricated_key
    ));
    assert_eq!(refusal.attempted_generation(), GenerationStamp::new(0));
    assert_eq!(refusal.revalue_generation(), GenerationStamp::new(1));
    assert_eq!(
        schedule
            .entries()
            .iter()
            .filter(|row| row.row_kind() == IntegrationScheduleRowKind::GrowthEntitlementRefusal)
            .count(),
        1,
        "one canonical refusal fact; no same-generation retry"
    );
    assert_eq!(
        schedule
            .entries()
            .iter()
            .filter(|row| row.row_kind() == IntegrationScheduleRowKind::ResidencyPlacementCommit)
            .count(),
        0,
        "provenance refusal mints no authoritative residency row"
    );

    // The retained U candidate is eligible for ordinary revaluation only at
    // the next generation. The real market path then admits and attaches it.
    session
        .tx
        .submit_boundary(BoundaryRequest::AddChild {
            parent: parent_id,
            child,
        })
        .unwrap();
    session.patcher.drain(
        &session.rx,
        &session.proto.registry,
        &session.proto.allocator,
        n_dims,
        &mut session.coord.shadow,
        None,
    );
    let accepted = session
        .proto
        .execute_with_boundary_hook_and_growth(
            Vec::new(),
            &mut session.patcher,
            &mut session.coord,
            &mut session.state,
            1,
            &mut schedule,
            |_| {},
            |allocator, generation, candidates, integration_schedule| {
                binding
                    .resolve_batch(allocator, generation, candidates, integration_schedule)
                    .map_err(|error| error.to_string())
            },
        )
        .expect("next-generation real grant revalues and attaches");
    assert!(session.proto.root.contains_id(child_id));
    assert!(session.proto.allocator.slot_of(child_id).is_some());
    assert!(session
        .proto
        .allocator
        .committed_residency_placement(root_id, child_id)
        .is_some());
    assert!(matches!(
        accepted.growth_residency_facts.as_slice(),
        [RecordedGrowthResidencyFact::Accepted(_)]
    ));
}

fn normalized_decisions(decisions: &[GrowthEntitlementDecision]) -> Vec<(u32, u32, Option<u64>)> {
    let mut rows = decisions
        .iter()
        .map(|decision| match *decision {
            GrowthEntitlementDecision::Granted {
                candidate,
                entitlement,
                ..
            } => (
                candidate.grantee().raw(),
                entitlement.quantity(),
                Some(entitlement.market_grant_key()),
            ),
            GrowthEntitlementDecision::Refused {
                candidate,
                granted,
                market_grant_key,
            } => (candidate.grantee().raw(), granted, market_grant_key),
        })
        .collect::<Vec<_>>();
    rows.sort();
    rows
}

fn place_and_record(
    mut allocator: SlotAllocator,
    generation: GenerationStamp,
    decisions: &[GrowthEntitlementDecision],
) -> (Vec<GrowthResidencyCommit>, IntegrationSchedule) {
    let mut schedule = IntegrationSchedule::new();
    let mut ordered = decisions.to_vec();
    ordered.sort_by_key(|decision| decision.candidate());
    let mut commits = Vec::new();
    for decision in ordered {
        match decision {
            GrowthEntitlementDecision::Granted {
                candidate,
                entitlement,
                ..
            } => commits.push(
                allocator
                    .realize_unattached_growth_residency(
                        entitlement,
                        candidate.structural_parent(),
                        generation,
                        &mut schedule,
                    )
                    .unwrap(),
            ),
            GrowthEntitlementDecision::Refused {
                candidate,
                market_grant_key,
                ..
            } => schedule.record_kind(
                IntegrationScheduleRowKind::GrowthEntitlementRefusal,
                generation,
                generation,
                market_grant_key.unwrap_or_else(|| candidate.product_key()),
            ),
        }
    }
    (commits, schedule)
}

#[test]
fn oversubscribed_mixed_batch_is_permutation_independent_through_schedule_and_placement() {
    const CANDIDATE_ORDER_CASES: [[usize; 2]; 2] = [[0, 1], [1, 0]];

    let mut root = SimThing::new(SimThingKind::World, 0);
    let parent = SimThing::new(SimThingKind::Location, 0);
    let parent_id = parent.id;
    root.add_child(parent);
    let granter = root.id;
    let mut allocator = SlotAllocator::new();
    allocator
        .install_initial_tree(&root)
        .expect("initial tree install");
    allocator
        .declare_root_residency_extent(granter, ResidencyExtent::try_new(0, 5).unwrap())
        .unwrap();
    let binding = GrowthEntitlementMarketBinding::implicit_root_standing(granter).unwrap();
    let fission = OrdinaryGrowthCandidate::new(
        parent_id,
        SimThing::new(SimThingKind::Cohort, 0).id,
        1,
        OrdinaryGrowthOrigin::Fission,
    );
    let add_child = OrdinaryGrowthCandidate::new(
        parent_id,
        SimThing::new(SimThingKind::Cohort, 0).id,
        3,
        OrdinaryGrowthOrigin::AddChild,
    );
    let generation = GenerationStamp::new(9);
    let candidates = [fission, add_child];
    let decisions = CANDIDATE_ORDER_CASES.map(|order| {
        let ordered = order.map(|index| candidates[index]);
        let mut grant_schedule = IntegrationSchedule::new();
        binding
            .resolve_batch(&allocator, generation, &ordered, &mut grant_schedule)
            .unwrap()
    });
    let [forward, reverse] = decisions;

    assert_eq!(
        normalized_decisions(&forward),
        normalized_decisions(&reverse)
    );
    assert_eq!(
        forward
            .iter()
            .filter(|decision| matches!(decision, GrowthEntitlementDecision::Granted { .. }))
            .count(),
        1
    );
    assert_eq!(
        forward
            .iter()
            .filter(|decision| matches!(decision, GrowthEntitlementDecision::Refused { .. }))
            .count(),
        1
    );
    let (forward_commits, forward_schedule) =
        place_and_record(allocator.clone(), generation, &forward);
    let (reverse_commits, reverse_schedule) = place_and_record(allocator, generation, &reverse);
    assert_eq!(forward_commits, reverse_commits);
    assert_eq!(forward_schedule, reverse_schedule);
}

#[test]
fn grantless_ordinary_add_child_is_rejected_at_the_only_structural_door() {
    let (scenario, parent_id) = add_child_scenario(4);
    let child = SimThing::new(SimThingKind::Cohort, 0);
    let child_id = child.id;
    let mut allocator = SlotAllocator::new();
    allocator
        .install_initial_tree(&scenario.root)
        .expect("initial tree install");
    let mut runtime = simthing_sim::SimRuntimeTree::admit(scenario.root);
    let mut registry = scenario.registry;
    let n_dims = registry.total_columns;
    let mut shadow = vec![0.0; 4 * n_dims];
    let outcome = simthing_sim::tree_mutation::apply_structural_mutations(
        vec![BoundaryRequest::AddChild {
            parent: parent_id,
            child,
        }],
        &mut runtime,
        &mut allocator,
        &mut registry,
        &mut shadow,
        n_dims,
        None,
        GenerationStamp::new(0),
        &mut Default::default(),
        &BTreeMap::new(),
    );
    assert_eq!(outcome.rejected_growth_entitlement, 1);
    assert!(!runtime.contains_id(child_id));
    assert!(allocator.slot_of(child_id).is_none());
}
