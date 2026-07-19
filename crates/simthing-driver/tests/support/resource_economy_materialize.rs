//! Shared helpers for resource economy driver materialization tests.

#![allow(dead_code)]

use simthing_core::{
    ClampBehavior, DimensionRegistry, EmlConsumerMask, EmlExecutionClass, EmlExpressionRegistry,
    EmlFormulaMeta, EmlNodeGpu, EmlTreeId, PropertyLayout, SimProperty, SimPropertyId,
    SubFieldRole, SubFieldSpec,
};
use simthing_spec::{
    compile_resource_economy, EmissionFormulaSpec, EmitBufferSpec, EmitOnThresholdSpec,
    PropertyKey, RecipeInputSpec, ResourceEconomySpec, ResourceEmissionSpec, ResourceRecipeSpec,
    ResourceTransferSpec, TriggerDirection,
};

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

pub fn pk(ns: &str, name: &str) -> PropertyKey {
    PropertyKey::new(ns, name)
}

pub fn amount_transfer(
    id: &str,
    source: &str,
    target: &str,
    amount: f32,
    band: u32,
) -> ResourceTransferSpec {
    ResourceTransferSpec {
        id: id.into(),
        source: pk("core", source),
        source_role: SubFieldRole::Named("amount".into()),
        target: pk("core", target),
        target_role: SubFieldRole::Named("amount".into()),
        amount,
        order_band: band,
                source_host_entity: None,
            target_host_entity: None,
        }
            source_host_span_token: None,
            target_host_span_token: None,
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

pub fn compile_fixture(
    spec: &ResourceEconomySpec,
    reg: &DimensionRegistry,
    eml: &EmlExpressionRegistry,
) -> simthing_spec::CompiledResourceEconomy {
    compile_resource_economy(spec, reg, eml).unwrap()
}

pub fn full_fixture_spec() -> ResourceEconomySpec {
    ResourceEconomySpec {
        transfers: vec![amount_transfer("t1", "food", "store", 1.0, 1)],
        recipes: vec![ResourceRecipeSpec {
            id: "r1".into(),
            inputs: vec![
                RecipeInputSpec {
                    property: pk("core", "food"),
                    role: SubFieldRole::Named("amount".into()),
                    unit_cost: 1.0,
                },
                RecipeInputSpec {
                    property: pk("core", "water"),
                    role: SubFieldRole::Named("amount".into()),
                    unit_cost: 2.0,
                },
            ],
            target: pk("core", "meal"),
            target_role: SubFieldRole::Named("amount".into()),
            throttle_hint_max_per_tick: 4,
        }],
        emissions: vec![
            ResourceEmissionSpec {
                id: "e_identity".into(),
                source: pk("core", "food"),
                source_role: SubFieldRole::Named("amount".into()),
                formula: EmissionFormulaSpec::IdentityFloor,
                        host_entity: None,
                    host_span_token: None,
        },
            ResourceEmissionSpec {
                id: "e_constant".into(),
                source: pk("core", "food"),
                source_role: SubFieldRole::Named("amount".into()),
                formula: EmissionFormulaSpec::Constant(3.5),
                        host_entity: None,
                    host_span_token: None,
        },
            ResourceEmissionSpec {
                id: "e_eval".into(),
                source: pk("core", "food"),
                source_role: SubFieldRole::Named("amount".into()),
                formula: EmissionFormulaSpec::EvalEml {
                    formula_key: "food_emission_v1".into(),
                },
                host_entity: None,
                        host_span_token: None,
        },
        ],
        emit_on_threshold: vec![EmitOnThresholdSpec {
            id: "th1".into(),
            source: pk("core", "heat"),
            source_role: SubFieldRole::Named("amount".into()),
            threshold: 100.0,
            direction: TriggerDirection::Rising,
            event_kind: 3,
            buffer: EmitBufferSpec::Output,
            host_entity: None,
            host_span_token: None,
        }],
        ..Default::default()
    }
}

pub fn register_full_fixture(reg: &mut DimensionRegistry) {
    for name in ["food", "water", "store", "meal", "heat"] {
        register_amount_property(reg, "core", name);
    }
}
