//! CPU oracle for resource economy transfer/recipe/emission parity (driver/test only).

use simthing_core::{
    rebuild_conjunctive_recipe_ops, rebuild_discrete_transfer_ops, AccumulatorOpBuilderError,
    ConjunctiveRecipeRegistration, DiscreteTransferRegistration,
};
use simthing_gpu::{
    execute_ops_cpu, EmissionFormula, EmissionRecord, EmissionRegistration,
};

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
        execute_ops_cpu(flat, &ops, band, n_dims).map_err(|e| ResourceEconomyOracleError::Cpu(e.to_string()))?;
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
    emissions
        .iter()
        .map(|emission| {
            Ok(EmissionRecord {
                reg_idx: emission.reg_idx,
                emit_count: expected_emission_emit_count(flat, n_dims, emission)?,
            })
        })
        .collect()
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

    #[test]
    fn oracle_discrete_transfer_conserves_pair_total() {
        let transfers = vec![simthing_core::DiscreteTransferRegistration {
            source_slot: 0,
            source_col: 0,
            target_slot: 1,
            target_col: 0,
            amount: 1.0,
            order_band: 0,
        }];
        let mut flat = vec![5.0, 0.0];
        let before = flat.clone();
        run_transfer_recipe_cpu_oracle(&mut flat, 1, &transfers, &[]).unwrap();
        assert_discrete_transfer_conserved(&before, &flat, 1, (0, 0), (1, 0));
        assert_eq!(flat[0].to_bits(), 4.0_f32.to_bits());
        assert_eq!(flat[1].to_bits(), 1.0_f32.to_bits());
    }

    #[test]
    fn oracle_recipe_debits_inputs_and_credits_target() {
        let recipes = vec![ConjunctiveRecipeRegistration {
            inputs: vec![
                ConjunctiveRecipeInput {
                    slot: 0,
                    col: 0,
                    unit_cost: 1.0,
                },
                ConjunctiveRecipeInput {
                    slot: 0,
                    col: 1,
                    unit_cost: 2.0,
                },
            ],
            target_slot: 0,
            target_col: 2,
            throttle_hint_max_per_tick: 99,
        }];
        let mut flat = vec![10.0, 8.0, 0.0];
        run_transfer_recipe_cpu_oracle(&mut flat, 3, &[], &recipes).unwrap();
        assert_eq!(flat[0].to_bits(), 6.0_f32.to_bits());
        assert_eq!(flat[1].to_bits(), 0.0_f32.to_bits());
        assert_eq!(flat[2].to_bits(), 4.0_f32.to_bits());
    }
}
