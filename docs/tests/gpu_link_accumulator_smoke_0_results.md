# GPU-LINK-ACCUMULATOR-SMOKE-0 — Vertical-seed-pulled GPU accumulation over structural links

> **Lifecycle: PROBATION** — pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## GPU adapter evidence state

**REAL_ADAPTER_OBSERVED** — GPU link accumulator smoke tests executed on a real adapter in this environment (8 GPU tests + 4 mapeditor bridge GPU tests passed; no adapter skips).

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Added GPU-LINK-ACCUMULATOR-SMOKE-0 PROBATION row |
| `docs/tests/gpu_link_accumulator_smoke_0_results.md` | PROBATION | This report |
| `docs/tests/vertical_test_scenario_seed_0_results.md` | PROBATION | Vertical seed prerequisite |
| `docs/tests/gpu_structural_validation_wgsl_0_results.md` | PROBATION | Validation prerequisite |
| `docs/0.8.3 Simthing Studio Production.md` | PROBATION | Standing Studio production synthesis updated |

## Why this is not hygiene

This pass answers: can the loaded `runtime_vertical_seed` scenario drive an actual GPU computation over its canonical structural links, with output compared against a CPU oracle, while keeping `SimThingScenarioSpec` as authority? Yes — the fixture now pulls a behavior proof, not generic scaffolding.

## Pre-edit orientation answers

| Question | Answer |
|---|---|
| Reusable accumulator patterns? | `structural_validation.wgsl` dispatch/bind-group/readback pattern; `structural_upload.rs` buffer residency; existing `accumulator_op`/`transfer_accumulator` remain separate domain stacks |
| Invariant proved over vertical seed? | `input=[10,20]` → `output=[20,10]` via undirected neighbor sum over canonical link dense indices 0↔1 |
| Fixed-point exactness? | Yes — `i32` values and `atomicAdd` on integer storage; CPU oracle uses `saturating_add` |
| GPU adapter evidence? | **REAL_ADAPTER_OBSERVED** |
| Not pathfinding/RF/MF/runtime? | Neighbor sum only; no path choice, no RF/MF kernels, no sim loop, no semantic WGSL |
| Output not authority? | Accumulator outputs are GPU readback/projection cache; scenario save/load unchanged |

## Vertical seed accumulator invariant

| Field | Value |
|---|---|
| `location_count` | 2 |
| `link_count` | 1 |
| `input` | `[10, 20]` |
| `expected_output` | `[20, 10]` |
| `invalid_link_endpoint_count` | 0 |
| `self_link_count` | 0 |

## CPU oracle summary

`cpu_structural_link_accumulate_i32(location_count, links, input_values)` — rejects wrong input length, invalid endpoints, self-links; deterministic neighbor accumulation. Covered: vertical seed, chain (3 nodes), triangle (3 nodes).

## WGSL pass summary

`structural_link_accumulator.wgsl` — one thread per link; reads frame/link/input buffers; `atomicAdd` on `i32` output; compact `StructuralLinkAccumulatorReportGpu` (32 B). Runs after structural validation on uploaded buffers.

## Fixed-point/exactness decision

`i32` fixed-point scalars with integer `atomicAdd` — avoids float atomic nondeterminism. CPU oracle matches GPU readback byte-for-byte on valid packets.

## Forbidden-token scan

WGSL scanned for: route, predecessor, pathfinding, movement_order, fleet, faction, owner, border, frontline, combat, economy, diplomacy.

## Valid GPU proof

Vertical seed, chain, and triangle packets: GPU output matches CPU oracle; report counters zero on valid rows.

## Bad-row GPU proof

Intentionally bad uploaded rows: endpoint `99` detected; self-link dense pair detected.

## Tests added

**simthing-gpu** (`structural_link_accumulator.rs`): 6 CPU oracle tests, 4 layout/scan tests, 8 GPU tests.

**simthing-mapeditor** (`tests/runtime_vertical_seed.rs`): 4 mapeditor bridge tests, 2 doc guard tests.

## Commands run

```text
cargo fmt --all -- --check
cargo check -p simthing-gpu
cargo test -p simthing-gpu
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor --test runtime_vertical_seed
cargo test -p simthing-core
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test stead_spatial_contract_guards
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy
cargo test -p simthing-clausething --test mapgen_rf_stead_binding
cargo test -p simthing-clausething --test mapgen_movement_front
cargo test -p simthing-spec --lib
git diff --check
```

## Windows/resource-limit notes

Initial parallel `cargo test -p simthing-spec` hit Windows paging-file linker errors (exit code 1102). Reran with `CARGO_BUILD_JOBS=1`; `cargo test -p simthing-spec --lib` (47 tests) passed. All packages required by this PR ran successfully.

## Files changed

- `crates/simthing-gpu/src/structural_link_accumulator.rs`
- `crates/simthing-gpu/src/shaders/structural_link_accumulator.wgsl`
- `crates/simthing-gpu/src/lib.rs`
- `crates/simthing-mapeditor/src/scenario_projection.rs`
- `crates/simthing-mapeditor/src/lib.rs`
- `crates/simthing-mapeditor/tests/runtime_vertical_seed.rs`
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/gpu_link_accumulator_smoke_0_results.md`

## Deleted/archived artifacts

None.

## Deferred work

RF/Accumulator full execution, Movement-Front execution, heatmap rendering, pathfinding, route/predecessor semantics, runtime sim loop, live Studio GPU UI wiring, runtime vertical-test execution beyond seeded authority.

## DA status

**PROBATION** — pending owner design-authority approval. GPU evidence: **REAL_ADAPTER_OBSERVED**.