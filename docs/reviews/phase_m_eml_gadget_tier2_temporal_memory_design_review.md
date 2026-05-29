# Phase M EML-GADGET-2 — Tier-2 Temporal-Memory Gadgets — Design Review

**Date:** 2026-05-29  
**Authority:** Design/review handoff only — **not** an implementation handoff. No code changed in this packet.  
**Builds on:** [`phase_m_eml_gadget_tier1_acceptance_opus_review.md`](phase_m_eml_gadget_tier1_acceptance_opus_review.md) (EML-GADGET-1 accepted), [`../workshop/eml_gadget_library_design_note.md`](../workshop/eml_gadget_library_design_note.md), [`../invariants.md`](../invariants.md) ("EML Gadget Library").  
**Related tests (Tier-1 baseline, unchanged):** [`../tests/phase_m_eml_gadget_tier1_test_results.md`](../tests/phase_m_eml_gadget_tier1_test_results.md), R1/R2 hygiene reports.

---

## 1. Executive verdict

**ACCEPTED — design gate only (Opus/product 2026-05-29, PASS WITH CONDITIONS).** See the acceptance
memo [`phase_m_eml_gadget_tier2_design_acceptance_opus_review.md`](phase_m_eml_gadget_tier2_design_acceptance_opus_review.md)
for verdicts, conditions (incl. the bounded-feedback refinements §3.4), candidate classification, and
the approved 2A→2D ladder. Implementation remains **unauthorized** until the separate EML-GADGET-2A
handoff lands; the snapshot/copy band is judged expressible with existing primitives
(`Identity + ResetTarget` at an authored `OrderBand`), to be confirmed by a 2A fixture-only proof.

**Phase M EML-GADGET-2 temporal-memory design is under review.** EML-GADGET-1 is accepted; Tier-2 temporal gadgets remain **unauthorized until this design is accepted and a separate implementation handoff lands.**

This packet reviews and proposes the design for Tier-2 temporal-memory gadgets — `VelocityMonitor`, `Decay` / `EMA`, `Acceleration`, `Hysteresis`, and `BoundedFeedback` — without implementing them. It preserves EML-GADGET-1 acceptance conditions **C-1 through C-4**:

| Condition | Binding meaning (preserved) |
|---|---|
| **C-1** | Preview ≠ runtime. No driver/gpu/sim code may consume `CompiledEmlGadgetStack`, composition nodes, or flatten preview as execution in this slice. |
| **C-2** | `PerGadgetOnly` remains the only multi-gadget composition until true intermediate-column wiring is separately proven. |
| **C-3** | No gadget, any tier, is admitted without CPU oracle and parity test. |
| **C-4** | The Opus acceptance memo supersedes the reverted parking packet (#262). |

**Recommended posture for Opus/product:** accept this design review as the pre-implementation gate; authorize implementation only via a separate EML-GADGET-2A–2D ladder handoff after answering the decision questions in §9.

---

## 2. What temporal memory means in SimThing

**Temporal memory** in the EML gadget layer is **explicit-column state carried across ticks**, not hidden runtime memory inside `EvalEML`, `simthing-gpu`, or `simthing-sim`.

Core definitions:

- **EML has no implicit previous-value read.** The postfix interpreter evaluates the current node buffer against current slot values; it does not secretly retain per-gadget history.
- **Temporal gadgets require explicit previous-value / snapshot columns.** Authors declare `current_col`, `previous_col`, and any derived `delta_col`, `state_col`, or `output_col` in the spec/admission layer.
- **A copy/snapshot band must run before the update band** when the signal is recomputed each tick (velocity, EMA-of-recomputed-input, hysteresis over committed output). The runtime must never infer or secretly allocate temporal memory.
- **Temporal memory is spec/admission/gadget-layer state**, authored in RON and admitted at import. It is **not** semantic `Gadget`/`Personality`/`Memory` state in `simthing-sim`. Persistent columns live in the existing GPU slot/column substrate (`SlotRange`-scoped Layer-3 values), the same way treasury/threat columns already persist — but the *meaning* ("previous velocity", "EMA state") is designer-authored and gadget-admitted, not sim-core semantics.

**Two complementary mechanisms** (from the design note):

1. **In-place persistent column** — a column that survives across ticks and is updated by consume modes (`ScaleTarget`, `AddToTarget`). Used by `Decay` and `BoundedFeedback` when the state column *is* the memory.
2. **Snapshot column pair** — `current_col` overwritten/recomputed each tick; `previous_col` holds a band-copied snapshot. Used by `VelocityMonitor`, EMA-of-recomputed-input, and output-referential hysteresis.

---

## 3. Tier-2 candidate table

| Gadget | Purpose | State columns required | Required order bands | Formula sketch | Admission guardrails | CPU oracle requirements | Status |
|---|---|---|---|---|---|---|---|
| **VelocityMonitor** | Rate-of-change / momentum signal | `current_col`, `previous_col`, `delta_col` (or output) | **Band A:** snapshot `previous ← current` (pre-update); **Band B:** `delta = (current − previous) / dt` | `velocity = current − previous` (optionally `/ dt`) | Columns authored + admitted; snapshot band mandatory; Layer-3 scope by default; finite `dt` if used | Stateful sequence parity; initial-step policy (0 or skip); edge: flat input → 0 delta | **Recommended** |
| **Decay / EMA** | Exponential smoothing / leaky integrator | **Decay:** in-place `state_col` only. **EMA:** `current_col`, `previous_col`, `output_col` | **Decay:** single band `ScaleTarget: col *= α`. **EMA:** **Band A** snapshot if input recomputed; **Band B** blend | `next = prev * decay + input * (1 − decay)`; decay form: `col *= α` | `0 <= decay < 1` (V1); clamp on output if bounded range required; reject unbounded recurrence | EMA: multi-step sequence; Decay: repeated scale; edge: decay=0, decay→1⁻, clamp boundaries | **Recommended** (Decay may alias existing `field_decay`; EMA as gadget macro) |
| **BoundedFeedback** | Stable self-referential accumulator | `state_col` (persistent), optional `input_col` | **Band B:** `next = clamp(prev * decay + input * gain, min, max)` | `clamp(prev * α + input * gain, lo, hi)` | **Mandatory:** `decay < 1` **and/or** explicit clamp; finite gain; reject positive unbounded recurrence | Stateful sequence; clamp saturation; near-boundary steps; reject cases documented | **Recommended** (strict admission) |
| **Hysteresis** | Anti-chatter / sticky threshold crossing | `current_col`, `previous_state_col`, `output_col` | **Band A:** optional snapshot of committed state; **Band B:** threshold select; **Band C:** optional emit | `SELECT(input > upper, 1, SELECT(input < lower, 0, prev_state))` with `lower < upper` | Output bounded `[0,1]` or boolean-like; clear threshold ordering; previous state column required | Sequence crossing upper/lower bands; hold-in-band behavior; edge: simultaneous threshold violation → reject at admission | **Conditional** (implement in 2D if product need clear) |
| **Acceleration** | Second difference of velocity | `current_col`, `previous_col`, `previous_velocity_col`, `velocity_col`, `accel_col` | **Band A:** snapshot current + velocity; **Band B:** `accel = velocity − prev_velocity` | `acceleration = velocity − previous_velocity` | Two prior columns + two snapshot bands; high column/band cost; weak default use-case | Multi-step parity with VelocityMonitor dependency | **Deferred** (unless strong product need) |

**Naming alignment:** The design note and Tier-1 admission reject list use `VelocityMonitor`, `EMA`, `Acceleration`, `Hysteresis`, `Decay`. This review uses **Decay / EMA** as one row (shared blend semantics; Decay is the in-place degenerate case) and adds **BoundedFeedback** as the explicit self-referential stability contract gadget from §7 of the design note.

---

## 4. Required column model

Authors admit columns explicitly. No hidden allocation. No `simthing-sim` semantic state. No dense per-cell temporal columns by default.

| Column role | Meaning | Typical scope |
|---|---|---|
| **`current_col`** | Signal value for this tick (may be recomputed from field reductions / upstream gadgets) | Layer-3 parent / faction / personality slot |
| **`previous_col`** | Snapshot of `current_col` from end of prior tick (or pre-update copy within tick) | Layer-3; one per tracked signal |
| **`delta_col`** | Derived difference output (`current − previous`, optionally scaled) | Layer-3; optional if written in-place to `output_col` |
| **`state_col`** | Persistent accumulator / hysteresis / EMA state that survives across ticks | Layer-3; must declare bound contract if self-referential |
| **`output_col`** | Gadget result consumed by downstream gadget or threshold emit | Layer-3 |

**Scope default:** Temporal memory defaults to **Layer-3 / parent / faction / personality / strategic columns** (`SlotRange`-scoped). **Do not** place temporal-memory columns on dense per-cell `RegionCell` fields by default. Dense per-cell temporal memory requires a **separate gate**: VRAM budget analysis, scenario justification, and explicit product authorization — not part of EML-GADGET-2 V1.

**Column admission rules (proposed):**

- Every temporal column must appear in the gadget descriptor `column_requirements` and pass existing column-bound checks.
- Snapshot pairs (`current_col`, `previous_col`) must be declared together for gadgets that need them.
- Self-referential `state_col` must declare a `bound_contract` (decay and/or clamp) or admission rejects (see §6).

---

## 5. Required band model

Temporal gadgets require **ordered bands** so snapshot precedes compute and same-tick algebraic cycles are forbidden.

```text
Band A — snapshot/copy:  previous_col ← current_col   (ResetTarget copy semantics)
Band B — compute:          EvalEML gadget using current + previous/state
Band C — optional emit:  Threshold + EmitEvent over output_col (existing substrate)
```

**Mechanism sketch (existing substrate, no new opcode assumed):**

- **Band A** uses existing consume modes already present in the accumulator/mapping substrate: `ResetTarget` (copy/replace), and where applicable `ScaleTarget` / `AddToTarget` for in-place accumulate/decay bands.
- **Band B** is a standard `EvalEML` gadget op in its own `OrderBand`, reading declared columns only.
- **Band C** reuses existing threshold/emit machinery; no new emit opcode.

**Copy-band gate (implementation dependency):**

If the current substrate cannot express **generic snapshot/copy bands** cleanly at the spec/admission layer (authored band point + column pair + consume mode), mark **EML-GADGET-2A blocked** pending a separate **generic copy/snapshot primitive gate**. The sandbox/toolkit preserve code references `copy_current_to_previous` and `ScaleTarget`/`ResetTarget`/`AddToTarget` patterns; production admission must not rely on ad-hoc per-scenario wiring.

**Band ordering invariant:** Band A completes before Band B reads `previous_col`. Feedback closes **across ticks**, not within the same tick's algebraic cycle.

---

## 6. Bounded-feedback admission contract

Any self-referential or recurrent temporal gadget must declare a **stability contract**. Enforced at import/admission in `simthing-spec` (designer/importer layer), before GPU execution.

**Binding rule:** If a gadget reads its own previous output/state, it must satisfy **all applicable** constraints:

```text
1. decay is finite
2. abs(decay) < 1  (V1 preference: 0 <= decay < 1)
3. output clamp declared OR formula analytically bounded
4. finite gain / input weights
5. no positive unbounded recurrence
```

Otherwise **reject at import/admission**.

**Examples:**

| Formula | Verdict |
|---|---|
| `next = clamp(prev * 0.8 + input * 0.2, 0, 1)` | **Allowed** — decay < 1 and clamp |
| `next = prev * 0.9 + input * 0.1` with declared `[0,1]` clamp on state column | **Allowed** |
| `next = prev + input` | **Rejected** — unbounded additive recurrence |
| `next = prev * 1.01 + input` | **Rejected** — expanding multiplier |
| `next = prev + positive_feedback` without clamp | **Rejected** |

This contract applies to **BoundedFeedback**, **EMA** (via `decay < 1`), **in-place Decay** (`ScaleTarget` with `α < 1`), and **Hysteresis** (bounded state column). It is the Tier-2 analog of source-cap / `bounded_field_update` rules.

**Relationship to EML-GADGET-1 C-3:** Every admitted Tier-2 gadget still requires CPU oracle + stateful sequence parity tests before admission.

---

## 7. CPU oracle parity plan

Tier-2 parity extends Tier-1: **stateful sequence parity**, not single-step only. Tolerance: `1e-6` for `ExactDeterministic` gadgets (same as Tier-1).

### VelocityMonitor

| Step | Input sequence | Expected delta (initial policy: first delta = 0) |
|---|---|---|
| t0 | 1.0 | 0.0 |
| t1 | 1.5 | 0.5 |
| t2 | 1.25 | −0.25 |

Additional cases: flat sequence → all zeros after t0; large jump; `/ dt` scaling with `dt = 2.0`.

### Decay / EMA

Input sequence `[0, 1, 1, 0]`, `decay = 0.5` (blend form `next = prev * 0.5 + input * 0.5`):

| Step | Expected EMA |
|---|---|
| t0 | 0.0 |
| t1 | 0.5 |
| t2 | 0.75 |
| t3 | 0.375 |

Decay-only: in-place `col *= 0.5` repeated — oracle matches closed form `x * 0.5^n`.

Edge cases: `decay = 0` (tracks input only); `decay` approaching 1; output clamp when declared.

### BoundedFeedback

Single step: `prev = 0.9`, `input = 1.0`, `decay = 0.9`, `gain = 0.5`, clamp `[0, 1]` → raw `0.9*0.9 + 1.0*0.5 = 1.31` → **expected 1.0** (clamp).

Sequence test: drive toward upper bound over multiple ticks; verify no overshoot; verify rejection when admission params omit clamp and `decay >= 1`.

### Hysteresis

Thresholds: `lower = 0.4`, `upper = 0.6`. Input sequence `[0.3, 0.5, 0.7, 0.5, 0.3]` → expected committed state `[0, 0, 1, 1, 0]` (boolean-like).

Edge cases: input exactly on threshold (policy: strict inequality as in sketch); invalid `lower >= upper` → admission reject.

### Acceleration (deferred)

If implemented later: compose oracle from VelocityMonitor sequences; test `[1, 2, 4]` → velocities `[0, 1, 2]` → accelerations `[0, 1, 1]` under consistent initial policy.

**Test harness (implementation handoff):** extend `eval_eml_postfix` or orchestrate multi-band CPU oracle driver in `simthing-spec` tests; parity compares compiled gadget nodes + band schedule against reference oracles — still **no driver/gpu/sim consumption**.

---

## 8. Non-authorizations (this review)

This design review does **not** authorize:

```text
No implementation in this review PR.
No runtime gadget execution.
No chained OrderBand runtime scheduling.
No dense per-cell temporal memory by default.
No new EML opcode.
No WGSL.
No per-gadget GPU kernels.
No simthing-sim Gadget/Personality/Memory semantics.
No CPU-side planner.
No production economy→mapping bridge.
No atlas / M-4A.
No default SimSession mapping wiring.
No Resource Flow E-11 default-on.
No DailyResolutionBoundary primitive.
No day/calendar/pause semantics inside simthing-sim.
No changes to simthing-gpu EvalEML interpreter behavior.
No transcendental (exp/logistic) temporal gadgets.
```

Preserved constitutional posture: V7.7 Mapping ADR, Phase M first-slice vertical proof, SummaryValidity V1 + V1-R1 parked, Queue-Write Scale Hardening V1, Map Residency V1, Boundary Resolution Doctrine, Daily Economy Fixture V1 (example only), Resource Economy Authoring Ergonomics V1, Economy + SEAD Product Fixture V1, EML-GADGET-1 Tier-1 library, `MappingExecutionProfile` default `Disabled`, spec/scenario presence alone does not execute mapping, `simthing-sim` map-free.

---

## 9. Recommended implementation ladder (later — not this PR)

| Slice | Scope | Gate |
|---|---|---|
| **EML-GADGET-2A** | Snapshot/copy band design acceptance + fixture-only proof | Generic copy/snapshot primitive sufficient? |
| **EML-GADGET-2B** | `VelocityMonitor` + `Decay`/`EMA` spec/admission/compiler/oracle | After 2A + this design accepted |
| **EML-GADGET-2C** | `BoundedFeedback` with strict admission | After 2B |
| **EML-GADGET-2D** | `Hysteresis` if product need confirmed | After 2C |
| **Deferred** | `Acceleration`; dense per-cell temporal memory | Separate product + VRAM gate |

---

## 10. Decision questions for Opus/product

1. **Which Tier-2 gadgets are accepted for implementation?** Recommended: VelocityMonitor, Decay/EMA, BoundedFeedback; conditional Hysteresis; defer Acceleration.
2. **Is snapshot/copy band support already generic enough, or does it need a separate primitive gate (2A)?**
3. **Should temporal state be limited to parent/personality columns in V1?** Design recommendation: **yes** — dense per-cell remains separately gated.
4. **Is the bounded-feedback admission contract in §6 sufficient?**
5. **Which implementation slice should land first?** Design recommendation: **2A** (snapshot/copy proof), then **2B**.

---

## 11. References

- Tier-1 acceptance: [`phase_m_eml_gadget_tier1_acceptance_opus_review.md`](phase_m_eml_gadget_tier1_acceptance_opus_review.md)
- Design note §6–§7: [`../workshop/eml_gadget_library_design_note.md`](../workshop/eml_gadget_library_design_note.md)
- Invariants: [`../invariants.md`](../invariants.md) — "EML Gadget Library"
- Tier-1 code (deferred kinds rejected today): `crates/simthing-spec/src/spec/eml_gadget.rs`, `compile/eml_gadget.rs`
- Review test report: [`../tests/phase_m_eml_gadget_tier2_design_review_test_results.md`](../tests/phase_m_eml_gadget_tier2_design_review_test_results.md)
