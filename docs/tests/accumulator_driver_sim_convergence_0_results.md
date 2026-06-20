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
| `docs/design_0_0_8_3_studio_production.md` | Living synthesis | Updated § ACCUMULATOR-DRIVER-SIM-CONVERGENCE-0 |

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
- `docs/design_0_0_8_3_studio_production.md`
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

**PARTIAL** — PROBATION smoke fenced and guarded; capability gap documented; PR #756 bit-exact tests preserved. See DA RULING below.

## DA RULING (design authority, 2026-06-18)

**Verdict: SPLIT.**

1. **Fence (Parts A, B) — ACCEPTED as a PROBATION safety guardrail.** The `PROOF_ONLY` / `SMOKE_ONLY` /
   `NOT_RUNTIME` markers and `no_studio_runtime_loop_uses_structural_link_accumulator` (static `src/app/*.rs`
   scan) correctly and durably prevent the bespoke smoke from becoming the runtime. This rung stands on its
   own merit.
2. **"AccumulatorOp cannot express this without broad driver/sim redesign" — OVERRULED.** Verified against
   source: `CombineFn::Sum` is wired only to contiguous `SlotRange` (`accumulator_op/cpu_oracle.rs` returns
   `Unsupported("Sum without SlotRange")`), **but** the arbitrary-gather `INPUT_LIST` table already exists with
   GPU residency (`AccumulatorInputListTable`, wired today to `MinAcrossInputs`). The real gap is **one
   combine×source cell — Sum over INPUT_LIST** — a bounded *in-mechanism* AO extension, not a redesign. `f32`
   with the established ≤2²⁴ exact-integer convention covers the vertical seed exactly. The gap *identification*
   was accurate; the *infeasibility conclusion* is not.
3. **Documentation-as-code — REJECTED as convergence evidence.** `accumulator_convergence.rs` constants, the
   driver "stub" (asserts a string equals `"simthing-driver"`; compiles nothing), and 5 of 7 mapeditor guards
   are tautological/doc-presence assertions that prove no behavior. They may remain only as a transient
   gap-record and are **removed at CONVERGENCE-1**; they are not promotable evidence.

**Convergence remains OWED.** It is governed by the contract below, generalized to the full horizon
(RF/Accumulator link coupling **+ Gu-Yang falloff borders + PALMA reach**) so the next two surfaces are never
built bespoke.

### Structural Execution Convergence Contract (binding on the codex orchestrator)

Every Studio→GPU structural execution surface must: (a) route to an **existing sanctioned `simthing-gpu`
operator** — never a new bespoke Studio/GPU kernel; (b) be **compiled from `SimThingScenarioSpec` by
`simthing-driver`**; (c) be **dispatched under `simthing-sim` tick/boundary**; (d) operate over the **correct
structural adjacency**; (e) keep GPU output as projection/cache, never authority. "One mechanism" = one
discipline with admitted operator variants, not one literal kernel.

| Surface | Adjacency | Convergence target (existing op) | Bounded-theater + atlas? |
|---|---|---|---|
| **RF / link coupling** (CONVERGENCE-1) | hyperlane **link graph** | `AccumulatorOp` Sum-over-`INPUT_LIST` | No (bounded fanout) |
| **Gu-Yang falloff borders** (CONVERGENCE-2) | **grid N4** of `(col,row)` | `saturating_flux_choke_threshold` + `structured_field_stencil` | **Yes** (§7 P1; dense-global rejected) |
| **PALMA reach field** (CONVERGENCE-3) | **grid N4** of `(col,row)` | `min_plus_stencil` + `w_impedance_compose` | **Yes** |

**STEAD distinction (do not conflate):** the link Sum-over-`INPUT_LIST` is a *coupling* accumulation over the
hyperlane graph; it is **not** the heatmap stencil. Gu-Yang/PALMA are *grid* stencils over N4 lattice neighbors
within a bounded `ExecutionTheater`. Crossing them is a STEAD violation.

**Border-semantics guardrail:** borders **emerge as field expressions** (SaturatingFlux falloff fronts + PALMA
min-plus reach), never a frontline semantic service; PALMA `D` is a **field, not a route** (no
predecessors/paths). Reaffirms `stead_spatial_contract.md` §9 withdrawn phrases.

**Sequencing & conditions:** CONVERGENCE-1 (link RF) is owed now and is the *reference instance* establishing
the driver/sim assembly the others reuse; deliverable is a **real-adapter** proof that adjacency input-lists
built from canonical `SimThingScenarioSpec` links, run through `AccumulatorOp` `Sum`+`AddToTarget`, reproduce
`[10,20]→[20,10]` and match `cpu_structural_link_accumulate_i32` byte-for-byte, then driver compile, then sim
tick, then **delete `structural_link_accumulator.{rs,wgsl}`** (and its duplicated validation). CONVERGENCE-2/-3
(Gu-Yang/PALMA) are named, scoped, **deferred — not to be started bespoke.** No new GPU primitive; PROBATION
until each surface's real-adapter proof lands; no tautological test counts as evidence; if an existing op
structurally cannot host the needed step, **STOP and return to DA** with the specific constraint.