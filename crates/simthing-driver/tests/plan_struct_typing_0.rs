//! Driver referee: NodeColumnRefs carries ColumnIndex end-to-end (no remint).

use simthing_core::SimPropertyId;
use simthing_core::{
    AccumulatorOp, AccumulatorRole, AccumulatorSpec, ClampBehavior, ColumnIndex, CombineFn,
    ConsumeMode, GateSpec, LogTier, PropertyColumnRange, PropertyLayout, ScaleSpec, SlotIndex,
    SourceSpec, SubFieldRole, SubFieldSpec,
};
use simthing_driver::arena_allocation_plan::plan_arena_allocation;
use simthing_driver::arena_hierarchy::{build_custom_layout, HierarchyNode};
use simthing_driver::arena_hierarchy::{resolve_node_columns, NodeColumnRefs};
use simthing_driver::arena_registry::GpuArenaDescriptor;

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

fn resolve_flow_cols() -> NodeColumnRefs {
    let layout = flow_layout();
    let range = PropertyColumnRange {
        start: 0,
        stride: layout.stride(),
    };
    resolve_node_columns(&range, &layout, "food").expect("cols")
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
