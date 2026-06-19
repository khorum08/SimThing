//! Studio-facing document/projection over canonical Scenario-root authority.
//!
//! Presentation/editor model only — not runtime simulation authority.

use simthing_core::SimThingKind;
use simthing_spec::{
    galaxy_map_display_name, galaxy_map_id, galaxy_map_role, game_session_child,
    game_session_galaxy_map, game_session_owners, gridcell_generated_system_id, gridcell_role,
    gridcell_structural_col, gridcell_structural_row, is_galaxy_map_entity, owner_archetype,
    owner_color_index, owner_display_name, owner_entity_id, owner_silo_marker,
    resolve_map_container, scenario_metadata_string, scenario_metadata_u32, spatial_authority_root,
    validate_legacy_world_root_compatibility, validate_scenario_root_authority, ScenarioRootError,
    ScenarioRootValidationMode, SimThingScenarioSpec, GALAXY_GRIDCELL_ROLE_INERT,
    GALAXY_GRIDCELL_ROLE_STAR_SYSTEM, SCENARIO_SCHEMA_VERSION_PROPERTY_ID,
    SCENARIO_SOURCE_LABEL_PROPERTY_ID,
};
use thiserror::Error;

use crate::studio_admission_report::StudioScenarioAdmissionSummary;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudioScenarioAuthorityKind {
    CanonicalScenario,
    LegacyWorldRoot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudioGridcellRole {
    Inert,
    StarSystem,
    Unknown,
}

impl StudioGridcellRole {
    pub fn from_role_str(role: Option<&str>) -> Self {
        match role {
            Some(GALAXY_GRIDCELL_ROLE_INERT) => Self::Inert,
            Some(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM) => Self::StarSystem,
            Some(_) => Self::Unknown,
            None => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioGameSessionView {
    pub simthing_id_raw: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioOwnerView {
    pub simthing_id_raw: u32,
    pub owner_id: String,
    pub display_name: Option<String>,
    pub archetype: Option<String>,
    pub color_index: Option<u32>,
    pub silo_marker: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioGalaxyMapView {
    pub simthing_id_raw: u32,
    pub galaxy_map_id: Option<String>,
    pub display_name: Option<String>,
    pub role: Option<String>,
    pub is_canonical_galaxy_map: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioGridcellView {
    pub simthing_id_raw: u32,
    pub role: StudioGridcellRole,
    pub generated_system_id: Option<u32>,
    pub structural_col: Option<u32>,
    pub structural_row: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StudioScenarioDocument {
    pub authority_kind: StudioScenarioAuthorityKind,
    pub scenario_id: String,
    pub schema_version: Option<u32>,
    pub source_label: Option<String>,
    pub game_session: Option<StudioGameSessionView>,
    pub owners: Vec<StudioOwnerView>,
    pub galaxy_map: Option<StudioGalaxyMapView>,
    pub gridcells: Vec<StudioGridcellView>,
    pub admission_summary: Option<StudioScenarioAdmissionSummary>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum StudioScenarioDocumentError {
    #[error("canonical scenario validation failed: {0}")]
    CanonicalValidation(#[from] ScenarioRootError),
    #[error("legacy World-root compatibility failed: {0}")]
    LegacyWorldRoot(String),
    #[error("spatial map container could not be resolved: {0}")]
    SpatialAuthority(String),
}

pub fn build_studio_scenario_document(
    spec: &SimThingScenarioSpec,
) -> Result<StudioScenarioDocument, StudioScenarioDocumentError> {
    build_studio_scenario_document_with_admission(spec, None)
}

pub fn build_studio_scenario_document_with_admission(
    spec: &SimThingScenarioSpec,
    admission_summary: Option<StudioScenarioAdmissionSummary>,
) -> Result<StudioScenarioDocument, StudioScenarioDocumentError> {
    let mut document = match &spec.root.kind {
        SimThingKind::Scenario => {
            validate_scenario_root_authority(spec, ScenarioRootValidationMode::Canonical)?;
            build_canonical_document(spec)?
        }
        SimThingKind::World => {
            validate_legacy_world_root_compatibility(spec)
                .map_err(|err| StudioScenarioDocumentError::LegacyWorldRoot(err.to_string()))?;
            build_legacy_world_document(spec)?
        }
        other => {
            return Err(StudioScenarioDocumentError::LegacyWorldRoot(format!(
                "unsupported root kind {other:?}"
            )));
        }
    };
    document.admission_summary = admission_summary;
    Ok(document)
}

fn build_canonical_document(
    spec: &SimThingScenarioSpec,
) -> Result<StudioScenarioDocument, StudioScenarioDocumentError> {
    let game_session = game_session_child(spec)?;
    let owners: Vec<StudioOwnerView> = game_session_owners(spec)?
        .into_iter()
        .map(owner_to_view)
        .collect();
    let galaxy_map_thing = game_session_galaxy_map(spec)?;
    let galaxy_map = Some(galaxy_map_to_view(galaxy_map_thing, true));
    let gridcells = gridcells_under_map(galaxy_map_thing);

    Ok(StudioScenarioDocument {
        authority_kind: StudioScenarioAuthorityKind::CanonicalScenario,
        scenario_id: spec.canonical_scenario_id(),
        schema_version: scenario_metadata_u32(&spec.root, SCENARIO_SCHEMA_VERSION_PROPERTY_ID),
        source_label: scenario_metadata_string(&spec.root, SCENARIO_SOURCE_LABEL_PROPERTY_ID),
        game_session: Some(StudioGameSessionView {
            simthing_id_raw: game_session.id.raw(),
        }),
        owners,
        galaxy_map,
        gridcells,
        admission_summary: None,
    })
}

fn build_legacy_world_document(
    spec: &SimThingScenarioSpec,
) -> Result<StudioScenarioDocument, StudioScenarioDocumentError> {
    let map_container = resolve_map_container(spec)
        .map_err(|err| StudioScenarioDocumentError::SpatialAuthority(err.to_string()))?;
    let is_galaxy = is_galaxy_map_entity(map_container);

    Ok(StudioScenarioDocument {
        authority_kind: StudioScenarioAuthorityKind::LegacyWorldRoot,
        scenario_id: spec.scenario_id.clone(),
        schema_version: None,
        source_label: None,
        game_session: None,
        owners: Vec::new(),
        galaxy_map: Some(galaxy_map_to_view(map_container, is_galaxy)),
        gridcells: gridcells_under_map(map_container),
        admission_summary: None,
    })
}

fn owner_to_view(owner: &simthing_core::SimThing) -> StudioOwnerView {
    StudioOwnerView {
        simthing_id_raw: owner.id.raw(),
        owner_id: owner_entity_id(owner).unwrap_or_default(),
        display_name: owner_display_name(owner),
        archetype: owner_archetype(owner),
        color_index: owner_color_index(owner),
        silo_marker: owner_silo_marker(owner),
    }
}

fn galaxy_map_to_view(map: &simthing_core::SimThing, is_canonical: bool) -> StudioGalaxyMapView {
    StudioGalaxyMapView {
        simthing_id_raw: map.id.raw(),
        galaxy_map_id: galaxy_map_id(map).or_else(|| {
            if is_canonical {
                None
            } else {
                Some(map.id.raw().to_string())
            }
        }),
        display_name: galaxy_map_display_name(map),
        role: galaxy_map_role(map),
        is_canonical_galaxy_map: is_canonical,
    }
}

fn gridcells_under_map(map: &simthing_core::SimThing) -> Vec<StudioGridcellView> {
    map.children
        .iter()
        .filter(|child| child.kind == SimThingKind::Location && !is_galaxy_map_entity(child))
        .map(|gridcell| StudioGridcellView {
            simthing_id_raw: gridcell.id.raw(),
            role: StudioGridcellRole::from_role_str(gridcell_role(gridcell).as_deref()),
            generated_system_id: gridcell_generated_system_id(gridcell),
            structural_col: gridcell_structural_col(gridcell),
            structural_row: gridcell_structural_row(gridcell),
        })
        .collect()
}

/// Rebuild galaxy-map gridcells from the canonical GalaxyMap child (not legacy World assumptions).
pub fn studio_galaxy_map_gridcells_from_spec(
    spec: &SimThingScenarioSpec,
) -> Result<Vec<StudioGridcellView>, StudioScenarioDocumentError> {
    if matches!(spec.root.kind, SimThingKind::Scenario) {
        let galaxy_map = game_session_galaxy_map(spec)?;
        Ok(gridcells_under_map(galaxy_map))
    } else {
        let spatial = spatial_authority_root(spec)
            .map_err(|err| StudioScenarioDocumentError::SpatialAuthority(err.to_string()))?;
        Ok(gridcells_under_map(spatial))
    }
}

pub fn load_canonical_studio_document_from_path(
    path: &std::path::Path,
) -> Result<(SimThingScenarioSpec, StudioScenarioDocument), crate::scenario_io::ScenarioIoError> {
    let spec = crate::scenario_io::load_scenario_authority_from_path(path)?;
    let admission_summary =
        crate::studio_admission_report::build_studio_admission_summary_from_spec(
            &spec.scenario_id,
            &spec,
        );
    let document = build_studio_scenario_document_with_admission(&spec, Some(admission_summary))?;
    Ok((spec, document))
}

pub fn save_studio_scenario_with_document_roundtrip(
    spec: &SimThingScenarioSpec,
    path: &std::path::Path,
) -> Result<StudioScenarioDocument, crate::scenario_io::ScenarioIoError> {
    crate::scenario_io::save_scenario_authority_to_path(path, spec)?;
    let reloaded = crate::scenario_io::load_scenario_authority_from_path(path)?;
    Ok(build_studio_scenario_document(&reloaded)?)
}
