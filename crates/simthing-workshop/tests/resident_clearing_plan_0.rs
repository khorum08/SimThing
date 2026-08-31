use std::cell::Cell;
use std::rc::Rc;

use serde_json::Value;
use simthing_core::{
    DimensionRegistry, ExecutionIncarnation, GenerationStamp, IntegrationSchedule, SimThing,
    TreeExecutionAuthority, TreeExecutionContext, TreeExecutionContextError,
    TreeGenerationAuthority, TreeRealmId,
};
use simthing_gpu::{
    GpuContext, ResidentClearingAbi, ResidentClearingBuffers, ResidentClearingGpuError,
};
use simthing_kernel::{
    DenseOrdinalRange, ResidentClearingAdmission, ResidentClearingBudgets, ResidentClearingPlan,
    ResidentClearingPlanError, ResidentClearingReplayEnvelope, ResidentClearingReplayError,
    ResidentDrawId, ResidentOwnerId, ResidentResourceId, ResidentScopeId, SlotAllocator,
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

fn replay_envelope() -> ResidentClearingReplayEnvelope {
    ResidentClearingReplayEnvelope::new(16, 16, 16, 16, 32, 16_384, 65_536, 8_192, 64)
        .expect("consumer-owned replay envelope")
}

fn bounded_replay_envelope() -> ResidentClearingReplayEnvelope {
    ResidentClearingReplayEnvelope::new(1, 1, 1, 1, 1, 2, 1, 1, 1)
        .expect("tiny consumer-owned replay envelope")
}

fn replay_json(
    packet: &str,
    trusted: ResidentClearingReplayEnvelope,
) -> Result<ResidentClearingPlan, ResidentClearingReplayError<serde_json::Error>> {
    let mut deserializer = serde_json::Deserializer::from_str(packet);
    let plan = ResidentClearingPlan::replay_with_budget_envelope(trusted, &mut deserializer)?;
    deserializer
        .end()
        .map_err(ResidentClearingReplayError::Transport)?;
    Ok(plan)
}

fn replay_value(
    packet: Value,
    trusted: ResidentClearingReplayEnvelope,
) -> Result<ResidentClearingPlan, ResidentClearingReplayError<serde_json::Error>> {
    ResidentClearingPlan::replay_with_budget_envelope(trusted, packet)
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
fn resident_plan_is_canonical_migratable_and_destination_local() {
    // Both roots arrive through the ordinary persisted-tree reconstruction
    // door with overlapping local id 7; no process-global id mint participates.
    let tree_a = loaded_tree(7, 8, 10);
    let tree_b = loaded_tree(7, 8, 20);
    let realm_a = TreeRealmId::from_u128(1).unwrap();
    let realm_b = TreeRealmId::from_u128(2).unwrap();
    let generation_a = TreeGenerationAuthority::new(GenerationStamp::new(10));
    let generation_b = TreeGenerationAuthority::new(GenerationStamp::new(20));
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

    let authority_a = TreeExecutionAuthority::seal(
        realm_a,
        ExecutionIncarnation::new(11).unwrap(),
        &tree_a,
        &generation_a,
        &schedule_a,
        &registry_a,
        &residency_a,
    )
    .unwrap();
    // Continuation target A falsifier: the unrepeatable wrapper mint belongs
    // to the one-per-tree generation authority, not to the first wrapper.
    assert!(matches!(
        TreeExecutionAuthority::seal(
            realm_a,
            ExecutionIncarnation::new(11).unwrap(),
            &tree_a,
            &generation_a,
            &schedule_a,
            &registry_a,
            &residency_a,
        ),
        Err(TreeExecutionContextError::GenerationAuthorityAlreadySealed)
    ));
    let authority_b = TreeExecutionAuthority::seal(
        realm_b,
        ExecutionIncarnation::new(22).unwrap(),
        &tree_b,
        &generation_b,
        &schedule_b,
        &registry_b,
        &residency_b,
    )
    .unwrap();
    let context_a = authority_a.seal_context().unwrap();
    let context_b = authority_b.seal_context().unwrap();

    // DA falsifier 1 (verbatim): context A + tree B + B
    // schedule/registry/residency fails typed even though both roots have raw id 7.
    assert!(matches!(
        context_a.bind(&authority_b),
        Err(TreeExecutionContextError::AuthorityCapsuleMismatch)
    ));
    // DA falsifier 3 (verbatim): two realm/incarnation contexts cannot bind
    // the same runtime authority capsule absent an explicit partition/lease law.
    assert!(matches!(
        authority_a.seal_context(),
        Err(TreeExecutionContextError::ContextAlreadyMinted)
    ));

    let binding_a = context_a.bind(&authority_a).unwrap();
    let binding_b = context_b.bind(&authority_b).unwrap();
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

    let plan_a = ResidentClearingPlan::build(&binding_a, semantic_rows_a.clone(), budgets())
        .expect("tree A plan");
    assert_eq!(
        plan_a.host_admission_row_storage(),
        (4, 32, 0),
        "logical admission length is distinct from exact reserved envelope"
    );
    assert_eq!(plan_a.streaming_semantic_bytes(), 308);
    let mut reversed = semantic_rows_a.clone();
    reversed.reverse();
    let plan_a_reversed = ResidentClearingPlan::build(&binding_a, reversed, budgets())
        .expect("reverse-admission plan");
    let mut permuted = semantic_rows_a.clone();
    permuted.rotate_left(1);
    let plan_a_permuted = ResidentClearingPlan::build(&binding_a, permuted, budgets())
        .expect("permuted-admission plan");
    let plan_a_replay = ResidentClearingPlan::build(&binding_a, semantic_rows_a.clone(), budgets())
        .expect("replay reconstruction");
    let serde_replay =
        replay_json(&serde_json::to_string(&plan_a).unwrap(), replay_envelope()).unwrap();

    assert_eq!(plan_a.dictionaries(), plan_a_reversed.dictionaries());
    assert_eq!(plan_a.ranges(), plan_a_reversed.ranges());
    assert_eq!(plan_a.rows(), plan_a_reversed.rows());
    assert_eq!(plan_a.canonical_bytes(), plan_a_reversed.canonical_bytes());
    assert_eq!(plan_a.canonical_bytes(), plan_a_permuted.canonical_bytes());
    assert_eq!(plan_a.canonical_bytes(), plan_a_replay.canonical_bytes());
    assert_eq!(plan_a.canonical_bytes(), serde_replay.canonical_bytes());
    assert_eq!(plan_a.digest(), plan_a_reversed.digest());
    assert_eq!(plan_a.digest(), plan_a_permuted.digest());
    assert_eq!(plan_a.digest(), plan_a_replay.digest());
    assert_eq!(plan_a.digest(), serde_replay.digest());

    let plan_b =
        ResidentClearingPlan::build(&binding_b, semantic_rows_b, budgets()).expect("tree B plan");
    assert_ne!(
        plan_b.owner_ordinal(b7).unwrap(),
        plan_a.owner_ordinal(b7).unwrap(),
        "destination remapping uses destination-local dictionary order"
    );

    let observation = observe_resident_clearing_plan(&plan_a);
    assert_eq!(observation.owner_count, 3);
    assert_eq!(observation.row_count, 4);
    assert_eq!(observation.digest, plan_a.digest());
    assert_eq!(observation.canonical_bytes, plan_a.canonical_bytes().len());

    let gpu = GpuContext::new_blocking().expect("real adapter for resident storage");
    let mut buffers_a = ResidentClearingBuffers::allocate(&gpu.device, &binding_a, &plan_a)
        .expect("tree A resident buffers");
    let buffers_b = ResidentClearingBuffers::allocate(&gpu.device, &binding_b, &plan_b)
        .expect("tree B resident buffers");
    assert_ne!(buffers_a.owner(), buffers_b.owner());
    assert!(!std::ptr::eq(
        buffers_a.header_buffer(),
        buffers_b.header_buffer()
    ));
    assert!(!std::ptr::eq(
        buffers_a.owner_buffer(),
        buffers_b.owner_buffer()
    ));

    // R2: N -> N+1 changes no semantic bytes/digest and retains every exact
    // resident buffer object. Only transient owner/header POD state advances.
    let semantic_before = plan_a.canonical_bytes();
    let digest_before = plan_a.digest();
    let header_identity = buffers_a.header_buffer() as *const _;
    let owner_identity = buffers_a.owner_buffer() as *const _;
    let row_identity = buffers_a.row_buffer() as *const _;
    generation_a.advance(GenerationStamp::new(11)).unwrap();
    let binding_a_n1 = context_a.bind(&authority_a).unwrap();
    let advance = buffers_a
        .advance_generation(&binding_a_n1, &plan_a)
        .expect("cheap resident generation advance");
    assert_eq!(advance.previous(), GenerationStamp::new(10));
    assert_eq!(advance.current(), GenerationStamp::new(11));
    assert_eq!(advance.digest(), digest_before);
    assert_eq!(plan_a.canonical_bytes(), semantic_before);
    assert_eq!(plan_a.digest(), digest_before);
    assert!(std::ptr::eq(
        header_identity,
        buffers_a.header_buffer() as *const _
    ));
    assert!(std::ptr::eq(
        owner_identity,
        buffers_a.owner_buffer() as *const _
    ));
    assert!(std::ptr::eq(
        row_identity,
        buffers_a.row_buffer() as *const _
    ));

    let migrated_a = authority_a
        .migrate_context(&context_a, context_a.incarnation().next().unwrap())
        .expect("migration advances the live authority record");
    assert!(matches!(
        context_a.bind(&authority_a),
        Err(TreeExecutionContextError::StaleIncarnation { .. })
    ));
    let migrated_binding = migrated_a.bind(&authority_a).unwrap();
    let migrated_plan =
        ResidentClearingPlan::build(&migrated_binding, semantic_rows_a, budgets()).unwrap();
    assert_eq!(context_a.realm(), migrated_a.realm());
    assert_eq!(context_a.root(), migrated_a.root());
    assert_ne!(context_a.incarnation(), migrated_a.incarnation());
    assert_eq!(plan_a.canonical_bytes(), migrated_plan.canonical_bytes());
    assert_eq!(plan_a.digest(), migrated_plan.digest());

    let fork_realm = authority_a.fork_realm(77).expect("semantic fork realm");
    assert_ne!(fork_realm, context_a.realm());

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
}

#[derive(Clone)]
struct PullCountingRows {
    pulls: Rc<Cell<usize>>,
    remaining: usize,
    row: ResidentClearingAdmission,
}

impl Iterator for PullCountingRows {
    type Item = ResidentClearingAdmission;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        self.pulls.set(self.pulls.get() + 1);
        Some(self.row)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX / 2, None)
    }
}

#[test]
fn all_layout_and_budget_failures_precede_gpu_allocation() {
    let tree = loaded_tree(7, 8, 4);
    let generation = TreeGenerationAuthority::new(GenerationStamp::new(4));
    let schedule = IntegrationSchedule::new();
    let registry = DimensionRegistry::new();
    let residency = SlotAllocator::new();
    let authority = TreeExecutionAuthority::seal(
        TreeRealmId::from_u128(9).unwrap(),
        ExecutionIncarnation::new(3).unwrap(),
        &tree,
        &generation,
        &schedule,
        &registry,
        &residency,
    )
    .unwrap();
    let context = authority.seal_context().unwrap();
    let binding = context.bind(&authority).unwrap();
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

    // R4: an adversarial iterator advertises a huge size and contains 1,000
    // rows, but admission pulls only max_rows + 1 and stores at most max_rows.
    let pulls = Rc::new(Cell::new(0));
    let bounded_rows = PullCountingRows {
        pulls: Rc::clone(&pulls),
        remaining: 1_000,
        row: admission(owner7, 1, 1, 1),
    };
    let row_limited = ResidentClearingBudgets::new(2, 2, 2, 2, 2, 4096, 4096, 128, 64)
        .expect("bounded rows budget");
    assert!(matches!(
        ResidentClearingPlan::build(&binding, bounded_rows, row_limited),
        Err(ResidentClearingPlanError::RowCountBudgetExceeded {
            observed: 3,
            admitted: 2,
            stored: 2,
            reserved: 2,
            reallocations: 0,
        })
    ));
    assert_eq!(pulls.get(), 3, "refusal occurs exactly at max_rows + 1");

    let exactly_admitted = ResidentClearingPlan::build(
        &binding,
        rows.clone(),
        ResidentClearingBudgets::new(2, 2, 2, 2, 2, 4096, 4096, 128, 64).unwrap(),
    )
    .expect("lawful rows fill the exact admitted row allocation");
    assert_eq!(exactly_admitted.host_admission_row_storage(), (2, 2, 0));

    // B2: a large count envelope cannot defer the smaller semantic-byte law
    // until dictionary/canonical-row materialization. The first projected row
    // requires 180 bytes and is refused while streaming, after one pull.
    let semantic_pulls = Rc::new(Cell::new(0));
    let semantic_rows = PullCountingRows {
        pulls: Rc::clone(&semantic_pulls),
        remaining: 1_000,
        row: admission(owner7, 1, 1, 1),
    };
    let streaming_limited =
        ResidentClearingBudgets::new(128, 128, 128, 128, 128, 179, 65_536, 8_192, 64)
            .expect("large count envelope with deliberately small byte budget");
    assert!(matches!(
        ResidentClearingPlan::build(&binding, semantic_rows, streaming_limited),
        Err(ResidentClearingPlanError::SemanticPlanBudgetExceeded {
            required: 180,
            admitted: 179,
        })
    ));
    assert_eq!(
        semantic_pulls.get(),
        1,
        "semantic bytes refuse first excess projection"
    );

    let owner_limited = ResidentClearingBudgets::new(1, 2, 2, 2, 4, 4096, 4096, 512, 64)
        .expect("internally consistent budget");
    assert!(matches!(
        ResidentClearingPlan::build(&binding, rows.clone(), owner_limited),
        Err(ResidentClearingPlanError::CountBudgetExceeded { axis: "owners", .. })
    ));

    let semantic_limited = ResidentClearingBudgets::new(2, 2, 2, 2, 4, 64, 4096, 512, 64)
        .expect("internally consistent budget");
    assert!(matches!(
        ResidentClearingPlan::build(&binding, rows.clone(), semantic_limited),
        Err(ResidentClearingPlanError::SemanticPlanBudgetExceeded { .. })
    ));

    let resident_limited = ResidentClearingBudgets::new(2, 2, 2, 2, 4, 4096, 128, 512, 64).unwrap();
    let plan = ResidentClearingPlan::build(&binding, rows, resident_limited)
        .expect("semantic plan builds before physical budget check");
    assert!(matches!(
        ResidentClearingAbi::from_plan(&binding, &plan),
        Err(ResidentClearingGpuError::ResidentBudgetExceeded { .. })
    ));
}

fn mutated_plan_rejects(plan: &ResidentClearingPlan, mutate: impl FnOnce(&mut Value)) {
    let mut wire = serde_json::to_value(plan).unwrap();
    mutate(&mut wire);
    assert!(replay_value(wire, replay_envelope()).is_err());
}

fn bounded_wire_prefix() -> &'static str {
    r#"{"version":2,"context":{"realm":[1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"root":7},"budgets":{"max_owners":1,"max_resources":1,"max_scopes":1,"max_draws":1,"max_rows":1,"max_semantic_plan_bytes":2,"max_resident_bytes":1,"max_scratch_bytes":1,"scratch_bytes_per_row":1},"#
}

fn assert_bounded_wire_first_excess(payload: &str, expected: ResidentClearingPlanError) {
    let packet = format!("{}{}", bounded_wire_prefix(), payload);
    let error = replay_json(&packet, bounded_replay_envelope())
        .expect_err("hostile wire must fail while consuming its first excess element");
    assert_eq!(
        match error {
            ResidentClearingReplayError::Plan(error) => error,
            ResidentClearingReplayError::Transport(error) => {
                panic!("plan refusal must win before malformed tail: {error}")
            }
        },
        expected
    );
}

#[test]
fn context_and_plan_fail_closed_on_mismatched_authority() {
    assert_eq!(std::mem::size_of::<simthing_core::SimThingId>(), 4);
    assert!(std::mem::size_of::<TreeExecutionContext>() <= 32);

    let tree = loaded_tree(7, 8, 4);
    let other_tree = loaded_tree(7, 10, 4);
    let generation = TreeGenerationAuthority::new(GenerationStamp::new(4));
    let other_generation = TreeGenerationAuthority::new(GenerationStamp::new(4));
    let schedule = IntegrationSchedule::new();
    let other_schedule = IntegrationSchedule::new();
    let registry = DimensionRegistry::new();
    let other_registry = DimensionRegistry::new();
    let residency = SlotAllocator::new();
    let other_residency = SlotAllocator::new();
    let authority = TreeExecutionAuthority::seal(
        TreeRealmId::from_u128(9).unwrap(),
        ExecutionIncarnation::new(3).unwrap(),
        &tree,
        &generation,
        &schedule,
        &registry,
        &residency,
    )
    .unwrap();
    let other_authority = TreeExecutionAuthority::seal(
        TreeRealmId::from_u128(10).unwrap(),
        ExecutionIncarnation::new(4).unwrap(),
        &other_tree,
        &other_generation,
        &other_schedule,
        &other_registry,
        &other_residency,
    )
    .unwrap();
    let context = authority.seal_context().unwrap();
    let binding = context.bind(&authority).unwrap();
    assert!(matches!(
        context.bind(&other_authority),
        Err(TreeExecutionContextError::AuthorityCapsuleMismatch)
    ));

    let owner7 = ResidentOwnerId::new(context.qualify(tree.id));
    let owner8 = ResidentOwnerId::new(context.qualify(tree.children[0].id));
    let row7 = admission(owner7, 1, 1, 1);
    let row8 = admission(owner8, 2, 2, 2);
    assert!(matches!(
        ResidentClearingPlan::build(&binding, [row7, row7], budgets()),
        Err(ResidentClearingPlanError::DuplicateAdmission)
    ));
    let plan = ResidentClearingPlan::build(&binding, [row7, row8], budgets()).unwrap();

    // Validated replay succeeds only through the consumer-owned envelope door
    // and still reconstructs through the same ordinary admission constructor.
    let replay = replay_value(serde_json::to_value(&plan).unwrap(), replay_envelope()).unwrap();
    assert_eq!(replay, plan);

    // Target C componentwise trust proof: every fixed-size wire budget claim
    // is compared to its independently constructed outer ceiling before any
    // dictionary, row, or canonical-byte sequence visitor can reserve.
    let outer_budget_checks = [
        ("max_owners", 17_u64, 16_u64),
        ("max_resources", 17, 16),
        ("max_scopes", 17, 16),
        ("max_draws", 17, 16),
        ("max_rows", 33, 32),
        ("max_semantic_plan_bytes", 16_385, 16_384),
        ("max_resident_bytes", 65_537, 65_536),
        ("max_scratch_bytes", 8_193, 8_192),
        ("scratch_bytes_per_row", 65, 64),
    ];
    for (field, claimed, admitted) in outer_budget_checks {
        let mut wire = serde_json::to_value(&plan).unwrap();
        wire["budgets"][field] = claimed.into();
        match replay_value(wire, replay_envelope()) {
            Err(ResidentClearingReplayError::Plan(
                ResidentClearingPlanError::WireBudgetExceedsTrustedEnvelope {
                    field: observed_field,
                    claimed: observed_claimed,
                    admitted: observed_admitted,
                },
            )) => {
                assert_eq!(observed_field, field);
                assert_eq!(observed_claimed, claimed);
                assert_eq!(observed_admitted, admitted);
            }
            other => panic!("expected typed trusted-envelope refusal for {field}: {other:?}"),
        }
    }

    // C3 trust-inversion falsifier: the fixed budget block is internally
    // consistent but forges every ceiling far above the consumer envelope.
    // A malformed sequence tail follows. The max_owners typed refusal wins at
    // the budget block, before the tail is parsed or any sequence is reserved.
    let forged_large_budget = r#"{"version":2,"context":{"realm":[1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"root":7},"budgets":{"max_owners":1000000,"max_resources":1000000,"max_scopes":1000000,"max_draws":1000000,"max_rows":1000000,"max_semantic_plan_bytes":1000000000,"max_resident_bytes":1000000000,"max_scratch_bytes":64000000,"scratch_bytes_per_row":64},"dictionaries":{"owners":[BROKEN"#;
    match replay_json(forged_large_budget, replay_envelope()) {
        Err(ResidentClearingReplayError::Plan(
            ResidentClearingPlanError::WireBudgetExceedsTrustedEnvelope {
                field,
                claimed,
                admitted,
            },
        )) => {
            assert_eq!(field, "max_owners");
            assert_eq!(claimed, 1_000_000);
            assert_eq!(admitted, 16);
        }
        other => panic!("expected typed forged-envelope refusal: {other:?}"),
    }

    // Target D: a plan-domain refusal produced after complete transport parse
    // survives the same typed domain channel with no prose recovery.
    let mut canonical_mismatch = serde_json::to_value(&plan).unwrap();
    canonical_mismatch["canonical_bytes"][0] = 0.into();
    assert!(matches!(
        replay_value(canonical_mismatch, replay_envelope()),
        Err(ResidentClearingReplayError::Plan(
            ResidentClearingPlanError::CanonicalBytesMismatch
        ))
    ));

    // Syntax failure carries the deserializer's concrete error, distinct from
    // all replay/domain refusals.
    assert!(matches!(
        replay_json("{", replay_envelope()),
        Err(ResidentClearingReplayError::Transport(_))
    ));

    // R5 malformed-wire census: zero realm/root authority, overflowing range,
    // zero/inconsistent budget, malformed dictionary/row, canonical mismatch,
    // and stored digest mismatch all fail before yielding a sealed plan.
    mutated_plan_rejects(&plan, |wire| {
        wire["context"]["realm"] = serde_json::to_value(vec![0_u8; 16]).unwrap();
    });
    mutated_plan_rejects(&plan, |wire| wire["context"]["root"] = 0.into());
    mutated_plan_rejects(&plan, |wire| {
        wire["ranges"]["rows"]["start"] = u32::MAX.into();
        wire["ranges"]["rows"]["len"] = 1.into();
    });
    mutated_plan_rejects(&plan, |wire| wire["budgets"]["max_rows"] = 0.into());
    mutated_plan_rejects(&plan, |wire| {
        wire["budgets"]["max_scratch_bytes"] = 1.into();
    });
    mutated_plan_rejects(&plan, |wire| {
        wire["dictionaries"]["owners"]
            .as_array_mut()
            .unwrap()
            .swap(0, 1);
    });
    mutated_plan_rejects(&plan, |wire| wire["rows"][0][0] = 999.into());
    mutated_plan_rejects(&plan, |wire| {
        let low = wire["digest"][0].as_u64().unwrap();
        wire["digest"][0] = (low ^ 1).into();
    });

    // Continuation B3: budgets precede all variable payloads. Each packet has
    // one valid admitted item, the first excess valid item, then deliberately
    // invalid JSON. The bounded visitor's budget error must win before that
    // tail can be parsed, proving it never materializes the hostile sequence.
    let owner = r#"{"realm":[1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"local":1}"#;
    assert_bounded_wire_first_excess(
        &format!(r#""dictionaries":{{"owners":[{owner},{owner},BROKEN"#),
        ResidentClearingPlanError::CountBudgetExceeded {
            axis: "owners",
            observed: 2,
            admitted: 1,
        },
    );
    assert_bounded_wire_first_excess(
        r#""dictionaries":{"resources":[1,2,BROKEN"#,
        ResidentClearingPlanError::CountBudgetExceeded {
            axis: "resources",
            observed: 2,
            admitted: 1,
        },
    );
    assert_bounded_wire_first_excess(
        r#""dictionaries":{"scopes":[1,2,BROKEN"#,
        ResidentClearingPlanError::CountBudgetExceeded {
            axis: "scopes",
            observed: 2,
            admitted: 1,
        },
    );
    assert_bounded_wire_first_excess(
        r#""dictionaries":{"draws":[1,2,BROKEN"#,
        ResidentClearingPlanError::CountBudgetExceeded {
            axis: "draws",
            observed: 2,
            admitted: 1,
        },
    );
    assert_bounded_wire_first_excess(
        r#""rows":[[0,0,0,0],[0,0,0,0],BROKEN"#,
        ResidentClearingPlanError::CountBudgetExceeded {
            axis: "rows",
            observed: 2,
            admitted: 1,
        },
    );
    assert_bounded_wire_first_excess(
        r#""canonical_bytes":[0,0,0,BROKEN"#,
        ResidentClearingPlanError::SemanticPlanBudgetExceeded {
            required: 3,
            admitted: 2,
        },
    );

    let payload_before_budget =
        r#"{"version":2,"context":{"realm":[1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"root":7},"rows":[]}"#;
    assert!(matches!(
        replay_json(payload_before_budget, replay_envelope()),
        Err(ResidentClearingReplayError::Plan(
            ResidentClearingPlanError::MalformedWire {
                field: "rows_before_budgets"
            }
        ))
    ));

    assert!(
        serde_json::from_value::<DenseOrdinalRange>(serde_json::json!({
            "start": u32::MAX,
            "len": 1
        }))
        .is_err()
    );
    assert!(
        serde_json::from_value::<ResidentClearingBudgets>(serde_json::json!({
            "max_owners": 1,
            "max_resources": 1,
            "max_scopes": 1,
            "max_draws": 1,
            "max_rows": 1,
            "max_semantic_plan_bytes": 1,
            "max_resident_bytes": 1,
            "max_scratch_bytes": 0,
            "scratch_bytes_per_row": 1
        }))
        .is_err()
    );
}
