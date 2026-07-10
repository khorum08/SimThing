use crate::diagnostics::SpecResult;
use crate::error::SpecError;
use crate::runtime::{CompiledThresholdTrigger, CompiledTrigger};
use crate::spec::TriggerSpec;
use simthing_core::DimensionRegistry;

pub fn compile_trigger(
    spec: &TriggerSpec,
    registry: &DimensionRegistry,
) -> SpecResult<CompiledTrigger> {
    let diagnostics = Default::default();
    let trigger = match spec {
        TriggerSpec::Threshold {
            target,
            property,
            role,
            threshold,
            direction,
        } => {
            let property_id = registry
                .id_of(&property.namespace, &property.name)
                .ok_or_else(|| SpecError::InvalidTriggerProperty {
                    namespace: property.namespace.clone(),
                    name: property.name.clone(),
                })?;
            let layout = &registry.property(property_id).layout;
            let range = registry.column_range(property_id);
            let col = range
                .col_for_role(role, layout)
                .ok_or_else(|| SpecError::InvalidTriggerRole {
                    property: format!("{}::{}", property.namespace, property.name),
                    role: format!("{role:?}"),
                })?
                .raw();
            CompiledTrigger::Threshold(CompiledThresholdTrigger {
                target: *target,
                property: property_id,
                role: role.clone(),
                col,
                threshold: *threshold,
                direction: direction.clone(),
            })
        }
        TriggerSpec::Predicate { predicate } => CompiledTrigger::Predicate(predicate.clone()),
    };

    Ok((trigger, diagnostics))
}
