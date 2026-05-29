# Phase M EML-GADGET-1 — Tier-1 Stateless Gadget Library — Review / Parking Packet

> **Audience:** Opus / product review; future agents  
> **Status:** **Parked for review** (2026-05-29). EML-GADGET-1 + R1 + R2 landed; acceptance memo pending.  
> **Date:** 2026-05-29  
> **Master baseline at parking:** `17be55c449a1edfbf74d1798154840cebd38ca3d` (EML-GADGET-1 R2 merge)

---

## 1. Executive verdict

**Phase M EML-GADGET-1 is parked for review.**

It implements Tier-1 stateless EML gadgets in `simthing-spec` as RON-authored node-template macros over the existing EvalEML opcode set.

The landed gadgets are **FieldSampler**, **WeightedAccumulator**, and algebraic **SoftStep**.

Each gadget compiles to existing EvalEML nodes and has CPU-oracle parity coverage.

No WGSL, GPU kernel, EML opcode, `simthing-gpu` runtime change, or `simthing-sim` semantic was introduced.

Phase M EML-GADGET-1 parking packet landed.
The repo now parks the Tier-1 stateless EML gadget library for Opus/product review: FieldSampler, WeightedAccumulator, and algebraic SoftStep live in simthing-spec as RON-authored node-template macros over existing EvalEML opcodes, with CPU-oracle parity.
R1 clarified composition semantics and removed the misleading executable multi-gadget flatten surface.
R2 corrected node-cap semantics so MAX_EML_TREE_NODES applies to each executable gadget/tree, while PerGadgetOnly multi-gadget totals may exceed the single-tree cap with diagnostics.
No runtime gadget execution was introduced.
No chained OrderBand runtime scheduling was introduced.
No temporal memory was introduced.
No new EML opcode was added.
No WGSL or GPU kernel was added.
No runtime economy behavior changed.
No production economy→mapping bridge was introduced.
No generic boundary-output packet was introduced.
No DailyResolutionBoundary primitive was introduced.
No day/calendar/pause semantics were added to simthing-sim.
No Resource Flow default changed.
No CPU-side economy executor, urgency computation, or AI planner was introduced.
No default SimSession mapping wiring was introduced.
No atlas batching landed.
simthing-sim remains map-free.
Defaults unchanged.

---

## 2. Landed sequence

### EML-GADGET-1

Tier-1 gadget registry, descriptor/spec shape, RON authoring, compiler, CPU oracles, parity tests.

- Merge PR #259 (`fec7eb2`)
- Report: [`../tests/phase_m_eml_gadget_tier1_test_results.md`](../tests/phase_m_eml_gadget_tier1_test_results.md)

### EML-GADGET-1 R1

Removed misleading executable `flattened_nodes` surface.

Added `EmlGadgetCompositionPlan`.

Single-gadget flatten preview may be executable.

Multi-gadget stacks are `PerGadgetOnly` unless true intermediate wiring is later proven.

- Merge PR #260 (`4a04a1d`)
- Report: [`../tests/phase_m_eml_gadget_tier1_r1_hygiene_test_results.md`](../tests/phase_m_eml_gadget_tier1_r1_hygiene_test_results.md)

### EML-GADGET-1 R2

Corrected node-cap semantics.

`MAX_EML_TREE_NODES` applies to each executable gadget/tree.

`PerGadgetOnly` multi-gadget stacks may exceed the single-tree total with diagnostics.

- Merge PR #261 (`17be55c`)
- Report: [`../tests/phase_m_eml_gadget_tier1_r2_node_cap_test_results.md`](../tests/phase_m_eml_gadget_tier1_r2_node_cap_test_results.md)

---

## 3. Tier-1 gadget table

| Gadget | Formula | Class | Purpose | Admission |
|---|---|---|---|---|
| **FieldSampler** | `clamp(input / cap, 0, 1)` | ExactDeterministic | Normalize raw field/signal value to `[0, 1]` | `cap` finite and > 0; columns in bounds |
| **WeightedAccumulator** | `sum(input_i * weight_i)` | ExactDeterministic | Reusable `field_urgency`-style weighted pressure/urgency composition | Non-empty inputs; input count == weight count; columns in bounds |
| **SoftStep** | `0.5 + 0.5 * u / (1 + abs(u))`, `u = steepness * (x - center)` | ExactDeterministic | Algebraic sigmoid-like nonlinear response that may feed hard thresholds | `center` finite; `steepness` finite and > 0; columns in bounds; **not** exp/logistic/transcendental |

**Deferred (rejected at admission):** VelocityMonitor, EMA, Acceleration, Hysteresis, Decay (gadget form).

---

## 4. Evidence table

### EML-GADGET-1 track reports

| Report | Purpose | Core result | Status |
|---|---|---|---|
| [`../tests/phase_m_eml_gadget_tier1_test_results.md`](../tests/phase_m_eml_gadget_tier1_test_results.md) | Tier-1 gadget library landing | Registry, compiler, RON stack, CPU oracles; 10/10 parity tests | PASS |
| [`../tests/phase_m_eml_gadget_tier1_r1_hygiene_test_results.md`](../tests/phase_m_eml_gadget_tier1_r1_hygiene_test_results.md) | R1 flatten semantics hygiene | `EmlGadgetCompositionPlan`; no executable multi-gadget flatten; 12/12 tests | PASS |
| [`../tests/phase_m_eml_gadget_tier1_r2_node_cap_test_results.md`](../tests/phase_m_eml_gadget_tier1_r2_node_cap_test_results.md) | R2 per-gadget node-cap hygiene | Cap per tree not stack total; over-total admits with diagnostic; 14/14 tests | PASS |
| [`../tests/phase_m_eml_gadget_tier1_parking_test_results.md`](../tests/phase_m_eml_gadget_tier1_parking_test_results.md) | Parking packet verification | Docs-only; targeted regressions green | PASS |

### Related regressions (unchanged / green at parking)

| Report | Purpose | Core result | Status |
|---|---|---|---|
| [`../tests/phase_m_resource_economy_authoring_ergonomics_test_results.md`](../tests/phase_m_resource_economy_authoring_ergonomics_test_results.md) | Resource economy authoring ergonomics | Preview/diagnostics; no gadget runtime coupling | PASS |
| [`../tests/phase_m_economy_sead_product_fixture_test_results.md`](../tests/phase_m_economy_sead_product_fixture_test_results.md) | Economy + SEAD product fixture | Fixture orchestration only; no gadget stack consumption | PASS (6/6) |
| [`../tests/phase_m_first_slice_runtime_test_results.md`](../tests/phase_m_first_slice_runtime_test_results.md) | First-slice runtime | GPU-resident SEAD chain; no gadget wiring | PASS |
| [`../tests/phase_m_product_fixture_chain_parking_test_results.md`](../tests/phase_m_product_fixture_chain_parking_test_results.md) | Product-fixture chain parking | Economy→SEAD fixture-only; accepted chain context | PASS |

---

## 5. Code surfaces

**Spec / admission / compiler / test only:**

| Path | Role |
|---|---|
| `crates/simthing-spec/src/spec/eml_gadget.rs` | RON `EmlGadgetStackSpec`, tagged `EmlGadgetInstanceSpec` |
| `crates/simthing-spec/src/compile/eml_gadget.rs` | Registry, compiler, CPU oracles, `EmlGadgetCompositionPlan`, preview report |
| `crates/simthing-spec/src/ron.rs` | `deserialize_eml_gadget_stack_ron` |
| `crates/simthing-spec/src/error.rs` | `SpecError::EmlGadgetAdmission` |
| `crates/simthing-spec/src/lib.rs` | Public exports |
| `crates/simthing-spec/src/compile/mod.rs` | Compile module exports |
| `crates/simthing-spec/src/spec/mod.rs` | Spec module exports |
| `crates/simthing-spec/tests/eml_gadget_tier1.rs` | Tier-1 + R1 + R2 test suite (14 tests) |

**Explicit non-surfaces:**

- These are spec/admission/compiler/test surfaces.
- They are **not** runtime scheduling surfaces.
- No driver production pass consumes `CompiledEmlGadgetStack`.
- No `simthing-gpu` code knows about `EmlGadget`.
- No `simthing-sim` code knows about `EmlGadget` or `Personality`.

---

## 6. What this proves

- Tier-1 stateless gadgets can be expressed as reusable EvalEML node-template macros.
- RON gadget stacks can be parsed and admitted.
- Each Tier-1 gadget has CPU oracle parity.
- SoftStep is algebraic and ExactDeterministic, not transcendental.
- Per-gadget templates are executable.
- Single-gadget inline preview is executable.
- Multi-gadget stack composition is explicitly `PerGadgetOnly` unless future wiring proves otherwise.
- Node cap semantics are correct: executable gadget/tree cap is enforced per tree, not by informational multi-gadget total.
- No new WGSL, opcode, GPU kernel, or `simthing-sim` semantic is needed.

---

## 7. What this does not prove

- Does not authorize EML-GADGET-2.
- Does not authorize temporal memory.
- Does not authorize velocity, EMA, acceleration, hysteresis, or bounded feedback.
- Does not authorize chained OrderBand runtime scheduling.
- Does not authorize runtime gadget stack execution.
- Does not authorize true inline multi-gadget flattening with intermediate column wiring.
- Does not authorize new EML opcodes.
- Does not authorize exp/logistic/transcendental SoftStep.
- Does not authorize semantic WGSL.
- Does not authorize production economy→mapping bridge.
- Does not authorize moving economy→SEAD coupling out of `tests/support`.
- Does not authorize default SimSession mapping wiring.
- Does not authorize atlas/M-4A.

---

## 8. Binding guardrails

- Do not add per-gadget WGSL.
- Do not add new GPU kernels.
- Do not add new EML opcodes without separate substrate gate.
- Do not use transcendental math in ExactDeterministic gadgets.
- Do not claim multi-gadget flatten is executable unless intermediate wiring is proven by tests.
- Do not execute gadget stacks at runtime until a separate runtime scheduling gate lands.
- Do not provision temporal memory on dense per-cell fields by default.
- Every gadget must ship with CPU oracle parity.

Related binding rows: [`../invariants.md`](../invariants.md) (Mapping, Boundary resolution, economy→mapping fixture-only).

---

## 9. Recommended next options

| Option | Description |
|---|---|
| **A — Opus/product acceptance** | Accept EML-GADGET-1 as complete Tier-1 spec/compiler proof. **Recommended first.** |
| **B — Designer preview UX** | Spec/admission/reporting preview for gadget stacks; no runtime coupling. |
| **C — EML-GADGET-2 design review** | Temporal-memory slice design review before implementation. |
| **D — Authoring Ergonomics R2** | After EML-GADGET-1 acceptance; consume gadget preview surfaces without runtime coupling. |
| **E — Mapping scale / M-4 atlas** | Only after separate named scenario, VRAM budget, and gate-passing PR. |

**Sequencing recommendation:**

1. **A first.**
2. Then **B** or **C**.
3. Do **not** implement EML-GADGET-2 until Opus/product accepts this parking packet.
4. Do **not** resume Resource Economy Authoring Ergonomics R2 until EML-GADGET-1 is accepted.

---

## 10. Design references

- [`../workshop/eml_gadget_library_design_note.md`](../workshop/eml_gadget_library_design_note.md)
- [`../accumulator_op_v2_production_plan.md`](../accumulator_op_v2_production_plan.md) (PR M-eml-gadget-library)
- [`../workshop/mapping_current_guidance.md`](../workshop/mapping_current_guidance.md)
