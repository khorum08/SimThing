//! Tick/boundary execution seam for driver-compiled AccumulatorOp plans.

use simthing_core::{is_exact_integer_f32, CompiledAccumulatorOpPlan};
use simthing_gpu::execute_ops_cpu;

#[derive(Debug, thiserror::Error)]
pub enum SimTickError {
    #[error("input length {actual} does not match slot_count {expected}")]
    InvalidInputLength { expected: usize, actual: usize },
    #[error("non-exact integer f32 input at index {index}: {value}")]
    NonExactIntegerInput { index: usize, value: f32 },
    #[error("CPU oracle execution failed: {0}")]
    Oracle(String),
}

/// Execute a driver-compiled AccumulatorOp plan under sim tick ownership (CPU oracle path).
///
/// Seeds `input_channel` from `input_values`, zeroes `output_channel`, runs the plan, and returns
/// per-slot output scalars from `output_channel`.
pub fn execute_accumulator_plan_tick_cpu(
    plan: &CompiledAccumulatorOpPlan,
    input_values: &[f32],
) -> Result<Vec<f32>, SimTickError> {
    let slot_count = plan.slot_count as usize;
    if input_values.len() != slot_count {
        return Err(SimTickError::InvalidInputLength {
            expected: slot_count,
            actual: input_values.len(),
        });
    }
    for (index, &value) in input_values.iter().enumerate() {
        if !is_exact_integer_f32(value) {
            return Err(SimTickError::NonExactIntegerInput { index, value });
        }
    }

    let n_dims = plan.n_dims as usize;
    let input_col = plan.input_channel.0 as usize;
    let output_col = plan.output_channel.0 as usize;
    let mut values = vec![0.0f32; slot_count * n_dims];
    for (slot, &input) in input_values.iter().enumerate() {
        values[slot * n_dims + input_col] = input;
    }

    execute_ops_cpu(&mut values, &plan.ops, 0, plan.n_dims)
        .map_err(|err| SimTickError::Oracle(err.to_string()))?;

    Ok((0..slot_count)
        .map(|slot| values[slot * n_dims + output_col])
        .collect())
}
