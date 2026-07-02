//! EML field formula class admission (V7.6 — designer/RON policy layer).

use simthing_core::{
    eml_opcode, EmlConsumerMask, EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta,
    EmlNodeGpu, EmlTreeId, EmlTreeMeta, WHITELISTED_FORMULA_CLASSES,
};

fn literal_node(v: f32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode: eml_opcode::LITERAL_F32,
        flags: 0,
        a: v.to_bits(),
        b: 0,
        c: 0,
        d: 0,
    }
}

fn exact_meta(id: u32, name: &str) -> EmlFormulaMeta {
    EmlFormulaMeta {
        tree_id: EmlTreeId(id),
        execution_class: EmlExecutionClass::ExactDeterministic,
        allowed_consumers: EmlConsumerMask(
            EmlConsumerMask::ALL_PRODUCTION | EmlConsumerMask::DEBUG_ORACLE,
        ),
        max_abs_error: None,
        deterministic_gpu: true,
        requires_guard_for_hard_threshold: false,
        node_count: 1,
        max_stack_depth: 0,
        has_loops: false,
        has_recursion: false,
        display_name: name.into(),
    }
}
