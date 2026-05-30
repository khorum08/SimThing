# SimThing — Design v7.8 Production Track (PR Ladders)

> **Status:** **Active production track (opened 2026-05-30).** This file is the **PR-ladder home**
> for the v7.8 bounded-posture expansion. [`design_v7_8.md`](design_v7_8.md) is the **constitution**:
> it holds the operating doctrine (§2) and the *current parked state* of the three capability lines
> (A — nested Resource Flow / E-11B, B — discrete hard-currency ordering / D-2·D-2a, C — atlas /
> M-4·M-4A). This file holds the **sequenced PR ladders** — landed, active, and future — that carry
> v7.8 forward. The constitution authorizes; this track sequences.
>
> **Binding rules unchanged.** Every ladder step below obeys `design_v7_8.md` §2 (operating
> doctrine), `invariants.md`, and `design_v7_7.md` §5 (Tier-1/Tier-2 gating). Promotion of a line
> from *parked* to *in-progress* still requires (a) its named scenario, (b) its recorded gate passing,
> and (c) design-authority + product acceptance. **This file tracks ladders; it authorizes nothing on
> its own.**
>
> **Companions:** [`design_v7_8.md`](design_v7_8.md) (constitution) ·
> [`design_v7_7.md`](design_v7_7.md) (CLOSED baseline) ·
> [`accumulator_op_v2_production_plan.md`](accumulator_op_v2_production_plan.md) (CLOSED) ·
> [`workshop/sead_self_ai_track.md`](workshop/sead_self_ai_track.md) (SEAD/self-AI charter §10–§11) ·
> [`workshop/mapping_current_guidance.md`](workshop/mapping_current_guidance.md) (status table) ·
> [`worklog.md`](worklog.md) (append-only history).

---

## 1. How this track works

A **ladder** is an ordered series of small, reviewable PRs that drive one capability from its named
scenario to acceptance. Each ladder step is:

- **One PR + one test report (`docs/tests/…_results.md`) + one status-table row** for Tier-1 fast-lane
  work; full design-review → acceptance → impl cadence for Tier-2 (anything touching a binding
  invariant, default wiring, new architecture, or a prohibition-list item). Any change to
  `invariants.md` is Tier-2.
- **Replay-fingerprint-pinned** where it lands a deterministic fixture (record the 16-hex combined
  fingerprint in this file's ladder table and in the PR report).
- **Honestly classified** — `GpuVerified` / `FixtureCandidate` / `FixtureOnly` / `ReplayAccepted` /
  `OwnColumnShadowWrite` / `BoundaryRequestShadowWrite` / `NotImplemented` / `Pending`. Implementer
  fixtures may report status but **may not declare a phase closed** (closure is design-authority +
  product only).

**Append-only.** New ladder steps and new ladders are appended; landed steps are not rewritten.
Per-step narrative lives in `worklog.md`; this file keeps the compact ladder tables.

## 2. Ladder index

| Ladder | Capability | Lane | Status | Gate to advance |
|---|---|---|---|---|
| **L0 — Frontier consumer** | Bounded multi-tick closed-loop self-AI consumer proof | T2 | **landed + ACCEPTED** (V1-5 → V2-0..4) | none — complete at fixture/test-support level; **no FrontierV2-5** |
| **L1 — simthing-spec buildout** | Designer-facing spec admission substrate (prep for ClauseThing) | T2 | **next / active gate** | product start + design-authority acceptance |
| **L2 — CLAUSE-SPEC** | Designer-authored FrontierV2 scenario admitted through `simthing-spec` → same accepted runtime artifacts | T2 | **parked (downstream of L1)** | L1 landed; `CLAUSE-SPEC-0` accepted |
| **L3 — ClauseThing** | ClauseScript-facing authoring front-end | T2 | **parked (separate track)** | L2 landed; explicit ClauseThing authorization |
| **A — Nested Resource Flow** | E-11B / E-11B-5 hierarchical allocation (depth > 2) | T2 | **parked** (flat-star is the posture) | named economy needing depth > 2 fanout |
| **B — Discrete hard-currency ordering** | D-2 / D-2a sequential cross-band ordering | T2 | **parked** (discrete AccumulatorOp path stands) | named multi-transaction hard-currency workload |
| **C — Atlas / multi-theater mapping** | M-4 / M-4A atlas batching | T2 | **provisional/unimplemented** (isolation ratified) | named multi-theater scenario + VRAM budget + §11-gate PR |

---

## 3. L0 — Frontier consumer ladder (landed + ACCEPTED)

The first bounded vertical: a default-off, opt-in self-AI consumer that runs the live GPU route
(mapping + EML → PIPE-0 → ACT-2 → ResourceFlowAllocator) and proves multi-tick closed-loop feedback
at **fixture/test-support level only**. Movement and structural outputs are **fixture-only shadows**,
never production state or commitments. **Design authority (Opus, 2026-05-30) ACCEPTED V1-5 → V2-0..4
and declared the consumer proof complete — there is no FrontierV2-5** (a further fixture step would be
a hygiene loop). Next gate is **L1/L2 (designer/spec admission)**, not another fixture.

| Step | What it proved | Class | Replay fingerprint | PR | Report |
|---|---|---|---|---|---|
| FrontierV1-5 | Live GPU-resident single-tick score→threshold→proposal→dispatch route; fixture-only feedback candidate | GpuVerified (route) | `1653b84847be2dd2` | #341 | [`phase_m_frontier_v1_5_live_self_ai_route_results.md`](tests/phase_m_frontier_v1_5_live_self_ai_route_results.md) |
| FrontierV2-0 | First two-tick closed-loop consumer; tick-0 feedback → tick-1 field input change | GpuVerified + FixtureOnly | `0238c18ce3b559da` | #342 | [`phase_m_frontier_v2_0_closed_loop_consumer_results.md`](tests/phase_m_frontier_v2_0_closed_loop_consumer_results.md) |
| FrontierV2-1 | Movement/structural FixtureCandidate evolution across ticks (`M1≠M0`, `S1≠S0`) | FixtureCandidate | `2d6e78a06d19736a` | #343 | [`phase_m_frontier_v2_1_candidate_evolution_results.md`](tests/phase_m_frontier_v2_1_candidate_evolution_results.md) |
| FrontierV2-2 | Own-column movement feedback application; shadow `(0,0)→(2,4)` feeds next-tick placement | OwnColumnShadowWrite | `6c01851a4afdfcbf` | #344 | [`phase_m_frontier_v2_2_movement_feedback_application_results.md`](tests/phase_m_frontier_v2_2_movement_feedback_application_results.md) |
| FrontierV2-3 | Structural BoundaryRequest shadow feedback application; structural context feeds next tick | BoundaryRequestShadowWrite | `0ad0e0d7c80316ee` | #345 | [`phase_m_frontier_v2_3_structural_feedback_application_results.md`](tests/phase_m_frontier_v2_3_structural_feedback_application_results.md) |
| FrontierV2-4 | Combined movement + structural feedback loop (4 ticks); both shadows feed downstream deterministically | OwnColumn + BoundaryRequest | `dbb54b952f9face8` | #346 | [`phase_m_frontier_v2_4_combined_feedback_loop_results.md`](tests/phase_m_frontier_v2_4_combined_feedback_loop_results.md) |

**Posture preserved across L0:** resource dispatch stayed through the Resource Flow allocator;
`simthing-sim` stayed semantic-free; no default `SimSession` wiring; no semantic WGSL; no ClauseThing;
no phase closure declared. Ruling: [`workshop/sead_self_ai_track.md`](workshop/sead_self_ai_track.md)
§10–§11.

---

## 4. L1 — simthing-spec buildout (next / active gate)

- **What:** build out `simthing-spec` as the designer-facing admission substrate that prep's for
  ClauseThing — the layer where the L0 fixtures' hard-coded protections become **admission
  rejections** (cross-entity writes, production commitment emission, Resource-Flow bypass, unbounded
  fanout, `simthing-sim` semantic leakage rejected at import; runtime stays the unconditional last
  line, per `design_v7_8.md` §2.1).
- **Status:** **Next named track per product direction.** Do **not** jump ahead to ClauseScript
  parsing or production wiring.
- **Ladder (to be filled as steps land):**

| Step | Intent | Class | Fingerprint | PR | Report |
|---|---|---|---|---|---|
| L1-0 | Designer admission substrate preflight: shared diagnostics + guardrail rejection vocabulary + accepted FrontierV2 artifact target names | Done | — | — | [`phase_m_l1_0_designer_admission_substrate_results.md`](tests/phase_m_l1_0_designer_admission_substrate_results.md) |

## 5. L2 — CLAUSE-SPEC (parked, downstream of L1)

- **What:** `CLAUSE-SPEC-0` — Designer-Facing FrontierV2 Spec Admission: admit a **designer-authored**
  FrontierV2 scenario through `simthing-spec` and compile it to the **same accepted runtime artifacts**
  the L0 fixtures exercised (RON-first; ClauseScript/ClauseThing later). The fixture guardrails
  **relocate to admission rejections** here.
- **Status:** **Parked — starts only after L1 lands.** Do not start before the simthing-spec buildout;
  do not implement the ClauseScript parser (that is L3, a separate track).
- **Ladder (to be filled):**

| Step | Intent | Class | Fingerprint | PR | Report |
|---|---|---|---|---|---|
| CLAUSE-SPEC-0 | Admit designer-authored FrontierV2 scenario → accepted runtime artifacts | Pending | — | — | — |

## 6. L3 — ClauseThing (parked, separate track)

- **What:** the ClauseScript-facing authoring front-end. The natural designer surface for the Frontier
  scenario; guardrails sit at the designer/spec-admission layer.
- **Status:** **Parked / NotImplemented.** Proposal-only until L2 lands and ClauseThing is explicitly
  authorized. Do not implement the ClauseScript parser speculatively.
- **Ladder (to be filled):**

| Step | Intent | Class | Fingerprint | PR | Report |
|---|---|---|---|---|---|
| L3-0 | _(reserved — first ClauseThing-facing slice, post-authorization)_ | Pending | — | — | — |

---

## 7. Line A — Nested Resource Flow ladder (E-11B / E-11B-5) — parked

- **Current state:** see [`design_v7_8.md`](design_v7_8.md) §3. `FlatStarResourceFlow` is the accepted
  posture; `PipelineFlags::default().use_accumulator_resource_flow` stays `false`. Readiness landed.
- **Unblocking named scenario:** an economy needing depth > 2 hierarchical fanout
  (`factions(1) → planets(100) → districts(1000) → factories(100000)`).
- **Ladder (to be filled when the scenario is named and accepted):**

| Step | Intent | Class | Fingerprint | PR | Report |
|---|---|---|---|---|---|
| A-0 | _(reserved — first nested-arena slice, post-named-scenario)_ | Pending | — | — | — |

## 8. Line B — Discrete hard-currency ordering ladder (D-2 / D-2a) — parked

- **Current state:** see [`design_v7_8.md`](design_v7_8.md) §4. Discrete AccumulatorOp transfer/recipe/
  emission (Phase T, accepted) stands; D-2 deferred indefinitely, D-2a defer-until-needed.
- **Unblocking named scenario:** a multi-transaction hard-currency workload needing sequential
  cross-band ordering at a scale the discrete path cannot meet; a narrow driver-only ladder
  (D-2a review) is the first step *if* approved.
- **Ladder (to be filled when the scenario is named and accepted):**

| Step | Intent | Class | Fingerprint | PR | Report |
|---|---|---|---|---|---|
| B-0 | _(reserved — first narrow driver-only D-2a slice, post-named-scenario)_ | Pending | — | — | — |

## 9. Line C — Atlas / multi-theater mapping ladder (M-4 / M-4A) — provisional

- **Current state:** see [`design_v7_8.md`](design_v7_8.md) §5. `request_atlas_batching` stays rejected
  at admission; `MappingExecutionProfile` default stays `Disabled`. Isolation policy ratified
  (algebraic tile-local mask `G=0` preferred at 1.0× VRAM; physical gutter `G≥H` fallback at 6.76×).
- **Unblocking named scenario:** a named multi-theater scenario **+** approved VRAM budget **+** a
  §11-gate-passing M-4 implementation PR (full-tile protocol-oracle parity). All three required.
- **Not in this ladder:** active-mask halo (**M-6A**) and source identity (**M-5 / `source_mask`**)
  stay deferred in place under the Mapping ADR.
- **Ladder (to be filled when all three gate conditions are met):**

| Step | Intent | Class | Fingerprint | PR | Report |
|---|---|---|---|---|---|
| C-0 | _(reserved — first §11-gate M-4 slice, post-named-scenario + VRAM budget)_ | Pending | — | — | — |

---

## 10. Sequencing summary

L0 is **done and accepted**. The live forward edge is **L1 (simthing-spec buildout)**, then
**L2 (CLAUSE-SPEC)**, then **L3 (ClauseThing)** — that chain is the expected source of the **named
scenarios** that unblock Lines A/B/C. Do not start any line speculatively or to escape a
hygiene/closure loop; start a step only when its gate passes. The AccumulatorOp v2 production plan
stays **CLOSED**; v7.7 stays the **binding baseline**; the three lines live in v7.8.
