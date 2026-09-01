# RESIDENT-CLEARING-SCORE-AND-BANDS-0 results

> **Status: PROBATION / archaeology-first / ordering-audit-complete /
> implementation-pending / DA-review-pending / UNMERGED.** Coding lane only;
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

## Evidence/source split at first step

Evidence added in this first step:

- this ordering audit;
- doctrine reach rows emitted by first-hand anchor queries.

Executable source delta at this checkpoint: **none**.

