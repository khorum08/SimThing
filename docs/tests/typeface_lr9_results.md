# TYPEFACE-LR9-FINAL-PERF-GATE-0 Results

## Status

PASS — final typeface performance gate profiles flat 5k-class labels, fixed-width numeric damage lane, dynamic style rows, warped nameplates, and Studio seam labels over the Bevy typeface runtime. **PROBATION / DA-sensitive final perf gate** — binding-scale evidence recorded in `typeface_lr9_binding_perf_results.md`; not DA-approved; typeface track remains OPEN.

## PR / branch / merge

- Branch: `typeface-lr9-final-perf-gate-0`
- PR: #895
- Post-merge evidence: `7fa3a4dc91`
- Binding evidence: `docs/tests/typeface_lr9_binding_perf_results.md` (#896, merge `bda6147c95`)

## LR8 closeout

- `TYPEFACE-LR8-STUDIO-LABEL-SEAM-0` — **DA APPROVED after #894** (merge `850a216a7a`, post-merge evidence `29d50465ba`)
- `TYPEFACE-LR8-STUDIO-PLUGIN-MOUNT-0R` — **ACCEPTED / closed** (#894, index finalize `da1f1687fd`)

## Validation host / adapter

- Host: Windows x86_64 validation host (CI agent class)
- Adapter: `REAL_ADAPTER_OBSERVED` when wgpu adapter present; otherwise `ADAPTER_SKIPPED` (honest)

## Scenario matrix

| Scenario | CI scale | Binding scale (`#[ignore]`) | Proved |
|---|---|---|---|
| A — flat animated labels | 1,000 | 5,000 | no-op rebuild 0; avg no-op < 1 ms |
| B — numeric damage lane | 100 + churn | 5,000 + churn | LR5T fixed-width contract; shaping bypass |
| C — dynamic style slots | 256 styled labels | — | row upload only on generation change |
| D — warped nameplates | 64 warped | 256 (`#[ignore]` profile) | tessellation stable on noop; bounded path change rebuild |
| E — Studio seam | 32 + emitter | — | manifest bake once; no bespoke fallback |
| F — combined MSDF/style/deform/path/warp | LR6D smoke retained | — | #891 proof still passes |

## Flat 5k animated label profile

CI (`LR9_CI_CONFIG`, 1,000 labels, 20 noop frames):

- avg_noop_update_ms: **< 1.0** (budget gate pass on validation host)
- shape_rebuild_count delta across noop frames: **0**
- instance_rebuild_count delta across noop frames: **0**
- default tessellation level: **0** (flat one-quad glyphs)

Manual binding (`#[ignore] flat_5k_binding_noop_perf_profile`) — **recorded in `typeface_lr9_binding_perf_results.md`**:

| Metric | Value |
|---|---|
| avg_noop_update_ms | **0.5037** |
| max_noop_update_ms | **1.0086** |
| aggregate_repack_count | **0** |
| noop shape/instance rebuild delta | **0** |

```bash
cargo test -p simthing-tools --test typeface_lr9 -- --ignored --nocapture flat_5k_binding_noop_perf_profile
```

## Numeric damage 5k profile

CI (100 numeric labels + 10 damage frames):

- avg_noop_update_ms: **< 1.0**
- uses `NumericDamageLabel` / LR5 fixed-width lane (not generic shaping)

Manual binding (`#[ignore] numeric_damage_5k_binding_perf_profile`) — **recorded in `typeface_lr9_binding_perf_results.md`**:

| Metric | Value |
|---|---|
| avg_noop_update_ms | **0.3260** |
| max_noop_update_ms | **0.6960** |
| avg_damage_update_ms | **2.5149** |
| shape_rebuild_count (noop phase) | **0** |
| aggregate_repack_count | **0** |

```bash
cargo test -p simthing-tools --test typeface_lr9 -- --ignored --nocapture numeric_damage_5k_binding_perf_profile
```

## Dynamic style profile

- Style rows with gradient / outline / glow / pulse authored in `TextStyleTableResource`
- `style_table_upload_count` stable across noop frames
- Upload increments only when `rows_generation` advances on row change
- No CPU-side gradient/pulse/glow evaluation in Update path (GPU style table)

## Warped/curved nameplate profile

- LR6C tessellation + LR6D path/warp slots on warped labels
- `tessellated_vertex_count` stable across noop frames after settle
- `path_warp_noop_reuse_count` increments on idle frames
- Path table change triggers bounded single-frame rebuild

Binding (`#[ignore] warped_nameplate_binding_perf_profile`, 256 warped) — **recorded in `typeface_lr9_binding_perf_results.md`**:

| Metric | Value |
|---|---|
| avg_noop_update_ms | **0.0683** |
| max_noop_update_ms | **0.2443** |
| avg_changed_update_ms | **1.1781** |
| aggregate_repack_count | **0** |

## Studio seam profile

- `lr9_studio_shell_app` = `SimthingToolsTextPlugin` + `StudioTypefaceLabelPlugin` (same stack as shell mount)
- `StudioTypefaceLabel` + `StudioDamageTextEmitter` exercised
- `manifest_reload_count == 1`; `runtime_svg_parse_count == 0`
- `bespoke_text_fallback_count == 0`

## Combined MSDF/style/deform/path/warp retention

- `combined_msdf_style_deform_path_warp_smoke_draws_nonzero_pixels` (LR6D) still passes
- Adapter-backed when GPU available; otherwise structural CPU proofs remain green

## Draw-call / batch behavior

- Single aggregate draw entity (`draw_entities <= 1`) on Studio seam profile
- Instanced draw path; `queued_draw_count` proxy recorded in metrics snapshot when render sub-app present

## Buffer residency

- LR6D regressions retained: `style_table_buffer_residency_still_passes`, `atlas_bind_group_residency_still_passes`, `path_warp_bind_group_reused_on_noop_frames`
- Style/deform/path/warp buffer create/write/reuse counts captured in `Lr9MetricsSnapshot`

## No-op / changed-update behavior

- Unchanged labels: zero shape/instance rebuild across noop frames (flat + warped)
- Style row change: one generation bump + one style upload
- Path row change: bounded rebuild; noop frames reuse tessellation/buffers

## GPU residency / CPU surfacing audit

Import/staging only — perf gate measures Update-path orchestration; no per-frame manifest/SVG IO.

- **Allowed CPU:** one-time label build (import/staging); aggregate patch on numeric/value change; style/path/warp table row upload on generation change; diagnostics; profile harness
- **Forbidden:** per-frame shaping/rasterization of unchanged labels; per-frame manifest reload; runtime SVG parse; bespoke CPU text renderer fallback
- **GPU owns:** atlas/MSDF sampling, style/effect composition, deformation, path/warp evaluation, instanced draw
- Deviations: none on proved paths

## Tests

`crates/simthing-tools/tests/typeface_lr9.rs` — 17 tests (2 `#[ignore]` binding profiles):

- `lr8_closeout_records_da_approval`
- `flat_5k_noop_perf_profile_records_budget`
- `numeric_damage_5k_perf_profile_records_budget`
- `dynamic_style_rows_upload_only_on_generation_change`
- `warped_nameplate_set_noop_reuses_tessellation_and_buffers`
- `warped_nameplate_change_rebuilds_bounded_once`
- `studio_shell_label_profile_uses_typeface_runtime`
- `manifest_icon_profile_does_not_reload_manifest_per_frame`
- `combined_msdf_style_deform_path_warp_smoke_still_passes`
- `atlas_style_deform_path_warp_buffer_residency_still_passes`
- `semantic_free_guard_still_passes`
- `gpu_residency_audit_documented_for_lr9`

Harness: `crates/simthing-tools/src/lr9.rs`

## Validation

```text
cargo fmt -p simthing-tools -p simthing-workshop -p simthing-mapeditor -- --check
cargo check -p simthing-tools
cargo check -p simthing-workshop
cargo check -p simthing-mapeditor
cargo test -p simthing-tools --test typeface_lr9
cargo test -p simthing-mapeditor --test typeface_lr8
git diff --check
```

Ignored binding profiles:

```bash
cargo test -p simthing-tools --test typeface_lr9 -- --ignored --nocapture
```

## Files changed

- `crates/simthing-tools/src/lr9.rs` (new)
- `crates/simthing-tools/src/lib.rs`
- `crates/simthing-tools/tests/typeface_lr9.rs` (new)
- Docs: this file, ladder, evidence index, production log, LR8 closeout updates

## Boundary / non-goals

- No `.ttf`/`.otf` exporter, COLRv1, variable font export
- No production icon source set invention
- No ScenarioSpec/RF/STEAD/sim changes
- No typeface track self-closure or LR9 DA self-approval

## Known gaps

- Full 5k binding profiles are `#[ignore]` manual runs (CI uses 1k/100 smoke scale)
- Interactive Studio window smoke not in LR9 CI
- Production icon source set remains owner debt
- Track-level DA approval deferred to Codex after LR9 review

## DA recommendation

Recommend **PROBATION** retention for LR9 perf gate evidence. **Do not** self-approve LR9 or close the typeface track.

## Next recommended action

Codex DA review of LR9 scenario matrix + binding profiles; owner decision on track closure after review.
