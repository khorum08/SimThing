//! BH3-CLOSEOUT PR8 — canonical sample driver admit/install + GPU-resident exercise.
//!
//! Test A proves generic install/admission with default-off posture (CPU path; optional
//! session check when GPU is present). Test B explicitly opts into the existing
//! first-slice mapping harness for bounded GPU-resident SaturatingFlux + commitment
//! evidence and a compact PALMA D probe. No full-field CPU decision readback.

use std::collections::HashMap;
use std::sync::Mutex;

use simthing_clausething::{
    hydrate_scenario, parse_raw_document, HydratedScenarioPack, HydratedScenarioPalmaFeedstock,
};
use simthing_core::DimensionRegistry;
use simthing_driver::{
    compiled_w_impedance_compose_to_gpu_config, composed_w_min_plus_stencil_config, install_atomic,
    FirstSliceMappingSession, FirstSliceSeed, FirstSliceTickOptions, Scenario, SimSession,
};
use simthing_gpu::wgpu::{self, util::DeviceExt};
use simthing_gpu::{
    GpuContext, MinPlusTraversalDProbeConfig, MinPlusTraversalDProbeOp,
    MinPlusTraversalExecutionOptions, MinPlusTraversalFieldOp, MinPlusTraversalInput,
    MinPlusTraversalWInputKind, WImpedanceComposeOp, MIN_PLUS_INF,
};
use simthing_spec::{
    compile_region_field_preview, compile_w_impedance_compose_preview, CompiledRegionFieldOperator,
    MappingExecutionProfile, RegionFieldOperatorSpec, WImpedanceComposeProfileSpec,
    WImpedanceComposeSpec,
};

const CANONICAL_SAMPLE: &str =
    include_str!("../../simthing-clausething/tests/fixtures/ct_bh3_closeout_sample.clause");

const TRAVERSAL_ITERATIONS: u32 = 4;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const FORBIDDEN_HOT_PATH: &[&str] = &[
    "sqrt",
    "length(",
    "distance",
    "normalize",
    "hypot",
    "pathfinding",
    "movement_engine",
    "predecessor",
    "cpu_planner",
];

fn try_gpu() -> bool {
    GpuContext::new_blocking().is_ok()
}

fn hydrate_canonical_sample() -> HydratedScenarioPack {
    let document = parse_raw_document(CANONICAL_SAMPLE.as_bytes()).expect("parse canonical sample");
    hydrate_scenario(&document).expect("hydrate canonical sample")
}

fn scenario_from_pack(pack: &HydratedScenarioPack) -> Scenario {
    // Mirror the `open_from_spec` convention: the base scenario registry carries only a
    // placeholder; install registers the authored spec properties (see session_integration.rs).
    let mut registry = DimensionRegistry::new();
    let _ = registry.register(simthing_core::SimProperty::simple(
        "_placeholder",
        "seed",
        0,
    ));
    let slot_count = count_simthings(&pack.root) as u32;
    Scenario {
        name: pack.scenario_id.clone(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: slot_count.max(32),
        registry,
        root: pack.root.clone(),
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: pack
            .install_targets
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<HashMap<_, _>>(),
    }
}

fn count_simthings(root: &simthing_core::SimThing) -> usize {
    1 + root.children.iter().map(count_simthings).sum::<usize>()
}

fn eml_weights(pack: &HydratedScenarioPack) -> (f32, f32) {
    let formula = pack.game_mode.region_fields[0]
        .parent_formula
        .as_ref()
        .expect("commitment parent_formula");
    (
        formula.weight_pressure.expect("weight_pressure"),
        formula.weight_resource.expect("weight_resource"),
    )
}

/// Test-only bridge: the canonical SaturatingFlux authors a single choke readout, but the generic
/// W-impedance compose operator consumes two distinct choke inputs. Use the smallest column index
/// not already claimed by the sample's source / authored choke / PALMA W/D outputs as the operator's
/// second (null) choke input, keeping every compose column distinct without fabricating semantics.
fn spare_choke_b_col(palma: &HydratedScenarioPalmaFeedstock) -> u32 {
    let choke_a = palma
        .choke_output_col
        .expect("palma feedstock requires saturating_flux choke_output_col");
    let claimed = [
        palma.source_col,
        choke_a,
        palma.w_output_col,
        palma.d_output_col,
    ];
    (0..palma.n_dims)
        .find(|col| !claimed.contains(col))
        .expect("canonical sample n_dims must leave a spare column for the second choke input")
}

/// Test-only bridge: derive generic W compose admission from PR5 PALMA feedstock DTO.
fn w_compose_spec_from_palma(palma: &HydratedScenarioPalmaFeedstock) -> WImpedanceComposeSpec {
    let choke_a = palma
        .choke_output_col
        .expect("palma feedstock requires saturating_flux choke_output_col");
    WImpedanceComposeSpec {
        width: palma.grid_size,
        height: palma.grid_size,
        n_dims: palma.n_dims,
        base_w_col: palma.source_col,
        choke_a_col: choke_a,
        choke_b_col: spare_choke_b_col(palma),
        profiles: vec![WImpedanceComposeProfileSpec {
            weight_a: 1.0,
            weight_b: 1.0,
            output_w_col: palma.w_output_col,
        }],
    }
}

fn assert_palma_feedstock_admitted(pack: &HydratedScenarioPack) {
    let palma = pack
        .palma_feedstock
        .as_ref()
        .expect("palma feedstock metadata");
    assert_eq!(palma.w_source_field_operator_id, "alpha_choke_flux");
    assert_eq!(palma.w_output_col, 3);
    assert_eq!(palma.d_output_col, 4);

    let w_spec = w_compose_spec_from_palma(palma);
    let compiled = compile_w_impedance_compose_preview(&w_spec).expect("w compose admission");
    let w_gpu = compiled_w_impedance_compose_to_gpu_config(&compiled);
    let _stencil =
        composed_w_min_plus_stencil_config(&w_gpu, 0, palma.d_output_col, (0, 0), MIN_PLUS_INF);
}

#[test]
fn closeout_sample_admits_installs_and_honors_default_off() {
    let pack = hydrate_canonical_sample();
    assert_eq!(pack.scenario_id, "ct_bh3_closeout_sample");
    assert_eq!(
        pack.game_mode.mapping_execution_profile,
        MappingExecutionProfile::Disabled
    );
    assert!(!pack.game_mode.mapping_execution_profile.enables_execution());
    assert_eq!(pack.root_node.children.len(), 3);
    assert_eq!(pack.grid_metadata.links.len(), 2);
    assert!(pack.palma_feedstock.is_some());
    assert!(pack.commitment.is_some());
    assert!(!pack.game_mode.properties.is_empty());
    assert!(!pack.game_mode.overlays.is_empty());

    let field = &pack.game_mode.region_fields[0];
    let preview = compile_region_field_preview(field).expect("region field admission");
    assert!(matches!(
        preview.stencil.operator,
        CompiledRegionFieldOperator::SaturatingFlux { .. }
    ));
    assert!(preview.commitment.is_some());
    assert!(field.parent_formula.is_some());
    assert!(field.reduction.is_some());
    assert_palma_feedstock_admitted(&pack);

    let scenario = scenario_from_pack(&pack);
    let mut registry = scenario.registry.clone();
    let mut root = pack.root.clone();
    let mut allocator = simthing_gpu::SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let spec_state = install_atomic(
        &pack.game_mode,
        &scenario,
        &mut registry,
        &mut root,
        &mut allocator,
    )
    .expect("install canonical sample");
    assert_eq!(
        root.children.len(),
        3,
        "install preserves scenario locations"
    );
    assert!(
        !pack.game_mode.properties.is_empty(),
        "install must preserve authored scenario properties"
    );
    let _ = spec_state;

    if try_gpu() {
        let session =
            SimSession::open_from_spec(scenario, &pack.game_mode).expect("open_from_spec");
        assert!(
            session.mapping.is_none(),
            "default-off profile must not wire session mapping"
        );
        assert!(session.mapping_commitments.is_empty());
    }
}

#[test]
fn closeout_sample_gpu_resident_path_exercises_compact_evidence() {
    let Ok(ctx) = GpuContext::new_blocking() else {
        eprintln!("skipping: no GPU adapter available");
        return;
    };
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let pack = hydrate_canonical_sample();
    let field = pack.game_mode.region_fields[0].clone();
    let preview = compile_region_field_preview(&field).expect("region field admission");
    let commitment = preview.commitment.expect("commitment admitted");
    let weights = eml_weights(&pack);

    let mut mapping =
        FirstSliceMappingSession::open(&ctx, MappingExecutionProfile::SparseRegionFieldV1, &field)
            .expect("open first-slice mapping");
    mapping
        .queue_seeds(&[FirstSliceSeed {
            row: 4,
            col: 4,
            value: 120.0,
        }])
        .expect("queue seed");

    let report = mapping
        .tick_with_commitment_spec_fixture(
            &ctx,
            FirstSliceTickOptions::hot_path(),
            weights,
            &commitment,
        )
        .expect("gpu mapping tick");
    assert!(report.mapping.enabled);
    assert!(report.mapping.scheduled);
    assert_eq!(report.mapping.reduction_stencil_readbacks, 0);
    assert!(report.mapping.field_values.is_none());
    assert!(report.mapping.reduction_parent_value.is_none());
    assert!(report.mapping.eml_output.is_none());
    assert!(report.mapping.summary.summary_used_for_commitment_scan);

    let (threat, urgency) = mapping
        .diagnostic_readback_reduction_eml(&ctx, weights)
        .expect("compact diagnostic readback");
    assert!(threat.is_finite());
    assert!(urgency.is_finite());
    assert!(
        urgency > commitment.threshold,
        "urgency {urgency} must cross authored threshold {}",
        commitment.threshold
    );
    assert_eq!(report.threshold_events.len(), 1);
    assert_eq!(report.threshold_events[0].event_kind, commitment.event_kind);

    let palma = pack.palma_feedstock.as_ref().expect("palma feedstock");
    let w_spec = w_compose_spec_from_palma(palma);
    let w_compiled = compile_w_impedance_compose_preview(&w_spec).expect("w admission");
    let w_gpu = compiled_w_impedance_compose_to_gpu_config(&w_compiled);
    let stencil =
        composed_w_min_plus_stencil_config(&w_gpu, 0, palma.d_output_col, (4, 4), MIN_PLUS_INF);

    let width = field.grid_size;
    let height = field.grid_size;
    let cells = (width * height) as usize;
    let n_dims = field.n_dims as usize;
    let mut values = vec![0.0f32; cells * n_dims];
    let idx = |slot: u32, col: u32| (slot as usize * n_dims) + col as usize;
    let choke_a = match &field.operator {
        RegionFieldOperatorSpec::SaturatingFlux {
            choke_output_col: Some(col),
            ..
        } => *col,
        _ => panic!("canonical sample field operator must author a choke output column"),
    };
    let choke_b = spare_choke_b_col(palma);
    for slot in 0..cells as u32 {
        values[idx(slot, field.source_col)] = 1.0;
        values[idx(slot, choke_a)] = 0.75;
        // The canonical sample authors only one choke readout; the generic operator's second
        // choke input is the null spare column (no second authored impedance).
        values[idx(slot, choke_b)] = 0.0;
    }

    let values_buffer = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("closeout_sample_interleaved"),
            contents: bytemuck::cast_slice(&values),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

    WImpedanceComposeOp::new(&ctx)
        .compose_resident_field(&ctx, &values_buffer, &w_gpu)
        .expect("w compose dispatch");

    let traversal =
        MinPlusTraversalFieldOp::new(&ctx, stencil.clone()).expect("min-plus traversal");
    let traversal_report = traversal
        .dispatch_traversal_from_input(
            &ctx,
            MinPlusTraversalInput::GpuInterleavedW(&values_buffer),
            None,
            MinPlusTraversalExecutionOptions::gpu_resident(TRAVERSAL_ITERATIONS),
        )
        .expect("resident D relaxation");
    assert_eq!(
        traversal_report.w_input_kind,
        MinPlusTraversalWInputKind::GpuInterleavedW
    );
    assert!(traversal_report.gpu_resident);
    assert!(
        traversal_report.values.is_none(),
        "production traversal must not read back full D"
    );

    let resident = traversal.output_handle(TRAVERSAL_ITERATIONS);
    let probe_config = MinPlusTraversalDProbeConfig::from_stencil_config(&stencil);
    let probe_cell = 4 * width + 4;
    let probe_result = MinPlusTraversalDProbeOp::new(&ctx)
        .probe_resident_d(
            &ctx,
            resident,
            &probe_config,
            &[probe_cell],
            stencil.cells(),
        )
        .expect("compact D probe");
    assert_eq!(probe_result.gathered.len(), 1);
    assert!(probe_result.gathered[0].is_finite());
    assert!(probe_result.min_d.is_finite());

    let bridge_src = include_str!("../src/w_impedance_compose_bridge.rs");
    for token in FORBIDDEN_HOT_PATH {
        assert!(
            !bridge_src.contains(token),
            "bridge must not contain forbidden token `{token}`"
        );
    }
}
