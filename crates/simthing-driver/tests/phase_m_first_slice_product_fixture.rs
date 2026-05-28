//! Phase M product fixture: product-style first-slice RegionFieldSpec over the
//! accepted GPU-resident first-slice runtime.

use simthing_driver::{
    compiled_stencil_to_gpu_config, estimate_first_slice_budget, FirstSliceMappingSession,
    FirstSliceSeed, FirstSliceTickOptions,
};
use simthing_gpu::{cpu_horizon, params_from_config, GpuContext, StructuredFieldStencilConfig};
use simthing_sim::PipelineFlags;
use simthing_spec::{
    compile_region_field_preview, deserialize_region_field_ron, MappingExecutionProfile,
    RegionFieldIsolationPolicyEstimate, RegionFieldSpec, SpecError,
};
use std::sync::Mutex;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const PRODUCT_FIXTURE_RON: &str =
    include_str!("fixtures/first_slice_product_suppression_field.ron");

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let ctx = GpuContext::new_blocking().expect("GPU required");
    f(&ctx);
}

fn product_spec() -> RegionFieldSpec {
    deserialize_region_field_ron(PRODUCT_FIXTURE_RON).expect("product fixture RON parses")
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn cpu_caller_managed_field(
    config: &StructuredFieldStencilConfig,
    seeds: &[(u32, u32, f32)],
) -> Vec<f32> {
    let params = params_from_config(config);
    let mut values = vec![0.0f32; config.values_len()];
    for &(row, col, value) in seeds {
        let slot = row * config.width + col;
        values[idx(slot, config.source_col, config.n_dims)] = value;
    }
    values = cpu_horizon(&values, &params, 1);
    for &(row, col, _) in seeds {
        let slot = row * config.width + col;
        values[idx(slot, config.source_col, config.n_dims)] = 0.0;
    }
    cpu_horizon(&values, &params, config.horizon)
}

fn assert_field_matches_cpu(
    spec: &RegionFieldSpec,
    field_values: &[f32],
    seeds: &[(u32, u32, f32)],
) {
    let preview = compile_region_field_preview(spec).expect("fixture admits");
    let config = compiled_stencil_to_gpu_config(&preview.stencil);
    let expected = cpu_caller_managed_field(&config, seeds);
    assert_eq!(field_values.len(), expected.len());
    for (i, (&gpu, &cpu)) in field_values.iter().zip(expected.iter()).enumerate() {
        assert!(
            (gpu - cpu).abs() <= 0.0001,
            "field mismatch at {i}: gpu={gpu} cpu={cpu}"
        );
    }
}

fn assert_region_field_err(spec: &RegionFieldSpec, reason_substr: &str) {
    let err = compile_region_field_preview(spec).expect_err("expected admission failure");
    match err {
        SpecError::RegionFieldAdmission { reason, .. } => {
            assert!(
                reason.contains(reason_substr),
                "expected `{reason_substr}` in `{reason}`"
            );
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn product_fixture_ron_admits_and_budget_passes() {
    let spec = product_spec();
    assert_eq!(spec.grid_size, 10);
    assert_eq!(spec.n_dims, 8);
    assert_eq!(spec.horizon, 8);
    assert!(!spec.request_atlas_batching);

    let preview = compile_region_field_preview(&spec).expect("fixture admits");
    assert_eq!(preview.name, "product_suppression_threat_field");
    assert_eq!(preview.cell_count, 100);
    assert!(preview.reduction.is_some());
    assert_eq!(
        preview.parent_formula_class.as_deref(),
        Some("field_urgency")
    );

    let budget =
        estimate_first_slice_budget(&spec, RegionFieldIsolationPolicyEstimate::SingleGridNoAtlas)
            .expect("budget preview passes");
    assert!(budget.estimated_bytes > 0);
    assert!(budget.estimated_bytes <= spec.max_region_field_vram_bytes.unwrap());
}

#[test]
fn product_fixture_default_profile_does_not_execute() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );

    with_gpu(|ctx| {
        let spec = product_spec();
        compile_region_field_preview(&spec).expect("spec presence admits");

        let mut session =
            FirstSliceMappingSession::open(ctx, MappingExecutionProfile::Disabled, &spec).unwrap();
        session
            .queue_seeds(&[FirstSliceSeed {
                row: 5,
                col: 5,
                value: 100.0,
            }])
            .unwrap();

        let report = session
            .tick(ctx, FirstSliceTickOptions::debug_readback(), (0.2, 0.1))
            .unwrap();
        assert!(!report.enabled);
        assert!(!report.scheduled);
        assert_eq!(report.source_setup_dispatches, 0);
        assert_eq!(report.propagation_dispatches, 0);
        assert_eq!(report.total_dispatches, 0);
        assert!(!report.reduction_executed);
        assert!(!report.eml_executed);
        assert_eq!(report.reduction_stencil_readbacks, 0);
        assert!(report.field_values.is_none());
        assert!(report.reduction_parent_value.is_none());
        assert!(report.eml_output.is_none());
        assert!(!report.readiness.mapping_enabled);
    });
}

#[test]
fn product_fixture_sparse_profile_executes_gpu_resident_hot_path() {
    with_gpu(|ctx| {
        let spec = product_spec();
        let mut session = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        session
            .queue_seeds(&[FirstSliceSeed {
                row: 5,
                col: 5,
                value: 100.0,
            }])
            .unwrap();
        assert!(
            session
                .scheduler()
                .regions()
                .first()
                .unwrap()
                .dirty
                .dirty_source_present
        );

        let report = session
            .tick(ctx, FirstSliceTickOptions::hot_path(), (0.2, 0.1))
            .unwrap();
        assert!(report.enabled);
        assert!(report.scheduled);
        assert_eq!(report.source_setup_dispatches, 1);
        assert_eq!(report.propagation_dispatches, spec.horizon);
        assert_eq!(report.total_dispatches, spec.horizon + 1);
        assert!(report.reduction_executed);
        assert!(report.eml_executed);
        assert_eq!(report.reduction_stencil_readbacks, 0);
        assert!(report.field_values.is_none());
        assert!(report.reduction_parent_value.is_none());
        assert!(report.eml_output.is_none());

        let readiness = &report.readiness;
        assert!(readiness.mapping_enabled);
        assert!(readiness.scheduled);
        assert_eq!(readiness.operator, "source_capped_normalized");
        assert_eq!(
            readiness.source_policy,
            "caller_managed_one_shot_seed_then_zero"
        );
        assert_eq!(readiness.boundary_mode, "zero");
        assert_eq!(
            readiness.gpu_bridge_bytes_copied,
            100 * spec.n_dims as u64 * 4
        );
        assert_eq!(readiness.gpu_bridge_slot_col_writes, 102);
        assert_eq!(readiness.reduction_stencil_readbacks, 0);
        assert!(readiness.budget_estimate_bytes.unwrap() > 0);
    });
}

#[test]
fn product_fixture_same_field_high_weight_yields_higher_urgency() {
    with_gpu(|ctx| {
        let spec = product_spec();
        let mut session = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        session
            .queue_seeds(&[FirstSliceSeed {
                row: 4,
                col: 4,
                value: 120.0,
            }])
            .unwrap();

        let hot_report = session
            .tick(ctx, FirstSliceTickOptions::hot_path(), (0.2, 0.1))
            .unwrap();
        assert!(hot_report.field_values.is_none());
        assert!(hot_report.reduction_parent_value.is_none());
        assert!(hot_report.eml_output.is_none());
        assert_eq!(hot_report.reduction_stencil_readbacks, 0);

        let (low_threat, low_urgency) = session
            .diagnostic_readback_reduction_eml(ctx, (0.2, 0.1))
            .unwrap();
        let (high_threat, high_urgency) = session
            .diagnostic_readback_reduction_eml(ctx, (0.9, 0.1))
            .unwrap();

        assert!(low_threat.is_finite() && low_threat > 0.0);
        assert!(high_threat.is_finite() && high_threat > 0.0);
        assert!((low_threat - high_threat).abs() < 0.01);
        assert!(low_urgency.is_finite());
        assert!(high_urgency.is_finite());
        assert!(
            high_urgency > low_urgency,
            "expected high profile urgency > low profile urgency: {high_urgency} <= {low_urgency}"
        );
    });
}

#[test]
fn product_fixture_edge_and_field_values_remain_finite() {
    with_gpu(|ctx| {
        let spec = product_spec();
        let seeds = [(0u32, 0u32, 100.0f32)];
        let mut session = FirstSliceMappingSession::open(
            ctx,
            MappingExecutionProfile::SparseRegionFieldV1,
            &spec,
        )
        .unwrap();
        session
            .queue_seeds(&[FirstSliceSeed {
                row: seeds[0].0,
                col: seeds[0].1,
                value: seeds[0].2,
            }])
            .unwrap();

        let report = session
            .tick(ctx, FirstSliceTickOptions::debug_readback(), (0.2, 0.1))
            .unwrap();
        let field = report.field_values.as_ref().unwrap();
        assert!(field.iter().all(|v| v.is_finite()));
        assert_field_matches_cpu(&spec, field, &seeds);

        let source_col = spec.source_col;
        let n_dims = spec.n_dims;
        assert!(field[idx(1, source_col, n_dims)] > 0.0);
        assert!(field[idx(10, source_col, n_dims)] > 0.0);
        assert_eq!(field[idx(99, source_col, n_dims)], 0.0);
        assert!(report.reduction_parent_value.unwrap() > 0.0);
        assert!(report.eml_output.unwrap().is_finite());
    });
}

#[test]
fn product_fixture_rejects_atlas_request() {
    let mut spec = product_spec();
    spec.request_atlas_batching = true;
    assert_region_field_err(&spec, "atlas batching");
}

#[test]
fn product_fixture_posture_preserved() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);

    let spec = product_spec();
    assert!(!spec.request_atlas_batching);

    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("RegionField"));
    assert!(!sim_lib.contains("MappingExecutionProfile"));

    let runtime_src = include_str!("../src/first_slice_mapping_runtime.rs");
    for source in [PRODUCT_FIXTURE_RON, runtime_src] {
        assert!(!source.contains("ActiveOnlyExperimentalNoHalo"));
        assert!(!source.contains("source_mask"));
        assert!(!source.contains("LocalBoundsMetadata"));
        assert!(!source.contains("AlgebraicTileLocalMask"));
        assert!(!source.contains("PhysicalGutter"));
        assert!(!source.contains("map residency"));
        assert!(!source.contains("semantic WGSL"));
    }
}
