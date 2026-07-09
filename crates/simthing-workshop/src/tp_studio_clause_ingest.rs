//! Workshop-homed TP candidate: ClauseScript scenario → ScenarioSpec ingest proof.
//!
//! TP-STUDIO-CLAUSE-INGEST-0R — scenario-candidate service. Exists only because the
//! 0.0.8.5 Terran-Pirate track needs a Studio wiring candidate. Not a production
//! Studio API; elevation to `simthing-mapeditor` requires DA/Owner admission.
//!
//! Reuses production parse/hydrate (`simthing-clausething`) and ScenarioSpec
//! authority serde (`simthing-spec` — the same layer mapeditor `scenario_io` wraps).
//! Does not mint a second authority model.

use std::path::{Path, PathBuf};

use simthing_clausething::{
    hydrate_scenario, parse_raw_document, HydrateError, HydratedScenarioPack, ParseError,
};
use simthing_spec::{
    deserialize_scenario_authority, serialize_scenario_authority, SimThingScenarioGrid,
    SimThingScenarioProvenance, SimThingScenarioSpec, SimThingStructuralGridFrame,
};
use thiserror::Error;

/// Placeholder embedded in approved TP clause fixtures for `source_json`.
pub const CLAUSE_FIXTURE_JSON_PLACEHOLDER: &str = "{{FIXTURE_JSON}}";

/// Workshop-default embedded base-disc JSON for the approved TP clause fixture.
///
/// Candidate/default only — not a production Studio default.
pub fn default_tp_base_disc_json_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-mapeditor/tests/fixtures/tp_base_disc_1500.simthing-scenario.json")
}

/// Options for the workshop-homed TP clause ingest candidate.
#[derive(Debug, Clone)]
pub struct TpStudioClauseIngestOptions {
    /// Path substituted for `{{FIXTURE_JSON}}` in clause text.
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
    /// Status text suitable for logs / candidate UI surfaces.
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

/// Result of workshop-homed TP ClauseScript scenario ingest.
#[derive(Debug, Clone)]
pub struct TpStudioClauseIngestResult {
    pub source_path: PathBuf,
    pub pack: HydratedScenarioPack,
    pub scenario: SimThingScenarioSpec,
}

/// Workshop-homed candidate: open a native `.clause` path for the TP track.
///
/// Flow: read → optional `{{FIXTURE_JSON}}` substitute → `parse_raw_document`
/// → `hydrate_scenario` → project to `SimThingScenarioSpec` candidate shape.
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

/// Project a hydrated TP pack to ScenarioSpec candidate authority shape.
///
/// Matches the FULL-TRANSPILE candidate projection: `authority_root` is root;
/// STEAD frame/provenance from embedded base; placements left empty until install rebind.
pub fn project_tp_pack_to_scenario_spec(
    pack: &HydratedScenarioPack,
) -> Result<SimThingScenarioSpec, TpStudioClauseIngestError> {
    let authority_root = pack.authority_root.clone().ok_or_else(|| {
        TpStudioClauseIngestError::Projection(
            "hydrated pack is missing authority_root; cannot project to SimThingScenarioSpec"
                .to_string(),
        )
    })?;

    let (frame, provenance) = if let Some(embedded) = pack.embedded_static_galaxy_scenarios.first()
    {
        (
            embedded.source_structural_grid.frame,
            SimThingScenarioProvenance {
                source: embedded.provenance.source.clone(),
                generator_seed: embedded.provenance.generator_seed,
                generator_shape: embedded.provenance.generator_shape.clone(),
                generator_profile_id: embedded.provenance.generator_profile_id.clone(),
                generator_params_json: embedded.provenance.generator_params_json.clone(),
                name_corpus_source: embedded.provenance.name_corpus_source.clone(),
                name_assignment_mode: embedded.provenance.name_assignment_mode.clone(),
            },
        )
    } else {
        (
            SimThingStructuralGridFrame {
                width: 0,
                height: 0,
                occupied_cells: 0,
            },
            SimThingScenarioProvenance {
                source: format!("clause:{}", pack.scenario_id),
                ..SimThingScenarioProvenance::default()
            },
        )
    };

    Ok(SimThingScenarioSpec {
        scenario_id: pack.scenario_id.clone(),
        root: authority_root,
        structural_grid: SimThingScenarioGrid {
            frame,
            map_container_id: String::new(),
            placements: Vec::new(),
        },
        links: Vec::new(),
        provenance,
    })
}

/// Save ScenarioSpec via production authority serde (same layer mapeditor scenario_io uses).
pub fn save_scenario_authority_json_to_path(
    path: &Path,
    scenario: &SimThingScenarioSpec,
) -> Result<(), TpStudioClauseIngestError> {
    let json = serialize_scenario_authority(scenario)
        .map_err(|e| TpStudioClauseIngestError::Serde(e.to_string()))?;
    std::fs::write(path, json)?;
    Ok(())
}

/// Load ScenarioSpec via production authority serde (same layer mapeditor scenario_io uses).
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
