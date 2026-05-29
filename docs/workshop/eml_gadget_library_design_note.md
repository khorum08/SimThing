# EML Gadget Library — Design Note (Phase M)

**Status:** **EML-GADGET-1 ACCEPTED (Opus/product 2026-05-29, PASS WITH CONDITIONS)** — Tier-1 stateless gadgets (`FieldSampler`, `WeightedAccumulator`, algebraic `SoftStep`) compile in `simthing-spec` with mandatory CPU-oracle parity; R1 composition + R2 node-cap hygiene accepted. Acceptance memo: [`../reviews/phase_m_eml_gadget_tier1_acceptance_opus_review.md`](../reviews/phase_m_eml_gadget_tier1_acceptance_opus_review.md). **EML-GADGET-2** temporal-memory **design ACCEPTED as a gate** (Opus/product 2026-05-29, PASS WITH CONDITIONS) — [`../reviews/phase_m_eml_gadget_tier2_design_acceptance_opus_review.md`](../reviews/phase_m_eml_gadget_tier2_design_acceptance_opus_review.md). Explicit-column temporal memory, Layer-3 default, authored snapshot/copy bands (`Identity`+`ResetTarget` — existing substrate; no new opcode), bounded-feedback admission contract. **Approved ladder:** 2A (snapshot/copy fixture proof) → 2B (`VelocityMonitor` + `Decay`/`EMA`) → 2C (`BoundedFeedback`, strict admission) → 2D (`Hysteresis`, conditional); `Acceleration` + dense per-cell deferred. **EML-GADGET-2A landed (2026-05-29, PASS).**

**Phase M EML-GADGET-2A snapshot/copy fixture proof landed.**
It proves that temporal snapshot/copy bands can be authored using existing substrate primitives: Identity combine + ResetTarget at an earlier OrderBand, copying current_col into previous_col before the update band.
No new EML opcode was added.
No new ConsumeMode was added.
No WGSL or GPU kernel was added.
No runtime gadget execution was introduced.
No temporal gadget implementation landed.
VelocityMonitor + Decay/EMA (2B) and BoundedFeedback (2C) landed. Hysteresis (conditional) and Acceleration remain deferred.
No hidden previous-value read was introduced.

**Phase M EML-GADGET-2B landed.**
It adds VelocityMonitor and Decay/EMA as Tier-2 temporal EML gadgets in simthing-spec, with explicit-column authoring, existing EvalEML node templates, and stateful-sequence CPU oracle parity.
VelocityMonitor computes current_col - previous_col, with optional positive finite dt scaling only if supported by existing opcodes.
Decay/EMA require 0 <= decay < 1 by default and compile to existing arithmetic node templates.

**EML-GADGET-2C BoundedFeedback landed (strict clamp-bounded admission).** BoundedFeedback is the sanctioned bounded recurrent accumulator form: `clamp(previous * decay + input * gain, min, max)`. It requires explicit finite clamp bounds (min < max) and rejects unbounded recurrence at admission.

Hysteresis remains conditional/deferred (2D). Acceleration and dense per-cell temporal memory remain deferred. Runtime gadget-stack execution and true chained OrderBand runtime scheduling remain unauthorized.

No new EML opcode was added.
No new ConsumeMode was added.
No WGSL or GPU kernel was added.
No runtime gadget execution was introduced.
No chained OrderBand runtime scheduling was introduced.
No hidden previous-value read was introduced.
Temporal memory remains explicit-column state.
Temporal memory remains Layer-3 scoped by default; dense per-cell temporal memory remains separately gated.
No simthing-sim Gadget/Personality/Memory semantics were added.
No production economy→mapping bridge was introduced.
No default SimSession mapping wiring was introduced.
No atlas batching landed.
Defaults unchanged.
Defaults unchanged.

**Phase M EML-GADGET-2A R1 hygiene landed.**
It keeps the original 2A snapshot/copy proof intact and cleans the multi-step sequence test/report so the evidence precisely shows previous_col capturing current_col before the update band while current_col advances afterward.
No new EML opcode was added.
No new ConsumeMode was added.
No WGSL or GPU kernel was added.
No runtime gadget execution was introduced.
No temporal gadget implementation landed.
VelocityMonitor + Decay/EMA (2B) and BoundedFeedback (2C) landed. Hysteresis (conditional) and Acceleration remain deferred.
No hidden previous-value read was introduced.

**Phase M EML-GADGET-2B landed.**
It adds VelocityMonitor and Decay/EMA as Tier-2 temporal EML gadgets in simthing-spec, with explicit-column authoring, existing EvalEML node templates, and stateful-sequence CPU oracle parity.
VelocityMonitor computes current_col - previous_col, with optional positive finite dt scaling only if supported by existing opcodes.
Decay/EMA require 0 <= decay < 1 by default and compile to existing arithmetic node templates.

**EML-GADGET-2C BoundedFeedback landed (strict clamp-bounded admission).** BoundedFeedback is the sanctioned bounded recurrent accumulator form: `clamp(previous * decay + input * gain, min, max)`. It requires explicit finite clamp bounds (min < max) and rejects unbounded recurrence at admission.

Hysteresis remains conditional/deferred (2D). Acceleration and dense per-cell temporal memory remain deferred. Runtime gadget-stack execution and true chained OrderBand runtime scheduling remain unauthorized.

No new EML opcode was added.
No new ConsumeMode was added.
No WGSL or GPU kernel was added.
No runtime gadget execution was introduced.
No chained OrderBand runtime scheduling was introduced.
No hidden previous-value read was introduced.
Temporal memory remains explicit-column state.
Temporal memory remains Layer-3 scoped by default; dense per-cell temporal memory remains separately gated.
No simthing-sim Gadget/Personality/Memory semantics were added.
No production economy→mapping bridge was introduced.
No default SimSession mapping wiring was introduced.
No atlas batching landed.
Defaults unchanged.
Temporal memory remains explicit-column state.
Temporal memory remains Layer-3 scoped by default; dense per-cell temporal memory remains separately gated.
No simthing-sim Gadget/Personality/Memory semantics were added.
No production economy→mapping bridge was introduced.
No default SimSession mapping wiring was introduced.
No atlas batching landed.
Defaults unchanged.

*(Note: the #262 parking packet was reverted off master; the acceptance memos are the authoritative artifacts.)*
**Sequencing:** Lands **before Phase M Resource Economy Authoring Ergonomics R2** — R2's
designer-facing authoring must be able to expose and leverage the gadget library, so the library
must exist first.
**Consolidated review artifact:** See `docs/reviews/phase_m_eml_gadget_2abc_temporal_substrate_parking_packet.md` for the full 2A + R1 + 2B + 2C evidence (review/parking only; no further implementation authorized by this packet).
**Layer:** `simthing-spec` (authoring + compiler) over the existing generic `EvalEML` interpreter
in `simthing-gpu`. **No new WGSL. No new GPU kernels.**
**Related:** `docs/reviews/phase_m_first_slice_vertical_proof_acceptance_opus_review.md`,
`docs/reviews/phase_m_product_fixture_chain_acceptance_opus_review.md`,
`docs/invariants.md` ("Mapping (Sparse RegionCell)", "Boundary resolution").

---

## 1. Concept

A **Gadget** is a named, RON-authored, reusable **EML node-template macro** — a small postfix
subgraph over the existing `EvalEML` opcode set — that transforms environment signals (heatmap
reductions, treasury, threat) into urgency/pressure signals. Designers compose **stacks of gadgets**
to author an AI "personality"; the spec compiler flattens/chains them into the existing GPU-resident
`EvalEML` path. Designers never write WGSL.

**Crucial framing (resolves the original proposal's contradiction): gadgets are NOT new WGSL
kernels.** `EvalEML` is already one generic postfix stack-machine interpreter walking a node buffer.
A gadget is authored node-template data, not a shader. This is what delivers "zero added dispatch
overhead" — gadgets add nodes/ops to a path that already runs, not new passes — and it is what keeps
the no-semantic-WGSL constitution intact.

---

## 2. Constitutional placement / guardrails

- **No new semantic WGSL; one generic interpreter stays.** Gadgets compile to postfix node
  templates over the existing opcodes (`LITERAL_F32, SLOT_VALUE, PARAM(dt), ADD, SUB, MUL, NEG, DIV,
  MIN, MAX, CLAMP_BOUNDED, CLAMP_FLOORED, ABS, CMP_*, SELECT, RETURN_TOP`).
- **Registry/composition lives in `simthing-spec`.** `simthing-gpu` keeps only the interpreter and
  the opcode set. The gadget library, RON authoring, flatten-to-nodes compiler, and CPU oracles are
  spec/importer-layer concerns. `simthing-sim` never sees "personality" or "gadget".
- **Opt-in, default-off, designer-authored meaning.** A gadget stack executes only under an explicit
  profile/scenario opt-in. The meaning ("aggression", "desperation") is RON-authored; the substrate
  sees generic columns and ops.
- **Mandatory CPU-oracle parity per gadget (binding admission rule).** No gadget is admitted without
  a bit-exact (or, for soft classes, tolerance-bounded) CPU reference. This matches the existing C-8
  `ExactDeterministic` policy and execution-class taxonomy.
- **Bounded-feedback admission guardrail (new — §7).**

---

## 3. Composition model

Two modes; the compiler chooses per stack:

- **(a) Inline flatten** — a single-gadget stack may expose an executable `InlineFlattenPreview` subject to `MAX_EML_TREE_NODES` per tree. Multi-gadget stacks with `output_col`/`input_col` chaining use `PerGadgetOnly` composition in V1; concatenated postfix without intermediate column wiring is **not** executable. **R2:** the node cap applies per gadget/tree, not to the informational `total_node_count` of a multi-gadget stack.
- **(b) Chained gadget ops (preferred for multi-gadget stacks)** — each gadget is its own `EvalEML`
  op in its own `OrderBand`, writing an **intermediate output column**; the next gadget reads that
  column. This *is* the "stack of gadgets" and it sidesteps the node-budget entirely. **Deferred in EML-GADGET-1/R1** — proven only through manual/orchestrated per-gadget parity tests.

---

## 4. Execution classes

- **`ExactDeterministic` (bit-exact, no transcendental):** FieldSampler, WeightedAccumulator,
  algebraic-sigmoid SoftStep, smoothstep, velocity (`cur−prev`), decay (`ScaleTarget`), EMA. These
  may feed a hard `EmitEvent` threshold directly.
- **`SoftDeterministic`/`FastApproximate`:** only if a true transcendental opcode (logistic `exp`) is
  ever added — a **separate gated substrate decision**. A soft value may **not** feed a hard
  threshold without an A-4 `SoftAggregateGuard` (quantize/hysteresis). The Tier-1 SoftStep avoids
  this by using the bit-exact algebraic form.

---

## 5. Tier 1 — stateless gadgets (land first; no substrate change)

Pure node-templates over current opcodes. `ExactDeterministic`. **Landed in EML-GADGET-1** (`simthing-spec`: registry, RON stack, compiler, CPU oracles, parity tests).

| Gadget | Effect | Op stack (sketch) | Notes |
|---|---|---|---|
| **FieldSampler** | normalize raw field → `[0,1]` | `CLAMP_BOUNDED(DIV(x, cap), 0, 1)` (or `MIN(1, MAX(0, x/cap))`) | divisor = authored cap or existing `source_cap`; scale-invariant input for personalities |
| **WeightedAccumulator** | `Σ(inputᵢ · weightᵢ)` → TotalUrgency | `ADD(MUL(in₁,w₁), MUL(in₂,w₂), …)` | this is essentially `field_urgency`; final node feeding the threshold gate |
| **SoftStep** (sigmoid effect) | non-linear "calm-until-critical" mapping | `0.5 + 0.5·u/(1+\|u\|)`, `u = k·(x−c)` | algebraic sigmoid; `c`=critical point, `k`=steepness = the personality knobs; bit-exact; **feeds a hard threshold directly**. Alt: `smoothstep` (true-flat dead zones); `SELECT(CMP_GE(x,c),hi,lo)` (hard knee) |

---

## 6. Tier 2 — temporal-memory primitive + stateful gadgets (separate gated slice)

**Design review (2026-05-29):** [`../reviews/phase_m_eml_gadget_tier2_temporal_memory_design_review.md`](../reviews/phase_m_eml_gadget_tier2_temporal_memory_design_review.md) — reviews `VelocityMonitor`, `Decay`/EMA, `Acceleration`, `Hysteresis`, and `BoundedFeedback` with explicit-column temporal state, snapshot/copy bands, bounded-feedback admission, and CPU oracle parity plans. **No implementation in that packet.**

Ships in `EML-GADGET-2` (after design acceptance + implementation handoff). The enabling primitive is **generic temporal memory**, not a single gadget.

**Primitive — snapshot/accumulate band:** a generic, semantic-free op that copies/accumulates
column `X → column Y` at a **declared band point** (`ResetTarget` copy / `ScaleTarget` / `AddToTarget`
— consume modes that already exist). Provisioned **per (signal × temporal-semantics)**:
velocity wants a *pre-update* snapshot; EMA/integral want *post-update* accumulation.

**Cost discipline:** these columns + their copy/EML ops are `SlotRange`-scoped to the
personality/faction slots (**Layer 3** — hierarchical, cheap; scales with number of AI agents, not
map cells). **Do not** provision temporal memory on the dense per-cell field by default (that scales
with cells, doubles field VRAM, and full-grid copies every tick — a separate, gated decision).

| Gadget | Effect | Mechanism |
|---|---|---|
| **Decay** | exponential decay of an in-place signal | `ScaleTarget: col *= α` — **already exists** (`field_decay`); the persistent column is its own memory, **no snapshot needed** |
| **VelocityMonitor** | rate-of-change ("desperation"/momentum) | pre-update snapshot `prev ← x`, then `EvalEML: (x − prev)/dt`; bit-exact |
| **Acceleration** | second difference ("accelerating failure") | add a `prev_velocity` column; `vel − prev_vel` |
| **EMA / low-pass** | trend smoothing (react to trends, not noise) | blend with prior: `α·input + (1−α)·prev` (needs snapshot when input is recomputed) |
| **Hysteresis** | sticky commitments; anti-chatter | previous-*committed* column → `SELECT(x>last+band, hi, SELECT(x<last−band, lo, last))`; **satisfies the A-4 soft-aggregate guard** |
| **Desperation (composite)** | level + downward velocity + accel | `WeightedAccumulator` over `(1−norm)`, `MAX(0,−velocity)`, `MAX(0,−accel)` |

**Decay vs last-value (clarification):** in-place decay of a self-persistent signal uses
`ScaleTarget` and needs no snapshot. The last-value/snapshot column is for signals that are
*overwritten/recomputed* each tick (velocity, EMA-of-recomputed-input, change detection) and for
*output-referential* feedback (hysteresis). They are complementary memory mechanisms.

---

## 7. Feedback loops + bounded-feedback admission guardrail (new)

A first-order feedback loop (leaky integrator / IIR — morale, grudge, escalation) is
`s_t = α·s_{t−1} + gain·inputₜ`, built from existing `ScaleTarget` (leak) + `AddToTarget` (drive)
on one persistent column. Loops **close across ticks** (band ordering forbids same-tick algebraic
cycles) — a deterministic, replayable discrete dynamical system. The last-value column enriches this
with output-referential feedback (hysteresis, momentum).

**Binding admission guardrail (designer/RON/importer layer):** a self-referential accumulator column
**must declare a bound** — a decay coefficient `α < 1` **or** a `CLAMP_BOUNDED` on the state column
(or both) — or admission **rejects** it. This is the feedback analog of the source-cap /
`bounded_field_update` rules and prevents divergence (the `raw_additive` blow-up failure mode).
Enforced at import, before the GPU.

---

## 8. Gadget descriptor (spec-side)

```text
GadgetDescriptor {
  name,
  input_cols, weight_cols, output_col,
  node_template_fn,           // emits postfix EmlNode subgraph
  cpu_oracle_fn,              // mandatory bit-exact (or tolerance) reference
  execution_class,           // ExactDeterministic | Soft… | Fast…
  column_requirements,       // e.g. needs a previous-value column
  band_requirements,         // e.g. pre-update snapshot band, output band
  bound_contract,            // for feedback: decay<1 and/or clamp (required if self-referential)
}
```

RON authoring: a "Personality" is an ordered list of gadget instances; the compiler flattens
(mode a) or chains (mode b) into `EvalEML` ops, resolves columns/bands, and runs admission +
oracle parity.

---

## 9. PR ladder (this track)

1. **PR EML-GADGET-1 — Tier-1 stateless gadgets (`simthing-spec`).** Gadget descriptor + registry +
   RON authoring + flatten/chain compiler + `FieldSampler`, `WeightedAccumulator`, `SoftStep`, with
   the **mandatory CPU-oracle parity suite** (`tests/eml_gadget_parity.rs` analog). No GPU/WGSL
   change. Default-off. **Composer 2.5 / Codex.**
2. **PR EML-GADGET-2 — Tier-2 temporal-memory slice.** Generic snapshot/accumulate-band primitive +
   temporal columns (`SlotRange`/Layer-3 scoped); `VelocityMonitor`, `Decay`/EMA, acceleration,
   hysteresis; the **bounded-feedback admission guardrail** (§7). Separate gate. **Composer 2.5.**
3. **Then: Phase M Resource Economy Authoring Ergonomics R2** — builds on the gadget library so the
   designer-facing economy authoring can expose and leverage gadgets. **Sequenced after this track.**

---

## 10. Stop conditions (escalate; do not land)

- No per-gadget WGSL kernel; no new GPU pass per gadget.
- No new EML opcode (incl. transcendental) without a separate, explicit substrate gate.
- No transcendental inside an `ExactDeterministic` gadget (SoftStep uses the algebraic/poly form).
- Temporal-memory columns stay Layer-3 (personality/faction) scoped by default — never dense per-cell
  without a separate gated decision.
- Self-referential feedback must be bounded (decay<1 and/or clamp) or admission rejects it.
- Default-off; opt-in only; no default `SimSession` mapping wiring; `simthing-sim` stays map-free.
- Every gadget ships with a CPU oracle; none is enshrined without one.
