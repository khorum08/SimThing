# OC-QUERY-0 Results

## Status

**PROBATION / proof-present / DA-review-pending** — gate-wiring query/resync surface.
Expected clearance: `DA-RESERVE(gate-wiring)`. Do not self-merge.

## Changed files

| Path | Role |
|---|---|
| `scripts/ci/anchor_query.sh` | `--domain` / `--paths` / `--grep` / `--prune` / `--selftest` |
| `scripts/ci/anchor_reach_log.tsv` | Append-only reach log (observability only) |
| `scripts/ci/anchor_check.sh` | `--resync` + FAIL remedies naming `--resync` |
| `scripts/ci/relay_lint.sh` | FAIL-as-teacher remedies → `anchor_query.sh` |
| `scripts/ci/clearance_check.sh` | GATE_WIRING_PATHS for query/reach-log/check |
| `scripts/ci/fixtures/**` | Expected verdicts + resync selftests |
| `docs/design_0_0_8_4_8_3_orientation_curation.md` | A3 IN PROGRESS stamp |
| `docs/orchestrator_orientation.md` | Regenerated |
| `docs/tests/oc_query_0_results.md` | This evidence |

## A2 graduation / A3 stamp

A2 graduated [#1266](https://github.com/khorum08/SimThing/pull/1266) @ `a42afa1f`.
A3 active in this PR; pointer `OC-QUERY-0` until merge.

## Modes / decay / resync

- `--domain gate-wiring` → `orientation-harness-core`
- `--paths` kernel → seal/admission anchors; wgsl → field-policy + eml
- `--grep` library-only; miss → `hit=none`, exit 0
- Reach-log header: `date role query anchors_served hit`; `--prune <days>` (closeout 30d)
- `--resync`: RESYNCED / ORPHANED+nearest; never silent drop; nonzero while orphans

## FAIL-as-teacher

missing-ack → `anchor_query.sh --paths`; unknown → `--domain`/`--grep`;
hash-drift → `anchor_check.sh --resync`; orphan → repair/`--resync`.

## Selftests / proofs

`anchor_query --selftest` PASS (8) · `anchor_check --selftest` PASS (7) ·
`relay_lint --selftest` PASS (29) · `clearance_check --selftest` PASS (78) ·
`anchor_check --check` PASS · `gen_orientation --check` PASS ·
`doc_budget_check` PASS · `test_inventory_drift_check` PASS ·
`agent_scan` PASS `delta_inspect=0`

## Known gaps

A4 orientation slice · A5 docs cascade · closeout — not implemented.
No Lane B / no parked 0.0.8.6 product · no reach-log gating · no new verdicts.

## Expected clearance

`DA-RESERVE(gate-wiring)`
