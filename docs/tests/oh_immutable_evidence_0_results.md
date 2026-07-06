# OH-IMMUTABLE-EVIDENCE-0 Results

## Status

**PROBATION / proof-present / DA-review-pending** — LIVE-POINTER lint, selftest decouple, merge-commit checkout fallback, hash normalization, and prose retirement; DA clearance required (gate-wiring).

## PR / branch / merge

| Field | Value |
|---|---|
| PR | pending |
| State | PROBATION / proof-present / DA-review-pending |
| Rung | `OH-IMMUTABLE-EVIDENCE-0` (2cR) |

## What changed

- Decoupled `orient.sh` and `relay_lint.sh` receipt selftests from live `orchestrator_orientation.md` via fixture-local `orientation_snapshot.md` + `orientation_state.txt`.
- Added `LIVE-POINTER` relay-lint verdict (`FAIL(live-pointer: <field>)`) with three fixtures.
- Swept `docs/tests/**_results.md` to remove forbidden live-pointer fields (`current_pr_head`, branch-tip head rows, docs-only head citations).
- `doctrine-exec-commands.yml`: `checkout_ref` falls back to `merge_commit_sha` on merged PRs for read-only command jobs.
- Hash normalization (BOM strip + CRLF→LF) in `anchor_check.sh`, `orient.sh`, `relay_lint.sh`, `gen_orientation.sh`.
- Retired §2 SHA-routing prose and `ci_screening_surface.md` §5A SHA-hygiene paragraph; recorded in §6 sunset ledger.
- CI-gated `orient.sh --selftest` and `relay_lint.sh --selftest` in `doctrine-scan.yml`.

## Load-bearing proofs

| Proof | Command / fixture | Catches |
|---|---|---|
| Orient selftest | `bash scripts/ci/orient.sh --selftest` | Receipt emission against fixture snapshot |
| Relay selftest | `bash scripts/ci/relay_lint.sh --selftest` | 18 fixtures incl. LIVE-POINTER fail/pass |
| Live pointer | `relay_lint_selftest_fail_live_pointer_*` | `FAIL(live-pointer: …)` |
| Anchor hash stability | `anchor_check.sh --check` | BOM/CRLF-normalized content hashes |
| Orientation freshness | `gen_orientation.sh --check` | Generated digest current |
| Digest freshness | `gen_digest.sh --check` | Sanctioned surface digest |

### Owner-local proof output

```
orient.sh --selftest: PASS
relay_lint.sh --selftest: PASS (18 fixtures)
anchor_check.sh --check: PASS
anchor_check.sh --selftest: PASS
gen_orientation.sh --check: PASS
gen_digest.sh --check: PASS
doctrine_selftest.sh: PASS
doctrine_scan.sh: INSPECT failures=0
```

## Falsification checks

| Mutation | Expected |
|---|---|
| Unrelated design-doc edit + `gen_orientation.sh` | `orient.sh --selftest` PASS; `relay_lint.sh --selftest` PASS |
| Relay with `current_pr_head:` | `FAIL(live-pointer: current_pr_head)` |
| Results doc with `live/docs-refresh head` | `FAIL(live-pointer: live/docs-refresh head)` |
| CRLF/BOM re-encoding of anchor text | Identical `content_hash` |

## Prose retirement proof

| Retired | Enforcing surface | Ledger |
|---|---|---|
| §2 no-SHA-equality routing prose | LIVE-POINTER + clearance tested-code-SHA | design §6 |
| §2 Immutability Law long argument | LIVE-POINTER + fixture snapshots | design §6 |
| ci_screening_surface.md §5A SHA-hygiene | LIVE-POINTER + clearance tested-code-SHA | design §6 |
| post-merge smoke-on-next-open-PR | merge-commit `checkout_ref` fallback | design §6 |

## Scope Ledger

| Path | Classification |
|---|---|
| `scripts/ci/orient.sh`, `relay_lint.sh`, `anchor_check.sh` | gate-wiring harness |
| `scripts/ci/fixtures/cold_start/**`, `relay_lint/**` | seal-proof fixtures |
| `.github/workflows/doctrine-scan.yml`, `doctrine-exec-commands.yml` | gate-wiring CI |
| `docs/design_0_0_8_4_7_orchestration_harness.md`, `ci_screening_surface.md` | prose retirement |
| Engine crates | untouched |

## Known gaps / next

- Merge-hold active: DA/Owner clearance required (gate-wiring).
- Next after DA clearance: `OH-TRIAGE-INDUCTION-0`.

## Graduation routing

| Field | Value |
|---|---|
| CI verdict | PASS-RELIABLE — GHA green on proof-bearing work |
| Triage entries | none |
| Risk class | gate-wiring |
| Falsification check | design edit + orient regen does not break selftests; live-pointer fields rejected |
| Recommended posture | deep — immutable-evidence admission and prose-retirement audit |