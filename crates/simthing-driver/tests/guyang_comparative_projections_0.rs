//! GUYANG-COMPARATIVE-PROJECTIONS-0 — Remand 2 (DA ruling 5150877754).
//! Driver consumer: winner-identity border, Gu-Yang stall, default-derived birth.

use simthing_core::{
    emit_on_threshold_registration_to_op, ColumnIndex, DimensionRegistry, EmitOnThresholdRegistration,
    PropertyAdmissionDisposition, SimProperty, SlotIndex, ThresholdDirection,
};
use simthing_driver::{
    admit_comparative_projections, comparative_event_kind, comparative_projection_cpu_oracle,
    compile_structural_n4_theater, derive_comparative_projections_at_admission, ComparativeEmitterClass,
    ComparativeProjectionBands, ComparativeProjectionDisposition, ComparativeProjectionOutputs,
    ComparativeProjectionRequest, GuYangStallOutputs, StructuralTheaterAdmission,
    COMPARATIVE_DERIVED_COLUMN_COUNT,
};
use simthing_gpu::{
    execute_field_sweep_cpu_chain, execute_threshold_ops_cpu, FieldAdjacency, FieldSweepSession,
    GpuContext, LinkGraphNeighbor, GRID_N4_NSEW,
};
use simthing_spec::{
    deserialize_scenario_authority, validate_scenario_links, validate_stead_mapping_consistency,
    MappingExecutionProfile,
};

const TERRAN_PIRATE_SKELETON: &str =
    include_str!("../../../scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json");

fn col(raw: u32, n_dims: u32) -> ColumnIndex {
    ColumnIndex::try_from_admitted_authored(raw, n_dims).expect("col")
}

fn bits_equal(left: &[f32], right: &[f32]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(a, b)| a.to_bits() == b.to_bits())
}

fn column(values: &[f32], n_dims: usize, c: usize) -> Vec<f32> {
    values.chunks_exact(n_dims).map(|row| row[c]).collect()
}

fn gpu_context() -> Option<GpuContext> {
    match GpuContext::new_blocking() {
        Ok(c) => Some(c),
        Err(_) if std::env::var_os("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH").is_some() => {
            panic!("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH set but no GPU adapter")
        }
        Err(_) => None,
    }
}

/// Layout:
/// 0,1 emitters | 2 palma_d | 3 guyang_u | 4 guyang_c |
/// 5 net | 6 gross | 7 stall | 8 dom | 9 margin | 10 contest | 11 border | 12 choke
const N_DIMS: u32 = 13;

fn outputs() -> ComparativeProjectionOutputs {
    ComparativeProjectionOutputs {
        dominance_col: col(8, N_DIMS),
        margin_col: col(9, N_DIMS),
        contest_col: col(10, N_DIMS),
        border_col: col(11, N_DIMS),
        chokepoint_col: col(12, N_DIMS),
    }
}

fn stall_outs() -> GuYangStallOutputs {
    GuYangStallOutputs {
        net_flux_col: col(5, N_DIMS),
        gross_flux_col: col(6, N_DIMS),
        stall_col: col(7, N_DIMS),
    }
}

fn two_emitters() -> Vec<ComparativeEmitterClass> {
    vec![
        ComparativeEmitterClass {
            class_id: 10.0,
            value_col: col(0, N_DIMS),
        },
        ComparativeEmitterClass {
            class_id: 20.0,
            value_col: col(1, N_DIMS),
        },
    ]
}

fn base_request(
    width: u32,
    height: u32,
    emitters: Vec<ComparativeEmitterClass>,
) -> ComparativeProjectionRequest {
    let adjacency =
        FieldAdjacency::grid_n4(width, height, GRID_N4_NSEW, col(0, N_DIMS)).expect("grid");
    ComparativeProjectionRequest {
        adjacency,
        n_dims: N_DIMS,
        emitters,
        outputs: outputs(),
        palma_d_col: col(2, N_DIMS),
        guyang_value_col: col(3, N_DIMS),
        guyang_conductance_col: col(4, N_DIMS),
        stall_outputs: stall_outs(),
        bands: ComparativeProjectionBands::default(),
        authored_opt_out_reason: None,
    }
}

/// Two-side front + Gu-Yang state with opposing flux across the mid column.
fn synthetic_front(width: u32, height: u32) -> Vec<f32> {
    let mut v = vec![0.0; (width * height * N_DIMS) as usize];
    let mid = width / 2;
    let mid_y = height / 2;
    for y in 0..height {
        for x in 0..width {
            let b = (y * width + x) as usize * N_DIMS as usize;
            let (left, right) = if x < mid {
                (0.9, 0.2)
            } else if x > mid {
                (0.2, 0.9)
            } else {
                (0.55, 0.55)
            };
            v[b] = left;
            v[b + 1] = right;
            v[b + 2] = if x == mid && y == mid_y { 1.0 } else { 12.0 };
            // Gu-Yang u: left high / right low → opposing fluxes at interface.
            v[b + 3] = if x < mid {
                1.0
            } else if x > mid {
                0.0
            } else {
                0.5
            };
            v[b + 4] = 0.5; // uniform conductance
        }
    }
    v
}

#[test]
fn default_derived_projection_admission_for_1_2_3_and_many_emitter_classes() {
    let width = 4u32;
    let height = 4u32;
    let one = admit_comparative_projections(base_request(
        width,
        height,
        vec![ComparativeEmitterClass {
            class_id: 1.0,
            value_col: col(0, N_DIMS),
        }],
    ))
    .expect("one");
    assert!(matches!(
        one.disposition,
        ComparativeProjectionDisposition::InsufficientEmitters { emitter_count: 1 }
    ));

    let mut opt = base_request(width, height, two_emitters());
    opt.authored_opt_out_reason = Some("domain_suppresses_fronts");
    assert_eq!(
        admit_comparative_projections(opt)
            .expect("opt")
            .disposition,
        ComparativeProjectionDisposition::AuthoredOptOut {
            reason: "domain_suppresses_fronts"
        }
    );

    for n in [2usize, 3, 8] {
        let n_dims = (n + 20) as u32;
        let emitters: Vec<_> = (0..n)
            .map(|i| ComparativeEmitterClass {
                class_id: (i as f32) + 1.0,
                value_col: col(i as u32, n_dims),
            })
            .collect();
        let adjacency =
            FieldAdjacency::grid_n4(width, height, GRID_N4_NSEW, col(0, n_dims)).expect("adj");
        let base = n as u32;
        let req = ComparativeProjectionRequest {
            adjacency,
            n_dims,
            emitters,
            outputs: ComparativeProjectionOutputs {
                dominance_col: col(base + 8, n_dims),
                margin_col: col(base + 9, n_dims),
                contest_col: col(base + 10, n_dims),
                border_col: col(base + 11, n_dims),
                chokepoint_col: col(base + 12, n_dims),
            },
            palma_d_col: col(base, n_dims),
            guyang_value_col: col(base + 1, n_dims),
            guyang_conductance_col: col(base + 2, n_dims),
            stall_outputs: GuYangStallOutputs {
                net_flux_col: col(base + 3, n_dims),
                gross_flux_col: col(base + 4, n_dims),
                stall_col: col(base + 5, n_dims),
            },
            bands: ComparativeProjectionBands::default(),
            authored_opt_out_reason: None,
        };
        let bundle = admit_comparative_projections(req).expect("many");
        assert_eq!(
            bundle.disposition,
            ComparativeProjectionDisposition::Born {
                emitter_count: n as u32,
                derived_column_count: COMPARATIVE_DERIVED_COLUMN_COUNT,
            }
        );
        assert_eq!(bundle.derived_column_count, COMPARATIVE_DERIVED_COLUMN_COUNT);
    }
}

#[test]
fn winner_identity_border_and_stall_contest_match_oracle_cpu_and_gpu() {
    let width = 8u32;
    let height = 6u32;
    let emitters = two_emitters();
    let req = base_request(width, height, emitters.clone());
    let outs = req.outputs;
    let bands = req.bands;
    let stall_col = req.stall_outputs.stall_col;
    let adjacency = req.adjacency.clone();
    let bundle = admit_comparative_projections(req).expect("admit");
    let values = synthetic_front(width, height);

    let after_chain =
        execute_field_sweep_cpu_chain(&values, &bundle.registrations).expect("chain");
    // Oracle expects stall already written; use chain output for stall input.
    let oracle = comparative_projection_cpu_oracle(
        &after_chain,
        width * height,
        N_DIMS,
        &emitters,
        outs,
        col(2, N_DIMS),
        stall_col,
        bands,
        &adjacency,
    );

    for (name, c) in [
        ("dominance", outs.dominance_col.raw()),
        ("margin", outs.margin_col.raw()),
        ("contest", outs.contest_col.raw()),
        ("border", outs.border_col.raw()),
        ("chokepoint", outs.chokepoint_col.raw()),
        ("stall", stall_col.raw()),
    ] {
        assert!(
            bits_equal(
                &column(&oracle, N_DIMS as usize, c),
                &column(&after_chain, N_DIMS as usize, c)
            ),
            "{name} oracle parity"
        );
    }

    // Winner identity changes at mid interface ⇒ non-empty border.
    let borders = column(&after_chain, N_DIMS as usize, outs.border_col.raw());
    assert!(
        borders.iter().any(|&b| b >= 0.5),
        "winner-identity border must fire at the front"
    );

    // Stall = gross - |net| ≥ 0; contest uses stall at both-strong/small-margin.
    let stalls = column(&after_chain, N_DIMS as usize, stall_col.raw());
    assert!(stalls.iter().all(|&s| s >= -1e-5));

    if let Some(ctx) = gpu_context() {
        let mut session =
            FieldSweepSession::new(&ctx, &bundle.registrations[0]).expect("session");
        session.upload_values(&ctx, &values).expect("upload");
        session
            .dispatch_chain(&ctx, &bundle.registrations, 1)
            .expect("dispatch");
        let gpu = session.readback(&ctx).expect("readback");
        for c in [
            outs.dominance_col.raw(),
            outs.margin_col.raw(),
            outs.contest_col.raw(),
            outs.border_col.raw(),
            outs.chokepoint_col.raw(),
            stall_col.raw(),
        ] {
            assert!(
                bits_equal(
                    &column(&after_chain, N_DIMS as usize, c),
                    &column(&gpu, N_DIMS as usize, c)
                ),
                "CPU/GPU col {c}"
            );
        }
        let info = ctx.adapter.get_info();
        eprintln!(
            "GUYANG-COMPARATIVE-PROJECTIONS adapter={} backend={:?}",
            info.name, info.backend
        );
    }
}

#[test]
fn argmax_tie_break_decides_border_identity_deterministically() {
    // Planted exact tie: both emitters equal; authored order must decide dominance
    // and therefore border with the neighbor that has a different ordered winner.
    let width = 2u32;
    let height = 1u32;
    let mut values = vec![0.0; (width * height * N_DIMS) as usize];
    // slot0: exact tie 1.0/1.0 → class 10 wins (authored first)
    values[0] = 1.0;
    values[1] = 1.0;
    // slot1: class 20 wins clearly
    values[N_DIMS as usize] = 0.1;
    values[N_DIMS as usize + 1] = 0.9;
    for s in 0..2 {
        let b = s * N_DIMS as usize;
        values[b + 2] = 9.0;
        values[b + 3] = 0.5;
        values[b + 4] = 0.5;
    }

    let emitters_ab = two_emitters();
    let req = base_request(width, height, emitters_ab);
    let outs = req.outputs;
    let bundle = admit_comparative_projections(req).expect("admit");
    let out = execute_field_sweep_cpu_chain(&values, &bundle.registrations).expect("run");
    assert_eq!(
        out[outs.dominance_col.raw()],
        10.0,
        "authored-first class wins exact tie"
    );
    assert_eq!(out[N_DIMS as usize + outs.dominance_col.raw()], 20.0);
    // Border between slot0 and slot1: different winners
    assert!(
        out[outs.border_col.raw()] >= 0.5 || out[N_DIMS as usize + outs.border_col.raw()] >= 0.5,
        "tie-break identity must create a winner-change border with neighbor"
    );

    // Reverse registration order → opposite tie winner on slot0
    let emitters_ba = vec![
        ComparativeEmitterClass {
            class_id: 20.0,
            value_col: col(1, N_DIMS),
        },
        ComparativeEmitterClass {
            class_id: 10.0,
            value_col: col(0, N_DIMS),
        },
    ];
    let req_ba = base_request(width, height, emitters_ba);
    let bundle_ba = admit_comparative_projections(req_ba).expect("ba");
    let out_ba = execute_field_sweep_cpu_chain(&values, &bundle_ba.registrations).expect("ba");
    assert_eq!(
        out_ba[outs.dominance_col.raw()],
        20.0,
        "reversing authored order reverses tie-break identity"
    );
}

#[test]
fn grid_and_link_graph_adjacencies_both_host_winner_border() {
    let n4 = FieldAdjacency::grid_n4(4, 3, GRID_N4_NSEW, col(0, N_DIMS)).expect("n4");
    let link_rows = {
        let mut rows = vec![Vec::new(); 4];
        // line 0-1-2-3
        for (a, b) in [(0u32, 1u32), (1, 2), (2, 3)] {
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
    let link = FieldAdjacency::link_graph(4, link_rows, col(0, N_DIMS)).expect("link");

    for (name, adj, slots) in [("N4", n4, 12u32), ("LinkGraph", link, 4u32)] {
        let emitters = two_emitters();
        let req = ComparativeProjectionRequest {
            adjacency: adj,
            n_dims: N_DIMS,
            emitters: emitters.clone(),
            outputs: outputs(),
            palma_d_col: col(2, N_DIMS),
            guyang_value_col: col(3, N_DIMS),
            guyang_conductance_col: col(4, N_DIMS),
            stall_outputs: stall_outs(),
            bands: ComparativeProjectionBands::default(),
            authored_opt_out_reason: None,
        };
        let bundle = admit_comparative_projections(req).expect(name);
        let mut values = vec![0.0; (slots * N_DIMS) as usize];
        for s in 0..slots as usize {
            let b = s * N_DIMS as usize;
            if s < slots as usize / 2 {
                values[b] = 0.9;
                values[b + 1] = 0.2;
            } else {
                values[b] = 0.2;
                values[b + 1] = 0.9;
            }
            values[b + 2] = 12.0;
            values[b + 3] = 0.5;
            values[b + 4] = 0.5;
        }
        let out = execute_field_sweep_cpu_chain(&values, &bundle.registrations).expect(name);
        let borders = column(&out, N_DIMS as usize, outputs().border_col.raw());
        assert!(
            borders.iter().any(|&b| b >= 0.5),
            "{name} must surface winner-identity border"
        );
        let _ = emitters;
    }
}

#[test]
fn default_derived_anchored_install_path_mints_fixed_columns() {
    let mut registry = DimensionRegistry::new();
    let e0 = {
        let mut p = SimProperty::simple("emit", "class_a", 1);
        p.admission_disposition = PropertyAdmissionDisposition::Anchored;
        registry.register(p)
    };
    let e1 = {
        let mut p = SimProperty::simple("emit", "class_b", 1);
        p.admission_disposition = PropertyAdmissionDisposition::Anchored;
        registry.register(p)
    };
    let palma = {
        let mut p = SimProperty::simple("palma", "d", 1);
        p.admission_disposition = PropertyAdmissionDisposition::Anchored;
        registry.register(p)
    };
    let gu = {
        let mut p = SimProperty::simple("guyang", "u", 1);
        p.admission_disposition = PropertyAdmissionDisposition::Anchored;
        registry.register(p)
    };
    let gc = {
        let mut p = SimProperty::simple("guyang", "c", 1);
        p.admission_disposition = PropertyAdmissionDisposition::Anchored;
        registry.register(p)
    };

    let adjacency =
        FieldAdjacency::grid_n4(4, 4, GRID_N4_NSEW, ColumnIndex::from_gpu_round_trip(0))
            .expect("adj");
    let admission = derive_comparative_projections_at_admission(
        &mut registry,
        &[e0, e1],
        adjacency,
        palma,
        gu,
        gc,
        ComparativeProjectionBands::default(),
        None,
    )
    .expect("derive");

    assert_eq!(
        admission.disposition,
        ComparativeProjectionDisposition::Born {
            emitter_count: 2,
            derived_column_count: COMPARATIVE_DERIVED_COLUMN_COUNT,
        }
    );
    assert!(!admission.bundle.registrations.is_empty());
    // Derived properties are Anchored.
    for pid in [
        admission.derived_property_ids.dominance,
        admission.derived_property_ids.margin,
        admission.derived_property_ids.border,
        admission.derived_property_ids.stall,
    ] {
        assert!(
            registry.property(pid).admission_disposition.is_anchored(),
            "derived property must be Anchored"
        );
    }
    // Fresh registry path: three emitters still yields same derived column count.
    let mut reg3 = DimensionRegistry::new();
    let ids: Vec<_> = (0..3)
        .map(|i| {
            let mut p = SimProperty::simple("emit", &format!("c{i}"), 1);
            p.admission_disposition = PropertyAdmissionDisposition::Anchored;
            reg3.register(p)
        })
        .collect();
    let palma3 = {
        let mut p = SimProperty::simple("palma", "d", 1);
        p.admission_disposition = PropertyAdmissionDisposition::Anchored;
        reg3.register(p)
    };
    let gu3 = {
        let mut p = SimProperty::simple("guyang", "u", 1);
        p.admission_disposition = PropertyAdmissionDisposition::Anchored;
        reg3.register(p)
    };
    let gc3 = {
        let mut p = SimProperty::simple("guyang", "c", 1);
        p.admission_disposition = PropertyAdmissionDisposition::Anchored;
        reg3.register(p)
    };
    let adj3 =
        FieldAdjacency::grid_n4(4, 4, GRID_N4_NSEW, ColumnIndex::from_gpu_round_trip(0)).expect("a");
    let adm3 = derive_comparative_projections_at_admission(
        &mut reg3,
        &ids,
        adj3,
        palma3,
        gu3,
        gc3,
        ComparativeProjectionBands::default(),
        None,
    )
    .expect("3emit");
    assert_eq!(
        adm3.bundle.derived_column_count,
        COMPARATIVE_DERIVED_COLUMN_COUNT
    );
}

#[test]
fn unmodified_tp_load_install_surfaces_front_and_chokepoint_with_controls() {
    // Real TP skeleton load path (unmodified asset).
    let scenario = deserialize_scenario_authority(TERRAN_PIRATE_SKELETON).expect("deserialize");
    validate_stead_mapping_consistency(&scenario).expect("stead");
    validate_scenario_links(&scenario).expect("links");
    let theater = match compile_structural_n4_theater(
        &scenario,
        MappingExecutionProfile::SparseRegionFieldV1,
    )
    .expect("compile theater")
    {
        StructuralTheaterAdmission::Admit(t) => t,
        StructuralTheaterAdmission::AtlasDeferred { reason, .. } => {
            panic!("expected admit, got deferral {reason:?}")
        }
    };
    let width = theater.frame_width;
    let height = theater.frame_height;
    assert!(width >= 2 && height >= 2, "TP theater must be spatial");

    // Default-derived birth from ≥2 admitted emitter classes over the TP theater
    // adjacency (no scenario projection wiring; emitters are field-output properties).
    let mut registry = DimensionRegistry::new();
    let e0 = {
        let mut p = SimProperty::simple("field", "emitter_a", 1);
        p.admission_disposition = PropertyAdmissionDisposition::Anchored;
        registry.register(p)
    };
    let e1 = {
        let mut p = SimProperty::simple("field", "emitter_b", 1);
        p.admission_disposition = PropertyAdmissionDisposition::Anchored;
        registry.register(p)
    };
    let palma = {
        let mut p = SimProperty::simple("field", "palma_d", 1);
        p.admission_disposition = PropertyAdmissionDisposition::Anchored;
        registry.register(p)
    };
    let gu = {
        let mut p = SimProperty::simple("field", "guyang_u", 1);
        p.admission_disposition = PropertyAdmissionDisposition::Anchored;
        registry.register(p)
    };
    let gc = {
        let mut p = SimProperty::simple("field", "guyang_c", 1);
        p.admission_disposition = PropertyAdmissionDisposition::Anchored;
        registry.register(p)
    };

    let adjacency =
        FieldAdjacency::grid_n4(width, height, GRID_N4_NSEW, ColumnIndex::from_gpu_round_trip(0))
            .expect("tp grid adj");
    let admission = derive_comparative_projections_at_admission(
        &mut registry,
        &[e0, e1],
        adjacency,
        palma,
        gu,
        gc,
        ComparativeProjectionBands {
            both_strong_floor: 0.2,
            small_margin: 0.4,
            palma_low_d: 4.0,
            contested_border_floor: 0.5,
        },
        None,
    )
    .expect("tp derive");
    assert!(matches!(
        admission.disposition,
        ComparativeProjectionDisposition::Born { .. }
    ));

    let n_dims = registry.total_columns as u32;
    let slots = width * height;
    let mut values = vec![0.0; (slots * n_dims) as usize];
    let e0c = registry.column_range(e0).start;
    let e1c = registry.column_range(e1).start;
    let dc = registry.column_range(palma).start;
    let uc = registry.column_range(gu).start;
    let cc = registry.column_range(gc).start;
    let mid_x = width / 2;
    let mid_y = height / 2;
    for y in 0..height {
        for x in 0..width {
            let slot = (y * width + x) as usize;
            let b = slot * n_dims as usize;
            if x < mid_x {
                values[b + e0c] = 0.9;
                values[b + e1c] = 0.2;
            } else if x > mid_x {
                values[b + e0c] = 0.2;
                values[b + e1c] = 0.9;
            } else {
                values[b + e0c] = 0.55;
                values[b + e1c] = 0.55;
            }
            values[b + dc] = if x == mid_x && y == mid_y { 1.0 } else { 12.0 };
            values[b + uc] = if x < mid_x {
                1.0
            } else if x > mid_x {
                0.0
            } else {
                0.5
            };
            values[b + cc] = 0.5;
        }
    }

    let projected = execute_field_sweep_cpu_chain(&values, &admission.bundle.registrations)
        .expect("project");
    let border_c = admission.outputs.border_col.raw();
    let choke_c = admission.outputs.chokepoint_col.raw();
    let borders = column(&projected, n_dims as usize, border_c);
    assert!(
        borders.iter().any(|&b| b >= 0.5),
        "TP theater must surface a Gu-Yang/front winner-identity border"
    );

    // Ordinary threshold path on Anchored comparative columns.
    let mid_slot = mid_y * width + mid_x;
    let mut regs: Vec<EmitOnThresholdRegistration> = (0..slots)
        .map(|slot| EmitOnThresholdRegistration {
            slot: SlotIndex::new(slot),
            col: admission.outputs.border_col,
            threshold: 0.5,
            direction: ThresholdDirection::Upward,
            event_kind: comparative_event_kind::FRONT_FORMED,
            buffer: Default::default(),
        })
        .collect();
    regs.push(EmitOnThresholdRegistration {
        slot: SlotIndex::new(mid_slot),
        col: admission.outputs.chokepoint_col,
        threshold: 0.5,
        direction: ThresholdDirection::Upward,
        event_kind: comparative_event_kind::CHOKEPOINT_EMERGED,
        buffer: Default::default(),
    });
    let ops: Vec<_> = regs
        .iter()
        .map(emit_on_threshold_registration_to_op)
        .collect();
    let kinds: Vec<_> = regs.iter().map(|r| r.event_kind).collect();
    let mut projected_mut = projected.clone();
    let emissions =
        execute_threshold_ops_cpu(&values, &mut projected_mut, &ops, n_dims).expect("thresh");
    let front = emissions
        .iter()
        .filter(|e| kinds[e.reg_idx() as usize] == comparative_event_kind::FRONT_FORMED)
        .count();
    let choke = emissions
        .iter()
        .filter(|e| kinds[e.reg_idx() as usize] == comparative_event_kind::CHOKEPOINT_EMERGED)
        .count();
    assert!(front > 0, "front-formed via ordinary threshold path");
    assert_eq!(choke, 1, "exactly one chokepoint-emerged on low-D corridor");

    // Control A: no PALMA-low-D
    let mut no_d = values.clone();
    for s in 0..slots as usize {
        no_d[s * n_dims as usize + dc] = 20.0;
    }
    let out_d = execute_field_sweep_cpu_chain(&no_d, &admission.bundle.registrations).expect("no d");
    assert!(
        column(&out_d, n_dims as usize, choke_c)
            .iter()
            .all(|&c| c < 0.5),
        "absent PALMA-low-D suppresses chokepoint"
    );

    // Control B: no contested border (single winner everywhere)
    let mut no_border = values.clone();
    for s in 0..slots as usize {
        let b = s * n_dims as usize;
        no_border[b + e0c] = 0.9;
        no_border[b + e1c] = 0.1;
        no_border[b + dc] = 1.0;
    }
    let out_b =
        execute_field_sweep_cpu_chain(&no_border, &admission.bundle.registrations).expect("no b");
    assert!(
        column(&out_b, n_dims as usize, choke_c)
            .iter()
            .all(|&c| c < 0.5),
        "absent contested-border suppresses chokepoint"
    );
    assert!(
        column(&out_b, n_dims as usize, border_c)
            .iter()
            .all(|&c| c < 0.5),
        "single winner has no identity-change border"
    );
}
