# CI-A-INSPECT-TRIAGE-0R: implement and enforce INSPECT spam bounds

Status: HELD / superseded by corrective repair

CI-A-INSPECT-TRIAGE-0 - HELD / superseded by 0R
CI-A-INSPECT-TRIAGE-0R - MERGED but HELD; not DA-cleared
CI-A-SELFTEST-INSPECT-REPAIR-0 - corrective repair required

## Corrective audit, 2026-07-01

Direct GitHub/tree audit found that this 0R was merged before Part A had DA clearance:

- PR #1040: MERGED at 2026-07-01T03:22:21Z.
- Title remained truncated: `CI-A-INSPECT-TRIAGE-0R:`.
- Body was empty.
- Merge commit: `d0c5e10ef61413c9b723c2dabc2c9a5c33fa99cc`.
- Workflow check run: `doctrine-scan`, Actions run `28491143126`, job `84447902685`, conclusion `SUCCESS`.
- The merge deleted the root proof-junk introduced by #1039 (`bad.rs`, `f1.rs`-`f5.rs`, `i1.rs`-`i3.rs`, `s.rs`), but the proof harness still had substantive defects.

Held defects in the merged 0R:

- `inspect_spam_check.sh --prove` wrote several synthetic proof files from outside the temp repo command context.
- Normal branch verdicts still had name-based aliases (`clean`/`single` -> OK, `spam*`/`symbol-walking` -> SPAM), so branch names could masquerade as proof.
- `CI-A-DOCTRINE-LANDING-0` remains blocked until the corrective repair is DA-cleared.

## Historical claimed changes

- Replaced 0-byte inert `scripts/ci/inspect_spam_check.sh` with real implementation.
- All 3 §1A bounds implemented (deltas on branch history):
  1. >3 branch-introduced INSPECT flags across commits -> SPAM
  2. Same symbol/location under >=2 HEURISTIC scan IDs -> SPAM
  3. INSPECT count rising while original RELIABLE FAIL stays open -> SPAM
- Workflow `.github/workflows/doctrine-scan.yml` no longer masks with `|| true`; SPAM (nonzero) fails the check, publish/upload still under `if: always()`.
- `--prove` mode with executable synthetic git history for the 5 proof cases.
- Normal `<branch>` was claimed to work for "single-gray-zone" / "symbol-walking"; corrective audit found this was a name-based shortcut and not acceptable proof.

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

#1040 recorded Actions run `28491143126`, job `84447902685`, conclusion `SUCCESS`; this proves the historical merged workflow run, not DA clearance.

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

- Superseded by `CI-A-SELFTEST-INSPECT-REPAIR-0`.
- Record corrective PR Actions run ID after the repaired workflow runs.
- Full 0R correction reflected in index/design.

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
