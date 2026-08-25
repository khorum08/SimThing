use std::collections::BTreeMap;

use simthing_core::{
    DimensionRegistry, GenerationStamp, IntegrationSchedule, IntegrationScheduleRowKind,
    SimProperty, SimThing, SimThingKind,
};
use simthing_driver::{GrowthEntitlementMarketBinding, Scenario, SimSession};
use simthing_feeder::BoundaryRequest;
use simthing_gpu::{GrowthResidencyCommit, ResidencyExtent, SlotAllocator};
use simthing_sim::{
    BoundaryDeltaEntry, GrowthEntitlementDecision, OrdinaryGrowthCandidate, OrdinaryGrowthOrigin,
    RecordedGrowthResidencyFact, ReplayDriver, ReplayFrame, ReplayGrowthError,
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

    let mut replay = ReplayDriver::from_snapshot(replay_snapshot);
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

fn normalized_decisions(decisions: &[GrowthEntitlementDecision]) -> Vec<(u32, u32, Option<u64>)> {
    let mut rows = decisions
        .iter()
        .map(|decision| match *decision {
            GrowthEntitlementDecision::Granted {
                candidate,
                entitlement,
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
    let mut root = SimThing::new(SimThingKind::World, 0);
    let parent = SimThing::new(SimThingKind::Location, 0);
    let parent_id = parent.id;
    root.add_child(parent);
    let granter = root.id;
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&root);
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
    let forward = binding
        .resolve_batch(&allocator, generation, &[fission, add_child])
        .unwrap();
    let reverse = binding
        .resolve_batch(&allocator, generation, &[add_child, fission])
        .unwrap();

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
    allocator.install_initial_tree(&scenario.root);
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
