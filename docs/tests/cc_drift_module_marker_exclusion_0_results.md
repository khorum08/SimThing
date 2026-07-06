# CC-DRIFT-MODULE-MARKER-EXCLUSION-0 Results

## Status

DA-GRADUATED / merged #1197 @ `6add3c772307bebe8953a6dec05909701f90d767`.

tested_code_sha: `92393b5efc40d18bfca99a15a8aab6a2584ac5ee`
coverage_basis: PASS - committed drift-gate implementation validated by the proofs below; final follow-up commits, if any, are proof-documentation only.

## ORIENT-RECEIPT

- ORIENT-RECEIPT: `317ab9b71a6c`
- role: `coding`
- orientation_rule_stamp: `94fd88f77043af7d`
- orientation_digest_sha: `bd74d9a81ebf7ba03dd281f7407959632eda3907313dc099623f112b6cf34b51`

## Trigger

The first actual module-marker sweep attempted to remove the `simthing-mapgenerator` `cfg_test_mod::tests` deletion-candidate row, but `test_inventory_drift_check.sh` rediscovered it from `crates/simthing-mapgenerator/src/report.rs` and failed with an unledgered row.

## What changed

- Updated the drift gate so `cfg_test_mod::*` module markers are not mandatory discovered test rows.
- Preserved strict drift for real test functions and other real witnesses.
- Added inline proof cases for module-marker exclusion, module-marker removal, pending deletion-candidate compatibility, real-test failure, and stale KEEP failure.
- Did not delete inventory rows.
- Did not edit Rust source.

## Drift behavior after fix

| Case | Expected |
|---|---|
| only cfg_test_mod module marker present, no inventory row | PASS |
| cfg_test_mod deletion-candidate row removed | PASS |
| real `#[test]` function row removed | FAIL |
| existing cfg_test_mod deletion-candidate rows remain pending sweep | PASS |
| KEEP/durable real test row removed | FAIL |

## Load-bearing proofs

```text
bash scripts/ci/test_inventory_drift_check.sh --prove
TEST-INVENTORY-DRIFT-PROVE REPORT
  unledgered-test: PASS (unledgered test rows: 1; remedy: add a classified ledger row or remove the test; first=[('simthing-spec', 'crates/simthing-spec/tests/unledgered_test.rs', 'unledgered_test_fixture_should_fail_drift', 'integration')])
  cfg-marker-unledgered-ok: PASS
  cfg-marker-removed-ok: PASS
  cfg-marker-deletion-candidate-stale-ok: PASS
  cfg-marker-KEEP-stale: PASS (ledgered-but-deleted test rows: 1; stale ledger first=[('simthing-mapgenerator', 'crates/simthing-mapgenerator/src/report.rs', 'cfg_test_mod::tests', 'unit')])
  stale-ledger: PASS (ledgered-but-deleted test rows: 1; stale ledger first=[('simthing-spec', 'crates/simthing-spec/tests/deleted.rs', 'deleted_test', 'integration')])
  unowned-KEEP: PASS
  kernel-sim-strict-tier: PASS
TEST-INVENTORY-DRIFT-PROVE-VERDICT: PASS
```

```text
bash scripts/ci/test_inventory_drift_check.sh
TEST-INVENTORY-DRIFT-CHECK-VERDICT: PASS
rows: 973
discovered: 834
unledgered: 0
stale: 0
```

```text
bash scripts/ci/test_lifecycle_expiry_check.sh --schema
LIFECYCLE-EXPIRY-VERDICT: PASS expired=0 audit=0 max_dsu_survivals=0 mode=schema
```

```text
bash scripts/ci/lifecycle_schema_pr_gate.sh master HEAD
LIFECYCLE-SCHEMA-PR-GATE: SKIP (no inventory/lifecycle TSV diff)
```

```text
bash scripts/ci/test_lifecycle_expiry_check.sh --scheduled
LIFECYCLE-EXPIRY-VERDICT: INSPECT expired=137 audit=0 max_dsu_survivals=0 mode=scheduled
```

```text
bash scripts/ci/doctrine_scan.sh
DOCTRINE-SCAN-VERDICT: INSPECT  failures=0 inspect=415 selftest=SKIPPED
```

```text
bash scripts/ci/gen_orientation.sh --check
gen_orientation --check: PASS
```

```text
bash scripts/ci/gen_digest.sh --check
gen_digest --check: PASS
```

```text
bash scripts/ci/clearance_check.sh --range master..HEAD
CLEARANCE-VERDICT: DA-RESERVE(novelty)
```

```text
git diff --check HEAD
PASS
```

Omissions:

- `cargo not run - no Rust crate touched`
- `doctrine_selftest.sh not run - scanner surface unchanged`
- `clearance_check.sh --selftest not run - clearance/router surface unchanged`
- `no inventory rows deleted`
- `no source files edited`

## Scope Ledger

| Surface | Status | Notes |
|---|---|---|
| scripts/ci/test_inventory_drift_check.sh | changed | cfg_test_mod exclusion |
| scripts/ci/fixtures/test_drift/** | not changed | inline prove fixtures used |
| docs/tests/cc_drift_module_marker_exclusion_0_results.md | added | proof artifact |
| scripts/ci/test_inventory.tsv | not changed | no sweep in this PR |
| crates/*/src/** | not touched | no source edits |
| crates/*/tests/** | not touched | no test edits/deletions |
| scanner surface | not touched | doctrine_selftest.sh not run |
| clearance/router surfaces | not touched | #1196 already landed |

## Conformance

- Drift no longer forces module-marker ledger rows to remain.
- Real tests remain drift-protected.
- Existing pending module-marker deletion-candidates do not fail stale drift.
- No actual sweep is performed.
- No source edited.
- No test deleted.
- No SHA matching.

## Known gaps / next

This drift-gate fix has landed. The corpus-clearance sweep track is parked until Opus/Fable is available again. After parking lifts, rerun the actual first module-marker ledger sweep for `simthing-mapgenerator` and remove the single `cfg_test_mod::tests` deletion-candidate row.

## Graduation routing

- Final status: DA-GRADUATED / merged #1197 @ `6add3c772307bebe8953a6dec05909701f90d767`
- CI verdict: proof-present; local checks pass with doctrine scan INSPECT-only
- Triage entries: none
- Risk class: gate-wiring, drift-gate, corpus-module-marker-sweep
- Falsification check:
  - cfg_test_mod-only fixture passes without inventory row
  - real-test unledgered fixture still fails
  - existing drift prove cases still pass
  - no inventory/source/test deletion in this PR
- Recommended posture: graduated/merged
