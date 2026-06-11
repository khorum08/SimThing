use crate::spec::script::ScopeRef;
use serde::{Deserialize, Serialize};
use simthing_core::OverlayId;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EffectSpec {
    Remove {
        target: ScopeRef,
    },
    ActivateOverlay {
        target: ScopeRef,
        overlay_id: OverlayId,
    },
    /// Authorable activation referencing a standalone pack overlay by its
    /// authored `OverlaySpec::id`. Resolved to a runtime `OverlayId` by the
    /// driver install path before event compilation; reaching
    /// `compile_effect` unresolved is a hard error. Consumer: CT-1b
    /// `triggered_modifier` lowering (§6 EffectSpec-widening backlog item).
    ActivateOverlayRef {
        target: ScopeRef,
        overlay_ref: String,
    },
    SuspendOverlay {
        target: ScopeRef,
        overlay_id: OverlayId,
    },
}
