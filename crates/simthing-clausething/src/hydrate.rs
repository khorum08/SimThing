//! CT-1a literal entity hydration into existing `simthing-spec` authoring structs.
//!
//! Maps a safe synthetic ClauseScript entity template (flat properties + literal
//! `modifier` blocks) to [`DomainPackSpec`]. No runtime semantics, no scope resolution.

use simthing_core::{OverlayKind, OverlayLifecycle, OverlaySource, SubFieldRole, TransformOp};
use simthing_spec::spec::domain_pack::DomainPackSpec;
use simthing_spec::spec::install_target::InstallTargetSpec;
use simthing_spec::spec::overlay::OverlaySpec;
use simthing_spec::spec::property::PropertySpec;

use crate::error::HydrateError;
use crate::raw::{RawDocument, RawProperty, RawValue};

/// Hydrated CT-1a entity pack plus install-time seed metadata.
#[derive(Debug, Clone)]
pub struct HydratedEntityPack {
    pub domain_pack: DomainPackSpec,
    /// Literal Amount sub-field seed used by the CT-1a install snapshot proof.
    pub seed_amount: f32,
}

/// Hydrate one top-level entity template from an expanded raw document.
pub fn hydrate_entity_pack(document: &RawDocument) -> Result<HydratedEntityPack, HydrateError> {
    let RawValue::Block(root) = &document.root else {
        return Err(HydrateError::new("document root must be a property block"));
    };
    if root.properties.len() != 1 {
        return Err(HydrateError::new(
            "CT-1a expects exactly one top-level entity template",
        ));
    }
    let entity = &root.properties[0];
    let RawValue::Block(body) = &entity.value else {
        return Err(HydrateError::new_spanned(
            "top-level entity value must be a block",
            Some(entity.key.span.clone()),
        ));
    };

    let pack_id = entity.key.text.clone();
    let mut display_name = pack_id.clone();
    let mut properties = Vec::new();
    let mut overlays = Vec::new();
    let mut seed_amount = None;

    for property in &body.properties {
        match property.key.text.as_str() {
            "display_name" => {
                display_name = read_scalar_text(property, "display_name")?;
            }
            "property" => {
                let (prop, seed) = parse_property_block(property)?;
                properties.push(prop);
                if seed_amount.is_some() {
                    return Err(HydrateError::new_spanned(
                        "duplicate property blocks are not supported in CT-1a",
                        Some(property.key.span.clone()),
                    ));
                }
                seed_amount = Some(seed);
            }
            "modifier" => {
                overlays.push(parse_modifier_block(property)?);
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported CT-1a entity field `{other}`"),
                    Some(property.key.span.clone()),
                ));
            }
        }
    }

    if properties.is_empty() {
        return Err(HydrateError::new(
            "CT-1a entity requires a `property` block",
        ));
    }
    if overlays.is_empty() {
        return Err(HydrateError::new(
            "CT-1a entity requires a `modifier` block",
        ));
    }

    Ok(HydratedEntityPack {
        domain_pack: DomainPackSpec {
            id: pack_id,
            display_name,
            metadata: Default::default(),
            properties,
            overlays,
            capability_trees: Vec::new(),
            events: Vec::new(),
        },
        seed_amount: seed_amount
            .ok_or_else(|| HydrateError::new("property block must include seed_amount"))?,
    })
}

fn parse_property_block(property: &RawProperty) -> Result<(PropertySpec, f32), HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`property` must be a block",
            Some(property.key.span.clone()),
        ));
    };

    let mut id = None;
    let mut namespace = None;
    let mut name = None;
    let mut display_name = String::new();
    let mut description = String::new();
    let mut seed_amount = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => id = Some(read_scalar_text(field, "id")?),
            "namespace" => namespace = Some(read_scalar_text(field, "namespace")?),
            "name" => name = Some(read_scalar_text(field, "name")?),
            "display_name" => display_name = read_scalar_text(field, "display_name")?,
            "description" => description = read_scalar_text(field, "description")?,
            "seed_amount" => {
                seed_amount = Some(read_scalar_f32(field, "seed_amount")?);
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported property field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    Ok((
        PropertySpec {
            id: require_field(id, "id", property)?,
            namespace: require_field(namespace, "namespace", property)?,
            name: require_field(name, "name", property)?,
            display_name,
            description,
            sub_fields: Vec::new(),
        },
        seed_amount.ok_or_else(|| {
            HydrateError::new_spanned(
                "property block requires seed_amount",
                Some(property.key.span.clone()),
            )
        })?,
    ))
}

fn parse_modifier_block(property: &RawProperty) -> Result<OverlaySpec, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`modifier` must be a block",
            Some(property.key.span.clone()),
        ));
    };

    let mut id = None;
    let mut display_name = String::new();
    let mut targets_property = None;
    let mut amount_mult = None;
    let mut amount_add = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => id = Some(read_scalar_text(field, "id")?),
            "display_name" => display_name = read_scalar_text(field, "display_name")?,
            "targets_property" => {
                targets_property = Some(read_scalar_text(field, "targets_property")?);
            }
            "amount_mult" => amount_mult = Some(read_scalar_f32(field, "amount_mult")?),
            "amount_add" => amount_add = Some(read_scalar_f32(field, "amount_add")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported modifier field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    let transform = match (amount_mult, amount_add) {
        (Some(mult), None) => TransformOp::Multiply(mult),
        (None, Some(add)) => TransformOp::Add(add),
        (Some(_), Some(_)) => {
            return Err(HydrateError::new_spanned(
                "modifier cannot specify both amount_mult and amount_add",
                Some(property.key.span.clone()),
            ));
        }
        (None, None) => {
            return Err(HydrateError::new_spanned(
                "modifier requires amount_mult or amount_add",
                Some(property.key.span.clone()),
            ));
        }
    };

    Ok(OverlaySpec {
        id: require_field(id, "id", property)?,
        display_name,
        targets_property: require_field(targets_property, "targets_property", property)?,
        sub_field_deltas: vec![(SubFieldRole::Amount, transform)],
        lifecycle: OverlayLifecycle::Permanent,
        kind: OverlayKind::Policy,
        source: OverlaySource::Player,
        install: InstallTargetSpec::SessionRoot,
    })
}

fn read_scalar_text(property: &RawProperty, field: &str) -> Result<String, HydrateError> {
    match &property.value {
        RawValue::Scalar(scalar) => Ok(scalar.text.clone()),
        _ => Err(HydrateError::new_spanned(
            format!("`{field}` must be a scalar"),
            Some(property.key.span.clone()),
        )),
    }
}

fn read_scalar_f32(property: &RawProperty, field: &str) -> Result<f32, HydrateError> {
    let text = read_scalar_text(property, field)?;
    text.parse::<f32>().map_err(|_| {
        HydrateError::new_spanned(
            format!("`{field}` must be a numeric literal, got `{text}`"),
            Some(property.key.span.clone()),
        )
    })
}

fn require_field<T>(
    value: Option<T>,
    field: &str,
    property: &RawProperty,
) -> Result<T, HydrateError> {
    value.ok_or_else(|| {
        HydrateError::new_spanned(
            format!("missing required field `{field}`"),
            Some(property.key.span.clone()),
        )
    })
}
