---
rung: STUDIO-FIELD-SESSION-ELEVATE-0
kind: rung
track: 0.0.8.6
base_sha: 456946421b662b0207b78405ba39b44c3c54fa5d
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(unclassified-scope)
owner_notes: "Fully-automated: coder=Grok CLI. Orchestrator owns CI/clearance/tree-review/remands + delegated merge. Escalations→orchestrator not DA. [OVL] Owner screenshot closes visual gate; code→PROBATION only."
surfaces: ["crates/simthing-mapeditor", "crates/simthing-workshop", "docs/design_0_0_8_6_studio_live_ops.md"]
forbidden: ["bespoke economy code in the tick", "CPU planner / AI-tick / decisions outside threshold crossings", "ScenarioSpec mutation from render/camera/UI", "new grammar/primitives in clausething or spec", "TP-specific code in a sealed crate (home to simthing-workshop)", "kernel/WGSL semantics"]
required_checks: ["cargo-check", "focused-tests", "doctrine-scan", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "tick-needs-bespoke-economy-code", "decision-fires-outside-threshold-crossing"]
---
## BUILD
- Open Studio live bridge **field-bearing** path (`open_from_spec` + authored profile); elevate TP-LIVE-RUN-0 residue so authored fields accrete under live ticks (disruption emitters; production/need from buildings + policy overlays; decisions only as threshold crossings / OC-K-DECISION-INGRESS-0). Replace property-strip for field-bearing scenarios.
- Keep structural-shell selectable as fallback (not deleted).
- **[OVL]** ops-telemetry: session path + per-tick field accretion samples for Owner live verification.
## FENCES
- Generic RF/STEAD only — **no bespoke economy in the tick**. If needed, STOP and report.
- Decisions = FIELD_POLICY threshold crossings only; ScenarioSpec sole authority (UI/render/telemetry presentation-only).
- No new clausething/spec grammar (12.6+12.8 inputs only).
- §12 `workshop-candidate-homing`: scenario-specific code/tests → `simthing-workshop`; WORKSHOP-HOMING-DETECTION PASS 0 — never weaken/exclude the scan.
## EXIT-PROOF
- Multi-tick headless on 12.8 canonical: disruption accretes; production/need accrete; threshold fires a decision and NOTHING fires absent a crossing. Tests BITE (delete accretion/threshold → fail).
- Structural-shell fallback green; cargo/doctrine-scan/orient/doc-budget green; tests ledgered (birth_track 0.0.8.6-studio-live-ops) + inspect/triage if TEST-BUDGET INSPECT.
- PROBATION leads 12.9 cell + Active-open-rung row; orient regen. **[OVL] open** until Owner screenshot.
