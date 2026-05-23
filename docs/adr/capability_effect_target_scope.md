# Capability Effect Target Scope

**Date:** 2026-05-23
**Status:** Accepted (implementation landed alongside this ADR)
**Blocks:** `simthing_modder_object_guide.md` capability-effect section, `simthing-studio` effect authoring UI
**Related:** [`game_mode_session_installation.md`](game_mode_session_installation.md) (O1, defines per-owner cloning), `capability_tree_v1.md` §14 (decision table)
**Independent of:** O1b `emit_activation` fix (landed `2eff1e0`) — see Decision §3

**Implementation notes (post-decision):**

Resolving `affects` alone is insufficient. The GPU's overlay-prep stage
(`crates/simthing-gpu/src/overlay_prep.rs`) walks the SimThing tree
depth-first and applies overlay transforms to every descendant slot
that carries the target property — it does **not** consult `overlay.affects`.
For `EffectTarget::Owner` to actually transform the owner's slot, the
overlay must **live on the owner** (or an ancestor), since the clone is
a child of the owner. The install layer therefore:

1. Places each cloned overlay on the **host SimThing** dictated by its
   `EffectTarget` (Owner → owner; CapabilityTree → clone; SessionRoot
   → root).
2. Stamps `CapabilityTreeInstance.overlay_hosts: HashMap<OverlayId,
   SimThingId>` so the boundary handler can pick the correct `target`
   on `ActivateOverlay` / `SuspendOverlay`.
3. Seeds the target property on the host's `properties` map so the GPU
   overlay-prep stage emits deltas for it.

The `affects` field on the cloned overlay is set to the resolved target
for documentation / debug-readability; the runtime hot path only reads
`overlay_hosts` and the SimThing tree topology.

## Context

`CapabilityTreeBuilder::build` produces one template `SimThing` per
`CapabilityTreeSpec` with suspended effect overlays carrying
`PropertyTransformDelta`s. At install time,
[`install_tree_for_owner` in `crates/simthing-driver/src/install.rs`](../../crates/simthing-driver/src/install.rs):214–248
clones that template per resolved owner and constructs each cloned overlay as:

```rust
let cloned_overlay = Overlay {
    id:        new_id,
    kind:      template_overlay.kind.clone(),
    source:    template_overlay.source.clone(),
    affects:   vec![cloned_tree_id],          // ← v0 behavior
    transform: template_overlay.transform.clone(),
    lifecycle: template_overlay.lifecycle.clone(),
};
```

The `affects` field on an `Overlay` is the list of `SimThingId`s whose
property slots the GPU Pass-3 evaluator applies `transform` to when the
overlay is `Permanent`. In v0 this list contains exactly the cloned
capability-tree `SimThing` — **not** the owning faction, the session root,
or any modder-authored target.

`CapabilityEffectSpec` in
[`crates/simthing-spec/src/spec/capability.rs`](../../crates/simthing-spec/src/spec/capability.rs)
exposes `targets_property` and `sub_field_deltas` but no scope selector:

```rust
CapabilityEffectSpec(
    targets_property: "military::fleet_speed",
    sub_field_deltas: [(Amount, Multiply(3.0))],
    when_activated: Permanent,
)
```

A modder reading that spec reasonably assumes "when warp_drive unlocks, the
owning faction's `military::fleet_speed` is multiplied by 3." The runtime
does not deliver that. It multiplies whatever `military::fleet_speed`
happens to be registered on the cloned tree node, which (unless the modder
explicitly registers properties on the tree kind) is nothing — the transform
silently lands on a column that does not exist on the clone's slot, or it
modifies a column the faction reduction pipeline never reads.

[`capability_tree_v1.md` §14](../capability_tree_v1.md) documents this as a
known warning and gates Studio + modder guide on this ADR's resolution.

Three forces shape the decision:

1. **Modder mental model.** Capability effects in tech trees / national
   ideas / talent trees universally mean "the owner gets this bonus."
   Forcing modders to reason about reduction layering (tree-node property →
   max-reduced → faction visibility) is a surprise that will not survive
   contact with first-week documentation. The default must match intent.

2. **Cross-crate boundaries.** `simthing-sim` must remain spec-free
   (PR 11 invariant). The decision must resolve `affects` at **install
   time** in `simthing-driver::install`, not at activation time in
   `simthing-spec::boundary::capability_handler`. The handler can stay
   ignorant of effect-target semantics; it just emits
   `BoundaryRequest::ActivateOverlay { target: instance.tree_thing_id,
   overlay_id }` where `target` is the SimThing the overlay **lives on**
   (the clone), independent of where the overlay's `affects` points.

3. **Separability from O1b.**
   [`capability_handler.rs:211`](../../crates/simthing-spec/src/boundary/capability_handler.rs)
   currently emits `ActivateOverlay` with template `overlay_id`s read from
   `entry.overlay_ids` instead of clone-resolved ids from
   `instance.by_overlay`. This is a separate, narrower bug (the O1b fix
   Codex owns). Fixing O1b does not depend on this ADR, and this ADR does
   not depend on O1b — they touch different fields (`overlay_id` vs.
   `affects`). They can land in either order.

## Decision

### 1. Effect target is authored per effect, defaulting to `Owner`

Add `effect_target` to `CapabilityEffectSpec`:

```rust
// crates/simthing-spec/src/spec/capability.rs
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub enum EffectTarget {
    /// Transform applies to the install-time owner (the SimThing the
    /// capability tree was cloned **for**, not the cloned tree itself).
    /// This is the v1 default.
    #[default]
    Owner,

    /// Transform applies to the cloned capability-tree SimThing. Use this
    /// when the effect modifies state intrinsic to the tree node — counters,
    /// tree-local cooldowns, internal bookkeeping. v0 behavior.
    CapabilityTree,

    /// Transform applies to `Scenario::root.id`. Use this for global
    /// effects (galaxy-wide discoveries, era-shift triggers).
    SessionRoot,
}

pub struct CapabilityEffectSpec {
    pub targets_property: String,
    pub sub_field_deltas: Vec<(SubFieldRole, TransformOp)>,
    pub when_activated:   OverlayLifecycle,
    #[serde(default)]
    pub effect_target:    EffectTarget,
}
```

`#[serde(default)]` keeps every existing RON file parseable; omitting
`effect_target` yields `Owner`. The two example files at
[`docs/examples/game_mode_install_*.ron`](../examples/) stay valid without
edits — their effect blocks gain `Owner` semantics, which matches their
illustrative intent ("a faction researches a tree and gets a bonus").

### 2. Install resolves `affects` from `effect_target`

`install_tree_for_owner` switches the hard-coded `affects: vec![cloned_tree_id]`
to a per-effect resolver. The cloned overlay carries an `affects` list
derived from the matching `CapabilityEffectSpec.effect_target`:

```rust
// in install_tree_for_owner, replacing the template_overlay loop
for (template_overlay, effect_spec) in template.overlays.iter()
    .zip(compiled.build_out.effect_specs.iter())
{
    let new_id = OverlayId::new();
    overlay_id_map.insert(template_overlay.id, new_id);
    let affects = resolve_effect_target(
        &effect_spec.effect_target,
        owner_id,
        cloned_tree_id,
        scenario.root.id,
    );
    let cloned_overlay = Overlay {
        id:        new_id,
        kind:      template_overlay.kind.clone(),
        source:    template_overlay.source.clone(),
        affects,
        transform: template_overlay.transform.clone(),
        lifecycle: template_overlay.lifecycle.clone(),
    };
    cloned.add_overlay(cloned_overlay);
}
```

`CapabilityTreeBuildOutput` gains a parallel `effect_specs: Vec<&CapabilityEffectSpec>`
(or a small `EffectMeta` struct carrying just the target) so the install
loop has the original authoring intent zipped 1:1 with the template's
overlays. The builder already iterates effects in a stable order to emit
the template overlays — exposing that order is additive.

```rust
pub fn resolve_effect_target(
    target:       &EffectTarget,
    owner_id:     SimThingId,
    clone_id:     SimThingId,
    root_id:      SimThingId,
) -> Vec<SimThingId> {
    match target {
        EffectTarget::Owner          => vec![owner_id],
        EffectTarget::CapabilityTree => vec![clone_id],
        EffectTarget::SessionRoot    => vec![root_id],
    }
}
```

### 3. `emit_activation` is unchanged by this ADR

[`capability_handler.rs:211–215`](../../crates/simthing-spec/src/boundary/capability_handler.rs)
emits:

```rust
ctx.requests.push(BoundaryRequest::ActivateOverlay {
    target:     instance.tree_thing_id,
    overlay_id: *overlay_id,
});
```

The `target` field here is "which SimThing's overlay list contains the
overlay to activate" — that is **always** the cloned capability-tree
(`instance.tree_thing_id`), because the overlay lives on the clone
regardless of what its `affects` field points at. This is correct today
and remains correct after this ADR.

The O1b fix (use `instance.by_overlay` to resolve clone ids instead of
reading template `entry.overlay_ids`) is **orthogonal** and must land
independently. The ignored test
`open_from_spec_capability_unlock_activates_overlay_for_next_tick` exists
to catch O1b — it will start passing once Codex's fix lands, regardless of
whether this ADR has been implemented yet.

### 4. Preview reads from the resolved target's slot

[`capability_preview.rs:67`](../../crates/simthing-spec/src/preview/capability_preview.rs)
currently indexes:

```rust
let idx = input.tree_slot as usize * input.n_dims + col;
```

The preview must read **the same slot the overlay will transform**. For
`EffectTarget::Owner` that is the owner's slot, not the tree's. Add
`owner_slot: u32` and `root_slot: u32` to `CapabilityPreviewInput`, then
pick the source slot per effect:

```rust
pub struct CapabilityPreviewInput<'a> {
    // ... existing fields ...
    pub tree_slot:  u32,
    pub owner_slot: u32,
    pub root_slot:  u32,
}

fn source_slot(target: &EffectTarget, input: &CapabilityPreviewInput<'_>) -> u32 {
    match target {
        EffectTarget::Owner          => input.owner_slot,
        EffectTarget::CapabilityTree => input.tree_slot,
        EffectTarget::SessionRoot    => input.root_slot,
    }
}
```

The driver wires `owner_slot` from `allocator.slot_of(instance.owner_id)`
and `root_slot` from `allocator.slot_of(scenario.root.id)`. Both lookups
already exist in `install.rs` for the slot-overflow check.

The preview's per-overlay breakdown gains an `affects: SimThingId` field
so the Studio UI can render "affects: Terran Empire (Faction)" beside
each delta. Modders see exactly which entity each effect lands on.

### 5. Property registration scope follows effect target

`Owner`-targeted effects require their `targets_property` to be a property
the owner's SimThing carries — i.e. registered in `DimensionRegistry`
without a kind restriction, or registered on the owner's kind. The
existing `compile_property` flow in `compile_and_install` registers
properties globally (`registry` is one flat namespace), so no change is
needed — any property declared in the game mode is reachable from any
slot.

The modder guide adds a single rule: **`targets_property` columns must
exist on the resolved target's slot**. This is a no-op for v0 content
that hasn't been written yet, and a clear authoring constraint going
forward. The Studio property editor can validate this statically by
checking whether the property registration covers the install target's
kind.

### 6. Authored override example

Per-effect authoring lets one entry mix targets when needed:

```ron
CapabilitySpec(
    id: "warp_drive",
    research_cost: 80000.0,
    effects: [
        // Faction-level bonus (default — modder writes nothing extra)
        CapabilityEffectSpec(
            targets_property: "military::fleet_speed",
            sub_field_deltas: [(Amount, Multiply(3.0))],
            when_activated: Permanent,
        ),
        // Internal counter on the tree node itself
        CapabilityEffectSpec(
            targets_property: "tree_local::research_milestones",
            sub_field_deltas: [(Named("warp_unlocked"), Set(1.0))],
            when_activated: Permanent,
            effect_target:   CapabilityTree,
        ),
        // Global era flag readable by every other faction's AI
        CapabilityEffectSpec(
            targets_property: "world::era_flags",
            sub_field_deltas: [(Named("warp_age_reached"), Set(1.0))],
            when_activated: Permanent,
            effect_target:   SessionRoot,
        ),
    ],
)
```

## Consequences

(a) **v0 install behavior changes.** `install_tree_for_owner` no longer
sets `affects` to the clone for every effect. Existing integration tests
in `crates/simthing-driver/tests/session_integration.rs` that assert the
clone's overlay `affects` content must be updated — most should expect
`vec![owner_id]` after the change. The ignored O1b test is unaffected by
this distinction (it asserts overlay activation, not the resolved
`affects` list).

(b) **Builder output gains effect provenance.**
`CapabilityTreeBuildOutput` adds either a parallel `Vec<EffectMeta>` or a
richer per-overlay record carrying `effect_target`. Consumers must zip
this with `tree.overlays` in the same order the builder emits. A small
unit test in `compile/capability.rs` should pin that ordering invariant
to avoid silent drift.

(c) **Preview interface expands.** `CapabilityPreviewInput` gains
`owner_slot` and `root_slot`. Every caller of `preview_capability_effect`
must supply both. PR6's preview tests need updating; the driver layer is
the natural source of both slot values.

(d) **Reduction is no longer load-bearing for the common case.** With
`Owner` as default, faction-facing bonuses skip the reduction layer
entirely — they land directly on the faction's slot. Tree-local
properties (when explicitly `CapabilityTree`-targeted) still need
reduction if they should surface elsewhere. This simplifies the GPU
pipeline analysis for the dominant content pattern.

(e) **Fission inherits per-effect targets cleanly.** When O5 fission
clones an owner, the install module re-runs `install_tree_for_owner` for
the new owner, which re-resolves every `affects` against the new
`owner_id`. No per-overlay rewriting in the fission path — the same
install pipeline handles both initial install and post-fission install.

(f) **O1b is the gating fix, not this ADR.** The currently-ignored test
`open_from_spec_capability_unlock_activates_overlay_for_next_tick` is
gated on O1b (clone overlay-id lookup), not on this ADR. Implementing
this ADR without the O1b fix still leaves that test ignored. Implementing
the O1b fix without this ADR makes the test pass with v0
`CapabilityTree`-default semantics. The two should land in whichever
order is convenient; neither blocks the other.

(g) **Backwards compatibility.** Every existing RON file deserializes
without modification — `effect_target` is `#[serde(default)]`. The
**runtime semantics** of those files change (default flips from
clone-targeted to owner-targeted). This is acceptable because there is
no released content. Pre-existing test fixtures that depend on clone
targeting must opt in explicitly via `effect_target: CapabilityTree`.

(h) **Modder docs gain one rule, not a system.** The
`simthing_modder_object_guide.md` capability section adds: "Effects
target the owner by default. Set `effect_target: CapabilityTree` for
tree-internal state, or `SessionRoot` for global effects." No taxonomy,
no scope expressions, no resolution DSL.

## V1 Scope (what implements this ADR)

- Add `EffectTarget` enum and `effect_target` field on `CapabilityEffectSpec`.
- Expose effect-target provenance from `CapabilityTreeBuilder::build` (parallel
  metadata alongside `tree.overlays`).
- Replace `affects: vec![cloned_tree_id]` in `install_tree_for_owner` with
  `resolve_effect_target`.
- Add `owner_slot` and `root_slot` to `CapabilityPreviewInput`; route the
  per-effect source slot via `effect_target`.
- Update existing tests that assert overlay `affects` content.
- Update `capability_tree_v1.md` §14 from "pending" to "Accepted (link)".
- Add one acceptance test: `Owner`-default effect modifies the owner's
  property slot after unlock, not the clone's.
- Add one acceptance test: `CapabilityTree` opt-in effect modifies the
  clone's slot (preserves v0 behavior under explicit opt-in).

## Out of scope (deferred)

- **Scope expressions** like `Children(of: Owner)`, `Allies(of: Owner)`,
  `WithinRange(of: Owner, range: 5)`. These need a target-resolution DSL
  with its own ADR. The three-variant enum is sufficient for v1 and
  forward-compatible (add new variants without breaking existing files).
- **Dynamic re-targeting** at runtime (e.g. an effect that follows a
  changing flagship). Install-time resolution is the only model in v1.
- **Multi-target effects** (one effect hits owner + all children). For v1,
  one effect = one target. Modders compose multi-target by authoring
  multiple `CapabilityEffectSpec` entries with different `effect_target`s.
- **Cross-faction targeting** (e.g. "debuff every enemy faction"). Needs
  a faction-relation model that does not exist yet in `simthing-core`.
- **Validation that `targets_property` resolves on the install target's
  kind.** A `validate_capability_tree` diagnostic in `simthing-spec` is
  natural follow-up work; v1 trusts the modder and lets the GPU silently
  ignore unknown columns.

## Alternatives considered

- **(Alt-1) Keep `CapabilityTree` as default (v0 behavior), require modders
  to opt into `Owner`.** Rejected: every realistic tech/ideas/talent tree
  wants owner-targeting. Making the common case verbose punishes 95% of
  authoring to preserve a 5% case. The forward-compatible path is to make
  the common case implicit and require explicit opt-in for the unusual case.

- **(Alt-2) Resolve `effect_target` at activation time in the boundary
  handler instead of install time.** Rejected: forces the handler to know
  about `EffectTarget`, expanding `simthing-spec::boundary` surface area
  for no runtime benefit. The handler already has `instance.owner_id` and
  `instance.tree_thing_id`, but install-time resolution keeps the data on
  the overlay where the GPU already reads it, and avoids re-resolving
  every tick.

- **(Alt-3) Single global `EffectTarget` per `CapabilityTreeSpec` instead
  of per-effect.** Rejected: real trees mix owner bonuses with global era
  flags (cf. §6 warp_drive example). Per-effect granularity is the right
  unit because the schema already has one `CapabilityEffectSpec` per
  transform.

- **(Alt-4) Use the existing `InstallTargetSpec` enum as the effect target
  enum.** Rejected: install target answers "which owners receive this
  tree at install time" (a *set* of `SimThingId`s, resolved against the
  scenario tree). Effect target answers "which entity does this transform
  modify when activated" (a *role* relative to the install context). They
  read the same in `SessionRoot` but diverge in `Owner` (single resolved
  id vs. set of matching ids) and `AllOfKind` (not meaningful for an
  individual effect). Conflating them would force every effect to carry
  an install-context-specific selector.

- **(Alt-5) Add a `targets_thing` string alongside `targets_property` that
  names an authored SimThing tag.** Rejected: requires a SimThing tag /
  alias system that does not exist, and tangles authoring of effects with
  scenario authoring. The three-variant enum sidesteps this entirely.

- **(Alt-6) Default to `SessionRoot` (every effect global by default).**
  Rejected: catastrophic for per-faction trees — one faction researching
  warp_drive would buff every faction. The default must isolate effects
  to the install owner.
