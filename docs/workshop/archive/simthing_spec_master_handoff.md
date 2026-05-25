> **SUPERSEDED for implementation progress** — Use [`simthing_spec_progress_log.md`](simthing_spec_progress_log.md) and [`README.md`](README.md). This file is retained for archaeology and decision-log reference only.

# simthing-spec — Master Implementation Handoff
## Date: 2026-05-22

This is the single authoritative document for implementing `simthing-spec`
PRs 2–6. It supersedes and consolidates:

- `docs/workshop/capability_tree_studio_workshop.md` (Claude workshop Q&A)
- `docs/workshop/simthing_spec_workshop.md` (ChatGPT/Codex design pass)
- `docs/workshop/tech_tree_decisions.md` (prior session decisions)

Read this document top to bottom before writing any code. Then read
`docs/design_v6.md` and `docs/invariants.md`.

**Do not implement more than one PR at a time.** The PRs are sequenced
deliberately. Later PRs depend on earlier ones being stable.

---

## Part 0 — Orientation

### Mission

`simthing-spec` compiles authored meaning into native SimThing primitives.
It does not execute the simulation.

The simulation crates (`simthing-core`, `simthing-gpu`, `simthing-sim`)
never see "tech tree", "national ideas", "talent tree", or any
domain-specific progression concept. Those strings exist only in RON files
and `simthing-spec` code. The simulation receives opaque `SimProperty`
registrations, `Overlay` instances, and `BoundaryRequest`s.

### Current repo state

- PR 1 landed (`7eb48dc`). **212 tests passing, 1 ignored, zero warnings.**
- `crates/simthing-spec/src/` exists with:
  `diagnostics.rs`, `error.rs`, `keys.rs`, `lib.rs`, `metadata.rs`,
  `ron.rs`, `validate.rs`, `version.rs`, `spec/capability.rs`,
  `spec/domain_pack.rs`, `spec/game_mode.rs`, `spec/mod.rs`,
  `spec/overlay.rs`, `spec/property.rs`, `spec/scenario.rs`,
  `spec/script_stub.rs`
- No runtime compiler modules exist yet. No feeder/sim capability-unlock
  plumbing exists yet.
- PR #45 (exploratory vertical slice) was reverted by PR #46. Treat PR #45
  as useful prior art, not current architecture. Do not copy it directly.

### What not to do

Do not jump ahead to:
- `CapabilityTreeBuilder` before PR 2 lands
- Boundary handler before PR 4 lands
- Script IR, EML, or Studio GUI (deferred — see §0.4)
- Any `simthing-sim` or `simthing-gpu` changes before PR 4

### Deferred / out of scope

- `simthing-studio` designer GUI — depends on `simthing-spec` when built
- Script IR — PR 7
- EML backend — see `docs/eml_integration_guidance.md`; optional future
  backend for pure numeric Script IR expressions only
- Scenario RON expansion (inline tree/registry/shadow seeds)
- Map-scale representation
- Full Clausewitz parser / Stellaris importer

---

## Part 1 — Architecture Reference

### 1.1 Crate Dependency Graph

```
simthing-core
      ↑
simthing-feeder        ← CapabilityUnlockRegistration (PR 4)
      ↑           ↑
simthing-sim    simthing-spec
                      ↑
              simthing-driver    (session assembly — later)
                      ↑
              simthing-studio    (deferred GUI)
```

`simthing-spec` dependencies:
```toml
simthing-core   = { path = "../simthing-core" }
simthing-feeder = { path = "../simthing-feeder" }   # added in PR 4
serde           = { workspace = true, features = ["derive"] }
ron             = { workspace = true }
thiserror       = { workspace = true }
```

`simthing-spec` must never depend on `simthing-sim` or `simthing-gpu`.

### 1.2 Module Layout

```
crates/simthing-spec/src/
  lib.rs
  error.rs
  diagnostics.rs
  keys.rs
  metadata.rs
  version.rs
  validate.rs
  ron.rs

  spec/                          ← PR 1 (exists)
    mod.rs
    capability.rs
    property.rs
    overlay.rs
    game_mode.rs
    domain_pack.rs
    scenario.rs
    script_stub.rs

  compile/                       ← PR 2+
    mod.rs
    context.rs
    property.rs
    overlay.rs
    capability.rs

  runtime/                       ← PR 3+
    mod.rs
    capability_definition.rs
    capability_state.rs

  boundary/                      ← PR 5
    mod.rs
    capability_handler.rs

  preview/                       ← PR 6
    mod.rs
    capability_preview.rs
```

### 1.3 ActivationMode State Machine

`ActivationMode` is declared in `spec/capability.rs`. It has three arms.
Each arm has distinct GPU and runtime behavior.

```rust
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ActivationMode {
    /// research_cost must be > 0.
    /// Builder registers a Pass 7 CapabilityUnlock threshold.
    /// GPU watches the progress column; fires when crossed.
    Threshold,

    /// No threshold registered. No GPU presence.
    /// Activated by explicit handler call from UI/session coordinator.
    /// research_cost is irrelevant.
    PlayerSelection,

    /// No threshold registered. No GPU presence.
    /// Entered at runtime when a Threshold entry reaches research_cost
    /// but prereqs are not yet met.
    /// Checked by CPU sweep after every CapabilityUnlock activation
    /// in the same tree, and once at session init.
    /// NOT valid as an authored default — runtime-only state.
    OnPrereqMet,
}
```

**State transition:**

```
Authored default: Threshold | PlayerSelection
                       │
          (Threshold fires, prereqs unmet)
                       │
                       ▼
                  OnPrereqMet
                       │
     (sweep: all prereqs now met)
                       │
                       ▼
               ActivateOverlay issued
```

**GPU presence by arm:**

| Arm | Pass 7 threshold | GPU buffer entry |
|---|---|---|
| `Threshold` | yes — one per entry | yes |
| `PlayerSelection` | no | no |
| `OnPrereqMet` | no | no |

**Per-faction mutable state.** `ActivationMode` per entry is mutable at
runtime (the `Threshold → OnPrereqMet` transition). The shared
`CapabilityTreeDefinition` is read-only. Per-faction mode state lives in
`CapabilityTreeState::activation_mode_by_entry`. It must be recorded in
the delta log and replayable.

**`OnPrereqMet` sweep triggers:**
1. After every `CapabilityUnlock` activation in the same faction's tree
   (post-activation step in the boundary handler)
2. Once at session init (in case scenario seeds prereqs already satisfied)

The sweep reads prereq shadow columns via `col_for_role` on the capability
tree slot. Pure CPU. No tree traversal beyond the capability tree node.

### 1.4 Data Model

#### Authoring structs (RON-facing, `spec/capability.rs`)

```rust
pub struct CapabilityTreeSpec {
    pub tree_id:    String,
    pub tree_kind:  String,          // e.g. "tech_tree", "national_ideas" — opaque
    pub owner_kind: String,          // SimThingKind::Custom(name) of owning node
    pub categories: Vec<CapabilityCategorySpec>,
}

pub struct CapabilityCategorySpec {
    pub id:                 String,
    pub property_namespace: String,
    pub property_name:      String,
    pub display_name:       String,
    pub tier:               Option<u32>,
    pub max_active:         Option<MaxActivePolicy>,
    pub entries:            Vec<CapabilitySpec>,
}

pub struct CapabilitySpec {
    pub id:            String,
    pub display_name:  String,
    pub description:   String,
    pub flavor_text:   Option<String>,
    pub activation:    ActivationMode,          // authored default
    pub research_cost: ResearchRateSpec,
    pub icon:          String,
    pub prereqs:       Vec<CapabilityPrereqSpec>,
    pub effects:       Vec<CapabilityEffectSpec>,
    // asset fields (thumbnail, card_image, etc.) omitted for brevity
}

pub struct CapabilityEffectSpec {
    pub targets_property: String,               // property namespace::name
    pub sub_field_deltas: Vec<(String, TransformOp)>,
    pub when_activated:   OverlayLifecycle,     // Permanent | Transient(...)
}

pub struct CapabilityPrereqSpec {
    pub category: String,
    pub entry_id: String,
}

#[non_exhaustive]
pub enum ResearchRateSpec {
    Literal(f32),
    // Script(ScriptExpr),  // future — PR 7
}

#[non_exhaustive]
pub enum MaxActivePolicy {
    Unlimited,
    Limited {
        count:       usize,
        replacement: ReplacementPolicy,
    },
}

pub enum ReplacementPolicy {
    SuspendOldest,               // v0 supported
    ExplicitSelectionRequired,   // future
}
```

**Validation rules on `CapabilitySpec`:**
- `ActivationMode::Threshold` requires `ResearchRateSpec::Literal(v)` where `v > 0.0`
- `ActivationMode::PlayerSelection` ignores `research_cost`
- `ActivationMode::OnPrereqMet` is not valid as an authored default — hard error

#### Runtime structs (`runtime/capability_definition.rs`)

```rust
pub struct CapabilityTreeDefinition {
    pub id:           CapabilityTreeDefinitionId,
    pub tree_id:      String,
    pub entries:      HashMap<CapabilityEntryKey, CapabilityDefinition>,
    // Fast lookup for boundary handler:
    pub by_threshold: HashMap<(SimPropertyId, SubFieldRole), CapabilityEntryKey>,
    // Fast lookup for UI/preview:
    pub by_overlay:   HashMap<OverlayId, CapabilityEntryKey>,
}

pub struct CapabilityDefinition {
    pub key:          CapabilityEntryKey,
    pub display_name: String,
    pub description:  String,
    pub flavor_text:  Option<String>,
    pub overlay_ids:  Vec<OverlayId>,     // one per effect; activated together
    pub prereqs:      Vec<CapabilityPrereq>,
    pub effect_keys:  Vec<CapabilityEffectKey>,  // logical keys for studio/debug
    // research_cost NOT stored — became a Pass 7 threshold registration
}

pub struct CapabilityPrereq {
    pub property_id: SimPropertyId,
    pub role:        SubFieldRole,
    pub col:         usize,         // resolved at build time via col_for_role
    pub min_value:   f32,           // research_cost of prereq entry
}
```

#### Runtime instance and state (`runtime/capability_state.rs`)

```rust
/// One per faction instance. Immutable after session init.
pub struct CapabilityTreeInstance {
    pub owner_id:      SimThingId,
    pub definition_id: CapabilityTreeDefinitionId,
    pub tree_thing_id: SimThingId,
    pub tree_slot:     u32,
}

/// One per faction instance. Mutable at boundary time.
pub struct CapabilityTreeState {
    pub owner_id:              SimThingId,
    pub definition_id:         CapabilityTreeDefinitionId,
    /// Tracks runtime activation mode per entry.
    /// Entries not present default to their authored ActivationMode.
    pub activation_mode_by_entry: HashMap<CapabilityEntryKey, ActivationMode>,
    /// Tracks currently active entries per category for mutual exclusivity.
    pub active_by_category:    HashMap<CategoryKey, Vec<CapabilityEntryKey>>,
}

/// Emitted by the boundary handler for the session coordinator to surface.
pub enum CapabilityTreeNotification {
    IdeaSwitched {
        owner_id:     SimThingId,
        category:     CategoryKey,
        suspended:    CapabilityEntryKey,
        activated:    CapabilityEntryKey,
    },
}
```

#### Builder output (`compile/capability.rs`)

```rust
pub struct CapabilityTreeBuildOutput {
    pub tree:                  SimThing,
    pub definition:            CapabilityTreeDefinition,
    pub unlock_registrations:  Vec<CapabilityUnlockRegistration>,  // simthing-feeder type
}
```

### 1.5 Error and Diagnostics Contract

```rust
pub type SpecResult<T> = Result<(T, SpecDiagnostics), SpecError>;

pub struct SpecDiagnostics {
    pub warnings: Vec<SpecWarning>,
}
```

**Hard errors (`SpecError`) — builder returns `Err`:**
- Duplicate tree IDs, category IDs, entry IDs
- Prereq references unknown category or entry
- Self-referential prereq cycle (if detectable at build time)
- `research_cost < 0.0`
- `ActivationMode::Threshold` with `research_cost == 0.0`
- `ActivationMode::OnPrereqMet` as authored default
- Effect references unknown property or sub-field role
- Invalid `OverlayLifecycle` payload
- `Limited(n > 1)` — `CapabilityTreeError::UnsupportedMaxActive` in v0

**Warnings (`SpecWarning`) — builder continues:**
- `clone_capability_children: true` but `capability_container_kinds` is empty
- Entry has no effects
- Category has no entries
- Asset path missing (non-strict mode)
- `max_active` configured but no `PlayerSelection` entries in category

**Invariant:** always-on validation for all authored input (RON files,
deserialized structs). `debug_assert!` only for internal code invariants
after validation has already passed.

### 1.6 Spec-Layer Invariants

These apply in addition to `docs/invariants.md`:

- **No capability-semantic strings in `simthing-sim` or `simthing-core`.**
  "tech_tree", "national_ideas", "talent_tree" exist only in RON files and
  `simthing-spec` code.
- **`ReductionRule::Max` on all capability sub-fields.** The builder
  enforces this unconditionally. Not caller-configurable. Mean would dilute
  sparse capability columns to near zero across sibling spatial nodes.
- **Suspended overlays are GPU-free.** `build_overlay_deltas` skips any
  overlay where `is_active()` is false. All capability effects start as
  `OverlayLifecycle::Suspended { when_activated }`. They reach the GPU only
  after `BoundaryRequest::ActivateOverlay` transitions them.
- **`ActivationMode::OnPrereqMet` has no GPU presence.** Never register a
  Pass 7 threshold for an `OnPrereqMet` entry.
- **Column resolution only via `col_for_role`.** No hardcoded column
  indices anywhere in `simthing-spec` (I1, I4).
- **Prereq column indices resolved at build time.** The boundary handler
  does array reads, not name lookups.
- **`CapabilityTreeDefinition` is read-only after session init.** All
  per-faction mutable state lives in `CapabilityTreeState`.
- **`OverlayId::new()` for runtime IDs; logical effect keys for
  debug/studio.** Do not test global atomic ID determinism. Test logical
  key stability instead.

---

## Part 2 — PR Implementation Instructions

### PR 2 — Property + Overlay Spec Compiler

**What to build:**

`compile/property.rs` — compile `PropertySpec` RON into live `SimProperty`
and register it with `DimensionRegistry`:

```rust
pub fn compile_property(
    spec:     &PropertySpec,
    registry: &mut DimensionRegistry,
) -> SpecResult<SimPropertyId>
```

Enforces:
- No duplicate property registrations (namespace + name)
- Valid sub-field roles
- Valid `ClampBehavior`, `ReductionRule`, `DecayBehavior` values
- `governed_by` role references an existing role in the same layout

`compile/overlay.rs` — compile `OverlaySpec` RON into a live `Overlay`
instance ready for attachment:

```rust
pub fn compile_overlay(
    spec:     &OverlaySpec,
    registry: &DimensionRegistry,
) -> SpecResult<Overlay>
```

Enforces:
- Target property exists in registry
- Sub-field roles exist in property layout
- `OverlayLifecycle` payload is valid

`compile/context.rs` — `CompileContext` threading registry through
multi-spec compilations:

```rust
pub struct CompileContext<'a> {
    pub registry: &'a mut DimensionRegistry,
}
```

**What not to touch:**
- `simthing-sim`, `simthing-gpu`, `simthing-feeder`
- Capability tree builder (PR 3)
- Threshold plumbing (PR 4)

**Acceptance criteria:**

```
compile_property_registers_simpropertyid
compile_property_duplicate_key_is_hard_error
compile_property_invalid_governed_by_role_is_hard_error
compile_overlay_resolves_subfield_roles_to_columns
compile_overlay_unknown_property_is_hard_error
compile_overlay_suspended_lifecycle_round_trips
compile_context_threads_registry_across_multiple_properties
```

`cargo test --workspace` must pass (212+) before and after.

---

### PR 3 — CapabilityTreeBuilder

**What to build:**

`compile/capability.rs` — `CapabilityTreeBuilder`:

```rust
pub struct CapabilityTreeBuilder;

impl CapabilityTreeBuilder {
    pub fn build(
        spec:     &CapabilityTreeSpec,
        registry: &mut DimensionRegistry,
    ) -> SpecResult<CapabilityTreeBuildOutput>
}
```

Builder responsibilities in order:

1. Validate the spec (hard errors listed in §1.5)
2. For each category, register a `SimProperty` with one sub-field per
   entry using `SubFieldRole::Named(entry_id)`. Enforce `ReductionRule::Max`
   on every sub-field unconditionally.
3. Construct the capability tree `SimThing` with `SimThingKind::Custom(tree_kind)`.
   Seed all progress sub-fields to `0.0`.
4. For each entry, for each effect:
   - Call `OverlayId::new()` to generate a runtime ID
   - Record the logical effect key: `tree_id/category_id/entry_id/effect_index`
   - Construct `OverlayLifecycle::Suspended { when_activated: Box::new(effect.when_activated) }`
   - Attach the overlay to the capability tree `SimThing`
5. Resolve prereqs: for each `CapabilityPrereqSpec`, resolve
   `property_id` and `col` via `col_for_role` on the registered property.
   Build `CapabilityPrereq { property_id, role, col, min_value }`.
6. Build `CapabilityTreeDefinition` with `by_threshold` and `by_overlay`
   lookup maps.
7. For each `ActivationMode::Threshold` entry, build one
   `CapabilityUnlockRegistration { sim_thing_id, property_id, sub_field, threshold }`.
   `ActivationMode::PlayerSelection` entries produce no registration.
8. Return `CapabilityTreeBuildOutput { tree, definition, unlock_registrations }`.

**`ReductionRule::Max` enforcement:** call `registry.set_reduction_rule`
(or equivalent) after registering each sub-field. Do not accept or forward
whatever the spec specifies for capability sub-fields — override silently.

**`ResearchRateSpec`:** only `Literal(f32)` supported in v0. The threshold
value is the literal. `Script` arm reserved for PR 7.

**What not to touch:**
- `simthing-feeder` (the `CapabilityUnlockRegistration` type doesn't exist
  there yet — PR 4 adds it; for PR 3, define a placeholder local type or
  stub it)
- `simthing-sim`, `simthing-gpu`
- Boundary handler (PR 5)

**Acceptance criteria:**

```
capability_tree_builder_registers_properties_and_overlays
capability_tree_builder_enforces_reduction_max
capability_tree_builder_validates_duplicate_entry_id
capability_tree_builder_validates_threshold_requires_positive_cost
capability_tree_builder_validates_on_prereq_met_authored_default_is_error
capability_tree_builder_player_selection_produces_no_unlock_registration
capability_tree_prereq_resolution_same_category
capability_tree_prereq_resolution_cross_category
capability_tree_builder_records_overlay_ids_for_each_effect
capability_tree_definition_lookup_by_overlay_id_returns_entry
capability_tree_logical_effect_keys_are_stable_across_builds
```

---

### PR 4 — Capability Unlock Registration Bridge

**What to build in `simthing-feeder`:**

```rust
// crates/simthing-feeder/src/capability.rs (new file)

pub struct CapabilityUnlockRegistration {
    pub sim_thing_id: SimThingId,
    pub property_id:  SimPropertyId,
    pub sub_field:    SubFieldRole,
    pub threshold:    f32,
}
```

Re-export from `simthing-feeder/src/lib.rs`.

**What to build in `simthing-sim`:**

Add `ThresholdSemantic::CapabilityUnlock` to `threshold_registry.rs`:

```rust
CapabilityUnlock {
    sim_thing_id: SimThingId,
    property_id:  SimPropertyId,
    sub_field:    SubFieldRole,
},
```

Extend `ThresholdBuilder` with a new entry point:

```rust
pub fn build_with_capability_unlocks(
    root:                  &SimThing,
    registry:              &DimensionRegistry,
    allocator:             &SlotAllocator,
    velocity_alerts:       &[VelocityAlertRegistration],
    capability_unlocks:    &[CapabilityUnlockRegistration],
) -> (Vec<ThresholdRegistration>, ThresholdRegistry)
```

For each `CapabilityUnlockRegistration`:
- Resolve `slot` via `allocator.slot_of(sim_thing_id)`
- Resolve `col` via `col_for_role(sub_field, layout)` on the property
- Emit `ThresholdRegistration { slot, col, threshold, direction: Upward,
  event_kind, buffer: THRESH_BUF_VALUES }`
- Emit `ThresholdSemantic::CapabilityUnlock { sim_thing_id, property_id, sub_field }`
  at the matching `event_kind` index

**Full-rebuild path only.** Do not integrate with the B2 append-only path
yet. When the boundary is eligible for B2 append, the current code skips
capability unlock registrations. That is acceptable in v0 — the first
fission boundary after a capability tree is initialized will take the
full-rebuild path regardless. B2 append optimization for capability unlocks
is a future PR.

**Update `simthing-spec/Cargo.toml`** to add `simthing-feeder` dependency.
Update `compile/capability.rs` to use the real
`CapabilityUnlockRegistration` type from `simthing-feeder`.

**What not to touch:**
- `simthing-gpu` shaders
- B2 append path in `gpu_sync.rs` (do not break it)
- Boundary handler (PR 5)

**Acceptance criteria:**

```
capability_unlock_registration_in_feeder_is_public
threshold_builder_with_capability_unlocks_emits_correct_event_kind
threshold_builder_capability_unlock_resolves_slot_and_col
threshold_semantic_capability_unlock_round_trips_serde
capability_unlock_fires_in_boundary_integration_test
```

The last test is an integration test in `simthing-sim/tests/`: seed a
capability tree `SimThing` with progress at threshold, call
`build_with_capability_unlocks`, run one tick to cross the threshold,
verify `BoundaryOutcome` contains a `ThresholdEvent` with the correct
`event_kind` mapping to `ThresholdSemantic::CapabilityUnlock`.

---

### PR 5 — Capability Runtime State + Boundary Handler

**What to build:**

`runtime/capability_state.rs` — `CapabilityTreeInstance` and
`CapabilityTreeState` as specified in §1.4.

`boundary/capability_handler.rs` — `CapabilityTreeBoundaryHandler`:

```rust
pub struct CapabilityTreeBoundaryHandler<'a> {
    pub registry:    &'a DimensionRegistry,
    pub definitions: &'a HashMap<CapabilityTreeDefinitionId, CapabilityTreeDefinition>,
}

pub struct CapabilityBoundaryContext<'a> {
    pub n_dims:      usize,
    pub shadow:      &'a mut [f32],
    pub instances:   &'a HashMap<SimThingId, CapabilityTreeInstance>,
    pub states:      &'a mut HashMap<SimThingId, CapabilityTreeState>,
    pub requests:    &'a mut Vec<BoundaryRequest>,
    pub notifications: &'a mut Vec<CapabilityTreeNotification>,
    pub diagnostics: &'a mut Vec<CapabilityTreeDiagnostic>,
}

impl<'a> CapabilityTreeBoundaryHandler<'a> {
    /// Called by session coordinator after Pass 7 events are surfaced.
    /// Processes only CapabilityUnlock events; ignores others.
    pub fn handle_threshold_events(
        &self,
        events:  &[ThresholdEvent],
        cpu_reg: &ThresholdRegistry,
        ctx:     &mut CapabilityBoundaryContext<'_>,
    ) -> Result<(), CapabilityTreeError>;

    /// Called by session coordinator on explicit player selection.
    pub fn handle_player_selection(
        &self,
        owner_id: SimThingId,
        entry_key: CapabilityEntryKey,
        ctx:      &mut CapabilityBoundaryContext<'_>,
    ) -> Result<(), CapabilityTreeError>;

    /// Called once at session init, and after every activation in the
    /// same tree. Checks all OnPrereqMet entries for the faction.
    pub fn sweep_on_prereq_met(
        &self,
        owner_id: SimThingId,
        ctx:      &mut CapabilityBoundaryContext<'_>,
    ) -> Result<(), CapabilityTreeError>;
}
```

**`handle_threshold_events` logic:**

```
for each ThresholdEvent:
  if cpu_reg[event.event_kind] != CapabilityUnlock: skip

  resolve CapabilityTreeInstance by sim_thing_id
  resolve CapabilityTreeDefinition by instance.definition_id
  resolve CapabilityTreeState by instance.owner_id

  look up CapabilityDefinition by (property_id, sub_field) in definition.by_threshold

  read prereq values from shadow[instance.tree_slot * n_dims + prereq.col]

  if all prereqs met:
    emit_activation(definition, entry_key, instance, state, ctx)
  else:
    // Failed prereq path
    reset progress: shadow[tree_slot * n_dims + progress_col] = research_cost - EPSILON
    transition state.activation_mode_by_entry[entry_key] = OnPrereqMet
```

**`emit_activation` logic:**

```
for each overlay_id in definition.overlay_ids:
  ctx.requests.push(BoundaryRequest::ActivateOverlay {
    target: instance.tree_thing_id,
    overlay_id,
  })

update state.active_by_category[category_key]:
  check max_active policy
  if Limited(1) and already one active:
    get oldest active entry_key
    for each overlay_id of oldest entry:
      ctx.requests.push(BoundaryRequest::SuspendOverlay { ... })
    remove oldest from active_by_category
    ctx.notifications.push(CapabilityTreeNotification::IdeaSwitched { ... })
  add entry_key to active_by_category[category_key]

sweep_on_prereq_met(owner_id, ctx)   // check if anything was waiting on this
```

**`sweep_on_prereq_met` logic:**

```
for each (entry_key, mode) in state.activation_mode_by_entry
  where mode == OnPrereqMet:
    resolve CapabilityDefinition for entry_key
    read all prereq values from shadow at instance.tree_slot
    if all prereqs met:
      emit_activation(...)
      // remove from activation_mode_by_entry (back to effective default)
      state.activation_mode_by_entry.remove(entry_key)
```

**Progress reset is a direct shadow mutation** — not a persistent overlay,
not an intent delta. Write directly to `ctx.shadow` at the correct index.
Record in delta log if session coordinator requires replay of mode transitions.

**`Limited(n > 1)` returns `CapabilityTreeError::UnsupportedMaxActive`.**

**What not to touch:**
- `simthing-gpu`
- `simthing-sim` boundary execution path (the handler is called by the
  session coordinator, not embedded in `BoundaryProtocol`)
- PR 6 (preview/mutual exclusivity detail is in this PR for `max_active`,
  but full preview is PR 6)

**Acceptance criteria:**

```
capability_tree_boundary_handler_activates_on_threshold
capability_tree_prereq_blocks_activation_and_resets_progress
capability_tree_failed_prereq_enters_on_prereq_met
capability_tree_on_prereq_met_sweep_activates_after_dependency_unlock
capability_tree_player_selection_activates_without_threshold
capability_tree_cross_category_prereq_resolves
capability_tree_state_is_per_faction_not_shared
national_ideas_mutual_exclusivity_suspends_sibling
national_ideas_mutual_exclusivity_emits_notification
capability_tree_sweep_runs_at_session_init
```

---

### PR 6 — Preview + Mutual Exclusivity Completion

**What to build:**

`preview/capability_preview.rs`:

```rust
pub struct CapabilityPreviewInput<'a> {
    pub definition: &'a CapabilityTreeDefinition,
    pub state:      &'a CapabilityTreeState,
    pub registry:   &'a DimensionRegistry,
    pub shadow:     &'a [f32],
    pub n_dims:     usize,
    pub tree_slot:  u32,
    pub entry:      CapabilityEntryKey,
}

pub struct CapabilityPreviewDelta {
    pub property_id: SimPropertyId,
    pub role:        SubFieldRole,
    pub overlay_id:  OverlayId,
    pub current:     f32,
    pub after:       f32,
}

pub struct CapabilityPreviewOverlayBreakdown {
    pub overlay_id: OverlayId,
    pub effect_key: CapabilityEffectKey,
    pub deltas:     Vec<CapabilityPreviewDelta>,
}

pub struct CapabilityPreviewReport {
    pub per_overlay: Vec<CapabilityPreviewOverlayBreakdown>,
    pub combined:    Vec<CapabilityPreviewDelta>,   // net before/after across all effects
}

pub fn preview_capability_effect(
    input: CapabilityPreviewInput<'_>,
) -> Result<CapabilityPreviewReport, CapabilityTreeError>
```

**Implementation:** apply each suspended overlay's `PropertyTransformDelta`
against `shadow[tree_slot * n_dims + col]` in CPU simulation. Do not mutate
shadow. No GPU operation.

For `combined`, fold all overlay deltas for the same `(property_id, role)`
pair together to produce net before/after.

**Mutual exclusivity completion:** PR 5 implements `SuspendOldest` for
`Limited(1)`. PR 6 verifies the full national ideas path with an end-to-end
test covering activate A → activate B (A suspends, notification emitted) →
verify A overlay is `Suspended`, B overlay is active.

**Acceptance criteria:**

```
capability_tree_impact_preview_returns_delta
capability_tree_impact_preview_per_overlay_breakdown
capability_tree_impact_preview_combined_net_effect
capability_tree_impact_preview_multi_effect_entry
national_ideas_full_path_activate_switch_verify
```

---

## Part 3 — Approved Decision Log

All decisions are fully approved as of 2026-05-22. No open questions remain.

| # | Decision |
|---|---|
| D0 | `simthing-spec` is the universal RON-to-runtime compiler layer. |
| D1 | `simthing-studio` is deferred UI/editor layer depending on `simthing-spec`. |
| D2 | `simthing-spec` depends on `simthing-core` + `simthing-feeder`. Not `simthing-sim` or `simthing-gpu`. |
| D3 | Capability effects compile to `OverlayLifecycle::Suspended { when_activated }` wrapping authored lifecycle. |
| D4 | Multiple effects per capability compile to multiple `OverlayId`s, activated together. |
| D5 | Runtime prereqs use unified `{ property_id, role, col, min_value }`. Column resolved at build time. |
| D6 | `ResearchRateSpec::Literal(f32)` now. `Script(ScriptExpr)` arm reserved for PR 7. |
| D7 | Builder v0: `build(spec, registry) -> CapabilityTreeBuildOutput`. No `SimThingIdSource` yet. |
| D8 | Runtime overlay IDs use `OverlayId::new()`. Logical effect keys (`tree/category/entry/effect_index`) preserved separately for debug/studio. |
| D9 | `ActivationMode::{Threshold, PlayerSelection, OnPrereqMet}`. `#[non_exhaustive]`. `OnPrereqMet` is runtime-only — not valid as authored default. |
| D10 | Builder enforces `ReductionRule::Max` on all capability sub-fields unconditionally. |
| D11 | `CapabilityUnlockRegistration` lives in `simthing-feeder`. |
| D12 | `ThresholdSemantic::CapabilityUnlock` in `simthing-sim` maps fired threshold to capability handler. |
| D13 | One immutable `CapabilityTreeDefinition` per spec, shared across faction instances. |
| D14 | Per-faction mutable state lives in `CapabilityTreeState`. Session coordinator owns instance + state maps. |
| D15 | Failed-prereq progress reset is direct boundary shadow mutation. Not a persistent overlay. |
| D16 | Multiple faction threshold events processed independently by instance/slot in same batch. |
| D17 | Mutual exclusivity uses `CapabilityTreeState.active_by_category`. No overlay list scanning. |
| D18 | Preview returns both per-overlay breakdown and combined net effect in `CapabilityPreviewReport`. |
| D19 | `max_active` is category-level. v0: `Unlimited` and `Limited(1)` only. Replacement policy: `SuspendOldest` with `CapabilityTreeNotification::IdeaSwitched` emitted to session coordinator. `Limited(n > 1)` returns `UnsupportedMaxActive` error in v0. |
| D20 | Always-on validation for authored input. `debug_assert!` only for internal code invariants after validation. |
| D21 | Do not test global atomic `OverlayId` determinism. Test: `builder_records_overlay_ids_for_each_effect`, `definition_lookup_by_overlay_id_returns_entry`, `logical_effect_keys_are_stable_across_builds`. |

---

## Appendix — EML Note

EML (`exp(x) - log(y)` expression trees) is approved as a future optional
backend for pure numeric Script IR expressions only. It is not the scripting
language, not a replacement for overlays or thresholds, and not a
Clausewitz transpilation target.

Current phase: Phase 0 (documentation only).

The `ResearchRateSpec::Literal(f32)` → `Script(ScriptExpr)` seam in the
data model is the only EML-adjacent preparation required in these PRs.
No EML work is implied.

See `docs/eml_integration_guidance.md` for the full pipeline and phase plan.
