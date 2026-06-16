# BEVY-MAPGEN-EDITOR-PR2 — selection, inspector, starburst sprites

**Classification: PROBATION until DA approval**

## Artifact lifecycle audit

| Artifact | Classification | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | PR2 PROBATION row |
| `docs/tests/bevy_mapgen_editor_pr1_results.md` | PROBATION | Unchanged lifecycle |
| `docs/tests/bevy_mapgen_editor_pr1r_results.md` | PROBATION | No owner DA signoff recorded |
| `docs/tests/bevy_mapgen_editor_pr2_results.md` | PROBATION | This report |

## PR1R DA status

No explicit owner/DA approval for #727 was recorded in this pass. PR1R remains **PROBATION**.

## Selection behavior

- `StudioSelectionState` tracks hover and click selection by `system_id`.
- Picking projects star render positions to screen space and selects nearest within 18px radius.
- Click selection persists until another star is selected, Clear button, or Esc.
- Selection is editor/view state only — does not mutate generated structural data.

## Inspector behavior

- Right panel hidden before generation (unchanged).
- After generation: galaxy status always visible.
- When a star is selected: **Selected system** section shows structural col/row, render-only height, degree, incident neighbor ids, and render-only note.
- Map quality status remains visible in galaxy status section.

## Starburst implementation

- Interim emissive spheres replaced with **billboard quads** using procedurally generated 64×64 starburst RGBA texture at startup (`starburst.rs`).
- Sprites scale from render-only `sprite_scale`; emissive strength varies for hover/selected states.
- Starburst texture/scale/emissive are presentation-only — structural `(col,row)` remain authoritative.

## Highlight behavior

- Hovered star: 1.15× scale + brighter emissive.
- Selected star: 1.35× scale + stronger emissive.
- Incident hyperlanes of selected star: independent highlight `LineList` mesh (bright blue, high alpha) overlaying near/mid/far base buckets.
- Base hyperlane depth buckets from PR1R preserved; per-segment camera-depth fade remains future polish.

## Panel hover precision

- Left panel opacity now uses left-panel `Area` response hover only (not global `is_pointer_over_area()`).

## Settings persistence

- `last_selected_system_id` and `last_camera` (`PersistedCameraState`) saved on graceful exit.

## Tests added

45 unit tests total (15 new): selection model, picking helpers, starburst render-only note, panel hover geometry helpers.

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

- `crates/simthing-mapeditor/src/selection.rs` (new)
- `crates/simthing-mapeditor/src/starburst.rs` (new)
- `crates/simthing-mapeditor/src/app/picking.rs` (new)
- `crates/simthing-mapeditor/src/app/galaxy_render.rs`
- `crates/simthing-mapeditor/src/app/ui.rs`
- `crates/simthing-mapeditor/src/app/mod.rs`
- `crates/simthing-mapeditor/src/app/window.rs`
- `crates/simthing-mapeditor/src/app/camera.rs`
- `crates/simthing-mapeditor/src/settings.rs`
- `crates/simthing-mapeditor/src/panel_layout.rs`
- `crates/simthing-mapeditor/src/lib.rs`
- `docs/…`

## Deferred features

- Hyperlane selection
- Star rename/edit
- Save/load/new sessions
- Live SimThing simulation
- Per-segment hyperlane camera-depth fade
- Clausewitz UI import / WebView/CSS

## DA status

**PROBATION — not pre-filed for DA approval.**

## Merge

- PR [#728](https://github.com/khorum08/SimThing/pull/728) — commit `91ca1f19`, merge `8db9c4f6`
