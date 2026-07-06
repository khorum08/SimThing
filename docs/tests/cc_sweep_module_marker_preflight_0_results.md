# CC-SWEEP-MODULE-MARKER-PREFLIGHT-0 Results

## Status

DA-GRADUATED / merged #1196 @ `8036795394`.

tested_code_sha: `89e6b57ce95d9f562a8d20cdd6e8a3e7489cdf3f`
coverage_basis: PASS - committed implementation validated by the harness proofs below; final follow-up commits, if any, are proof-documentation only.
ci_green: local required checks pass; GitHub CI pending after PR publication.
no_engine_src: PASS - no `crates/*/src/**` files changed.

## ORIENT-RECEIPT

- ORIENT-RECEIPT: `3e9d974ebaf1`
- role: `coding`
- orientation_rule_stamp: `66e76ed9955fae1b`
- orientation_digest_sha: `ffa9f046632fbeebfe170f8c50f29b267237237e6408747c7caa6cbff355fded`
- after this rung's class-table edit: `ORIENT-SINCE-VERDICT: STALE(rule-source)`, current_receipt `317ab9b71a6c`, orientation_rule_stamp `94fd88f77043af7d`

## Trigger

The first calibrated sweep found no lawful `crates/*/tests/**` deletion candidate in the small-crate set. The deletion-candidate residue is `cfg(test)` module-marker inventory rows under `crates/*/src/**`, which must not be edited as source.

## What changed

- Added clearance class ownership for module-marker ledger-only sweeps.
- Added route/content validation requiring removed inventory rows to be `cfg_test_mod::*` `deletion-candidate` rows with `ledger-only` ownership.
- Added selftest fixtures proving module-marker ledger sweep routing.
- Documented the two sweep shapes:
  - test-file deletion sweep
  - module-marker ledger-only sweep
- Did not delete tests.
- Did not delete corpus inventory rows in this preflight.
- Did not touch Rust source.

## Routing proof

```text
bash scripts/ci/clearance_check.sh --selftest
CLEARANCE-SELFTEST: PASS (22 fixtures)
```

```text
bash scripts/ci/clearance_check.sh --fixture clearance_selftest_clearable_module_marker_sweep
CLEARANCE-VERDICT: ORCHESTRATOR-CLEARABLE
```

```text
bash scripts/ci/clearance_check.sh --fixture clearance_selftest_module_marker_without_result_no_match
CLEARANCE-VERDICT: DA-RESERVE(novelty)
```

```text
bash scripts/ci/clearance_check.sh --fixture clearance_selftest_module_marker_bad_inventory_no_match
CLEARANCE-VERDICT: DA-RESERVE(novelty)
```

```text
bash scripts/ci/clearance_check.sh --fixture clearance_selftest_module_marker_source_edit_rejected
CLEARANCE-VERDICT: DA-RESERVE(novelty)
```

Self-application routing for this preflight PR is expected to be:

```text
CLEARANCE-VERDICT: DA-RESERVE(gate-wiring)
```

because this rung edits router/class gate surfaces. The fixture above proves the future module-marker sweep shape itself is clearable.

## Load-bearing proofs

```text
bash scripts/ci/clearance_check.sh --selftest
CLEARANCE-SELFTEST: PASS (22 fixtures)
```

```text
bash scripts/ci/test_inventory_drift_check.sh
TEST-INVENTORY-DRIFT-CHECK-VERDICT: PASS
```

```text
bash scripts/ci/test_lifecycle_expiry_check.sh --schema
LIFECYCLE-EXPIRY-VERDICT: PASS expired=0 audit=0 max_dsu_survivals=0 mode=schema
```

```text
bash scripts/ci/lifecycle_schema_pr_gate.sh master HEAD
LIFECYCLE-SCHEMA-PR-GATE: RUN
LIFECYCLE-EXPIRY-VERDICT: PASS expired=0 audit=0 max_dsu_survivals=0 mode=schema
```

```text
bash scripts/ci/clearance_check.sh --range master..HEAD
CLEARANCE-VERDICT: DA-RESERVE(gate-wiring)
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
git diff --check HEAD
PASS
```

Omissions:

- `doctrine_selftest.sh not run - scanner surface unchanged`
- `cargo not run - no Rust crate touched`
- `no tests deleted`
- `no corpus inventory rows deleted`
- `no source touched`

## Scope Ledger

| Surface | Status | Notes |
|---|---|---|
| scripts/ci/precedented_classes.tsv | changed | module-marker ledger sweep ownership |
| scripts/ci/clearance_check.sh | changed | module-marker route/content validation and selftests |
| clearance fixtures | changed | module-marker routing proof |
| scripts/ci/test_inventory.tsv | changed | fixture ledger rows added only; no corpus rows deleted |
| docs/design_0_0_8_4_8_corpus_clearance.md | changed | two sweep shapes documented |
| docs/orchestrator_orientation.md | changed | regenerated from class-table source |
| docs/tests/cc_sweep_module_marker_preflight_0_results.md | added | proof artifact |
| crates/*/src/** | not touched | no source edits |
| crates/*/tests/** | not touched | no test deletion in preflight |
| scanner surface | not touched | doctrine_selftest.sh not run |

## Conformance

- Future module-marker ledger-only sweeps have a deterministic clearance class.
- Test-file deletion sweeps remain handled by corpus-sweep.
- Retired corpus-baseline remains retired.
- Arbitrary inventory edits without the module-marker results doc do not match.
- Non-module-marker inventory deletions with the module-marker results doc do not match.
- Source edits are not accepted as module-marker sweep proof.
- No tests deleted in this preflight.
- No corpus inventory rows deleted in this preflight.
- No SHA matching.

## Known gaps / next

This preflight has landed. The corpus-clearance sweep track is now parked after #1197 until Opus/Fable is available again. After parking lifts, run the first module-marker ledger-only sweep on `simthing-mapgenerator`.

## Graduation routing

- Final status: DA-GRADUATED / merged #1196 @ `8036795394`
- CI verdict: proof-present; local checks pass with doctrine scan INSPECT-only
- Triage entries: none
- Risk class: gate-wiring, corpus-clearance-preflight, inventory-ledger-sweep
- Falsification check:
  - module-marker ledger-only fixture routes deterministically
  - arbitrary inventory edit does not match
  - source edit is not accepted
  - no tests/source/corpus inventory rows deleted in preflight
- Recommended posture: graduated/merged
