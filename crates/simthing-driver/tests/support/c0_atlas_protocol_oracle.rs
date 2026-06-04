//! C-0 atlas protocol-oracle fixture support (test-only).

use simthing_gpu::{
    atlas_config, atlas_dims, atlas_side, build_flush_atlas, vram_multiplier, AtlasIsolationPolicy,
    C0AtlasFixtureShape,
};
use simthing_spec::{
    compile_region_field_preview, MappingExecutionProfile, RegionFieldSpec, V78AtlasVramBudget,
    V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES,
};

pub const C0_REPLAY_FINGERPRINT: &str = "a974fe44e20620f3";

#[derive(Clone, Debug, PartialEq)]
pub struct C0VramBudgetReport {
    pub active_budget_bytes: u64,
    pub active_budget_gib: f64,
    pub budget_configurable: bool,
    pub architectural_hard_cap: bool,
    pub multiplier_reporting_required: bool,
    pub tile_count: u32,
    pub tile_width: u32,
    pub tile_height: u32,
    pub horizon: u32,
    pub atlas_width: u32,
    pub atlas_height: u32,
    pub payload_cell_count: u64,
    pub isolation_cell_count: u64,
    pub total_atlas_cell_count: u64,
    pub n_dims: u32,
    pub bytes_per_cell: u64,
    pub buffer_multiplier: f32,
    pub algebraic_mask_multiplier: f64,
    pub algebraic_mask_bytes: u64,
    pub algebraic_mask_fits_active_budget: bool,
    pub physical_gutter_multiplier: f64,
    pub physical_gutter_bytes: u64,
    pub physical_gutter_fits_active_budget: bool,
    pub headroom_bytes: i64,
    pub headroom_percent: f64,
    pub isolation_policy: AtlasIsolationPolicy,
    pub fallback_isolation: AtlasIsolationPolicy,
}

pub fn c0_fixture_shape() -> C0AtlasFixtureShape {
    C0AtlasFixtureShape::c0_default()
}

pub fn active_v78_atlas_vram_budget() -> V78AtlasVramBudget {
    V78AtlasVramBudget::default_1p5_gib()
}

pub fn build_c0_vram_budget_report(shape: &C0AtlasFixtureShape) -> C0VramBudgetReport {
    let budget = active_v78_atlas_vram_budget();
    let tile_size = shape.tile_size();
    let (aw, ah) = atlas_dims(shape.tile_count, tile_size);
    let payload_cells = shape.tile_count as u64 * tile_size as u64 * tile_size as u64;
    let total_cells = aw as u64 * ah as u64;
    let n_dims = shape.n_dims;
    let bytes_per_cell = n_dims as u64 * 4;
    let buffer_multiplier = 2.0f32; // ping-pong field storage

    let algebraic_multiplier = 1.0;
    let algebraic_bytes = ((total_cells * bytes_per_cell) as f64
        * algebraic_multiplier
        * f64::from(buffer_multiplier))
    .ceil() as u64;

    let gutter = shape.horizon;
    let physical_multiplier = vram_multiplier(tile_size, gutter);
    let side = atlas_side(shape.tile_count);
    let pitch = tile_size + 2 * gutter;
    let gutter_aw = side * pitch;
    let gutter_ah = side * pitch;
    let gutter_total_cells = gutter_aw as u64 * gutter_ah as u64;
    let physical_bytes =
        ((gutter_total_cells * bytes_per_cell) as f64 * f64::from(buffer_multiplier)).ceil() as u64;

    let active = budget.max_bytes;
    let headroom = active as i64 - algebraic_bytes as i64;
    let headroom_pct = if active > 0 {
        (headroom as f64 / active as f64) * 100.0
    } else {
        0.0
    };

    C0VramBudgetReport {
        active_budget_bytes: active,
        active_budget_gib: active as f64 / (1024.0 * 1024.0 * 1024.0),
        budget_configurable: budget.configurable,
        architectural_hard_cap: budget.architectural_hard_cap,
        multiplier_reporting_required: budget.multiplier_reporting_required,
        tile_count: shape.tile_count,
        tile_width: shape.tile_width,
        tile_height: shape.tile_height,
        horizon: shape.horizon,
        atlas_width: aw,
        atlas_height: ah,
        payload_cell_count: payload_cells,
        isolation_cell_count: total_cells,
        total_atlas_cell_count: total_cells,
        n_dims,
        bytes_per_cell,
        buffer_multiplier,
        algebraic_mask_multiplier: algebraic_multiplier,
        algebraic_mask_bytes: algebraic_bytes,
        algebraic_mask_fits_active_budget: algebraic_bytes <= active,
        physical_gutter_multiplier: physical_multiplier,
        physical_gutter_bytes: physical_bytes,
        physical_gutter_fits_active_budget: physical_bytes <= active,
        headroom_bytes: headroom,
        headroom_percent: headroom_pct,
        isolation_policy: AtlasIsolationPolicy::AlgebraicTileLocalMaskG0,
        fallback_isolation: AtlasIsolationPolicy::PhysicalGutterGteH,
    }
}

pub fn c0_region_field_spec_no_atlas() -> RegionFieldSpec {
    RegionFieldSpec {
        name: "c0_guardrail_no_atlas".into(),
        grid_size: 8,
        n_dims: 4,
        source_col: 0,
        target_col: 0,
        operator: simthing_spec::RegionFieldOperatorSpec::Normalized,
        horizon: 8,
        allow_extended_horizon: false,
        alpha_self: 1.0,
        gamma_neighbor: 0.8,
        source_cap: None,
        source_policy: Default::default(),
        cadence: simthing_spec::RegionFieldCadenceSpec::EveryTick,
        grid_profile: Default::default(),
        reduction: None,
        parent_formula: None,
        commitment: None,
        request_atlas_batching: false,
        max_region_field_vram_bytes: None,
        summary_policy: Default::default(),
    }
}

pub fn c0_region_field_spec_atlas_rejected() -> RegionFieldSpec {
    RegionFieldSpec {
        request_atlas_batching: true,
        ..c0_region_field_spec_no_atlas()
    }
}

pub fn atlas_batching_still_rejected_at_admission() -> bool {
    let spec = c0_region_field_spec_atlas_rejected();
    compile_region_field_preview(&spec).is_err()
}

pub fn mapping_profile_default_disabled() -> bool {
    MappingExecutionProfile::default() == MappingExecutionProfile::Disabled
}

pub fn default_budget_is_1p5_gib_configurable_no_hard_cap() -> bool {
    let b = active_v78_atlas_vram_budget();
    b.max_bytes == V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES
        && b.configurable
        && !b.architectural_hard_cap
        && b.multiplier_reporting_required
}

pub fn build_c0_atlas_fixture_values(shape: &C0AtlasFixtureShape) -> (Vec<f32>, u32, u32) {
    build_flush_atlas(shape.tile_count, shape.tile_size(), shape.n_dims)
}

pub fn c0_atlas_config(
    width: u32,
    height: u32,
    shape: &C0AtlasFixtureShape,
) -> simthing_gpu::StructuredFieldStencilConfig {
    atlas_config(width, height, shape.horizon, shape.n_dims, false)
}

pub fn fnv64(seed: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in seed {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

pub fn fnv_append_u32(hash: u64, v: u32) -> u64 {
    let mut h = hash;
    h ^= u64::from(v);
    h.wrapping_mul(0x100000001b3)
}

pub fn hash_vram_report(report: &C0VramBudgetReport) -> u64 {
    let mut h = fnv64(b"c0_vram");
    h = fnv_append_u32(h, report.active_budget_bytes as u32);
    h = fnv_append_u32(h, (report.active_budget_bytes >> 32) as u32);
    h = fnv_append_u32(h, report.tile_count);
    h = fnv_append_u32(h, report.algebraic_mask_bytes as u32);
    h
}
