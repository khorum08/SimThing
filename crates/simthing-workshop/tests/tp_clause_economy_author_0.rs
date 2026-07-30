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
use simthing_core::{SubFieldRole, TransformOp};
use simthing_spec::{
    save_scenario_spec_to_canonical_json, EmissionFormulaSpec, EmlGadgetInstanceSpec,
    InstallTargetSpec, ResourceEconomyOptInMode, TriggerDirection,
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
