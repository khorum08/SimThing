use crate::diagnostics::SpecResult;
use crate::runtime::CompiledEffect;
use crate::spec::EffectSpec;

pub fn compile_effect(spec: &EffectSpec) -> SpecResult<CompiledEffect> {
    let effect = match spec {
        EffectSpec::Remove { target } => CompiledEffect::Remove { target: *target },
        EffectSpec::ActivateOverlay { target, overlay_id } => CompiledEffect::ActivateOverlay {
            target: *target,
            overlay_id: *overlay_id,
        },
        EffectSpec::SuspendOverlay { target, overlay_id } => CompiledEffect::SuspendOverlay {
            target: *target,
            overlay_id: *overlay_id,
        },
    };
    Ok((effect, Default::default()))
}
