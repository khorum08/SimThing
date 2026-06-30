//! CPU-oracle twin of kernel EmitEvent readback for driver parity burn-in.

use crate::sealed::EmissionRecord;

#[derive(Clone, Debug, thiserror::Error, PartialEq, Eq)]
pub enum EmissionOracleError {
    #[error("EvalEml emission formula requires EML registry")]
    MissingEmlRegistry,
}

/// Formula kind for CPU emission oracle (mirrors gpu `EmissionFormula` without gpu deps).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EmissionOracleFormula {
    IdentityFloor,
    Constant { value: f32 },
    EvalEml,
}

/// One emission registration for CPU oracle parity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EmissionOracleRegistration {
    pub reg_idx: u32,
    pub source_slot: u32,
    pub source_col: u32,
    pub formula: EmissionOracleFormula,
}

fn emission_cell_index(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

/// CPU-oracle twin of kernel EmitEvent readback for driver parity burn-in.
pub fn cpu_oracle_emission_records(
    flat: &[f32],
    n_dims: u32,
    emissions: &[EmissionOracleRegistration],
) -> Result<Vec<EmissionRecord>, EmissionOracleError> {
    emissions
        .iter()
        .map(|emission| {
            let idx = emission_cell_index(emission.source_slot, emission.source_col, n_dims);
            let source = flat[idx];
            let emit_count = match emission.formula {
                EmissionOracleFormula::IdentityFloor => source.floor().max(0.0) as u32,
                EmissionOracleFormula::Constant { value } => u32::from(source >= value),
                EmissionOracleFormula::EvalEml => {
                    return Err(EmissionOracleError::MissingEmlRegistry);
                }
            };
            Ok(EmissionRecord::from_cpu_oracle(
                emission.reg_idx,
                emit_count,
            ))
        })
        .collect()
}
