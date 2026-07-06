# OH-DOCS-SUNSET-0 Results

## Status

**PROBATION / proof-present / DA-review-pending** — mechanized prose compressed to enforcing-surface pointers; DOC-BUDGET, rule-expiry, AGENTS stub scan, Harness Extension Protocol §7, cold-start entrypoint, and closeout telemetry landed; DA clearance required (gate-wiring).

## PR / branch / merge

| Field | Value |
|---|---|
| PR | [#1173](https://github.com/khorum08/SimThing/pull/1173) |
| Rung | `OH-DOCS-SUNSET-0` (rung 4 / closing) |
| Risk | gate-wiring |

## Gate 0 — #1172 closure

| Location | State |
|---|---|
| design rung 3 | **DA-GRADUATED / merged #1172 @ `d81c7161cba7f6ceae9102933479345118f9879a`** |
| evidence index | OH-TRIAGE-INDUCTION-0 DA-GRADUATED |
| `oh_triage_induction_0_results.md` | DA-GRADUATED closure row |

## What changed

- Compressed mechanized §5A/§1/§5/§12 prose in `ci_screening_surface.md` to enforcing-surface pointers.
- Compressed design §3 M4 operator prose; added §7 Harness Extension Protocol (nine steps) + cold-start entrypoint.
- Added `doc_budget_check.sh` + `doc_budget_baseline.tsv` (DOC-BUDGET tripwire).
- Added `rule_expiry_check.sh` (expiry candidates + retired-prose orphan detection).
- Added `agents_stub_check.sh` + root `AGENTS.md` (≤5-line pointer stub).
- Wired DOC-BUDGET, RULE-EXPIRY, AGENTS-STUB into `doctrine_scan.sh` and `doctrine-scan.yml`.
- Regenerated `docs/orchestrator_orientation.md` with cold-start entrypoint section.
- §6 sunset ledger rows for all compressed prose blocks.

## Net-negative proof

| File | Before | After | Delta |
|---|---|---|---|
| `docs/ci_screening_surface.md` | 630 | 550 | −80 |
| `docs/design_0_0_8_4_7_orchestration_harness.md` | 208 | 231 | +23 (§7 Protocol + sunset rows; outweighed by ci_screening compression) |

## Closeout telemetry (`clearance_ledger.tsv`)

| Metric | Count |
|---|---|
| ORCHESTRATOR-CLEARABLE | 0 |
| DA-RESERVE | 0 |
| FAIL | 0 |
| §5.1-sketch rows | 0 |
| Ledger data rows | 0 |

## LED verdict

**LED-VERDICT: hold** — insufficient §5.1-sketch ledger rows (0 with-sketch vs 0 without) for evidence-based promote/retire comparison at closeout.

## Doctrine anchors

`scripts/ci/doctrine_anchors.tsv` unchanged (byte-identical check at proof time).

## Load-bearing proofs

| Proof | Command | Catches |
|---|---|---|
| DOC-BUDGET | `bash scripts/ci/doc_budget_check.sh --check` | prose growth above baseline |
| Rule expiry | `bash scripts/ci/rule_expiry_check.sh --check` | retired prose orphans / expiry candidates |
| AGENTS stub | `bash scripts/ci/agents_stub_check.sh --check` | pointer-stub violations |
| Orientation | `bash scripts/ci/gen_orientation.sh --check` | cold-start entrypoint present |
| Stock gates | `bash scripts/ci/doctrine_scan.sh` | DOC-BUDGET/RULE-EXPIRY/AGENTS-STUB wired |

### Owner-local proof output

```
doc_budget_check.sh --check: PASS
rule_expiry_check.sh --check: PASS or INSPECT(expiry-candidates=N)
agents_stub_check.sh --check: PASS
gen_orientation.sh --check: PASS
gen_digest.sh --check: PASS
orient.sh --selftest: PASS
relay_lint.sh --selftest: PASS
anchor_check.sh --check: PASS
clearance_check.sh --selftest: PASS
doctrine_exec_triage.sh --selftest: PASS
triage_log_check.sh --check: PASS
doctrine_selftest.sh: PASS
doctrine_scan.sh: failures=0
```

## Falsification checks

| Mutation | Expected |
|---|---|
| Protected prose growth above baseline | `DOC-BUDGET-VERDICT: FAIL(prose-growth)` |
| AGENTS.md >5 lines | `AGENTS-STUB-VERDICT: FAIL(pointer-stub)` |
| §7 missing a step | design doc audit FAIL |
| Cold-start missing from orientation | `gen_orientation.sh --check` FAIL |

## Scope Ledger

| Path | Classification |
|---|---|
| `doc_budget_check.sh`, `rule_expiry_check.sh`, `agents_stub_check.sh` | gate-wiring harness |
| `docs/ci_screening_surface.md`, design doc §3–§7 | prose compression + generative §7 |
| `AGENTS.md` | pointer stub |
| `doctrine_anchors.tsv` | untouched |
| Engine crates | untouched |

## Known gaps / next

- Merge-hold: DA/Owner clearance required (gate-wiring).
- After DA clearance: **0.0.8.4.7 DA-CLOSED**.

## Graduation routing

| Field | Value |
|---|---|
| CI verdict | PASS-RELIABLE when GHA green |
| Triage entries | none unless new INSPECT residue |
| Risk class | gate-wiring |
| Falsification check | DOC-BUDGET fails growth; AGENTS stub fails guidance paragraphs |
| Recommended posture | deep — closing rung edits doctrine text and closes 0.0.8.4.7 |