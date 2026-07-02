# CI-B-WEBCHAT-PR1 Results

## Status

**PROBATION** — Track B webchat orchestration + GitHub-side CPU proof slice wired. No self-merge; DA/orchestrator clearance required.

## PR / branch / merge

- Branch: `ci-b-webchat-pr1`
- PR: [#1083](https://github.com/khorum08/SimThing/pull/1083)
- Merge: pending DA clearance

## What changed

- **Docs-only pass (Part 0):** `docs/ci_screening_surface.md` §9 (webchat orchestration quick-reference); `docs/design_0_0_8_4_6_ci_scaffolding.md` §3B.1 cites §9.
- **Workflow:** `.github/workflows/doctrine-exec.yml` — non-blocking CPU proof (`continue-on-error: true`); `.github/workflows/doctrine-exec-commands.yml` — `/seal-proof` and `/triage` command surfaces.
- **Scripts:** `doctrine_exec.sh`, `doctrine_exec_plan.sh`, `doctrine_exec_probes.sh`, `doctrine_surface_truth.sh`, `doctrine_exec_stale_check.sh`, `doctrine_exec_comment.sh`, `doctrine_exec_commands.sh`, `doctrine_exec_triage.sh`.
- **Data:** `doctrine_exec_profiles.tsv` (`ci-b-webchat-smoke`, `seal-residue`, `data-deliverable`, `timeout-finalize-proof`, `owner-deep-full-cpu-quarantined`); `kernel_public_api_baseline.txt` (cargo-public-api pub-line baseline).
- **Design:** §3B rungs 3–6 → **PROBATION** (`CI-B-WEBCHAT-PR1`).
- **Untouched:** `.github/workflows/doctrine-scan.yml`, Track A blocking gate, authority crate sources.

## CI-B-WEBCHAT-PR1R owner-edict repair

Owner edict: long full-battery tests must be vanishingly rare. Full-crate `cargo test -p <crate>` is forbidden in automatic PR-triggered, comment-triggered, and default doctrine-exec paths.

Cause of long run: the original Track B default resolved to `full-cpu`, whose profile ran broad per-crate `cargo test -p <crate>` commands. A small workflow edit could therefore launch a long repo-scale test battery before promptly emitting `DOCTRINE-EXEC-VERDICT`, `doctrine_exec_report.json`, and sticky-comment proof.

Repair:

- automatic PR profile changed from `full-cpu` to `ci-b-webchat-smoke`
- broad full-cpu profile renamed/quarantined as `owner-deep-full-cpu-quarantined`
- owner-deep profile requires explicit `workflow_dispatch` `owner_deep=true`
- `/seal-proof` default resolves to `ci-b-webchat-smoke`
- `/seal-proof profile=owner-deep-*` is rejected from comment/review-command paths
- `profile_class = smoke | targeted | probe | owner-deep` added to `doctrine_exec_profiles.tsv`
- profile lint rejects bare full-crate cargo tests in casual profiles
- per-command timeouts added
- fail-fast-with-finalize added for non-owner-deep profiles
- footer/artifact/sticky proof surface now appears even after fail/timeout

Local PR1R proof:

| Proof | Result |
|---|---|
| smoke/default profile | PASS - `DOCTRINE-EXEC-VERDICT: PASS`, `profile=ci-b-webchat-smoke`, `profile_class=smoke`, `owner_deep=false`, `doctrine_exec_report.json` written |
| profile lint reject | PASS - temp targeted profile containing `cargo test -p simthing-kernel` produced `PROFILE-LINT: FAIL` |
| owner-deep comment rejection | PASS - fake issue comment `/seal-proof profile=owner-deep-full-cpu-quarantined` produced `COMMAND: seal-proof-rejected` |
| owner-deep runner rejection | PASS - `owner-deep-full-cpu-quarantined` without `owner_deep=true` finalized `DOCTRINE-EXEC-VERDICT: FAIL` and wrote JSON |
| owner-deep dispatch authorization shape | PASS - `DOCTRINE_EXEC_OWNER_DEEP_ALLOWED=true doctrine_exec_plan.sh --profile owner-deep-full-cpu-quarantined` resolves the quarantined profile in plan mode without execution |
| timeout finalization | PASS - `timeout-finalize-proof` with `DOCTRINE_EXEC_COMMAND_TIMEOUT_SECONDS=1` finalized `DOCTRINE-EXEC-VERDICT: FAIL` and wrote JSON |
| probe finalization | PASS - `panic-swallow` probe finalized with `DOCTRINE-EXEC-VERDICT` and JSON |
| plan mode | PASS - `doctrine_exec_plan.sh --profile ci-b-webchat-smoke` printed resolved commands and ran nothing |
| stale check | PASS - matching `head_sha` passed; mismatched `head_sha` failed |

Live PR1R proof on final repaired head `346a7be716d60b91b1ffc5723acdb8aa09bc07b7`: doctrine-exec PASS in 9s (run 28596787324, job 84794183743) using `ci-b-webchat-smoke`, with Rust install steps skipped, `DOCTRINE-EXEC-VERDICT` emitted, `doctrine_exec_report.json` uploaded, stale check PASS, and sticky PR comment SHA-bound to final head with `owner_deep=false`. Doctrine-scan PASS in 1m9s (run 28596787019, job 84794182586). PR1R keeps posture **PROBATION** until DA/orchestrator clearance.

## Docs-only orchestration guidance pass

- `ci_screening_surface.md` has §9 Track B webchat orchestration section.
- `design_0_0_8_4_6_ci_scaffolding.md` §3B.1 cites that section as operator quick-reference.
- First commit is docs-only; no workflow/script/code in that commit.

## Workflow proof

| Proof | Status |
|---|---|
| `doctrine-exec.yml` present, non-blocking | wired — `continue-on-error: true` |
| `doctrine-scan.yml` unchanged | verified — no diff |
| Path-filter + `workflow_dispatch` triggers | wired |
| Profile-bound executable proof | wired via `doctrine_exec_profiles.tsv` + `doctrine_exec.sh`; default is `ci-b-webchat-smoke`, broad full-crate batteries are owner-deep only |
| GPU legs → INSPECT never PASS | wired via `run_inspect_cmd` + profile `gpu_required` |
| Live green on PR head | PR1R final repaired head `346a7be716d60b91b1ffc5723acdb8aa09bc07b7`: doctrine-exec PASS in 9s (run 28596787324, job 84794183743); doctrine-scan PASS in 1m9s (run 28596787019, job 84794182586); sticky comment SHA-bound to final head with profile `ci-b-webchat-smoke`, `owner_deep=false`; `doctrine_exec_report.json` uploaded. |
| Known-bad probe finalizes | local `panic-swallow` probe finalized with footer + JSON; broader workflow probe exercise remains DA/orchestrator optional |

## SHA freshness proof

- Reports carry `pr`, `head_sha`, `base_sha`, `tested_ref`, `workflow_run_id`, `job_id`, `merge_ref_status`.
- `doctrine_exec_stale_check.sh` rejects when `head_sha != current PR head`.
- Sticky comment + job summary + `doctrine_exec_report.json` generated from same run.

## Merge-ref proof

- Workflow fetches `refs/pull/<PR>/merge`; sets `merge_ref_status: PASS` when available, else `UNAVAILABLE`.
- `UNAVAILABLE` contributes to `INSPECT` verdict for merge-sensitive rungs in `doctrine_exec.sh`.

## Report artifact proof

- Uploads `doctrine_exec_report.json` with `artifact_version: doctrine-exec.v1`.
- Required fields: verdict, pr, head_sha, base_sha, tested_ref, merge_ref_status, workflow_run_id, job_id, commands, tests, failures, inspect entries, triage rows.
- Footer `DOCTRINE-EXEC-VERDICT:` is source of truth; JSON is generated mirror.

## Sticky comment proof

- `doctrine_exec_comment.sh` updates one comment marked `<!-- doctrine-exec-sticky -->`; creates if absent, PATCH if present.

## Command-channel proof

- `doctrine-exec-commands.yml` listens on `issue_comment`, `pull_request_review`, `pull_request_review_comment`.
- Collaborator-only (`OWNER|MEMBER|COLLABORATOR`); fork PRs ignored for write-token paths.
- Live automatic PR doctrine-exec proof passed on repaired head and updated the sticky comment. Additional manual comment-command exercises remain DA/orchestrator optional.

## Plan-mode proof

- `/seal-proof plan` / `/seal-proof plan profile=<id>` prints resolved commands via `doctrine_exec_plan.sh`; does not dispatch `doctrine-exec.yml` run steps.

## Probe-mode proof

| Probe | Expected |
|---|---|
| `compile-fail-seal-break` | known-bad doc test goes red |
| `invisible-pub-use` | surface-truth INSPECT divergence |
| `panic-swallow` | pattern detected (full TEST-PANIC-SWALLOW scan deferred PR-2) |
| `macro-expanded-seal-export` | fixture present for grep guard target |

## Triage command proof

- `/triage <scan-id> <delete|green|escalate> <reason>` appends §1A row via `doctrine_exec_triage.sh` and commits to PR branch.
- Malformed commands post `FORMAT: /triage <scan-id> <delete|green|escalate> <reason>`.

## Surface-truth proof

- `doctrine_surface_truth.sh` diffs `cargo +nightly public-api` pub-lines vs `kernel_public_api_baseline.txt`.
- Divergence → `SURFACE-TRUTH: INSPECT`; probe `invisible-pub-use` exercises seeded invisible export.

## Load-bearing validation

| Command | Result |
|---|---|
| `bash scripts/ci/doctrine_scan.sh` | PASS — `DOCTRINE-SCAN-VERDICT: PASS failures=0 inspect=0` |
| `bash scripts/ci/doctrine_selftest.sh` | PASS — `DOCTRINE-SELFTEST-VERDICT: PASS` |
| `bash scripts/ci/gen_digest.sh --check` | PASS — `gen_digest --check: PASS` |

## INSPECT / triage

- Triage rows for this PR: appended on `/triage` command (none pre-seeded).
- `merge_ref_status: UNAVAILABLE` → INSPECT for merge-sensitive batteries.
- GPU-required profile legs → INSPECT when skipped.

## Scope Ledger

| Path | Touched |
|---|---|
| `.github/workflows/doctrine-exec.yml` | yes (new) |
| `.github/workflows/doctrine-exec-commands.yml` | yes (new) |
| `.github/workflows/doctrine-scan.yml` | **no** |
| `scripts/ci/doctrine_exec*.sh` | yes |
| `scripts/ci/doctrine_surface_truth.sh` | yes |
| `scripts/ci/doctrine_exec_profiles.tsv` | yes |
| `scripts/ci/kernel_public_api_baseline.txt` | yes |
| `scripts/ci/fixtures/probes/` | yes |
| `docs/ci_screening_surface.md` | yes |
| `docs/design_0_0_8_4_6_ci_scaffolding.md` | yes |
| `docs/tests/ci_b_webchat_pr1_results.md` | yes |
| `docs/tests/current_evidence_index.md` | yes |
| `crates/simthing-kernel/**` | **no** |
| `crates/simthing-sim/**` | **no** |
| `crates/simthing-gpu/**` | **no** |
| `crates/simthing-driver/**` | **no** |

## Graduation routing

```
Graduation routing (for DA — why PROBATION, not COMPLETE):
  CI verdict:          PASS-RELIABLE | INSPECT(n) | FAIL
  Triage entries:      this PR's rows in scripts/ci/triage_log.tsv, or "none"
  Risk class:          gate-wiring + data-deliverable + workflow-orchestration surface
  Falsification check: Verify ci_screening_surface.md contains the Track B webchat orchestration operator section and the CI scaffolding design cites it; verify doctrine-exec is non-blocking and does not edit doctrine-scan.yml; verify SHA-bound stale-report rejection; verify merge-ref reporting; verify DOCTRINE-EXEC-VERDICT footer and doctrine_exec_report.json agree; verify /seal-proof and /triage work from issue_comment and PR review/comment events; verify plan mode runs nothing; verify seed probes go red on known-bads; verify surface-truth catches a seeded invisible pub use; verify triage rows are committed to triage_log.tsv and malformed triage is rejected; verify no crate authority/runtime code changed and no PR verdict labels were added.
  Recommended posture: deep — this PR changes the webchat orchestration proof surface and gate-state tooling; DA must prove the guard bites before merge.
```

## Known gaps / next

- `TEST-PANIC-SWALLOW` scan + adversarial evasion fixture corpus → PR-2.
- Live GitHub proofs (16-item minimum set) require workflow runs on the opened PR.
- `CI-B-ORCH-GUIDANCE-0` satisfied by §9 + §3B.1 cross-reference.
- Doctrine-scan sticky comment (§3B rung 6a) not in this PR — triage `/triage` remote surface only.
- Branch protection / auto-merge / CODEOWNERS unchanged per hard boundaries.
