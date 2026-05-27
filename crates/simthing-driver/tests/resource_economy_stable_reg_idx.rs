//! Phase T-3 — stable emission reg_idx assignment tests.

#[path = "support/resource_economy_materialize.rs"]
mod support;

use simthing_driver::materialize_resource_economy_registrations;
use simthing_spec::{
    EmissionFormulaSpec, PropertyKey, ResourceEconomySpec, ResourceEmissionSpec,
};
use support::{
    amount_transfer, compile_fixture, exact_eml_registry, register_amount_property,
};

fn emission_spec(id: &str) -> ResourceEmissionSpec {
    ResourceEmissionSpec {
        id: id.into(),
        source: PropertyKey::new("core", "food"),
        source_role: simthing_core::SubFieldRole::Named("amount".into()),
        formula: EmissionFormulaSpec::IdentityFloor,
    }
}

fn emission_reg_map(spec: &ResourceEconomySpec) -> std::collections::BTreeMap<String, u32> {
    let mut reg = support::empty_registry();
    register_amount_property(&mut reg, "core", "food");
    let eml = exact_eml_registry(&[]);
    let compiled = compile_fixture(spec, &reg, &eml);
    materialize_resource_economy_registrations(&compiled, &reg, &eml)
        .unwrap()
        .report
        .emission_reg_idx_by_id
}

#[test]
fn resource_economy_stable_reg_idx_same_spec_same_order() {
    let spec = ResourceEconomySpec {
        emissions: vec![
            emission_spec("alpha"),
            emission_spec("beta"),
            emission_spec("gamma"),
        ],
        ..Default::default()
    };
    let first = emission_reg_map(&spec);
    let second = emission_reg_map(&spec);
    assert_eq!(first, second);
    assert_eq!(first.get("alpha"), Some(&0));
    assert_eq!(first.get("beta"), Some(&1));
    assert_eq!(first.get("gamma"), Some(&2));
}

#[test]
fn resource_economy_stable_reg_idx_same_spec_different_process_order() {
    let spec_a = ResourceEconomySpec {
        emissions: vec![
            emission_spec("alpha"),
            emission_spec("beta"),
            emission_spec("gamma"),
        ],
        ..Default::default()
    };
    let spec_b = ResourceEconomySpec {
        emissions: vec![
            emission_spec("gamma"),
            emission_spec("alpha"),
            emission_spec("beta"),
        ],
        ..Default::default()
    };
    assert_eq!(emission_reg_map(&spec_a), emission_reg_map(&spec_b));
}

#[test]
fn resource_economy_stable_reg_idx_survives_unrelated_authoring_order_change() {
    let base = ResourceEconomySpec {
        emissions: vec![emission_spec("alpha"), emission_spec("beta")],
        ..Default::default()
    };

    let mut reg = support::empty_registry();
    register_amount_property(&mut reg, "core", "food");
    register_amount_property(&mut reg, "core", "store");
    let eml = exact_eml_registry(&[]);

    let with_reordered_transfers = ResourceEconomySpec {
        transfers: vec![
            amount_transfer("t2", "food", "store", 1.0, 1),
            amount_transfer("t1", "food", "store", 2.0, 2),
        ],
        emissions: vec![emission_spec("beta"), emission_spec("alpha")],
        ..Default::default()
    };

    let base_map = emission_reg_map(&base);
    let compiled = compile_fixture(&with_reordered_transfers, &reg, &eml);
    let noisy_map = materialize_resource_economy_registrations(&compiled, &reg, &eml)
        .unwrap()
        .report
        .emission_reg_idx_by_id;

    assert_eq!(base_map, noisy_map);
    assert_eq!(base_map.get("alpha"), Some(&0));
    assert_eq!(base_map.get("beta"), Some(&1));
}
