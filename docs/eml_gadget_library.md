# EML Gadget Library

**Status:** durable constitutional payload for `eml-extension-ladder`, moved out of the workshop layer during OC-K-EML-OPCODE-GATE-0. The historical workshop note remains represented here only where it is still load-bearing: gadget concept, constitutional guardrails, extension ladder, sanctioned catalogue, and stop conditions.

**Layer:** `simthing-spec` authoring/compiler over the existing generic `EvalEML` interpreter in `simthing-gpu`. **No new semantic WGSL. No new GPU kernels.**

---

## 1. Concept

A **Gadget** is a named, RON-authored, reusable **EML node-template macro** — a small postfix subgraph over the existing `EvalEML` opcode set — that transforms environment signals (heatmap reductions, treasury, threat) into urgency/pressure signals. Designers compose stacks of gadgets to author an AI profile; the spec compiler flattens/chains them into the existing GPU-resident `EvalEML` path. Designers never write WGSL.

Gadgets are not new WGSL kernels. `EvalEML` is already one generic postfix stack-machine interpreter walking a node buffer. A gadget is authored node-template data, not a shader. Gadgets add nodes/ops to a path that already runs, not new passes, and keep the no-semantic-WGSL constitution intact.

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

### 2.1 Extension ladder (OC-K-EML-OPCODE-GATE-0 / `eml-extension-ladder`)

**Anchor B** — Andrzej Odrzywołek, *All elementary functions from a single operator*
(arXiv:2603.21852); core design §1.1. One fixed generic interpreter (`EvalEML`) encodes any
scripted interaction as postfix data — never a new semantic kernel, opcode, or subsystem.

**Core §4.1 three-step extension ladder** (agents hunting “how do I add branching?” land here
before any WGSL path):

1. **EML gadget tree** over the existing interpreter (default; branchless column dataflow).
2. **New generic primitive** only by Tier-2 gate with bit-exact CPU-oracle parity
   (`CpuOracleParityProof` + `OpcodeRegistrationGate`); vocabulary stays closed until DA-scoped
   expansion lands the opcode in the closed set.
3. **Semantic / scenario-specific op is never admissible**
   (`SemanticOpcodeRegistration` hard-rejects at the gate).

**Gadget catalogue** (sanctioned macros; no new WGSL): FieldSampler, WeightedAccumulator,
SoftStep, Decay / EMA, VelocityMonitor, Acceleration, BoundedFeedback, Hysteresis, Desperation
(composer over the above; not a new opcode).

**Worked SoftStep policy conditional** (branchless branching as column data):

```text
input columns → SoftStep predicate → weighted branch A/B contribution → accumulator column
out = B + softstep(x) * (A - B)
```

No if/else WGSL. No semantic opcode. No scenario-specific combine. Kernel proof:
`SoftStepPolicyConditional` / `oc_k_eml_opcode_gate_0_softstep_policy_conditional_compiles_as_gadget`.

Closed registration door: `OpcodeRegistrationGate` + closed opcode/combine vocabulary on
`EmlGpuProgramTable::upload_trees` and packed combine admission.

---

## 3. Composition model

Two modes; the compiler chooses per stack:

- **Inline flatten** — a single-gadget stack may expose an executable `InlineFlattenPreview` subject to `MAX_EML_TREE_NODES` per tree. Multi-gadget stacks with `output_col`/`input_col` chaining use `PerGadgetOnly` composition in V1; concatenated postfix without intermediate column wiring is not executable.
- **Chained gadget ops** — each gadget is its own `EvalEML` op in its own `OrderBand`, writing an intermediate output column; the next gadget reads that column. This is the stack-of-gadgets model and avoids semantic WGSL.

---

## 4. Execution classes

- **`ExactDeterministic`**: FieldSampler, WeightedAccumulator, algebraic-sigmoid SoftStep, smoothstep, velocity, decay, EMA. These may feed hard thresholds when their parity class permits.
- **`SoftDeterministic` / `FastApproximate`**: only if a true transcendental opcode is separately admitted. Soft values may not feed hard thresholds without the applicable soft aggregate guard.

---

## 5. Tier 1 — stateless gadgets

Pure node templates over current opcodes. `ExactDeterministic`.

| Gadget | Effect | Op stack sketch | Notes |
|---|---|---|---|
| FieldSampler | normalize raw field to `[0,1]` | `CLAMP_BOUNDED(DIV(x, cap), 0, 1)` | scale-invariant input |
| WeightedAccumulator | weighted sum to urgency | `ADD(MUL(in₁,w₁), MUL(in₂,w₂), …)` | final node feeding threshold gate |
| SoftStep | branchless conditional / sigmoid-like pressure | `0.5 + 0.5*u/(1+abs(u))` | bit-exact algebraic form |

---

## 6. Tier 2 — temporal memory and stateful gadgets

Temporal gadgets use explicit columns and declared band points. Snapshot/copy bands, VelocityMonitor, Decay/EMA, BoundedFeedback, Hysteresis, and Acceleration remain generic EML/dataflow constructs. Dense per-cell temporal memory and hidden previous-value reads remain separately gated.

---

## 7. Feedback loops + bounded-feedback admission guardrail

A first-order feedback loop is `s_t = α*s_{t-1} + gain*input_t`, built from existing target-scaling and additive update primitives over one persistent column. Loops close across ticks; band ordering forbids same-tick algebraic cycles.

A self-referential accumulator column must declare a bound — a decay coefficient `α < 1` or a `CLAMP_BOUNDED` on the state column, or both — or admission rejects it before GPU upload.

---

## 8. Gadget descriptor

```text
GadgetDescriptor {
  name,
  input_cols, weight_cols, output_col,
  node_template_fn,
  cpu_oracle_fn,
  execution_class,
  column_requirements,
  band_requirements,
  bound_contract,
}
```

RON authoring defines an ordered list of gadget instances; the compiler resolves columns/bands, emits EvalEML nodes, and runs admission plus oracle parity.

---

## 9. Stop conditions

- No per-gadget WGSL kernel.
- No new GPU pass per gadget.
- No new EML opcode, including transcendental, without a separate explicit substrate gate.
- No transcendental inside an `ExactDeterministic` gadget.
- Temporal-memory columns stay Layer-3 scoped by default.
- Self-referential feedback must be bounded or admission rejects it.
- Default-off; opt-in only.
- `simthing-sim` remains semantic-free.
- Every gadget ships with a CPU oracle.
