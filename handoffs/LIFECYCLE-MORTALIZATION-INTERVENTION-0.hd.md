---
rung: LIFECYCLE-MORTALIZATION-INTERVENTION-0
kind: transport
track: 0.0.8.7
base_sha: 32ed2b80
audience: orchestrator
model_tier: std
owner_approved: true
expected_route: ORCHESTRATOR-CLEARABLE
owner_notes: "Owner mandate 2026-07-30: NO PROOF IS EVER PERMANENT. Governance regime changed under the frozen 5.3c work. Orchestrator HOLD 5132075417 was correct; this is the awaited DA guidance. Re-orient BOTH tiers before resuming."
surfaces: ["scripts/ci", "handoffs", "docs/tests"]
forbidden: ["rebasing #1507 mechanically before reconciling its governance delta", "restoring any durability exemption", "renewing a proof without a named consumer", "5.4-5.8 dispatch"]
required_checks: ["orient.sh --role=orchestrator (fresh)", "track_closeout.sh --rungclose", "test_lifecycle_expiry_check.sh --scheduled", "doctrine-scan", "clearance"]
stop_conditions: ["stale-orient-receipt", "a proof needs renewal but no live consumer can be named", "reap backlog cannot clear before 2026-08-11"]
---
## BUILD
- RE-ORIENT BOTH TIERS FIRST. Rule-source files changed; carried receipts
  are stale. You re-read `docs/orchestrator_orientation.md` at head; Grok
  runs `orient.sh --role=coding` fresh. A relay quoting an old receipt
  FAILs relay-lint mechanically.
- REGIME (merged, master `32ed2b80`): #1513 mortalized the proof lifecycle
  — durability allowlists are EMPTY, so the 1317 rows (78%) formerly immune
  by class now expire; a row is reapable 5 wall-clock days after its birth
  track closes; renewal = a `downstream-utility: <consumer>` note plus a
  `dsu_survivals` bump; a 4th renewal is a hard FAIL demanding promotion
  evaluation. #1514 added `track_closeout.sh --rungclose <RUNG-ID>`: the
  DA-facing graduation gate. It blocks graduation while ANY proof is
  expired. 495 rows come due 2026-08-11.
- WHAT THIS INVALIDATES IN #1507 — governance delta only; the PRODUCTION
  repair stands. The `prepare_threshold_scan`/`finish_threshold_scan`
  restoration in `world_state.rs`, the untouched golden, and the green
  `s6_threshold_events_match_cpu_golden` all survive. Stale: the orient
  receipt, the exact-base clearance, the lifecycle `--prove` evidence, and
  the closeout evidence. #1507 also edits `scripts/ci/track_closeout.sh`,
  which #1514/#1515 changed underneath it — reconcile, never mechanically
  rebase, or the rungclose gate will be silently reverted.
- REVISE THE NEXT GROK HANDOFF to carry: (a) re-orient fresh; (b) any test
  it adds or touches needs a lifecycle-legal inventory row — no
  permanent-residue as a shield, a named consumer if renewed; (c) its
  authorized-renames ledger work must merge WITH the new gate, not over it;
  (d) proof-package evidence must be regenerated at the new head.
- REAP PLANNING (do not execute here): the 495 due 2026-08-11 must be
  deleted as PAIRS — test fn AND inventory row — because the drift gate is
  bidirectional and only pairs stay green. Anything load-bearing (RF-1
  conservation, CPU/GPU parity, replay determinism) is RENEWED with a named
  consumer, never exempted. Report the proposed reap/renew split to the DA
  before executing.
## FENCES
- The production repair is not reopened by this intervention.
- No durability exemption may return; the `durable-immune` prove case is
  inverted precisely so restoring it breaks a test.
- Renewal requires a live named consumer. A re-stamp is not a renewal.
## EXIT-PROOF
- Both tiers re-oriented with fresh receipts quoted.
- #1507's governance delta reconciled against master `32ed2b80` (gate
  intact, ledger preserved) and the revised Grok handoff issued.
- `--rungclose THRESHOLD-EVENT-REGRESSION-REPAIR-0` reports only the
  expected pre-graduation failures; the reap/renew split is reported to DA.
