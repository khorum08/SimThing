# STUDIO-LOADER-DIALOG-REPAIR-0 Results

## Status
**PROBATION** - [PR #1324](https://github.com/khorum08/SimThing/pull/1324); branch `codex/studio-loader-dialog-repair-0`; tested code `5024d5203206abaf8f795f178babb21f84cf4c8b`; NOT MERGED.

## What Changed
- Scenario Library is now a minimal path / `Select File…` / `Load` / `Cancel` modal; fresh path empty and progress hidden.
- Selection only updates the path. Load starts a presentation-owned worker and returns to egui immediately; the main thread polls real resolve, parse, hydrate, rebind, persist, authority reload, and Studio projection events each frame.
- Pure scene/status preparation runs on the worker. Bevy entity mutation remains on the main thread and creates stars in bounded batches plus at most one hyperlane bucket per frame.
- Monotonic attempt tokens reject late cancelled/superseded events. Selection, path editing, and duplicate Load are disabled while an attempt is active.
- Failure retains path/modal/previous session. Success atomically swaps the completed scene, adopts through the existing session/reset/cache/camera boundaries, closes only after the final batch, and remains paused.
- Blank creation, JSON/manual actions, and candidate/runtime controls moved to Telemetry -> Scenario.
- `Studio_ops Telemetry` is hidden by default, non-modal, read-only, and reports the last load's eight stages.

## Load-Bearing Proofs
| proof | catches | result |
|---|---|---|
| minimal defaults | legacy modal controls / fake visible progress | PASS |
| select-only picker | picker immediately ingesting or cancel erasing path | PASS |
| ordered timed stages | fabricated/reordered production progress | PASS |
| failure atomicity | path/session loss or silent fallback | PASS |
| async-return dispatch | ingest or rebuild returning to the egui call stack | PASS |
| incremental polling | worker callbacks mutating presentation outside frame polling | PASS |
| stale attempt rejection | cancelled/superseded result adopting late | PASS |
| forced-small batching | scene adoption collapsing into one unbounded pass | PASS |
| success lifecycle | autoplay, missing reset/adoption/rebuild | PASS |
| OVL telemetry | missing/default-visible/authoritative telemetry | PASS |

Focused remediation 10/10 PASS. Named regressions: Scenario Library 12/12, Clause loader 10/10, faction nameplates 10/10 PASS. `cargo check`, `agent_scan.sh` (justified TEST-BUDGET INSPECT; no hard failures), and debug `target/debug/simthing-studio.exe` build PASS. Touched diff is formatted and whitespace-clean.

## Scope Ledger
Specified/implemented: presentation dialog, picker/controller factoring, production-stage observation, worker/poller lifecycle, attempt-token cancellation, pure scene preparation, bounded Bevy scene batches, OVL telemetry, focused proofs. Proxied: existing ClauseScript ingest, ScenarioSpec persistence/session projection, app adoption and render-cache boundaries. Deferred: renewed Owner live-client verdict. Untouched: Spec, ClauseThing, driver, sim, kernel, GPU/WGSL, scenarios, workflows, clearance/class/bindings/anchors.

## OVL
**REMAND ADDRESSED / RE-VERIFY PENDING** - The prior post-completion screenshot proved timings but also proved that synchronous loading blocked egui. Code SHA `5024d5203206abaf8f795f178babb21f84cf4c8b` moves pure work off-thread and batches Bevy adoption. Owner must capture a fresh screenshot while loading is actively underway showing both the modal progress bar and `Studio_ops Telemetry` with the same stage running.

## Known Gaps / Next
- Owner verifies `target/debug/simthing-studio.exe` and returns PASS or a new REMAND.
- Expected route: `studio-live-ops-ui-clock` / ORCHESTRATOR-CLEARABLE only after OVL PASS and a current green sticky.
