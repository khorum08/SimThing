# TYPEFACE-LR6D-TEXT-ON-PATH-WARP-FIELD-0 Results

## Status

PASS — GPU-resident path/warp tables; vertex-shader text-on-path + lattice warp; stable `source_uv` atlas sampling; composes with LR6C deformation and LR6B style slots. **DA APPROVED** after combined MSDF+style+deform+path+warp proof (#891, merge `ffc4bb6891`, post-merge evidence `6a32763bdd`).

## PR / branch / merge

- Branch: `typeface-lr6d-text-on-path-warp-field-0`
- PR: #890
- Merge SHA: `c0fb11c3cb`
- Post-merge evidence: `fca5c8b555`
- Index finalize: `75da170c49`
- Combined proof PR: #891, merge `ffc4bb6891`, closeout evidence `6a32763bdd`

## LR6C closeout

- `TYPEFACE-LR6C-ATLAS-RESIDENCY-DEFORM-0` — **DA APPROVED** after UV remediation (#889)
- `TYPEFACE-LR6C-DEFORM-UV-SAMPLING-0R` — **ACCEPTED / closed**
- PR #889, merge `913b148323`, post-merge evidence `2e83ff80c8`, index finalize `581fe06f84`

## Mission

Add opt-in text-on-path and warp field/control lattice for labels while preserving LR6C stable source-atlas UV sampling. Path/warp affect vertex positions only; CPU uploads static tables on change; GPU evaluates per vertex.

## Text-on-path model

- `TextPathKind`: None, Arc, QuadraticBezier, CubicBezier, SampledPolyline
- `TextPathParams` / `TextPathRowGpu` / `TextPathTable` / `TextPathTableResource`
- CPU samples path rows on `set_row` / dirty upload only
- `path_params_for_slot(path_slot, path_u_offset, path_u_scale)` on instances

## Warp field / control lattice model

- `TextWarpKind`: None, Affine, Lattice2x2, Lattice3x3, RadialBend
- `TextWarpParams` / `TextWarpRowGpu` / `TextWarpTable` / `TextWarpTableResource`
- Lattice2x2 control points in GPU rows; vertex shader bilinear offset
- `warp_params_for_slot(warp_slot, strength_mul)` on instances

## Instance metadata

- `GlyphInstanceGpu.path_params: [f32; 4]` — path_slot, path_u_offset, path_u_scale, reserved
- `GlyphInstanceGpu.warp_params: [f32; 4]` — warp_slot, strength_mul, reserved
- Flat labels default slot 0 (no path/warp)

## Shader path/warp path

Composition order in `text_instanced.wgsl`:

```text
source_uv = vertex.uv
deformed_uv = apply_parametric_deform(source_uv, deform_slot)
local_xy = deformed_uv * size + origin
local_xy = apply_text_path(local_xy, path_slot, path_u)
local_xy = apply_warp_field(local_xy, warp_slot, source_uv)
atlas_uv = mix(uv_rect, source_uv)
style local_uv = source_uv
```

Bind groups 5 (path_rows) and 6 (warp_rows).

## Bevy/plugin path

- `TextLabel::with_path_slot` / `with_warp_slot`
- `TextPathTableResource`, `TextWarpTableResource`, `ExtractedTextPathTable`, `ExtractedTextWarpTable`
- `sync_path_table_rows_if_changed`, `sync_warp_table_rows_if_changed`
- `text_path_warp_diagnostics(&app)`

## Shader smoke proof

- REAL_ADAPTER_OBSERVED: `wgpu_path_warp_instanced_text_smoke` — arc/bezier path + lattice2x2 warp
- Path vs flat and warp vs flat pixel distribution diff > 16
- Combined MSDF + style + deform + path + warp draws colored pixels
- ADAPTER_SKIPPED when no adapter

## Source atlas UV preservation

- Atlas/MSDF sampling uses `source_uv` only; path/warp do not warp atlas UV
- Test `path_warp_preserves_source_atlas_uv` + `deform_uv_sampling_regression_still_passes`

## Gradient coordinate policy

Style gradients use `out.local_uv = source_uv` under path/warp (LR6C policy unchanged). UV-warp semantics are **not** introduced in LR6D. Documented here and tested in `gradient_policy_under_path_warp_documented_and_tested`.

## Performance / no-regression proof

- Path/warp table upload on `rows_dirty` only; cache hits on noop frames
- Path/warp bind groups and buffers created once then reused (`TextPathWarpRenderDiagnostics`)
- LR0–LR6C regressions pass; style/atlas buffer residency tests pass

## GPU residency / CPU surfacing audit

- **Allowed CPU:** path/warp slot assignment; static path row / lattice authoring on table change; table upload on dirty; diagnostics; wgpu smoke readback
- **Forbidden:** per-frame CPU text-on-path layout; per-frame CPU warp evaluation; per-frame retessellation for unchanged path/warp; runtime SVG/path parsing
- **GPU owns:** `apply_text_path`, `apply_warp_field`, composition with LR6C deformation, stable MSDF/atlas sampling, instanced draw
- Persistent `TextPathGpuResource` / `TextWarpGpuResource`; upload on `rows_generation` only
- Deviations: none

## Tests

`crates/simthing-tools/tests/typeface_lr6d.rs` — 21 tests including LR6C closeout, opt-in metadata, table dirty upload, bind-group reuse, path/warp smoke, combined MSDF smoke, regression guards, GPU residency doc check.

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
cargo test -p simthing-tools --test typeface_lr6b
cargo test -p simthing-tools --test typeface_lr6c
cargo test -p simthing-tools --test typeface_lr6d
git diff --check
```

## Files changed

- `crates/simthing-tools/src/path.rs`, `warp.rs` (new)
- `crates/simthing-tools/src/bevy.rs`, `text_render.rs`, `lib.rs`
- `crates/simthing-tools/src/shaders/text_instanced.wgsl`
- `crates/simthing-tools/src/wgpu_smoke.rs`
- `crates/simthing-tools/tests/typeface_lr6d.rs` (new)
- Docs: ladder, evidence index, production log, lr6c closeout, this file

## Boundary / non-goals

- No LR7 icon-font manifest, TTF export, COLRv1, variable fonts
- No Studio/game label seam, ScenarioSpec/RF/STEAD changes
- No UV-warp semantics (explicit future mode only)

## Known gaps

- SampledPolyline path kind table upload stub (arc/bezier/lattice proven)
- Affine/Lattice3x3 variants remain foundation-only
- Combined MSDF proof: see `typeface_lr6d_combined_msdf_deform_results.md` (remediated in 0R)

## DA recommendation

**DA APPROVED** after combined MSDF/deform/path/warp proof (#891). LR7 manifest rung active next.

## Next recommended action

LR7 icon-font manifest (`TYPEFACE-LR7-ICON-FONT-MANIFEST-0`); production icon source set remains input debt.
