use simthing_core::{
    ClampBehavior, DimensionRegistry, DissolveCondition, OverlayKind, OverlayLifecycle,
    OverlaySource, SubFieldRole, SubFieldSpec, TransformOp,
};
use simthing_spec::{
    compile_overlay, compile_property, CompileContext, OverlaySpec, PropertySpec, SpecError,
};

fn empty_property(namespace: &str, name: &str) -> PropertySpec {
    PropertySpec {
        id: format!("{namespace}_{name}"),
        namespace: namespace.into(),
        name: name.into(),
        display_name: String::new(),
        description: String::new(),
        sub_fields: vec![],
    }
}

fn ethics_property() -> PropertySpec {
    PropertySpec {
        id: "ethics".into(),
        namespace: "philosophy".into(),
        name: "ethics".into(),
        display_name: "Ethics".into(),
        description: "Designer-named layout with a governing sub-field.".into(),
        sub_fields: vec![
            SubFieldSpec {
                role: SubFieldRole::Named("axis_position".into()),
                width: 1,
                clamp: ClampBehavior::Bounded {
                    min: -10.0,
                    max: 10.0,
                },
                velocity_max: None,
                default: 0.0,
                display_name: "Axis".into(),
                display_range: None,
                governed_by: Some(SubFieldRole::Named("axis_drift".into())),
                reduction_override: None,
                soft_aggregate_guard: None,
                accumulator_spec: None,
            },
            SubFieldSpec {
                role: SubFieldRole::Named("axis_drift".into()),
                width: 1,
                clamp: ClampBehavior::Bounded {
                    min: -1.0,
                    max: 1.0,
                },
                velocity_max: None,
                default: 0.0,
                display_name: "Drift".into(),
                display_range: None,
                governed_by: None,
                reduction_override: None,
                soft_aggregate_guard: None,
                accumulator_spec: None,
            },
        ],
    }
}

// ── compile_property ──────────────────────────────────────────────────────────

#[test]
fn compile_property_registers_simpropertyid() {
    let mut registry = DimensionRegistry::new();
    let spec = empty_property("military", "fleet_speed");

    let (id, diag) = compile_property(&spec, &mut registry).expect("compile");
    assert!(diag.diagnostics.is_empty());

    // Round-trip through the registry: the returned id must look up to the
    // same property by canonical key.
    assert_eq!(registry.id_of("military", "fleet_speed"), Some(id));

    // Default layout is `PropertyLayout::standard(0)` → 3 columns
    // (Amount, Velocity, Intensity).
    let range = registry.column_range(id);
    assert_eq!(range.start, 0);
    assert_eq!(range.stride, 3);
}

#[test]
fn compile_property_uses_authored_sub_fields_when_provided() {
    let mut registry = DimensionRegistry::new();
    let (id, _) = compile_property(&ethics_property(), &mut registry).expect("compile");

    let prop = registry.property(id);
    assert_eq!(prop.layout.sub_fields.len(), 2);
    assert_eq!(prop.layout.stride(), 2);
    assert_eq!(
        prop.layout
            .offset_of(&SubFieldRole::Named("axis_position".into()))
            .map(|offset| offset.lane()),
        Some(0),
    );
    assert_eq!(
        prop.layout
            .offset_of(&SubFieldRole::Named("axis_drift".into()))
            .map(|offset| offset.lane()),
        Some(1),
    );
}


// ── compile_overlay ───────────────────────────────────────────────────────────

fn fleet_speed_overlay() -> OverlaySpec {
    OverlaySpec {
        id: "trade_boost".into(),
        display_name: "Trade Boost".into(),
        targets_property: "military::fleet_speed".into(),
        sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Multiply(1.1))],
        lifecycle: OverlayLifecycle::Permanent,
        kind: OverlayKind::Policy,
        source: OverlaySource::Player,
        install: simthing_spec::InstallTargetSpec::SessionRoot,
    }
}

#[test]
fn compile_overlay_resolves_subfield_roles_to_columns() {
    let mut registry = DimensionRegistry::new();
    let (prop_id, _) = compile_property(&empty_property("military", "fleet_speed"), &mut registry)
        .expect("compile property");

    let (overlay, diag) = compile_overlay(&fleet_speed_overlay(), &registry).expect("compile");
    assert!(diag.diagnostics.is_empty());

    // The overlay carries the resolved property id and the role-keyed deltas.
    assert_eq!(overlay.transform.property_id, prop_id);
    assert_eq!(overlay.transform.sub_field_deltas.len(), 1);

    // Sub-field roles resolve to columns via the registry's column_range +
    // layout.offset_of pipeline — the same one Pass 3 uses at runtime.
    let range = registry.column_range(prop_id);
    let layout = &registry.property(prop_id).layout;
    let amount_col = range
        .col_for_role(&overlay.transform.sub_field_deltas[0].0, layout)
        .expect("amount role resolves");
    assert_eq!(amount_col, 0);
}

#[test]
fn compile_overlay_unknown_property_is_hard_error() {
    let registry = DimensionRegistry::new();
    let err = compile_overlay(&fleet_speed_overlay(), &registry).expect_err("must fail");
    match err {
        SpecError::UnknownProperty {
            overlay,
            namespace,
            name,
        } => {
            assert_eq!(overlay, "trade_boost");
            assert_eq!(namespace, "military");
            assert_eq!(name, "fleet_speed");
        }
        other => panic!("expected UnknownProperty, got {other:?}"),
    }
}

#[test]
fn compile_overlay_suspended_lifecycle_round_trips() {
    let mut registry = DimensionRegistry::new();
    compile_property(&empty_property("military", "fleet_speed"), &mut registry).unwrap();

    let mut spec = fleet_speed_overlay();
    spec.lifecycle = OverlayLifecycle::Suspended {
        when_activated: Box::new(OverlayLifecycle::Transient {
            dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 100 }],
        }),
    };

    let (overlay, _) = compile_overlay(&spec, &registry).expect("compile");

    // Sanity: a suspended overlay is GPU-inert by V6 invariant.
    assert!(!overlay.is_active());

    // The suspended wrapper preserves the inner lifecycle byte-for-byte.
    match &overlay.lifecycle {
        OverlayLifecycle::Suspended { when_activated } => match when_activated.as_ref() {
            OverlayLifecycle::Transient {
                dissolution_conditions,
            } => {
                assert_eq!(dissolution_conditions.len(), 1);
                match &dissolution_conditions[0] {
                    DissolveCondition::AfterTicks { remaining } => assert_eq!(*remaining, 100),
                    other => panic!("expected AfterTicks, got {other:?}"),
                }
            }
            other => panic!("expected Transient inner, got {other:?}"),
        },
        other => panic!("expected Suspended lifecycle, got {other:?}"),
    }
}

// ── CompileContext ────────────────────────────────────────────────────────────

#[test]
fn compile_context_threads_registry_across_multiple_properties() {
    let mut registry = DimensionRegistry::new();
    let mut ctx = CompileContext::new(&mut registry);

    // The borrow of `registry` lives in `ctx` for the duration of this block.
    let specs = [
        empty_property("core", "loyalty"),
        empty_property("core", "food_security"),
        ethics_property(),
    ];

    let mut ids = Vec::new();
    for spec in &specs {
        let (id, _) = compile_property(spec, ctx.registry_mut()).expect("compile");
        ids.push(id);
    }

    // All three got distinct ids, each looked up by canonical key.
    assert_eq!(ids.len(), 3);
    assert_eq!(ctx.registry().id_of("core", "loyalty"), Some(ids[0]));
    assert_eq!(ctx.registry().id_of("core", "food_security"), Some(ids[1]));
    assert_eq!(ctx.registry().id_of("philosophy", "ethics"), Some(ids[2]));

    // Columns are appended in registration order — invariant from registry.rs.
    let r0 = ctx.registry().column_range(ids[0]);
    let r1 = ctx.registry().column_range(ids[1]);
    let r2 = ctx.registry().column_range(ids[2]);
    assert_eq!(r0.start, 0);
    assert_eq!(r1.start, r0.start + r0.stride);
    assert_eq!(r2.start, r1.start + r1.stride);
}

#[test]
fn compile_context_overlay_after_property_registration() {
    let mut registry = DimensionRegistry::new();
    let mut ctx = CompileContext::new(&mut registry);

    compile_property(
        &empty_property("military", "fleet_speed"),
        ctx.registry_mut(),
    )
    .unwrap();

    // After mutation, the context can be used immutably for overlay compilation
    // in the same lexical scope — the borrow checker accepts the alternation.
    let (overlay, _) = compile_overlay(&fleet_speed_overlay(), ctx.registry()).expect("compile");
    // The overlay's resolved property id matches what the registry holds.
    let prop_id = ctx.registry().id_of("military", "fleet_speed").unwrap();
    assert_eq!(overlay.transform.property_id, prop_id);
}
