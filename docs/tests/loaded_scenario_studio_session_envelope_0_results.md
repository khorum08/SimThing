# LOADED-SCENARIO-STUDIO-SESSION-ENVELOPE-0 Results

## Status

PASS

## PR / branch / merge

- Branch: `loaded-scenario-studio-session-envelope-0`
- PR: #836
- Merge SHA: `52ded5b5f02bcb3489a7904e1b7ee315a608213c`

## Mission

Implement Closing Track Rung 2: define loaded ScenarioSpec authority envelope for Studio, composing canonical IO (#828) and STEAD map roundtrip (#834).

## Canonical IO baseline

`LoadedScenarioAuthorityEnvelope.canonical_io_ready` composes `prove_scenario_canonical_load_save_roundtrip` via #834 STEAD evaluation. No canonical IO reimplementation.

## STEAD map roundtrip baseline

Authority envelope fields `stead_ids_stable`, `links_stable`, `spatial_tree_stable`, `rf_metadata_stable`, and `owner_metadata_not_spatial_parentage` compose `evaluate_scenario_stead_map_roundtrip_from_json_str` (#834).

## Loaded Scenario authority envelope

`LoadedScenarioAuthorityEnvelope` reports ScenarioSpec digest, scenario id, canonical IO readiness, STEAD roundtrip readiness, projection rebuild readiness, recursive RF prerequisites, and import/export eligibility.

## Runtime sidecar envelope

`LoadedScenarioRuntimeSidecarEnvelope` reports runtime report sidecar availability and recursive RF prerequisite readiness without executing runtime ticks.

## Non-authority surfaces

`LoadedScenarioStudioSessionEnvelope` sets `studio_config_is_authority`, `bevy_state_is_authority`, `gpu_buffers_are_authority`, and `runtime_reports_are_authority` to **false**.

## Import/export readiness

`scenario_import_ready` and `scenario_export_ready` report true on `owner_silo_disburse_down_scoped` corpus fixture when canonical IO and STEAD roundtrip pass.

## Projection rebuild readiness

`studio_projection_rebuild_ready` composes STEAD validation and canonical ingestion admission from #834.

## Recursive RF prerequisite readiness

`recursive_rf_prerequisites_ready` composes `local_rf_parent_node_resolution_prerequisites_present` from #834.

## Boundary / non-goals

Runtime tick execution, runtime mutation, semantic execution, savefile persistence, persistent history, Studio UI wiring, and GPU dispatch remain deferred. `docs/0.8.3 Simthing Studio Production.md` remains deleted.

## Validation

| Command | Status |
|---------|--------|
| `cargo fmt -p simthing-spec -p simthing-driver -- --check` | PASS |
| `cargo test -p simthing-spec --test loaded_scenario_studio_session_envelope` | PASS (11) |
| `cargo test -p simthing-driver --test loaded_scenario_studio_session_envelope` | PASS (8) |
| `cargo test -p simthing-spec --test scenario_stead_map_roundtrip` | PASS |
| `cargo test -p simthing-spec --test scenario_canonical_io` | PASS |
| `cargo test -p simthing-driver --test scenario_stead_map_roundtrip` | PASS |
| `cargo test -p simthing-driver --test scenario_canonical_io` | PASS |
| `git diff --check` | PASS |
| Alias deletion guard | PASS |

## Files changed

- `crates/simthing-spec/src/spec/loaded_scenario_studio_session_envelope.rs` (new)
- `crates/simthing-spec/tests/loaded_scenario_studio_session_envelope.rs` (new)
- `crates/simthing-driver/src/loaded_scenario_studio_session_envelope_compile.rs` (new)
- `crates/simthing-driver/tests/loaded_scenario_studio_session_envelope.rs` (new)
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/loaded_scenario_studio_session_envelope_0_results.md` (this report)

## Evidence lifecycle

**LOADED-SCENARIO-STUDIO-SESSION-ENVELOPE-0** — PROBATION. Not DA-promoted.

## Known gaps

- Closing track Rungs 3–7 not yet implemented.
- Recursive RF runtime attachment (Rung 3) is next.

## Next recommended action

Implement **LOADED-SCENARIO-RECURSIVE-RF-RUNTIME-0** (Closing Track Rung 3).

This rung is not another hygiene-only wrapper. It moves the Scenario Runtime + Save/Load Closing Track forward by defining the loaded Scenario Studio session envelope that future rungs will consume. The envelope composes canonical IO and STEAD map roundtrip readiness into a Studio-facing authority/session boundary while preserving ScenarioSpec as the only authority and keeping Studio config, Bevy state, GPU buffers, and runtime reports non-authoritative.