//! 15.12 DA 5572589566: the generic CPU oracle consumes the existing exact
//! live-basis reference; proportional basis retains the legacy neutral result.
use simthing_core::{
    ColumnIndex, DimensionRegistry, ExecutionIncarnation, GenerationStamp, IntegrationSchedule,
    OwnerRef, SimProperty, SimThing, SimThingKind, SlotIndex, TransformOp, TreeExecutionAuthority,
    TreeGenerationAuthority, TreeRealmId,
};
use simthing_kernel::{
    ResidentApportionmentClaim, ResidentApportionmentPlan, ResidentClearingAdmission,
    ResidentClearingBudgets, ResidentClearingPlan, ResidentDrawId, ResidentExactBasisIdentity,
    ResidentOwnerId, ResidentResourceId, ResidentScopeId, SlotAllocator,
};
use simthing_spec::{
    clear_constrained_claims_at_generation, AuthoredClearingProgram, ClearingRemainderAuthority,
    ConstrainedClaim, ConstrainedSupply, OwnerChannelScopeKey, ResourceKey,
    RuntimeOwnerSiloDemandBucket, ScopeId,
};

#[test]
fn admitted_live_basis_preserves_neutral_corpus_and_caps() {
    let tree = SimThing::new(SimThingKind::GameSession, 0);
    let mut registry = DimensionRegistry::new();
    registry.register(SimProperty::simple("15.12-referee", "allocated", 0));
    let mut slots = SlotAllocator::new();
    slots.install_initial_tree(&tree).unwrap();
    let schedule = IntegrationSchedule::new();
    let generation = TreeGenerationAuthority::new(GenerationStamp::new(0));
    let authority = TreeExecutionAuthority::seal(
        TreeRealmId::from_u128(0x1512).unwrap(),
        ExecutionIncarnation::new(1).unwrap(),
        &tree,
        &generation,
        &schedule,
        &registry,
        &slots,
    )
    .unwrap();
    let context = authority.seal_context().unwrap();
    let binding = context.bind(&authority).unwrap();
    let plan = ResidentClearingPlan::build(
        &binding,
        (0..3).map(|index| ResidentClearingAdmission {
            owner: ResidentOwnerId::new(context.qualify(tree.id)),
            resource: ResidentResourceId::new(1),
            scope: ResidentScopeId::new(1),
            draw: ResidentDrawId::new(100 + index),
        }),
        ResidentClearingBudgets::new(1, 1, 1, 3, 3, 8192, 8192, 192, 64).unwrap(),
    )
    .unwrap();
    let scope = OwnerChannelScopeKey {
        owner_ref: OwnerRef::new("15.12-neutral"),
        resource_key: ResourceKey::new("capacity"),
        scope_id: ScopeId::new("scope"),
    };
    let program = AuthoredClearingProgram::new(TransformOp::set(1.0));
    let make = |requests: &[u32]| {
        requests
            .iter()
            .enumerate()
            .map(|(i, &requested)| {
                ConstrainedClaim::from_runtime_demand(
                    &RuntimeOwnerSiloDemandBucket {
                        owner_ref: scope.owner_ref.clone(),
                        resource_key: scope.resource_key.clone(),
                        scope_id: scope.scope_id.clone(),
                        requested,
                        priority: 0,
                        source_simthing_id_raw: Some(100 + i as u32),
                    },
                    1.0,
                )
                .unwrap()
            })
            .collect::<Vec<_>>()
    };
    let bind = |claims: &mut [ConstrainedClaim], bases: Vec<f32>, supply, stamp, reverse| {
        let mut rows: Vec<_> = claims
            .iter()
            .enumerate()
            .map(|(i, claim)| {
                let row = plan
                    .rows()
                    .iter()
                    .position(|row| {
                        plan.dictionaries().draws()[row.draw().get() as usize].get()
                            == 100 + i as u64
                    })
                    .unwrap();
                ResidentApportionmentClaim::new(
                    row as u32,
                    claim.source_simthing_id(),
                    claim.requested(),
                    supply,
                    0,
                    SlotIndex::new(i as u32),
                    ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
                    ResidentExactBasisIdentity::LiveAllocatedFlow,
                )
            })
            .collect();
        if reverse {
            rows.reverse();
        }
        let exact = ResidentApportionmentPlan::build(&plan, rows, tree.id, stamp, 0).unwrap();
        ConstrainedClaim::bind_resident_oracle(claims, &plan, exact, bases, 1).unwrap();
    };
    let mut neutral_count = 0;
    for requests in [
        &[0, 0][..],
        &[0, 5],
        &[1, 1, 1],
        &[1, 2, 4],
        &[1, 100],
        &[10, 10],
        &[100, 200, 300],
        &[65535, 32768, 1],
        &[u32::MAX],
    ] {
        let total: u32 = requests.iter().sum();
        for supply in [0, 1, 3, 7, total / 2, total] {
            for stamp in [
                GenerationStamp::new(0),
                GenerationStamp::new(1),
                GenerationStamp::new(u32::MAX),
            ] {
                // At u32::MAX the exact cap repairs the sole f32 conversion.
                for factor in if total == u32::MAX {
                    &[1.0][..]
                } else {
                    &[0.5, 1.0, 2.0][..]
                } {
                    for reverse in [false, true] {
                        let supplies = [ConstrainedSupply {
                            scope: scope.clone(),
                            available: supply,
                        }];
                        let authority = ClearingRemainderAuthority {
                            granter: tree.id,
                            generation: stamp,
                        };
                        let mut claims = make(requests);
                        let neutral = clear_constrained_claims_at_generation(
                            &supplies, &claims, &program, authority,
                        )
                        .unwrap();
                        bind(
                            &mut claims,
                            requests
                                .iter()
                                .map(|value| *value as f32 * factor)
                                .collect(),
                            supply,
                            stamp,
                            reverse,
                        );
                        let live = clear_constrained_claims_at_generation(
                            &supplies, &claims, &program, authority,
                        )
                        .unwrap();
                        assert_eq!(live,neutral,"proportional basis {requests:?} / {supply} / {factor} / {stamp:?} / {reverse}");
                        neutral_count += 1;
                    }
                }
            }
        }
    }
    let mut nondegenerate = 0;
    for (requests, bases, supply, expected) in [
        (&[18, 5][..], &[1.0, 1.0][..], 4, &[2, 2][..]),
        (&[1, 100][..], &[1.0, 1.0][..], 101, &[1, 100][..]),
        (&[1, 3, 100][..], &[1.0, 1.0, 1.0][..], 100, &[1, 3, 96][..]),
        (&[10, 10][..], &[0.0, 1.0][..], 10, &[0, 10][..]),
        (&[0, 5][..], &[1.0, 1.0][..], 4, &[0, 4][..]),
    ] {
        for reverse in [false, true] {
            let mut claims = make(requests);
            let stamp = GenerationStamp::new(2);
            bind(&mut claims, bases.to_vec(), supply, stamp, reverse);
            let result = clear_constrained_claims_at_generation(
                &[ConstrainedSupply {
                    scope: scope.clone(),
                    available: supply,
                }],
                &claims,
                &program,
                ClearingRemainderAuthority {
                    granter: tree.id,
                    generation: stamp,
                },
            )
            .unwrap();
            assert_eq!(
                result[0]
                    .grants
                    .iter()
                    .map(|g| g.granted)
                    .collect::<Vec<_>>(),
                expected
            );
            nondegenerate += 1;
        }
    }
    println!("15.12 converged Spec oracle: {neutral_count} exact full-result neutral-degeneration comparisons; {nondegenerate} nondegenerate mixed/cap/zero/order cases; all use sealed LiveAllocatedFlow plans and unchanged Kernel reference");
}
