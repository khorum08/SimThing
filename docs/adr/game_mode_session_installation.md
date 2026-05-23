# Game Mode → Session Installation

**Date:** 2026-05-23
**Status:** Accepted (O1 landed PR #53)
**Blocks:** O3 follow-up wiring (logical install targets), partial O4 (per-owner scripted event install)

## Context

Today `SimSession::open` builds a GPU-backed session from a `Scenario`
(builtin tree + registry). Spec runtime state is installed afterwards via
`SimSession::install_spec_state(SpecSessionState)`, and every test in
[`crates/simthing-driver/tests/session_integration.rs`](../../crates/simthing-driver/tests/session_integration.rs)
hand-assembles that `SpecSessionState`: it allocates a tree `SimThing`,
seeds a `DimensionRegistry`, runs `CapabilityTreeBuilder::build`, packages
`CapabilityTreeInstance` + `CapabilityTreeState` per faction, and calls
`add_capability_tree_instance`. No code path drives this from a
`GameModeSpec` or `DomainPackSpec`.

`GameModeSpec` and `DomainPackSpec` (see
[`spec/game_mode.rs`](../../crates/simthing-spec/src/spec/game_mode.rs) and
[`spec/domain_pack.rs`](../../crates/simthing-spec/src/spec/domain_pack.rs))
already carry `properties`, `overlays`, and `capability_trees`. They do **not**
carry: a list of factions, install targets (which owners get which trees),
seed values for tree progress, scripted events, or scenario shape (slot
count, ticks/day). A faction model does not exist at all yet — scenarios
contain a `root: SimThing` and nothing else.

`CapabilityTreeBuilder::build` produces a single template `SimThing` per
spec (`SimThingKind::Custom(tree_kind)`, `id` assigned by `SimThing::new`)
plus a `CapabilityTreeDefinition` and a `Vec<CapabilityUnlockRegistration>`
keyed to that template id (see [capability builder](../../crates/simthing-spec/src/compile/capability.rs):260–270).
The builder is owner-agnostic: the unlock registrations point at the
template's `tree_thing_id`, not at any faction. The current driver tests
work around this by treating `tree_thing_id == owner_id` — one faction,
one tree, same `SimThingId`.

Three concrete forces shape this ADR:

1. The `simthing-driver` integration test cited in the handoff acceptance
   criterion — "open a session from a `.ron` file with one capability tree
   spec and one faction, and the existing E2E unlock path works without any
   manual `add_capability_tree_instance` calls" — requires both a faction
   enumeration mechanism and a single `SimSession::open_from_spec` entry
   point.
2. `simthing-sim` must remain spec-free (PR 11 invariant). All installation
   logic therefore belongs in `simthing-driver` or `simthing-spec`, not in
   `BoundaryProtocol`.
3. Scripted event install targets are entangled with the
   [scripted event scope ADR](scripted_event_scope_model.md). This ADR keeps
   that decision narrow: scripted events install **session-globally** in v0
   (matching today's `scripted_current_slot`), and the scope ADR governs how
   per-owner scoping later layers on top.

## Decision

### 1. Add an authoring concept: `InstallTargetSpec`

Capability trees and (future) scripted events declare *which entities they
attach to* via a discriminated install-target spec, authored on the tree (not
on the `GameModeSpec` envelope, because a domain pack should be able to
distribute trees with their install rules intact).

```rust
// crates/simthing-spec/src/spec/install_target.rs
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum InstallTargetSpec {
    /// Install on every SimThing in the session root whose `kind` matches.
    /// V0 default for capability trees that target factions.
    AllOfKind { kind: SimThingKindTag },

    /// Install on the SimThings explicitly named in the scenario's
    /// `install_targets` map under this id.
    ScenarioListed { target_id: String },

    /// Install once on the session root (`Scenario::root.id`). Used for
    /// scripted events in v0 — see `scripted_event_scope_model.md`.
    SessionRoot,
}
```

Add to `CapabilityTreeSpec`:

```rust
pub struct CapabilityTreeSpec {
    pub tree_id: String,
    pub tree_kind: String,
    pub owner_kind: String,                          // already present
    pub categories: Vec<CapabilityCategorySpec>,
    #[serde(default = "default_capability_install")]
    pub install: InstallTargetSpec,
}

fn default_capability_install() -> InstallTargetSpec {
    InstallTargetSpec::AllOfKind {
        kind: SimThingKindTag::Custom, // resolved against owner_kind at install
    }
}
```

The default keeps PR 11 hand-built tests valid: omitting `install` from a RON
file means "every faction-like SimThing of the matching kind." Scenarios that
want explicit ownership use `ScenarioListed`.

### 2. Scenario gains an install target registry

```rust
// crates/simthing-driver/src/scenario.rs
pub struct Scenario {
    // ... existing fields ...
    /// Maps a `ScenarioListed { target_id }` reference to a set of
    /// `SimThingId`s present in `root`. Empty in v0 builtins.
    pub install_targets: HashMap<String, Vec<SimThingId>>,
}
```

`AllOfKind` does not need this map — it is resolved at install time by walking
`root` and matching `kind`. `ScenarioListed` exists so domain packs can ship
"goes on the human player's faction only" without a kind tag covering it.
`SessionRoot` resolves to `scenario.root.id` directly.

### 3. New driver entry point: `SimSession::open_from_spec`

```rust
// crates/simthing-driver/src/session.rs
impl SimSession {
    pub fn open_from_spec(
        scenario: Scenario,
        game_mode: &GameModeSpec,
    ) -> Result<Self, SessionError> {
        let mut session = Self::open(scenario)?;
        let spec_state = install::compile_and_install(
            game_mode,
            &session.scenario,
            &mut session.proto.registry,
            &mut session.proto.root,
            &mut session.proto.allocator,
        )?;
        session.install_spec_state(spec_state);
        Ok(session)
    }
}
```

`SimSession::open` stays unchanged for scenario-only callers. The new path
composes scenario + spec without forking the existing constructor. The legacy
`install_spec_state(SpecSessionState)` entry point remains public for tests
that want to hand-build state.

### 4. Installation logic lives in `simthing-driver::install`

A new `crates/simthing-driver/src/install.rs` owns the orchestration:

```rust
pub fn compile_and_install(
    game_mode: &GameModeSpec,
    scenario: &Scenario,
    registry: &mut DimensionRegistry,
    root: &mut SimThing,
    allocator: &mut SlotAllocator,
) -> Result<SpecSessionState, InstallError> {
    let mut state = SpecSessionState::new();

    // 1. Compile properties + overlays from game_mode + every domain pack.
    //    `compile_property` registers each on `registry`.
    for pack in &game_mode.domain_packs { compile_pack(pack, registry)?; }
    for prop in &game_mode.properties   { compile_property(prop, registry)?; }
    for ov   in &game_mode.overlays     { /* deferred — global overlays */ }

    // 2. Compile each capability tree spec once.
    let mut compiled: Vec<CompiledTree> = Vec::new();
    for tree_spec in capability_trees(game_mode) {
        let build_out = CapabilityTreeBuilder::build(tree_spec, registry)?.0;
        compiled.push(CompiledTree { spec: tree_spec, build_out });
    }

    // 3. Resolve install targets and clone trees per owner.
    for compiled in &compiled {
        let owners = resolve_install_target(
            &compiled.spec.install,
            scenario,
            root,
        );
        for owner_id in owners {
            install_tree_for_owner(
                compiled,
                owner_id,
                registry,
                root,
                allocator,
                &mut state,
            )?;
        }
    }

    // 4. Scripted events (deferred to scope ADR for ownership semantics).
    //    V0: every authored event installs at SessionRoot scope and uses
    //    `scripted_current_slot` exactly as it does today.
    Ok(state)
}
```

`InstallError` is a new `simthing-driver` error type wrapping `SpecError`,
unresolved install targets, missing scenario owners, and slot exhaustion. It
is returned through `SessionError::Install`.

### 5. Per-owner tree cloning: a real `SimThing`, not a template alias

The current builder produces one template `SimThing` per spec, and tests use
it directly. For multiple owners, each must get its own tree `SimThing` —
that is the only way `CapabilityUnlockRegistration { sim_thing_id, .. }`
can scope thresholds per owner.

`install_tree_for_owner`:

1. Clones `build_out.tree`, gives it a fresh `SimThingId` (via
   `SimThing::new(SimThingKind::Custom(tree_kind), tier)`), and copies the
   property values + overlays from the template. The clone's `OverlayId`s
   are **re-stamped** — see consequence (c) below.
2. Attaches the cloned tree as a child of `owner_id` in `root`.
3. Calls `allocator.populate_from_tree` on the new subtree to assign
   `tree_slot`.
4. Builds a `CapabilityUnlockRegistration` set for the cloned tree by
   re-running the unlock collection logic with `sim_thing_id = cloned_tree_id`
   (rather than reusing the template's registrations, which carry the
   template's id).
5. Constructs `CapabilityTreeInstance { owner_id, definition_id,
   tree_thing_id: cloned_tree_id, tree_slot }`, an empty
   `CapabilityTreeState`, and calls
   `state.add_capability_tree_instance(definition.clone(), instance, …, regs)`.

`CapabilityTreeDefinition` is shared across all clones of the same spec
(same `definition_id`). Only the instance + threshold registrations are
per-owner.

### 6. Scripted events: v0 install at session scope

Until the scope ADR lands, every `EventSpec` in a `GameModeSpec` compiles to
exactly one `ScriptedEventDefinition` added via
`state.add_scripted_event(definition)`, with `scripted_current_slot` set to
the session root's slot. This matches the existing PR 9/10 behavior.
`InstallTargetSpec::SessionRoot` is the implicit target.

The scope ADR will replace this step with per-owner instances; the install
crate gains a new `install_scripted_events_for_owner` call there.

### 7. RON authoring shape

A minimal RON file that exercises the new path:

```ron
GameModeSpec(
    id: "demo",
    display_name: "Demo Mode",
    spec_version: SpecVersion(major: 0, minor: 1, patch: 0),
    capability_trees: [
        CapabilityTreeSpec(
            tree_id: "ideas",
            tree_kind: "national_ideas",
            owner_kind: "Faction",
            install: AllOfKind(kind: Custom("Faction")),
            categories: [ /* unchanged */ ],
        ),
    ],
)
```

Scenarios using explicit lists drop `install_targets` keyed map entries
matching the tree's `ScenarioListed { target_id }`. Today's builtins (no
`InstallTargetSpec` authored at all) keep working under the default.

## Consequences

(a) **Driver gains an install module.** `crates/simthing-driver/src/install.rs`
becomes the single home for "compile spec → mutate session state." Tests that
still want hand-built `SpecSessionState` use `install_spec_state` directly;
nothing forces them through the new path.

(b) **`SimThingKindTag` becomes an authoring vocabulary.** `AllOfKind`
requires a way to compare authored kind strings against `SimThing.kind`.
Either widen `SimThingKindTag` to carry custom strings, or add a small
`kind_matches(authored: &str, sim: &SimThingKind) -> bool` helper in
`simthing-core`. Recommendation: the helper — it stays in core, is unit
testable, and avoids polluting the tag enum.

(c) **`OverlayId` re-stamping per clone.** Each cloned tree must allocate
fresh `OverlayId`s (via `OverlayId::new()`) for its suspended overlays, and
the `by_overlay` map on the shared `CapabilityTreeDefinition` becomes
ambiguous (one logical entry → N runtime overlay ids across N owners). The
two options:

  - **(c.i, recommended)** Move `by_overlay` off the shared
    `CapabilityTreeDefinition` and onto `CapabilityTreeInstance`. The
    definition keeps `effect_keys` (logical, stable); the instance owns
    `Vec<OverlayId>` per entry. This is the same shape needed by the
    [replay ADR](spec_session_state_replay.md), and the two changes should
    land together.
  - **(c.ii)** Keep per-definition `by_overlay` and forbid multiple owners
    per spec until per-owner overlay maps exist. Rejected: it caps O1 at one
    faction, which fails the acceptance criterion the moment a second
    faction is added.

(d) **Scenario root mutation.** `compile_and_install` mutates `root` by
attaching cloned trees as children. `SimSession::open` already calls
`populate_from_tree` and `initial_gpu_sync`; the new code mutates *after*
those and must call `populate_from_tree` again and re-run
`initial_gpu_sync`. `install_spec_state` already triggers a re-sync via
`sync_spec_threshold_registrations` + `initial_gpu_sync`, so the existing
hook covers this — but the allocator must be re-populated explicitly inside
`install_tree_for_owner`, not at the end.

(e) **Empty-`install_targets` is a hard error.** A spec that resolves to
zero owners is almost always an authoring mistake (wrong kind tag,
mis-typed `target_id`). `InstallError::NoMatchingOwners { tree_id, target }`
makes this loud. The escape hatch for "intentionally not installed yet" is
to drop the tree from `game_mode.capability_trees`.

(f) **`open_from_spec` does not write replay snapshots.** Replay capture is
still driven by `SimSession::record_to_path`. Spec-driven session init has
no interaction with replay until the [replay ADR](spec_session_state_replay.md)
lands; that ADR is responsible for ensuring the cloned tree structure and
spec installation are reproducible on replay open.

(g) **Backwards compatibility.** Existing scenario-only callers
(`SimSession::open`) keep working unchanged. Existing integration tests that
call `install_spec_state` directly keep working unchanged. The new path is
opt-in via `open_from_spec`.

## V0 Scope (what O1 implements)

- `InstallTargetSpec`, `Scenario::install_targets`, `InstallError`,
  `install.rs` orchestration, `SimSession::open_from_spec`.
- Capability tree install for `AllOfKind`, `ScenarioListed`, and
  `SessionRoot`. Per-owner clone with re-stamped `OverlayId`s.
- Move `by_overlay` from definition to instance (consequence c.i).
- Scripted events install session-globally exactly as today.
- One integration test: load a one-tree, one-faction RON file via
  `open_from_spec`, run the existing capability-unlock E2E flow, assert
  overlay activation — no manual `add_capability_tree_instance`.

## Out of scope (deferred)

- Per-owner scripted event install. Handled by
  [`scripted_event_scope_model.md`](scripted_event_scope_model.md).
- Replay of installed spec state. Handled by
  [`spec_session_state_replay.md`](spec_session_state_replay.md).
- Mid-session install/uninstall (hot-loading a domain pack). The install
  module is structured to allow this later — `compile_and_install` could
  become `install_pack(pack, …)` — but v0 only runs at session open.
- `simthing-studio` authoring UI for install targets.
- A scenario RON expansion that inlines spec content. Scenarios load
  separately and pass into `open_from_spec(scenario, game_mode)`.

## Alternatives considered

- **(Alt-1) Make `SimSession::open` always take a `GameModeSpec`.** Rejected:
  forces every scenario test to thread an empty `GameModeSpec`, and conflates
  "I want a GPU session" with "I want spec runtime." Keeping `open` and
  `open_from_spec` separate keeps the layering honest.
- **(Alt-2) Put install logic in `simthing-spec` instead of
  `simthing-driver`.** Rejected: install mutates `SimThing` tree, `SlotAllocator`,
  and `DimensionRegistry` — concrete runtime objects the spec crate does not
  own and should not depend on. Driver is the only crate that already depends
  on both spec and the runtime types.
- **(Alt-3) Put `install_targets` on the `GameModeSpec` envelope, not the
  tree.** Rejected: prevents a domain pack from shipping a tree that knows
  where it installs. Authoring needs the colocation.
- **(Alt-4) Auto-derive faction list from `SimThingKindTag::Faction`** in
  `Scenario`. Held off: no faction kind exists in `simthing-core` yet. The
  `kind_matches` helper plus authored `AllOfKind` strings gets us there
  without prematurely committing to a faction taxonomy.
