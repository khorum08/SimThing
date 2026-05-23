# SimThing Modding Guide: Core Authoring Objects

> **Audience:** modders, scenario designers, systems designers, and strategy-game tinkerers.  
> **Assumption:** you understand game scripting concepts such as modifiers, events, triggers, tech trees, traits, traditions, policies, and AI weights. You do **not** need deep engine knowledge to use this guide.

---

## 1. The Big Idea

SimThing is built around a simple but powerful idea:

> **Everything in the game world is state. Everything that changes state should be visible, explainable, and composable.**

Instead of building one-off systems for technology, races, pops, diplomacy, events, ship designs, rebellions, policies, traditions, and AI decisions, SimThing tries to express them through a shared vocabulary:

- **Properties** describe measurable state.
- **Overlays** modify state.
- **Capability Trees** organize unlocks, traditions, doctrines, talents, and selectable progressions.
- **Scripted Values** calculate numbers from state.
- **Triggers** decide when something is true.
- **Effects** describe what happens.
- **Events** connect triggers to effects.
- **Domain Packs** group related content.
- **Game Modes** assemble everything into a playable ruleset.

If you have modded games like Stellaris, Crusader Kings, Victoria, Europa Universalis, RimWorld, or other systems-heavy games, many of these concepts will feel familiar. The difference is that SimThing wants these concepts to compile into a single coherent simulation model instead of many disconnected subsystems.

A SimThing mod is not just a collection of scripts. It is a structured model of how the world behaves.

---

## 2. The Content Hierarchy

At the highest level, a SimThing game mode looks like this:

```text
GameModeSpec
  DomainPackSpec
    PropertySpec
    OverlaySpec
    CapabilityTreeSpec
    ScriptedValueSpec
    TriggerSpec
    EffectSpec
    EventSpec
```

Think of it like this:

```text
GameModeSpec
  = the whole mod / ruleset / total conversion

DomainPackSpec
  = a themed content pack inside that game mode

PropertySpec
  = a measurable thing in the world

OverlaySpec
  = a modifier that changes a property

CapabilityTreeSpec
  = a tech tree, tradition tree, doctrine tree, talent tree, species trait tree, etc.

ScriptedValueSpec
  = a reusable formula

TriggerSpec
  = a reusable condition

EffectSpec
  = a reusable action

EventSpec
  = when condition is true, do actions
```

A simple mod might have one domain pack:

```text
GameModeSpec: "Frontier Empires"
  DomainPackSpec: "Core Economy"
```

A larger game mode might have many:

```text
GameModeSpec: "Veiled Spire"
  DomainPackSpec: "Population"
  DomainPackSpec: "Trade"
  DomainPackSpec: "Fleet Warfare"
  DomainPackSpec: "Diplomacy"
  DomainPackSpec: "Traditions"
  DomainPackSpec: "Technology"
  DomainPackSpec: "Crisis Events"
```

The goal is to make your content modular, inspectable, and reusable.

---

## 3. GameModeSpec — The Whole Ruleset

### What it is

`GameModeSpec` is the top-level object for a SimThing mod or ruleset.

If you are building a total conversion, a scenario pack, a historical ruleset, a sci-fi grand strategy game, or a testing sandbox, the `GameModeSpec` is the root.

It says:

> “This is the game mode. These are its content packs. These are the rules and authored systems that belong to it.”

### What it contains

A `GameModeSpec` can contain:

- metadata such as name, version, description, author tags
- a list of `DomainPackSpec` modules
- global properties
- global overlays
- capability trees
- scripted values
- triggers
- effects
- events
- eventually scenarios, map setup, starting factions, and save/load defaults

### Modder mental model

If SimThing Studio is the editor, the `GameModeSpec` is the project file.

It is the thing you open, save, validate, package, and share.

### Example uses

- A full grand-strategy ruleset
- A balance overhaul mod
- A “hard mode” variant
- A fantasy total conversion
- A combat-system experiment
- A pop/economy simulation sandbox
- A set of scripted crisis scenarios

### Core principle

> **A GameModeSpec describes the game’s authored rules, not the current runtime state of a save file.**

It describes how the world can behave, not everything that has happened in a particular playthrough.

---

## 4. DomainPackSpec — A Modular Content Pack

### What it is

`DomainPackSpec` is a themed module inside a game mode.

It lets you organize your content by gameplay domain.

Instead of dumping every property, overlay, event, and capability into one giant file, you can group them by purpose:

```text
DomainPackSpec: "Economy"
DomainPackSpec: "Warfare"
DomainPackSpec: "Diplomacy"
DomainPackSpec: "Species"
DomainPackSpec: "Traditions"
DomainPackSpec: "Technology"
DomainPackSpec: "Crisis Events"
```

### Why it matters

A strategy game quickly becomes huge. If everything lives in one file, modding becomes painful.

Domain packs let you create content in layers:

```text
Core Economy
  defines production, trade, scarcity, inflation

Population
  defines cohorts, migration, happiness, unrest

Fleet Warfare
  defines fleet power, projection, ship speed, supply

Diplomacy
  defines trust, threat, treaties, acceptance scores

Traditions
  defines national ideas and selectable doctrines
```

### Modder mental model

A domain pack is like a DLC-sized or mod-folder-sized chunk of rules.

You should be able to open one domain pack and understand a specific gameplay system without reading the entire game.

### Example

A `Trade` domain pack might define:

- `trade_value`
- `market_access`
- `tariff_pressure`
- `smuggling_pressure`
- overlays for trade policies
- scripted values for route desirability
- events for trade disruption
- triggers for embargo thresholds

### Core principle

> **Domain packs keep the game mode understandable.**

They are not separate engines. They are organized bundles of specs that all compile into the same SimThing world model.

---

## 5. PropertySpec — The Vocabulary of State

### What it is

`PropertySpec` defines a measurable piece of game state.

In other games, you might call these stats, variables, modifiers, resources, outputs, needs, meters, or values. In SimThing, the general term is **property**.

Examples:

```text
loyalty
food_security
trade_value
fleet_projection
research_progress
migration_pull
rebellion_pressure
disease_pressure
diplomatic_trust
industrial_output
naval_capacity
```

A property is not just a number. It can have multiple subfields, such as:

```text
amount
velocity
intensity
confidence
pressure
capacity
```

The exact subfields depend on what the property means.

### Why properties are powerful

Properties form the shared language of the simulation.

If a mod defines `food_security`, then many systems can refer to it:

- a famine event trigger
- a migration pull formula
- a rebellion pressure calculation
- a trade route priority
- an AI risk frontier
- a policy overlay
- a diplomacy penalty

Because all of these systems read the same property, the world feels connected.

### Modder mental model

A property is a stat with structure.

Instead of writing isolated scripts like:

```text
if planet_has_food_shortage then add_unrest
```

you define a real state dimension:

```text
food_security
```

Then many systems can respond to it.

### Common property patterns

#### Simple meter

```text
loyalty.amount
```

Represents the current level of loyalty.

#### Moving trend

```text
loyalty.amount
loyalty.velocity
```

Represents loyalty and the direction it is changing.

#### Risk field

```text
rebellion_pressure.amount
rebellion_pressure.velocity
rebellion_pressure.intensity
```

Represents not just whether rebellion exists, but how strongly it is forming.

#### Capability progress

```text
ion_drive_progress.amount
```

or as a named subfield in a capability category:

```text
propulsion.ion_drive
```

### Core principle

> **If something matters to multiple systems, make it a property.**

Do not hide important game logic in one-off scripts. Make it visible as state.

---

## 6. OverlaySpec — The Universal Modifier

### What it is

`OverlaySpec` defines a modifier that changes one or more property values.

If you have modded Paradox-style games, you can think of overlays as the general form of:

```text
planet_modifier
country_modifier
pop_modifier
ship_modifier
policy_modifier
trait_modifier
edict_modifier
technology_modifier
tradition_modifier
```

But in SimThing, they all use the same basic mechanism: an overlay transforms properties.

### Examples

A war economy policy might apply:

```text
industrial_output.amount × 1.20
stability.amount - 0.05
consumer_goods.amount - 0.10
```

A species trait might apply:

```text
food_consumption.amount × 0.90
research_output.amount × 1.05
```

A technology unlock might apply:

```text
fleet_speed.amount × 1.30
```

A temporary disaster might apply:

```text
food_security.amount - 0.25
rebellion_pressure.amount + 0.15
```

### Overlay lifecycles

Overlays can have different lifecycles.

#### Permanent

Always active once attached.

Use for:

```text
species traits
built infrastructure
long-term policy effects
permanent technology bonuses
```

#### Transient

Temporary.

Use for:

```text
disasters
temporary edicts
short-lived morale shocks
event aftermath
```

#### Suspended

Defined but inactive until activated.

This is extremely important.

Use for:

```text
technology effects before the technology is researched
national tradition effects before selected
latent racial abilities
locked policy doctrines
future event effects
```

A suspended overlay is like a sealed modifier waiting inside the game data. It exists, it can be previewed, and Studio can show it, but it does not affect the simulation until activated.

### Modder mental model

An overlay is the answer to:

> “What changes when this trait, policy, tech, tradition, event, or condition applies?”

### Why overlays matter

SimThing avoids separate modifier systems for every domain. Instead of having special code for tech modifiers, trait modifiers, policy modifiers, and event modifiers, they all become overlays.

That makes content easier to reason about.

### Core principle

> **Most gameplay bonuses, penalties, and modifiers should be overlays.**

If it changes a value, it probably wants to be an overlay.

---

## 7. CapabilityTreeSpec — Unlocks, Traditions, Doctrines, Talents, and More

### What it is

`CapabilityTreeSpec` defines a tree or web of unlockable or selectable capabilities.

Despite the name, this is not only for “technology.” It can represent many progression systems:

```text
technology trees
national traditions
doctrine trees
leader talents
species trait evolutions
racial abilities
policy branches
religious reforms
civic development
ship design unlocks
crisis progressions
```

### What a capability entry can do

A capability entry can:

- track progress
- require prerequisites
- activate suspended overlays
- belong to a category
- participate in max-active rules
- be selected by the player
- represent an unlock, doctrine, tradition, or ability

### Example: technology

```text
Basic Propulsion
  unlocks:
    fleet_speed +10%

Ion Drive
  requires:
    Basic Propulsion
  unlocks:
    fleet_speed +30%
```

### Example: national traditions

```text
Military Tradition Category
  max_active = 1

Citizen Admiralty
  effect:
    naval_capacity +20

Warrior Houses
  effect:
    army_damage +15%

Naval Logistics
  effect:
    ship_upkeep -10%
```

Only one may be active at a time if the category uses `max_active = 1`.

### Example: species abilities

```text
Kverlikon Physiology
  Stone-Blood Metabolism
    food_consumption -15%

  High-G Memory
    research_output +10%

  Slow Reproduction
    pop_growth -20%
```

### Why capability trees are different from events

Events happen when conditions are met.

Capabilities represent structured progression: something is researched, chosen, unlocked, inherited, activated, or suspended.

A capability tree gives shape to long-term development.

### How capability effects work

The preferred pattern is:

```text
Capability entry
  owns suspended overlays

When unlocked or selected:
  activate those overlays
```

This gives Studio a very clean preview:

```text
If you unlock Ion Drive:
  fleet_speed.amount: 10 → 13
```

### Core principle

> **Capabilities organize latent power. Overlays apply that power.**

A capability tree should not be a pile of arbitrary script effects. It should be a clear progression structure whose consequences are visible.

---

## 8. ScriptedValueSpec — Reusable Formulas

### What it is

`ScriptedValueSpec` defines a reusable formula.

A scripted value answers:

> “How much?”

Examples:

```text
migration_pull
trade_route_score
rebellion_risk
diplomatic_acceptance
colonization_priority
fleet_threat_score
research_rate
ship_design_power
```

### Example formula

```text
rebellion_risk =
  grievance * 1.5
  + food_insecurity * 0.8
  - fleet_projection * 0.4
```

In Studio, this could be edited as a formula graph:

```text
Read grievance
Read food_insecurity
Read fleet_projection
Multiply
Add
Subtract
Clamp
```

### Why scripted values matter

Without scripted values, every event, AI behavior, diplomacy action, and economic system would need to repeat the same formulas.

With scripted values, you define the formula once and reuse it everywhere.

### Common uses

#### AI weights

```text
attack_opportunity
colonize_priority
ally_value
trade_route_desirability
```

#### Risk scores

```text
rebellion_risk
famine_risk
disease_risk
piracy_risk
```

#### Economic calculations

```text
market_pull
migration_pull
tariff_pressure
smuggling_profit
```

#### Design constraints

```text
ship_power_usage
component_weight
fleet_supply_need
```

### Script IR and EML

Scripted values are represented internally through Script IR, SimThing’s formula language.

Later, some pure numeric formulas may be compiled into GPU-friendly EML or other optimized backends. As a modder, you should not need to think about that. You write the formula; the engine decides how to evaluate it.

### Core principle

> **If several systems need the same calculation, make it a scripted value.**

Formulas should be reusable, inspectable, and explainable.

---

## 9. TriggerSpec — Conditions That Watch the World

### What it is

`TriggerSpec` defines a condition.

A trigger answers:

> “When is this true?”

Examples:

```text
food_security < 0.25
loyalty < 0.10
rebellion_risk > 0.75
game_time.day > 30
has_active_overlay = war_economy
has_capability = ion_drive
```

### Simple triggers

Simple triggers compare a property to a threshold:

```text
loyalty.amount < 0.10
```

These can often be compiled efficiently into threshold checks.

### Composite triggers

Composite triggers combine multiple conditions:

```text
food_security < 0.25
AND rebellion_pressure > 0.50
AND game_time.day > 30
```

These use Script IR predicates.

### Triggers and game time

Game time is also state.

That means timed logic can be written as ordinary trigger logic:

```text
game_time.day > 365
```

or:

```text
current_day - last_fired_day > 30
```

This keeps timing visible and scriptable instead of hiding it in engine code.

### Common trigger uses

- event activation
- capability unlock checks
- crisis phase changes
- AI decision gates
- policy availability
- ship design requirements
- diplomacy conditions
- tradition unlocks

### Core principle

> **Triggers should describe observable conditions, not hidden engine magic.**

If something fires, designers should be able to inspect why.

---

## 10. EffectSpec — Things That Happen

### What it is

`EffectSpec` defines an action the game can take.

An effect answers:

> “What happens when this fires?”

Examples:

```text
activate an overlay
suspend an overlay
attach a modifier
add rebellion pressure
remove a temporary effect
trigger a notification
change a property
spawn or attach a SimThing
```

### Important design rule

Effects should be **closed and safe**.

That means effects should come from a known list of operations rather than arbitrary code.

Why?

Because SimThing wants effects to remain:

```text
validatable
previewable
replayable
saveable
explainable
safe for simulation boundaries
```

### Examples

#### Activate a technology bonus

```text
ActivateOverlay: ion_drive_speed_bonus
```

#### Apply event aftermath

```text
AddPropertyDelta: rebellion_pressure +0.10
```

#### Suspend a national idea

```text
SuspendOverlay: citizen_admiralty_bonus
```

#### Notify the player

```text
EmitNotification: food_shortage_warning
```

### Effects vs overlays

This distinction matters:

```text
Overlay
  = a modifier that changes state while active

Effect
  = an action that causes something to happen
```

An effect may activate an overlay, attach an overlay, or change a property, but it is not the same thing as the overlay itself.

### Core principle

> **Effects should describe actions in a way the engine can validate, preview, and replay.**

Avoid one-off magical actions when a structured effect will do.

---

## 11. EventSpec — Cause and Consequence

### What it is

`EventSpec` connects triggers to effects.

An event answers:

> “When this condition is true, what should happen?”

Basic shape:

```text
Event
  Trigger
  Effects
  Cooldown
  Priority
  Metadata
```

### Example

```text
Food Shortage Unrest

Trigger:
  food_security.amount < 0.25
  AND game_time.day > 30

Effects:
  rebellion_pressure.amount +0.10
  activate_overlay: local_unrest_warning

Cooldown:
  10 days
```

### Why events are important

Events are how the world becomes legible.

They turn continuous state into moments of meaning:

```text
a rebellion begins
a famine warning appears
a treaty expires
a tradition becomes available
a crisis phase changes
a faction demands autonomy
a fleet supply collapse begins
```

### Event graphs

In Studio, events should probably be shown as cause/effect graphs:

```text
Trigger → Event → Effects
```

Example:

```text
food_security < 0.25
        │
        ▼
Food Shortage Unrest
        │
        ├── Add rebellion_pressure
        ├── Activate unrest overlay
        └── Notify player
```

### Events and cooldowns

Cooldowns should also be based on observable game time.

Instead of hiding cooldown logic in the engine, an event can track or reference:

```text
core::game_time.day
```

This keeps timed logic scriptable and visible.

### Core principle

> **Events are not arbitrary surprises. They are explainable consequences of world state.**

A good event should tell the player something meaningful about the simulation.

---

## 12. How These Objects Work Together

Here is a simple chain:

```text
PropertySpec
  defines food_security

TriggerSpec
  checks food_security < 0.25

EventSpec
  fires Food Shortage Unrest

EffectSpec
  adds rebellion_pressure

OverlaySpec
  applies a temporary unrest penalty
```

Another chain:

```text
CapabilityTreeSpec
  defines Ion Drive

Capability entry
  requires Basic Propulsion

OverlaySpec
  defines fleet_speed × 1.30

Unlock
  activates the suspended overlay
```

Another chain:

```text
ScriptedValueSpec
  calculates diplomatic_acceptance

TriggerSpec
  checks diplomatic_acceptance > 0.70

EventSpec or action
  allows treaty signing
```

The power comes from composition.

You are not writing isolated scripts. You are building a network of state, modifiers, conditions, and consequences.

---

## 13. Suggested Modding Workflow

A good modding flow might look like this:

### Step 1: Define your domain

Decide what system you are building:

```text
trade
migration
species traits
ship design
religion
technology
diplomacy
faction politics
```

Create a `DomainPackSpec`.

### Step 2: Define properties

Ask:

> What state does this system need?

For trade:

```text
trade_value
market_access
tariff_pressure
smuggling_pressure
route_risk
```

For politics:

```text
loyalty
legitimacy
rebellion_pressure
elite_discontent
popular_support
```

### Step 3: Define overlays

Ask:

> What modifies those properties?

Examples:

```text
trade_policy_free_ports
military_crackdown
species_industrious
ion_drive_bonus
war_economy
```

### Step 4: Define capabilities

Ask:

> What can be unlocked, selected, inherited, or progressed?

Examples:

```text
tech tree
tradition tree
doctrine tree
species adaptation tree
leader talent tree
```

### Step 5: Define scripted values

Ask:

> What formulas will be reused?

Examples:

```text
rebellion_risk
migration_pull
trade_route_score
diplomatic_acceptance
```

### Step 6: Define triggers

Ask:

> What conditions matter?

Examples:

```text
rebellion_risk > 0.75
food_security < 0.25
game_time.day > 365
has_capability = ion_drive
```

### Step 7: Define effects

Ask:

> What actions should the game take?

Examples:

```text
activate overlay
add pressure
spawn notification
suspend old tradition
```

### Step 8: Define events

Ask:

> What meaningful moments should emerge?

Examples:

```text
Famine Warning
Secession Crisis
Trade Route Collapse
New Doctrine Available
Treaty Expired
Religious Schism
```

---

## 14. The Studio Vision

SimThings Studio should make these objects visual.

Not one giant graph, but multiple graph views:

```text
Capability Tree Graph
  unlocks, prereqs, traditions, doctrines

Script Formula Graph
  calculations, AI weights, predicates

Event Graph
  trigger → event → effects

Overlay Graph
  what modifies what

SimThing Structure Graph
  where state lives in the world

Property Editor
  dimensions, subfields, reductions, clamps
```

The same data appears through different lenses.

A capability entry might connect to:

- its prereqs in the capability graph
- its suspended overlays in the overlay graph
- its effects in the inspector
- its preview in the preview panel
- its properties in the property editor

This lets modders build complex systems without losing track of how they work.

---

## 15. Design Philosophy for Modders

### Make state visible

If something matters, expose it as a property.

Bad:

```text
hidden unrest script
```

Better:

```text
rebellion_pressure.amount
rebellion_pressure.velocity
```

### Prefer overlays for modifiers

If something changes a value, make it an overlay.

Bad:

```text
special case: if ion_drive then speed += 30%
```

Better:

```text
Ion Drive activates fleet_speed × 1.30 overlay
```

### Use scripted values for formulas

If a formula is reused, name it.

Bad:

```text
repeat the same diplomatic acceptance math in ten events
```

Better:

```text
scripted_value: diplomatic_acceptance
```

### Use triggers for meaningful thresholds

If state crossing a line matters, make a trigger.

Bad:

```text
buried check inside event code
```

Better:

```text
Trigger: food_security < 0.25
```

### Use events for meaning

Events should turn simulation state into readable moments.

Bad:

```text
random unrest popup
```

Better:

```text
Food Shortage Unrest fires because food_security is low and rebellion_pressure is rising
```

### Keep domains modular

Do not put everything in one file.

Use domain packs.

---

## 16. A Small Example: Food Shortage Unrest

Here is the conceptual structure of a small system.

### DomainPackSpec

```text
Domain: Population Stability
```

### PropertySpec

```text
food_security
rebellion_pressure
loyalty
```

### OverlaySpec

```text
local_unrest_overlay
  loyalty.amount -0.05
  rebellion_pressure.amount +0.10
```

### ScriptedValueSpec

```text
food_unrest_risk =
  (1.0 - food_security.amount) + rebellion_pressure.amount
```

### TriggerSpec

```text
food_unrest_risk > 0.75
AND game_time.day > 30
```

### EffectSpec

```text
activate local_unrest_overlay
notify player
```

### EventSpec

```text
Food Shortage Unrest
  trigger: food_unrest_risk > 0.75
  effects:
    activate local_unrest_overlay
    notify player
  cooldown: 10 days
```

This is a tiny system, but it already shows the SimThing pattern:

```text
state → formula → trigger → event → effect → overlay → state
```

That loop is the heart of the engine.

---

## 17. Another Example: Ion Drive Technology

### CapabilityTreeSpec

```text
Technology Tree: Propulsion
```

### Capability entry

```text
Ion Drive
  requires: Basic Propulsion
  research_cost: 100
```

### OverlaySpec

```text
ion_drive_speed_bonus
  fleet_speed.amount × 1.30
  lifecycle: Suspended
```

### Unlock behavior

```text
When Ion Drive reaches its research threshold:
  activate ion_drive_speed_bonus
```

The player sees a tech unlock.

The engine sees a suspended overlay become active.

That is the SimThing style: designer-friendly on top, consistent mechanics underneath.

---

## 18. Final Mental Model

If you remember only one thing, remember this:

```text
Properties are what the world is.
Overlays are what changes the world.
Capabilities are what unlocks changes.
Scripted values are how the world is measured.
Triggers are when the world crosses a line.
Effects are what happens.
Events are why the player notices.
Domain packs organize systems.
Game modes assemble the game.
```

Or even shorter:

> **Define state. Modify it with overlays. Watch it with triggers. Explain it with events. Organize it with domain packs.**

That is the SimThing modding philosophy.

---

## 19. Why Modders Should Care

SimThing is trying to make complex strategy-game systems more transparent and more composable.

As a modder, this means:

- your systems can interact naturally
- your bonuses and penalties are previewable
- your formulas are reusable
- your events can explain real simulation state
- your AI weights can use the same properties players see
- your techs, traditions, traits, and policies can share one modifier model
- your content can be organized into modular domain packs
- your rules can become visual in Studio

The dream is not just to make modding possible.

The dream is to make deep systemic modding **legible**.

If you can describe your system as state, modifiers, conditions, and consequences, SimThing should be able to help you build it.
