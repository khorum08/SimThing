---
rung: RF-COLUMN-ADMISSION-GATE-0
kind: rung
track: 0.0.8.7
base_sha: cebead45d05d4af7e466828a95492944434427b8
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "Rung 0.1, first rung of 0.0.8.7. Frontier lane (Codex 5.6). Greenfield discretion charter applies (§3b): optimize the taxonomy for elegance/performance; escalate only if a §2/§4 law would change. NON-BREAKING is the load-bearing constraint."
surfaces: ["crates/simthing-core/src", "scripts/ci/scans.tsv", "scripts/ci/doctrine_scan.sh", "scripts/ci", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/ci_screening_surface.md"]
forbidden: ["editing ColumnIndex call sites outside simthing-core (migration is rung 9.2 — NOT now)", "breaking any existing ::new call site (84 remain; corpus must stay green untouched)", "new exclusion rows on COLUMN-INDEX-MINT (frozen; DA sign-off only)", "kernel/driver/sim behavior changes"]
required_checks: ["cargo build --workspace", "cargo test -p simthing-core", "doctrine-scan", "doctrine-selftest (scans.tsv changes)", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "taxonomy-would-break-existing-call-sites", "a-door-needs-kernel-semantics-changes"]
---
## BUILD
- `ColumnIndex` constructor taxonomy in `simthing-core` (design §5 / OC-K2.1a; 0.0.8.7 §3
  Phase 0 row + P0/P4 pillars — ANCHOR-ACK core-0087 anchors). Legal doors: (a) layout-derived
  paths — `PropertyLayout::offset_of` / `col_for_role` / arena-layout ranges become the sole
  blessed derivation surface (the role pathway IS the door; core `registry.rs` hosts it);
  (b) a doc-fenced GPU ROUND-TRIP door (re-materializing `gpu.*_col` plan fields — adapter
  family); (c) a doc-fenced RAW door for oracle/rehearsal code (judging independence requires
  raw mint). Each fenced door = a distinctly-named constructor with a doctrine doc-comment
  naming its family + promotion-blocker.
- NON-BREAKING: existing `::new` call sites keep compiling — `new` remains available as (or
  delegating to) the fenced raw door with a deprecation-note steering new code to the doors.
  Zero call-site edits outside simthing-core.
- Retarget `COLUMN-INDEX-MINT` from `ColumnIndex::new` to the fenced-door TOKENS (watch the
  named doors, not 16 excused files); keep HEURISTIC/INSPECT. Mechanize the exclusion freeze:
  scan-row exclusion edits route DA-RESERVE (gate-wiring class already covers scripts/ci/**;
  add the selftest fixture proving an exclusion-row diff FAILs orchestrator-clearable).
## FENCES
- Migration is rung 9.2 — do not sweep call sites. No new exclusions. Corpus green untouched:
  every crate builds, simthing-core battery green, no kernel/driver behavior deltas.
## EXIT-PROOF
- Post-rung scan proves every remaining `::new` resolves inside the fence (doors enumerated,
  zero unexcused raw reaches); selftest fixture: exclusion-row edit → DA-RESERVE. cargo
  workspace build + core tests green; doctrine-scan 0 hard failures; stamp the 0.1 ladder cell
  PROBATION on merge; orientation regenerated.
