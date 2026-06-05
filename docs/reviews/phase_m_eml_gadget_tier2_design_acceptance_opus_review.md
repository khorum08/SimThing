# Opus/Product Acceptance Review — Phase M EML-GADGET-2 Temporal-Memory Design

**Date:** 2026-05-29
**Authority:** Opus 4.8, mapping/FIELD_POLICY design authority under human delegation. Guardrail authority
extends to the designer-facing studio/importer layer and the scenario definition stage — not the sim
or boundary layer.
**Decision type:** **Design-gate acceptance — not an implementation handoff.** No code changed; no
temporal gadget is authorized for implementation by this memo.
**Reviews:** `docs/reviews/phase_m_eml_gadget_tier2_temporal_memory_design_review.md` and
`docs/tests/phase_m_eml_gadget_tier2_design_review_test_results.md`.
**Builds on:** `docs/reviews/phase_m_eml_gadget_tier1_acceptance_opus_review.md` (EML-GADGET-1
accepted), `docs/workshop/eml_gadget_library_design_note.md`, `docs/invariants.md`.

---

## 1. Executive verdict

**PASS WITH CONDITIONS.** The EML-GADGET-2 temporal-memory design is **accepted as a design gate
only.** Implementation remains unauthorized until a separate EML-GADGET-2A handoff. The four core
design claims hold; the candidate classification and the 2A→2D ladder are approved with the
conditions below. Temporal memory stays **explicit-column state** (no implicit previous-value read),
**Layer-3 scoped by default**, with **authored snapshot/copy bands** and a **bounded-feedback
admission contract**. `VelocityMonitor`, `Decay`/`EMA`, and `BoundedFeedback` are approved
candidates; `Hysteresis` is conditional; `Acceleration` is deferred.

This design is, notably, the *more* legible and deterministic choice: explicit-column temporal state
is replay-serializable and admission-inspectable, where a hidden previous-buffer opcode would not be.
The missing-opcode constraint produced the better architecture — accept it as a feature.

---

## 2. Evidence reviewed

The design-review packet (read in full) and the verification report. Cross-checked against the
Tier-1 acceptance, the design note §6–§7, and the invariants. **Verified in code (not taken on the
packet's word):**

- **The snapshot/copy band is expressible with existing primitives — no new opcode.** A pure
  cross-column snapshot `previous_col ← current_col` is `Identity` combine + `ConsumeMode::ResetTarget`
  into the `previous_col` target, scheduled at an earlier `OrderBand`. The first-slice runtime
  (`first_slice_mapping_runtime.rs`) already drives `Sum/EvalEML → ResetTarget` into arbitrary target
  columns with band ordering (`OrderBand(0)` reduction → `OrderBand(1)` EML), and `ResetTarget`/
  `ScaleTarget`/`AddToTarget` ordinals are live in `accumulator_op/types.rs`. So Band-A snapshot,
  Band-B compute, Band-C emit all map onto the existing substrate.
- **Decay already exists** as `field_decay` / in-place `ScaleTarget` (`col *= α`) — no snapshot
  needed for the pure-decay degenerate case.
- **Tier-1 invariants and admission are intact** (deferred Tier-2 kinds still rejected at admission;
  oracle-per-gadget binding). Re-ran the regression set below.

**Independent verification run (this machine):** `eml_gadget_tier1` **14/14**,
`region_field_spec_admission` **11/11**, `resource_economy_authoring_preview` **8/8** — Tier-1 and
admission baselines green; nothing in this design-gate touches them.

---

## 3. Acceptance decision (answers to the four core claims + key question)

1. **Temporal memory is explicit-column state — ACCEPT.** EML gains no implicit previous-value read;
   temporal gadgets declare `current/previous/state/output` columns; snapshot/copy bands are authored
   before update bands; the runtime never infers or secretly allocates temporal memory. This is the
   correct, deterministic, replay-safe model.
2. **Layer-3 scoped by default — ACCEPT.** Temporal memory defaults to parent/faction/personality/
   strategic columns (`SlotRange`-scoped). Dense per-cell temporal memory is **not** part of V1 and
   requires a separate gate (VRAM budget + named scenario + product authorization). Matches the cost
   discipline (work scales with the number of AI agents, not map cells).
3. **Snapshot/copy band model — ACCEPT. Key question: EXISTING SUBSTRATE SUFFICIENT.** Band A
   (`ResetTarget` copy) / Band B (`EvalEML` compute) / Band C (existing threshold+emit) are
   expressible with `Identity + ResetTarget` (and `ScaleTarget`/`AddToTarget` for in-place forms) at
   authored `OrderBand`s — verified above. **Condition:** EML-GADGET-2A must still land first as a
   *fixture-only proof* that the spec/admission layer can author the band point + column pair +
   consume mode **cleanly** (no ad-hoc per-scenario wiring). A separate generic copy/snapshot
   primitive gate is required **only if** 2A shows clean spec-layer authoring is not achievable with
   the existing consume modes. Expectation: no new primitive.
4. **Bounded-feedback admission contract — ACCEPT WITH CONDITIONS.** The §6 contract is sound:
   `|decay| < 1` guarantees BIBO stability (the recurrence converges). Conditions/refinements:
   - **(a)** V1 **defaults to `0 ≤ decay < 1`**; negative decay (bounded but oscillatory) requires
     explicit opt-in + justification, since oscillation can be gameplay-chaotic even when convergent.
   - **(b)** When a recurrent gadget's output feeds a **hard threshold/`EmitEvent`**, an explicit
     output clamp is **required** regardless of analytic boundedness (defense in depth; ties to the
     A-4 `SoftAggregateGuard` rule).
   - **(c)** The "analytically bounded formula" escape must be **admission-checkable** — the bound has
     to be derivable from the declared decay/gain/clamp, not asserted. If it isn't mechanically
     checkable, require the clamp. No hand-waved boundedness passes admission.

---

## 4. Conditions

- **C-1 (design gate only).** This memo authorizes **no implementation**. Temporal gadgets stay
  rejected at admission until the EML-GADGET-2A/2B/2C/2D ladder handoffs land, each with CPU-oracle
  parity.
- **C-2 (2A proves the snapshot band).** 2A is a fixture-only proof that the snapshot/copy band is
  cleanly authorable with existing consume modes. Escalate to a separate copy/snapshot primitive
  gate only if 2A demonstrates it cannot be.
- **C-3 (bounded-feedback refinements).** §3.4 (a)/(b)/(c) above are binding on any recurrent gadget.
- **C-4 (Tier-1 conditions preserved).** EML-GADGET-1 C-1–C-4 carry forward: preview ≠ runtime;
  `PerGadgetOnly` only; oracle-per-gadget; no driver/gpu/sim consumption of compiled gadget output.
- **C-5 (stateful-sequence parity).** Tier-2 oracle parity is **multi-step sequence** parity (not
  single-step), with a declared initial-step policy (first delta = 0 / skip), per the §7 plan.

---

## 5. Tier-2 candidate classification

| Gadget | Verdict | Notes |
|---|---|---|
| **VelocityMonitor** | **ACCEPT** | `(current − previous)/dt`; pre-update snapshot column; `ExactDeterministic`; lands in 2B |
| **Decay / EMA** | **ACCEPT** | Decay = in-place `ScaleTarget` (no snapshot, may alias `field_decay`); EMA = blend needing a snapshot column. One descriptor, snapshot column present only for the EMA form. `0 ≤ decay < 1`. 2B |
| **BoundedFeedback** | **ACCEPT WITH STRICT ADMISSION** | The §6 contract + §3.4 refinements. Riskiest (divergence) → lands **after** 2B, in 2C |
| **Hysteresis** | **CONDITIONAL / DEFER to 2D** | Design accepted; implement only on demonstrated commitment-chatter need. It is the sanctioned A-4 soft-aggregate guard, so it will likely be needed — but gate on the actual need |
| **Acceleration** | **DEFER** | Trivial second-difference extension of VelocityMonitor once that lands; high column/band cost; no demonstrated need. Revisit on product need |

---

## 6. Binding non-authorizations (kept binding)

```text
No implementation from this review; no runtime gadget execution.
No chained OrderBand runtime scheduling until separately gated.
No dense per-cell temporal memory by default (separate VRAM + scenario + product gate).
No new EML opcode; no transcendental temporal gadget.
No WGSL / per-gadget GPU kernel; no change to the EvalEML interpreter.
No Gadget/Personality/Memory semantics in simthing-sim.
No CPU-side planner.
No production economy→mapping bridge; economy→FIELD_POLICY stays tests/support.
No default SimSession mapping wiring; MappingExecutionProfile default Disabled.
No Resource Flow E-11 default-on.
No atlas / M-4A.
No DailyResolutionBoundary; no day/calendar/pause semantics in simthing-sim.
```

Authoritative home: `docs/invariants.md` — "EML Gadget Library" rows (temporal-memory design
contract added this pass).

---

## 7. Approved implementation ladder

| Slice | Scope | Gate |
|---|---|---|
| **EML-GADGET-2A** | Snapshot/copy band **fixture-only proof** (Identity+ResetTarget cross-column at an authored band; CPU oracle) | Lands first; confirms existing-substrate sufficiency or escalates to a copy/snapshot primitive gate |
| **EML-GADGET-2B** | `VelocityMonitor` + `Decay`/`EMA` spec/admission/compiler/stateful-sequence oracle | After 2A |
| **EML-GADGET-2C** | `BoundedFeedback` with strict admission (§6 + §3.4 refinements) | After 2B |
| **EML-GADGET-2D** | `Hysteresis` — only on demonstrated product need | After 2C, conditional |
| **Deferred** | `Acceleration`; dense per-cell temporal memory | Separate product + VRAM gate |

**Answers to the ladder questions:** (1) Yes — 2A lands first as the copy/snapshot proof.
(2) Yes — VelocityMonitor and EMA wait for 2A (they consume the proven snapshot column). (3) Yes —
BoundedFeedback waits until after VelocityMonitor/EMA (riskiest; strict admission). (4) Yes —
Hysteresis stays deferred/conditional (2D). (5) Yes — Acceleration stays deferred.

---

## 8. Stop conditions for the next handoff (2A; escalate, do not land)

The 2A handoff (and any 2B–2D that follows) must not:
- consume compiled gadget output / the flatten preview as a runtime execution path (Tier-1 C-1);
- add a new EML opcode, WGSL, or per-gadget GPU kernel, or change the `EvalEML` interpreter;
- add chained OrderBand *runtime scheduling* beyond the authored snapshot/compute/emit bands proven
  by fixture (true cross-gadget runtime chaining stays separately gated);
- place temporal-memory columns on the dense per-cell field (Layer-3 only in V1);
- admit a recurrent gadget without the bounded-feedback contract (`0 ≤ decay < 1` default; clamp
  required when feeding a hard threshold; analytically-bounded escape must be admission-checkable);
- admit any gadget without a CPU oracle + **stateful-sequence** parity;
- add Gadget/Personality/Memory semantics to `simthing-sim`, a production economy→mapping bridge,
  default mapping wiring, Resource-Flow default-on, atlas, or any day/calendar/pause sim semantics.

For **2A specifically:** it is a *fixture-only proof*; if clean spec-layer authoring of the snapshot
band is not achievable with existing consume modes, **stop and escalate** for a generic copy/snapshot
primitive gate rather than wiring it ad hoc.

---

## 9. Doc / ADR / invariant updates made alongside this memo

- **New:** this memo (authoritative design-gate acceptance).
- **`docs/reviews/phase_m_eml_gadget_tier2_temporal_memory_design_review.md`** — status flipped to
  **ACCEPTED (design gate)**.
- **`docs/invariants.md`** — "EML Gadget Library" temporal-memory row expanded to the accepted design
  contract (explicit-column state; Layer-3 default; authored snapshot/copy bands; bounded-feedback
  admission with the §3.4 refinements) — recorded as the **binding contract the EML-GADGET-2 slice
  must satisfy**, with implementation still gated.
- **`docs/workshop/eml_gadget_library_design_note.md`** — Tier-2 status → design accepted; ladder
  approved.
- **`docs/workshop/mapping_current_guidance.md`**, **`docs/workshop/workshop_current_state.md`**,
  **`docs/accumulator_op_v2_production_plan.md`**, **`docs/todo.md`** — next step = EML-GADGET-2A
  (snapshot/copy fixture proof); ladder recorded.
- **`docs/worklog.md`** — dated 2026-05-29 design-acceptance entry.

All updates are decision/classification only. No production code changed; `simthing-gpu` stays the
generic interpreter; `simthing-sim` map-free; `MappingExecutionProfile` default `Disabled`;
Resource Flow E-11 default-off; `request_atlas_batching` rejected at admission.
