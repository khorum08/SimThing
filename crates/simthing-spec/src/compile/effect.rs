use crate::diagnostics::SpecResult;
use crate::error::SpecError;
use crate::runtime::CompiledEffect;
use crate::spec::EffectSpec;

pub fn compile_effect(spec: &EffectSpec) -> SpecResult<CompiledEffect> {
    let effect = match spec {
        EffectSpec::Remove { target } => CompiledEffect::Remove { target: *target },
        EffectSpec::ActivateOverlay { target, overlay_id } => CompiledEffect::ActivateOverlay {
            target: *target,
            overlay_id: *overlay_id,
        },
        EffectSpec::ActivateOverlayRef { overlay_ref, .. } => {
            return Err(SpecError::UnresolvedOverlayRef {
                overlay_ref: overlay_ref.clone(),
            });
        }
        EffectSpec::SuspendOverlay { target, overlay_id } => CompiledEffect::SuspendOverlay {
            target: *target,
            overlay_id: *overlay_id,
        },
    };
    Ok((effect, Default::default()))
}
