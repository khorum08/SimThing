---
rung: STUDIO-DISRUPTION-READOUT-0
kind: rung
track: 0.0.8.6
base_sha: 7013f68953bcacd93eb14b33bfe375c494002da0
audience: coding
model_tier: frontier
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "Fully-automated stack: coder = Codex CLI (gpt-5.5 high), DA-driven headless. Buildable PRE-12.8: the accessor contract + fail-soft 0.0 + fail-loud error paths are the deliverable now; live values arrive when 12.8 lands. Owner carries progression."
surfaces: ["crates/simthing-spec", "crates/simthing-mapeditor", "crates/simthing-clausething"]
forbidden: ["field/kernel/WGSL semantics", "writes to field state", "scheduling changes", "scripts/ci logic", "new tables", "CPU planner"]
required_checks: ["cargo-check-3-crates", "focused-tests", "agent-scan", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "field-write-surface"]
---
## BUILD
- Read-only per-system disruption snapshot over the live session: max-disruption-accreted per
  star-system gridcell, snapshot-consistent per tick, keyed by generated system id, wired through
  the existing live-session bridge to a mapeditor-consumable map (mirror the 12.4 fleet-presence
  shape: spec helper owns authority readback + id translation; mapeditor consumes typed records).
- Fail-soft to neutral: `0.0` per system when the disruption field is absent (incl. the
  structural-shell fallback session — 12.8 has not landed; that path is today's reality).
  Fail-loud (typed error) on authority readback error. No public mutation surface.
## FENCES
- **Read-only**: no writes to field state, no scheduling/tick changes, no kernel/WGSL semantics,
  no CPU planner. Snapshot type owns its data (private fields, accessor like `records()`).
- No raw property-id tokens cross into mapeditor (spec/clausething own translation — the #1355
  lesson; the fence is proven by diff + grep, not a bespoke guard).
- If a needed field-authority surface genuinely cannot exist pre-12.8, deliver the accessor +
  fail-soft/fail-loud paths with the live-readback seam marked `HORIZON-ENTRY(<today>): 12.8
  STUDIO-FIELD-SESSION-ELEVATE-0 wires live values` — dated per HC-6 convention.
## EXIT-PROOF
- Named tests, each catching a real regression: fail-soft 0.0 on absent field / structural shell;
  fail-loud on readback error; typed map keyed by generated system id; no-Spec-mutation proof;
  snapshot consistency. `cargo check` green on the 3 crates; agent-scan, orientation-check,
  doc-budget green. Zero raw-id tokens in mapeditor src (grep).
- PROBATION LEADS the 12.2 exit-proof cell in-diff AND the authoritative Active-open-rung row
  updated (two-source rule); orientation regenerated; DA stamps graduation at merge.
