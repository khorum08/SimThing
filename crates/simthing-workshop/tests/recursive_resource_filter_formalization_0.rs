//! RECURSIVE-RESOURCE-FILTER-FORMALIZATION-0 terminal theorem referee.
//!
//! This is a proof-only composition over graduated Phase-14 authorities. It
//! introduces no production operator, quantizer, response state, or market
//! vocabulary: `execute_reference_r` delegates to the existing generalized
//! allocation oracle and `project_q` delegates to the frozen exact oracle.

use std::collections::{BTreeMap, HashMap};

use simthing_core::owner_channel::OwnerRef;
use simthing_core::{
    ColumnIndex, CombineFn, DimensionRegistry, ExecutionIncarnation, GenerationStamp,
    IntegrationSchedule, SimPropertyId, SimThing, SimThingId, SlotIndex, TransformOp,
    TreeExecutionAuthority, TreeGenerationAuthority, TreeRealmId,
};
use simthing_driver::{
    build_custom_layout, plan_arena_allocation, plan_resident_exact_apportionment,
    produce_runtime_rf_next_generation_demands_for_tick, run_arena_allocation_oracle,
    ArenaAllocationOracleTrace, ArenaTreeLayout, FissionPolicy, GpuArenaDescriptor, HierarchyNode,
    NodeColumnRefs,
};
use simthing_kernel::{
    execute_resident_apportionment_cpu, ResidentApportionmentClaim, ResidentClearingAdmission,
    ResidentClearingBudgets, ResidentClearingPlan, ResidentConstrainedProduct, ResidentDrawId,
    ResidentOwnerId, ResidentRecursiveSupplyIntake, ResidentResourceId, ResidentScopeId,
    ResidentSettlementOutput, SlotAllocator,
};
use simthing_spec::{
    AuthoredClearingProgram, ClearingRemainderAuthority, ConstrainedClaim, ConstrainedSupply,
    OwnerChannelScopeKey, ResourceKey, RuntimeOwnerSiloDemandBucket,
    RuntimeRfDemandGenerationAuthority, RuntimeRfTickErrorKind, ScopeId,
};

const N_DIMS: u32 = 9;

type CellKey = (SlotIndex, ColumnIndex);

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

/// One root, one interior, and two leaves. Root/interior are deliberately not
/// represented by different economic operators; the leaf is the same binding
/// with an empty child set.
fn recursive_layout() -> ArenaTreeLayout {
    let columns = cols();
    let leaf = |slot: u32, id: u32| HierarchyNode {
        participant_slot: SlotIndex::new(slot),
        hosted_simthing_id: SimThingId::from_session_raw(id),
        depth: 2,
        children: vec![],
        cols: columns,
    };
    let interior = HierarchyNode {
        participant_slot: SlotIndex::new(1),
        hosted_simthing_id: SimThingId::from_session_raw(101),
        depth: 1,
        children: vec![leaf(2, 102), leaf(3, 103)],
        cols: columns,
    };
    build_custom_layout(
        0,
        &GpuArenaDescriptor {
            name: "recursive-resource-filter-formalization".into(),
            flow_property_id: SimPropertyId(1),
            balance_property_id: None,
            max_participants: 4,
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
            hosted_simthing_id: SimThingId::from_session_raw(100),
            depth: 0,
            children: vec![interior],
            cols: columns,
        }],
    )
    .expect("one admitted recursive filter layout")
}

fn born_values(weights: [f32; 2]) -> HashMap<CellKey, f32> {
    HashMap::from([
        ((SlotIndex::new(0), cols().intrinsic_flow_col), 50.0),
        ((SlotIndex::new(2), cols().weight_col), weights[0]),
        ((SlotIndex::new(3), cols().weight_col), weights[1]),
    ])
}

/// The only executable reference `R`: delegate to the already-generalized
/// share-vector oracle. This wrapper exists solely to make the theorem shape
/// explicit; it contains no arithmetic or alternate economic authority.
fn execute_reference_r(
    layout: &ArenaTreeLayout,
    values: &mut HashMap<CellKey, f32>,
) -> ArenaAllocationOracleTrace {
    run_arena_allocation_oracle(layout, values, 1.0)
}

fn dense_values(values: &HashMap<CellKey, f32>) -> Vec<f32> {
    let mut dense = vec![0.0; 4 * N_DIMS as usize];
    for (&(slot, column), &value) in values {
        dense[(slot.raw() * N_DIMS + column.raw_u32()) as usize] = value;
    }
    dense
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
            "spawned_generation": 23
        }"#,
    )
    .expect("persisted recursive-filter semantic root")
}

fn budgets() -> ResidentClearingBudgets {
    ResidentClearingBudgets::new(2, 1, 32, 32, 32, 262_144, 1_048_576, 32_768, 64)
        .expect("graduated exact scratch rows")
}

fn semantic_plan() -> ResidentClearingPlan {
    let tree = loaded_tree();
    let realm = TreeRealmId::from_u128(0x15_00).unwrap();
    let generation = TreeGenerationAuthority::new(GenerationStamp::new(23));
    let schedule = IntegrationSchedule::new();
    let registry = DimensionRegistry::new();
    let mut residency = SlotAllocator::new();
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
    let admissions = [1, 2].map(|draw| ResidentClearingAdmission {
        owner,
        resource: ResidentResourceId::new(1),
        scope: ResidentScopeId::new(1),
        draw: ResidentDrawId::new(draw),
    });
    ResidentClearingPlan::build(&binding, admissions, budgets()).unwrap()
}

fn semantic_row_for_draw(plan: &ResidentClearingPlan, draw: u64) -> u32 {
    plan.rows()
        .iter()
        .position(|row| plan.dictionaries().draws()[row.draw().get() as usize].get() == draw)
        .and_then(|row| u32::try_from(row).ok())
        .expect("draw has one canonical semantic row")
}

fn claims(
    plan: &ResidentClearingPlan,
    requests: [u32; 2],
    precedence: [u32; 2],
) -> Vec<ResidentApportionmentClaim> {
    [0, 1]
        .into_iter()
        .map(|index| {
            ResidentApportionmentClaim::new(
                semantic_row_for_draw(plan, 1 + index as u64),
                SimThingId::from_session_raw(1_000 + index),
                requests[index as usize],
                19,
                precedence[index as usize],
                SlotIndex::new(2 + index),
                cols().allocated_flow_col,
                simthing_kernel::ResidentExactBasisIdentity::LiveAllocatedFlow,
            )
        })
        .collect()
}

/// The only exact projection `Q`: delegate to the frozen Q149 oracle over the
/// `AllocatedFlow` vector emitted by `R`.
fn project_q(
    layout: &ArenaTreeLayout,
    semantic: &ResidentClearingPlan,
    values: &HashMap<CellKey, f32>,
    requests: [u32; 2],
    precedence: [u32; 2],
) -> Vec<ResidentConstrainedProduct> {
    let plan = plan_resident_exact_apportionment(
        layout,
        semantic,
        claims(semantic, requests, precedence),
        SimThingId::from_session_raw(7),
        GenerationStamp::new(23),
    )
    .unwrap();
    execute_resident_apportionment_cpu(&plan, &dense_values(values), N_DIMS).unwrap()
}

fn products(products: &[ResidentConstrainedProduct]) -> BTreeMap<u32, (u32, u32)> {
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

fn scope() -> OwnerChannelScopeKey {
    OwnerChannelScopeKey {
        owner_ref: OwnerRef::new("recursive-filter"),
        resource_key: ResourceKey::new("quanta"),
        scope_id: ScopeId::new("temporal-axis"),
    }
}

fn demand(key: &OwnerChannelScopeKey, requested: u32) -> RuntimeOwnerSiloDemandBucket {
    RuntimeOwnerSiloDemandBucket {
        owner_ref: key.owner_ref.clone(),
        resource_key: key.resource_key.clone(),
        scope_id: key.scope_id.clone(),
        requested,
        priority: 0,
        source_simthing_id_raw: Some(41),
    }
}

#[test]
fn recursive_resource_filter_formalization_terminal_referee() {
    let layout = recursive_layout();
    let plan = plan_arena_allocation(&layout, &[], 4).unwrap();
    let allocator_tree_ids: Vec<_> = plan
        .cpu_ops
        .iter()
        .filter_map(|op| match op.combine {
            CombineFn::EvalEML { tree_id } => Some(tree_id),
            _ => None,
        })
        .collect();
    assert_eq!(allocator_tree_ids.len(), 3);
    assert!(allocator_tree_ids.windows(2).all(|pair| pair[0] == pair[1]));

    let mut neutral_values = born_values([17.0, 33.0]);
    let neutral_trace = execute_reference_r(&layout, &mut neutral_values);
    assert_eq!(
        neutral_trace.disbursements,
        vec![
            (SlotIndex::new(0), SlotIndex::new(1), 50.0),
            (SlotIndex::new(1), SlotIndex::new(2), 17.0),
            (SlotIndex::new(1), SlotIndex::new(3), 33.0),
        ],
        "root and interior execute one R; leaves degenerate to no outgoing edge"
    );
    assert_eq!(
        neutral_trace
            .reductions
            .iter()
            .map(|(slot, _, pressure)| (slot.raw(), *pressure))
            .collect::<Vec<_>>(),
        vec![(1, 50.0), (0, 50.0)],
        "one direct-child P_up is published at every spatial edge"
    );

    let semantic = semantic_plan();
    let neutral_ts = project_q(&layout, &semantic, &neutral_values, [17, 33], [0, 0]);
    assert_eq!(
        products(&neutral_ts),
        BTreeMap::from([(1_000, (6, 11)), (1_001, (13, 20))])
    );

    // Multiplying every born eligible weight by one common positive scalar
    // leaves R's emitted F bits and Q's exact T_s bits unchanged. The
    // normalizing lambda is therefore implicit in weight/weight_sum, not a
    // runtime plane or stored response curve.
    let mut scaled_values = born_values([34.0, 66.0]);
    execute_reference_r(&layout, &mut scaled_values);
    assert_eq!(
        [2, 3].map(|slot| {
            neutral_values[&(SlotIndex::new(slot), cols().allocated_flow_col)].to_bits()
        }),
        [2, 3].map(|slot| {
            scaled_values[&(SlotIndex::new(slot), cols().allocated_flow_col)].to_bits()
        })
    );
    assert_eq!(
        neutral_ts,
        project_q(&layout, &semantic, &scaled_values, [17, 33], [0, 0])
    );

    // Continuous edge flow x alone is insufficient for the exact projection:
    // holding its bits, supply, identity, generation, and band fixed while
    // changing the already-admitted demand changes the request cap and U. The
    // necessary Q tuple `(requested, AllocatedFlow)` already exists; this does
    // not justify A(lambda) or any richer response state.
    let changed_p = project_q(&layout, &semantic, &neutral_values, [5, 45], [0, 0]);
    assert_ne!(products(&changed_p), products(&neutral_ts));

    // Hard precedence is an external ordered-band authority. It changes T_s
    // while P and F remain fixed, so encoding it as response curvature would
    // collapse two deliberately distinct projections.
    let hard_precedence = project_q(&layout, &semantic, &neutral_values, [17, 33], [0, 1]);
    assert_eq!(
        products(&hard_precedence),
        BTreeMap::from([(1_000, (17, 0)), (1_001, (2, 31))])
    );

    // Spatial exact recursion is literal type identity: T_s output is the
    // next edge's intake, with no adapter or role-newtype translation.
    let output: ResidentSettlementOutput = neutral_ts[0];
    let intake: ResidentRecursiveSupplyIntake = output;
    assert_eq!(intake, output);

    // Temporal recursion remains the one graduated Current-to-Next door.
    // d_effective(N+1) = d_authored(N+1) + U(N), exactly once.
    let key = scope();
    let current_demand = demand(&key, 10);
    let current_claim = ConstrainedClaim::from_runtime_demand(&current_demand, 1.0).unwrap();
    let authority = RuntimeRfDemandGenerationAuthority::new(ClearingRemainderAuthority {
        granter: SimThingId::from_session_raw(7),
        generation: GenerationStamp::new(23),
    });
    let (current, next) = produce_runtime_rf_next_generation_demands_for_tick(
        &authority,
        &[ConstrainedSupply {
            scope: key.clone(),
            available: 4,
        }],
        &[current_claim],
        &AuthoredClearingProgram::new(TransformOp::set(0.0)),
        vec![demand(&key, 2)],
    )
    .unwrap();
    assert_eq!(current[0].unresolved_total, 6);
    assert_eq!(next[0].generation(), GenerationStamp::new(24));
    assert_eq!(next[0].product().requested, 8);
    assert_eq!(
        produce_runtime_rf_next_generation_demands_for_tick(
            &authority,
            &[],
            &[],
            &AuthoredClearingProgram::new(TransformOp::set(0.0)),
            vec![],
        )
        .unwrap_err()
        .kind,
        RuntimeRfTickErrorKind::DemandCurrentToNextAlreadyProduced
    );

    println!(
        "RECURSIVE-RESOURCE-FILTER-FORMALIZATION R=PASS Q-compose=PASS P-F=sufficient-no-storage exact-tuple=existing lambda=implicit spatial=PASS temporal=PASS triad=reused-born-state"
    );
}
