use simthing_core::{DimensionRegistry, OverlayId, SubFieldRole};
use simthing_spec::{
    compile_effect, compile_event, compile_property, compile_trigger, CompiledEffect,
    CompiledTrigger, CooldownSpec, EffectSpec, EventPriority, EventSpec, PropertyKey,
    PropertySpec, ScopeRef, ScriptExpr, ScriptPredicate, SpecError, TriggerDirection, TriggerSpec,
};

fn registry_with_loyalty() -> DimensionRegistry {
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
    registry
}

fn threshold_trigger() -> TriggerSpec {
    TriggerSpec::Threshold {
        target:    ScopeRef::Current,
        property:  PropertyKey::new("core", "loyalty"),
        role:      SubFieldRole::Amount,
        threshold: 0.25,
        direction: TriggerDirection::Falling,
    }
}

#[test]
fn compile_simple_threshold_trigger_resolves_property_role_and_col() {
    let registry = registry_with_loyalty();
    let (compiled, diagnostics) = compile_trigger(&threshold_trigger(), &registry).unwrap();

    assert!(diagnostics.diagnostics.is_empty());
    let loyalty = registry.id_of("core", "loyalty").unwrap();
    match compiled {
        CompiledTrigger::Threshold(trigger) => {
            assert_eq!(trigger.target, ScopeRef::Current);
            assert_eq!(trigger.property, loyalty);
            assert_eq!(trigger.role, SubFieldRole::Amount);
            assert_eq!(trigger.col, 0);
            assert_eq!(trigger.threshold, 0.25);
            assert_eq!(trigger.direction, TriggerDirection::Falling);
        }
        other => panic!("expected threshold trigger, got {other:?}"),
    }
}

#[test]
fn compile_predicate_trigger_preserves_script_predicate() {
    let registry = registry_with_loyalty();
    let predicate = ScriptPredicate::Greater(ScriptExpr::Const(2.0), ScriptExpr::Const(1.0));
    let spec = TriggerSpec::Predicate {
        predicate: predicate.clone(),
    };

    let (compiled, diagnostics) = compile_trigger(&spec, &registry).unwrap();

    assert!(diagnostics.diagnostics.is_empty());
    assert_eq!(compiled, CompiledTrigger::Predicate(predicate));
}

#[test]
fn compile_trigger_rejects_unknown_property() {
    let registry = registry_with_loyalty();
    let spec = TriggerSpec::Threshold {
        target:    ScopeRef::Current,
        property:  PropertyKey::new("missing", "thing"),
        role:      SubFieldRole::Amount,
        threshold: 1.0,
        direction: TriggerDirection::Rising,
    };

    assert!(matches!(
        compile_trigger(&spec, &registry),
        Err(SpecError::InvalidTriggerProperty { .. })
    ));
}

#[test]
fn compile_trigger_rejects_unknown_role() {
    let registry = registry_with_loyalty();
    let spec = TriggerSpec::Threshold {
        target:    ScopeRef::Current,
        property:  PropertyKey::new("core", "loyalty"),
        role:      SubFieldRole::Named("not_present".into()),
        threshold: 1.0,
        direction: TriggerDirection::Rising,
    };

    assert!(matches!(
        compile_trigger(&spec, &registry),
        Err(SpecError::InvalidTriggerRole { .. })
    ));
}

#[test]
fn compile_effect_builds_boundary_request_template() {
    let overlay_id = OverlayId::new();
    let spec = EffectSpec::ActivateOverlay {
        target: ScopeRef::Slot(3),
        overlay_id,
    };

    let (compiled, diagnostics) = compile_effect(&spec).unwrap();

    assert!(diagnostics.diagnostics.is_empty());
    assert_eq!(
        compiled,
        CompiledEffect::ActivateOverlay {
            target: ScopeRef::Slot(3),
            overlay_id,
        }
    );
}

#[test]
fn compile_event_combines_trigger_effects_and_metadata() {
    let registry = registry_with_loyalty();
    let overlay_id = OverlayId::new();
    let spec = EventSpec {
        id:       "low_loyalty_warning".into(),
        trigger:  threshold_trigger(),
        effects:  vec![
            EffectSpec::SuspendOverlay {
                target: ScopeRef::Current,
                overlay_id,
            },
            EffectSpec::Remove {
                target: ScopeRef::Slot(4),
            },
        ],
        cooldown: Some(CooldownSpec { ticks: 12 }),
        priority: EventPriority::High,
        install:  simthing_spec::InstallTargetSpec::SessionRoot,
    };

    let (compiled, diagnostics) = compile_event(&spec, &registry).unwrap();

    assert!(diagnostics.diagnostics.is_empty());
    assert_eq!(compiled.id.0, "low_loyalty_warning");
    assert!(matches!(compiled.trigger, CompiledTrigger::Threshold(_)));
    assert_eq!(compiled.effects.len(), 2);
    assert_eq!(compiled.cooldown, Some(CooldownSpec { ticks: 12 }));
    assert_eq!(compiled.priority, EventPriority::High);
}

#[test]
fn event_spec_round_trips_through_serde() {
    let original = EventSpec {
        id:       "predicate_event".into(),
        trigger:  TriggerSpec::Predicate {
            predicate: ScriptPredicate::True,
        },
        effects:  vec![EffectSpec::Remove {
            target: ScopeRef::Current,
        }],
        cooldown: None,
        priority: EventPriority::Normal,
        install:  simthing_spec::InstallTargetSpec::SessionRoot,
    };

    let json = serde_json::to_string(&original).expect("serialize");
    let restored: EventSpec = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(restored, original);
}
