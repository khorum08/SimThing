//! E-11 CPU allocation oracle (memo §9.1).

use std::collections::HashMap;

use crate::arena_hierarchy::ArenaTreeLayout;
use crate::arena_registry::SlotId;
use crate::child_share_eml::child_share_cpu;

type CellKey = (SlotId, u32);

#[derive(Clone, Debug, Default)]
pub struct ArenaAllocationOracleTrace {
    pub resets: Vec<SlotId>,
    pub reductions: Vec<(SlotId, f32, f32)>,
    pub disbursements: Vec<(SlotId, SlotId, f32)>,
}

impl ArenaAllocationOracleTrace {
    fn record_reset(&mut self, slot: SlotId) {
        self.resets.push(slot);
    }

    fn record_reduction(&mut self, slot: SlotId, i_f_sum: f32, weight_sum: f32) {
        self.reductions.push((slot, i_f_sum, weight_sum));
    }

    fn record_disbursement(&mut self, parent: SlotId, child: SlotId, share: f32) {
        self.disbursements.push((parent, child, share));
    }
}

fn get(values: &HashMap<CellKey, f32>, slot: SlotId, col: u32) -> f32 {
    values.get(&(slot, col)).copied().unwrap_or(0.0)
}

fn set(values: &mut HashMap<CellKey, f32>, slot: SlotId, col: u32, v: f32) {
    values.insert((slot, col), v);
}

fn add(values: &mut HashMap<CellKey, f32>, slot: SlotId, col: u32, delta: f32) {
    let cell = values.entry((slot, col)).or_insert(0.0);
    *cell += delta;
}

pub fn run_arena_allocation_oracle(
    layout: &ArenaTreeLayout,
    values: &mut HashMap<CellKey, f32>,
    dt: f32,
) -> ArenaAllocationOracleTrace {
    let mut trace = ArenaAllocationOracleTrace::default();

    // Phase 0 — reset allocated_flow and per-tick internal columns.
    for node in layout.iter_all() {
        set(
            values,
            node.participant_slot,
            node.cols.allocated_flow_col,
            0.0,
        );
        set(
            values,
            node.participant_slot,
            node.cols.intrinsic_flow_sum_col,
            0.0,
        );
        set(values, node.participant_slot, node.cols.weight_sum_col, 0.0);
        set(
            values,
            node.participant_slot,
            node.cols.propagated_intrinsic_flow_col,
            0.0,
        );
        set(
            values,
            node.participant_slot,
            node.cols.propagated_allocated_flow_col,
            0.0,
        );
        set(
            values,
            node.participant_slot,
            node.cols.propagated_weight_sum_col,
            0.0,
        );
        trace.record_reset(node.participant_slot);
    }

    if layout.max_depth <= 1 {
        integrate_balance(layout, values, dt);
        return trace;
    }

    // Phase 1 — up-sweep (deepest interior first).
    for depth in (0..layout.max_depth.saturating_sub(1)).rev() {
        for parent in layout.iter_at_depth(depth) {
            if parent.children.is_empty() {
                continue;
            }
            let mut i_f_sum = 0.0_f32;
            let mut weight_sum = 0.0_f32;
            for child in &parent.children {
                i_f_sum += get(
                    values,
                    child.participant_slot,
                    child.cols.intrinsic_flow_col,
                );
                weight_sum += get(values, child.participant_slot, child.cols.weight_col);
            }
            set(
                values,
                parent.participant_slot,
                parent.cols.intrinsic_flow_sum_col,
                i_f_sum,
            );
            set(
                values,
                parent.participant_slot,
                parent.cols.weight_sum_col,
                weight_sum,
            );
            trace.record_reduction(parent.participant_slot, i_f_sum, weight_sum);
        }
    }

    // Phase 2 — down-sweep.
    for depth in 0..layout.max_depth.saturating_sub(1) {
        for parent in layout.iter_at_depth(depth) {
            if parent.children.is_empty() {
                continue;
            }
            let p_if = if depth == 0 {
                get(
                    values,
                    parent.participant_slot,
                    parent.cols.intrinsic_flow_col,
                )
            } else {
                get(
                    values,
                    parent.participant_slot,
                    parent.cols.intrinsic_flow_sum_col,
                )
            };
            let p_af = if depth == 0 {
                0.0
            } else {
                get(
                    values,
                    parent.participant_slot,
                    parent.cols.allocated_flow_col,
                )
            };
            let p_ws = get(values, parent.participant_slot, parent.cols.weight_sum_col);
            for child in &parent.children {
                set(
                    values,
                    child.participant_slot,
                    child.cols.propagated_intrinsic_flow_col,
                    p_if,
                );
                set(
                    values,
                    child.participant_slot,
                    child.cols.propagated_allocated_flow_col,
                    p_af,
                );
                set(
                    values,
                    child.participant_slot,
                    child.cols.propagated_weight_sum_col,
                    p_ws,
                );
            }
        }
        for parent in layout.iter_at_depth(depth) {
            for child in &parent.children {
                let p_if = get(
                    values,
                    child.participant_slot,
                    child.cols.propagated_intrinsic_flow_col,
                );
                let p_af = get(
                    values,
                    child.participant_slot,
                    child.cols.propagated_allocated_flow_col,
                );
                let w = get(values, child.participant_slot, child.cols.weight_col);
                let p_ws = get(
                    values,
                    child.participant_slot,
                    child.cols.propagated_weight_sum_col,
                );
                let share = child_share_cpu(p_if, p_af, w, p_ws);
                add(
                    values,
                    child.participant_slot,
                    child.cols.allocated_flow_col,
                    share,
                );
                trace.record_disbursement(parent.participant_slot, child.participant_slot, share);
            }
        }
    }

    integrate_balance(layout, values, dt);
    trace
}

fn integrate_balance(layout: &ArenaTreeLayout, values: &mut HashMap<CellKey, f32>, dt: f32) {
    for node in layout.iter_all() {
        if let Some(balance_col) = node.cols.balance_col {
            let i_f = get(values, node.participant_slot, node.cols.intrinsic_flow_col);
            let a_f = get(values, node.participant_slot, node.cols.allocated_flow_col);
            add(values, node.participant_slot, balance_col, (i_f + a_f) * dt);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arena_hierarchy::{build_custom_layout, HierarchyNode, NodeColumnRefs};
    use crate::arena_registry::GpuArenaDescriptor;
    use simthing_core::SimPropertyId;

    fn arena_desc(max_depth_budget: u32) -> GpuArenaDescriptor {
        GpuArenaDescriptor {
            name: "food".into(),
            flow_property_id: SimPropertyId(1),
            balance_property_id: None,
            max_participants: 16,
            max_coupling_fanout: 4,
            max_orderband_depth: max_depth_budget,
            fission_policy: Default::default(),
            participant_range: (0, 0),
            wildcard_max_expansion: None,
            reserved_orderband_depth: 0,
        }
    }

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

    fn d2_star() -> ArenaTreeLayout {
        let c = cols();
        let leaves = vec![
            HierarchyNode {
                participant_slot: 11,
                hosted_simthing_id: Default::default(),
                depth: 1,
                children: vec![],
                cols: c,
                gap_used: 0,
            },
            HierarchyNode {
                participant_slot: 12,
                hosted_simthing_id: Default::default(),
                depth: 1,
                children: vec![],
                cols: c,
                gap_used: 0,
            },
        ];
        let root = HierarchyNode {
            participant_slot: 10,
            hosted_simthing_id: Default::default(),
            depth: 0,
            children: leaves,
            cols: c,
            gap_used: 0,
        };
        build_custom_layout(0, &arena_desc(16), c, Default::default(), 9, vec![root]).unwrap()
    }

}
