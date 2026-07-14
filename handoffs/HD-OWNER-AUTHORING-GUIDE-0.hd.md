---
rung: HD-OWNER-AUTHORING-GUIDE-0
kind: rung
track: 0.0.8.4.8.4
base_sha: a0240715a6286a10dad8966ef6f953272d56e7b5
audience: coding
model_tier: std
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "Owner-directed 2026-07-14, manual-progression mode (Owner prompts each agent against the board). Docs-only rung: write for a NEW owner/collaborator who has never seen this repo — plain language first, machinery names second."
surfaces: ["docs/agent_onboarding.md", "docs/agents.md", "docs/handoff_template.md", "scripts/ci/doc_budget_baseline.tsv"]
forbidden: ["crates/**", "Studio/UI", "scripts/ci/*.sh logic changes", ".github/workflows changes", "new tables", "restating doctrine prose"]
required_checks: ["agent-scan", "orientation-check", "doc-budget", "relay-lint-selftest"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "net-prose-increase-unjustified"]
---
## BUILD
- Owner/collaborator authoring guide, folded into `docs/agent_onboarding.md` as one section (or a
  compact companion doc with its own cap row if folding breaks the 150 cap): how to AUTHOR a new
  workplan (status header, ladder table with lead-position status stamps, never `\|` in cells,
  binding conditions recorded at open); how to REVISE one (amendments stamp the exit-proof cell,
  ruling 6; prose verbs / /handoff comments — nobody hand-edits .hd files); the track LIFECYCLE
  (open via `gen_orientation.sh --open` — the HD-6 gate refuses unless the outgoing track is
  CLOSED/PARKED; `--force-owner "<why>"` records an owner directive row); REGENERATING the library
  and TSVs (orientation regen after ladder edits; `anchor_check.sh --resync` after anchored-doc
  edits; `librarian.sh --staleness / --cull / --catalog --role <r>` as the stewardship verbs);
  and the two progression modes (manual: Owner prompts each agent to check the board; automated:
  DA drives — dearer, for unattended runs).
- Role-slot sweep (ruling 7): tier headings become role-first with current fills as one-line
  examples; `docs/handoff_template.md` routing table becomes capability-tiered (std coder /
  frontier DA), vendors as examples only; no doc may condition a step on a vendor name.
- Record the queued cloud caveat in `docs/agents.md` Cursor-cloud section: the VM needs
  `python-is-python3` (agent_scan invokes `python`); one line.
## FENCES
- Docs + doc_budget_baseline.tsv ONLY. Cap raises paired with named compression elsewhere;
  per-file adds/deletes tabled in the results doc (HD-CLOSEOUT-0 net-decrease is measured at
  track close — justify this rung's net in the table).
- Generated docs change via generator data only; no doctrine restated — pointers and anchor IDs.
## EXIT-PROOF
- A cold reader can author/revise/park a workplan from the guide alone (orchestrator verifies by
  walking the guide against the live HD-6/HD-7 machinery names).
- Role-first headings + capability-tiered routing verified; zero vendor-conditioned steps remain.
- Prose-delta table per file; battery green; PROBATION LEADS the HD-8 cell in-diff; orientation
  regenerated. DA authors the graduation stamp at merge (ruling 6); relay carries this receipt.
