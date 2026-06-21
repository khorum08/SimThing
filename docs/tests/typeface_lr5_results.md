# TYPEFACE-LR5-HIGH-VOLUME-BENCH-BUDGET-0 Results

## Status

PASS — LR4 accepted/closed; deterministic high-volume typeface benchmark and conservative CPU budget gates implemented in `simthing-tools`; LR5 lands at **PROBATION / DA-SENSITIVE**.

## PR / branch / merge

- Branch: `typeface-lr5-high-volume-bench-budget-0`
- PR: pending
- Merge SHA: pending

## Mission

Close LR4 documentation lifecycle and add a deterministic high-volume performance benchmark and budget gate for the existing LR0–LR4 typeface stack: TTF shaped text, SVG/PUA icons, shared `GlyphAtlasCore`, mixed text+icon instances, and LR3 changed-detection semantics.

## LR4 closeout

`TYPEFACE-LR4-SVG-PUA-ICON-INGESTION-0` — **ACCEPTED / closed**. PR #878, merge `990d6ce5ce804523564fe65e56725ece23a7a37d`. Post-merge evidence commit `7c8cb1bd15`. Static SVG icon ingestion at PUA codepoints accepted. Role-aware `IconVector` IR accepted. Shared atlas insertion and mixed text+icon instance proof accepted. Typeface track remains OPEN.

## Design ladder sync

LR4 is **DONE / ACCEPTED (#878)**. LR5 is the active **DONE / PROBATION** DA-sensitive perf gate. Typeface track remains OPEN.

## Implementation

- Added `crates/simthing-tools/src/bench.rs` with `TypefaceBenchConfig`, `TypefaceBenchResult`, `TypefaceBenchHarness`, and `run_typeface_bench`.
- CPU-side label pool mirrors LR3 changed-detection: labels rebuild only when text/px/color changes.
- Added `IconSet::cache_entry_count()` for icon cache observability.
- Added `crates/simthing-tools/tests/typeface_lr5.rs` with CI-scale and optional heavy `#[ignore]` bench.

## Benchmark architecture

`TypefaceBenchHarness` owns `ProbeFont`, `ShapingEngine`, `GlyphAtlasCore`, and `IconSet`. Static and damage label vectors carry a `built` flag and cached `GlyphInstanceGpu` lists. Rebuild increments `TypefaceBenchDiagnostics.shape_rebuild_count`; atlas `rasterize_count` tracks new tile inserts. `run_typeface_bench` orchestrates three phases: initial build, no-op frames, and damage churn frames.

## Scenario 1: static nameplates

Build 500 static labels (CI config) with repeated map-style strings (`Sol Prime`, `Altair`, …) and PUA icons every 5th label. Initial build shapes and rasterizes once. No-op frames assert zero additional shape rebuilds and zero rasterize delta.

## Scenario 2: damage text churn

100 damage labels (CI config) mutate deterministically each frame (`-{value}`). Only damage labels rebuild; static labels remain cached. Rebuild count equals `damage_labels × damage_frames`.

## Scenario 3: mixed text+icon atlas stress

Two fixture SVG icons register at `ICON_PUA_START + 1` and `+ 2` into the same `GlyphAtlasCore` used by font glyphs. Mixed labels reference PUA codepoints; icon cache entries remain stable under load.

## Budget gates

- No-op frames must not increase shape rebuild count — **PASS**
- No-op frames must not increase rasterize count — **PASS**
- No-op frames must not increase icon cache entries — **PASS**
- Repeated static labels reuse cached shaping/atlas/instance data — **PASS**
- Mixed text+icon workloads use one atlas path — **PASS**
- CI integration test completes under 30 seconds — **PASS** (~0.2s on validation host)

## Performance observations

CI config (`CI_BENCH_CONFIG`: 500 static + 100 damage, 12 frames, 2048 atlas): full bench harness completes in ~170ms on validation host. No-op phase wall-clock is sub-millisecond relative to initial build. Heavy config (`HEAVY_BENCH_CONFIG`: 5000 static + 500 damage) is available as `#[ignore]` manual bench only.

## Determinism notes

Structural counts (label counts, shape rebuild counts, rasterize deltas, instance count, icon cache entries) match across repeated `run_typeface_bench(CI_BENCH_CONFIG)` runs. Wall-clock timings are recorded but not asserted equal.

## Tests

`crates/simthing-tools/tests/typeface_lr5.rs` — 7 CI tests + 1 ignored heavy bench:

- `high_volume_static_labels_noop_frame_does_not_reshape`
- `high_volume_static_labels_noop_frame_does_not_rerasterize`
- `damage_text_churn_rebuilds_only_changed_labels`
- `mixed_text_icon_workload_reuses_one_atlas`
- `repeated_svg_icons_are_cached_under_load`
- `bench_result_report_is_deterministic_enough`
- `ci_bench_budget_gates_pass`
- `heavy_bench_manual` (`#[ignore]`)

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
- `cargo test -p simthing-tools --test typeface_lr5` — 7/7 PASS (1 ignored heavy bench)
- `git diff --check` — PASS

## Files changed

- `crates/simthing-tools/src/bench.rs` (new)
- `crates/simthing-tools/src/lib.rs`
- `crates/simthing-tools/src/icons.rs`
- `crates/simthing-tools/tests/typeface_lr5.rs` (new)
- `docs/design_typeface_ladder.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/typeface_lr4_results.md`
- `docs/tests/typeface_lr5_results.md` (new)
- `docs/workshop/studio_production_log.md`

## Boundary / non-goals

No MSDF, style tables, gradients, glyph deformation, text-on-path, TTF/OTF export, COLRv1, icon-pack licensing workflow, Studio/game label integration, or ScenarioSpec/RF/STEAD changes.

## Known gaps

- LR5 records CPU-side build budget only; GPU draw-call / real-adapter FPS gate deferred to DA review and optional heavy manual bench.
- Original ladder binding budget (≥5000 labels @ ≥60 FPS, CPU build < 1 ms/frame on GPU path) is not fully exercised in default CI — heavy bench is `#[ignore]`.
- Bevy plugin path is not re-benchmarked at 5000 labels in CI; harness uses direct `IconSet`/`ShapingEngine`/`GlyphAtlasCore` API matching LR3 semantics.

## DA recommendation

Recommend DA review of LR5 structural budget gates and recorded baselines before LR6 MSDF work. LR5 should remain **PROBATION** until Codex sign-off.

## Next recommended action

Land LR5 at PROBATION; DA review perf baselines; proceed to LR6 MSDF atlas when approved.
