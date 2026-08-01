//! GUYANG-COMPARATIVE-PROJECTIONS-0 — sealed dominance/margin/contest/border/chokepoint
//! projections over co-located generic field-sweep outputs.

use simthing_core::{
    emit_on_threshold_registration_to_op, ColumnIndex, EmitOnThresholdRegistration, SlotIndex,
    ThresholdDirection,
};
use simthing_driver::{
    admit_comparative_projections, comparative_event_kind, comparative_projection_cpu_oracle,
    ComparativeEmitterClass, ComparativeProjectionBands, ComparativeProjectionDisposition,
    ComparativeProjectionOutputs, ComparativeProjectionRequest, COMPARATIVE_DERIVED_COLUMN_COUNT,
};
use simthing_gpu::{
    execute_field_sweep_cpu_chain, execute_threshold_ops_cpu, FieldAdjacency, FieldSweepSession,
    GpuContext, GRID_N4_NSEW,
};

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
        Ok(context) => Some(context),
        Err(_) if std::env::var_os("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH").is_some() => {
            panic!("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH set but no GPU adapter is available")
        }
        Err(_) => None,
    }
}

fn outputs(n_dims: u32) -> ComparativeProjectionOutputs {
    ComparativeProjectionOutputs {
        dominance_col: col(3, n_dims),
        margin_col: col(4, n_dims),
        contest_col: col(5, n_dims),
        border_col: col(6, n_dims),
        chokepoint_col: col(7, n_dims),
    }
}

fn two_emitters(n_dims: u32) -> Vec<ComparativeEmitterClass> {
    vec![
        ComparativeEmitterClass {
            class_id: 10.0,
            value_col: col(0, n_dims),
        },
        ComparativeEmitterClass {
            class_id: 20.0,
            value_col: col(1, n_dims),
        },
    ]
}

fn base_request(
    width: u32,
    height: u32,
    n_dims: u32,
    emitters: Vec<ComparativeEmitterClass>,
) -> ComparativeProjectionRequest {
    let adjacency = FieldAdjacency::grid_n4(width, height, GRID_N4_NSEW, col(0, n_dims))
        .expect("grid adjacency");
    ComparativeProjectionRequest {
        adjacency,
        n_dims,
        emitters,
        outputs: outputs(n_dims),
        palma_d_col: col(2, n_dims),
        bands: ComparativeProjectionBands::default(),
        authored_opt_out_reason: None,
    }
}

/// Two opposing influence blobs with a mid-front and a low-D corridor cell.
///
/// At the mid column both emitters are equal and both-strong so margin is zero
/// (border band) while left/right of the front one side dominates.
fn synthetic_front_values(width: u32, height: u32, n_dims: u32) -> Vec<f32> {
    let mut values = vec![0.0; (width * height * n_dims) as usize];
    let mid_x = width / 2;
    let mid_y = height / 2;
    for y in 0..height {
        for x in 0..width {
            let slot = (y * width + x) as usize;
            let base = slot * n_dims as usize;
            let (left, right) = if x < mid_x {
                (0.9, 0.2)
            } else if x > mid_x {
                (0.2, 0.9)
            } else {
                // Contested front column: both-strong, exact tie → margin 0.
                (0.55, 0.55)
            };
            values[base] = left;
            values[base + 1] = right;
            values[base + 2] = if x == mid_x && y == mid_y {
                1.0
            } else if x == mid_x {
                8.0
            } else {
                12.0
            };
        }
    }
    values
}

#[test]
fn default_derived_projection_admission_for_1_2_3_and_many_emitter_classes() {
    let n_dims = 16u32;
    let width = 4u32;
    let height = 4u32;

    let one = admit_comparative_projections(base_request(
        width,
        height,
        n_dims,
        vec![ComparativeEmitterClass {
            class_id: 1.0,
            value_col: col(0, n_dims),
        }],
    ))
    .expect("admit one");
    assert!(matches!(
        one.disposition,
        ComparativeProjectionDisposition::InsufficientEmitters { emitter_count: 1 }
    ));
    assert!(one.registrations.is_empty());
    assert_eq!(one.derived_column_count, 0);

    let mut opt = base_request(width, height, n_dims, two_emitters(n_dims));
    opt.authored_opt_out_reason = Some("domain_suppresses_fronts");
    let opted = admit_comparative_projections(opt).expect("opt-out");
    assert_eq!(
        opted.disposition,
        ComparativeProjectionDisposition::AuthoredOptOut {
            reason: "domain_suppresses_fronts"
        }
    );
    assert!(opted.registrations.is_empty());

    for n in [2usize, 3, 8] {
        let emitters: Vec<_> = (0..n)
            .map(|i| ComparativeEmitterClass {
                class_id: (i as f32) + 1.0,
                value_col: col(i as u32, n_dims),
            })
            .collect();
        let mut req = base_request(width, height, n_dims, emitters);
        let base = n as u32 + 1;
        req.palma_d_col = col(n as u32, n_dims);
        req.outputs = ComparativeProjectionOutputs {
            dominance_col: col(base, n_dims),
            margin_col: col(base + 1, n_dims),
            contest_col: col(base + 2, n_dims),
            border_col: col(base + 3, n_dims),
            chokepoint_col: col(base + 4, n_dims),
        };
        let bundle = admit_comparative_projections(req).expect("admit many");
        assert_eq!(
            bundle.disposition,
            ComparativeProjectionDisposition::Born {
                emitter_count: n as u32,
                derived_column_count: COMPARATIVE_DERIVED_COLUMN_COUNT,
            }
        );
        assert_eq!(bundle.derived_column_count, COMPARATIVE_DERIVED_COLUMN_COUNT);
        assert!(!bundle.registrations.is_empty());
    }
}

#[test]
fn cpu_oracle_and_field_eml_agree_on_dominance_margin_contest_border_chokepoint() {
    let width = 8u32;
    let height = 6u32;
    let n_dims = 8u32;
    let emitters = two_emitters(n_dims);
    let req = base_request(width, height, n_dims, emitters.clone());
    let outs = req.outputs;
    let bands = req.bands;
    let adjacency = req.adjacency.clone();
    let bundle = admit_comparative_projections(req).expect("admit");
    let values = synthetic_front_values(width, height, n_dims);

    let oracle = comparative_projection_cpu_oracle(
        &values,
        width * height,
        n_dims,
        &emitters,
        outs,
        col(2, n_dims),
        bands,
        &adjacency,
    );
    let eml = execute_field_sweep_cpu_chain(&values, &bundle.registrations).expect("eml chain");

    for (name, c) in [
        ("dominance", outs.dominance_col.raw()),
        ("margin", outs.margin_col.raw()),
        ("contest", outs.contest_col.raw()),
        ("border", outs.border_col.raw()),
        ("chokepoint", outs.chokepoint_col.raw()),
    ] {
        assert!(
            bits_equal(
                &column(&oracle, n_dims as usize, c),
                &column(&eml, n_dims as usize, c)
            ),
            "{name} must match oracle bit-for-bit"
        );
    }

    let borders = column(&eml, n_dims as usize, outs.border_col.raw());
    assert!(
        borders.iter().any(|&b| b >= 0.5),
        "synthetic front must surface a border band"
    );
    let chokes = column(&eml, n_dims as usize, outs.chokepoint_col.raw());
    let choke_count = chokes.iter().filter(|&&c| c >= 0.5).count();
    assert_eq!(choke_count, 1, "exactly one chokepoint-emerged locus");
}

#[test]
fn cpu_gpu_bit_parity_for_comparative_projection_chain() {
    let Some(ctx) = gpu_context() else {
        eprintln!("skipping GPU parity — no adapter");
        return;
    };
    let width = 8u32;
    let height = 6u32;
    let n_dims = 8u32;
    let req = base_request(width, height, n_dims, two_emitters(n_dims));
    let outs = req.outputs;
    let bundle = admit_comparative_projections(req).expect("admit");
    let values = synthetic_front_values(width, height, n_dims);
    let cpu = execute_field_sweep_cpu_chain(&values, &bundle.registrations).expect("cpu");

    let mut session =
        FieldSweepSession::new(&ctx, &bundle.registrations[0]).expect("gpu session");
    session
        .upload_values(&ctx, &values)
        .expect("upload comparative values");
    session
        .dispatch_chain(&ctx, &bundle.registrations, 1)
        .expect("dispatch comparative chain");
    let gpu = session.readback(&ctx).expect("readback comparative values");

    for (name, c) in [
        ("dominance", outs.dominance_col.raw()),
        ("margin", outs.margin_col.raw()),
        ("contest", outs.contest_col.raw()),
        ("border", outs.border_col.raw()),
        ("chokepoint", outs.chokepoint_col.raw()),
    ] {
        assert!(
            bits_equal(
                &column(&cpu, n_dims as usize, c),
                &column(&gpu, n_dims as usize, c)
            ),
            "CPU/GPU {name} parity"
        );
    }
    let info = ctx.adapter.get_info();
    eprintln!(
        "GUYANG-COMPARATIVE-PROJECTIONS adapter={} backend={:?}",
        info.name, info.backend
    );
}

#[test]
fn deterministic_authored_tie_break_and_registration_order_reversal_falsifier() {
    let width = 2u32;
    let height = 1u32;
    let n_dims = 8u32;
    let mut values = vec![0.0; (width * height * n_dims) as usize];
    values[0] = 1.0;
    values[1] = 1.0;
    values[2] = 9.0;
    values[n_dims as usize] = 0.2;
    values[n_dims as usize + 1] = 0.8;
    values[n_dims as usize + 2] = 9.0;

    let emitters_ab = vec![
        ComparativeEmitterClass {
            class_id: 1.0,
            value_col: col(0, n_dims),
        },
        ComparativeEmitterClass {
            class_id: 2.0,
            value_col: col(1, n_dims),
        },
    ];
    let emitters_ba = vec![
        ComparativeEmitterClass {
            class_id: 2.0,
            value_col: col(1, n_dims),
        },
        ComparativeEmitterClass {
            class_id: 1.0,
            value_col: col(0, n_dims),
        },
    ];

    let req_ab = base_request(width, height, n_dims, emitters_ab);
    let outs = req_ab.outputs;
    let bundle_ab = admit_comparative_projections(req_ab).expect("ab");
    let out_ab = execute_field_sweep_cpu_chain(&values, &bundle_ab.registrations).expect("run ab");
    assert_eq!(
        out_ab[outs.dominance_col.raw()],
        1.0,
        "authored order A-before-B must win exact ties"
    );
    assert_eq!(out_ab[outs.margin_col.raw()], 0.0, "exact tie margin is zero");

    let req_ba = base_request(width, height, n_dims, emitters_ba);
    let bundle_ba = admit_comparative_projections(req_ba).expect("ba");
    let out_ba = execute_field_sweep_cpu_chain(&values, &bundle_ba.registrations).expect("run ba");
    assert_eq!(
        out_ba[outs.dominance_col.raw()],
        2.0,
        "reversing authored emitter order must reverse the tie-break winner"
    );

    let planted_wrong = 2.0;
    assert_ne!(
        out_ab[outs.dominance_col.raw()],
        planted_wrong,
        "tie-break must not be class_id magnitude or hash order"
    );
}

#[test]
fn chokepoint_conjunction_controls_suppress_when_either_predicate_absent() {
    let width = 4u32;
    let height = 3u32;
    let n_dims = 8u32;
    let req = base_request(width, height, n_dims, two_emitters(n_dims));
    let outs = req.outputs;
    let bundle = admit_comparative_projections(req).expect("admit");
    let mut values = synthetic_front_values(width, height, n_dims);

    let full = execute_field_sweep_cpu_chain(&values, &bundle.registrations).expect("full");
    let full_chokes = column(&full, n_dims as usize, outs.chokepoint_col.raw())
        .into_iter()
        .filter(|&c| c >= 0.5)
        .count();
    assert_eq!(full_chokes, 1);

    for slot in 0..(width * height) as usize {
        values[slot * n_dims as usize + 2] = 20.0;
    }
    let no_low_d = execute_field_sweep_cpu_chain(&values, &bundle.registrations).expect("no d");
    assert!(
        column(&no_low_d, n_dims as usize, outs.chokepoint_col.raw())
            .iter()
            .all(|&c| c < 0.5),
        "absent PALMA-low-D must suppress chokepoint-emerged"
    );

    let mut values = synthetic_front_values(width, height, n_dims);
    for slot in 0..(width * height) as usize {
        let base = slot * n_dims as usize;
        values[base] = 0.9;
        values[base + 1] = 0.1;
    }
    let mid = ((height / 2) * width + width / 2) as usize;
    values[mid * n_dims as usize + 2] = 1.0;
    let no_border =
        execute_field_sweep_cpu_chain(&values, &bundle.registrations).expect("no border");
    assert!(
        column(&no_border, n_dims as usize, outs.chokepoint_col.raw())
            .iter()
            .all(|&c| c < 0.5),
        "absent contested-border must suppress chokepoint-emerged"
    );
}

#[test]
fn unmodified_tp_scenario_has_zero_projection_wiring_and_front_chokepoint_witness() {
    let clause = include_str!("../../../scenarios/terran_pirate_galaxy.clause");
    for forbidden in [
        "comparative_projection",
        "dominance_col",
        "chokepoint_emerged",
        "border_band",
        "guyang_projection",
        "front_formed",
    ] {
        assert!(
            !clause.to_ascii_lowercase().contains(forbidden),
            "TP scenario must not author projection wiring token {forbidden}"
        );
    }

    let width = 8u32;
    let height = 6u32;
    let n_dims = 8u32;
    let req = base_request(width, height, n_dims, two_emitters(n_dims));
    let outs = req.outputs;
    let bundle = admit_comparative_projections(req).expect("admit");
    let values = synthetic_front_values(width, height, n_dims);
    let mut projected =
        execute_field_sweep_cpu_chain(&values, &bundle.registrations).expect("project");

    let mid = ((height / 2) * width + width / 2) as usize;
    let mut regs: Vec<EmitOnThresholdRegistration> = (0..width * height)
        .map(|slot| EmitOnThresholdRegistration {
            slot: SlotIndex::new(slot),
            col: outs.border_col,
            threshold: 0.5,
            direction: ThresholdDirection::Upward,
            event_kind: comparative_event_kind::FRONT_FORMED,
            buffer: Default::default(),
        })
        .collect();
    regs.push(EmitOnThresholdRegistration {
        slot: SlotIndex::new(mid as u32),
        col: outs.chokepoint_col,
        threshold: 0.5,
        direction: ThresholdDirection::Upward,
        event_kind: comparative_event_kind::CHOKEPOINT_EMERGED,
        buffer: Default::default(),
    });

    let prev = values.clone();
    let ops: Vec<_> = regs
        .iter()
        .map(emit_on_threshold_registration_to_op)
        .collect();
    let kinds: Vec<_> = regs.iter().map(|r| r.event_kind).collect();
    let emissions =
        execute_threshold_ops_cpu(&prev, &mut projected, &ops, n_dims).expect("threshold ops");

    let front_events = emissions
        .iter()
        .filter(|e| kinds[e.reg_idx() as usize] == comparative_event_kind::FRONT_FORMED)
        .count();
    let choke_events = emissions
        .iter()
        .filter(|e| kinds[e.reg_idx() as usize] == comparative_event_kind::CHOKEPOINT_EMERGED)
        .count();
    assert!(front_events > 0, "front-formed must emerge from border band");
    assert_eq!(
        choke_events, 1,
        "exactly one chokepoint-emerged through ordinary threshold path"
    );
}

#[test]
fn derived_column_count_independent_of_owner_count_census() {
    let width = 2u32;
    let height = 2u32;
    let n_dims = 32u32;
    let mut counts = Vec::new();
    for n in [2u32, 3, 5, 12] {
        let emitters: Vec<_> = (0..n)
            .map(|i| ComparativeEmitterClass {
                class_id: i as f32,
                value_col: col(i, n_dims),
            })
            .collect();
        let mut req = base_request(width, height, n_dims, emitters);
        req.palma_d_col = col(n, n_dims);
        let base = n + 1;
        req.outputs = ComparativeProjectionOutputs {
            dominance_col: col(base, n_dims),
            margin_col: col(base + 1, n_dims),
            contest_col: col(base + 2, n_dims),
            border_col: col(base + 3, n_dims),
            chokepoint_col: col(base + 4, n_dims),
        };
        let bundle = admit_comparative_projections(req).expect("admit");
        counts.push(bundle.derived_column_count);
    }
    assert!(
        counts.iter().all(|&c| c == COMPARATIVE_DERIVED_COLUMN_COUNT),
        "derived column count must be independent of emitter/owner count: {counts:?}"
    );
}
