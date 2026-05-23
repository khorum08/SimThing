# Install Clone-Then-Commit

**Date:** 2026-05-23
**Status:** Accepted (I1 implementation landed)
**Blocks:** I1 (Studio preview / hot-reload safety) — ✅ landed
**Related:** [`game_mode_session_installation.md`](game_mode_session_installation.md), [`spec_session_state_replay.md`](spec_session_state_replay.md)

## Context

`compile_and_install` (in [`install.rs`](../../crates/simthing-driver/src/install.rs))
mutates three caller-owned inputs in place during a single call:

1. **`registry: &mut DimensionRegistry`** — `compile_property` and
   `CapabilityTreeBuilder::build` register new `SimPropertyId`s (one per
   `PropertySpec`, plus capability-category properties).
2. **`root: &mut SimThing`** — cloned capability trees are attached as
   children of resolved owners; properties are seeded on owner / root nodes;
   owner-targeted and root-targeted overlays are pushed onto host
   SimThings; etc.
3. **`allocator: &mut SlotAllocator`** — slots are re-populated after every
   tree attachment to give the cloned subtree dense indices.

If the install errors partway — e.g., a later capability tree fails to
build, an `InstallTargetSpec` resolves to zero owners, the allocator
overflows the scenario's reserved slot cap — the caller is left with:

- a registry that has *some* of the new properties registered,
- a `root` tree with *some* of the cloned subtrees attached,
- an allocator that has slots allocated to nodes from the now-failed
  install attempt.

For `SimSession::open_from_spec`, this isn't load-bearing today: the
session is constructed inside the function and dropped on error, so partial
state never escapes. But:

1. **Studio preview workflows need it.** A designer hits "preview install"
   on a `GameModeSpec`. The Studio view should show the proposed state
   (registry, tree, allocator, spec state) *without* mutating the live
   session, so the designer can choose to commit or discard.
2. **Hot-reload safety.** When the user modifies a spec mid-session and
   asks to re-install, a failed re-install must not corrupt the running
   session — the user expects "old state preserved on error."
3. **Cleaner contract.** A function that mutates three caller buffers and
   may fail halfway through is a footgun. Atomic-on-error matches the
   semantic the callers actually want.

This was called out as a footgun in
[`design_v6.5.md`](../design_v6.5.md) §6 and the Sonnet/Opus handoff doc
(post-O2 Opus work item I1).

The forces shaping this ADR:

1. **Studio preview is a real product use case** — not just a hypothetical
   atomicity nicety. The output of "preview" must be inspectable without
   side effects on the live session.
2. **Memory budget is fine for the install duration.** Capability trees +
   registry + slot table are all small relative to the GPU value buffer.
   Doubling them for the duration of one install call is a non-issue.
3. **`compile_and_install` is non-trivial to refactor into a pure delta
   pipeline.** `compile_property` allocates `SimPropertyId`s by mutating
   the registry as it goes; `CapabilityTreeBuilder::build` similarly
   registers category properties during the build. Reworking those into
   delta-recording would touch `simthing-spec`'s compile path, which is
   out of proportion to the I1 problem.

## Decision

**Adopt scratch-clone with explicit commit.** Two new entry points on top
of the existing `compile_and_install`, which becomes the in-place worker
both wrappers call into a freshly-cloned scratch space:

### 1. `preview_install` — staged data, no caller mutation

```rust
// crates/simthing-driver/src/install.rs

/// Staged result of a `preview_install`. Holds the fully-populated
/// registry / root / allocator / spec state that *would* be produced by
/// running the install for real. The caller chooses to commit (via
/// `apply_install_preview`) or discard.
pub struct InstallPreview {
    pub registry:  DimensionRegistry,
    pub root:      SimThing,
    pub allocator: SlotAllocator,
    pub state:     SpecSessionState,
}

pub fn preview_install(
    game_mode: &GameModeSpec,
    scenario:  &Scenario,
    registry:  &DimensionRegistry,
    root:      &SimThing,
    allocator: &SlotAllocator,
) -> Result<InstallPreview, InstallError>;
```

Internally: clone the three inputs, call `compile_and_install` against
the clones, return the populated scratch on success. On error, scratch
is dropped and caller state is untouched (because it was never passed
mutably).

### 2. `install_atomic` — drop-in replacement for `compile_and_install`

```rust
pub fn install_atomic(
    game_mode: &GameModeSpec,
    scenario:  &Scenario,
    registry:  &mut DimensionRegistry,
    root:      &mut SimThing,
    allocator: &mut SlotAllocator,
) -> Result<SpecSessionState, InstallError>;
```

Internally: same as `preview_install` plus a final commit step
(`*registry = preview.registry; *root = preview.root; *allocator =
preview.allocator`). On error, no commit happens and caller state is
unchanged.

### 3. `SimSession::apply_install_preview`

```rust
impl SimSession {
    pub fn apply_install_preview(&mut self, preview: InstallPreview) {
        self.proto.registry  = preview.registry;
        self.proto.root      = preview.root;
        self.proto.allocator = preview.allocator;
        self.install_spec_state(preview.state);
    }
}
```

Studio "preview then accept" lands cleanly: call `preview_install` against
the session's current state, examine the returned `InstallPreview`,
optionally call `session.apply_install_preview(preview)` to commit.

### 4. `open_from_spec` switches to `install_atomic`

```rust
pub fn open_from_spec(
    scenario:  Scenario,
    game_mode: &GameModeSpec,
) -> Result<Self, SessionError> {
    let mut session = Self::open(scenario)?;
    let spec_state = install_atomic(
        game_mode, &session.scenario,
        &mut session.proto.registry,
        &mut session.proto.root,
        &mut session.proto.allocator,
    )?;
    session.install_spec_state(spec_state);
    Ok(session)
}
```

Today's session-drop-on-error masks the partial-state problem; switching
to `install_atomic` makes the contract explicit and immediately useful
for downstream callers that might want to retry install with a corrected
spec.

### `compile_and_install` semantics

`compile_and_install` remains exported and unchanged in behavior. It is
now documented as the **in-place worker** used by the two atomic
wrappers, and is the right call when the caller is constructing a
fresh-and-dropped session (e.g., the existing test fixtures that build
state and discard it on error before any session exists). The wrapper
functions cost one round of clones, which is the only way to give
atomicity in general.

## Consequences

(a) **Memory tradeoff acknowledged.** Peak memory during an atomic
install is roughly 2× the registry + root + allocator size (caller's
original plus the scratch clone). These structures are small in
practice — KB-scale, even for large spec trees — and the duration is one
install call.

(b) **No changes to `simthing-spec`.** The compile path stays in-place
on its scratch `DimensionRegistry` clone; nothing needs to be reworked.

(c) **`SlotAllocator` gains `#[derive(Clone)]`.** All three fields
(`Vec<Option<SimThingId>>`, `HashMap<SimThingId, u32>`, `Vec<u32>`) are
trivially clonable. No semantic change.

(d) **Studio preview is unblocked.** The `InstallPreview` struct is
exactly the data Studio's "what would this install do?" panel needs.

(e) **`react_to_fission_clones` is untouched.** That path runs mid-session,
not at install — atomicity there is a separate concern (deferred).

(f) **Replay open path naturally benefits.** `open_replay_with_spec` calls
`open_from_spec`, which now uses `install_atomic`. A spec-mismatched
replay (recorded against a different game mode) fails cleanly with no
mutation.

## Alternatives considered

### Alt-1: Delta-recording install pipeline

Refactor `compile_and_install` to first compute a `InstallPlan` (new
properties, new SimThings, new slot allocations) and then `apply` it.
Rejected — touches the entire compile path in `simthing-spec` because
`compile_property` and `CapabilityTreeBuilder::build` allocate ids by
mutating the registry. Memory savings (no clone) aren't worth the
disruption for inputs that are KB-scale.

### Alt-2: Rollback on error

Track which mutations were applied and reverse them on error. Rejected —
slot tombstoning, registry restoration, and tree detachment are all
non-trivial inverses; the error paths multiply quickly.

### Alt-3: Two-phase commit at the spec level

Add a `prepare` / `commit` pair to every mutating spec function. Rejected
— same disruption as Alt-1, plus an API surface that would propagate
through Studio and tests.

## V0 Scope (what I1 implements)

- `InstallPreview` struct.
- `preview_install` (clones inputs, returns staged data).
- `install_atomic` (clones inputs, commits on success).
- `SimSession::apply_install_preview` for Studio integration.
- `SimSession::open_from_spec` switches to `install_atomic`.
- `SlotAllocator` gains `Clone`.
- Unit tests asserting atomic-on-error.
- Integration test: `apply_install_preview` produces a running session
  equivalent to `open_from_spec`.

## Out of scope (deferred)

- Atomicity for mid-session install (e.g., domain-pack hot-load). The
  `InstallPreview` machinery exists; the runtime integration to swap
  registry/root/allocator on a *running* session (with GPU resync,
  threshold re-registration, slot reallocation) is its own concern.
- Atomic spec hot-reload that preserves in-flight `SpecSessionState`
  (player selections, cooldowns) across re-install. Needs replay-style
  state merging — see `spec_session_state_replay.md` for the pattern.
- Memory optimization via incremental cloning (Cow or arena). Not
  needed at current scales.
