# Capability Tree Studio Layer — Workshop
## Session: 2026-05-22

> **Superseded for implementation.** Read [`docs/design_v6.5.md`](../design_v6.5.md),
> [`simthing_spec_progress_log.md`](simthing_spec_progress_log.md), and
> [`docs/adr/`](../adr/README.md). Retained for workshop Q&A rationale only.

This file records approved design decisions for the `simthing-studio`
capability tree builder. Decisions are locked once marked **APPROVED**.
Open questions are marked **OPEN** or **DISCUSS**.

Reference documents:
- `docs/capability_tree_v1.md` — RON format and worked examples
- `docs/design_v6.md` — V6 architecture spec
- `docs/invariants.md` — non-negotiable code rules
- `docs/workshop/tech_tree_decisions.md` — prior session decisions (all DECIDED)

**Simulation crates are frozen for this work.** All prior workshop decisions
(§1–9 of `tech_tree_decisions.md`) are DECIDED and already landed. This
workshop designs only the `simthing-studio` crate.

---

## Context: What Is Already Built

The simulation side is complete:

- `OverlayLifecycle::Suspended { when_activated }` — landed (`f39fe6d`)
- `BoundaryRequest::ActivateOverlay` / `SuspendOverlay` — landed
- `FissionTemplate::clone_capability_children` + `capability_container_kinds` — landed (PR #38)
- 202 tests passing, zero warnings

What does NOT exist yet:

- The `simthing-studio` crate (possibly a scaffold exists — TBD)
- `CapabilityTreeSpec` / `CapabilityCategorySpec` / `CapabilitySpec` Rust structs
- `CapabilityTreeBuilder` — session-init wiring
- `CapabilityTreeDefinition` — runtime lookup table
- Boundary handler — prereq checking + `ActivateOverlay` issuance
- Impact preview — CPU-side effect calculation

---

## Workshop Agenda

1. Crate structure and module layout
2. Data model: spec structs and RON deserialization
3. `CapabilityTreeBuilder` — session init responsibilities
4. `CapabilityTreeDefinition` — runtime lookup table shape
5. Boundary handler — event routing, prereq checking, activation
6. Impact preview — CPU query API
7. Mutual exclusivity (national ideas) — mechanism
8. Error handling and validation strategy
9. Test plan

---

## 1. Crate Structure

### Questions

**Q1.1** Does a `simthing-studio` crate already exist (even as a scaffold)?
- [ ] DISCUSS — need to check `crates/` directory

**Q1.2** What is the minimal crate dependency graph for studio?
- Prior decision: studio depends on `simthing-core` and `simthing-feeder`
  (for `BoundaryRequest`). Does NOT depend on `simthing-sim` directly.
- [ ] CONFIRM or ADJUST

**Q1.3** Module layout inside `simthing-studio`:
```
simthing-studio/
  src/
    lib.rs
    spec.rs          — CapabilityTreeSpec + RON deserialization structs
    builder.rs       — CapabilityTreeBuilder (session init)
    definition.rs    — CapabilityTreeDefinition (runtime lookup table)
    boundary.rs      — boundary handler: threshold events → BoundaryRequest
    preview.rs       — impact preview (CPU-side effect calculation)
    error.rs         — CapabilityTreeError enum
```
- [ ] APPROVE or ADJUST

---

## 2. Data Model

### Already decided (from `tech_tree_decisions.md` §5a, §8)

```rust
// Authoring structs (RON-facing)
struct CapabilityTreeSpec { tree_id, tree_kind, owner_kind, categories }
struct CapabilityCategorySpec { property_namespace, property_name, display_name,
                                tier?, max_active?, entries }
struct CapabilitySpec { id, display_name, description, flavor_text?,
                        research_cost, icon, thumbnail, card_image,
                        unlock_video?, model_preview?,
                        prereqs, unlocks_ship_components, unlocks_buildings,
                        unlocks_units, unlocks_weapons, effects }
struct CapabilityEffectSpec { targets_property, sub_field_deltas, when_activated }
struct CapabilityPrereqSpec { category, entry_id }

// Runtime struct
struct CapabilityDefinition { id, display_name, description, flavor_text,
                               icon, thumbnail, card_image, unlock_video?,
                               model_preview?,
                               unlocks_ship_components, unlocks_buildings,
                               unlocks_units,
                               overlay_ids: Vec<OverlayId>,  // one per effect
                               prereqs: Vec<CapabilityPrereq>,
                               // research_cost NOT stored }
```

### Open questions

**Q2.1** `CapabilityEffectSpec::when_activated` — the RON format in
`capability_tree_v1.md` shows `when_activated: Permanent` or
`when_activated: Transient(...)` per effect. Should this map 1:1 to
`OverlayLifecycle::Suspended { when_activated: Box<OverlayLifecycle> }`
at session init? That is: every effect starts `Suspended` wrapping the
specified `when_activated` lifecycle.
- [ ] APPROVE (this seems right — the spec already decided this, just confirming the mapping)

**Q2.2** Multiple effects per `CapabilitySpec` (§3f in `capability_tree_v1.md`).
Each effect becomes one overlay on the capability tree node. The `CapabilityDefinition`
holds `overlay_ids: Vec<OverlayId>`. All are activated together. Any concern?
- [ ] APPROVE

**Q2.3** `CapabilityPrereq` runtime struct — at session init prereq specs
(`category`, `entry_id`) must be resolved to column indices so the boundary
handler can do fast array reads rather than name lookups. Shape:
```rust
struct CapabilityPrereq {
    col: usize,      // col_for_role(Named("entry_id"), layout) on capability tree slot
    min_value: f32,  // research_cost of the prereq entry (progress must reach this)
}
```
- [ ] APPROVE or ADJUST

**Q2.4** Cross-category prereqs (`propulsion::ion_drive` as prereq of
`physics::gravitic_theory`) — these reference a different `SimPropertyId`
on the same capability tree slot. The runtime prereq needs both `col` and
which property's column range to resolve against.
```rust
enum CapabilityPrereq {
    SameProperty { col: usize, min_value: f32 },
    CrossProperty { property_id: SimPropertyId, col: usize, min_value: f32 },
}
```
- [ ] APPROVE or ADJUST

### Forward-compatibility note: `_rate` sub-field spec (no decision required now)

The `_rate` sub-field on each capability entry is currently a static `f32`
set by an overlay when the player allocates research. This is the right
choice for this session.

However, the EML integration guidance (`docs/eml_integration_guidance.md`,
Phase 1–4) anticipates that research rate formulas will eventually be
expressible as pure numeric expressions over resolved property reads —
e.g. `base_research_output * 1.2 + specialist_count * 0.05`. Those are
valid `ScriptExpr` / EML targets once that infrastructure exists.

**Constraint for the builder's data model:** do not represent the rate as
a bare `f32` field directly on `CapabilityCategorySpec` or `CapabilitySpec`.
Wrap it in a `ResearchRateSpec` enum from the start:

```rust
enum ResearchRateSpec {
    Literal(f32),
    // Script(ScriptExpr),  // Phase 1 — not yet
}
```

This costs nothing now, and means when `ScriptExpr` exists the RON format
and builder signature do not need a breaking change — the `Script` arm slots
in. A bare `f32` in the RON would require a migration.

**This is a build-time authoring seam, not an EML decision.** EML is Phase
0 (documentation only). `ScriptExpr` is Phase 1 (not yet built). No EML
work is required or implied for this session.

---

## 3. CapabilityTreeBuilder — Session Init

### Prior decisions (§5c, §8b)

At session init the builder:
1. Reads `CapabilityTreeSpec` from RON
2. Registers `SimProperty` definitions (one per category)
3. Constructs the TechTree `SimThing` with all properties at default and
   all suspended overlays attached
4. Builds `CapabilityTreeDefinition` (runtime lookup table)
5. Registers Pass 7 thresholds (one per entry, threshold = `research_cost`)
6. Returns `(SimThing, CapabilityTreeDefinition)` to the session coordinator

### Open questions

**Q3.1** Builder signature. Proposed:
```rust
impl CapabilityTreeBuilder {
    pub fn build(
        spec: &CapabilityTreeSpec,
        registry: &mut DimensionRegistry,
        id_source: &mut dyn SimThingIdSource,
    ) -> Result<(SimThing, CapabilityTreeDefinition), CapabilityTreeError>
}
```
- [ ] APPROVE or ADJUST

**Q3.2** Overlay ID generation. Each effect gets one `OverlayId`. Where
does the ID come from?
- ~~Option A: Derived from content hash~~ — collides with existing counter-issued IDs
- ~~Option C: Shared ID space with `SimThingIdSource`~~ — conflates distinct identity spaces
- **APPROVED — call `OverlayId::new()` per effect.** `OverlayId` is already a
  global static atomic counter in `simthing-core/src/ids.rs`, identical pattern
  to `SimThingId::new()`. No new mechanism needed. Replay correctness follows the
  same contract as `SimThingId` — builder called in same order with same spec
  produces same sequence.

**Q3.3** `research_cost: 0.0` for national ideas (player selection, not
research). At session init with `research_cost: 0.0`, the entry should
produce NO threshold registration (a threshold at 0.0 would fire
immediately). The builder must special-case this. How?
- ~~Option A: Skip threshold registration if `research_cost == 0.0`~~
- **APPROVED — Option B:** Explicit `ActivationMode` enum on `CapabilitySpec`.
  Builder reads it, registers a threshold or not accordingly. Boundary handler
  exposes both `handle_threshold_events` and `handle_player_selection`.
  ```rust
  enum ActivationMode {
      Threshold,        // research_cost > 0 — Pass 7 threshold registered, fires automatically
      PlayerSelection,  // no threshold registered — fired by explicit UI call
  }
  ```
  **Note:** `ActivationMode` should be designed as a growable enum from the
  start. Future arms like `ExternalTrigger`, `TimedUnlock`, or `ScriptedCondition`
  are plausible as the studio layer matures. Non-exhaustive (`#[non_exhaustive]`)
  or match-with-fallback patterns should be used at call sites to avoid
  churn when new arms are added.

**Q3.4** `ReductionRule::Max` is mandatory for all capability sub-fields
(§3c of prior decisions). The builder should enforce this — it must
override or assert whatever the caller-supplied spec says. Agreed?
- **APPROVED.** Builder enforces `ReductionRule::Max` on all capability
  sub-fields unconditionally. `Mean` would dilute tech columns to near zero
  because spatial sibling nodes carry no tech columns. This is not
  caller-configurable.

**Q3.5** Threshold event semantics. Prior open question Q3 from
`tech_tree_decisions.md`: "ThresholdSemantic::UnlockTrigger vs reuse of
existing semantic."
- **APPROVED — add `ThresholdSemantic::CapabilityUnlock` to `threshold_registry.rs`.**
  `ThresholdSemantic` already has named variants (`FissionTrigger`, `FusionTrigger`,
  `PropertyExpiry`, `VelocityAlert`, `AggregateAlert`) — each tells the boundary
  handler exactly what to do with a fired event. Capability unlocks follow the
  same pattern:
  ```rust
  CapabilityUnlock {
      sim_thing_id: SimThingId,   // the capability tree node
      property_id:  SimPropertyId,
      sub_field:    SubFieldRole, // Named("entry_id") progress col
  }
  ```
  The builder registers this via `ThresholdBuilder` (or a parallel studio-owned
  registration path — see Q4). The boundary handler matches on `CapabilityUnlock`
  and routes to the studio layer's capability handler. No filtering work at the
  call site. This is the one point where `simthing-studio` requires a thin hook
  into `simthing-sim/src/threshold_registry.rs`.

---

## 4. CapabilityTreeDefinition — Runtime Lookup Table

### Prior decisions (§8c)

```rust
struct CapabilityTreeDefinition {
    tree_id:       String,
    tree_thing_id: SimThingId,
    // Keyed by threshold event for fast boundary-handler lookup:
    by_threshold:  HashMap<(SimPropertyId, SubFieldRole), CapabilityDefinition>,
    // Keyed by overlay_id for UI queries:
    by_overlay:    HashMap<OverlayId, CapabilityDefinitionRef>,
}
```

### Crate naming decision

The RON-to-runtime compiler does not belong in `simthing-core` (which must
stay a stable foundation of pure data types) and is not a designer UI layer.
It is a spec interpretation layer — reads authored data formats, validates
them, and produces session-ready runtime objects.

**APPROVED — new crate: `simthing-spec`.**
Dependencies: `simthing-core`, `simthing-feeder`. No dependency on
`simthing-sim` or `simthing-gpu`. The eventual `simthing-studio` designer UI
will depend on `simthing-spec`, not the other way around.

### Open questions

**Q4.1** The boundary handler receives a threshold event as `(property_id,
sub_field_role, slot)`. It needs to look up the `CapabilityDefinition`.
The slot is the TechTree slot — is it sufficient to key on
`(SimPropertyId, SubFieldRole)` without the slot? Or do multiple factions
each have their own definition (no — they share the same spec, different slots)?
- **APPROVED — one `CapabilityTreeDefinition` per spec, shared across all
  faction instances.** Slot is used only to read shadow values at boundary
  time. Per-faction slot mapping lives in the session coordinator (see Q4.2).

**Q4.2** Should `CapabilityTreeDefinition` own a reference to the
capability tree slot per faction instance, or should the session
coordinator maintain the per-faction mapping?
- **APPROVED — Option B: session coordinator owns it.**
  `CapabilityTreeDefinition` is pure read-only data after session init.
  The session coordinator holds:
  ```rust
  HashMap<SimThingId, (CapabilityTreeDefinitionId, Slot)>
  // faction_id → (which definition, that faction's tree slot)
  ```
  Updated when fissions spawn new capability trees. Definition never mutates.

**Q4.3** `CapabilityUnlock` threshold registrations must reach
`ThresholdBuilder`. The registration type needs to live where both
`simthing-spec` and `simthing-sim` can see it without a circular dependency.
- **APPROVED — Option C: `CapabilityUnlockRegistration` defined in
  `simthing-feeder`**, parallel to `VelocityAlertRegistration` and
  `AggregateAlertRegistration`. `ThresholdBuilder` grows a
  `build_with_capability_unlocks` entry point. `simthing-spec` constructs
  the registrations and passes them to the session coordinator, which
  forwards them to `ThresholdBuilder` at boundary build time.
  Crate dependency graph remains acyclic:
  ```
  simthing-core
    ↑
  simthing-feeder   (CapabilityUnlockRegistration lives here)
    ↑         ↑
  simthing-sim   simthing-spec
  ```

---

## 5. Boundary Handler

### Prior decisions (§5d)

```
BoundaryOutcome threshold event fires
  → look up CapabilityDefinition by (property_id, sub_field_role)
  → read prereq col values from shadow at TechTree slot
  → if all prereqs met: issue BoundaryRequest::ActivateOverlay for each overlay_id
  → if prereqs NOT met: issue intent delta resetting progress sub-field to prereq_threshold
```

### Open questions

**Q5.1** Boundary handler signature. The handler is called per-faction
per threshold event. Proposed:
```rust
impl CapabilityTreeBoundaryHandler {
    pub fn handle_threshold_events(
        &self,
        events: &[ThresholdEvent],
        shadow: &WorldShadow,
        faction_tree_slots: &HashMap<SimThingId, Slot>,
        requests: &mut Vec<BoundaryRequest>,
    )
}
```
- [ ] APPROVE or ADJUST

**Q5.2** Progress reset on failed prereq.
- ~~Option A: Reset to 0.0~~ — ruled out on gameplay grounds
- ~~Option C: Do nothing~~ — threshold never re-fires, entry silently stuck
- **APPROVED — Option B + `ActivationMode` state transition.**

  When a `Threshold` entry fires with unmet prereqs:
  1. Reset progress to `research_cost - EPSILON` (holds just below threshold,
     preserves all prior research, threshold does not re-fire)
  2. Transition the entry's per-faction `ActivationMode` from `Threshold`
     to `OnPrereqMet`

  `OnPrereqMet` is a new `ActivationMode` arm:
  ```rust
  #[non_exhaustive]
  enum ActivationMode {
      Threshold,        // Pass 7 threshold registered — fires when progress crosses research_cost
      PlayerSelection,  // no threshold — fired by explicit UI call
      OnPrereqMet,      // no threshold — swept by handler after any activation in same tree,
                        // and once at session init
  }
  ```

  **`OnPrereqMet` is entirely CPU-side. No Pass 7 threshold registration.
  No GPU buffer entry. No GPU involvement until activation.**

  The sweep that checks `OnPrereqMet` entries runs:
  - After every `CapabilityUnlock` activation in the same faction's tree
    (post-activation step in the boundary handler)
  - Once at session init (in case scenario seeds prereqs already satisfied)

  Sweep logic: walk all `OnPrereqMet` entries for the faction, read prereq
  shadow columns via `col_for_role`, issue `ActivateOverlay` for any entry
  where all prereqs pass. Pure CPU shadow reads — no tree traversal beyond
  the capability tree node itself.

  **Per-faction mutable state note:** `ActivationMode` per entry must be
  per-faction mutable state, not part of the shared read-only
  `CapabilityTreeDefinition`. Lives in a `CapabilityTreeState` struct owned
  by the session coordinator alongside the slot mapping (extends Q4.2).
  Must be recorded in the delta log and replayable.

**Q5.3** Multiple factions may fire the same threshold event in the same
boundary step. The handler must process them independently (different
slots, different shadow reads). Any concern with the batch API?
- [ ] CONFIRM no concern

**Q5.4** Mutual exclusivity (national ideas, `max_active: 1`). When
`ActivateOverlay` is issued for a new idea, the handler must also issue
`SuspendOverlay` for currently-active siblings in the same category.
The handler needs to know which overlays in the same category are
currently active (i.e., `OverlayLifecycle::Permanent`).
Two sub-questions:
- 5.4a: How does the handler discover currently-active sibling overlays?
  Does it walk the TechTree SimThing's overlay list? Or is there a
  per-category active-overlay index maintained in `CapabilityTreeDefinition`?
- 5.4b: Is mutual exclusivity a `CapabilityCategorySpec`-level flag
  (`max_active: 1`) enforced here, or a separate mechanism?
- [ ] DECIDE

---

## 6. Impact Preview

### Prior decisions (§5e)

CPU-side calculation: apply the suspended overlay's `PropertyTransformDelta`
against current `output_vectors` shadow values for the relevant columns.
No GPU operation. Available at any time for UI display.

### Open questions

**Q6.1** Preview API:
```rust
impl CapabilityTreeDefinition {
    pub fn preview_effect(
        &self,
        entry_id: &str,
        shadow: &WorldShadow,
        faction_slot: Slot,
        registry: &DimensionRegistry,
    ) -> Vec<(SimPropertyId, SubFieldRole, f32 /* current */, f32 /* with effect */)>
}
```
- [ ] APPROVE or ADJUST

**Q6.2** Preview for multi-effect entries (multiple overlays): should the
preview return combined effect or per-overlay breakdown?
- [ ] DECIDE

---

## 7. Mutual Exclusivity (National Ideas Detail)

See Q5.4 above. Additional questions:

**Q7.1** Can a `CapabilityCategorySpec` have `max_active: N` where N > 1
(e.g. "pick any 2 from this tier")? Or is it always 0 (no limit) or 1
(exclusive)?
- [ ] DECIDE

**Q7.2** When a player switches ideas (activates B, which suspends A), do
A's effects suspend atomically with B's activation? Yes — both
`SuspendOverlay` and `ActivateOverlay` go through boundary step 9 in the
same boundary cycle, so the transition is one-tick atomic.
- [ ] CONFIRM

---

## 8. Error Handling and Validation

**Q8.1** What errors should `CapabilityTreeBuilder::build` surface?
Candidates:
- Duplicate entry IDs within a category
- Prereq reference to unknown category or entry ID
- `research_cost < 0.0`
- Effect references property that isn't registered in `DimensionRegistry`
- `max_active` value out of range
- [ ] APPROVE list or add/remove items

**Q8.2** Are errors hard failures (builder returns `Err`) or soft
warnings (builder continues with a degraded result)?
- [ ] DECIDE

**Q8.3** `debug_assert!` vs runtime validation: should invariants like
"all capability sub-fields use `ReductionRule::Max`" be debug-only
assertions or always-on validation?
- [ ] DECIDE

---

## 9. Test Plan

**Q9.1** Proposed tests for the initial PR:

Integration tests (in `simthing-studio/tests/`):
- `capability_tree_builder_registers_properties_and_overlays` — build
  from a minimal spec; assert `DimensionRegistry` has the expected
  sub-fields; assert TechTree SimThing has the expected suspended overlays.
- `capability_tree_boundary_handler_activates_on_threshold` — seed a
  TechTree with progress at threshold, no prereqs; boundary handler
  issues `ActivateOverlay`.
- `capability_tree_prereq_blocks_activation_and_resets_progress` — seed
  progress at threshold but prereq not met; boundary handler issues
  progress-reset intent delta, not `ActivateOverlay`.
- `capability_tree_cross_category_prereq_resolves` — cross-category prereq
  check resolves correctly.
- `capability_tree_impact_preview_returns_delta` — preview returns correct
  before/after values for a multiply effect.
- `national_ideas_mutual_exclusivity_suspends_sibling` — activate idea B
  while idea A is active; handler issues `SuspendOverlay` for A and
  `ActivateOverlay` for B.

Unit tests (in `simthing-studio/src/`):
- `prereq_resolution_same_category` — verify prereq struct gets correct `col`
- `prereq_resolution_cross_category` — verify cross-property prereq struct
- `overlay_id_generation_is_deterministic` — same spec always produces same IDs

- [ ] APPROVE list or add/remove tests

---

## Approved Decisions Log

*(All decisions closed. Unified into master handoff document.)*

| # | Decision | Notes |
|---|---|---|
| Q3.2 | `OverlayId::new()` per effect — global atomic counter, same pattern as `SimThingId`. No new mechanism. | Approved 2026-05-22 |
| Q3.3 | `ActivationMode` enum on `CapabilitySpec` — `Threshold` registers Pass 7 threshold; `PlayerSelection` registers nothing. Builder handles both. Enum is `#[non_exhaustive]` — growable. | Approved 2026-05-22 |
| Q3.4 | Builder enforces `ReductionRule::Max` on all capability sub-fields unconditionally. Not caller-configurable. | Approved 2026-05-22 |
| Q3.5 | Add `ThresholdSemantic::CapabilityUnlock { sim_thing_id, property_id, sub_field }` to `threshold_registry.rs`. Builder registers it. Boundary handler matches on it and routes to studio layer. | Approved 2026-05-22 |
| Crate | RON-to-runtime compiler lives in new crate `simthing-spec`. Depends on `simthing-core` + `simthing-feeder`. Not `simthing-core` (too stable) and not `simthing-studio` (that's a UI layer). | Approved 2026-05-22 |
| Q4.1 | One `CapabilityTreeDefinition` per spec, shared across all faction instances. Read-only after session init. Slot used only for shadow reads at boundary time. | Approved 2026-05-22 |
| Q4.2 | Session coordinator owns `HashMap<SimThingId, (CapabilityTreeDefinitionId, Slot)>`. Definition never mutates. | Approved 2026-05-22 |
| Q4.3 | `CapabilityUnlockRegistration` in `simthing-feeder`. `ThresholdBuilder` grows `build_with_capability_unlocks`. Dep graph: `simthing-core` ← `simthing-feeder` ← `simthing-sim` / `simthing-spec`. | Approved 2026-05-22 |
| Q5.2 | Failed prereq: reset progress to `research_cost - EPSILON`, transition per-faction `ActivationMode` from `Threshold` → `OnPrereqMet`. `OnPrereqMet` is CPU-only, no GPU registration. Swept after every activation in same tree and once at session init. Per-faction mode state lives in `CapabilityTreeState` (extends Q4.2), recorded in delta log. | Approved 2026-05-22 |
| ActivationMode | Three arms: `Threshold` (Pass 7 threshold), `PlayerSelection` (explicit UI call), `OnPrereqMet` (CPU sweep only). `#[non_exhaustive]`. No GPU presence for `PlayerSelection` or `OnPrereqMet`. | Approved 2026-05-22 |
| D17 | Mutual exclusivity uses `CapabilityTreeState.active_by_category`. No overlay scanning. | Approved 2026-05-22 |
| D18 | Preview returns both per-overlay breakdown and combined result in `CapabilityPreviewReport`. | Approved 2026-05-22 |
| D19 | `max_active` is category-level. v0: `Unlimited` and `Limited(1)`. Replacement policy: `SuspendOldest` — handler issues `SuspendOverlay` automatically and emits `CapabilityTreeNotification::IdeaSwitched` for the session coordinator to surface to the UI. Switch is not silent but does not block on player input. `Limited(n > 1)` returns `CapabilityTreeError::UnsupportedMaxActive` in v0. | Approved 2026-05-22 |
| D20 | Always-on validation for authored input. `debug_assert` only for internal invariants. | Approved 2026-05-22 |
| D21 | Do not test global atomic `OverlayId` determinism. Test instead: `builder_records_overlay_ids_for_each_effect`, `definition_lookup_by_overlay_id_returns_entry`, `logical_effect_keys_are_stable_across_builds`. | Approved 2026-05-22 |

---

## Deferred / Out of Scope

- `simthing-studio` designer UI (graphical)
- Scenario RON expansion (inline tree references)
- GPU column budget monitoring
- `SimThingKind::Tech` promotion from `Custom("tech_tree")` (ergonomics only)
- Race system / biological-filter on World overlays (upstream dependency)
- World-level or system-level capability trees (design as needed)
