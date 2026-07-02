use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use simthing_clausething::{
    hydrate_scenario, parse_raw_document, HydratedScenarioPack,
};
use simthing_core::{SimThing, SimThingKind};
use simthing_spec::{
    all_planet_gridcells, evaluate_planet_child_locations, evaluate_planet_child_rf_admission,
    evaluate_planet_child_rf_reduce_up, game_session_child, game_session_galaxy_map,
    game_session_owners, is_admitted_planet_non_grid_child, is_planet_gridcell,
    is_surface_gridcell, owner_entity_id, owner_flow_owner_ref, planet_gameplay_children,
    planet_surface_gridcell, star_system_gridcells, PlanetChildRfAdmissionClassification,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-mapeditor/tests/fixtures/tp_base_disc_1500.simthing-scenario.json")
}

fn fixture_path_text() -> String {
    fixture_path().to_string_lossy().replace('\\', "/")
}

fn combined_clause() -> String {
    format!(
        r#"
scenario = tp_planet_surface_payload_0 {{
    metadata = {{
        display_name = "TP Planet Surface Payload 0"
        runtime_owner = "scenario-container"
    }}
    static_galaxy_scenario = base_disc {{
        namespace = "tp_base"
        source_json = "{}"
        map_quality_status = PASS
    }}
    owner = terran {{
        owner_key = "terran"
        display_name = "Terran Compact"
        archetype = "settler_policy"
    }}
    owner = pirate {{
        owner_key = "pirate"
        display_name = "Pirate Cartel"
        archetype = "raider_policy"
    }}
    ownership_volume = terran_core {{
        owner = "terran"
        count = 200
        selection = chebyshev_contiguous
        seed = 770421
        anchor_row = 199
        anchor_col = 80
    }}
    ownership_volume = pirate_border {{
        owner = "pirate"
        count = 50
        selection = chebyshev_contiguous
        adjacent_to = "terran_core"
        seed = 770421
    }}
    planet_surface_payload = owned_system_payload {{
        applies_to = owned_systems
        planets_per_system_min = 1
        surface_grid = "1x1"
        factory_min = 1
        cohort_min = 1
        category_map = {{
            pop_factory = {{ kind = Cohort depth = 3 }}
        }}
        resource = {{
            id = "tp_minerals"
            namespace = "tp"
            name = "minerals"
            display_name = "Minerals"
        }}
        modifier = {{
            pop_factory_minerals_produces_mult = 0.10
            pop_factory_minerals_upkeep_add = 1
        }}
    }}
    planet_surface_payload = neutral_system_payload {{
        applies_to = neutral_systems
        planets_per_system_min = 1
        surface_grid = "1x1"
        factory_min = 0
        cohort_min = 0
    }}
}}
"#,
        fixture_path_text()
    )
}

fn hydrate_pack() -> HydratedScenarioPack {
    let document = parse_raw_document(combined_clause().as_bytes()).expect("parse combined clause");
    hydrate_scenario(&document).expect("hydrate planet surface payload clause")
}

fn authority_root(pack: &HydratedScenarioPack) -> SimThing {
    pack.authority_root
        .clone()
        .expect("payload pack carries authority root")
}

fn scenario_from_root(root: SimThing) -> simthing_spec::SimThingScenarioSpec {
    simthing_spec::SimThingScenarioSpec {
        scenario_id: "tp_planet_surface_payload_0".to_string(),
        root,
        structural_grid: simthing_spec::SimThingScenarioGrid {
            frame: simthing_spec::SimThingStructuralGridFrame {
                width: 0,
                height: 0,
                occupied_cells: 0,
            },
            map_container_id: String::new(),
            placements: Vec::new(),
        },
        links: Vec::new(),
        provenance: simthing_spec::SimThingScenarioProvenance::default(),
    }
}

fn owned_target_ids(pack: &HydratedScenarioPack) -> BTreeSet<String> {
    pack.ownership_volumes
        .iter()
        .flat_map(|volume| volume.assigned_systems.iter())
        .map(|system| system.target_id.clone())
        .collect()
}

#[test]
fn embedded_base_owner_siblings_ownership_and_planet_surface_payload_parse() {
    let pack = hydrate_pack();
    assert_eq!(pack.embedded_static_galaxy_scenarios.len(), 1);
    assert_eq!(pack.owners.len(), 2);
    assert_eq!(pack.ownership_volumes.len(), 2);
    assert_eq!(pack.planet_surface_payloads.len(), 2);
    assert!(pack
        .planet_surface_payloads
        .iter()
        .any(|payload| payload.applies_to == "owned_systems"));
    assert!(pack
        .planet_surface_payloads
        .iter()
        .any(|payload| payload.applies_to == "neutral_systems"));
}

#[test]
fn owned_systems_have_planet_surface_factory_and_cohort() {
    let pack = hydrate_pack();
    let root = authority_root(&pack);
    let spec = scenario_from_root(root);
    let owned = owned_target_ids(&pack);
    let systems = star_system_gridcells(&spec).expect("star-system gridcells");

    for system in &systems {
        let owner = owner_flow_owner_ref(system);
        let planets: Vec<_> = system
            .children
            .iter()
            .filter(|child| is_planet_gridcell(child))
            .collect();
        if owner.is_some() {
            assert!(
                !planets.is_empty(),
                "owned system must have at least one planet gridcell"
            );
            let surface_present = planets.iter().any(|planet| planet_surface_gridcell(planet).is_some());
            assert!(surface_present, "owned system must have mandated 1x1 surface");
            let factories = planets
                .iter()
                .flat_map(|planet| planet_gameplay_children(planet))
                .filter(|child| matches!(child.kind, SimThingKind::Custom(ref name) if name == "Infrastructure"))
                .count();
            let cohorts = planets
                .iter()
                .flat_map(|planet| planet_gameplay_children(planet))
                .filter(|child| child.kind == SimThingKind::Cohort)
                .count();
            assert!(factories >= 1, "owned system must have at least one factory");
            assert!(cohorts >= 1, "owned system must have at least one cohort");
        }
    }

    let owned_with_payload = systems
        .iter()
        .filter(|system| owner_flow_owner_ref(system).is_some())
        .count();
    assert_eq!(owned_with_payload, 250);
    assert_eq!(owned.len(), 250);
}

#[test]
fn neutral_systems_have_planet_surface_without_factory_or_cohort() {
    let pack = hydrate_pack();
    let root = authority_root(&pack);
    let spec = scenario_from_root(root);
    let systems = star_system_gridcells(&spec).expect("star-system gridcells");
    let neutral: Vec<_> = systems
        .iter()
        .filter(|system| owner_flow_owner_ref(system).is_none())
        .collect();

    assert_eq!(neutral.len(), 1250);
    for system in neutral {
        let planets: Vec<_> = system
            .children
            .iter()
            .filter(|child| is_planet_gridcell(child))
            .collect();
        assert!(!planets.is_empty(), "neutral system must have planet");
        assert!(
            planets
                .iter()
                .any(|planet| planet_surface_gridcell(planet).is_some()),
            "neutral system must have mandated 1x1 surface"
        );
        let gameplay: Vec<_> = planets
            .iter()
            .flat_map(|planet| planet_gameplay_children(planet))
            .collect();
        assert!(
            gameplay.is_empty(),
            "neutral system must not admit factory/cohort gameplay children"
        );
    }
}

#[test]
fn surface_tier_is_non_vacuous_and_does_not_silently_collapse() {
    let pack = hydrate_pack();
    let root = authority_root(&pack);
    let spec = scenario_from_root(root);
    let report = evaluate_planet_child_locations(&spec);
    assert!(report.surface_gridcell_tier_required);
    assert!(report.surface_gridcell_tier_present);
    assert!(report.surface_gridcell_count > 0);
    assert_eq!(report.planet_surface_gridcell_count, report.surface_gridcell_count);
}

#[test]
fn ownership_columns_remain_200_50_1250() {
    let pack = hydrate_pack();
    let mut counts = BTreeMap::new();
    for volume in &pack.ownership_volumes {
        *counts.entry(volume.owner.clone()).or_default() += volume.assigned_systems.len();
    }
    assert_eq!(counts.get("terran").copied(), Some(200));
    assert_eq!(counts.get("pirate").copied(), Some(50));
    let assigned = pack
        .ownership_volumes
        .iter()
        .flat_map(|volume| volume.assigned_systems.iter())
        .count();
    assert_eq!(1500 - assigned, 1250);
}

#[test]
fn owner_refs_remain_gamesession_sibling_targets() {
    let pack = hydrate_pack();
    let root = authority_root(&pack);
    let spec = scenario_from_root(root);
    let owners: BTreeSet<_> = game_session_owners(&spec)
        .expect("owners")
        .into_iter()
        .filter_map(owner_entity_id)
        .collect();
    let game_session = game_session_child(&spec).expect("GameSession");
    let galaxy_map = game_session_galaxy_map(&spec).expect("GalaxyMap");

    for volume in &pack.ownership_volumes {
        for system in &volume.assigned_systems {
            assert!(owners.contains(&system.owner_ref));
        }
    }
    assert!(game_session
        .children
        .iter()
        .filter(|child| child.kind == SimThingKind::Owner)
        .all(|owner| owner.children.is_empty()));
    assert_eq!(galaxy_map.children.len(), 1500);
}

#[test]
fn rf_settlement_path_exists_for_owned_surface_participants() {
    let pack = hydrate_pack();
    let root = authority_root(&pack);
    let spec = scenario_from_root(root);
    let admission = evaluate_planet_child_rf_admission(&spec);
    assert_ne!(
        admission.classification,
        PlanetChildRfAdmissionClassification::Rejected
    );
    assert!(admission.total_participant_count >= 500);
    let reduce_up = evaluate_planet_child_rf_reduce_up(&spec);
    assert_ne!(
        reduce_up.classification,
        PlanetChildRfAdmissionClassification::Rejected
    );
    assert!(!reduce_up.buckets.is_empty());
    assert!(reduce_up.buckets.iter().any(|bucket| {
        bucket.scope.planet_id.is_some() && bucket.scope.star_system_gridcell_id_raw.is_some()
    }));
}

#[test]
fn modifier_chains_admitted_through_existing_decoder_surfaces() {
    let pack = hydrate_pack();
    let owned = pack
        .planet_surface_payloads
        .iter()
        .find(|payload| payload.applies_to == "owned_systems")
        .expect("owned payload");
    assert_eq!(owned.decoded_modifier_keys.len(), 2);
    assert!(owned
        .decoded_modifier_keys
        .iter()
        .any(|key| key.category == "pop_factory" && key.resource == "minerals"));
    assert!(!pack.game_mode.overlays.is_empty());
}

#[test]
fn unsupported_payload_fields_hard_error_with_span() {
    let source = combined_clause().replace(
        "factory_min = 1",
        "factory_min = 1\n        unsupported_field = true",
    );
    let document = parse_raw_document(source.as_bytes()).expect("parse unsupported field");
    let err = hydrate_scenario(&document).expect_err("unsupported field must hard-error");
    assert!(err.to_string().contains("unsupported planet_surface_payload"));
    assert!(err.span.is_some());
}

#[test]
fn invalid_owned_payload_counts_hard_error_with_span() {
    let source = combined_clause().replace("factory_min = 1", "factory_min = 0");
    let document = parse_raw_document(source.as_bytes()).expect("parse invalid owned counts");
    let err = hydrate_scenario(&document).expect_err("invalid owned counts must hard-error");
    assert!(err.to_string().contains("factory_min"));
    assert!(err.span.is_some());
}

#[test]
fn neutral_payload_with_factory_hard_errors_with_span() {
    let source = combined_clause().replace(
        "planet_surface_payload = neutral_system_payload",
        "planet_surface_payload = neutral_bad_payload",
    )
    .replace(
        "applies_to = neutral_systems\n        planets_per_system_min = 1\n        surface_grid = \"1x1\"\n        factory_min = 0",
        "applies_to = neutral_systems\n        planets_per_system_min = 1\n        surface_grid = \"1x1\"\n        factory_min = 1",
    );
    let document = parse_raw_document(source.as_bytes()).expect("parse neutral factory");
    let err = hydrate_scenario(&document).expect_err("neutral factory must hard-error");
    assert!(err.to_string().contains("neutral_systems"));
    assert!(err.span.is_some());
}

#[test]
fn all_planet_gridcells_enumerate_surface_tier() {
    let pack = hydrate_pack();
    let root = authority_root(&pack);
    let spec = scenario_from_root(root);
    let planets = all_planet_gridcells(&spec);
    assert_eq!(planets.len(), 1500);
    for planet in planets {
        let surface = planet_surface_gridcell(planet).expect("each planet has surface tier");
        assert!(is_surface_gridcell(surface));
        for child in &surface.children {
            assert!(is_admitted_planet_non_grid_child(&child.kind));
        }
    }
}