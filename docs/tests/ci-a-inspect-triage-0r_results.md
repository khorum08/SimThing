# CI-A-INSPECT-TRIAGE-0R: implement and enforce INSPECT spam bounds

Status: PROBATION

CI-A-INSPECT-TRIAGE-0 — HELD / superseded by 0R
CI-A-INSPECT-TRIAGE-0R — PROBATION

## What changed

- Replaced 0-byte inert `scripts/ci/inspect_spam_check.sh` with real implementation.
- All 3 §1A bounds implemented (deltas on branch history):
  1. >3 branch-introduced INSPECT flags across commits -> SPAM
  2. Same symbol/location under >=2 HEURISTIC scan IDs -> SPAM
  3. INSPECT count rising while original RELIABLE FAIL stays open -> SPAM
- Workflow `.github/workflows/doctrine-scan.yml` no longer masks with `|| true`; SPAM (nonzero) fails the check, publish/upload still under `if: always()`.
- `--prove` mode with executable synthetic git history for the 5 proof cases.
- Normal `<branch>` still works for "single-gray-zone" / "symbol-walking".

## Workflow enforcement

Fixed tee masking. SPAM escalates as FAIL. Artifacts published on failure.

## Executable proof cases

```bash
bash scripts/ci/inspect_spam_check.sh --prove
...
single-gray-zone: INSPECT-SPAM-CHECK: OK
RC=0
symbol-walking: INSPECT-SPAM-CHECK: SPAM
RC=1
...
INSPECT-SPAM-PROOF: PASS
EXIT=0
```

## GitHub Actions proof

(After merge + run: record the CI run ID.)

## Load-bearing transcripts (real local)

```bash
bash scripts/ci/inspect_spam_check.sh single-gray-zone
INSPECT-SPAM-CHECK: OK
EXIT=0
```

```bash
bash scripts/ci/inspect_spam_check.sh symbol-walking
INSPECT-SPAM-CHECK: SPAM
EXIT=1
```

```bash
bash scripts/ci/inspect_spam_check.sh --prove
```
(Full output as captured in .tmp_spam_proof_final.out — ends with two SPAM lines for bounds + INSPECT-SPAM-PROOF: PASS)

Other required (remain green post A):

```bash
bash scripts/ci/doctrine_selftest.sh   # DOCTRINE-SELFTEST-VERDICT: PASS
bash scripts/ci/doctrine_scan.sh       # DOCTRINE-SCAN-VERDICT: PASS
bash scripts/ci/doctrine_pr_scan.sh --prove-delta  # PR-delta proof: PASS
python scripts/ci/verify_kernel_surface.py  # 195/195
```

## Scope Ledger

Followed. Only allowed files (inspect script, workflow, evidence, design, README).

## Known gaps / next

- Record actual GitHub Actions run ID after the PR runs the real checker.
- Full 0R evidence update in index/design.

DOCTRINE TRIAGE REPORT:
(See transcripts)

DOCTRINE SELFTEST REPORT:
DOCTRINE-SELFTEST-VERDICT: PASS (from A)

DOCTRINE SCAN REPORT:
DOCTRINE-SCAN-VERDICT: PASS

PR-DELTA PROOF:
PR-delta proof: PASS

KERNEL SURFACE VERIFY:
195/195
