//! C-8c transfer substrate parity and conservation tests.

use simthing_core::{
    ClampBehavior, DimensionRegistry, PropertyLayout, SimProperty, SubFieldRole, SubFieldSpec,
};
use simthing_gpu::{
    build_governed_pairs, execute_ops_cpu, plan_transfer_ops, plan_velocity_integration,
    set_debug_readback_allowed, AccumulatorInputListTable, AccumulatorPipelineSessions, GpuContext,
    Pipelines, TransferInputRef, TransferPlanError, TransferRegistration, WorldGpuState,
};

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

/// Registry with unbounded Named columns only — avoids legacy velocity pass clamping Amount.
fn transfer_registry_with_columns(cols: u32) -> DimensionRegistry {
    let mut reg = DimensionRegistry::new();
    let sub_fields: Vec<SubFieldSpec> = (0..cols)
        .map(|i| SubFieldSpec {
            role: SubFieldRole::Named(format!("col{i}")),
            width: 1,
            clamp: ClampBehavior::Unbounded,
            velocity_max: None,
            default: 0.0,
            display_name: format!("col{i}"),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: None,
        })
        .collect();
    reg.register(SimProperty {
        namespace: "transfer".into(),
        name: "resources".into(),
        layout: PropertyLayout { sub_fields },
        decay: None,
        intensity_behavior: None,
        fission_templates: vec![],
        fusion_templates: vec![],
        on_expire: None,
        description: String::new(),
        intensity_labels: vec![],
    });
    reg
}

fn setup_transfer_state(n_slots: u32, per_slot: &[f32]) -> WorldGpuState {
    let reg = transfer_registry_with_columns(per_slot.len() as u32);
    assert_eq!(reg.total_columns as usize, per_slot.len());
    let state = WorldGpuState::new(GpuContext::new_blocking().expect("gpu"), &reg, n_slots);
    assert_eq!(
        state.n_governed_pairs, 0,
        "transfer tests must not run legacy velocity"
    );
    let mut flat = vec![0.0_f32; state.values_len()];
    for slot in 0..n_slots {
        let base = slot as usize * state.n_dims as usize;
        for (col, &v) in per_slot.iter().enumerate() {
            flat[base + col] = v;
        }
    }
    state.install_resolved_values_at_boundary(&flat);
    state
}

fn run_accumulator_transfer(state: &mut WorldGpuState, dt: f32) -> Vec<f32> {
    let pipelines = Pipelines::new(&state.ctx);
    let mut transfer_session = state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .take_transfer_session();
    pipelines.run_tick_pipeline_with_accumulators(
        state,
        dt,
        AccumulatorPipelineSessions {
            intent: None,
            threshold: None,
            overlay_add: None,
            reduction_soft: None,
            velocity: None,
            intensity_eml: None,
            transfer: transfer_session.as_mut(),
            emission: None,
            encode_world_summary: false,
        },
    );
    state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .restore_transfer_session(transfer_session);
    state.read_values()
}

#[test]
fn c8c_single_source_transfer_conserves_exactly() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    set_debug_readback_allowed(true);
    let mut state = setup_transfer_state(1, &[10.0, 2.0]);
    let regs = vec![TransferRegistration {
        inputs: vec![TransferInputRef {
            slot: 0,
            col: 0,
            unit_cost: 1.0,
        }],
        target_slot: 0,
        target_col: 1,
        output_scale: 1.0,
        max_transfer: Some(3.0),
        tree_id: None,
        order_band: 0,
    }];
    state
        .sync_transfer_accumulator(&regs)
        .expect("C-8c transfer plan rejected: consumed input contention or invalid unit cost");
    let after = run_accumulator_transfer(&mut state, 1.0);
    assert_eq!(after[0].to_bits(), 7.0f32.to_bits());
    assert_eq!(after[1].to_bits(), 5.0f32.to_bits());
    assert_eq!(after[0] + after[1], 12.0);
}

#[test]
fn c8c_conjunctive_transfer_min_across_inputs() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    set_debug_readback_allowed(true);
    let mut state = setup_transfer_state(1, &[10.0, 9.0, 100.0, 0.0]);
    let regs = vec![TransferRegistration {
        inputs: vec![
            TransferInputRef {
                slot: 0,
                col: 0,
                unit_cost: 5.0,
            },
            TransferInputRef {
                slot: 0,
                col: 1,
                unit_cost: 3.0,
            },
            TransferInputRef {
                slot: 0,
                col: 2,
                unit_cost: 10.0,
            },
        ],
        target_slot: 0,
        target_col: 3,
        output_scale: 1.0,
        max_transfer: None,
        tree_id: None,
        order_band: 0,
    }];
    state
        .sync_transfer_accumulator(&regs)
        .expect("C-8c transfer plan rejected: consumed input contention or invalid unit cost");
    let after = run_accumulator_transfer(&mut state, 1.0);
    assert_eq!(after[0].to_bits(), 0.0f32.to_bits());
    assert_eq!(after[1].to_bits(), 3.0f32.to_bits());
    assert_eq!(after[2].to_bits(), 80.0f32.to_bits());
    assert_eq!(after[3].to_bits(), 2.0f32.to_bits());
}

#[test]
fn c8c_transfer_path_no_cpu_mediated_evaluation() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut state = setup_transfer_state(1, &[10.0, 0.0]);
    let regs = vec![TransferRegistration {
        inputs: vec![TransferInputRef {
            slot: 0,
            col: 0,
            unit_cost: 1.0,
        }],
        target_slot: 0,
        target_col: 1,
        output_scale: 1.0,
        max_transfer: Some(1.0),
        tree_id: None,
        order_band: 0,
    }];
    state
        .sync_transfer_accumulator(&regs)
        .expect("C-8c transfer plan rejected: consumed input contention or invalid unit cost");
    assert!(state.accumulator_transfer_active);
    let _ = run_accumulator_transfer(&mut state, 1.0);
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

fn run_accumulator_velocity_and_transfer(state: &mut WorldGpuState, dt: f32) -> Vec<f32> {
    let pipelines = Pipelines::new(&state.ctx);
    let runtime = state.accumulator_runtime.as_mut().unwrap();
    let mut velocity_session = runtime.take_velocity_session();
    let mut transfer_session = runtime.take_transfer_session();
    pipelines.run_tick_pipeline_with_accumulators(
        state,
        dt,
        AccumulatorPipelineSessions {
            intent: None,
            threshold: None,
            overlay_add: None,
            reduction_soft: None,
            velocity: velocity_session.as_mut(),
            intensity_eml: None,
            transfer: transfer_session.as_mut(),
            emission: None,
            encode_world_summary: false,
        },
    );
    state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .restore_velocity_session(velocity_session);
    state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .restore_transfer_session(transfer_session);
    state.read_values()
}

#[test]
fn c8c_rejects_nonfinite_transfer_values() {
    let regs = vec![TransferRegistration {
        inputs: vec![TransferInputRef {
            slot: 0,
            col: 0,
            unit_cost: 1.0,
        }],
        target_slot: 0,
        target_col: 1,
        output_scale: 1.0,
        max_transfer: Some(f32::NAN),
        tree_id: None,
        order_band: 0,
    }];
    assert_eq!(
        plan_transfer_ops(&regs),
        Err(TransferPlanError::InvalidMaxTransfer)
    );
}

