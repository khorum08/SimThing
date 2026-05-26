use crate::diagnostics::{SpecDiagnostics, SpecResult};
use crate::error::SpecError;
use crate::spec::overlay::OverlaySpec;
use simthing_core::{DimensionRegistry, Overlay, OverlayId, PropertyTransformDelta, SubFieldRole};

/// Compile an `OverlaySpec` into a live `Overlay` instance.
///
/// Validation:
/// 1. `spec.targets_property` must be `"namespace::name"` — malformed → `InvalidPropertyReference`.
/// 2. The referenced property must exist in the registry → `UnknownProperty` otherwise.
/// 3. Every `(SubFieldRole, TransformOp)` pair in `sub_field_deltas` must have its
///    role present in the target property's layout → `InvalidSubFieldRole` otherwise.
///    This is the "resolves sub-field roles to columns" guarantee — at runtime
///    `PropertyTransformDelta::apply_to_data` silently skips unknown roles, which
///    would hide authoring bugs. We catch them at compile time instead.
///
/// `affects` is left empty — overlays are attached to specific SimThings at
/// runtime by the caller (e.g. the capability builder or session coordinator).
pub fn compile_overlay(spec: &OverlaySpec, registry: &DimensionRegistry) -> SpecResult<Overlay> {
    let (ns, name) = parse_property_ref(&spec.id, &spec.targets_property)?;

    let property_id = registry
        .id_of(ns, name)
        .ok_or_else(|| SpecError::UnknownProperty {
            overlay: spec.id.clone(),
            namespace: ns.to_owned(),
            name: name.to_owned(),
        })?;

    let layout = &registry.property(property_id).layout;
    for (role, _op) in &spec.sub_field_deltas {
        if layout.offset_of(role).is_none() {
            return Err(SpecError::InvalidSubFieldRole {
                overlay: spec.id.clone(),
                property: format!("{ns}::{name}"),
                role: format_role(role),
            });
        }
    }

    let overlay = Overlay {
        id: OverlayId::new(),
        kind: spec.kind.clone(),
        source: spec.source.clone(),
        affects: vec![],
        transform: PropertyTransformDelta {
            property_id,
            sub_field_deltas: spec.sub_field_deltas.clone(),
        },
        lifecycle: spec.lifecycle.clone(),
    };

    Ok((overlay, SpecDiagnostics::default()))
}

fn parse_property_ref<'a>(
    overlay_id: &str,
    refstr: &'a str,
) -> Result<(&'a str, &'a str), SpecError> {
    let mut parts = refstr.splitn(2, "::");
    let ns = parts.next().unwrap_or("");
    let name = parts.next();
    match name {
        Some(name) if !ns.is_empty() && !name.is_empty() => Ok((ns, name)),
        _ => Err(SpecError::InvalidPropertyReference {
            overlay: overlay_id.to_owned(),
            targets_property: refstr.to_owned(),
        }),
    }
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
