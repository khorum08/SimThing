//! Phase T-3 — resource economy driver materialization tests.

#[path = "support/resource_economy_materialize.rs"]
mod support;

use simthing_core::{
    discrete_transfer_registration_to_op, rebuild_conjunctive_recipe_ops,
    rebuild_emit_on_threshold_ops, ConsumeMode, EmitOnThresholdBuffer, ThresholdDirection,
};
use simthing_driver::materialize_resource_economy_registrations;
use simthing_gpu::{plan_emission_ops, EmissionFormula};
use simthing_spec::ResourceEconomySpec;
use support::{
    amount_transfer, compile_fixture, exact_eml_registry, full_fixture_spec, pk,
    register_amount_property, register_full_fixture,
};

#[test]
fn resource_economy_materializes_valid_transfer_registration() {
    let mut reg = support::empty_registry();
    let food = register_amount_property(&mut reg, "core", "food");
    let store = register_amount_property(&mut reg, "core", "store");
    let eml = exact_eml_registry(&[]);
    let spec = ResourceEconomySpec {
        transfers: vec![amount_transfer("t1", "food", "store", 3.5, 2)],
        ..Default::default()
    };
    let compiled = compile_fixture(&spec, &reg, &eml);
    let materialized = materialize_resource_economy_registrations(&compiled, &reg, &eml).unwrap();

    assert_eq!(materialized.transfers.len(), 1);
    let t = &materialized.transfers[0];
    assert_eq!(t.source_slot, food.0);
    assert_eq!(t.target_slot, store.0);
    assert_eq!(t.source_col, 0);
    assert_eq!(t.target_col, 1);
    assert_eq!(t.amount, 3.5);

    let op = discrete_transfer_registration_to_op(t).unwrap();
    assert_eq!(op.consume, ConsumeMode::SubtractFromSource);
    assert_eq!(materialized.report.transfer_ids, vec!["t1".to_string()]);
    assert_eq!(materialized.report.transfer_order_band_by_id.get("t1"), Some(&2));
}

#[test]
fn resource_economy_materializes_valid_recipe_registration() {
    let mut reg = support::empty_registry();
    let food = register_amount_property(&mut reg, "core", "food");
    let water = register_amount_property(&mut reg, "core", "water");
    let meal = register_amount_property(&mut reg, "core", "meal");
    let eml = exact_eml_registry(&[]);
    let spec = ResourceEconomySpec {
        recipes: vec![simthing_spec::ResourceRecipeSpec {
            id: "r1".into(),
            inputs: vec![
                simthing_spec::RecipeInputSpec {
                    property: pk("core", "food"),
                    role: simthing_core::SubFieldRole::Named("amount".into()),
                    unit_cost: 2.0,
                },
                simthing_spec::RecipeInputSpec {
                    property: pk("core", "water"),
                    role: simthing_core::SubFieldRole::Named("amount".into()),
                    unit_cost: 1.0,
                },
            ],
            target: pk("core", "meal"),
            target_role: simthing_core::SubFieldRole::Named("amount".into()),
            throttle_hint_max_per_tick: 4,
        }],
        ..Default::default()
    };
    let compiled = compile_fixture(&spec, &reg, &eml);
    let materialized = materialize_resource_economy_registrations(&compiled, &reg, &eml).unwrap();

    assert_eq!(materialized.recipes.len(), 1);
    let r = &materialized.recipes[0];
    assert_eq!(r.inputs.len(), 2);
    assert_eq!(r.inputs[0].slot, food.0);
    assert_eq!(r.inputs[0].col, 0);
    assert_eq!(r.inputs[0].unit_cost, 2.0);
    assert_eq!(r.inputs[1].slot, water.0);
    assert_eq!(r.target_slot, meal.0);
    assert_eq!(r.throttle_hint_max_per_tick, 4);

    let ops = rebuild_conjunctive_recipe_ops(&materialized.recipes).unwrap();
    assert_eq!(ops.len(), 1);
    assert_eq!(ops[0].consume, ConsumeMode::SubtractFromAllInputs);
}

#[test]
fn resource_economy_materializes_identity_floor_emission_registration() {
    let mut reg = support::empty_registry();
    register_amount_property(&mut reg, "core", "food");
    let eml = exact_eml_registry(&[]);
    let spec = ResourceEconomySpec {
        emissions: vec![simthing_spec::ResourceEmissionSpec {
            id: "e1".into(),
            source: pk("core", "food"),
            source_role: simthing_core::SubFieldRole::Named("amount".into()),
            formula: simthing_spec::EmissionFormulaSpec::IdentityFloor,
        }],
        ..Default::default()
    };
    let compiled = compile_fixture(&spec, &reg, &eml);
    let materialized = materialize_resource_economy_registrations(&compiled, &reg, &eml).unwrap();

    assert_eq!(materialized.emissions.len(), 1);
    assert_eq!(materialized.emissions[0].formula, EmissionFormula::IdentityFloor);
    assert_eq!(materialized.emissions[0].max_emit, None);
    plan_emission_ops(&materialized.emissions, Some(&eml)).unwrap();
}

#[test]
fn resource_economy_materializes_constant_emission_registration() {
    let mut reg = support::empty_registry();
    register_amount_property(&mut reg, "core", "food");
    let eml = exact_eml_registry(&[]);
    let spec = ResourceEconomySpec {
        emissions: vec![simthing_spec::ResourceEmissionSpec {
            id: "e1".into(),
            source: pk("core", "food"),
            source_role: simthing_core::SubFieldRole::Named("amount".into()),
            formula: simthing_spec::EmissionFormulaSpec::Constant(7.25),
        }],
        ..Default::default()
    };
    let compiled = compile_fixture(&spec, &reg, &eml);
    let materialized = materialize_resource_economy_registrations(&compiled, &reg, &eml).unwrap();

    assert_eq!(
        materialized.emissions[0].formula,
        EmissionFormula::Constant { value: 7.25 }
    );
}

#[test]
fn resource_economy_materializes_eval_eml_emission_registration() {
    let mut reg = support::empty_registry();
    register_amount_property(&mut reg, "core", "food");
    let eml = exact_eml_registry(&[("food_emission_v1", 42)]);
    let spec = ResourceEconomySpec {
        emissions: vec![simthing_spec::ResourceEmissionSpec {
            id: "e1".into(),
            source: pk("core", "food"),
            source_role: simthing_core::SubFieldRole::Named("amount".into()),
            formula: simthing_spec::EmissionFormulaSpec::EvalEml {
                formula_key: "food_emission_v1".into(),
            },
        }],
        ..Default::default()
    };
    let compiled = compile_fixture(&spec, &reg, &eml);
    let materialized = materialize_resource_economy_registrations(&compiled, &reg, &eml).unwrap();

    match &materialized.emissions[0].formula {
        EmissionFormula::EvalEml { tree_id } => assert_eq!(tree_id.0, 42),
        other => panic!("expected EvalEml, got {other:?}"),
    }
    assert_eq!(materialized.emissions[0].tree_id, Some(simthing_core::EmlTreeId(42)));
    plan_emission_ops(&materialized.emissions, Some(&eml)).unwrap();
}

#[test]
fn resource_economy_materializes_emit_on_threshold_registration() {
    let mut reg = support::empty_registry();
    register_amount_property(&mut reg, "core", "heat");
    let eml = exact_eml_registry(&[]);
    let spec = ResourceEconomySpec {
        emit_on_threshold: vec![simthing_spec::EmitOnThresholdSpec {
            id: "th1".into(),
            source: pk("core", "heat"),
            source_role: simthing_core::SubFieldRole::Named("amount".into()),
            threshold: 100.0,
            direction: simthing_spec::TriggerDirection::Rising,
            event_kind: 3,
            buffer: simthing_spec::EmitBufferSpec::Output,
        }],
        ..Default::default()
    };
    let compiled = compile_fixture(&spec, &reg, &eml);
    let materialized = materialize_resource_economy_registrations(&compiled, &reg, &eml).unwrap();

    assert_eq!(materialized.emit_on_threshold.len(), 1);
    let th = &materialized.emit_on_threshold[0];
    assert_eq!(th.threshold, 100.0);
    assert_eq!(th.direction, ThresholdDirection::Upward);
    assert_eq!(th.event_kind, 3);
    assert_eq!(th.buffer, EmitOnThresholdBuffer::Output);
    rebuild_emit_on_threshold_ops(&materialized.emit_on_threshold);
}

#[test]
fn resource_economy_materialization_report_counts_all_variants() {
    let mut reg = support::empty_registry();
    register_full_fixture(&mut reg);
    let eml = exact_eml_registry(&[("food_emission_v1", 1)]);
    let spec = full_fixture_spec();
    let compiled = compile_fixture(&spec, &reg, &eml);
    let materialized = materialize_resource_economy_registrations(&compiled, &reg, &eml).unwrap();
    let report = &materialized.report;

    assert_eq!(report.transfer_count, 1);
    assert_eq!(report.recipe_count, 1);
    assert_eq!(report.recipe_input_count, 2);
    assert_eq!(report.emission_count, 3);
    assert_eq!(report.threshold_emit_count, 1);
    assert_eq!(report.eval_eml_emission_count, 1);
    assert_eq!(report.transfer_ids, vec!["t1".to_string()]);
    assert_eq!(report.recipe_ids, vec!["r1".to_string()]);
    assert_eq!(report.threshold_emit_ids, vec!["th1".to_string()]);
}

#[test]
fn resource_economy_no_simthing_sim_spec_import() {
    let sim_cargo = include_str!("../../simthing-sim/Cargo.toml");
    assert!(
        !sim_cargo.contains("simthing-spec"),
        "simthing-sim must not depend on simthing-spec"
    );
}
