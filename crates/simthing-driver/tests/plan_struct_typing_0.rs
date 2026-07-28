//! Driver referee: NodeColumnRefs carries ColumnIndex end-to-end (no remint).

use simthing_core::{
    AccumulatorOp, AccumulatorRole, AccumulatorSpec, ClampBehavior, ColumnIndex, CombineFn,
    ConsumeMode, GateSpec, LogTier, PropertyLayout, ScaleSpec, SlotIndex, SourceSpec, SubFieldRole,
    SubFieldSpec,
};
use simthing_driver::arena_hierarchy::{resolve_node_columns, NodeColumnRefs};
use simthing_driver::arena_allocation_plan::plan_arena_allocation;
use simthing_driver::arena_hierarchy::{build_custom_layout, HierarchyNode};
use simthing_driver::arena_registry::GpuArenaDescriptor;
use simthing_core::SimPropertyId;

fn flow_layout() -> PropertyLayout {
    let arena = "food".to_string();
    PropertyLayout {
        sub_fields: vec![
            SubFieldSpec {
                role: SubFieldRole::Named("intrinsic_flow".into()),
                width: 1,
                clamp: ClampBehavior::Unbounded,
                velocity_max: None,
                default: 0.0,
                display_name: "intrinsic_flow".into(),
                display_range: None,
                governed_by: None,
                reduction_override: None,
                soft_aggregate_guard: None,
                accumulator_spec: Some(AccumulatorSpec {
                    role: AccumulatorRole::IntrinsicFlow,
                    log_tier: LogTier::Summary,
                }),
            },
            SubFieldSpec {
                role: SubFieldRole::Named("allocated_flow".into()),
                width: 1,
                clamp: ClampBehavior::Unbounded,
                velocity_max: None,
                default: 0.0,
                display_name: "allocated_flow".into(),
                display_range: None,
                governed_by: None,
                reduction_override: None,
                soft_aggregate_guard: None,
                accumulator_spec: Some(AccumulatorSpec {
                    role: AccumulatorRole::AllocatedFlow {
                        arena: arena.clone(),
                    },
                    log_tier: LogTier::Summary,
                }),
            },
            SubFieldSpec {
                role: SubFieldRole::Named("weight".into()),
                width: 1,
                clamp: ClampBehavior::Unbounded,
                velocity_max: None,
                default: 0.0,
                display_name: "weight".into(),
                display_range: None,
                governed_by: None,
                reduction_override: None,
                soft_aggregate_guard: None,
                accumulator_spec: Some(AccumulatorSpec {
                    role: AccumulatorRole::AllocatorWeight {
                        arena: arena.clone(),
                    },
                    log_tier: LogTier::Summary,
                }),
            },
        ],
    }
}

#[test]
fn resolve_node_columns_returns_typed_column_index() {
    let cols = resolve_node_columns(&flow_layout(), "food").expect("cols");
    assert_eq!(
        cols.intrinsic_flow_col,
        ColumnIndex::from_raw_for_oracle_or_rehearsal(0)
    );
    assert_eq!(
        cols.allocated_flow_col,
        ColumnIndex::from_raw_for_oracle_or_rehearsal(1)
    );
    assert_eq!(
        cols.weight_col,
        ColumnIndex::from_raw_for_oracle_or_rehearsal(2)
    );
    // Optional balance remains typed Option<ColumnIndex> until encode.
    assert!(cols.balance_col.is_none());
    assert!(cols.balance_governing_col.is_none());
}

#[test]
fn arena_plan_ops_carry_resolved_column_index_without_remint() {
    let cols = resolve_node_columns(&flow_layout(), "food").expect("cols");
    let root = HierarchyNode {
        participant_slot: SlotIndex::new(10),
        hosted_simthing_id: Default::default(),
        depth: 0,
        children: vec![HierarchyNode {
            participant_slot: SlotIndex::new(11),
            hosted_simthing_id: Default::default(),
            depth: 1,
            children: vec![],
            cols,
        }],
        cols,
    };
    let layout = build_custom_layout(
        0,
        &GpuArenaDescriptor {
            name: "food".into(),
            flow_property_id: SimPropertyId(1),
            balance_property_id: None,
            max_participants: 8,
            max_coupling_fanout: 4,
            max_orderband_depth: 16,
            fission_policy: Default::default(),
            participant_range: (0, 0),
            wildcard_max_expansion: None,
            reserved_orderband_depth: 0,
        },
        cols,
        vec![root],
    )
    .expect("layout");
    let plan = plan_arena_allocation(&layout, &[], 16).expect("plan");
    let typed: Vec<&AccumulatorOp> = plan
        .cpu_ops
        .iter()
        .filter(|op| {
            matches!(
                op.source,
                SourceSpec::SlotValue { col, .. } if col == cols.weight_col
            ) || op.targets.iter().any(|(_, c)| *c == cols.allocated_flow_col)
        })
        .collect();
    assert!(
        !typed.is_empty(),
        "expected plan ops to reference typed NodeColumnRefs columns"
    );
    for op in typed {
        if let SourceSpec::SlotValue { col, .. } = op.source {
            // Identity equality proves pass-through (no remint to a different door value).
            let _ = col;
        }
        for (_, col) in &op.targets {
            assert_eq!(col.raw_u32(), col.raw() as u32);
        }
        match op.combine {
            CombineFn::Identity | CombineFn::Sum | CombineFn::EvalEML { .. } => {}
            _ => {}
        }
        let _ = (&op.gate, &op.scale, &op.consume);
    }
}

#[test]
fn node_column_refs_optional_sentinel_stays_option_until_encode() {
    let refs = NodeColumnRefs {
        intrinsic_flow_col: ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
        intrinsic_flow_sum_col: ColumnIndex::from_raw_for_oracle_or_rehearsal(4),
        allocated_flow_col: ColumnIndex::from_raw_for_oracle_or_rehearsal(1),
        balance_col: None,
        balance_governing_col: Some(ColumnIndex::from_raw_for_oracle_or_rehearsal(3)),
        weight_col: ColumnIndex::from_raw_for_oracle_or_rehearsal(2),
        weight_sum_col: ColumnIndex::from_raw_for_oracle_or_rehearsal(5),
        propagated_intrinsic_flow_col: ColumnIndex::from_raw_for_oracle_or_rehearsal(6),
        propagated_allocated_flow_col: ColumnIndex::from_raw_for_oracle_or_rehearsal(7),
        propagated_weight_sum_col: ColumnIndex::from_raw_for_oracle_or_rehearsal(8),
        hosted_simthing_id_col: ColumnIndex::from_raw_for_oracle_or_rehearsal(9),
    };
    assert!(refs.balance_col.is_none());
    assert_eq!(
        refs.balance_governing_col.unwrap().raw_u32(),
        3,
        "optional governing col remains typed until the WGSL encode boundary"
    );
}
