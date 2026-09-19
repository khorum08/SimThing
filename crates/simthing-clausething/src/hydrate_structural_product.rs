//! Native `structural_product` authoring (DA, relay 5737649159).
//!
//! Lowers a funded, DETACHED structural template onto
//! `GameModeSpec::structural_products`. Syntax, the duplicate-field law and
//! owner references resolve here with spans; registry and tree references
//! (funding property/role/host, parent, template properties) resolve at
//! session build, before activation, where the typed refusal carries this
//! declaration's token. Nothing here is instantiated into the N0 tree.

use super::*;
use crate::hydrate_field_economy::set_once;
use simthing_spec::spec::script::PropertyKey;
use simthing_spec::{
    StructuralFundingSpec, StructuralProductSpec, StructuralTemplateNodeSpec,
    StructuralTemplatePropertySpec,
};
use std::num::NonZeroU32;

fn refuse(field: &RawProperty, message: impl Into<String>) -> HydrateError {
    HydrateError::new_spanned(message, Some(field.key.span.clone()))
}

pub(super) fn parse_structural_products(
    fields: &[RawProperty],
    owners: &[HydratedScenarioOwner],
    seen_overlay_ids: &mut BTreeSet<String>,
) -> Result<Vec<StructuralProductSpec>, HydrateError> {
    let owner_keys: BTreeSet<&str> = owners.iter().map(|owner| owner.owner_key.as_str()).collect();
    let owner_names: BTreeSet<&str> = owners
        .iter()
        .flat_map(|owner| [owner.id.as_str(), owner.owner_key.as_str()])
        .collect();
    let mut seen = BTreeSet::new();
    let mut products = Vec::with_capacity(fields.len());
    for field in fields {
        let product = parse_product(field, &owner_keys, &owner_names, seen_overlay_ids)?;
        if !seen.insert(product.id.clone()) {
            return Err(refuse(
                field,
                format!("duplicate structural_product id `{}`", product.id),
            ));
        }
        products.push(product);
    }
    Ok(products)
}

fn parse_product(
    field: &RawProperty,
    owner_keys: &BTreeSet<&str>,
    owner_names: &BTreeSet<&str>,
    seen_overlay_ids: &mut BTreeSet<String>,
) -> Result<StructuralProductSpec, HydrateError> {
    let (id, block) = header_or_block_body(field, "structural_product")?;
    if id.is_empty() {
        return Err(refuse(field, "`structural_product` requires an id"));
    }
    let mut funding = None;
    let mut count = None;
    let mut parent = None;
    let mut template = None;
    for entry in &block.properties {
        match entry.key.text.as_str() {
            "funding" => set_once(&mut funding, parse_funding(entry)?, entry, "structural_product")?,
            "count" => {
                let units = read_scalar_u32(entry, "count")?;
                let units = NonZeroU32::new(units).ok_or_else(|| {
                    refuse(entry, "structural_product.count must be a positive integer")
                })?;
                set_once(&mut count, units, entry, "structural_product")?
            }
            "parent" => set_once(
                &mut parent,
                read_scalar_text(entry, "parent")?,
                entry,
                "structural_product",
            )?,
            "template" => set_once(
                &mut template,
                parse_template(entry, &id, owner_keys, seen_overlay_ids)?,
                entry,
                "structural_product",
            )?,
            other => {
                return Err(refuse(
                    entry,
                    format!("unsupported structural_product field `{other}`"),
                ))
            }
        }
    }
    let required = |what: &str| refuse(field, format!("structural_product `{id}` requires `{what}`"));
    let parent = parent.ok_or_else(|| required("parent"))?;
    if owner_names.contains(parent.as_str()) {
        return Err(refuse(
            field,
            format!(
                "structural_product `{id}` parent `{parent}` is an owner seat; declare ownership \
                 on the template with `owner_ref`"
            ),
        ));
    }
    Ok(StructuralProductSpec {
        funding: funding.ok_or_else(|| required("funding"))?,
        count: count.ok_or_else(|| required("count"))?,
        template: template.ok_or_else(|| required("template"))?,
        parent_entity: parent,
        source_span_token: Some(field.key.span.token_index),
        id,
    })
}

fn parse_funding(field: &RawProperty) -> Result<StructuralFundingSpec, HydrateError> {
    let block = require_block(field, "funding")?;
    let mut entity = None;
    let mut property = None;
    let mut role = None;
    for entry in &block.properties {
        match entry.key.text.as_str() {
            "entity" => set_once(
                &mut entity,
                read_scalar_text(entry, "entity")?,
                entry,
                "structural_product.funding",
            )?,
            "property" => {
                let raw = read_scalar_text(entry, "property")?;
                let key = match raw.split_once("::") {
                    Some((namespace, name)) if !namespace.is_empty() && !name.is_empty() => {
                        PropertyKey::new(namespace, name)
                    }
                    _ => {
                        return Err(refuse(
                            entry,
                            format!("funding property must be `namespace::name`, got `{raw}`"),
                        ))
                    }
                };
                set_once(&mut property, key, entry, "structural_product.funding")?
            }
            "role" => set_once(
                &mut role,
                rehearsal_ingress_fields::parse_role(&read_scalar_text(entry, "role")?),
                entry,
                "structural_product.funding",
            )?,
            other => {
                return Err(refuse(
                    entry,
                    format!("unsupported structural_product.funding field `{other}`"),
                ))
            }
        }
    }
    let required = |what: &str| refuse(field, format!("funding requires `{what}`"));
    Ok(StructuralFundingSpec {
        host_entity: entity.ok_or_else(|| required("entity"))?,
        property: property.ok_or_else(|| required("property"))?,
        role: role.ok_or_else(|| required("role"))?,
    })
}

fn parse_template(
    field: &RawProperty,
    product_id: &str,
    owner_keys: &BTreeSet<&str>,
    seen_overlay_ids: &mut BTreeSet<String>,
) -> Result<StructuralTemplateNodeSpec, HydrateError> {
    let block = match &field.value {
        RawValue::Block(block) => block,
        _ => header_or_block_body(field, &field.key.text)?.1,
    };
    let mut kind = None;
    let mut owner_ref = None;
    let mut property_values: Vec<StructuralTemplatePropertySpec> = Vec::new();
    let mut overlays = Vec::new();
    let mut children = Vec::new();
    let mut overlays_seen = false;
    let mut children_seen = false;
    for entry in &block.properties {
        match entry.key.text.as_str() {
            "kind" => {
                let parsed = parse_kind(entry)?;
                if simthing_spec::is_owner_entity_kind(&parsed) {
                    return Err(refuse(entry, "an owner seat cannot be born as a structural product"));
                }
                set_once(&mut kind, parsed, entry, "structural_product.template")?
            }
            "owner_ref" => {
                let owner = read_scalar_text(entry, "owner_ref")?;
                if owner != simthing_core::owner_channel::UNOWNED_OWNER_REF
                    && !owner_keys.contains(owner.as_str())
                {
                    return Err(refuse(entry, format!("unknown owner_ref `{owner}`")));
                }
                set_once(&mut owner_ref, owner, entry, "structural_product.template")?
            }
            "property_value" => {
                let cell = parse_property_value(entry)?;
                if property_values.iter().any(|seen| seen.property == cell.property) {
                    return Err(refuse(entry, "duplicate property_value on one template node"));
                }
                property_values.push(cell);
            }
            "overlays" => {
                if std::mem::replace(&mut overlays_seen, true) {
                    return Err(refuse(entry, "duplicate structural_product.template field `overlays`"));
                }
                overlays = parse_overlays_block(entry, product_id, seen_overlay_ids)?;
            }
            "children" => {
                if std::mem::replace(&mut children_seen, true) {
                    return Err(refuse(entry, "duplicate structural_product.template field `children`"));
                }
                for child in &require_block(entry, "children")?.properties {
                    if child.key.text != "child" {
                        return Err(refuse(
                            child,
                            format!("unsupported template children entry `{}`", child.key.text),
                        ));
                    }
                    children.push(parse_template(child, product_id, owner_keys, seen_overlay_ids)?);
                }
            }
            other => {
                return Err(refuse(
                    entry,
                    format!("unsupported structural_product.template field `{other}`"),
                ))
            }
        }
    }
    Ok(StructuralTemplateNodeSpec {
        kind: kind.ok_or_else(|| refuse(field, "template node requires `kind`"))?,
        owner_ref,
        property_values,
        overlays,
        children,
    })
}

fn parse_property_value(field: &RawProperty) -> Result<StructuralTemplatePropertySpec, HydrateError> {
    let block = require_block(field, "property_value")?;
    let mut property = None;
    let mut values: Vec<(SubFieldRole, f32)> = Vec::new();
    for entry in &block.properties {
        if entry.key.text == "property" {
            let raw = read_scalar_text(entry, "property")?;
            let (namespace, name) = raw
                .split_once("::")
                .filter(|(namespace, name)| !namespace.is_empty() && !name.is_empty())
                .ok_or_else(|| refuse(entry, format!("property must be `namespace::name`, got `{raw}`")))?;
            set_once(
                &mut property,
                PropertyKey::new(namespace, name),
                entry,
                "structural_product.template.property_value",
            )?;
            continue;
        }
        let role = rehearsal_ingress_fields::parse_role(&entry.key.text);
        if values.iter().any(|(seen, _)| *seen == role) {
            return Err(refuse(entry, format!("duplicate scalar `{}`", entry.key.text)));
        }
        let value = read_scalar_f32(entry, &entry.key.text)?;
        if !value.is_finite() {
            return Err(refuse(entry, "property value must be finite"));
        }
        values.push((role, value));
    }
    Ok(StructuralTemplatePropertySpec {
        property: property.ok_or_else(|| refuse(field, "property_value requires `property`"))?,
        values,
    })
}
