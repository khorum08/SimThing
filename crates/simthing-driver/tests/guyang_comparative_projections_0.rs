//! GUYANG-COMPARATIVE-PROJECTIONS-0 — scenario-neutral 5.8 proofs.
//!
//! Explicit, fail-closed birth is the 5.8 production posture (DA `5151136145`).
//! Default-derived install birth is 5.8b (out of scope). TP witness is void.

use simthing_core::{
    emit_on_threshold_registration_to_op, ColumnIndex, DimensionRegistry,
    EmitOnThresholdRegistration, PropertyAdmissionDisposition, SimProperty, SimPropertyId,
    SimThing, SimThingKind, SlotIndex, ThresholdDirection,
};
use simthing_driver::{
    admit_comparative_projections, comparative_event_kind, comparative_projection_cpu_oracle,
    compile_and_install, compile_comparative_bundle, neighbor_slots_from_grid,
    neighbor_slots_from_link_rows, ComparativeBandReadouts, ComparativeEmitterClass,
    ComparativeProjectionBands, ComparativeProjectionDisposition, ComparativeProjectionOutputs,
    ComparativeProjectionRequest, GuYangStallOutputs, Scenario, BAND_READOUT_COLUMN_COUNT,
    COMPARATIVE_DERIVED_COLUMN_COUNT,
};
use simthing_gpu::{
    execute_field_sweep_cpu_chain, execute_threshold_ops_cpu, FieldAdjacency, FieldSweepSession,
    GpuContext, LinkGraphNeighbor, SlotAllocator, GRID_N4_NSEW,
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

fn col(c: u32, n_dims: u32) -> ColumnIndex {
    ColumnIndex::try_from_admitted_authored(c, n_dims).unwrap()
}

/// Inline feedstock properties (any names). Identity for comparative is
/// `ComparativeEmitterClass` columns + authored_order — not namespace law.
fn register_feedstock(reg: &mut DimensionRegistry) -> [SimPropertyId; 5] {
    let mut ids = [SimPropertyId(0); 5];
    for (i, (ns, name)) in [
        ("feed", "e0"),
        ("feed", "e1"),
        ("feed", "d"),
        ("feed", "u"),
        ("feed", "c"),
    ]
    .iter()
    .enumerate()
    {
        let mut p = SimProperty::simple(ns, name, 1);
        p.admission_disposition = PropertyAdmissionDisposition::Anchored;
        ids[i] = reg.register(p);
    }
    ids
}

fn emitters_from_ids(
    reg: &DimensionRegistry,
    ids: &[SimPropertyId; 5],
) -> Vec<ComparativeEmitterClass> {
    vec![
        ComparativeEmitterClass {
            authored_order: 0,
            class_id: 10.0,
            value_col: ColumnIndex::from_gpu_round_trip(reg.column_range(ids[0]).start as u32),
        },
        ComparativeEmitterClass {
            authored_order: 1,
            class_id: 20.0,
            value_col: ColumnIndex::from_gpu_round_trip(reg.column_range(ids[1]).start as u32),
        },
    ]
}

fn triad_cols(
    reg: &DimensionRegistry,
    ids: &[SimPropertyId; 5],
) -> (ColumnIndex, ColumnIndex, ColumnIndex) {
    (
        ColumnIndex::from_gpu_round_trip(reg.column_range(ids[2]).start as u32),
        ColumnIndex::from_gpu_round_trip(reg.column_range(ids[3]).start as u32),
        ColumnIndex::from_gpu_round_trip(reg.column_range(ids[4]).start as u32),
    )
}

fn admit_grid(
    reg: &mut DimensionRegistry,
    width: u32,
    height: u32,
) -> simthing_driver::ComparativeProjectionAdmission {
    let ids = register_feedstock(reg);
    let gather = ColumnIndex::from_gpu_round_trip(0);
    let adj = FieldAdjacency::grid_n4(width, height, GRID_N4_NSEW, gather).expect("grid");
    let neighbors = neighbor_slots_from_grid(&adj).expect("neighbors");
    let (d, u, c) = triad_cols(reg, &ids);
    let emitters = emitters_from_ids(reg, &ids);
    admit_comparative_projections(
        reg,
        adj,
        neighbors,
        emitters,
        d,
        u,
        c,
        ComparativeProjectionBands::default(),
        None,
    )
    .expect("admit_comparative (explicit columns + admitted adjacency)")
}

#[test]
fn grid_and_link_graph_cpu_oracle_and_gpu_parity() {
    let mut reg = DimensionRegistry::new();
    let width = 6u32;
    let height = 4u32;
    let admission = admit_grid(&mut reg, width, height);
    let n_dims = reg.total_columns as u32;
    let slots = width * height;
    let outs = admission.outputs;
    let bands_r = admission.band_readouts;
    let stall = admission.stall_outputs.stall_col;
    let bands = ComparativeProjectionBands::default();

    // Recover feedstock columns by fixed registration order in admit_grid.
    // Feedstock was registered first in admit_grid (ids 0..4).
    let e0 = reg.column_range(SimPropertyId(0)).start;
    let e1 = reg.column_range(SimPropertyId(1)).start;
    let d = reg.column_range(SimPropertyId(2)).start;
    let u = reg.column_range(SimPropertyId(3)).start;
    let c = reg.column_range(SimPropertyId(4)).start;

    let mut values = vec![0.0f32; (slots * n_dims) as usize];
    let mid = width / 2;
    let mid_y = height / 2;
    for y in 0..height {
        for x in 0..width {
            let b = (y * width + x) as usize * n_dims as usize;
            if x < mid {
                values[b + e0] = 0.9;
                values[b + e1] = 0.2;
            } else if x > mid {
                values[b + e0] = 0.2;
                values[b + e1] = 0.9;
            } else {
                values[b + e0] = 0.55;
                values[b + e1] = 0.55;
            }
            values[b + d] = if x == mid && y == mid_y { 1.0 } else { 12.0 };
            values[b + u] = if x < mid {
                1.0
            } else if x > mid {
                0.0
            } else {
                0.5
            };
            values[b + c] = 0.5;
        }
    }

    let chain =
        execute_field_sweep_cpu_chain(&values, &admission.bundle.registrations).expect("chain");
    let neighbors = neighbor_slots_from_grid(
        &FieldAdjacency::grid_n4(
            width,
            height,
            GRID_N4_NSEW,
            ColumnIndex::from_gpu_round_trip(0),
        )
        .unwrap(),
    )
    .unwrap();
    let emitters = vec![
        ComparativeEmitterClass {
            authored_order: 0,
            class_id: 10.0,
            value_col: ColumnIndex::from_gpu_round_trip(e0 as u32),
        },
        ComparativeEmitterClass {
            authored_order: 1,
            class_id: 20.0,
            value_col: ColumnIndex::from_gpu_round_trip(e1 as u32),
        },
    ];
    let oracle = comparative_projection_cpu_oracle(
        &chain,
        slots,
        n_dims,
        &emitters,
        outs,
        bands_r,
        ColumnIndex::from_gpu_round_trip(d as u32),
        stall,
        bands,
        &neighbors,
    );
    for col_i in [
        outs.dominance_col.raw(),
        outs.margin_col.raw(),
        outs.contest_col.raw(),
        bands_r.border_col.raw(),
        bands_r.chokepoint_col.raw(),
        stall.raw(),
    ] {
        assert!(
            bits_equal(
                &column(&oracle, n_dims as usize, col_i),
                &column(&chain, n_dims as usize, col_i)
            ),
            "grid oracle parity col {col_i}"
        );
    }
    assert!(column(&chain, n_dims as usize, bands_r.border_col.raw())
        .iter()
        .any(|&b| b >= 0.5));

    // Grid GPU
    if let Some(ctx) = gpu_context() {
        let mut session = FieldSweepSession::new(&ctx, &admission.bundle.registrations[0]).unwrap();
        session.upload_values(&ctx, &values).unwrap();
        session
            .dispatch_chain(&ctx, &admission.bundle.registrations, 1)
            .unwrap();
        let gpu = session.readback(&ctx).unwrap();
        assert!(bits_equal(
            &column(&chain, n_dims as usize, outs.dominance_col.raw()),
            &column(&gpu, n_dims as usize, outs.dominance_col.raw())
        ));
        let info = ctx.adapter.get_info();
        eprintln!(
            "GUYANG-COMPARATIVE-PROJECTIONS grid adapter={} backend={:?}",
            info.name, info.backend
        );
    }

    // LinkGraph CPU oracle + GPU (Remand 4 item 5)
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
    let mut reg_l = DimensionRegistry::new();
    let ids_l = register_feedstock(&mut reg_l);
    let link_adj =
        FieldAdjacency::link_graph(4, link_rows.clone(), ColumnIndex::from_gpu_round_trip(0))
            .unwrap();
    let link_neighbors = neighbor_slots_from_link_rows(&link_rows);
    let (dl, ul, cl) = triad_cols(&reg_l, &ids_l);
    let emitters_l = emitters_from_ids(&reg_l, &ids_l);
    let e0l = reg_l.column_range(ids_l[0]).start;
    let e1l = reg_l.column_range(ids_l[1]).start;
    let d_slot = reg_l.column_range(ids_l[2]).start;
    let u_slot = reg_l.column_range(ids_l[3]).start;
    let c_slot = reg_l.column_range(ids_l[4]).start;
    let adm_l = admit_comparative_projections(
        &mut reg_l,
        link_adj,
        link_neighbors.clone(),
        emitters_l.clone(),
        dl,
        ul,
        cl,
        ComparativeProjectionBands::default(),
        None,
    )
    .expect("link admit");
    let n_dims_l = reg_l.total_columns as u32;
    let mut vals_l = vec![0.0f32; (4 * n_dims_l) as usize];
    for s in 0..4usize {
        let b = s * n_dims_l as usize;
        if s < 2 {
            vals_l[b + e0l] = 0.9;
            vals_l[b + e1l] = 0.2;
        } else {
            vals_l[b + e0l] = 0.2;
            vals_l[b + e1l] = 0.9;
        }
        vals_l[b + d_slot] = 12.0;
        vals_l[b + u_slot] = if s < 2 { 1.0 } else { 0.0 };
        vals_l[b + c_slot] = 0.5;
    }
    let chain_l =
        execute_field_sweep_cpu_chain(&vals_l, &adm_l.bundle.registrations).expect("link chain");
    let oracle_l = comparative_projection_cpu_oracle(
        &chain_l,
        4,
        n_dims_l,
        &emitters_l,
        adm_l.outputs,
        adm_l.band_readouts,
        dl,
        adm_l.stall_outputs.stall_col,
        ComparativeProjectionBands::default(),
        &link_neighbors,
    );
    for col_i in [
        adm_l.outputs.dominance_col.raw(),
        adm_l.outputs.margin_col.raw(),
        adm_l.outputs.contest_col.raw(),
        adm_l.band_readouts.border_col.raw(),
        adm_l.band_readouts.chokepoint_col.raw(),
        adm_l.stall_outputs.stall_col.raw(),
    ] {
        assert!(
            bits_equal(
                &column(&oracle_l, n_dims_l as usize, col_i),
                &column(&chain_l, n_dims_l as usize, col_i)
            ),
            "link oracle parity col {col_i}"
        );
    }
    assert!(column(
        &chain_l,
        n_dims_l as usize,
        adm_l.band_readouts.border_col.raw()
    )
    .iter()
    .any(|&b| b >= 0.5));

    if let Some(ctx) = gpu_context() {
        let mut session = FieldSweepSession::new(&ctx, &adm_l.bundle.registrations[0]).unwrap();
        session.upload_values(&ctx, &vals_l).unwrap();
        session
            .dispatch_chain(&ctx, &adm_l.bundle.registrations, 1)
            .unwrap();
        let gpu = session.readback(&ctx).unwrap();
        for col_i in [
            adm_l.outputs.dominance_col.raw(),
            adm_l.outputs.margin_col.raw(),
            adm_l.outputs.contest_col.raw(),
            adm_l.band_readouts.border_col.raw(),
            adm_l.band_readouts.chokepoint_col.raw(),
            adm_l.stall_outputs.stall_col.raw(),
        ] {
            assert!(
                bits_equal(
                    &column(&chain_l, n_dims_l as usize, col_i),
                    &column(&gpu, n_dims_l as usize, col_i)
                ),
                "link GPU parity col {col_i}"
            );
        }
        let info = ctx.adapter.get_info();
        eprintln!(
            "GUYANG-COMPARATIVE-PROJECTIONS link adapter={} backend={:?}",
            info.name, info.backend
        );
    }
}
