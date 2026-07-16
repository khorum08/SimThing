---
rung: HC-EXCLUSION-REVIEW-GATE-0
kind: rung
track: 0.0.8.4.8.4.1
base_sha: d7284cb906489d8d2f38516e63e3bb472bfc952b
audience: coding
model_tier: std
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "Owner ruling 2026-07-16: DELETE the token outright — pairing is off the table. Exclusions become DA-authored named symbols only: one gate-wired path, no self-service door. Owner carries progression."
surfaces: ["scripts/ci/scans.tsv", "scripts/ci/doctrine_scan.sh", "docs/ci_screening_surface.md"]
forbidden: ["crates/**", "Studio/UI", "new tables", "clearance router changes", "gen_orientation gate changes", "re-suppressing accounted INSPECTs"]
required_checks: ["doctrine-selftest", "agent-scan", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "census-finds-live-users"]
---
## BUILD
- Delete `role-resolution-exclude-site` from the `SPEC-LOWERER-KIND-READ` exclusion column in
  `scripts/ci/scans.tsv` — a GENERIC token any implementer can drop in a comment to void a
  HEURISTIC finding with no review and no trace. That hole is what this rung closes.
- Census every exclusion column in `scans.tsv`: classify each token as DA-authored named symbol
  (keep — it cost a reviewed gate-wiring edit, e.g. `planet_non_grid_child_kind_label`) or generic
  self-service token (delete). Report the census table in the results doc.
- Any finding newly surfaced by a deletion is accounted as an INSPECT with
  `inspect_justifications.tsv` + `triage_log.tsv` rows — never re-suppressed by another token.
## FENCES
- The two `fleet_presence.rs` kind reads stay accounted INSPECTs (justified #1355): do not
  re-suppress, do not re-litigate.
- Zero live users of the target token exist (verified 2026-07-16); deletion should change no
  verdict. If the census finds live users, STOP and report — that is a DA call, not a fix.
- `ci_screening_surface.md` stays within its DOC-BUDGET cap. No new tables, no router/lexicon
  change, no `gen_orientation` edits (HC-4/HC-5 own that surface).
## EXIT-PROOF
- Falsifier that BITES (ruling 3): a fixture site bearing the deleted token is SCANNED, not
  excluded — prove it FAILS on the pre-fix tree and passes after. Green-both-ways proves nothing.
- Census table in results; `doctrine_selftest.sh` green (controls still bite); agent-scan,
  orientation-check, doc-budget green; 0.0.8.6 park round-trip re-proved (§3a).
- PROBATION LEADS the HC-1 cell in-diff; orientation regenerated; DA stamps graduation at merge.
