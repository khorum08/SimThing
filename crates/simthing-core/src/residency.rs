//! Object-issued dense-row residency requests.
//!
//! A [`SimThing`](crate::SimThing) owns the object relation. The kernel owns
//! slot identity. This module is the typed hand-off between those authorities:
//! the object emits a root/child relation request and `simthing-kernel`
//! executes that request against its slot allocator.
//!
//! Requests deliberately carry no authored or serialized slot number. A slot
//! is ephemeral session state minted only after the kernel accepts the object
//! relation.

use crate::SimThingId;

/// The structural relation that determines how one SimThing becomes resident.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ObjectResidencyRelation {
    /// The root of one admitted runtime tree.
    Root,
    /// A child row emitted by its structural parent object.
    ChildOf(SimThingId),
}

/// Linear request emitted by a SimThing root or by one verified direct-child
/// attachment.
///
/// Fields are private so downstream code cannot assemble a relation from raw
/// ids beside the object-semantic door. The request is intentionally neither
/// `Copy` nor `Clone`: one observed attachment emits one value for immediate
/// allocator execution.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ObjectResidencyRequest {
    object: SimThingId,
    relation: ObjectResidencyRelation,
}

impl ObjectResidencyRequest {
    pub(crate) fn root(object: SimThingId) -> Self {
        Self {
            object,
            relation: ObjectResidencyRelation::Root,
        }
    }

    pub(crate) fn attached_child(object: SimThingId, parent: SimThingId) -> Self {
        Self {
            object,
            relation: ObjectResidencyRelation::ChildOf(parent),
        }
    }

    pub fn object(&self) -> SimThingId {
        self.object
    }

    pub fn relation(&self) -> ObjectResidencyRelation {
        self.relation
    }
}

/// Object-issued request to retire one ephemeral dense row.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ObjectResidencyRelease {
    object: SimThingId,
}

impl ObjectResidencyRelease {
    pub(crate) fn new(object: SimThingId) -> Self {
        Self { object }
    }

    pub fn object(&self) -> SimThingId {
        self.object
    }
}
