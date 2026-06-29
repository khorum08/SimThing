//! Opaque runtime tree root admitted at the CPU boundary — no public `.kind`.
//!
//! `BoundaryProtocol` owns a `SimRuntimeTree` instead of exposing raw `SimThing`
//! construction at the sim crate boundary.
//!
//! Direct kind construction at the boundary is uncompilable
//! (`boundary_protocol_rejects_semantic_root_compile_fail`):
//!
//! ```compile_fail
//! use simthing_core::{DimensionRegistry, SimThing, SimThingKind};
//! use simthing_gpu::SlotAllocator;
//! use simthing_sim::BoundaryProtocol;
//!
//! fn admit_kind_at_boundary() {
//!     let _ = BoundaryProtocol::new(
//!         SimThing::new(SimThingKind::World, 0),
//!         DimensionRegistry::default(),
//!         SlotAllocator::new(1),
//!     );
//! }
//! ```
//!
//! `SimRuntimeTree` exposes no kind accessor (`sim_runtime_tree_hides_kind_compile_fail`):
//!
//! ```compile_fail
//! use simthing_sim::SimRuntimeTree;
//!
//! fn peek_runtime_kind(v: SimRuntimeTree) {
//!     let _ = v.kind;
//! }
//! ```
//!
//! Raw-tree borrows are not public (`sim_runtime_tree_rejects_access_kind_backdoor_compile_fail`):
//!
//! ```compile_fail
//! use simthing_sim::SimRuntimeTree;
//!
//! fn peek_kind_via_access(v: SimRuntimeTree) {
//!     let _ = v.access(|root| root.kind.clone());
//! }
//! ```
//!
//! ```compile_fail
//! use simthing_sim::SimRuntimeTree;
//!
//! fn peek_kind_via_access_mut(mut v: SimRuntimeTree) {
//!     v.access_mut(|root| root.kind.clone());
//! }
//! ```

use serde::{Deserialize, Serialize};
use simthing_core::{
    DimensionRegistry, OverlayId, PropertyValue, SimPropertyId, SimThing, SimThingId,
};
use simthing_gpu::SlotAllocator;
use std::collections::HashSet;
use std::fmt;

/// Kind-free snapshot of one node for public queries (no `.kind`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeNodeSnapshot {
    pub id: SimThingId,
    pub children: Vec<SimThingId>,
    pub overlay_ids: Vec<OverlayId>,
    pub property_ids: Vec<SimPropertyId>,
}

/// Runtime tree root admitted once at the CPU boundary — semantic kind is not
/// readable through this type's public surface.
#[derive(Clone)]
pub struct SimRuntimeTree {
    inner: SimThing,
}

impl SimRuntimeTree {
    /// Admit an authored tree from spec/driver/core at session open.
    pub fn admit(inner: SimThing) -> Self {
        Self { inner }
    }

    pub fn id(&self) -> SimThingId {
        self.inner.id
    }

    pub fn subtree_size(&self) -> usize {
        self.inner.subtree_size()
    }

    /// CPU-boundary admission inverse — returns the admitted tree by value.
    pub fn into_admitted(self) -> SimThing {
        self.inner
    }

    /// Replace the admitted tree; returns the previous tree by value.
    pub fn replace(&mut self, tree: SimThing) -> SimThing {
        std::mem::replace(&mut self.inner, tree)
    }

    pub fn direct_child_id(&self, index: usize) -> Option<SimThingId> {
        self.inner.children.get(index).map(|c| c.id)
    }

    pub fn direct_child_ids(&self) -> Vec<SimThingId> {
        self.inner.children.iter().map(|c| c.id).collect()
    }

    pub fn child_id(&self, parent: SimThingId, index: usize) -> Option<SimThingId> {
        let node = find_node(&self.inner, parent)?;
        node.children.get(index).map(|c| c.id)
    }

    pub fn child_count(&self, id: SimThingId) -> Option<usize> {
        find_node(&self.inner, id).map(|n| n.children.len())
    }

    pub fn overlay_count(&self, id: SimThingId) -> Option<usize> {
        find_node(&self.inner, id).map(|n| n.overlays.len())
    }

    pub fn has_overlay(&self, id: SimThingId, overlay_id: OverlayId) -> bool {
        find_node(&self.inner, id)
            .map(|n| n.overlays.iter().any(|o| o.id == overlay_id))
            .unwrap_or(false)
    }

    pub fn contains_id(&self, id: SimThingId) -> bool {
        find_node(&self.inner, id).is_some()
    }

    /// Query one node without exposing raw `SimThing` or `.kind`.
    pub fn snapshot_node(&self, id: SimThingId) -> Option<RuntimeNodeSnapshot> {
        let node = find_node(&self.inner, id)?;
        Some(RuntimeNodeSnapshot {
            id: node.id,
            children: node.children.iter().map(|c| c.id).collect(),
            overlay_ids: node.overlays.iter().map(|o| o.id).collect(),
            property_ids: node.properties.keys().copied().collect(),
        })
    }

    pub fn add_property_to_node(
        &mut self,
        target: SimThingId,
        property_id: SimPropertyId,
        value: PropertyValue,
    ) -> bool {
        if let Some(node) = find_node_mut(&mut self.inner, target) {
            node.add_property(property_id, value);
            true
        } else {
            false
        }
    }

    pub fn seed_properties_on_node(
        &mut self,
        target: SimThingId,
        props: &HashSet<SimPropertyId>,
        registry: &DimensionRegistry,
    ) {
        if props.is_empty() {
            return;
        }
        if let Some(node) = find_node_mut(&mut self.inner, target) {
            for prop_id in props {
                if !node.properties.contains_key(prop_id) && registry.is_active(*prop_id) {
                    node.add_property(*prop_id, registry.property(*prop_id).default_value());
                }
            }
        }
    }

    pub fn project_to_values(
        &self,
        registry: &DimensionRegistry,
        allocator: &SlotAllocator,
        n_dims: usize,
        out: &mut [f32],
    ) {
        simthing_gpu::project_tree_to_values(&self.inner, registry, allocator, n_dims, out);
    }

    pub(crate) fn access<R>(&self, f: impl FnOnce(&SimThing) -> R) -> R {
        f(&self.inner)
    }

    pub(crate) fn access_mut<R>(&mut self, f: impl FnOnce(&mut SimThing) -> R) -> R {
        f(&mut self.inner)
    }

    pub(crate) fn inner(&self) -> &SimThing {
        &self.inner
    }

    pub(crate) fn inner_mut(&mut self) -> &mut SimThing {
        &mut self.inner
    }
}

impl fmt::Debug for SimRuntimeTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimRuntimeTree")
            .field("id", &self.inner.id)
            .field("subtree_size", &self.inner.subtree_size())
            .finish()
    }
}

impl Serialize for SimRuntimeTree {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.inner.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SimRuntimeTree {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        SimThing::deserialize(deserializer).map(Self::admit)
    }
}

fn find_node<'a>(root: &'a SimThing, id: SimThingId) -> Option<&'a SimThing> {
    if root.id == id {
        return Some(root);
    }
    for child in &root.children {
        if let Some(found) = find_node(child, id) {
            return Some(found);
        }
    }
    None
}

fn find_node_mut<'a>(root: &'a mut SimThing, id: SimThingId) -> Option<&'a mut SimThing> {
    if root.id == id {
        return Some(root);
    }
    for child in &mut root.children {
        if let Some(found) = find_node_mut(child, id) {
            return Some(found);
        }
    }
    None
}
