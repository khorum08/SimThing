# TP-OWNERSHIP-COLUMNS-0 Results

Status: **DONE — DA-APPROVED (2026-07-02, executive DA full review; PROBATION cleared, merge of PR #1078 authorized)**.

## Scope

`TP-OWNERSHIP-COLUMNS-0` assigns deterministic ownership columns to the embedded canonical 1500-star Terran-Pirate base disc:

- 200 Terran systems.
- 50 Pirate systems adjacent to the Terran volume.
- 1250 neutral systems.

Ownership is authored as a column on existing star-system gridcell SimThings. It is not spatial parenting: Terran and Pirate remain direct `GameSession` sibling `Owner` SimThings, and all star-system gridcells remain under the `GalaxyMap` sibling.

## Authoring Syntax

PASS: `cargo test -p simthing-clausething --test tp_ownership_columns_0 embedded_base_owner_siblings_and_ownership_volumes_parse -- --nocapture`

The scenario-container grammar accepts `ownership_volume` blocks through the existing raw parser and hydrator path:

```text
ownership_volume = terran_core {
    owner = "terran"
    count = 200
    selection = chebyshev_contiguous
    seed = 770421
    anchor_row = 199
    anchor_col = 80
}

ownership_volume = pirate_border {
    owner = "pirate"
    count = 50
    selection = chebyshev_contiguous
    adjacent_to = "terran_core"
    seed = 770421
}
```

Selection is authoring-time over integer `(row,col)` grid coordinates using Chebyshev distance. It does not use Euclidean or floating-point ownership authority.

## Count / Non-Overlap Proof

PASS: `cargo test -p simthing-clausething --test tp_ownership_columns_0 ownership_counts_are_200_50_1250_and_non_overlapping -- --nocapture`

The hydrated ownership volumes assign exactly 200 Terran systems and 50 Pirate systems. The canonical embedded base has 1500 star placements, so 1250 systems remain neutral. The proof verifies no star-system target id appears in more than one ownership volume.

## Contiguity / Adjacency Proof

PASS: `cargo test -p simthing-clausething --test tp_ownership_columns_0 terran_and_pirate_volumes_are_chebyshev_contiguous_and_adjacent -- --nocapture`

The Terran core is the deterministic Chebyshev-neighborhood prefix around anchor `(row=199, col=80)`. The Pirate border is the deterministic nearest unassigned Chebyshev-border prefix adjacent to the Terran core. The proof verifies both volume selections and exact Pirate/Terran Chebyshev adjacency.

## Owner-Ref / No-Reparenting Proof

PASS: `cargo test -p simthing-clausething --test tp_ownership_columns_0 owner_references_resolve_to_gamesession_sibling_owners -- --nocapture`

PASS: `cargo test -p simthing-clausething --test tp_ownership_columns_0 owned_gridcells_remain_under_galaxymap_not_owners -- --nocapture`

PASS: `cargo test -p simthing-clausething --test tp_ownership_columns_0 galaxymap_remains_gamesession_sibling -- --nocapture`

Every owned gridcell carries `OWNER_FLOW_OWNER_REF_PROPERTY_ID` pointing to an existing `GameSession` sibling owner key. No gridcell is reparented under Terran or Pirate; all 1500 embedded star-system gridcells remain direct GalaxyMap descendants.

## Capture-As-Column-Flip Proof

PASS: `cargo test -p simthing-clausething --test tp_ownership_columns_0 capture_as_column_flip_preserves_id_parentage_and_children -- --nocapture`

The unit proof flips one Terran-owned gridcell's owner column to Pirate and verifies that only the owner ref changes. The SimThing id, kind, parentage position, and children remain unchanged.

## Hard-Error Proofs

PASS: `cargo test -p simthing-clausething --test tp_ownership_columns_0 unknown_owner_reference_hard_errors_with_span -- --nocapture`

PASS: `cargo test -p simthing-clausething --test tp_ownership_columns_0 overlapping_ownership_selections_hard_error_with_span -- --nocapture`

Unknown owner references hard-error with span-bearing diagnostics. Overlapping ownership volumes also hard-error with spans, preventing silent multi-owner systems.

## Load-Bearing Validation

Local targeted validation:

```bash
cargo check -p simthing-clausething
cargo check -p simthing-spec
cargo test -p simthing-clausething --test tp_owner_siblings_0 -- --nocapture
cargo test -p simthing-clausething --test tp_ownership_columns_0 -- --nocapture
bash scripts/ci/gen_digest.sh --check
bash scripts/ci/doctrine_scan.sh
```

## INSPECT / Triage

Local doctrine scan: PASS, failures=0, inspect=0.
Local gen_digest --check: PASS.
Live GitHub Doctrine Scan: PASS on PR #1078.

## Scope Ledger

- Ownership columns only on star-system gridcells.
- Terran volume count: 200.
- Pirate volume count: 50.
- Neutral count: 1250.
- Terran and Pirate owner refs resolve to sibling `Owner` SimThings.
- GalaxyMap remains a GameSession sibling.
- Star-system gridcells remain under GalaxyMap.
- Capture is represented as an owner-column flip without identity or parentage mutation.
- No planets, surfaces, factories, cohorts, fleets, ships, combat, diplomacy, AI, pathfinding, Movement-Front execution, runtime/GPU changes, new `AccumulatorRole`, scanner/allowlist edits, new CI workflow, second parser, or owner-as-parent model.

## Graduation Routing

CI verdict: PASS-RELIABLE; PROBATION pending orchestrator review.

Triage entries: none locally.

Risk class: ownership-column authoring over canonical embedded TP base

Falsification check: Verify combined embedded base plus owner siblings plus ownership volumes parse; verify exactly 200 Terran, 50 Pirate, and 1250 neutral systems; verify Terran/Pirate Chebyshev selection and adjacency; verify owner refs point to existing sibling owners; verify systems remain spatial children of GalaxyMap; verify capture-as-column-flip changes only the owner ref; verify no Phase 2 child content, runtime/GPU change, scanner/allowlist edit, new AccumulatorRole, or owner-as-parent semantics.

Recommended posture: DONE — DA-APPROVED (2026-07-02). Executive DA re-ran the full 9-test suite (PASS), verified the Chebyshev selection is integer-only in `hydrate_scenario.rs`, and confirmed live Doctrine Scan green on PR #1078 (run 28563325202). Merge authorized under the §0.9.5 merge-hold rule.

## Known Gaps / Next

Next active rung after clearance is `TP-PLANET-SURFACE-PAYLOAD-0`.
