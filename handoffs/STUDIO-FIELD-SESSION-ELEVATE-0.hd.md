---
rung: STUDIO-FIELD-SESSION-ELEVATE-0
kind: rung
track: 0.0.8.6
base_sha: 456946421b662b0207b78405ba39b44c3c54fa5d
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(unclassified-scope)
owner_notes: "Fully-automated: coder = Grok CLI (grok-4.5, full write access). Orchestrator owns CI/clearance/tree-review/remands AND delegated merge. Escalations route to the orchestrator, never straight to DA. [OVL] rung: Owner screenshot verifies live accretion — code reaches PROBATION, Owner closes the visual gate."
surfaces: ["crates/simthing-mapeditor", "crates/simthing-workshop", "docs/design_0_0_8_6_studio_live_ops.md"]
forbidden: ["bespoke economy code in the tick", "CPU planner / AI-tick / decisions outside threshold crossings", "ScenarioSpec mutation from render/camera/UI", "new grammar/primitives in clausething or spec", "TP-specific code in a sealed crate (home to simthing-workshop)", "kernel/WGSL semantics"]
required_checks: ["cargo-check", "focused-tests", "doctrine-scan", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "tick-needs-bespoke-economy-code", "decision-fires-outside-threshold-crossing"]
---
## BUILD
- Open the Studio live bridge's **field-bearing session path** (`open_from_spec` + authored profile),
  elevating the TP-LIVE-RUN-0 workshop residue to production, so authored fields accumulate under
  live ticks: disruption accretes from authored emitters; production/need accrete from authored
  buildings + policy overlays; decisions fire ONLY as threshold crossings (sealed ingress per
  OC-K-DECISION-INGRESS-0). Replaces the property-strip posture for field-bearing scenarios.
- Keep the structural-shell path available as fallback (selectable, not deleted).
- **[OVL]** ops-telemetry rows: session path (structural-shell vs field-bearing) and per-tick field
  accretion samples, so the Owner can verify live accretion from the running Studio.
## FENCES
- Generic RF/STEAD pipeline ONLY — **no bespoke economy code in the tick**. If the tick appears to
  need economy-specific logic, STOP and report; do not special-case.
- Decisions stay FIELD_POLICY threshold crossings; no CPU planner / AI-tick. ScenarioSpec stays the
  sole authority — Bevy/egui/camera/telemetry are presentation and must not mutate it.
- No new grammar/primitives in clausething/spec (12.6 grammar + 12.8 authored data are the inputs).
- §12 homing (anchor `workshop-candidate-homing`, ack it): scenario-specific code/tests home to
  `simthing-workshop`; the `WORKSHOP-HOMING-DETECTION` scan must stay PASS 0 on the PR delta — never
  add an exclusion or weaken the scan.
## EXIT-PROOF
- Multi-tick headless proof on the 12.8-authored canonical scenario: disruption accretes from the
  authored emitter; production/need accrete from authored buildings + policy overlays; a threshold
  crossing fires a decision and NOTHING fires absent a crossing. Each named test catches a real
  regression and BITES (deleting the accretion coupling or the threshold must fail it).
- Structural-shell fallback still selectable and green. `cargo check` green; doctrine-scan (PR delta)
  clean; orientation-check / doc-budget green; new tests ledgered (birth_track 0.0.8.6-studio-live-ops)
  with an inspect_justifications row + SHA-bound triage if TEST-BUDGET INSPECTs.
- PROBATION LEADS the 12.9 cell + the authoritative Active-open-rung row updated; orientation
  regenerated. **[OVL] stays open** until the Owner's screenshot confirms live accretion.
