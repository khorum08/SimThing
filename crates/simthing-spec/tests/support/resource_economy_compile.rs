//! Shared helpers for resource economy compile tests.

#![allow(dead_code)]

use simthing_core::{
    ClampBehavior, DimensionRegistry, EmlConsumerMask, EmlExecutionClass, EmlExpressionRegistry,
    EmlFormulaMeta, EmlNodeGpu, EmlTreeId, PropertyLayout, SimProperty, SimPropertyId,
    SubFieldRole, SubFieldSpec,
};
use simthing_spec::{PropertyKey, ResourceTransferSpec};

pub fn empty_registry() -> DimensionRegistry {
    DimensionRegistry::new()
}

pub fn register_amount_property(
    reg: &mut DimensionRegistry,
    ns: &str,
    name: &str,
) -> SimPropertyId {
    reg.register(SimProperty {
        namespace: ns.into(),
        name: name.into(),
        layout: PropertyLayout {
            sub_fields: vec![SubFieldSpec {
                role: SubFieldRole::Named("amount".into()),
                width: 1,
                clamp: ClampBehavior::Unbounded,
                velocity_max: None,
                default: 0.0,
                display_name: "amount".into(),
                display_range: None,
                governed_by: None,
                reduction_override: None,
                soft_aggregate_guard: None,
                accumulator_spec: None,
            }],
        },
        decay: None,
        intensity_behavior: None,
        fission_templates: vec![],
        fusion_templates: vec![],
        on_expire: None,
        description: String::new(),
        intensity_labels: vec![],
    })
}

pub fn amount_property(
    id: &str,
    source: &str,
    target: &str,
    amount: f32,
    order_band: u32,
) -> ResourceTransferSpec {
    ResourceTransferSpec {
        id: id.into(),
        source: PropertyKey::new("core", source),
        source_role: SubFieldRole::Named("amount".into()),
        target: PropertyKey::new("core", target),
        target_role: SubFieldRole::Named("amount".into()),
        amount,
        order_band,
    }
}

pub fn exact_eml_registry(entries: &[(&str, u32)]) -> EmlExpressionRegistry {
    let mut eml = EmlExpressionRegistry::new();
    for (name, id) in entries {
        let tree_id = EmlTreeId(*id);
        let meta = EmlFormulaMeta {
            tree_id,
            execution_class: EmlExecutionClass::ExactDeterministic,
            allowed_consumers: EmlConsumerMask(
                EmlConsumerMask::ALL_PRODUCTION | EmlConsumerMask::DEBUG_ORACLE,
            ),
            max_abs_error: None,
            deterministic_gpu: true,
            requires_guard_for_hard_threshold: false,
            node_count: 1,
            max_stack_depth: 1,
            has_loops: false,
            has_recursion: false,
            display_name: (*name).into(),
        };
        eml.register_formula(
            tree_id,
            meta,
            vec![EmlNodeGpu {
                opcode: simthing_core::eml_opcode::LITERAL_F32,
                flags: 0,
                a: 1.0_f32.to_bits(),
                b: 0,
                c: 0,
                d: 0,
            }],
        )
        .unwrap();
    }
    eml
}

pub fn fast_eml_registry(name: &str, id: u32) -> EmlExpressionRegistry {
    let mut eml = EmlExpressionRegistry::new();
    let tree_id = EmlTreeId(id);
    let meta = EmlFormulaMeta {
        tree_id,
        execution_class: EmlExecutionClass::FastApproximate,
        allowed_consumers: EmlConsumerMask(EmlConsumerMask::DEBUG_ORACLE),
        max_abs_error: None,
        deterministic_gpu: false,
        requires_guard_for_hard_threshold: false,
        node_count: 1,
        max_stack_depth: 1,
        has_loops: false,
        has_recursion: false,
        display_name: name.into(),
    };
    eml.register_formula(
        tree_id,
        meta,
        vec![EmlNodeGpu {
            opcode: simthing_core::eml_opcode::LITERAL_F32,
            flags: 0,
            a: 1.0_f32.to_bits(),
            b: 0,
            c: 0,
            d: 0,
        }],
    )
    .unwrap();
    eml
}
