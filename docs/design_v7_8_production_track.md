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
> [`accumulator_op_v2_production_plan.md`](accumulator_op_v2_production_plan.md) (CLOSED stub — archived full plan under [`archive/closed_production/`](archive/closed_production/)) ·
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
| **L1 — simthing-spec buildout** | Designer-facing spec admission substrate (prep for ClauseThing) | T2 | **landed + ACCEPTED** (L1-0, L1-1; L1-ACCEPT-0) | none — sufficient to open L2 |
| **L2 — CLAUSE-SPEC** | Designer-authored FrontierV2 scenario admitted through `simthing-spec` → same accepted runtime artifacts | T2 | **ACCEPTED (Opus design authority, 2026-05-30; code-verified)** (`CLAUSE-SPEC-0`) — [`phase_m_clause_spec0_acceptance_review_results.md`](tests/phase_m_clause_spec0_acceptance_review_results.md) | L3 stays parked unless product separately authorizes ClauseThing |
| **L3 — ClauseThing** | ClauseScript-facing authoring front-end | T2 | **parked (separate track) — pending separate product authorization** | explicit ClauseThing authorization (NOT opened by L2 acceptance) |
| **A — Nested Resource Flow** | E-11B / E-11B-5 hierarchical allocation (depth > 2) | T2 | **NamedScenarioAccepted; A-0 landed — pending Opus review** | design-authority acceptance of A-0 |
| **B — Discrete hard-currency ordering** | D-2 / D-2a sequential cross-band ordering | T2 | **B-0 ACCEPTED (B-0-ACCEPT-0, 2026-05-30) — Line B CLOSED at narrow smoke level** | **none** — no B-1; future mixed-kind/multi-band ordering needs a named scenario |
| **C — Atlas / multi-theater mapping** | M-4 / M-4A atlas batching | T2 | **C-0/C-1/C-2 ACCEPTED (C-2-ACCEPT-0, 2026-05-30) — map batching CLOSED at the designer surface** (proof + scale model + atlas admission relaxation) — VRAM budget 1.5 GiB default, configurable, no hard cap | **none** — production atlas runtime / sparse-residency scheduler is a separate later gate (not open) |

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
- **Status:** **Landed + ACCEPTED (L1-ACCEPT-0, design authority, 2026-05-30).** L1-0 + L1-1 are the
  sufficient designer/spec admission substrate; L2 / `CLAUSE-SPEC-0` is now open. Do **not** jump
  ahead to ClauseScript parsing or production wiring.
- **Ladder:**

| Step | Intent | Class | Fingerprint | PR | Report |
|---|---|---|---|---|---|
| L1-0 | Designer admission substrate preflight: shared diagnostics + guardrail rejection vocabulary + accepted FrontierV2 artifact target names | Done | — | — | [`phase_m_l1_0_designer_admission_substrate_results.md`](tests/phase_m_l1_0_designer_admission_substrate_results.md) |
| L1-1 | Designer admission RON preflight manifest + diagnostic preview | Done | — | — | [`phase_m_l1_1_designer_preflight_manifest_results.md`](tests/phase_m_l1_1_designer_preflight_manifest_results.md) |
| L1-ACCEPT-0 | Design-authority closure: L1 sufficient to open L2 / CLAUSE-SPEC-0 (one non-blocking preview.rs diagnostic-code nit noted for L2) | Accepted | — | — | [`phase_m_l1_acceptance_review_results.md`](tests/phase_m_l1_acceptance_review_results.md) |

## 5. L2 — CLAUSE-SPEC (ACCEPTED)

- **What:** `CLAUSE-SPEC-0` — Designer-Facing FrontierV2 Spec Admission: admit a **designer-authored**
  FrontierV2 scenario through `simthing-spec` and compile it to the **same accepted runtime artifacts**
  the L0 fixtures exercised (RON-first; ClauseScript/ClauseThing later). The fixture guardrails
  **relocate to admission rejections** here, consuming the L1-0 diagnostic vocabulary and L1-1
  preflight manifest substrate.
- **Status: ACCEPTED (Opus design authority, 2026-05-30; code-verified).** Acceptance of record:
  [`tests/phase_m_clause_spec0_acceptance_review_results.md`](tests/phase_m_clause_spec0_acceptance_review_results.md).
  The RON-first scenario admits, **reuses L1 preflight/diagnostics** (`preview_designer_admission_preflight`),
  lowers **metadata-only** to accepted FrontierV2 artifact targets, and enforces **all** guardrails at
  admission (Resource-Flow bypass, cross-entity/production movement, production commitment, shared-pool
  tick writes, nested E-11B / E-11B-5 / D-2a, CPU planner/urgency/commitment, scheduler/cache, semantic
  WGSL, `simthing-sim` semantic leakage, FrontierV2-5, ACT/EVENT/OBS/PIPE reopen, ClauseScript/ClauseThing
  — each rejected). The L1 diagnostic nit is resolved (`MalformedManifest` + `UnknownArtifactTarget`).
  Scope is designer-authored FrontierV2 admission + compile-to-accepted-artifacts only — **not** the
  ClauseScript parser/front-end (L3, still parked) and **not** production `SimSession` wiring.
  Default-off, opt-in. **This acceptance does NOT open L3.**
- **Ladder:**

| Step | Intent | Class | Fingerprint | PR | Report |
|---|---|---|---|---|---|
| CLAUSE-SPEC-0 | Admit designer-authored FrontierV2 scenario → accepted runtime artifacts | **ACCEPTED (design authority)** | — | — | impl: [`phase_m_clause_spec0_frontier_v2_admission_results.md`](tests/phase_m_clause_spec0_frontier_v2_admission_results.md); acceptance: [`phase_m_clause_spec0_acceptance_review_results.md`](tests/phase_m_clause_spec0_acceptance_review_results.md) |

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

## M/E/T named consumer scenarios

V7.8-MET-SCENARIO-0 creates the minimum named consumer scenario pack for the promoted M/E/T
lines:

- E / Line A: nested Resource Flow fanout scenario.
- T / Line B: hard-currency contention ordering scenario.
- M / Line C: multi-theater atlas mapping scenario.

These scenarios name the need and prepare the gates. They do not authorize implementation by
themselves.

| Step | Intent | Class | Fingerprint | PR | Report |
|---|---|---|---|---|---|
| V7.8-MET-SCENARIO-0 | Named consumer scenario pack for promoted M/E/T lines | Done | — | — | [`phase_m_v7_8_met_consumer_scenarios_results.md`](tests/phase_m_v7_8_met_consumer_scenarios_results.md) |

---

## 7. Line A — Nested Resource Flow ladder (E-11B / E-11B-5) — **NamedScenarioAccepted; A-0 landed (pending review)**

- **Current state:** **NamedScenarioAccepted** (`NestedResourceFlowDepthFanout`, depth 4) — see
  [`phase_m_v7_8_met_scenario_acceptance_review_results.md`](tests/phase_m_v7_8_met_scenario_acceptance_review_results.md).
  **A-0 implementation evidence landed** (static nested arena materialization + D=3/D=4 GPU parity); **pending Opus/design-authority review** — not accepted yet. `FlatStarResourceFlow` remains the bounded production posture; `PipelineFlags::default().use_accumulator_resource_flow` stays `false`.
- **A-0 scope:** first nested-arena slice — static nested materialization + per-parent contiguous SlotRange + D=3/D=4 GPU/CPU oracle parity; not default-on Resource Flow; not E-11B-5 dynamic enrollment.
- **Ladder:**

| Step | Intent | Class | Fingerprint | PR | Report |
|---|---|---|---|---|---|
| A-0 | Static nested Resource Flow first slice: nested arena materialization + D=3/D=4 GPU parity + per-parent contiguous SlotRange proof. WGSL-GUARD-0 removed stale global filename bans; WGSL-GUARD-R1 cleaned stray artifacts and no-op placeholders. | Done / Pending Opus Review | — | #358 | impl: [`phase_e_a0_nested_resource_flow_static_results.md`](tests/phase_e_a0_nested_resource_flow_static_results.md); A-0-R1 + R1 cleanup: [`phase_e_wgsl_guardrail_r1_cleanup_results.md`](tests/phase_e_wgsl_guardrail_r1_cleanup_results.md) |

## 8. Line B — Discrete hard-currency ordering ladder (D-2 / D-2a) — **B-0 ACCEPTED; Line B CLOSED at narrow smoke level**

- **Current state:** **B-0 ACCEPTED (B-0-ACCEPT-0, Opus design authority, 2026-05-30; code+test
  verified)** — [`phase_t_b0_acceptance_review_results.md`](tests/phase_t_b0_acceptance_review_results.md).
  Authored `order_band` → existing AccumulatorOp `GateSpec::OrderBand(n)` execution; cross-band
  same-source sequential debits deterministic with exact CPU-oracle parity; same-band double-debit
  rejection preserved (per-band); Resource Flow not used for hard-currency; no new WGSL/role/CPU
  fallback/global scheduler. **Line B/T is closed at the narrow smoke level for the current named
  scenario; no B-1 opens.**
- **Future (named-scenario-gated only):** mixed-kind (transfer/recipe/emission) multi-band ordering
  or an all-band-union contention policy — the schedule key already carries `kind_rank`, but
  exercising it needs a named scenario, not a speculative B-1.
- **Ladder:**

| Step | Intent | Class | Fingerprint | PR | Report |
|---|---|---|---|---|---|
| B-0 | Narrow driver-only D-2a hard-currency ordering smoke: authored order_band wiring + deterministic boundary schedule + exact oracle parity | **ACCEPTED (design authority)** — GpuVerified + exact CPU parity | — | #357 | impl: [`phase_t_b0_d2a_hard_currency_ordering_results.md`](tests/phase_t_b0_d2a_hard_currency_ordering_results.md); acceptance: [`phase_t_b0_acceptance_review_results.md`](tests/phase_t_b0_acceptance_review_results.md) |

## 9. Line C — Atlas / multi-theater mapping ladder (M-4 / M-4A) — **NamedScenarioAccepted; C-0 OPEN (priority)**

- **Current state:** **C-0/C-1/C-2 ACCEPTED — map batching CLOSED at the designer surface**
  (C-2-ACCEPT-0, Opus design authority + product, 2026-05-30; code+test verified, 2 compile breaks
  remediated inline) — [`phase_m_c2_acceptance_review_results.md`](tests/phase_m_c2_acceptance_review_results.md),
  [`phase_m_c_acceptance_review_results.md`](tests/phase_m_c_acceptance_review_results.md).
  Proof (C-0, real packed-atlas GPU path, algebraic G=0, full-tile protocol-oracle parity
  `GpuVerifiedApproximate`) + 2000-star scale model (C-1) + **bounded algebraic-G=0 atlas admission
  relaxation (C-2)**. `request_atlas_batching` now admits **only** bounded algebraic-G=0,
  homogeneous-square, protocol-oracle-backed specs that fit the active budget with multiplier
  reporting; everything else stays rejected. `MappingExecutionProfile` default stays `Disabled`.
  **The atlas production runtime / sparse-residency scheduler is a separate later gate, not open.**
  Isolation policy ratified (algebraic `G=0` at 1.0× VRAM; physical gutter `G≥H` fallback at 6.76×,
  not a C-2 path).
- **VRAM budget (set by design authority/product, 2026-05-30):** **1.5 GiB default ceiling**
  (`V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES = 1_610_612_736`), **configurable, no architectural hard
  cap** — dedicated/headless servers and bigger-VRAM cards raise `max_bytes` far beyond the default;
  **VRAM-multiplier reporting mandatory** (occupancy checked against the *active* budget, not a
  constant). Typed term: `V78AtlasVramBudget` in `simthing-spec`.
- **C-0 scope (bounded):** first **§11-gate M-4 slice** — full-tile protocol-oracle parity (vs an
  exact per-tile-protocol CPU oracle, not corridor-t44 alone) **+** a VRAM-multiplier report against
  the active budget. Algebraic mask `G=0` preferred; physical gutter `G≥H` fallback. Opt-in,
  default-off; no semantic WGSL; `simthing-sim` stays map-free. **C-0 is opened for implementation;
  the scenario-acceptance pass did not implement M-4.**
- **Not in this ladder:** active-mask halo (**M-6A**) and source identity (**M-5 / `source_mask`**)
  stay deferred in place under the Mapping ADR.
- **Ladder:**

| Step | Intent | Class | Fingerprint | PR | Report |
|---|---|---|---|---|---|
| C-0 | First §11-gate M-4 atlas slice: full-tile protocol-oracle parity + VRAM-multiplier report vs active budget (1.5 GiB default, configurable) | **ACCEPTED (design authority)** — GpuVerifiedApproximate (3.05e-5 ≤ 1e-4) | `a974fe44e20620f3` | — | impl: [`phase_m_c0_m4_atlas_protocol_oracle_results.md`](tests/phase_m_c0_m4_atlas_protocol_oracle_results.md); acceptance: [`phase_m_c_acceptance_review_results.md`](tests/phase_m_c_acceptance_review_results.md) |
| C-1 | 2000-star atlas scale model: budget fit for algebraic G=0 and gutter fallback estimate | **ACCEPTED (design authority)** | — | — | impl: [`phase_m_c1_atlas_2000_star_scale_model_results.md`](tests/phase_m_c1_atlas_2000_star_scale_model_results.md); acceptance: [`phase_m_c_acceptance_review_results.md`](tests/phase_m_c_acceptance_review_results.md) |
| C-2 | Atlas admission relaxation (designer/spec): admit bounded **algebraic-G=0** atlas specs that are homogeneous-square, protocol-oracle-backed, fit the active `V78AtlasVramBudget`, and carry multiplier reporting. `request_atlas_batching` relaxed only through this scope. No production runtime / default wiring / default-on / scheduler. | **ACCEPTED (design authority)** — 2 compile breaks remediated inline (non-exhaustive match + private-import test) | — | — | impl: [`phase_m_c2_atlas_admission_relaxation_results.md`](tests/phase_m_c2_atlas_admission_relaxation_results.md); acceptance: [`phase_m_c2_acceptance_review_results.md`](tests/phase_m_c2_acceptance_review_results.md) |

---

## Cleanup / evidence hygiene

V7.8-CLEAN-0 preserved authoritative evidence and, at the time, confirmed L1 as the next gate.
Since then L1/L2 have been accepted and **C-0 is now the open implementation gate** (landed,
pending Opus review).

| Step | Intent | Class | Fingerprint | PR | Report |
|---|---|---|---|---|---|
| V7.8-CLEAN-0 | Active-docs slimming, archive move, and stale evidence prune | Docs-only | — | — | [`phase_m_v7_8_cleanup_track_prune_results.md`](tests/phase_m_v7_8_cleanup_track_prune_results.md) |

---

## 10. Sequencing summary

L0 is **done and accepted**. L1 (simthing-spec buildout) is **done and accepted** (L1-ACCEPT-0).
L2 / CLAUSE-SPEC-0 is **ACCEPTED**. L3 ClauseThing remains parked unless product separately
authorizes it. **Line C / M (map batching) is CLOSED at the designer surface: C-0, C-1, and C-2 are
all ACCEPTED** (proof + 2000-star scale model + bounded algebraic-G=0 atlas admission relaxation).
The atlas **production runtime / sparse-residency scheduler** is a separate later gate and is **not
open**. **Line B/T (discrete hard-currency ordering) is CLOSED at the narrow smoke level: B-0 is
ACCEPTED (B-0-ACCEPT-0); no B-1 opens.** **A-0 implementation landed — pending Opus/design-authority review** (static nested Resource Flow first slice; not accepted yet). E-11B-5 dynamic enrollment remains deferred;
ClauseThing/ClauseScript stay parked until explicitly authorized. The AccumulatorOp v2 production plan stays **CLOSED**; v7.7
stays the **binding baseline**; the three lines live in v7.8.
