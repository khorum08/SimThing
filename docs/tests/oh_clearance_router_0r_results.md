# OH-CLEARANCE-ROUTER-0R Results

## Status

**PROBATION / gate-wiring — not self-mergeable.** Remedial router precision fix; DA clearance required before graduation.

## PR / branch / merge

| Field | Value |
|---|---|
| PR | pending |
| Branch | `oh-clearance-router-0r-empty-diff` |
| Base | `master` @ `d4969f1c8` |
| Rung | `OH-CLEARANCE-ROUTER-0R` |

## What changed

- `scripts/ci/clearance_check.sh` — empty requested diff (PR/range/fixture target) routes `DA-RESERVE(harness-error)` instead of `DA-RESERVE(novelty)`; novelty reserved for resolved non-empty diffs with no precedented class match.
- Bare `bash scripts/ci/clearance_check.sh <pr-number>` resolves via `gh pr diff --name-only` (API fallback); hard-errors with `--range` remedy when local resolution fails.
- Added selftest fixture `clearance_selftest_fail_closed_empty_requested_diff` with `target_mode.txt` marker.
- Ledgered three new fixture files in `test_inventory.tsv`.

## Load-bearing proofs

| Proof | Command / fixture | Catches |
|---|---|---|
| Empty requested diff | `clearance_selftest_fail_closed_empty_requested_diff` | Empty changed-files after target resolution must not fall through to novelty |
| Router selftest battery | `bash scripts/ci/clearance_check.sh --selftest` | All ten regression classes including empty-diff precision |
| #1163 bare PR | `bash scripts/ci/clearance_check.sh 1163` | Local PR-number resolves or hard-errors; never silent empty → novelty |
| Router surface range | `bash scripts/ci/clearance_check.sh --range <base>..<head>` | Self-application on gate-wiring paths |
| Inventory drift | `doctrine_selftest.sh` → `inventory drift proof: PASS` | Unledgered clearance fixtures cannot break Doctrine self-test |

### Proof output (owner-local, 2026-07-05)

**clearance_check.sh --selftest**
```
PASS clearance_selftest_clearable_1150_shape
PASS clearance_selftest_clearable_1151_shape
PASS clearance_selftest_clearable_1152_shape
PASS clearance_selftest_reserve_1154_binding_conditions
PASS clearance_selftest_fail_closed_malformed_tsv
PASS clearance_selftest_fail_closed_ambiguous_class
PASS clearance_selftest_gate_wiring_self_application
PASS clearance_selftest_suspended_class
PASS clearance_selftest_missing_required_proof_fields
PASS clearance_selftest_fail_closed_empty_requested_diff
CLEARANCE-VERDICT: ORCHESTRATOR-CLEARABLE
CLEARANCE-SELFTEST: PASS (10 fixtures)
```

**clearance_check.sh 1163**
```
CLEARANCE-VERDICT: DA-RESERVE(gate-wiring)
```

**clearance_check.sh --range (router surface)**
```
CLEARANCE-VERDICT: DA-RESERVE(gate-wiring)
```

**doctrine_selftest.sh**
```
inventory drift proof: PASS
DOCTRINE-SELFTEST-VERDICT: PASS
```

**gen_digest.sh --check**
```
gen_digest --check: PASS
```

**doctrine_scan.sh**
```
TEST-INVENTORY-DRIFT  PASS  0
DOCTRINE-SCAN-VERDICT: INSPECT  failures=0 inspect=415 selftest=SKIPPED
```

## Scope Ledger

| Path | Touched | Notes |
|---|---|---|
| `scripts/ci/clearance_check.sh` | yes | empty-diff + PR resolution precision |
| `scripts/ci/fixtures/clearance/clearance_selftest_fail_closed_empty_requested_diff/**` | yes | +1 fixture (3 files) |
| `scripts/ci/test_inventory.tsv` | yes | +3 ledger rows |
| `docs/tests/oh_clearance_router_0r_results.md` | yes | this evidence doc |
| `docs/tests/current_evidence_index.md` | yes | 0R PROBATION row + relay-lint graduation |
| `docs/design_0_0_8_4_7_orchestration_harness.md` | yes | rung 1 graduation + 1R row |
| Engine crates | **no** | |

## Known gaps / next

- GHA post-push results recorded below once settled.
- Pre-existing `SPEC-LOWERER-KIND-READ` INSPECT(415) unchanged — not introduced by this PR.
- `OH-ORIENTATION-DIGEST-0` (rung 2) not started.

## Graduation routing

| Field | Value |
|---|---|
| CI verdict | pending GHA |
| Triage entries | none |
| Risk class | gate-wiring |
| Falsification check | Remove `target_mode.txt` from empty-diff fixture → novelty regression; remove one ledger row → drift gate FAIL |
| Recommended posture | **deep** — router precision and self-application surface |