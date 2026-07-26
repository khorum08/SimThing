---
rung: CONSTITUTION-TRIPWIRES-0
kind: rung
track: 0.0.8.7
base_sha: 407850e21d8294b0a18866f8b1ca314173b08404
audience: coding
model_tier: std
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "Rung 0.3, Std lane (Grok, pin -m grok-4.5). Last Phase 0 rung. Pattern-follow 0.1/0.2: scans.tsv rows + known-bad fixtures + selftest wiring. All three tripwires are HEURISTIC/INSPECT-only reach detectors feeding the reach log — evidence collectors, never hard gates."
surfaces: ["scripts/ci/scans.tsv", "scripts/ci/doctrine_scan.sh", "scripts/ci/doctrine_selftest.sh", "scripts/ci/fixtures/known_bad", "scripts/ci/anchor_reach_log.tsv", "scripts/ci", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/ci_screening_surface.md"]
forbidden: ["RELIABLE severity (all three are HEURISTIC — delta-scoped, net-new only)", "new exclusion rows on any scan (frozen; DA sign-off)", "engine crate edits (crates/** untouched)", "hard-failing any existing green path"]
required_checks: ["doctrine-scan", "doctrine-selftest (scans.tsv changes trigger battery)", "clearance selftest", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "a-tripwire-regex-false-fires-on-legit-engine-code-and-cannot-be-narrowed"]
---
## BUILD
- Three HEURISTIC scans.tsv tripwires (0.0.8.7 §2 P0(e)/P5 + §4 TRIAD DOORS — ANCHOR-ACK the
  core-0087 anchors), each with a known_bad fixture + selftest proof, each INSPECT-only:
  1. `CELL-STORAGE-POLYMORPHISM` — reaches for tagged-union/templated/heterogeneous matrix
     cell storage in engine crates (e.g. enum/union/generic-param cell value types near the
     matrix lanes). Fence (i) of the P0(e) domain rider.
  2. `BESPOKE-PATHFINDER` — A*/Dijkstra/priority-queue graph-search reaches in production
     crates (BinaryHeap+came_from/open_set/g_score family). The PALMA pathing door.
  3. `BORDER-SERVICE` — border/contour/frontline tracer-service reaches (marching squares,
     contour extraction, border objects) in production crates. The Gu-Yang border door.
     Presentation-side polyline extraction in mapeditor render paths is the known-legit
     family — narrow the pattern or route those hits to INSPECT triage, never widen to FAIL.
- Reach-log wiring: fired tripwires append a reach-log row (date, scan id, file:line) so
  accumulated reaches become the evidence base for ruling expansions (§2 fence text).
- Regexes narrow > wide: false-fire on legit engine idioms is the failure mode that killed
  vocabulary detectors (12.7 lesson) — prefer multi-token PCRE2 conjunctions; net-new only.
## FENCES
- HEURISTIC severity only; no exclusion-row additions; zero crates/** edits; the three scans
  must not change any existing scan's verdict (doctrine scan stays 0 hard failures).
## EXIT-PROOF
- Doctrine selftest proves each fixture fires (three known-bad fixtures → INSPECT rows) and
  clean tree stays clean (0 hard failures; delta-scan INSPECT-only for net-new). Reach-log
  append demonstrated in fixture run. Stamp 0.3 PROBATION in-diff + regen orientation.
  Clearance route DA-RESERVE(gate-wiring) expected (scans.tsv is gate-wired since 0.1).
