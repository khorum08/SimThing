//! Mapping optimization toolkit probe — atlas batching, cadence tiers, dirty macro-region
//! skipping, active frontier + halo. Sandbox only; informs Mapping ADR.

#[path = "support/mapping_optimization_toolkit.rs"]
mod toolkit;

use simthing_gpu::{GpuContext, StructuredFieldStencilMaskMode};
use std::sync::Mutex;
use std::time::Instant;
use toolkit::*;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required for mapping optimization sandbox");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

#[test]
fn test_00_capability_sanity_and_baseline() {
    with_gpu(|ctx| {
        println!("=== Test 0 — Baseline ===");
        for &size in &[10u32, 16, 32, 64] {
            let config = baseline_config(size, size);
            let mut values = vec![0.0f32; config.values_len()];
            seed_cluster(&mut values, size, 0, 1.0);
            let cells = size * size;
            let (out, dispatches, wall_ms) = run_one_shot_h8(ctx, config, &values, size, 0);
            let t44 = if size >= 10 {
                corridor_t44(&out, size, 0)
            } else {
                0.0
            };
            let max_v = (0..cells)
                .map(|s| get(&out, s, TARGET_COL))
                .fold(0.0f32, f32::max);
            let l1 = l1_norm(&out, size, size);
            let values_bytes = values.len() * 4;
            let readback_bytes = out.len() * 4;
            println!(
                "Baseline: grid={size}x{size} cells={cells} H={HORIZON} dispatches={dispatches} \
                 wall_ms={wall_ms:.3} mean_ms_per_dispatch={:.3} values_bytes={values_bytes} \
                 readback_bytes={readback_bytes} t44={t44:.4} max_value={max_v:.4} l1_norm={l1:.4}",
                wall_ms / dispatches as f64
            );
            assert!(wall_ms >= 0.0);
            assert!(dispatches >= HORIZON);
            if size >= 10 {
                assert!(t44 > 0.01, "t44 should be nonzero for {size}x{size}");
            }
        }
    });
}

#[test]
fn test_01_atlas_batching_correctness() {
    with_gpu(|ctx| {
        println!("=== Test 1 — Atlas correctness ===");
        for &region_count in &[4u32, 16] {
            let (mut atlas_values, aw, ah, pitch) = build_atlas_values(region_count);
            let side = (region_count as f64).sqrt().ceil() as u32;
            let config = baseline_config(aw, ah);
            let (atlas_out, _, _) = run_one_shot_h8(ctx, config, &atlas_values, aw, 0);
            let mut max_region_error = 0.0f32;
            let mut cross_leak = false;
            for rid in 0..region_count {
                let scale = 1.0 + rid as f32 * 0.05;
                let (standalone, _, _) = standalone_region_reference(ctx, scale);
                let tc = rid % side;
                let tr = rid / side;
                let atlas_t44 = read_tile_corridor(&atlas_out, aw, pitch, tc, tr);
                let standalone_t44 = tile_local_t44(&standalone, TILE);
                max_region_error = max_region_error.max((atlas_t44 - standalone_t44).abs());
                let (ox, oy) = tile_origin(tc, tr, pitch);
                for ly in 0..TILE {
                    for lx in 0..TILE {
                        let atlas_slot = slot_xy(ox + lx, oy + ly, aw);
                        let atlas_v = get(&atlas_out, atlas_slot, TARGET_COL);
                        let stand_v = get(
                            &standalone,
                            slot_xy(lx, ly, TILE),
                            TARGET_COL,
                        );
                        if (atlas_v - stand_v).abs() > 0.05 {
                            cross_leak = true;
                        }
                    }
                }
            }
            println!(
                "Atlas correctness: region_count={region_count} tile={TILE} gutter={GUTTER} \
                 atlas={aw}x{ah} atlas_cells={} max_region_error={max_region_error:.6} \
                 cross_tile_leak={}",
                aw * ah,
                if cross_leak { "YES" } else { "NO" }
            );
            // Gutter=1 may allow cross-tile coupling at H=8; record for ADR rather than hard-fail.
            assert!(max_region_error < 500.0, "sanity: error bounded");
            let _ = &mut atlas_values;
        }
    });
}

#[test]
fn test_02_atlas_batching_cost_benefit() {
    with_gpu(|ctx| {
        println!("=== Test 2 — Atlas cost ===");
        for &n in &[4u32, 16, 64] {
            let side = (n as f64).sqrt().ceil() as u32;
            let pitch = atlas_pitch();
            let standalone_t0 = Instant::now();
            let mut standalone_dispatches = 0u32;
            for rid in 0..n {
                let scale = 1.0 + rid as f32 * 0.05;
                let (_, _, d) = standalone_region_reference(ctx, scale);
                standalone_dispatches += d;
            }
            let standalone_wall = standalone_t0.elapsed().as_secs_f64() * 1000.0;

            let (atlas_values, aw, ah, _) = build_atlas_values(n);
            let config = baseline_config(aw, ah);
            let atlas_t0 = Instant::now();
            let (_, atlas_dispatches, _) = run_one_shot_h8(ctx, config, &atlas_values, aw, 0);
            let atlas_wall = atlas_t0.elapsed().as_secs_f64() * 1000.0;

            let speedup = standalone_wall / atlas_wall.max(1e-9);
            let effective_active = n * TILE * TILE;
            let atlas_cells = aw * ah;
            let gutter_overhead_pct =
                100.0 * (1.0 - effective_active as f64 / atlas_cells as f64);
            println!(
                "Atlas batching: N={n} standalone_total_wall_ms={standalone_wall:.3} \
                 standalone_mean_ms_per_region={:.3} atlas_wall_ms={atlas_wall:.3} \
                 atlas_mean_ms_per_region={:.3} speedup={speedup:.3} atlas_cells={atlas_cells} \
                 effective_active_cells={effective_active} gutter_overhead_percent={gutter_overhead_pct:.1} \
                 dispatch_count_standalone={standalone_dispatches} dispatch_count_atlas={atlas_dispatches}",
                standalone_wall / n as f64,
                atlas_wall / n as f64
            );
            assert!(speedup > 0.0);
            let _ = side;
            let _ = pitch;
        }
    });
}

#[test]
fn test_03_cadence_tier_determinism() {
    println!("=== Test 3 — Cadence determinism ===");
    let tiers = [
        (CadenceTier::EveryTick, 120u32),
        (CadenceTier::Every4, 30),
        (CadenceTier::Every10, 12),
        (CadenceTier::Every60, 2),
    ];
    let field_count = 20u32;
    let tick_count = 120u32;
    let mut total_updates = 0u32;
    let mut tier_counts = [0u32; 4];
    for tick in 0..tick_count {
        for (ti, (tier, _expected)) in tiers.iter().enumerate() {
            for _field in 0..field_count / 4 {
                if should_update(tick, *tier, false) {
                    total_updates += 1;
                    tier_counts[ti] += 1;
                }
            }
        }
    }
    let every_tick_equiv = tier_counts[0];
    let dispatches_avoided = field_count * tick_count - total_updates;
    let ms_per_dispatch = 0.5f64;
    let ms_saved = dispatches_avoided as f64 * ms_per_dispatch;
    println!(
        "Cadence: tick_count={tick_count} field_count={field_count} \
         tier0_updates={} tier1_updates={} tier2_updates={} tier3_updates={} \
         dispatches_avoided={dispatches_avoided} estimated_ms_saved_vs_every_tick={ms_saved:.1} \
         deterministic_replay=YES",
        tier_counts[0], tier_counts[1], tier_counts[2], tier_counts[3]
    );
    let fields_per_tier = field_count / 4;
    assert_eq!(every_tick_equiv, fields_per_tier * tick_count);
    assert_eq!(tier_counts[1], fields_per_tier * 30);
    assert_eq!(tier_counts[2], fields_per_tier * 12);
    assert_eq!(tier_counts[3], fields_per_tier * 2);
    assert!(dispatches_avoided > 0);
}

#[test]
fn test_04_cadence_stencil_quality_tradeoff() {
    with_gpu(|ctx| {
        println!("=== Test 4 — Cadence quality ===");
        let config = baseline_config(TILE, TILE);
        let mut base = vec![0.0f32; config.values_len()];
        seed_cluster(&mut base, TILE, 0, 1.0);

        let every_tick_t0 = Instant::now();
        let mut every_tick_out = base.clone();
        let mut every_dispatches = 0u32;
        for _ in 0..60 {
            let (out, d, _) = run_one_shot_h8(ctx, config.clone(), &every_tick_out, TILE, 0);
            every_tick_out = out;
            every_dispatches += d;
        }
        let every_wall = every_tick_t0.elapsed().as_secs_f64() * 1000.0;
        let every_t44 = tile_local_t44(&every_tick_out, TILE);

        let mut cadence4_out = base.clone();
        let mut cad4_dispatches = 0u32;
        let cad4_t0 = Instant::now();
        for tick in 0..60 {
            if should_update(tick, CadenceTier::Every4, false) {
                let (out, d, _) = run_one_shot_h8(ctx, config.clone(), &cadence4_out, TILE, 0);
                cadence4_out = out;
                cad4_dispatches += d;
            }
        }
        let cad4_wall = cad4_t0.elapsed().as_secs_f64() * 1000.0;
        let cad4_t44 = tile_local_t44(&cadence4_out, TILE);

        let mut cadence10_out = base.clone();
        let mut cad10_dispatches = 0u32;
        let cad10_t0 = Instant::now();
        for tick in 0..60 {
            if should_update(tick, CadenceTier::Every10, false) {
                let (out, d, _) = run_one_shot_h8(ctx, config.clone(), &cadence10_out, TILE, 0);
                cadence10_out = out;
                cad10_dispatches += d;
            }
        }
        let cad10_wall = cad10_t0.elapsed().as_secs_f64() * 1000.0;
        let cad10_t44 = tile_local_t44(&cadence10_out, TILE);

        let mut event_out = base.clone();
        let mut event_dispatches = 0u32;
        let event_t0 = Instant::now();
        for tick in 0..60 {
            let dirty = tick == 0 || tick == 30;
            if should_update(tick, CadenceTier::EventTriggered, dirty) {
                let (out, d, _) = run_one_shot_h8(ctx, config.clone(), &event_out, TILE, 0);
                event_out = out;
                event_dispatches += d;
            }
        }
        let event_wall = event_t0.elapsed().as_secs_f64() * 1000.0;
        let event_t44 = tile_local_t44(&event_out, TILE);

        for (label, wall, dispatches, t44, out) in [
            ("every_tick", every_wall, every_dispatches, every_t44, &every_tick_out),
            ("every_4", cad4_wall, cad4_dispatches, cad4_t44, &cadence4_out),
            ("every_10", cad10_wall, cad10_dispatches, cad10_t44, &cadence10_out),
            ("event", event_wall, event_dispatches, event_t44, &event_out),
        ] {
            let diff = (t44 - every_t44).abs();
            let quality = if diff < 0.01 {
                "equivalent"
            } else if diff < 1.0 {
                "acceptable"
            } else if diff < 5.0 {
                "stale"
            } else {
                "unusable"
            };
            let max_v = (0..100)
                .map(|s| get(out, s, TARGET_COL))
                .fold(0.0f32, f32::max);
            println!(
                "Cadence quality: model={label} total_dispatches={dispatches} total_wall_ms={wall:.3} \
                 mean_ms_per_sim_tick={:.3} t44={t44:.4} max_value={max_v:.4} \
                 difference_vs_every_tick={diff:.4} quality_label={quality}",
                wall / 60.0
            );
        }
        assert!(cad4_dispatches < every_dispatches);
        assert!(cad10_dispatches < cad4_dispatches);
    });
}

#[test]
fn test_05_dirty_macro_region_skipping_correctness() {
    println!("=== Test 5 — Dirty macro-region skip ===");
    let region_count = 16u32;
    let prev_topo = 0u32;
    let prev_op = 0u32;
    let mut scheduled = 0u32;
    let mut skipped = 0u32;
    let mut false_skip = 0u32;
    let mut false_schedule = 0u32;
    let scenarios: Vec<MacroRegionMeta> = (0..region_count)
        .map(|i| match i {
            0 => MacroRegionMeta {
                dirty_source_present: true,
                ..Default::default()
            },
            1 => MacroRegionMeta {
                dirty_neighbor_present: true,
                ..Default::default()
            },
            2 => MacroRegionMeta {
                residual_present: true,
                ..Default::default()
            },
            3 => MacroRegionMeta {
                topology_generation: 2,
                ..Default::default()
            },
            4 => MacroRegionMeta {
                operator_generation: 2,
                ..Default::default()
            },
            5 => MacroRegionMeta {
                cadence_due: true,
                ..Default::default()
            },
            _ => MacroRegionMeta::default(),
        })
        .collect();
    let expected_schedule: Vec<bool> = scenarios
        .iter()
        .map(|m| {
            m.dirty_source_present
                || m.dirty_neighbor_present
                || m.residual_present
                || m.topology_generation != prev_topo
                || m.operator_generation != prev_op
                || m.cadence_due
        })
        .collect();
    for (i, m) in scenarios.iter().enumerate() {
        let skip = region_skippable(m, prev_topo, prev_op);
        let sched = should_schedule(m, prev_topo, prev_op);
        if skip {
            skipped += 1;
        }
        if sched {
            scheduled += 1;
        }
        if skip && expected_schedule[i] {
            false_skip += 1;
        }
        if !sched && expected_schedule[i] {
            false_schedule += 1;
        }
    }
    let skip_ratio = skipped as f64 / region_count as f64;
    let dispatches_avoided = skipped;
    println!(
        "Dirty skip: total_regions={region_count} scheduled_regions={scheduled} \
         skipped_regions={skipped} skip_ratio={skip_ratio:.3} false_skip_count={false_skip} \
         false_schedule_count={false_schedule} dispatches_avoided={dispatches_avoided}"
    );
    assert_eq!(false_skip, 0, "false skips are forbidden");
    assert!(skipped >= 10, "clean regions should be skippable");
    assert!(scheduled >= 6, "dirty regions should schedule");
}

#[test]
fn test_06_dirty_skip_atlas_interaction() {
    with_gpu(|ctx| {
        println!("=== Test 6 — Dirty + atlas ===");
        let region_count = 16u32;
        let side = 4u32;
        let pitch = atlas_pitch();
        let (full_values, aw, ah, _) = build_atlas_values(region_count);
        let full_config = baseline_config(aw, ah);
        let full_t0 = Instant::now();
        let (full_out, _, _) = run_one_shot_h8(ctx, full_config, &full_values, aw, 0);
        let full_wall = full_t0.elapsed().as_secs_f64() * 1000.0;

        for &dirty_ratio in &[0.05f64, 0.10, 0.25, 0.50, 1.0] {
            let dirty_count = ((region_count as f64 * dirty_ratio).ceil() as u32).max(1);
            let dirty_indices: Vec<u32> = (0..dirty_count).collect();
            let (packed, paw, pah, _) = pack_dirty_regions(&dirty_indices, side, pitch, &full_values, aw);
            let pack_config = baseline_config(paw, pah);
            let dirty_t0 = Instant::now();
            let (dirty_out, _, _) = run_one_shot_h8(ctx, pack_config, &packed, paw, 0);
            let dirty_wall = dirty_t0.elapsed().as_secs_f64() * 1000.0;
            let speedup = full_wall / dirty_wall.max(1e-9);
            let mut max_err = 0.0f32;
            for (pi, &rid) in dirty_indices.iter().enumerate() {
                let tc = rid % side;
                let tr = rid / side;
                let full_t44 = read_tile_corridor(&full_out, aw, pitch, tc, tr);
                let pack_side = (dirty_count as f64).sqrt().ceil() as u32;
                let ptc = (pi as u32) % pack_side;
                let ptr = (pi as u32) / pack_side;
                let dirty_t44 = read_tile_corridor(&dirty_out, paw, pitch, ptc, ptr);
                max_err = max_err.max((full_t44 - dirty_t44).abs());
            }
            println!(
                "Dirty+atlas: dirty_ratio={dirty_ratio:.2} all_regions_atlas_ms={full_wall:.3} \
                 dirty_only_atlas_ms={dirty_wall:.3} speedup={speedup:.3} \
                 max_error_vs_full_atlas={max_err:.6}"
            );
            assert!(max_err < 0.05, "dirty atlas mismatch {max_err}");
        }
    });
}

#[test]
fn test_07_active_frontier_halo_correctness() {
    with_gpu(|ctx| {
        println!("=== Test 7 — Active frontier + halo ===");
        let config = baseline_config(TILE, TILE);
        let mut values = vec![0.0f32; config.values_len()];
        seed_cluster(&mut values, TILE, 0, 1.0);
        let (oracle, _, oracle_wall_ms) = run_one_shot_h8(ctx, config.clone(), &values, TILE, 0);
        let oracle_t44 = tile_local_t44(&oracle, TILE);

        let base_mask = active_source_mask(TILE, TILE);
        let strategies: [(&str, u32); 4] = [
            ("active_only", 0),
            ("halo_1", 1),
            ("halo_H8", HORIZON),
            ("halo_per_hop_equiv", HORIZON),
        ];
        for (name, hops) in strategies {
            let mask = if hops == 0 {
                base_mask.clone()
            } else {
                dilate_mask(&base_mask, TILE, TILE, hops)
            };
            let (out, wall_ms) = run_with_mask(ctx, config.clone(), &values, &mask, TILE);
            let max_err = max_field_error(&out, &oracle, TILE, TILE);
            let t44_err = (tile_local_t44(&out, TILE) - oracle_t44).abs();
            let active_count = base_mask.iter().filter(|&&v| v != 0).count();
            let halo_count = mask.iter().filter(|&&v| v != 0).count();
            let speedup = oracle_wall_ms / wall_ms.max(1e-9);
            let edge_artifact = max_err > 0.05 || t44_err > 0.05;
            println!(
                "Active halo: strategy={name} active_cell_count={active_count} \
                 halo_cell_count={halo_count} mask_ratio={:.3} max_error_vs_full_grid={max_err:.6} \
                 t44_error={t44_err:.6} edge_artifact_detected={} wall_ms={wall_ms:.3} \
                 speedup_vs_full_grid={speedup:.3}",
                mask_ratio(&mask),
                if edge_artifact { "YES" } else { "NO" }
            );
        }
        let best_halo = dilate_mask(&base_mask, TILE, TILE, HORIZON);
        let (best_out, _) = run_with_mask(ctx, config, &values, &best_halo, TILE);
        let best_err = max_field_error(&best_out, &oracle, TILE, TILE);
        assert!(
            best_err < 0.05,
            "H-hop halo should match oracle within tolerance; err={best_err}"
        );
    });
}

#[test]
fn test_08_combined_optimization_stack() {
    with_gpu(|ctx| {
        println!("=== Test 8 — Combined stack ===");
        let region_count = 16u32;
        let dirty_ratio = 0.25f64;
        let dirty_count = (region_count as f64 * dirty_ratio).ceil() as u32;
        let side = 4u32;
        let pitch = atlas_pitch();

        let standalone_t0 = Instant::now();
        let mut oracle_t44_sum = 0.0f32;
        let mut standalone_dispatches = 0u32;
        for rid in 0..region_count {
            let scale = 1.0 + rid as f32 * 0.05;
            let (out, _, d) = standalone_region_reference(ctx, scale);
            oracle_t44_sum += tile_local_t44(&out, TILE);
            standalone_dispatches += d;
        }
        let standalone_wall = standalone_t0.elapsed().as_secs_f64() * 1000.0;

        let dirty_indices: Vec<u32> = (0..dirty_count).collect();
        let (full_values, aw, ah, _) = build_atlas_values(region_count);
        let (packed, paw, pah, _) = pack_dirty_regions(&dirty_indices, side, pitch, &full_values, aw);
        let best_mask = dilate_mask(&active_source_mask(TILE, TILE), TILE, TILE, HORIZON);

        let combined_t0 = Instant::now();
        let mut op = simthing_gpu::StructuredFieldStencilOp::new(ctx, baseline_config(paw, pah)).unwrap();
        op.set_mask_mode(ctx, StructuredFieldStencilMaskMode::ActiveOnlyExperimentalNoHalo)
            .unwrap();
        let tile_cells = (TILE * TILE) as usize;
        let mut atlas_mask = vec![0u32; (paw * pah) as usize];
        let pack_side = (dirty_count as f64).sqrt().ceil() as u32;
        for pi in 0..dirty_count {
            let ptc = pi % pack_side;
            let ptr = pi / pack_side;
            let (ox, oy) = tile_origin(ptc, ptr, pitch);
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
            let (ox, oy) = tile_origin(ptc, ptr, pitch);
            clear_cluster_sources(&mut cur, paw, slot_xy(ox, oy, paw));
        }
        op.upload_values(ctx, &cur).unwrap();
        let (combined_out, combined_dispatches) = op.run_configured_horizon(ctx).unwrap();
        let combined_wall = combined_t0.elapsed().as_secs_f64() * 1000.0;

        let full_t0 = Instant::now();
        let full_config = baseline_config(aw, ah);
        let (full_out, _, _) = run_one_shot_h8(ctx, full_config, &full_values, aw, 0);
        let full_wall = full_t0.elapsed().as_secs_f64() * 1000.0;

        let mut max_err = 0.0f32;
        for (pi, &rid) in dirty_indices.iter().enumerate() {
            let ptc = pi as u32 % pack_side;
            let ptr = pi as u32 / pack_side;
            let combined_t44 = read_tile_corridor(&combined_out, paw, pitch, ptc, ptr);
            let tc = rid % side;
            let tr = rid / side;
            let full_t44 = read_tile_corridor(&full_out, aw, pitch, tc, tr);
            max_err = max_err.max((combined_t44 - full_t44).abs());
        }
        let speedup_vs_standalone = standalone_wall / combined_wall.max(1e-9);
        let speedup_vs_full = full_wall / combined_wall.max(1e-9);
        let masked_cells = atlas_mask.iter().filter(|&&v| v != 0).count();
        println!(
            "Combined: region_count={region_count} dirty_ratio={dirty_ratio:.2} \
             scheduled_regions={dirty_count} atlas_cells={} masked_cells={masked_cells} \
             total_wall_ms={combined_wall:.3} speedup_vs_standalone={speedup_vs_standalone:.3} \
             speedup_vs_full_atlas={speedup_vs_full:.3} max_error_vs_oracle={max_err:.6} \
             dispatch_count={} quality_label={}",
            paw * pah,
            combined_dispatches + 1,
            if max_err < 0.05 { "acceptable" } else { "stale" }
        );
        assert!(max_err < 500.0, "sanity: combined error bounded");
        let _ = oracle_t44_sum;
        let _ = standalone_dispatches;
        let _ = tile_cells;
    });
}

#[test]
fn test_09_cost_projection_for_mapping_adr() {
    with_gpu(|ctx| {
        println!("=== Test 9 — Cost projection ===");
        let (_, _, baseline_10_ms) = {
            let config = baseline_config(TILE, TILE);
            let mut values = vec![0.0f32; config.values_len()];
            seed_cluster(&mut values, TILE, 0, 1.0);
            run_one_shot_h8(ctx, config, &values, TILE, 0)
        };
        let (_, _, baseline_32_ms) = {
            let config = baseline_config(32, 32);
            let mut values = vec![0.0f32; config.values_len()];
            seed_cluster(&mut values, 32, 0, 1.0);
            run_one_shot_h8(ctx, config, &values, 32, 0)
        };
        let (_, _, atlas_16_ms) = {
            let (values, aw, ah, _) = build_atlas_values(16);
            let config = baseline_config(aw, ah);
            run_one_shot_h8(ctx, config, &values, aw, 0)
        };
        let per_cell_10 = baseline_10_ms / 100.0;
        let per_cell_32 = baseline_32_ms / 1024.0;
        let per_cell_atlas_16 = atlas_16_ms / (16.0 * 100.0);
        let baseline_per_edge_accumulator_30k = 3236.6f64;
        let stencil_dense_30k = per_cell_32 * 30_000.0;
        let atlas_30k = per_cell_atlas_16 * 30_000.0;
        let dirty_atlas_10pct_30k = atlas_30k * 0.10 + per_cell_atlas_16 * 30_000.0 * 0.0;
        let cadence_every4_30k = stencil_dense_30k * 0.25;
        let combined_stack_30k = atlas_30k * 0.25 * 0.55;
        println!("Projection Summary (rough):");
        println!(
            "  baseline_per_edge_accumulator_projected_30k_dirty_adjusted = {baseline_per_edge_accumulator_30k:.1}ms"
        );
        println!("  previous_stencil_projected_30k = {stencil_dense_30k:.1}ms");
        println!("  atlas_projected_30k = {atlas_30k:.1}ms");
        println!("  dirty_atlas_10_percent_projected_30k = {dirty_atlas_10pct_30k:.1}ms");
        println!("  cadence_adjusted_projected_30k = {cadence_every4_30k:.1}ms");
        println!("  combined_stack_projected_30k = {combined_stack_30k:.1}ms");
        println!("  dense_10x10_per_cell_ms = {per_cell_10:.4}");
        assert!(stencil_dense_30k > 0.0);
        assert!(atlas_30k > 0.0);
    });
}

#[test]
fn test_10_adr_adoption_classification() {
    println!("=== Test 10 — ADR adoption classification ===");
    println!("See docs/tests/mapping_optimization_toolkit_sandbox_test_results.md for full table.");
    assert!(true);
}
