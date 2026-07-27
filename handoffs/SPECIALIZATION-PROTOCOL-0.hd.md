---
rung: SPECIALIZATION-PROTOCOL-0
kind: rung
track: 0.0.8.7
base_sha: d295d1515d7e13c347de19f46f4129c26e567f3f
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(unclassified-scope)
owner_notes: "Rung 3.1, Phase 3 opener (P3). Coder = FABLE (Owner routing 2026-07-27: Codex rate-limited; rung important enough for Fable tokens). Self-authorship disclosed: DA implements under this handoff's fences exactly as a coder would; the orchestrator's verification tier is the independent check. Greenfield discretion charter applies at full strength."
surfaces: ["crates/simthing-core/src", "crates/simthing-driver/src", "crates/simthing-driver/tests", "crates/simthing-clausething/tests", "scripts/ci", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/tests"]
forbidden: ["breaking ANY existing kind code (kinds stay as serialization/authority markers; exhaustive arms compile; serde untouched)", "editing ANY existing test (the compatibility falsifier IS the unmodified suite)", "runtime kind-branching (profiles are admission-time observation/validation; zero tick cost)", "a trait-object/vtable specialist hierarchy (profiles are DATA per P0 lightweight guarantee)", "kernel/GPU/WGSL edits", "new ColumnIndex mints outside the 0.1 doors"]
required_checks: ["cargo build --workspace", "full simthing-driver battery on live GPU", "doctrine-scan", "orientation-check", "doc-budget", "clearance"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "an-existing-corpus-tree-fails-profile-derivation-in-a-way-that-requires-test-edits", "profile-validation-requires-runtime-branching"]
---
## BUILD
- The richer-than-kind SPECIALIZATION PROTOCOL (ANCHOR-ACK core-0087; P0 additive-only law +
  P3 pillar): a specialization is a typed, DATA-DECLARED **profile** — a bundle of
  root-contract usages: required properties/roles, arena/field participations, topology
  expectations (grid coordinate, seat position, root posture), hosted overlay families.
  Profiles live in `simthing-core` as data (registry rows, spec-authorable in 3.2) — never a
  trait hierarchy, never runtime dispatch.
- **Derivation + validation at admission (the 1.1 pattern):** structural conformance is
  DERIVED (a SimThing carrying a grid coordinate + spatial participation conforms to
  `spatial` — observation, zero authoring burden); an explicitly DECLARED profile (new
  optional serde-defaulted field — legacy trees load unchanged) is VALIDATED with spanned
  hard errors (missing required property/role/participation names profile/simthing/gap/span).
  An inspectable `SpecializationReport` (derivation-report pattern) lists per-SimThing
  conformances.
- Three seed profiles minimal in 3.1 (full first-citizen enrichment is 3.2): `spatial`
  (Location contract: grid coordinate + spatial-arena participation), `owner-seat` (Owner
  contract: session-root child + policy/weight hosting), `session-root` (GameSession
  contract). Seeds must be satisfied by the EXISTING corpus as it stands — the TP scenario
  derives all three with zero authoring changes.
- Referee (new tests only): TP hydration derives expected profile sets; a
  declared-but-nonconforming fixture spans at admission; a legacy tree with no declarations
  admits identically (bit-compat proof).
- Doctrine-CI co-evolution rides the PR; stamp 3.1 + advance posture row to
  `FIRST-CITIZEN-SPECIALISTS-0` in-diff; regen orientation.
## FENCES
- Compatibility falsifier: the FULL existing suite passes UNMODIFIED (zero test edits).
  Additive-only: profiles extend, never gate — no existing behavior consults them yet
  (consumers arrive in 3.2+). Admission-time only; a SimThing at rest remains a row.
## EXIT-PROOF
- Full driver battery green UNMODIFIED on live GPU; new referee green (derive + span +
  legacy-compat); workspace build green; zero test edits (diff-proven); profiles as data
  (grep: no new trait objects/dyn in core). Stamp + posture advance in-diff per the ritual.
