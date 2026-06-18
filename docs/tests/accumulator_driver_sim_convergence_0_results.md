# ACCUMULATOR-DRIVER-SIM-CONVERGENCE-0 — Structural execution convergence results

> **Lifecycle: PARTIAL** — direct AccumulatorOp / AO-WGSL-0 migration not landed in this PR; capability gap documented; PROBATION smoke fenced and guarded.

**Date:** 2026-06-18  
**PR:** ACCUMULATOR-DRIVER-SIM-CONVERGENCE-0  
**Base:** `master` after PR #757 (STUDIO-SCENARIO-NATIVE-FILEDIALOG-0)

## Artifact lifecycle audit

| Artifact | Regime | Action |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated with convergence row |
| `docs/tests/gpu_link_accumulator_smoke_0_results.md` | PROBATION | Preserved — baseline bit-exact smoke evidence |
| `docs/tests/accumulator_driver_sim_convergence_0_results.md` | PARTIAL | Created (this file) |
| `docs/0.8.3 Simthing Studio Production.md` | Living synthesis | Updated § ACCUMULATOR-DRIVER-SIM-CONVERGENCE-0 |

No scratch logs, temp GPU dumps, or superseded reports deleted. Live ledger preserved.

## Why this is not hygiene

This PR secures the **constitutional execution spine** before further byte-order or GPU hygiene work. The horizon is playing a loaded scenario simulation out end-to-end — that must not mature Studio direct-to-GPU bespoke kernels as runtime execution. Big-endian/portable byte-proof hardening is explicitly deferred behind this structural convergence.

## Opus verdict ingestion

Opus identified that the proven stack:

```text
runtime_vertical_seed.simthing-scenario.json
  -> SimThingScenarioSpec authority
  -> Studio scenario IO load
  -> StudioStructuralProjection
  -> StudioGpuStructuralUploadPacket
  -> GPU structural buffers
  -> WGSL structural validation
  -> bespoke structural_link_accumulator smoke
  -> checked i32 CPU oracle + GPU byte proof
```

is valid **PROBATION proof work** but not yet a constitutional execution foundation. This PR fences that path and names driver/sim/AccumulatorOp as the convergence target.

## Why prior bypass was allowed

Earlier bypass of canonical AccumulatorOp was allowed only because work was explicitly bounded as PROBATION proof scaffolding:

- prove scenario-derived structural packet
- prove GPU buffer residency
- prove WGSL structural validation
- prove one tiny bit-exact compute smoke

That bypass is no longer acceptable for the horizon.

## Current structural_link_accumulator status

- Marked **proof_only / smoke_only / not_runtime** in Rust module docs, public API doc comments, and WGSL header.
- Constants: `STRUCTURAL_LINK_ACCUMULATOR_PROOF_ONLY`, `STRUCTURAL_LINK_ACCUMULATOR_SMOKE_ONLY`, `STRUCTURAL_LINK_ACCUMULATOR_NOT_RUNTIME`.
- PR #756 bit-exact smoke tests preserved and still pass.
- Must be retired, wrapped by, or re-expressed through canonical AccumulatorOp before runtime promotion.

## PR #756 bit-exactness preservation

- CPU oracle: `checked_add`, overflow-as-error before dispatch.
- Vertical seed: `input=[10,20]` → `output=[20,10]`.
- GPU readback value equality and byte equality against CPU oracle.
- All existing `gpu_link_accumulator_*` tests remain passing.

## AccumulatorOp capability analysis

**Existing capabilities (AO-WGSL-0 / AccumulatorOp v2):**

- `f32` slot/column value grid (`AccumulatorOpGpu`, `AccumulatorOpSession`)
- Source kinds: `SlotValue`, `SlotRange`, `INPUT_LIST`, EML trees, threshold emissions
- Combine modes: `SUM`, `IDENTITY`, reductions, EML eval, etc. (floating-point)
- Consume: `AddToTarget`, `ResetTarget`, etc.
- CPU oracle: `execute_ops_cpu` over `AccumulatorOp` plans
- AO-WGSL-0 fast path via OrderBand-gated plans (`execute_orderband_bands`)

**Vertical-seed invariant to express canonically:**

```text
For each canonical structural link (a,b):
  output[a] += input[b]
  output[b] += input[a]
input = [10, 20] → expected_output = [20, 10]
```

**Can AccumulatorOp express this now?** **No** — not without broad driver/sim redesign in this PR.

## Direct migration result: gap (outcome B)

Direct convergence to AccumulatorOp / AO-WGSL-0 is **too broad for this PR**. Exact missing generic capabilities:

AccumulatorOp needs scenario-derived structural coupling rows:

- dense source index
- dense target index
- input scalar channel
- output scalar channel
- combine mode: checked exact sum (i32 or explicitly documented fixed-point integer semantics)
- overflow rejected or errored before GPU dispatch
- driver compile/assembly from `SimThingScenarioSpec` links into generic op plans
- sim tick/boundary lifecycle for dispatch (not Studio proof helpers)

Additional blockers:

- AccumulatorOp value grid is **f32**; PROBATION smoke contract is **i32** bit-exact with `checked_add`.
- AO-WGSL-0 plan shapes are OrderBand/RF-oriented; structural neighbor gather is not a first-class generic plan shape.
- No `simthing-driver` production path compiles scenario links → AccumulatorOp plans.
- No `simthing-sim` tick/boundary wiring owns structural neighbor accumulation dispatch.

## Driver/sim execution responsibility statement

| Layer | Responsibility |
|---|---|
| `SimThingScenarioSpec` | Model authority |
| `simthing-mapeditor` (Studio) | Projection, presentation, explicit proof helpers only |
| `simthing-driver` | Scenario/runtime compile or assembly into generic operations |
| `simthing-sim` | Tick, boundary lifecycle, simulation progression |
| `simthing-gpu` | Semantic-free generic GPU primitives requested by driver/sim |

Compile-plan stub: `crates/simthing-driver/tests/accumulator_driver_sim_convergence_stub.rs`

## Studio-runtime-bypass guard

- Verified: no Bevy `src/app/*.rs` file references structural link accumulator smoke paths.
- Proof helpers `prove_gpu_link_accumulator_smoke_blocking` and `prove_runtime_vertical_seed_gpu_link_accumulator_blocking` documented as proof_only / smoke_only / not_runtime.
- Test: `no_studio_runtime_loop_uses_structural_link_accumulator` in `accumulator_convergence_guards.rs`.

## Big-endian / portable byte-proof backlog (deferred)

The current byte-proof is valid for current little-endian development evidence. Future portability hardening should replace host-endian bytemuck byte evidence with explicit little-endian conversions for canonical artifacts:

- `i32::to_le_bytes` for CPU oracle byte vectors
- explicit readback byte interpretation
- cross-platform byte-order tests where possible

Deferred until after structural execution convergence.

## Forbidden-token scan

Scanned new/updated GPU-adjacent identifiers and WGSL. Part H forbidden semantic token scan: clean (no new domain semantics in GPU/WGSL surfaces).

## Tests added

| Test | Location |
|---|---|
| `structural_link_accumulator_marked_proof_only` | `simthing-gpu`, `simthing-mapeditor` |
| `structural_link_accumulator_public_docs_say_not_runtime` | `simthing-gpu` |
| `gpu_link_accumulator_smoke_bit_exact_tests_still_pass` | `simthing-gpu` |
| `no_studio_runtime_loop_uses_structural_link_accumulator` | `simthing-mapeditor` |
| `accumulator_convergence_gap_report_exists` | `simthing-mapeditor` |
| `accumulator_convergence_gap_report_names_missing_generic_capability` | `simthing-mapeditor`, `simthing-gpu` |
| `accumulator_convergence_gap_report_does_not_use_domain_semantics` | `simthing-mapeditor`, `simthing-gpu` |
| `production_doc_names_driver_sim_as_execution_owner` (test name) | `simthing-mapeditor` |
| `production_doc_names_accumulator_op_as_convergence_target` | `simthing-mapeditor` |
| `driver_compile_plan_stub_names_driver_as_structural_accumulator_owner` (test name) | `simthing-driver` |

## Commands run

See PR validation section (cargo fmt/check/test per crate, constitutional guards, `git diff --check`).

## Windows / resource-limit notes

If paging-file/linker limits occur: rerun serially with `CARGO_BUILD_JOBS=1` and record in PR body.

## Files changed

- `crates/simthing-gpu/src/structural_link_accumulator.rs`
- `crates/simthing-gpu/src/shaders/structural_link_accumulator.wgsl`
- `crates/simthing-gpu/src/accumulator_convergence.rs` (new)
- `crates/simthing-gpu/src/lib.rs`
- `crates/simthing-mapeditor/src/scenario_projection.rs`
- `crates/simthing-mapeditor/tests/accumulator_convergence_guards.rs` (new)
- `crates/simthing-driver/tests/accumulator_driver_sim_convergence_stub.rs` (new)
- `docs/0.8.3 Simthing Studio Production.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/accumulator_driver_sim_convergence_0_results.md` (new)

## Deleted / archived artifacts

None.

## Deferred work

- Direct AccumulatorOp / AO-WGSL-0 re-expression of structural neighbor sum (i32 checked exact sum)
- Driver production compile/assembly from scenario links
- Sim tick/boundary lifecycle for accumulation dispatch
- Full scenario play-out execution (deferred until above)
- Big-endian/portable byte-proof hardening

## DA status

**PARTIAL** — PROBATION smoke fenced and guarded; capability gap documented; PR #756 bit-exact tests preserved. Pending executive DA approval for promotion.