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
    ResidentClearingAdmission, ResidentClearingBudgets, ResidentClearingPlan, ResidentDrawId,
    ResidentExactBasisIdentity, ResidentOwnerId, ResidentResourceId, ResidentScopeId,
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
