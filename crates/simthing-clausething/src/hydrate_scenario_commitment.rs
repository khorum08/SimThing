//! PR6: scenario-contained FIELD_POLICY threshold / commitment feedstock authoring.
//!
//! Lowers authored `commitment` blocks into existing `FirstSliceCommitmentSpec`,
//! `RegionFieldFormulaBindingSpec`, and reduction feedstock on the referenced scenario
//! field operator. No CPU planner, movement, routes, or runtime semantics.

use simthing_spec::FIRST_SLICE_FIELD_URGENCY_COL;
use simthing_spec::spec::region_field::{
    CommitmentEffectSpec, FirstSliceCommitmentDirectionSpec, FirstSliceCommitmentSpec,
    RegionFieldFormulaBindingSpec, RegionFieldOperatorSpec, RegionFieldReductionSpec,
};

use crate::error::HydrateError;
use crate::hydrate_category_economy::parse_commitment_effect_block;
use crate::hydrate_field_operator::HydratedFieldOperatorPack;
use crate::raw::{RawBlock, RawHeaderValue, RawProperty, RawSpan, RawValue};

/// PR6 admits one scenario-contained commitment block per document.
pub const PR6_MAX_SCENARIO_COMMITMENT: usize = 1;

const FORBIDDEN_COMMITMENT_FIELDS: &[&str] = &[
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
    "palma_feedstock",
    "field_operator",
];

/// Scenario commitment metadata lowered from authoring.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HydratedScenarioCommitment {
    pub commitment_id: String,
    pub source_field_operator_id: String,
    pub field_urgency_column: Option<u32>,
    pub commitment: FirstSliceCommitmentSpec,
}

pub(crate) enum ParsedCommitmentEffectDraft {
    Resolved(CommitmentEffectSpec),
    AttachOverlay {
        target_id: String,
        overlay_id: String,
    },
}

pub(crate) struct ParsedCommitmentDraft {
    pub commitment_id: String,
    pub threshold: f32,
    pub event_kind: u32,
    pub field_urgency_source: String,
    pub field_urgency_column: Option<u32>,
    pub weight_pressure: f32,
    pub weight_resource: f32,
    pub effect: Option<ParsedCommitmentEffectDraft>,
}

pub struct FinalizedScenarioCommitment {
    pub metadata: HydratedScenarioCommitment,
    pub parent_formula: RegionFieldFormulaBindingSpec,
    pub reduction: RegionFieldReductionSpec,
}

pub fn parse_commitment_property(
    property: &RawProperty,
) -> Result<ParsedCommitmentDraft, HydrateError> {
    let (commitment_id, body) = commitment_id_and_body(property)?;
    parse_commitment_body(&commitment_id, body, Some(property.key.span.clone()))
}

pub fn finalize_scenario_commitment(
    draft: ParsedCommitmentDraft,
    field_operator: &HydratedFieldOperatorPack,
    effect: Option<CommitmentEffectSpec>,
) -> Result<FinalizedScenarioCommitment, HydrateError> {
    if draft.field_urgency_source != field_operator.game_mode.id {
        return Err(HydrateError::new(format!(
            "commitment field_urgency source `{}` is not a scenario field_operator id",
            draft.field_urgency_source
        )));
    }

    let field = field_operator
        .game_mode
        .region_fields
        .first()
        .ok_or_else(|| {
            HydrateError::new("commitment field_urgency source field_operator has no region field")
        })?;

    let choke_output_col = match &field.operator {
        RegionFieldOperatorSpec::SaturatingFlux {
            choke_output_col, ..
        } => *choke_output_col,
        _ => {
            return Err(HydrateError::new(
                "commitment field_urgency source field_operator must lower SaturatingFlux",
            ));
        }
    };

    if let Some(column) = draft.field_urgency_column {
        validate_urgency_column(column, field.n_dims, field.source_col)?;
        if choke_output_col != Some(column) {
            return Err(HydrateError::new(format!(
                "commitment field_urgency column {column} must match saturating_flux choke_output_col"
            )));
        }
    }

    let cell_count = field.grid_size.saturating_mul(field.grid_size);
    let parent_formula = RegionFieldFormulaBindingSpec {
        formula_class: "field_urgency".into(),
        tree_id: None,
        weight_pressure: Some(draft.weight_pressure),
        weight_resource: Some(draft.weight_resource),
    };
    let reduction = RegionFieldReductionSpec {
        child_slot_start: 0,
        child_slot_count: cell_count,
        child_col: 0,
        parent_slot: cell_count,
        parent_col: 0,
        order_band: 0,
    };
    let commitment = FirstSliceCommitmentSpec {
        source_formula_class: "field_urgency".into(),
        parent_slot: cell_count,
        urgency_col: FIRST_SLICE_FIELD_URGENCY_COL,
        threshold: draft.threshold,
        direction: FirstSliceCommitmentDirectionSpec::Upward,
        event_kind: draft.event_kind,
        effect,
    };

    Ok(FinalizedScenarioCommitment {
        metadata: HydratedScenarioCommitment {
            commitment_id: draft.commitment_id,
            source_field_operator_id: draft.field_urgency_source,
            field_urgency_column: draft.field_urgency_column,
            commitment: commitment.clone(),
        },
        parent_formula,
        reduction,
    })
}

fn parse_commitment_body(
    commitment_id: &str,
    body: &RawBlock,
    span: Option<RawSpan>,
) -> Result<ParsedCommitmentDraft, HydrateError> {
    let mut threshold = None;
    let mut event_kind = None;
    let mut field_urgency = None;
    let mut effect = None;

    for field in &body.properties {
        reject_forbidden_commitment_field(field)?;
        match field.key.text.as_str() {
            "threshold" => threshold = Some(read_scalar_f32(field, "threshold")?),
            "event_kind" => event_kind = Some(read_scalar_u32(field, "event_kind")?),
            "field_urgency" => field_urgency = Some(parse_field_urgency_block(field)?),
            "effect" => {
                effect = Some(parse_scenario_commitment_effect(field)?);
            }
            "enabled" => {
                let enabled = read_scalar_text(field, "enabled")?;
                if matches!(enabled.as_str(), "true" | "yes" | "1") {
                    return Err(HydrateError::new_spanned(
                        "commitment must remain default-off; enabled=true is rejected",
                        Some(field.key.span.clone()),
                    ));
                }
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported commitment field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    let field_urgency = require_field(field_urgency, "field_urgency", span.clone())?;
    Ok(ParsedCommitmentDraft {
        commitment_id: commitment_id.to_string(),
        threshold: require_field(threshold, "threshold", span.clone())?,
        event_kind: require_field(event_kind, "event_kind", span.clone())?,
        field_urgency_source: field_urgency.source,
        field_urgency_column: field_urgency.column,
        weight_pressure: field_urgency.weight_pressure,
        weight_resource: field_urgency.weight_resource,
        effect,
    })
}

struct ParsedFieldUrgencyDraft {
    source: String,
    column: Option<u32>,
    weight_pressure: f32,
    weight_resource: f32,
}

fn parse_field_urgency_block(
    property: &RawProperty,
) -> Result<ParsedFieldUrgencyDraft, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`field_urgency` must be a block",
            Some(property.key.span.clone()),
        ));
    };

    let mut source = None;
    let mut column = None;
    let mut weight = None;
    let mut weight_pressure = None;
    let mut weight_resource = None;

    for field in &block.properties {
        reject_forbidden_commitment_field(field)?;
        match field.key.text.as_str() {
            "source" => source = Some(read_scalar_text(field, "source")?),
            "column" => column = Some(read_scalar_u32(field, "column")?),
            "weight" => weight = Some(read_scalar_f32(field, "weight")?),
            "weight_pressure" => {
                weight_pressure = Some(read_scalar_f32(field, "weight_pressure")?);
            }
            "weight_resource" => {
                weight_resource = Some(read_scalar_f32(field, "weight_resource")?);
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported field_urgency field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    let default_weight = weight.unwrap_or(1.0);
    if !default_weight.is_finite() {
        return Err(HydrateError::new("`field_urgency.weight` must be finite"));
    }
    let weight_pressure = weight_pressure.unwrap_or(default_weight);
    let weight_resource = weight_resource.unwrap_or(default_weight);
    if !weight_pressure.is_finite() {
        return Err(HydrateError::new(
            "`field_urgency.weight_pressure` must be finite",
        ));
    }
    if !weight_resource.is_finite() {
        return Err(HydrateError::new(
            "`field_urgency.weight_resource` must be finite",
        ));
    }

    Ok(ParsedFieldUrgencyDraft {
        source: require_field(source, "source", Some(property.key.span.clone()))?,
        column,
        weight_pressure,
        weight_resource,
    })
}

fn parse_scenario_commitment_effect(
    property: &RawProperty,
) -> Result<ParsedCommitmentEffectDraft, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`effect` must be a block",
            Some(property.key.span.clone()),
        ));
    };

    let mut attach_overlay = None;
    let mut target = None;
    for field in &block.properties {
        reject_forbidden_commitment_field(field)?;
        match field.key.text.as_str() {
            "attach_overlay" => attach_overlay = Some(read_scalar_text(field, "attach_overlay")?),
            "target" => target = Some(read_scalar_text(field, "target")?),
            "targets_property" | "amount_mult" | "amount_add" => {}
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported effect field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    if attach_overlay.is_some() {
        let overlay_id = attach_overlay.expect("checked above");
        let target_id = require_field(target, "target", Some(property.key.span.clone()))?;
        return Ok(ParsedCommitmentEffectDraft::AttachOverlay {
            target_id,
            overlay_id,
        });
    }

    Ok(ParsedCommitmentEffectDraft::Resolved(
        parse_commitment_effect_block(property)?,
    ))
}

fn commitment_id_and_body<'a>(
    property: &'a RawProperty,
) -> Result<(String, &'a RawBlock), HydrateError> {
    match &property.value {
        RawValue::Header(RawHeaderValue { header, payload }) => {
            let RawValue::Block(block) = payload.as_ref() else {
                return Err(HydrateError::new_spanned(
                    "`commitment` header payload must be a block",
                    Some(header.span.clone()),
                ));
            };
            if header.text.is_empty() {
                return Err(HydrateError::new_spanned(
                    "`commitment` requires an id",
                    Some(header.span.clone()),
                ));
            }
            Ok((header.text.clone(), block))
        }
        RawValue::Block(block) => {
            if property.key.text.is_empty() {
                return Err(HydrateError::new_spanned(
                    "`commitment` requires an id",
                    Some(property.key.span.clone()),
                ));
            }
            Ok((property.key.text.clone(), block))
        }
        _ => Err(HydrateError::new_spanned(
            "`commitment` must be a block or header block",
            Some(property.key.span.clone()),
        )),
    }
}

fn validate_urgency_column(column: u32, n_dims: u32, source_col: u32) -> Result<(), HydrateError> {
    if column >= n_dims {
        return Err(HydrateError::new(format!(
            "commitment field_urgency column {column} out of range for n_dims {n_dims}"
        )));
    }
    if column == source_col {
        return Err(HydrateError::new(format!(
            "commitment field_urgency column {column} must differ from source_col {source_col}"
        )));
    }
    Ok(())
}

fn reject_forbidden_commitment_field(property: &RawProperty) -> Result<(), HydrateError> {
    let key = property.key.text.as_str();
    if FORBIDDEN_COMMITMENT_FIELDS.contains(&key) {
        return Err(HydrateError::new_spanned(
            format!("`{key}` is outside PR6 commitment grammar"),
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

fn read_scalar_f32(property: &RawProperty, field: &str) -> Result<f32, HydrateError> {
    let text = read_scalar_text(property, field)?;
    let value = text.parse::<f32>().map_err(|_| {
        HydrateError::new_spanned(
            format!("`{field}` must be a numeric literal, got `{text}`"),
            Some(property.key.span.clone()),
        )
    })?;
    if !value.is_finite() {
        return Err(HydrateError::new_spanned(
            format!("`{field}` must be finite"),
            Some(property.key.span.clone()),
        ));
    }
    Ok(value)
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
