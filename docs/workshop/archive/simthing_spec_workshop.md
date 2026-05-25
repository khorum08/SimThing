> **SUPERSEDED for implementation progress** — Use [`simthing_spec_progress_log.md`](simthing_spec_progress_log.md) and [`README.md`](README.md). Decision log D0–D21 below remains useful reference; step-by-step PR instructions are historical.

# simthing-spec workshop

## Purpose

This worksheet is the implementation handoff for the new `simthing-spec` crate.

It starts from the outstanding questions in the Claude capability-tree worksheet and folds in the subsequent ChatGPT/Codex design decisions about making `simthing-spec` the universal RON-to-runtime compiler layer.

The original Claude worksheet remains distinct and should not be edited in place. This document is the forward-working digest for Claude/Cursor/Codex implementation coherence.

---

## Source Context

Primary source worksheet:

- `Capability Tree Studio Layer — Workshop`, session `2026-05-22`

Current architectural backdrop:

- `docs/design_v6.md` is canonical.
- V6 suspended overlays and capability fission are landed.
- V6 guardrails are landed.
- B2 Approaches A/B/C are landed:
  - targeted value upload across growth
  - append-only threshold registry for pure fission growth
  - cached incremental reduction topology via `TopologyState`
- `docs/eml_integration_guidance.md` exists and frames EML as an optional backend for pure numeric expressions, not as the scripting language.

Core doctrine to preserve:

```text
During ticks:
  GPU values are authoritative.

During boundaries:
  GPU values are read into coord.shadow.
  coord.shadow is authoritative for mutation.
  coord.shadow is uploaded back to GPU.

SimThing.properties:
  semantic presence/default/initial values only.
  not authoritative runtime numeric state after simulation starts.
```

---

# 0. Mission Statement for `simthing-spec`

## APPROVED

`simthing-spec` is the crate where authored RON/game-design data is converted into live SimThing runtime artifacts.

It owns:

```text
RON schemas
spec validation
name/key resolution
property/registry construction
overlay construction
capability tree building
scripted trigger/effect parsing
scripted event definitions
threshold/unlock registration construction
boundary request templates
preview/explanation metadata
```

It does **not** own:

```text
GPU buffers
tick execution
boundary execution
fission/fusion execution
Pass 3 overlay application
Pass 7 threshold scan
slot allocator mutation during live simulation
```

The rule:

> `simthing-spec` compiles authored meaning into native SimThing primitives. It does not execute the simulation.

---

# 1. Crate Boundary

## Claude Q1.1 — Does `simthing-studio` already exist?

### Resolution

For this implementation path, do **not** begin with `simthing-studio`.

Create a new crate:

```text
crates/simthing-spec
```

`simthing-studio` remains the eventual designer GUI/editor layer and will depend on `simthing-spec`.

### Decision

```text
APPROVED:
  simthing-spec is the RON-to-runtime compiler crate.
  simthing-studio is deferred UI/editor/importer surface.
```

---

## Claude Q1.2 — Minimal dependency graph

Original prior decision: studio depends on `simthing-core` and `simthing-feeder`, not `simthing-sim`.

### Resolution

Apply that dependency rule to `simthing-spec`.

```text
simthing-core
   ↑
simthing-feeder
   ↑
simthing-spec
```

`simthing-spec` may depend on:

```text
simthing-core
simthing-feeder
serde
ron
thiserror
smallvec if useful
```

`simthing-spec` must **not** depend on:

```text
simthing-sim
simthing-gpu
simthing-driver
```

`simthing-driver` may depend on `simthing-spec` to assemble sessions.

### Decision

```text
APPROVED:
  simthing-spec depends on simthing-core + simthing-feeder.
  It does not depend on simthing-sim or simthing-gpu.
```

---

## Claude Q1.3 — Module layout

Original proposed studio layout:

```text
simthing-studio/
  spec.rs
  builder.rs
  definition.rs
  boundary.rs
  preview.rs
  error.rs
```

### Resolution

Adjust to `simthing-spec` and broaden the crate so it can eventually own all RON conversion.

Recommended layout:

```text
crates/simthing-spec/
  Cargo.toml
  src/
    lib.rs

    error.rs
    diagnostics.rs
    keys.rs

    spec/
      mod.rs
      property.rs
      overlay.rs
      trigger.rs
      effect.rs
      event.rs
      capability.rs
      scenario.rs

    compile/
      mod.rs
      context.rs
      registry.rs
      overlays.rs
      triggers.rs
      effects.rs
      events.rs
      capability.rs
      scenario.rs

    runtime/
      mod.rs
      capability_definition.rs
      capability_state.rs
      scripted_event_definition.rs
      compiled_trigger.rs
      compiled_effect.rs

    boundary/
      mod.rs
      capability_handler.rs
      event_handler.rs

    preview/
      mod.rs
      capability_preview.rs
      overlay_preview.rs

    ron.rs
```

Minimal first PR may include only:

```text
lib.rs
error.rs
diagnostics.rs
keys.rs
spec/capability.rs
compile/capability.rs
runtime/capability_definition.rs
runtime/capability_state.rs
preview/capability_preview.rs
```

### Decision

```text
APPROVED:
  Start with the capability modules, but shape the crate for all future RON-to-runtime conversion.
```

---

# 2. Data Model

## Claude Q2.1 — `CapabilityEffectSpec::when_activated`

### Question

Should `when_activated` map 1:1 to:

```rust
OverlayLifecycle::Suspended {
    when_activated: Box<OverlayLifecycle>
}
```

at session init?

### Resolution

Yes.

Every capability effect starts as a suspended overlay. The authored `when_activated` lifecycle becomes the payload that is unwrapped by `BoundaryRequest::ActivateOverlay`.

Example:

```ron
effect: (
  targets_property: "fleet_speed",
  sub_field_deltas: [("amount", Multiply(1.30))],
  when_activated: Permanent,
)
```

compiles to:

```rust
OverlayLifecycle::Suspended {
    when_activated: Box::new(OverlayLifecycle::Permanent)
}
```

### Decision

```text
APPROVED:
  Capability effects compile to suspended overlays wrapping the specified when_activated lifecycle.
```

---

## Claude Q2.2 — Multiple effects per capability

### Resolution

Approve.

Each effect compiles to one suspended overlay. `CapabilityDefinition` stores all effect overlay IDs.

```rust
CapabilityDefinition {
    overlay_ids: Vec<OverlayId>,
    ...
}
```

On activation, all overlay IDs are activated together.

### Decision

```text
APPROVED:
  CapabilitySpec.effects[] → Vec<OverlayId>.
  Activation applies all effect overlays.
```

---

## Claude Q2.3 / Q2.4 — Runtime prereq shape

### Original proposal

Same-property prereq:

```rust
struct CapabilityPrereq {
    col: usize,
    min_value: f32,
}
```

Cross-property prereq:

```rust
enum CapabilityPrereq {
    SameProperty { col: usize, min_value: f32 },
    CrossProperty { property_id: SimPropertyId, col: usize, min_value: f32 },
}
```

### Resolution

Prefer a unified runtime struct. Same-property is just the same capability tree slot with a specific `property_id`.

```rust
pub struct CapabilityPrereq {
    pub property_id: SimPropertyId,
    pub role: SubFieldRole,
    pub col: usize,
    pub min_value: f32,
}
```

`col` is global column index inside a row, resolved at build time from:

```text
registry.column_range(property_id) + layout.offset_of(role)
```

For prereq reads, the handler uses the capability tree slot for the faction instance:

```rust
let value = shadow[tree_slot * n_dims + prereq.col];
```

This supports same-category and cross-category prereqs without an enum.

### Decision

```text
APPROVED:
  Use unified CapabilityPrereq { property_id, role, col, min_value }.
  Resolve column indices at build time.
```

---

## Forward compatibility — `ResearchRateSpec`

### Resolution

Approve the worksheet’s future-proofing.

Do not store research rate as bare `f32`.

Use:

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ResearchRateSpec {
    Literal { value: f32 },
    // Script { expr: ScriptExpr }, // future
}
```

or if RON ergonomics favor enum shorthand:

```rust
pub enum ResearchRateSpec {
    Literal(f32),
    // Script(ScriptExpr),
}
```

No EML work is implied. This is only the authoring seam for future Script IR.

### Decision

```text
APPROVED:
  ResearchRateSpec::Literal(f32) now.
  ScriptExpr arm reserved for future.
```

---

# 3. CapabilityTreeBuilder

## Claude Q3.1 — Builder signature

### Original proposal

```rust
impl CapabilityTreeBuilder {
    pub fn build(
        spec: &CapabilityTreeSpec,
        registry: &mut DimensionRegistry,
        id_source: &mut dyn SimThingIdSource,
    ) -> Result<(SimThing, CapabilityTreeDefinition), CapabilityTreeError>
}
```

### Resolution

Do not introduce `SimThingIdSource` yet unless the core already supports it. Existing code uses global ID constructors. Keep first implementation simple and compatible.

Recommended first signature:

```rust
impl CapabilityTreeBuilder {
    pub fn build(
        spec: &CapabilityTreeSpec,
        registry: &mut DimensionRegistry,
    ) -> Result<CapabilityTreeBuildOutput, CapabilityTreeError>
}
```

Where:

```rust
pub struct CapabilityTreeBuildOutput {
    pub tree: SimThing,
    pub definition: CapabilityTreeDefinition,
    pub unlock_registrations: Vec<CapabilityUnlockRegistration>,
}
```

Future deterministic/stable content IDs can be added later through logical keys.

### Decision

```text
APPROVED FOR V0:
  CapabilityTreeBuilder::build(spec, registry) -> CapabilityTreeBuildOutput.
  Do not add SimThingIdSource yet.
```

### Note

Do not overclaim deterministic runtime IDs from `OverlayId::new()` or `SimThingId::new()`. They are acceptable v0 runtime IDs, but Studio/spec should preserve logical content keys separately.

---

## Claude Q3.2 — Overlay ID generation

### Existing worksheet decision

`OverlayId::new()` per effect.

### Amendment

Approve for runtime v0, with a caveat.

```text
Runtime overlay IDs:
  OverlayId::new()

Logical content keys:
  tree_id/category_id/entry_id/effect_index
```

Replay should serialize runtime IDs. Studio diff/authoring tools should use logical content keys.

### Decision

```text
APPROVED WITH CAVEAT:
  Use OverlayId::new() for runtime overlay ids.
  Also preserve logical effect keys for definition/debug/studio use.
```

---

## Claude Q3.3 — `research_cost: 0.0` and activation mode

### Existing worksheet decision

Add:

```rust
#[non_exhaustive]
pub enum ActivationMode {
    Threshold,
    PlayerSelection,
    OnPrereqMet,
}
```

### Resolution

Approve.

Use `ActivationMode` on `CapabilitySpec`.

```rust
pub struct CapabilitySpec {
    pub id: String,
    pub activation: ActivationMode,
    pub research_cost: f32,
    ...
}
```

Semantics:

```text
Threshold:
  registers Pass 7 capability unlock threshold.
  research_cost must be > 0.

PlayerSelection:
  registers no threshold.
  activation occurs through explicit handler call.

OnPrereqMet:
  runtime/per-faction state only.
  no GPU threshold.
  entered when a Threshold entry reaches cost but prereqs fail.
```

Important distinction:

- `ActivationMode` on `CapabilitySpec` is the authored default.
- Runtime `ActivationMode` per faction/entry lives in `CapabilityTreeState`.

### Decision

```text
APPROVED:
  ActivationMode is part authoring default and part runtime mutable state.
  Shared definition remains immutable.
```

---

## Claude Q3.4 — Capability subfields use `ReductionRule::Max`

### Resolution

Approve.

The builder must enforce `ReductionRule::Max` for all capability subfields.

Reason:

```text
Mean would dilute sparse capability columns across sibling nodes.
Max preserves unlocked/progress state through the reduction tier.
```

### Decision

```text
APPROVED:
  CapabilityTreeBuilder enforces ReductionRule::Max unconditionally.
```

---

## Claude Q3.5 — Threshold semantic for capability unlock

### Existing worksheet decision

Add:

```rust
ThresholdSemantic::CapabilityUnlock {
    sim_thing_id: SimThingId,
    property_id: SimPropertyId,
    sub_field: SubFieldRole,
}
```

### Resolution

Approve conceptually, but implement conservatively.

Add `CapabilityUnlockRegistration` in `simthing-feeder`.

```rust
pub struct CapabilityUnlockRegistration {
    pub sim_thing_id: SimThingId,
    pub property_id: SimPropertyId,
    pub sub_field: SubFieldRole,
    pub threshold: f32,
}
```

`simthing-spec` produces these registrations.

`simthing-sim` consumes them when building threshold registries, producing the `ThresholdSemantic::CapabilityUnlock` CPU semantic and the matching GPU threshold registration.

### Important B2 note

Do not immediately integrate capability unlocks into the B2 append-only threshold path. First implement full-rebuild support. Append optimization can be added later only if needed.

### Decision

```text
APPROVED:
  CapabilityUnlockRegistration lives in simthing-feeder.
  simthing-spec produces registrations.
  simthing-sim maps them to ThresholdSemantic::CapabilityUnlock.
  Full rebuild path first; append optimization later.
```

---

# 4. CapabilityTreeDefinition and Runtime State

## Claude Q4.1 — Definition shared across faction instances

### Resolution

Approve.

One `CapabilityTreeDefinition` per spec. Multiple faction instances share it.

The definition contains immutable metadata and lookup tables.

```rust
pub struct CapabilityTreeDefinition {
    pub id: CapabilityTreeDefinitionId,
    pub tree_id: String,
    pub entries: HashMap<CapabilityEntryKey, CapabilityDefinition>,
    pub by_threshold: HashMap<(SimPropertyId, SubFieldRole), CapabilityEntryKey>,
    pub by_overlay: HashMap<OverlayId, CapabilityEntryKey>,
}
```

### Decision

```text
APPROVED:
  Definition is shared and read-only.
```

---

## Claude Q4.2 — Who owns per-faction mapping?

### Existing worksheet decision

Session coordinator owns:

```rust
HashMap<SimThingId, (CapabilityTreeDefinitionId, Slot)>
```

### Resolution

Approve, but extend.

Introduce explicit runtime instance/state structs.

```rust
pub struct CapabilityTreeInstance {
    pub owner_id: SimThingId,
    pub definition_id: CapabilityTreeDefinitionId,
    pub tree_thing_id: SimThingId,
    pub tree_slot: u32,
}

pub struct CapabilityTreeState {
    pub owner_id: SimThingId,
    pub definition_id: CapabilityTreeDefinitionId,
    pub activation_mode_by_entry: HashMap<CapabilityEntryKey, ActivationMode>,
    pub active_by_category: HashMap<CategoryKey, Vec<CapabilityEntryKey>>,
}
```

The session coordinator owns instances/states.

`simthing-spec` defines the types and handlers.

### Decision

```text
APPROVED:
  Definition immutable.
  Per-faction mutable state lives in CapabilityTreeState.
  Session coordinator owns instance mapping.
```

---

## Claude Q4.3 — Where capability unlock registration lives

### Existing worksheet decision

`CapabilityUnlockRegistration` in `simthing-feeder`.

### Resolution

Approve.

This avoids circular dependency:

```text
simthing-core
  ↑
simthing-feeder
  ↑       ↑
simthing-sim   simthing-spec
```

### Decision

```text
APPROVED:
  CapabilityUnlockRegistration lives in simthing-feeder.
```

---

# 5. Boundary Handler

## Claude Q5.1 — Handler signature

### Original proposal

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

### Resolution

Adjust to use definitions + instances + state explicitly.

Recommended shape:

```rust
pub struct CapabilityTreeBoundaryHandler<'a> {
    pub registry: &'a DimensionRegistry,
    pub definitions: &'a HashMap<CapabilityTreeDefinitionId, CapabilityTreeDefinition>,
}

pub struct CapabilityBoundaryContext<'a> {
    pub n_dims: usize,
    pub shadow: &'a mut [f32],
    pub instances: &'a HashMap<SimThingId, CapabilityTreeInstance>,
    pub states: &'a mut HashMap<SimThingId, CapabilityTreeState>,
    pub requests: &'a mut Vec<BoundaryRequest>,
    pub diagnostics: &'a mut Vec<CapabilityTreeDiagnostic>,
}

impl<'a> CapabilityTreeBoundaryHandler<'a> {
    pub fn handle_threshold_events(
        &self,
        events: &[ThresholdEvent],
        ctx: &mut CapabilityBoundaryContext<'_>,
    ) -> Result<CapabilityBoundaryOutcome, CapabilityTreeError>;
}
```

The handler needs mutable shadow if it performs failed-prereq progress resets.

### Decision

```text
PROPOSED APPROVAL:
  Handler receives events + mutable boundary context.
  It emits BoundaryRequests and performs boundary-safe progress resets.
```

---

## Claude Q5.2 — Failed prereq behavior

### Existing worksheet decision

When a threshold entry fires but prereqs are unmet:

```text
1. reset progress to research_cost - EPSILON
2. transition runtime ActivationMode Threshold → OnPrereqMet
3. no GPU threshold registered while OnPrereqMet
4. sweep OnPrereqMet after every activation in same tree and once at session init
```

### Resolution

Approve, with one mechanical clarification.

Progress reset is a **boundary shadow mutation**, not a persistent overlay and not a normal tick-path intent delta.

Implement as either:

```rust
CompiledEffect::SetPropertyAtBoundary { ... }
```

or an internal handler method:

```rust
fn reset_progress_below_threshold(
    shadow: &mut [f32],
    tree_slot: u32,
    n_dims: usize,
    col: usize,
    value: f32,
)
```

Record it in capability state/delta log if replay needs to reconstruct runtime mode and progress.

### Decision

```text
APPROVED:
  Failed prereq reset is boundary-authoritative shadow mutation.
  Do not model it as persistent overlay.
```

---

## Claude Q5.3 — Multiple factions firing same threshold

### Resolution

No concern if the event includes or can resolve to the capability tree `SimThingId` / slot.

Each threshold event must be processed against its own instance:

```text
event slot/tree id
→ CapabilityTreeInstance
→ CapabilityTreeState
→ shared CapabilityTreeDefinition
→ read prereqs from that instance’s tree_slot
```

### Decision

```text
APPROVED:
  Batch API is fine.
  Handler processes events independently by instance/slot.
```

---

## Claude Q5.4 — Mutual exclusivity

### Original open question

How to discover active sibling overlays? Is `max_active` category-level?

### Resolution

Use per-instance state, not overlay-list scanning.

```rust
pub struct CapabilityTreeState {
    pub active_by_category: HashMap<CategoryKey, Vec<CapabilityEntryKey>>,
    ...
}
```

When activating entry `B` in category `C`:

```text
if max_active is Some(n):
  if active count >= n:
    suspend entries that must be displaced
    emit SuspendOverlay for their overlay_ids
    update active_by_category

emit ActivateOverlay for B overlay_ids
add B to active_by_category[C]
```

For v0, support:

```rust
pub enum MaxActive {
    Unlimited,
    Limited(usize),
}
```

Implement/test `Limited(1)` first. Allow `Limited(n)` in data model, but return a clear error if n > 1 is not implemented yet, or support it with FIFO/explicit replacement policy.

### Decision

```text
PROPOSED APPROVAL:
  max_active belongs on CapabilityCategorySpec.
  active siblings are tracked in CapabilityTreeState.active_by_category.
  Do not scan overlays as source of truth.
```

---

# 6. Impact Preview

## Claude Q6.1 — Preview API

### Original proposal

```rust
impl CapabilityTreeDefinition {
    pub fn preview_effect(
        &self,
        entry_id: &str,
        shadow: &WorldShadow,
        faction_slot: Slot,
        registry: &DimensionRegistry,
    ) -> Vec<(SimPropertyId, SubFieldRole, f32, f32)>
}
```

### Resolution

Move preview to a preview service, not directly on definition, because preview needs registry, shadow, target/affects resolution, and maybe output vectors later.

Recommended:

```rust
pub struct CapabilityPreviewInput<'a> {
    pub definition: &'a CapabilityTreeDefinition,
    pub state: &'a CapabilityTreeState,
    pub registry: &'a DimensionRegistry,
    pub shadow: &'a [f32],
    pub n_dims: usize,
    pub tree_slot: u32,
    pub entry: CapabilityEntryKey,
}

pub struct CapabilityPreviewDelta {
    pub property_id: SimPropertyId,
    pub role: SubFieldRole,
    pub current: f32,
    pub after: f32,
    pub overlay_id: OverlayId,
}

pub fn preview_capability_effect(
    input: CapabilityPreviewInput<'_>,
) -> Result<CapabilityPreviewReport, CapabilityTreeError>;
```

### Decision

```text
PROPOSED APPROVAL:
  Preview lives in simthing-spec::preview, not directly on definition.
```

---

## Claude Q6.2 — Combined vs per-overlay preview

### Resolution

Return both.

```rust
pub struct CapabilityPreviewReport {
    pub per_overlay: Vec<CapabilityPreviewOverlayBreakdown>,
    pub combined: Vec<CapabilityPreviewDelta>,
}
```

Designers need per-effect explanation. Players usually need combined before/after.

### Decision

```text
PROPOSED APPROVAL:
  Preview returns both per-overlay breakdown and combined result.
```

---

# 7. Mutual Exclusivity Details

## Claude Q7.1 — Can max_active be N > 1?

### Resolution

Data model should allow N > 1 now.

Implementation can initially support:

```text
None / Unlimited
Some(1)
```

For `Some(n > 1)`, either:

1. implement it generically now, or
2. reject with `CapabilityTreeError::UnsupportedMaxActive`.

Generic support is not hard if the activation call specifies replacement policy.

Recommended:

```rust
pub enum MaxActivePolicy {
    Unlimited,
    Limited {
        count: usize,
        replacement: ReplacementPolicy,
    },
}

pub enum ReplacementPolicy {
    RejectIfFull,
    SuspendOldest,
    ExplicitSelectionRequired,
}
```

For v0:

```text
Unlimited
Limited { count: 1, replacement: SuspendOldest or ExplicitSelectionRequired }
```

For national ideas, `ExplicitSelectionRequired` may be cleaner: UI chooses what to replace.

### Decision

```text
PROPOSED APPROVAL:
  max_active is category-level and supports future N.
  v0 must support unlimited and exclusive-one.
```

---

## Claude Q7.2 — Atomic idea switching

### Resolution

Confirm.

Activation and suspension are both boundary structural requests. Issuing `SuspendOverlay` for A and `ActivateOverlay` for B in the same boundary cycle gives one-tick atomic transition.

### Decision

```text
APPROVED:
  Idea switch is boundary-atomic.
```

---

# 8. Error Handling and Validation

## Claude Q8.1 — Error list

### Resolution

Approve and expand.

`CapabilityTreeBuilder::build` should hard-fail on:

```text
duplicate tree ids
duplicate category ids
duplicate entry ids within tree or category
unknown prereq category
unknown prereq entry
self-prereq cycle if detectable
research_cost < 0
ActivationMode::Threshold with research_cost <= 0
ActivationMode::PlayerSelection with threshold-only fields set inconsistently
effect references unknown property
effect references unknown subfield role
invalid lifecycle payload
invalid max_active
empty capability_container_kinds when clone_capability_children expected by a fission template — warning, not builder error
missing required asset fields if strict asset validation enabled
```

For general `simthing-spec`, also validate:

```text
unknown property references
unknown overlay references
unknown event references
invalid scope references
unsupported ScriptExpr backend
```

### Decision

```text
PROPOSED APPROVAL:
  Builder returns hard errors for invalid semantics.
  Warnings only for suspicious but legal authoring choices.
```

---

## Claude Q8.2 — Hard failures vs warnings

### Resolution

Use both.

```rust
pub struct SpecDiagnostics {
    pub warnings: Vec<SpecWarning>,
}

pub type SpecResult<T> = Result<(T, SpecDiagnostics), SpecError>;
```

Hard errors for impossible/unsafe runtime configs.

Warnings for suspicious choices:

```text
clone_capability_children=true but capability_container_kinds=[]
unused category
entry has no effects
asset path missing in non-strict mode
max_active configured but no PlayerSelection entries
```

### Decision

```text
PROPOSED APPROVAL:
  Hard errors for invalid runtime semantics.
  Warnings for suspicious authoring.
```

---

## Claude Q8.3 — debug_assert vs runtime validation

### Resolution

Always-on validation for authored input.

`debug_assert!` is only for internal invariants after validation.

Examples:

```text
Capability subfields use ReductionRule::Max:
  always enforced/validated by builder.

Topology cache ordering:
  debug_assert/internal invariant.
```

### Decision

```text
PROPOSED APPROVAL:
  External authored input gets always-on validation.
  debug_assert only for internal code invariants.
```

---

# 9. Test Plan

## Claude Q9.1 — Proposed tests

### Resolution

Approve most tests, revise the overlay ID determinism test.

Integration tests:

```text
capability_tree_builder_registers_properties_and_overlays
capability_tree_boundary_handler_activates_on_threshold
capability_tree_prereq_blocks_activation_and_resets_progress
capability_tree_cross_category_prereq_resolves
capability_tree_impact_preview_returns_delta
national_ideas_mutual_exclusivity_suspends_sibling
```

Add:

```text
capability_tree_player_selection_activates_without_threshold
capability_tree_on_prereq_met_sweep_activates_after_dependency_unlock
capability_tree_failed_prereq_enters_on_prereq_met
capability_tree_state_is_per_faction_not_shared
```

Unit tests:

```text
prereq_resolution_same_category
prereq_resolution_cross_category
capability_subfields_force_reduction_max
activation_mode_threshold_requires_positive_cost
activation_mode_player_selection_registers_no_threshold
```

Remove or replace:

```text
overlay_id_generation_is_deterministic
```

Replace with:

```text
builder_records_overlay_ids_for_each_effect
definition_lookup_by_overlay_id_returns_entry
logical_effect_keys_are_stable_across_builds
```

### Decision

```text
PROPOSED APPROVAL:
  Use the revised test plan.
  Do not test global atomic OverlayId determinism.
```

---

# 10. General RON-to-Live-SimThing Expansion

## Approved aspiration

`simthing-spec` should eventually own all RON conversion into live SimThing space.

That means capability trees are the first target, not the final target.

`simthing-spec` should grow toward:

```text
GameSpec
PropertySpec
OverlaySpec
TriggerSpec
EffectSpec
EventSpec
CapabilityTreeSpec
ScenarioSpec
```

and compile them into:

```text
DimensionRegistry
SimThing trees
Overlay instances
Threshold registrations
Boundary request templates
Scripted event definitions
Capability tree definitions/states
Preview/explanation metadata
```

---

## 10.1 Property specs

RON defines properties and layouts.

Compiles to:

```text
SimProperty
PropertyLayout
SubFieldSpec
DimensionRegistry registrations
```

Validation:

```text
duplicate property keys
invalid role references
invalid clamp behavior
invalid reduction rule
invalid governed_by link
```

---

## 10.2 Overlay specs

RON defines overlays.

Compiles to:

```text
Overlay
PropertyTransformDelta
OverlayLifecycle
```

Rules:

```text
Flat modifier → Permanent or Transient overlay
Unlockable effect → Suspended overlay
Timed effect → Transient overlay
```

---

## 10.3 Trigger specs

RON triggers compile to:

```text
simple threshold trigger → ThresholdRegistration
composite numeric trigger → ScriptExpr / derived field + threshold
semantic trigger → CPU boundary predicate
```

Do not force all triggers into EML.

---

## 10.4 Effect specs

RON effects compile to:

```text
BoundaryRequest templates
boundary shadow patches
overlay activation/suspension actions
AddChild/Remove/Reparent requests
event emission definitions
```

Effects do not compile to EML.

---

## 10.5 Scripted events

RON scripted events are:

```text
event = trigger + effects + optional cooldown/priority/scope
```

Runtime representation:

```rust
pub struct ScriptedEventDefinition {
    pub id: EventKey,
    pub trigger: CompiledTrigger,
    pub effects: Vec<CompiledEffect>,
    pub cooldown: Option<CooldownSpec>,
    pub priority: EventPriority,
}
```

Execution remains CPU boundary side.

---

## 10.6 Script IR

Add canonical Script IR before EML.

```rust
pub enum ScriptExpr {
    Const(f32),
    Read { scope: ScopeRef, property: PropertyKey, role: SubFieldRole },
    Add(Box<ScriptExpr>, Box<ScriptExpr>),
    Sub(Box<ScriptExpr>, Box<ScriptExpr>),
    Mul(Box<ScriptExpr>, Box<ScriptExpr>),
    Div(Box<ScriptExpr>, Box<ScriptExpr>),
    Min(Box<ScriptExpr>, Box<ScriptExpr>),
    Max(Box<ScriptExpr>, Box<ScriptExpr>),
    Clamp { value: Box<ScriptExpr>, min: f32, max: f32 },
    Gate(Box<ScriptPredicate>),
}
```

EML remains optional future backend.

---

# 11. Implementation Path

## PR 1 — `simthing-spec` scaffold + core data model

Implement:

```text
crate setup
error.rs
diagnostics.rs
keys.rs
spec/capability.rs
ActivationMode
ResearchRateSpec
CapabilityTreeSpec
CapabilityCategorySpec
CapabilitySpec
CapabilityEffectSpec
CapabilityPrereqSpec
```

No sim integration yet.

---

## PR 2 — CapabilityTreeBuilder

Implement:

```text
CapabilityTreeBuilder
property registration
ReductionRule::Max enforcement
suspended overlay construction
CapabilityTreeDefinition
CapabilityTreeBuildOutput
validation errors/warnings
```

---

## PR 3 — Capability unlock registration plumbing

Implement:

```text
CapabilityUnlockRegistration in simthing-feeder
ThresholdSemantic::CapabilityUnlock in simthing-sim
full-rebuild threshold builder support
```

Do not add B2 append optimization yet.

---

## PR 4 — Capability boundary handler

Implement:

```text
CapabilityTreeBoundaryHandler
CapabilityTreeInstance
CapabilityTreeState
handle_threshold_events
handle_player_selection
OnPrereqMet sweep
failed-prereq progress reset
ActivateOverlay/SuspendOverlay emission
```

---

## PR 5 — Preview + mutual exclusivity

Implement:

```text
preview_capability_effect
per-overlay and combined preview
max_active
active_by_category
national idea sibling suspension
```

---

## PR 6 — General spec compiler foundation

Add:

```text
PropertySpec
OverlaySpec
TriggerSpec
EffectSpec
EventSpec
GameSpec
```

This begins the general RON-to-live-SimThing path.

---

## PR 7 — Script IR

Implement ScriptExpr/ScriptPredicate and CPU evaluator.

No EML yet.

---

## PR 8 — Trigger/effect/event compiler

Implement:

```text
simple trigger → threshold
effect → boundary request template
event → trigger + effects
```

---

# 12. Deferred / Out of Scope

Do not implement yet:

```text
GUI simthing-studio
full Clausewitz parser
Stellaris importer
EML compiler
GPU expression evaluator
arbitrary Rust callbacks
complete game economy/combat/diplomacy systems
```

---

# 13. Approved Decision Log

| # | Decision | Status |
|---|---|---|
| D0 | `simthing-spec` is the universal RON-to-runtime compiler layer. | APPROVED |
| D1 | `simthing-studio` is deferred UI/editor layer depending on `simthing-spec`. | APPROVED |
| D2 | `simthing-spec` depends on `simthing-core` + `simthing-feeder`, not `simthing-sim` or `simthing-gpu`. | APPROVED |
| D3 | Capability effects compile to suspended overlays wrapping authored `when_activated`. | APPROVED |
| D4 | Multiple effects per capability compile to multiple overlay IDs, activated together. | APPROVED |
| D5 | Runtime prereqs use unified `{ property_id, role, col, min_value }`. | APPROVED |
| D6 | `ResearchRateSpec::Literal(f32)` now; future ScriptExpr arm reserved. | APPROVED |
| D7 | Builder v0 signature returns `CapabilityTreeBuildOutput`; no `SimThingIdSource` yet. | APPROVED |
| D8 | Runtime overlay IDs use `OverlayId::new()`, but logical effect keys are preserved. | APPROVED |
| D9 | `ActivationMode::{Threshold, PlayerSelection, OnPrereqMet}`. | APPROVED |
| D10 | Capability subfields force `ReductionRule::Max`. | APPROVED |
| D11 | `CapabilityUnlockRegistration` lives in `simthing-feeder`. | APPROVED |
| D12 | `ThresholdSemantic::CapabilityUnlock` maps fired threshold to capability handler. | APPROVED |
| D13 | One immutable `CapabilityTreeDefinition` per spec. | APPROVED |
| D14 | Per-faction mutable state lives in `CapabilityTreeState`. | APPROVED |
| D15 | Failed-prereq reset is boundary shadow mutation. | APPROVED |
| D16 | Multiple faction events process independently by instance/slot. | APPROVED |
| D17 | Mutual exclusivity uses `CapabilityTreeState.active_by_category`. | PROPOSED |
| D18 | Preview returns per-overlay breakdown and combined effect. | PROPOSED |
| D19 | max_active is category-level; support unlimited and exclusive-one in v0. | PROPOSED |
| D20 | Authored-input validation is always-on; debug_assert only for internal invariants. | PROPOSED |
| D21 | Do not test global atomic OverlayId determinism. Test logical key stability instead. | PROPOSED |

---

# 14. Next Claude/Cursor Prompt

Use this prompt to resume implementation:

```text
Read docs/design_v6.md, docs/invariants.md, docs/worklog.md, docs/eml_integration_guidance.md, and simthing_spec_workshop.md.

Implement PR 1 only:
- add crates/simthing-spec to the workspace
- depend only on simthing-core, simthing-feeder, serde, thiserror, and ron if needed
- add spec data model for capability trees
- add ActivationMode, ResearchRateSpec, CapabilityTreeSpec, CapabilityCategorySpec, CapabilitySpec, CapabilityEffectSpec, CapabilityPrereqSpec
- add CapabilityTreeError and diagnostics skeleton
- add keys for tree/category/entry/effect logical IDs
- add RON deserialize tests for a minimal capability tree
- do not touch simthing-sim, simthing-gpu, or simthing-driver yet except workspace Cargo.toml

Preserve V6 invariants. Do not implement EML, Clausewitz parsing, or Studio UI.
```
