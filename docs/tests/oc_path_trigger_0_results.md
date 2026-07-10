# OC-PATH-TRIGGER-0 Results

## Status

**PROBATION / proof-present / DA-review-pending** — gate-wiring path triggers. Expected clearance: `DA-RESERVE(gate-wiring)`. Do not self-merge.

## Changed files

| Path | Role |
|---|---|
| `scripts/ci/anchor_triggers.tsv` | Path→domain table (14 rows) |
| `scripts/ci/doctrine_anchors.tsv` | Domain alignment + session heading |
| `scripts/ci/relay_lint.sh` | Path-primary + prose-secondary domains |
| `scripts/ci/clearance_check.sh` | GATE_WIRING_PATHS + REQUIRED-ANCHORS |
| `.github/workflows/clearance.yml` | workflow_dispatch clearance |
| `scripts/ci/clearance_comment.sh` | Sticky comment helper |
| `scripts/ci/fixtures/**` | Path-trigger + gate-wiring selftests |
| `docs/design_0_0_8_4_8_3_orientation_curation.md` | A2 in-progress stamp |
| `docs/orchestrator_orientation.md` | Regenerated |
| `docs/tests/oc_path_trigger_0_results.md` | This evidence |

## A1 repair

A1 already graduated on master (#1264 @ `b680d2e1`). A2 stamps design ladder A2 as in-progress; pointer remains `OC-PATH-TRIGGER-0`.

## Proof summary

- `anchor_triggers.tsv` rows: **14**
- Path domains: kernel/gpu/sim/wgsl/driver/mapeditor/mapgenerator/clausething/stead/adr/constitution/core + gate-wiring tables
- `REQUIRED-ANCHORS:` emitted on every DA-RESERVE (kernel fixture contains `seal-residue-cross-crate`)
- r1: anchors-tsv / triggers-tsv → `DA-RESERVE(gate-wiring)` PASS
- r2: `session-lifecycle-adr-family` → `heading:# Architecture Decision Records` PASS
- r3: `.github/workflows/clearance.yml` has `workflow_dispatch` `pr_number` + sticky comment + artifact
- `relay_lint --selftest` PASS (29)
- `clearance_check --selftest` PASS (78)
- `anchor_check --check` PASS

## agent_scan

`AGENT-SCAN-VERDICT: PASS delta_inspect=0` — `DOCTRINE-SCAN-VERDICT: PASS failures=0 inspect=0`

## Known gaps

A3 query/reach-log/resync · A4 orientation slice · A5 docs cascade — not implemented.

## Expected clearance

`DA-RESERVE(gate-wiring)`
