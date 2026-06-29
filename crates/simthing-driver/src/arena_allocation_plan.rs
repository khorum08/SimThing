//! E-11 AccumulatorOp planner (memo §2.3).

use simthing_core::{
    AccumulatorOp, ColumnIndex, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SlotIndex, SourceSpec,
};
use simthing_gpu::{plan_governed_integration_at_band, GovernedPair, PlannerError};
use thiserror::Error;

use crate::arena_hierarchy::{ArenaTreeLayout, HierarchyError, HierarchyNode, NodeColumnRefs};
use crate::arena_registry::SlotId;
use crate::child_share_eml::child_share_tree_id;

#[derive(Clone, Debug, PartialEq)]
pub struct ArenaAllocationPlan {
    pub cpu_ops: Vec<AccumulatorOp>,
    pub n_bands: u32,
    pub integration_band: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum AllocationPlanError {
    #[error(transparent)]
    Hierarchy(#[from] HierarchyError),
    #[error(transparent)]
    Integration(#[from] PlannerError),
    #[error("non-contiguous participant children for parent slot {parent_slot}")]
    NonContiguousChildren { parent_slot: SlotId },
}

pub fn plan_arena_allocation(
    layout: &ArenaTreeLayout,
    governed_pairs: &[GovernedPair],
    n_slots: u32,
) -> Result<ArenaAllocationPlan, AllocationPlanError> {
    for node in layout.iter_all() {
        node.verify_child_contiguity().map_err(|e| match e {
            HierarchyError::NonContiguousChildren { parent_slot } => {
                AllocationPlanError::NonContiguousChildren { parent_slot }
            }
            other => AllocationPlanError::Hierarchy(other),
        })?;
    }

    let mut ops_cpu = Vec::new();
    let bands = layout.band_layout;
    let d = layout.max_depth;

    if d > 1 {
        for node in layout.iter_all() {
            for col in reset_columns(node.cols) {
                ops_cpu.push(reset_op(node.participant_slot.raw(), col, bands.reset_band));
            }
        }

        for depth in (0..d.saturating_sub(1)).rev() {
            let band = bands.upsweep_band(depth, d);
            for parent in layout.iter_at_depth(depth) {
                if parent.children.is_empty() {
                    continue;
                }
                let (start, count) = child_range(parent);
                ops_cpu.push(sum_reduction_op(
                    start,
                    count,
                    parent.participant_slot.raw(),
                    parent.cols.intrinsic_flow_col,
                    parent.cols.intrinsic_flow_sum_col,
                    band,
                ));
                ops_cpu.push(sum_reduction_op(
                    start,
                    count,
                    parent.participant_slot.raw(),
                    parent.cols.weight_col,
                    parent.cols.weight_sum_col,
                    band,
                ));
            }
        }

        for depth in 0..d.saturating_sub(1) {
            let broadcast_band = bands.broadcast_band(depth, d);
            let disburse_band = bands.disburse_band(depth, d);
            for parent in layout.iter_at_depth(depth) {
                if parent.children.is_empty() {
                    continue;
                }
                let p_if = if depth == 0 {
                    parent.cols.intrinsic_flow_col
                } else {
                    parent.cols.intrinsic_flow_sum_col
                };
                let p_ws = parent.cols.weight_sum_col;
                for child in &parent.children {
                    ops_cpu.push(broadcast_op(
                        parent.participant_slot.raw(),
                        p_if,
                        child.participant_slot.raw(),
                        child.cols.propagated_intrinsic_flow_col,
                        broadcast_band,
                    ));
                    if depth == 0 {
                        ops_cpu.push(const_broadcast_op(
                            0.0,
                            child.participant_slot.raw(),
                            child.cols.propagated_allocated_flow_col,
                            broadcast_band,
                        ));
                    } else {
                        ops_cpu.push(broadcast_op(
                            parent.participant_slot.raw(),
                            parent.cols.allocated_flow_col,
                            child.participant_slot.raw(),
                            child.cols.propagated_allocated_flow_col,
                            broadcast_band,
                        ));
                    }
                    ops_cpu.push(broadcast_op(
                        parent.participant_slot.raw(),
                        p_ws,
                        child.participant_slot.raw(),
                        child.cols.propagated_weight_sum_col,
                        broadcast_band,
                    ));
                }
            }
            for parent in layout.iter_at_depth(depth) {
                for child in &parent.children {
                    ops_cpu.push(disburse_op(
                        child.participant_slot.raw(),
                        child.cols.allocated_flow_col,
                        disburse_band,
                    ));
                }
            }
        }
    }

    let participant_slots: Vec<u32> = layout
        .participant_slots()
        .into_iter()
        .map(SlotId::raw)
        .collect();
    let integration = plan_governed_integration_at_band(
        governed_pairs,
        n_slots,
        bands.integration_band,
        if participant_slots.is_empty() {
            None
        } else {
            Some(participant_slots.as_slice())
        },
    )?;

    for gpu in &integration.ops {
        ops_cpu.push(cpu_op_from_integration_gpu(gpu));
    }

    Ok(ArenaAllocationPlan {
        cpu_ops: ops_cpu,
        n_bands: bands.total_bands_used,
        integration_band: bands.integration_band,
    })
}

fn cpu_op_from_integration_gpu(gpu: &simthing_gpu::AccumulatorOpGpu) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: SlotIndex::new(gpu.source_slot),
            col: ColumnIndex::new(gpu.source_col as usize),
        },
        combine: CombineFn::IntegrateWithClamp {
            dt: 0.0,
            vel_max: f32::from_bits(gpu.combine_a),
            amount_min: f32::from_bits(gpu.combine_b),
            amount_max: f32::from_bits(gpu.combine_c),
        },
        gate: GateSpec::OrderBand(gpu.gate_a),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::None,
        targets: vec![(
            SlotIndex::new(gpu.target0_slot),
            ColumnIndex::new(gpu.target0_col as usize),
        )],
    }
}

fn reset_columns(cols: NodeColumnRefs) -> Vec<u32> {
    vec![
        cols.allocated_flow_col,
        cols.intrinsic_flow_sum_col,
        cols.weight_sum_col,
        cols.propagated_intrinsic_flow_col,
        cols.propagated_allocated_flow_col,
        cols.propagated_weight_sum_col,
    ]
}

fn child_range(parent: &HierarchyNode) -> (u32, u32) {
    let start = parent.children[0].participant_slot.raw();
    let count = parent.children.len() as u32;
    (start, count)
}

fn reset_op(slot: u32, col: u32, band: u32) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::Constant(0.0),
        combine: CombineFn::Identity,
        gate: GateSpec::OrderBand(band),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets: vec![(SlotIndex::new(slot), ColumnIndex::new(col as usize))],
    }
}

fn sum_reduction_op(
    start: u32,
    count: u32,
    parent_slot: u32,
    source_col: u32,
    target_col: u32,
    band: u32,
) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::SlotRange {
            start: SlotIndex::new(start),
            count,
            col: ColumnIndex::new(source_col as usize),
        },
        combine: CombineFn::Sum,
        gate: GateSpec::OrderBand(band),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets: vec![(
            SlotIndex::new(parent_slot),
            ColumnIndex::new(target_col as usize),
        )],
    }
}

fn broadcast_op(
    src_slot: u32,
    src_col: u32,
    dst_slot: u32,
    dst_col: u32,
    band: u32,
) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: SlotIndex::new(src_slot),
            col: ColumnIndex::new(src_col as usize),
        },
        combine: CombineFn::Identity,
        gate: GateSpec::OrderBand(band),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets: vec![(SlotIndex::new(dst_slot), ColumnIndex::new(dst_col as usize))],
    }
}

fn const_broadcast_op(value: f32, dst_slot: u32, dst_col: u32, band: u32) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::Constant(value),
        combine: CombineFn::Identity,
        gate: GateSpec::OrderBand(band),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets: vec![(SlotIndex::new(dst_slot), ColumnIndex::new(dst_col as usize))],
    }
}

fn disburse_op(child_slot: u32, a_f_col: u32, band: u32) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: SlotIndex::new(child_slot),
            col: ColumnIndex::new(0),
        },
        combine: CombineFn::EvalEML {
            tree_id: child_share_tree_id().0,
        },
        gate: GateSpec::OrderBand(band),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::AddToTarget,
        targets: vec![(
            SlotIndex::new(child_slot),
            ColumnIndex::new(a_f_col as usize),
        )],
    }
}

pub fn max_disbursement_band(layout: &ArenaTreeLayout) -> u32 {
    if layout.max_depth <= 1 {
        return 0;
    }
    layout
        .band_layout
        .disburse_band(layout.max_depth.saturating_sub(2), layout.max_depth)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arena_hierarchy::{build_custom_layout, HierarchyNode};
    use crate::arena_registry::GpuArenaDescriptor;
    use simthing_core::SimPropertyId;

    fn cols() -> NodeColumnRefs {
        NodeColumnRefs {
            intrinsic_flow_col: 0,
            intrinsic_flow_sum_col: 4,
            allocated_flow_col: 1,
            balance_col: Some(3),
            weight_col: 2,
            weight_sum_col: 5,
            propagated_intrinsic_flow_col: 6,
            propagated_allocated_flow_col: 7,
            propagated_weight_sum_col: 8,
            hosted_simthing_id_col: 9,
        }
    }

    fn d2_layout() -> ArenaTreeLayout {
        let c = cols();
        let root = HierarchyNode {
            participant_slot: 10,
            hosted_simthing_id: Default::default(),
            depth: 0,
            children: vec![HierarchyNode {
                participant_slot: 11,
                hosted_simthing_id: Default::default(),
                depth: 1,
                children: vec![],
                cols: c,
                gap_used: 0,
            }],
            cols: c,
            gap_used: 0,
        };
        build_custom_layout(
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
            c,
            Default::default(),
            9,
            vec![root],
        )
        .unwrap()
    }

    #[test]
    fn plan_emits_separate_broadcast_and_disburse_bands() {
        let layout = d2_layout();
        let plan = plan_arena_allocation(&layout, &[], 16).unwrap();
        let broadcast_band = layout.band_layout.broadcast_band(0, 2);
        let disburse_band = layout.band_layout.disburse_band(0, 2);
        let broadcast = plan
            .cpu_ops
            .iter()
            .filter(|op| matches!(op.gate, GateSpec::OrderBand(b) if b == broadcast_band))
            .count();
        let disburse = plan
            .cpu_ops
            .iter()
            .filter(|op| matches!(op.gate, GateSpec::OrderBand(b) if b == disburse_band))
            .count();
        assert!(broadcast > 0);
        assert!(disburse > 0);
        assert!(plan
            .cpu_ops
            .iter()
            .any(|op| matches!(op.combine, CombineFn::EvalEML { .. })));
    }

    #[test]
    fn integration_band_follows_deepest_disbursement() {
        let layout = d2_layout();
        let plan = plan_arena_allocation(&layout, &[], 16).unwrap();
        assert_eq!(plan.integration_band, max_disbursement_band(&layout) + 1);
    }
}
