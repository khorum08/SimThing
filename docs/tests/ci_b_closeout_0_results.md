# CI-B-CLOSEOUT-0 Results

## Status

**DA-CLOSED (2026-07-04)** — Track B doctrine/status closeout, **graduated by DA/Owner directive** after full acceptance-gate review; merged to master via #1133. Docs-only. The Track B local executable-proof ladder (`CI-B-TRACK-OPEN-0` → `CI-B-LOCAL-HARNESS-0` → `CI-B-TRIPWIRE-TAGS-0` → `CI-B-CLOSEOUT-0`) is closed; the owner-local executable-proof contract and citation rule are the standing authority.

### DA graduation verdict

All 12 acceptance gates met; diff is 4 docs only (no scripts/workflows/product/cargo); every proof gate green and live CI `doctrine-scan` PASS. #1129 and #1132 recorded DONE/DA-APPROVED; #1129 stale evidence-index line fixed; #1132 non-blocking refinements ledgered (§ below); result doc honest. Graduated at Owner's explicit "graduate-merge" directive, which is the DA/Owner clearance the closeout handoff reserved for this action.

## Identity

| Field | Value |
|---|---|
| Rung ID | `CI-B-CLOSEOUT-0` |
| Track | 0.0.8.4.6 CI Scaffolding / Track B |
| Branch | `ci-b-closeout-0` |
| Base | `origin/master` @ `d1b75c57f35b4b891840bc0c15b953fc6258b8e3` (post-#1132 merge) |
| Head | Proof run at branch tip; final PR head verified by orchestrator/DA. |
| Merge rule | not self-mergeable |

## Predecessor merges

| Rung | Merge commit |
|---|---|
| `CI-B-LOCAL-HARNESS-0` (#1129) | `16845b7a6104b860042775a2456aef053770d08a` |
| `CI-B-TRIPWIRE-TAGS-0` (#1132) | `d1b75c57f35b4b891840bc0c15b953fc6258b8e3` |

## Files changed

- `docs/design_0_0_8_4_6_ci_scaffolding.md`
- `docs/ci_screening_surface.md`
- `docs/tests/current_evidence_index.md`
- `docs/tests/ci_b_closeout_0_results.md`

## Track B executable-proof contract (final)

Track B is the executable-proof layer, kept strictly separate from Track A (no-toolchain / no-build / no-cargo grep+allowlist gate). The final split:

- **GitHub-side CPU proof** (Doctrine Exec, §3B) is **nonblocking** and is the **citable** path for CPU proof classes. Full-crate batteries stay quarantined behind owner-deep `workflow_dispatch`.
- **GPU / Bevy / Studio-`simthing-mapeditor` / desktop-typeface / real-adapter parity proof** is **owner-local-only**. GHA must not run those binaries or install desktop/GPU dependencies (enforced by `doctrine_exec_gha_proof_seal.sh` via `doctrine_exec_profile_lint.sh`).
- The owner-local harness `scripts/ci/doctrine_tests.sh` emits the §1 report with the strict `DOCTRINE-TESTS-VERDICT` footer and explicit `--- tripwire-tags ---`. Skipped/unverified owner-local proof is **INSPECT, never a silent PASS** (#1132, DA-verified on a real no-GPU host).

## Owner-local proof citation contract

GPU/Bevy/Desktop proof remains owner-local-only, but a fresh owner-local `DOCTRINE-TESTS-VERDICT: PASS` report is **citable validation** for the owner-local-only proof class when a GitHub-side check, Doctrine Exec report, webchat orchestrator, or DA review needs proof of a class GitHub is structurally unable or forbidden to execute.

A report is citable **iff** all hold:

| Requirement | Rule |
|---|---|
| `head_sha` | matches the current PR head SHA (stale/mismatched → rejected) |
| profile | the tested profile is named in the footer |
| `owner_local=true` | present in the footer |
| strict footer | `DOCTRINE-TESTS-VERDICT: PASS failures=N inspect=N profile=<id> owner_local=true head_sha=<sha>` preserved |
| verdict | `PASS` (INSPECT is never validation) |
| PASS tripwire tag | at least one of `COMPILE_FAIL_PROVEN` / `PARITY_BIT_EXACT` / `OWNER_LOCAL_PASS` for the relevant class |

Citation of owner-local PASS proof does **not** license GHA-side execution of GPU/Bevy/Desktop probes. It only recognizes a fresh owner-local green report as evidence for a class GitHub cannot run. The GitHub-side CPU lane and the owner-local lane never merge.

## DA non-blocking refinements recorded from #1132

Recorded as future refinements, not blockers (all fail closed — never toward a false PASS):

1. **Multi-command PASS tag from `commands[0]`.** A multi-command PASS emits a single proof-class tag derived from the first command. Acceptable for a mechanics rung; a future refinement may add per-command / aggregate proof-class tags.
2. **`run_band` not wired in the live execute path.** `FLAKY` / `PERF_VARIANCE` are proven synthetically only; the real `--profile` path does not yet perform multi-run banding. Acceptable for mechanics; a future refinement may wire real banding.
3. **Fail-closed empty-tag edge.** An INSPECT caused only by a resolver note on a desktop-only command set (no GPU legs, prereq ok) could produce empty tags and flip to FAIL. Safe (fails closed, never false PASS); not reachable with the current inventory (GPU legs always present). A future refinement may seed a generic INSPECT tag on any INSPECT path.
4. **`datetime.utcnow()` DeprecationWarning.** Cosmetic; a future small cleanup may switch to timezone-aware datetime.
5. **Stale #1129 evidence-index status.** Fixed in this rung (now `DONE — DA-APPROVED / merged #1129`).

## Proof

Proof environment: Windows Git Bash; `PYTHON_BIN` set explicitly to the local Python 3.12 (the harness/gate scripts fall back from `python3` to `python`; `gen_digest.sh` reads neither and needs Python on PATH — see note).

| Command | Result |
|---|---|
| `doctrine_scan.sh` | `DOCTRINE-SCAN-VERDICT: PASS failures=0 inspect=0 selftest=SKIPPED` |
| `gen_digest.sh --check` | `gen_digest --check: PASS` (also re-run green by the live `doctrine-scan` CI check on PR #1133) |
| `doctrine_exec_profile_lint.sh` | `PROFILE-LINT: PASS profiles=6 default=ci-b-webchat-smoke` + `GHA-PROOF-SEAL: PASS profiles=6` |
| `doctrine_exec_profile_lint.sh --prove-no-track-d-deletion-profiles` | `NO-TRACK-D-PROFILE-PROVE: PASS` |
| `git diff --check origin/master...HEAD` | clean (no output) |

Live CI on PR #1133: `doctrine-scan` PASS (1m2s, run 28713981790).

## Scope ledger

| Item | Touched? |
|---|---|
| product code | no |
| workflows (`.github/**`) | no |
| `scripts/ci/doctrine_tests.sh` | no |
| `scripts/ci/doctrine_tests_profiles.tsv` | no |
| `scripts/ci/doctrine_exec_profiles.tsv` | no |
| lifecycle expiry files | no |
| `test_inventory.tsv` / inventory checks | no |
| scans / allowlists | no |
| cargo / workspace test run | no |
| auto-deletion | no |
| semantic note-truth scan | no |
| ChatOps command | no |

## Graduation routing

- CI-B-CLOSEOUT-0 complete (docs/status closeout)
- PROBATION / DA-OWNER REVIEW
- DA/Owner clearance required
- not self-mergeable
- next step after clearance: Track B CLOSED; resume the downstream 0.0.8.5 production queue.
