# SimThing Base Economic System: Resource Balances, Transfers, Thresholds, and Fission

## Purpose

This working document captures the current base-economy synthesis for SimThing. It is meant to be readable by modders while still being precise enough for Claude, Cursor, Codex, or a future implementation chat.

The conclusion:

> The base SimThing economy should not be a separate economy engine. It should be ordinary SimThing semantics applied to resource-bearing properties, transfer overlays, thresholds, and boundary effects.

Research, construction, upkeep, shipbuilding, cohort production, repairs, terraforming, logistics, diplomacy projects, and population growth can all be authored as the same pattern:

> A SimThing has resource balances. Overlays modify flow or transfer balances. Thresholds interpret completion, shortage, exhaustion, or overflow. Boundary handlers apply consequences such as unlocks, fission, instantiation, count increments, or transfer.

---

## Canonical Model

```text
Resource property:
  Amount = balance
  Velocity = net flow
  Intensity = stress / volatility / urgency

Cost:
  negative Amount on a target/project SimThing

Production:
  positive Velocity

Consumption / upkeep:
  negative Velocity

Funding / construction / reinforcement:
  transfer overlay from source resource balance to target negative balance

Completion / deficit / exhaustion:
  threshold over Amount, Velocity, or Intensity

Output:
  boundary handler, fission, child instantiation, capability unlock, count increment, or transfer

Priority:
  overlay metadata, not a default resource column

Resolver:
  optional later policy layer that converts competing claims into transfer rates
```

Shortest form:

> Resource = Amount + Velocity + Intensity. Cost = negative Amount. Generation = positive Velocity. Consumption = negative Velocity. Funding = transfer overlay. Completion = threshold at zero. Output = boundary/fission/transfer.

---

## Resource Properties

A resource property is a normal `SimProperty` with a resource semantic role or tag.

Canonical layout:

```text
resource::<key>
  Amount
  Velocity
  Intensity
```

`Amount` is the current balance. Positive amount means surplus, stock, accumulated value, satisfied value, or available balance depending on context. Negative amount means requirement, debt, shortage, unsatisfied input, or remaining cost depending on context.

`Velocity` is flow. Positive velocity generates or restores the resource. Negative velocity consumes, drains, degrades, or incurs upkeep.

`Intensity` remains stress, volatility, urgency, or expression strength. Do not use Intensity as base allocation priority.

Examples:

```text
Empire.resource::alloys.Amount = +1200
Planet.resource::alloys.Velocity = +18
Fleet.resource::energy.Velocity = -5
FrigateProject.resource::alloys.Amount = -400
CohortProject.resource::electronics.Amount = -100
```

---

## Costs as Negative Balances

A cost is represented as a negative `Amount` on the project or consuming SimThing.

```text
FrigateConstruction
  resource::alloys.Amount = -400
  resource::electronics.Amount = -80
  resource::shipyard_capacity.Amount = -60
```

This means the project needs 400 alloys, 80 electronics, and 60 shipyard-capacity units. Transfers move those balances toward zero.

Completion is just a threshold:

```text
alloys.Amount >= 0
AND electronics.Amount >= 0
AND shipyard_capacity.Amount >= 0
```

Once the threshold fires, a boundary handler creates or transfers the completed output.

This removes the need for a special project-progress abstraction in the base case. Progress is the movement of a negative balance toward zero.

---

## Velocity as Flow

Generation and consumption are velocity.

```text
AlloyFoundry.resource::alloys.Velocity = +8
AlloyFoundry.resource::energy.Velocity = -2
Fleet.resource::fuel.Velocity = -4
Colony.resource::food.Velocity = +12
```

Overlays modify velocity:

```text
UnrestOverlay:
  resource::alloys.Velocity -= 5

AutomationTechOverlay:
  resource::alloys.Velocity += 3

RationingPolicyOverlay:
  resource::food.Velocity += 2
  population::satisfaction.Velocity -= 0.1
```

The normal simulation rule remains:

```text
Amount += Velocity * dt
```

Do not reinterpret Velocity as priority. Velocity is flow.

---

## Thresholds as Meaning

Thresholds are where resource state becomes consequence.

```text
resource::alloys.Amount >= 0
  -> material requirement satisfied

resource::food.Amount < 0
  -> hunger pressure

resource::energy.Amount < -100
  -> underpowered crisis

resource::engineering_research.Amount >= 0
  -> unlock tech capability

resource::unit_count.Amount >= target_count
  -> cohort ready
```

Thresholds remain the owner of completion, deficit, exhaustion, overflow, unlock, fission, and action semantics.

---

## Transfer Overlays, Not a Heavy Economy Resolver

The fundamental primitive should be resource transfer, not a hardcoded economy resolver.

Conceptual shape:

```text
TransferOverlay:
  source: SimThing.resource::<key>.Amount or flow
  target: SimThing.resource::<key>.Amount
  rate: amount per tick/day OR share/cap
  constraints:
    - source availability
    - target remaining deficit
    - scope / ownership / capability
  lifecycle:
    - until target reaches zero
    - until source exhausted
    - while policy active
    - while queue item active
```

Runtime effect:

```text
x = bounded_transfer_amount(source, target, rate, constraints)
source.Amount -= x
target.Amount += x
```

Target pull is derived from negative balance:

```text
pull = max(0, -target.Amount)
```

Base economy loop:

```text
1. Integrate resource velocities.
2. Execute transfer overlays.
3. Evaluate thresholds.
4. Apply boundary effects.
```

A full allocation resolver is only needed later when rates are not explicit and multiple targets compete for the same pool. In that case, the resolver should convert priority claims into transfer rates. It should not unlock techs, spawn buildings, create ships, or hardcode domain behavior.

---

## Resource Participation Rule

Only resource properties participate in resource transfer semantics.

```text
No Resource property -> no resource transfer participation.
No transfer/allocation overlay -> no routed resource flow.
Thresholds can still wait on ordinary prerequisites.
```

Resource gate:

```text
FrigateProject
  resource::alloys.Amount = -400
  resource::labor.Amount = -100

Threshold:
  alloys.Amount >= 0
  labor.Amount >= 0
  -> complete frigate
```

Non-resource prerequisite gate:

```text
capability::basic_frigate_unlocked == true
orbital_shipyard_available == true
no_blockade == true
```

Most projects will use both resource balances and non-resource prerequisite thresholds.

---

## Queue-able Construction

Ships, cohorts, armies, buildings, modules, repairs, and reinforcements should be queue-able construction participants.

The queue item is not the completed unit. It is a construction/deployment SimThing or queue entry that resolves into completed output.

```text
QueueableConstructionItem:
  output_kind
  desired_count
  completed_count
  target: extant SimThing or parent container
  per-unit or per-batch negative resource balances
  completion threshold: all required balances >= 0
  completion effect: instantiate child OR increment target amount
```

Per-unit example:

```text
Build 3 Frigates

Per frigate debt:
  resource::alloys.Amount = -400
  resource::electronics.Amount = -80
  resource::shipyard_capacity.Amount = -60

On threshold satisfaction:
  instantiate Frigate SimThing
  attach to target Fleet or reserve pool
  completed_count += 1
  if completed_count < desired_count:
    reload next frigate debt
  else:
    expire queue item
```

Per-batch example:

```text
Build Cohort Batch

Batch output:
  output_count = 25 units

Batch debt:
  resource::alloys.Amount = -50
  resource::electronics.Amount = -25
  resource::assembly_labor.Amount = -12.5

On threshold satisfaction:
  target_cohort.resource::unit_count.Amount += 25
  completed_count += 25
  reload or expire queue item
```

Reinforcement is the same system: construction into an extant target rather than creation of a new target.

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

So the economic model still needs explicit transfer semantics for:

```text
source.Amount -= x
target.Amount += x
```

Fission/fusion remains relevant as the structural boundary family for completed outputs, but construction needs new or extended effects such as:

```text
TransferAmount
IncrementTargetProperty
InstantiateChildFromTemplate
ReloadQueueDebt
ExpireQueueItem
```

---

## Parent/Child Production and Aggregation

A SimThing can produce resources through its own properties and through attached children.

Canonical rule:

> Parent-level resource availability may be derived from reducible descendant state, subject to authored scope, ownership, boundary, capability, logistics, and overlay rules.

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
Is this resource exportable upward?
Does the child reserve local needs first?
Does ownership permit parent allocation?
Do blockade/logistics reduce export?
Do autonomy/corruption tax the flow?
Does local policy redirect it?
```

---

## Authored Regimes Built on the Base

Do not hardcode domain systems such as `ResearchEngine`, `ConstructionEngine`, `UpkeepEngine`, `MarketEngine`, or `ShipbuildingEngine` in v1.

Instead, author regimes on the same primitives.

Research:

```text
IonDriveResearch
  resource::engineering_research.Amount = -100

TransferOverlay:
  source = Empire.resource::engineering_research
  target = IonDriveResearch.resource::engineering_research

Threshold:
  engineering_research.Amount >= 0
  prerequisites satisfied
  -> unlock ion_drive capability
```

Construction:

```text
AlloyFoundryProject
  resource::minerals.Amount = -300
  resource::labor.Amount = -100

Threshold:
  minerals >= 0
  labor >= 0
  -> instantiate AlloyFoundry building
```

Shipbuilding:

```text
FrigateConstruction
  resource::alloys.Amount = -400
  resource::electronics.Amount = -80
  resource::shipyard_capacity.Amount = -60

Threshold:
  all resource balances >= 0
  -> instantiate Frigate child under Fleet or reserve
```

Upkeep:

```text
Fleet
  resource::energy.Velocity = -5
  resource::munitions.Velocity = -1

Threshold:
  energy.Amount < 0
  -> activate underpowered overlay
```

---

## Priority and Allocation Claims

Priority should live on overlays or allocation claims, not as a default column on every project/resource.

Direct transfer:

```text
TransferOverlay:
  source = Planet.resource::alloys
  target = FrigateConstruction.resource::alloys
  rate = 20/day
  stop_when = target.Amount >= 0
```

Contested allocation claim:

```text
AllocationClaimOverlay:
  source_pool = Empire.resource::alloys
  target = FrigateConstruction.resource::alloys
  priority = 90
  weight = 1.0
  cap = 20/day
```

A later resolver may compute:

```text
available = source_pool.available
pull = max(0, -target.Amount)
raw_weight = priority_curve(priority) * weight * remaining_factor(pull)
share = raw_weight / sum(raw_weight for pool)
transfer = min(available * share, cap, pull)
```

Then execute normal transfer.

---

## Column Discipline

Do not bloat the dense property matrix.

Default resource layout should stay lean:

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
allocated_this_tick = diagnostic output
cost = initial negative Amount or construction spec metadata
priority = overlay metadata
```

Optional advanced layouts may exist, but they are opt-in.

---

## Property Kind / Metadata Recommendation

The architecture likely needs a way to identify resource properties.

Possible approaches:

```text
SimPropertyKind::Resource
PropertyMetadata.economic_role = Resource
PropertyTag::Resource
```

Requirements:

```text
A resource property can be used as transfer source or target.
A negative Amount can be interpreted as unsatisfied requirement/debt.
Velocity is interpreted as net flow.
Thresholds can watch resource state normally.
Aggregation specs can identify which resources move upward.
Transfer overlays can refer to resource properties by key/path.
```

Do not hardcode resource keys like `Alloys`, `Food`, or `Influence` into core enums. Prefer authored keys:

```text
resource::alloys
resource::food
resource::engineering_research
resource::legitimacy
resource::activation_compute
resource::shipyard_capacity
```

---

## Runtime Phase Model

Likely order:

```text
1. Apply ordinary overlays to property values and velocities.
2. Integrate velocity into amount.
3. Apply child-to-parent resource aggregation/materialization where authored.
4. Execute direct transfer overlays.
5. Optionally resolve allocation claims into transfers.
6. Evaluate thresholds.
7. Apply boundary handlers: unlock, fission, transfer, instantiate, expire, reload.
8. Update overlay lifecycle.
9. Emit diagnostics/events.
```

Conceptual invariant:

```text
state movement before threshold interpretation;
threshold interpretation before boundary consequences.
```

---

## GPU Position

Do **not** implement this on GPU in v1.

V1 should prove CPU/session semantics first:

```text
resource property semantics
negative Amount as cost/debt
Velocity as flow
direct transfer overlays
threshold completion
boundary/fission/transfer effects
queue fixtures for ships/cohorts
```

GPU transfer/allocation work is a later lowering path only after CPU semantics and fixtures are stable.

Later, if profiling and scale require it, transfer execution may be lowered into:

```text
compact transfer work-item buffer
compact allocation claim buffer
GPU transfer pass
GPU group reduction for contested allocation
```

These are optimization targets, not canonical requirements.

The contested allocation case is especially not v1 because it requires grouping and reduction across claims.

---

## Implementation Handoff

### A0 — ADR / Design Only

Create an ADR:

```text
docs/adr/resource_balance_transfer_model.md
```

It should establish:

```text
1. Resource properties use normal Amount / Velocity / Intensity semantics.
2. Costs are represented as negative Amounts on target/project SimThings.
3. Production and upkeep are represented as Velocity.
4. Transfer overlays move resource balances between SimThings.
5. Completion, shortage, overflow, unlock, fission, and output creation are threshold/boundary activity.
6. Priority belongs to overlays/claims, not default resource columns.
7. A full allocation resolver is optional and only converts competing claims into transfer operations.
8. Queue-able construction is repeated satisfaction of per-unit or per-batch negative balances.
9. Ships and cohorts are outputs of construction queue items, not the queue items themselves.
10. Non-resource prerequisites remain ordinary threshold predicates.
11. GPU property columns remain lean; derived values should not become columns unless needed.
12. V1 is CPU/session-side only; GPU lowering is deferred.
```

### Suggested Implementation Ladder

```text
A1 — Minimal resource tagging
A2 — Transfer overlay spec
A3 — CPU/session direct transfer runtime
A4 — Ship construction fixture
A5 — Batch/cohort fixture
A6 — Aggregation fixture
A7 — Optional CPU allocation-claim resolver
A8 — Modder guide and examples
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

> A resource is a balance. Positive means you have it. Negative means you need it. Velocity says whether it is rising or falling. Transfers pay debts. Thresholds decide when a debt is satisfied or a shortage becomes dangerous.

Examples:

```text
A factory is a SimThing whose resources flow.
A project is a SimThing with negative resource balances.
A construction queue is a list of projects that reload their debt after each completed output.
A ship is born when a project crosses zero and fissions into a fleet.
A cohort grows when completed batches transfer unit count into the extant cohort.
A famine begins when food stays below zero long enough to cross a threshold.
A technology unlocks when research debt reaches zero and prerequisites are satisfied.
```

The modder-facing slogan:

> To build something, give it a resource debt. To fund it, transfer resources into it. To finish it, watch it cross zero.

---

## Final Canonical Summary

The base economic system is the smallest possible economic substrate that still feels alive:

```text
Resource = Amount + Velocity + Intensity
Cost = negative Amount
Generation = positive Velocity
Consumption = negative Velocity
Funding = transfer overlay
Priority = overlay metadata
Completion = threshold at zero
Output = boundary/fission/transfer
Resolver = optional claim-to-transfer scheduler
GPU = later optimization, not v1 semantics
```

A ship is not built by a shipbuilding engine.

A ship is born when a construction SimThing’s resource debts are paid to zero and a threshold fissions the completed vessel into the fleet.

A cohort is not filled by a recruitment engine.

It grows when batch debts cross zero and boundary handling transfers completed unit count into the extant cohort.

A technology is not unlocked by a research engine.

It is unlocked when research resource debt reaches zero and prerequisite thresholds are satisfied.

This is the SimThing economic principle:

> Give a thing a debt, give the world a flow, route the flow by overlays, and let thresholds decide what becomes real.
