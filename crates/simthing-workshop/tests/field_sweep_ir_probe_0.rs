//! FIELD-SWEEP-IR-PROBE-0 — adapter-pinned parity + diagnostic measurement (test-only).
//!
//! Probe lives under `tests/support/` — not a workshop library export.
//! Absolute N4 bit-exact parity precedes timing. Inline synthetic only.

#[path = "support/field_sweep_ir_probe.rs"]
mod field_sweep_ir_probe;

use field_sweep_ir_probe::{
    bits_eq, build_gather, counter_surface_report, cpu_sweep_iters, cpu_sweep_once,
    format_degree_distribution, live_eml_cap_facts, max_ulp_diff, median_f64,
    planted_left_fold_stack_probe, program_banded_flux, program_metrics, program_min_x_input_list,
    program_product_conductance, threshold_adjudication_status, worst_f64, GatherTable,
    MeasurementRow, ProbeGpuSession, N4_OFFSETS_NSEW, N4_OFFSETS_WENS, N8_OFFSETS_THROWAWAY,
    RESOURCE_CLASS_LABEL, SAMPLE_RUNS, WARM_RUNS,
};
use simthing_gpu::{
    cpu_horizon, cpu_min_plus_d_from_w, extract_d_flat, pack_w_and_initial_d, params_from_config,
    GpuContext, MinPlusStencilConfig, MinPlusStencilOp, StructuredFieldStencilBoundaryMode,
    StructuredFieldStencilConfig, StructuredFieldStencilMaskMode, StructuredFieldStencilOp,
    StructuredFieldStencilOperator, StructuredFieldStencilSourcePolicy, MIN_PLUS_INF,
    SATURATING_FLUX_CHI_CFL_MAX,
};
use std::time::Instant;

fn require_gpu_ctx() -> Option<GpuContext> {
    match GpuContext::new_blocking() {
        Ok(ctx) => Some(ctx),
        Err(_) if std::env::var_os("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH").is_some() => {
            panic!("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH set but no GPU adapter");
        }
        Err(_) => None,
    }
}

fn require_probe_session() -> Option<ProbeGpuSession> {
    match ProbeGpuSession::new_blocking() {
        Ok(h) => Some(h),
        Err(_) if std::env::var_os("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH").is_some() => {
            panic!("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH set but probe session failed");
        }
        Err(_) => None,
    }
}

fn synthetic_w(width: u32, height: u32) -> Vec<f32> {
    let mut w = vec![1.0f32; (width * height) as usize];
    for y in 0..height {
        for x in 0..width {
            let i = (y * width + x) as usize;
            w[i] = 1.0 + ((x * 3 + y * 5) % 7) as f32 * 0.125;
        }
    }
    if width > 4 && height > 4 {
        for y in 1..height - 1 {
            w[(y * width + width / 2) as usize] = 4.0;
        }
    }
    w
}

fn synthetic_flux_values(width: u32, height: u32, n_dims: u32) -> Vec<f32> {
    let mut values = vec![0.0f32; (width * height * n_dims) as usize];
    let nd = n_dims as usize;
    let cx = (width / 2) as usize;
    let cy = (height / 2) as usize;
    values[(cy * width as usize + cx) * nd] = 0.8;
    values
}

fn time_bespoke_palma_matched(
    ctx: &GpuContext,
    w: &[f32],
    config: &MinPlusStencilConfig,
    iterations: u32,
    warm: usize,
    samples: usize,
) -> (Vec<f64>, Vec<f64>) {
    let values = pack_w_and_initial_d(w, config).expect("pack");
    let op = MinPlusStencilOp::new(ctx, config.clone()).expect("op");
    for _ in 0..warm {
        op.upload_values(ctx, &values).expect("upload");
        op.dispatch_ping_pong(ctx, iterations).expect("dispatch");
    }
    let mut dispatch_times = Vec::with_capacity(samples);
    let mut e2e_times = Vec::with_capacity(samples);
    for _ in 0..samples {
        let e2e0 = Instant::now();
        op.upload_values(ctx, &values).expect("upload");
        let d0 = Instant::now();
        op.dispatch_ping_pong(ctx, iterations).expect("dispatch");
        dispatch_times.push(d0.elapsed().as_secs_f64() * 1_000_000.0);
        e2e_times.push(e2e0.elapsed().as_secs_f64() * 1_000_000.0);
    }
    (dispatch_times, e2e_times)
}

fn time_bespoke_flux_matched(
    ctx: &GpuContext,
    values: &[f32],
    config: &StructuredFieldStencilConfig,
    warm: usize,
    samples: usize,
) -> (Vec<f64>, Vec<f64>) {
    let op = StructuredFieldStencilOp::new(ctx, config.clone()).expect("flux op");
    let hops = config.horizon;
    for _ in 0..warm {
        op.upload_values(ctx, values).expect("upload");
        op.dispatch_ping_pong(ctx, hops).expect("dispatch");
    }
    let mut dispatch_times = Vec::with_capacity(samples);
    let mut e2e_times = Vec::with_capacity(samples);
    for _ in 0..samples {
        let e2e0 = Instant::now();
        op.upload_values(ctx, values).expect("upload");
        let d0 = Instant::now();
        op.dispatch_ping_pong(ctx, hops).expect("dispatch");
        dispatch_times.push(d0.elapsed().as_secs_f64() * 1_000_000.0);
        e2e_times.push(e2e0.elapsed().as_secs_f64() * 1_000_000.0);
    }
    (dispatch_times, e2e_times)
}

fn row_from(
    case_name: &str,
    session: &ProbeGpuSession,
    gather: &GatherTable,
    theater: &str,
    metrics: &field_sweep_ir_probe::ProgramMetrics,
    path_kind: &str,
    dispatch_times: &[f64],
    e2e_times: &[f64],
    dispatch_count: u32,
    timing_note: &str,
) -> MeasurementRow {
    let (stall, status) = counter_surface_report(session.timestamp_supported);
    let d_med = median_f64(dispatch_times.to_vec());
    let d_worst = worst_f64(dispatch_times);
    let e_med = median_f64(e2e_times.to_vec());
    let e_worst = worst_f64(e2e_times);
    let edges = gather.edge_count as f64;
    MeasurementRow {
        case_name: case_name.to_string(),
        adapter_backend: format!("{} / {}", session.adapter_name, session.backend),
        adjacency_kind: gather.adjacency_kind.to_string(),
        theater_size: theater.to_string(),
        degree_distribution: format_degree_distribution(&gather.degree_histogram),
        map_nodes: metrics.map_nodes,
        fold_nodes: metrics.fold_nodes,
        post_nodes: metrics.post_nodes,
        actual_peak_operand_stack: metrics.actual_peak_operand_stack,
        configured_scratch_capacity: metrics.configured_scratch_capacity,
        column_reads_per_edge: metrics.column_reads_per_edge,
        resource_class: RESOURCE_CLASS_LABEL.to_string(),
        matched_occupancy: "UNMEASURED".to_string(),
        matched_work_basis: "same_theater_degree_edges_iterations_column_reads_NOT_occupancy"
            .to_string(),
        dispatch_count_per_sample: dispatch_count,
        warmup_count: WARM_RUNS,
        sample_count: SAMPLE_RUNS,
        dispatch_time_us_median: d_med,
        dispatch_time_us_worst: d_worst,
        e2e_time_us_median: e_med,
        e2e_time_us_worst: e_worst,
        edges_per_s_dispatch_median: if d_med > 0.0 {
            edges / (d_med / 1_000_000.0)
        } else {
            0.0
        },
        stall_memory_counters: stall,
        counter_surface_status: status,
        path_kind: path_kind.to_string(),
        timing_note: timing_note.to_string(),
    }
}

#[test]
fn field_sweep_ir_probe_0_n4_parity_absolute_before_timing() {
    let width = 16u32;
    let height = 16u32;
    let iterations = 8u32;
    let dest_x = 2u32;
    let dest_y = 2u32;
    let dest = dest_y * width + dest_x;

    let w = synthetic_w(width, height);
    let palma_cfg = MinPlusStencilConfig {
        width,
        height,
        n_dims: 2,
        d_col: 0,
        w_col: 1,
        dest_x,
        dest_y,
        inf_sentinel: MIN_PLUS_INF,
    };
    let bespoke_d = cpu_min_plus_d_from_w(&w, &palma_cfg, iterations).expect("bespoke d");
    let values0 = pack_w_and_initial_d(&w, &palma_cfg).expect("pack");
    let gather_palma = build_gather(width, height, &N4_OFFSETS_WENS, "GridN4_WENS");
    let prog_min = program_min_x_input_list(0, 1);
    let generic_vals = cpu_sweep_iters(
        &values0,
        width,
        height,
        2,
        0,
        &gather_palma,
        &prog_min,
        Some(dest),
        iterations,
    );
    let generic_d: Vec<f32> = (0..bespoke_d.len()).map(|i| generic_vals[i * 2]).collect();
    assert!(
        bits_eq(&bespoke_d, &generic_d),
        "MIN×INPUT_LIST must be bit-exact vs PALMA CPU; max_ulp={}",
        max_ulp_diff(&bespoke_d, &generic_d)
    );

    let (wn, ws, we, ww) = StructuredFieldStencilConfig::zero_directional_weights();
    let flux_cfg = StructuredFieldStencilConfig {
        width,
        height,
        n_dims: 2,
        source_col: 0,
        target_col: 0,
        horizon: 1,
        alpha_self: 0.0,
        gamma_neighbor: 0.0,
        weight_north: wn,
        weight_south: ws,
        weight_east: we,
        weight_west: ww,
        source_cap: None,
        operator: StructuredFieldStencilOperator::SaturatingFlux {
            u_sat: 1.0,
            chi: SATURATING_FLUX_CHI_CFL_MAX,
            choke_output_col: None,
        },
        source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
        boundary_mode: StructuredFieldStencilBoundaryMode::Clamp,
        mask_mode: StructuredFieldStencilMaskMode::All,
        allow_extended_horizon: false,
    };
    let flux_values = synthetic_flux_values(width, height, 2);
    let flux_params = params_from_config(&flux_cfg);
    let bespoke_flux = cpu_horizon(&flux_values, &flux_params, 1);

    let gather_flux = build_gather(width, height, &N4_OFFSETS_NSEW, "GridN4_NSEW");
    let prog_c = program_product_conductance(0, 1.0, SATURATING_FLUX_CHI_CFL_MAX);
    let after_c = cpu_sweep_once(
        &flux_values,
        width,
        height,
        2,
        1,
        &gather_flux,
        &prog_c,
        None,
    );
    let mut dual = after_c;
    for i in 0..(width * height) as usize {
        dual[i * 2] = flux_values[i * 2];
    }
    let prog_flux = program_banded_flux(0, 1);
    let generic_flux = cpu_sweep_once(&dual, width, height, 2, 0, &gather_flux, &prog_flux, None);

    let bespoke_u: Vec<f32> = (0..(width * height) as usize)
        .map(|i| bespoke_flux[i * 2])
        .collect();
    let generic_u: Vec<f32> = (0..(width * height) as usize)
        .map(|i| generic_flux[i * 2])
        .collect();
    assert!(
        bits_eq(&bespoke_u, &generic_u),
        "PRODUCT×INPUT_LIST+banded flux must be bit-exact vs Gu-Yang CPU; max_ulp={}",
        max_ulp_diff(&bespoke_u, &generic_u)
    );
}

#[test]
fn field_sweep_ir_probe_0_n8_throwaway_gather_cliff_and_caps() {
    // Planted left-fold: node count ≠ peak operand stack (postfix), and scratch ≠ stack.
    let (planted, node_count, peak) = planted_left_fold_stack_probe();
    assert!(
        node_count > peak,
        "planted left-fold must have node_count ({node_count}) > peak stack ({peak})"
    );
    assert_eq!(
        peak, 2,
        "left-fold ((((a+b)+c)+d)+e) peak operand stack is 2"
    );
    let planted_m = program_metrics(&planted);
    assert_eq!(planted_m.actual_peak_operand_stack, peak);
    assert_ne!(planted_m.actual_peak_operand_stack, planted_m.total_nodes);
    assert_eq!(planted_m.runtime_eval_model, "scratch_indexed_dag");
    assert_eq!(planted_m.configured_scratch_capacity, 32);

    let width = 12u32;
    let height = 12u32;
    let gather_n4 = build_gather(width, height, &N4_OFFSETS_NSEW, "GridN4_NSEW");
    let gather_n8 = build_gather(width, height, &N8_OFFSETS_THROWAWAY, "WorkshopThrowawayN8");
    assert!(gather_n8.edge_count > gather_n4.edge_count);
    assert_eq!(gather_n8.offsets.len(), 8);

    let prog = program_banded_flux(0, 1);
    let m = program_metrics(&prog);
    let per_tree = m.map_nodes.max(m.fold_nodes).max(m.post_nodes);
    let caps = live_eml_cap_facts(per_tree, m.total_nodes, m.actual_peak_operand_stack);
    assert_eq!(caps.configured_max_tree_nodes, 32);
    assert_eq!(caps.configured_stack_max, 32);
    assert_eq!(
        caps.observed_max_tree_nodes, 9,
        "Gu-Yang-shaped map tree is 9 nodes"
    );
    assert_eq!(
        caps.observed_total_program_nodes, 13,
        "map+fold+post composition only"
    );
    assert!(caps.observed_max_tree_nodes <= caps.configured_max_tree_nodes);
    assert!(caps.observed_peak_operand_stack <= caps.configured_stack_max);
    assert_ne!(m.actual_peak_operand_stack, m.total_nodes);

    let edge_ratio = gather_n8.edge_count as f64 / gather_n4.edge_count as f64;
    assert!(
        edge_ratio > 1.5,
        "N8 cliff not located: edge_ratio={edge_ratio}"
    );
}

#[test]
fn field_sweep_ir_probe_0_adapter_pinned_measurement() {
    let Some(ctx) = require_gpu_ctx() else {
        eprintln!("skipping field_sweep_ir_probe_0_adapter_pinned_measurement: no GPU");
        return;
    };
    let Some(mut session) = require_probe_session() else {
        eprintln!("skipping measurement: probe session unavailable");
        return;
    };

    let width = 32u32;
    let height = 32u32;
    let iterations = 4u32;
    let dest_x = 1u32;
    let dest_y = 1u32;
    let dest = dest_y * width + dest_x;
    let theater = format!("{width}x{height}");

    let w = synthetic_w(width, height);
    let palma_cfg = MinPlusStencilConfig {
        width,
        height,
        n_dims: 2,
        d_col: 0,
        w_col: 1,
        dest_x,
        dest_y,
        inf_sentinel: MIN_PLUS_INF,
    };
    let values0 = pack_w_and_initial_d(&w, &palma_cfg).expect("pack");
    let gather_palma = build_gather(width, height, &N4_OFFSETS_WENS, "GridN4_WENS");
    let prog_min = program_min_x_input_list(0, 1);
    let m_min = program_metrics(&prog_min);

    // Absolute GPU parity before admitting timing.
    let op = MinPlusStencilOp::new(&ctx, palma_cfg.clone()).expect("palma op");
    op.upload_values(&ctx, &values0).expect("upload");
    let gpu_bespoke_vals = op.run_ping_pong(&ctx, iterations).expect("bespoke gpu");
    let bespoke_d = extract_d_flat(&gpu_bespoke_vals, &palma_cfg).expect("extract");

    session
        .configure(width * height, 2, &gather_palma, &prog_min)
        .expect("configure");
    session.upload_values(&values0).expect("upload");
    session
        .dispatch_iters(&prog_min, 0, Some(dest), iterations)
        .expect("dispatch");
    let generic_gpu = session.readback().expect("readback");
    let generic_d: Vec<f32> = (0..bespoke_d.len()).map(|i| generic_gpu[i * 2]).collect();
    assert!(
        bits_eq(&bespoke_d, &generic_d),
        "GPU MIN×INPUT_LIST must be bit-exact vs PALMA GPU; max_ulp={}",
        max_ulp_diff(&bespoke_d, &generic_d)
    );

    let (wn, ws, we, ww) = StructuredFieldStencilConfig::zero_directional_weights();
    let flux_cfg = StructuredFieldStencilConfig {
        width,
        height,
        n_dims: 2,
        source_col: 0,
        target_col: 0,
        horizon: 1,
        alpha_self: 0.0,
        gamma_neighbor: 0.0,
        weight_north: wn,
        weight_south: ws,
        weight_east: we,
        weight_west: ww,
        source_cap: None,
        operator: StructuredFieldStencilOperator::SaturatingFlux {
            u_sat: 1.0,
            chi: SATURATING_FLUX_CHI_CFL_MAX,
            choke_output_col: None,
        },
        source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
        boundary_mode: StructuredFieldStencilBoundaryMode::Clamp,
        mask_mode: StructuredFieldStencilMaskMode::All,
        allow_extended_horizon: false,
    };
    let flux_values = synthetic_flux_values(width, height, 2);
    let gather_flux = build_gather(width, height, &N4_OFFSETS_NSEW, "GridN4_NSEW");
    let prog_c = program_product_conductance(0, 1.0, SATURATING_FLUX_CHI_CFL_MAX);
    let prog_flux = program_banded_flux(0, 1);
    let m_flux = program_metrics(&prog_flux);

    let flux_op = StructuredFieldStencilOp::new(&ctx, flux_cfg.clone()).expect("flux");
    flux_op.upload_values(&ctx, &flux_values).expect("upload");
    let (bespoke_flux_gpu, _) = flux_op.run_configured_horizon(&ctx).expect("flux gpu");

    session
        .configure(width * height, 2, &gather_flux, &prog_flux)
        .expect("configure flux");
    session.upload_values(&flux_values).expect("upload");
    session
        .dispatch_c_then_flux(&prog_c, &prog_flux, true)
        .expect("c+flux");
    let generic_flux_gpu = session.readback().expect("readback");
    let bu: Vec<f32> = (0..(width * height) as usize)
        .map(|i| bespoke_flux_gpu[i * 2])
        .collect();
    let gu: Vec<f32> = (0..(width * height) as usize)
        .map(|i| generic_flux_gpu[i * 2])
        .collect();
    assert!(
        bits_eq(&bu, &gu),
        "GPU PRODUCT+flux must be bit-exact vs Gu-Yang GPU; max_ulp={}",
        max_ulp_diff(&bu, &gu)
    );

    // Matched-envelope diagnostic timing (persistent buffers; upload+dispatch).
    // Occupancy remains UNMEASURED — no threshold ROUTE-SPECIALIZATION/JIT claim.
    session
        .configure(width * height, 2, &gather_palma, &prog_min)
        .expect("reconfigure palma");
    let (gen_palma_d, gen_palma_e2e) = session
        .time_dispatch_us(
            &values0,
            &prog_min,
            0,
            Some(dest),
            iterations,
            WARM_RUNS,
            SAMPLE_RUNS,
        )
        .expect("time generic palma");
    let (bes_palma_d, bes_palma_e2e) =
        time_bespoke_palma_matched(&ctx, &w, &palma_cfg, iterations, WARM_RUNS, SAMPLE_RUNS);

    session
        .configure(width * height, 2, &gather_flux, &prog_flux)
        .expect("reconfigure flux");
    let (gen_flux_d, gen_flux_e2e) = session
        .time_c_then_flux_us(&flux_values, &prog_c, &prog_flux, WARM_RUNS, SAMPLE_RUNS)
        .expect("time generic flux");
    let (bes_flux_d, bes_flux_e2e) =
        time_bespoke_flux_matched(&ctx, &flux_values, &flux_cfg, WARM_RUNS, SAMPLE_RUNS);

    // N8 cliff diagnostic (generic only).
    let gather_n8 = build_gather(width, height, &N8_OFFSETS_THROWAWAY, "WorkshopThrowawayN8");
    // Build dual on GPU via C then time flux-only on N8 would need C first — for cliff,
    // time one flux dispatch over N8 gather on precomputed dual from CPU (setup excluded).
    let after_c_cpu = cpu_sweep_once(
        &flux_values,
        width,
        height,
        2,
        1,
        &gather_flux,
        &prog_c,
        None,
    );
    let mut dual = after_c_cpu;
    for i in 0..(width * height) as usize {
        dual[i * 2] = flux_values[i * 2];
    }
    session
        .configure(width * height, 2, &gather_n8, &prog_flux)
        .expect("n8 configure");
    let (n8_d, n8_e2e) = session
        .time_dispatch_us(&dual, &prog_flux, 0, None, 1, WARM_RUNS, SAMPLE_RUNS)
        .expect("n8 time");

    let palma_note =
        "matched_envelope: persistent op/session; timed upload+GPU-resident dispatch; no per-iter realloc/readback; occupancy UNMEASURED";
    let flux_note =
        "matched_envelope: persistent ops; timed upload+GPU-resident dispatch (no readback); generic=2 dispatches (C then flux, no CPU merge); bespoke=1 horizon dispatch; counts published; occupancy UNMEASURED";

    let rows = vec![
        row_from(
            "min_x_input_list_n4_bespoke",
            &session,
            &gather_palma,
            &theater,
            &m_min,
            "bespoke_palma",
            &bes_palma_d,
            &bes_palma_e2e,
            iterations,
            palma_note,
        ),
        row_from(
            "min_x_input_list_n4_generic",
            &session,
            &gather_palma,
            &theater,
            &m_min,
            "generic_ir",
            &gen_palma_d,
            &gen_palma_e2e,
            iterations,
            palma_note,
        ),
        row_from(
            "product_banded_flux_n4_bespoke",
            &session,
            &gather_flux,
            &theater,
            &m_flux,
            "bespoke_guyang",
            &bes_flux_d,
            &bes_flux_e2e,
            1,
            flux_note,
        ),
        row_from(
            "product_banded_flux_n4_generic",
            &session,
            &gather_flux,
            &theater,
            &m_flux,
            "generic_ir",
            &gen_flux_d,
            &gen_flux_e2e,
            2,
            flux_note,
        ),
        row_from(
            "product_banded_flux_n8_generic_cliff",
            &session,
            &gather_n8,
            &theater,
            &m_flux,
            "generic_ir_n8_throwaway",
            &n8_d,
            &n8_e2e,
            1,
            "N8 cliff diagnostic; occupancy UNMEASURED",
        ),
    ];

    assert!(rows.iter().all(|r| r.matched_occupancy == "UNMEASURED"));
    assert!(rows
        .iter()
        .all(|r| r.counter_surface_status.starts_with("STOP(")));

    let adjudication = threshold_adjudication_status(false, true);
    assert_eq!(
        adjudication,
        "DIAGNOSTIC_ONLY(occupancy_UNMEASURED;no_threshold_verdict)"
    );

    let per_tree_min = m_min.map_nodes.max(m_min.fold_nodes).max(m_min.post_nodes);
    let per_tree_flux = m_flux
        .map_nodes
        .max(m_flux.fold_nodes)
        .max(m_flux.post_nodes);
    let caps = live_eml_cap_facts(
        per_tree_min.max(per_tree_flux),
        m_min.total_nodes.max(m_flux.total_nodes),
        m_min
            .actual_peak_operand_stack
            .max(m_flux.actual_peak_operand_stack),
    );
    assert_eq!(caps.observed_max_tree_nodes, 9);
    assert_eq!(caps.observed_total_program_nodes, 13);

    eprintln!("FIELD-SWEEP-IR-PROBE-0 Remand-2 complete measurement rows");
    eprintln!(
        "adapter={} backend={}",
        session.adapter_name, session.backend
    );
    eprintln!(
        "caps configured_max_tree_nodes={} configured_stack_max={} observed_max_tree_nodes={} observed_total_program_nodes={} peak_operand_stack={} scratch_cap={} class={}",
        caps.configured_max_tree_nodes,
        caps.configured_stack_max,
        caps.observed_max_tree_nodes,
        caps.observed_total_program_nodes,
        caps.observed_peak_operand_stack,
        caps.probe_scratch_capacity,
        caps.resource_class_label
    );
    eprintln!("threshold_adjudication={adjudication}");
    eprintln!(
        "diagnostic_ratios palma_dispatch_med={:.3} flux_dispatch_med={:.3} (NOT threshold verdict)",
        median_f64(gen_palma_d.clone()) / median_f64(bes_palma_d.clone()).max(1e-9),
        median_f64(gen_flux_d.clone()) / median_f64(bes_flux_d.clone()).max(1e-9)
    );
    eprintln!(
        "n8_cliff edges n4={} n8={}",
        gather_flux.edge_count, gather_n8.edge_count
    );
    for r in &rows {
        eprintln!("{}", r.to_tsv_line());
    }
}
