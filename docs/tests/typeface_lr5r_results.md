# TYPEFACE-LR5-PERF-PATH-0R Results

## Status

PASS — Bevy-path perf remediation landed; 5k no-op binding met; damage churn CPU cost recorded honestly; LR5 remains **PROBATION / DA HOLD pending LR5R review**.

## PR / branch / merge

- Branch: `typeface-lr5-perf-path-0r`
- PR: pending
- Merge SHA: pending

## DA HOLD being remediated

LR5 PR #879 added a useful CPU-side `TypefaceBenchHarness` but did not exercise the Bevy plugin path at scale or prove the ladder binding budget (≥5,000 labels, no-op CPU build <1 ms/frame, bounded draw calls, single atlas bind, instanced). The Bevy path had per-frame aggregate rebuild, draw-entity clone, full-atlas sync on rebuild, and fresh GPU buffer creation each render prepare.

## Code weaknesses addressed

1. **CPU harness did not catch Bevy churn** — added Bevy-path diagnostics and structural tests.
2. **`aggregate_label_instances` scanned all labels every frame** — dirty/version gating; no-op aggregate delta = 0.
3. **`sync_draw_entity_instances` cloned every frame** — sync only when aggregate version changes.
4. **`TextDrawExtract` clone every extraction** — version-stamped extract; clone count tracked once per version change on render path.
5. **`prepare_text_instance_buffers` recreated buffers every frame** — reuse/upload when `data_version` unchanged; create only on capacity/version change.
6. **Full atlas image copy on any shape rebuild** — dirty-rect sync via `GlyphAtlasCore::dirty_regions()`; no-op atlas sync delta = 0.

## Implementation

- Extended `TextPerfDiagnostics` / `TextRenderPerfDiagnostics` with aggregate, draw sync, extract, buffer, atlas, and queue counters.
- Added `TextAggregateVersion` dirty/version resource with `ExtractResource` mirror.
- Chained `ApplyDeferred` before aggregate so deferred label instance inserts are visible.
- Added `SimthingToolsTextPlugin::with_atlas_size`, `spawn_static_text_labels`, `profile_bevy_text_bench`, `text_perf_diagnostics`.
- Exposed `AtlasDirtyRect`, `dirty_regions()`, `dirty_region_byte_count()` on `GlyphAtlasCore`.

## Direct CPU harness result

All LR5 direct harness tests PASS unchanged. No-op shape/raster/icon cache deltas remain zero under load.

## Bevy no-op 5k result

Manual `binding_5k_budget_profile_records_avg_and_max_frame_cost` (`#[ignore]`) on validation host:

| Field | Value |
|---|---|
| labels | 5500 (5000 static + 500 damage) |
| noop frames | 60 |
| avg_noop_update_ms | **0.0634** |
| max_noop_update_ms | **0.1685** |
| shape rebuild delta (noop) | 0 |
| aggregate rebuild delta (noop) | 0 |
| draw sync delta (noop) | 0 |
| atlas sync delta (noop) | 0 |

**PASS** — avg no-op CPU update < 1 ms/frame.

## Bevy damage-churn result

Same 5k profile, 60 damage frames mutating 500 labels/frame:

| Field | Value |
|---|---|
| avg_damage_update_ms | **3.6937** |
| max_damage_update_ms | **4.5344** |
| shape rebuilds (total after damage) | 35500 (= 5500 initial + 500×60) |
| aggregate rebuilds (total) | 61 (= 1 initial + 60 damage frames) |

**HOLD on damage CPU budget** — 500-label churn/frame exceeds 1 ms/frame average; recorded as baseline debt, not hidden.

## Aggregate/versioning proof

- No-op frames: `aggregate_rebuild_count` delta = 0.
- Damage frame with 100 label changes: aggregate rebuild delta = 1 (not 100).
- `draw_entity_sync_count` increments once per aggregate version change.

## Instance-buffer reuse proof

Render-path test `bevy_noop_frames_do_not_recreate_instance_buffer` (512 labels + GPU):

- `instance_buffer_create_count` delta = 0 across 6 no-op render frames.
- `instance_buffer_reuse_count` increases on no-op render prepares.

## Atlas sync proof

- No-op: `atlas_sync_count` and `atlas_sync_bytes` deltas = 0.
- Initial build: dirty-rect sync only (`atlas_sync_bytes=10968` for 16 regions at 5k profile, not full 4096² atlas).

## Draw-call / atlas-bind / instancing proof

`bevy_queue_remains_single_draw_entity_single_atlas_bind`:

- One `TextInstancedDraw` entity.
- `queued_draw_count = 1`.
- `queued_instance_count` equals aggregate instance count.
- One atlas bind group path unchanged.

## Binding 5k budget profile

Recorded from `cargo test -p simthing-tools --test typeface_lr5 binding_5k_budget_profile_records_avg_and_max_frame_cost -- --ignored --nocapture`:

```
labels=5500
damage_labels=500
avg_noop_update_ms=0.0634
max_noop_update_ms=0.1685
avg_damage_update_ms=3.6937
max_damage_update_ms=4.5344
aggregate_rebuild_count (after noop phase): 1
draw_entity_sync_count (after noop phase): 1
atlas_sync_count: 1
atlas_sync_bytes: 10968
```

## Performance opportunities landed

- Dirty aggregate versioning with deferred-command flush.
- Vec capacity reuse on label instance rebuild and aggregate rebuild.
- GPU instance buffer reuse/upload keyed by aggregate `data_version`.
- Atlas dirty-rect CPU sync instead of full-atlas copy on every rebuild.
- Draw-entity sync gated on aggregate version.

## Remaining performance debt

- Damage churn at 500 labels/frame exceeds 1 ms CPU update average (~3.7 ms on validation host).
- `TextDrawExtract` still clones instance data when aggregate version changes (bounded, not per-label).
- Bevy `Assets<Image>` partial GPU upload still CPU-copies dirty rects into main-world image buffer (no wgpu queue partial upload in plugin path yet).
- Render-path extract/queue counters are zero in CPU-only 5k profile (no RenderApp); GPU structural tests cover queue/buffer proof separately.

## Tests

`crates/simthing-tools/tests/typeface_lr5.rs` — 15 CI + 2 ignored:

- Direct harness regressions (LR5 retained)
- `bevy_noop_frames_do_not_reaggregate_or_resync`
- `bevy_damage_churn_rebuilds_changed_labels_only`
- `bevy_damage_churn_aggregates_once_per_frame_not_per_label`
- `bevy_noop_frames_do_not_sync_full_atlas`
- `bevy_noop_frames_do_not_recreate_instance_buffer` (GPU)
- `bevy_queue_remains_single_draw_entity_single_atlas_bind` (GPU)
- `binding_1k_budget_profile_records_avg_and_max_frame_cost`
- `binding_5k_budget_profile_records_avg_and_max_frame_cost` (`#[ignore]`)

LR3/LR4 regressions enforced by validation commands.

## Validation

All gates PASS on validation host (2026-06-21):

- `cargo fmt -p simthing-tools -p simthing-workshop -- --check` — PASS
- `cargo check -p simthing-tools` — PASS
- `cargo check -p simthing-workshop` — PASS
- `cargo test -p simthing-workshop --test typeface_lr0` — 7/7 PASS
- `cargo test -p simthing-workshop --test typeface_lr1` — 7/7 PASS
- `cargo test -p simthing-workshop --test typeface_lr2` — 8/8 PASS
- `cargo test -p simthing-tools --test typeface_lr3` — 10/10 PASS
- `cargo test -p simthing-tools --test semantic_free_guard` — 1/1 PASS
- `cargo test -p simthing-tools --test typeface_lr4` — 9/9 PASS
- `cargo test -p simthing-tools --test typeface_lr5` — 15/15 PASS (2 ignored)
- `cargo test -p simthing-tools --test typeface_lr5 binding_5k_budget_profile_records_avg_and_max_frame_cost -- --ignored --nocapture` — PASS (5k profile recorded above)
- `git diff --check` — PASS

## Files changed

- `crates/simthing-tools/src/bevy.rs`
- `crates/simthing-tools/src/text_render.rs`
- `crates/simthing-tools/src/atlas.rs`
- `crates/simthing-tools/src/lib.rs`
- `crates/simthing-tools/tests/typeface_lr5.rs`
- `docs/design_typeface_ladder.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/typeface_lr5r_results.md`
- `docs/workshop/studio_production_log.md`

## Boundary / non-goals

No MSDF, style tables, gradients, deformation, text-on-path, export, COLRv1, Studio/game integration, or ScenarioSpec/RF/STEAD changes.

## DA recommendation

Accept LR5R structural remediation and 5k no-op binding proof. Keep LR5 at **PROBATION**; damage-churn CPU cost remains open for DA review before LR6. Do not mark LR5 DA-approved.

## Next recommended action

DA review LR5R evidence; plan damage-churn optimization or revised binding interpretation; proceed to LR6 only after DA sign-off.
