//! BH-2B: admission for generic W impedance composition operator.

use crate::error::SpecError;
use crate::spec::w_impedance_compose::{WImpedanceComposeSpec, W_IMPEDANCE_COMPOSE_MAX_PROFILES};

#[derive(Clone, Debug, PartialEq)]
pub struct CompiledWImpedanceComposeProfile {
    pub weight_a: f32,
    pub weight_b: f32,
    pub output_w_col: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompiledWImpedanceCompose {
    pub width: u32,
    pub height: u32,
    pub n_dims: u32,
    pub base_w_col: u32,
    pub choke_a_col: u32,
    pub choke_b_col: u32,
    pub profiles: Vec<CompiledWImpedanceComposeProfile>,
}

fn admission_err(reason: impl Into<String>) -> SpecError {
    SpecError::WImpedanceComposeAdmission {
        reason: reason.into(),
    }
}

fn validate_finite_weight(name: &str, value: f32) -> Result<(), SpecError> {
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

pub fn compile_w_impedance_compose_preview(
    spec: &WImpedanceComposeSpec,
) -> Result<CompiledWImpedanceCompose, SpecError> {
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

    validate_column("base_w_col", spec.base_w_col, spec.n_dims)?;
    validate_column("choke_a_col", spec.choke_a_col, spec.n_dims)?;
    validate_column("choke_b_col", spec.choke_b_col, spec.n_dims)?;

    if spec.profiles.is_empty() {
        return Err(admission_err("profiles must contain at least one entry"));
    }
    if spec.profiles.len() > W_IMPEDANCE_COMPOSE_MAX_PROFILES {
        return Err(admission_err(format!(
            "profiles exceeds max {} profiles",
            W_IMPEDANCE_COMPOSE_MAX_PROFILES
        )));
    }

    let mut compiled_profiles = Vec::with_capacity(spec.profiles.len());
    let mut output_cols = Vec::with_capacity(spec.profiles.len());
    for (i, profile) in spec.profiles.iter().enumerate() {
        validate_finite_weight(&format!("profiles[{i}].weight_a"), profile.weight_a)?;
        validate_finite_weight(&format!("profiles[{i}].weight_b"), profile.weight_b)?;
        validate_column(
            &format!("profiles[{i}].output_w_col"),
            profile.output_w_col,
            spec.n_dims,
        )?;
        if output_cols.contains(&profile.output_w_col) {
            return Err(admission_err(format!(
                "duplicate output_w_col {} across profiles",
                profile.output_w_col
            )));
        }
        output_cols.push(profile.output_w_col);
        compiled_profiles.push(CompiledWImpedanceComposeProfile {
            weight_a: profile.weight_a,
            weight_b: profile.weight_b,
            output_w_col: profile.output_w_col,
        });
    }

    let mut all_cols = vec![spec.base_w_col, spec.choke_a_col, spec.choke_b_col];
    all_cols.extend(spec.profiles.iter().map(|p| p.output_w_col));
    let unique: std::collections::BTreeSet<_> = all_cols.iter().copied().collect();
    if unique.len() != all_cols.len() {
        return Err(admission_err(
            "column aliasing: base_w_col, choke_a_col, choke_b_col, and output_w_col must be distinct",
        ));
    }

    Ok(CompiledWImpedanceCompose {
        width: spec.width,
        height: spec.height,
        n_dims: spec.n_dims,
        base_w_col: spec.base_w_col,
        choke_a_col: spec.choke_a_col,
        choke_b_col: spec.choke_b_col,
        profiles: compiled_profiles,
    })
}
