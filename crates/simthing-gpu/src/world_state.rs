//! WorldGpuState — owns every persistent GPU buffer the simulation reads or writes.
//!
//! Buffer layout follows agents.md:
//!   values, previous_values, output_vectors  : [N_slots × N_dims]      (row-major)
//!   governed_pairs                           : [N_pairs × GovernedPair]      (property-level)
//!   intensity_params                         : [N_int_params × IntensityParams]  (property-level)
//!   overlay_deltas                           : [N_deltas × OverlayDelta]     (per-tick upload)
//!   slot_delta_ranges                        : [N_slots × SlotDeltaRange]    (per-tick upload)
//!
//! Pass 3 reads overlay_deltas via slot_delta_ranges and applies each op
//! iteratively per slot. See agents.md "Transform application — iterative on GPU".
//!
//! Threshold registry / event_candidates buffers are deferred to Pass 7 work.

use bytemuck::{Pod, Zeroable};
use simthing_core::{ClampBehavior, DimensionRegistry, SimPropertyId, SubFieldRole};
use wgpu::{Buffer, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Maintain, MapMode};

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

/// Walk every active property in the registry and emit one GovernedPair per
/// sub-field whose `governed_by` is `Some`. Matches the CPU oracle in
/// `PropertyValue::integrate` — one pair per sub-field, indexed at the first
/// float of each (governed sub-fields are scalar in practice).
pub fn build_governed_pairs(registry: &DimensionRegistry) -> Vec<GovernedPair> {
    let mut pairs = Vec::new();
    for (idx, prop) in registry.properties.iter().enumerate() {
        let id = SimPropertyId(idx as u32);
        if !registry.is_active(id) {
            continue;
        }
        let range = registry.column_range(id);
        let layout = &prop.layout;
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
    }
    pairs
}

// ── IntensityParams — per-property coefficients for Pass 2 ───────────────────

/// One entry per active property that has both `IntensityBehavior` and the
/// required Velocity + Intensity sub-fields in its layout. Pass 2 dispatches
/// one thread per `(slot, intensity_param_idx)`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct IntensityParams {
    pub velocity_col: u32,
    pub intensity_col: u32,
    pub velocity_threshold: f32,
    pub build_coefficient: f32,
    pub decay_coefficient: f32,
    /// Pad to 24 bytes so storage-buffer array stride is unambiguous and
    /// matches the WGSL struct layout.
    pub _pad: u32,
}

/// Walk every active property in the registry and emit one IntensityParams per
/// property whose `intensity_behavior` is `Some` AND whose layout contains both
/// Velocity and Intensity roles. Mirrors the CPU `PropertyValue::update_intensity`
/// short-circuit logic — a property missing either role is silently skipped.
pub fn build_intensity_params(registry: &DimensionRegistry) -> Vec<IntensityParams> {
    let mut params = Vec::new();
    for (idx, prop) in registry.properties.iter().enumerate() {
        let id = SimPropertyId(idx as u32);
        if !registry.is_active(id) {
            continue;
        }
        let Some(behavior) = &prop.intensity_behavior else {
            continue;
        };
        let range = registry.column_range(id);
        let layout = &prop.layout;
        let Some(velocity_col) = range.col_for_role(&SubFieldRole::Velocity, layout) else {
            continue;
        };
        let Some(intensity_col) = range.col_for_role(&SubFieldRole::Intensity, layout) else {
            continue;
        };
        params.push(IntensityParams {
            velocity_col: velocity_col as u32,
            intensity_col: intensity_col as u32,
            velocity_threshold: behavior.velocity_threshold,
            build_coefficient: behavior.build_coefficient,
            decay_coefficient: behavior.decay_coefficient,
            _pad: 0,
        });
    }
    params
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

// ── ThresholdRegistration / ThresholdEvent — Pass 7 ─────────────────────────

pub const DIR_UPWARD: u32 = 0;
pub const DIR_DOWNWARD: u32 = 1;
pub const DIR_EITHER: u32 = 2;

/// One GPU threshold registration. Resolved (slot, col) pair plus the trigger
/// threshold, direction, and an opaque `event_kind` that downstream CPU code
/// interprets (fission stage / decay expiry / velocity warning / etc.).
///
/// `direction`: 0 = `DIR_UPWARD` (prev ≤ t, curr > t), 1 = `DIR_DOWNWARD`
/// (prev ≥ t, curr < t), 2 = `DIR_EITHER` (either crossing).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ThresholdRegistration {
    pub slot: u32,
    pub col: u32,
    pub threshold: f32,
    pub direction: u32,
    pub event_kind: u32,
    pub _pad: u32,
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

pub const RULE_MEAN:  u32 = 0;
pub const RULE_SUM:   u32 = 1;
pub const RULE_MAX:   u32 = 2;
pub const RULE_MIN:   u32 = 3;
pub const RULE_FIRST:         u32 = 4;
pub const RULE_WEIGHTED_MEAN: u32 = 5;

/// Sentinel in the per-column weight slot when the rule is not `WeightedMean`.
pub const WEIGHT_COL_NONE: u32 = u32::MAX;

pub fn encode_rule(rule: simthing_core::ReductionRule) -> u32 {
    use simthing_core::ReductionRule::*;
    match rule {
        Mean  => RULE_MEAN,
        Sum   => RULE_SUM,
        Max   => RULE_MAX,
        Min   => RULE_MIN,
        First => RULE_FIRST,
        WeightedMean { .. } => RULE_WEIGHTED_MEAN,
    }
}

/// Uniform for one reduction dispatch. Drives the depth-bucket pass: each
/// thread processes one slot from `depth_slots[depth_offset .. depth_offset + bucket_size]`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct ReduceParams {
    pub n_dims:       u32,
    pub depth_offset: u32,
    pub bucket_size:  u32,
    pub _pad:         u32,
}

// ── WorldGpuState ─────────────────────────────────────────────────────────────

pub struct WorldGpuState {
    pub ctx: GpuContext,
    pub n_slots: u32,
    pub n_dims: u32,
    pub n_governed_pairs: u32,
    pub n_intensity_params: u32,
    pub n_overlay_deltas: u32,

    /// Current property values, row-major: index = slot * n_dims + col.
    pub values: Buffer,
    /// Snapshot of `values` taken at Pass 0 each tick.
    pub previous_values: Buffer,
    /// Per-slot post-reduction output (Pass 4–6 destination).
    pub output_vectors: Buffer,

    /// Property-level flat buffer of GovernedPair structs. Same pairs apply
    /// to every slot — Pass 1 dispatches `(n_pairs × n_slots)` threads.
    pub governed_pairs: Buffer,

    /// Property-level flat buffer of IntensityParams structs. Pass 2 dispatches
    /// `(n_intensity_params × n_slots)` threads.
    pub intensity_params: Buffer,

    /// Flat per-tick array of overlay deltas, ancestor stack then local, in
    /// evaluation order. Grows as needed via `upload_overlay_deltas`.
    pub overlay_deltas: Buffer,

    /// Per-slot (offset, length) into `overlay_deltas`. Size: `n_slots × 8B`.
    pub slot_delta_ranges: Buffer,

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
    pub child_starts:  Buffer,
    /// Concatenated child slot indices, in canonical (ascending slot) order.
    pub child_indices: Buffer,
    /// Per-column reduction rule (u32), length `n_dims`.
    pub column_rules:  Buffer,
    /// Concatenated depth buckets — slot indices grouped by tree depth.
    /// `depth_bucket_ranges` tells `Pipelines::run_reduction_passes` how to
    /// slice this. Empty when no topology has been uploaded yet.
    pub depth_slots:   Buffer,
    /// (offset, size) into `depth_slots` per depth. The dispatcher iterates
    /// these from the last entry (deepest) to the first (root depth).
    pub depth_bucket_ranges: Vec<(u32, u32)>,
}

impl WorldGpuState {
    pub fn new(ctx: GpuContext, registry: &DimensionRegistry, n_slots: u32) -> Self {
        assert!(n_slots > 0, "n_slots must be > 0");
        assert!(registry.total_columns > 0, "registry has no columns");

        let n_dims = registry.total_columns as u32;
        let pairs = build_governed_pairs(registry);
        let iparams = build_intensity_params(registry);

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

        // Pass 3 buffers — overlay_deltas grows on demand via upload_overlay_deltas.
        // Initial size is one placeholder OverlayDelta so the binding is valid.
        let overlay_deltas = mk("overlay_deltas", std::mem::size_of::<OverlayDelta>() as u64);
        let slot_delta_ranges = mk(
            "slot_delta_ranges",
            (n_slots as u64) * std::mem::size_of::<SlotDeltaRange>() as u64,
        );

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

        // intensity_params: same placeholder-allocation strategy as governed_pairs.
        let n_intensity_params = iparams.len() as u32;
        let iparams_bytes =
            std::mem::size_of::<IntensityParams>() as u64 * iparams.len().max(1) as u64;
        let intensity_params = ctx.device.create_buffer(&BufferDescriptor {
            label: Some("intensity_params"),
            size: iparams_bytes,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        if !iparams.is_empty() {
            ctx.queue
                .write_buffer(&intensity_params, 0, bytemuck::cast_slice(&iparams));
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
        let child_starts = mk(
            "child_starts",
            ((n_slots as u64) + 1) * 4,
        );
        let child_indices = mk("child_indices", 4); // placeholder 1 u32
        let column_rules = mk("column_rules", (n_dims as u64) * 8);
        let depth_slots  = mk("depth_slots", 4);    // placeholder 1 u32

        Self {
            ctx,
            n_slots,
            n_dims,
            n_governed_pairs,
            n_intensity_params,
            n_overlay_deltas: 0,
            values,
            previous_values,
            output_vectors,
            governed_pairs,
            intensity_params,
            overlay_deltas,
            slot_delta_ranges,
            threshold_registry,
            event_count,
            event_candidates,
            n_thresholds: 0,
            child_starts,
            child_indices,
            column_rules,
            depth_slots,
            depth_bucket_ranges: Vec::new(),
        }
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
        self.slot_delta_ranges = self.mk_storage_buffer(
            "slot_delta_ranges",
            (self.n_slots as u64) * std::mem::size_of::<SlotDeltaRange>() as u64,
        );

        self.overlay_deltas =
            self.mk_storage_buffer("overlay_deltas", std::mem::size_of::<OverlayDelta>() as u64);
        self.n_overlay_deltas = 0;

        self.rebuild_property_buffers(registry);

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
        self.child_starts =
            self.mk_storage_buffer("child_starts", ((self.n_slots as u64) + 1) * 4);
        self.child_indices = self.mk_storage_buffer("child_indices", 4);
        self.depth_slots   = self.mk_storage_buffer("depth_slots", 4);
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

        let iparams = build_intensity_params(registry);
        let iparam_bytes =
            std::mem::size_of::<IntensityParams>() as u64 * iparams.len().max(1) as u64;
        self.intensity_params = self.mk_storage_buffer("intensity_params", iparam_bytes);
        self.n_intensity_params = iparams.len() as u32;
        if !iparams.is_empty() {
            self.ctx
                .queue
                .write_buffer(&self.intensity_params, 0, bytemuck::cast_slice(&iparams));
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

    /// Upload a fresh set of GPU threshold registrations. Reallocates both
    /// `threshold_registry` and `event_candidates` if larger than the current
    /// capacity. Pass 7 dispatches one thread per registration, and emits at
    /// most one event per registration, so `event_candidates` is sized to
    /// match.
    ///
    /// Empty input is allowed: `n_thresholds` becomes 0 and `run_threshold_scan`
    /// will early-return without dispatching.
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

    /// Upload reduction topology + per-column rule table. Called once per
    /// boundary after the tree shape changes (or once at session start).
    ///
    /// - `child_starts.len()` must equal `n_slots + 1`.
    /// - `column_rules.len()` must equal `n_dims * 2` (rule kind + weight col per column).
    /// - `depth_bucket_ranges` is stored CPU-side; the dispatcher walks it
    ///   from the last entry (deepest) up to the first (root depth).
    pub fn upload_reduction_topology(
        &mut self,
        child_starts:  &[u32],
        child_indices: &[u32],
        column_rules:  &[u32],
        depth_slots:   &[u32],
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
            self.ctx
                .queue
                .write_buffer(&self.child_indices, 0, bytemuck::cast_slice(child_indices));
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

    /// Reset the per-tick atomic event counter to zero. Call this before each
    /// `run_threshold_scan`; `Pipelines::run_threshold_scan` does it internally.
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
        if n == 0 {
            return Vec::new();
        }
        let bytes = self.read_buffer_bytes(&self.event_candidates);
        let used = (n as usize) * std::mem::size_of::<ThresholdEvent>();
        bytemuck::cast_slice(&bytes[..used]).to_vec()
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
            + self.governed_pairs.size()
            + self.intensity_params.size()
            + self.overlay_deltas.size()
            + self.slot_delta_ranges.size()
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

    pub fn read_previous_values(&self) -> Vec<f32> {
        self.read_buffer_f32(&self.previous_values)
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

    pub fn read_intensity_params(&self) -> Vec<IntensityParams> {
        let bytes = self.read_buffer_bytes(&self.intensity_params);
        if self.n_intensity_params == 0 {
            return Vec::new();
        }
        let p_size = std::mem::size_of::<IntensityParams>();
        let used = p_size * self.n_intensity_params as usize;
        bytemuck::cast_slice(&bytes[..used]).to_vec()
    }

    fn read_buffer_f32(&self, buf: &Buffer) -> Vec<f32> {
        let bytes = self.read_buffer_bytes(buf);
        bytemuck::cast_slice(&bytes).to_vec()
    }

    fn read_buffer_bytes(&self, buf: &Buffer) -> Vec<u8> {
        let size = buf.size();
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
        encoder.copy_buffer_to_buffer(buf, 0, &staging, 0, size);
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

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{DimensionRegistry, IntensityBehavior, SimProperty};

    fn try_gpu() -> Option<GpuContext> {
        GpuContext::new_blocking().ok()
    }

    fn property_with_intensity(name: &str) -> SimProperty {
        let mut p = SimProperty::simple("core", name, 0);
        p.intensity_behavior = Some(IntensityBehavior::default());
        p
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
        assert_eq!(state.n_intensity_params, 1);
        assert_eq!(state.read_values().len(), 12);
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

    #[test]
    fn intensity_params_only_for_properties_with_behavior() {
        // Two properties: only the second has intensity_behavior set.
        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "plain", 0)); // no behavior → skipped
        reg.register(property_with_intensity("loyalty")); // has behavior → included

        let params = build_intensity_params(&reg);
        assert_eq!(params.len(), 1);

        let p = params[0];
        // "loyalty" is the second property: stride 3 each, so its range starts at col 3.
        // Within the standard layout: amount=+0, velocity=+1, intensity=+2.
        assert_eq!(p.velocity_col, 4);
        assert_eq!(p.intensity_col, 5);
        let default = IntensityBehavior::default();
        assert_eq!(p.velocity_threshold, default.velocity_threshold);
        assert_eq!(p.build_coefficient, default.build_coefficient);
        assert_eq!(p.decay_coefficient, default.decay_coefficient);
    }

    #[test]
    fn intensity_params_skip_tombstoned_properties() {
        let mut reg = DimensionRegistry::new();
        let id = reg.register(property_with_intensity("loyalty"));
        assert_eq!(build_intensity_params(&reg).len(), 1);
        reg.tombstone(id);
        assert_eq!(build_intensity_params(&reg).len(), 0);
    }

    #[test]
    fn intensity_params_upload_roundtrip() {
        let Some(ctx) = try_gpu() else {
            eprintln!("skipping: no GPU");
            return;
        };

        let mut reg = DimensionRegistry::new();
        reg.register(property_with_intensity("a"));
        reg.register(property_with_intensity("b"));

        let expected = build_intensity_params(&reg);
        let state = WorldGpuState::new(ctx, &reg, 1);
        let got = state.read_intensity_params();
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
        //   values + previous + output     = 3 × (100 × 8 × 4) = 9600
        //   governed_pairs                 = 1 pair × 24       =   24
        //   intensity_params (placeholder) = 1 × 24            =   24
        //   overlay_deltas (placeholder)   = 1 × 16            =   16
        //   slot_delta_ranges              = 100 × 8           =  800
        //   threshold_registry (placeholder) = 1 × 24          =   24
        //   event_count                    = 4                 =    4
        //   event_candidates (placeholder) = 1 × 16            =   16
        let projected: u64 = 9600 + 24 + 24 + 16 + 800 + 24 + 4 + 16;
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
                _pad: 0,
            },
            ThresholdRegistration {
                slot: 1,
                col: 0,
                threshold: 0.7,
                direction: DIR_UPWARD,
                event_kind: 2,
                _pad: 0,
            },
            ThresholdRegistration {
                slot: 2,
                col: 1,
                threshold: 0.0,
                direction: DIR_EITHER,
                event_kind: 3,
                _pad: 0,
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
