# STUDIO-STAR-NAMEPLATE-SCREEN-BILLBOARD-0 Results

## Status

PROBATION / visual remediation — screen-companion placement merged; owner visual sign-off pending.

## PR / branch / merge

- Branch: `codex/studio-star-nameplate-screen-billboard-0` (deleted after merge)
- PR: #910
- Merge: `05920a6d40`

## Root cause

#909 removed the arbitrary 3× nameplate multiplier and added a world-space legibility fade, but labels still used world-perspective glyph quads (`anchor + camera_right * world_width + camera_up * world_height`). At galaxy overview that projection compresses raster glyphs into dash/stroke artifacts regardless of fade tuning.

## Fix

- Added `WorldTextPlacementMode::ScreenCompanion` on `WorldTextBillboard`; Studio star nameplates use it exclusively.
- Packed GPU instances with `natural_run_aspect` (`run_width / run_height`) and a screen-mode sentinel in `size_params.w`.
- Extended `text_instanced.wgsl` WORLD_TEXT vertex path:
  - project star anchor to clip space,
  - measure rendered star blur diameter in screen pixels,
  - place glyph offsets in screen pixels below the star envelope,
  - hard-hide labels when projected height `< 12 px`.
- Preserved aggregate GPU world-text renderer and generic `WorldPerspective` path for future world text.

## Screen-space placement semantics

```text
label_height_px = projected rendered star blur diameter at current depth
offset_px.x = local_xy.x * label_height_px * relative_width
offset_px.y = below star envelope + gap + glyph-local y
clip = project(anchor) + screen_px_to_clip(offset_px)
alpha = 0 when label_height_px < 12
```

Camera movement remains GPU-side; CPU rebuild only on galaxy/settings/text/atlas changes.

## Width/height behavior

- Height capped by projected star blur envelope (100% of rendered blur diameter).
- Width flexes with string length via normalized glyph `local_xy.x` and packed `natural_run_aspect`.
- `nameplate_relative_width` is an x-scale multiplier, not a fixed plate width.

## Studio boot-safety constraints preserved

- `SimthingToolsTextPlugin::world_text_only()` unchanged
- `without_lut_d3_view_fix()` unchanged
- No `Camera2d`, LUT mutation, offscreen LUT/D3 workaround, render-plugin surgery, forced discrete GPU, or egui/window boot-order changes
- Single aggregate `WorldTextDrawEntity` path preserved

## Visual smoke

Release/debug Studio on Windows Vulkan (RTX 4080 Laptop GPU):

- Boot smoke: no panic, LUT, or shader pipeline errors in native-window runs.
- **Owner follow-up:** capture overview / medium / close zoom and slider sweeps (width, transparency, falloff distance/transparency) on ~2,400-star elliptical galaxy for promotion.

## Settings behavior

- Existing Settings → Star nameplates sliders unchanged.
- `sync_star_nameplate_settings_system` updates `WorldTextBillboard` on change without galaxy regeneration.
- Settings persist via `settings.ron` / `StudioConfig`.

## Focused validation only

| Command | Result |
|---|---|
| `cargo fmt -p simthing-tools -p simthing-mapeditor -- --check` | PASS |
| `cargo check -p simthing-tools --features world-text-3d` | PASS |
| `cargo check -p simthing-mapeditor` | PASS |
| `cargo test -p simthing-tools --features world-text-3d --test semantic_free_guard` | PASS |
| `cargo test -p simthing-tools --features world-text-3d world_text --lib` | PASS |
| `cargo test -p simthing-mapeditor nameplate --lib` | PASS |
| `git diff --check` | PASS |

## Tests deliberately not run

No full `cargo test -p simthing-tools`, no full `cargo test -p simthing-mapeditor`, no workspace test battery, and no nextest run were executed because this was a targeted visual placement fix.

## Remaining debts

- Owner screenshot/visual sign-off at overview, medium, close, and live slider sweeps.
- Optional telemetry field for live sampled projected label height in px (GPU-side only; deferred).

## DA recommendation

**PROBATION** — accept on boot safety, aggregate renderer preservation, and focused validation. Promote to **ACCEPTED** after owner confirms readable medium/close labels, clean overview fade (no dash field), and coherent blur-linked height/width behavior.
