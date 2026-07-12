//! STUDIO-STAR-NAMING-REPAIR-0 canonical authority/data proofs.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use simthing_clausething::{
    hydrate_scenario_with_source_base, parse_raw_document, rebind_pack_to_structural_rebind_ready,
};
use simthing_mapgenerator::assign_star_names;
use simthing_spec::{
    apply_star_system_display_name_metadata, gridcell_generated_system_id, is_owner_entity_kind,
    load_scenario_spec_from_json_str, owner_entity_id, owner_faction_alliance,
    owner_faction_color_rgb, owner_faction_display_name, resolve_map_container_mut,
    save_scenario_spec_to_canonical_json, star_system_display_name, SimThingScenarioSpec,
};

const TP_SEED: u64 = 770_421;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

fn canonical_base_path() -> PathBuf {
    repo_root().join("scenarios/terran_pirate_galaxy.base_disc.json")
}

fn canonical_clause_path() -> PathBuf {
    repo_root().join("scenarios/terran_pirate_galaxy.clause")
}

fn load_canonical_base() -> SimThingScenarioSpec {
    let path = canonical_base_path();
    let json = std::fs::read_to_string(&path).expect("read canonical base disc");
    load_scenario_spec_from_json_str(path.to_str().expect("path"), &json)
        .expect("load canonical base disc")
        .0
}

fn load_through_clause() -> SimThingScenarioSpec {
    let path = canonical_clause_path();
    let text = std::fs::read_to_string(&path).expect("read canonical clause");
    let document = parse_raw_document(text.as_bytes()).expect("parse canonical clause");
    let pack = hydrate_scenario_with_source_base(&document, path.parent())
        .expect("hydrate canonical clause with sibling source_base");
    rebind_pack_to_structural_rebind_ready(&pack)
        .expect("structural rebind")
        .0
}

fn expected_names(spec: &SimThingScenarioSpec) -> BTreeMap<u32, String> {
    assign_star_names(
        TP_SEED,
        spec.gridcell_locations()
            .filter_map(gridcell_generated_system_id),
    )
    .into_iter()
    .map(|assignment| (assignment.system_id, assignment.display_name))
    .collect()
}

fn apply_expected_names(spec: &mut SimThingScenarioSpec) {
    let expected = expected_names(spec);
    let map = resolve_map_container_mut(spec).expect("map container");
    for system in &mut map.children {
        if let Some(system_id) = gridcell_generated_system_id(system) {
            apply_star_system_display_name_metadata(
                system,
                expected.get(&system_id).expect("stable name assignment"),
            );
        }
    }
}

fn resolved_names(spec: &SimThingScenarioSpec) -> Vec<(u32, String)> {
    spec.gridcell_locations()
        .map(|system| {
            (
                gridcell_generated_system_id(system).expect("system id"),
                star_system_display_name(system).expect("persisted display name"),
            )
        })
        .collect()
}

fn owner_identity(spec: &SimThingScenarioSpec) -> BTreeSet<(String, String, (u8, u8, u8), String)> {
    fn collect(
        node: &simthing_core::SimThing,
        out: &mut BTreeSet<(String, String, (u8, u8, u8), String)>,
    ) {
        if is_owner_entity_kind(&node.kind) {
            out.insert((
                owner_entity_id(node).expect("owner id"),
                owner_faction_display_name(node).expect("faction name"),
                owner_faction_color_rgb(node).expect("faction color"),
                owner_faction_alliance(node).expect("faction alliance"),
            ));
        }
        for child in &node.children {
            collect(child, out);
        }
    }
    let mut out = BTreeSet::new();
    collect(&spec.root, &mut out);
    out
}

#[test]
fn studio_star_naming_repair_canonical_clause_all_1500_have_display_names() {
    let spec = load_through_clause();
    let names = resolved_names(&spec);
    assert_eq!(names.len(), 1_500);
    assert_eq!(names.iter().map(|(_, name)| name).collect::<BTreeSet<_>>().len(), 1_500);
}

#[test]
fn studio_star_naming_repair_names_non_empty_and_not_hex_fallback() {
    for (system_id, name) in resolved_names(&load_through_clause()) {
        assert!(!name.trim().is_empty(), "system {system_id} has blank name");
        assert!(!name.starts_with('#'), "system {system_id} retained hex fallback {name}");
        assert_ne!(name, system_id.to_string(), "system id used as display name");
        assert_ne!(name.to_ascii_lowercase(), format!("{system_id:x}"));
        assert_ne!(name.to_ascii_uppercase(), format!("{system_id:X}"));
    }
}

#[test]
fn studio_star_naming_repair_deterministic_across_two_regenerations() {
    let mut first = load_canonical_base();
    let mut second = load_canonical_base();
    apply_expected_names(&mut first);
    apply_expected_names(&mut second);
    let first_json = save_scenario_spec_to_canonical_json(&first)
        .expect("first canonical save")
        .canonical_json;
    let second_json = save_scenario_spec_to_canonical_json(&second)
        .expect("second canonical save")
        .canonical_json;
    assert_eq!(first_json, second_json);
    assert_eq!(resolved_names(&first), resolved_names(&second));
}

#[test]
fn studio_star_naming_repair_canonical_json_is_up_to_date() {
    let path = canonical_base_path();
    let mut regenerated = load_canonical_base();
    apply_expected_names(&mut regenerated);
    let expected = save_scenario_spec_to_canonical_json(&regenerated)
        .expect("canonical save")
        .canonical_json;
    assert_eq!(std::fs::read_to_string(path).expect("committed JSON"), expected);
}

#[test]
fn studio_star_naming_repair_preserves_placements_links_and_owners() {
    let mut repaired = load_canonical_base();
    let placements = repaired.structural_grid.placements.clone();
    let links = repaired.links.clone();
    let owners = owner_identity(&repaired);
    apply_expected_names(&mut repaired);
    assert_eq!(repaired.structural_grid.placements, placements);
    assert_eq!(repaired.links, links);
    assert_eq!(owner_identity(&repaired), owners);
}

#[test]
fn studio_star_naming_repair_no_mapeditor_render_changes() {
    let presentation = include_str!("../../simthing-mapeditor/src/studio_faction_nameplates.rs");
    let galaxy_render = include_str!("../../simthing-mapeditor/src/app/galaxy_render.rs");
    assert!(presentation.contains("star_system_display_name"));
    assert!(presentation.contains("fallback_simthing_nameplate_id"));
    assert!(galaxy_render.contains("star_nameplate_presentations"));
}

#[test]
fn studio_star_naming_repair_clause_loader_regression() {
    let clause = std::fs::read_to_string(canonical_clause_path()).expect("canonical clause");
    assert!(clause.contains("source_json = \"terran_pirate_galaxy.base_disc.json\""));
    assert!(!clause.contains("{{FIXTURE_JSON}}"));
    let spec = load_through_clause();
    assert_eq!(spec.scenario_id, "terran_pirate_galaxy");
    assert_eq!(spec.gridcell_locations().count(), 1_500);
}

#[test]
fn studio_star_naming_repair_11_2_identity_regression() {
    let owners = owner_identity(&load_through_clause());
    assert!(owners.contains(&(
        "terran".into(),
        "Terran".into(),
        (64, 160, 255),
        "none".into(),
    )));
    assert!(owners.contains(&(
        "pirate".into(),
        "Pirate".into(),
        (220, 64, 48),
        "none".into(),
    )));
}
