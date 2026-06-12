//! BH-2S: admission for generic GPU stress field algebra.

use std::collections::BTreeSet;

use crate::error::SpecError;
use crate::spec::stress_compose::{
    StressComposeSpec, StressOperatorSpec, STRESS_COMPOSE_MAX_INPUT_FIELDS,
    STRESS_COMPOSE_MAX_PROFILES,
};

#[derive(Clone, Debug, PartialEq)]
pub struct CompiledStressComposeProfile {
    pub operator_kind: u32,
    pub weight_a: f32,
    pub weight_b: f32,
    pub output_col: u32,
    pub choke_now_col: u32,
    pub choke_prev_col: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompiledStressCompose {
    pub width: u32,
    pub height: u32,
    pub n_dims: u32,
    pub choke_a_col: u32,
    pub choke_b_col: u32,
    pub profiles: Vec<CompiledStressComposeProfile>,
}

fn admission_err(reason: impl Into<String>) -> SpecError {
    SpecError::StressComposeAdmission {
        reason: reason.into(),
    }
}

fn validate_finite(name: &str, value: f32) -> Result<(), SpecError> {
    if !value.is_finite() {
        return Err(admission_err(format!(
            "{name} must be finite (got {value})"
        )));
    }
    Ok(())
}

fn validate_column(name: &str, col: u32, n_dims: u32) -> Result<(), SpecError> {
    if n_dims == 0 {
        return Err(admission_err("n_dims must be > 0"));
    }
    if col >= n_dims {
        return Err(admission_err(format!(
            "{name} column {col} out of range for n_dims {n_dims}"
        )));
    }
    Ok(())
}

pub fn compile_stress_compose_preview(
    spec: &StressComposeSpec,
) -> Result<CompiledStressCompose, SpecError> {
    if spec.width == 0 || spec.height == 0 {
        return Err(admission_err(format!(
            "width/height must be > 0 (got {}x{})",
            spec.width, spec.height
        )));
    }
    let cells = spec.width as u64 * spec.height as u64;
    if cells > u32::MAX as u64 {
        return Err(admission_err("width * height overflows u32 cell count"));
    }
    let values_len = cells * spec.n_dims as u64;
    if values_len > u32::MAX as u64 {
        return Err(admission_err(
            "width * height * n_dims overflows representable flat buffer length",
        ));
    }

    validate_column("choke_a_col", spec.choke_a_col, spec.n_dims)?;
    validate_column("choke_b_col", spec.choke_b_col, spec.n_dims)?;

    if spec.profiles.is_empty() {
        return Err(admission_err("profiles must contain at least one entry"));
    }
    if spec.profiles.len() > STRESS_COMPOSE_MAX_PROFILES {
        return Err(admission_err(format!(
            "profiles exceeds max {} profiles",
            STRESS_COMPOSE_MAX_PROFILES
        )));
    }

    let mut input_fields = BTreeSet::from([spec.choke_a_col, spec.choke_b_col]);
    let mut output_cols = BTreeSet::new();
    let mut compiled_profiles = Vec::with_capacity(spec.profiles.len());

    for (i, profile) in spec.profiles.iter().enumerate() {
        validate_column(
            &format!("profiles[{i}].output_col"),
            profile.output_col,
            spec.n_dims,
        )?;
        if !output_cols.insert(profile.output_col) {
            return Err(admission_err(format!(
                "duplicate output_col {} across profiles",
                profile.output_col
            )));
        }

        let compiled = match &profile.operator {
            StressOperatorSpec::Overlap => CompiledStressComposeProfile {
                operator_kind: crate::spec::stress_compose::STRESS_OP_OVERLAP,
                weight_a: 0.0,
                weight_b: 0.0,
                output_col: profile.output_col,
                choke_now_col: spec.choke_a_col,
                choke_prev_col: 0,
            },
            StressOperatorSpec::Mismatch => CompiledStressComposeProfile {
                operator_kind: crate::spec::stress_compose::STRESS_OP_MISMATCH,
                weight_a: 0.0,
                weight_b: 0.0,
                output_col: profile.output_col,
                choke_now_col: spec.choke_a_col,
                choke_prev_col: 0,
            },
            StressOperatorSpec::Weighted { weight_a, weight_b } => {
                validate_finite(&format!("profiles[{i}].weight_a"), *weight_a)?;
                validate_finite(&format!("profiles[{i}].weight_b"), *weight_b)?;
                CompiledStressComposeProfile {
                    operator_kind: crate::spec::stress_compose::STRESS_OP_WEIGHTED,
                    weight_a: *weight_a,
                    weight_b: *weight_b,
                    output_col: profile.output_col,
                    choke_now_col: spec.choke_a_col,
                    choke_prev_col: 0,
                }
            }
            StressOperatorSpec::Velocity {
                choke_now_col,
                choke_prev_col,
            } => {
                validate_column(
                    &format!("profiles[{i}].choke_now_col"),
                    *choke_now_col,
                    spec.n_dims,
                )?;
                validate_column(
                    &format!("profiles[{i}].choke_prev_col"),
                    *choke_prev_col,
                    spec.n_dims,
                )?;
                if choke_now_col == choke_prev_col {
                    return Err(admission_err(format!(
                        "profiles[{i}] velocity choke_now_col and choke_prev_col must differ"
                    )));
                }
                input_fields.insert(*choke_now_col);
                input_fields.insert(*choke_prev_col);
                CompiledStressComposeProfile {
                    operator_kind: crate::spec::stress_compose::STRESS_OP_VELOCITY,
                    weight_a: 0.0,
                    weight_b: 0.0,
                    output_col: profile.output_col,
                    choke_now_col: *choke_now_col,
                    choke_prev_col: *choke_prev_col,
                }
            }
        };
        compiled_profiles.push(compiled);
    }

    if input_fields.len() > STRESS_COMPOSE_MAX_INPUT_FIELDS {
        return Err(admission_err(format!(
            "input field fan-in {} exceeds max {} distinct columns",
            input_fields.len(),
            STRESS_COMPOSE_MAX_INPUT_FIELDS
        )));
    }

    let mut all_cols: Vec<u32> = input_fields.iter().copied().collect();
    all_cols.extend(output_cols.iter().copied());
    let unique: BTreeSet<_> = all_cols.iter().copied().collect();
    if unique.len() != all_cols.len() {
        return Err(admission_err(
            "column aliasing: input choke columns and output_col must be distinct",
        ));
    }

    Ok(CompiledStressCompose {
        width: spec.width,
        height: spec.height,
        n_dims: spec.n_dims,
        choke_a_col: spec.choke_a_col,
        choke_b_col: spec.choke_b_col,
        profiles: compiled_profiles,
    })
}
