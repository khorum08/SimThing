use crate::diagnostics::{SpecDiagnostics, SpecResult};
use crate::error::SpecError;
use crate::spec::overlay::OverlaySpec;
use simthing_core::{
    DimensionRegistry, Overlay, OverlayId, PropertyTransformDelta, SimThingId, SubFieldRole,
};
use std::collections::{BTreeMap, BTreeSet};

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
/// `origin` is supplied by that caller because it owns the authority-tree
/// context; authored overlays use the ScenarioThing id.
pub fn compile_overlay(
    spec: &OverlaySpec,
    registry: &DimensionRegistry,
    origin: SimThingId,
) -> SpecResult<Overlay> {
    validate_evaluation_admission(spec)?;
    simthing_core::admit_overlay_lifecycle(&spec.lifecycle).map_err(|error| {
        SpecError::OverlayLifecycleAdmission {
            overlay: spec.id.clone(),
            reason: error.to_string(),
        }
    })?;
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
        origin,
        affects: vec![],
        transform: PropertyTransformDelta {
            property_id,
            sub_field_deltas: spec.sub_field_deltas.clone(),
        },
        lifecycle: spec.lifecycle.clone(),
    };

    Ok((overlay, SpecDiagnostics::default()))
}

const MAX_OVERLAY_DEPENDENCY_EDGES: usize = 256;

fn evaluation_error(spec: &OverlaySpec, reason: impl Into<String>) -> SpecError {
    SpecError::OverlayEvaluationAdmission {
        overlay: spec.id.clone(),
        reason: reason.into(),
        source_span_token: spec.source_span_token,
    }
}

fn validate_evaluation_admission(spec: &OverlaySpec) -> Result<(), SpecError> {
    match spec.composition_class.as_deref().unwrap_or("sequential") {
        "sequential" => {}
        "conjunctive-restriction" => {
            for (role, op) in &spec.sub_field_deltas {
                let Some(factor) = op.as_multiply_literal() else {
                    return Err(evaluation_error(
                        spec,
                        format!(
                            "conjunctive restriction for {} must lower to Multiply; Set/Add could weaken an ancestor restriction",
                            format_role(role)
                        ),
                    ));
                };
                if !factor.is_finite() || !(0.0..=1.0).contains(&factor) {
                    return Err(evaluation_error(
                        spec,
                        format!(
                            "conjunctive restriction factor {factor:?} for {} must be finite and within [0, 1]",
                            format_role(role)
                        ),
                    ));
                }
            }
        }
        other => {
            return Err(evaluation_error(
                spec,
                format!("unknown overlay composition class `{other}`"),
            ));
        }
    }

    if spec.current_dependency_edges.len() > MAX_OVERLAY_DEPENDENCY_EDGES
        || spec.next_dependency_edges.len() > MAX_OVERLAY_DEPENDENCY_EDGES
    {
        return Err(evaluation_error(
            spec,
            format!(
                "dependency edge budget exceeded (current={}, next={}, max_each={MAX_OVERLAY_DEPENDENCY_EDGES})",
                spec.current_dependency_edges.len(),
                spec.next_dependency_edges.len()
            ),
        ));
    }
    for (from, to) in spec
        .current_dependency_edges
        .iter()
        .chain(&spec.next_dependency_edges)
    {
        if from.trim().is_empty() || to.trim().is_empty() {
            return Err(evaluation_error(
                spec,
                "dependency endpoints must be non-empty statically admitted names",
            ));
        }
    }

    // Only pure Current -> Current edges participate. A cycle that includes
    // any explicit Next edge is time evolution and intentionally absent here.
    let mut outgoing: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (from, to) in &spec.current_dependency_edges {
        outgoing.entry(from.as_str()).or_default().push(to.as_str());
    }
    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();
    let mut stack = Vec::new();
    for node in outgoing.keys().copied().collect::<Vec<_>>() {
        if let Some(cycle) = find_current_cycle(
            node,
            &outgoing,
            &mut visiting,
            &mut visited,
            &mut stack,
        ) {
            return Err(evaluation_error(
                spec,
                format!(
                    "pure Current -> Current algebraic cycle `{}`; declare a Current -> Next edge to pace feedback",
                    cycle.join(" -> ")
                ),
            ));
        }
    }
    Ok(())
}

fn find_current_cycle<'a>(
    node: &'a str,
    outgoing: &BTreeMap<&'a str, Vec<&'a str>>,
    visiting: &mut BTreeSet<&'a str>,
    visited: &mut BTreeSet<&'a str>,
    stack: &mut Vec<&'a str>,
) -> Option<Vec<&'a str>> {
    if visited.contains(node) {
        return None;
    }
    if let Some(start) = stack.iter().position(|candidate| *candidate == node) {
        let mut cycle = stack[start..].to_vec();
        cycle.push(node);
        return Some(cycle);
    }
    visiting.insert(node);
    stack.push(node);
    for next in outgoing.get(node).into_iter().flatten().copied() {
        if visiting.contains(next) {
            let start = stack
                .iter()
                .position(|candidate| *candidate == next)
                .unwrap_or(0);
            let mut cycle = stack[start..].to_vec();
            cycle.push(next);
            return Some(cycle);
        }
        if let Some(cycle) = find_current_cycle(next, outgoing, visiting, visited, stack) {
            return Some(cycle);
        }
    }
    stack.pop();
    visiting.remove(node);
    visited.insert(node);
    None
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
