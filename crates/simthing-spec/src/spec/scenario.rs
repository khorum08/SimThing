//! Recursive SimThing scenario authority.
//!
//! This is the save/load-facing scenario authority shape: a real recursive
//! `simthing_core::SimThing` tree plus structural STEAD grid placements. Render
//! views, Studio indexes, and Bevy entities are projections over this object.
//!
//! Save/load must serialize and deserialize the whole `SimThingScenarioSpec`.
//! Naked `root: SimThing` alone is insufficient unless bundled with structural
//! grid metadata, links, and provenance.

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

/// Maximum structural integer that can be mirrored exactly in an f32 property.
/// Values above this are rejected; primary authority remains `structural_grid.placements`.
pub const SCENARIO_STRUCTURAL_INTEGER_MAX: u32 = 16_777_216;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimThingScenarioSpec {
    pub scenario_id: String,
    pub root: SimThing,
    pub structural_grid: SimThingScenarioGrid,
    #[serde(default)]
    pub links: Vec<SimThingScenarioLink>,
    #[serde(default)]
    pub provenance: SimThingScenarioProvenance,
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
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SteadMappingError {
    #[error("scenario authority root must be a World SimThing")]
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
    #[error("scenario authority map container `{0}` is not a direct child of the World root")]
    MapContainerNotWorldChild(String),
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
pub enum ScenarioSerdeError {
    #[error("failed to serialize scenario authority: {0}")]
    Serialize(String),
    #[error("failed to deserialize scenario authority: {0}")]
    Deserialize(String),
    #[error("deserialized scenario authority failed STEAD validation: {0}")]
    Validation(#[from] SteadMappingError),
    #[error("deserialized scenario authority failed id reservation: {0}")]
    IdReservation(#[from] SimThingIdReservationError),
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

    pub fn world_root(&self) -> &SimThing {
        &self.root
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
    let map_container_id = scenario.structural_grid.map_container_id.trim();
    if map_container_id.is_empty() {
        return Err(SteadMappingError::MissingMapContainerId);
    }
    let raw = map_container_id
        .parse::<u32>()
        .map_err(|_| SteadMappingError::DanglingMapContainerId(map_container_id.to_string()))?;
    let container = find_simthing_by_raw_id(&scenario.root, raw).ok_or_else(|| {
        SteadMappingError::DanglingMapContainerId(scenario.structural_grid.map_container_id.clone())
    })?;
    if container.kind != SimThingKind::Location {
        return Err(SteadMappingError::MapContainerNotLocation {
            map_container_id: scenario.structural_grid.map_container_id.clone(),
            kind: format!("{:?}", container.kind),
        });
    }
    let is_world_child = scenario
        .root
        .children
        .iter()
        .any(|child| child.id.raw() == raw);
    if !is_world_child {
        return Err(SteadMappingError::MapContainerNotWorldChild(
            scenario.structural_grid.map_container_id.clone(),
        ));
    }
    Ok(container)
}

pub fn resolve_map_container_mut<'a>(
    scenario: &'a mut SimThingScenarioSpec,
) -> Result<&'a mut SimThing, SteadMappingError> {
    let map_container_id = scenario.structural_grid.map_container_id.trim().to_string();
    if map_container_id.is_empty() {
        return Err(SteadMappingError::MissingMapContainerId);
    }
    let raw = map_container_id
        .parse::<u32>()
        .map_err(|_| SteadMappingError::DanglingMapContainerId(map_container_id.clone()))?;
    let root_children_len = scenario.root.children.len();
    for index in 0..root_children_len {
        if scenario.root.children[index].id.raw() == raw {
            let kind = scenario.root.children[index].kind.clone();
            if kind != SimThingKind::Location {
                return Err(SteadMappingError::MapContainerNotLocation {
                    map_container_id: scenario.structural_grid.map_container_id.clone(),
                    kind: format!("{kind:?}"),
                });
            }
            return Ok(&mut scenario.root.children[index]);
        }
    }
    if find_simthing_by_raw_id(&scenario.root, raw).is_some() {
        return Err(SteadMappingError::MapContainerNotWorldChild(
            scenario.structural_grid.map_container_id.clone(),
        ));
    }
    Err(SteadMappingError::DanglingMapContainerId(
        scenario.structural_grid.map_container_id.clone(),
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
    serde_json::to_string(spec).map_err(|err| ScenarioSerdeError::Serialize(err.to_string()))
}

pub fn deserialize_scenario_authority(
    src: &str,
) -> Result<SimThingScenarioSpec, ScenarioSerdeError> {
    let spec: SimThingScenarioSpec = serde_json::from_str(src)
        .map_err(|err| ScenarioSerdeError::Deserialize(err.to_string()))?;
    validate_stead_mapping_consistency(&spec)?;
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
    if spec.root.kind != SimThingKind::World {
        return Err(SteadMappingError::RootIsNotWorld);
    }
    spec.validate_unique_simthing_ids()?;
    reject_render_coordinate_properties(&spec.root)?;

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
    let value = *value.data.first()?;
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
    PropertyValue {
        data: vec![value as f32],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{SimThingId, SimThingKind};

    fn small_scenario() -> SimThingScenarioSpec {
        let mut root = SimThing::new(SimThingKind::World, 0);
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
        let mut payload = SimThing::new(SimThingKind::Cohort, 0);
        payload.add_property(
            SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
            structural_property_value_u32(1),
        );
        cell.add_child(payload);
        let cell_raw = cell.id.raw();
        let map_raw = map.id.raw();
        map.add_child(cell);
        root.add_child(map);
        SimThingScenarioSpec {
            scenario_id: "small_spec".to_string(),
            root,
            structural_grid: SimThingScenarioGrid {
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
            links: Vec::new(),
            provenance: SimThingScenarioProvenance::default(),
        }
    }

    #[test]
    fn stead_validator_rejects_missing_map_container_id() {
        let mut scenario = small_scenario();
        scenario.structural_grid.map_container_id.clear();
        let err = validate_stead_mapping_consistency(&scenario).expect_err("missing id");
        assert!(matches!(err, SteadMappingError::MissingMapContainerId));
    }

    #[test]
    fn stead_validator_rejects_dangling_map_container_id() {
        let mut scenario = small_scenario();
        scenario.structural_grid.map_container_id = "99999999".to_string();
        let err = validate_stead_mapping_consistency(&scenario).expect_err("dangling id");
        assert!(matches!(err, SteadMappingError::DanglingMapContainerId(_)));
    }

    #[test]
    fn stead_validator_rejects_map_container_id_pointing_to_non_location() {
        let mut scenario = small_scenario();
        let cohort = SimThing::new(SimThingKind::Cohort, 0);
        let cohort_raw = cohort.id.raw();
        scenario.root.add_child(cohort);
        scenario.structural_grid.map_container_id = cohort_raw.to_string();
        let err = validate_stead_mapping_consistency(&scenario).expect_err("non-location");
        assert!(matches!(
            err,
            SteadMappingError::MapContainerNotLocation { .. }
        ));
    }

    #[test]
    fn stead_validator_rejects_gridcell_not_under_declared_map_container() {
        let mut scenario = small_scenario();
        let mut other_map = SimThing::new(SimThingKind::Location, 0);
        let mut orphan = SimThing::new(SimThingKind::Location, 0);
        orphan.add_property(
            SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
            structural_property_value_u32(2),
        );
        orphan.add_property(
            SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
            structural_property_value_u32(4),
        );
        orphan.add_property(
            SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
            structural_property_value_u32(4),
        );
        orphan.add_child(SimThing::new(SimThingKind::Cohort, 0));
        let orphan_raw = orphan.id.raw();
        other_map.add_child(orphan);
        scenario.root.add_child(other_map);
        scenario
            .structural_grid
            .placements
            .push(SimThingStructuralGridPlacement {
                location_id: "orphan".to_string(),
                target_id: "orphan".to_string(),
                system_id: 2,
                row: 4,
                col: 4,
                simthing_id_raw: orphan_raw,
            });
        scenario.structural_grid.frame.occupied_cells = 2;
        let err = validate_stead_mapping_consistency(&scenario).expect_err("orphan gridcell");
        assert!(matches!(
            err,
            SteadMappingError::GridcellNotUnderDeclaredMapContainer(_)
        ));
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
        scenario.root.children.insert(0, decoy);
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
    fn structural_integer_property_rejects_or_avoids_non_exact_f32_range() {
        let above_max = PropertyValue {
            data: vec![20_000_000.0],
        };
        assert_eq!(property_u32(&above_max), None);
        assert_eq!(
            property_u32(&structural_property_value_u32(
                SCENARIO_STRUCTURAL_INTEGER_MAX
            )),
            Some(SCENARIO_STRUCTURAL_INTEGER_MAX)
        );
        let fractional = PropertyValue { data: vec![1.5] };
        assert_eq!(property_u32(&fractional), None);
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
    fn validator_rejects_structural_property_mismatch() {
        let mut scenario = small_scenario();
        let map_container = resolve_map_container_mut(&mut scenario).expect("map");
        map_container.children[0].add_property(
            SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
            structural_property_value_u32(99),
        );
        let err = validate_stead_mapping_consistency(&scenario).expect_err("mismatch");
        assert!(matches!(
            err,
            SteadMappingError::StructuralPropertyMismatch { .. }
        ));
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
        let mut scenario = small_scenario();
        scenario.links.push(SimThingScenarioLink {
            from_system_id: "1".to_string(),
            to_system_id: "2".to_string(),
        });
        let json = serialize_scenario_authority(&scenario).expect("serialize");
        let round = deserialize_scenario_authority(&json).expect("deserialize");
        assert_eq!(round.links, scenario.links);
    }

    #[test]
    fn simthing_scenario_spec_roundtrip_preserves_provenance() {
        let mut scenario = small_scenario();
        scenario.provenance.source = "test-source".to_string();
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
    fn loaded_scenario_rejects_duplicate_simthing_ids() {
        let mut scenario = small_scenario();
        scenario.root.children[0].id = scenario.root.id;
        let err = reserve_simthing_ids_from_scenario(&scenario).expect_err("duplicate");
        assert!(matches!(err, SimThingIdReservationError::DuplicateId(_)));
    }

    #[test]
    fn loaded_scenario_rejects_or_reports_exhausted_id_space() {
        let mut world = SimThing::new(SimThingKind::World, 0);
        world.id = SimThingId::from_session_raw(u32::MAX);
        let err = reserve_simthing_ids_from_tree(&world).expect_err("exhausted");
        assert!(matches!(err, SimThingIdReservationError::IdSpaceExhausted));
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
