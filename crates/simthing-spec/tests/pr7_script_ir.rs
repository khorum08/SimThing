use simthing_core::{DimensionRegistry, SubFieldRole};
use simthing_spec::{
    compile_property, PropertyKey, PropertySpec, ScopeRef, ScriptEvalContext, ScriptEvalError,
    ScriptExpr, ScriptPredicate,
};

fn registry_with_properties() -> DimensionRegistry {
    let mut registry = DimensionRegistry::new();
    compile_property(
        &PropertySpec {
            id:           "core_loyalty".into(),
            namespace:    "core".into(),
            name:         "loyalty".into(),
            display_name: "Loyalty".into(),
            description:  String::new(),
            sub_fields:   vec![],
        },
        &mut registry,
    )
    .expect("loyalty property");
    compile_property(
        &PropertySpec {
            id:           "core_stability".into(),
            namespace:    "core".into(),
            name:         "stability".into(),
            display_name: "Stability".into(),
            description:  String::new(),
            sub_fields:   vec![],
        },
        &mut registry,
    )
    .expect("stability property");
    registry
}

fn ctx<'a>(
    registry: &'a DimensionRegistry,
    shadow: &'a [f32],
    current_slot: u32,
) -> ScriptEvalContext<'a> {
    ScriptEvalContext {
        registry,
        shadow,
        n_dims: registry.total_columns,
        current_slot,
    }
}

fn read(ns: &str, name: &str, role: SubFieldRole) -> ScriptExpr {
    ScriptExpr::Read {
        scope:    ScopeRef::Current,
        property: PropertyKey::new(ns, name),
        role,
    }
}

#[test]
fn script_expr_reads_current_scope_value_from_shadow() {
    let registry = registry_with_properties();
    let loyalty = registry.id_of("core", "loyalty").unwrap();
    let amount_col = registry
        .column_range(loyalty)
        .col_for_role(&SubFieldRole::Amount, &registry.property(loyalty).layout)
        .unwrap();
    let mut shadow = vec![0.0; registry.total_columns * 2];
    shadow[registry.total_columns + amount_col] = 0.42;

    let expr = read("core", "loyalty", SubFieldRole::Amount);

    assert_eq!(expr.eval(&ctx(&registry, &shadow, 1)).unwrap(), 0.42);
}

#[test]
fn script_expr_reads_explicit_slot_scope() {
    let registry = registry_with_properties();
    let loyalty = registry.id_of("core", "loyalty").unwrap();
    let amount_col = registry
        .column_range(loyalty)
        .col_for_role(&SubFieldRole::Amount, &registry.property(loyalty).layout)
        .unwrap();
    let mut shadow = vec![0.0; registry.total_columns * 2];
    shadow[amount_col] = 0.25;
    shadow[registry.total_columns + amount_col] = 0.75;

    let expr = ScriptExpr::Read {
        scope:    ScopeRef::Slot(0),
        property: PropertyKey::new("core", "loyalty"),
        role:     SubFieldRole::Amount,
    };

    assert_eq!(expr.eval(&ctx(&registry, &shadow, 1)).unwrap(), 0.25);
}

#[test]
fn script_expr_evaluates_arithmetic_min_max_and_clamp() {
    let registry = registry_with_properties();
    let shadow = vec![0.0; registry.total_columns];
    let expr = ScriptExpr::Clamp {
        value: Box::new(ScriptExpr::Max(
            Box::new(ScriptExpr::Min(
                Box::new(ScriptExpr::Add(
                    Box::new(ScriptExpr::Const(10.0)),
                    Box::new(ScriptExpr::Mul(
                        Box::new(ScriptExpr::Const(2.0)),
                        Box::new(ScriptExpr::Const(4.0)),
                    )),
                )),
                Box::new(ScriptExpr::Const(20.0)),
            )),
            Box::new(ScriptExpr::Sub(
                Box::new(ScriptExpr::Const(30.0)),
                Box::new(ScriptExpr::Div(
                    Box::new(ScriptExpr::Const(12.0)),
                    Box::new(ScriptExpr::Const(3.0)),
                )),
            )),
        )),
        min:   0.0,
        max:   25.0,
    };

    assert_eq!(expr.eval(&ctx(&registry, &shadow, 0)).unwrap(), 25.0);
}

#[test]
fn script_predicate_evaluates_comparisons_and_boolean_logic() {
    let registry = registry_with_properties();
    let shadow = vec![0.0; registry.total_columns];
    let predicate = ScriptPredicate::And(vec![
        ScriptPredicate::Greater(ScriptExpr::Const(3.0), ScriptExpr::Const(2.0)),
        ScriptPredicate::Not(Box::new(ScriptPredicate::Less(
            ScriptExpr::Const(5.0),
            ScriptExpr::Const(4.0),
        ))),
        ScriptPredicate::Equalish(ScriptExpr::Const(1.0), ScriptExpr::Const(1.000_01)),
    ]);

    assert!(predicate.eval(&ctx(&registry, &shadow, 0)).unwrap());
}

#[test]
fn script_expr_gate_returns_one_or_zero() {
    let registry = registry_with_properties();
    let shadow = vec![0.0; registry.total_columns];
    let pass = ScriptExpr::Gate(Box::new(ScriptPredicate::True));
    let fail = ScriptExpr::Gate(Box::new(ScriptPredicate::False));

    assert_eq!(pass.eval(&ctx(&registry, &shadow, 0)).unwrap(), 1.0);
    assert_eq!(fail.eval(&ctx(&registry, &shadow, 0)).unwrap(), 0.0);
}

#[test]
fn script_expr_reports_unknown_property() {
    let registry = registry_with_properties();
    let shadow = vec![0.0; registry.total_columns];
    let expr = read("missing", "property", SubFieldRole::Amount);

    assert!(matches!(
        expr.eval(&ctx(&registry, &shadow, 0)),
        Err(ScriptEvalError::UnknownProperty { .. })
    ));
}

#[test]
fn script_expr_reports_unknown_role() {
    let registry = registry_with_properties();
    let shadow = vec![0.0; registry.total_columns];
    let expr = read("core", "loyalty", SubFieldRole::Named("not_present".into()));

    assert!(matches!(
        expr.eval(&ctx(&registry, &shadow, 0)),
        Err(ScriptEvalError::UnknownRole { .. })
    ));
}

#[test]
fn script_expr_reports_slot_out_of_bounds() {
    let registry = registry_with_properties();
    let shadow = vec![0.0; registry.total_columns];
    let expr = ScriptExpr::Read {
        scope:    ScopeRef::Slot(2),
        property: PropertyKey::new("core", "loyalty"),
        role:     SubFieldRole::Amount,
    };

    assert_eq!(
        expr.eval(&ctx(&registry, &shadow, 0)),
        Err(ScriptEvalError::SlotOutOfBounds { slot: 2, slots: 1 })
    );
}

#[test]
fn script_expr_reports_division_by_zero_and_invalid_clamp() {
    let registry = registry_with_properties();
    let shadow = vec![0.0; registry.total_columns];
    let div = ScriptExpr::Div(
        Box::new(ScriptExpr::Const(1.0)),
        Box::new(ScriptExpr::Const(0.0)),
    );
    let clamp = ScriptExpr::Clamp {
        value: Box::new(ScriptExpr::Const(1.0)),
        min:   2.0,
        max:   1.0,
    };

    assert_eq!(
        div.eval(&ctx(&registry, &shadow, 0)),
        Err(ScriptEvalError::DivisionByZero)
    );
    assert_eq!(
        clamp.eval(&ctx(&registry, &shadow, 0)),
        Err(ScriptEvalError::InvalidClamp { min: 2.0, max: 1.0 })
    );
}

#[test]
fn script_ir_round_trips_through_serde() {
    let original = ScriptExpr::Gate(Box::new(ScriptPredicate::Or(vec![
        ScriptPredicate::Greater(ScriptExpr::Const(2.0), ScriptExpr::Const(1.0)),
        ScriptPredicate::False,
    ])));

    let json = serde_json::to_string(&original).expect("serialize");
    let restored: ScriptExpr = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(restored, original);
}
