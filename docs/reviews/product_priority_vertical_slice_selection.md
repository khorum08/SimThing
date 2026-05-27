# Product-Priority Vertical Slice Selection Review

**Status:** Accepted (design/readiness review, docs-only)  
**Date:** 2026-05-27  
**Scope:** Post–continued flat-star soak checkpoint — select the next implementation track only after naming the product scenario that justifies it  
**Audience:** Cursor implementation handoff, GPT review, production plan maintainers  
**Prior art:** [`d2a_boundary_transaction_scheduling_readiness.md`](d2a_boundary_transaction_scheduling_readiness.md), [`e11b_nested_dynamic_enrollment_readiness.md`](e11b_nested_dynamic_enrollment_readiness.md), [`resource_flow_limited_scenario_class_posture.md`](../resource_flow_limited_scenario_class_posture.md)

---

## Executive summary

Engineering confidence work for bounded **FlatStarResourceFlow** is in a good state: RF-T1 through RF-T6, continued flat-star soak (512 static @ 1000 ticks, dynamic Policy A, multi-arena, replay), E-11 flat-star, E-2B static + Policy A dynamic enrollment, and Phase T hard-currency opt-in paths are landed and green.

**No named product scenario** currently justifies opening D-2a, E-11B-5, simthing-spec/RON rebuild, or a new runtime vertical slice. The substrate is ready; the missing input is **product-authored scenario definition** (participants, boundaries, ordering needs, authoring surface, success metrics).

**Recommendation: F — Pause implementation and gather product requirements.**

| Option | Verdict |
|--------|---------|
| **A. New scenario-driven vertical slice (FlatStarResourceFlow / Phase T primitives)** | **Not authorized** — no named scenario |
| **B. Narrow D-2a** | **Not authorized** — no named hard-currency ordering scenario |
| **C. Narrow E-11B-5** | **Not authorized** — no named nested dynamic Resource Flow scenario |
| **D. simthing-spec/RON/Designer rebuild** | **Not authorized** — authoring track has not intentionally opened |
| **E. Additional flat-star soak** | **Not needed** — continued soak closed the identified evidence gap |
| **F. Pause and gather product requirements** | **Recommended** |

**Next implementation gate depends on this review recommendation:** do not implement runtime behavior until product names a scenario and the corresponding track (A–E) is re-selected with explicit authorization.

---

## 1. Current-state audit

| Layer | Status |
|-------|--------|
| AccumulatorOp v2 substrate | Production runtime path; legacy passes deleted |
| FlatStarResourceFlow | Accepted bounded production Resource Flow posture |
| Continued flat-star soak | **Done** — 12/12; 512 static @ 1000 ticks; replay green |
| E-11B | **Paused** — static nested D=3/D=4 + fission/gap hardening landed |
| E-11B-5 nested dynamic enrollment | Readiness done; **not authorized** without named scenario |
| D-2a boundary scheduling | Readiness done; **deferred** without named multi-transaction scenario |
| Phase T hard-currency | Complete; opt-in only; global transfer/emission flags default false |
| simthing-spec/RON/Designer rebuild | **Deferred** until authoring track opens |
| Global Resource Flow default-on | **Rejected / deferred** |

**Constitutional posture preserved:** `simthing-sim` arena-ignorant and spec-free; Resource Flow separate from Phase T; no WGSL, new roles, CPU fallback, slot compaction, indirection SlotRange, or Policy B.

---

## 2. Review questions — concrete answers

| # | Question | Answer |
|---|----------|--------|
| 1 | What product scenario is now the best next target? | **None named.** Engineering is ahead of product scenario definition. |
| 2 | Does it require D-2a hard-currency ordering? | **No** — no scenario identified that needs sequential cross-band debits on the same source within one boundary. |
| 3 | Does it require E-11B-5 nested dynamic enrollment? | **No** — no scenario identified that needs nested Resource Flow growth after session open. |
| 4 | Does it require simthing-spec/RON/Designer guardrail rebuild? | **No** — no authoring-track opening or designer UX scenario named. |
| 5 | Can it be expressed using current bounded FlatStarResourceFlow? | **Unknown** — cannot evaluate without a named scenario. Existing posture covers flat-star D=2, static/dynamic Policy A enrollment, multi-arena no-coupling, and continued soak scale. |
| 6 | Does it touch Phase T hard-currency transfer/recipe/emission? | **Unknown** — Phase T primitives exist for opt-in discrete transfer/recipe/emission; no new product scenario specifies hard-currency behavior beyond current T-5/T-6 burn-in fixtures. |
| 7 | What evidence already exists? | RF-T1–T6; E-11/E-2B/E-2B-5 soak; continued flat-star soak; Phase T burn-in + designer RON smoke; D-2a and E-11B readiness reviews. |
| 8 | What new implementation gate should be authorized? | **None now.** Re-run this selection review after product names a scenario. |
| 9 | What tests must exist before implementation? | Track-specific — see §4. Any new gate requires named fixture + burn-in/replay before runtime changes. |
| 10 | What stop conditions require Opus? | Policy B, selector re-run, compaction, indirection SlotRange, new WGSL/roles, global default-on, hard-currency through Resource Flow, simthing-sim arena awareness — per prior readiness reviews. |

---

## 3. Track evaluation

### A. Scenario-driven vertical slice (FlatStarResourceFlow / Phase T)

**When to choose:** Product names a bounded scenario expressible with:
- `ResourceFlowExecutionProfile::FlatStarResourceFlow` and/or `FlatStarOptIn`
- Static and/or Policy A dynamic fission enrollment
- Optional Phase T `ResourceEconomyOptInMode` for discrete hard-currency (separate from Resource Flow)

**Current blocker:** No scenario brief (name, participant scale, fission cadence, arenas, hard-currency rules, replay/conservation acceptance).

### B. Narrow D-2a

**When to choose:** Product names a **multi-transaction hard-currency** scenario requiring sequential debits on the same `(property, col)` across authored `order_band` values within one boundary, and same-band rejection is insufficient.

**Current blocker:** D-2a readiness explicitly found no shipped workload blocked today; `order_band` wiring gap documented but no product pull.

### C. Narrow E-11B-5

**When to choose:** Product names a **nested dynamic Resource Flow** scenario requiring post-open allocation participant admission under nested interior parents (not flat-star Policy A append).

**Current blocker:** E-11B paused; nested dynamic enrollment readiness recommends defer until named scenario.

### D. simthing-spec/RON/Designer rebuild

**When to choose:** Authoring track intentionally opens with designer UX, guardrail, and RON validation requirements beyond Phase T smoke.

**Current blocker:** Full rebuild remains deferred; smoke addendum sufficient for current Phase T posture.

### E. Additional flat-star soak

**When to choose:** A **specific evidence gap** remains after continued soak (e.g., new fixture class, longer tick horizon, new telemetry field).

**Current assessment:** Continued soak closed scale/replay/dynamic/multi-arena confidence for 1000 ticks including 512 static. No specific gap named.

### F. Pause and gather product requirements (recommended)

**Rationale:**
1. Substrate and bounded posture are proven; further engineering without product scenario risks speculative scope expansion.
2. D-2a, E-11B-5, and spec rebuild each have clear **named-scenario gates** — none are satisfied.
3. Global default-on remains rejected; no pressure to widen Resource Flow semantics preemptively.

---

## 4. Required product input before any implementation gate

Product should supply a **scenario brief** containing at minimum:

| Field | Purpose |
|-------|---------|
| Scenario name / ID | Authorization anchor for implementation PR |
| Primary track (A–E) | Which gate to open |
| Resource Flow shape | Flat-star vs nested; static vs dynamic; arena count |
| Participant scale / fission cadence | Fixture sizing |
| Hard-currency needs | Whether Phase T opt-in suffices or D-2a ordering is required |
| Authoring surface | Spec/RON/Designer involvement |
| Conservation / replay acceptance | Burn-in criteria |
| Explicit non-goals | What the scenario must not trigger (global default-on, Policy B, etc.) |

### Tests required before implementation (by track)

| Track | Pre-implementation test obligation |
|-------|-----------------------------------|
| **A** | Named fixture in opt-in or profile soak suite; 1000-tick burn-in + replay; telemetry contract |
| **B** | Named multi-debit hard-currency fixture; T-2 compile + boundary sync expectations; D-2a ladder from readiness review |
| **C** | Named nested dynamic fixture; E-11B-5 pre-implementation test list from nested dynamic enrollment readiness |
| **D** | Authoring acceptance criteria + RON roundtrip/regression plan (separate track) |
| **E** | Named evidence gap + targeted soak extension only |

---

## 5. Stop conditions / Opus triggers

Recommend **Opus review** if a named product scenario requires any of:

- Policy B Reevaluate or selector re-run
- Global Resource Flow default-on
- Hard-currency transfer through Resource Flow
- Nested dynamic enrollment without E-11B-5 narrow ladder
- D-2a expansion beyond driver-only `order_band` wiring
- New WGSL or new `AccumulatorRole` variants
- CPU production allocation fallback
- Boundary-time slot compaction or indirection-list SlotRange
- `simthing-sim` arena or spec awareness

---

## 6. Recommendation

### Verdict: **F — Pause implementation and gather product requirements**

**Authorized now:**
- Documentation and product scenario definition only
- Continued use of landed FlatStarResourceFlow and Phase T opt-in paths in product design discussions

**Not authorized now:**
- D-2a implementation
- E-11B-5 nested dynamic enrollment
- simthing-spec/RON/Designer rebuild
- New runtime Resource Flow semantics
- Global default-on
- Additional flat-star soak (no named gap)

**Re-selection trigger:** Product delivers a named scenario brief satisfying §4. Then re-run this review or a narrow track-specific readiness memo and authorize **exactly one** of A–E.

---

## 7. Docs update requirements

When a track is authorized after product names a scenario:

- Update `accumulator_op_v2_production_plan.md`, `todo.md`, `worklog.md`, `workshop_current_state.md`
- Add named fixture references and test report paths before implementation PR
- Do not widen posture beyond the named scenario

**This review PR:** docs-only; no production code.

---

## 8. Verdict table

| Question | Answer |
|----------|--------|
| Best next target? | **None named — pause (F)** |
| D-2a required? | **No** |
| E-11B-5 required? | **No** |
| Spec/RON rebuild required? | **No** |
| FlatStarResourceFlow sufficient for unnamed future? | **Unknown until scenario named** |
| Phase T involved? | **Only if scenario specifies hard-currency; no scenario named** |
| Next gate | **Gather product requirements; re-select A–E when scenario exists** |
