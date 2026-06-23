# STUDIO-STAR-NAMEPLATE-VISUAL-CONTRACT-0 Results

## Status

PROBATION / visual remediation — visual envelope contract merged; owner visual sign-off pending.

## PR / branch / merge

- Branch: `codex/studio-star-nameplate-visual-contract-0`
- PR: pending
- Merge: pending

## Root cause

#910 introduced screen-companion placement but fed the shader from `base_scale_variation * base_blur_radius`, a partial proxy for the rendered star. That omitted `compute_star_distance_visual` core/aura layer scales (including crisp-circle `near_core_scale`), and packed `natural_run_aspect` as if the shader multiplied width by it while CPU glyph x was already normalized by run height (contract mismatch). `MIN_LEGIBLE_LABEL_PX = 12` still allowed semi-visible raster stroke debris at overview.

## Fix

- Added `WorldTextBillboard::visual_envelope_world_height` packed into `anchor_height.w` for screen companion mode.
- Studio derives envelope from `star_rendered_visual_envelope_world_diameter` (same math as `sync_star_visuals_system`: `base_scale_variation * max(core_scale, aura_radius)` at near camera).
- Shader uses projected star visual envelope with `NAMEPLATE_HEIGHT_RATIO = 1.0`; hard cull at `MIN_LEGIBLE_LABEL_PX = 18`.
- Contract A: glyph x normalized by run height; horizontal offset `local_xy.x * label_height_px * width_ratio` (no shader aspect multiply).
- Telemetry dialog reworked into collapsible sections with Nameplate debug open by default.

## Visual envelope contract

```text
visual_envelope_near_world = base_scale_variation * max(core_scale, aura_radius) at depth 0%
GPU anchor_height.w = visual_envelope_near_world
star_visual_world(depth) = visual_envelope_near_world * envelope_height_ratio(depth)
label_height_px = project(star_visual_world) * 1.0
alpha = 0 when label_height_px < 18
```

## Width/aspect contract

Contract A — CPU normalizes glyph x by run height, so `local_xy.x` spans natural run aspect. Shader width:

```text
offset_x_px = local_xy.x * label_height_px * relative_width
full_width_px ≈ natural_run_aspect * label_height_px * relative_width  (telemetry)
```

`natural_run_aspect_from_glyphs()` retained for debug telemetry only.

## Telemetry additions

Telemetry window sections (collapsing headers):

- **Nameplate debug** — open by default: counts, sample envelope/height/width/alpha, culled-too-small count
- **Performance summary** — collapsed
- **Performance isolation** — collapsed
- **Render loop / GPU / VRAM** — collapsed

## Studio boot-safety constraints preserved

- `SimthingToolsTextPlugin::world_text_only()` unchanged
- `without_lut_d3_view_fix()` unchanged
- No `Camera2d`, LUT mutation, offscreen LUT/D3 workaround, render-plugin surgery, forced discrete GPU, or egui/window boot-order changes
- Single aggregate `WorldTextDrawEntity` path preserved

## Visual smoke

Debug Studio build on Windows. **Owner follow-up:** capture overview/medium/close zoom and slider sweeps on ~2,400-star elliptical galaxy; confirm Telemetry → Nameplate debug shows sample height/width and culled count at overview.

## Settings behavior

Star nameplate sliders unchanged; live sync via `sync_star_nameplate_settings_system`; settings persist via `settings.ron`.

## Focused validation only

| Command | Result |
|---|---|
| `cargo fmt -p simthing-tools -p simthing-mapeditor -- --check` | PASS |
| `cargo check -p simthing-tools --features world-text-3d` | PASS |
| `cargo check -p simthing-mapeditor` | PASS |
| `cargo test -p simthing-tools --features world-text-3d --test semantic_free_guard` | PASS |
| `cargo test -p simthing-tools --features world-text-3d world_text --lib` | PASS |
| `cargo test -p simthing-mapeditor nameplate --lib` | PASS (8 tests) |
| `git diff --check` | PASS |

## Tests deliberately not run

No full `cargo test -p simthing-tools`, no full `cargo test -p simthing-mapeditor`, no workspace test battery, and no nextest run were executed because this was a targeted visual adapter fix.

## Remaining debts

- Owner screenshot/visual sign-off across overview/medium/close and slider sweeps.
- GPU-exact projected label height telemetry (CPU uses FOV approximation).

## DA recommendation

**PROBATION** — accept on boot safety, envelope contract clarity, and focused validation. Promote after owner confirms readable medium/close labels, clean overview cull (no dash field), blur-linked height, and useful Nameplate debug telemetry.
