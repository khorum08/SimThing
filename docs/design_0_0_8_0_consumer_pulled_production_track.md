# SimThing — Design 0.0.8.0 Consumer-Pulled Production Track

> **Status:** OPEN as track definition; no implementation gate open.
>
> **Active constitution:** [`design_0_0_8_0.md`](design_0_0_8_0.md).
>
> **0.0.7.9 mobility/transfer substrate:** COMPLETE and PARKED
> ([`design_v7_9_mobility_transfer_allocation_production_track.md`](design_v7_9_mobility_transfer_allocation_production_track.md)).
> No parked substrate moves to production until a **named product scenario** pulls it.

---

## 1. Purpose

0.0.8.0 redirects effort from **substrate-ahead-of-need** to **consumer-pulled integration**
(`design_0_0_8_0.md` §1, §5). The 0.0.7.9 track proved large, correct, opt-in/default-off substrate
that nothing in production consumed — and generated hygiene loops (recombination soaks, accounting-over-
accounting, passive proof wrappers) while waiting for a consumer.

**This track operationalizes SCENARIO-FIRST authoring:** the next artifact is a **named product
scenario**, not another fixture, soak, replay, accounting summary, manifest, or substrate expansion.
A substrate's production-path gate opens **only because a named scenario consumes it** — never
speculatively.

This is not implementation. This is not a substrate track. This is not another proof wrapper.

**ClauseThing / ClauseScript** remains a horizontal future authoring surface. Scenario admission
must not assume ClauseThing support, but should avoid choices that would make a future ClauseThing
front-end impossible.

---

## 2. Operating doctrine summary

- **Guardrails at designer/spec admission; runtime enforces hard safety** (`design_0_0_8_0.md` §2.1;
  `invariants.md` two-layer guardrail placement).
- **Tier-1 fast-lane** — accepted, generic/semantic-free, opt-in/default-off, parity-backed,
  reversible work ships as one implementation PR + one test report + one status-row update
  (`phase_m_gating_and_doc_policy.md` §1).
- **Tier-2 gate** — new scenario gates, default `SimSession`, default schedule, gameplay surface,
  semantic WGSL, invariant edits, new architecture, or closed-ladder reopen.
- **Proven-capability stop rule** — no passive proof wrappers unless they unlock execution
  (`phase_m_gating_and_doc_policy.md` §6; 0.0.7.9 track §2.2).
- **One principle per class; no per-slice accretion** (`invariants.md` governing doctrine).

---

## 3. ClauseThing horizon non-goal

ClauseThing / ClauseScript is a **horizontal future designer-facing aspiration** — not active
scope for this track.

- This track does **not** open ClauseThing.
- This track does **not** require ClauseScript expressiveness.
- This track does **not** alter `simthing-spec`.
- Scenario packets should target the **current accepted `simthing-spec` admission surface**
  (L0/L1/L2 / CLAUSE-SPEC).
- Future ClauseThing compatibility is a **non-blocking horizon concern only** — avoid admission
  choices that would foreclose a later front-end, but do not design for ClauseScript now.

---

## 4. Parked substrate inventory

Compact reference only — do not re-litigate. Each row is **complete or accepted at first slice,
opt-in/default-off, parked** until a named scenario pulls it.

| Capability | State | Opens when… |
|---|---|---|
| **0.0.7.9 mobility/transfer substrate** (ALLOC, REENROLL, IDROUTE, ECON, OWNER + RUNTIME-0/1A/1B + semantic-free GPU kernel substrate) | COMPLETE + PARKED | a named scenario needs mobility/ownership/economy in the default `SimSession` path |
| **Line A — nested Resource Flow (A-0)** | ACCEPTED, static nested first slice; production posture is `FlatStarResourceFlow` | a named economy needs depth>2 nested fanout |
| **Line B — discrete hard-currency ordering (B-0)** | ACCEPTED, narrow smoke; no B-1 | a named multi-transaction hard-currency workload |
| **Line C — atlas / multi-theater mapping (C-0/C-1/C-2)** | ACCEPTED; map batching CLOSED at designer surface | a named multi-theater scenario opens the atlas production runtime gate |
| **simthing-spec / CLAUSE-SPEC (L0/L1/L2)** | ACCEPTED designer-admission substrate | engine of this track's scenario authoring |
| **ClauseThing / ClauseScript (L3)** | PARKED pending product authorization | product authorizes the front-end |
| **Deferred-by-design** | E-11B-5, atlas production runtime, B-1, Hybrid-Strata/faction-index ECON scaling, FrontierV2-5, ACT/EVENT/OBS/PIPE | each requires its own named product scenario or product authorization — not open questions |

---

## 5. First gate: SCENARIO-0080-0

| Field | Value |
|---|---|
| **Gate ID** | `SCENARIO-0080-0` |
| **Type** | Tier-2 scenario/admission gate |
| **Deliverable** | Named product-scenario / admission packet |
| **Runtime implementation** | **No** |
| **Production wiring** | **No** |
| **Default `SimSession`** | **No** |
| **Default schedule** | **No** |
| **Gameplay surface** | **No** |
| **On acceptance** | Opens **only** the named substrate's already-defined production-path gate |

---

## 6. SCENARIO-0080-0 admission packet — required declarations

The admission packet must declare:

1. **Scenario name and product purpose**
2. **User-facing / product behavior** the scenario is meant to enable
3. **Which one parked substrate** it consumes (exactly one)
4. **Why that substrate is required now** (consumer pull — not speculative substrate build)
5. **Bounds and scale**
6. **Designer/spec admission vocabulary**
7. **Rejection vocabulary and diagnostics**
8. **Runtime path requested**, if any
9. **Whether production default path is requested**
10. **Whether default schedule is requested**
11. **Whether gameplay-facing integration is requested**
12. **Test/evidence needed for admission**
13. **Explicit non-goals**
14. **Exit criteria for acceptance**

Acceptance is **design-authority + product only**. No self-acceptance.

---

## 7. Recommended first candidate (not authorization)

**Most-ready candidate:** 0.0.7.9 mobility/transfer substrate.

**Reason:** Its production-path gate is already mapped as the first non-test-support default
`SimSession` path / default schedule (0.0.7.9 track top status; MOBILITY-SCENARIO-0 accepted the
scenario discipline).

**Constraint:** It must still be **pulled by a named product scenario**. Do not open
`PRODUCTION-PATH-0080-0` speculatively.

---

## 8. Stop conditions

Reject the scenario if it requires any of:

- owner-entity as spatial parent
- capture-as-reparenting
- semantic/raw WGSL
- default-on behavior without a production gate
- hard-currency through Resource Flow
- CPU planner / urgency / commitment emission
- reopening atlas runtime, E-11B-5, B-1, ClauseThing/L3 front-end, FrontierV2-5, ACT/EVENT/OBS/PIPE,
  Hybrid-Strata/faction-index scaling, or any closed ladder without its own product authorization
- passive proof wrappers that do not unlock execution

---

## 9. Ladder index

| Gate | Description | Status |
|---|---|---|
| `SCENARIO-0080-0` | Scenario/admission packet | **OPEN** — first gate; no implementation |
| `PRODUCTION-PATH-0080-0` | First substrate production-path gate | **CLOSED** until SCENARIO-0080-0 accepts a named consumer |
| `GAMEPLAY-0080-0` | Gameplay-facing integration | **CLOSED** |
| `SEMANTIC-WGSL-0080-0` | Semantic shader surface | **CLOSED** |
| `CLAUSETHING-L3-0080-0` | Front-end / parser / product authoring surface | **PARKED** pending product authorization |

---

## 10. Evidence / test-report policy

- **This track file:** no test report required.
- **Future SCENARIO-0080-0 admission:** one compact admission report only:
  [`docs/tests/phase_scenario_0080_0_admission_results.md`](tests/phase_scenario_0080_0_admission_results.md)
  (create when the admission packet is authored — not in this PR).
- Do not create reports for passive proof wrappers.
- Do not create per-slice ceremony.

---

## 11. Pointers

- Active constitution: [`design_0_0_8_0.md`](design_0_0_8_0.md)
- Parked 0.0.7.9 mobility/transfer track: [`design_v7_9_mobility_transfer_allocation_production_track.md`](design_v7_9_mobility_transfer_allocation_production_track.md)
- Gating mechanics + proven-capability stop rule: [`workshop/phase_m_gating_and_doc_policy.md`](workshop/phase_m_gating_and_doc_policy.md)
- Binding structural rules: [`invariants.md`](invariants.md)
- Active status table + read order: [`workshop/mapping_current_guidance.md`](workshop/mapping_current_guidance.md)
