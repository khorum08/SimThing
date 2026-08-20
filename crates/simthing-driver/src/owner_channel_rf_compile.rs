//! OWNER-CHANNEL-INTRINSIC-0 — AccumulatorOp lowering for generalized owner scopes.

use simthing_core::owner_channel::resolve_owners_in_order;
use simthing_core::{CompiledAccumulatorOpPlan, SimThing, StructuralScalarChannel};
use simthing_gpu::GpuContext;
use simthing_sim::{execute_accumulator_plan_tick_cpu, execute_accumulator_plan_tick_gpu};
use simthing_spec::{
    reduce_owner_channel_rf, OwnerChannelRfError, OwnerChannelRfOwnAggregate,
    OwnerChannelRfReduceUpReport, OwnerChannelScopeKey, PersistentRfLayout,
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
    pub layout: PersistentRfLayout,
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
    let stamped =
        reduce_owner_channel_rf(root, own_aggregates, simthing_core::GenerationStamp::new(0))?;
    let reduce_up_report = stamped.into_product();
    let mut layout = PersistentRfLayout::new();
    for (_, owner) in resolve_owners_in_order(root)
        .map_err(|e| OwnerChannelRfGpuProofError::Execution(e.to_string()))?
    {
        layout.intern_owner(&owner);
    }
    let mut ordered_buckets = reduce_up_report.buckets.clone();
    ordered_buckets.sort_by(|a, b| {
        let a_id = layout
            .interner
            .id_of(&a.scope.owner_ref)
            .expect("tree-walk interned owner");
        let b_id = layout
            .interner
            .id_of(&b.scope.owner_ref)
            .expect("tree-walk interned owner");
        a_id.cmp(&b_id)
            .then_with(|| a.scope.resource_key.cmp(&b.scope.resource_key))
            .then_with(|| a.scope.scope_id.cmp(&b.scope.scope_id))
    });
    let logical_slots: Vec<_> = ordered_buckets
        .iter()
        .enumerate()
        .map(|(i, _)| simthing_core::SlotIndex::new(i as u32))
        .collect();
    layout.assign_from_buckets(&ordered_buckets, &logical_slots);
    let bucket_plans = ordered_buckets
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
        layout,
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
    let canonical_bucket_ordering = intern_layout_order_holds(plan);
    if !canonical_bucket_ordering {
        return Err(OwnerChannelRfGpuProofError::NonCanonicalBucketOrder);
    }

    for (bucket_index, compiled) in plan.bucket_plans.iter().enumerate() {
        let bucket = plan
            .reduce_up_report
            .buckets
            .iter()
            .find(|b| b.scope == compiled.scope)
            .ok_or(OwnerChannelRfGpuProofError::NonCanonicalBucketOrder)?;
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

fn intern_layout_order_holds(plan: &OwnerChannelRfGpuProofPlan) -> bool {
    plan.bucket_plans.windows(2).all(|pair| {
        let Some(a) = plan.layout.interner.id_of(&pair[0].scope.owner_ref) else {
            return false;
        };
        let Some(b) = plan.layout.interner.id_of(&pair[1].scope.owner_ref) else {
            return false;
        };
        (a, &pair[0].scope.resource_key, &pair[0].scope.scope_id)
            <= (b, &pair[1].scope.resource_key, &pair[1].scope.scope_id)
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
