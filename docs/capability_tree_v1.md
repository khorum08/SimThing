# SimThing Capability Tree — Concept Document V1
## Design Reference for SimThing Spec-Layer RON Format

---

## 1. What a Capability Tree Is

A **Capability Tree** is a single `SimThing` of kind `Custom("tech_tree")`,
`Custom("national_ideas")`, `Custom("talent_tree")`, or any designer-chosen
label, attached as a direct child of an owning node — typically a Faction.

It is not a tree of SimThings. It is one SimThing that contains:

- **Properties** — float sub-fields tracking progress and unlock state
  for each capability entry. These live in the GPU matrix and are
  threshold-detected, reduced, and AI-observable like any other property.
- **Overlays** — one suspended overlay per capability entry carrying the
  effect payload. The overlay is invisible to the GPU pipeline until the
  entry is unlocked, at which point `ActivateOverlay` transitions it from
  `Suspended` to `Permanent` and Pass 3 begins applying it every tick.

The tree structure, prereq dependencies, research costs, display names,
and asset references are **metadata** that lives in the spec layer (`simthing-spec`)
in RON files. The simulation never sees them.

```
Faction (SimThing)
  └── TechTree (SimThing, Custom("tech_tree"))
        properties:
          tech::propulsion  → PropertyValue  ← GPU-tracked floats
          tech::industry    → PropertyValue
          tech::biology     → PropertyValue
        overlays:
          ion_drive_effect          → Suspended { when_activated: Permanent }
          warp_drive_effect         → Suspended { when_activated: Permanent }
          iron_smelting_effect      → Suspended { when_activated: Permanent }
          steel_production_effect   → Suspended { when_activated: Permanent }
          ...
```

---

## 2. How the Simulation Sees It

The simulation sees only floats and overlays. It has no concept of
"tech tree," "national ideas," or any specific capability system.

**Properties** are registered in `DimensionRegistry` under namespace
`"tech"` (or `"ideas"`, `"talents"`, etc.). Each category of capabilities
is one `SimProperty`. Each capability entry is two sub-fields:

```
Named("entry_id")       — progress float, 0.0 → research_cost
Named("entry_id_rate")  — research rate, governed_by drives integration
```

**Overlays** are attached to the capability tree node at session init
with `OverlayLifecycle::Suspended { when_activated: Box::new(Permanent) }`.

> **v0 install note (O1):** After `open_from_spec`, each owner gets a **cloned**
> capability-tree `SimThing`. Suspended effect overlays on that clone currently
> set `affects` to the **cloned tree id**, not the owning faction. See
> [§14 — v0 capability effect target semantics](#14-addendum--v0-capability-effect-target-semantics-opus-p3-pending).

**Unlock** fires when the progress sub-field crosses `research_cost` in
Pass 7. The spec layer's boundary handler reads the threshold event,
checks prereqs against the CPU shadow, and issues
`BoundaryRequest::ActivateOverlay` for the corresponding overlay. The
simulation executes the activation at boundary step 9.

**Reduction** — all tech sub-fields use `ReductionRule::Max`. The faction's
`output_vectors[entry_col]` = 1.0 the tick after the entry unlocks, making
"which factions have warp_drive" a flat buffer scan with no tree walk.

---

## 3. Overlay Payload Configurations

A capability entry's effect is a `PropertyTransformDelta` — a property id
and a list of `(SubFieldRole, TransformOp)` pairs. The following patterns
cover the majority of gameplay needs.

### 3a. Stat Bonus — Multiply

The most common payload. Multiplies a named sub-field on a property the
faction's territory carries.

```ron
effect: PropertyTransformDelta(
    property_id: "economy::industrial_output",
    sub_field_deltas: [
        (Named("output"), Multiply(1.15)),
    ],
)
```

Stacks multiplicatively with other overlays in Pass 3 evaluation order.
A faction with three +15% bonuses gets `1.15 × 1.15 × 1.15 ≈ 1.52`.

### 3b. Flat Bonus — Add

Adds a fixed amount to a property sub-field.

```ron
effect: PropertyTransformDelta(
    property_id: "military::unit_morale",
    sub_field_deltas: [
        (Amount, Add(0.1)),
    ],
)
```

### 3c. Multi-Property Bonus

One capability entry can carry multiple `sub_field_deltas` selection the
same or different properties. Each pair is one transform op in Pass 3.

```ron
effect: PropertyTransformDelta(
    property_id: "economy::trade",
    sub_field_deltas: [
        (Named("trade_value"),    Multiply(1.20)),
        (Named("trade_range"),    Multiply(1.10)),
        (Named("trade_capacity"), Add(2.0)),
    ],
)
```

Note: a single `PropertyTransformDelta` targets one `property_id`. For
bonuses spanning multiple properties, use multiple overlays — one per
property — all attached to the capability tree node and activated
together.

### 3d. Unlock Flag — Set

Boolean unlock flags for buildable buildings, ship components, units, or
weapon systems. The flag property carries one `Named` sub-field per
unlockable item, default 0.0, set to 1.0 on unlock.

```ron
// Unlock plasma_cannon ship component
effect: PropertyTransformDelta(
    property_id: "construction::unlocked_ship_components",
    sub_field_deltas: [
        (Named("plasma_cannon"), Set(1.0)),
    ],
)
```

The construction system queries
`output_vectors[faction_slot][plasma_cannon_col] >= 1.0` to know whether
the faction can build it. The reduction tier (`ReductionRule::Max`) surfaces
this to the faction level automatically.

### 3e. Multiple Unlock Flags in One Overlay

A single technology can unlock several items simultaneously.

```ron
effect: PropertyTransformDelta(
    property_id: "construction::unlocked_ship_components",
    sub_field_deltas: [
        (Named("plasma_thruster"),  Set(1.0)),
        (Named("afterburner"),      Set(1.0)),
        (Named("jump_capacitor"),   Set(1.0)),
    ],
)
```

### 3f. Unlock + Bonus in the Same Entry

Two overlays, same capability entry, same `when_activated`:

```ron
// First overlay: production bonus
effect_1: PropertyTransformDelta(
    property_id: "economy::industrial_output",
    sub_field_deltas: [(Named("output"), Multiply(1.25))],
)

// Second overlay: unlock steel foundry building
effect_2: PropertyTransformDelta(
    property_id: "construction::unlocked_buildings",
    sub_field_deltas: [(Named("steel_foundry"), Set(1.0))],
)
```

Both overlays share the same `CapabilitySpec` entry. The spec layer
attaches both to the capability tree node suspended, and activates both
together when the entry unlocks.

### 3g. Velocity Modifier

Affects the rate of change of a property rather than its current value.
Useful for growth rate bonuses, decay rate reductions, or drift effects.

```ron
effect: PropertyTransformDelta(
    property_id: "demographics::population",
    sub_field_deltas: [
        (Named("growth_rate"), Multiply(1.10)),
    ],
)
```

### 3h. Transient Effect on Unlock

A capability that applies a one-time burst effect rather than a permanent
bonus. Uses `when_activated: Transient` instead of `Permanent`.

```ron
// Crisis response tech: +0.3 loyalty pulse that decays over 10 boundaries
when_activated: Transient(
    dissolution_conditions: [AfterTicks(remaining: 10)],
)
effect: PropertyTransformDelta(
    property_id: "politics::loyalty",
    sub_field_deltas: [(Amount, Add(0.3))],
)
```

---

## 4. RON Format — CapabilityTreeSpec

This is the V1 RON format for **`simthing-spec`**. It is the authoring
surface that `CapabilityTreeBuilder` (in `simthing-spec`) consumes at session init.

### Top-level structure

```ron
CapabilityTreeSpec(
    tree_id:      "terran_tech_tree",
    tree_kind:    "tech_tree",          // becomes SimThingKind::Custom(tree_kind)
    owner_kind:   "Faction",            // metadata; install target selects owners (§13)
    install:      AllOfKind(kind: "Faction"),  // optional; defaults to AllOfKind Faction
    categories: [
        // ...CapabilityCategorySpec entries...
    ],
)
```

See [`docs/examples/README.md`](examples/README.md) for full `GameModeSpec`
examples of `AllOfKind`, `ScenarioListed`, and `SessionRoot`.

### Category

One category = one `SimProperty` registration in `DimensionRegistry`.

```ron
CapabilityCategorySpec(
    property_namespace: "tech",
    property_name:      "propulsion",
    display_name:       "Propulsion",
    entries: [
        // ...CapabilitySpec entries...
    ],
)
```

### Entry

One entry = two sub-fields (progress + rate) + one or more suspended
overlays + optional prereqs + all metadata.

```ron
CapabilitySpec(
    id:            "warp_drive",
    display_name:  "Warp Drive",
    description:   "Faster-than-light travel via spatial folding.",
    flavor_text:   "The distance between stars is a matter of perspective.",
    research_cost: 80000.0,

    // Assets — string refs, loaded lazily by asset pipeline
    icon:          "assets/tech/icons/warp_drive.png",
    thumbnail:     "assets/tech/thumbnails/warp_drive.png",
    card_image:    "assets/tech/cards/warp_drive.jpg",
    unlock_video:  Some("assets/tech/videos/warp_drive_unlock.mp4"),
    model_preview: Some("assets/models/engines/warp_drive.gltf"),

    // Prereqs — within same category by id, cross-category by category::id
    prereqs: [
        CapabilityPrereqSpec(category: "propulsion", entry_id: "ion_drive"),
    ],

    // Builder/construction unlocks
    unlocks_ship_components: ["warp_engine_mk1", "jump_drive_housing"],
    unlocks_buildings:       [],
    unlocks_units:           [],
    unlocks_weapons:         [],

    // Effect overlays — one per PropertyTransformDelta
    // All activated together when this entry unlocks
    effects: [
        CapabilityEffectSpec(
            targets_property: "military::fleet_speed",
            sub_field_deltas: [
                (Amount, Multiply(3.0)),
            ],
            when_activated: Permanent,
        ),
        CapabilityEffectSpec(
            targets_property: "construction::unlocked_ship_components",
            sub_field_deltas: [
                (Named("warp_engine_mk1"),      Set(1.0)),
                (Named("jump_drive_housing"),   Set(1.0)),
            ],
            when_activated: Permanent,
        ),
    ],
)
```

---

## 5. Example: Technology Tree — Terran Empire

A two-category linear tech tree demonstrating prereq chains, stat bonuses,
building unlocks, and multi-overlay entries.

```ron
CapabilityTreeSpec(
    tree_id:    "terran_tech_tree",
    tree_kind:  "tech_tree",
    owner_kind: "Faction",
    categories: [

        // ── Propulsion ────────────────────────────────────────────────────
        CapabilityCategorySpec(
            property_namespace: "tech",
            property_name:      "propulsion",
            display_name:       "Propulsion",
            entries: [

                CapabilitySpec(
                    id:            "chemical_drive",
                    display_name:  "Chemical Drive",
                    description:   "Standard reaction engines for in-system travel.",
                    research_cost: 5000.0,
                    icon:          "assets/tech/icons/chemical_drive.png",
                    prereqs:       [],
                    unlocks_ship_components: ["chemical_thruster"],
                    unlocks_buildings:       [],
                    effects: [
                        CapabilityEffectSpec(
                            targets_property: "military::fleet_speed",
                            sub_field_deltas: [(Amount, Multiply(1.10))],
                            when_activated: Permanent,
                        ),
                        CapabilityEffectSpec(
                            targets_property: "construction::unlocked_ship_components",
                            sub_field_deltas: [(Named("chemical_thruster"), Set(1.0))],
                            when_activated: Permanent,
                        ),
                    ],
                ),

                CapabilitySpec(
                    id:            "ion_drive",
                    display_name:  "Ion Drive",
                    description:   "Electromagnetic ion acceleration for faster transit.",
                    research_cost: 20000.0,
                    icon:          "assets/tech/icons/ion_drive.png",
                    prereqs: [
                        CapabilityPrereqSpec(
                            category: "propulsion",
                            entry_id: "chemical_drive",
                        ),
                    ],
                    unlocks_ship_components: ["ion_thruster", "ion_afterburner"],
                    unlocks_buildings: ["ion_drive_foundry"],
                    effects: [
                        CapabilityEffectSpec(
                            targets_property: "military::fleet_speed",
                            sub_field_deltas: [(Amount, Multiply(1.30))],
                            when_activated: Permanent,
                        ),
                        CapabilityEffectSpec(
                            targets_property: "construction::unlocked_ship_components",
                            sub_field_deltas: [
                                (Named("ion_thruster"),   Set(1.0)),
                                (Named("ion_afterburner"), Set(1.0)),
                            ],
                            when_activated: Permanent,
                        ),
                        CapabilityEffectSpec(
                            targets_property: "construction::unlocked_buildings",
                            sub_field_deltas: [(Named("ion_drive_foundry"), Set(1.0))],
                            when_activated: Permanent,
                        ),
                    ],
                ),

                CapabilitySpec(
                    id:            "plasma_drive",
                    display_name:  "Plasma Drive",
                    description:   "Magnetically confined plasma for high-thrust propulsion.",
                    research_cost: 80000.0,
                    icon:          "assets/tech/icons/plasma_drive.png",
                    prereqs: [
                        CapabilityPrereqSpec(
                            category: "propulsion",
                            entry_id: "ion_drive",
                        ),
                    ],
                    unlocks_ship_components: ["plasma_thruster", "plasma_afterburner"],
                    unlocks_buildings: ["plasma_refinery"],
                    effects: [
                        CapabilityEffectSpec(
                            targets_property: "military::fleet_speed",
                            sub_field_deltas: [(Amount, Multiply(2.0))],
                            when_activated: Permanent,
                        ),
                        CapabilityEffectSpec(
                            targets_property: "economy::industrial_output",
                            sub_field_deltas: [(Named("output"), Multiply(1.05))],
                            when_activated: Permanent,
                        ),
                        CapabilityEffectSpec(
                            targets_property: "construction::unlocked_ship_components",
                            sub_field_deltas: [
                                (Named("plasma_thruster"),   Set(1.0)),
                                (Named("plasma_afterburner"), Set(1.0)),
                            ],
                            when_activated: Permanent,
                        ),
                        CapabilityEffectSpec(
                            targets_property: "construction::unlocked_buildings",
                            sub_field_deltas: [(Named("plasma_refinery"), Set(1.0))],
                            when_activated: Permanent,
                        ),
                    ],
                ),

                CapabilitySpec(
                    id:            "warp_drive",
                    display_name:  "Warp Drive",
                    description:   "Faster-than-light travel via spatial folding.",
                    flavor_text:   "The distance between stars is a matter of perspective.",
                    research_cost: 200000.0,
                    icon:          "assets/tech/icons/warp_drive.png",
                    unlock_video:  Some("assets/tech/videos/warp_drive_unlock.mp4"),
                    prereqs: [
                        CapabilityPrereqSpec(
                            category: "propulsion",
                            entry_id: "plasma_drive",
                        ),
                        // Cross-category prereq: requires gravitic theory from physics
                        CapabilityPrereqSpec(
                            category: "physics",
                            entry_id: "gravitic_theory",
                        ),
                    ],
                    unlocks_ship_components: [
                        "warp_engine_mk1",
                        "jump_drive_housing",
                        "warp_stabilizer",
                    ],
                    effects: [
                        CapabilityEffectSpec(
                            targets_property: "military::fleet_speed",
                            sub_field_deltas: [(Amount, Multiply(5.0))],
                            when_activated: Permanent,
                        ),
                        CapabilityEffectSpec(
                            targets_property: "construction::unlocked_ship_components",
                            sub_field_deltas: [
                                (Named("warp_engine_mk1"),   Set(1.0)),
                                (Named("jump_drive_housing"), Set(1.0)),
                                (Named("warp_stabilizer"),   Set(1.0)),
                            ],
                            when_activated: Permanent,
                        ),
                    ],
                ),
            ],
        ),

        // ── Industry ──────────────────────────────────────────────────────
        CapabilityCategorySpec(
            property_namespace: "tech",
            property_name:      "industry",
            display_name:       "Industry",
            entries: [

                CapabilitySpec(
                    id:            "iron_smelting",
                    display_name:  "Iron Smelting",
                    description:   "Basic metalworking. Foundation of industrial capacity.",
                    research_cost: 3000.0,
                    icon:          "assets/tech/icons/iron_smelting.png",
                    prereqs:       [],
                    unlocks_buildings: ["iron_forge", "basic_factory"],
                    effects: [
                        CapabilityEffectSpec(
                            targets_property: "economy::industrial_output",
                            sub_field_deltas: [(Named("output"), Multiply(1.15))],
                            when_activated: Permanent,
                        ),
                        CapabilityEffectSpec(
                            targets_property: "construction::unlocked_buildings",
                            sub_field_deltas: [
                                (Named("iron_forge"),    Set(1.0)),
                                (Named("basic_factory"), Set(1.0)),
                            ],
                            when_activated: Permanent,
                        ),
                    ],
                ),

                CapabilitySpec(
                    id:            "steel_production",
                    display_name:  "Steel Production",
                    description:   "Advanced alloy refinement for structural materials.",
                    research_cost: 12000.0,
                    icon:          "assets/tech/icons/steel_production.png",
                    prereqs: [
                        CapabilityPrereqSpec(
                            category: "industry",
                            entry_id: "iron_smelting",
                        ),
                    ],
                    unlocks_buildings: ["steel_foundry", "heavy_industry_complex"],
                    unlocks_ship_components: ["reinforced_hull", "steel_plating"],
                    effects: [
                        CapabilityEffectSpec(
                            targets_property: "economy::industrial_output",
                            sub_field_deltas: [(Named("output"), Multiply(1.25))],
                            when_activated: Permanent,
                        ),
                        CapabilityEffectSpec(
                            targets_property: "military::unit_defense",
                            sub_field_deltas: [(Amount, Multiply(1.10))],
                            when_activated: Permanent,
                        ),
                        CapabilityEffectSpec(
                            targets_property: "construction::unlocked_buildings",
                            sub_field_deltas: [
                                (Named("steel_foundry"),          Set(1.0)),
                                (Named("heavy_industry_complex"), Set(1.0)),
                            ],
                            when_activated: Permanent,
                        ),
                        CapabilityEffectSpec(
                            targets_property: "construction::unlocked_ship_components",
                            sub_field_deltas: [
                                (Named("reinforced_hull"), Set(1.0)),
                                (Named("steel_plating"),   Set(1.0)),
                            ],
                            when_activated: Permanent,
                        ),
                    ],
                ),

            ],
        ),
    ],
)
```

---

## 6. Example: National Ideas Tree — Terran Empire

National ideas are a **mutually exclusive choice tree**: the player picks
one idea from each tier. The mechanism is identical to tech trees except:

- `tree_kind: "national_ideas"` instead of `"tech_tree"`
- `research_cost` is 0.0 — progress is set directly by player selection,
  not by research integration
- The spec layer enforces mutual exclusivity by suspending sibling entries
  when one is activated (CPU boundary logic — see §7)
- Ideas are typically organized into tiers rather than linear chains

```ron
CapabilityTreeSpec(
    tree_id:    "terran_national_ideas",
    tree_kind:  "national_ideas",
    owner_kind: "Faction",
    categories: [

        // ── Tier 1: Founding Principle ────────────────────────────────────
        CapabilityCategorySpec(
            property_namespace: "ideas",
            property_name:      "founding_principle",
            display_name:       "Founding Principle",
            tier:               1,
            max_active:         1,       // studio enforces — only one active at a time
            entries: [

                CapabilitySpec(
                    id:            "expansionist",
                    display_name:  "Expansionist",
                    description:   "The empire is defined by its reach.",
                    flavor_text:   "Every horizon is a border yet to be drawn.",
                    research_cost: 0.0,   // selected by player, not researched
                    icon:          "assets/ideas/icons/expansionist.png",
                    card_image:    "assets/ideas/cards/expansionist.jpg",
                    prereqs:       [],
                    effects: [
                        CapabilityEffectSpec(
                            targets_property: "military::fleet_capacity",
                            sub_field_deltas: [(Amount, Multiply(1.20))],
                            when_activated: Permanent,
                        ),
                        CapabilityEffectSpec(
                            targets_property: "economy::colonization_speed",
                            sub_field_deltas: [(Named("speed"), Multiply(1.30))],
                            when_activated: Permanent,
                        ),
                        CapabilityEffectSpec(
                            targets_property: "politics::stability",
                            sub_field_deltas: [(Amount, Add(-0.05))],
                            when_activated: Permanent,
                        ),
                    ],
                ),

                CapabilitySpec(
                    id:            "industrialist",
                    display_name:  "Industrialist",
                    description:   "Production is the foundation of power.",
                    flavor_text:   "Build enough and the stars will come to you.",
                    research_cost: 0.0,
                    icon:          "assets/ideas/icons/industrialist.png",
                    card_image:    "assets/ideas/cards/industrialist.jpg",
                    prereqs:       [],
                    effects: [
                        CapabilityEffectSpec(
                            targets_property: "economy::industrial_output",
                            sub_field_deltas: [(Named("output"), Multiply(1.25))],
                            when_activated: Permanent,
                        ),
                        CapabilityEffectSpec(
                            targets_property: "economy::research_output",
                            sub_field_deltas: [(Named("output"), Multiply(1.10))],
                            when_activated: Permanent,
                        ),
                    ],
                ),

                CapabilitySpec(
                    id:            "diplomatic",
                    display_name:  "Diplomatic",
                    description:   "Strength through alliances.",
                    flavor_text:   "The pen outlasts the sword, if wielded wisely.",
                    research_cost: 0.0,
                    icon:          "assets/ideas/icons/diplomatic.png",
                    card_image:    "assets/ideas/cards/diplomatic.jpg",
                    prereqs:       [],
                    effects: [
                        CapabilityEffectSpec(
                            targets_property: "politics::influence",
                            sub_field_deltas: [(Amount, Multiply(1.30))],
                            when_activated: Permanent,
                        ),
                        CapabilityEffectSpec(
                            targets_property: "economy::trade",
                            sub_field_deltas: [(Named("trade_value"), Multiply(1.20))],
                            when_activated: Permanent,
                        ),
                        CapabilityEffectSpec(
                            targets_property: "politics::stability",
                            sub_field_deltas: [(Amount, Add(0.10))],
                            when_activated: Permanent,
                        ),
                    ],
                ),

            ],
        ),

        // ── Tier 2: Military Doctrine ─────────────────────────────────────
        CapabilityCategorySpec(
            property_namespace: "ideas",
            property_name:      "military_doctrine",
            display_name:       "Military Doctrine",
            tier:               2,
            max_active:         1,
            entries: [

                CapabilitySpec(
                    id:            "fleet_in_being",
                    display_name:  "Fleet in Being",
                    description:   "Concentrated naval power as strategic deterrent.",
                    research_cost: 0.0,
                    icon:          "assets/ideas/icons/fleet_in_being.png",
                    prereqs:       [],
                    unlocks_ship_components: ["command_bridge_mk2", "fleet_coordination_array"],
                    effects: [
                        CapabilityEffectSpec(
                            targets_property: "military::fleet_combat",
                            sub_field_deltas: [(Amount, Multiply(1.25))],
                            when_activated: Permanent,
                        ),
                        CapabilityEffectSpec(
                            targets_property: "military::fleet_capacity",
                            sub_field_deltas: [(Amount, Add(5.0))],
                            when_activated: Permanent,
                        ),
                    ],
                ),

                CapabilitySpec(
                    id:            "guerrilla_warfare",
                    display_name:  "Guerrilla Warfare",
                    description:   "Distributed strikes over entrenched defense.",
                    research_cost: 0.0,
                    icon:          "assets/ideas/icons/guerrilla.png",
                    prereqs:       [],
                    unlocks_units: ["raider_corvette", "disruption_probe"],
                    effects: [
                        CapabilityEffectSpec(
                            targets_property: "military::unit_attrition",
                            sub_field_deltas: [(Amount, Multiply(0.70))],
                            when_activated: Permanent,
                        ),
                        CapabilityEffectSpec(
                            targets_property: "military::unit_speed",
                            sub_field_deltas: [(Amount, Multiply(1.20))],
                            when_activated: Permanent,
                        ),
                    ],
                ),

            ],
        ),

    ],
)
```

---

## 7. Example: Talent Tree — Faction Leader

A talent tree represents accumulated expertise and specialization choices
for a faction leader, governor, or admiral. Unlike tech trees (research
over time) or national ideas (one-time selection), talents are earned
by crossing thresholds on experience or achievement properties.

```ron
CapabilityTreeSpec(
    tree_id:    "leader_talent_tree",
    tree_kind:  "talent_tree",
    owner_kind: "Faction",   // or Location for a governor, Fleet for an admiral
    categories: [

        // ── Administrative ────────────────────────────────────────────────
        CapabilityCategorySpec(
            property_namespace: "talents",
            property_name:      "administrative",
            display_name:       "Administrative",
            entries: [

                CapabilitySpec(
                    id:            "efficient_bureaucracy",
                    display_name:  "Efficient Bureaucracy",
                    description:   "Streamlined governance reduces administrative overhead.",
                    research_cost: 500.0,   // 500 admin experience points
                    icon:          "assets/talents/icons/efficient_bureaucracy.png",
                    prereqs:       [],
                    effects: [
                        CapabilityEffectSpec(
                            targets_property: "economy::upkeep_cost",
                            sub_field_deltas: [(Amount, Multiply(0.90))],
                            when_activated: Permanent,
                        ),
                    ],
                ),

                CapabilitySpec(
                    id:            "master_administrator",
                    display_name:  "Master Administrator",
                    description:   "Elite governance unlocks advanced administrative tools.",
                    research_cost: 2000.0,
                    icon:          "assets/talents/icons/master_administrator.png",
                    prereqs: [
                        CapabilityPrereqSpec(
                            category: "administrative",
                            entry_id: "efficient_bureaucracy",
                        ),
                    ],
                    unlocks_buildings: ["administrative_center", "census_bureau"],
                    effects: [
                        CapabilityEffectSpec(
                            targets_property: "economy::upkeep_cost",
                            sub_field_deltas: [(Amount, Multiply(0.85))],
                            when_activated: Permanent,
                        ),
                        CapabilityEffectSpec(
                            targets_property: "politics::stability",
                            sub_field_deltas: [(Amount, Add(0.15))],
                            when_activated: Permanent,
                        ),
                        CapabilityEffectSpec(
                            targets_property: "construction::unlocked_buildings",
                            sub_field_deltas: [
                                (Named("administrative_center"), Set(1.0)),
                                (Named("census_bureau"),         Set(1.0)),
                            ],
                            when_activated: Permanent,
                        ),
                    ],
                ),

            ],
        ),

        // ── Military ─────────────────────────────────────────────────────
        CapabilityCategorySpec(
            property_namespace: "talents",
            property_name:      "military_talent",
            display_name:       "Military",
            entries: [

                CapabilitySpec(
                    id:            "tactician",
                    display_name:  "Tactician",
                    description:   "Combat instincts sharpen unit coordination.",
                    research_cost: 300.0,  // 300 military experience points
                    icon:          "assets/talents/icons/tactician.png",
                    prereqs:       [],
                    effects: [
                        CapabilityEffectSpec(
                            targets_property: "military::unit_combat",
                            sub_field_deltas: [(Amount, Multiply(1.10))],
                            when_activated: Permanent,
                        ),
                    ],
                ),

                CapabilitySpec(
                    id:            "legendary_commander",
                    display_name:  "Legendary Commander",
                    description:   "This leader's presence inspires extraordinary valor.",
                    research_cost: 5000.0,
                    icon:          "assets/talents/icons/legendary_commander.png",
                    unlock_video:  Some("assets/talents/videos/legendary_commander.mp4"),
                    prereqs: [
                        CapabilityPrereqSpec(
                            category: "military_talent",
                            entry_id: "tactician",
                        ),
                    ],
                    unlocks_units: ["elite_guard_regiment", "honor_fleet"],
                    effects: [
                        CapabilityEffectSpec(
                            targets_property: "military::unit_combat",
                            sub_field_deltas: [(Amount, Multiply(1.25))],
                            when_activated: Permanent,
                        ),
                        CapabilityEffectSpec(
                            targets_property: "military::unit_morale",
                            sub_field_deltas: [(Amount, Add(0.20))],
                            when_activated: Permanent,
                        ),
                        CapabilityEffectSpec(
                            targets_property: "construction::unlocked_units",
                            sub_field_deltas: [
                                (Named("elite_guard_regiment"), Set(1.0)),
                                (Named("honor_fleet"),          Set(1.0)),
                            ],
                            when_activated: Permanent,
                        ),
                    ],
                ),

            ],
        ),
    ],
)
```

---

## 8. Mutual Exclusivity (National Ideas)

National ideas require that activating one entry from a tier suspends its
siblings. This is enforced by the spec layer at boundary time, not by
the simulation.

When the spec layer processes an `ActivateOverlay` for an idea entry, it
also issues `SuspendOverlay` for every sibling in the same category that
is currently active. The `max_active` field on `CapabilityCategorySpec`
declares the constraint; the spec layer enforces it.

The simulation executes all `ActivateOverlay` and `SuspendOverlay` requests
in the same boundary step 9, so the transition is atomic at the day level.

---

## 9. Faction Fission and Inheritance

When a faction fissions and the `FissionTemplate` has
`clone_capability_children: true`, the spawned faction receives a deep
clone of every parent child whose `Custom(...)` kind appears in
`capability_container_kinds` (for example `tech_tree`, `national_ideas`,
`talent_tree`, or modder-defined labels). The clone carries:

- The same property values — research progress and unlock states copied
  from the parent's GPU shadow row at fission time
- Fresh `SimThingId`s — the child owns independent trees that diverge
  going forward
- Overlay `affects` remapped — suspended and active overlays that targeted
  the parent faction now target the child faction

The child faction starts with exactly the knowledge and ideas the parent
had at the moment of fission. What happens after that is its own history.

---

## 10. What the Simulation Never Knows

The following are entirely studio-layer concepts. The simulation crates
have no knowledge of them:

- The distinction between tech trees, national ideas, and talent trees
- Research costs — consumed at session init, become Pass 7 thresholds
- Display names, descriptions, flavor text, asset paths
- Tier numbers or `max_active` constraints
- The mapping between overlay ids and capability entry ids
- `unlocks_buildings`, `unlocks_units`, `unlocks_ship_components`,
  `unlocks_weapons` — these are `CapabilityDefinition` metadata read by
  the construction system, never by the simulation

The simulation sees: floats accumulating in property columns, thresholds
firing when they cross values, overlays transitioning from Suspended to
Permanent, and effects propagating to spatial children. Nothing else.

---

## 11. Addendum — `capability_container_kinds` (PR #38, 2026-05-22)

**Commit:** `a8aab5b` · **PR:** #38 · **Spec cross-ref:** `design_v6.md` preface
addendum (same change)

### What changed

The V1 concept document described faction fission inheriting
`tech_tree`, `national_ideas`, and `talent_tree` as if those names were
simulation knowledge. PR #38 removes that coupling. The simulation now clones
only children whose `SimThingKind::Custom(name)` appears in
`FissionTemplate::capability_container_kinds` — a list of opaque strings
authored by the spec layer (`simthing-spec`).

### Spec-layer responsibility

When defining a **faction fission template** in RON (or generated from
`CapabilityTreeSpec` / game rules), the spec layer must:

1. Set `clone_capability_children: true` when the spawned faction should inherit
   capability state from the parent.
2. Populate `capability_container_kinds` with every `Custom(...)` label used for
   capability containers attached to factions in that game/mod.

Example labels from this document: `"tech_tree"`, `"national_ideas"`,
`"talent_tree"`. Modders may add `"racial_abilities"`, `"policy_tree"`, or any
other label — no simulation recompile.

```ron
// On the SimProperty fission template for faction separatism / civil war:
FissionTemplate(
    child_kind:                 Faction,
    fusion_intensity_threshold: 0.8,
    fusion_scar_coefficient:    0.05,
    resolution_label:           "separatism",
    clone_capability_children:  true,
    capability_container_kinds: [
        "tech_tree",
        "national_ideas",
        "talent_tree",
        "racial_abilities",
    ],
)
```

Cohort or location fission templates omit both fields (serde defaults:
`clone_capability_children: false`, `capability_container_kinds: []`).

### Option A semantics (no sim fallback)

If `clone_capability_children: true` but `capability_container_kinds` is empty,
**nothing is cloned**. This is intentional: the studio must declare which kinds
qualify. There is no built-in default list in `simthing-sim`.

### What the simulation still never knows

Even after PR #38, the simulation does not know:

- Which strings are "capability trees" vs ordinary `Custom` children
- That `"tech_tree"` means research progress vs `"national_ideas"` means ideas
- Research costs, prereqs, display names, or RON metadata

It only knows: "clone parent children whose `Custom(name)` is in this list."

### §9 update (faction fission inheritance)

Section §9 remains valid with this clarification: inheritance is keyed by
`capability_container_kinds`, not by fixed kind names. The spec layer maps
each game's capability container labels into that list when authoring fission
templates.

### Tests proving the contract

- `clone_capability_children_empty_kinds_clones_nothing` — bool true, list empty
  → spawned faction has no capability children.
- `fission_clone_capability_children_remaps_affects_and_copies_shadow` — list
  contains `"tech_tree"` → clone + shadow copy + `affects` remap.
- `projected_fission_slots_counts_cloned_capability_subtrees` — pre-grow headroom
  uses the same list (asserts 3 slots for faction + 2-node tech subtree).

### Next spec-layer work (unchanged from V1 scope)

- `CapabilityTreeBuilder` session init: attach capability SimThings, seed
  suspended overlays, register property sub-fields.
- Boundary handler: read Pass 7 threshold events → issue `ActivateOverlay`.
- Faction fission template generation: populate `capability_container_kinds`
  from game rules / mod config alongside capability tree labels used in the mod.

---

## 12. Addendum — capability unlock event bridge + spec deps (PRs 4–10, 2026-05-22)

**Status:** Capability tree authoring through boundary handlers is landed in
`simthing-spec`. Session/driver assembly (PR 11) is **not yet implemented**.

### Handler input shape

`CapabilityTreeBoundaryHandler::handle_capability_unlock_events` now takes
`&[CapabilityUnlockEvent]` (defined in `simthing-feeder`), **not** raw
`ThresholdEvent`s. The session/driver layer is responsible for resolving GPU
events before calling the handler.

### Conversion bridge

Callers that hold raw Pass 7 events use:

```text
ThresholdRegistry::extract_capability_unlocks(threshold_events)
  → Vec<CapabilityUnlockEvent>
  → CapabilityTreeBoundaryHandler::handle_capability_unlock_events(...)
```

Non-`CapabilityUnlock` semantic arms and out-of-range `event_kind` values are
silently filtered (same pattern as `extract_scripted_event_triggers`).

### Entry point rename

The handler entry point was renamed from `handle_threshold_events` to
`handle_capability_unlock_events` during the spec→sim dependency cleanup.
Player-driven activation remains `handle_player_selection`.

### Crate boundary (production)

`simthing-spec` production dependencies are **`simthing-core` +
`simthing-feeder` only**. It must not depend on `simthing-sim` or
`simthing-gpu` in production code. `simthing-sim` and `simthing-gpu` remain
dev-dependencies for integration tests.

### Append helpers (Track B)

`ThresholdBuilder::append_capability_unlocks` exposes the existing private
push helper for B2 append-only session wiring (Track A / PR 11).

### Next

PR 11 session assembly and O1 install are landed. See
`docs/workshop/simthing_spec_progress_log.md` § Open work. Install-target
authoring: [§13](#13-addendum--installtargetspec-and-authored-kind-strings-o1-pr-53).
Effect scope: [§14](#14-addendum--v0-capability-effect-target-semantics-opus-p3-pending).

---

## 13. Addendum — InstallTargetSpec and authored kind strings (O1, PR #53)

**Status:** Landed in `simthing-driver::install` · **ADR:**
[`game_mode_session_installation.md`](adr/game_mode_session_installation.md) ·
**Examples:** [`docs/examples/README.md`](examples/README.md)

Each `CapabilityTreeSpec` carries an authored **`install`** field
(`InstallTargetSpec`) declaring **which scenario owners** receive a cloned
copy of the tree when `SimSession::open_from_spec` runs.

```ron
// Default when omitted — every Faction in the scenario tree:
install: AllOfKind(kind: "Faction"),

// Explicit list from the scenario (not the game mode file):
install: ScenarioListed(target_id: "player_faction"),

// One clone attached under Scenario::root:
install: SessionRoot,
```

### Resolution semantics (v0)

| Variant | Behavior |
|---------|----------|
| `AllOfKind { kind }` | Walk `Scenario::root`; install on every `SimThingId` whose kind **exactly** matches `kind` via `simthing_core::kind_matches`. |
| `ScenarioListed { target_id }` | Install on ids listed in `Scenario::install_targets[target_id]`. Missing key → hard error. Empty list → `NoMatchingOwners`. |
| `SessionRoot` | Install once on `Scenario::root.id`. |

If `AllOfKind` matches **zero** owners, install fails with
`InstallError::NoMatchingOwners`. There is no silent skip.

### Authored kind strings for `AllOfKind`

Matching is **intentionally simple and exact** in v0: case-sensitive string
equality against runtime `SimThingKind`. There is no tag system, fuzzy match,
or case folding.

**Built-in kinds** (use enum identifier spelling):

| Authored string | Runtime kind |
|-----------------|--------------|
| `"World"` | `SimThingKind::World` |
| `"Faction"` | `SimThingKind::Faction` |
| `"StarSystem"` | `SimThingKind::StarSystem` |
| `"Location"` | `SimThingKind::Location` |
| `"Cohort"` | `SimThingKind::Cohort` |
| `"Fleet"` | `SimThingKind::Fleet` |
| `"Station"` | `SimThingKind::Station` |

**Custom kinds** — match the label passed to `SimThingKind::Custom(name)`.
Examples from capability trees: `"tech_tree"`, `"national_ideas"`,
`"talent_tree"`. A modder-authored container `"racial_abilities"` matches only
when the runtime node is `Custom("racial_abilities")`.

`tree_kind` on `CapabilityTreeSpec` is the **label of the cloned capability-tree
node itself**, not the install filter. `install: AllOfKind(kind: "Faction")`
finds faction owners; the installed child is still `Custom(tree_kind)`.

### Out of scope (v0)

Tag selectors, owner expressions, dynamic filters, and scenario RON expansion
of `install_targets` are not implemented. Do not rely on undocumented matching
rules — if a kind string does not match, installation fails loudly.

---

## 14. Addendum — Capability effect target semantics (EffectTarget ADR)

**Status:** Accepted in [`adr/capability_effect_target_scope.md`](adr/capability_effect_target_scope.md)
· **Implementation:** landed · **Gates cleared:**
`simthing_modder_object_guide.md`, `simthing-studio` effect authoring UI

### What the ADR decides

`CapabilityEffectSpec` gains an authored `effect_target: EffectTarget`
selector with three variants and **`Owner` as the default**:

```ron
CapabilityEffectSpec(
    targets_property: "military::fleet_speed",
    sub_field_deltas: [(Amount, Multiply(3.0))],
    when_activated: Permanent,
    // effect_target: Owner    ← implicit when omitted
)
```

| Variant | Resolved `affects` at install time | Use case |
|---------|------------------------------------|----------|
| `Owner` *(default)* | `vec![owner_id]` | Faction-level bonuses, unlock flags, stat multipliers — the common case. |
| `CapabilityTree` | `vec![cloned_tree_id]` | Tree-internal counters, milestones, bookkeeping that should stay local to the clone. **v0 behavior.** |
| `SessionRoot` | `vec![scenario.root.id]` | Global era flags, world-state triggers other factions can observe. |

`#[serde(default)]` on `effect_target` keeps every existing RON file
parseable. **Runtime semantics flip** from clone-targeted to
owner-targeted unless modders opt into `CapabilityTree`. This is
intentional and pre-content; see ADR consequence (g).

### What changes in code

1. `install_tree_for_owner` in
   [`crates/simthing-driver/src/install.rs`](../crates/simthing-driver/src/install.rs)
   stops hard-coding `affects: vec![cloned_tree_id]` and routes through a
   `resolve_effect_target` helper per cloned overlay.
2. `CapabilityTreeBuildOutput` exposes effect-target provenance parallel
   to `tree.overlays` so the install loop can zip them.
3. `CapabilityPreviewInput` gains `owner_slot` and `root_slot`; the
   preview reads from whichever slot matches the effect's target so
   Studio previews show the actual transformed cell.
4. `CapabilityTreeBoundaryHandler::emit_activation` is **unchanged** —
   `target: instance.tree_thing_id` continues to point at the SimThing
   the overlay lives on (the clone). The O1b fix
   (`overlay_id` resolution via `instance.by_overlay`) is independent of
   this ADR and gates the ignored
   `open_from_spec_capability_unlock_activates_overlay_for_next_tick` test.

### Authoring rule going forward

`targets_property` columns must exist on the resolved target's slot. The
default `Owner` target means faction-facing properties should be
registered without kind restriction (or on the faction kind). The
Studio property editor can validate this statically when it ships.
