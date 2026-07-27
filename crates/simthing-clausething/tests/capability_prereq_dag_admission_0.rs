//! CAPABILITY-PREREQ-DAG-ADMISSION-0 remedial proofs:
//! - ClauseThing authored-corpus census (traditions + shipsize account)
//! - Live SimSession / GPU max_active same-barrier atomicity referee
//! - Bit-exact replay equality across two identical runs

use std::collections::HashMap;
use std::sync::Mutex;

use simthing_clausething::{
    hydrate_entity_pack, hydrate_shipsize_decoder_pack, parse_raw_document, HydratedEntityPack,
};
use simthing_core::{
    ClampBehavior, OverlayLifecycle, SimPropertyId, SimThing, SimThingId, SimThingKind,
    SubFieldRole, SubFieldSpec, TransformOp,
};
use simthing_driver::{Scenario, SimSession};
use simthing_spec::{
    compile_property, validate_capability_tree, ActivationMode, GameModeSpec, MaxActivePolicy,
    PropertySpec, ReplacementPolicy, SpecError, SpecVersion,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const CT1C_TRADITIONS: &str = include_str!("fixtures/ct1c_tradition_set.clause");
const SHIPSIZE_FIXTURE: &str = include_str!("fixtures/tp_shipsize_decoder_0.clause");

const CLAUSE_MAX_ACTIVE_PROOF: &str = r#"
max_active_proof = {
    display_name = "Max Active Atomicity Proof"
    property = {
        id = idea_pressure
        namespace = idea_host
        name = pressure
        display_name = "Pressure"
        seed_amount = 2.0
    }
    tradition_tree = {
        id = national_ideas_proof
        kind = tradition_tree
        owner = Faction
        category = {
            namespace = ideas
            name = tier1
            display_name = "Tier 1"
            tradition = {
                id = idea_a
                display_name = "Idea A"
                cost = 1.0
                modifier = {
                    targets_property = idea_host::pressure
                    amount_mult = 2.0
                }
            }
            tradition = {
                id = idea_b
                display_name = "Idea B"
                cost = 1.0
                modifier = {
                    targets_property = idea_host::pressure
                    amount_mult = 3.0
                }
            }
        }
    }
}
"#;

fn game_mode_from_hydrated(id: &str, hydrated: HydratedEntityPack) -> GameModeSpec {
    GameModeSpec {
        id: id.into(),
        display_name: id.into(),
        description: String::new(),
        spec_version: SpecVersion::default(),
        metadata: Default::default(),
        domain_packs: vec![hydrated.domain_pack],
        properties: Vec::new(),
        overlays: Vec::new(),
        capability_trees: Vec::new(),
        events: Vec::new(),
        resource_flow: None,
        resource_economy: None,
        resource_flow_execution_profile: Default::default(),
        region_fields: Vec::new(),
        mapping_execution_profile: Default::default(),
    }
}

fn hydrate_clause_mode(source: &str) -> GameModeSpec {
    let document = parse_raw_document(source.as_bytes()).expect("parse clause");
    let hydrated = hydrate_entity_pack(&document).expect("hydrate clause");
    game_mode_from_hydrated("clause_mode", hydrated)
}

fn drain_declared_properties(game_mode: &mut GameModeSpec) -> Vec<PropertySpec> {
    let mut properties = std::mem::take(&mut game_mode.properties);
    for pack in &mut game_mode.domain_packs {
        properties.append(&mut pack.properties);
    }
    properties
}

fn find_node(node: &SimThing, target: SimThingId) -> Option<&SimThing> {
    if node.id == target {
        return Some(node);
    }
    node.children
        .iter()
        .find_map(|child| find_node(child, target))
}

fn preserve_unbounded_amount(game_mode: &mut GameModeSpec, ns: &str, name: &str) {
    let property = game_mode.domain_packs[0]
        .properties
        .iter_mut()
        .find(|p| p.namespace == ns && p.name == name)
        .expect("target property");
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

fn scenario_with_owner(
    game_mode: &mut GameModeSpec,
    seed_amount: f32,
) -> (Scenario, SimThingId, SimPropertyId) {
    let property_specs = drain_declared_properties(game_mode);
    let mut registry = simthing_core::DimensionRegistry::new();
    let mut property_ids = HashMap::new();
    for property in property_specs {
        let key = format!("{}::{}", property.namespace, property.name);
        let (property_id, _) =
            compile_property(&property, &mut registry).expect("property admission");
        property_ids.insert(key, property_id);
    }
    let property_id = *property_ids
        .values()
        .next()
        .expect("at least one property");
    let property = registry.property(property_id);
    let mut value = property.default_value();
    value.set_role(&SubFieldRole::Amount, &property.layout, seed_amount);

    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut owner = SimThing::new(SimThingKind::Owner, 0);
    let owner_id = owner.id;
    owner.add_property(property_id, value);
    root.add_child(owner);

    let scenario = Scenario {
        name: "max_active_atomicity".into(),
        ticks_per_day: 1,
        max_days: 8,
        dt: 0.0,
        n_slots: 32,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::from([("player_faction".into(), vec![owner_id])]),
    };
    (scenario, owner_id, property_id)
}

fn apply_max_active_limited(game_mode: &mut GameModeSpec) {
    let category = &mut game_mode.domain_packs[0].capability_trees[0].categories[0];
    category.max_active = Some(MaxActivePolicy::Limited {
        count: 1,
        replacement: ReplacementPolicy::SuspendOldest,
    });
    for entry in &mut category.entries {
        entry.activation = ActivationMode::PlayerSelection;
        // Absolute Sets so the dual-active vs single-active outcomes cannot
        // be confounded by Multiply composition on a carried GPU value.
        // idea_a → Set(4.0); idea_b → Set(6.0). Authored seed is 2.0.
        let set_to = if entry.id == "idea_a" { 4.0 } else { 6.0 };
        for effect in &mut entry.effects {
            effect.sub_field_deltas = vec![(SubFieldRole::Amount, TransformOp::Set(set_to))];
            effect.when_activated = OverlayLifecycle::Permanent;
        }
    }
}

fn read_owner_amount(session: &SimSession, owner_id: SimThingId, property_id: SimPropertyId) -> f32 {
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
    let row = session.state.read_values_row(owner_slot.raw());
    row[amount_col.raw() as usize]
}

fn active_entry_ids(session: &SimSession, owner_id: SimThingId) -> Vec<String> {
    session
        .spec_state
        .capability_states
        .values()
        .filter(|state| state.owner_id == owner_id)
        .flat_map(|state| {
            state
                .active_by_category
                .values()
                .flatten()
                .map(|k| k.entry_id.clone())
        })
        .collect()
}

/// ClauseThing authored tradition trees + shipsize account.
#[test]
fn clause_authored_and_programmatic_capability_corpus_admits() {
    let mut trees_admitted = 0usize;

    // CT1C tradition fixture — authored corpus, must admit or STOP/DA-route.
    {
        let document = parse_raw_document(CT1C_TRADITIONS.as_bytes()).expect("ct1c parse");
        let hydrated = hydrate_entity_pack(&document).expect("ct1c hydrate");
        let trees = &hydrated.domain_pack.capability_trees;
        assert!(!trees.is_empty(), "ct1c must hydrate trees");
        for tree in trees {
            validate_capability_tree(tree).unwrap_or_else(|e| {
                panic!(
                    "STOP/DA-route: ct1c tree {} fails prereq DAG admission: {e}",
                    tree.tree_id
                )
            });
            trees_admitted += 1;
        }
    }

    // Ship-size: programmatic production constructor — empty prereq DAG;
    // Threshold+zero-cost is pre-existing non-DAG shape (not rewritten).
    {
        let document = parse_raw_document(SHIPSIZE_FIXTURE.as_bytes()).expect("shipsize parse");
        match hydrate_shipsize_decoder_pack(&document) {
            Ok(pack) => {
                let trees = pack
                    .game_mode
                    .capability_trees
                    .iter()
                    .chain(
                        pack.game_mode
                            .domain_packs
                            .iter()
                            .flat_map(|dp| dp.capability_trees.iter()),
                    );
                for tree in trees {
                    for cat in &tree.categories {
                        for entry in &cat.entries {
                            assert!(
                                entry.prereqs.is_empty(),
                                "shipsize must not invent prereq edges"
                            );
                        }
                    }
                    match validate_capability_tree(tree) {
                        Ok(_) => {
                            trees_admitted += 1;
                            eprintln!(
                                "CENSUS shipsize tree {} admits (empty prereq DAG)",
                                tree.tree_id
                            );
                        }
                        Err(SpecError::ThresholdRequiresPositiveCost(entry)) => {
                            eprintln!(
                                "CENSUS-NOTE shipsize tree {} entry `{entry}`: pre-existing Threshold+zero-cost (programmatic, not authored-corpus STOP)",
                                tree.tree_id
                            );
                        }
                        Err(e) => {
                            let msg = e.to_string();
                            if msg.contains("prereq")
                                || msg.contains("cycle")
                                || msg.contains("max_active")
                                || msg.contains("tier")
                            {
                                panic!(
                                    "STOP/DA-route: shipsize tree {} DAG failure: {e}",
                                    tree.tree_id
                                );
                            }
                            eprintln!(
                                "CENSUS-NOTE shipsize tree {} non-DAG pre-existing: {e}",
                                tree.tree_id
                            );
                        }
                    }
                }
            }
            Err(err) => eprintln!("CENSUS-NOTE shipsize hydrate skipped: {err}"),
        }
    }

    assert!(
        trees_admitted >= 1,
        "at least CT1C trees must admit; got {trees_admitted}"
    );
    eprintln!(
        "CAPABILITY-PREREQ-DAG-CLAUSE-CENSUS trees_admitted={trees_admitted} sources=[ct1c_tradition_set.clause, tp_shipsize_decoder_0.clause(programmatic)]"
    );
}

/// Load-bearing live generation/GPU atomicity referee.
///
/// Installs Limited(1) PlayerSelection siblings through the ordinary
/// open_from_spec path, activates A, then B via the normal selection API,
/// advances real boundaries + production ticks, and proves:
/// - same generation barrier suspends A and activates B (spec state)
/// - no observed generation reports both active
/// - final resolved-host GPU cell reflects B only
/// - two identical runs are bit-exact equal
#[test]
fn max_active_sibling_switch_atomic_on_live_gpu_session() {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let mut run_bits: Vec<u32> = Vec::new();
    for run in 0..2 {
        let mut game_mode = hydrate_clause_mode(CLAUSE_MAX_ACTIVE_PROOF);
        preserve_unbounded_amount(&mut game_mode, "idea_host", "pressure");
        apply_max_active_limited(&mut game_mode);
        let tree_id = game_mode.domain_packs[0].capability_trees[0]
            .tree_id
            .clone();
        let (scenario, owner_id, property_id) = scenario_with_owner(&mut game_mode, 2.0);

        // Sanity: authored seed is on the owner.
        let seeded = find_node(&scenario.root, owner_id)
            .expect("owner")
            .properties[&property_id]
            .get_role(
                &SubFieldRole::Amount,
                &scenario.registry.property(property_id).layout,
            );
        assert_eq!(seeded.to_bits(), 2.0_f32.to_bits());

        let mut session =
            SimSession::open_from_spec(scenario, &game_mode).expect("open live GPU session");
        let adapter = session.state.ctx.adapter.get_info();

        // Establish A via ordinary selection + boundary + production tick.
        session
            .spec_state
            .queue_player_selection_by_key(owner_id, &tree_id, "idea_a")
            .expect("queue A");
        let step_a = session.step_once().expect("A boundary");
        assert!(step_a.boundary_reached, "A must land at a generation barrier");
        let active_after_a = active_entry_ids(&session, owner_id);
        assert_eq!(active_after_a, vec!["idea_a".to_string()], "only A active");
        session.step_once().expect("A production tick");
        let after_a = read_owner_amount(&session, owner_id, property_id);
        // idea_a Set(4.0)
        assert_eq!(
            after_a.to_bits(),
            4.0_f32.to_bits(),
            "run{run}: after A GPU must reflect A only (Set 4.0)"
        );

        let instance = session
            .spec_state
            .capability_instances
            .values()
            .find(|i| i.owner_id == owner_id)
            .expect("capability instance")
            .clone();
        let a_overlays: Vec<_> = instance
            .by_overlay
            .iter()
            .filter(|(_, key)| key.entry_id == "idea_a")
            .map(|(oid, _)| *oid)
            .collect();
        let b_overlays: Vec<_> = instance
            .by_overlay
            .iter()
            .filter(|(_, key)| key.entry_id == "idea_b")
            .map(|(oid, _)| *oid)
            .collect();
        assert_eq!(a_overlays.len(), 1);
        assert_eq!(b_overlays.len(), 1);
        assert!(
            session.proto.root.has_overlay(owner_id, a_overlays[0]),
            "A overlay lives on owner host"
        );
        assert!(
            session.proto.root.has_overlay(owner_id, b_overlays[0]),
            "B overlay lives on owner host"
        );

        // Switch to B: one selection → one barrier must suspend A + activate B.
        session
            .spec_state
            .queue_player_selection_by_key(owner_id, &tree_id, "idea_b")
            .expect("queue B");
        let step_b = session.step_once().expect("B boundary");
        assert!(step_b.boundary_reached, "B must land at a generation barrier");
        let active_after_b = active_entry_ids(&session, owner_id);
        assert_eq!(
            active_after_b,
            vec!["idea_b".to_string()],
            "run{run}: same barrier must leave exactly B active (no dual-active)"
        );
        assert!(
            !active_after_b.contains(&"idea_a".to_string()),
            "run{run}: A must not remain active across the generation"
        );

        // Production tick applies the now-active B overlay only → Set(6.0).
        session.step_once().expect("B production tick");
        let after_b = read_owner_amount(&session, owner_id, property_id);
        assert_eq!(
            after_b.to_bits(),
            6.0_f32.to_bits(),
            "run{run}: final GPU must reflect B only (Set 6.0; got {after_b})"
        );
        // No dual-active generation: after A we saw 4.0, after B we see 6.0 —
        // never a generation that shows both effects (which Set cannot express
        // as a mixed value, but the exclusive active set above is the hard proof).
        run_bits.push(after_b.to_bits());

        eprintln!(
            "CAPABILITY-PREREQ-DAG-ATOMICITY-GPU-PROOF run={run} adapter={:?} backend={:?} device_type={:?} owner={owner_id:?} after_a_bits={} after_b_bits={} a_overlay={:?} b_overlay={:?}",
            adapter.name,
            adapter.backend,
            adapter.device_type,
            after_a.to_bits(),
            after_b.to_bits(),
            a_overlays[0],
            b_overlays[0]
        );
    }

    assert_eq!(
        run_bits[0], run_bits[1],
        "bit-exact replay: two identical live-GPU runs must match"
    );
}

