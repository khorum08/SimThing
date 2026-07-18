---
rung: RF-NEED-WEIGHT-PROFILE-TRANSPORT-0
kind: rung
track: 0.0.8.6
base_sha: 99fa9d06898a1b03b490b7f876a17dfd4568e500
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "RF-4/12.9 DA-graduated and merged #1413 at 99fa9d06; Owner OVL PASS/CLOSED. RF-5 is the approved bounded need/weight_profile split and precedes 12.10. Orchestration owns review/merge; Owner alone closes RF-5 [OVL]."
surfaces: ["crates/simthing-clausething/src", "crates/simthing-spec/src", "crates/simthing-sim/src", "crates/simthing-driver/src", "crates/simthing-mapeditor/src", "crates/simthing-mapeditor/tests", "crates/simthing-workshop/tests", "docs/design_0_0_8_6_studio_live_ops.md", "docs/tests", "docs/orchestrator_orientation.md", "scripts/ci"]
forbidden: ["new ClauseScript syntax", "new kernel/WGSL primitive", "Studio-side need arithmetic or direct Arena-cell mutation", "synthetic Owner hierarchy", "TP-specific production branch", "12.10 emergence claims", "merge/readiness mutation"]
required_checks: ["cargo-check+studio-build", "focused-tests", "live-gpu-proof", "agent+doctrine-scan", "orientation+inventory", "doc-budget", "owner-ovl"]
stop_conditions: ["binding requires new syntax or primitive", "existing EML stack cannot reach an existing Arena participant role/cell generically", "need state can only be presentation-computed", "scenario-specific sealed-crate branch appears", "RF-4 oracle or GPU policy regresses"]
---
## BUILD
- First commit: stamp 12.9/RF-4 **DA-GRADUATED / merged #1413 @ `99fa9d06`**, add RF-5 to the ladder, make RF-5 active, and regenerate orientation.
- Define one generic spec-owned binding from an existing hydrated need/`weight_profile` EML gadget stack to an existing Arena participant role/cell; no new authoring syntax.
- Install that binding through the ordinary GameMode/session-open compiler and existing EML/Accumulator machinery; Studio must not patch participant cells.
- Preserve canonical authority: real GameSession → real Owner → real authored children and ordinary `SimSession::open_from_spec + step_once`.
- Make authored profile weights affect live need state and FIELD_POLICY threshold decisions; changing only the authored profile must change the live result with zero code change.
- Expose the admitted live need/profile value and threshold result through read-only Studio telemetry; label source identity and current tick.
- Keep the canonical Terran/Pirate proof workshop-homed and add a neutral second-scenario proof through the same generic path.
- Build a frozen Windows debug executable only after implementation/tests are stable; bind source SHA and hash for Owner capture.

## FENCES
- No new grammar, kernel/WGSL primitive, planner, or scenario API.
- No Studio-side arithmetic, feeder patch, direct accumulator mutation, or presentation-only mirror.
- No TP/faction/kind special case in sealed production crates.
- Existing RF-1 oracle and RF-4 recursive execution remain unchanged authorities.
- RF-5 proves transport and threshold effect only; 12.10 macro-emergence/policy-divergence claims remain out of scope.
- Codex does not close [OVL], graduate, mark ready, or merge.

## EXIT-PROOF
- Canonical ClauseScript hydration carries authored need/`weight_profile` data into an admitted spec-owned binding and ordinary session open installs it.
- Live GPU execution reads the bound profile through existing EML/Accumulator machinery and changes an existing need-bearing participant state.
- Paired authorings differing only in profile weight produce different live need values and the expected threshold/no-threshold result with zero code change.
- Below-threshold control fires no decision; crossing control fires only through sealed FIELD_POLICY ingress.
- Removing or misbinding the profile fails closed and cannot silently fall back to a default weight or Studio-computed value.
- Neutral second scenario follows the same generic path; scenario vocabulary remains workshop-homed.
- RF-4 regressions remain green: Owner aggregate `15/10`, marginal `5`, governed-Balance disconnect → `ResidualNotIntegrated`, exact RTX policy unchanged.
- Owner [OVL] screenshot C shows canonical scenario, tick, authored profile identity/value, live need value, and threshold result from the frozen executable.
- Focused suites, Studio build, live-GPU proof, inventory/orientation, scans, budget, exact-head Clearance, and metadata bindings are green before DA relay.
