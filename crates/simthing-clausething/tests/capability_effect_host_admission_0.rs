use std::collections::HashMap;
use std::sync::Mutex;

use simthing_clausething::raw::RawValue;
use simthing_clausething::{
    hydrate_entity_pack, parse_raw_document, HydratedEntityPack, RawDocument,
};
use simthing_core::{
    ClampBehavior, DimensionRegistry, Overlay, SimPropertyId, SimThing, SimThingId, SimThingKind,
    SubFieldRole, SubFieldSpec,
};
use simthing_driver::{preview_install, InstallError, Scenario, SimSession};
use simthing_gpu::SlotAllocator;
use simthing_spec::{compile_property, ActivationMode, GameModeSpec, PropertySpec, SpecVersion};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const CLAUSE_EFFECT_PROOF: &str = r#"
effect_host_proof = {
    display_name = "Effect Host Proof"
    property = {
        id = effect_host_pressure
        namespace = effect_host
        name = pressure
        display_name = "Pressure"
        seed_amount = 2.0
    }
    tradition_tree = {
        id = effect_host_tree
        kind = tradition_tree
        owner = Faction
        category = {
            namespace = traditions
            name = effect_host
            display_name = "Effect Host"
            tradition = {
                id = triple_pressure
                display_name = "Triple Pressure"
                cost = 1.0
                modifier = {
                    targets_property = effect_host::pressure
                    amount_mult = 3.0
                }
            }
        }
    }
}
"#;

const CT1C_TRADITIONS: &str = include_str!("fixtures/ct1c_tradition_set.clause");
const MINIMAL_GAME_MODE: &str =
    include_str!("../../simthing-spec/tests/fixtures/minimal_game_mode.ron");
const EXAMPLE_ALL_FACTIONS: &str =
    include_str!("../../../docs/examples/game_mode_install_all_factions.ron");
const EXAMPLE_SCENARIO_LISTED: &str =
    include_str!("../../../docs/examples/game_mode_install_scenario_listed.ron");
const EXAMPLE_SESSION_ROOT: &str =
    include_str!("../../../docs/examples/game_mode_install_session_root.ron");

fn game_mode_from_hydrated(hydrated: HydratedEntityPack) -> GameModeSpec {
    GameModeSpec {
        id: "clause_effect_host_proof".into(),
        display_name: "Clause effect-host proof".into(),
        description: String::new(),
        spec_version: SpecVersion::default(),
        metadata: Default::default(),
        domain_packs: vec![hydrated.domain_pack],
        properties: Vec::new(),
        overlays: Vec::new(),
        order_weight_classes: vec![],
        capability_trees: Vec::new(),
        events: Vec::new(),
        resource_flow: None,
        resource_economy: None,
        region_fields: Vec::new(),
        mapping_execution_profile: Default::default(),
    }
}

fn hydrate_clause_mode(source: &str) -> (RawDocument, GameModeSpec) {
    let document = parse_raw_document(source.as_bytes()).expect("parse ClauseThing effect fixture");
    let hydrated = hydrate_entity_pack(&document).expect("hydrate ClauseThing effect fixture");
    (document, game_mode_from_hydrated(hydrated))
}

fn drain_declared_properties(game_mode: &mut GameModeSpec) -> Vec<PropertySpec> {
    let mut properties = std::mem::take(&mut game_mode.properties);
    for pack in &mut game_mode.domain_packs {
        properties.append(&mut pack.properties);
    }
    properties
}

fn scenario_with_admitted_properties(
    game_mode: &mut GameModeSpec,
    materialize_on_hosts: bool,
) -> (Scenario, SimThingId, HashMap<String, SimPropertyId>) {
    let property_specs = drain_declared_properties(game_mode);
    let mut registry = DimensionRegistry::new();
    let mut property_ids = HashMap::new();
    for property in property_specs {
        let key = format!("{}::{}", property.namespace, property.name);
        let (property_id, _) =
            compile_property(&property, &mut registry).expect("scenario property admission");
        property_ids.insert(key, property_id);
    }

    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut owner = SimThing::new(SimThingKind::Owner, 0);
    let owner_id = owner.id;
    if materialize_on_hosts {
        for property_id in property_ids.values().copied() {
            let default = registry.property(property_id).default_value();
            root.add_property(property_id, default.clone());
            owner.add_property(property_id, default);
        }
    }
    root.add_child(owner);

    let scenario = Scenario {
        name: "capability_effect_host_admission".into(),
        ticks_per_day: 1,
        max_days: 3,
        dt: 0.0,
        n_slots: 32,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::from([("player_faction".into(), vec![owner_id])]),
    };
    (scenario, owner_id, property_ids)
}

fn set_owner_amount(
    scenario: &mut Scenario,
    owner_id: SimThingId,
    property_id: SimPropertyId,
    amount: f32,
) {
    let property = scenario.registry.property(property_id);
    let mut value = property.default_value();
    value.set_role(&SubFieldRole::Amount, &property.layout, amount);
    let owner = find_node_mut(&mut scenario.root, owner_id).expect("owner host");
    owner.add_property(property_id, value);
}

fn preserve_authored_amount_through_ticks(game_mode: &mut GameModeSpec) {
    let property = game_mode.domain_packs[0]
        .properties
        .iter_mut()
        .find(|property| property.namespace == "effect_host" && property.name == "pressure")
        .expect("hydrated effect-host property");
    property.sub_fields = vec![SubFieldSpec {
        role: SubFieldRole::Amount,
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: "amount".into(),
        display_range: None,
        governed_by: None,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: None,
    }];
}

fn find_node(node: &SimThing, target: SimThingId) -> Option<&SimThing> {
    if node.id == target {
        return Some(node);
    }
    node.children
        .iter()
        .find_map(|child| find_node(child, target))
}

fn find_node_mut(node: &mut SimThing, target: SimThingId) -> Option<&mut SimThing> {
    if node.id == target {
        return Some(node);
    }
    node.children
        .iter_mut()
        .find_map(|child| find_node_mut(child, target))
}

fn find_overlay(node: &SimThing, overlay_id: simthing_core::OverlayId) -> Option<&Overlay> {
    node.overlays
        .iter()
        .find(|overlay| overlay.id == overlay_id)
        .or_else(|| {
            node.children
                .iter()
                .find_map(|child| find_overlay(child, overlay_id))
        })
}

fn property_reference_span(document: &RawDocument) -> (usize, usize) {
    fn walk(value: &RawValue, found: &mut Option<(usize, usize)>) {
        match value {
            RawValue::Block(block) => {
                for property in &block.properties {
                    if property.key.text == "targets_property" {
                        let RawValue::Scalar(reference) = &property.value else {
                            panic!("targets_property reference must be scalar");
                        };
                        *found = Some((reference.span.token_index, property.key.span.token_index));
                        return;
                    }
                    walk(&property.value, found);
                    if found.is_some() {
                        return;
                    }
                }
            }
            RawValue::Array(array) => {
                for value in &array.items {
                    walk(value, found);
                    if found.is_some() {
                        return;
                    }
                }
            }
            RawValue::Header(header) => walk(&header.payload, found),
            RawValue::Scalar(_) => {}
        }
    }

    let mut found = None;
    walk(&document.root, &mut found);
    found.expect("authored targets_property reference span")
}

fn effect_count(game_mode: &GameModeSpec) -> usize {
    game_mode
        .capability_trees
        .iter()
        .chain(
            game_mode
                .domain_packs
                .iter()
                .flat_map(|pack| pack.capability_trees.iter()),
        )
        .flat_map(|tree| tree.categories.iter())
        .flat_map(|category| category.entries.iter())
        .map(|entry| entry.effects.len())
        .sum()
}

fn assert_preview_placement(preview: &simthing_driver::InstallPreview) -> usize {
    let mut admitted = 0;
    for instance in preview.state.capability_instances.values() {
        for overlay_id in instance.by_overlay.keys().copied() {
            let host = *instance
                .overlay_hosts
                .get(&overlay_id)
                .expect("overlay host admission record");
            let overlay = find_overlay(&preview.root, overlay_id)
                .expect("overlay physically placed on admitted tree");
            assert_eq!(overlay.affects, vec![host]);
            assert!(
                find_node(&preview.root, host)
                    .expect("admitted host")
                    .overlays
                    .iter()
                    .any(|candidate| candidate.id == overlay_id),
                "overlay must live on the host recorded by overlay_hosts"
            );
            admitted += 1;
        }
    }
    admitted
}

/// catches: admitted ClauseThing effect placement is never consumed by the ordinary
/// player-selection boundary, overlay-prep, and GPU session path.
#[test]
fn clause_effect_host_executes_through_boundary_overlay_prep_and_gpu() {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|error| error.into_inner());
    let (_, mut game_mode) = hydrate_clause_mode(CLAUSE_EFFECT_PROOF);
    // ClauseThing's compact property syntax hydrates the standard unit-interval
    // layout. This execution referee supplies the ordinary compiled spec with an
    // explicit unbounded Amount so the authored 2.0 baseline survives Pass 2.
    preserve_authored_amount_through_ticks(&mut game_mode);
    game_mode.domain_packs[0].capability_trees[0].categories[0].entries[0].activation =
        ActivationMode::PlayerSelection;
    let (mut scenario, owner_id, property_ids) =
        scenario_with_admitted_properties(&mut game_mode, true);
    let property_id = property_ids["effect_host::pressure"];
    set_owner_amount(&mut scenario, owner_id, property_id, 2.0);
    let authored_property = scenario.registry.property(property_id);
    assert_eq!(
        find_node(&scenario.root, owner_id)
            .expect("authored owner")
            .properties[&property_id]
            .get_role(&SubFieldRole::Amount, &authored_property.layout)
            .to_bits(),
        2.0_f32.to_bits()
    );

    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&scenario.root);
    let preview = preview_install(
        &game_mode,
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
    )
    .expect("correctly hosted ClauseThing capability effect must admit");
    assert_eq!(assert_preview_placement(&preview), 1);

    let mut session =
        SimSession::open_from_spec(scenario, &game_mode).expect("open live GPU session");
    let adapter = session.state.ctx.adapter.get_info();
    let instance = session
        .spec_state
        .capability_instances
        .values()
        .next()
        .expect("installed capability instance")
        .clone();
    let overlay_id = *instance
        .by_overlay
        .keys()
        .next()
        .expect("installed overlay");
    assert_eq!(instance.overlay_hosts.get(&overlay_id), Some(&owner_id));
    assert!(session.proto.root.has_overlay(owner_id, overlay_id));
    let owner_slot = session
        .proto
        .allocator
        .slot_of(owner_id)
        .expect("owner slot");
    let property = session.proto.registry.property(property_id);
    let amount_col = session
        .proto
        .registry
        .column_range(property_id)
        .col_for_role(&SubFieldRole::Amount, &property.layout)
        .expect("Amount column");
    let opened = session.state.read_values_row(owner_slot.raw());
    assert_eq!(
        opened[amount_col.raw() as usize].to_bits(),
        2.0_f32.to_bits()
    );

    session
        .spec_state
        .queue_player_selection_by_key(owner_id, "effect_host_tree", "triple_pressure")
        .expect("queue ordinary capability selection");
    assert!(
        session
            .step_once()
            .expect("activation boundary")
            .boundary_reached,
        "selection must activate through the ordinary boundary"
    );
    let activated_boundary = session.state.read_values_row(owner_slot.raw());
    assert_eq!(
        activated_boundary[amount_col.raw() as usize].to_bits(),
        2.0_f32.to_bits()
    );
    session
        .step_once()
        .expect("post-activation production GPU tick");

    let row = session.state.read_values_row(owner_slot.raw());
    assert_eq!(row[amount_col.raw() as usize].to_bits(), 6.0_f32.to_bits());
    eprintln!(
        "OVERLAY-EFFECT-HOST-GPU-PROOF adapter={:?} backend={:?} device_type={:?} owner={owner_id:?} overlay={overlay_id:?} value_bits={} expected_bits={}",
        adapter.name,
        adapter.backend,
        adapter.device_type,
        row[amount_col.raw() as usize].to_bits(),
        6.0_f32.to_bits()
    );
}

/// catches: a ClauseThing admission error echoes a manually injected token instead
/// of preserving the parser span of the authored targets_property scalar.
#[test]
fn clause_effect_host_rejection_uses_authored_property_reference_span() {
    let (document, mut game_mode) = hydrate_clause_mode(CLAUSE_EFFECT_PROOF);
    let (reference_span, key_span) = property_reference_span(&document);
    assert_ne!(
        reference_span, key_span,
        "proof must distinguish the property reference from its field key"
    );
    let (scenario, owner_id, _) = scenario_with_admitted_properties(&mut game_mode, false);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&scenario.root);

    let error = preview_install(
        &game_mode,
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
    )
    .expect_err("missing authored host property must fail admission");
    let rendered = error.to_string();
    match error {
        InstallError::CapabilityOverlayHostAdmission {
            overlay_id,
            resolved_host,
            property,
            source_span_token,
            reason,
        } => {
            assert_eq!(resolved_host, owner_id);
            assert_eq!(property, "effect_host::pressure");
            assert_eq!(source_span_token, Some(reference_span));
            assert!(reason.contains("does not carry"));
            assert!(rendered.contains(&format!("{overlay_id:?}")));
            assert!(rendered.contains(&format!("{owner_id:?}")));
            assert!(rendered.contains("effect_host::pressure"));
            assert!(rendered.contains(&format!("Some({reference_span})")));
        }
        other => panic!("unexpected admission error: {other}"),
    }
}

/// catches: an existing RON/example/ClauseThing capability effect becomes a
/// missing-property admission failure when scenario admission materializes its
/// declared property on the resolved host.
#[test]
fn existing_capability_effect_corpus_has_no_mis_hosted_overlay() {
    let (_, clause_mode) = hydrate_clause_mode(CT1C_TRADITIONS);
    let mut corpus = vec![
        (
            "minimal_game_mode",
            ron::from_str::<GameModeSpec>(MINIMAL_GAME_MODE).expect("minimal game mode"),
        ),
        (
            "example_all_factions",
            ron::from_str::<GameModeSpec>(EXAMPLE_ALL_FACTIONS).expect("all-factions example"),
        ),
        (
            "example_scenario_listed",
            ron::from_str::<GameModeSpec>(EXAMPLE_SCENARIO_LISTED)
                .expect("scenario-listed example"),
        ),
        (
            "example_session_root",
            ron::from_str::<GameModeSpec>(EXAMPLE_SESSION_ROOT).expect("session-root example"),
        ),
        ("clausething_ct1c_traditions", clause_mode),
    ];

    let mut total_effects = 0;
    let mut total_admitted = 0;
    for (name, mut game_mode) in corpus.drain(..) {
        let authored = effect_count(&game_mode);
        let (scenario, _, _) = scenario_with_admitted_properties(&mut game_mode, true);
        let mut allocator = SlotAllocator::new();
        allocator.populate_from_tree(&scenario.root);
        let preview = preview_install(
            &game_mode,
            &scenario,
            &scenario.registry,
            &scenario.root,
            &allocator,
        )
        .unwrap_or_else(|error| {
            panic!("production capability-effect corpus `{name}` failed host admission: {error}")
        });
        let admitted = assert_preview_placement(&preview);
        assert_eq!(
            admitted, authored,
            "every authored effect in `{name}` must have one admitted placement"
        );
        eprintln!(
            "OVERLAY-EFFECT-HOST-CENSUS corpus={name} authored={authored} admitted={admitted} missing=0"
        );
        total_effects += authored;
        total_admitted += admitted;
    }
    assert_eq!(total_effects, 7);
    assert_eq!(total_admitted, total_effects);
    eprintln!(
        "OVERLAY-EFFECT-HOST-CENSUS total_authored={total_effects} total_admitted={total_admitted} missing=0"
    );
}
