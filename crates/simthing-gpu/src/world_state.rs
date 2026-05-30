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

use bytemuck::{Pod, Zeroable};
use simthing_core::{ClampBehavior, DimensionRegistry, PropertyColumnRange, PropertyLayout, SimPropertyId};
use wgpu::{Buffer, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Maintain, MapMode};

use crate::accumulator_op::DEFAULT_THRESHOLD_EMISSION_CAPACITY;
use crate::context::GpuContext;

// ── GovernedPair — GPU-friendly encoding of a (governed, governing) sub-field pair ──

pub const CLAMP_BOUNDED: u32 = 0;
pub const CLAMP_FLOORED: u32 = 1;
pub const CLAMP_UNBOUNDED: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct GovernedPair {
    pub governed_col: u32,
    pub governing_col: u32,
    pub clamp_min: f32,
    pub clamp_max: f32,
    pub vel_max: f32,
    pub clamp_kind: u32,
}

impl GovernedPair {
    fn encode_clamp(c: &ClampBehavior) -> (u32, f32, f32) {
        match c {
            ClampBehavior::Bounded { min, max } => (CLAMP_BOUNDED, *min, *max),
            ClampBehavior::Floored { min } => (CLAMP_FLOORED, *min, f32::INFINITY),
            ClampBehavior::Unbounded => (CLAMP_UNBOUNDED, f32::NEG_INFINITY, f32::INFINITY),
        }
    }
}

/// Emit one [`GovernedPair`] per sub-field with `governed_by: Some(_)` in `layout`.
///
/// E-7: role-agnostic discovery — supports `(Amount, Velocity)`, `(Named("balance"),
/// Named("flow"))`, and any other declared pair. Skips entries whose governing role
/// is absent from the layout (matches CPU `PropertyValue::integrate`). Invalid
/// `governed_by` links are hard errors at the `simthing-spec` compile layer.
pub fn governed_pairs_for_property(
    range: &PropertyColumnRange,
    layout: &PropertyLayout,
) -> Vec<GovernedPair> {
    let mut pairs = Vec::new();
    for sf in &layout.sub_fields {
        let Some(gov_role) = &sf.governed_by else {
            continue;
        };
        let Some(governed_col) = range.col_for_role(&sf.role, layout) else {
            continue;
        };
        let Some(governing_col) = range.col_for_role(gov_role, layout) else {
            continue;
        };
        let (clamp_kind, clamp_min, clamp_max) = GovernedPair::encode_clamp(&sf.clamp);
        let vel_max = sf.velocity_max.unwrap_or(f32::INFINITY);
        pairs.push(GovernedPair {
            governed_col: governed_col as u32,
            governing_col: governing_col as u32,
            clamp_min,
            clamp_max,
            vel_max,
            clamp_kind,
        });
    }
    pairs
}

/// Walk every active property in the registry and emit one [`GovernedPair`] per
/// governed sub-field. Matches the CPU oracle in `PropertyValue::integrate`.
pub fn build_governed_pairs(registry: &DimensionRegistry) -> Vec<GovernedPair> {
    let mut pairs = Vec::new();
    for (idx, prop) in registry.properties.iter().enumerate() {
        let id = SimPropertyId(idx as u32);
        if !registry.is_active(id) {
            continue;
        }
        pairs.extend(governed_pairs_for_property(
            &registry.column_range(id),
            &prop.layout,
        ));
    }
    pairs
}

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
/// Set/Add/Multiply ops targeting the same cell, while the numeric
/// read-modify-write stays on the GPU.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct IntentDelta {
    pub slot: u32,
    pub col: u32,
    pub mul: f32,
    pub add: f32,
}

// ── ThresholdRegistration / ThresholdEvent — Pass 7 ─────────────────────────

pub const DIR_UPWARD: u32 = 0;
pub const DIR_DOWNWARD: u32 = 1;
pub const DIR_EITHER: u32 = 2;

/// Pass 7 reads crossing state from `values` / `previous_values`.
pub const THRESH_BUF_VALUES: u32 = 0;
/// Pass 7 reads crossing state from `output_vectors` / `previous_output_vectors`
/// (post-reduction aggregates).
pub const THRESH_BUF_OUTPUT: u32 = 1;

/// One GPU threshold registration. Resolved (slot, col) pair plus the trigger
/// threshold, direction, and an opaque `event_kind` that downstream CPU code
/// interprets (fission stage / decay expiry / velocity warning / etc.).
///
/// `direction`: 0 = `DIR_UPWARD` (prev ≤ t, curr > t), 1 = `DIR_DOWNWARD`
/// (prev ≥ t, curr < t), 2 = `DIR_EITHER` (either crossing).
/// `buffer`: `THRESH_BUF_VALUES` or `THRESH_BUF_OUTPUT`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ThresholdRegistration {
    pub slot: u32,
    pub col: u32,
    pub threshold: f32,
    pub direction: u32,
    pub event_kind: u32,
    pub buffer: u32,
}

/// One sparse threshold-crossing event emitted by Pass 7. CPU reads these at
/// the day boundary and maps `event_kind` back to a semantic action.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ThresholdEvent {
    pub slot: u32,
    pub col: u32,
    pub value: f32,
    pub event_kind: u32,
}

// ── Reduction (Passes 4–6) ────────────────────────────────────────────────────

pub const RULE_MEAN: u32 = 0;
pub const RULE_SUM: u32 = 1;
pub const RULE_MAX: u32 = 2;
pub const RULE_MIN: u32 = 3;
pub const RULE_FIRST: u32 = 4;
pub const RULE_WEIGHTED_MEAN: u32 = 5;

/// Sentinel in the per-column weight slot when the rule is not `WeightedMean`.
pub const WEIGHT_COL_NONE: u32 = u32::MAX;

pub fn encode_rule(rule: simthing_core::ReductionRule) -> u32 {
    use simthing_core::ReductionRule::*;
    match rule {
        Mean => RULE_MEAN,
        Sum => RULE_SUM,
        Max => RULE_MAX,
        Min => RULE_MIN,
        First => RULE_FIRST,
        WeightedMean { .. } => RULE_WEIGHTED_MEAN,
    }
}

// ── WorldGpuState ─────────────────────────────────────────────────────────────

pub struct WorldGpuState {
    pub ctx: GpuContext,
    pub n_slots: u32,
    pub n_dims: u32,
    pub n_governed_pairs: u32,
    pub n_overlay_deltas: u32,
    pub n_intent_deltas: u32,

    /// Current property values, row-major: index = slot * n_dims + col.
    pub values: Buffer,
    /// Snapshot of `values` taken at Pass 0 each tick.
    pub previous_values: Buffer,
    /// Per-slot post-reduction output (Pass 4–6 destination).
    pub output_vectors: Buffer,
    /// Snapshot of `output_vectors` taken at Pass 0 each tick (before this
    /// tick's reduction overwrites aggregates). Used by output-buffer Pass 7
    /// registrations.
    pub previous_output_vectors: Buffer,

    /// Property-level flat buffer of GovernedPair structs. Same pairs apply
    /// to every slot — Pass 1 dispatches `(n_pairs × n_slots)` threads.
    pub governed_pairs: Buffer,

    /// Flat per-tick array of overlay deltas, ancestor stack then local, in
    /// evaluation order. Grows as needed via `upload_overlay_deltas`.
    pub overlay_deltas: Buffer,

    /// Per-slot (offset, length) into `overlay_deltas`. Size: `n_slots × 8B`.
    pub slot_delta_ranges: Buffer,

    /// Flat per-tick array of folded player/AI/feeder intent deltas. Grows as
    /// needed via `upload_intent_deltas`.
    pub intent_deltas: Buffer,

    /// Pass 7 inputs: flat array of ThresholdRegistration structs.
    /// Grows on demand via `upload_thresholds`.
    pub threshold_registry: Buffer,
    /// Pass 7 outputs: 4-byte atomic counter (`u32`) reset at each tick.
    pub event_count: Buffer,
    /// Pass 7 outputs: flat array of ThresholdEvent slots (sparse).
    /// Capacity grows to match `n_thresholds` on upload.
    pub event_candidates: Buffer,

    /// Number of currently-registered thresholds (i.e. valid entries in
    /// `threshold_registry`). Pass 7 dispatches one thread per registration.
    pub n_thresholds: u32,

    // ── Reduction (Passes 4–6) ───────────────────────────────────────────────
    /// CSR child topology: `child_starts[i]..child_starts[i+1]` indexes
    /// children of parent slot `i`. Length `n_slots + 1` u32s.
    pub child_starts: Buffer,
    /// Concatenated child slot indices, in canonical (ascending slot) order.
    pub child_indices: Buffer,
    /// Per-column reduction rule (u32), length `n_dims`.
    pub column_rules: Buffer,
    /// Concatenated depth buckets — slot indices grouped by tree depth.
    /// `depth_bucket_ranges` tells AccumulatorOp reduction encoding how to
    /// slice this. Empty when no topology has been uploaded yet.
    pub depth_slots: Buffer,
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
}

impl WorldGpuState {
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
        let event_candidates = mk(
            "event_candidates",
            std::mem::size_of::<ThresholdEvent>() as u64,
        );
        // event_count is always exactly 4 bytes — an atomic<u32> reset per tick.
        let event_count = ctx.device.create_buffer(&BufferDescriptor {
            label: Some("event_count"),
            size: 4,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
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
            values,
            previous_values,
            output_vectors,
            previous_output_vectors,
            governed_pairs,
            overlay_deltas,
            slot_delta_ranges,
            intent_deltas,
            threshold_registry,
            event_count,
            event_candidates,
            n_thresholds: 0,
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
            runtime.dispatch_world_summary(&self.ctx, &self.values);
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
            .ensure_reduction_soft_session(&self.ctx, n_slots, n_dims, &self.output_vectors);
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
            && crate::accumulator_op::ao_wgsl0_fast_path_compatible(runtime.resource_flow_gpu_ops());
        let Some(mut session) = runtime.take_resource_flow_session() else {
            self.accumulator_runtime = Some(runtime);
            return;
        };
        let eml = runtime.eml_bind_buffers();
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
                &self.values,
                &self.previous_values,
                n_bands,
                dt,
                eml,
            );
        } else {
            session.encode_orderband_with_eml_into(
                &self.ctx,
                &mut encoder,
                &self.values,
                &self.previous_values,
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
                intensity_cols: entries.iter().map(|e| e.intensity_col).collect(),
                velocity_cols: entries.iter().map(|e| e.velocity_col).collect(),
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
            let ranges = runtime.input_lists.as_mut().unwrap().upload_lists(
                &self.ctx,
                &non_empty_lists,
                source_generation,
            )?;
            let gen = runtime.input_lists.as_ref().unwrap().generation;
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
        if let Some(runtime) = self.accumulator_runtime.as_mut() {
            runtime.upload_threshold_ops(&self.ctx, regs)
        } else {
            Ok(())
        }
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
        self.values = self.mk_storage_buffer("values", per_slot_per_col_bytes);
        self.previous_values = self.mk_storage_buffer("previous_values", per_slot_per_col_bytes);
        self.output_vectors = self.mk_storage_buffer("output_vectors", per_slot_per_col_bytes);
        self.previous_output_vectors =
            self.mk_storage_buffer("previous_output_vectors", per_slot_per_col_bytes);
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
        self.event_candidates = self.mk_storage_buffer(
            "event_candidates",
            std::mem::size_of::<ThresholdEvent>() as u64,
        );
        self.event_count = self.mk_storage_buffer("event_count", 4);
        self.n_thresholds = 0;

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
            encoder.copy_buffer_to_buffer(&self.values, 0, &new_values, 0, preserve_bytes);
            encoder.copy_buffer_to_buffer(
                &self.previous_values,
                0,
                &new_previous_values,
                0,
                preserve_bytes,
            );
            encoder.copy_buffer_to_buffer(
                &self.output_vectors,
                0,
                &new_output_vectors,
                0,
                preserve_bytes,
            );
            encoder.copy_buffer_to_buffer(
                &self.previous_output_vectors,
                0,
                &new_previous_output_vectors,
                0,
                preserve_bytes,
            );
            self.ctx.queue.submit(Some(encoder.finish()));
        }

        self.values = new_values;
        self.previous_values = new_previous_values;
        self.output_vectors = new_output_vectors;
        self.previous_output_vectors = new_previous_output_vectors;

        // slot_delta_ranges and child_starts are reset — overlay-delta sync
        // and topology sync both fully rewrite them at every active boundary.
        self.slot_delta_ranges = self.mk_storage_buffer(
            "slot_delta_ranges",
            (self.n_slots as u64) * std::mem::size_of::<SlotDeltaRange>() as u64,
        );
        self.child_starts = self.mk_storage_buffer("child_starts", ((self.n_slots as u64) + 1) * 4);

        self.rebuild_property_buffers(registry);

        self.n_overlay_deltas = 0;
        self.n_intent_deltas = 0;
        self.n_thresholds = 0;
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
        let event_bytes = (needed_count * std::mem::size_of::<ThresholdEvent>()) as u64;

        if reg_bytes > self.threshold_registry.size() {
            self.threshold_registry = self.ctx.device.create_buffer(&BufferDescriptor {
                label: Some("threshold_registry"),
                size: reg_bytes,
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }
        if event_bytes > self.event_candidates.size() {
            self.event_candidates = self.ctx.device.create_buffer(&BufferDescriptor {
                label: Some("event_candidates"),
                size: event_bytes,
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        self.n_thresholds = regs.len() as u32;
        if !regs.is_empty() {
            self.ctx
                .queue
                .write_buffer(&self.threshold_registry, 0, bytemuck::cast_slice(regs));
        }
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
        let event_size = std::mem::size_of::<ThresholdEvent>();

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
        if needed_event_bytes > self.event_candidates.size() {
            self.event_candidates = self.ctx.device.create_buffer(&BufferDescriptor {
                label: Some("event_candidates"),
                size: needed_event_bytes.max(event_size as u64),
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
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
        self.read_buffer_f32(&self.output_vectors)
    }

    pub fn write_output_vectors(&self, data: &[f32]) {
        assert_eq!(data.len(), self.values_len());
        self.ctx
            .queue
            .write_buffer(&self.output_vectors, 0, bytemuck::cast_slice(data));
    }

    /// Reset the per-tick atomic event counter to zero before threshold
    /// AccumulatorOp dispatch.
    pub fn reset_event_count(&self) {
        self.ctx
            .queue
            .write_buffer(&self.event_count, 0, &0u32.to_le_bytes());
    }

    /// Read the atomic event counter back to the CPU.
    pub fn read_event_count(&self) -> u32 {
        let bytes = self.read_buffer_bytes(&self.event_count);
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }

    /// Read back exactly `n` `ThresholdEvent`s produced by the most recent
    /// Pass 7 dispatch. Caller is responsible for passing the count read via
    /// `read_event_count()` first (or capping at `n_thresholds`).
    pub fn read_event_candidates(&self, n: u32) -> Vec<ThresholdEvent> {
        let n = n.min(self.n_thresholds);
        if n == 0 {
            return Vec::new();
        }
        let used = (n as usize) * std::mem::size_of::<ThresholdEvent>();
        let bytes = self.read_buffer_bytes_range(&self.event_candidates, 0, used as u64);
        bytemuck::cast_slice(&bytes).to_vec()
    }

    pub fn values_len(&self) -> usize {
        (self.n_slots * self.n_dims) as usize
    }

    /// Sum of every persistent GPU buffer's size in bytes. Used by VRAM budget
    /// checks and as a sanity signal that buffer sizing matches the design
    /// (agents.md "Transform application — iterative on GPU"). Excludes
    /// short-lived staging buffers and the per-pass uniform buffer.
    pub fn total_buffer_bytes(&self) -> u64 {
        self.values.size()
            + self.previous_values.size()
            + self.output_vectors.size()
            + self.previous_output_vectors.size()
            + self.governed_pairs.size()
            + self.overlay_deltas.size()
            + self.slot_delta_ranges.size()
            + self.intent_deltas.size()
            + self.threshold_registry.size()
            + self.event_count.size()
            + self.event_candidates.size()
            + self.child_starts.size()
            + self.child_indices.size()
            + self.column_rules.size()
            + self.depth_slots.size()
    }

    pub fn write_values(&self, data: &[f32]) {
        assert_eq!(
            data.len(),
            self.values_len(),
            "values write length {} != n_slots * n_dims = {}",
            data.len(),
            self.values_len()
        );
        self.ctx
            .queue
            .write_buffer(&self.values, 0, bytemuck::cast_slice(data));
    }

    pub fn write_previous_values(&self, data: &[f32]) {
        assert_eq!(data.len(), self.values_len());
        self.ctx
            .queue
            .write_buffer(&self.previous_values, 0, bytemuck::cast_slice(data));
    }

    pub fn read_values(&self) -> Vec<f32> {
        self.read_buffer_f32(&self.values)
    }

    /// Read one slot's row from the GPU `values` buffer (post-integration).
    pub fn read_values_row(&self, slot: u32) -> Vec<f32> {
        let row_bytes = (self.n_dims as u64) * 4;
        let offset = (slot as u64) * row_bytes;
        let bytes = self.read_buffer_bytes_range(&self.values, offset, row_bytes);
        bytemuck::cast_slice(&bytes).to_vec()
    }

    pub fn read_previous_values(&self) -> Vec<f32> {
        self.read_buffer_f32(&self.previous_values)
    }

    pub fn write_previous_output_vectors(&self, data: &[f32]) {
        assert_eq!(data.len(), self.values_len());
        self.ctx
            .queue
            .write_buffer(&self.previous_output_vectors, 0, bytemuck::cast_slice(data));
    }

    pub fn read_previous_output_vectors(&self) -> Vec<f32> {
        self.read_buffer_f32(&self.previous_output_vectors)
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
            .wrapping_add(reg.target_col as u64)
            .wrapping_add(reg.output_scale.to_bits() as u64);
        if let Some(max) = reg.max_transfer {
            h = h.wrapping_add(max.to_bits() as u64);
        }
        for inp in &reg.inputs {
            h = h
                .wrapping_mul(17)
                .wrapping_add(inp.slot as u64)
                .wrapping_add(inp.col as u64)
                .wrapping_add(inp.unit_cost.to_bits() as u64);
        }
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{ClampBehavior, DimensionRegistry, IntensityBehavior, PropertyLayout, SimProperty, SubFieldRole, SubFieldSpec};

    fn try_gpu() -> Option<GpuContext> {
        GpuContext::new_blocking().ok()
    }

    fn property_with_intensity(name: &str) -> SimProperty {
        let mut p = SimProperty::simple("core", name, 0);
        p.intensity_behavior = Some(IntensityBehavior::default());
        p
    }

    #[test]
    fn governed_pairs_named_balance_flow() {
        let layout = PropertyLayout {
            sub_fields: vec![
                SubFieldSpec {
                    role: SubFieldRole::Named("flow".into()),
                    width: 1,
                    clamp: ClampBehavior::Unbounded,
                    velocity_max: None,
                    default: 0.0,
                    display_name: "flow".into(),
                    display_range: None,
                    governed_by: None,
                    reduction_override: None,
                    soft_aggregate_guard: None,
                    accumulator_spec: None,
                },
                SubFieldSpec {
                    role: SubFieldRole::Named("balance".into()),
                    width: 1,
                    clamp: ClampBehavior::Bounded {
                        min: 0.0,
                        max: 1000.0,
                    },
                    velocity_max: Some(5.0),
                    default: 0.0,
                    display_name: "balance".into(),
                    display_range: None,
                    governed_by: Some(SubFieldRole::Named("flow".into())),
                    reduction_override: None,
                    soft_aggregate_guard: None,
                    accumulator_spec: None,
                },
            ],
        };
        let range = PropertyColumnRange { start: 10, stride: 2 };
        let pairs = governed_pairs_for_property(&range, &layout);
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0].governed_col, 11);
        assert_eq!(pairs[0].governing_col, 10);
        assert_eq!(pairs[0].vel_max, 5.0);
    }

    #[test]
    fn governed_pairs_skip_missing_governing_role() {
        let layout = PropertyLayout {
            sub_fields: vec![SubFieldSpec {
                role: SubFieldRole::Named("balance".into()),
                width: 1,
                clamp: ClampBehavior::Unbounded,
                velocity_max: None,
                default: 0.0,
                display_name: "balance".into(),
                display_range: None,
                governed_by: Some(SubFieldRole::Named("flow".into())),
                reduction_override: None,
                soft_aggregate_guard: None,
                    accumulator_spec: None,
            }],
        };
        let range = PropertyColumnRange { start: 0, stride: 1 };
        assert!(governed_pairs_for_property(&range, &layout).is_empty());
    }

    #[test]
    fn governed_pairs_from_standard_layout() {
        // standard(0): amount governed_by velocity; intensity has no governor.
        // Expect one pair: governed=0 (amount), governing=1 (velocity).
        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 0));

        let pairs = build_governed_pairs(&reg);
        assert_eq!(pairs.len(), 1);
        let p = pairs[0];
        assert_eq!(p.governed_col, 0);
        assert_eq!(p.governing_col, 1);
        assert_eq!(p.clamp_kind, CLAMP_BOUNDED);
        assert_eq!(p.clamp_min, 0.0);
        assert_eq!(p.clamp_max, 1.0);
        assert_eq!(p.vel_max, f32::INFINITY);
    }

    #[test]
    fn governed_pairs_skip_tombstoned_properties() {
        let mut reg = DimensionRegistry::new();
        let id = reg.register(SimProperty::simple("core", "loyalty", 0));
        assert_eq!(build_governed_pairs(&reg).len(), 1);
        reg.tombstone(id);
        assert_eq!(build_governed_pairs(&reg).len(), 0);
    }

    #[test]
    fn governed_pairs_offset_across_multiple_properties() {
        // Two properties, each contributing one governed pair. The second
        // property's columns must offset by the first's stride (3).
        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 0)); // stride 3
        reg.register(SimProperty::simple("core", "food_security", 0)); // stride 3

        let pairs = build_governed_pairs(&reg);
        assert_eq!(pairs.len(), 2);
        assert_eq!(pairs[0].governed_col, 0);
        assert_eq!(pairs[0].governing_col, 1);
        assert_eq!(pairs[1].governed_col, 3);
        assert_eq!(pairs[1].governing_col, 4);
    }

    #[test]
    fn write_read_values_roundtrip() {
        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };

        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 2)); // stride 5

        let state = WorldGpuState::new(ctx, &reg, 4);
        assert_eq!(state.n_dims, 5);
        assert_eq!(state.n_slots, 4);
        assert_eq!(state.n_governed_pairs, 1);
        assert_eq!(state.values_len(), 20);

        let input: Vec<f32> = (0..20).map(|i| i as f32 * 0.1).collect();
        state.write_values(&input);
        let output = state.read_values();

        for (i, (a, b)) in input.iter().zip(output.iter()).enumerate() {
            assert_eq!(
                a.to_bits(),
                b.to_bits(),
                "mismatch at index {i}: {a} vs {b}"
            );
        }
    }

    #[test]
    fn c2_registry_growth_recreates_accumulator_sessions() {
        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };

        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 0));
        let mut state = WorldGpuState::new(ctx, &reg, 2);
        state.ensure_intent_accumulator();
        state.ensure_threshold_accumulator(4096);
        state.ensure_overlay_add_accumulator();
        assert!(state.accumulator_runtime.as_ref().unwrap().intent_active());
        assert!(state
            .accumulator_runtime
            .as_ref()
            .unwrap()
            .threshold_active());

        reg.register(property_with_intensity("food_security"));
        state.rebuild_for_registry(&reg);

        assert!(state.accumulator_runtime.is_none());
    }

    #[test]
    fn rebuild_for_registry_expands_layout_buffers() {
        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };

        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 0));
        let mut state = WorldGpuState::new(ctx, &reg, 2);
        assert_eq!(state.n_dims, 3);
        assert_eq!(state.values_len(), 6);

        reg.register(property_with_intensity("food_security"));
        state.rebuild_for_registry(&reg);

        assert_eq!(state.n_dims, 6);
        assert_eq!(state.values_len(), 12);
        assert_eq!(state.n_governed_pairs, 2);
        assert_eq!(state.read_values().len(), 12);
    }

    #[test]
    fn rebuild_for_slots_expands_slot_dependent_buffers() {
        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };

        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 0));
        let mut state = WorldGpuState::new(ctx, &reg, 2);

        state.rebuild_for_slots(8, &reg);

        assert_eq!(state.n_slots, 8);
        assert_eq!(state.n_dims, 3);
        assert_eq!(state.values_len(), 24);
        assert_eq!(state.read_values().len(), 24);
        assert!(state.slot_delta_ranges.size() >= 8 * std::mem::size_of::<SlotDeltaRange>() as u64);
        assert!(state.child_starts.size() >= 9 * 4);
    }

    #[test]
    fn governed_pairs_upload_roundtrip() {
        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };

        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 0));
        reg.register(SimProperty::simple("core", "food_security", 0));

        let expected = build_governed_pairs(&reg);
        let state = WorldGpuState::new(ctx, &reg, 1);
        let got = state.read_governed_pairs();
        assert_eq!(got, expected);
    }

    /// Week 2 success criterion: VRAM usage at 100 SimThings, 8 dimensions must
    /// be within 5% of the projected budget. Verifies that buffer sizing matches
    /// the iterative-on-GPU buffer plan (no matrix buffers, deltas + ranges only).
    #[test]
    fn vram_budget_at_100_slots_8_dims() {
        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };

        // standard(5): stride = 3 + 5 = 8 → exactly 8 dims with one property.
        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 5));

        let state = WorldGpuState::new(ctx, &reg, 100);
        assert_eq!(state.n_dims, 8);
        assert_eq!(state.n_slots, 100);

        // Projected layout (bytes):
        //   values + previous + output + previous_output = 4 × (100 × 8 × 4) = 12800
        //   governed_pairs                 = 1 pair × 24       =   24
        //   overlay_deltas (placeholder)   = 1 × 16            =   16
        //   slot_delta_ranges              = 100 × 8           =  800
        //   threshold_registry (placeholder) = 1 × 24          =   24
        //   event_count                    = 4                 =    4
        //   event_candidates (placeholder) = 1 × 16            =   16
        //   child_starts                   = 101 × 4           =  404
        //   child_indices (placeholder)  = 4                 =    4
        //   column_rules                 = 8 × 8             =   64
        //   depth_slots (placeholder)    = 4                 =    4
        let projected: u64 = 12800 + 24 + 16 + 800 + 24 + 4 + 16 + 404 + 4 + 64 + 4;
        let actual = state.total_buffer_bytes();

        let diff = actual as i64 - projected as i64;
        let pct = (diff.unsigned_abs() as f64) / (projected as f64);
        assert!(
            pct < 0.05,
            "VRAM {actual} B diverges from projection {projected} B by {:.1}% (>5%)",
            pct * 100.0,
        );
    }

    #[test]
    fn upload_thresholds_grows_buffer_and_tracks_count() {
        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };
        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 0));
        let mut state = WorldGpuState::new(ctx, &reg, 4);

        // Starts with the placeholder allocation (1 × 24 B).
        assert_eq!(state.n_thresholds, 0);
        let initial = state.threshold_registry.size();
        assert_eq!(initial, std::mem::size_of::<ThresholdRegistration>() as u64);

        // Upload 3 thresholds — buffer must grow.
        let regs = vec![
            ThresholdRegistration {
                slot: 0,
                col: 0,
                threshold: 0.3,
                direction: DIR_DOWNWARD,
                event_kind: 1,
                buffer: THRESH_BUF_VALUES,
            },
            ThresholdRegistration {
                slot: 1,
                col: 0,
                threshold: 0.7,
                direction: DIR_UPWARD,
                event_kind: 2,
                buffer: THRESH_BUF_VALUES,
            },
            ThresholdRegistration {
                slot: 2,
                col: 1,
                threshold: 0.0,
                direction: DIR_EITHER,
                event_kind: 3,
                buffer: THRESH_BUF_VALUES,
            },
        ];
        state.upload_thresholds(&regs);
        assert_eq!(state.n_thresholds, 3);
        assert!(
            state.threshold_registry.size()
                >= 3 * std::mem::size_of::<ThresholdRegistration>() as u64
        );
        assert!(state.event_candidates.size() >= 3 * std::mem::size_of::<ThresholdEvent>() as u64);
    }

    #[test]
    fn reset_event_count_writes_zero() {
        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };
        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 0));
        let state = WorldGpuState::new(ctx, &reg, 1);

        // Write a sentinel non-zero value first, then reset, then read back.
        state
            .ctx
            .queue
            .write_buffer(&state.event_count, 0, &42u32.to_le_bytes());
        state.reset_event_count();
        assert_eq!(state.read_event_count(), 0);
    }

    #[test]
    fn empty_governed_pairs_buffer_is_bindable() {
        // A property with no governed sub-fields still produces a usable
        // WorldGpuState (governed_pairs buffer has a placeholder allocation).
        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };

        let mut reg = DimensionRegistry::new();
        let id = reg.register(SimProperty::simple("core", "loyalty", 0));
        reg.tombstone(id);
        // tombstoned but total_columns > 0, so registry still has dimensions.

        let state = WorldGpuState::new(ctx, &reg, 2);
        assert_eq!(state.n_governed_pairs, 0);
        assert_eq!(state.read_governed_pairs().len(), 0);
    }
}
