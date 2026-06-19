# DRIVER-MAPPING-PLAN-COMPILE-0 — driver generic mapping-plan assembly

**Status:** PROBATION  
**Date:** 2026-06-18  
**Base:** `master` after PR #770 / SIM-MAPPING-PLAN-TICK-SEAM-0  
**Branch:** `driver-mapping-plan-compile-0`

## Orientation answers

| Question | Answer |
|---|---|
| Test-local assembly promoted? | `assemble_terran_pirate_mapping_plan` from `terran_pirate_mapping_plan_tick.rs` |
| New driver API? | `compile_mapping_plan_from_admitted_theater`, `compile_structured_field_mapping_plan`, `MappingPlanCompileSpec` |
| Takes theater + specs, not scenario? | **Yes** — admitted `CompiledStructuralN4Theater` + admitted `CompiledRegionFieldPreview` / `CompiledWImpedanceCompose` |
| Scenario authority boundary? | Scenario deserialization + STEAD validation stay in integration test; compile API never reads scenario |
| simthing-sim upward deps? | **Clean** |
| Returns type? | `simthing_sim::CompiledMappingPlan` directly |
| Config compilers used? | `compiled_stencil_to_gpu_config`, `compiled_w_impedance_compose_to_gpu_config`, `composed_w_min_plus_stencil_config` |
| Generic descriptors only? | **Yes** — no GPU operator instantiation |
| Forbidden semantics? | **None** in compile module |
| Terran Pirate uses new API? | **Yes** |
| None readback unchanged? | **Yes** |
| New primitive/shader? | **No** |

## Why this is not mere refactor

PR #770 moved resident tick lifecycle into sim, but Terran Pirate still assembled `CompiledMappingPlan` in a test helper. Without a driver compile surface, future mapping work would copy-paste assembly logic. This PR formalizes driver-owned generic plan assembly while sim retains tick lifecycle.

## New driver compile API

**Module:** `crates/simthing-driver/src/mapping_plan_compile.rs`

```text
MappingPlanCompileSpec { structured_field, hops, column_writes, w_compose, min_plus params }
  -> compile_mapping_plan_from_admitted_theater(theater, spec)
  -> CompiledMappingPlan (sim-owned descriptor)

compile_structured_field_mapping_plan(theater, preview, hops, writes, interleaved_n_dims)
  -> single-step structured field plan
```

## Proofs

| Proof | Status |
|---|---|
| mapping_plan_compile (5 tests) | PASS |
| terran_pirate_mapping_plan_tick (3 tests) | PASS — REAL_ADAPTER_OBSERVED |
| terran_pirate_mapping_first_slice | PASS |
| structural_n4_theater_compile | PASS |
| sim mapping_plan_tick | PASS |
| e10 seam guards | PASS |

## Validation commands

`CARGO_BUILD_JOBS=1` on Windows.

| Command | Status |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test mapping_plan_compile` | PASS (5/5) |
| `cargo test -p simthing-driver --test terran_pirate_mapping_plan_tick` | PASS (3/3) |
| `cargo test -p simthing-driver terran_pirate` | PASS |
| `cargo test -p simthing-sim --test mapping_plan_tick` | PASS |
| `cargo test -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test e10_resource_flow_admission e10_does_not_import_arena_registry_into_simthing_sim` | PASS |
| `git diff --check` | PASS |

## Files changed

- `crates/simthing-driver/src/mapping_plan_compile.rs` (new)
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/mapping_plan_compile.rs` (new)
- `crates/simthing-driver/tests/terran_pirate_mapping_plan_tick.rs`
- `crates/simthing-spec/tests/e10_resource_flow_admission.rs`
- `docs/tests/driver_mapping_plan_compile_0_results.md`
- `docs/tests/current_evidence_index.md`
- `docs/0.8.3 Simthing Studio Production.md`

## Deferred work

- Big-endian/portable byte-proof hardening
- Production None-policy formal DebugReadbackGuard coupling for structured-field internal paths
- Multi-theater atlas scheduling runtime

## DA status

**PROBATION** — no DA/owner sign-off claimed.