//! Fission clone-source runtime view — no `kind` accessor.
//!
//! Direct kind access is uncompilable (`fission_clone_source_view_hides_kind_compile_fail`):
//!
//! ```compile_fail
//! use simthing_sim::FissionCloneSourceView;
//!
//! fn peek_kind(v: FissionCloneSourceView<'_>) {
//!     let _ = v.kind;
//! }
//! ```
//!
//! Recovering `SimThing` for kind reads is also uncompilable
//! (`fission_clone_source_view_inner_kind_backdoor_compile_fail`):
//!
//! ```compile_fail
//! use simthing_sim::FissionCloneSourceView;
//!
//! fn peek_inner_kind(v: FissionCloneSourceView<'_>) {
//!     let _ = v.inner().kind;
//! }
//! ```

use simthing_core::{is_fission_clone_source, SimThing, SimThingId};

/// SimThing subtree node for fission clone-source selection — exposes id/children only.
pub struct FissionCloneSourceView<'a> {
    node: &'a SimThing,
}

impl<'a> FissionCloneSourceView<'a> {
    pub(crate) fn from_node(node: &'a SimThing) -> Self {
        Self { node }
    }

    pub fn id(&self) -> SimThingId {
        self.node.id
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
