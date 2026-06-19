# SIM-MAPPING-PLAN-TICK-SEAM-0 — sim-owned resident mapping tick seam

**Status:** PROBATION  
**Date:** 2026-06-18  
**Base:** `master` after PR #769 / STRUCTURAL-N4-THEATER-COMPILE-0  
**Branch:** `sim-mapping-plan-tick-seam-0`

## Orientation answers

| Question | Answer |
|---|---|
| Driver test that owned direct GPU execution? | `terran_pirate_mapping_first_slice.rs` (still compile/operator bridge evidence) |
| Minimal sim descriptor? | `CompiledMappingPlan` + `CompiledMappingStep` (StructuredFieldStencil / WImpedanceCompose / MinPlusStencil) |
| Descriptor location? | `crates/simthing-sim/src/mapping_plan_tick.rs` |
| simthing-sim upward deps? | **Clean** — no driver/spec/mapeditor |
| GPU operators wrapped? | `StructuredFieldStencilOp`, `WImpedanceComposeOp`, `MinPlusStencilOp` |
| Resident reuse? | **Yes** — operators/buffers created once in `SimGpuMappingTickState::new`; `tick_count` proves multi-tick reuse |
| Readback policies? | `SimGpuMappingReadbackPolicy::None` / `ProofReadback` |
| None policy? | Uses `dispatch_ping_pong` / `GpuResident` only; no proof readback returned |
| Driver integration? | `terran_pirate_mapping_plan_tick.rs` assembles plan from scenario; sim executes tick |
| CPU oracle? | `cpu_structured_field_horizon`, `cpu_w_impedance_compose_oracle`, `cpu_min_plus_d_from_composed_interleaved` |
| New primitive/shader/route? | **No** |

## Why this is not mere plumbing

PR #769 established driver structural N4 compile/admission, but mapping GPU execution still lived in driver tests via direct operator instantiation. The constitution assigns tick lifecycle to `simthing-sim`. This PR introduces the first sim-owned resident mapping tick seam over generic compiled plans while keeping scenario meaning in driver/spec.

## Mapping execution ownership

| Before | After |
|---|---|
| Driver tests call `StructuredFieldStencilOp` / `WImpedanceComposeOp` / `MinPlusStencilOp` directly | Driver assembles `CompiledMappingPlan`; `SimGpuMappingTickState` owns resident operators |
| No sim mapping tick API | `simthing-sim` owns resident lifecycle + readback policy |

## New sim API

**Module:** `crates/simthing-sim/src/mapping_plan_tick.rs`

- `CompiledMappingPlan` / `CompiledMappingStep` — generic GPU operator descriptors only
- `SimGpuMappingTickState` — resident operator state
- `SimGpuMappingReadbackPolicy` — `None` | `ProofReadback`
- `MappingTickInputs` — structured field value buffers + optional interleaved buffer
- Generic column scatter `(field_col, interleaved_col)` on structured-field steps (numeric plumbing only)

## Proofs

| Proof | Status |
|---|---|
| Sim-local structured field None (no readback) | PASS |
| Sim-local structured field ProofReadback CPU/GPU parity | PASS — REAL_ADAPTER_OBSERVED |
| Sim-local w_compose + min_plus ProofReadback parity | PASS — REAL_ADAPTER_OBSERVED |
| Proof then None (no leak) | PASS |
| Resident reuse across two ticks | PASS |
| Empty plan rejection | PASS |
| Driver Terran Pirate structural N4 + link separation | PASS |
| Driver Terran Pirate structured field via sim tick | PASS — REAL_ADAPTER_OBSERVED |
| Driver Terran Pirate full chain D-field via sim tick | PASS — REAL_ADAPTER_OBSERVED |

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated |
| `docs/tests/structural_n4_theater_compile_0_results.md` | PROBATION (PR #769) | Retained |
| `docs/tests/terran_pirate_mapping_first_slice_0_results.md` | PROBATION (PR #768) | Retained |
| `docs/tests/sim_mapping_plan_tick_seam_0_results.md` | PROBATION (this file) | Created |
| `docs/0.8.3 Simthing Studio Production.md` | Living synthesis | Updated |
| `scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json` | Canonical authority | Untouched |

## Validation commands

`CARGO_BUILD_JOBS=1` on Windows.

| Command | Status |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-sim` | PASS |
| `cargo test -p simthing-sim --test mapping_plan_tick` | PASS (7/7) |
| `cargo test -p simthing-sim --test forked_four_slot_input_list_tick` | PASS |
| `cargo test -p simthing-sim --test accumulator_plan_tick_convergence` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver terran_pirate` | PASS |
| `cargo test -p simthing-driver --test terran_pirate_mapping_plan_tick` | PASS (3/3) |
| `cargo test -p simthing-driver --test terran_pirate_mapping_first_slice` | PASS |
| `cargo test -p simthing-driver --test structural_n4_theater_compile` | PASS |
| `cargo test -p simthing-driver --test terran_pirate_skeleton_compile` | PASS |
| `cargo test -p simthing-driver --test terran_pirate_skeleton_resident_tick` | PASS |
| `cargo test -p simthing-driver --test ct_bh3_closeout_sample_driver bh3_field_operator` | PASS |
| `cargo test -p simthing-driver --test palma_path_5_session_property` | PASS |
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

## Files changed

- `crates/simthing-sim/src/mapping_plan_tick.rs` (new)
- `crates/simthing-sim/src/lib.rs`
- `crates/simthing-sim/tests/mapping_plan_tick.rs` (new)
- `crates/simthing-driver/tests/terran_pirate_mapping_plan_tick.rs` (new)
- `crates/simthing-spec/tests/e10_resource_flow_admission.rs`
- `crates/simthing-spec/tests/region_field_spec_admission.rs` (guard updated for mapping tick seam)
- `docs/tests/sim_mapping_plan_tick_seam_0_results.md`
- `docs/tests/current_evidence_index.md`
- `docs/0.8.3 Simthing Studio Production.md`

## Deferred work

- Multi-theater atlas scheduling runtime
- Studio mapping output visualization
- Big-endian/portable byte-proof hardening
- Production None-policy formal coupling to accumulator `DebugReadbackGuard` for structured-field internal paths (structured-field uses direct dispatch API path; min-plus uses `GpuResident` mode)

## DA status

**PROBATION** — no DA/owner sign-off claimed.