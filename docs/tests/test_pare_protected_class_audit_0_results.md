# TEST-PARE-PROTECTED-CLASS-AUDIT-0 Results

## Status

PROBATION. Audit implemented and pushed for DA/orchestrator review. This PR deletes zero tests and does not authorize self-merge.

## #1101 closeout

`GHA-PROOF-SEAL-0` is DONE and merged as #1101. Merge commit: `e49c8a258e4bd58d9c78b6c82b698cd5650dbaca`. Head: `317aba88f649a027fcd2c9997b182a7c27005cce`. The enforced rule is that non-owner-deep GitHub Doctrine Exec profiles cannot contain Atlas/Bevy/GPU/desktop/mapeditor/tools runtime proof tokens. Prove path: `bash scripts/ci/doctrine_exec_profile_lint.sh --prove-gha-proof-seal`.

## Current inventory baseline

- inventory rows: 5332
- protected audit rows reviewed: 5079
- KEEP permanent-residue/doc-named rows reviewed: 829
- AUDIT judgment-class rows reviewed: 4250

## Why-chain legend

See `docs/tests/test_residue_class_legend.md`. The audit binds every permanent-residue class to an owning doctrine rather than accepting a class label as a magic shield.

## Judgment-note rule

`scripts/ci/test_inventory_check.sh` now rejects any future KEEP row in `behavior-regression` or `escaped-bug` unless its note starts with `catches: ` and names a specific regression or bug. Boilerplate such as `catches: behavior regression`, `catches: escaped bug`, `catches: important coverage`, `permanent-residue:behavior-regression`, and `regression test` is rejected. Prove path: `bash scripts/ci/test_inventory_check.sh --prove-judgment-note-rule`.

## Protected rows audited

| Class | Rows |
|---|---:|
| `behavior-regression` | 4250 |
| `oracle-parity` | 484 |
| `stead-required` | 121 |
| `golden-byte` | 113 |
| `seal-proof` | 110 |
| `invariant-required` | 1 |

| Truth verdict | Rows |
|---|---:|
| `OUT_OF_SCOPE` | 4250 |
| `TRUE_MEMBER` | 829 |

| Proposed next action | Rows |
|---|---:|
| `RECLASSIFY_TO_AUDIT` | 4250 |
| `KEEP` | 829 |

## Coverage maps

- `docs/tests/test_pare_protected_class_audit_0_review.tsv`: all protected-surface rows and proposed disposition.
- `docs/tests/protected_class_oracle_parity_coverage.tsv`: 484 oracle-parity survivor rows.
- `docs/tests/protected_class_seal_proof_coverage.tsv`: 110 seal-proof survivor rows.
- `docs/tests/protected_class_golden_byte_coverage.tsv`: 113 golden-byte survivor rows.
- `docs/tests/protected_class_stead_required_coverage.tsv`: 121 STEAD-required survivor rows.
- `docs/tests/protected_class_doc_named_coverage.tsv`: 1 doc-named invariant survivor rows.
- `docs/tests/protected_class_judgment_keep_audit.tsv`: 4250 judgment-class rows; all current rows are AUDIT, not KEEP shields.

## Oracle-parity findings

All current `permanent-residue:oracle-parity` KEEP rows remain TRUE_MEMBER survivor claims. GHA proof must not run local-owner-deep Atlas/GPU/desktop rows; those rows are marked `proof_mode=local-owner-deep` where their names or paths require it.

## Seal-proof findings

All current `permanent-residue:seal-proof` KEEP rows remain TRUE_MEMBER survivor claims. Compile-fail/probe fixture rows remain canonical sealed-boundary proof and are never deletion candidates.

## Golden-byte findings

All current `permanent-residue:golden-byte` KEEP rows remain TRUE_MEMBER survivor claims for deterministic replay, canonical bytes, or equivalent exact-output surfaces.

## STEAD-required findings

All current `permanent-residue:stead-required` KEEP rows remain TRUE_MEMBER survivor claims tied to `docs/stead_spatial_contract.md` section 8 mapgen contract surfaces.

## Doc-named findings

`custom_layout_ethics_axis` remains the sole current doc-named invariant survivor and is tied to `docs/invariants.md`.

## Judgment-class findings

The ledger currently has no KEEP `behavior-regression` or `escaped-bug` rows. The 4,250 current behavior-regression rows are AUDIT rows and therefore do not create permanent-residue shields. Future KEEP judgment rows must use the new `catches:` note rule.

## Reclassifications

No inventory reclassification is performed in this PR. The audit records 4,250 judgment-class AUDIT rows as `OUT_OF_SCOPE` for protected-survivor membership because they are not KEEP rows.

## Deletion queue for TEST-PARE-PROTECTED-RESIDUE-0

No protected KEEP row is queued for deletion by this audit. Later `TEST-PARE-PROTECTED-RESIDUE-0` work may process non-KEEP AUDIT rows through their boundary owners, but this PR does not delete or relabel them.

## Necessary/cited/dependency floor

No crate/test/product code was touched. Executable changes are limited to the inventory checker's judgment-note guard/prove mode, the generated protected-class audit helper, and a Python-version compatibility fix in `test_edit_scope_check.sh` that replaces the Python 3.13-only `PurePosixPath.full_match` call with an equivalent full-path `fnmatchcase` fallback.

## GHA proof-seal compliance

This PR adds no Doctrine Exec profile and no GHA command that runs Atlas, Bevy, GPU, desktop, mapeditor/tools runtime, WGPU, X11/Wayland, `apt-get`, workspace tests, or all-crate cargo tests.

## Validation

Local Git Bash validation PASS:

- `bash scripts/ci/test_inventory_check.sh --prove-judgment-note-rule` - PASS
- `bash scripts/ci/test_inventory_check.sh` - PASS
- `bash scripts/ci/test_pare_boundary_check.sh` - PASS
- `bash scripts/ci/test_inventory_drift_check.sh` - PASS
- `bash scripts/ci/test_edit_scope_check.sh --prove` - PASS
- `bash scripts/ci/doctrine_exec_profile_lint.sh` - PASS
- `bash scripts/ci/doctrine_exec_profile_lint.sh --prove-gha-proof-seal` - PASS
- `bash scripts/ci/doctrine_scan.sh` - PASS, failures=0 inspect=0
- `bash scripts/ci/gen_digest.sh --check` - PASS

## Scope Ledger

- runtime code: untouched
- crate tests: untouched
- workflows: untouched
- scanner allowlists/data: untouched
- inventory rows: no deletion, no reclassification
- edit-scope policy: unchanged; compatibility-only fallback added for local/GHA Python versions without `PurePosixPath.full_match`
- docs/audit evidence: updated

## Graduation routing

Status remains PROBATION. PR A must not merge until DA/orchestrator clearance. PR B (`TEST-PARE-PROTECTED-RESIDUE-0`) starts only after PR A is cleared and merged.

## Known gaps / next

Await DA/orchestrator review. If cleared and merged, open `TEST-PARE-PROTECTED-RESIDUE-0` to process any remaining protected-residue work under the audited owners and proof modes.
