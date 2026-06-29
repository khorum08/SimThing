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

use simthing_core::{SimThing, SimThingId};

/// Runtime tree root admitted once at the CPU boundary — semantic kind is not
/// readable through this type's public surface.
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

    /// CPU-boundary tree access for driver/spec layers (AS-4 residue — 0B).
    pub fn access<R>(&self, f: impl FnOnce(&SimThing) -> R) -> R {
        f(&self.inner)
    }

    /// CPU-boundary tree mutation for driver/spec layers (AS-4 residue — 0B).
    pub fn access_mut<R>(&mut self, f: impl FnOnce(&mut SimThing) -> R) -> R {
        f(&mut self.inner)
    }

    pub(crate) fn inner(&self) -> &SimThing {
        &self.inner
    }

    pub(crate) fn inner_mut(&mut self) -> &mut SimThing {
        &mut self.inner
    }
}

impl Clone for SimRuntimeTree {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
