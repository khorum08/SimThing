# OVERLAY-GERM-ARCHAEOLOGY-0 results

- Track: 0.0.8.7 RF arena modernization (rung 7.6)
- Status: **PROBATION / proof-present / DA-review-pending**
- HD-RECEIPT: `e16ae62d9ce6`
- ORIENT-RECEIPT: `e0ac65d38d15`
- orientation_rule_stamp: `497b2a43330e6f9d`
- ANCHOR-ACK: `orientation-harness-core@8a365d1c0864`
- ANCHOR-ACK: `scanner-selftest-delta-gate@34fb2662baae`
- Board dispatch: comment `5289606240`
- Remand: `5289859340` (DA `5289831443`; parked mechanical `5289705592`)
- expected_route: `DA-RESERVE(gate-wiring)`
- Coverage: docs/TSV/TXT/script only; zero `crates/**`; zero `.github/workflows/**`; checker not CI-wired (DA graduation PR)

## What landed

Tree-derived census of every overlay attach/activate/suspend/dissolve/apply/override/expire route plus the three v4 unification surfaces. Known starting files were seeds, not the universe.

Declared surfaces (handoff `surfaces:` exactly):

| Artifact | Role |
|---|---|
| `scripts/ci/overlay_germ_archaeology_census.tsv` | classified routes + folded `# RESIDUE` justifications |
| `scripts/ci/overlay_germ_archaeology_census_universe.txt` | pinned harvest (71 tokens) |
| `scripts/ci/overlay_germ_archaeology_census_check.sh` | `--check` / `--harvest` / `--selftest` |
| `docs/tests/overlay_germ_archaeology_0_results.md` | this signal-only record |
| `docs/tests/current_evidence_index.md` | one evidence line |

## Family counts (classified rows)

| Family | Rows | What it enumerates |
|---|---|---|
| OVERLAY | 52 | attach/activate/suspend/dissolve/apply/expire/override incl. 3 analysis |
| INHERIT | 8 | `resolve_owner`-class walks, `inherit_active_overlays`, grant/prereq |
| CROSSING-WRITE | 8 | sealed crossings, 7.3 subordinate activation, ActionBand StateNext / OverlayEvent |
| EML-REGISTRY | 6 | `EmlExpressionRegistry`, session cache, JIT `pipeline_cache_digest` |
| **total** | **74** | 71 harvested + 3 analysis |

Class: 56 GENUINELY-STRUCTURAL / 16 SEMANTIC-DUPLICATE / 2 DEAD. Zero unclassified. Zero fourth class.

`OverrideReceived` is **SEMANTIC-DUPLICATE / migrate**: reachable-but-unimplemented. Authored input can name the serde-visible variant, admission accepts a non-empty dissolution-condition list, lifecycle evaluation returns false, and the documented attach-side replacement handler does not exist. 7.6 does not implement replacement.

## Reconciliation

```text
RECONCILIATION: routes=74 discovery=71 residue=49 unclassified=0 open=0
CENSUS-CHECK-VERDICT: PASS
```

`--selftest` plants `fn dissolve_overlay` outside the pin:

```text
CENSUS-CHECK-VERDICT: FAIL(unlisted-route:core:planted_unlisted_route.rs:dissolve_overlay)
CENSUS-SELFTEST-VERDICT: PASS
```

`--harvest` is local-only: dirty tree → `CENSUS-HARVEST-VERDICT: FAIL(dirty-tree)`; clean-tree drift → `CENSUS-HARVEST-VERDICT: STALE (universe drifted; re-reconcile the TSV, do not hand-edit)`.

## Fences held

No migrate/delete/refactor of engine routes. No 7.7/7.8/7.8a/7.9 start. No workflow wiring. No `binding_conditions.tsv` edit. Pointer unchanged.
