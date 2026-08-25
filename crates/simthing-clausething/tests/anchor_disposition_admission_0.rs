//! ANCHOR-DISPOSITION-ADMISSION-0 (0.0.8.7 rung 5.1) referee.
//!
//! Proves the P0(e) admission fulcrum without implementing the future write
//! door, GPU anchor table, or any observation consumer:
//! - omission is Anchored through hydrate -> compile -> install report;
//! - authored Unobserved retains a non-empty reason and scalar source span;
//! - blank reasons fail at hydration with that source span;
//! - a minimal fixture install has one total disposition per resource property;
//! - Board/orientation inventory is generated from that live installed report.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use simthing_clausething::raw::RawValue;
use simthing_clausething::{
    hydrate_resource_flow_pack, parse_raw_document, HydratedScenarioPack, RawDocument,
};
use simthing_core::{
    DimensionRegistry, PropertyAdmissionDisposition, PropertyAdmissionReport, SimThing,
    SimThingKind,
};
use simthing_driver::{preview_install, InstallPreview, Scenario};
use simthing_gpu::SlotAllocator;

const MICRO_ECONOMY: &str = include_str!("fixtures/ct2a_micro_economy.clause");
const FIXTURE_SCENARIO: &str = include_str!("fixtures/disposition_admission_minimal.clause");

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn micro_preview(source: &str) -> (RawDocument, InstallPreview) {
    let document = parse_raw_document(source.as_bytes()).expect("parse micro economy");
    let mut pack = hydrate_resource_flow_pack(&document).expect("hydrate micro economy");
    // The disposition contract is property admission. This minimal install
    // deliberately omits RF enrollment because no hosted values are needed.
    pack.game_mode.resource_flow = None;

    let registry = DimensionRegistry::new();
    let root = SimThing::new(SimThingKind::World, 0);
    let scenario = Scenario {
        name: "anchor_disposition_admission_0".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 8,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::new(),
    };
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&scenario.root);
    let preview = preview_install(
        &pack.game_mode,
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
    )
    .expect("ordinary preview install");
    (document, preview)
}

fn with_disposition(reason: &str) -> String {
    MICRO_ECONOMY.replacen(
        "        display_name = \"Food Flow\"",
        &format!(
            "        display_name = \"Food Flow\"\n        disposition = Unobserved {{ reason = \"{reason}\" }}"
        ),
        1,
    )
}

fn first_scalar_token(document: &RawDocument, key: &str) -> usize {
    fn visit(value: &RawValue, key: &str, out: &mut Option<usize>) {
        match value {
            RawValue::Block(block) => {
                for property in &block.properties {
                    if property.key.text == key {
                        if let RawValue::Scalar(scalar) = &property.value {
                            *out = Some(scalar.span.token_index);
                            return;
                        }
                    }
                    visit(&property.value, key, out);
                    if out.is_some() {
                        return;
                    }
                }
            }
            RawValue::Array(array) => {
                for value in &array.items {
                    visit(value, key, out);
                    if out.is_some() {
                        return;
                    }
                }
            }
            RawValue::Header(header) => visit(&header.payload, key, out),
            RawValue::Scalar(_) => {}
        }
    }
    let mut out = None;
    visit(&document.root, key, &mut out);
    out.expect("requested scalar token exists")
}

fn render_dark_inventory(report: &PropertyAdmissionReport) -> String {
    let mut dark = report.dark_properties().collect::<Vec<_>>();
    dark.sort_by_key(|row| row.canonical_identity());
    let mut rendered = format!(
        "# ANCHOR-DISPOSITION-ADMISSION-0; GENERATED - do not hand-edit\n\
         # Counts = SpecSessionState.property_admission over a MINIMAL FIXTURE install.\n\
         # Regenerate: bash scripts/ci/gen_property_admission_inventory.sh\n\
         summary\tanchored\t{}\n\
         summary\tunobserved\t{}\n\
         summary\ttotal\t{}\n",
        report.anchored_count(),
        report.unobserved_count(),
        report.resource_properties.len()
    );
    for row in dark {
        let PropertyAdmissionDisposition::Unobserved {
            reason,
            source_span_token,
        } = &row.disposition
        else {
            unreachable!("dark_properties returns only Unobserved rows");
        };
        rendered.push_str(&format!(
            "dark\t{}\t{}\t{}\n",
            row.canonical_identity(),
            reason.replace(['\t', '\n', '\r'], " "),
            source_span_token
        ));
    }
    rendered
}

/// Install used by the disposition referee.
///
/// This is deliberately a MINIMAL FIXTURE, not a shipped scenario. The law
/// under test is a DISPOSITION law — every property an install produces is
/// either anchored or explicitly declared dark, and none is silently dropped —
/// and that holds for ANY install, so the smallest one is a complete witness.
///
/// It previously read `scenarios/` and asserted the directory held exactly one
/// canonical clause. That made a shipped game scenario a STRUCTURAL REQUIREMENT
/// OF THE BUILD, which is backwards: scenarios are external assets, and the
/// engine must stand without any of them. Worse, it pumped that scenario's
/// property vocabulary into `property_admission_inventory.tsv`, which feeds the
/// orientation digest and `handoff_dispatch.sh` — so a disposable rehearsal's
/// names reached every agent as ambient fact, annotated with forward
/// obligations on future rungs. That is how a purged scenario returns: as a
/// fixture, then a proof, then code.
fn canonical_pack() -> HydratedScenarioPack {
    let document = parse_raw_document(FIXTURE_SCENARIO.as_bytes()).expect("parse fixture clause");
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    simthing_clausething::hydrate_scenario_with_source_base(&document, Some(&base))
        .expect("hydrate fixture clause")
}

fn canonical_preview() -> InstallPreview {
    let pack = canonical_pack();
    // Install the hydrated fixture property set through the ordinary
    // compile/install door. Unrelated economy/overlay execution is excluded so
    // this signal-only referee judges disposition rather than host wiring.
    let game_mode = simthing_spec::GameModeSpec {
        id: pack.game_mode.id.clone(),
        display_name: pack.game_mode.display_name.clone(),
        properties: pack.game_mode.properties.clone(),
        ..Default::default()
    };
    let root = pack.root.clone();
    let registry = DimensionRegistry::new();
    let scenario = Scenario {
        name: pack.scenario_id.clone(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: (root.subtree_size() as u32).saturating_add(2048),
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::new(),
    };
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&scenario.root);
    preview_install(
        &game_mode,
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
    )
    .expect("fixture ordinary preview install")
}

fn live_canonical_inventory() -> String {
    render_dark_inventory(&canonical_preview().state.property_admission)
}

fn assert_inventory_matches_live(path: &Path) -> Result<(), String> {
    let expected = live_canonical_inventory();
    let actual = std::fs::read_to_string(path).map_err(|err| {
        format!(
            "missing property-admission inventory {}: {err}",
            path.display()
        )
    })?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "stale property_admission_inventory.tsv ({}); run bash scripts/ci/gen_property_admission_inventory.sh",
            path.display()
        ))
    }
}

#[test]
fn omitted_resource_property_defaults_anchored_through_install_report() {
    let (_, preview) = micro_preview(MICRO_ECONOMY);
    let report = &preview.state.property_admission;
    assert_eq!(report.resource_properties.len(), 1);
    assert_eq!(report.anchored_count(), 1);
    assert_eq!(report.unobserved_count(), 0);
    assert_eq!(
        report.resource_properties[0].canonical_identity(),
        "simthing::food_flow"
    );
    assert!(
        !report.resource_properties[0].roles.is_empty(),
        "install report must retain the registry role pathway"
    );
    assert_eq!(
        report.resource_properties[0].disposition,
        PropertyAdmissionDisposition::Anchored
    );

    let property = preview
        .registry
        .properties
        .iter()
        .find(|property| property.name == "food_flow")
        .expect("compiled resource property");
    let json = serde_json::to_string(property).expect("serialize property");
    assert!(
        !json.contains("admission_disposition"),
        "default Anchored must not change existing serialized bytes"
    );
}

#[test]
fn explicit_unobserved_preserves_reason_span_and_dark_inventory() {
    let source = with_disposition("sealed external feed");
    let (document, preview) = micro_preview(&source);
    let expected_span = first_scalar_token(&document, "reason");
    let report = &preview.state.property_admission;
    assert_eq!(report.resource_properties.len(), 1);
    assert_eq!(report.anchored_count(), 0);
    assert_eq!(report.unobserved_count(), 1);
    assert_eq!(
        report.resource_properties[0].disposition,
        PropertyAdmissionDisposition::Unobserved {
            reason: "sealed external feed".into(),
            source_span_token: expected_span,
        }
    );
    let inventory = render_dark_inventory(report);
    assert!(inventory.contains("summary\tunobserved\t1"));
    assert!(inventory.contains(&format!(
        "dark\tsimthing::food_flow\tsealed external feed\t{expected_span}"
    )));
}

#[test]
fn blank_unobserved_reason_hard_errors_at_scalar_span() {
    let source = with_disposition("   ");
    let document = parse_raw_document(source.as_bytes()).expect("parse blank-reason fixture");
    let expected_span = first_scalar_token(&document, "reason");
    let err = hydrate_resource_flow_pack(&document).expect_err("blank reason must fail hydration");
    assert_eq!(
        err.span.as_ref().map(|span| span.token_index),
        Some(expected_span)
    );
    assert!(err.message.contains("must be non-empty"));
}

/// Corrected totality semantics (admission-governs-existence): closed Anchored/Unobserved
/// partition with derived counts published; no fixed 18/7 target.
fn assert_canonical_tp_disposition_admission_totality() {
    let preview = canonical_preview();
    let report = &preview.state.property_admission;
    assert!(
        !report.resource_properties.is_empty(),
        "fixture install must admit resource-bearing properties"
    );
    assert_eq!(
        report.resource_properties.len(),
        report.anchored_count() + report.unobserved_count(),
        "closed type yields exactly one disposition per resource property"
    );
    eprintln!(
        "FIXTURE DISPOSITION (derived): Anchored={} Unobserved={}",
        report.anchored_count(),
        report.unobserved_count()
    );
    assert!(
        report.anchored_count() > 0,
        "fixture install must admit Anchored resource properties"
    );
    assert!(
        report.unobserved_count() > 0,
        "fixture install must admit Unobserved dark cells where hostless"
    );
    assert_eq!(
        report.resource_properties,
        preview
            .registry
            .property_admission_report()
            .resource_properties,
        "install report is projected from the canonical live registry"
    );
    for (index, row) in report.resource_properties.iter().enumerate() {
        assert_eq!(row.property_id.index(), index);
        assert!(
            !row.roles.is_empty(),
            "resource property has no role pathway"
        );
    }
}

/// Protected inventory identity (birth track 0.0.8.7); wrapper over corrected totality helper.
#[test]
fn canonical_tp_install_has_total_default_anchored_disposition() {
    assert_canonical_tp_disposition_admission_totality();
}

/// Invoked only by scripts/ci/gen_property_admission_inventory.sh.
#[test]
#[ignore = "generator CLI for property_admission_inventory.tsv"]
fn generator_cli() {
    let output = PathBuf::from(
        std::env::var("PROPERTY_ADMISSION_INVENTORY_OUT")
            .unwrap_or_else(|_| "scripts/ci/property_admission_inventory.tsv".into()),
    );
    let mode =
        std::env::var("PROPERTY_ADMISSION_INVENTORY_MODE").unwrap_or_else(|_| "write".into());
    match mode.as_str() {
        "check" => {
            assert_inventory_matches_live(&output).unwrap_or_else(|err| panic!("{err}"));
            eprintln!(
                "gen_property_admission_inventory --check: PASS ({})",
                output.display()
            );
        }
        "write" => {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).expect("create inventory output parent");
            }
            std::fs::write(&output, live_canonical_inventory())
                .expect("write property-admission inventory");
            eprintln!(
                "gen_property_admission_inventory: wrote {}",
                output.display()
            );
        }
        other => panic!("unknown PROPERTY_ADMISSION_INVENTORY_MODE={other}"),
    }
}

#[test]
fn canonical_inventory_matches_generator_source_tsv() {
    let path = repo_root().join("scripts/ci/property_admission_inventory.tsv");
    assert_inventory_matches_live(&path).expect("generator TSV must match live install");
}

#[test]
fn board_and_orientation_render_property_admission_inventory() {
    let path = repo_root().join("scripts/ci/property_admission_inventory.tsv");
    assert_inventory_matches_live(&path).expect("generator freshness required");
    let inventory = std::fs::read_to_string(path).expect("read generated inventory");
    let summary = inventory
        .lines()
        .filter(|line| line.starts_with("summary\t"))
        .map(|line| {
            let mut parts = line.split('\t');
            let _ = parts.next();
            let key = parts.next().expect("summary key");
            let value = parts.next().expect("summary value");
            (key, value)
        })
        .collect::<HashMap<_, _>>();
    let render_line = format!(
        "anchored={} unobserved={} total={}",
        summary["anchored"], summary["unobserved"], summary["total"]
    );
    let orientation = std::fs::read_to_string(repo_root().join("docs/orchestrator_orientation.md"))
        .expect("orientation digest present");
    assert!(orientation.contains("## Live install inventories"));
    assert!(
        orientation.contains(&format!("Property admission: {render_line}")),
        "orientation must render generated admission counts: `{render_line}`"
    );

    let bash = if cfg!(windows) {
        PathBuf::from(std::env::var_os("ProgramFiles").expect("ProgramFiles on Windows"))
            .join("Git/bin/bash.exe")
    } else {
        PathBuf::from("bash")
    };
    let board = std::process::Command::new(bash)
        .args([
            "scripts/ci/handoff_dispatch.sh",
            "--board-json",
            "handoffs/ANCHOR-DISPOSITION-ADMISSION-0.hd.md",
        ])
        .current_dir(repo_root())
        .output()
        .expect("execute Board JSON renderer");
    assert!(
        board.status.success(),
        "Board JSON renderer failed ({:?}): stdout={} stderr={}",
        board.status.code(),
        String::from_utf8_lossy(&board.stdout),
        String::from_utf8_lossy(&board.stderr)
    );
    let board: serde_json::Value = serde_json::from_slice(&board.stdout).expect("parse Board JSON");
    // The Board must report the pointer the design doc CURRENTLY names. This
    // previously hard-coded a literal rung id, which made the referee go red at
    // every graduation -- a false-red generator, and it was already red on
    // master (expected CANONICAL-ANCHOR-MATERIALIZATION-0, actual
    // COMPARATIVE-DEFAULT-BIRTH-0). Compare against the doc so the assertion
    // tracks the Two-Source Pointer Rule instead of drifting away from it.
    let design =
        std::fs::read_to_string(repo_root().join("docs/design_0_0_8_7_rf_arena_modernization.md"))
            .expect("read active design doc");
    let expected_pointer = design
        .lines()
        .find(|line| line.starts_with("| Active open rung |"))
        .and_then(|line| line.split('`').nth(1))
        .expect("design doc names an active open rung")
        .to_string();
    assert_eq!(
        board["active_pointer"],
        serde_json::json!(expected_pointer),
        "Board pointer must equal the design doc's active open rung"
    );
    // Board inventory mirrors the live regenerated TSV (derived counts, not targets).
    let live = canonical_preview().state.property_admission;
    assert_eq!(
        board["property_admission"]["anchored"],
        live.anchored_count() as u64
    );
    assert_eq!(
        board["property_admission"]["unobserved"],
        live.unobserved_count() as u64
    );
    assert_eq!(
        board["property_admission"]["total"],
        live.resource_properties.len() as u64
    );
    let dark = board["property_admission"]["dark"]
        .as_array()
        .expect("property_admission.dark array");
    assert_eq!(
        dark.len(),
        live.unobserved_count(),
        "Board dark inventory must list every Unobserved cell from live admission"
    );
}
