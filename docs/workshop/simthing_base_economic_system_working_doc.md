# SimThing Base Economic System: Provisional Thresholded Accumulation and Emission

> **Status: PROVISIONAL WORKING DOCUMENT**
>
> This is a design synthesis, not an accepted ADR and not an implementation spec. It records the current direction of thought for the base SimThing economic/causal substrate. Future ADRs should promote, revise, or reject these ideas after inspection against the current engine invariants.

## Purpose

This working document captures the current base-economy synthesis for SimThing. It is meant to be readable by modders while still being precise enough for Claude, Cursor, Codex, or a future implementation chat.

The current provisional thesis:

> The base economic system may be a special case of a more general intrinsic SimThing facility: **thresholded accumulation and emission**.

Queue construction, factory production, fleet replenishment, weapons damage, hitpoints, population growth, diplomatic relation shifts, research, repair, logistics, and upkeep are all possible authored cases of the same deeper pattern:

```text
property accumulates value
→ overlays modify, route, or convert that value
→ threshold detects a crossing
→ boundary semantic emits effects
→ accumulator is deducted, reset, retained, scarred, expired, or transformed
```

The base SimThing economy should therefore not become a separate economy engine. It should remain ordinary SimThing semantics applied to resource-bearing or progress-bearing properties, overlays, thresholds, and boundary effects.

---

## Provisional Core Thesis

The proposed intrinsic primitive is not `QueueSettlement`, `ConstructionQueue`, or `ResourceConsumed`.

The more general primitive is something like:

```rust
ThresholdSemantic::EmitOnThreshold
```

or:

```rust
ThresholdSemantic::ThresholdedEmission
ThresholdSemantic::AccumulatorEmission
```

Naming remains provisional. The intended meaning is:

> Any SimThing property can act as an accumulator. When the accumulator crosses an authored threshold, a core boundary semantic may emit one or more effects, scale those effects by the number of completed units, and then apply an authored consume/reset rule to the accumulator.

This is the constitutional grammar:

```text
Properties hold accumulation state.
Overlays route, generate, consume, or transform accumulation.
Thresholds detect crossings.
Boundary semantics emit consequences.
```

Shortest provisional form:

```text
Accumulator = property/subfield with Amount, Velocity, Intensity, or authored layout
Input = overlay, transfer, velocity, damage, growth, diplomacy pressure, etc.
Threshold = crossing condition
Emission count = floor(accumulator_amount / unit_size), or other authored pure formula
Effects = increments, transfers, overlays, fission, instantiation, unlocks, events
Consume mode = subtract, clamp, retain, scar, expire, reload debt, or transfer remainder
```

---

## Current Threshold Event Model

The existing GPU threshold path is already close to what this needs.

Current GPU event shape:

```rust
ThresholdEvent {
    slot,
    col,
    value,
    event_kind,
}
```

Important interpretation:

```text
slot       = SimThing slot whose watched value crossed
col        = watched global column
value      = current crossed value, not the threshold and not the delta
event_kind = opaque u32 resolved by CPU ThresholdRegistry into ThresholdSemantic
```

That means a debt-band accumulator can already use the reported `value` on the CPU side.

Example:

```text
queued_build.Amount previous = -200
queued_build.Amount current  = -145
unit_cost = 20
queued_count = 10
registered threshold = -180
```

GPU detects the crossing and emits an event with `value = -145`. CPU can compute:

```text
paid = queued_count * unit_cost + value
paid = 10 * 20 + (-145)
paid = 55
emit_count = floor(paid / unit_cost) = 2
```

The GPU already detects the crossing. The open design question is whether the GPU should also compute `emit_count` and return it as an auxiliary payload.

---

## Debt-Band Emission Mode

A strong one-column candidate is **DebtBandEmission**.

State:

```text
queued_build.Amount = -200
unit_cost = 20
queued_count = 10
```

Meaning:

```text
10 units remain queued.
Each unit costs 20 normalized value.
Total remaining debt is -200.
```

Register the next per-unit threshold:

```text
next_threshold = -((queued_count - 1) * unit_cost)
next_threshold = -180
```

If the accumulator jumps from `-200` to `-145`, the GPU fires because it crossed `-180`. The boundary handler computes:

```text
paid = queued_count * unit_cost + current_value
paid = 10 * 20 + (-145) = 55
emit_count = floor(55 / 20) = 2
```

Then effects can apply:

```text
target.unit_count += 2
queued_count -= 2
queued_build.Amount remains -145
```

New state:

```text
queued_count = 8
queued_build.Amount = -145
next_threshold = -((8 - 1) * 20) = -140
```

The remaining `15` value is preserved as carryover toward the next unit. This gives:

```text
one accumulator column
one active threshold registration
multi-unit emission if input overshoots several bands
fractional carryover
no per-resource resident debt columns
```

---

## EML Position for This Use Case

This document must follow `docs/eml_integration_guidance.md`.

That guidance says EML may become a backend for **pure numeric expressions**, derived fields, AI weights, scripted values, and composite predicates. It should not replace native SimThing primitives such as overlays, thresholds, boundary requests, or effect handlers.

Therefore this use case splits cleanly:

```text
Good EML / numeric backend candidate:
  emit_count = floor((queued_count * unit_cost + current_value) / unit_cost)

Not EML:
  target.unit_count += emit_count
  queued_count -= emit_count
  expire queue item
  instantiate child
  attach overlay
  emit event
  record delta log entries
```

The pure formula may later be lowered to a native GPU expression backend or EML. The actual consequence remains CPU boundary semantics or another native SimThing boundary mechanism.

Provisional rule:

> EML may compute or help compute the numeric emission payload. EML must not own the effectful emission.

---

## Proposed Event Payload Track

Current event:

```rust
ThresholdEvent {
    slot: u32,
    col: u32,
    value: f32,
    event_kind: u32,
}
```

Possible later event or side-buffer payload:

```rust
ThresholdEventV2 {
    slot: u32,
    col: u32,
    value: f32,
    event_kind: u32,
    aux0: f32, // e.g. emit_count or computed score
    aux1: f32, // e.g. remainder, optional
}
```

Alternative:

```text
ThresholdEvent remains unchanged.
A compact emission payload buffer is indexed by event ordinal or event_kind.
CPU reads payload only for threshold semantics that need it.
```

No payload expansion should happen until v1 semantics are proven with the current event shape.

---

## V1–V3 Validation and Testing Regime

### V1 — CPU Semantic Baseline

Goal:

```text
Prove the intrinsic thresholded-emission model using existing GPU ThresholdEvent.value and CPU boundary semantics.
```

Implementation shape:

```text
GPU:
  increments/integrates accumulator normally
  detects threshold crossing normally
  emits ThresholdEvent { slot, col, value, event_kind }

CPU:
  event_kind -> ThresholdSemantic::EmitOnThreshold / ThresholdedEmission
  reads emission metadata from session/spec registry
  computes emit_count from event.value
  applies boundary effects
  updates accumulator/queue metadata
  re-registers next threshold if needed
```

Required fixtures:

```text
1. Debt-band queue:
   Amount -200, unit_cost 20, queued_count 10, value jumps to -145.
   Expected emit_count = 2, queued_count = 8, carryover preserved.

2. One-column positive accumulator:
   growth.Amount 2.4, unit_size 1.
   Expected emit_count = 2, remainder 0.4.

3. Retain-mode gate:
   trust.Amount crosses threshold.
   Expected overlay/event emitted without consuming trust.

4. Damage/scar mode:
   damage crosses band.
   Expected overlay/event emitted and accumulator scarred or retained.

5. No overshoot case:
   value crosses exactly one band.

6. Large overshoot case:
   value crosses many bands in one tick; handler emits all allowed units.
```

Correctness checks:

```text
CPU reference calculator and boundary result agree.
Fractional carryover is preserved.
Consume modes behave deterministically.
Delta log/replay path remains coherent.
Threshold rebuild points at the next active band.
Existing fission/fusion/capability/scripted-event tests remain green.
```

Performance measurements:

```text
N active emitters: 1k, 10k, 100k, 1M if feasible.
Crossing density: 1%, 10%, 50%, 100%.
Measure boundary CPU time for emit_count computation and effect dispatch.
Measure threshold registry rebuild/append cost.
Measure event readback size and time.
```

Exit criteria:

```text
Semantics are stable.
CPU handler cost is understood.
No GPU payload changes are justified until CPU cost is a measured bottleneck.
```

### V2 — Native GPU Payload Prototype

Goal:

```text
Determine whether computing emit_count on GPU with a native expression/payload path reduces boundary cost enough to justify event-payload complexity.
```

Implementation shape:

```text
GPU:
  detects threshold crossing
  computes emit_count using a fixed native formula for DebtBandEmission
  writes emit_count to aux payload or side buffer

CPU:
  still owns all effects
  reads emit_count instead of recomputing it
```

Prototype formula:

```text
emit_count = floor((queued_count * unit_cost + current_value) / unit_cost)
```

Required GPU-resident metadata:

```text
unit_cost
queued_count or remaining_count
max_emit_per_boundary, optional
mode identifier, optional
```

Required comparisons against V1:

```text
Same fixture results as V1.
Bitwise or tolerance-defined agreement for emit_count.
No missed emissions under overshoot.
No duplicate emissions after threshold rebuild.
Event payload readback does not exceed CPU recomputation cost.
```

Performance measurements:

```text
GPU pass time with and without payload computation.
Event/payload buffer readback size.
CPU boundary time saved.
Total frame/day-boundary time.
Scaling under high event density.
```

Exit criteria:

```text
V2 must beat or simplify V1 under realistic high-density scenarios.
If CPU recomputation is cheaper than payload complexity, do not promote V2.
If payload readback dominates, do not promote V2.
```

### V3 — ScriptExpr / EML Backend Experiment

Goal:

```text
Test whether a general pure numeric expression backend, potentially EML, is beneficial for emission formulas beyond the fixed native DebtBandEmission case.
```

Implementation shape:

```text
Author formula as ScriptExpr.
Compile ScriptExpr to CPU reference evaluator.
Compile ScriptExpr to native GPU expression graph and/or EML backend.
Use formula only to compute numeric payloads such as emit_count, scores, caps, or conversion rates.
Effects remain boundary semantics.
```

Candidate formulas:

```text
Debt-band emit count.
Capped production emission.
Diminishing-return growth emission.
Damage threshold severity score.
Diplomatic relation threshold score.
Factory output under efficiency and disruption modifiers.
```

Correctness tests:

```text
CPU reference evaluator matches native GPU backend.
Native GPU backend matches EML backend within defined tolerance.
Domain guards prevent NaN/inf propagation.
Rounding/floor semantics are deterministic and documented.
Designer-authored formula lowers through ScriptExpr, not raw EML.
```

Safety tests:

```text
safe_log behavior, if EML is used.
safe_exp behavior, if EML is used.
overflow/underflow behavior.
NaN and infinity handling.
clamping and totalized semantics.
```

Performance tests:

```text
Expression compile time.
GPU evaluation time by expression size.
Tree-size growth for EML lowering.
Debuggability and explanation output.
Comparison against fixed native formulas.
Comparison against CPU recomputation.
```

Exit criteria:

```text
EML is promoted only if it improves or unifies enough complex pure numeric formulas to justify safety, debugging, and implementation cost.
If native GPU expressions are faster/simpler, prefer native GPU expressions.
If CPU computation is sufficient, defer both.
```

---

## Relationship to Resources

Resource balances remain valid, but they are now one authored use of the broader accumulator/emitter facility.

Canonical resource layout remains:

```text
resource::<key>
  Amount
  Velocity
  Intensity
```

Meanings:

```text
Amount:
  Current resource balance.
  Positive amount means surplus, stock, accumulated value, satisfied value, or available balance depending on context.
  Negative amount means requirement, debt, shortage, unsatisfied input, or remaining cost depending on context.

Velocity:
  Net flow.
  Positive velocity generates or restores the resource.
  Negative velocity consumes, drains, degrades, or incurs upkeep.

Intensity:
  Stress, volatility, urgency, instability, or expression strength.
  It should not be used as base allocation priority.
```

Resource-debt example:

```text
FrigateConstruction
  resource::alloys.Amount = -400
  resource::electronics.Amount = -80
  resource::shipyard_capacity.Amount = -60
```

One-column normalized accumulator example:

```text
queue::unit_progress.Amount = 0.0
threshold: unit_progress >= 1.0
```

Multi-resource cost can live in transfer/conversion metadata that feeds normalized progress.

---

## Overlays Own Input and Intent

Overlays should express active economic or causal instructions:

```text
which source resources are routed
which target accumulator is fed
which policy/tech/unrest modifies velocity
which conversion rate applies
which priority or cap applies
whether the process is active, suspended, or expired
```

Examples:

```text
ConstructionFundingOverlay:
  source: alloys/electronics/labor
  target: queue::unit_progress
  conversion: per-unit cost metadata -> normalized progress
  cap: max unit_progress per day

FactoryOperationOverlay:
  energy.Velocity -= 2
  minerals.Velocity -= 6
  assembly_progress.Velocity += 3

DamageOverlay:
  ship::damage.Amount += weapon_damage
```

Overlays answer:

```text
How does value enter or leave the accumulator?
```

Thresholded emission answers:

```text
What happens when enough value has accumulated?
```

---

## Transfer Overlays Remain a Useful Primitive

Resource transfer is still needed for direct movement of values between SimThings.

Conceptual transfer overlay:

```text
TransferOverlay:
  source: SimThing.property/subfield
  target: SimThing.property/subfield
  rate: amount per tick/day OR share/cap
  constraints:
    - source availability
    - target pull or capacity
    - scope / ownership / capability
  lifecycle:
    - until target reaches threshold
    - until source exhausted
    - while policy active
    - while process active
```

Runtime effect:

```text
x = bounded_transfer_amount(source, target, rate, constraints)
source -= x
target += x
```

A heavy allocation resolver is not required for v1. A later resolver may convert competing priority claims into explicit transfer rates.

---

## Current Fission/Fusion Is Not Economic Transfer Yet

Current fission/fusion provides useful boundary-time structural mutation machinery, but it does not currently perform economic amount transfer.

Current fission is closer to:

```text
child.properties = copy(parent.properties)
child.activating_property.Amount = 0
parent unchanged
```

Current fusion is closer to:

```text
parent.activating_property.Amount *= (1 - fusion_scar_coefficient)
remove child
tombstone child slot
```

So the economic/emission model should not assume current fission/fusion already supports:

```text
source.Amount -= x
target.Amount += x
```

That operation still needs explicit transfer or emission effects.

Fission/fusion remains relevant as one possible boundary effect for emitted consequences, not as the entire resource/queue mechanism.

---

## Queue Construction as an Example, Not the Root Primitive

Queueable items are still useful, but they should be described as a specialization of thresholded emission.

One-column queue version:

```text
QueueableItem SimThing:
  queue::unit_progress.Amount

Metadata/spec/session state:
  queued_count
  output target
  output mode
  per-unit input conversion
```

Threshold:

```text
unit_progress.Amount >= 1.0
```

Boundary emission:

```text
completed = min(queued_count, floor(unit_progress.Amount))
target.unit_count += completed
queued_count -= completed
unit_progress.Amount -= completed
if queued_count == 0: expire queue item
```

Debt-band queue version:

```text
queued_build.Amount = -200
unit_cost = 20
queued_count = 10
next_threshold = -180
```

Boundary emission uses event value to compute overshoot:

```text
paid = queued_count * unit_cost + event.value
emit_count = floor(paid / unit_cost)
```

---

## Parent/Child Production and Aggregation

A SimThing can produce resources or accumulated value through its own properties and through attached children.

Canonical provisional rule:

> Parent-level availability may be derived from reducible descendant state, subject to authored scope, ownership, boundary, capability, logistics, and overlay rules.

Aggregation should be authored or declared, not automatic for every property.

Open questions:

```text
Is this resource/value exportable upward?
Does the child reserve local needs first?
Does ownership permit parent allocation?
Do blockade/logistics reduce export?
Do autonomy/corruption tax the flow?
Does local policy redirect it?
```

---

## Column Discipline

Do not bloat the dense property matrix.

Default resource/accumulator layouts should stay lean:

```text
Amount
Velocity
Intensity
```

Avoid default resident columns like:

```text
demand
allocated
priority
cost
remaining
source
sink
production
consumption
waste
suppression
```

unless the value must participate in simulation, reduction, thresholding, save/load, AI observation, or player-visible diagnostics.

Derived values should remain derived:

```text
remaining = max(0, -Amount)
pull = max(0, -target.Amount)
emit_count = floor(Amount / unit_size)
allocated_this_tick = diagnostic output
priority = overlay metadata
```

Optional advanced layouts may exist, but they are opt-in.

---

## Property Kind / Metadata Recommendation

The architecture likely needs a way to identify resource properties and/or accumulator-emitter properties.

Possible approaches:

```text
SimPropertyKind::Resource
SimPropertyKind::Accumulator
PropertyMetadata.economic_role = Resource
PropertyMetadata.emission_role = Accumulator
PropertyTag::Resource
PropertyTag::Accumulator
```

Do not hardcode authored resource names like `Alloys`, `Food`, or `Influence` into core enums. Prefer authored keys:

```text
resource::alloys
resource::food
resource::engineering_research
resource::legitimacy
queue::unit_progress
factory::assembly_progress
population::growth
ship::damage
```

Open question:

> Is `EmitOnThreshold` attached to property metadata, threshold metadata, a spec-side emission registry, or a new core threshold semantic registration path?

---

## Runtime Phase Model

Likely conceptual order:

```text
1. Apply ordinary overlays to property values and velocities.
2. Integrate velocity into amount.
3. Apply child-to-parent aggregation/materialization where authored.
4. Execute direct transfer overlays where authored.
5. Optionally resolve allocation claims into transfers.
6. Evaluate thresholds.
7. Apply boundary handlers: emission, unlock, fission, transfer, instantiate, expire, reload.
8. Update overlay lifecycle.
9. Emit diagnostics/events.
```

Conceptual invariant:

```text
state movement before threshold interpretation;
threshold interpretation before boundary consequences.
```

Exact order must be validated against the current boundary sequence before implementation.

---

## Implementation Handoff

### A0 — ADR / Design Only

Create an ADR after this design stabilizes, likely:

```text
docs/adr/thresholded_accumulation_emission.md
```

The ADR should establish or reject:

```text
1. Whether thresholded accumulation/emission is intrinsic SimThing semantics.
2. Whether the core semantic is named EmitOnThreshold, ThresholdedEmission, or something else.
3. How emission specs are registered and resolved from threshold events.
4. Which consume/reset modes are supported in v1.
5. Which effect kinds are supported in v1.
6. How resource transfer overlays feed accumulators.
7. How queue construction is represented as one authored use case.
8. Why GPU/EML lowering is deferred until validated.
9. How this interacts with existing fission/fusion, capability unlocks, and scripted events.
```

### Suggested Implementation Ladder

```text
A1 — ADR for thresholded accumulation/emission
A2 — Minimal emission spec registration path
A3 — CPU/session boundary handler for EmitOnThreshold
A4 — One-column queue fixture
A5 — Population growth or factory-output fixture
A6 — Resource transfer overlay fixture
A7 — Optional resource/accumulator tags
A8 — Native GPU payload prototype, only after profiling
A9 — ScriptExpr / EML backend experiment, only after V2 baseline
A10 — Modder guide and examples
```

Do not implement in v1:

```text
hardcoded economy engine
hardcoded research engine
hardcoded construction engine
hardcoded market system
hardcoded job/pop system
GPU allocation pass
GPU transfer pass
universal priority columns
universal demand columns
universal allocated columns
complex market price formation
```

---

## Modder Mental Model

> A SimThing can accumulate value in a property. Overlays push or pull on that value. When the value crosses a threshold, the SimThing emits effects. The accumulator may spend the emitted amount, keep a remainder, retain the state, scar itself, expire, or reload.

Examples:

```text
A factory accumulates assembly progress and emits inventory.
A population accumulates growth and emits new population count.
A ship accumulates damage and emits hull-breach overlays or events.
A diplomatic relation accumulates trust and emits treaty availability.
A queue accumulates unit progress and emits completed ships/cohorts.
A technology accumulates research and emits a capability unlock.
A resource pool accumulates stock and emits shortage/overflow consequences.
```

The modder-facing slogan:

> Accumulate value, cross a threshold, emit consequences.

Older economic slogan, still valid for resource-debt use cases:

> To build something, give it a debt. To fund it, transfer value into it. To finish it, watch it cross zero.

---

## Final Provisional Summary

This document currently proposes a broader base principle than the earlier resource/queue framing:

```text
Base primitive = thresholded accumulation and emission
Resource economy = one use case
Queue construction = one use case
Factory production = one use case
Damage/hitpoints = one use case
Population growth = one use case
Diplomacy/relation thresholds = one use case
```

Potential core form:

```text
Accumulator = property/subfield
Input = overlay/transfer/velocity
Threshold = crossing condition
Emit count = floor(value / unit_size), or authored pure formula
Effects = increments/transfers/overlays/instantiation/fission/unlocks/events
Consume mode = subtract/clamp/retain/scar/expire/reload
Numeric payload backend = CPU first, native GPU second, EML only if validated
```

This remains **provisional** until validated in an ADR and tested against current engine constraints.
