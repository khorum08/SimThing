# PR 5 Handoff Digest — Capability Runtime State + Boundary Handler

**Date:** 2026-05-22
**Author:** Claude Opus 4.7 (handoff at session end after landing PRs 2–4)
**Target agent:** Codex 5.5
**Branch state:** `master` @ `aac6d1f`. 245 tests passing, 1 ignored, zero warnings.

---

## Read first (in this order)

1. **`docs/workshop/simthing_spec_master_handoff.md`** — authoritative spec for PRs 2–6. **Part 2 → PR 5** is the section you implement.
2. **`docs/invariants.md`** — column resolution, shadow authority, fission rules.
3. **`docs/design_v6.md`** — V6 contracts, especially suspended-overlay lifecycle.
4. **`docs/worklog.md`** — recent session entries (PRs 2, 3, 4 detail).

Do not implement PR 6. Stop at PR 5. The master handoff is explicit: one PR at a time.

---

## What PRs 2–4 left for you

| PR | What it built | Where |
|---|---|---|
| 2 | `compile_property`, `compile_overlay`, `CompileContext` | `simthing-spec/src/compile/{property,overlay,context}.rs` |
| 3 | `CapabilityTreeBuilder` + runtime types | `simthing-spec/src/compile/capability.rs`, `simthing-spec/src/runtime/capability_definition.rs` |
| 4 | `CapabilityUnlockRegistration` (in `simthing-feeder`), `ThresholdSemantic::CapabilityUnlock`, `ThresholdBuilder::build_with_capability_unlocks` | `simthing-feeder/src/capability.rs`, `simthing-sim/src/threshold_registry.rs` |

PR 4's GPU integration test (`capability_unlock_fires_in_boundary_integration_test` in `simthing-sim/tests/boundary_integration.rs`) is the closest existing pattern to the end-to-end test you'll need for PR 5. **Read it before writing yours.**

---

## What PR 5 must build

### Files to create

```
crates/simthing-spec/src/runtime/capability_state.rs        (new)
crates/simthing-spec/src/boundary/mod.rs                    (new)
crates/simthing-spec/src/boundary/capability_handler.rs     (new)
crates/simthing-spec/tests/pr5_capability_handler.rs        (new)
```

### Files to modify

```
crates/simthing-spec/src/runtime/mod.rs                     (export capability_state)
crates/simthing-spec/src/lib.rs                             (add `pub mod boundary`, re-exports)
crates/simthing-spec/src/error.rs                           (new variants, see below)
crates/simthing-spec/src/spec/capability.rs                 (MaxActivePolicy + ReplacementPolicy — see divergences)
docs/todo.md                                                (mark PR 5 done)
docs/worklog.md                                             (entry at the top of the file)
```

### Types to add — `runtime/capability_state.rs`

Verbatim from master handoff §1.4 — match the field names exactly. Use `std::collections::HashMap`.

```rust
/// One per faction instance. Immutable after session init.
pub struct CapabilityTreeInstance {
    pub owner_id:      SimThingId,                        // the faction
    pub definition_id: CapabilityTreeDefinitionId,
    pub tree_thing_id: SimThingId,                        // the cap tree SimThing under the faction
    pub tree_slot:     u32,                               // resolved via allocator.slot_of(tree_thing_id)
}

/// One per faction instance. Mutable at boundary time.
pub struct CapabilityTreeState {
    pub owner_id:                 SimThingId,
    pub definition_id:            CapabilityTreeDefinitionId,
    /// Tracks runtime activation mode per entry. Entries not present
    /// default to their authored ActivationMode.
    pub activation_mode_by_entry: HashMap<CapabilityEntryKey, ActivationMode>,
    /// Tracks currently active entries per category for mutual exclusivity.
    /// Vec order = activation order (oldest first → newest last). PR 5's
    /// `SuspendOldest` policy pops the front.
    pub active_by_category:       HashMap<CategoryKey, Vec<CapabilityEntryKey>>,
}

pub enum CapabilityTreeNotification {
    IdeaSwitched {
        owner_id:  SimThingId,
        category:  CategoryKey,
        suspended: CapabilityEntryKey,
        activated: CapabilityEntryKey,
    },
}
```

Diagnostic type for context.diagnostics (handler may surface non-fatal issues):

```rust
pub enum CapabilityTreeDiagnostic {
    UnknownThresholdSimThing { sim_thing_id: SimThingId },
    UnknownDefinition { definition_id: CapabilityTreeDefinitionId },
    EntryNotInTree { definition_id: CapabilityTreeDefinitionId, entry: CapabilityEntryKey },
}
```

### Handler — `boundary/capability_handler.rs`

The master handoff §PR5 has the signatures verbatim. Reproduced here for convenience:

```rust
pub struct CapabilityTreeBoundaryHandler<'a> {
    pub registry:    &'a DimensionRegistry,
    pub definitions: &'a HashMap<CapabilityTreeDefinitionId, CapabilityTreeDefinition>,
}

pub struct CapabilityBoundaryContext<'a> {
    pub n_dims:        usize,
    pub shadow:        &'a mut [f32],                                            // direct mutation for progress reset
    pub instances:     &'a HashMap<SimThingId, CapabilityTreeInstance>,          // keyed by owner faction id
    pub states:        &'a mut HashMap<SimThingId, CapabilityTreeState>,
    pub requests:      &'a mut Vec<BoundaryRequest>,                             // from simthing-feeder
    pub notifications: &'a mut Vec<CapabilityTreeNotification>,
    pub diagnostics:   &'a mut Vec<CapabilityTreeDiagnostic>,
}
```

Three public methods on `CapabilityTreeBoundaryHandler<'a>`:

#### 1. `handle_threshold_events(&self, events: &[ThresholdEvent], cpu_reg: &ThresholdRegistry, ctx: &mut CapabilityBoundaryContext<'_>) -> Result<(), CapabilityTreeError>`

For each event:
- `let sem = cpu_reg.get(event.event_kind)`. Skip if not `ThresholdSemantic::CapabilityUnlock { sim_thing_id, property_id, sub_field }`.
- Look up `ctx.instances.get(&sim_thing_id)` — if absent, push `UnknownThresholdSimThing` diagnostic, continue.
   - **Note:** In the spec, `instances` is keyed by **owner_id (the faction)**, but the threshold semantic carries the **tree_thing_id**. Resolve by scanning `ctx.instances.values()` for the one with `tree_thing_id == sim_thing_id`. (Or rekey — see decision below.)
- Look up `ctx.states.get_mut(&instance.owner_id)`.
- Look up `definitions.get(&instance.definition_id)` → if absent, push `UnknownDefinition`, continue.
- Look up `entry_key = definition.by_threshold.get(&(property_id, sub_field.clone()))`. If absent, push `EntryNotInTree`, continue.
- Read prereq values from `ctx.shadow[instance.tree_slot * n_dims + prereq.col]` for each `prereq` in `definition.entries[entry_key].prereqs`. Pass if `value >= prereq.min_value`.
- If all pass → call `emit_activation` (private helper, see below).
- If any fail:
  - Write `ctx.shadow[tree_slot * n_dims + progress_col] = research_cost - EPSILON` where `progress_col = definition.entries[entry_key].progress_col` (you'll need to store this on `CapabilityDefinition` in PR 3's struct — actually it's resolvable: the `by_threshold` key has the `(property_id, sub_field)`, and `col_for_role(sub_field, layout)` resolves to `col`).
  - `state.activation_mode_by_entry.insert(entry_key.clone(), ActivationMode::OnPrereqMet)`.

After processing all events, for each owner whose tree was touched, call `sweep_on_prereq_met(owner_id, ctx)`.

#### 2. `handle_player_selection(&self, owner_id: SimThingId, entry_key: CapabilityEntryKey, ctx: &mut CapabilityBoundaryContext<'_>) -> Result<(), CapabilityTreeError>`

Pre-validate that the entry's authored ActivationMode is `PlayerSelection`. If not → `CapabilityTreeError::PlayerSelectionRequiresPlayerSelectionMode`.
Otherwise just `emit_activation` (player-selection entries skip prereq checks per master handoff).

#### 3. `sweep_on_prereq_met(&self, owner_id: SimThingId, ctx: &mut CapabilityBoundaryContext<'_>) -> Result<(), CapabilityTreeError>`

For each `(entry_key, mode)` in `ctx.states[&owner_id].activation_mode_by_entry` where `mode == OnPrereqMet`:
- Re-check all prereqs from shadow.
- If all met:
  - `emit_activation(...)` (note: this can transition more entries into OnPrereqMet or out of it — keep iterating until no change)
  - Remove the entry from `activation_mode_by_entry` (back to its authored default).

**Iteration safety:** collect candidate entry_keys into a Vec before mutating `activation_mode_by_entry` to avoid borrow checker issues.

### `emit_activation` (private)

```text
for each overlay_id in definition.entries[entry_key].overlay_ids:
  ctx.requests.push(BoundaryRequest::ActivateOverlay {
    target: instance.tree_thing_id,
    overlay_id,
  })

let category_key = entry_key.category.clone();
let category_max_active = definition.categories[&category_key].max_active;  // see note below
let mut category_active = ctx.states[owner_id].active_by_category
    .entry(category_key.clone()).or_default();

match category_max_active {
  None | Unlimited =>
    category_active.push(entry_key.clone());
  Limited { count: 1, replacement: SuspendOldest } => {
    if let Some(oldest) = category_active.first().cloned() {
      if oldest != entry_key {
        for overlay_id in definition.entries[&oldest].overlay_ids {
          ctx.requests.push(BoundaryRequest::SuspendOverlay {
            target: instance.tree_thing_id,
            overlay_id,
          });
        }
        category_active.remove(0);
        ctx.notifications.push(CapabilityTreeNotification::IdeaSwitched {
          owner_id, category: category_key,
          suspended: oldest, activated: entry_key.clone(),
        });
      }
    }
    category_active.push(entry_key.clone());
  }
  Limited { count: n, .. } if n != 1 =>
    // Should be unreachable — validator rejects in PR 3. Defensive panic.
    return Err(CapabilityTreeError::UnsupportedMaxActive);
}

// After activation, sweep OnPrereqMet entries (now-met dependencies).
self.sweep_on_prereq_met(instance.owner_id, ctx)?;
```

---

## Divergences from spec to resolve

### D1. `max_active` field type

**Current:** `CapabilityCategorySpec.max_active: Option<usize>` (a raw count).
**Handoff §1.4:** `Option<MaxActivePolicy>` where `MaxActivePolicy::Limited { count, replacement: ReplacementPolicy }`.
**PR 3 validator** rejects `Some(n)` for `n != 1`, so today only `None` and `Some(1)` reach the runtime.

**Resolution for PR 5:** Add the `replacement` field. Two paths:

- **Path A (preferred):** Change `CapabilityCategorySpec.max_active` to `Option<MaxActivePolicy>` and update `MaxActivePolicy` enum to `Limited { count: usize, replacement: ReplacementPolicy }`. Update validator + builder to use the new shape. Update the `pr1_spec.rs` and `pr3_capability_builder.rs` tests that touch `max_active`.
- **Path B (minimal):** Keep `Option<usize>` on the spec; hardcode `ReplacementPolicy::SuspendOldest` in the handler (it's the only v0 policy anyway). Add `ReplacementPolicy` enum for future use but don't surface it in the spec yet.

**Recommend Path A** — the handoff is explicit about the field shape and Path B will need redoing when v1 adds `ExplicitSelectionRequired`. But Path B is acceptable if you want to scope PR 5 tighter — note it as deferred in the worklog.

### D2. `CapabilityTreeDefinition.categories` field doesn't exist

PR 3 built `CapabilityTreeDefinition { entries, by_threshold, by_overlay }` but not a `categories` map. PR 5's `emit_activation` needs to look up `category.max_active` and similar policy fields.

**Resolution:** Add `categories: HashMap<CategoryKey, CapabilityCategoryDefinition>` to `CapabilityTreeDefinition` and populate in `CapabilityTreeBuilder::build`. The struct can be minimal:

```rust
pub struct CapabilityCategoryDefinition {
    pub key:         CategoryKey,
    pub property_id: SimPropertyId,
    pub max_active:  Option<MaxActivePolicy>,  // or Option<usize> per D1 path B
    pub tier:        u32,
}
```

### D3. `CapabilityDefinition` needs `progress_col` and `research_cost`

PR 5 reads:
- The progress column to write `research_cost - EPSILON` on failed prereq
- The research cost for that EPSILON math

PR 3's `CapabilityDefinition` doesn't store these directly. Resolve by:

- Add `pub progress_col: usize` and `pub research_cost: f32` to `CapabilityDefinition` in `runtime/capability_definition.rs`.
- Set them in the PR 3 builder. `progress_col` is `range.col_for_role(&SubFieldRole::Named(entry.id), layout).unwrap()`; `research_cost` is `entry.research_cost`.

**This is a one-line change in PR 3's builder and a small struct addition. Do it as part of PR 5; do not split into a separate PR.**

### D4. Instance lookup by tree_thing_id vs owner_id

`CapabilityTreeBoundaryHandler` receives `ThresholdEvent`s whose `sim_thing_id` is the **tree** (per PR 3: `unlock.sim_thing_id = tree.id`). But the handoff keys `ctx.instances` by **owner** (faction). The handler needs reverse lookup.

**Resolution:** Either:
- Key `ctx.instances` by `tree_thing_id` (rename the field or keep two maps)
- Add `tree_thing_id_to_owner: HashMap<SimThingId, SimThingId>` as an inverse lookup
- Scan `ctx.instances.values()` linearly (fine for v0, faction counts are small)

**Recommend:** Pass two maps in the context — `instances_by_owner` and `instances_by_tree_thing`. Both reference the same `CapabilityTreeInstance` via `Rc<>` or via owner-id-then-lookup. Or just key by `tree_thing_id` — that's what threshold events naturally surface, and the session coordinator can build per-owner views.

### D5. `CapabilityTreeError` enum

Doesn't exist yet. Create in `boundary/capability_handler.rs`:

```rust
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum CapabilityTreeError {
    #[error("entry `{0}` has authored ActivationMode != PlayerSelection — cannot drive via handle_player_selection")]
    PlayerSelectionRequiresPlayerSelectionMode(String),
    #[error("category `{0}` declared max_active > 1 but only Unlimited / Limited(1) are supported in v0")]
    UnsupportedMaxActive(String),
    // add more as needed
}
```

---

## Acceptance tests (from master handoff)

Implement all 10 in `crates/simthing-spec/tests/pr5_capability_handler.rs`:

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

These are **CPU-only unit tests**. No GPU required. Each builds:
- A `CapabilityTreeBuilder::build(...)` output (or a hand-rolled `CapabilityTreeDefinition`)
- A small `instances` / `states` map for one or two factions
- A fake `shadow: Vec<f32>` initialized to specific values
- Drives the handler method
- Asserts on `ctx.requests` (which `BoundaryRequest` variants got pushed) and `ctx.notifications`

`shadow` is just a `Vec<f32>` of size `n_slots * n_dims`. Write to `shadow[tree_slot * n_dims + col]` to simulate progress / prereqs.

---

## Gotchas from PRs 2–4

1. **The pass-order trap.** GPU pipeline runs `intent_deltas → snapshot → velocity → intensity → overlay → threshold`. `BoundaryRequest::ActivateOverlay` works because it flips a `Suspended` overlay's lifecycle at boundary time; the **next tick**'s Pass 3 then applies it (already post-snapshot). PR 5's `emit_activation` is correct: it issues `ActivateOverlay` requests; the boundary handler doesn't need to push values mid-tick. **Your unit tests don't run the GPU**, so you don't hit this — but if you write a GPU integration test, follow the V6 Priority 1 pattern (`activated_suspended_overlay_appears_in_gpu_delta_and_affects_values` in `simthing-sim/tests/boundary_integration.rs`).

2. **`registry.register` panics on duplicates.** PR 2's `compile_property` checks `id_of` first and returns `SpecError::DuplicateProperty`. PR 3's `CapabilityTreeBuilder::build` does the same. **PR 5 doesn't register properties** — but if you're tempted to in the test fixtures, use `compile_property` not `registry.register`.

3. **`ReductionRule::Max` is forced** on capability progress sub-fields by PR 3's builder via `SubFieldSpec::reduction_override: Some(ReductionRule::Max)`. **PR 5 doesn't change reduction rules** — just be aware that the `default_for_role(Named)` would be `Mean`, which would silently bug-out aggregation.

4. **`OverlayId` is non-deterministic across builds** (atomic counter). Use `CapabilityEffectKey { entry, effect_index }` for logical identity. Tests asserting overlay-related state should look up via `definition.by_overlay`, not by raw id equality across builds.

5. **`CapabilityCategorySpec` has no `id` field.** Categories are identified by `CategoryKey { namespace, name }`. PR 3 uses `"namespace::name"` strings in prereq references; PR 5 should follow the same convention.

6. **`research_rate: ResearchRateSpec` is vestigial.** Ignore it. `research_cost: f32` is what the runtime reads.

7. **`OnPrereqMet` is runtime-only.** PR 3's validator rejects it as an authored default. PR 5 transitions entries INTO this state at runtime; storage lives in `CapabilityTreeState.activation_mode_by_entry`.

8. **Diagnostics vs errors.** Per master handoff D20: "Always-on validation for authored input. `debug_assert!` only for internal code invariants after validation has already passed." For PR 5: hard errors (the `CapabilityTreeError` enum) cover programmer mistakes by the session coordinator (e.g., calling `handle_player_selection` on a `Threshold` entry). Diagnostics cover data inconsistencies (unknown sim_thing, missing definition) that the handler can keep going past.

---

## Test patterns to mimic

- **Builder fixture helpers:** `crates/simthing-spec/tests/pr3_capability_builder.rs` has `registry_with_fleet_speed()`, `entry(...)`, `category(...)`, `tree_spec(...)`. Copy/adapt for PR 5.
- **Multi-faction setup:** Build two `CapabilityTreeBuilder::build` outputs against the same registry, instantiate one tree per faction, share the same `CapabilityTreeDefinition`. Use this to test `capability_tree_state_is_per_faction_not_shared` — activate an entry on faction A, verify faction B's state is untouched.
- **`shadow` construction:** Just `vec![0.0_f32; n_slots * n_dims]`. Set specific cells before calling the handler.

---

## Suggested implementation order

1. **Add `CapabilityCategoryDefinition` + `categories` map** in `runtime/capability_definition.rs`; populate in `compile/capability.rs`. Add `progress_col` + `research_cost` to `CapabilityDefinition`. Update PR 3 tests if they break.
2. **Resolve D1** (MaxActivePolicy shape) — recommend Path A; pick one.
3. **Add `CapabilityTreeError`** in `boundary/capability_handler.rs`.
4. **Implement `CapabilityTreeInstance` + `CapabilityTreeState` + `CapabilityTreeNotification` + `CapabilityTreeDiagnostic`** in `runtime/capability_state.rs`.
5. **Implement `CapabilityTreeBoundaryHandler` + `CapabilityBoundaryContext`** in `boundary/capability_handler.rs`. Stub all three methods returning `Ok(())`.
6. **Implement `handle_threshold_events`** + `emit_activation`. Test with the first two acceptance tests.
7. **Implement `sweep_on_prereq_met`** + the `OnPrereqMet` flow. Test 3, 4.
8. **Implement `handle_player_selection`**. Test 5.
9. **Cross-category, multi-faction, mutual exclusivity.** Tests 6–10.
10. **`cargo test --workspace`** — should be at 245 + ~10 new + ~2 builder-change-related changes = ~255–257 passing.
11. **Update `docs/todo.md` and `docs/worklog.md`.** Commit, push.

---

## Quick-reference: imports the handler will need

```rust
use crate::diagnostics::SpecDiagnostics;
use crate::keys::{CapabilityEntryKey, CategoryKey};
use crate::runtime::{
    CapabilityDefinition, CapabilityPrereq, CapabilityTreeDefinition,
    CapabilityTreeDefinitionId, CapabilityTreeInstance, CapabilityTreeState,
    CapabilityTreeNotification, CapabilityTreeDiagnostic,
};
use crate::spec::capability::ActivationMode;
use simthing_core::{DimensionRegistry, SimPropertyId, SimThingId, SubFieldRole};
use simthing_feeder::BoundaryRequest;
use simthing_sim::{ThresholdRegistry, ThresholdSemantic};  // ← cross-crate, check Cargo.toml
```

**Important:** `simthing-spec` currently does NOT depend on `simthing-sim`. The handler reads `ThresholdEvent` / `ThresholdRegistry` / `ThresholdSemantic` from `simthing-sim`. Adding the dep would create the layering `simthing-sim → simthing-spec → simthing-sim` cycle IF `simthing-sim` ever depended on `simthing-spec` — but it doesn't today. So adding `simthing-sim` to `simthing-spec/Cargo.toml` is safe.

**Alternative:** Lift `ThresholdSemantic` and `ThresholdRegistry` into `simthing-feeder` (where `CapabilityUnlockRegistration` lives). Then `simthing-spec` only depends on `simthing-feeder`. But this is a bigger refactor than PR 5 should take on. Defer.

**Recommend:** Add `simthing-sim = { path = "../simthing-sim" }` to `simthing-spec/Cargo.toml`. Document it as the new acceptable layering.

---

## Definition of done

- `cargo test --workspace` passes with all PR 5 acceptance tests + previous 245.
- `cargo build --workspace --tests` is zero-warnings.
- `docs/todo.md` shows PR 5 as `[x]` with summary.
- `docs/worklog.md` has a fresh PR 5 entry at the top.
- Commit on `master` with a message in the style of `aac6d1f` / `f1dbfa1` (sentence-case subject, technical body, Co-Authored-By footer).
- Pushed to `origin/master`.

After PR 5: PR 6 (preview + mutual exclusivity completion) remains. PR 6 is smaller — preview routine + an end-to-end mutual exclusivity test that ties PR 5's handler to a real activate-switch flow.

---

## If you get stuck

- The master handoff doc has the full data model + state machine for everything in this PR. Re-read §1.3 (ActivationMode state machine) and §PR5 if you lose the thread.
- The mutual exclusivity logic is the trickiest part — work it out on paper before coding `emit_activation`. The category's `active_by_category` Vec is the single source of truth for "what's active right now"; do not duplicate this in overlay scanning (D17 from master handoff).
- The `sweep_on_prereq_met` recursion (activating one entry might satisfy another's prereqs, which activates that, which satisfies more, etc.) needs a fixpoint loop or repeated sweeps. Don't recurse into `emit_activation` from inside the sweep without bounding the depth or collecting candidates upfront.
- `cargo test -p simthing-spec --test pr5_capability_handler` to run just your new tests.
