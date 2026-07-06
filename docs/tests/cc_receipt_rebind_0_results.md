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

Final posture: PROBATION / proof-present / orchestrator-routing-pending.

## Clearance self-contamination remedy

- Problem: `/clearance` on PR #1189 appended a row to `scripts/ci/clearance_ledger.tsv` and pushed that commit onto the PR branch (`clearance: ledger row for PR 1189`). That put a gate-wiring surface into the PR diff, so subsequent `/clearance` runs saw harness contamination instead of routing the receipt-rebind work.
- Root cause: `.github/workflows/doctrine-exec-commands.yml` `clearance-run` had post-verdict steps that called `doctrine_exec_clearance.sh` to mutate the checked-out ledger and then `git commit` + `git push` back to `head_ref`. `clearance_check.sh` already ran with `CLEARANCE_LEDGER_APPEND=0`; the branch mutation came entirely from those workflow steps.
- Fix: removed the `Append clearance ledger row` and `Commit clearance ledger row to PR branch` steps from `clearance-run`. `/clearance` still runs `clearance_check.sh`, emits the sticky PR comment via `doctrine_exec_clearance_comment.sh`, and records verdict metadata in `clearance-report.txt` / workflow output only.
- Proof that `/clearance` no longer leaves `scripts/ci/clearance_ledger.tsv` in the PR diff: workflow no longer commits or pushes ledger rows; local `git diff master...HEAD -- scripts/ci/clearance_ledger.tsv` is empty after reverting the accidental GHA ledger commits; `clearance_check.sh --selftest` still passes with ledger writes confined to fixture temp files.
- Remaining routing posture: orchestrator should rerun `/clearance` on the branch containing this remedy. Expected routing is substantive (`gate-wiring` or `novelty` for this rung), not `DA-RESERVE(harness-error)` from ledger self-contamination.

Final posture: PROBATION / proof-present / orchestrator-routing-pending.
