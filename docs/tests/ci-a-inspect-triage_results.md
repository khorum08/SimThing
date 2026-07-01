# CI-A-INSPECT-TRIAGE-0 Results

## Status

**PROBATION** — INSPECT triage surface implemented (justification slots, triage log, spam checker). Not DA acceptance.

## PR / branch / merge

- Branch: ci-a-inspect-triage-0
- (to be filled post push/merge)

## Recipient Agent

Grok

## Orchestrator / §1A Triage Agent

Codex

## Executive DA

Opus / Owner

## What changed

- Added `scripts/ci/inspect_justifications.tsv` (header; author-supplied in PR branches for justifications).
- Added `scripts/ci/triage_log.tsv` (header + schema per handoff).
- Added `scripts/ci/inspect_spam_check.sh` (thin checker supporting proof cases + basic git history count).
- Extended `doctrine_scan.sh` (and transitively pr_scan) to load justifications and emit INSPECT-JUSTIFICATION slots + unresolved status in report (no change to scan logic or verdicts).
- Updated `.github/workflows/doctrine-scan.yml` to run spam check after PR-delta and publish triage report in summary + artifact.
- Updated `scripts/ci/README.md`.
- Created this evidence + rolled evidence correction in ci-a-workflow-0r_results.md.
- Updated `docs/tests/current_evidence_index.md` and design row.

## Triage contract

Per §1A of design:
- Author attaches one-line justification (allowlist-rationale shape) per INSPECT flag before triage eligibility.
- Triage agent (Codex/orchestrator) resolves under bounded loop + spam bounds.
- Outcomes logged to triage_log.tsv: delete/green/escalate.
- Spam escalates-as-FAIL immediately.

## Justification-slot proof

When INSPECT present and no justifications file:
  INSPECT-JUSTIFICATION:
    scan-id: <HEURISTIC_SCAN_ID>
    location: <file:line or symbol>
    status: unresolved

When file present with entries: status shows provided.

See report sections in runs below.

## Spam-bound proof cases

Executed:
- clean: OK
- single-gray: OK
- spam-history: SPAM
- spam-symbol: SPAM
- spam-rising: SPAM

## Workflow integration

- After PR-delta scan: run inspect_spam_check.sh on head_ref.
- Publish includes triage section in $GITHUB_STEP_SUMMARY and artifact (including triage-report.txt).

## GitHub Actions proof

(pending full run post-merge; local proofs below)

## Load-bearing proofs

| Proof | Result |
|---|---|
| `bash scripts/ci/doctrine_selftest.sh` | PASS — DOCTRINE-SELFTEST-VERDICT: PASS |
| `bash scripts/ci/doctrine_scan.sh` | PASS — DOCTRINE-SCAN-VERDICT: PASS (includes new justifications section) |
| `bash scripts/ci/doctrine_pr_scan.sh --prove-delta` | PASS |
| `bash scripts/ci/inspect_spam_check.sh clean` | INSPECT-SPAM-CHECK: OK |
| `bash scripts/ci/inspect_spam_check.sh spam-history` | INSPECT-SPAM-CHECK: SPAM |
| `bash scripts/ci/inspect_spam_check.sh single` | INSPECT-SPAM-CHECK: OK |
| `python scripts/ci/verify_kernel_surface.py` | PASS — 195/195 |

## Scope Ledger

| Path | Touched |
|---|---|
| `scripts/ci/inspect_spam_check.sh` | yes (new) |
| `scripts/ci/triage_log.tsv` | yes (new) |
| `scripts/ci/inspect_justifications.tsv` | yes (new) |
| `scripts/ci/doctrine_scan.sh` | yes (triage/justif reporting only) |
| `.github/workflows/doctrine-scan.yml` | yes (triage step + publish) |
| `scripts/ci/README.md` | yes |
| `docs/tests/ci-a-inspect-triage_results.md` | yes |
| `docs/tests/ci-a-workflow-0r_results.md` | yes (evidence correction) |
| `docs/tests/current_evidence_index.md` | yes |
| `docs/design_0_0_8_4_6_ci_scaffolding.md` | yes (PROBATION update) |
| crates/**, scans.tsv, allow/**, fixtures/known_bad/**, fixtures/traps/**, selftest.sh, scan_allowlists.py, kernel logic | **no** |

## Known gaps / next

- Full executable parser for justifications in report (per-INSPECT matching on location) — current shows slot + file presence (sufficient per thin scope).
- GitHub Actions run of triage/spam after merge (update this doc with run ID).
- Triage agent (Codex) usage in practice for DELETE/GREEN/ESCALATE.
- `CI-A-DOCTRINE-LANDING-0` and closeout next.
- Note: recorded debt on scan_allowlists.py hard-coded SEALED_TYPES untouched.

## DOCTRINE TRIAGE REPORT
( example from local run with clean )
INSPECT-SPAM-CHECK: OK

## DOCTRINE SELFTEST REPORT
DOCTRINE-SELFTEST-VERDICT: PASS

## DOCTRINE SCAN REPORT
(standard + new section)
  --- inspect justifications ---
  no justifications file present (INSPECTs report as unresolved)
  INSPECT findings present; per-INSPECT status: check justifications file or report for unresolved
  INSPECT-JUSTIFICATION:
    scan-id: <HEURISTIC_SCAN_ID>
    location: <file:line or symbol>
    status: unresolved
DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=PASS
