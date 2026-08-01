//! COMPARATIVE-DEFAULT-BIRTH-0 (5.8b) — DA `5153818317` / remand `5153845512`.
//! Ordinary install delivers field-plan product; roles from Matrix output();
//! LinkGraph neighbors captured at link_graph lowering site (5.8 seam).

use simthing_core::{
    eml_opcode, ColumnIndex, DimensionRegistry, EmlNodeGpu, PropertyAdmissionDisposition,
    SimProperty, SimThing, SimThingKind, SlotIndex,
};
use simthing_driver::{
    admit_comparative_projections, admit_field_plan_report, comparative_projection_cpu_oracle,
    compile_and_install, neighbor_slots_from_grid,
    neighbor_slots_from_link_rows, ComparativeEmitterClass, ComparativeProjectionBands,
    ComparativeProjectionDisposition, Scenario, COMPARATIVE_DERIVED_COLUMN_COUNT,
};
use simthing_gpu::{
    apply_field_sweep_registration, encode_column, execute_field_sweep_cpu_chain, field_param,
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
fn param(i: u32) -> EmlNodeGpu {
    node(eml_opcode::PARAM, 0, i, 0)
}

fn matrix_reg(
    adjacency: &FieldAdjacency,
    n_dims: u32,
    out: ColumnIndex,
    law: FieldLawProof,
) -> FieldSweepRegistration {
    let order = adjacency.apply_canonical_order_proof();
    apply_field_sweep_registration(FieldSweepRegistrationRequest {
        adjacency: adjacency.clone(),
        n_dims,
        output: FieldSweepOutput::Matrix(out),
        map_program: vec![literal(0.0), ret()],
        fold_program: vec![param(field_param::ACCUMULATOR), ret()],
        identity_bits: 0.0f32.to_bits(),
        post_program: vec![target(out), ret()],
        field_law_proof: Some(law),
        transient_read_proof: None,
        canonical_order_proof: Some(order),
        dt: 1.0,
    })
    .expect("reg")
}

fn register_cols(reg: &mut DimensionRegistry, names: &[&str]) -> Vec<ColumnIndex> {
    names
        .iter()
        .map(|name| {
            let mut p = SimProperty::simple("field", name, 1);
            p.admission_disposition = PropertyAdmissionDisposition::Anchored;
            let id = reg.register(p);
            ColumnIndex::from_gpu_round_trip(reg.column_range(id).start as u32)
        })
        .collect()
}

fn empty_game_mode() -> GameModeSpec {
    GameModeSpec {
        id: "cdb0".into(),
        display_name: "cdb0".into(),
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

fn base_scenario(n_slots: u32, registry: DimensionRegistry) -> Scenario {
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
        field_plan_admission: None,
    }
}

fn guyang_pair(
    adjacency: &FieldAdjacency,
    n_dims: u32,
    u: ColumnIndex,
    c: ColumnIndex,
) -> (FieldSweepRegistration, FieldSweepRegistration) {
    let order = adjacency.apply_canonical_order_proof();
    let conductance = matrix_reg(adjacency, n_dims, c, FieldLawProof::apply_non_conservative());
    let symmetry = adjacency.apply_undirected_symmetry_certificate().unwrap();
    let chi = adjacency
        .apply_conductance_certificate(vec![1.0; adjacency.slots() as usize], 8.0)
        .unwrap();
    let flux = apply_field_sweep_registration(FieldSweepRegistrationRequest {
        adjacency: adjacency.clone(),
        n_dims,
        output: FieldSweepOutput::Matrix(u),
        map_program: vec![literal(0.0), ret()],
        fold_program: vec![param(field_param::ACCUMULATOR), ret()],
        identity_bits: 0.0f32.to_bits(),
        post_program: vec![target(u), ret()],
        field_law_proof: Some(FieldLawProof::apply_conservative(symmetry, chi)),
        transient_read_proof: None,
        canonical_order_proof: Some(order),
        dt: 1.0,
    })
    .unwrap();
    let _ = conductance;
    (
        matrix_reg(adjacency, n_dims, c, FieldLawProof::apply_non_conservative()),
        flux,
    )
}

/// Build grid field-plan report: emitters + palma + guyang C/U.
fn grid_plan(
    reg: &mut DimensionRegistry,
    width: u32,
    height: u32,
    emitter_names: &[&str],
) -> (
    simthing_driver::FieldPlanAdmissionReport,
    Vec<ColumnIndex>,
    ColumnIndex,
    ColumnIndex,
    ColumnIndex,
) {
    let mut names = emitter_names.to_vec();
    names.extend_from_slice(&["triad_d", "triad_w", "triad_u", "triad_c"]);
    let cols = register_cols(reg, &names);
    let n_emit = emitter_names.len();
    let e_cols: Vec<_> = cols[..n_emit].to_vec();
    let d = cols[n_emit];
    let _w = cols[n_emit + 1];
    let u = cols[n_emit + 2];
    let c = cols[n_emit + 3];
    let n_dims = reg.total_columns as u32;
    let adj = FieldAdjacency::grid_n4(width, height, GRID_N4_NSEW, e_cols[0]).unwrap();
    let neighbors = neighbor_slots_from_grid(&adj).unwrap();
    let emitters: Vec<_> = e_cols
        .iter()
        .map(|col| matrix_reg(&adj, n_dims, *col, FieldLawProof::apply_non_conservative()))
        .collect();
    let palma = matrix_reg(&adj, n_dims, d, FieldLawProof::apply_non_conservative());
    let (gc, gu) = guyang_pair(&adj, n_dims, u, c);
    let report = admit_field_plan_report(adj, neighbors, emitters, palma, gc, gu, None).unwrap();
    (report, e_cols, d, u, c)
}

fn install_with_plan(
    reg: &mut DimensionRegistry,
    n_slots: u32,
    report: simthing_driver::FieldPlanAdmissionReport,
) -> simthing_driver::SpecSessionState {
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let mut scenario = base_scenario(n_slots, reg.clone());
    scenario.field_plan_admission = Some(report);
    compile_and_install(
        &empty_game_mode(),
        &scenario,
        reg,
        &mut root,
        &mut allocator,
    )
    .expect("ordinary install")
}

#[test]
fn ordinary_install_default_births_two_emitters() {
    let mut reg = DimensionRegistry::new();
    let (report, _, _, _, _) = grid_plan(&mut reg, 4, 4, &["e0", "e1"]);
    let state = install_with_plan(&mut reg, 16, report);
    assert!(state.field_plan_admission.is_some());
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
fn emitter_counts_1_2_3_many_fixed_census() {
    for names in [
        vec!["e0"],
        vec!["e0", "e1"],
        vec!["e0", "e1", "e2"],
        vec!["a", "b", "c", "d", "e"],
    ] {
        let mut reg = DimensionRegistry::new();
        let (report, _, _, _, _) = grid_plan(&mut reg, 4, 4, &names);
        let state = install_with_plan(&mut reg, 16, report);
        let adm = state.comparative_projection.expect("adm");
        match names.len() {
            1 => assert!(matches!(
                adm.disposition,
                ComparativeProjectionDisposition::InsufficientEmitters { emitter_count: 1 }
            )),
            n => {
                assert_eq!(
                    adm.disposition,
                    ComparativeProjectionDisposition::Born {
                        emitter_count: n as u32,
                        comparative_column_count: COMPARATIVE_DERIVED_COLUMN_COUNT,
                    }
                );
                assert_eq!(adm.bundle.comparative_column_count, 3);
            }
        }
    }
}

#[test]
fn authored_opt_out_visible() {
    let mut reg = DimensionRegistry::new();
    let (mut report, _, _, _, _) = grid_plan(&mut reg, 4, 4, &["e0", "e1"]);
    // Re-mint with opt-out
    let report = admit_field_plan_report(
        report.adjacency().clone(),
        report.neighbor_slots().to_vec(),
        report.emitter_registrations().to_vec(),
        report.palma_d().clone(),
        report.guyang_conductance().clone(),
        report.guyang_value().clone(),
        Some("authored opt-out test"),
    )
    .unwrap();
    let state = install_with_plan(&mut reg, 16, report);
    let adm = state.comparative_projection.expect("adm");
    assert!(matches!(
        adm.disposition,
        ComparativeProjectionDisposition::AuthoredOptOut { .. }
    ));
}

#[test]
fn default_matches_explicit_grid_bit_for_bit() {
    let width = 6u32;
    let height = 4u32;
    let mut reg_d = DimensionRegistry::new();
    let (report, e_cols, d, u, c) = grid_plan(&mut reg_d, width, height, &["e0", "e1"]);
    let state = install_with_plan(&mut reg_d, width * height, report.clone());
    let default = state.comparative_projection.expect("default");

    let mut reg_e = DimensionRegistry::new();
    let cols_e = register_cols(&mut reg_e, &["e0", "e1", "d", "w", "u", "c"]);
    let emitters: Vec<_> = report
        .emitter_registrations()
        .iter()
        .enumerate()
        .map(|(i, reg)| {
            let FieldSweepOutput::Matrix(col) = reg.output() else {
                panic!("matrix");
            };
            ComparativeEmitterClass {
                authored_order: i as u32,
                class_id: col.raw_u32() as f32 + 1.0,
                value_col: cols_e[i],
            }
        })
        .collect();
    // Map triad to explicit registry cols (indices 2,4,5 after e0,e1,d,w,u,c)
    let explicit = admit_comparative_projections(
        &mut reg_e,
        report.adjacency().clone(),
        report.neighbor_slots().to_vec(),
        emitters,
        cols_e[2],
        cols_e[4],
        cols_e[5],
        ComparativeProjectionBands::default(),
        None,
    )
    .unwrap();

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
            vd[bd + e_cols[0].raw()] = v0;
            vd[bd + e_cols[1].raw()] = v1;
            vd[bd + d.raw()] = dval;
            vd[bd + u.raw()] = uval;
            vd[bd + c.raw()] = 0.5;
            ve[be + cols_e[0].raw()] = v0;
            ve[be + cols_e[1].raw()] = v1;
            ve[be + cols_e[2].raw()] = dval;
            ve[be + cols_e[4].raw()] = uval;
            ve[be + cols_e[5].raw()] = 0.5;
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
        (
            default.band_readouts.chokepoint_col.raw(),
            explicit.band_readouts.chokepoint_col.raw(),
        ),
    ] {
        assert!(
            bits_equal(
                &column(&out_d, n_d as usize, cd),
                &column(&out_e, n_e as usize, ce)
            ),
            "mismatch {cd}/{ce}"
        );
    }
}

#[test]
fn authored_order_invariant_under_registration_vector_reversal() {
    use simthing_driver::{
        compile_comparative_bundle, ComparativeBandReadouts, ComparativeProjectionOutputs,
        ComparativeProjectionRequest, GuYangStallOutputs,
    };
    let width = 2u32;
    let height = 1u32;
    let n_dims = 20u32;
    let col = |i: u32| ColumnIndex::try_from_admitted_authored(i, n_dims).unwrap();
    let adj = FieldAdjacency::grid_n4(width, height, GRID_N4_NSEW, col(0)).unwrap();
    let neighbors = neighbor_slots_from_grid(&adj).unwrap();
    let mk = |emitters: Vec<ComparativeEmitterClass>| {
        compile_comparative_bundle(ComparativeProjectionRequest {
            adjacency: adj.clone(),
            neighbor_slots: neighbors.clone(),
            n_dims,
            emitters,
            outputs: ComparativeProjectionOutputs {
                dominance_col: col(10),
                margin_col: col(11),
                contest_col: col(12),
            },
            band_readouts: ComparativeBandReadouts {
                border_col: col(13),
                chokepoint_col: col(14),
            },
            palma_d_col: col(2),
            guyang_value_col: col(3),
            guyang_conductance_col: col(4),
            stall_outputs: GuYangStallOutputs {
                net_flux_col: col(5),
                gross_flux_col: col(6),
                stall_col: col(7),
            },
            bands: ComparativeProjectionBands::default(),
            authored_opt_out_reason: None,
        })
        .unwrap()
    };
    let e_ab = vec![
        ComparativeEmitterClass {
            authored_order: 0,
            class_id: 10.0,
            value_col: col(0),
        },
        ComparativeEmitterClass {
            authored_order: 1,
            class_id: 20.0,
            value_col: col(1),
        },
    ];
    let e_ba = vec![e_ab[1], e_ab[0]];
    let mut values = vec![0.0f32; (width * height * n_dims) as usize];
    values[0] = 1.0;
    values[1] = 1.0;
    values[2] = 9.0;
    values[3] = 0.5;
    values[4] = 0.5;
    let out_ab = execute_field_sweep_cpu_chain(&values, &mk(e_ab).registrations).unwrap();
    let out_ba = execute_field_sweep_cpu_chain(&values, &mk(e_ba).registrations).unwrap();
    assert_eq!(out_ab[10], out_ba[10], "authored order, not vec order");
    assert_eq!(out_ab[10], 10.0);
}

#[test]
fn planted_grid_topology_substitute_rejected() {
    let mut reg = DimensionRegistry::new();
    let (report, e_cols, d, u, c) = grid_plan(&mut reg, 4, 4, &["e0", "e1"]);
    let wrong = FieldAdjacency::grid_n4(4, 4, GRID_N4_WENS, e_cols[0]).unwrap();
    let neighbors = neighbor_slots_from_grid(&wrong).unwrap();
    let err = admit_field_plan_report(
        wrong,
        neighbors,
        report.emitter_registrations().to_vec(),
        report.palma_d().clone(),
        report.guyang_conductance().clone(),
        report.guyang_value().clone(),
        None,
    )
    .expect_err("adj mismatch");
    assert!(matches!(
        err,
        simthing_driver::FieldPlanAdmissionError::AdjacencyIdentityMismatch
    ));
    let _ = (d, u, c);
}

#[test]
fn planted_link_neighbor_substitute_rejected_by_length_and_identity() {
    let mut reg = DimensionRegistry::new();
    let cols = register_cols(&mut reg, &["e0", "e1", "d", "w", "u", "c"]);
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
    // S1: capture at link_graph construction site
    let adj = FieldAdjacency::link_graph(4, link_rows.clone(), cols[0]).unwrap();
    let neighbors = neighbor_slots_from_link_rows(&link_rows);
    let emitters = vec![
        matrix_reg(&adj, n_dims, cols[0], FieldLawProof::apply_non_conservative()),
        matrix_reg(&adj, n_dims, cols[1], FieldLawProof::apply_non_conservative()),
    ];
    let palma = matrix_reg(&adj, n_dims, cols[2], FieldLawProof::apply_non_conservative());
    let (gc, gu) = guyang_pair(&adj, n_dims, cols[4], cols[5]);
    let ok = admit_field_plan_report(
        adj.clone(),
        neighbors.clone(),
        emitters.clone(),
        palma.clone(),
        gc.clone(),
        gu.clone(),
        None,
    );
    assert!(ok.is_ok());

    // Wrong length → NeighborSlotsMismatch
    let mut short = neighbors.clone();
    short.pop();
    let err = admit_field_plan_report(
        adj.clone(),
        short,
        emitters.clone(),
        palma.clone(),
        gc.clone(),
        gu.clone(),
        None,
    )
    .expect_err("len");
    assert!(matches!(
        err,
        simthing_driver::FieldPlanAdmissionError::NeighborSlotsMismatch { .. }
    ));

    // Different adjacency identity with same-length fabricated rows → AdjacencyIdentityMismatch
    // (regs bind adj; substitute adj fails PartialEq against reg.adjacency())
    let other_rows = {
        let mut rows = vec![Vec::new(); 4];
        rows[0].push(LinkGraphNeighbor {
            slot: SlotIndex::new(2),
            weight: 1.0,
        });
        rows[2].push(LinkGraphNeighbor {
            slot: SlotIndex::new(0),
            weight: 1.0,
        });
        rows
    };
    let other_adj = FieldAdjacency::link_graph(4, other_rows.clone(), cols[0]).unwrap();
    let other_neighbors = neighbor_slots_from_link_rows(&other_rows);
    let err = admit_field_plan_report(
        other_adj,
        other_neighbors,
        emitters,
        palma,
        gc,
        gu,
        None,
    )
    .expect_err("identity");
    assert!(matches!(
        err,
        simthing_driver::FieldPlanAdmissionError::AdjacencyIdentityMismatch
    ));
}

#[test]
fn link_default_matches_explicit_and_gpu() {
    let mut reg = DimensionRegistry::new();
    let cols = register_cols(&mut reg, &["e0", "e1", "d", "w", "u", "c"]);
    let n_dims0 = reg.total_columns as u32;
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
    let adj = FieldAdjacency::link_graph(4, link_rows.clone(), cols[0]).unwrap();
    let neighbors = neighbor_slots_from_link_rows(&link_rows);
    let emitters = vec![
        matrix_reg(&adj, n_dims0, cols[0], FieldLawProof::apply_non_conservative()),
        matrix_reg(&adj, n_dims0, cols[1], FieldLawProof::apply_non_conservative()),
    ];
    let palma = matrix_reg(&adj, n_dims0, cols[2], FieldLawProof::apply_non_conservative());
    let (gc, gu) = guyang_pair(&adj, n_dims0, cols[4], cols[5]);
    let report = admit_field_plan_report(adj, neighbors.clone(), emitters, palma, gc, gu, None)
        .unwrap();
    let state = install_with_plan(&mut reg, 4, report.clone());
    let default = state.comparative_projection.expect("birth");
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
            values[b + cols[0].raw()] = 0.9;
            values[b + cols[1].raw()] = 0.2;
        } else {
            values[b + cols[0].raw()] = 0.2;
            values[b + cols[1].raw()] = 0.9;
        }
        values[b + cols[2].raw()] = 12.0;
        values[b + cols[4].raw()] = 0.5;
        values[b + cols[5].raw()] = 0.5;
    }
    let chain = execute_field_sweep_cpu_chain(&values, &default.bundle.registrations).unwrap();
    let (em, d, u, c) = simthing_driver::comparative_inputs_from_field_plan(&report).unwrap();
    let oracle = comparative_projection_cpu_oracle(
        &chain,
        4,
        n_dims,
        &em,
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
    let _ = (u, c);
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

#[test]
fn install_without_field_plan_no_invent() {
    let mut reg = DimensionRegistry::new();
    let _ = reg.register(SimProperty::simple("_", "pad", 0));
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let scenario = base_scenario(16, reg.clone());
    let state = compile_and_install(
        &empty_game_mode(),
        &scenario,
        &mut reg,
        &mut root,
        &mut allocator,
    )
    .unwrap();
    assert!(state.field_plan_admission.is_none());
    assert!(state.comparative_projection.is_none());
}

#[test]
fn adjacency_mismatch_on_emitter_reg_fails_closed() {
    let mut reg = DimensionRegistry::new();
    let (report, e_cols, _, _, _) = grid_plan(&mut reg, 4, 4, &["e0", "e1"]);
    let other = FieldAdjacency::grid_n4(4, 4, GRID_N4_NSEW, e_cols[0]).unwrap();
    // same shape offsets order NSEW same gather — might be equal PartialEq!
    // Use WENS for different identity
    let other = FieldAdjacency::grid_n4(4, 4, GRID_N4_WENS, e_cols[0]).unwrap();
    let n_dims = reg.total_columns as u32;
    let bad_emitter = matrix_reg(&other, n_dims, e_cols[0], FieldLawProof::apply_non_conservative());
    let err = admit_field_plan_report(
        report.adjacency().clone(),
        report.neighbor_slots().to_vec(),
        vec![bad_emitter, report.emitter_registrations()[1].clone()],
        report.palma_d().clone(),
        report.guyang_conductance().clone(),
        report.guyang_value().clone(),
        None,
    )
    .expect_err("mismatch");
    assert!(matches!(
        err,
        simthing_driver::FieldPlanAdmissionError::AdjacencyIdentityMismatch
    ));
}
