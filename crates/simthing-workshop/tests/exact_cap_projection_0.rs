//! 15.9 Owner falsifier, planted before any production apportionment edit.
use simthing_core::{
    ColumnIndex, DimensionRegistry, ExecutionIncarnation, GenerationStamp, IntegrationSchedule,
    SimProperty, SimThing, SimThingId, SimThingKind, SlotIndex, TreeExecutionAuthority,
    TreeGenerationAuthority, TreeRealmId,
};
use simthing_gpu::{
    wgpu, GpuContext, ResidentApportionmentDispatch, ResidentApportionmentSession,
    ResidentApportionmentWorkgroupSize, ResidentClearingBuffers, WorldGpuState,
};
use simthing_kernel::{
    execute_resident_apportionment_cpu, ResidentApportionmentClaim, ResidentApportionmentPlan,
    ResidentClearingAdmission, ResidentClearingBudgets, ResidentClearingPlan,
    ResidentConstrainedProduct, ResidentDrawId, ResidentExactBasisIdentity, ResidentOwnerId,
    ResidentResourceId, ResidentScopeId,
};

#[test]
fn owner_cap_collision_saturates_and_redistributes_on_cpu_and_gpu() {
    let ctx = GpuContext::new_blocking().expect("real GPU for the 15.9 Owner falsifier");
    let tree = SimThing::new(SimThingKind::GameSession, 4);
    let realm = TreeRealmId::from_u128(0x1590).unwrap();
    let generation = TreeGenerationAuthority::new(GenerationStamp::new(4));
    let schedule = IntegrationSchedule::new();
    let mut registry = DimensionRegistry::new();
    registry.register(SimProperty::simple("cap-referee", "allocated-flow", 1));
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
    let semantic_plan = ResidentClearingPlan::build(
        &binding,
        (0..2).map(|index| ResidentClearingAdmission {
            owner,
            resource: ResidentResourceId::new(1),
            scope: ResidentScopeId::new(10),
            draw: ResidentDrawId::new(1_000 + index),
        }),
        ResidentClearingBudgets::new(4, 4, 4, 128, 128, 65_536, 262_144, 8_192, 64).unwrap(),
    )
    .unwrap();
    let buffers = ResidentClearingBuffers::allocate(&ctx.device, &binding, &semantic_plan).unwrap();
    let requests = [1, 100];
    let claims = requests
        .iter()
        .enumerate()
        .map(|(index, &requested)| {
            let semantic_row = semantic_plan
                .rows()
                .iter()
                .position(|row| {
                    semantic_plan.dictionaries().draws()[row.draw().get() as usize].get()
                        == 1_000 + index as u64
                })
                .unwrap();
            ResidentApportionmentClaim::new(
                semantic_row as u32,
                SimThingId::from_session_raw(1_000 + index as u32),
                requested,
                101,
                0,
                SlotIndex::new(index as u32),
                ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
                ResidentExactBasisIdentity::LiveAllocatedFlow,
            )
        })
        .collect();
    let plan = ResidentApportionmentPlan::build(
        &semantic_plan,
        claims,
        tree.id,
        GenerationStamp::new(4),
        0,
    )
    .unwrap();
    let state = WorldGpuState::new(ctx, &registry, 2);
    let values = vec![1.0; state.values_len()];
    state.install_resolved_values_at_boundary(&values);
    let cpu = execute_resident_apportionment_cpu(&plan, &values, state.n_dims);
    println!("15.9 Owner RED/ GREEN referee: requests=[1,100] bases=[1,1] S=101 CPU={cpu:?}");

    let mut executor = ResidentApportionmentSession::new(&state.ctx);
    let (semantic_rows, scratch) = buffers.apportionment_buffers(&plan).unwrap();
    let mut gpu_results = Vec::new();
    for workgroup in [
        ResidentApportionmentWorkgroupSize::W32,
        ResidentApportionmentWorkgroupSize::W64,
    ] {
        for rows_per_dispatch in [1, u32::MAX] {
            let dispatch =
                ResidentApportionmentDispatch::new(workgroup, rows_per_dispatch).unwrap();
            let mut encoder =
                state
                    .ctx
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("15.9 Owner cap-collision falsifier"),
                    });
            state
                .encode_resident_apportionment_with_dispatch_into(
                    &mut executor,
                    &mut encoder,
                    semantic_rows,
                    scratch,
                    &plan,
                    dispatch,
                )
                .unwrap();
            state.ctx.queue.submit(Some(encoder.finish()));
            let _ = state.ctx.device.poll(wgpu::Maintain::Wait);
            let gpu = executor.readback_products(&state.ctx, scratch, &plan);
            println!("15.9 Owner GPU workgroup={workgroup:?} rows_per_dispatch={rows_per_dispatch}: {gpu:?}");
            gpu_results.push(gpu);
        }
    }
    let cpu = cpu.expect("feasible cap collision must saturate and redistribute to (1,100)");
    assert_eq!(
        cpu.iter()
            .map(|p| (p.granted(), p.unresolved()))
            .collect::<Vec<_>>(),
        [(1, 0), (100, 0)]
    );
    for gpu in gpu_results {
        assert_eq!(gpu.unwrap(), cpu);
    }
}

struct ExactFixture {
    state: WorldGpuState,
    semantic_plan: ResidentClearingPlan,
    buffers: ResidentClearingBuffers,
    executor: ResidentApportionmentSession,
}

struct Case {
    requests: Vec<u32>,
    bases: Vec<f32>,
    supply: u32,
    precedences: Vec<u32>,
    generation: u32,
    neutral: bool,
}

impl Case {
    fn live(requests: &[u32], bases: &[f32], supply: u32) -> Self {
        assert_eq!(requests.len(), bases.len());
        Self {
            requests: requests.to_vec(),
            bases: bases.to_vec(),
            supply,
            precedences: vec![0; requests.len()],
            generation: 4,
            neutral: false,
        }
    }
}

impl ExactFixture {
    fn new(reverse_admission: bool) -> Self {
        let ctx = GpuContext::new_blocking().unwrap();
        let tree = SimThing::new(SimThingKind::GameSession, 4);
        let realm = TreeRealmId::from_u128(0x1591).unwrap();
        let generation = TreeGenerationAuthority::new(GenerationStamp::new(4));
        let schedule = IntegrationSchedule::new();
        let mut registry = DimensionRegistry::new();
        registry.register(SimProperty::simple("cap-referee", "allocated-flow", 1));
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
        let mut admissions: Vec<_> = (0..4)
            .map(|index| ResidentClearingAdmission {
                owner,
                resource: ResidentResourceId::new(1),
                scope: ResidentScopeId::new(10),
                draw: ResidentDrawId::new(1_000 + index),
            })
            .collect();
        if reverse_admission {
            admissions.reverse();
        }
        let semantic_plan = ResidentClearingPlan::build(
            &binding,
            admissions,
            ResidentClearingBudgets::new(4, 4, 4, 128, 128, 65_536, 262_144, 8_192, 64).unwrap(),
        )
        .unwrap();
        let buffers =
            ResidentClearingBuffers::allocate(&ctx.device, &binding, &semantic_plan).unwrap();
        let executor = ResidentApportionmentSession::new(&ctx);
        let state = WorldGpuState::new(ctx, &registry, 4);
        Self {
            state,
            semantic_plan,
            buffers,
            executor,
        }
    }

    fn prepare(
        &self,
        case: &Case,
        order: &[usize],
        rotate_slots: bool,
    ) -> (ResidentApportionmentPlan, Vec<f32>) {
        let mut values = vec![0.0; self.state.values_len()];
        let claims = order
            .iter()
            .map(|&index| {
                let slot = (index + usize::from(rotate_slots)) % 4;
                values[slot * self.state.n_dims as usize] = case.bases[index];
                let semantic_row = self
                    .semantic_plan
                    .rows()
                    .iter()
                    .position(|row| {
                        self.semantic_plan.dictionaries().draws()[row.draw().get() as usize].get()
                            == 1_000 + index as u64
                    })
                    .unwrap();
                ResidentApportionmentClaim::new(
                    semantic_row as u32,
                    SimThingId::from_session_raw(1_000 + index as u32),
                    case.requests[index],
                    case.supply,
                    case.precedences[index],
                    SlotIndex::new(slot as u32),
                    ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
                    if case.neutral {
                        ResidentExactBasisIdentity::NeutralRequest
                    } else {
                        ResidentExactBasisIdentity::LiveAllocatedFlow
                    },
                )
            })
            .collect();
        let plan = ResidentApportionmentPlan::build(
            &self.semantic_plan,
            claims,
            SimThingId::from_session_raw(7),
            GenerationStamp::new(case.generation),
            3,
        )
        .unwrap();
        self.state.install_resolved_values_at_boundary(&values);
        (plan, values)
    }

    fn gpu(
        &mut self,
        plan: &ResidentApportionmentPlan,
        dispatch: ResidentApportionmentDispatch,
    ) -> Vec<ResidentConstrainedProduct> {
        let (semantic_rows, scratch) = self.buffers.apportionment_buffers(plan).unwrap();
        let mut encoder =
            self.state
                .ctx
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("15.9 exact cap cross-product"),
                });
        self.state
            .encode_resident_apportionment_with_dispatch_into(
                &mut self.executor,
                &mut encoder,
                semantic_rows,
                scratch,
                plan,
                dispatch,
            )
            .unwrap();
        self.state.ctx.queue.submit(Some(encoder.finish()));
        let _ = self.state.ctx.device.poll(wgpu::Maintain::Wait);
        self.executor
            .readback_products(&self.state.ctx, scratch, plan)
            .unwrap()
    }
}

#[test]
fn no_collision_products_remain_bit_identical_to_dispatched_master() {
    let mut fixture = ExactFixture::new(false);
    let mut cases = Vec::new();
    for a in 0..4 {
        for b in 0..4 {
            for c in 0..4 {
                for supply in [0, 1, 2, 5, 17] {
                    // Each quota is at most S <= 17, strictly below every cap.
                    cases.push(Case::live(
                        &[100; 3],
                        &[a as f32, b as f32, c as f32],
                        supply,
                    ));
                }
            }
        }
    }
    cases.push(Case::live(&[100, 200, 300], &[100.0, 200.0, 300.0], 100));
    for supply in [0, 7, 100] {
        cases.push(Case::live(&[17, 33, 50], &[17.0, 33.0, 50.0], supply));
    }
    cases.push(Case::live(&[u32::MAX], &[f32::MAX], u32::MAX));
    cases.push(Case::live(&[0, 5], &[0.0, 5.0], 3));
    for generation in 0..8 {
        let mut case = Case::live(&[7; 4], &[1.0; 4], 5);
        case.generation = generation;
        cases.push(case);
    }
    let mut mixed = Case::live(&[100, 1, 9], &[0.0, 1.0, 9.0], 10);
    mixed.precedences = vec![0, 0, 1];
    cases.push(mixed);
    cases.push(Case::live(
        &[100; 3],
        &[f32::from_bits(1), f32::from_bits(2), f32::from_bits(3)],
        5,
    ));
    cases.push(Case::live(
        &[u32::MAX, 1],
        &[f32::MAX, f32::from_bits(1)],
        u32::MAX,
    ));
    let mut neutral = Case::live(&[16_777_217, 16_777_216], &[16_777_216.0; 2], 1);
    neutral.neutral = true;
    cases.push(neutral);

    let mut digest = 0xcbf2_9ce4_8422_2325u64;
    for (index, case) in cases.iter().enumerate() {
        let order: Vec<_> = (0..case.requests.len()).collect();
        let (plan, values) = fixture.prepare(case, &order, false);
        let cpu = execute_resident_apportionment_cpu(&plan, &values, fixture.state.n_dims).unwrap();
        let gpu = fixture.gpu(&plan, ResidentApportionmentDispatch::single_pass());
        assert_eq!(gpu, cpu, "no-collision case {index}");
        for byte in bytemuck::cast_slice::<_, u8>(&cpu) {
            digest ^= u64::from(*byte);
            digest = digest.wrapping_mul(0x100_0000_01b3);
        }
    }
    println!("15.9 dispatched-master no-collision corpus: cases={} canonical-product-byte-digest={digest:016x}", cases.len());
    // Captured from the unchanged dispatched solver before production edits.
    assert_eq!(cases.len(), 338);
    assert_eq!(digest, 0x05cb_01d9_6dc6_9dbe);
}

#[test]
fn capped_active_sets_preserve_semantics_across_physical_shapes() {
    let mut cases = vec![
        (
            Case::live(&[1, 2, 100], &[1.0, 2.0, 1.0], 103),
            vec![1, 2, 100],
        ),
        // 10/3 freezes cap 1; then 9/2 freezes cap 4; the survivor gets 5.
        (Case::live(&[1, 4, 100], &[1.0; 3], 10), vec![1, 4, 5]),
        (
            Case::live(&[1, 100, 100], &[1.0, 0.3125, 7.0], 12),
            vec![1, 0, 11],
        ),
        // A frozen row's final notional quotient exceeds u32::MAX. Cap
        // classification must precede division over the final active basis.
        (
            Case::live(&[1, 100], &[1.0, f32::from_bits(1)], 101),
            vec![1, 100],
        ),
        (
            Case::live(&[1, 4, 100], &[f32::from_bits(1); 3], 10),
            vec![1, 4, 5],
        ),
    ];
    let mut mixed = Case::live(&[1, 100, 100, 9], &[1.0, 1.0, 0.0, 9.0], 110);
    mixed.precedences = vec![0, 0, 0, 1];
    cases.push((mixed, vec![1, 100, 0, 9]));
    for generation in 0..6 {
        let mut case = Case::live(&[1, 100, 100, 100], &[1.0; 4], 6);
        case.generation = generation;
        let mut expected = vec![1, 1, 1, 1];
        let rotation = (7 + generation as usize) % 3;
        expected[1 + rotation] += 1;
        expected[1 + (rotation + 1) % 3] += 1;
        cases.push((case, expected));
    }
    fn permutations(indices: &mut [usize], start: usize, result: &mut Vec<Vec<usize>>) {
        if start == indices.len() {
            result.push(indices.to_vec());
        } else {
            for next in start..indices.len() {
                indices.swap(start, next);
                permutations(indices, start + 1, result);
                indices.swap(start, next);
            }
        }
    }
    let mut canonical = Vec::new();
    let mut gpu_runs = 0;
    for reverse_admission in [false, true] {
        let mut fixture = ExactFixture::new(reverse_admission);
        for (case_index, (case, expected)) in cases.iter().enumerate() {
            let mut orders = Vec::new();
            permutations(
                &mut (0..case.requests.len()).collect::<Vec<_>>(),
                0,
                &mut orders,
            );
            for order in orders {
                for rotate_slots in [false, true] {
                    let (plan, values) = fixture.prepare(case, &order, rotate_slots);
                    let cpu =
                        execute_resident_apportionment_cpu(&plan, &values, fixture.state.n_dims)
                            .unwrap();
                    assert_eq!(
                        cpu.iter()
                            .map(|product| product.granted())
                            .collect::<Vec<_>>(),
                        *expected,
                        "case {case_index}"
                    );
                    for (index, product) in cpu.iter().enumerate() {
                        assert_eq!(product.unresolved(), case.requests[index] - expected[index]);
                    }
                    if canonical.len() == case_index {
                        canonical.push(cpu.clone());
                    }
                    assert_eq!(
                        cpu, canonical[case_index],
                        "canonical product drift, case {case_index}"
                    );
                    for workgroup in [
                        ResidentApportionmentWorkgroupSize::W32,
                        ResidentApportionmentWorkgroupSize::W64,
                    ] {
                        for rows_per_dispatch in [u32::MAX, 1] {
                            let dispatch =
                                ResidentApportionmentDispatch::new(workgroup, rows_per_dispatch)
                                    .unwrap();
                            assert_eq!(fixture.gpu(&plan, dispatch), cpu, "case {case_index}, order {order:?}, reverse admission {reverse_admission}, rotated slots {rotate_slots}, {dispatch:?}");
                            gpu_runs += 1;
                        }
                    }
                }
            }
        }
    }
    println!("15.9 multi-cap/multi-iteration/final-active-tie corpus: cases={} GPU physical runs={gpu_runs}", cases.len());
}
