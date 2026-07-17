//! TP-CLAUSE-ECONOMY-AUTHOR-0 — canonical TP field-economy authoring proofs.
//!
//! Scenario-specific proofs live in simthing-workshop (§12 homing). Production
//! crates must not gain net-new TP vocabulary for this rung.

use std::env;
use std::path::{Path, PathBuf};

use simthing_clausething::{
    hydrate_scenario_with_source_base, parse_raw_document, resolve_clause_source_path,
    HydratedScenarioPack,
};
use simthing_spec::{
    EmissionFormulaSpec, EmlGadgetInstanceSpec, ResourceEconomyOptInMode, save_scenario_spec_to_canonical_json,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

fn canonical_clause_path() -> PathBuf {
    repo_root().join("scenarios/terran_pirate_galaxy.clause")
}

fn canonical_base_json_path() -> PathBuf {
    repo_root().join("scenarios/terran_pirate_galaxy.base_disc.json")
}

fn hydrate_canonical() -> HydratedScenarioPack {
    let clause_path = canonical_clause_path();
    let text = std::fs::read_to_string(&clause_path).expect("read canonical clause");
    let document = parse_raw_document(text.as_bytes()).expect("parse canonical clause");
    let base = clause_path.parent().expect("clause parent").to_path_buf();
    hydrate_scenario_with_source_base(&document, Some(&base)).expect("hydrate with clause base")
}

fn constant_emission(pack: &HydratedScenarioPack, id: &str) -> f32 {
    let resource_economy = pack
        .game_mode
        .resource_economy
        .as_ref()
        .expect("resource economy");
    let emission = resource_economy
        .emissions
        .iter()
        .find(|entry| entry.id == id)
        .unwrap_or_else(|| panic!("missing emission {id}"));
    match &emission.formula {
        EmissionFormulaSpec::Constant(value) => *value,
        other => panic!("expected constant emission {id}, got {other:?}"),
    }
}

fn economy_fingerprint(pack: &HydratedScenarioPack) -> String {
    let economy = pack.field_economy.as_ref().expect("field economy");
    let resource = pack
        .game_mode
        .resource_economy
        .as_ref()
        .expect("resource economy");
    let overlays: Vec<_> = pack
        .game_mode
        .overlays
        .iter()
        .filter(|overlay| overlay.id.contains("tp_economy") || overlay.id.contains("owner_policy"))
        .map(|overlay| {
            (
                overlay.id.clone(),
                overlay.targets_property.clone(),
                format!("{:?}", overlay.sub_field_deltas),
            )
        })
        .collect();
    serde_json::to_string(&(economy, resource, overlays)).expect("serialize economy fingerprint")
}

/// catches: authored TP clause losing field-economy lowering onto existing RF/overlay/EML surfaces.
#[test]
fn canonical_tp_clause_hydrates_field_economy_to_existing_surfaces() {
    let pack = hydrate_canonical();
    let economy = pack.field_economy.as_ref().expect("field economy must hydrate");
    assert_eq!(economy.id, "tp_economy");
    assert_eq!(economy.namespace, "tp_economy");
    assert_eq!(economy.production_buildings.len(), 1);
    assert_eq!(economy.production_buildings[0].id, "shipyard_factory");
    assert_eq!(economy.production_buildings[0].location, "terran_shipyard");
    assert_eq!(economy.production_buildings[0].input_resource, "minerals");
    assert_eq!(economy.production_buildings[0].output_resource, "hulls");
    assert_eq!(economy.field_resource_quantities.len(), 1);
    assert_eq!(economy.disruption_presences.len(), 1);
    assert_eq!(economy.disruption_presences[0].location, "pirate_outpost");
    assert_eq!(economy.owner_policy_overlays.len(), 3);
    assert_eq!(economy.weight_profiles.len(), 3);
    assert!(
        economy
            .weight_profiles
            .iter()
            .any(|profile| profile.profile == "expansion-need")
    );
    assert!(
        economy
            .weight_profiles
            .iter()
            .any(|profile| profile.profile == "manufacturing-need")
    );
    assert!(
        economy
            .weight_profiles
            .iter()
            .any(|profile| profile.profile == "disruption-need")
    );
    match &economy.weight_profiles[0].stack.gadgets[0] {
        EmlGadgetInstanceSpec::WeightedAccumulator { .. } => {}
        other => panic!("expected WeightedAccumulator, got {other:?}"),
    }

    let resource_economy = pack
        .game_mode
        .resource_economy
        .as_ref()
        .expect("resource economy");
    assert_eq!(
        resource_economy.opt_in_mode,
        ResourceEconomyOptInMode::TransferAndEmission
    );
    assert!(
        resource_economy
            .recipes
            .iter()
            .any(|recipe| recipe.id == "tp_economy_recipe_shipyard_factory")
    );
    assert_eq!(
        resource_economy
            .recipes
            .iter()
            .find(|recipe| recipe.id == "tp_economy_recipe_shipyard_factory")
            .unwrap()
            .target
            .name,
        "terran_shipyard_hulls_quantity"
    );
    assert!(
        resource_economy
            .emit_on_threshold
            .iter()
            .any(|entry| entry.source.name == "pirate_outpost_disruption_presence")
    );
    assert_eq!(
        constant_emission(&pack, "tp_economy_quantity_emission_shipyard_minerals"),
        100.0
    );
    assert!(
        resource_economy
            .emissions
            .iter()
            .all(|entry| !entry.id.contains("production_yield"))
    );

    // Fleets and base disc remain production-hydrated siblings of the authored economy.
    assert_eq!(pack.fleet_ship_payloads.len(), 2);
    assert_eq!(pack.embedded_static_galaxy_scenarios.len(), 1);
    assert_eq!(pack.owners.len(), 2);
}

/// catches: non-deterministic field-economy / resource-economy regeneration across identical inputs.
#[test]
fn field_economy_regeneration_is_byte_identical_across_two_hydrations() {
    let first = hydrate_canonical();
    let second = hydrate_canonical();
    let first_fp = economy_fingerprint(&first);
    let second_fp = economy_fingerprint(&second);
    assert_eq!(
        first_fp, second_fp,
        "same clause input must regenerate byte-identical economy fingerprint"
    );

    // Sibling base-disc canonical JSON is stable (production artifact, not hand-edited in-rung).
    let base_bytes = std::fs::read(canonical_base_json_path()).expect("read base disc");
    let base_text = String::from_utf8(base_bytes.clone()).expect("utf8 base disc");
    let authority = simthing_spec::deserialize_scenario_authority(&base_text)
        .expect("base disc is production ScenarioSpec JSON");
    let roundtrip =
        save_scenario_spec_to_canonical_json(&authority).expect("canonical re-serialize");
    // Two serializations of the same authority must match each other (determinism of production path).
    let roundtrip_again =
        save_scenario_spec_to_canonical_json(&authority).expect("canonical re-serialize again");
    assert_eq!(
        roundtrip.canonical_json, roundtrip_again.canonical_json,
        "production canonical serialization must be deterministic"
    );
    assert_eq!(base_bytes.len(), 858222, "committed base-disc size must remain stable");
}

/// catches: bare source_json resolving against process CWD instead of clause directory.
#[test]
fn canonical_clause_blind_hydrates_from_alien_cwd() {
    let original = env::current_dir().expect("cwd");
    let alien = repo_root().join("crates").join("simthing-workshop");
    env::set_current_dir(&alien).expect("chdir alien");
    let result = std::panic::catch_unwind(|| {
        assert!(
            !env::current_dir()
                .unwrap()
                .join("terran_pirate_galaxy.base_disc.json")
                .is_file(),
            "base disc must not exist under alien CWD"
        );
        let pack = hydrate_canonical();
        assert_eq!(pack.scenario_id, "terran_pirate_galaxy");
        assert!(pack.field_economy.is_some(), "economy must hydrate blind");
        assert_eq!(pack.embedded_static_galaxy_scenarios.len(), 1);
        let resolved = resolve_clause_source_path(
            "terran_pirate_galaxy.base_disc.json",
            Some(canonical_clause_path().parent().unwrap()),
        );
        assert!(
            resolved.is_file(),
            "sibling base disc must resolve via clause-dir: {resolved:?}"
        );
    });
    env::set_current_dir(&original).expect("restore cwd");
    result.expect("blind hydrate from alien cwd");
}

/// catches: hand-edited economy JSON/RON sidecars appearing beside the clause.
#[test]
fn no_hand_edited_economy_json_or_ron_sidecar() {
    let scenarios = repo_root().join("scenarios");
    let clause = std::fs::read_to_string(canonical_clause_path()).expect("read clause");
    assert!(
        clause.contains("field_economy = tp_economy"),
        "economy must be authored as ClauseScript DATA"
    );
    assert!(
        clause.contains("source_json = \"terran_pirate_galaxy.base_disc.json\""),
        "base disc remains sibling relative path"
    );
    assert!(
        !clause.contains("{{FIXTURE_JSON}}"),
        "canonical operator clause must not use fixture tokens"
    );

    // Only the committed production base-disc JSON sibling is allowed — no economy RON/JSON sidecars.
    let entries: Vec<_> = std::fs::read_dir(&scenarios)
        .expect("read scenarios")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    assert!(
        entries
            .iter()
            .any(|name| name == "terran_pirate_galaxy.base_disc.json"),
        "production base-disc sibling must remain"
    );
    let forbidden: Vec<_> = entries
        .iter()
        .filter(|name| {
            let lower = name.to_ascii_lowercase();
            (lower.ends_with(".ron") || lower.ends_with(".json"))
                && lower.contains("economy")
                && !lower.contains("base_disc")
        })
        .cloned()
        .collect();
    assert!(
        forbidden.is_empty(),
        "hand-edited economy JSON/RON sidecars are forbidden: {forbidden:?}"
    );

    // Hydration must not invent economy sidecars either.
    let before: std::collections::BTreeSet<_> = entries.iter().cloned().collect();
    let _ = hydrate_canonical();
    let after: std::collections::BTreeSet<_> = std::fs::read_dir(&scenarios)
        .expect("read scenarios after hydrate")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(
        before, after,
        "production hydrate must not write new scenario sidecars"
    );
}
