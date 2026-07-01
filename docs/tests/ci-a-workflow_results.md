# CI-A-WORKFLOW-0 Results

## Status

**PROBATION** — GitHub Actions doctrine scan gate wired. Not DA acceptance.

## PR / branch / merge

- Branch: `ci-a-workflow-0`
- PR: (pending)
- Merge: (pending)

## Recipient Agent

Cursor

## Orchestrator / §1A Triage Agent

Codex

## Executive DA

Opus / Owner

## What changed

- Added `.github/workflows/doctrine-scan.yml` — authoritative doctrine scan gate on `ubuntu-latest`.
- Triggers: `pull_request` and `push` to `master`.
- Steps: checkout → ensure `rg` (apt fallback) → `doctrine_selftest.sh` → `doctrine_scan.sh` → publish reports to job summary → upload artifacts.
- Self-test and scan steps use `set -o pipefail` so `tee` preserves non-zero exit codes (self-test failure or hard FAIL fails the job).
- Publish/upload steps use `if: always()` so reports remain visible when a scan step fails.
- Updated `scripts/ci/README.md`, design row `CI-A-WORKFLOW-0` → **PROBATION**.

## Workflow contract

| Condition | Workflow result |
|---|---|
| `doctrine_selftest.sh` failure | FAIL |
| `doctrine_scan.sh` hard FAIL (RELIABLE) | FAIL |
| `doctrine_scan.sh` INSPECT-only | PASS (verdict visible in summary/artifact) |
| Clean scan + self-test PASS | PASS |

No Rust/cargo/Node toolchain. Python stdlib only (preinstalled on runner; used by existing `scan_allowlists.py`).

## Local/load-bearing proofs

| Proof | Result |
|---|---|
| `bash scripts/ci/doctrine_selftest.sh` | PASS — `DOCTRINE-SELFTEST-VERDICT: PASS` |
| `bash scripts/ci/doctrine_scan.sh` | PASS — `DOCTRINE-SCAN-VERDICT: PASS` |
| `python scripts/ci/verify_kernel_surface.py` | PASS — 195/195 |

## GitHub Actions proof

Structural verification (pre-merge):

- YAML present at `.github/workflows/doctrine-scan.yml`.
- `runs-on: ubuntu-latest`.
- `actions/checkout@v4` precedes scan steps.
- Ripgrep fallback via `apt-get install -y ripgrep` when `rg` absent.
- `Doctrine self-test` step precedes `Doctrine scan` step.
- `Publish doctrine reports` appends to `$GITHUB_STEP_SUMMARY` with `if: always()`.
- `Upload doctrine reports` uses `actions/upload-artifact@v4` with `if: always()`.

First workflow run: (pending — recorded after merge to `master`).

## Scope Ledger

| Path | Touched |
|---|---|
| `.github/workflows/doctrine-scan.yml` | yes (new) |
| `scripts/ci/README.md` | yes |
| `docs/tests/ci-a-workflow_results.md` | yes |
| `docs/tests/current_evidence_index.md` | yes |
| `docs/design_0_0_8_4_6_ci_scaffolding.md` | yes (PROBATION) |
| `crates/**`, `scripts/ci/scans.tsv`, scanners, allowlists, fixtures, triage | **no** |

## Known gaps / next

- Record first GitHub Actions run URL/conclusion after merge push to `master`.
- `CI-A-INSPECT-TRIAGE-0` — triage log, spam bounds, per-INSPECT justification slots.
- HEURISTIC delta-scoping on PR diff (design constraint; current workflow runs whole-tree `doctrine_scan.sh` as positive-control parity with local runner).

## DOCTRINE SELFTEST REPORT

```
DOCTRINE SELFTEST REPORT
  positive control: PASS
  known-bad: (16 cases) PASS
  heuristic controls: (4 cases) PASS
  traps: (6 cases) PASS
  rot test: PASS
DOCTRINE-SELFTEST-VERDICT: PASS
```

## DOCTRINE SCAN REPORT

```
DOCTRINE SCAN REPORT  (commit afbb801f19, 2026-07-01T00:09:51Z)
  scanner self-test: SKIPPED
  --- results ---
  B3-BUFFER-ESCAPE  PASS  0  design §5 B3 buffer escape
  FORGE-MINTERS  PASS  0  design §5 forge minters
  UNSAFE-FN  PASS  0  design §5 unsafe fn
  UNSAFE-ALLOW-ATTR  PASS  0  design §5 allow unsafe attr
  UNSAFE-FORBID-ATTR  PASS  0  design §5 forbid unsafe attr
  AS5-COLUMN-ALIAS  PASS  0  design §5 AS-5 ColumnIndex alias
  DENY-TOML-STUB  PASS  0  design §0.6.6 deny.toml stub
  RAW-DATA-INDEX  PASS  0  design §5 raw data[N] index
  SIM-KIND-READ  PASS  0  design §5 sim .kind read
  SEMANTIC-WORDS  PASS  0  design §5 semantic words below spec
  SPEC-STRING-CHANNEL  PASS  0  design §5 stringly channel identity
  ALLOW-SEALED-PRODUCERS  PASS  0  design §5 sealed producer allowlist
  ALLOW-BUFFER-HANDLES  PASS  0  design §5 buffer handle allowlist
  ALLOW-KERNEL-SURFACE  PASS  0  design §5 kernel surface allowlist
  --- summary ---
  hard failures: 0   inspect flags: 0   reliability: RELIABLE=hard FAIL; HEURISTIC=INSPECT only
DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED
```
