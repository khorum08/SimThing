# SimThing: A GPU-Native Recursive World State Architecture
## Design Document v4 — The Complete Specification

---

## 1. The Central Statement

A grand strategy simulation can be expressed as a single recursive data structure where every entity in the game — from the entire world down to a single population cohort — is an instance of the same type. That type is **SimThing**.

The world state lives in the GPU as dense vector matrices. It is continuously evaluated there. The CPU gives meaning to what the GPU computes — interpreting numbers as events, managing overlay lifecycles, and translating player and AI intent into transform deltas.

```
GPU owns:     world state as dense matrices, continuous evaluation
Feeder owns:  translation between semantic intent and GPU-native operations
CPU owns:     meaning, lifecycle, events, player interface
```

Everything that acts on the world is an **overlay**. Every meaningful quantity is a **property** whose sub-field layout is declared in the registry. Everything that differentiates entities — including rebellion, revolution, separatism, civil war, disease, diplomacy, and ethics — is expressed as property values crossing thresholds, not as discrete flags or special entity types.

```
One recursive type.               SimThing { properties, overlays, children }
One evaluation algorithm.         evaluate(ancestor_transforms) → FieldVector
One GPU pipeline.                 velocity → transform → reduce → threshold
One mechanism for change.         overlay TransformDelta on appropriate SimThing
One mechanism for differentiation. intensity threshold in the registry
One source of truth.              GPU dense matrices + CPU semantic interpretation
One place to edit any property.   the DimensionRegistry
```

Everything else is a consequence.

---

## 2. SimThing

Every entity in the simulation is a SimThing.

```rust
struct SimThing {
    id:          SimThingId,
    kind:        SimThingKind,
    properties:  HashMap<SimPropertyId, PropertyValue>,  // sparse, growable
    overlays:    Vec<Overlay>,                            // all effects
    children:    Vec<SimThing>,                           // spatial ownership
    spawned_day: u32,
}
```

The property container is a growable map keyed by `SimPropertyId`. A SimThing carries only the properties that are meaningful for its current state. Adding a new property dimension never changes this struct.

The spatial tree expresses physical ownership. It is largely static.

```
World
  └── Star Systems
        └── Locations  (planets, stations, outposts)
              └── Cohorts  (population masses)
```

Factions, regions, alliances, and all political structures are overlays on the spatial tree, not nodes within it. The tree is the physical map. The political map is expressed through overlays on it.

---

## 3. Properties — The Complete Model

Every meaningful quantity is a `PropertyValue` — a flat `Vec<f32>` whose meaning at every index is declared by the `SimProperty` definition in the `DimensionRegistry`. No hardcoded index constants exist anywhere in the codebase except inside `PropertyLayout` itself.

```rust
struct PropertyValue {
    data: Vec<f32>,    // flat, layout defined by registry
}
```

### SubFieldSpec — The Designer's Control Surface

The layout of every property is a `Vec<SubFieldSpec>`. Each spec declares one contiguous block of floats:

```rust
struct SubFieldSpec {
    role:          SubFieldRole,         // semantic identity
    width:         usize,                // 1 = scalar, N = vector of N floats
    clamp:         ClampBehavior,        // how values are bounded after integration
    velocity_max:  Option<f32>,          // optional cap on |velocity| before integration
    default:       f32,                  // initial value for each float in this block
    display_name:  String,               // for UI and observability tooling
    display_range: Option<(f32, f32)>,   // UI hint only, no simulation effect
    governed_by:   Option<SubFieldRole>, // which sub-field drives this one's rate of change
                                         // None = not evolved by integration
}
```

**`governed_by` is the integration control mechanism.** A sub-field advances by `governing_value * delta_time` each tick. The relationship between "position" and "rate of change" is explicit in the spec, not assumed. This means:

- Standard properties: `Amount` governed by `Velocity`
- Ethics axis: `Named("axis_position")` governed by `Named("axis_drift")`
- Population: `Named("count")` governed by `Named("growth_rate")`
- A velocity sub-field with no `governed_by` is only modified by overlay transforms

When a designer decides a sub-field should be governed by a different sub-field, or promoted from scalar to vector, they edit one `SubFieldSpec`. Everything downstream adjusts automatically.

### SubFieldRole

```rust
enum SubFieldRole {
    Amount,              // primary scalar value on the spectrum
    Velocity,            // rate of change for a governed sub-field
    Intensity,           // expression strength (0–1); drives fission conditions
    Named(String),       // designer-named: bonus vectors, proportions, pressures, etc.
    Custom(String),      // mod-defined; evaluator treats as generic float
}
```

`Named` replaces the old `VectorComponent(usize)`. A designer names sub-fields by what they mean, not by their position. A 3-wide ethics bonus vector is one `Named("ethics_bonus")` spec with `width: 3` — not three separately indexed components.

### ClampBehavior

Different properties at different levels of the hierarchy have fundamentally different bounding semantics. Clamping is per-sub-field, not per-property.

```rust
enum ClampBehavior {
    Bounded { min: f32, max: f32 },  // hard floor and ceiling
                                      // loyalty, food_security, ethics axis position
    Floored { min: f32 },            // floor only, unbounded upward
                                      // population, industrial_capacity, treasury
    Unbounded,                        // no clamping
                                      // vector components, signed pressures, aggregates
}
```

**Velocity pinning at boundaries:** when integration drives an amount to its floor or ceiling, the governing velocity is pinned to zero in the saturated direction. A cohort pinned at the loyalty floor cannot accumulate hidden negative velocity that would resist recovery. That kind of inertia belongs in vector components where it is observable and attributable — not hidden in a velocity that the player cannot read.

### PropertyLayout

```rust
struct PropertyLayout {
    sub_fields: Vec<SubFieldSpec>,
}
impl PropertyLayout {
    fn stride(&self) -> usize { /* sum of sub-field widths — computed, not stored */ }
    fn offset_of(&self, role: &SubFieldRole) -> Option<usize>
    fn width_of(&self,  role: &SubFieldRole) -> Option<usize>
    fn default_data(&self) -> Vec<f32>
    fn standard(vector_len: usize) -> Self  // backward-compatible: amount/velocity/intensity + N named vecs
}
```

`stride()` is computed from sub-field widths every time it is called. It is never stored. Adding a sub-field or changing a `width` automatically adjusts the GPU column count at next registration.

`PropertyLayout::standard(N)` produces the backward-compatible layout matching the old fixed model: Amount governed by Velocity, Intensity updated by IntensityBehavior, N named vector components. This is the default for `SimProperty::simple()`.

### SimProperty

```rust
struct SimProperty {
    // identity — the HashMap key; Eq and Hash on namespace+name only
    namespace: String,
    name:      String,
    // layout — fully declarative, designer-controlled
    layout: PropertyLayout,
    // behavior — all optional; defaults to dormant
    decay:              Option<DecayBehavior>,
    intensity_behavior: Option<IntensityBehavior>,
    fission_templates:  Vec<FissionThreshold>,
    fusion_templates:   Vec<FusionThreshold>,
    on_expire:          Option<ExpireHandler>,
    // metadata
    description:      String,
    intensity_labels: Vec<IntensityRange>,
}
```

`valid_range`, `default_velocity`, and `default_intensity` are removed. These are now declared per-sub-field in `PropertyLayout` via `ClampBehavior` and `SubFieldSpec::default`. The registry is the single place to edit any property's behavior.

### Dormant Properties

Most properties across most SimThings are dormant most of the time:

```
all sub-fields:  at their default values
all velocities:  0.0
all intensities: 0.0
governed_by:     produces 0 * dt = 0 change
```

Cost of a dormant property: floats that participate in matrix multiplies and produce no change. The capability is always there. The cost is always proportional to what is actually happening.

### Property Universality

The same `SimProperty` type handles every simulation domain without special cases:

| Domain | Example property | Layout notes |
|---|---|---|
| Cohort state | loyalty | Amount governed by Velocity; Intensity drives fission |
| Population | pop_count | Named("count") governed by Named("growth_rate"); Floored(0) |
| Ethics axis | spiritualism_materialism | Signed Bounded(-10,10); Named("population_proportion") width 1 |
| Ethics bonus | ethics_bonus | Named("ethics_bonus") width 3 = [research, stability, unity]; default 1.0 |
| Disease | plague_vector | Named("infection_rate"), Named("mortality"), Named("immunity"); transient overlay |
| Diplomacy | relationship | Signed Bounded(-1,1); Named("trust_momentum") Unbounded |
| Faction output | economic_output | Named("output") Floored(0); Unbounded upward |
| Aggregate | world_instability | Unbounded; reduction of faction pressure fields |

A designer adding the ethics bonus vector changes one `SubFieldSpec::width` from 1 to 3 and sets `default: 1.0`. GPU column count adjusts. UI shows three bars. Overlay transforms reference the sub-field by name. One edit, no cascade.

---

## 4. The Dimension Registry

The `DimensionRegistry` is the **single source of truth for all property layout knowledge**. It is the only place column arithmetic lives. No external code computes `slot * N_DIMS + dim`. Every system that needs to know where a sub-field lives asks the registry.

```rust
struct DimensionRegistry {
    properties:    Vec<SimProperty>,
    by_name:       HashMap<(String, String), SimPropertyId>,
    active:        Vec<bool>,
    column_ranges: Vec<PropertyColumnRange>,
    column_owners: Vec<(SimPropertyId, usize)>,
    total_columns: usize,
}
struct PropertyColumnRange {
    start:  usize,
    stride: usize,
}
impl PropertyColumnRange {
    fn col_for_role(&self, role: &SubFieldRole, layout: &PropertyLayout) -> Option<usize>
    fn col_range_for_role(&self, role: &SubFieldRole, layout: &PropertyLayout) -> Option<(usize, usize)>
}
```

`col_for_role` delegates to `layout.offset_of()` and adds `self.start`. This is the entire global column arithmetic for any property's any sub-field. Nothing else in the codebase does column math.

### Registration

```rust
let loyalty_id = registry.register(SimProperty {
    namespace: "core".into(),
    name:      "loyalty".into(),
    layout: PropertyLayout {
        sub_fields: vec![
            SubFieldSpec { role: SubFieldRole::Amount,   width: 1,
                           clamp: ClampBehavior::Bounded { min: 0.0, max: 1.0 },
                           governed_by: Some(SubFieldRole::Velocity), default: 0.0, ... },
            SubFieldSpec { role: SubFieldRole::Velocity, width: 1,
                           clamp: ClampBehavior::Bounded { min: -1.0, max: 1.0 },
                           governed_by: None, default: 0.0, ... },
            SubFieldSpec { role: SubFieldRole::Intensity, width: 1,
                           clamp: ClampBehavior::Bounded { min: 0.0, max: 1.0 },
                           governed_by: None, default: 0.0, ... },
            SubFieldSpec { role: SubFieldRole::Named("grievance_inertia".into()),
                           width: 1, clamp: ClampBehavior::Unbounded,
                           governed_by: None, default: 0.0, ... },
            SubFieldSpec { role: SubFieldRole::Named("faction_pull".into()),
                           width: 1, clamp: ClampBehavior::Unbounded,
                           governed_by: None, default: 0.0, ... },
            SubFieldSpec { role: SubFieldRole::Named("governance_drag".into()),
                           width: 1, clamp: ClampBehavior::Unbounded,
                           governed_by: None, default: 0.0, ... },
        ],
    },
    ...
});
// registry assigns columns 0–5
// col_for_role(Amount)                         = 0
// col_for_role(Velocity)                       = 1
// col_for_role(Intensity)                      = 2
// col_for_role(Named("grievance_inertia"))      = 3
// col_for_role(Named("faction_pull"))           = 4
// col_for_role(Named("governance_drag"))        = 5
```

### Designer Workflow

```
To change a property's behavior:
  1. Find the SimProperty definition
  2. Edit the relevant SubFieldSpec
  3. Done

To add a new sub-field to an existing property:
  1. Add a SubFieldSpec to the layout
  2. stride() recomputes
  3. GPU column count adjusts at next registration
  4. Done

To promote a scalar sub-field to a vector:
  1. Change SubFieldSpec::width from 1 to N
  2. Change SubFieldSpec::default to starting value
  3. Done — col_range_for_role returns the full range

To change which sub-field governs another:
  1. Change SubFieldSpec::governed_by on the governed spec
  2. Done — evaluator reads governed_by at runtime
```

### Column Tombstoning

Columns are never removed from the GPU matrix within a session. When a property type's last instance expires, its columns are tombstoned — marked inactive, available for reuse. The matrix grows to its session high-water mark and stays there. Tombstoned columns hold zero values and contribute nothing to evaluation.

---

## 5. Overlays — The Universal Mechanism

An overlay is anything that modifies SimThing evaluation without becoming a permanent part of its identity.

```rust
struct Overlay {
    id:        OverlayId,
    kind:      OverlayKind,
    source:    OverlaySource,
    affects:   Vec<SimThingId>,
    transform: PropertyTransformDelta,
    lifecycle: OverlayLifecycle,
}
struct PropertyTransformDelta {
    property_id:      SimPropertyId,
    sub_field_deltas: Vec<(SubFieldRole, TransformOp)>,
    // semantic intent — CPU prep resolves roles → column indices via registry
    // GPU sees only the resulting matrix coefficients
}
```

Overlay transforms reference sub-fields by `SubFieldRole`, not by column index. The CPU preparation pass calls `col_for_role` to resolve roles to column indices before GPU dispatch. An overlay that modifies `Named("ethics_bonus")` automatically gets the correct column range whether that sub-field is 1 wide or 3 wide.

Overlays unify every system that would otherwise require separate architecture:

| What it represents | How it appears |
|---|---|
| Regional governance | Overlay on faction, affects location ids |
| Empire policy | Overlay on world or faction |
| Alliance treaty | Overlay on world, affects member faction ids |
| Governor | Overlay on location |
| Orbital station | Overlay on location |
| Plague | Transient overlay on location |
| Technology effect | Overlay on research completion |
| Ethics pressure | Overlay modifying Named("axis_drift") velocity |
| Player instruction | Transient overlay with urgency vector |
| AI intent | Transient overlay with urgency vector |
| Fleet movement | Transient: destination + urgency, dissolves on arrival |
| Construction order | Transient spawning permanent overlay on completion |
| Crisis | Transient world-level overlay |

### Overlay Tombstoning

When an overlay dissolves, its GPU transform slot receives an identity matrix — the multiplicative equivalent of doing nothing. The slot persists in the matrix. Multiplying by identity costs one operation with no effect. The slot is reused by the next overlay of the same type.

### Instruction Overlays As Standing Registers

Player and AI instructions are overlays with urgency vectors and dissolution conditions. An instruction overlay slot holds an identity matrix when no instruction is active. The CPU only writes to it when intent changes. A quiet day — no new instructions, no dissolved instructions — costs zero CPU write operations on instruction slots.

Player and AI are completely symmetric. Both express intent as instruction overlays in the same language. The player's active instruction overlays are visible to the AI as strategic facts in the field.

---

## 6. Self-Managing Property Lifecycle

Properties manage their own expiry. No external scan required.

```rust
enum DecayBehavior {
    TowardZero     { rate: f32 },
    OnThreshold    { threshold: f32, direction: Direction },
    AfterTicks     { remaining: u32 },
    WhenProperty   { other: SimPropertyId, threshold: f32 },
    IntensityGated { intensity_floor: f32 },
}
```

When a SimProperty with decay behavior is added to a SimThing, its expiry condition is automatically registered as a GPU threshold. When the threshold fires at the day boundary, the CPU removes the key from the SimThing's property map.

```
PropertyValue instance  →  self-removes when decay condition met
                            GPU threshold detects; CPU removes at boundary
                            one HashMap remove, one column tombstone

SimProperty definition  →  persists in DimensionRegistry
                            columns stay warm in GPU matrix
                            reuse is zero-cost — columns already assigned
```

The property just won't be there the next day — in the sparse map, which is what the simulation reasons about. In the GPU matrix there are zeros in columns that no longer mean anything for that slot. The evaluation pipeline never branches on it.

---

## 7. Intensity-Driven Differentiation and Fission

There is no rebel cohort type. There is no loyalty state machine. There is no rebellion flag.

There is one property — loyalty — with a `SubFieldSpec` for Amount, Velocity, Intensity, and named vector components. The Amount is where loyalty currently sits on its spectrum. The Intensity is how strongly that position is expressing. The registry declares what combinations of Amount and Intensity are worth naming.

```
amount: 0.0 ──────────────────────────────────────── 1.0
              revolution  unrest  agitated  content  loyal

intensity: 0.0 ──────────────────────────── 1.0
                dormant    building    fully expressed
```

Semantic labels live in `IntensityRange` definitions on the `SimProperty`. They are read by the CPU semantic layer for display only. The simulation never sees them.

### IntensityBehavior

```rust
struct IntensityBehavior {
    velocity_threshold: f32,   // |velocity| below which intensity decays
    build_coefficient:  f32,   // intensity gain per unit |velocity| per day
    decay_coefficient:  f32,   // fraction of intensity shed per day when stable
}
```

Linear coefficients, not function pointers. Fully serializable. Maps directly to WGSL uniforms in the GPU pass. The same two numbers control intensity dynamics at every level of the hierarchy.

Intensity builds when the governed sub-field is changing rapidly. It decays when the sub-field stabilizes. A loyalty that has been falling for weeks carries high intensity — the change has been accumulating expression. A loyalty that stabilizes at a low value gradually loses intensity — defeat without energy.

### Escalating Fission Thresholds

```rust
struct FissionThreshold {
    dimension:  SimPropertyId,
    sub_field:  SubFieldRole,
    threshold:  f32,
    direction:  Direction,
    template:   FissionTemplate,
    secondary:  Option<SecondaryCondition>,
}
struct FissionTemplate {
    child_kind:                 SimThingKindTag,
    fusion_intensity_threshold: f32,
    fusion_scar_coefficient:    f32,
    resolution_label:           String,
}
```

Multiple thresholds on the same property produce escalating fission stages:

```
Unrest:      amount < 0.30, intensity > 0.35  → spawn unrest cohort
Resistance:  amount < 0.15, intensity > 0.55  → spawn resistance cohort
Rebellion:   amount < 0.05, intensity > 0.70  → spawn rebellion cohort
Revolution:  intensity > 0.90, amount < 0.10  → spawn revolutionary cohort
```

Each stage has a corresponding fusion threshold on the spawned child. Collapse mirrors escalation. A resistance cohort fuses back into the unrest cohort, which fuses back into the loyal cohort. The parent cohort carries a permanent vector mark on each fusion — the memory of what happened, expressed as elevated grievance inertia.

### Fission and Post-Fission Cost

```
pre-fission:   1 cohort row in GPU matrix
post-fission:  2 cohort rows in GPU matrix
additional cost:
  1 extra matrix multiply per GPU pass
  1 extra threshold registration
  no new overlay system, no new entity type, no new AI category
```

The rebellion is not a special game state. It is a location with two cohort rows instead of one.

### Ancestor Fission

When an ancestor SimThing fissions — a faction, a region, a star system — the fission propagates downward. Each child is assessed against the activating property's current Amount, Intensity, and named vector components to determine which side of the split it belongs to and what fraction of its population transfers.

The vectors already knew which side they belonged on. They had been evolving continuously for months or years before the ancestor fission triggered. The fission reads what was already there.

```
before: Tarkan Empire → 8 systems → 40 locations → 200 cohorts
after:  Tarkan Empire (loyalist rump)     → 5 systems, 28 locations, 140 cohorts
        Tarkan Separatists (new SimThing) → 3 systems, 12 locations, 60 cohorts
                                          + 4 contested locations, split populations
```

Nobody scripted which systems would rebel.

---

## 8. Evaluation — One Pass, Both Directions

```rust
impl SimThing {
    fn evaluate(&self, ancestor_transforms: &TransformStack) -> FieldVector {
        let local_transforms = self.overlays.iter()
            .fold(ancestor_transforms.clone(), |stack, overlay| {
                stack.compose(overlay.transform)
            });
        let child_vectors: Vec<FieldVector> = self.children
            .par_iter()
            .map(|child| child.evaluate(&local_transforms))
            .collect();
        let base           = self.reduce(child_vectors);
        let with_velocity  = self.integrate_properties(base);  // uses PropertyLayout
        let with_intensity = self.update_intensities(with_velocity);
        local_transforms.apply(with_intensity)
    }
}
```

The `ancestor_transforms` parameter is the downward pass. Every transform from every ancestor — world crisis overlays, faction policy overlays, regional governance, location context — is composed into the stack before any child evaluates.

Integration reads `SubFieldSpec::governed_by` to know which sub-field evolves which. Intensity update reads `SubFieldSpec::role == Intensity` and the property's `IntensityBehavior`. No hardcoded index assumptions exist in the evaluator.

**Note:** transforms are applied after velocity integration. A policy change on day N affects evaluation from day N+1 onward. This is intentional and matches the day-boundary protocol.

---

## 9. The GPU Pipeline

### CPU Preparation Pass

Before each tick, the CPU traverses the SimThing tree, composes ancestor transforms per node, and assembles the dense `EvaluationBatch`. During this pass:

- `PropertyTransformDelta` sub-field roles are resolved to global column indices via `registry.col_for_role()`
- Multi-width sub-fields produce a range of column coefficients
- `governed_by` relationships are encoded in the batch as column pairs
- All semantic knowledge is resolved to column indices before GPU dispatch

The GPU receives only floats at column indices. It has no awareness of sub-field roles, property names, or governed_by relationships.

### Seven GPU Passes

```
Pass 0:  Snapshot          previous_values ← values  (2.8 MB GPU memcpy)

Pass 1:  Velocity Integration
         for each (governed_col, governing_col, clamp, vel_max) pair:
           effective_vel = clamp(values[governing_col], -vel_max, vel_max)
           values[governed_col] += effective_vel × delta_time
           values[governed_col] = clamp_behavior.apply(values[governed_col])
           pin governing velocity at saturated boundary

Pass 2:  Intensity Update
         values[intensity_col] += build_coeff * |values[velocity_col]| * dt
         OR
         values[intensity_col] -= decay_coeff * values[intensity_col] * dt
         clamp to [0, 1]

Pass 3:  Cohort Transform Application
         ancestor_xform × local_xform × base_vector → output_vector
         (sub-field roles already resolved to column indices in prep pass)

Pass 4:  Location Reduction      reduce cohorts → location field vector
Pass 5:  System/Faction Reduction
Pass 6:  World Reduction         → global strategic field

Pass 7:  Threshold Scan
         for each registration: compare values vs previous_values
         write crossings to sparse event_candidates buffer
         includes value thresholds, intensity thresholds, velocity thresholds
```

### Endgame Memory Budget

```
Simulation parameters at maximum scale:
  11,520 SimThings  (1 world, 20 factions, 500 systems,
                     1,000 locations, 10,000 cohorts, ~1,000 fleets/stations)
  64 dimensions     (launch configuration)
  128 dimensions    (headroom for DLC/mod expansion)
  ~30,000 threshold registrations

Buffer sizes at 64 dimensions:
  Property values:              11,520 × 64 × 4B      =   2.8 MB
  Property velocities:          11,520 × 64 × 4B      =   2.8 MB
  Previous values:              11,520 × 64 × 4B      =   2.8 MB
  Local transform matrices:     11,520 × 64 × 64 × 4B = 182.0 MB
  Ancestor transform matrices:  11,520 × 64 × 64 × 4B = 182.0 MB
  Output vectors:               11,520 × 64 × 4B      =   2.8 MB
  Topology + threshold + misc:                        =   8.0 MB
  ──────────────────────────────────────────────────────────────
  Total at 64 dimensions:                             ≈ 383 MB
  Total at 128 dimensions:                            ≈ 728 MB
```

Both figures are well within the VRAM budget of any modern GPU. A midrange desktop has 8-12 GB. The entire endgame world state at maximum scale consumes less than 7% of available VRAM.

---

## 10. The Day Boundary

The day is the atomic unit of world state resolution and the natural synchronization primitive.

**Within a day:** GPU runs continuously. No authoritative state written. No structural mutations.

**At the day boundary:**

```
1.  GPU completes current dispatch
2.  Threshold scan reads event_candidates      (GPU → CPU, sparse)
3.  Feeder queue drains
4.  Overlay lifecycle resolves                 (dissolve, spawn, writeback)
5.  Property expiry resolves                   (HashMap removes, column tombstones)
6.  Fission and fusion events execute          (tree mutations)
7.  New instruction overlays applied           (player/AI for day N+1)
8.  DimensionRegistry and slot table sync
9.  Feeder patches GPU buffers                 (delta upload only)
10. Day N+1 dispatch begins
```

Time acceleration is a timer on the day boundary. The simulation always runs at one day per boundary cycle. The GPU never stops updating derived state. The map is alive even when paused.

---

## 11. The Feeder Thread Architecture

Three sub-threads with clear ownership:

**Transform Patcher** — continuous within day. Receives `PatchTransform` work items. Resolves `PropertyTransformDelta` sub-field roles to column indices via registry before writing to GPU buffer rows.

**Dispatch Coordinator** — continuous. Sequences GPU passes 0–7. Reads threshold candidates. Signals boundary completion.

**Tree Maintainer** — day boundary only. Handles slot allocation, deallocation, reparenting, and `AddDimension` events (when a new `SubFieldSpec` is registered mid-session by a mod or DLC).

---

## 12. Threshold Detection

GPU Pass 7 runs one thread per threshold registration. Output is a sparse `event_candidates` buffer.

```wgsl
@compute @workgroup_size(256)
fn threshold_scan(@builtin(global_invocation_id) id: vec3<u32>) {
    let reg = registry[id.x];
    let current  = values[reg.slot * N_DIMS + reg.dim];
    let previous = previous_values[reg.slot * N_DIMS + reg.dim];
    if crossing_detected(previous, current, reg.threshold, reg.direction) {
        let out_idx = atomicAdd(&candidate_count, 1u);
        candidates[out_idx] = ThresholdEvent {
            slot: reg.slot, dimension: reg.dim,
            value: current,  event_kind: reg.event_kind,
        };
    }
}
```

CPU reads at boundary: 4 bytes (count) + 0 to ~3 KB (crossings). Never reads full world state.

**Velocity thresholds** — registered on rate-of-change rather than value — give AI actors early warning before value thresholds are reached. The AI registers velocity thresholds on Named sub-fields it cares about and responds to trajectories.

---

## 13. Downstream Systems

### Presentation

Reads exclusively from GPU output buffers. Never tick-stale. Velocity indicators are free — the governing sub-field value is already in the GPU output. The UI reads `col_for_role(Velocity)` for any property and displays the directional arrow without additional computation.

### Network Play

Synchronizes semantic deltas only: overlay changes, structural mutations, threshold events. The GPU-native representation is not transmitted. Each client reconstructs it locally from the same authoritative semantic state. `PropertyTransformDelta` sub-field roles serialize cleanly — no column indices cross the wire, only semantic role names.

### Replay

The replay file is the semantic delta log — every overlay change, structural mutation, fission, and fusion. `SubFieldSpec::governed_by` relationships and `ClampBehavior` are part of the `SimProperty` definition serialized at session start. On playback, full simulation richness is reconstructed from that compact record.

### AI

AI reads the global strategic field and generates intent overlays. Velocity threshold registrations on Named sub-fields give early warning on developing situations. The AI can register a threshold on `Named("axis_drift")` of an ethics property to detect when a polity is shifting ideology faster than expected — before the axis position itself crosses any meaningful boundary.

### Observability

```
query: why is loyalty intensity high on Ptala IV, Kverlikon/bonded cohort?

decomposition:
  SubFieldRole::Amount:               0.08  (falling, velocity -0.008/day)
  SubFieldRole::Intensity:            0.71  (high — property expressing strongly)
  Named("grievance_inertia"):        +0.18  (dominant contributor)

transform contributions to Intensity (col_for_role resolves names to columns):
  extraction_policy_overlay:         ×0.97
  garrison_overlay:                  ×0.88
  martial_law_overlay:               ×0.82
  net intensity change/day:          -0.042 (declining — suppression working)

velocity threshold registered:
  Named("axis_drift") rising > 0.05/day → IdeologyShifting
  current rate: 0.008/day (below threshold)
```

---

## 14. Component Discipline: Auditable, Testable, Reversible

**Auditable:** The `SimProperty` definition is the documentation. What does this property do is completely readable from its `PropertyLayout`. What is currently affecting a Named sub-field is a query against the active overlay stack filtered by `col_for_role`.

**Testable:** Unit tests verify transform outputs for given layouts. The `custom_layout_ethics_axis` test pattern demonstrates how to verify any designer-defined layout before it reaches the simulation. Fission can be injected by setting sub-field values to threshold conditions in test fixtures.

**Reversible:** Changing a `SubFieldSpec` is a registry edit. The previous definition is in version control. Column rollback requires only a registry re-registration at session start.

---

## 15. The Emergent Richness

The interaction between systems is not authored — it is structural.

A freight efficiency technology modifies `Named("transport_capacity")` and `Named("labor_arbitrage")` on the faction policy overlay. Those named sub-fields exist in the evaluation context of every location and cohort. Every downstream calculation that touches them automatically reflects the change.

An ethics overlay modifying `Named("axis_drift")` on a cultural exchange fleet causes the ethics axis position of every location it visits to shift slightly each tick — because the overlay is on the fleet SimThing and propagates down through the evaluation tree. Nobody authored the connection between fleet movement and ideological drift. The tree delivered it.

The richness scales with the quality of sub-field and governed_by design, not with the quantity of authored content.

---

## 16. Implementation State

### Completed

**Week 1 — Semantic Foundation**
- `SimThing` struct with sparse `HashMap<SimPropertyId, PropertyValue>`
- `DimensionRegistry` with append-only column assignment, tombstoning, reverse lookup
- CPU reference evaluator with ancestor `TransformStack` propagation
- Deterministic `FieldSnapshot` with JSON serialize/deserialize round-trip
- 9 tests: velocity integration, ancestor propagation, determinism, snapshot round-trip, subtree size, column assignment, id lookup, tombstone/restore, duplicate-registration panic

**Property Generalization**
- `PropertyLayout { sub_fields: Vec<SubFieldSpec> }` — fully declarative, stride computed not stored
- `SubFieldSpec` with role, width, ClampBehavior, velocity_max, default, governed_by
- `ClampBehavior`: Bounded / Floored / Unbounded with at_floor / at_ceiling
- `SubFieldRole::Named(String)` replaces `VectorComponent(usize)`
- `TransformSemantics` removed; `SimProperty::valid_range/default_velocity/default_intensity` removed
- `PropertyValue::integrate(&layout, dt)` — layout-aware, governed_by driven, velocity pinning
- `PropertyValue::update_intensity(&ib, &layout, dt)` — layout-aware
- `PropertyColumnRange::col_for_role(role, layout)` / `col_range_for_role`
- `PropertyTransformDelta::apply_to_data(&mut data, &layout)` — no hardcoded indices
- `TransformStack::apply_to(id, value, layout)` — passes layout through
- `evaluate_node` fully layout-aware: integrate → update_intensity → apply_to all take layout
- 14 tests passing including `custom_layout_ethics_axis` (designer-defined layout with drift governor and width-3 bonus vector)

### Next — Week 2: GPU Foundation

- Add `wgpu` and `rayon` to workspace dependencies
- `simthing-gpu` crate
- `WorldGpuState`: buffer layout mirroring `DimensionRegistry` column ranges
- CPU preparation pass: tree walk → dense `EvaluationBatch` using `col_for_role`
- GPU Pass 0: snapshot (memcpy previous_values ← values)
- GPU Pass 1: velocity integration (governed_by pairs encoded as column pairs)
- GPU Pass 2: intensity update
- Exact output verification: GPU output matches CPU reference evaluator to float bit
- Memory budget measurement vs. projections

---

## 17. Summary

```
One recursive type.         SimThing { properties, overlays, children }
One evaluation algorithm.   evaluate(ancestor_transforms) → FieldVector
One GPU pipeline.           velocity → intensity → transform → reduce → threshold
One mechanism for change.   overlay TransformDelta referencing SubFieldRole by name
One source of truth.        GPU dense matrices + CPU semantic interpretation
One place to edit.          DimensionRegistry — SubFieldSpec governs everything

SubFieldSpec declares:
  role          what this block of floats means
  width         scalar (1) or vector (N)
  clamp         Bounded | Floored | Unbounded
  velocity_max  optional rate cap before integration
  default       starting value
  governed_by   which sub-field drives this one's integration
                None = modified only by overlay transforms

Player instructions are overlays.
AI intent is overlays.
Policies are overlays.
Technologies are overlays.
Ethics drift is an overlay on Named("axis_drift").
Disease is an overlay spawning Named("infection_rate") properties.
Diplomacy is an overlay on Named("relationship_value").
Rebellion is a cohort whose Amount is 0.03 and whose Intensity is 0.87.

Everything else is a consequence.
```

The simulation is not a state machine that produces events. It is a continuously evolving field that produces events when its values cross thresholds worth naming.

The player does not issue commands to a simulation. The player places overlays on a world.

The GPU does not accelerate a simulation. The GPU is the simulation.

The CPU does not run the world. The CPU understands it.

There is no rebel cohort. There is a cohort whose loyalty Amount is 0.03 and whose Intensity is 0.87.

There is no ethics system. There is a property whose Named("axis_position") is governed by Named("axis_drift"), whose Named("ethics_bonus") is a width-3 vector defaulting to 1.0.

The struct is the design. Build it.
