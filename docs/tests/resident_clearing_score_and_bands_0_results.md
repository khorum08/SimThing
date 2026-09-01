# RESIDENT-CLEARING-SCORE-AND-BANDS-0 results

> **Status: PROBATION / archaeology-first / ordering-audit-complete /
> proof-present / DA-review-pending / UNMERGED.** Coding lane only;
> no merge, graduation, pointer movement, closeout apply, apportionment, cutover,
> or 14.4+ work.

**Date:** 2026-09-01
**Owner handoff:** Board issue `#1332`, comment `5494489777`
**Exact handoff base:** `8b5740978186d75b080b65e32b77b3d84eef3520`
**Prior graduated rung:** `RESIDENT-CLEARING-PLAN-0`, PR `#1911`,
`217e8fca`
**ORIENT-RECEIPT (first-hand):** `26ec91084cea`
**orientation_rule_stamp:** `0310b6f6a40140be`
**orientation_digest_sha:**
`42ba9690ed79d63da4aa524b45883f4bd28c98c7b0e24e1fa1c9d55dd35a3854`

The handoff names receipt `58e554dc9bca`; a fresh first-hand coding ingress on
the exact base emitted `26ec91084cea` with the same rule stamp and digest. This
record carries the first-hand receipt and does not manufacture an HD receipt.

## Required anchor acknowledgements

- `founding-ontology-invariants@46802793fba7`
- `intrinsic-constrained-clearing@957b7c81b756`
- `rf-market-candidate-laws@ef8d37c169b7`
- `rf-market-falsifiers@30c747c48e9a`
- `rf-market-mirror-cycle@13231438befe`
- `rf-market-port-census@6dc8eb45dd49`
- `rf-market-receive-not-recompute@2356c9e3f477`
- `rf-market-settled-code-census@959d022781da`

## Mandatory first-step ordering-law completeness audit

This audit was completed before any executable source edit on this branch. It
separates execution scheduling from economic precedence, continuous allocation,
and the later exact residue. The question is whether a successful clearing or
continuous allocator result can change because a container, physical row,
dispatch, workgroup, or atomic arrival happened to present values in another
order.

| Ordering surface | Written authority | Settled implementation | Classification |
|---|---|---|---|
| RF execution-stage order | `ArenaBandLayout` names reset, leaf-to-root upsweep, top-down broadcast/disburse, and integration bands; `OrderBand` admits only the named band. Generation pacing forbids same-generation recursive convergence. | `arena_hierarchy.rs::ArenaBandLayout`; `arena_allocation_plan.rs::plan_arena_allocation` | **WRITTEN EXECUTION LAW.** Stage order establishes data readiness only; it is not claimant precedence. |
| Scope iteration | A full `OwnerChannelScopeKey` is the canonical segregated RF bucket identity. Structural aggregation is by that key, never by segment/upload position. | `channel_key.rs::OwnerChannelScopeKey` derives `Ord`; supplies, claims, and results use `BTreeMap`. | **WRITTEN STRUCTURAL ORDER.** Canonical key order determines result order; physical scope-segment storage does not. |
| Hard economic precedence | Higher admitted EML score clears first. Equal score bits form one band. Within a band, claimant logical identity is the deterministic secondary order. Signed zero is canonicalized before bit comparison. | `AuthoredClearingProgram::score`; `f32::total_cmp` descending then `SimThingId`; equality by `to_bits`. | **WRITTEN ECONOMIC LAW.** Stable-sort behavior is irrelevant because duplicate `(scope, source)` claims are refused and the comparator is total for admitted rows. Continuous pressure values are not promoted into this exact precedence. |
| Continuous pressure-informed share | Direct-child pressure is additive; parent intrinsic plus parent `AllocatedFlow` is shared continuously in proportion to the existing child `AllocatorWeight` over the parent's direct-child weight sum. | `sum_reduction_ops`; `child_share_formula`; `disburse_op`. | **WRITTEN CONTINUOUS LAW.** Logical direct-child inputs decide the share. The outstanding 14.3 binding is removal of the materialized propagated copy, not invention of a score layer. |
| Exact fractional remainder comparison | Largest remainder orders exact integer numerators modulo the exact band request total, descending, then claimant identity. | `u64` numerator/base/remainder construction and `remainder_order.sort_by`. | **WRITTEN RESIDUE LAW, CPU ORACLE ONLY IN 14.3.** Resident integer apportionment is fenced to 14.4. |
| Exact-tie rotation indexing | Equal exact remainders rotate by `(granter logical id + granter generation) mod tie length`. | `ClearingRemainderAuthority`; `rotate_left`. | **WRITTEN RESIDUE LAW, CPU ORACLE ONLY IN 14.3.** Neither upload nor atomic append ordinal participates. |
| Grant output order | Per-scope grants are returned by claimant logical identity after allocation. | `grants.sort_by_key(source_simthing_id)`. | **WRITTEN CANONICAL PRESENTATION.** Result vector order follows canonical scope-key order. |
| Direct-child accumulation order | Addition is over the admitted direct-child set, represented as a contiguous logical span or an explicit input list that preserves hierarchy child order. Descendants are represented once through their direct parent. | `ArenaTreeLayout`; `sum_reduction_ops`; sparse child-list tests. | **WRITTEN RECURSIVE SHAPE.** No descendant scan, max/tropical fold, or physical-row rank is admitted. Floating addition order remains the admitted logical hierarchy order and is rebuilt from logical identity after a row rebind. |
| Invalid-input detection order | 14.5 expressly owns the negative/error matrix, equivalence of typed refusals, canonical result order, and no partial writes. 14.3 executes no resident failure path and changes no CPU validation traversal. | Existing CPU oracle returns before producing a result; resident implementation is not cut over in 14.3. | **LATER BINDING, NOT A 14.3 RESULT ORDER.** No successful economic result is exposed on failure. Exact resident refusal equivalence remains fenced to 14.5 and is not claimed here. |

### Audit verdict

**PASS — zero new `MISSING — STOP` ordering findings.** Every ordering that can
affect an admitted successful CPU clearing result or the existing continuous RF
allocator has a written logical authority. Physical arrival, row address,
scope-segment storage, workgroup shape, dispatch partition, and atomic append
ordinal are absent from those authorities. The 14.3 implementation may proceed,
but it must retain a planted atomic-arrival tie resolver that REDs and a
cross-layout invariance witness before return.

This verdict does not claim resident exact apportionment or resident negative
parity: those are explicitly owned by 14.4 and 14.5 respectively.

## Reopened binding map

| Surface-reuse row | 14.3 binding |
|---|---|
| 2 — branch-attributed subtree pressure | Preserve the existing direct-child additive reduction. Each descendant contributes through exactly one direct parent. |
| 3 — neutral pressure to weight | Reuse `EvalEML` and `AccumulatorRole::AllocatorWeight`; identity binds eligible immediate-flow `F` or entitlement-first raw `P`. No private urgency or score field. |
| 4 — pressure to continuous allocation | Reuse `child_share_formula` and the existing allocator columns; the formula reads the admitted direct-child weight and parent aggregate. |
| 5 — continuous child share | Bind level-N `AllocatedFlow` directly as an input to the same `EvalEML` allocator operation that writes child level N+1 `AllocatedFlow`. Remove the propagated economic copy. |
| 8 — additive subtree pressure | Retain `CombineFn::Sum` over the admitted child span/list. No alternate ledger or fold. |
| 11 — unresolved demand recurrence | Carry `U(N)` by neutral identity exactly once into the same `RuntimeOwnerSiloDemandBucket` at N+1 through existing `GenerationStamped<T>` Current-to-Next transport. No new lane, column, demand type, or authored bridge. |

## Implemented direct resident allocation binding

The resident allocator continues to use the existing `AccumulatorOp`, packed
`INPUT_LIST`, `EvalEML`, `AllocatorWeight`, and `AllocatedFlow` vocabulary.
There is no new economic row, column, lane, role newtype, semantic adapter, or
clearing-owned score layer.

The child-share operation now binds its declared input-list rows as:

```text
PARAM(0) = parent intrinsic-flow authority
PARAM(1) = parent AllocatedFlow (the live level-N cell)
PARAM(2) = parent direct-child weight sum
SLOT_VALUE(weight_col) = target child's resident AllocatorWeight
target = child AllocatedFlow (level N+1)
```

The packed encoder admits at most four `EvalEML` parameter inputs and exactly
one target eval slot. WGSL reads those cells directly in declared logical order.
The pre-existing broadcast bands remain as stage indices for ABI/band stability,
but contain no materialization operations. The three frozen `propagated_*`
column references remain in `NodeColumnRefs` for frozen layout compatibility and
have zero read/write use in the allocator plan. Residual closure now applies the
existing constant scale to the direct child-`AllocatedFlow` sum, eliminating its
former scratch copy as well.

The live witness is a D3 chain:

```text
root intrinsic supply 8
  -> child AllocatedFlow 8
       -> direct PARAM(1) at the next recursive allocator
            -> grandchild AllocatedFlow 8
```

CPU oracle and production GPU output match by `f32::to_bits`. Compact physical
rows `[0,1,2]` and rebound sparse rows `[129,65,3]` produce the same normalized
direct-allocation `[8,8]` bits. The same live sessions then sum the child
`AllocatedFlow` with `ScaleSpec::Constant(-1)` and produce residual input `-8`
bit-exactly, without a propagated negative cell. A six-chain plan crosses the
64-invocation workgroup boundary and retains all three bits. Replacing the
direct parent cell with the old propagated column is a planted defect and
diverges.

## Row-11 ordinary unresolved-demand recurrence

`carry_unresolved_demand_to_next_generation` consumes an optional
`UnresolvedDemandObservation`, adds its `u` once to the claimant's independently
produced `d'`, and returns the **same** `RuntimeOwnerSiloDemandBucket` inside the
existing `GenerationStamped<T>` carrier at exactly N+1. It checks the observed
generation, full RF scope, source identity, and checked arithmetic. It has no
EML, CostBand, Overlay, new column, new demand type, or alternate persistence
lane. `fund_unresolved_persistence` remains optional secondary deformation.

Five-part falsifier result:

| Requirement | Witness |
|---|---|
| `d' + u` without authored path | N demand 10, supply 4 produces `u=6`; independent `d'=2` becomes the ordinary N+1 demand 8 through the neutral function whose signature has no authored program. |
| exactly once; parent once | N+1 claim `requested=8`; the explicit double-carry mutant would be 14 and REDs. |
| N unchanged; no same-generation re-clear | original demand remains 10 and grant remains `(requested=10, granted=4, unresolved=6)`; output stamp is 11; an observation/current-generation mismatch refuses. |
| later supply drains U | N+1 supply 8 grants 8 and yields `unresolved=0`; no further observation is minted. |
| `u=0` negative control | `None` produces a stamped product equal to the input `RuntimeOwnerSiloDemandBucket` in every field. |

## Physical-order invariance

The canonical clearing witness perturbs all non-authoritative schedule/layout
inputs before invoking the settled CPU oracle:

| Axis | Perturbation | Result |
|---|---|---|
| claim upload | three different arrival permutations | identical canonical snapshots |
| physical row / epoch rebind | compact and sparse/permuted row addresses | identical D3 CPU/GPU normalized bits |
| scope segment storage | segment ids permuted; supply vector reversed | identical canonical scope order |
| workgroup scheduling | simulated legal scheduling widths 16, 32, and 64 change arrival order | identical score bits, equality bands, claimant total order, grants |
| dispatch partition | 1, 3, and 4 arrival partitions plus actual one-workgroup vs multi-workgroup resident dispatch cardinality | identical canonical snapshots / D3 bits |
| atomic append tie resolver | planted first-arrival winner over an exact tie | different physical schedules choose different winners: **RED** |

Logical full scope, EML score bits, claimant identity, and granter generation are
the only clearing authorities. The actual resident `AccumulatorOp` shader has
one admitted compile-time workgroup size in this rung; 14.3 does not mint a
second shader/pipeline variant merely to add another physical choice. The
scheduling-width perturbation proves the economic oracle is independent of any
future lawful partition shape, while the actual GPU witness covers compact,
sparse-rebound, and one- versus multi-workgroup dispatch cardinalities.

## File census

| Class | Files | Purpose |
|---|---|---|
| direct allocator | `crates/simthing-driver/src/{arena_allocation_plan.rs,arena_allocation_oracle.rs,arena_allocation_sync.rs,child_share_eml.rs}` | direct parent-cell plan, CPU oracle, packed-list sync, parameterized child-share EML |
| established kernel execution | `crates/simthing-kernel/src/accumulator_op/{encode.rs,packed_session_upload.rs}`, `crates/simthing-kernel/src/{cpu_oracle.rs,shaders/accumulator_op.wgsl}` | bounded input-list parameter admission/upload; scaled sums; direct resident input reads |
| row-11 law | `crates/simthing-spec/src/spec/constrained_clearing.rs`, re-exports in `spec/mod.rs` and `lib.rs` | same-`T_d` neutral Current-to-Next recurrence |
| proof | `crates/simthing-workshop/tests/resident_clearing_score_and_bands_0.rs`, `scripts/ci/test_inventory.tsv` | D3 live GPU chain, five-part recurrence, physical-order/mutant falsifiers |
| evidence | this file, `docs/tests/current_evidence_index.md`, `scripts/ci/anchor_reach_log.tsv` | ordering audit, durable return, doctrine reach |

## Source/evidence split

Production source changes are restricted to the direct allocator binding,
existing generic packed-EvalEML execution, scaled-sum semantics already named
by `ScaleSpec`, and the same-demand recurrence. Workshop contains only the
consumer/referee. Documentation, inventory, and reach logs are evidence only.

## Test evidence

- `cargo test -p simthing-workshop --test resident_clearing_score_and_bands_0`
  — 3 passed, including live GPU on NVIDIA RTX 4080 Laptop GPU / Vulkan.
- `cargo test -p simthing-driver --test arena_participant_elimination_0`
  — 2 passed on the same GPU; existing sparse INPUT_LIST and fission/replay
  production paths remain bit-exact.
- `cargo test -p simthing-driver --test cpu_gpu_parity_matrix_0` — 2 passed,
  including all planted-defect REDs.
- `cargo test -p simthing-driver --test plan_struct_typing_0` — 4 passed.
- `cargo test -p simthing-spec --lib` — 14 passed; `cargo test -p
  simthing-driver --lib` — 17 passed; `cargo test -p simthing-kernel --lib` —
  42 passed.
- `bash scripts/ci/test_inventory_check.sh` and `bash
  scripts/ci/test_inventory_drift_check.sh` — PASS (`1373` discovered,
  missing/unledgered/stale `0`).

Exact-head clearance, relay lint, and full certification are recorded in the PR
return rather than hard-coding a self-referential head here.

## Evidence/source split at first step

Evidence added in this first step:

- this ordering audit;
- doctrine reach rows emitted by first-hand anchor queries.

Executable source delta at that checkpoint: **none**. Commit `f74fa6ad` preserves
the audit as the first branch commit before all executable edits.
