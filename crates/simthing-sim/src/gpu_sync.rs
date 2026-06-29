//! GPU buffer sync - step 9 of the day boundary.
//!
//! After all structural mutations complete (overlay lifecycle, property expiry,
//! fission/fusion, slot sync), the GPU buffers must reflect the new state.
//!
//! Synchronization targets:
//!
//! 1. Overlay deltas.
//!    Rebuilt or reused by the AccumulatorOp OrderBand compiler because
//!    attached/dissolved overlays directly change the transform list.
//!
//! 2. Threshold registrations (Pass 7).
//!    Rebuilt only when the tree, registry, alert registrations, or fission
//!    lineage changed. Overlay-only boundaries can retain the existing buffer.
//!
//! 3. Values shadow flush.
//!    Boundaries can upload known dirty rows, with full shadow upload as the
//!    conservative fallback after slot growth, dimension rebuilds, or tombstones.
//!
//! 4. Reduction topology and column rules (Passes 4-6).
//!    Rebuilt only when tree shape, slot assignment, or registry layout changed.

use crate::fission::FissionLineageRecord;
use crate::sim_runtime_tree::SimRuntimeTree;
use crate::threshold_registry::{
    AggregateAlertRegistration, ThresholdBuilder, ThresholdRegistry, VelocityAlertRegistration,
};
use simthing_core::DimensionRegistry;
use simthing_feeder::{
    CapabilityUnlockRegistration, DispatchCoordinator, ScriptedEventTriggerRegistration,
};
use simthing_gpu::{
    build_column_rule_descriptors, build_governed_pairs, encode_column_rules,
    plan_velocity_integration, ReductionPlanError, SlotAllocator, ThresholdRegistration,
    TopologyState, WorldGpuState,
};

fn upload_accumulator_reduction_plan(
    state: &mut WorldGpuState,
    topology_state: &TopologyState,
    registry: &DimensionRegistry,
    use_accumulator_reduction_exact: bool,
) {
    assert!(
        use_accumulator_reduction_exact,
        "S-4: reduction requires both use_accumulator_reduction_soft and exact"
    );
    let descriptors = build_column_rule_descriptors(registry, state.n_dims as usize);
    match simthing_gpu::plan_reduction_orderband(topology_state, &descriptors, state.n_dims) {
        Ok(plan) => {
            state.ensure_reduction_soft_accumulator();
            state
                .upload_reduction_soft_ops_with_bands(&plan.ops, plan.n_bands)
                .expect("reduction op upload");
        }
        Err(ReductionPlanError::NonContiguousChildren { .. }) => {
            if let Some(runtime) = state.accumulator_runtime.as_mut() {
                runtime.clear_reduction_soft();
            }
            state.set_reduction_soft_dispatch(false, 0);
            state.set_reduction_exact_dispatch(false);
        }
        Err(e) => panic!("reduction OrderBand plan: {e}"),
    }
}

/// Outcome of the GPU sync step.
#[derive(Clone, Debug, Default)]
pub struct GpuSyncOutcome {
    pub overlay_deltas_uploaded: u32,
    pub threshold_regs_uploaded: u32,
    pub new_threshold_registry: Option<ThresholdRegistry>,
    /// GPU threshold registrations when `rebuild_thresholds` ran (C-1 sync).
    pub rebuilt_threshold_regs: Option<Vec<ThresholdRegistration>>,
    pub reduction_depths: u32,
    pub reduction_edges: u32,
    pub reduction_slots: u32,
    pub boundary_upload_bytes: u64,
    pub value_rows_uploaded: u32,
    pub full_value_upload: bool,
}

/// Rebuild boundary-dependent GPU buffers.
///
/// `rebuild_thresholds` and `rebuild_reduction_topology` allow active but
/// topology-stable boundaries, such as player/AI overlay attachment, to retain
/// expensive GPU buffers while still refreshing overlays and value rows.
pub fn sync_gpu_buffers(
    root: &SimRuntimeTree,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    coord: &DispatchCoordinator,
    state: &mut WorldGpuState,
    velocity_alerts: &[VelocityAlertRegistration],
    aggregate_alerts: &[AggregateAlertRegistration],
    capability_unlocks: &[CapabilityUnlockRegistration],
    scripted_event_triggers: &[ScriptedEventTriggerRegistration],
    fission_lineage: &[FissionLineageRecord],
    dirty_value_slots: Option<&[u32]>,
    rebuild_thresholds: bool,
    rebuild_reduction_topology: bool,
    use_accumulator_overlay_add: bool,
    use_accumulator_reduction_soft: bool,
    use_accumulator_reduction_exact: bool,
    use_accumulator_velocity: bool,
    use_accumulator_eml: bool,
    overlay_compile_revision: u64,
    // B2 Approach C: the canonical TopologyState owned by the boundary.
    // When `rebuild_reduction_topology` is true, this routine refreshes
    // the cache from a full tree walk and re-flattens to CSR. When false
    // (because the boundary already applied an incremental delta and
    // uploaded), the cache is left as-is.
    topology_state: &mut TopologyState,
) -> GpuSyncOutcome {
    let mut out = GpuSyncOutcome::default();

    // 1. Overlay deltas - compiled into AccumulatorOp OrderBands.
    //
    // `build_overlay_deltas` returns one range per allocated slot, so
    // `ranges.len() == allocator.capacity()`. `state.upload_overlay_deltas`
    // requires `ranges.len() == state.n_slots` (the static GPU buffer size).
    // Pad with zero-length ranges for slots that don't exist yet; Pass 3's
    // shader skips those naturally because `range.length == 0`.
    let mut n_deltas = 0u32;
    let mut overlay_upload_bytes = 0u64;
    if use_accumulator_overlay_add {
        state.ensure_overlay_accumulator();

        let cache_current = state
            .accumulator_runtime
            .as_ref()
            .and_then(|runtime| runtime.overlay_compile_cache.as_ref())
            .is_some_and(|cache| cache.compiled_at_revision == overlay_compile_revision);

        if cache_current {
            if let Some(cache) = state
                .accumulator_runtime
                .as_ref()
                .and_then(|runtime| runtime.overlay_compile_cache.as_ref())
            {
                n_deltas = cache.cached_deltas.len() as u32;
                state.set_overlay_add_dispatch(
                    cache.cached_op_buffer_uploaded_n_ops > 0,
                    cache.cached_n_bands,
                );
            }
        } else {
            let (deltas, mut ranges) =
                simthing_gpu::build_overlay_deltas(root.inner(), registry, allocator);
            if (ranges.len() as u32) < state.n_slots {
                ranges.resize(
                    state.n_slots as usize,
                    simthing_gpu::SlotDeltaRange::default(),
                );
            }
            n_deltas = deltas.len() as u32;

            let cache_equal = state
                .accumulator_runtime
                .as_ref()
                .and_then(|runtime| runtime.overlay_compile_cache.as_ref())
                .is_some_and(|cache| {
                    cache.cached_deltas == deltas && cache.cached_ranges == ranges
                });

            if cache_equal {
                let mut dispatch = None;
                if let Some(runtime) = state.accumulator_runtime.as_mut() {
                    if let Some(cache) = runtime.overlay_compile_cache.as_mut() {
                        cache.compiled_at_revision = overlay_compile_revision;
                        dispatch = Some((
                            cache.cached_op_buffer_uploaded_n_ops > 0,
                            cache.cached_n_bands,
                        ));
                    }
                }
                if let Some((active, n_bands)) = dispatch {
                    state.set_overlay_add_dispatch(active, n_bands);
                }
            } else {
                let prior_counts = state
                    .accumulator_runtime
                    .as_ref()
                    .and_then(|runtime| runtime.overlay_compile_cache.as_ref())
                    .map(|cache| (cache.compile_count, cache.upload_count))
                    .unwrap_or((0, 0));
                let simthing_gpu::OverlayOrderBandPlan { ops, n_bands } =
                    simthing_gpu::plan_overlay_orderband(&deltas, &ranges, state.n_slots);
                state
                    .upload_overlay_ops_with_bands(&ops, n_bands)
                    .expect("overlay OrderBand op upload failed");
                if let Some(runtime) = state.accumulator_runtime.as_mut() {
                    runtime.overlay_compile_cache = Some(simthing_gpu::OverlayCompileCache {
                        compiled_at_revision: overlay_compile_revision,
                        cached_deltas: deltas.clone(),
                        cached_ranges: ranges.clone(),
                        cached_n_bands: n_bands,
                        cached_op_buffer_uploaded_n_ops: ops.len() as u32,
                        compile_count: prior_counts.0 + 1,
                        upload_count: prior_counts.1 + 1,
                    });
                }
                overlay_upload_bytes =
                    ops.len() as u64 * std::mem::size_of::<simthing_gpu::AccumulatorOpGpu>() as u64;
            }
        }
    } else {
        let (deltas, mut ranges) =
            simthing_gpu::build_overlay_deltas(root.inner(), registry, allocator);
        if (ranges.len() as u32) < state.n_slots {
            ranges.resize(
                state.n_slots as usize,
                simthing_gpu::SlotDeltaRange::default(),
            );
        }
        n_deltas = deltas.len() as u32;
        if !deltas.is_empty() {
            panic!(
                "Legacy overlay path was deleted in S-3; AccumulatorOp overlay must remain enabled."
            );
        }
        if let Some(runtime) = state.accumulator_runtime.as_mut() {
            runtime.clear_overlay_orderband();
        }
        state.set_overlay_add_dispatch(false, 0);
    }
    out.overlay_deltas_uploaded = n_deltas;

    let mut threshold_upload_bytes = 0u64;
    if rebuild_thresholds {
        // 2. Threshold registrations, including fission lineage to FusionTrigger regs.
        let (mut gpu_regs, mut cpu_reg) = ThresholdBuilder::build_with_lineage(
            root,
            registry,
            allocator,
            velocity_alerts,
            aggregate_alerts,
            fission_lineage,
        );
        ThresholdBuilder::append_capability_unlocks(
            registry,
            allocator,
            capability_unlocks,
            &mut gpu_regs,
            &mut cpu_reg,
        );
        ThresholdBuilder::append_scripted_event_triggers(
            scripted_event_triggers,
            &mut gpu_regs,
            &mut cpu_reg,
        );
        threshold_upload_bytes = gpu_regs.len() as u64
            * std::mem::size_of::<simthing_gpu::ThresholdRegistration>() as u64;
        out.threshold_regs_uploaded = gpu_regs.len() as u32;
        state.upload_thresholds(&gpu_regs);
        out.rebuilt_threshold_regs = Some(gpu_regs);
        out.new_threshold_registry = Some(cpu_reg);
    }

    // 3. Values shadow flush.
    //
    // For the dirty-slot path we coalesce adjacent slot indices into one
    // contiguous range per `queue.write_buffer` call. Fission and AddChild
    // pre-grow allocate slots sequentially, so the typical dense case
    // collapses into a small handful of ranges — avoiding the per-slot
    // driver overhead that quickly dominates at thousands of dirty slots.
    let value_upload_bytes = if let Some(slots) = dirty_value_slots {
        let mut sorted: Vec<u32> = slots.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        let mut i = 0;
        while i < sorted.len() {
            let start = sorted[i];
            let mut end = start;
            while i + 1 < sorted.len() && sorted[i + 1] == end + 1 {
                end = sorted[i + 1];
                i += 1;
            }
            coord.upload_row_range(state, start, end - start + 1);
            i += 1;
        }
        out.value_rows_uploaded = sorted.len() as u32;
        out.full_value_upload = false;
        sorted.len() as u64 * state.n_dims as u64 * std::mem::size_of::<f32>() as u64
    } else {
        coord.upload_full_shadow(state);
        out.value_rows_uploaded = state.n_slots;
        out.full_value_upload = true;
        coord.shadow.len() as u64 * std::mem::size_of::<f32>() as u64
    };

    let mut reduction_upload_bytes = 0u64;
    if rebuild_reduction_topology {
        // 4. Reduction topology + per-column rule table.
        // Refresh the boundary's canonical TopologyState from the tree
        // and re-flatten. The cache stays in lockstep with the GPU buffer.
        *topology_state = TopologyState::build(root.inner(), allocator);
        let topo = topology_state.flatten();
        let descriptors = build_column_rule_descriptors(registry, state.n_dims as usize);
        let rules_u32 = encode_column_rules(&descriptors);

        let mut depth_slots: Vec<u32> = Vec::new();
        let mut depth_ranges: Vec<(u32, u32)> = Vec::new();
        for bucket in &topo.depth_buckets {
            let offset = depth_slots.len() as u32;
            depth_slots.extend_from_slice(bucket);
            depth_ranges.push((offset, bucket.len() as u32));
        }
        out.reduction_depths = depth_ranges.len() as u32;
        out.reduction_slots = depth_slots.len() as u32;

        // `upload_reduction_topology` asserts `child_starts.len() == n_slots + 1`.
        // `build_topology` produces a CSR sized to `allocator.capacity()`, which
        // can be less than `state.n_slots` when WorldGpuState has growth headroom.
        // Pad with the sentinel value so unallocated slots have empty child ranges.
        let n_slots = state.n_slots as usize;
        let mut child_starts = topo.child_starts.clone();
        if child_starts.len() < n_slots + 1 {
            let last = *child_starts.last().unwrap_or(&0);
            child_starts.resize(n_slots + 1, last);
        }

        state.upload_reduction_topology(
            &child_starts,
            &topo.child_indices,
            &rules_u32,
            &depth_slots,
            depth_ranges,
        );
        out.reduction_edges = topo.child_indices.len() as u32;
        reduction_upload_bytes = child_starts.len() as u64 * std::mem::size_of::<u32>() as u64
            + topo.child_indices.len() as u64 * std::mem::size_of::<u32>() as u64
            + rules_u32.len() as u64 * std::mem::size_of::<u32>() as u64
            + depth_slots.len() as u64 * std::mem::size_of::<u32>() as u64;

        if use_accumulator_reduction_soft {
            upload_accumulator_reduction_plan(
                state,
                topology_state,
                registry,
                use_accumulator_reduction_exact,
            );
        }
    } else if use_accumulator_reduction_soft {
        upload_accumulator_reduction_plan(
            state,
            topology_state,
            registry,
            use_accumulator_reduction_exact,
        );
    }

    let pairs = build_governed_pairs(registry);
    if use_accumulator_velocity {
        state.ensure_velocity_accumulator();
        let plan = plan_velocity_integration(&pairs, state.n_slots);
        state
            .upload_velocity_ops_with_bands(&plan.ops, plan.n_bands)
            .expect("velocity op upload failed");
    } else {
        if !pairs.is_empty() {
            panic!(
                "Legacy velocity_integration.wgsl was deleted in S-5; AccumulatorOp velocity must remain enabled when governed velocity pairs exist."
            );
        }
        if let Some(runtime) = state.accumulator_runtime.as_mut() {
            runtime.clear_velocity();
            state.set_velocity_dispatch(false, 0);
        }
    }

    if use_accumulator_eml {
        state
            .sync_eml_program_table()
            .expect("EML program table upload failed");
    }

    out.boundary_upload_bytes =
        value_upload_bytes + overlay_upload_bytes + threshold_upload_bytes + reduction_upload_bytes;

    out
}

#[cfg(test)]
mod tests {
    // Integration tests for gpu_sync require a GPU adapter.
    // They live in tests/boundary_integration.rs to keep this file light.
}
