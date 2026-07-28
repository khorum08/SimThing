---
rung: ROW-SLOT-OBJECT-SEMANTICS-0
kind: rung
track: 0.0.8.7
base_sha: 310242c34b7c10178a1d8423a5fc38a3d29d4693
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(unclassified-scope)
owner_notes: "Rung 4.1, Phase 4 opener (P4). Frontier lane: Codex 5.6 Sol MAX (Owner-enabled). Greenfield discretion charter at full strength. The referee is bit-exactness: every move in this rung is a RELOCATION, never a behavior change — replay + RF-1 + identical slot layouts judge everything."
surfaces: ["crates/simthing-core/src", "crates/simthing-kernel/src", "crates/simthing-gpu/src", "crates/simthing-driver/src", "crates/simthing-driver/tests", "scripts/ci", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/tests"]
forbidden: ["ANY behavior/value change (bit-exact replay + RF-1 + slot-layout identity judge; referees unedited)", "WGSL shader semantic edits (structural Rust moves in kernel/driver are in scope; shaders are not)", "new ColumnIndex mints outside the 0.1 doors (this rung must SHRINK raw-door reliance, never grow it)", "editing ANY existing test", "a second allocator or layout authority"]
required_checks: ["cargo build --workspace", "full simthing-driver battery on live GPU", "doctrine-scan", "orientation-check", "doc-budget", "clearance"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "any-replay-or-value-divergence (a move changed behavior — STOP, never patch the referee)", "slot-derivation-cannot-reproduce-an-existing-layout", "a-split-requires-a-behavior-change (0.2 ruling gap — DA-route)"]
---
## BUILD
- P4 object semantics (ANCHOR-ACK core-0087): slot identity and child-row
  allocation become derivations OF THE OBJECT MODEL. A SimThing knows how it
  slots into its parent and how its child rows are allocated relative to it:
  formalize this as typed core semantics (tree -> slot layout via the role
  pathway) that the existing allocator consumes. The tree is the single layout
  authority; the derivation must reproduce every existing layout EXACTLY
  (canonical TP + all fixtures: identical slot assignments pre/post).
- The two 0.2-ruled structural splits (moves, not rewrites):
  (a) `reduction.rs`: extract the `cpu_reduce_oracle*` family into the kernel
  cpu-oracle home — the file becomes pure compile-plan;
  (b) `world_state.rs`: move `encode_rule` / `build_governed_pairs` into the
  single encode/decode boundary module — the file becomes pure executed.
- Establish THE single WGSL encode/decode boundary module: the one fenced
  place where typed plan values drop to raw u32 for GPU upload (and return).
  4.2 types everything up to this frontier; name and doc-fence it accordingly
  (0.1 door doc-comment style: family + promotion blocker).
- Census co-evolution: the 0.2 execution-status census follows the moved code
  (both mixed_ruled rows resolve to clean primaries; totals re-verify; board
  render updates through the generator).
- Stamp 4.1 + advance posture row to `PLAN-STRUCT-TYPING-0` in-diff; regen
  orientation.
## FENCES
- Relocation-only law: bit-exact replay + RF-1 green with referees UNEDITED;
  slot layouts byte-identical pre/post on canonical + fixtures. Zero new
  public raw-u32 column surfaces; deprecated-`new` count must not rise.
## EXIT-PROOF
- Slot-layout identity proof (canonical + fixtures, derived vs legacy: equal);
  full driver battery green UNMODIFIED on live GPU; both splits landed with
  file purity provable by the census (mixed_ruled=0); boundary module exists,
  doc-fenced, and is the ONLY raw-drop site (grep-proven); workspace build
  green. Stamp + posture advance in-diff per the ritual.
