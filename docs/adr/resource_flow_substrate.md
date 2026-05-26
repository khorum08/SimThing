# ADR: Resource Flow Substrate

**Status:** Accepted  
**Date:** 2026-05-26  
**Authors:** Opus 4.7 (synthesis), with critique from Gemini and ChatGPT, and design picks by project owner  
**Extends:** `docs/adr_accumulator_op_v2.md` (the AccumulatorOp v2 primitive is the substrate this ADR builds on)  
**Implementation companion:** `docs/workshop/resource_flow_adr_shaping.md` — full schemas, kernel patterns, and implementation evidence  
**Feasibility evidence:** `docs/workshop/resource_flow_feasibility_opus.md` — source-grounded verification of the substrate's compatibility

---

## Context

The AccumulatorOp v2 ADR (2026-05-24) established a unified GPU
gather/combine/gate/scatter primitive and the per-transaction transfer
substrate (C-8c). It did not specify how continuous resource dynamics —
food, energy, research, suppression pressure, trade access — compose at
scale across thousands of participants. The original D-1 design gate
was reserved for that question, scoped as a GPU hot-pool allocator
strategy for many-to-one tick-time contention. The workshop measured
that contention regime at 0.14× CPU performance (7× slower) on the
hotspot scenario (16 pools × 100k requesters).

Two synthesis passes (Gemini, then ChatGPT) and a feasibility evaluation
established that the contention regime D-1 was scoped to solve does not
need a GPU allocator at all — it needs a different *architecture*.
Continuous resource flow expressed as **per-tick parallel rate
reduction + per-boundary hierarchical allocation** eliminates the
shared-pool write at tick time entirely. The original D-1 dissolves
for the continuous-flow case.

This ADR formalizes the substrate that replaces it.

---

## Decision

**Adopt the Resource Flow Substrate: a unified GPU-resident model for
continuous resource dynamics, hierarchical allocation, and arena-to-arena
coupling, built entirely on the existing AccumulatorOp v2 primitive.**

The substrate is symmetric. Reduction sweeps leaf→root via the existing
C-5/C-6 OrderBand mechanism (intrinsic Flow aggregates upward).
Allocation sweeps root→leaf via a new OrderBand sweep in reverse
direction (budget partitions downward). Both passes use the same kernel
primitive; only the band ordering differs. The boundary-time allocator
of the original D-1 design is replaced by per-tick GPU passes that
write to independent participant columns with no shared-slot contention.

### Four architectural commitments

These are load-bearing; together they define the substrate.

**1. Unified field substrate.** One `ArenaRegistry` artifact per
session. One execution model across all arenas (food, research,
piracy_suppression, ...). Many named arenas, distinguished by column
range, not by kernel variant. Explicit coupling edges between arenas
carry declared delay form.

**2. Hierarchical allocation via overlay-modifiable weights.** Each
intermediate participant in an arena's tree (faction → planet →
district → factory) can allocate its received budget to its children.
Allocation is a reverse-direction OrderBand sweep. Each intermediate
allocator is dual-role: contributes `intrinsic_flow` upward
(participates in reduction), receives `allocated_flow` downward
(participates in allocation). The two roles do not conflict because
they participate in opposite-direction sweeps.

**3. Per-coupling delay form, designer-picked.** Arena coupling edges
declare their delay form per edge: `Algebraic` (same tick),
`OneTickDelay` (reads previous values), `BoundaryStage { stage }`
(boundary-time ordered), or `AccumulatorState { property }` (routes
through a Balance column). The spec compiler rejects any dependency
cycle whose edges are all `Algebraic` — every cycle must contain at
least one delay-bearing edge.

**4. Default-overlay allocation policy.** The allocation policy at
each intermediate node is expressed through the *existing* overlay
stack on weight columns, not through a new policy enum. Default
behavior in the absence of overlays is proportional to children's
`Demand`. Policies, interdiction, player intent, and AI policy all
compose through Add/Multiply/Set OrderBands on weight columns. EML
is available as the weight-evaluator for conditional logic
(`weight = 2.0 if intensity > 0.7 else 1.0`).

### Conservation policy (precision-aware)

Three independent conservation invariants, each enforced where it
naturally belongs:

**Per-recipe (exact):** `MinAcrossInputs + SubtractFromAllInputs`
guarantees `Σ_j ΔNeed_j + emit_count × Σ_j c_j = 0` exactly per
invocation. Unchanged from C-8c. Enforced structurally by the kernel
and the planner.

**Per-allocator-level (approximate-deterministic):** For every
intermediate `I` with children `C_1, ..., C_n`:

```
| Σ_i disbursed(I → C_i) − budget(I) | ≤ O(ε × n)
```

The bound is O(machine epsilon × n_children) per allocation step,
arising from per-child f32 division `child_share = budget × w_i /
weight_sum`. Even though `Σ w_i = weight_sum` by Sum-reduction
construction, the *sum of independently-computed quotients* is not
exactly `budget` in f32. **The residual integrates into the parent's
`Balance` via the existing `governed_by` machinery — the same path
that handles leaf residuals.** The error is deterministic (same
inputs produce same residual), so replay is bit-exact. Over time the
residual is bounded because Balance integration closes the gap on
subsequent ticks.

This is acceptable because allocation IS continuous flow — soft drift
of O(ε × n) per tick is the correct semantics for a rate signal.
Discrete source-debit transfers (E-2 discrete builder) remain exact
via `SubtractFromSource`.

**Per-arena (structural):** Total intrinsic flow plus inbound coupling
contributions equals total leaf allocations plus Balance changes plus
emission consumption. The spec compiler verifies no orphan
participants exist (every participant traces to a declared
intrinsic-flow source or an inbound coupling).

**Zero-weight handling.** When `weight_sum == 0` at an intermediate
node (all children have zero demand and no policy overlays), every
`child_share` evaluates to 0 via EML `SELECT`. The undisbursed budget
integrates into the parent's own Balance via the standard governed_by
path. No special-case kernel handling required beyond the SELECT
guard in the per-arena allocation EML tree.

### Draconian content guardrail (spec-layer firewall)

The runtime substrate cannot defend against explosive content
authored at the spec layer. `simthing-spec` is the enforcement
layer. The compiler **rejects at session build time** (not warns):

- Implicit participation (property possession ≠ admission)
- Unbounded wildcards without declared upper bound
- Per-arena cap violations (`max_participants`,
  `max_coupling_fanout`, `max_orderband_depth`)
- Coupling cycles without any delay-bearing edge
- Hidden fanout exceeding declared budget

Every arena declares its `FissionPolicy` for spawned children:
`Inherit`, `Reevaluate` (default), or `Reject`. **`Custom` is
deferred to a follow-on ADR** if a game later needs it; the v1
substrate ships with three variants only, to prevent the content
layer complexity this ADR exists to restrain.

The compiler emits an expansion report per build listing per-arena
participant counts, per-coupling fanout, total registration count,
total OrderBand depth used, and any rejected-risk diagnostics.

### Constitutional summary (four rules, land in `design_v7.md §2`)

```
Capability is universal       — any SimThing CAN participate.
Participation is explicit     — admission requires a selector.
Expansion is bounded          — every arena declares hard caps.
Unsafe content is rejected    — at import / session build, not at runtime.
```

---

## Consequences

### Positive

- **D-1 architecturally dissolved for continuous flow.** The workshop's
  0.14× CPU regime (16 pools × 100k requesters at tick time) cannot
  arise: there is no shared pool slot being written at tick time.
  Reduction is parallel within OrderBands; allocation writes to
  independent participant columns via the existing
  `atomic_add_single_writer_f32_at` (single-writer-per-band guarantee).
- **No new GPU primitive.** Every arena compiles down to existing
  `CombineFn`, `GateSpec`, `ConsumeMode` variants. The kernel is
  unchanged. The Resource Flow Substrate is a registration discipline,
  not a primitive addition.
- **Balance is the carryforward ledger.** Leaf residuals,
  per-allocator-level rounding residuals, and surplus from
  zero-demand cases all integrate into Balance via the existing
  `governed_by` machinery. No separate per-arena budget state exists
  in the runtime. Players and AI observe Balance directly through the
  existing reduction/threshold/observability machinery.
- **Policy stays in spec.** Policies, interdiction, player intent, and
  AI policy compose through the existing overlay primitive. No new
  policy enum proliferates in the kernel or in ArenaSpec. The substrate
  is minimal; designer expressiveness lives entirely in overlays + EML.
- **Hierarchical fanout absorption.** A faction with 100k participants
  becomes `factions(1) → planets(100) → districts(1000) →
  factories(100000)`. Each level handles ≤100 children, so per-level
  contention is bounded regardless of total participant count.
- **`simthing-sim` stays arena-ignorant.** `ArenaRegistry` lives in
  `simthing-driver`. Only flat `AccumulatorOp` registrations cross to
  `simthing-sim`. The existing invariant *simthing-sim never knows
  recipe semantics* is preserved.

### Negative / caveats

- **Allocation precision is O(ε × n) per level, not exact.** The
  conservation invariant is approximate-deterministic, not strict
  equality. This is documented and intentional, but it means continuous
  flow paths cannot be used for any consumer that requires exact
  per-tick equality (e.g. hard-currency transactions). Discrete
  source-debit transfers (E-2) remain available for that case.
- **The allocation pass is a real new GPU capability.** Although it
  uses the existing AccumulatorOp kernel, it is structured as a
  reverse-direction OrderBand sweep with per-intermediate weight
  reductions and per-child share computations. E-11 must include
  parity tests against a CPU oracle and stability tests under
  hierarchical fanout.
- **ArenaRegistry refresh on fission cascade can be expensive.** E-9
  must implement incremental refresh — re-evaluate admission selectors
  only for the changed subtree, not the whole tree. Naive global
  refresh on every fission would create a boundary-time bloat vector.
  The expansion report must update incrementally to match.
- **The CPU-side 4-input cap in `AccumulatorOp::validate` is a known
  blocker.** The GPU input-list table (C-8c, binding 10) supports
  arbitrary N. The CPU cap is a one-line change; E-3 lifts it.
- **`accumulator_spec` is planned in `design_v7.md §3` but not yet
  landed.** E-8 lands the field. E-7 (governed_by generalization) and
  E-8 are prerequisites for E-9 (ArenaRegistry).

### Hot-pool contention policy (D-1 disposition)

The original D-1 design scope (GPU allocator for hot-pool contention)
is **rescoped**:

- **D-1 (rescoped):** A short discrete-transaction contention analysis
  memo. The memo evaluates whether discrete transactions
  (construction commits, treaty payments, emergency spend) reach
  contention scales that justify a GPU allocator. Likely outcome:
  CPU-side priority queue with `SubtractFromSource` ops at boundary
  time is sufficient at realistic scales (O(10²) discrete decisions per
  faction per boundary, vs the workshop's O(10⁵)).
- **D-2 (deferred indefinitely):** Hot-pool allocator v2
  implementation. Defer until a discrete-transaction workload
  demonstrates the need.
- **D-3 (unchanged):** Compact logs + replay. Independent of allocator
  question.
- **D-4 (unchanged):** Cross-pool queue contention gate. Expected to
  pass trivially because per-tick pool contention is gone.

---

## Substrate shape (summary; full schemas in `resource_flow_adr_shaping.md` §5)

### `SubFieldSpec.accumulator_spec` (E-8 lands)

```rust
pub struct SubFieldSpec {
    // ... existing fields unchanged ...
    pub accumulator_spec: Option<AccumulatorSpec>,
}

pub enum AccumulatorRole {
    IntrinsicFlow,
    AllocatedFlow { arena: ArenaName },
    Balance(BalanceSpec),
    AllocatorWeight { arena: ArenaName },
}
```

`AccumulatorRole` is **compile-time spec metadata**, not a runtime
participant taxonomy. By the time `AccumulatorOp` registrations reach
`simthing-sim`, the role has compiled away into specific
combine/gate/consume choices. It must not become runtime semantic
branching.

`Flow` and `Balance` remain `SubFieldRole::Named("flow")` and
`SubFieldRole::Named("balance")`. The role enum does not grow.

### `ArenaRegistry` (E-9 lands; driver-session artifact)

```rust
pub struct ArenaRegistry {
    pub arenas:       Vec<GpuArenaDescriptor>,
    pub participants: Vec<(ArenaIdx, SlotId)>,
    pub couplings:    Vec<ArenaCoupling>,
    pub generation:   u64,
}

pub enum CouplingDelay {
    Algebraic,
    OneTickDelay,
    BoundaryStage { stage: u32 },
    AccumulatorState { property: SimPropertyId },
}

pub enum FissionPolicy {
    Inherit,
    Reevaluate,    // default
    Reject,
    // Custom deferred to follow-on ADR
}
```

Lives in `simthing-driver`. Compiled by `simthing-spec` at session
open. Refreshed at boundary structural mutations via incremental
subtree-scoped selector re-evaluation. The driver compiles registry →
flat `AccumulatorOp` registrations through existing
`WorldGpuState::sync_accumulator_*_session` paths.

### Hierarchical allocation kernel pattern (E-11 lands)

Per intermediate participant, the allocation pass produces two
AccumulatorOps:

```
1. Weight-sum reduction (upward sweep, alongside intrinsic Flow):
   source:  SlotRange { children }
   combine: Sum
   gate:    OrderBand(reduction_band)
   consume: ResetTarget
   target:  intermediate.weight_sum

2. Per-child disbursement (downward sweep):
   source:  SlotValue { intermediate, budget_col }
   combine: EvalEML { child_share_formula }
              where formula = select(weight_sum > 0,
                                      budget * child_weight / weight_sum,
                                      0)
   gate:    OrderBand(allocation_band)
   consume: AddToTarget
   target:  child.allocated_flow_col
```

`child_share_formula` is a fixed EML tree (well within the 32-node
`ExactDeterministic` class limit). One tree per arena, not per
intermediate. The EML `SELECT` op handles the `weight_sum == 0`
case without kernel modification.

OrderBand budget per arena: `2 × tree_depth` (reduction depth +
allocation depth).

---

## Invariants (additions land in `docs/invariants.md`)

| Rule | Enforced by |
|---|---|
| Arena participation is explicit | `simthing-spec` rejects implicit/wildcard admission without declared upper bound at session build; property possession alone never admits to an arena |
| Arena caps are declared and enforced | Every `GpuArenaDescriptor` carries `max_participants`, `max_coupling_fanout`, `max_orderband_depth`; spec compiler fails the build if computed expansion exceeds any declared cap |
| Coupling cycles must contain a delay-bearing edge | Spec compiler walks the coupling graph; any cycle whose edges are all `CouplingDelay::Algebraic` fails the build |
| Hierarchical conservation is approximate-deterministic | For every intermediate allocator, `|Σ disbursed − budget| ≤ O(ε × n_children)`. The residual integrates into the parent's `Balance` via existing `governed_by`. Error is deterministic; replay is bit-exact |
| Balance is the sole carryforward ledger for resource flow | Leaf residual, rounding residual, and zero-weight surplus all integrate into `Balance` via existing `governed_by` machinery; no separate per-arena budget state may exist in the runtime |
| Allocation policy is expressed through overlays, not policy enums | The allocator kernel reads weight columns; weight columns are default Demand-proportional and overlay-modifiable via existing Add/Multiply/Set OrderBands. No new policy enum is added to ArenaSpec or kernel |
| `simthing-sim` never sees `ArenaRegistry` | The driver compiles registry → flat `AccumulatorOp` registrations before upload; `simthing-sim` sees only `AccumulatorOp` structs and remains arena-ignorant |
| Fission inheritance is declared per arena | Each arena declares its `FissionPolicy` from `{Inherit, Reevaluate, Reject}` (default `Reevaluate`); the boundary protocol applies the policy at fission time via incremental subtree-scoped re-evaluation |
| `AccumulatorRole` is compile-time metadata only | Roles compile away into combine/gate/consume choices before reaching the GPU; `simthing-sim` never branches on `AccumulatorRole` at runtime |
| ArenaRegistry refresh is subtree-incremental | Boundary structural mutations refresh only the affected subtree's selector evaluations, not the global registry; expansion report updates correspondingly |

### Existing invariant revision

| Old text | New text |
|---|---|
| `SubtractFromSource is the only transfer mechanism` | `SubtractFromSource is the transfer mechanism for source-debit transactions (discrete transfers, recipe consumption via SubtractFromAllInputs); allocator disbursements use AddToTarget on independent target slots with approximate-deterministic conservation per resource_flow_substrate ADR; no two-overlay transfers anywhere` |

---

## Production plan delta (lands in `accumulator_op_v2_production_plan.md`)

Phase E grows from six PRs to eleven. Phase D rescopes.

| PR | Status | Notes |
|---|---|---|
| **D-1 RESCOPED** | Memo only | Discrete-transaction contention analysis. No GPU allocator design. |
| **D-2 DEFERRED** | Indefinite | Pending discrete-transaction workload demonstrating need. |
| **D-3** | UNCHANGED | Compact logs + replay. Independent. |
| **D-4** | UNCHANGED | Cross-pool gate. Expected to pass trivially. |
| **E-1** | UNCHANGED | Debt-band `emit_on_threshold` builder. |
| **E-2** | SPLIT | `resource_transfer_discrete` (source-debit) + `resource_flow_participant` (continuous enrollment). |
| **E-3** | DO SOON | Lift CPU-side 4-input cap in `AccumulatorOp::validate`. One-line + test. |
| **E-4** | RETHINK | RON arena admission with caps, fission policy, coupling graph. |
| **E-5** | UNCHANGED | Compact log integration. |
| **E-6** | FOLLOWS | Doc update; mostly covered by this ADR + v7.5 bump. |
| **E-7** | NEW | `governed_by` generalization: C-7 `IntegrateWithClamp` planner from `(Amount, Velocity)` to arbitrary `(Named, Named)` pairs. Kernel unchanged. |
| **E-8** | NEW | **Done** — `accumulator_spec: Option<AccumulatorSpec>` on `SubFieldSpec`. |
| **E-9** | NEW | `ArenaRegistry` in `simthing-driver` with incremental boundary refresh. |
| **E-10** | NEW | `simthing-spec` admission framework: caps, fission policy, cycle-with-delay check, expansion report. |
| **E-11** | NEW | Hierarchical allocation kernel pattern + CPU oracle parity + stability tests under hierarchical fanout. |

PR sequencing: **E-7 and E-8 are prerequisites for E-9. E-9 is a
prerequisite for E-10 and E-11.** E-1, E-3, E-5 independent. E-2 split
can land independently. Do not execute E-11 before E-9 is stable.

---

## Out of scope

This ADR explicitly does NOT commit to:

- **Combat as a Flow arena.** Combat is a downstream application of
  the substrate, not a constitutional concern. Separate ADR if and
  when designed.
- **Diplomacy as a Flow arena.** Same disposition.
- **Multi-faction trade as a Flow arena.** Same.
- **EML classes beyond `ExactDeterministic`** for arena allocation
  formulas. Follow the existing C-8 production policy.
  `SoftDeterministic` / `FastApproximate` / `CpuOracleOnly` classes
  are unchanged and admitted by explicit per-PR opt-in only.
- **Cross-arena coupling at scale (>1000 couplings).** Per-arena caps
  bound expansion; a separate performance gate may be needed at
  large coupling counts. Not specified here.
- **Allocator policy beyond Demand-proportional default + overlays.**
  Custom policies are expressed through overlay stacks, not through
  new enum variants. If a future need requires non-overlay policy
  expression, separate ADR.
- **`FissionPolicy::Custom`.** Deferred to follow-on ADR if needed.

---

## References

- `docs/adr_accumulator_op_v2.md` — the underlying substrate
- `docs/design_v7.md` (bumped to v7.5 by this ADR's landing) — v7
  architecture spec, now incorporating the Resource Flow constitutional
  clause
- `docs/invariants.md` — extended by this ADR's invariant additions
- `docs/workshop/resource_flow_adr_shaping.md` — full implementation
  detail, kernel patterns, schemas; the design rationale companion
- `docs/workshop/resource_flow_feasibility_opus.md` — source-grounded
  feasibility verification
- `docs/accumulator_op_v2_production_plan.md` — the PR ladder
  (updated by this ADR for E-7 through E-11 and D-1 rescope)
- `crates/simthing-core/src/accumulator_op.rs` — primitive types
- `crates/simthing-core/src/property.rs` — `SubFieldSpec` (E-8 lands
  `accumulator_spec` here)
- `crates/simthing-gpu/src/shaders/accumulator_op.wgsl` — kernel
- `crates/simthing-gpu/src/transfer_accumulator.rs` — C-8c transfer
  substrate (continuous-flow extension lands in E-11 alongside this)
- `crates/simthing-gpu/src/reduction_orderband.rs` — C-5/C-6 reduction
  OrderBand pattern that the allocation sweep mirrors in reverse
