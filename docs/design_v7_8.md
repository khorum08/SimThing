# SimThing — Design v7.8 Production Track (Bounded-Posture Expansion)

> **Status:** **Open track, fully parked (2026-05-30).** v7.8 is the production track that
> carries forward the **deferred capability lines** promoted out of the now-**CLOSED**
> AccumulatorOp v2 production plan. Each line is **default-off, unimplemented, and gated behind a
> named product scenario** — v7.8 *tracks* them; it authorizes no implementation by itself.
> **Opened by:** project owner (product direction, 2026-05-30); design authority Opus 4.8.
> **Relationship to v7.7:** v7.7 is CLOSED at the **bounded posture** — single-theater mapping,
> `FlatStarResourceFlow`, single-tick discrete economy. v7.8 is where that posture **expands**;
> it inherits the v7.7 constitutional baseline and **relaxes nothing** until a line's named
> scenario lands.
>
> **Companions:** [`design_v7_7.md`](design_v7_7.md) (CLOSED constitution amendment) ·
> [`accumulator_op_v2_production_plan.md`](accumulator_op_v2_production_plan.md) (CLOSED — these
> lines were promoted out of it) · [`adr/resource_flow_substrate.md`](adr/resource_flow_substrate.md) ·
> [`adr/mapping_sparse_regioncell.md`](adr/mapping_sparse_regioncell.md) · [`invariants.md`](invariants.md) ·
> [`workshop/sead_self_ai_track.md`](workshop/sead_self_ai_track.md).

---

## 1. What v7.8 is

When the AccumulatorOp v2 production plan closed, three capability lines were **deferred behind
named scenarios** rather than built — the deliberate bounded posture of v7.7. They are real,
readiness-reviewed, and architecturally decided, but they each await a product scenario that
*needs* them. Leaving them buried as "deferred" rows in a closed plan loses them. **v7.8 is their
forward home:** a production track that holds the three lines, their landed readiness evidence,
and the exact named-scenario gate that unblocks each.

v7.8 does **not** reopen the accumulator plan and does **not** authorize implementation. A line
moves from "parked" to "in progress" only when (a) its named scenario exists, (b) it passes its
recorded gate, and (c) design-authority + product accept the start — the standard Tier-2 cadence.

The expected source of those named scenarios is the **ClauseThing / simthing-spec** direction:
as designers author richer scenarios (multi-theater maps, deep economic hierarchies, hard-currency
ordering), they will *name the need* that unblocks the matching v7.8 line. Until then, v7.8 stays
parked and the bounded posture holds.

## 2. Line A — Nested Resource Flow (promotes **E-11B / E-11B-5**)

- **What:** hierarchical allocation deeper than the accepted flat-star `depth-2`
  (`faction → district`): true nested arenas (`faction → planet → district → factory`), with
  dynamic enrollment on fission cascades. The reverse-OrderBand allocation sweep and the
  approximate-deterministic conservation contract are decided in
  [`adr/resource_flow_substrate.md`](adr/resource_flow_substrate.md).
- **Status:** **Deferred / parked.** `FlatStarResourceFlow` is the accepted bounded posture;
  `PipelineFlags::default().use_accumulator_resource_flow` stays `false`.
- **Readiness already landed:** [`reviews/e11b_nested_hierarchy_gpu_readiness_review.md`](reviews/e11b_nested_hierarchy_gpu_readiness_review.md),
  [`reviews/e11b_nested_dynamic_enrollment_readiness.md`](reviews/e11b_nested_dynamic_enrollment_readiness.md),
  [`workshop/e11_hierarchical_allocation_design.md`](workshop/e11_hierarchical_allocation_design.md),
  [`workshop/e11_readiness_review.md`](workshop/e11_readiness_review.md),
  [`workshop/e11_implementation_handoff.md`](workshop/e11_implementation_handoff.md).
- **Named-scenario gate to unblock:** a scenario whose economy genuinely needs depth > 2 (a
  faction with O(10⁵) participants requiring the `factions(1) → planets(100) → districts(1000) →
  factories(100000)` fanout absorption). Flat-star covers everything shallower.
- **Constraints carried:** approximate-deterministic conservation (O(ε·n)/level, residual into
  `Balance`), incremental subtree-scoped registry refresh on fission, `simthing-sim` stays
  arena-ignorant, no new `AccumulatorRole`, no CPU fallback.

## 3. Line B — Discrete hot-pool / hard-currency ordering (promotes **D-2 / D-2a**)

- **What:** sequential cross-band ordering for **discrete** hard-currency transactions
  (construction commits, treaty payments, emergency spend) at contention scales that the per-tick
  parallel rate-reduction substrate does not address — i.e., a GPU hot-pool allocator (D-2) and/or
  boundary transaction scheduling (D-2a).
- **Status:** **Deferred.** D-2 deferred indefinitely; D-2a recommendation is *defer implementation*
  until needed. Hard-currency transfers remain **exact discrete AccumulatorOp transfer/recipe/
  emission** (Phase T, accepted), which is sufficient at realistic discrete scales (O(10²)
  decisions/faction/boundary).
- **Readiness already landed:** [`reviews/d1_discrete_transaction_contention_memo.md`](reviews/d1_discrete_transaction_contention_memo.md),
  [`reviews/d2a_boundary_transaction_scheduling_readiness.md`](reviews/d2a_boundary_transaction_scheduling_readiness.md).
- **Named-scenario gate to unblock:** a scenario with a **multi-transaction hard-currency
  workload requiring sequential cross-band ordering** at a contention scale the discrete
  AccumulatorOp path cannot meet. Until such a workload is named, the discrete path stands and a
  narrow driver-only ladder (documented in the D-2a review) is the first step *if* approved.
- **Constraints carried:** Resource Flow stays separate from hard-currency discrete transfer; no
  global Resource-Flow default-on; no new `AccumulatorRole`; no CPU production fallback;
  `simthing-sim` stays spec-free.

## 4. Line C — Atlas / multi-theater mapping (promotes **M-4 / M-4A**)

- **What:** atlas batching — packing many scheduled RegionCell tiles into one atlas with one
  dispatch — to scale mapping beyond the accepted **single ≤32×32 theater** to multi-theater.
  The isolation policy is **ratified** ([`reviews/m4_m4a_first_slice_oversight_opus_review.md`](reviews/m4_m4a_first_slice_oversight_opus_review.md)):
  algebraic tile-local mask `G=0` is the preferred isolation candidate (1.0× VRAM), physical
  gutter `G≥H` the fallback (6.76× VRAM, mandatory VRAM-multiplier reporting).
- **Status:** **Provisional / unimplemented.** `request_atlas_batching` stays **rejected at
  admission**; `MappingExecutionProfile` default stays `Disabled`.
- **Readiness already landed:** [`workshop/mapping_atlas_batching_isolation_design_note.md`](workshop/mapping_atlas_batching_isolation_design_note.md)
  (§11 binding acceptance gate), [`tests/mapping_atlas_algebraic_mask_sandbox_test_results.md`](tests/mapping_atlas_algebraic_mask_sandbox_test_results.md),
  [`tests/phase_m_m4a_atlas_readiness_gate_results.md`](tests/phase_m_m4a_atlas_readiness_gate_results.md),
  [`reviews/m4_m4a_first_slice_oversight_opus_review.md`](reviews/m4_m4a_first_slice_oversight_opus_review.md).
- **Named-scenario gate to unblock:** a **named multi-theater scenario** *and* an approved VRAM
  budget *and* a §11-gate-passing M-4 implementation PR (full-tile protocol-oracle parity, not
  corridor agreement alone). All three required; single-grid scenarios never need atlas.
- **Constraints carried:** `ActiveOnlyExperimentalNoHalo` is **never** production-authorized
  (halo mandatory); atlas never without explicit isolation policy **and** VRAM-multiplier
  reporting; no semantic/map-specific WGSL; `simthing-sim` stays map-free.
- **Not promoted (stay deferred in place):** active-mask halo (**M-6A**) and behavioral source
  identity (**M-5 / `source_mask`**) are *not* part of v7.8 — they remain separately deferred
  under the Mapping ADR until their own named needs arise.

## 5. Constitutional posture (inherited from v7.7, unchanged)

v7.8 relaxes nothing on its own. Every line, when it eventually starts, stays inside the v7.7
baseline: opt-in / default-off; `simthing-sim` semantic-/arena-/map-free; no semantic WGSL; no new
`AccumulatorRole`; CPU-oracle bit-exact parity where exact; and — per the v7.7 doctrine —
**guardrails placed at the designer/spec-admission layer** (unsafe authoring rejected at import,
runtime as the unconditional last line). Promotion to v7.8 changes a line's *home and visibility*,
not its authorization: each remains Tier-2 and gated on its named scenario + acceptance.

## 6. Sequencing

v7.8 runs **parallel to and mostly downstream of** the simthing-spec / ClauseThing direction
(`sead_self_ai_track.md` §11 → simthing-spec buildout → ClauseThing-facing authoring). The
ClauseThing scenario corpus is the most likely source of the named scenarios that unblock Lines
A–C. Do **not** start any v7.8 line speculatively or to escape a hygiene/closure loop; start a
line only when its named scenario exists and its gate passes. The AccumulatorOp v2 production plan
stays **CLOSED** — these three lines live here now.

## 7. Tracking table

| Line | Promoted from | Status | Unblocking named scenario |
|---|---|---|---|
| A — Nested Resource Flow | E-11B / E-11B-5 | parked (flat-star is the posture) | economy needing depth > 2 hierarchical fanout |
| B — Discrete hard-currency ordering | D-2 / D-2a | parked (discrete AccumulatorOp path stands) | multi-transaction hard-currency workload needing sequential cross-band ordering |
| C — Atlas / multi-theater mapping | M-4 / M-4A | provisional/unimplemented (isolation policy ratified) | named multi-theater scenario + approved VRAM budget + §11-gate PR |
