# TEST-PARE-STANDARD-DA-0 Results

## Status

**PROBATION** - boundary-keyed Track D standard adopted after `DA-RULING: ADMISSION-BOUNDARY-COLLAPSE`. This rung performs no deletions and no crate edits; it replaces the old source-file-family ledger as deletion authority for future waves.

## DA ruling encoded

The DA ruling is encoded as:

```text
DA-RULING: ADMISSION-BOUNDARY-COLLAPSE
```

Admission-adjacent, hygiene-theater, and usecase-superseded rows now carry a retention burden. Historical rejection examples are not inherently valuable; rows survive only when they name a unique invariant not owned elsewhere.

## What changed

- Added `scripts/ci/test_pare_boundaries.tsv`, the owning-boundary table.
- Added `scripts/ci/test_pare_boundary_rows.tsv`, the row-to-boundary mapping for all live inventory rows plus historical PARED rows.
- Added `scripts/ci/test_pare_boundary_check.sh`.
- Updated `scripts/ci/test_inventory.tsv` so every live row names its owning boundary in `superseding_boundary`.
- Updated `scripts/ci/test_inventory_check.sh` to identify the boundary rows file as the current authoritative ledger and the old audit file as legacy.
- Updated Track D design/operator/evidence docs and marked D1 source-family audit authority as superseded.

## Boundary-keyed schema

`test_pare_boundaries.tsv` is keyed by `boundary_id` with tier, owner, boundary kind, owning artifact, representative requirement, representative, retirement policy, and notes.

`test_pare_boundary_rows.tsv` maps each row to `boundary_id`, `boundary_tier`, recommended disposition, representative/consolidation target, promotion requirement, confidence, and note.

## Tier rules

- Tier 1 type/seal owner: illegal state is uncompilable; runtime duplicate rows are DELETE candidates.
- Tier 2 admission hard-error owner: one negative representative per boundary; variants collapse to the representative.
- Tier 3 doctrine scan owner: scanner self-test is the representative; source tests duplicating scanned invariants are DELETE candidates.
- Tier 4 classifier consolidation: variants become one table-driven test.
- Tier 5 behavior regression: retained unless a future stronger owner is named.
- Tier 6 promotion required: real invariant with no current owner; rustification queue, not a shield.
- Tier 7 never-pare: terminal proof classes and active live rung suites.

## Consolidation exit

Classifier-input families consolidate to one table-driven test when variants exercise distinct classifier paths. This preserves meaningful input coverage without keeping N independent tests. Current Tier 4 candidates are hygiene/classifier consolidation rows, not immediate deletes.

## Never-pare set

Protected rows include compile_fail / trybuild seal proofs, CPU-oracle parity tests, golden-byte and deterministic artifact tests, doc-named invariant tests, STEAD-required rows, `custom_layout_ethics_axis`, escaped-bug regressions, and active TP live rung suites while that product track remains open.

## Promotion rider

Rows with a real invariant but no type/admission/scan/oracle/golden/required owner are `PROMOTION_REQUIRED`. The row must name the missing boundary to promote; this is not an indefinite audit blocker.

## Module-marker expansion

`cfg_test_mod::...` module markers no longer remain generic `AUDIT-BLOCKED` rows. The boundary ledger maps 134 module-marker rows to child inventory already present, keeps 5 never-pare module markers protected, and marks 4 rows `PROMOTION_REQUIRED` because mechanical expansion found no child test row in the same file.

## Reclassified row counts

```text
total live inventory rows:       6296
boundary-row mappings:          6300
historical PARED rows mapped:   4
rows with owning boundary:      6296
rows missing owning boundary:   0
Tier 1 DELETE candidates:       5
Tier 2 COLLAPSE candidates:     779
Tier 3 DELETE candidates:       50
Tier 4 CONSOLIDATE candidates:  141
NEVER_PARE rows:                899
PROMOTION_REQUIRED rows:        138
module-marker rows expanded:    134
module-marker rows remaining:   0 generic blockers
active TP rows protected:       81
```

## Tier 1 DELETE candidates

Tier 1 candidates are AS/newtype/seal-owned runtime duplicates. Current candidates:

- `simthing-driver::owner_silo_runtime_writeback_compile_rejects_unknown_owner_ref`
- `simthing-spec::planet_child_rf_empty_owner_ref_still_rejects`
- `simthing-spec::planet_child_rf_unknown_owner_ref_still_rejects`
- `simthing-spec::owner_silo_writeback_inputs_reject_unknown_owner_ref`
- `simthing-spec::owner_silo_unknown_owner_ref_rejected`

## Tier 2 COLLAPSE candidates

Tier 2 has 779 collapse candidates after selecting one live representative per admission hard-error boundary. Largest crate surfaces:

- `simthing-spec`: 401
- `simthing-driver`: 181
- `simthing-mapeditor`: 54
- `simthing-clausething`: 41
- `simthing-mapgenerator`: 31

## Tier 3 DELETE candidates

Tier 3 scan-owned delete candidates: 50.

- `B-T3-SEMANTIC-FREE-SCAN`: 36
- `B-T3-SURFACE-SCAN`: 13
- `B-T3-FORGE-UNSAFE-SCAN`: 1

## Tier 4 CONSOLIDATE candidates

Tier 4 consolidation candidates: 141.

All current Tier 4 rows map to `B-T4-HYGIENE-THEATER-CONSOLIDATION` with table-driven consolidation targets.

## Promotion-required rows

Promotion-required rows: 138. These are primarily module-marker/unknown rows that must be expanded, mapped, or promoted to a concrete owner before any deletion wave touches them.

## Next deletion waves

Recommended next wave: `TEST-PARE-SPEC-0`.

Reason: `simthing-spec` has the largest cohesive Tier 2 admission hard-error collapse surface at 401 candidates. Under the new cadence, the wave should process complete named boundary families, not a row quota.

## Validation

Commands:

```bash
bash scripts/ci/test_inventory_check.sh
bash scripts/ci/test_pare_boundary_check.sh
bash scripts/ci/doctrine_scan.sh
bash scripts/ci/gen_digest.sh --check
```

Result:

```text
TEST-INVENTORY-CHECK-VERDICT: PASS
TEST-PARE-BOUNDARY-CHECK-VERDICT: PASS
DOCTRINE-SCAN-VERDICT: PASS failures=0 inspect=0
gen_digest --check: PASS
```

## Scope Ledger

- No test deletions.
- No crate source edits.
- No crate test edits.
- No workflow edits.
- No scanner/allowlist edits.
- No cargo tests.
- Old `scripts/ci/test_pare_audit.tsv` remains historical context only.

## Graduation routing

Recommended status: **PROBATION**.

Why: this replaces the deletion authority for all future Track D waves. DA/orchestrator should verify boundary ownership, tier rules, consolidation exit, promotion-required rows, module-marker handling, TP live-suite protection, no deletion, and validation.

## Known gaps / next

- Future waves must process every row in their named boundary families to terminal disposition.
- `TEST-PARE-SPEC-0` is the recommended first material reduction wave.
- Weekly scheduled sentinel remains sentinel-core only until material reduction lands.
- Full quarantined battery remains workflow_dispatch-only.
