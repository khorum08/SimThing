# TP-BASE-EMBED-0 Results

Status: **PROBATION** - implementation proof complete; orchestrator review required before merge.

## What Changed

`TP-BASE-EMBED-0` adds scenario-container admission for an embedded `static_galaxy_scenario` block. The outer `.clause` still parses through `parse_raw_document`; the embed is hydrated inside the existing scenario-container front end and carries the producer-owned base provenance separately from the overlay runtime metadata.

## Embedded Base Source

- Source artifact: `crates/simthing-mapeditor/tests/fixtures/tp_base_disc_1500.simthing-scenario.json`
- Source rung: `TP-BASE-DISC-GEN-0`
- Source SHA-256: `aab8b0d2bd229bca620986d1c56ac8f658c0842b432f6131b66ac5bf99266ac0`
- Source bytes: `889808`
- Source seed: `770421`
- Source profile: `disc_1500_connected`
- Source map quality: `PASS`

## Combined Grammar Proof

PASS: `cargo test -p simthing-clausething --test tp_base_embed_0 combined_clause_parses_with_embedded_static_galaxy_scenario -- --nocapture`

The combined scenario-container `.clause` parses through the existing raw neutral-AST path and hydrates one embedded `static_galaxy_scenario` block with 1500 placements and a 300-cell frame.

## Round-Trip Identity Proof

PASS: `cargo test -p simthing-clausething --test tp_base_embed_0 embedded_base_lattice_round_trips_identical_to_canonical_artifact -- --nocapture`

The embedded base preserves the canonical artifact structural grid, scenario id, producer provenance, canonical byte length, and canonical JSON bytes.

## Namespace / Duplicate-Id Proof

PASS: `cargo test -p simthing-clausething --test tp_base_embed_0 base_ids_are_namespaced_into_overlay_location_targets -- --nocapture`

PASS: `cargo test -p simthing-clausething --test tp_base_embed_0 duplicate_namespaced_base_ids_hard_error_with_span -- --nocapture`

Every embedded base placement is exposed as a namespaced overlay location-target (`tp_base::...`). Reusing the same namespace for the same base hard-errors with a span-bearing duplicate location-target diagnostic.

## Provenance / Runtime Ownership Proof

PASS: `cargo test -p simthing-clausething --test tp_base_embed_0 producer_provenance_remains_separate_from_overlay_runtime_owner -- --nocapture`

Producer provenance remains attached to the embedded base (`MapGeneratorLibrary`, seed `770421`). The scenario-container pack owns the runtime-facing `game_mode.id` and metadata; base generator params are not copied into overlay runtime metadata.

## Load-Bearing Validation

Local targeted validation:

```bash
cargo check -p simthing-clausething
cargo check -p simthing-mapeditor
cargo check -p simthing-spec
cargo test -p simthing-mapeditor --test tp_base_disc_gen
cargo test -p simthing-clausething --test tp_base_embed_0 -- --nocapture
bash scripts/ci/gen_digest.sh --check
bash scripts/ci/doctrine_scan.sh
```

## INSPECT / Triage

Local doctrine scan: PASS, failures=0, inspect=0.
Local gen_digest --check: PASS.
Live PR-head CI: pending after push.

## Scope Ledger

- Grammar/admission embedding only.
- No Terran owner or Pirate owner.
- No ownership columns.
- No planets, factories, cohorts, fleets, ships, combat, diplomacy, `ai_will_do`, route solver, or pathfinding.
- No runtime/GPU changes.
- No new `AccumulatorRole`.
- No scanner or allowlist edits.
- No new CI workflow.
- No second parser and no third loading path.

## Graduation Routing

Graduation routing (for orchestrator review - why PROBATION, not COMPLETE):

CI verdict: local PASS; live PR-head CI pending after push

Triage entries: none locally

Risk class: combined-document grammar + base-provenance binding

Falsification check: Verify combined `.clause` parses through the existing neutral-AST parser; verify embedded base lattice round-trips identical to TP-BASE-DISC-GEN-0; verify namespaced ids become overlay location-targets; verify duplicate ids hard-error with a span; verify producer provenance remains distinct from overlay runtime; verify no Phase 2+ content, runtime/GPU change, new AccumulatorRole, scanner/allowlist edit, or second parser/third loading path.

Recommended posture: deep - this is the grammar path every later Terran/Pirate overlay rung will consume.

## Known Gaps / Next

Do not self-merge. Leave PR in PROBATION for orchestrator review.
