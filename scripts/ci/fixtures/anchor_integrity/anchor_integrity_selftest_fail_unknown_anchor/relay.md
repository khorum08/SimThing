ORIENT-RECEIPT: 9676741d29a1
role: coding
orientation_digest_sha: 98aca33cc2f0fe3d49e8c93899b3d7c74b60e8d93d92e796e835dd99d6114dbe
source_stamp: 4845f9f87f727322
generated_at: source-bound

ANCHOR-ACK: orientation-harness-core@8a365d1c0864
ANCHOR-ACK: handoff-template-admission@4799682fba31
ANCHOR-ACK: receipt-admission@7b886656d959
ANCHOR-ACK: nonexistent-anchor@deadbeef0000

## Status

**PROBATION / DA-OWNER REVIEW** — OH-ANCHOR-INTEGRITY-0 pending DA clearance of gate-wiring admission.

## PR / branch / merge

| Field | Value |
|---|---|
| PR | [#TBD](https://github.com/khorum08/SimThing/pull/TBD) |
| Branch | `oh-anchor-integrity-0` |
| Merge | held pending DA clearance |

## What changed

- Added `doctrine_anchors.tsv` seed rows for core design, handoff template, and receipt-admission spans.
- Added `anchor_check.sh` for table verification and anchor-stamp emission.
- Extended `relay_lint.sh` with ANCHOR-ACK validation keyed to trigger domains.

## Load-bearing proofs (+ what each catches)

| Proof | Catches |
|---|---|
| `anchor_check.sh --selftest` | Hash drift, missing anchors, malformed table |
| `relay_lint.sh --selftest` (anchor fixtures) | Missing/stale/unknown ANCHOR-ACK |
| `orient.sh --role=coding` | ORIENT-RECEIPT folds anchor_stamp |

```
DOCTRINE-TESTS-VERDICT: PASS
tested_code_sha: 5b03bfb1948d315b49a14a97cbe38f60ef08112d
coverage_basis: PASS — commits after tested_code_sha are docs/evidence-only and do not affect the tested binary
```

## Scope Ledger

| Path | Classification | Notes |
|---|---|---|
| `scripts/ci/doctrine_anchors.tsv` | gate-wiring admission | seed anchor rows |
| `scripts/ci/anchor_check.sh` | gate-wiring harness | table + stamp |
| `scripts/ci/relay_lint.sh` | gate-wiring harness | ANCHOR-ACK lint |

## Conformance (spine/D-directives held)

- Doctrine anchors quote-verbatim from pinned doc sections; no paraphrase admission.
- ANCHOR-ACK short hashes match live anchor_check state at relay lint time.

## Homing Boundary Classification

| Symbol | Classification | Action |
|---|---|---|
| `anchor_check.sh` | CI harness | keep in scripts/ci |
| `doctrine_anchors.tsv` | doctrine table | version with repo |

## Known gaps / next

- DA clearance required before graduation of rung 2c.

## Graduation routing

| Field | Value |
|---|---|
| CI verdict | PASS-RELIABLE — anchor + relay lint selftests green at head |
| Triage entries | none |
| Risk class | gate-wiring |
| Falsification check | `bash scripts/ci/anchor_check.sh --selftest` + `bash scripts/ci/relay_lint.sh --selftest` → anchor fixtures PASS |
| Recommended posture | deep — first doctrine anchor admission track |