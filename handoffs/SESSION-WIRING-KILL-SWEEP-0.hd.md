---
rung: SESSION-WIRING-KILL-SWEEP-0
kind: rung
track: 0.0.8.7
base_sha: 488ad2c9696cbec4a52722eb5a635349dd61e3f5
audience: coding
model_tier: std
owner_approved: true
expected_route: DA-RESERVE(unclassified-scope)
owner_notes: "Rung 1.2, Std lane (Grok, pin -m grok-4.5). Two mechanical sweeps riding 1.1's precedent: opt-in toggle kill + calendar-vocabulary rename. Behavioral identity is again the referee — existing batteries unedited and green."
surfaces: ["crates/simthing-core/src", "crates/simthing-driver/src", "crates/simthing-mapeditor/src", "crates/simthing-driver/tests", "scripts/ci", "docs/design_0_0_8_7_rf_arena_modernization.md"]
forbidden: ["editing referee tests (existing battery assertions are the judge — unedited)", "changing any resolved field/flow value (min_plus derivation must reproduce prior enabled-run outputs exactly)", "removing authored opt-outs (DefaultDisabled family stays; only mandatory TOGGLES die)", "new ColumnIndex mints outside the 0.1 doors", "serialization breakage (spawn-day field renames must keep serde compat via alias)"]
required_checks: ["cargo build --workspace", "full simthing-driver battery on live GPU", "cargo test -p simthing-core", "doctrine-scan", "orientation-check", "doc-budget", "clearance"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "a-toggle-kill-changes-any-existing-test-output", "serde-compat-cannot-be-preserved"]
---
## BUILD
- **Toggle kill (P1 seam collapse, 1.1 pattern):** `min_plus_traversal_field`'s
  `enabled()/enable()/disable()` session wiring dies — a traversal field runs iff its band
  registration is ADMITTED (derived-at-admission; the registration IS the intent; absence IS
  the off state). Sweep the remaining constitutional-path opt-in toggles the same way; authored
  opt-outs (`DefaultDisabled` family) are NOT toggles and stay. Grep-proof the kill: no
  `enable()/disable()` mutators remain on constitutional paths.
- **Calendar rename (P0 generation ruling, Phase 1 ride-along):** `simthing-core` sheds
  calendar vocabulary — `evaluate.rs` `FieldSnapshot.day` + `evaluate(root, day)` and the
  SimThing spawn-day field rename to generation terms; serde compat preserved via
  `#[serde(alias = "day")]`-style aliases so existing serialized trees load. Kernel/gpu are
  already calendar-free; driver/sim/feeder/clausething keep their day language (front-end
  cadence binding — NOT this rung's scope).
- Doctrine-CI co-evolution rides the PR (scans/anchors touching renamed symbols; §3b stamp
  + orientation regen in-diff).
## FENCES
- Behavioral identity: existing batteries UNEDITED and green (workspace + driver GPU + core);
  `mapgen_palma` outputs identical pre/post the toggle kill. Any divergence = STOP.
- Zero semantic changes: renames are renames; toggle-kill only relocates the on/off decision
  from a runtime mutator to admission presence.
## EXIT-PROOF
- Grep-proof: constitutional-path toggle mutators = 0; core day-refs = 0 (comments included).
- Full driver battery + core battery green with zero test edits; serde round-trip fixture
  proves old serialized `day` fields still load. Stamp 1.2 in-diff; regen orientation.
