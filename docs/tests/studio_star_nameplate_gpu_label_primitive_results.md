# STUDIO-STAR-NAMEPLATE-GPU-LABEL-PRIMITIVE-0 Results

## Status

FIXED (pending owner visual smoke) — the GPU screen-label primitive now renders coherent text.
Root cause was a vertex-attribute delivery defect in the 3D world-text pipeline, not LOD, star
sizing, or the screen-affine math that #906–#913 kept tuning.

## PR / merge

- Branch: `codex/studio-star-nameplate-procedural-quad-0`
- PR: (see merge commit)
- Merge: (recorded on merge)

## Why prior attempts failed

#906–#913 all assumed the glyph-quad geometry was being delivered correctly and chased
placement/LOD/size. The actual defect was upstream of all of that: the world-text glyph quads
were collapsing to a single point per glyph, which read on screen as "dash/dot debris."
Because every placement mode (world-perspective, screen-companion, GPU screen-label) feeds off
the same per-vertex quad coordinate, the artifact survived every placement-mode rewrite.

Two non-product confounders also masked the real bug during this investigation and are recorded
so they are not repeated:

1. **Borderless-window duplicate process.** `open_application` could not match the decoration-less
   Studio window, so it launched a *second* `simthing-studio.exe`. Driving Generate on one window
   while reading stderr from the other produced "0 nameplate entities" readings that looked like a
   spawn bug but were just the wrong process. Always confirm exactly one process and drive the
   process whose stderr you are reading.
2. **Self-inflicted shader break.** A debug `return vec4(1,0,0,1);` left unreachable code after it,
   which naga rejected (`Entry point fragment ... is invalid`). The whole text pipeline then failed
   to build, so `SetItemPipeline` skipped and nothing drew at all. Any "nothing renders" reading
   must be cross-checked against `pipeline_cache: failed to process shader` in the log.

## Single-label GPU primitive proof (live diagnosis)

Diagnosis was done in the live Studio with a clean single process and stderr instrumentation:

- Entities/components are correct: `placements=2400 simthing_ids=2400`, `nameplates_spawned=2400`,
  and `rebuild_world_text_instances` processes all 2400 (`both=2400` have
  `WorldTextBillboard + TextGlyphInstances`).
- The aggregate → draw-entity → extract → queue chain is healthy: `queue_world_text` queues the draw
  (`views=1 draws=1`, `instances=24000`) and `DrawWorldTextMesh` runs (`DRAWING count=24000`).
- CPU normalized glyph coordinates are correct (`norm w ≈ 0.4–0.6`, `norm h ≈ -0.62`,
  `local_x ∈ [-2.4, 2.4]`), so the run-normalization (Contract A) is not at fault.
- Despite a correct draw, each glyph collapsed to a point. A bisect that replaced the quad
  coordinate source — `vertex.uv` (loc 2) then `vertex.position` (loc 0) — left the collapse
  unchanged, proving **the slot-0 mesh vertex buffer is not delivering distinct per-vertex
  attributes to this shader**. Reconstructing the quad corner from `@builtin(vertex_index)` over a
  procedural non-indexed draw produced coherent, readable text across the whole field.

## Root cause

The world-text 3D pipeline is `MeshPipeline`-specialized and draws the shared `Rectangle` quad
through the `MeshAllocator` slab. The per-vertex mesh attributes (`position@0`, `uv@2`) did not
reach this shader as distinct corners, so `local_xy = source_uv * pos_size.zw + pos_size.xy`
evaluated to a constant per glyph — a single point. Ten points spread along the run read as the
dash/dot artifact.

## Fix

Generate the glyph quad procedurally instead of relying on mesh vertex attributes:

- Shader (`text_instanced.wgsl`): under `WORLD_TEXT`, reconstruct the unit-quad corner from
  `@builtin(vertex_index)` (two triangles, 6 corners) instead of `vertex.uv`. The screen-2D `#else`
  path keeps `vertex.uv` (Mesh2dPipeline delivers it correctly).
- Draw (`world_text.rs` `DrawWorldTextMesh`): issue a non-indexed `draw(0..6, instances)` so
  `vertex_index` is `0..5` with no base-vertex offset from the shared slab (an indexed draw adds
  `vertex_slice.range.start` as base vertex, which would corrupt the index→corner mapping). The mesh
  vertex buffer is still bound to satisfy the pipeline's slot-0 layout; its data is unused.

## Glyph coordinate contract

Unchanged and verified correct: CPU normalizes glyph x by run height (centered, Contract A) and y in
line-height units; the shader applies one screen-space affine per label
(`offset_px = (local_x * label_height_px * width_ratio, vertical_below_star + local_y * label_height_px)`).
Atlas UVs still derive from `source_uv` (`out.uv = mix(uv_rect.xy, uv_rect.zw, source_uv)`), now fed
by the procedurally reconstructed corner.

## Shader branch isolation

Unchanged: `GpuScreenLabel` (`size_params.w = -2.0`) branches before deform/path/warp; only the
quad-corner source changed.

## Depth / draw-order policy

Unchanged: world text renders in `Transparent3d` with `depth_write_enabled = false`,
`distance = -inf`, `cull_mode = None`.

## Selected-label visual proof

Live Studio confirmation: with the fix, "Force all labels" populates the 2,400-star elliptical disk
with coherent horizontal text strings positioned below each star (previously a blank/dash field). The
collapse-to-dash artifact is gone. Owner should confirm the selected-star label is readable at close,
medium, and overview zoom on the debug build.

## Unselected-label LOD behavior

Unchanged: unselected labels cull below 24 px, over density budget, alpha < 0.02, or offscreen;
focused labels floor at the 16 px readable minimum and skip the density gate.

## Studio boot constraints preserved

No changes to `SimthingToolsTextPlugin::world_text_only()`, `without_lut_d3_view_fix()`, camera,
tonemapping LUT handling, plugin/boot order, or adapter selection. Only the world-text shader quad
reconstruction and the world-text draw call were changed.

## Focused validation only

```
cargo fmt -p simthing-tools -p simthing-mapeditor -- --check        # clean
cargo check -p simthing-tools --features world-text-3d              # ok
cargo check -p simthing-mapeditor                                   # ok
cargo test -p simthing-tools --features world-text-3d --test semantic_free_guard  # 1 passed
cargo test -p simthing-tools --features world-text-3d world_text --lib             # 4 passed
cargo test -p simthing-mapeditor nameplate --lib                                   # 10 passed
git diff --check                                                    # clean
```

## Tests deliberately not run

No full `cargo test -p simthing-tools`, no full `cargo test -p simthing-mapeditor`, no workspace
battery, and no nextest run — this was a targeted GPU TypeFace primitive fix.

## Owner visual smoke

- [ ] Selected star label is readable below the selected star on the live debug build.
- [ ] Overview shows no dash/stroke debris (only LOD-gated coherent labels).

## DA recommendation

PROBATION until owner confirms the selected star label is readable and overview is debris-free on
the debug build. The structural defect (glyph-quad collapse) is fixed and the GPU TypeFace path is
preserved (simthing-tools shaping → glyph atlas → instanced GPU draw → atlas-sampling shader; not
egui text).
