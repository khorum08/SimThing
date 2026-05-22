# SimThing Capability Tree — Concept Document V1
## Design Reference for SimThing Studios RON Format

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
and asset references are **metadata** that lives in the studio layer and
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
The overlay's `affects` field targets the owning Faction node so that when
activated, its `PropertyTransformDelta` propagates down through all spatial
children via Pass 3.

**Unlock** fires when the progress sub-field crosses `research_cost` in
Pass 7. The studio layer's boundary handler reads the threshold event,
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

One capability entry can carry multiple `sub_field_deltas` targeting the
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

Both overlays share the same `CapabilitySpec` entry. The studio layer
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

This is the V1 RON format for `simthing-studio`. It is the authoring
surface that `CapabilityTreeBuilder` consumes at session init.

### Top-level structure

```ron
CapabilityTreeSpec(
    tree_id:      "terran_tech_tree",
    tree_kind:    "tech_tree",          // becomes SimThingKind::Custom(tree_kind)
    owner_kind:   "Faction",            // which SimThingKind owns this tree
    categories: [
        // ...CapabilityCategorySpec entries...
    ],
)
```

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
- The studio layer enforces mutual exclusivity by suspending sibling entries
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
siblings. This is enforced by the studio layer at boundary time, not by
the simulation.

When the studio layer processes an `ActivateOverlay` for an idea entry, it
also issues `SuspendOverlay` for every sibling in the same category that
is currently active. The `max_active` field on `CapabilityCategorySpec`
declares the constraint; the studio layer enforces it.

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
