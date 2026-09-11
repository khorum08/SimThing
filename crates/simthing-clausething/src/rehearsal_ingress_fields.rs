//! Native declarations of existing property cells, resource-parent edges and owner bindings.
//! Values lower directly into the existing SimThing and PropertySpec surfaces. Historical
//! hydrate-only documents that do not use these declarations keep their stage contract.

use super::*;
use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, DimensionRegistry, LogTier,
    SubFieldSpec,
};

pub(super) fn parse_role(name: &str) -> SubFieldRole {
    match name {
        "Amount" => SubFieldRole::Amount,
        "Velocity" => SubFieldRole::Velocity,
        other => SubFieldRole::Named(other.into()),
    }
}

pub(super) fn parse_subfield(property: &RawProperty) -> Result<SubFieldSpec, HydrateError> {
    let block = require_block(property, "sub_field")?;
    let mut role = None;
    let mut default = 0.0;
    let mut governed_by = None;
    let mut accumulator = None;
    let mut fields = BTreeSet::new();
    for field in &block.properties {
        if !fields.insert(&field.key.text) {
            return Err(failure(field, "duplicate sub_field declaration"));
        }
        match field.key.text.as_str() {
            "role" => role = Some(parse_role(&read_scalar_text(field, "role")?)),
            "default" => default = finite(field)?,
            "governed_by" => {
                governed_by = Some(parse_role(&read_scalar_text(field, "governed_by")?))
            }
            "accumulator" => {
                let role = match &field.value {
                    RawValue::Scalar(value) => match value.text.as_str() {
                        "IntrinsicFlow" => AccumulatorRole::IntrinsicFlow,
                        "Balance" => AccumulatorRole::Balance(BalanceSpec::default()),
                        other => {
                            return Err(failure(
                                field,
                                format!("unsupported accumulator `{other}`"),
                            ))
                        }
                    },
                    RawValue::Header(header) => {
                        let RawValue::Block(body) = header.payload.as_ref() else {
                            return Err(failure(field, "accumulator requires an arena block"));
                        };
                        if body.properties.len() != 1 || body.properties[0].key.text != "arena" {
                            return Err(failure(field, "accumulator requires exactly one arena"));
                        }
                        let arena = read_scalar_text(&body.properties[0], "arena")?;
                        match header.header.text.as_str() {
                            "AllocatedFlow" => AccumulatorRole::AllocatedFlow { arena },
                            "AllocatorWeight" => AccumulatorRole::AllocatorWeight { arena },
                            other => {
                                return Err(failure(
                                    field,
                                    format!("unsupported accumulator `{other}`"),
                                ))
                            }
                        }
                    }
                    _ => return Err(failure(field, "invalid accumulator declaration")),
                };
                accumulator = Some(AccumulatorSpec {
                    role,
                    log_tier: LogTier::Summary,
                });
            }
            other => {
                return Err(failure(
                    field,
                    format!("unsupported sub_field field `{other}`"),
                ))
            }
        }
    }
    let role = role.ok_or_else(|| failure(property, "sub_field requires role"))?;
    Ok(SubFieldSpec {
        display_name: format!("{role:?}"),
        role,
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default,
        display_range: None,
        governed_by,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: accumulator,
    })
}

fn failure(property: &RawProperty, message: impl Into<String>) -> HydrateError {
    HydrateError::new_spanned(message, Some(property.key.span.clone()))
}

fn finite(property: &RawProperty) -> Result<f32, HydrateError> {
    let value = read_scalar_f32(property, &property.key.text)?;
    if !value.is_finite() {
        return Err(failure(property, "property value must be finite"));
    }
    Ok(value)
}

fn node_mut(node: &mut SimThing, id: SimThingId) -> Option<&mut SimThing> {
    if node.id == id {
        return Some(node);
    }
    node.children
        .iter_mut()
        .find_map(|child| node_mut(child, id))
}

fn source_nodes<'a>(
    source: &'a RawProperty,
    out: &mut Vec<&'a RawProperty>,
) -> Result<(), HydrateError> {
    out.push(source);
    let (_, body) = header_or_block_body(source, &source.key.text)?;
    for field in &body.properties {
        if matches!(field.key.text.as_str(), "location" | "owner" | "child") {
            source_nodes(field, out)?;
        } else if field.key.text == "children" {
            for child in &require_block(field, "children")?.properties {
                source_nodes(child, out)?;
            }
        }
    }
    Ok(())
}

pub(super) fn apply_source_fields(
    source: &RawProperty,
    pack: &mut HydratedScenarioPack,
) -> Result<(), HydrateError> {
    let mut nodes = Vec::new();
    source_nodes(source, &mut nodes)?;
    let uses_fields = nodes.iter().any(|node| {
        header_or_block_body(node, &node.key.text).is_ok_and(|(_, body)| {
            body.properties.iter().any(|field| {
                matches!(
                    field.key.text.as_str(),
                    "property_value" | "resource_parent" | "owner_ref"
                ) || (node.key.text == "owner" && field.key.text == "overlays")
            })
        })
    });
    if !uses_fields {
        return Ok(());
    }
    let mut registry = DimensionRegistry::new();
    for property in &pack.game_mode.properties {
        simthing_spec::compile_property(property, &mut registry).map_err(|error| {
            failure(
                source,
                format!(
                    "property {}::{}: {error}",
                    property.namespace, property.name
                ),
            )
        })?;
    }
    let mut targets = pack.install_targets.clone();
    if let Some(authority) = &pack.authority_root {
        let session = authority
            .children
            .iter()
            .find(|node| node.kind == SimThingKind::GameSession)
            .ok_or_else(|| failure(source, "canonical source has no GameSession"))?;
        targets.insert(pack.scenario_id.clone(), vec![session.id]);
        for owner in &pack.owners {
            let node = session
                .children
                .iter()
                .find(|node| {
                    simthing_spec::owner_entity_id(node).as_deref() == Some(&owner.owner_key)
                })
                .ok_or_else(|| {
                    failure(
                        source,
                        "authored owner identity missing from canonical tree",
                    )
                })?;
            for alias in [&owner.id, &owner.owner_key] {
                if targets
                    .get(alias)
                    .is_some_and(|ids| ids.as_slice() != [node.id])
                {
                    return Err(failure(
                        source,
                        format!("ambiguous authored target `{alias}`"),
                    ));
                }
                targets.insert(alias.clone(), vec![node.id]);
            }
        }
    }
    let resolve = |target: &str, field: &RawProperty| -> Result<SimThingId, HydrateError> {
        match targets.get(target).map(Vec::as_slice) {
            Some([id]) => Ok(*id),
            _ => Err(failure(
                field,
                format!("target `{target}` must resolve to exactly one authored node"),
            )),
        }
    };
    let owners: BTreeSet<_> = pack
        .owners
        .iter()
        .map(|owner| owner.owner_key.clone())
        .collect();
    let mut overlay_ids = pack
        .game_mode
        .overlays
        .iter()
        .map(|overlay| overlay.id.clone())
        .collect();
    for source_node in nodes {
        let (header, body) = header_or_block_body(source_node, &source_node.key.text)?;
        let target = body
            .properties
            .iter()
            .find(|field| field.key.text == "id")
            .map(|field| read_scalar_text(field, "id"))
            .transpose()?
            .unwrap_or(header);
        let id = resolve(&target, source_node)?;
        let mut property_cells = BTreeSet::new();
        let mut parent_keys = BTreeSet::new();
        let mut owner_bound = false;
        for field in &body.properties {
            if !matches!(
                field.key.text.as_str(),
                "property_value" | "resource_parent" | "owner_ref" | "overlays"
            ) {
                continue;
            }
            if field.key.text == "overlays" && source_node.key.text != "owner" {
                continue;
            }
            let node = pack
                .authority_root
                .as_mut()
                .and_then(|root| node_mut(root, id))
                .or_else(|| node_mut(&mut pack.root, id))
                .ok_or_else(|| {
                    failure(
                        field,
                        format!("target `{target}` is absent from the hydrated tree"),
                    )
                })?;
            match field.key.text.as_str() {
                "owner_ref" => {
                    let owner = read_scalar_text(field, "owner_ref")?;
                    if owner_bound
                        || (!owners.contains(&owner)
                            && owner != simthing_core::owner_channel::UNOWNED_OWNER_REF)
                    {
                        return Err(failure(
                            field,
                            format!("duplicate or unknown owner_ref `{owner}`"),
                        ));
                    }
                    simthing_core::owner_channel::bind_owner(
                        node,
                        &simthing_core::owner_channel::OwnerRef::new(owner),
                    );
                    owner_bound = true;
                }
                "overlays" => {
                    let overlays = parse_overlays_block(field, &target, &mut overlay_ids)?;
                    if overlays.iter().any(|overlay| {
                        overlay.sub_field_deltas.iter().any(|(role, _)| {
                            overlay
                                .targets_property
                                .split_once("::")
                                .and_then(|(ns, name)| registry.id_of(ns, name))
                                .is_some_and(|pid| {
                                    registry.property(pid).layout.sub_fields.iter().any(|sub| {
                                        &sub.role == role
                                            && sub.accumulator_spec.as_ref().is_some_and(|acc| {
                                                matches!(
                                                    acc.role,
                                                    AccumulatorRole::AllocatorWeight { .. }
                                                )
                                            })
                                    })
                                })
                        })
                    }) {
                        simthing_spec::apply_owner_policy_weight_authority(node);
                    }
                    pack.game_mode.overlays.extend(overlays);
                }
                "property_value" | "resource_parent" => {
                    let block = require_block(field, &field.key.text)?;
                    let property_fields: Vec<_> = block
                        .properties
                        .iter()
                        .filter(|f| f.key.text == "property")
                        .collect();
                    if property_fields.len() != 1 {
                        return Err(failure(field, "requires exactly one property key"));
                    }
                    let key = read_scalar_text(property_fields[0], "property")?;
                    let (namespace, name) = key
                        .split_once("::")
                        .ok_or_else(|| failure(field, "property must be namespace::name"))?;
                    let pid = registry
                        .id_of(namespace, name)
                        .ok_or_else(|| failure(field, format!("unknown property `{key}`")))?;
                    if field.key.text == "resource_parent" {
                        if !parent_keys.insert(key.clone()) {
                            return Err(failure(field, "duplicate resource parent"));
                        }
                        if block.properties.len() != 2 {
                            return Err(failure(
                                field,
                                "resource_parent requires property and parent only",
                            ));
                        }
                        let parent = block
                            .properties
                            .iter()
                            .find(|f| f.key.text == "parent")
                            .ok_or_else(|| failure(field, "resource_parent requires parent"))?;
                        node.add_resource_parent_edge(
                            namespace,
                            name,
                            resolve(&read_scalar_text(parent, "parent")?, parent)?,
                            None,
                        );
                    } else {
                        if !property_cells.insert(pid) {
                            return Err(failure(field, "duplicate property_value"));
                        }
                        let property = registry.property(pid);
                        let mut value = property.default_value();
                        let mut roles = BTreeSet::new();
                        for cell in block.properties.iter().filter(|f| f.key.text != "property") {
                            let role = parse_role(&cell.key.text);
                            if !roles.insert(cell.key.text.clone())
                                || !property
                                    .layout
                                    .sub_fields
                                    .iter()
                                    .any(|sub| sub.role == role && sub.width == 1)
                            {
                                return Err(failure(
                                    cell,
                                    format!(
                                        "duplicate or unknown scalar subfield `{}` of `{key}`",
                                        cell.key.text
                                    ),
                                ));
                            }
                            value.set_role(&role, &property.layout, finite(cell)?);
                        }
                        node.add_property(pid, value);
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    pack.install_targets = targets;
    Ok(())
}
