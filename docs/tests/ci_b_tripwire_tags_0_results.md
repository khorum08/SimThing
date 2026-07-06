# CI-B-TRIPWIRE-TAGS-0 Results

## Status

**PROBATION / DA-OWNER REVIEW** — executable-specific tripwire tags for owner-local harness. Not self-mergeable; DA/Owner clearance required.

## Identity

| Field | Value |
|---|---|
| PR | [#1132](https://github.com/khorum08/SimThing/pull/1132) |
| Branch | `ci-b-tripwire-tags-0` |
| Base | `origin/master` @ `16845b7a6104b860042775a2456aef053770d08a` (post-#1129 merge) |
| Proof | tested_code_sha binding per coverage_basis |

## Files changed

- `scripts/ci/doctrine_tests.sh`
- `docs/design_0_0_8_4_6_ci_scaffolding.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/ci_b_tripwire_tags_0_results.md`

## Implemented tag contract

Reports include `--- tripwire-tags ---` with stable names:

| Tag | Verdict when set |
|---|---|
| `GPU_SKIPPED` | INSPECT |
| `BEVY_SKIPPED` | INSPECT |
| `DESKTOP_SKIPPED` | INSPECT |
| `OWNER_PREREQ_MISSING` | INSPECT |
| `PLAN_ONLY` | INSPECT |
| `GITHUB_ACTIONS_REFUSAL` | INSPECT |
| `NO_LIVE_LOCAL_PROOF_TARGET` | INSPECT |
| `FLAKY` | INSPECT (+ run-band evidence) |
| `PERF_VARIANCE` | INSPECT (+ run-band evidence) |
| `COMPILE_FAIL_PROVEN` | PASS |
| `PARITY_BIT_EXACT` | PASS |
| `OWNER_LOCAL_PASS` | PASS |

Strict footer preserved: `DOCTRINE-TESTS-VERDICT: PASS|FAIL|INSPECT failures=N inspect=N profile=<id> owner_local=true head_sha=<sha>`.

`--prove-report` validates footer, profile TSV, GHA overlap, all tag paths, and rejects PASS without PASS tags.

## Proof

Proof environment: Windows Git Bash; `PYTHON_BIN` set explicitly.

| Command | Result |
|---|---|
| `bash -n scripts/ci/doctrine_tests.sh` | PASS |
| `--list` | PASS |
| `--plan --profile owner-local-gpu-bevy` | INSPECT; tags: PLAN_ONLY, GPU_SKIPPED, BEVY_SKIPPED, DESKTOP_SKIPPED, OWNER_PREREQ_MISSING |
| `--prove-report` | PASS |
| `--profile owner-local-gpu-bevy` | INSPECT; tags: GPU_SKIPPED, BEVY_SKIPPED, DESKTOP_SKIPPED, OWNER_PREREQ_MISSING |
| `doctrine_scan.sh` | PASS failures=0 inspect=0 |
| `gen_digest.sh --check` | PASS |
| `doctrine_exec_profile_lint.sh` | PASS |
| `--prove-no-track-d-deletion-profiles` | PASS |
| `git diff --check origin/master...HEAD` | PASS |

## Scope ledger

| Item | Touched? |
|---|---|
| product code | no |
| workflows | no |
| doctrine_exec_profiles.tsv | no |
| lifecycle expiry files | no |
| test_inventory.tsv | no |
| scans/allowlists | no |
| cargo/workspace test run | no |
| auto-deletion | no |
| semantic note-truth scan | no |
| ChatOps command | no |

## Graduation routing

- CI-B-TRIPWIRE-TAGS-0 complete
- PROBATION / DA-OWNER REVIEW
- DA/Owner clearance required
- not self-mergeable
- next rung after clearance: CI-B-CLOSEOUT-0