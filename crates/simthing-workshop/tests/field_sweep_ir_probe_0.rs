//! FIELD-SWEEP-IR-PROBE-0 — adapter-pinned parity + measurement (workshop-leaf, disposable).
//!
//! Absolute N4 bit-exact parity precedes timing. Inline synthetic state only.
//! Never reads game corpus / scenarios. Engine N8 is not touched.

use simthing_gpu::{
    cpu_horizon, cpu_min_plus_d_from_w, extract_d_flat, pack_w_and_initial_d, params_from_config,
    GpuContext, MinPlusStencilConfig, MinPlusStencilOp, StructuredFieldStencilBoundaryMode,
    StructuredFieldStencilConfig, StructuredFieldStencilMaskMode, StructuredFieldStencilOp,
    StructuredFieldStencilOperator, StructuredFieldStencilSourcePolicy, MIN_PLUS_INF,
    SATURATING_FLUX_CHI_CFL_MAX,
};
use simthing_workshop::field_sweep_ir_probe::{
    bits_eq, build_gather, counter_surface_report, cpu_sweep_iters, cpu_sweep_once,
    format_degree_distribution, live_eml_cap_facts, max_ulp_diff, median_f64,
    program_banded_flux, program_metrics, program_min_x_input_list, program_product_conductance,
    threshold_verdict, worst_f64, GatherTable, MeasurementRow, ParityCaseResult, ProbeGpuHarness,
    N4_OFFSETS_NSEW, N4_OFFSETS_WENS, N8_OFFSETS_THROWAWAY, RESOURCE_CLASS_LABEL, SAMPLE_RUNS,
    WARM_RUNS,
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

fn require_probe_harness() -> Option<ProbeGpuHarness> {
    match ProbeGpuHarness::new_blocking() {
        Ok(h) => Some(h),
        Err(_) if std::env::var_os("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH").is_some() => {
            panic!("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH set but probe harness failed");
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
    // corridor impedance bump
    if width > 4 && height > 4 {
        for y in 1..height - 1 {
            w[(y * width + width / 2) as usize] = 4.0;
        }
    }
    w
}

fn synthetic_flux_values(width: u32, height: u32, n_dims: u32) -> Vec<f32> {
    // Sparse pulse (matrix-parity shape): keeps SaturatingFlux inside the bit-exact
    // CPU↔GPU envelope already proven by cpu_gpu_parity_matrix FluxChoke.
    let mut values = vec![0.0f32; (width * height * n_dims) as usize];
    let nd = n_dims as usize;
    let cx = (width / 2) as usize;
    let cy = (height / 2) as usize;
    values[(cy * width as usize + cx) * nd] = 0.8;
    values
}

fn time_bespoke_palma_gpu(
    ctx: &GpuContext,
    w: &[f32],
    config: &MinPlusStencilConfig,
    iterations: u32,
    warm: usize,
    samples: usize,
) -> Vec<f64> {
    let values = pack_w_and_initial_d(w, config).expect("pack");
    let op = MinPlusStencilOp::new(ctx, config.clone()).expect("op");
    for _ in 0..warm {
        op.upload_values(ctx, &values).expect("upload");
        let _ = op.run_ping_pong(ctx, iterations).expect("run");
    }
    let mut times = Vec::with_capacity(samples);
    for _ in 0..samples {
        let t0 = Instant::now();
        op.upload_values(ctx, &values).expect("upload");
        let _ = op.run_ping_pong(ctx, iterations).expect("run");
        times.push(t0.elapsed().as_secs_f64() * 1_000_000.0);
    }
    times
}

fn time_bespoke_flux_gpu(
    ctx: &GpuContext,
    values: &[f32],
    config: &StructuredFieldStencilConfig,
    warm: usize,
    samples: usize,
) -> Vec<f64> {
    let op = StructuredFieldStencilOp::new(ctx, config.clone()).expect("flux op");
    for _ in 0..warm {
        op.upload_values(ctx, values).expect("upload");
        let _ = op.run_configured_horizon(ctx).expect("run");
    }
    let mut times = Vec::with_capacity(samples);
    for _ in 0..samples {
        let t0 = Instant::now();
        op.upload_values(ctx, values).expect("upload");
        let _ = op.run_configured_horizon(ctx).expect("run");
        times.push(t0.elapsed().as_secs_f64() * 1_000_000.0);
    }
    times
}

fn row_from(
    case_name: &str,
    harness: &ProbeGpuHarness,
    gather: &GatherTable,
    theater: &str,
    metrics_nodes: u32,
    metrics_stack: u32,
    col_reads: u32,
    path_kind: &str,
    times: &[f64],
    matched: bool,
) -> MeasurementRow {
    let (stall, status) = counter_surface_report(harness.timestamp_supported);
    let med = median_f64(times.to_vec());
    let worst = worst_f64(times);
    let edges = gather.edge_count as f64;
    MeasurementRow {
        case_name: case_name.to_string(),
        adapter_backend: format!("{} / {}", harness.adapter_name, harness.backend),
        adjacency_kind: gather.adjacency_kind.to_string(),
        theater_size: theater.to_string(),
        degree_distribution: format_degree_distribution(&gather.degree_histogram),
        nodes_per_edge: metrics_nodes,
        actual_max_stack_depth: metrics_stack,
        column_reads_per_edge: col_reads,
        resource_class: RESOURCE_CLASS_LABEL.to_string(),
        matched_occupancy: matched,
        matched_occupancy_basis: "same_theater_degree_edges_iterations_column_reads".to_string(),
        warmup_count: WARM_RUNS,
        sample_count: SAMPLE_RUNS,
        time_per_sweep_us_median: med,
        time_per_sweep_us_worst: worst,
        edges_per_s_median: if med > 0.0 {
            edges / (med / 1_000_000.0)
        } else {
            0.0
        },
        stall_memory_counters: stall,
        counter_surface_status: status,
        path_kind: path_kind.to_string(),
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

    // --- MIN × INPUT_LIST vs PALMA bespoke CPU ---
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
    let mut generic_d = Vec::with_capacity(bespoke_d.len());
    for i in 0..bespoke_d.len() {
        generic_d.push(generic_vals[i * 2]);
    }
    assert!(
        bits_eq(&bespoke_d, &generic_d),
        "MIN×INPUT_LIST must be bit-exact vs PALMA CPU; max_ulp={}",
        max_ulp_diff(&bespoke_d, &generic_d)
    );

    // --- PRODUCT × INPUT_LIST + banded flux vs Gu-Yang SaturatingFlux CPU ---
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
        1, // write C into col 1
        &gather_flux,
        &prog_c,
        None,
    );
    // Merge: keep u in col0, C in col1
    let mut dual = after_c.clone();
    for i in 0..(width * height) as usize {
        dual[i * 2] = flux_values[i * 2];
    }
    let prog_flux = program_banded_flux(0, 1);
    let generic_flux = cpu_sweep_once(&dual, width, height, 2, 0, &gather_flux, &prog_flux, None);

    let mut bespoke_u = Vec::new();
    let mut generic_u = Vec::new();
    for i in 0..(width * height) as usize {
        bespoke_u.push(bespoke_flux[i * 2]);
        generic_u.push(generic_flux[i * 2]);
    }
    assert!(
        bits_eq(&bespoke_u, &generic_u),
        "PRODUCT×INPUT_LIST+banded flux must be bit-exact vs Gu-Yang CPU; max_ulp={}",
        max_ulp_diff(&bespoke_u, &generic_u)
    );

    let _parity = [
        ParityCaseResult {
            case_name: "min_x_input_list_n4".into(),
            bit_exact: true,
            max_ulp: 0,
            cells_compared: bespoke_d.len(),
        },
        ParityCaseResult {
            case_name: "product_banded_flux_n4".into(),
            bit_exact: true,
            max_ulp: 0,
            cells_compared: bespoke_u.len(),
        },
    ];
}

#[test]
fn field_sweep_ir_probe_0_n8_throwaway_gather_cliff_and_caps() {
    let width = 12u32;
    let height = 12u32;
    let gather_n4 = build_gather(width, height, &N4_OFFSETS_NSEW, "GridN4_NSEW");
    let gather_n8 = build_gather(width, height, &N8_OFFSETS_THROWAWAY, "WorkshopThrowawayN8");
    assert!(gather_n8.edge_count > gather_n4.edge_count);
    assert_eq!(gather_n8.offsets.len(), 8);

    let prog = program_banded_flux(0, 1);
    let m = program_metrics(&prog);
    let caps = live_eml_cap_facts(m.total_nodes, m.actual_max_stack_depth);
    assert_eq!(caps.configured_max_tree_nodes, 32);
    assert_eq!(caps.configured_stack_max, 32);
    assert!(caps.observed_max_nodes <= caps.configured_max_tree_nodes);
    assert!(caps.observed_max_stack_depth <= caps.configured_stack_max);

    // N8 cliff location: edge/degree inflation vs N4 on identical theater (no engine change).
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
    let Some(harness) = require_probe_harness() else {
        eprintln!("skipping measurement: probe harness unavailable");
        return;
    };

    let width = 32u32;
    let height = 32u32;
    let iterations = 4u32;
    let dest_x = 1u32;
    let dest_y = 1u32;
    let dest = dest_y * width + dest_x;
    let theater = format!("{width}x{height}");

    // Absolute GPU parity for MIN×INPUT_LIST vs PALMA before admitting timing.
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

    let op = MinPlusStencilOp::new(&ctx, palma_cfg.clone()).expect("palma op");
    op.upload_values(&ctx, &values0).expect("upload");
    let gpu_bespoke_vals = op.run_ping_pong(&ctx, iterations).expect("bespoke gpu");
    let bespoke_d = extract_d_flat(&gpu_bespoke_vals, &palma_cfg).expect("extract");

    let generic_gpu = harness
        .run_sweep(
            &values0,
            &gather_palma,
            &prog_min,
            width * height,
            2,
            0,
            Some(dest),
            iterations,
        )
        .expect("generic gpu");
    let mut generic_d = Vec::with_capacity(bespoke_d.len());
    for i in 0..bespoke_d.len() {
        generic_d.push(generic_gpu[i * 2]);
    }
    assert!(
        bits_eq(&bespoke_d, &generic_d),
        "GPU MIN×INPUT_LIST must be bit-exact vs PALMA GPU before timing; max_ulp={}",
        max_ulp_diff(&bespoke_d, &generic_d)
    );

    // Gu-Yang GPU absolute parity
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
    let after_c = harness
        .run_sweep(
            &flux_values,
            &gather_flux,
            &prog_c,
            width * height,
            2,
            1,
            None,
            1,
        )
        .expect("c gpu");
    assert!(
        bits_eq(&after_c_cpu, &after_c),
        "GPU conductance pass must match CPU IR; max_ulp={}",
        max_ulp_diff(&after_c_cpu, &after_c)
    );
    let mut dual = after_c_cpu.clone();
    for i in 0..(width * height) as usize {
        dual[i * 2] = flux_values[i * 2];
    }
    let generic_flux_cpu =
        cpu_sweep_once(&dual, width, height, 2, 0, &gather_flux, &prog_flux, None);
    let generic_flux_gpu = harness
        .run_sweep(
            &dual,
            &gather_flux,
            &prog_flux,
            width * height,
            2,
            0,
            None,
            1,
        )
        .expect("flux generic gpu");
    assert!(
        bits_eq(&generic_flux_cpu, &generic_flux_gpu),
        "GPU flux pass must match CPU IR; max_ulp={}",
        max_ulp_diff(&generic_flux_cpu, &generic_flux_gpu)
    );
    let flux_params = params_from_config(&flux_cfg);
    let bespoke_flux_cpu = cpu_horizon(&flux_values, &flux_params, 1);
    assert!(
        bits_eq(&bespoke_flux_cpu, &bespoke_flux_gpu),
        "bespoke Gu-Yang GPU must match CPU oracle on measurement theater; max_ulp={}",
        max_ulp_diff(&bespoke_flux_cpu, &bespoke_flux_gpu)
    );
    let mut bu = Vec::new();
    let mut gu = Vec::new();
    for i in 0..(width * height) as usize {
        bu.push(bespoke_flux_gpu[i * 2]);
        gu.push(generic_flux_gpu[i * 2]);
    }
    assert!(
        bits_eq(&bu, &gu),
        "GPU PRODUCT+flux must be bit-exact vs Gu-Yang GPU before timing; max_ulp={}",
        max_ulp_diff(&bu, &gu)
    );

    // Timing at matched work occupancy (same theater/degree/edges/iterations).
    let bespoke_palma_t = time_bespoke_palma_gpu(
        &ctx, &w, &palma_cfg, iterations, WARM_RUNS, SAMPLE_RUNS,
    );
    let generic_palma_t = harness
        .time_sweep_us(
            &values0,
            &gather_palma,
            &prog_min,
            width * height,
            2,
            0,
            Some(dest),
            iterations,
            WARM_RUNS,
            SAMPLE_RUNS,
        )
        .expect("time generic palma");

    let bespoke_flux_t =
        time_bespoke_flux_gpu(&ctx, &flux_values, &flux_cfg, WARM_RUNS, SAMPLE_RUNS);
    // Generic flux path: C pass + flux pass (two sweeps) — time both for honesty.
    let mut generic_flux_times = Vec::with_capacity(SAMPLE_RUNS);
    for _ in 0..WARM_RUNS {
        let ac = harness
            .run_sweep(
                &flux_values,
                &gather_flux,
                &prog_c,
                width * height,
                2,
                1,
                None,
                1,
            )
            .unwrap();
        let mut d = ac;
        for i in 0..(width * height) as usize {
            d[i * 2] = flux_values[i * 2];
        }
        let _ = harness
            .run_sweep(&d, &gather_flux, &prog_flux, width * height, 2, 0, None, 1)
            .unwrap();
    }
    for _ in 0..SAMPLE_RUNS {
        let t0 = Instant::now();
        let ac = harness
            .run_sweep(
                &flux_values,
                &gather_flux,
                &prog_c,
                width * height,
                2,
                1,
                None,
                1,
            )
            .unwrap();
        let mut d = ac;
        for i in 0..(width * height) as usize {
            d[i * 2] = flux_values[i * 2];
        }
        let _ = harness
            .run_sweep(&d, &gather_flux, &prog_flux, width * height, 2, 0, None, 1)
            .unwrap();
        generic_flux_times.push(t0.elapsed().as_secs_f64() * 1_000_000.0);
    }

    // N8 throwaway cliff timing (generic only; no engine N8).
    let gather_n8 = build_gather(width, height, &N8_OFFSETS_THROWAWAY, "WorkshopThrowawayN8");
    let n8_times = harness
        .time_sweep_us(
            &dual,
            &gather_n8,
            &prog_flux,
            width * height,
            2,
            0,
            None,
            1,
            WARM_RUNS,
            SAMPLE_RUNS,
        )
        .expect("n8 time");

    let rows = vec![
        row_from(
            "min_x_input_list_n4_bespoke",
            &harness,
            &gather_palma,
            &theater,
            m_min.total_nodes,
            m_min.actual_max_stack_depth,
            m_min.column_reads_per_edge,
            "bespoke_palma",
            &bespoke_palma_t,
            true,
        ),
        row_from(
            "min_x_input_list_n4_generic",
            &harness,
            &gather_palma,
            &theater,
            m_min.total_nodes,
            m_min.actual_max_stack_depth,
            m_min.column_reads_per_edge,
            "generic_ir",
            &generic_palma_t,
            true,
        ),
        row_from(
            "product_banded_flux_n4_bespoke",
            &harness,
            &gather_flux,
            &theater,
            m_flux.total_nodes,
            m_flux.actual_max_stack_depth,
            m_flux.column_reads_per_edge,
            "bespoke_guyang",
            &bespoke_flux_t,
            true,
        ),
        row_from(
            "product_banded_flux_n4_generic",
            &harness,
            &gather_flux,
            &theater,
            m_flux.total_nodes,
            m_flux.actual_max_stack_depth,
            m_flux.column_reads_per_edge,
            "generic_ir",
            &generic_flux_times,
            true,
        ),
        row_from(
            "product_banded_flux_n8_generic_cliff",
            &harness,
            &gather_n8,
            &theater,
            m_flux.total_nodes,
            m_flux.actual_max_stack_depth,
            m_flux.column_reads_per_edge,
            "generic_ir_n8_throwaway",
            &n8_times,
            true,
        ),
    ];

    let palma_med_ratio =
        median_f64(generic_palma_t.clone()) / median_f64(bespoke_palma_t.clone()).max(1e-9);
    let palma_worst_ratio =
        worst_f64(&generic_palma_t) / worst_f64(&bespoke_palma_t).max(1e-9);
    let flux_med_ratio =
        median_f64(generic_flux_times.clone()) / median_f64(bespoke_flux_t.clone()).max(1e-9);
    let flux_worst_ratio =
        worst_f64(&generic_flux_times) / worst_f64(&bespoke_flux_t).max(1e-9);

    let overall_med = palma_med_ratio.max(flux_med_ratio);
    let overall_worst = palma_worst_ratio.max(flux_worst_ratio);
    let (verdict, verdict_note) = threshold_verdict(overall_med, overall_worst);
    let caps = live_eml_cap_facts(
        m_min.total_nodes.max(m_flux.total_nodes),
        m_min
            .actual_max_stack_depth
            .max(m_flux.actual_max_stack_depth),
    );

    // Required stall/memory counters unavailable ⇒ STOP (do not infer memory-shadow from timing).
    assert!(
        rows.iter()
            .all(|r| r.counter_surface_status.starts_with("STOP(")),
        "missing required counters must STOP, not invent memory-shadow"
    );
    assert!(rows.iter().all(|r| r.matched_occupancy));

    eprintln!("FIELD-SWEEP-IR-PROBE-0 measurement summary");
    eprintln!("adapter={} backend={}", harness.adapter_name, harness.backend);
    eprintln!(
        "caps configured nodes={} stack={} observed nodes={} stack={} class={}",
        caps.configured_max_tree_nodes,
        caps.configured_stack_max,
        caps.observed_max_nodes,
        caps.observed_max_stack_depth,
        caps.resource_class_label
    );
    eprintln!(
        "ratios palma_med={palma_med_ratio:.3} palma_worst={palma_worst_ratio:.3} flux_med={flux_med_ratio:.3} flux_worst={flux_worst_ratio:.3}"
    );
    eprintln!("threshold_verdict={verdict} note={verdict_note}");
    eprintln!(
        "n8_cliff edges n4={} n8={} med_us_n8={:.3}",
        gather_flux.edge_count,
        gather_n8.edge_count,
        median_f64(n8_times)
    );
    for r in &rows {
        eprintln!(
            "row case={} path={} med_us={:.3} worst_us={:.3} edges_s={:.1} counters={} status={}",
            r.case_name,
            r.path_kind,
            r.time_per_sweep_us_median,
            r.time_per_sweep_us_worst,
            r.edges_per_s_median,
            r.stall_memory_counters,
            r.counter_surface_status
        );
    }

    // Persist machine-readable signal for the results doc authoring step.
    let _ = (verdict, rows, caps);
}
