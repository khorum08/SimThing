# SCENARIO-RUNTIME-SAVELOAD-DA-PRECHECK-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `scenario-runtime-saveload-da-precheck-0`
- PR: #848
- Merge SHA: `46c2c4b21fe06f1a923956a0a9ce8e0ae14f5f28`

## Mission

Consolidate Scenario Runtime + Save/Load Closing Track evidence (rungs 0–7 plus pre-DA hardening) for human DA review. No new runtime features.

## Constitution / ADR alignment

Evaluated against 0.0.8.1+ constitution carried into 0.0.8.3+, SimThing core design, ScenarioSpec authority model, STEAD mapping/stable ID contract, AccumulatorOp v2 doctrine, Resource Flow substrate doctrine, sparse spatial/bounded-theater doctrine, and GPU residency doctrine. All closing-track rungs align: ScenarioSpec is authority; CPU work is oracle/reference/serialization/validation/reporting only; runtime surfaces are GPU-compatible row/table shapes.

## Closing track rung table

| Rung | Track ID | PR | Status |
|---:|---|---|---|
| 0 | SCENARIO-CANONICAL-LOAD-SAVE-ROUNDTRIP-0 | #828 | DONE |
| 1 | SCENARIO-STEAD-MAP-ROUNDTRIP-0 | #834 | DONE |
| 2 | LOADED-SCENARIO-STUDIO-SESSION-ENVELOPE-0 | #836 | DONE |
| 3 | LOADED-SCENARIO-RECURSIVE-RF-RUNTIME-0 | #838 | DONE |
| 4 | LOADED-SCENARIO-RUNTIME-REPORT-CHAIN-0 | #840 | DONE |
| 5 | SCENARIO-CANDIDATE-FROM-RUNTIME-0 | #842 | DONE |
| 6 | SCENARIO-CANDIDATE-SAVE-REOPEN-0 | #844 | DONE |
| — | SCENARIO-CANDIDATE-SAVE-REOPEN-HARDEN-0 | #845 | DONE (pre-UI) |
| 7 | STUDIO-SCENARIO-RUNTIME-SAVELOAD-UI-0 | #846 | DONE |
| — | STUDIO-CANDIDATE-REOPEN-ADOPT-0 | #847 | DONE (pre-DA) |
| 8 | SCENARIO-RUNTIME-SAVELOAD-DA-PRECHECK-0 | #848 | DONE |

## Evidence completeness matrix

| Track ID | PR | Merge / evidence SHA | Lifecycle | Result report | Production doc section | Validation status | Authority boundary status | DA status |
|---|---|---|---|---|---|---|---|---|
| SCENARIO-CANONICAL-LOAD-SAVE-ROUNDTRIP-0 | #828 | `ee651acdf4b4a0e94b707300974d285ea2664754` | PROBATION | `scenario_canonical_load_save_roundtrip_0_results.md` | Rung 0 / baseline | PASS (spec+driver) | ScenarioSpec authority; no savefile/UI/GPU | Not promoted |
| SCENARIO-STEAD-MAP-ROUNDTRIP-0 | #834 | `50c310a0fc26cf5be39145e9438fc9e661cf3fa4` | PROBATION | `scenario_stead_map_roundtrip_0_results.md` | § SCENARIO-STEAD-MAP-ROUNDTRIP-0 | PASS (spec+driver) | STEAD/tree/RF preserved; owner≠spatial parentage | Not promoted |
| LOADED-SCENARIO-STUDIO-SESSION-ENVELOPE-0 | #836 | `52ded5b5f02bcb3489a7904e1b7ee315a608213c` | PROBATION | `loaded_scenario_studio_session_envelope_0_results.md` | § LOADED-SCENARIO-STUDIO-SESSION-ENVELOPE-0 | PASS (spec+driver) | Studio/Bevy/GPU/runtime non-authoritative | Not promoted |
| LOADED-SCENARIO-RECURSIVE-RF-RUNTIME-0 | #838 | `a09a40f86f01c7fe14c9eab7789cd4db7fbf7c15` | PROBATION | `loaded_scenario_recursive_rf_runtime_0_results.md` | § LOADED-SCENARIO-RECURSIVE-RF-RUNTIME-0 | PASS (spec+driver) | Local arena settlement; GPU-compatible rows; CPU oracle | Not promoted |
| LOADED-SCENARIO-RUNTIME-REPORT-CHAIN-0 | #840 | `f9d838400bcd4073f2f1d74fea85c4dae94a51cd` | PROBATION | `loaded_scenario_runtime_report_chain_0_results.md` | § LOADED-SCENARIO-RUNTIME-REPORT-CHAIN-0 | PASS (spec+driver) | Explicit report mode; no hidden mutation | Not promoted |
| SCENARIO-CANDIDATE-FROM-RUNTIME-0 | #842 | `d66fbd856c32cfc88a85953e10455e20824d74b2` | PROBATION | `scenario_candidate_from_runtime_0_results.md` | § SCENARIO-CANDIDATE-FROM-RUNTIME-0 | PASS (spec+driver) | Original authority preserved; candidate clone only | Not promoted |
| SCENARIO-CANDIDATE-SAVE-REOPEN-0 | #844 | `aecb4421975f301af34437987ff87ffdee55f89a` | PROBATION | `scenario_candidate_save_reopen_0_results.md` | § SCENARIO-CANDIDATE-SAVE-REOPEN-0 | PASS (spec+driver) | Canonical JSON only; digest stable on reopen | Not promoted |
| SCENARIO-CANDIDATE-SAVE-REOPEN-HARDEN-0 | #845 | `8aa72e6c5e395420a91df843d6b24a5bd2e39334` | PROBATION | `scenario_candidate_save_reopen_harden_0_results.md` | § SCENARIO-CANDIDATE-SAVE-REOPEN-HARDEN-0 | PASS (spec 23 tests) | Create-new writer; existing target preserved | Not promoted |
| STUDIO-SCENARIO-RUNTIME-SAVELOAD-UI-0 | #846 | `83bc5810276724fc044bd0361d6cf817f8f513c8` | PROBATION | `studio_scenario_runtime_saveload_ui_0_results.md` | § STUDIO-SCENARIO-RUNTIME-SAVELOAD-UI-0 | PASS (mapeditor 13) | UI/Bevy/runtime/GPU non-authoritative | Not promoted |
| STUDIO-CANDIDATE-REOPEN-ADOPT-0 | #847 | `8cefd9c8da067a4a10fe9b4f87cc712c27aaf890` | PROBATION | `studio_candidate_reopen_adopt_0_results.md` | § STUDIO-CANDIDATE-REOPEN-ADOPT-0 | PASS (mapeditor 10) | Successful reopen adopts session; failed preserves | Not promoted |
| SCENARIO-RUNTIME-SAVELOAD-DA-PRECHECK-0 | #848 | `46c2c4b21fe06f1a923956a0a9ce8e0ae14f5f28` | PROBATION | this report | § SCENARIO-RUNTIME-SAVELOAD-DA-PRECHECK-0 | PASS (regression) | Consolidation only; no new authority surfaces | Awaiting human DA |

## ScenarioSpec authority preservation

Rung 0 establishes canonical load/save with stable authority digest. Rungs 1–4 preserve authority through STEAD roundtrip, session envelope, recursive RF, and report chain without mutating loaded ScenarioSpec. Rung 5 clones candidate while preserving original authority. Rungs 6–7 save/reopen candidate canonical JSON without distinct savefile format. #847 adopts reopened candidate into Studio session only after validation passes.

ScenarioSpec remains the only serialized scenario authority. Studio UI state, Bevy ECS state, runtime reports, runtime property-view rows, candidate save/reopen status, and GPU buffers are presentation, proof, cache, or command surfaces only. Candidate ScenarioSpec creation and canonical JSON save/reopen are explicit authority-boundary operations and do not mutate the loaded original ScenarioSpec.

## STEAD contract validation

#834 proves STEAD IDs, links, ownership metadata, RF metadata, and spatial tree survive canonical roundtrip. #836–#847 compose STEAD validation on loaded, candidate, reopened, and adopted scenarios. Owner metadata remains distinct from spatial parentage throughout.

## Spatial tree preservation

Spatial parentage, interior grids, and Location hierarchy shape are preserved through STEAD roundtrip (#834), session envelope (#836), candidate clone (#842), candidate save/reopen (#844), and Studio adoption (#847).

## RF / Accumulator / GPU-residency alignment

#838 attaches recursive Accumulator RF runtime to loaded spatial trees with local parent-arena settlement before upward bubbling by owner/resource/scope. #840 chains full runtime reports through property-view rows. GPU-compatible row/table surfaces are emitted; CPU work is oracle/reference/report formatting only.

The closing track remains GPU-resident in shape: runtime RF, report-chain, candidate-source, and property-view data are emitted as GPU-compatible row/table surfaces. CPU work in this track is limited to deterministic oracle/reference evaluation, ScenarioSpec serialization, file IO, validation, proof construction, and UI/report formatting. No CPU production simulation authority is introduced.

## Loaded scenario session envelope

#836 defines authority envelope with import/export eligibility, projection rebuild readiness, recursive RF prerequisites, and explicit non-authority surfaces for Studio config, Bevy, GPU buffers, and runtime reports.

## Recursive RF runtime surface

#838 proves parent Location arena discovery, sibling settlement, upward bubbling, and owner/resource/scope channel metadata on loaded scenarios.

## Runtime report chain

#840 composes recursive RF → owner-silo → allocation → effects → semantic → execution → delta → runtime state → property view for loaded scenarios in explicit report mode.

## Candidate ScenarioSpec mutation

#842 materializes cloned candidate from runtime property-view rows; original loaded ScenarioSpec authority unchanged; mutation records preserve participant/owner/resource/scope metadata.

## Candidate save/reopen

#844 saves candidate as canonical JSON, reopens, validates STEAD/tree/projection, proves digest stability. #845 hardens create-new writer with same-directory temp and existing-target preservation.

## Candidate write hardening

#845: create-new policy, no target removal before guaranteed write, temp cleanup on failure. Save Candidate in Studio (#846) uses hardened writer exclusively.

## Studio UI save/reopen workflow

#846 exposes digest, validation, RF readiness, report-chain readiness, candidate readiness, Save Candidate, Reopen Candidate. UI state explicitly non-authoritative.

## Reopen candidate adoption

#847 fixes pre-DA defect: successful Reopen Candidate adopts reopened candidate into active Studio session with digest/status refresh and galaxy rebuild; failed reopen preserves session.

## Non-authority UI / Bevy / runtime / GPU surfaces

Explicit across #836, #846, #847: `ui_state_is_authority: false`, `bevy_state_is_authority: false`, `runtime_reports_are_authority: false`, `gpu_buffers_are_authority: false`.

## Deferred boundaries

- replace-existing candidate save flow / overwrite confirmation
- persistent timeline/history model
- GPU dispatch / WGSL implementation
- route/pathfinding
- combat
- full economy execution
- fleet movement/supply
- DA promotion itself

## Regression validation

| Command | Status |
|---------|--------|
| `cargo fmt --all -- --check` | SUBSTITUTE — Windows path-length error (os error 206); scoped `cargo fmt -p simthing-spec -p simthing-driver -p simthing-mapeditor -- --check` PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo check -p simthing-mapeditor` | PASS |
| `cargo test -p simthing-spec --test scenario_canonical_io` | PASS (7) |
| `cargo test -p simthing-spec --test scenario_stead_map_roundtrip` | PASS (10) |
| `cargo test -p simthing-spec --test loaded_scenario_studio_session_envelope` | PASS (11) |
| `cargo test -p simthing-spec --test loaded_scenario_recursive_rf_runtime` | PASS (15) |
| `cargo test -p simthing-spec --test loaded_scenario_runtime_report_chain` | PASS (18) |
| `cargo test -p simthing-spec --test scenario_candidate_from_runtime` | PASS (17) |
| `cargo test -p simthing-spec --test scenario_candidate_save_reopen` | PASS (23) |
| `cargo test -p simthing-driver --test scenario_canonical_io` | PASS (4) |
| `cargo test -p simthing-driver --test scenario_stead_map_roundtrip` | PASS (8) |
| `cargo test -p simthing-driver --test loaded_scenario_studio_session_envelope` | PASS (8) |
| `cargo test -p simthing-driver --test loaded_scenario_recursive_rf_runtime` | PASS (9) |
| `cargo test -p simthing-driver --test loaded_scenario_runtime_report_chain` | PASS (9) |
| `cargo test -p simthing-driver --test scenario_candidate_from_runtime` | PASS (9) |
| `cargo test -p simthing-driver --test scenario_candidate_save_reopen` | PASS (8) |
| `cargo test -p simthing-mapeditor --test studio_scenario_runtime_saveload_ui` | PASS (13) |
| `cargo test -p simthing-mapeditor --test studio_candidate_reopen_adopt` | PASS (10) |
| `git diff --check` | PASS |
| alias guard (`test ! -f docs/0.8.3 Simthing Studio Production.md`) | PASS — alias doc absent |

## Evidence lifecycle and cleanup

During this PR, no live PROBATION evidence rows were deleted. No scratch result reports beyond this DA precheck report were retained. All closing-track result reports remain referenced by `current_evidence_index.md`.

## Remaining risks / known gaps

- Replace-existing candidate save / overwrite confirmation not implemented.
- `scenario_io.rs` general Save Scenario still uses remove-then-rename (distinct from hardened candidate path).
- Persistent history and GPU dispatch remain deferred.
- All closing-track evidence remains PROBATION until human DA promotes.

## DA recommendation

Recommendation: READY FOR HUMAN DA REVIEW.

The Scenario Runtime + Save/Load Closing Track is complete through Studio-visible candidate save/reopen and reopened-candidate adoption. ScenarioSpec remains authority. STEAD IDs, links, RF metadata, and spatial tree shape are preserved through canonical load/save/reopen. Runtime and candidate flows are report/authority-boundary operations, not persistent history or CPU production simulation. Studio UI, Bevy state, runtime reports, and GPU buffers remain non-authoritative. Persistent history, replace-existing overwrite flow, and GPU dispatch remain deferred.

## Boundary / non-goals

No new runtime features, ScenarioSpec mutation paths, savefile formats, GPU dispatch, combat, pathfinding, economy, or fleet movement. This PR consolidates evidence only.

## Files changed

- `docs/tests/scenario_runtime_saveload_da_precheck_0_results.md`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`

## Next recommended action

Human DA review of consolidated Scenario Runtime + Save/Load Closing Track evidence. DA promotion decision is out of scope for this PR.