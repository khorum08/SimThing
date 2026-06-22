# TYPEFACE-LR6C-ATLAS-RESIDENCY-DEFORM-0 Results

## Status

PASS — atlas bind group persistence remediated; Tier-1 parametric deformation (stretch/skew/fold/pulse-scale) evaluated in vertex shader; adaptive tessellation on deform opt-in; flat labels remain one quad. **PROBATION / DA-sensitive** — not DA-approved.

## PR / branch / merge

- Branch: `typeface-lr6c-atlas-residency-deform-0`
- PR: #888
- Merge SHA: `2f029bdb97`
- Post-merge evidence: `b21e8ad34e`
- Index finalize: `06346ed794`

## LR6B closeout

- `TYPEFACE-LR6B-GPU-STYLE-TABLE-EFFECTS-0` — **DA APPROVED** after #887
- `TYPEFACE-LR6B-STYLE-BUFFER-RESIDENCY-0R` — **ACCEPTED / closed**
- PR #887, merge `6117dd5fd5`, post-merge evidence `19ca9f8420`, index finalize `79ee674026`

## Atlas bind-group residency remediation

- `TextAtlasGpuResource` stores persistent atlas bind group + `AssetId<Image>`
- `prepare_text_atlas_bind_group` creates only when missing or atlas id changes; reuses on stable frames
- `TextAtlasRenderDiagnostics`: `atlas_bind_group_create_count`, `atlas_bind_group_reuse_count`, `atlas_bind_group_remove_count`
- Test `atlas_bind_group_created_once_then_reused` proves create=1 and reuse increments across noop prepares

## Deformation model

- `TextDeformKind`: None, Scale, Stretch, Skew, Fold, PulseScale (semantic-free)
- `TextDeformParams` + `TextDeformTable` / `TextDeformTableResource` / `ExtractedTextDeformTable`
- CPU uploads rows on `rows_dirty` only; no per-frame CPU deformation evaluation

## Instance / mesh metadata

- `GlyphInstanceGpu.deform_params: [f32; 4]` — deform_slot, tess_level, reserved
- Flat default: slot 0, tess_level 0

## Tessellation strategy

- Level 0: one quad per glyph (default)
- Level 4 (`DEFORM_TESS_LEVEL_DEFORM`): 5×5 grid when `deform_slot > 0`
- Tessellation on label/deform-mode change via `build_tessellated_glyph_mesh`; draw entity mesh switches once in `sync_draw_entity_mesh_for_deformation`
- No-op frames: `deformation_noop_reuse_count` increments; no retessellation

## Shader deformation path

- `text_instanced.wgsl`: bind group 4 `deform_rows`; `apply_parametric_deform()` in vertex shader before clip transform
- MSDF/SDF/raster fragment sampling unchanged; static atlas tiles
- Pulse-scale reads `style_globals.x` time in vertex shader only

## Bevy/plugin path

- `TextLabel::with_deform_slot(u16)`
- `TextDeformTableResource`, `TextDeformDiagnostics`, `text_deform_diagnostics(&app)`
- Style slots remain compatible with deformation slots

## Shader smoke proof

- REAL_ADAPTER_OBSERVED: `wgpu_deformed_instanced_text_smoke` — nonzero alpha pixels with stretch deform
- Skew vs flat pixel distribution diff > 16; stretch vs flat diff > 16
- MSDF + style slot + deform draws colored pixels
- ADAPTER_SKIPPED when no adapter (integration tests gate on adapter availability)

## Performance / no-regression proof

- Flat labels: tess_level 0, deform_instance_count 0
- Style buffer residency tests still pass (create=1, reuse on noop)
- LR0–LR6B + semantic_free_guard pass

## GPU residency / CPU surfacing audit

- **Allowed CPU:** deform profile/slot assignment on label change; static tessellation mesh build on deform enable; deform/style table upload on dirty/generation change; diagnostics and wgpu smoke readback
- **Forbidden (not introduced):** per-frame CPU deformation/warp/tessellation; runtime outline/MSDF regeneration; text-on-path / warp field
- **GPU owns:** parametric deformation evaluation (vertex shader), MSDF/SDF sampling, instanced draw, persistent atlas/style/deform bind groups and buffers
- Atlas bind group: persistent like style buffers (#887); create once, reuse on stable atlas id
- Deform rows buffer: persistent `TextDeformGpuResource`; upload on `rows_generation` only
- Deviations: none
- Next debt: LR6D text-on-path / warp field (out of scope)

## Tests

`crates/simthing-tools/tests/typeface_lr6c.rs`:

- `atlas_bind_group_created_once_then_reused`
- `flat_labels_remain_one_quad_no_deform`
- `deform_opt_in_sets_deform_params_or_slot`
- `deform_opt_in_tessellates_only_when_needed`
- `deform_noop_frames_do_not_retessellate`
- `deform_param_change_rebuilds_bounded_geometry_once`
- `vertex_shader_deform_smoke_draws_nonzero_pixels`
- `fold_or_skew_deform_smoke_changes_pixel_distribution` (skew + stretch pixel diff)
- `msdf_deformed_label_still_draws_with_style_slot`
- `style_table_buffer_residency_still_passes`
- `layered_icon_style_slots_still_pass`
- `lr5_numeric_damage_lane_still_passes`
- `semantic_free_guard_still_passes`
- `gpu_residency_audit_documented_for_lr6c`
- `deform_table_uploads_only_when_changed`
- `deform_gpu_buffers_created_once_then_reused`

## Validation

- `cargo fmt -p simthing-tools -p simthing-workshop -- --check`
- `cargo check -p simthing-tools`, `cargo check -p simthing-workshop`
- LR0–LR6C typeface tests + `semantic_free_guard`
- Scope guards: no simthing-spec/driver/sim/gpu/mapeditor app changes; no LR6D keywords; no runtime outline regen; CPU surfacing grep clean

## Files changed

- `crates/simthing-tools/src/deform.rs` (new)
- `crates/simthing-tools/src/text_render.rs` — atlas/deform GPU residency, diagnostics
- `crates/simthing-tools/src/bevy.rs` — deform plugin path, tessellation mesh
- `crates/simthing-tools/src/shaders/text_instanced.wgsl` — vertex deformation
- `crates/simthing-tools/src/wgpu_smoke.rs` — deformed smoke
- `crates/simthing-tools/tests/typeface_lr6c.rs` (new)
- docs: ladder, evidence index, production log, LR6B closeout, this file

## Boundary / non-goals

No text-on-path, region warp, spline layout, border-conforming nameplates, TTF export, COLRv1, icon-font manifest, Studio/game integration, ScenarioSpec/RF/STEAD changes.

## Known gaps

- Fold deformation proof on production tessellated Bevy path only; raw-wgpu smoke uses quad mesh (skew + stretch for pixel distribution proof)
- LR6C not DA-approved; probation pending Codex review

## DA recommendation

Accept LR6C at **PROBATION** after review of atlas bind-group residency, deform GPU path, and regression evidence. Do not mark DA-approved until sign-off.

## Next recommended action

Codex DA review → activate LR6D text-on-path / warp field rung per ladder priority.
