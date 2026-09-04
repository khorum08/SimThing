//! RESIDENT-CLEARING-CUTOVER-0 production cutover referee.

use simthing_core::{
    ClearingExecutionPosture, ColumnIndex, DimensionRegistry, ExecutionIncarnation,
    ExecutionPosture, GenerationStamp, IntegrationSchedule, ResidentClearingScheduleFact,
    ResidentScheduleError, SimProperty, SimThing, SimThingId, SimThingKind, TreeExecutionAuthority,
    TreeGenerationAuthority, TreeRealmId,
};
use simthing_driver::resident_clearing_runtime::{
    build_default_resident_arena_registry, install_default_resident_rf_property,
    ResidentClearingBatchBinding, ResidentClearingDispatchTicket, ResidentClearingRuntime,
    ResidentClearingRuntimeError, ResidentMarketQualification, ResidentSpatialClaimBinding,
};
use simthing_driver::{
    resolve_node_columns_for_property, sync_resource_flow_accumulator, ArenaRegistry, Scenario,
    SimSession,
};
use simthing_gpu::{
    GpuContext, ResidentClearingAdmission, ResidentClearingBudgets, ResidentClearingPlan,
    ResidentDrawId, ResidentOwnerId, ResidentResourceId, ResidentScopeId, SlotAllocator,
    WorldGpuState, QUALIFIED_RESIDENT_CLEARING_FINGERPRINT,
};
use simthing_kernel::ResidentClearingReplayEnvelope;

fn loaded_tree(generation: u32) -> SimThing {
    serde_json::from_str(&format!(
        r#"{{
            "id": 7,
            "kind": "GameSession",
            "properties": [],
            "resource_parent_edges": [],
            "overlays": [],
            "children": [{{
                "id": 8,
                "kind": "Owner",
                "properties": [],
                "resource_parent_edges": [],
                "overlays": [],
                "children": [{{
                    "id": 9,
                    "kind": "Cohort",
                    "properties": [],
                    "resource_parent_edges": [],
                    "overlays": [],
                    "children": [],
                    "spawned_generation": {generation}
                }}],
                "spawned_generation": {generation}
            }}, {{
                "id": 10,
                "kind": "Cohort",
                "properties": [],
                "resource_parent_edges": [],
                "overlays": [],
                "children": [],
                "spawned_generation": {generation}
            }}],
            "spawned_generation": {generation}
        }}"#
    ))
    .expect("persisted overlapping-id tree")
}

fn resident_rows() -> Vec<ResidentClearingBatchBinding> {
    vec![
        ResidentClearingBatchBinding {
            source_simthing_id: simthing_core::SimThingId::from_session_raw(7),
            rf_participant: simthing_core::SimThingId::from_session_raw(8),
            requested: 1,
            available: 1,
            precedence: 0,
        },
        ResidentClearingBatchBinding {
            source_simthing_id: simthing_core::SimThingId::from_session_raw(8),
            rf_participant: simthing_core::SimThingId::from_session_raw(10),
            requested: 1,
            available: 1,
            precedence: 0,
        },
    ]
}

struct ResidentHarness {
    runtime: ResidentClearingRuntime,
    state: WorldGpuState,
    qualification: ResidentMarketQualification,
    arena_registry: ArenaRegistry,
    intrinsic_flow_col: ColumnIndex,
    weight_col: ColumnIndex,
    n_bands: u32,
}

impl ResidentHarness {
    fn run_rf_with_weights(&mut self, root_flow: f32, weights: &[(SimThingId, f32)]) {
        let mut values = self.state.read_values();
        let n_dims = self.state.n_dims as usize;
        let root_slot = self
            .arena_registry
            .participant_slot(SimThingId::from_session_raw(7), 0)
            .expect("real RF root row");
        values[root_slot.raw() as usize * n_dims + self.intrinsic_flow_col.raw()] = root_flow;
        for (participant, weight) in weights {
            let slot = self
                .arena_registry
                .participant_slot(*participant, 0)
                .expect("real RF participant row");
            values[slot.raw() as usize * n_dims + self.weight_col.raw()] = *weight;
        }
        self.state.install_resolved_values_at_boundary(&values);
        self.state.run_resource_flow_bands(self.n_bands, 1.0);
    }

    fn dispatch(
        &mut self,
        schedule: &mut IntegrationSchedule,
        granter: SimThingId,
        generation: GenerationStamp,
        rows: &[ResidentClearingBatchBinding],
    ) -> Result<ResidentClearingDispatchTicket, ResidentClearingRuntimeError> {
        self.runtime.dispatch(
            &self.state,
            &self.qualification,
            schedule,
            granter,
            generation,
            rows,
        )
    }

    fn dispatch_spatial(
        &mut self,
        schedule: &mut IntegrationSchedule,
        parent: &ResidentClearingDispatchTicket,
        granter: SimThingId,
        generation: GenerationStamp,
        rows: &[ResidentSpatialClaimBinding],
    ) -> Result<ResidentClearingDispatchTicket, ResidentClearingRuntimeError> {
        self.runtime.dispatch_spatial(
            &self.state,
            &self.qualification,
            schedule,
            parent,
            granter,
            generation,
            rows,
        )
    }

    fn materialize(
        &mut self,
        schedule: &mut IntegrationSchedule,
        ticket: ResidentClearingDispatchTicket,
    ) -> Result<Vec<simthing_gpu::ResidentConstrainedProduct>, ResidentClearingRuntimeError> {
        self.runtime
            .materialize(&self.state, &self.qualification, schedule, ticket)
    }

    fn readback_allocated_flow_for_proof(
        &self,
        participants: &[SimThingId],
    ) -> Result<Vec<f32>, ResidentClearingRuntimeError> {
        self.runtime.readback_allocated_flow_for_proof(
            &self.state,
            &self.qualification,
            participants,
        )
    }

    fn qualification(&self) -> &simthing_gpu::ResidentClearingQualification {
        self.runtime.qualification()
    }

    fn realm(&self) -> TreeRealmId {
        self.runtime.realm()
    }

    fn buffer_owner(&self) -> simthing_gpu::ResidentClearingBufferOwner {
        self.runtime.buffer_owner()
    }
}

fn admit_runtime(
    gpu: &GpuContext,
    realm: TreeRealmId,
    generation: GenerationStamp,
) -> (ResidentHarness, IntegrationSchedule) {
    let mut tree = loaded_tree(generation.get());
    let mut registry = DimensionRegistry::new();
    let property_id = install_default_resident_rf_property(&mut registry, &mut tree);
    let mut residency = SlotAllocator::new();
    residency
        .install_initial_tree(&tree)
        .expect("tree-local residency");
    let mut schedule = IntegrationSchedule::new();
    schedule
        .admit_resident_live_head(4)
        .expect("bounded resident live head");
    let arena_registry = build_default_resident_arena_registry(property_id, &tree, &residency, 8)
        .expect("real recursive RF arena");
    let columns = resolve_node_columns_for_property(
        &registry,
        property_id,
        simthing_driver::resident_clearing_runtime::RESIDENT_MARKET_RF_ARENA,
    )
    .expect("canonical RF columns");
    let mut state = WorldGpuState::new(gpu.clone(), &registry, residency.capacity() as u32);
    let mut projected = vec![0.0; state.values_len()];
    simthing_gpu::project_tree_to_values(
        &tree,
        &registry,
        &residency,
        state.n_dims as usize,
        &mut projected,
    );
    state.install_resolved_values_at_boundary(&projected);
    let flow = sync_resource_flow_accumulator(&mut state, &registry, &arena_registry, &[], &[])
        .expect("ordinary RF plan upload");
    state.run_resource_flow_bands(flow.n_bands, 1.0);
    let runtime = ResidentClearingRuntime::admit(
        gpu,
        realm,
        &tree,
        &registry,
        &arena_registry,
        &residency,
        &schedule,
        generation,
        4,
    )
    .expect("qualified production resident executor");
    let qualification = runtime.market_qualification();
    (
        ResidentHarness {
            runtime,
            state,
            qualification,
            arena_registry,
            intrinsic_flow_col: columns.intrinsic_flow_col,
            weight_col: columns.weight_col,
            n_bands: flow.n_bands,
        },
        schedule,
    )
}

#[derive(Debug, PartialEq, Eq)]
struct CausalAllocationCase {
    allocated_flow_bits: Vec<u32>,
    generation_n: Vec<(u32, u32, u32)>,
    child_same_generation: Vec<(u32, u32, u32)>,
}

fn economic_products(
    products: &[simthing_gpu::ResidentConstrainedProduct],
) -> Vec<(u32, u32, u32)> {
    products
        .iter()
        .map(|product| {
            (
                product.source_simthing_id().raw(),
                product.granted(),
                product.unresolved(),
            )
        })
        .collect()
}

fn run_causal_allocation_case(
    gpu: &GpuContext,
    realm: TreeRealmId,
    generation: u32,
    available: u32,
    weights: [f32; 2],
) -> CausalAllocationCase {
    let (mut runtime, mut schedule) = admit_runtime(gpu, realm, GenerationStamp::new(generation));
    let rows = [
        ResidentClearingBatchBinding {
            source_simthing_id: simthing_core::SimThingId::from_session_raw(7),
            rf_participant: simthing_core::SimThingId::from_session_raw(8),
            requested: 17,
            available,
            precedence: 0,
        },
        ResidentClearingBatchBinding {
            source_simthing_id: simthing_core::SimThingId::from_session_raw(8),
            rf_participant: simthing_core::SimThingId::from_session_raw(10),
            requested: 33,
            available,
            precedence: 0,
        },
    ];
    runtime.run_rf_with_weights(
        weights.into_iter().sum(),
        &[
            // Branch 8 is recursive; its level-N allocator basis is the
            // reduced weight of leaf 9, not a host-authored branch shortcut.
            (simthing_core::SimThingId::from_session_raw(9), weights[0]),
            (simthing_core::SimThingId::from_session_raw(10), weights[1]),
        ],
    );
    let generation_n = runtime
        .dispatch(
            &mut schedule,
            simthing_core::SimThingId::from_session_raw(0),
            GenerationStamp::new(generation),
            &rows,
        )
        .expect("generation N production continuous allocation");
    let child_same_generation = runtime
        .dispatch_spatial(
            &mut schedule,
            &generation_n,
            simthing_core::SimThingId::from_session_raw(8),
            GenerationStamp::new(generation),
            &[ResidentSpatialClaimBinding {
                source_simthing_id: simthing_core::SimThingId::from_session_raw(9),
                rf_participant: simthing_core::SimThingId::from_session_raw(8),
                requested: 33,
                precedence: 0,
            }],
        )
        .expect("same-generation child consumes immutable parent T_s.G");
    assert!(schedule.entries().is_empty());

    // Every observer runs only after both submissions; the child consumed the
    // resident parent product without a host readback.
    let allocated_flow_bits = runtime
        .readback_allocated_flow_for_proof(&[
            simthing_core::SimThingId::from_session_raw(8),
            simthing_core::SimThingId::from_session_raw(10),
        ])
        .expect("observe live shared RF cells after both submissions")
        .into_iter()
        .map(f32::to_bits)
        .collect();
    let generation_n = runtime
        .materialize(&mut schedule, generation_n)
        .expect("materialize generation N after N+1 submission");
    let child_same_generation = runtime
        .materialize(&mut schedule, child_same_generation)
        .expect("materialize child market after resident spatial consumption");

    CausalAllocationCase {
        allocated_flow_bits,
        generation_n: economic_products(&generation_n),
        child_same_generation: economic_products(&child_same_generation),
    }
}

fn prove_continuous_pressure_and_spatial_intake_are_causal(gpu: &GpuContext) {
    let neutral = run_causal_allocation_case(
        gpu,
        TreeRealmId::from_u128(0x14_06_c01).unwrap(),
        101,
        19,
        [17.0, 33.0],
    );
    let pressured = run_causal_allocation_case(
        gpu,
        TreeRealmId::from_u128(0x14_06_c02).unwrap(),
        101,
        19,
        [1.0, 49.0],
    );
    let supply_perturbed = run_causal_allocation_case(
        gpu,
        TreeRealmId::from_u128(0x14_06_c03).unwrap(),
        101,
        7,
        [17.0, 33.0],
    );

    // Neutral pressure makes the graduated child-share evaluator reproduce
    // the former request-bound share bits exactly, including a scarce 19/50
    // settlement. The expected grants are the frozen exact residue law.
    assert_eq!(
        neutral.allocated_flow_bits,
        vec![17.0f32.to_bits(), 33.0f32.to_bits()]
    );
    assert_eq!(neutral.generation_n, vec![(7, 6, 11), (8, 13, 20)]);
    assert_eq!(neutral.child_same_generation, vec![(9, 13, 20)]);

    // Only the generation-N upstream pressure changes. The real child-share
    // EML emits different AllocatedFlow bits, exact settlement changes, and
    // the same-generation child budget preserves that changed causal history.
    assert_eq!(
        pressured.allocated_flow_bits,
        vec![1.0f32.to_bits(), 49.0f32.to_bits()]
    );
    assert_eq!(pressured.generation_n, vec![(7, 1, 16), (8, 18, 15)]);
    assert_eq!(pressured.child_same_generation, vec![(9, 18, 15)]);
    assert_ne!(pressured.allocated_flow_bits, neutral.allocated_flow_bits);
    assert_ne!(pressured.generation_n, neutral.generation_n);

    // This falsifier holds the emitted continuous shares bit-identical and
    // changes only generation N's upstream available supply. N's T_s changes,
    // then the child grant changes with immutable G.
    assert_eq!(
        supply_perturbed.allocated_flow_bits,
        neutral.allocated_flow_bits
    );
    assert_eq!(supply_perturbed.generation_n, vec![(7, 2, 15), (8, 5, 28)]);
    assert_eq!(supply_perturbed.child_same_generation, vec![(9, 5, 28)]);
    assert_ne!(supply_perturbed.generation_n, neutral.generation_n);
    assert_ne!(
        supply_perturbed.child_same_generation,
        neutral.child_same_generation
    );

    println!(
        "RESIDENT-CLEARING-CUTOVER causal=PASS neutral={neutral:?} pressured={pressured:?} supply_perturbed={supply_perturbed:?}"
    );
}

#[test]
fn production_executor_is_async_tree_local_and_spatially_causal() {
    let gpu = GpuContext::new_blocking().expect("real cutover adapter");
    prove_continuous_pressure_and_spatial_intake_are_causal(&gpu);
    let realm_a = TreeRealmId::from_u128(0x14_06_a).unwrap();
    let realm_b = TreeRealmId::from_u128(0x14_06_b).unwrap();
    let (mut runtime_a, mut schedule_a) = admit_runtime(&gpu, realm_a, GenerationStamp::new(11));
    let (mut runtime_b, mut schedule_b) = admit_runtime(&gpu, realm_b, GenerationStamp::new(29));

    assert_eq!(
        runtime_a.qualification().fingerprint(),
        QUALIFIED_RESIDENT_CLEARING_FINGERPRINT
    );
    assert_ne!(runtime_a.realm(), runtime_b.realm());
    assert_ne!(runtime_a.buffer_owner(), runtime_b.buffer_owner());
    assert!(!std::ptr::eq(&schedule_a, &schedule_b));

    // Interleave divergent-generation trees with the same raw 7/8 claimant
    // ids. Neither dispatch maps a buffer, appends a host Vec row, waits for
    // the other tree, or consults a process-global semantic identity.
    let ticket_b = runtime_b
        .dispatch(
            &mut schedule_b,
            simthing_core::SimThingId::from_session_raw(0),
            GenerationStamp::new(29),
            &resident_rows(),
        )
        .expect("tree B resident dispatch");
    let ticket_a = runtime_a
        .dispatch(
            &mut schedule_a,
            simthing_core::SimThingId::from_session_raw(0),
            GenerationStamp::new(11),
            &resident_rows(),
        )
        .expect("tree A resident dispatch");
    let ticket_a_child = runtime_a
        .dispatch_spatial(
            &mut schedule_a,
            &ticket_a,
            simthing_core::SimThingId::from_session_raw(8),
            GenerationStamp::new(11),
            &[ResidentSpatialClaimBinding {
                source_simthing_id: simthing_core::SimThingId::from_session_raw(9),
                rf_participant: simthing_core::SimThingId::from_session_raw(8),
                requested: 1,
                precedence: 0,
            }],
        )
        .expect("tree A child consumes parent T_s while tree B remains isolated");
    assert_eq!(ticket_a.submission().generation(), GenerationStamp::new(11));
    assert_eq!(ticket_b.submission().generation(), GenerationStamp::new(29));
    assert_eq!(
        ticket_a_child.submission().generation(),
        GenerationStamp::new(11)
    );
    assert_eq!(
        ticket_a_child.submission().authority_granter(),
        simthing_core::SimThingId::from_session_raw(8)
    );
    assert_ne!(
        ticket_a.semantic_scope_owner(),
        ticket_a_child.semantic_scope_owner()
    );
    assert!(schedule_a.entries().is_empty());
    assert!(schedule_b.entries().is_empty());
    assert!(schedule_a.resident_materialization_pending());
    assert!(schedule_b.resident_materialization_pending());

    // No observer is needed before the child submission.
    let products_a_first = runtime_a
        .materialize(&mut schedule_a, ticket_a)
        .expect("tree A first asynchronous history materialization");
    let products_a = runtime_a
        .materialize(&mut schedule_a, ticket_a_child)
        .expect("tree A child asynchronous history materialization");
    let products_b = runtime_b
        .materialize(&mut schedule_b, ticket_b)
        .expect("tree B asynchronous history materialization");
    assert_eq!(economic_products(&products_a), vec![(9, 1, 0)]);
    assert_eq!(
        schedule_a.entries().len(),
        products_a_first.len() + products_a.len()
    );
    assert_eq!(schedule_b.entries().len(), products_b.len());
    assert!(!schedule_a.resident_materialization_pending());
    assert!(!schedule_b.resident_materialization_pending());

    let mut exhausted = IntegrationSchedule::new();
    exhausted.admit_resident_live_head(3).unwrap();
    let first = exhausted.reserve_resident_rows(1).unwrap();
    let second = exhausted.reserve_resident_rows(2).unwrap();
    assert_eq!(first.start(), 0);
    assert_eq!(second.start(), 1);
    assert_eq!(exhausted.resident_live_head_occupied_rows(), 3);
    assert!(matches!(
        exhausted.reserve_resident_rows(1),
        Err(ResidentScheduleError::ReplayEgressExhausted {
            requested: 4,
            capacity: 3
        })
    ));
    let fact = |semantic_row| ResidentClearingScheduleFact {
        semantic_row,
        source_simthing_id_raw: 7 + semantic_row,
        granted: 1,
        unresolved: 0,
        generation: GenerationStamp::new(11),
        integration_band: 0,
    };
    assert!(matches!(
        exhausted.materialize_resident_rows(second, &[fact(1), fact(2)]),
        Err(ResidentScheduleError::ReservationMismatch)
    ));
    exhausted
        .materialize_resident_rows(first, &[fact(0)])
        .unwrap();
    assert_eq!(exhausted.resident_live_head_occupied_rows(), 3);
    exhausted
        .materialize_resident_rows(second, &[fact(1), fact(2)])
        .unwrap();
    assert_eq!(exhausted.resident_live_head_occupied_rows(), 0);
    assert_eq!(exhausted.entries().len(), 3);
    let wire = serde_json::to_vec(&exhausted).unwrap();
    let mut resumed: IntegrationSchedule = serde_json::from_slice(&wire).unwrap();
    resumed.admit_resident_live_head(3).unwrap();
    assert_eq!(
        resumed.reserve_resident_rows(1).unwrap().first_sequence(),
        3
    );

    println!(
        "RESIDENT-CLEARING-CUTOVER two_tree=PASS realms={:?}/{:?} generations=A:11/B:29 spatial_child=A:11 live_head_capacity=4 materialized={}/{}",
        realm_a,
        realm_b,
        products_a_first.len() + products_a.len(),
        products_b.len()
    );
}

#[test]
fn scheduling_and_clearing_postures_are_independent() {
    for clearing in [
        ClearingExecutionPosture::ResidentRequired,
        ClearingExecutionPosture::CpuVendorizedOracle,
    ] {
        let mut registry = DimensionRegistry::new();
        registry.register(SimProperty::simple("cutover", "posture", 0));
        let scenario = Scenario {
            name: format!("cutover-posture-{clearing:?}"),
            ticks_per_day: 1,
            max_days: 2,
            dt: 1.0,
            n_slots: 2,
            registry,
            root: SimThing::new(SimThingKind::World, 0),
            shadow_seeds: Vec::new(),
            tick_patches: Vec::new(),
            install_targets: Default::default(),
        };
        let mut session = SimSession::open_with_clearing_posture(scenario, clearing)
            .expect("each clearing posture admits independently");
        assert_eq!(session.execution_posture(), ExecutionPosture::Paced);
        assert_eq!(session.clearing_execution_posture(), clearing);
        session
            .set_execution_posture(ExecutionPosture::continuous(2).unwrap())
            .expect("continuous scheduling does not alter clearing authority");
        assert_eq!(
            session.execution_posture(),
            ExecutionPosture::continuous(2).unwrap()
        );
        assert_eq!(session.clearing_execution_posture(), clearing);
    }
}

fn seam_budgets() -> ResidentClearingBudgets {
    ResidentClearingBudgets::new(2, 1, 1, 2, 2, 4096, 8192, 128, 64).unwrap()
}

fn seam_envelope() -> ResidentClearingReplayEnvelope {
    ResidentClearingReplayEnvelope::new(2, 1, 1, 2, 2, 4096, 8192, 128, 64).unwrap()
}

#[test]
fn realm_qualified_seam_wire_recreates_under_a_separate_executor_context() {
    let realm = TreeRealmId::from_u128(0x14_06_5ea).unwrap();
    let source_tree = loaded_tree(41);
    let source_generation = TreeGenerationAuthority::new(GenerationStamp::new(41));
    let source_schedule = IntegrationSchedule::new();
    let source_registry = DimensionRegistry::new();
    let mut source_residency = SlotAllocator::new();
    source_residency.install_initial_tree(&source_tree).unwrap();
    let source_authority = TreeExecutionAuthority::seal(
        realm,
        ExecutionIncarnation::new(1).unwrap(),
        &source_tree,
        &source_generation,
        &source_schedule,
        &source_registry,
        &source_residency,
    )
    .unwrap();
    let source_context = source_authority.seal_context().unwrap();
    let source_binding = source_context.bind(&source_authority).unwrap();
    let source_plan = ResidentClearingPlan::build(
        &source_binding,
        [
            ResidentClearingAdmission {
                owner: ResidentOwnerId::new(source_context.qualify(source_tree.id)),
                resource: ResidentResourceId::new(1),
                scope: ResidentScopeId::new(1),
                draw: ResidentDrawId::new(1),
            },
            ResidentClearingAdmission {
                owner: ResidentOwnerId::new(source_context.qualify(source_tree.children[0].id)),
                resource: ResidentResourceId::new(1),
                scope: ResidentScopeId::new(1),
                draw: ResidentDrawId::new(2),
            },
        ],
        seam_budgets(),
    )
    .unwrap();
    let wire = serde_json::to_vec(&source_plan).expect("serialize realm-qualified seam fact");
    let source_digest = source_plan.digest();
    drop(source_binding);
    drop(source_context);
    drop(source_authority);

    // Recreate the receiver under a new authority capsule and physical row
    // allocation. The wire carries realm-qualified semantic identity; no
    // source pointer, row, buffer handle, or translated economic payload can.
    let receiver_tree = loaded_tree(41);
    let receiver_generation = TreeGenerationAuthority::new(GenerationStamp::new(41));
    let receiver_schedule = IntegrationSchedule::new();
    let receiver_registry = DimensionRegistry::new();
    let mut receiver_residency = SlotAllocator::new();
    receiver_residency
        .install_initial_tree(&receiver_tree)
        .unwrap();
    let receiver_authority = TreeExecutionAuthority::seal(
        realm,
        ExecutionIncarnation::new(2).unwrap(),
        &receiver_tree,
        &receiver_generation,
        &receiver_schedule,
        &receiver_registry,
        &receiver_residency,
    )
    .unwrap();
    let receiver_context = receiver_authority.seal_context().unwrap();
    let receiver_binding = receiver_context.bind(&receiver_authority).unwrap();
    let mut deserializer = serde_json::Deserializer::from_slice(&wire);
    let received =
        ResidentClearingPlan::replay_with_budget_envelope(seam_envelope(), &mut deserializer)
            .expect("bounded seam receipt");
    deserializer.end().unwrap();
    received
        .bind_context(&receiver_binding)
        .expect("equivalent receipt binds in recreated context");
    assert_eq!(received.digest(), source_digest);
    assert_eq!(received.canonical_bytes(), source_plan.canonical_bytes());
    assert!(!std::ptr::eq(&source_tree, &receiver_tree));
    assert!(!std::ptr::eq(&source_residency, &receiver_residency));

    println!(
        "RESIDENT-CLEARING-CUTOVER seam=PASS realm={:?} digest={} source_incarnation=1 receiver_incarnation=2",
        realm,
        source_digest.to_hex()
    );
}
