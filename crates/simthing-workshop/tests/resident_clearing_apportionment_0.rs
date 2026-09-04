//! RESIDENT-CLEARING-APPORTIONMENT-0 exact-law and physical-order witnesses.

use std::collections::BTreeMap;
use std::mem::{align_of, size_of};

use simthing_core::owner_channel::OwnerRef;
use simthing_core::{
    ColumnIndex, DimensionRegistry, ExecutionIncarnation, GenerationStamp, IntegrationSchedule,
    SimProperty, SimPropertyId, SimThing, SimThingId, SlotIndex, TransformOp,
    TreeExecutionAuthority, TreeGenerationAuthority, TreeRealmId,
};
use simthing_driver::{
    build_custom_layout, plan_resident_exact_apportionment, ArenaTreeLayout, FissionPolicy,
    GpuArenaDescriptor, HierarchyNode, NodeColumnRefs,
};
use simthing_gpu::{
    wgpu, GpuContext, ResidentApportionmentDispatch, ResidentApportionmentSession,
    ResidentApportionmentWorkgroupSize, ResidentClearingBuffers, WorldGpuState,
};
use simthing_kernel::{
    execute_resident_apportionment_cpu, ResidentApportionmentClaim, ResidentApportionmentError,
    ResidentApportionmentPlan, ResidentClearingAdmission, ResidentClearingBudgets,
    ResidentClearingPlan, ResidentConstrainedProduct, ResidentDrawId, ResidentExactBasisIdentity,
    ResidentOwnerId, ResidentRecursiveSupplyIntake, ResidentResourceId, ResidentScopeId,
    ResidentSettlementOutput, RESIDENT_APPORTIONMENT_SCRATCH_BYTES_PER_ROW,
};
use simthing_spec::{
    clear_constrained_claims_at_generation, AuthoredClearingProgram, ClearingRemainderAuthority,
    ConstrainedClaim, ConstrainedClearingError, ConstrainedSupply, OwnerChannelScopeKey,
    ResourceKey, RuntimeOwnerSiloDemandBucket, ScopeId,
};

fn col(raw: usize) -> ColumnIndex {
    ColumnIndex::from_raw_for_oracle_or_rehearsal(raw)
}

fn loaded_tree() -> SimThing {
    serde_json::from_str(
        r#"{
            "id": 7,
            "kind": "GameSession",
            "properties": [],
            "resource_parent_edges": [],
            "overlays": [],
            "children": [],
            "spawned_generation": 4
        }"#,
    )
    .expect("persisted resident fixture")
}

fn budgets() -> ResidentClearingBudgets {
    ResidentClearingBudgets::new(4, 4, 4, 128, 128, 65_536, 262_144, 8_192, 64)
        .expect("14.2-admitted 64-byte exact scratch rows")
}

fn resident_plan(
    ctx: &GpuContext,
    count: u32,
    alternating_scopes: bool,
    reverse_admission: bool,
) -> (ResidentClearingPlan, ResidentClearingBuffers) {
    let tree = loaded_tree();
    let realm = TreeRealmId::from_u128(0x1440).unwrap();
    let generation = TreeGenerationAuthority::new(GenerationStamp::new(4));
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
            scope: ResidentScopeId::new(if alternating_scopes {
                10 + u64::from(index % 2)
            } else {
                10
            }),
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
    available: u32,
    order: impl IntoIterator<Item = u32>,
    rebound_slots: bool,
) -> Vec<ResidentApportionmentClaim> {
    exact_claims_with_identity(
        semantic_plan,
        requests,
        available,
        order,
        rebound_slots,
        |_| ResidentExactBasisIdentity::LiveAllocatedFlow,
    )
}

fn exact_claims_with_identity(
    semantic_plan: &ResidentClearingPlan,
    requests: &[u32],
    available: u32,
    order: impl IntoIterator<Item = u32>,
    rebound_slots: bool,
    identity: impl Fn(u32) -> ResidentExactBasisIdentity,
) -> Vec<ResidentApportionmentClaim> {
    order
        .into_iter()
        .map(|index| {
            let slot = if rebound_slots {
                (index * 17) % requests.len() as u32
            } else {
                index
            };
            ResidentApportionmentClaim::new(
                semantic_row_for_draw(semantic_plan, u64::from(1_000 + index)),
                source(index),
                requests[index as usize],
                available,
                0,
                SlotIndex::new(slot),
                col(0),
                identity(index),
            )
        })
        .collect()
}

fn arena_layout() -> ArenaTreeLayout {
    let cols = NodeColumnRefs {
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
    };
    build_custom_layout(
        0,
        &GpuArenaDescriptor {
            name: "resident-exact-terminal".into(),
            flow_property_id: SimPropertyId(1),
            balance_property_id: None,
            max_participants: 128,
            max_coupling_fanout: 4,
            max_orderband_depth: 16,
            fission_policy: FissionPolicy::default(),
            participant_range: (0, 0),
            wildcard_max_expansion: None,
            reserved_orderband_depth: 0,
        },
        cols,
        vec![HierarchyNode {
            participant_slot: SlotIndex::new(0),
            hosted_simthing_id: SimThingId::from_session_raw(7),
            depth: 0,
            children: vec![],
            cols,
        }],
    )
    .unwrap()
}

fn world(ctx: GpuContext, n_slots: u32) -> (WorldGpuState, Vec<f32>) {
    let mut registry = DimensionRegistry::new();
    registry.register(SimProperty::simple("resident", "allocated_flow", 1));
    let state = WorldGpuState::new(ctx, &registry, n_slots.max(1));
    let values = vec![1.0; state.values_len()];
    state.install_resolved_values_at_boundary(&values);
    (state, values)
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
            label: Some("resident_exact_terminal_band_witness"),
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

fn product_map(products: &[ResidentConstrainedProduct]) -> BTreeMap<SimThingId, (u32, u32)> {
    products
        .iter()
        .map(|product| {
            (
                product.source_simthing_id(),
                (product.granted(), product.unresolved()),
            )
        })
        .collect()
}

fn oracle_scope(index: u32) -> OwnerChannelScopeKey {
    OwnerChannelScopeKey {
        owner_ref: OwnerRef::new("resident-exact"),
        resource_key: ResourceKey::new("quanta"),
        scope_id: ScopeId::from_boundary(SimThingId::from_session_raw(70 + index)),
    }
}

fn oracle_claim(scope: &OwnerChannelScopeKey, index: u32, requested: u32) -> ConstrainedClaim {
    ConstrainedClaim::from_runtime_demand(
        &RuntimeOwnerSiloDemandBucket {
            owner_ref: scope.owner_ref.clone(),
            resource_key: scope.resource_key.clone(),
            scope_id: scope.scope_id.clone(),
            requested,
            priority: 0,
            source_simthing_id_raw: Some(source(index).raw()),
        },
        1.0,
    )
    .unwrap()
}

fn oracle(
    requests: &[u32],
    available: u32,
    generation: GenerationStamp,
    granter: SimThingId,
) -> Result<BTreeMap<SimThingId, (u32, u32)>, ConstrainedClearingError> {
    let scope = oracle_scope(0);
    let claims: Vec<_> = requests
        .iter()
        .enumerate()
        .map(|(index, &requested)| oracle_claim(&scope, index as u32, requested))
        .collect();
    let results = clear_constrained_claims_at_generation(
        &[ConstrainedSupply { scope, available }],
        &claims,
        &AuthoredClearingProgram::new(TransformOp::set(0.0)),
        ClearingRemainderAuthority {
            granter,
            generation,
        },
    )?;
    Ok(results[0]
        .grants
        .iter()
        .map(|grant| (grant.source_simthing_id, (grant.granted, grant.unresolved)))
        .collect())
}

#[test]
fn e5_integer_exact_neutral_basis_preserves_the_u32_winner() {
    let ctx = GpuContext::new_blocking().expect("real GPU for 15.6 E5 referee");
    let (semantic_plan, buffers) = resident_plan(&ctx, 2, false, false);
    let (state, mut values) = world(ctx, 2);
    let mut session = ResidentApportionmentSession::new(&state.ctx);
    let requests = [16_777_217, 16_777_216];
    let available = 1;
    let granter = SimThingId::from_session_raw(0);
    let generation = GenerationStamp::new(1);

    values.fill(0.0);
    for (slot, &requested) in requests.iter().enumerate() {
        values[slot * state.n_dims as usize] = requested as f32;
    }
    assert_eq!(
        values[0].to_bits(),
        values[state.n_dims as usize].to_bits(),
        "the pre-fix binary32 collapse remains mechanically represented"
    );
    state.install_resolved_values_at_boundary(&values);
    let neutral_plan = plan_resident_exact_apportionment(
        &arena_layout(),
        &semantic_plan,
        exact_claims_with_identity(&semantic_plan, &requests, available, 0..2, false, |_| {
            ResidentExactBasisIdentity::NeutralRequest
        }),
        granter,
        generation,
    )
    .unwrap();
    let exact = oracle(&requests, available, generation, granter).unwrap();
    let legacy_float_tie =
        BTreeMap::from([(source(0), (0, 16_777_217)), (source(1), (1, 16_777_215))]);
    assert_ne!(legacy_float_tie, exact, "the planted pre-fix path is RED");

    let neutral_cpu = product_map(
        &execute_resident_apportionment_cpu(&neutral_plan, &values, state.n_dims).unwrap(),
    );
    let neutral_gpu = product_map(
        &run_gpu(
            &state,
            &mut session,
            &buffers,
            &neutral_plan,
            ResidentApportionmentDispatch::single_pass(),
        )
        .unwrap(),
    );
    assert_eq!(
        neutral_cpu, exact,
        "CPU resident mirror recovers the carried exact neutral request"
    );
    assert_eq!(
        neutral_gpu, exact,
        "resident GPU recovers the carried exact neutral request"
    );

    // Remand 5540959678: these same binary32 cells can instead be the genuine
    // result of a non-neutral policy allocating one less than source 0 asked.
    // The carried live identity must preserve that case without fabricating
    // the request unit lost before the policy result reached Q.
    let below_cap_plan = plan_resident_exact_apportionment(
        &arena_layout(),
        &semantic_plan,
        exact_claims(&semantic_plan, &requests, available, 0..2, false),
        granter,
        generation,
    )
    .unwrap();
    let below_cap_expected = legacy_float_tie;
    let below_cap_cpu = product_map(
        &execute_resident_apportionment_cpu(&below_cap_plan, &values, state.n_dims).unwrap(),
    );
    let below_cap_gpu = product_map(
        &run_gpu(
            &state,
            &mut session,
            &buffers,
            &below_cap_plan,
            ResidentApportionmentDispatch::single_pass(),
        )
        .unwrap(),
    );
    assert_eq!(
        below_cap_cpu, below_cap_expected,
        "a genuinely below-request live policy result must not fabricate the missing unit"
    );
    assert_eq!(
        below_cap_gpu, below_cap_expected,
        "resident GPU must preserve a genuinely below-request live allocation"
    );

    // A genuinely above-request policy result uses the same live identity;
    // the exact Q149 comparison caps it at the exact admitted integer request.
    values[0] = 33_554_432.0;
    state.install_resolved_values_at_boundary(&values);
    let above_cap_cpu = product_map(
        &execute_resident_apportionment_cpu(&below_cap_plan, &values, state.n_dims).unwrap(),
    );
    let above_cap_gpu = product_map(
        &run_gpu(
            &state,
            &mut session,
            &buffers,
            &below_cap_plan,
            ResidentApportionmentDispatch::single_pass(),
        )
        .unwrap(),
    );
    assert_eq!(above_cap_cpu, exact, "CPU exact cap retains the u32 winner");
    assert_eq!(above_cap_gpu, exact, "GPU exact cap retains the u32 winner");
}

#[test]
fn neutral_continuous_shares_match_frozen_cpu_law_across_boundary_cases() {
    let ctx = GpuContext::new_blocking().expect("real GPU for resident exact witness");
    let (semantic_plan, buffers) = resident_plan(&ctx, 3, false, false);
    let (state, mut values) = world(ctx, 3);
    let mut session = ResidentApportionmentSession::new(&state.ctx);
    let layout = arena_layout();
    let granter = SimThingId::from_session_raw(u32::MAX);
    let generation = GenerationStamp::new(u32::MAX);
    let cases: &[(&[u32], u32, &str)] = &[
        (&[100, 200, 300], 100, "canonical-17-33-50"),
        (&[17, 33, 50], 0, "zero-supply"),
        (&[17, 33, 50], 100, "full-supply"),
        (&[17, 33, 50], 7, "short-supply"),
        (&[u32::MAX], u32::MAX, "u32-max-square-numerator"),
        (&[0, 5], 3, "zero-request-omission"),
    ];

    for &(requests, available, label) in cases {
        values.fill(0.0);
        for (slot, &requested) in requests.iter().enumerate() {
            values[slot * state.n_dims as usize] = requested as f32;
        }
        state.install_resolved_values_at_boundary(&values);
        let claims = exact_claims(
            &semantic_plan,
            requests,
            available,
            0..requests.len() as u32,
            false,
        );
        let plan =
            plan_resident_exact_apportionment(&layout, &semantic_plan, claims, granter, generation)
                .unwrap();
        assert_eq!(plan.integration_band(), layout.band_layout.integration_band);
        assert_eq!(
            layout.band_layout.integration_band + 1,
            layout.band_layout.total_bands_used,
            "exact product is stamped at the one terminal resident RF band"
        );
        let cpu_mirror = execute_resident_apportionment_cpu(&plan, &values, state.n_dims).unwrap();
        let gpu = run_gpu(
            &state,
            &mut session,
            &buffers,
            &plan,
            ResidentApportionmentDispatch::single_pass(),
        )
        .unwrap();
        let authority = oracle(requests, available, generation, granter).unwrap();
        assert_eq!(product_map(&cpu_mirror), authority, "CPU mirror: {label}");
        assert_eq!(product_map(&gpu), authority, "resident GPU: {label}");
        assert!(gpu
            .iter()
            .all(|product| product.integration_band() == layout.band_layout.integration_band));
    }

    let canonical = oracle(&[100, 200, 300], 100, generation, granter).unwrap();
    assert_eq!(canonical[&source(0)].0, 17);
    assert_eq!(canonical[&source(1)].0, 33);
    assert_eq!(canonical[&source(2)].0, 50);

    let mixed_requests = [100, 1, 9];
    let mixed_precedences = [0, 0, 1];
    values.fill(0.0);
    values[state.n_dims as usize] = 1.0;
    values[2 * state.n_dims as usize] = 9.0;
    state.install_resolved_values_at_boundary(&values);
    let mixed_claims = (0..3)
        .map(|index| {
            ResidentApportionmentClaim::new(
                semantic_row_for_draw(&semantic_plan, u64::from(1_000 + index)),
                source(index),
                mixed_requests[index as usize],
                10,
                mixed_precedences[index as usize],
                SlotIndex::new(index),
                col(0),
                ResidentExactBasisIdentity::LiveAllocatedFlow,
            )
        })
        .collect();
    let mixed_plan = plan_resident_exact_apportionment(
        &layout,
        &semantic_plan,
        mixed_claims,
        granter,
        generation,
    )
    .unwrap();
    let mixed_cpu = execute_resident_apportionment_cpu(&mixed_plan, &values, state.n_dims).unwrap();
    let mixed_gpu = run_gpu(
        &state,
        &mut session,
        &buffers,
        &mixed_plan,
        ResidentApportionmentDispatch::single_pass(),
    )
    .unwrap();
    assert_eq!(mixed_gpu, mixed_cpu);
    let mixed = product_map(&mixed_gpu);
    assert_eq!(mixed[&source(0)], (0, 100));
    assert_eq!(mixed[&source(1)], (1, 0));
    assert_eq!(mixed[&source(2)], (9, 0));

    let sealed_input_plan = plan_resident_exact_apportionment(
        &layout,
        &semantic_plan,
        exact_claims(&semantic_plan, &[1], 1, 0..1, false),
        granter,
        generation,
    )
    .unwrap();
    let mut invalid_live_allocation = values.clone();
    invalid_live_allocation[0] = f32::NAN;
    state.install_resolved_values_at_boundary(&invalid_live_allocation);
    let invalid_result = run_gpu(
        &state,
        &mut session,
        &buffers,
        &sealed_input_plan,
        ResidentApportionmentDispatch::single_pass(),
    );
    assert!(
        matches!(
        &invalid_result,
        Err(ResidentApportionmentError::InvalidContinuousAllocation { source_id })
            if *source_id == source(0)
        ),
        "sealed live allocation refusal: {invalid_result:?}"
    );
}

#[test]
fn finite_allocated_flow_proportions_are_the_exact_numerator_basis() {
    let ctx = GpuContext::new_blocking().expect("real GPU for continuous-authority falsifier");
    let (semantic_plan, buffers) = resident_plan(&ctx, 2, false, false);
    let (state, mut values) = world(ctx, 2);
    let mut session = ResidentApportionmentSession::new(&state.ctx);
    let plan = plan_resident_exact_apportionment(
        &arena_layout(),
        &semantic_plan,
        exact_claims(&semantic_plan, &[100, 100], 8, 0..2, false),
        SimThingId::from_session_raw(0),
        GenerationStamp::new(0),
    )
    .unwrap();

    let mut outcomes = Vec::new();
    for shares in [
        [0.75, 0.25],
        [0.25, 0.75],
        [f32::from_bits(1), f32::from_bits(3)],
    ] {
        values[0] = shares[0];
        values[state.n_dims as usize] = shares[1];
        state.install_resolved_values_at_boundary(&values);
        let cpu = execute_resident_apportionment_cpu(&plan, &values, state.n_dims).unwrap();
        let gpu = run_gpu(
            &state,
            &mut session,
            &buffers,
            &plan,
            ResidentApportionmentDispatch::single_pass(),
        )
        .unwrap();
        assert_eq!(gpu, cpu);
        outcomes.push(product_map(&gpu));
    }

    assert_eq!(outcomes[0][&source(0)], (6, 94));
    assert_eq!(outcomes[0][&source(1)], (2, 98));
    assert_eq!(outcomes[1][&source(0)], (2, 98));
    assert_eq!(outcomes[1][&source(1)], (6, 94));
    assert_eq!(outcomes[2][&source(0)], (2, 98));
    assert_eq!(outcomes[2][&source(1)], (6, 94));
    assert_ne!(
        outcomes[0], outcomes[1],
        "fixed requests/supply/precedence must not mask changed AllocatedFlow proportions"
    );
}

#[test]
fn continuous_basis_obeys_request_caps_and_zero_routes_full_unresolved_u() {
    let ctx = GpuContext::new_blocking().expect("real GPU for cap/zero falsifier");
    let (semantic_plan, buffers) = resident_plan(&ctx, 2, false, false);
    let (state, mut values) = world(ctx, 2);
    let mut session = ResidentApportionmentSession::new(&state.ctx);
    let plan = plan_resident_exact_apportionment(
        &arena_layout(),
        &semantic_plan,
        exact_claims(&semantic_plan, &[5, 5], 5, 0..2, false),
        SimThingId::from_session_raw(0),
        GenerationStamp::new(0),
    )
    .unwrap();

    values[0] = f32::MAX;
    values[state.n_dims as usize] = 0.0;
    state.install_resolved_values_at_boundary(&values);
    let cpu = execute_resident_apportionment_cpu(&plan, &values, state.n_dims).unwrap();
    let gpu = run_gpu(
        &state,
        &mut session,
        &buffers,
        &plan,
        ResidentApportionmentDispatch::single_pass(),
    )
    .unwrap();
    assert_eq!(gpu, cpu);
    let capped = product_map(&gpu);
    assert_eq!(capped[&source(0)], (5, 0));
    assert_eq!(capped[&source(1)], (0, 5));

    values.fill(0.0);
    state.install_resolved_values_at_boundary(&values);
    let cpu = execute_resident_apportionment_cpu(&plan, &values, state.n_dims).unwrap();
    let gpu = run_gpu(
        &state,
        &mut session,
        &buffers,
        &plan,
        ResidentApportionmentDispatch::single_pass(),
    )
    .unwrap();
    assert_eq!(gpu, cpu);
    assert_eq!(product_map(&gpu)[&source(0)], (0, 5));
    assert_eq!(product_map(&gpu)[&source(1)], (0, 5));
}

#[test]
fn exact_ties_rotate_by_generation_and_unresolved_overflow_refuses() {
    let ctx = GpuContext::new_blocking().expect("real GPU for resident tie witness");
    let (semantic_plan, buffers) = resident_plan(&ctx, 2, false, false);
    let (state, values) = world(ctx, 2);
    let mut session = ResidentApportionmentSession::new(&state.ctx);
    let layout = arena_layout();
    let granter = SimThingId::from_session_raw(0);
    let mut generations = Vec::new();
    for generation_raw in [0, 1] {
        let generation = GenerationStamp::new(generation_raw);
        let plan = plan_resident_exact_apportionment(
            &layout,
            &semantic_plan,
            exact_claims(&semantic_plan, &[1, 1], 1, 0..2, false),
            granter,
            generation,
        )
        .unwrap();
        let cpu = execute_resident_apportionment_cpu(&plan, &values, state.n_dims).unwrap();
        let gpu = run_gpu(
            &state,
            &mut session,
            &buffers,
            &plan,
            ResidentApportionmentDispatch::single_pass(),
        )
        .unwrap();
        let authority = oracle(&[1, 1], 1, generation, granter).unwrap();
        assert_eq!(product_map(&cpu), authority);
        assert_eq!(product_map(&gpu), authority);
        generations.push(authority);
    }
    assert_ne!(generations[0], generations[1]);

    let overflow_plan = plan_resident_exact_apportionment(
        &layout,
        &semantic_plan,
        exact_claims(&semantic_plan, &[u32::MAX, u32::MAX], 0, 0..2, false),
        granter,
        GenerationStamp::new(2),
    )
    .unwrap();
    assert!(matches!(
        oracle(&[u32::MAX, u32::MAX], 0, GenerationStamp::new(2), granter),
        Err(ConstrainedClearingError::ArithmeticOverflow)
    ));
    assert!(matches!(
        execute_resident_apportionment_cpu(&overflow_plan, &values, state.n_dims),
        Err(ResidentApportionmentError::ArithmeticOverflow)
    ));
    assert!(matches!(
        run_gpu(
            &state,
            &mut session,
            &buffers,
            &overflow_plan,
            ResidentApportionmentDispatch::single_pass(),
        ),
        Err(ResidentApportionmentError::ArithmeticOverflow)
    ));
}

#[test]
fn physical_upload_scope_slot_workgroup_and_partition_shapes_are_invariant() {
    const COUNT: u32 = 65;
    let ctx = GpuContext::new_blocking().expect("real GPU for resident invariance witness");
    let (semantic_plan, buffers) = resident_plan(&ctx, COUNT, true, false);
    let (permuted_semantic_plan, _) = resident_plan(&ctx, COUNT, true, true);
    assert_eq!(semantic_plan.digest(), permuted_semantic_plan.digest());
    assert_eq!(
        semantic_plan.canonical_bytes(),
        permuted_semantic_plan.canonical_bytes(),
        "scope/admission storage order canonicalizes before exact settlement"
    );
    let (state, values) = world(ctx, COUNT);
    let mut session = ResidentApportionmentSession::new(&state.ctx);
    let layout = arena_layout();
    let requests = vec![1; COUNT as usize];
    let granter = SimThingId::from_session_raw(19);
    let generation = GenerationStamp::new(23);

    let baseline_plan = plan_resident_exact_apportionment(
        &layout,
        &semantic_plan,
        exact_claims(&semantic_plan, &requests, 9, 0..COUNT, false),
        granter,
        generation,
    )
    .unwrap();
    let baseline_cpu = execute_resident_apportionment_cpu(&baseline_plan, &values, state.n_dims)
        .expect("canonical CPU mirror");
    let baseline = run_gpu(
        &state,
        &mut session,
        &buffers,
        &baseline_plan,
        ResidentApportionmentDispatch::new(ResidentApportionmentWorkgroupSize::W64, COUNT).unwrap(),
    )
    .unwrap();
    assert_eq!(baseline, baseline_cpu);

    let reverse_order: Vec<_> = (0..COUNT).rev().collect();
    let perturbed_plan = plan_resident_exact_apportionment(
        &layout,
        &permuted_semantic_plan,
        exact_claims(
            &permuted_semantic_plan,
            &requests,
            9,
            reverse_order.iter().copied(),
            true,
        ),
        granter,
        generation,
    )
    .unwrap();
    let perturbed = run_gpu(
        &state,
        &mut session,
        &buffers,
        &perturbed_plan,
        ResidentApportionmentDispatch::new(ResidentApportionmentWorkgroupSize::W32, 7).unwrap(),
    )
    .unwrap();
    assert_eq!(
        baseline, perturbed,
        "claim upload, canonical scope storage, epoch-rebound slots, W32/W64, and 7-row dispatch partitions are non-semantic"
    );

    let arrival_mutant = |order: &[u32]| -> BTreeMap<u32, u32> {
        let mut remaining = [9u32, 9u32];
        order
            .iter()
            .copied()
            .map(|index| {
                let scope = (index % 2) as usize;
                let grant = u32::from(remaining[scope] != 0);
                remaining[scope] = remaining[scope].saturating_sub(grant);
                (source(index).raw(), grant)
            })
            .collect()
    };
    let ascending: Vec<_> = (0..COUNT).collect();
    assert_ne!(
        arrival_mutant(&ascending),
        arrival_mutant(&reverse_order),
        "planted atomic/arrival-order remainder authority must RED"
    );
}

#[test]
fn canonical_product_is_recursive_intake_without_adapter_or_legacy_bridge() {
    fn recursive_intake(_: Option<ResidentRecursiveSupplyIntake>) {}
    let settlement: Option<ResidentSettlementOutput> = None;
    recursive_intake(settlement);
    assert_eq!(
        size_of::<ResidentSettlementOutput>(),
        size_of::<ResidentRecursiveSupplyIntake>()
    );
    assert_eq!(
        align_of::<ResidentSettlementOutput>(),
        align_of::<ResidentRecursiveSupplyIntake>()
    );
    assert_eq!(size_of::<ResidentConstrainedProduct>(), 32);
    assert_eq!(RESIDENT_APPORTIONMENT_SCRATCH_BYTES_PER_ROW, 64);

    let exact_source = include_str!("../../simthing-kernel/src/resident_clearing_apportionment.rs");
    let shader =
        include_str!("../../simthing-kernel/src/shaders/resident_clearing_apportionment.wgsl");
    let legacy_market = include_str!("../../simthing-spec/src/spec/flow_market.rs");
    assert!(!exact_source.contains("from_cleared_offering"));
    assert!(!shader.contains("from_cleared_offering"));
    assert!(legacy_market.contains("    fn from_cleared_offering("));
    assert!(!legacy_market.contains("    pub fn from_cleared_offering("));
    assert!(shader.contains("fn wide_mul_u32"));
    assert!(shader.contains("fn wide_divmod"));
    assert!(shader.contains("fn exact_capped_basis"));
    assert!(!exact_source.contains("requested as f32"));
    assert!(!shader.contains("f32(requested)"));
    assert!(shader.contains("fn exact_divmod"));
    assert!(exact_source.contains("EXACT_BASIS_FRACTION_BITS: u32 = 149"));
    assert!(shader.contains("vec2<u32>"));
    assert!(shader.contains("@workgroup_size(32)"));
    assert!(shader.contains("@workgroup_size(64)"));
    assert!(!shader.to_ascii_lowercase().contains("atomic"));
}
