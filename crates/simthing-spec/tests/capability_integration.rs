use simthing_core::{DimensionRegistry, SimProperty, SubFieldRole, TransformOp};
use simthing_spec::{
    deserialize_capability_tree_ron, ActivationMode, CapabilityCategorySpec, CapabilityEntryKey,
    CapabilityTreeBuilder, CapabilityTreeState, CategoryKey,
};

#[test]
fn minimal_tech_tree_ron_deserializes() {
    let text = include_str!("fixtures/minimal_tech_tree.ron");
    let spec = deserialize_capability_tree_ron(text).expect("parse ron");
    assert_eq!(spec.tree_id, "minimal_tech");
    assert_eq!(spec.categories.len(), 1);
    assert_eq!(spec.categories[0].entries[0].id, "chemical_drive");
}

#[test]
fn end_to_end_build_from_fixture() {
    let text = include_str!("fixtures/minimal_tech_tree.ron");
    let spec = deserialize_capability_tree_ron(text).unwrap();
    let mut registry = DimensionRegistry::new();
    registry.register(SimProperty::simple("military", "fleet_speed", 0));

    let (output, diagnostics) = CapabilityTreeBuilder::build(&spec, &mut registry).unwrap();
    assert!(diagnostics.warnings.is_empty());
    assert_eq!(output.unlock_registrations.len(), 1);
    assert_eq!(output.tree.overlays.len(), 1);

    let category = CategoryKey::new("tech", "propulsion");
    let entry = CapabilityEntryKey::new(category, "chemical_drive");
    assert!(output.definition.entry(&entry).is_some());
}

#[test]
fn national_ideas_max_active_one() {
    use simthing_spec::CapabilitySpec;
    let spec = simthing_spec::CapabilityTreeSpec {
        tree_id:    "ideas".into(),
        tree_kind:  "national_ideas".into(),
        owner_kind: "Faction".into(),
        categories: vec![CapabilityCategorySpec {
            property_namespace: "ideas".into(),
            property_name:      "tier1".into(),
            display_name:       "Tier 1".into(),
            tier:               1,
            max_active:         Some(1),
            entries:            vec![
                CapabilitySpec {
                    id: "a".into(),
                    display_name: "A".into(),
                    description: String::new(),
                    flavor_text: String::new(),
                    research_cost: 0.0,
                    activation: ActivationMode::PlayerSelection,
                    research_rate: Default::default(),
                    icon: String::new(),
                    thumbnail: String::new(),
                    card_image: String::new(),
                    unlock_video: None,
                    model_preview: None,
                    prereqs: vec![],
                    unlocks_ship_components: vec![],
                    unlocks_buildings: vec![],
                    unlocks_units: vec![],
                    unlocks_weapons: vec![],
                    effects: vec![simthing_spec::CapabilityEffectSpec {
                        targets_property: "military::fleet_speed".into(),
                        sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Multiply(1.1))],
                        when_activated: simthing_core::OverlayLifecycle::Permanent,
                    }],
                },
                CapabilitySpec {
                    id: "b".into(),
                    display_name: "B".into(),
                    description: String::new(),
                    flavor_text: String::new(),
                    research_cost: 0.0,
                    activation: ActivationMode::PlayerSelection,
                    research_rate: Default::default(),
                    icon: String::new(),
                    thumbnail: String::new(),
                    card_image: String::new(),
                    unlock_video: None,
                    model_preview: None,
                    prereqs: vec![],
                    unlocks_ship_components: vec![],
                    unlocks_buildings: vec![],
                    unlocks_units: vec![],
                    unlocks_weapons: vec![],
                    effects: vec![simthing_spec::CapabilityEffectSpec {
                        targets_property: "military::fleet_speed".into(),
                        sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Multiply(1.2))],
                        when_activated: simthing_core::OverlayLifecycle::Permanent,
                    }],
                },
            ],
        }],
    };

    let mut registry = DimensionRegistry::new();
    registry.register(SimProperty::simple("military", "fleet_speed", 0));
    let (output, _) = CapabilityTreeBuilder::build(&spec, &mut registry).unwrap();
    assert!(output.unlock_registrations.is_empty());
    let _state = CapabilityTreeState::new(simthing_core::SimThingId::new(), output.definition.id);
}
