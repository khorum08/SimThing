---
rung: CONSTITUTION-TRIPWIRES-0
kind: rung
track: 0.0.8.7
base_sha: bfc561ee8fbe4988f2544164a97d7fc1b7fd34bc
audience: coding
model_tier: std
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "Rung 0.3 revised after DA graduation of 0.2. Grok CLI, pin -m grok-4.5. Deliver the three constitutional reach detectors and fold the DA-ruling documentation/count corrections plus Board/orientation staleness repair into the same harness-only change."
surfaces: ["scripts/ci/scans.tsv", "scripts/ci/doctrine_scan.sh", "scripts/ci/doctrine_selftest.sh", "scripts/ci/fixtures/known_bad", "scripts/ci/anchor_reach_log.tsv", "scripts/ci/execution_status_taxonomy.tsv", "scripts/ci/execution_status_mixed_posture.tsv", "scripts/ci/execution_status_census.py", "scripts/ci/gen_orientation.sh", "scripts/ci/handoff_dispatch.sh", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/ci_screening_surface.md", "docs/orchestrator_orientation.md"]
forbidden: ["RELIABLE severity or hard-failing the three HEURISTIC tripwires", "new exclusion rows on any scan", "engine crate edits or the deferred 4.2 structural splits", "a fifth execution-status class", "manual Board text edits that bypass the generator/source of truth"]
required_checks: ["doctrine-scan", "doctrine-selftest (including all three fixtures)", "clearance selftest", "handoff-dispatch selftest", "orientation-check and doc-budget", "board-json/render-board proof at exact head"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "a tripwire cannot avoid legitimate-engine false positives", "DA primary-class rulings cannot be represented without changing the four-class law"]
---
## BUILD
- Add three delta-scoped HEURISTIC/INSPECT-only tripwires, each with a known-bad fixture and selftest:
  1. `CELL-STORAGE-POLYMORPHISM` for tagged/templated heterogeneous matrix-cell storage reaches.
  2. `BESPOKE-PATHFINDER` for production A*/Dijkstra/priority-queue graph-search reaches outside PALMA.
  3. `BORDER-SERVICE` for contour/frontline/border-service reaches outside Gu-Yang; legitimate mapeditor rendering must remain non-failing.
- Wire fired tripwires into `anchor_reach_log.tsv`; patterns must be narrow, multi-token, and net-new only.
- Fold the DA 0.2 rulings into machine data and generated docs without changing engine code:
  - `reduction.rs` primary=`compile-plan`; `world_state.rs` primary=`executed`.
  - Preserve exactly four legal classes; retain secondary-posture evidence without a fifth class.
  - Primary-inclusive totals must render `executed=57 oracle=6 rehearsal=14 compile-plan=45 mixed_ruled=2`; remove the false `mixed_pending_da=2` state.
- Repair Board/orientation staleness at the generator/source: `active_pointer` and `current_handoff` must resolve to `CONSTITUTION-TRIPWIRES-0` and this revised HD receipt after regeneration, never by hand-editing the Board comment.
## FENCES
- Harness/data/docs only; zero `crates/**` edits and no 4.2 file splits.
- No exclusion additions, no hard-gate promotion, and no changes to existing scan verdicts.
- Board and orientation remain generated mirrors; fix their source or generation path and add regression coverage.
## EXIT-PROOF
- All three fixtures emit INSPECT, clean-tree Doctrine Scan has zero hard failures, and reach-log append is demonstrated.
- Census and generated orientation/Board show the DA-ruled primary-inclusive counts and `mixed_ruled=2`.
- `handoff_dispatch --board-json` plus rendered Board prove fresh 0.3 pointer/current-handoff state at exact head.
- Stamp rung 0.3 PROBATION in-diff, regenerate orientation, run required checks, and return exact-head proof for DA deep review.
