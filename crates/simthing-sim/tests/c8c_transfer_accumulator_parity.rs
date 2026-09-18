//! C-8c transfer substrate parity and conservation tests.

use simthing_core::{
    ClampBehavior, ColumnIndex, DimensionRegistry, PropertyLayout, SimProperty, SubFieldRole,
    SubFieldSpec,
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
        admission_disposition: Default::default(),
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
            col: ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
            unit_cost: 1.0,
        }],
        target_slot: 0,
        target_col: ColumnIndex::from_raw_for_oracle_or_rehearsal(1),
        output_scale: 1.0,
        max_transfer: Some(3.0),
        max_units_per_generation: None,
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
                col: ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
                unit_cost: 5.0,
            },
            TransferInputRef {
                slot: 0,
                col: ColumnIndex::from_raw_for_oracle_or_rehearsal(1),
                unit_cost: 3.0,
            },
            TransferInputRef {
                slot: 0,
                col: ColumnIndex::from_raw_for_oracle_or_rehearsal(2),
                unit_cost: 10.0,
            },
        ],
        target_slot: 0,
        target_col: ColumnIndex::from_raw_for_oracle_or_rehearsal(3),
        output_scale: 1.0,
        max_transfer: None,
        max_units_per_generation: None,
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

fn cap_col(i: usize) -> ColumnIndex {
    ColumnIndex::from_raw_for_oracle_or_rehearsal(i)
}

fn capped_recipe(
    inputs: &[(usize, f32)],
    target: usize,
    output_scale: f32,
    cap: Option<u32>,
) -> TransferRegistration {
    TransferRegistration {
        inputs: inputs
            .iter()
            .map(|&(col, unit_cost)| TransferInputRef {
                slot: 0,
                col: cap_col(col),
                unit_cost,
            })
            .collect(),
        target_slot: 0,
        target_col: cap_col(target),
        output_scale,
        max_transfer: None,
        max_units_per_generation: cap.map(|units| std::num::NonZeroU32::new(units).unwrap()),
        tree_id: None,
        order_band: 0,
    }
}

/// Execute `phases` of generations on the GPU AND on the CPU oracle. Before
/// each phase the given stocks are installed at the boundary. The two paths
/// must agree bit-exactly; the agreed stocks are returned.
fn capped_generations(regs: &[TransferRegistration], phases: &[(&[f32], u32)]) -> Vec<f32> {
    let n = phases[0].0.len();
    let mut state = setup_transfer_state(1, phases[0].0);
    state
        .sync_transfer_accumulator(regs)
        .expect("capped conjunctive plan admits");
    let plan = plan_transfer_ops(regs).expect("capped conjunctive plan");
    let mut gpu = phases[0].0.to_vec();
    let mut cpu = phases[0].0.to_vec();
    for &(stocks, generations) in phases {
        state.install_resolved_values_at_boundary(stocks);
        cpu = stocks.to_vec();
        for _ in 0..generations {
            gpu = run_accumulator_transfer(&mut state, 1.0)[..n].to_vec();
            for band in 0..plan.n_bands {
                execute_ops_cpu(&mut cpu, &plan.ops, band, n as u32).expect("CPU oracle");
            }
        }
    }
    let bits = |values: &[f32]| values.iter().map(|v| v.to_bits()).collect::<Vec<_>>();
    assert_eq!(bits(&gpu), bits(&cpu), "GPU and CPU oracle agree bit-exactly");
    gpu
}

/// catches: a recipe-unit cap applied to the target write only (inputs debited
/// at the uncapped count), a cap that banks unused units across generations,
/// a cap that overrides min-across-input affordability, or an uncapped legacy
/// recipe no longer emitting every affordable unit (DA, relay 5735839909).
#[test]
fn c8c_conjunctive_recipe_unit_cap_is_atomic_and_per_generation() {
    let Some(_ctx) = try_gpu() else {
        panic!("recipe-unit cap witness requires a GPU adapter");
    };
    set_debug_readback_allowed(true);
    // Columns: a b c t d e u — a/2 + b/1 affords 10 units, c/3 affords 3.
    let stocks: &[f32] = &[20.0, 10.0, 9.0, 0.0, 6.0, 12.0, 0.0];
    let two = [(0, 2.0), (1, 1.0)];
    let run = |regs: Vec<TransferRegistration>| capped_generations(&regs, &[(stocks, 1)]);

    let one = run(vec![capped_recipe(&two, 3, 1.0, Some(1))]);
    assert_eq!(&one[..4], &[18.0, 9.0, 9.0, 1.0], "cap 1: one unit, both inputs debit once");
    let two_units = run(vec![capped_recipe(&two, 3, 1.0, Some(2))]);
    assert_eq!(&two_units[..4], &[16.0, 8.0, 9.0, 2.0], "cap 2: exactly two units");
    let legacy = run(vec![capped_recipe(&two, 3, 1.0, None)]);
    assert_eq!(&legacy[..4], &[0.0, 0.0, 9.0, 10.0], "uncapped emits every affordable unit");
    let reordered = run(vec![capped_recipe(&[(1, 1.0), (0, 2.0)], 3, 1.0, Some(1))]);
    assert_eq!(reordered, one, "input order carries no economics");

    let three = [(0, 2.0), (1, 1.0), (2, 3.0)];
    let wide = run(vec![capped_recipe(&three, 3, 1.0, Some(2))]);
    assert_eq!(&wide[..4], &[16.0, 8.0, 3.0, 2.0], "three-input recipe honours the cap");
    let limited = run(vec![capped_recipe(&three, 3, 1.0, Some(5))]);
    assert_eq!(&limited[..4], &[14.0, 7.0, 0.0, 3.0], "below the cap, the limiting input rules");

    let scaled = run(vec![capped_recipe(&two, 3, 1.5, Some(1))]);
    assert_eq!(&scaled[..4], &[18.0, 9.0, 9.0, 1.5], "coefficient scales credit, not debit");

    let pair = run(vec![
        capped_recipe(&two, 3, 1.0, Some(1)),
        capped_recipe(&[(4, 3.0), (5, 4.0)], 6, 1.0, Some(2)),
    ]);
    assert_eq!(pair, vec![18.0, 9.0, 9.0, 1.0, 0.0, 4.0, 2.0], "same-band recipes cap independently");

    let cadence = capped_generations(&[capped_recipe(&two, 3, 1.0, Some(1))], &[(stocks, 3)]);
    assert_eq!(&cadence[..4], &[14.0, 7.0, 9.0, 3.0], "one unit per generation over three");
    let starved: &[f32] = &[20.0, 0.0, 9.0, 0.0, 6.0, 12.0, 0.0];
    let no_carry = capped_generations(
        &[capped_recipe(&two, 3, 1.0, Some(1))],
        &[(starved, 2), (stocks, 1)],
    );
    assert_eq!(&no_carry[..4], &[18.0, 9.0, 9.0, 1.0], "idle generations bank nothing");

    let fixed = TransferRegistration {
        inputs: vec![TransferInputRef {
            slot: 0,
            col: cap_col(0),
            unit_cost: 1.0,
        }],
        max_transfer: Some(3.0),
        ..capped_recipe(&[(0, 1.0)], 3, 1.0, Some(1))
    };
    assert_eq!(
        plan_transfer_ops(&[fixed]).err(),
        Some(TransferPlanError::UnitCapOnFixedTransfer),
        "the single-source fixed transfer never takes a recipe-unit cap"
    );
}
