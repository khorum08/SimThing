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
    /// Optional order-weight class id (ORDER-WEIGHT-CLASS-0). When set, this
    /// overlay is an operator directive: source must be `Player`, magnitude
    /// must match the named finite class, and non-finite values are rejected.
    #[serde(default)]
    pub order_weight_class: Option<String>,
    /// Admission-only combine class. `conjunctive-restriction` accepts only
    /// finite Multiply factors in `[0, 1]`, so a descendant contribution
    /// cannot weaken an ancestor restriction. The string is consumed here
    /// and never survives as a runtime dispatch key.
    #[serde(default)]
    pub composition_class: Option<String>,
    /// Pure Current -> Current dependencies for this bounded template.
    /// Admission requires this graph to be acyclic.
    #[serde(default)]
    pub current_dependency_edges: Vec<(String, String)>,
    /// Explicit Current -> Next/staged dependencies. These edges are omitted
    /// from the pure-current DAG, making generation-paced feedback lawful.
    #[serde(default)]
    pub next_dependency_edges: Vec<(String, String)>,
    /// Loader-derived source position for admission diagnostics.
    #[serde(skip)]
    pub source_span_token: Option<usize>,
}
