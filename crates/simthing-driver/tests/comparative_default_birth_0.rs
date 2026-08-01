//! COMPARATIVE-DEFAULT-BIRTH-0 (5.8b) — scenario-neutral proofs.
//! Seam A: admitted FieldAdjacency/field-plan carried into SpecSessionState.
//! Seam B: roles from registration structure + authored order (no string grammar).

use simthing_core::{
    eml_opcode, ColumnIndex, DimensionRegistry, EmlNodeGpu, PropertyAdmissionDisposition,
    SimProperty, SimThing, SimThingKind, SlotIndex,
};
use simthing_driver::{
    admit_field_plan_binding, admit_field_plan_binding_with_neighbors,
    comparative_projection_cpu_oracle, compile_and_install, compile_and_install_with_field_plan,
    compile_gu_yang_n4_field_sweeps, compile_palma_n4_field_sweep,
    default_comparative_birth_from_field_plan, derive_comparative_inputs_from_field_plan,
    neighbor_slots_from_link_rows, admit_comparative_projections, ComparativeEmitterClass,
    ComparativeProjectionBands, ComparativeProjectionDisposition, GuYangN4FieldSweepSpec,
    PalmaN4FieldSweepSpec, Scenario, COMPARATIVE_DERIVED_COLUMN_COUNT,
};
use simthing_gpu::{
    apply_field_sweep_registration, encode_column, field_param, execute_field_sweep_cpu_chain,
    FieldAdjacency, FieldLawProof, FieldSweepOutput, FieldSweepRegistration,
    FieldSweepRegistrationRequest, FieldSweepSession, GpuContext, LinkGraphNeighbor, SlotAllocator,
    GRID_N4_NSEW, GRID_N4_WENS,
};
use simthing_spec::{GameModeSpec, SpecVersion};
use std::collections::HashMap;

fn bits_equal(a: &[f32], b: &[f32]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.to_bits() == y.to_bits())
}

fn column(values: &[f32], n_dims: usize, c: usize) -> Vec<f32> {
    values.chunks_exact(n_dims).map(|row| row[c]).collect()
}

fn gpu_context() -> Option<GpuContext> {
    match GpuContext::new_blocking() {
        Ok(c) => Some(c),
        Err(_) if std::env::var_os("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH").is_some() => {
            panic!("GPU required")
        }
        Err(_) => None,
    }
}

fn node(opcode: u32, flags: u32, a: u32, b: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode,
        flags,
        a,
        b,
        c: 0,
        d: 0,
    }
}
fn literal(v: f32) -> EmlNodeGpu {
    node(eml_opcode::LITERAL_F32, 0, v.to_bits(), 0)
}
fn ret() -> EmlNodeGpu {
    node(eml_opcode::RETURN_TOP, 0, 0, 0)
}
fn target(col: ColumnIndex) -> EmlNodeGpu {
    node(eml_opcode::TARGET_VALUE, 0, encode_column(col), 0)
}
fn neighbor(col: ColumnIndex) -> EmlNodeGpu {
    node(eml_opcode::NEIGHBOR_VALUE, 0, encode_column(col), 0)
}
fn param(i: u32) -> EmlNodeGpu {
    node(eml_opcode::PARAM, 0, i, 0)
}
fn binary(op: u32) -> EmlNodeGpu {
    node(op, 0, 0, 0)
}

/// Residual emitter: matrix output, non-conservative, fold has neither MIN nor MUL.
fn emitter_reg(
    adjacency: &FieldAdjacency,
    n_dims: u32,
    value_col: ColumnIndex,
) -> FieldSweepRegistration {
    let order = adjacency.apply_canonical_order_proof();
    apply_field_sweep_registration(FieldSweepRegistrationRequest {
        adjacency: adjacency.clone(),
        n_dims,
        output: FieldSweepOutput::Matrix(value_col),
        map_program: vec![literal(0.0), ret()],
        fold_program: vec![param(field_param::ACCUMULATOR), ret()],
        identity_bits: 0.0f32.to_bits(),
        post_program: vec![target(value_col), ret()],
        field_law_proof: Some(FieldLawProof::apply_non_conservative()),
        transient_read_proof: None,
        canonical_order_proof: Some(order),
        dt: 1.0,
    })
    .expect("emitter")
}

fn palma_reg(
    adjacency: &FieldAdjacency,
    n_dims: u32,
    d_col: ColumnIndex,
    w_col: ColumnIndex,
) -> FieldSweepRegistration {
    let order = adjacency.apply_canonical_order_proof();
    apply_field_sweep_registration(FieldSweepRegistrationRequest {
        adjacency: adjacency.clone(),
        n_dims,
        output: FieldSweepOutput::Matrix(d_col),
        map_program: vec![neighbor(d_col), ret()],
        fold_program: vec![
            param(field_param::ACCUMULATOR),
            param(field_param::MAPPED),
            binary(eml_opcode::MIN),
            ret(),
        ],
        identity_bits: 1.0e30f32.to_bits(),
        post_program: vec![
            param(field_param::TARGET_SLOT),
            literal(0.0),
            binary(eml_opcode::CMP_EQ),
            literal(0.0),
            target(w_col),
            param(field_param::FOLDED),
            binary(eml_opcode::ADD),
            node(eml_opcode::SELECT, 0, 0, 0),
            ret(),
        ],
        field_law_proof: Some(FieldLawProof::apply_non_conservative()),
        transient_read_proof: None,
        canonical_order_proof: Some(order),
        dt: 1.0,
    })
    .expect("palma")
}

fn guyang_regs(
    adjacency: &FieldAdjacency,
    n_dims: u32,
    value_col: ColumnIndex,
    conductance_col: ColumnIndex,
) -> [FieldSweepRegistration; 2] {
    let order = adjacency.apply_canonical_order_proof();
    let conductance = apply_field_sweep_registration(FieldSweepRegistrationRequest {
        adjacency: adjacency.clone(),
        n_dims,
        output: FieldSweepOutput::Matrix(conductance_col),
        map_program: vec![literal(1.0), ret()],
        fold_program: vec![
            param(field_param::ACCUMULATOR),
            param(field_param::MAPPED),
            binary(eml_opcode::MUL),
            ret(),
        ],
        identity_bits: 1.0f32.to_bits(),
        post_program: vec![param(field_param::FOLDED), ret()],
        field_law_proof: Some(FieldLawProof::apply_non_conservative()),
        transient_read_proof: None,
        canonical_order_proof: Some(order),
        dt: 1.0,
    })
    .expect("C");
    let symmetry = adjacency.apply_undirected_symmetry_certificate().expect("sym");
    let chi = adjacency
        .apply_conductance_certificate(vec![1.0; adjacency.slots() as usize], 8.0)
        .expect("chi");
    let flux = apply_field_sweep_registration(FieldSweepRegistrationRequest {
        adjacency: adjacency.clone(),
        n_dims,
        output: FieldSweepOutput::Matrix(value_col),
        map_program: vec![literal(0.0), ret()],
        fold_program: vec![
            param(field_param::ACCUMULATOR),
            param(field_param::MAPPED),
            binary(eml_opcode::ADD),
            ret(),
        ],
        identity_bits: 0.0f32.to_bits(),
        post_program: vec![
            target(value_col),
            param(field_param::FOLDED),
            binary(eml_opcode::ADD),
            ret(),
        ],
        field_law_proof: Some(FieldLawProof::apply_conservative(symmetry, chi)),
        transient_read_proof: None,
        canonical_order_proof: Some(order),
        dt: 1.0,
    })
    .expect("U");
    [conductance, flux]
}

fn register_feedstock(reg: &mut DimensionRegistry) -> [ColumnIndex; 6] {
    let mut cols = [ColumnIndex::from_gpu_round_trip(0); 6];
    for (i, name) in ["e0", "e1", "d", "w", "u", "c"].iter().enumerate() {
        let mut p = SimProperty::simple("field", name, 1);
        p.admission_disposition = PropertyAdmissionDisposition::Anchored;
        let id = reg.register(p);
        cols[i] = ColumnIndex::from_gpu_round_trip(reg.column_range(id).start as u32);
    }
    cols
}

fn empty_game_mode() -> GameModeSpec {
    GameModeSpec {
        id: "cdb0".into(),
        display_name: "comparative default birth".into(),
        description: String::new(),
        spec_version: SpecVersion::default(),
        metadata: Default::default(),
        domain_packs: Vec::new(),
        properties: Vec::new(),
        overlays: Vec::new(),
        order_weight_classes: Vec::new(),
        capability_trees: Vec::new(),
        events: Vec::new(),
        resource_flow: None,
        resource_economy: None,
        resource_flow_execution_profile: Default::default(),
        region_fields: Vec::new(),
        mapping_execution_profile: Default::default(),
    }
}

fn empty_scenario(n_slots: u32, registry: DimensionRegistry) -> Scenario {
    Scenario {
        name: "cdb0".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 0.0,
        n_slots,
        registry,
        root: SimThing::new(SimThingKind::World, 0),
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::new(),
    }
}

/// All registrations share one NSEW FieldAdjacency (gather e0).
fn build_grid_plan(
    reg: &mut DimensionRegistry,
    width: u32,
    height: u32,
) -> (simthing_driver::AdmittedFieldPlanBinding, [ColumnIndex; 6]) {
    let cols = register_feedstock(reg);
    let [e0, e1, d, w, u, c] = cols;
    let n_dims = reg.total_columns as u32;
    let adj = FieldAdjacency::grid_n4(width, height, GRID_N4_NSEW, e0).expect("grid");
    let g = guyang_regs(&adj, n_dims, u, c);
    let regs = vec![
        emitter_reg(&adj, n_dims, e0),
        emitter_reg(&adj, n_dims, e1),
        palma_reg(&adj, n_dims, d, w),
        g[0].clone(),
        g[1].clone(),
    ];
    let plan = admit_field_plan_binding(adj, regs).expect("bind");
    (plan, cols)
}

#[test]
fn install_carries_admitted_field_plan_and_default_births() {
    let mut registry = DimensionRegistry::new();
    let (plan, _) = build_grid_plan(&mut registry, 4, 4);
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let scenario = empty_scenario(16, registry.clone());
    let state = compile_and_install_with_field_plan(
        &empty_game_mode(),
        &scenario,
        &mut registry,
        &mut root,
        &mut allocator,
        Some(plan),
    )
    .expect("install");
    assert!(state.admitted_field_plan.is_some());
    let adm = state.comparative_projection.expect("birth");
    assert_eq!(
        adm.disposition,
        ComparativeProjectionDisposition::Born {
            emitter_count: 2,
            comparative_column_count: COMPARATIVE_DERIVED_COLUMN_COUNT,
        }
    );
}

#[test]
fn default_matches_explicit_5_8_on_dual_emitter_front() {
    let width = 6u32;
    let height = 4u32;
    let mut reg_d = DimensionRegistry::new();
    let (plan, cols) = build_grid_plan(&mut reg_d, width, height);
    let [e0, e1, d, _w, u, c] = cols;
    let inputs = derive_comparative_inputs_from_field_plan(&plan).expect("derive");
    assert_eq!(inputs.emitters.len(), 2);

    let default = default_comparative_birth_from_field_plan(
        &mut reg_d,
        &plan,
        ComparativeProjectionBands::default(),
        None,
    )
    .expect("default");

    let mut reg_e = DimensionRegistry::new();
    let cols_e = register_feedstock(&mut reg_e);
    let [e0e, e1e, de, _we, ue, ce] = cols_e;
    let explicit = admit_comparative_projections(
        &mut reg_e,
        plan.adjacency().clone(),
        plan.neighbor_slots().to_vec(),
        vec![
            ComparativeEmitterClass {
                authored_order: 0,
                class_id: inputs.emitters[0].class_id,
                value_col: e0e,
            },
            ComparativeEmitterClass {
                authored_order: 1,
                class_id: inputs.emitters[1].class_id,
                value_col: e1e,
            },
        ],
        de,
        ue,
        ce,
        ComparativeProjectionBands::default(),
        None,
    )
    .expect("explicit");

    let n_d = reg_d.total_columns as u32;
    let n_e = reg_e.total_columns as u32;
    let slots = width * height;
    let mut vd = vec![0.0f32; (slots * n_d) as usize];
    let mut ve = vec![0.0f32; (slots * n_e) as usize];
    let mid = width / 2;
    for y in 0..height {
        for x in 0..width {
            let bd = (y * width + x) as usize * n_d as usize;
            let be = (y * width + x) as usize * n_e as usize;
            let (v0, v1) = if x < mid {
                (0.9, 0.2)
            } else if x > mid {
                (0.2, 0.9)
            } else {
                (0.55, 0.55)
            };
            let dval = if x == mid { 1.0 } else { 12.0 };
            let uval = if x < mid {
                1.0
            } else if x > mid {
                0.0
            } else {
                0.5
            };
            vd[bd + e0.raw()] = v0;
            vd[bd + e1.raw()] = v1;
            vd[bd + d.raw()] = dval;
            vd[bd + u.raw()] = uval;
            vd[bd + c.raw()] = 0.5;
            ve[be + e0e.raw()] = v0;
            ve[be + e1e.raw()] = v1;
            ve[be + de.raw()] = dval;
            ve[be + ue.raw()] = uval;
            ve[be + ce.raw()] = 0.5;
        }
    }
    let out_d = execute_field_sweep_cpu_chain(&vd, &default.bundle.registrations).unwrap();
    let out_e = execute_field_sweep_cpu_chain(&ve, &explicit.bundle.registrations).unwrap();
    for (cd, ce) in [
        (
            default.outputs.dominance_col.raw(),
            explicit.outputs.dominance_col.raw(),
        ),
        (
            default.outputs.margin_col.raw(),
            explicit.outputs.margin_col.raw(),
        ),
        (
            default.outputs.contest_col.raw(),
            explicit.outputs.contest_col.raw(),
        ),
        (
            default.band_readouts.border_col.raw(),
            explicit.band_readouts.border_col.raw(),
        ),
    ] {
        assert!(
            bits_equal(
                &column(&out_d, n_d as usize, cd),
                &column(&out_e, n_e as usize, ce)
            ),
            "default/explicit mismatch col {cd}/{ce}"
        );
    }
}

#[test]
fn planted_topology_substitute_rejected() {
    let mut reg = DimensionRegistry::new();
    let (plan, cols) = build_grid_plan(&mut reg, 4, 4);
    let [e0, ..] = cols;
    let wrong = FieldAdjacency::grid_n4(4, 4, GRID_N4_WENS, e0).expect("wens");
    let err = admit_field_plan_binding(wrong, plan.registrations().to_vec())
        .expect_err("topology substitute");
    assert!(matches!(
        err,
        simthing_driver::FieldPlanBindingError::AdjacencyIdentityMismatch { .. }
    ));
}

#[test]
fn one_emitter_insufficient() {
    let mut reg = DimensionRegistry::new();
    let cols = register_feedstock(&mut reg);
    let [e0, _e1, d, w, u, c] = cols;
    let n_dims = reg.total_columns as u32;
    let adj = FieldAdjacency::grid_n4(4, 4, GRID_N4_NSEW, e0).unwrap();
    let g = guyang_regs(&adj, n_dims, u, c);
    let regs = vec![
        emitter_reg(&adj, n_dims, e0),
        palma_reg(&adj, n_dims, d, w),
        g[0].clone(),
        g[1].clone(),
    ];
    let plan = admit_field_plan_binding(adj, regs).unwrap();
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let scenario = empty_scenario(16, reg.clone());
    let state = compile_and_install_with_field_plan(
        &empty_game_mode(),
        &scenario,
        &mut reg,
        &mut root,
        &mut allocator,
        Some(plan),
    )
    .unwrap();
    let adm = state.comparative_projection.unwrap();
    assert!(matches!(
        adm.disposition,
        ComparativeProjectionDisposition::InsufficientEmitters { emitter_count: 1 }
    ));
}

#[test]
fn install_without_field_plan_no_invent() {
    let mut registry = DimensionRegistry::new();
    let _ = registry.register(SimProperty::simple("_seed", "pad", 0));
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let scenario = empty_scenario(16, registry.clone());
    let state = compile_and_install(
        &empty_game_mode(),
        &scenario,
        &mut registry,
        &mut root,
        &mut allocator,
    )
    .unwrap();
    assert!(state.admitted_field_plan.is_none());
    assert!(state.comparative_projection.is_none());
}

#[test]
fn link_graph_default_birth_oracle_and_gpu() {
    let mut reg = DimensionRegistry::new();
    let cols = register_feedstock(&mut reg);
    let [e0, e1, d, w, u, c] = cols;
    let n_dims = reg.total_columns as u32;
    let link_rows = {
        let mut rows = vec![Vec::new(); 4];
        for (a, b) in [(0u32, 1), (1, 2), (2, 3)] {
            rows[a as usize].push(LinkGraphNeighbor {
                slot: SlotIndex::new(b),
                weight: 1.0,
            });
            rows[b as usize].push(LinkGraphNeighbor {
                slot: SlotIndex::new(a),
                weight: 1.0,
            });
        }
        for r in &mut rows {
            r.sort_by_key(|n| n.slot.raw());
        }
        rows
    };
    let adj = FieldAdjacency::link_graph(4, link_rows.clone(), e0).unwrap();
    let neighbors = neighbor_slots_from_link_rows(&link_rows);
    let g = guyang_regs(&adj, n_dims, u, c);
    let regs = vec![
        emitter_reg(&adj, n_dims, e0),
        emitter_reg(&adj, n_dims, e1),
        palma_reg(&adj, n_dims, d, w),
        g[0].clone(),
        g[1].clone(),
    ];
    let plan = admit_field_plan_binding_with_neighbors(adj, regs, neighbors.clone()).unwrap();
    let default = default_comparative_birth_from_field_plan(
        &mut reg,
        &plan,
        ComparativeProjectionBands::default(),
        None,
    )
    .unwrap();
    assert!(matches!(
        default.disposition,
        ComparativeProjectionDisposition::Born {
            emitter_count: 2,
            ..
        }
    ));
    let n_dims = reg.total_columns as u32;
    let mut values = vec![0.0f32; (4 * n_dims) as usize];
    for s in 0..4usize {
        let b = s * n_dims as usize;
        if s < 2 {
            values[b + e0.raw()] = 0.9;
            values[b + e1.raw()] = 0.2;
        } else {
            values[b + e0.raw()] = 0.2;
            values[b + e1.raw()] = 0.9;
        }
        values[b + d.raw()] = 12.0;
        values[b + u.raw()] = 0.5;
        values[b + c.raw()] = 0.5;
    }
    let chain = execute_field_sweep_cpu_chain(&values, &default.bundle.registrations).unwrap();
    let emitters = derive_comparative_inputs_from_field_plan(&plan)
        .unwrap()
        .emitters;
    let oracle = comparative_projection_cpu_oracle(
        &chain,
        4,
        n_dims,
        &emitters,
        default.outputs,
        default.band_readouts,
        d,
        default.stall_outputs.stall_col,
        ComparativeProjectionBands::default(),
        &neighbors,
    );
    assert!(bits_equal(
        &column(&oracle, n_dims as usize, default.outputs.dominance_col.raw()),
        &column(&chain, n_dims as usize, default.outputs.dominance_col.raw())
    ));
    if let Some(ctx) = gpu_context() {
        let mut session = FieldSweepSession::new(&ctx, &default.bundle.registrations[0]).unwrap();
        session.upload_values(&ctx, &values).unwrap();
        session
            .dispatch_chain(&ctx, &default.bundle.registrations, 1)
            .unwrap();
        let gpu = session.readback(&ctx).unwrap();
        assert!(bits_equal(
            &column(&chain, n_dims as usize, default.outputs.dominance_col.raw()),
            &column(&gpu, n_dims as usize, default.outputs.dominance_col.raw())
        ));
        eprintln!(
            "COMPARATIVE-DEFAULT-BIRTH link adapter={} backend={:?}",
            ctx.adapter.get_info().name,
            ctx.adapter.get_info().backend
        );
    }
}

// Silence unused official compilers kept as regression anchors for grid helpers.
#[allow(dead_code)]
fn _anchor_official_compilers() {
    let _ = (
        compile_palma_n4_field_sweep as fn(PalmaN4FieldSweepSpec) -> _,
        compile_gu_yang_n4_field_sweeps as fn(GuYangN4FieldSweepSpec) -> _,
    );
}
