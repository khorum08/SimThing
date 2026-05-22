# SimThing: A GPU-Native Recursive World State Architecture
## Design Document v6 — Suspended Overlays, Capability Trees, Fission Inheritance

---

## Preface: From V5 to V6

Design v5 was the implementation-synchronized specification. V6 adds three
architectural decisions made during the capability tree workshop that are
substantive enough to make v5 incomplete as a reference:

1. **`OverlayLifecycle::Suspended`** — a new lifecycle state for overlays
   that exist in the CPU tree and are visible to observability and UI queries
   but are never uploaded to the GPU delta buffer. Pass 3 never applies them.
   Activated at boundary time via `BoundaryRequest::ActivateOverlay`. This is
   a general architecture addition: the mechanism for defining an effect
   upfront and activating it when conditions are met.

2. **`BoundaryRequest::ActivateOverlay`** — a new structural mutation that
   transitions a named overlay from `Suspended` to `Permanent` on any
   SimThing. Handled in step 9 of the day boundary, alongside all other
   structural mutations. Zero slot allocation, zero tree reshape.

3. **`FissionTemplate::clone_capability_children`** — a new field on
   `FissionTemplate` that controls whether capability subtrees
   (`Custom("tech_tree")`, `Custom("national_ideas")`, etc.) are deep-cloned
   from the parent into the fissioned child at spawn time. Default `false`.
   Used for faction-level fissions where the child faction should inherit the
   parent's capability state.

None of these change the GPU pipeline, the reduction tier, the threshold
system, or the evaluation algorithm. They extend the overlay lifecycle model
and the fission machinery.

### Addendum — Implementation status

The V6 simulation changes landed on `master` in commit `f39fe6d`.

Implemented:

- `OverlayLifecycle::Suspended { when_activated: Box<OverlayLifecycle> }`.
- CPU evaluator and GPU overlay-prep filtering for inactive/suspended overlays.
- `BoundaryRequest::ActivateOverlay` and `BoundaryRequest::SuspendOverlay`.
- Idempotent boundary handlers that unwrap or park overlay lifecycle state.
- Delta-log entries `OverlayActivated` and `OverlaySuspended`.
- Replay support for activation and suspension transitions.
- `OverlayContribution.active` for inspector/UI attribution.
- Empty-boundary skip logic that treats suspended overlays as inert.
- `FissionTemplate::clone_capability_children: bool` with serde default.
- Fission-time capability-subtree cloning for `Custom("tech_tree")`,
  `Custom("national_ideas")`, and `Custom("talent_tree")`.
- Fresh IDs and slots for cloned capability subtrees.
- Shadow-row copies for cloned capability nodes.
- Overlay `affects` remapping from the parent owner to the spawned owner.
- Pre-grow slot headroom for cloned capability subtrees before fission writes
  into the CPU shadow.

Verified:

- `cargo test` passes across the workspace after the implementation.
- Focused tests cover suspended overlay filtering, activation/suspension
  mutation, delta-log entries, replay transitions, observability attribution,
  empty-boundary skipping, fission capability cloning, overlay-affects remap,
  shadow-row copy, and fission headroom estimation.

Still open:

- A GPU boundary integration test proving an activated suspended overlay appears
  in the next Pass 3 upload and affects the next tick.
- An end-to-end replay test for a fissioned child that carries a cloned
  capability subtree.
- A future studio/registry decision for capability-container names beyond the
  three built-in `Custom(...)` identifiers.

---

## 1. The Central Statement

A grand strategy simulation can be expressed as a single recursive data
structure where every entity in the game — from the entire world down to a
single population cohort — is an instance of the same type. That type is
**SimThing**.

The world state lives in the GPU as dense vector matrices. It is continuously
evaluated there. The CPU gives meaning to what the GPU computes — interpreting
numbers as events, managing overlay lifecycles, and translating player and AI
intent into transform deltas.

```
GPU owns:     world state as dense matrices, continuous evaluation
Feeder owns:  translation between semantic intent and GPU-native operations
CPU owns:     meaning, lifecycle, events, player interface
```

Everything that acts on the world is an **overlay**. Every meaningful quantity
is a **property** whose sub-field layout is declared in the registry.
Everything that differentiates entities — including rebellion, revolution,
separatism, civil war, disease, diplomacy, and ethics — is expressed as
property values crossing thresholds, not as discrete flags or special entity
types.

```
One recursive type.                SimThing { properties, overlays, children }
One evaluation algorithm.          evaluate(ancestor_transforms) → FieldVector
One GPU pipeline.                  intent → velocity → transform → reduce → threshold
One mechanism for change.          overlay TransformDelta on appropriate SimThing
One mechanism for differentiation. intensity threshold in the registry
One source of truth.               GPU dense matrices + CPU semantic interpretation
One place to edit any property.    the DimensionRegistry
One reduction rule per sub-field.  SubFieldSpec::reduction_override or role default
One overlay lifecycle model.       Permanent | Transient | Suspended
```

Everything else is a consequence.

---

## 2. SimThing

Unchanged from v5. Every entity in the simulation is a SimThing.

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

The spatial tree expresses physical ownership. It is largely static.

```
World
  └── Star Systems
        └── Locations  (planets, stations, outposts)
              └── Cohorts  (population masses)
```

Factions, regions, alliances, and all political structures are overlays on
the spatial tree, not nodes within it. The tree is the physical map. The
political map is expressed through overlays on it.

---

## 3. Properties — The Complete Model

Unchanged from v5. See v5 §3 for full `SubFieldSpec`, `SubFieldRole`,
`ClampBehavior`, and `PropertyLayout` specifications.

---

## 4. The Dimension Registry

Unchanged from v5. See v5 §4.

---

## 5. Reduction — The Presentation Tier

Unchanged from v5. See v5 §5.

---

## 6. Overlays — The Universal Mechanism (UPDATED in V6)

An overlay is anything that modifies SimThing evaluation without becoming a
permanent part of its identity.

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

### OverlayLifecycle (UPDATED in V6)

```rust
enum OverlayLifecycle {
    Permanent,
    Transient {
        dissolution_conditions: Vec<DissolveCondition>,
    },
    Suspended {
        when_activated: Box<OverlayLifecycle>,   // NEW in V6
    },
}
```

`Suspended` is the third lifecycle state. A suspended overlay:

- **Exists** in the SimThing's `overlays` vec — CPU tree, CPU shadow, delta
  log, and observability queries all see it
- **Is not uploaded** to the GPU overlay delta buffer —
  `build_overlay_deltas` in `gpu_sync.rs` skips it entirely
- **Is never applied** by Pass 3 — the GPU pipeline has no knowledge of it
- **Does not prevent** the static boundary fast-path —
  `tree_has_boundary_lifecycle_work` does not count suspended overlays as
  lifecycle work requiring a boundary. Only the *active* lifecycle
  (`Permanent` or `Transient`) creates lifecycle work; the `Suspended`
  wrapper and its `when_activated` payload are both inert until activation
- **Does not trigger** fission — suspended overlays have no GPU-side
  expression

### is_active() helper (UPDATED in V6)

```rust
impl Overlay {
    // Replaces is_permanent() from v5
    pub fn is_active(&self) -> bool {
        matches!(self.lifecycle,
            OverlayLifecycle::Permanent | OverlayLifecycle::Transient { .. })
        // Suspended { .. } is explicitly not active regardless of when_activated
    }
}
```

### Activation

A suspended overlay is activated by `BoundaryRequest::ActivateOverlay`
(see §11). On activation `overlay.lifecycle` is set to `*when_activated`:

```rust
// In apply_structural_mutations, ActivateOverlay handler:
if let OverlayLifecycle::Suspended { when_activated } = overlay.lifecycle {
    overlay.lifecycle = *when_activated;
}
```

The declared `when_activated` lifecycle takes effect immediately. A tech
effect declared `Suspended { when_activated: Box::new(Permanent) }` becomes
permanent. A crisis response declared `Suspended { when_activated:
Box::new(Transient { dissolution_conditions: vec![AfterTicks { remaining: 30 }] }) }`
becomes transient and dissolves after 30 ticks. The intended lifecycle was
always part of the overlay definition — activation just exposes it.

From the next tick onward `build_overlay_deltas` includes the now-active
overlay in the GPU delta buffer and Pass 3 applies it.

### Use Cases

`Suspended` is a general mechanism for defining an effect upfront and
activating it when conditions are met:

| Use case | when_activated lifecycle | Activated when | Suspended when |
|---|---|---|---|
| Capability tree payload | `Permanent` | Research threshold crossed | Tech lost / obsoleted |
| National idea | `Permanent` | Player selects idea | Idea abandoned |
| Policy | `Permanent` or `Transient` | Player enacts policy | Policy suspended |
| Treaty effect | `Transient { PropertyBelow { .. } }` | Ratification boundary | Treaty suspended |
| Crisis response | `Transient { AfterTicks { .. } }` | Trigger condition fires | Crisis deescalated |
| Racial ability | `Permanent` | Threshold or event | Condition no longer met |

### Overlay Tombstoning

Unchanged from v5. When an overlay dissolves, its GPU transform slot
receives an identity operation. Dissolution recorded in delta log as
`BoundaryDeltaEntry::OverlayDissolved`.

### Overlay Table

Overlays unify every system that would otherwise require separate
architecture. V5 table plus new rows:

| What it represents | How it appears |
|---|---|
| Regional governance | Overlay on faction, affects location ids |
| Empire policy | Overlay on world or faction |
| Alliance treaty | Overlay on world, affects member faction ids |
| Governor | Overlay on location |
| Orbital station | Overlay on location |
| Plague | Transient overlay on location |
| Technology effect | Suspended overlay on capability node, activated on unlock |
| National idea effect | Suspended overlay on capability node, activated on selection |
| Ethics pressure | Overlay modifying Named("axis_drift") velocity |
| Player instruction | Transient overlay with urgency vector |
| AI intent | Transient overlay with urgency vector |
| Fleet movement | Transient: destination + urgency, dissolves on arrival |
| Construction order | Transient spawning permanent overlay on completion |
| Crisis | Transient world-level overlay |

---

## 7. Self-Managing Property Lifecycle

Unchanged from v5. See v5 §7.

---

## 8. Intensity-Driven Differentiation and Fission (UPDATED in V6)

Same model as v5 §8 for cohort-level fission. Two additions for
faction-level fission.

### FissionTemplate (UPDATED in V6)

```rust
struct FissionTemplate {
    child_kind:                 SimThingKindTag,
    fusion_intensity_threshold: f32,
    fusion_scar_coefficient:    f32,
    resolution_label:           String,
    clone_capability_children:  bool,   // NEW in V6 — default false
}
```

When `clone_capability_children: true`, `execute_fission` after spawning
the child SimThing also deep-clones all capability container children from
the parent into the child. Capability containers are identified by kind:
`Custom("tech_tree")`, `Custom("national_ideas")`, `Custom("talent_tree")`,
and any other `Custom` kind the studio layer registers as a capability
container.

**Default is `false`.** Cohort-level fission, location fission, and any
fission that should not inherit capability state uses the default and is
entirely unaffected by this addition.

### Capability Child Clone

For each cloned capability container:
- Deep-clone the `SimThing` struct with a **fresh `SimThingId`** — the
  child faction owns an independent tree that can diverge going forward
- Allocate one new slot in `SlotAllocator`
- Copy the source node's shadow row into the new slot's row — one
  `memcpy` of `n_dims × 4B`
- Clone all overlays from the source node
- Run `remap_overlay_affects(cloned_node, parent_faction_id, child_faction_id)`
  to replace the parent faction's id with the child faction's id in every
  cloned overlay's `affects` field

### remap_overlay_affects

```rust
fn remap_overlay_affects(
    node:   &mut SimThing,
    old_id: SimThingId,
    new_id: SimThingId,
) {
    for overlay in &mut node.overlays {
        for affects in &mut overlay.affects {
            if *affects == old_id { *affects = new_id; }
        }
    }
    for child in &mut node.children {
        remap_overlay_affects(child, old_id, new_id);
    }
}
```

Called once per cloned capability subtree after cloning, before attaching
to the child faction. Small, self-contained, independently testable.

### Pre-grow Headroom (UPDATED in V6)

The pre-grow at boundary step 4 previously allocated `events.len()` slots
of headroom — one per fission event. A faction fission with
`clone_capability_children: true` needs additional slots: one per
capability container child being cloned.

The pre-grow now reads `clone_capability_children` from each triggered
`FissionTemplate` and adds the parent faction's capability child count to
the headroom estimate for that event.

### Delta Log and Replay

`FissionOccurred { parent, node: SimThing }` carries the full spawned
faction subtree including all cloned capability children. Replay
reconstructs correctly for free — no additional delta log changes required.

### Fission Lineage

Unchanged from v5. Lineage records are emitted for the spawned faction
node only, not for the cloned capability children (they are not fission
products in the semantic sense).

---

## 9. Evaluation — One Pass, Both Directions

Unchanged from v5. See v5 §9.

---

## 10. The GPU Pipeline

Unchanged from v5. See v5 §10. Suspended overlays are invisible to all
GPU passes — `build_overlay_deltas` filters them before upload, so the
shader never sees them.

---

## 11. The Day Boundary (UPDATED in V6)

The 13-step boundary sequence from v5 is unchanged. Step 9 now handles
one additional `BoundaryRequest` variant.

### BoundaryRequest::ActivateOverlay (NEW in V6)

```rust
pub enum BoundaryRequest {
    AddChild        { parent: SimThingId, child: SimThing },
    Remove          { target: SimThingId },
    Reparent        { child: SimThingId, new_parent: SimThingId },
    AttachOverlay   { target: SimThingId, overlay: Overlay },
    AddDimension    { property: SimPropertyId },
    ActivateOverlay { target: SimThingId, overlay_id: OverlayId },  // NEW in V6
    SuspendOverlay  { target: SimThingId, overlay_id: OverlayId },  // NEW in V6
}
```

`ActivateOverlay` is handled in `apply_structural_mutations` at step 9:

1. Find the target SimThing by id via the boundary tree index
2. Find the overlay by `overlay_id` in its `overlays` vec
3. Unwrap `Suspended { when_activated }` — set `overlay.lifecycle = *when_activated`
4. Mark `topology_dirty = true` so `build_overlay_deltas` re-uploads the
   now-active overlay on the next `sync_gpu_buffers` call

If the overlay is not currently `Suspended` (e.g. already `Permanent`),
`ActivateOverlay` is a no-op — idempotent, no error.

No slot allocation. No tree reshape. The overlay was already in the tree
— only its lifecycle field changes. The GPU sees the effect from the next
tick's Pass 3 onward.

### BoundaryRequest::SuspendOverlay (NEW in V6)

The symmetric counterpart to `ActivateOverlay`. Transitions an active
overlay back to `Suspended`, removing it from the GPU delta buffer while
preserving its full definition for later reactivation.

`SuspendOverlay` is handled in `apply_structural_mutations` at step 9:

1. Find the target SimThing by id via the boundary tree index
2. Find the overlay by `overlay_id` in its `overlays` vec
3. Wrap the current lifecycle: `overlay.lifecycle = Suspended { when_activated: Box::new(current_lifecycle) }`
4. Mark `topology_dirty = true` so `build_overlay_deltas` omits it from
   the next GPU upload

```rust
// In apply_structural_mutations, SuspendOverlay handler:
match overlay.lifecycle.clone() {
    OverlayLifecycle::Suspended { .. } => { /* already suspended — no-op */ }
    active_lifecycle => {
        overlay.lifecycle = OverlayLifecycle::Suspended {
            when_activated: Box::new(active_lifecycle),
        };
    }
}
```

**Dissolution conditions are frozen while suspended.** A `Transient`
overlay with `AfterTicks { remaining: 5 }` that is suspended at 3 ticks
remaining will still have 3 ticks remaining when reactivated — the
countdown does not run while the overlay is not active. This is correct:
a paused effect should not be dissolving while paused.

**Idempotent.** If the overlay is already `Suspended`, `SuspendOverlay`
is a no-op.

**Round-trip.** `SuspendOverlay` followed by `ActivateOverlay` restores
the overlay to its exact prior state. The `when_activated` payload
carries everything needed.

### Static Boundary Fast-Path (unchanged from v5)

`can_skip_empty_boundary` is unaffected. Suspended overlays do not count
as transient lifecycle work. A tree full of suspended overlays with no
active events still skips the boundary entirely.

---

## 12. The Feeder Thread Architecture

Unchanged from v5 except that `BoundaryRequest::ActivateOverlay` is now
a valid work item. The Transform Patcher parks it for boundary time
alongside all other `BoundaryRequest` variants. No mid-day fast path
exists for overlay activation — it is always a boundary operation.

---

## 13. Threshold Detection

Unchanged from v5. See v5 §13.

---

## 14. Observability — Two Fidelity Levels (UPDATED in V6)

Both `observe()` and `observe_live()` are unchanged in signature and
semantics. The `ObservabilityReport` now distinguishes suspended overlays
from active overlays in `OverlayContribution`:

```rust
struct OverlayContribution {
    overlay_id: OverlayId,
    source:     OverlaySource,
    deltas:     Vec<(SubFieldRole, TransformOp)>,
    inherited:  bool,    // true = lives on an ancestor, not this node
    active:     bool,    // NEW in V6 — false = Suspended, not applied by GPU
}
```

`active: false` means the overlay is defined and visible but not currently
affecting GPU evaluation. UI systems use this to display "available but
not yet unlocked" effects vs "currently active" effects without separate
data structures.

---

## 15. The Delta Log and Replay (UPDATED in V6)

### Delta Log

One new entry variant:

```rust
enum BoundaryDeltaEntry {
    // ... all v5 variants unchanged ...
    OverlayActivated  { target: SimThingId, overlay_id: OverlayId },  // NEW in V6
    OverlaySuspended  { target: SimThingId, overlay_id: OverlayId },  // NEW in V6
}
```

`OverlayActivated` and `OverlaySuspended` complete the lifecycle audit
trail: `OverlayAttached` records the initial attach (with lifecycle
state), `OverlayActivated` records transition from Suspended to active,
`OverlaySuspended` records transition from active to Suspended,
`OverlayDissolved` records dissolution.

`ReplayDriver::apply_entry` handles both variants by finding the overlay
on the target node and applying the same lifecycle transition as live
boundary execution. Round-trips correctly — suspend followed by activate
restores exact prior state.

### Replay

`ReplaySnapshot` is unchanged. Suspended overlays are part of the
serialized `SimThing` tree — they round-trip through LDJSON as ordinary
overlay structs with `lifecycle: Suspended`. No additional snapshot
fields required.

---

## 16. Downstream Systems

### Presentation

Unchanged from v5. Reads from `output_vectors`.

### Network Play

`ActivateOverlay` and `SuspendOverlay` boundary requests serialize
cleanly — only `(SimThingId, OverlayId)` crosses the wire, same as any
other structural mutation. No column indices, no GPU state transmitted.

### AI

AI can register `AggregateAlertRegistration` on capability tree
`output_vectors` columns (e.g. faction-level research progress) to detect
when unlock thresholds are approaching. Velocity alerts on the same
columns give trajectory warnings. Neither requires knowledge that the
columns belong to a capability tree — from the AI's perspective they are
float columns like any other.

### Observability

`active: bool` on `OverlayContribution` allows the inspector UI to
distinguish "what this entity would gain if this overlay activated" from
"what is currently affecting this entity." Both read from the same
`ObservabilityReport` with no additional queries.

### Studio Layer (simthing-studio)

The capability tree pattern — one SimThing child per owning node,
properties tracking progress, suspended overlays as payloads — is
implemented entirely in `simthing-studio`. The simulation crates have no
concept of tech trees, national ideas, talent trees, or any specific
progression system. See `docs/workshop/capability_tree_decisions.md` for
the full capability tree design.

---

## 17. Component Discipline: Auditable, Testable, Reversible

**Auditable:** `OverlayContribution::active` makes suspended-vs-active
state readable from any observability query. The lifecycle of every
overlay — from `Suspended` at attach time, through `OverlayActivated` at
activation, to `OverlayDissolved` at dissolution — is fully recorded in
the delta log.

**Testable:** `Suspended` overlays must be verified absent from GPU delta
buffers in `gpu_sync` tests. `ActivateOverlay` must be verified to
produce a delta buffer entry on the following tick. `clone_capability_children`
must be verified to produce correct slot counts, shadow row copies, and
overlay affects remapping. New proof tests:
- `suspended_overlay_absent_from_gpu_delta_buffer`
- `activate_overlay_appears_in_pass3_next_tick`
- `suspend_active_overlay_absent_from_gpu_delta_buffer_next_tick`
- `suspend_then_activate_restores_exact_prior_lifecycle`
- `suspend_transient_overlay_freezes_dissolution_countdown`
- `fission_with_clone_capability_children_remaps_affects`

**Reversible:** Lifecycle state is a field on `Overlay`. Changing it is a
registry-equivalent operation — one field, one boundary request. The delta
log records every transition. Version control tracks the `FissionTemplate`
`clone_capability_children` flag like any other template field.

---

## 18. Implementation State

**188/188 tests passing on v5 master. V6 additions are not yet
implemented.**

### V6 Work Required

#### Design changes (require thought before coding)

| Location | Change |
|---|---|
| `simthing-core/src/overlay.rs` | Add `Suspended` to `OverlayLifecycle`; replace `is_permanent()` with `is_active()` |
| `simthing-feeder/src/work.rs` | Add `ActivateOverlay { target, overlay_id }` and `SuspendOverlay { target, overlay_id }` to `BoundaryRequest` |
| `simthing-core/src/property.rs` | Add `clone_capability_children: bool` to `FissionTemplate` |
| `simthing-sim/src/fission.rs` | Add `clone_capability_children` logic and `remap_overlay_affects` function; update pre-grow headroom |

#### Mechanical completions (compiler-driven exhaustive match)

| Location | Change |
|---|---|
| `simthing-sim/src/gpu_sync.rs` | Skip `Suspended` in `build_overlay_deltas` |
| `simthing-sim/src/overlay_lifecycle.rs` | Skip `Suspended` in dissolution logic |
| `simthing-sim/src/delta_log.rs` | Add `OverlayActivated` variant; record lifecycle in `OverlayAttached` |
| `simthing-sim/src/observability.rs` | Add `active: bool` to `OverlayContribution` |
| `simthing-sim/src/tree_mutation.rs` | Add `ActivateOverlay` and `SuspendOverlay` handlers |
| `simthing-sim/src/boundary.rs` | Route `ActivateOverlay` and `SuspendOverlay` through step 9; `tree_has_boundary_lifecycle_work` must not treat `Suspended` as lifecycle work |

#### What does not change

- All GPU passes (0–7) and all WGSL shaders — no changes
- `simthing-gpu` — no changes
- `DimensionRegistry` — no changes
- Reduction tier — no changes
- Threshold detection — no changes
- `simthing-feeder` beyond `work.rs` — no changes

### V5 Performance Baselines (unchanged)

| Scenario | Scale | ms/sim-day |
|---|---|---|
| `intent_stress` | 100k slots, 4 ticks/day | ~20 ms |
| `map_1m_light` | 1M slots, 8 ticks/day | ~25 ms |
| `fission_stress` | 20k→40k slots, mass fission | ~53 ms |

### Open Work (carries forward from v5)

- **Topology retain/batch on fission growth (B2)**
- **Full RON scenario expansion**
- **`simthing-studio` capability tree implementation** — `CapabilityTreeSpec`,
  `CapabilityTreeBuilder`, `CapabilityTreeDefinition`, RON loader. Depends
  on V6 simulation changes landing first.
- **`simthing-studio` designer UI** — tabled.

---

## 19. Summary

```
One recursive type.         SimThing { properties, overlays, children }
One evaluation algorithm.   evaluate(ancestor_transforms) → FieldVector
One GPU pipeline.           intent → snapshot → velocity → intensity →
                            transform → reduce → threshold
One mechanism for change.   overlay TransformDelta referencing SubFieldRole by name
One overlay lifecycle.      Permanent | Transient | Suspended
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
An unlocked tech is a Suspended { when_activated: Permanent } overlay whose lifecycle became Permanent.

Aggregate alerts watch the faction-level average, not the individual cohort.
Velocity alerts watch the trajectory before the value threshold is reached.
The intent delta buffer eliminates CPU round-trips for all tick-time transforms.
Static boundaries cost nothing. Suspended overlays cost nothing.
Reduction is not post-processing — it is the presentation tier.

Everything else is a consequence.
```

The simulation is not a state machine that produces events. It is a
continuously evolving field that produces events when its values cross
thresholds worth naming.

The player does not issue commands to a simulation. The player places
overlays on a world.

The GPU does not accelerate a simulation. The GPU is the simulation.

The CPU does not run the world. The CPU understands it.

There is no rebel cohort. There is a cohort whose loyalty Amount is 0.03
and whose Intensity is 0.87.

There is no tech tree. There is a SimThing whose properties track research
progress as floats and whose suspended overlays become permanent when those
floats cross thresholds.

The struct is the design. Build it.
