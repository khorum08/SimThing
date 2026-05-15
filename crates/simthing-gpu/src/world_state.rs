//! WorldGpuState — owns every persistent GPU buffer the simulation reads or writes.
//!
//! Buffer layout follows agents.md:
//!   values, previous_values, output_vectors  : [N_slots × N_dims]      (row-major)
//!   local_transforms, ancestor_transforms    : [N_slots × N_dims × N_dims]
//!   governed_pairs                           : [N_pairs × GovernedPair]      (property-level)
//!   intensity_params                         : [N_int_params × IntensityParams]  (property-level)
//!
//! Threshold registry / event_candidates buffers are deferred to Pass 7 work.

use bytemuck::{Pod, Zeroable};
use simthing_core::{ClampBehavior, DimensionRegistry, SimPropertyId, SubFieldRole};
use wgpu::{Buffer, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Maintain,
           MapMode};

use crate::context::GpuContext;

// ── GovernedPair — GPU-friendly encoding of a (governed, governing) sub-field pair ──

pub const CLAMP_BOUNDED:   u32 = 0;
pub const CLAMP_FLOORED:   u32 = 1;
pub const CLAMP_UNBOUNDED: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct GovernedPair {
    pub governed_col:  u32,
    pub governing_col: u32,
    pub clamp_min:     f32,
    pub clamp_max:     f32,
    pub vel_max:       f32,
    pub clamp_kind:    u32,
}

impl GovernedPair {
    fn encode_clamp(c: &ClampBehavior) -> (u32, f32, f32) {
        match c {
            ClampBehavior::Bounded { min, max } => (CLAMP_BOUNDED, *min, *max),
            ClampBehavior::Floored { min }      => (CLAMP_FLOORED, *min, f32::INFINITY),
            ClampBehavior::Unbounded            => (CLAMP_UNBOUNDED, f32::NEG_INFINITY, f32::INFINITY),
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
        if !registry.is_active(id) { continue; }
        let range  = registry.column_range(id);
        let layout = &prop.layout;
        for sf in &layout.sub_fields {
            let Some(gov_role)      = &sf.governed_by                            else { continue };
            let Some(governed_col)  = range.col_for_role(&sf.role, layout)       else { continue };
            let Some(governing_col) = range.col_for_role(gov_role, layout)       else { continue };
            let (clamp_kind, clamp_min, clamp_max) = GovernedPair::encode_clamp(&sf.clamp);
            let vel_max = sf.velocity_max.unwrap_or(f32::INFINITY);
            pairs.push(GovernedPair {
                governed_col:  governed_col  as u32,
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
    pub velocity_col:        u32,
    pub intensity_col:       u32,
    pub velocity_threshold:  f32,
    pub build_coefficient:   f32,
    pub decay_coefficient:   f32,
    /// Pad to 24 bytes so storage-buffer array stride is unambiguous and
    /// matches the WGSL struct layout.
    pub _pad:                u32,
}

/// Walk every active property in the registry and emit one IntensityParams per
/// property whose `intensity_behavior` is `Some` AND whose layout contains both
/// Velocity and Intensity roles. Mirrors the CPU `PropertyValue::update_intensity`
/// short-circuit logic — a property missing either role is silently skipped.
pub fn build_intensity_params(registry: &DimensionRegistry) -> Vec<IntensityParams> {
    let mut params = Vec::new();
    for (idx, prop) in registry.properties.iter().enumerate() {
        let id = SimPropertyId(idx as u32);
        if !registry.is_active(id) { continue; }
        let Some(behavior) = &prop.intensity_behavior            else { continue };
        let range  = registry.column_range(id);
        let layout = &prop.layout;
        let Some(velocity_col)  = range.col_for_role(&SubFieldRole::Velocity,  layout) else { continue };
        let Some(intensity_col) = range.col_for_role(&SubFieldRole::Intensity, layout) else { continue };
        params.push(IntensityParams {
            velocity_col:       velocity_col  as u32,
            intensity_col:      intensity_col as u32,
            velocity_threshold: behavior.velocity_threshold,
            build_coefficient:  behavior.build_coefficient,
            decay_coefficient:  behavior.decay_coefficient,
            _pad:               0,
        });
    }
    params
}

// ── WorldGpuState ─────────────────────────────────────────────────────────────

pub struct WorldGpuState {
    pub ctx:                GpuContext,
    pub n_slots:            u32,
    pub n_dims:             u32,
    pub n_governed_pairs:   u32,
    pub n_intensity_params: u32,

    /// Current property values, row-major: index = slot * n_dims + col.
    pub values:           Buffer,
    /// Snapshot of `values` taken at Pass 0 each tick.
    pub previous_values:  Buffer,
    /// Per-slot post-reduction output (Pass 4–6 destination).
    pub output_vectors:   Buffer,

    /// Per-slot local transform matrices [n_slots × n_dims × n_dims].
    pub local_transforms:    Buffer,
    /// Per-slot composed ancestor transform matrices.
    pub ancestor_transforms: Buffer,

    /// Property-level flat buffer of GovernedPair structs. Same pairs apply
    /// to every slot — Pass 1 dispatches `(n_pairs × n_slots)` threads.
    pub governed_pairs: Buffer,

    /// Property-level flat buffer of IntensityParams structs. Pass 2 dispatches
    /// `(n_intensity_params × n_slots)` threads.
    pub intensity_params: Buffer,
}

impl WorldGpuState {
    pub fn new(ctx: GpuContext, registry: &DimensionRegistry, n_slots: u32) -> Self {
        assert!(n_slots > 0, "n_slots must be > 0");
        assert!(registry.total_columns > 0, "registry has no columns");

        let n_dims = registry.total_columns as u32;
        let pairs  = build_governed_pairs(registry);
        let iparams = build_intensity_params(registry);

        let per_slot_per_col_bytes = (n_slots as u64) * (n_dims as u64) * 4;
        let per_slot_per_mat_bytes = (n_slots as u64) * (n_dims as u64) * (n_dims as u64) * 4;

        let mk = |label: &'static str, size: u64| -> Buffer {
            ctx.device.create_buffer(&BufferDescriptor {
                label: Some(label),
                size,
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        };

        let values              = mk("values",              per_slot_per_col_bytes);
        let previous_values     = mk("previous_values",     per_slot_per_col_bytes);
        let output_vectors      = mk("output_vectors",      per_slot_per_col_bytes);
        let local_transforms    = mk("local_transforms",    per_slot_per_mat_bytes);
        let ancestor_transforms = mk("ancestor_transforms", per_slot_per_mat_bytes);

        // Always allocate at least one pair's worth so the buffer is bindable
        // even when no governed sub-fields exist. The shader iterates n_governed_pairs,
        // not buffer size, so zero pairs = zero work.
        let n_governed_pairs = pairs.len() as u32;
        let governed_bytes   = std::mem::size_of::<GovernedPair>() as u64
                             * pairs.len().max(1) as u64;
        let governed_pairs = ctx.device.create_buffer(&BufferDescriptor {
            label: Some("governed_pairs"),
            size:  governed_bytes,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        if !pairs.is_empty() {
            ctx.queue.write_buffer(&governed_pairs, 0, bytemuck::cast_slice(&pairs));
        }

        // intensity_params: same placeholder-allocation strategy as governed_pairs.
        let n_intensity_params = iparams.len() as u32;
        let iparams_bytes      = std::mem::size_of::<IntensityParams>() as u64
                               * iparams.len().max(1) as u64;
        let intensity_params = ctx.device.create_buffer(&BufferDescriptor {
            label: Some("intensity_params"),
            size:  iparams_bytes,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        if !iparams.is_empty() {
            ctx.queue.write_buffer(&intensity_params, 0, bytemuck::cast_slice(&iparams));
        }

        Self {
            ctx,
            n_slots,
            n_dims,
            n_governed_pairs,
            n_intensity_params,
            values,
            previous_values,
            output_vectors,
            local_transforms,
            ancestor_transforms,
            governed_pairs,
            intensity_params,
        }
    }

    pub fn values_len(&self) -> usize {
        (self.n_slots * self.n_dims) as usize
    }

    pub fn write_values(&self, data: &[f32]) {
        assert_eq!(data.len(), self.values_len(),
            "values write length {} != n_slots * n_dims = {}",
            data.len(), self.values_len());
        self.ctx.queue.write_buffer(&self.values, 0, bytemuck::cast_slice(data));
    }

    pub fn write_previous_values(&self, data: &[f32]) {
        assert_eq!(data.len(), self.values_len());
        self.ctx.queue.write_buffer(&self.previous_values, 0, bytemuck::cast_slice(data));
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

        let mut encoder = self.ctx.device.create_command_encoder(
            &CommandEncoderDescriptor { label: Some("read_buffer_encoder") },
        );
        encoder.copy_buffer_to_buffer(buf, 0, &staging, 0, size);
        self.ctx.queue.submit(Some(encoder.finish()));

        let slice = staging.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(MapMode::Read, move |r| { let _ = tx.send(r); });
        self.ctx.device.poll(Maintain::Wait);
        rx.recv().expect("map_async sender dropped").expect("buffer map failed");

        let mapped = slice.get_mapped_range();
        let out    = mapped.to_vec();
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
        assert_eq!(p.governed_col,  0);
        assert_eq!(p.governing_col, 1);
        assert_eq!(p.clamp_kind,    CLAMP_BOUNDED);
        assert_eq!(p.clamp_min,     0.0);
        assert_eq!(p.clamp_max,     1.0);
        assert_eq!(p.vel_max,       f32::INFINITY);
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
        reg.register(SimProperty::simple("core", "loyalty",       0)); // stride 3
        reg.register(SimProperty::simple("core", "food_security", 0)); // stride 3

        let pairs = build_governed_pairs(&reg);
        assert_eq!(pairs.len(), 2);
        assert_eq!(pairs[0].governed_col,  0);
        assert_eq!(pairs[0].governing_col, 1);
        assert_eq!(pairs[1].governed_col,  3);
        assert_eq!(pairs[1].governing_col, 4);
    }

    #[test]
    fn write_read_values_roundtrip() {
        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };

        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 2)); // stride 5

        let state = WorldGpuState::new(ctx, &reg, 4);
        assert_eq!(state.n_dims,           5);
        assert_eq!(state.n_slots,          4);
        assert_eq!(state.n_governed_pairs, 1);
        assert_eq!(state.values_len(),     20);

        let input: Vec<f32> = (0..20).map(|i| i as f32 * 0.1).collect();
        state.write_values(&input);
        let output = state.read_values();

        for (i, (a, b)) in input.iter().zip(output.iter()).enumerate() {
            assert_eq!(a.to_bits(), b.to_bits(), "mismatch at index {i}: {a} vs {b}");
        }
    }

    #[test]
    fn governed_pairs_upload_roundtrip() {
        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };

        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty",       0));
        reg.register(SimProperty::simple("core", "food_security", 0));

        let expected = build_governed_pairs(&reg);
        let state    = WorldGpuState::new(ctx, &reg, 1);
        let got      = state.read_governed_pairs();
        assert_eq!(got, expected);
    }

    #[test]
    fn intensity_params_only_for_properties_with_behavior() {
        // Two properties: only the second has intensity_behavior set.
        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "plain",        0)); // no behavior → skipped
        reg.register(property_with_intensity("loyalty"));             // has behavior → included

        let params = build_intensity_params(&reg);
        assert_eq!(params.len(), 1);

        let p = params[0];
        // "loyalty" is the second property: stride 3 each, so its range starts at col 3.
        // Within the standard layout: amount=+0, velocity=+1, intensity=+2.
        assert_eq!(p.velocity_col,  4);
        assert_eq!(p.intensity_col, 5);
        let default = IntensityBehavior::default();
        assert_eq!(p.velocity_threshold, default.velocity_threshold);
        assert_eq!(p.build_coefficient,  default.build_coefficient);
        assert_eq!(p.decay_coefficient,  default.decay_coefficient);
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
        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };

        let mut reg = DimensionRegistry::new();
        reg.register(property_with_intensity("a"));
        reg.register(property_with_intensity("b"));

        let expected = build_intensity_params(&reg);
        let state    = WorldGpuState::new(ctx, &reg, 1);
        let got      = state.read_intensity_params();
        assert_eq!(got, expected);
    }

    #[test]
    fn empty_governed_pairs_buffer_is_bindable() {
        // A property with no governed sub-fields still produces a usable
        // WorldGpuState (governed_pairs buffer has a placeholder allocation).
        let Some(ctx) = try_gpu() else { eprintln!("skipping: no GPU"); return };

        let mut reg = DimensionRegistry::new();
        let id = reg.register(SimProperty::simple("core", "loyalty", 0));
        reg.tombstone(id);
        // tombstoned but total_columns > 0, so registry still has dimensions.

        let state = WorldGpuState::new(ctx, &reg, 2);
        assert_eq!(state.n_governed_pairs, 0);
        assert_eq!(state.read_governed_pairs().len(), 0);
    }
}
