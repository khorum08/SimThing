# Capability Tree Pattern — Workshop Decisions
## Session: 2026-05-21

This document records design decisions reached in the workshop session on
the capability tree pattern. "Tech tree" is used as the worked example
throughout but the mechanism is general — it applies equally to national
ideas, talent trees, policy structures, racial ability trees, cultural
tradition trees, or any designer-defined progression system.

The simulation crates have no concept of "tech tree." The pattern is a
studio-layer primitive instantiated from RON specs. The spec documents
the primitive; the studio layer instantiates it for each use case.
It is a handoff document for implementation. Do not start writing code
until all decisions in a given section are marked **DECIDED**.

Reference documents: `docs/design_v6.md`, `docs/design_v5.md`,
`docs/chatgpt_implementation_review.md`, `docs/capability_tree_v1.md`.

---

## 1. Resource Production Schema

### 1a. Aggregate production via reduction tier

Per-resource production properties (`food_prod`, `factory_prod`,
`research_output`, etc.) are registered as `SimProperty` definitions
with `ReductionRule::Sum` on their Amount sub-field. The reduction tier
(GPU Passes 4–6) automatically aggregates cohort-level production up
through locations, systems, and factions into `output_vectors` every
tick. No separate `local_resource_prod` property is needed at any parent
level — `output_vectors[faction_slot][food_prod_col]` is the faction
total, available for free after each tick.

**DECIDED.**

### 1b. Base production value

The base resource production value for a cohort lives on the cohort as
a seeded `PropertyValue`. The seeding source (race template, spawn
logic, session initializer) is **not yet decided** — depends on the race
system design, which is explicitly left open. Overlays (terrain, policy,
faction bonuses) multiply or add on top of the seeded base value via
Pass 3.

**DECIDED** (schema). Race system seeding: **OPEN**.

### 1c. Global baseline via World-level overlay

Game-wide production baselines (e.g. "all biological cohorts produce at
least 1.0 food") are expressed as a `Permanent` overlay on the World
SimThing with `TransformOp::Add(n)`. The `n` value is configurable at
game creation time — the UI slider maps directly to the overlay's add
coefficient. Biological-only filtering depends on the race system design.

**DECIDED** (mechanism). Biological filter: **OPEN**.

---

## 2. Suspended Overlay Lifecycle (core architecture addition)

`OverlayLifecycle` gains a new variant:

```rust
pub enum OverlayLifecycle {
    Permanent,
    Transient { dissolution_conditions: Vec<DissolveCondition> },
    Suspended,   // present in CPU tree, skipped by GPU pipeline
}
```

A `Suspended` overlay exists structurally in the SimThing's `overlays`
vec. It is visible to the UI and to CPU prereq/observability queries.
It is **not** uploaded to the GPU overlay delta buffer —
`build_overlay_deltas` in `gpu_sync.rs` skips it. Pass 3 never applies
it. It does not prevent the static boundary fast-path.

Activation: `BoundaryRequest::ActivateOverlay { target: SimThingId,
overlay_id: OverlayId }` sets lifecycle from `Suspended` to `Permanent`
at boundary time. Cheap — no slot allocation, no tree reshape. The
overlay appears in the GPU delta buffer on the next tick.

This is a general architecture addition with utility beyond tech trees:
national ideas, policies, treaties, crisis responses, racial abilities —
anything defined upfront and activated when conditions are met.

**DECIDED.** Implement as its own PR before any tech tree code.

### Files affected

| File | Change |
|---|---|
| `simthing-core/src/overlay.rs` | Add `Suspended` variant; `is_active()` helper |
| `simthing-sim/src/gpu_sync.rs` | Skip suspended overlays in `build_overlay_deltas` |
| `simthing-sim/src/overlay_lifecycle.rs` | Skip suspended overlays in dissolution logic |
| `simthing-sim/src/delta_log.rs` | Record lifecycle state in `OverlayAttached` entries |
| `simthing-sim/src/observability.rs` | Distinguish suspended vs active in `OverlayContribution` |
| `simthing-sim/src/tree_mutation.rs` | Add `BoundaryRequest::ActivateOverlay` + handler |
| `simthing-sim/src/boundary.rs` | Route `ActivateOverlay` through step 9 |

---

## 3. Capability Tree Architecture

### 3a. The primitive is general

The capability tree pattern has no concept of "tech tree" in the
simulation crates. It is a named SimThing child with properties and
suspended overlays. The same primitive instantiated from different RON
files produces:

- Tech trees (`Custom("tech_tree")`)
- National ideas (`Custom("national_ideas")`)
- Talent trees (`Custom("talent_tree")`)
- Policy structures (`Custom("policy_tree")`)
- Racial ability trees (`Custom("racial_abilities")`)
- Cultural tradition trees (`Custom("traditions")`)
- Any other designer-defined progression system

The owning node is not restricted to Faction — any SimThing kind can
own a capability tree. A World-level capability tree would affect all
factions. A StarSystem-level tree could represent local development.
The pattern is the same regardless of where it attaches.

**DECIDED.**

### 3b. One capability SimThing per owning node

A capability tree is **one SimThing** of kind
`Custom("tech_tree")`, a direct child of the Faction node. There are no
individual SimThing nodes for individual techs.

The TechTree SimThing carries:

- **Properties** — one `SimProperty` per tech category, each with one
  sub-field per tech tracking research progress as a float. A paired
  rate sub-field governs integration. These live in `values[]` on the
  GPU and are tracked, threshold-detected, and reduced like any other
  property.
- **Overlays** — one `Suspended` overlay per tech carrying the effect
  `PropertyTransformDelta`. Each overlay's `affects` field targets the
  Faction node so Pass 3 propagates the effect to all spatial children
  on activation.

```
Faction (SimThing)
  └── TechTree (SimThing, Custom("tech_tree"))
        properties:
          metallurgy_pid  → PropertyValue { data: [0.0, 0.0, 0.0, 0.0] }
                                                   iron  iron  steel steel
                                                   prog  rate  prog  rate
          propulsion_pid  → PropertyValue { data: [0.0, 0.0, 0.0, 0.0] }
                                                   ion   ion   warp  warp
                                                   prog  rate  prog  rate
        overlays:
          iron_smelting_effect    → Suspended, affects: [faction_id]
          steel_production_effect → Suspended, affects: [faction_id]
          ion_drive_effect        → Suspended, affects: [faction_id]
          warp_drive_effect       → Suspended, affects: [faction_id]
```

**DECIDED.**

### 3c. Sub-field layout per entry

Each tech has two sub-fields in its category `SimProperty`:

```rust
// Progress sub-field — accumulates via integration
SubFieldSpec {
    role:               Named("plasma_drive"),
    width:              1,
    clamp:              Floored { min: 0.0 },  // unbounded upward, no ceiling
    default:            0.0,
    governed_by:        Some(Named("plasma_drive_rate")),
    reduction_override: Some(ReductionRule::Max),
}

// Rate sub-field — set by overlay when player allocates research
SubFieldSpec {
    role:               Named("plasma_drive_rate"),
    width:              1,
    clamp:              Floored { min: 0.0 },
    default:            0.0,
    governed_by:        None,
    reduction_override: Some(ReductionRule::Max),
}
```

`ReductionRule::Max` is mandatory on all tech sub-fields — it correctly
surfaces unlock state from the TechTree node up to the faction's
`output_vectors`. `Mean` would produce wrong results because spatial
sibling nodes (star systems, locations, cohorts) carry no tech columns
and would dilute the average to near zero. This constraint is enforced
by code convention in the studio layer — `SimProperty` code is unchanged.

**DECIDED.**

### 3d. Progress and unlock threshold

Research progress is a float that integrates from 0.0 toward
`research_cost` (e.g. 80,000.0 for plasma_drive) via Pass 1. The
`research_cost` value lives in `TechSpec` at authoring time and becomes
a Pass 7 threshold registration at session init. It does not exist as a
named value at runtime — it has been compiled into the threshold.

Pass 7 fires when the progress sub-field crosses `research_cost` in the
Rising direction. The studio layer's boundary handler then checks
prereqs and issues `BoundaryRequest::ActivateOverlay` if met.

**DECIDED.**

### 3e. Prereq checking

Prereq checking is performed by the studio layer's boundary handler
against the CPU shadow at boundary time. All prereqs read from the same
TechTree slot — same-node, same-property (within-category) or
same-node cross-property (cross-category). Both are simple array index
reads into the shadow. No tree traversal, no cross-node SimThing lookup.

Within-category prereq: read `Named("iron_smelting")` col on the same
slot as the triggering tech. Cross-category prereq: read a different
`SimPropertyId`'s column on the same slot. Both resolve via
`col_for_role` on the TechTree's slot index.

This does not require extending `SecondaryCondition` — prereq logic
lives entirely in the studio layer's boundary handler.

**DECIDED.**

### 3f. Reduction and parent-level queries

`ReductionRule::Max` on tech sub-fields means the faction's
`output_vectors[warp_drive_col]` = 1.0 the tick after the TechTree's
`warp_drive` progress crosses `research_cost` and the overlay activates.

Querying which factions have a tech unlocked is a flat buffer scan:

```rust
faction_slots.iter()
    .filter(|&&slot| reduced_field.row(slot)[warp_drive_col] >= 1.0)
    .map(|&slot| allocator.id_of(slot))
    .collect::<Vec<SimThingId>>()
```

No tree walk. O(n_factions) flat array reads. Answer available every
tick without explicit query work.

**DECIDED.**

### 3g. Reduction pollution invariant

The TechTree node carries no spatial properties (`food_prod`, `loyalty`,
etc.). Tech `SimProperty` registrations use namespace `"tech"` and must
not collide with spatial property names. Spatial reduction passes over
the TechTree contributing zero to spatial aggregates.

Enforced by convention in the studio layer. No simulation code change.

**DECIDED.**

---

## 4. Fission and Capability Tree Inheritance

### 4a. FissionTemplate flag

```rust
struct FissionTemplate {
    child_kind:                 SimThingKindTag,
    fusion_intensity_threshold: f32,
    fusion_scar_coefficient:    f32,
    resolution_label:           String,
    clone_capability_children:  bool,  // NEW
}
```

When `clone_capability_children: true`, `execute_fission` after spawning
the child faction also clones all `Custom("tech_tree")`,
`Custom("national_ideas")`, and `Custom("talent_tree")` children from
the parent faction into the child faction.

Default is `false` — no behavior change for cohort-level fissions.

**DECIDED.**

### 4b. Clone cost

With one TechTree node per faction the clone is:
- Deep-clone one `SimThing` struct
- Allocate one new slot
- Copy one shadow row (`memcpy` of `n_dims × 4B`)
- One pass over the cloned node's overlay vec remapping `affects` ids
  from parent faction id to child faction id

Trivially cheap. Pre-grow headroom: faction fission with
`clone_capability_children: true` needs 2 slots (faction + TechTree),
not 1. Pre-grow accounting updated accordingly.

**DECIDED.**

### 4c. Overlay affects remapping

After cloning the TechTree, a single pass replaces the parent faction id
with the child faction id in all cloned overlay `affects` fields. New
function `remap_overlay_affects(node, old_id, new_id)` in `fission.rs`.
Small, self-contained, independently testable.

**DECIDED.**

### 4d. Delta log and replay

`FissionOccurred { parent, node: SimThing }` carries the full spawned
faction subtree including the cloned TechTree. Replay reconstructs
correctly for free — no additional delta log changes needed.

**DECIDED.**

---

## 5. Studio Layer — CapabilityTreeBuilder (simthing-studio crate)

> **Superseded (2026-05-22):** Crate naming and placement changed. The RON→runtime
> compiler lives in **`simthing-spec`** (depends on `simthing-core` + `simthing-feeder`).
> **`simthing-studio`** is deferred GUI/editor only. Mechanism decisions in §5–8
> remain valid; substitute `simthing-spec` wherever this section names
> `simthing-studio` as the builder crate. Canonical handoff:
> `docs/workshop/simthing_spec_workshop.md`.

All tech tree authoring and runtime handling lives in `simthing-studio`.
The simulation crates (`simthing-core`, `simthing-gpu`, `simthing-feeder`,
`simthing-sim`) are unchanged except for the `Suspended` overlay addition.

### 5a. Authoring data structures

```rust
struct CapabilityTreeSpec {
    tree_id:    String,
    categories: Vec<CapabilityCategorySpec>,
}

struct CapabilityCategorySpec {
    property_namespace: String,
    property_name:      String,
    entries:            Vec<CapabilitySpec>,
}

struct CapabilitySpec {
    id:            String,       // matches Named sub-field name
    display_name:  String,
    description:   String,
    prereqs:       Vec<TechPrereqSpec>,
    research_cost: f32,          // becomes Pass 7 threshold — not stored at runtime
    effect:        PropertyTransformDelta,
}

struct CapabilityPrereqSpec {
    category: String,
    tech_id:  String,
}
```

### 5b. RON format

Tech trees are authored as RON files referenced from the scenario file:

```ron
// scenario.ron
FactionSpec(
    id: "terran_empire",
    tech_tree:      "specs/terran_tech_tree.ron",     // CapabilityTreeSpec
    national_ideas: "specs/terran_ideas.ron",     // CapabilityTreeSpec
    talent_tree:    "specs/terran_talents.ron"    // CapabilityTreeSpec,
)
```

Modders author RON files directly. The studio UI (when built) is a
graphical interface over the same RON format.

### 5c. TechTreeBuilder

At session init the studio layer reads the RON file and produces:

- `SimProperty` registrations (one per category)
- The TechTree `SimThing` with all properties at default and all
  suspended overlays attached
- A `CapabilityTreeDefinition` runtime lookup table mapping threshold events
  to prereqs and overlay ids
- Pass 7 threshold registrations (one per tech, threshold = research_cost)

`research_cost` is consumed here and not stored beyond this point.

### 5d. Runtime boundary handler

The studio layer processes `BoundaryOutcome` threshold events:
1. Looks up the fired tech in `CapabilityTreeDefinition`
2. Reads prereq sub-field values from shadow on the TechTree slot
3. If all prereqs met: issues `BoundaryRequest::ActivateOverlay`
4. If prereqs not met: issues intent delta resetting progress sub-field

### 5e. Impact preview

CPU-side calculation: apply the suspended overlay's
`PropertyTransformDelta` against current `output_vectors` shadow values
for the relevant columns. No GPU operation. Available at any time for
UI display of "what this tech would do if unlocked."

**All of §5 DECIDED.**

---

## 6. Open Questions

| # | Question | Status |
|---|---|---|
| Q1 | Race system design and cohort property seeding | OPEN — upstream |
| Q2 | Biological-race filter on World-level overlays | OPEN — depends on Q1 |
| Q3 | `ThresholdSemantic::UnlockTrigger` vs reuse of existing semantic | OPEN — decide at impl time |
| Q7 | Owning node kinds beyond Faction (World-level, system-level capability trees) | OPEN — design as needed |
| Q4 | Mutual exclusivity mechanism for national ideas | OPEN — defer |
| Q5 | GPU column budget vs full property set | OPEN — monitor |
| Q6 | `SimThingKind::Tech` promotion from `Custom("tech_tree")` | OPEN — ergonomics only |

---

## 7. Implementation Order

1. **`OverlayLifecycle::Suspended` + `BoundaryRequest::ActivateOverlay`**
   Standalone PR. No tech tree code until this lands.

2. **`FissionTemplate::clone_capability_children` +
   `remap_overlay_affects`**
   Small addition to `fission.rs`. Own PR.

3. **`simthing-studio` crate scaffold**
   `CapabilityTreeSpec`, `TechCategorySpec`, `TechSpec`, `CapabilityTreeBuilder`,
   `CapabilityTreeDefinition`. No UI. RON deserialization only.

4. **Session init wiring**
   Builder reads RON, registers properties, builds TechTree SimThing,
   registers Pass 7 thresholds. Verify GPU tracking and reduction work.

5. **Boundary handler**
   Prereq checking, `ActivateOverlay` issuance, progress reset on
   failed prereq.

6. **Impact preview**
   CPU-side calculation in studio observability layer.

7. **RON scenario expansion**
   Inline tech tree spec references in scenario files. Aligns with
   existing open work item in `docs/worklog.md`.

---

## 8. CapabilitySpec Asset and Semantic Payload

### 8a. Full CapabilitySpec RON structure

`CapabilitySpec` in the RON file carries three distinct concerns:

- **Mechanical** — the `PropertyTransformDelta` that becomes the suspended
  overlay effect. Goes into the simulation.
- **Semantic** — what the tech unlocks for builder/construction UIs.
  Goes into `CapabilityDefinition` in the studio layer.
- **Assets** — string references to art, UI, and media assets. Goes into
  `CapabilityDefinition`, loaded lazily by the asset pipeline.

```ron
TechSpec(
    id: "plasma_drive",
    display_name: "Plasma Drive",
    description: "Ionized plasma propulsion enabling faster-than-light travel.",
    flavor_text: "The stars are no longer distant.",
    research_cost: 80000.0,

    // UI assets — string refs, loaded by asset pipeline not simulation
    icon:          "assets/tech/icons/plasma_drive.png",
    thumbnail:     "assets/tech/thumbnails/plasma_drive.png",
    card_image:    "assets/tech/cards/plasma_drive.jpg",
    unlock_video:  "assets/tech/videos/plasma_drive_unlock.mp4",
    model_preview: "assets/models/engines/plasma_drive.gltf",

    // Builder UI unlock lists — semantic, not mechanical
    unlocks_ship_components: ["plasma_thruster", "afterburner", "jump_capacitor"],
    unlocks_buildings:       ["plasma_refinery"],
    unlocks_units:           [],

    // Mechanical effect — becomes the suspended overlay
    effect: PropertyTransformDelta(
        property_id: "economy::industrial_output",
        sub_field_deltas: [(Named("output"), Multiply(1.15))],
    ),

    prereqs: [
        TechPrereqSpec(category: "propulsion", tech_id: "warp_drive"),
    ],
)
```

**DECIDED.**

### 8b. TechTreeBuilder split

At session init `CapabilityTreeBuilder` reads the RON file and routes each
field to the correct destination:

```
RON TechSpec
  ↓ simulation side:
      SimProperty registrations (sub-fields, governed_by, reduction rules)
      TechTree SimThing (properties at default, suspended overlays attached)
      Pass 7 threshold registrations (threshold = research_cost)
  ↓ studio/UI side:
      TechTreeDefinition { overlay_id → TechDefinition }
```

`research_cost` is consumed here and does not exist at runtime beyond
the Pass 7 threshold registration.

**DECIDED.**

### 8c. CapabilityDefinition runtime struct

```rust
struct CapabilityDefinition {
    id:                     String,
    display_name:           String,
    description:            String,
    flavor_text:            String,
    // Asset refs — strings only, asset pipeline loads lazily
    icon:                   String,
    thumbnail:              String,
    card_image:             String,
    unlock_video:           Option<String>,
    model_preview:          Option<String>,
    // Builder UI unlock lists
    unlocks_ship_components: Vec<String>,
    unlocks_buildings:       Vec<String>,
    unlocks_units:           Vec<String>,
    // Runtime bridge
    overlay_id:             OverlayId,
    prereqs:                Vec<TechPrereq>,
    // research_cost NOT stored — became a Pass 7 threshold registration
}
```

**DECIDED.**

### 8d. Builder UI query pattern

The builder UI queries available options for a faction by:

1. Finding the faction's TechTree node
2. Filtering its overlays for `OverlayLifecycle::Permanent` (active techs)
3. Looking up each active overlay's `OverlayId` in `CapabilityTreeDefinition`
4. Collecting `unlocks_ship_components` / `unlocks_buildings` / etc.
   across all active `CapabilityDefinition` entries

Pure CPU. No GPU involvement. No tree walk beyond the TechTree node
itself. Fast enough for per-frame UI queries.

**DECIDED.**

### 8e. Modder access

All asset references and unlock lists are in RON files. Adding a new
tech with custom art, custom video, and custom ship component unlocks
is entirely a data authoring task — no Rust recompilation required.
The simulation crates never see asset paths, display names, or unlock
lists. The `OverlayId` is the only runtime bridge between simulation
state and studio metadata.

**DECIDED.**

---

## 9. Simulation Source Changes Required

This section is the complete and exhaustive list of changes required to
the simulation crates (`simthing-core`, `simthing-gpu`, `simthing-feeder`,
`simthing-sim`). Everything else lives in **`simthing-spec`** (compiler) with
**`simthing-studio`** deferred as GUI only.

### 9a. Design changes (require thought)

**`simthing-core/src/overlay.rs`** — Add `Suspended` variant:

```rust
pub enum OverlayLifecycle {
    Permanent,
    Transient { dissolution_conditions: Vec<DissolveCondition> },
    Suspended,  // NEW
}
```

Replace `is_permanent()` with `is_active()`:

```rust
pub fn is_active(&self) -> bool {
    matches!(self.lifecycle,
        OverlayLifecycle::Permanent | OverlayLifecycle::Transient { .. })
}
```

**`simthing-feeder/src/work.rs`** — Add `ActivateOverlay` and `SuspendOverlay` to `BoundaryRequest`:

```rust
pub enum BoundaryRequest {
    AddChild { .. },
    Remove { .. },
    Reparent { .. },
    AttachOverlay { .. },
    AddDimension { .. },
    ActivateOverlay {       // NEW
        target:     SimThingId,
        overlay_id: OverlayId,
    },
    SuspendOverlay {        // NEW
        target:     SimThingId,
        overlay_id: OverlayId,
    },
}
```

**`simthing-core/src/property.rs`** — Add `clone_capability_children`
to `FissionTemplate`:

```rust
pub struct FissionTemplate {
    pub child_kind:                 SimThingKindTag,
    pub fusion_intensity_threshold: f32,
    pub fusion_scar_coefficient:    f32,
    pub resolution_label:           String,
    pub clone_capability_children:  bool,  // NEW — default false
}
```

**`simthing-sim/src/fission.rs`** — Add capability child cloning and
overlay affects remapping to `execute_fission` when
`clone_capability_children: true`. New functions:
`clone_capability_children`, `remap_overlay_affects`.

### 9b. Mechanical completions (compiler-driven exhaustive match)

These touch no design decisions — they are match arm additions required
by the new variants above. The compiler will flag every location.

| File | Change |
|---|---|
| `simthing-sim/src/gpu_sync.rs` | `build_overlay_deltas` skips `Suspended` overlays |
| `simthing-sim/src/overlay_lifecycle.rs` | Dissolution logic skips `Suspended` overlays |
| `simthing-sim/src/delta_log.rs` | `OverlayAttached` records lifecycle state |
| `simthing-sim/src/observability.rs` | `OverlayContribution` distinguishes suspended vs active |
| `simthing-sim/src/tree_mutation.rs` | `ActivateOverlay` handler: find overlay by id, set lifecycle to `Permanent` |
| `simthing-sim/src/boundary.rs` | Route `ActivateOverlay` through step 9 structural mutations; `tree_has_boundary_lifecycle_work` must not treat `Suspended` as lifecycle work |
| `simthing-sim/src/boundary.rs` | Pre-grow headroom: faction fission with `clone_capability_children: true` needs 2 slots not 1 |

### 9c. What does not change

- `simthing-gpu` — no changes. GPU pipeline has no concept of suspended
  overlays or capability trees. `build_overlay_deltas` skipping suspended
  overlays is a `simthing-sim` change, not a GPU change.
- `simthing-feeder` beyond `work.rs` — no changes.
- `simthing-core` beyond `overlay.rs` and `property.rs` — no changes.
- `DimensionRegistry` — no changes.
- All GPU passes (0–7) — no changes.
- All shader code — no changes.

**DECIDED.**
