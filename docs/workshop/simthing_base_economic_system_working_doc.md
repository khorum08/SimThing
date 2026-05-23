# SimThing Base Economic System: Resource Balances, Transfers, Thresholds, and Fission

## Purpose

This working document canonizes the emerging base economic model for SimThing.

It is written for two audiences at once:

1. **Modders and designers**, who need an evocative mental model for authoring economies without thinking in terms of hardcoded subsystems.
2. **Implementation agents and maintainers**, including Claude, Cursor, and Codex, who need precise architectural boundaries, invariants, and implementation order.

The central conclusion is simple:

> The base SimThing economy should not be a separate economy engine. It should be the ordinary SimThing model applied to resource-bearing properties, transfer overlays, thresholds, and boundary effects.

Research, construction, upkeep, shipbuilding, droid cohort production, repairs, terraforming, diplomacy projects, logistics, and population growth are all variations of the same underlying pattern:

> A SimThing has resource balances. Overlays modify flow or transfer resources. Thresholds interpret satisfied, exhausted, completed, or deficient state. Boundary handlers apply consequences such as unlocks, fission, instantiation, or transfer.

---

## Design Thesis

The base economic substrate is not markets, jobs, buildings, pops, trade routes, or money. Those are authored regimes.

The base economic substrate is:

```text
Resource property:
  Amount = balance
  Velocity = flow
  Intensity = stress / volatility / urgency

Cost:
  negative Amount on a target/project SimThing

Production:
  positive Velocity on a resource property

Consumption / upkeep:
  negative Velocity on a resource property

Funding / construction / reinforcement:
  transfer overlay from source resource balance to target negative balance

Completion / exhaustion / deficit:
  threshold over resource Amount, Velocity, or Intensity

Output:
  boundary handler, fission, child instantiation, capability unlock, or transfer

Priority:
  overlay metadata, not a default resource column

Resolver:
  optional higher-level policy layer that computes transfer rates from competing priority claims
```

The strongest canonical sentence:

> SimThings own economic state. Overlays own economic intent. Runtime transfer semantics move balances. Thresholds own meaning. Boundary handlers own consequences.

---

## Core Ontology

### SimThings as Economic Bodies

Every SimThing may be an economic participant if it has resource properties.

A SimThing can be:

- a producer
- a consumer
- a stockpile
- a deficit holder
- a project
- a queue item
- a cohort
- a fleet
- a building
- a population unit
- an institution
- a policy apparatus
- a logistics node
- a construction yard
- a research program

But it only participates in resource transfer semantics if it exposes one or more properties interpreted as resources.

### Resource Properties

A resource property is a normal `SimProperty` with a resource semantic role.

Canonical layout:

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
  Positive amount means surplus, stock, satisfied value, accumulated value, or available balance depending on context.
  Negative amount means requirement, debt, shortage, unsatisfied input, or remaining cost depending on context.

Velocity:
  Net flow.
  Positive velocity generates or restores the resource.
  Negative velocity consumes, drains, degrades, or incurs upkeep.

Intensity:
  Stress, volatility, urgency, instability, or expression strength.
  It should not be used as base allocation priority.
```

The same property kind can represent many roles:

```text
Empire.resource::alloys.Amount = +1200
Planet.resource::alloys.Velocity = +18
Fleet.resource::energy.Velocity = -5
FrigateProject.resource::alloys.Amount = -400
DroidCohortProject.resource::electronics.Amount = -100
```

### Costs as Negative Resource Balances

A cost is not a separate engine-level object by default.

A cost is represented as a negative `Amount` on the consuming/project SimThing.

Example:

```text
FrigateConstruction
  resource::alloys.Amount = -400
  resource::electronics.Amount = -80
  resource::shipyard_capacity.Amount = -60
```

This means:

```text
The project requires 400 alloys, 80 electronics, and 60 shipyard-capacity units.
```

Allocation or transfer moves these amounts toward zero.

Completion is a threshold:

```text
alloys.Amount >= 0
AND electronics.Amount >= 0
AND shipyard_capacity.Amount >= 0
```

Once the threshold fires, a boundary handler creates or transfers the completed output.

### Production and Consumption as Velocity

Generation and upkeep are simply velocity.

```text
AlloyFoundry.resource::alloys.Velocity = +8
AlloyFoundry.resource::energy.Velocity = -2
Fleet.resource::fuel.Velocity = -4
Colony.resource::food.Velocity = +12
StarvingPopulation.resource::food.Velocity = -3
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

BlockadeOverlay:
  resource::imports.Velocity -= 10
```

The base simulation rule remains unchanged:

```text
Amount += Velocity * dt
```

Do not reinterpret velocity as priority. Velocity is flow.

### Thresholds as Meaning

Thresholds are where resource state becomes game consequence.

Examples:

```text
resource::alloys.Amount >= 0
  -> construction material requirement satisfied

resource::food.Amount < 0
  -> activate hunger pressure overlay

resource::energy.Amount < -100
  -> underpowered crisis event

resource::engineering_research.Amount >= 0
  -> unlock tech capability

resource::droid_units.Amount >= target_count
  -> cohort ready

resource::readiness.Intensity > 0.8
  -> fission unrest / morale collapse / emergency event
```

Thresholds should remain the place where completion, deficit, exhaustion, overflow, unlock, fission, and actions are interpreted.

---

## Resource Transfer, Not a Heavy Economy Resolver

The design discussion began with a proposed `ResourceAllocationResolver`. That concept remains useful, but the base model can be much leaner.

The fundamental primitive should be **resource transfer**, not a hardcoded economy resolver.

### Transfer Overlay

A transfer overlay moves resource balance from a source property to a target property.

Conceptual shape:

```text
TransferOverlay:
  source: SimThing.resource::<key>.Amount or flow
  target: SimThing.resource::<key>.Amount
  rate: amount per tick/day OR share/cap
  constraints:
    - source availability
    - target remaining deficit
    - scope/ownership/capability
  lifecycle:
    - permanent until changed
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

Where target demand is naturally derived from negative balance:

```text
pull = max(0, -target.Amount)
```

So the base economy can operate without a complex allocator:

```text
1. Resource values integrate velocity.
2. Transfer overlays move balances.
3. Thresholds detect satisfied or failed states.
4. Boundary handlers apply consequences.
```

### When Is a Resolver Still Needed?

A full allocation resolver is only needed when transfer rates are not explicitly authored or player-directed.

For example, no resolver is needed when an overlay says:

```text
Move up to 20 alloys/day from PlanetStockpile to FrigateConstruction.
```

A resolver is useful when the system says:

```text
Empire has 100 alloys/day.
Five shipyards, three repair projects, and two emergency defenses all want alloys.
Distribute available supply according to active priorities, weights, caps, policy, and scope.
```

In that case, the resolver is not the base economy. It is a higher-level scheduling layer that converts competing allocation claims into concrete transfer rates.

### Revised Resolver Role

The optional resolver should be defined as:

```text
AllocationResolver:
  reads active allocation/claim overlays
  reads source pools and target negative balances
  applies priority/weight/caps/eligibility
  emits or executes bounded transfer operations
```

It should not:

```text
unlock technologies directly
spawn buildings directly
create ships directly
decide narrative outcomes directly
hardcode research/construction/upkeep logic
own permanent priority/demand/allocated columns by default
```

The resolver computes transfer. Thresholds and boundary handlers own consequences.

---

## Resource Participation Rule

Only resource properties participate in resource transfer semantics.

```text
No Resource property -> no resource transfer participation.
No transfer/allocation overlay -> no routed resource flow.
Thresholds can still wait on ordinary prerequisites.
```

This separates resource gates from non-resource gates.

### Resource-Satisfied Gate

```text
FrigateProject
  resource::alloys.Amount = -400
  resource::labor.Amount = -100

Threshold:
  alloys.Amount >= 0
  labor.Amount >= 0
  -> complete frigate
```

This participates in transfer.

### Non-Resource Prerequisite Gate

```text
FrigateProject prerequisites:
  capability::basic_frigate_unlocked == true
  orbital_shipyard_available == true
  no_blockade == true
```

These do not participate in resource transfer. They are ordinary threshold conditions.

### Mixed Gate

Most meaningful projects will use both:

```text
FrigateProject
  Resource requirements:
    alloys >= 0
    electronics >= 0
    shipyard_capacity >= 0

  Non-resource prerequisites:
    frigate_design_unlocked == true
    drydock_available == true
    no_blockade == true

  Completion threshold:
    all resource balances satisfied
    AND all prerequisites satisfied
```

---

## Queue-able Construction

Ships, cohorts, droids, armies, buildings, modules, repairs, and reinforcements should be queue-able construction participants.

The queue item is not the completed unit. It is a construction/deployment SimThing or queue entry that resolves into completed output.

### Canonical Queue Item

```text
QueueableConstructionItem:
  output_kind: Frigate | DroidBatch | CohortReinforcement | HabitatModule
  desired_count: N
  completed_count: M
  target: extant SimThing or parent container
  per_unit_or_per_batch_resource_balances:
    resource::alloys.Amount = -400
    resource::electronics.Amount = -80
    resource::labor.Amount = -120
  completion_threshold:
    all required resource balances >= 0
  completion_effect:
    instantiate child OR increment target amount
```

### Per-Unit Completion

For ships:

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

### Per-Batch Completion

For droids or cohorts:

```text
Build Droid Batch

Batch output:
  output_count = 25 droids

Batch debt:
  resource::alloys.Amount = -50
  resource::electronics.Amount = -25
  resource::assembly_labor.Amount = -12.5
  resource::activation_compute.Amount = -5

On threshold satisfaction:
  target_cohort.resource::droid_units.Amount += 25
  completed_count += 25
  if completed_count < desired_count:
    reload next batch debt
  else:
    expire queue item
```

This lets the same model handle:

```text
new ship construction
army recruitment
droid cohort manufacturing
cohort reinforcement
building construction
module installation
repairs
replacement crews
terraforming phases
```

### Reinforcement Is Construction Into an Extant Target

Reinforcement is not a separate system.

```text
ExtantCohort
  resource::droid_units.Amount = 63
  resource::droid_units.Capacity = 100

ReinforcementQueueItem
  desired_count = 37
  target = ExtantCohort
  per_batch_resource_debt:
    resource::alloys.Amount = -50
    resource::electronics.Amount = -25

Threshold completion:
  ExtantCohort.resource::droid_units.Amount += 25
```

Building a new cohort and reinforcing an existing cohort differ only in target and boundary effect.

---

## Parent/Child Production and Aggregation

A SimThing can produce resources through its own properties and through attached children.

Canonical rule:

> Parent-level resource availability may be derived from the reducible state of descendant SimThings, subject to authored scope, ownership, boundary, capability, logistics, and overlay rules.

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

But aggregation should be authored or declared, not assumed for every property.

Questions to settle in implementation:

```text
Is this resource exportable upward?
Does the child reserve local needs first?
Does ownership permit parent allocation?
Does blockade/logistics reduce export?
Does autonomy/corruption tax the flow?
Does local policy redirect it?
```

Overlays can modify aggregation:

```text
BlockadeOverlay:
  exported_food.Velocity *= 0.2

AutonomyOverlay:
  parent_transfer_share *= 0.5

CorruptionOverlay:
  resource::tax_revenue.Velocity -= corruption_loss
```

---

## Authored Economic Regimes Built on the Base

The base model should not hardcode domain systems.

Avoid adding:

```text
ResearchEngine
ConstructionEngine
UpkeepEngine
SimThingEconomyEngine
HardcodedMarketEngine
HardcodedShipbuildingEngine
```

Instead, author regimes on the same primitives.

### Research

```text
IonDriveResearch
  resource::engineering_research.Amount = -100

TransferOverlay:
  source = Empire.resource::engineering_research
  target = IonDriveResearch.resource::engineering_research
  rate/priority = player or AI authored

Threshold:
  engineering_research.Amount >= 0
  prerequisites satisfied
  -> unlock ion_drive capability
```

### Construction

```text
AlloyFoundryProject
  resource::minerals.Amount = -300
  resource::labor.Amount = -100

Threshold:
  minerals >= 0
  labor >= 0
  -> instantiate AlloyFoundry building
```

### Shipbuilding

```text
FrigateConstruction
  resource::alloys.Amount = -400
  resource::electronics.Amount = -80
  resource::shipyard_capacity.Amount = -60

Threshold:
  all resource balances >= 0
  -> instantiate Frigate child under Fleet or reserve
```

### Droid Cohort Manufacturing

```text
DroidBatchConstruction
  output_count = 25
  resource::alloys.Amount = -50
  resource::electronics.Amount = -25
  resource::activation_compute.Amount = -5

Threshold:
  all resource balances >= 0
  -> target_cohort.resource::droid_units.Amount += 25
```

### Upkeep

```text
Fleet
  resource::energy.Velocity = -5
  resource::munitions.Velocity = -1

Threshold:
  energy.Amount < 0
  -> activate underpowered overlay

Threshold:
  munitions.Amount < -50
  -> combat effectiveness penalty
```

### Diplomacy Project

```text
TreatyNegotiation
  resource::influence.Amount = -120
  resource::trust.Amount = -40

Threshold:
  influence >= 0
  trust >= 0
  legitimacy_prereq satisfied
  -> activate treaty overlay
```

### Terraforming

```text
TerraformingPhase
  resource::volatiles.Amount = -500
  resource::engineering_work.Amount = -300
  resource::biosphere_stability.Amount = -100

Threshold:
  all balances >= 0
  -> activate next planetary habitability overlay
```

---

## Transfer Overlays Versus Allocation Claims

There are two levels of authoring.

### Direct Transfer Overlay

Use when the transfer is explicit.

```text
TransferOverlay:
  source = Planet.resource::alloys
  target = FrigateConstruction.resource::alloys
  rate = 20/day
  stop_when = target.Amount >= 0
```

This needs no allocation resolver beyond core transfer execution.

### Allocation Claim Overlay

Use when the system must distribute contested supply.

```text
AllocationClaimOverlay:
  source_pool = Empire.resource::alloys
  target = FrigateConstruction.resource::alloys
  priority = 90
  weight = 1.0
  cap = 20/day
```

A higher-level resolver may convert claims into concrete transfers:

```text
available = source_pool.available
pull = max(0, -target.Amount)
raw_weight = priority_curve(priority) * weight * remaining_factor(pull)
share = raw_weight / sum(raw_weight for pool)
transfer = min(available * share, cap, pull)
```

Then execute:

```text
source.Amount -= transfer
target.Amount += transfer
```

### Priority Belongs to Overlays

Priority should not be a default column on every resource or project.

Priority is an instruction, not intrinsic resource state.

```text
Good:
  BuildFrigatePriorityOverlay(priority = 90)

Avoid by default:
  FrigateConstruction.resource::alloys.Priority = 90
```

Only promote priority into resident property state if it must itself be simulated, reduced, thresholded, or AI-observed as state.

---

## Column Discipline

Do not bloat the dense property matrix.

Default resource layout should stay lean:

```text
Amount
Velocity
Intensity
```

Avoid adding default resident columns such as:

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

Example advanced diagnostic resource:

```text
resource::alloys
  Amount
  Velocity
  Intensity
  Named("production")
  Named("upkeep")
  Named("waste")
  Named("suppression")
```

But this should not be the base requirement.

---

## SimProperty Kind / Metadata Recommendation

The architecture likely needs a way to identify resource properties.

Possible approaches:

```text
SimPropertyKind::Resource
```

or less invasively:

```text
PropertyMetadata.economic_role = Resource
```

or:

```text
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

Do not hardcode domain resources into core enums.

Avoid:

```text
ResourceKind::Alloys
ResourceKind::Food
ResourceKind::Influence
```

Prefer authored keys:

```text
resource::alloys
resource::food
resource::engineering_research
resource::legitimacy
resource::activation_compute
resource::shipyard_capacity
```

---

## Boundary Effects

Thresholds should hand off to existing boundary mechanisms.

Completion effects may include:

```text
activate overlay
expire overlay
unlock capability
instantiate child SimThing
transfer child SimThing
increment target property
reload queue debt
expire queue item
fission a completed object from project state
merge/fuse into extant cohort
emit event
```

### Queue Completion Boundary

Conceptual boundary handler:

```text
OnQueueItemSatisfied:
  if output_mode == InstantiateChild:
    create output SimThing from template
    attach to target parent

  if output_mode == IncrementTargetAmount:
    target.property += output_count

  completed_count += output_count

  if completed_count < desired_count:
    reload per-unit or per-batch negative resource balances
  else:
    expire queue item
```

This is where construction becomes fission/transfer in practice.

---

## Runtime Phase Model

A likely runtime order:

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

The exact phase order should respect the current SimThing engine invariants, but the conceptual ordering should remain:

```text
state movement before threshold interpretation;
threshold interpretation before boundary consequences.
```

---

## Modder Mental Model

A modder should be able to think this way:

> A resource is a balance. Positive means you have it. Negative means you need it. Velocity says whether it is rising or falling. Transfers pay debts. Thresholds decide when a debt is satisfied or a shortage becomes dangerous.

Examples:

```text
A factory is a SimThing whose resources flow.
A project is a SimThing with negative resource balances.
A construction queue is a list of projects that reload their debt after each completed output.
A ship is born when a project crosses zero and fissions into a fleet.
A droid cohort grows when completed batches transfer unit count into an extant cohort.
A famine begins when food stays below zero long enough to cross a threshold.
A technology unlocks when research debt reaches zero and prerequisites are satisfied.
```

The modder-facing slogan:

> To build something, give it a resource debt. To fund it, transfer resources into it. To finish it, watch it cross zero.

---

## Implementation Handoff Notes

### Do First: ADR

Create an ADR, likely:

```text
docs/adr/resource_balance_transfer_model.md
```

Possible title:

```text
ADR: Resource Balances, Transfer Overlays, and Thresholded Construction
```

The ADR should establish:

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
```

### Likely Spec Additions

Possible authored structs:

```text
ResourcePropertySpec
TransferOverlaySpec
AllocationClaimOverlaySpec
QueueConstructionSpec
QueueOutputSpec
ResourceRequirementSpec
ResourceAggregationSpec
```

But avoid overbuilding. The first implementation may need only:

```text
resource semantic tag/kind
transfer overlay spec
queue completion boundary fixture
examples/tests
```

### Likely Driver Additions

```text
Transfer execution phase
Queue completion boundary handler
Diagnostic events for transfer and completion
Logical APIs:
  enqueue_construction
  set_transfer_rate
  suspend_transfer
  resume_transfer
  set_allocation_priority later
```

### Likely Core Changes

Prefer minimal core changes.

Possibly:

```text
resource semantic metadata or property tag
transfer transform representation if current overlays cannot express source->target movement
```

Avoid adding domain-specific economy concepts to core.

### Likely GPU Changes

None in v1.

Later, if needed:

```text
compact transfer work-item buffer
compact allocation claim buffer
GPU transfer pass
GPU group reduction for contested allocation
```

Do not add broad priority/demand/allocated/cost columns to the dense matrix by default.

---

## Open Design Questions

1. **Property tagging**
   - Should `Resource` be a `SimPropertyKind`, metadata tag, or spec-layer convention?

2. **Negative amount interpretation**
   - How do stockpiles, projects, debts, shortages, and requirements distinguish their use of negative Amount?
   - Is that contextual by SimThing role, property metadata, or threshold semantics?

3. **Transfer overlay representation**
   - Can current overlay transforms express source-to-target transfer?
   - Or is a new semantic overlay kind needed?

4. **Source availability**
   - Can transfers draw from Amount, Velocity, or both?
   - How are stockpile-based and flow-based transfers distinguished?

5. **Aggregation**
   - Are child resources automatically reducible upward, or only when authored?
   - How are local reserves, export rules, ownership, logistics, and blockades represented?

6. **Queue item representation**
   - Is a queue item always a SimThing child?
   - Or can it be a property/subfield for lightweight queues?

7. **Completion reload semantics**
   - On per-unit completion, does the same queue item reload negative balances?
   - Or does it fission a new queue item for the next unit?

8. **Discrete versus count outputs**
   - Ships likely instantiate child SimThings.
   - Droids/cohorts likely increment a count property.
   - What interface should boundary handlers use for both?

9. **Priority and allocation**
   - What is the minimal v1: direct transfer only, or claim-to-transfer priority resolver?
   - If priority exists, should it live only in overlay metadata?

10. **Diagnostics**
    - Should allocated amount this tick be emitted as event diagnostics rather than resident columns?

---

## Recommended Implementation Ladder

### A0 — ADR / Design Only

Write the ADR before implementation.

Focus on:

```text
resource balances
negative cost
velocity as flow
transfer overlays
threshold completion
queue outputs
optional allocation resolver
column discipline
```

### A1 — Minimal Resource Tagging

Add a way to mark a property as resource-like.

Goal:

```text
resource properties can be discovered by transfer/queue specs
```

### A2 — Transfer Overlay Spec

Add a semantic transfer overlay or equivalent spec construct.

Goal:

```text
source.Amount -= x
target.Amount += x
bounded by source availability and target pull
```

### A3 — Direct Transfer Runtime

Implement direct transfer execution in driver/session runtime.

No priority allocation yet.

### A4 — Queue Construction Fixture

Implement a simple fixture:

```text
FrigateConstruction starts at alloys = -400.
Transfer overlay routes alloys into it.
Threshold at 0 instantiates Frigate.
```

### A5 — Batch/Cohort Fixture

Implement:

```text
DroidBatch starts with negative resource balances.
Threshold at 0 increments target cohort.droid_units by batch size.
Queue reloads until desired_count reached.
```

### A6 — Aggregation Fixture

Implement child production aggregation into parent resource availability.

### A7 — Optional Allocation Claim Resolver

Only after direct transfer works.

Implement contested allocation as a thin policy layer producing transfer rates.

### A8 — Modder Guide / Examples

Add examples showing:

```text
research as resource debt
construction as resource debt
shipbuilding as queue/fission
cohort reinforcement as count transfer
upkeep shortage as negative flow + threshold
```

---

## Non-Goals

Do not implement in v1:

```text
hardcoded economy engine
hardcoded research engine
hardcoded construction engine
hardcoded market system
hardcoded job/pop system
GPU allocation pass
universal priority columns
universal demand columns
universal allocated columns
complex market price formation
```

Do not make the resolver a normal SimThing.

The runtime transfer/claim resolver is beneath SimThings as execution semantics. SimThings may own policies, institutions, queues, and claims, but the transfer machinery itself is runtime logic.

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
```

A ship is not built by a shipbuilding engine.

A ship is born when a construction SimThing’s resource debts are paid to zero and a threshold fissions the completed vessel into the fleet.

A droid cohort is not filled by a recruitment engine.

It grows when batch debts cross zero and boundary handling transfers completed unit count into the extant cohort.

A technology is not unlocked by a research engine.

It is unlocked when research resource debt reaches zero and prerequisite thresholds are satisfied.

This is the SimThing economic principle:

> Give a thing a debt, give the world a flow, route the flow by overlays, and let thresholds decide what becomes real.

