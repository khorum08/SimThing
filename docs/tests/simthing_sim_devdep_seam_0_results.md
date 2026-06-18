# SIMTHING-SIM-DEVDEP-SEAM-0 — Restore sim-owned tick dependency boundary

> **Lifecycle: PROBATION** — simthing-sim dev-dependency seam restored; e10 guard passes; Terran Pirate integration proof moved to simthing-driver. Pending owner DA approval.

**Date:** 2026-06-18  
**PR:** SIMTHING-SIM-DEVDEP-SEAM-0  
**Base:** `master` after PR #765 / TERRAN-PIRATE-SCENARIO-SKELETON-0R

## Artifact lifecycle audit

| Artifact | Regime | Action |
|---|---|---|
| `crates/simthing-sim/Cargo.toml` | SEAM_BOUNDARY | Removed upward dev-deps |
| `crates/simthing-sim/tests/support/accumulator_plan_fixtures.rs` | SIM_LOCAL_FIXTURE | Created |
| `crates/simthing-sim/tests/forked_four_slot_input_list_tick.rs` | SIM_LOCAL_PROOF | Created (replaces terran_pirate in sim) |
| `crates/simthing-driver/tests/terran_pirate_skeleton_resident_tick.rs` | INTEGRATION_PROOF | Created |
| `crates/simthing-sim/tests/terran_pirate_skeleton_tick.rs` | OBSOLETE | Deleted |
| `docs/tests/simthing_sim_devdep_seam_0_results.md` | PROBATION | Created (this file) |
| `docs/0.8.3 Simthing Studio Production.md` | Living synthesis | Updated |
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated |

## Why this is not hygiene

PR #765 promoted canonical scenario authority but left `simthing-sim` dev-depending on `simthing-driver`, `simthing-mapeditor`, and `simthing-spec`. The `e10_does_not_import_arena_registry_into_simthing_sim` guard failed. Before Gu-Yang/PALMA GPU-resident horizon work, the sim-owned tick seam must accept already-compiled generic plans without upward imports to driver, spec, or Studio/editor crates.

## Dependency-boundary failure summary

**Root cause:** `crates/simthing-sim/Cargo.toml` listed `simthing-driver`, `simthing-mapeditor`, and `simthing-spec` under `[dev-dependencies]` (introduced for SKELETON-0/0R horizon proofs).

**Failure mode:** Cargo.toml string match in `e10_does_not_import_arena_registry_into_simthing_sim`.

**Not transitive:** Production `simthing-sim` dependencies were already clean; failure was dev-dependency and test-source imports only.

## Orientation answers

| Question | Answer |
|---|---|
| e10 failure cause? | `simthing-sim/Cargo.toml` contained `simthing-driver` (and mapeditor/spec) dev-deps |
| Cargo vs source vs transitive? | Cargo dev-dependencies + test source imports; not production transitive deps |
| sim tests requiring simthing-driver? | `terran_pirate_skeleton_tick.rs`, `accumulator_plan_tick_convergence.rs` |
| sim tests requiring simthing-spec? | `terran_pirate_skeleton_tick.rs` only |
| sim tests requiring simthing-mapeditor? | `accumulator_plan_tick_convergence.rs` only |
| Convertible to hand-built plans? | All CPU/GPU resident tick proofs over compiled `CompiledAccumulatorOpPlan` |
| Must move to integration owner? | Terran Pirate scenario→driver→sim resident GPU chain |
| sim compiles without upward dev-deps? | Yes — `[dev-dependencies] tempfile` only |
| Terran Pirate proof outside sim? | Yes — `crates/simthing-driver/tests/terran_pirate_skeleton_resident_tick.rs` |
| cargo test -p simthing-spec full pass? | Yes (including e10) |

## simthing-sim Cargo cleanup

**Before:**
```toml
[dev-dependencies]
tempfile = "3"
simthing-driver = { path = "../simthing-driver" }
simthing-mapeditor = { path = "../simthing-mapeditor" }
simthing-spec = { path = "../simthing-spec" }
```

**After:**
```toml
[dev-dependencies]
tempfile = "3"
```

## Sim-local generic plan fixtures

- `two_slot_vertical_input_list_plan()` — 2 slots, outputs `[20, 10]` on inputs `[10, 20]`
- `forked_four_slot_input_list_plan()` — 4 slots, outputs `[20, 80, 20, 20]` on inputs `[10, 20, 40, 30]`
- Semantic-free: `CombineFn::Sum`, `ConsumeMode::AddToTarget`, `SourceSpec::ConjunctiveCrossing`

## Moved Terran Pirate integration proof

**Location:** `crates/simthing-driver/tests/terran_pirate_skeleton_resident_tick.rs`

**Chain:** canonical JSON → `deserialize_scenario_authority` → `compile_structural_link_neighbor_sum_plan` → `SimGpuAccumulatorTickState` → scoped `ProofReadback` → CPU/GPU `[20, 80, 20, 20]` → authority non-mutation → ProofReadback→None non-leak.

## Dependency graph before/after

```text
Before (test seam):
  simthing-sim --dev--> simthing-driver, simthing-mapeditor, simthing-spec

After:
  simthing-driver --dep--> simthing-sim (integration proof host)
  simthing-sim --dep--> simthing-core, simthing-gpu, simthing-feeder only
```

## Production None-tick / readback preservation

All GPU readback-sensitive sim tests use process-wide file lock + `with_isolated_readback_gate_test`. `SimGpuReadbackPolicy::None` remains readback-free. ProofReadback→None non-leak preserved in sim-local and driver integration proofs.

## Gu-Yang / PALMA (deferred)

**Gu-Yang falloff borders:** grid N4 structural adjacency; `StructuredFieldStencilOp` / SaturatingFlux-class operator; saturating flux choke column + bounded field values; bounded theater first slice; typed atlas deferral; no border/frontline service.

**PALMA reach:** grid N4 structural adjacency; `WImpedanceComposeOp` → `MinPlusStencilOp`; D field + W impedance/feedstock; bounded theater first slice; typed atlas deferral; no routes/predecessors/pathfinding.

Hyperlane link gather remains bounded `AccumulatorOp` Sum-over-INPUT_LIST — not conflated with Gu-Yang/PALMA grid N4 stencils.

## Big-endian backlog

Deferred: explicit little-endian byte helpers, cross-platform byte-order evidence, host-endian bytemuck replacement in canonical artifact byte proofs.

## Forbidden-token scan

No route/predecessor/pathfinding/border-service semantics in changed sources.

## Tests added/changed

| Change | Detail |
|---|---|
| Deleted | `crates/simthing-sim/tests/terran_pirate_skeleton_tick.rs` |
| Added | `forked_four_slot_input_list_tick.rs`, `support/accumulator_plan_fixtures.rs`, `support/readback_gate.rs` |
| Updated | `accumulator_plan_tick_convergence.rs` — hand-built two-slot plan |
| Added | `terran_pirate_skeleton_resident_tick.rs` (driver integration) |
| Extended | `e10_does_not_import_arena_registry_into_simthing_sim` guard |

## Commands run (exact outcomes)

| Command | Status | Notes |
|---|---|---|
| `cargo fmt --all -- --check` | PASS | |
| `cargo check -p simthing-sim` | PASS | |
| `cargo test -p simthing-sim --test accumulator_plan_tick_convergence` | PASS | 28/28 |
| `cargo test -p simthing-sim --test forked_four_slot_input_list_tick` | PASS | 6/6 |
| `cargo check -p simthing-driver` | PASS | |
| `cargo test -p simthing-driver --test terran_pirate_skeleton_compile` | PASS | 5/5 |
| `cargo test -p simthing-driver --test terran_pirate_skeleton_resident_tick` | PASS | 4/4 |
| `cargo test -p simthing-driver terran_pirate` | SKIP | Workspace compiles all driver test binaries; unrelated `bh3_authoring` binary fails compile — use `--test` targets above |
| `cargo check -p simthing-spec` | PASS | |
| `cargo test -p simthing-spec` | PASS | Full suite including e10 |
| `cargo test -p simthing-spec --test e10_resource_flow_admission e10_does_not_import_arena_registry_into_simthing_sim` | PASS | |
| `cargo test -p simthing-mapeditor --test terran_pirate_skeleton` | PASS | 10/10 |
| `cargo test -p simthing-mapeditor --test accumulator_convergence_1_guards` | PASS | 19/19 (after production doc update) |
| `cargo test -p simthing-gpu --test debug_readback_scope` | PASS | 5/5 |
| `cargo test -p simthing-clausething --test stead_spatial_contract_guards` | PASS | |
| `cargo test -p simthing-clausething --test mapgen_lattice_hierarchy` | PASS | |
| `cargo test -p simthing-clausething --test mapgen_rf_stead_binding` | PASS | |
| `cargo test -p simthing-clausething --test mapgen_movement_front` | PASS | |
| `git diff --check` | PASS | |

## Windows / resource-limit notes

`cargo test -p simthing-spec` and sim GPU tests run with `CARGO_BUILD_JOBS=1` when needed. Process-wide temp-file lock serializes GPU readback tests across sim/driver test binaries sharing `debug_readback_allowed`.

## Files changed

- `crates/simthing-sim/Cargo.toml`
- `crates/simthing-sim/tests/accumulator_plan_tick_convergence.rs`
- `crates/simthing-sim/tests/forked_four_slot_input_list_tick.rs`
- `crates/simthing-sim/tests/support/` (new)
- `crates/simthing-driver/tests/terran_pirate_skeleton_resident_tick.rs` (new)
- `crates/simthing-spec/tests/e10_resource_flow_admission.rs`
- `crates/simthing-mapeditor/tests/accumulator_convergence_1_guards.rs`
- `docs/tests/simthing_sim_devdep_seam_0_results.md`
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `Cargo.lock`

## Deleted/archived artifacts

- `crates/simthing-sim/tests/terran_pirate_skeleton_tick.rs` (superseded by sim-local forked plan + driver integration)

## Deferred work

- Gu-Yang falloff borders (STEAD §10)
- PALMA reach field (STEAD §10)
- Big-endian portable byte-proof helpers
- Full terran-pirate play-out

## DA status

**PROBATION** — dependency seam restored; e10 passes; awaiting owner sign-off.