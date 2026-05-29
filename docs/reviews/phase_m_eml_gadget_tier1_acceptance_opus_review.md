# Opus/Product Acceptance Review — Phase M EML-GADGET-1 (Tier-1 Stateless Gadget Library)

**Date:** 2026-05-29
**Authority:** Opus 4.8, mapping/SEAD design authority under human delegation. Guardrail authority
extends to the designer-facing studio/importer layer and the scenario definition stage — not the sim
or boundary layer.
**Decision type:** Acceptance review — **not** an implementation handoff. No code changed.
**Reviews:** the landed EML-GADGET-1 / R1 / R2 code and reports
(`docs/tests/phase_m_eml_gadget_tier1_test_results.md`,
`..._r1_hygiene_test_results.md`, `..._r2_node_cap_test_results.md`); design note
`docs/workshop/eml_gadget_library_design_note.md`.
**Builds on:** `docs/reviews/phase_m_product_fixture_chain_acceptance_opus_review.md`.

> **Reconciliation note (read first).** PR #262 (the EML-GADGET-1 parking packet) was **reverted off
> master** (`87665e0`). That revert was **docs-only** — it removed the parking *review packet* and
> *parking test report* the handoff names as "primary," but left the gadget **code** and the three
> substantive test reports (`test_results`, `r1_hygiene`, `r2_node_cap`) fully intact and wired
> (`lib.rs`, `ron.rs`, `compile/mod.rs`, `spec/mod.rs`). This memo is therefore the authoritative
> review artifact; it reviews the present code + reports directly and does not depend on the reverted
> packet. The reverted parking report is not required for acceptance (the three landed reports carry
> the evidence); restore it only if the team wants the parking snapshot retained.

---

## 1. Executive verdict

**PASS WITH CONDITIONS.** Phase M EML-GADGET-1 is **accepted as the Tier-1 stateless gadget
library.** `FieldSampler`, `WeightedAccumulator`, and the algebraic `SoftStep` are accepted as
`simthing-spec` node-template macros over the existing `EvalEML` opcode set, with mandatory
CPU-oracle parity. The R1 composition model (per-gadget executable; single-gadget
`InlineFlattenPreview` executable; multi-gadget `PerGadgetOnly` with chained OrderBand scheduling
deferred) and the R2 node-cap hygiene (cap per executable tree, not the informational
`PerGadgetOnly` stack total) are accepted. Runtime gadget execution, chained OrderBand scheduling,
temporal memory, new opcodes, WGSL, `simthing-sim` semantics, the production economy→mapping bridge,
default mapping wiring, and atlas remain unauthorized.

---

## 2. Evidence reviewed

The three landed reports + design note, and the code surfaces below read directly (not taken on the
reports' word):

- **`crates/simthing-spec/src/spec/eml_gadget.rs`** — `EmlGadgetStackSpec` + tagged
  `EmlGadgetInstanceSpec` (FieldSampler / SoftStep / WeightedAccumulator). Narrow RON authoring
  surface: column bindings + params, optional `output_col`. No runtime, no WGSL.
- **`crates/simthing-spec/src/compile/eml_gadget.rs`** — registry, per-gadget postfix node
  templates over existing opcodes only (`LITERAL_F32, SLOT_VALUE, ADD, SUB, MUL, DIV, ABS,
  CLAMP_BOUNDED, RETURN_TOP`), CPU oracles, a test postfix evaluator, `EmlGadgetCompositionPlan`,
  and admission. **Verified:** deferred Tier-2 kinds (`VelocityMonitor, EMA, Acceleration,
  Hysteresis, Decay`) are rejected at admission; column bounds, finite/`>0` params, non-empty
  WeightedAccumulator inputs, and `input_count == weight_count` are enforced; execution class is
  `ExactDeterministic`.
- **R2 node-cap policy (`apply_stack_node_cap_policy`)** — verified in code: `InlineFlattenPreview`
  (single executable tree) **rejects** over `MAX_EML_TREE_NODES` (32); `PerGadgetOnly` multi-gadget
  stacks only emit a `stack_total_exceeds_inline_cap` **diagnostic** and admit; per-gadget templates
  cap separately. Matches the R2 claim exactly.
- **SoftStep is algebraic, not transcendental** — `oracle_soft_step` and the postfix evaluator
  implement `0.5 + 0.5·u/(1+|u|)` with `ADD/SUB/MUL/DIV/ABS` only; no `EXP`/logistic anywhere.
  Stays `ExactDeterministic` and may feed a hard threshold directly.

**Independent verification run (this review, this machine):**

| Suite | Result |
|---|---|
| `eml_gadget_tier1` | **14/14** (incl. all three oracle-parity tests, `no_runtime_flatten_preview_consumption`, R2 node-cap cases) |
| `region_field_spec_admission` | **11/11** |
| `resource_economy_authoring_preview` | **8/8** |

---

## 3. Acceptance decision (answers to the five questions)

1. **Tier-1 gadget library — ACCEPT.** FieldSampler / WeightedAccumulator / SoftStep land cleanly as
   bit-exact `ExactDeterministic` node-template macros with oracle parity and bounded admission.
2. **Spec-layer placement — PASS.** Registry/composition live in `simthing-spec`; gadgets are node
   templates over existing opcodes; `simthing-gpu` is unchanged (one generic interpreter);
   `simthing-sim` has no Gadget/Personality types.
3. **CPU-oracle parity — PASS.** Every admitted Tier-1 gadget has an oracle; compiled postfix
   evaluation matches within `1e-6` (exact at FieldSampler boundaries). SoftStep is
   `ExactDeterministic` precisely because it is algebraic, not transcendental.
4. **Composition model (R1/R2) — ACCEPT.** Per-gadget templates executable; single-gadget inline
   preview executable; multi-gadget composition is `PerGadgetOnly`; chained OrderBand runtime
   scheduling deferred; node cap applies per executable tree, not to the informational stack total.
5. **Binding non-authorizations — ACCEPT (kept binding).** See §5.

---

## 4. Conditions

- **C-1 (preview ≠ runtime).** `InlineFlattenPreview { executable: true }` is a *preview* property; a
  gadget tree must **not** be consumed as a production runtime execution path without the separate
  runtime-wiring/EML-GADGET-2 gate. The `no_runtime_flatten_preview_consumption` test guards this —
  it stays a binding expectation; no driver/gpu/sim code may consume `CompiledEmlGadgetStack`,
  `composition` nodes, or the flatten preview as execution in this slice.
- **C-2 (PerGadgetOnly is the only multi-gadget composition).** Multi-gadget stacks stay
  `PerGadgetOnly` until *true intermediate-column wiring* (chained OrderBand scheduling) is proven
  under its own gate. No inline multi-gadget flattening that implies executable cross-gadget output.
- **C-3 (oracle-per-gadget is binding, forever).** No gadget — Tier 1 or any future tier — is
  admitted without a CPU oracle and parity test. Codified in invariants this pass.
- **C-4 (housekeeping).** The reverted #262 parking packet is not restored; this memo supersedes it.
  If the parking snapshot is wanted for the record, restore it separately — it does not gate
  acceptance.

---

## 5. Binding guardrails / non-authorizations (kept binding)

```text
No EML-GADGET-2 yet (temporal memory: velocity, EMA, acceleration, hysteresis, bounded feedback).
No chained OrderBand runtime scheduling; no runtime gadget-stack execution.
No true inline multi-gadget flattening unless intermediate column wiring is separately proven.
No new EML opcodes; no exp/logistic/transcendental SoftStep.
No semantic WGSL; no per-gadget GPU kernel; simthing-gpu stays the one generic interpreter.
No Gadget/Personality types in simthing-sim.
No production economy→mapping bridge; economy→SEAD stays tests/support fixture-only.
No default SimSession mapping wiring; MappingExecutionProfile default Disabled.
No Resource Flow E-11 default-on.
No atlas / M-4A.
```

Authoritative home: `docs/invariants.md` — new "EML Gadget Library" rows + existing "Mapping
(Sparse RegionCell)" and "Boundary resolution" rows.

---

## 6. Recommended next implementation handoff

**A is accepted/parked now** (this memo). Next:

- **C — EML-GADGET-2 temporal-memory *design review* (recommended).** Review the snapshot/accumulate-
  band primitive + temporal columns (`VelocityMonitor`, `Decay`/EMA, acceleration, hysteresis) and
  the bounded-feedback admission guardrail **before implementation**. The design is already sketched
  in the gadget design-note §6–§7; this is the design-review gate ahead of building it.
- **B — designer preview UX for gadget stacks** (spec/admission/reporting only) is an acceptable
  parallel/alternative; no runtime coupling.

With EML-GADGET-1 accepted, **Resource Economy Authoring Ergonomics R2 (option D) is unblocked** —
but only if it *consumes the gadget preview/admission surfaces without runtime coupling* (no gadget
execution, no chained scheduling). It is not the recommended immediate next step; B or C is.

**Not now: E (mapping scale / M-4 atlas)** — only after a named multi-theater scenario, an approved
VRAM budget, and a §11-gate-passing M-4 PR.

---

## 7. Stop conditions for the next handoff (escalate; do not land)

Whichever of B / C / D is next must not introduce any of the §5 non-authorizations, and specifically
must not:
- consume the gadget flatten preview or `CompiledEmlGadgetStack` as a runtime execution path;
- add chained OrderBand runtime scheduling or any runtime gadget-stack execution outside a dedicated,
  separately-gated slice;
- add a new EML opcode (incl. a transcendental for a "real" sigmoid) without its own substrate gate;
- add temporal memory / feedback **without** the bounded-feedback admission guardrail (self-
  referential accumulator must declare decay<1 and/or a clamp, or admission rejects);
- place temporal-memory columns on the dense per-cell field by default (Layer-3/personality scope
  only);
- add Gadget/Personality semantics to `simthing-sim`, semantic WGSL, default mapping wiring, a
  production economy→mapping bridge, or Resource-Flow default-on.

For **C (EML-GADGET-2 design review):** it is a *design* gate — no implementation lands from it; the
output is an accepted design + the bounded-feedback guardrail spec, then a separate implementation PR.

---

## 8. Doc / ADR / invariant updates made alongside this memo

- **New:** this memo (authoritative review artifact, replacing the reverted parking packet).
- **`docs/invariants.md`** — new **"EML Gadget Library"** binding rows: spec-layer node-template
  macros over existing opcodes (no new WGSL/opcode/kernel); mandatory CPU-oracle parity per gadget;
  multi-gadget composition is `PerGadgetOnly` (inline preview ≠ runtime) until intermediate wiring is
  separately gated; `simthing-sim` has no Gadget/Personality types.
- **`docs/workshop/eml_gadget_library_design_note.md`** — status advanced to
  **EML-GADGET-1 ACCEPTED**; reconciliation note on the #262 revert.
- **`docs/workshop/mapping_current_guidance.md`**, **`docs/workshop/workshop_current_state.md`**,
  **`docs/accumulator_op_v2_production_plan.md`**, **`docs/todo.md`** — status → ACCEPTED; next step
  = EML-GADGET-2 design review (C) or preview UX (B); R2 unblocked-but-no-runtime-coupling; not atlas.
- **`docs/worklog.md`** — dated 2026-05-29 acceptance entry (incl. the #262 revert reconciliation).

All updates are decision/classification only. No production code changed; `simthing-gpu` stays the
generic interpreter; `simthing-sim` map-free; `MappingExecutionProfile` default `Disabled`;
Resource Flow E-11 default-off; `request_atlas_batching` rejected at admission.
