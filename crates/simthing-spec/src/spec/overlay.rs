use serde::{Deserialize, Serialize};
use simthing_core::{OverlayKind, OverlayLifecycle, OverlaySource, SubFieldRole, TransformOp};

use super::install_target::InstallTargetSpec;

fn default_overlay_install() -> InstallTargetSpec {
    InstallTargetSpec::SessionRoot
}

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
    /// Where this standalone overlay attaches at session install. Defaults to
    /// `SessionRoot` so existing RON omits the field.
    #[serde(default = "default_overlay_install")]
    pub install: InstallTargetSpec,
}
