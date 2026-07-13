---
rung: HD-OWNER-INTERFACE-0
kind: rung
track: 0.0.8.4.8.4
base_sha: 2424f447d1417fdbc73ad1daf27abc527fe34aca
audience: coding
model_tier: std
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "Owner-directed dispatch 2026-07-12 ('proceed'). Interface must stay simple for future collaborators: prompts and GitHub comments, never hand-edited files."
surfaces: ["scripts/ci/handoff_dispatch.sh", "scripts/ci/doctrine_exec_commands.sh", ".github/workflows/clearance.yml"]
forbidden: ["crates/**", "Studio/UI", "HD-5", "new clearance classes", "new route verdicts"]
required_checks: ["handoff-dispatch-selftest", "relay-lint-selftest", "agent-scan", "orientation-check"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "silent-mutation-without-echo"]
---
## BUILD
- `/handoff approve|amend: <text>|hold|status` comment commands on the board issue and rung PRs,
  reusing the existing doctrine-exec comment-command machinery (`doctrine_exec_commands.sh` family).
- approve: flips `owner_approved: true` (owner-authored comments only). amend: appends text to
  `owner_notes` and re-renders sticky/board. hold: flips `owner_approved: false` and freezes
  dispatch. status: posts the current board digest as a comment reply.
- Scribe protocol (short doc section, orchestrator-facing): Owner prose -> .hd mutation -> echo the
  exact diff back for confirmation before push. Never mutate silently.
- "Current handoff approved, implement" ingress protocol (doc section, all roles): resolve the
  current .hd, render your role projection, verify `owner_approved: true`, proceed; lint already
  hard-blocks unapproved dispatch.
## FENCES
- Reuse doctrine-exec command plumbing; no new workflow file; no new verdict lexicon beyond
  HD-LINT; no new TSVs.
- Non-owner `/handoff approve|amend|hold` never mutates: route to owner-review (comment reply
  naming the requester), exactly like existing owner-gated command handling.
- `owner_notes` guaranteed-render invariant must hold after every mutation path.
## EXIT-PROOF
- Fixtures bite: non-owner-amend-routes-to-owner-review; approve-flips-gate; hold-freezes-dispatch;
  owner-notes-render-after-amend.
- Selftests + agent-scan + orientation + doc-budget green; live demonstration: a `/handoff status`
  comment on the rung PR answered by the workflow.
- PROBATION stamp in-diff (implementer); DA authors graduation stamp at merge (standing ruling 6).
