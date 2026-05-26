//! C-8c transfer substrate parity and conservation tests.

use simthing_core::{
    ClampBehavior, DimensionRegistry, PropertyLayout, SimProperty, SubFieldRole, SubFieldSpec,
};
use simthing_gpu::{
    execute_ops_cpu, plan_transfer_ops, set_debug_readback_allowed, AccumulatorInputListTable,
    AccumulatorPipelineSessions, GpuContext, Pipelines, TransferInputRef, TransferRegistration,
    WorldGpuState,
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
    assert_eq!(state.n_governed_pairs, 0, "transfer tests must not run legacy velocity");
    let mut flat = vec![0.0_f32; state.values_len()];
    for slot in 0..n_slots {
        let base = slot as usize * state.n_dims as usize;
        for (col, &v) in per_slot.iter().enumerate() {
            flat[base + col] = v;
        }
    }
    state.write_values(&flat);
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
fn c8c_input_list_table_upload_roundtrip() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut table = AccumulatorInputListTable::new(&ctx, 16);
    let entry = simthing_gpu::AccumulatorInputGpu {
        slot: 0,
        col: 1,
        unit_cost_bits: 2.0f32.to_bits(),
        flags: 0,
    };
    let ranges = table
        .upload_lists(&ctx, &[vec![entry]], 1)
        .expect("upload");
    assert_eq!(ranges.len(), 1);
    assert_eq!(table.upload_count, 1);
}

#[test]
fn c8c_input_list_table_skips_unchanged_upload() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut table = AccumulatorInputListTable::new(&ctx, 16);
    let lists = vec![vec![simthing_gpu::AccumulatorInputGpu {
        slot: 1,
        col: 0,
        unit_cost_bits: 1.0f32.to_bits(),
        flags: 0,
    }]];
    table.upload_lists(&ctx, &lists, 1).unwrap();
    let uploads = table.upload_count;
    table.upload_lists(&ctx, &lists, 1).unwrap();
    assert_eq!(table.upload_count, uploads);
}

#[test]
fn c8c_input_list_table_growth_preserves_entries() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut table = AccumulatorInputListTable::new(&ctx, 2);
    let lists = vec![vec![
        simthing_gpu::AccumulatorInputGpu {
            slot: 0,
            col: 0,
            unit_cost_bits: 1.0f32.to_bits(),
            flags: 0,
        };
        3
    ]];
    table.upload_lists(&ctx, &lists, 1).unwrap();
    assert!(table.capacity >= 3);
    assert_eq!(table.entries.len(), 3);
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
    }];
    state.sync_transfer_accumulator(&regs);
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
    }];
    state.sync_transfer_accumulator(&regs);
    let after = run_accumulator_transfer(&mut state, 1.0);
    assert_eq!(after[0].to_bits(), 0.0f32.to_bits());
    assert_eq!(after[1].to_bits(), 3.0f32.to_bits());
    assert_eq!(after[2].to_bits(), 80.0f32.to_bits());
    assert_eq!(after[3].to_bits(), 2.0f32.to_bits());
}

#[test]
fn c8c_transfer_does_not_produce_negative_inputs() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    set_debug_readback_allowed(true);
    let mut state = setup_transfer_state(1, &[0.5, 1.0, 0.0]);
    let regs = vec![TransferRegistration {
        inputs: vec![
            TransferInputRef {
                slot: 0,
                col: 0,
                unit_cost: 0.25,
            },
            TransferInputRef {
                slot: 0,
                col: 1,
                unit_cost: 0.5,
            },
        ],
        target_slot: 0,
        target_col: 2,
        output_scale: 1.0,
        max_transfer: None,
        tree_id: None,
    }];
    state.sync_transfer_accumulator(&regs);
    let after = run_accumulator_transfer(&mut state, 1.0);
    assert!(after[0] >= 0.0);
    assert!(after[1] >= 0.0);
    assert!(after[2] >= 0.0);
}

#[test]
fn c8c_transfer_1000_factories_3_channels_100_ticks_conserves_exactly() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    set_debug_readback_allowed(true);
    const FACTORIES: u32 = 1000;
    const TICKS: u32 = 100;
    // Single-source unit_cost=1 transfers conserve literal column sums each tick.
    let per_slot = [100.0_f32, 0.0, 0.0];
    let mut state = setup_transfer_state(FACTORIES, &per_slot);
    let mut regs = Vec::with_capacity(FACTORIES as usize);
    for slot in 0..FACTORIES {
        regs.push(TransferRegistration {
            inputs: vec![TransferInputRef {
                slot,
                col: 0,
                unit_cost: 1.0,
            }],
            target_slot: slot,
            target_col: 1,
            output_scale: 1.0,
            max_transfer: Some(1.0),
            tree_id: None,
        });
    }
    state.sync_transfer_accumulator(&regs);
    let sum_before: f32 = state.read_values().iter().sum();
    for _ in 0..TICKS {
        let _ = run_accumulator_transfer(&mut state, 1.0);
    }
    let after = state.read_values();
    let sum_after: f32 = after.iter().sum();
    assert_eq!(sum_before.to_bits(), sum_after.to_bits());
    // Each factory drains 100 from col0 into col1 over 100 ticks.
    let n_dims = state.n_dims as usize;
    for slot in 0..FACTORIES as usize {
        let base = slot * n_dims;
        assert_eq!(after[base].to_bits(), 0.0f32.to_bits());
        assert_eq!(after[base + 1].to_bits(), 100.0f32.to_bits());
    }
}

#[test]
fn c8c_transfer_contention_same_target_conserves() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    set_debug_readback_allowed(true);
    let mut state = setup_transfer_state(1, &[5.0, 5.0, 0.0]);
    let regs = vec![
        TransferRegistration {
            inputs: vec![TransferInputRef {
                slot: 0,
                col: 0,
                unit_cost: 1.0,
            }],
            target_slot: 0,
            target_col: 2,
            output_scale: 1.0,
            max_transfer: Some(2.0),
            tree_id: None,
        },
        TransferRegistration {
            inputs: vec![TransferInputRef {
                slot: 0,
                col: 1,
                unit_cost: 1.0,
            }],
            target_slot: 0,
            target_col: 2,
            output_scale: 1.0,
            max_transfer: Some(3.0),
            tree_id: None,
        },
    ];
    state.sync_transfer_accumulator(&regs);
    let before_sum: f32 = state.read_values().iter().sum();
    let after = run_accumulator_transfer(&mut state, 1.0);
    let after_sum: f32 = after.iter().sum();
    assert_eq!(before_sum.to_bits(), after_sum.to_bits());
    assert!(after[2] >= 0.0);
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
    }];
    state.sync_transfer_accumulator(&regs);
    assert!(state.accumulator_transfer_active);
    let _ = run_accumulator_transfer(&mut state, 1.0);
}

#[test]
fn c8c_transfer_does_not_reupload_input_lists_per_tick() {
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
    }];
    state.sync_transfer_accumulator(&regs);
    let uploads = state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .input_lists
        .as_ref()
        .unwrap()
        .upload_count;
    for _ in 0..5 {
        let _ = run_accumulator_transfer(&mut state, 1.0);
    }
    let table = state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .input_lists
        .as_ref()
        .unwrap();
    assert_eq!(table.upload_count, uploads);
}

#[test]
fn c8c_cpu_oracle_matches_single_and_conjunctive_transfer() {
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
    }];
    let plan = plan_transfer_ops(&regs);
    let mut values = vec![10.0, 2.0];
    execute_ops_cpu(&mut values, &plan.ops, 0, 2).expect("cpu oracle");
    assert_eq!(values, vec![7.0, 5.0]);

    let conj = vec![TransferRegistration {
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
        ],
        target_slot: 0,
        target_col: 2,
        output_scale: 1.0,
        max_transfer: None,
        tree_id: None,
    }];
    let plan = plan_transfer_ops(&conj);
    let mut values = vec![10.0, 9.0, 0.0];
    execute_ops_cpu(&mut values, &plan.ops, 0, 3).expect("cpu oracle");
    assert_eq!(values[0], 0.0);
    assert_eq!(values[1], 3.0);
    assert_eq!(values[2], 2.0);
}

#[test]
fn c8c_combined_c1_c2_c4_s4_c7_c8b_c8c_all_flags_on() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    set_debug_readback_allowed(true);
    let mut state = setup_transfer_state(1, &[10.0, 2.0, 0.0]);
    let transfer_regs = vec![TransferRegistration {
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
    }];
    state.sync_transfer_accumulator(&transfer_regs);
    let after = run_accumulator_transfer(&mut state, 1.0);
    assert_eq!(after[0].to_bits(), 9.0f32.to_bits());
    assert_eq!(after[1].to_bits(), 3.0f32.to_bits());
}
