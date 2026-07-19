//! E-11 boundary/session sync for resource-flow AccumulatorOp planning.

use simthing_core::{DimensionRegistry, EmlExpressionRegistry};
use simthing_gpu::{build_governed_pairs, WorldGpuState};
use simthing_sim::SimRuntimeTree;

use crate::arena_allocation_plan::{
    append_residual_closure_ops, plan_arena_allocation, ArenaAllocationPlan,
};
use crate::arena_hierarchy::{
    build_execution_plan, resolve_node_columns, ArenaExecutionPlan, HierarchyError,
};
use crate::arena_participant::ArenaParticipantScaffold;
use crate::arena_registry::ArenaRegistry;
use crate::child_share_eml::register_child_share_formula;
use thiserror::Error;

#[derive(Clone, Debug, Default)]
pub struct ResourceFlowSyncReport {
    pub arenas_planned: u32,
    pub total_ops: u32,
    pub n_bands: u32,
    pub enabled: bool,
}

#[derive(Debug, Error)]
pub enum ResourceFlowSyncError {
    #[error(transparent)]
    Hierarchy(#[from] HierarchyError),
    #[error(transparent)]
    OpUpload(#[from] simthing_gpu::AccumulatorOpSessionError),
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

/// Plan and upload E-11 allocation ops when `use_accumulator_resource_flow` is enabled.
///
/// When gated rates exist (CT-RF-EML-RATE-0), every arena op shifts up one
/// OrderBand and the effective-rate `EvalEML` ops occupy band 0, so the
/// intrinsic columns are recomputed from base/gate state before any reduce.
pub fn sync_resource_flow_accumulator(
    state: &mut WorldGpuState,
    registry: &DimensionRegistry,
    arena_registry: &ArenaRegistry,
    scaffold: &ArenaParticipantScaffold,
    root: &SimRuntimeTree,
    allocator: &simthing_gpu::SlotAllocator,
    gated_rates: &[crate::gated_rates::ResolvedGatedRate],
    need_bindings: &[crate::need_binding::ResolvedNeedBinding],
    enabled: bool,
) -> Result<ResourceFlowSyncReport, ResourceFlowSyncError> {
    sync_resource_flow_accumulator_with_options(
        state,
        registry,
        arena_registry,
        scaffold,
        root,
        allocator,
        gated_rates,
        need_bindings,
        enabled,
        true,
    )
}

/// Same as [`sync_resource_flow_accumulator`] with RF-5A stage-projection control.
pub(crate) fn sync_resource_flow_accumulator_with_options(
    state: &mut WorldGpuState,
    registry: &DimensionRegistry,
    arena_registry: &ArenaRegistry,
    scaffold: &ArenaParticipantScaffold,
    root: &SimRuntimeTree,
    allocator: &simthing_gpu::SlotAllocator,
    gated_rates: &[crate::gated_rates::ResolvedGatedRate],
    need_bindings: &[crate::need_binding::ResolvedNeedBinding],
    enabled: bool,
    include_need_stage_projections: bool,
) -> Result<ResourceFlowSyncReport, ResourceFlowSyncError> {
    if !enabled || arena_registry.arenas.is_empty() {
        state.clear_resource_flow_accumulator();
        return Ok(ResourceFlowSyncReport {
            enabled: false,
            ..Default::default()
        });
    }

    let plan = build_execution_plan(
        registry,
        &arena_registry.arenas,
        root,
        allocator,
        scaffold,
        arena_registry.generation,
    )?;

    let mut eml_registry = EmlExpressionRegistry::new();
    for arena in &plan.arenas {
        let layout = registry.property(arena.flow_property_id).layout.clone();
        let cols = resolve_node_columns(
            &layout,
            &arena_registry.arenas[arena.arena_idx as usize].name,
        )?;
        register_child_share_formula(&mut eml_registry, cols).expect("child_share EML registers");
    }

    let governed = build_governed_pairs(registry);
    let mut combined_cpu = Vec::new();
    let mut max_bands = 0u32;
    for arena in &plan.arenas {
        let mut alloc =
            plan_arena_allocation(arena, &governed, state.n_slots).map_err(|e| match e {
                crate::arena_allocation_plan::AllocationPlanError::Hierarchy(h) => h,
                _ => HierarchyError::EmptyParticipants {
                    arena: arena_registry.arenas[arena.arena_idx as usize].name.clone(),
                },
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

    Ok(ResourceFlowSyncReport {
        arenas_planned: plan.arenas.len() as u32,
        total_ops: combined_cpu.len() as u32,
        n_bands: max_bands,
        enabled: true,
    })
}

#[cfg(test)]
mod pre_band_tests {
    use super::*;

    #[test]
    fn gated_rate_and_need_binding_bands_are_dependency_ordered() {
        let map = compose_pre_band_map(true, true);
        assert_eq!(map.gated_start, Some(0));
        assert_eq!(map.need_stage, Some(1));
        assert_eq!(map.need_eval, Some(2));
        assert_eq!(map.arena_start, 3);
        assert!(map.gated_start.unwrap() < map.need_stage.unwrap());
        assert!(map.need_stage.unwrap() < map.need_eval.unwrap());
        assert!(map.need_eval.unwrap() < map.arena_start);
    }
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
