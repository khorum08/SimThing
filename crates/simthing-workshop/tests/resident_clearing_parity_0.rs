//! RESIDENT-CLEARING-PARITY-0 — neutral oracle parity, lawful pressure drift,
//! fail-closed error equivalence, recursive `T_s`, scale, multi-tree, and
//! adapter-qualification referee.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::process::Command;

use simthing_core::owner_channel::OwnerRef;
use simthing_core::{
    ColumnIndex, CombineFn, DimensionRegistry, ExecutionIncarnation, GenerationStamp,
    IntegrationSchedule, SimProperty, SimPropertyId, SimThing, SimThingId, SlotIndex, SourceSpec,
    TransformOp, TreeExecutionAuthority, TreeGenerationAuthority, TreeRealmId,
};
use simthing_driver::{
    build_custom_layout, plan_arena_allocation, plan_resident_exact_apportionment,
    produce_runtime_rf_next_generation_demands_for_tick, run_arena_allocation_oracle,
    ArenaTreeLayout, FissionPolicy, GpuArenaDescriptor, HierarchyNode, NodeColumnRefs,
};
use simthing_gpu::{
    wgpu, GpuContext, ResidentApportionmentDispatch, ResidentApportionmentSession,
    ResidentApportionmentWorkgroupSize, ResidentClearingBuffers, ResidentClearingGpuError,
    WorldGpuState, RESIDENT_CLEARING_ABI_VERSION,
};
use simthing_kernel::{
    execute_resident_apportionment_cpu, ResidentApportionmentClaim, ResidentApportionmentError,
    ResidentApportionmentPlan, ResidentClearingAdmission, ResidentClearingBudgets,
    ResidentClearingPlan, ResidentConstrainedProduct, ResidentDrawId, ResidentOwnerId,
    ResidentRecursiveSupplyIntake, ResidentResourceId, ResidentScopeId, ResidentSettlementOutput,
};
use simthing_spec::{
    clear_constrained_claims_at_generation, clear_reduced_owner_channels_at_generation,
    AuthoredClaimClearingData, AuthoredClearingProgram, ClearingRemainderAuthority,
    ConstrainedClaim, ConstrainedClearingError, ConstrainedClearingResult, ConstrainedSupply,
    OwnerChannelRfBucket, OwnerChannelRfOwnAggregate, OwnerChannelRfReduceUpReport,
    OwnerChannelRfSteadSurface, OwnerChannelScopeKey, ResourceKey, RuntimeOwnerSiloDemandBucket,
    RuntimeRfDemandGenerationAuthority, RuntimeRfTickErrorKind, ScopeId,
};

const QUALIFIED_RECORD_FINGERPRINT: u64 = 0x73ae_5e62_1b3e_5021;

fn col(raw: usize) -> ColumnIndex {
    ColumnIndex::from_raw_for_oracle_or_rehearsal(raw)
}

fn cols() -> NodeColumnRefs {
    NodeColumnRefs {
        intrinsic_flow_col: col(0),
        allocated_flow_col: col(1),
        weight_col: col(2),
        intrinsic_flow_sum_col: col(3),
        weight_sum_col: col(4),
        balance_col: None,
        balance_governing_col: None,
        propagated_intrinsic_flow_col: col(5),
        propagated_allocated_flow_col: col(6),
        propagated_weight_sum_col: col(7),
        hosted_simthing_id_col: col(8),
    }
}

fn loaded_tree(root_raw: u32) -> SimThing {
    serde_json::from_str(&format!(
        r#"{{
            "id": {root_raw},
            "kind": "GameSession",
            "properties": [],
            "resource_parent_edges": [],
            "overlays": [],
            "children": [],
            "spawned_generation": 4
        }}"#
    ))
    .expect("persisted resident parity fixture")
}

fn budgets() -> ResidentClearingBudgets {
    ResidentClearingBudgets::new(8, 8, 256, 256, 256, 262_144, 1_048_576, 32_768, 64)
        .expect("14.2-admitted exact scratch rows")
}

fn resident_plan(
    ctx: &GpuContext,
    realm_raw: u128,
    root_raw: u32,
    generation_raw: u32,
    count: u32,
    scope_count: u32,
    reverse_admission: bool,
) -> (ResidentClearingPlan, ResidentClearingBuffers) {
    let tree = loaded_tree(root_raw);
    let realm = TreeRealmId::from_u128(realm_raw).unwrap();
    let generation = TreeGenerationAuthority::new(GenerationStamp::new(generation_raw));
    let schedule = IntegrationSchedule::new();
    let registry = DimensionRegistry::new();
    let mut residency = simthing_kernel::SlotAllocator::new();
    residency.install_initial_tree(&tree).unwrap();
    let authority = TreeExecutionAuthority::seal(
        realm,
        ExecutionIncarnation::new(1).unwrap(),
        &tree,
        &generation,
        &schedule,
        &registry,
        &residency,
    )
    .unwrap();
    let context = authority.seal_context().unwrap();
    let binding = context.bind(&authority).unwrap();
    let owner = ResidentOwnerId::new(context.qualify(tree.id));
    let mut admissions: Vec<_> = (0..count)
        .map(|index| ResidentClearingAdmission {
            owner,
            resource: ResidentResourceId::new(1),
            scope: ResidentScopeId::new(10 + u64::from(index % scope_count.max(1))),
            draw: ResidentDrawId::new(u64::from(1_000 + index)),
        })
        .collect();
    if reverse_admission {
        admissions.reverse();
    }
    let plan = ResidentClearingPlan::build(&binding, admissions, budgets()).unwrap();
    let buffers = ResidentClearingBuffers::allocate(&ctx.device, &binding, &plan).unwrap();
    (plan, buffers)
}

fn semantic_row_for_draw(plan: &ResidentClearingPlan, draw: u64) -> u32 {
    plan.rows()
        .iter()
        .position(|row| plan.dictionaries().draws()[row.draw().get() as usize].get() == draw)
        .and_then(|index| u32::try_from(index).ok())
        .expect("draw has one canonical semantic row")
}

fn source(index: u32) -> SimThingId {
    SimThingId::from_session_raw(1_000 + index)
}

fn exact_claims(
    semantic_plan: &ResidentClearingPlan,
    requests: &[u32],
    order: impl IntoIterator<Item = u32>,
    available: impl Fn(u32) -> u32,
    precedence: impl Fn(u32) -> u32,
    slot: impl Fn(u32) -> u32,
) -> Vec<ResidentApportionmentClaim> {
    order
        .into_iter()
        .map(|index| {
            ResidentApportionmentClaim::new(
                semantic_row_for_draw(semantic_plan, u64::from(1_000 + index)),
                source(index),
                requests[index as usize],
                available(index),
                precedence(index),
                SlotIndex::new(slot(index)),
                col(0),
            )
        })
        .collect()
}

fn arena_layout() -> ArenaTreeLayout {
    build_custom_layout(
        0,
        &GpuArenaDescriptor {
            name: "resident-parity-terminal".into(),
            flow_property_id: SimPropertyId(1),
            balance_property_id: None,
            max_participants: 256,
            max_coupling_fanout: 4,
            max_orderband_depth: 16,
            fission_policy: FissionPolicy::default(),
            participant_range: (0, 0),
            wildcard_max_expansion: None,
            reserved_orderband_depth: 0,
        },
        cols(),
        vec![HierarchyNode {
            participant_slot: SlotIndex::new(0),
            hosted_simthing_id: SimThingId::from_session_raw(7),
            depth: 0,
            children: vec![],
            cols: cols(),
        }],
    )
    .unwrap()
}

fn world(ctx: GpuContext, n_slots: u32) -> (WorldGpuState, Vec<f32>) {
    let mut registry = DimensionRegistry::new();
    registry.register(SimProperty::simple("resident", "allocated_flow", 1));
    let state = WorldGpuState::new(ctx, &registry, n_slots.max(1));
    let values = vec![0.0; state.values_len()];
    state.install_resolved_values_at_boundary(&values);
    (state, values)
}

fn set_allocated(values: &mut [f32], n_dims: u32, slot: u32, value: f32) {
    values[(slot * n_dims) as usize] = value;
}

fn run_gpu(
    state: &WorldGpuState,
    session: &mut ResidentApportionmentSession,
    buffers: &ResidentClearingBuffers,
    plan: &ResidentApportionmentPlan,
    dispatch: ResidentApportionmentDispatch,
) -> Result<Vec<ResidentConstrainedProduct>, ResidentApportionmentError> {
    let (semantic_rows, scratch) = buffers.apportionment_buffers(plan).unwrap();
    let mut encoder = state
        .ctx
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("resident_parity_terminal_band_referee"),
        });
    state.encode_resident_apportionment_with_dispatch_into(
        session,
        &mut encoder,
        semantic_rows,
        scratch,
        plan,
        dispatch,
    )?;
    state.ctx.queue.submit(Some(encoder.finish()));
    let _ = state.ctx.device.poll(wgpu::Maintain::Wait);
    session.readback_products(&state.ctx, scratch, plan)
}

fn product_map(products: &[ResidentConstrainedProduct]) -> BTreeMap<u32, (u32, u32)> {
    products
        .iter()
        .map(|product| {
            (
                product.source_simthing_id().raw(),
                (product.granted(), product.unresolved()),
            )
        })
        .collect()
}

fn scope(name: &str) -> OwnerChannelScopeKey {
    OwnerChannelScopeKey {
        owner_ref: OwnerRef::new("resident-parity"),
        resource_key: ResourceKey::new("quanta"),
        scope_id: ScopeId::new(name),
    }
}

fn demand(
    key: &OwnerChannelScopeKey,
    source_raw: Option<u32>,
    requested: u32,
) -> RuntimeOwnerSiloDemandBucket {
    RuntimeOwnerSiloDemandBucket {
        owner_ref: key.owner_ref.clone(),
        resource_key: key.resource_key.clone(),
        scope_id: key.scope_id.clone(),
        requested,
        priority: 0,
        source_simthing_id_raw: source_raw,
    }
}

fn claim(
    key: &OwnerChannelScopeKey,
    index: u32,
    requested: u32,
    order_weight: f32,
) -> ConstrainedClaim {
    ConstrainedClaim::from_runtime_demand(
        &demand(key, Some(source(index).raw()), requested),
        order_weight,
    )
    .unwrap()
}

fn neutral_oracle(
    requests: &[u32],
    available: u32,
    granter: SimThingId,
    generation: GenerationStamp,
) -> Vec<ConstrainedClearingResult> {
    let key = scope("neutral");
    let claims: Vec<_> = requests
        .iter()
        .enumerate()
        .map(|(index, &requested)| claim(&key, index as u32, requested, 1.0))
        .collect();
    clear_constrained_claims_at_generation(
        &[ConstrainedSupply {
            scope: key,
            available,
        }],
        &claims,
        &AuthoredClearingProgram::new(TransformOp::set(-0.0)),
        ClearingRemainderAuthority {
            granter,
            generation,
        },
    )
    .unwrap()
}

#[test]
fn resident_clearing_parity_terminal_referee() {
    nine_neutral_items_replay_exactly_and_pressure_lawfully_changes_grants();
    negative_error_matrix_is_typed_unrepresentable_or_sealed_without_partial_products();
    three_recursive_edges_self_consume_exact_ts_and_u_recurs_once_at_n_plus_one();
    scale_multitree_physical_invariance_and_exact_qualification_hold();
}

fn nine_neutral_items_replay_exactly_and_pressure_lawfully_changes_grants() {
    let ctx = GpuContext::new_blocking().expect("real GPU for 14.5 parity referee");
    let (semantic_plan, buffers) = resident_plan(&ctx, 0x1450, 7, 23, 3, 1, false);
    let (state, mut values) = world(ctx, 3);
    let mut session = ResidentApportionmentSession::new(&state.ctx);
    let requests = [100, 200, 300];
    for (index, requested) in requests.into_iter().enumerate() {
        set_allocated(&mut values, state.n_dims, index as u32, requested as f32);
    }
    state.install_resolved_values_at_boundary(&values);
    let granter = SimThingId::from_session_raw(0);
    let generation = GenerationStamp::new(0);
    let plan = plan_resident_exact_apportionment(
        &arena_layout(),
        &semantic_plan,
        exact_claims(
            &semantic_plan,
            &requests,
            0..3,
            |_| 100,
            |_| 0,
            |index| index,
        ),
        granter,
        generation,
    )
    .unwrap();
    let oracle = neutral_oracle(&requests, 100, granter, generation);
    let oracle_grants = &oracle[0].grants;

    // Item 1: the admitted EML program canonicalizes signed zero to +0 bits.
    assert_eq!(
        oracle_grants
            .iter()
            .map(|grant| grant.clearing_score.to_bits())
            .collect::<Vec<_>>(),
        vec![0.0f32.to_bits(); 3]
    );
    // Item 2: those identical bits form exactly one equality band.
    assert_eq!(
        oracle_grants
            .iter()
            .map(|grant| grant.clearing_score.to_bits())
            .collect::<BTreeSet<_>>()
            .len(),
        1
    );
    // Items 3/4: one canonical scope and the checked request total are exact.
    assert_eq!(oracle.len(), 1);
    assert_eq!(
        oracle_grants
            .iter()
            .map(|grant| u64::from(grant.requested))
            .sum::<u64>(),
        600
    );
    // Items 5/6: bases 16/33/50 and remainders 400/200/0 assign the leftover
    // to source 1000, yielding the frozen 17/33/50 result.
    assert_eq!(
        oracle_grants
            .iter()
            .map(|grant| grant.granted)
            .collect::<Vec<_>>(),
        vec![17, 33, 50]
    );
    let numerator_remainders = requests
        .into_iter()
        .map(|requested| (100u64 * u64::from(requested)) % 600)
        .collect::<Vec<_>>();
    assert_eq!(numerator_remainders, vec![400, 200, 0]);

    let cpu = execute_resident_apportionment_cpu(&plan, &values, state.n_dims).unwrap();
    let gpu = run_gpu(
        &state,
        &mut session,
        &buffers,
        &plan,
        ResidentApportionmentDispatch::single_pass(),
    )
    .unwrap();
    let expected = BTreeMap::from([(1_000, (17, 83)), (1_001, (33, 167)), (1_002, (50, 250))]);
    // Item 8: grants and unresolved U match exactly.
    assert_eq!(product_map(&cpu), expected);
    assert_eq!(product_map(&gpu), expected);
    // Item 9: the same immutable plan and values replay bit-exactly.
    assert_eq!(
        run_gpu(
            &state,
            &mut session,
            &buffers,
            &plan,
            ResidentApportionmentDispatch::new(ResidentApportionmentWorkgroupSize::W32, 2).unwrap(),
        )
        .unwrap(),
        gpu
    );

    // Item 7: exact ties rotate under granter generation on both authorities.
    let tie_requests = [1, 1];
    let (tie_semantic, tie_buffers) = resident_plan(&state.ctx, 0x1451, 8, 24, 2, 1, false);
    let mut tie_winners = Vec::new();
    for generation_raw in [0, 1] {
        let tie_plan = plan_resident_exact_apportionment(
            &arena_layout(),
            &tie_semantic,
            exact_claims(
                &tie_semantic,
                &tie_requests,
                0..2,
                |_| 1,
                |_| 0,
                |index| index,
            ),
            granter,
            GenerationStamp::new(generation_raw),
        )
        .unwrap();
        set_allocated(&mut values, state.n_dims, 0, 1.0);
        set_allocated(&mut values, state.n_dims, 1, 1.0);
        state.install_resolved_values_at_boundary(&values);
        let resident = run_gpu(
            &state,
            &mut session,
            &tie_buffers,
            &tie_plan,
            ResidentApportionmentDispatch::single_pass(),
        )
        .unwrap();
        let frozen = neutral_oracle(
            &tie_requests,
            1,
            granter,
            GenerationStamp::new(generation_raw),
        );
        assert_eq!(
            product_map(&resident),
            frozen[0]
                .grants
                .iter()
                .map(|grant| {
                    (
                        grant.source_simthing_id.raw(),
                        (grant.granted, grant.unresolved),
                    )
                })
                .collect()
        );
        tie_winners.push(product_map(&resident));
    }
    assert_ne!(tie_winners[0], tie_winners[1]);

    // Graduated 14.3 pressure outcomes: born immediate-flow F=(3,5) gives
    // AllocatedFlow=(5.25,8.75); entitlement-first raw P=(9,5) gives
    // AllocatedFlow=(9,5). Exact 14.4 settlement consumes those magnitudes.
    let pressure_requests = [14, 14];
    let (pressure_semantic, pressure_buffers) =
        resident_plan(&state.ctx, 0x1452, 9, 25, 2, 1, false);
    let mut pressure_outcomes = Vec::new();
    for allocated in [[5.25, 8.75], [9.0, 5.0]] {
        set_allocated(&mut values, state.n_dims, 0, allocated[0]);
        set_allocated(&mut values, state.n_dims, 1, allocated[1]);
        state.install_resolved_values_at_boundary(&values);
        let pressure_plan = plan_resident_exact_apportionment(
            &arena_layout(),
            &pressure_semantic,
            exact_claims(
                &pressure_semantic,
                &pressure_requests,
                0..2,
                |_| 7,
                |_| 0,
                |index| index,
            ),
            granter,
            generation,
        )
        .unwrap();
        pressure_outcomes.push(product_map(
            &run_gpu(
                &state,
                &mut session,
                &pressure_buffers,
                &pressure_plan,
                ResidentApportionmentDispatch::single_pass(),
            )
            .unwrap(),
        ));
    }
    assert_eq!(pressure_outcomes[0][&1_000], (3, 11));
    assert_eq!(pressure_outcomes[0][&1_001], (4, 10));
    assert_eq!(pressure_outcomes[1][&1_000], (5, 9));
    assert_eq!(pressure_outcomes[1][&1_001], (2, 12));
    assert_ne!(pressure_outcomes[0], pressure_outcomes[1]);
}

fn reduced_report(
    key: &OwnerChannelScopeKey,
    source_id: SimThingId,
    deficit: u32,
) -> OwnerChannelRfReduceUpReport {
    OwnerChannelRfReduceUpReport {
        participant_count: 1,
        owner_count: 1,
        bucket_count: 1,
        surplus_total: 4,
        deficit_total: deficit,
        buckets: vec![OwnerChannelRfBucket {
            scope: key.clone(),
            source_row_indices: vec![0],
            participant_count: 1,
            surplus_total: 4,
            deficit_total: deficit,
            net_surplus: 0,
            net_deficit: deficit.saturating_sub(4),
        }],
        stead: OwnerChannelRfSteadSurface {
            own_aggregates: vec![OwnerChannelRfOwnAggregate {
                simthing_id: source_id,
                resource_key: key.resource_key.clone(),
                surplus: 0,
                deficit,
            }],
            crossing_flows: vec![],
        },
    }
}

fn negative_error_matrix_is_typed_unrepresentable_or_sealed_without_partial_products() {
    let key = scope("negative");
    let program = AuthoredClearingProgram::new(TransformOp::set(0.0));
    let authority = ClearingRemainderAuthority {
        granter: SimThingId::from_session_raw(7),
        generation: GenerationStamp::new(10),
    };
    let one = claim(&key, 0, 5, 1.0);
    assert_eq!(
        clear_constrained_claims_at_generation(
            &[
                ConstrainedSupply {
                    scope: key.clone(),
                    available: 4,
                },
                ConstrainedSupply {
                    scope: key.clone(),
                    available: 4,
                },
            ],
            std::slice::from_ref(&one),
            &program,
            authority,
        ),
        Err(ConstrainedClearingError::DuplicateSupply)
    );
    assert_eq!(
        clear_constrained_claims_at_generation(
            &[],
            std::slice::from_ref(&one),
            &program,
            authority
        ),
        Err(ConstrainedClearingError::MissingSupply {
            source_id: source(0)
        })
    );
    assert_eq!(
        clear_constrained_claims_at_generation(
            &[ConstrainedSupply {
                scope: key.clone(),
                available: 4,
            }],
            &[one.clone(), one.clone()],
            &program,
            authority,
        ),
        Err(ConstrainedClearingError::DuplicateClaim {
            source_id: source(0)
        })
    );
    let empty = clear_constrained_claims_at_generation(
        &[ConstrainedSupply {
            scope: key.clone(),
            available: 4,
        }],
        &[],
        &program,
        authority,
    )
    .unwrap();
    assert!(empty[0].grants.is_empty());

    for invalid in [f32::NAN, f32::INFINITY, -1.0] {
        assert_eq!(
            clear_constrained_claims_at_generation(
                &[ConstrainedSupply {
                    scope: key.clone(),
                    available: 4,
                }],
                std::slice::from_ref(&one),
                &AuthoredClearingProgram::new(TransformOp::set(invalid)),
                authority,
            ),
            Err(ConstrainedClearingError::InvalidScore {
                source_id: source(0)
            })
        );
    }

    let report = reduced_report(&key, source(0), 5);
    let authored = AuthoredClaimClearingData {
        demand: demand(&key, Some(source(0).raw()), 5),
        order_weight: 1.0,
    };
    assert_eq!(
        clear_reduced_owner_channels_at_generation(
            &report,
            &[authored.clone(), authored.clone()],
            &program,
            authority,
        ),
        Err(ConstrainedClearingError::DuplicateAuthoredData {
            source_id: source(0)
        })
    );
    assert_eq!(
        clear_reduced_owner_channels_at_generation(&report, &[], &program, authority),
        Err(ConstrainedClearingError::MissingAuthoredData {
            source_id: source(0)
        })
    );
    let missing_source = AuthoredClaimClearingData {
        demand: demand(&key, None, 5),
        order_weight: 1.0,
    };
    assert_eq!(
        clear_reduced_owner_channels_at_generation(&report, &[missing_source], &program, authority,),
        Err(ConstrainedClearingError::MissingDemandSource)
    );
    let mismatched = AuthoredClaimClearingData {
        demand: demand(&key, Some(source(0).raw()), 6),
        order_weight: 1.0,
    };
    assert_eq!(
        clear_reduced_owner_channels_at_generation(&report, &[mismatched], &program, authority,),
        Err(ConstrainedClearingError::DemandDoesNotMatchReducedClaim {
            source_id: source(0)
        })
    );

    let ctx = GpuContext::new_blocking().expect("real GPU for negative parity matrix");
    let (semantic_plan, buffers) = resident_plan(&ctx, 0x1453, 10, 26, 3, 1, false);
    let duplicate_target = vec![
        ResidentApportionmentClaim::new(0, source(0), 1, 1, 0, SlotIndex::new(0), col(0)),
        ResidentApportionmentClaim::new(0, source(1), 1, 1, 0, SlotIndex::new(1), col(0)),
    ];
    assert!(matches!(
        ResidentApportionmentPlan::build(
            &semantic_plan,
            duplicate_target,
            SimThingId::from_session_raw(0),
            GenerationStamp::new(0),
            0,
        ),
        Err(ResidentApportionmentError::DuplicateLogicalClaim { .. })
            | Err(ResidentApportionmentError::DuplicateSemanticTarget { .. })
    ));
    let inconsistent = vec![
        ResidentApportionmentClaim::new(0, source(0), 1, 1, 0, SlotIndex::new(0), col(0)),
        ResidentApportionmentClaim::new(1, source(1), 1, 2, 0, SlotIndex::new(1), col(0)),
    ];
    assert!(matches!(
        ResidentApportionmentPlan::build(
            &semantic_plan,
            inconsistent,
            SimThingId::from_session_raw(0),
            GenerationStamp::new(0),
            0,
        ),
        Err(ResidentApportionmentError::InconsistentSupply { .. })
    ));

    let requests = [1, 1, 1];
    let plan = plan_resident_exact_apportionment(
        &arena_layout(),
        &semantic_plan,
        exact_claims(&semantic_plan, &requests, 0..3, |_| 2, |_| 0, |index| index),
        SimThingId::from_session_raw(0),
        GenerationStamp::new(0),
    )
    .unwrap();
    let (state, mut values) = world(ctx, 3);
    let mut session = ResidentApportionmentSession::new(&state.ctx);

    let zero_request_plan = plan_resident_exact_apportionment(
        &arena_layout(),
        &semantic_plan,
        exact_claims(
            &semantic_plan,
            &[0, 1, 1],
            0..3,
            |_| 2,
            |_| 0,
            |index| index,
        ),
        SimThingId::from_session_raw(0),
        GenerationStamp::new(0),
    )
    .unwrap();
    assert_eq!(zero_request_plan.claims().len(), 2);
    values.fill(0.0);
    set_allocated(&mut values, state.n_dims, 1, 1.0);
    set_allocated(&mut values, state.n_dims, 2, 1.0);
    state.install_resolved_values_at_boundary(&values);
    let zero_request_products = run_gpu(
        &state,
        &mut session,
        &buffers,
        &zero_request_plan,
        ResidentApportionmentDispatch::single_pass(),
    )
    .unwrap();
    assert_eq!(zero_request_products.len(), 2);
    assert!(zero_request_products
        .iter()
        .all(|product| product.source_simthing_id() != source(0)));

    for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, -1.0] {
        values.fill(0.0);
        set_allocated(&mut values, state.n_dims, 0, 1.0);
        set_allocated(&mut values, state.n_dims, 1, invalid);
        set_allocated(&mut values, state.n_dims, 2, 1.0);
        state.install_resolved_values_at_boundary(&values);
        assert!(matches!(
            execute_resident_apportionment_cpu(&plan, &values, state.n_dims),
            Err(ResidentApportionmentError::InvalidContinuousAllocation { .. })
        ));
        let gpu_invalid = run_gpu(
            &state,
            &mut session,
            &buffers,
            &plan,
            ResidentApportionmentDispatch::single_pass(),
        );
        assert!(
            matches!(
                gpu_invalid,
                Err(ResidentApportionmentError::InvalidContinuousAllocation { .. })
            ),
            "invalid allocation {invalid:?}: {gpu_invalid:?}"
        );
    }
    values.fill(0.0);
    for slot in 0..3 {
        set_allocated(&mut values, state.n_dims, slot, 1.0);
    }
    state.install_resolved_values_at_boundary(&values);
    let recovered = run_gpu(
        &state,
        &mut session,
        &buffers,
        &plan,
        ResidentApportionmentDispatch::single_pass(),
    )
    .unwrap();
    assert_eq!(
        recovered,
        execute_resident_apportionment_cpu(&plan, &values, state.n_dims).unwrap(),
        "a failed dispatch exposes no partial vector and cannot contaminate the next clear"
    );

    values.fill(0.0);
    set_allocated(&mut values, state.n_dims, 0, -0.0);
    state.install_resolved_values_at_boundary(&values);
    let signed_zero = run_gpu(
        &state,
        &mut session,
        &buffers,
        &plan,
        ResidentApportionmentDispatch::single_pass(),
    )
    .unwrap();
    set_allocated(&mut values, state.n_dims, 0, 0.0);
    state.install_resolved_values_at_boundary(&values);
    assert_eq!(
        signed_zero,
        run_gpu(
            &state,
            &mut session,
            &buffers,
            &plan,
            ResidentApportionmentDispatch::single_pass(),
        )
        .unwrap()
    );

    let overflow_plan = plan_resident_exact_apportionment(
        &arena_layout(),
        &semantic_plan,
        exact_claims(
            &semantic_plan,
            &[u32::MAX, u32::MAX],
            0..2,
            |_| 0,
            |_| 0,
            |index| index,
        ),
        SimThingId::from_session_raw(0),
        GenerationStamp::new(0),
    )
    .unwrap();
    assert!(matches!(
        execute_resident_apportionment_cpu(&overflow_plan, &values, state.n_dims),
        Err(ResidentApportionmentError::ArithmeticOverflow)
    ));

    let (foreign_plan, _) = resident_plan(&state.ctx, 0x1454, 10, 26, 3, 1, false);
    let foreign_exact = plan_resident_exact_apportionment(
        &arena_layout(),
        &foreign_plan,
        exact_claims(&foreign_plan, &requests, 0..3, |_| 2, |_| 0, |index| index),
        SimThingId::from_session_raw(0),
        GenerationStamp::new(0),
    )
    .unwrap();
    assert!(matches!(
        buffers.apportionment_buffers(&foreign_exact),
        Err(ResidentClearingGpuError::ApportionmentPlanDigestMismatch)
    ));
}

fn four_level_layout() -> ArenaTreeLayout {
    let columns = cols();
    let leaf = HierarchyNode {
        participant_slot: SlotIndex::new(3),
        hosted_simthing_id: SimThingId::from_session_raw(4),
        depth: 3,
        children: vec![],
        cols: columns,
    };
    let level_two = HierarchyNode {
        participant_slot: SlotIndex::new(2),
        hosted_simthing_id: SimThingId::from_session_raw(3),
        depth: 2,
        children: vec![leaf],
        cols: columns,
    };
    let level_one = HierarchyNode {
        participant_slot: SlotIndex::new(1),
        hosted_simthing_id: SimThingId::from_session_raw(2),
        depth: 1,
        children: vec![level_two],
        cols: columns,
    };
    build_custom_layout(
        0,
        &GpuArenaDescriptor {
            name: "resident-four-level-germ".into(),
            flow_property_id: SimPropertyId(1),
            balance_property_id: None,
            max_participants: 8,
            max_coupling_fanout: 2,
            max_orderband_depth: 16,
            fission_policy: FissionPolicy::default(),
            participant_range: (0, 0),
            wildcard_max_expansion: None,
            reserved_orderband_depth: 0,
        },
        columns,
        vec![HierarchyNode {
            participant_slot: SlotIndex::new(0),
            hosted_simthing_id: SimThingId::from_session_raw(1),
            depth: 0,
            children: vec![level_one],
            cols: columns,
        }],
    )
    .unwrap()
}

fn three_recursive_edges_self_consume_exact_ts_and_u_recurs_once_at_n_plus_one() {
    let layout = four_level_layout();
    let plan = plan_arena_allocation(&layout, &[], 32).unwrap();
    let upward_edges = plan
        .cpu_ops
        .iter()
        .filter(|op| {
            op.combine == CombineFn::Sum
                && matches!(
                    op.source,
                    SourceSpec::SlotRange { .. } | SourceSpec::ConjunctiveCrossing { .. }
                )
                && op
                    .targets
                    .iter()
                    .any(|(_, column)| *column == cols().weight_sum_col)
        })
        .count();
    assert_eq!(
        upward_edges, 3,
        "upward recursive demand/pressure exists at every edge"
    );
    let mut allocation_values = HashMap::from([
        ((SlotIndex::new(0), cols().intrinsic_flow_col), 8.0),
        ((SlotIndex::new(1), cols().weight_col), 1.0),
        ((SlotIndex::new(2), cols().weight_col), 1.0),
        ((SlotIndex::new(3), cols().weight_col), 1.0),
    ]);
    run_arena_allocation_oracle(&layout, &mut allocation_values, 1.0);
    assert_eq!(
        (1..=3)
            .map(|slot| allocation_values[&(SlotIndex::new(slot), cols().allocated_flow_col)])
            .collect::<Vec<_>>(),
        vec![8.0, 8.0, 8.0]
    );

    let ctx = GpuContext::new_blocking().expect("real GPU for recursive T_s referee");
    let (semantic_plan, buffers) = resident_plan(&ctx, 0x1455, 7, 10, 3, 3, false);
    let (state, mut values) = world(ctx, 3);
    for slot in 0..3 {
        set_allocated(&mut values, state.n_dims, slot, 8.0);
    }
    state.install_resolved_values_at_boundary(&values);
    let exact = plan_resident_exact_apportionment(
        &layout,
        &semantic_plan,
        exact_claims(
            &semantic_plan,
            &[8, 8, 8],
            0..3,
            |_| 8,
            |_| 0,
            |index| index,
        ),
        SimThingId::from_session_raw(7),
        GenerationStamp::new(10),
    )
    .unwrap();
    let mut session = ResidentApportionmentSession::new(&state.ctx);
    let settlements: Vec<ResidentSettlementOutput> = run_gpu(
        &state,
        &mut session,
        &buffers,
        &exact,
        ResidentApportionmentDispatch::single_pass(),
    )
    .unwrap();
    assert_eq!(settlements.len(), 3);
    for (edge, settlement) in settlements.into_iter().enumerate() {
        let intake: ResidentRecursiveSupplyIntake = settlement;
        assert_eq!(intake.granted(), 8, "edge {edge} exact T_s grant");
        assert_eq!(intake.unresolved(), 0);
        assert_eq!(intake.generation(), GenerationStamp::new(10));
        assert_eq!(
            intake.integration_band(),
            layout.band_layout.integration_band
        );
    }

    let n = GenerationStamp::new(10);
    let key = scope("recursive-u");
    let current_demand = demand(&key, Some(41), 10);
    let current_claim = ConstrainedClaim::from_runtime_demand(&current_demand, 1.0).unwrap();
    let next_authored = demand(&key, Some(41), 2);
    let clearing_authority = ClearingRemainderAuthority {
        granter: SimThingId::from_session_raw(7),
        generation: n,
    };
    let production_authority = RuntimeRfDemandGenerationAuthority::new(clearing_authority);
    let (current, carried) = produce_runtime_rf_next_generation_demands_for_tick(
        &production_authority,
        &[ConstrainedSupply {
            scope: key,
            available: 4,
        }],
        &[current_claim],
        &AuthoredClearingProgram::new(TransformOp::set(0.0)),
        vec![next_authored],
    )
    .unwrap();
    assert_eq!(current[0].grants[0].unresolved, 6);
    assert_eq!(carried[0].generation(), GenerationStamp::new(11));
    assert_eq!(carried[0].product().requested, 8);
    assert_eq!(
        produce_runtime_rf_next_generation_demands_for_tick(
            &production_authority,
            &[],
            &[],
            &AuthoredClearingProgram::new(TransformOp::set(0.0)),
            vec![],
        )
        .unwrap_err()
        .kind,
        RuntimeRfTickErrorKind::DemandCurrentToNextAlreadyProduced
    );
}

fn stable_hash(mut state: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        state ^= u64::from(*byte);
        state = state.wrapping_mul(0x100_0000_01b3);
    }
    state
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ResidentQualificationRecord {
    backend: String,
    adapter: String,
    vendor: u32,
    device: u32,
    device_class: String,
    driver_runtime: String,
    features: String,
    compiler: String,
    shader_compiler: String,
    cargo_lock_hash: u64,
    shader_source_hash: u64,
    workgroups: [u32; 2],
    subgroup_assumption: String,
    abi_version: u32,
}

impl ResidentQualificationRecord {
    fn capture(ctx: &GpuContext) -> Self {
        let info = ctx.adapter.get_info();
        let compiler = Command::new("rustc")
            .arg("-Vv")
            .output()
            .expect("rustc identity for exact qualification");
        assert!(compiler.status.success());
        let compiler = String::from_utf8(compiler.stdout)
            .unwrap()
            .replace("\r\n", "\n")
            .trim()
            .to_owned();
        Self {
            backend: format!("{:?}", info.backend),
            adapter: info.name,
            vendor: info.vendor,
            device: info.device,
            device_class: format!("{:?}", info.device_type),
            driver_runtime: format!("{} {}", info.driver, info.driver_info),
            features: format!("{:?}", ctx.adapter.features()),
            compiler,
            shader_compiler: "wgpu 22.1.0 / naga 22.1.0".into(),
            cargo_lock_hash: stable_hash(
                0xcbf2_9ce4_8422_2325,
                include_bytes!("../../../Cargo.lock"),
            ),
            shader_source_hash: stable_hash(
                0xcbf2_9ce4_8422_2325,
                include_bytes!(
                    "../../simthing-kernel/src/shaders/resident_clearing_apportionment.wgsl"
                ),
            ),
            workgroups: [32, 64],
            subgroup_assumption: "subgroup-independent:no-subgroup-builtins-or-size-authority"
                .into(),
            abi_version: RESIDENT_CLEARING_ABI_VERSION,
        }
    }

    fn fingerprint(&self) -> u64 {
        let mut state = 0xcbf2_9ce4_8422_2325;
        for bytes in [
            self.backend.as_bytes(),
            self.adapter.as_bytes(),
            self.device_class.as_bytes(),
            self.driver_runtime.as_bytes(),
            self.features.as_bytes(),
            self.compiler.as_bytes(),
            self.shader_compiler.as_bytes(),
            self.subgroup_assumption.as_bytes(),
        ] {
            state = stable_hash(state, &(bytes.len() as u64).to_le_bytes());
            state = stable_hash(state, bytes);
        }
        for value in [
            u64::from(self.vendor),
            u64::from(self.device),
            self.cargo_lock_hash,
            self.shader_source_hash,
            u64::from(self.workgroups[0]),
            u64::from(self.workgroups[1]),
            u64::from(self.abi_version),
        ] {
            state = stable_hash(state, &value.to_le_bytes());
        }
        state
    }
}

fn scale_multitree_physical_invariance_and_exact_qualification_hold() {
    let ctx = GpuContext::new_blocking().expect("real GPU for scale/qualification referee");
    let record = ResidentQualificationRecord::capture(&ctx);
    eprintln!("RESIDENT-CLEARING-QUALIFICATION: {record:#?}");
    eprintln!(
        "RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: {:016x}",
        record.fingerprint()
    );
    assert_eq!(record.fingerprint(), QUALIFIED_RECORD_FINGERPRINT);
    let mut mutants = Vec::new();
    macro_rules! mutate {
        ($field:ident, $value:expr) => {{
            let mut mutant = record.clone();
            mutant.$field = $value;
            mutants.push(mutant);
        }};
    }
    mutate!(backend, "ForeignBackend".into());
    mutate!(adapter, "ForeignAdapter".into());
    mutate!(vendor, record.vendor ^ 1);
    mutate!(device, record.device ^ 1);
    mutate!(device_class, "ForeignClass".into());
    mutate!(driver_runtime, format!("{}-drift", record.driver_runtime));
    mutate!(features, format!("{}-drift", record.features));
    mutate!(compiler, format!("{}-drift", record.compiler));
    mutate!(shader_compiler, "wgpu/naga-drift".into());
    mutate!(cargo_lock_hash, record.cargo_lock_hash ^ 1);
    mutate!(shader_source_hash, record.shader_source_hash ^ 1);
    mutate!(workgroups, [16, 64]);
    mutate!(subgroup_assumption, "subgroup-size-is-authority".into());
    mutate!(abi_version, record.abi_version + 1);
    assert!(mutants
        .iter()
        .all(|mutant| mutant.fingerprint() != QUALIFIED_RECORD_FINGERPRINT));

    let (semantic_plan, buffers) = resident_plan(&ctx, 0x1456, 7, 31, 128, 7, false);
    let (state, mut values) = world(ctx, 128);
    let mut session = ResidentApportionmentSession::new(&state.ctx);
    values.fill(1.0);
    state.install_resolved_values_at_boundary(&values);
    for count in [1u32, 2, 31, 32, 33, 64, 65, 127, 128] {
        let requests = vec![1u32; count as usize];
        let baseline = plan_resident_exact_apportionment(
            &arena_layout(),
            &semantic_plan,
            exact_claims(
                &semantic_plan,
                &requests,
                0..count,
                |_| 5,
                |index| index % 3,
                |index| index,
            ),
            SimThingId::from_session_raw(19),
            GenerationStamp::new(23),
        )
        .unwrap();
        let expected = run_gpu(
            &state,
            &mut session,
            &buffers,
            &baseline,
            ResidentApportionmentDispatch::new(
                ResidentApportionmentWorkgroupSize::W64,
                count.max(1),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            expected,
            execute_resident_apportionment_cpu(&baseline, &values, state.n_dims).unwrap()
        );
        let reverse = plan_resident_exact_apportionment(
            &arena_layout(),
            &semantic_plan,
            exact_claims(
                &semantic_plan,
                &requests,
                (0..count).rev(),
                |_| 5,
                |index| index % 3,
                |index| (index * 17) % 128,
            ),
            SimThingId::from_session_raw(19),
            GenerationStamp::new(23),
        )
        .unwrap();
        assert_eq!(
            run_gpu(
                &state,
                &mut session,
                &buffers,
                &reverse,
                ResidentApportionmentDispatch::new(ResidentApportionmentWorkgroupSize::W32, 7,)
                    .unwrap(),
            )
            .unwrap(),
            expected,
            "count {count}: scope/equality/physical/workgroup/partition order"
        );
    }

    let boundary_requests = [u32::MAX, u32::MAX];
    let boundary_plan = plan_resident_exact_apportionment(
        &arena_layout(),
        &semantic_plan,
        exact_claims(
            &semantic_plan,
            &boundary_requests,
            0..2,
            |_| u32::MAX,
            |_| 0,
            |index| index,
        ),
        SimThingId::from_session_raw(u32::MAX),
        GenerationStamp::new(u32::MAX),
    )
    .unwrap();
    for pair in [
        [f32::from_bits(1), f32::from_bits(3)],
        [f32::from_bits(0x007f_ffff), f32::from_bits(0x0080_0000)],
        [1.0, f32::MAX],
    ] {
        set_allocated(&mut values, state.n_dims, 0, pair[0]);
        set_allocated(&mut values, state.n_dims, 1, pair[1]);
        state.install_resolved_values_at_boundary(&values);
        assert_eq!(
            run_gpu(
                &state,
                &mut session,
                &buffers,
                &boundary_plan,
                ResidentApportionmentDispatch::new(ResidentApportionmentWorkgroupSize::W32, 1,)
                    .unwrap(),
            )
            .unwrap(),
            execute_resident_apportionment_cpu(&boundary_plan, &values, state.n_dims).unwrap()
        );
    }

    // Two independent devices/queues stand in for independently schedulable
    // trees. Their plans/buffers have divergent generations and realms but use
    // overlapping raw root and claimant ids through ordinary per-tree doors.
    let ctx_a = GpuContext::new_blocking().unwrap();
    let ctx_b = GpuContext::new_blocking().unwrap();
    let (plan_a, buffers_a) = resident_plan(&ctx_a, 0xaaa, 7, 3, 2, 1, false);
    let (plan_b, buffers_b) = resident_plan(&ctx_b, 0xbbb, 7, 97, 2, 1, true);
    assert_ne!(buffers_a.owner().realm(), buffers_b.owner().realm());
    assert_ne!(
        buffers_a.owner().generation(),
        buffers_b.owner().generation()
    );
    assert_ne!(plan_a.digest(), plan_b.digest());
    let (state_a, mut values_a) = world(ctx_a, 2);
    let (state_b, mut values_b) = world(ctx_b, 2);
    for slot in 0..2 {
        set_allocated(&mut values_a, state_a.n_dims, slot, 1.0);
        set_allocated(&mut values_b, state_b.n_dims, slot, 1.0);
    }
    state_a.install_resolved_values_at_boundary(&values_a);
    state_b.install_resolved_values_at_boundary(&values_b);
    let exact_a = plan_resident_exact_apportionment(
        &arena_layout(),
        &plan_a,
        exact_claims(&plan_a, &[1, 1], 0..2, |_| 1, |_| 0, |index| index),
        SimThingId::from_session_raw(7),
        GenerationStamp::new(3),
    )
    .unwrap();
    let exact_b = plan_resident_exact_apportionment(
        &arena_layout(),
        &plan_b,
        exact_claims(&plan_b, &[1, 1], (0..2).rev(), |_| 1, |_| 0, |index| index),
        SimThingId::from_session_raw(7),
        GenerationStamp::new(97),
    )
    .unwrap();
    let mut session_b = ResidentApportionmentSession::new(&state_b.ctx);
    let mut session_a = ResidentApportionmentSession::new(&state_a.ctx);
    let b_first = run_gpu(
        &state_b,
        &mut session_b,
        &buffers_b,
        &exact_b,
        ResidentApportionmentDispatch::new(ResidentApportionmentWorkgroupSize::W32, 1).unwrap(),
    )
    .unwrap();
    let a_second = run_gpu(
        &state_a,
        &mut session_a,
        &buffers_a,
        &exact_a,
        ResidentApportionmentDispatch::new(ResidentApportionmentWorkgroupSize::W64, 2).unwrap(),
    )
    .unwrap();
    assert_eq!(
        b_first,
        execute_resident_apportionment_cpu(&exact_b, &values_b, state_b.n_dims).unwrap()
    );
    assert_eq!(
        a_second,
        execute_resident_apportionment_cpu(&exact_a, &values_a, state_a.n_dims).unwrap()
    );
    assert_eq!(
        product_map(&a_second).keys().copied().collect::<Vec<_>>(),
        product_map(&b_first).keys().copied().collect::<Vec<_>>(),
        "raw claimant ids overlap while realm/buffer/generation authority remains independent"
    );
}
