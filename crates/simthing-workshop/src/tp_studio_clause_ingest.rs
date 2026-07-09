//! Workshop-homed TP candidate wrapper around production clausething projection.
//!
//! Keeps TP fixture defaults for expirable track tests only. Production mapeditor API
//! has no TP defaults (see simthing-mapeditor::clause_scenario_ingest).

use std::path::{Path, PathBuf};

use simthing_clausething::{
    hydrate_scenario, parse_raw_document, project_pack_to_authority_tree_candidate, HydrateError,
    HydratedScenarioPack, ParseError,
};
use simthing_spec::{
    deserialize_scenario_authority, serialize_scenario_authority, SimThingScenarioSpec,
};
use thiserror::Error;

/// Placeholder embedded in approved TP clause fixtures for `source_json`.
pub const CLAUSE_FIXTURE_JSON_PLACEHOLDER: &str = "{{FIXTURE_JSON}}";

/// Workshop-default embedded base-disc JSON for the approved TP clause fixture.
pub fn default_tp_base_disc_json_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-mapeditor/tests/fixtures/tp_base_disc_1500.simthing-scenario.json")
}

#[derive(Debug, Clone)]
pub struct TpStudioClauseIngestOptions {
    pub embedded_source_json_path: Option<PathBuf>,
}

impl Default for TpStudioClauseIngestOptions {
    fn default() -> Self {
        Self {
            embedded_source_json_path: Some(default_tp_base_disc_json_path()),
        }
    }
}

#[derive(Debug, Error)]
pub enum TpStudioClauseIngestError {
    #[error("TP clause ingest IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TP clause parse error: {0}")]
    Parse(#[from] ParseError),
    #[error("TP clause hydrate error: {0}")]
    Hydrate(#[from] HydrateError),
    #[error("TP clause projection error: {0}")]
    Projection(String),
    #[error("TP clause scenario authority serde error: {0}")]
    Serde(String),
}

impl TpStudioClauseIngestError {
    pub fn status_message(&self) -> String {
        self.to_string()
    }

    pub fn hydrate_token_index(&self) -> Option<usize> {
        match self {
            Self::Hydrate(err) => err.span.as_ref().map(|s| s.token_index),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TpStudioClauseIngestResult {
    pub source_path: PathBuf,
    pub pack: HydratedScenarioPack,
    pub scenario: SimThingScenarioSpec,
}

pub fn ingest_tp_clause_scenario_path(
    path: &Path,
    options: &TpStudioClauseIngestOptions,
) -> Result<TpStudioClauseIngestResult, TpStudioClauseIngestError> {
    let raw = std::fs::read_to_string(path)?;
    let source = resolve_clause_source_text(&raw, options)?;
    let document = parse_raw_document(source.as_bytes())?;
    let pack = hydrate_scenario(&document)?;
    let scenario = project_tp_pack_to_scenario_spec(&pack)?;
    Ok(TpStudioClauseIngestResult {
        source_path: path.to_path_buf(),
        pack,
        scenario,
    })
}

pub fn project_tp_pack_to_scenario_spec(
    pack: &HydratedScenarioPack,
) -> Result<SimThingScenarioSpec, TpStudioClauseIngestError> {
    project_pack_to_authority_tree_candidate(pack)
        .map_err(|e| TpStudioClauseIngestError::Projection(e.message))
}

pub fn save_scenario_authority_json_to_path(
    path: &Path,
    scenario: &SimThingScenarioSpec,
) -> Result<(), TpStudioClauseIngestError> {
    let json = serialize_scenario_authority(scenario)
        .map_err(|e| TpStudioClauseIngestError::Serde(e.to_string()))?;
    std::fs::write(path, json)?;
    Ok(())
}

pub fn load_scenario_authority_json_from_path(
    path: &Path,
) -> Result<SimThingScenarioSpec, TpStudioClauseIngestError> {
    let text = std::fs::read_to_string(path)?;
    deserialize_scenario_authority(&text)
        .map_err(|e| TpStudioClauseIngestError::Serde(e.to_string()))
}

fn resolve_clause_source_text(
    raw: &str,
    options: &TpStudioClauseIngestOptions,
) -> Result<String, TpStudioClauseIngestError> {
    if !raw.contains(CLAUSE_FIXTURE_JSON_PLACEHOLDER) {
        return Ok(raw.to_string());
    }
    let fixture = options.embedded_source_json_path.as_ref().ok_or_else(|| {
        TpStudioClauseIngestError::Projection(format!(
            "clause contains {CLAUSE_FIXTURE_JSON_PLACEHOLDER} but no embedded_source_json_path was provided"
        ))
    })?;
    if !fixture.is_file() {
        return Err(TpStudioClauseIngestError::Projection(format!(
            "embedded source_json path does not exist: {}",
            fixture.display()
        )));
    }
    let fixture_path = fixture.to_string_lossy().replace('\\', "/");
    Ok(raw.replace(CLAUSE_FIXTURE_JSON_PLACEHOLDER, &fixture_path))
}
