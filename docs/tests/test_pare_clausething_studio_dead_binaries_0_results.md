# TEST-PARE-CLAUSETHING-STUDIO-DEAD-BINARIES-0 Results

Status: **PROBATION / ready for DA review**. Merge is not authorized by this rung.

This wave deliberately protects Admission Substrate, SimThing-Kernel, and CI Scaffolding corpora while deleting redundant Clausething/Studio fossils superseded by the 0.0.8.4.x regimes.

## Scope

- Branch: `test-pare-clausething-studio-dead-binaries-0`
- Base: stacked on `test-repair-or-replace-true-oracle-binaries-0` because #1106 / 0I evidence is not merged to master.
- Admission Substrate touched: no.
- SimThing-Kernel touched: no.
- CI Scaffolding touched: no gate/profile/allowlist edits; only Track D evidence docs and the required inventory/audit ledgers changed.
- Clausething deletions: 3 files / 9 rows.
- Studio deletions: 0; Studio/typeface sweep recorded as DA-deferred because reviewed rows are interleaved with golden-byte/oracle/local-owner-deep signals.

## Deleted Clausething Fossils

| File | Rows | Decision | Reason |
| --- | ---: | --- | --- |
| `crates/simthing-clausething/tests/ct_1b_recalc.rs` | 4 | `DELETE_NONCOMPILING_FOSSIL` | No live product-code or standing-doc reference; inherited #1106 dead-binary classification; superseded by 0.0.8.4.x regimes. |
| `crates/simthing-clausething/tests/ct_1c_tradition.rs` | 3 | `DELETE_NONCOMPILING_FOSSIL` | No live product-code or standing-doc reference; inherited #1106 dead-binary classification; superseded by 0.0.8.4.x regimes. |
| `crates/simthing-clausething/tests/ct_3b_4a_headline.rs` | 2 | `DELETE_NOT_LIVE_REFERENCED` | No live product-code or standing-doc reference; not a current oracle, seal, golden, STEAD, or standing-doc proof. |

## Kept / Deferred Rows

- `ct_3b_4a_gpu_projection.rs`: kept as current `oracle-parity` permanent-residue owner.
- `ct_3b_4a_session_loop.rs`: kept because live non-archive docs name it as canonical session-loop enforcement.
- `ct_rf_eml_rate.rs`: kept because live worklog records its bit-exact GPU proof.
- Studio/typeface rows: deferred for DA/owner-deep review; obvious candidates had golden-byte, oracle, proof-seal, or standing lineage signals.

## Inventory

- Before: 4,252 live rows.
- After: 4,243 live rows.
- Delta: -9 live rows.
- Historical audit: 9 new `PARED` rows in `scripts/ci/test_pare_audit.tsv`.
- Deferred protected rows: 15 restored/deferred Admission Substrate rows remain untouched in `simthing-spec`; 6 Clausething rows remain because they have current protection reasons.

## Evidence

- Review table: `docs/tests/test_pare_clausething_studio_dead_binaries_0_review.tsv`.
- Results doc: this file.
- Evidence index: `docs/tests/current_evidence_index.md`.
- Design row: `docs/design_0_0_8_4_6_ci_scaffolding.md`, Track D D2q.

## Proof

- Doctrine Scan: PASS (`DOCTRINE-SCAN-VERDICT: PASS failures=0 inspect=0 selftest=SKIPPED`).
- Digest: PASS (`gen_digest --check: PASS`).
- Inventory: PASS (`TEST-INVENTORY-CHECK-VERDICT: PASS`; rows/discovered 4,243, missing 0, extra 0).
- Boundary: PASS (`TEST-PARE-BOUNDARY-CHECK-VERDICT: PASS`; live rows with owning boundary 4,243, missing 0).
- Drift: PASS (`TEST-INVENTORY-DRIFT-CHECK-VERDICT: PASS`; unledgered 0, stale 0).
- Cargo checks: PASS (`cargo check -p simthing-clausething`; warnings only).

## Graduation Routing

- Risk class: test deletion over Clausething/Studio fossils.
- Recommended posture: medium/deep DA review.
- Merge authorization: none.
