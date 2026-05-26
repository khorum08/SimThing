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
    SuspendOverlay {
        target: ScopeRef,
        overlay_id: OverlayId,
    },
}
