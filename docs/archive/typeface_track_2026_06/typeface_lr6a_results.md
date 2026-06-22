# TYPEFACE-LR6A-PRODUCTION-MSDF-WIRING-0R Results

## Status

PASS — production MSDF opt-in wired through Bevy plugin; glyph-ID MSDF generation corrected via patched `msdf-font`; icon MSDF formally deferred; LR0–LR6 regressions preserved. **PROBATION / DA remediation** — recommend LR6 review for DA approval after Codex sign-off.

## PR / branch / merge

- Branch: `typeface-lr6a-production-msdf-wiring-0r`
- PR: #884
- Merge SHA: `3fda0eeaf0`

## DA HOLD being remediated

LR6 (#883) landed MSDF foundation but production Bevy labels remained raster-by-default, glyph MSDF used ASCII/Latin-1 codepoint reverse lookup, and icon MSDF was an vague deferral.

## Baseline from LR6

| Item | LR6 (#883) |
|---|---|
| DistanceFieldAtlasCore | yes (standalone tests) |
| Production Bevy MSDF opt-in | no |
| Glyph ID MSDF | codepoint scan fallback |
| Icon MSDF | vague defer |
| Raw-wgpu smoke | helper-only |

## Glyph-source correction

- Vendored patch to `msdf-font` 0.3.1 adds public `GlyphBuilder::build_glyph_id(GlyphId)`.
- Production and test paths call `build_glyph_id` directly — no `char_for_glyph_id` / ASCII scan in `simthing-tools` src.
- Invalid/out-of-range glyph IDs return `DistanceFieldError::MissingOutline` explicitly.
- Ligature / shaped glyph IDs validated via fixture `fi` / `ﬁ` test.

## Production MSDF opt-in path

- `TextLabel.render_mode: TextLabelRenderMode` — `Raster` (default), `Sdf`, `Msdf`.
- `TextLabel::raster()` / `TextLabel::msdf()` helpers.
- `rebuild_changed_labels` resolves distance-field tiles via `get_or_generate_glyph_into_shared_atlas` on label change only.
- `GlyphInstanceGpu.sdf_params` set to mode 1/2 for opt-in labels.
- `DistanceFieldDiagnostics.production_msdf_label_count` / `production_msdf_instance_count` track opt-in builds.

## Raster default preservation

- Default `TextLabelRenderMode::Raster`; spawn helpers unchanged for LR5 benches.
- Raster instances keep `sdf_params.x = 0`.
- Numeric damage lane unchanged (raster).

## Icon MSDF disposition

**Deferred** — see `docs/tests/typeface_lr6a_icon_msdf_deferred.md`.

Blocker: `IconVector` IR lacks normalized curve geometry for `msdf-font`. LR4 raster icon path preserved; `get_or_generate_icon_msdf` returns explicit `IconDeferred`.

## Bevy plugin proof

| Check | Result |
|---|---|
| Raster + MSDF labels spawned | PASS |
| MSDF instances `sdf_params.x = 2` | PASS |
| Raster instances `sdf_params.x = 0` | PASS |
| No-op MSDF label: no DF regenerate | PASS |
| Mixed labels share aggregate | PASS |
| Numeric damage lane structural guard | PASS |

## Raw-wgpu smoke proof

- LR6 helper smoke: `docs/tests/typeface_lr6_sdf_smoke.png` (REAL_ADAPTER_OBSERVED when adapter present).
- Production opt-in instances smoke via `msdf_opt_in_raw_wgpu_smoke_draws_nonzero_pixels` — REAL_ADAPTER_OBSERVED or ADAPTER_SKIPPED.

## Cache behavior

- Same glyph + px bucket hits cache on shared atlas path.
- No-op frames do not increment `glyph_msdf_generate_count`.

## Performance / no-regression proof

- LR0–LR5 tests pass.
- No-op MSDF label: zero shape/instance rebuild delta; zero MSDF generate delta.
- MSDF generation on cache miss / label change only.

## GPU residency / CPU surfacing audit

- CPU operations introduced:
  - Patched import-time glyph-ID MSDF generation (`build_glyph_id`).
  - Shared-atlas packing on cache miss (`get_or_generate_glyph_into_shared_atlas`).
  - Opt-in render mode metadata on `TextLabel`.
- CPU operations removed:
  - ASCII/Latin-1 `char_for_glyph_id` reverse lookup in production src.
- CPU operations retained and why:
  - Raster atlas path (default labels, icons, numeric lane).
  - Shaping on label change (boundary orchestration).
  - Import-time/cache-miss distance-field generation only.
- Numeric production authority remains GPU-resident: **yes**
- Deviations: **none**
- Next GPU-residency debt: icon MSDF outline extraction; optional numeric lane MSDF opt-in; in-Bevy SDF readback PNG.

## Tests

`crates/simthing-tools/tests/typeface_lr6.rs` — LR6 foundation + LR6A production tests (19 total).

## Validation

```text
cargo fmt/check, cargo check, LR0–LR6 tests, semantic_free_guard, git diff --check
```

## Files changed

- `vendor/msdf_font/` — patched `build_glyph_id`
- `Cargo.toml` — `[patch.crates-io]` msdf_font
- `crates/simthing-tools/src/msdf.rs`
- `crates/simthing-tools/src/bevy.rs`
- `crates/simthing-tools/src/lib.rs`
- `crates/simthing-tools/tests/typeface_lr6.rs`
- `docs/tests/typeface_lr6a_results.md`
- `docs/tests/typeface_lr6a_icon_msdf_deferred.md`
- `docs/design_typeface_ladder.md`
- `docs/tests/current_evidence_index.md`
- `docs/workshop/studio_production_log.md`
- `THIRD_PARTY_LICENSES.md`

## Boundary / non-goals

No LR6B style tables, deformation, text-on-path, LR7 manifest, Studio/game, Scenario/RF/STEAD changes.

## DA recommendation

Accept LR6A remediation at **PROBATION**; recommend **LR6 DA approval** pending Codex review of production opt-in proof, glyph-ID correction, and icon deferral disposition.

## Next recommended action

DA review LR6A; accept or reject icon MSDF deferral; proceed to LR6B style table only after LR6 DA sign-off.
