//! OWNER-CHANNEL-INTRINSIC-0 — AccumulatorOp lowering for generalized owner scopes.

use simthing_core::{CompiledAccumulatorOpPlan, SimThing, StructuralScalarChannel};
use simthing_gpu::GpuContext;
use simthing_sim::{execute_accumulator_plan_tick_cpu, execute_accumulator_plan_tick_gpu};
use simthing_spec::{
    reduce_owner_channel_rf, OwnerChannelRfError, OwnerChannelRfOwnAggregate,
    OwnerChannelRfReduceUpReport, OwnerChannelScopeKey,
};

use crate::owner_silo_accumulator_compile::compile_participant_channel_sum_plan;

#[derive(Debug, Clone, PartialEq)]
pub struct OwnerChannelRfBucketAccumulatorPlan {
    pub scope: OwnerChannelScopeKey,
    pub source_row_indices: Vec<usize>,
    pub surplus_plan: CompiledAccumulatorOpPlan,
    pub deficit_plan: CompiledAccumulatorOpPlan,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OwnerChannelRfGpuProofPlan {
    pub reduce_up_report: OwnerChannelRfReduceUpReport,
    pub bucket_plans: Vec<OwnerChannelRfBucketAccumulatorPlan>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerChannelRfGpuParityReport {
    pub bucket_count: u32,
    pub canonical_bucket_ordering: bool,
    pub cpu_gpu_bit_exact: bool,
}

#[derive(Debug)]
pub enum OwnerChannelRfGpuProofError {
    ReduceUp(OwnerChannelRfError),
    Execution(String),
    NonCanonicalBucketOrder,
    CpuGpuMismatch {
        bucket_index: usize,
        channel: &'static str,
        expected: u32,
        cpu_bits: u32,
        gpu_bits: u32,
    },
}

impl From<OwnerChannelRfError> for OwnerChannelRfGpuProofError {
    fn from(value: OwnerChannelRfError) -> Self {
        Self::ReduceUp(value)
    }
}

/// Compile one existing generic sum plan per canonical owner/resource/scope bucket.
pub fn compile_owner_channel_rf_gpu_proof_plan(
    root: &SimThing,
    own_aggregates: &[OwnerChannelRfOwnAggregate],
) -> Result<OwnerChannelRfGpuProofPlan, OwnerChannelRfGpuProofError> {
    let reduce_up_report = reduce_owner_channel_rf(root, own_aggregates)?;
    let bucket_plans = reduce_up_report
        .buckets
        .iter()
        .map(|bucket| OwnerChannelRfBucketAccumulatorPlan {
            scope: bucket.scope.clone(),
            source_row_indices: bucket.source_row_indices.clone(),
            surplus_plan: compile_participant_channel_sum_plan(
                bucket.participant_count,
                StructuralScalarChannel::INPUT,
                StructuralScalarChannel::OUTPUT,
            ),
            deficit_plan: compile_participant_channel_sum_plan(
                bucket.participant_count,
                StructuralScalarChannel::INPUT,
                StructuralScalarChannel::OUTPUT,
            ),
        })
        .collect();
    Ok(OwnerChannelRfGpuProofPlan {
        reduce_up_report,
        bucket_plans,
    })
}

pub fn owner_channel_rf_bucket_surplus_tick_inputs(
    plan: &OwnerChannelRfGpuProofPlan,
    bucket: &OwnerChannelRfBucketAccumulatorPlan,
) -> Vec<f32> {
    let mut values = vec![0.0; bucket.surplus_plan.slot_count as usize];
    for (slot, &row_index) in bucket.source_row_indices.iter().enumerate() {
        values[slot] = plan.reduce_up_report.stead.own_aggregates[row_index].surplus as f32;
    }
    values
}

pub fn owner_channel_rf_bucket_deficit_tick_inputs(
    plan: &OwnerChannelRfGpuProofPlan,
    bucket: &OwnerChannelRfBucketAccumulatorPlan,
) -> Vec<f32> {
    let mut values = vec![0.0; bucket.deficit_plan.slot_count as usize];
    for (slot, &row_index) in bucket.source_row_indices.iter().enumerate() {
        values[slot] = plan.reduce_up_report.stead.own_aggregates[row_index].deficit as f32;
    }
    values
}

pub fn owner_channel_rf_bucket_aggregate_slot(
    bucket: &OwnerChannelRfBucketAccumulatorPlan,
) -> usize {
    bucket.source_row_indices.len()
}

/// Execute every bucket through the CPU oracle and GPU adapter and require bit-exact totals.
pub fn prove_owner_channel_rf_cpu_gpu_parity(
    ctx: &GpuContext,
    plan: &OwnerChannelRfGpuProofPlan,
) -> Result<OwnerChannelRfGpuParityReport, OwnerChannelRfGpuProofError> {
    let canonical_bucket_ordering = plan
        .reduce_up_report
        .buckets
        .windows(2)
        .all(|pair| pair[0].scope < pair[1].scope);
    if !canonical_bucket_ordering {
        return Err(OwnerChannelRfGpuProofError::NonCanonicalBucketOrder);
    }

    for (bucket_index, (bucket, compiled)) in plan
        .reduce_up_report
        .buckets
        .iter()
        .zip(&plan.bucket_plans)
        .enumerate()
    {
        prove_channel(
            ctx,
            bucket_index,
            "surplus",
            bucket.surplus_total,
            &compiled.surplus_plan,
            &owner_channel_rf_bucket_surplus_tick_inputs(plan, compiled),
            owner_channel_rf_bucket_aggregate_slot(compiled),
        )?;
        prove_channel(
            ctx,
            bucket_index,
            "deficit",
            bucket.deficit_total,
            &compiled.deficit_plan,
            &owner_channel_rf_bucket_deficit_tick_inputs(plan, compiled),
            owner_channel_rf_bucket_aggregate_slot(compiled),
        )?;
    }

    Ok(OwnerChannelRfGpuParityReport {
        bucket_count: plan.reduce_up_report.bucket_count,
        canonical_bucket_ordering,
        cpu_gpu_bit_exact: true,
    })
}

fn prove_channel(
    ctx: &GpuContext,
    bucket_index: usize,
    channel: &'static str,
    expected: u32,
    compiled: &CompiledAccumulatorOpPlan,
    inputs: &[f32],
    aggregate_slot: usize,
) -> Result<(), OwnerChannelRfGpuProofError> {
    let cpu = execute_accumulator_plan_tick_cpu(compiled, inputs)
        .map_err(|error| OwnerChannelRfGpuProofError::Execution(error.to_string()))?;
    let gpu = execute_accumulator_plan_tick_gpu(ctx, compiled, inputs)
        .map_err(|error| OwnerChannelRfGpuProofError::Execution(error.to_string()))?;
    let cpu_bits = cpu[aggregate_slot].to_bits();
    let gpu_bits = gpu[aggregate_slot].to_bits();
    if cpu_bits != gpu_bits || cpu[aggregate_slot] != expected as f32 {
        return Err(OwnerChannelRfGpuProofError::CpuGpuMismatch {
            bucket_index,
            channel,
            expected,
            cpu_bits,
            gpu_bits,
        });
    }
    Ok(())
}
