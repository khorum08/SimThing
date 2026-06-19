# SIM-MAPPING-ATLAS-SCHEDULER-0 — sim-owned multi-theater mapping scheduler

> **Lifecycle: PROBATION** — sim-owned atlas scheduler landed; sim-local and driver integration proofs pass; full validation sweep recorded. Pending owner DA approval.

**Date:** 2026-06-18  
**PR:** SIM-MAPPING-ATLAS-SCHEDULER-0  
**Base:** `master` after PR #772 / SIM-MAPPING-READBACK-POLICY-HARDEN-0

## Artifact lifecycle audit

| Artifact | Regime | Action |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated with ATLAS-SCHEDULER-0 row |
| `docs/tests/sim_mapping_readback_policy_harden_0_results.md` | PROBATION (PR #772) | Retained — readback prerequisite |
| `docs/tests/driver_mapping_plan_compile_0_results.md` | PROBATION (PR #771) | Retained — compile surface |
| `docs/tests/sim_mapping_plan_tick_seam_0_results.md` | PROBATION (PR #770) | Retained — single-plan tick seam |
| `docs/tests/sim_mapping_atlas_scheduler_0_results.md` | PROBATION | Created (this file) |
| `docs/0.8.3 Simthing Studio Production.md` | Living synthesis | Updated § ATLAS-SCHEDULER-0 |
| `scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json` | Canonical authority | Untouched |

## Why this is the next rung

PR #772 hardened None/ProofReadback discipline for single-plan resident ticks. PR #769 introduced typed atlas deferral for oversize structural theaters. The remaining horizon item is scheduling multiple already-compiled mapping plans without importing scenario meaning into sim or letting Studio own runtime dispatch.

## Orientation answers

| Question | Answer |
|---|---|
| What is scheduled? | **Already-compiled generic `CompiledMappingPlan` batches** — not scenario authority or admitted theaters |
| Driver compile vs sim scheduling boundary? | Driver: scenario → theater admission → plan compile. Sim: resident scheduler over generic plans |
| simthing-sim upward deps? | **Clean** — no driver/spec/mapeditor |
| Scheduler owns resident tick state per plan? | **Yes** — one `SimGpuMappingTickState` per slot |
| Resident reuse across ticks? | **Yes** — states built once in `new`, pointer-stable across ticks |
| Per-theater output identification? | Stable `MappingTheaterSlot` index (0..n-1); outputs in matching vec order |
| None readback-free all theaters? | **Proven** |
| ProofReadback per-theater without leak? | **Proven** |
| Forbidden semantics introduced? | **No** |
| Scenario authority mutated? | **No** |
| Driver integration uses scenario on driver side only? | **Yes** |

## Scheduler ownership before/after

| Before | After |
|---|---|
| One `SimGpuMappingTickState` per compiled plan, caller-driven | `SimGpuMappingAtlasScheduler` batches multiple resident states |
| No multi-theater sim seam | Stable-slot multi-theater tick execution |

## New sim scheduler API

**Module:** `crates/simthing-sim/src/mapping_atlas_scheduler.rs`

- `MappingTheaterSlot(u32)` — numeric slot index only
- `CompiledMappingAtlas { plans: Vec<CompiledMappingPlan> }`
- `MappingAtlasTickInputs { theater_inputs: Vec<MappingTickInputs> }`
- `MappingAtlasTickOutput { theater_outputs: Vec<SimGpuMappingTickOutput> }`
- `SimGpuMappingAtlasScheduler` — `new`, `tick`, `resident_tick_count`, `theater_resident_tick_count`

Readback policy delegates to `SimGpuMappingTickState::tick` — no direct proof readback helpers in scheduler source.

## Resident reuse proof

`mapping_atlas_scheduler_reuses_resident_states_across_two_ticks` — scheduler pointer stable; per-theater `resident_tick_count` advances to 2.

## None-policy proof (all theaters)

`mapping_atlas_scheduler_none_returns_no_proof_values_for_all_theaters` — PASS

## ProofReadback proof (all theaters)

`mapping_atlas_scheduler_proof_returns_per_theater_outputs` — PASS — REAL_ADAPTER_OBSERVED

## ProofReadback → None proof

`mapping_atlas_scheduler_proof_then_none_does_not_leak_readback` — PASS — REAL_ADAPTER_OBSERVED

## None → ProofReadback → None proof

`mapping_atlas_scheduler_none_then_proof_then_none_does_not_leak_readback` — PASS — REAL_ADAPTER_OBSERVED

## Terran Pirate driver integration proof

`terran_pirate_mapping_atlas_scheduler_two_theater_driver_compile_to_sim` — PASS — REAL_ADAPTER_OBSERVED

- Slot 0: Terran Pirate full chain from driver compile; D-field parity < 1e-4
- Slot 1: synthetic generic structured-field plan (no extra scenario artifact)
- Scenario authority not mutated after scheduler ticks

## Atlas partition/admission deferral

This PR schedules already-compiled plans only. Driver `AtlasDeferred` for oversize structural theaters is unchanged. Multi-theater partition/admission is **not** claimed solved.

## Forbidden-token scan

Scheduler source/tests — no route/predecessor/pathfinding/border/frontline/cpu_planner/scenario types — PASS (e10 + local guard).

## Big-endian / portable byte-proof backlog

Deferred.

## Tests added

**simthing-sim (7):** `mapping_atlas_scheduler.rs`  
**simthing-driver (1):** `terran_pirate_mapping_atlas_scheduler.rs`  
**simthing-spec (e10):** atlas scheduler seam guards

## Validation commands

| Command | Status |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-sim` | PASS |
| `cargo test -p simthing-sim --test mapping_atlas_scheduler` | PASS (7/7) |
| `cargo test -p simthing-sim --test mapping_plan_tick` | PASS |
| `cargo test -p simthing-sim --test forked_four_slot_input_list_tick` | PASS |
| `cargo test -p simthing-sim --test accumulator_plan_tick_convergence` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test terran_pirate_mapping_atlas_scheduler` | PASS (1/1) |
| `cargo test -p simthing-driver --test mapping_plan_compile` | PASS |
| `cargo test -p simthing-driver --test terran_pirate_mapping_plan_tick` | PASS |
| `cargo test -p simthing-driver --test terran_pirate_mapping_first_slice` | PASS |
| `cargo test -p simthing-driver --test structural_n4_theater_compile` | PASS |
| `cargo test -p simthing-driver terran_pirate` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test e10_resource_flow_admission e10_does_not_import_arena_registry_into_simthing_sim` | PASS |
| `cargo test -p simthing-mapeditor --test terran_pirate_skeleton` | PASS |
| `cargo test -p simthing-mapeditor --test accumulator_convergence_1_guards` | PASS |
| `cargo test -p simthing-gpu --test debug_readback_scope` | PASS |
| `cargo test -p simthing-clausething --test stead_spatial_contract_guards` | PASS |
| `cargo test -p simthing-clausething --test mapgen_lattice_hierarchy` | PASS |
| `cargo test -p simthing-clausething --test mapgen_rf_stead_binding` | PASS |
| `cargo test -p simthing-clausething --test mapgen_movement_front` | PASS |
| `git diff --check` | PASS |

Windows: no paging-file/linker limit failures observed.

## Files changed

- `crates/simthing-sim/src/mapping_atlas_scheduler.rs` (new)
- `crates/simthing-sim/src/lib.rs`
- `crates/simthing-sim/tests/mapping_atlas_scheduler.rs` (new)
- `crates/simthing-driver/tests/terran_pirate_mapping_atlas_scheduler.rs` (new)
- `crates/simthing-spec/tests/e10_resource_flow_admission.rs`
- `docs/tests/sim_mapping_atlas_scheduler_0_results.md`
- `docs/tests/current_evidence_index.md`
- `docs/0.8.3 Simthing Studio Production.md`

## Deferred work

- Atlas partition/admission for oversize scenarios (driver-side)
- Big-endian / portable byte-proof backlog
- DA promotion to CURRENT_EVIDENCE (requires owner sign-off)

## DA status

**PROBATION** — no DA approval claimed.