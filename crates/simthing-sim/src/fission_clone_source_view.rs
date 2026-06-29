//! Fission clone-source runtime view — no `kind` accessor.
//!
//! Transposition between clone-source selection and semantic kind reads is
//! uncompilable:
//!
//! ```compile_fail
//! use simthing_sim::FissionCloneSourceView;
//!
//! fn peek_kind(v: FissionCloneSourceView<'_>) {
//!     let _ = v.kind;
//! }
//! ```

use simthing_core::{is_fission_clone_source, SimThing, SimThingId};

/// SimThing subtree node for fission clone-source selection — exposes id/children only.
pub struct FissionCloneSourceView<'a> {
    node: &'a SimThing,
}

impl<'a> FissionCloneSourceView<'a> {
    pub fn from_node(node: &'a SimThing) -> Self {
        Self { node }
    }

    pub fn id(&self) -> SimThingId {
        self.node.id
    }

    pub fn inner(&self) -> &'a SimThing {
        self.node
    }

    pub fn children(&self) -> impl Iterator<Item = FissionCloneSourceView<'a>> + 'a {
        self.node
            .children
            .iter()
            .map(FissionCloneSourceView::from_node)
    }
}

pub(crate) fn fission_clone_source_children<'a>(
    parent: &'a SimThing,
    container_kinds: &'a [String],
) -> impl Iterator<Item = &'a SimThing> + 'a {
    parent
        .children
        .iter()
        .filter(|child| is_fission_clone_source(child, container_kinds))
}
