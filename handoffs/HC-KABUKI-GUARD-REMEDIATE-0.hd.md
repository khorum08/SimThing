---
rung: HC-KABUKI-GUARD-REMEDIATE-0
kind: rung
track: 0.0.8.4.8.4.1
base_sha: 3853dbdd3d434f3c94cc85073c0b73ccf0572c95
audience: coding
model_tier: std
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "Fully-automated: coder = Grok CLI (grok-4.5), DA-driven. crates/** IS in scope here — this is the remediation rung. Both targets are DA-assessed kabuki (unmarked, no HORIZON-ENTRY). Ignore benign tzutil errors. Owner carries progression."
surfaces: ["crates/simthing-gpu/src/structural_validation.rs", "crates/simthing-mapeditor/src/studio_live_observe.rs", "scripts/ci/triage_log.tsv", "scripts/ci/inspect_justifications.tsv"]
forbidden: ["Studio/UI runtime behavior", "gen_orientation logic", "scans.tsv scan defs", "clearance router changes", "new tables", "adding a HORIZON-ENTRY marker to dodge deletion"]
required_checks: ["doctrine-selftest", "agent-scan", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "build-red-after-delete"]
---
## BUILD
- Delete the two kabuki guards HC-2 flagged (both DA-assessed kabuki, neither carries a HORIZON-ENTRY marker):
  1. `simthing-gpu::scan_for_forbidden_validation_tokens` (DEAD — zero callers): delete the fn +
     its `lib.rs` re-export.
  2. `simthing-mapeditor::observe_module_source_forbids_workshop_residue` (the #1355 self-scan
     shape — its test feeds it `include_str!` of its own source): delete the fn + its `lib.rs`
     re-export + the test's self-scan assertion (KEEP the rest of that test file).
- RETIRE the two GUARD-KABUKI-TRIPWIRE rows in `triage_log.tsv` + `inspect_justifications.tsv`
  (they described these now-deleted guards) — net ledger −4 (retirement).
## FENCES
- crates edits limited to deleting these two guards + exports + the one test assertion. Do NOT add
  a HORIZON-ENTRY marker to keep them (they are kabuki, not future API). If either invariant is
  truly wanted it returns later as a real admission-type/test, not a self-scan — out of scope here.
  No new tables, no router, no gen_orientation/scans-def changes.
## EXIT-PROOF
- Falsifier / proof: after deletion the GUARD-KABUKI-TRIPWIRE live-hit count drops 2 → 0
  (doctrine-selftest / agent-scan reflect it); `cargo check -p simthing-gpu` and
  `cargo check -p simthing-mapeditor` GREEN (nothing depended on the deleted fns). Zero raw-token
  residue. Build-green-after-delete is the bite (a naive delete that broke the build would fail).
- doctrine-selftest + agent-scan + orientation-check + doc-budget green; live 0.0.8.6 --unpark
  re-proved (19e0e85c8d3f); triage/justification rows gone (count −2 each).
- PROBATION LEADS the HC-7 cell in-diff; orientation regenerated; DA stamps graduation at merge.
