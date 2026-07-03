//! C-0 — First §11-gate M-4 atlas slice: full-tile protocol-oracle parity + VRAM report.
//!
//! Fixture/test-support only. No production mapping runtime, no default SimSession wiring,
//! no semantic WGSL, no simthing-sim map awareness.

#[path = "support/c0_atlas_protocol_oracle.rs"]
mod c0_support;

use std::sync::Mutex;

use c0_support::*;
use simthing_gpu::{
    combined_fingerprint_hex, corridor_t44_max_error, cpu_caller_managed_atlas_protocol,
    fnv64_hash_f32, full_tile_l_inf, make_atlas_mask_params, max_full_tile_error,
    AtlasIsolationMode, AtlasMaskGpuOp, AtlasNormalizeVariant, GpuContext,
};
use simthing_spec::{
    compile_region_field_preview, evaluate_designer_admission_request, DesignerAdmissionRequest,
    MappingExecutionProfile, V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping GPU assertions: no GPU");
        return;
    };
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn run_c0_algebraic_atlas_parity() -> Option<(f32, f32, f32, u64, u64, String, u32)> {
    let shape = c0_fixture_shape();
    let (values, aw, ah) = build_c0_atlas_fixture_values(&shape);
    let config = c0_atlas_config(aw, ah, &shape);
    let oracle = cpu_caller_managed_atlas_protocol(
        &values,
        &config,
        shape.tile_size(),
        shape.tile_count,
        AtlasIsolationMode::FlushTileLocalMask,
        AtlasNormalizeVariant::FixedDenominator,
    );
    let protocol_hash = fnv64_hash_f32(&oracle, b"c0_protocol_oracle");

    let mut gpu_out = None;
    with_gpu(|ctx| {
        let params = make_atlas_mask_params(
            aw,
            ah,
            shape.tile_size(),
            shape.n_dims,
            false,
            true,
            AtlasNormalizeVariant::FixedDenominator,
        );
        let len = values.len();
        let op = AtlasMaskGpuOp::new(ctx, params, len);
        let (out, dispatches) = op.gpu_caller_managed_atlas_protocol(
            ctx,
            &values,
            shape.tile_count,
            shape.tile_size(),
            shape.horizon,
            shape.n_dims,
        );
        assert!(
            dispatches >= 2,
            "atlas path must run multiple dispatches, not per-tile fake"
        );
        gpu_out = Some((out, dispatches));
    });

    let (gpu, _dispatches) = gpu_out?;
    let gpu_hash = fnv64_hash_f32(&gpu, b"c0_gpu_atlas");
    let full_tile = max_full_tile_error(
        &gpu,
        &oracle,
        aw,
        shape.tile_size(),
        shape.tile_count,
        shape.n_dims,
    );
    let l_inf = full_tile_l_inf(
        &gpu,
        &oracle,
        aw,
        shape.tile_size(),
        shape.tile_count,
        shape.n_dims,
    );
    let corridor = corridor_t44_max_error(
        &gpu,
        &oracle,
        aw,
        shape.tile_size(),
        shape.tile_count,
        shape.n_dims,
    );
    let fp = combined_fingerprint_hex(protocol_hash, gpu_hash);
    let cells = (shape.tile_count as u64) * (shape.tile_size() as u64) * (shape.tile_size() as u64);
    Some((
        full_tile,
        l_inf,
        corridor,
        protocol_hash,
        gpu_hash,
        fp,
        cells as u32,
    ))
}

#[test]
fn c0_happy_path_algebraic_mask_atlas_protocol_oracle_parity() {
    let Some((full_tile, l_inf, _, _, _, fp, cells)) = run_c0_algebraic_atlas_parity() else {
        return;
    };
    println!(
        "C-0 parity: full_tile_max_abs_error={full_tile} l_inf={l_inf} cells={cells} fingerprint={fp}"
    );
    assert!(
        full_tile <= 0.0001,
        "full-tile protocol-oracle parity required; got max_abs_error={full_tile}"
    );
    assert!(l_inf <= 0.0001);
}

#[test]
fn c0_full_tile_parity_not_corridor_only() {
    let Some((full_tile, _, corridor, _, _, _, _)) = run_c0_algebraic_atlas_parity() else {
        return;
    };
    println!("full_tile={full_tile} corridor_t44={corridor} (corridor non-authoritative)");
    assert!(
        full_tile <= 0.0001,
        "acceptance is full-tile, not corridor-only"
    );
    // Corridor may agree even when full-tile fails — here both should pass; document corridor is diagnostic.
    assert!(corridor <= 0.0001);
}

#[test]
fn c0_mapping_profile_default_remains_disabled() {
    assert!(mapping_profile_default_disabled());
    assert!(!MappingExecutionProfile::default().enables_execution());
}

