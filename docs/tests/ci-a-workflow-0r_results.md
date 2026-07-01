# CI-A-WORKFLOW-0R Results

## Status

**PROBATION** — PR-delta HEURISTIC scan semantics enforced in GitHub Actions. `CI-A-WORKFLOW-0` remains PROBATION. Not DA acceptance.

## PR / branch / merge

- Branch: `ci-a-workflow-0r`
- PR: [#1035](https://github.com/khorum08/SimThing/pull/1035)
- Merge: `4c14dd6487` (master)

## Recipient Agent

Cursor

## Orchestrator / §1A Triage Agent

Codex

## Executive DA

Opus / Owner

## What changed

- Added `scripts/ci/doctrine_pr_scan.sh` — PR workflow wrapper calling `doctrine_scan.sh --pr-delta BASE HEAD`; includes `--prove-delta` local proof harness.
- Extended `doctrine_scan.sh` with `--pr-delta BASE HEAD` mode:
  - RELIABLE scans unchanged (whole-tree).
  - HEURISTIC scans limited to changed files intersecting target glob, matches filtered to changed lines in `git diff -U0`.
  - Report adds scope lines: `scan mode: PR delta`, `reliable scope: whole-tree`, `heuristic scope: changed files / changed lines`.
- Updated `.github/workflows/doctrine-scan.yml`:
  - `fetch-depth: 0` for PR diffs.
  - `pull_request`: self-test → `doctrine_pr_scan.sh` with base/head SHAs.
  - `push` to `master`: self-test → whole-tree `doctrine_scan.sh`.
- Updated `scripts/ci/README.md`, design row `CI-A-WORKFLOW-0R` → **PROBATION**.

## PR-delta contract

| Scan severity | Scope on PR runs |
|---|---|
| RELIABLE | Whole-tree (unchanged) |
| HEURISTIC | Changed files ∩ target glob; match line must appear in PR diff hunks |

| Outcome | Exit code | Workflow |
|---|---|---|
| Self-test failure | nonzero | FAIL |
| RELIABLE hard FAIL | nonzero | FAIL |
| HEURISTIC INSPECT only | zero | PASS (visible in report) |
| Clean scan | zero | PASS |

Master push: whole-tree positive-control scan (no PR delta required).

## Workflow contract

- **pull_request:** checkout (full history) → rg fallback → `doctrine_selftest.sh` → `doctrine_pr_scan.sh base head` → summary + artifact (`if: always()`).
- **push master:** checkout → rg fallback → `doctrine_selftest.sh` → `doctrine_scan.sh` → summary + artifact.

## PR-delta proof cases

Local: `bash scripts/ci/doctrine_pr_scan.sh --prove-delta`

| Case | Expected | Result |
|---|---|---|
| PR delta with no HEURISTIC violation | PASS, exit 0 | PASS |
| Pre-existing HEURISTIC outside delta | not re-flagged (SEMANTIC-WORDS) | PASS |
| HEURISTIC violation in delta | INSPECT, exit 0 | PASS |
| RELIABLE violation in tree | FAIL, exit nonzero | PASS |

## GitHub Actions proof

(pending — recorded after PR merge)

## Load-bearing proofs

| Proof | Result |
|---|---|
| `bash scripts/ci/doctrine_selftest.sh` | PASS — `DOCTRINE-SELFTEST-VERDICT: PASS` |
| `bash scripts/ci/doctrine_scan.sh` | PASS — whole-tree `DOCTRINE-SCAN-VERDICT: PASS` |
| `bash scripts/ci/doctrine_pr_scan.sh --prove-delta` | PASS — 4/4 cases |
| `python scripts/ci/verify_kernel_surface.py` | PASS — 195/195 |

## Scope Ledger

| Path | Touched |
|---|---|
| `.github/workflows/doctrine-scan.yml` | yes |
| `scripts/ci/doctrine_pr_scan.sh` | yes (new) |
| `scripts/ci/doctrine_scan.sh` | yes (PR-delta mode only) |
| `scripts/ci/README.md` | yes |
| `docs/tests/ci-a-workflow-0r_results.md` | yes |
| `docs/tests/current_evidence_index.md` | yes |
| `docs/design_0_0_8_4_6_ci_scaffolding.md` | yes |
| `scans.tsv`, allowlists, fixtures, crates, triage | **no** |

## Known gaps / next

- `CI-A-INSPECT-TRIAGE-0` — triage log, spam bounds, per-INSPECT justification slots (authorized after 0R).

## DOCTRINE SELFTEST REPORT

```
DOCTRINE-SELFTEST-VERDICT: PASS
```

## DOCTRINE SCAN REPORT

```
DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED
  scan mode: whole-tree
  reliable scope: whole-tree
  heuristic scope: whole-tree
```
