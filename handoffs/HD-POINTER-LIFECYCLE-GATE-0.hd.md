---
rung: HD-POINTER-LIFECYCLE-GATE-0
kind: rung
track: 0.0.8.4.8.4
base_sha: 57914caa225a19125471fe909bf21fd96c7033ef
audience: coding
model_tier: std
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "Owner-approved 2026-07-14. Coder this rung: Cursor cloud (Linux VM; git pull master FIRST). One automated rung at a time; Owner reviews after graduation."
surfaces: ["scripts/ci/gen_orientation.sh", ".github/workflows/clearance.yml", "scripts/ci/owner_directives.tsv"]
forbidden: ["crates/**", "Studio/UI", "handoff_dispatch.sh logic beyond board-sync reuse", "new tables", "new verdict lexicon"]
required_checks: ["gen-orientation-selftest", "agent-scan", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "gate-weakening"]
---
## BUILD
- `gen_orientation.sh --open <track.md>`: REFUSE the pointer flip (FAIL verdict line, no writes)
  while the OUTGOING track doc's status header does not contain CLOSED or PARKED.
- `--force-owner "<directive text>"` escape: proceeds AND appends an `owner_directives.tsv` row
  (directive text, scope=outgoing track id, status=active, set_by=Owner-<date>). No silent force.
- Board freshness: the clearance workflow's board sync also fires on push to master (render
  board-json + update the SimThing Board issue; skip PR-ingress steps on push events).
## FENCES
- Reuse the existing board-sync steps and `owner_directives.tsv`; no new tables, no new verdict
  vocabulary beyond the --open gate detail; no changes to dispatch/lint logic.
- The gate must not weaken: no env bypass other than `--force-owner`; fixtures prove refusal.
## EXIT-PROOF
- Fixtures bite: open-from-OPEN FAILs (no pointer write); open-from-PARKED and open-from-CLOSED
  pass; force-owner records the directive row; workflow-level board render with
  `current_handoff: none` passes (the #1342 class, proven at the workflow layer).
- Battery green (gen_orientation --selftest + --check, agent_scan, doc_budget, relay_lint).
- Live proof: post-merge push event re-renders the board with the fresh master_head.
- PROBATION stamp LEADS the HD-6 exit-proof cell in-diff; DA authors graduation at merge (ruling 6).
