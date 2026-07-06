# OH-ANCHOR-INTEGRITY-0 Results

## Status

**PROBATION / proof-present / DA-review-pending** — doctrine anchors + ANCHOR-ACK admission; DA clearance required (gate-wiring).

## PR / branch / merge

| Field | Value |
|---|---|
| PR | [#1167](https://github.com/khorum08/SimThing/pull/1167) |
| Branch | `oh-anchor-integrity-0` |
| Head (implementation/proof) | `295ebd6afa5c865680df800611ad5dc83d5261e3` |
| Evidence docs head | pending docs-only commit (see note below) |
| Base | `master` @ `d5c76215e` (#1166 merge) |

> Evidence lifecycle rows refreshed by docs-only commit (implementation/proof head remains `295ebd6a`).
| Rung | `OH-ANCHOR-INTEGRITY-0` |

## Exit-proof columns

| Location | State |
|---|---|
| **#1166 design row** | DA-GRADUATED / merged #1166 @ `d5c76215e` |
| **#1166 evidence index** | OH-COLD-START-0 merged `d5c76215e` — DA-GRADUATED |
| **#1166 result doc closure** | OH-COLD-START-0 DA-GRADUATED / merged #1166 @ `d5c76215e0a80ac54c4d1e79d8d4165a39f1e94c` |

## What changed

- Added `scripts/ci/doctrine_anchors.tsv` — five seed anchors with exact canonical text hashes.
- Added `scripts/ci/anchor_check.sh` — `FAIL(missing-anchor|anchor-hash-drift|anchor-table)`; `--anchor-stamp` for receipt binding.
- Extended `scripts/ci/relay_lint.sh` — ANCHOR-ACK validation (`missing-anchor-ack`, `stale-anchor-ack`, `unknown-anchor`) keyed to `required_trigger_domains.txt`.
- Extended `scripts/ci/orient.sh` — `anchor_stamp` folded into `ORIENT-RECEIPT` (staleness on anchor edits).
- Added `/anchor` on `doctrine-exec-commands.yml` via `doctrine_exec_anchor.sh` + sticky comment helper.
- Extended `gen_orientation.sh` + regenerated `docs/orchestrator_orientation.md` with anchor sections.
- Added `docs/handoff_template.md` §10c `ANCHOR-ACK` slot (relay-lint enforced).
- Nine anchor-integrity fixture dirs (22 files), ledgered in `test_inventory.tsv`.
- Flipped #1166 exit-proof rows to DA-GRADUATED @ `d5c76215e` (gate 0).

## Load-bearing proofs

| Proof | Command / fixture | Catches |
|---|---|---|
| Anchor table | `bash scripts/ci/anchor_check.sh --check` | Hash drift / missing doc / malformed TSV |
| Anchor selftest | `bash scripts/ci/anchor_check.sh --selftest` | drift, missing, malformed, receipt-stamp drift |
| Coding receipt | `bash scripts/ci/orient.sh --role=coding` | Receipt + anchor_stamp emission |
| Freshness | `bash scripts/ci/gen_orientation.sh --check` | Stale orientation digest |
| Relay selftest | `bash scripts/ci/relay_lint.sh --selftest` | 15 fixtures incl. ANCHOR-ACK pass/fail |
| Valid gate-wiring ack | `anchor_integrity_selftest_pass_gate_wiring_ack` | All required ANCHOR-ACK lines |
| Missing ack | `anchor_integrity_selftest_fail_missing_ack` | `FAIL(missing-anchor-ack)` |
| Stale ack | `anchor_integrity_selftest_fail_stale_ack` | `FAIL(stale-anchor-ack)` |
| Unknown anchor | `anchor_integrity_selftest_fail_unknown_anchor` | `FAIL(unknown-anchor)` |
| Receipt staleness | `anchor_integrity_selftest_receipt_stales_on_anchor_change` | anchor_stamp drift |

### Owner-local proof output

```
anchor_check.sh --check: PASS
anchor_check.sh --selftest: PASS (5 fixtures)
relay_lint.sh --selftest: PASS (15 fixtures)
orient.sh --selftest: PASS (1 fixtures)
gen_orientation.sh --check: PASS
```

## Post-open /anchor smoke

- PR used: [#1167](https://github.com/khorum08/SimThing/pull/1167)
- comment/run: `/anchor movement-front` and `/anchor receipt-admission` posted on PR (comments [#4888440640](https://github.com/khorum08/SimThing/pull/1167#issuecomment-4888440640), [#4888440700](https://github.com/khorum08/SimThing/pull/1167#issuecomment-4888440700))
- Pre-merge `/anchor` GHA smoke is blocked because `issue_comment` workflows execute master's `doctrine-exec-commands.yml`, which lacks `/anchor` until #1167 merges. Owner-local `/anchor` proof substitutes pre-merge. Re-post `/anchor movement-front` and `/anchor receipt-admission` after merge.
- owner-local smoke: `doctrine_exec_anchor.sh movement-front` → `ANCHOR-REPORT: OK` (`movement-front@a0592b2f37ca`); `doctrine_exec_anchor.sh receipt-admission` → `ANCHOR-REPORT: OK` (`receipt-admission@7b886656d959`; anchor_id exact-match priority over trigger-domain collision)

## GHA CI (implementation/proof head `295ebd6a`)

| Check | Run | Verdict |
|---|---|---|
| Doctrine Exec | [28762948547](https://github.com/khorum08/SimThing/actions/runs/28762948547) | PASS (`DOCTRINE-EXEC-VERDICT: PASS`) |
| Doctrine Scan | [28762948587](https://github.com/khorum08/SimThing/actions/runs/28762948587) | PASS (incl. doctrine anchor integrity: PASS; orientation digest freshness: PASS; doctrine self-test: PASS; PR delta scan: PASS; triage spam check: PASS) |

## Scope Ledger

| Path | Classification | Notes |
|---|---|---|
| `scripts/ci/doctrine_anchors.tsv` | gate-wiring data | M6 anchor table |
| `scripts/ci/anchor_check.sh` | gate-wiring harness | hash verification |
| `scripts/ci/doctrine_exec_anchor*.sh` | gate-wiring harness | `/anchor` GHA |
| `scripts/ci/relay_lint.sh` | gate-wiring harness | ANCHOR-ACK validation |
| `scripts/ci/orient.sh` | gate-wiring harness | anchor_stamp in receipt |
| `scripts/ci/fixtures/anchor_integrity/**` | seal-proof fixtures | 9 dirs / 22 files |
| Engine crates | untouched | no engine edits |

## Known gaps / next

- Merge-hold active: DA/Owner clearance required (gate-wiring).
- Quote-verbatim scan on generated docs beyond anchor table deferred to tightening pass if INSPECT deltas appear.

## Graduation routing

| Field | Value |
|---|---|
| CI verdict | PASS-RELIABLE — Doctrine Scan and Doctrine Exec green on implementation head `295ebd6a` (runs 28762948547 / 28762948587) |
| Triage entries | none unless new INSPECT delta appears |
| Risk class | gate-wiring |
| Falsification check | edit anchored text / remove ack / wrong ack hash / unknown anchor id / anchor edit stale receipt |
| Recommended posture | deep — doctrine-anchor authority and admission wiring |