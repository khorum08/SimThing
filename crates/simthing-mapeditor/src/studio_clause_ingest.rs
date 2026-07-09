//! Studio-facing ClauseScript scenario ingest.
//!
//! TP-STUDIO-CLAUSE-INGEST-0 — open a native `.clause` scenario path, invoke
//! `simthing-clausething` parse/hydrate, project to canonical
//! `SimThingScenarioSpec`, and hand off to existing Studio ScenarioSpec IO.
//!
//! Does not duplicate parser logic. Does not create a second scenario authority.

use std::path::{Path, PathBuf};

use simthing_clausething::{
    hydrate_scenario, parse_raw_document, HydrateError, HydratedScenarioPack, ParseError,
};
use simthing_spec::{
    SimThingScenarioGrid, SimThingScenarioProvenance, SimThingScenarioSpec,
    SimThingStructuralGridFrame,
};
use thiserror::Error;

use crate::generation::GenerationProfile;
use crate::scenario_io::ScenarioIoError;
use crate::session::StudioSession;

/// Placeholder embedded in approved TP clause fixtures for `source_json`.
pub const CLAUSE_FIXTURE_JSON_PLACEHOLDER: &str = "{{FIXTURE_JSON}}";

/// Default embedded base-disc JSON used when resolving TP clause placeholders.
pub fn default_tp_base_disc_json_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/tp_base_disc_1500.simthing-scenario.json")
}

#[derive(Debug, Clone)]
pub struct StudioClauseIngestOptions {
    /// Absolute or workspace path substituted for `{{FIXTURE_JSON}}` in clause text.
    pub embedded_source_json_path: Option<PathBuf>,
}

impl Default for StudioClauseIngestOptions {
    fn default() -> Self {
        Self {
            embedded_source_json_path: Some(default_tp_base_disc_json_path()),
        }
    }
}

#[derive(Debug, Error)]
pub enum StudioClauseIngestError {
    #[error("clause scenario file IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("clause scenario parse error: {0}")]
    Parse(#[from] ParseError),
    #[error("clause scenario hydrate error: {0}")]
    Hydrate(#[from] HydrateError),
    #[error("clause scenario projection error: {0}")]
    Projection(String),
    #[error("studio scenario IO after clause ingest: {0}")]
    ScenarioIo(#[from] ScenarioIoError),
}

impl StudioClauseIngestError {
    /// Human-readable status text suitable for Studio status/UI surfaces.
    pub fn status_message(&self) -> String {
        self.to_string()
    }

    /// Optional hydrate token index when the underlying error is spanned.
    pub fn hydrate_token_index(&self) -> Option<usize> {
        match self {
            Self::Hydrate(err) => err.span.as_ref().map(|s| s.token_index),
            _ => None,
        }
    }
}

/// Result of Studio-facing ClauseScript scenario ingest.
#[derive(Debug, Clone)]
pub struct StudioClauseIngestResult {
    pub source_path: PathBuf,
    pub pack: HydratedScenarioPack,
    pub scenario: SimThingScenarioSpec,
}

/// Open a native `.clause` path through the Studio-facing ingest service.
///
/// Flow: read file → optional `{{FIXTURE_JSON}}` substitute → `parse_raw_document`
/// → `hydrate_scenario` → project to `SimThingScenarioSpec` (authority_root).
pub fn ingest_clause_scenario_path(
    path: &Path,
    options: &StudioClauseIngestOptions,
) -> Result<StudioClauseIngestResult, StudioClauseIngestError> {
    let raw = std::fs::read_to_string(path)?;
    let source = resolve_clause_source_text(&raw, options)?;
    let document = parse_raw_document(source.as_bytes())?;
    let pack = hydrate_scenario(&document)?;
    let scenario = project_hydrated_pack_to_scenario_spec(&pack)?;
    Ok(StudioClauseIngestResult {
        source_path: path.to_path_buf(),
        pack,
        scenario,
    })
}

/// Project a hydrated pack to the canonical Studio ScenarioSpec authority shape.
///
/// Matches the TP-FULL-TRANSPILE-0 authority projection: `authority_root` is the
/// ScenarioSpec root; STEAD lattice/frame metadata is taken from the embedded base
/// disc; placement rebind onto authority nodes is out of scope for this rung.
pub fn project_hydrated_pack_to_scenario_spec(
    pack: &HydratedScenarioPack,
) -> Result<SimThingScenarioSpec, StudioClauseIngestError> {
    let authority_root = pack.authority_root.clone().ok_or_else(|| {
        StudioClauseIngestError::Projection(
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

/// Ingest a `.clause` path and adopt it as a Studio session when STEAD mapping allows.
///
/// TP authority projections from FULL-TRANSPILE intentionally leave placements /
/// `map_container_id` empty until install rebind; those specs may fail session
/// hydrate with `SteadMappingInconsistent`. Prefer `ingest_clause_scenario_path`
/// + `save_scenario_authority_to_path` for the wiring proof path.
pub fn load_studio_session_from_clause_path(
    path: &Path,
    options: &StudioClauseIngestOptions,
    profile_hint: Option<GenerationProfile>,
) -> Result<(StudioClauseIngestResult, StudioSession), StudioClauseIngestError> {
    let ingest = ingest_clause_scenario_path(path, options)?;
    let session = StudioSession::from_loaded_scenario(
        ingest.scenario.clone(),
        path.to_path_buf(),
        profile_hint,
    )
    .map_err(ScenarioIoError::from)?;
    Ok((ingest, session))
}

fn resolve_clause_source_text(
    raw: &str,
    options: &StudioClauseIngestOptions,
) -> Result<String, StudioClauseIngestError> {
    if !raw.contains(CLAUSE_FIXTURE_JSON_PLACEHOLDER) {
        return Ok(raw.to_string());
    }
    let fixture = options.embedded_source_json_path.as_ref().ok_or_else(|| {
        StudioClauseIngestError::Projection(format!(
            "clause contains {CLAUSE_FIXTURE_JSON_PLACEHOLDER} but no embedded_source_json_path was provided"
        ))
    })?;
    if !fixture.is_file() {
        return Err(StudioClauseIngestError::Projection(format!(
            "embedded source_json path does not exist: {}",
            fixture.display()
        )));
    }
    let fixture_path = fixture
        .to_string_lossy()
        .replace('\\', "/");
    Ok(raw.replace(CLAUSE_FIXTURE_JSON_PLACEHOLDER, &fixture_path))
}
