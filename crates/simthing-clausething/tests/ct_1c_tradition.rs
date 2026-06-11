//! CT-1c: a ClauseScript-authored tradition set hydrates onto the
//! `capability_tree_v1` pattern — prereq DAG → threshold ordering, payload
//! activation — and runs through a real GPU session: the first
//! "designer writes Clausewitz, SimThing runs it" proof.

use std::collections::HashMap;

use simthing_clausething::{HydratedEntityPack, hydrate_entity_pack, parse_raw_document};
use simthing_core::{
    DimensionRegistry, Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource,
    PropertyTransformDelta, SimProperty, SimThing, SimThingId, SimThingKind, SubFieldRole,
    TransformOp,
};
use simthing_driver::{Scenario, SimSession, preview_install};
use simthing_gpu::SlotAllocator;
use simthing_spec::spec::capability::ActivationMode;
use simthing_spec::spec::domain_pack::DomainPackSpec;
use simthing_spec::{GameModeSpec, SpecVersion};

const CLAUSE_FIXTURE: &str = include_str!("fixtures/ct1c_tradition_set.clause");
const RON_BASELINE: &str = include_str!("fixtures/ct1c_tradition_set_baseline.ron");

fn hydrate_fixture() -> HydratedEntityPack {
    let document = parse_raw_document(CLAUSE_FIXTURE.as_bytes()).expect("parse tradition fixture");
    hydrate_entity_pack(&document).expect("hydrate tradition fixture")
}

fn canonical_json(pack: &DomainPackSpec) -> String {
    serde_json::to_string(pack).expect("serialize domain pack")
}

fn ct1c_scenario(max_days: u32) -> (Scenario, SimThingId) {
    let mut registry = DimensionRegistry::new();
    let _ = registry.register(SimProperty::simple("_placeholder", "seed", 0));
    let mut root = SimThing::new(SimThingKind::World, 0);
    let faction = SimThing::new(SimThingKind::Faction, 0);
    let faction_id = faction.id;
    root.add_child(faction);
    (
        Scenario {
            name: "ct1c_tradition".into(),
            ticks_per_day: 1,
            max_days,
            dt: 1.0,
            n_slots: 16,
            registry,
            root,
            shadow_seeds: Vec::new(),
            tick_patches: Vec::new(),
            install_targets: HashMap::new(),
        },
        faction_id,
    )
}

fn game_mode_with_pack(pack: DomainPackSpec) -> GameModeSpec {
    GameModeSpec {
        id: "ct1c_tradition".into(),
        display_name: "CT-1c Tradition Proof".into(),
        description: String::new(),
        spec_version: SpecVersion::default(),
        metadata: Default::default(),
        domain_packs: vec![pack],
        properties: Vec::new(),
        overlays: Vec::new(),
        capability_trees: Vec::new(),
        events: Vec::new(),
        resource_flow: None,
        resource_economy: None,
        resource_flow_execution_profile: Default::default(),
        region_fields: vec![],
        mapping_execution_profile: Default::default(),
    }
}

fn find_simthing_mut(node: &mut SimThing, target: SimThingId) -> Option<&mut SimThing> {
    if node.id == target {
        return Some(node);
    }
    for child in &mut node.children {
        if let Some(found) = find_simthing_mut(child, target) {
            return Some(found);
        }
    }
    None
}

fn count_suspended(node: &SimThing) -> usize {
    let mut count = node
        .overlays
        .iter()
        .filter(|o| matches!(o.lifecycle, OverlayLifecycle::Suspended { .. }))
        .count();
    for child in &node.children {
        count += count_suspended(child);
    }
    count
}

/// Drive research progress on the cloned tradition tree, mirroring the
/// session-integration seeding pattern (a permanent progress overlay plus a
/// compile-revision bump + GPU resync because the tree is mutated directly).
fn seed_progress(session: &mut SimSession, entry: &str, delta: f32) {
    let instance = session
        .spec_state
        .capability_instances
        .values()
        .next()
        .expect("installed tradition instance");
    let tree_id = instance.tree_thing_id;
    let cat_prop_id = session
        .proto
        .registry
        .id_of("traditions", "adaptability")
        .expect("traditions::adaptability registered by install");
    let overlay = Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Custom("research_progress".into()),
        source: OverlaySource::System,
        affects: vec![tree_id],
        transform: PropertyTransformDelta {
            property_id: cat_prop_id,
            sub_field_deltas: vec![(SubFieldRole::Named(entry.into()), TransformOp::Add(delta))],
        },
        lifecycle: OverlayLifecycle::Permanent,
    };
    find_simthing_mut(&mut session.proto.root, tree_id)
        .expect("cloned tradition tree in session root")
        .add_overlay(overlay);
    session.proto.bump_overlay_compile_revision_for_test();
    session
        .proto
        .initial_gpu_sync(&session.coord, &mut session.state);
}

fn activation_mode(session: &SimSession, entry_id: &str) -> Option<ActivationMode> {
    let state = session
        .spec_state
        .capability_states
        .values()
        .next()
        .expect("tradition tree state");
    state
        .activation_mode_by_entry
        .iter()
        .find(|(key, _)| key.entry_id == entry_id)
        .map(|(_, mode)| *mode)
}

fn active_entry_ids(session: &SimSession) -> Vec<String> {
    let state = session
        .spec_state
        .capability_states
        .values()
        .next()
        .expect("tradition tree state");
    let mut ids: Vec<String> = state
        .active_by_category
        .values()
        .flat_map(|entries| entries.iter().map(|key| key.entry_id.clone()))
        .collect();
    ids.sort();
    ids
}

// ── Authoring identity ────────────────────────────────────────────────────────

#[test]
fn hydrated_tradition_set_matches_ron_baseline() {
    let hydrated = hydrate_fixture();
    let baseline: DomainPackSpec = ron::from_str(RON_BASELINE).expect("parse RON baseline");
    assert_eq!(
        canonical_json(&hydrated.domain_pack),
        canonical_json(&baseline),
        "hydrated tradition set must match the hand-authored RON baseline"
    );
}

// ── Install shape (CPU only) ──────────────────────────────────────────────────

#[test]
fn tradition_tree_installs_per_faction_with_suspended_payloads() {
    let hydrated = hydrate_fixture();
    let (scenario, faction_id) = ct1c_scenario(1);
    let allocator = SlotAllocator::new();
    let preview = preview_install(
        &game_mode_with_pack(hydrated.domain_pack),
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
    )
    .expect("preview tradition install");

    assert_eq!(preview.state.capability_instances.len(), 1);
    let instance = preview
        .state
        .capability_instances
        .values()
        .next()
        .expect("instance");
    assert_eq!(
        instance.owner_id, faction_id,
        "tree installs on the faction"
    );
    assert_eq!(
        preview.state.capability_unlock_registrations.len(),
        3,
        "one GPU threshold registration per tradition"
    );
    assert_eq!(
        count_suspended(&preview.root),
        3,
        "one suspended payload overlay per tradition"
    );
    assert!(
        preview
            .registry
            .id_of("traditions", "adaptability")
            .is_some()
    );
    assert!(preview.registry.id_of("ct1c", "potency").is_some());

    let faction = {
        fn find(node: &SimThing, id: SimThingId) -> Option<&SimThing> {
            if node.id == id {
                return Some(node);
            }
            node.children.iter().find_map(|c| find(c, id))
        }
        find(&preview.root, faction_id).expect("faction in preview tree")
    };
    assert!(
        faction
            .children
            .iter()
            .any(|c| matches!(&c.kind, SimThingKind::Custom(k) if k == "tradition_tree")),
        "cloned tradition tree attaches under the faction"
    );
}

// ── The consumer runs: prereq DAG ordering + payload activation (GPU) ─────────

#[test]
fn tradition_prereq_dag_orders_activation_on_gpu() {
    let hydrated = hydrate_fixture();
    let (scenario, faction_id) = ct1c_scenario(8);
    // Probe GPU availability separately so a real install error fails the
    // test instead of masquerading as a missing adapter.
    if let Err(err) = SimSession::open(scenario.clone()) {
        eprintln!("skipping: no GPU session ({err})");
        return;
    }
    let mut session =
        SimSession::open_from_spec(scenario, &game_mode_with_pack(hydrated.domain_pack))
            .expect("open_from_spec with hydrated tradition set");

    // Stage 1: recycling's threshold fires while its prereq (adopt) is
    // unresearched — the entry must wait in OnPrereqMet, payload suspended.
    seed_progress(&mut session, "tr_adapt_recycling", 16.0);
    session.run(2).expect("stage 1 run");

    assert_eq!(
        activation_mode(&session, "tr_adapt_recycling"),
        Some(ActivationMode::OnPrereqMet),
        "recycling fired without its prereq and must wait"
    );
    assert_eq!(active_entry_ids(&session), Vec::<String>::new());
    assert_eq!(count_suspended(&session.proto.root), 3);

    // Stage 2: adopt crosses its threshold — adopt activates, and the
    // OnPrereqMet sweep activates recycling in dependency order. The
    // finisher's prereq is now met but its threshold never fired, so it
    // must stay suspended (threshold AND prereqs, not OR).
    seed_progress(&mut session, "tr_adapt_adopt", 11.0);
    session.run(3).expect("stage 2 run");

    assert_eq!(
        active_entry_ids(&session),
        vec![
            "tr_adapt_adopt".to_string(),
            "tr_adapt_recycling".to_string()
        ],
        "adopt + recycling active in dependency order; diagnostics: {:?}",
        session.spec_state.capability_diagnostics
    );
    assert_eq!(
        count_suspended(&session.proto.root),
        1,
        "finisher stays suspended: prereq met but threshold never fired"
    );
    assert!(
        session.spec_state.handler_errors.is_empty(),
        "handler errors: {:?}",
        session.spec_state.handler_errors
    );

    // Payload proof: both activated Owner-targeted payloads (Add 5 + Add 7)
    // now transform the faction's ct1c::potency on GPU ticks.
    let registry = &session.proto.registry;
    let potency_id = registry.id_of("ct1c", "potency").expect("ct1c::potency");
    let potency_col = registry
        .column_range(potency_id)
        .col_for_role(&SubFieldRole::Amount, &registry.property(potency_id).layout)
        .expect("potency amount col");
    let faction_slot = session
        .proto
        .allocator
        .slot_of(faction_id)
        .expect("faction slot");
    let values = session.state.read_values();
    let potency = values[faction_slot as usize * session.coord.n_dims() as usize + potency_col];
    assert!(
        potency >= 12.0,
        "activated payloads must raise faction potency (Add 5 + Add 7 per tick), got {potency}"
    );
}
