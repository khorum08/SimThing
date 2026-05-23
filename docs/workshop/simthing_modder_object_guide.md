# SimThing Modding Guide: Core Authoring Objects

> **Audience:** modders, scenario designers, systems designers, and strategy-game tinkerers.  
> **Assumption:** you understand game scripting concepts such as modifiers, events, triggers, tech trees, traits, traditions, policies, and AI weights. You do **not** need deep engine knowledge to use this guide.

---

## 0. Current status

This guide describes the current **v0 authoring surface** after the `simthing-spec` implementation ladder and the post-ladder Opus P0 batch.

Implemented now:

- `GameModeSpec` and `DomainPackSpec` authoring.
- Properties, overlays, capability trees, scripted values, triggers, effects, and events.
- `SimSession::open_from_spec`, backed by atomic install.
- `InstallTargetSpec` for capability trees and scripted events.
- `EffectTarget` for capability effects.
- Per-owner scripted event instances with independent cooldowns.
- Replay v3 for spec runtime state.
- Studio-safe install preview data.

Still future or tabled:

- Studio GUI.
- Full scenario RON expansion.
- Tag selectors or dynamic install filters.
- `ScopeRef::Owner` and cross-owner scripted event extensions.
- Mid-session GPU resync for applying previews to an already-running session.
- Hot reload with state preservation.
- Base economy ADR and transfer-overlay semantics.

Reference-level syntax lives in `docs/capability_tree_v1.md`, `docs/examples/`, and the ADRs under `docs/adr/`.

---

## 1. The Big Idea

SimThing is built around a simple idea:

> **Everything in the game world is state. Everything that changes state should be visible, explainable, and composable.**

Instead of building separate hardcoded systems for technology, traditions, events, policies, traits, AI weights, and scenario logic, SimThing uses a shared vocabulary:

- **Properties** describe measurable state.
- **Overlays** modify state.
- **Capability Trees** organize unlocks and choices.
- **Scripted Values** calculate numbers from state.
- **Triggers** decide when something is true.
- **Effects** describe what happens.
- **Events** connect triggers to effects.
- **Install Targets** decide which world objects receive a tree or event instance.
- **Domain Packs** group related content.
- **Game Modes** assemble everything into a playable ruleset.

A SimThing mod is not just a collection of scripts. It is a structured model of how the world behaves.

---

## 2. The Content Hierarchy

At the highest level, a game mode looks like this:

```text
GameModeSpec
  DomainPackSpec
    PropertySpec
    OverlaySpec
    CapabilityTreeSpec
      install: InstallTargetSpec
      CapabilityEffectSpec
        effect_target: EffectTarget
    ScriptedValueSpec
    TriggerSpec
    EffectSpec
    EventSpec
      install: InstallTargetSpec
```

Think of it like this:

```text
GameModeSpec      = the whole mod / ruleset / total conversion
DomainPackSpec    = a themed content pack inside that game mode
PropertySpec      = a measurable thing in the world
OverlaySpec       = a modifier that changes a property
CapabilityTreeSpec= a tech tree, tradition tree, doctrine tree, talent tree, etc.
InstallTargetSpec = where a tree or event should be attached
EffectTarget      = where a capability effect applies
ScriptedValueSpec = a reusable formula
TriggerSpec       = a reusable condition
EffectSpec        = a reusable action
EventSpec         = when condition is true, do actions
```

The goal is to make content modular, inspectable, and reusable.

---

## 3. GameModeSpec and DomainPackSpec

`GameModeSpec` is the top-level object for a mod or ruleset. It describes authored rules, not save-game state.

A `GameModeSpec` can contain metadata, domain packs, global properties, overlays, capability trees, scripted values, triggers, effects, and events.

`DomainPackSpec` is a themed module inside a game mode. Use domain packs to group content by purpose:

```text
Core Economy
Population
Fleet Warfare
Diplomacy
Traditions
Technology
Scenario Events
```

A domain pack is not a separate engine. It is an organized bundle of specs that compiles into the same SimThing world model.

The normal runtime entry point is:

```rust
let game_mode = simthing_spec::deserialize_game_mode_ron(&ron_text)?;
let session = SimSession::open_from_spec(scenario, &game_mode)?;
```

`open_from_spec` uses the safe atomic install path, so a failed install does not leak a partially mutated session to the caller.

---

## 4. PropertySpec — The Vocabulary of State

`PropertySpec` defines a measurable piece of game state.

Examples:

```text
loyalty
food_security
trade_value
fleet_projection
research_progress
migration_pull
rebellion_pressure
diplomatic_trust
industrial_output
naval_capacity
```

A property can have multiple subfields:

```text
amount
velocity
intensity
confidence
pressure
capacity
Named("ion_drive")
Named("ion_drive_rate")
```

Properties are the shared language of the simulation. If a mod defines `food_security`, many systems can use it: events, formulas, overlays, AI weights, policy checks, and capability prereqs.

> **Rule:** If something matters to multiple systems, make it a property.

---

## 5. OverlaySpec — The Universal Modifier

`OverlaySpec` defines a modifier that changes one or more property values.

Examples:

```text
industrial_output.amount × 1.20
stability.amount - 0.05
fleet_speed.amount × 1.30
food_security.amount - 0.25
rebellion_pressure.amount + 0.15
```

Overlay lifecycles:

| Lifecycle | Meaning | Use when |
|-----------|---------|----------|
| `Permanent` | Active once attached or activated | traits, buildings, policies, permanent unlocks |
| `Transient` | Temporary | event aftermath, pulses, short-lived penalties |
| `Suspended` | Defined but inactive until activated | tech effects, traditions, latent abilities |

Suspended overlays are especially important. A capability entry usually owns suspended overlays; when the entry unlocks or is selected, those overlays activate.

### Placement matters

For ordinary authors, the rule is:

> **An overlay must live on the SimThing whose subtree should receive the modifier.**

The GPU overlay-prep stage walks the SimThing tree from the overlay host. For capability effects, the install layer handles this by placing cloned overlays on the correct host and recording `overlay_hosts` internally.

As a modder, you normally author `effect_target` and let install do the placement.

---

## 6. InstallTargetSpec — Where Content Attaches

Capability trees and scripted events are authored once, then installed into a scenario. The install target answers:

> “Which SimThings receive this tree or event instance?”

v0 supports three install targets.

### AllOfKind

```ron
install: AllOfKind(kind: "Faction"),
```

Installs one copy for every SimThing whose kind exactly matches the string.

Use for content every object of a kind should receive:

```text
every faction gets a technology tree
every faction gets a low-loyalty event
every fleet gets a supply event
```

Built-in kind strings use enum spelling, such as `"World"`, `"Faction"`, `"StarSystem"`, `"Location"`, `"Cohort"`, `"Fleet"`, and `"Station"`. Custom kinds match their authored label, such as `"national_ideas"`.

Matching is exact and case-sensitive in v0.

### ScenarioListed

```ron
install: ScenarioListed(target_id: "player_faction"),
```

The scenario supplies the actual owner IDs under that target name. Use this when scenario setup decides which specific object receives content.

If the scenario does not provide the target ID, install fails.

### SessionRoot

```ron
install: SessionRoot,
```

Installs once on the scenario root. Use for world-level or session-global systems.

Defaults:

- `CapabilityTreeSpec.install` defaults to `AllOfKind(kind: "Faction")`.
- `EventSpec.install` defaults to `SessionRoot`.

If an install target resolves to no owners, install fails. Silent no-op installs are treated as broken content.

---

## 7. CapabilityTreeSpec — Unlocks and Choices

`CapabilityTreeSpec` defines a tree or web of unlockable or selectable capabilities.

It can represent:

```text
technology trees
national traditions
doctrine trees
leader talents
species trait evolutions
policy branches
ship design unlocks
scenario progressions
```

### Runtime model

A capability tree is authored once, then cloned per install target.

```text
GameModeSpec defines: Terran Technology Tree
Install target: AllOfKind("Faction")
Runtime result:
  Faction A gets clone A
  Faction B gets clone B
  Faction C gets clone C
```

Each clone has independent progress, active choices, and cloned overlays.

### Capability entries

A capability entry can:

- track progress
- require prerequisites
- activate suspended overlays
- belong to a category
- participate in max-active rules
- be selected by the player
- represent an unlock, doctrine, tradition, or ability

### Capability effects and EffectTarget

Capability effects declare where their overlay should apply. The default is `Owner`, which matches the common expectation: a faction researches a technology, and the faction receives the bonus.

```ron
CapabilityEffectSpec(
    targets_property: "military::fleet_speed",
    sub_field_deltas: [(Amount, Multiply(1.30))],
    when_activated: Permanent,
    effect_target: Owner,
)
```

Targets:

| Target | Meaning | Use when |
|--------|---------|----------|
| `Owner` | Applies to the SimThing the tree was installed for | faction tech bonuses, traditions, traits, fleet bonuses |
| `CapabilityTree` | Applies to the cloned tree node itself | tree-local counters or bookkeeping |
| `SessionRoot` | Applies to the scenario root | global era flags or world-level modifiers |

You can omit `effect_target`; v0 defaults it to `Owner`.

### Player selections

For selectable ideas or doctrines, gameplay code should queue selections by logical key:

```text
owner_id + tree_logical_id + entry_id
```

Avoid exposing raw runtime overlay IDs to UI or mod scripts.

> **Rule:** Capabilities organize latent power. Overlays apply that power. Install targets decide who receives it.

---

## 8. ScriptedValueSpec — Reusable Formulas

`ScriptedValueSpec` defines a reusable formula. A scripted value answers:

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

Use scripted values for formulas that multiple systems need. They make calculations reusable, inspectable, and explainable.

---

## 9. TriggerSpec — Conditions That Watch the World

`TriggerSpec` defines a condition. A trigger answers:

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

### Threshold triggers

Simple property threshold triggers can compile into efficient GPU threshold checks.

```text
loyalty.amount < 0.10
```

Threshold-only scripted events can skip quiet boundaries when no relevant threshold has fired.

### Predicate triggers

Composite predicates combine multiple conditions through Script IR:

```text
food_security < 0.25
AND rebellion_pressure > 0.50
AND game_time.day > 30
```

Predicate-trigger events are checked at boundary time because they re-evaluate against current CPU shadow state.

> **Rule:** Triggers should describe observable conditions, not hidden engine magic.

---

## 10. EffectSpec — Things That Happen

`EffectSpec` defines an action the game can take. An effect answers:

> “What happens when this fires?”

Examples:

```text
activate an overlay
suspend an overlay
attach a modifier
add pressure
remove a temporary effect
emit a notification
change a property
spawn or attach a SimThing
```

Effects should be closed and safe. They should come from a known list of operations rather than arbitrary code, so they remain validatable, previewable, replayable, saveable, and explainable.

Distinction:

```text
Overlay = a modifier that changes state while active
Effect  = an action that causes something to happen
```

An effect may activate an overlay, attach an overlay, or change a property, but it is not the same thing as the overlay itself.

---

## 11. EventSpec — Cause and Consequence

`EventSpec` connects triggers to effects. An event answers:

> “When this condition is true, what should happen?”

Basic shape:

```text
Event
  Install target
  Trigger
  Effects
  Cooldown
  Priority
  Metadata
```

### Per-owner events

Events can install just like capability trees.

```ron
install: AllOfKind(kind: "Faction"),
```

A per-faction event installed on all factions creates one event instance per faction. Each instance has its own owner, current slot, and cooldown.

Omitting `install` defaults to `SessionRoot`, preserving session-global behavior.

### ScopeRef::Current

For a per-owner event, `ScopeRef::Current` means:

> “the SimThing this event instance is installed on.”

That lets you write one event definition and install it on many owners without hardcoding slots.

Example mental model:

```text
Low Loyalty Warning
  install: AllOfKind("Faction")
  trigger: Current.loyalty.amount < 0.10
  cooldown: 10 days

Runtime:
  Faction A can fire and enter cooldown
  Faction B can still fire independently
```

### Boundary behavior

The driver only forces spec boundary work when needed. Quiet threshold-only events can skip. Boundary work is forced by queued player selections, active cooldowns, predicate events, `OnPrereqMet` capability sweeps, or matching fired threshold events.

---

## 12. How These Objects Work Together

Example chain:

```text
PropertySpec
  defines food_security

ScriptedValueSpec
  calculates food_unrest_risk

TriggerSpec
  checks food_unrest_risk > 0.75

EventSpec
  fires Food Shortage Unrest

EffectSpec
  activates local_unrest_overlay

OverlaySpec
  applies loyalty penalty + pressure
```

Capability chain:

```text
CapabilityTreeSpec
  installs on all factions

Capability entry
  requires Basic Propulsion

CapabilityEffectSpec
  targets military::fleet_speed
  effect_target: Owner

Unlock
  activates the suspended overlay on that owner
```

Per-owner event chain:

```text
EventSpec
  install: AllOfKind("Faction")

ScopeRef::Current
  resolves to each faction's slot

Cooldown
  is independent per faction event instance
```

---

## 13. Suggested Modding Workflow

1. **Define your domain.** Create a `DomainPackSpec` for a coherent gameplay system.
2. **Define properties.** Decide what state the system needs.
3. **Define overlays.** Decide what modifies that state.
4. **Define install targets.** Decide who receives each tree or event.
5. **Define capabilities.** Decide what can be unlocked, selected, or progressed.
6. **Define effect targets.** Default to `Owner` unless the effect is tree-local or global.
7. **Define scripted values.** Name reusable formulas.
8. **Define triggers.** Decide which conditions matter.
9. **Define effects and events.** Turn state changes into readable moments.

---

## 14. Preview, Replay, and Stable IDs

### Preview install

The install system supports a preview flow:

```text
preview_install
  builds proposed registry/root/allocator/spec state on scratch clones

apply_install_preview
  commits the preview if accepted
```

This is mainly for Studio. It allows a designer to inspect what a game mode would add without corrupting a live session if validation fails.

v0 caution: applying a preview to an already-running GPU session still needs future GPU resync work. For session open, `open_from_spec` is safe today.

### Replay v3

Replay now preserves spec runtime state:

```text
capability activation modes
active selections per category
scripted event cooldowns
queued player selections
capability notifications
```

Replay uses logical keys, not raw runtime IDs. Stable authored IDs are part of the replay/save-like contract.

Good:

```text
tree_id: "terran_tech_tree"
entry_id: "ion_drive"
event_id: "food_shortage_unrest"
```

Bad mental model:

```text
hardcoded OverlayId or process-local runtime id
```

> **Rule:** Stable authoring IDs are not cosmetic. They are how tools, replay, and save-like flows recognize your content.

---

## 15. The Studio Vision

SimThing Studio should eventually make these objects visual through multiple views:

```text
Capability Tree Graph
Install Target View
Script Formula Graph
Event Graph
Overlay Graph
SimThing Structure Graph
Property Editor
Preview Panel
```

A capability entry might connect to its prereqs, suspended overlays, `effect_target`, preview output, install scope, and properties.

The same data appears through different lenses so modders can build complex systems without losing track of how they work.

---

## 16. Design Philosophy for Modders

### Make state visible

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

Bad:

```text
special case: if ion_drive then speed += 30%
```

Better:

```text
Ion Drive activates fleet_speed × 1.30 overlay
```

### Put effects at the right scope

Most capability effects should target `Owner`.

Bad:

```text
tech bonus accidentally modifies only the tech-tree node
```

Better:

```text
effect_target: Owner
```

### Install content deliberately

Ask:

```text
Does every faction get this?
Does only a scenario-listed owner get this?
Is this truly session-global?
```

### Use events for meaning

Events should turn simulation state into readable moments.

Bad:

```text
random popup detached from visible state
```

Better:

```text
Food Shortage Unrest fires because food_security is low and pressure is rising
```

---

## 17. Example: Food Shortage Unrest

```text
DomainPackSpec: Population Stability

PropertySpec:
  food_security
  pressure
  loyalty

OverlaySpec:
  local_unrest_overlay
    loyalty.amount -0.05
    pressure.amount +0.10

ScriptedValueSpec:
  food_unrest_risk = (1.0 - food_security.amount) + pressure.amount

EventSpec:
  Food Shortage Unrest
    install: AllOfKind(kind: "Faction")
    trigger: food_unrest_risk > 0.75
    effects:
      activate local_unrest_overlay
      notify player
    cooldown: 10 days
```

Runtime meaning:

```text
Each faction has its own event instance.
Each faction checks its own current slot.
Each faction has its own cooldown.
```

---

## 18. Example: Ion Drive Technology

```text
CapabilityTreeSpec:
  Technology Tree: Propulsion
  install: AllOfKind(kind: "Faction")

Capability entry:
  Ion Drive
    requires: Basic Propulsion
    research_cost: 100

CapabilityEffectSpec:
  targets_property: "military::fleet_speed"
  sub_field_deltas: [(Amount, Multiply(1.30))]
  when_activated: Permanent
  effect_target: Owner
```

Runtime meaning:

```text
When a faction unlocks Ion Drive:
  activate that faction's speed bonus overlay
```

The player sees a technology unlock. The engine sees a suspended overlay become active on the correct owner.

---

## 19. Final Mental Model

```text
Properties are what the world is.
Overlays are what changes the world.
Install targets decide who receives authored systems.
Capabilities are what unlocks changes.
Effect targets decide where capability changes apply.
Scripted values are how the world is measured.
Triggers are when the world crosses a line.
Effects are what happens.
Events are why the player notices.
Domain packs organize systems.
Game modes assemble the game.
```

Or even shorter:

> **Define state. Modify it with overlays. Attach systems with install targets. Watch state with triggers. Explain change with events. Organize everything with domain packs.**

That is the SimThing modding philosophy.

---

## 20. Why Modders Should Care

SimThing is trying to make complex strategy-game systems more transparent and more composable.

As a modder, this means:

- your systems can interact naturally
- your bonuses and penalties are previewable
- your formulas are reusable
- your events can explain real simulation state
- your AI weights can use the same properties players see
- your techs, traditions, traits, and policies can share one modifier model
- your content can be organized into modular domain packs
- your install scopes are explicit
- your runtime state can survive replay through stable logical keys
- your rules can become visual in Studio

The dream is not just to make modding possible.

The dream is to make deep systemic modding **legible**.

If you can describe your system as state, modifiers, conditions, install scope, and consequences, SimThing should be able to help you build it.
