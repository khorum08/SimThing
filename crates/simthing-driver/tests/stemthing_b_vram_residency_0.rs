//! STEMTHING-B-VRAM-RESIDENCY-0 standing integration witness.

use std::collections::BTreeSet;

use simthing_core::owner_channel::OwnerRef;
use simthing_core::{
    resolve_slot_through_chain, AnchorRemapOperation, AnchoredLocusMap, GenerationStamp,
    IntegrationSchedule, IntegrationScheduleRowKind, SimThing, SimThingKind, TransformOp,
};
use simthing_driver::residency_market::{
    realize_market_grant_residency, relocate_market_grant_residency, ResidencyMarketBridgeError,
};
use simthing_gpu::{
    ResidencyExtent, ResidencyPlacementDisposition, ResidencyPlacementError,
    ResidencyPlacementRefusalReason, SlotAllocator,
};
use simthing_spec::{
    admit_specialization_flow_market, clear_constrained_claims_at_generation,
    AdmittedSpecializationFlowMarket, AuthoredClearingProgram, ClearingRemainderAuthority,
    ConservedOfferingSpec, ConstrainedClaim, ConstrainedSupply, DrawEnvelopeTemplateSpec,
    GrantLifecycleError, MarketGrantRecord, OfferingPriceVectorSpec, OwnerChannelScopeKey,
    ResourceKey, RuntimeOwnerSiloDemandBucket, ScopeId, SpecializationFlowMarketSpec,
};

fn admitted_market() -> AdmittedSpecializationFlowMarket {
    admit_specialization_flow_market(
        &simthing_core::seed_profiles(),
        &BTreeSet::from(["while-resident".into()]),
        SpecializationFlowMarketSpec {
            specialization_profile_id: "session-root".into(),
            offerings: vec![ConservedOfferingSpec {
                id: "residency-claim".into(),
                resource_key: ResourceKey::new("residency-slots"),
                price: OfferingPriceVectorSpec {
                    unit_cost: 1.0,
                    default_clearing_weight: 1.0,
                },
            }],
            draw_envelopes: vec![DrawEnvelopeTemplateSpec {
                id: "residency-draw".into(),
                offering_refs: vec!["residency-claim".into()],
                lifecycle_trigger_refs: vec!["while-resident".into()],
                min_quantity: 1,
                max_quantity: 16,
            }],
        },
    )
    .expect("market admission")
}

fn grant(
    market: &AdmittedSpecializationFlowMarket,
    granter: simthing_core::SimThingId,
    grantee: simthing_core::SimThingId,
    quantity: u32,
    generation: u32,
) -> MarketGrantRecord {
    let scope = OwnerChannelScopeKey {
        owner_ref: OwnerRef::new("alpha"),
        resource_key: ResourceKey::new("residency-slots"),
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
    let cleared = clear_constrained_claims_at_generation(
        &[ConstrainedSupply {
            scope,
            available: quantity,
        }],
        &[claim],
        &AuthoredClearingProgram::new(TransformOp::set(1.0)),
        ClearingRemainderAuthority {
            granter,
            generation: GenerationStamp::new(generation),
        },
    )
    .expect("constrained clearing mints the sealed grant");
    let mut mutated = cleared[0].grants[0].clone();
    mutated.granted = mutated.granted.saturating_add(1);
    let mut lifecycle_schedule = IntegrationSchedule::new();
    assert!(matches!(
        market.record_cleared_grant(
            granter,
            "residency-claim",
            &mutated,
            GenerationStamp::new(generation),
            &mut lifecycle_schedule,
        ),
        Err(GrantLifecycleError::InvalidClearingSeal)
    ));
    market
        .record_cleared_grant(
            granter,
            "residency-claim",
            &cleared[0].grants[0],
            GenerationStamp::new(generation),
            &mut lifecycle_schedule,
        )
        .expect("graduated clearing result mints the only market grant")
}

#[test]
fn cleared_entitlement_places_locally_refuses_to_u_then_revalues_and_relocates() {
    let market = admitted_market();
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    let mut granter = SimThing::new(SimThingKind::Custom("granter".into()), 0);
    let worker = SimThing::new(SimThingKind::Custom("worker".into()), 0);
    let sibling = SimThing::new(SimThingKind::Custom("sibling".into()), 0);
    let root_id = root.id;
    let granter_id = granter.id;
    let worker_id = worker.id;
    let sibling_id = sibling.id;
    granter.add_child(worker);
    granter.add_child(sibling);
    root.add_child(granter);

    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&root);
    allocator
        .declare_root_residency_extent(root_id, ResidencyExtent::try_new(0, 16).unwrap())
        .unwrap();
    let mut schedule = IntegrationSchedule::new();

    let granter_grant = grant(&market, root_id, granter_id, 8, 2);
    realize_market_grant_residency(
        &mut allocator,
        &market,
        &granter_grant,
        ResidencyExtent::try_new(0, 8).unwrap(),
        GenerationStamp::new(3),
        &mut schedule,
    )
    .expect("root checks only its direct child's extent");

    let worker_grant = grant(&market, granter_id, worker_id, 3, 3);
    let first = realize_market_grant_residency(
        &mut allocator,
        &market,
        &worker_grant,
        ResidencyExtent::try_new(0, 3).unwrap(),
        GenerationStamp::new(3),
        &mut schedule,
    )
    .expect("child granter derives its containing extent from its committed placement");
    assert_eq!(
        first.disposition(),
        ResidencyPlacementDisposition::Committed
    );

    let sibling_grant = grant(&market, granter_id, sibling_id, 2, 4);
    let refusal = realize_market_grant_residency(
        &mut allocator,
        &market,
        &sibling_grant,
        ResidencyExtent::try_new(2, 2).unwrap(),
        GenerationStamp::new(4),
        &mut schedule,
    )
    .expect_err("ordinary overlap is physical infeasibility, not a crash");
    let ResidencyMarketBridgeError::Placement(ResidencyPlacementError::Refused(refusal)) = refusal
    else {
        panic!("ordinary infeasibility must remain a typed refusal");
    };
    assert!(matches!(
        refusal.reason(),
        ResidencyPlacementRefusalReason::Overlap { .. }
    ));
    assert_eq!(refusal.retained_unmet_quantity(), 2);
    assert_eq!(refusal.revalue_generation(), GenerationStamp::new(5));
    assert!(allocator
        .committed_residency_placement(granter_id, sibling_id)
        .is_none());
    assert_eq!(
        schedule
            .entries_of_kind(IntegrationScheduleRowKind::ResidencyPlacementRefusal)
            .count(),
        1
    );

    // A later generation may explicitly revalue U. The failed generation does not retry,
    // converge, or silently re-clear the grant.
    realize_market_grant_residency(
        &mut allocator,
        &market,
        &sibling_grant,
        ResidencyExtent::try_new(3, 2).unwrap(),
        GenerationStamp::new(5),
        &mut schedule,
    )
    .expect("next-generation revaluation can choose a new proposed extent");

    let rows_before_unchanged = schedule.entries().len();
    let unchanged = realize_market_grant_residency(
        &mut allocator,
        &market,
        &worker_grant,
        ResidencyExtent::try_new(0, 3).unwrap(),
        GenerationStamp::new(6),
        &mut schedule,
    )
    .expect("unchanged placement needs no global per-generation proof");
    assert_eq!(
        unchanged.disposition(),
        ResidencyPlacementDisposition::Unchanged
    );
    assert_eq!(schedule.entries().len(), rows_before_unchanged);

    let old_identity = first.placement().identity();
    let old_slot = allocator.slot_of(worker_id).unwrap();
    let sibling_slot = allocator.slot_of(sibling_id).unwrap();
    let binding_before_relocation = allocator.binding_table_snapshot();
    let rows_before_failed_relocation = schedule.entries().len();
    let mut invalid_assignment = binding_before_relocation.clone();
    invalid_assignment.insert(
        worker_id,
        simthing_core::SlotIndex::new(allocator.capacity() as u32),
    );
    assert!(matches!(
        relocate_market_grant_residency(
            &mut allocator,
            &market,
            &worker_grant,
            ResidencyExtent::try_new(5, 3).unwrap(),
            GenerationStamp::new(7),
            &invalid_assignment,
            &AnchoredLocusMap::new(),
            &AnchoredLocusMap::new(),
            &mut schedule,
        ),
        Err(ResidencyMarketBridgeError::Placement(
            ResidencyPlacementError::RemapRefused { .. }
        ))
    ));
    assert_eq!(
        allocator.binding_table_snapshot(),
        binding_before_relocation
    );
    assert_eq!(schedule.entries().len(), rows_before_failed_relocation);
    assert_eq!(
        allocator
            .committed_residency_placement(granter_id, worker_id)
            .unwrap()
            .extent(),
        ResidencyExtent::try_new(0, 3).unwrap()
    );

    let mut assignment = binding_before_relocation;
    assignment.insert(worker_id, sibling_slot);
    assignment.insert(sibling_id, old_slot);
    let relocation = relocate_market_grant_residency(
        &mut allocator,
        &market,
        &worker_grant,
        ResidencyExtent::try_new(5, 3).unwrap(),
        GenerationStamp::new(7),
        &assignment,
        &AnchoredLocusMap::new(),
        &AnchoredLocusMap::new(),
        &mut schedule,
    )
    .expect("relocation routes through the existing epoch-rebind authority");
    assert_eq!(
        relocation.placement().disposition(),
        ResidencyPlacementDisposition::Relocated
    );
    assert_eq!(relocation.placement().placement().identity(), old_identity);
    assert_eq!(
        relocation.remap().operation,
        AnchorRemapOperation::EpochRebind
    );
    assert_eq!(
        resolve_slot_through_chain([relocation.remap()], worker_id, old_slot),
        sibling_slot,
        "stable object identity follows THE existing remap chain"
    );
    assert_eq!(
        schedule
            .entries_of_kind(IntegrationScheduleRowKind::ResidencyRelocation)
            .count(),
        1
    );
    allocator
        .audit_residency_level(granter_id, GenerationStamp::new(7), &mut schedule)
        .expect("the owning granter's direct placements remain disjoint and in-bounds");
}
