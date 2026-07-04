# SURFACE-TRUTH-INSPECT-REASON-SPLIT-0 Results

## Status

**PROBATION / DA REVIEW**. Merge not authorized for Grok.

## Mission

Split surface-truth INSPECT reporting into machine-readable `match` / `divergence` / `tooling-gap` reasons so Doctrine Exec triage can distinguish real public API divergence from tooling/environment gaps. This rung does not change the authority of surface-truth checks. It only splits INSPECT reason reporting so triage can distinguish real public API divergence from a tooling/environment gap.

## Scope

In scope:

- `scripts/ci/doctrine_surface_truth.sh` — emit `SURFACE-TRUTH-REASON` on every path
- `scripts/ci/doctrine_surface_truth_inspect.sh` — shared Doctrine Exec inspect-line mapping
- `scripts/ci/doctrine_exec.sh` — preserve specific inspect reason text
- `scripts/ci/doctrine_exec_probes.sh` — `invisible-pub-use` checks `SURFACE-TRUTH-REASON: divergence`
- `scripts/ci/doctrine_surface_truth_reason_test.sh` — lightweight reason-split proof
- Design/evidence index updates; D2x closure; D2y row

Out of scope:

- `crates/**`, inventory/audit/boundary ledgers, `doctrine_exec_profiles.tsv`, allowlists, workflows
- Installing `cargo-public-api`, baseline edits, public API semantics changes

## Behavior before

- `doctrine_surface_truth.sh` emitted `SURFACE-TRUTH: PASS` or `SURFACE-TRUTH: INSPECT` without a stable reason line.
- `doctrine_exec.sh` collapsed all INSPECT cases into `surface-truth divergence or tooling gap`.

## Behavior after

| Path | `SURFACE-TRUTH` | `SURFACE-TRUTH-REASON` | Doctrine Exec `inspect_details` |
|---|---|---|---|
| PASS | `PASS public API matches baseline` | `match` | (none) |
| Real divergence | `INSPECT public API diverges from baseline` | `divergence` | `surface-truth divergence` |
| Missing `cargo-public-api` | `INSPECT cargo-public-api not installed` | `tooling-gap` | `surface-truth tooling-gap` |
| Missing baseline | `INSPECT missing baseline …` | `tooling-gap` | `surface-truth tooling-gap` |
| Empty current listing | `INSPECT empty current public API listing` | `tooling-gap` | `surface-truth tooling-gap` |
| Unrecognized/missing reason | `INSPECT …` | (absent) | `surface-truth inspect unknown-reason` |

PASS/INSPECT authority unchanged. Tooling-gap remains INSPECT; divergence is never downgraded.

## Reason taxonomy

Exactly three reason strings:

- `match`
- `divergence`
- `tooling-gap`

Test-only synthetic probes (`synthetic-match`, `synthetic-divergence`, `synthetic-tooling-gap-*`) require `DOCTRINE_SURFACE_TRUTH_SYNTHETIC_ALLOWED=1` and run before the `cargo-public-api` availability check so offline reason-split proof does not depend on toolchain installs.

## 0R remediation

The initial implementation handled synthetic probes after the `cargo-public-api` availability check, so synthetic proof still depended on `cargo-public-api`. 0R moves/guards synthetic test modes so offline reason-split proof works without `cargo-public-api` while keeping production surface-truth behavior unchanged. Synthetic modes require `DOCTRINE_SURFACE_TRUTH_SYNTHETIC_ALLOWED=1` and are test-only.

## Proof

Recorded on branch `grok/surface-truth-inspect-reason-split-0` (base `0df0be7e18`):

- `bash scripts/ci/doctrine_surface_truth_reason_test.sh`: PASS (11 cases; guarded offline synthetic proof with `PATH` excluding `cargo-public-api`)
- `bash scripts/ci/doctrine_scan.sh`: PASS `failures=0 inspect=0`
- `bash scripts/ci/gen_digest.sh --check`: PASS
- `git diff --check origin/master...HEAD`: PASS
- `DOCTRINE_EXEC_MODE=plan DOCTRINE_EXEC_PROFILE=ci-b-webchat-smoke bash scripts/ci/doctrine_exec.sh`: PASS

Initial implementation proof (pre-0R): Doctrine Scan PASS, reason-test PASS, plan smoke PASS.

Forbidden proof avoided: no broad `cargo test`, no owner-deep profiles, no `workflow_dispatch`, no Bevy/desktop/GPU proof.

## Files changed

- `scripts/ci/doctrine_surface_truth.sh`
- `scripts/ci/doctrine_surface_truth_inspect.sh` (new)
- `scripts/ci/doctrine_exec.sh`
- `scripts/ci/doctrine_exec_probes.sh`
- `scripts/ci/doctrine_surface_truth_reason_test.sh` (new)
- `docs/design_0_0_8_4_6_ci_scaffolding.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/surface_truth_inspect_reason_split_0_results.md` (this file)

## Graduation routing

- Risk class: CI proof semantics / gate-state / DA-held
- Protected corpus touched: no
- CI/gate/profile/scanner/allowlist/workflow touched: no (script semantics only)
- DA question: Does Opus accept the reason taxonomy and Doctrine Exec report semantics?
- Not orchestrator-clearable