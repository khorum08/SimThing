# TYPEFACE-LR5-NUMERIC-DAMAGE-LANE-0R Results

## Status

PASS — fixed-width numeric damage glyph lane meets 5k/500 binding budget; shaping bypass and aggregate stability proven; **recommend LR5 DA approval** pending Design Authority review.

## PR / branch / merge

- Branch: `typeface-lr5-numeric-damage-lane-0r`

## DA HOLD being remediated

LR5S (#881) reduced variable-width damage CPU ~39% but left avg damage ~2.26 ms/frame with shaping ~1.06 ms/frame dominant and variable-width strings triggering 48 aggregate repacks over 60 damage frames.

## Baseline from LR5S

| Metric | LR5S |
|---|---|
| avg_noop_update_ms | 0.0632 |
| avg_damage_update_ms (variable-width) | 2.2550 |
| dominant phase | shaping ~1.06 ms/frame |
| aggregate repacks (60 damage frames) | 48 |

## Numeric damage lane design

`NumericDamageLabel` component + import-time `NumericGlyphRunTable` (`numeric_damage.rs`). Plugin init shapes reference `-0000` once, rasterizes minus + digits 0–9, stores slot-x positions and UV templates. Runtime `update_numeric_damage_labels` decomposes integer `value` into digits and writes fixed 5-glyph runs without cosmic-text or string formatting.

## Fixed-width contract

- Format: `-####` (minus + 4 digits; magnitude 0–9999)
- Glyph count: 5 per label (fixed)
- Update path: `NumericDamageLabel.value: i32` mutation only
- Binding profile uses `spawn_static_and_numeric_damage_labels` + `profile_bevy_fixed_width_numeric_damage_bench`

## Shaping bypass proof

| Check | Result |
|---|---|
| `shape_rebuild_count` delta (60 damage frames) | **0** |
| `shape_cache_miss_count` delta (timed damage) | **0** |
| `TextDamagePhaseProfile.shaping_ns` (damage run) | **0** |
| `numeric_shape_bypass_count` (total) | **30500** (= 500 init + 500×60) |

## Aggregate stability proof

| Check | Result |
|---|---|
| `aggregate_repack_count` delta (timed damage) | **0** |
| `aggregate_patch_count` (damage run) | **30000** (= 500×60) |
| `aggregate_full_rebuild_count` (damage run) | **1** (initial only) |

## Atlas warmup / sync proof

- Table build + digit prewarm at plugin init (`atlas_sync_bytes` once at startup)
- Timed damage frames: `atlas_sync_bytes` delta = **0**; `atlas_sync_ns` = **0**

## Variable-width historical profile

LR5S recorded evidence (unchanged; `profile_bevy_text_bench` retained for TextLabel path):

| Field | Value |
|---|---|
| avg_damage_update_ms | **2.2550** |
| status | HOLD / historical |

## Fixed-width 5k damage profile

Manual `binding_5k_fixed_width_numeric_damage_profile` (`#[ignore]`) on validation host:

| Field | LR5S (variable) | LR5T (fixed-width numeric) |
|---|---|---|
| labels | 5500 | 5500 |
| avg_noop_update_ms | 0.0632 | **0.0590** |
| max_noop_update_ms | 0.2032 | **0.3309** |
| avg_damage_update_ms | 2.2550 | **0.5773** |
| max_damage_update_ms | 5.5561 | **0.7608** |
| aggregate_repack delta | 48 | **0** |
| shape_cache_miss delta (damage) | many | **0** |

**PASS** — avg fixed-width damage update **0.577 ms/frame** (<1 ms binding).

Dominant remaining phase: **draw_sync** (~0.30 ms/frame from phase profile).

## GPU residency / CPU surfacing audit

- CPU operations introduced: import-time `NumericGlyphRunTable` build; integer digit decomposition in numeric lane (staging only, not gameplay semantics).
- CPU operations removed: per-frame cosmic-text shaping for fixed-width numeric damage; per-frame string formatting in fixed-width binding profile.
- CPU operations retained and why: static `TextLabel` still uses admitted cosmic-text import/layout; dirty-rect atlas blit; aggregate patch + draw-entity sync orchestration.
- Numeric production authority remains GPU-resident: **yes**
- Deviations: none recorded.
- Next GPU-residency debt: static nameplate `TextLabel` path still CPU-shaped; variable-width TextLabel damage retained for regression tests only.

## Tests

- `numeric_damage_lane_bypasses_cosmic_text_after_init`
- `numeric_damage_lane_keeps_segment_width_stable`
- `numeric_damage_lane_patches_aggregate_without_repack`
- `numeric_damage_lane_uses_prewarmed_digit_tiles`
- `numeric_damage_lane_does_not_allocate_or_format_strings_per_frame_where_measurable`
- `binding_5k_fixed_width_numeric_damage_profile_under_1ms_or_honest_hold`
- `gpu_residency_audit_updated_for_numeric_lane`

## Validation

```text
cargo fmt -p simthing-tools -p simthing-workshop -- --check
cargo check -p simthing-tools
cargo check -p simthing-workshop
cargo test -p simthing-workshop --test typeface_lr0/lr1/lr2
cargo test -p simthing-tools --test typeface_lr3/lr4/lr5/semantic_free_guard
binding_5k_fixed_width_numeric_damage_profile (ignored manual)
```

## Files changed

- `crates/simthing-tools/src/numeric_damage.rs` (new)
- `crates/simthing-tools/src/bevy.rs`
- `crates/simthing-tools/src/lib.rs`
- `crates/simthing-tools/tests/typeface_lr5.rs`
- `docs/tests/typeface_lr5t_results.md`
- `docs/design_typeface_ladder.md`, `docs/tests/current_evidence_index.md`, `docs/workshop/studio_production_log.md`

## Boundary / non-goals

No LR6 MSDF, style, deformation, export, Studio, ScenarioSpec/RF/STEAD changes.

## DA recommendation

**Recommend LR5 DA approval** — fixed-width numeric damage binding met (<1 ms avg at 5k/500/60); no-op binding preserved; shaping bypass and aggregate stability proven. Variable-width TextLabel damage remains documented historical debt, not the binding path.

## Next recommended action

DA review and disposition LR5; if approved, advance typeface track to LR6 MSDF planning.
