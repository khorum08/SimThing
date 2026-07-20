---
rung: STUDIO-DISRUPTION-SELECT-SCREEN-0
kind: rung
track: 0.0.8.6
base_sha: da8f7035c8b876c8e81d0450b68289230ef8f87e
audience: coding
model_tier: std
owner_approved: true
expected_route: DA-RESERVE(unclassified-scope)
owner_notes: "MANUAL PROGRESSION. Owner drives coder + orchestrator pokes; orchestrator owns review + delegated merge; escalations->orchestrator not DA. [OVL]: Owner screenshot closes the visual gate. 12.5 STUDIO-FLEET-ICONS-0 is queued next on this board."
surfaces: ["crates/simthing-mapeditor/src", "crates/simthing-mapeditor/tests", "docs/design_0_0_8_6_studio_live_ops.md", "docs/tests"]
forbidden: ["ScenarioSpec mutation from render/camera/UI", "new WGSL kernel semantics (presentation shader if unavoidable is DA-reserve)", "CPU planner / decisions outside threshold crossings", "scenario-specific code in a sealed crate", "breaking the 11.6 owned-set brighten"]
required_checks: ["cargo-check+studio-build", "focused-tests", "doctrine-scan", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "needs-new-wgsl-kernel-semantics"]
---
## BUILD
- Selecting ANY star (owned, neutral, hostile) screens the SELECTED star's blur and tint by its max
  accreted disruption, piecewise-linear and clamped: disruption 0 -> 100% blur / 0% red; 50 -> 200%
  blur / 50% red; 100 -> 500% blur / 100% red; >100 clamps. Deselect restores defaults.
- Attach via the EXISTING per-star visual path: `compute_star_radius_visual` scale-mul and the
  `sync_star_visuals_system` color branch (the 11.6 pattern). Read-only display expression.
- [OVL] ops-telemetry rows: selected system id, raw disruption, computed blur-scale / red-fraction —
  so the Owner screenshot verifies the screen effect against the numbers.
## FENCES
- Presentation only — no Spec mutation, no new WGSL semantics, no decision authority. Must coexist
  with the 11.6 owned-set brighten (both effects composable, order-stable).
- Disruption value read from the live session's admitted readout (the 12.2 disruption surface), never
  recomputed UI-side.
## EXIT-PROOF
- Named tests BITE: piecewise mapping exact at 0/50/100 + clamp above; deselect restores; 11.6
  brighten regression green. cargo check + studio build green; doctrine-scan (PR delta) /
  orientation-check / doc-budget green; tests ledgered (birth_track 0.0.8.6-studio-live-ops).
- PROBATION LEADS the 12.3 cell + Active-open-rung row; orientation regenerated. [OVL] stays open
  until the Owner screenshot confirms the effect against the telemetry rows.
