//! Tick/boundary execution seam for driver-compiled AccumulatorOp plans.

use simthing_core::{is_exact_integer_f32, CompiledAccumulatorOpPlan};
use simthing_gpu::execute_ops_cpu;
use simthing_gpu::{set_debug_readback_allowed, AccumulatorOpSession, GpuContext};

#[derive(Debug, thiserror::Error)]
pub enum SimTickError {
    #[error("input length {actual} does not match slot_count {expected}")]
    InvalidInputLength { expected: usize, actual: usize },
    #[error("non-exact integer f32 input at index {index}: {value}")]
    NonExactIntegerInput { index: usize, value: f32 },
    #[error("CPU oracle execution failed: {0}")]
    Oracle(String),
    #[error("GPU adapter unavailable: {0}")]
    GpuUnavailable(String),
    #[error("GPU accumulator execution failed: {0}")]
    GpuAccumulator(String),
    #[error("GPU readback failed: {0}")]
    Readback(String),
}

/// Backend selection for sim-owned AccumulatorOp plan tick execution.
pub enum AccumulatorTickBackend<'a> {
    CpuOracle,
    Gpu(&'a GpuContext),
}

fn validate_accumulator_plan_inputs(
    plan: &CompiledAccumulatorOpPlan,
    input_values: &[f32],
) -> Result<(), SimTickError> {
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
    Ok(())
}

fn seed_value_grid(plan: &CompiledAccumulatorOpPlan, input_values: &[f32]) -> Vec<f32> {
    let slot_count = plan.slot_count as usize;
    let n_dims = plan.n_dims as usize;
    let input_col = plan.input_channel.0 as usize;
    let mut values = vec![0.0f32; slot_count * n_dims];
    for (slot, &input) in input_values.iter().enumerate() {
        values[slot * n_dims + input_col] = input;
    }
    values
}

fn extract_output_channel(plan: &CompiledAccumulatorOpPlan, values: &[f32]) -> Vec<f32> {
    let slot_count = plan.slot_count as usize;
    let n_dims = plan.n_dims as usize;
    let output_col = plan.output_channel.0 as usize;
    (0..slot_count)
        .map(|slot| values[slot * n_dims + output_col])
        .collect()
}

/// Execute a driver-compiled AccumulatorOp plan under sim tick ownership (CPU oracle path).
///
/// Seeds `input_channel` from `input_values`, zeroes `output_channel`, runs the plan, and returns
/// per-slot output scalars from `output_channel`.
pub fn execute_accumulator_plan_tick_cpu(
    plan: &CompiledAccumulatorOpPlan,
    input_values: &[f32],
) -> Result<Vec<f32>, SimTickError> {
    validate_accumulator_plan_inputs(plan, input_values)?;
    let mut values = seed_value_grid(plan, input_values);

    execute_ops_cpu(&mut values, &plan.ops, 0, plan.n_dims)
        .map_err(|err| SimTickError::Oracle(err.to_string()))?;

    Ok(extract_output_channel(plan, &values))
}

/// Execute a driver-compiled AccumulatorOp plan under sim tick ownership (GPU backend).
///
/// Reuses `AccumulatorOpSession` / AO-WGSL-0. Output is projection/cache — not scenario authority.
pub fn execute_accumulator_plan_tick_gpu(
    ctx: &GpuContext,
    plan: &CompiledAccumulatorOpPlan,
    input_values: &[f32],
) -> Result<Vec<f32>, SimTickError> {
    validate_accumulator_plan_inputs(plan, input_values)?;
    let values = seed_value_grid(plan, input_values);

    set_debug_readback_allowed(true);
    let mut session = AccumulatorOpSession::new(ctx, plan.slot_count, plan.n_dims);
    session.upload_values(ctx, &values);
    session
        .upload_ops_resolving_input_lists(ctx, &plan.ops)
        .map_err(|err| SimTickError::GpuAccumulator(err.to_string()))?;
    session
        .tick(ctx, 0)
        .map_err(|err| SimTickError::GpuAccumulator(err.to_string()))?;
    let readback = session
        .readback_full(ctx)
        .map_err(|err| SimTickError::Readback(err.to_string()))?;

    Ok(extract_output_channel(plan, &readback))
}

/// Execute a driver-compiled plan with an explicit CPU or GPU backend.
pub fn execute_accumulator_plan_tick_with_backend(
    backend: AccumulatorTickBackend<'_>,
    plan: &CompiledAccumulatorOpPlan,
    input_values: &[f32],
) -> Result<Vec<f32>, SimTickError> {
    match backend {
        AccumulatorTickBackend::CpuOracle => execute_accumulator_plan_tick_cpu(plan, input_values),
        AccumulatorTickBackend::Gpu(ctx) => {
            execute_accumulator_plan_tick_gpu(ctx, plan, input_values)
        }
    }
}

/// Blocking GPU context for sim-owned accumulator tick proofs and callers without an existing context.
pub fn gpu_context_blocking() -> Result<GpuContext, SimTickError> {
    GpuContext::new_blocking().map_err(|err| SimTickError::GpuUnavailable(err.to_string()))
}
