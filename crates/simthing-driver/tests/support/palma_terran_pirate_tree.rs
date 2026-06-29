//! PALMA-PATH-3R — admitted Location → gridcell → convoy SimThing tree for structural proof.
//!
//! Builds a live recursive tree and applies generic `BoundaryRequest::Reparent` through
//! `simthing_sim::apply_structural_mutations`. Terran convoy / pirate labels stay fixture-only.

use simthing_core::{DimensionRegistry, SimThing, SimThingId, SimThingKind};
use simthing_feeder::{BoundaryRequest, MaintainerOutcome};
use simthing_gpu::SlotAllocator;
use simthing_sim::{apply_structural_mutations, SimRuntimeTree};

use super::palma_terran_pirate_fixture::{
    convoy_simthing_id, gridcell_simthing_id, CONVOY_START, FIXTURE_HEIGHT, FIXTURE_WIDTH,
};

pub const LOCATION_SIMTHING_RAW: u32 = 100;

pub fn location_simthing_id() -> SimThingId {
    SimThingId::from_session_raw(LOCATION_SIMTHING_RAW)
}

fn gridcell_kind() -> SimThingKind {
    SimThingKind::Custom("GridCell".into())
}

fn with_id(mut node: SimThing, id: SimThingId) -> SimThing {
    node.id = id;
    node
}

pub fn find_node<'a>(node: &'a SimThing, id: SimThingId) -> Option<&'a SimThing> {
    if node.id == id {
        return Some(node);
    }
    for child in &node.children {
        if let Some(found) = find_node(child, id) {
            return Some(found);
        }
    }
    None
}

pub fn find_parent(root: &SimThing, child: SimThingId) -> Option<SimThingId> {
    fn walk(node: &SimThing, child: SimThingId) -> Option<SimThingId> {
        for c in &node.children {
            if c.id == child {
                return Some(node.id);
            }
            if let Some(parent) = walk(c, child) {
                return Some(parent);
            }
        }
        None
    }
    walk(root, child)
}

/// Minimal admitted tree: World → Location → 8×8 gridcells; Terran convoy Fleet under start cell.
pub struct PalmaAdmittedTree {
    pub root: SimThing,
    pub alloc: SlotAllocator,
    pub reg: DimensionRegistry,
    pub shadow: Vec<f32>,
    pub n_dims: usize,
    pub location_id: SimThingId,
    pub convoy_id: SimThingId,
    pub convoy_parent_gridcell_id: SimThingId,
}

impl PalmaAdmittedTree {
    pub fn build() -> Self {
        let mut reg = DimensionRegistry::new();
        let n_dims = reg.total_columns;

        let mut root = SimThing::new(SimThingKind::World, 0);
        let mut alloc = SlotAllocator::new();
        alloc.alloc(root.id);

        let location = with_id(
            SimThing::new(SimThingKind::Location, 0),
            location_simthing_id(),
        );
        alloc.alloc(location.id);
        root.add_child(location);

        let location_idx = root.children.len() - 1;
        for y in 0..FIXTURE_HEIGHT as usize {
            for x in 0..FIXTURE_WIDTH as usize {
                let cell = with_id(
                    SimThing::new(gridcell_kind(), 0),
                    gridcell_simthing_id(x, y),
                );
                alloc.alloc(cell.id);
                root.children[location_idx].add_child(cell);
            }
        }

        let convoy_parent_gridcell_id = gridcell_simthing_id(CONVOY_START.0, CONVOY_START.1);
        let convoy = with_id(SimThing::new(SimThingKind::Fleet, 0), convoy_simthing_id());
        alloc.alloc(convoy.id);

        let cell = find_node_mut(&mut root, convoy_parent_gridcell_id)
            .expect("convoy start gridcell must exist in admitted tree");
        cell.add_child(convoy);

        let shadow = vec![0.0f32; alloc.capacity() * n_dims.max(1)];

        Self {
            root,
            alloc,
            reg,
            shadow,
            n_dims,
            location_id: location_simthing_id(),
            convoy_id: convoy_simthing_id(),
            convoy_parent_gridcell_id,
        }
    }

    pub fn gridcell_ids(&self) -> Vec<SimThingId> {
        find_node(&self.root, self.location_id)
            .map(|loc| loc.children.iter().map(|c| c.id).collect())
            .unwrap_or_default()
    }

    pub fn is_gridcell_child_of_location(&self, id: SimThingId) -> bool {
        find_node(&self.root, self.location_id)
            .map(|loc| loc.children.iter().any(|c| c.id == id))
            .unwrap_or(false)
    }

    pub fn parent_id(&self, child: SimThingId) -> Option<SimThingId> {
        find_parent(&self.root, child)
    }

    pub fn apply_reparent(&mut self, request: BoundaryRequest) -> MaintainerOutcome {
        let mut runtime = SimRuntimeTree::admit(self.root.clone());
        let outcome = apply_structural_mutations(
            vec![request],
            &mut runtime,
            &mut self.alloc,
            &mut self.reg,
            &mut self.shadow,
            self.n_dims,
            None,
        );
        self.root = runtime.into_admitted();
        outcome
    }
}

fn find_node_mut<'a>(node: &'a mut SimThing, id: SimThingId) -> Option<&'a mut SimThing> {
    if node.id == id {
        return Some(node);
    }
    for child in &mut node.children {
        if let Some(found) = find_node_mut(child, id) {
            return Some(found);
        }
    }
    None
}
