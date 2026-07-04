//! C-5/C-6 reduction → AccumulatorOp OrderBand planning.

use simthing_core::ReductionRule;

use crate::accumulator_op::{
    combine_kind, consume_kind, gate_kind, scale_kind, source_kind, AccumulatorOpGpu,
};
use crate::reduction::{ColumnRuleDescriptor, TopologyState};

#[derive(Clone, Debug, PartialEq)]
pub struct ReductionOrderBandPlan {
    pub ops: Vec<AccumulatorOpGpu>,
    pub n_bands: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ReductionPlanError {
    #[error("non-contiguous child slots for parent {parent_slot}")]
    NonContiguousChildren { parent_slot: u32 },
    #[error("WeightedMean weight_col {weight_col} >= n_dims {n_dims}")]
    InvalidWeightCol { weight_col: u32, n_dims: u32 },
}

fn make_reduction_op(
    first_child: u32,
    n_children: u32,
    parent_slot: u32,
    col: u32,
    combine: u32,
    weight_col: u32,
    band: u32,
) -> AccumulatorOpGpu {
    AccumulatorOpGpu {
        source_kind: source_kind::SLOT_RANGE,
        source_slot: first_child,
        source_col: col,
        source_count: n_children,
        combine_kind: combine,
        combine_a: weight_col,
        combine_b: 0,
        combine_c: 0,
        combine_d: 0,
        gate_kind: gate_kind::ORDER_BAND,
        gate_a: band,
        gate_b: 0,
        scale_kind: scale_kind::IDENTITY,
        scale_a: 0,
        consume: consume_kind::RESET_TARGET,
        target0_slot: parent_slot,
        target0_col: col,
        target1_slot: 0,
        target1_col: 0,
        target2_slot: 0,
        target2_col: 0,
        target3_slot: 0,
        target3_col: 0,
        n_targets: 1,
        _pad: 0,
    }
}

fn combine_for_rule(
    desc: &ColumnRuleDescriptor,
    n_dims: u32,
) -> Result<(u32, u32), ReductionPlanError> {
    let weight_col = match desc.rule {
        ReductionRule::Mean => (combine_kind::MEAN, 0),
        ReductionRule::WeightedMean { .. } => {
            if desc.weight_col >= n_dims {
                return Err(ReductionPlanError::InvalidWeightCol {
                    weight_col: desc.weight_col,
                    n_dims,
                });
            }
            (combine_kind::WEIGHTED_MEAN, desc.weight_col)
        }
        ReductionRule::Sum => (combine_kind::SUM, 0),
        ReductionRule::Max => (combine_kind::MAX, 0),
        ReductionRule::Min => (combine_kind::MIN, 0),
        ReductionRule::First => (combine_kind::FIRST, 0),
    };
    Ok(weight_col)
}

/// Plan reduction ops for AccumulatorOp OrderBand execution.
pub fn plan_reduction_orderband(
    topology: &TopologyState,
    column_rules: &[ColumnRuleDescriptor],
    n_dims: u32,
) -> Result<ReductionOrderBandPlan, ReductionPlanError> {
    let topo = topology.flatten();
    let max_depth = topo.depth_buckets.len().saturating_sub(1) as u32;
    let mut ops = Vec::new();
    let mut n_bands = 0u32;

    for slot in 0..topo.n_slots() as u32 {
        let start = topo.child_starts[slot as usize] as usize;
        let end = topo.child_starts[slot as usize + 1] as usize;
        let n_children = end.saturating_sub(start);
        if n_children == 0 {
            continue;
        }
        let children = &topo.child_indices[start..end];
        let first_child = children[0];
        for (i, &child) in children.iter().enumerate() {
            if child != first_child + i as u32 {
                return Err(ReductionPlanError::NonContiguousChildren { parent_slot: slot });
            }
        }

        let depth = topology
            .depths
            .get(slot as usize)
            .and_then(|d| *d)
            .unwrap_or(0);
        let band = max_depth.saturating_sub(1).saturating_sub(depth);

        for col in 0..n_dims as usize {
            if col >= column_rules.len() {
                break;
            }
            let desc = column_rules[col];
            let (combine, weight_col) = combine_for_rule(&desc, n_dims)?;
            ops.push(make_reduction_op(
                first_child,
                n_children as u32,
                slot,
                col as u32,
                combine,
                weight_col,
                band,
            ));
            n_bands = n_bands.max(band + 1);
        }
    }

    debug_assert_no_duplicate_band_slot_col(&ops);

    Ok(ReductionOrderBandPlan { ops, n_bands })
}

/// Map a legacy reduction depth-bucket index to the C-5 OrderBand index.
///
/// `depth_bucket_ranges[i]` holds slots at tree depth `i` (root = 0). Legacy
/// reduction walks buckets deepest-first; soft bands use the same parent-depth
/// mapping as [`plan_reduction_orderband`]: deepest internal parents are band
/// 0, then shallower parents. The leaf-only deepest bucket has no soft band.
pub fn reduction_soft_band_for_depth_bucket(
    max_tree_depth: u32,
    depth_bucket_index: u32,
) -> Option<u32> {
    if depth_bucket_index >= max_tree_depth {
        return None;
    }
    Some(
        max_tree_depth
            .saturating_sub(1)
            .saturating_sub(depth_bucket_index),
    )
}

fn debug_assert_no_duplicate_band_slot_col(ops: &[AccumulatorOpGpu]) {
    #[cfg(debug_assertions)]
    {
        use std::collections::HashSet;

        let mut seen: HashSet<(u32, u32, u32)> = HashSet::new();
        for op in ops {
            let key = (op.gate_a, op.target0_slot, op.target0_col);
            assert!(
                seen.insert(key),
                "reduction OrderBand planner produced duplicate (band, slot, col): {key:?}"
            );
        }
    }
    #[cfg(not(debug_assertions))]
    let _ = ops;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reduction::build_column_rule_descriptors;
    use crate::slot::SlotAllocator;
    use simthing_core::{
        DimensionRegistry, PropertyValue, ReductionRule, SimProperty, SimThing, SimThingKind,
    };

    fn two_level_fixture() -> (TopologyState, DimensionRegistry, u32) {
        let mut reg = DimensionRegistry::new();
        let pid = reg.register(SimProperty::simple("core", "loyalty", 0));
        let mut world = SimThing::new(SimThingKind::World, 0);
        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        cohort.add_property(pid, PropertyValue::from_layout(&reg.property(pid).layout));
        world.add_child(cohort);
        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&world);
        let topo = TopologyState::build(&world, &alloc);
        let n_dims = reg.total_columns as u32;
        (topo, reg, n_dims)
    }

    fn sum_property_fixture() -> (TopologyState, DimensionRegistry, u32) {
        let mut reg = DimensionRegistry::new();
        let mut prop = SimProperty::simple("demo", "population", 0);
        prop.layout.sub_fields[0].reduction_override = Some(ReductionRule::Sum);
        let pid = reg.register(prop);
        let mut world = SimThing::new(SimThingKind::World, 0);
        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        cohort.add_property(pid, PropertyValue::from_layout(&reg.property(pid).layout));
        world.add_child(cohort);
        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&world);
        let topo = TopologyState::build(&world, &alloc);
        let n_dims = reg.total_columns as u32;
        (topo, reg, n_dims)
    }

}
