//! WorldGpuState — owns every persistent GPU buffer the simulation reads or writes.
//!
//! Buffer layout follows agents.md:
//!   values, previous_values, output_vectors  : [N_slots × N_dims]      (row-major)
//!   governed_pairs                           : [N_pairs × GovernedPair]      (property-level)
//!   overlay_deltas                           : [N_deltas × OverlayDelta]     (per-tick upload)
//!   slot_delta_ranges                        : [N_slots × SlotDeltaRange]    (per-tick upload)
//!
//! Pass 3 reads overlay_deltas via slot_delta_ranges and applies each op
//! iteratively per slot. See agents.md "Transform application — iterative on GPU".
//!
//! Threshold registry / event_candidates buffers are deferred to Pass 7 work.

use crate::gpu_readback::ThresholdEventCandidatesReadback;
use crate::resolved::ResolvedGpuBuffers;
use crate::sealed::{
    birth_anchor_rows_gpu, decode_anchor_table_gpu, encode_anchor_table_gpu, AnchorRemapOpGpu,
    AnchorRemapParams, AnchorTableRowGpu, ResolvedWriteAuthority, ANCHOR_REMAP_KIND_MOVE,
    ANCHOR_REMAP_KIND_RETIRE, ANCHOR_REMAP_KIND_ROW_MOVE,
};
use crate::wgsl_encode::{build_governed_pairs, encode_column, GovernedPair};
use bytemuck::{Pod, Zeroable};
use simthing_core::{AnchorRemapSection, AnchorTable, DimensionRegistry};
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages,
    CommandEncoderDescriptor, ComputePassDescriptor, ComputePipelineDescriptor, Maintain, MapMode,
    PipelineLayoutDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages,
};

use crate::accumulator_op::DEFAULT_THRESHOLD_EMISSION_CAPACITY;
use crate::context::GpuContext;

// ── GovernedPair — GPU-friendly encoding of a (governed, governing) sub-field pair ──

/// Emit one [`GovernedPair`] per sub-field with `governed_by: Some(_)` in `layout`.
///
/// E-7: role-agnostic discovery — supports `(Amount, Velocity)`, `(Named("balance"),
/// Named("flow"))`, and any other declared pair. Skips entries whose governing role
/// is absent from the layout (matches CPU `PropertyValue::integrate`). Invalid
/// `governed_by` links are hard errors at the `simthing-spec` compile layer.
/// Walk every active property in the registry and emit one [`GovernedPair`] per
/// governed sub-field. Matches the CPU oracle in `PropertyValue::integrate`.
// ── OverlayDelta — one applied op, in evaluation order ──────────────────────

pub const OP_MULTIPLY: u32 = 0;
pub const OP_ADD: u32 = 1;
pub const OP_SET: u32 = 2;

/// A single column-targeted overlay op, ready to apply on the GPU.
/// `col` is the global column index (already resolved through `col_for_role`
/// during the CPU prep pass). `op_kind` is one of OP_MULTIPLY / OP_ADD / OP_SET.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct OverlayDelta {
    pub col: u32,
    pub op_kind: u32,
    pub value: f32,
    pub _pad: u32,
}

/// Per-slot index range into the flat `overlay_deltas` buffer. A slot with
/// no overlays has `length == 0` and Pass 3 is a no-op for it.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct SlotDeltaRange {
    pub offset: u32,
    pub length: u32,
}

/// A single per-tick intent transform, folded to affine form for one resolved
/// `(slot, col)`: `value = value * mul + add`.
///
/// Folding on the CPU preserves original arrival order for any sequence of
/// Set/Add/Multiply ops selection the same cell, while the numeric
/// read-modify-write stays on the GPU.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct IntentDelta {
    pub slot: u32,
    pub col: u32,
    pub mul: f32,
    pub add: f32,
}

// ── ThresholdRegistration / ThresholdEvent re-exports for legacy import paths ──

pub use crate::registration::{
    ThresholdRegistration, DIR_DOWNWARD, DIR_EITHER, DIR_UPWARD, THRESH_BUF_OUTPUT,
    THRESH_BUF_VALUES,
};
pub use crate::sealed::{cpu_oracle_threshold_events, ThresholdEvent, ThresholdEventGpu};

// ── Reduction (Passes 4–6) ────────────────────────────────────────────────────
// ── WorldGpuState ─────────────────────────────────────────────────────────────

pub struct WorldGpuState {
    pub ctx: GpuContext,
    pub n_slots: u32,
    pub n_dims: u32,
    pub n_governed_pairs: u32,
    pub n_overlay_deltas: u32,
    pub n_intent_deltas: u32,

    pub(crate) resolved: ResolvedGpuBuffers,

    /// Property-level flat buffer of GovernedPair structs. Same pairs apply
    /// to every slot — Pass 1 dispatches `(n_pairs × n_slots)` threads.
    pub(crate) governed_pairs: Buffer,

    /// Flat per-tick array of overlay deltas, ancestor stack then local, in
    /// evaluation order. Grows as needed via `upload_overlay_deltas`.
    pub(crate) overlay_deltas: Buffer,

    /// Per-slot (offset, length) into `overlay_deltas`. Size: `n_slots × 8B`.
    pub(crate) slot_delta_ranges: Buffer,

    /// Flat per-tick array of folded player/AI/feeder intent deltas. Grows as
    /// needed via `upload_intent_deltas`.
    pub(crate) intent_deltas: Buffer,

    /// Pass 7 inputs: flat array of ThresholdRegistration structs.
    /// Grows on demand via `upload_thresholds`.
    pub(crate) threshold_registry: Buffer,
    /// Pass 7 outputs: kernel-owned event candidate readback buffers.
    threshold_events: ThresholdEventCandidatesReadback,

    /// Number of currently-registered thresholds (i.e. valid entries in
    /// `threshold_registry`). Pass 7 dispatches one thread per registration.
    pub n_thresholds: u32,

    /// Derived STEAD anchor table (ANCHOR-TABLE-SURFACE-0). Grows via
    /// `upload_anchor_table`. This GPU POD buffer is the sole production
    /// observation authority (orch remand `5120259758`); CPU staging on the
    /// boundary is writer-side only — not the values matrix.
    pub(crate) anchor_table: Buffer,
    /// Valid row count currently uploaded into `anchor_table`.
    pub n_anchor_rows: u32,
    /// Dispatch generation stamped onto fused GPU crossing updates.
    pub anchor_table_generation: u32,

    /// EVENT-GENERATION-STAMP-0: admitted observer egress ring for sealed emissions.
    /// Production emission tick pushes through `push_emissions_into_production_egress`.
    /// Forced observer lag applies ring backpressure without writing sim state.
    pub production_event_egress: simthing_core::StampedEventRing,

    // ── Reduction (Passes 4–6) ───────────────────────────────────────────────
    /// CSR child topology: `child_starts[i]..child_starts[i+1]` indexes
    /// children of parent slot `i`. Length `n_slots + 1` u32s.
    pub(crate) child_starts: Buffer,
    /// Concatenated child slot indices, in canonical (ascending slot) order.
    pub(crate) child_indices: Buffer,
    /// Per-column reduction rule (u32), length `n_dims`.
    pub(crate) column_rules: Buffer,
    /// Concatenated depth buckets — slot indices grouped by tree depth.
    /// `depth_bucket_ranges` tells AccumulatorOp reduction encoding how to
    /// slice this. Empty when no topology has been uploaded yet.
    pub(crate) depth_slots: Buffer,
    /// (offset, size) into `depth_slots` per depth. The dispatcher iterates
    /// these from the last entry (deepest) to the first (root depth).
    pub depth_bucket_ranges: Vec<(u32, u32)>,

    /// AccumulatorOp v2 world runtime (C-INF-1): one session, named op sets.
    pub accumulator_runtime: Option<crate::WorldAccumulatorRuntime>,
    /// Cached C-3 overlay dispatch signal (mirrors runtime; survives runtime `take()`).
    pub accumulator_overlay_add_active: bool,
    pub accumulator_overlay_add_bands: u32,
    /// Cached C-5 soft reduction dispatch signal (mirrors runtime; survives runtime `take()`).
    pub accumulator_reduction_soft_active: bool,
    pub accumulator_reduction_soft_bands: u32,
    /// Cached C-6 exact reduction dispatch signal (requires soft flag).
    pub accumulator_reduction_exact_active: bool,
    /// Cached C-7 velocity integration dispatch signal.
    pub accumulator_velocity_active: bool,
    pub accumulator_velocity_bands: u32,
    /// Cached C-8b intensity EvalEML dispatch signal.
    pub accumulator_intensity_eml_active: bool,
    pub accumulator_intensity_eml_bands: u32,
    /// Cached C-8c transfer dispatch signal.
    pub accumulator_transfer_active: bool,
    pub accumulator_transfer_bands: u32,
    /// Cached C-8d emission dispatch signal.
    pub accumulator_emission_active: bool,
    pub accumulator_emission_bands: u32,
    /// E-11 resource-flow allocation OrderBand dispatch (default off).
    pub accumulator_resource_flow_active: bool,
    pub accumulator_resource_flow_bands: u32,
    /// Full threshold regs last uploaded (restore after need-only rescan).
    post_rf_full_threshold_regs: Vec<ThresholdRegistration>,
    /// Need-cell thresholds only for post-RF append-only rescan.
    post_rf_need_threshold_regs: Vec<ThresholdRegistration>,
    /// GPU-resident structural remap pipeline (orch remand `5120847431`).
    anchor_remap_pipeline: wgpu::ComputePipeline,
    anchor_remap_layout: wgpu::BindGroupLayout,
    anchor_remap_uniform: Buffer,
    /// Values-only magnitude refresh when no threshold session is present
    /// (orch remand `5121185090`).
    anchor_magnitude_values_pipeline: wgpu::ComputePipeline,
    anchor_magnitude_values_layout: wgpu::BindGroupLayout,
    anchor_magnitude_values_uniform: Buffer,
}

impl WorldGpuState {
    /// C-1 threshold scan against sealed resolved buffers (external observation path).
    pub fn dispatch_accumulator_threshold_scan(
        &self,
        session: &mut crate::AccumulatorOpSession,
    ) -> Result<(), crate::AccumulatorOpSessionError> {
        // Direct-drive must prepare tick uniform / emission count before encode
        // (same contract as AccumulatorOpSession::dispatch_threshold_scan).
        session.prepare_threshold_scan(&self.ctx);
        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("accumulator_threshold_scan"),
            });
        session.encode_threshold_scan_with_anchor_maintain_into(
            &self.ctx,
            &mut encoder,
            &self.resolved.values(),
            &self.resolved.previous_values(),
            &self.resolved.output_vectors(),
            &self.resolved.previous_output_vectors(),
            Some((
                &self.anchor_table,
                self.n_anchor_rows,
                self.anchor_table_generation,
            )),
        );
        self.ctx.queue.submit(Some(encoder.finish()));
        session.finish_threshold_scan(&self.ctx);
        Ok(())
    }

    /// Magnitude-only fused companion (post-remap / no-crossing refresh).
    ///
    /// Must run only after structural GPU value sync has installed canonical
    /// values at the rows' current slot/col (orch remand `5121185090`). When a
    /// threshold session is present, urgency is derived from threshold ops;
    /// otherwise observed_value is refreshed from the values plane with urgency 0.
    pub fn run_anchor_table_magnitude_maintain(&mut self) {
        if self.n_anchor_rows == 0 {
            return;
        }
        if let Some(runtime) = self.accumulator_runtime.as_mut() {
            if let Some(session) = runtime.take_threshold_session() {
                let mut encoder =
                    self.ctx
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("anchor_table_magnitude_maintain"),
                        });
                session.encode_anchor_table_maintain_into(
                    &self.ctx,
                    &mut encoder,
                    &self.resolved.values(),
                    &self.anchor_table,
                    self.n_anchor_rows,
                    self.anchor_table_generation,
                    false,
                );
                self.ctx.queue.submit(Some(encoder.finish()));
                if let Some(runtime) = self.accumulator_runtime.as_mut() {
                    runtime.restore_threshold_session(Some(session));
                }
                return;
            }
        }
        self.dispatch_anchor_magnitude_values_only();
    }

    fn dispatch_anchor_magnitude_values_only(&mut self) {
        #[repr(C)]
        #[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
        struct MagnitudeValuesParams {
            n_dims: u32,
            n_anchor_rows: u32,
            _pad0: u32,
            _pad1: u32,
        }
        let params = MagnitudeValuesParams {
            n_dims: self.n_dims,
            n_anchor_rows: self.n_anchor_rows,
            _pad0: 0,
            _pad1: 0,
        };
        self.ctx.queue.write_buffer(
            &self.anchor_magnitude_values_uniform,
            0,
            bytemuck::bytes_of(&params),
        );
        let bind_group = self.ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("anchor_magnitude_values_bg"),
            layout: &self.anchor_magnitude_values_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: self.resolved.values().as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: self.anchor_table.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: self.anchor_magnitude_values_uniform.as_entire_binding(),
                },
            ],
        });
        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("anchor_table_magnitude_values"),
            });
        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("anchor_magnitude_values_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.anchor_magnitude_values_pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            let groups = self.n_anchor_rows.div_ceil(64);
            pass.dispatch_workgroups(groups, 1, 1);
        }
        self.ctx.queue.submit(Some(encoder.finish()));
    }

    pub fn set_anchor_table_generation(&mut self, generation: u32) {
        self.anchor_table_generation = generation;
    }

    /// EVENT-GENERATION-STAMP-0: bind the tree's generation authority for production
    /// seal/readback. Called at the ordinary generation step boundary (same day
    /// counter that advances `anchor_table_generation`). Every session that
    /// mints sealed events/emissions inherits this stamp source — callers do
    /// not need a separate optional setter.
    pub fn bind_production_generation(&mut self, generation: u32) {
        self.anchor_table_generation = generation;
        if let Some(runtime) = self.accumulator_runtime.as_mut() {
            for session in runtime.all_sessions_mut() {
                session.bind_generation_authority(generation);
            }
        }
    }

    /// Admission / test upload of a typed STEAD table (encodes POD at this boundary).
    pub fn upload_typed_anchor_table(&mut self, table: &AnchorTable) {
        self.upload_anchor_table(&encode_anchor_table_gpu(table));
    }

    /// Consumer / oracle readback as the typed STEAD table.
    pub fn read_typed_anchor_table(&self, registry: &DimensionRegistry) -> AnchorTable {
        decode_anchor_table_gpu(&self.read_anchor_table(), registry)
    }

    /// GPU-resident structural remap (orch remand `5120847431`).
    ///
    /// Applies move/retire on the live GPU buffer and appends birth seeds minted
    /// from the registry — never reads the live table to CPU for mutation.
    pub fn apply_anchor_remap_section(
        &mut self,
        section: &AnchorRemapSection,
        registry: &DimensionRegistry,
    ) {
        if section.remap_not_required || section.remaps.is_empty() {
            return;
        }
        let mut ops = Vec::new();
        let mut births = Vec::new();
        for remap in &section.remaps {
            let Some(property_id) = remap.property_id() else {
                // ObjectRow epoch rebind: one op moves EVERY row of the object
                // from its old physical slot; columns are untouched by
                // construction (the subject has no column fields).
                if let (Some(from_slot), Some(to_slot)) = (remap.from_slot, remap.to_slot) {
                    ops.push(AnchorRemapOpGpu {
                        sim_thing_id: remap.sim_thing_id.raw(),
                        property_id: 0,
                        kind: ANCHOR_REMAP_KIND_ROW_MOVE,
                        from_slot: from_slot.raw(),
                        from_col: 0,
                        to_slot: to_slot.raw(),
                        to_col: 0,
                        _pad: 0,
                    });
                }
                continue;
            };
            match (
                remap.to_slot,
                remap.to_col(),
                remap.from_slot,
                remap.from_col(),
            ) {
                (None, None, Some(from_slot), Some(from_col)) => {
                    ops.push(AnchorRemapOpGpu {
                        sim_thing_id: remap.sim_thing_id.raw(),
                        property_id: property_id.0,
                        kind: ANCHOR_REMAP_KIND_RETIRE,
                        from_slot: from_slot.raw(),
                        from_col: from_col.raw_u32(),
                        to_slot: 0,
                        to_col: 0,
                        _pad: 0,
                    });
                }
                (Some(to_slot), Some(to_col), Some(from_slot), Some(from_col)) => {
                    ops.push(AnchorRemapOpGpu {
                        sim_thing_id: remap.sim_thing_id.raw(),
                        property_id: property_id.0,
                        kind: ANCHOR_REMAP_KIND_MOVE,
                        from_slot: from_slot.raw(),
                        from_col: from_col.raw_u32(),
                        to_slot: to_slot.raw(),
                        to_col: to_col.raw_u32(),
                        _pad: 0,
                    });
                }
                (Some(to_slot), Some(to_col), None, None) => {
                    births.extend(birth_anchor_rows_gpu(
                        remap.sim_thing_id,
                        property_id,
                        to_slot,
                        to_col,
                        registry,
                    ));
                }
                _ => {}
            }
        }
        if ops.is_empty() && births.is_empty() {
            return;
        }
        // Structural remap only — magnitude refresh must wait until Step 9
        // value sync installs canonical cells at the new coordinates.
        self.dispatch_anchor_remap_gpu(&ops, &births);
    }

    fn dispatch_anchor_remap_gpu(
        &mut self,
        ops: &[AnchorRemapOpGpu],
        births: &[AnchorTableRowGpu],
    ) {
        let n_src = self.n_anchor_rows;
        let capacity = (n_src as usize)
            .saturating_add(births.len())
            .saturating_add(ops.len())
            .max(1);
        let row_bytes = std::mem::size_of::<AnchorTableRowGpu>() as u64;
        let dest = self.ctx.device.create_buffer(&BufferDescriptor {
            label: Some("anchor_table_remap_dest"),
            size: row_bytes * capacity as u64,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let ops_buf = self.ctx.device.create_buffer(&BufferDescriptor {
            label: Some("anchor_remap_ops"),
            size: (std::mem::size_of::<AnchorRemapOpGpu>() * ops.len().max(1)) as u64,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        if !ops.is_empty() {
            self.ctx
                .queue
                .write_buffer(&ops_buf, 0, bytemuck::cast_slice(ops));
        }
        let births_buf = self.ctx.device.create_buffer(&BufferDescriptor {
            label: Some("anchor_remap_births"),
            size: (std::mem::size_of::<AnchorTableRowGpu>() * births.len().max(1)) as u64,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        if !births.is_empty() {
            self.ctx
                .queue
                .write_buffer(&births_buf, 0, bytemuck::cast_slice(births));
        }
        let count_buf = self.ctx.device.create_buffer(&BufferDescriptor {
            label: Some("anchor_remap_out_count"),
            size: 4,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.ctx
            .queue
            .write_buffer(&count_buf, 0, &0u32.to_le_bytes());
        let params = AnchorRemapParams {
            n_src_rows: n_src,
            n_ops: ops.len() as u32,
            n_births: births.len() as u32,
            _pad: 0,
        };
        self.ctx
            .queue
            .write_buffer(&self.anchor_remap_uniform, 0, bytemuck::bytes_of(&params));
        let bind_group = self.ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("anchor_table_remap_bg"),
            layout: &self.anchor_remap_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: self.anchor_table.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: ops_buf.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: births_buf.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: dest.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 4,
                    resource: count_buf.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 5,
                    resource: self.anchor_remap_uniform.as_entire_binding(),
                },
            ],
        });
        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("anchor_table_remap_encoder"),
            });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("anchor_table_remap_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.anchor_remap_pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        let count_readback = self.ctx.device.create_buffer(&BufferDescriptor {
            label: Some("anchor_remap_count_readback"),
            size: 4,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        encoder.copy_buffer_to_buffer(&count_buf, 0, &count_readback, 0, 4);
        // Grow live table if needed, then copy dest → live.
        let needed_bytes = row_bytes * capacity as u64;
        if needed_bytes > self.anchor_table.size() {
            self.anchor_table = self.ctx.device.create_buffer(&BufferDescriptor {
                label: Some("anchor_table"),
                size: needed_bytes,
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }
        encoder.copy_buffer_to_buffer(&dest, 0, &self.anchor_table, 0, needed_bytes);
        self.ctx.queue.submit(Some(encoder.finish()));
        self.ctx.device.poll(Maintain::Wait);
        let slice = count_readback.slice(..);
        slice.map_async(MapMode::Read, |_| {});
        self.ctx.device.poll(Maintain::Wait);
        let data = slice.get_mapped_range();
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&data[..4]);
        drop(data);
        count_readback.unmap();
        self.n_anchor_rows = u32::from_le_bytes(bytes);
    }

    /// Indexed scatter from resolved `values` into `dest` (mapping hot path).
    pub fn dispatch_indexed_scatter_from_resolved_values(
        &self,
        scatter: &crate::IndexedScatterOp,
        dest: &Buffer,
        entries: &[crate::ScatterEntry],
    ) -> Result<(), crate::IndexedScatterError> {
        scatter.dispatch(&self.ctx, self.resolved.values(), dest, entries)
    }

    /// Encode AccumulatorOp OrderBand passes against sealed resolved buffers.
    pub fn encode_accumulator_orderband_into(
        &self,
        session: &mut crate::AccumulatorOpSession,
        encoder: &mut wgpu::CommandEncoder,
        n_bands: u32,
        dt: f32,
        eml: Option<&crate::EmlGpuProgramTable>,
        fast_path: bool,
    ) {
        if fast_path {
            session.encode_orderband_fast_into(
                &self.ctx,
                encoder,
                self.resolved.values(),
                self.resolved.previous_values(),
                n_bands,
                dt,
                eml,
            );
        } else {
            session.encode_orderband_with_eml_into(
                &self.ctx,
                encoder,
                self.resolved.values(),
                self.resolved.previous_values(),
                n_bands,
                dt,
                eml,
            );
        }
    }

    /// Encode the exact constrained product directly after the resident RF
    /// integration band in the caller's existing command encoder.
    ///
    /// The resolved values buffer stays sealed: the apportioner receives the
    /// live `AllocatedFlow` cells without a host readback or copied economic
    /// intermediary. The plan's band is minted by the arena planner.
    pub fn encode_resident_apportionment_into(
        &self,
        session: &mut crate::ResidentApportionmentSession,
        encoder: &mut wgpu::CommandEncoder,
        semantic_rows: &wgpu::Buffer,
        scratch: &wgpu::Buffer,
        plan: &crate::ResidentApportionmentPlan,
    ) -> Result<(), crate::ResidentApportionmentError> {
        self.encode_resident_apportionment_with_dispatch_into(
            session,
            encoder,
            semantic_rows,
            scratch,
            plan,
            crate::ResidentApportionmentDispatch::single_pass(),
        )
    }

    /// Same sealed resident path with a caller-selected physical dispatch
    /// shape. This exists for the binding physical-order proof; dispatch shape
    /// is not carried in the semantic plan or canonical product.
    pub fn encode_resident_apportionment_with_dispatch_into(
        &self,
        session: &mut crate::ResidentApportionmentSession,
        encoder: &mut wgpu::CommandEncoder,
        semantic_rows: &wgpu::Buffer,
        scratch: &wgpu::Buffer,
        plan: &crate::ResidentApportionmentPlan,
        dispatch: crate::ResidentApportionmentDispatch,
    ) -> Result<(), crate::ResidentApportionmentError> {
        session.encode_at_integration_band_with_dispatch(
            &self.ctx,
            encoder,
            self.resolved.values(),
            semantic_rows,
            scratch,
            self.n_slots,
            self.n_dims,
            plan,
            dispatch,
        )
    }

    /// Execute the same exact settlement kernel with same-generation child
    /// supply read directly from immutable parent `T_s.G`.
    #[allow(clippy::too_many_arguments)]
    pub fn encode_resident_apportionment_from_spatial_products_with_dispatch_into(
        &self,
        session: &mut crate::ResidentApportionmentSession,
        encoder: &mut wgpu::CommandEncoder,
        semantic_rows: &wgpu::Buffer,
        scratch: &wgpu::Buffer,
        products: &wgpu::Buffer,
        product_start: u32,
        product_count: u32,
        plan: &crate::ResidentApportionmentPlan,
        dispatch: crate::ResidentApportionmentDispatch,
    ) -> Result<(), crate::ResidentApportionmentError> {
        session.encode_from_spatial_products_at_integration_band_with_dispatch(
            &self.ctx,
            encoder,
            self.resolved.values(),
            semantic_rows,
            scratch,
            products,
            product_start,
            product_count,
            self.n_slots,
            self.n_dims,
            plan,
            dispatch,
        )
    }

    /// Execute generation N+1 through the same exact settlement kernel with
    /// request quantities read from the ordinary resident demand mint.
    #[allow(clippy::too_many_arguments)]
    pub fn encode_resident_apportionment_from_temporal_demands_with_dispatch_into(
        &self,
        session: &mut crate::ResidentApportionmentSession,
        encoder: &mut wgpu::CommandEncoder,
        semantic_rows: &wgpu::Buffer,
        scratch: &wgpu::Buffer,
        demands: &wgpu::Buffer,
        demand_count: u32,
        plan: &crate::ResidentApportionmentPlan,
        dispatch: crate::ResidentApportionmentDispatch,
    ) -> Result<(), crate::ResidentApportionmentError> {
        session.encode_from_temporal_demands_at_integration_band_with_dispatch(
            &self.ctx,
            encoder,
            self.resolved.values(),
            semantic_rows,
            scratch,
            demands,
            demand_count,
            self.n_slots,
            self.n_dims,
            plan,
            dispatch,
        )
    }

    pub fn new(ctx: GpuContext, registry: &DimensionRegistry, n_slots: u32) -> Self {
        assert!(n_slots > 0, "n_slots must be > 0");
        assert!(registry.total_columns > 0, "registry has no columns");

        let n_dims = registry.total_columns as u32;
        let pairs = build_governed_pairs(registry);

        let per_slot_per_col_bytes = (n_slots as u64) * (n_dims as u64) * 4;

        let mk = |label: &'static str, size: u64| -> Buffer {
            ctx.device.create_buffer(&BufferDescriptor {
                label: Some(label),
                size,
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        };

        let values = mk("values", per_slot_per_col_bytes);
        let previous_values = mk("previous_values", per_slot_per_col_bytes);
        let output_vectors = mk("output_vectors", per_slot_per_col_bytes);
        let previous_output_vectors = mk("previous_output_vectors", per_slot_per_col_bytes);

        // Pass 3 buffers — overlay_deltas grows on demand via upload_overlay_deltas.
        // Initial size is one placeholder OverlayDelta so the binding is valid.
        let overlay_deltas = mk("overlay_deltas", std::mem::size_of::<OverlayDelta>() as u64);
        let slot_delta_ranges = mk(
            "slot_delta_ranges",
            (n_slots as u64) * std::mem::size_of::<SlotDeltaRange>() as u64,
        );
        let intent_deltas = mk("intent_deltas", std::mem::size_of::<IntentDelta>() as u64);

        // Always allocate at least one pair's worth so the buffer is bindable
        // even when no governed sub-fields exist. The shader iterates n_governed_pairs,
        // not buffer size, so zero pairs = zero work.
        let n_governed_pairs = pairs.len() as u32;
        let governed_bytes = std::mem::size_of::<GovernedPair>() as u64 * pairs.len().max(1) as u64;
        let governed_pairs = ctx.device.create_buffer(&BufferDescriptor {
            label: Some("governed_pairs"),
            size: governed_bytes,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        if !pairs.is_empty() {
            ctx.queue
                .write_buffer(&governed_pairs, 0, bytemuck::cast_slice(&pairs));
        }

        // Pass 7 buffers — both grow on demand via upload_thresholds.
        // Placeholder allocations keep bindings valid when no thresholds exist.
        let threshold_registry = mk(
            "threshold_registry",
            std::mem::size_of::<ThresholdRegistration>() as u64,
        );
        let threshold_events = ThresholdEventCandidatesReadback::new(
            &ctx.device,
            std::mem::size_of::<ThresholdEventGpu>() as u64,
        );

        let anchor_table = mk(
            "anchor_table",
            std::mem::size_of::<AnchorTableRowGpu>() as u64,
        );

        let remap_shader = ctx.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("anchor_table_remap"),
            source: ShaderSource::Wgsl(include_str!("shaders/anchor_table_remap.wgsl").into()),
        });
        let storage = |binding: u32, read_only: bool| BindGroupLayoutEntry {
            binding,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Storage { read_only },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };
        let anchor_remap_layout = ctx
            .device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("anchor_table_remap_layout"),
                entries: &[
                    storage(0, true),
                    storage(1, true),
                    storage(2, true),
                    storage(3, false),
                    storage(4, false),
                    BindGroupLayoutEntry {
                        binding: 5,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });
        let remap_pl = ctx
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("anchor_table_remap_pl"),
                bind_group_layouts: &[&anchor_remap_layout],
                push_constant_ranges: &[],
            });
        let anchor_remap_pipeline =
            ctx.device
                .create_compute_pipeline(&ComputePipelineDescriptor {
                    label: Some("anchor_table_remap_pipeline"),
                    layout: Some(&remap_pl),
                    module: &remap_shader,
                    entry_point: "apply_anchor_remaps",
                    compilation_options: Default::default(),
                    cache: None,
                });
        let anchor_remap_uniform = ctx.device.create_buffer(&BufferDescriptor {
            label: Some("anchor_table_remap_uniform"),
            size: std::mem::size_of::<AnchorRemapParams>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let magnitude_values_shader = ctx.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("anchor_table_magnitude_values"),
            source: ShaderSource::Wgsl(
                include_str!("shaders/anchor_table_magnitude_values.wgsl").into(),
            ),
        });
        let anchor_magnitude_values_layout =
            ctx.device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("anchor_magnitude_values_layout"),
                    entries: &[
                        storage(0, true),
                        storage(1, false),
                        BindGroupLayoutEntry {
                            binding: 2,
                            visibility: ShaderStages::COMPUTE,
                            ty: BindingType::Buffer {
                                ty: BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                });
        let magnitude_values_pl = ctx
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("anchor_magnitude_values_pl"),
                bind_group_layouts: &[&anchor_magnitude_values_layout],
                push_constant_ranges: &[],
            });
        let anchor_magnitude_values_pipeline =
            ctx.device
                .create_compute_pipeline(&ComputePipelineDescriptor {
                    label: Some("anchor_magnitude_values_pipeline"),
                    layout: Some(&magnitude_values_pl),
                    module: &magnitude_values_shader,
                    entry_point: "maintain_anchor_magnitudes_values_only",
                    compilation_options: Default::default(),
                    cache: None,
                });
        let anchor_magnitude_values_uniform = ctx.device.create_buffer(&BufferDescriptor {
            label: Some("anchor_magnitude_values_uniform"),
            size: 16,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Reduction buffers — placeholder allocations, filled by upload_reduction_topology.
        let child_starts = mk("child_starts", ((n_slots as u64) + 1) * 4);
        let child_indices = mk("child_indices", 4); // placeholder 1 u32
        let column_rules = mk("column_rules", (n_dims as u64) * 8);
        let depth_slots = mk("depth_slots", 4); // placeholder 1 u32

        Self {
            ctx,
            n_slots,
            n_dims,
            n_governed_pairs,
            n_overlay_deltas: 0,
            n_intent_deltas: 0,
            resolved: ResolvedGpuBuffers::new(
                values,
                previous_values,
                output_vectors,
                previous_output_vectors,
            ),
            governed_pairs,
            overlay_deltas,
            slot_delta_ranges,
            intent_deltas,
            threshold_registry,
            threshold_events,
            n_thresholds: 0,
            anchor_table,
            n_anchor_rows: 0,
            anchor_table_generation: 0,
            production_event_egress: simthing_core::StampedEventRing::admit(
                256,
                simthing_core::BackpressurePolicy::OverwriteOldest,
            ),
            child_starts,
            child_indices,
            column_rules,
            depth_slots,
            depth_bucket_ranges: Vec::new(),
            accumulator_runtime: None,
            accumulator_overlay_add_active: false,
            accumulator_overlay_add_bands: 0,
            accumulator_reduction_soft_active: false,
            accumulator_reduction_soft_bands: 0,
            accumulator_reduction_exact_active: false,
            accumulator_velocity_active: false,
            accumulator_velocity_bands: 0,
            accumulator_intensity_eml_active: false,
            accumulator_intensity_eml_bands: 0,
            accumulator_transfer_active: false,
            accumulator_transfer_bands: 0,
            accumulator_emission_active: false,
            accumulator_emission_bands: 0,
            accumulator_resource_flow_active: false,
            accumulator_resource_flow_bands: 0,
            post_rf_full_threshold_regs: Vec::new(),
            post_rf_need_threshold_regs: Vec::new(),
            anchor_remap_pipeline,
            anchor_remap_layout,
            anchor_remap_uniform,
            anchor_magnitude_values_pipeline,
            anchor_magnitude_values_layout,
            anchor_magnitude_values_uniform,
        }
    }

    /// Drop AccumulatorOp runtime so it is recreated after layout changes.
    pub fn clear_accumulator_sessions(&mut self) {
        self.accumulator_runtime = None;
        self.accumulator_overlay_add_active = false;
        self.accumulator_overlay_add_bands = 0;
        self.accumulator_reduction_soft_active = false;
        self.accumulator_reduction_soft_bands = 0;
        self.accumulator_reduction_exact_active = false;
        self.accumulator_velocity_active = false;
        self.accumulator_velocity_bands = 0;
        self.accumulator_intensity_eml_active = false;
        self.accumulator_intensity_eml_bands = 0;
        self.accumulator_transfer_active = false;
        self.accumulator_transfer_bands = 0;
        self.accumulator_emission_active = false;
        self.accumulator_emission_bands = 0;
        self.accumulator_resource_flow_active = false;
        self.accumulator_resource_flow_bands = 0;
    }

    /// Clear one migrated AccumulatorOp family when its feature flag is off.
    pub fn disable_accumulator_family(&mut self, family: crate::OperationFamily) {
        if let Some(runtime) = self.accumulator_runtime.as_mut() {
            match family {
                crate::OperationFamily::Intent => runtime.clear_intent(),
                crate::OperationFamily::Threshold => runtime.clear_threshold(),
                crate::OperationFamily::OverlayAdd | crate::OperationFamily::OverlayOrderBand => {
                    runtime.clear_overlay_orderband();
                    self.set_overlay_add_dispatch(false, 0);
                }
                crate::OperationFamily::ReductionSoft => {
                    runtime.clear_reduction_soft();
                    self.set_reduction_soft_dispatch(false, 0);
                    self.set_reduction_exact_dispatch(false);
                }
                crate::OperationFamily::ReductionExact => {
                    self.set_reduction_exact_dispatch(false);
                }
                crate::OperationFamily::Velocity => {
                    runtime.clear_velocity();
                    self.set_velocity_dispatch(false, 0);
                }
                crate::OperationFamily::EvalEml => {
                    runtime.clear_intensity_eml();
                    self.set_intensity_eml_dispatch(false, 0);
                }
                _ => {}
            }
        } else if matches!(
            family,
            crate::OperationFamily::OverlayAdd | crate::OperationFamily::OverlayOrderBand
        ) {
            self.set_overlay_add_dispatch(false, 0);
        } else if matches!(family, crate::OperationFamily::ReductionSoft) {
            self.set_reduction_soft_dispatch(false, 0);
            self.set_reduction_exact_dispatch(false);
        } else if matches!(family, crate::OperationFamily::ReductionExact) {
            self.set_reduction_exact_dispatch(false);
        } else if matches!(family, crate::OperationFamily::Velocity) {
            self.set_velocity_dispatch(false, 0);
        } else if matches!(family, crate::OperationFamily::EvalEml) {
            self.set_intensity_eml_dispatch(false, 0);
        }
    }

    /// Ensure B-4 summary resources exist for integrated world values.
    pub fn ensure_accumulator_summary_runtime(&mut self) {
        if self.accumulator_runtime.is_none() {
            self.accumulator_runtime = Some(crate::WorldAccumulatorRuntime::new());
        }
        let n_slots = self.n_slots;
        let n_dims = self.n_dims;
        self.accumulator_runtime
            .as_mut()
            .unwrap()
            .ensure_summary(&self.ctx, n_slots, n_dims);
    }

    /// Standalone submit: refresh B-4 summaries from `values`.
    pub fn dispatch_accumulator_world_summary(&mut self) {
        if let Some(runtime) = self.accumulator_runtime.as_mut() {
            runtime.ensure_summary(&self.ctx, self.n_slots, self.n_dims);
            runtime.dispatch_world_summary(&self.ctx, self.resolved.values());
        }
    }

    /// Read B-4 slot summaries for the integrated world path.
    pub fn readback_accumulator_summary(
        &self,
    ) -> Result<Vec<crate::SlotSummary>, crate::AccumulatorOpSessionError> {
        if let Some(runtime) = self.accumulator_runtime.as_ref() {
            runtime.readback_world_summary(&self.ctx)
        } else {
            Ok(Vec::new())
        }
    }

    /// Ensure the C-2 intent AccumulatorOp runtime is enabled.
    pub fn ensure_intent_accumulator(&mut self) {
        if self.accumulator_runtime.is_none() {
            self.accumulator_runtime = Some(crate::WorldAccumulatorRuntime::new());
        }
        let n_slots = self.n_slots;
        let n_dims = self.n_dims;
        self.accumulator_runtime
            .as_mut()
            .unwrap()
            .ensure_intent_session(
                &self.ctx,
                n_slots,
                n_dims,
                DEFAULT_THRESHOLD_EMISSION_CAPACITY,
            );
    }

    /// Upload folded intent deltas to the C-2 AccumulatorOp runtime.
    pub fn upload_accumulator_intents(
        &mut self,
        deltas: &[IntentDelta],
    ) -> Result<(), crate::AccumulatorOpSessionError> {
        if let Some(runtime) = self.accumulator_runtime.as_mut() {
            runtime.upload_intent_ops(&self.ctx, deltas)
        } else {
            Ok(())
        }
    }

    /// Ensure the C-4 overlay OrderBand AccumulatorOp runtime is enabled.
    pub fn ensure_overlay_add_accumulator(&mut self) {
        self.ensure_overlay_accumulator();
    }

    pub fn ensure_overlay_accumulator(&mut self) {
        if self.accumulator_runtime.is_none() {
            self.accumulator_runtime = Some(crate::WorldAccumulatorRuntime::new());
        }
        let n_slots = self.n_slots;
        let n_dims = self.n_dims;
        self.accumulator_runtime
            .as_mut()
            .unwrap()
            .ensure_overlay_session(
                &self.ctx,
                n_slots,
                n_dims,
                DEFAULT_THRESHOLD_EMISSION_CAPACITY,
            );
    }

    /// Upload pre-encoded overlay ops to the C-4 AccumulatorOp runtime.
    pub fn upload_overlay_add_ops(
        &mut self,
        ops: &[crate::AccumulatorOpGpu],
    ) -> Result<(), crate::AccumulatorOpSessionError> {
        self.upload_overlay_add_ops_with_bands(ops, 1)
    }

    /// Upload overlay ops and set OrderBand pass count (C-4 boundary sync).
    pub fn upload_overlay_add_ops_with_bands(
        &mut self,
        ops: &[crate::AccumulatorOpGpu],
        n_bands: u32,
    ) -> Result<(), crate::AccumulatorOpSessionError> {
        self.upload_overlay_ops_with_bands(ops, n_bands)
    }

    pub fn upload_overlay_ops_with_bands(
        &mut self,
        ops: &[crate::AccumulatorOpGpu],
        n_bands: u32,
    ) -> Result<(), crate::AccumulatorOpSessionError> {
        if let Some(runtime) = self.accumulator_runtime.as_mut() {
            runtime.upload_overlay_ops(&self.ctx, ops, n_bands)?;
        }
        self.set_overlay_add_dispatch(!ops.is_empty(), n_bands);
        Ok(())
    }

    pub fn set_overlay_add_dispatch(&mut self, active: bool, n_bands: u32) {
        self.accumulator_overlay_add_active = active;
        self.accumulator_overlay_add_bands = n_bands;
    }

    /// Ensure the C-5 soft-reduction AccumulatorOp runtime is enabled.
    pub fn ensure_reduction_soft_accumulator(&mut self) {
        if self.accumulator_runtime.is_none() {
            self.accumulator_runtime = Some(crate::WorldAccumulatorRuntime::new());
        }
        let n_slots = self.n_slots;
        let n_dims = self.n_dims;
        self.accumulator_runtime
            .as_mut()
            .unwrap()
            .ensure_reduction_soft_session(
                &self.ctx,
                n_slots,
                n_dims,
                self.resolved.output_vectors(),
            );
    }

    /// Upload C-5/C-6 reduction ops and set OrderBand pass count.
    pub fn upload_reduction_soft_ops_with_bands(
        &mut self,
        ops: &[crate::AccumulatorOpGpu],
        n_bands: u32,
    ) -> Result<(), crate::AccumulatorOpSessionError> {
        if let Some(runtime) = self.accumulator_runtime.as_mut() {
            runtime.upload_reduction_soft_ops(&self.ctx, ops, n_bands, true)?;
        }
        let active = !ops.is_empty();
        self.set_reduction_soft_dispatch(active, n_bands);
        self.set_reduction_exact_dispatch(active);
        Ok(())
    }

    pub fn set_reduction_soft_dispatch(&mut self, active: bool, n_bands: u32) {
        self.accumulator_reduction_soft_active = active;
        self.accumulator_reduction_soft_bands = n_bands;
        if !active {
            self.accumulator_reduction_exact_active = false;
        }
    }

    pub fn set_reduction_exact_dispatch(&mut self, active: bool) {
        self.accumulator_reduction_exact_active = active;
    }

    /// Ensure the C-7 velocity AccumulatorOp runtime is enabled.
    pub fn ensure_velocity_accumulator(&mut self) {
        if self.accumulator_runtime.is_none() {
            self.accumulator_runtime = Some(crate::WorldAccumulatorRuntime::new());
        }
        let n_slots = self.n_slots;
        let n_dims = self.n_dims;
        self.accumulator_runtime
            .as_mut()
            .unwrap()
            .ensure_velocity_session(
                &self.ctx,
                n_slots,
                n_dims,
                DEFAULT_THRESHOLD_EMISSION_CAPACITY,
            );
    }

    /// Upload C-7 velocity ops and set dispatch metadata.
    pub fn upload_velocity_ops_with_bands(
        &mut self,
        ops: &[crate::AccumulatorOpGpu],
        n_bands: u32,
    ) -> Result<(), crate::AccumulatorOpSessionError> {
        if let Some(runtime) = self.accumulator_runtime.as_mut() {
            runtime.upload_velocity_ops(&self.ctx, ops, n_bands)?;
        }
        self.set_velocity_dispatch(!ops.is_empty(), n_bands);
        Ok(())
    }

    pub fn set_velocity_dispatch(&mut self, active: bool, n_bands: u32) {
        self.accumulator_velocity_active = active;
        self.accumulator_velocity_bands = n_bands;
    }

    pub fn clear_resource_flow_accumulator(&mut self) {
        if let Some(runtime) = self.accumulator_runtime.as_mut() {
            runtime.clear_resource_flow();
        }
        self.accumulator_resource_flow_active = false;
        self.accumulator_resource_flow_bands = 0;
    }

    pub fn ensure_resource_flow_accumulator(&mut self) {
        if self.accumulator_runtime.is_none() {
            self.accumulator_runtime = Some(crate::WorldAccumulatorRuntime::new());
        }
        let n_slots = self.n_slots;
        let n_dims = self.n_dims;
        self.accumulator_runtime
            .as_mut()
            .unwrap()
            .ensure_resource_flow_session(
                &self.ctx,
                n_slots,
                n_dims,
                DEFAULT_THRESHOLD_EMISSION_CAPACITY,
            );
    }

    /// Upload E-11 resource-flow ops and register supplemental EML formulas.
    pub fn sync_resource_flow_ops_from_cpu(
        &mut self,
        ops: &[simthing_core::AccumulatorOp],
        n_bands: u32,
        supplemental_eml: &simthing_core::EmlExpressionRegistry,
    ) -> Result<(), crate::AccumulatorOpSessionError> {
        if self.accumulator_runtime.is_none() {
            self.accumulator_runtime = Some(crate::WorldAccumulatorRuntime::new());
        }
        {
            let runtime = self.accumulator_runtime.as_mut().unwrap();
            runtime.ensure_eml_program_table(&self.ctx);
            for (id, meta, nodes) in supplemental_eml.formulas_for_gpu_upload() {
                if runtime.eml_registry.get(id).is_none() {
                    runtime
                        .eml_registry
                        .register_formula(id, meta.clone(), nodes.to_vec())
                        .expect("resource-flow EML registration");
                }
            }
            runtime
                .upload_eml_trees(&self.ctx)
                .expect("resource-flow EML upload");
        }
        self.ensure_resource_flow_accumulator();
        let gpu_ops: Vec<crate::AccumulatorOpGpu> = {
            let runtime = self.accumulator_runtime.as_ref().unwrap();
            ops.iter()
                .map(|op| {
                    crate::AccumulatorOpGpu::from_op_with_eml(op, Some(&runtime.eml_registry))
                        .expect("resource-flow op encode")
                })
                .collect()
        };
        if let Some(runtime) = self.accumulator_runtime.as_mut() {
            runtime.upload_resource_flow_ops(&self.ctx, &gpu_ops, n_bands)?;
        }
        self.set_resource_flow_dispatch(!ops.is_empty(), n_bands);
        Ok(())
    }

    /// Upload pre-encoded GPU ops (legacy path when EML already marked uploaded).
    pub fn sync_resource_flow_ops(
        &mut self,
        ops: &[crate::AccumulatorOpGpu],
        n_bands: u32,
        supplemental_eml: &simthing_core::EmlExpressionRegistry,
    ) -> Result<(), crate::AccumulatorOpSessionError> {
        if self.accumulator_runtime.is_none() {
            self.accumulator_runtime = Some(crate::WorldAccumulatorRuntime::new());
        }
        {
            let runtime = self.accumulator_runtime.as_mut().unwrap();
            runtime.ensure_eml_program_table(&self.ctx);
            for (id, meta, nodes) in supplemental_eml.formulas_for_gpu_upload() {
                if runtime.eml_registry.get(id).is_none() {
                    runtime
                        .eml_registry
                        .register_formula(id, meta.clone(), nodes.to_vec())
                        .expect("resource-flow EML registration");
                }
            }
            runtime
                .upload_eml_trees(&self.ctx)
                .expect("resource-flow EML upload");
        }
        self.ensure_resource_flow_accumulator();
        if let Some(runtime) = self.accumulator_runtime.as_mut() {
            runtime.upload_resource_flow_ops(&self.ctx, ops, n_bands)?;
        }
        self.set_resource_flow_dispatch(!ops.is_empty(), n_bands);
        Ok(())
    }

    pub fn set_resource_flow_dispatch(&mut self, active: bool, n_bands: u32) {
        self.accumulator_resource_flow_active = active;
        self.accumulator_resource_flow_bands = n_bands;
    }

    /// Dispatch uploaded E-11 resource-flow OrderBand ops (test/session helper).
    pub fn run_resource_flow_bands(&mut self, n_bands: u32, dt: f32) {
        self.run_resource_flow_bands_with_fast_path(n_bands, dt, false);
    }

    /// AO-WGSL-0: dispatch with fused multi-band fast path when compatible.
    pub fn run_resource_flow_bands_with_fast_path(
        &mut self,
        n_bands: u32,
        dt: f32,
        prefer_fast_path: bool,
    ) {
        if !self.accumulator_resource_flow_active || n_bands == 0 {
            return;
        }
        let Some(mut runtime) = self.accumulator_runtime.take() else {
            return;
        };
        let use_fast = prefer_fast_path
            && crate::accumulator_op::ao_wgsl0_fast_path_compatible(
                runtime.resource_flow_gpu_ops(),
            );
        let Some(mut session) = runtime.take_resource_flow_session() else {
            self.accumulator_runtime = Some(runtime);
            return;
        };
        let eml = runtime.eml_program_table();
        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("resource_flow_bands_encoder"),
            });
        if use_fast {
            session.encode_orderband_fast_into(
                &self.ctx,
                &mut encoder,
                self.resolved.values(),
                self.resolved.previous_values(),
                n_bands,
                dt,
                eml,
            );
        } else {
            session.encode_orderband_with_eml_into(
                &self.ctx,
                &mut encoder,
                self.resolved.values(),
                self.resolved.previous_values(),
                n_bands,
                dt,
                eml,
            );
        }
        self.ctx.queue.submit(Some(encoder.finish()));
        let _ = self.ctx.device.poll(wgpu::Maintain::Wait);
        runtime.restore_resource_flow_session(Some(session));
        self.accumulator_runtime = Some(runtime);
    }

    /// Ensure C-8b intensity EvalEML AccumulatorOp runtime is enabled.
    pub fn ensure_intensity_eml_accumulator(&mut self) {
        if self.accumulator_runtime.is_none() {
            self.accumulator_runtime = Some(crate::WorldAccumulatorRuntime::new());
        }
        let n_slots = self.n_slots;
        let n_dims = self.n_dims;
        self.accumulator_runtime
            .as_mut()
            .unwrap()
            .ensure_intensity_eml_session(
                &self.ctx,
                n_slots,
                n_dims,
                DEFAULT_THRESHOLD_EMISSION_CAPACITY,
            );
    }

    /// Register intensity EML formulas, upload GPU table, and upload EvalEML ops.
    pub fn sync_intensity_eml_accumulator(&mut self, registry: &DimensionRegistry) {
        use crate::intensity_accumulator::plan_intensity_eml_ops;
        if self.accumulator_runtime.is_none() {
            self.accumulator_runtime = Some(crate::WorldAccumulatorRuntime::new());
        }
        {
            let runtime = self.accumulator_runtime.as_mut().unwrap();
            runtime.ensure_eml_program_table(&self.ctx);
            runtime
                .register_intensity_eml_at_boundary(registry)
                .expect("intensity EML formula registration failed");
            runtime
                .upload_eml_trees(&self.ctx)
                .expect("intensity EML program table upload failed");
        }
        self.ensure_intensity_eml_accumulator();
        let entries = crate::build_intensity_eml_entries(registry);
        let ops = plan_intensity_eml_ops(&entries, self.n_slots);
        let n_bands = if ops.is_empty() { 0 } else { 1 };
        if let Some(runtime) = self.accumulator_runtime.as_mut() {
            let signature = crate::IntensityEmlOpPlanSignature {
                eml_registry_generation: runtime.eml_registry.generation(),
                n_slots: self.n_slots,
                n_dims: self.n_dims,
                n_entries: entries.len() as u32,
                n_ops: ops.len() as u32,
                tree_ids: entries.iter().map(|e| e.tree_id.0).collect(),
                intensity_cols: entries
                    .iter()
                    .map(|e| encode_column(e.intensity_col))
                    .collect(),
                velocity_cols: entries
                    .iter()
                    .map(|e| encode_column(e.velocity_col))
                    .collect(),
            };
            runtime
                .upload_intensity_eml_ops(&self.ctx, &ops, n_bands, signature)
                .expect("intensity EvalEML op upload failed");
        }
        self.set_intensity_eml_dispatch(!ops.is_empty(), n_bands);
    }

    pub fn set_intensity_eml_dispatch(&mut self, active: bool, n_bands: u32) {
        self.accumulator_intensity_eml_active = active;
        self.accumulator_intensity_eml_bands = n_bands;
    }

    pub fn ensure_transfer_accumulator(&mut self) {
        if self.accumulator_runtime.is_none() {
            self.accumulator_runtime = Some(crate::WorldAccumulatorRuntime::new());
        }
        let n_slots = self.n_slots;
        let n_dims = self.n_dims;
        self.accumulator_runtime
            .as_mut()
            .unwrap()
            .ensure_transfer_session(
                &self.ctx,
                n_slots,
                n_dims,
                crate::DEFAULT_THRESHOLD_EMISSION_CAPACITY,
            );
    }

    /// Upload input-list table and transfer ops for C-8c.
    pub fn sync_transfer_accumulator(
        &mut self,
        registrations: &[crate::TransferRegistration],
    ) -> Result<(), crate::TransferSyncError> {
        use crate::transfer_accumulator::{encode_transfer_plan, plan_transfer_ops};
        let plan = plan_transfer_ops(registrations)?;
        if self.accumulator_runtime.is_none() {
            self.accumulator_runtime = Some(crate::WorldAccumulatorRuntime::new());
        }
        let source_generation = transfer_registrations_generation(registrations);
        let non_empty_lists: Vec<_> = plan
            .input_lists
            .iter()
            .filter(|l| !l.is_empty())
            .cloned()
            .collect();
        let (input_list_generation, ranges) = {
            let runtime = self.accumulator_runtime.as_mut().unwrap();
            runtime.ensure_input_list_table(&self.ctx);
            let ranges = runtime.input_lists_mut().unwrap().upload_lists(
                &self.ctx,
                &non_empty_lists,
                source_generation,
            )?;
            let gen = runtime.input_lists_ref().unwrap().generation;
            (gen, ranges)
        };
        self.ensure_transfer_accumulator();
        let gpu_ops = encode_transfer_plan(&plan, &ranges)?;
        let mut input_slots = Vec::new();
        let mut input_cols = Vec::new();
        let mut unit_cost_bits = Vec::new();
        for list in &plan.input_lists {
            for inp in list {
                input_slots.push(inp.slot);
                input_cols.push(inp.col);
                unit_cost_bits.push(inp.unit_cost_bits);
            }
        }
        if let Some(runtime) = self.accumulator_runtime.as_mut() {
            let signature = crate::TransferOpPlanSignature {
                n_slots: self.n_slots,
                n_dims: self.n_dims,
                n_ops: gpu_ops.len() as u32,
                n_registrations: registrations.len() as u32,
                input_list_generation,
                input_slots,
                input_cols,
                unit_cost_bits,
                encoded_ops_fingerprint: gpu_ops.iter().fold(
                    0xcbf2_9ce4_8422_2325u64,
                    |mut hash, op| {
                        for byte in bytemuck::bytes_of(op) {
                            hash ^= u64::from(*byte);
                            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
                        }
                        hash
                    },
                ),
            };
            runtime.upload_transfer_ops(&self.ctx, &gpu_ops, plan.n_bands, signature)?;
        }
        self.set_transfer_dispatch(!gpu_ops.is_empty(), plan.n_bands);
        Ok(())
    }

    pub fn set_transfer_dispatch(&mut self, active: bool, n_bands: u32) {
        self.accumulator_transfer_active = active;
        self.accumulator_transfer_bands = n_bands;
    }

    pub fn ensure_emission_accumulator(&mut self) {
        if self.accumulator_runtime.is_none() {
            self.accumulator_runtime = Some(crate::WorldAccumulatorRuntime::new());
        }
        let n_slots = self.n_slots;
        let n_dims = self.n_dims;
        self.accumulator_runtime
            .as_mut()
            .unwrap()
            .ensure_emission_session(
                &self.ctx,
                n_slots,
                n_dims,
                crate::DEFAULT_THRESHOLD_EMISSION_CAPACITY,
            );
    }

    /// Upload emission ops for C-8d.
    pub fn sync_emission_accumulator(
        &mut self,
        registrations: &[crate::EmissionRegistration],
    ) -> Result<(), crate::EmissionSyncError> {
        use crate::emission_accumulator::{
            emission_plan_signature_fields, encode_emission_plan, plan_emission_ops,
            EmissionFormula,
        };

        if registrations.is_empty() {
            if let Some(runtime) = self.accumulator_runtime.as_mut() {
                runtime.clear_emission();
            }
            self.set_emission_dispatch(false, 0);
            return Ok(());
        }

        if self.accumulator_runtime.is_none() {
            self.accumulator_runtime = Some(crate::WorldAccumulatorRuntime::new());
        }

        let needs_eml = registrations
            .iter()
            .any(|r| matches!(r.formula, EmissionFormula::EvalEml { .. }));

        let plan = {
            let runtime = self.accumulator_runtime.as_mut().unwrap();
            if needs_eml {
                runtime.ensure_eml_program_table(&self.ctx);
            }
            plan_emission_ops(registrations, Some(&runtime.eml_registry))?
        };

        if needs_eml {
            self.accumulator_runtime
                .as_mut()
                .unwrap()
                .upload_eml_trees(&self.ctx)?;
        }

        self.ensure_emission_accumulator();
        let gpu_ops = {
            let runtime = self.accumulator_runtime.as_ref().unwrap();
            encode_emission_plan(&plan, Some(&runtime.eml_registry))?
        };
        let (
            source_slots,
            source_cols,
            tree_ids,
            formula_kinds,
            reg_indices,
            constant_value_bits,
            max_emit,
        ) = emission_plan_signature_fields(registrations);
        if let Some(runtime) = self.accumulator_runtime.as_mut() {
            let signature = crate::EmissionOpPlanSignature {
                eml_registry_generation: runtime.eml_registry.generation(),
                n_slots: self.n_slots,
                n_dims: self.n_dims,
                n_registrations: registrations.len() as u32,
                n_ops: gpu_ops.len() as u32,
                source_slots,
                source_cols,
                tree_ids,
                formula_kinds,
                reg_indices,
                constant_value_bits,
                max_emit,
            };
            runtime.upload_emission_ops(&self.ctx, &gpu_ops, plan.n_bands, signature)?;
        }
        self.set_emission_dispatch(!gpu_ops.is_empty(), plan.n_bands);
        Ok(())
    }

    pub fn set_emission_dispatch(&mut self, active: bool, n_bands: u32) {
        self.accumulator_emission_active = active;
        self.accumulator_emission_bands = n_bands;
    }

    /// Ensure the C-1 threshold AccumulatorOp runtime is enabled.
    pub fn ensure_threshold_accumulator(&mut self, emission_capacity: u32) {
        if self.accumulator_runtime.is_none() {
            self.accumulator_runtime = Some(crate::WorldAccumulatorRuntime::new());
        }
        let n_slots = self.n_slots;
        let n_dims = self.n_dims;
        self.accumulator_runtime
            .as_mut()
            .unwrap()
            .ensure_threshold_session(&self.ctx, n_slots, n_dims, emission_capacity);
    }

    /// Upload threshold registrations to the C-1 AccumulatorOp runtime.
    pub fn upload_accumulator_threshold_ops(
        &mut self,
        regs: &[ThresholdRegistration],
    ) -> Result<(), crate::AccumulatorOpSessionError> {
        self.n_thresholds = regs.len() as u32;
        self.post_rf_full_threshold_regs = regs.to_vec();
        if !self.post_rf_need_threshold_regs.is_empty() {
            let keys: std::collections::HashSet<(u32, u32, u32)> =
                regs.iter().map(|r| (r.slot, r.col, r.event_kind)).collect();
            self.post_rf_need_threshold_regs
                .retain(|r| keys.contains(&(r.slot, r.col, r.event_kind)));
        }
        if let Some(runtime) = self.accumulator_runtime.as_mut() {
            runtime.upload_threshold_ops(&self.ctx, regs)
        } else {
            Ok(())
        }
    }

    pub fn configure_overlay_lifecycle_projection(
        &mut self,
        plan: &crate::accumulator_op::OverlayLifecycleProjectionPlan,
    ) -> Result<(), crate::AccumulatorOpSessionError> {
        let runtime = self
            .accumulator_runtime
            .as_mut()
            .ok_or(crate::AccumulatorOpSessionError::NoOps)?;
        runtime.configure_overlay_lifecycle_projection(&self.ctx, plan)
    }

    pub fn freeze_overlay_lifecycle_admission(
        &mut self,
        plan: &crate::accumulator_op::OverlayLifecycleProjectionPlan,
        registrations: &[ThresholdRegistration],
    ) -> Result<(), crate::AccumulatorOpSessionError> {
        let runtime = self
            .accumulator_runtime
            .as_mut()
            .ok_or(crate::AccumulatorOpSessionError::NoOps)?;
        runtime.freeze_overlay_lifecycle_admission(plan, registrations)
    }

    pub fn preflight_overlay_lifecycle_admission(
        &self,
        plan: &crate::accumulator_op::OverlayLifecycleProjectionPlan,
        registrations: &[ThresholdRegistration],
    ) -> Result<(), crate::AccumulatorOpSessionError> {
        let runtime = self
            .accumulator_runtime
            .as_ref()
            .ok_or(crate::AccumulatorOpSessionError::NoOps)?;
        runtime.preflight_overlay_lifecycle_admission(plan, registrations)
    }

    pub fn readback_overlay_lifecycle_states(
        &self,
    ) -> Result<
        Vec<crate::accumulator_op::OverlayLifecycleStateGpu>,
        crate::AccumulatorOpSessionError,
    > {
        let runtime = self
            .accumulator_runtime
            .as_ref()
            .ok_or(crate::AccumulatorOpSessionError::NoOps)?;
        runtime.readback_overlay_lifecycle_states(&self.ctx)
    }

    pub fn set_post_rf_need_threshold_regs(&mut self, regs: Vec<ThresholdRegistration>) {
        self.post_rf_need_threshold_regs = regs;
    }

    /// Need-only append rescan after RF (no prepare wipe of pre-RF events).
    ///
    /// Failures propagate — callers must not treat a silent skip as success.
    pub fn rescan_accumulator_thresholds_after_resource_flow(&mut self) -> Result<(), String> {
        if self.post_rf_need_threshold_regs.is_empty() {
            return Ok(());
        }
        let need = self.post_rf_need_threshold_regs.clone();
        let full = self.post_rf_full_threshold_regs.clone();
        let Some(runtime) = self.accumulator_runtime.as_mut() else {
            return Err("post-RF need threshold rescan requires accumulator_runtime".into());
        };
        let Some(mut session) = runtime.take_threshold_session() else {
            runtime.restore_threshold_session(None);
            return Err(
                "post-RF need threshold rescan requires an active threshold session".into(),
            );
        };
        let need_upload = match crate::PackedThresholdUpload::from_registrations(&need) {
            Ok(u) => u,
            Err(e) => {
                runtime.restore_threshold_session(Some(session));
                return Err(format!("pack need threshold regs: {e}"));
            }
        };
        if let Err(e) = session.upload_packed_threshold_ops(&self.ctx, &need_upload) {
            runtime.restore_threshold_session(Some(session));
            return Err(format!("upload need threshold regs: {e}"));
        }
        // upload_packed_threshold_ops changes n_ops. Refresh the dispatch
        // uniform without resetting the existing per-tick event counter;
        // otherwise retained full-scan ops past the shorter need packet run
        // again under the stale full-scan n_ops value.
        session.prepare_threshold_append_scan(&self.ctx);
        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("post_rf_need_threshold_rescan"),
            });
        session.encode_threshold_scan_with_anchor_maintain_into(
            &self.ctx,
            &mut encoder,
            &self.resolved.values(),
            &self.resolved.previous_values(),
            &self.resolved.output_vectors(),
            &self.resolved.previous_output_vectors(),
            Some((
                &self.anchor_table,
                self.n_anchor_rows,
                self.anchor_table_generation,
            )),
        );
        self.ctx.queue.submit(Some(encoder.finish()));
        session.finish_threshold_scan(&self.ctx);
        match crate::PackedThresholdUpload::from_registrations(&full) {
            Ok(full_upload) => {
                if let Err(e) = session.upload_packed_threshold_ops(&self.ctx, &full_upload) {
                    runtime.restore_threshold_session(Some(session));
                    return Err(format!(
                        "restore full threshold regs after need rescan: {e}"
                    ));
                }
            }
            Err(e) => {
                runtime.restore_threshold_session(Some(session));
                return Err(format!("pack full threshold regs after need rescan: {e}"));
            }
        }
        runtime.restore_threshold_session(Some(session));
        Ok(())
    }

    pub fn append_accumulator_threshold_ops(
        &mut self,
        regs: &[ThresholdRegistration],
    ) -> Result<(), crate::AccumulatorOpSessionError> {
        if let Some(runtime) = self.accumulator_runtime.as_mut() {
            runtime.append_threshold_ops(&self.ctx, regs)
        } else {
            Ok(())
        }
    }

    /// Mutable access to the world EML formula registry (C-8a).
    pub fn eml_registry_mut(&mut self) -> &mut simthing_core::EmlExpressionRegistry {
        if self.accumulator_runtime.is_none() {
            self.accumulator_runtime = Some(crate::WorldAccumulatorRuntime::new());
        }
        &mut self.accumulator_runtime.as_mut().unwrap().eml_registry
    }

    /// Upload registered EML trees to the persistent GPU program table.
    pub fn sync_eml_program_table(&mut self) -> Result<(), crate::EmlUploadError> {
        if self.accumulator_runtime.is_none() {
            self.accumulator_runtime = Some(crate::WorldAccumulatorRuntime::new());
        }
        if let Some(runtime) = self.accumulator_runtime.as_mut() {
            runtime.upload_eml_trees(&self.ctx)
        } else {
            Ok(())
        }
    }

    pub fn eml_generation(&self) -> u64 {
        self.accumulator_runtime
            .as_ref()
            .map(|r| r.eml_generation())
            .unwrap_or(0)
    }

    /// Reallocate every layout-dependent buffer after the registry grows.
    /// Values are uploaded by the boundary sync immediately after this call.
    pub fn rebuild_for_registry(&mut self, registry: &DimensionRegistry) {
        assert!(registry.total_columns > 0, "registry has no columns");
        let n_dims = registry.total_columns as u32;
        if n_dims == self.n_dims {
            self.rebuild_property_buffers(registry);
            return;
        }
        assert!(
            n_dims > self.n_dims,
            "dimension shrink is not supported: {} -> {}",
            self.n_dims,
            n_dims,
        );

        self.n_dims = n_dims;
        let per_slot_per_col_bytes = (self.n_slots as u64) * (self.n_dims as u64) * 4;
        self.resolved
            .set_values(self.mk_storage_buffer("values", per_slot_per_col_bytes));
        self.resolved
            .set_previous_values(self.mk_storage_buffer("previous_values", per_slot_per_col_bytes));
        self.resolved
            .set_output_vectors(self.mk_storage_buffer("output_vectors", per_slot_per_col_bytes));
        self.resolved.set_previous_output_vectors(
            self.mk_storage_buffer("previous_output_vectors", per_slot_per_col_bytes),
        );
        self.slot_delta_ranges = self.mk_storage_buffer(
            "slot_delta_ranges",
            (self.n_slots as u64) * std::mem::size_of::<SlotDeltaRange>() as u64,
        );

        self.overlay_deltas =
            self.mk_storage_buffer("overlay_deltas", std::mem::size_of::<OverlayDelta>() as u64);
        self.n_overlay_deltas = 0;
        self.intent_deltas =
            self.mk_storage_buffer("intent_deltas", std::mem::size_of::<IntentDelta>() as u64);
        self.n_intent_deltas = 0;

        self.rebuild_property_buffers(registry);

        self.clear_accumulator_sessions();

        self.threshold_registry = self.mk_storage_buffer(
            "threshold_registry",
            std::mem::size_of::<ThresholdRegistration>() as u64,
        );
        self.threshold_events = ThresholdEventCandidatesReadback::new(
            &self.ctx.device,
            std::mem::size_of::<ThresholdEventGpu>() as u64,
        );
        self.n_thresholds = 0;

        self.anchor_table = self.mk_storage_buffer(
            "anchor_table",
            std::mem::size_of::<AnchorTableRowGpu>() as u64,
        );
        self.n_anchor_rows = 0;

        // Reduction: column_rules grows with n_dims; child_starts grows with n_slots.
        self.column_rules = self.mk_storage_buffer("column_rules", (self.n_dims as u64) * 8);
        self.child_starts = self.mk_storage_buffer("child_starts", ((self.n_slots as u64) + 1) * 4);
        self.child_indices = self.mk_storage_buffer("child_indices", 4);
        self.depth_slots = self.mk_storage_buffer("depth_slots", 4);
        self.depth_bucket_ranges.clear();
    }

    /// Reallocate every slot-capacity-dependent buffer after tree growth.
    ///
    /// Existing GPU data for slots `[0..old_n_slots]` is preserved across
    /// the resize via a `copy_buffer_to_buffer` on the device queue. Slots
    /// `[old_n_slots..new_n_slots]` are zero-initialized by the new buffer
    /// allocation. The caller only needs to upload data for newly-allocated
    /// slots or slots whose CPU shadow diverged from the GPU between
    /// boundaries (tracked via the dirty-slot list).
    pub fn rebuild_for_slots(&mut self, new_n_slots: u32, registry: &DimensionRegistry) {
        assert!(new_n_slots > 0, "n_slots must be > 0");
        if new_n_slots == self.n_slots {
            self.rebuild_property_buffers(registry);
            return;
        }
        assert!(
            new_n_slots > self.n_slots,
            "slot shrink is not supported: {} -> {}",
            self.n_slots,
            new_n_slots,
        );

        let old_n_slots = self.n_slots;
        let old_n_dims = self.n_dims;
        let new_n_dims = registry.total_columns as u32;

        // We can only preserve GPU contents when n_dims is unchanged. If
        // n_dims shifted (a dimension was added/removed), the column layout
        // of the new buffers does not match the old layout and a CPU-side
        // reseed must follow; in that case fall through to the reset path.
        let preserve = old_n_dims == new_n_dims && old_n_slots > 0;
        let preserve_bytes = if preserve {
            (old_n_slots as u64) * (old_n_dims as u64) * 4
        } else {
            0
        };

        self.n_slots = new_n_slots;
        self.n_dims = new_n_dims;
        self.clear_accumulator_sessions();
        let per_slot_per_col_bytes = (self.n_slots as u64) * (self.n_dims as u64) * 4;

        let new_values = self.mk_storage_buffer("values", per_slot_per_col_bytes);
        let new_previous_values = self.mk_storage_buffer("previous_values", per_slot_per_col_bytes);
        let new_output_vectors = self.mk_storage_buffer("output_vectors", per_slot_per_col_bytes);
        let new_previous_output_vectors =
            self.mk_storage_buffer("previous_output_vectors", per_slot_per_col_bytes);

        if preserve {
            // One encoder copies all four buffers from old → new in a single
            // submit. Cheap: GPU-local memory copy, no CPU round trip.
            let mut encoder =
                self.ctx
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("rebuild_for_slots:preserve"),
                    });
            encoder.copy_buffer_to_buffer(
                self.resolved.values(),
                0,
                &new_values,
                0,
                preserve_bytes,
            );
            encoder.copy_buffer_to_buffer(
                self.resolved.previous_values(),
                0,
                &new_previous_values,
                0,
                preserve_bytes,
            );
            encoder.copy_buffer_to_buffer(
                self.resolved.output_vectors(),
                0,
                &new_output_vectors,
                0,
                preserve_bytes,
            );
            encoder.copy_buffer_to_buffer(
                self.resolved.previous_output_vectors(),
                0,
                &new_previous_output_vectors,
                0,
                preserve_bytes,
            );
            self.ctx.queue.submit(Some(encoder.finish()));
        }

        self.resolved.set_values(new_values);
        self.resolved.set_previous_values(new_previous_values);
        self.resolved.set_output_vectors(new_output_vectors);
        self.resolved
            .set_previous_output_vectors(new_previous_output_vectors);

        // slot_delta_ranges and child_starts are reset — overlay-delta sync
        // and topology sync both fully rewrite them at every active boundary.
        self.slot_delta_ranges = self.mk_storage_buffer(
            "slot_delta_ranges",
            (self.n_slots as u64) * std::mem::size_of::<SlotDeltaRange>() as u64,
        );
        self.child_starts = self.mk_storage_buffer("child_starts", ((self.n_slots as u64) + 1) * 4);
        // column_rules tracks n_dims; slot growth often coincides with registry
        // expansion from spec install (rebuild_for_registry is skipped when slots grow).
        self.column_rules = self.mk_storage_buffer("column_rules", (self.n_dims as u64) * 8);
        if !preserve {
            self.child_indices = self.mk_storage_buffer("child_indices", 4);
            self.depth_slots = self.mk_storage_buffer("depth_slots", 4);
        }

        self.rebuild_property_buffers(registry);

        self.n_overlay_deltas = 0;
        self.n_intent_deltas = 0;
        self.n_thresholds = 0;
        self.n_anchor_rows = 0;
        self.depth_bucket_ranges.clear();
    }

    fn mk_storage_buffer(&self, label: &'static str, size: u64) -> Buffer {
        self.ctx.device.create_buffer(&BufferDescriptor {
            label: Some(label),
            size,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn rebuild_property_buffers(&mut self, registry: &DimensionRegistry) {
        let pairs = build_governed_pairs(registry);
        let pair_bytes = std::mem::size_of::<GovernedPair>() as u64 * pairs.len().max(1) as u64;
        self.governed_pairs = self.mk_storage_buffer("governed_pairs", pair_bytes);
        self.n_governed_pairs = pairs.len() as u32;
        if !pairs.is_empty() {
            self.ctx
                .queue
                .write_buffer(&self.governed_pairs, 0, bytemuck::cast_slice(&pairs));
        }
    }

    /// Upload a fresh batch of per-tick overlay deltas + per-slot ranges.
    /// Reallocates `overlay_deltas` if larger than the current buffer.
    /// `ranges.len()` must equal `n_slots`.
    pub fn upload_overlay_deltas(&mut self, deltas: &[OverlayDelta], ranges: &[SlotDeltaRange]) {
        assert_eq!(
            ranges.len(),
            self.n_slots as usize,
            "ranges length {} != n_slots {}",
            ranges.len(),
            self.n_slots,
        );

        let needed_count = deltas.len().max(1);
        let needed_bytes = (needed_count * std::mem::size_of::<OverlayDelta>()) as u64;
        if needed_bytes > self.overlay_deltas.size() {
            self.overlay_deltas = self.ctx.device.create_buffer(&BufferDescriptor {
                label: Some("overlay_deltas"),
                size: needed_bytes,
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        self.n_overlay_deltas = deltas.len() as u32;
        if !deltas.is_empty() {
            self.ctx
                .queue
                .write_buffer(&self.overlay_deltas, 0, bytemuck::cast_slice(deltas));
        }
        self.ctx
            .queue
            .write_buffer(&self.slot_delta_ranges, 0, bytemuck::cast_slice(ranges));
    }

    /// Upload folded per-tick intent deltas. Empty input clears the active
    /// count while keeping the placeholder buffer bindable.
    pub fn upload_intent_deltas(&mut self, deltas: &[IntentDelta]) {
        let needed_count = deltas.len().max(1);
        let needed_bytes = (needed_count * std::mem::size_of::<IntentDelta>()) as u64;
        if needed_bytes > self.intent_deltas.size() {
            self.intent_deltas = self.ctx.device.create_buffer(&BufferDescriptor {
                label: Some("intent_deltas"),
                size: needed_bytes,
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        self.n_intent_deltas = deltas.len() as u32;
        if !deltas.is_empty() {
            self.ctx
                .queue
                .write_buffer(&self.intent_deltas, 0, bytemuck::cast_slice(deltas));
        }
    }

    /// Upload a fresh set of GPU threshold registrations. Reallocates both
    /// `threshold_registry` and `event_candidates` if larger than the current
    /// capacity. AccumulatorOp threshold scan emits at most one event per
    /// registration, so `event_candidates` is sized to match.
    ///
    /// Empty input is allowed: `n_thresholds` becomes 0 and threshold dispatch
    /// will early-return without scanning.
    pub fn upload_thresholds(&mut self, regs: &[ThresholdRegistration]) {
        let needed_count = regs.len().max(1);
        let reg_bytes = (needed_count * std::mem::size_of::<ThresholdRegistration>()) as u64;
        let event_bytes = (needed_count * std::mem::size_of::<ThresholdEventGpu>()) as u64;

        if reg_bytes > self.threshold_registry.size() {
            self.threshold_registry = self.ctx.device.create_buffer(&BufferDescriptor {
                label: Some("threshold_registry"),
                size: reg_bytes,
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }
        if event_bytes > self.threshold_events.candidates_size() {
            self.threshold_events
                .ensure_candidates_bytes(&self.ctx.device, event_bytes);
        }

        self.n_thresholds = regs.len() as u32;
        if !regs.is_empty() {
            self.ctx
                .queue
                .write_buffer(&self.threshold_registry, 0, bytemuck::cast_slice(regs));
        }
    }

    /// Upload the derived STEAD anchor table POD twin. Reallocates when larger
    /// than current capacity. Empty input clears the live row count.
    /// Crate-internal: POD type is not a public kernel-surface export.
    pub(crate) fn upload_anchor_table(&mut self, rows: &[AnchorTableRowGpu]) {
        let needed_count = rows.len().max(1);
        let bytes = (needed_count * std::mem::size_of::<AnchorTableRowGpu>()) as u64;
        if bytes > self.anchor_table.size() {
            self.anchor_table = self.ctx.device.create_buffer(&BufferDescriptor {
                label: Some("anchor_table"),
                size: bytes,
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }
        self.n_anchor_rows = rows.len() as u32;
        if !rows.is_empty() {
            self.ctx
                .queue
                .write_buffer(&self.anchor_table, 0, bytemuck::cast_slice(rows));
        }
    }

    /// Read back uploaded anchor-table POD rows (GPU/oracle parity).
    /// Crate-internal: POD type is not a public kernel-surface export.
    pub(crate) fn read_anchor_table(&self) -> Vec<AnchorTableRowGpu> {
        if self.n_anchor_rows == 0 {
            return Vec::new();
        }
        let row_size = std::mem::size_of::<AnchorTableRowGpu>();
        let used = row_size * self.n_anchor_rows as usize;
        let bytes = self.read_buffer_bytes(&self.anchor_table);
        bytemuck::cast_slice(&bytes[..used]).to_vec()
    }

    /// Append new registrations at offset `n_thresholds * sizeof(...)` without
    /// disturbing the existing buffer contents. Grows the underlying buffer
    /// via `copy_buffer_to_buffer` when capacity is insufficient, preserving
    /// already-uploaded registrations. Companion to B2 Approach B's
    /// append-only threshold rebuild on pure-fission growth boundaries.
    ///
    /// Caller is responsible for ensuring the CPU `ThresholdRegistry` is
    /// extended in lockstep with the same registrations.
    pub fn append_thresholds(&mut self, new_regs: &[ThresholdRegistration]) {
        if new_regs.is_empty() {
            return;
        }
        let reg_size = std::mem::size_of::<ThresholdRegistration>();
        let event_size = std::mem::size_of::<ThresholdEventGpu>();

        let old_count = self.n_thresholds as u64;
        let new_count = old_count + new_regs.len() as u64;
        let needed_reg_bytes = new_count * reg_size as u64;
        let needed_event_bytes = new_count * event_size as u64;

        // Grow the registry buffer if needed, preserving existing contents.
        if needed_reg_bytes > self.threshold_registry.size() {
            let new_buffer = self.ctx.device.create_buffer(&BufferDescriptor {
                label: Some("threshold_registry"),
                size: needed_reg_bytes.max(reg_size as u64),
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            if old_count > 0 {
                let mut encoder =
                    self.ctx
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("append_thresholds:preserve"),
                        });
                encoder.copy_buffer_to_buffer(
                    &self.threshold_registry,
                    0,
                    &new_buffer,
                    0,
                    old_count * reg_size as u64,
                );
                self.ctx.queue.submit(Some(encoder.finish()));
            }
            self.threshold_registry = new_buffer;
        }

        // Grow the candidates buffer if needed. Contents are scratch (Pass 7
        // writes into it each tick), so no preservation is required.
        if needed_event_bytes > self.threshold_events.candidates_size() {
            self.threshold_events.ensure_candidates_bytes(
                &self.ctx.device,
                needed_event_bytes.max(event_size as u64),
            );
        }

        // Write the new registrations at the tail.
        let offset = old_count * reg_size as u64;
        self.ctx.queue.write_buffer(
            &self.threshold_registry,
            offset,
            bytemuck::cast_slice(new_regs),
        );
        self.n_thresholds = new_count as u32;
    }

    /// Upload reduction topology + per-column rule table. Called once per
    /// boundary after the tree shape changes (or once at session start).
    ///
    /// - `child_starts.len()` must equal `n_slots + 1`.
    /// - `column_rules.len()` must equal `n_dims * 2` (rule kind + weight col per column).
    /// - `depth_bucket_ranges` is stored CPU-side; the dispatcher walks it
    ///   from the last entry (deepest) up to the first (root depth).
    pub fn upload_reduction_topology(
        &mut self,
        child_starts: &[u32],
        child_indices: &[u32],
        column_rules: &[u32],
        depth_slots: &[u32],
        depth_bucket_ranges: Vec<(u32, u32)>,
    ) {
        assert_eq!(
            child_starts.len(),
            self.n_slots as usize + 1,
            "child_starts length {} != n_slots + 1 = {}",
            child_starts.len(),
            self.n_slots as usize + 1,
        );
        assert_eq!(
            column_rules.len(),
            self.n_dims as usize * 2,
            "column_rules length {} != n_dims * 2 = {}",
            column_rules.len(),
            self.n_dims as usize * 2,
        );

        // child_indices grows on demand.
        let ci_needed = (child_indices.len().max(1) * 4) as u64;
        if ci_needed > self.child_indices.size() {
            self.child_indices = self.mk_storage_buffer("child_indices", ci_needed);
        }
        // depth_slots grows on demand.
        let ds_needed = (depth_slots.len().max(1) * 4) as u64;
        if ds_needed > self.depth_slots.size() {
            self.depth_slots = self.mk_storage_buffer("depth_slots", ds_needed);
        }

        self.ctx
            .queue
            .write_buffer(&self.child_starts, 0, bytemuck::cast_slice(child_starts));
        if !child_indices.is_empty() {
            self.ctx.queue.write_buffer(
                &self.child_indices,
                0,
                bytemuck::cast_slice(child_indices),
            );
        }
        self.ctx
            .queue
            .write_buffer(&self.column_rules, 0, bytemuck::cast_slice(column_rules));
        if !depth_slots.is_empty() {
            self.ctx
                .queue
                .write_buffer(&self.depth_slots, 0, bytemuck::cast_slice(depth_slots));
        }
        self.depth_bucket_ranges = depth_bucket_ranges;
    }

    pub fn read_output_vectors(&self) -> Vec<f32> {
        self.read_buffer_f32(self.resolved.output_vectors())
    }

    /// Boundary/admission install of post-reduction output vectors (not a live tick write path).
    pub fn install_resolved_output_vectors_at_boundary(&self, data: &[f32]) {
        assert_eq!(data.len(), self.values_len());
        self.ctx.queue.write_buffer(
            self.resolved.output_vectors(),
            0,
            bytemuck::cast_slice(data),
        );
    }

    /// Reset the per-tick atomic event counter to zero before threshold
    /// AccumulatorOp dispatch.
    pub fn reset_event_count(&self) {
        self.threshold_events.reset_count(&self.ctx.queue);
    }

    /// Read the atomic event counter back to the CPU.
    pub fn read_event_count(&self) -> u32 {
        self.threshold_events
            .read_count(&self.ctx.device, &self.ctx.queue)
    }

    /// Read back exactly `n` `ThresholdEvent`s produced by the most recent
    /// Pass 7 dispatch. Caller is responsible for passing the count read via
    /// `read_event_count()` first (or capping at `n_thresholds`).
    /// Production seal: every event carries the world generation by construction.
    /// Uses `anchor_table_generation` as the tree's generation authority for this
    /// state (same cadence as STEAD table maintenance at the boundary).
    pub fn read_event_candidates(&self, n: u32) -> Vec<ThresholdEvent> {
        self.threshold_events.read_events(
            &self.ctx.device,
            &self.ctx.queue,
            self.n_thresholds,
            n,
            self.anchor_table_generation,
        )
    }

    pub fn values_len(&self) -> usize {
        (self.n_slots * self.n_dims) as usize
    }

    /// Sum of every persistent GPU buffer's size in bytes. Used by VRAM budget
    /// checks and as a sanity signal that buffer sizing matches the design
    /// (agents.md "Transform application — iterative on GPU"). Excludes
    /// short-lived staging buffers and the per-pass uniform buffer.
    pub fn total_buffer_bytes(&self) -> u64 {
        self.resolved.values().size()
            + self.resolved.previous_values().size()
            + self.resolved.output_vectors().size()
            + self.resolved.previous_output_vectors().size()
            + self.governed_pairs.size()
            + self.overlay_deltas.size()
            + self.slot_delta_ranges.size()
            + self.intent_deltas.size()
            + self.threshold_registry.size()
            + self.threshold_events.total_buffer_bytes()
            + self.anchor_table.size()
            + self.child_starts.size()
            + self.child_indices.size()
            + self.column_rules.size()
            + self.depth_slots.size()
    }

    /// Boundary/admission install of resolved values from CPU shadow (not a live tick write path).
    pub fn install_resolved_values_at_boundary(&self, data: &[f32]) {
        self.install_resolved_values_at_boundary_with_auth(
            data,
            ResolvedWriteAuthority::boundary_install(),
        );
    }

    /// Boundary/admission install of a contiguous slot row range from CPU shadow.
    pub fn install_resolved_value_rows_at_boundary(&self, slot_start: u32, rows: &[f32]) {
        if rows.is_empty() {
            return;
        }
        assert_eq!(rows.len() % self.n_dims as usize, 0);
        let offset = (slot_start as u64) * (self.n_dims as u64) * 4;
        self.ctx
            .queue
            .write_buffer(self.resolved.values(), offset, bytemuck::cast_slice(rows));
    }

    fn install_resolved_values_at_boundary_with_auth(
        &self,
        data: &[f32],
        _auth: ResolvedWriteAuthority,
    ) {
        assert_eq!(
            data.len(),
            self.values_len(),
            "values write length {} != n_slots * n_dims = {}",
            data.len(),
            self.values_len()
        );
        self.ctx
            .queue
            .write_buffer(self.resolved.values(), 0, bytemuck::cast_slice(data));
    }

    /// Boundary/admission install of previous-tick resolved values snapshot.
    pub fn install_resolved_previous_values_at_boundary(&self, data: &[f32]) {
        assert_eq!(data.len(), self.values_len());
        self.ctx.queue.write_buffer(
            self.resolved.previous_values(),
            0,
            bytemuck::cast_slice(data),
        );
    }

    pub fn read_values(&self) -> Vec<f32> {
        self.read_buffer_f32(self.resolved.values())
    }

    /// Read one slot's row from the GPU `values` buffer (post-integration).
    pub fn read_values_row(&self, slot: u32) -> Vec<f32> {
        let row_bytes = (self.n_dims as u64) * 4;
        let offset = (slot as u64) * row_bytes;
        let bytes = self.read_buffer_bytes_range(self.resolved.values(), offset, row_bytes);
        bytemuck::cast_slice(&bytes).to_vec()
    }

    pub fn read_previous_values(&self) -> Vec<f32> {
        self.read_buffer_f32(self.resolved.previous_values())
    }

    /// Boundary/admission install of previous-tick post-reduction output snapshot.
    pub fn install_resolved_previous_output_vectors_at_boundary(&self, data: &[f32]) {
        assert_eq!(data.len(), self.values_len());
        self.ctx.queue.write_buffer(
            self.resolved.previous_output_vectors(),
            0,
            bytemuck::cast_slice(data),
        );
    }

    pub fn read_previous_output_vectors(&self) -> Vec<f32> {
        self.read_buffer_f32(self.resolved.previous_output_vectors())
    }

    pub fn read_governed_pairs(&self) -> Vec<GovernedPair> {
        let bytes = self.read_buffer_bytes(&self.governed_pairs);
        if self.n_governed_pairs == 0 {
            return Vec::new();
        }
        let pair_size = std::mem::size_of::<GovernedPair>();
        let used = pair_size * self.n_governed_pairs as usize;
        bytemuck::cast_slice(&bytes[..used]).to_vec()
    }

    fn read_buffer_f32(&self, buf: &Buffer) -> Vec<f32> {
        let bytes = self.read_buffer_bytes(buf);
        bytemuck::cast_slice(&bytes).to_vec()
    }

    fn read_buffer_bytes(&self, buf: &Buffer) -> Vec<u8> {
        self.read_buffer_bytes_range(buf, 0, buf.size())
    }

    fn read_buffer_bytes_range(&self, buf: &Buffer, offset: u64, size: u64) -> Vec<u8> {
        let staging = self.ctx.device.create_buffer(&BufferDescriptor {
            label: Some("staging_read"),
            size,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("read_buffer_encoder"),
            });
        encoder.copy_buffer_to_buffer(buf, offset, &staging, 0, size);
        self.ctx.queue.submit(Some(encoder.finish()));

        let slice = staging.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(MapMode::Read, move |r| {
            let _ = tx.send(r);
        });
        self.ctx.device.poll(Maintain::Wait);
        rx.recv()
            .expect("map_async sender dropped")
            .expect("buffer map failed");

        let mapped = slice.get_mapped_range();
        let out = mapped.to_vec();
        drop(mapped);
        staging.unmap();
        out
    }
}

fn transfer_registrations_generation(regs: &[crate::TransferRegistration]) -> u64 {
    let mut h = 1u64;
    for reg in regs {
        h = h
            .wrapping_mul(31)
            .wrapping_add(reg.target_slot as u64)
            .wrapping_add(encode_column(reg.target_col) as u64)
            .wrapping_add(reg.output_scale.to_bits() as u64);
        if let Some(max) = reg.max_transfer {
            h = h.wrapping_add(max.to_bits() as u64);
        }
        for inp in &reg.inputs {
            h = h
                .wrapping_mul(17)
                .wrapping_add(inp.slot as u64)
                .wrapping_add(encode_column(inp.col) as u64)
                .wrapping_add(inp.unit_cost.to_bits() as u64);
        }
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        ClampBehavior, DimensionRegistry, IntensityBehavior, PropertyLayout, SimProperty,
        SubFieldRole, SubFieldSpec,
    };

    fn try_gpu() -> Option<GpuContext> {
        GpuContext::new_blocking().ok()
    }

    fn property_with_intensity(name: &str) -> SimProperty {
        let mut p = SimProperty::simple("core", name, 0);
        p.intensity_behavior = Some(IntensityBehavior::default());
        p
    }
}
