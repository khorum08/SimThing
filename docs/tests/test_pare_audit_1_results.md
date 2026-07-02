# TEST-PARE-AUDIT-1 Results

## Status

**SUPERSEDED** - D1 created the old source-file-family paring ledger. `TEST-PARE-STANDARD-DA-0` replaces it as deletion authority with a boundary-keyed ledger after `DA-RULING: ADMISSION-BOUNDARY-COLLAPSE`.

Prerequisite satisfied: PR #1084 (`TEST-PARE-INVENTORY-0`) merged to `master` at `0aa10bd0b4f954defeef00fe5a979e8598d8f23f` before this branch was opened.

## What Changed

- Added `scripts/ci/test_pare_audit.tsv`, the D1 audit ledger for every D0 candidate row.
- Updated `scripts/ci/test_inventory.tsv` candidate notes to point at the D1 audit ledger.
- Extended `scripts/ci/test_inventory_check.sh` to validate the audit ledger, candidate coverage, PARE/COLLAPSE boundary fields, AUDIT-BLOCKED reasons, never-pare protection, and zero crate source/test diffs.
- Updated Track D design/operator/evidence docs.

## Audit Method

Supersession note: this method is retained as historical context only. The old `PARE`, `COLLAPSE`, and `AUDIT-BLOCKED` classifications do not authorize deletion after the DA ruling. Current Track D deletion authority is:

- `scripts/ci/test_pare_boundaries.tsv`
- `scripts/ci/test_pare_boundary_rows.tsv`
- `scripts/ci/test_pare_boundary_check.sh`

Under the new standard, admission-adjacent, hygiene-theater, and usecase-superseded rows are presumptive retire/collapse/consolidate unless the row names a unique invariant not enforced by a type/seal boundary, admission hard-error, doctrine scan, compile-fail proof, oracle, golden artifact, required invariant, or active live rung suite.

D1 audits the D0 candidate classes:

- `admission-adjacent`
- `hygiene-theater`
- `usecase-superseded`
- `unknown`
- `duplicate-battery`

The audit allows paring when a row is superseded by, dominated by, or reasonably adjacent to an admission hard-error, type boundary, reliable doctrine scan, compile-fail/trybuild seal proof, CPU oracle/parity proof, golden/deterministic proof, required invariant/STEAD test, or later production rung. Every PARE/COLLAPSE row names its boundary. COLLAPSE rows also name the representative row to keep.

Never-pare remains categorical KEEP: `compile_fail`, trybuild, seal-proof, oracle/parity, golden-byte, invariant-required, STEAD-required, and `custom_layout_ethics_axis`.

## Candidate Queue Coverage

All D0 candidate rows are accounted for:

- Candidate rows: 1,226
- Audit rows: 1,226
- Missing audit rows: 0
- Extra audit rows: 0

By original class:

- `admission-adjacent`: 891
- `hygiene-theater`: 147
- `usecase-superseded`: 50
- `unknown`: 138
- `duplicate-battery`: 0

## PARE Summary

PARE-plan rows: **50**.

All PARE rows come from `usecase-superseded` and name the same deletion basis: post-0.0.8.4 admission substrate and successor runtime paths make the legacy/sunset use case impossible or irrelevant. These are not deleted in D1; they become per-crate deletion candidates.

PARE by crate:

- `simthing-spec`: 19
- `simthing-sim`: 15
- `simthing-driver`: 8
- `simthing-mapeditor`: 6
- `simthing-kernel`: 2

## COLLAPSE Summary

COLLAPSE-plan rows: **542**.

By class:

- `admission-adjacent`: 503
- `hygiene-theater`: 39

By crate:

- `simthing-spec`: 415
- `simthing-clausething`: 49
- `simthing-mapgenerator`: 31
- `simthing-mapeditor`: 20
- `simthing-tools`: 16
- `simthing-workshop`: 11

Every COLLAPSE row names `representative_to_keep`.

## KEEP Summary

D1 does not reclassify categorical never-pare rows in the audit ledger. The checker verifies the D0 inventory still marks never-pare rows KEEP. The protected KEEP set still includes compile-fail, trybuild, seal-proof, oracle/parity, golden-byte, invariant-required, STEAD-required, and `custom_layout_ethics_axis`.

## AUDIT-BLOCKED Summary

AUDIT-BLOCKED rows: **634**.

By original class:

- `admission-adjacent`: 388
- `hygiene-theater`: 108
- `unknown`: 138

By crate:

- `simthing-driver`: 318
- `simthing-mapeditor`: 95
- `simthing-sim`: 57
- `simthing-kernel`: 42
- `simthing-spec`: 36
- `simthing-core`: 28
- `simthing-gpu`: 24
- `simthing-clausething`: 8
- `simthing-feeder`: 8
- `simthing-mapgenerator`: 7
- `simthing-tools`: 6
- `simthing-workshop`: 5

Blocked rows are not punts: each row carries a reason. Runtime/driver/kernel/gpu rows are blocked for explicit DA/per-crate review; unknown rows are blocked because D0 only saw a `#[cfg(test)]` module marker and not row-level intent.

## Unknown Reduction

Unknown D0 rows: 138.

Unknown D1 rows: 0.

All 138 unknown rows are now `AUDIT-BLOCKED` with the `module-marker-blocked` audit class and a reason: the row represents a test module marker, so deletion/collapse requires source-level expansion before a paring decision.

Boundary-standard correction: `AUDIT-BLOCKED` is a queue state, not a shield. `TEST-PARE-STANDARD-DA-0` maps module markers with child rows already inventoried and marks the remaining mechanically unexpanded markers `PROMOTION_REQUIRED` with a specific missing-boundary reason.

## Boundary Evidence

Boundary families used in `scripts/ci/test_pare_audit.tsv`:

- Admission hard-error boundary in the same parser/spec surface, with one representative hard-error row retained.
- Representative hygiene/perf boundary retained in the same file; broad perf/soak batteries stay owner-deep/quarantined rather than PR-default proof.
- Production rung/sunset boundary: post-0.0.8.4 admission substrate and successor runtime paths make legacy use cases impossible or irrelevant.

The audit deliberately blocks driver/kernel/sim/gpu rows unless a later per-crate rung has explicit DA review.

## Recommended Deletion Waves

- Wave 1: scripts/ci fixture duplicate/hygiene rows - no D1 candidate rows found.
- Wave 2: clausething admission duplicate batteries - 57 rows, 49 COLLAPSE-plan rows and 8 blocked singleton/runtime-adjacent rows.
- Wave 3: spec admission duplicate batteries - 470 rows.
- Wave 4: mapgenerator/mapeditor/tools/workshop superseded use-case and hygiene rows - 197 rows.
- Wave 5: kernel/sim/gpu/driver only after explicit DA review - 502 rows.

## First Per-Crate Paring PR

Recommended first PR: `TEST-PARE-CLAUSETHING-0`.

Scope: collapse the 49 `simthing-clausething` Wave 2 COLLAPSE rows after targeted source review, keeping the representatives named in `scripts/ci/test_pare_audit.tsv`.

Exact row IDs are the audit rows with:

```text
deletion_wave == "Wave 2: clausething admission duplicate batteries"
audit_verdict starts with "COLLAPSE("
```

Representative groups:

- `crates/simthing-clausething/tests/bh3_authoring_parse.rs`: collapse 2 -> 1, keep `bh3_authoring_rejects_invalid_chi_literal`.
- `crates/simthing-clausething/tests/ct_0c_expansion.rs`: collapse 2 -> 1, keep `missing_parameter_is_a_spanned_diagnostic`.
- `crates/simthing-clausething/tests/ct_2c_category_economy.rs`: collapse 2 -> 1, keep `economic_key_decoder_rejects_ambiguity`.
- `crates/simthing-clausething/tests/ct_scenario_container.rs`: collapse 28 -> 1, keep `custom_or_deprecated_child_kinds_are_rejected`.
- `crates/simthing-clausething/tests/mapgen_links.rs`: collapse 8 -> 1, keep `expansion_report_declares_caps_and_rejection_counts`.
- `crates/simthing-clausething/tests/mapgenerator_cli_pr6_generated_hyperlanes_lower.rs`: collapse 3 -> 1, keep `generated_hyperlane_scenario_rejects_duplicate_link_without_widening`.
- `crates/simthing-clausething/tests/tp_owner_siblings_0.rs`: collapse 2 -> 1, keep `duplicate_owner_ids_hard_error_with_span`.
- `crates/simthing-clausething/tests/tp_planet_surface_payload_0.rs`: collapse 2 -> 1, keep `invalid_owned_payload_counts_hard_error_with_span`.

## Self-Check

Command:

```bash
bash scripts/ci/test_inventory_check.sh
```

Result:

```text
TEST-INVENTORY-CHECK REPORT
  rows: 6300
  discovered: 6300
  missing: 0
  extra: 0
  inspect: none
TEST-PARE-AUDIT REPORT
  audit rows: 1226
  candidate rows: 1226
  missing audit rows: 0
  extra audit rows: 0
TEST-INVENTORY-CHECK-VERDICT: PASS
```

## Scope Ledger

- Zero crate source edits.
- Zero crate test source edits.
- Zero test deletions.
- Zero workflow edits.
- Zero scanner/allowlist edits.
- Zero runtime/GPU edits.
- Zero branch-protection, auto-merge, or CODEOWNERS edits.
- No all-crates cargo tests and no bare/full-crate `cargo test -p <crate>` runs.
- Owner-deep full batteries remain quarantined artillery.
- Smoke PASS remains mechanics-only, not seal-proof.

## Graduation Routing

Recommended status: **SUPERSEDED BY TEST-PARE-STANDARD-DA-0**.

Why: D1 created the first deletion/collapse plan, but the DA ruling replaces its source-family standard with boundary ownership, tiered dispositions, a consolidation exit, and promotion-required rows.

## Known Gaps / Next

- `TEST-PARE-CLAUSETHING-0`: first proposed per-crate collapse PR.
- Expand `module-marker-blocked` rows before any unknown-derived deletion/collapse.
- Driver/kernel/sim/gpu rows stay blocked until explicit DA review.
- Owner-deep cadence remains deferred until actual per-crate paring data exists and Track B profile evidence is reviewed.
