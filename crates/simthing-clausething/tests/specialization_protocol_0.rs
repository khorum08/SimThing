//! SPECIALIZATION-PROTOCOL-0 (0.0.8.7 rung 3.1) referee — remand-2 `5098401165` form.
//!
//! Legs:
//! 1. ONE ordinary installed canonical-TP report (production
//!    `preview_install_with_observations` over the authority root with the
//!    hydrated placement artifact) derives ALL THREE seed populations, each
//!    checked against independent artifact oracles, row by row;
//! 2. owner-seat binds to the ADMITTED owner-silo policy/weight locus — an
//!    Owner with an unrelated accumulator-bearing property does NOT derive;
//! 3. session-root enforces the strict sole/direct-child invariant (three
//!    negatives + the admitted absolute-root posture proven separately);
//! 4. authored provenance for BOTH error classes (unknown profile + unmet
//!    requirement) with exact tokens from the parsed document;
//! 5. a concrete pre-3.1 serialized fixture loads, admits, and re-serializes
//!    without the new field.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use simthing_clausething::raw::RawValue;
use simthing_clausething::{
    hydrate_scenario_with_source_base, parse_raw_document, HydratedScenarioNode,
    HydratedScenarioPack, RawDocument,
};
use simthing_core::{
    derive_specializations, kind_identity, seed_profiles, AccumulatorRole, AccumulatorSpec,
    ClampBehavior, DimensionRegistry, KindIdentity, LogTier, SimProperty, SimThing, SimThingKind,
    SimThingKindTag, SpecializationError, SpecializationObservations, SpecializationRequirement,
    SubFieldRole, SubFieldSpec, PROFILE_OWNER_SEAT, PROFILE_SESSION_ROOT, PROFILE_SPATIAL,
};
use simthing_driver::{
    preview_install, preview_install_with_observations, InstallError, Scenario,
};
use simthing_gpu::SlotAllocator;
use simthing_spec::{compile_property, owner_has_silo_metadata, GameModeSpec, PropertySpec};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn hydrate_canonical() -> HydratedScenarioPack {
    let clause_path = repo_root().join("scenarios/terran_pirate_galaxy.clause");
    let text = std::fs::read_to_string(&clause_path).expect("read canonical clause");
    let document = parse_raw_document(text.as_bytes()).expect("parse canonical clause");
    let base = clause_path.parent().expect("clause parent").to_path_buf();
    hydrate_scenario_with_source_base(&document, Some(&base)).expect("hydrate canonical clause")
}

fn walk<'a>(node: &'a SimThing, f: &mut impl FnMut(&'a SimThing)) {
    f(node);
    for child in &node.children {
        walk(child, f);
    }
}

/// Authoritative placement artifact for the authority tree: hydration stamps
/// each embedded-grid gridcell with structural col/row properties written
/// FROM `embedded.namespaced_placements`. The stamped population is
/// cross-checked against the grid artifact count so the observation set is
/// tied to the spec-side grid, not to kinds.
fn authority_placed_set(pack: &HydratedScenarioPack, authority: &SimThing) -> BTreeSet<u32> {
    let mut placed = BTreeSet::new();
    walk(authority, &mut |n| {
        if simthing_spec::gridcell_structural_col(n).is_some()
            && simthing_spec::gridcell_structural_row(n).is_some()
        {
            placed.insert(n.id.raw());
        }
    });
    let artifact_count: usize = pack
        .embedded_static_galaxy_scenarios
        .iter()
        .map(|e| e.namespaced_placements.len())
        .sum();
    assert_eq!(
        placed.len(),
        artifact_count,
        "structural stamps must correspond 1:1 with the embedded grid placements"
    );
    placed
}

/// Hydrated placement artifact for the authored world tree: grid placements
/// keyed by authored location id; `root_node` maps authored ids to ids.
fn canonical_placed_set(pack: &HydratedScenarioPack) -> BTreeSet<u32> {
    fn index(node: &HydratedScenarioNode, map: &mut std::collections::HashMap<String, u32>) {
        map.insert(node.id.clone(), node.simthing_id.raw());
        for child in &node.children {
            index(child, map);
        }
    }
    let mut by_authored_id = std::collections::HashMap::new();
    index(&pack.root_node, &mut by_authored_id);
    pack.grid_metadata
        .placements
        .iter()
        .filter_map(|p| by_authored_id.get(&p.location_id).copied())
        .collect()
}

fn minimal_scenario(root: SimThing) -> Scenario {
    let mut registry = DimensionRegistry::new();
    let _ = registry.register(SimProperty::simple("_session", "seed", 0));
    Scenario {
        name: "specialization_protocol_0".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 8192,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: Default::default(),
    }
}

#[test]
fn one_installed_canonical_report_derives_all_three_seed_populations() {
    let pack = hydrate_canonical();
    let authority = pack.authority_root.as_ref().expect("authority root");
    let placed = authority_placed_set(&pack, authority);
    assert!(!placed.is_empty(), "canonical grid carries placements");
    // The authored world-tree placement mapping stays consistent too.
    assert!(!canonical_placed_set(&pack).is_empty());

    // Independent artifact oracles, computed with no profile machinery:
    let mut oracle_spatial = BTreeSet::new();
    let mut oracle_seats = BTreeSet::new();
    let mut oracle_sessions = BTreeSet::new();
    walk(authority, &mut |n| {
        if n.kind == SimThingKind::Location && placed.contains(&n.id.raw()) {
            oracle_spatial.insert(n.id.raw());
        }
        if n.kind == SimThingKind::Owner && owner_has_silo_metadata(n) {
            oracle_seats.insert(n.id.raw());
        }
        if n.kind == SimThingKind::GameSession {
            oracle_sessions.insert(n.id.raw());
        }
    });
    assert!(!oracle_spatial.is_empty());
    assert!(oracle_seats.len() >= 2, "Terran + Pirate seats");
    assert_eq!(oracle_sessions.len(), 1);
    assert_eq!(
        authority.children.len(),
        1,
        "canonical Scenario root has the GameSession as its sole direct child"
    );

    // ONE ordinary installed report through the production admission path.
    let scenario = minimal_scenario(authority.clone());
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&scenario.root);
    let placed_vec: Vec<u32> = placed.iter().copied().collect();
    let preview = preview_install_with_observations(
        &GameModeSpec::default(),
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
        &placed_vec,
    )
    .expect("canonical authority tree admits");
    let report = &preview.state.specialization;

    let derived_of = |profile: &str| -> BTreeSet<u32> {
        report
            .rows
            .iter()
            .filter(|r| r.derived.iter().any(|d| d == profile))
            .map(|r| r.simthing)
            .collect()
    };
    assert_eq!(derived_of(PROFILE_SPATIAL), oracle_spatial);
    assert_eq!(derived_of(PROFILE_OWNER_SEAT), oracle_seats);
    assert_eq!(derived_of(PROFILE_SESSION_ROOT), oracle_sessions);

    // Row-by-row contract check against the oracles (no vacuous rows).
    for row in &report.rows {
        for d in &row.derived {
            match d.as_str() {
                x if x == PROFILE_SPATIAL => {
                    assert_eq!(row.kind, KindIdentity::BuiltIn(SimThingKindTag::Location));
                    assert!(placed.contains(&row.simthing));
                }
                x if x == PROFILE_OWNER_SEAT => {
                    assert_eq!(row.kind, KindIdentity::BuiltIn(SimThingKindTag::Owner));
                    assert!(oracle_seats.contains(&row.simthing));
                }
                x if x == PROFILE_SESSION_ROOT => {
                    assert_eq!(
                        row.kind,
                        KindIdentity::BuiltIn(SimThingKindTag::GameSession)
                    );
                    assert!(oracle_sessions.contains(&row.simthing));
                }
                other => panic!("unexpected derived profile {other}"),
            }
        }
    }
}

#[test]
fn owner_seat_requires_the_admitted_silo_locus_not_any_accumulator() {
    // An Owner hosting an UNRELATED accumulator-bearing property (a production
    // flow) is NOT the seat; a real silo-locus Owner is.
    let mut registry = DimensionRegistry::new();
    let property = PropertySpec {
        id: "unrelated_flow".into(),
        namespace: "factory".into(),
        name: "unrelated_flow".into(),
        display_name: "Unrelated Flow".into(),
        description: String::new(),
        sub_fields: vec![SubFieldSpec {
            role: SubFieldRole::Named("flow".into()),
            width: 1,
            clamp: ClampBehavior::Unbounded,
            velocity_max: None,
            default: 5.0,
            display_name: "flow".into(),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: Some(AccumulatorSpec {
                role: AccumulatorRole::IntrinsicFlow,
                log_tier: LogTier::Summary,
            }),
        }],
    };
    let (pid, _diag) = compile_property(&property, &mut registry).expect("property admission");

    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    let mut accumulator_owner = SimThing::new(SimThingKind::Owner, 0);
    let default_value = registry.property(pid).default_value();
    accumulator_owner.add_property(pid, default_value);
    let accumulator_owner_id = accumulator_owner.id.raw();

    let mut silo_owner = simthing_spec::make_owner_entity("seat", "Seat", "settler");
    simthing_spec::apply_owner_silo_metadata(&mut silo_owner, 3, Some(10));
    let silo_owner_id = silo_owner.id.raw();

    root.add_child(accumulator_owner);
    root.add_child(silo_owner);

    let mut scenario = minimal_scenario(root);
    scenario.registry = registry;
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&scenario.root);

    let preview = preview_install(
        &GameModeSpec::default(),
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
    )
    .expect("admits");
    let report = &preview.state.specialization;
    assert!(
        !report
            .derived_ids(accumulator_owner_id)
            .contains(&PROFILE_OWNER_SEAT),
        "an unrelated accumulator host must NOT derive owner-seat"
    );
    assert!(report.derived_ids(silo_owner_id).contains(&PROFILE_OWNER_SEAT));
}

#[test]
fn session_root_enforces_the_strict_sole_direct_child_invariant() {
    let derive = |root: &SimThing| {
        derive_specializations(root, &seed_profiles(), &SpecializationObservations::default())
            .expect("derives")
            .rows
            .iter()
            .filter(|r| r.derived.iter().any(|d| d == PROFILE_SESSION_ROOT))
            .count()
    };

    // Negative 1: two GameSessions.
    let mut two = SimThing::new(SimThingKind::Scenario, 0);
    two.add_child(SimThing::new(SimThingKind::GameSession, 0));
    two.add_child(SimThing::new(SimThingKind::GameSession, 0));
    assert_eq!(derive(&two), 0);

    // Negative 2: one GameSession plus a non-GameSession sibling.
    let mut sibling = SimThing::new(SimThingKind::Scenario, 0);
    sibling.add_child(SimThing::new(SimThingKind::GameSession, 0));
    sibling.add_child(SimThing::new(SimThingKind::World, 0));
    assert_eq!(derive(&sibling), 0);

    // Negative 3: nested / wrongly parented GameSession.
    let mut nested = SimThing::new(SimThingKind::Scenario, 0);
    let mut world = SimThing::new(SimThingKind::World, 0);
    world.add_child(SimThing::new(SimThingKind::GameSession, 0));
    nested.add_child(world);
    assert_eq!(derive(&nested), 0);

    // Positive: the sole direct GameSession child.
    let mut sole = SimThing::new(SimThingKind::Scenario, 0);
    sole.add_child(SimThing::new(SimThingKind::GameSession, 0));
    assert_eq!(derive(&sole), 1);

    // Admitted absolute-root posture, proven separately.
    let absolute = SimThing::new(SimThingKind::GameSession, 0);
    assert_eq!(derive(&absolute), 1);
}

#[test]
fn custom_kind_impostors_never_satisfy_builtin_identity() {
    let mut scenario_root = SimThing::new(SimThingKind::Scenario, 0);
    let mut session = SimThing::new(SimThingKind::Custom("GameSession".into()), 0);
    let impostor_owner = SimThing::new(SimThingKind::Custom("Owner".into()), 0);
    let impostor_location = SimThing::new(SimThingKind::Custom("Location".into()), 0);
    let impostor_location_id = impostor_location.id.raw();
    session.add_child(impostor_owner);
    session.add_child(impostor_location);
    scenario_root.add_child(session);

    let mut obs = SpecializationObservations::default();
    obs.structurally_placed.insert(impostor_location_id);
    walk(&scenario_root, &mut |n| {
        obs.policy_weight_hosts.insert(n.id.raw());
    });

    let report =
        derive_specializations(&scenario_root, &seed_profiles(), &obs).expect("derives");
    for row in &report.rows {
        assert!(
            row.derived.is_empty(),
            "impostor kinds must derive nothing, got {:?} for {:?}",
            row.derived,
            row.kind
        );
    }
}

const CLAUSE_UNMET_PROOF: &str = r#"
scenario = span_proof {
    location = anchor_cell {
        display_name = "Anchor Cell"
    }
    owner = misdeclared {
        owner_key = "misdeclared"
        display_name = "Misdeclared"
        archetype = "expansionist"
        specialization = spatial
    }
}
"#;

const CLAUSE_UNKNOWN_PROOF: &str = r#"
scenario = span_proof_unknown {
    location = anchor_cell {
        display_name = "Anchor Cell"
    }
    owner = goblin_fan {
        owner_key = "goblin_fan"
        display_name = "Goblin Fan"
        archetype = "expansionist"
        specialization = warp_goblin
    }
}
"#;

fn declared_scalar_token(document: &RawDocument) -> usize {
    fn find(value: &RawValue, out: &mut Option<usize>) {
        match value {
            RawValue::Block(block) => {
                for property in &block.properties {
                    if property.key.text == "specialization" {
                        if let RawValue::Scalar(scalar) = &property.value {
                            *out = Some(scalar.span.token_index);
                            return;
                        }
                    }
                    find(&property.value, out);
                    if out.is_some() {
                        return;
                    }
                }
            }
            RawValue::Array(array) => {
                for item in &array.items {
                    find(item, out);
                    if out.is_some() {
                        return;
                    }
                }
            }
            RawValue::Header(header) => find(&header.payload, out),
            RawValue::Scalar(_) => {}
        }
    }
    let mut out = None;
    find(&document.root, &mut out);
    out.expect("authored specialization scalar present")
}

fn authored_owner_id(pack: &HydratedScenarioPack) -> u32 {
    pack.owners
        .first()
        .expect("authored owner")
        .simthing_id
        .raw()
}

#[test]
fn authored_unmet_requirement_carries_simthing_requirement_and_span() {
    let document = parse_raw_document(CLAUSE_UNMET_PROOF.as_bytes()).expect("parse");
    let expected_token = declared_scalar_token(&document);
    let pack = simthing_clausething::hydrate_scenario(&document).expect("hydrate");
    let authority = pack.authority_root.as_ref().expect("authority root");
    let expected_owner = authored_owner_id(&pack);

    let error = derive_specializations(
        authority,
        &seed_profiles(),
        &SpecializationObservations::default(),
    )
    .expect_err("Owner declaring `spatial` must fail");
    match error {
        SpecializationError::RequirementUnmet {
            simthing,
            kind,
            profile,
            requirement,
            span_token,
        } => {
            assert_eq!(simthing, expected_owner);
            assert_eq!(kind, kind_identity(&SimThingKind::Owner));
            assert_eq!(profile, "spatial");
            assert_eq!(
                requirement,
                SpecializationRequirement::Kind(KindIdentity::BuiltIn(SimThingKindTag::Location)),
                "the precise unmet typed requirement is reported"
            );
            assert_eq!(span_token, Some(expected_token));
        }
        other => panic!("expected RequirementUnmet, got {other:?}"),
    }
}

#[test]
fn authored_unknown_profile_carries_simthing_profile_and_span() {
    let document = parse_raw_document(CLAUSE_UNKNOWN_PROOF.as_bytes()).expect("parse");
    let expected_token = declared_scalar_token(&document);
    let pack = simthing_clausething::hydrate_scenario(&document).expect("hydrate");
    let authority = pack.authority_root.as_ref().expect("authority root");
    let expected_owner = authored_owner_id(&pack);

    // Through the ordinary install path.
    let scenario = minimal_scenario(authority.clone());
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&scenario.root);
    let error = preview_install(
        &GameModeSpec::default(),
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
    )
    .expect_err("unknown profile must fail admission");
    match error {
        InstallError::Specialization(SpecializationError::UnknownProfile {
            simthing,
            profile,
            span_token,
        }) => {
            assert_eq!(simthing, expected_owner);
            assert_eq!(profile, "warp_goblin");
            assert_eq!(span_token, Some(expected_token));
        }
        other => panic!("expected UnknownProfile install error, got {other:?}"),
    }
}

/// Concrete pre-3.1 wire fixture: no `declared_specializations`, and the
/// pre-generation `spawned_day` field name (1.2 alias) — the oldest supported
/// wire shape this rung must not disturb.
const PRE_3_1_SIMTHING_JSON: &str = r#"{
    "id": 424242,
    "kind": "GameSession",
    "properties": [],
    "overlays": [],
    "children": [],
    "spawned_day": 0
}"#;

#[test]
fn pre_3_1_fixture_loads_admits_and_reserializes_without_the_new_field() {
    let legacy: SimThing =
        serde_json::from_str(PRE_3_1_SIMTHING_JSON).expect("pre-3.1 fixture loads");
    assert!(legacy.declared_specializations.is_empty());
    assert_eq!(legacy.id.raw(), 424242);
    assert_eq!(legacy.spawned_generation, 0);

    // Ordinary admission over the legacy tree.
    let scenario = minimal_scenario(legacy.clone());
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&scenario.root);
    preview_install(
        &GameModeSpec::default(),
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
    )
    .expect("pre-3.1 tree admits unchanged");

    // Re-serialization does not invent the new field.
    let json = serde_json::to_string(&legacy).expect("serialize");
    assert!(!json.contains("declared_specializations"));
    assert!(!json.contains("span_token"));
}
