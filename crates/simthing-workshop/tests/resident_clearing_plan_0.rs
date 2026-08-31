use simthing_core::{
    DimensionRegistry, ExecutionIncarnation, GenerationStamp, IntegrationSchedule, SeamFact,
    SeamFactId, SimThing, TreeExecutionContext, TreeExecutionContextError, TreeRealmId,
};
use simthing_gpu::{
    GpuContext, ResidentClearingAbi, ResidentClearingBuffers, ResidentClearingGpuError,
};
use simthing_kernel::{
    DenseOrdinalRange, ResidentClearingAdmission, ResidentClearingBudgets, ResidentClearingPlan,
    ResidentClearingPlanError, ResidentDrawId, ResidentOwnerId, ResidentResourceId,
    ResidentScopeId, SlotAllocator,
};
use simthing_workshop::resident_clearing_plan::observe_resident_clearing_plan;

fn loaded_tree(root_id: u32, child_id: u32, generation: u32) -> SimThing {
    serde_json::from_str(&format!(
        r#"{{
            "id": {root_id},
            "kind": "GameSession",
            "properties": [],
            "resource_parent_edges": [],
            "overlays": [],
            "children": [{{
                "id": {child_id},
                "kind": "Owner",
                "properties": [],
                "resource_parent_edges": [],
                "overlays": [],
                "children": [],
                "spawned_generation": {generation}
            }}],
            "spawned_generation": {generation}
        }}"#
    ))
    .expect("real persisted SimThing tree")
}

fn budgets() -> ResidentClearingBudgets {
    ResidentClearingBudgets::new(16, 16, 16, 16, 32, 16_384, 65_536, 8_192, 64)
        .expect("admitted fixture budgets")
}

fn admission(
    owner: ResidentOwnerId,
    resource: u64,
    scope: u64,
    draw: u64,
) -> ResidentClearingAdmission {
    ResidentClearingAdmission {
        owner,
        resource: ResidentResourceId::new(resource),
        scope: ResidentScopeId::new(scope),
        draw: ResidentDrawId::new(draw),
    }
}

#[test]
fn resident_plan_is_canonical_migratable_and_seam_remapped() {
    // Both roots arrive through the ordinary persisted-tree reconstruction
    // door with overlapping local id 7; no process-global id mint participates.
    let tree_a = loaded_tree(7, 8, 10);
    let tree_b = loaded_tree(7, 8, 20);
    let realm_a = TreeRealmId::from_u128(1).unwrap();
    let realm_b = TreeRealmId::from_u128(2).unwrap();
    let context_a = TreeExecutionContext::new(
        realm_a,
        ExecutionIncarnation::new(11).unwrap(),
        tree_a.id,
        GenerationStamp::new(10),
    );
    let context_b = TreeExecutionContext::new(
        realm_b,
        ExecutionIncarnation::new(22).unwrap(),
        tree_b.id,
        GenerationStamp::new(20),
    );

    let generation_a = GenerationStamp::new(10);
    let generation_b = GenerationStamp::new(20);
    let schedule_a = IntegrationSchedule::new();
    let schedule_b = IntegrationSchedule::new();
    let registry_a = DimensionRegistry::new();
    let registry_b = DimensionRegistry::new();
    let mut residency_a = SlotAllocator::new();
    let mut residency_b = SlotAllocator::new();
    residency_a
        .install_initial_tree(&tree_a)
        .expect("tree A real residency");
    residency_b
        .install_initial_tree(&tree_b)
        .expect("tree B real residency");
    let binding_a = context_a
        .bind(
            &tree_a,
            &generation_a,
            &schedule_a,
            &registry_a,
            &residency_a,
        )
        .expect("tree A checked binding");
    let binding_b = context_b
        .bind(
            &tree_b,
            &generation_b,
            &schedule_b,
            &registry_b,
            &residency_b,
        )
        .expect("tree B checked binding");
    assert_eq!(binding_a.root().id.raw(), 7);
    assert_eq!(binding_b.root().id.raw(), 7);
    assert_ne!(binding_a.context().realm(), binding_b.context().realm());
    assert!(!std::ptr::eq(binding_a.schedule(), binding_b.schedule()));
    assert!(!std::ptr::eq(binding_a.registry(), binding_b.registry()));
    assert!(!std::ptr::eq(binding_a.residency(), binding_b.residency()));

    let a7 = ResidentOwnerId::new(context_a.qualify(tree_a.id));
    let a8 = ResidentOwnerId::new(context_a.qualify(tree_a.children[0].id));
    let b7 = ResidentOwnerId::new(context_b.qualify(tree_b.id));
    let b8 = ResidentOwnerId::new(context_b.qualify(tree_b.children[0].id));
    let semantic_rows_a = vec![
        admission(b7, 2, 9, 30),
        admission(a8, 1, 9, 20),
        admission(a7, 2, 8, 10),
        admission(a7, 1, 8, 11),
    ];
    let semantic_rows_b = vec![admission(b8, 1, 8, 20), admission(b7, 1, 8, 10)];

    let plan_a = ResidentClearingPlan::build(context_a, semantic_rows_a.clone(), budgets())
        .expect("tree A plan");
    let mut reversed = semantic_rows_a.clone();
    reversed.reverse();
    let plan_a_reversed = ResidentClearingPlan::build(context_a, reversed, budgets())
        .expect("reverse-admission plan");
    let mut permuted = semantic_rows_a.clone();
    permuted.rotate_left(1);
    let plan_a_permuted = ResidentClearingPlan::build(context_a, permuted, budgets())
        .expect("permuted-admission plan");
    let plan_a_replay = ResidentClearingPlan::build(context_a, semantic_rows_a.clone(), budgets())
        .expect("replay reconstruction");

    assert_eq!(plan_a.dictionaries(), plan_a_reversed.dictionaries());
    assert_eq!(plan_a.ranges(), plan_a_reversed.ranges());
    assert_eq!(plan_a.rows(), plan_a_reversed.rows());
    assert_eq!(plan_a.canonical_bytes(), plan_a_reversed.canonical_bytes());
    assert_eq!(plan_a.canonical_bytes(), plan_a_permuted.canonical_bytes());
    assert_eq!(plan_a.canonical_bytes(), plan_a_replay.canonical_bytes());
    assert_eq!(plan_a.digest(), plan_a_reversed.digest());
    assert_eq!(plan_a.digest(), plan_a_permuted.digest());
    assert_eq!(plan_a.digest(), plan_a_replay.digest());

    let migrated_a = context_a
        .migrate(context_a.incarnation().next().unwrap())
        .expect("migration changes incarnation");
    let migrated_plan = ResidentClearingPlan::build(migrated_a, semantic_rows_a.clone(), budgets())
        .expect("migration recreation");
    assert_eq!(context_a.realm(), migrated_a.realm());
    assert_eq!(context_a.root(), migrated_a.root());
    assert_ne!(context_a.incarnation(), migrated_a.incarnation());
    assert_ne!(context_a.canonical_bytes(), migrated_a.canonical_bytes());
    assert_eq!(plan_a.canonical_bytes(), migrated_plan.canonical_bytes());
    assert_eq!(plan_a.digest(), migrated_plan.digest());
    assert_eq!(
        plan_a.bind_context(migrated_a).unwrap().incarnation(),
        migrated_a.incarnation()
    );

    let forked_a = context_a
        .fork(77, ExecutionIncarnation::new(1).unwrap())
        .expect("semantic fork");
    assert_ne!(forked_a.realm(), context_a.realm());
    assert_eq!(forked_a.root(), context_a.root());

    let plan_b =
        ResidentClearingPlan::build(context_b, semantic_rows_b, budgets()).expect("tree B plan");
    let source_ordinal = plan_b.owner_ordinal(b7).unwrap();
    let fact_id = SeamFactId::new(realm_b, 0xabc, context_b.generation(), source_ordinal.get());
    let fact = SeamFact::new(
        fact_id,
        context_b.incarnation(),
        context_b.qualify(tree_b.id),
    )
    .expect("canonical seam fact");
    context_b.admit_seam_fact(&fact).expect("current fact");
    let destination_ordinal = plan_a
        .remap_seam_owner(context_b, &fact)
        .expect("destination remap from canonical identity");
    assert_ne!(
        source_ordinal, destination_ordinal,
        "source ordinal must not be consumed as destination identity"
    );
    assert_eq!(
        plan_a.dictionaries().owners()
            [usize::try_from(destination_ordinal.get()).expect("u32 ordinal fits host index")],
        b7
    );

    let migrated_b = context_b
        .migrate(context_b.incarnation().next().unwrap())
        .unwrap();
    assert!(matches!(
        migrated_b.admit_seam_fact(&fact),
        Err(TreeExecutionContextError::StaleIncarnation { .. })
    ));
    let retry = fact.id();
    let lawful_multiplicity = SeamFactId::new(
        realm_b,
        fact_id.seam_id(),
        fact_id.source_generation(),
        fact_id.source_ordinal() + 1,
    );
    assert_eq!(retry, fact_id, "retry preserves exact identity");
    assert_ne!(
        lawful_multiplicity, retry,
        "lawful multiplicity is distinct"
    );

    let observation = observe_resident_clearing_plan(&plan_a);
    assert_eq!(observation.owner_count, 3);
    assert_eq!(observation.row_count, 4);
    assert_eq!(observation.digest, plan_a.digest());
    assert_eq!(observation.canonical_bytes, plan_a.canonical_bytes().len());
    println!(
        "RESIDENT-CLEARING-PLAN canonical_bytes={} digest={} owners={} resources={} scopes={} draws={} rows={}",
        observation.canonical_bytes,
        observation.digest.to_hex(),
        observation.owner_count,
        observation.resource_count,
        observation.scope_count,
        observation.draw_count,
        observation.row_count,
    );

    // Two simultaneously live final-home GPU buffer owners share only the
    // adapter. Their authority, dictionaries, generations, and allocations
    // are separately constructed and realm/incarnation-bound.
    let gpu = GpuContext::new_blocking().expect("real adapter for resident storage");
    let buffers_a = ResidentClearingBuffers::allocate(&gpu.device, context_a, &plan_a)
        .expect("tree A resident buffers");
    let buffers_b = ResidentClearingBuffers::allocate(&gpu.device, context_b, &plan_b)
        .expect("tree B resident buffers");
    assert_ne!(buffers_a.owner(), buffers_b.owner());
    assert_ne!(buffers_a.owner().realm(), buffers_b.owner().realm());
    assert_ne!(
        buffers_a.owner().generation(),
        buffers_b.owner().generation()
    );
    assert!(!std::ptr::eq(
        buffers_a.header_buffer(),
        buffers_b.header_buffer()
    ));
    assert!(!std::ptr::eq(
        buffers_a.owner_buffer(),
        buffers_b.owner_buffer()
    ));
    assert_ne!(plan_a.canonical_bytes(), plan_b.canonical_bytes());
    for descriptor in buffers_a.abi().descriptors() {
        println!(
            "RESIDENT-CLEARING-BUFFER kind={} count={} stride={} logical={} allocated={}",
            descriptor.kind(),
            descriptor.count(),
            descriptor.stride_bytes(),
            descriptor.logical_bytes(),
            descriptor.allocated_bytes(),
        );
    }
    println!(
        "RESIDENT-CLEARING-BUDGET total_allocated={} admitted_resident={} scratch_allocated={} admitted_scratch={}",
        buffers_a.abi().total_allocated_bytes(),
        plan_a.budgets().max_resident_bytes(),
        buffers_a
            .abi()
            .descriptor(simthing_gpu::RESIDENT_BUFFER_SCRATCH)
            .unwrap()
            .allocated_bytes(),
        plan_a.budgets().max_scratch_bytes(),
    );
}

#[test]
fn all_layout_and_budget_failures_precede_gpu_allocation() {
    let tree = loaded_tree(7, 8, 4);
    let context = TreeExecutionContext::new(
        TreeRealmId::from_u128(9).unwrap(),
        ExecutionIncarnation::new(3).unwrap(),
        tree.id,
        GenerationStamp::new(4),
    );
    let owner7 = ResidentOwnerId::new(context.qualify(tree.id));
    let owner8 = ResidentOwnerId::new(context.qualify(tree.children[0].id));
    let rows = vec![admission(owner7, 1, 1, 1), admission(owner8, 1, 1, 1)];

    assert!(matches!(
        DenseOrdinalRange::try_new(u32::MAX, 1),
        Err(ResidentClearingPlanError::OrdinalRangeOverflow { .. })
    ));
    assert!(matches!(
        ResidentClearingBudgets::new(2, 2, 2, 2, 4, 4096, 4096, 64, 64),
        Err(ResidentClearingPlanError::ScratchBudgetInconsistent { .. })
    ));

    let owner_limited = ResidentClearingBudgets::new(1, 2, 2, 2, 4, 4096, 4096, 512, 64)
        .expect("internally consistent budget");
    assert!(matches!(
        ResidentClearingPlan::build(context, rows.clone(), owner_limited),
        Err(ResidentClearingPlanError::CountBudgetExceeded { axis: "owners", .. })
    ));

    let semantic_limited = ResidentClearingBudgets::new(2, 2, 2, 2, 4, 64, 4096, 512, 64)
        .expect("internally consistent budget");
    assert!(matches!(
        ResidentClearingPlan::build(context, rows.clone(), semantic_limited),
        Err(ResidentClearingPlanError::SemanticPlanBudgetExceeded { .. })
    ));

    let resident_limited = ResidentClearingBudgets::new(2, 2, 2, 2, 4, 4096, 128, 512, 64).unwrap();
    let plan = ResidentClearingPlan::build(context, rows, resident_limited)
        .expect("semantic plan builds before physical budget check");
    assert!(matches!(
        ResidentClearingAbi::from_plan(context, &plan),
        Err(ResidentClearingGpuError::ResidentBudgetExceeded { .. })
    ));
}

#[test]
fn context_and_plan_fail_closed_on_mismatched_authority() {
    assert_eq!(std::mem::size_of::<simthing_core::SimThingId>(), 4);

    let tree = loaded_tree(7, 8, 4);
    let other_tree = loaded_tree(9, 10, 4);
    let context = TreeExecutionContext::new(
        TreeRealmId::from_u128(9).unwrap(),
        ExecutionIncarnation::new(3).unwrap(),
        tree.id,
        GenerationStamp::new(4),
    );
    let generation = GenerationStamp::new(4);
    let schedule = IntegrationSchedule::new();
    let registry = DimensionRegistry::new();
    let residency = SlotAllocator::new();

    assert!(matches!(
        context.bind(&other_tree, &generation, &schedule, &registry, &residency),
        Err(TreeExecutionContextError::RootMismatch { .. })
    ));
    assert!(matches!(
        context.bind(
            &tree,
            &GenerationStamp::new(5),
            &schedule,
            &registry,
            &residency
        ),
        Err(TreeExecutionContextError::GenerationAuthorityMismatch { .. })
    ));

    let owner = ResidentOwnerId::new(context.qualify(tree.id));
    let row = admission(owner, 1, 1, 1);
    assert!(matches!(
        ResidentClearingPlan::build(context, [row, row], budgets()),
        Err(ResidentClearingPlanError::DuplicateAdmission)
    ));
    let plan = ResidentClearingPlan::build(context, [row], budgets()).unwrap();
    assert!(matches!(
        plan.bind_context(context.at_generation(GenerationStamp::new(5))),
        Err(ResidentClearingPlanError::ContextMismatch { .. })
    ));

    assert_eq!(context.canonical_bytes().len(), 32);
    assert_eq!(
        SeamFactId::new(context.realm(), 1, context.generation(), 0)
            .canonical_bytes()
            .len(),
        32
    );
}
