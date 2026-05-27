use crate::diagnostics::{SpecDiagnostics, SpecResult};
use crate::error::SpecError;
use crate::spec::property::PropertySpec;
use simthing_core::{
    expand_arena_internal_columns, DimensionRegistry, PropertyLayout, SimProperty,
    SimPropertyId, SubFieldRole,
};

/// Compile a `PropertySpec` into a live `SimProperty` and register it with the
/// supplied `DimensionRegistry`.
///
/// Validation:
/// 1. `registry.id_of(namespace, name)` must be `None` — duplicate registrations
///    are a hard error (avoids the `DimensionRegistry::register` panic).
/// 2. For each sub-field with `governed_by: Some(role)`, that role must exist
///    in the same layout — otherwise the integration step would silently no-op.
///
/// `ClampBehavior` and `ReductionRule` validity are enforced structurally by
/// `simthing_core` (closed enums, range checks in `apply`); no extra spec-layer
/// validation is needed there.
///
/// Layout selection:
/// - Empty `sub_fields` → `PropertyLayout::standard(0)` (Amount + Velocity + Intensity).
/// - Non-empty `sub_fields` → those specs become the layout verbatim.
pub fn compile_property(
    spec: &PropertySpec,
    registry: &mut DimensionRegistry,
) -> SpecResult<SimPropertyId> {
    if registry.id_of(&spec.namespace, &spec.name).is_some() {
        return Err(SpecError::DuplicateProperty {
            namespace: spec.namespace.clone(),
            name: spec.name.clone(),
        });
    }

    let layout = if spec.sub_fields.is_empty() {
        PropertyLayout::standard(0)
    } else {
        PropertyLayout {
            sub_fields: spec.sub_fields.clone(),
        }
    };
    let layout = expand_arena_internal_columns(layout);

    validate_governed_by(spec, &layout)?;

    let prop = SimProperty {
        namespace: spec.namespace.clone(),
        name: spec.name.clone(),
        layout,
        decay: None,
        intensity_behavior: None,
        fission_templates: vec![],
        fusion_templates: vec![],
        on_expire: None,
        description: spec.description.clone(),
        intensity_labels: vec![],
    };

    let id = registry.register(prop);
    Ok((id, SpecDiagnostics::default()))
}

fn validate_governed_by(spec: &PropertySpec, layout: &PropertyLayout) -> Result<(), SpecError> {
    for sf in &layout.sub_fields {
        if let Some(gov_role) = &sf.governed_by {
            if layout.offset_of(gov_role).is_none() {
                return Err(SpecError::InvalidGovernedByRole {
                    property: format!("{}::{}", spec.namespace, spec.name),
                    sub_field: format_role(&sf.role),
                    governed_by: format_role(gov_role),
                });
            }
        }
    }
    Ok(())
}

fn format_role(role: &SubFieldRole) -> String {
    match role {
        SubFieldRole::Amount => "Amount".into(),
        SubFieldRole::Velocity => "Velocity".into(),
        SubFieldRole::Intensity => "Intensity".into(),
        SubFieldRole::Named(n) => format!("Named({n})"),
        SubFieldRole::Custom(n) => format!("Custom({n})"),
    }
}
