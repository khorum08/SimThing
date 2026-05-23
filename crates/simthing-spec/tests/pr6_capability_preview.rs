use simthing_core::{
    DimensionRegistry, OverlayLifecycle, SimThing, SimThingId, SubFieldRole, TransformOp,
};
use simthing_feeder::BoundaryRequest;
use simthing_gpu::SlotAllocator;
use simthing_sim::apply_structural_mutations;
use simthing_spec::{
    compile_property, preview_capability_effect, ActivationMode, CapabilityBoundaryContext,
    CapabilityCategorySpec, CapabilityEffectSpec, CapabilityEntryKey, CapabilityPrereqSpec,
    CapabilityPreviewInput, CapabilitySpec, CapabilityTreeBuilder, CapabilityTreeDefinition,
    CapabilityTreeInstance, CapabilityTreeNotification, CapabilityTreeState, CategoryKey,
    MaxActivePolicy, PropertySpec, ReplacementPolicy,
};
use std::collections::HashMap;

fn registry_with_fleet_speed() -> DimensionRegistry {
    let mut registry = DimensionRegistry::new();
    compile_property(
        &PropertySpec {
            id: "military_fleet_speed".into(),
            namespace: "military".into(),
            name: "fleet_speed".into(),
            display_name: "Fleet Speed".into(),
            description: String::new(),
            sub_fields: vec![],
        },
        &mut registry,
    )
    .expect("seed fleet_speed");
    registry
}

fn effect(role: SubFieldRole, op: TransformOp) -> CapabilityEffectSpec {
    CapabilityEffectSpec {
        targets_property: "military::fleet_speed".into(),
        sub_field_deltas: vec![(role, op)],
        when_activated: OverlayLifecycle::Permanent,
    }
}

fn entry_with_effects(
    id: &str,
    activation: ActivationMode,
    effects: Vec<CapabilityEffectSpec>,
) -> CapabilitySpec {
    CapabilitySpec {
        id: id.into(),
        display_name: id.into(),
        description: String::new(),
        flavor_text: String::new(),
        research_cost: if activation == ActivationMode::Threshold {
            100.0
        } else {
            0.0
        },
        activation,
        icon: String::new(),
        thumbnail: String::new(),
        card_image: String::new(),
        unlock_video: None,
        model_preview: None,
        prereqs: Vec::<CapabilityPrereqSpec>::new(),
        unlocks_ship_components: vec![],
        unlocks_buildings: vec![],
        unlocks_units: vec![],
        unlocks_weapons: vec![],
        effects,
    }
}

fn category(
    ns: &str,
    name: &str,
    max_active: Option<MaxActivePolicy>,
    entries: Vec<CapabilitySpec>,
) -> CapabilityCategorySpec {
    CapabilityCategorySpec {
        property_namespace: ns.into(),
        property_name: name.into(),
        display_name: name.into(),
        tier: 0,
        max_active,
        entries,
    }
}

fn tree_spec(categories: Vec<CapabilityCategorySpec>) -> simthing_spec::CapabilityTreeSpec {
    simthing_spec::CapabilityTreeSpec {
        tree_id: "terran_ideas".into(),
        tree_kind: "national_ideas".into(),
        owner_kind: "Faction".into(),
        categories,
    }
}

struct Fixture {
    registry: DimensionRegistry,
    definition: CapabilityTreeDefinition,
    tree: SimThing,
    owner_id: SimThingId,
    n_dims: usize,
}

impl Fixture {
    fn new(spec: simthing_spec::CapabilityTreeSpec) -> Self {
        let mut registry = registry_with_fleet_speed();
        let (out, _) = CapabilityTreeBuilder::build(&spec, &mut registry).expect("build");
        let n_dims = registry.total_columns;
        Self {
            registry,
            definition: out.definition,
            tree: out.tree,
            owner_id: SimThingId::new(),
            n_dims,
        }
    }

    fn key(&self, ns: &str, name: &str, entry_id: &str) -> CapabilityEntryKey {
        CapabilityEntryKey::new(CategoryKey::new(ns, name), entry_id)
    }

    fn state(&self) -> CapabilityTreeState {
        CapabilityTreeState {
            owner_id: self.owner_id,
            definition_id: self.definition.id,
            activation_mode_by_entry: HashMap::new(),
            active_by_category: HashMap::new(),
        }
    }

    fn preview(
        &self,
        entry: CapabilityEntryKey,
        shadow: &[f32],
    ) -> simthing_spec::CapabilityPreviewReport {
        preview_capability_effect(CapabilityPreviewInput {
            definition: &self.definition,
            state: &self.state(),
            registry: &self.registry,
            shadow,
            n_dims: self.n_dims,
            tree_slot: 0,
            entry,
        })
        .expect("preview")
    }
}

fn preview_fixture() -> Fixture {
    Fixture::new(tree_spec(vec![category(
        "ideas",
        "national",
        None,
        vec![entry_with_effects(
            "naval_ethos",
            ActivationMode::PlayerSelection,
            vec![effect(SubFieldRole::Amount, TransformOp::Multiply(1.25))],
        )],
    )]))
}

#[test]
fn capability_tree_impact_preview_returns_delta() {
    let fixture = preview_fixture();
    let key = fixture.key("ideas", "national", "naval_ethos");
    let fleet = fixture.registry.id_of("military", "fleet_speed").unwrap();
    let amount_col = fixture
        .registry
        .column_range(fleet)
        .col_for_role(
            &SubFieldRole::Amount,
            &fixture.registry.property(fleet).layout,
        )
        .unwrap();
    let mut shadow = vec![0.0; fixture.n_dims];
    shadow[amount_col] = 20.0;

    let report = fixture.preview(key, &shadow);

    assert_eq!(report.combined.len(), 1);
    assert_eq!(report.combined[0].current, 20.0);
    assert_eq!(report.combined[0].after, 25.0);
}

#[test]
fn capability_tree_impact_preview_per_overlay_breakdown() {
    let fixture = preview_fixture();
    let key = fixture.key("ideas", "national", "naval_ethos");
    let shadow = vec![4.0; fixture.n_dims];

    let report = fixture.preview(key.clone(), &shadow);

    let entry = &fixture.definition.entries[&key];
    assert_eq!(report.per_overlay.len(), 1);
    assert_eq!(report.per_overlay[0].overlay_id, entry.overlay_ids[0]);
    assert_eq!(report.per_overlay[0].effect_key, entry.effect_keys[0]);
    assert_eq!(report.per_overlay[0].deltas.len(), 1);
}

#[test]
fn capability_tree_impact_preview_combined_net_effect() {
    let fixture = Fixture::new(tree_spec(vec![category(
        "ideas",
        "national",
        None,
        vec![entry_with_effects(
            "stacked_ethos",
            ActivationMode::PlayerSelection,
            vec![
                effect(SubFieldRole::Amount, TransformOp::Multiply(2.0)),
                effect(SubFieldRole::Amount, TransformOp::Add(3.0)),
            ],
        )],
    )]));
    let key = fixture.key("ideas", "national", "stacked_ethos");
    let fleet = fixture.registry.id_of("military", "fleet_speed").unwrap();
    let amount_col = fixture
        .registry
        .column_range(fleet)
        .col_for_role(
            &SubFieldRole::Amount,
            &fixture.registry.property(fleet).layout,
        )
        .unwrap();
    let mut shadow = vec![0.0; fixture.n_dims];
    shadow[amount_col] = 10.0;

    let report = fixture.preview(key, &shadow);

    assert_eq!(report.per_overlay.len(), 2);
    assert_eq!(report.combined.len(), 1);
    assert_eq!(report.combined[0].current, 10.0);
    assert_eq!(report.combined[0].after, 23.0);
}

#[test]
fn capability_tree_impact_preview_multi_effect_entry() {
    let fixture = Fixture::new(tree_spec(vec![category(
        "ideas",
        "national",
        None,
        vec![entry_with_effects(
            "combined_ethos",
            ActivationMode::PlayerSelection,
            vec![
                effect(SubFieldRole::Amount, TransformOp::Multiply(1.5)),
                effect(SubFieldRole::Velocity, TransformOp::Add(0.25)),
            ],
        )],
    )]));
    let key = fixture.key("ideas", "national", "combined_ethos");
    let shadow = vec![2.0; fixture.n_dims];

    let report = fixture.preview(key, &shadow);

    assert_eq!(report.per_overlay.len(), 2);
    assert_eq!(report.combined.len(), 2);
    assert!(report
        .combined
        .iter()
        .any(|d| d.role == SubFieldRole::Amount));
    assert!(report
        .combined
        .iter()
        .any(|d| d.role == SubFieldRole::Velocity));
}

#[test]
fn national_ideas_full_path_activate_switch_verify() {
    let mut fixture = Fixture::new(tree_spec(vec![category(
        "ideas",
        "national",
        Some(MaxActivePolicy::Limited {
            count: 1,
            replacement: ReplacementPolicy::SuspendOldest,
        }),
        vec![
            entry_with_effects(
                "naval_ethos",
                ActivationMode::PlayerSelection,
                vec![effect(SubFieldRole::Amount, TransformOp::Multiply(1.25))],
            ),
            entry_with_effects(
                "industrial_ethos",
                ActivationMode::PlayerSelection,
                vec![effect(SubFieldRole::Velocity, TransformOp::Add(0.25))],
            ),
        ],
    )]));
    let naval = fixture.key("ideas", "national", "naval_ethos");
    let industry = fixture.key("ideas", "national", "industrial_ethos");
    let naval_overlay = fixture.definition.entries[&naval].overlay_ids[0];
    let industry_overlay = fixture.definition.entries[&industry].overlay_ids[0];

    let mut definitions = HashMap::new();
    definitions.insert(fixture.definition.id, fixture.definition.clone());
    let mut instances = HashMap::new();
    instances.insert(
        fixture.owner_id,
        CapabilityTreeInstance {
            owner_id: fixture.owner_id,
            definition_id: fixture.definition.id,
            tree_thing_id: fixture.tree.id,
            tree_slot: 0,
        },
    );
    let mut states = HashMap::from([(fixture.owner_id, fixture.state())]);
    let mut requests = Vec::<BoundaryRequest>::new();
    let mut notifications = Vec::<CapabilityTreeNotification>::new();
    let mut diagnostics = Vec::new();
    let handler = simthing_spec::CapabilityTreeBoundaryHandler {
        registry: &fixture.registry,
        definitions: &definitions,
    };
    let mut shadow = vec![0.0; fixture.n_dims];
    {
        let mut ctx = CapabilityBoundaryContext {
            n_dims: fixture.n_dims,
            shadow: &mut shadow,
            instances: &instances,
            states: &mut states,
            requests: &mut requests,
            notifications: &mut notifications,
            diagnostics: &mut diagnostics,
        };
        handler
            .handle_player_selection(fixture.owner_id, naval.clone(), &mut ctx)
            .unwrap();
        handler
            .handle_player_selection(fixture.owner_id, industry.clone(), &mut ctx)
            .unwrap();
    }

    let mut allocator = SlotAllocator::new();
    allocator.alloc(fixture.tree.id);
    apply_structural_mutations(
        requests,
        &mut fixture.tree,
        &mut allocator,
        &mut fixture.registry,
        &mut shadow,
        fixture.n_dims,
        None,
    );

    let naval_lifecycle = &fixture
        .tree
        .overlays
        .iter()
        .find(|overlay| overlay.id == naval_overlay)
        .unwrap()
        .lifecycle;
    let industry_lifecycle = &fixture
        .tree
        .overlays
        .iter()
        .find(|overlay| overlay.id == industry_overlay)
        .unwrap()
        .lifecycle;

    assert!(matches!(
        naval_lifecycle,
        OverlayLifecycle::Suspended { .. }
    ));
    assert!(matches!(industry_lifecycle, OverlayLifecycle::Permanent));
    assert_eq!(
        notifications,
        vec![CapabilityTreeNotification::IdeaSwitched {
            owner_id: fixture.owner_id,
            category: naval.category.clone(),
            suspended: naval,
            activated: industry,
        }]
    );
}
