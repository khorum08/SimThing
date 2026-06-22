# TYPEFACE-LR6D-COMBINED-MSDF-DEFORM-PROOF-0R Results

## Status

PASS — raw-wgpu smoke now proves MSDF + style + deform + path + warp composition with visible output and centroid shift vs flat MSDF control. **PROBATION / DA remediation** — recommend DA approval of LR6D after review; not self-approved.

## PR / branch / merge

- Branch: `typeface-lr6d-combined-msdf-deform-proof-0r`
- PR: #891
- Merge SHA: `ffc4bb6891`

## DA HOLD being remediated

LR6D (#890) landed path/warp tables and arc/lattice smoke proofs, but `msdf_style_deform_path_warp_combined_smoke_draws_nonzero_pixels` used a **raster** glyph without deform slot. That did not prove the full composed shader stack.

## Baseline from LR6D

Preserved from #890:

- GPU path/warp tables and vertex-shader evaluation
- `TextLabel::with_path_slot` / `with_warp_slot`
- Persistent path/warp GPU resources
- LR6C `source_uv` atlas sampling split
- Arc/bezier and lattice warp distribution proofs

## Combined instance proof

`combined_msdf_style_deform_path_warp_instance_sets_all_modes` asserts:

```text
sdf_params.x == DISTANCE_FIELD_RENDER_MSDF (2.0)
style_params.x > 0
deform_params.x > 0
path_params.x > 0
warp_params.x > 0
```

Flat control: MSDF + style slot 1, deform/path/warp slot 0.

## Combined shader smoke proof

- Extended `WGPU_DEFORM_SMOKE_SHADER` fragment path with `sdf_coverage()` (raster + SDF + MSDF)
- `combined_msdf_style_deform_path_warp_smoke_draws_nonzero_pixels` — REAL_ADAPTER_OBSERVED colored pixels
- Uses `test_deform_table_skew()`, `test_style_table_solid_red()`, arc path slot 1, lattice2x2 warp slot 1

## Distribution-difference proof

`combined_msdf_style_deform_path_warp_changes_distribution_vs_flat_msdf` draws flat MSDF styled control vs full combined instance; both have visible pixels; vertical centroid differs by > 4 px.

## Source atlas UV preservation

- Production shader: `mix(uv_rect, source_uv)`; `out.local_uv = source_uv`
- Path/warp smoke shader: same vertex UV policy
- `combined_path_warp_preserves_source_atlas_uv` + existing `path_warp_preserves_source_atlas_uv`

## Gradient coordinate policy

Style gradients use `in.local_uv` which is `source_uv` in the path/warp smoke vertex shader (unchanged LR6C policy). Documented here and tested in `combined_gradient_uses_source_uv_under_path_warp`.

## Performance / no-regression proof

- No new per-frame CPU work
- Path/warp/deform/style/atlas buffer residency tests unchanged
- LR0–LR6D matrix green after remediation

## GPU residency / CPU surfacing audit

- **Allowed CPU:** MSDF tile generation at test setup; table authoring; smoke readback
- **Forbidden:** per-frame path/warp/deform/style evaluation on CPU
- **GPU owns:** MSDF median reconstruction, style fill, LR6C deform, LR6D path/warp, stable atlas UV, instanced draw
- Deviations: none

## Tests

`crates/simthing-tools/tests/typeface_lr6d.rs`:

- `combined_msdf_style_deform_path_warp_instance_sets_all_modes`
- `combined_msdf_style_deform_path_warp_smoke_draws_nonzero_pixels`
- `combined_msdf_style_deform_path_warp_changes_distribution_vs_flat_msdf`
- `combined_path_warp_preserves_source_atlas_uv`
- `combined_gradient_uses_source_uv_under_path_warp`
- `msdf_style_deform_path_warp_combined_smoke_draws_nonzero_pixels` (delegates to MSDF combined instance)

## Validation

```text
cargo fmt -p simthing-tools -p simthing-workshop -- --check
cargo check -p simthing-tools
cargo test -p simthing-tools --test typeface_lr6d
cargo test -p simthing-tools --test typeface_lr6c
cargo test -p simthing-tools --test semantic_free_guard
git diff --check
```

## Files changed

- `crates/simthing-tools/src/wgpu_smoke.rs` — MSDF/SDF fragment in path/warp smoke shader
- `crates/simthing-tools/tests/typeface_lr6d.rs` — combined proof tests
- Docs: this file, ladder, evidence index, production log, `typeface_lr6d_results.md`

## Boundary / non-goals

- No LR7 manifest/export/Studio/Scenario changes
- No SampledPolyline implementation
- Proof-only remediation

## DA recommendation

Recommend **DA APPROVED** for LR6D after Codex review of combined MSDF/deform/path/warp smoke evidence. Do not self-approve.

## Next recommended action

Codex DA sign-off on LR6D; then LR7 icon-font manifest when owner supplies PUA/icon source set.
