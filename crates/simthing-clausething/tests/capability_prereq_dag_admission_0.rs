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
        order_weight_classes: vec![],
        capability_trees: Vec::new(),
        events: Vec::new(),
        resource_flow: None,
        resource_economy: None,
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
    let property_id = *property_ids.values().next().expect("at least one property");
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
        // Set transforms: GPU readback corroborates that the active overlay
        // applied (A→4.0, B→6.0). Lifecycle on session.proto.root is the
        // load-bearing exclusivity proof (remand-2): dual-active last-writer
        // Set could mask suspension, so we assert Suspended/active on the
        // authoritative installed overlays + host active-overlay census == 1.
        let set_to = if entry.id == "idea_a" { 4.0 } else { 6.0 };
        for effect in &mut entry.effects {
            effect.sub_field_deltas = vec![(SubFieldRole::Amount, TransformOp::set(set_to))];
            effect.when_activated = OverlayLifecycle::UntilDissolved;
        }
    }
}

/// Complete observed atomicity trace for dual-run equality (not final bits alone).
#[derive(Clone, Debug, PartialEq, Eq)]
struct AtomicityTrace {
    after_a_active: Vec<String>,
    after_a_a_active: bool,
    after_a_b_suspended: bool,
    after_a_bits: u32,
    after_b_boundary_active: Vec<String>,
    after_b_a_suspended: bool,
    after_b_b_active: bool,
    after_b_bits: u32,
    boundaries_a: u64,
    boundaries_b: u64,
}

fn read_owner_amount(
    session: &SimSession,
    owner_id: SimThingId,
    property_id: SimPropertyId,
) -> f32 {
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

/// ClauseThing authored tradition trees + shipsize account (fail-closed).
#[test]
fn clause_authored_and_programmatic_capability_corpus_admits() {
    let mut ct1c_trees = 0usize;
    let mut shipsize_trees = 0usize;
    let mut shipsize_threshold_zero_cost = 0usize;

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
            ct1c_trees += 1;
        }
    }

    // Ship-size: programmatic production constructor — hydration MUST succeed.
    // Empty prereq DAG required. Threshold+zero-cost is the only allowed
    // non-DAG disposition; DAG-class errors and hydrate failure are hard FAIL.
    {
        let document = parse_raw_document(SHIPSIZE_FIXTURE.as_bytes()).expect("shipsize parse");
        let pack = hydrate_shipsize_decoder_pack(&document)
            .expect("shipsize hydrate must succeed for the checked fixture (fail-closed account)");
        let trees: Vec<_> = pack
            .game_mode
            .capability_trees
            .iter()
            .chain(
                pack.game_mode
                    .domain_packs
                    .iter()
                    .flat_map(|dp| dp.capability_trees.iter()),
            )
            .collect();
        assert!(
            !trees.is_empty(),
            "shipsize fixture must produce at least one capability tree"
        );
        for tree in trees {
            shipsize_trees += 1;
            for cat in &tree.categories {
                for entry in &cat.entries {
                    assert!(
                        entry.prereqs.is_empty(),
                        "shipsize tree {} entry {} must not invent prereq edges",
                        tree.tree_id,
                        entry.id
                    );
                }
            }
            match validate_capability_tree(tree) {
                Ok(_) => {
                    eprintln!(
                        "CENSUS shipsize tree {} admits (empty prereq DAG)",
                        tree.tree_id
                    );
                }
                Err(SpecError::ThresholdRequiresPositiveCost(entry)) => {
                    shipsize_threshold_zero_cost += 1;
                    eprintln!(
                        "CENSUS-NOTE shipsize tree {} entry `{entry}`: allowed pre-existing Threshold+zero-cost (programmatic, non-DAG)",
                        tree.tree_id
                    );
                }
                Err(e) => {
                    panic!(
                        "STOP/DA-route: shipsize tree {} unexpected admission error (DAG or other): {e}",
                        tree.tree_id
                    );
                }
            }
        }
        assert!(
            shipsize_trees >= 1,
            "shipsize tree count must be reported and non-zero"
        );
    }

    eprintln!(
        "CAPABILITY-PREREQ-DAG-CLAUSE-CENSUS sources=2 labels=[ct1c_tradition_set.clause,tp_shipsize_decoder_0.clause(programmatic)] ct1c_trees={ct1c_trees} shipsize_trees={shipsize_trees} shipsize_threshold_zero_cost={shipsize_threshold_zero_cost}"
    );
}

/// Load-bearing live generation/GPU atomicity referee.
///
/// Lifecycle assertions on `session.proto.root` are the exclusivity proof.
/// Multiply GPU oracle is corroboration only (dual-active would yield 12, not 6).
#[test]
fn max_active_sibling_switch_atomic_on_live_gpu_session() {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let mut traces: Vec<AtomicityTrace> = Vec::new();
    for run in 0..2 {
        let mut game_mode = hydrate_clause_mode(CLAUSE_MAX_ACTIVE_PROOF);
        preserve_unbounded_amount(&mut game_mode, "idea_host", "pressure");
        apply_max_active_limited(&mut game_mode);
        let tree_id = game_mode.domain_packs[0].capability_trees[0]
            .tree_id
            .clone();
        let (scenario, owner_id, property_id) = scenario_with_owner(&mut game_mode, 2.0);

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

        // Resolve overlay ids + hosts before any activation.
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
        let a_oid = a_overlays[0];
        let b_oid = b_overlays[0];
        assert_eq!(
            instance.overlay_hosts.get(&a_oid),
            Some(&owner_id),
            "A host must be owner"
        );
        assert_eq!(
            instance.overlay_hosts.get(&b_oid),
            Some(&owner_id),
            "B host must be owner"
        );
        assert!(session.proto.root.has_overlay(owner_id, a_oid));
        assert!(session.proto.root.has_overlay(owner_id, b_oid));
        // Initial: both suspended (capability install starts Suspended).
        assert_eq!(
            session.proto.root.overlay_is_suspended(owner_id, a_oid),
            Some(true),
            "A starts Suspended"
        );
        assert_eq!(
            session.proto.root.overlay_is_suspended(owner_id, b_oid),
            Some(true),
            "B starts Suspended"
        );

        // ── A selection + barrier ──────────────────────────────────────────
        session
            .spec_state
            .queue_player_selection_by_key(owner_id, &tree_id, "idea_a")
            .expect("queue A");
        let step_a = session.step_once().expect("A boundary");
        assert!(
            step_a.boundary_reached,
            "A must land at a generation barrier"
        );
        assert!(
            step_a.boundaries_run >= 1,
            "A boundary must execute structural mutations"
        );
        let after_a_active = active_entry_ids(&session, owner_id);
        assert_eq!(
            after_a_active,
            vec!["idea_a".to_string()],
            "only A in spec state"
        );

        // Load-bearing lifecycle: A active, B still suspended — before production tick.
        let after_a_a_active = session
            .proto
            .root
            .overlay_is_active(owner_id, a_oid)
            .expect("A overlay present on host");
        let after_a_b_suspended = session
            .proto
            .root
            .overlay_is_suspended(owner_id, b_oid)
            .expect("B overlay present on host");
        assert!(
            after_a_a_active,
            "run{run}: after A boundary A lifecycle must be active (not missing/no-op)"
        );
        assert!(
            after_a_b_suspended,
            "run{run}: after A boundary B lifecycle must remain Suspended"
        );
        assert_eq!(
            session.proto.root.overlay_is_suspended(owner_id, a_oid),
            Some(false),
            "A is not Suspended after activation"
        );

        session.step_once().expect("A production tick");
        let after_a = read_owner_amount(&session, owner_id, property_id);
        // Corroboration only: A Set(4.0) applied.
        assert_eq!(
            after_a.to_bits(),
            4.0_f32.to_bits(),
            "run{run}: after A GPU Set(4.0) corroboration"
        );

        // ── B selection + barrier (atomicity load-bearing point) ───────────
        session
            .spec_state
            .queue_player_selection_by_key(owner_id, &tree_id, "idea_b")
            .expect("queue B");
        let step_b = session.step_once().expect("B boundary");
        assert!(
            step_b.boundary_reached,
            "B must land at a generation barrier"
        );
        let after_b_boundary_active = active_entry_ids(&session, owner_id);
        assert_eq!(
            after_b_boundary_active,
            vec!["idea_b".to_string()],
            "run{run}: same barrier leaves exactly B in spec state"
        );

        // Load-bearing lifecycle BEFORE next production tick:
        // A Suspended, B active — proves Suspend+Activate both applied (not missing/no-op).
        let after_b_a_suspended = session
            .proto
            .root
            .overlay_is_suspended(owner_id, a_oid)
            .expect("A overlay present");
        let after_b_b_active = session
            .proto
            .root
            .overlay_is_active(owner_id, b_oid)
            .expect("B overlay present");
        assert!(
            after_b_a_suspended,
            "run{run}: after B boundary A lifecycle must be Suspended (authoritative root)"
        );
        assert!(
            after_b_b_active,
            "run{run}: after B boundary B lifecycle must be active (authoritative root)"
        );
        assert_eq!(
            session.proto.root.overlay_is_active(owner_id, a_oid),
            Some(false),
            "A not lifecycle-active after B boundary"
        );
        assert_eq!(
            session.proto.root.overlay_is_suspended(owner_id, b_oid),
            Some(false),
            "B not Suspended after B boundary"
        );
        // Census every overlay on the host: exactly one lifecycle-active overlay (B).
        let host_overlay_ids = session
            .proto
            .root
            .snapshot_node(owner_id)
            .expect("owner snapshot")
            .overlay_ids;
        let mut active_on_host = 0usize;
        for oid in &host_overlay_ids {
            if session
                .proto
                .root
                .overlay_is_active(owner_id, *oid)
                .unwrap_or(false)
            {
                active_on_host += 1;
                assert_eq!(
                    *oid, b_oid,
                    "run{run}: only B may be lifecycle-active on host; unexpected active {oid:?}"
                );
            }
        }
        assert_eq!(
            active_on_host, 1,
            "run{run}: exactly one lifecycle-active overlay on host after B boundary (got {active_on_host}; host_overlays={host_overlay_ids:?})"
        );

        // Production tick: B Set(6.0) corroboration. Lifecycle exclusivity is
        // already proven above (A Suspended, B active, host active count == 1)
        // before this tick — Set cannot mask that load-bearing check.
        session.step_once().expect("B production tick");
        let after_b = read_owner_amount(&session, owner_id, property_id);
        assert_eq!(
            after_b.to_bits(),
            6.0_f32.to_bits(),
            "run{run}: GPU corroboration B Set(6.0) applied (got {after_b})"
        );
        // Lifecycle still exclusive after the production tick.
        assert_eq!(
            session.proto.root.overlay_is_suspended(owner_id, a_oid),
            Some(true),
            "run{run}: A remains Suspended after B production tick"
        );
        assert_eq!(
            session.proto.root.overlay_is_active(owner_id, b_oid),
            Some(true),
            "run{run}: B remains active after B production tick"
        );

        let trace = AtomicityTrace {
            after_a_active: after_a_active.clone(),
            after_a_a_active,
            after_a_b_suspended,
            after_a_bits: after_a.to_bits(),
            after_b_boundary_active: after_b_boundary_active.clone(),
            after_b_a_suspended,
            after_b_b_active,
            after_b_bits: after_b.to_bits(),
            boundaries_a: step_a.boundaries_run,
            boundaries_b: step_b.boundaries_run,
        };
        traces.push(trace);

        eprintln!(
            "CAPABILITY-PREREQ-DAG-ATOMICITY-GPU-PROOF run={run} adapter={:?} backend={:?} device_type={:?} owner={owner_id:?} a_oid={a_oid:?} b_oid={b_oid:?} after_a_active={after_a_active:?} after_a_lifecycle_A_active={after_a_a_active} after_a_lifecycle_B_suspended={after_a_b_suspended} after_a_bits={} after_b_boundary_active={after_b_boundary_active:?} after_b_lifecycle_A_suspended={after_b_a_suspended} after_b_lifecycle_B_active={after_b_b_active} after_b_bits={} boundaries_a={} boundaries_b={}",
            adapter.name,
            adapter.backend,
            adapter.device_type,
            after_a.to_bits(),
            after_b.to_bits(),
            step_a.boundaries_run,
            step_b.boundaries_run
        );
    }

    assert_eq!(
        traces[0], traces[1],
        "complete-trace equality: A/B active sets, lifecycle flags, GPU bits, boundary markers"
    );
}
