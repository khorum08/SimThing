---
rung: HC-GUARD-KABUKI-TRIPWIRE-0
kind: rung
track: 0.0.8.4.8.4.1
base_sha: 0c3135baada423f4c7c70886f086564939849efe
audience: coding
model_tier: std
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "Mechanizes anti-kabuki rule 2, prose-only today. INSPECT+triage, never FAIL — legitimate guards must be justifiable, not silenced. Owner carries progression."
surfaces: ["scripts/ci/scans.tsv", "scripts/ci/doctrine_selftest.sh", "docs/ci_screening_surface.md", "docs/handoff_template.md"]
forbidden: ["crates/**", "Studio/UI", "new tables", "clearance router changes", "gen_orientation changes", "FAIL-class verdict for this scan"]
required_checks: ["doctrine-selftest", "agent-scan", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "durable-ledger-growth"]
---
## BUILD
- New HEURISTIC scan in `scans.tsv` catching bespoke source-scanning guards — the shape #1355
  shipped green: a production `pub fn` taking `source: &str`/`&Path` that string-scans, and
  `include_str!("../src/` in tests. HEURISTIC ⇒ INSPECT + triage.
- Selftest control(s) in `doctrine_selftest.sh`: scan detects the shape, does not fire on ordinary
  code.
- Demote the prose: `handoff_template.md` §H rule 2 becomes a pointer to this scan rather than the
  enforcement (net-ledger carve-out, §0). `ci_screening_surface.md` row stays in cap.
## FENCES
- **INSPECT, never FAIL** — a legitimate guard is justifiable via `inspect_justifications.tsv`, not
  blocked. FAIL-class for this scan is forbidden.
- **Zero durable ledger growth — imitate HC-1 (#1363):** generate control samples inside the
  selftest sandbox at runtime; do NOT check fixture files in. Checked-in fixtures need
  `test_inventory.tsv` rows or the drift gate crashes `doctrine_scan` (§3a, observed live).
- Any hit on the existing tree is accounted in this PR (`inspect_justifications.tsv` +
  `triage_log.tsv`) — no suppression; HC-1 deleted the exclusion door.
- No crates, no `gen_orientation` (HC-4/HC-5), no router, no new tables.
## EXIT-PROOF
- Falsifier that BITES (ruling 3): removing the new scan row makes the control FAIL; it passes with
  the row. Green-both-ways proves nothing.
- Census of existing-tree hits + accounting rows; `doctrine_selftest.sh` green (old controls still
  bite); agent-scan, orientation-check, doc-budget green.
- 0.0.8.6 `--unpark` re-proved in a sandbox (§3a, receipt `19e0e85c8d3f`).
- PROBATION LEADS the HC-2 cell in-diff; orientation regenerated; DA stamps graduation at merge.
