# SimThing Base Economic System: Provisional Thresholded Accumulation and Emission

> **Status: PROVISIONAL WORKING DOCUMENT**
>
> This is a design synthesis, not an accepted ADR and not an implementation spec. It records the current direction of thought for the base SimThing economic/causal substrate. Future ADRs should promote, revise, or reject these ideas after inspection against the current engine invariants.

## Purpose

This working document captures the current base-economy synthesis for SimThing. It is meant to be readable by modders while still being precise enough for Claude, Cursor, Codex, or a future implementation chat.

The newest provisional change is important:

> The base economic system may be a special case of a more general intrinsic SimThing facility: **thresholded accumulation and emission**.

The earlier queue/construction model remains useful, but it should not be treated as the root abstraction. Queue construction, factory production, fleet replenishment, weapons damage, hitpoints, population growth, diplomatic relation shifts, research, repair, logistics, and upkeep are all possible authored cases of the same deeper pattern:

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
Emission count = floor(accumulator_amount / unit_size), or other authored formula
Effects = increments, transfers, overlays, fission, instantiation, unlocks, events
Consume mode = subtract, clamp, retain, scar, expire, reload debt, or transfer remainder
```

---

## Why This Is Broader Than Queue Construction

Queue construction is one use case:

```text
unit_progress.Amount >= 1.0
  -> emit completed units into target cohort/fleet/parent
  -> subtract emitted_units from unit_progress
```

But the same facility could express:

```text
factory::assembly_progress.Amount >= 1.0
  -> emit inventory units
  -> subtract emitted units, keep fractional remainder

population::growth.Amount >= 1.0
  -> increment population count
  -> subtract emitted growth units

fleet::replenishment_progress.Amount >= 1.0
  -> increment missile/fighter/crew/readiness stock
  -> subtract emitted units

ship::damage.Amount >= armor_break_threshold
  -> attach hull_breach overlay or emit damage event
  -> retain, scar, or subtract damage depending on authored consume mode

diplomacy::trust.Amount >= treaty_threshold
  -> activate treaty_available overlay or emit diplomatic event
  -> retain, clamp, or spend trust depending on authored consume mode

research::ion_drive_progress.Amount >= 1.0
  -> unlock ion_drive capability
  -> expire research item or retain completion marker
```

The facility should be intrinsic because it is not a Stellaris-like queue rule. It is a general SimThing pattern for anything that accumulates state and turns threshold crossings into consequences.

---

## Provisional Emit-On-Threshold Model

An authored/runtime emission spec might look like this conceptually:

```text
ThresholdedEmissionSpec:
  owner: SimThingId
  watched_property: SimPropertyId
  watched_subfield: Amount | Velocity | Intensity | Named(...)
  threshold: f32
  direction: Rising | Falling
  unit_size: f32
  max_emit_per_boundary: optional
  output_count_formula: optional
  consume_mode: ConsumeMode
  effects: Vec<EmissionEffect>
```

On threshold fire:

```text
value = read watched property/subfield
emit_count = compute_emit_count(value, unit_size, max_emit_per_boundary)
apply effects scaled by emit_count
apply consume/reset mode to watched value
```

For the common one-column accumulator case:

```text
emit_count = floor(value / unit_size)
```

Example:

```text
unit_progress.Amount = 11.5
unit_size = 1.0
emit_count = 11
```

Boundary result:

```text
target_cohort.unit_count += 11
queued_count -= 11, if bounded by a queue count
unit_progress.Amount -= 11
unit_progress.Amount remains 0.5
```

This gives one completion column and one threshold while still allowing multi-unit emission in one boundary.

---

## Consume / Reset Modes

The consume/reset rule is the key to making the primitive general.

Provisional consume modes:

```text
SubtractEmitted:
  value -= emit_count * unit_size
  Keeps fractional remainder. Good for production, growth, replenishment.

ClampToZero:
  value = 0 after firing. Good for one-shot pressure discharge.

Retain:
  value remains. Good for persistent gates such as diplomacy states or tech completion markers.

Expire:
  remove property, queue item, or process after firing. Good for one-shot projects.

Scar:
  value *= coefficient. Similar to current fusion scar semantics.

ReloadDebt:
  value -= next_required_amount. Good for repeated batch queues or staged projects.

TransferRemainder:
  move overflow/remainder to another property or SimThing.
```

This should be designed as boundary semantics, not as ordinary numeric overlay logic.

---

## Emission Effects

Effects should be generic and composable. Provisional examples:

```text
IncrementProperty:
  target property += amount_per_emit * emit_count

TransferAmount:
  source property -= amount_per_emit * emit_count
  target property += amount_per_emit * emit_count

InstantiateChild:
  create child from template, count = emit_count or one per emit

AttachOverlay / ActivateOverlay / SuspendOverlay:
  modify active overlay state

ActivateCapability:
  unlock or activate a capability/effect

EmitEvent:
  produce a scripted or feeder event

Fission / StructuralMutation:
  spawn, split, reparent, remove, or fuse SimThings

ExpireSelf:
  remove or tombstone the emitting item/process after settlement
```

The key rule:

> Thresholded emission should move through existing boundary protocols wherever possible. It should not become a hidden domain engine.

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

Examples:

```text
Empire.resource::alloys.Amount = +1200
Planet.resource::alloys.Velocity = +18
Fleet.resource::energy.Velocity = -5
FrigateProject.resource::alloys.Amount = -400
CohortProject.resource::electronics.Amount = -100
```

Cost-as-negative-balance is still useful:

```text
FrigateConstruction
  resource::alloys.Amount = -400
  resource::electronics.Amount = -80
  resource::shipyard_capacity.Amount = -60
```

But this is no longer the only possible representation. A queue/factory may instead convert inputs into a normalized one-column accumulator:

```text
queue::unit_progress.Amount = 0.0
threshold: unit_progress >= 1.0
```

Multi-resource cost can live in transfer/conversion metadata that feeds normalized progress.

---

## Overlays Still Own Input and Intent

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

## Current Threshold Action Model

Current engine behavior is important for implementation planning:

```text
GPU Pass 7 emits ThresholdEvent { slot, col, value, event_kind }.
CPU ThresholdRegistry maps event_kind -> ThresholdSemantic.
Boundary phases/handlers interpret the semantic action.
```

Existing semantic arms include fission, fusion, property expiry, velocity/aggregate alerts, capability unlocks, and scripted event triggers.

A provisional thresholded-emission facility would likely require a new semantic arm such as:

```rust
ThresholdSemantic::EmitOnThreshold { emitter_id, spec_id }
```

or:

```rust
ThresholdSemantic::ThresholdedEmission { emitter_id, spec_id }
```

The GPU event should remain small and opaque. The boundary handler should read the current accumulator value from shadow, compute emit count, apply effects, and update the accumulator according to consume mode.

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

This is the same pattern as population growth or factory output. The difference is only the effect list and consume mode.

---

## Parent/Child Production and Aggregation

A SimThing can produce resources or accumulated value through its own properties and through attached children.

Canonical provisional rule:

> Parent-level availability may be derived from reducible descendant state, subject to authored scope, ownership, boundary, capability, logistics, and overlay rules.

Example:

```text
Empire
  Planet A
    District 1 -> resource::minerals.Velocity = +4
    District 2 -> resource::minerals.Velocity = +6
  Planet B
    District 3 -> resource::minerals.Velocity = +3
```

Empire-level pool may aggregate:

```text
resource::minerals.Velocity = +13
```

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

## GPU Position

Do **not** implement this on GPU in v1.

V1 should prove CPU/session semantics first:

```text
resource/accumulator property semantics
Velocity as flow
one-column thresholded emission
explicit transfer overlays
boundary emission effects
queue/factory/growth fixtures
```

GPU transfer/emission/allocation work is a later lowering path only after CPU semantics and fixtures are stable.

Later, if profiling and scale require it, some pieces may be lowered into:

```text
compact transfer work-item buffer
compact emission work-item buffer
compact allocation claim buffer
GPU transfer pass
GPU group reduction for contested allocation
```

These are optimization targets, not canonical requirements.

---

## Implementation Handoff

### A0 — ADR / Design Only

Create an ADR after this design stabilizes, likely:

```text
docs/adr/thresholded_accumulation_emission.md
```

or, if still framed economically:

```text
docs/adr/resource_balance_transfer_model.md
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
8. Why GPU lowering is deferred.
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
A8 — Modder guide and examples
A9 — Later profiling/GPU lowering only if needed
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
Emit count = floor(value / unit_size), or authored formula
Effects = increments/transfers/overlays/instantiation/fission/unlocks/events
Consume mode = subtract/clamp/retain/scar/expire/reload
```

This remains **provisional** until validated in an ADR and tested against current engine constraints.
