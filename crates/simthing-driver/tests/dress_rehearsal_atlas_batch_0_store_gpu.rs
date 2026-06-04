#[path = "../src/dress_rehearsal_atlas_batch_0_store_gpu.rs"]
mod dress_rehearsal_atlas_batch_0_store_gpu;

use std::sync::{Mutex, OnceLock};

use dress_rehearsal_atlas_batch_0_store_gpu::{
    adapter_name_is_discrete_rtx_target, build_store_gpu_fixture_layout, build_store_gpu_ops,
    canonical_materialization, canonical_pirate_shared_galactic_cell, canonical_store_oracle,
    encode_channel_f32, encode_owner_f32, entries_at_cell_index, fixture_inputs_are_semantic_free,
    format_parity_report, gpu_tests_requested, mask_scale_spec,
    register_constructed_co_location_occupants, requested_adapter_substring,
    store_oracle_constructed_planet_patrol_pirate, store_oracle_from_materialization,
    validate_adapter_selection, ChannelKind, LocationId, Owner, StoreGpuAdapterSelection,
    StoreGpuParityReport, DRESS_REHEARSAL_ATLAS_BATCH_0_STORE_GPU_ID,
    DRESS_REHEARSAL_ATLAS_BATCH_0_STORE_GPU_STATUS_PASS,
};
use dress_rehearsal_atlas_batch_0_store_gpu::{fill_values_buffer, run_ec_a3_gpu_suite};
use simthing_core::ScaleSpec;
use simthing_gpu::GpuContext;
use wgpu::{Backends, Instance, InstanceDescriptor};

static GPU_MUTEX: Mutex<()> = Mutex::new(());
static GPU_PARITY: OnceLock<StoreGpuParityReport> = OnceLock::new();
static GPU_ADAPTER: OnceLock<StoreGpuAdapterSelection> = OnceLock::new();

fn wgpu_adapter_inventory() -> Vec<String> {
    let instance = Instance::new(InstanceDescriptor {
        backends: Backends::PRIMARY,
        ..Default::default()
    });
    instance
        .enumerate_adapters(Backends::PRIMARY)
        .into_iter()
        .map(|adapter| adapter.get_info().name)
        .collect()
}

fn init_discrete_gpu_context() -> (GpuContext, StoreGpuAdapterSelection) {
    if let Some(substring) = requested_adapter_substring() {
        std::env::set_var("WGPU_ADAPTER_NAME", &substring);
    }
    let inventory = wgpu_adapter_inventory();
    let ctx = GpuContext::new_blocking().expect(
        "SIMTHING_RUN_GPU_TESTS=1 requires a GPU adapter; skipped GPU is not PASS evidence",
    );
    let selection = validate_adapter_selection(&ctx, &inventory)
        .expect("STORE-GPU requires discrete RTX/NVIDIA adapter evidence");
    (ctx, selection)
}

fn cached_gpu_adapter_selection() -> Option<&'static StoreGpuAdapterSelection> {
    if !gpu_tests_requested() {
        eprintln!("skipping GPU tests: SIMTHING_RUN_GPU_TESTS not set to 1");
        return None;
    }
    Some(GPU_ADAPTER.get_or_init(|| {
        let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        init_discrete_gpu_context().1
    }))
}

fn cached_gpu_report() -> Option<&'static StoreGpuParityReport> {
    if !gpu_tests_requested() {
        eprintln!("skipping GPU tests: SIMTHING_RUN_GPU_TESTS not set to 1");
        return None;
    }
    Some(GPU_PARITY.get_or_init(|| {
        let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let (ctx, selection) = init_discrete_gpu_context();
        run_ec_a3_gpu_suite(&ctx, &selection)
    }))
}

#[test]
fn store_gpu_status_matches_gate() {
    assert_eq!(
        DRESS_REHEARSAL_ATLAS_BATCH_0_STORE_GPU_ID,
        "ATLAS-BATCH-0-STORE-GPU"
    );
    let status = DRESS_REHEARSAL_ATLAS_BATCH_0_STORE_GPU_STATUS_PASS;
    assert!(status.contains("IMPLEMENTED / PASS"));
    assert!(status.contains("EC-A3-gpu"));
    assert!(status.contains("OWNER/channel"));
    assert!(status.contains("STORE oracle"));
    assert!(status.contains("fixture composition only"));
    assert!(status.contains("R3/runtime parked"));
    assert!(status.contains("RTX/NVIDIA adapter"));
    assert!(status.contains("ExactDeterministic bit-exact"));
    assert!(!status.contains("GpuVerified fallback"));
    assert!(!status.contains("R1") && !status.contains("R2") && !status.contains("R4"));
    assert!(!status.to_lowercase().contains("economy"));
    assert!(!status.contains("SEAD"));
    assert!(!status.contains("REENROLL"));
    assert!(!status.contains("movement"));
    assert!(!status.contains("combat"));
}

#[test]
fn store_gpu_consumes_accepted_store_oracle() {
    let materialization = canonical_materialization();
    let oracle_a = store_oracle_from_materialization(&materialization);
    let oracle_b = store_oracle_from_materialization(&materialization);
    assert_eq!(oracle_a, oracle_b);
    let layout = build_store_gpu_fixture_layout(&oracle_a);
    assert_eq!(layout.n_target_slots as usize, oracle_a.entries.len());
    let store_source = include_str!("../src/dress_rehearsal_atlas_batch_0_store.rs");
    assert!(!store_source.contains("dress_rehearsal_atlas_batch_0_store_gpu"));
    assert!(!store_source.contains("AccumulatorOpSession"));
}

#[test]
fn no_semantic_shader_or_gameplay_inputs() {
    let oracle = canonical_store_oracle();
    let layout = build_store_gpu_fixture_layout(&oracle);
    let values = fill_values_buffer(&oracle, &layout);
    assert!(fixture_inputs_are_semantic_free(&values, &layout));
    assert!(matches!(mask_scale_spec(), ScaleSpec::ByColumn { col: 1 }));
    assert_eq!(encode_owner_f32(Owner::Terran), 0.0);
    assert_eq!(encode_owner_f32(Owner::Pirate), 1.0);
    assert_eq!(encode_channel_f32(ChannelKind::PiratePresence), 5.0);
    let source = include_str!("../src/dress_rehearsal_atlas_batch_0_store_gpu.rs");
    for term in ["faction", "map_name", "gameplay", "BoundedFeedback", "diffusion"] {
        assert!(
            !source.contains(term),
            "fixture must not embed gameplay semantics: {term}"
        );
    }
}

#[test]
fn no_r1_r2_r3_r4_behavior() {
    let status = DRESS_REHEARSAL_ATLAS_BATCH_0_STORE_GPU_STATUS_PASS;
    for term in [
        "BoundedFeedback",
        "diffusion",
        "economy",
        "stockpile",
        "SEAD",
        "BoundaryRequest",
        "REENROLL",
        "combat",
        "capability-tree",
        "session pass",
    ] {
        assert!(!status.to_lowercase().contains(&term.to_lowercase()));
    }
    let source = include_str!("../src/dress_rehearsal_atlas_batch_0_store_gpu.rs");
    for term in ["StructuredFieldStencilOp", "execute_configured", "RegionField", "simthing_sim"] {
        assert!(!source.contains(term), "must not wire {term}");
    }
}

#[test]
fn gpu_adapter_is_discrete_rtx_target() {
    let Some(selection) = cached_gpu_adapter_selection() else {
        return;
    };
    assert!(selection.selected_adapter_is_discrete_rtx);
    assert!(adapter_name_is_discrete_rtx_target(&selection.selected_adapter_name));
    assert!(!selection.selected_adapter_name.to_ascii_lowercase().contains("intel"));
    if selection.require_adapter_match {
        assert!(selection.adapter_target_matched);
    }
}

#[test]
fn gpu_parity_full_store_table() {
    let Some(report) = cached_gpu_report() else {
        return;
    };
    assert!(report.ec_a3_gpu_closed, "full table parity: {:?}", report);
    assert_eq!(report.bit_exact_mismatches, 0);
    assert_eq!(
        report.cpu_oracle_entry_count,
        report.gpu_output_entry_count
    );
    println!("{}", format_parity_report(report, true));
}

#[test]
fn gpu_preserves_10_pirate_shared_cell_channels() {
    let Some(report) = cached_gpu_report() else {
        return;
    };
    assert!(report.ten_pirate_shared_cell_ok);
    let oracle = canonical_store_oracle();
    let materialization = canonical_materialization();
    let (location_id, _, _, cell_index) =
        canonical_pirate_shared_galactic_cell(&materialization);
    let at_cell = entries_at_cell_index(&oracle, location_id, cell_index);
    assert_eq!(at_cell.len(), 2);
    for entry in at_cell {
        assert_eq!(entry.key.owner, Owner::Pirate);
        assert!(matches!(
            entry.key.channel,
            ChannelKind::PiratePresence | ChannelKind::FleetStrength(Owner::Pirate)
        ));
    }
}

#[test]
fn gpu_preserves_constructed_planet_patrol_pirate_distinction() {
    let Some(report) = cached_gpu_report() else {
        return;
    };
    assert!(report.constructed_co_location_ok);
    let materialization = canonical_materialization();
    let extended = register_constructed_co_location_occupants(&materialization);
    let oracle = store_oracle_constructed_planet_patrol_pirate(&materialization);
    let loc = extended.location(LocationId(1)).expect("system");
    use dress_rehearsal_atlas_batch_0_store_gpu::cell_index;
    let index = cell_index(loc.map_base, loc.width, 3, 3);
    let at_cell = entries_at_cell_index(&oracle, LocationId(1), index);
    assert!(at_cell.len() >= 3);
}

#[test]
fn gpu_owner_indexed_entries_do_not_blind_sum_by_position() {
    let Some(report) = cached_gpu_report() else {
        return;
    };
    assert!(report.owner_channel_no_blind_sum_ok);
    let materialization = canonical_materialization();
    let extended = register_constructed_co_location_occupants(&materialization);
    let oracle = store_oracle_from_materialization(&extended);
    let loc = extended.location(LocationId(1)).expect("system");
    use dress_rehearsal_atlas_batch_0_store_gpu::cell_index;
    let index = cell_index(loc.map_base, loc.width, 3, 3);
    let at_cell = entries_at_cell_index(&oracle, LocationId(1), index);
    let terran = at_cell
        .iter()
        .find(|e| e.key.channel == ChannelKind::FleetStrength(Owner::Terran));
    let pirate = at_cell
        .iter()
        .find(|e| e.key.channel == ChannelKind::FleetStrength(Owner::Pirate));
    assert!(terran.is_some() && pirate.is_some());
    assert_ne!(terran.unwrap().value, pirate.unwrap().value);
}

#[test]
fn gpu_channel_entries_do_not_blind_sum_by_position() {
    let Some(report) = cached_gpu_report() else {
        return;
    };
    assert!(report.owner_channel_no_blind_sum_ok);
    let oracle = canonical_store_oracle();
    let ops = build_store_gpu_ops(&oracle, &build_store_gpu_fixture_layout(&oracle));
    assert!(ops.iter().any(|op| matches!(op.combine, simthing_core::CombineFn::Sum)));
    assert!(ops.iter().any(|op| {
        matches!(op.combine, simthing_core::CombineFn::EvalEML { .. })
    }));
}
