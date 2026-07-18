---
rung: RF-NEED-WEIGHT-PROFILE-TRANSPORT-0
kind: rung
track: 0.0.8.6
base_sha: 99fa9d06898a1b03b490b7f876a17dfd4568e500
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "RF-4/12.9 DA-graduated + merged #1413 @ 99fa9d06; OVL closed. RF-5 is the approved need/weight_profile split before 12.10; Owner alone closes RF-5 OVL."
surfaces: ["crates/simthing-{clausething,spec,sim,driver,mapeditor}/src", "crates/simthing-mapeditor/tests", "crates/simthing-workshop/tests", "docs + scripts/ci"]
forbidden: ["new syntax or kernel/WGSL primitive", "Studio arithmetic/direct cell mutation/synthetic hierarchy", "TP-specific sealed-crate branch or 12.10 claims", "merge/readiness mutation"]
required_checks: ["cargo-check+studio-build", "focused+live-gpu proofs", "agent+doctrine", "orientation+inventory+budget", "owner-ovl"]
stop_conditions: ["generic binding needs new syntax/primitive", "profile cannot reach an existing participant role/cell through admitted machinery", "RF-4 oracle/GPU policy regresses"]
---
## BUILD
- First commit stamps 12.9/RF-4 **DA-GRADUATED / merged #1413 @ `99fa9d06`**, adds RF-5, makes it active, and regenerates orientation.
- Define one generic spec-owned binding from existing hydrated need/`weight_profile` EML data to an existing Arena participant role/cell; install it through ordinary GameMode/session-open and existing EML/Accumulator machinery.
- Authored weights must change live need state and FIELD_POLICY threshold outcome with zero code change; expose only admitted read-only telemetry.
- Prove the canonical scenario plus a neutral second scenario, preserve ordinary `open_from_spec + step_once`, then freeze a Windows debug executable for Owner capture.

## FENCES
- No new grammar, primitive, planner, scenario API, Studio feeder/mirror, direct accumulator mutation, synthetic Owner, or sealed-crate TP special case.
- RF-1 and RF-4 remain unchanged authorities; RF-5 proves transport/threshold only, not 12.10 macro emergence.
- Codex does not close OVL, graduate, mark ready, or merge.

## EXIT-PROOF
- Hydrated profile data enters an admitted spec binding and ordinary session open installs it; live GPU execution changes an existing need-bearing participant through existing machinery.
- Paired authorings differing only in weight produce different live need values; below-threshold fires none and crossing fires only through sealed FIELD_POLICY ingress.
- Removed/misbound profile fails closed without default-weight fallback or Studio-computed value; neutral scenario uses the same generic path.
- RF-4 regressions stay green: Owner aggregate `15/10`, marginal `5`, governed-Balance disconnect → `ResidualNotIntegrated`, exact RTX policy unchanged.
- Owner screenshot C shows scenario, tick, authored profile identity/value, live need value, and threshold result from the frozen executable; all focused/governance gates bind the final SHAs before DA relay.
