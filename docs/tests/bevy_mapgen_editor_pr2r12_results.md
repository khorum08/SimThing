# BEVY-MAPGEN-EDITOR-PR2R12 - Settings X hitbox excludes title drag

> **Lifecycle: PROBATION** - pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Added PR2R12 PROBATION row |
| `docs/tests/bevy_mapgen_editor_pr2r11_results.md` | PROBATION | Previous camera-facing ribbon + strategic view pass |
| `docs/tests/bevy_mapgen_editor_pr2r12_results.md` | PROBATION | This report |
| Screenshots | Not retained | No new screenshot evidence kept |

## Owner bug report

Owner live run confirmed the PR2R11 visual work:

- Hyperlanes and stars now look good.
- Camera-facing ribbon work is successful.
- Settings top-right `X` still does not hide the dialog.
- Bottom `Close` does hide the dialog.

## Hitbox diagnosis

The dialog model close path already hid the dialog, but the UI title-bar drag interaction was allocated over
the entire title row after the close button was drawn. That full-row `ui.interact(..., Sense::drag())`
covered the `X` button's hitbox, allowing the drag interaction to intercept the pointer path the close
button needed.

## UI fix

- The Settings title row now captures the actual `X` button rect.
- The title-bar drag rect is derived from the title row rect but stops before the close button rect with a
  small gap.
- The `X` and bottom `Close` paths continue to route through the same hide/persist state helper.
- Settings dialog position, star render settings, star render mode, and hyperlane render settings are
  preserved across both close paths.
- This is UI behavior only and does not affect star rendering, hyperlane rendering, view modes,
  generation, topology, simulation, pathfinding, save/load, or structural authority.

## Tests added or updated

- `settings_title_drag_rect_does_not_overlap_close_rect`
- `settings_title_drag_rect_covers_title_area`
- `settings_close_icon_action_hides_dialog`
- `settings_bottom_close_action_hides_dialog`
- `settings_close_paths_preserve_star_values`
- `settings_close_paths_preserve_hyperlane_values`

## Commands run

```text
cargo fmt --all -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test stead_spatial_contract_guards
cargo build -p simthing-mapeditor --bin simthing-studio
target\debug\simthing-studio.exe launch smoke
git diff --check
git diff --name-only master...HEAD
```

## Manual Windows check

This pass was implemented on Windows and includes focused UI-hitbox regression tests plus hidden executable
launch smoke. Interactive owner/DA visual confirmation remains pending.

| Check | Status |
|---|---|
| Gear opens Settings | Existing path preserved; pending DA visual confirmation |
| Top-right X hides Settings immediately | Hitbox/model tests PASS; pending DA visual confirmation |
| Bottom Close hides Settings immediately | Model tests PASS |
| Reopening Settings restores values | Model tests PASS |
| Dragging title bar still moves dialog | Existing movement path preserved; pending DA visual confirmation |
| Clicking X does not drag the window | Drag/close rect separation test PASS |
| Settings window remains bounded by panels | Existing clamp tests PASS |
| Hyperlane/star rendering remains unchanged | No render code changed |
| Studio executable starts | Launch smoke PASS |

## Files changed

- `crates/simthing-mapeditor/src/app/ui.rs`
- `crates/simthing-mapeditor/src/dialog.rs`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/bevy_mapgen_editor_pr2r12_results.md`

## DA status

**PROBATION** - no pre-filed DA approval. Owner sign-off required before promotion to CURRENT_EVIDENCE.
