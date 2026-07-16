---
rung: HC-LADDER-COLUMN-INTEGRITY-0
kind: rung
track: 0.0.8.4.8.4.1
base_sha: 01935bb6204ac4638be8c55c3b218031a7cc6300
audience: coding
model_tier: std
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "Fully-automated: coder = Grok CLI (grok-4.5), DA-driven. Assert AT the parse_rungs choke point (active/park/unpark only, §3a) — do NOT widen to a repo-wide doc scan. Owner carries progression."
surfaces: ["scripts/ci/gen_orientation.sh"]
forbidden: ["crates/**", "Studio/UI", "scans.tsv", "clearance router changes", "new tables", "repo-wide ladder scan"]
required_checks: ["gen-orientation-selftest", "agent-scan", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "closed-track-ladder-reddened"]
---
## BUILD
- `parse_rungs()` bounds too-few columns but never too-many, so an escaped or bare pipe in a cell
  shifts `parts[3]` off the Exit-proof and the stamp silently misreads. Assert each ladder data row
  matches the column count its own header declares; `gen_orientation.sh --check` FAILs on a
  mismatch, naming the row + remedy ("say it without a pipe; a bare pipe splits too").
- Scope: assert only at the existing `parse_rungs` choke point (active gen/--check, --park,
  --unpark) — §3a. NO repo-wide ladder walk; closed tracks (design_0_0_8_4_6 has a legit
  escaped-pipe ladder row) must NOT be reddened.
## FENCES
- No crates, no scans.tsv, no router, no new tables. 0.0.8.6 --park/--unpark stays byte-exact (§3a).
## EXIT-PROOF
- Falsifier BITES (ruling 3): a fixture ladder row with an escaped pipe in its Scope cell parses to
  the WRONG exit-proof and --check PASSes pre-fix, FAILs after; a clean row passes. Prove pre/post.
- gen_orientation --selftest + agent-scan + --check + doc-budget green; live 0.0.8.6 --unpark
  re-proved (19e0e85c8d3f); the active HC workplan ladder still parses clean.
- PROBATION LEADS the HC-5 cell in-diff; orientation regenerated; DA stamps graduation at merge.
