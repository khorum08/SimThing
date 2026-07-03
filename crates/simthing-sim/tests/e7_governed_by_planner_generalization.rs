//! E-7 — governed_by planner generalization beyond `(Amount, Velocity)`.

use std::path::Path;

use simthing_core::{
    ClampBehavior, DimensionRegistry, PropertyLayout, PropertyValue, SimProperty, SubFieldRole,
    SubFieldSpec,
};
use simthing_gpu::{
    build_governed_pairs, governed_pairs_for_property, plan_governed_integration, GpuContext,
    Pipelines, WorldGpuState,
};
use simthing_sim::PipelineFlags;

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn assert_bits_eq(label: &str, expected: &[f32], actual: &[f32]) {
    assert_eq!(expected.len(), actual.len(), "{label}: length mismatch");
    for (i, (a, b)) in expected.iter().zip(actual.iter()).enumerate() {
        assert_eq!(
            a.to_bits(),
            b.to_bits(),
            "{label}: index {i} diverges — expected={a}, actual={b}",
        );
    }
}

fn governed_amount_velocity_property(vel_max: Option<f32>, clamp: ClampBehavior) -> SimProperty {
    let mut p = SimProperty::simple("core", "governed", 0);
    for sf in &mut p.layout.sub_fields {
        if matches!(sf.role, SubFieldRole::Amount) {
            sf.velocity_max = vel_max;
            sf.clamp = clamp.clone();
        }
    }
    p
}

fn balance_flow_property(flow_max: Option<f32>, balance_clamp: ClampBehavior) -> SimProperty {
    SimProperty {
        namespace: "resource".into(),
        name: "arena".into(),
        layout: PropertyLayout {
            sub_fields: vec![
                SubFieldSpec {
                    role: SubFieldRole::Named("flow".into()),
                    width: 1,
                    clamp: ClampBehavior::Unbounded,
                    velocity_max: None,
                    default: 0.0,
                    display_name: "flow".into(),
                    display_range: None,
                    governed_by: None,
                    reduction_override: None,
                    soft_aggregate_guard: None,
                    accumulator_spec: None,
                },
                SubFieldSpec {
                    role: SubFieldRole::Named("balance".into()),
                    width: 1,
                    clamp: balance_clamp,
                    velocity_max: flow_max,
                    default: 0.0,
                    display_name: "balance".into(),
                    display_range: None,
                    governed_by: Some(SubFieldRole::Named("flow".into())),
                    reduction_override: None,
                    soft_aggregate_guard: None,
                    accumulator_spec: None,
                },
            ],
        },
        decay: None,
        intensity_behavior: None,
        fission_templates: vec![],
        fusion_templates: vec![],
        on_expire: None,
        description: String::new(),
        intensity_labels: vec![],
    }
}

fn setup_integration_state(
    reg: &DimensionRegistry,
    n_slots: u32,
    initial: &[f32],
) -> WorldGpuState {
    let mut state = WorldGpuState::new(GpuContext::new_blocking().expect("gpu"), reg, n_slots);
    let n_dims = state.n_dims as usize;
    let mut flat = vec![0.0_f32; state.values_len()];
    for (slot, row) in initial.chunks(n_dims).enumerate() {
        flat[slot * n_dims..slot * n_dims + n_dims].copy_from_slice(row);
    }
    state.install_resolved_values_at_boundary(&flat);

    state.ensure_velocity_accumulator();
    let pairs = build_governed_pairs(reg);
    let plan = plan_governed_integration(&pairs, n_slots);
    state
        .upload_velocity_ops_with_bands(&plan.ops, plan.n_bands)
        .expect("governed integration upload");
    state
}

fn cpu_integrate_rows(
    layout: &simthing_core::PropertyLayout,
    rows: &[Vec<f32>],
    dt: f32,
) -> Vec<f32> {
    let stride = layout.stride();
    let mut out = Vec::with_capacity(rows.len() * stride);
    for row in rows {
        let mut pv = PropertyValue {
            data: row[..stride].to_vec(),
        };
        pv.integrate(&layout, dt);
        out.extend_from_slice(&pv.data);
    }
    out
}

fn run_accumulator_integration(state: &mut WorldGpuState, dt: f32) -> Vec<f32> {
    let pipelines = Pipelines::new(&state.ctx);
    let mut runtime = state.accumulator_runtime.take().unwrap();
    let mut velocity_session = runtime.take_velocity_session();
    pipelines.run_tick_pipeline_with_accumulators(
        state,
        dt,
        simthing_gpu::AccumulatorPipelineSessions {
            intent: None,
            threshold: None,
            overlay_add: None,
            reduction_soft: None,
            velocity: velocity_session.as_mut(),
            intensity_eml: None,
            transfer: None,
            emission: None,
            encode_world_summary: false,
        },
    );
    runtime.restore_velocity_session(velocity_session);
    state.accumulator_runtime = Some(runtime);
    state.read_values()
}

#[test]
fn e7_amount_velocity_existing_path_bit_exact() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut reg = DimensionRegistry::new();
    let pid = reg.register(governed_amount_velocity_property(
        Some(10.0),
        ClampBehavior::Bounded {
            min: 0.0,
            max: 100.0,
        },
    ));
    let layout = reg.property(pid).layout.clone();
    let n_dims = reg.total_columns;
    let mut row = vec![0.0_f32; n_dims];
    row[0] = 10.0;
    row[1] = 2.0;

    let mut state = setup_integration_state(&reg, 1, &row);
    let dt = 0.5;
    let expected = cpu_integrate_rows(&layout, std::slice::from_ref(&row), dt);
    let actual = run_accumulator_integration(&mut state, dt);
    assert_bits_eq("amount/velocity", &expected, &actual);
    assert_eq!(actual[0].to_bits(), 11.0_f32.to_bits());
    assert_eq!(actual[1].to_bits(), 2.0_f32.to_bits());
}

#[test]
fn e7_named_balance_flow_integrates_correctly() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut reg = DimensionRegistry::new();
    let pid = reg.register(balance_flow_property(
        Some(10.0),
        ClampBehavior::Bounded {
            min: 0.0,
            max: 1000.0,
        },
    ));
    let layout = reg.property(pid).layout.clone();
    let flow_off = layout
        .offset_of(&SubFieldRole::Named("flow".into()))
        .unwrap();
    let balance_off = layout
        .offset_of(&SubFieldRole::Named("balance".into()))
        .unwrap();

    let pairs = build_governed_pairs(&reg);
    assert_eq!(pairs.len(), 1);
    assert_eq!(pairs[0].governing_col, flow_off as u32);
    assert_eq!(pairs[0].governed_col, balance_off as u32);

    let n_dims = reg.total_columns;
    let mut row = vec![0.0_f32; n_dims];
    row[flow_off] = 3.0;
    row[balance_off] = 10.0;

    let mut state = setup_integration_state(&reg, 1, &row);
    let dt = 0.5;
    let expected = cpu_integrate_rows(&layout, std::slice::from_ref(&row), dt);
    let actual = run_accumulator_integration(&mut state, dt);
    assert_bits_eq("balance/flow", &expected, &actual);
    assert_eq!(actual[balance_off].to_bits(), 11.5_f32.to_bits());
    assert_eq!(actual[flow_off].to_bits(), 3.0_f32.to_bits());
}

#[test]
fn e7_named_pair_respects_clamp_behavior() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut reg = DimensionRegistry::new();
    let pid = reg.register(balance_flow_property(
        Some(10.0),
        ClampBehavior::Bounded { min: 0.0, max: 1.0 },
    ));
    let layout = reg.property(pid).layout.clone();
    let flow_off = layout
        .offset_of(&SubFieldRole::Named("flow".into()))
        .unwrap();
    let balance_off = layout
        .offset_of(&SubFieldRole::Named("balance".into()))
        .unwrap();
    let n_dims = reg.total_columns;

    let mut row = vec![0.0_f32; n_dims];
    row[flow_off] = 0.5;
    row[balance_off] = 0.9;

    let mut state = setup_integration_state(&reg, 1, &row);
    let expected = cpu_integrate_rows(&layout, std::slice::from_ref(&row), 1.0);
    let actual = run_accumulator_integration(&mut state, 1.0);
    assert_bits_eq("balance clamp max", &expected, &actual);
    assert_eq!(actual[balance_off].to_bits(), 1.0_f32.to_bits());
}
#[test]
fn e7_no_legacy_velocity_shader_or_pipeline_dependency() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-gpu/src/shaders/velocity_integration.wgsl");
    assert!(
        !path.exists(),
        "legacy velocity shader still exists: {path:?}"
    );
    assert!(PipelineFlags::default().use_accumulator_velocity);
}
