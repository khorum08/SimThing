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
        assert!(dispatches >= 2, "atlas path must run multiple dispatches, not per-tile fake");
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
    Some((full_tile, l_inf, corridor, protocol_hash, gpu_hash, fp, cells as u32))
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
    assert!(full_tile <= 0.0001, "acceptance is full-tile, not corridor-only");
    // Corridor may agree even when full-tile fails — here both should pass; document corridor is diagnostic.
    assert!(corridor <= 0.0001);
}

#[test]
fn c0_vram_multiplier_report_uses_active_budget() {
    let shape = c0_fixture_shape();
    let report = build_c0_vram_budget_report(&shape);
    println!("VRAM report: {report:#?}");
    assert_eq!(report.active_budget_bytes, V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES);
    assert!(report.multiplier_reporting_required);
    assert!(report.algebraic_mask_fits_active_budget);
    assert!(report.headroom_bytes > 0);
    assert!((report.algebraic_mask_multiplier - 1.0).abs() < 1e-9);
}

#[test]
fn c0_default_budget_is_1p5_gib_configurable_no_hard_cap() {
    assert!(default_budget_is_1p5_gib_configurable_no_hard_cap());
    let b = active_v78_atlas_vram_budget();
    assert_eq!(b.max_bytes, 1_610_612_736);
    assert!(b.configurable);
    assert!(!b.architectural_hard_cap);
    assert!(b.multiplier_reporting_required);
}

#[test]
fn c0_physical_gutter_fallback_reports_6p76x_or_formula() {
    let shape = c0_fixture_shape();
    let report = build_c0_vram_budget_report(&shape);
    // For 8×8 tiles G=H=8: multiplier = (24²)/(8²) = 9.0. For 10×10 reference: 6.76×.
    let ref_10x10 = simthing_gpu::vram_multiplier(10, 8);
    assert!((ref_10x10 - 6.76).abs() < 0.01, "10×10 G=H=8 reference ≈ 6.76×");
    let tile8 = simthing_gpu::vram_multiplier(8, shape.horizon);
    assert!((report.physical_gutter_multiplier - tile8).abs() < 1e-9);
    assert!(report.physical_gutter_bytes > report.algebraic_mask_bytes);
    println!(
        "gutter multiplier 8×8={} 10×10 ref={} bytes={}",
        report.physical_gutter_multiplier, ref_10x10, report.physical_gutter_bytes
    );
}

#[test]
fn c0_request_atlas_batching_still_rejected_until_gate_acceptance() {
    assert!(atlas_batching_still_rejected_at_admission());
    let spec = c0_region_field_spec_atlas_rejected();
    let err = compile_region_field_preview(&spec).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("atlas") || msg.contains("Atlas") || msg.contains("batching"));
}

#[test]
fn c0_mapping_profile_default_remains_disabled() {
    assert!(mapping_profile_default_disabled());
    assert!(!MappingExecutionProfile::default().enables_execution());
}

#[test]
fn c0_no_active_mask_or_source_identity() {
    let atlas = evaluate_designer_admission_request(DesignerAdmissionRequest::ActiveMask);
    assert!(!atlas.accepted);
    let source = evaluate_designer_admission_request(DesignerAdmissionRequest::SourceIdentity);
    assert!(!source.accepted);
}

#[test]
fn c0_no_semantic_wgsl_or_simthing_sim_awareness() {
    let src = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!src.contains("FrontierV2"));
    assert!(!src.contains("atlas_batching"));
    assert!(!src.contains("ClauseThing"));
    let gpu_src = include_str!("../../simthing-gpu/src/atlas_mask.rs");
    assert!(!gpu_src.contains("faction"));
    assert!(!gpu_src.contains("semantic_wgsl"));
    assert!(!gpu_src.contains("map-specific"));
    assert!(gpu_src.contains("semantic-free") || gpu_src.contains("tile-local mask"));
}

#[test]
fn c0_replay_reproducibility() {
    let Some((_, _, _, h1, g1, fp1, _)) = run_c0_algebraic_atlas_parity() else {
        return;
    };
    let Some((_, _, _, h2, g2, fp2, _)) = run_c0_algebraic_atlas_parity() else {
        return;
    };
    assert_eq!(h1, h2, "protocol oracle hash must be reproducible");
    assert_eq!(g1, g2, "GPU output hash must be reproducible");
    assert_eq!(fp1, fp2, "combined fingerprint must be reproducible");
    println!("C-0 replay fingerprint: {fp1}");
}

#[test]
fn c0_no_implementation_of_a0_b0_l3_frontierv2_5() {
    let gpu_src = include_str!("../../simthing-gpu/src/atlas_mask.rs");
    assert!(!gpu_src.contains("NestedE11B"));
    assert!(!gpu_src.contains("nested_arena"));
    assert!(!gpu_src.contains("ClauseThing"));
    assert!(!gpu_src.contains("FrontierV2Five"));
    let a0 = evaluate_designer_admission_request(DesignerAdmissionRequest::NestedE11B);
    assert!(!a0.accepted);
    let b0 = evaluate_designer_admission_request(DesignerAdmissionRequest::D2aBoundaryScheduling);
    assert!(!b0.accepted);
    let fv25 = evaluate_designer_admission_request(DesignerAdmissionRequest::FrontierV2Five);
    assert!(!fv25.accepted);
}

#[test]
fn c0_rejects_non_homogeneous_square_batch_for_g0_mask() {
    let mut shape = c0_fixture_shape();
    shape.tile_width = 8;
    shape.tile_height = 10;
    let result = std::panic::catch_unwind(|| shape.tile_size());
    assert!(result.is_err(), "non-square tiles must not pass G=0 homogeneous batch");
}

#[test]
fn c0_cpu_protocol_oracle_matches_itself() {
    let shape = c0_fixture_shape();
    let (values, aw, ah) = build_c0_atlas_fixture_values(&shape);
    let config = c0_atlas_config(aw, ah, &shape);
    let a = cpu_caller_managed_atlas_protocol(
        &values,
        &config,
        shape.tile_size(),
        shape.tile_count,
        AtlasIsolationMode::FlushTileLocalMask,
        AtlasNormalizeVariant::FixedDenominator,
    );
    let b = cpu_caller_managed_atlas_protocol(
        &values,
        &config,
        shape.tile_size(),
        shape.tile_count,
        AtlasIsolationMode::FlushTileLocalMask,
        AtlasNormalizeVariant::FixedDenominator,
    );
    assert_eq!(max_full_tile_error(&a, &b, aw, shape.tile_size(), shape.tile_count, shape.n_dims), 0.0);
}
