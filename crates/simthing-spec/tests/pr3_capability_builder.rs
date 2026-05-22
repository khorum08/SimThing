use simthing_core::{
    DimensionRegistry, OverlayLifecycle, ReductionRule, SimThingKind, SubFieldRole, TransformOp,
};
use simthing_spec::{
    compile_property, ActivationMode, CapabilityCategorySpec, CapabilityEffectSpec,
    CapabilityEntryKey, CapabilityPrereqSpec, CapabilitySpec, CapabilityTreeBuilder,
    CapabilityTreeSpec, CategoryKey, PropertySpec, SpecError,
};

// ── Fixture helpers ───────────────────────────────────────────────────────────

fn registry_with_fleet_speed() -> DimensionRegistry {
    let mut registry = DimensionRegistry::new();
    // Tests' effects target `military::fleet_speed`. Register it up front.
    compile_property(
        &PropertySpec {
            id:           "military_fleet_speed".into(),
            namespace:    "military".into(),
            name:         "fleet_speed".into(),
            display_name: "Fleet Speed".into(),
            description:  String::new(),
            sub_fields:   vec![],  // standard layout (Amount, Velocity, Intensity)
        },
        &mut registry,
    )
    .expect("seed fleet_speed");
    registry
}

fn entry(
    id: &str,
    research_cost: f32,
    activation: ActivationMode,
    prereqs: Vec<CapabilityPrereqSpec>,
) -> CapabilitySpec {
    CapabilitySpec {
        id:            id.into(),
        display_name:  id.into(),
        description:   String::new(),
        flavor_text:   String::new(),
        research_cost,
        activation,
        research_rate: Default::default(),
        icon:          String::new(),
        thumbnail:     String::new(),
        card_image:    String::new(),
        unlock_video:  None,
        model_preview: None,
        prereqs,
        unlocks_ship_components: vec![],
        unlocks_buildings:       vec![],
        unlocks_units:           vec![],
        unlocks_weapons:         vec![],
        effects: vec![CapabilityEffectSpec {
            targets_property: "military::fleet_speed".into(),
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Multiply(1.10))],
            when_activated:   OverlayLifecycle::Permanent,
        }],
    }
}

fn category(ns: &str, name: &str, entries: Vec<CapabilitySpec>) -> CapabilityCategorySpec {
    CapabilityCategorySpec {
        property_namespace: ns.into(),
        property_name:      name.into(),
        display_name:       name.into(),
        tier:               0,
        max_active:         None,
        entries,
    }
}

fn tree_spec(categories: Vec<CapabilityCategorySpec>) -> CapabilityTreeSpec {
    CapabilityTreeSpec {
        tree_id:    "terran_tech".into(),
        tree_kind:  "tech_tree".into(),
        owner_kind: "Faction".into(),
        categories,
    }
}

// ── Acceptance tests ──────────────────────────────────────────────────────────

#[test]
fn capability_tree_builder_registers_properties_and_overlays() {
    let mut registry = registry_with_fleet_speed();
    let spec = tree_spec(vec![category(
        "tech",
        "propulsion",
        vec![entry("chemical_drive", 5000.0, ActivationMode::Threshold, vec![])],
    )]);

    let (out, diag) = CapabilityTreeBuilder::build(&spec, &mut registry).expect("build");
    assert!(diag.diagnostics.is_empty());

    // The category property is registered.
    let prop_id = registry.id_of("tech", "propulsion").expect("category prop registered");
    let prop    = registry.property(prop_id);
    assert_eq!(prop.layout.sub_fields.len(), 1);
    assert_eq!(
        prop.layout.sub_fields[0].role,
        SubFieldRole::Named("chemical_drive".into()),
    );

    // The tree SimThing is the correct kind and carries the category property
    // (seeded to 0.0 default value).
    assert_eq!(out.tree.kind, SimThingKind::Custom("tech_tree".into()));
    assert!(out.tree.properties.contains_key(&prop_id));
    let pv = out.tree.property(prop_id).unwrap();
    assert_eq!(pv.data, vec![0.0]);

    // One overlay attached per effect — suspended.
    assert_eq!(out.tree.overlays.len(), 1);
    let overlay = &out.tree.overlays[0];
    assert!(!overlay.is_active(), "overlay must compile as Suspended");
    match &overlay.lifecycle {
        OverlayLifecycle::Suspended { when_activated } => {
            assert!(matches!(when_activated.as_ref(), OverlayLifecycle::Permanent));
        }
        other => panic!("expected Suspended, got {other:?}"),
    }
}

#[test]
fn capability_tree_builder_enforces_reduction_max() {
    let mut registry = registry_with_fleet_speed();
    let spec = tree_spec(vec![category(
        "tech",
        "propulsion",
        vec![
            entry("chemical_drive", 5000.0, ActivationMode::Threshold, vec![]),
            entry("fusion_drive",   8000.0, ActivationMode::Threshold, vec![]),
        ],
    )]);

    CapabilityTreeBuilder::build(&spec, &mut registry).expect("build");

    let prop_id = registry.id_of("tech", "propulsion").unwrap();
    let layout  = &registry.property(prop_id).layout;

    // Every capability sub-field must resolve to ReductionRule::Max regardless
    // of `default_for_role` (which would return Mean for Named).
    for sf in &layout.sub_fields {
        assert_eq!(
            sf.resolved_reduction(),
            ReductionRule::Max,
            "sub-field {:?} did not get ReductionRule::Max", sf.role,
        );
        assert_eq!(sf.reduction_override, Some(ReductionRule::Max));
    }
}

#[test]
fn capability_tree_builder_validates_duplicate_entry_id() {
    let mut registry = registry_with_fleet_speed();
    let spec = tree_spec(vec![category(
        "tech",
        "propulsion",
        vec![
            entry("drive", 100.0, ActivationMode::Threshold, vec![]),
            entry("drive", 200.0, ActivationMode::Threshold, vec![]),  // duplicate id
        ],
    )]);

    let err = CapabilityTreeBuilder::build(&spec, &mut registry).expect_err("must reject");
    assert!(matches!(err, SpecError::DuplicateEntry(_, _)), "got {err:?}");
}

#[test]
fn capability_tree_builder_validates_threshold_requires_positive_cost() {
    let mut registry = registry_with_fleet_speed();
    let spec = tree_spec(vec![category(
        "tech",
        "propulsion",
        vec![entry("drive", 0.0, ActivationMode::Threshold, vec![])],
    )]);

    let err = CapabilityTreeBuilder::build(&spec, &mut registry).expect_err("must reject");
    assert!(matches!(err, SpecError::ThresholdRequiresPositiveCost(_)), "got {err:?}");
}

#[test]
fn capability_tree_builder_validates_on_prereq_met_authored_default_is_error() {
    let mut registry = registry_with_fleet_speed();
    let spec = tree_spec(vec![category(
        "tech",
        "propulsion",
        vec![entry("drive", 100.0, ActivationMode::OnPrereqMet, vec![])],
    )]);

    let err = CapabilityTreeBuilder::build(&spec, &mut registry).expect_err("must reject");
    assert!(matches!(err, SpecError::OnPrereqMetAuthoredDefault(_)), "got {err:?}");
}

#[test]
fn capability_tree_builder_player_selection_produces_no_unlock_registration() {
    let mut registry = registry_with_fleet_speed();
    let spec = tree_spec(vec![category(
        "tech",
        "propulsion",
        vec![
            entry("chemical_drive", 5000.0, ActivationMode::Threshold,       vec![]),
            entry("philosophy",        0.0, ActivationMode::PlayerSelection, vec![]),
        ],
    )]);

    let (out, _) = CapabilityTreeBuilder::build(&spec, &mut registry).expect("build");

    // PlayerSelection contributes no Pass 7 registration.
    assert_eq!(out.unlock_registrations.len(), 1);
    let reg = &out.unlock_registrations[0];
    assert_eq!(reg.threshold, 5000.0);
    assert_eq!(reg.sub_field, SubFieldRole::Named("chemical_drive".into()));

    // PlayerSelection also is absent from by_threshold; only Threshold entries
    // map back from a fired event.
    assert_eq!(out.definition.by_threshold.len(), 1);
}

#[test]
fn capability_tree_prereq_resolution_same_category() {
    let mut registry = registry_with_fleet_speed();
    let spec = tree_spec(vec![category(
        "tech",
        "propulsion",
        vec![
            entry("chemical_drive", 5000.0, ActivationMode::Threshold, vec![]),
            entry(
                "fusion_drive",
                8000.0,
                ActivationMode::Threshold,
                vec![CapabilityPrereqSpec {
                    category: "tech::propulsion".into(),
                    entry_id: "chemical_drive".into(),
                }],
            ),
        ],
    )]);

    let (out, _) = CapabilityTreeBuilder::build(&spec, &mut registry).expect("build");

    let key = CapabilityEntryKey::new(CategoryKey::new("tech", "propulsion"), "fusion_drive");
    let def = out.definition.entries.get(&key).expect("entry");
    assert_eq!(def.prereqs.len(), 1);
    let p = &def.prereqs[0];

    // Resolves to the chemical_drive column in the same property at min 5000.0.
    let propulsion_id = registry.id_of("tech", "propulsion").unwrap();
    assert_eq!(p.property_id, propulsion_id);
    assert_eq!(p.role, SubFieldRole::Named("chemical_drive".into()));
    // First sub-field within propulsion's column range (offset 0 in its layout).
    let range = registry.column_range(propulsion_id);
    assert_eq!(p.col, range.start);
    assert_eq!(p.min_value, 5000.0);
}

#[test]
fn capability_tree_prereq_resolution_cross_category() {
    let mut registry = registry_with_fleet_speed();
    let spec = tree_spec(vec![
        category(
            "tech",
            "physics",
            vec![entry("relativity", 3000.0, ActivationMode::Threshold, vec![])],
        ),
        category(
            "tech",
            "propulsion",
            vec![entry(
                "warp_drive",
                12000.0,
                ActivationMode::Threshold,
                vec![CapabilityPrereqSpec {
                    category: "tech::physics".into(),
                    entry_id: "relativity".into(),
                }],
            )],
        ),
    ]);

    let (out, _) = CapabilityTreeBuilder::build(&spec, &mut registry).expect("build");

    let warp = CapabilityEntryKey::new(CategoryKey::new("tech", "propulsion"), "warp_drive");
    let def  = out.definition.entries.get(&warp).expect("entry");
    assert_eq!(def.prereqs.len(), 1);
    let p = &def.prereqs[0];

    // Cross-category prereq points at the physics property, not propulsion.
    let physics_prop = registry.id_of("tech", "physics").unwrap();
    assert_eq!(p.property_id, physics_prop);
    assert_eq!(p.role, SubFieldRole::Named("relativity".into()));
    assert_eq!(p.min_value, 3000.0);

    // Column resolution went through col_for_role on the physics layout —
    // physics is registered first, so it starts at column 0.
    let phys_range = registry.column_range(physics_prop);
    assert_eq!(p.col, phys_range.start);
}

#[test]
fn capability_tree_builder_records_overlay_ids_for_each_effect() {
    let mut registry = registry_with_fleet_speed();

    // Two effects on one entry — must produce two distinct OverlayIds.
    let mut e = entry("multi_effect", 100.0, ActivationMode::Threshold, vec![]);
    e.effects.push(CapabilityEffectSpec {
        targets_property: "military::fleet_speed".into(),
        sub_field_deltas: vec![(SubFieldRole::Velocity, TransformOp::Add(0.01))],
        when_activated:   OverlayLifecycle::Permanent,
    });

    let spec = tree_spec(vec![category("tech", "propulsion", vec![e])]);
    let (out, _) = CapabilityTreeBuilder::build(&spec, &mut registry).expect("build");

    let key = CapabilityEntryKey::new(CategoryKey::new("tech", "propulsion"), "multi_effect");
    let def = out.definition.entries.get(&key).expect("entry");
    assert_eq!(def.overlay_ids.len(), 2);
    assert_eq!(def.effect_keys.len(), 2);
    assert_ne!(def.overlay_ids[0], def.overlay_ids[1]);

    // Both overlays landed on the tree SimThing.
    assert_eq!(out.tree.overlays.len(), 2);
}

#[test]
fn capability_tree_definition_lookup_by_overlay_id_returns_entry() {
    let mut registry = registry_with_fleet_speed();
    let spec = tree_spec(vec![category(
        "tech",
        "propulsion",
        vec![entry("chemical_drive", 5000.0, ActivationMode::Threshold, vec![])],
    )]);

    let (out, _) = CapabilityTreeBuilder::build(&spec, &mut registry).expect("build");

    let key = CapabilityEntryKey::new(CategoryKey::new("tech", "propulsion"), "chemical_drive");
    let def = out.definition.entries.get(&key).expect("entry");
    let overlay_id = def.overlay_ids[0];

    // by_overlay lookup round-trips to the same entry key.
    assert_eq!(out.definition.by_overlay.get(&overlay_id), Some(&key));
}

#[test]
fn capability_tree_logical_effect_keys_are_stable_across_builds() {
    // OverlayId is a global atomic — non-deterministic across builds.
    // CapabilityEffectKey is logical (`entry / effect_index`) — stable.
    let spec = tree_spec(vec![category(
        "tech",
        "propulsion",
        vec![{
            let mut e = entry("drive", 100.0, ActivationMode::Threshold, vec![]);
            e.effects.push(CapabilityEffectSpec {
                targets_property: "military::fleet_speed".into(),
                sub_field_deltas: vec![(SubFieldRole::Velocity, TransformOp::Add(0.01))],
                when_activated:   OverlayLifecycle::Permanent,
            });
            e
        }],
    )]);

    let mut reg_a = registry_with_fleet_speed();
    let mut reg_b = registry_with_fleet_speed();

    let (out_a, _) = CapabilityTreeBuilder::build(&spec, &mut reg_a).unwrap();
    let (out_b, _) = CapabilityTreeBuilder::build(&spec, &mut reg_b).unwrap();

    let key = CapabilityEntryKey::new(CategoryKey::new("tech", "propulsion"), "drive");
    let def_a = out_a.definition.entries.get(&key).unwrap();
    let def_b = out_b.definition.entries.get(&key).unwrap();

    // Effect keys are bit-identical across builds.
    assert_eq!(def_a.effect_keys, def_b.effect_keys);
    assert_eq!(def_a.effect_keys[0].effect_index, 0);
    assert_eq!(def_a.effect_keys[1].effect_index, 1);

    // OverlayIds, by contrast, differ — the atomic moved forward between builds.
    // (We don't assert _which_ direction, only that we don't rely on equality.)
    assert_eq!(def_a.overlay_ids.len(), def_b.overlay_ids.len());
}

// ── Supplementary tests (beyond the 11 acceptance criteria) ───────────────────

#[test]
fn capability_tree_builder_rejects_self_referential_prereq() {
    let mut registry = registry_with_fleet_speed();
    let spec = tree_spec(vec![category(
        "tech",
        "propulsion",
        vec![entry(
            "drive",
            100.0,
            ActivationMode::Threshold,
            vec![CapabilityPrereqSpec {
                category: "tech::propulsion".into(),
                entry_id: "drive".into(),
            }],
        )],
    )]);

    let err = CapabilityTreeBuilder::build(&spec, &mut registry).expect_err("must reject");
    assert!(matches!(err, SpecError::SelfReferentialPrereq(_)), "got {err:?}");
}

#[test]
fn capability_tree_builder_rejects_unknown_prereq_category() {
    let mut registry = registry_with_fleet_speed();
    let spec = tree_spec(vec![category(
        "tech",
        "propulsion",
        vec![entry(
            "drive",
            100.0,
            ActivationMode::Threshold,
            vec![CapabilityPrereqSpec {
                category: "tech::bogus".into(),
                entry_id: "anything".into(),
            }],
        )],
    )]);

    let err = CapabilityTreeBuilder::build(&spec, &mut registry).expect_err("must reject");
    assert!(matches!(err, SpecError::UnknownPrereqCategory { .. }), "got {err:?}");
}

#[test]
fn capability_tree_builder_rejects_unknown_prereq_entry() {
    let mut registry = registry_with_fleet_speed();
    let spec = tree_spec(vec![category(
        "tech",
        "propulsion",
        vec![entry(
            "drive",
            100.0,
            ActivationMode::Threshold,
            vec![CapabilityPrereqSpec {
                category: "tech::propulsion".into(),
                entry_id: "nonexistent".into(),
            }],
        )],
    )]);

    let err = CapabilityTreeBuilder::build(&spec, &mut registry).expect_err("must reject");
    assert!(matches!(err, SpecError::UnknownPrereqEntry { .. }), "got {err:?}");
}

#[test]
fn capability_tree_builder_rejects_unsupported_max_active() {
    let mut registry = registry_with_fleet_speed();
    let spec = tree_spec(vec![CapabilityCategorySpec {
        property_namespace: "tech".into(),
        property_name:      "propulsion".into(),
        display_name:       "Propulsion".into(),
        tier:               0,
        max_active:         Some(3),    // v0 supports only 1
        entries:            vec![entry("drive", 100.0, ActivationMode::PlayerSelection, vec![])],
    }]);

    let err = CapabilityTreeBuilder::build(&spec, &mut registry).expect_err("must reject");
    assert!(matches!(err, SpecError::UnsupportedMaxActive { .. }), "got {err:?}");
}

#[test]
fn capability_tree_builder_rejects_invalid_effect_target_property() {
    let mut registry = registry_with_fleet_speed();
    let mut e = entry("drive", 100.0, ActivationMode::Threshold, vec![]);
    e.effects[0].targets_property = "nonexistent::property".into();

    let spec = tree_spec(vec![category("tech", "propulsion", vec![e])]);

    let err = CapabilityTreeBuilder::build(&spec, &mut registry).expect_err("must reject");
    assert!(matches!(err, SpecError::InvalidEffectTarget { .. }), "got {err:?}");
}
