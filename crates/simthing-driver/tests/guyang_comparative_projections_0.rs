//! GUYANG-COMPARATIVE-PROJECTIONS-0 — driver-local consumer over generic
//! field-sweep outputs (Remand 1A scope envelope: no kernel/GPU public doors).

use simthing_core::ColumnIndex;
use simthing_driver::{
    admit_comparative_projections, comparative_projection_cpu_oracle, ComparativeEmitterClass,
    ComparativeProjectionBands, ComparativeProjectionDisposition, ComparativeProjectionOutputs,
    ComparativeProjectionRequest, COMPARATIVE_DERIVED_COLUMN_COUNT,
};
use simthing_gpu::{
    execute_field_sweep_cpu_chain, FieldAdjacency, FieldSweepSession, GpuContext, GRID_N4_NSEW,
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

/// Layout: e0, e1, palma_d, guyang_stall, dominance, margin, contest, border, choke
fn outputs(n_dims: u32) -> ComparativeProjectionOutputs {
    ComparativeProjectionOutputs {
        dominance_col: col(4, n_dims),
        margin_col: col(5, n_dims),
        contest_col: col(6, n_dims),
        border_col: col(7, n_dims),
        chokepoint_col: col(8, n_dims),
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
        guyang_stall_col: col(3, n_dims),
        bands: ComparativeProjectionBands::default(),
        authored_opt_out_reason: None,
    }
}

fn synthetic_values(width: u32, height: u32, n_dims: u32) -> Vec<f32> {
    let mut values = vec![0.0; (width * height * n_dims) as usize];
    let mid_x = width / 2;
    let mid_y = height / 2;
    for y in 0..height {
        for x in 0..width {
            let base = (y * width + x) as usize * n_dims as usize;
            let (left, right) = if x < mid_x {
                (0.9, 0.2)
            } else if x > mid_x {
                (0.2, 0.9)
            } else {
                (0.55, 0.55)
            };
            values[base] = left;
            values[base + 1] = right;
            values[base + 2] = if x == mid_x && y == mid_y { 1.0 } else { 12.0 };
            // Admitted stall magnitude (Gu-Yang choke-class readout), not runner-up.
            values[base + 3] = if x == mid_x { 0.75 } else { 0.05 };
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

    let mut opt = base_request(width, height, n_dims, two_emitters(n_dims));
    opt.authored_opt_out_reason = Some("domain_suppresses_fronts");
    let opted = admit_comparative_projections(opt).expect("opt-out");
    assert_eq!(
        opted.disposition,
        ComparativeProjectionDisposition::AuthoredOptOut {
            reason: "domain_suppresses_fronts"
        }
    );

    for n in [2usize, 3, 8] {
        let emitters: Vec<_> = (0..n)
            .map(|i| ComparativeEmitterClass {
                class_id: (i as f32) + 1.0,
                value_col: col(i as u32, n_dims),
            })
            .collect();
        let mut req = base_request(width, height, n_dims, emitters);
        // emitters 0..n-1, palma=n, stall=n+1, outputs after
        req.palma_d_col = col(n as u32, n_dims);
        req.guyang_stall_col = col(n as u32 + 1, n_dims);
        let base = n as u32 + 2;
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
    }
}

#[test]
fn cpu_oracle_and_field_eml_agree_on_dominance_margin_and_stall_contest() {
    let width = 8u32;
    let height = 6u32;
    let n_dims = 9u32;
    let emitters = two_emitters(n_dims);
    let req = base_request(width, height, n_dims, emitters.clone());
    let outs = req.outputs;
    let bands = req.bands;
    let stall = req.guyang_stall_col;
    let adjacency = req.adjacency.clone();
    let bundle = admit_comparative_projections(req).expect("admit");
    let values = synthetic_values(width, height, n_dims);

    let oracle = comparative_projection_cpu_oracle(
        &values,
        width * height,
        n_dims,
        &emitters,
        outs,
        col(2, n_dims),
        stall,
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

    // Contest is stall magnitude where both-strong/small-margin, not runner-up.
    let contests = column(&eml, n_dims as usize, outs.contest_col.raw());
    let mid = (height / 2 * width + width / 2) as usize;
    // Mid column has equal emitters (margin 0) and stall 0.75.
    assert!(
        (contests[mid] - 0.75).abs() < 1e-6,
        "contest must carry stall magnitude under both-strong/small-margin, got {}",
        contests[mid]
    );

    // Exact top1−top2 is non-negative ⇒ sign-flip border arm is empty.
    // This is the load-bearing residual (Remand 1 item 4), not a silent proxy.
    let borders = column(&eml, n_dims as usize, outs.border_col.raw());
    assert!(
        borders.iter().all(|&b| b < 0.5),
        "sign-flip of non-negative top1-top2 margin must not fabricate borders; got {borders:?}"
    );
}

#[test]
fn cpu_gpu_bit_parity_for_comparative_projection_chain() {
    let Some(ctx) = gpu_context() else {
        eprintln!("skipping GPU parity — no adapter");
        return;
    };
    let width = 8u32;
    let height = 6u32;
    let n_dims = 9u32;
    let req = base_request(width, height, n_dims, two_emitters(n_dims));
    let outs = req.outputs;
    let bundle = admit_comparative_projections(req).expect("admit");
    let values = synthetic_values(width, height, n_dims);
    let cpu = execute_field_sweep_cpu_chain(&values, &bundle.registrations).expect("cpu");

    let mut session =
        FieldSweepSession::new(&ctx, &bundle.registrations[0]).expect("gpu session");
    session
        .upload_values(&ctx, &values)
        .expect("upload");
    session
        .dispatch_chain(&ctx, &bundle.registrations, 1)
        .expect("dispatch");
    let gpu = session.readback(&ctx).expect("readback");

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
    let n_dims = 9u32;
    let mut values = vec![0.0; (width * height * n_dims) as usize];
    values[0] = 1.0;
    values[1] = 1.0;
    values[2] = 9.0;
    values[3] = 0.0;
    values[n_dims as usize] = 0.2;
    values[n_dims as usize + 1] = 0.8;
    values[n_dims as usize + 2] = 9.0;
    values[n_dims as usize + 3] = 0.0;

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
    assert_eq!(out_ab[outs.dominance_col.raw()], 1.0);
    assert_eq!(out_ab[outs.margin_col.raw()], 0.0);

    let req_ba = base_request(width, height, n_dims, emitters_ba);
    let bundle_ba = admit_comparative_projections(req_ba).expect("ba");
    let out_ba = execute_field_sweep_cpu_chain(&values, &bundle_ba.registrations).expect("run ba");
    assert_eq!(out_ba[outs.dominance_col.raw()], 2.0);
}

#[test]
fn non_negative_top1_top2_margin_makes_sign_flip_border_unreachable() {
    // Remand 1 item 4 residual, kept as a falsifier — not a green proxy.
    let width = 4u32;
    let height = 3u32;
    let n_dims = 9u32;
    let req = base_request(width, height, n_dims, two_emitters(n_dims));
    let outs = req.outputs;
    let bundle = admit_comparative_projections(req).expect("admit");
    let values = synthetic_values(width, height, n_dims);
    let out = execute_field_sweep_cpu_chain(&values, &bundle.registrations).expect("run");
    let margins = column(&out, n_dims as usize, outs.margin_col.raw());
    assert!(
        margins.iter().all(|&m| m >= 0.0),
        "exact top1-top2 margin must be non-negative"
    );
    let borders = column(&out, n_dims as usize, outs.border_col.raw());
    assert!(
        borders.iter().all(|&b| b < 0.5),
        "sign-flip of non-negative margins cannot form a border band"
    );
    let chokes = column(&out, n_dims as usize, outs.chokepoint_col.raw());
    assert!(
        chokes.iter().all(|&c| c < 0.5),
        "chokepoint conjunction cannot fire without contested-border"
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
        req.guyang_stall_col = col(n + 1, n_dims);
        let base = n + 2;
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
    assert!(counts.iter().all(|&c| c == COMPARATIVE_DERIVED_COLUMN_COUNT));
}
