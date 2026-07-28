//! FIRST-CITIZEN-SPECIALISTS-0 (0.0.8.7 rung 3.2) referee.
//!
//! Legs:
//! 1. Authored Location `specialization = spatial` admits through ordinary
//!    hydrate→preview_install when system_target-enrolled (hydration stamps
//!    structural col/row); unplaced fails `StructurallyPlaced` with the exact
//!    scalar token via the same install error path;
//! 2. Authored entity declaring `owner-seat` fails through ordinary install
//!    with `Kind(Owner)` and the exact scalar token;
//! 3. Canonical installed citizen counts match the generator-source TSV
//!    produced by `scripts/ci/gen_specialization_citizen_counts.sh` (never a
//!    hand-edited mirror); board/orientation render requires generator freshness.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use simthing_clausething::raw::RawValue;
use simthing_clausething::{
    hydrate_scenario, parse_raw_document, HydratedScenarioPack, RawDocument,
};
use simthing_core::{
    kind_identity, DimensionRegistry, KindIdentity, SimProperty, SimThing, SimThingKind,
    SimThingKindTag, SpecializationError, SpecializationRequirement, PROFILE_OWNER_SEAT,
    PROFILE_SESSION_ROOT, PROFILE_SPATIAL,
};
use simthing_driver::{preview_install, InstallError, InstallPreview, Scenario};
use simthing_gpu::SlotAllocator;
use simthing_spec::GameModeSpec;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn base_disc_fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-mapeditor/tests/fixtures/tp_base_disc_1500.simthing-scenario.json")
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

fn preview_err(root: SimThing) -> InstallError {
    let scenario = minimal_scenario(root);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&scenario.root);
    preview_install(
        &GameModeSpec::default(),
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
    )
    .expect_err("expected specialization install failure")
}

fn preview_ok(root: SimThing) -> InstallPreview {
    let scenario = minimal_scenario(root);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&scenario.root);
    preview_install(
        &GameModeSpec::default(),
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
    )
    .expect("expected specialization install success")
}

fn clause_location_spatial_placed() -> String {
    let path = base_disc_fixture_path()
        .to_string_lossy()
        .replace('\\', "/");
    format!(
        r#"
scenario = location_spatial_placed_proof {{
    static_galaxy_scenario = base_disc {{
        namespace = "fcs"
        source_json = "{path}"
        map_quality_status = PASS
    }}
    location = placed_cell {{
        display_name = "Placed Cell"
        system_target = "row199_col80"
        specialization = spatial
    }}
}}
"#
    )
}

const CLAUSE_LOCATION_SPATIAL_UNPLACED: &str = r#"
scenario = location_spatial_unplaced_proof {
    location = unplaced_cell {
        display_name = "Unplaced Cell"
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
    // Positive: ordinary hydrate of system_target-enrolled Location + spatial
    // declaration — production hydration stamps structural col/row; preview_install.
    let placed_src = clause_location_spatial_placed();
    let placed_doc = parse_raw_document(placed_src.as_bytes()).expect("parse placed");
    let placed_token = declared_scalar_token(&placed_doc);
    let placed_pack = hydrate_scenario(&placed_doc).expect("hydrate placed");
    let location_id = placed_pack
        .root_node
        .children
        .iter()
        .find(|n| n.id == "placed_cell")
        .expect("authored location")
        .simthing_id
        .raw();
    assert!(
        placed_pack
            .root_node
            .children
            .iter()
            .find(|n| n.id == "placed_cell")
            .expect("authored location")
            .system_target
            .is_some(),
        "positive fixture must enroll via system_target"
    );
    let preview = preview_ok(placed_pack.root.clone());
    assert!(
        preview
            .state
            .specialization
            .derived_ids(location_id)
            .contains(&PROFILE_SPATIAL),
        "placed authored Location declaring spatial must derive spatial via ordinary install"
    );
    assert_eq!(
        placed_pack
            .root
            .children
            .iter()
            .find(|n| n.id.raw() == location_id)
            .expect("location on root")
            .declared_specializations
            .first()
            .map(|d| d.span_token),
        Some(Some(placed_token))
    );

    // Negative: authored Location without system_target stays unstamped;
    // ordinary install fails StructurallyPlaced with the exact scalar token.
    let unplaced_doc =
        parse_raw_document(CLAUSE_LOCATION_SPATIAL_UNPLACED.as_bytes()).expect("parse unplaced");
    let unplaced_token = declared_scalar_token(&unplaced_doc);
    let unplaced_pack = hydrate_scenario(&unplaced_doc).expect("hydrate unplaced");
    let unplaced_id = unplaced_pack
        .root_node
        .children
        .iter()
        .find(|n| n.id == "unplaced_cell")
        .expect("authored unplaced location")
        .simthing_id
        .raw();
    match preview_err(unplaced_pack.root) {
        InstallError::Specialization(SpecializationError::RequirementUnmet {
            simthing,
            kind,
            profile,
            requirement,
            span_token,
        }) => {
            assert_eq!(simthing, unplaced_id);
            assert_eq!(kind, kind_identity(&SimThingKind::Location));
            assert_eq!(profile, PROFILE_SPATIAL);
            assert_eq!(requirement, SpecializationRequirement::StructurallyPlaced);
            assert_eq!(span_token, Some(unplaced_token));
        }
        other => panic!("expected RequirementUnmet via install, got {other:?}"),
    }
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

    match preview_err(pack.root) {
        InstallError::Specialization(SpecializationError::RequirementUnmet {
            simthing,
            kind,
            profile,
            requirement,
            span_token,
        }) => {
            assert_eq!(simthing, fleet_id);
            assert_eq!(kind, kind_identity(&SimThingKind::Fleet));
            assert_eq!(profile, PROFILE_OWNER_SEAT);
            assert_eq!(
                requirement,
                SpecializationRequirement::Kind(KindIdentity::BuiltIn(SimThingKindTag::Owner)),
            );
            assert_eq!(span_token, Some(expected_token));
        }
        other => panic!("expected RequirementUnmet via install, got {other:?}"),
    }
}

fn load_citizen_counts_tsv(path: &Path) -> BTreeMap<String, usize> {
    let text = std::fs::read_to_string(path).expect("citizen counts TSV present");
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

fn live_citizen_counts() -> (usize, usize, usize) {
    let pack = hydrate_canonical();
    let authority = pack.authority_root.as_ref().expect("authority root");
    let preview = preview_ok(authority.clone());
    let counts = preview.state.specialization.citizen_counts();
    (counts.spatial, counts.owner_seat, counts.session_root)
}

fn render_citizen_counts_tsv(spatial: usize, owner_seat: usize, session_root: usize) -> String {
    format!(
        "\
# profile\tcount\tbasis  # FIRST-CITIZEN-SPECIALISTS-0; GENERATED — do not hand-edit
# Counts = SpecSessionState.specialization.citizen_counts() over the canonical TP authority install.
# Regenerate: bash scripts/ci/gen_specialization_citizen_counts.sh
spatial\t{spatial}\tcanonical TP authority install — structurally placed Locations
owner-seat\t{owner_seat}\tcanonical TP authority install — Terran + Pirate policy/weight authorities
session-root\t{session_root}\tcanonical TP authority install — sole GameSession root
"
    )
}

fn assert_tsv_matches_live(path: &Path) -> Result<(), String> {
    if !path.is_file() {
        return Err(format!("missing citizen-counts TSV: {}", path.display()));
    }
    let expected = load_citizen_counts_tsv(path);
    let (spatial, owner_seat, session_root) = live_citizen_counts();
    let mut errs = Vec::new();
    if expected.get("spatial").copied() != Some(spatial) {
        errs.push(format!(
            "spatial: tsv={:?} live={spatial}",
            expected.get("spatial")
        ));
    }
    if expected.get("owner-seat").copied() != Some(owner_seat) {
        errs.push(format!(
            "owner-seat: tsv={:?} live={owner_seat}",
            expected.get("owner-seat")
        ));
    }
    if expected.get("session-root").copied() != Some(session_root) {
        errs.push(format!(
            "session-root: tsv={:?} live={session_root}",
            expected.get("session-root")
        ));
    }
    if errs.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "stale specialization_citizen_counts.tsv ({}); run bash scripts/ci/gen_specialization_citizen_counts.sh",
            errs.join("; ")
        ))
    }
}

/// Invoked by `scripts/ci/gen_specialization_citizen_counts.sh` only.
#[test]
#[ignore = "generator CLI for scripts/ci/gen_specialization_citizen_counts.sh"]
fn generator_cli() {
    let out = PathBuf::from(
        std::env::var("FCS_CITIZEN_COUNTS_OUT")
            .unwrap_or_else(|_| "scripts/ci/specialization_citizen_counts.tsv".into()),
    );
    let mode = std::env::var("FCS_CITIZEN_COUNTS_MODE").unwrap_or_else(|_| "write".into());
    match mode.as_str() {
        "check" => {
            if let Err(err) = assert_tsv_matches_live(&out) {
                panic!("{err}");
            }
            eprintln!("gen_specialization_citizen_counts --check: PASS ({})", out.display());
        }
        "write" => {
            let (spatial, owner_seat, session_root) = live_citizen_counts();
            if let Some(parent) = out.parent() {
                std::fs::create_dir_all(parent).expect("create output parent");
            }
            std::fs::write(&out, render_citizen_counts_tsv(spatial, owner_seat, session_root))
                .expect("write citizen counts TSV");
            eprintln!(
                "gen_specialization_citizen_counts: wrote {} (spatial={spatial} owner-seat={owner_seat} session-root={session_root})",
                out.display()
            );
        }
        other => panic!("unknown FCS_CITIZEN_COUNTS_MODE={other}"),
    }
}

#[test]
fn canonical_citizen_counts_match_generator_source_tsv() {
    let tsv_path = repo_root().join("scripts/ci/specialization_citizen_counts.tsv");
    assert_tsv_matches_live(&tsv_path).expect("generator TSV must match live install");
    let expected = load_citizen_counts_tsv(&tsv_path);
    assert!(expected["spatial"] > 0);
    assert!(expected["owner-seat"] >= 2);
    assert_eq!(expected["session-root"], 1);
    assert_eq!(expected[PROFILE_SPATIAL], expected["spatial"]);
    assert_eq!(expected[PROFILE_SESSION_ROOT], expected["session-root"]);
    assert_eq!(expected[PROFILE_OWNER_SEAT], expected["owner-seat"]);
}

#[test]
fn generator_check_fails_when_citizen_count_corrupted() {
    // Escaped-bug: a hand-corrupted mirror must fail the generator freshness gate.
    let dir = std::env::temp_dir().join(format!("fcs_corrupt_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp dir");
    let corrupted = dir.join("specialization_citizen_counts.tsv");
    std::fs::write(
        &corrupted,
        "# corrupted mirror\nspatial\t0\tbad\nowner-seat\t0\tbad\nsession-root\t0\tbad\n",
    )
    .expect("write corrupted tsv");
    let err = assert_tsv_matches_live(&corrupted).expect_err("corrupted TSV must fail freshness");
    assert!(
        err.contains("stale specialization_citizen_counts.tsv"),
        "unexpected freshness error: {err}"
    );
}

#[test]
fn board_and_orientation_render_citizen_counts() {
    // Actual generator-path freshness (same assert the shell --check uses).
    let tsv_path = repo_root().join("scripts/ci/specialization_citizen_counts.tsv");
    assert_tsv_matches_live(&tsv_path).expect("generator freshness required for board/orientation");

    let expected = load_citizen_counts_tsv(&tsv_path);
    let render_line = format!(
        "spatial={} owner-seat={} session-root={}",
        expected["spatial"], expected["owner-seat"], expected["session-root"]
    );

    let orientation = std::fs::read_to_string(repo_root().join("docs/orchestrator_orientation.md"))
        .expect("orientation digest present");
    assert!(
        orientation.contains(&render_line),
        "orientation must render citizen counts from generator TSV; missing `{render_line}`"
    );
    assert!(
        orientation.contains("## Specialization citizens"),
        "orientation must carry the Specialization citizens section"
    );

    // Board markdown shape matches handoff_dispatch's generator-TSV summary line.
    let board_line = format!("- specialization_citizens: {render_line}");
    assert_eq!(
        board_line,
        format!(
            "- specialization_citizens: spatial={} owner-seat={} session-root={}",
            expected["spatial"], expected["owner-seat"], expected["session-root"]
        )
    );
}
