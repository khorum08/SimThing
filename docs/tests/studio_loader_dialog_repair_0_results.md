# STUDIO-LOADER-DIALOG-REPAIR-0 Results

## Status
**PROBATION** - [PR #1324](https://github.com/khorum08/SimThing/pull/1324); branch `codex/studio-loader-dialog-repair-0`; tested code `2a91eec750c1a466fa2d438735a141e247476340`; NOT MERGED.

## What Changed
- Scenario Library is now a minimal path / `Select File…` / `Load` / `Cancel` modal; fresh path empty and progress hidden.
- Selection only updates the path. Load composes existing resolve, parse, hydrate, rebind, persist, authority reload, Studio projection, and scene-adoption calls.
- Each real stage records running/passed/failed, elapsed time, and the actual failure; later stages remain not-run.
- Failure retains path/modal/previous session. Success uses existing adoption, bridge reset, scene rebuild, cache invalidation, and camera reset, then closes paused.
- Blank creation, JSON/manual actions, and candidate/runtime controls moved to Telemetry -> Scenario.
- `Studio_ops Telemetry` is hidden by default, non-modal, read-only, and reports the last load's eight stages.

## Load-Bearing Proofs
| proof | catches | result |
|---|---|---|
| minimal defaults | legacy modal controls / fake visible progress | PASS |
| select-only picker | picker immediately ingesting or cancel erasing path | PASS |
| ordered timed stages | fabricated/reordered production progress | PASS |
| failure atomicity | path/session loss or silent fallback | PASS |
| success lifecycle | autoplay, missing reset/adoption/rebuild | PASS |
| OVL telemetry | missing/default-visible/authoritative telemetry | PASS |

Named regressions: Scenario Library 12/12, Clause loader 10/10, faction nameplates 10/10 PASS. `cargo check` and debug `simthing-studio.exe` build PASS. Formatter check exposes pre-existing untouched-master differences under the installed rustfmt; touched diff is formatted and whitespace-clean.

## Scope Ledger
Specified/implemented: presentation dialog, picker/controller factoring, production-stage observation, OVL telemetry, focused proofs. Proxied: existing ClauseScript ingest, ScenarioSpec persistence/session projection, app adoption and scene rebuild. Deferred: Owner live-client verdict. Untouched: Spec, ClauseThing, driver, sim, kernel, GPU/WGSL, scenarios, workflows, clearance/class/bindings/anchors.

## OVL
**PENDING** - Owner screenshot and satisfaction statement required; coding agent did not launch the client.

## Known Gaps / Next
- Owner verifies the supplied debug executable and returns PASS or REMAND.
- Expected route: `studio-live-ops-ui-clock` / ORCHESTRATOR-CLEARABLE only after OVL PASS and a current green sticky.
