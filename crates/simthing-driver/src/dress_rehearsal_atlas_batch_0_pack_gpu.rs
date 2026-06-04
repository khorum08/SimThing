//! SCENARIO-0080-2 — `ATLAS-BATCH-0-PACK-GPU` (EC-A2b GpuVerified).
//!
//! Fixture-only: one batched `AtlasMaskGpuOp` path per homogeneous PACK tile class vs
//! caller-managed CPU oracle (`TileLocalMaskG0`). Not exported from `lib.rs`.

pub const DRESS_REHEARSAL_ATLAS_BATCH_0_PACK_GPU_ID: &str = "ATLAS-BATCH-0-PACK-GPU";
pub const DRESS_REHEARSAL_ATLAS_BATCH_0_PACK_GPU_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - EC-A2b GpuVerified full-tile Linf <= 1e-4 (three tile classes); EC-A2b-exact deferred; STORE unimplemented";

pub const GPU_PARITY_TOLERANCE: f32 = 1e-4;
pub const PACK_GPU_HORIZON: u32 = 8;

#[path = "dress_rehearsal_atlas_batch_0_pack.rs"]
mod pack;

pub use pack::{
    AtlasBatchPlan, LocationMaterialization, PackedTile, TileClassDescriptor,
    CLASS_GALACTIC_20X20, CLASS_PLANET_SURFACE_10X10, CLASS_STAR_SYSTEM_10X10,
};

use simthing_gpu::{
    atlas_cell_index, atlas_config, atlas_slot_xy, cpu_atlas_horizon,
    cpu_caller_managed_atlas_protocol, make_atlas_mask_params, AtlasIsolationMode,
    AtlasMaskGpuOp, AtlasMaskParamsGpu, AtlasNormalizeVariant, C0_DEFAULT_N_DIMS, GpuContext,
};

const SOURCE_COL: u32 = 0;
const TARGET_COL: u32 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct PackGpuClassParityReport {
    pub class_id: String,
    pub tile_count: u32,
    pub atlas_width: u32,
    pub atlas_height: u32,
    pub output_element_count: usize,
    pub full_tile_l_inf: f32,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PackGpuParitySummary {
    pub adapter_name: String,
    pub device_name: String,
    pub classes: Vec<PackGpuClassParityReport>,
    pub ec_a2b_closed: bool,
}

pub fn gpu_tests_requested() -> bool {
    std::env::var("SIMTHING_RUN_GPU_TESTS")
        .ok()
        .as_deref()
        == Some("1")
}

pub fn canonical_pack_plan() -> AtlasBatchPlan {
    AtlasBatchPlan::canonical()
}

pub fn atlas_mask_params_for_class(class: &TileClassDescriptor) -> AtlasMaskParamsGpu {
    make_atlas_mask_params(
        class.atlas_width,
        class.atlas_height,
        class.tile_width,
        C0_DEFAULT_N_DIMS,
        false,
        true,
        AtlasNormalizeVariant::FixedDenominator,
    )
}

/// Params are generic width/height/tile_size/mask only — no map/faction/gameplay fields.
pub fn atlas_mask_params_are_semantic_free(params: &AtlasMaskParamsGpu) -> bool {
    params.width > 0
        && params.height > 0
        && params.n_dims == C0_DEFAULT_N_DIMS
        && params.use_tile_local_mask == 1
        && params.source_cap == 0.0
        && params.variant == 1
}

fn seed_cluster_at(
    values: &mut [f32],
    width: u32,
    ox: u32,
    oy: u32,
    scale: f32,
    n_dims: u32,
) {
    for &(dx, dy, v) in &[(0u32, 0, 80.0f32), (1, 0, 60.0), (0, 1, 60.0), (1, 1, 40.0)] {
        if ox + dx < width {
            values[atlas_cell_index(
                atlas_slot_xy(ox + dx, oy + dy, width),
                SOURCE_COL,
                n_dims,
            )] = v * scale;
        }
    }
}

fn clear_seed_cells_only_local(values: &mut [f32], width: u32, origin_slot: u32, n_dims: u32) {
    let ox = origin_slot % width;
    let oy = origin_slot / width;
    for dy in 0..2 {
        for dx in 0..2 {
            if ox + dx < width {
                values[atlas_cell_index(
                    atlas_slot_xy(ox + dx, oy + dy, width),
                    SOURCE_COL,
                    n_dims,
                )] = 0.0;
            }
        }
    }
}

pub fn pack_tile_origins(tiles: &[&PackedTile]) -> Vec<(u32, u32)> {
    tiles.iter().map(|tile| tile.atlas_origin).collect()
}

pub fn build_class_scalar_field(class: &TileClassDescriptor, tiles: &[&PackedTile]) -> Vec<f32> {
    let n_dims = C0_DEFAULT_N_DIMS;
    let len = (class.atlas_width * class.atlas_height * n_dims) as usize;
    let mut values = vec![0.0f32; len];
    for (index, tile) in tiles.iter().enumerate() {
        let scale = 1.0 + index as f32 * 0.03;
        let (ox, oy) = tile.atlas_origin;
        seed_cluster_at(
            values.as_mut_slice(),
            class.atlas_width,
            ox,
            oy,
            scale,
            n_dims,
        );
    }
    values
}

fn atlas_config_for_class(class: &TileClassDescriptor) -> simthing_gpu::StructuredFieldStencilConfig {
    atlas_config(
        class.atlas_width,
        class.atlas_height,
        PACK_GPU_HORIZON,
        C0_DEFAULT_N_DIMS,
        false,
    )
}

/// Caller-managed CPU oracle with PACK row-major tile origins (same algorithm as `cpu_caller_managed_atlas_protocol`).
fn cpu_caller_managed_atlas_protocol_pack_origins(
    values: &[f32],
    config: &simthing_gpu::StructuredFieldStencilConfig,
    tile_size: u32,
    origins: &[(u32, u32)],
    mode: AtlasIsolationMode,
    norm: AtlasNormalizeVariant,
) -> Vec<f32> {
    let width = config.width;
    let n_dims = config.n_dims;
    let mut cur = values.to_vec();
    cur = cpu_atlas_horizon(&cur, config, tile_size, mode, norm, 1);
    for &(ox, oy) in origins {
        clear_seed_cells_only_local(
            &mut cur,
            width,
            atlas_slot_xy(ox, oy, width),
            n_dims,
        );
    }
    if config.horizon > 1 {
        cur = cpu_atlas_horizon(&cur, config, tile_size, mode, norm, config.horizon);
    }
    cur
}

fn max_full_tile_error_pack_origins(
    got: &[f32],
    expected: &[f32],
    width: u32,
    tile_size: u32,
    origins: &[(u32, u32)],
    n_dims: u32,
) -> f32 {
    let mut max_err = 0.0f32;
    for &(ox, oy) in origins {
        for ly in 0..tile_size {
            for lx in 0..tile_size {
                let a = got[atlas_cell_index(
                    atlas_slot_xy(ox + lx, oy + ly, width),
                    TARGET_COL,
                    n_dims,
                )];
                let b = expected[atlas_cell_index(
                    atlas_slot_xy(ox + lx, oy + ly, width),
                    TARGET_COL,
                    n_dims,
                )];
                max_err = max_err.max((a - b).abs());
            }
        }
    }
    max_err
}

fn gpu_caller_managed_atlas_protocol_pack_origins(
    op: &AtlasMaskGpuOp,
    ctx: &GpuContext,
    values: &[f32],
    width: u32,
    _tile_size: u32,
    origins: &[(u32, u32)],
    horizon: u32,
    n_dims: u32,
) -> (Vec<f32>, u32) {
    use wgpu::{BufferDescriptor, BufferUsages};

    let len = values.len();
    let device = &ctx.device;
    let input = device.create_buffer(&BufferDescriptor {
        label: Some("pack_gpu_atlas_in"),
        size: (len * 4) as u64,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let output = device.create_buffer(&BufferDescriptor {
        label: Some("pack_gpu_atlas_out"),
        size: (len * 4) as u64,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    ctx.queue
        .write_buffer(&input, 0, bytemuck::cast_slice(values));
    op.dispatch_once(ctx, &input, &output);
    let mut cur = op.readback(ctx, &output, len);
    let mut dispatches = 1u32;

    for &(ox, oy) in origins {
        clear_seed_cells_only_local(
            &mut cur,
            width,
            atlas_slot_xy(ox, oy, width),
            n_dims,
        );
    }
    ctx.queue
        .write_buffer(&input, 0, bytemuck::cast_slice(&cur));

    let mut use_input = true;
    for _ in 0..horizon {
        if use_input {
            op.dispatch_once(ctx, &input, &output);
        } else {
            op.dispatch_once(ctx, &output, &input);
        }
        dispatches += 1;
        use_input = !use_input;
    }
    let out_buf = if horizon % 2 == 1 {
        &output
    } else {
        &input
    };
    (op.readback(ctx, out_buf, len), dispatches)
}

fn run_class_parity(
    plan: &AtlasBatchPlan,
    class_id: &str,
    ctx: &GpuContext,
) -> PackGpuClassParityReport {
    let class = plan
        .class(class_id)
        .expect("canonical plan includes class");
    let tiles: Vec<_> = plan.tiles_in_class(class_id);
    let tile_count = tiles.len() as u32;
    let tile_size = class.tile_width;
    let origins = pack_tile_origins(&tiles);
    let values = build_class_scalar_field(class, &tiles);
    let config = atlas_config_for_class(class);
    let mode = AtlasIsolationMode::FlushTileLocalMask;
    let norm = AtlasNormalizeVariant::FixedDenominator;

    let oracle = if tile_count == 1 && origins == [(0, 0)] {
        cpu_caller_managed_atlas_protocol(
            &values,
            &config,
            tile_size,
            tile_count,
            mode,
            norm,
        )
    } else {
        cpu_caller_managed_atlas_protocol_pack_origins(
            &values,
            &config,
            tile_size,
            &origins,
            mode,
            norm,
        )
    };

    let params = atlas_mask_params_for_class(class);
    let op = AtlasMaskGpuOp::new(ctx, params, values.len());
    let (gpu, dispatches) = if tile_count == 1 && origins == [(0, 0)] {
        op.gpu_caller_managed_atlas_protocol(
            ctx,
            &values,
            tile_count,
            tile_size,
            PACK_GPU_HORIZON,
            C0_DEFAULT_N_DIMS,
        )
    } else {
        gpu_caller_managed_atlas_protocol_pack_origins(
            &op,
            ctx,
            &values,
            class.atlas_width,
            tile_size,
            &origins,
            PACK_GPU_HORIZON,
            C0_DEFAULT_N_DIMS,
        )
    };
    assert!(
        dispatches >= 2,
        "batched atlas path must run multiple dispatches, not per-tile fake; got {dispatches}"
    );

    let l_inf = if tile_count == 1 && origins == [(0, 0)] {
        simthing_gpu::full_tile_l_inf(
            &gpu,
            &oracle,
            class.atlas_width,
            tile_size,
            tile_count,
            C0_DEFAULT_N_DIMS,
        )
    } else {
        max_full_tile_error_pack_origins(
            &gpu,
            &oracle,
            class.atlas_width,
            tile_size,
            &origins,
            C0_DEFAULT_N_DIMS,
        )
    };

    PackGpuClassParityReport {
        class_id: class_id.to_string(),
        tile_count,
        atlas_width: class.atlas_width,
        atlas_height: class.atlas_height,
        output_element_count: gpu.len(),
        full_tile_l_inf: l_inf,
        passed: l_inf <= GPU_PARITY_TOLERANCE,
    }
}

pub fn run_ec_a2b_parity_all_classes(ctx: &GpuContext) -> PackGpuParitySummary {
    let plan = canonical_pack_plan();
    let adapter_name = ctx.adapter.get_info().name;
    let device_name = "simthing-gpu device".to_string();
    let classes = [
        CLASS_GALACTIC_20X20,
        CLASS_STAR_SYSTEM_10X10,
        CLASS_PLANET_SURFACE_10X10,
    ];
    let mut reports = Vec::new();
    for class_id in classes {
        reports.push(run_class_parity(&plan, class_id, ctx));
    }
    let ec_a2b_closed = reports.iter().all(|report| report.passed);
    PackGpuParitySummary {
        adapter_name,
        device_name,
        classes: reports,
        ec_a2b_closed,
    }
}

/// Cross-tile / out-of-atlas samples are zero under tile-local G=0 (PACK plan layout).
pub fn verify_g_zero_blocks_cross_tile_and_out_of_atlas() {
    let plan = canonical_pack_plan();
    let class_id = CLASS_STAR_SYSTEM_10X10;
    let class = plan.class(class_id).expect("class");
    let tiles = plan.tiles_in_class(class_id);
    let tile_a = tiles[0];
    let tile_b = tiles[1];
    let (ax, ay) = (tile_a.atlas_origin.0 + 5, tile_a.atlas_origin.1 + 5);
    let (bx, by) = (tile_b.atlas_origin.0 + 5, tile_b.atlas_origin.1 + 5);
    // PACK `g_zero_sample` uses one scalar per atlas cell (same as EC-A2a pack tests).
    let atlas_len = (class.atlas_width * class.atlas_height) as usize;
    let mut field = vec![0.0f32; atlas_len];
    for (index, slot) in field.iter_mut().enumerate() {
        *slot = (index as f32 + 1.0) * 0.001;
    }

    let in_tile = pack::g_zero_sample(&plan, class_id, ax, ay, (ax, ay), &field);
    assert!(in_tile > 0.0, "in-tile neighbor must pass through");

    let across = pack::g_zero_sample(&plan, class_id, ax, ay, (bx, by), &field);
    assert_eq!(across, 0.0, "cross-tile neighbor must be zero");

    let out_of_atlas = pack::g_zero_sample(
        &plan,
        class_id,
        class.atlas_width - 1,
        class.atlas_height - 1,
        (class.atlas_width, class.atlas_height),
        &field,
    );
    assert_eq!(out_of_atlas, 0.0, "out-of-atlas neighbor must be zero");
}

pub fn format_parity_report(summary: &PackGpuParitySummary, gpu_tier_ran: bool) -> String {
    let mut lines = Vec::new();
    lines.push(format!("adapter_name: {}", summary.adapter_name));
    lines.push(format!("device_name: {}", summary.device_name));
    lines.push(format!("gpu_tier_ran: {gpu_tier_ran}"));
    lines.push("tile_classes_tested: Galactic20x20, StarSystem10x10, PlanetSurface10x10".to_string());
    for report in &summary.classes {
        lines.push(format!(
            "class={} tile_count={} atlas={}x{} output_elements={} full_tile_Linf={:.6} passed_Linf_le_1e-4={}",
            report.class_id,
            report.tile_count,
            report.atlas_width,
            report.atlas_height,
            report.output_element_count,
            report.full_tile_l_inf,
            report.passed
        ));
    }
    lines.push(format!(
        "EC-A2b_GpuVerified_closed: {}",
        summary.ec_a2b_closed && gpu_tier_ran
    ));
    lines.push("EC-A2b-exact: deferred (not bit-exact / not to_bits)".to_string());
    lines.join("\n")
}
