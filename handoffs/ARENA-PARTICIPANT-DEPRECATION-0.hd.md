---
rung: ARENA-PARTICIPANT-DEPRECATION-0
kind: rung
track: 0.0.8.7
base_sha: 3b2ee9d021a822221cf9a545b4ee601f05675aef
audience: coding
model_tier: std
owner_approved: true
expected_route: DA-RESERVE(unclassified-scope)
owner_notes: "Rung 1.3, last Phase 1 rung. Std lane (Grok, pin -m grok-4.5). This is the P0 FALSIFIER rung: the ArenaParticipant wrapper kind existing was the symptom of participation-by-wiring; its deprecation is the proof of participation-by-derivation. StarSystem/Station disposition exactly."
surfaces: ["crates/simthing-core/src/simthing.rs", "crates/simthing-core/src", "crates/simthing-driver/src", "crates/simthing-driver/tests", "scripts/ci", "docs/design_0_0_8_7_rf_arena_modernization.md"]
forbidden: ["removing the ArenaParticipant variant (compile-compat retained — deprecation, not deletion)", "behavior changes to existing sessions that still carry wrapper nodes (they keep working)", "new authoring paths that mint the kind", "referee edits (RF-1/replay/palma untouched)"]
required_checks: ["cargo build --workspace", "full simthing-driver battery on live GPU", "doctrine-scan", "orientation-check", "doc-budget", "clearance"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "a-production-path-still-REQUIRES-minting-the-kind (means 1.1 derivation has a gap — STOP, DA-route, do not paper over)"]
---
## BUILD
- Deprecate `SimThingKind::ArenaParticipant` with the StarSystem/Station disposition
  (ANCHOR-ACK core-0087): `#[deprecated]` + DA-ruling doc comment citing P0's falsifier
  ("the wrapper kind existing was the symptom of participation-by-wiring; 1.1's derivation
  is the cure; retained only for legacy serialized data / compile-compat — do not author").
- Census + fence: prove no production construction site mints the kind post-1.1 (test/fixture
  constructors may remain, `#[allow(deprecated)]`-annotated); if a production path still
  REQUIRES it, that is a 1.1 derivation gap — STOP and DA-route.
- Doctrine-CI co-evolution rides the PR: if any scan/anchor text names ArenaParticipant as a
  live pattern, re-point it; stamp 1.3 exit-proof cell + advance the posture row to
  `OVERLAY-EFFECT-HOST-ADMISSION-0` (Phase 2) in-diff; regen orientation.
## FENCES
- Deprecation, never deletion: the exhaustive kind_matches / serde arms keep compiling;
  legacy trees still load. Zero behavior deltas: full battery green with referees unedited.
## EXIT-PROOF
- Variant deprecated w/ ruling doc-comment; workspace build green (deprecation warnings on
  test-only constructors are acceptable steering); full driver battery on live GPU green;
  census proves zero production mint sites; grep proves no new authoring path. Stamp +
  posture advance in-diff per the graduation ritual.
