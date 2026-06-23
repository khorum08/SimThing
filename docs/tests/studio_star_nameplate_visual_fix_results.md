# STUDIO-STAR-NAMEPLATE-VISUAL-FIX-0R Results

## Status

PROBATION / visual remediation — focused presentation fix merged; owner visual sign-off on live slider sweeps remains pending.

## PR / branch / merge

- Branch: `codex/studio-star-nameplate-visual-fix-0r`
- PR: pending merge record
- Merge: pending

## Root cause

Star nameplates used `STAR_NAMEPLATE_HEIGHT_FACTOR = 3.0`, inflating label world height to 300% of the star blur radius instead of the documented 100% envelope. At galaxy overview, thousands of sub-pixel world-space glyphs still drew with residual alpha, producing scene-wide dash/stroke noise detached from the star billboards.

## Fix

- Set nameplate near height to **100% of rendered star blur radius** (`base_scale_variation × base_blur_radius`) via `nameplate_near_label_height_world`.
- Removed the arbitrary 3× multiplier (`STAR_NAMEPLATE_HEIGHT_FACTOR = 1.0`).
- Extended `text_instanced.wgsl` WORLD_TEXT vertex path with GPU-side **screen-legibility fade**: labels below ~12 px projected height fade out smoothly (no CPU instance rebuild on camera movement).
- Added lightweight nameplate diagnostics to Studio performance telemetry (label count, glyph instance count, sampled effective height/alpha settings).
- Preserved aggregate GPU world-text renderer: one draw entity, persistent instance buffer, camera-facing billboard shader, change-time CPU shaping only.

## Studio boot-safety constraints preserved

- `SimthingToolsTextPlugin::world_text_only()` unchanged
- `without_lut_d3_view_fix()` unchanged
- No `Camera2d` reintroduction
- No tonemapping LUT mutation path
- No offscreen LUT / D3 texture-view workaround regression
- No Bevy render-plugin surgery
- No manual `RenderCreation` / forced-discrete-GPU experiment
- No new startup plugin changing egui or window boot order
- Single aggregate `WorldTextDrawEntity` confirmed (no per-label draw entities)

## Visual smoke

Release build `target/release/simthing-studio.exe` on Windows Vulkan (NVIDIA RTX 4080 Laptop GPU):

- Studio booted without panic, LUT error, shader validation failure, or black screen in a 25 s native-window smoke.
- Prior #906 dress rehearsal reported 2,400 systems / ~24,000 glyph instances in one draw field; aggregate path unchanged.
- **Owner follow-up:** capture overview / medium / close zoom screenshots and live slider sweeps (width 20/100/200%, transparency 25/100%, falloff distance 5/50/100%, falloff transparency 0/50/100%) for final visual sign-off.

## Settings behavior

- Existing Settings → Star nameplates sliders unchanged (relative width, base transparency, relative falloff distance, relative falloff transparency).
- `sync_star_nameplate_settings_system` still updates `WorldTextBillboard` placement on settings change without galaxy regeneration.
- Settings persist through existing `settings.ron` / `StudioConfig` path.

## Focused validation only

| Command | Result |
|---|---|
| `cargo fmt -p simthing-tools -p simthing-mapeditor -- --check` | PASS |
| `cargo check -p simthing-tools --features world-text-3d` | PASS |
| `cargo check -p simthing-mapeditor` | PASS |
| `cargo test -p simthing-tools --features world-text-3d --test semantic_free_guard` | PASS |
| `cargo test -p simthing-mapeditor nameplate --lib` | PASS (6 tests) |
| `cargo build --release -p simthing-mapeditor --bin simthing-studio` | PASS |
| `git diff --check` | PASS |

## Tests deliberately not run

No full `cargo test -p simthing-tools`, no full `cargo test -p simthing-mapeditor`, and no workspace test battery were run because this was a targeted presentation fix.

## Remaining debts

- Owner visual sign-off with screenshot evidence at overview / medium / close zoom and live slider sweeps.
- Optional debug “Show nameplate bounds” overlay deferred (not required for this narrow fix).
- Screen-legibility fade thresholds (4–12 px) may need tuning after owner review.

## DA recommendation

**PROBATION** — accept merge on boot safety, aggregate renderer preservation, focused validation green, and shader-side legibility fade. Promote to **ACCEPTED** after owner confirms readable IDs at medium/close zoom and no overview dash noise under default elliptical galaxy (~2,400 stars).
