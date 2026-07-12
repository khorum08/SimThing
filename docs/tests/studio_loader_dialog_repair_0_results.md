# STUDIO-LOADER-DIALOG-REPAIR-0 Results

## Status
**PROBATION** - [PR #1324](https://github.com/khorum08/SimThing/pull/1324); branch `codex/studio-loader-dialog-repair-0`; tested code `6220d41ab7d761388f2f07165d819a34c037c4a6`; NOT MERGED.

## What Changed
- Scenario Library is a minimal path / `Select File…` / `Load` / `Cancel` modal; fresh path empty and progress hidden.
- Selection only updates the path. Load starts a presentation-owned worker; main thread polls real resolve → parse → hydrate → rebind → persist → session build → projection → scene adopt events.
- Pure scene preparation runs on the worker. Bevy entity mutation is main-thread only and creates stars in bounded batches plus at most one hyperlane bucket per frame.
- **OVL Remand 2 (atomic visibility):** pending scene spawns under a `PendingGalaxySceneRoot` with `Visibility::Hidden`; children use `Visibility::Inherited` + `PendingSceneMember` (debug visibility cannot force them Visible). Opaque egui `Order::Middle` loading cover hides the world for the whole attempt. Final batch commits session while parent stays Hidden; a dedicated reveal frame sets the parent `Visible` once, then drops the cover and closes the modal. Cancel/stale paths despawn the pending parent and never reveal.
- Monotonic attempt tokens reject late cancelled/superseded events. Modal visible ⇒ paused; no autoplay.
- `Studio_ops Telemetry` remains non-modal, read-only, eight-stage.

## Load-Bearing Proofs
| proof | catches | result |
|---|---|---|
| prior 10 focused proofs | dialog/worker/batch/token/OVL base | PASS |
| pending_scene_root_is_hidden_during_every_build_batch | pending render before commit | PASS |
| pending_scene_entities_inherit_hidden_root_visibility | spawn outside root / force Visible | PASS |
| loading_cover_is_active_for_entire_attempt | old scene exposure mid-load | PASS |
| scene_reveal_occurs_only_after_final_batch | early root visibility | PASS |
| commit_reveals_scene_as_one_parent_visibility_transition | per-entity / partial reveal | PASS |
| cancelled_or_stale_attempt_never_reveals_pending_scene | late visibility after cancel | PASS |
| failure_cleans_hidden_pending_scene_and_restores_prior_presentation | leaked pending / blank client | PASS |
| loading_cover_does_not_obscure_modal_or_studio_ops_telemetry | cover hiding OVL controls | PASS |

Focused 18/18 PASS. Named regressions: Scenario Library 12/12, Clause loader 10/10, faction nameplates 10/10 PASS.

## Scope Ledger
Specified/implemented: hidden pending root, Inherited children, loading cover, two-phase commit/reveal, cancel cleanup, proofs. Proxied: existing ingest/worker/batch seams. Deferred: Owner live-client OVL re-verify. Untouched: Spec, ClauseThing, driver, sim, kernel, GPU/WGSL, scenarios output, workflows, clearance/class/bindings/anchors.

## OVL
**REMAND ADDRESSED / RE-VERIFY PENDING** — Owner must re-run `target\debug\simthing-studio.exe` and confirm no starmap (old, partial, or progressive) is visible while Scene adopt is running; complete map appears only after load completion.

## Known Gaps / Next
- Owner live-client verdict only.
- Expected route: `studio-live-ops-ui-clock` / ORCHESTRATOR-CLEARABLE only after OVL PASS and a current green sticky.
