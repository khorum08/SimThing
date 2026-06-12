//! E-11 boundary/session sync for resource-flow AccumulatorOp planning.

use simthing_core::{DimensionRegistry, EmlExpressionRegistry};
use simthing_gpu::{build_governed_pairs, WorldGpuState};

use crate::arena_allocation_plan::{plan_arena_allocation, ArenaAllocationPlan};
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
    root: &simthing_core::SimThing,
    allocator: &simthing_gpu::SlotAllocator,
    gated_rates: &[crate::gated_rates::ResolvedGatedRate],
    enabled: bool,
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
        let alloc =
            plan_arena_allocation(arena, &governed, state.n_slots).map_err(|e| match e {
                crate::arena_allocation_plan::AllocationPlanError::Hierarchy(h) => h,
                _ => HierarchyError::EmptyParticipants {
                    arena: arena_registry.arenas[arena.arena_idx as usize].name.clone(),
                },
            })?;
        max_bands = max_bands.max(alloc.n_bands);
        combined_cpu.extend(alloc.cpu_ops);
    }

    if !gated_rates.is_empty() {
        for op in &mut combined_cpu {
            if let simthing_core::GateSpec::OrderBand(band) = op.gate {
                op.gate = simthing_core::GateSpec::OrderBand(band + 1);
            }
        }
        let rate_ops = crate::gated_rates::build_gated_rate_ops(gated_rates, &mut eml_registry);
        let mut all_ops = rate_ops;
        all_ops.extend(combined_cpu);
        combined_cpu = all_ops;
        max_bands += 1;
    }

    state.sync_resource_flow_ops_from_cpu(&combined_cpu, max_bands, &eml_registry)?;

    Ok(ResourceFlowSyncReport {
        arenas_planned: plan.arenas.len() as u32,
        total_ops: combined_cpu.len() as u32,
        n_bands: max_bands,
        enabled: true,
    })
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
            plan_arena_allocation(arena, &governed, n_slots).map_err(|e| match e {
                crate::arena_allocation_plan::AllocationPlanError::Hierarchy(h) => h,
                _ => HierarchyError::EmptyParticipants {
                    arena: "test".into(),
                },
            })
        })
        .collect()
}
