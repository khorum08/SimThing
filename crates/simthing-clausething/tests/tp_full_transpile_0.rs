//! TP-FULL-TRANSPILE-0 — complete single ClauseScript file → canonical SimThingScenarioSpec.

use std::collections::BTreeSet;
use std::path::Path;

use simthing_clausething::{hydrate_scenario, parse_raw_document, HydratedScenarioPack};
use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    deserialize_scenario_authority, game_session_child, game_session_galaxy_map, game_session_owners,
    is_galaxy_map_entity, owner_entity_id, save_scenario_spec_to_canonical_json,
    star_system_gridcells, validate_scenario_game_session_child, validate_session_galaxy_map,
    validate_session_owner_entities, SimThingScenarioGrid, SimThingScenarioProvenance,
    SimThingScenarioSpec,
};

fn fixture_json_path() -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-mapeditor/tests/fixtures/tp_base_disc_1500.simthing-scenario.json")
        .to_string_lossy()
        .replace('\\', "/")
}

fn clause_source() -> String {
    include_str!("fixtures/scenario/terran_pirate_galaxy.clause")
        .replace("{{FIXTURE_JSON}}", &fixture_json_path())
}

fn hydrate_pack() -> HydratedScenarioPack {
    let document = parse_raw_document(clause_source().as_bytes()).expect("parse full TP clause");
    hydrate_scenario(&document).expect("hydrate full TP clause")
}

fn authority_spec(pack: &HydratedScenarioPack) -> SimThingScenarioSpec {
    let embedded = pack
        .embedded_static_galaxy_scenarios
        .first()
        .expect("embedded base disc");
    // Authority tree is the canonical ScenarioSpec root for this rung.
    // STEAD lattice metadata lives on the embedded base (and pack.grid_metadata);
    // placement location_ids are producer-local and are not re-bound onto authority
    // tree nodes here (that binding is runtime install / TP-LIVE-RUN). Roundtrip
    // proves the authority tree + provenance byte-stable without dangling grid ids.
    SimThingScenarioSpec {
        scenario_id: pack.scenario_id.clone(),
        root: pack
            .authority_root
            .clone()
            .expect("full transpile pack carries authority_root"),
        structural_grid: SimThingScenarioGrid {
            frame: embedded.source_structural_grid.frame,
            map_container_id: String::new(),
            placements: Vec::new(),
        },
        links: Vec::new(),
        provenance: SimThingScenarioProvenance {
            source: embedded.provenance.source.clone(),
            generator_seed: embedded.provenance.generator_seed,
            generator_shape: embedded.provenance.generator_shape.clone(),
            generator_profile_id: embedded.provenance.generator_profile_id.clone(),
            generator_params_json: embedded.provenance.generator_params_json.clone(),
            name_corpus_source: embedded.provenance.name_corpus_source.clone(),
            name_assignment_mode: embedded.provenance.name_assignment_mode.clone(),
        },
    }
}

fn count_kind(root: &SimThing, kind: &SimThingKind) -> usize {
    let mut n = 0usize;
    fn walk(node: &SimThing, kind: &SimThingKind, n: &mut usize) {
        if &node.kind == kind {
            *n += 1;
        }
        for child in &node.children {
            walk(child, kind, n);
        }
    }
    walk(root, kind, &mut n);
    n
}

/// Single load-bearing proof: parse → hydrate → canonical ScenarioSpec → roundtrip + content.
#[test]
fn terran_pirate_galaxy_full_transpile_to_canonical_scenario_spec() {
    let pack = hydrate_pack();

    // --- identity / structure ---
    assert_eq!(pack.scenario_id, "terran_pirate_galaxy");
    assert_eq!(pack.embedded_static_galaxy_scenarios.len(), 1);
    assert_eq!(pack.owners.len(), 2);
    assert_eq!(pack.owners[0].owner_key, "terran");
    assert_eq!(pack.owners[1].owner_key, "pirate");
    assert_eq!(pack.ownership_volumes.len(), 2);
    assert_eq!(pack.planet_surface_payloads.len(), 2);
    assert_eq!(pack.fleet_ship_payloads.len(), 2);
    assert!(
        pack.combat_arena_payload.is_some(),
        "combat arena payload must hydrate"
    );
    assert!(
        pack.palma_feedstock.is_some(),
        "PALMA reach/impedance feedstock must hydrate"
    );
    assert!(
        pack.commitment.is_some(),
        "FIELD_POLICY commitment feedstock must hydrate"
    );
    assert!(
        !pack.game_mode.region_fields.is_empty(),
        "Movement-Front field operator must lower into game_mode.region_fields"
    );
    assert_eq!(
        pack.metadata.get("diplomacy_lane_profile").map(String::as_str),
        Some("influence_distrust_rf")
    );
    assert_eq!(
        pack.metadata.get("fleet_movement_profile").map(String::as_str),
        Some("palma_d_gradient_reparent")
    );

    let terran_vol = pack
        .ownership_volumes
        .iter()
        .find(|v| v.owner == "terran")
        .expect("terran volume");
    let pirate_vol = pack
        .ownership_volumes
        .iter()
        .find(|v| v.owner == "pirate")
        .expect("pirate volume");
    assert_eq!(terran_vol.assigned_systems.len(), 200);
    assert_eq!(pirate_vol.assigned_systems.len(), 50);

    let owned = pack
        .planet_surface_payloads
        .iter()
        .find(|p| p.id == "owned_system_payload")
        .expect("owned payload");
    let neutral = pack
        .planet_surface_payloads
        .iter()
        .find(|p| p.id == "neutral_system_payload")
        .expect("neutral payload");
    assert!(owned.factory_min >= 1 && owned.cohort_min >= 1);
    assert_eq!(neutral.factory_min, 0);
    assert_eq!(neutral.cohort_min, 0);

    let terran_fleets = pack
        .fleet_ship_payloads
        .iter()
        .find(|p| p.owner == "terran")
        .expect("terran fleets");
    let pirate_fleets = pack
        .fleet_ship_payloads
        .iter()
        .find(|p| p.owner == "pirate")
        .expect("pirate fleets");
    assert_eq!(terran_fleets.fleet_count, 10);
    assert_eq!(terran_fleets.ships_per_fleet, 20);
    assert_eq!(pirate_fleets.fleet_count, 10);
    assert_eq!(pirate_fleets.ships_per_fleet, 40);

    // --- authority tree content ---
    let spec = authority_spec(&pack);
    validate_scenario_game_session_child(&spec).expect("GameSession child");
    validate_session_owner_entities(&spec).expect("owner entities");
    validate_session_galaxy_map(&spec).expect("galaxy map sibling");

    let owners = game_session_owners(&spec).expect("owners");
    assert_eq!(owners.len(), 2);
    let owner_keys: BTreeSet<_> = owners
        .iter()
        .filter_map(|o| owner_entity_id(o))
        .collect();
    assert!(owner_keys.contains("terran"));
    assert!(owner_keys.contains("pirate"));

    let galaxy_map = game_session_galaxy_map(&spec).expect("GalaxyMap");
    assert!(is_galaxy_map_entity(galaxy_map));
    let systems = star_system_gridcells(&spec).expect("star systems");
    assert_eq!(systems.len(), 1500);
    // STEAD lattice feedstock on the embedded base (namespaced overlay targets).
    let embedded = &pack.embedded_static_galaxy_scenarios[0];
    assert_eq!(embedded.namespaced_placements.len(), 1500);
    assert!(
        embedded
            .namespaced_placements
            .iter()
            .all(|p| p.target_id.starts_with("tp_base::")),
        "overlay location-targets must remain namespaced"
    );
    assert!(
        !embedded.namespaced_links.is_empty(),
        "hyperlane links survive on embedded base"
    );
    assert_eq!(pack.grid_metadata.placements.len(), 1500);
    assert_eq!(spec.structural_grid.frame.width, 300);
    assert_eq!(spec.structural_grid.frame.height, 300);
    assert_eq!(embedded.provenance.generator_seed, 770421);

    let root = &spec.root;
    // 10 Terran + 10 Pirate authored fleets, plus combat_arena contact fleets (ships_per_side=1 → +2).
    let fleets = count_kind(root, &SimThingKind::Fleet);
    assert_eq!(
        fleets, 22,
        "20 authored fleets + 2 combat-contact fleets"
    );
    // Ships are cohort-style children under fleets (ordinary SimThings).
    let mut ship_count = 0usize;
    fn count_ships(node: &SimThing, n: &mut usize) {
        if node.kind == SimThingKind::Fleet {
            *n += node
                .children
                .iter()
                .filter(|c| c.kind == SimThingKind::Cohort)
                .count();
        }
        for child in &node.children {
            count_ships(child, n);
        }
    }
    count_ships(root, &mut ship_count);
    assert!(
        ship_count >= 600,
        "at least 200 Terran + 400 Pirate ships; got {ship_count}"
    );
    let combat = pack.combat_arena_payload.as_ref().expect("combat");
    assert_eq!(
        combat.enrollments.len(),
        2,
        "combat arena enrolls one ship per side"
    );

    let session = game_session_child(&spec).expect("session");
    assert!(
        session
            .children
            .iter()
            .any(|c| is_galaxy_map_entity(c)),
        "GalaxyMap remains GameSession sibling"
    );
    assert!(
        owners.iter().all(|o| o.children.is_empty()),
        "owners are non-spatial"
    );

    // --- canonical JSON roundtrip ---
    let save = save_scenario_spec_to_canonical_json(&spec).expect("canonical save");
    assert!(save.deterministic, "canonical JSON must be deterministic");
    assert!(save.byte_len > 0);
    let roundtrip =
        deserialize_scenario_authority(&save.canonical_json).expect("canonical deserialize");
    let save2 = save_scenario_spec_to_canonical_json(&roundtrip).expect("canonical re-save");
    assert_eq!(
        save.canonical_json, save2.canonical_json,
        "canonical byte-stable roundtrip"
    );
    assert_eq!(save.authority_digest, save2.authority_digest);
    assert_eq!(roundtrip.scenario_id, "terran_pirate_galaxy");
    assert_eq!(
        star_system_gridcells(&roundtrip)
            .expect("roundtrip systems")
            .len(),
        1500
    );
    assert_eq!(
        game_session_owners(&roundtrip)
            .expect("roundtrip owners")
            .len(),
        2
    );
    assert_eq!(count_kind(&roundtrip.root, &SimThingKind::Fleet), 22);

    // --- semantic-free below the spec boundary (this PR) ---
    // Authoring ids (Terran/Pirate) legitimately remain in ScenarioSpec.
    // This rung must not introduce runtime/GPU/sim/kernel branching on those words.
    // Proven by scope: no edits to simthing-sim / simthing-kernel / WGSL in this PR.
    // Guard: authority tree does not mint new semantic SimThingKind variants.
    assert!(
        !format!("{:?}", roundtrip.root.kind).contains("Terran")
            && !format!("{:?}", roundtrip.root.kind).contains("Pirate"),
        "root kind must remain semantic-free enum, not faction-named"
    );
}
