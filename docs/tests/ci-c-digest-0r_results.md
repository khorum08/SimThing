# CI-C-DIGEST-0R Results

Date: 2026-07-01

## Status

**PROBATION** - pending live PR-head Doctrine Scan and DA/Owner review. C3 remains PROBATION. CF remains DEFERRED.

## What changed

- Added a `Sanctioned surface digest freshness` step to `.github/workflows/doctrine-scan.yml`.
- The step runs `bash scripts/ci/gen_digest.sh --check` on every PR and master push before report publication/upload.
- The digest freshness output is published in the job summary and uploaded as `doctrine-digest-report.txt`.
- Stale digest failures now include the remedy: run `bash scripts/ci/gen_digest.sh` and commit `docs/sanctioned_surface.md`.

No global scan definitions, global allowlists, crates, C3 addendum behavior, dashboards, metrics, or closeout docs were edited.

## Freshness enforcement proof

### Fresh digest

```bash
bash scripts/ci/gen_digest.sh --check
```

```text
gen_digest --check: PASS
```

### Stale digest

Proof method: temporarily appended one dirty line to `docs/sanctioned_surface.md`, ran the same global check path, then restored the file from a temp backup before continuing.

```bash
bash scripts/ci/gen_digest.sh --check
```

```text
STALE_PROOF_EXIT=1
gen_digest: docs/sanctioned_surface.md is stale; expected output written to C:\Users\mvorm\AppData\Local\Temp\tmp18wypev3.md; remedy: run `bash scripts/ci/gen_digest.sh` and commit docs/sanctioned_surface.md
```

This proves stale `docs/sanctioned_surface.md` hard-fails the exact command wired into CI, and that the check is global by default with no `--track-doc`.

## Load-bearing proofs

Local commands were run through Git Bash on Windows with bundled Codex Python and `rg` prepended to `PATH`, so the command under proof remained the requested `bash ...` form.

### `bash scripts/ci/doctrine_scan.sh`

Result: PASS

```text
DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED
```

### `bash -n scripts/ci/gen_digest.sh`

Result: PASS

```text
<no output; exit 0>
```

### `git diff -- scripts/ci/scans.tsv scripts/ci/allow`

Result: no output.

Global scan/allowlist data stayed byte-unchanged.

### `git diff --name-only master...HEAD`

Result:

```text
.github/workflows/doctrine-scan.yml
docs/ci_screening_surface.md
docs/design_0_0_8_4_6_ci_scaffolding.md
docs/tests/ci-c-digest-0r_results.md
docs/tests/current_evidence_index.md
scripts/ci/README.md
scripts/ci/gen_digest.sh
```

No `scripts/ci/scans.tsv`, `scripts/ci/allow/**`, `crates/**`, C3 addendum files, dashboards/metrics/coverage, or closeout docs are present.

## INSPECT / triage

No INSPECT fired in the local doctrine scan. No `scripts/ci/triage_log.tsv` row was added.

## Scope Ledger

| Path | Touched | Note |
|---|---|---|
| `.github/workflows/doctrine-scan.yml` | yes | CI now runs global digest freshness before report publication/upload |
| `scripts/ci/gen_digest.sh` | yes | stale failure now prints the regenerate remedy |
| `scripts/ci/README.md` | yes | documents local/CI-enforced global check |
| `docs/ci_screening_surface.md` | yes | authoritative workflow map includes digest freshness |
| `docs/design_0_0_8_4_6_ci_scaffolding.md` | yes | C2 row moves from HELD to PROBATION after 0R |
| `docs/tests/current_evidence_index.md` | yes | adds CI-C-DIGEST-0R evidence row |
| `docs/tests/ci-c-digest-0r_results.md` | yes | this evidence report |
| `scripts/ci/scans.tsv`, `scripts/ci/allow/**`, `crates/**`, C3 addendum files, dashboards/metrics/coverage, closeout docs | no | forbidden / untouched |

## Known gaps / next

- Live PR-head Doctrine Scan must pass after the workflow change before merge.
