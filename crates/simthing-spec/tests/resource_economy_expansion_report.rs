//! Phase T-2 — resource economy compile positive + expansion report tests.

#[path = "support/resource_economy_compile.rs"]
mod support;

use simthing_core::{SubFieldRole, ThresholdDirection};
use simthing_spec::{
    compile_resource_economy, CompiledEmissionFormula, EmissionFormulaSpec, EmitBufferSpec,
    EmitOnThresholdSpec, PropertyKey, RecipeInputSpec, ResourceEconomySpec, ResourceEmissionSpec,
    ResourceRecipeSpec, TriggerDirection,
};
use support::{amount_property, exact_eml_registry, register_amount_property};

fn pk(ns: &str, name: &str) -> PropertyKey {
    PropertyKey::new(ns, name)
}

#[test]
fn resource_economy_compile_valid_transfer() {
    let mut reg = support::empty_registry();
    let food = register_amount_property(&mut reg, "core", "food");
    let store = register_amount_property(&mut reg, "core", "store");
    let eml = exact_eml_registry(&[]);
    let spec = ResourceEconomySpec {
        transfers: vec![amount_property("t1", "food", "store", 3.5, 2)],
        ..Default::default()
    };
    let compiled = compile_resource_economy(&spec, &reg, &eml).unwrap();
    assert_eq!(compiled.transfers.len(), 1);
    let t = &compiled.transfers[0];
    assert_eq!(t.id, "t1");
    assert_eq!(t.source_property, food);
    assert_eq!(t.target_property, store);
    assert_eq!(t.amount, 3.5);
    assert_eq!(t.order_band, 2);
    assert_eq!(t.source_col, 0);
    assert_eq!(t.target_col, 1);
}

#[test]
fn resource_economy_compile_valid_recipe() {
    let mut reg = support::empty_registry();
    let food = register_amount_property(&mut reg, "core", "food");
    let water = register_amount_property(&mut reg, "core", "water");
    let out = register_amount_property(&mut reg, "core", "meal");
    let eml = exact_eml_registry(&[]);
    let spec = ResourceEconomySpec {
        recipes: vec![ResourceRecipeSpec {
            id: "r1".into(),
            inputs: vec![
                RecipeInputSpec {
                    property: pk("core", "food"),
                    role: SubFieldRole::Named("amount".into()),
                    unit_cost: 2.0,
                },
                RecipeInputSpec {
                    property: pk("core", "water"),
                    role: SubFieldRole::Named("amount".into()),
                    unit_cost: 1.0,
                },
            ],
            target: pk("core", "meal"),
            target_role: SubFieldRole::Named("amount".into()),
            throttle_hint_max_per_tick: 4,
        }],
        ..Default::default()
    };
    let compiled = compile_resource_economy(&spec, &reg, &eml).unwrap();
    assert_eq!(compiled.recipes.len(), 1);
    let r = &compiled.recipes[0];
    assert_eq!(r.id, "r1");
    assert_eq!(r.inputs.len(), 2);
    assert_eq!(r.inputs[0].property, food);
    assert_eq!(r.inputs[1].property, water);
    assert_eq!(r.target_property, out);
    assert_eq!(r.throttle_hint_max_per_tick, 4);
}

#[test]
fn resource_economy_compile_valid_identity_floor_emission() {
    let mut reg = support::empty_registry();
    register_amount_property(&mut reg, "core", "food");
    let eml = exact_eml_registry(&[]);
    let spec = ResourceEconomySpec {
        emissions: vec![ResourceEmissionSpec {
            id: "e1".into(),
            source: pk("core", "food"),
            source_role: SubFieldRole::Named("amount".into()),
            formula: EmissionFormulaSpec::IdentityFloor,
        }],
        ..Default::default()
    };
    let compiled = compile_resource_economy(&spec, &reg, &eml).unwrap();
    assert_eq!(compiled.emissions.len(), 1);
    assert_eq!(
        compiled.emissions[0].formula,
        CompiledEmissionFormula::IdentityFloor
    );
}

#[test]
fn resource_economy_compile_valid_constant_emission() {
    let mut reg = support::empty_registry();
    register_amount_property(&mut reg, "core", "food");
    let eml = exact_eml_registry(&[]);
    let spec = ResourceEconomySpec {
        emissions: vec![ResourceEmissionSpec {
            id: "e1".into(),
            source: pk("core", "food"),
            source_role: SubFieldRole::Named("amount".into()),
            formula: EmissionFormulaSpec::Constant(7.25),
        }],
        ..Default::default()
    };
    let compiled = compile_resource_economy(&spec, &reg, &eml).unwrap();
    assert_eq!(
        compiled.emissions[0].formula,
        CompiledEmissionFormula::Constant(7.25)
    );
}

#[test]
fn resource_economy_compile_valid_eval_eml_emission() {
    let mut reg = support::empty_registry();
    register_amount_property(&mut reg, "core", "food");
    let eml = exact_eml_registry(&[("food_emission_v1", 42)]);
    let spec = ResourceEconomySpec {
        emissions: vec![ResourceEmissionSpec {
            id: "e1".into(),
            source: pk("core", "food"),
            source_role: SubFieldRole::Named("amount".into()),
            formula: EmissionFormulaSpec::EvalEml {
                formula_key: "food_emission_v1".into(),
            },
        }],
        ..Default::default()
    };
    let compiled = compile_resource_economy(&spec, &reg, &eml).unwrap();
    match &compiled.emissions[0].formula {
        CompiledEmissionFormula::EvalEml {
            formula_key,
            tree_id,
        } => {
            assert_eq!(formula_key, "food_emission_v1");
            assert_eq!(tree_id.0, 42);
        }
        other => panic!("expected EvalEml, got {other:?}"),
    }
}

#[test]
fn resource_economy_compile_valid_emit_on_threshold() {
    let mut reg = support::empty_registry();
    register_amount_property(&mut reg, "core", "heat");
    let eml = exact_eml_registry(&[]);
    let spec = ResourceEconomySpec {
        emit_on_threshold: vec![EmitOnThresholdSpec {
            id: "th1".into(),
            source: pk("core", "heat"),
            source_role: SubFieldRole::Named("amount".into()),
            threshold: 100.0,
            direction: TriggerDirection::Rising,
            event_kind: 3,
            buffer: EmitBufferSpec::Output,
        }],
        ..Default::default()
    };
    let compiled = compile_resource_economy(&spec, &reg, &eml).unwrap();
    assert_eq!(compiled.emit_on_threshold.len(), 1);
    let th = &compiled.emit_on_threshold[0];
    assert_eq!(th.id, "th1");
    assert_eq!(th.threshold, 100.0);
    assert_eq!(th.direction, ThresholdDirection::Upward);
    assert_eq!(th.event_kind, 3);
    assert_eq!(th.buffer, EmitBufferSpec::Output);
}

#[test]
fn resource_economy_expansion_report_counts_all_variants() {
    let mut reg = support::empty_registry();
    register_amount_property(&mut reg, "core", "food");
    register_amount_property(&mut reg, "core", "water");
    register_amount_property(&mut reg, "core", "store");
    register_amount_property(&mut reg, "core", "meal");
    register_amount_property(&mut reg, "core", "heat");
    let eml = exact_eml_registry(&[("food_emission_v1", 1)]);
    let spec = ResourceEconomySpec {
        transfers: vec![amount_property("t1", "food", "store", 1.0, 1)],
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
            throttle_hint_max_per_tick: 1,
        }],
        emissions: vec![
            ResourceEmissionSpec {
                id: "e1".into(),
                source: pk("core", "food"),
                source_role: SubFieldRole::Named("amount".into()),
                formula: EmissionFormulaSpec::IdentityFloor,
            },
            ResourceEmissionSpec {
                id: "e2".into(),
                source: pk("core", "food"),
                source_role: SubFieldRole::Named("amount".into()),
                formula: EmissionFormulaSpec::Constant(1.0),
            },
            ResourceEmissionSpec {
                id: "e3".into(),
                source: pk("core", "food"),
                source_role: SubFieldRole::Named("amount".into()),
                formula: EmissionFormulaSpec::EvalEml {
                    formula_key: "food_emission_v1".into(),
                },
            },
        ],
        emit_on_threshold: vec![EmitOnThresholdSpec {
            id: "th1".into(),
            source: pk("core", "heat"),
            source_role: SubFieldRole::Named("amount".into()),
            threshold: 1.0,
            direction: TriggerDirection::Falling,
            event_kind: 1,
            buffer: EmitBufferSpec::Values,
        }],
        ..Default::default()
    };
    let compiled = compile_resource_economy(&spec, &reg, &eml).unwrap();
    let report = &compiled.report;
    assert_eq!(report.transfer_count, 1);
    assert_eq!(report.recipe_count, 1);
    assert_eq!(report.recipe_input_count, 2);
    assert_eq!(report.emission_count, 3);
    assert_eq!(report.threshold_emit_count, 1);
    assert_eq!(report.eval_eml_emission_count, 1);
    assert!(report.diagnostics.is_empty());
}

#[test]
fn resource_economy_expansion_report_order_is_stable() {
    let mut reg = support::empty_registry();
    register_amount_property(&mut reg, "core", "food");
    register_amount_property(&mut reg, "core", "store");
    let eml = exact_eml_registry(&[]);
    let spec = ResourceEconomySpec {
        transfers: vec![
            amount_property("t1", "food", "store", 1.0, 0),
            amount_property("t2", "food", "store", 2.0, 1),
        ],
        ..Default::default()
    };
    let first = compile_resource_economy(&spec, &reg, &eml).unwrap();
    let second = compile_resource_economy(&spec, &reg, &eml).unwrap();
    assert_eq!(first.report, second.report);
    assert_eq!(first.transfers[0].id, second.transfers[0].id);
    assert_eq!(first.transfers[1].id, second.transfers[1].id);
}
