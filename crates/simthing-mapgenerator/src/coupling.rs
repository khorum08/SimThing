//! Producer-side hyperlane/coupling edge classification (render/report only — not grammar).

use crate::topology::HyperlaneEdge;

/// Classification of emitted `add_hyperlane` feedstock for preview/report surfaces only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CouplingEdgeKind {
    BaseHyperlane,
    SpecialRouteCoupling,
    PartitionBridgeCoupling,
    ClusterBridgeCoupling,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassifiedCouplingEdge {
    pub edge: HyperlaneEdge,
    pub kind: CouplingEdgeKind,
}

impl ClassifiedCouplingEdge {
    pub fn new(edge: HyperlaneEdge, kind: CouplingEdgeKind) -> Self {
        Self { edge, kind }
    }
}
