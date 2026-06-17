//! Recursive SimThing scenario authority.
//!
//! This is the save/load-facing scenario authority shape: a real recursive
//! `simthing_core::SimThing` tree plus structural STEAD grid placements. Render
//! views, Studio indexes, and Bevy entities are projections over this object.

use serde::{Deserialize, Serialize};
use simthing_core::{SimThing, SimThingKind};

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

impl SimThingScenarioSpec {
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
}
