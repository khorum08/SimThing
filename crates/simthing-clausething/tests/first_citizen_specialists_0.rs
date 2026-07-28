//! FIRST-CITIZEN-SPECIALISTS-0 (0.0.8.7 rung 3.2) referee.
//!
//! Legs:
//! 1. Authored Location `specialization = spatial` admits when structurally
//!    placed, and spans `StructurallyPlaced` unmet when unplaced;
//! 2. Authored entity (non-Owner child) declaring `owner-seat` spans
//!    `Kind(Owner)` unmet with the exact clause scalar token;
//! 3. Canonical installed citizen counts match the generator-source TSV
//!    consumed by board/orientation (never a hand-edited mirror).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use simthing_clausething::raw::RawValue;
use simthing_clausething::{
    hydrate_scenario, parse_raw_document, HydratedScenarioPack, RawDocument,
};
use simthing_core::{
    kind_identity, seed_profiles, derive_specializations, DimensionRegistry, KindIdentity,
    SimProperty, SimThing, SimThingKind, SimThingKindTag, SpecializationError,
    SpecializationObservations, SpecializationRequirement, PROFILE_OWNER_SEAT, PROFILE_SESSION_ROOT,
    PROFILE_SPATIAL,
};
use simthing_driver::{preview_install, Scenario};
use simthing_gpu::SlotAllocator;
use simthing_spec::{
    scenario_metadata_u32_value, GameModeSpec, SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
    SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn hydrate_canonical() -> HydratedScenarioPack {
    let clause_path = repo_root().join("scenarios/terran_pirate_galaxy.clause");
    let text = std::fs::read_to_string(&clause_path).expect("read canonical clause");
    let document = parse_raw_document(text.as_bytes()).expect("parse canonical clause");
    let base = clause_path.parent().expect("clause parent").to_path_buf();
    simthing_clausething::hydrate_scenario_with_source_base(&document, Some(&base))
        .expect("hydrate canonical clause")
}

fn minimal_scenario(root: SimThing) -> Scenario {
    let mut registry = DimensionRegistry::new();
    let _ = registry.register(SimProperty::simple("_session", "seed", 0));
    Scenario {
        name: "first_citizen_specialists_0".into(),
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

fn find_location_mut<'a>(root: &'a mut SimThing, id: u32) -> Option<&'a mut SimThing> {
    if root.id.raw() == id {
        return Some(root);
    }
    for child in &mut root.children {
        if let Some(found) = find_location_mut(child, id) {
            return Some(found);
        }
    }
    None
}

const CLAUSE_LOCATION_SPATIAL: &str = r#"
scenario = location_spatial_proof {
    location = placed_cell {
        display_name = "Placed Cell"
        specialization = spatial
    }
}
"#;

const CLAUSE_ENTITY_OWNER_SEAT: &str = r#"
scenario = entity_seat_proof {
    location = anchor_cell {
        display_name = "Anchor Cell"
        children = {
            child = fleet_impostor {
                kind = Fleet
                display_name = "Fleet Impostor"
                specialization = owner-seat
            }
        }
    }
}
"#;

#[test]
fn authored_location_spatial_admits_when_placed_and_spans_when_unplaced() {
    let document = parse_raw_document(CLAUSE_LOCATION_SPATIAL.as_bytes()).expect("parse");
    let expected_token = declared_scalar_token(&document);
    let pack = hydrate_scenario(&document).expect("hydrate");
    let location_id = pack
        .root_node
        .children
        .iter()
        .find(|n| n.id == "placed_cell")
        .expect("authored location")
        .simthing_id
        .raw();

    // Negative: unplaced Location declaring spatial → StructurallyPlaced + span.
    let unplaced_root = pack.root.clone();
    let error = derive_specializations(
        &unplaced_root,
        &seed_profiles(),
        &SpecializationObservations::default(),
    )
    .expect_err("unplaced Location declaring spatial must fail");
    match error {
        SpecializationError::RequirementUnmet {
            simthing,
            kind,
            profile,
            requirement,
            span_token,
        } => {
            assert_eq!(simthing, location_id);
            assert_eq!(kind, kind_identity(&SimThingKind::Location));
            assert_eq!(profile, PROFILE_SPATIAL);
            assert_eq!(requirement, SpecializationRequirement::StructurallyPlaced);
            assert_eq!(span_token, Some(expected_token));
        }
        other => panic!("expected RequirementUnmet, got {other:?}"),
    }

    // Positive: stamp the grid_metadata placement artifact onto the Location,
    // then ordinary install admits and derives spatial.
    let placement = pack
        .grid_metadata
        .placements
        .iter()
        .find(|p| p.location_id == "placed_cell")
        .expect("authored location has a grid placement");
    let mut placed_root = pack.root.clone();
    let location = find_location_mut(&mut placed_root, location_id).expect("location in tree");
    location.add_property(
        SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
        scenario_metadata_u32_value(placement.col),
    );
    location.add_property(
        SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
        scenario_metadata_u32_value(placement.row),
    );

    let scenario = minimal_scenario(placed_root);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&scenario.root);
    let preview = preview_install(
        &GameModeSpec::default(),
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
    )
    .expect("placed Location declaring spatial admits");
    assert!(preview
        .state
        .specialization
        .derived_ids(location_id)
        .contains(&PROFILE_SPATIAL));
}

#[test]
fn authored_entity_owner_seat_spans_kind_unmet_with_exact_token() {
    let document = parse_raw_document(CLAUSE_ENTITY_OWNER_SEAT.as_bytes()).expect("parse");
    let expected_token = declared_scalar_token(&document);
    let pack = hydrate_scenario(&document).expect("hydrate");
    let fleet_id = pack
        .root_node
        .children
        .iter()
        .flat_map(|loc| loc.children.iter())
        .find(|n| n.id == "fleet_impostor")
        .expect("authored entity")
        .simthing_id
        .raw();

    let error = derive_specializations(
        &pack.root,
        &seed_profiles(),
        &SpecializationObservations::default(),
    )
    .expect_err("Fleet declaring owner-seat must fail");
    match error {
        SpecializationError::RequirementUnmet {
            simthing,
            kind,
            profile,
            requirement,
            span_token,
        } => {
            assert_eq!(simthing, fleet_id);
            assert_eq!(kind, kind_identity(&SimThingKind::Fleet));
            assert_eq!(profile, PROFILE_OWNER_SEAT);
            assert_eq!(
                requirement,
                SpecializationRequirement::Kind(KindIdentity::BuiltIn(SimThingKindTag::Owner)),
            );
            assert_eq!(span_token, Some(expected_token));
        }
        other => panic!("expected RequirementUnmet, got {other:?}"),
    }
}

fn load_citizen_counts_tsv() -> BTreeMap<String, usize> {
    let path = repo_root().join("scripts/ci/specialization_citizen_counts.tsv");
    let text = std::fs::read_to_string(&path).expect("citizen counts TSV present");
    let mut out = BTreeMap::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split('\t');
        let profile = parts.next().expect("profile").to_string();
        let count: usize = parts
            .next()
            .expect("count")
            .parse()
            .expect("count parses");
        out.insert(profile, count);
    }
    out
}

#[test]
fn canonical_citizen_counts_match_generator_source_tsv() {
    let pack = hydrate_canonical();
    let authority = pack.authority_root.as_ref().expect("authority root");
    let scenario = minimal_scenario(authority.clone());
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&scenario.root);
    let preview = preview_install(
        &GameModeSpec::default(),
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
    )
    .expect("canonical authority tree admits");
    let live = preview.state.specialization.citizen_counts();
    let expected = load_citizen_counts_tsv();
    assert_eq!(
        live.spatial,
        expected["spatial"],
        "spatial citizen count must match generator-source TSV"
    );
    assert_eq!(
        live.owner_seat,
        expected["owner-seat"],
        "owner-seat citizen count must match generator-source TSV"
    );
    assert_eq!(
        live.session_root,
        expected["session-root"],
        "session-root citizen count must match generator-source TSV"
    );
    assert!(live.spatial > 0);
    assert!(live.owner_seat >= 2);
    assert_eq!(live.session_root, 1);
    assert_eq!(expected[PROFILE_SPATIAL], live.spatial);
    assert_eq!(expected[PROFILE_SESSION_ROOT], live.session_root);
    assert_eq!(expected[PROFILE_OWNER_SEAT], live.owner_seat);
}

#[test]
fn board_and_orientation_render_citizen_counts() {
    let expected = load_citizen_counts_tsv();
    let render_line = format!(
        "spatial={} owner-seat={} session-root={}",
        expected["spatial"], expected["owner-seat"], expected["session-root"]
    );

    // Orientation is generator-sourced — after regen it must carry the TSV counts.
    let orientation = std::fs::read_to_string(repo_root().join("docs/orchestrator_orientation.md"))
        .expect("orientation digest present");
    assert!(
        orientation.contains(&render_line),
        "orientation must render citizen counts from specialization_citizen_counts.tsv; missing `{render_line}`"
    );
    assert!(
        orientation.contains("## Specialization citizens"),
        "orientation must carry the Specialization citizens section"
    );

    // Board markdown line shape (same TSV → generator path as handoff_dispatch).
    let board_line = format!(
        "- specialization_citizens: spatial={} owner-seat={} session-root={}",
        expected["spatial"], expected["owner-seat"], expected["session-root"]
    );
    assert_eq!(
        board_line,
        "- specialization_citizens: spatial=1500 owner-seat=2 session-root=1"
    );
}
