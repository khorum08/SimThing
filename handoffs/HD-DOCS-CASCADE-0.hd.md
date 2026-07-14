---
rung: HD-DOCS-CASCADE-0
kind: rung
track: 0.0.8.4.8.4
base_sha: c38e1516942db2c5a63d409b2440c558df502769
audience: coding
model_tier: std
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "Owner-directed dispatch 2026-07-13. Coding agent is the Cursor cloud agent: Linux VM, cloned repo — git pull origin master FIRST so this object exists locally; studio GUI is Windows-only, do not attempt it. FOLD the pre-landed agent_onboarding operator section; never duplicate it."
surfaces: ["docs/agent_onboarding.md", "docs/agents.md", "docs/ci_screening_surface.md", "docs/handoff_template.md", "scripts/ci/doc_budget_baseline.tsv"]
forbidden: ["crates/**", "Studio/UI", "scripts/ci/*.sh logic changes", "new tables", "restating doctrine prose"]
required_checks: ["agent-scan", "orientation-check", "doc-budget", "relay-lint-selftest"]
stop_conditions: ["stale-orient-receipt", "net-prose-increase", "template-schema-drift"]
---
## BUILD
- Make HD unmissable at onboarding: each tier section in `docs/agent_onboarding.md` LEADS with its
  HD ingress line ("handoffs arrive as HD projections — render yours; 'approved, implement'
  protocol"); `docs/agents.md` router gains the HD entry; `docs/ci_screening_surface.md` gains the
  HD rows (handoff lint, receipt drift, ingress/board sync) within its existing cap.
- FOLD the pre-landed "HD Board — dispatch prompting & handoff lifecycle" section into the
  restructured onboarding doc: one home for the operator protocol, zero duplication.
- Compress `docs/handoff_template.md` to SCHEMA + AUTHORING RULES only: frontmatter field
  reference, body caps and delta-only discipline, projection/receipt semantics, response-format
  block. DELETE the required-reading blocks and restated doctrine — anchors carry doctrine now.
- Fence the compressed template against re-fattening: (a) record its hard per-file cap in
  `scripts/ci/doc_budget_baseline.tsv` at the compressed size; (b) prepend the self-describing
  header: "schema only; new doctrine goes to anchors; growth here is the regression this track
  closed."
- Relay short-form: the template's response-format section keys relays to HD-RECEIPT + the
  Graduation-routing block; delete superseded long-form relay prose.
## FENCES
- Net corpus prose DECREASES in this PR: total lines deleted across the touched docs must exceed
  lines added (HD-CLOSEOUT-0 binding is measured at close; this rung must land negative on its own).
- Generated docs (orchestrator_orientation.md) change only via generator data — no hand edits.
- No doctrine text is restated anywhere; pointers and anchor IDs only.
- No script-logic changes; `doc_budget_baseline.tsv` rows may be edited only for the template cap
  and any cap REDUCTIONS earned by deletion.
## EXIT-PROOF
- Prose-delta table in the results doc: per-file added/deleted, net NEGATIVE total.
- Template cap row live at the compressed size; anti-reaccretion header present; DOC-BUDGET PASS.
- Each tier section's first line is its HD ingress; screening surface lists the HD rows.
- Battery green: agent_scan, gen_orientation --check, doc_budget_check --check, relay_lint --selftest.
- PROBATION stamp in-diff (implementer); DA authors the graduation stamp at merge (ruling 6).
