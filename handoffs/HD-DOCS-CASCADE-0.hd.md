---
rung: HD-DOCS-CASCADE-0
kind: rung
track: 0.0.8.4.8.4
base_sha: c38e1516942db2c5a63d409b2440c558df502769
audience: coding
model_tier: std
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "Owner-directed 2026-07-13. Cursor cloud agent (Linux VM): git pull master first; studio GUI is Windows-only. FOLD the pre-landed agent_onboarding operator section; never duplicate."
surfaces: ["docs/agent_onboarding.md", "docs/agents.md", "docs/ci_screening_surface.md", "docs/handoff_template.md", "scripts/ci/doc_budget_baseline.tsv"]
forbidden: ["crates/**", "Studio/UI", "scripts/ci/*.sh logic changes", "new tables", "restating doctrine prose"]
required_checks: ["agent-scan", "orientation-check", "doc-budget", "relay-lint-selftest"]
stop_conditions: ["stale-orient-receipt", "net-prose-increase", "template-schema-drift"]
---
## BUILD
- Each onboarding tier section LEADS with its HD ingress line; `docs/agents.md` gains the HD
  entry; `docs/ci_screening_surface.md` gains the HD rows within its cap.
- FOLD the pre-landed operator-protocol section into the restructure: one home, zero duplication.
- Compress `docs/handoff_template.md` to schema + authoring rules; DELETE required-reading and
  restated doctrine (anchors carry doctrine); relay short-form keys to HD-RECEIPT.
- Fence the template: cap row in `doc_budget_baseline.tsv` at compressed size + anti-reaccretion
  header ("schema only; new doctrine goes to anchors; growth here is the regression this closed").
## FENCES
- Net corpus prose DECREASES in this PR (per-file delta table, net negative total).
- Generated docs change only via generator; no doctrine restated; no script-logic changes;
  budget rows edited only for the template cap or deletion-earned reductions.
## EXIT-PROOF
- Prose-delta table net NEGATIVE; template cap live + header present; tier sections lead with HD
  ingress; battery green (agent_scan, orientation --check, doc_budget --check, relay_lint
  --selftest); PROBATION stamp in-diff; DA stamps graduation at merge (ruling 6).
