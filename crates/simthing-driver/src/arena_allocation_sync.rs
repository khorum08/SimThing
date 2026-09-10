//! E-11 boundary/session sync for resource-flow AccumulatorOp planning.

use simthing_core::{DimensionRegistry, EmlExpressionRegistry, GenerationStamp, SourceSpec};
use simthing_gpu::{build_governed_pairs, PackedAccumulatorUpload, WorldGpuState};

use crate::arena_allocation_plan::{
    append_residual_closure_ops, plan_arena_allocation, plan_arena_allocation_with_pressure,
    AllocationPlanError, ArenaAllocationPlan,
};
use crate::arena_hierarchy::{
    build_execution_plan, resolve_node_columns_for_property, ArenaExecutionPlan, HierarchyError,
};
use crate::arena_registry::ArenaRegistry;
use crate::child_share_eml::register_child_share_formula;
use thiserror::Error;

#[derive(Clone, Debug, Default)]
pub struct ResourceFlowSyncReport {
    pub arenas_planned: u32,
    pub total_ops: u32,
    pub n_bands: u32,
}

#[derive(Debug, Error)]
pub enum ResourceFlowSyncError {
    #[error(transparent)]
    Hierarchy(#[from] HierarchyError),
    #[error(transparent)]
    OpUpload(#[from] simthing_gpu::AccumulatorOpSessionError),
    #[error("resource-flow sparse input-list encoding failed: {0}")]
    SparseInputListEncoding(String),
    #[error(transparent)]
    Allocation(#[from] AllocationPlanError),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PreBandMap {
    gated_start: Option<u32>,
    need_stage: Option<u32>,
    need_eval: Option<u32>,
    arena_start: u32,
}

fn compose_pre_band_map(has_gated_rates: bool, has_need_bindings: bool) -> PreBandMap {
    let gated_width = u32::from(has_gated_rates);
    let need_stage = has_need_bindings.then_some(gated_width);
    let need_eval = has_need_bindings.then_some(gated_width + 1);
    PreBandMap {
        gated_start: has_gated_rates.then_some(0),
        need_stage,
        need_eval,
        arena_start: gated_width
            + if has_need_bindings {
                crate::need_binding::NEED_BINDING_PRE_BANDS
            } else {
                0
            },
    }
}

/// Plan and upload E-11 allocation ops through the sole production path.
///
/// When gated rates exist (CT-RF-EML-RATE-0), every arena op shifts up one
/// OrderBand and the effective-rate `EvalEML` ops occupy band 0, so the
/// intrinsic columns are recomputed from base/gate state before any reduce.
pub fn sync_resource_flow_accumulator(
    state: &mut WorldGpuState,
    registry: &DimensionRegistry,
    arena_registry: &ArenaRegistry,
    gated_rates: &[crate::gated_rates::ResolvedGatedRate],
    need_bindings: &[crate::need_binding::ResolvedNeedBinding],
    weight_overlay_targets: &std::collections::BTreeMap<
        simthing_core::SimPropertyId,
        std::collections::BTreeSet<simthing_core::SimThingId>,
    >,
) -> Result<ResourceFlowSyncReport, ResourceFlowSyncError> {
    sync_resource_flow_accumulator_with_pressure(
        state,
        registry,
        arena_registry,
        gated_rates,
        need_bindings,
        &[],
        &[],
        GenerationStamp::new(0),
        GenerationStamp::new(1),
        weight_overlay_targets,
    )
}

/// Build the per-flow-property installed-overlay AllocatorWeight target map
/// from the sealed runtime tree (relay 5626653653, completing DA ruling
/// 5626045761). Observation-only ids from the tree's narrow query; the ONE
/// canonical authority remains the installed overlay/program set itself.
pub fn collect_weight_overlay_targets(
    root: &simthing_sim::SimRuntimeTree,
    arena_registry: &ArenaRegistry,
) -> std::collections::BTreeMap<
    simthing_core::SimPropertyId,
    std::collections::BTreeSet<simthing_core::SimThingId>,
> {
    let role = simthing_core::SubFieldRole::Named("weight".into());
    let mut map = std::collections::BTreeMap::new();
    for arena in &arena_registry.arenas {
        let ids = root.overlay_transform_targets(arena.flow_property_id, &role);
        if !ids.is_empty() {
            map.insert(arena.flow_property_id, ids);
        }
    }
    map
}

/// Sole production sync with the existing typed ActionBand pressure products.
/// The caller supplies the session's current generation only so the binding
/// door can enforce N -> N+1; the recurring OrderBand plan owns no clock.
pub fn sync_resource_flow_accumulator_with_pressure(
    state: &mut WorldGpuState,
    registry: &DimensionRegistry,
    arena_registry: &ArenaRegistry,
    gated_rates: &[crate::gated_rates::ResolvedGatedRate],
    need_bindings: &[crate::need_binding::ResolvedNeedBinding],
    conserved_progress_bindings: &[crate::CompiledActionBandConservedProgressBinding],
    active_instances: &[crate::ActionBandActiveInstance],
    observed_generation: GenerationStamp,
    allocation_generation: GenerationStamp,
    weight_overlay_targets: &std::collections::BTreeMap<
        simthing_core::SimPropertyId,
        std::collections::BTreeSet<simthing_core::SimThingId>,
    >,
) -> Result<ResourceFlowSyncReport, ResourceFlowSyncError> {
    sync_resource_flow_accumulator_with_options(
        state,
        registry,
        arena_registry,
        gated_rates,
        need_bindings,
        conserved_progress_bindings,
        active_instances,
        observed_generation,
        allocation_generation,
        true,
        weight_overlay_targets,
    )
}

/// Same as [`sync_resource_flow_accumulator`] with RF-5A stage-projection control.
pub(crate) fn sync_resource_flow_accumulator_with_options(
    state: &mut WorldGpuState,
    registry: &DimensionRegistry,
    arena_registry: &ArenaRegistry,
    gated_rates: &[crate::gated_rates::ResolvedGatedRate],
    need_bindings: &[crate::need_binding::ResolvedNeedBinding],
    conserved_progress_bindings: &[crate::CompiledActionBandConservedProgressBinding],
    active_instances: &[crate::ActionBandActiveInstance],
    observed_generation: GenerationStamp,
    allocation_generation: GenerationStamp,
    include_need_stage_projections: bool,
    weight_overlay_targets: &std::collections::BTreeMap<
        simthing_core::SimPropertyId,
        std::collections::BTreeSet<simthing_core::SimThingId>,
    >,
) -> Result<ResourceFlowSyncReport, ResourceFlowSyncError> {
    if arena_registry.arenas.is_empty() {
        state.clear_resource_flow_accumulator();
        return Ok(ResourceFlowSyncReport::default());
    }

    let plan = build_execution_plan(registry, arena_registry)?;

    let mut eml_registry = EmlExpressionRegistry::new();
    for arena in &plan.arenas {
        let cols = resolve_node_columns_for_property(
            registry,
            arena.flow_property_id,
            &arena_registry.arenas[arena.arena_idx as usize].name,
        )?;
        register_child_share_formula(&mut eml_registry, cols).expect("child_share EML registers");
    }

    let governed = build_governed_pairs(registry);
    // INTERIOR-POLICY COMPOSITION LAW (DA admission 2026-09-10, relay
    // 5625360554): participants carrying an authored AllocatorWeight program
    // (the resolved need bindings — the ONE canonical policy authority) keep
    // that authored value as their upward participation weight; the recursive
    // upsweep aggregate feeds only their own child-share denominator. Neutral
    // participants remain pure pressure carriers, bit-identical to the
    // historical plan.
    // Relay 5626653653 completion: the policy-bearing set derives from EVERY
    // canonical authored AllocatorWeight authority — the resolved need
    // bindings AND active installed overlays targeting the flow property's
    // AllocatorWeight sub-field (ids supplied per property by the sealed
    // runtime tree's observation-only query). Classification only; the ONE
    // authority remains the installed program/overlay set itself.
    let authored_weight_slots_base: std::collections::BTreeSet<u32> =
        need_bindings.iter().map(|b| b.participant_slot).collect();
    let mut combined_cpu = Vec::new();
    let mut max_bands = 0u32;
    for arena in &plan.arenas {
        let mut authored_weight_slots = authored_weight_slots_base.clone();
        if let Some(overlay_ids) =
            weight_overlay_targets.get(&arena.flow_property_id)
        {
            for node in arena.iter_all() {
                if overlay_ids.contains(&node.hosted_simthing_id) {
                    authored_weight_slots.insert(node.participant_slot.raw());
                }
            }
        }
        let mut alloc = plan_arena_allocation_with_pressure(
            arena,
            &governed,
            state.n_slots,
            conserved_progress_bindings,
            active_instances,
            observed_generation,
            allocation_generation,
            &authored_weight_slots,
        )
        .map_err(|error| match error {
            AllocationPlanError::Hierarchy(error) => ResourceFlowSyncError::Hierarchy(error),
            error => ResourceFlowSyncError::Allocation(error),
        })?;
        append_residual_closure_ops(arena, &mut alloc.cpu_ops);
        max_bands = max_bands.max(alloc.n_bands);
        combined_cpu.extend(alloc.cpu_ops);
    }

    // RF-2A / RF-5A additive pre-bands (deterministic producer → stage → eval):
    //   gated-rate EvalEML @ 0..gated_pre-1
    //   need stage @ gated_pre + 0
    //   need EvalEML @ gated_pre + 1
    //   arena reduce/disburse @ gated_pre + need_pre + ...
    let band_map = compose_pre_band_map(!gated_rates.is_empty(), !need_bindings.is_empty());
    let pre_bands = band_map.arena_start;
    if pre_bands > 0 {
        for op in &mut combined_cpu {
            if let simthing_core::GateSpec::OrderBand(band) = op.gate {
                op.gate = simthing_core::GateSpec::OrderBand(band + pre_bands);
            }
        }
        let mut all_ops = Vec::new();
        if !gated_rates.is_empty() {
            all_ops.extend(crate::gated_rates::build_gated_rate_ops(
                gated_rates,
                &mut eml_registry,
            ));
        }
        if !need_bindings.is_empty() {
            all_ops.extend(crate::need_binding::build_need_binding_ops_with_options(
                need_bindings,
                &mut eml_registry,
                include_need_stage_projections,
                band_map.need_stage.expect("need stage band exists"),
            ));
        }
        all_ops.extend(combined_cpu);
        combined_cpu = all_ops;
        max_bands += pre_bands;
    }

    state.sync_resource_flow_ops_from_cpu(&combined_cpu, max_bands, &eml_registry)?;
    resolve_sparse_input_lists(state, &combined_cpu, max_bands)?;

    Ok(ResourceFlowSyncReport {
        arenas_planned: plan.arenas.len() as u32,
        total_ops: combined_cpu.len() as u32,
        n_bands: max_bands,
    })
}

/// Resolve sparse RF reductions through the substrate's admitted INPUT_LIST source.
///
/// `sync_resource_flow_ops_from_cpu` resolves the EML program ranges for the complete op set.
/// The packed list upload then installs the explicit source rows, and the final pre-encoded
/// upload retains that list buffer while restoring the complete EML-resolved op set. Dispatch
/// cannot occur between these boundary-time uploads.
fn resolve_sparse_input_lists(
    state: &mut WorldGpuState,
    ops: &[simthing_core::AccumulatorOp],
    n_bands: u32,
) -> Result<(), ResourceFlowSyncError> {
    let sparse_ops: Vec<_> = ops
        .iter()
        .filter(|op| matches!(op.source, SourceSpec::ConjunctiveCrossing { .. }))
        .cloned()
        .collect();
    if sparse_ops.is_empty() {
        return Ok(());
    }

    let sparse_upload = {
        let uploaded_eml = &state
            .accumulator_runtime
            .as_ref()
            .expect("resource-flow runtime exists after logical upload")
            .eml_registry;
        PackedAccumulatorUpload::from_ops_resolving_input_lists_with_eml(
            &sparse_ops,
            Some(uploaded_eml),
        )
        .map_err(|err| ResourceFlowSyncError::SparseInputListEncoding(err.to_string()))?
    };
    let mut patched_gpu_ops = state
        .accumulator_runtime
        .as_ref()
        .expect("resource-flow runtime exists after logical upload")
        .resource_flow_gpu_ops()
        .to_vec();
    let mut resolved_sparse = sparse_upload.ops().iter();
    for (logical, gpu) in ops.iter().zip(&mut patched_gpu_ops) {
        if matches!(logical.source, SourceSpec::ConjunctiveCrossing { .. }) {
            let resolved = resolved_sparse
                .next()
                .expect("one packed input-list row per sparse logical op");
            gpu.source_kind = resolved.source_kind;
            gpu.source_slot = resolved.source_slot;
            gpu.source_col = resolved.source_col;
            gpu.source_count = resolved.source_count;
        }
    }
    assert!(
        resolved_sparse.next().is_none(),
        "all packed sparse input-list rows must be consumed"
    );

    let ctx = &state.ctx;
    let runtime = state
        .accumulator_runtime
        .as_mut()
        .expect("resource-flow runtime exists after logical upload");
    runtime
        .resource_flow_session_mut()
        .expect("resource-flow session exists after logical upload")
        .upload_packed_ops(ctx, &sparse_upload)?;
    runtime.upload_resource_flow_ops(ctx, &patched_gpu_ops, n_bands)?;
    Ok(())
}

#[cfg(test)]
mod pre_band_tests {
    use super::*;
}

pub fn build_plan_for_tests(
    execution: &ArenaExecutionPlan,
    registry: &DimensionRegistry,
    n_slots: u32,
) -> Result<Vec<ArenaAllocationPlan>, HierarchyError> {
    let governed = build_governed_pairs(registry);
    execution
        .arenas
        .iter()
        .map(|arena| {
            let mut plan =
                plan_arena_allocation(arena, &governed, n_slots).map_err(|e| match e {
                    crate::arena_allocation_plan::AllocationPlanError::Hierarchy(h) => h,
                    _ => HierarchyError::EmptyParticipants {
                        arena: "test".into(),
                    },
                })?;
            append_residual_closure_ops(arena, &mut plan.cpu_ops);
            Ok(plan)
        })
        .collect()
}
