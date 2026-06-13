//! PR5: scenario-contained PALMA W/D feedstock authoring → generic feedstock DTO.
//!
//! Lowers authored `palma_feedstock` blocks into inert serde metadata consumed by later
//! driver/admission rungs. No pathfinding, movement, routes, or runtime semantics.

use serde::{Deserialize, Serialize};
use simthing_spec::RegionFieldOperatorSpec;

use crate::error::HydrateError;
use crate::hydrate_field_operator::HydratedFieldOperatorPack;
use crate::raw::{RawBlock, RawHeaderValue, RawProperty, RawSpan, RawValue};

/// PR5 admits one scenario-contained PALMA feedstock block per document.
pub const PR5_MAX_SCENARIO_PALMA_FEEDSTOCK: usize = 1;

const FORBIDDEN_PALMA_FEEDSTOCK_FIELDS: &[&str] = &[
    "adjacency",
    "edge",
    "route",
    "path",
    "predecessor",
    "movement",
    "movement_order",
    "waypoint",
    "destination",
    "frontline",
    "border",
    "pathfinding",
    "arbitrary_graph",
    "non_grid_topology",
    "plan",
    "field_operator",
];

/// Generic PALMA W/D feedstock metadata lowered from scenario authoring.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HydratedScenarioPalmaFeedstock {
    pub feedstock_id: String,
    pub w_source_field_operator_id: String,
    pub w_output_col: u32,
    pub d_output_col: u32,
    pub grid_size: u32,
    pub n_dims: u32,
    pub source_col: u32,
    pub choke_output_col: Option<u32>,
}

/// Parse one scenario `palma_feedstock` property block (validation deferred).
pub fn parse_palma_feedstock_property(
    property: &RawProperty,
) -> Result<ParsedPalmaFeedstockDraft, HydrateError> {
    let (feedstock_id, body) = palma_feedstock_id_and_body(property)?;
    parse_palma_feedstock_body(&feedstock_id, body, Some(property.key.span.clone()))
}

pub(crate) struct ParsedPalmaFeedstockDraft {
    pub feedstock_id: String,
    pub w_source: String,
    pub w_output_col: u32,
    pub d_output_col: u32,
}

pub fn finalize_palma_feedstock(
    draft: ParsedPalmaFeedstockDraft,
    field_operator: &HydratedFieldOperatorPack,
) -> Result<HydratedScenarioPalmaFeedstock, HydrateError> {
    if draft.w_source != field_operator.game_mode.id {
        return Err(HydrateError::new(format!(
            "palma_feedstock w_source `{}` is not a scenario field_operator id",
            draft.w_source
        )));
    }

    let field = field_operator
        .game_mode
        .region_fields
        .first()
        .ok_or_else(|| {
            HydrateError::new("palma_feedstock w_source field_operator has no region field")
        })?;

    let choke_output_col = match &field.operator {
        RegionFieldOperatorSpec::SaturatingFlux {
            choke_output_col, ..
        } => *choke_output_col,
        _ => {
            return Err(HydrateError::new(
                "palma_feedstock w_source field_operator must lower SaturatingFlux",
            ));
        }
    };

    validate_palma_columns(
        draft.w_output_col,
        draft.d_output_col,
        field.n_dims,
        field.source_col,
    )?;

    Ok(HydratedScenarioPalmaFeedstock {
        feedstock_id: draft.feedstock_id,
        w_source_field_operator_id: draft.w_source,
        w_output_col: draft.w_output_col,
        d_output_col: draft.d_output_col,
        grid_size: field.grid_size,
        n_dims: field.n_dims,
        source_col: field.source_col,
        choke_output_col,
    })
}

fn palma_feedstock_id_and_body<'a>(
    property: &'a RawProperty,
) -> Result<(String, &'a RawBlock), HydrateError> {
    match &property.value {
        RawValue::Header(RawHeaderValue { header, payload }) => {
            let RawValue::Block(block) = payload.as_ref() else {
                return Err(HydrateError::new_spanned(
                    "`palma_feedstock` header payload must be a block",
                    Some(header.span.clone()),
                ));
            };
            if header.text.is_empty() {
                return Err(HydrateError::new_spanned(
                    "`palma_feedstock` requires an id",
                    Some(header.span.clone()),
                ));
            }
            Ok((header.text.clone(), block))
        }
        RawValue::Block(block) => {
            if property.key.text.is_empty() {
                return Err(HydrateError::new_spanned(
                    "`palma_feedstock` requires an id",
                    Some(property.key.span.clone()),
                ));
            }
            Ok((property.key.text.clone(), block))
        }
        _ => Err(HydrateError::new_spanned(
            "`palma_feedstock` must be a block or header block",
            Some(property.key.span.clone()),
        )),
    }
}

fn parse_palma_feedstock_body(
    feedstock_id: &str,
    body: &RawBlock,
    span: Option<RawSpan>,
) -> Result<ParsedPalmaFeedstockDraft, HydrateError> {
    let mut w_source = None;
    let mut w_output_col = None;
    let mut d_output_col = None;

    for field in &body.properties {
        reject_forbidden_palma_field(field)?;
        match field.key.text.as_str() {
            "w_source" => w_source = Some(read_scalar_text(field, "w_source")?),
            "w_output_col" => w_output_col = Some(read_scalar_u32(field, "w_output_col")?),
            "d_output_col" => d_output_col = Some(read_scalar_u32(field, "d_output_col")?),
            "enabled" => {
                let enabled = read_scalar_text(field, "enabled")?;
                if matches!(enabled.as_str(), "true" | "yes" | "1") {
                    return Err(HydrateError::new_spanned(
                        "palma_feedstock must remain default-off; enabled=true is rejected",
                        Some(field.key.span.clone()),
                    ));
                }
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported palma_feedstock field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    Ok(ParsedPalmaFeedstockDraft {
        feedstock_id: feedstock_id.to_string(),
        w_source: require_field(w_source, "w_source", span.clone())?,
        w_output_col: require_field(w_output_col, "w_output_col", span.clone())?,
        d_output_col: require_field(d_output_col, "d_output_col", span)?,
    })
}

fn validate_palma_columns(
    w_output_col: u32,
    d_output_col: u32,
    n_dims: u32,
    source_col: u32,
) -> Result<(), HydrateError> {
    if w_output_col >= n_dims {
        return Err(HydrateError::new(format!(
            "palma_feedstock w_output_col {w_output_col} out of range for n_dims {n_dims}"
        )));
    }
    if d_output_col >= n_dims {
        return Err(HydrateError::new(format!(
            "palma_feedstock d_output_col {d_output_col} out of range for n_dims {n_dims}"
        )));
    }
    if w_output_col == d_output_col {
        return Err(HydrateError::new(
            "palma_feedstock w_output_col and d_output_col must differ",
        ));
    }
    if w_output_col == source_col {
        return Err(HydrateError::new(format!(
            "palma_feedstock w_output_col {w_output_col} must differ from source_col {source_col}"
        )));
    }
    if d_output_col == source_col {
        return Err(HydrateError::new(format!(
            "palma_feedstock d_output_col {d_output_col} must differ from source_col {source_col}"
        )));
    }
    Ok(())
}

fn reject_forbidden_palma_field(property: &RawProperty) -> Result<(), HydrateError> {
    let key = property.key.text.as_str();
    if FORBIDDEN_PALMA_FEEDSTOCK_FIELDS.contains(&key) {
        return Err(HydrateError::new_spanned(
            format!("`{key}` is outside PR5 palma_feedstock grammar"),
            Some(property.key.span.clone()),
        ));
    }
    Ok(())
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
    span: Option<RawSpan>,
) -> Result<T, HydrateError> {
    value
        .ok_or_else(|| HydrateError::new_spanned(format!("missing required field `{field}`"), span))
}
