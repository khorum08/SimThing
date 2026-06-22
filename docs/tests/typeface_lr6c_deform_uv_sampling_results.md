# TYPEFACE-LR6C-DEFORM-UV-SAMPLING-0R Results

## Status

PASS — vertex shader splits source glyph-local UV from deformed position coordinates; atlas/MSDF sampling uses stable `source_uv`; gradients use `source_local_uv`. **PROBATION / DA remediation** — recommend DA approval of LR6C after review; not self-approved.

## PR / branch / merge

- Branch: `typeface-lr6c-deform-uv-sampling-0r`
- PR: #889
- Merge SHA: `913b148323`
- Post-merge evidence: `2e83ff80c8`

## DA HOLD being remediated

LR6C (#888) applied `apply_parametric_deform()` to both vertex position and atlas UV interpolation. That warped the texture sampling domain and violated LR6C doctrine: glyph identity and MSDF/raster tiles stay static; deformation is mesh/vertex deformation only.

## Baseline from LR6C

Preserved from #888:

- Atlas bind-group residency (`TextAtlasGpuResource`)
- Style/deform buffer residency
- `TextDeformKind` / `TextDeformTable` / `deform_params` instance metadata
- Adaptive tessellation opt-in
- Vertex-shader stretch/skew/fold/pulse-scale for position
- LR6C integration tests and wgpu smoke harness

## Shader coordinate split

`text_instanced.wgsl` and deformed wgpu smoke shader now use:

```wgsl
let source_uv = vertex.uv;
let deformed_uv = apply_parametric_deform(source_uv, deform_slot);
// position from deformed_uv
// atlas UV from source_uv
// style gradient from source_uv via out.local_uv
```

## Atlas UV preservation

- `out.uv = mix(instance.uv_rect.xy, instance.uv_rect.zw, source_uv)`
- SDF/MSDF fragment path samples `in.uv` (stable atlas tile domain)
- Diagnostic atlas test: magenta neighbor padding must not appear under strong stretch/skew deform

## Gradient coordinate policy

- **Atlas sampling:** `source_uv` (stable tile identity)
- **SDF/MSDF reconstruction:** stable atlas UV (`in.uv`)
- **Gradient fill:** `source_local_uv` via `out.local_uv = source_uv` — gradient follows undeformed glyph-local coordinates; deformation changes screen geometry only
- No LR6D UV-warp semantics introduced

## Shader smoke proof

- REAL_ADAPTER_OBSERVED: diagnostic border atlas — zero magenta neighbor pixels under stretch/skew deform; white interior still draws
- Skew vs flat screen pixel distribution still differs (geometry deformation preserved)
- MSDF + style + deform smoke still draws; deformed MSDF alpha coverage bounded vs flat
- Gradient varies horizontally under skewed geometry using source-local U

## Performance / no-regression proof

- No new CPU paths; vertex shader adds one extra `vec2` (source vs deformed)
- Atlas/style/deform buffer residency tests unchanged
- LR0–LR6C + semantic_free_guard pass

## GPU residency / CPU surfacing audit

- **CPU:** unchanged — slot assignment, static tessellation on change, table upload on dirty, diagnostics/smoke readback only
- **GPU:** vertex-position deformation; atlas/MSDF sampling from stable UVs; style gradient from source_local_uv
- **Forbidden (not introduced):** UV warp fields, per-frame CPU deformation, runtime MSDF/outline regen, text-on-path
- **Deviations:** none

## Tests

`crates/simthing-tools/tests/typeface_lr6c.rs` extended:

- `deform_shader_preserves_source_atlas_uv` — structural shader guard
- `stretch_deform_does_not_sample_outside_tile` — diagnostic atlas magenta guard
- `skew_deform_preserves_glyph_tile_coverage` — no neighbor bleed + geometry still changes
- `msdf_deformed_label_preserves_static_tile_identity` — bounded MSDF coverage under skew
- `gradient_coordinate_policy_documented_and_tested` — doc + gradient smoke under deform

## Validation

- `cargo fmt --check`, `cargo check`, LR0–LR6C tests, `semantic_free_guard`
- Shader guard: no `mix(..., local_uv)` for atlas UV in production shader

## Files changed

- `crates/simthing-tools/src/shaders/text_instanced.wgsl`
- `crates/simthing-tools/src/wgpu_smoke.rs`
- `crates/simthing-tools/tests/typeface_lr6c.rs`
- docs: ladder, evidence index, production log, LR6C results note, this file

## Boundary / non-goals

No text-on-path, region warp, UV warp fields, TTF export, COLRv1, Studio/ScenarioSpec changes.

## DA recommendation

Recommend **DA approval of LR6C** after review of UV-sampling split and diagnostic atlas tests. LR6D remains blocked until LR6C approved.

## Next recommended action

Codex DA review → activate LR6D text-on-path / warp field per ladder.
