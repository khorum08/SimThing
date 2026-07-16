---
rung: TP-FIELD-ECONOMY-GRAMMAR-0
kind: rung
track: 0.0.8.6
base_sha: 5adb5be8b80668f24d6684d0e8429d0dd4d00383
audience: coding
model_tier: frontier
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "Fully-automated: coder = Codex CLI (gpt-5.5 high). Orchestrator (webchat Codex) bears CI/clearance/tree-review + remands directly; DA rules only on the final ORCHESTRATOR->DA RELAY. One rung at a time."
surfaces: ["crates/simthing-clausething", "crates/simthing-spec", "docs/design_0_0_8_6_studio_live_ops.md"]
forbidden: ["TP tokens/literals in clausething", "CPU planner / AI-tick", "kernel/WGSL semantics", "new overlay/RF primitives", "scripts/ci logic", "Studio/UI"]
required_checks: ["cargo-check", "focused-tests", "agent-scan", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "tp-token-in-clausething", "new-primitive-instead-of-lowering"]
---
## BUILD
- Generic scenario-agnostic ClauseScript grammar + hydration for field-enrolled economics:
  production buildings (factory/starport chains), stockpile silos, field-enrolled resource
  quantities, disruption-emitting presence, owner policy overlay blocks — need/opportunity WEIGHT
  PROFILES (expansion-need, disruption-need, manufacturing-need).
- LOWER onto EXISTING surfaces only: OverlaySpec, ResourceEconomySpec, EML weight profiles
  (TP-COMMITMENTS-0). No new overlay/RF/FIELD_POLICY primitive.
- Spatial enrollment obeys STEAD §5: Location participants carry StructuralGridPlacement.
## FENCES
- ZERO TP tokens/literals in clausething — scenario-agnostic; a SECOND synthetic (non-TP) scenario
  must hydrate through the SAME grammar. Decisions stay FIELD_POLICY / threshold crossings (no CPU
  planner / AI-tick). No kernel/WGSL, scripts/ci, Studio/UI.
- If a lowering target genuinely does not exist, mark the seam HORIZON-ENTRY(<today>): <downstream
  rung> wires it (HC-6 dated convention); do not invent a primitive; STOP+report if a fence breaks.
## EXIT-PROOF
- Falsifiers (adversarial paired fixtures): well-formed economy block hydrates to the expected
  overlay/RF/weight-profile lowering; malformed is a spanned hard error at admission (not a runtime
  branch). The second synthetic scenario proves scenario-agnosticism. Named tests each catch a real
  regression.
- cargo check green; zero TP tokens in clausething (grep); agent-scan/orientation-check/doc-budget
  green; new tests ledgered (birth_track 0.0.8.6-studio-live-ops).
- PROBATION LEADS the 12.6 cell + the authoritative Active-open-rung row updated; orientation
  regenerated; DA stamps graduation at merge.
