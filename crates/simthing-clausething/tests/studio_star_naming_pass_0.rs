//! STUDIO-STAR-NAMING-PASS-0 canonical TP authority proofs.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use simthing_clausething::{
    hydrate_scenario_with_source_base, parse_raw_document, rebind_pack_to_structural_rebind_ready,
    HydratedScenarioPack,
};
use simthing_mapgenerator::assign_star_names;
use simthing_spec::{
    apply_star_system_display_name_metadata, gridcell_generated_system_id, is_owner_entity_kind,
    load_scenario_spec_from_json_str, owner_entity_id, owner_faction_color_rgb,
    owner_faction_display_name, resolve_map_container_mut, save_scenario_spec_to_canonical_json,
    star_system_display_name, SimThingScenarioSpec,
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

fn load_canonical_base() -> SimThingScenarioSpec {
    let path = canonical_base_path();
    let json = std::fs::read_to_string(&path).expect("read base disc");
    load_scenario_spec_from_json_str(path.to_str().expect("path"), &json)
        .expect("load base disc")
        .0
}

fn hydrate_canonical() -> HydratedScenarioPack {
    let path = repo_root().join("scenarios/terran_pirate_galaxy.clause");
    let text = std::fs::read_to_string(&path).expect("read clause");
    let document = parse_raw_document(text.as_bytes()).expect("parse clause");
    hydrate_scenario_with_source_base(&document, path.parent()).expect("empty-resolver hydrate")
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
                expected.get(&system_id).expect("name assignment"),
            );
        }
    }
}

#[test]
fn star_naming_canonical_tp_all_systems_have_display_names() {
    let path = canonical_base_path();
    let mut expected = load_canonical_base();
    let placements_before = expected.structural_grid.placements.clone();
    let links_before = expected.links.clone();
    apply_expected_names(&mut expected);
    assert_eq!(expected.structural_grid.placements, placements_before);
    assert_eq!(expected.links, links_before);
    let expected_json = save_scenario_spec_to_canonical_json(&expected)
        .expect("canonical save")
        .canonical_json;
    if std::env::var_os("UPDATE_STUDIO_STAR_NAMING_GOLDEN").is_some() {
        std::fs::write(&path, &expected_json).expect("update canonical golden");
    }
    assert_eq!(
        std::fs::read_to_string(&path).expect("read committed golden"),
        expected_json,
        "canonical star-name golden is stale"
    );
    let names: Vec<_> = expected
        .gridcell_locations()
        .map(|system| star_system_display_name(system).expect("display name"))
        .collect();
    assert_eq!(names.len(), 1_500);
    assert_eq!(names.iter().collect::<BTreeSet<_>>().len(), names.len());
}
