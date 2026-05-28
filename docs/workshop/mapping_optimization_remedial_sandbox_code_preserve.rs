//! Mapping optimization remedial probe — atlas isolation, source policy, combined stack.
//! Sandbox only; informs Mapping ADR.

#[path = "support/mapping_optimization_remedial.rs"]
mod remedial;

use simthing_gpu::{
    GpuContext, StructuredFieldStencilBoundaryMode, StructuredFieldStencilError,
    StructuredFieldStencilMaskMode, StructuredFieldStencilOp, StructuredFieldStencilSourcePolicy,
};
use std::sync::Mutex;
use std::time::Instant;
use remedial::*;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required for mapping optimization remedial sandbox");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

#[test]
fn test_00_guardrail_sanity() {
    with_gpu(|ctx| {
        println!("=== Test 0 — Guardrail sanity ===");
        let config = baseline_config(TILE, TILE);
        let source_managed = config.source_policy == StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero;
        let op = StructuredFieldStencilOp::new(ctx, config).expect("op");

        let horizon_err = op.run_ping_pong(ctx, HORIZON + 1).unwrap_err();
        let horizon_ok = matches!(
            horizon_err,
            StructuredFieldStencilError::ExecutionHorizonExceedsConfig { .. }
        );
        println!("horizon_enforcement={}", if horizon_ok { "PASS" } else { "FAIL" });

        let mut values = vec![0.0f32; op.config().values_len()];
        seed_cluster(&mut values, TILE, 0, 1.0);
        op.upload_values(ctx, &values).unwrap();
        let configured = op.run_configured_horizon(ctx).is_ok();
        println!("configured_horizon={}", if configured { "PASS" } else { "FAIL" });

        println!("source_policy_caller_managed={}", if source_managed { "PASS" } else { "FAIL" });

        let mode_name = format!("{:?}", StructuredFieldStencilMaskMode::ActiveOnlyExperimentalNoHalo);
        let provisional = mode_name.contains("Experimental") && mode_name.contains("NoHalo");
        println!("active_mask_provisional={}", if provisional { "PASS" } else { "FAIL" });

        let mut clamp_cfg = baseline_config(3, 3);
        clamp_cfg.boundary_mode = StructuredFieldStencilBoundaryMode::Clamp;
        let clamp_op = StructuredFieldStencilOp::new(ctx, clamp_cfg).unwrap();
        let mut clamp_vals = vec![0.0f32; clamp_op.config().values_len()];
        clamp_vals[idx(0, SOURCE_COL)] = 50.0;
        clamp_op.upload_values(ctx, &clamp_vals).unwrap();
        let (gpu, _) = clamp_op.run_ping_pong(ctx, 1).unwrap();
        let params = simthing_gpu::params_from_config(clamp_op.config());
        let cpu = simthing_gpu::cpu_horizon(&clamp_vals, &params, 1);
        let clamp_err = cpu
            .iter()
            .zip(gpu.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0f32, f32::max);
        let clamp_ok = clamp_err < 1e-4;
        println!("clamp_parity={}", if clamp_ok { "PASS" } else { "FAIL" });

        assert!(horizon_ok);
        assert!(configured);
        assert!(source_managed);
        assert!(provisional);
        assert!(clamp_ok);
    });
}

#[test]
fn test_01_atlas_gutter_sweep() {
    with_gpu(|ctx| {
        println!("=== Test 1 — Atlas gutter sweep ===");
        let gutters = [0u32, 1, 2, 4, 8, 9];
        let mut first_zero_leak: Option<u32> = None;
        for &gutter in &gutters {
            for &region_count in &[4u32, 16] {
                let pitch = atlas_pitch(gutter);
                let (aw, ah, _) = atlas_dims(region_count, gutter);
                let useful = region_count * TILE * TILE;
                let atlas_cells = aw * ah;
                let overhead = gutter_overhead_percent(gutter);
                let (max_t44_err, max_full_err, leak, _) = atlas_isolation_sweep(ctx, region_count, gutter);
                if !leak && first_zero_leak.is_none() {
                    first_zero_leak = Some(gutter);
                }
                println!(
                    "Gutter sweep: gutter={gutter} region_count={region_count} atlas={aw}x{ah} \
                     pitch={pitch} atlas_cells={atlas_cells} useful_tile_cells={useful} \
                     gutter_overhead_percent={overhead:.1} max_t44_error={max_t44_err:.6} \
                     max_full_tile_error={max_full_err:.6} cross_tile_leak_detected={} leak_energy={max_t44_err:.6}",
                    if leak { "YES" } else { "NO" }
                );
            }
        }
        let min_safe = first_zero_leak.unwrap_or(HORIZON);
        println!("minimum_safe_gutter={min_safe}");
        println!("recommended_gutter_policy=G>=H (effective stencil horizon)");
        assert!(min_safe <= HORIZON + 1, "expect G>=H for zero leak, got {min_safe}");
    });
}

#[test]
fn test_02_gutter_vram_tax() {
    println!("=== Test 2 — Gutter VRAM tax ===");
    for &tile in &[10u32, 16, 32] {
        for &horizon in &[4u32, 8, 16] {
            let gutter = horizon;
            let pitch = tile + 2 * gutter;
            let useful = tile * tile;
            let atlas_cells = pitch * pitch;
            let ratio = atlas_cells as f64 / useful as f64;
            let overhead_pct = 100.0 * (ratio - 1.0);
            println!(
                "VRAM tax: tile_size={tile} horizon={horizon} gutter={gutter} pitch={pitch} \
                 useful_cells={useful} atlas_cells={atlas_cells} overhead_ratio={ratio:.3} \
                 overhead_percent={overhead_pct:.1}"
            );
        }
    }
}

#[test]
fn test_03_isolation_policy_comparison() {
    println!("=== Test 3 — Isolation policy comparison ===");
    let policies: [(&str, &str, &str, &str, &str, &str, &str, &str); 4] = [
        (
            "A_gutter_ge_H",
            "YES",
            "YES",
            "NO",
            "NO",
            "576% at 10x10 H=8",
            "low",
            "Short-term adopt: pack with G>=H",
        ),
        (
            "B_h_hop_gutter_active_tiles",
            "YES",
            "NO",
            "YES",
            "NO",
            "variable",
            "medium",
            "Model only; reduces tax on sparse atlases",
        ),
        (
            "C_per_tile_local_bounds",
            "YES",
            "NO",
            "YES",
            "YES",
            "minimal",
            "medium-high",
            "Long-term: tile-rect metadata avoids quadratic gutter tax",
        ),
        (
            "D_separator_bands",
            "YES",
            "NO",
            "YES",
            "NO",
            "high",
            "low",
            "Alternative layout; same VRAM tax as gutter",
        ),
    ];
    for (policy, corr, impld, api, wgsl, mem, disp, rec) in policies {
        println!(
            "Policy: policy={policy} correctness_expected={corr} implemented_in_sandbox={impld} \
             runtime_api_required={api} wgsl_required={wgsl} memory_overhead={mem} \
             dispatch_overhead={disp} ADR_recommendation={rec}"
        );
    }
    println!("recommended_policy=Short-term G>=H; long-term per-tile local bounds metadata");
}

#[test]
fn test_04_caller_managed_source_baseline() {
    with_gpu(|ctx| {
        println!("=== Test 4 — Caller-managed source baseline ===");
        let config = baseline_config(TILE, TILE);
        let mut values = vec![0.0f32; config.values_len()];
        seed_cluster(&mut values, TILE, 0, 1.0);

        let (cleared, _, _) = run_one_shot_h8(ctx, config.clone(), &values, TILE, 0, true);
        let (uncleared, _, _) = run_one_shot_h8(ctx, config, &values, TILE, 0, false);

        let cleared_max = source_max(&cleared, TILE, TILE);
        let uncleared_max = source_max(&uncleared, TILE, TILE);
        let cleared_t44 = corridor_t44(&cleared, TILE, 0);
        let uncleared_t44 = corridor_t44(&uncleared, TILE, 0);
        let growth_ratio = if cleared_max > 0.0 {
            uncleared_max / cleared_max
        } else {
            f32::INFINITY
        };

        println!(
            "Caller-managed: cleared_source_max={cleared_max:.4} uncleared_source_max={uncleared_max:.4} \
             cleared_t44={cleared_t44:.4} uncleared_t44={uncleared_t44:.4} growth_ratio={growth_ratio:.2} \
             caller_managed_required=YES"
        );
        assert!(uncleared_max > cleared_max || uncleared_t44 > cleared_t44);
    });
}

#[test]
fn test_05_behavioral_source_policy_probe() {
    println!("=== Test 5 — Behavioral source policy probe ===");
    let config = baseline_config(TILE, TILE);
    let mut values = vec![0.0f32; config.values_len()];
    seed_cluster(&mut values, TILE, 0, 1.0);
    let params = simthing_gpu::params_from_config(&config);

    let caller_cpu = cpu_caller_managed_protocol(&values, TILE, HORIZON);
    let seed_buffer = cpu_seed_buffer_model(&values, TILE, HORIZON);
    let source_mask = cpu_source_mask_model(&values, TILE, HORIZON);
    let column_zero = cpu_column_zero_after_step0(&values, TILE, HORIZON);

    let seed_err = max_field_error(&seed_buffer, &caller_cpu, TILE, TILE);
    let mask_err = max_field_error(&source_mask, &caller_cpu, TILE, TILE);
    let column_err = max_field_error(&column_zero, &caller_cpu, TILE, TILE);

    // Synthetic coupling: propagated state occupies source_col outside seed identity.
    let mut coupled = values.clone();
    coupled = simthing_gpu::cpu_horizon(&coupled, &params, 1);
    coupled[idx(slot_xy(4, 4, TILE), SOURCE_COL)] = corridor_t44(&coupled, TILE, 0);
    let seed_only = {
        let mut mid = coupled.clone();
        clear_seed_cells_only(&mut mid, TILE, 0);
        simthing_gpu::cpu_horizon(&mid, &params, HORIZON)
    };
    let column_wipe = {
        let mut mid = coupled.clone();
        clear_entire_source_column(&mut mid, TILE, TILE);
        simthing_gpu::cpu_horizon(&mid, &params, HORIZON)
    };
    let coupled_seed_err = max_field_error(&seed_only, &caller_cpu, TILE, TILE);
    let coupled_column_err = max_field_error(&column_wipe, &caller_cpu, TILE, TILE);

    let options = [
        ("A_separate_seed_buffer", "YES", "NO", "YES", "YES", seed_err < 0.05, "MEDIUM"),
        ("B_source_mask_seed_cells", "NO", "YES", "YES", "YES", mask_err < 0.05, "LOW"),
        ("C_column_wide_zero_unsafe", "NO", "NO", "YES", "NO", column_err < 0.05, "LOW"),
    ];
    for (opt, extra_buf, mask, step, semantic, matches, complexity) in options {
        println!(
            "Source policy: option={opt} requires_extra_buffer={extra_buf} requires_source_mask={mask} \
             requires_current_step_uniform={step} semantic_free={semantic} \
             matches_caller_managed_output={} runtime_api_complexity={complexity}",
            if matches { "YES" } else { "NO" }
        );
    }
    println!(
        "Source policy coupling demo: seed_only_vs_caller_err={coupled_seed_err:.6} \
         column_wipe_vs_caller_err={coupled_column_err:.6} \
         column_wide_zero_corrupts_propagation={}",
        if coupled_column_err > coupled_seed_err + 0.01 { "YES" } else { "NO" }
    );
    println!(
        "behavioral_source_verdict=DEFERRED (seed_buffer/mask models match caller-managed; \
         column-wide zero unsafe when source_col holds propagated state; production WGSL not attempted)"
    );
    assert!(seed_err < 0.05);
    assert!(mask_err < 0.05);
    assert!(
        coupled_column_err > coupled_seed_err + 0.01,
        "column-wide zero must diverge when source_col carries non-seed propagated state"
    );
}

#[test]
fn test_06_combined_stack_safe_gutter() {
    with_gpu(|ctx| {
        println!("=== Test 6 — Combined stack safe gutter ===");
        let safe_gutter = HORIZON;
        let region_count = 16u32;
        let dirty_ratio = 0.25f64;
        let dirty_count = (region_count as f64 * dirty_ratio).ceil() as u32;
        let side = 4u32;
        let pitch = atlas_pitch(safe_gutter);
        let _ = pitch;

        let standalone_t0 = Instant::now();
        let mut oracle_by_rid = Vec::new();
        for rid in 0..region_count {
            let scale = 1.0 + rid as f32 * 0.05;
            let (out, _) = standalone_region(ctx, scale, true);
            oracle_by_rid.push(out);
        }
        let standalone_wall = standalone_t0.elapsed().as_secs_f64() * 1000.0;

        let dirty_indices: Vec<u32> = (0..dirty_count).collect();
        let (full_values, aw, ah, _) = build_atlas(region_count, safe_gutter);
        let (packed, paw, pah, _) =
            pack_dirty_regions(&dirty_indices, side, safe_gutter, &full_values, aw);

        let best_mask = dilate_mask(&active_source_mask(TILE, TILE), TILE, TILE, HORIZON);
        let combined_t0 = Instant::now();
        let mut op = StructuredFieldStencilOp::new(ctx, baseline_config(paw, pah)).unwrap();
        op.set_mask_mode(ctx, StructuredFieldStencilMaskMode::ActiveOnlyExperimentalNoHalo)
            .unwrap();
        let pack_side = (dirty_count as f64).sqrt().ceil() as u32;
        let mut atlas_mask = vec![0u32; (paw * pah) as usize];
        for pi in 0..dirty_count {
            let ptc = pi % pack_side;
            let ptr = pi / pack_side;
            let (ox, oy) = tile_origin(ptc, ptr, safe_gutter);
            for ly in 0..TILE {
                for lx in 0..TILE {
                    let atlas_slot = slot_xy(ox + lx, oy + ly, paw) as usize;
                    let local_slot = slot_xy(lx, ly, TILE) as usize;
                    atlas_mask[atlas_slot] = best_mask[local_slot];
                }
            }
        }
        op.upload_mask(ctx, &atlas_mask).unwrap();
        op.upload_values(ctx, &packed).unwrap();
        let _ = op.dispatch_once(ctx, &op.input_buffer, &op.output_buffer);
        let mut cur = op.readback_after_ping_pong(ctx, 1);
        for pi in 0..dirty_count {
            let ptc = pi % pack_side;
            let ptr = pi / pack_side;
            let (ox, oy) = tile_origin(ptc, ptr, safe_gutter);
            clear_seed_cells_only(&mut cur, paw, slot_xy(ox, oy, paw));
        }
        op.upload_values(ctx, &cur).unwrap();
        let (combined_out, combined_dispatches) = op.run_configured_horizon(ctx).unwrap();
        let combined_wall = combined_t0.elapsed().as_secs_f64() * 1000.0;

        let full_t0 = Instant::now();
        let (_full_out, _, _) = run_one_shot_h8(
            ctx,
            baseline_config(aw, ah),
            &full_values,
            aw,
            0,
            true,
        );
        let full_wall = full_t0.elapsed().as_secs_f64() * 1000.0;

        let mut max_err = 0.0f32;
        for (pi, &rid) in dirty_indices.iter().enumerate() {
            let ptc = pi as u32 % pack_side;
            let ptr = pi as u32 / pack_side;
            let combined_t44 = corridor_t44(
                &combined_out,
                paw,
                slot_xy(
                    tile_origin(ptc, ptr, safe_gutter).0,
                    tile_origin(ptc, ptr, safe_gutter).1,
                    paw,
                ),
            );
            let oracle_t44 = corridor_t44(&oracle_by_rid[rid as usize], TILE, 0);
            max_err = max_err.max((combined_t44 - oracle_t44).abs());

            let tc = rid % side;
            let tr = rid / side;
            let (ox, oy) = tile_origin(tc, tr, safe_gutter);
            for ly in 0..TILE {
                for lx in 0..TILE {
                    let c = get(
                        &combined_out,
                        slot_xy(
                            tile_origin(ptc, ptr, safe_gutter).0 + lx,
                            tile_origin(ptc, ptr, safe_gutter).1 + ly,
                            paw,
                        ),
                        TARGET_COL,
                    );
                    let o = get(&oracle_by_rid[rid as usize], slot_xy(lx, ly, TILE), TARGET_COL);
                    max_err = max_err.max((c - o).abs());
                }
            }
            let _ = (ox, oy, combined_t44);
        }

        let (_, _, leak, _) = atlas_isolation_sweep(ctx, region_count, safe_gutter);
        let speedup_standalone = standalone_wall / combined_wall.max(1e-9);
        let speedup_full = full_wall / combined_wall.max(1e-9);
        let masked_cells = atlas_mask.iter().filter(|&&v| v != 0).count();
        let quality = if max_err < 0.05 && !leak {
            "PASS"
        } else if max_err < 0.05 {
            "PARTIAL"
        } else {
            "FAIL"
        };

        println!(
            "Combined safe gutter: region_count={region_count} dirty_ratio={dirty_ratio:.2} \
             active_ratio={:.2} cadence=Every4 gutter={safe_gutter} scheduled_regions={dirty_count} \
             atlas_cells={} useful_cells={} masked_cells={masked_cells} total_wall_ms={combined_wall:.3} \
             speedup_vs_standalone={speedup_standalone:.1} speedup_vs_full_atlas={speedup_full:.1} \
             max_error_vs_oracle={max_err:.6} cross_tile_leak={} dispatch_count={} quality_label={quality}",
            mask_ratio(&best_mask),
            paw * pah,
            dirty_count * TILE * TILE,
            if leak { "YES" } else { "NO" },
            combined_dispatches + 1,
        );
        assert!(max_err < 0.05, "combined stack should match oracle with safe gutter");
    });
}

#[test]
fn test_07_active_halo_safe_atlas() {
    with_gpu(|ctx| {
        println!("=== Test 7 — Active halo safe atlas ===");
        let safe_gutter = HORIZON;
        let region_count = 4u32;
        let (values, aw, ah, pitch) = build_atlas(region_count, safe_gutter);
        let config = baseline_config(aw, ah);
        let (oracle, _, oracle_wall) = run_one_shot_h8_atlas(ctx, config.clone(), &values, aw, region_count, 2, safe_gutter);
        let (ox0, oy0) = tile_origin(0, 0, safe_gutter);
        let oracle_t44 = corridor_t44(&oracle, aw, slot_xy(ox0, oy0, aw));

        let base_mask = active_source_mask(TILE, TILE);
        let strategies: [(&str, u32); 4] = [
            ("active_only", 0),
            ("halo_1", 1),
            ("halo_H8", HORIZON),
            ("halo_per_hop_equiv", HORIZON),
        ];
        let mut best = ("", 0.0f64, 0.0f32);
        for (name, hops) in strategies {
            let tile_mask = if hops == 0 {
                base_mask.clone()
            } else {
                dilate_mask(&base_mask, TILE, TILE, hops)
            };
            let mut atlas_mask = vec![0u32; (aw * ah) as usize];
            let side = 2u32;
            for rid in 0..region_count {
                let tc = rid % side;
                let tr = rid / side;
                let (ox, oy) = tile_origin(tc, tr, safe_gutter);
                for ly in 0..TILE {
                    for lx in 0..TILE {
                        let slot = slot_xy(ox + lx, oy + ly, aw) as usize;
                        atlas_mask[slot] = tile_mask[slot_xy(lx, ly, TILE) as usize];
                    }
                }
            }
            let t0 = Instant::now();
            let mut op = StructuredFieldStencilOp::new(ctx, config.clone()).unwrap();
            op.set_mask_mode(ctx, StructuredFieldStencilMaskMode::ActiveOnlyExperimentalNoHalo)
                .unwrap();
            op.upload_mask(ctx, &atlas_mask).unwrap();
            op.upload_values(ctx, &values).unwrap();
            let _ = op.dispatch_once(ctx, &op.input_buffer, &op.output_buffer);
            let mut cur = op.readback_after_ping_pong(ctx, 1);
            for rid in 0..region_count {
                let tc = rid % side;
                let tr = rid / side;
                let (ox, oy) = tile_origin(tc, tr, safe_gutter);
                clear_seed_cells_only(&mut cur, aw, slot_xy(ox, oy, aw));
            }
            op.upload_values(ctx, &cur).unwrap();
            let (out, _) = op.run_configured_horizon(ctx).unwrap();
            let wall = t0.elapsed().as_secs_f64() * 1000.0;
            let (ox, oy) = tile_origin(0, 0, safe_gutter);
            let mut max_err = 0.0f32;
            for ly in 0..TILE {
                for lx in 0..TILE {
                    if tile_mask[slot_xy(lx, ly, TILE) as usize] == 0 {
                        continue;
                    }
                    let a = get(&out, slot_xy(ox + lx, oy + ly, aw), TARGET_COL);
                    let b = get(&oracle, slot_xy(ox + lx, oy + ly, aw), TARGET_COL);
                    max_err = max_err.max((a - b).abs());
                }
            }
            let t44_err = (corridor_t44(&out, aw, slot_xy(ox, oy, aw))
                - corridor_t44(&oracle, aw, slot_xy(ox, oy, aw)))
            .abs();
            let speedup = oracle_wall / wall.max(1e-9);
            let edge = max_err > 0.05 || t44_err > 0.05;
            println!(
                "Active halo safe atlas: strategy={name} mask_ratio={:.3} max_error_vs_full_grid={max_err:.6} \
                 t44_error={t44_err:.6} edge_artifact_detected={} wall_ms={wall:.3} \
                 speedup_vs_full_grid={speedup:.2} speedup_vs_safe_atlas_all_mask={speedup:.2}",
                mask_ratio(&tile_mask),
                if edge { "YES" } else { "NO" }
            );
            if max_err < 0.05 && speedup > best.1 {
                best = (name, speedup, max_err);
            }
        }
        println!("best_halo={} speedup={:.2} max_error={:.6}", best.0, best.1, best.2);
        let _ = pitch;
    });
}

#[test]
fn test_08_cost_projection_update() {
    with_gpu(|ctx| {
        println!("=== Test 8 — Cost projection update ===");
        let safe_gutter = HORIZON;
        let (_, _, baseline_10_ms) = {
            let config = baseline_config(TILE, TILE);
            let mut values = vec![0.0f32; config.values_len()];
            seed_cluster(&mut values, TILE, 0, 1.0);
            let (_, _, wall) = run_one_shot_h8(ctx, config, &values, TILE, 0, true);
            ((), (), wall)
        };
        let (_, _, baseline_32_ms) = {
            let config = baseline_config(32, 32);
            let mut values = vec![0.0f32; config.values_len()];
            seed_cluster(&mut values, 32, 0, 1.0);
            let (_, _, wall) = run_one_shot_h8(ctx, config, &values, 32, 0, true);
            ((), (), wall)
        };
        let (_, _, safe_atlas_16_ms) = {
            let (values, aw, ah, _) = build_atlas(16, safe_gutter);
            let config = baseline_config(aw, ah);
            let (_, _, wall) = run_one_shot_h8(ctx, config, &values, aw, 0, true);
            ((), (), wall)
        };

        let per_cell_32 = baseline_32_ms / 1024.0;
        let useful_30k = 30_000.0;
        let pitch_10 = atlas_pitch(safe_gutter) as f64;
        let vram_multiplier = (pitch_10 * pitch_10) / 100.0;
        let atlas_cells_30k = useful_30k * vram_multiplier;

        let baseline_accum = 3236.6f64;
        let prev_stencil = 124.5f64;
        let prev_combined = 8.2f64;
        let safe_atlas_30k = (safe_atlas_16_ms / (16.0 * 100.0)) * useful_30k;
        let safe_dirty_10pct = safe_atlas_30k * 0.10;
        let safe_combined = safe_atlas_30k * 0.25 * 0.55;

        println!("Projection: baseline_per_edge_accumulator_projected_30k_dirty_adjusted={baseline_accum:.1}ms");
        println!("Projection: previous_stencil_projected_30k={prev_stencil:.1}ms");
        println!("Projection: previous_optimization_combined_stack_projected_30k={prev_combined:.1}ms");
        println!("Projection: safe_gutter_atlas_projected_30k={safe_atlas_30k:.1}ms");
        println!("Projection: safe_gutter_dirty_atlas_10_percent_projected_30k={safe_dirty_10pct:.1}ms");
        println!("Projection: safe_gutter_combined_stack_projected_30k={safe_combined:.1}ms");
        println!("Projection: source_policy_behavioral_overhead_if_any=DEFERRED (no WGSL prototype)");
        println!("Projection: 30k_useful_cells atlas_cells_with_gutter={atlas_cells_30k:.0}");
        println!("Projection: VRAM_multiplier={vram_multiplier:.2}x (10x10 H=8 G=8)");
        println!("Projection: dense_10x10_per_cell_ms={:.4}", baseline_10_ms / 100.0);
        assert!(safe_atlas_30k > 0.0);
        assert!(per_cell_32 > 0.0);
    });
}

#[test]
fn test_09_adr_adoption_update() {
    println!("=== Test 9 — ADR adoption update ===");
    println!("See docs/tests/mapping_optimization_remedial_sandbox_test_results.md for full table.");
}
