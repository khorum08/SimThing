---
rung: HC-HORIZON-ENTRY-CONVENTION-0
kind: rung
track: 0.0.8.4.8.4.1
base_sha: 88332c5bf6e6911ffd0792ed0e4c3018ecbb1593
audience: coding
model_tier: std
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "Fully-automated: coder = Grok CLI (grok-4.5), DA-driven. Owner distinction (ruling 6): kabuki is unmarked self-referential scaffolding; a HORIZON-ENTRY is future API laid down ahead of a consumer, marked+dated, exempt from the tripwire but lifecycle-assessed. Ignore benign tzutil errors. Owner carries progression."
surfaces: ["scripts/ci/scans.tsv", "scripts/ci/doctrine_scan.sh", "scripts/ci/gen_orientation.sh", "docs/handoff_template.md", "docs/owner_authoring_guide.md"]
forbidden: ["crates/**", "Studio/UI", "clearance router changes", "new tables", "FAIL-class exemption (must stay assessable)"]
required_checks: ["doctrine-selftest", "gen-orientation-selftest", "agent-scan", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "silent-forever-pass"]
---
## BUILD
- Define a greppable, DATED HORIZON-ENTRY marker: `HORIZON-ENTRY(<iso-date>): <intended consumer / design ref>`
  that a symbol carries to affirm future intent. Document it in `handoff_template.md` and `owner_authoring_guide.md`.
- The GUARD-KABUKI-TRIPWIRE scan (HC-2) EXEMPTS a symbol bearing a well-formed FRESH marker, so
  future consumerless API no longer trips it. The exemption is greppable+dated+assessed — NOT the
  silent self-service door HC-1 deleted (unmarked stays flagged; no bare token voids a finding).
- Lifecycle: --park/--unpark/track_closeout (and/or a staleness pass) ASSESS markers — a marker
  older than a staleness window with still no consumer FLAGS to INSPECT (stale/superseded =
  deletion candidate); NEVER auto-delete, NEVER a forever-pass.
## FENCES
- No crates, no router, no new tables. Exemption is INSPECT-assessable, never a FAIL-suppression.
  0.0.8.6 --park/--unpark byte-exact (§3a).
## EXIT-PROOF
- Falsifiers BITE (ruling 3): a fresh-marked consumerless fn is EXEMPT from the tripwire; the SAME
  fn unmarked, or marked with a STALE date, is FLAGGED. Prove each differs pre/post.
- doctrine-selftest + gen_orientation --selftest + agent-scan + --check + doc-budget green; live
  0.0.8.6 --unpark re-proved (19e0e85c8d3f). Net ledger not worsened.
- PROBATION LEADS the HC-6 cell in-diff; orientation regenerated; DA stamps graduation at merge.
