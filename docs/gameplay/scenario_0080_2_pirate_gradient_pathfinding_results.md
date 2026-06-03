# SCENARIO-0080-2: Pirate Gradient Pathfinding — Results Report

*For players, modders, and scenario designers. No engine internals assumed.*

---

## What this scenario proves

This test battery proves that a pirate faction AI can navigate a multi-system starmap using
**nothing but field readings and a threshold trigger** — no scripted routes, no target lists,
no pathfinding algorithm, no awareness of game rules. Direction emerges from the shape of a
numeric field. The pirate is drawn toward undisrupted, unpatrolled systems and leaves a fading
trail of disruption behind it that continually pushes it onward.

The scenario also proves the **disruption property itself** — a new value column that attaches
to any location (starsystem, planet, grid cell) and accumulates when a fleet is present, decays
when it leaves, can be suppressed by a patrol fleet, and feeds back into AI decisions without
requiring special-case code anywhere.

---

## The four things this battery tested

### 1 — Disruption is a real, persistent, decaying property on locations

Every location in the sim can carry a **disruption value** — an integer that rises when hostile
forces are present and falls when they leave. This battery tested four distinct node cases in
isolation over 20 ticks:

| Node | Conditions | What it proves |
|---|---|---|
| **Node 0** | Pirate present ticks 0–5, then quiet, then patrol arrives ticks 12–19 | Accumulate → natural decay → patrol-accelerated decay |
| **Node 1** | Nobody ever visits | Disruption stays zero; a clean system is clean |
| **Node 2** | Patrol always present, pirate never arrives | A guarded system stays at zero; patrols are a deterrent |
| **Node 3** | Pirate present every tick | Disruption saturates at the hard ceiling and never overflows |

**What passed:**
- Disruption rises proportionally to presence and falls geometrically when the pirate leaves.
- A patrol removes disruption faster than natural decay alone — patrol bonuses are meaningful.
- A system no one touches stays at zero permanently.
- A continuously raided system caps out, not overflows — the ceiling is a hard guarantee.
- Two identical 20-tick runs produce identical numbers down to the last integer (deterministic replay).

**What this means for modders:** The disruption property is tunable per-scenario via three
parameters — base decay rate, gain per raider unit, suppression per patrol unit — and any of
those can be further modified by faction tech unlocks or starsystem natural bonuses (broadcast
down as read-side weights, not as direct column writes).

---

### 2 — A compound desirability field correctly translates game-state into navigable terrain

Every location has a **desirability score** derived from its current disruption level and the
patrol presence there. This is the "map" the pirate reads when it decides where to go.

The formula tested:

```
desirability = BASE (50 000) − patrol_repulsion × patrols − disruption_penalty × disruption_units
               clamped to [0, max]
```

| Node conditions | Desirability | What it proves |
|---|---|---|
| No patrol, no disruption | 50 000 (maximum) | Untouched systems are most attractive |
| 1 patrol, no disruption | 35 000 | Patrols strongly reduce attractiveness |
| 3+ patrols | Floors to 0 | Heavily guarded systems are off-limits |
| Max disruption, no patrol | ~20 000 | Disrupted systems are still passable corridors |

**What passed:**
- Patrolled systems are consistently below base — patrols actually change AI behaviour.
- A maximally-disrupted system still has positive desirability — it is a corridor, not a wall.
  The AI can transit through systems it has previously raided to reach a clean one beyond.
- Clean (untouched, unguarded) systems always reach the full base score.
- At the end of 20 ticks the desirability ordering matches the designed intent:
  `clean node > recovering node > disrupted node > patrolled node`.

**What this means for modders:** The two penalty weights (`patrol_repulsion` and
`disruption_penalty`) are the primary tuning knobs. A high disruption penalty makes pirates
strongly route around their own trail. A low penalty lets them re-enter previously raided
systems quickly — piracy becomes more persistent and concentrated. Patrol repulsion tuning
determines how much fleet presence is needed to make a system truly off-limits.

---

### 3 — The gradient kernel extracts movement direction directly from field shape

A **gradient** is a direction arrow computed at each location: it points toward higher
desirability. This test battery proved that a single GPU kernel pass can extract both the
east-west and north-south components simultaneously (the `GradientXY` feature).

**What passed:**
- The dual-output kernel writes both direction components (X and Y) in one pass — proven to
  produce identical numbers to running two single-axis passes separately.
- The CPU calculation and the GPU hardware calculation agree within measurement tolerance —
  the AI behaviour is the same whether it runs on the CPU reference or the GPU.
- Producing two outputs from one dispatch never corrupts either output (no-aliasing proven).
- Bad configuration is caught at load time: if both outputs were pointed at the same column,
  the engine rejects it before the scenario starts.

**What this means for modders:** You don't need to think about the gradient kernel. It is
infrastructure. What matters is that the desirability field you define (through disruption
weights, patrol penalties, tech modifiers) *is* the AI's navigation map — shaping the field
is shaping the AI.

---

### 4 — The pirate navigates by reading its local gradient, not by planning a route

The full 20-tick scenario runs a pirate across a five-system line. The pirate has no list of
targets, no route, no memory of where it has been. Each tick it simply reads the direction
arrow at its current location and asks: *"Is the slope steep enough to move?"* If yes, it
takes one step. If no, it stays.

**What passed:**

**Self-disruption drives migration (the core AI loop):**
The pirate starts at system 0. Tick 0: it raids system 0, raising disruption there. The
disruption lowers system 0's desirability. The gradient now points east toward the still-clean
system 1. The threshold is crossed, an event fires, and the pirate steps east. It then raids
system 1, which starts to degrade, and the gradient shifts east again. Over 20 ticks the pirate
migrates across the line — not because it was told to, but because it keeps making each system
less desirable than the one ahead.

- Pirate travels at least 3 systems from its start position over 20 ticks.
- Visits at least 4 distinct systems.
- Takes at least 4 distinct movement steps.

**Threshold gating (commitment is meaningful):**
With the movement threshold raised to an extreme value, the gradient never crosses it and the
pirate never moves — it raids system 0 indefinitely as disruption saturates. This proves the
threshold is a real gate, not a formality. Lowering the threshold makes the AI more reactive;
raising it makes it more inertial.

**Patrols alter the trajectory:**
With a patrol stationed at system 2 (mid-line), the pirate's 20-tick path is measurably
different. The repulsion from system 2's reduced desirability reshapes the gradient and
produces a distinct final position and checksum — patrols have proven causal effect on AI
navigation.

**Every move is exactly one step:**
No tick produces a teleport or a multi-system jump. Each committed move crosses into a single
adjacent system only. The AI can move at most once per tick.

**Deterministic replay:**
Two identical 20-tick runs produce identical move sequences, positions, and checksums.

---

## The AI principles this battery demonstrates

### Principle 1 — The field is the policy

The pirate has no scripted behaviour and no awareness of game concepts like "patrol" or
"disruption." It reads a number at each neighbouring location and moves toward the higher one
when the difference is large enough. All of the interesting behaviour — route choice, patrol
avoidance, migration patterns — is a consequence of how the field is shaped by the game state.
Changing the field changes the AI without touching any AI logic.

### Principle 2 — Self-disruption is an emergent migration driver

The pirate is not given a "move on after raiding" instruction. It moves because raiding makes
its current position less desirable than the next one. This means:
- Raid intensity controls migration speed (heavier raiding degrades the location faster).
- Decay rate controls how long a raided system stays "hot" before becoming attractive again.
- The pirate naturally avoids freshly-raided systems — not by rule, but because the gradient
  doesn't point back until the disruption has faded.

### Principle 3 — Patrols work by shaping terrain, not by scripted chase logic

A patrol fleet makes its location repulsive in the desirability field. The pirate routes around
it because the gradient points away from it — not because there is any "flee from patrol" code.
This means:
- Patrol placement determines which routes the pirate avoids.
- A patrol that moves changes the shape of the field, which changes the AI's path.
- Multiple patrols cooperate naturally: they jointly suppress a region of the field without any
  coordination code.

### Principle 4 — Threshold gating makes commitment visible and tunable

Movement only happens when the desirability difference between a location and its neighbours
exceeds a tunable threshold. Below that threshold the pirate holds still even if there is a
gradient. This produces natural AI inertia — a pirate that has just moved into a fresh system
does not immediately move again until that system has degraded enough to open a gradient steep
enough to commit. The threshold is a designer-facing knob.

### Principle 5 — No CPU planner, no lookahead, no rules engine

Every movement decision is made with only local information: the desirability at adjacent
nodes this tick, compared to the threshold. The AI cannot see around corners, cannot predict
where patrols will be, and does not search for an optimal path. This is a feature, not a
limitation — it means the AI scales to arbitrarily large maps, costs the same per tick
regardless of map size, and produces behaviour that players can learn to predict and
strategically exploit.

---

## What is proven vs. what was explicitly not implemented

### Proven and in the engine

- Disruption accumulation, natural decay, patrol-accelerated decay, saturation ceiling.
- Compound desirability field (patrol repulsion + disruption penalty, configurable weights).
- Dual-output gradient kernel (both spatial direction components from one GPU pass).
- Gradient-follow movement: field-sourced, threshold-gated, one step per tick.
- Deterministic replay: same inputs → same outputs, always.
- Tech-modifier channel: decay rate can be tuned by faction tech unlock (broadcast as an
  overlay weight, not a direct column write).
- Decay-acceleration tech is admission-safe by construction; persistence-increasing tech
  requires explicit bounds check.

### Deliberately not in this scenario (each requires its own gate)

| Feature | Why not here |
|---|---|
| Up-aggregation (planet disruption rolls up to starsystem) | Architecture is correct; no scenario has pulled it yet |
| Down-broadcast (starsystem disruption shadows children) | Same — parked pending a named consumer |
| Dense per-cell temporal memory | Expensive VRAM gate; this scenario uses per-node (sparse) state only |
| Multi-step pathfinding / lookahead | Explicitly tested and rejected — the engine refuses this at admission |
| CPU planner / urgency computation | Same — refused at admission |
| UI, real-time loop, player command loop | Not in scope; would be a separate gate |
| Disruption affecting trade / happiness / culture | Named as a future scenario; the column exists, the penalty just needs a consumer |

---

## How to use this in a scenario you are designing

**To make pirates migrate predictably:** keep the disruption penalty high relative to the patrol
repulsion. Pirates will leave quickly after raiding and won't return until disruption fades.

**To make pirates persistent and concentrated:** lower the disruption penalty. Raided systems
remain attractive for longer; pirates circle back.

**To make patrols hard deterrents:** raise patrol repulsion. A single patrol fleet makes a
system effectively invisible to pirate gradients.

**To give a faction "disruption resistance" tech:** implement a tech unlock that contributes a
retention modifier ≤ 1 on the decay weight (a negative percentage on the ownership overlay).
This makes disruption fade faster on that faction's holdings. The engine validates that it can
only accelerate decay, never make it slower — the field stays bounded.

**To create piracy hotspots:** place systems with very low natural desirability (e.g. a
permanently patrolled bottleneck on one side, high disruption from prior raids on the other).
The gradient will funnel pirate traffic through adjacent gaps without any scripting.

---

*Scenario: SCENARIO-0080-2 "Pirate Gradient Pathfinding" — implementation COMPLETE.*
*Test summary: 17 + 18 + 30 + 17 = 82 tests, 0 failures across all four rungs.*
*GPU hardware parity verified (rung 3 dual-output gradient kernel).*
*Deterministic replay verified at every layer.*
