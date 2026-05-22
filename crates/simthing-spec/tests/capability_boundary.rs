use simthing_core::{DimensionRegistry, OverlayLifecycle, SimProperty, SimThingId, SubFieldRole};
use simthing_feeder::BoundaryRequest;
use simthing_spec::{
    ActivationMode, CapabilityBoundaryContext, CapabilityEntryKey, CapabilityTreeBoundaryHandler,
    CapabilityTreeBuilder, CapabilityTreeInstance, CapabilityTreeState, CapabilityUnlockEvent,
    CategoryKey,
};
use std::collections::HashMap;

#[test]
fn threshold_unlock_activates_overlays_when_prereqs_met() {
    let text = include_str!("fixtures/minimal_tech_tree.ron");
    let spec = simthing_spec::deserialize_capability_tree_ron(text).unwrap();
    let mut registry = DimensionRegistry::new();
    registry.register(SimProperty::simple("military", "fleet_speed", 0));

    let (built, _) = CapabilityTreeBuilder::build(&spec, &mut registry).unwrap();
    let owner_id = SimThingId::new();
    let mut tree = built.tree;
    simthing_spec::set_overlay_affects(&mut tree, owner_id);

    let definition_id = built.definition.id;
    let tree_thing_id = tree.id;
    let entry = CapabilityEntryKey::new(CategoryKey::new("tech", "propulsion"), "chemical_drive");
    let progress_col = built
        .definition
        .entry(&entry)
        .unwrap()
        .progress_col;

    let mut definitions = HashMap::new();
    definitions.insert(definition_id, built.definition);

    let mut instances = HashMap::new();
    instances.insert(
        owner_id,
        CapabilityTreeInstance {
            owner_id,
            definition_id,
            tree_thing_id,
            tree_slot: 1,
        },
    );

    let mut states = HashMap::new();
    states.insert(
        owner_id,
        CapabilityTreeState::new(owner_id, definition_id),
    );

    let n_dims = registry.total_columns;
    let mut shadow = vec![0.0f32; n_dims * 2];
    let progress_idx = 1usize * n_dims + progress_col;
    shadow[progress_idx] = 5000.0;

    let mut requests = Vec::new();
    let mut diagnostics = Vec::new();
    let mut ctx = CapabilityBoundaryContext {
        n_dims,
        shadow: &mut shadow,
        instances: &instances,
        states: &mut states,
        requests: &mut requests,
        diagnostics: &mut diagnostics,
    };

    let handler = CapabilityTreeBoundaryHandler {
        registry: &registry,
        definitions: &definitions,
    };

    let outcome = handler
        .handle_threshold_events(
            &[CapabilityUnlockEvent {
                tree_thing_id,
                tree_slot: 1,
                entry: entry.clone(),
                value: 5000.0,
            }],
            &mut ctx,
        )
        .unwrap();

    assert_eq!(outcome.activations, 1);
    assert_eq!(requests.len(), 1);
    assert!(matches!(
        requests[0],
        BoundaryRequest::ActivateOverlay { .. }
    ));
}

#[test]
fn player_selection_respects_max_active_one() {
    use simthing_spec::{CapabilityCategorySpec, CapabilityEffectSpec, CapabilitySpec, CapabilityTreeSpec};
    use simthing_core::TransformOp;

    let spec = CapabilityTreeSpec {
        tree_id:    "ideas".into(),
        tree_kind:  "national_ideas".into(),
        owner_kind: "Faction".into(),
        categories: vec![CapabilityCategorySpec {
            property_namespace: "ideas".into(),
            property_name:      "tier1".into(),
            display_name:       "Tier 1".into(),
            tier:               1,
            max_active:         Some(1),
            entries: vec![
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
                    effects: vec![CapabilityEffectSpec {
                        targets_property: "military::fleet_speed".into(),
                        sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Multiply(1.1))],
                        when_activated: OverlayLifecycle::Permanent,
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
                    effects: vec![CapabilityEffectSpec {
                        targets_property: "military::fleet_speed".into(),
                        sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Multiply(1.2))],
                        when_activated: OverlayLifecycle::Permanent,
                    }],
                },
            ],
        }],
    };

    let mut registry = DimensionRegistry::new();
    registry.register(SimProperty::simple("military", "fleet_speed", 0));
    let (built, _) = CapabilityTreeBuilder::build(&spec, &mut registry).unwrap();
    let owner_id = SimThingId::new();
    let definition_id = built.definition.id;
    let tree_thing_id = built.tree.id;

    let entry_a = CapabilityEntryKey::new(CategoryKey::new("ideas", "tier1"), "a");
    let entry_b = CapabilityEntryKey::new(CategoryKey::new("ideas", "tier1"), "b");

    let mut definitions = HashMap::new();
    definitions.insert(definition_id, built.definition);

    let mut instances = HashMap::new();
    instances.insert(
        owner_id,
        CapabilityTreeInstance {
            owner_id,
            definition_id,
            tree_thing_id,
            tree_slot: 1,
        },
    );

    let mut states = HashMap::new();
    states.insert(owner_id, CapabilityTreeState::new(owner_id, definition_id));

    let n_dims = registry.total_columns;
    let mut shadow = vec![0.0f32; n_dims * 2];
    let mut requests = Vec::new();
    let mut diagnostics = Vec::new();

    let handler = CapabilityTreeBoundaryHandler {
        registry: &registry,
        definitions: &definitions,
    };

    {
        let mut ctx = CapabilityBoundaryContext {
            n_dims,
            shadow: &mut shadow,
            instances: &instances,
            states: &mut states,
            requests: &mut requests,
            diagnostics: &mut diagnostics,
        };
        handler
            .handle_player_selection(owner_id, entry_a.clone(), &mut ctx)
            .unwrap();
    }

    requests.clear();
    {
        let mut ctx = CapabilityBoundaryContext {
            n_dims,
            shadow: &mut shadow,
            instances: &instances,
            states: &mut states,
            requests: &mut requests,
            diagnostics: &mut diagnostics,
        };
        handler
            .handle_player_selection(owner_id, entry_b.clone(), &mut ctx)
            .unwrap();
    }

    assert_eq!(requests.len(), 2);
    assert!(matches!(requests[0], BoundaryRequest::SuspendOverlay { .. }));
    assert!(matches!(requests[1], BoundaryRequest::ActivateOverlay { .. }));
}
