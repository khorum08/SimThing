#[path = "../src/dress_rehearsal_atlas_batch_0_pack_gpu.rs"]
mod dress_rehearsal_atlas_batch_0_pack_gpu;

use std::sync::{Mutex, OnceLock};

use dress_rehearsal_atlas_batch_0_pack_gpu::{
    atlas_mask_params_are_semantic_free, atlas_mask_params_for_class, canonical_pack_plan,
    format_parity_report, gpu_tests_requested, run_ec_a2b_parity_all_classes,
    verify_g_zero_blocks_cross_tile_and_out_of_atlas, AtlasBatchPlan, GPU_PARITY_TOLERANCE,
    PackGpuParitySummary, CLASS_GALACTIC_20X20, CLASS_PLANET_SURFACE_10X10,
    CLASS_STAR_SYSTEM_10X10, DRESS_REHEARSAL_ATLAS_BATCH_0_PACK_GPU_ID,
    DRESS_REHEARSAL_ATLAS_BATCH_0_PACK_GPU_STATUS_PASS,
};
use simthing_gpu::GpuContext;

static GPU_MUTEX: Mutex<()> = Mutex::new(());
static GPU_PARITY_SUMMARY: OnceLock<PackGpuParitySummary> = OnceLock::new();

/// One GPU init + one full EC-A2b pass (all three classes) per test binary.
fn cached_gpu_parity_summary() -> Option<&'static PackGpuParitySummary> {
    if !gpu_tests_requested() {
        eprintln!("skipping GPU tests: SIMTHING_RUN_GPU_TESTS not set to 1");
        return None;
    }
    Some(GPU_PARITY_SUMMARY.get_or_init(|| {
        let ctx = GpuContext::new_blocking().expect(
            "SIMTHING_RUN_GPU_TESTS=1 requires a GPU adapter; skipped GPU is not PASS evidence",
        );
        let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        run_ec_a2b_parity_all_classes(&ctx)
    }))
}

#[test]
fn pack_gpu_status_matches_gate() {
    assert_eq!(
        DRESS_REHEARSAL_ATLAS_BATCH_0_PACK_GPU_ID,
        "ATLAS-BATCH-0-PACK-GPU"
    );
    let status = DRESS_REHEARSAL_ATLAS_BATCH_0_PACK_GPU_STATUS_PASS;
    assert!(status.contains("IMPLEMENTED / PASS"));
    assert!(status.contains("EC-A2b"));
    assert!(status.contains("GpuVerified") || status.contains("Linf"));
    assert!(status.contains("EC-A2b-exact") && status.contains("deferred"));
    assert!(!status.contains("to_bits"));
    assert!(!status.contains("ExactDeterministic"));
    assert!(!status.contains("bit-exact"));
    assert!(
        !status.contains("STORE") || status.contains("unimplemented"),
        "must not claim STORE implemented"
    );
    assert!(!status.contains("R1") && !status.contains("R4"));
}

#[test]
fn gpu_fixture_uses_accepted_pack_plan() {
    let plan = canonical_pack_plan();
    let canonical = AtlasBatchPlan::canonical();
    assert_eq!(plan, canonical);
    assert_eq!(plan.classes.len(), 3);
    assert_eq!(plan.tiles.len(), 27);
    assert_eq!(plan.class(CLASS_GALACTIC_20X20).unwrap().source_location_ids.len(), 1);
    assert_eq!(
        plan.class(CLASS_STAR_SYSTEM_10X10)
            .unwrap()
            .source_location_ids
            .len(),
        13
    );
}

#[test]
fn channel_metadata_survives() {
    let plan = canonical_pack_plan();
    use dress_rehearsal_atlas_batch_0_pack_gpu::LocationMaterialization;
    let materialization = LocationMaterialization::canonical();
    for class in &plan.classes {
        assert!(!class.channels.channels.is_empty());
        for location in &materialization.locations {
            if location.role != class.role {
                continue;
            }
            assert_eq!(
                location.channels.channels.len(),
                class.channels.channels.len()
            );
        }
    }
    let galactic = plan.class(CLASS_GALACTIC_20X20).unwrap();
    assert_eq!(galactic.channels.channels.len(), 5);
    let system = plan.class(CLASS_STAR_SYSTEM_10X10).unwrap();
    assert_eq!(system.channels.channels.len(), 2);
}

#[test]
fn no_semantic_shader_inputs() {
    let plan = canonical_pack_plan();
    for class in &plan.classes {
        let params = atlas_mask_params_for_class(class);
        assert!(atlas_mask_params_are_semantic_free(&params));
    }
}

#[test]
fn gpu_oracle_parity_galactic_20x20() {
    let Some(summary) = cached_gpu_parity_summary() else {
        return;
    };
    let report = summary
        .classes
        .iter()
        .find(|c| c.class_id == CLASS_GALACTIC_20X20)
        .expect("galactic report");
    assert!(
        report.passed,
        "galactic full-tile Linf {} > {}",
        report.full_tile_l_inf,
        GPU_PARITY_TOLERANCE
    );
    println!(
        "PACK-GPU galactic: Linf={} tiles={} adapter={}",
        report.full_tile_l_inf, report.tile_count, summary.adapter_name
    );
}

#[test]
fn gpu_oracle_parity_star_system_10x10_batch() {
    let Some(summary) = cached_gpu_parity_summary() else {
        return;
    };
    let report = summary
        .classes
        .iter()
        .find(|c| c.class_id == CLASS_STAR_SYSTEM_10X10)
        .expect("star-system report");
    assert_eq!(report.tile_count, 13);
    assert_eq!(report.atlas_width, 130);
    assert_eq!(report.atlas_height, 10);
    assert!(
        report.passed,
        "star-system full-tile Linf {} > {}",
        report.full_tile_l_inf,
        GPU_PARITY_TOLERANCE
    );
    println!(
        "PACK-GPU star-system batch: Linf={} atlas={}x{}",
        report.full_tile_l_inf, report.atlas_width, report.atlas_height
    );
}

#[test]
fn gpu_oracle_parity_planet_surface_10x10_batch() {
    let Some(summary) = cached_gpu_parity_summary() else {
        return;
    };
    let report = summary
        .classes
        .iter()
        .find(|c| c.class_id == CLASS_PLANET_SURFACE_10X10)
        .expect("planet-surface report");
    assert_eq!(report.tile_count, 13);
    assert!(
        report.passed,
        "planet-surface full-tile Linf {} > {}",
        report.full_tile_l_inf,
        GPU_PARITY_TOLERANCE
    );
    println!(
        "PACK-GPU planet-surface batch: Linf={}",
        report.full_tile_l_inf
    );
}

#[test]
fn g_zero_blocks_cross_tile_and_out_of_atlas() {
    let Some(summary) = cached_gpu_parity_summary() else {
        return;
    };
    verify_g_zero_blocks_cross_tile_and_out_of_atlas();
    assert!(summary.ec_a2b_closed);
    let text = format_parity_report(summary, true);
    println!("{text}");
}
