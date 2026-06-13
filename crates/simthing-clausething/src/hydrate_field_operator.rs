//! BH-3-AUTHORING-0 / PR4: ClauseScript field-operator profile → generic `simthing-spec` surfaces.
//!
//! Provisional project-local authoring syntax lowers into existing RegionField,
//! W impedance compose, stress compose, and threshold feedstock structs.
//! No runtime wiring; default-off.

use simthing_spec::spec::region_field::{
    FirstSliceCommitmentDirectionSpec, FirstSliceCommitmentSpec, MappingExecutionProfile,
    RegionFieldCadenceSpec, RegionFieldFormulaBindingSpec, RegionFieldGridProfile,
    RegionFieldOperatorSpec, RegionFieldReductionSpec, RegionFieldSourcePolicySpec,
    RegionFieldSpec,
};
use simthing_spec::spec::stress_compose::{
    StressComposeProfileSpec, StressComposeSpec, StressOperatorSpec,
};
use simthing_spec::spec::w_impedance_compose::{
    WImpedanceComposeProfileSpec, WImpedanceComposeSpec,
};
use simthing_spec::{GameModeSpec, SpecVersion};

use crate::error::HydrateError;
use crate::raw::{RawBlock, RawDocument, RawHeaderValue, RawProperty, RawSpan, RawValue};

/// Maximum authored impedance profiles per field-operator block (BH-3 v0 cap).
pub const BH3_MAX_FIELD_IMPEDANCE_PROFILES: usize = 1;
/// Maximum authored stress profiles per field-operator block (BH-3 v0 cap).
pub const BH3_MAX_FIELD_STRESS_PROFILES: usize = 1;
/// BH-0 CFL bound mirrored at hydrate time: dt is fixed at 1.0, so chi <= 0.25.
pub const BH3_SATURATING_FLUX_CHI_CFL_MAX: f32 = 0.25;

const FORBIDDEN_FIELD_OPERATOR_FIELDS: &[&str] = &[
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
    "palma_feedstock",
];

/// Hydrated BH-3 field-operator pack: generic spec surfaces only.
#[derive(Debug, Clone)]
pub struct HydratedFieldOperatorPack {
    pub game_mode: GameModeSpec,
    pub w_impedance_compose: Option<WImpedanceComposeSpec>,
    pub stress_compose: Option<StressComposeSpec>,
}

/// Hydrate one top-level BH-3 field-operator profile from an expanded raw document.
pub fn hydrate_field_operator_pack(
    document: &RawDocument,
) -> Result<HydratedFieldOperatorPack, HydrateError> {
    let RawValue::Block(root) = &document.root else {
        return Err(HydrateError::new("document root must be a property block"));
    };
    if root.properties.len() != 1 {
        return Err(HydrateError::new(
            "BH-3 expects exactly one top-level field_operator block",
        ));
    }
    hydrate_field_operator_property(&root.properties[0])
}

/// Hydrate one authored `field_operator` property block (standalone or scenario-contained).
pub fn hydrate_field_operator_property(
    property: &RawProperty,
) -> Result<HydratedFieldOperatorPack, HydrateError> {
    let (pack_id, body) = field_operator_pack_id_and_body(property)?;
    hydrate_field_operator_body(&pack_id, body, Some(property.key.span.clone()))
}

fn field_operator_pack_id_and_body<'a>(
    property: &'a RawProperty,
) -> Result<(String, &'a RawBlock), HydrateError> {
    match &property.value {
        RawValue::Header(RawHeaderValue { header, payload }) => {
            let RawValue::Block(block) = payload.as_ref() else {
                return Err(HydrateError::new_spanned(
                    "`field_operator` header payload must be a block",
                    Some(header.span.clone()),
                ));
            };
            if header.text.is_empty() {
                return Err(HydrateError::new_spanned(
                    "`field_operator` requires an id",
                    Some(header.span.clone()),
                ));
            }
            Ok((header.text.clone(), block))
        }
        RawValue::Block(block) => {
            if property.key.text.is_empty() {
                return Err(HydrateError::new_spanned(
                    "`field_operator` requires an id",
                    Some(property.key.span.clone()),
                ));
            }
            Ok((property.key.text.clone(), block))
        }
        _ => Err(HydrateError::new_spanned(
            "`field_operator` must be a block or header block",
            Some(property.key.span.clone()),
        )),
    }
}

fn hydrate_field_operator_body(
    pack_id: &str,
    body: &RawBlock,
    pack_span: Option<RawSpan>,
) -> Result<HydratedFieldOperatorPack, HydrateError> {
    let mut display_name = pack_id.to_string();
    let mut grid_size = None;
    let mut source_col = None;
    let mut target_col = None;
    let mut n_dims = None;
    let mut alpha_self = 0.5_f32;
    let mut gamma_neighbor = 0.125_f32;
    let mut horizon = 8_u32;
    let mut saturating_flux = None;
    let mut field_impedance_property = None;
    let mut field_stress_property = None;
    let mut threshold_feedstock = None;
    let mut parent_formula = None;

    for property in &body.properties {
        reject_forbidden_field_operator_field(property)?;
        match property.key.text.as_str() {
            "display_name" => display_name = read_scalar_text(property, "display_name")?,
            "grid_size" => grid_size = Some(read_scalar_u32(property, "grid_size")?),
            "source_col" => source_col = Some(read_scalar_u32(property, "source_col")?),
            "target_col" => target_col = Some(read_scalar_u32(property, "target_col")?),
            "n_dims" => n_dims = Some(read_scalar_u32(property, "n_dims")?),
            "alpha_self" => alpha_self = read_scalar_f32(property, "alpha_self")?,
            "gamma_neighbor" => gamma_neighbor = read_scalar_f32(property, "gamma_neighbor")?,
            "horizon" => horizon = read_scalar_u32(property, "horizon")?,
            "saturating_flux" => {
                saturating_flux = Some(parse_saturating_flux_block(property)?);
            }
            "field_impedance" => {
                field_impedance_property = Some(property);
            }
            "field_stress" => {
                field_stress_property = Some(property);
            }
            "threshold_feedstock" => {
                threshold_feedstock = Some(parse_threshold_feedstock_block(property)?);
            }
            "parent_formula" => {
                parent_formula = Some(parse_parent_formula_block(property)?);
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported field_operator field `{other}`"),
                    Some(property.key.span.clone()),
                ));
            }
        }
    }

    let grid_size = require_field(grid_size, "grid_size", pack_span.clone())?;
    let source_col = require_field(source_col, "source_col", pack_span.clone())?;
    let target_col = require_field(target_col, "target_col", pack_span.clone())?;
    let n_dims = require_field(n_dims, "n_dims", pack_span.clone())?;
    let (u_sat, chi, choke_output_col) =
        require_field(saturating_flux, "saturating_flux", pack_span.clone())?;

    validate_saturating_flux_params(
        u_sat,
        chi,
        choke_output_col,
        source_col,
        n_dims,
        pack_span.clone(),
    )?;

    let field_impedance = field_impedance_property
        .map(|property| parse_field_impedance_block(property, choke_output_col))
        .transpose()?;
    let field_stress = field_stress_property
        .map(|property| parse_field_stress_block(property, choke_output_col))
        .transpose()?;

    if source_col != target_col {
        return Err(HydrateError::new_spanned(
            "SaturatingFlux authoring requires source_col == target_col",
            pack_span,
        ));
    }

    let region_field = RegionFieldSpec {
        name: format!("{pack_id}_field"),
        grid_size,
        n_dims,
        source_col,
        target_col,
        operator: RegionFieldOperatorSpec::SaturatingFlux {
            u_sat,
            chi,
            choke_output_col,
        },
        horizon,
        allow_extended_horizon: false,
        alpha_self,
        gamma_neighbor,
        source_cap: None,
        source_policy: RegionFieldSourcePolicySpec::CallerManagedOneShotSeedThenZero,
        cadence: RegionFieldCadenceSpec::EveryTick,
        grid_profile: RegionFieldGridProfile::StandardSquare,
        reduction: threshold_feedstock
            .as_ref()
            .map(|commitment| RegionFieldReductionSpec {
                child_slot_start: 0,
                child_slot_count: 100,
                child_col: 0,
                parent_slot: commitment.parent_slot,
                parent_col: 0,
                order_band: 0,
            }),
        parent_formula,
        commitment: threshold_feedstock,
        request_atlas_batching: false,
        max_region_field_vram_bytes: None,
        summary_policy: Default::default(),
        pressure_binding: None,
    };

    let w_impedance_compose = field_impedance.map(
        |(base_w_col, choke_a_col, choke_b_col, weight_a, weight_b, output_w_col)| {
            WImpedanceComposeSpec {
                width: grid_size,
                height: grid_size,
                n_dims,
                base_w_col,
                choke_a_col,
                choke_b_col,
                profiles: vec![WImpedanceComposeProfileSpec {
                    weight_a,
                    weight_b,
                    output_w_col,
                }],
            }
        },
    );

    let stress_compose =
        field_stress.map(
            |(choke_a_col, choke_b_col, operator, output_col)| StressComposeSpec {
                width: grid_size,
                height: grid_size,
                n_dims,
                choke_a_col,
                choke_b_col,
                profiles: vec![StressComposeProfileSpec {
                    operator,
                    output_col,
                }],
            },
        );

    Ok(HydratedFieldOperatorPack {
        game_mode: GameModeSpec {
            id: pack_id.to_string(),
            display_name,
            description: String::new(),
            spec_version: SpecVersion::default(),
            metadata: Default::default(),
            domain_packs: vec![],
            properties: vec![],
            overlays: vec![],
            capability_trees: vec![],
            events: vec![],
            resource_flow: None,
            resource_economy: None,
            resource_flow_execution_profile: Default::default(),
            region_fields: vec![region_field],
            mapping_execution_profile: MappingExecutionProfile::Disabled,
        },
        w_impedance_compose,
        stress_compose,
    })
}

fn validate_saturating_flux_params(
    u_sat: f32,
    chi: f32,
    choke_output_col: Option<u32>,
    source_col: u32,
    n_dims: u32,
    span: Option<RawSpan>,
) -> Result<(), HydrateError> {
    if !u_sat.is_finite() || u_sat <= 0.0 {
        return Err(HydrateError::new_spanned(
            "SaturatingFlux u_sat must be finite and > 0",
            span.clone(),
        ));
    }
    if !chi.is_finite() || chi <= 0.0 {
        return Err(HydrateError::new_spanned(
            "SaturatingFlux chi must be finite and > 0",
            span.clone(),
        ));
    }
    if chi > BH3_SATURATING_FLUX_CHI_CFL_MAX {
        return Err(HydrateError::new_spanned(
            format!(
                "SaturatingFlux chi {chi} exceeds CFL bound {BH3_SATURATING_FLUX_CHI_CFL_MAX} (dt=1.0)"
            ),
            span.clone(),
        ));
    }
    if let Some(choke_col) = choke_output_col {
        if choke_col >= n_dims {
            return Err(HydrateError::new_spanned(
                format!("SaturatingFlux choke_output_col {choke_col} out of range for n_dims"),
                span.clone(),
            ));
        }
        if choke_col == source_col {
            return Err(HydrateError::new_spanned(
                format!("SaturatingFlux choke_output_col {choke_col} must differ from source_col"),
                span,
            ));
        }
    }
    Ok(())
}

fn reject_forbidden_field_operator_field(property: &RawProperty) -> Result<(), HydrateError> {
    let key = property.key.text.as_str();
    if FORBIDDEN_FIELD_OPERATOR_FIELDS.contains(&key) {
        return Err(HydrateError::new_spanned(
            format!("`{key}` is outside BH-3 field_operator grammar"),
            Some(property.key.span.clone()),
        ));
    }
    Ok(())
}

fn parse_saturating_flux_block(
    property: &RawProperty,
) -> Result<(f32, f32, Option<u32>), HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`saturating_flux` must be a block",
            Some(property.key.span.clone()),
        ));
    };

    let mut u_sat = None;
    let mut chi = None;
    let mut choke_output_col = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "u_sat" => u_sat = Some(read_scalar_f32(field, "u_sat")?),
            "chi" => chi = Some(read_scalar_f32(field, "chi")?),
            "choke_output_col" => {
                choke_output_col = Some(read_scalar_u32(field, "choke_output_col")?);
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported saturating_flux field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    Ok((
        require_field(u_sat, "u_sat", Some(property.key.span.clone()))?,
        require_field(chi, "chi", Some(property.key.span.clone()))?,
        choke_output_col,
    ))
}

fn parse_field_impedance_block(
    property: &RawProperty,
    flux_choke_col: Option<u32>,
) -> Result<(u32, u32, u32, f32, f32, u32), HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`field_impedance` must be a block",
            Some(property.key.span.clone()),
        ));
    };

    let mut base_w_col = None;
    let mut choke_a_col = None;
    let mut choke_b_col = None;
    let mut weight_a = None;
    let mut weight_b = None;
    let mut output_w_col = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "base_w_col" => base_w_col = Some(read_scalar_u32(field, "base_w_col")?),
            "choke_a_col" => choke_a_col = Some(read_scalar_u32(field, "choke_a_col")?),
            "choke_b_col" => choke_b_col = Some(read_scalar_u32(field, "choke_b_col")?),
            "weight_a" => weight_a = Some(read_scalar_f32(field, "weight_a")?),
            "weight_b" => weight_b = Some(read_scalar_f32(field, "weight_b")?),
            "output_w_col" => output_w_col = Some(read_scalar_u32(field, "output_w_col")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported field_impedance field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    let choke_a = choke_a_col.or(flux_choke_col).ok_or_else(|| {
        HydrateError::new_spanned(
            "field_impedance requires choke_a_col or saturating_flux.choke_output_col",
            Some(property.key.span.clone()),
        )
    })?;
    let choke_b = choke_b_col
        .or(flux_choke_col.map(|c| c.saturating_add(1)))
        .ok_or_else(|| {
            HydrateError::new_spanned(
                "field_impedance requires choke_b_col or saturating_flux.choke_output_col",
                Some(property.key.span.clone()),
            )
        })?;

    Ok((
        require_field(base_w_col, "base_w_col", Some(property.key.span.clone()))?,
        choke_a,
        choke_b,
        require_field(weight_a, "weight_a", Some(property.key.span.clone()))?,
        require_field(weight_b, "weight_b", Some(property.key.span.clone()))?,
        require_field(
            output_w_col,
            "output_w_col",
            Some(property.key.span.clone()),
        )?,
    ))
}

fn parse_field_stress_block(
    property: &RawProperty,
    flux_choke_col: Option<u32>,
) -> Result<(u32, u32, StressOperatorSpec, u32), HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`field_stress` must be a block",
            Some(property.key.span.clone()),
        ));
    };

    let mut operator = None;
    let mut output_col = None;
    let mut choke_a_col = None;
    let mut choke_b_col = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "choke_a_col" => choke_a_col = Some(read_scalar_u32(field, "choke_a_col")?),
            "choke_b_col" => choke_b_col = Some(read_scalar_u32(field, "choke_b_col")?),
            "operator" => {
                let text = read_scalar_text(field, "operator")?;
                operator = Some(match text.as_str() {
                    "overlap" => StressOperatorSpec::Overlap,
                    "mismatch" => StressOperatorSpec::Mismatch,
                    other => {
                        return Err(HydrateError::new_spanned(
                            format!(
                                "`field_stress.operator` must be `overlap` or `mismatch`, got `{other}`"
                            ),
                            Some(field.key.span.clone()),
                        ));
                    }
                });
            }
            "output_col" => output_col = Some(read_scalar_u32(field, "output_col")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported field_stress field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    let choke_a = choke_a_col.or(flux_choke_col).ok_or_else(|| {
        HydrateError::new_spanned(
            "field_stress requires choke_a_col or saturating_flux.choke_output_col",
            Some(property.key.span.clone()),
        )
    })?;
    let choke_b = choke_b_col
        .or(flux_choke_col.map(|c| c.saturating_add(1)))
        .ok_or_else(|| {
            HydrateError::new_spanned(
                "field_stress requires choke_b_col or saturating_flux.choke_output_col",
                Some(property.key.span.clone()),
            )
        })?;

    Ok((
        choke_a,
        choke_b,
        require_field(operator, "operator", Some(property.key.span.clone()))?,
        require_field(output_col, "output_col", Some(property.key.span.clone()))?,
    ))
}

fn parse_threshold_feedstock_block(
    property: &RawProperty,
) -> Result<FirstSliceCommitmentSpec, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`threshold_feedstock` must be a block",
            Some(property.key.span.clone()),
        ));
    };

    let mut parent_slot = None;
    let mut urgency_col = None;
    let mut threshold = None;
    let mut direction = None;
    let mut event_kind = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "parent_slot" => parent_slot = Some(read_scalar_u32(field, "parent_slot")?),
            "urgency_col" => urgency_col = Some(read_scalar_u32(field, "urgency_col")?),
            "threshold" => threshold = Some(read_scalar_f32(field, "threshold")?),
            "direction" => {
                let text = read_scalar_text(field, "direction")?;
                direction = Some(match text.as_str() {
                    "upward" => FirstSliceCommitmentDirectionSpec::Upward,
                    "downward" => {
                        return Err(HydrateError::new_spanned(
                            "`threshold_feedstock.direction` must be `upward` in first-slice v1",
                            Some(field.key.span.clone()),
                        ));
                    }
                    other => {
                        return Err(HydrateError::new_spanned(
                            format!(
                                "`threshold_feedstock.direction` must be `upward`, got `{other}`"
                            ),
                            Some(field.key.span.clone()),
                        ));
                    }
                });
            }
            "event_kind" => event_kind = Some(read_scalar_u32(field, "event_kind")?),
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported threshold_feedstock field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    Ok(FirstSliceCommitmentSpec {
        source_formula_class: "field_urgency".into(),
        parent_slot: require_field(parent_slot, "parent_slot", Some(property.key.span.clone()))?,
        urgency_col: require_field(urgency_col, "urgency_col", Some(property.key.span.clone()))?,
        threshold: require_field(threshold, "threshold", Some(property.key.span.clone()))?,
        direction: require_field(direction, "direction", Some(property.key.span.clone()))?,
        event_kind: require_field(event_kind, "event_kind", Some(property.key.span.clone()))?,
        effect: None,
    })
}

fn parse_parent_formula_block(
    property: &RawProperty,
) -> Result<RegionFieldFormulaBindingSpec, HydrateError> {
    let RawValue::Block(block) = &property.value else {
        return Err(HydrateError::new_spanned(
            "`parent_formula` must be a block",
            Some(property.key.span.clone()),
        ));
    };

    let mut formula_class = None;
    let mut weight_pressure = None;
    let mut weight_resource = None;

    for field in &block.properties {
        match field.key.text.as_str() {
            "formula_class" => formula_class = Some(read_scalar_text(field, "formula_class")?),
            "weight_pressure" => {
                weight_pressure = Some(read_scalar_f32(field, "weight_pressure")?);
            }
            "weight_resource" => {
                weight_resource = Some(read_scalar_f32(field, "weight_resource")?);
            }
            other => {
                return Err(HydrateError::new_spanned(
                    format!("unsupported parent_formula field `{other}`"),
                    Some(field.key.span.clone()),
                ));
            }
        }
    }

    Ok(RegionFieldFormulaBindingSpec {
        formula_class: require_field(
            formula_class,
            "formula_class",
            Some(property.key.span.clone()),
        )?,
        tree_id: None,
        weight_pressure,
        weight_resource,
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
