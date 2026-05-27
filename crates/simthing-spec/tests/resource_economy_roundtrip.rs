//! Phase T-1 — resource economy authoring RON roundtrip tests.

use simthing_core::SubFieldRole;
use simthing_spec::{
    EmissionFormulaSpec, EmitBufferSpec, EmitOnThresholdSpec, PropertyKey, RecipeInputSpec,
    ResourceEconomySpec, ResourceEmissionSpec, ResourceRecipeSpec, ResourceTransferSpec,
    TriggerDirection,
};

fn roundtrip_ron<T>(value: &T)
where
    T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug + PartialEq,
{
    let text = ron::ser::to_string_pretty(value, ron::ser::PrettyConfig::default())
        .expect("serialize RON");
    let decoded: T = ron::from_str(&text).expect("deserialize RON");
    assert_eq!(*value, decoded, "RON roundtrip mismatch for {text}");
}

fn sample_property(ns: &str, name: &str) -> PropertyKey {
    PropertyKey::new(ns, name)
}

fn sample_transfer() -> ResourceTransferSpec {
    ResourceTransferSpec {
        id: "food_to_store".into(),
        source: sample_property("core", "food"),
        source_role: SubFieldRole::Named("amount".into()),
        target: sample_property("core", "store"),
        target_role: SubFieldRole::Named("amount".into()),
        amount: 5.0,
        order_band: 2,
    }
}

#[test]
fn resource_transfer_spec_roundtrip() {
    roundtrip_ron(&sample_transfer());
}

#[test]
fn resource_recipe_spec_roundtrip() {
    let spec = ResourceRecipeSpec {
        id: "craft_ration".into(),
        inputs: vec![
            RecipeInputSpec {
                property: sample_property("core", "food"),
                role: SubFieldRole::Named("amount".into()),
                unit_cost: 2.0,
            },
            RecipeInputSpec {
                property: sample_property("core", "water"),
                role: SubFieldRole::Named("amount".into()),
                unit_cost: 1.0,
            },
        ],
        target: sample_property("core", "ration"),
        target_role: SubFieldRole::Named("amount".into()),
        throttle_hint_max_per_tick: 4,
    };
    roundtrip_ron(&spec);
}

#[test]
fn resource_emission_identity_floor_roundtrip() {
    let spec = ResourceEmissionSpec {
        id: "emit_food_floor".into(),
        source: sample_property("core", "food"),
        source_role: SubFieldRole::Named("amount".into()),
        formula: EmissionFormulaSpec::IdentityFloor,
    };
    roundtrip_ron(&spec);
}

#[test]
fn resource_emission_constant_roundtrip() {
    let spec = ResourceEmissionSpec {
        id: "emit_flat".into(),
        source: sample_property("core", "signal"),
        source_role: SubFieldRole::Named("value".into()),
        formula: EmissionFormulaSpec::Constant(3.5),
    };
    roundtrip_ron(&spec);
}

#[test]
fn resource_emission_eval_eml_roundtrip() {
    let spec = ResourceEmissionSpec {
        id: "emit_eml".into(),
        source: sample_property("core", "food"),
        source_role: SubFieldRole::Named("flow".into()),
        formula: EmissionFormulaSpec::EvalEml {
            formula_key: "food_emission_v1".into(),
        },
    };
    roundtrip_ron(&spec);
}

#[test]
fn emit_on_threshold_spec_roundtrip() {
    let spec = EmitOnThresholdSpec {
        id: "food_low_alert".into(),
        source: sample_property("core", "food"),
        source_role: SubFieldRole::Named("amount".into()),
        threshold: 10.0,
        direction: TriggerDirection::Falling,
        event_kind: 42,
        buffer: EmitBufferSpec::Output,
    };
    roundtrip_ron(&spec);
}

#[test]
fn resource_economy_spec_roundtrip_all_variants() {
    let spec = ResourceEconomySpec {
        transfers: vec![sample_transfer()],
        recipes: vec![ResourceRecipeSpec {
            id: "craft".into(),
            inputs: vec![RecipeInputSpec {
                property: sample_property("core", "food"),
                role: SubFieldRole::Named("amount".into()),
                unit_cost: 1.0,
            }],
            target: sample_property("core", "goods"),
            target_role: SubFieldRole::Named("amount".into()),
            throttle_hint_max_per_tick: 1,
        }],
        emissions: vec![
            ResourceEmissionSpec {
                id: "e1".into(),
                source: sample_property("core", "food"),
                source_role: SubFieldRole::Named("amount".into()),
                formula: EmissionFormulaSpec::IdentityFloor,
            },
            ResourceEmissionSpec {
                id: "e2".into(),
                source: sample_property("core", "food"),
                source_role: SubFieldRole::Named("amount".into()),
                formula: EmissionFormulaSpec::Constant(1.0),
            },
            ResourceEmissionSpec {
                id: "e3".into(),
                source: sample_property("core", "food"),
                source_role: SubFieldRole::Named("amount".into()),
                formula: EmissionFormulaSpec::EvalEml {
                    formula_key: "k".into(),
                },
            },
        ],
        emit_on_threshold: vec![EmitOnThresholdSpec {
            id: "t1".into(),
            source: sample_property("core", "food"),
            source_role: SubFieldRole::Named("amount".into()),
            threshold: 0.0,
            direction: TriggerDirection::Rising,
            event_kind: 1,
            buffer: EmitBufferSpec::Values,
        }],
        ..Default::default()
    };
    roundtrip_ron(&spec);
}

#[test]
fn resource_economy_missing_lists_default_empty() {
    let text = r#"(
        transfers: [],
    )"#;
    let spec: ResourceEconomySpec = ron::from_str(text).expect("parse partial economy");
    assert!(spec.transfers.is_empty());
    assert!(spec.recipes.is_empty());
    assert!(spec.emissions.is_empty());
    assert!(spec.emit_on_threshold.is_empty());
    assert_eq!(
        spec.opt_in_mode,
        simthing_spec::ResourceEconomyOptInMode::Disabled
    );
}

#[test]
fn resource_recipe_throttle_hint_roundtrip_metadata_only() {
    let spec = ResourceRecipeSpec {
        id: "hint_only".into(),
        inputs: vec![RecipeInputSpec {
            property: sample_property("core", "input"),
            role: SubFieldRole::Named("amount".into()),
            unit_cost: 1.0,
        }],
        target: sample_property("core", "output"),
        target_role: SubFieldRole::Named("amount".into()),
        throttle_hint_max_per_tick: 99,
    };
    roundtrip_ron(&spec);
    assert_eq!(spec.throttle_hint_max_per_tick, 99);
}

#[test]
fn resource_emission_spec_does_not_expose_max_emit() {
    let ron_with_max_emit = r#"(
        id: "bad",
        source: (namespace: "core", name: "food"),
        source_role: Named("amount"),
        formula: IdentityFloor,
        max_emit: 10,
    )"#;
    let err = ron::from_str::<ResourceEmissionSpec>(ron_with_max_emit).unwrap_err();
    assert!(
        err.to_string().contains("unknown field") || err.to_string().contains("max_emit"),
        "expected unknown field rejection, got {err}"
    );
}

#[test]
fn resource_transfer_spec_rejects_unsafe_authoring_fields() {
    for (field, snippet) in [
        (
            "consume_mode",
            r#"(
                id: "x",
                source: (namespace: "core", name: "a"),
                source_role: Named("amount"),
                target: (namespace: "core", name: "b"),
                target_role: Named("amount"),
                amount: 1.0,
                order_band: 0,
                consume_mode: "SubtractFromSource",
            )"#,
        ),
        (
            "rate",
            r#"(
                id: "x",
                source: (namespace: "core", name: "a"),
                source_role: Named("amount"),
                target: (namespace: "core", name: "b"),
                target_role: Named("amount"),
                amount: 1.0,
                order_band: 0,
                rate: 0.5,
            )"#,
        ),
        (
            "probability",
            r#"(
                id: "x",
                source: (namespace: "core", name: "a"),
                source_role: Named("amount"),
                target: (namespace: "core", name: "b"),
                target_role: Named("amount"),
                amount: 1.0,
                order_band: 0,
                probability: 0.25,
            )"#,
        ),
    ] {
        let err = ron::from_str::<ResourceTransferSpec>(snippet).unwrap_err();
        assert!(
            err.to_string().contains(field),
            "expected rejection of {field}, got {err}"
        );
    }
}

#[test]
fn resource_recipe_spec_rejects_max_per_tick_alias() {
    let snippet = r#"(
        id: "bad",
        inputs: [],
        target: (namespace: "core", name: "out"),
        target_role: Named("amount"),
        max_per_tick: 3,
    )"#;
    let err = ron::from_str::<ResourceRecipeSpec>(snippet).unwrap_err();
    assert!(
        err.to_string().contains("max_per_tick"),
        "expected rejection of max_per_tick alias, got {err}"
    );
}
