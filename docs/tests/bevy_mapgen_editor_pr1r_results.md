# BEVY-MAPGEN-EDITOR-PR1R — repair Studio shell contract

**Classification: PROBATION until DA approval**

## Artifact lifecycle audit

| Artifact | Classification | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Guardrail row + PROBATION amendment for PR1R |
| `docs/tests/bevy_mapgen_editor_pr1_results.md` | PROBATION | PR1 baseline; addendum references PR1R repairs |
| `docs/tests/bevy_mapgen_editor_pr1r_results.md` | PROBATION | This report |
| `crates/simthing-mapeditor/` | PROBATION | Shell contract repairs |

## Old PR1 bugs fixed

| Bug | PR1R fix |
|---|---|
| Disabled New/Load/Save could not trigger warning dialog | Greyed **active** buttons via `inactive_button`; clicks invoke warning dialog |
| Inactive presets could not trigger warning | Same inactive-button pattern |
| Left title showed `SimThing Studio` before generation | Title bar empty pre-generation; product id moved to subtle footer |
| Auto-collapse used impossible `20% > 50%` test | Real rule: collapse when `screen_w * 0.20 < 320px` |
| Overhead/Reset buttons no-op | Wired to `snap_overhead` / `reset_camera_after_generation` |
| RMB orbit spun by fixed frame delta | RMB + `MouseMotion` delta via `apply_orbit_delta` |
| Hyperlane fade was global per-network material | Three depth buckets (near/mid/far) with separate meshes/materials |
| Crate-level `#![cfg(windows)]` on main hid non-Windows fallback | Per-function `#[cfg(windows)]` / `#[cfg(not(windows))]` main |
| Left panel docked flush to left edge, ~33% width | Detached floating panel: 20% width, 3% margin, 5% corner radius |

## Windows live panel defect

PR1 placed the left panel at `(0,0)` with width clamped up to 50% and default settings `last_panel_width: 0.28`, producing a docked sidebar ~28–33% of screen. PR1R uses explicit floating layout from `panel_layout.rs`.

## New floating panel behavior

- `panel_margin = 3%` of screen width/height on all sides
- `panel_width = 20%` of screen width (never exceeds 20%)
- `corner_radius = max(8px, panel_width * 5%)`
- Auto-collapse when `screen_w * 0.20 < 320px`
- Collapsed tab at 3% left/top margin with `>>` expand control
- Opacity 50% idle / 80% hover (eased)

## Warning dialog behavior

Greyed controls use styled active buttons that capture clicks. New/Load/Save, inactive presets, and deferred footer controls all invoke `WarningDialogModel`.

## Camera / orbit repairs

- Overhead (O) and Reset (R) buttons and hotkeys call shared pure helpers in `camera_control.rs`
- RMB alone does not rotate; RMB + mouse delta adjusts yaw/pitch with pitch clamp

## Hyperlane depth fade status

PR1R implements **three depth buckets** (near/mid/far) by normalized midpoint distance from galactic center at view-model build time. Each bucket renders as an independent `LineList` mesh with its own material. Near lanes are light blue/high alpha; far lanes are dark grey-blue/low alpha. Per-bucket alpha further adjusts by camera distance to bucket average.

## Star rendering status

PR1/PR1R uses **emissive unlit spheres** as interim star markers. Starburst sprites remain PR2/visual polish — not claimed as landed.

## Architecture (unchanged)

```text
MapGenerator typed output → StudioSession → StudioGalaxyViewModel → Bevy render + egui panels
```

Structural `(col,row)` remain authoritative. Bevy transforms and Y-height are render-only.

## Tests added

30 unit tests in `simthing-mapeditor` including:

- `panel_layout`: floating geometry, margins, collapse, title bar rules
- `camera_control`: overhead, reset, mouse delta orbit, pitch clamp
- `hyperlane_buckets`: near/mid/far classification, alpha ordering
- `dialog`: inactive control/preset/New-Load-Save warning reachability

## Commands run

```text
cargo fmt --all -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test stead_spatial_contract_guards
git diff --check
```

## Files changed

- `crates/simthing-mapeditor/src/panel_layout.rs` (new)
- `crates/simthing-mapeditor/src/camera_control.rs` (new)
- `crates/simthing-mapeditor/src/hyperlane_buckets.rs` (new)
- `crates/simthing-mapeditor/src/app/ui.rs`
- `crates/simthing-mapeditor/src/app/camera.rs`
- `crates/simthing-mapeditor/src/app/galaxy_render.rs`
- `crates/simthing-mapeditor/src/app/mod.rs`
- `crates/simthing-mapeditor/src/view_model.rs`
- `crates/simthing-mapeditor/src/dialog.rs`
- `crates/simthing-mapeditor/src/settings.rs`
- `crates/simthing-mapeditor/src/main.rs`
- `docs/…`

## Deferred features

- Save/load/new sessions (warning-only)
- Star/hyperlane selection (PR2)
- Live SimThing simulation
- Starburst sprites
- Non-Windows support
- Clausewitz UI import / WebView/CSS

## DA status

**PROBATION — not pre-filed for DA approval.**

## Merge

- PR [#727](https://github.com/khorum08/SimThing/pull/727) — commit `ef5e0ec0`, merge `71d60cc0`
