---
rung: TP-CLAUSE-ECONOMY-AUTHOR-0
kind: rung
track: 0.0.8.6
base_sha: 5912e6094bc3ffb9bc190931911057fd6777d5f0
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "Fully-automated: coder = Grok CLI (grok-4.5, full write access, visible console). Orchestrator (webchat) owns CI/clearance/tree-review/remands AND holds DELEGATED MERGE. DA rules only if absolutely necessary. Escalations route to the orchestrator, never straight to DA."
surfaces: ["scenarios/terran_pirate_galaxy.clause", "crates/simthing-workshop", "docs/design_0_0_8_6_studio_live_ops.md"]
forbidden: ["new grammar/primitives in clausething or spec", "TP-specific code/tests in ANY sealed production crate (home to simthing-workshop)", "hand-edited JSON/RON anywhere", "CPU planner / AI-tick", "kernel/WGSL/Studio/UI", "scripts/ci logic"]
required_checks: ["cargo-check", "focused-tests", "doctrine-scan", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "grammar-gap-needs-new-primitive", "scenario-code-would-land-in-sealed-crate"]
---
## BUILD
- Author the canonical TP economy as human-authored ClauseScript in `scenarios/terran_pirate_galaxy.clause`: Terran manufacturing base (factories -> production fields -> ship-construction need), Pirate disruption emitters, fleets, and owner policy overlays (Terran expansion/manufacturing-need weights; Pirate disruption/raid-need weights).
- Use ONLY the generic field-economy grammar landed by 12.6 (`hydrate_field_economy`) and the existing overlay/RF/EML surfaces. Author DATA, not code.
- Sibling base-disc regenerates ONLY through production hydration — no hand-edited JSON/RON; regeneration must be deterministic and must hydrate blind from an alien cwd.
## FENCES
- §12 homing (anchor `workshop-candidate-homing`, ack it): scenario-specific service/struct/fn/heuristic/test does NOT go in a sealed crate — homes to `simthing-workshop`. `WORKSHOP-HOMING-DETECTION` flags net-new TP vocabulary in production crates; any hit must be workshop-homed, not excluded. Scenario DATA in `scenarios/**` is the correct economy home.
- If the 12.6 grammar cannot express something, STOP and report — do NOT invent a primitive or widen clausething/spec. A `HORIZON-ENTRY(2026-07-16)` seam exists in `hydrate_field_economy.rs` for production output coefficients; use it as documented, do not extend it.
- No hand-edited JSON/RON; decisions stay FIELD_POLICY threshold crossings (no CPU planner).
## EXIT-PROOF
- Authored `.clause` hydrates through production hydration to the expected economy spec; regeneration is deterministic (same input -> byte-identical output) and hydrates blind from an alien cwd. Named tests each catch a real regression.
- Zero net-new TP vocabulary in sealed production crates (doctrine-scan clean or every hit workshop-homed); cargo check + doctrine-scan + orientation-check + doc-budget green; new tests ledgered (birth_track 0.0.8.6-studio-live-ops).
- PROBATION LEADS the 12.8 cell + the authoritative Active-open-rung row updated; orientation regenerated.
