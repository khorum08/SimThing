//! RECURSIVE-RESOURCE-FILTER-UNIFICATION-0 closing referee.
//!
//! The executable proofs remain the graduated R, Q, resident, persistence,
//! parity, and causal witnesses. This referee proves that the new canonical
//! vocabulary is conversion-free and that the audited peer public/runtime
//! authority surface strictly decreased while the five CPU doors stayed
//! quarantined.

use std::any::TypeId;
use std::collections::HashMap;

use simthing_core::{ColumnIndex, SimPropertyId, SimThingId, SlotIndex};
use simthing_driver::resident_clearing_runtime::{
    RecursiveResourceFilterRuntime, ResidentClearingRuntime,
};
use simthing_driver::{
    build_custom_layout, evaluate_recursive_resource_filter_oracle, run_arena_allocation_oracle,
    ArenaTreeLayout, FissionPolicy, GpuArenaDescriptor, HierarchyNode, NodeColumnRefs,
};

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

fn chain_node(slot: u32, depth: u32, child: Option<HierarchyNode>) -> HierarchyNode {
    HierarchyNode {
        participant_slot: SlotIndex::new(slot),
        hosted_simthing_id: SimThingId::from_session_raw(100 + slot),
        depth,
        children: child.into_iter().collect(),
        cols: cols(),
    }
}

fn three_edge_layout() -> ArenaTreeLayout {
    let leaf = chain_node(3, 3, None);
    let lower = chain_node(2, 2, Some(leaf));
    let upper = chain_node(1, 1, Some(lower));
    build_custom_layout(
        0,
        &GpuArenaDescriptor {
            name: "recursive-resource-filter-unification".into(),
            flow_property_id: SimPropertyId(1),
            balance_property_id: None,
            max_participants: 4,
            max_coupling_fanout: 1,
            max_orderband_depth: 24,
            fission_policy: FissionPolicy::default(),
            participant_range: (0, 0),
            wildcard_max_expansion: None,
            reserved_orderband_depth: 0,
        },
        cols(),
        vec![chain_node(0, 0, Some(upper))],
    )
    .expect("one recursive filter spans three edges")
}

fn born_values() -> HashMap<CellKey, f32> {
    let mut values = HashMap::new();
    values.insert((SlotIndex::new(0), cols().intrinsic_flow_col), 13.0);
    values.insert((SlotIndex::new(3), cols().weight_col), 1.0);
    values
}

#[test]
fn canonical_alias_is_bit_identity_over_three_recursive_edges() {
    assert_eq!(
        TypeId::of::<RecursiveResourceFilterRuntime>(),
        TypeId::of::<ResidentClearingRuntime>(),
        "the canonical production name is the resident runtime itself"
    );
    assert_eq!(
        std::mem::size_of::<RecursiveResourceFilterRuntime>(),
        std::mem::size_of::<ResidentClearingRuntime>(),
        "the alias adds zero resident state per node or program family"
    );
    assert!(std::ptr::fn_addr_eq(
        evaluate_recursive_resource_filter_oracle
            as fn(&ArenaTreeLayout, &mut HashMap<CellKey, f32>, f32) -> _,
        run_arena_allocation_oracle as fn(&ArenaTreeLayout, &mut HashMap<CellKey, f32>, f32) -> _
    ));

    let layout = three_edge_layout();
    let mut graduated = born_values();
    let graduated_trace = run_arena_allocation_oracle(&layout, &mut graduated, 1.0);
    let mut canonical = born_values();
    let canonical_trace = evaluate_recursive_resource_filter_oracle(&layout, &mut canonical, 1.0);

    let bits = |values: &HashMap<CellKey, f32>| {
        [0, 1, 2, 3].map(|slot| {
            values
                .get(&(SlotIndex::new(slot), cols().allocated_flow_col))
                .copied()
                .unwrap_or_default()
                .to_bits()
        })
    };
    assert_eq!(bits(&canonical), bits(&graduated));
    assert_eq!(canonical_trace.resets, graduated_trace.resets);
    assert_eq!(canonical_trace.reductions, graduated_trace.reductions);
    assert_eq!(
        canonical_trace.disbursements,
        vec![
            (SlotIndex::new(0), SlotIndex::new(1), 13.0),
            (SlotIndex::new(1), SlotIndex::new(2), 13.0),
            (SlotIndex::new(2), SlotIndex::new(3), 13.0),
        ],
        "each child consumes the same edge flow as supply with no intermediary representation"
    );
    assert_eq!(
        canonical_trace.reductions,
        vec![
            (SlotIndex::new(2), 0.0, 1.0),
            (SlotIndex::new(1), 0.0, 1.0),
            (SlotIndex::new(0), 0.0, 1.0),
        ],
        "each subtree query uses the born direct-child sufficient statistic"
    );

    println!(
        "RECURSIVE-RESOURCE-FILTER-UNIFICATION alias=IDENTITY edges=3 state=O(1) subtree=born-statistic operator=one"
    );
}

fn has_public_fn(source: &str, symbol: &str) -> bool {
    let declaration = format!("pub fn {symbol}");
    source
        .lines()
        .map(str::trim_start)
        .any(|line| line.starts_with(&declaration))
}

#[test]
fn peer_authority_deletion_and_five_door_quarantine_are_exact() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let read = |path: &str| std::fs::read_to_string(root.join(path)).unwrap();

    let owner_disburse = read("crates/simthing-spec/src/spec/owner_silo_disburse_down.rs");
    let owner_report = read("crates/simthing-spec/src/spec/owner_silo_recursive_rf_source.rs");
    let local_report =
        read("crates/simthing-spec/src/spec/local_allocation_recursive_rf_source.rs");
    let compile = read("crates/simthing-driver/src/owner_silo_disburse_down_compile.rs");
    let audited = [
        (
            "apply_owner_silo_runtime_disburse_down_cpu",
            &owner_disburse,
        ),
        ("compile_owner_silo_disburse_down_plan", &compile),
        (
            "compile_owner_silo_disburse_down_plan_from_owner_view",
            &compile,
        ),
        (
            "evaluate_owner_silo_disburse_down_with_rf_source",
            &owner_report,
        ),
        (
            "runtime_local_allocation_from_owner_silo_disburse_report",
            &local_report,
        ),
    ];
    let before = audited.len();
    let after: Vec<_> = audited
        .iter()
        .filter(|(symbol, source)| has_public_fn(source, symbol))
        .map(|(symbol, _)| *symbol)
        .collect();
    assert_eq!(
        after,
        vec![
            "apply_owner_silo_runtime_disburse_down_cpu",
            "evaluate_owner_silo_disburse_down_with_rf_source",
        ],
        "only still-required quarantined oracle/report witnesses remain public"
    );
    assert_eq!(before, 5);
    assert!(after.len() < before);

    let oracle = read("crates/simthing-driver/src/arena_allocation_oracle.rs");
    let resident = read("crates/simthing-driver/src/resident_clearing_runtime.rs");
    assert!(oracle.contains(
        "pub use run_arena_allocation_oracle as evaluate_recursive_resource_filter_oracle;"
    ));
    assert!(resident.contains("pub type RecursiveResourceFilterRuntime = ResidentClearingRuntime;"));
    assert!(!oracle.contains("pub fn evaluate_recursive_resource_filter_oracle"));
    assert!(!resident.contains("pub struct RecursiveResourceFilterRuntime"));

    let constrained = read("crates/simthing-spec/src/spec/constrained_clearing.rs");
    let temporal = read("crates/simthing-spec/src/spec/runtime_rf_tick.rs");
    let five_doors = [
        ("clear_constrained_claims_at_generation", &constrained),
        ("clear_reduced_owner_channels", &constrained),
        ("clear_reduced_owner_channels_at_generation", &constrained),
        ("clear_stamped_owner_channels", &constrained),
        ("produce_runtime_rf_next_generation_demands", &temporal),
    ];
    assert!(five_doors
        .iter()
        .all(|(symbol, source)| has_public_fn(source, symbol)));

    let growth = read("crates/simthing-driver/src/growth_entitlement.rs");
    let tick = read("crates/simthing-driver/src/runtime_rf_tick_compile.rs");
    assert_eq!(
        growth
            .matches("clear_constrained_claims_at_generation(")
            .count(),
        1
    );
    assert_eq!(
        tick.matches("produce_runtime_rf_next_generation_demands(")
            .count(),
        1
    );
    for (symbol, _) in &five_doors[1..4] {
        assert!(!growth.contains(symbol));
        assert!(!tick.contains(symbol));
    }

    println!(
        "RECURSIVE-RESOURCE-FILTER-DELETION before=5 after=2 strict-decrease=3 aliases=2 new-economic-authorities=0 cpu-doors=5-quarantined"
    );
}
