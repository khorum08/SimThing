//! GUYANG-COMPARATIVE-PROJECTIONS-0 — Remand 3A (Owner absolute: TP purged forever).
//! Scenario-neutral fixtures only. No TP asset, fixture, corpus, or coupling.
//! Board correction `5150987561`: TP witness is not a coder obligation.

use simthing_core::{
    emit_on_threshold_registration_to_op, ColumnIndex, DimensionRegistry, EmitOnThresholdRegistration,
    PropertyAdmissionDisposition, SimProperty, SimThing, SimThingKind, SlotIndex, ThresholdDirection,
};
use simthing_driver::{
    admit_default_comparative_projections, comparative_event_kind, comparative_projection_cpu_oracle,
    compile_and_install, neighbor_slots_from_grid, neighbor_slots_from_link_rows,
    ComparativeProjectionBands, ComparativeProjectionDisposition, Scenario, BAND_READOUT_COLUMN_COUNT,
    COMPARATIVE_DERIVED_COLUMN_COUNT, COMPARATIVE_EMITTER_NAMESPACE, TRIAD_GUYANG_C, TRIAD_GUYANG_U,
    TRIAD_NAMESPACE, TRIAD_PALMA_D,
};
use simthing_gpu::{
    execute_field_sweep_cpu_chain, execute_threshold_ops_cpu, FieldAdjacency, FieldSweepSession,
    GpuContext, LinkGraphNeighbor, SlotAllocator, GRID_N4_NSEW,
};
use simthing_spec::{GameModeSpec, PropertySpec, SpecVersion};
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

/// Register ≥2 comparative emitters + triad fields; name order = authored_order.
fn registry_with_emitters(names: &[&str]) -> DimensionRegistry {
    let mut reg = DimensionRegistry::new();
    for name in names {
        let mut p = SimProperty::simple(COMPARATIVE_EMITTER_NAMESPACE, name, 1);
        p.admission_disposition = PropertyAdmissionDisposition::Anchored;
        reg.register(p);
    }
    for (ns, name) in [
        (TRIAD_NAMESPACE, TRIAD_PALMA_D),
        (TRIAD_NAMESPACE, TRIAD_GUYANG_U),
        (TRIAD_NAMESPACE, TRIAD_GUYANG_C),
    ] {
        let mut p = SimProperty::simple(ns, name, 1);
        p.admission_disposition = PropertyAdmissionDisposition::Anchored;
        reg.register(p);
    }
    reg
}

fn admit_grid(
    reg: &mut DimensionRegistry,
    width: u32,
    height: u32,
) -> simthing_driver::ComparativeProjectionAdmission {
    let gather = ColumnIndex::from_gpu_round_trip(0);
    let adj = FieldAdjacency::grid_n4(width, height, GRID_N4_NSEW, gather).expect("grid");
    let neighbors = neighbor_slots_from_grid(&adj).expect("neighbors");
    admit_default_comparative_projections(
        reg,
        adj,
        neighbors,
        ComparativeProjectionBands::default(),
    )
    .expect("admit_default")
}

fn prop(ns: &str, name: &str) -> PropertySpec {
    PropertySpec {
        id: format!("{ns}_{name}"),
        namespace: ns.into(),
        name: name.into(),
        display_name: name.into(),
        description: String::new(),
        sub_fields: Vec::new(),
        admission_disposition: PropertyAdmissionDisposition::Anchored,
    }
}

/// Canonical install path: GameMode properties → compile_and_install → automatic birth.
#[test]
fn default_derived_birth_via_canonical_install_path() {
    let n_slots = 16u32; // perfect square → Grid-N4 theater in install hook
    let mut registry = DimensionRegistry::new();
    let _ = registry.register(SimProperty::simple("_seed", "pad", 0));
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let scenario = Scenario {
        name: "comparative_install_neutral".into(),
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
        id: "comparative_install".into(),
        display_name: "comparative install".into(),
        description: String::new(),
        spec_version: SpecVersion::default(),
        metadata: Default::default(),
        domain_packs: Vec::new(),
        properties: vec![
            prop(COMPARATIVE_EMITTER_NAMESPACE, "alpha"),
            prop(COMPARATIVE_EMITTER_NAMESPACE, "beta"),
            prop(TRIAD_NAMESPACE, TRIAD_PALMA_D),
            prop(TRIAD_NAMESPACE, TRIAD_GUYANG_U),
            prop(TRIAD_NAMESPACE, TRIAD_GUYANG_C),
        ],
        overlays: Vec::new(),
        order_weight_classes: Vec::new(),
        capability_trees: Vec::new(),
        events: Vec::new(),
        resource_flow: None,
        resource_economy: None,
        resource_flow_execution_profile: Default::default(),
        region_fields: Vec::new(),
        mapping_execution_profile: Default::default(),
    };
    let state = compile_and_install(&game_mode, &scenario, &mut registry, &mut root, &mut allocator)
        .expect("install");
    let admission = state
        .comparative_projection
        .expect("install must birth comparative projection when ≥2 emitters + triad present");
    assert_eq!(
        admission.disposition,
        ComparativeProjectionDisposition::Born {
            emitter_count: 2,
            comparative_column_count: COMPARATIVE_DERIVED_COLUMN_COUNT,
        }
    );
    assert_eq!(
        admission.bundle.comparative_column_count,
        COMPARATIVE_DERIVED_COLUMN_COUNT
    );
    assert!(!admission.bundle.registrations.is_empty());
}

#[test]
fn default_derived_birth_via_production_door_not_manual_request() {
    // Sole production door discovers emitters from the admitted registry.
    let mut reg = registry_with_emitters(&["alpha", "beta"]);
    let admission = admit_grid(&mut reg, 4, 4);
    assert_eq!(
        admission.disposition,
        ComparativeProjectionDisposition::Born {
            emitter_count: 2,
            comparative_column_count: COMPARATIVE_DERIVED_COLUMN_COUNT,
        }
    );
    assert_eq!(
        admission.bundle.comparative_column_count,
        COMPARATIVE_DERIVED_COLUMN_COUNT
    );
    assert_eq!(COMPARATIVE_DERIVED_COLUMN_COUNT, 3);
    assert_eq!(BAND_READOUT_COLUMN_COUNT, 2);
    assert!(!admission.bundle.registrations.is_empty());

    // One emitter → insufficient (no fabricated comparison).
    let mut one = registry_with_emitters(&["solo"]);
    // Still need triad for the door to run past discovery — actually with 1 emitter
    // admit_default returns Insufficient before triad check... discovery happens first.
    let gather = ColumnIndex::from_gpu_round_trip(0);
    let adj = FieldAdjacency::grid_n4(2, 2, GRID_N4_NSEW, gather).expect("g");
    let n = neighbor_slots_from_grid(&adj).expect("n");
    let one_adm = admit_default_comparative_projections(
        &mut one,
        adj,
        n,
        ComparativeProjectionBands::default(),
    )
    .expect("one");
    assert!(matches!(
        one_adm.disposition,
        ComparativeProjectionDisposition::InsufficientEmitters { emitter_count: 1 }
    ));

    // 3 emitters still 3 comparative columns.
    let mut three = registry_with_emitters(&["a", "b", "c"]);
    let adm3 = admit_grid(&mut three, 4, 4);
    assert_eq!(
        adm3.bundle.comparative_column_count,
        COMPARATIVE_DERIVED_COLUMN_COUNT
    );
}

#[test]
fn authored_order_tie_break_invariant_under_registration_vector_reversal() {
    // Same durable names/order; reverse *registration vector* of discovered
    // emitters is impossible at production door (name sort). Prove by admitting
    // with names that sort alpha,beta then beta,alpha as *different* name sets
    // would change authored order — instead: same two names always sort the same.
    //
    // Explicit unit: build two emitter vecs with swapped *vec* order but same
    // authored_order keys → compile_comparative_bundle must agree.
    use simthing_driver::{compile_comparative_bundle, ComparativeEmitterClass,
        ComparativeProjectionOutputs, ComparativeBandReadouts, GuYangStallOutputs,
        ComparativeProjectionRequest};

    let width = 2u32;
    let height = 1u32;
    let n_dims = 20u32;
    let gather = ColumnIndex::try_from_admitted_authored(0, n_dims).unwrap();
    let adj = FieldAdjacency::grid_n4(width, height, GRID_N4_NSEW, gather).unwrap();
    let neighbors = neighbor_slots_from_grid(&adj).unwrap();

    let col = |r: u32| ColumnIndex::try_from_admitted_authored(r, n_dims).unwrap();
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
        .expect("bundle")
    };

    // Same authored_order keys, reversed vector iteration.
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
    let e_ba = vec![e_ab[1], e_ab[0]]; // reversed vec, same authored_order

    let mut values = vec![0.0f32; (width * height * n_dims) as usize];
    // Exact tie on slot 0
    values[0] = 1.0;
    values[1] = 1.0;
    values[2] = 9.0;
    values[3] = 0.5;
    values[4] = 0.5;
    // slot1: clear winner on col1
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

    // Planted wrong: swap authored_order keys while keeping class_id/value_col → must change winner
    let e_wrong = vec![
        ComparativeEmitterClass {
            authored_order: 1,
            class_id: 10.0,
            value_col: col(0),
        },
        ComparativeEmitterClass {
            authored_order: 0,
            class_id: 20.0,
            value_col: col(1),
        },
    ];
    let out_wrong = execute_field_sweep_cpu_chain(&values, &mk(e_wrong).registrations).unwrap();
    assert_eq!(
        out_wrong[10], 20.0,
        "planted incidental-order (swapped authored_order) must flip the referee"
    );
    assert_ne!(out_ab[10], out_wrong[10]);
}

#[test]
fn grid_and_link_graph_cpu_oracle_and_gpu_parity() {
    let mut reg = registry_with_emitters(&["left", "right"]);
    let width = 6u32;
    let height = 4u32;
    let admission = admit_grid(&mut reg, width, height);
    let n_dims = reg.total_columns as u32;
    let slots = width * height;
    let outs = admission.outputs;
    let bands_r = admission.band_readouts;
    let stall = admission.stall_outputs.stall_col;
    let bands = ComparativeProjectionBands::default();

    // Scenario-neutral dual-emitter front (no TP).
    let mut values = vec![0.0f32; (slots * n_dims) as usize];
    let e0 = reg
        .column_range(admission.emitter_property_ids[0])
        .start;
    let e1 = reg
        .column_range(admission.emitter_property_ids[1])
        .start;
    let d = reg
        .column_range(reg.id_of(TRIAD_NAMESPACE, TRIAD_PALMA_D).unwrap())
        .start;
    let u = reg
        .column_range(reg.id_of(TRIAD_NAMESPACE, TRIAD_GUYANG_U).unwrap())
        .start;
    let c = reg
        .column_range(reg.id_of(TRIAD_NAMESPACE, TRIAD_GUYANG_C).unwrap())
        .start;
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
    // Re-derive class_ids from names the same way production does (durable name hash).
    let names = ["left", "right"];
    let emitters: Vec<_> = names
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let mut h: u32 = 2166136261;
            for b in name.as_bytes() {
                h ^= u32::from(*b);
                h = h.wrapping_mul(16777619);
            }
            simthing_driver::ComparativeEmitterClass {
                authored_order: i as u32,
                class_id: (h % 1_000_000) as f32 + 1.0,
                value_col: ColumnIndex::from_gpu_round_trip(
                    reg.column_range(admission.emitter_property_ids[i]).start as u32,
                ),
            }
        })
        .collect();

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
    for c in [
        outs.dominance_col.raw(),
        outs.margin_col.raw(),
        outs.contest_col.raw(),
        bands_r.border_col.raw(),
        bands_r.chokepoint_col.raw(),
        stall.raw(),
    ] {
        assert!(
            bits_equal(
                &column(&oracle, n_dims as usize, c),
                &column(&chain, n_dims as usize, c)
            ),
            "grid oracle parity col {c}"
        );
    }
    assert!(column(&chain, n_dims as usize, bands_r.border_col.raw())
        .iter()
        .any(|&b| b >= 0.5));

    // LinkGraph parity
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
    let mut reg_l = registry_with_emitters(&["left", "right"]);
    let link_adj =
        FieldAdjacency::link_graph(4, link_rows.clone(), ColumnIndex::from_gpu_round_trip(0))
            .unwrap();
    let link_neighbors = neighbor_slots_from_link_rows(&link_rows);
    let adm_l = admit_default_comparative_projections(
        &mut reg_l,
        link_adj,
        link_neighbors.clone(),
        ComparativeProjectionBands::default(),
    )
    .expect("link admit");
    let n_dims_l = reg_l.total_columns as u32;
    let mut vals_l = vec![0.0f32; (4 * n_dims_l) as usize];
    let e0l = reg_l.column_range(adm_l.emitter_property_ids[0]).start;
    let e1l = reg_l.column_range(adm_l.emitter_property_ids[1]).start;
    let dl = reg_l
        .column_range(reg_l.id_of(TRIAD_NAMESPACE, TRIAD_PALMA_D).unwrap())
        .start;
    let ul = reg_l
        .column_range(reg_l.id_of(TRIAD_NAMESPACE, TRIAD_GUYANG_U).unwrap())
        .start;
    let cl = reg_l
        .column_range(reg_l.id_of(TRIAD_NAMESPACE, TRIAD_GUYANG_C).unwrap())
        .start;
    for s in 0..4usize {
        let b = s * n_dims_l as usize;
        if s < 2 {
            vals_l[b + e0l] = 0.9;
            vals_l[b + e1l] = 0.2;
        } else {
            vals_l[b + e0l] = 0.2;
            vals_l[b + e1l] = 0.9;
        }
        vals_l[b + dl] = 12.0;
        vals_l[b + ul] = 0.5;
        vals_l[b + cl] = 0.5;
    }
    let chain_l =
        execute_field_sweep_cpu_chain(&vals_l, &adm_l.bundle.registrations).expect("link chain");
    let emitters_l: Vec<_> = ["left", "right"]
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let mut h: u32 = 2166136261;
            for b in name.as_bytes() {
                h ^= u32::from(*b);
                h = h.wrapping_mul(16777619);
            }
            simthing_driver::ComparativeEmitterClass {
                authored_order: i as u32,
                class_id: (h % 1_000_000) as f32 + 1.0,
                value_col: ColumnIndex::from_gpu_round_trip(
                    reg_l.column_range(adm_l.emitter_property_ids[i]).start as u32,
                ),
            }
        })
        .collect();
    let oracle_l = comparative_projection_cpu_oracle(
        &chain_l,
        4,
        n_dims_l,
        &emitters_l,
        adm_l.outputs,
        adm_l.band_readouts,
        ColumnIndex::from_gpu_round_trip(dl as u32),
        adm_l.stall_outputs.stall_col,
        ComparativeProjectionBands::default(),
        &link_neighbors,
    );
    for c in [
        adm_l.outputs.dominance_col.raw(),
        adm_l.outputs.margin_col.raw(),
        adm_l.band_readouts.border_col.raw(),
    ] {
        assert!(
            bits_equal(
                &column(&oracle_l, n_dims_l as usize, c),
                &column(&chain_l, n_dims_l as usize, c)
            ),
            "link oracle parity col {c}"
        );
    }
    assert!(
        column(&chain_l, n_dims_l as usize, adm_l.band_readouts.border_col.raw())
            .iter()
            .any(|&b| b >= 0.5)
    );

    if let Some(ctx) = gpu_context() {
        let mut session =
            FieldSweepSession::new(&ctx, &admission.bundle.registrations[0]).unwrap();
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
            "GUYANG-COMPARATIVE-PROJECTIONS adapter={} backend={:?}",
            info.name, info.backend
        );
    }
}

#[test]
fn front_formed_hardened_and_chokepoint_via_ordinary_thresholds() {
    let mut reg = registry_with_emitters(&["alpha", "beta"]);
    let width = 6u32;
    let height = 4u32;
    // Default harden band: contest must genuinely exceed front_harden_contest (no 0.0 kabuki).
    let admission = admit_grid(&mut reg, width, height);
    let n_dims = reg.total_columns as u32;
    let slots = width * height;
    let mut values = vec![0.0f32; (slots * n_dims) as usize];
    let e0 = reg.column_range(admission.emitter_property_ids[0]).start;
    let e1 = reg.column_range(admission.emitter_property_ids[1]).start;
    let d = reg
        .column_range(reg.id_of(TRIAD_NAMESPACE, TRIAD_PALMA_D).unwrap())
        .start;
    let u = reg
        .column_range(reg.id_of(TRIAD_NAMESPACE, TRIAD_GUYANG_U).unwrap())
        .start;
    let c = reg
        .column_range(reg.id_of(TRIAD_NAMESPACE, TRIAD_GUYANG_C).unwrap())
        .start;
    let mid = width / 2;
    let mid_y = height / 2;
    // Scenario-neutral dual-emitter front:
    // - mid column: both-strong + small margin so contest consumes stall
    // - mid u=0.5 with left u=1 / right u=0 → opposing flux → stall > 0
    // - mid+mid_y: low Palma D + border → chokepoint band
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

    // Contest must be genuinely positive under opposing Gu-Yang flux + both-strong
    // mid-column emitters (not a threshold-kabuki at 0.0).
    let contest_col = admission.outputs.contest_col.raw();
    let contest_vals = column(&projected, n_dims as usize, contest_col);
    assert!(
        contest_vals
            .iter()
            .any(|&c| c > ComparativeProjectionBands::default().front_harden_contest),
        "expected contest > front_harden_contest somewhere on the front"
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
    let emissions = execute_threshold_ops_cpu(&values, &mut cur, &ops, n_dims).expect("thresh");

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
    assert!(hardened > 0, "front-hardened from contest band");
    assert_eq!(choke, 1, "chokepoint-emerged");

    // Controls: no low-D
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

    // Controls: single winner → no border
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
