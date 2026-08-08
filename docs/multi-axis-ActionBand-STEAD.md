# Multi-Axis ActionBand + STEAD
## Semantic value axes, recursive RF state, PALMA gradients, and typed multisource commitment

> **Status: WORKSHOP / DESIGN SYNTHESIS — NOT DESIGN-ADMITTED BY THIS DOCUMENT.**
>
> This document consolidates two lines of SimThing design that have repeatedly reappeared as if they were separate problems:
>
> 1. the **Capability Tree / talent / technology unlock** work, which already established the shape of a heterogeneous multi-source action requirement; and
> 2. the **0.0.8.7 RF + STEAD + PALMA + ActionBand** work, which establishes how recursively evaluated SimThing state becomes observable field pressure and action potential.
>
> The additional clarifying model is the familiar **Stellaris-style opposing value-axis web**: a small number of well-chosen axes define a coordinate space; higher-order labels arise from location and motion in that space rather than becoming primitive mechanics.
>
> The thesis of this document is that these are one composition:
>
> ```text
> recursive RF state
>     ↓
> multi-axis semantic projection
>     ↓
> STEAD field bundle / heat volume
>     ↓
> actor/action-specific EML projection
>     ↓
> PALMA potential / impedance gradient
>     ↓
> ActionBand candidate commitment
>     ↓
> typed multisource requirements
>     ↓
> RF clear / hold / disburse
>     ↓
> CostBand executable depth
>     ↓
> ordinary consequence
>     ↓
> new recursive RF state
> ```
>
> This closes the loop without introducing domain planners, bespoke goal systems, or a universal transaction subsystem.

---

## 1. Design question

Can a SimThing world express rich demographic, economic, social, spatial, strategic, or production behavior using:

- a small set of **strong primitive value/resource axes**;
- recursive RF evaluation over SimThing parent/child structure;
- STEAD as the spatial/relational propagation of those values;
- EML as the actor-specific projection of those values into action desirability;
- PALMA as the topology-aware potential/impedance surface over which candidate progress is found;
- and a **typed multisource ActionBand** that distinguishes capability checks, signals, consumables, reservations, transfers, and progress?

The proposed answer is **yes in structure, subject to empirical falsification of the chosen semantic basis and quantitative commitment lowering**.

The important constraint is that the engine must not confuse three different spaces:

1. **semantic / observational field space** — what conditions exist and what they imply;
2. **resource / conservation space** — what can actually be consumed, reserved, transferred, or allocated;
3. **action / progress space** — what an actor can actually commit and complete this generation.

These spaces interact, but they are not interchangeable.

---

## 2. Archaeology: what the Capability Tree already solved

The Capability Tree design is the strongest prior art for multisource action semantics inside SimThing.

A capability tree is one SimThing attached to an owner. The simulation does not know “technology tree,” “talent tree,” or “national ideas.” It sees:

- GPU-tracked progress/rate properties;
- threshold crossings;
- prerequisite predicates;
- suspended overlays;
- and boundary activation of effects.

The domain labels stay in the spec layer.

### 2.1 The already-settled shape

The capability work established the following pattern:

```text
progress / achievement
        ↓
threshold reached
        ↓
all prerequisite conditions checked
        ↓
if all pass
        ↓
atomic consequence activation
```

The important part is that prerequisites were **heterogeneous in meaning but homogeneous in runtime form**. Same-category and cross-category dependencies were not separate engine concepts. They were resolved to ordinary property/value tests.

A technology such as a hypothetical warp drive could require:

```text
propulsion::plasma_drive >= 1
physics::gravitic_theory >= 1
research_progress >= cost
```

The simulation did not need a “cross-category tech prerequisite” subsystem. It needed a conjunction of resolved conditions.

### 2.2 Progress and eligibility were distinct

The capability work also established an important rule that maps directly onto current ActionBand persistence:

> **completed or accumulated work is not the same thing as current eligibility to commit the consequence.**

If progress was complete but another prerequisite was not, the design preserved the achieved progress and waited for the missing prerequisite rather than repeatedly erasing and repaying completed work.

That is the ancestor of the current distinction between:

- accumulated CostBand work/remainder;
- unresolved RF claims;
- current eligibility/guards;
- and later persistence or dissolution.

### 2.3 Consequences could be plural and atomic at the boundary

One capability entry could activate several overlays together:

- unlock a building;
- increase industrial output;
- modify fleet speed;
- enable a construction option.

Again, the simulation did not need separate activation machinery per domain. The consequence bundle was authored data applied at the boundary.

### 2.4 What the Capability Tree did **not** prove

The capability system mostly dealt with:

- nonconsuming prerequisite checks;
- accumulated progress;
- boolean/selectable activation;
- and atomic effect activation.

It did **not** fully prove the hardest quantitative case:

```text
requires simultaneously:
  fuel 4          consumed
  money 20        transferred
  machine slot 1  reserved
  tech X          checked only
  10 work         accumulated
```

when the consumable/reservable quantities are independently contested in RF arenas.

That lower-level atomic-grant problem is real.

But the semantic problem — “how can one action depend on multiple kinds of prerequisites?” — was already mostly solved.

### 2.5 Correct inheritance into 0.0.8.7

The current RF/STEAD/ActionBand work should therefore inherit this rule:

> **Multisource action is settled in shape. The remaining open problem is the quantitative lowering of simultaneous contested grants, not whether heterogeneous requirements can compose.**

---

## 3. Stellaris-style axes as a clarification of semantic field space

The Stellaris ethics web is useful not because SimThing should copy its specific political values, but because it demonstrates a compact representation principle:

> **A small number of logically arranged axes can locate a complex state in a multidimensional value space without naming every composite category.**

Stellaris exposes pairs such as:

```text
Authoritarian  ←→ Egalitarian
Militarist     ←→ Pacifist
Xenophobe      ←→ Xenophile
Spiritualist   ←→ Materialist
```

The useful abstraction is:

```text
state = coordinates on several meaningful axes
```

rather than:

```text
state = one named category
```

For SimThing, the axes need not be ideological and need not all be bipolar. They may be:

- signed adversarial axes;
- bounded complementary axes;
- positive resource/opportunity magnitudes;
- pressures, deficits, or capacities;
- relational values over LinkGraph topology;
- spatial values over GridOffsets topology.

The key is that they are **primitive causal coordinates**, not human descriptive labels.

---

## 4. Strong axes, emergent semantics

A candidate semantic law follows from the recursive SimThing principle:

> **Author strong primitive causal axes and generic resource mechanics; prefer higher-order domain concepts as observations over the interaction and trajectories of those axes.**

This is the semantic analogue of “derive specialists from SimThing rather than minting bespoke object systems.”

### 4.1 Primitive versus derived concepts

For an urban simulation, plausible primitive axes might include:

```text
housing capacity / scarcity
housing condition
land or rent cost
vacancy
employment opportunity
labor availability
consumer demand
capital availability
transit accessibility
freight accessibility
safety / threat
environmental quality
education access
health access
amenity / leisure access
material supply quality
material price pressure
congestion / network pressure
```

By contrast, concepts such as:

```text
desirability
affordability
blight
gentrification
prosperity
food desert
transit-oriented development
commercial vitality
neighborhood decline
```

should normally be **derived projections or temporal observations**, not primitive engine fields.

### 4.2 Example: blight and gentrification

Blight may be observed from a combination such as:

```text
high vacancy
+ poor housing condition
+ weak investment
+ low service access
+ safety pressure
```

Gentrification may be observed as a trajectory in which:

```text
low current property cost
+ increasing accessibility
+ increasing amenity richness
+ strong employment opportunity
+ available capital
        ↓
investment actions increase
        ↓
housing condition rises
amenities rise
prices/rents rise
income composition shifts
incumbent low-income displacement pressure rises
```

No runtime `GentrificationSystem` is required.

The world simply enters a recognizable region and trajectory in the primitive coordinate space.

---

## 5. Recursive RF state produces semantic coordinates

A spatial or relational SimThing should not merely own arbitrary manually authored “heat values.” Its field coordinates should be derived from ordinary recursively evaluated state.

For SimThing `s`, let its recursively evaluated state be:

```text
R_s = {
    own resource properties,
    child reductions,
    parent/ancestor effects,
    overlays,
    RF pressure/allocation state,
    ordinary anchored observables
}
```

Then define a semantic coordinate vector:

\[
S_s = [S_{s,0}, S_{s,1}, ..., S_{s,K-1}]
\]

where each coordinate is a sealed authored EML projection:

\[
S_{s,k} = F_k(R_s)
\]

The important design rule is:

> **The semantic coordinate is a projection of causal state, not an independently authored label.**

This means a neighborhood, company, residency, polity, fleet, or grid cell can become an emitter because its recursive state already summarizes the relevant subtree.

### 5.1 Fractal interpretation

The same pattern applies at different scales:

```text
person / cohort
    ↓
residency
    ↓
block / district
    ↓
city / polity
    ↓
region / faction
```

Each level recursively evaluates its children and can project its resulting state onto an appropriate semantic basis.

The semantic basis may differ by property set or scenario vocabulary, but the mechanism does not.

---

## 6. STEAD as a multi-channel heat volume

STEAD should not collapse the semantic basis into one universal “goodness” scalar.

Instead, define a field tensor over admitted topology:

\[
\Phi(x,k)
\]

where:

- `x` is a grid cell, LinkGraph node, or other admitted locus;
- `k` is a primitive semantic field channel.

Each SimThing emitter contributes to one or more channels according to its current semantic coordinate vector and falloff profile.

For channel `k`:

\[
\Phi_k(x) = \operatorname{Reduce}_{e \in E_k}
\left( A_{e,k} \cdot f_{e,k}(d(e,x)) \right)
\]

where:

- `A_{e,k}` is the emitter’s current projected amplitude on axis `k`;
- `f` is the admitted STEAD falloff/propagation law;
- `Reduce` is the admitted non-conserved accumulation law.

The result is a **multi-axis heat volume**, not a single map.

### 6.1 Opposing axes

Some semantic dimensions are naturally adversarial.

For an opposing pair `a` and `b`, the engine may represent either:

1. two independent nonnegative channels; or
2. one signed differential channel where the authored semantics make that legitimate.

Example:

```text
safety  ←→ threat
abundance ←→ scarcity
integration ←→ isolation
```

The important point is that opposition is authored mathematical structure, not implied by the presence of two human-language labels.

### 6.2 Complementary axes

Other dimensions interact positively:

```text
transit accessibility
+ employment opportunity
+ amenity richness
```

may jointly produce a strong residential or commercial potential even though none is the opposite of another.

Complementarity belongs in the **actor/action projection**, not necessarily in the primitive field storage.

### 6.3 Derived fields are lawful caches, not new authorities

Frequently used projections may be cached as derived field columns:

```text
working_class_housing_potential
commercial_investment_potential
healthcare_accessibility
```

but their authority remains the primitive fields plus their admitted EML projection.

A cached compound field is a lowering/cache, not a new semantic primitive.

---

## 7. Actors do not experience the same field identically

A field channel describes the world. An actor-specific projection describes what that world means to a particular actor/action.

Let actor/cohort `a` carry compact state:

\[
D_a = \text{deficits / needs}
\]

\[
Q_a = \text{profile / preferences / capabilities}
\]

\[
C_a = \text{current commitment / hysteresis}
\]

and observe local field vector:

\[
\Phi(x) = [\Phi_0(x), ..., \Phi_{K-1}(x)]
\]

Then an authored EML projection produces action-specific potential:

\[
P_{a,m}(x) = F_m(\Phi(x), D_a, Q_a, C_a, \text{prices}, \text{local capacity}, ...)
\]

where `m` is an action family or candidate interpretation.

### 7.1 Example: same city, different projections

A low-income household may value:

```text
+ employment access
+ safety
+ school access
- rent burden (high weight)
- transport cost
```

A high-income young professional may value:

```text
+ employment access
+ transit
+ amenities
+ food quality
- rent burden (lower weight)
```

A developer may value:

```text
+ expected demand
+ accessibility
+ amenity growth
+ development capacity
- current land cost
- construction impedance
```

The shared world field does not change.

The projection does.

---

## 8. PALMA: once the projection exists, pathing falls out

PALMA does not need to understand “housing,” “shopping,” “employment,” “diplomacy,” or “investment.”

It requires an admitted topology and a scalar potential/impedance field.

Given actor/action potential `P`, candidate progress along edge `e=(x→y)` can be expressed from:

\[
\Delta P_e = P(y) - P(x)
\]

or through a sealed impedance projection `W(e)` used by the `(min,+)` PALMA sweep.

The action therefore becomes:

```text
multi-axis STEAD field
        ↓
actor/action EML projection
        ↓
scalar potential / impedance
        ↓
PALMA
        ↓
candidate descent / progress direction
```

### 8.1 “Path” is broader than physical movement

The topology may be:

- GridOffsets for physical locality;
- road/transit LinkGraph;
- supply/trade graph;
- social/diplomatic graph;
- recursive tree relations;
- another admitted relation surface.

A path can therefore represent:

- commuting;
- migration;
- goods routing;
- investment flow;
- diplomatic attention;
- production sourcing;
- service access;
- capability derivation.

The core sees only potential over topology.

---

## 9. ActionBand begins where potential meets reality

The semantic field answers:

> **What change is attractive, urgent, or valuable?**

RF + CostBand answer:

> **Can the actor actually commit the resources required to realize it, and how much determinate progress can occur?**

These are intentionally separate.

A high housing desirability potential does not itself create an apartment, money, transport capacity, or legal permission.

ActionBand is the bridge:

```text
potential / candidate
      ↓
typed requirement bundle
      ↓
claims + guards + progress
      ↓
RF / CostBand
      ↓
consequence
```

---

## 10. Typed multisource requirements

The capability-tree archaeology strongly suggests that “requirement” must not be synonymous with “consumable resource.”

A generic action requirement bundle should distinguish at least the following semantic roles.

| Role | Meaning | Example | RF behavior |
|---|---|---|---|
| **Guard / Capability** | observed prerequisite, not consumed | has tech, legal permission, age threshold | read/check only |
| **Signal / Preference** | valuation input, not a prerequisite resource | policy bias, urgency, prestige | EML input only |
| **Consume** | conserved resource destroyed/spent by action | fuel, food, steel | claim → clear → consume |
| **Transfer** | conserved resource changes owner | payment, traded goods | claim → clear → transfer |
| **Reserve** | scarce capacity held/assigned and later released | housing slot, bed, job, machine | claim → clear → hold/assign |
| **Progress / Work** | accumulated determinate work | research, build work, travel progress | CostBand accumulation |
| **Temporal guard** | elapsed time/state condition | generation ≥ T | observe unless time itself is scarce |

The runtime must not infer these roles from names.

They are authored/admitted semantics.

### 10.1 Capability-tree degenerate case

A capability unlock is simply:

```text
Guard: prerequisite tech A
Guard: prerequisite tech B
Progress: research >= threshold
```

with a depth-limited consequence:

```text
N ∈ {0,1}
```

### 10.2 Production case

A recipe may be:

```text
Guard: technology unlocked
Guard: factory operational
Consume: steel 2
Consume: energy 1
Reserve: labor capacity 1
Reserve: machine capacity 1
Progress: production work
```

### 10.3 Household move

A move may be:

```text
Guard: residency legally compatible
Reserve/Transfer: housing slot 1
Transfer: money deposit
Reserve/Consume: transport capacity
Progress: move work
```

The semantic shape is the same.

---

## 11. Vector CostBand is the quantitative all-of gate

The old capability gate was conceptually:

\[
G = \bigwedge_i (x_i \ge t_i)
\]

A depth-1 unlock occurs only if `G=1` and progress is complete.

For quantitative scarce resources, the natural generalization is:

\[
N = G \cdot \min_{j \in Q}
\left\lfloor\frac{V_j}{C_j}\right\rfloor
\]

where:

- `G ∈ {0,1}` is the conjunction of nonconsuming guards;
- `Q` contains only quantitative constrained requirements;
- `V_j` is the provisional/granted available value on lane `j`;
- `C_j` is cost per determinate action quantum;
- `N` is common executable depth.

This is the important reinterpretation:

> **Vector CostBand is not the semantics of multisource action. It is the quantitative lowering of the all-of gate for the subset of requirements that are actually scarce numerical resources.**

The multisource semantic structure was already known.

### 11.1 Disposition remains per lane

The common `N` answers only:

> how many action quanta can execute?

It does not decide what each lane means after commit.

For each lane, authored role determines disposition:

```text
Consume  → subtract N*C
Transfer → move N*C to destination
Reserve  → hold/assign N*C
Progress → advance work state
```

Guards and signals are not consumed.

---

## 12. Provisional grants and the holding problem

The genuinely open multisource quantitative issue is independent RF clears.

Suppose one action requires:

```text
fuel        4
money      20
housing     1
```

and current clears return:

```text
fuel grant     = 4
money grant    = 20
housing grant  = 0
```

The action cannot commit, but already-granted scarce quantities must not be double-spent.

The lawful candidate is:

```text
independent RF clears
        ↓
provisional grants
        ↓
in-flight holding account
        ↓
common executable N
        ↓
commit N*C_j by lane role
        ↓
unused excess returns at next normal resolution opportunity
```

This holding problem is the part that was not already solved by capability prerequisites.

### 12.1 No accidental long-lived hoarding

Same-commit provisional holding must remain distinct from authored persistent reservation.

A failed action must not automatically hold resources across generations forever.

Persistent reservation must itself be an authored `Reserve` behavior or explicit persistence consequence.

### 12.2 No same-generation convergence loop

Do not perform:

```text
clear
→ hold
→ fail common N
→ return
→ re-clear
→ repeat until convergence
```

within one generation.

Returned capacity re-enters the next ordinary resolution opportunity.

Generation pacing remains sovereign.

---

## 13. Full recursive closed loop

The combined model is now:

### 13.1 Recursive state evaluation

```text
children RF state
      ↓
reduce upward
      ↓
parent overlays / policy / prices
      ↓
disburse downward
      ↓
current SimThing state
```

### 13.2 Semantic projection

```text
current recursive state
      ↓
EML semantic-axis projections
      ↓
S = [s0 ... sk]
```

### 13.3 STEAD propagation

```text
semantic-axis amplitudes
      ↓
STEAD falloff / superposition
      ↓
Φ[x,k]
```

### 13.4 Actor-specific action landscape

```text
Φ[x,*]
+ actor deficits
+ profile
+ prices
+ current commitment
      ↓
EML
      ↓
P_action(x)
```

### 13.5 PALMA candidate progress

```text
P_action / W_action
      ↓
PALMA over admitted topology
      ↓
candidate gradient / path
```

### 13.6 Typed multisource commitment

```text
candidate action
      ↓
Guard / Signal / Consume / Transfer / Reserve / Progress
      ↓
RF claims and checks
      ↓
common executable CostBand depth N
```

### 13.7 Consequence

```text
N
 ↓
ordinary state change / transfer / work / reservation
 ↓
structural request if required
 ↓
next generation recursive state
```

This is a closed dynamical system.

---

## 14. CS2-style city simulation as a worked witness

Cities: Skylines II semantics are useful because they appear domain-rich while being unusually reducible to this generic composition.

### 14.1 Candidate primitive urban axes

A deliberately compact city basis might include:

```text
housing abundance / pressure
housing condition
land/rent price pressure
employment opportunity
labor availability
consumer demand
capital/investment access
material supply access
material price/quality
safety / threat
environmental quality
education access
health access
amenity/leisure access
passenger accessibility
freight accessibility
congestion/capacity pressure
```

These are not asserted as the final correct basis. Studio must test them.

### 14.2 Household/cohort state remains small

The population particle need not carry destination lists or private world copies.

A compact row can retain only:

```text
count
residency / household bindings
behavior profile
small deficit vector
personal reserves
current commitment
CostBand remainder / slow hysteresis
```

The world supplies:

- housing opportunity;
- jobs;
- service access;
- commercial opportunity;
- transportation impedance;
- safety;
- prices;
- amenities;
- congestion.

### 14.3 Shopping

```text
material deficit
+ food/material quality-price fields
+ household money
+ PALMA access impedance
      ↓
shopping potential
      ↓
claims: goods + money + transport/time
      ↓
RF / typed commit
      ↓
deficit reduction
```

### 14.4 Employment

```text
income/resource pressure
+ employment field
+ qualification guard
+ commute impedance
      ↓
job action potential
      ↓
claim job capacity
      ↓
clear / relation consequence
```

### 14.5 Housing

```text
shelter pressure
+ rent
+ safety
+ employment/school/service access
+ household resources
      ↓
housing potential
      ↓
PALMA candidate destination
      ↓
housing reservation + money transfer + move work
      ↓
RF / CostBand
      ↓
residency reparent at boundary
```

### 14.6 Traffic

Strategic movement:

```text
need/action target
      ↓
PALMA generalized impedance
      ↓
transport demand
      ↓
road/transit capacity claims
```

Microscopic local traffic:

```text
position / lane / local occupancy / signal
      ↓
Wei-style local automata
```

PALMA answers where flow wants to go.

RF answers how much capacity is available.

Wei answers how instantiated traffic locally advances.

### 14.7 Gentrification

No gentrification mechanic is required.

A trajectory such as:

```text
high density
+ low property cost
+ moderate blight
+ strong employment/transit access
+ young population
        ↓
amenity demand
        ↓
commercial investment
        ↓
amenities and desirability rise
        ↓
capital/developer action gradient strengthens
        ↓
redevelopment
        ↓
condition rises, rent rises
        ↓
low-income shelter pressure / migration rises
```

is observable as gentrification.

The runtime never names it.

---

## 15. Generalization beyond cities

The same geometry applies to other SimThing domains.

### 15.1 Diplomacy

Primitive relation axes may include:

```text
threat
trade opportunity
trust
ideological affinity
strategic access
internal political cost
```

A polity projects those into candidate diplomatic potentials.

PALMA over LinkGraph supplies relational gradients.

ActionBand claims political capital, money, attention, logistics, or other real resources.

### 15.2 Production

Primitive axes may include:

```text
input scarcity
labor scarcity
capital scarcity
consumer demand
logistics impedance
risk
```

EML projects marginal production value.

ActionBand claims inputs/capacity.

CostBand determines executable output depth.

### 15.3 Military strategy

Primitive axes may include:

```text
threat
supply
terrain impedance
force concentration
command pressure
strategic value
```

STEAD/PALMA produce action surfaces.

RF provides fuel, supply, command, and capacity.

Gu-Yang supplies conserved saturating flux where applicable.

### 15.4 Capability / derivation

Deriving a specialized SimThing can use the same composition:

```text
growth/specialization potential
      ↓
EML valuation
      ↓
Guard: capability/legal prerequisites
Reserve: residency capacity
Consume/Transfer: derivation resources
Progress: derivation work
      ↓
CostBand N
      ↓
existing structural boundary mints descendants
```

The child is the product.

---

## 16. Adversarial and complementary axis design

The semantic basis must support both opposition and combination without making either a special engine concept.

### 16.1 Adversarial axes

Examples:

```text
safety ↔ threat
abundance ↔ scarcity
integration ↔ isolation
stability ↔ disruption
```

These may be represented as:

- a signed coordinate;
- two separate positive channels with EML comparison;
- dominance + margin derived from competing emitter classes.

The correct form is a modeling decision.

### 16.2 Complementary axes

Examples:

```text
transit accessibility
× employment opportunity
× amenity richness
```

may jointly create strong residential desirability.

Likewise:

```text
cheap land
× strong future demand
× capital availability
```

may create investment opportunity.

Complementarity is an EML composition over fields, not a reason to mint a “gentrification” or “investment” primitive field.

### 16.3 Nonlinear interaction

With `EXP`/`LN` available under admitted semantics, authored projections may use:

- saturation;
- logistic gates;
- softmax-like competition;
- power-law response;
- decay/hysteresis;
- multiplicative/complementary effects.

This is important because social/economic interactions are rarely purely linear.

---

## 17. Axis admission discipline

The biggest danger is semantic axis explosion.

A proposed primitive axis should be admitted only if it passes tests such as:

1. **Independent causality** — does it represent state that cannot be reconstructed from existing axes?
2. **Behavioral consequence** — does some admitted actor/action respond differently because this axis exists?
3. **Non-duplication** — is it more than a correlated restatement of existing dimensions?
4. **Stable interpretation** — can its sign/range/meaning remain coherent across scenarios?
5. **Observable source** — can ordinary SimThing state actually emit or derive it?

Names such as:

```text
good neighborhood
desirable area
high quality zone
prosperous district
```

should usually fail as primitive axes because they are overlapping projections.

---

## 18. Studio as the semantic laboratory

The semantic basis cannot be validated by code review alone.

Studio should test whether intended qualitative regimes emerge from primitive axes and laws.

### 18.1 Workflow

```text
author primitive axes
      ↓
author emitter projections / falloff
      ↓
author actor EML projections
      ↓
author typed requirement bundles
      ↓
run controlled scenario
      ↓
observe field trajectories and RF flows
      ↓
classify higher-order behavior
      ↓
falsify / recalibrate basis
```

### 18.2 Gentrification witness

Initial condition:

```text
high density
moderate blight
cheap housing
strong transit
near employment
young incoming population
available capital
```

Expected qualitative trajectory:

```text
amenity demand ↑
commercial investment ↑
property investment ↑
housing condition ↑
amenity density ↑
rent/land price ↑
higher-income inflow ↑
incumbent displacement pressure ↑
```

If this does not emerge, inspect:

- missing primitive axis;
- wrong EML sign/magnitude;
- incorrect RF constraint;
- PALMA impedance shape;
- wrong time constant;
- actor profile weighting.

Do **not** fix it by adding a hidden `gentrification_bonus` mechanic.

### 18.3 Named phenomena belong in observers

Studio may define observers for:

```text
blight
gentrification
food desert
suburbanization
industrial decline
housing bubble
crime spiral
transit-oriented development
```

These observers classify trajectories.

They do not become simulation authority.

---

## 19. Candidate constitutional laws

The following are candidate laws for later design review, not admitted by this workshop document.

### 19.1 Primitive Causality / Emergent Semantics Law

> SimThing authors independently causal state, resources, topology, and response laws as authoritative substrate. Composite domain concepts should preferentially be represented as projections or temporal observations over that substrate.

### 19.2 Multi-Axis STEAD Law

> A spatial/relational SimThing may project its recursively evaluated state onto an admitted semantic coordinate basis. STEAD propagates those coordinates as independent field channels; it does not require premature collapse to one universal scalar.

### 19.3 Actor Projection Law

> The world owns shared primitive field coordinates. Actors own deficits, profiles, resources, and commitment state. Actor/action-specific EML projections produce scalar potentials/impedances from the shared field bundle.

### 19.4 PALMA Consequence

> Once a lawful scalar action potential/impedance exists over admitted topology, quantitative candidate progress derives through PALMA rather than a domain planner.

### 19.5 Typed Multisource Requirement Law

> Action requirements retain authored semantic roles. Guards and signals are observed; conserved resources are claimed; consumables, transfers, reservations, and progress have distinct post-clear dispositions. Runtime never infers role from domain labels.

### 19.6 Quantitative All-Of Law

> For quantitative constrained requirements, common executable depth is the minimum affordable depth across required lanes, gated by nonconsuming prerequisites. Vector CostBand is the quantitative all-of lowering, not the semantic definition of multisource action.

### 19.7 Generation-Paced Holding Law

> Provisional grants required for atomic multi-lane commitment may be held in-flight through the commit decision. Unused grants return only at the next ordinary resolution opportunity; no same-generation iterative re-clear convergence is introduced.

---

## 20. Falsifiers

This synthesis should be considered wrong or incomplete if any of the following occur.

### 20.1 Semantic-basis falsifier

A required gameplay phenomenon cannot be produced without adding a named domain state that contains independent causal information unavailable from the primitive basis.

### 20.2 Planner falsifier

A major class of action requires an engine-authored domain planner because no lawful field/topology projection can express candidate progress.

### 20.3 Resource-role falsifier

A real action cannot be represented without runtime guessing whether a requirement is a guard, signal, consumable, transfer, reservation, or progress quantity.

### 20.4 Atomicity falsifier

Independent RF clears plus provisional holding cannot implement common multi-resource commitment without violating conservation, fairness/policy, or generation pacing.

### 20.5 Axis-explosion falsifier

A plausible scenario requires hundreds or thousands of primitive semantic channels because derived projections fail to preserve necessary distinctions.

### 20.6 Cohort/particle falsifier

Actor-specific behavior requires so much persistent per-row state that the field abstraction merely relocates rather than removes domain complexity.

---

## 21. Implications for 0.0.8.7

This document does not change the current workplan pointer by itself.

It does suggest several clarifications for future DA review.

### 21.1 ActionBand should inherit multisource semantics

The 0.0.8.7 ActionBand discussion should not repeatedly reopen the question of whether an action can depend on multiple heterogeneous sources.

That structure is inherited from capability-tree doctrine.

The remaining new problem is narrower:

> **prove quantitative atomic commitment across multiple independently cleared scarce lanes.**

### 21.2 Vector CostBand should be framed as lowering

Vector CostBand should not become a giant semantic type that absorbs permissions, policies, preferences, and consumables.

It is the common-depth arithmetic for the constrained quantitative subset of a typed requirement bundle.

### 21.3 STEAD should preserve multi-axis information

The semantic field should remain a vector/tensor of primitive channels long enough for different actors and actions to project it differently.

A single universal “desirability” or “utility” STEAD scalar would destroy important structure.

### 21.4 Field outputs remain ordinary anchored columns

The current 0.0.8.7 P0/P5 rule remains desirable:

- field outputs are ordinary property columns;
- those columns are born STEAD anchors;
- derived outputs can recursively become later inputs;
- no separate semantic-field service is required.

### 21.5 Physical lowering is not semantic authority

Whether a field bundle is evaluated through interpreted EML, SSA JIT, tiled gather, cached profile projection, or later fusion is an execution question.

The semantic authority remains:

- admitted axes;
- admitted projection laws;
- RF roles;
- CostBand arithmetic;
- topology;
- generation pacing.

---

## 22. Recommended prototype sequence

A minimal falsification program can remain small.

### Probe A — capability-shaped typed commitment

Prove one action with:

```text
2 Guards
1 Consume lane
1 Reserve lane
1 Progress lane
```

and a depth-1 consequence.

This demonstrates inheritance from capability unlock semantics.

### Probe B — quantitative repeated work

Extend the same requirement bundle to `N > 1` and prove:

\[
N = G \cdot \min_j \lfloor V_j/C_j \rfloor
\]

with per-lane disposition.

### Probe C — multi-axis field projection

Create a small grid with perhaps four primitive axes:

```text
opportunity
cost
safety
amenity
```

and several emitters with overlapping falloff.

Prove two actor profiles produce different scalar potentials from the same field bundle.

### Probe D — PALMA action descent

Use one of those actor-specific projections as PALMA impedance/potential and show candidate movement/action differs by profile without domain planner code.

### Probe E — closed-loop emergence

Allow committed actions to modify the primitive emitter state and demonstrate a stable feedback pattern not explicitly scripted as an outcome.

A tiny urban witness could use:

```text
housing cost
amenity
opportunity
investment capacity
```

and test whether redevelopment/displacement-like dynamics can emerge.

---

## 23. Core deliverable

The combined resolution is:

### 23.1 Stellaris-type axes solve the semantic organization problem

A small, logically arranged coordinate basis gives the world a reusable vocabulary for expressing complementary, adversarial, and independent conditions.

Higher-order concepts arise from combinations and trajectories through the basis.

### 23.2 Recursive RF solves where field values come from

A SimThing’s field coordinates are projections of its recursively evaluated causal/resource state, not manually pasted semantic labels.

### 23.3 STEAD turns those coordinates into shared heat fields

The semantic basis becomes a multi-channel spatial/relational heat volume through ordinary emitter falloff and superposition.

### 23.4 EML + PALMA turn heat fields into candidate action

Actors project the shared field according to their own deficits, profiles, prices, and commitment state. PALMA turns the resulting potential/impedance into topology-aware candidate progress.

### 23.5 Capability-tree doctrine solves heterogeneous action prerequisites

Actions may depend simultaneously on checks, signals, consumed resources, transfers, reservations, and accumulated work. Those roles are authored and mechanically distinct.

### 23.6 Vector CostBand solves only the remaining quantitative all-of step

For the subset of requirements that are actual constrained quantitative lanes, common executable action depth is the minimum affordable depth across all required grants.

The holding account preserves provisional conservation until commit.

### 23.7 The full loop is recursive

```text
RF state
  ↓
semantic axes
  ↓
STEAD field bundle
  ↓
EML projection
  ↓
PALMA gradient
  ↓
ActionBand
  ↓
typed multisource requirements
  ↓
RF / CostBand
  ↓
consequence
  ↓
RF state
```

The apparent domains — technology unlocks, household moves, production recipes, shopping, service access, diplomacy, derivation, investment, movement — become authored interpretations of the same recursive composition.

---

## 24. Closing judgment

The recurring “multisource action” problem appears less open than the current workshop vocabulary has made it seem.

The Capability Tree work already established the essential semantic structure:

- heterogeneous prerequisites;
- progress separate from eligibility;
- generic value checks;
- atomic consequence activation;
- domain labels outside the simulation substrate.

The 0.0.8.7 work adds what that earlier design did not yet possess:

- intrinsic recursive RF participation;
- STEAD field propagation;
- EML-computed action valuation;
- PALMA topology-aware potentials;
- CostBand as exact determinate work depth;
- and a candidate holding mechanism for multiple contested quantitative grants.

The Stellaris-style axis web supplies the missing geometric clarification: **semantic state should be organized as a compact coordinate basis whose interactions create higher-order behavior, rather than as a catalog of named domain mechanics.**

The resulting architecture is therefore not “semantic fields plus multi-resource transactions.” It is one recursive SimThing cycle:

> **Strong causal axes create shared fields; actors descend the projections of those fields; typed multisource requirements determine whether and how far the desired action can become real; the consequence changes the same recursive state that emits the next field.**

That is the candidate multi-axis ActionBand + STEAD unification.