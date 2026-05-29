# Phase M-5: GradientExtraction Operator + L3 Strategic Pressure Composition Pattern

**Status:** Approved candidate track (Opus design authority, 2026-05-29).
**Sequencing:** Candidate for M-5, in parallel with or after the EML-GADGET-2 ladder completion
and alongside Phase M Resource Economy Authoring Ergonomics R2.
**Layer split:**
- `StructuredFieldStencilOp` kernel: generic per-direction weight extension (WGSL) — **new generic WGSL allowed under the revised guardrail** (§1).
- `RegionFieldSpec` admission: `GradientXY` operator variant — where the *meaning* "gradient extraction" is pinned at the designer/spec layer.
- RON composition pattern: designer-authored multi-field → L3 WeightedAccumulator + EMA — **no new substrate**.

---

## 1. The revised WGSL guardrail

The SimThing constitution forbids **semantic WGSL** — shaders that embed map/faction/AI/gameplay
meaning. It does **not** and has never forbidden **generic kernel extensions** — new parameters that
remain purely mathematical (column indices, scalar weights, bounds), with all meaning pinned at the
designer/spec admission layer.

**Old framing (too broad):** "No new WGSL" — treated any shader change as forbidden.

**Correct framing (revised, Opus 2026-05-29):**
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

This directly matches the V7.6 greenfield admission criterion: "New GPU/EML work is allowed when
it is a generic primitive (not scenario semantics), opt-in (no production default changes), does
not impair Resource Flow / E-11B / Phase T / simthing-sim behavior, and is covered by regression
tests with documented admission constraints."

**Where meaning is raised to:** The designer/RON/spec admission layer. The shader receives four
generic per-direction weights (`weight_north`, `weight_south`, `weight_east`, `weight_west`). The
spec compiler maps the `GradientXY` operator variant to specific weight patterns. The GPU never
knows the word "gradient."

---

## 2. Why the stencil kernel can accept per-direction weights

The current `structured_field_stencil.wgsl` already has:
- A `variant` field controlling operator behavior (isotropic vs directional)
- A `directed_mode` field for partial-neighbor patterns
- A `_pad` field (spare uniform space)

The kernel was **designed for extension**. Adding `weight_north`, `weight_south`, `weight_east`,
`weight_west` (four f32s) to `FieldStencilParams` replaces the single scalar `gamma_neighbor` with
four independent weights. The computation becomes:

```wgsl
next = alpha_self_decay * center
     + weight_north * north
     + weight_south * south
     + weight_east  * east
     + weight_west  * west
```

This is still purely generic linear combination over flat buffers. No semantic knowledge required.

### Central-difference gradient weights (spec layer, not shader layer)

The `GradientXY` operator variant in `RegionFieldOperatorSpec` compiles to:
```
X-gradient:  weight_east=+0.5, weight_west=−0.5, weight_north=0, weight_south=0
Y-gradient:  weight_north=−0.5, weight_south=+0.5, weight_east=0, weight_west=0
```

The shader sees `[0, 0, +0.5, −0.5]` — generic floats. The designer authors `GradientXY`. The CPU
oracle evaluates the same formula. The GPU never branches on "gradient."

**Note on direction convention:** `north = iy−1`, `south = iy+1` per the existing kernel.
Y-gradient sign is a designer-admitted convention, not a semantic.

### Magnitude approximation without new opcodes

True Euclidean magnitude requires `sqrt`, which is not in the EvalEML opcode set. Two clean options
expressible with existing opcodes:

| Form | Expression | Notes |
|---|---|---|
| **Manhattan** `\|grad_x\| + \|grad_y\|` | `ABS(grad_x) + ABS(grad_y)` | via existing `ABS + ADD`; overestimates by factor ≤ √2 |
| **Squared magnitude** `grad_x² + grad_y²` | `MUL(grad_x,grad_x) + MUL(grad_y,grad_y)` | exact; good for threshold comparisons; overestimates for linear distances |

Both are `ExactDeterministic` EML gadget compositions. For AI steering the gradient *components*
(x and y separately) are often more useful than magnitude — they directly encode direction.

---

## 3. RegionFieldSpec: GradientXY operator variant

**Proposed addition to `RegionFieldOperatorSpec`:**

```rust
pub enum RegionFieldOperatorSpec {
    Normalized,
    SourceCappedNormalized,
    GradientXY {
        /// Output column for X component (east − west) / 2
        output_col_x: u32,
        /// Output column for Y component (south − north) / 2
        output_col_y: u32,
    },
}
```

**Admission rules (designer/spec layer — new):**
- `output_col_x != output_col_y`
- Both output columns in range `[0, n_dims)`
- `output_col_x != source_col`, `output_col_y != source_col`
- `output_col_x != target_col`, `output_col_y != target_col`
- A gradient field should not be the `source_col` for the same field (you don't gradient-diffuse a gradient)
- CPU oracle parity test (central-difference computation; bit-exact)

**What the designer gets:** two output columns `grad_x` and `grad_y` for each RegionCell, computed
from the existing diffused field every tick. These are first-class Layer-1 output columns, readable
by the Layer-2 reduction and Layer-3 EML exactly like any other field column.

---

## 4. L3 Strategic Pressure Composition Pattern (no new substrate)

This pattern is the sanctioned way to composite multiple independent stencil fields into a stable
faction-level strategic pressure signal. It uses only landed substrate.

### Pattern

```text
Field A (threat)   → L1 StructuredFieldStencilOp → L2 SlotRange Sum → parent threat_col
Field B (supply)   → L1 StructuredFieldStencilOp → L2 SlotRange Sum → parent supply_col
Field C (gradient) → L1 GradientXY op            → L2 SlotRange Sum → parent grad_x_col, grad_y_col

L3 EvalEML (WeightedAccumulator over EMA-smoothed inputs):
  ema_threat   = EMA(threat_col,   decay=0.8)   // temporal smoothing — prevents flicker
  ema_supply   = EMA(supply_col,   decay=0.8)
  ema_grad_mag = EMA(|grad_x|+|grad_y|, decay=0.9)  // gradient magnitude for directionality

  pressure = WeightedAccumulator([ema_threat, ema_supply, ema_grad_mag],
                                  weights=[w_aggression, w_risk, w_direction])
  → Threshold + EmitEvent (SEAD commitment)
```

### Why EMA is mandatory here

Without EMA on the L3 inputs, a one-cell change in the field produces a one-tick pressure spike
that can cross the commitment threshold and immediately fall back — jitter. The EMA low-pass filter
gives "strategic persistence": the AI reacts to *trends* (rising threat, falling supply) rather
than single-tick noise. This is not an option; it is the pattern for any pressure-to-commitment
chain.

### Separation of concerns

| Layer | Responsibility | Substrate |
|---|---|---|
| L1 | Spatial diffusion + gradient extraction | `StructuredFieldStencilOp` (generic) |
| L2 | Faction aggregation | `SlotRange Sum` reduction |
| L3 | Personality-weighted strategic pressure | `EvalEML` (WeightedAccumulator + EMA) |
| Commitment | Threshold event | `Threshold + EmitEvent` (existing) |

Fields evolve **independently** at L1. Coupling happens only at L3 via the weighted compositor.
This is the three-layer model applied — L1 coupling (cross-field stencil at the diffusion layer)
remains deferred as "massive scheduling headache" (per the proposal) and banned in V1.

---

## 5. PR ladder

| Slice | Scope | Gate |
|---|---|---|
| **M-5A** | Stencil kernel per-direction weight extension + `GradientXY` operator variant in `RegionFieldSpec`; CPU oracle; parity tests; admission | Generic WGSL extension under revised guardrail |
| **M-5B** | Reference RON composition fixture: multi-field L3 WeightedAccumulator + EMA pattern (no new substrate; RON/test only) | After M-5A or in parallel; no substrate change |
| **Deferred** | Euclidean magnitude (`sqrt` opcode); L1 field coupling; dense per-cell gradient columns | Separate product + VRAM + gate |

---

## 6. Stop conditions

A future M-5 implementation PR must not:
- add map/faction/AI/gameplay semantics to the stencil shader (shader sees generic floats only)
- skip CPU-oracle parity for the new `GradientXY` kernel path
- add `sqrt` or any transcendental without a separate opcode gate
- implement L1 field coupling (cross-field stencil terms)
- place gradient columns on dense per-cell memory beyond the existing RegionCell field allocation
- change `simthing-sim` or `simthing-gpu`'s semantic surface
- wire gradient fields into the default session pass-graph

---

## 7. Relationship to M-5 (source-identity buffer)

The existing M-5 (`PR M-5`) is the generic source-identity buffer, which unblocks behavioral source
policy (`source_mask`). That track is orthogonal. If both are needed:
- Keep M-5 as the source-identity buffer (behavioral source policy gate)
- Name this track **M-5-gradient** or fold into Phase M-5 as a parallel sub-track

The tracks do not conflict; gradient extraction uses `source_capped_normalized` diffused fields (no
source-mask dependency) and the L3 composition pattern is pure EML gadgets.
