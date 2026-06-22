# TYPEFACE-LR6-MSDF-ATLAS-SHADER-0 Results

## Status

PASS — MSDF/SDF atlas core, deterministic cache, WGSL SDF/MSDF sampling path, and raw-wgpu smoke proof landed; raster + LR5 numeric lane preserved. **PROBATION / DA-sensitive** — not DA-approved.

## PR / branch / merge

- Branch: `typeface-lr6-msdf-atlas-shader-0`
- PR: #883
- Merge SHA: `c08ac5ce06`

## LR5 DA closeout

- `TYPEFACE-LR5-HIGH-VOLUME-BENCH-BUDGET-0` — **DA APPROVED** after LR5R/LR5S/LR5T (#879–#882).
- `TYPEFACE-LR5-NUMERIC-DAMAGE-LANE-0R` — **ACCEPTED / closed** as final LR5 remediation (PR #882, merge `c05baef87bc2b5c24f2c77f706d3aef32665a2ea`).
- Post-merge evidence: `daaeb1795a` (LR5T results finalization).

## Mission

Establish import-time MSDF/SDF tile generation, atlas insertion, shader metadata, GPU SDF/MSDF sampling, raw-wgpu smoke proof, and Bevy instanced path compatibility without LR6B/C/D/LR7 scope.

## Implementation

- `crates/simthing-tools/src/msdf.rs` — `DistanceFieldAtlasCore`, cache keys, guillotiere packing, diagnostics.
- `GlyphInstanceGpu.sdf_params` — render mode (0 raster / 1 SDF / 2 MSDF) + `px_range`.
- `text_instanced.wgsl` + `wgpu_smoke.rs` — median MSDF reconstruction and scale-aware AA on GPU.
- `build_distance_field_instance()` helper for smoke/tests.
- Default Bevy text path remains raster (`sdf_params.x = 0`).

## MSDF/SDF generation design

Import-time only via `msdf-font` `GlyphBuilder` over TTF outlines. Cache key: `(source_id, glyph_id, px_bucket, kind)`. Guillotiere atlas packing into RGBA staging buffer (MSDF RGB in atlas; SDF alpha in atlas).

## Glyph distance-field support

**Yes** — `get_or_generate_glyph_msdf` and `get_or_generate_glyph_sdf` for TTF glyphs at quantized px buckets.

## Icon distance-field support / deferral

**Deferred (LR6A debt)** — `get_or_generate_icon_msdf` returns `DistanceFieldError::IconDeferred`; LR4 raster icon path unchanged.

## Shader path

Extended instanced text shader samples atlas, reconstructs signed distance on GPU (SDF alpha or MSDF median), applies scale-aware antialiasing via `dpdx`/`dpdy`. No gameplay/policy branching.

## Raw-wgpu smoke proof

`wgpu_sdf_instanced_text_smoke` draws MSDF instances through the SDF shader path with texture readback. Smoke PNG: `docs/tests/typeface_lr6_sdf_smoke.png` when adapter available; tests report `ADAPTER_SKIPPED` honestly otherwise.

## Cache behavior

Same glyph + px bucket hits cache (`msdf_cache_hit_count`); distinct px buckets generate distinct tiles; no per-frame generation in tests or render loop.

## Performance / no-regression proof

- Raster default path unchanged; LR3–LR5 structural tests retained.
- Numeric damage lane: shaping bypass + aggregate patch semantics unchanged (`typeface_lr6.rs` structural guard).
- MSDF generation counted only on cache miss at import/cache-miss time.

## GPU residency / CPU surfacing audit

- CPU operations introduced:
  - Import-time MSDF/SDF generation from TTF outlines (`msdf-font`).
  - Atlas packing, cache-key management, staging buffer blit.
  - CPU oracle tests for determinism/cache.
  - Raw-wgpu smoke setup/readback (test/diagnostic).
- CPU operations removed:
  - None (foundation rung).
- CPU operations retained and why:
  - Raster atlas path for default text/icons (LR3–LR5 behavior).
  - Numeric damage import-time glyph table (LR5T).
  - Shaping on text change (import/boundary orchestration).
- Numeric production authority remains GPU-resident: **yes** — edge reconstruction, AA, and draw instancing are shader-owned; CPU does not reconstruct distances per frame.
- Deviations: **none**
- Next GPU-residency debt: wire production Bevy draw to opt-in MSDF tiles (feature flag / glyph source); icon MSDF (LR6A); style/deformation remain GPU rungs LR6B+.

## Tests

`crates/simthing-tools/tests/typeface_lr6.rs`:

| Test | Result |
|---|---|
| `msdf_glyph_tile_is_deterministic` | PASS |
| `same_glyph_same_px_msdf_is_cached` | PASS |
| `different_px_bucket_gets_distinct_msdf_tile` | PASS |
| `msdf_tile_has_distance_field_metadata` | PASS |
| `icon_msdf_deferred_raster_icon_path_preserved` | PASS |
| `sdf_shader_smoke_draws_nonzero_pixels` | PASS or ADAPTER_SKIPPED |
| `raster_path_regression_still_draws` | PASS or ADAPTER_SKIPPED |
| `numeric_damage_lane_still_passes_binding_or_structural_guard` | PASS |
| `semantic_free_guard_still_passes` | PASS |
| `gpu_residency_audit_documented_for_lr6` | PASS |

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
cargo test -p simthing-tools --test typeface_lr6
git diff --check
```

## Files changed

- `crates/simthing-tools/src/msdf.rs` (new)
- `crates/simthing-tools/src/lib.rs`
- `crates/simthing-tools/src/bevy.rs`
- `crates/simthing-tools/src/text_render.rs`
- `crates/simthing-tools/src/shaders/text_instanced.wgsl`
- `crates/simthing-tools/src/wgpu_smoke.rs`
- `crates/simthing-tools/src/numeric_damage.rs`
- `crates/simthing-tools/src/icons.rs`
- `crates/simthing-tools/Cargo.toml`
- `crates/simthing-tools/tests/typeface_lr6.rs` (new)
- `Cargo.lock`
- `THIRD_PARTY_LICENSES.md`
- `docs/design_typeface_ladder.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/typeface_lr5t_results.md`
- `docs/workshop/studio_production_log.md`

## Boundary / non-goals

No LR6B style tables, LR6C deformation, LR6D text-on-path, LR7 icon-font manifest, TTF/OTF export, COLRv1, Studio/game integration, or Scenario/RF/STEAD changes.

## Known gaps

- Icon MSDF deferred to LR6A; production Bevy path still raster-by-default.
- MSDF tiles not yet wired into live label rebuild (opt-in foundation only).
- Bevy in-engine SDF readback PNG deferred (raw-wgpu smoke is primary proof).

## DA recommendation

Accept LR6 foundation at **PROBATION** pending Codex review of shader smoke, cache determinism, and GPU residency audit. Do not mark LR6 DA-approved until production draw opt-in and icon MSDF debt are dispositioned.

## Next recommended action

DA review LR6 evidence; scope LR6A icon MSDF or production MSDF glyph source wiring; proceed to LR6B style table only after LR6 PROBATION sign-off.
