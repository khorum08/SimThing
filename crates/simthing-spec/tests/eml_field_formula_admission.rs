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

#[test]
fn test_f_field_formula_legacy_whitelist_accepts() {
    let classes = [
        "field_pressure",
        "field_urgency",
        "field_decay",
        "bounded_field_update",
        "conversion_rate",
    ];
    for (i, class) in classes.iter().enumerate() {
        assert!(
            WHITELISTED_FORMULA_CLASSES.contains(class),
            "{class} missing from WHITELISTED_FORMULA_CLASSES"
        );
        let mut reg = EmlExpressionRegistry::new();
        reg.register(
            EmlTreeId(100 + i as u32),
            EmlTreeMeta {
                node_count: 1,
                has_transcendental: false,
                formula_class: class.to_string(),
            },
        )
        .unwrap_or_else(|e| panic!("legacy register {class}: {e}"));
        assert!(reg.assert_whitelisted(EmlTreeId(100 + i as u32)).is_ok());
    }
}

#[test]
fn test_f_c8_register_formula_accepts_field_classes() {
    let classes = [
        "field_pressure",
        "field_urgency",
        "field_decay",
        "bounded_field_update",
    ];
    for (i, class) in classes.iter().enumerate() {
        let mut reg = EmlExpressionRegistry::new();
        reg.register_formula(
            EmlTreeId(200 + i as u32),
            exact_meta(200 + i as u32, class),
            vec![literal_node(1.0)],
        )
        .unwrap_or_else(|e| panic!("C-8 register {class}: {e}"));
    }
}
