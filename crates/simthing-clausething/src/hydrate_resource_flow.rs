//! CT-2a literal `produces` / `upkeep` hydration into existing Resource Flow authoring.
//!
//! Maps a small synthetic ClauseScript micro-economy template to [`GameModeSpec`]
//! with flow-property sub-fields and an explicit [`ResourceFlowSpec`] arena graph.
//! Rates are carried separately for fixture seeding; they are not a runtime engine.

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, ClampBehavior, LogTier, SubFieldRole, SubFieldSpec,
};
use simthing_spec::spec::resource_flow::{ResourceFlowOptInMode, ResourceFlowSpec};
use simthing_spec::spec::script::PropertyKey;
use simthing_spec::{ArenaSpec, FissionPolicySpec, GameModeSpec, PropertySpec, SpecVersion};

use crate::error::HydrateError;
use crate::raw::{RawDocument, RawProperty, RawValue};

/// Hydrated Resource Flow pack plus literal produce/upkeep rates for fixture seeding.
#[derive(Debug, Clone)]
pub struct HydratedResourceFlowPack {
    pub game_mode: GameModeSpec,
    /// Positive literal production rate from the `produces` block.
    pub produces_rate: f32,
    /// Positive literal upkeep consumption rate from the `upkeep` block.
    pub upkeep_rate: f32,
}

/// Net signed intrinsic flow at the arena root for one tick: `produces - upkeep`.
pub fn net_intrinsic_flow(pack: &HydratedResourceFlowPack) -> f32 {
    pack.produces_rate - pack.upkeep_rate
}

/// Hydrate one top-level CT-2a micro-economy template from an expanded raw document.
pub fn hydrate_resource_flow_pack(
    document: &RawDocument,
) -> Result<HydratedResourceFlowPack, HydrateError> {
    let RawValue::Block(root) = &document.root else {
        return Err(HydrateError::new("document root must be a property block"));
    };
    if root.properties.len() != 1 {
        return Err(HydrateError::new(
            "CT-2a expects exactly one top-level entity template",
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
    let mut flow_property = None;
    let mut arena = None;
    let mut produces = None;
    let mut upkeep = None;

    for property in &body.properties {
        match property.key.text.as_str() {
            "display_name" => {
                display_name = read_scalar_text(property, "display_name")?;
            }
            "flow_property" => {
                flow_property = Some(parse_flow_property_block(property)?);
            }
            "arena" => {
                arena = Some(parse_arena_block(property)?);
            }
            "produces" => {
                produces = Some(parse_rate_block(property, "produces")?);
            }
            "upkeep" => {
                upkeep = Some(parse_rate_block(property, "upkeep")?);
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported entity field `{other}`"),
                    Some(property.key.span.clone()),
                ));
            }
        }
    }

    let flow_property = require_field(flow_property, "flow_property", entity)?;
    let (
        arena_name,
        arena_flow_key,
        max_participants,
        max_coupling_fanout,
        max_orderband_depth,
        opt_in_mode,
    ) = require_field(arena, "arena", entity)?;
    let produces = require_field(produces, "produces", entity)?;
    let upkeep = require_field(upkeep, "upkeep", entity)?;

    if produces.0 != arena_flow_key || upkeep.0 != arena_flow_key {
        return Err(HydrateError::new(
            "`produces.property` and `upkeep.property` must match `arena.flow_property`",
        ));
    }
    if produces.0.namespace != flow_property.namespace || produces.0.name != flow_property.name {
        return Err(HydrateError::new(
            "`flow_property` identity must match `produces.property` / `upkeep.property`",
        ));
    }

    let property_spec = build_flow_property_spec(&flow_property, &arena_name);
    let resource_flow = ResourceFlowSpec {
        opt_in_mode,
        arenas: vec![ArenaSpec {
            name: arena_name,
            flow_property: arena_flow_key,
            balance_property: None,
            max_participants,
            max_coupling_fanout,
            max_orderband_depth,
            fission_policy: FissionPolicySpec::Reject,
            reserved_orderband_depth: 0,
            reserved_gap_per_intermediate: 0,
            expected_max_children_per_intermediate: 0,
            explicit_participants: Vec::new(),
            enrollment: None,
            wildcard_admission: None,
        }],
        couplings: vec![],
        base_obligations: vec![],
        capacity_budget: None,
        gated_rates: vec![],
                need_weight_profiles: vec![],
    };

    Ok(HydratedResourceFlowPack {
        game_mode: GameModeSpec {
            id: pack_id,
            display_name,
            description: String::new(),
            spec_version: SpecVersion::default(),
            metadata: Default::default(),
            domain_packs: vec![],
            properties: vec![property_spec],
            overlays: vec![],
            capability_trees: vec![],
            events: vec![],
            resource_flow: Some(resource_flow),
            resource_economy: None,
            resource_flow_execution_profile: Default::default(),
            region_fields: vec![],
            mapping_execution_profile: Default::default(),
        },
        produces_rate: produces.1,
        upkeep_rate: upkeep.1,
    })
}

#[derive(Debug, Clone)]
struct FlowPropertyIdentity {
    id: String,
    namespace: String,
    name: String,
    display_name: String,
    description: String,
}

fn parse_flow_property_block(property: &RawProperty) -> Result<FlowPropertyIdentity, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`flow_property` must be a block",
            Some(property.key.span.clone()),
        ));
    };

    let mut id = None;
    let mut namespace = None;
    let mut name = None;
    let mut display_name = String::new();
    let mut description = String::new();

    for field in &block.properties {
        match field.key.text.as_str() {
            "id" => id = Some(read_scalar_text(field, "id")?),
            "namespace" => namespace = Some(read_scalar_text(field, "namespace")?),
            "name" => name = Some(read_scalar_text(field, "name")?),
            "display_name" => display_name = read_scalar_text(field, "display_name")?,
            "description" => description = read_scalar_text(field, "description")?,
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported flow_property field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    Ok(FlowPropertyIdentity {
        id: require_field(id, "id", property)?,
        namespace: require_field(namespace, "namespace", property)?,
        name: require_field(name, "name", property)?,
        display_name,
        description,
    })
}

fn parse_arena_block(
    property: &RawProperty,
) -> Result<(String, PropertyKey, u32, u32, u32, ResourceFlowOptInMode), HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`arena` must be a block",
            Some(property.key.span.clone()),
        ));
    };

    let mut name = None;
    let mut flow_property = None;
    let mut max_participants = None;
    let mut max_coupling_fanout = None;
    let mut max_orderband_depth = None;
    let mut opt_in_mode = ResourceFlowOptInMode::Disabled;

    for field in &block.properties {
        match field.key.text.as_str() {
            "name" => name = Some(read_scalar_text(field, "name")?),
            "flow_property" => flow_property = Some(parse_property_key(field)?),
            "max_participants" => {
                max_participants = Some(read_scalar_u32(field, "max_participants")?);
            }
            "max_coupling_fanout" => {
                max_coupling_fanout = Some(read_scalar_u32(field, "max_coupling_fanout")?);
            }
            "max_orderband_depth" => {
                max_orderband_depth = Some(read_scalar_u32(field, "max_orderband_depth")?);
            }
            "opt_in" => {
                opt_in_mode = parse_opt_in_mode(field)?;
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported arena field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    Ok((
        require_field(name, "name", property)?,
        require_field(flow_property, "flow_property", property)?,
        require_field(max_participants, "max_participants", property)?,
        require_field(max_coupling_fanout, "max_coupling_fanout", property)?,
        require_field(max_orderband_depth, "max_orderband_depth", property)?,
        opt_in_mode,
    ))
}

fn parse_rate_block(
    property: &RawProperty,
    block_name: &str,
) -> Result<(PropertyKey, f32), HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            format!("`{block_name}` must be a block"),
            Some(property.key.span.clone()),
        ));
    };

    let mut property_key = None;
    let mut rate = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "property" => property_key = Some(parse_property_key(field)?),
            "rate" => rate = Some(read_scalar_f32(field, "rate")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported {block_name} field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    Ok((
        require_field(property_key, "property", property)?,
        require_field(rate, "rate", property)?,
    ))
}

fn parse_property_key(property: &RawProperty) -> Result<PropertyKey, HydrateError> {
    let text = read_scalar_text(property, "property")?;
    let Some((namespace, name)) = text.split_once("::") else {
        return Err(HydrateError::new_spanned(
            format!("property reference must be `namespace::name`, got `{text}`"),
            Some(property.key.span.clone()),
        ));
    };
    if namespace.is_empty() || name.is_empty() {
        return Err(HydrateError::new_spanned(
            format!("property reference must be `namespace::name`, got `{text}`"),
            Some(property.key.span.clone()),
        ));
    }
    Ok(PropertyKey::new(namespace, name))
}

fn parse_opt_in_mode(property: &RawProperty) -> Result<ResourceFlowOptInMode, HydrateError> {
    let text = read_scalar_text(property, "opt_in")?;
    match text.as_str() {
        "FlatStarOptIn" => Ok(ResourceFlowOptInMode::FlatStarOptIn),
        "Disabled" => Ok(ResourceFlowOptInMode::Disabled),
        other => Err(HydrateError::new_spanned(
            format!("`opt_in` must be `FlatStarOptIn` or `Disabled`, got `{other}`"),
            Some(property.key.span.clone()),
        )),
    }
}

fn build_flow_property_spec(flow: &FlowPropertyIdentity, arena_name: &str) -> PropertySpec {
    PropertySpec {
        id: flow.id.clone(),
        namespace: flow.namespace.clone(),
        name: flow.name.clone(),
        display_name: flow.display_name.clone(),
        description: flow.description.clone(),
        sub_fields: vec![
            flow_subfield("flow", AccumulatorRole::IntrinsicFlow),
            flow_subfield(
                "allocated",
                AccumulatorRole::AllocatedFlow {
                    arena: arena_name.into(),
                },
            ),
            flow_subfield(
                "weight",
                AccumulatorRole::AllocatorWeight {
                    arena: arena_name.into(),
                },
            ),
        ],
    }
}

fn flow_subfield(name: &str, role: AccumulatorRole) -> SubFieldSpec {
    SubFieldSpec {
        role: SubFieldRole::Named(name.into()),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: name.into(),
        display_range: None,
        governed_by: None,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: Some(AccumulatorSpec {
            role,
            log_tier: LogTier::Summary,
        }),
    }
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

fn read_scalar_u32(property: &RawProperty, field: &str) -> Result<u32, HydrateError> {
    let text = read_scalar_text(property, field)?;
    text.parse::<u32>().map_err(|_| {
        HydrateError::new_spanned(
            format!("`{field}` must be a non-negative integer literal, got `{text}`"),
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
