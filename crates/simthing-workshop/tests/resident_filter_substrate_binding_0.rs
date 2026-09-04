//! RESIDENT-FILTER-SUBSTRATE-BINDING-0 focused production referee.

use simthing_core::{
    ColumnIndex, DimensionRegistry, GenerationStamp, IntegrationSchedule,
    PersistenceDeformationProgram, SimProperty, SimThing, SimThingId, TransformOp,
    TreeExecutionAuthority, TreeGenerationAuthority, TreeRealmId,
};
use simthing_driver::resident_clearing_runtime::{
    build_default_resident_arena_registry, install_default_resident_rf_property,
    ResidentAuthoredDemand, ResidentClearingBatchBinding, ResidentClearingRuntime,
    ResidentClearingRuntimeError, ResidentMarketAdmission, ResidentPersistenceDeformationBinding,
    RESIDENT_MARKET_RF_ARENA,
};
use simthing_driver::{
    resolve_node_columns_for_property, sync_resource_flow_accumulator, ArenaRegistry,
};
use simthing_gpu::{GpuContext, ResidentExactBasisIdentity, SlotAllocator, WorldGpuState};

const ROOT: u32 = 7;
const LEFT: u32 = 8;
const RIGHT: u32 = 9;
const LEFT_LEAF: u32 = 10;
const GROWTH: u32 = 11;

fn id(raw: u32) -> SimThingId {
    SimThingId::from_session_raw(raw)
}

fn loaded_tree() -> SimThing {
    serde_json::from_str(
        r#"{
            "id": 7,
            "kind": "GameSession",
            "properties": [],
            "resource_parent_edges": [],
            "overlays": [],
            "children": [{
                "id": 8,
                "kind": "Owner",
                "properties": [],
                "resource_parent_edges": [],
                "overlays": [],
                "children": [{
                    "id": 10,
                    "kind": "Cohort",
                    "properties": [],
                    "resource_parent_edges": [],
                    "overlays": [],
                    "children": [],
                    "spawned_generation": 30
                }],
                "spawned_generation": 30
            }, {
                "id": 9,
                "kind": "Cohort",
                "properties": [],
                "resource_parent_edges": [],
                "overlays": [],
                "children": [],
                "spawned_generation": 30
            }],
            "spawned_generation": 30
        }"#,
    )
    .expect("real recursive resident-filter tree")
}

fn authored_market(draw: &str) -> ResidentMarketAdmission {
    authored_market_with_basis(draw, ResidentExactBasisIdentity::LiveAllocatedFlow)
}

fn authored_market_with_basis(
    draw: &str,
    exact_basis_identity: ResidentExactBasisIdentity,
) -> ResidentMarketAdmission {
    ResidentMarketAdmission::new(
        "authored::shipyard-residency-market",
        "authored::residency-row-capacity",
        "authored::owner-channel/shipyard",
        draw,
        Some(RESIDENT_MARKET_RF_ARENA.into()),
        "authored::hard-precedence/u32-ascending",
        "authored::child-share-eml/e11-0001",
        exact_basis_identity,
    )
}

struct RealArenaFixture {
    tree: SimThing,
    registry: DimensionRegistry,
    allocator: SlotAllocator,
    arena_registry: ArenaRegistry,
    state: WorldGpuState,
    intrinsic_flow_col: ColumnIndex,
    allocated_flow_col: ColumnIndex,
    weight_col: ColumnIndex,
    n_bands: u32,
}

impl RealArenaFixture {
    fn new(gpu: &GpuContext) -> Self {
        let mut tree = loaded_tree();
        let mut registry = DimensionRegistry::new();
        let property_id = install_default_resident_rf_property(&mut registry, &mut tree);
        let mut allocator = SlotAllocator::new();
        allocator.install_initial_tree(&tree).unwrap();
        let arena_registry =
            build_default_resident_arena_registry(property_id, &tree, &allocator, 4).unwrap();
        let columns =
            resolve_node_columns_for_property(&registry, property_id, RESIDENT_MARKET_RF_ARENA)
                .unwrap();
        let mut state = WorldGpuState::new(gpu.clone(), &registry, allocator.capacity() as u32);
        let mut projected = vec![0.0; state.values_len()];
        simthing_gpu::project_tree_to_values(
            &tree,
            &registry,
            &allocator,
            state.n_dims as usize,
            &mut projected,
        );
        state.install_resolved_values_at_boundary(&projected);
        let flow = sync_resource_flow_accumulator(&mut state, &registry, &arena_registry, &[], &[])
            .unwrap();
        Self {
            tree,
            registry,
            allocator,
            arena_registry,
            state,
            intrinsic_flow_col: columns.intrinsic_flow_col,
            allocated_flow_col: columns.allocated_flow_col,
            weight_col: columns.weight_col,
            n_bands: flow.n_bands,
        }
    }

    fn install_allocated_flows(&mut self, left: f32, right: f32) {
        let mut values = self.state.read_values();
        let n_dims = self.state.n_dims as usize;
        for (participant, allocated) in [(LEFT, left), (RIGHT, right)] {
            let slot = self
                .arena_registry
                .participant_slot(id(participant), 0)
                .unwrap()
                .raw() as usize;
            values[slot * n_dims + self.allocated_flow_col.raw()] = allocated;
        }
        self.state.install_resolved_values_at_boundary(&values);
    }

    fn run_rf(&mut self, left_weight: f32, right_weight: f32) {
        let mut values = self.state.read_values();
        let n_dims = self.state.n_dims as usize;
        let write = |values: &mut [f32], slot: u32, column: ColumnIndex, value: f32| {
            values[slot as usize * n_dims + column.raw()] = value;
        };
        write(
            &mut values,
            self.arena_registry
                .participant_slot(id(ROOT), 0)
                .unwrap()
                .raw(),
            self.intrinsic_flow_col,
            left_weight + right_weight,
        );
        // LEFT is recursive, so its branch basis is the ordinary reduction of
        // LEFT_LEAF rather than a branch-local host shortcut.
        write(
            &mut values,
            self.arena_registry
                .participant_slot(id(LEFT_LEAF), 0)
                .unwrap()
                .raw(),
            self.weight_col,
            left_weight,
        );
        write(
            &mut values,
            self.arena_registry
                .participant_slot(id(RIGHT), 0)
                .unwrap()
                .raw(),
            self.weight_col,
            right_weight,
        );
        self.state.install_resolved_values_at_boundary(&values);
        self.state.run_resource_flow_bands(self.n_bands, 1.0);
    }

    fn schedule() -> IntegrationSchedule {
        let mut schedule = IntegrationSchedule::new();
        schedule.admit_resident_live_head(32).unwrap();
        schedule
    }

    fn admit(
        &self,
        gpu: &GpuContext,
        schedule: &IntegrationSchedule,
        market: ResidentMarketAdmission,
        deformations: &[ResidentPersistenceDeformationBinding],
        lane_capacity: u32,
    ) -> Result<ResidentClearingRuntime, ResidentClearingRuntimeError> {
        ResidentClearingRuntime::admit_market_with_persistence_deformations(
            gpu,
            TreeRealmId::from_u128(0x15_06).unwrap(),
            &self.tree,
            &self.registry,
            &self.arena_registry,
            &self.allocator,
            schedule,
            GenerationStamp::new(30),
            lane_capacity,
            market,
            deformations,
        )
    }
}

fn two_branch_rows() -> [ResidentClearingBatchBinding; 2] {
    [
        ResidentClearingBatchBinding {
            source_simthing_id: id(LEFT),
            rf_participant: id(LEFT),
            requested: 10,
            available: 1,
            precedence: 0,
        },
        ResidentClearingBatchBinding {
            source_simthing_id: id(RIGHT),
            rf_participant: id(RIGHT),
            requested: 10,
            available: 1,
            precedence: 0,
        },
    ]
}

#[derive(Clone, Copy)]
struct ExactBasisProductionCase {
    label: &'static str,
    exact_basis_identity: ResidentExactBasisIdentity,
    left_allocated_flow: f32,
    expected_grants: [u32; 2],
}

const EXACT_BASIS_PRODUCTION_CASES: [ExactBasisProductionCase; 3] = [
    ExactBasisProductionCase {
        label: "neutral",
        exact_basis_identity: ResidentExactBasisIdentity::NeutralRequest,
        left_allocated_flow: 16_777_216.0,
        expected_grants: [1, 0],
    },
    ExactBasisProductionCase {
        label: "genuine-below-cap",
        exact_basis_identity: ResidentExactBasisIdentity::LiveAllocatedFlow,
        left_allocated_flow: 16_777_216.0,
        expected_grants: [0, 1],
    },
    ExactBasisProductionCase {
        label: "genuine-above-cap",
        exact_basis_identity: ResidentExactBasisIdentity::LiveAllocatedFlow,
        left_allocated_flow: 33_554_432.0,
        expected_grants: [1, 0],
    },
];

#[test]
fn exact_basis_identity_is_qualification_bound_not_dispatch_authority() {
    let gpu = GpuContext::new_blocking().expect("qualified resident adapter");
    let mut fixture = RealArenaFixture::new(&gpu);
    fixture.install_allocated_flows(16_777_216.0, 16_777_216.0);
    let mut schedule = RealArenaFixture::schedule();
    let live_runtime = fixture
        .admit(
            &gpu,
            &schedule,
            authored_market("authored::draw/e5-same-token-mutant"),
            &[],
            2,
        )
        .unwrap();
    let mut neutral_runtime = fixture
        .admit(
            &gpu,
            &schedule,
            authored_market_with_basis(
                "authored::draw/e5-same-token-mutant",
                ResidentExactBasisIdentity::NeutralRequest,
            ),
            &[],
            2,
        )
        .unwrap();
    let live_qualification = live_runtime.market_qualification();
    let neutral_qualification = neutral_runtime.market_qualification();
    assert_eq!(
        live_qualification.exact_basis_identity(),
        ResidentExactBasisIdentity::LiveAllocatedFlow
    );
    assert_eq!(
        neutral_qualification.exact_basis_identity(),
        ResidentExactBasisIdentity::NeutralRequest
    );
    assert_ne!(live_qualification, neutral_qualification);
    assert_ne!(
        live_qualification.market_semantic_digest(),
        neutral_qualification.market_semantic_digest()
    );
    assert_eq!(
        live_qualification.exact_projection_abi_digest(),
        neutral_qualification.exact_projection_abi_digest()
    );
    let rows = [
        ResidentClearingBatchBinding {
            source_simthing_id: id(LEFT),
            rf_participant: id(LEFT),
            requested: 16_777_217,
            available: 1,
            precedence: 0,
        },
        ResidentClearingBatchBinding {
            source_simthing_id: id(RIGHT),
            rf_participant: id(RIGHT),
            requested: 16_777_216,
            available: 1,
            precedence: 0,
        },
    ];
    assert!(matches!(
        neutral_runtime.dispatch(
            &fixture.state,
            &live_qualification,
            &mut schedule,
            id(ROOT),
            GenerationStamp::new(30),
            &rows,
        ),
        Err(ResidentClearingRuntimeError::StaleMarketQualification)
    ));

    let grants = |products: &[simthing_gpu::ResidentConstrainedProduct]| {
        products
            .iter()
            .map(|product| product.granted())
            .collect::<Vec<_>>()
    };
    for case in EXACT_BASIS_PRODUCTION_CASES {
        fixture.install_allocated_flows(case.left_allocated_flow, 16_777_216.0);
        let mut runtime = fixture
            .admit(
                &gpu,
                &schedule,
                authored_market_with_basis(
                    "authored::draw/e5-production-table",
                    case.exact_basis_identity,
                ),
                &[],
                2,
            )
            .unwrap();
        let qualification = runtime.market_qualification();
        let ticket = runtime
            .dispatch(
                &fixture.state,
                &qualification,
                &mut schedule,
                id(ROOT),
                GenerationStamp::new(30),
                &rows,
            )
            .unwrap();
        let products = runtime
            .materialize(&fixture.state, &qualification, &mut schedule, ticket)
            .unwrap();
        assert_eq!(grants(&products), case.expected_grants, "{}", case.label);
    }

    println!(
        "15.6 E5 PRODUCTION PASS dispatch-tag=ABSENT cross-basis-token=TYPED-REFUSAL neutral=source-8 below-cap=source-9 above-cap=source-8"
    );
}

#[test]
fn authored_market_qualifies_and_live_arena_cells_defeat_stale_host_assumptions() {
    let gpu = GpuContext::new_blocking().expect("qualified resident adapter");
    let mut fixture = RealArenaFixture::new(&gpu);
    let mut schedule = RealArenaFixture::schedule();
    let mut runtime = fixture
        .admit(
            &gpu,
            &schedule,
            authored_market("authored::draw/shipyard"),
            &[],
            2,
        )
        .expect("genuinely authored non-implicit market lowers completely");
    let qualification = runtime.market_qualification();
    assert!(qualification.has_intact_seal());
    assert_ne!(qualification.market_semantic_digest(), 0);
    assert_ne!(qualification.resource_shape_digest(), 0);
    assert_ne!(qualification.scope_draw_shape_digest(), 0);
    assert_ne!(qualification.topology_digest(), 0);
    assert_ne!(qualification.registry_layout_digest(), 0);
    assert_ne!(qualification.precedence_digest(), 0);
    assert_ne!(qualification.continuous_policy_digest(), 0);
    assert_ne!(qualification.exact_projection_abi_digest(), 0);

    let rows = two_branch_rows();
    fixture.run_rf(9.0, 1.0);
    let first = runtime
        .dispatch(
            &fixture.state,
            &qualification,
            &mut schedule,
            id(ROOT),
            GenerationStamp::new(30),
            &rows,
        )
        .unwrap();
    fixture.run_rf(1.0, 9.0);
    let second = runtime
        .dispatch(
            &fixture.state,
            &qualification,
            &mut schedule,
            id(ROOT),
            GenerationStamp::new(30),
            &rows,
        )
        .unwrap();
    let first = runtime
        .materialize(&fixture.state, &qualification, &mut schedule, first)
        .unwrap();
    let second = runtime
        .materialize(&fixture.state, &qualification, &mut schedule, second)
        .unwrap();
    assert_eq!((first[0].granted(), first[1].granted()), (1, 0));
    assert_eq!((second[0].granted(), second[1].granted()), (0, 1));

    let mutated_market = fixture
        .admit(
            &gpu,
            &schedule,
            authored_market("authored::draw/changed"),
            &[],
            2,
        )
        .unwrap()
        .market_qualification();
    assert_ne!(qualification, mutated_market);

    let mut mutated_registry = fixture.registry.clone();
    mutated_registry.register(SimProperty::simple("mutation", "layout", 1));
    let mutated_registry_runtime =
        ResidentClearingRuntime::admit_market_with_persistence_deformations(
            &gpu,
            TreeRealmId::from_u128(0x15_06).unwrap(),
            &fixture.tree,
            &mutated_registry,
            &fixture.arena_registry,
            &fixture.allocator,
            &schedule,
            GenerationStamp::new(30),
            2,
            authored_market("authored::draw/shipyard"),
            &[],
        )
        .unwrap();
    assert_ne!(
        qualification.registry_layout_digest(),
        mutated_registry_runtime
            .market_qualification()
            .registry_layout_digest()
    );

    let refusal = fixture.admit(
        &gpu,
        &schedule,
        ResidentMarketAdmission::new(
            "authored::cannot-lower",
            "authored::resource",
            "authored::scope",
            "authored::draw",
            Some("absent-arena".into()),
            "authored::precedence",
            "authored::policy",
            ResidentExactBasisIdentity::LiveAllocatedFlow,
        ),
        &[],
        2,
    );
    assert!(matches!(
        refusal,
        Err(ResidentClearingRuntimeError::MarketCannotLower { .. })
    ));

    println!(
        "15.6 AUTHORED-MARKET PASS live-AllocatedFlow-winners=left/right market-mutation=INVALID registry-mutation=INVALID incomplete-lowering=TYPED-REFUSAL"
    );
}

#[test]
fn topology_growth_rebind_preserves_identity_live_head_and_pending_provenance() {
    let gpu = GpuContext::new_blocking().expect("qualified resident adapter");
    let mut fixture = RealArenaFixture::new(&gpu);
    fixture.run_rf(5.0, 5.0);
    let mut schedule = RealArenaFixture::schedule();
    let half = PersistenceDeformationProgram::admit(TransformOp::multiply(0.5), 10).unwrap();
    let mut runtime = fixture
        .admit(
            &gpu,
            &schedule,
            authored_market("authored::draw/rebind"),
            &[ResidentPersistenceDeformationBinding {
                source_simthing_id: id(LEFT),
                program: half,
            }],
            2,
        )
        .unwrap();
    let old_qualification = runtime.market_qualification();
    let realm = runtime.realm();
    let incarnation = runtime.incarnation();
    let pending = runtime
        .dispatch(
            &fixture.state,
            &old_qualification,
            &mut schedule,
            id(ROOT),
            GenerationStamp::new(30),
            &[ResidentClearingBatchBinding {
                source_simthing_id: id(LEFT),
                rf_participant: id(LEFT),
                requested: 10,
                available: 0,
                precedence: 0,
            }],
        )
        .unwrap();

    let mut growth = SimThing::new(simthing_core::SimThingKind::Cohort, 31);
    growth.id = id(GROWTH);
    fixture.tree.add_child(growth);
    install_default_resident_rf_property(&mut fixture.registry, &mut fixture.tree);
    fixture
        .allocator
        .install_initial_tree(&fixture.tree)
        .unwrap();
    if fixture.state.n_slots < fixture.allocator.capacity() as u32 {
        fixture
            .state
            .rebuild_for_slots(fixture.allocator.capacity() as u32, &fixture.registry);
    }
    let growth_slot = fixture.allocator.slot_of(id(GROWTH)).unwrap();
    fixture
        .arena_registry
        .admit_participant_runtime(0, growth_slot, id(GROWTH), Some(id(ROOT)))
        .unwrap();
    fixture.arena_registry.bump_generation_after_runtime_admit();
    let mut projected = vec![0.0; fixture.state.values_len()];
    simthing_gpu::project_tree_to_values(
        &fixture.tree,
        &fixture.registry,
        &fixture.allocator,
        fixture.state.n_dims as usize,
        &mut projected,
    );
    fixture
        .state
        .install_resolved_values_at_boundary(&projected);
    let flow = sync_resource_flow_accumulator(
        &mut fixture.state,
        &fixture.registry,
        &fixture.arena_registry,
        &[],
        &[],
    )
    .unwrap();
    fixture.state.run_resource_flow_bands(flow.n_bands, 1.0);

    let generation_authority = TreeGenerationAuthority::new(GenerationStamp::new(30));
    let authority = TreeExecutionAuthority::seal(
        realm,
        incarnation,
        &fixture.tree,
        &generation_authority,
        &schedule,
        &fixture.registry,
        &fixture.allocator,
    )
    .unwrap();
    let context = authority.seal_context().unwrap();
    let binding = context.bind(&authority).unwrap();
    let new_qualification = runtime
        .rebind_after_topology_change(&gpu, &binding, &fixture.arena_registry)
        .unwrap();
    assert_eq!(runtime.realm(), realm);
    assert_eq!(runtime.incarnation(), incarnation);
    assert_eq!(runtime.lane_capacity(), 2);
    assert_ne!(old_qualification, new_qualification);
    assert_ne!(
        old_qualification.topology_digest(),
        new_qualification.topology_digest()
    );
    assert!(matches!(
        runtime.dispatch(
            &fixture.state,
            &old_qualification,
            &mut schedule,
            id(ROOT),
            GenerationStamp::new(30),
            &two_branch_rows(),
        ),
        Err(ResidentClearingRuntimeError::StaleMarketQualification)
    ));

    let pending_products = runtime
        .materialize(&fixture.state, &new_qualification, &mut schedule, pending)
        .unwrap();
    assert_eq!(pending_products[0].generation(), GenerationStamp::new(30));
    assert_eq!(
        (
            pending_products[0].granted(),
            pending_products[0].unresolved()
        ),
        (0, 10)
    );
    let post_rebind = runtime
        .dispatch(
            &fixture.state,
            &new_qualification,
            &mut schedule,
            id(ROOT),
            GenerationStamp::new(30),
            &[ResidentClearingBatchBinding {
                source_simthing_id: id(GROWTH),
                rf_participant: id(GROWTH),
                requested: 1,
                available: 1,
                precedence: 0,
            }],
        )
        .unwrap();
    let post_rebind_products = runtime
        .materialize(
            &fixture.state,
            &new_qualification,
            &mut schedule,
            post_rebind,
        )
        .unwrap();
    assert_eq!(
        pending_products[0].semantic_row(),
        post_rebind_products[0].semantic_row(),
        "pre-existing owner/resource/scope/Draw semantic row remains stable"
    );

    let persistent = runtime
        .dispatch(
            &fixture.state,
            &new_qualification,
            &mut schedule,
            id(ROOT),
            GenerationStamp::new(30),
            &[ResidentClearingBatchBinding {
                source_simthing_id: id(LEFT),
                rf_participant: id(LEFT),
                requested: 10,
                available: 0,
                precedence: 0,
            }],
        )
        .unwrap();
    let demand = runtime
        .prepare_temporal_demands(
            &fixture.state,
            &new_qualification,
            &persistent,
            GenerationStamp::new(31),
            &[ResidentAuthoredDemand {
                source_simthing_id: id(LEFT),
                quantity: 0,
            }],
        )
        .unwrap();
    let minted = runtime
        .readback_temporal_demands_for_proof(&fixture.state, &new_qualification, &demand)
        .unwrap();
    assert_eq!(minted[0].quantity(), 5);

    println!(
        "15.6 E7 PASS topology-participants=4->5 lane-capacity={} realm=PRESERVED incarnation={} generation=30 pending-live-head=PRESERVED semantic-row={} persistence-demand=5 topology-token=INVALIDATED",
        runtime.lane_capacity(),
        runtime.incarnation().get(),
        pending_products[0].semantic_row()
    );
}

#[test]
fn synthetic_production_substrate_and_fresh_reconstruction_are_mechanically_absent() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let resident = std::fs::read_to_string(
        root.join("crates/simthing-driver/src/resident_clearing_runtime.rs"),
    )
    .unwrap();
    for forbidden in [
        "continuous_plane",
        "continuous_values",
        "prepare_continuous_allocation",
        "continuous_weight",
    ] {
        assert_eq!(resident.matches(forbidden).count(), 0, "{forbidden}");
    }
    let session =
        std::fs::read_to_string(root.join("crates/simthing-driver/src/session.rs")).unwrap();
    let rebind = session
        .split("pub fn react_to_fission_resource_flow_enrollment")
        .nth(1)
        .unwrap()
        .split("fn validate_resource_flow_execution")
        .next()
        .unwrap();
    assert!(rebind.contains("rebind_resident_clearing_to_current_arena"));
    assert!(!rebind.contains("ResidentClearingRuntime::admit"));

    println!(
        "15.6 ZERO-USE PASS private-world=0 host-vector=0 per-dispatch-plan=0 host-weight=0 E7-fresh-executor=0"
    );
}
