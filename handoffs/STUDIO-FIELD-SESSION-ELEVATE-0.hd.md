---
rung: STUDIO-FIELD-SESSION-ELEVATE-0
kind: rung
track: 0.0.8.6
base_sha: 456946421b662b0207b78405ba39b44c3c54fa5d
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(unclassified-scope)
owner_notes: "Fully-automated: coder=Grok CLI. Orchestrator owns CI/clearance/tree-review/remands + delegated merge. Escalations→orchestrator not DA. [OVL] Owner screenshots close visual gate. Windows debug Studio build only after adjudication-clean final SHA (not committed)."
surfaces: ["crates/simthing-mapeditor", "crates/simthing-workshop", "docs/design_0_0_8_6_studio_live_ops.md"]
forbidden: ["bespoke economy code in the tick", "CPU planner / AI-tick / decisions outside threshold crossings", "ScenarioSpec mutation from render/camera/UI", "new grammar/primitives in clausething or spec", "TP-specific code in a sealed crate (home to simthing-workshop)", "kernel/WGSL semantics"]
required_checks: ["cargo-check", "focused-tests", "doctrine-scan", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "tick-needs-bespoke-economy-code", "decision-fires-outside-threshold-crossing"]
---
## BUILD
- Open Studio live bridge **field-bearing** path (`open_from_spec` + authored profile); elevate residue so authored fields accrete under live ticks (disruption emitters; production/**need** + policy overlays; decisions only as threshold crossings). Keep structural-shell fallback.
- **[OVL]** ops-telemetry (session path + per-tick field accretion samples bound to resolved emission slot/col). Windows **debug** `simthing-studio` at exact final source SHA for Owner screenshots (runbook in results; exe not committed) — build only after need-seam adjudication clean.
## FENCES
- Generic RF/STEAD only — **no bespoke economy in the tick**. Decisions = FIELD_POLICY threshold crossings only; ScenarioSpec sole authority.
- No new clausething/spec grammar. §12 `workshop-candidate-homing`; WORKSHOP-HOMING-DETECTION PASS 0. Do not invent need/weight-profile install APIs in this PR.
## EXIT-PROOF
- Multi-tick headless: disruption accretes; **production/need accrete from authored buildings and policy overlays** (design 12.9); threshold fires under live ticks (zero at open / zero without threshold). Tests BITE. Studio readout samples show multi-tick value deltas.
- **BLOCKED (need half):** no admitted GameMode/open_from_spec seam for field-economy `weight_profile` EML stacks — need live output not proven; not self-narrowed to production/policy. Owner/orchestrator adjudicate (e.g. 12.9a seam rung) before full 12.9 PROBATION.
- cargo/doctrine-scan/orient/doc-budget green; tests ledgered. **[OVL] open** until Owner screenshots (identity + live accretion + progression).
