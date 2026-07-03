//! CPU oracle for resource economy transfer/recipe/emission parity (driver/test only).

use simthing_core::{
    rebuild_conjunctive_recipe_ops, rebuild_discrete_transfer_ops, AccumulatorOpBuilderError,
    ConjunctiveRecipeRegistration, DiscreteTransferRegistration,
};
use simthing_gpu::{execute_ops_cpu, EmissionFormula, EmissionRecord, EmissionRegistration};

#[derive(Debug, thiserror::Error)]
pub enum ResourceEconomyOracleError {
    #[error(transparent)]
    Builder(#[from] AccumulatorOpBuilderError),
    #[error("cpu oracle failed: {0}")]
    Cpu(String),
    #[error(transparent)]
    EmissionPlan(#[from] simthing_gpu::EmissionPlanError),
    #[error("unsupported emission formula for burn-in oracle")]
    UnsupportedEmissionFormula,
}

fn max_transfer_band(
    transfers: &[DiscreteTransferRegistration],
    recipes: &[ConjunctiveRecipeRegistration],
) -> u32 {
    transfers
        .iter()
        .map(|t| t.order_band)
        .chain(recipes.iter().map(|_| 0u32))
        .max()
        .unwrap_or(0)
}

/// Run discrete transfer + conjunctive recipe ops on a flat values buffer for one tick.
pub fn run_transfer_recipe_cpu_oracle(
    flat: &mut [f32],
    n_dims: u32,
    transfers: &[DiscreteTransferRegistration],
    recipes: &[ConjunctiveRecipeRegistration],
) -> Result<(), ResourceEconomyOracleError> {
    let mut ops = rebuild_discrete_transfer_ops(transfers)?;
    ops.extend(rebuild_conjunctive_recipe_ops(recipes)?);
    let max_band = max_transfer_band(transfers, recipes);
    for band in 0..=max_band {
        execute_ops_cpu(flat, &ops, band, n_dims)
            .map_err(|e| ResourceEconomyOracleError::Cpu(e.to_string()))?;
    }
    Ok(())
}

/// Expected emission emit_count for landed IdentityFloor / Constant formulas.
pub fn expected_emission_emit_count(
    flat: &[f32],
    n_dims: u32,
    emission: &EmissionRegistration,
) -> Result<u32, ResourceEconomyOracleError> {
    let idx = cell_index(emission.source_slot, emission.source_col, n_dims);
    let source = flat[idx];
    Ok(match &emission.formula {
        EmissionFormula::IdentityFloor => source.floor().max(0.0) as u32,
        EmissionFormula::Constant { value } => u32::from(source >= *value),
        EmissionFormula::EvalEml { .. } => {
            return Err(ResourceEconomyOracleError::UnsupportedEmissionFormula);
        }
    })
}

/// Run emission oracle for all registrations and return per-reg_idx emit counts.
pub fn run_emission_cpu_oracle(
    flat: &[f32],
    n_dims: u32,
    emissions: &[EmissionRegistration],
) -> Result<Vec<EmissionRecord>, ResourceEconomyOracleError> {
    simthing_gpu::cpu_oracle_emission_records(flat, n_dims, emissions)
        .map_err(ResourceEconomyOracleError::EmissionPlan)
}

/// Sum selected flat-buffer cells for conservation checks.
pub fn sum_cells(flat: &[f32], n_dims: u32, cells: &[(u32, u32)]) -> f32 {
    cells
        .iter()
        .map(|&(slot, col)| flat[cell_index(slot, col, n_dims)])
        .sum()
}

pub fn cell_index(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

/// Exact discrete transfer conservation: source+target total unchanged across one transfer op.
pub fn assert_discrete_transfer_conserved(
    before: &[f32],
    after: &[f32],
    n_dims: u32,
    source: (u32, u32),
    target: (u32, u32),
) {
    let before_sum = flat_at(before, source, n_dims) + flat_at(before, target, n_dims);
    let after_sum = flat_at(after, source, n_dims) + flat_at(after, target, n_dims);
    assert_eq!(
        before_sum.to_bits(),
        after_sum.to_bits(),
        "discrete transfer must conserve source+target total"
    );
}

fn flat_at(flat: &[f32], cell: (u32, u32), n_dims: u32) -> f32 {
    flat[cell_index(cell.0, cell.1, n_dims)]
}

#[cfg(test)]
mod tests {
    use simthing_core::ConjunctiveRecipeInput;

    use super::*;

}
