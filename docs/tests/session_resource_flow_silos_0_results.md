# SESSION-RESOURCE-FLOW-SILOS-0 — generic Owner silo reduce-up/disburse-down first slice

> **Lifecycle: PROBATION** — Owner silo property model, deterministic admission oracle, ingestion integration, and driver ResourceFlow materialization proof landed. GPU-resident owner-silo tick execution deferred. Pending owner DA approval.

**Date:** 2026-06-19  
**PR:** #784 — SESSION-RESOURCE-FLOW-SILOS-0  
**Merge SHA:** `873692bb`  
**Base:** `master` after PR #783 / GENERAL-SCENARIO-INGESTION-ADMISSION-0 (`397a048d`)

## Current defect summary

PR #783 admitted arbitrary Scenario ingestion but owner resource flow remained blanket-deferred via `OwnerResourceFlowNotYetExecuted` whenever `owner_silo_marker` was present, with no reduce-up/disburse-down semantics or typed owner-silo admission report.

## Owner-silo property model

| Property ID | Name | Location |
|---|---|---|
| `8300304` | `owner_silo_marker` | Owner SimThing (placeholder / active marker) |
| `8300305` | `owner_silo_current` | Owner SimThing |
| `8300306` | `owner_silo_capacity` | Owner SimThing (optional) |

Helpers: `apply_owner_silo_metadata`, `owner_has_silo_metadata`, `owner_silo_current`, `owner_silo_capacity`.

## Owner-reference property model

| Property ID | Name | Location |
|---|---|---|
| `8300307` | `owner_flow_owner_ref` | Spatial participant (string `owner_id`) |
| `8300308` | `owner_flow_surplus` | Spatial participant (exact u32 f32 mirror) |
| `8300309` | `owner_flow_deficit` | Spatial participant (exact u32 f32 mirror) |

Helper: `apply_participant_owner_flow_metadata`. Ownership is property/column based — Owners are not spatial parents.

## Reduce-up/disburse-down semantics

Non-mutating oracle `evaluate_owner_silo_flow`:

1. Per owner with silo metadata, start from `owner_silo_current`.
2. Sum non-negative surplus from participants referencing `owner_id`.
3. Add surplus up to capacity (or unbounded when capacity absent).
4. Sum deficits from participants referencing `owner_id`.
5. Resolve deficits from silo after surplus absorption.
6. Report `reducible_surplus_total`, `resolvable_deficit_total`, `unresolved_deficit_total`.

## Mutation vs non-mutating oracle decision

**Non-mutating oracle** (`evaluate_owner_silo_flow`) is the first-slice authority. No `apply_owner_silo_flow_tick` mutation in this PR.

## Ingestion integration status

**PASS** — `ScenarioIngestionResult.owner_silo` carries `OwnerSiloAdmissionReport`. Admitted flows with participants suppress blanket `OwnerResourceFlowNotYetExecuted`. Silo placeholder without participants still receives typed deferral. Rejected silo errors propagate to ingestion errors.

## Driver/resource-flow materialization status

**PASS** — `simthing-driver::session_resource_flow_silos` builds generic `ResourceFlowSpec` with explicit participants, calls `compile_resource_flow_admission`, `compile_and_materialize_resource_flow`, and `materialize_arena_registry`.

## GPU-resident execution status

**PARTIAL/deferred** — Generic admission/materialization over existing ResourceFlow surfaces passes. Owner-silo reduce-up/disburse-down tick execution remains CPU-oracle only; no new WGSL/shader/primitive.

## Corpus fixture list

| Fixture | Purpose |
|---|---|
| `owner_silo_balanced_flow.simthing-scenario.json` | Surplus + deficit balanced under `owner_a` |
| `owner_silo_unresolved_deficit.simthing-scenario.json` | Insufficient silo for deficit |
| `owner_silo_unknown_owner_ref.simthing-scenario.json` | Unknown `owner_id` reference |
| `owner_silo_missing_silo.simthing-scenario.json` | Participant without owner silo metadata |

## Terran Pirate legacy status

**PASS** — unchanged lower-layer golden fixture; not used as canonical owner-silo proof.

## Studio boundary status

**SKIP** — no Studio resource-flow tick, GPU dispatch, or owner-silo authority. Existing read-only `owner_silo_marker` display unchanged.

## Production synthesis update summary

Added § SESSION-RESOURCE-FLOW-SILOS-0; reprioritized Next Production Rungs (Studio ingestion UI, structural edits, GPU execution, planet admission).

## Evidence lifecycle cleanup summary

**PASS** — live ledger preserved; new PROBATION results doc added; no DA promotion; no contradictory evidence left.

## Specified-vs-implemented ledger

| Requirement | Status |
|---|---|
| Owner silo metadata on Owner SimThings | PASS |
| Owner references on spatial participants | PASS |
| Reduce-up/disburse-down oracle + report | PASS |
| Ingestion integration / deferral suppression | PASS |
| Driver RF admission/materialization reuse | PASS |
| Explicit-participant only | PASS |
| Corpus fixtures (4) | PASS |
| Spec tests (11) | PASS |
| Driver tests (4) | PASS |
| e10 constitution guards | PASS |
| No Owner/Faction/Economy/Silo engine | PASS |
| No new GPU primitive/WGSL | PASS |
| GPU-resident owner-silo tick | PARTIAL/deferred |

## Validation commands

| Command | Status |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-core` | PASS |
| `cargo test -p simthing-core` | PASS (0 tests) |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec` | PARTIAL (rustc STATUS_STACK_BUFFER_OVERRUN during full lib test link; targeted suites PASS) |
| `cargo test -p simthing-spec --test session_resource_flow_silos` | PASS (10/10 + 1 ignored) |
| `cargo test -p simthing-spec --test scenario_ingestion_admission` | PASS (12/12 + 1 ignored) |
| `cargo test -p simthing-spec --test scenario_galaxymap_worldstate` | PASS |
| `cargo test -p simthing-spec --test scenario_owner_entities` | PASS |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS (18/18) |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test session_resource_flow_silos` | PASS (4/4) |
| `cargo test -p simthing-driver --test scenario_ingestion_compile_readiness` | PASS (4/4) |
| `cargo check -p simthing-mapeditor` | PASS |
| `cargo test -p simthing-mapeditor --test canonical_scenario_load_save_display` | PASS |
| `cargo test -p simthing-mapeditor --test terran_pirate_skeleton` | PASS |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-spec/src/spec/scenario.rs`
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/spec/session_resource_flow.rs` (new)
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/session_resource_flow_silos.rs` (new)
- `crates/simthing-spec/tests/scenario_ingestion_admission.rs`
- `crates/simthing-spec/tests/e10_resource_flow_admission.rs`
- `crates/simthing-driver/src/session_resource_flow_silos.rs` (new)
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/session_resource_flow_silos.rs` (new)
- `scenarios/corpus/owner_silo_*.simthing-scenario.json` (4 fixtures)
- `docs/tests/session_resource_flow_silos_0_results.md`
- `docs/tests/current_evidence_index.md`
- `docs/0.8.3 Simthing Studio Production.md`

## Deleted/archived artifacts

None — no scratch logs or superseded reports removed.

## Deferred next rung recommendation

1. Studio ingestion/admission report display for `owner_silo` and ingestion classification.
2. Optional `apply_owner_silo_flow_tick` mutation seam once GPU-resident RF execution is wired.
3. GPU-resident owner-silo visualization over admitted arena registry.

## DA status

**PROBATION** — not DA-promoted. Prerequisites PRs #776–#783 recorded in evidence index.