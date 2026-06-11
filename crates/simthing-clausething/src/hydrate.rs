//! CT-1a/CT-1b literal entity hydration into existing `simthing-spec` authoring structs.
//!
//! Maps a safe synthetic ClauseScript entity template (flat properties, literal
//! `modifier` blocks, and same-scope `triggered_modifier` blocks) to
//! [`DomainPackSpec`]. No runtime semantics; scope support is same-scope only
//! (`ScopeRef::Current`) per the accepted SCOPE-MEMO §8.

use simthing_core::{OverlayKind, OverlayLifecycle, OverlaySource, SubFieldRole, TransformOp};
use simthing_spec::spec::capability::{
    CapabilityCategorySpec, CapabilityEffectSpec, CapabilityPrereqSpec, CapabilitySpec,
    CapabilityTreeSpec, EffectTarget,
};
use simthing_spec::spec::domain_pack::DomainPackSpec;
use simthing_spec::spec::effect::EffectSpec;
use simthing_spec::spec::event::EventSpec;
use simthing_spec::spec::install_target::InstallTargetSpec;
use simthing_spec::spec::overlay::OverlaySpec;
use simthing_spec::spec::property::PropertySpec;
use simthing_spec::spec::script::{PropertyKey, ScopeRef};
use simthing_spec::spec::trigger::{TriggerDirection, TriggerSpec};

use crate::error::HydrateError;
use crate::raw::{RawDocument, RawProperty, RawValue};

/// Hydrated entity pack plus install-time seed metadata.
#[derive(Debug, Clone)]
pub struct HydratedEntityPack {
    pub domain_pack: DomainPackSpec,
    /// Literal Amount sub-field seed used by the CT-1a install snapshot proof
    /// (the first authored property's seed).
    pub seed_amount: f32,
    /// All authored `"namespace::name"` → seed pairs, in source order (CT-1b
    /// multi-property corpora).
    pub seeds: Vec<(String, f32)>,
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
    let mut capability_trees = Vec::new();
    let mut events = Vec::new();
    let mut seeds: Vec<(String, f32)> = Vec::new();

    for property in &body.properties {
        match property.key.text.as_str() {
            "display_name" => {
                display_name = read_scalar_text(property, "display_name")?;
            }
            "property" => {
                let (prop, seed) = parse_property_block(property)?;
                seeds.push((format!("{}::{}", prop.namespace, prop.name), seed));
                properties.push(prop);
            }
            "modifier" => {
                overlays.push(parse_modifier_block(property)?);
            }
            "triggered_modifier" => {
                let (overlay, event) = parse_triggered_modifier_block(property)?;
                overlays.push(overlay);
                events.push(event);
            }
            "tradition_tree" => {
                capability_trees.push(parse_tradition_tree_block(property)?);
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported entity field `{other}`"),
                    Some(property.key.span.clone()),
                ));
            }
        }
    }

    if properties.is_empty() {
        return Err(HydrateError::new("entity requires a `property` block"));
    }
    if overlays.is_empty() && capability_trees.is_empty() {
        return Err(HydrateError::new(
            "entity requires a `modifier`, `triggered_modifier`, or `tradition_tree` block",
        ));
    }

    let seed_amount = seeds[0].1;
    Ok(HydratedEntityPack {
        domain_pack: DomainPackSpec {
            id: pack_id,
            display_name,
            metadata: Default::default(),
            properties,
            overlays,
            capability_trees,
            events,
        },
        seed_amount,
        seeds,
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

/// CT-1b: `triggered_modifier { id potential { property at_least } modifier { … } }`
/// → one `Suspended` overlay (activated lifecycle `Permanent`) plus one
/// same-scope threshold event whose effect activates it by authored id.
fn parse_triggered_modifier_block(
    property: &RawProperty,
) -> Result<(OverlaySpec, EventSpec), HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`triggered_modifier` must be a block",
            Some(property.key.span.clone()),
        ));
    };

    let mut id = None;
    let mut potential = None;
    let mut modifier = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => id = Some(read_scalar_text(field, "id")?),
            "potential" => potential = Some(parse_potential_block(field)?),
            "modifier" => modifier = Some(parse_modifier_block(field)?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported triggered_modifier field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    let id = require_field(id, "id", property)?;
    let (potential_property, threshold) = require_field(potential, "potential", property)?;
    let mut overlay = require_field(modifier, "modifier", property)?;
    overlay.lifecycle = OverlayLifecycle::Suspended {
        when_activated: Box::new(OverlayLifecycle::Permanent),
    };

    let event = EventSpec {
        id: id.clone(),
        trigger: TriggerSpec::Threshold {
            target: ScopeRef::Current,
            property: potential_property,
            role: SubFieldRole::Amount,
            threshold,
            direction: TriggerDirection::Rising,
        },
        effects: vec![EffectSpec::ActivateOverlayRef {
            target: ScopeRef::Current,
            overlay_ref: overlay.id.clone(),
        }],
        cooldown: None,
        priority: Default::default(),
        install: InstallTargetSpec::SessionRoot,
    };

    Ok((overlay, event))
}

/// CT-1c: `tradition_tree { id kind owner category { … tradition { … } } }`
/// → [`CapabilityTreeSpec`] on the `capability_tree_v1` pattern. Prereqs come
/// from `possible { has_tradition = X }` (same-category, source order);
/// payload `modifier` blocks become Owner-targeted `Permanent` effects.
fn parse_tradition_tree_block(property: &RawProperty) -> Result<CapabilityTreeSpec, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`tradition_tree` must be a block",
            Some(property.key.span.clone()),
        ));
    };

    let mut tree_id = None;
    let mut tree_kind = None;
    let mut owner_kind = None;
    let mut categories = Vec::new();

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => tree_id = Some(read_scalar_text(field, "id")?),
            "kind" => tree_kind = Some(read_scalar_text(field, "kind")?),
            "owner" => owner_kind = Some(read_scalar_text(field, "owner")?),
            "category" => categories.push(parse_tradition_category_block(field)?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported tradition_tree field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    if categories.is_empty() {
        return Err(HydrateError::new_spanned(
            "tradition_tree requires at least one `category` block",
            Some(property.key.span.clone()),
        ));
    }

    let owner_kind = require_field(owner_kind, "owner", property)?;
    Ok(CapabilityTreeSpec {
        tree_id: require_field(tree_id, "id", property)?,
        tree_kind: require_field(tree_kind, "kind", property)?,
        owner_kind: owner_kind.clone(),
        categories,
        install: InstallTargetSpec::AllOfKind { kind: owner_kind },
    })
}

fn parse_tradition_category_block(
    property: &RawProperty,
) -> Result<CapabilityCategorySpec, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`category` must be a block",
            Some(property.key.span.clone()),
        ));
    };

    let mut namespace = None;
    let mut name = None;
    let mut display_name = String::new();
    let mut entries = Vec::new();

    for field in &block.properties {
        match field.key.text.as_str() {
            "namespace" => namespace = Some(read_scalar_text(field, "namespace")?),
            "name" => name = Some(read_scalar_text(field, "name")?),
            "display_name" => display_name = read_scalar_text(field, "display_name")?,
            "tradition" => entries.push(parse_tradition_entry_block(field)?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported category field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    if entries.is_empty() {
        return Err(HydrateError::new_spanned(
            "category requires at least one `tradition` block",
            Some(property.key.span.clone()),
        ));
    }

    let namespace = require_field(namespace, "namespace", property)?;
    let name = require_field(name, "name", property)?;
    let mut category = CapabilityCategorySpec {
        property_namespace: namespace.clone(),
        property_name: name.clone(),
        display_name,
        tier: 0,
        max_active: None,
        entries,
    };
    // `possible { has_tradition = X }` prereqs are same-category in the v1
    // dialect; stamp the `namespace::name` category ref the parser could not
    // know mid-entry (the builder's `parse_category_ref` format).
    let category_ref = format!("{namespace}::{name}");
    for entry in &mut category.entries {
        for prereq in &mut entry.prereqs {
            prereq.category = category_ref.clone();
        }
    }
    Ok(category)
}

fn parse_tradition_entry_block(property: &RawProperty) -> Result<CapabilitySpec, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`tradition` must be a block",
            Some(property.key.span.clone()),
        ));
    };

    let mut id = None;
    let mut display_name = String::new();
    let mut cost = None;
    let mut prereqs = Vec::new();
    let mut effects = Vec::new();

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => id = Some(read_scalar_text(field, "id")?),
            "display_name" => display_name = read_scalar_text(field, "display_name")?,
            "cost" => cost = Some(read_scalar_f32(field, "cost")?),
            "possible" => parse_possible_block(field, &mut prereqs)?,
            "modifier" => effects.push(parse_tradition_effect_block(field)?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported tradition field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    if effects.is_empty() {
        return Err(HydrateError::new_spanned(
            "tradition requires at least one `modifier` block",
            Some(property.key.span.clone()),
        ));
    }

    Ok(CapabilitySpec {
        id: require_field(id, "id", property)?,
        display_name,
        description: String::new(),
        flavor_text: String::new(),
        research_cost: require_field(cost, "cost", property)?,
        activation: Default::default(),
        icon: String::new(),
        thumbnail: String::new(),
        card_image: String::new(),
        unlock_video: None,
        model_preview: None,
        prereqs,
        unlocks_ship_components: Vec::new(),
        unlocks_buildings: Vec::new(),
        unlocks_units: Vec::new(),
        unlocks_weapons: Vec::new(),
        effects,
    })
}

fn parse_possible_block(
    property: &RawProperty,
    prereqs: &mut Vec<CapabilityPrereqSpec>,
) -> Result<(), HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`possible` must be a block",
            Some(property.key.span.clone()),
        ));
    };

    for field in &block.properties {
        match field.key.text.as_str() {
            "has_tradition" => {
                prereqs.push(CapabilityPrereqSpec {
                    // Same-category in the v1 dialect; the category name is
                    // stamped by `parse_tradition_category_block`.
                    category: String::new(),
                    entry_id: read_scalar_text(field, "has_tradition")?,
                });
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported possible field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }
    Ok(())
}

fn parse_tradition_effect_block(
    property: &RawProperty,
) -> Result<CapabilityEffectSpec, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`modifier` must be a block",
            Some(property.key.span.clone()),
        ));
    };

    let mut targets_property = None;
    let mut amount_mult = None;
    let mut amount_add = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "targets_property" => {
                targets_property = Some(read_scalar_text(field, "targets_property")?);
            }
            "amount_mult" => amount_mult = Some(read_scalar_f32(field, "amount_mult")?),
            "amount_add" => amount_add = Some(read_scalar_f32(field, "amount_add")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported tradition modifier field `{other}`"),
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
                "tradition modifier cannot specify both amount_mult and amount_add",
                Some(property.key.span.clone()),
            ));
        }
        (None, None) => {
            return Err(HydrateError::new_spanned(
                "tradition modifier requires amount_mult or amount_add",
                Some(property.key.span.clone()),
            ));
        }
    };

    Ok(CapabilityEffectSpec {
        targets_property: require_field(targets_property, "targets_property", property)?,
        sub_field_deltas: vec![(SubFieldRole::Amount, transform)],
        when_activated: OverlayLifecycle::Permanent,
        effect_target: EffectTarget::Owner,
    })
}

fn parse_potential_block(property: &RawProperty) -> Result<(PropertyKey, f32), HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`potential` must be a block",
            Some(property.key.span.clone()),
        ));
    };

    let mut property_key = None;
    let mut at_least = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "property" => {
                let text = read_scalar_text(field, "property")?;
                let Some((namespace, name)) = text.split_once("::") else {
                    return Err(HydrateError::new_spanned(
                        format!("`property` must be `namespace::name`, got `{text}`"),
                        Some(field.key.span.clone()),
                    ));
                };
                property_key = Some(PropertyKey::new(namespace, name));
            }
            "at_least" => at_least = Some(read_scalar_f32(field, "at_least")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported potential field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    Ok((
        require_field(property_key, "property", property)?,
        require_field(at_least, "at_least", property)?,
    ))
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
