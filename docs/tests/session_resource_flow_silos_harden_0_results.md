# SESSION-RESOURCE-FLOW-SILOS-HARDEN-0 — malformed Owner silo metadata rejection

> **Lifecycle: PROBATION** — Strict silo numeric validation, InvalidSiloAmount reachability, ingestion propagation, and driver rejection proof landed. Pending owner DA approval.

**Date:** 2026-06-19  
**PR:** SESSION-RESOURCE-FLOW-SILOS-HARDEN-0  
**Base:** `master` after PR #784 / SESSION-RESOURCE-FLOW-SILOS-0 (`873692bb`)

## Current defect summary

PR #784 introduced `OwnerSiloAdmissionErrorKind::InvalidSiloAmount` but `evaluate_owner_silo_flow` used `owner_silo_current(owner).unwrap_or(0)` and `owner_silo_capacity(owner).unwrap_or(u32::MAX)`, silently accepting malformed or missing silo properties.

## Silo numeric validation behavior

| Case | Behavior |
|---|---|
| `owner_silo_current` property present, malformed | `InvalidSiloAmount` → Rejected |
| `owner_silo_capacity` property present, malformed | `InvalidSiloAmount` → Rejected |
| `owner_silo_marker` present, active flow, `owner_silo_current` absent | `InvalidSiloAmount` (current required) |
| `owner_silo_marker` only, no participants | Placeholder deferral (unchanged) |
| `owner_silo_capacity` absent | Unbounded capacity (`u32::MAX`) |
| `current > capacity` | `InvalidSiloAmount` → Rejected |

Helpers: `read_owner_silo_amount`, `validate_owner_silo_property_values`, `read_owner_silo_state_for_flow`.

## InvalidSiloAmount reachability proof

**PASS** — spec tests: `owner_silo_invalid_current_rejected`, `owner_silo_invalid_capacity_rejected`, `owner_silo_current_greater_than_capacity_rejected_or_typed_deferred`, `owner_silo_marker_without_current_behavior_is_explicitly_tested`, `owner_silo_invalid_silo_amount_corpus_rejected`.

## Ingestion propagation proof

**PASS** — `ingestion_rejects_invalid_owner_silo_current`, `ingestion_rejects_invalid_owner_silo_capacity`; `ScenarioIngestionResult.owner_silo` carries errors; ingestion `errors` list uses `code: "owner_silo"` with `InvalidSiloAmount` message.

## Driver rejection/materialization proof

**PASS** — `owner_silo_driver_rejects_invalid_silo_amount`, `owner_silo_driver_does_not_materialize_rejected_silo_flow`; `build_owner_silo_resource_flow_spec` returns `None` for rejected reports.

## Full simthing-spec package test status

**PASS** — `cargo test -p simthing-spec` completed successfully on re-run (2026-06-19). Prior PR #784 `STATUS_STACK_BUFFER_OVERRUN` during lib test link **did not reproduce** on this run (Windows, `rustc 1.85.x` stable-msvc). Named integration suites remain the primary proof surface regardless.

## Targeted suite validation table

| Command | Status |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-core` | PASS |
| `cargo test -p simthing-core` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test session_resource_flow_silos` | PASS (17/17 + 1 ignored) |
| `cargo test -p simthing-spec --test scenario_ingestion_admission` | PASS |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test session_resource_flow_silos` | PASS (6/6) |
| `cargo test -p simthing-driver --test scenario_ingestion_compile_readiness` | PASS |
| `cargo check -p simthing-mapeditor` | PASS |
| `cargo test -p simthing-mapeditor --test canonical_scenario_load_save_display` | PASS |
| `cargo test -p simthing-mapeditor --test terran_pirate_skeleton` | PASS |
| `git diff --check` | PASS |

## Production synthesis update summary

Added § SESSION-RESOURCE-FLOW-SILOS-HARDEN-0 under SESSION-RESOURCE-FLOW-SILOS-0 in `docs/0.8.3 Simthing Studio Production.md`.

## Evidence lifecycle cleanup summary

**PASS** — live ledger preserved; new PROBATION results doc added; PR #784 prerequisite unchanged; no DA promotion.

## Specified-vs-implemented ledger

| Requirement | Status |
|---|---|
| Malformed current rejected | PASS |
| Malformed capacity rejected | PASS |
| current > capacity rejected | PASS |
| InvalidSiloAmount not dead enum | PASS |
| Ingestion propagates errors | PASS |
| Driver refuses materialization | PASS |
| Existing silo tests green | PASS |
| No engines / no GPU primitive | PASS |
| Full package test honest record | PASS |

## Files changed

- `crates/simthing-spec/src/spec/session_resource_flow.rs`
- `crates/simthing-spec/tests/session_resource_flow_silos.rs`
- `crates/simthing-spec/tests/e10_resource_flow_admission.rs`
- `crates/simthing-driver/tests/session_resource_flow_silos.rs`
- `scenarios/corpus/owner_silo_invalid_silo_amount.simthing-scenario.json`
- `docs/tests/session_resource_flow_silos_harden_0_results.md`
- `docs/tests/current_evidence_index.md`
- `docs/0.8.3 Simthing Studio Production.md`

## Deleted/archived artifacts

None.

## Deferred next rung recommendation

1. Studio ingestion/admission report display for `owner_silo` errors.
2. GPU-resident owner-silo tick over admitted arena registry (no new primitive).
3. Structural placement edit commands.

## DA status

**PROBATION** — not DA-promoted. Prerequisite: PR #784.