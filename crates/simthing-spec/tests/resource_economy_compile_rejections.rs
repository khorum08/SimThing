//! Phase T-2 — resource economy compile rejection tests.

#[path = "support/resource_economy_compile.rs"]
mod support;

use simthing_core::SubFieldRole;
use simthing_spec::{
    compile_resource_economy, EmissionFormulaSpec, PropertyKey, RecipeInputSpec,
    ResourceEconomySpec, ResourceEmissionSpec, ResourceRecipeSpec, ResourceTransferSpec, SpecError,
};
use support::{
    amount_property, exact_eml_registry, fast_eml_registry, register_amount_property,
};

fn pk(ns: &str, name: &str) -> PropertyKey {
    PropertyKey::new(ns, name)
}

#[test]
fn resource_economy_rejects_unknown_source_property() {
    let reg = support::empty_registry();
    let eml = exact_eml_registry(&[]);
    let spec = ResourceEconomySpec {
        transfers: vec![ResourceTransferSpec {
            id: "t1".into(),
            source: pk("core", "missing"),
            source_role: SubFieldRole::Named("amount".into()),
            target: pk("core", "store"),
            target_role: SubFieldRole::Named("amount".into()),
            amount: 1.0,
            order_band: 0,
        }],
        ..Default::default()
    };
    assert_eq!(
        compile_resource_economy(&spec, &reg, &eml),
        Err(SpecError::UnknownResourceEconomySourceProperty {
            transfer: "t1".into(),
            namespace: "core".into(),
            name: "missing".into(),
        })
    );
}

#[test]
fn resource_economy_rejects_unknown_target_property() {
    let mut reg = support::empty_registry();
    register_amount_property(&mut reg, "core", "food");
    let eml = exact_eml_registry(&[]);
    let spec = ResourceEconomySpec {
        transfers: vec![ResourceTransferSpec {
            id: "t1".into(),
            source: pk("core", "food"),
            source_role: SubFieldRole::Named("amount".into()),
            target: pk("core", "missing"),
            target_role: SubFieldRole::Named("amount".into()),
            amount: 1.0,
            order_band: 0,
        }],
        ..Default::default()
    };
    assert_eq!(
        compile_resource_economy(&spec, &reg, &eml),
        Err(SpecError::UnknownResourceEconomyTargetProperty {
            transfer: "t1".into(),
            namespace: "core".into(),
            name: "missing".into(),
        })
    );
}

#[test]
fn resource_economy_rejects_unknown_subfield_role() {
    let mut reg = support::empty_registry();
    register_amount_property(&mut reg, "core", "food");
    let eml = exact_eml_registry(&[]);
    let spec = ResourceEconomySpec {
        transfers: vec![ResourceTransferSpec {
            id: "t1".into(),
            source: pk("core", "food"),
            source_role: SubFieldRole::Named("missing".into()),
            target: pk("core", "food"),
            target_role: SubFieldRole::Named("amount".into()),
            amount: 1.0,
            order_band: 0,
        }],
        ..Default::default()
    };
    let err = compile_resource_economy(&spec, &reg, &eml).unwrap_err();
    assert!(matches!(
        err,
        SpecError::InvalidResourceEconomyRole { .. }
    ));
}

#[test]
fn resource_economy_rejects_duplicate_authoring_id() {
    let mut reg = support::empty_registry();
    register_amount_property(&mut reg, "core", "food");
    let eml = exact_eml_registry(&[]);
    let spec = ResourceEconomySpec {
        transfers: vec![
            ResourceTransferSpec {
                id: "dup".into(),
                source: pk("core", "food"),
                source_role: SubFieldRole::Named("amount".into()),
                target: pk("core", "food"),
                target_role: SubFieldRole::Named("amount".into()),
                amount: 1.0,
                order_band: 0,
            },
            ResourceTransferSpec {
                id: "dup".into(),
                source: pk("core", "food"),
                source_role: SubFieldRole::Named("amount".into()),
                target: pk("core", "food"),
                target_role: SubFieldRole::Named("amount".into()),
                amount: 2.0,
                order_band: 1,
            },
        ],
        ..Default::default()
    };
    assert_eq!(
        compile_resource_economy(&spec, &reg, &eml),
        Err(SpecError::DuplicateResourceEconomyId {
            id: "dup".into()
        })
    );
}

#[test]
fn resource_economy_rejects_negative_transfer_amount() {
    let mut reg = support::empty_registry();
    register_amount_property(&mut reg, "core", "food");
    register_amount_property(&mut reg, "core", "store");
    let eml = exact_eml_registry(&[]);
    let spec = ResourceEconomySpec {
        transfers: vec![amount_property("t1", "food", "store", -1.0, 0)],
        ..Default::default()
    };
    assert_eq!(
        compile_resource_economy(&spec, &reg, &eml),
        Err(SpecError::InvalidTransferAmount {
            transfer: "t1".into()
        })
    );
}

#[test]
fn resource_economy_rejects_zero_recipe_unit_cost() {
    let mut reg = support::empty_registry();
    register_amount_property(&mut reg, "core", "food");
    register_amount_property(&mut reg, "core", "out");
    let eml = exact_eml_registry(&[]);
    let spec = ResourceEconomySpec {
        recipes: vec![ResourceRecipeSpec {
            id: "r1".into(),
            inputs: vec![RecipeInputSpec {
                property: pk("core", "food"),
                role: SubFieldRole::Named("amount".into()),
                unit_cost: 0.0,
            }],
            target: pk("core", "out"),
            target_role: SubFieldRole::Named("amount".into()),
            throttle_hint_max_per_tick: 1,
        }],
        ..Default::default()
    };
    assert_eq!(
        compile_resource_economy(&spec, &reg, &eml),
        Err(SpecError::InvalidRecipeUnitCost {
            recipe: "r1".into()
        })
    );
}

#[test]
fn resource_economy_rejects_negative_recipe_unit_cost() {
    let mut reg = support::empty_registry();
    register_amount_property(&mut reg, "core", "food");
    register_amount_property(&mut reg, "core", "out");
    let eml = exact_eml_registry(&[]);
    let spec = ResourceEconomySpec {
        recipes: vec![ResourceRecipeSpec {
            id: "r1".into(),
            inputs: vec![RecipeInputSpec {
                property: pk("core", "food"),
                role: SubFieldRole::Named("amount".into()),
                unit_cost: -1.0,
            }],
            target: pk("core", "out"),
            target_role: SubFieldRole::Named("amount".into()),
            throttle_hint_max_per_tick: 1,
        }],
        ..Default::default()
    };
    assert_eq!(
        compile_resource_economy(&spec, &reg, &eml),
        Err(SpecError::InvalidRecipeUnitCost {
            recipe: "r1".into()
        })
    );
}

#[test]
fn resource_economy_rejects_empty_recipe_inputs() {
    let mut reg = support::empty_registry();
    register_amount_property(&mut reg, "core", "out");
    let eml = exact_eml_registry(&[]);
    let spec = ResourceEconomySpec {
        recipes: vec![ResourceRecipeSpec {
            id: "r1".into(),
            inputs: vec![],
            target: pk("core", "out"),
            target_role: SubFieldRole::Named("amount".into()),
            throttle_hint_max_per_tick: 1,
        }],
        ..Default::default()
    };
    assert_eq!(
        compile_resource_economy(&spec, &reg, &eml),
        Err(SpecError::EmptyRecipeInputs {
            recipe: "r1".into()
        })
    );
}

#[test]
fn resource_economy_rejects_unknown_eml_formula() {
    let mut reg = support::empty_registry();
    register_amount_property(&mut reg, "core", "food");
    let eml = exact_eml_registry(&[]);
    let spec = ResourceEconomySpec {
        emissions: vec![ResourceEmissionSpec {
            id: "e1".into(),
            source: pk("core", "food"),
            source_role: SubFieldRole::Named("amount".into()),
            formula: EmissionFormulaSpec::EvalEml {
                formula_key: "missing_formula".into(),
            },
        }],
        ..Default::default()
    };
    assert_eq!(
        compile_resource_economy(&spec, &reg, &eml),
        Err(SpecError::UnknownEmissionFormulaKey {
            emission: "e1".into(),
            formula_key: "missing_formula".into(),
        })
    );
}

#[test]
fn resource_economy_rejects_soft_or_fast_emission_eml() {
    let mut reg = support::empty_registry();
    register_amount_property(&mut reg, "core", "food");
    let eml = fast_eml_registry("fast_emit", 9);
    let spec = ResourceEconomySpec {
        emissions: vec![ResourceEmissionSpec {
            id: "e1".into(),
            source: pk("core", "food"),
            source_role: SubFieldRole::Named("amount".into()),
            formula: EmissionFormulaSpec::EvalEml {
                formula_key: "fast_emit".into(),
            },
        }],
        ..Default::default()
    };
    assert_eq!(
        compile_resource_economy(&spec, &reg, &eml),
        Err(SpecError::EmissionEmlNotExactDeterministic {
            emission: "e1".into(),
            formula_key: "fast_emit".into(),
        })
    );
}

#[test]
fn resource_economy_rejects_same_band_consumed_input_contention_when_detectable() {
    let mut reg = support::empty_registry();
    register_amount_property(&mut reg, "core", "food");
    register_amount_property(&mut reg, "core", "store");
    let eml = exact_eml_registry(&[]);
    let spec = ResourceEconomySpec {
        transfers: vec![
            amount_property("t1", "food", "store", 1.0, 0),
            amount_property("t2", "food", "store", 2.0, 0),
        ],
        ..Default::default()
    };
    let err = compile_resource_economy(&spec, &reg, &eml).unwrap_err();
    assert!(matches!(
        err,
        SpecError::ResourceEconomyConsumedInputContention { .. }
    ));
}
