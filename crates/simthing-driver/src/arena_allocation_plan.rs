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
    #[error(transparent)]
    ExactApportionment(#[from] simthing_gpu::ResidentApportionmentError),
}

/// Bind the exact constrained-product stage to the arena's one terminal band.
///
/// The caller supplies already-admitted exact claims whose live inputs are the
/// existing `AllocatedFlow` cells. No second terminal band or host post-pass is
/// created: `ResidentApportionmentPlan::integration_band()` is minted directly
/// from [`ArenaBandLayout`](crate::arena_hierarchy::ArenaBandLayout).
pub fn plan_resident_exact_apportionment(
    layout: &ArenaTreeLayout,
    semantic_plan: &simthing_gpu::ResidentClearingPlan,
    claims: Vec<simthing_gpu::ResidentApportionmentClaim>,
    authority_granter: simthing_core::SimThingId,
    generation: GenerationStamp,
) -> Result<simthing_gpu::ResidentApportionmentPlan, AllocationPlanError> {
    Ok(simthing_gpu::ResidentApportionmentPlan::build(
        semantic_plan,
        claims,
        authority_granter,
        generation,
        layout.band_layout.integration_band,
    )?)
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
        &std::collections::BTreeSet::new(),
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
    authored_weight_slots: &std::collections::BTreeSet<u32>,
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
                // Reduce it exactly once over this parent's direct children.
                // The aggregate ALWAYS publishes to `weight_sum_col` (this
                // parent's own child-share denominator). Whether it ALSO
                // becomes this parent's upward contribution (`weight_col`)
                // is the INTERIOR-POLICY COMPOSITION LAW (DA admission
                // 2026-09-10, orchestrator relay 5625360554): a NEUTRAL
                // interior is a pure pressure carrier — the aggregate is its
                // weight at the next shallower band, exactly the historical
                // semantics. A parent carrying an authored AllocatorWeight
                // program KEEPS that authored value as its participation
                // identity upward — the constitutional reduce -> apply-policy
                // -> disburse order per level; silent installation followed
                // by erasure was the defect. Descendants are never recounted
                // or scanned independently in either case.
                let mut aggregate_targets =
                    vec![(parent.participant_slot, parent.cols.weight_sum_col)];
                if !authored_weight_slots.contains(&parent.participant_slot.raw()) {
                    aggregate_targets
                        .insert(0, (parent.participant_slot, parent.cols.weight_col));
                }
                ops_cpu.extend(sum_reduction_to_targets_ops(
                    parent,
                    parent.cols.weight_col,
                    aggregate_targets,
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

pub(crate) fn cpu_op_from_integration_gpu(gpu: &simthing_gpu::AccumulatorOpGpu) -> AccumulatorOp {
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
    // SETTLEMENT RESIDUAL LAW (DA admission, relay 5690342946): EVERY
    // balance-governed participant that lawfully owns residual work receives
    // the one admitted residual semantics — seed from its own budget, add its
    // own AllocatedFlow, subtract what it disbursed to children. A LEAF
    // disburses nothing and its `intrinsic_flow_sum_col` is never produced,
    // so its budget is its own `intrinsic_flow_col` and no subtraction op is
    // planned. Interior participants take the byte-identical historical path.
    for parent in layout.iter_all() {
        let Some(rate_col) = parent.cols.balance_governing_col else {
            continue;
        };
        let is_leaf = parent.children.is_empty();
        let budget_intrinsic_col = if parent.depth == 0 || is_leaf {
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
        if !is_leaf {
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
        // PARAM(3) is the target child's OWN currently-resolved weight column
        // (INDEPENDENT-RESOURCE BINDING LAW, relay 5688590364): the shared
        // column-agnostic formula receives it per-operation, so independent
        // resources never alias one arena's weight column.
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
                InputSpec {
                    slot: child.participant_slot,
                    col: child.cols.weight_col,
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
    fn cols_shifted(base: usize) -> NodeColumnRefs {
        fn col(n: usize) -> ColumnIndex {
            ColumnIndex::from_raw_for_oracle_or_rehearsal(n)
        }
        NodeColumnRefs {
            intrinsic_flow_col: col(base),
            intrinsic_flow_sum_col: col(base + 4),
            allocated_flow_col: col(base + 1),
            balance_col: Some(col(base + 3)),
            balance_governing_col: None,
            weight_col: col(base + 2),
            weight_sum_col: col(base + 5),
            propagated_intrinsic_flow_col: col(base + 6),
            propagated_allocated_flow_col: col(base + 7),
            propagated_weight_sum_col: col(base + 8),
            hosted_simthing_id_col: col(base + 9),
        }
    }

    fn d2_layout_for(arena_idx: u32, name: &str, prop: u32, c: NodeColumnRefs) -> ArenaTreeLayout {
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
            arena_idx,
            &GpuArenaDescriptor {
                name: name.into(),
                flow_property_id: SimPropertyId(prop),
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

    fn cols_governed(base: usize) -> NodeColumnRefs {
        let mut c = cols_shifted(base);
        c.balance_governing_col =
            Some(ColumnIndex::from_raw_for_oracle_or_rehearsal(base + 10));
        c
    }

    // SETTLEMENT RESIDUAL LAW witness (DA admission, relay 5690342946): a
    // balance-governed LEAF receives the one admitted residual semantics —
    // seed from its OWN intrinsic column (its sum column is never produced),
    // add its own AllocatedFlow, and NO child-subtraction op — while the
    // interior keeps the byte-identical historical three-op shape.
    #[test]
    fn balance_governed_leaves_receive_residual_closure() {
        let c = cols_governed(0);
        let layout = d2_layout_for(0, "governed", 1, c);
        let mut ops = Vec::new();
        append_residual_closure_ops(&layout, &mut ops);
        let rate = c.balance_governing_col.unwrap();
        let leaf_slot = layout.participant_roots[0].children[0].participant_slot;
        let root_slot = layout.participant_roots[0].participant_slot;
        let to_rate_at = |slot: SlotIndex| {
            ops.iter()
                .filter(|op| op.targets == vec![(slot, rate)])
                .collect::<Vec<_>>()
        };
        let leaf_ops = to_rate_at(leaf_slot);
        assert_eq!(leaf_ops.len(), 2, "leaf: seed + add-allocated, no subtraction");
        assert!(
            leaf_ops.iter().all(|op| !matches!(op.scale, ScaleSpec::Constant(s) if s < 0.0)),
            "leaf residual must have no child-subtraction op"
        );
        assert!(
            leaf_ops.iter().any(|op| matches!(
                &op.source,
                SourceSpec::SlotValue { slot, col } if *slot == leaf_slot && *col == c.intrinsic_flow_col
            )),
            "leaf budget seeds from its OWN intrinsic column, never the unproduced sum"
        );
        let root_ops = to_rate_at(root_slot);
        assert!(
            root_ops.len() >= 3,
            "interior keeps the historical seed + add + subtraction shape"
        );
    }

    // ONE-INTEGRATION-AUTHORITY LAW witness (DA admission, relay 5690342946):
    // an arena plan built the way the production sync builds it (empty
    // governed set) carries ZERO governed-integration ops — the single
    // registry-wide integration tail is the sync's, appended exactly once.
    // Pre-repair, each arena embedded the full registry-wide integration,
    // multiplying every governed rate by the arena count (the ordinary
    // session always holds the user arena PLUS residency-row-capacity).
    #[test]
    fn arena_plans_carry_no_governed_integration() {
        let layout = d2_layout_for(0, "minerals", 1, cols());
        let plan = plan_arena_allocation(&layout, &[], 256).expect("plan");
        assert!(
            plan.cpu_ops
                .iter()
                .all(|op| !matches!(op.combine, CombineFn::IntegrateWithClamp { .. })),
            "per-arena plans must not embed governed integration"
        );
    }

    // INDEPENDENT-RESOURCE BINDING LAW witness (DA admission, relay
    // 5688590364): with two distinct resource column layouts in ONE session,
    // every disbursement operation supplies the TARGET CHILD'S OWN resolved
    // weight column as the fourth admitted input (PARAM 3), in either
    // planning order. The outlawed defect — one arena's absolute weight
    // column aliased into every arena's child-share formula through the
    // register-once global tree — cannot recur because the shared formula
    // owns no column at all (see child_share_eml witness) and the binding
    // below is per-operation from each arena's own layout.
    #[test]
    fn independent_resource_layouts_bind_their_own_child_weight_in_both_orders() {
        let a = d2_layout_for(0, "minerals", 1, cols());
        let b = d2_layout_for(1, "energy", 2, cols_shifted(20));
        for order in [[&a, &b], [&b, &a]] {
            for layout in order {
                let plan = plan_arena_allocation_with_pressure(
                    layout,
                    &[],
                    16,
                    &[],
                    &[],
                    GenerationStamp::new(0),
                    GenerationStamp::new(1),
                    &std::collections::BTreeSet::new(),
                )
                .expect("plan");
                let expected_weight_col = layout.participant_roots[0].children[0].cols.weight_col;
                let child_slot = layout.participant_roots[0].children[0].participant_slot;
                let disburse: Vec<_> = plan
                    .cpu_ops
                    .iter()
                    .filter(|op| {
                        matches!(op.combine, CombineFn::EvalEML { tree_id }
                            if tree_id == child_share_tree_id().0)
                    })
                    .collect();
                assert!(!disburse.is_empty(), "d2 layout must emit a disbursement");
                for op in disburse {
                    let SourceSpec::ConjunctiveCrossing { inputs } = &op.source else {
                        panic!("disbursement must be a conjunctive crossing");
                    };
                    assert_eq!(inputs.len(), 4, "child weight must ride as the fourth input");
                    assert_eq!(inputs[3].col, expected_weight_col, "own arena weight column");
                    assert_eq!(inputs[3].slot, child_slot, "evaluated at the target child slot");
                }
            }
        }
    }

    #[test]
    fn interior_authored_weight_survives_the_pressure_upsweep() {
        // INTERIOR-POLICY COMPOSITION LAW witness (DA admission 2026-09-10,
        // orchestrator relay 5625360554): a parent carrying an authored
        // AllocatorWeight program keeps `weight_col` as its upward
        // participation identity; the recursive child aggregate feeds ONLY
        // its own child-share denominator (`weight_sum_col`). Silent
        // installation followed by erasure is the outlawed defect.
        let layout = d2_layout();
        let root = &layout.participant_roots[0];
        let root_slot = root.participant_slot;
        let mut policy = std::collections::BTreeSet::new();
        policy.insert(root_slot.raw());
        let plan = plan_arena_allocation_with_pressure(
            &layout,
            &[],
            16,
            &[],
            &[],
            GenerationStamp::new(0),
            GenerationStamp::new(1),
            &policy,
        )
        .expect("policy-bearing plan");
        let band = layout.band_layout.upsweep_band(0, 2);
        let agg: Vec<_> = plan
            .cpu_ops
            .iter()
            .filter(|op| {
                op.gate == GateSpec::OrderBand(band)
                    && op.targets
                        .contains(&(root_slot, root.cols.weight_sum_col))
            })
            .collect();
        assert_eq!(agg.len(), 1, "exactly one branch-pressure aggregate writer");
        assert_eq!(
            agg[0].targets,
            vec![(root_slot, root.cols.weight_sum_col)],
            "authored interior: the aggregate must NOT overwrite the authored weight_col"
        );
        assert!(
            plan.cpu_ops
                .iter()
                .all(|op| !op.targets.contains(&(root_slot, root.cols.weight_col))),
            "no plan op may erase an authored interior AllocatorWeight"
        );
    }

    #[test]
    fn neutral_interior_pressure_carrier_is_bit_identical_to_history() {
        // Neutral degeneration: an empty authored-weight set reproduces the
        // historical plan exactly — the aggregate remains the neutral
        // interior's upward weight via the double-target write.
        let layout = d2_layout();
        let a = plan_arena_allocation(&layout, &[], 16).expect("neutral entry");
        let b = plan_arena_allocation_with_pressure(
            &layout,
            &[],
            16,
            &[],
            &[],
            GenerationStamp::new(0),
            GenerationStamp::new(1),
            &std::collections::BTreeSet::new(),
        )
        .expect("empty-policy plan");
        assert_eq!(
            format!("{:?}", a.cpu_ops),
            format!("{:?}", b.cpu_ops),
            "empty policy set must degenerate to the historical plan bit-for-bit"
        );
    }

}
