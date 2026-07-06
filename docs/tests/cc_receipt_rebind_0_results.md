# CC-RECEIPT-REBIND-0 Results

Status: PROBATION

tested_code_sha: fad0b4dd13af90aa9ff6b7c5ce6fdc64f22c7db6
coverage_basis: PASS - required harness checks run against the implementation commit; Rust crates were not touched, so no `cargo check -p <crate>` target applied.

Carried orientation receipt:

```text
ORIENT-RECEIPT: 473d92b33409
role: coding
orientation_digest_sha: 95d8d1778b2ab0d3f7cb6758ff668970ec898ce61d563f189d70aed7b6892ed0
generated_at: source-bound
```

The receipt above was the one-time emergency refresh allowed by the handoff before this rebind changed the receipt schema. The implementation live rule stamp after the rebind is `226295d8778ed5f9`; the generated orientation digest after the rebind is `086ab4600a9ca18332c7e1cfeec888849a8461d8c06a46cc006c52ad2342dc50`.

## Implementation Notes

- `orient.sh` now computes `orientation_rule_stamp` from `scripts/ci/precedented_classes.tsv`, `scripts/ci/binding_conditions.tsv`, and `scripts/ci/doctrine_anchors.tsv`.
- `ORIENT-RECEIPT` is now `role + orientation_rule_stamp`; `orientation_digest_sha` is emitted as informational context only.
- `relay_lint.sh` validates receipt freshness against `orientation_rule_stamp`, not the rendered orientation digest.
- `orient.sh --since=<receipt>` emits `CURRENT` or `STALE(rule-source)` without reprinting the full orientation.
- Cold-start and anchor fixtures were regenerated, including `cold_start_selftest_pass_prose_digest_churn` to prove digest churn does not stale a valid receipt.

## Proofs

```text
bash scripts/ci/orient.sh --selftest
ORIENT-SELFTEST: PASS (3 fixtures)
```

```text
bash scripts/ci/relay_lint.sh --selftest
RELAY-LINT-SELFTEST: PASS (24 fixtures)
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
bash scripts/ci/test_inventory_drift_check.sh
TEST-INVENTORY-DRIFT-CHECK-VERDICT: PASS
rows: 935
discovered: 933
unledgered: 0
stale: 0
```

```text
bash scripts/ci/doctrine_scan.sh
DOCTRINE-SCAN-VERDICT: INSPECT  failures=0 inspect=415 selftest=SKIPPED
```

```text
git diff --check HEAD
PASS
```

Not run:

- `doctrine_selftest.sh` - scanner surface unchanged; handoff explicitly prohibited running it.
- `clearance_check.sh` / `/clearance` - clearance surface not changed for routing decision, and this gate-wiring rung is PROBATION only.
- `cargo check -p <crate>` - no Rust crate was touched.

## Routing

CLEARANCE-VERDICT: ORCHESTRATOR-TO-RUN

Final posture: PROBATION / proof-present / DA-review-pending.
