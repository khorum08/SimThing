//! E-11 child-share EML formula registration (memo §7.1).

use simthing_core::eml_nodes::{self, EmlNode};
use simthing_core::{
    EmlConsumerMask, EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta, EmlRegistryError,
    EmlTreeId,
};

use crate::arena_hierarchy::CHILD_SHARE_FORMULA_TREE_ID;

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
/// All four operands arrive directly through the operation's admitted input
/// list as `PARAM(0..=3)`. `PARAM(1)` is the parent's live `AllocatedFlow`;
/// no propagated economic copy exists between recursive levels. `PARAM(3)` is
/// the target child's own currently-resolved `AllocatorWeight`, supplied by
/// each `disburse_op`'s input list — INDEPENDENT-RESOURCE BINDING LAW (DA
/// admission, relay 5688590364): the shared formula owns no absolute arena
/// column, so independent resources in one EML registry can never alias one
/// child-share weight column regardless of arena registration order.
pub fn compile_child_share_formula_nodes() -> Vec<EmlNode> {
    vec![
        param(2),
        lit(0.0),
        unary(eml_nodes::opcode::CMP_GT),
        param(0),
        param(1),
        unary(eml_nodes::opcode::ADD),
        param(3),
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

/// Register once per session. The formula is column-agnostic (all operands
/// arrive as PARAMs from each operation's admitted input list), so a single
/// generic registration serves every arena and rebinding on layout change is
/// structurally unnecessary.
pub fn register_child_share_formula(
    registry: &mut EmlExpressionRegistry,
) -> Result<(), EmlRegistryError> {
    let id = child_share_tree_id();
    if registry.get(id).is_some() {
        return Ok(());
    }
    let nodes = compile_child_share_formula_nodes();
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

    // INDEPENDENT-RESOURCE BINDING LAW witness: the compiled formula owns no
    // absolute column — every operand is a PARAM fed by the operation's
    // admitted input list, and the child weight is PARAM(3) exactly.
    #[test]
    fn child_share_formula_is_column_agnostic_with_child_weight_as_param_3() {
        let nodes = compile_child_share_formula_nodes();
        assert!(
            nodes
                .iter()
                .all(|n| n.opcode != eml_nodes::opcode::SLOT_VALUE),
            "child-share formula must not bake any absolute arena column"
        );
        assert!(
            nodes
                .iter()
                .any(|n| n.opcode == eml_nodes::opcode::PARAM && n.a == 3),
            "target child's own weight must arrive as PARAM(3)"
        );
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
