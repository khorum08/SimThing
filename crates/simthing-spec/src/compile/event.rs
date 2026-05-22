use crate::compile::{compile_effect, compile_trigger};
use crate::diagnostics::{SpecDiagnostics, SpecResult};
use crate::runtime::ScriptedEventDefinition;
use crate::spec::{EventKey, EventSpec};
use simthing_core::DimensionRegistry;

pub fn compile_event(
    spec:     &EventSpec,
    registry: &DimensionRegistry,
) -> SpecResult<ScriptedEventDefinition> {
    let (trigger, trigger_diag) = compile_trigger(&spec.trigger, registry)?;
    let mut diagnostics = SpecDiagnostics::default();
    diagnostics.merge(trigger_diag);

    let mut effects = Vec::with_capacity(spec.effects.len());
    for effect in &spec.effects {
        let (compiled, effect_diag) = compile_effect(effect)?;
        diagnostics.merge(effect_diag);
        effects.push(compiled);
    }

    Ok((
        ScriptedEventDefinition {
            id: EventKey::new(&spec.id),
            trigger,
            effects,
            cooldown: spec.cooldown,
            priority: spec.priority,
        },
        diagnostics,
    ))
}
