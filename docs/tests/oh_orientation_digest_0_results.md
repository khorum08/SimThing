# OH-ORIENTATION-DIGEST-0 Results

## Status

**PROBATION / proof-present / DA-review-pending — not self-mergeable.** M2 orientation digest; DA clearance required before graduation.

## PR / branch / merge

| Field | Value |
|---|---|
| PR | pending |
| Branch | `oh-orientation-digest-0` |
| Base | `master` @ `ad46a0be8` |
| Rung | `OH-ORIENTATION-DIGEST-0` |

## What changed

- Added `scripts/ci/gen_orientation.sh` — generates `docs/orchestrator_orientation.md` from live harness TSVs/design state; `--check` freshness gate; `--selftest` with two fixtures.
- Added generated `docs/orchestrator_orientation.md` with source stamps and operational contract sections.
- Wired orientation freshness into `.github/workflows/doctrine-scan.yml`.
- Added `/orient` (+ `role=orchestrator|coding|da`) on `doctrine-exec-commands.yml` via `doctrine_exec_orient.sh` + `doctrine_exec_orient_comment.sh`.
- Closure: flipped #1164 rows to DA-GRADUATED @ `ad46a0be8`.
- Ledgered three orientation fixture files in `test_inventory.tsv`.

## Load-bearing proofs

| Proof | Command / fixture | Catches |
|---|---|---|
| Generator | `bash scripts/ci/gen_orientation.sh` | Live TSV → digest |
| Freshness | `bash scripts/ci/gen_orientation.sh --check` | Stale hand-edit fails |
| Stale digest fixture | `orientation_digest_selftest_stale_digest` | Hand-edited digest fails --check |
| Live TSV change fixture | `orientation_digest_selftest_live_tsv_change` | Source drift requires regen |
| Doctrine self-test | `bash scripts/ci/doctrine_selftest.sh` | Inventory drift + positive control |
| Doctrine scan | `bash scripts/ci/doctrine_scan.sh` | CI integration |

### Post-merge /relay-lint smoke

- PR used: [#1164](https://github.com/khorum08/SimThing/pull/1164) (merged; head branch deleted)
- comment/run: [issuecomment-4888019870](https://github.com/khorum08/SimThing/pull/1164#issuecomment-4888019870) → workflow [28759107869](https://github.com/khorum08/SimThing/actions/runs/28759107869)
- result: **workflow FAIL** — checkout could not resolve deleted head branch `oh-clearance-router-0r-empty-diff` (post-merge branch deletion; not relay-lint logic failure)
- observed RELAY-LINT-VERDICT: n/a (workflow did not reach lint step)
- follow-up: re-post `/relay-lint` on this PR (live branch) once open

### Owner-local proof output (2026-07-05)

**gen_orientation.sh --selftest**
```
PASS orientation_digest_selftest_stale_digest
PASS orientation_digest_selftest_live_tsv_change
ORIENTATION-DIGEST-SELFTEST: PASS (2 fixtures)
```

**gen_orientation.sh --check**
```
gen_orientation --check: PASS
```

## Scope Ledger

| Path | Touched | Notes |
|---|---|---|
| `scripts/ci/gen_orientation.sh` | yes | M2 generator |
| `scripts/ci/doctrine_exec_orient*.sh` | yes | GHA helpers |
| `docs/orchestrator_orientation.md` | yes | generated digest |
| `scripts/ci/fixtures/orientation_digest/**` | yes | 2 selftest fixtures |
| `scripts/ci/test_inventory.tsv` | yes | +3 fixture rows |
| `.github/workflows/doctrine-scan.yml` | yes | freshness gate |
| `.github/workflows/doctrine-exec-commands.yml` | yes | `/orient` |
| `docs/tests/oh_clearance_router_0r_results.md` | yes | #1164 graduation |
| Engine crates | **no** | |

## Known gaps / next

- GHA Doctrine Scan + `/orient` smoke recorded below once settled.
- `OH-COLD-START-0` (rung 2b) not started — no ORIENT-RECEIPT, `--since`, or stubification in this rung.
- Pre-existing `SPEC-LOWERER-KIND-READ` INSPECT(415) unchanged.

## Graduation routing

| Field | Value |
|---|---|
| CI verdict | pending GHA |
| Triage entries | none |
| Risk class | gate-wiring |
| Falsification check | Hand-mutate `orchestrator_orientation.md` → `--check` FAIL; remove fixture ledger row → drift FAIL |
| Recommended posture | **deep** — generated governance surface + command wiring |