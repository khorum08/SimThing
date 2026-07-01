# CI-A-SELFTEST-INSPECT-REPAIR-0: corrective audit and proof-harness repair

Status: PROBATION / corrective repair pending DA ruling

## Why this exists

Tree audit contradicted the relay for the 0R sequence:

- #1039 (`CI-A-SELFTEST-0R:`) was merged at 2026-07-01T03:21:47Z before DA verification. Body was empty, title was truncated, merge commit `7c705fca525563156ee44ae1e62d01a41d8a7fac`, Actions run `28491141025` job `84447896167`.
- #1039's PR diff added root proof-junk files: `bad.rs`, `f1.rs`-`f5.rs`, `i1.rs`-`i3.rs`, `s.rs`.
- #1040 (`CI-A-INSPECT-TRIAGE-0R:`) was merged at 2026-07-01T03:22:21Z before Part A DA clearance. Body was empty, title was truncated, merge commit `d0c5e10ef61413c9b723c2dabc2c9a5c33fa99cc`, Actions run `28491143126` job `84447902685`.
- #1040 deleted the root proof-junk files, but left proof-harness defects and therefore did not clear the sequence violation.

## What changed

- Repaired `scripts/ci/inspect_spam_check.sh --prove` so all synthetic proof file writes and git operations run against temp repos via explicit temp paths / `git -C "$td"`.
- Removed normal branch-name verdict shortcuts. Branch names such as `spam`, `clean`, or `symbol-walking` no longer decide OK/SPAM by themselves.
- Added real proof coverage for the three section 1A spam bounds:
  - more than 3 branch-introduced INSPECT additions;
  - same symbol under two heuristic classes;
  - INSPECT additions rising while a RELIABLE marker remains open.
- Added an alias-quarantine proof: a branch named `spam` with no spam history returns OK.

## Current tree state

- Current master/head is free of root proof-junk files: `bad.rs`, `f1.rs`-`f5.rs`, `i1.rs`-`i3.rs`, `s.rs`.
- Workflow enforcement remains non-masked: `inspect_spam_check.sh` writes `doctrine-triage-report.txt`, cats it, and a SPAM exit fails the step.
- `CI-A-DOCTRINE-LANDING-0` remains BLOCKED.
- `CI-A-SELFTEST-0R` remains merged but not DA-cleared.
- `CI-A-INSPECT-TRIAGE-0R` remains merged but HELD until this corrective repair is DA-cleared.

## Local transcripts

All local Bash runs used Git for Windows Bash with bundled Python on PATH where Python was required.

```bash
bash scripts/ci/doctrine_selftest.sh
DOCTRINE-SELFTEST-VERDICT: PASS
```

```bash
bash scripts/ci/doctrine_scan.sh
DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED
```

```bash
bash scripts/ci/doctrine_pr_scan.sh --prove-delta
PR-delta proof: PASS
```

```bash
bash scripts/ci/inspect_spam_check.sh --prove
single-gray-zone: INSPECT-SPAM-CHECK: OK
symbol-walking: INSPECT-SPAM-CHECK: SPAM (same symbol >=2 HEUR)
>3-inspect: INSPECT-SPAM-CHECK: SPAM (>3 branch-introduced)
branch-name-alias-only: INSPECT-SPAM-CHECK: OK
rising-while-reliable: INSPECT-SPAM-CHECK: SPAM (INSPECT rising while RELIABLE open)
INSPECT-SPAM-PROOF: PASS
```

```bash
python scripts/ci/verify_kernel_surface.py
lib.rs exports: 195
kernel_surface.txt: 195
missing: []
extra: []
```

## GitHub Actions proof

Corrective PR #1041 first workflow run:

- Actions run `28492389014`
- Job `84451616254`
- Check `doctrine-scan`
- Conclusion `SUCCESS`
- Completed `2026-07-01T03:56:52Z`

## Scope ledger

Touched:

- `scripts/ci/inspect_spam_check.sh`
- `scripts/ci/README.md`
- `docs/design_0_0_8_4_6_ci_scaffolding.md`
- `docs/tests/ci-a-inspect-triage-0r_results.md`
- `docs/tests/ci-a-selftest-0r_results.md`
- `docs/tests/ci-a-selftest-inspect-repair-0_results.md`
- `docs/tests/current_evidence_index.md`

Forbidden / untouched:

- crates and runtime logic
- scan definitions / allowlists / fixtures
- unrelated SEALED_TYPES closeout debt

## Acceptance criteria

- Root proof-junk files absent from the current tree.
- `inspect_spam_check.sh --prove` writes no proof files into the repo root.
- Normal branch names are not verdict aliases.
- All required local transcripts pass.
- Corrective GitHub Actions run ID recorded after the branch runs.
- Doctrine landing remains blocked pending explicit DA clearance.
