//! SCENARIO-CANONICAL-LOAD-SAVE-ROUNDTRIP-0 — canonical ScenarioSpec import/export I/O.
//!
//! Headless load/save roundtrip for Studio scenario import/export. CPU responsibilities:
//! canonical JSON parse/serialize, ingestion admission reporting, and authority digest checks.

use serde_json::Value;

use crate::error::SpecError;

use super::scenario::{deserialize_scenario_authority, ScenarioSerdeError, SimThingScenarioSpec};
use super::scenario_ingestion::{
    ingest_scenario, studio_canonical_ingestion_profile, ScenarioIngestionClassification,
    ScenarioIngestionResult,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioCanonicalLoadReport {
    pub source_label: String,
    pub loaded: bool,
    pub authority_digest: u64,
    pub scenario_id: Option<String>,
    pub simthing_count: u32,
    pub ingestion_ready: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioCanonicalSaveReport {
    pub canonical_json: String,
    pub authority_digest: u64,
    pub byte_len: u32,
    pub deterministic: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioCanonicalRoundtripReport {
    pub source_label: String,
    pub initial_load: ScenarioCanonicalLoadReport,
    pub canonical_save: ScenarioCanonicalSaveReport,
    pub roundtrip_load: ScenarioCanonicalLoadReport,
    pub initial_digest: u64,
    pub roundtrip_digest: u64,
    pub digest_stable: bool,
    pub canonical_bytes_stable: bool,
    pub scenario_authority_preserved: bool,
}

/// Load ScenarioSpec from canonical JSON with validation and ingestion admission report.
pub fn load_scenario_spec_from_json_str(
    source_label: &str,
    json: &str,
) -> Result<(SimThingScenarioSpec, ScenarioCanonicalLoadReport), SpecError> {
    let scenario = deserialize_scenario_authority(json).map_err(map_serde_error)?;
    let ingestion = ingest_scenario(
        source_label,
        &scenario,
        studio_canonical_ingestion_profile(),
    );
    let report = build_load_report(source_label, &scenario, &ingestion, true)?;
    Ok((scenario, report))
}

/// Serialize ScenarioSpec to deterministic canonical JSON.
pub fn save_scenario_spec_to_canonical_json(
    scenario: &SimThingScenarioSpec,
) -> Result<ScenarioCanonicalSaveReport, SpecError> {
    let canonical_json = to_deterministic_canonical_json(scenario)?;
    let authority_digest = scenario_authority_digest_u64_from_json(&canonical_json);
    let repeat = to_deterministic_canonical_json(scenario)?;
    let deterministic = repeat == canonical_json;
    let byte_len = u32::try_from(canonical_json.len()).map_err(|_| SpecError::ValidationFailed)?;

    Ok(ScenarioCanonicalSaveReport {
        canonical_json,
        authority_digest,
        byte_len,
        deterministic,
    })
}

/// Prove canonical load/save roundtrip preserves Scenario authority digest.
pub fn prove_scenario_canonical_load_save_roundtrip(
    source_label: &str,
    json: &str,
) -> Result<ScenarioCanonicalRoundtripReport, SpecError> {
    let (initial_scenario, initial_load) = load_scenario_spec_from_json_str(source_label, json)?;
    let canonical_save = save_scenario_spec_to_canonical_json(&initial_scenario)?;
    let (roundtrip_scenario, roundtrip_load) =
        load_scenario_spec_from_json_str(source_label, &canonical_save.canonical_json)?;
    let repeat_save = save_scenario_spec_to_canonical_json(&roundtrip_scenario)?;

    let initial_digest = canonical_save.authority_digest;
    let roundtrip_digest = roundtrip_load.authority_digest;
    let digest_stable = initial_digest == roundtrip_digest;
    let canonical_bytes_stable = canonical_save.canonical_json == repeat_save.canonical_json;
    let scenario_authority_preserved = digest_stable
        && canonical_bytes_stable
        && canonical_save.deterministic
        && repeat_save.deterministic;

    Ok(ScenarioCanonicalRoundtripReport {
        source_label: source_label.to_string(),
        initial_load,
        canonical_save,
        roundtrip_load,
        initial_digest,
        roundtrip_digest,
        digest_stable,
        canonical_bytes_stable,
        scenario_authority_preserved,
    })
}

fn build_load_report(
    source_label: &str,
    scenario: &SimThingScenarioSpec,
    ingestion: &ScenarioIngestionResult,
    loaded: bool,
) -> Result<ScenarioCanonicalLoadReport, SpecError> {
    let canonical_json = to_deterministic_canonical_json(scenario)?;
    let authority_digest = scenario_authority_digest_u64_from_json(&canonical_json);
    let simthing_count =
        u32::try_from(scenario.root.subtree_size()).map_err(|_| SpecError::ValidationFailed)?;
    let scenario_id = if scenario.scenario_id.is_empty() {
        None
    } else {
        Some(scenario.scenario_id.clone())
    };
    let ingestion_ready = loaded
        && ingestion.validation.json_parse_ok
        && matches!(
            ingestion.classification,
            ScenarioIngestionClassification::Admitted
                | ScenarioIngestionClassification::PartiallyAdmitted
        );

    Ok(ScenarioCanonicalLoadReport {
        source_label: source_label.to_string(),
        loaded,
        authority_digest,
        scenario_id,
        simthing_count,
        ingestion_ready,
    })
}

fn to_deterministic_canonical_json(scenario: &SimThingScenarioSpec) -> Result<String, SpecError> {
    let mut to_write = scenario.clone();
    to_write.sync_sidecar_from_root_metadata();
    let value = serde_json::to_value(&to_write).map_err(|err| map_json_error(err.to_string()))?;
    let sorted = sort_json_value(value);
    serde_json::to_string(&sorted).map_err(|err| map_json_error(err.to_string()))
}

fn sort_json_value(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut keys: Vec<String> = map.keys().cloned().collect();
            keys.sort();
            let mut sorted = serde_json::Map::new();
            for key in keys {
                if let Some(child) = map.get(&key) {
                    let normalized = if key == "properties" {
                        sort_properties_array(child.clone())
                    } else {
                        sort_json_value(child.clone())
                    };
                    sorted.insert(key, normalized);
                }
            }
            Value::Object(sorted)
        }
        Value::Array(items) => Value::Array(items.into_iter().map(sort_json_value).collect()),
        other => other,
    }
}

fn sort_properties_array(value: Value) -> Value {
    match value {
        Value::Array(items) if is_property_tuple_array(&items) => {
            let mut pairs: Vec<Value> = items.into_iter().map(sort_json_value).collect();
            pairs.sort_by_key(property_tuple_id);
            Value::Array(pairs)
        }
        other => sort_json_value(other),
    }
}

fn is_property_tuple_array(items: &[Value]) -> bool {
    items.iter().all(|item| property_tuple_id(item).is_some())
}

fn property_tuple_id(value: &Value) -> Option<u64> {
    let Value::Array(pair) = value else {
        return None;
    };
    if pair.len() != 2 {
        return None;
    }
    match &pair[0] {
        Value::Number(number) => number.as_u64(),
        _ => None,
    }
}

fn map_serde_error(_err: ScenarioSerdeError) -> SpecError {
    SpecError::ValidationFailed
}

fn map_json_error(_message: String) -> SpecError {
    SpecError::ValidationFailed
}

fn scenario_authority_digest_u64_from_json(json: &str) -> u64 {
    fnv1a64_u64(json)
}

fn fnv1a64_u64(input: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in input.as_bytes() {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
