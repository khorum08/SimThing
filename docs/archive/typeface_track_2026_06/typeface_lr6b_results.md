# TYPEFACE-LR6B-GPU-STYLE-TABLE-EFFECTS-0 Results

## Status

PASS — GPU style table buffer, shader-side style lookup, gradient/pulse/outline/glow, layered icon style slots; LR0–LR6A regressions preserved. **DA APPROVED** after buffer residency remediation `TYPEFACE-LR6B-STYLE-BUFFER-RESIDENCY-0R` (#887). PR #886, merge `54e226471c`, post-merge evidence `78456f666d`.

## PR / branch / merge

- Branch: `typeface-lr6b-gpu-style-table-effects-0`
- PR: #886
- Merge SHA: `54e226471c`
- Post-merge evidence: `78456f666d`

## LR6A-ICON closeout

- `TYPEFACE-LR6A-ICON-VECTOR-GEOMETRY-0R` promoted to **ACCEPTED / closed**
- PR #885, merge `a3f7dcd30d`, post-merge evidence `35e2c3a6cc`
- Evidence index and ladder updated

## Mission

Implement GPU-resident style table and shader-side gradient/effect path for glyphs, labels, numeric labels, and layered icons without CPU per-frame style math.

## Style table design

- `TextStyleRowGpu` / `TextStyleRow` / `TextStyleTable` (max 32 slots)
- Slot 0 default identity (fill white, opacity 1, no gradient)
- `TextStyleTableResource` uploads row buffer only when `rows_dirty`
- `ExtractedTextStyleTable` extracted to render world (rows + globals time)

## Instance metadata

- `GlyphInstanceGpu.style_params`: `[style_slot, role_slot, reserved, reserved]`
- Existing raster/SDF/MSDF `sdf_params` unchanged
- `TextLabel.style_slot` + `with_style_slot()`

## Shader style path

- `text_instanced.wgsl` bind group 3: `style_globals` + `style_rows` array
- Fragment: style row lookup by `style_params.x`, fill/opacity/gradient in WGSL
- Raster + SDF/MSDF paths share style fill; SDF/MSDF adds outline/glow effects

## Gradient/effect support

- **Gradient:** `GRADIENT_MODE_LINEAR_U/V` evaluated in WGSL using instance `local_uv`
- **Pulse:** shader-side `sin(time * frequency + phase) * amplitude` modulates opacity
- CPU uploads endpoints/colors/mode only — no per-glyph gradient bake

## Outline/glow disposition

**GPU-side on SDF/MSDF instances.** `apply_sdf_effects` uses distance coverage + `outline_width` / `glow_radius` from style row. Raster glyphs/icons receive fill/opacity/gradient only (documented SDF-only outline/glow).

## Pulse disposition

**Implemented shader-side** via `style_globals.time` + style row pulse params. Globals time updates each frame without full style-table row reupload.

## Layered icon style slots

- `IconSet::build_layered_icon_style_instances` builds per-role instances with distinct `style_slot` + `role_slot`
- Uses `IconStyleLayerRef` raster tiles from LR6A-ICON — no raw SVG at runtime
- LR4 composite raster registration unchanged (`register_svg` composite tile)

## Bevy/plugin path

- `SimthingToolsTextPlugin` initializes `TextStyleTableResource`, `TextStyleDiagnostics`, `ExtractedTextStyleTable`
- `sync_style_table_rows_if_changed` uploads rows on change only
- Label rebuild keyed on `TextLabel` change, not stable style table

## Shader smoke proof

- `wgpu_styled_instanced_text_smoke` exercises production-style shader with style table
- REAL_ADAPTER_OBSERVED: style color tint, gradient horizontal variation, MSDF+style
- ADAPTER_SKIPPED recorded when no adapter

## Cache / upload behavior

- `style_table_upload_count` increments only on row dirty sync
- `style_table_cache_hit_count` when rows unchanged across sync attempts
- Stable style table does not trigger label instance rebuild

## Performance / no-regression proof

- LR5 numeric damage lane structural guard passes
- Raster default slot 0 smoke matches prior draw path
- MSDF opt-in + style slot coexist on instances

## GPU residency / CPU surfacing audit

- CPU operations introduced: import-time style row authoring; style slot assignment on instances; style row buffer upload on table change; globals time orchestration; CPU oracle/smoke readback
- CPU operations removed: none (greenfield style path)
- CPU operations retained and why: LR4/LR6A import-time icon geometry/raster; label shaping on change; numeric damage table init
- Numeric production authority remains GPU-resident: **yes** — gradient, pulse, outline/glow, fill/opacity evaluated in WGSL
- Deviations: none
- Next GPU-residency debt: per-role icon MSDF + style in production Bevy draw; Studio style binding (LR8)

## Tests

`crates/simthing-tools/tests/typeface_lr6b.rs` (14 tests):

- `style_table_default_slot_preserves_existing_render`
- `style_table_uploads_only_when_changed`
- `msdf_label_uses_style_slot_in_instance_data`
- `raster_label_uses_style_slot_without_msdf_regression`
- `shader_smoke_style_color_draws_nonzero_pixels`
- `shader_smoke_gradient_changes_pixels_across_glyph`
- `sdf_outline_or_glow_is_gpu_side_or_formally_deferred`
- `pulse_is_shader_side_or_formally_deferred`
- `layered_icon_roles_map_to_distinct_style_slots`
- `layered_icon_style_does_not_require_raw_svg_runtime`
- `icon_msdf_or_role_raster_style_path_preserves_lr4_composite_fallback`
- `lr5_numeric_damage_lane_still_passes`
- `gpu_residency_audit_documented_for_lr6b`
- `msdf_smoke_with_style_slot_still_draws`

## Validation

```text
cargo fmt -p simthing-tools -p simthing-workshop -- --check
cargo check -p simthing-tools
cargo check -p simthing-workshop
cargo test LR0–LR6A + typeface_lr6b + semantic_free_guard
git diff --check
```

## Files changed

- `crates/simthing-tools/src/style.rs` — new
- `crates/simthing-tools/src/shaders/text_instanced.wgsl`
- `crates/simthing-tools/src/text_render.rs`
- `crates/simthing-tools/src/bevy.rs`
- `crates/simthing-tools/src/icons.rs`
- `crates/simthing-tools/src/wgpu_smoke.rs`
- `crates/simthing-tools/tests/typeface_lr6b.rs`
- docs ladder/evidence/log/results

## Boundary / non-goals

No LR6C deformation, text-on-path, warp, TTF export, COLRv1, icon-font manifest, Studio integration, ScenarioSpec/RF/STEAD changes.

## Known gaps

- Production mixed text+icon draw still uses composite raster tile (not per-role styled layers)
- Per-role icon MSDF + style in Bevy production path deferred
- Style table row buffer residency — **remediated by #887** (LR6B DA APPROVED)

## DA recommendation

**DA APPROVED** after #887 buffer residency remediation and Codex review.

## Next recommended action

LR6C atlas residency + parametric deformation; LR6D text-on-path per ladder priority.
