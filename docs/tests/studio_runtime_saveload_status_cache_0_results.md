# STUDIO-RUNTIME-SAVELOAD-STATUS-CACHE-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `studio-runtime-saveload-status-cache-0`
- PR: #856
- Merge SHA: `4d653a927071dd06dbc81c72007f9002cf6bcda1`

## Mission

Cache Studio runtime/candidate save-load status presentation and stop per-frame recomputation of the DA-approved Scenario Runtime + Save/Load proof/report chain from the egui draw path.

## Performance root cause

The performance regression was caused by running the expensive Scenario Runtime + Save/Load readiness proof chain from the egui UI draw path. The UI now displays cached status and refreshes the proof-derived status only on load/save/reopen/manual-refresh or explicit dirty-state transitions.

## Previous per-frame path

`draw_runtime_candidate_saveload_controls` called `refresh_runtime_saveload_status_from_session` every frame when a loaded session existed. That invoked canonical JSON serialization plus `load_scenario_spec_from_json_str`, loaded session envelope evaluation, runtime report-chain compilation, and candidate save/reopen compilation.

## New cached refresh model

`StudioAppState` stores cached `runtime_saveload_status`, dirty/source-digest flags, refresh-in-progress state, and last refresh duration. `refresh_runtime_saveload_status_if_needed` and `runtime_saveload_refresh_decision` gate the expensive proof chain. Studio refreshes on scenario load/adoption, successful Save Candidate, successful Reopen Candidate adoption, session adoption dirty marking, explicit **Refresh Runtime Status**, or dirty/force transitions. Draw displays cached digest, STEAD readiness, recursive RF readiness, report-chain readiness, candidate readiness, last refresh ms, and stale/pending hints.

## ScenarioSpec authority preservation

ScenarioSpec remains authority. Cached UI status is presentation-only and non-authoritative.

## Non-authority UI / Bevy / runtime / GPU boundary

UI state, Bevy ECS, runtime reports, and GPU buffers remain non-authoritative. Persistent history and GPU dispatch remain deferred.

## Candidate save/reopen preservation

Save Candidate still uses hardened create-new writer and preserves loaded session on failure. Reopen Candidate still validates canonical ScenarioSpec JSON. Successful Reopen Candidate adoption still replaces the active Studio session; failed reopen preserves the prior session. Adoption applies refreshed cached status once.

## Corrected surface gridcell preservation

Planet 1×1 surface gridcell tier proofs remain unchanged. Cache tests assert `surface_gridcell_tier_present` on the owner-silo corpus fixture.

## Validation

```text
cargo fmt -p simthing-mapeditor -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor --test studio_scenario_runtime_saveload_ui
cargo test -p simthing-mapeditor --test studio_candidate_reopen_adopt
cargo test -p simthing-mapeditor --test studio_runtime_saveload_status_cache
git diff --check
```

Source guard: `refresh_runtime_saveload_status_from_session` is absent from unconditional `ui.rs` draw code.

## Evidence lifecycle and cleanup

`STUDIO-RUNTIME-SAVELOAD-STATUS-CACHE-0` is recorded as **PROBATION** in `docs/tests/current_evidence_index.md`. No DA-approved rows were demoted. No closing-track rows were reopened.

## Boundary / non-goals

No GPU dispatch, persistent history, replace-existing candidate overwrite, pathfinding, combat, economy execution, fleet movement, ScenarioSpec schema changes, or DA lifecycle promotion.

## Files changed

- `crates/simthing-mapeditor/src/scenario_runtime_saveload_ui.rs`
- `crates/simthing-mapeditor/src/app/mod.rs`
- `crates/simthing-mapeditor/src/app/ui.rs`
- `crates/simthing-mapeditor/src/lib.rs`
- `crates/simthing-mapeditor/tests/studio_runtime_saveload_status_cache.rs`
- `docs/tests/studio_runtime_saveload_status_cache_0_results.md`
- `docs/tests/current_evidence_index.md`
- `docs/design_0_0_8_3_studio_production.md`

## Known gaps

- Authority digest change detection without an explicit dirty transition relies on event-driven dirty marking; there is no per-frame canonical digest polling.
- Frame-loop integration timing is proven via cache helper tests rather than a live egui harness.

## Next recommended action

Run Studio interactively to confirm FPS recovery on loaded scenarios, then promote `STUDIO-RUNTIME-SAVELOAD-STATUS-CACHE-0` from PROBATION after human review if performance and status accuracy hold.