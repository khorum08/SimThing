use serde::{Deserialize, Serialize};
use simthing_core::{OverlayKind, OverlayLifecycle, OverlaySource, SubFieldRole, TransformOp};

/// Authored overlay (standalone, non-capability). Capability effects compile
/// to overlays inline via the PR 3 builder; this spec is for top-level player /
/// AI / system overlays declared in domain packs.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OverlaySpec {
    pub id: String,
    #[serde(default)]
    pub display_name: String,
    /// `"namespace::name"` of the target property. Resolved at compile time.
    pub targets_property: String,
    pub sub_field_deltas: Vec<(SubFieldRole, TransformOp)>,
    pub lifecycle: OverlayLifecycle,
    pub kind: OverlayKind,
    pub source: OverlaySource,
}
