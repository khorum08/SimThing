# SimThing: A GPU-Native Recursive World State Architecture
## Design Document v5 — Implementation-Synchronized Specification

---

## Preface: From V4 to V5

Design v4 was the blueprint. V5 is the blueprint updated to match what was actually built. The
architecture is sound and was implemented faithfully, but six meaningful additions were made
during implementation that the v4 document does not cover:

1. **`ReductionRule` and the Reduction Tier** — a full reduction system (§5, §9) was not
   specified in v4 beyond a sketch. The implementation chose per-sub-field declarative rules,
   a WeightedMean variant, and a CPU oracle that must stay bit-exact with the GPU shader.

2. **The GPU Intent Delta Hot Path** — v4 described tick-time transforms as shadow writes +
   dirty-row uploads. The implementation replaced this with an affine `IntentDelta { mul, add }`
   GPU buffer that folds all per-tick patches into a single before-snapshot GPU pass, eliminating
   all read-modify-write CPU round-trips in the normal case.

3. **Consolidated Tick Command Submission** — v4 implied seven separate queue submissions per
   tick. The implementation records the entire tick pipeline (intent delta → snapshot → velocity
   → intensity → overlay → reduction → threshold) into one command encoder and submits once.
   2D workgroup dispatch handles WebGPU's per-axis limit at large slot counts.

4. **Static Boundary Fast-Path** — v4 did not address the cost of no-op boundaries. The
   implementation adds a `can_skip_empty_boundary` check: days with no threshold events, no
   pending requests, and no transient lifecycle work bypass the full 13-step boundary sequence
   entirely, costing zero CPU or GPU work.

5. **Aggregate Alerts on `output_vectors`** — v4's threshold system described threshold scanning
   only on `values` (per-entity). The implementation adds `AggregateAlertRegistration` for
   thresholding post-reduction `output_vectors` (parent aggregates), enabling AI to watch
   world-level or faction-level reduced fields rather than individual entity values.

6. **Two-Fidelity Observability** — v4 described a single observability query. The implementation
   has two: `observe` (cheap, CPU shadow, correct after any boundary) and `observe_live` (one
   GPU row readback, current integrated values, intended for UI/debug only).

None of these change the core architecture. They are implementations of capabilities that v4
either implied, sketched, or left undefined. The version bump to v5 is warranted because the
changes are substantive enough to make v4 misleading as a reference for anyone reading the code.

---

## 1. The Central Statement

A grand strategy simulation can be expressed as a single recursive data structure where every
entity in the game — from the entire world down to a single population cohort — is an instance
of the same type. That type is **SimThing**.

The world state lives in the GPU as dense vector matrices. It is continuously evaluated there.
The CPU gives meaning to what the GPU computes — interpreting numbers as events, managing overlay
lifecycles, and translating player and AI intent into transform deltas.

```
GPU owns:     world state as dense matrices, continuous evaluation
Feeder owns:  translation between semantic intent and GPU-native operations
CPU owns:     meaning, lifecycle, events, player interface
```

Everything that acts on the world is an **overlay**. Every meaningful quantity is a **property**
whose sub-field layout is declared in the registry. Everything that differentiates entities —
including rebellion, revolution, separatism, civil war, disease, diplomacy, and ethics — is
expressed as property values crossing thresholds, not as discrete flags or special entity types.

```
One recursive type.                SimThing { properties, overlays, children }
One evaluation algorithm.          evaluate(ancestor_transforms) → FieldVector
One GPU pipeline.                  intent → velocity → transform → reduce → threshold
One mechanism for change.          overlay TransformDelta on appropriate SimThing
One mechanism for differentiation. intensity threshold in the registry
One source of truth.               GPU dense matrices + CPU semantic interpretation
One place to edit any property.    the DimensionRegistry
One reduction rule per sub-field.  SubFieldSpec::reduction_override or role default
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

The property container is a growable map keyed by `SimPropertyId`. A SimThing carries only the
properties that are meaningful for its current state. Adding a new property dimension never
changes this struct.

The spatial tree expresses physical ownership. It is largely static.

```
World
  └── Star Systems
        └── Locations  (planets, stations, outposts)
              └── Cohorts  (population masses)
```

Factions, regions, alliances, and all political structures are overlays on the spatial tree, not
nodes within it. The tree is the physical map. The political map is expressed through overlays
on it.

---

## 3. Properties — The Complete Model

Every meaningful quantity is a `PropertyValue` — a flat `Vec<f32>` whose meaning at every index
is declared by the `SimProperty` definition in the `DimensionRegistry`. No hardcoded index
constants exist anywhere in the codebase except inside `PropertyLayout` itself.

```rust
struct PropertyValue {
    data: Vec<f32>,    // flat, layout defined by registry
}
```

### SubFieldSpec — The Designer's Control Surface

The layout of every property is a `Vec<SubFieldSpec>`. Each spec declares one contiguous block
of floats:

```rust
struct SubFieldSpec {
    role:              SubFieldRole,         // semantic identity
    width:             usize,                // 1 = scalar, N = vector of N floats
    clamp:             ClampBehavior,        // how values are bounded after integration
    velocity_max:      Option<f32>,          // optional cap on |velocity| before integration
    default:           f32,                  // initial value for each float in this block
    display_name:      String,               // for UI and observability tooling
    display_range:     Option<(f32, f32)>,   // UI hint only, no simulation effect
    governed_by:       Option<SubFieldRole>, // which sub-field drives this one's rate of change
                                             // None = not evolved by integration
    reduction_override: Option<ReductionRule>, // overrides role-default reduction (see §5)
}
```

`governed_by` is the integration control mechanism. A sub-field advances by
`governing_value * delta_time` each tick. `reduction_override` controls how this sub-field's
values are aggregated from children to parents during GPU Passes 4–6. Both are set in the same
`SubFieldSpec` — the designer's entire control surface for one block of floats is one struct.

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

### ClampBehavior

```rust
enum ClampBehavior {
    Bounded { min: f32, max: f32 },  // hard floor and ceiling
    Floored { min: f32 },            // floor only, unbounded upward
    Unbounded,                        // no clamping
}
```

**Velocity pinning at boundaries:** when integration drives an amount to its floor or ceiling,
the governing velocity is pinned to zero in the saturated direction.

### PropertyLayout

```rust
struct PropertyLayout {
    sub_fields: Vec<SubFieldSpec>,
}
impl PropertyLayout {
    fn stride(&self) -> usize            // computed, never stored
    fn offset_of(&self, role) -> Option<usize>
    fn width_of(&self, role) -> Option<usize>
    fn default_data(&self) -> Vec<f32>
    fn standard(vector_len: usize) -> Self
}
```

`PropertyLayout::standard(N)` produces the backward-compatible layout: Amount governed by
Velocity, Intensity updated by IntensityBehavior, N named vector components.

---

## 4. The Dimension Registry

The `DimensionRegistry` is the **single source of truth for all property layout knowledge**.

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
    fn col_for_role(&self, role, layout) -> Option<usize>
    fn col_range_for_role(&self, role, layout) -> Option<(usize, usize)>
}
```

`col_for_role` is the entire global column arithmetic for any property's sub-field. Nothing else
in the codebase does column math.

### Column Tombstoning

Columns are never removed from the GPU matrix within a session. When a property type's last
instance expires, its columns are tombstoned — marked inactive, available for reuse. The matrix
grows to its session high-water mark and stays there.

---

## 5. Reduction — The Presentation Tier (NEW in V5)

**This section was not specified in v4.** It describes the bottom-up reduction passes (GPU
Passes 4–6) that aggregate child values into parent rows for the presentation layer.

### ReductionRule

Every sub-field has a reduction rule that governs how child column values collapse into the
parent's `output_vectors` row. The rule is resolved from `SubFieldSpec::reduction_override`
if set; otherwise from the role default.

```rust
enum ReductionRule {
    Mean,                            // arithmetic mean of children's values
    Sum,                             // algebraic sum
    Max,                             // maximum (loudest child surfaces)
    Min,                             // minimum
    First,                           // first child in canonical slot order
    WeightedMean { by: SimPropertyId }, // mean weighted by another property's Amount
}
```

**Role defaults:**

| Role | Default rule | Rationale |
|---|---|---|
| Amount | Mean | A region's loyalty is the average of its cohorts' |
| Velocity | Mean | Rate-of-change averages, not sums |
| Intensity | Max | The loudest child voice surfaces at the parent |
| Named(_) | Mean | Reasonable default; override with Sum/Max where needed |
| Custom(_) | Mean | Same |

The default is almost always correct. `WeightedMean { by: pop_property_id }` is the primary
override case: loyalty weighted by cohort population rather than a bare average.

### Determinism Contract

Both the CPU oracle and the GPU shader iterate children in canonical slot order (ascending slot
index as assigned by `SlotAllocator`). Float addition is not associative — reordering children
diverges. This ordering contract is enforced: callers sort by (slot, col, event_kind) for any
parity test.

### Two Buffers

```
values         — per-slot post-Pass-3 state (individual entity values)
output_vectors — per-slot post-reduction aggregates (parent views of their subtrees)
```

Leaves: `output_vectors[slot] = values[slot]` (copied from post-Pass-3 state).
Inner nodes: each column is reduced across children using `column_rules[col]`.

Pass 7 can scan either buffer via `ThresholdRegistration.buffer` (`THRESH_BUF_VALUES` or
`THRESH_BUF_OUTPUT`). This enables both per-entity thresholds (individual cohort loyalty
crossing 0.3) and aggregate thresholds (faction-wide average loyalty dropping below 0.4).

---

## 6. Overlays — The Universal Mechanism

An overlay is anything that modifies SimThing evaluation without becoming a permanent part of
its identity.

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
}
```

Overlay transforms reference sub-fields by `SubFieldRole`, not by column index. The CPU
preparation pass calls `col_for_role` to resolve roles to column indices before GPU dispatch.

Overlays unify every system that would otherwise require separate architecture. See v4 §5 for
the full table.

### Overlay Tombstoning

When an overlay dissolves, its GPU transform slot receives an identity operation. Dissolution is
recorded in the delta log as `BoundaryDeltaEntry::OverlayDissolved { target, overlay_id }`.

---

## 7. Self-Managing Property Lifecycle

Properties manage their own expiry. No external scan required. (Same as v4 §6.)

```rust
enum DecayBehavior {
    TowardZero     { rate: f32 },
    OnThreshold    { threshold: f32, direction: Direction },
    AfterTicks     { remaining: u32 },
    WhenProperty   { other: SimPropertyId, threshold: f32 },
    IntensityGated { intensity_floor: f32 },
}
```

---

## 8. Intensity-Driven Differentiation and Fission

Same model as v4 §7. Fission lineage is now a first-class persistent structure.

### Fission Lineage

`BoundaryProtocol` maintains a persistent `Vec<FissionLineageRecord>` across boundaries:

```rust
struct FissionLineageRecord {
    parent_id:   SimThingId,
    child_id:    SimThingId,
    property_id: SimPropertyId,
    template_idx: usize,
}
```

Every successful fission appends a record. `ThresholdBuilder` emits one `FusionTrigger`
registration per record, watching the child's activating-property Intensity column. When
the fusion trigger fires, `execute_fusion` applies a multiplicative scar to the parent's
Amount: `parent.amount *= (1 - fusion_scar_coefficient)`.

Records are pruned when either endpoint is tombstoned (by Remove, fusion, or reparent).
Pruning runs twice per boundary: after fission (step 6) and after structural mutations
(steps 7–8).

**Fission re-fire policy:** recurring rebellions are intentional. A `FissionTrigger`
remains live after spawning; if the activating Amount re-crosses the threshold in a later
boundary, a new child spawns. No suppression latch. Idempotency within a single boundary
only (deduplicate by `(parent_id, template_idx)`).

---

## 9. Evaluation — One Pass, Both Directions

Same as v4 §8. The CPU reference evaluator (`Evaluator` in `simthing-core::evaluate`) is the
oracle. GPU output must match it to the float bit on every pass. Intermediate `let` bindings
in WGSL shaders prevent FMA fusion (Option B decision, verified bit-exact on naga + DX12).

---

## 10. The GPU Pipeline

### GPU Passes — Eight, Not Seven (UPDATED in V5)

V4 described seven passes. The implementation has an additional pass before Pass 0: the
**Intent Delta Pass**. The complete sequence is:

```
Intent Pass:  GPU-side affine intent deltas applied to values[] (BEFORE snapshot)
Pass 0:  Snapshot  values → previous_values
                   output_vectors → previous_output_vectors
Pass 1:  Velocity Integration
Pass 2:  Intensity Update
Pass 3:  Overlay Transform Application (iterative, not matrix)
Pass 4:  Leaf Copy       output_vectors[leaf] = values[leaf]
Pass 5:  Inner Reduction bottom-up, depth by depth (deepest first)
Pass 6:  (continuation of Pass 5 — one dispatch per tree depth)
Pass 7:  Threshold Scan  scans values[] or output_vectors[] per registration
```

All passes after the intent pass are recorded into **one command encoder** and submitted
in a single `queue.submit`. This is a hard implementation constraint: multiple submissions
per tick add unnecessary synchronization overhead at large slot counts.

### The Intent Delta Pass (NEW in V5)

Per-tick feeder/player/AI transforms are folded on the CPU into affine `IntentDelta` records
before GPU dispatch:

```rust
struct IntentDelta {
    slot: u32,
    col:  u32,
    mul:  f32,   // applied as: values[slot * n_dims + col] = values[...] * mul + add
    add:  f32,
}
```

Multiple operations to the same `(slot, col)` within one tick are folded into a single
`IntentDelta` on the CPU before upload. Fold rules:
- `Set(k)`:      `(mul=0.0, add=k)` — resets any accumulated mul/add
- `Add(a)`:      `add += a`
- `Multiply(m)`: `mul *= m` and `add *= m` (preserves any pending add)

The intent buffer is uploaded and applied **before Pass 0**. This is intentional: Pass 0
snapshots `values → previous_values`. Applying patches after the snapshot would cause every
threshold on a patched cell to fire spuriously on the next tick — a phantom crossing from the
upload, not from simulation. Applying first absorbs the patch into the previous-state reference
frame.

**RMW elimination:** the primary motivation. V4's shadow path needed a GPU readback before any
Add/Multiply to avoid stale RMW. The intent delta buffer eliminates this entirely: Add and
Multiply fold directly into `(mul, add)` pairs without reading the current GPU value.

Player and AI intents follow a **two-phase path**:
1. Mid-tick: transform delta folded into the intent buffer (same-tick effect on GPU values).
2. Boundary: full `Overlay` structurally attached to the tree (persistent Pass 3 effect).

For `Set`, mid-tick and Pass 3 are idempotent. For `Add`/`Multiply`, mid-tick applies once
and boundary attach makes the effect persistent on subsequent ticks — no double-counting.

### 2D Workgroup Dispatch

WebGPU has a per-axis dispatch group limit of 65535. At >4M invocations (e.g., 1M slots × 4
dims, or large overlay delta counts), a 1D dispatch overflows. All passes use a 2D dispatch:

```
groups = ceil(total_invocations / WORKGROUP_SIZE)
x = min(groups, 65535)
y = ceil(groups / 65535)
dispatch_workgroups(x, y, 1)
```

Shaders decode their linear thread index as `gid.x + gid.y * 65535 * WORKGROUP_SIZE`.
This is transparent to callers: `dispatch_linear(pass, n)` handles it.

### Pass 0 — Snapshot (UPDATED in V5)

Pass 0 now snapshots two buffers, not one:

```
values          → previous_values
output_vectors  → previous_output_vectors
```

`previous_output_vectors` is the baseline for Pass 7 aggregate threshold crossing detection.
Without it, aggregate thresholds could not detect crossings between ticks.

### Pass 7 — Threshold Scan (UPDATED in V5)

Each `ThresholdRegistration` now carries a `buffer` field:

```
buffer: THRESH_BUF_VALUES  (0) — scans values[] vs previous_values[]
buffer: THRESH_BUF_OUTPUT  (1) — scans output_vectors[] vs previous_output_vectors[]
```

This allows AI-registered `AggregateAlertRegistration` instances to watch post-reduction
parent aggregates rather than individual entity values. A world-level instability aggregate
can trigger an early-warning alert without requiring a threshold on every individual cohort.

### GPU Memory Budget (same as v4)

At 64 dimensions, endgame scale (11,520 SimThings):

```
Property values:              11,520 × 64 × 4B      =   2.8 MB
Property velocities:          11,520 × 64 × 4B      =   2.8 MB
Previous values:              11,520 × 64 × 4B      =   2.8 MB
Output vectors:               11,520 × 64 × 4B      =   2.8 MB  (NEW)
Previous output vectors:      11,520 × 64 × 4B      =   2.8 MB  (NEW)
Overlay deltas:               ~4 MB                             (replaces transform matrices)
Intent delta buffer:          per-tick, uploaded/consumed       (NEW)
Topology + threshold + misc:                        =   8.0 MB
─────────────────────────────────────────────────────────────────
Total at 64 dimensions:                             ≈ 393 MB
Total at 128 dimensions:                            ≈ 738 MB
```

Both figures remain well within the VRAM budget of any modern GPU. V4's projection of 383 MB
was for values only; V5 includes output_vectors and previous_output_vectors.

---

## 11. The Day Boundary (UPDATED in V5)

The day boundary sequence has grown from 10 steps to 13 steps, with GPU amortized growth
and lineage maintenance as explicit phases:

```
0.  Pre-flight: read GPU values back into coord.shadow (integration output is GPU-only)
1.  Collect velocity alerts and aggregate alerts from threshold events
2.  Overlay lifecycle resolves (dissolve + expire effects → shadow mutations)
3.  Property expiry resolves (threshold-driven + CPU TowardZero/AfterTicks sweep)
4.  Pre-grow GPU state for fission headroom (amortized doubling)
5.  Fission and fusion execute (spawns children, seeds from shadow, applies scar)
6.  Lineage maintenance (append new records, remove fused records, prune tombstoned)
7.  Drain boundary requests from patcher (player/AI intents converted to AttachOverlay)
8.  Pre-grow GPU state for AddChild headroom
9.  Structural mutations execute (AddChild, Remove, Reparent, AttachOverlay, AddDimension)
10. Dimension rebuild if AddDimension expanded total_columns
11. Final capacity sync (one more grow if step 9 added slots)
12. GPU sync (overlay deltas, thresholds, reduction topology, full shadow upload)
13. Delta log emission (entries_from_outcome, using one-pass tree index for O(1) lookups)
```

### Static Boundary Fast-Path (NEW in V5)

A boundary can be **skipped entirely** when:
- No threshold events fired in the last tick
- No pending boundary requests in the patcher (player/AI intents included)
- No transient overlays or CPU-decay properties anywhere in the tree

`BoundaryProtocol::can_skip_empty_boundary` checks all three conditions. When satisfied,
the caller skips `execute()` entirely — no GPU readback, no lifecycle scan, no shadow
upload. At static-map scale with no active events, this reduces boundary cost to near zero.

### GPU Growth (UPDATED in V5)

V4 did not address what happens when fission or AddChild causes slot count to exceed the
initial GPU buffer allocation. The implementation uses **amortized doubling**:

```
required_slots > current_n_slots:
    new_n_slots = current_n_slots
    while new_n_slots < required_slots: new_n_slots *= 2
    coord.resize_slots(new_n_slots)
    patcher.resize(new_n_slots)
    state.rebuild_for_slots(new_n_slots, &registry)
```

The CPU shadow is the preservation source during growth: GPU buffers are reallocated and
receive the full shadow upload after resizing. Growth happens twice per boundary (pre-fission
and pre-AddChild) plus a final capacity sync, ensuring no step operates on undersized buffers.

---

## 12. The Feeder Thread Architecture (UPDATED in V5)

Three sub-roles from v4 §11, now fully implemented:

**Transform Patcher** — continuous within day. Two modes:
- **Intent delta path (normal):** `apply_collected_as_intents` folds feeder/AI/player items
  into `Vec<IntentDelta>`, returned to the dispatcher for GPU upload. No shadow mutation,
  no RMW readback. Structural `BoundaryRequest` items are parked for boundary time.
- **Shadow path (direct/replay):** `apply_collected` mutates the CPU shadow directly. `Set`
  applies immediately; `Add`/`Multiply` require `ShadowFreshness::GpuSynced` to guard against
  stale RMW. Used for session seeding and replay-style callers.

**Dispatch Coordinator** — continuous. Owns the CPU shadow. `tick()` sequence:
  1. Drain feeder channel + fold AI items → intent deltas
  2. Upload intent deltas to GPU
  3. Upload legacy dirty shadow rows (for direct-path callers)
  4. `run_tick_pipeline(state, dt)` — one encoder, all passes, one submit
  5. Read threshold event count and candidates
  6. Advance tick/day counters

**Tree Maintainer** — day boundary only. `apply_structural_mutations` handles every
`BoundaryRequest` variant with real execution bodies.

---

## 13. Threshold Detection (UPDATED in V5)

Pass 7 runs one thread per threshold registration. Output is a sparse `event_candidates`
buffer. Early-return when `n_thresholds == 0` — zero cost for days with no registrations.

**Three threshold categories:**

```
FissionTrigger    — per-entity, scans values[]
FusionTrigger     — per-child, scans child's Intensity col in values[]
PropertyExpiry    — per-entity, scans values[]
VelocityAlert     — AI-registered, scans values[] (velocity sub-field)
AggregateAlert    — AI-registered, scans output_vectors[] (post-reduction)
```

The last category is new in v5. AI registers `AggregateAlertRegistration` on a specific
SimThing's post-reduction output (e.g., faction-level loyalty average). Pass 7 scans
`output_vectors` for those registrations and surfaces crossings through
`BoundaryOutcome::aggregate_alerts`. This is the AI early-warning system for developing
situations at strategic scale.

CPU reads at boundary: 4 bytes (count) + 0 to ~3 KB (crossings). Same as v4.

---

## 14. Observability — Two Fidelity Levels (UPDATED in V5)

V4 described a single observability query. V5 has two, with explicit fidelity semantics:

```rust
// Cheap. Reads from CPU shadow. Correct after any boundary.
// May lag mid-tick integration on rows not patched this tick.
BoundaryProtocol::observe(coord, target) -> Option<ObservabilityReport>

// One GPU row readback per call. Live integrated values.
// Intended for inspector UI and debug only, not per-tick batch queries.
BoundaryProtocol::observe_live(coord, state, target) -> Option<ObservabilityReport>
```

Both return:

```rust
struct ObservabilityReport {
    sim_thing_id: SimThingId,
    properties: Vec<PropertyObservation {
        property_id:   SimPropertyId,
        property_name: String,          // "namespace::name"
        sub_fields:    Vec<SubFieldObservation { role, value }>,
        overlay_contributions: Vec<OverlayContribution {
            overlay_id: OverlayId,
            source:     OverlaySource,
            deltas:     Vec<(SubFieldRole, TransformOp)>,
            inherited:  bool,           // true = lives on an ancestor, not this node
        }>,
    }>,
}
```

`OverlayContribution::inherited` correctly attributes policies on a World-level SimThing
as separate from local governance on the queried SimThing. This mirrors the design intent
from v4 §13 but was not formally specified there.

---

## 15. The Delta Log and Replay

### Delta Log

`BoundaryProtocol` accumulates a `Vec<BoundaryDeltaEntry>` each boundary. Callers drain with
`take_delta_log()`. The complete set of entry variants:

```rust
enum BoundaryDeltaEntry {
    OverlayAttached    { target: SimThingId, overlay: Overlay },
    OverlayDissolved   { target: SimThingId, overlay_id: OverlayId },  // NEW in V5
    SimThingAdded      { parent: SimThingId, node: SimThing },
    SimThingRemoved    { id: SimThingId },
    DimensionAdded     { property_id: SimPropertyId },
    FissionOccurred    { parent: SimThingId, node: SimThing },
    FusionOccurred     { ... },
    PropertyExpired    { sim_thing_id: SimThingId, property_id: SimPropertyId },
    SimThingReparented { child: SimThingId, new_parent: SimThingId },
    VelocityAlert      { sim_thing_id, property_id, sub_field, value },
    AggregateAlert     { sim_thing_id, property_id, sub_field, value }, // NEW in V5
    FissionLineageAdded   { record: FissionLineageRecord },              // NEW in V5
    FissionLineageRemoved { record: FissionLineageRecord },              // NEW in V5
}
```

`OverlayDissolved` closes the lifecycle loop: v4 captured attachment but not dissolution.
`AggregateAlert` entries carry observation-only context for replay audit.
`FissionLineageAdded/Removed` entries allow `ReplayDriver` to reconstruct fusion threshold
registrations correctly — without them, replay reconstructs a tree where fission happened
but fusion thresholds are never registered for subsequent boundaries.

Delta log emission builds a one-pass tree index (`SimThingId → &SimThing` and
`SimThingId → parent_id`) before emitting entries, giving O(1) lookups instead of
re-scanning the entire tree per entry.

### Replay

LDJSON format: `ReplaySnapshot` as first line, `ReplayFrame` per boundary thereafter.

```rust
struct ReplaySnapshot {
    day:             u32,
    root:            SimThing,
    registry:        DimensionRegistry,
    fission_lineage: Vec<FissionLineageRecord>,  // NEW in V5 — needed for fusion triggers
}
struct ReplayFrame {
    day:           u32,
    entries:       Vec<BoundaryDeltaEntry>,
    shadow_values: Option<Vec<f32>>,  // optional post-boundary numeric checkpoint
}
```

`shadow_values` is the numeric audit trail. Replay does not re-run GPU passes — structural
reproduction only. `shadow_values` provides reference data for diffing.

`ReplayDriver` reconstructs: tree + registry + allocator + fission lineage from snapshot,
then applies frames via structural mutations equivalent to the live `BoundaryProtocol`.

---

## 16. Downstream Systems

### Presentation

Reads from `output_vectors` (post-reduction). Faction-level loyalty is the weighted mean
of location loyalties, which is the weighted mean of cohort loyalties. The reduction tier
makes this the natural output for presentation — not a separate aggregation step.

### Network Play

Unchanged from v4. Semantic deltas only: overlay changes, structural mutations, threshold
events. `PropertyTransformDelta` sub-field roles serialize cleanly — no column indices
cross the wire, only semantic role names.

### AI

AI registers velocity alerts on `values[]` (trajectory warnings on individual entities)
and aggregate alerts on `output_vectors[]` (strategic situation tracking on parents).
The velocity alert fires before the value threshold; the aggregate alert fires when the
faction-wide picture crosses a strategic boundary. Together they give the AI both tactical
early warning and strategic awareness.

### Observability

See §14. The two-fidelity model (shadow for steady-state, GPU row for live debug) is the
primary tool for answering "why is X high on Y?" both in production and during development.

---

## 17. Component Discipline: Auditable, Testable, Reversible

**Auditable:** `SubFieldSpec::reduction_override` is now also part of the audit trail — the
sub-field's reduction semantics are as readable as its integration semantics.

**Testable:** GPU/CPU parity tests use `to_bits()` equality, not approximate comparison.
The CPU reduction oracle and GPU shader both iterate children in canonical slot order, making
float-exact parity achievable. The `custom_layout_ethics_axis` test remains the proof of the
layout generalization. `pass3_overlay_matches_evaluator` is the proof of iterative transform
correctness. `weighted_mean_reduction_matches_cpu_oracle` is the proof of reduction parity.

**Reversible:** Same as v4. Registry edits in version control.

---

## 18. Implementation State

**187/187 tests passing, zero warnings, master current through B1 targeted
boundary value upload.**

### Complete

- `simthing-core` — all types, CPU evaluator, registry, reduction rules
- `simthing-gpu` — all 8 passes, intent delta pipeline, consolidated tick submission,
  2D dispatch, WeightedMean reduction, output_vectors threshold scanning, amortized growth
- `simthing-feeder` — intent delta hot path, reusable fold accumulators, static boundary
  skip, sparse dirty-row tracking, player/AI intent two-phase path, patcher boundary/intent
  separation
- `simthing-sim` — full 13-step boundary protocol, `tree_index` for fission + structural
  lookups plus lifecycle/expiry reuse, fission lineage + scar semantics, fusion trigger registration, observability
  (shadow + live, mid-tick staleness documented), replay v2 (full payloads), aggregate alert
  registration, delta log with OverlayDissolved + lineage entries, boundary phase timing
  attribution, indexed delta log emission, targeted boundary value-row uploads with full
  fallback after rebuild/tombstone cases
- `simthing-driver` — record/replay/bench CLI, all builtin stress scenarios,
  full benchmark metric reporting; `rebellion_demo` record/replay smoke verified

### Performance Baselines

| Scenario | Scale | ms/sim-day |
|---|---|---|
| `intent_stress` | 100k slots, 4 ticks/day | ~20 ms |
| `map_1m_light` | 1M slots, 8 ticks/day | ~25 ms |
| `fission_stress` | 20k→40k slots, mass fission | ~53 ms |

Static boundaries (no events, no pending work) skip entirely. Sparse dirty rows eliminated
non-GPU overhead from static map runs.

### Open Work

- **Topology retain/batch on fission growth (B2)** — `fission_stress` at ~53 ms/day;
  remaining cost is threshold readback, fission seeding, and full topology rebuild.
- **Full RON scenario expansion** — inline tree/registry in scenario files; currently all
  scenarios are hardcoded Rust builtins.
- **`simthing-studio` designer UI** — tabled.

---

## 19. Summary

```
One recursive type.         SimThing { properties, overlays, children }
One evaluation algorithm.   evaluate(ancestor_transforms) → FieldVector
One GPU pipeline.           intent → snapshot → velocity → intensity →
                            transform → reduce → threshold
One mechanism for change.   overlay TransformDelta referencing SubFieldRole by name
One source of truth.        GPU dense matrices + CPU semantic interpretation
One place to edit.          DimensionRegistry — SubFieldSpec governs everything

SubFieldSpec declares:
  role              what this block of floats means
  width             scalar (1) or vector (N)
  clamp             Bounded | Floored | Unbounded
  velocity_max      optional rate cap before integration
  default           starting value
  governed_by       which sub-field drives this one's integration
  reduction_override how this sub-field aggregates from children to parents

Player instructions are overlays. AI intent is overlays.
Policies are overlays. Technologies are overlays.
Ethics drift is an overlay on Named("axis_drift").
Disease is an overlay spawning Named("infection_rate") properties.
Rebellion is a cohort whose Amount is 0.03 and whose Intensity is 0.87.

Aggregate alerts watch the faction-level average, not the individual cohort.
Velocity alerts watch the trajectory before the value threshold is reached.
The intent delta buffer eliminates CPU round-trips for all tick-time transforms.
Static boundaries cost nothing.
Reduction is not post-processing — it is the presentation tier.

Everything else is a consequence.
```

The simulation is not a state machine that produces events. It is a continuously evolving
field that produces events when its values cross thresholds worth naming.

The player does not issue commands to a simulation. The player places overlays on a world.

The GPU does not accelerate a simulation. The GPU is the simulation.

The CPU does not run the world. The CPU understands it.

There is no rebel cohort. There is a cohort whose loyalty Amount is 0.03 and whose Intensity
is 0.87.

There is no AI early warning system. There is an `AggregateAlertRegistration` on the faction's
reduced loyalty `output_vectors` column, threshold 0.4, direction Downward.

The struct is the design. It was built.
