# BEVY-MAPGEN-EDITOR-PR2R11 - camera-facing hyperlane ribbons and strategic view

> **Lifecycle: PROBATION** - pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Added PR2R11 PROBATION row |
| `docs/tests/bevy_mapgen_editor_pr2r10_results.md` | PROBATION | Previous Settings close + hyperlane controls pass |
| `docs/tests/bevy_mapgen_editor_pr2r11_results.md` | PROBATION | This report |
| Screenshots | Not retained | No new screenshot evidence kept |

## Owner feedback addressed

Owner requested camera-facing hyperlane ribbons so near/foreground lanes do not collapse edge-on, plus a
`Tab` overhead strategic view for map legibility.

## Implementation summary

- Default 3D hyperlanes now render as camera-facing triangle-strip ribbons instead of XZ-flat world strips.
- Ribbon width vectors are computed from lane direction and the active camera basis, with a stable fallback
  for degenerate lane/view alignments.
- The ribbon mesh still uses the PR2R10 soft-edge profile: 80% opaque core with 10% vertex-alpha falloff on
  each side.
- `Tab` toggles `StudioViewMode` between normal 3D and overhead strategic presentation. Strategic mode
  snaps to a top-down legibility camera path and keeps hyperlane settings live.
- The left camera panel shows the current view mode and exposes the same toggle.

## Render-authority confirmations

- Hyperlane ribbon width vectors are render/editor metadata only.
- Overhead strategic view is presentation-only.
- The `Tab` view toggle does not alter structural map authority.
- Hyperlane endpoints remain tied to shared render anchors.
- No MapGenerator, topology, runtime simulation, GPU, spec, ClauseThing, pathfinding, save/load, or
  structural authority changed.
- Structural gridcell coordinates and generated topology remain authoritative.

## Tests added or updated

- `camera_facing_ribbon_width_is_nonzero_for_edge_on_lane`
- `camera_facing_ribbon_uses_render_anchor_endpoints`
- `camera_facing_ribbon_degenerate_case_uses_stable_fallback`
- `hyperlane_ribbon_preserves_anchor_height`
- `hyperlane_ribbon_thickness_applies_settings`
- `hyperlane_opacity_zero_hides_all_ribbons`
- `soft_edge_core_fraction_is_80_percent`
- `soft_edge_side_falloff_is_10_percent_each_side`
- `tab_toggles_view_mode_between_three_d_and_overhead`
- `overhead_mode_uses_legibility_render_path`
- `overhead_mode_does_not_mutate_render_anchors`
- `switching_modes_does_not_regenerate_galaxy`
- `settings_hyperlane_sliders_affect_three_d_mode`
- `settings_hyperlane_sliders_affect_overhead_mode`
- `settings_x_close_still_hides_dialog`

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

This pass was implemented on Windows and includes automated model tests plus hidden executable launch smoke.
Interactive DA visual confirmation remains pending.

| Check | Status |
|---|---|
| Default 3D lanes keep width during camera rotation | Ribbon math tests PASS; pending DA visual confirmation |
| Lane endpoints stay attached to stars/shared anchors | Anchor/ribbon tests PASS |
| Degenerate lane/view alignment has stable fallback width | Helper test PASS |
| Ribbon thickness follows live hyperlane settings | Helper/render tests PASS |
| Opacity 0 hides all ribbon rendering visually | Helper test PASS |
| Soft-edge profile remains 80/10/10 | Helper tests PASS |
| `Tab` toggles 3D/strategic view state | Camera state test PASS |
| Overhead mode uses legibility render path | Camera state test PASS |
| View switching does not regenerate galaxy | View-model test PASS |
| View switching does not mutate render anchors | View-model test PASS |
| Settings hyperlane sliders affect both 3D and overhead modes | Helper tests PASS |
| Settings top-right X still hides dialog | Dialog model test PASS |
| Studio executable starts | Launch smoke PASS |

## Files changed

- `crates/simthing-mapeditor/src/app/camera.rs`
- `crates/simthing-mapeditor/src/app/galaxy_render.rs`
- `crates/simthing-mapeditor/src/app/ui.rs`
- `crates/simthing-mapeditor/src/dialog.rs`
- `crates/simthing-mapeditor/src/hyperlane_buckets.rs`
- `crates/simthing-mapeditor/src/view_model.rs`
- `docs/clausething/MapGeneratorCLI.md`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/bevy_mapgen_editor_pr2r11_results.md`

## Deferred features

- DA visual approval and promotion from PROBATION to CURRENT_EVIDENCE
- Save/load/new session flows
- Live SimThing simulation in Studio
- Clausewitz UI import / WebView/CSS substrate
- Custom shader/screen-space material if later polish needs stronger lane compositing

## DA status

**PROBATION** - no pre-filed DA approval. Owner sign-off required before promotion to CURRENT_EVIDENCE.
