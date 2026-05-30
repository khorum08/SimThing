# SimThing — Design v7.8 Constitution (Bounded-Posture Expansion)

> **Status:** **Open track, fully parked (2026-05-30).** v7.8 is the **constitution** for the
> bounded-posture *expansion* — it carries forward the **deferred capability lines** promoted out of
> the now-**CLOSED** AccumulatorOp v2 production plan, and holds the operating doctrine and each line's
> current state. The **sequenced PR ladders** live in the companion
> [`design_v7_8_production_track.md`](design_v7_8_production_track.md). Each line is **default-off,
> unimplemented, and gated behind a named product scenario** — v7.8 *tracks* them; it authorizes no
> implementation by itself.
> **Opened by:** project owner (product direction, 2026-05-30); design authority Opus 4.8.
> **Relationship to v7.7:** v7.7 is CLOSED at the **bounded posture** — single-theater mapping,
> `FlatStarResourceFlow`, single-tick discrete economy. v7.8 is where that posture **expands**;
> it inherits the v7.7 constitutional baseline and **relaxes nothing** until a line's named
> scenario lands.
>
> **Companions:** [`design_v7_8_production_track.md`](design_v7_8_production_track.md) (**the PR-ladder
> track — this constitution authorizes; that file sequences**) ·
> [`design_v7_7.md`](design_v7_7.md) (CLOSED constitution amendment) ·
> [`accumulator_op_v2_production_plan.md`](accumulator_op_v2_production_plan.md) (CLOSED — archived stub; see [`archive/closed_production/accumulator_op_v2_production_plan.md`](archive/closed_production/accumulator_op_v2_production_plan.md)) · [`adr/resource_flow_substrate.md`](adr/resource_flow_substrate.md) ·
> [`adr/mapping_sparse_regioncell.md`](adr/mapping_sparse_regioncell.md) · [`invariants.md`](invariants.md) ·
> [`workshop/sead_self_ai_track.md`](workshop/sead_self_ai_track.md).
>
> **▶ Where the PR ladders live.** This file is the **constitution**: operating doctrine (§2) and the
> *current parked state* of each capability line (§3–§5). The **sequenced PR ladders — landed, active,
> and all future** — live in [`design_v7_8_production_track.md`](design_v7_8_production_track.md). When
> you are about to build, go there; when you need the rules and the line state, stay here.
>
> **▶ Operating doctrine — §2 (read first).** Guardrails live at the designer/spec-admission layer;
> the WGSL ban is on *semantic* WGSL only (generic non-semantic WGSL is admissible with CPU-oracle
> parity); EML gadgets/formula classes are admitted at the designer layer; Tier-1/Tier-2 gating +
> doc hygiene + anti-loop apply; and the §2.5 non-negotiables (oracle parity, `simthing-sim`
> semantic-free, opt-in/default-off, artifact-backed exact authority, no CPU planner) are untouched.
> These govern all v7.8 work **and** the simthing-spec → ClauseThing buildout.

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

## 2. Operating doctrine — read before any v7.8 / simthing-spec / ClauseThing work

These rules govern all v7.8 work **and** the downstream simthing-spec → ClauseThing buildout. They
are **surfaced here for immediate visibility**; the binding homes are `invariants.md` and
`design_v7_7.md` §5 (cited per item). The throughline: **as work moves to the designer-facing
surface, guardrails relocate to spec admission — they do not disappear, and they do not stay as
hard-coded runtime/fixture special-cases.**

### 2.1 Guardrails live at the designer/spec-admission layer (the doctrine)

**Guardrail placement is two-layered** (`invariants.md` "Guardrail placement is two-layered"): the
**RON/Designer/spec layer owns expressive policy and rejects unsafe *authoring* at import**
(unbounded fanout, cross-entity writes, production commitment emission, Resource-Flow bypass,
coupling cycles without delay, semantic leakage, out-of-class formulas); the **runtime enforces
hard safety unconditionally** (horizon caps, source-cap clamp, finite/column validation, ping-pong
correctness, bounded clamps) as the *unconditional last line*. Authoring is never trusted to have
been safe. **This is the optimal home for guardrails** — reject early at the designer surface with
good diagnostics; the runtime catches anything that slips. The CLAUSE-SPEC work makes this literal:
the Frontier fixtures' hard-coded protections become **admission rejections**.

### 2.2 The WGSL ban is on *semantic* WGSL only — generic non-semantic WGSL is admissible

There is **no blanket WGSL ban.** What is banned is **semantic WGSL**: gameplay/map/faction/AI
concepts in shader text (`invariants.md` "No semantic WGSL"). **Generic, semantic-free shader
extensions are admissible** — new parameters, operator variants, JIT-emitted straight-line kernels —
**when** (a) they carry no map/faction/AI semantics, (b) they are paired with **CPU-oracle parity**,
and (c) their meaning is **pinned entirely at the designer/spec admission layer** (the shader sees
only floats/indices). See `invariants.md` "Dense local fields use the generic primitive only" +
`workshop/m5_gradient_extraction_design_note.md` §1, and the proven precedents: the M-JIT generic
EvalEML WGSL emission and the artifact-backed exact `sqrt` (Candidate F, hash `e2e9e27601ee2e13`,
admitted through the descriptor surface). New generic WGSL is a Tier-2 gate, not a prohibition.

### 2.3 EML gadgets are admissible at the designer layer (relaxed from the legacy whitelist)

The legacy `EmlExpressionRegistry` whitelist rejection was a **wrong-layer** decision, not a runtime
safety property (`invariants.md` "Field formula-class admission is a designer-layer policy"). So:
- **Formula classes** (`field_pressure` / `field_urgency` / `field_decay` / `bounded_field_update`)
  are admitted at the **RON/Designer/spec layer** (C-8 `register_formula` is runtime-sufficient).
- **EML gadgets** compile to a postfix subgraph over the **existing `EvalEML` opcode set** — **no
  new WGSL, no per-gadget GPU kernel, no new opcode** (`invariants.md` "Gadgets are spec-layer
  node-template macros"). Tier-1 stateless (FieldSampler / WeightedAccumulator / algebraic SoftStep)
  and Tier-2 temporal (VelocityMonitor / Decay-EMA / BoundedFeedback / Hysteresis / explicit-velocity
  Acceleration) are admitted per the accepted EML-GADGET-1/2 gates.
- Admission validates at the designer layer (kinds, column bounds, finite/`>0` params, matched
  input/weight counts) and a **bounded-feedback contract** governs any recurrent gadget
  (default `0 ≤ decay < 1`, explicit clamp when feeding a hard threshold, no positive unbounded
  recurrence) — plus **stateful-sequence CPU-oracle parity**. See `workshop/eml_gadget_library_design_note.md`.

### 2.4 Gating & documentation hygiene (constitutional, `design_v7_7.md` §5)

- **Two lanes:** **Tier-1 fast lane** (within accepted design, generic substrate, opt-in/default-off,
  oracle-parity-backed, reversible) ships as **one PR + one test report + one status-table row**.
  **Tier-2 gated** (touches a binding invariant, introduces default-on/default wiring, new
  architecture, open design question, or prohibition-list item) keeps design-review → acceptance →
  impl. **Any change to `invariants.md` is Tier-2.** Closure/acceptance memos are **design-authority
  + product only** — implementer fixtures may report `ReplayAccepted`/`GpuVerified` but may not
  declare a phase closed.
- **Doc discipline:** standing posture asserted **once** per PR test report, not duplicated across
  files; active docs carry a compact status table, append-only history in `worklog.md`; collapse
  verbose blocks when touched.
- **Anti-loop stop rule:** an agent about to write a *third* meta-document for one slice, or spawn a
  consumer-less hygiene/prooflet pass, is in the ceremony loop — **ship the code / orient to the
  named consumer instead.** Operational detail: `workshop/phase_m_gating_and_doc_policy.md`.

### 2.5 Non-negotiable rigor (relaxation does **not** touch these)

The relaxations above are about *where* rules live and *what* is admissible — never about dropping
safety. These stay binding regardless:
- **CPU-oracle bit-exact parity** where a kernel claims `ExactDeterministic`; honest classification
  (`ApproximateJitOnly` / `ReplayAccepted` / `GpuVerified`) otherwise — no overclaiming.
- **`simthing-sim` stays semantic-/arena-/map-/Gadget-/Personality-free.** All semantics compile away
  at the spec/driver layer to flat `AccumulatorOp` / overlay / threshold registrations.
- **Opt-in / default-off.** No default `SimSession` wiring, scheduler, cache, or economy→mapping
  production bridge without its own named gate.
- **Exact authority is artifact-backed and proof-gated** (the `sqrt` Candidate F precedent): exactness
  is earned by exhaustive proof, hash-pinned, granted at admission — not asserted.
- **No CPU planner / CPU urgency / CPU commitment emission.** AI is a SimThing: decisions are
  GPU-resident threshold crossings.

## 3. Line A — Nested Resource Flow (promotes **E-11B / E-11B-5**)

- **What:** hierarchical allocation deeper than the accepted flat-star `depth-2`
  (`faction → district`): true nested arenas (`faction → planet → district → factory`), with
  dynamic enrollment on fission cascades. The reverse-OrderBand allocation sweep and the
  approximate-deterministic conservation contract are decided in
  [`adr/resource_flow_substrate.md`](adr/resource_flow_substrate.md).
- **Status:** **NamedScenarioAccepted (2026-05-30); A-0 QUEUED** (not opened — product priority is
  Line C / map batching first). `FlatStarResourceFlow` remains the accepted posture;
  `PipelineFlags::default().use_accumulator_resource_flow` stays `false` until A-0 opens.
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

## 4. Line B — Discrete hot-pool / hard-currency ordering (promotes **D-2 / D-2a**)

- **What:** sequential cross-band ordering for **discrete** hard-currency transactions
  (construction commits, treaty payments, emergency spend) at contention scales that the per-tick
  parallel rate-reduction substrate does not address — i.e., a GPU hot-pool allocator (D-2) and/or
  boundary transaction scheduling (D-2a).
- **Status:** **NamedScenarioAccepted (2026-05-30); B-0 QUEUED** (not opened — product priority is
  Line C / map batching first). Hard-currency transfers remain **exact discrete AccumulatorOp
  transfer/recipe/emission** (Phase T, accepted) as the posture until B-0 opens; D-2 still deferred
  indefinitely, D-2a is the narrow driver-only first slice when B-0 opens.
- **Readiness already landed:** [`reviews/d1_discrete_transaction_contention_memo.md`](reviews/d1_discrete_transaction_contention_memo.md),
  [`reviews/d2a_boundary_transaction_scheduling_readiness.md`](reviews/d2a_boundary_transaction_scheduling_readiness.md).
- **Named-scenario gate to unblock:** a scenario with a **multi-transaction hard-currency
  workload requiring sequential cross-band ordering** at a contention scale the discrete
  AccumulatorOp path cannot meet. Until such a workload is named, the discrete path stands and a
  narrow driver-only ladder (documented in the D-2a review) is the first step *if* approved.
- **Constraints carried:** Resource Flow stays separate from hard-currency discrete transfer; no
  global Resource-Flow default-on; no new `AccumulatorRole`; no CPU production fallback;
  `simthing-sim` stays spec-free.

## 5. Line C — Atlas / multi-theater mapping (promotes **M-4 / M-4A**)

- **What:** atlas batching — packing many scheduled RegionCell tiles into one atlas with one
  dispatch — to scale mapping beyond the accepted **single ≤32×32 theater** to multi-theater.
  The isolation policy is **ratified** ([`reviews/m4_m4a_first_slice_oversight_opus_review.md`](reviews/m4_m4a_first_slice_oversight_opus_review.md)):
  algebraic tile-local mask `G=0` is the preferred isolation candidate (1.0× VRAM), physical
  gutter `G≥H` the fallback (6.76× VRAM, mandatory VRAM-multiplier reporting).
- **Status:** **C-0/C-1 ACCEPTED (C-ACCEPT-0, 2026-05-30); C-2 OPEN.** C-0 (first §11-gate M-4 atlas
  slice — real packed-atlas GPU path, algebraic G=0, full-tile protocol-oracle parity,
  `GpuVerifiedApproximate`) and C-1 (2000-star budget envelope) are accepted
  ([`tests/phase_m_c_acceptance_review_results.md`](tests/phase_m_c_acceptance_review_results.md)).
  Next gate **C-2 = atlas admission relaxation (algebraic-G=0 only).** `request_atlas_batching` stays
  **rejected at admission** and `MappingExecutionProfile` default stays `Disabled` until C-2 lands its
  bounded admission scope; the atlas **production runtime / sparse-residency scheduler** (C-1's noted
  need at true 2000-star scale) is a **separate later gate**, not C-2.
- **VRAM budget (set 2026-05-30, design authority/product):** **1.5 GiB default ceiling**
  (`V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES = 1_610_612_736`), **configurable, no architectural hard
  cap** — dedicated/headless servers and larger-VRAM cards raise `max_bytes` far beyond 1.5 GiB;
  VRAM-multiplier reporting mandatory (algebraic mask G=0 ≈ 1.0×; gutter G≥H ≈ 6.76×). Typed term
  `V78AtlasVramBudget` in `simthing-spec`; atlas occupancy is checked against the *active* budget,
  never a constant.
- **Readiness already landed:** [`workshop/mapping_atlas_batching_isolation_design_note.md`](workshop/mapping_atlas_batching_isolation_design_note.md)
  (§11 binding acceptance gate), [`tests/mapping_atlas_algebraic_mask_sandbox_test_results.md`](tests/mapping_atlas_algebraic_mask_sandbox_test_results.md),
  [`tests/phase_m_m4a_atlas_readiness_gate_results.md`](tests/phase_m_m4a_atlas_readiness_gate_results.md),
  [`reviews/m4_m4a_first_slice_oversight_opus_review.md`](reviews/m4_m4a_first_slice_oversight_opus_review.md).
- **C-0 gate (landed, pending Opus review):** first §11-gate M-4 atlas slice evidence at
  fixture/test-support level — [`tests/phase_m_c0_m4_atlas_protocol_oracle_results.md`](tests/phase_m_c0_m4_atlas_protocol_oracle_results.md).
  `request_atlas_batching` stays rejected at admission until C-0 is accepted; production mapping
  runtime remains separately gated. Active-mask/source-identity remain deferred.
- **C-1 (scale model, pending Opus review):** modeled the exact 2000-star target envelope against the active `V78AtlasVramBudget`. Algebraic G=0 fits the 1.5 GiB default; physical gutter fallback requires raised budget. No production runtime or posture relaxation. See [`tests/phase_m_c1_atlas_2000_star_scale_model_results.md`](tests/phase_m_c1_atlas_2000_star_scale_model_results.md).
- **C-0 gate (originally opened 2026-05-30):** the §11-gate-passing M-4 implementation PR — full-tile protocol-oracle
  parity (not corridor agreement alone) + VRAM-multiplier report against the active budget. Named
  scenario + VRAM budget are now satisfied; the §11 implementation PR is the remaining gate.
- **Constraints carried:** `ActiveOnlyExperimentalNoHalo` is **never** production-authorized
  (halo mandatory); atlas never without explicit isolation policy **and** VRAM-multiplier
  reporting; no semantic/map-specific WGSL; `simthing-sim` stays map-free.
- **Not promoted (stay deferred in place):** active-mask halo (**M-6A**) and behavioral source
  identity (**M-5 / `source_mask`**) are *not* part of v7.8 — they remain separately deferred
  under the Mapping ADR until their own named needs arise.

## 6. Constitutional posture

The full operating doctrine is **§2** (read it first). In short: v7.8 relaxes nothing on its own —
each line stays inside the v7.7 baseline and the §2.5 non-negotiables, with guardrails at the
designer/spec-admission layer (§2.1). Promotion to v7.8 changes a line's *home and visibility*, not
its authorization: each remains Tier-2 and gated on its named scenario + acceptance.

## 7. Sequencing

v7.8 runs **parallel to and mostly downstream of** the simthing-spec / ClauseThing direction
(`sead_self_ai_track.md` §11 → simthing-spec buildout → ClauseThing-facing authoring). The
ClauseThing scenario corpus is the most likely source of the named scenarios that unblock Lines
A–C. Do **not** start any v7.8 line speculatively or to escape a hygiene/closure loop; start a
line only when its named scenario exists and its gate passes. The AccumulatorOp v2 production plan
stays **CLOSED** — these three lines live here now.

## 8. Line-state tracking (current state)

This constitution owns the **current state** of each line (below). The **PR ladders** that will move a
line forward — landed, active, and future — live in
[`design_v7_8_production_track.md`](design_v7_8_production_track.md), not here. Update the production
track when a step lands; update this table only when a line's *constitutional state* changes (parked →
in-progress → accepted), which is a Tier-2 design-authority + product action.

| Line | Promoted from | Current state | Unblocking named scenario | Ladder |
|---|---|---|---|---|
| A — Nested Resource Flow | E-11B / E-11B-5 | parked (flat-star is the posture) | economy needing depth > 2 hierarchical fanout | production track §7 |
| B — Discrete hard-currency ordering | D-2 / D-2a | parked (discrete AccumulatorOp path stands) | multi-transaction hard-currency workload needing sequential cross-band ordering | production track §8 |
| C — Atlas / multi-theater mapping | M-4 / M-4A | provisional/unimplemented (isolation policy ratified) | named multi-theater scenario + approved VRAM budget + §11-gate PR | production track §9 |

The forward edge that is expected to *name* the scenarios above is the
**simthing-spec → CLAUSE-SPEC → ClauseThing** chain, whose ladders (and the landed Frontier consumer
ladder that precedes them) are tracked in the production track file (§3–§6).

**V7.8-MET-SCENARIO-0 note (2026-05-30):** named consumer scenarios were proposed
for Lines A/B/C through the accepted CLAUSE-SPEC / simthing-spec layer: nested
Resource Flow fanout, hard-currency contention ordering, and multi-theater atlas
mapping. Constitutional state remains parked/provisional until design-authority
and product accept a line start.
