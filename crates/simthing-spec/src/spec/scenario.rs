//! Recursive SimThing scenario authority.
//!
//! This is the save/load-facing scenario authority shape: a real recursive
//! `simthing_core::SimThing` tree plus structural STEAD grid placements. Render
//! views, Studio indexes, and Bevy entities are projections over this object.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use simthing_core::{
    reserve_simthing_ids_from_tree, PropertyValue, SimPropertyId, SimThing, SimThingKind,
};
use thiserror::Error;

pub const SIMTHING_SCENARIO_AUTHORITY_LABEL: &str = "SimThing-Spec-compliant scenario authority";
pub const SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_000);
pub const SCENARIO_STRUCTURAL_COL_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_001);
pub const SCENARIO_STRUCTURAL_ROW_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_002);
pub const SCENARIO_RENDER_WORLD_X_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_100);
pub const SCENARIO_RENDER_WORLD_Y_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_101);
pub const SCENARIO_RENDER_WORLD_Z_PROPERTY_ID: SimPropertyId = SimPropertyId(8_300_102);

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
    #[error("scenario authority is missing a galaxy map Location container")]
    MissingMapContainer,
    #[error("scenario authority map container has duplicate gridcell Location id {0}")]
    DuplicateGridcellLocationId(u32),
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
    #[error("scenario authority contains render-only coordinate property id {0}")]
    RenderCoordinatePropertyPresent(u32),
    #[error("scenario authority frame occupied cells {frame} does not match placement count {placements}")]
    OccupiedCellCountMismatch { frame: u64, placements: u64 },
}

impl SimThingScenarioSpec {
    pub fn authority_label() -> &'static str {
        SIMTHING_SCENARIO_AUTHORITY_LABEL
    }

    pub fn world_root(&self) -> &SimThing {
        &self.root
    }

    pub fn galaxy_map_container(&self) -> Option<&SimThing> {
        self.root
            .children
            .iter()
            .find(|child| child.kind == SimThingKind::Location)
    }

    pub fn gridcell_locations(&self) -> impl Iterator<Item = &SimThing> {
        self.galaxy_map_container()
            .into_iter()
            .flat_map(|container| container.children.iter())
            .filter(|child| child.kind == SimThingKind::Location)
    }

    pub fn reserve_loaded_simthing_ids(&self) {
        reserve_simthing_ids_from_tree(&self.root);
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

pub fn reserve_simthing_ids_from_scenario(spec: &SimThingScenarioSpec) {
    spec.reserve_loaded_simthing_ids();
}

pub fn validate_stead_mapping_consistency(
    spec: &SimThingScenarioSpec,
) -> Result<(), SteadMappingError> {
    if spec.root.kind != SimThingKind::World {
        return Err(SteadMappingError::RootIsNotWorld);
    }
    spec.validate_unique_simthing_ids()?;
    reject_render_coordinate_properties(&spec.root)?;

    let map_container = spec
        .galaxy_map_container()
        .ok_or(SteadMappingError::MissingMapContainer)?;

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
                SteadMappingError::MissingGridcellLocation(placement.location_id.clone())
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
        return Err(SteadMappingError::StructuralPropertyMismatch {
            location_id: location_id.to_string(),
            property,
            expected,
            found: u32::MAX,
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

fn property_u32(value: &PropertyValue) -> Option<u32> {
    let value = *value.data.first()?;
    if value.is_finite() && value >= 0.0 && value.fract() == 0.0 && value <= u32::MAX as f32 {
        Some(value as u32)
    } else {
        None
    }
}
