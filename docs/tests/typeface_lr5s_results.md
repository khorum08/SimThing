# TYPEFACE-LR5-DAMAGE-CHURN-GPU-AUDIT-0R Results

## Status

PASS — LR5S damage-churn remediation landed; 5k no-op binding preserved; damage-frame phase profile and aggregate patching added; damage CPU budget remeasured; LR5 remains **PROBATION / DA HOLD pending DA review**.

## PR / branch / merge

- Branch: `typeface-lr5-damage-churn-gpu-audit-0r`

## DA HOLD being remediated

LR5R (#880) fixed no-op Bevy-path churn but left 500-label damage churn at ~3.7 ms/frame average (target &lt;1 ms). Changed-label rebuild still cloned instance `Vec`s; aggregate still full-rebuilt every damage frame; shaping reran cosmic-text for every numeric damage string; no phase-level breakdown or GPU-residency audit.

## Baseline from LR5R

| Metric | LR5R (#880) |
|---|---|
| 5k avg_noop_update_ms | 0.0634 |
| 5k avg_damage_update_ms | 3.6937 |
| aggregate rebuild / damage frame | 1 full rebuild |
| changed-label instance clone | yes (`.0.clone()` + temp Vec) |

## Code weaknesses addressed

1. **Changed-label instance Vec clone** — rebuild pushes directly into existing `TextGlyphInstances.0` or inserts a new Vec; no temp clone/copy-back.
2. **Full aggregate rebuild every damage frame** — segmented `LabelAggregateSegment` metadata; patch in place when instance width stable; full rebuild/repack only on lifecycle or width change.
3. **Repeated cosmic-text shaping for numeric damage strings** — `ShapingEngine::shape_cached` for `-` + digit labels; digit/`minus` glyph prewarm at plugin init.
4. **No phase breakdown** — `TextDamagePhaseProfile` records shaping, rasterize, instance rebuild, aggregate patch/full, draw sync, atlas sync nanoseconds.
5. **No GPU-residency audit language** — this report adds required audit section.

## Damage-frame phase profile

Manual 5k binding run records cumulative phase totals over 60 damage frames (500 labels/frame). See binding section for totals; dominant phases identified in DA recommendation.

| Phase bucket | Role |
|---|---|
| shaping | cosmic-text or shape cache for changed labels |
| rasterize | atlas tile lookup/raster inside instance rebuild |
| instance_rebuild | per-label GPU instance struct fill |
| aggregate_patch | in-place segment copy into aggregate |
| aggregate_full_rebuild | full scan + segment metadata rebuild |
| draw_sync | draw-entity copy when aggregate version changes |
| atlas_sync | dirty-rect CPU→image blit |

## Changed-label rebuild optimization

- Removed `.map(|existing| existing.0.clone())` and temp-Vec `extend_from_slice` copy-back.
- Guard test `changed_label_rebuild_does_not_clone_old_instance_vec` greps source.

## Aggregate patching / rebuild strategy

- `LabelAggregateSegment { offset, len }` per label entity; `SegmentDirty` marker on changed labels.
- Stable-width damage churn patches `aggregate.0[offset..offset+len]` per label (500 patches/frame in CI proof).
- Width change or label add/remove triggers `aggregate_full_rebuild_count` and optional `aggregate_repack_count`.
- Counters: `aggregate_patch_count`, `aggregate_full_rebuild_count`, `aggregate_repack_count`, `aggregate_patched_instance_count`, `aggregate_full_rebuild_instance_count`.

## Numeric damage-label cache behavior

- Plugin init prewarms `0-9` and `-` glyphs at 24 px into atlas (dirty cleared before first frame).
- `shape_cached` stores shaped runs keyed by `(text, px_bucket)` for numeric damage labels only.
- `shape_cache_hit_count` / `shape_cache_miss_count` tracked in `TextPerfDiagnostics`.

## Atlas sync behavior under damage churn

- Dirty-rect sync preserved from LR5R; no-op atlas sync delta remains zero.
- After digit prewarm, damage churn atlas sync bytes trend down (`damage_churn_dirty_atlas_sync_trends_to_zero_after_warmup`).

## Render-world counter coverage

- CPU-only 5k profile still measures main-world diagnostics; render queue/buffer counters remain covered by adapter-optional structural tests (`bevy_queue_remains_single_draw_entity_single_atlas_bind`, `bevy_noop_frames_do_not_recreate_instance_buffer`).
- CPU-update profile and GPU structural proof are explicitly separated in this report.

## 5k damage profile

Manual `binding_5k_budget_profile_records_avg_and_max_frame_cost` (`#[ignore]`) on validation host:

| Field | LR5R | LR5S |
|---|---|---|
| labels | 5500 | 5500 |
| avg_noop_update_ms | 0.0634 | **0.0632** |
| max_noop_update_ms | 0.1685 | **0.2032** |
| avg_damage_update_ms | 3.6937 | **2.2550** |
| max_damage_update_ms | 4.5344 | **5.5561** |
| noop shape/aggregate/draw/atlas deltas | 0 | **0** |
| shape_cache_hits (60 damage frames) | n/a | **21345** |
| aggregate_patch_count (damage run) | n/a | **6000** |
| aggregate_full_rebuild_count (damage run) | 61 total | **49** (48 repacks from variable-width `-{value}`) |

**No-op PASS** — avg &lt;1 ms preserved. **Damage HOLD** — avg 2.26 ms/frame; ~39% reduction from LR5R but still above 1 ms binding.

Phase totals over 60 damage frames (`TextDamagePhaseProfile`):

| Phase | Total ns | ~ms/frame |
|---|---|---|
| shaping | 63,401,800 | **1.06** |
| instance_rebuild | 5,786,200 | 0.10 |
| aggregate_full_rebuild | 11,535,700 | 0.19 |
| draw_sync | 7,391,500 | 0.12 |
| rasterize (in rebuild) | 4,314,000 | 0.07 |
| aggregate_patch | 106,000 | ~0.002 |
| atlas_sync | 0 | 0 (digits prewarmed) |

Dominant remaining cost: **shaping** (cache hits reduce but misses + variable-width repacks remain).

## GPU residency / CPU surfacing audit

- CPU operations introduced: damage-frame phase timing resource (diagnostic only); shape cache for repeated numeric strings (import/layout-stage cache, not presentation policy); aggregate segment bookkeeping for patch path.
- CPU operations removed: per-changed-label instance Vec clone; unconditional full aggregate rebuild on stable-width damage frames.
- CPU operations retained and why: TTF parse + cosmic-text shaping on cache miss (admitted CPU import/layout stage); dirty-rect atlas staging blit (GPU upload orchestration); per-changed-label instance struct fill (staging before GPU instance buffer); draw-entity sync on aggregate version change (boundary orchestration).
- Numeric production authority remains GPU-resident: **yes** — placement fields remain `GlyphInstanceGpu` in instance/aggregate buffers consumed by WGSL; CPU does not own gameplay meaning, thresholds, or deformation.
- Deviations: none recorded.
- Next GPU-residency debt: cosmic-text shaping remains CPU authority for layout until a GPU shaping path or baked glyph-run import exists; variable-width damage strings still trigger aggregate repack on width change.

## Tests

Added/strengthened in `crates/simthing-tools/tests/typeface_lr5.rs`:

- `changed_label_rebuild_does_not_clone_old_instance_vec`
- `damage_churn_phase_profile_records_breakdown`
- `damage_churn_uses_aggregate_patch_when_instance_width_stable`
- `damage_churn_full_rebuild_only_when_segment_width_changes`
- `digit_glyphs_are_cached_after_prewarm`
- `damage_churn_dirty_atlas_sync_trends_to_zero_after_warmup`
- `binding_5k_damage_profile_remeasured`
- `gpu_residency_audit_documented`

## Validation

```text
cargo fmt -p simthing-tools -p simthing-workshop -- --check
cargo check -p simthing-tools
cargo check -p simthing-workshop
cargo test -p simthing-workshop --test typeface_lr0
cargo test -p simthing-workshop --test typeface_lr1
cargo test -p simthing-workshop --test typeface_lr2
cargo test -p simthing-tools --test typeface_lr3
cargo test -p simthing-tools --test semantic_free_guard
cargo test -p simthing-tools --test typeface_lr4
cargo test -p simthing-tools --test typeface_lr5
cargo test -p simthing-tools --test typeface_lr5 binding_5k_budget_profile_records_avg_and_max_frame_cost -- --ignored --nocapture
git diff --check
```

## Files changed

- `crates/simthing-tools/src/bevy.rs` — patch aggregate, phase profile, no-clone rebuild, digit prewarm, extended diagnostics
- `crates/simthing-tools/src/shaping.rs` — shape cache
- `crates/simthing-tools/src/lib.rs` — exports
- `crates/simthing-tools/tests/typeface_lr5.rs` — LR5S tests
- `docs/tests/typeface_lr5s_results.md` — this report
- `docs/design_typeface_ladder.md`, `docs/tests/current_evidence_index.md`, `docs/workshop/studio_production_log.md`

## Boundary / non-goals

No LR6 MSDF, fdsm, style tables, gradients, deformation, text-on-path, export, COLRv1, Studio/game integration, or ScenarioSpec/RF/STEAD changes.

## DA recommendation

**Keep LR5 at PROBATION / DA HOLD** until DA reviews remeasured 5k damage profile and accepts either &lt;1 ms avg damage update or a binding reinterpretation. Recommend LR6 MSDF only after LR5 DA disposition.

## Next recommended action

DA review LR5S evidence; if damage budget still exceeds 1 ms, identify dominant phase from `TextDamagePhaseProfile` and scope LR5T shaping/atlas debt reduction before MSDF.
