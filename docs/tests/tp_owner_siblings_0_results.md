# TP-OWNER-SIBLINGS-0 Results

Status: **PROBATION**.

## Scope

`TP-OWNER-SIBLINGS-0` adds ClauseScript authoring for Terran and Pirate `Owner` SimThings as direct `GameSession` children. The hydrated canonical authority tree is:

```text
Scenario
└── GameSession
    ├── Owner: Terran
    ├── Owner: Pirate
    └── GalaxyMap
```

The legacy scenario-container `root` and embedded-base grid metadata remain intact for existing consumers; the new canonical tree is exposed separately as the owner-sibling authority root.

## Owner Syntax Proof

PASS: `cargo test -p simthing-clausething --test tp_owner_siblings_0 scenario_container_parses_embedded_base_and_owner_blocks -- --nocapture`

The scenario-container parser accepts the embedded `static_galaxy_scenario` block plus `owner = terran { ... }` and `owner = pirate { ... }` blocks through the existing raw parser/hydrator path. Owner declarations carry owner key, display name, archetype, color index, stockpile seed/capacity, and policy/personality/capability profile hooks.

## Tree / Sibling Proof

PASS: `cargo test -p simthing-clausething --test tp_owner_siblings_0 owners_are_direct_gamesession_children -- --nocapture`

PASS: `cargo test -p simthing-clausething --test tp_owner_siblings_0 galaxy_map_remains_gamesession_sibling_not_owner_child -- --nocapture`

Terran and Pirate lower to canonical `Owner` SimThings with unique non-empty owner ids. They are direct `GameSession` children; the canonical GalaxyMap is also a direct `GameSession` child and is not nested under either owner.

## Embedded Base Preservation

PASS: `cargo test -p simthing-clausething --test tp_owner_siblings_0 embedded_base_placements_remain_unchanged_from_base_embed -- --nocapture`

The embedded base placements remain byte-shape equivalent to `TP-BASE-EMBED-0`: the source structural grid still matches the canonical 1500-star artifact, and the scenario-container grid metadata remains the namespaced embedded placements.

## Hard-Error Proofs

PASS: `cargo test -p simthing-clausething --test tp_owner_siblings_0 duplicate_owner_ids_hard_error_with_span -- --nocapture`

PASS: `cargo test -p simthing-clausething --test tp_owner_siblings_0 unsupported_owner_fields_hard_error_with_span -- --nocapture`

Duplicate owner ids hard-error with a span-bearing diagnostic. Unsupported owner fields also hard-error with a span, preventing this rung from silently admitting ownership columns or later Phase 2 content.

## Scenario Roundtrip Proof

PASS: `cargo test -p simthing-clausething --test tp_owner_siblings_0 scenario_roundtrip_preserves_owner_metadata_distinct_from_spatial_parentage -- --nocapture`

Canonical scenario JSON save/reopen preserves owner id, display name, archetype, color, and stockpile metadata while keeping owners childless and spatial authority separate under the GalaxyMap sibling.

## Load-Bearing Validation

Local targeted validation:

```bash
cargo check -p simthing-clausething
cargo check -p simthing-spec
cargo test -p simthing-clausething --test tp_base_embed_0 -- --nocapture
cargo test -p simthing-clausething --test tp_owner_siblings_0 -- --nocapture
bash scripts/ci/gen_digest.sh --check
bash scripts/ci/doctrine_scan.sh
```

## INSPECT / Triage

Local doctrine scan: PASS, failures=0, inspect=0.
Local gen_digest --check: PASS.
Live GitHub Doctrine Scan: pending on PR head.

## Scope Ledger

- Owner SimThings only: Terran and Pirate.
- Owners are direct GameSession children, never spatial parents.
- GalaxyMap remains a GameSession sibling.
- Embedded base placements remain unchanged from `TP-BASE-EMBED-0`.
- No owner columns on systems.
- No ownership assignment or capture.
- No planets, factories, cohorts, fleets, ships, combat, diplomacy, `ai_will_do`, route solver, or pathfinding.
- No runtime/GPU changes.
- No new `AccumulatorRole`.
- No scanner or allowlist edits.
- No new CI workflow.
- No second parser and no third loading path.

## Graduation Routing

CI verdict: PASS-RELIABLE pending live CI

Triage entries: none

Risk class: first owner-as-sibling ClauseScript authoring over embedded TP base

Falsification check: Verify Terran/Pirate owner blocks parse through the existing scenario-container path; verify hydrated owners are direct GameSession children; verify GalaxyMap remains a sibling and not an owner child; verify embedded base placements are unchanged; verify duplicate owner ids and unsupported fields hard-error with spans; verify scenario save/reopen preserves owner metadata distinct from spatial parentage; verify no owner columns, Phase 2+ content, runtime/GPU change, scanner/allowlist edit, new AccumulatorRole, or alternate parser/loading path.

Recommended posture: PROBATION for orchestrator review; do not self-merge this rung.

## Known Gaps / Next

Next active rung after orchestrator clearance is `TP-OWNERSHIP-COLUMNS-0`.
