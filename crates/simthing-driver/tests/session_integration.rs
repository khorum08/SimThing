//! End-to-end GPU tests for `SimSession`. Skips cleanly when no adapter.

use std::collections::{HashMap, HashSet};
use std::io::BufReader;
use std::time::Instant;

use simthing_core::{
    Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource, PropertyTransformDelta,
    SubFieldRole, TransformOp,
};
use simthing_driver::{check_bench_ceiling, Scenario, SimSession, SpecSessionState};
use simthing_gpu::GpuContext;
use simthing_sim::{BoundaryDeltaEntry, ReplayDriver, ReplayReader};
use simthing_spec::{
    compile_property, ActivationMode, CapabilityCategorySpec, CapabilityEffectSpec, CapabilitySpec,
    CapabilityTreeBuilder, CapabilityTreeInstance, CapabilityTreeSpec, CapabilityTreeState,
    PropertySpec,
};

fn try_gpu() -> bool {
    GpuContext::new_blocking().is_ok()
}

fn run_bench_scenario(ron: &str) -> (f64, simthing_driver::RunSummary, String) {
    let scenario = Scenario::from_ron_str(ron).expect("scenario parse");
    let name = scenario.name.clone();
    let max_days = scenario.max_days;
    let mut session = SimSession::open(scenario).expect("session open");
    let started = Instant::now();
    let summary = session.run(max_days).expect("session run");
    (started.elapsed().as_secs_f64() * 1000.0, summary, name)
}

#[test]
fn rebellion_demo_ron_runs_fission_via_sim_session() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let ron = include_str!("../../../scenarios/rebellion_demo.ron");
    let scenario = Scenario::from_ron_str(ron).expect("scenario parse");
    let mut session = SimSession::open(scenario).expect("session open");

    let summary = session.run(4).expect("session run");

    assert_eq!(summary.boundaries_run, 4);
    assert_eq!(summary.ticks_run, 4);
    assert!(
        summary.fission_events >= 1,
        "rebellion demo should fission within 4 days, got {}",
        summary.fission_events
    );
    assert_eq!(
        session.proto.root.subtree_size(),
        4,
        "world + location + cohort + one fission child"
    );
    assert_eq!(session.proto.fission_lineage().len(), 1);
}

#[test]
fn record_rebellion_demo_replay_round_trips_structural_state() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("session.replay.ldjson");

    let ron = include_str!("../../../scenarios/rebellion_demo.ron");
    let scenario = Scenario::from_ron_str(ron).expect("scenario parse");
    let mut session = SimSession::open(scenario).expect("session open");

    let summary = session.record_to_path(&path, 4).expect("record session");
    assert_eq!(summary.frames_written, 4);
    assert!(summary.fission_events >= 1);

    let file = std::fs::File::open(&path).expect("replay file");
    let mut reader = ReplayReader::new(BufReader::new(file));
    let mut driver = ReplayDriver::from_snapshot(reader.read_snapshot().expect("snapshot"));

    let mut frames = 0u32;
    let mut entry_counts: HashMap<&'static str, u32> = HashMap::new();
    while let Some(frame) = reader.next_frame().expect("read frame") {
        for entry in &frame.entries {
            *entry_counts.entry(replay_entry_kind(entry)).or_default() += 1;
        }
        driver.apply_frame(frame);
        frames += 1;
    }

    assert_eq!(frames, 4);
    assert_eq!(driver.day, 4);
    assert_eq!(driver.root.subtree_size(), 4);
    assert_eq!(driver.fission_lineage.len(), 1);
    assert_eq!(
        driver.fission_lineage.len(),
        session.proto.fission_lineage().len()
    );
    assert!(
        entry_counts.get("FissionOccurred").copied().unwrap_or(0) >= 1,
        "expected FissionOccurred in replay log, got {entry_counts:?}"
    );
    assert!(
        entry_counts
            .get("FissionLineageAdded")
            .copied()
            .unwrap_or(0)
            >= 1,
        "expected FissionLineageAdded in replay log, got {entry_counts:?}"
    );
}

#[test]
fn spec_session_capability_unlock_activates_overlay_for_next_tick() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let mut registry = simthing_core::DimensionRegistry::new();
    let (power_id, _) = compile_property(
        &PropertySpec {
            id: "core_power".into(),
            namespace: "core".into(),
            name: "power".into(),
            display_name: "Power".into(),
            description: String::new(),
            sub_fields: Vec::new(),
        },
        &mut registry,
    )
    .expect("compile power property");

    let cap_spec = CapabilityTreeSpec {
        tree_id: "tech_tree".into(),
        tree_kind: "tech_tree".into(),
        owner_kind: "faction".into(),
        install: simthing_spec::InstallTargetSpec::faction_default(),
        categories: vec![CapabilityCategorySpec {
            property_namespace: "tech".into(),
            property_name: "propulsion".into(),
            display_name: "Propulsion".into(),
            tier: 0,
            max_active: None,
            entries: vec![CapabilitySpec {
                id: "chemical_drive".into(),
                display_name: "Chemical Drive".into(),
                description: String::new(),
                flavor_text: String::new(),
                research_cost: 10.0,
                activation: ActivationMode::Threshold,
                icon: String::new(),
                thumbnail: String::new(),
                card_image: String::new(),
                unlock_video: None,
                model_preview: None,
                prereqs: Vec::new(),
                unlocks_ship_components: Vec::new(),
                unlocks_buildings: Vec::new(),
                unlocks_units: Vec::new(),
                unlocks_weapons: Vec::new(),
                effects: vec![CapabilityEffectSpec {
                    targets_property: "core::power".into(),
                    sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(2.0))],
                    when_activated: OverlayLifecycle::Permanent,
                    effect_target: simthing_spec::EffectTarget::CapabilityTree,
                }],
            }],
        }],
    };
    let (mut built, _) =
        CapabilityTreeBuilder::build(&cap_spec, &mut registry).expect("capability build");
    let tree_id = built.tree.id;
    built
        .tree
        .add_property(power_id, registry.property(power_id).default_value());
    for overlay in &mut built.tree.overlays {
        overlay.affects = vec![tree_id];
    }

    let cap_category = built
        .definition
        .categories
        .values()
        .next()
        .expect("one capability category");
    let progress_overlay = Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Custom("research_progress".into()),
        source: OverlaySource::System,
        affects: vec![tree_id],
        transform: PropertyTransformDelta {
            property_id: cap_category.property_id,
            sub_field_deltas: vec![(
                SubFieldRole::Named("chemical_drive".into()),
                TransformOp::Add(11.0),
            )],
        },
        lifecycle: OverlayLifecycle::Permanent,
    };
    built.tree.add_overlay(progress_overlay);

    let scenario = Scenario {
        name: "capability_unlock_session".into(),
        ticks_per_day: 1,
        max_days: 2,
        dt: 0.0,
        n_slots: 8,
        registry,
        root: built.tree,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::new(),
    };
    let mut session = SimSession::open(scenario).expect("session open");
    let tree_slot = session.proto.allocator.slot_of(tree_id).expect("tree slot");

    let instance = CapabilityTreeInstance {
        owner_id: tree_id,
        definition_id: built.definition.id,
        tree_thing_id: tree_id,
        tree_slot,
        by_overlay: built.template_by_overlay.clone(),
        overlay_hosts: HashMap::new(),
    };
    let state = CapabilityTreeState {
        owner_id: tree_id,
        definition_id: built.definition.id,
        activation_mode_by_entry: Default::default(),
        active_by_category: Default::default(),
    };
    let mut spec_state = SpecSessionState::new();
    spec_state.add_capability_tree_instance(
        built.definition,
        instance,
        state,
        built.unlock_registrations,
    );
    session.install_spec_state(spec_state);

    let summary = session.run(2).expect("session run");
    assert_eq!(summary.boundaries_run, 2);
    assert!(
        !session.spec_state.capability_notifications.is_empty()
            || session.spec_state.capability_diagnostics.is_empty(),
        "unexpected capability diagnostics: {:?}",
        session.spec_state.capability_diagnostics
    );
    assert!(
        session.spec_state.handler_errors.is_empty(),
        "unexpected handler errors: {:?}",
        session.spec_state.handler_errors
    );

    let power_col = session
        .proto
        .registry
        .column_range(power_id)
        .col_for_role(
            &SubFieldRole::Amount,
            &session.proto.registry.property(power_id).layout,
        )
        .expect("power amount col");
    let values = session.state.read_values();
    let idx = tree_slot as usize * session.coord.n_dims() as usize + power_col;
    assert!(
        values[idx] >= 2.0,
        "activated capability overlay should affect next tick power, got {}",
        values[idx]
    );
}

#[test]
fn bench_stress_scenarios_within_ceiling() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    for (ron, label) in [
        (
            include_str!("../../../scenarios/intent_stress.ron"),
            "intent_stress",
        ),
        (
            include_str!("../../../scenarios/fission_stress.ron"),
            "fission_stress",
        ),
    ] {
        let (elapsed_ms, summary, name) = run_bench_scenario(ron);
        assert_eq!(name, label);
        check_bench_ceiling(&name, elapsed_ms, &summary).unwrap_or_else(|err| {
            panic!("{label} bench ceiling: {err} (elapsed_ms={elapsed_ms:.3})");
        });
    }
}

// ── O1: session installation tests ────────────────────────────────────────────

use simthing_core::{SimProperty, SimThing as CoreSimThing, SimThingKind};
use simthing_driver::{InstallError, SessionError};
use simthing_spec::{
    CapabilityTreeDefinition, GameModeSpec, InstallTargetSpec, SpecVersion,
};

fn make_capability_tree_spec(install: InstallTargetSpec) -> CapabilityTreeSpec {
    CapabilityTreeSpec {
        tree_id: "ideas".into(),
        tree_kind: "national_ideas".into(),
        owner_kind: "Faction".into(),
        install,
        categories: vec![CapabilityCategorySpec {
            property_namespace: "ideas".into(),
            property_name: "national".into(),
            display_name: "National".into(),
            tier: 0,
            max_active: None,
            entries: vec![CapabilitySpec {
                id: "focus".into(),
                display_name: "Focus".into(),
                description: String::new(),
                flavor_text: String::new(),
                research_cost: 1.0,
                activation: ActivationMode::PlayerSelection,
                icon: String::new(),
                thumbnail: String::new(),
                card_image: String::new(),
                unlock_video: None,
                model_preview: None,
                prereqs: Vec::new(),
                unlocks_ship_components: Vec::new(),
                unlocks_buildings: Vec::new(),
                unlocks_units: Vec::new(),
                unlocks_weapons: Vec::new(),
                effects: vec![CapabilityEffectSpec {
                    targets_property: "core::power".into(),
                    sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(1.0))],
                    when_activated: OverlayLifecycle::Permanent,
                    effect_target: simthing_spec::EffectTarget::CapabilityTree,
                }],
            }],
        }],
    }
}

fn make_game_mode_spec(install: InstallTargetSpec) -> GameModeSpec {
    GameModeSpec {
        id: "demo".into(),
        display_name: "Demo".into(),
        description: String::new(),
        spec_version: SpecVersion::default(),
        metadata: Default::default(),
        domain_packs: Vec::new(),
        properties: vec![PropertySpec {
            id: "core_power".into(),
            namespace: "core".into(),
            name: "power".into(),
            display_name: "Power".into(),
            description: String::new(),
            sub_fields: Vec::new(),
        }],
        overlays: Vec::new(),
        capability_trees: vec![make_capability_tree_spec(install)],
        events: Vec::new(),
    }
}

/// Game mode with a `Threshold` capability entry and `core::power` introduced
/// only via spec properties (not the base scenario registry).
fn make_threshold_unlock_game_mode() -> GameModeSpec {
    GameModeSpec {
        id: "threshold_unlock".into(),
        display_name: "Threshold Unlock".into(),
        description: String::new(),
        spec_version: SpecVersion::default(),
        metadata: Default::default(),
        domain_packs: Vec::new(),
        properties: vec![PropertySpec {
            id: "core_power".into(),
            namespace: "core".into(),
            name: "power".into(),
            display_name: "Power".into(),
            description: String::new(),
            sub_fields: Vec::new(),
        }],
        overlays: Vec::new(),
        capability_trees: vec![CapabilityTreeSpec {
            tree_id: "tech_tree".into(),
            tree_kind: "tech_tree".into(),
            owner_kind: "Faction".into(),
            install: InstallTargetSpec::AllOfKind {
                kind: "Faction".into(),
            },
            categories: vec![CapabilityCategorySpec {
                property_namespace: "tech".into(),
                property_name: "propulsion".into(),
                display_name: "Propulsion".into(),
                tier: 0,
                max_active: None,
                entries: vec![CapabilitySpec {
                    id: "chemical_drive".into(),
                    display_name: "Chemical Drive".into(),
                    description: String::new(),
                    flavor_text: String::new(),
                    research_cost: 10.0,
                    activation: ActivationMode::Threshold,
                    icon: String::new(),
                    thumbnail: String::new(),
                    card_image: String::new(),
                    unlock_video: None,
                    model_preview: None,
                    prereqs: Vec::new(),
                    unlocks_ship_components: Vec::new(),
                    unlocks_buildings: Vec::new(),
                    unlocks_units: Vec::new(),
                    unlocks_weapons: Vec::new(),
                    effects: vec![CapabilityEffectSpec {
                        targets_property: "core::power".into(),
                        sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(2.0))],
                        when_activated: OverlayLifecycle::Permanent,
                        effect_target: simthing_spec::EffectTarget::CapabilityTree,
                    }],
                }],
            }],
        }],
        events: Vec::new(),
    }
}

fn find_simthing_mut<'a>(
    node: &'a mut CoreSimThing,
    id: simthing_core::SimThingId,
) -> Option<&'a mut CoreSimThing> {
    if node.id == id {
        return Some(node);
    }
    for child in &mut node.children {
        if let Some(found) = find_simthing_mut(child, id) {
            return Some(found);
        }
    }
    None
}

/// Seed research progress on the cloned capability tree after `open_from_spec`.
/// Does not use `install_spec_state`; only post-open test setup.
fn seed_research_progress_after_open(session: &mut SimSession, progress_delta: f32) {
    let instance = session
        .spec_state
        .capability_instances
        .values()
        .next()
        .expect("one capability instance after install");
    let cat_prop_id = session
        .proto
        .registry
        .id_of("tech", "propulsion")
        .expect("spec-installed category property");
    let tree_id = instance.tree_thing_id;
    let progress_overlay = Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Custom("research_progress".into()),
        source: OverlaySource::System,
        affects: vec![tree_id],
        transform: PropertyTransformDelta {
            property_id: cat_prop_id,
            sub_field_deltas: vec![(
                SubFieldRole::Named("chemical_drive".into()),
                TransformOp::Add(progress_delta),
            )],
        },
        lifecycle: OverlayLifecycle::Permanent,
    };
    find_simthing_mut(&mut session.proto.root, tree_id)
        .expect("cloned capability tree in session root")
        .add_overlay(progress_overlay);
    session
        .proto
        .initial_gpu_sync(&session.coord, &mut session.state);
}

fn scenario_with_factions(n_factions: usize, n_slots: u32) -> (Scenario, Vec<simthing_core::SimThingId>) {
    let mut registry = simthing_core::DimensionRegistry::new();
    // Reserve a placeholder property so the registry isn't empty; `core::power`
    // is added by the spec at install time.
    let _ = registry.register(SimProperty::simple("_placeholder", "seed", 0));

    let mut world = CoreSimThing::new(SimThingKind::World, 0);
    let mut faction_ids = Vec::with_capacity(n_factions);
    for _ in 0..n_factions {
        let faction = CoreSimThing::new(SimThingKind::Faction, 0);
        faction_ids.push(faction.id);
        world.add_child(faction);
    }

    let scenario = Scenario {
        name: "install_test".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 0.0,
        n_slots,
        registry,
        root: world,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::new(),
    };
    (scenario, faction_ids)
}

fn count_tree_children(root: &CoreSimThing, owner_id: simthing_core::SimThingId) -> usize {
    fn find<'a>(node: &'a CoreSimThing, owner: simthing_core::SimThingId) -> Option<&'a CoreSimThing> {
        if node.id == owner {
            return Some(node);
        }
        for c in &node.children {
            if let Some(n) = find(c, owner) {
                return Some(n);
            }
        }
        None
    }
    find(root, owner_id)
        .map(|owner| {
            owner
                .children
                .iter()
                .filter(|c| matches!(c.kind, SimThingKind::Custom(ref k) if k == "national_ideas"))
                .count()
        })
        .unwrap_or(0)
}

/// E2E acceptance test for O1 + O1b: threshold unlock via `open_from_spec`.
///
/// Exercises the full pipeline: spec install (clone tree, re-stamp overlay ids,
/// seed effect-target properties), GPU integration of the seeded progress
/// overlay, threshold firing, handler emits ActivateOverlay with per-clone
/// overlay ids resolved from `instance.by_overlay`, and GPU Pass 3 applies the
/// activated transform to the cloned tree's slot on the next tick.
#[test]
fn open_from_spec_capability_unlock_activates_overlay_for_next_tick() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let game_mode = make_threshold_unlock_game_mode();
    let (mut scenario, _faction_ids) = scenario_with_factions(1, 16);
    scenario.max_days = 2;
    assert!(
        scenario.registry.id_of("core", "power").is_none(),
        "base scenario must not define core::power — property comes from GameModeSpec"
    );
    assert!(
        scenario.registry.id_of("tech", "propulsion").is_none(),
        "base scenario must not define tech::propulsion — category comes from spec install"
    );

    let mut session =
        SimSession::open_from_spec(scenario, &game_mode).expect("open_from_spec");

    assert!(
        session
            .proto
            .registry
            .id_of("core", "power")
            .is_some(),
        "open_from_spec must register spec properties before run"
    );
    assert_eq!(session.spec_state.capability_instances.len(), 1);
    assert_eq!(
        session.spec_state.capability_unlock_registrations.len(),
        1,
        "Threshold entry must produce unlock registration via open_from_spec install"
    );

    let instance = session
        .spec_state
        .capability_instances
        .values()
        .next()
        .expect("installed instance");
    let tree_slot = instance.tree_slot;

    seed_research_progress_after_open(&mut session, 11.0);

    let summary = session.run(2).expect("session run");
    assert_eq!(summary.boundaries_run, 2);

    let instance = session
        .spec_state
        .capability_instances
        .values()
        .next()
        .expect("installed instance");
    let definition = session
        .spec_state
        .capability_definitions
        .get(&instance.definition_id)
        .expect("definition");
    let entry = definition.entries.values().next().expect("one entry");
    let cloned_tree = find_simthing_mut(&mut session.proto.root, instance.tree_thing_id)
        .expect("cloned tree");
    let cloned_overlay_ids: HashSet<OverlayId> =
        cloned_tree.overlays.iter().map(|o| o.id).collect();
    for template_id in &entry.overlay_ids {
        assert!(
            !cloned_overlay_ids.contains(template_id),
            "install re-stamps overlay ids on clone; definition still holds template ids"
        );
    }

    assert!(
        session.spec_state.handler_errors.is_empty(),
        "unexpected handler errors: {:?}",
        session.spec_state.handler_errors
    );
    assert!(
        session.spec_state.capability_diagnostics.is_empty(),
        "unexpected capability diagnostics: {:?}",
        session.spec_state.capability_diagnostics
    );

    let power_id = session
        .proto
        .registry
        .id_of("core", "power")
        .expect("core::power");
    let power_col = session
        .proto
        .registry
        .column_range(power_id)
        .col_for_role(
            &SubFieldRole::Amount,
            &session.proto.registry.property(power_id).layout,
        )
        .expect("power amount col");
    let values = session.state.read_values();
    let idx = tree_slot as usize * session.coord.n_dims() as usize + power_col;
    assert!(
        values[idx] >= 2.0,
        "threshold unlock via open_from_spec should activate overlay for next tick; \
         expected core::power >= 2.0 at cloned tree slot, got {} \
         (registry cols={}, coord n_dims={})",
        values[idx],
        session.proto.registry.total_columns,
        session.coord.n_dims(),
    );
}

/// EffectTarget ADR acceptance: an `Owner`-targeted (v1 default) effect
/// modifies the **owner's** property slot when the unlock fires, not the
/// cloned tree's slot. Mirror of the `CapabilityTree` test above with
/// `effect_target: Owner` and assertion against `owner_slot`.
#[test]
fn open_from_spec_owner_targeted_effect_modifies_owner_slot() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    // Build a game mode identical to `make_threshold_unlock_game_mode`
    // but with `effect_target: Owner` on the single effect.
    let mut game_mode = make_threshold_unlock_game_mode();
    for tree in &mut game_mode.capability_trees {
        for cat in &mut tree.categories {
            for entry in &mut cat.entries {
                for effect in &mut entry.effects {
                    effect.effect_target = simthing_spec::EffectTarget::Owner;
                }
            }
        }
    }

    let (mut scenario, faction_ids) = scenario_with_factions(1, 16);
    scenario.max_days = 2;
    let owner_id = faction_ids[0];

    let mut session =
        SimSession::open_from_spec(scenario, &game_mode).expect("open_from_spec");
    let instance = session
        .spec_state
        .capability_instances
        .values()
        .next()
        .expect("installed instance");
    let tree_slot = instance.tree_slot;
    let owner_slot = session
        .proto
        .allocator
        .slot_of(owner_id)
        .expect("owner slot");
    assert_ne!(tree_slot, owner_slot, "owner and clone must occupy distinct slots");

    seed_research_progress_after_open(&mut session, 11.0);
    let summary = session.run(2).expect("session run");
    assert_eq!(summary.boundaries_run, 2);

    let power_id = session.proto.registry.id_of("core", "power").unwrap();
    let power_col = session
        .proto
        .registry
        .column_range(power_id)
        .col_for_role(
            &SubFieldRole::Amount,
            &session.proto.registry.property(power_id).layout,
        )
        .expect("power amount col");
    let values = session.state.read_values();
    let n_dims = session.coord.n_dims() as usize;
    let owner_idx = owner_slot as usize * n_dims + power_col;
    let clone_idx = tree_slot as usize * n_dims + power_col;
    assert!(
        values[owner_idx] >= 2.0,
        "Owner-targeted effect must apply to the owner's slot; got {} at owner_slot={}",
        values[owner_idx],
        owner_slot,
    );
    assert_eq!(
        values[clone_idx], 0.0,
        "Owner-targeted effect must NOT apply to the clone's slot; got {} at clone_slot={}",
        values[clone_idx], tree_slot,
    );
}

#[test]
fn open_from_spec_installs_capability_tree_for_each_matching_owner() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }
    let (scenario, faction_ids) = scenario_with_factions(1, 8);
    let owner_id = faction_ids[0];
    let game_mode = make_game_mode_spec(InstallTargetSpec::AllOfKind { kind: "Faction".into() });

    let session = SimSession::open_from_spec(scenario, &game_mode).expect("open_from_spec");

    // Structural: faction has exactly one cloned tree child.
    assert_eq!(
        count_tree_children(&session.proto.root, owner_id),
        1,
        "faction should have one cloned capability tree child"
    );

    // Spec state: one capability instance owned by the faction.
    assert_eq!(session.spec_state.capability_instances.len(), 1);
    let instance = session
        .spec_state
        .capability_instances
        .values()
        .next()
        .expect("one instance");
    assert_eq!(instance.owner_id, owner_id);
    assert_ne!(
        instance.tree_thing_id, owner_id,
        "cloned tree id must differ from owner id"
    );

    // by_overlay lives on the instance, with the same number of entries as
    // overlays compiled from the tree spec's effects (one entry × one effect).
    assert_eq!(
        instance.by_overlay.len(),
        1,
        "instance by_overlay should hold one re-stamped overlay id"
    );

    // No threshold registrations because the only entry is PlayerSelection.
    assert!(
        session.spec_state.capability_unlock_registrations.is_empty(),
        "PlayerSelection entries produce no unlock registrations"
    );
}

#[test]
fn open_from_spec_installs_separate_tree_per_owner() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }
    let (scenario, faction_ids) = scenario_with_factions(2, 16);
    let game_mode = make_game_mode_spec(InstallTargetSpec::AllOfKind { kind: "Faction".into() });

    let session = SimSession::open_from_spec(scenario, &game_mode).expect("open_from_spec");

    // One tree per faction.
    for owner_id in &faction_ids {
        assert_eq!(
            count_tree_children(&session.proto.root, *owner_id),
            1,
            "each faction should get its own cloned tree"
        );
    }

    // Two distinct instances with distinct tree_thing_ids.
    assert_eq!(session.spec_state.capability_instances.len(), 2);
    let tree_ids: HashSet<simthing_core::SimThingId> = session
        .spec_state
        .capability_instances
        .values()
        .map(|i| i.tree_thing_id)
        .collect();
    assert_eq!(tree_ids.len(), 2, "tree_thing_ids must be distinct per owner");

    // by_overlay maps must use distinct OverlayIds per instance.
    let mut all_overlay_ids: HashSet<OverlayId> = HashSet::new();
    for instance in session.spec_state.capability_instances.values() {
        for overlay_id in instance.by_overlay.keys() {
            assert!(
                all_overlay_ids.insert(*overlay_id),
                "OverlayIds must be re-stamped per clone — found duplicate {:?}",
                overlay_id
            );
        }
    }
}

#[test]
fn open_from_spec_scenario_listed_target() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }
    let (mut scenario, faction_ids) = scenario_with_factions(3, 16);
    let target_owner = faction_ids[1]; // middle faction
    scenario
        .install_targets
        .insert("player_faction".into(), vec![target_owner]);
    let game_mode = make_game_mode_spec(InstallTargetSpec::ScenarioListed {
        target_id: "player_faction".into(),
    });

    let session = SimSession::open_from_spec(scenario, &game_mode).expect("open_from_spec");

    // Only the listed faction gets a tree.
    assert_eq!(
        count_tree_children(&session.proto.root, target_owner),
        1,
        "listed faction should get a tree"
    );
    for owner_id in &faction_ids {
        if *owner_id == target_owner {
            continue;
        }
        assert_eq!(
            count_tree_children(&session.proto.root, *owner_id),
            0,
            "unlisted faction must not get a tree"
        );
    }
    assert_eq!(session.spec_state.capability_instances.len(), 1);
}

#[test]
fn open_from_spec_no_matching_owners_is_error() {
    // GPU not strictly required — open_from_spec will fail before the
    // install path mutates GPU state — but the surrounding `Self::open`
    // does need GPU.  Skip cleanly when absent.
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }
    // World-only scenario, no Faction children.
    let (scenario, _) = scenario_with_factions(0, 4);
    let game_mode = make_game_mode_spec(InstallTargetSpec::AllOfKind { kind: "Faction".into() });

    let result = SimSession::open_from_spec(scenario, &game_mode);
    match result {
        Err(SessionError::Install(InstallError::NoMatchingOwners { tree_id, .. })) => {
            assert_eq!(tree_id, "ideas");
        }
        other => panic!("expected NoMatchingOwners install error, got {:?}", other.map(|_| ())),
    }
}

#[test]
fn open_from_spec_legacy_install_spec_state_still_works() {
    // Confirms that the legacy explicit-install path (used by every PR 5/11
    // integration test) compiles and runs cleanly alongside open_from_spec.
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }
    let (scenario, _) = scenario_with_factions(1, 8);
    let session = SimSession::open(scenario).expect("open");
    // install_spec_state with a fresh, empty state — no spec runtime, but the
    // entry point still validates and does its initial_gpu_sync.
    let mut session = session;
    session.install_spec_state(SpecSessionState::new());
    assert!(session.spec_state.capability_instances.is_empty());
    assert!(session.spec_state.handler_errors.is_empty());
}

#[test]
fn capability_tree_by_overlay_lives_on_instance_not_definition() {
    // Compile-time sanity check that the by_overlay migration landed in the
    // right place. Asserting via a small fake-build keeps this test free of
    // GPU dependencies.
    let mut registry = simthing_core::DimensionRegistry::new();
    let _ = registry.register(SimProperty::simple("core", "power", 0));
    let spec = make_capability_tree_spec(InstallTargetSpec::faction_default());
    let (out, _) = CapabilityTreeBuilder::build(&spec, &mut registry).expect("build");

    // The build output exposes the template by_overlay separately (used by
    // the install module to re-stamp per clone).
    assert!(
        !out.template_by_overlay.is_empty(),
        "builder should populate template_by_overlay"
    );

    // The definition no longer carries by_overlay: this is a type assertion,
    // not a runtime one. If by_overlay were re-added to CapabilityTreeDefinition
    // the type would not compile against this constructor literal.
    fn assert_definition_shape(_def: &CapabilityTreeDefinition) {
        // Compile-time: this function body intentionally references only fields
        // that the migrated definition exposes. Any future re-addition of
        // by_overlay to the definition would produce dead code, not a failure;
        // we therefore also assert structurally below.
    }
    assert_definition_shape(&out.definition);

    // Structural: the template by_overlay refers to overlays that live on
    // `out.tree`, not on `out.definition`. Definition holds entries only.
    for overlay_id in out.template_by_overlay.keys() {
        let on_tree = out.tree.overlays.iter().any(|o| o.id == *overlay_id);
        assert!(
            on_tree,
            "template overlay ids must be present on the tree, not just on the definition"
        );
    }
}

fn replay_entry_kind(entry: &BoundaryDeltaEntry) -> &'static str {
    match entry {
        BoundaryDeltaEntry::OverlayAttached { .. } => "OverlayAttached",
        BoundaryDeltaEntry::OverlayDissolved { .. } => "OverlayDissolved",
        BoundaryDeltaEntry::OverlayActivated { .. } => "OverlayActivated",
        BoundaryDeltaEntry::OverlaySuspended { .. } => "OverlaySuspended",
        BoundaryDeltaEntry::SimThingAdded { .. } => "SimThingAdded",
        BoundaryDeltaEntry::SimThingRemoved { .. } => "SimThingRemoved",
        BoundaryDeltaEntry::DimensionAdded { .. } => "DimensionAdded",
        BoundaryDeltaEntry::FissionOccurred { .. } => "FissionOccurred",
        BoundaryDeltaEntry::FusionOccurred { .. } => "FusionOccurred",
        BoundaryDeltaEntry::PropertyExpired { .. } => "PropertyExpired",
        BoundaryDeltaEntry::SimThingReparented { .. } => "SimThingReparented",
        BoundaryDeltaEntry::VelocityAlert { .. } => "VelocityAlert",
        BoundaryDeltaEntry::AggregateAlert { .. } => "AggregateAlert",
        BoundaryDeltaEntry::FissionLineageAdded { .. } => "FissionLineageAdded",
        BoundaryDeltaEntry::FissionLineageRemoved { .. } => "FissionLineageRemoved",
    }
}

/// S5 follow-up acceptance: when fission clones a capability subtree, the
/// driver registers a new `CapabilityTreeInstance` + threshold registrations
/// for the spawned owner. Without this, unlocks on the cloned tree never
/// fire because no `CapabilityUnlockRegistration` targets the clone's
/// `sim_thing_id`.
#[test]
fn fission_cloned_capability_subtree_registers_new_instance_and_thresholds() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    // Build a scenario with one faction carrying a loyalty fission template
    // that clones tech_tree containers. The capability tree itself is
    // installed by `open_from_spec` — not pre-attached to the faction.
    let mut registry = simthing_core::DimensionRegistry::new();
    let mut loyalty = SimProperty::simple("core", "loyalty", 0);
    loyalty.intensity_behavior = Some(simthing_core::IntensityBehavior::default());
    loyalty.fission_templates = vec![simthing_core::FissionThreshold {
        sub_field: SubFieldRole::Amount,
        threshold: 0.3,
        direction: simthing_core::Direction::Falling,
        template: simthing_core::FissionTemplate {
            child_kind: simthing_core::SimThingKindTag::Faction,
            fusion_intensity_threshold: 0.8,
            fusion_scar_coefficient: 0.05,
            resolution_label: "schism".into(),
            clone_capability_children: true,
            capability_container_kinds: vec!["tech_tree".into()],
        },
        secondary: None,
    }];
    let loyalty_pid = registry.register(loyalty);
    let layout = registry.property(loyalty_pid).layout.clone();
    let amount_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
    let vel_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();

    let mut faction = CoreSimThing::new(SimThingKind::Faction, 0);
    let mut faction_loyalty =
        simthing_core::PropertyValue::from_layout(&registry.property(loyalty_pid).layout);
    faction_loyalty.data[amount_off] = 0.5;
    faction_loyalty.data[vel_off] = -0.21;
    faction.add_property(loyalty_pid, faction_loyalty);
    let original_faction_id = faction.id;

    let mut location = CoreSimThing::new(SimThingKind::Location, 0);
    location.add_child(faction);
    let mut world = CoreSimThing::new(SimThingKind::World, 0);
    world.add_child(location);

    // Seed loyalty on the faction so the threshold fires after a few ticks.
    let _ = (amount_off, vel_off); // offsets only used for ShadowSeed below
    let scenario = Scenario {
        name: "s5_followup".into(),
        ticks_per_day: 1,
        max_days: 8,
        dt: 0.5,
        n_slots: 32,
        registry,
        root: world,
        shadow_seeds: vec![simthing_driver::ShadowSeed {
            thing_id: original_faction_id,
            namespace: "core".into(),
            name: "loyalty".into(),
            amount: 0.5,
            velocity: -0.21,
        }],
        tick_patches: Vec::new(),
        install_targets: HashMap::new(),
    };

    // Capability tree authored against the same tech_tree kind that fission
    // clones. `open_from_spec` installs a fresh tech_tree on the existing
    // faction; fission later clones it onto the spawned faction.
    let game_mode = make_threshold_unlock_game_mode();
    assert_eq!(game_mode.capability_trees[0].tree_kind, "tech_tree");

    let mut session = SimSession::open_from_spec(scenario, &game_mode).expect("open_from_spec");
    assert_eq!(
        session.spec_state.capability_instances.len(),
        1,
        "install should produce one instance for the original faction"
    );
    let baseline_unlock_count = session.spec_state.capability_unlock_registrations.len();
    assert!(baseline_unlock_count > 0, "threshold-mode entry should register an unlock");

    // Run up to 8 boundaries — the loyalty fission threshold fires within
    // a couple of ticks given the initial velocity.
    let summary = session.run(8).expect("session run");
    assert!(
        summary.fission_events >= 1,
        "loyalty fission must fire (got {} events)",
        summary.fission_events
    );

    // S5 follow-up assertion: spec_state grew an instance for the spawned
    // owner, the new instance points at the cloned tree, and threshold
    // registrations cover both the original and the clone.
    assert!(
        session.spec_state.capability_instances.len() >= 2,
        "expected ≥2 capability instances after fission (original + clone), got {}",
        session.spec_state.capability_instances.len()
    );
    let new_instances: Vec<_> = session
        .spec_state
        .capability_instances
        .values()
        .filter(|inst| inst.owner_id != original_faction_id)
        .collect();
    assert!(
        !new_instances.is_empty(),
        "at least one CapabilityTreeInstance must have a non-original owner"
    );
    for new_inst in &new_instances {
        assert_ne!(new_inst.tree_thing_id, original_faction_id);
        assert!(
            session
                .spec_state
                .capability_unlock_registrations
                .iter()
                .any(|reg| reg.sim_thing_id == new_inst.tree_thing_id),
            "fission-cloned tree {:?} must have at least one threshold registration",
            new_inst.tree_thing_id,
        );
        assert!(
            !new_inst.by_overlay.is_empty(),
            "fission-cloned instance's by_overlay must be populated from overlay_id_pairs"
        );
    }
}

/// O4 acceptance: a scripted event spec authoring
/// `install: AllOfKind { kind: "Faction" }` produces one
/// `ScriptedEventInstance` per faction at install time, with independent
/// owner ids and slots — the v0 PR 11 session-global behavior has been
/// replaced by the per-owner model in
/// `docs/adr/scripted_event_scope_model.md`.
#[test]
fn open_from_spec_installs_one_scripted_event_instance_per_faction() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }
    let (scenario, faction_ids) = scenario_with_factions(2, 8);

    // Game mode with no capability trees (orthogonal to this test) and one
    // event authored against `AllOfKind { kind: "Faction" }`. The event is
    // a Predicate(True) trigger with cooldown 5 — predicate triggers don't
    // need GPU threshold registrations, so this exercises only the install +
    // per-instance handler dispatch paths.
    let event_spec = simthing_spec::EventSpec {
        id: "tick_marker".into(),
        trigger: simthing_spec::TriggerSpec::Predicate {
            predicate: simthing_spec::ScriptPredicate::True,
        },
        effects: Vec::new(),
        cooldown: Some(simthing_spec::CooldownSpec { ticks: 5 }),
        priority: simthing_spec::EventPriority::Normal,
        install: InstallTargetSpec::AllOfKind {
            kind: "Faction".into(),
        },
    };
    let game_mode = simthing_spec::GameModeSpec {
        id: "o4_demo".into(),
        display_name: "O4 Demo".into(),
        description: String::new(),
        spec_version: SpecVersion::default(),
        metadata: Default::default(),
        domain_packs: Vec::new(),
        properties: Vec::new(),
        overlays: Vec::new(),
        capability_trees: Vec::new(),
        events: vec![event_spec],
    };

    let session = SimSession::open_from_spec(scenario, &game_mode).expect("open_from_spec");

    // One definition, two per-owner instances.
    assert_eq!(
        session.spec_state.scripted_event_definitions.len(),
        1,
        "one definition for one event spec"
    );
    assert_eq!(
        session.spec_state.scripted_event_instances.len(),
        2,
        "AllOfKind Faction with 2 factions ⇒ 2 instances"
    );

    // Each instance keys on a distinct faction; slots match the allocator.
    let owners: HashSet<_> = session
        .spec_state
        .scripted_event_instances
        .values()
        .map(|inst| inst.key.owner_id)
        .collect();
    for fid in &faction_ids {
        assert!(owners.contains(fid), "instance must exist for faction {:?}", fid);
    }
    for inst in session.spec_state.scripted_event_instances.values() {
        let expected_slot = session
            .proto
            .allocator
            .slot_of(inst.key.owner_id)
            .expect("owner allocated");
        assert_eq!(inst.current_slot, expected_slot);
        assert_eq!(inst.cooldown_remaining, 0, "fresh install starts ready");
    }
}

// ── O2: Replay v3 spec snapshot + delta round-trip ────────────────────────────

/// O2 acceptance: record a session that produces a capability threshold
/// unlock (which mutates `active_by_category` and emits an unlock-side
/// ActivateOverlay), then reopen the replay via `open_replay_with_spec`,
/// apply structural frames + spec deltas, and assert the post-replay
/// `SpecSessionState` is field-equivalent to the recorded final state.
///
/// Exercises the full v3 pipeline:
/// - `collect_spec_snapshot` on recording start
/// - per-frame `diff_and_emit` after the spec hook runs
/// - `ReplayWriter::write_extra` for the `spec_snapshot` line
/// - `read_spec_replay_file` decoding both record kinds
/// - `apply_spec_snapshot` against a freshly-installed session
/// - `apply_spec_delta` over each frame's `spec_entries`
/// - logical-key resolution: snapshots reference `tree_id`/`event_id`, never
///   raw `OverlayId`.
#[test]
fn record_and_replay_with_spec_round_trips_capability_state() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    use simthing_driver::{
        apply_spec_delta, apply_spec_snapshot, open_replay_with_spec, read_spec_replay_file,
        SpecDelta,
    };

    // ── Record ────────────────────────────────────────────────────────────
    let game_mode = make_threshold_unlock_game_mode();
    let (mut scenario, _) = scenario_with_factions(1, 16);
    scenario.max_days = 2;

    let mut session =
        SimSession::open_from_spec(scenario.clone(), &game_mode).expect("open_from_spec");
    seed_research_progress_after_open(&mut session, 11.0);

    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("o2_round_trip.replay.ldjson");

    let summary = session.record_to_path(&path, 2).expect("record session");
    assert_eq!(summary.boundaries_run, 2);

    // Capture the live post-record spec state for comparison, keyed by
    // logical ids (owner_id + tree_logical_id) since process-local atomic
    // ids (CapabilityTreeDefinitionId, tree_thing_id) differ across the
    // second install run inside open_replay_with_spec.
    let live_states: HashMap<(simthing_core::SimThingId, String), CapabilityTreeState> = session
        .spec_state
        .capability_states
        .iter()
        .map(|(key, st)| {
            let logical = session
                .spec_state
                .capability_definitions
                .get(&key.definition_id)
                .map(|d| d.tree_id.clone())
                .unwrap_or_default();
            ((key.owner_id, logical), st.clone())
        })
        .collect();
    let live_cooldowns: HashMap<(simthing_core::SimThingId, String), u32> = session
        .spec_state
        .scripted_event_instances
        .iter()
        .map(|(k, v)| ((k.owner_id, k.event_id.0.clone()), v.cooldown_remaining))
        .collect();

    // ── Read the replay file: sanity-check format extensions ──────────────
    let loaded = read_spec_replay_file(&path).expect("read replay");
    assert!(
        loaded.spec_snapshot.is_some(),
        "open_from_spec session must emit a spec_snapshot line"
    );
    let total_spec_deltas: usize = loaded.frames.iter().map(|(_, d)| d.len()).sum();
    assert!(
        total_spec_deltas > 0,
        "threshold unlock must emit at least one SpecDelta (CapabilityActiveSetChanged)"
    );

    let saw_active_set_changed = loaded
        .frames
        .iter()
        .flat_map(|(_, d)| d.iter())
        .any(|d| matches!(d, SpecDelta::CapabilityActiveSetChanged { .. }));
    assert!(
        saw_active_set_changed,
        "unlock must produce CapabilityActiveSetChanged; got {:?}",
        loaded
            .frames
            .iter()
            .flat_map(|(_, d)| d.iter())
            .collect::<Vec<_>>()
    );

    // No raw OverlayIds should appear in the serialized deltas (logical-key
    // invariant). The JSON for each spec_entry must not contain an
    // `overlay_id` field at top level (CapabilityNotification is purely
    // logical, no overlay ids; other variants don't carry overlay ids).
    for (_, deltas_json) in loaded
        .frames
        .iter()
        .map(|(f, _)| (f.day, &f.spec_entries))
    {
        for v in deltas_json {
            let s = v.to_string();
            assert!(
                !s.contains("overlay_id"),
                "spec_entry must not serialize raw OverlayId (logical-key invariant): {s}"
            );
        }
    }

    // ── Replay ────────────────────────────────────────────────────────────
    let (mut replay_session, mut replay_driver, frames) =
        open_replay_with_spec(&path, &game_mode, scenario).expect("open_replay_with_spec");

    // open_replay_with_spec already applied the spec snapshot (which is
    // empty in this test since recording started at day 0 with no
    // cooldowns and no active sets yet). Idempotency check:
    if let Some(ss) = &loaded.spec_snapshot {
        apply_spec_snapshot(&mut replay_session.spec_state, ss).expect("snapshot apply idempotent");
    }

    for (frame, deltas) in frames {
        replay_driver.apply_frame(frame);
        for delta in &deltas {
            apply_spec_delta(&mut replay_session.spec_state, delta)
                .expect("spec delta applies cleanly");
        }
    }

    // ── Assert field-equivalence (logical-key matched) ────────────────────
    assert_eq!(
        replay_session.spec_state.capability_states.len(),
        live_states.len(),
        "post-replay must have same number of capability states as live session"
    );
    // Index the replay session's states by (owner_id, tree_logical_id) so
    // we can match across the fresh atomic ids handed out by the second
    // install.
    let replay_states: HashMap<(simthing_core::SimThingId, String), CapabilityTreeState> =
        replay_session
            .spec_state
            .capability_states
            .iter()
            .map(|(key, st)| {
                let logical = replay_session
                    .spec_state
                    .capability_definitions
                    .get(&key.definition_id)
                    .map(|d| d.tree_id.clone())
                    .unwrap_or_default();
                ((key.owner_id, logical), st.clone())
            })
            .collect();

    for (logical_key, live_state) in &live_states {
        let replayed = replay_states.get(logical_key).unwrap_or_else(|| {
            panic!(
                "post-replay missing capability state for {logical_key:?} \
                 (live={:?}, replay={:?})",
                live_states.keys().collect::<Vec<_>>(),
                replay_states.keys().collect::<Vec<_>>(),
            )
        });
        assert_eq!(
            replayed.activation_mode_by_entry, live_state.activation_mode_by_entry,
            "activation_mode_by_entry must round-trip for {logical_key:?}"
        );
        assert_eq!(
            replayed.active_by_category, live_state.active_by_category,
            "active_by_category must round-trip for {logical_key:?} \
             (live={:?}, replay={:?})",
            live_state.active_by_category, replayed.active_by_category
        );
    }

    // Scripted-event cooldowns also round-trip (zero in this fixture since
    // there are no scripted events; assertion structure stays useful when
    // the fixture is extended).
    let replay_cooldowns: HashMap<(simthing_core::SimThingId, String), u32> = replay_session
        .spec_state
        .scripted_event_instances
        .iter()
        .map(|(k, v)| ((k.owner_id, k.event_id.0.clone()), v.cooldown_remaining))
        .collect();
    for (key, live_cd) in &live_cooldowns {
        let replayed = replay_cooldowns.get(key).copied().unwrap_or(0);
        assert_eq!(replayed, *live_cd, "cooldown round-trips for {key:?}");
    }
}

/// O2: forward compatibility — a sim-only consumer (`ReplayReader`) opening
/// a v3 replay must skip the `spec_snapshot` line silently and still read
/// every structural frame.
#[test]
fn replay_reader_skips_spec_snapshot_line_for_sim_only_consumer() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let game_mode = make_threshold_unlock_game_mode();
    let (mut scenario, _) = scenario_with_factions(1, 16);
    scenario.max_days = 2;
    let mut session =
        SimSession::open_from_spec(scenario, &game_mode).expect("open_from_spec");
    seed_research_progress_after_open(&mut session, 11.0);

    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("v3_sim_only.replay.ldjson");
    session.record_to_path(&path, 2).expect("record");

    // Sim-only path: just ReplayReader + ReplayDriver, no spec_replay.
    let file = std::fs::File::open(&path).expect("file");
    let mut reader = ReplayReader::new(BufReader::new(file));
    let snap = reader.read_snapshot().expect("structural snapshot");
    let mut driver = ReplayDriver::from_snapshot(snap);
    let mut frames = 0u32;
    while let Some(frame) = reader.next_frame().expect("frame parses") {
        driver.apply_frame(frame);
        frames += 1;
    }
    assert_eq!(
        frames, 2,
        "sim-only reader must see both frames despite intervening spec_snapshot line"
    );
}
