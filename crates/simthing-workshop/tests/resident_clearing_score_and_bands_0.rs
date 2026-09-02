//! RESIDENT-CLEARING-SCORE-AND-BANDS-0 — direct continuous self-consumption,
//! row-11 recurrence, and physical-order invariance witnesses.

use simthing_core::{
    AccumulatorOp, ColumnIndex, CombineFn, CompiledAccumulatorOpPlan, ConsumeMode,
    DimensionRegistry, EmitOnThresholdBuffer, EmitOnThresholdRegistration, EmlExpressionRegistry,
    EmlNodeGpu, GateSpec, GenerationStamp, InputSpec, ScaleSpec, SimProperty, SimPropertyId,
    SimThingId, SlotIndex, SourceSpec, StructuralScalarChannel, ThresholdDirection, TransformOp,
};
use simthing_driver::need_binding::{
    bind_immediate_flow_pressure_to_allocator_weight, NeutralPressureBindingError,
};
use simthing_driver::{
    build_custom_layout, child_share_cpu, compile_action_band_gpu_execution_with_native_lanes,
    compile_gu_yang_n4_field_sweeps, plan_arena_allocation, plan_arena_allocation_with_pressure,
    produce_runtime_rf_next_generation_demands_for_tick, register_child_share_formula,
    run_arena_allocation_oracle, ActionBandActiveInstance, ActionBandNativeLaneAdmission,
    ArenaTreeLayout, FissionPolicy, GpuArenaDescriptor, GuYangN4FieldSweepSpec, HierarchyNode,
    NodeColumnRefs,
};
use simthing_gpu::{
    AccumulatorOpSession, ActionBandEmissionBindingGpu, EmlGpuProgramTable, FieldSweepSession,
    GpuContext, PackedAccumulatorUpload,
};
use simthing_sim::ThresholdRegistry;
use simthing_spec::{
    clear_constrained_claims_at_generation, ActionBandAdmissionBudgetSpec, ActionBandBandSpec,
    ActionBandChannelBindingSpec, ActionBandChannelKind, ActionBandConservedProgressBindingSpec,
    ActionBandConservedProgressBoundSourceSpec, ActionBandRequirementSemantics,
    ActionBandSessionBuildDoor, ActionBandSessionSpec, ActionBandTargetSpec,
    ActionBandTemplateSpec, AuthoredClearingProgram, ClearingRemainderAuthority, ConstrainedClaim,
    ConstrainedClearingResult, ConstrainedSupply, OwnerChannelScopeKey, OwnerRef, ResourceKey,
    RuntimeOwnerSiloDemandBucket, RuntimeRfDemandGenerationAuthority, RuntimeRfTickErrorKind,
    ScalarBoundDirection, ScopeId,
};
use std::collections::HashMap;

fn col(raw: usize) -> ColumnIndex {
    ColumnIndex::from_raw_for_oracle_or_rehearsal(raw)
}

fn cols() -> NodeColumnRefs {
    NodeColumnRefs {
        intrinsic_flow_col: col(0),
        allocated_flow_col: col(1),
        weight_col: col(2),
        intrinsic_flow_sum_col: col(3),
        weight_sum_col: col(4),
        balance_col: None,
        balance_governing_col: None,
        // Frozen ABI columns remain present but the 14.3 allocator plan must
        // neither read nor write them.
        propagated_intrinsic_flow_col: col(5),
        propagated_allocated_flow_col: col(6),
        propagated_weight_sum_col: col(7),
        hosted_simthing_id_col: col(8),
    }
}

fn d3_root(slots: [u32; 3], identity_base: u32) -> HierarchyNode {
    let cols = cols();
    let grandchild = HierarchyNode {
        participant_slot: SlotIndex::new(slots[2]),
        hosted_simthing_id: SimThingId::from_session_raw(identity_base + 2),
        depth: 2,
        children: vec![],
        cols,
    };
    let child = HierarchyNode {
        participant_slot: SlotIndex::new(slots[1]),
        hosted_simthing_id: SimThingId::from_session_raw(identity_base + 1),
        depth: 1,
        children: vec![grandchild],
        cols,
    };
    HierarchyNode {
        participant_slot: SlotIndex::new(slots[0]),
        hosted_simthing_id: SimThingId::from_session_raw(identity_base),
        depth: 0,
        children: vec![child],
        cols,
    }
}

fn layout_from_roots(roots: Vec<HierarchyNode>) -> ArenaTreeLayout {
    layout_from_roots_with_cols(roots, cols())
}

fn layout_from_roots_with_cols(roots: Vec<HierarchyNode>, cols: NodeColumnRefs) -> ArenaTreeLayout {
    build_custom_layout(
        0,
        &GpuArenaDescriptor {
            name: "resident-self-consumption".into(),
            flow_property_id: SimPropertyId(1),
            balance_property_id: None,
            max_participants: 64,
            max_coupling_fanout: 4,
            max_orderband_depth: 16,
            fission_policy: FissionPolicy::default(),
            participant_range: (0, 0),
            wildcard_max_expansion: None,
            reserved_orderband_depth: 0,
        },
        cols,
        roots,
    )
    .expect("D3 resident self-consumption layout")
}

fn d3_layout(slots: [u32; 3]) -> ArenaTreeLayout {
    layout_from_roots(vec![d3_root(slots, 1)])
}

fn wide_d3_layout() -> ArenaTreeLayout {
    layout_from_roots(
        (0..6)
            .map(|index| {
                let base = index * 10;
                d3_root([base, base + 1, base + 2], 1 + index * 3)
            })
            .collect(),
    )
}

fn pressure_cols() -> NodeColumnRefs {
    NodeColumnRefs {
        intrinsic_flow_col: col(0),
        allocated_flow_col: col(1),
        weight_col: col(2),
        intrinsic_flow_sum_col: col(3),
        weight_sum_col: col(4),
        balance_col: Some(col(5)),
        balance_governing_col: None,
        propagated_intrinsic_flow_col: col(6),
        propagated_allocated_flow_col: col(7),
        propagated_weight_sum_col: col(8),
        hosted_simthing_id_col: col(9),
    }
}

const GU_YANG_VALUE_COL: usize = 10;
const GU_YANG_CONDUCTANCE_COL: usize = 11;
const RF_CLAIM_COL: usize = 12;
const RF_RESULT_COL: usize = 13;
const PRESSURE_N_DIMS: u32 = 14;

fn asymmetric_pressure_layout() -> ArenaTreeLayout {
    let cols = pressure_cols();
    let leaf = |slot: u32, id: u32| HierarchyNode {
        participant_slot: SlotIndex::new(slot),
        hosted_simthing_id: SimThingId::from_session_raw(id),
        depth: 2,
        children: vec![],
        cols,
    };
    let branch = |slot: u32, id: u32, children: Vec<HierarchyNode>| HierarchyNode {
        participant_slot: SlotIndex::new(slot),
        hosted_simthing_id: SimThingId::from_session_raw(id),
        depth: 1,
        children,
        cols,
    };
    layout_from_roots_with_cols(
        vec![HierarchyNode {
            participant_slot: SlotIndex::new(0),
            hosted_simthing_id: SimThingId::from_session_raw(100),
            depth: 0,
            children: vec![
                branch(1, 101, vec![leaf(3, 103), leaf(4, 104)]),
                branch(2, 102, vec![leaf(5, 105), leaf(6, 106)]),
            ],
            cols,
        }],
        cols,
    )
}

fn assert_direct_recursive_shape(layout: &ArenaTreeLayout) {
    let plan = plan_arena_allocation(layout, &[], 256).expect("resident allocator plan");
    let cols = cols();
    let parent = &layout.participant_roots[0].children[0];
    let grandchild = &parent.children[0];
    let grandchild_disburse = plan
        .cpu_ops
        .iter()
        .find(|op| {
            op.targets
                == vec![(
                    grandchild.participant_slot,
                    grandchild.cols.allocated_flow_col,
                )]
                && matches!(op.combine, simthing_core::CombineFn::EvalEML { .. })
        })
        .expect("same allocator operation writes the next recursive level");
    let SourceSpec::ConjunctiveCrossing { inputs } = &grandchild_disburse.source else {
        panic!("child-share EvalEML must read its parent through the admitted input list");
    };
    assert_eq!(
        inputs
            .iter()
            .map(|input| (input.slot, input.col))
            .collect::<Vec<_>>(),
        vec![
            (parent.participant_slot, parent.cols.intrinsic_flow_sum_col),
            (parent.participant_slot, parent.cols.allocated_flow_col),
            (parent.participant_slot, parent.cols.weight_sum_col),
        ],
        "PARAM(1) is the exact level-N AllocatedFlow cell, not an intermediary",
    );

    let propagated = [
        cols.propagated_intrinsic_flow_col,
        cols.propagated_allocated_flow_col,
        cols.propagated_weight_sum_col,
    ];
    for op in &plan.cpu_ops {
        assert!(
            !op.targets
                .iter()
                .any(|(_, column)| propagated.contains(column)),
            "the resident plan must not materialize a propagated economic copy"
        );
        match &op.source {
            SourceSpec::SlotValue { col, .. } | SourceSpec::SlotRange { col, .. } => assert!(
                !propagated.contains(col),
                "the resident plan must not read a propagated economic copy"
            ),
            SourceSpec::ConjunctiveCrossing { inputs } => assert!(
                !inputs.iter().any(|input| propagated.contains(&input.col)),
                "the resident plan must not gather a propagated economic copy"
            ),
            SourceSpec::Constant(_) => {}
        }
    }
}

fn run_gpu_chain(layout: &ArenaTreeLayout, plant_propagated_copy_defect: bool) -> Option<Vec<u32>> {
    let ctx = match GpuContext::new_blocking() {
        Ok(ctx) => ctx,
        Err(_) if std::env::var_os("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH").is_some() => {
            panic!("resident clearing 14.3 requires the selected GPU adapter")
        }
        Err(_) => return None,
    };
    let mut plan = plan_arena_allocation(layout, &[], 256).expect("resident allocator plan");
    if plant_propagated_copy_defect {
        let parent = &layout.participant_roots[0].children[0];
        let grandchild = &parent.children[0];
        let op = plan
            .cpu_ops
            .iter_mut()
            .find(|op| {
                op.targets
                    == vec![(
                        grandchild.participant_slot,
                        grandchild.cols.allocated_flow_col,
                    )]
                    && matches!(op.combine, simthing_core::CombineFn::EvalEML { .. })
            })
            .expect("grandchild disbursement");
        let SourceSpec::ConjunctiveCrossing { inputs } = &mut op.source else {
            panic!("direct child-share input list");
        };
        inputs[1] = InputSpec {
            slot: parent.participant_slot,
            col: parent.cols.propagated_allocated_flow_col,
            unit_cost: 1.0,
        };
    }

    let mut registry = EmlExpressionRegistry::new();
    register_child_share_formula(&mut registry, cols()).expect("child-share formula registration");
    let upload_rows: Vec<_> = registry
        .formulas_for_gpu_upload()
        .map(|(id, meta, nodes)| {
            (
                id,
                meta.clone(),
                nodes
                    .iter()
                    .map(|node| EmlNodeGpu {
                        opcode: node.opcode,
                        flags: node.flags,
                        a: node.a,
                        b: node.b,
                        c: node.c,
                        d: node.d,
                    })
                    .collect(),
            )
        })
        .collect();
    let mut table = EmlGpuProgramTable::new(&ctx, 64, 4);
    let uploaded = table
        .upload_trees(&ctx, &upload_rows)
        .expect("resident child-share EML upload");
    for (tree_id, range_index) in uploaded {
        registry
            .mark_tree_uploaded(tree_id, range_index, table.generation)
            .expect("uploaded tree binding");
    }
    let upload = PackedAccumulatorUpload::from_ops_resolving_input_lists_with_eml(
        &plan.cpu_ops,
        Some(&registry),
    )
    .expect("packed resident input-list plan");

    let n_slots = layout
        .iter_all()
        .iter()
        .map(|node| node.participant_slot.raw())
        .max()
        .unwrap()
        + 1;
    let n_dims = 9u32;
    let mut values = vec![0.0f32; (n_slots * n_dims) as usize];
    let index =
        |slot: SlotIndex, column: ColumnIndex| (slot.raw() * n_dims + column.raw_u32()) as usize;
    let root = &layout.participant_roots[0];
    let child = &root.children[0];
    let grandchild = &child.children[0];
    values[index(root.participant_slot, root.cols.intrinsic_flow_col)] = 8.0;
    values[index(child.participant_slot, child.cols.weight_col)] = 1.0;
    values[index(grandchild.participant_slot, grandchild.cols.weight_col)] = 1.0;

    let mut session = AccumulatorOpSession::new_attached(&ctx, n_slots, n_dims, 4);
    session.upload_values(&ctx, &values);
    session.copy_values_to_previous(&ctx);
    session
        .upload_packed_ops(&ctx, &upload)
        .expect("resident ops upload");
    for band in 0..plan.n_bands {
        session
            .tick_with_eml(&ctx, band, Some(&table))
            .expect("resident OrderBand dispatch");
    }
    let observed = session.readback_full(&ctx).expect("proof readback");
    let direct_bits = vec![
        observed[index(child.participant_slot, child.cols.allocated_flow_col)].to_bits(),
        observed[index(
            grandchild.participant_slot,
            grandchild.cols.allocated_flow_col,
        )]
        .to_bits(),
    ];

    // The residual closure now consumes child AllocatedFlow directly too. Seal
    // the generic Sum scale on the same live session: no per-child propagated
    // negative cell may be materialized merely to subtract the child total.
    let residual = AccumulatorOp {
        source: SourceSpec::SlotRange {
            start: child.participant_slot,
            count: root.children.len() as u32,
            col: child.cols.allocated_flow_col,
        },
        combine: CombineFn::Sum,
        gate: GateSpec::OrderBand(0),
        scale: ScaleSpec::Constant(-1.0),
        consume: ConsumeMode::ResetTarget,
        targets: vec![(root.participant_slot, root.cols.weight_sum_col)],
    };
    let residual_upload =
        PackedAccumulatorUpload::from_ops(&[residual]).expect("scaled residual Sum upload");
    session
        .upload_packed_ops(&ctx, &residual_upload)
        .expect("scaled residual ops upload");
    session.tick(&ctx, 0).expect("scaled residual Sum dispatch");
    let residual_observed = session
        .readback_full(&ctx)
        .expect("residual proof readback");

    Some(vec![
        direct_bits[0],
        direct_bits[1],
        residual_observed[index(root.participant_slot, root.cols.weight_sum_col)].to_bits(),
    ])
}

#[test]
fn allocated_flow_is_direct_recursive_evaleml_input_and_gpu_self_consumes() {
    let compact = d3_layout([0, 1, 2]);
    let rebound = d3_layout([129, 65, 3]);
    let wide = wide_d3_layout();
    assert_direct_recursive_shape(&compact);
    assert_direct_recursive_shape(&rebound);
    assert_direct_recursive_shape(&wide);
    prove_direct_child_pressure_sums_once_and_neutral_f_or_p_drives_next_generation_share();

    let mut oracle_values = HashMap::from([
        ((SlotIndex::new(0), cols().intrinsic_flow_col), 8.0),
        ((SlotIndex::new(1), cols().weight_col), 1.0),
        ((SlotIndex::new(2), cols().weight_col), 1.0),
    ]);
    run_arena_allocation_oracle(&compact, &mut oracle_values, 1.0);
    assert_eq!(
        oracle_values[&(SlotIndex::new(1), cols().allocated_flow_col)].to_bits(),
        8.0f32.to_bits()
    );
    assert_eq!(
        oracle_values[&(SlotIndex::new(2), cols().allocated_flow_col)].to_bits(),
        8.0f32.to_bits(),
        "level-N AllocatedFlow must become level-N+1 allocator input"
    );

    let Some(compact_bits) = run_gpu_chain(&compact, false) else {
        eprintln!("SKIP: no GPU adapter for resident clearing 14.3 direct-chain witness");
        return;
    };
    let rebound_bits = run_gpu_chain(&rebound, false).expect("same selected adapter");
    let wide_bits = run_gpu_chain(&wide, false).expect("same selected adapter");
    assert_eq!(
        compact_bits,
        vec![8.0f32.to_bits(), 8.0f32.to_bits(), (-8.0f32).to_bits()]
    );
    assert_eq!(rebound_bits, compact_bits, "epoch row rebind is bit-exact");
    assert_eq!(
        wide_bits, compact_bits,
        "one-workgroup and multi-workgroup dispatch cardinalities are bit-exact"
    );
    assert_ne!(
        run_gpu_chain(&compact, true).expect("same selected adapter"),
        compact_bits,
        "planted propagated-copy intermediary must RED"
    );
}

fn pressure_registry() -> DimensionRegistry {
    let mut registry = DimensionRegistry::new();
    for raw in 0..4 {
        registry.register(SimProperty::simple(
            "resident-pressure",
            &format!("column-{raw}"),
            1,
        ));
    }
    assert!(registry.total_columns >= PRESSURE_N_DIMS as usize);
    registry
}

fn pressure_rf_plan() -> CompiledAccumulatorOpPlan {
    CompiledAccumulatorOpPlan {
        slot_count: 7,
        n_dims: PRESSURE_N_DIMS,
        input_channel: StructuralScalarChannel::new(RF_CLAIM_COL as u32),
        output_channel: StructuralScalarChannel::new(RF_RESULT_COL as u32),
        ops: vec![AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(0),
                col: col(RF_CLAIM_COL),
            },
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(SlotIndex::new(0), col(RF_RESULT_COL))],
        }],
    }
}

fn compiled_pressure_product(
    source: ActionBandConservedProgressBoundSourceSpec,
) -> (
    simthing_driver::CompiledActionBandConservedProgressBinding,
    Vec<ActionBandActiveInstance>,
) {
    let registry = pressure_registry();
    let threshold = EmitOnThresholdRegistration {
        slot: SlotIndex::new(3),
        col: col(GU_YANG_VALUE_COL),
        threshold: 0.0,
        direction: ThresholdDirection::Upward,
        event_kind: 14_300,
        buffer: EmitOnThresholdBuffer::Values,
    };
    let spec = ActionBandSessionSpec {
        budget: ActionBandAdmissionBudgetSpec {
            axis_channel_count: 1,
            dependency_binding_count: 0,
            storage_rows: 4,
            eml_program_count: 0,
            emission_binding_count: 1,
        },
        templates: vec![ActionBandTemplateSpec {
            id: "resident-pressure".into(),
            label: None,
            axis_channels: vec![ActionBandChannelBindingSpec {
                column: GU_YANG_VALUE_COL as u32,
                kind: ActionBandChannelKind::Primitive,
            }],
            target: ActionBandTargetSpec::ScalarBound {
                channel: GU_YANG_VALUE_COL as u32,
                bound: 0.0,
                direction: ScalarBoundDirection::AtLeast,
            },
            velocity: None,
            bands: vec![ActionBandBandSpec {
                threshold_registration_index: 0,
                eml_program: None,
                emission_binding_indices: vec![0],
            }],
            subordinate_template_ids: vec![],
            max_active_subordinates: 0,
            reserved_instance_rows: 4,
            requirement_semantics: ActionBandRequirementSemantics::Ordinary,
        }],
    };
    let conserved = [ActionBandConservedProgressBindingSpec {
        template_id: "resident-pressure".into(),
        band_index: 0,
        emission_binding_index: 0,
        bound_source: source,
    }];
    let eml = EmlExpressionRegistry::new();
    let mut door = ActionBandSessionBuildDoor::new();
    let frozen = door
        .admit_once_with_conserved_progress_at_session_build(
            &spec,
            &conserved,
            &registry,
            &eml,
            std::slice::from_ref(&threshold),
        )
        .expect("sealed conserved-pressure admission")
        .clone();
    let active = (3..=6)
        .map(|slot| {
            ActionBandActiveInstance::new(
                frozen.templates()[0].index(),
                SlotIndex::new(slot),
                [0.0; 4],
            )
        })
        .collect::<Vec<_>>();
    let rf_plan = pressure_rf_plan();
    let native = ActionBandNativeLaneAdmission::from_existing_surfaces(
        &registry,
        &[],
        std::slice::from_ref(&rf_plan),
        &[],
        &ThresholdRegistry::new(),
    );
    let compiled = compile_action_band_gpu_execution_with_native_lanes(
        &frozen,
        &eml,
        &[ActionBandEmissionBindingGpu::rf_claim(RF_CLAIM_COL as u32)],
        &active,
        &native,
    )
    .expect("typed ActionBand pressure compile");
    (compiled.conserved_progress_bindings()[0], active)
}

fn pressure_source_slots(op: &AccumulatorOp) -> Vec<SlotIndex> {
    match &op.source {
        SourceSpec::SlotRange { start, count, .. } => (0..*count)
            .map(|offset| SlotIndex::new(start.raw() + offset))
            .collect(),
        SourceSpec::ConjunctiveCrossing { inputs } => {
            inputs.iter().map(|input| input.slot).collect()
        }
        other => panic!("branch pressure must use RF range/input-list Sum, got {other:?}"),
    }
}

fn pressure_oracle(immediate_flow: bool) -> (f32, f32, f32, f32, f32) {
    let layout = asymmetric_pressure_layout();
    let cols = pressure_cols();
    let selected = if immediate_flow {
        [(3, 2.0), (4, 1.0), (5, 3.0), (6, 2.0)]
    } else {
        [(3, 6.0), (4, 3.0), (5, 3.0), (6, 2.0)]
    };
    let mut values = HashMap::from([((SlotIndex::new(0), cols.intrinsic_flow_col), 14.0)]);
    for (slot, pressure) in selected {
        values.insert((SlotIndex::new(slot), cols.weight_col), pressure);
    }
    run_arena_allocation_oracle(&layout, &mut values, 1.0);
    (
        values[&(SlotIndex::new(1), cols.weight_col)],
        values[&(SlotIndex::new(2), cols.weight_col)],
        values[&(SlotIndex::new(0), cols.weight_sum_col)],
        values[&(SlotIndex::new(1), cols.allocated_flow_col)],
        values[&(SlotIndex::new(2), cols.allocated_flow_col)],
    )
}

fn run_gpu_pressure_case(immediate_flow: bool) -> Option<Vec<u32>> {
    let ctx = match GpuContext::new_blocking() {
        Ok(ctx) => ctx,
        Err(_) if std::env::var_os("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH").is_some() => {
            panic!("resident clearing 14.3 requires the selected GPU adapter")
        }
        Err(_) => return None,
    };
    let layout = asymmetric_pressure_layout();
    let cols = pressure_cols();
    let (gu_yang_binding, active) =
        compiled_pressure_product(ActionBandConservedProgressBoundSourceSpec::GuYangRealized);
    let plan = if immediate_flow {
        plan_arena_allocation_with_pressure(
            &layout,
            &[],
            7,
            std::slice::from_ref(&gu_yang_binding),
            &active,
            GenerationStamp::new(10),
            GenerationStamp::new(11),
        )
        .expect("ordinary plan inserts born F for N+1")
    } else {
        plan_arena_allocation(&layout, &[], 7).expect("raw-P allocation plan")
    };
    let ops = plan.cpu_ops.clone();

    let mut registry = EmlExpressionRegistry::new();
    register_child_share_formula(&mut registry, cols).expect("child-share formula registration");
    let upload_rows: Vec<_> = registry
        .formulas_for_gpu_upload()
        .map(|(id, meta, nodes)| {
            (
                id,
                meta.clone(),
                nodes
                    .iter()
                    .map(|node| EmlNodeGpu {
                        opcode: node.opcode,
                        flags: node.flags,
                        a: node.a,
                        b: node.b,
                        c: node.c,
                        d: node.d,
                    })
                    .collect(),
            )
        })
        .collect();
    let mut table = EmlGpuProgramTable::new(&ctx, 64, 4);
    let uploaded = table
        .upload_trees(&ctx, &upload_rows)
        .expect("pressure child-share EML upload");
    for (tree_id, range_index) in uploaded {
        registry
            .mark_tree_uploaded(tree_id, range_index, table.generation)
            .expect("uploaded tree binding");
    }
    let upload =
        PackedAccumulatorUpload::from_ops_resolving_input_lists_with_eml(&ops, Some(&registry))
            .expect("packed pressure plan");

    let n_dims = PRESSURE_N_DIMS;
    let mut values = vec![0.0f32; 7 * n_dims as usize];
    let index = |slot: u32, column: ColumnIndex| (slot * n_dims + column.raw_u32()) as usize;
    values[index(0, cols.intrinsic_flow_col)] = 14.0;
    let source_pressures = if immediate_flow {
        let registrations = compile_gu_yang_n4_field_sweeps(GuYangN4FieldSweepSpec {
            width: 7,
            height: 1,
            n_dims,
            value_col: col(GU_YANG_VALUE_COL),
            conductance_col: col(GU_YANG_CONDUCTANCE_COL),
            saturation: 10.0,
            chi: 0.25,
            dt: 0.25,
        })
        .expect("canonical Gu-Yang field producer");
        let mut born_values = vec![0.0; 7 * n_dims as usize];
        for (slot, seed) in [(3, 6.0), (4, 3.0), (5, 3.0), (6, 2.0)] {
            born_values[index(slot, col(GU_YANG_VALUE_COL))] = seed;
        }
        let mut field =
            FieldSweepSession::new(&ctx, &registrations[0]).expect("canonical Gu-Yang session");
        field.upload_values(&ctx, &born_values).unwrap();
        field.dispatch_chain(&ctx, &registrations, 1).unwrap();
        let born = field.readback(&ctx).unwrap();
        assert_eq!(field.registration_dispatches(), 2);
        [3, 4, 5, 6].map(|slot| born[index(slot, col(GU_YANG_VALUE_COL))])
    } else {
        [6.0, 3.0, 3.0, 2.0]
    };
    for (offset, slot) in [3, 4, 5, 6].into_iter().enumerate() {
        let pressure = source_pressures[offset];
        if immediate_flow {
            values[index(slot, col(GU_YANG_VALUE_COL))] = pressure;
        } else {
            // These are the lawful leaf inputs. The only branch-level raw P
            // products are the plan's rows-2/8 direct-child Sum publications.
            values[index(slot, cols.weight_col)] = pressure;
        }
    }

    let mut session = AccumulatorOpSession::new_attached(&ctx, 7, n_dims, 128);
    session.upload_values(&ctx, &values);
    session.copy_values_to_previous(&ctx);
    session
        .upload_packed_ops(&ctx, &upload)
        .expect("pressure ops upload");
    for band in 0..plan.n_bands {
        session
            .tick_with_eml(&ctx, band, Some(&table))
            .expect("pressure OrderBand dispatch");
    }
    let observed = session
        .readback_full(&ctx)
        .expect("pressure proof readback");
    let observed_bits = [
        (1, cols.weight_col),
        (2, cols.weight_col),
        (0, cols.weight_sum_col),
        (1, cols.allocated_flow_col),
        (2, cols.allocated_flow_col),
    ]
    .into_iter()
    .map(|(slot, column)| observed[index(slot, column)].to_bits())
    .collect::<Vec<_>>();
    let branch_a = source_pressures[0] + source_pressures[1];
    let branch_b = source_pressures[2] + source_pressures[3];
    let total = branch_a + branch_b;
    let expected = [
        branch_a,
        branch_b,
        total,
        child_share_cpu(14.0, 0.0, branch_a, total),
        child_share_cpu(14.0, 0.0, branch_b, total),
    ]
    .map(f32::to_bits)
    .to_vec();
    assert_eq!(
        &observed_bits[..3],
        &expected[..3],
        "born pressure and both direct-child Sum products are bit-exact"
    );
    assert!(
        observed_bits[3..]
            .iter()
            .zip(&expected[3..])
            .all(|(gpu, cpu)| gpu.abs_diff(*cpu) <= 1),
        "real born Gu-Yang F / rows-2/8 raw P must traverse native weight and child share within the admitted GPU arithmetic ULP"
    );
    Some(observed_bits)
}

fn prove_direct_child_pressure_sums_once_and_neutral_f_or_p_drives_next_generation_share() {
    let layout = asymmetric_pressure_layout();
    let cols = pressure_cols();
    let plan = plan_arena_allocation(&layout, &[], 7).expect("pressure allocation plan");

    let mut seen_edges = HashMap::new();
    for parent in layout
        .iter_all()
        .into_iter()
        .filter(|node| !node.children.is_empty())
    {
        let targets = vec![
            (parent.participant_slot, parent.cols.weight_col),
            (parent.participant_slot, parent.cols.weight_sum_col),
        ];
        let op = plan
            .cpu_ops
            .iter()
            .find(|op| op.targets == targets)
            .expect("one branch-pressure writer per interior node");
        assert_eq!(
            op.combine,
            CombineFn::Sum,
            "pressure is additive, never max/tropical"
        );
        assert_eq!(op.consume, ConsumeMode::ResetTarget);
        let sources = pressure_source_slots(op);
        assert_eq!(
            sources,
            parent
                .children
                .iter()
                .map(|child| child.participant_slot)
                .collect::<Vec<_>>(),
            "the parent consumes direct children in logical hierarchy order",
        );
        for child in sources {
            *seen_edges
                .entry((parent.participant_slot, child))
                .or_insert(0usize) += 1;
        }
    }
    let expected_edges: Vec<_> = layout
        .iter_all()
        .into_iter()
        .flat_map(|parent| {
            parent
                .children
                .iter()
                .map(move |child| (parent.participant_slot, child.participant_slot))
        })
        .collect();
    assert_eq!(seen_edges.len(), expected_edges.len());
    assert!(
        expected_edges
            .iter()
            .all(|edge| seen_edges.get(edge) == Some(&1)),
        "every logical tree edge contributes exactly once",
    );
    let root_pressure_op = plan
        .cpu_ops
        .iter()
        .find(|op| {
            op.targets
                == vec![
                    (SlotIndex::new(0), cols.weight_col),
                    (SlotIndex::new(0), cols.weight_sum_col),
                ]
        })
        .expect("root pressure reduction");
    assert_eq!(
        pressure_source_slots(root_pressure_op),
        vec![SlotIndex::new(1), SlotIndex::new(2)],
        "root may not recount grandchildren",
    );

    let immediate = pressure_oracle(true);
    assert_eq!(immediate, (3.0, 5.0, 8.0, 5.25, 8.75));
    let entitlement = pressure_oracle(false);
    assert_eq!(entitlement, (9.0, 5.0, 14.0, 9.0, 5.0));
    assert!(
        immediate.4 > immediate.3,
        "immediate-flow compares born serviceable F: branch B 5 > branch A 3",
    );
    assert!(
        entitlement.3 > entitlement.4,
        "entitlement-first compares born raw P: branch A 9 > branch B 5",
    );

    let (gu_yang_binding, active) =
        compiled_pressure_product(ActionBandConservedProgressBoundSourceSpec::GuYangRealized);
    let pressure_plan = plan_arena_allocation_with_pressure(
        &layout,
        &[],
        7,
        std::slice::from_ref(&gu_yang_binding),
        &active,
        GenerationStamp::new(10),
        GenerationStamp::new(11),
    )
    .expect("ordinary allocator plan owns typed Gu-Yang insertion");
    let identity_ops = pressure_plan
        .cpu_ops
        .iter()
        .filter(|op| {
            matches!(
                op.source,
                SourceSpec::SlotValue { col: source_col, .. }
                    if source_col == col(GU_YANG_VALUE_COL)
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        identity_ops.len(),
        4,
        "all relevant admitted leaf products are inserted without caller-built ops"
    );
    assert!(identity_ops.iter().all(|op| {
        matches!(op.source, SourceSpec::SlotValue { .. })
            && op.combine == CombineFn::Identity
            && op.scale == ScaleSpec::Identity
            && op.targets.len() == 1
            && op.targets[0].1 == cols.weight_col
    }), "born F is copied by identity into existing AllocatorWeight; there is no private solver or score layer");
    let no_pressure_plan = plan_arena_allocation_with_pressure(
        &layout,
        &[],
        7,
        &[],
        &[],
        GenerationStamp::new(10),
        GenerationStamp::new(11),
    )
    .expect("empty typed-pressure set");
    assert_eq!(
        no_pressure_plan, plan,
        "no-pressure ordinary plan remains bit-exact"
    );
    assert!(
        no_pressure_plan.cpu_ops.iter().all(|op| !matches!(
            op.source,
            SourceSpec::SlotValue { col: source_col, .. }
                if source_col == col(GU_YANG_VALUE_COL)
        )),
        "entitlement-first has no ad hoc cell projection; raw P is the rows-2/8 Sum product"
    );

    let private_serviceability_recompute_mutant = entitlement;
    assert_ne!(
        private_serviceability_recompute_mutant, immediate,
        "replacing the born serviceable F cells with a private raw-P serviceability surrogate changes branch pressure and REDs",
    );

    assert!(
        matches!(
            plan_arena_allocation_with_pressure(
                &layout,
                &[],
                7,
                std::slice::from_ref(&gu_yang_binding),
                &active,
                GenerationStamp::new(10),
                GenerationStamp::new(10),
            ),
            Err(simthing_driver::AllocationPlanError::NeutralPressure(
                NeutralPressureBindingError::NotNextGeneration { .. }
            ))
        ),
        "same-generation pressure -> reweight -> re-clear must RED"
    );
    let (wrong_role, wrong_role_active) =
        compiled_pressure_product(ActionBandConservedProgressBoundSourceSpec::RfGrant);
    assert!(matches!(
        bind_immediate_flow_pressure_to_allocator_weight(
            wrong_role,
            wrong_role_active[0],
            wrong_role_active[0].slot(),
            cols.weight_col,
            GenerationStamp::new(10),
            GenerationStamp::new(11),
            0,
        ),
        Err(NeutralPressureBindingError::ImmediateFlowSourceNotGuYang)
    ), "a real compiled wrong-role product is typed-refused; a fabricated ResolvedFullCell cannot type-check at this door");

    let descendant_recount_mutant = entitlement.2 + 6.0 + 3.0 + 3.0 + 2.0;
    assert_ne!(
        descendant_recount_mutant, entitlement.2,
        "adding grandchildren beside their branch aggregates double-counts pressure and REDs",
    );
    let arbitrary_score_winner = SlotIndex::new(2);
    let pressure_share_winner = if entitlement.3 > entitlement.4 {
        SlotIndex::new(1)
    } else {
        SlotIndex::new(2)
    };
    assert_ne!(
        arbitrary_score_winner, pressure_share_winner,
        "arbitrary score-bit precedence cannot substitute for continuous pressure share",
    );

    let Some(immediate_gpu) = run_gpu_pressure_case(true) else {
        eprintln!("SKIP: no GPU adapter for row-2/3/8 pressure witness");
        return;
    };
    let entitlement_gpu = run_gpu_pressure_case(false).expect("same selected adapter");
    assert_eq!(
        entitlement_gpu,
        vec![
            9.0f32.to_bits(),
            5.0f32.to_bits(),
            14.0f32.to_bits(),
            9.0f32.to_bits(),
            5.0f32.to_bits(),
        ],
    );
    assert_ne!(
        immediate_gpu, entitlement_gpu,
        "live born Gu-Yang F may lawfully drift from raw-P entitlement pressure"
    );
}

fn scope(name: &str) -> OwnerChannelScopeKey {
    OwnerChannelScopeKey {
        owner_ref: OwnerRef::new("owner/7"),
        resource_key: ResourceKey::new("resource/food"),
        scope_id: ScopeId::new(name),
    }
}

fn demand(
    scope: OwnerChannelScopeKey,
    source: u32,
    requested: u32,
) -> RuntimeOwnerSiloDemandBucket {
    RuntimeOwnerSiloDemandBucket {
        owner_ref: scope.owner_ref,
        resource_key: scope.resource_key,
        scope_id: scope.scope_id,
        requested,
        priority: 1,
        source_simthing_id_raw: Some(source),
    }
}

fn claim(demand: &RuntimeOwnerSiloDemandBucket) -> ConstrainedClaim {
    ConstrainedClaim::from_runtime_demand(demand, 1.0).expect("ordinary runtime demand claim")
}

#[test]
fn unresolved_demand_recurs_once_at_n_plus_one_and_drains_without_authored_path() {
    let n = GenerationStamp::new(10);
    let key = scope("scope/a");
    let current_demand = demand(key.clone(), 41, 10);
    let current_supplies = [ConstrainedSupply {
        scope: key.clone(),
        available: 4,
    }];
    let current_claims = [claim(&current_demand)];
    let program = AuthoredClearingProgram::new(TransformOp::set(0.0));
    let clearing_authority = ClearingRemainderAuthority {
        granter: SimThingId::from_session_raw(7),
        generation: n,
    };
    let next_authored = demand(key.clone(), 41, 2);
    let production_authority = RuntimeRfDemandGenerationAuthority::new(clearing_authority);
    let (current, carried) = produce_runtime_rf_next_generation_demands_for_tick(
        &production_authority,
        &current_supplies,
        &current_claims,
        &program,
        vec![next_authored.clone()],
    )
    .expect("production-owned generation-N clear and neutral Current-to-Next carry");
    let grant = &current[0].grants[0];
    assert_eq!(
        (grant.requested, grant.granted, grant.unresolved),
        (10, 4, 6)
    );
    assert_eq!(carried.len(), 1);
    let carried = &carried[0];
    assert_eq!(carried.generation(), GenerationStamp::new(11));
    assert_eq!(carried.product().requested, 8, "d' + u exactly once");
    assert_eq!(current_demand.requested, 10, "generation N is unchanged");
    assert_eq!(grant.unresolved, 6, "generation N is not re-cleared");
    assert_eq!(
        carried.product().requested,
        next_authored.requested + grant.unresolved,
        "the parent sees the recurrent demand once"
    );

    let second_consumption = produce_runtime_rf_next_generation_demands_for_tick(
        &production_authority,
        &current_supplies,
        &current_claims,
        &program,
        vec![next_authored.clone()],
    )
    .expect_err("the production authority must refuse a second Current-to-Next mint");
    assert_eq!(
        second_consumption.kind,
        RuntimeRfTickErrorKind::DemandCurrentToNextAlreadyProduced,
        "double consumption must be typed-refused by the real production door"
    );

    let omission_authority = RuntimeRfDemandGenerationAuthority::new(clearing_authority);
    let omitted = produce_runtime_rf_next_generation_demands_for_tick(
        &omission_authority,
        &current_supplies,
        &current_claims,
        &program,
        Vec::new(),
    )
    .expect_err("a caller cannot opt out of an owned unresolved row");
    assert_eq!(
        omitted.kind,
        RuntimeRfTickErrorKind::DemandCurrentToNextRejected
    );
    assert!(omitted.message.contains("omitted an unresolved"));

    let next = clear_constrained_claims_at_generation(
        &[ConstrainedSupply {
            scope: key.clone(),
            available: 8,
        }],
        &[claim(carried.product())],
        &AuthoredClearingProgram::new(TransformOp::set(0.0)),
        ClearingRemainderAuthority {
            granter: SimThingId::from_session_raw(7),
            generation: carried.generation(),
        },
    )
    .expect("ordinary generation-N+1 clear");
    assert_eq!(next[0].grants[0].unresolved, 0, "new supply drains u");

    let zero_supplies = [ConstrainedSupply {
        scope: key,
        available: current_demand.requested,
    }];
    let zero_authority = RuntimeRfDemandGenerationAuthority::new(clearing_authority);
    let (zero_current, zero_control) = produce_runtime_rf_next_generation_demands_for_tick(
        &zero_authority,
        &zero_supplies,
        &current_claims,
        &program,
        vec![next_authored.clone()],
    )
    .expect("u=0 through the production Current-to-Next door");
    assert_eq!(zero_current[0].grants[0].unresolved, 0);
    assert_eq!(zero_control.len(), 1);
    assert_eq!(
        zero_control[0].product(),
        &next_authored,
        "u=0 through the same door leaves the established demand product bit-exact"
    );
}

fn canonical_snapshot(
    results: &[ConstrainedClearingResult],
) -> Vec<(String, Vec<(u32, u32, u32)>)> {
    results
        .iter()
        .map(|result| {
            (
                result.scope.scope_id.as_str().to_owned(),
                result
                    .grants
                    .iter()
                    .map(|grant| {
                        (
                            grant.source_simthing_id.raw(),
                            grant.granted,
                            grant.clearing_score.to_bits(),
                        )
                    })
                    .collect(),
            )
        })
        .collect()
}

fn scheduled_order(rows: &[(usize, u32, u32)], workgroup: u32, partitions: u32) -> Vec<usize> {
    let mut scheduled = rows.to_vec();
    scheduled.sort_by_key(|&(logical, physical_row, segment)| {
        (
            (physical_row / workgroup) % partitions,
            segment,
            physical_row % workgroup,
            logical,
        )
    });
    scheduled.into_iter().map(|row| row.0).collect()
}

#[test]
fn canonical_clearing_ignores_physical_order_and_atomic_arrival_mutant_reds() {
    let a = scope("scope/a");
    let b = scope("scope/b");
    let demands = vec![
        demand(a.clone(), 11, 1),
        demand(a.clone(), 12, 1),
        demand(b.clone(), 21, 2),
        demand(b.clone(), 22, 2),
    ];
    let program = AuthoredClearingProgram::new(TransformOp::set(1.0));
    let authority = ClearingRemainderAuthority {
        granter: SimThingId::from_session_raw(5),
        generation: GenerationStamp::new(17),
    };
    let supplies = vec![
        ConstrainedSupply {
            scope: a,
            available: 1,
        },
        ConstrainedSupply {
            scope: b,
            available: 2,
        },
    ];
    let variants = [
        (vec![(0, 0, 0), (1, 1, 0), (2, 2, 1), (3, 3, 1)], 32, 1),
        (vec![(0, 130, 1), (1, 2, 0), (2, 67, 3), (3, 33, 2)], 64, 3),
        (vec![(0, 7, 3), (1, 129, 2), (2, 1, 1), (3, 65, 0)], 16, 4),
    ];
    let mut snapshots = Vec::new();
    let mut arrivals = Vec::new();
    for (index, (rows, workgroup, partitions)) in variants.iter().enumerate() {
        let order = scheduled_order(rows, *workgroup, *partitions);
        arrivals.push(order.clone());
        let claims: Vec<_> = order.iter().map(|&row| claim(&demands[row])).collect();
        let mut local_supplies = supplies.clone();
        if index % 2 == 1 {
            local_supplies.reverse();
        }
        let result =
            clear_constrained_claims_at_generation(&local_supplies, &claims, &program, authority)
                .expect("canonical clear under physical perturbation");
        snapshots.push(canonical_snapshot(&result));
    }
    assert!(snapshots.windows(2).all(|pair| pair[0] == pair[1]));
    for (_, grants) in &snapshots[0] {
        assert!(
            grants.windows(2).all(|pair| pair[0].0 < pair[1].0),
            "hard total order is claimant logical identity within an equality band"
        );
        assert!(
            grants.iter().all(|grant| grant.2 == 1.0f32.to_bits()),
            "EML score bits and equality-band segmentation match the CPU oracle"
        );
    }

    // Planted atomic-append resolver: the first arrival in the exact tie wins.
    // The two physical schedules choose different logical claimants, proving
    // that physical arrival cannot be an admitted tie authority.
    let planted_winner = |order: &[usize]| demands[order[0]].source_simthing_id_raw.unwrap();
    assert_ne!(
        planted_winner(&arrivals[0]),
        planted_winner(&arrivals[1]),
        "planted atomic append-order tie resolver must RED"
    );
}
