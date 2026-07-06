# CC-SWEEP-PREFLIGHT-0 Results

## Status

DA-GRADUATED / merged #1195 @ `689efe5418`.

tested_code_sha: `3290337e441dc50d76b2a82f66db11d77d370175`
coverage_basis: PASS - committed implementation validated by the harness proofs below; final follow-up commits, if any, are proof-documentation only.

## ORIENT-RECEIPT

- ORIENT-RECEIPT: `080cebaa64e8`
- role: `coding`
- orientation_rule_stamp: `9787ebf92814f10a`
- orientation_digest_sha: `6c973eee8e61a7c41cd14c60ffe3ec420e6a14b38824ef0eafa3a29600c42786`
- after this rung's rule-source edit: `ORIENT-SINCE-VERDICT: STALE(rule-source)`, current_receipt `3e9d974ebaf1`, orientation_rule_stamp `66e76ed9955fae1b`

## What changed

- Retired the one-shot `corpus-baseline` clearance class.
- Added repeatable `corpus-sweep` clearance ownership for sweep-shaped diffs.
- Added clearance fixtures proving sweep-shape routing is unambiguous, retired baseline no longer matches, and engine-source changes are rejected.
- Added a negative clearance fixture proving a sweep results artifact alone is not enough to match `corpus-sweep`.
- Wired lifecycle schema check into Doctrine Scan for inventory/lifecycle TSV PR diffs.
- Added lifecycle schema gate fixtures proving invalid birth_track failure, clean pass, and unrelated skip.

## Why

CC-BASELINE-0 left two preflight hazards before the first sweep. First, `corpus-baseline` owned `scripts/ci/test_inventory.tsv`, which every sweep must also edit, so future sweeps could collide with the baseline class. Second, lifecycle schema validation was operator-run proof, not a blocking PR gate for inventory/lifecycle TSV diffs.

## Load-bearing proofs

```text
bash scripts/ci/clearance_check.sh --selftest
CLEARANCE-SELFTEST: PASS (18 fixtures)
```

```text
bash scripts/ci/clearance_check.sh --fixture clearance_selftest_clearable_corpus_sweep_shape
CLEARANCE-VERDICT: ORCHESTRATOR-CLEARABLE
```

```text
bash scripts/ci/clearance_check.sh --range master..HEAD
CLEARANCE-VERDICT: DA-RESERVE(gate-wiring)
```

```text
bash scripts/ci/lifecycle_schema_pr_gate.sh --selftest
LIFECYCLE-SCHEMA-PR-GATE-SELFTEST: PASS (3 fixtures)
```

```text
bash scripts/ci/lifecycle_schema_pr_gate.sh master HEAD
LIFECYCLE-SCHEMA-PR-GATE: RUN
LIFECYCLE-EXPIRY-VERDICT: PASS expired=0 audit=0 max_dsu_survivals=0 mode=schema
```

```text
bash scripts/ci/test_lifecycle_expiry_check.sh --schema
LIFECYCLE-EXPIRY-VERDICT: PASS expired=0 audit=0 max_dsu_survivals=0 mode=schema
```

```text
bash scripts/ci/test_inventory_drift_check.sh
TEST-INVENTORY-DRIFT-CHECK-VERDICT: PASS
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
python -c/PyYAML workflow validation
YAML OK
```

```text
git diff --check
PASS
```

Omissions:

- `doctrine_selftest.sh not run locally - scanner surface unchanged`
- `cargo not run - no Rust crate touched`
- `no tests deleted`
- `no inventory rows deleted`

## Routing expectation

Local clearance should route this PR as:

```text
DA-RESERVE(gate-wiring)
```

because it changes router classes / workflow gate wiring.

It must not route as:

```text
DA-RESERVE(harness-error)
```

## Scope Ledger

| Surface | Status | Notes |
| --- | --- | --- |
| scripts/ci/precedented_classes.tsv | changed | retire baseline, add sweep |
| scripts/ci/clearance_check.sh | changed | anchor corpus class matches and enforce `no_engine_src` |
| .github/workflows/doctrine-scan.yml | changed | PR-only lifecycle schema gate |
| scripts/ci/lifecycle_schema_pr_gate.sh | added | diff-scoped lifecycle schema gate and selftest |
| scripts/ci/test_lifecycle_expiry_check.sh | changed | env overrides for schema-gate fixtures |
| clearance fixtures | changed | sweep-shape routing |
| lifecycle/schema fixtures | changed | invalid/clean/skip gate proof |
| docs/tests/cc_sweep_preflight_0_results.md | added | proof artifact |
| docs/orchestrator_orientation.md | regenerated | class table changed |
| docs/orchestrator_orientation_digest.md | unchanged | `gen_digest --check` passed |
| test files | not deleted | no sweep yet |
| inventory rows | not deleted | no sweep yet |
| Rust production code | not touched | |
| scanner surface | not touched | doctrine_selftest.sh not run locally |

## Conformance

- `corpus-baseline` retired.
- `corpus-sweep` active and deterministic.
- Sweep-shaped diff routes to `corpus-sweep` without multi-match.
- Lifecycle schema check blocks invalid inventory/lifecycle TSV diffs in PR CI.
- Clean inventory/lifecycle TSV diff passes schema gate.
- Unrelated diffs skip schema gate.
- No tests deleted.
- No inventory rows deleted.
- No engine source touched.
- Scanner surface unchanged.
- No SHA matching.

## Known gaps / next

This preflight has landed. The corpus-clearance sweep track is now parked after #1197 until Opus/Fable is available again; no sweep is authorized by this status document.

## Graduation routing

- Final status: DA-GRADUATED / merged #1195 @ `689efe5418`
- CI verdict: proof-present; local checks pass with doctrine scan INSPECT-only
- Triage entries: none
- Risk class: gate-wiring, harness-enforcement, corpus-sweep-preflight
- Falsification check:
  - `corpus-baseline` status is retired
  - `corpus-sweep` exists and is active
  - sweep fixture routes only to corpus-sweep
  - lifecycle schema gate fails invalid birth_track
  - lifecycle schema gate passes clean inventory/lifecycle diff
  - unrelated diff skips schema gate
  - no test or inventory deletion happened
- Recommended posture: graduated/merged
