# Phase M-5-gradient: Gradient Operator (single-target) + L3 Strategic Pressure Composition Pattern

**Status:** **M-5A/B/C/D-gradient landed (2026-05-29).** Single-target `Gradient { axis, output_col }`
admission + generic per-direction stencil weights (M-5A); L3 composition RON fixture (M-5B+R1);
need/routing signal fixture (M-5C); frame/scenario-level gradient strict-sink admission (M-5D).
Dual-output `GradientXY` remains deferred (§6).
**Track naming:** `M-5-gradient` / `M-5A-gradient` / `M-5B-gradient` — **distinct from the existing
`M-5` source-identity/source-mask track**. Do not call this bare `M-5A`.
**Sequencing:** Candidate after the EML-GADGET-2 ladder, alongside Resource Economy Authoring
Ergonomics R2.
**Layer split:**
- `StructuredFieldStencilOp` kernel: generic **per-direction weights** (`weight_north/south/east/west`),
  **single output column** — new *generic* WGSL under the revised guardrail (§1). No change to the
  single-output write contract.
- `RegionFieldSpec` admission: a single-target `Gradient { axis, output_col }` operator variant —
  the *meaning* ("gradient extraction") is pinned at the designer/spec layer.
- RON composition pattern: designer-authored multi-field → L3 WeightedAccumulator + EMA — **no new
  substrate**.

---

## 0. This is generic field-calculus tooling, not an AI feature

The gradient operator and the EML gadget library are **generic field-calculus primitives** — the
"EML-MGSL" tooling (EML gadgets + generic shader/stencil extensions) the design authority directed
us to pursue **for lateral benefit beyond the AI subsystem.** A gradient over *any* authored field
yields a generic "flow-toward-need" signal. The substrate never knows what the field *means*.

| Authored field (RON/spec layer) | Gradient gives you | Consumer |
|---|---|---|
| Threat / pressure | direction of greatest danger | SEAD AI steering (the original motivation) |
| **Unmet demand / scarcity** | direction resources should flow | **resource routing by need configuration** |
| **Job openings − available workers** | direction of opportunity | **migrant/labor dispatch** |
| Price differential | arbitrage direction | trade-flow routing |
| Supply reach / logistics cost | gradient of reachability | reinforcement/supply routing |

All of these use the **same** `Gradient` operator → `SlotRange` reduction → `EvalEML`
(WeightedAccumulator/EMA) → `Threshold`/`EmitEvent` chain. The shader sees floats; the meaning is
authored at the spec layer. This lateral generality is *why* the gradient belongs in the generic
substrate and not in an AI module — and it is the strongest argument that the per-direction-weight
kernel extension is constitutionally clean.

---

## 1. The revised WGSL guardrail

The SimThing constitution forbids **semantic WGSL** — shaders that embed map/faction/AI/gameplay
meaning. It does **not**, and never did, forbid **generic kernel extensions** — new parameters that
remain purely mathematical (column indices, scalar weights, bounds), with all meaning pinned at the
designer/spec admission layer.

**Old framing (too broad):** "No new WGSL."

**Correct framing (revised, Opus 2026-05-29, authorized by product):**
```
No new *semantic* WGSL.
Generic shader extensions — new parameters that carry no map/faction/AI semantics,
  with meaning pinned entirely at the designer/spec admission layer — are admissible
  when they satisfy all of:
  1. Generic: the shader sees only floats/indices, not "threat", "faction", "gradient"
  2. Bounded: no unbounded reads or writes; existing buffer-bound invariants hold
  3. Opt-in: behind a new operator variant / explicit spec field
  4. Semantic-free in the shader: no semantic branching on game concepts
  5. CPU-oracle parity: the new kernel path has a bit-exact CPU reference
  6. No simthing-sim awareness: the sim still sees only flat columns and AccumulatorOp
```

This matches the V7.6 greenfield admission criterion: new GPU/EML work is allowed when it is a
generic primitive (not scenario semantics), opt-in (no production default changes), does not impair
Resource Flow / E-11B / Phase T / `simthing-sim`, and is covered by regression tests with documented
admission constraints.

**Where meaning is raised to:** the designer/RON/spec admission layer. The shader receives four
generic per-direction weights. The spec compiler maps a `Gradient { axis }` operator variant to a
specific weight pattern. The GPU never knows the word "gradient."

---

## 2. Single-target per-direction weights (preserves the existing kernel contract)

The current `structured_field_stencil.wgsl` already carries `variant`, `directed_mode`, and `_pad`
fields — it was **designed for extension** — and writes exactly **one** `target_col`:

```wgsl
output_values[base + params.target_col] = next;   // single output, unchanged
```

M-5A-gradient replaces the single scalar `gamma_neighbor` with four independent per-direction
weights and keeps the **single-output** write:

```wgsl
next = alpha_self_decay * center
     + weight_north * north
     + weight_south * south
     + weight_east  * east
     + weight_west  * west
// → output_values[base + target_col] = next   (one column, contract unchanged)
```

This is generic linear combination over flat buffers. No semantic knowledge; no widening of the
output contract; no ping-pong/layout review needed.

### A gradient field is a single-target field with `alpha_self = 0`

A single-axis gradient is just these weights with no center term:

```
GradientX field:  alpha_self=0, weight_east=+0.5, weight_west=−0.5, weight_north=0, weight_south=0
                  → output_col = (east − west) / 2
GradientY field:  alpha_self=0, weight_north=−0.5, weight_south=+0.5, weight_east=0, weight_west=0
                  → output_col = (south − north) / 2     (north=iy−1, south=iy+1)
```

The shader sees `[−0.5, +0.5, 0, 0]` — generic floats. The designer authors `Gradient { axis: Y }`.
Both X and Y components are obtained by **authoring two explicit gradient field passes**, each
single-target. No dual-output kernel is required.

### Magnitude without new opcodes

Euclidean magnitude needs `sqrt`, which is **not** in the opcode set and is **deferred**. Two
existing-opcode forms compose the components at L3:

| Form | Expression | Notes |
|---|---|---|
| **Manhattan** `\|gx\| + \|gy\|` | `ABS(gx) + ABS(gy)` | `ABS + ADD`; overestimates by ≤ √2 |
| **Squared** `gx² + gy²` | `MUL(gx,gx) + MUL(gy,gy)` | exact; good for threshold comparisons |

Both are `ExactDeterministic`. For steering/routing the *components* are usually more useful than
the magnitude — they encode direction directly.

---

## 3. RegionFieldSpec: single-target `Gradient` operator variant

**Proposed addition to `RegionFieldOperatorSpec` (single-target, staged):**

```rust
pub enum RegionFieldOperatorSpec {
    Normalized,
    SourceCappedNormalized,
    Gradient {
        axis: GradientAxisSpec,
        output_col: u32,
    },
}

pub enum GradientAxisSpec {
    X, // (east − west) / 2
    Y, // (south − north) / 2
}
```

(Equivalent `GradientX { output_col }` / `GradientY { output_col }` variants are acceptable; the
single-target, one-output-column contract is the binding requirement, not the exact enum shape.)

**Admission rules (designer/spec layer):**
- exactly **one** `output_col` per admitted gradient field, in range `[0, n_dims)`
- explicit `axis`
- `output_col != source_col` (no self-gradient write loop) — **enforced today** (single-spec
  admission, M-5A landed)
- `output_col == target_col` for a pure gradient field (the gradient's output *is* its target) —
  **enforced today**
- per-direction weights resolved by the compiler from `axis`; the shader never receives "gradient"
- CPU-oracle parity test required (see §5)

### Input Validation Rule — gradient fields are strict sinks (binding; frame/scenario-level)

A gradient (directional-derivative) field must act **strictly as a sink**. The base field it
differentiates is **immutable within the frame** from the gradient's perspective: the gradient only
*reads* the base column and *writes* its own derivative column; it never feeds back into a diffusion
input within the same frame. Concretely, spec/scenario admission must **reject** any configuration where:

1. **a field reads its own immediate output column as a diffusion source within the same frame**
   (`source_col == output_col`/`target_col`) — the per-field self read-after-write hazard.
   *(Already enforced at single-spec admission.)*
2. **a gradient field's `output_col` is used as the `source_col` of any diffusion/stencil field in
   the same frame** — i.e. the derivative is wired back as a diffusion input. The gradient output is
   consumed only **downstream** (Layer-2 reduction, Layer-3 EML, thresholds), never as an upstream
   diffusion source within the frame. *(Enforced at frame/scenario-level admission via
   `validate_region_field_frame_gradient_sinks` / `compile_region_field_frame_preview`; M-5D landed.)*

Rationale: within-frame feedback (a field's derivative re-entering its own or the base field's
diffusion in the same tick) breaks the base field's immutability, creates a read-after-write hazard,
and violates the band-ordering invariant that feedback closes **across ticks, not within a tick's
algebraic cycle**. Cross-tick coupling (this tick's gradient influencing next tick's field via an
authored, ordered path) remains allowed; *same-frame* feedback into a diffusion source is rejected.

**Status:** clause (1) enforced (M-5A). Clause (2) enforced (M-5D) at frame/scenario-level admission.

**What the designer gets:** one output column of the requested gradient component for each
RegionCell, computed from the field every tick — a first-class Layer-1 column readable by the
Layer-2 reduction and Layer-3 EML like any other field column, but **not** by a same-frame diffusion
source.

**`GradientXY` (dual-output) is NOT the M-5A-gradient contract.** It remains a possible future
ergonomic alias / single-pass optimization — see §6.

---

## 4. L3 Strategic Pressure Composition Pattern (no new substrate)

The sanctioned way to composite multiple independent fields into a stable signal — for AI pressure
*or* resource/migration routing. Uses only landed substrate.

```text
Field A (threat / scarcity / openings) → L1 stencil      → L2 SlotRange Sum → parent col
Field B (supply / cost)                → L1 stencil      → L2 SlotRange Sum → parent col
Field gx (Gradient axis=X)             → L1 Gradient op  → L2 SlotRange Sum → parent gx_col
Field gy (Gradient axis=Y)             → L1 Gradient op  → L2 SlotRange Sum → parent gy_col

L3 EvalEML (WeightedAccumulator over EMA-smoothed inputs):
  ema_a    = EMA(a_col,  decay=0.8)            // temporal smoothing — prevents flicker
  ema_b    = EMA(b_col,  decay=0.8)
  ema_mag  = EMA(|gx|+|gy|, decay=0.9)         // gradient magnitude (Manhattan, existing opcodes)
  pressure = WeightedAccumulator([ema_a, ema_b, ema_mag], weights=[w1, w2, w3])
  → Threshold + EmitEvent (commitment / routing trigger)
```

### EMA on L3 inputs is mandatory

Without EMA, a single-tick field change can spike pressure across the threshold and immediately
retract — jitter (an AI that flickers, or a resource router that thrashes). The EMA low-pass gives
*persistence*: the system reacts to trends, not single-tick noise. This is the pattern for any
field→threshold chain, AI or not.

### Separation of concerns (three-layer model)

| Layer | Responsibility | Substrate |
|---|---|---|
| L1 | Spatial diffusion + gradient extraction | `StructuredFieldStencilOp` (generic, single-target) |
| L2 | Aggregation | `SlotRange Sum` reduction |
| L3 | Weighted composite signal | `EvalEML` (WeightedAccumulator + EMA) |
| Commitment/route | Threshold event | `Threshold + EmitEvent` (existing) |

Fields evolve **independently** at L1. Coupling happens only at L3. L1 cross-field coupling (stencil
terms that read another field) remains **deferred** (scheduling complexity) and banned in V1.

---

## 5. PR ladder (M-5-gradient)

| Slice | Scope | Gate |
|---|---|---|
| **M-5A-gradient** | (1) generic per-direction stencil weights (`weight_north/south/east/west`) replacing `gamma_neighbor`, **single output column**; (2) single-target `Gradient { axis, output_col }` operator variant + admission; (3) CPU oracle `GradientX=(east−west)/2`, `GradientY=(south−north)/2`, boundary behavior matching existing boundary mode, GPU parity on small grids; (4) docs + test report | Generic WGSL extension under revised guardrail; single-output contract preserved |
| **M-5B-gradient** | Reference RON composition fixture: multi-field L3 WeightedAccumulator + EMA over gradient + other reductions (no new substrate; RON/test only) | After M-5A-gradient; no substrate change |
| **M-5C-gradient** | Product-facing need/routing signal RON fixture | After M-5B; no production bridge |
| **M-5D-gradient** | Frame/scenario-level gradient strict-sink admission | After M-5B/C; spec/admission only |
| **Deferred** | Dual-output `GradientXY` (one-pass); Euclidean magnitude (`sqrt` opcode); L1 field coupling; dense per-cell gradient columns | Separate optimization/product gates (§6) |

**Required test report (implementation handoff must create):**
`docs/tests/phase_m_m5a_gradient_single_target_test_results.md` — recording: exact files changed;
exact tests run; exact scans run; GPU parity results if the GPU path is touched; transient-log
cleanup; and explicit statements that **no semantic WGSL** was added, **no default mapping wiring**
was added, **no `simthing-sim` semantics** changed, and **no production economy→mapping bridge** was
added.

---

## 6. Deferred: dual-output `GradientXY` (separate optimization gate)

A single-pass `GradientXY { output_col_x, output_col_y }` computing both components in one dispatch
is a legitimate *future optimization*, but it is **not** the M-5A-gradient substrate contract
because it requires, all under their own gate:
- two output columns written in one kernel dispatch (widened output contract)
- updated GPU output-write protocol + uniform/layout review
- a dual-output CPU oracle and dual-output GPU parity tests
- stronger write-conflict admission (two targets, no aliasing)
- explicit interaction review with ping-pong buffers and source/target column constraints

Until that gate is taken, designers obtain both components by authoring two single-target `Gradient`
field passes (`axis: X` and `axis: Y`). Dual-output may later land as an ergonomic alias that
compiles to the same generic weights.

---

## 7. Stop conditions (binding — implementation must stop and escalate)

A future M-5-gradient implementation PR must stop rather than implement if it:
- cannot support `GradientX`/`GradientY` via the **single-target** path without dual-output kernel changes
- requires **semantic WGSL** (shader naming "gradient"/"threat"/"faction"/gameplay) or map/faction/AI branching in the shader
- requires `sqrt` or any transcendental opcode
- requires **L1 cross-field coupling**
- requires **source-mask / source-identity** work or behavioral source policy (that is the separate `M-5` track)
- requires **atlas / M-4A**
- changes `simthing-sim` (no map/Gadget/Personality awareness)
- wires mapping into the **default SimSession** pass-graph (mapping stays opt-in/default-off)
- adds a **CPU-side AI planner** or CPU urgency computation (SEAD commitment stays GPU-resident:
  field propagation → parent reduction → `field_urgency` EvalEML → Threshold + EmitEvent)
- adds a **production economy→mapping bridge**
- changes Resource Flow defaults (stays default-off)
- cannot make GPU parity **exact** against the CPU oracle

---

## 8. Relationship to the existing `M-5` (source-identity) track

The existing **`M-5`** is the generic source-identity / `source_mask` buffer (behavioral source
policy gate) — a **different track**. This gradient work is **`M-5-gradient`** and is orthogonal:
gradient extraction operates on `Normalized`/`SourceCappedNormalized` diffused fields with no
source-mask dependency, and the L3 composition pattern is pure EML gadgets. Use the `-gradient`
suffix in all handoffs to avoid the naming collision; do not retire or rename the source-identity
`M-5` track here.
