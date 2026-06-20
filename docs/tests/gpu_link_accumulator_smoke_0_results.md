# GPU-LINK-ACCUMULATOR-SMOKE-0 — Vertical-seed-pulled GPU accumulation over structural links

> **Lifecycle: PROBATION** — pending owner design-authority approval. Do not treat as CURRENT_EVIDENCE until DA sign-off.

## GPU adapter evidence state

**REAL_ADAPTER_OBSERVED** — GPU link accumulator smoke tests executed on a real adapter in this environment (11 GPU tests + 4 mapeditor bridge GPU tests passed; no adapter skips).

## Artifact lifecycle audit

| Artifact | Lifecycle | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | GPU-LINK-ACCUMULATOR-SMOKE-0 PROBATION row (bit-exact contract) |
| `docs/tests/gpu_link_accumulator_smoke_0_results.md` | PROBATION | This report |
| `docs/tests/vertical_test_scenario_seed_0_results.md` | PROBATION | Vertical seed prerequisite; PR #754 Windows resource-limit note amended |
| `docs/tests/gpu_structural_validation_wgsl_0_results.md` | PROBATION | Validation prerequisite |
| `docs/design_0_0_8_3_studio_production.md` | PROBATION | Standing Studio production synthesis updated |

## Why this is not hygiene

This pass answers: can the loaded `runtime_vertical_seed` scenario drive an actual **bit-exact** GPU computation over its canonical structural links, with output compared against a CPU oracle, while keeping `SimThingScenarioSpec` as authority? Yes — the fixture now pulls a behavior proof, not generic scaffolding.

## Pre-edit orientation answers

| Question | Answer |
|---|---|
| Reusable accumulator patterns? | `structural_validation.wgsl` dispatch/bind-group/readback pattern; `structural_upload.rs` buffer residency; existing `accumulator_op`/`transfer_accumulator` remain separate domain stacks |
| Invariant proved over vertical seed? | `input=[10,20]` → `output=[20,10]` via undirected neighbor sum over canonical link dense indices 0↔1 |
| Exact arithmetic semantics? | Signed `i32` fixed-point scalars; for each link `(a,b)`: `output[a] += input[b]`, `output[b] += input[a]`; CPU oracle uses **`checked_add`**; overflow is an explicit error before GPU dispatch |
| CPU oracle arithmetic? | **`checked_add`** — not `saturating_add`, not silent wrap |
| WGSL matches CPU oracle? | **Yes** — `atomicAdd` on `i32` storage only after CPU oracle succeeds (inputs proven non-overflowing); GPU readback values and little-endian bytes match CPU oracle |
| GPU adapter evidence? | **REAL_ADAPTER_OBSERVED** |
| Not pathfinding/RF/MF/runtime? | Neighbor sum only; no path choice, no RF/MF kernels, no sim loop, no semantic WGSL |
| Output not authority? | Accumulator outputs are GPU readback/projection cache; scenario save/load unchanged |
| PR #754 evidence correction? | `vertical_test_scenario_seed_0_results.md` amended to record parallel `cargo test -p simthing-spec` paging-file failure and serial `--lib` rerun |

## Vertical seed accumulator invariant

| Field | Value |
|---|---|
| `location_count` | 2 |
| `link_count` | 1 |
| `input` | `[10, 20]` |
| `expected_output` | `[20, 10]` |
| `invalid_link_endpoint_count` | 0 |
| `self_link_count` | 0 |

## Bit-exact arithmetic contract

```text
Input values are signed i32 fixed-point integers.
Each output is the exact sum of neighbor input values over canonical structural links.
All sums are proven in-range before GPU dispatch via CPU checked_add oracle.
Overflow is an error (AccumulatorOverflow), not saturation and not silent wrap.
CPU oracle uses checked_add.
GPU tests use value ranges that cannot overflow.
CPU oracle arithmetic and WGSL atomicAdd have the same observable semantics for every tested case.
```

Byte proof: `structural_link_accumulator_output_bytes` uses `bytemuck::cast_slice` (little-endian host assumption documented); GPU readback bytes must equal CPU oracle bytes.

## CPU oracle summary

`cpu_structural_link_accumulate_i32(location_count, links, input_values)` — rejects wrong input length (`InvalidInputLength`), invalid endpoints (`InvalidEndpoint`), self-links (`SelfLink`), and overflow (`AccumulatorOverflow`). Covered: vertical seed, chain (3 nodes), triangle (3 nodes), bad endpoint, self-link, wrong length, overflow.

## WGSL pass summary

`structural_link_accumulator.wgsl` — one thread per link; reads frame/link/input buffers; `atomicAdd` on `i32` output; compact `StructuralLinkAccumulatorReportGpu` (32 B). Runs after structural validation on uploaded buffers. Host runs CPU oracle first; dispatch only when oracle succeeds.

## Forbidden-token scan

WGSL scanned for: route, predecessor, pathfinding, movement_order, fleet, faction, owner, border, frontline, combat, economy, diplomacy.

## Valid GPU proof

Vertical seed, chain, and triangle packets: GPU output values and bytes match CPU oracle; report counters zero on valid rows.

## Bad-row GPU proof

Intentionally bad uploaded rows: endpoint `99` detected; self-link dense pair detected (via `accumulate_structural_rows_on_gpu_report_probe`).

## Overflow proof

CPU oracle rejects overflow inputs; `gpu_link_accumulator_rejects_overflow_before_dispatch` confirms GPU path does not dispatch on overflow-prone inputs.

## Tests added

**simthing-gpu** (`structural_link_accumulator.rs`): 7 CPU oracle tests, 4 layout/scan tests, 11 GPU tests (including 3 byte-match tests).

**simthing-mapeditor** (`tests/runtime_vertical_seed.rs`): 4 mapeditor bridge tests (including byte equality), 2 doc guard tests.

## Commands run

```text
cargo fmt --all -- --check
cargo check -p simthing-gpu
cargo test -p simthing-gpu structural_link_accumulator
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor --test runtime_vertical_seed
cargo test -p simthing-core
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test stead_spatial_contract_guards
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy
cargo test -p simthing-clausething --test mapgen_rf_stead_binding
cargo test -p simthing-clausething --test mapgen_movement_front
CARGO_BUILD_JOBS=1 cargo test -p simthing-spec --lib
git diff --check
```

## Windows/resource-limit notes

| Item | Detail |
|---|---|
| Failed command | `cargo test -p simthing-spec` (default parallel build/link) during PR #754 validation |
| OS/resource reason | Windows paging file too small for parallel linker jobs (exit code **1102**) |
| Serial/scoped rerun | `CARGO_BUILD_JOBS=1 cargo test -p simthing-spec --lib` (47 passed) |
| Test binaries that ran on rerun | `simthing-spec` lib unit tests |
| Test binaries that did **not** run in failed parallel attempt | `simthing-spec` integration test binaries that failed to link under parallel jobs |
| PROBATION impact | **Unchanged** — required validations for this PR passed after serial rerun |

## Files changed

- `crates/simthing-gpu/src/structural_link_accumulator.rs`
- `crates/simthing-gpu/src/shaders/structural_link_accumulator.wgsl`
- `crates/simthing-gpu/src/lib.rs`
- `crates/simthing-mapeditor/src/scenario_projection.rs`
- `crates/simthing-mapeditor/src/lib.rs`
- `crates/simthing-mapeditor/tests/runtime_vertical_seed.rs`
- `docs/design_0_0_8_3_studio_production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/vertical_test_scenario_seed_0_results.md`
- `docs/tests/gpu_link_accumulator_smoke_0_results.md`

## Deleted/archived artifacts

None.

## Deferred work

RF/Accumulator full execution, Movement-Front execution, heatmap rendering, pathfinding, route/predecessor semantics, runtime sim loop, live Studio GPU UI wiring, runtime vertical-test execution beyond seeded authority.

## DA status

**PROBATION** — pending owner design-authority approval. GPU evidence: **REAL_ADAPTER_OBSERVED**.