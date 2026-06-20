# SIM-MAPPING-READBACK-POLICY-HARDEN-0 — mapping None/ProofReadback discipline

> **Lifecycle: PROBATION** — mapping readback-policy hardening complete; full validation sweep recorded. Pending owner DA approval.

**Date:** 2026-06-18  
**PR:** SIM-MAPPING-READBACK-POLICY-HARDEN-0  
**Base:** `master` after PR #771 / DRIVER-MAPPING-PLAN-COMPILE-0 (`1ade951c`)

## Artifact lifecycle audit

| Artifact | Regime | Action |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated with READBACK-POLICY-HARDEN-0 row |
| `docs/tests/driver_mapping_plan_compile_0_results.md` | PROBATION (PR #771) | Retained — compile-surface evidence |
| `docs/tests/sim_mapping_plan_tick_seam_0_results.md` | PROBATION (PR #770) | Retained — tick-seam evidence |
| `docs/tests/sim_gpu_readback_scope_0_results.md` | PROBATION (PR #762) | Retained — scoped guard precedent |
| `docs/tests/sim_mapping_readback_policy_harden_0_results.md` | PROBATION | Created (this file) |
| `docs/design_0_0_8_3_studio_production.md` | Living synthesis | Updated § READBACK-POLICY-HARDEN-0 |
| `scenarios/horizon/terran_pirate_skeleton.simthing-scenario.json` | Canonical authority | Untouched |

## Why this is not hygiene

PR #770/#771 placed the mapping chain constitutionally (driver compile → sim resident tick → GPU operators). Before multi-theater atlas scheduling, repeated mapping ticks under `None` must not perform hidden proof readback. Unscoped debug readback in structured-field proof paths would leak into production ticks — the same failure class as accumulator readback scope (PR #762).

## Orientation answers

| Question | Answer |
|---|---|
| Mapping tick calls proof readback under `None`? | **No** — None uses `dispatch_ping_pong` / `compose_resident_field` / `GpuResident` only |
| Operators with explicit readback modes? | StructuredField: scoped `readback_after_ping_pong`; MinPlus: `GpuResident` vs `DiagnosticReadback`; W-compose: no readback API |
| StructuredFieldStencilOp debug gate? | Uses existing `scoped_debug_readback_allowed` via `run_with_proof_readback_enabled` (accumulator `DebugReadbackGuard` precedent) |
| WImpedanceComposeOp readback-free under None? | **Yes** — `compose_resident_field` only |
| MinPlus GpuResident under None / DiagnosticReadback under ProofReadback? | **Yes** — explicit mode selection in tick |
| Global debug flag touched by mapping ticks? | Only via scoped RAII guard on ProofReadback structured-field path |
| ProofReadback scoped and restorable? | **Yes** — RAII `DebugReadbackGuard` |
| ProofReadback → None readback-free? | **Proven** — sim + driver tests |
| None → ProofReadback → None readback-free? | **Proven** — sim + driver tests |
| Deferred item closed? | **CLOSED** — no new GPU API required; existing operators + scoped guard sufficient |

## Operator readback path audit

### StructuredFieldStencil

| Path | Behavior |
|---|---|
| None | `upload_values` + `dispatch_ping_pong` (+ optional scatter); `proof_values == None` |
| ProofReadback | `run_with_proof_readback_enabled` → `scoped_debug_readback_allowed(true)` → `readback_after_ping_pong` |
| Hidden readback risk | **Low** — readback only inside explicit ProofReadback branch + scoped guard |

### WImpedanceCompose

| Path | Behavior |
|---|---|
| None | `compose_resident_field` only |
| ProofReadback | Same dispatch; no readback API |
| Hidden readback risk | **None** — operator has no readback pathway |

### MinPlusStencil

| Path | Behavior |
|---|---|
| None | `MinPlusTraversalExecutionMode::GpuResident` |
| ProofReadback | `MinPlusTraversalExecutionMode::DiagnosticReadback` |
| Hidden readback risk | **Low** — mode explicit per policy |

## None-policy proof

- `mapping_tick_none_never_returns_proof_values_for_structured_field` — PASS
- `mapping_tick_none_never_returns_proof_values_for_w_compose_min_plus` — PASS
- `mapping_plan_tick_does_not_silently_enable_debug_readback` — PASS (source guard)
- `mapping_plan_tick_none_branch_readback_policy_source_guard` — PASS (source guard)

## ProofReadback proof

- `mapping_plan_tick_structured_field_proof_matches_cpu_oracle` — PASS — REAL_ADAPTER_OBSERVED
- `mapping_plan_tick_w_compose_min_plus_proof_matches_cpu_oracle` — PASS — REAL_ADAPTER_OBSERVED

## ProofReadback → None proof

- `mapping_tick_proof_then_none_does_not_leak_readback` — PASS — REAL_ADAPTER_OBSERVED
- Driver `terran_pirate_mapping_plan_tick_full_chain` ProofReadback → None — PASS

## None → ProofReadback → None proof

- `mapping_tick_none_then_proof_then_none_does_not_leak_readback` — PASS — REAL_ADAPTER_OBSERVED
- Driver `terran_pirate_mapping_plan_tick_readback_policy_sequencing` (None → Proof → None → None) — PASS — REAL_ADAPTER_OBSERVED

## Error/panic guard restoration proof

- `mapping_tick_error_does_not_leave_readback_enabled_if_guard_exists` — PASS
- `mapping_tick_panic_restores_readback_guard_if_guard_exists` — PASS

## Driver integration readback preservation

| Proof | Status |
|---|---|
| Terran Pirate full chain D-field parity | PASS — REAL_ADAPTER_OBSERVED |
| Readback policy sequencing (None/Proof/None/None) | PASS — REAL_ADAPTER_OBSERVED |
| Scenario authority not mutated | PASS |
| `cargo test -p simthing-driver terran_pirate` | PASS (12 tests) |

## simthing-sim dependency seam proof

- `simthing-sim/Cargo.toml` — no driver/spec/mapeditor deps — PASS (e10 guard)
- Mapping tick source/tests — no upward imports or scenario JSON — PASS (e10 guard)

## Studio non-ownership proof

Studio remains loader/projection consumer; no mapping runtime dispatch added.

## Forbidden-token scan

Mapping tick source/tests — no route/predecessor/pathfinding/border/frontline/cpu_planner tokens — PASS.

## Big-endian / portable byte-proof backlog

Deferred: explicit little-endian helpers, cross-platform byte-order evidence, replacing host-endian bytemuck casts in canonical artifact byte proofs.

## Atlas scheduling deferral

Not implemented in this PR. Mapping readback discipline is prerequisite evidence only.

## Tests added/changed/deleted

**Added (sim `mapping_plan_tick.rs`):**
- `mapping_tick_none_never_returns_proof_values_for_structured_field`
- `mapping_tick_none_never_returns_proof_values_for_w_compose_min_plus`
- `mapping_tick_proof_then_none_does_not_leak_readback`
- `mapping_tick_none_then_proof_then_none_does_not_leak_readback`
- `mapping_tick_resident_reuse_preserves_readback_policy`
- `mapping_tick_error_does_not_leave_readback_enabled_if_guard_exists`
- `mapping_tick_panic_restores_readback_guard_if_guard_exists`
- `mapping_plan_tick_does_not_silently_enable_debug_readback`
- `mapping_plan_tick_none_branch_readback_policy_source_guard`

**Replaced:** older duplicate proof-then-none / resident-reuse tests consolidated into hardened variants.

**Added (driver `terran_pirate_mapping_plan_tick.rs`):**
- `terran_pirate_mapping_plan_tick_readback_policy_sequencing`

**Extended (e10):** mapping readback policy source guards.

**Deleted:** none.

## Validation commands

| Command | Status |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-sim` | PASS |
| `cargo test -p simthing-sim --test mapping_plan_tick` | PASS (13/13) |
| `cargo test -p simthing-sim --test forked_four_slot_input_list_tick` | PASS (6/6) |
| `cargo test -p simthing-sim --test accumulator_plan_tick_convergence` | PASS (28/28) |
| `cargo check -p simthing-driver` | PASS |
| `cargo test -p simthing-driver --test mapping_plan_compile` | PASS (5/5) |
| `cargo test -p simthing-driver --test terran_pirate_mapping_plan_tick` | PASS (4/4) |
| `cargo test -p simthing-driver --test terran_pirate_mapping_first_slice` | PASS (4/4) |
| `cargo test -p simthing-driver --test structural_n4_theater_compile` | PASS (3/3) |
| `cargo test -p simthing-driver terran_pirate` | PASS (12/12) |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec` | PASS (47 lib + integration) |
| `cargo test -p simthing-spec --test e10_resource_flow_admission e10_does_not_import_arena_registry_into_simthing_sim` | PASS |
| `cargo test -p simthing-mapeditor --test terran_pirate_skeleton` | PASS (10/10, 1 ignored) |
| `cargo test -p simthing-mapeditor --test accumulator_convergence_1_guards` | PASS (19/19) |
| `cargo test -p simthing-gpu --test debug_readback_scope` | PASS (5/5) |
| `cargo test -p simthing-clausething --test stead_spatial_contract_guards` | PASS (12/12) |
| `cargo test -p simthing-clausething --test mapgen_lattice_hierarchy` | PASS (10/10) |
| `cargo test -p simthing-clausething --test mapgen_rf_stead_binding` | PASS (7/7) |
| `cargo test -p simthing-clausething --test mapgen_movement_front` | PASS (23/23) |
| `git diff --check` | PASS |

Windows: no paging-file/linker limit failures observed.

## Files changed

- `crates/simthing-sim/tests/mapping_plan_tick.rs`
- `crates/simthing-driver/tests/terran_pirate_mapping_plan_tick.rs`
- `crates/simthing-spec/tests/e10_resource_flow_admission.rs`
- `docs/tests/sim_mapping_readback_policy_harden_0_results.md`
- `docs/tests/current_evidence_index.md`
- `docs/design_0_0_8_3_studio_production.md`

No changes to `mapping_plan_tick.rs` source — existing API discipline sufficient; tests prove invariant.

## Deleted/archived artifacts

None.

## Deferred work

- Atlas scheduling runtime
- Big-endian / portable byte-proof backlog
- DA promotion to CURRENT_EVIDENCE (requires owner sign-off)

## DA status

**PROBATION** — no DA approval claimed.