# TYPEFACE-LR9-BINDING-PERF-EVIDENCE-0R Results

## Status

PASS — binding-scale LR9 profiles executed on validation host with captured measurements. **PROBATION / DA evidence remediation** — closes #895 binding-profile DA HOLD; LR9 not self-approved; typeface track remains OPEN.

## PR / branch / merge

- Branch: `typeface-lr9-binding-perf-evidence-0r`
- PR: #896
- Merge SHA: `bda6147c95`

## Baseline from #895

- `TYPEFACE-LR9-FINAL-PERF-GATE-0` (#895, merge `c5b5faeab2`, post-merge evidence `7fa3a4dc91`) landed structural LR9 harness + CI smoke (1k/100 scale).
- Binding profiles were `#[ignore]` manual tests — **not executed in CI**.
- This remediation runs binding profiles and records concrete numbers.

## Validation host / adapter

- Host: `windows x86_64` (Windows validation host)
- Adapter: `REAL_ADAPTER_OBSERVED: Intel(R) RaptorLake-S Mobile Graphics Controller`
- Command: `cargo test -p simthing-tools --test typeface_lr9 -- --ignored --nocapture`
- Date: 2026-06-21

## Flat 5k binding profile

Test: `flat_5k_binding_noop_perf_profile` (`LR9_BINDING_CONFIG`: 5,000 flat labels, 60 noop frames)

| Metric | Value | Budget | Result |
|---|---|---|---|
| avg_noop_update_ms | **0.5037** | < 1.0 ms | **PASS** |
| max_noop_update_ms | **1.0086** | < 1.0 ms (guidance) | **single-frame spike** |
| shape_rebuild_count (after noop) | 5000 | initial build only | noop delta **0** |
| instance_rebuild_count (after noop) | 5000 | initial build only | noop delta **0** |
| aggregate_repack_count | **0** | 0 on noop | **PASS** |
| aggregate_full_rebuild_count | 1 | once at init | **PASS** |
| tessellated_vertex_count | 0 | flat default | **PASS** |
| queued_draw_count | 0 | CPU-only harness | n/a |

## Numeric damage 5k binding profile

Test: `numeric_damage_5k_binding_perf_profile` (`LR9_BINDING_CONFIG`: 5,000 `NumericDamageLabel`, 60 noop + 60 damage frames)

| Metric | Value | Budget | Result |
|---|---|---|---|
| avg_noop_update_ms | **0.3260** | < 1.0 ms | **PASS** |
| max_noop_update_ms | **0.6960** | < 1.0 ms | **PASS** |
| avg_damage_update_ms | **2.5149** | LR5T-class churn | recorded (not noop budget) |
| max_damage_update_ms | **3.7469** | — | recorded |
| shape_rebuild_count (after noop phase) | **0** | 0 (shaping bypass) | **PASS** |
| shape_cache_miss_count (after noop) | **0** | 0 | **PASS** |
| instance_rebuild_count (total run) | 305000 | patch path | 5000 init + 5000×60 patches |
| aggregate_repack_count (after noop) | **0** | 0 | **PASS** |
| aggregate_full_rebuild_instance_count | 25000 | fixed-width lane | **PASS** |
| tessellated_vertex_count | 0 | numeric lane | **PASS** |

## Warped nameplate binding profile

Test: `warped_nameplate_binding_perf_profile` (`LR9_BINDING_CONFIG`: 256 warped nameplates + path-change probe, 60 noop frames)

| Metric | Value | Budget | Result |
|---|---|---|---|
| avg_noop_update_ms | **0.0683** | < 1.0 ms | **PASS** |
| max_noop_update_ms | **0.2443** | < 1.0 ms | **PASS** |
| avg_changed_update_ms (path row change) | **1.1781** | bounded single change | **PASS** |
| labels (total) | 257 | 256 + 1 change probe | — |
| tessellated_vertex_count (diag) | **25** | > 0 warped | recorded |
| aggregate_repack_count | **0** | 0 on noop | **PASS** |
| path_table_upload_count | 2 | init + change | **PASS** |
| shape_rebuild delta (noop frames) | **0** | 0 | **PASS** |

Note: warped binding uses 256 labels (not 5k) — tessellation/path/warp cost scales with label count; binding config targets 100–500 warped labels per LR9 ladder.

## Studio seam profile retention

- CI-scale `studio_shell_label_profile_uses_typeface_runtime` unchanged and green.
- No binding-scale Studio profile required — shell label count is presentation-bound, not 5k volume.

## Combined proof retention

- `combined_msdf_style_deform_path_warp_smoke_still_passes` — green (LR6D #891 retention).

## Buffer residency retention

- `atlas_style_deform_path_warp_buffer_residency_still_passes` — green (LR6D style/atlas/path-warp residency).

## No-op / changed-update behavior

- Flat 5k: zero shape/instance rebuild across 60 noop frames after warmup.
- Numeric 5k: zero shape rebuild / zero repack on noop phase; fixed-width patch on damage frames.
- Warped 256: tessellation/path counts stable across noop; single bounded rebuild on path table change.

## GPU residency / CPU surfacing audit

Import/staging only — binding profiles measure Update-path CPU orchestration on headless Bevy harness (no render sub-app draw queue).

- **Allowed CPU:** initial label build; aggregate patch; numeric lane patch; path/warp table upload on generation change
- **Forbidden:** per-frame shaping of unchanged labels; per-frame manifest/SVG reload
- Deviations: flat 5k **max_noop** single-frame spike **1.0086 ms** on validation host (avg **0.5037 ms** meets budget)

## Validation

```text
cargo test -p simthing-tools --test typeface_lr9
cargo test -p simthing-tools --test typeface_lr9 -- --ignored --nocapture
cargo test -p simthing-tools --test semantic_free_guard
git diff --check
```

## Files changed

- `crates/simthing-tools/tests/typeface_lr9.rs` — warped binding profile + perf diagnostic eprintln
- `docs/tests/typeface_lr9_binding_perf_results.md` (this file)
- Docs: `typeface_lr9_results.md`, evidence index, production log, ladder

## Known gaps

- Flat 5k max noop frame slightly exceeds 1 ms guidance on validation host (1.0086 ms); avg meets budget.
- Damage churn avg ~2.5 ms/frame at 5k scale — separate from settled noop budget; consistent with LR5T-class fixed-width lane.
- Full render-subapp draw-call binding at 5k not re-measured here (CPU harness `queued_draw_count=0`).
- Typeface track closure deferred to Codex DA review.

## DA recommendation

Recommend **PROBATION** retention with binding evidence recorded. No-op budgets met on avg for flat 5k, numeric 5k, and warped 256. Note max flat noop spike for Codex review. **Do not** self-approve LR9 or close the typeface track.

## Next recommended action

Codex DA review of binding measurements; owner decision on track closure after LR9 + perf re-review.
