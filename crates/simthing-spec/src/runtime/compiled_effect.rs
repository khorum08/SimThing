use crate::spec::script::ScopeRef;
use simthing_core::OverlayId;

#[derive(Clone, Debug, PartialEq)]
pub enum CompiledEffect {
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
