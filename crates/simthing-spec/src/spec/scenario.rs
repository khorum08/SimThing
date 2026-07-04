//! Recursive SimThing scenario authority.
//!
//! Canonical save/load authority: a **`Scenario`** [`SimThing`] file root plus
//! structural STEAD grid placements and links. Scenario id, schema version,
//! provenance, and source metadata live on the Scenario root as properties —
//! sidecar `scenario_id` / `provenance` fields are transitional serde mirrors only.
//!
//! Legacy **World**-root fixtures (e.g. Terran Pirate golden fixture) deserialize
//! through an explicit compatibility path; World root is not the future ontology.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use simthing_core::{
    reserve_simthing_ids_from_tree, PropertyValue, SimPropertyId, SimThing,
    SimThingIdReservationError, SimThingKind,
};
use thiserror::Error;

pub const SIMTHING_SCENARIO_AUTHORITY_LABEL: &str = "SimThing-Spec-compliant scenario authority";
pub const SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_000);
pub const SCENARIO_STRUCTURAL_COL_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_001);
pub const SCENARIO_STRUCTURAL_ROW_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_002);
pub const SCENARIO_RENDER_WORLD_X_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_100);
pub const SCENARIO_RENDER_WORLD_Y_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_101);
pub const SCENARIO_RENDER_WORLD_Z_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_102);

/// Canonical scenario metadata on the Scenario root SimThing (string: length + UTF-8 bytes as f32).
pub const SCENARIO_ID_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_200);
pub const SCENARIO_SCHEMA_VERSION_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_201);
pub const SCENARIO_SOURCE_LABEL_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_202);
pub const SCENARIO_GENERATOR_SHAPE_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_203);
pub const SCENARIO_GENERATOR_SEED_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_204);
pub const SCENARIO_GENERATOR_PROFILE_ID_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_205);
pub const SCENARIO_GENERATOR_PARAMS_JSON_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_206);
pub const SCENARIO_NAME_CORPUS_SOURCE_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_207);
pub const SCENARIO_NAME_ASSIGNMENT_MODE_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_208);

/// Canonical Owner entity metadata on direct GameSession children (string: length + UTF-8 bytes as f32).
pub const OWNER_ID_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_300);
pub const OWNER_DISPLAY_NAME_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_301);
pub const OWNER_ARCHETYPE_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_302);
pub const OWNER_COLOR_INDEX_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_303);
/// Inert stockpile/silo placeholder — admitted by SESSION-RESOURCE-FLOW-SILOS-0.
pub const OWNER_SILO_MARKER_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_304);
/// Current stored silo amount on Owner SimThings (exact u32 f32 mirror).
pub const OWNER_SILO_CURRENT_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_305);
/// Max silo capacity on Owner SimThings (exact u32 f32 mirror).
pub const OWNER_SILO_CAPACITY_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_306);
/// Owner id reference on spatial participants — not spatial parenting.
pub const OWNER_FLOW_OWNER_REF_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_307);
/// Generic produced surplus on owner-referenced spatial participants.
pub const OWNER_FLOW_SURPLUS_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_308);
/// Generic unmet demand / deficit on owner-referenced spatial participants.
pub const OWNER_FLOW_DEFICIT_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_309);
/// Local disburse-down demand amount on owner-referenced spatial participants (exact u32 f32 mirror).
pub const OWNER_FLOW_DEMAND_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_310);
/// Disburse-down priority on owner-referenced spatial participants (lower wins; exact u32 f32 mirror).
pub const OWNER_FLOW_PRIORITY_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_311);
/// Explicit resource channel key on owner-referenced spatial participants.
pub const OWNER_FLOW_RESOURCE_KEY_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_312);

/// Reserved candidate ScenarioSpec property slots for runtime preview applied amount.
pub const RUNTIME_PREVIEW_APPLIED_SIM_PROPERTY_ID: SimPropertyId = SimPropertyId(8_302_010);
/// Reserved candidate ScenarioSpec property slots for runtime preview satisfied delta.
pub const RUNTIME_PREVIEW_SATISFIED_SIM_PROPERTY_ID: SimPropertyId = SimPropertyId(8_302_011);
/// Reserved candidate ScenarioSpec property slots for runtime preview shortfall delta.
pub const RUNTIME_PREVIEW_SHORTFALL_SIM_PROPERTY_ID: SimPropertyId = SimPropertyId(8_302_012);

/// Default disburse-down priority when metadata is absent.
pub const OWNER_FLOW_DEFAULT_PRIORITY: u32 = 100;

/// Default resource key when participant metadata carries surplus/demand only.
pub const OWNER_FLOW_DEFAULT_RESOURCE_KEY: &str = "generic";

/// Canonical GalaxyMap / WorldStateMap spatial root metadata on direct GameSession children.
pub const GALAXY_MAP_ID_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_400);
pub const GALAXY_MAP_ROLE_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_401);
pub const GALAXY_MAP_DISPLAY_NAME_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_402);
/// Gridcell role marker under GalaxyMap (`inert` / `star_system`).
pub const GALAXY_GRIDCELL_ROLE_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_403);
/// Semantic display name on a star-system GalaxyMap gridcell Location.
pub const STAR_SYSTEM_DISPLAY_NAME_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_404);

/// Deprecated compatibility alias for [`LOCAL_GRIDCELL_ROLE_PROPERTY_ID`].
pub const GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_500);
pub const PLANET_ID_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_501);
pub const PLANET_DISPLAY_NAME_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_502);
pub const PLANET_CLASS_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_503);
pub const PLANET_OWNER_REF_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_504);
pub const PLANET_ORBIT_INDEX_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_505);

/// Universal interior local-grid frame metadata on spatial gridcell Location SimThings.
pub const LOCAL_GRID_FRAME_COLS_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_515);
pub const LOCAL_GRID_FRAME_ROWS_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_516);
pub const LOCAL_GRID_DEFAULT_COLS: u32 = 1;
pub const LOCAL_GRID_DEFAULT_ROWS: u32 = 1;

/// Star-system local grid frame metadata on galactic star-system gridcell Locations.
pub const STAR_SYSTEM_LOCAL_GRID_FRAME_COLS_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_510);
pub const STAR_SYSTEM_LOCAL_GRID_FRAME_ROWS_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_511);
pub const STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS: u32 = 10;
pub const STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS: u32 = 10;

/// Local gridcell role/col/row on child Location SimThings under star-system gridcells.
pub const LOCAL_GRIDCELL_ROLE_PROPERTY_ID: SimPropertyId = GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID;
pub const LOCAL_GRIDCELL_COL_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_513);
pub const LOCAL_GRIDCELL_ROW_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_514);

pub const GALAXY_MAP_ROLE_CANONICAL: &str = "galaxy_map";
pub const GALAXY_GRIDCELL_ROLE_INERT: &str = "inert";
pub const GALAXY_GRIDCELL_ROLE_STAR_SYSTEM: &str = "star_system";
/// Deprecated compatibility alias for [`LOCAL_GRIDCELL_ROLE_PLANET`].
pub const GALAXY_CHILD_LOCATION_ROLE_PLANET: &str = "planet";
pub const GALAXY_CHILD_LOCATION_ROLE_MOON: &str = "moon";
pub const LOCAL_GRIDCELL_ROLE_INERT: &str = "inert";
pub const LOCAL_GRIDCELL_ROLE_PLANET: &str = "planet";
pub const LOCAL_GRIDCELL_ROLE_SURFACE: &str = "surface";
pub const LOCAL_GRIDCELL_ROLE_RECEIVER: &str = "receiver";

pub const SCENARIO_SCHEMA_VERSION: u32 = 1;

/// Maximum structural integer that can be mirrored exactly in an f32 property.
/// Values above this are rejected; primary authority remains `structural_grid.placements`.
pub const SCENARIO_STRUCTURAL_INTEGER_MAX: u32 = 16_777_216;

/// Save/load-facing scenario authority. **`root`** must be [`SimThingKind::Scenario`] for
/// canonical files; [`SimThingKind::World`] is legacy-only (explicit compatibility path).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimThingScenarioSpec {
    /// Transitional serde mirror — canonical authority is [`SCENARIO_ID_PROPERTY_ID`] on root.
    #[serde(default)]
    pub scenario_id: String,
    pub root: SimThing,
    pub structural_grid: SimThingScenarioGrid,
    #[serde(default)]
    pub links: Vec<SimThingScenarioLink>,
    /// Transitional serde mirror — canonical authority is Scenario-root metadata properties.
    #[serde(default)]
    pub provenance: SimThingScenarioProvenance,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScenarioRootValidationMode {
    Canonical,
    LegacyCompat,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ScenarioRootError {
    #[error("scenario authority root must be a Scenario SimThing")]
    RootIsNotScenario,
    #[error("legacy World-root scenario admitted through explicit compatibility path")]
    LegacyWorldRootAdmitted,
    #[error("legacy World-root scenario rejected: {0}")]
    LegacyWorldRootRejected(String),
    #[error("scenario authority root is not a legacy World SimThing")]
    RootIsNotWorld,
    #[error("scenario authority root kind {kind} is not Scenario or legacy World")]
    ArbitraryRootKind { kind: String },
    #[error("canonical Scenario root is missing metadata property `{0}`")]
    MissingScenarioMetadata(&'static str),
    #[error(
        "canonical Scenario metadata `{field}` on root ({root}) does not match transitional sidecar ({sidecar})"
    )]
    ScenarioMetadataMismatch {
        field: &'static str,
        root: String,
        sidecar: String,
    },
    #[error("canonical Scenario root is missing required GameSession child")]
    MissingGameSessionChild,
    #[error("canonical Scenario root has {count} GameSession children; exactly one required")]
    MultipleGameSessionChildren { count: usize },
    #[error("canonical Scenario root child is not GameSession (found {found})")]
    GameSessionChildWrongKind { found: String },
    #[error("legacy World-root scenario has no GameSession child requirement")]
    LegacyWorldRootHasNoGameSessionRequirement,
    #[error("canonical GameSession has no Owner child entities")]
    MissingOwnerEntities,
    #[error("Owner SimThing is missing non-empty owner_id metadata")]
    OwnerMissingId,
    #[error("duplicate owner_id `{owner_id}` among GameSession Owner children")]
    DuplicateOwnerId { owner_id: String },
    #[error("Owner SimThing is not a direct GameSession child")]
    OwnerNotDirectGameSessionChild,
    #[error("legacy World-root scenario has no Owner child requirement")]
    LegacyWorldRootHasNoOwnerRequirement,
    #[error("canonical GameSession has no GalaxyMap / WorldStateMap child")]
    MissingGalaxyMap,
    #[error("canonical GameSession has {count} GalaxyMap children; exactly one required")]
    MultipleGalaxyMaps { count: usize },
    #[error("GameSession child is not a canonical GalaxyMap spatial root (found {found})")]
    GalaxyMapWrongKind { found: String },
    #[error("GalaxyMap SimThing is missing non-empty galaxy_map_id metadata")]
    GalaxyMapMissingId,
    #[error("GalaxyMap SimThing is not a direct GameSession child")]
    GalaxyMapNotDirectGameSessionChild,
    #[error("legacy World-root scenario has no GalaxyMap child requirement")]
    LegacyWorldRootHasNoGalaxyMapRequirement,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimThingScenarioGrid {
    pub frame: SimThingStructuralGridFrame,
    pub map_container_id: String,
    pub placements: Vec<SimThingStructuralGridPlacement>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimThingStructuralGridFrame {
    pub width: u32,
    pub height: u32,
    pub occupied_cells: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimThingStructuralGridPlacement {
    pub location_id: String,
    pub target_id: String,
    pub system_id: u32,
    pub row: u32,
    pub col: u32,
    pub simthing_id_raw: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimThingScenarioLink {
    pub from_system_id: String,
    pub to_system_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimThingScenarioProvenance {
    pub source: String,
    pub generator_seed: u64,
    pub generator_shape: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub generator_profile_id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub generator_params_json: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name_corpus_source: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name_assignment_mode: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SteadMappingError {
    #[error("scenario authority has no spatial World subtree for STEAD validation")]
    MissingSpatialAuthority,
    #[error("scenario authority spatial root must be a World SimThing")]
    RootIsNotWorld,
    #[error("scenario authority has duplicate SimThing id {0}")]
    DuplicateSimThingId(u32),
    #[error("scenario authority structural_grid.map_container_id is missing")]
    MissingMapContainerId,
    #[error(
        "scenario authority map_container_id `{0}` does not resolve to a SimThing in the tree"
    )]
    DanglingMapContainerId(String),
    #[error(
        "scenario authority map_container_id `{map_container_id}` resolves to SimThing kind {kind}, expected Location"
    )]
    MapContainerNotLocation {
        map_container_id: String,
        kind: String,
    },
    #[error(
        "scenario authority map container `{0}` is not the spatial root nor a direct child of it"
    )]
    MapContainerNotSpatialRootChild(String),
    #[error("scenario authority is missing a galaxy map Location container")]
    MissingMapContainer,
    #[error("scenario authority map container has duplicate gridcell Location id {0}")]
    DuplicateGridcellLocationId(u32),
    #[error("scenario authority gridcell `{0}` is not a child of the declared map container")]
    GridcellNotUnderDeclaredMapContainer(String),
    #[error("scenario authority has duplicate structural placement for SimThing id {0}")]
    DuplicatePlacementForLocation(u32),
    #[error("scenario authority has duplicate structural coordinate ({col},{row})")]
    DuplicateCoordinate { col: u32, row: u32 },
    #[error("scenario authority has duplicate generated system id {0}")]
    DuplicateSystemId(u32),
    #[error("scenario authority placement `{0}` references no gridcell Location SimThing")]
    MissingGridcellLocation(String),
    #[error("scenario authority gridcell `{0}` is missing child payload SimThings")]
    GridcellMissingChildren(String),
    #[error("scenario authority gridcell `{0}` is missing mirrored structural property `{1}`")]
    MissingStructuralProperty(String, &'static str),
    #[error(
        "scenario authority gridcell `{location_id}` mirrored structural property `{property}` is {found}, expected {expected}"
    )]
    StructuralPropertyMismatch {
        location_id: String,
        property: &'static str,
        expected: u32,
        found: u32,
    },
    #[error(
        "scenario authority gridcell `{location_id}` mirrored structural property `{property}` is not an exact f32 integer <= {max}"
    )]
    StructuralPropertyNonExactFloat {
        location_id: String,
        property: &'static str,
        max: u32,
    },
    #[error("scenario authority contains render-only coordinate property id {0}")]
    RenderCoordinatePropertyPresent(u32),
    #[error("scenario authority frame occupied cells {frame} does not match placement count {placements}")]
    OccupiedCellCountMismatch { frame: u64, placements: u64 },
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ScenarioLinkError {
    #[error("scenario authority link references unknown endpoint from={from} to={to}")]
    InvalidEndpoint { from: String, to: String },
    #[error("scenario authority link is a self-link for system {system_id}")]
    SelfLink { system_id: String },
    #[error("scenario authority link is a duplicate adjacency edge from={from} to={to}")]
    DuplicateLink { from: String, to: String },
    #[error("scenario authority link is a reversed duplicate adjacency edge from={from} to={to}")]
    ReversedDuplicateLink { from: String, to: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ScenarioSerdeError {
    #[error("failed to serialize scenario authority: {0}")]
    Serialize(String),
    #[error("failed to deserialize scenario authority: {0}")]
    Deserialize(String),
    #[error("deserialized scenario authority failed STEAD validation: {0}")]
    Validation(#[from] SteadMappingError),
    #[error("deserialized scenario authority failed link validation: {0}")]
    LinkValidation(#[from] ScenarioLinkError),
    #[error("deserialized scenario authority failed id reservation: {0}")]
    IdReservation(#[from] SimThingIdReservationError),
    #[error("deserialized scenario authority failed root validation: {0}")]
    RootValidation(#[from] ScenarioRootError),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ScenarioEditError {
    #[error("scenario edit could not resolve declared map container: {0}")]
    MapContainer(#[from] SteadMappingError),
    #[error(
        "scenario edit target SimThing id {0} is not a gridcell under the declared map container"
    )]
    GridcellNotFound(u32),
    #[error("scenario edit rejected render-only property id {0}")]
    RenderPropertyForbidden(u32),
    #[error("scenario edit structural property {0} must mirror an exact f32 integer <= {1}")]
    StructuralPropertyNonExact(u32, u32),
}

impl SimThingScenarioSpec {
    pub fn authority_label() -> &'static str {
        SIMTHING_SCENARIO_AUTHORITY_LABEL
    }

    /// Canonical scenario id: Scenario-root property first, transitional sidecar second.
    pub fn canonical_scenario_id(&self) -> String {
        if self.root.kind == SimThingKind::Scenario {
            if let Some(id) = scenario_metadata_string(&self.root, SCENARIO_ID_PROPERTY_ID) {
                return id;
            }
        }
        self.scenario_id.clone()
    }

    pub fn world_root(&self) -> &SimThing {
        &self.root
    }

    pub fn validate_scenario_root_authority(
        &self,
        mode: ScenarioRootValidationMode,
    ) -> Result<(), ScenarioRootError> {
        validate_scenario_root_authority(self, mode)
    }

    pub fn validate_legacy_world_root_compatibility(&self) -> Result<(), ScenarioRootError> {
        validate_legacy_world_root_compatibility(self)
    }

    pub fn galaxy_map_container(&self) -> Option<&SimThing> {
        resolve_map_container(self).ok()
    }

    pub fn gridcell_locations(&self) -> impl Iterator<Item = &SimThing> {
        let map_container = match resolve_map_container(self) {
            Ok(container) => container,
            Err(_) => return Either::Left(std::iter::empty()),
        };
        Either::Right(
            map_container
                .children
                .iter()
                .filter(|child| child.kind == SimThingKind::Location),
        )
    }

    pub fn reserve_loaded_simthing_ids(&self) -> Result<(), SimThingIdReservationError> {
        reserve_simthing_ids_from_tree(&self.root)
    }

    pub fn validate_unique_simthing_ids(&self) -> Result<(), SteadMappingError> {
        let mut seen = BTreeSet::new();
        visit_simthings(&self.root, &mut |thing| {
            if !seen.insert(thing.id.raw()) {
                return Err(SteadMappingError::DuplicateSimThingId(thing.id.raw()));
            }
            Ok(())
        })
    }

    pub fn validate_stead_mapping_consistency(&self) -> Result<(), SteadMappingError> {
        validate_stead_mapping_consistency(self)
    }

    /// Write transitional sidecar fields from canonical Scenario-root metadata.
    pub fn sync_sidecar_from_root_metadata(&mut self) {
        sync_sidecar_from_root_metadata(self);
    }

    /// Populate Scenario-root metadata from transitional sidecar fields (legacy load path).
    pub fn sync_root_metadata_from_sidecar(&mut self) {
        sync_root_metadata_from_sidecar(self);
    }
}

pub fn scenario_metadata_string_value(text: &str) -> PropertyValue {
    let mut data = Vec::with_capacity(1 + text.len());
    data.push(text.len() as f32);
    for byte in text.bytes() {
        data.push(byte as f32);
    }
    PropertyValue::from_raw_lanes(data)
}

pub fn scenario_metadata_string(thing: &SimThing, property_id: SimPropertyId) -> Option<String> {
    let value = thing.properties.get(&property_id)?;
    let lanes = value.raw_lanes_for_serialization();
    let len = *lanes.first()? as usize;
    if lanes.len() != 1 + len {
        return None;
    }
    let bytes: Vec<u8> = lanes[1..].iter().map(|f| *f as u8).collect();
    String::from_utf8(bytes).ok()
}

pub fn scenario_metadata_u32_value(value: u32) -> PropertyValue {
    PropertyValue::from_raw_lanes(vec![value as f32])
}

pub fn scenario_metadata_u32(thing: &SimThing, property_id: SimPropertyId) -> Option<u32> {
    property_u32(thing.properties.get(&property_id)?)
}

/// Lossless u64 seed encoding: four u16 chunks stored as exact f32 integers (0..=65535).
pub fn scenario_metadata_seed_value(seed: u64) -> PropertyValue {
    PropertyValue::from_raw_lanes(vec![
        (seed & 0xFFFF) as f32,
        ((seed >> 16) & 0xFFFF) as f32,
        ((seed >> 32) & 0xFFFF) as f32,
        ((seed >> 48) & 0xFFFF) as f32,
    ])
}

pub fn scenario_metadata_seed(thing: &SimThing) -> Option<u64> {
    let value = thing.properties.get(&SCENARIO_GENERATOR_SEED_PROPERTY_ID)?;
    let lanes = value.raw_lanes_for_serialization();
    if lanes.len() != 4 {
        return None;
    }
    let a = checked_u16_f32(lanes[0])? as u64;
    let b = checked_u16_f32(lanes[1])? as u64;
    let c = checked_u16_f32(lanes[2])? as u64;
    let d = checked_u16_f32(lanes[3])? as u64;
    Some(a | (b << 16) | (c << 32) | (d << 48))
}

fn checked_u16_f32(value: f32) -> Option<u16> {
    if value.is_finite() && value >= 0.0 && value.fract() == 0.0 && value <= u16::MAX as f32 {
        Some(value as u16)
    } else {
        None
    }
}

pub fn apply_scenario_metadata_to_root(
    root: &mut SimThing,
    scenario_id: &str,
    provenance: &SimThingScenarioProvenance,
    schema_version: u32,
) {
    debug_assert_eq!(root.kind, SimThingKind::Scenario);
    root.add_property(
        SCENARIO_ID_PROPERTY_ID,
        scenario_metadata_string_value(scenario_id),
    );
    root.add_property(
        SCENARIO_SCHEMA_VERSION_PROPERTY_ID,
        scenario_metadata_u32_value(schema_version),
    );
    root.add_property(
        SCENARIO_SOURCE_LABEL_PROPERTY_ID,
        scenario_metadata_string_value(&provenance.source),
    );
    root.add_property(
        SCENARIO_GENERATOR_SHAPE_PROPERTY_ID,
        scenario_metadata_string_value(&provenance.generator_shape),
    );
    root.add_property(
        SCENARIO_GENERATOR_SEED_PROPERTY_ID,
        scenario_metadata_seed_value(provenance.generator_seed),
    );
    if !provenance.generator_profile_id.is_empty() {
        root.add_property(
            SCENARIO_GENERATOR_PROFILE_ID_PROPERTY_ID,
            scenario_metadata_string_value(&provenance.generator_profile_id),
        );
    }
    if !provenance.generator_params_json.is_empty() {
        root.add_property(
            SCENARIO_GENERATOR_PARAMS_JSON_PROPERTY_ID,
            scenario_metadata_string_value(&provenance.generator_params_json),
        );
    }
    if !provenance.name_corpus_source.is_empty() {
        root.add_property(
            SCENARIO_NAME_CORPUS_SOURCE_PROPERTY_ID,
            scenario_metadata_string_value(&provenance.name_corpus_source),
        );
    }
    if !provenance.name_assignment_mode.is_empty() {
        root.add_property(
            SCENARIO_NAME_ASSIGNMENT_MODE_PROPERTY_ID,
            scenario_metadata_string_value(&provenance.name_assignment_mode),
        );
    }
}

pub fn sync_sidecar_from_root_metadata(spec: &mut SimThingScenarioSpec) {
    if spec.root.kind != SimThingKind::Scenario {
        return;
    }
    if let Some(id) = scenario_metadata_string(&spec.root, SCENARIO_ID_PROPERTY_ID) {
        spec.scenario_id = id;
    }
    spec.provenance.source =
        scenario_metadata_string(&spec.root, SCENARIO_SOURCE_LABEL_PROPERTY_ID).unwrap_or_default();
    spec.provenance.generator_shape =
        scenario_metadata_string(&spec.root, SCENARIO_GENERATOR_SHAPE_PROPERTY_ID)
            .unwrap_or_default();
    if let Some(seed) = scenario_metadata_seed(&spec.root) {
        spec.provenance.generator_seed = seed;
    }
    spec.provenance.generator_profile_id =
        scenario_metadata_string(&spec.root, SCENARIO_GENERATOR_PROFILE_ID_PROPERTY_ID)
            .unwrap_or_default();
    spec.provenance.generator_params_json =
        scenario_metadata_string(&spec.root, SCENARIO_GENERATOR_PARAMS_JSON_PROPERTY_ID)
            .unwrap_or_default();
    spec.provenance.name_corpus_source =
        scenario_metadata_string(&spec.root, SCENARIO_NAME_CORPUS_SOURCE_PROPERTY_ID)
            .unwrap_or_default();
    spec.provenance.name_assignment_mode =
        scenario_metadata_string(&spec.root, SCENARIO_NAME_ASSIGNMENT_MODE_PROPERTY_ID)
            .unwrap_or_default();
}

pub fn sync_root_metadata_from_sidecar(spec: &mut SimThingScenarioSpec) {
    if spec.root.kind != SimThingKind::Scenario {
        return;
    }
    apply_scenario_metadata_to_root(
        &mut spec.root,
        &spec.scenario_id,
        &spec.provenance,
        SCENARIO_SCHEMA_VERSION,
    );
}

pub fn validate_scenario_root_authority(
    spec: &SimThingScenarioSpec,
    mode: ScenarioRootValidationMode,
) -> Result<(), ScenarioRootError> {
    if spec.root.kind != SimThingKind::Scenario {
        return Err(ScenarioRootError::RootIsNotScenario);
    }
    let required: &[(&'static str, SimPropertyId)] = &[
        ("scenario_id", SCENARIO_ID_PROPERTY_ID),
        (
            "scenario_schema_version",
            SCENARIO_SCHEMA_VERSION_PROPERTY_ID,
        ),
        ("source_label", SCENARIO_SOURCE_LABEL_PROPERTY_ID),
        ("generator_shape", SCENARIO_GENERATOR_SHAPE_PROPERTY_ID),
        ("generator_seed", SCENARIO_GENERATOR_SEED_PROPERTY_ID),
    ];
    for (name, property_id) in required {
        if !spec.root.properties.contains_key(property_id) {
            return Err(ScenarioRootError::MissingScenarioMetadata(name));
        }
    }
    let root_id = scenario_metadata_string(&spec.root, SCENARIO_ID_PROPERTY_ID)
        .ok_or(ScenarioRootError::MissingScenarioMetadata("scenario_id"))?;
    if root_id.trim().is_empty() {
        return Err(ScenarioRootError::MissingScenarioMetadata("scenario_id"));
    }
    if !spec.scenario_id.is_empty() && spec.scenario_id != root_id {
        return Err(ScenarioRootError::ScenarioMetadataMismatch {
            field: "scenario_id",
            root: root_id,
            sidecar: spec.scenario_id.clone(),
        });
    }
    let root_source =
        scenario_metadata_string(&spec.root, SCENARIO_SOURCE_LABEL_PROPERTY_ID).unwrap_or_default();
    if !spec.provenance.source.is_empty() && spec.provenance.source != root_source {
        return Err(ScenarioRootError::ScenarioMetadataMismatch {
            field: "source_label",
            root: root_source,
            sidecar: spec.provenance.source.clone(),
        });
    }
    let root_seed = scenario_metadata_seed(&spec.root)
        .ok_or(ScenarioRootError::MissingScenarioMetadata("generator_seed"))?;
    if spec.provenance.generator_seed != 0 && spec.provenance.generator_seed != root_seed {
        return Err(ScenarioRootError::ScenarioMetadataMismatch {
            field: "generator_seed",
            root: root_seed.to_string(),
            sidecar: spec.provenance.generator_seed.to_string(),
        });
    }
    if mode == ScenarioRootValidationMode::Canonical {
        let version = scenario_metadata_u32(&spec.root, SCENARIO_SCHEMA_VERSION_PROPERTY_ID)
            .ok_or(ScenarioRootError::MissingScenarioMetadata(
                "scenario_schema_version",
            ))?;
        if version != SCENARIO_SCHEMA_VERSION {
            return Err(ScenarioRootError::ScenarioMetadataMismatch {
                field: "scenario_schema_version",
                root: version.to_string(),
                sidecar: SCENARIO_SCHEMA_VERSION.to_string(),
            });
        }
        validate_scenario_game_session_child(spec)?;
        validate_session_owner_entities(spec)?;
        validate_session_galaxy_map(spec)?;
    }
    Ok(())
}

pub fn is_owner_entity_kind(kind: &SimThingKind) -> bool {
    matches!(kind, SimThingKind::Owner | SimThingKind::Faction)
}

pub fn owner_entity_id(thing: &SimThing) -> Option<String> {
    let id = scenario_metadata_string(thing, OWNER_ID_PROPERTY_ID)?;
    if id.trim().is_empty() {
        None
    } else {
        Some(id)
    }
}

pub fn apply_owner_entity_metadata(
    owner: &mut SimThing,
    owner_id: &str,
    display_name: &str,
    archetype: &str,
) {
    debug_assert!(is_owner_entity_kind(&owner.kind));
    owner.add_property(
        OWNER_ID_PROPERTY_ID,
        scenario_metadata_string_value(owner_id),
    );
    owner.add_property(
        OWNER_DISPLAY_NAME_PROPERTY_ID,
        scenario_metadata_string_value(display_name),
    );
    owner.add_property(
        OWNER_ARCHETYPE_PROPERTY_ID,
        scenario_metadata_string_value(archetype),
    );
    owner.add_property(
        OWNER_SILO_MARKER_PROPERTY_ID,
        scenario_metadata_u32_value(0),
    );
}

pub fn make_owner_entity(owner_id: &str, display_name: &str, archetype: &str) -> SimThing {
    let mut owner = SimThing::new(SimThingKind::Owner, 0);
    apply_owner_entity_metadata(&mut owner, owner_id, display_name, archetype);
    owner
}

/// Direct [`SimThingKind::Owner`] (or legacy [`SimThingKind::Faction`]) children of the GameSession.
pub fn game_session_owners(
    spec: &SimThingScenarioSpec,
) -> Result<Vec<&SimThing>, ScenarioRootError> {
    let game_session = game_session_child(spec)?;
    Ok(game_session
        .children
        .iter()
        .filter(|child| is_owner_entity_kind(&child.kind))
        .collect())
}

/// Canonical GameSession must have at least one direct Owner child with unique non-empty owner ids.
pub fn validate_session_owner_entities(
    spec: &SimThingScenarioSpec,
) -> Result<(), ScenarioRootError> {
    if spec.root.kind == SimThingKind::World {
        return Err(ScenarioRootError::LegacyWorldRootHasNoOwnerRequirement);
    }
    let game_session = game_session_child(spec)?;
    if find_owner_not_direct_gamesession_child(&spec.root, game_session).is_some() {
        return Err(ScenarioRootError::OwnerNotDirectGameSessionChild);
    }
    let owners = game_session_owners(spec)?;
    if owners.is_empty() {
        return Err(ScenarioRootError::MissingOwnerEntities);
    }
    let mut seen = BTreeSet::new();
    for owner in owners {
        let Some(owner_id) = owner_entity_id(owner) else {
            return Err(ScenarioRootError::OwnerMissingId);
        };
        if !seen.insert(owner_id.clone()) {
            return Err(ScenarioRootError::DuplicateOwnerId { owner_id });
        }
    }
    Ok(())
}

fn find_owner_not_direct_gamesession_child<'a>(
    scenario_root: &'a SimThing,
    game_session: &'a SimThing,
) -> Option<&'a SimThing> {
    fn walk<'a>(
        parent: &'a SimThing,
        thing: &'a SimThing,
        game_session: &'a SimThing,
    ) -> Option<&'a SimThing> {
        if is_owner_entity_kind(&thing.kind) && parent.id != game_session.id {
            return Some(thing);
        }
        for child in &thing.children {
            if let Some(found) = walk(thing, child, game_session) {
                return Some(found);
            }
        }
        None
    }
    if scenario_root.kind != SimThingKind::Scenario {
        return None;
    }
    for child in &scenario_root.children {
        if let Some(found) = walk(scenario_root, child, game_session) {
            return Some(found);
        }
    }
    None
}

pub fn is_galaxy_map_entity(thing: &SimThing) -> bool {
    thing.kind == SimThingKind::Location
        && scenario_metadata_string(thing, GALAXY_MAP_ROLE_PROPERTY_ID).as_deref()
            == Some(GALAXY_MAP_ROLE_CANONICAL)
}

pub fn galaxy_map_id(thing: &SimThing) -> Option<String> {
    let id = scenario_metadata_string(thing, GALAXY_MAP_ID_PROPERTY_ID)?;
    if id.trim().is_empty() {
        None
    } else {
        Some(id)
    }
}

pub fn apply_galaxy_map_metadata(map: &mut SimThing, map_id: &str, display_name: &str) {
    debug_assert_eq!(map.kind, SimThingKind::Location);
    map.add_property(
        GALAXY_MAP_ID_PROPERTY_ID,
        scenario_metadata_string_value(map_id),
    );
    map.add_property(
        GALAXY_MAP_ROLE_PROPERTY_ID,
        scenario_metadata_string_value(GALAXY_MAP_ROLE_CANONICAL),
    );
    map.add_property(
        GALAXY_MAP_DISPLAY_NAME_PROPERTY_ID,
        scenario_metadata_string_value(display_name),
    );
}

pub fn make_galaxy_map(map_id: &str, display_name: &str) -> SimThing {
    let mut map = SimThing::new(SimThingKind::Location, 0);
    apply_galaxy_map_metadata(&mut map, map_id, display_name);
    map
}

pub fn apply_gridcell_role_metadata(gridcell: &mut SimThing, role: &str) {
    debug_assert_eq!(gridcell.kind, SimThingKind::Location);
    gridcell.add_property(
        GALAXY_GRIDCELL_ROLE_PROPERTY_ID,
        scenario_metadata_string_value(role),
    );
}

pub fn apply_star_system_display_name_metadata(gridcell: &mut SimThing, display_name: &str) {
    debug_assert_eq!(gridcell.kind, SimThingKind::Location);
    gridcell.add_property(
        STAR_SYSTEM_DISPLAY_NAME_PROPERTY_ID,
        scenario_metadata_string_value(display_name),
    );
}

pub fn owner_display_name(thing: &SimThing) -> Option<String> {
    scenario_metadata_string(thing, OWNER_DISPLAY_NAME_PROPERTY_ID)
}

pub fn owner_archetype(thing: &SimThing) -> Option<String> {
    scenario_metadata_string(thing, OWNER_ARCHETYPE_PROPERTY_ID)
}

pub fn owner_color_index(thing: &SimThing) -> Option<u32> {
    scenario_metadata_u32(thing, OWNER_COLOR_INDEX_PROPERTY_ID)
}

pub fn owner_silo_marker(thing: &SimThing) -> Option<u32> {
    scenario_metadata_u32(thing, OWNER_SILO_MARKER_PROPERTY_ID)
}

pub fn owner_silo_current(thing: &SimThing) -> Option<u32> {
    scenario_metadata_u32(thing, OWNER_SILO_CURRENT_PROPERTY_ID)
}

pub fn owner_silo_capacity(thing: &SimThing) -> Option<u32> {
    scenario_metadata_u32(thing, OWNER_SILO_CAPACITY_PROPERTY_ID)
}

pub fn owner_flow_owner_ref(thing: &SimThing) -> Option<String> {
    scenario_metadata_string(thing, OWNER_FLOW_OWNER_REF_PROPERTY_ID)
}

pub fn owner_flow_surplus(thing: &SimThing) -> Option<u32> {
    scenario_metadata_u32(thing, OWNER_FLOW_SURPLUS_PROPERTY_ID)
}

pub fn owner_flow_deficit(thing: &SimThing) -> Option<u32> {
    scenario_metadata_u32(thing, OWNER_FLOW_DEFICIT_PROPERTY_ID)
}

pub fn owner_flow_demand(thing: &SimThing) -> Option<u32> {
    scenario_metadata_u32(thing, OWNER_FLOW_DEMAND_PROPERTY_ID)
}

pub fn owner_flow_priority(thing: &SimThing) -> Option<u32> {
    scenario_metadata_u32(thing, OWNER_FLOW_PRIORITY_PROPERTY_ID)
}

pub fn owner_flow_resource_key(thing: &SimThing) -> String {
    scenario_metadata_string(thing, OWNER_FLOW_RESOURCE_KEY_PROPERTY_ID)
        .filter(|key| !key.trim().is_empty())
        .unwrap_or_else(|| OWNER_FLOW_DEFAULT_RESOURCE_KEY.to_string())
}

pub fn owner_has_silo_metadata(owner: &SimThing) -> bool {
    owner_silo_marker(owner).is_some() || owner_silo_current(owner).is_some()
}

pub fn apply_owner_silo_metadata(owner: &mut SimThing, current: u32, capacity: Option<u32>) {
    debug_assert!(is_owner_entity_kind(&owner.kind));
    owner.add_property(
        OWNER_SILO_MARKER_PROPERTY_ID,
        scenario_metadata_u32_value(1),
    );
    owner.add_property(
        OWNER_SILO_CURRENT_PROPERTY_ID,
        scenario_metadata_u32_value(current),
    );
    if let Some(capacity) = capacity {
        owner.add_property(
            OWNER_SILO_CAPACITY_PROPERTY_ID,
            scenario_metadata_u32_value(capacity),
        );
    }
}

pub fn apply_participant_owner_flow_metadata(
    participant: &mut SimThing,
    owner_id: &str,
    surplus: u32,
    deficit: u32,
) {
    participant.add_property(
        OWNER_FLOW_OWNER_REF_PROPERTY_ID,
        scenario_metadata_string_value(owner_id),
    );
    if surplus > 0 {
        participant.add_property(
            OWNER_FLOW_SURPLUS_PROPERTY_ID,
            scenario_metadata_u32_value(surplus),
        );
    }
    if deficit > 0 {
        participant.add_property(
            OWNER_FLOW_DEFICIT_PROPERTY_ID,
            scenario_metadata_u32_value(deficit),
        );
    }
}

pub fn apply_participant_owner_flow_resource_key_metadata(
    participant: &mut SimThing,
    resource_key: &str,
) {
    if !resource_key.trim().is_empty() {
        participant.add_property(
            OWNER_FLOW_RESOURCE_KEY_PROPERTY_ID,
            scenario_metadata_string_value(resource_key),
        );
    }
}

pub fn apply_participant_owner_flow_demand_metadata(
    participant: &mut SimThing,
    owner_id: &str,
    demand: u32,
    priority: Option<u32>,
) {
    participant.add_property(
        OWNER_FLOW_OWNER_REF_PROPERTY_ID,
        scenario_metadata_string_value(owner_id),
    );
    if demand > 0 {
        participant.add_property(
            OWNER_FLOW_DEMAND_PROPERTY_ID,
            scenario_metadata_u32_value(demand),
        );
    }
    if let Some(priority) = priority {
        participant.add_property(
            OWNER_FLOW_PRIORITY_PROPERTY_ID,
            scenario_metadata_u32_value(priority),
        );
    }
}

pub fn galaxy_map_display_name(thing: &SimThing) -> Option<String> {
    scenario_metadata_string(thing, GALAXY_MAP_DISPLAY_NAME_PROPERTY_ID)
}

pub fn galaxy_map_role(thing: &SimThing) -> Option<String> {
    scenario_metadata_string(thing, GALAXY_MAP_ROLE_PROPERTY_ID)
}

pub fn gridcell_role(thing: &SimThing) -> Option<String> {
    scenario_metadata_string(thing, GALAXY_GRIDCELL_ROLE_PROPERTY_ID)
}

pub fn star_system_display_name(thing: &SimThing) -> Option<String> {
    scenario_metadata_string(thing, STAR_SYSTEM_DISPLAY_NAME_PROPERTY_ID)
        .filter(|name| !name.trim().is_empty())
}

pub fn gridcell_generated_system_id(thing: &SimThing) -> Option<u32> {
    scenario_metadata_u32(thing, SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID)
}

pub fn gridcell_structural_col(thing: &SimThing) -> Option<u32> {
    scenario_metadata_u32(thing, SCENARIO_STRUCTURAL_COL_PROPERTY_ID)
}

pub fn gridcell_structural_row(thing: &SimThing) -> Option<u32> {
    scenario_metadata_u32(thing, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID)
}

pub fn set_owner_display_name(
    spec: &mut SimThingScenarioSpec,
    owner_id: &str,
    display_name: &str,
) -> Result<(), ScenarioRootError> {
    let owners = game_session_owners(spec)?;
    let owner = owners
        .into_iter()
        .find(|owner| owner_entity_id(owner).as_deref() == Some(owner_id))
        .ok_or(ScenarioRootError::OwnerMissingId)?;
    let owner_raw = owner.id.raw();
    let game_session = game_session_child_mut(spec)?;
    let owner_mut = game_session
        .children
        .iter_mut()
        .find(|child| child.id.raw() == owner_raw)
        .ok_or(ScenarioRootError::OwnerMissingId)?;
    owner_mut.add_property(
        OWNER_DISPLAY_NAME_PROPERTY_ID,
        scenario_metadata_string_value(display_name),
    );
    Ok(())
}

pub fn set_galaxy_map_display_name(
    spec: &mut SimThingScenarioSpec,
    display_name: &str,
) -> Result<(), ScenarioRootError> {
    let galaxy_map = game_session_galaxy_map(spec)?;
    let map_raw = galaxy_map.id.raw();
    let game_session = game_session_child_mut(spec)?;
    let map_mut = game_session
        .children
        .iter_mut()
        .find(|child| child.id.raw() == map_raw)
        .expect("galaxy map is direct GameSession child");
    map_mut.add_property(
        GALAXY_MAP_DISPLAY_NAME_PROPERTY_ID,
        scenario_metadata_string_value(display_name),
    );
    Ok(())
}

/// Direct canonical GalaxyMap / WorldStateMap children of the GameSession.
pub fn game_session_galaxy_maps(
    spec: &SimThingScenarioSpec,
) -> Result<Vec<&SimThing>, ScenarioRootError> {
    let game_session = game_session_child(spec)?;
    Ok(game_session
        .children
        .iter()
        .filter(|child| is_galaxy_map_entity(child))
        .collect())
}

/// Returns the sole direct canonical GalaxyMap child of the GameSession.
pub fn game_session_galaxy_map(
    spec: &SimThingScenarioSpec,
) -> Result<&SimThing, ScenarioRootError> {
    let maps = game_session_galaxy_maps(spec)?;
    if maps.is_empty() {
        return Err(ScenarioRootError::MissingGalaxyMap);
    }
    if maps.len() > 1 {
        return Err(ScenarioRootError::MultipleGalaxyMaps { count: maps.len() });
    }
    Ok(maps[0])
}

/// Canonical GameSession must have exactly one direct GalaxyMap child with non-empty map id.
pub fn validate_session_galaxy_map(spec: &SimThingScenarioSpec) -> Result<(), ScenarioRootError> {
    if spec.root.kind == SimThingKind::World {
        return Err(ScenarioRootError::LegacyWorldRootHasNoGalaxyMapRequirement);
    }
    let game_session = game_session_child(spec)?;
    if let Some(nested) = find_galaxy_map_not_direct_gamesession_child(&spec.root, game_session) {
        let _ = nested;
        return Err(ScenarioRootError::GalaxyMapNotDirectGameSessionChild);
    }
    let galaxy_map = game_session_galaxy_map(spec)?;
    if galaxy_map_id(galaxy_map).is_none() {
        return Err(ScenarioRootError::GalaxyMapMissingId);
    }
    Ok(())
}

fn find_galaxy_map_not_direct_gamesession_child<'a>(
    scenario_root: &'a SimThing,
    game_session: &'a SimThing,
) -> Option<&'a SimThing> {
    fn walk<'a>(
        parent: &'a SimThing,
        thing: &'a SimThing,
        game_session: &'a SimThing,
    ) -> Option<&'a SimThing> {
        if is_galaxy_map_entity(thing) && parent.id != game_session.id {
            return Some(thing);
        }
        for child in &thing.children {
            if let Some(found) = walk(thing, child, game_session) {
                return Some(found);
            }
        }
        None
    }
    if scenario_root.kind != SimThingKind::Scenario {
        return None;
    }
    for child in &scenario_root.children {
        if let Some(found) = walk(scenario_root, child, game_session) {
            return Some(found);
        }
    }
    None
}

fn is_admitted_spatial_root(thing: &SimThing) -> bool {
    thing.kind == SimThingKind::World || is_galaxy_map_entity(thing)
}

/// Returns the sole direct [`SimThingKind::GameSession`] child of a canonical Scenario root.
pub fn game_session_child(spec: &SimThingScenarioSpec) -> Result<&SimThing, ScenarioRootError> {
    if spec.root.kind != SimThingKind::Scenario {
        if spec.root.kind == SimThingKind::World {
            return Err(ScenarioRootError::LegacyWorldRootHasNoGameSessionRequirement);
        }
        return Err(ScenarioRootError::RootIsNotScenario);
    }
    let gamesessions: Vec<_> = spec
        .root
        .children
        .iter()
        .filter(|child| child.kind == SimThingKind::GameSession)
        .collect();
    if gamesessions.is_empty() {
        if spec.root.children.len() == 1 {
            let child = &spec.root.children[0];
            let found = match &child.kind {
                SimThingKind::Custom(name) => name.clone(),
                other => format!("{other:?}"),
            };
            return Err(ScenarioRootError::GameSessionChildWrongKind { found });
        }
        return Err(ScenarioRootError::MissingGameSessionChild);
    }
    if gamesessions.len() > 1 || spec.root.children.len() > 1 {
        return Err(ScenarioRootError::MultipleGameSessionChildren {
            count: gamesessions.len(),
        });
    }
    Ok(gamesessions[0])
}

pub(crate) fn game_session_child_mut<'a>(
    spec: &'a mut SimThingScenarioSpec,
) -> Result<&'a mut SimThing, ScenarioRootError> {
    if spec.root.kind != SimThingKind::Scenario {
        if spec.root.kind == SimThingKind::World {
            return Err(ScenarioRootError::LegacyWorldRootHasNoGameSessionRequirement);
        }
        return Err(ScenarioRootError::RootIsNotScenario);
    }
    let gamesession_count = spec
        .root
        .children
        .iter()
        .filter(|child| child.kind == SimThingKind::GameSession)
        .count();
    if gamesession_count == 0 {
        if spec.root.children.len() == 1 {
            let child = &spec.root.children[0];
            let found = match &child.kind {
                SimThingKind::Custom(name) => name.clone(),
                other => format!("{other:?}"),
            };
            return Err(ScenarioRootError::GameSessionChildWrongKind { found });
        }
        return Err(ScenarioRootError::MissingGameSessionChild);
    }
    if gamesession_count > 1 || spec.root.children.len() > 1 {
        return Err(ScenarioRootError::MultipleGameSessionChildren {
            count: gamesession_count,
        });
    }
    Ok(spec
        .root
        .children
        .iter_mut()
        .find(|child| child.kind == SimThingKind::GameSession)
        .expect("gamesession count validated"))
}

/// Canonical Scenario roots require exactly one direct [`SimThingKind::GameSession`] child.
pub fn validate_scenario_game_session_child(
    spec: &SimThingScenarioSpec,
) -> Result<(), ScenarioRootError> {
    game_session_child(spec).map(|_| ())
}

fn scenario_spatial_root_child<'a>(scenario_root: &'a SimThing) -> Option<&'a SimThing> {
    if scenario_root.kind != SimThingKind::Scenario {
        return None;
    }
    if let Some(game_session) = scenario_root
        .children
        .iter()
        .find(|child| child.kind == SimThingKind::GameSession)
    {
        if let Some(galaxy_map) = game_session
            .children
            .iter()
            .find(|child| is_galaxy_map_entity(child))
        {
            return Some(galaxy_map);
        }
        if let Some(world) = game_session
            .children
            .iter()
            .find(|child| child.kind == SimThingKind::World)
        {
            return Some(world);
        }
    }
    scenario_root
        .children
        .iter()
        .find(|child| child.kind == SimThingKind::World)
}

pub fn validate_legacy_world_root_compatibility(
    spec: &SimThingScenarioSpec,
) -> Result<(), ScenarioRootError> {
    if spec.root.kind != SimThingKind::World {
        return Err(ScenarioRootError::RootIsNotWorld);
    }
    if spec.scenario_id.trim().is_empty() {
        return Err(ScenarioRootError::LegacyWorldRootRejected(
            "legacy World-root fixture requires transitional scenario_id sidecar".into(),
        ));
    }
    Ok(())
}

pub fn spatial_authority_root<'a>(
    spec: &'a SimThingScenarioSpec,
) -> Result<&'a SimThing, SteadMappingError> {
    match spec.root.kind {
        SimThingKind::World => Ok(&spec.root),
        SimThingKind::Scenario => scenario_spatial_root_child(&spec.root)
            .ok_or(SteadMappingError::MissingSpatialAuthority),
        _ => Err(SteadMappingError::RootIsNotWorld),
    }
}

fn is_empty_structural_grid(grid: &SimThingScenarioGrid) -> bool {
    grid.placements.is_empty() && grid.map_container_id.trim().is_empty()
}

enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R, T> Iterator for Either<L, R>
where
    L: Iterator<Item = T>,
    R: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Either::Left(iter) => iter.next(),
            Either::Right(iter) => iter.next(),
        }
    }
}

pub fn resolve_map_container<'a>(
    scenario: &'a SimThingScenarioSpec,
) -> Result<&'a SimThing, SteadMappingError> {
    let spatial_root = spatial_authority_root(scenario)?;
    let map_container_id = scenario.structural_grid.map_container_id.trim();
    if map_container_id.is_empty() {
        return Err(SteadMappingError::MissingMapContainerId);
    }
    let raw = map_container_id
        .parse::<u32>()
        .map_err(|_| SteadMappingError::DanglingMapContainerId(map_container_id.to_string()))?;
    let container = find_simthing_by_raw_id(spatial_root, raw).ok_or_else(|| {
        SteadMappingError::DanglingMapContainerId(scenario.structural_grid.map_container_id.clone())
    })?;
    if container.kind != SimThingKind::Location {
        return Err(SteadMappingError::MapContainerNotLocation {
            map_container_id: scenario.structural_grid.map_container_id.clone(),
            kind: format!("{:?}", container.kind),
        });
    }
    let is_spatial_root_child = if spatial_root.id.raw() == raw {
        is_galaxy_map_entity(spatial_root)
    } else {
        spatial_root
            .children
            .iter()
            .any(|child| child.id.raw() == raw)
    };
    if !is_spatial_root_child {
        return Err(SteadMappingError::MapContainerNotSpatialRootChild(
            scenario.structural_grid.map_container_id.clone(),
        ));
    }
    Ok(container)
}

fn spatial_authority_root_mut<'a>(
    spec: &'a mut SimThingScenarioSpec,
) -> Result<&'a mut SimThing, SteadMappingError> {
    match spec.root.kind {
        SimThingKind::World => Ok(&mut spec.root),
        SimThingKind::Scenario => {
            let game_session_idx = spec
                .root
                .children
                .iter()
                .position(|child| child.kind == SimThingKind::GameSession);
            if let Some(gs_idx) = game_session_idx {
                let galaxy_idx = spec.root.children[gs_idx]
                    .children
                    .iter()
                    .position(|child| is_galaxy_map_entity(child));
                if let Some(g_idx) = galaxy_idx {
                    return Ok(&mut spec.root.children[gs_idx].children[g_idx]);
                }
                let world_idx = spec.root.children[gs_idx]
                    .children
                    .iter()
                    .position(|child| child.kind == SimThingKind::World);
                if let Some(w_idx) = world_idx {
                    return Ok(&mut spec.root.children[gs_idx].children[w_idx]);
                }
            }
            let world_idx = spec
                .root
                .children
                .iter()
                .position(|child| child.kind == SimThingKind::World);
            world_idx
                .map(|idx| &mut spec.root.children[idx])
                .ok_or(SteadMappingError::MissingSpatialAuthority)
        }
        _ => Err(SteadMappingError::RootIsNotWorld),
    }
}

pub fn resolve_map_container_mut<'a>(
    scenario: &'a mut SimThingScenarioSpec,
) -> Result<&'a mut SimThing, SteadMappingError> {
    let map_container_id_field = scenario.structural_grid.map_container_id.clone();
    let map_container_id = map_container_id_field.trim();
    if map_container_id.is_empty() {
        return Err(SteadMappingError::MissingMapContainerId);
    }
    let raw = map_container_id
        .parse::<u32>()
        .map_err(|_| SteadMappingError::DanglingMapContainerId(map_container_id_field.clone()))?;
    let (is_spatial_root_binding, exists_in_subtree) = {
        let spatial = spatial_authority_root(scenario)?;
        let is_binding =
            spatial.id.raw() == raw || spatial.children.iter().any(|child| child.id.raw() == raw);
        let exists = find_simthing_by_raw_id(spatial, raw).is_some();
        (is_binding, exists)
    };
    let spatial_root = spatial_authority_root_mut(scenario)?;
    if spatial_root.id.raw() == raw {
        if spatial_root.kind != SimThingKind::Location {
            return Err(SteadMappingError::MapContainerNotLocation {
                map_container_id: map_container_id_field,
                kind: format!("{:?}", spatial_root.kind),
            });
        }
        return Ok(spatial_root);
    }
    if let Some(child) = spatial_root
        .children
        .iter_mut()
        .find(|child| child.id.raw() == raw)
    {
        if child.kind != SimThingKind::Location {
            return Err(SteadMappingError::MapContainerNotLocation {
                map_container_id: map_container_id_field,
                kind: format!("{:?}", child.kind),
            });
        }
        return Ok(child);
    }
    if exists_in_subtree && !is_spatial_root_binding {
        return Err(SteadMappingError::MapContainerNotSpatialRootChild(
            map_container_id_field,
        ));
    }
    Err(SteadMappingError::DanglingMapContainerId(
        map_container_id_field,
    ))
}

pub fn reserve_simthing_ids_from_scenario(
    spec: &SimThingScenarioSpec,
) -> Result<(), SimThingIdReservationError> {
    spec.reserve_loaded_simthing_ids()
}

pub fn serialize_scenario_authority(
    spec: &SimThingScenarioSpec,
) -> Result<String, ScenarioSerdeError> {
    let mut to_write = spec.clone();
    to_write.sync_sidecar_from_root_metadata();
    serde_json::to_string(&to_write).map_err(|err| ScenarioSerdeError::Serialize(err.to_string()))
}

pub fn canonical_scenario_link_pair(
    from: &str,
    to: &str,
) -> Result<(String, String), ScenarioLinkError> {
    if from.is_empty() || to.is_empty() {
        return Err(ScenarioLinkError::InvalidEndpoint {
            from: from.to_string(),
            to: to.to_string(),
        });
    }
    if from == to {
        return Err(ScenarioLinkError::SelfLink {
            system_id: from.to_string(),
        });
    }
    if from < to {
        Ok((from.to_string(), to.to_string()))
    } else {
        Ok((to.to_string(), from.to_string()))
    }
}

pub fn canonical_scenario_link_key(
    link: &SimThingScenarioLink,
) -> Result<(String, String), ScenarioLinkError> {
    canonical_scenario_link_pair(&link.from_system_id, &link.to_system_id)
}

pub fn validate_scenario_links(spec: &SimThingScenarioSpec) -> Result<(), ScenarioLinkError> {
    let known_ids: BTreeSet<String> = spec
        .structural_grid
        .placements
        .iter()
        .map(|placement| placement.system_id.to_string())
        .collect();
    let mut seen_canonical: BTreeMap<(String, String), (String, String)> = BTreeMap::new();
    for link in &spec.links {
        if link.from_system_id.is_empty() || link.to_system_id.is_empty() {
            return Err(ScenarioLinkError::InvalidEndpoint {
                from: link.from_system_id.clone(),
                to: link.to_system_id.clone(),
            });
        }
        if !known_ids.contains(&link.from_system_id) {
            return Err(ScenarioLinkError::InvalidEndpoint {
                from: link.from_system_id.clone(),
                to: link.to_system_id.clone(),
            });
        }
        if !known_ids.contains(&link.to_system_id) {
            return Err(ScenarioLinkError::InvalidEndpoint {
                from: link.from_system_id.clone(),
                to: link.to_system_id.clone(),
            });
        }
        if link.from_system_id == link.to_system_id {
            return Err(ScenarioLinkError::SelfLink {
                system_id: link.from_system_id.clone(),
            });
        }
        let canonical = canonical_scenario_link_pair(&link.from_system_id, &link.to_system_id)?;
        let directed = (link.from_system_id.clone(), link.to_system_id.clone());
        if let Some((first_from, first_to)) = seen_canonical.get(&canonical) {
            if first_from == &directed.0 && first_to == &directed.1 {
                return Err(ScenarioLinkError::DuplicateLink {
                    from: directed.0,
                    to: directed.1,
                });
            }
            return Err(ScenarioLinkError::ReversedDuplicateLink {
                from: directed.0,
                to: directed.1,
            });
        }
        seen_canonical.insert(canonical, directed);
    }
    Ok(())
}

pub fn deserialize_scenario_authority(
    src: &str,
) -> Result<SimThingScenarioSpec, ScenarioSerdeError> {
    let spec: SimThingScenarioSpec = serde_json::from_str(src)
        .map_err(|err| ScenarioSerdeError::Deserialize(err.to_string()))?;
    match spec.root.kind {
        SimThingKind::Scenario => {
            validate_scenario_root_authority(&spec, ScenarioRootValidationMode::Canonical)?;
        }
        SimThingKind::World => {
            validate_legacy_world_root_compatibility(&spec)?;
        }
        other => {
            return Err(ScenarioRootError::ArbitraryRootKind {
                kind: format!("{other:?}"),
            }
            .into());
        }
    }
    validate_stead_mapping_consistency(&spec)?;
    validate_scenario_links(&spec)?;
    reserve_simthing_ids_from_scenario(&spec)?;
    Ok(spec)
}

pub fn apply_gridcell_property_edit(
    scenario: &mut SimThingScenarioSpec,
    simthing_id_raw: u32,
    property_id: SimPropertyId,
    value: PropertyValue,
) -> Result<(), ScenarioEditError> {
    if matches!(
        property_id,
        SCENARIO_RENDER_WORLD_X_PROPERTY_ID
            | SCENARIO_RENDER_WORLD_Y_PROPERTY_ID
            | SCENARIO_RENDER_WORLD_Z_PROPERTY_ID
    ) {
        return Err(ScenarioEditError::RenderPropertyForbidden(property_id.0));
    }
    if matches!(
        property_id,
        SCENARIO_STRUCTURAL_COL_PROPERTY_ID
            | SCENARIO_STRUCTURAL_ROW_PROPERTY_ID
            | SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID
    ) && property_u32(&value).is_none()
    {
        return Err(ScenarioEditError::StructuralPropertyNonExact(
            property_id.0,
            SCENARIO_STRUCTURAL_INTEGER_MAX,
        ));
    }

    let map_container = resolve_map_container_mut(scenario)?;
    let gridcell = map_container
        .children
        .iter_mut()
        .find(|child| child.id.raw() == simthing_id_raw && child.kind == SimThingKind::Location)
        .ok_or(ScenarioEditError::GridcellNotFound(simthing_id_raw))?;
    gridcell.add_property(property_id, value);
    Ok(())
}

pub fn validate_stead_mapping_consistency(
    spec: &SimThingScenarioSpec,
) -> Result<(), SteadMappingError> {
    spec.validate_unique_simthing_ids()?;
    reject_render_coordinate_properties(&spec.root)?;
    if spec.root.kind == SimThingKind::Scenario && is_empty_structural_grid(&spec.structural_grid) {
        return Ok(());
    }
    let spatial_root = spatial_authority_root(spec)?;
    if !is_admitted_spatial_root(spatial_root) {
        return Err(SteadMappingError::RootIsNotWorld);
    }
    reject_render_coordinate_properties(spatial_root)?;

    let map_container = resolve_map_container(spec)?;

    let mut gridcells_by_raw = BTreeMap::new();
    for gridcell in map_container
        .children
        .iter()
        .filter(|child| child.kind == SimThingKind::Location)
    {
        if gridcells_by_raw
            .insert(gridcell.id.raw(), gridcell)
            .is_some()
        {
            return Err(SteadMappingError::DuplicateGridcellLocationId(
                gridcell.id.raw(),
            ));
        }
    }

    let mut placed_raw = BTreeSet::new();
    let mut coords = BTreeSet::new();
    let mut system_ids = BTreeSet::new();
    for placement in &spec.structural_grid.placements {
        if !placed_raw.insert(placement.simthing_id_raw) {
            return Err(SteadMappingError::DuplicatePlacementForLocation(
                placement.simthing_id_raw,
            ));
        }
        if !coords.insert((placement.col, placement.row)) {
            return Err(SteadMappingError::DuplicateCoordinate {
                col: placement.col,
                row: placement.row,
            });
        }
        if !system_ids.insert(placement.system_id) {
            return Err(SteadMappingError::DuplicateSystemId(placement.system_id));
        }

        let gridcell = gridcells_by_raw
            .get(&placement.simthing_id_raw)
            .ok_or_else(|| {
                if find_simthing_by_raw_id(&spec.root, placement.simthing_id_raw).is_some() {
                    SteadMappingError::GridcellNotUnderDeclaredMapContainer(
                        placement.location_id.clone(),
                    )
                } else {
                    SteadMappingError::MissingGridcellLocation(placement.location_id.clone())
                }
            })?;
        if gridcell.children.is_empty() {
            return Err(SteadMappingError::GridcellMissingChildren(
                placement.location_id.clone(),
            ));
        }
        require_u32_property(
            gridcell,
            &placement.location_id,
            SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
            "generated_system_id",
            placement.system_id,
        )?;
        require_u32_property(
            gridcell,
            &placement.location_id,
            SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
            "structural_col",
            placement.col,
        )?;
        require_u32_property(
            gridcell,
            &placement.location_id,
            SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
            "structural_row",
            placement.row,
        )?;
    }

    for (raw, _gridcell) in gridcells_by_raw {
        if !placed_raw.contains(&raw) {
            return Err(SteadMappingError::MissingGridcellLocation(format!(
                "simthing_raw_{raw}"
            )));
        }
    }

    let placement_count = spec.structural_grid.placements.len() as u64;
    if spec.structural_grid.frame.occupied_cells != placement_count {
        return Err(SteadMappingError::OccupiedCellCountMismatch {
            frame: spec.structural_grid.frame.occupied_cells,
            placements: placement_count,
        });
    }

    Ok(())
}

fn find_simthing_by_raw_id<'a>(thing: &'a SimThing, raw: u32) -> Option<&'a SimThing> {
    if thing.id.raw() == raw {
        return Some(thing);
    }
    for child in &thing.children {
        if let Some(found) = find_simthing_by_raw_id(child, raw) {
            return Some(found);
        }
    }
    None
}

fn visit_simthings(
    thing: &SimThing,
    f: &mut impl FnMut(&SimThing) -> Result<(), SteadMappingError>,
) -> Result<(), SteadMappingError> {
    f(thing)?;
    for child in &thing.children {
        visit_simthings(child, f)?;
    }
    Ok(())
}

fn reject_render_coordinate_properties(thing: &SimThing) -> Result<(), SteadMappingError> {
    for property_id in [
        SCENARIO_RENDER_WORLD_X_PROPERTY_ID,
        SCENARIO_RENDER_WORLD_Y_PROPERTY_ID,
        SCENARIO_RENDER_WORLD_Z_PROPERTY_ID,
    ] {
        if thing.properties.contains_key(&property_id) {
            return Err(SteadMappingError::RenderCoordinatePropertyPresent(
                property_id.0,
            ));
        }
    }
    for child in &thing.children {
        reject_render_coordinate_properties(child)?;
    }
    Ok(())
}

fn require_u32_property(
    thing: &SimThing,
    location_id: &str,
    property_id: SimPropertyId,
    property: &'static str,
    expected: u32,
) -> Result<(), SteadMappingError> {
    let value = thing.properties.get(&property_id).ok_or_else(|| {
        SteadMappingError::MissingStructuralProperty(location_id.to_string(), property)
    })?;
    let Some(found) = property_u32(value) else {
        return Err(SteadMappingError::StructuralPropertyNonExactFloat {
            location_id: location_id.to_string(),
            property,
            max: SCENARIO_STRUCTURAL_INTEGER_MAX,
        });
    };
    if found != expected {
        return Err(SteadMappingError::StructuralPropertyMismatch {
            location_id: location_id.to_string(),
            property,
            expected,
            found,
        });
    }
    Ok(())
}

pub fn property_u32(value: &PropertyValue) -> Option<u32> {
    let value = *value.raw_lanes_for_serialization().first()?;
    if value.is_finite()
        && value >= 0.0
        && value.fract() == 0.0
        && value <= SCENARIO_STRUCTURAL_INTEGER_MAX as f32
    {
        Some(value as u32)
    } else {
        None
    }
}

pub fn structural_property_value_u32(value: u32) -> PropertyValue {
    PropertyValue::from_raw_lanes(vec![value as f32])
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{SimThingId, SimThingKind};

    fn add_gridcell(
        map: &mut SimThing,
        system_id: u32,
        row: u32,
        col: u32,
    ) -> (u32, SimThingStructuralGridPlacement) {
        let mut cell = SimThing::new(SimThingKind::Location, 0);
        cell.add_property(
            SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
            structural_property_value_u32(system_id),
        );
        cell.add_property(
            SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
            structural_property_value_u32(col),
        );
        cell.add_property(
            SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
            structural_property_value_u32(row),
        );
        let mut payload = SimThing::new(SimThingKind::Cohort, 0);
        payload.add_property(
            SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
            structural_property_value_u32(system_id),
        );
        cell.add_child(payload);
        let cell_raw = cell.id.raw();
        let placement = SimThingStructuralGridPlacement {
            location_id: format!("cell_{system_id}"),
            target_id: format!("cell_{system_id}"),
            system_id,
            row,
            col,
            simthing_id_raw: cell_raw,
        };
        map.add_child(cell);
        (cell_raw, placement)
    }

    fn wrap_canonical_scenario_root(
        galaxy_map: SimThing,
        scenario_id: &str,
        structural_grid: SimThingScenarioGrid,
        links: Vec<SimThingScenarioLink>,
        provenance: SimThingScenarioProvenance,
    ) -> SimThingScenarioSpec {
        let mut scenario_root = SimThing::new(SimThingKind::Scenario, 0);
        apply_scenario_metadata_to_root(
            &mut scenario_root,
            scenario_id,
            &provenance,
            SCENARIO_SCHEMA_VERSION,
        );
        let mut game_session = SimThing::new(SimThingKind::GameSession, 0);
        game_session.add_child(make_owner_entity("spec_owner", "Spec Owner", "player"));
        game_session.add_child(galaxy_map);
        scenario_root.add_child(game_session);
        let mut spec = SimThingScenarioSpec {
            scenario_id: scenario_id.to_string(),
            root: scenario_root,
            structural_grid,
            links,
            provenance,
        };
        spec.sync_sidecar_from_root_metadata();
        spec
    }

    fn two_cell_scenario() -> SimThingScenarioSpec {
        let mut galaxy_map = make_galaxy_map("two_cell_galaxy", "Two Cell Galaxy");
        let map_raw = galaxy_map.id.raw();
        let (_, placement_a) = add_gridcell(&mut galaxy_map, 1, 2, 3);
        let (_, placement_b) = add_gridcell(&mut galaxy_map, 2, 2, 4);
        wrap_canonical_scenario_root(
            galaxy_map,
            "two_cell_spec",
            SimThingScenarioGrid {
                frame: SimThingStructuralGridFrame {
                    width: 8,
                    height: 8,
                    occupied_cells: 2,
                },
                map_container_id: map_raw.to_string(),
                placements: vec![placement_a, placement_b],
            },
            vec![SimThingScenarioLink {
                from_system_id: "1".to_string(),
                to_system_id: "2".to_string(),
            }],
            SimThingScenarioProvenance::default(),
        )
    }

    fn small_scenario() -> SimThingScenarioSpec {
        let mut galaxy_map = make_galaxy_map("small_galaxy", "Small Galaxy");
        let mut cell = SimThing::new(SimThingKind::Location, 0);
        cell.add_property(
            SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
            structural_property_value_u32(1),
        );
        cell.add_property(
            SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
            structural_property_value_u32(3),
        );
        cell.add_property(
            SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
            structural_property_value_u32(2),
        );
        let mut payload = SimThing::new(SimThingKind::Cohort, 0);
        payload.add_property(
            SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
            structural_property_value_u32(1),
        );
        cell.add_child(payload);
        let cell_raw = cell.id.raw();
        let map_raw = galaxy_map.id.raw();
        galaxy_map.add_child(cell);
        wrap_canonical_scenario_root(
            galaxy_map,
            "small_spec",
            SimThingScenarioGrid {
                frame: SimThingStructuralGridFrame {
                    width: 8,
                    height: 8,
                    occupied_cells: 1,
                },
                map_container_id: map_raw.to_string(),
                placements: vec![SimThingStructuralGridPlacement {
                    location_id: "small_cell".to_string(),
                    target_id: "small_cell".to_string(),
                    system_id: 1,
                    row: 2,
                    col: 3,
                    simthing_id_raw: cell_raw,
                }],
            },
            Vec::new(),
            SimThingScenarioProvenance::default(),
        )
    }

    fn spatial_root_mut(scenario: &mut SimThingScenarioSpec) -> &mut SimThing {
        spatial_authority_root_mut(scenario).expect("spatial root child")
    }

    fn legacy_world_scenario() -> SimThingScenarioSpec {
        let mut world = SimThing::new(SimThingKind::World, 0);
        let mut map = SimThing::new(SimThingKind::Location, 0);
        let mut cell = SimThing::new(SimThingKind::Location, 0);
        cell.add_property(
            SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
            structural_property_value_u32(1),
        );
        cell.add_property(
            SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
            structural_property_value_u32(3),
        );
        cell.add_property(
            SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
            structural_property_value_u32(2),
        );
        cell.add_child(SimThing::new(SimThingKind::Cohort, 0));
        let cell_raw = cell.id.raw();
        let map_raw = map.id.raw();
        map.add_child(cell);
        world.add_child(map);
        SimThingScenarioSpec {
            scenario_id: "legacy_world_spec".to_string(),
            root: world,
            structural_grid: SimThingScenarioGrid {
                frame: SimThingStructuralGridFrame {
                    width: 8,
                    height: 8,
                    occupied_cells: 1,
                },
                map_container_id: map_raw.to_string(),
                placements: vec![SimThingStructuralGridPlacement {
                    location_id: "legacy_cell".to_string(),
                    target_id: "legacy_cell".to_string(),
                    system_id: 1,
                    row: 2,
                    col: 3,
                    simthing_id_raw: cell_raw,
                }],
            },
            links: Vec::new(),
            provenance: SimThingScenarioProvenance::default(),
        }
    }

    #[test]
    fn stead_validator_accepts_declared_map_container_with_gridcells() {
        let scenario = small_scenario();
        validate_stead_mapping_consistency(&scenario).expect("valid");
        let resolved = resolve_map_container(&scenario).expect("resolve");
        assert_eq!(
            resolved.id.raw(),
            scenario
                .structural_grid
                .map_container_id
                .parse::<u32>()
                .unwrap()
        );
    }

    #[test]
    fn map_container_resolution_does_not_use_first_location_fallback() {
        let mut scenario = small_scenario();
        let decoy = SimThing::new(SimThingKind::Location, 0);
        let decoy_raw = decoy.id.raw();
        spatial_root_mut(&mut scenario).children.insert(0, decoy);
        let resolved = resolve_map_container(&scenario).expect("resolve declared container");
        assert_ne!(resolved.id.raw(), decoy_raw);
        assert_eq!(
            resolved.id.raw(),
            scenario
                .structural_grid
                .map_container_id
                .parse::<u32>()
                .unwrap()
        );
    }

    #[test]
    fn structural_integer_properties_roundtrip_exactly() {
        let value = structural_property_value_u32(42);
        assert_eq!(property_u32(&value), Some(42));
    }

    #[test]
    fn structural_grid_placement_remains_primary_authority() {
        let scenario = small_scenario();
        let placement = &scenario.structural_grid.placements[0];
        assert_eq!(placement.col, 3);
        assert_eq!(placement.row, 2);
        validate_stead_mapping_consistency(&scenario).expect("placements drive validation");
    }

    #[test]
    fn mirrored_structural_properties_match_structural_grid() {
        let scenario = small_scenario();
        validate_stead_mapping_consistency(&scenario).expect("mirrors match");
    }



    #[test]
    fn saving_root_alone_is_documented_insufficient_or_not_exposed_as_authority() {
        let scenario = small_scenario();
        let root_only = serde_json::to_string(&scenario.root).expect("root json");
        let full = serialize_scenario_authority(&scenario).expect("full authority");
        assert_ne!(root_only, full);
        assert!(full.contains("structural_grid"));
        assert!(full.contains("map_container_id"));
    }

    #[test]
    fn simthing_scenario_spec_roundtrip_preserves_root_and_structural_grid() {
        let scenario = small_scenario();
        let json = serialize_scenario_authority(&scenario).expect("serialize");
        let round = deserialize_scenario_authority(&json).expect("deserialize");
        assert_eq!(round.scenario_id, scenario.scenario_id);
        assert_eq!(round.structural_grid, scenario.structural_grid);
        assert_eq!(round.root.subtree_size(), scenario.root.subtree_size());
    }

    #[test]
    fn simthing_scenario_spec_roundtrip_preserves_map_container_binding() {
        let scenario = small_scenario();
        let json = serialize_scenario_authority(&scenario).expect("serialize");
        let round = deserialize_scenario_authority(&json).expect("deserialize");
        assert_eq!(
            round.structural_grid.map_container_id,
            scenario.structural_grid.map_container_id
        );
        resolve_map_container(&round).expect("binding preserved");
    }

    #[test]
    fn simthing_scenario_spec_roundtrip_preserves_links() {
        let scenario = two_cell_scenario();
        let json = serialize_scenario_authority(&scenario).expect("serialize");
        let round = deserialize_scenario_authority(&json).expect("deserialize");
        assert_eq!(round.links, scenario.links);
    }

    #[test]
    fn scenario_links_accept_known_distinct_endpoints() {
        let scenario = two_cell_scenario();
        validate_scenario_links(&scenario).expect("valid link");
    }

    #[test]
    fn scenario_link_canonical_key_is_deterministic() {
        let forward = SimThingScenarioLink {
            from_system_id: "2".to_string(),
            to_system_id: "1".to_string(),
        };
        let reverse = SimThingScenarioLink {
            from_system_id: "1".to_string(),
            to_system_id: "2".to_string(),
        };
        assert_eq!(
            canonical_scenario_link_key(&forward).expect("forward"),
            canonical_scenario_link_key(&reverse).expect("reverse")
        );
        assert_eq!(
            canonical_scenario_link_key(&forward).expect("forward"),
            ("1".to_string(), "2".to_string())
        );
    }

    #[test]
    fn simthing_scenario_spec_roundtrip_preserves_provenance() {
        let mut scenario = small_scenario();
        scenario.provenance.source = "test-source".to_string();
        scenario.sync_root_metadata_from_sidecar();
        let json = serialize_scenario_authority(&scenario).expect("serialize");
        let round = deserialize_scenario_authority(&json).expect("deserialize");
        assert_eq!(round.provenance, scenario.provenance);
    }

    #[test]
    fn loaded_scenario_reserves_existing_simthing_ids() {
        let mut scenario = small_scenario();
        scenario.root.id = SimThingId::from_session_raw(2_000_000);
        reserve_simthing_ids_from_scenario(&scenario).expect("reserve");
        let spawned = SimThing::new(SimThingKind::Cohort, 0);
        assert!(spawned.id.raw() > 2_000_000);
    }

    #[test]
    fn new_simthing_after_loaded_scenario_does_not_collide() {
        let scenario = small_scenario();
        let existing: BTreeSet<u32> = scenario
            .gridcell_locations()
            .map(|gridcell| gridcell.id.raw())
            .collect();
        reserve_simthing_ids_from_scenario(&scenario).expect("reserve");
        let spawned = SimThing::new(SimThingKind::Location, 0);
        assert!(!existing.contains(&spawned.id.raw()));
    }

    #[test]
    fn model_edit_applies_to_simthing_scenario_authority() {
        let mut scenario = small_scenario();
        let cell_raw = scenario.structural_grid.placements[0].simthing_id_raw;
        apply_gridcell_property_edit(
            &mut scenario,
            cell_raw,
            SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
            structural_property_value_u32(7),
        )
        .expect("edit");
        let gridcell = resolve_map_container(&scenario)
            .expect("map")
            .children
            .iter()
            .find(|child| child.id.raw() == cell_raw)
            .expect("cell");
        assert_eq!(
            property_u32(
                gridcell
                    .property(SCENARIO_STRUCTURAL_COL_PROPERTY_ID)
                    .expect("col")
            ),
            Some(7)
        );
    }
}
