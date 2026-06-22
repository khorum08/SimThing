# TYPEFACE-LR6A-ICON-VECTOR-GEOMETRY-0R Results

## Status

PASS — `IconVector` promoted to normalized role-layer geometry IR; icon MSDF implemented from geometry via `PathGlyphBuilder`; per-role raster fallback ready for LR6B style slots; LR4 raster path preserved; static-SVG security intact. **PROBATION / DA remediation before LR6B.**

## PR / branch / merge

- Branch: `typeface-lr6a-icon-vector-geometry-0r`
- PR: opened after push
- Merge SHA: recorded in post-merge evidence commit

## DA blocker being remediated

LR6B style tables require icon role/layer geometry consumable without raw SVG or path signatures. LR6A (#884) deferred icon MSDF because `IconVectorLayer` stored `path_signature` only.

## Amended-track rationale

LR6B includes GPU style table + gradient/effect shader and layered icons = multiple style slots. This bridge rung closes the icon geometry blocker before LR6B is activated.

## IconVector geometry IR

- `IconPathCommand` — MoveTo, LineTo, QuadTo, CubicTo, Close
- `IconFillRule` — NonZero, EvenOdd
- `IconVectorPath` — commands, fill_rule, bounds
- `IconVectorLayer` — role, paths, bounds
- `IconVector::geometry_hash()`, `layer_roles()`, `paths_for_role()`, `to_msdf_bezpath(px)`
- `IconStyleLayerRef` — role, geometry_hash, raster_tile (per-role style-slot binding)

## Static SVG normalization

- Import-time `usvg` parse; absolute transform applied to path segments
- LR4 static-SVG-only rejection preserved (script, external href, animate, etc.)
- No raw SVG stored as runtime authority

## Role/layer preservation

- `data-simthing-role` tags preserved in deterministic DFS order
- ROLE_SVG test: Background → Accent → Outline

## Icon MSDF implementation or deferral

**Implemented.** Vendored `msdf-font` adds `PathGlyphBuilder::build_from_bezpath`. `get_or_generate_icon_msdf` consumes `IconVector::to_msdf_bezpath`, caches by geometry hash, returns `DistanceFieldTile`. Empty geometry still defers via `IconDeferred`.

## Role-layered style-slot readiness

- `IconSet::style_layers_for(codepoint, px)` returns per-role `IconStyleLayerRef` with geometry hash + dedicated raster tile
- `IconVector::paths_for_role` exposes geometry for LR6B style binding without SVG interpretation

## Raster icon regression

- LR4 composite raster registration unchanged (`register_svg` → shared atlas tile)
- Mixed text+icon instances still use raster icon tiles in production draw path

## GPU residency / CPU surfacing audit

- CPU operations introduced: import-time static SVG parsing; import-time SVG→geometry normalization; import-time per-role raster tiles; cache-miss icon MSDF generation via `PathGlyphBuilder`; CPU oracle tests
- CPU operations removed: path-signature-only icon IR as sole geometry representation
- CPU operations retained and why: LR4 resvg composite raster (production icon draw); static-SVG rejection scan (import security); geometry hash (cache key only)
- Numeric production authority remains GPU-resident: **yes** — style/effect sampling deferred to LR6B; draw uses GPU atlas + instanced shader
- Deviations: none
- Next GPU-residency debt: LR6B style table sampling on GPU; per-role MSDF opt-in for icon layers (optional enhancement)

## Tests

`crates/simthing-tools/tests/typeface_lr6a_icon_geometry.rs` (13 tests):

- `icon_vector_extracts_normalized_geometry`
- `icon_vector_preserves_role_layer_order`
- `icon_vector_rejects_dynamic_svg_still`
- `icon_vector_geometry_is_deterministic`
- `icon_vector_has_no_raw_svg_runtime_dependency`
- `icon_msdf_generates_or_explicitly_defers_after_geometry`
- `same_icon_same_px_msdf_or_layered_raster_is_cached`
- `role_layered_icon_data_is_ready_for_style_slots`
- `lr4_raster_icon_path_still_passes`
- `gpu_residency_audit_documented_for_icon_geometry`
- `icon_msdf_tile_has_distance_field_metadata`
- `icon_msdf_raw_wgpu_smoke_draws_nonzero_pixels_or_adapter_skipped`
- `icon_msdf_empty_geometry_still_defers`

Updated: `typeface_lr4.rs`, `typeface_lr6.rs`

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
cargo test -p simthing-tools --test typeface_lr6a_icon_geometry
git diff --check
```

REAL_ADAPTER_OBSERVED for icon MSDF raw-wgpu smoke (or ADAPTER_SKIPPED recorded).

## Files changed

- `crates/simthing-tools/src/icons.rs` — geometry IR, normalization, role-layer raster, style-slot refs
- `crates/simthing-tools/src/msdf.rs` — icon MSDF generation + cache
- `crates/simthing-tools/src/lib.rs` — exports
- `crates/simthing-tools/Cargo.toml` — kurbo dep
- `vendor/msdf_font/src/shape.rs`, `glyph.rs` — `Shape::from_bezpath`, `PathGlyphBuilder`
- `crates/simthing-tools/tests/typeface_lr6a_icon_geometry.rs` — new
- `crates/simthing-tools/tests/typeface_lr4.rs`, `typeface_lr6.rs` — updated
- `docs/design_typeface_ladder.md`, `docs/tests/current_evidence_index.md`, `docs/workshop/studio_production_log.md`
- `docs/tests/typeface_lr6a_icon_msdf_deferred.md` — IMPLEMENTED disposition
- `docs/tests/typeface_lr6a_icon_geometry_results.md` — this report

## Boundary / non-goals

No LR6B style table, gradients, glow/pulse, deformation, text-on-path, icon-font manifest, TTF export, COLRv1, Studio integration, ScenarioSpec/RF/STEAD changes.

## DA recommendation

Accept LR6A-ICON-VECTOR-GEOMETRY-0R at PROBATION; keep LR6B **BLOCKED** until Codex reviews this bridge; recommend LR6 + LR6A glyph MSDF DA approval chain completion.

## Next recommended action

Codex review → LR6B GPU style table + gradient/effect shader (layered icon style slots).
