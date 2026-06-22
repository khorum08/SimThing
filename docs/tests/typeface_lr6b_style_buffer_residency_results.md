# TYPEFACE-LR6B-STYLE-BUFFER-RESIDENCY-0R Results

## Status

PASS — render-world style globals/rows buffers and bind group are persistent; rows upload on `rows_generation` only; globals/time written as small buffer writes each prepare. **PROBATION / DA remediation complete** — pending DA review for LR6B approval; not self-approved.

## PR / branch / merge

- Branch: `typeface-lr6b-style-buffer-residency-0r`
- PR: opened after push
- Merge SHA: recorded in post-merge evidence commit

## DA HOLD being remediated

LR6B (#886) introduced GPU style table and shader-side effects but recreated `text_style_globals_buffer`, `text_style_rows_buffer`, and `text_style_bind_group` every render prepare. This PR closes that residency gap before LR6C.

## Baseline from LR6B

- `TextStyleTableResource` / `ExtractedTextStyleTable` with main-world `rows_dirty` gating preserved
- Shader semantics unchanged (`text_instanced.wgsl` style lookup, gradient, pulse, SDF outline/glow)
- Layered icon style slots via `IconSet::build_layered_icon_style_instances` unchanged

## Persistent style GPU resource

- `TextStyleGpuResource` owns persistent `globals_buffer`, `rows_buffer`, `bind_group`, and `rows_generation`
- Created on first prepare when extracted style table is present
- Removed only when extracted style table is absent

## Buffer / bind-group versioning

- **Globals:** `write_buffer` each render prepare (time/slot_count from extracted globals)
- **Rows:** `write_buffer` only when `ExtractedTextStyleTable.rows_generation` changes
- **Bind group:** created once with persistent buffer bindings; reused while layout/buffers valid

## Render-world diagnostics

`TextStyleRenderDiagnostics`:

- `globals_buffer_create_count` / `globals_buffer_write_count`
- `rows_buffer_create_count` / `rows_buffer_write_count`
- `style_bind_group_create_count` / `style_bind_group_reuse_count`

Accessible via `text_style_render_diagnostics(&app)` from integration tests.

## Style behavior preservation

- Slot 0 identity default unchanged
- Solid fill, linear U/V gradient, pulse via globals time preserved
- SDF/MSDF outline/glow and raster fill/opacity/gradient unchanged
- Layered icon role style slots unchanged
- LR5 numeric damage lane structural guard passes

## Shader smoke proof

- REAL_ADAPTER_OBSERVED: styled solid red, gradient horizontal variation (existing LR6B smoke tests)
- ADAPTER_SKIPPED when no adapter

## Performance / no-regression proof

- LR0–LR6B + semantic_free_guard pass
- No-op style-table frames: buffer create counts stable; bind group reuse increments
- Style row change: single rows-buffer write per generation bump

## GPU residency / CPU surfacing audit

- CPU operations introduced: none (render prepare now writes existing GPU buffers)
- CPU operations removed: per-prepare full style buffer + bind group allocation
- CPU operations retained and why: main-world row authoring; `rows_dirty` sync; small globals orchestration; diagnostics/smoke readback
- Numeric production authority remains GPU-resident: **yes**
- Deviations: none
- Next GPU-residency debt: atlas bind group still recreated each prepare (pre-existing; out of scope)

## Tests

`crates/simthing-tools/tests/typeface_lr6b.rs` extended with buffer residency tests:

- `style_gpu_buffers_created_once_then_reused`
- `style_rows_buffer_uploads_only_when_rows_generation_changes`
- `style_globals_buffer_updates_without_rows_reupload`
- `style_bind_group_reused_when_layout_and_buffers_unchanged`
- `stable_style_table_noop_frames_do_not_recreate_buffers`
- `style_table_change_uploads_rows_once`
- smoke + layered icon + numeric damage + semantic guard aliases

## Validation

- `cargo fmt --check`, `cargo check`, LR0–LR6B tests, `semantic_free_guard`
- Style buffer churn guard: no per-prepare `create_buffer_with_data` for style rows in hot path

## Files changed

- `crates/simthing-tools/src/text_render.rs` — persistent `TextStyleGpuResource`, diagnostics
- `crates/simthing-tools/src/lib.rs` — exports
- `crates/simthing-tools/tests/typeface_lr6b.rs` — residency tests
- docs: ladder, evidence index, production log, LR6B results note, this file

## Boundary / non-goals

- No LR6C deformation, text-on-path, export, Studio, ScenarioSpec changes
- Atlas bind group persistence deferred

## DA recommendation

Recommend **DA approval of LR6B** after review of buffer residency diagnostics and test evidence. LR6C remains blocked until LR6B approved.

## Next recommended action

DA/Codex review of #886 + this remediation; if approved, activate LR6C deformation rung.
