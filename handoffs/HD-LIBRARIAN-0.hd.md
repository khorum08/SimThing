---
rung: HD-LIBRARIAN-0
kind: rung
track: 0.0.8.4.8.4
base_sha: 2424f447d1417fdbc73ad1daf27abc527fe34aca
audience: coding
model_tier: std
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "Owner-directed dispatch 2026-07-12 ('proceed'). Owner verbs: 'check library staleness' and 'cull dead tsv rows' must map 1:1 onto these flags."
surfaces: ["scripts/ci/librarian.sh", "scripts/ci/anchor_check.sh", "scripts/ci/track_closeout.sh"]
forbidden: ["crates/**", "Studio/UI", "HD-5", "new gating", "new tables", "bespoke reap logic"]
required_checks: ["librarian-selftest", "agent-scan", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "reap-outside-safe-classes"]
---
## BUILD
- `scripts/ci/librarian.sh` with three verbs, composing EXISTING machinery only:
- `--staleness`: one report <=60 lines folding anchor resync/orphans (`anchor_check.sh --resync`
  dry view), dead-listener trigger domains no glob emits, reach-log prune candidates, lease /
  closeout-artifact aging, and doc-budget headroom.
- `--cull`: dry-run by default, `--confirm` to act; composes `track_closeout.sh --discover` /
  `--decommission` + reach-log prune + orphan-anchor retirement. src/ and authority paths always
  route to DA (inherit reaper safe-class rules verbatim); emits LIBRARIAN-CULL-VERDICT with
  per-item disposition.
- `--catalog [--role coding|orchestrator|da]`: which anchors, payload sections, and always-on
  spine each role can reach — the agent-confusion antidote; <=60 lines per role.
- `/librarian` comment command (doctrine-exec plumbing) + one doc line mapping Owner prose verbs
  ("check library staleness", "cull dead tsv rows") to the flags.
## FENCES
- Observability + reaping only: no new gates, no new tables, no second reap implementation —
  decommission logic stays in `track_closeout.sh` and is invoked, not copied.
- Never delete outside the reaper's safe classes; `--confirm` without dry-run review output is
  still bounded by safe classes + DA routing.
- Reports are complete-or-fail at the 60-line cap (HD-2 precedent), never truncated.
## EXIT-PROOF
- Fixtures bite: cull-dry-run-default (no writes without `--confirm`); src-path-routes-to-DA;
  staleness-report-cap; catalog-per-role.
- Selftests + agent-scan + orientation + doc-budget green.
- PROBATION stamp in-diff (implementer); DA authors graduation stamp at merge (standing ruling 6).
