---
rung: ROW-SLOT-OBJECT-SEMANTICS-0
kind: rung
track: 0.0.8.7
base_sha: 310242c34b7c10178a1d8423a5fc38a3d29d4693
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(unclassified-scope)
owner_notes: "Rung 4.1, Phase 4 opener (P4). Frontier lane: Codex 5.6 Sol MAX (Owner-enabled). Draft credit: orchestrator pre-draft (path enumeration + wire-state fence). One-time DA synthesis — NOT precedent (Owner ruling): the orchestrator drafts handoffs; the DA reviews and issues. Greenfield charter at full strength. Every move is a RELOCATION or a DERIVATION reproducing existing behavior — bit-exactness judges everything."
surfaces: ["crates/simthing-core/src", "crates/simthing-kernel/src", "crates/simthing-sim/src", "crates/simthing-gpu/src", "crates/simthing-driver/src", "crates/simthing-driver/tests", "scripts/ci", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/tests"]
forbidden: ["ANY behavior/value change (replay + RF-1 + slot-layout identity judge; referee ASSERTIONS unedited — mechanical test moves for the ruled file splits only)", "serializing ephemeral GPU slot numbers into authored/wire SimThing state", "production child-row minting with raw SlotIndex/u32 beside the object-semantic door", "ColumnIndex plan-struct migration (rung 4.2) or legacy mint sweep (rung 9.2)", "WGSL shader semantic edits", "kind-branching to choose residency behavior", "a second allocator, layout authority, or topology registry"]
required_checks: ["cargo build --workspace", "cargo test -p simthing-core", "cargo test -p simthing-kernel", "cargo test -p simthing-sim", "full simthing-driver battery on live GPU", "doctrine-scan", "orientation-check", "doc-budget", "clearance"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "any-replay-or-value-divergence (STOP, never patch the referee)", "slot-derivation-cannot-reproduce-an-existing-layout", "object-semantics-require-persistent-authored-slot-ids", "a-production-path-still-needs-a-sidecar-alloc-bypass-after-the-door-lands", "a-split-requires-a-behavior-change (0.2 ruling gap — DA-route)"]
---
## BUILD
- P4 residency as object semantics (ANCHOR-ACK core-0087): the smallest typed
  core contract by which a SimThing slots into its parent and allocates child
  rows — the tree/object EMITS the residency request; the allocator EXECUTES
  it. The derivation must reproduce every existing layout EXACTLY (canonical
  TP + fixtures: identical slot assignments, derived vs legacy).
- Route EVERY production structural path through that one door: initial tree
  population, spec/capability child install, boundary AddChild, fission and
  cloned subtrees, remove/tombstone, reparent (reparent preserves stable row
  identity while the parent relation changes). No raw `alloc(id)` side doors
  remain on these paths.
- One authority end-to-end: object relation -> typed residency door ->
  SlotAllocator / parent-child topology / dense projection / WorldGpuState
  upload all agree, with no independent row minting. Columns continue only
  through the role-pathway doors.
- The two 0.2-ruled structural splits (relocations, not rewrites):
  (a) `reduction.rs` -> pure compile-plan (oracle family moves to the kernel
  cpu-oracle home); (b) `world_state.rs` -> pure executed (encode helpers move
  out). Census follows the moved code: `mixed_ruled` resolves to 0.
- Establish THE single WGSL encode/decode boundary module: the one doc-fenced
  place typed plan values drop to raw u32 for upload (0.1 door-comment style:
  family + promotion blocker). The relocated encode helpers land HERE; 4.2
  types everything up to this frontier.
- New rung referee: install / AddChild / fission-clone / reparent / tombstone
  each prove object-issued row identity, topology, projection, and stable-slot
  invariants; escaped-bug case proves a sidecar raw child-slot mint cannot
  create a valid production residency relation; slot-layout identity proof
  (derived vs legacy: equal) on canonical + fixtures.
- Stamp 4.1 + advance posture to `PLAN-STRUCT-TYPING-0` in-diff; regen
  orientation.
## FENCES
- Relocation/derivation-only law: bit-exact replay + RF-1 + CPU/GPU reduction
  parity + child-iteration determinism green with referee assertions
  UNEDITED; slot layouts byte-identical pre/post. Zero new public raw-u32
  surfaces; deprecated-`new` count must not rise.
## EXIT-PROOF
- Layout-identity + path-coverage referees green; grep/census: zero production
  child-row alloc bypasses outside fenced allocator internals; both splits
  landed, census `mixed_ruled=0`; boundary module exists and is the ONLY
  raw-drop site (grep-proven); workspace + core/kernel/sim + full adapter-
  pinned driver battery green; existing tests semantically unmodified. Stamp +
  posture advance in-diff per the ritual.
