# STUDIO-INGESTION-ADMISSION-REPORT-DISPLAY-0 — Studio displays canonical ingestion/admission status

> **Lifecycle: PROBATION** — Studio admission report presentation over spec-owned ingestion authority landed. Pending owner DA approval.

**Date:** 2026-06-19  
**PR:** #787 — STUDIO-INGESTION-ADMISSION-REPORT-DISPLAY-0
**Merge:** `5f738f05`
**Base:** `master` after PR #786 / SIM-GPU-OWNER-SILO-RESOURCE-FLOW-TICK-0 (`3ef8fd03`)

## Current defect / UI visibility summary

Studio previously loaded and displayed canonical Scenario trees but did not surface ingestion/admission classification, typed deferrals/errors, legacy compatibility status, or owner-silo GPU readiness from spec authority.

## Studio admission report model

Presentation types in `studio_admission_report.rs`:

- `StudioScenarioAdmissionSummary` — classification, canonical tree status, errors, deferrals, owner silo summary, compile readiness, legacy flag
- `StudioOwnerSiloSummary` — participant totals + GPU participant readiness + full mutation deferral
- `StudioCompileReadinessSummary` — structural/mapping readiness + owner-silo GPU fields

Built from `ScenarioIngestionResult` via `build_studio_admission_summary_from_ingestion`; no duplicated validation traversal.

## Authority boundary summary

| Layer | Role |
|---|---|
| simthing-spec | Ingestion authority (`ingest_scenario`, `ingest_scenario_from_str`) |
| simthing-mapeditor | Presentation copy (`StudioScenarioAdmissionSummary`) |
| simthing-driver | Not called from Studio runtime for admission display |
| simthing-sim / simthing-gpu | Not called from Studio for admission display |

## Canonical valid scenario display proof

**PASS** — `studio_displays_admitted_canonical_scenario_report`, `studio_loaded_session_carries_admission_summary`.

## Invalid scenario display proof

**PASS** — `studio_displays_rejected_missing_gamesession_report`, `studio_displays_rejected_duplicate_owner_report` (report without requiring valid `StudioScenarioDocument`).

## Unsupported scenario display proof

**PASS** — `studio_displays_unsupported_planet_child_deferral`, `studio_displays_unknown_gridcell_role_deferral`.

## Legacy World-root display proof

**PASS** — `studio_legacy_terran_pirate_report_is_legacy_compatibility` (LegacyWorldRootCompatibility deferral + `legacy_world_root` flag).

## Owner-silo GPU readiness display proof

**PASS** — `studio_displays_owner_silo_admission_summary`, `studio_displays_owner_silo_gpu_participant_ready_and_full_mutation_deferred`.

Displayed semantics:

- GPU participant accumulation ready: true (balanced flow corpus)
- Full owner-silo state mutation: deferred
- Scenario authority mutation from GPU proof: no

## No GPU dispatch / no authority mutation proof

**PASS** — `studio_ingestion_report_does_not_dispatch_gpu`, `studio_ingestion_report_does_not_mutate_scenario_authority`, e10 guards extended.

## Production synthesis update summary

Added § STUDIO-INGESTION-ADMISSION-REPORT-DISPLAY-0; reprioritized Next Production Rungs.

## Evidence lifecycle cleanup summary

**PASS** — PR #786 evidence-index row corrected to `#786` / `3ef8fd03`; `sim_gpu_owner_silo_resource_flow_tick_0_results.md` merge metadata updated; live ledger preserved; no DA promotion.

## Specified-vs-implemented ledger

| Requirement | Status |
|---|---|
| Studio displays ingestion classification | PASS |
| Invalid scenario errors without document | PASS |
| Unsupported/partial deferrals visible | PASS |
| Legacy Terran Pirate compatibility visible | PASS |
| Owner silo admission summary | PASS |
| GPU participant accumulation readiness visible | PASS |
| Full state mutation deferred visible | PASS |
| No SimGpuAccumulatorTickState in Studio | PASS |
| No compile_owner_silo_gpu_tick_plan in Studio | PASS |
| No ingestion authority duplication | PASS |
| No scenario authority mutation | PASS |
| #786 metadata cleanup | PASS |

## Validation commands

| Command | Status |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-core` | PASS |
| `cargo test -p simthing-core` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test scenario_ingestion_admission` | PASS |
| `cargo test -p simthing-spec --test session_resource_flow_silos` | PASS |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test owner_silo_gpu_tick` | PASS |
| `cargo check -p simthing-mapeditor` | PASS |
| `cargo test -p simthing-mapeditor --test canonical_scenario_load_save_display` | PASS |
| `cargo test -p simthing-mapeditor --test studio_ingestion_admission_report` | PASS (12/12) |
| `cargo test -p simthing-mapeditor --test terran_pirate_skeleton` | PASS |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-mapeditor/src/studio_admission_report.rs` (new)
- `crates/simthing-mapeditor/src/studio_scenario_document.rs`
- `crates/simthing-mapeditor/src/session.rs`
- `crates/simthing-mapeditor/src/lib.rs`
- `crates/simthing-mapeditor/tests/studio_ingestion_admission_report.rs` (new)
- `crates/simthing-spec/src/spec/scenario_ingestion.rs`
- `crates/simthing-spec/src/spec/session_resource_flow.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/tests/e10_resource_flow_admission.rs`
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/sim_gpu_owner_silo_resource_flow_tick_0_results.md`
- `docs/tests/studio_ingestion_admission_report_display_0_results.md` (new)

## Deleted/archived artifacts

None.

## Deferred next rung recommendation

1. Structural placement edit commands.
2. Planet/child-location admission.
3. Full owner-silo state mutation if still deferred.
4. Broader scenario corpus ingestion UX / batch reports.

## DA status

**PROBATION** — pending owner DA approval. No DA promotion in this PR.