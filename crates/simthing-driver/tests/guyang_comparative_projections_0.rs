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

/// 5.8: ordinary install does not invent comparative birth (5.8b owns that).
#[test]
fn install_does_not_invent_topology_or_string_default_birth() {
    let n_slots = 16u32; // perfect square is not topology authority
    let mut registry = DimensionRegistry::new();
    let _ = registry.register(SimProperty::simple("_seed", "pad", 0));
    for (ns, name) in [
        ("feed", "e0"),
        ("feed", "e1"),
        ("feed", "d"),
        ("feed", "u"),
        ("feed", "c"),
    ] {
        let mut p = SimProperty::simple(ns, name, 1);
        p.admission_disposition = PropertyAdmissionDisposition::Anchored;
        registry.register(p);
    }
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let scenario = Scenario {
        name: "no_invent".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 0.0,
        n_slots,
        registry: registry.clone(),
        root: root.clone(),
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::new(),
    };
    let game_mode = GameModeSpec {
        id: "no_invent".into(),
        display_name: "no invent".into(),
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
        region_fields: Vec::new(),
        mapping_execution_profile: Default::default(),
    };
    let state = compile_and_install(
        &game_mode,
        &scenario,
        &mut registry,
        &mut root,
        &mut allocator,
    )
    .expect("install");
    assert!(
        state.comparative_projection.is_none(),
        "5.8 install leaves comparative birth unset; default birth is 5.8b"
    );
}

#[test]
fn explicit_admit_dispositions_and_fixed_comparative_column_count() {
    let mut reg = DimensionRegistry::new();
    let ids = register_feedstock(&mut reg);
    let gather = ColumnIndex::from_gpu_round_trip(0);
    let adj = FieldAdjacency::grid_n4(4, 4, GRID_N4_NSEW, gather).unwrap();
    let neighbors = neighbor_slots_from_grid(&adj).unwrap();
    let (d, u, c) = triad_cols(&reg, &ids);

    let emitters_all = emitters_from_ids(&reg, &ids);
    let one = admit_comparative_projections(
        &mut reg,
        adj.clone(),
        neighbors.clone(),
        emitters_all.iter().copied().take(1).collect(),
        d,
        u,
        c,
        ComparativeProjectionBands::default(),
        None,
    )
    .expect("one");
    assert!(matches!(
        one.disposition,
        ComparativeProjectionDisposition::InsufficientEmitters { emitter_count: 1 }
    ));

    let two = admit_comparative_projections(
        &mut reg,
        adj,
        neighbors,
        emitters_all,
        d,
        u,
        c,
        ComparativeProjectionBands::default(),
        None,
    )
    .expect("two");
    assert_eq!(
        two.disposition,
        ComparativeProjectionDisposition::Born {
            emitter_count: 2,
            comparative_column_count: COMPARATIVE_DERIVED_COLUMN_COUNT,
        }
    );
    assert_eq!(
        two.bundle.comparative_column_count,
        COMPARATIVE_DERIVED_COLUMN_COUNT
    );
    assert_eq!(COMPARATIVE_DERIVED_COLUMN_COUNT, 3);
    assert_eq!(BAND_READOUT_COLUMN_COUNT, 2);
    assert!(!two.bundle.registrations.is_empty());
}

#[test]
fn authored_order_tie_break_invariant_under_registration_vector_reversal() {
    let width = 2u32;
    let height = 1u32;
    let n_dims = 20u32;
    let gather = col(0, n_dims);
    let adj = FieldAdjacency::grid_n4(width, height, GRID_N4_NSEW, gather).unwrap();
    let neighbors = neighbor_slots_from_grid(&adj).unwrap();

    let mk = |emitters: Vec<ComparativeEmitterClass>| {
        compile_comparative_bundle(ComparativeProjectionRequest {
            adjacency: adj.clone(),
            neighbor_slots: neighbors.clone(),
            n_dims,
            emitters,
            outputs: ComparativeProjectionOutputs {
                dominance_col: col(10, n_dims),
                margin_col: col(11, n_dims),
                contest_col: col(12, n_dims),
            },
            band_readouts: ComparativeBandReadouts {
                border_col: col(13, n_dims),
                chokepoint_col: col(14, n_dims),
            },
            palma_d_col: col(2, n_dims),
            guyang_value_col: col(3, n_dims),
            guyang_conductance_col: col(4, n_dims),
            stall_outputs: GuYangStallOutputs {
                net_flux_col: col(5, n_dims),
                gross_flux_col: col(6, n_dims),
                stall_col: col(7, n_dims),
            },
            bands: ComparativeProjectionBands::default(),
            authored_opt_out_reason: None,
        })
        .expect("bundle")
    };

    let e_ab = vec![
        ComparativeEmitterClass {
            authored_order: 0,
            class_id: 10.0,
            value_col: col(0, n_dims),
        },
        ComparativeEmitterClass {
            authored_order: 1,
            class_id: 20.0,
            value_col: col(1, n_dims),
        },
    ];
    let e_ba = vec![e_ab[1], e_ab[0]];

    let mut values = vec![0.0f32; (width * height * n_dims) as usize];
    values[0] = 1.0;
    values[1] = 1.0;
    values[2] = 9.0;
    values[3] = 0.5;
    values[4] = 0.5;
    values[n_dims as usize] = 0.1;
    values[n_dims as usize + 1] = 0.9;
    values[n_dims as usize + 2] = 9.0;
    values[n_dims as usize + 3] = 0.5;
    values[n_dims as usize + 4] = 0.5;

    let out_ab = execute_field_sweep_cpu_chain(&values, &mk(e_ab).registrations).unwrap();
    let out_ba = execute_field_sweep_cpu_chain(&values, &mk(e_ba).registrations).unwrap();
    assert_eq!(
        out_ab[10], out_ba[10],
        "reversing registration/vector iteration must NOT change authored tie-break winner"
    );
    assert_eq!(out_ab[10], 10.0, "authored_order 0 wins exact tie");

    let e_wrong = vec![
        ComparativeEmitterClass {
            authored_order: 1,
            class_id: 10.0,
            value_col: col(0, n_dims),
        },
        ComparativeEmitterClass {
            authored_order: 0,
            class_id: 20.0,
            value_col: col(1, n_dims),
        },
    ];
    let out_wrong = execute_field_sweep_cpu_chain(&values, &mk(e_wrong).registrations).unwrap();
    assert_eq!(
        out_wrong[10], 20.0,
        "planted wrong authored_order flips winner"
    );
    assert_ne!(out_ab[10], out_wrong[10]);
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

/// Threshold-band compatibility of the plan (ordinary EmitOnThreshold).
/// Session-wide threshold install wiring is 5.8b/session surface work, not 5.8.
#[test]
fn front_formed_hardened_and_chokepoint_threshold_plan_compatible() {
    let mut reg = DimensionRegistry::new();
    let width = 6u32;
    let height = 4u32;
    let admission = admit_grid(&mut reg, width, height);
    let n_dims = reg.total_columns as u32;
    let slots = width * height;
    let e0 = reg.column_range(SimPropertyId(0)).start;
    let e1 = reg.column_range(SimPropertyId(1)).start;
    let d = reg.column_range(SimPropertyId(2)).start;
    let u = reg.column_range(SimPropertyId(3)).start;
    let c = reg.column_range(SimPropertyId(4)).start;
    let mid = width / 2;
    let mid_y = height / 2;
    let mut values = vec![0.0f32; (slots * n_dims) as usize];
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
    let projected =
        execute_field_sweep_cpu_chain(&values, &admission.bundle.registrations).expect("proj");
    let contest_vals = column(
        &projected,
        n_dims as usize,
        admission.outputs.contest_col.raw(),
    );
    assert!(
        contest_vals
            .iter()
            .any(|&v| v > ComparativeProjectionBands::default().front_harden_contest),
        "contest must exceed harden band under opposing Gu-Yang flux"
    );

    let plan = &admission.threshold_plan;
    let mut regs = Vec::new();
    for slot in 0..slots {
        regs.push(EmitOnThresholdRegistration {
            slot: SlotIndex::new(slot),
            col: plan.front_formed.0,
            threshold: plan.front_formed.1,
            direction: ThresholdDirection::Upward,
            event_kind: plan.front_formed.2,
            buffer: Default::default(),
        });
        regs.push(EmitOnThresholdRegistration {
            slot: SlotIndex::new(slot),
            col: plan.front_hardened.0,
            threshold: plan.front_hardened.1,
            direction: ThresholdDirection::Upward,
            event_kind: plan.front_hardened.2,
            buffer: Default::default(),
        });
    }
    let mid_slot = mid_y * width + mid;
    regs.push(EmitOnThresholdRegistration {
        slot: SlotIndex::new(mid_slot),
        col: plan.chokepoint_emerged.0,
        threshold: plan.chokepoint_emerged.1,
        direction: ThresholdDirection::Upward,
        event_kind: plan.chokepoint_emerged.2,
        buffer: Default::default(),
    });
    let ops: Vec<_> = regs
        .iter()
        .map(emit_on_threshold_registration_to_op)
        .collect();
    let kinds: Vec<_> = regs.iter().map(|r| r.event_kind).collect();
    let mut cur = projected.clone();
    let emissions = execute_threshold_ops_cpu(&values, &mut cur, &ops, n_dims, 0).expect("thresh");
    let formed = emissions
        .iter()
        .filter(|e| kinds[e.reg_idx() as usize] == comparative_event_kind::FRONT_FORMED)
        .count();
    let hardened = emissions
        .iter()
        .filter(|e| kinds[e.reg_idx() as usize] == comparative_event_kind::FRONT_HARDENED)
        .count();
    let choke = emissions
        .iter()
        .filter(|e| kinds[e.reg_idx() as usize] == comparative_event_kind::CHOKEPOINT_EMERGED)
        .count();
    assert!(formed > 0, "front-formed");
    assert!(hardened > 0, "front-hardened");
    assert_eq!(choke, 1, "chokepoint-emerged");

    let mut no_d = values.clone();
    for s in 0..slots as usize {
        no_d[s * n_dims as usize + d] = 20.0;
    }
    let out_d = execute_field_sweep_cpu_chain(&no_d, &admission.bundle.registrations).unwrap();
    assert!(column(
        &out_d,
        n_dims as usize,
        admission.band_readouts.chokepoint_col.raw()
    )
    .iter()
    .all(|&x| x < 0.5));

    let mut flat = values.clone();
    for s in 0..slots as usize {
        let b = s * n_dims as usize;
        flat[b + e0] = 0.9;
        flat[b + e1] = 0.1;
        flat[b + d] = 1.0;
    }
    let out_f = execute_field_sweep_cpu_chain(&flat, &admission.bundle.registrations).unwrap();
    assert!(column(
        &out_f,
        n_dims as usize,
        admission.band_readouts.border_col.raw()
    )
    .iter()
    .all(|&x| x < 0.5));
    assert!(column(
        &out_f,
        n_dims as usize,
        admission.band_readouts.chokepoint_col.raw()
    )
    .iter()
    .all(|&x| x < 0.5));
}
