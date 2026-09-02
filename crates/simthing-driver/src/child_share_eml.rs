//! E-11 child-share EML formula registration (memo §7.1).

use simthing_core::eml_nodes::{self, EmlNode};
use simthing_core::{
    ColumnIndex, EmlConsumerMask, EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta,
    EmlRegistryError, EmlTreeId,
};
use simthing_gpu::encode_column;

use crate::arena_hierarchy::{NodeColumnRefs, CHILD_SHARE_FORMULA_TREE_ID};

pub fn child_share_tree_id() -> EmlTreeId {
    EmlTreeId(CHILD_SHARE_FORMULA_TREE_ID)
}

fn lit(v: f32) -> EmlNode {
    EmlNode {
        opcode: eml_nodes::opcode::LITERAL_F32,
        flags: 0,
        a: v.to_bits(),
        b: 0,
        c: 0,
        d: 0,
    }
}

fn slot(col: ColumnIndex) -> EmlNode {
    EmlNode {
        opcode: eml_nodes::opcode::SLOT_VALUE,
        flags: 0,
        a: encode_column(col),
        b: 0,
        c: 0,
        d: 0,
    }
}

fn param(index: u32) -> EmlNode {
    EmlNode {
        opcode: eml_nodes::opcode::PARAM,
        flags: 0,
        a: index,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn unary(opcode: u32) -> EmlNode {
    EmlNode {
        opcode,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

/// Postfix tree: `select(pWS > 0, (pIF + pAF) * w / pWS, 0)` — 13 nodes.
///
/// The three parent operands arrive directly through the operation's admitted
/// input list as `PARAM(0..=2)`. `SLOT_VALUE(weight_col)` is evaluated at the
/// target child slot. In particular, `PARAM(1)` is the parent's live
/// `AllocatedFlow`; no propagated economic copy exists between recursive
/// levels.
pub fn compile_child_share_formula_nodes(cols: NodeColumnRefs) -> Vec<EmlNode> {
    vec![
        param(2),
        lit(0.0),
        unary(eml_nodes::opcode::CMP_GT),
        param(0),
        param(1),
        unary(eml_nodes::opcode::ADD),
        slot(cols.weight_col),
        unary(eml_nodes::opcode::MUL),
        param(2),
        EmlNode {
            opcode: eml_nodes::opcode::DIV,
            flags: 1,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        },
        lit(0.0),
        unary(eml_nodes::opcode::SELECT),
        unary(eml_nodes::opcode::RETURN_TOP),
    ]
}

pub fn child_share_formula_meta() -> EmlFormulaMeta {
    EmlFormulaMeta {
        tree_id: child_share_tree_id(),
        execution_class: EmlExecutionClass::ExactDeterministic,
        allowed_consumers: EmlConsumerMask(
            EmlConsumerMask::TRANSFER_CONSERVATION | EmlConsumerMask::DEBUG_ORACLE,
        ),
        max_abs_error: None,
        deterministic_gpu: true,
        requires_guard_for_hard_threshold: false,
        node_count: 13,
        max_stack_depth: 4,
        has_loops: false,
        has_recursion: false,
        display_name: "child_share_formula".into(),
    }
}

/// Register once per session when cols are resolved for an arena flow property.
pub fn register_child_share_formula(
    registry: &mut EmlExpressionRegistry,
    cols: NodeColumnRefs,
) -> Result<(), EmlRegistryError> {
    let id = child_share_tree_id();
    if registry.get(id).is_some() {
        return Ok(());
    }
    let nodes = compile_child_share_formula_nodes(cols);
    let mut meta = child_share_formula_meta();
    meta.node_count = nodes.len() as u32;
    registry.register_formula(id, meta, nodes)
}

/// CPU oracle branch matching GPU `select(pWS > 0, …, 0)`.
#[inline]
pub fn child_share_cpu(p_if: f32, p_af: f32, w: f32, p_ws: f32) -> f32 {
    if p_ws > 0.0 {
        (p_if + p_af) * w / p_ws
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arena_hierarchy::NodeColumnRefs;

    fn sample_cols() -> NodeColumnRefs {
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

    #[test]
    fn child_share_cpu_zero_weight_is_zero_not_nan() {
        assert_eq!(
            child_share_cpu(5.0, 0.0, 0.0, 0.0).to_bits(),
            0.0_f32.to_bits()
        );
        assert_eq!(
            child_share_cpu(-5.0, 0.0, 0.0, 0.0).to_bits(),
            0.0_f32.to_bits()
        );
        assert_eq!(
            child_share_cpu(0.0, 0.0, 0.0, 0.0).to_bits(),
            0.0_f32.to_bits()
        );
    }
}
