# TEST-PARE-PROTECTED-CLASS-AUDIT-0 Results

## Status

PROBATION / HOLD cleared for 0R push only. This PR remains PR A and must not merge until DA/orchestrator clearance. This PR deletes zero tests and does not authorize self-merge.

## #1101 closeout

`GHA-PROOF-SEAL-0` is DONE and merged as #1101. Merge commit: `e49c8a258e4bd58d9c78b6c82b698cd5650dbaca`. Head: `317aba88f649a027fcd2c9997b182a7c27005cce`. The enforced rule is that non-owner-deep GitHub Doctrine Exec profiles cannot contain Atlas/Bevy/GPU/desktop/mapeditor/tools runtime proof tokens. Prove path: `bash scripts/ci/doctrine_exec_profile_lint.sh --prove-gha-proof-seal`.

## Current inventory baseline

- inventory rows: 5332
- protected rows audited: 5079
- protected KEEP rows audited: 829
- TRUE_MEMBER count: 582
- FALSE_MEMBER count: 98
- NEEDS_PROMOTION count: 112
- NECESSARY_CITED_DEPENDENCY count: 33
- LEDGER_DEFECT count: 4
- OUT_OF_SCOPE judgment AUDIT count: 4250
- deletion queue size for TEST-PARE-PROTECTED-RESIDUE-0: 98

## Why-chain legend

See `docs/tests/test_residue_class_legend.md`. The 0R audit binds every permanent-residue class to an owning doctrine and tests membership class-by-class. `KEEP` is not treated as proof.

## Audit logic correction

The original PR A generator was tautological: non-KEEP became `OUT_OF_SCOPE` and KEEP became `TRUE_MEMBER`. 0R replaces that with class-specific verification:

- oracle-parity rows must name or source-cite CPU-to-GPU, CPU-to-kernel, or CPU-to-live-op parity for a live surface.
- seal-proof rows must be compile_fail/trybuild proofs for a live sealed boundary, while CI fixtures are dependency-floor rows rather than automatic seal-proof.
- golden-byte rows must identify byte identity, canonical format, deterministic diagnostic, deterministic replay, or canonical corpus surfaces.
- stead-required rows must live in the section 8 named suites or direct helper surfaces.
- doc-named invariant rows must be explicitly named by live non-archive docs.
- judgment rows remain OUT_OF_SCOPE unless they are KEEP rows with a specific `catches:` note.

## Judgment-note rule

`scripts/ci/test_inventory_check.sh` rejects future KEEP rows in `behavior-regression` or `escaped-bug` unless the note starts with `catches: ` and names a specific regression or bug. Boilerplate such as `catches: behavior regression`, `catches: escaped bug`, `catches: important coverage`, `permanent-residue:behavior-regression`, and `regression test` fails. Prove path: `bash scripts/ci/test_inventory_check.sh --prove-judgment-note-rule`.

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
| `TRUE_MEMBER` | 582 |
| `NEEDS_PROMOTION` | 112 |
| `FALSE_MEMBER` | 98 |
| `NECESSARY_CITED_DEPENDENCY` | 33 |
| `LEDGER_DEFECT` | 4 |

| Proposed next action | Rows |
|---|---:|
| `RECLASSIFY_TO_AUDIT` | 4250 |
| `KEEP` | 582 |
| `KEEP_PROMOTION_REQUIRED` | 112 |
| `RECLASSIFY_TO_PARE` | 98 |
| `KEEP_DEPENDENCY_FLOOR` | 33 |
| `FIX_LEDGER_ONLY` | 4 |

| Proof mode | Rows |
|---|---:|
| `not-required` | 4464 |
| `gha-cpu` | 435 |
| `local-owner-deep` | 180 |

## Coverage maps

- `docs/tests/test_pare_protected_class_audit_0_review.tsv`: all protected-surface rows and proposed disposition.
- `docs/tests/protected_class_oracle_parity_coverage.tsv`: 484 oracle rows with live parity surfaces and false/dependency outcomes.
- `docs/tests/protected_class_seal_proof_coverage.tsv`: 110 seal rows mapped to sealed boundaries or CI fixture dependencies.
- `docs/tests/protected_class_golden_byte_coverage.tsv`: 113 golden rows mapped to deterministic/canonical byte surfaces.
- `docs/tests/protected_class_stead_required_coverage.tsv`: 121 STEAD rows mapped to section 8 named suites/helper surfaces.
- `docs/tests/protected_class_doc_named_coverage.tsv`: 1 doc-named rows with live non-archive doc citations.
- `docs/tests/protected_class_judgment_keep_audit.tsv`: 4250 judgment-class rows; current judgment rows are AUDIT, not survivor shields.

## Oracle coverage

Oracle TRUE_MEMBER rows name live parity surfaces such as mobility kernels, resource-flow parity, atlas parity, owner-silo GPU tick parity, and velocity integration. Rows that only report, construct, or forbid CPU oracles without asserting parity are FALSE_MEMBER and queued for PR B reclassification/deletion review. GPU/WGPU/desktop-like parity surfaces remain `proof_mode=local-owner-deep`; no GHA GPU proof is added.

## Seal coverage

Compile-fail/trybuild rows are TRUE_MEMBER only when they map to a live sealed boundary. CI fixtures under `scripts/ci/fixtures/**` are NECESSARY_CITED_DEPENDENCY rows for scanner/probe self-tests; they are not counted as canonical product seal proofs.

## Golden coverage

Golden TRUE_MEMBER rows identify deterministic replay, canonical byte/format, stable diagnostic, checksum, digest, roundtrip, or exact-output surfaces. Rows without such a surface are NEEDS_PROMOTION rather than auto-accepted.

## STEAD coverage

STEAD TRUE_MEMBER rows live in the `docs/stead_spatial_contract.md` section 8 named suites: mapgen_constitution_guards, mapgen_lattice_hierarchy, mapgen_movement_front, mapgen_palma, mapgen_resource_flow, mapgen_rf_stead_binding, mapgen_structural_admission, mapgen_vast_scale_layout, stead_spatial_contract_guards. `crates/simthing-clausething/src/mapgen_palma.rs` is accepted as a direct helper for the named `mapgen_palma` suite. No section 8 suite is sub-pared in this PR.

## Doc-named coverage

`custom_layout_ethics_axis` remains TRUE_MEMBER because live `docs/invariants.md` explicitly names it as the invariant proof. Archive-only citations are ignored.

## Judgment-class findings

The ledger currently has no KEEP `behavior-regression` or `escaped-bug` rows. The 4250 current behavior-regression rows are AUDIT rows and therefore OUT_OF_SCOPE for protected-survivor membership. Future KEEP judgment rows must use the `catches:` note rule.

## Deletion queue for TEST-PARE-PROTECTED-RESIDUE-0

Queue size: 98. These are protected KEEP rows whose class-specific membership test is FALSE_MEMBER. PR A does not delete them. PR B must either reclassify/delete them under an owning boundary or produce stronger live evidence.

## Necessary/cited/dependency floor

Dependency-floor rows cite exact live CI/doc surfaces in the coverage maps, primarily `scripts/ci/fixtures/**` scanner/probe fixtures. Executable changes are limited to the inventory checker's judgment-note guard/prove mode, the generated protected-class audit helper, and the Python-version compatibility fix in `test_edit_scope_check.sh`.

## GHA proof-seal compliance

This PR adds no Doctrine Exec profile and no GHA command that runs Atlas, Bevy, GPU, desktop, mapeditor/tools runtime, WGPU, X11/Wayland, `apt-get`, workspace tests, all-crate cargo tests, or bare full-crate cargo tests.

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
- test deletion: none
- GHA Atlas/Bevy/GPU/desktop proof: none
- inventory rows: no deletion in PR A
- docs/audit evidence: updated for 0R

## Graduation routing

Graduation routing (for DA/orchestrator - why PROBATION, not COMPLETE):
  CI verdict:          PASS-RELIABLE
  Triage entries:      none
  Risk class:          data-deliverable + gate-wiring + protected-class reclassification
  Falsification check: protected KEEP rows are not auto-TRUE; every TRUE_MEMBER has a class-specific live surface; every FALSE_MEMBER is queued or reclassified; every NEEDS_PROMOTION has a named promotion target; dependency-floor rows cite exact live docs/profiles/selftests/dependencies; zero tests deleted; GHA proof seal remains green and no Atlas/Bevy/GPU/desktop proof enters non-owner-deep profiles.
  Recommended posture: deep - this audit defines the deletion queue for TEST-PARE-PROTECTED-RESIDUE-0.

## Known gaps / next

Await DA/orchestrator review. If cleared and merged, open `TEST-PARE-PROTECTED-RESIDUE-0` to process FALSE_MEMBER and NEEDS_PROMOTION rows under the audited owners and proof modes.
