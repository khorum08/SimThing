//! SPECIALIZATION-PROTOCOL-0 (0.0.8.7 rung 3.1) referee.
//!
//! Proves the three exit-proof legs on the REAL corpus:
//! 1. canonical TP hydration derives the expected profile sets with zero
//!    authoring changes (session-root exactly once; owner-seat per owner;
//!    spatial exactly on the Location population);
//! 2. a declared-but-nonconforming profile is a hard admission error through
//!    the ordinary install path;
//! 3. legacy trees (no declarations) round-trip wire-identically — the new
//!    field serializes to nothing when empty.

use std::path::{Path, PathBuf};

use simthing_clausething::{
    hydrate_scenario_with_source_base, parse_raw_document, HydratedScenarioPack,
};
use simthing_core::{
    derive_specializations, seed_profiles, DimensionRegistry, SimProperty, SimThing, SimThingKind,
    PROFILE_OWNER_SEAT, PROFILE_SESSION_ROOT, PROFILE_SPATIAL,
};
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

fn count_kind(node: &SimThing, kind: &SimThingKind, acc: &mut usize) {
    if node.kind == *kind {
        *acc += 1;
    }
    for child in &node.children {
        count_kind(child, kind, acc);
    }
}

#[test]
fn canonical_tp_corpus_derives_the_three_seed_profiles_unchanged() {
    let pack = hydrate_canonical();
    let authority = pack
        .authority_root
        .as_ref()
        .expect("canonical TP pack carries the authority root");
    let report =
        derive_specializations(authority, &seed_profiles()).expect("canonical corpus derives");

    let session_roots: Vec<_> = report
        .rows
        .iter()
        .filter(|r| r.derived.iter().any(|d| d == PROFILE_SESSION_ROOT))
        .collect();
    assert_eq!(
        session_roots.len(),
        1,
        "exactly one session-root in the canonical corpus"
    );

    let owner_seats = report
        .rows
        .iter()
        .filter(|r| r.derived.iter().any(|d| d == PROFILE_OWNER_SEAT))
        .count();
    assert!(
        owner_seats >= 2,
        "canonical TP carries at least the Terran and Pirate owner seats (got {owner_seats})"
    );

    let mut location_population = 0usize;
    count_kind(authority, &SimThingKind::Location, &mut location_population);
    let spatial = report
        .rows
        .iter()
        .filter(|r| r.derived.iter().any(|d| d == PROFILE_SPATIAL))
        .count();
    assert_eq!(
        spatial, location_population,
        "spatial derives exactly on the Location population (\u{a7}7: no non-spatial Location)"
    );
    assert!(spatial > 0, "canonical TP has spatial participants");

    assert!(
        report.rows.iter().all(|r| r.declared.is_empty()),
        "legacy canonical corpus declares nothing (derivation is pure observation)"
    );
}

fn minimal_scenario(root: SimThing) -> Scenario {
    let mut registry = DimensionRegistry::new();
    let _ = registry.register(SimProperty::simple("_session", "seed", 0));
    Scenario {
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
    }
}

#[test]
fn declared_nonconforming_profile_is_a_hard_admission_error() {
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    let mut impostor = SimThing::new(SimThingKind::Cohort, 0);
    impostor
        .declared_specializations
        .push(PROFILE_SPATIAL.to_string());
    root.add_child(impostor);

    let scenario = minimal_scenario(root);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&scenario.root);
    let game_mode = GameModeSpec::default();

    let error = preview_install(
        &game_mode,
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
    )
    .expect_err("a Cohort declaring `spatial` must fail admission");
    assert!(
        matches!(error, InstallError::Specialization(_)),
        "expected Specialization admission error, got: {error:?}"
    );
}

#[test]
fn declared_unknown_profile_is_a_hard_admission_error() {
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    root.declared_specializations.push("warp-goblin".into());

    let scenario = minimal_scenario(root);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&scenario.root);

    let error = preview_install(
        &GameModeSpec::default(),
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
    )
    .expect_err("an unknown declared profile must fail admission");
    assert!(matches!(error, InstallError::Specialization(_)));
}

#[test]
fn legacy_trees_round_trip_wire_identically() {
    // A tree with no declarations must serialize WITHOUT the new field and
    // deserialize from pre-3.1 JSON unchanged.
    let root = SimThing::new(SimThingKind::World, 0);
    let json = serde_json::to_string(&root).expect("serialize");
    assert!(
        !json.contains("declared_specializations"),
        "empty declarations must not appear on the wire: {json}"
    );
    let legacy: SimThing = serde_json::from_str(&json).expect("legacy JSON loads");
    assert!(legacy.declared_specializations.is_empty());
}

#[test]
fn structural_map_root_derives_spatial_on_every_gridcell() {
    let pack = hydrate_canonical();
    let report =
        derive_specializations(&pack.root, &seed_profiles()).expect("map root derives");
    let mut locations = 0usize;
    count_kind(&pack.root, &SimThingKind::Location, &mut locations);
    let spatial = report
        .rows
        .iter()
        .filter(|r| r.derived.iter().any(|d| d == PROFILE_SPATIAL))
        .count();
    assert_eq!(spatial, locations);
    assert!(spatial > 0, "structural map carries gridcells");
}
