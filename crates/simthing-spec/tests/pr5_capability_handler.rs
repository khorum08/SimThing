use simthing_core::{DimensionRegistry, OverlayLifecycle, SimThingId, SubFieldRole, TransformOp};
use simthing_feeder::BoundaryRequest;
use simthing_gpu::ThresholdEvent;
use simthing_sim::{ThresholdRegistry, ThresholdSemantic};
use simthing_spec::{
    compile_property, ActivationMode, CapabilityBoundaryContext, CapabilityCategorySpec,
    CapabilityEffectSpec, CapabilityEntryKey, CapabilityPrereqSpec, CapabilitySpec,
    CapabilityTreeBuilder, CapabilityTreeDefinition, CapabilityTreeDiagnostic,
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

fn entry(
    id: &str,
    research_cost: f32,
    activation: ActivationMode,
    prereqs: Vec<CapabilityPrereqSpec>,
) -> CapabilitySpec {
    CapabilitySpec {
        id: id.into(),
        display_name: id.into(),
        description: String::new(),
        flavor_text: String::new(),
        research_cost,
        activation,
        research_rate: Default::default(),
        icon: String::new(),
        thumbnail: String::new(),
        card_image: String::new(),
        unlock_video: None,
        model_preview: None,
        prereqs,
        unlocks_ship_components: vec![],
        unlocks_buildings: vec![],
        unlocks_units: vec![],
        unlocks_weapons: vec![],
        effects: vec![CapabilityEffectSpec {
            targets_property: "military::fleet_speed".into(),
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Multiply(1.10))],
            when_activated: OverlayLifecycle::Permanent,
        }],
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
        tree_id: "terran_tech".into(),
        tree_kind: "tech_tree".into(),
        owner_kind: "Faction".into(),
        categories,
    }
}

struct Fixture {
    registry: DimensionRegistry,
    definition: CapabilityTreeDefinition,
    owner_id: SimThingId,
    tree_id: SimThingId,
    tree_slot: u32,
    n_dims: usize,
}

impl Fixture {
    fn new(spec: simthing_spec::CapabilityTreeSpec) -> Self {
        let mut registry = registry_with_fleet_speed();
        let (out, _) = CapabilityTreeBuilder::build(&spec, &mut registry).expect("build");
        let owner_id = SimThingId::new();
        let tree_id = SimThingId::new();
        let n_dims = registry.total_columns;
        Self {
            registry,
            definition: out.definition,
            owner_id,
            tree_id,
            tree_slot: 0,
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

    fn instance(&self) -> CapabilityTreeInstance {
        CapabilityTreeInstance {
            owner_id: self.owner_id,
            definition_id: self.definition.id,
            tree_thing_id: self.tree_id,
            tree_slot: self.tree_slot,
        }
    }

    fn threshold_event(&self, key: &CapabilityEntryKey) -> (ThresholdRegistry, ThresholdEvent) {
        let entry = &self.definition.entries[key];
        let property_id = self.definition.categories[&key.category].property_id;
        let mut cpu = ThresholdRegistry::new();
        let event_kind = cpu.push(ThresholdSemantic::CapabilityUnlock {
            sim_thing_id: self.tree_id,
            property_id,
            sub_field: SubFieldRole::Named(key.entry_id.clone()),
        });
        (
            cpu,
            ThresholdEvent {
                slot: self.tree_slot,
                col: entry.progress_col as u32,
                value: entry.research_cost,
                event_kind,
            },
        )
    }

    fn shadow(&self, slots: usize) -> Vec<f32> {
        vec![0.0; slots * self.n_dims]
    }
}

fn with_context<R>(
    fixture: &Fixture,
    shadow: &mut [f32],
    states: &mut HashMap<SimThingId, CapabilityTreeState>,
    requests: &mut Vec<BoundaryRequest>,
    notifications: &mut Vec<CapabilityTreeNotification>,
    diagnostics: &mut Vec<CapabilityTreeDiagnostic>,
    f: impl FnOnce(
        &simthing_spec::CapabilityTreeBoundaryHandler<'_>,
        &mut CapabilityBoundaryContext<'_>,
    ) -> R,
) -> R {
    let mut definitions = HashMap::new();
    definitions.insert(fixture.definition.id, fixture.definition.clone());
    let mut instances = HashMap::new();
    instances.insert(fixture.owner_id, fixture.instance());
    let handler = simthing_spec::CapabilityTreeBoundaryHandler {
        registry: &fixture.registry,
        definitions: &definitions,
    };
    let mut ctx = CapabilityBoundaryContext {
        n_dims: fixture.n_dims,
        shadow,
        instances: &instances,
        states,
        requests,
        notifications,
        diagnostics,
    };
    f(&handler, &mut ctx)
}

fn two_entry_threshold_fixture() -> Fixture {
    Fixture::new(tree_spec(vec![category(
        "tech",
        "propulsion",
        None,
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
    )]))
}

#[test]
fn capability_tree_boundary_handler_activates_on_threshold() {
    let fixture = Fixture::new(tree_spec(vec![category(
        "tech",
        "propulsion",
        None,
        vec![entry(
            "chemical_drive",
            5000.0,
            ActivationMode::Threshold,
            vec![],
        )],
    )]));
    let key = fixture.key("tech", "propulsion", "chemical_drive");
    let (cpu, event) = fixture.threshold_event(&key);
    let mut shadow = fixture.shadow(1);
    let mut states = HashMap::from([(fixture.owner_id, fixture.state())]);
    let mut requests = Vec::new();
    let mut notifications = Vec::new();
    let mut diagnostics = Vec::new();

    with_context(
        &fixture,
        &mut shadow,
        &mut states,
        &mut requests,
        &mut notifications,
        &mut diagnostics,
        |handler, ctx| {
            handler
                .handle_threshold_events(&[event], &cpu, ctx)
                .unwrap()
        },
    );

    let overlay_id = fixture.definition.entries[&key].overlay_ids[0];
    assert!(matches!(
        requests.as_slice(),
        [BoundaryRequest::ActivateOverlay { target, overlay_id: id }]
            if *target == fixture.tree_id && *id == overlay_id
    ));
    assert!(diagnostics.is_empty());
}

#[test]
fn capability_tree_prereq_blocks_activation_and_resets_progress() {
    let fixture = two_entry_threshold_fixture();
    let key = fixture.key("tech", "propulsion", "fusion_drive");
    let (cpu, event) = fixture.threshold_event(&key);
    let mut shadow = fixture.shadow(1);
    let progress_col = fixture.definition.entries[&key].progress_col;
    shadow[progress_col] = 8000.0;
    let mut states = HashMap::from([(fixture.owner_id, fixture.state())]);
    let mut requests = Vec::new();
    let mut notifications = Vec::new();
    let mut diagnostics = Vec::new();

    with_context(
        &fixture,
        &mut shadow,
        &mut states,
        &mut requests,
        &mut notifications,
        &mut diagnostics,
        |handler, ctx| {
            handler
                .handle_threshold_events(&[event], &cpu, ctx)
                .unwrap()
        },
    );

    assert!(requests.is_empty());
    assert!(shadow[progress_col] < 8000.0);
    assert!(shadow[progress_col] > 7999.0);
}

#[test]
fn capability_tree_failed_prereq_enters_on_prereq_met() {
    let fixture = two_entry_threshold_fixture();
    let key = fixture.key("tech", "propulsion", "fusion_drive");
    let (cpu, event) = fixture.threshold_event(&key);
    let mut shadow = fixture.shadow(1);
    let mut states = HashMap::from([(fixture.owner_id, fixture.state())]);
    let mut requests = Vec::new();
    let mut notifications = Vec::new();
    let mut diagnostics = Vec::new();

    with_context(
        &fixture,
        &mut shadow,
        &mut states,
        &mut requests,
        &mut notifications,
        &mut diagnostics,
        |handler, ctx| {
            handler
                .handle_threshold_events(&[event], &cpu, ctx)
                .unwrap()
        },
    );

    assert_eq!(
        states[&fixture.owner_id].activation_mode_by_entry.get(&key),
        Some(&ActivationMode::OnPrereqMet)
    );
}

#[test]
fn capability_tree_on_prereq_met_sweep_activates_after_dependency_unlock() {
    let fixture = two_entry_threshold_fixture();
    let chemical = fixture.key("tech", "propulsion", "chemical_drive");
    let fusion = fixture.key("tech", "propulsion", "fusion_drive");
    let (cpu, event) = fixture.threshold_event(&chemical);
    let mut shadow = fixture.shadow(1);
    shadow[fixture.definition.entries[&chemical].progress_col] = 5000.0;
    let mut state = fixture.state();
    state
        .activation_mode_by_entry
        .insert(fusion.clone(), ActivationMode::OnPrereqMet);
    let mut states = HashMap::from([(fixture.owner_id, state)]);
    let mut requests = Vec::new();
    let mut notifications = Vec::new();
    let mut diagnostics = Vec::new();

    with_context(
        &fixture,
        &mut shadow,
        &mut states,
        &mut requests,
        &mut notifications,
        &mut diagnostics,
        |handler, ctx| {
            handler
                .handle_threshold_events(&[event], &cpu, ctx)
                .unwrap()
        },
    );

    assert_eq!(requests.len(), 2);
    assert!(!states[&fixture.owner_id]
        .activation_mode_by_entry
        .contains_key(&fusion));
}

#[test]
fn capability_tree_player_selection_activates_without_threshold() {
    let fixture = Fixture::new(tree_spec(vec![category(
        "ideas",
        "national",
        None,
        vec![entry(
            "naval_ethos",
            0.0,
            ActivationMode::PlayerSelection,
            vec![],
        )],
    )]));
    let key = fixture.key("ideas", "national", "naval_ethos");
    let mut shadow = fixture.shadow(1);
    let mut states = HashMap::from([(fixture.owner_id, fixture.state())]);
    let mut requests = Vec::new();
    let mut notifications = Vec::new();
    let mut diagnostics = Vec::new();

    with_context(
        &fixture,
        &mut shadow,
        &mut states,
        &mut requests,
        &mut notifications,
        &mut diagnostics,
        |handler, ctx| {
            handler
                .handle_player_selection(fixture.owner_id, key.clone(), ctx)
                .unwrap()
        },
    );

    assert!(matches!(
        requests[0],
        BoundaryRequest::ActivateOverlay { .. }
    ));
}

#[test]
fn capability_tree_cross_category_prereq_resolves() {
    let fixture = Fixture::new(tree_spec(vec![
        category(
            "tech",
            "physics",
            None,
            vec![entry(
                "relativity",
                3000.0,
                ActivationMode::Threshold,
                vec![],
            )],
        ),
        category(
            "tech",
            "propulsion",
            None,
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
    ]));
    let prereq = fixture.key("tech", "physics", "relativity");
    let warp = fixture.key("tech", "propulsion", "warp_drive");
    let (cpu, event) = fixture.threshold_event(&warp);
    let mut shadow = fixture.shadow(1);
    shadow[fixture.definition.entries[&prereq].progress_col] = 3000.0;
    let mut states = HashMap::from([(fixture.owner_id, fixture.state())]);
    let mut requests = Vec::new();
    let mut notifications = Vec::new();
    let mut diagnostics = Vec::new();

    with_context(
        &fixture,
        &mut shadow,
        &mut states,
        &mut requests,
        &mut notifications,
        &mut diagnostics,
        |handler, ctx| {
            handler
                .handle_threshold_events(&[event], &cpu, ctx)
                .unwrap()
        },
    );

    assert_eq!(requests.len(), 1);
    assert!(matches!(
        requests[0],
        BoundaryRequest::ActivateOverlay { .. }
    ));
}

#[test]
fn capability_tree_state_is_per_faction_not_shared() {
    let fixture = Fixture::new(tree_spec(vec![category(
        "tech",
        "propulsion",
        None,
        vec![entry(
            "chemical_drive",
            5000.0,
            ActivationMode::Threshold,
            vec![],
        )],
    )]));
    let key = fixture.key("tech", "propulsion", "chemical_drive");
    let owner_b = SimThingId::new();
    let tree_b = SimThingId::new();
    let mut instances = HashMap::new();
    instances.insert(fixture.owner_id, fixture.instance());
    instances.insert(
        owner_b,
        CapabilityTreeInstance {
            owner_id: owner_b,
            definition_id: fixture.definition.id,
            tree_thing_id: tree_b,
            tree_slot: 1,
        },
    );
    let mut definitions = HashMap::new();
    definitions.insert(fixture.definition.id, fixture.definition.clone());
    let mut state_b = fixture.state();
    state_b.owner_id = owner_b;
    let mut states = HashMap::from([(fixture.owner_id, fixture.state()), (owner_b, state_b)]);
    let (cpu, event) = fixture.threshold_event(&key);
    let mut shadow = fixture.shadow(2);
    let mut requests = Vec::new();
    let mut notifications = Vec::new();
    let mut diagnostics = Vec::new();
    let handler = simthing_spec::CapabilityTreeBoundaryHandler {
        registry: &fixture.registry,
        definitions: &definitions,
    };
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
        .handle_threshold_events(&[event], &cpu, &mut ctx)
        .unwrap();

    assert!(states[&fixture.owner_id].active_by_category[&key.category].contains(&key));
    assert!(states[&owner_b].active_by_category.is_empty());
}

fn ideas_fixture() -> Fixture {
    Fixture::new(tree_spec(vec![category(
        "ideas",
        "national",
        Some(MaxActivePolicy::Limited {
            count: 1,
            replacement: ReplacementPolicy::SuspendOldest,
        }),
        vec![
            entry("naval_ethos", 0.0, ActivationMode::PlayerSelection, vec![]),
            entry(
                "industrial_ethos",
                0.0,
                ActivationMode::PlayerSelection,
                vec![],
            ),
        ],
    )]))
}

#[test]
fn national_ideas_mutual_exclusivity_suspends_sibling() {
    let fixture = ideas_fixture();
    let naval = fixture.key("ideas", "national", "naval_ethos");
    let industry = fixture.key("ideas", "national", "industrial_ethos");
    let mut shadow = fixture.shadow(1);
    let mut states = HashMap::from([(fixture.owner_id, fixture.state())]);
    let mut requests = Vec::new();
    let mut notifications = Vec::new();
    let mut diagnostics = Vec::new();

    with_context(
        &fixture,
        &mut shadow,
        &mut states,
        &mut requests,
        &mut notifications,
        &mut diagnostics,
        |handler, ctx| {
            handler
                .handle_player_selection(fixture.owner_id, naval.clone(), ctx)
                .unwrap();
            handler
                .handle_player_selection(fixture.owner_id, industry.clone(), ctx)
                .unwrap();
        },
    );

    assert!(requests
        .iter()
        .any(|req| matches!(req, BoundaryRequest::SuspendOverlay { .. })));
    assert_eq!(
        states[&fixture.owner_id].active_by_category[&naval.category],
        vec![industry]
    );
}

#[test]
fn national_ideas_mutual_exclusivity_emits_notification() {
    let fixture = ideas_fixture();
    let naval = fixture.key("ideas", "national", "naval_ethos");
    let industry = fixture.key("ideas", "national", "industrial_ethos");
    let mut shadow = fixture.shadow(1);
    let mut states = HashMap::from([(fixture.owner_id, fixture.state())]);
    let mut requests = Vec::new();
    let mut notifications = Vec::new();
    let mut diagnostics = Vec::new();

    with_context(
        &fixture,
        &mut shadow,
        &mut states,
        &mut requests,
        &mut notifications,
        &mut diagnostics,
        |handler, ctx| {
            handler
                .handle_player_selection(fixture.owner_id, naval.clone(), ctx)
                .unwrap();
            handler
                .handle_player_selection(fixture.owner_id, industry.clone(), ctx)
                .unwrap();
        },
    );

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

#[test]
fn capability_tree_sweep_runs_at_session_init() {
    let fixture = two_entry_threshold_fixture();
    let chemical = fixture.key("tech", "propulsion", "chemical_drive");
    let fusion = fixture.key("tech", "propulsion", "fusion_drive");
    let mut shadow = fixture.shadow(1);
    shadow[fixture.definition.entries[&chemical].progress_col] = 5000.0;
    let mut state = fixture.state();
    state
        .activation_mode_by_entry
        .insert(fusion.clone(), ActivationMode::OnPrereqMet);
    let mut states = HashMap::from([(fixture.owner_id, state)]);
    let mut requests = Vec::new();
    let mut notifications = Vec::new();
    let mut diagnostics = Vec::new();

    with_context(
        &fixture,
        &mut shadow,
        &mut states,
        &mut requests,
        &mut notifications,
        &mut diagnostics,
        |handler, ctx| handler.sweep_on_prereq_met(fixture.owner_id, ctx).unwrap(),
    );

    assert_eq!(requests.len(), 1);
    assert!(!states[&fixture.owner_id]
        .activation_mode_by_entry
        .contains_key(&fusion));
}
