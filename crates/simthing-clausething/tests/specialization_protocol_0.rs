//! SPECIALIZATION-PROTOCOL-0 (0.0.8.7 rung 3.1) referee — remand `5098201367` form.
//!
//! Non-tautological legs:
//! 1. spatial derives from the AUTHORITATIVE placement artifact (embedded
//!    structural grid), not from kind: placed Locations derive, an unplaced
//!    Location does NOT, and a placed `Custom("Location")` impostor does NOT;
//! 2. owner-seat requires the HOSTING fact: with an empty host observation the
//!    canonical owners do NOT derive; through the production install path
//!    (registry-backed hosting walk) a hosting Owner DOES derive and a bare
//!    Owner does NOT;
//! 3. session-root requires the SOLE-child invariant: two GameSessions under
//!    a Scenario root derive nothing;
//! 4. declared-profile failures carry REAL authored spans from the clause
//!    loader (token index proven against the parsed document, 2.1 pattern);
//! 5. legacy trees stay wire-identical.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use simthing_clausething::raw::RawValue;
use simthing_clausething::{
    hydrate_scenario_with_source_base, parse_raw_document, HydratedScenarioPack, RawDocument,
};
use simthing_core::{
    derive_specializations, seed_profiles, AccumulatorRole, AccumulatorSpec, ClampBehavior,
    DimensionRegistry, LogTier, SimProperty, SimThing, SimThingKind, SpecializationError,
    SpecializationObservations, SubFieldRole, SubFieldSpec, PROFILE_OWNER_SEAT,
    PROFILE_SESSION_ROOT, PROFILE_SPATIAL,
};
use simthing_spec::{compile_property, PropertySpec};
use simthing_driver::{preview_install, InstallError, Scenario};
use simthing_gpu::SlotAllocator;
use simthing_spec::GameModeSpec;

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

fn canonical_placed_set(pack: &HydratedScenarioPack) -> BTreeSet<u32> {
    // Hydrated placement artifact: grid_metadata placements are keyed by
    // authored location id; root_node maps authored ids to SimThing ids.
    use simthing_clausething::HydratedScenarioNode;
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

#[test]
fn spatial_derives_from_the_placement_artifact_not_from_kind() {
    let pack = hydrate_canonical();
    let authority = pack.authority_root.as_ref().expect("authority root");
    let placed = canonical_placed_set(&pack);
    assert!(!placed.is_empty(), "canonical grid carries placements");

    let obs = SpecializationObservations {
        structurally_placed: placed.clone(),
        resource_property_hosts: BTreeSet::new(),
    };

    // Placements bind to the WORLD structural tree (pack.root). Spatial must
    // derive there exactly on placed Locations — the artifact, not the kind.
    let world_report =
        derive_specializations(&pack.root, &seed_profiles(), &obs).expect("world derives");
    let mut expected_spatial = BTreeSet::new();
    walk(&pack.root, &mut |n| {
        if n.kind == SimThingKind::Location && placed.contains(&n.id.raw()) {
            expected_spatial.insert(n.id.raw());
        }
    });
    let derived_spatial: BTreeSet<u32> = world_report
        .rows
        .iter()
        .filter(|r| r.derived.iter().any(|d| d == PROFILE_SPATIAL))
        .map(|r| r.simthing)
        .collect();
    assert_eq!(derived_spatial, expected_spatial);
    assert!(!derived_spatial.is_empty());

    // The placement requirement BITES on the live corpus: the AUTHORITY tree's
    // 4500+ Locations carry no world-grid placements, so with this artifact
    // set none of them may derive spatial — kind alone is insufficient.
    let report = derive_specializations(authority, &seed_profiles(), &obs).expect("derives");
    let mut authority_locations = 0usize;
    walk(authority, &mut |n| {
        if n.kind == SimThingKind::Location {
            authority_locations += 1;
        }
    });
    assert!(authority_locations > 0);
    assert!(
        !report
            .rows
            .iter()
            .any(|r| r.derived.iter().any(|d| d == PROFILE_SPATIAL)),
        "unplaced authority Locations must not derive spatial"
    );

    // The hosting requirement BITES: with empty host observations, canonical
    // owners must NOT derive owner-seat (kind+posture alone is insufficient).
    assert!(
        !report
            .rows
            .iter()
            .any(|r| r.derived.iter().any(|d| d == PROFILE_OWNER_SEAT)),
        "owner-seat must not derive without the hosting fact"
    );

    // Exactly one sole session root in the canonical authority tree.
    let session_roots = report
        .rows
        .iter()
        .filter(|r| r.derived.iter().any(|d| d == PROFILE_SESSION_ROOT))
        .count();
    assert_eq!(session_roots, 1);
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
        obs.resource_property_hosts.insert(n.id.raw());
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

#[test]
fn session_root_requires_the_sole_child_invariant() {
    let mut scenario_root = SimThing::new(SimThingKind::Scenario, 0);
    scenario_root.add_child(SimThing::new(SimThingKind::GameSession, 0));
    scenario_root.add_child(SimThing::new(SimThingKind::GameSession, 0));
    let report = derive_specializations(
        &scenario_root,
        &seed_profiles(),
        &SpecializationObservations::default(),
    )
    .expect("derives");
    assert!(
        !report
            .rows
            .iter()
            .any(|r| r.derived.iter().any(|d| d == PROFILE_SESSION_ROOT)),
        "two GameSessions under one Scenario: neither is the sole session root"
    );
}

#[test]
fn production_install_derives_owner_seat_from_registry_backed_hosting() {
    let mut registry = DimensionRegistry::new();
    let property = PropertySpec {
        id: "stockpile".into(),
        namespace: "seat".into(),
        name: "stockpile".into(),
        display_name: "Stockpile".into(),
        description: String::new(),
        sub_fields: vec![SubFieldSpec {
            role: SubFieldRole::Named("amount".into()),
            width: 1,
            clamp: ClampBehavior::Unbounded,
            velocity_max: None,
            default: 5.0,
            display_name: "amount".into(),
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
    let (pid, _diag) =
        compile_property(&property, &mut registry).expect("seat property admission");

    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    let mut hosting_owner = SimThing::new(SimThingKind::Owner, 0);
    let default_value = registry.property(pid).default_value();
    hosting_owner.add_property(pid, default_value);
    let hosting_id = hosting_owner.id.raw();
    let bare_owner = SimThing::new(SimThingKind::Owner, 0);
    let bare_id = bare_owner.id.raw();
    root.add_child(hosting_owner);
    root.add_child(bare_owner);

    let scenario = Scenario {
        name: "specialization_protocol_0".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 32,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: Default::default(),
    };
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&scenario.root);

    let preview = preview_install(
        &GameModeSpec::default(),
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
    )
    .expect("hosting owner admits");
    let report = &preview.state.specialization;
    assert!(report.derived_ids(hosting_id).contains(&PROFILE_OWNER_SEAT));
    assert!(
        !report.derived_ids(bare_id).contains(&PROFILE_OWNER_SEAT),
        "a bare Owner without resource hosting is not the seat"
    );
}

const CLAUSE_SPAN_PROOF: &str = r#"
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

#[test]
fn authored_declaration_failure_carries_the_real_clause_span() {
    let document = parse_raw_document(CLAUSE_SPAN_PROOF.as_bytes()).expect("parse span proof");
    let expected_token = declared_scalar_token(&document);
    let pack = simthing_clausething::hydrate_scenario(&document).expect("hydrate span proof");
    let authority = pack.authority_root.as_ref().expect("authority root");

    let error = derive_specializations(
        authority,
        &seed_profiles(),
        &SpecializationObservations::default(),
    )
    .expect_err("an Owner declaring `spatial` must fail validation");
    match error {
        SpecializationError::RequirementUnmet {
            profile,
            span_token,
            ..
        } => {
            assert_eq!(profile, "spatial");
            assert_eq!(
                span_token,
                Some(expected_token),
                "admission error must carry the authored scalar's token index"
            );
        }
        other => panic!("expected RequirementUnmet, got {other:?}"),
    }
}

#[test]
fn unknown_declared_profile_is_a_spanned_install_error() {
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    root.declared_specializations
        .push(simthing_core::DeclaredSpecialization {
            profile: "warp-goblin".into(),
            span_token: None,
        });
    let mut registry = DimensionRegistry::new();
    let _ = registry.register(SimProperty::simple("_session", "seed", 0));
    let scenario = Scenario {
        name: "specialization_protocol_0".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 32,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: Default::default(),
    };
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
    assert!(matches!(
        error,
        InstallError::Specialization(SpecializationError::UnknownProfile { .. })
    ));
}

#[test]
fn legacy_trees_round_trip_wire_identically() {
    let root = SimThing::new(SimThingKind::World, 0);
    let json = serde_json::to_string(&root).expect("serialize");
    assert!(
        !json.contains("declared_specializations"),
        "empty declarations must not appear on the wire: {json}"
    );
    let legacy: SimThing = serde_json::from_str(&json).expect("legacy JSON loads");
    assert!(legacy.declared_specializations.is_empty());
}
