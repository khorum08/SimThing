---
rung: HD-OWNER-AUTHORING-GUIDE-0
kind: rung
track: 0.0.8.4.8.4
base_sha: a0240715a6286a10dad8966ef6f953272d56e7b5
audience: coding
model_tier: std
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "Owner-directed 2026-07-14, manual-progression. Docs-only. Write for a NEW owner who has never seen this repo: plain language first, machinery names second."
surfaces: ["docs/agent_onboarding.md", "docs/agents.md", "docs/handoff_template.md", "scripts/ci/doc_budget_baseline.tsv"]
forbidden: ["crates/**", "Studio/UI", "scripts/ci/*.sh logic changes", ".github/workflows changes", "new tables", "restating doctrine prose"]
required_checks: ["agent-scan", "orientation-check", "doc-budget", "relay-lint-selftest"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "net-prose-increase-unjustified"]
---
## BUILD
- Owner authoring guide, one section in `docs/agent_onboarding.md` (or compact companion doc with
  its own cap row if the 150 cap breaks): AUTHOR a workplan (status header; ladder with
  lead-position stamps; never escaped pipes in cells; bindings recorded at open). REVISE one
  (amendments stamp the exit-proof cell — ruling 6; prose verbs and /handoff comments; nobody
  hand-edits .hd files). LIFECYCLE (`gen_orientation.sh --open` + HD-6 gate + `--force-owner`).
  REGENERATE (orientation after ladder edits; `anchor_check.sh --resync`; librarian verbs).
  The two progression modes (manual = Owner prompts agents against the board; automated = DA-driven,
  dearer, for unattended runs).
- Role-slot sweep (ruling 7): role-first tier headings, vendors as one-line examples;
  `handoff_template.md` routing capability-tiered; zero vendor-conditioned steps anywhere.
- One line in `docs/agents.md` cloud section: VM needs `python-is-python3` (agent_scan calls `python`).
## FENCES
- Docs + `doc_budget_baseline.tsv` ONLY; cap raises paired with named compression; per-file
  adds/deletes tabled in the results doc (justify this rung's net).
- Generated docs via generator data only; pointers and anchor IDs, never restated doctrine.
## EXIT-PROOF
- Cold reader can author/revise/park a workplan from the guide alone (orchestrator walks the
  guide against live HD-6/HD-7 machinery).
- Role-first headings + tiered routing verified; prose-delta table; battery green.
- PROBATION LEADS the HD-8 cell in-diff; orientation regenerated; DA stamps graduation (ruling 6).
