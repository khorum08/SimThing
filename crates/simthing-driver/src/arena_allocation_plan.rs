//! E-11 AccumulatorOp planner (memo §2.3).

use simthing_core::{
    AccumulatorOp, ColumnIndex, CombineFn, ConsumeMode, GateSpec, GenerationStamp, InputSpec,
    ScaleSpec, SlotIndex, SourceSpec,
};
use simthing_gpu::{
    column_from_wire, plan_governed_integration_at_band, GovernedPair, PlannerError,
};
use thiserror::Error;

use crate::arena_hierarchy::{ArenaTreeLayout, HierarchyError, HierarchyNode, NodeColumnRefs};
use crate::arena_registry::SlotId;
use crate::child_share_eml::child_share_tree_id;
use crate::{ActionBandActiveInstance, CompiledActionBandConservedProgressBinding};

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
    #[error(transparent)]
    NeutralPressure(#[from] crate::need_binding::NeutralPressureBindingError),
    #[error("more than one born Gu-Yang pressure product targets arena participant slot {slot}")]
    DuplicateImmediateFlowPressureTarget { slot: u32 },
}

pub fn plan_arena_allocation(
    layout: &ArenaTreeLayout,
    governed_pairs: &[GovernedPair],
    n_slots: u32,
) -> Result<ArenaAllocationPlan, AllocationPlanError> {
    plan_arena_allocation_with_pressure(
        layout,
        governed_pairs,
        n_slots,
        &[],
        &[],
        GenerationStamp::new(0),
        GenerationStamp::new(1),
    )
}

/// Ordinary arena allocation planner with default-on native market pressure.
///
/// Immediate-flow pressure is selected only from sealed ActionBand Gu-Yang
/// bindings associated with an admitted participant instance. Entitlement-first
/// raw `P` needs no parallel binding: the rows-2/8 direct-child `Sum` below is
/// its plan-owned producer and publishes directly to the existing weight lane.
pub fn plan_arena_allocation_with_pressure(
    layout: &ArenaTreeLayout,
    governed_pairs: &[GovernedPair],
    n_slots: u32,
    conserved_progress_bindings: &[CompiledActionBandConservedProgressBinding],
    active_instances: &[ActionBandActiveInstance],
    observed_generation: GenerationStamp,
    allocation_generation: GenerationStamp,
) -> Result<ArenaAllocationPlan, AllocationPlanError> {
    let mut ops_cpu = Vec::new();
    let bands = layout.band_layout;
    let d = layout.max_depth;

    if d > 1 {
        for node in layout.iter_all() {
            for col in reset_columns(node.cols) {
                ops_cpu.push(reset_op(node.participant_slot.raw(), col, bands.reset_band));
            }
        }

        append_immediate_flow_pressure_ops(
            layout,
            conserved_progress_bindings,
            active_instances,
            observed_generation,
            allocation_generation,
            bands.reset_band,
            &mut ops_cpu,
        )?;

        for depth in (0..d.saturating_sub(1)).rev() {
            let band = bands.upsweep_band(depth, d);
            for parent in layout.iter_at_depth(depth) {
                if parent.children.is_empty() {
                    continue;
                }
                ops_cpu.extend(sum_reduction_ops(
                    parent,
                    parent.participant_slot.raw(),
                    parent.cols.intrinsic_flow_col,
                    parent.cols.intrinsic_flow_sum_col,
                    band,
                ));
                // The child's AllocatorWeight is the branch-pressure carrier.
                // Reduce it exactly once over this parent's direct children,
                // then publish the one result to both existing allocator
                // operands. At the next shallower band, this parent's weight
                // is itself one direct-child contribution; descendants are
                // never recounted or scanned independently.
                ops_cpu.extend(sum_reduction_to_targets_ops(
                    parent,
                    parent.cols.weight_col,
                    vec![
                        (parent.participant_slot, parent.cols.weight_col),
                        (parent.participant_slot, parent.cols.weight_sum_col),
                    ],
                    band,
                ));
            }
        }

        for depth in 0..d.saturating_sub(1) {
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
                    ops_cpu.push(disburse_op(parent, child, p_if, p_ws, disburse_band));
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

fn append_immediate_flow_pressure_ops(
    layout: &ArenaTreeLayout,
    conserved_progress_bindings: &[CompiledActionBandConservedProgressBinding],
    active_instances: &[ActionBandActiveInstance],
    observed_generation: GenerationStamp,
    allocation_generation: GenerationStamp,
    band: u32,
    ops: &mut Vec<AccumulatorOp>,
) -> Result<(), AllocationPlanError> {
    use simthing_spec::AdmittedActionBandConservedProgressBoundSource;
    use std::collections::HashSet;

    let leaves = layout
        .iter_all()
        .into_iter()
        .filter(|node| node.children.is_empty())
        .map(|node| (node.participant_slot, node.cols.weight_col))
        .collect::<std::collections::HashMap<_, _>>();
    let mut targeted = HashSet::new();
    for binding in conserved_progress_bindings
        .iter()
        .copied()
        .filter(|binding| {
            matches!(
                binding.bound_source(),
                AdmittedActionBandConservedProgressBoundSource::GuYangAvailable(_)
                    | AdmittedActionBandConservedProgressBoundSource::GuYangRealized(_)
            ) && binding.destination() == simthing_gpu::ActionBandEmissionDestination::RfClaim
        })
    {
        for instance in active_instances
            .iter()
            .copied()
            .filter(|instance| instance.template() == binding.template())
        {
            let Some(&weight_col) = leaves.get(&instance.slot()) else {
                continue;
            };
            if !targeted.insert(instance.slot()) {
                return Err(AllocationPlanError::DuplicateImmediateFlowPressureTarget {
                    slot: instance.slot().raw(),
                });
            }
            ops.push(
                crate::need_binding::bind_immediate_flow_pressure_to_allocator_weight(
                    binding,
                    instance,
                    instance.slot(),
                    weight_col,
                    observed_generation,
                    allocation_generation,
                    band,
                )?,
            );
        }
    }
    Ok(())
}

fn cpu_op_from_integration_gpu(gpu: &simthing_gpu::AccumulatorOpGpu) -> AccumulatorOp {
    let encoded_targets = [
        (gpu.target0_slot, gpu.target0_col),
        (gpu.target1_slot, gpu.target1_col),
        (gpu.target2_slot, gpu.target2_col),
        (gpu.target3_slot, gpu.target3_col),
    ];
    let targets = encoded_targets
        .iter()
        .take(gpu.n_targets.min(encoded_targets.len() as u32) as usize)
        .map(|(slot, col)| (SlotIndex::new(*slot), column_from_wire(*col)))
        .collect();
    AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: SlotIndex::new(gpu.source_slot),
            col: column_from_wire(gpu.source_col),
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
        targets,
    }
}

pub(crate) fn append_residual_closure_ops(
    layout: &ArenaTreeLayout,
    ops_cpu: &mut Vec<AccumulatorOp>,
) {
    if layout.max_depth <= 1
        || !layout
            .iter_all()
            .iter()
            .any(|node| node.cols.balance_governing_col.is_some())
    {
        return;
    }

    let seed_band = layout.band_layout.integration_band - 4;
    let add_allocated_band = seed_band + 1;
    let sum_children_band = seed_band + 3;
    for parent in layout
        .iter_all()
        .into_iter()
        .filter(|node| node.is_interior())
    {
        let Some(rate_col) = parent.cols.balance_governing_col else {
            continue;
        };
        let budget_intrinsic_col = if parent.depth == 0 {
            parent.cols.intrinsic_flow_col
        } else {
            parent.cols.intrinsic_flow_sum_col
        };
        ops_cpu.push(slot_value_op(
            parent.participant_slot.raw(),
            budget_intrinsic_col,
            parent.participant_slot.raw(),
            rate_col,
            seed_band,
            ConsumeMode::ResetTarget,
            ScaleSpec::Identity,
        ));
        ops_cpu.push(slot_value_op(
            parent.participant_slot.raw(),
            parent.cols.allocated_flow_col,
            parent.participant_slot.raw(),
            rate_col,
            add_allocated_band,
            ConsumeMode::AddToTarget,
            ScaleSpec::Identity,
        ));
        ops_cpu.extend(sum_accumulation_ops(
            parent,
            parent.participant_slot.raw(),
            parent.cols.allocated_flow_col,
            rate_col,
            sum_children_band,
            ScaleSpec::Constant(-1.0),
        ));
    }
}

fn reset_columns(cols: NodeColumnRefs) -> Vec<ColumnIndex> {
    vec![
        cols.allocated_flow_col,
        cols.intrinsic_flow_sum_col,
        cols.weight_sum_col,
    ]
}

fn child_range(parent: &HierarchyNode) -> (u32, u32) {
    let start = parent.children[0].participant_slot.raw();
    let count = parent.children.len() as u32;
    (start, count)
}

fn children_are_contiguous(parent: &HierarchyNode) -> bool {
    parent
        .children
        .windows(2)
        .all(|pair| pair[1].participant_slot.raw() == pair[0].participant_slot.raw() + 1)
}

fn reset_op(slot: u32, col: ColumnIndex, band: u32) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::Constant(0.0),
        combine: CombineFn::Identity,
        gate: GateSpec::OrderBand(band),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets: vec![(SlotIndex::new(slot), col)],
    }
}

fn sum_reduction_ops(
    parent: &HierarchyNode,
    parent_slot: u32,
    source_col: ColumnIndex,
    target_col: ColumnIndex,
    band: u32,
) -> Vec<AccumulatorOp> {
    sum_reduction_to_targets_ops(
        parent,
        source_col,
        vec![(SlotIndex::new(parent_slot), target_col)],
        band,
    )
}

fn sum_reduction_to_targets_ops(
    parent: &HierarchyNode,
    source_col: ColumnIndex,
    targets: Vec<(SlotIndex, ColumnIndex)>,
    band: u32,
) -> Vec<AccumulatorOp> {
    if children_are_contiguous(parent) {
        let (start, count) = child_range(parent);
        return vec![AccumulatorOp {
            source: SourceSpec::SlotRange {
                start: SlotIndex::new(start),
                count,
                col: source_col,
            },
            combine: CombineFn::Sum,
            gate: GateSpec::OrderBand(band),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets,
        }];
    }
    vec![AccumulatorOp {
        source: sparse_child_input_list(parent, source_col),
        combine: CombineFn::Sum,
        gate: GateSpec::OrderBand(band),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets,
    }]
}

fn slot_value_op(
    src_slot: u32,
    src_col: ColumnIndex,
    dst_slot: u32,
    dst_col: ColumnIndex,
    band: u32,
    consume: ConsumeMode,
    scale: ScaleSpec,
) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: SlotIndex::new(src_slot),
            col: src_col,
        },
        combine: CombineFn::Identity,
        gate: GateSpec::OrderBand(band),
        scale,
        consume,
        targets: vec![(SlotIndex::new(dst_slot), dst_col)],
    }
}

fn sum_accumulation_ops(
    parent: &HierarchyNode,
    parent_slot: u32,
    source_col: ColumnIndex,
    target_col: ColumnIndex,
    band: u32,
    scale: ScaleSpec,
) -> Vec<AccumulatorOp> {
    if children_are_contiguous(parent) {
        let (start, count) = child_range(parent);
        return vec![AccumulatorOp {
            source: SourceSpec::SlotRange {
                start: SlotIndex::new(start),
                count,
                col: source_col,
            },
            combine: CombineFn::Sum,
            gate: GateSpec::OrderBand(band),
            scale,
            consume: ConsumeMode::AddToTarget,
            targets: vec![(SlotIndex::new(parent_slot), target_col)],
        }];
    }
    vec![AccumulatorOp {
        source: sparse_child_input_list(parent, source_col),
        combine: CombineFn::Sum,
        gate: GateSpec::OrderBand(band),
        scale,
        consume: ConsumeMode::AddToTarget,
        targets: vec![(SlotIndex::new(parent_slot), target_col)],
    }]
}

fn sparse_child_input_list(parent: &HierarchyNode, source_col: ColumnIndex) -> SourceSpec {
    SourceSpec::ConjunctiveCrossing {
        inputs: parent
            .children
            .iter()
            .map(|child| InputSpec {
                slot: child.participant_slot,
                col: source_col,
                unit_cost: 1.0,
            })
            .collect(),
    }
}

fn disburse_op(
    parent: &HierarchyNode,
    child: &HierarchyNode,
    parent_intrinsic_col: ColumnIndex,
    parent_weight_sum_col: ColumnIndex,
    band: u32,
) -> AccumulatorOp {
    AccumulatorOp {
        // Declared input order is the EML PARAM order. The parent's live
        // AllocatedFlow is PARAM(1), consumed directly by the same allocator
        // operation that writes the child level; no propagated copy is read.
        source: SourceSpec::ConjunctiveCrossing {
            inputs: vec![
                InputSpec {
                    slot: parent.participant_slot,
                    col: parent_intrinsic_col,
                    unit_cost: 1.0,
                },
                InputSpec {
                    slot: parent.participant_slot,
                    col: parent.cols.allocated_flow_col,
                    unit_cost: 1.0,
                },
                InputSpec {
                    slot: parent.participant_slot,
                    col: parent_weight_sum_col,
                    unit_cost: 1.0,
                },
            ],
        },
        combine: CombineFn::EvalEML {
            tree_id: child_share_tree_id().0,
        },
        gate: GateSpec::OrderBand(band),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::AddToTarget,
        targets: vec![(child.participant_slot, child.cols.allocated_flow_col)],
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
    use simthing_core::{SimPropertyId, SlotIndex};

    fn cols() -> NodeColumnRefs {
        fn col(n: usize) -> ColumnIndex {
            ColumnIndex::from_raw_for_oracle_or_rehearsal(n)
        }
        NodeColumnRefs {
            intrinsic_flow_col: col(0),
            intrinsic_flow_sum_col: col(4),
            allocated_flow_col: col(1),
            balance_col: Some(col(3)),
            balance_governing_col: None,
            weight_col: col(2),
            weight_sum_col: col(5),
            propagated_intrinsic_flow_col: col(6),
            propagated_allocated_flow_col: col(7),
            propagated_weight_sum_col: col(8),
            hosted_simthing_id_col: col(9),
        }
    }

    fn d2_layout() -> ArenaTreeLayout {
        let c = cols();
        let root = HierarchyNode {
            participant_slot: SlotIndex::new(10),
            hosted_simthing_id: Default::default(),
            depth: 0,
            children: vec![HierarchyNode {
                participant_slot: SlotIndex::new(11),
                hosted_simthing_id: Default::default(),
                depth: 1,
                children: vec![],
                cols: c,
            }],
            cols: c,
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
            vec![root],
        )
        .unwrap()
    }
    #[test]
    fn sparse_child_rows_compile_to_one_ordered_input_list_writer() {
        let mut layout = d2_layout();
        {
            let root = &mut layout.participant_roots[0];
            let mut second = root.children[0].clone();
            second.participant_slot = SlotIndex::new(13);
            root.children.push(second);
        }

        let plan = plan_arena_allocation(&layout, &[], 16).expect("sparse rows plan");
        let root = &layout.participant_roots[0];
        let root_slot = root.participant_slot;
        let sparse_weight_sum_ops: Vec<_> = plan
            .cpu_ops
            .iter()
            .filter(|op| {
                op.gate == GateSpec::OrderBand(layout.band_layout.upsweep_band(0, 2))
                    && op.targets
                        == vec![
                            (root_slot, root.cols.weight_col),
                            (root_slot, root.cols.weight_sum_col),
                        ]
            })
            .collect();
        assert_eq!(
            sparse_weight_sum_ops.len(),
            1,
            "sparse branch pressure must have exactly one writer to both existing RF targets"
        );
        assert_eq!(sparse_weight_sum_ops[0].combine, CombineFn::Sum);
        assert_eq!(sparse_weight_sum_ops[0].consume, ConsumeMode::ResetTarget);
        let SourceSpec::ConjunctiveCrossing { inputs } = &sparse_weight_sum_ops[0].source else {
            panic!("sparse children must lower through the admitted input-list source");
        };
        assert_eq!(
            inputs
                .iter()
                .map(|input| input.slot.raw())
                .collect::<Vec<_>>(),
            vec![11, 13],
            "input-list order must preserve hierarchy child order"
        );
    }
}
